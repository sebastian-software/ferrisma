// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! AcStrategyHeuristics from lib/jxl/enc_ac_strategy.{h,cc}: Init,
//! PrepareForThreads, ProcessRect, Finalize, and the one-lane (HWY_SCALAR)
//! ProcessRectACS with MultiBlockTransformCrossesHorizontalBoundary,
//! MultiBlockTransformCrossesVerticalBoundary, EstimateEntropy,
//! FindBest8x8Transform, TryMergeAcs, SetEntropyForTransform,
//! AcsSquare/AcsVerticalSplit/AcsHorizontalSplit and
//! FindBestFirstLevelDivisionForSquare. The JXL_DEBUG_AC_STRATEGY image dump
//! (TypeColor, TypeMask, DumpAcStrategy) is disabled at the pinned revision
//! and unported.
//!
//! ACSConfig's row pointers into the caller's planes become the borrows of
//! [`AcsPlanes`], which ProcessRect receives for the owners passed to Init:
//! the caller adjusts the quant field between tiles, so no borrow outlives a
//! call (ADR 0003). Highway's scalar Mul is the correctly rounded binary32
//! product; MulAdd, Sqrt and Round are its one-lane operations. The original
//! object contracts EstimateEntropy's norm sum and its three `entropy +=`
//! products and FindBest8x8Transform's two multiplier adjustments: fused on
//! AArch64 and separate on x86-64 (tools/encoder_heuristics_contraction.json),
//! as `mul_add_f32` reproduces; YtoXRatio/YtoBRatio and the AFV and inverse
//! transforms reuse their already contracted ports. `pow(float, float)` calls
//! the host `powf`, and the static `pow(double, double)` channel weights use
//! the host `pow`. Scratch memory is the source's per-thread AlignedMemory.
use super::ac_strategy::{
    AcStrategy, AcStrategyImage, AcStrategyType, MAX_BLOCK_DIM, MAX_COEFF_AREA,
};
use super::base::bits::ceil_log2_nonzero;
use super::base::fast_math::fast_powf;
use super::base::float::{max_f32, mul_add_f32};
use super::base::hwy_scalar::round;
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::chroma_from_luma::{COLOR_TILE_DIM_IN_BLOCKS, ColorCorrelationMap};
use super::common::SpeedTier;
use super::dec_transforms::transform_to_pixels;
use super::enc_aux_out::AuxOut;
use super::enc_chroma_from_luma::MAX_VECTOR_SIZE;
use super::enc_params::CompressParams;
use super::enc_transforms::transform_from_pixels;
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE, FrameDimensions};
use super::image::{Image3F, ImageF};
use super::memory_manager_internal::{AlignedMemory, MemoryManagerPtr};
use super::quant_weights::DequantMatrices;

/// The scalar fields of the source ACSConfig that Init computes.
#[derive(Clone, Copy, Debug, Default)]
pub struct AcsConfigScalars {
    pub quant_field_stride: usize,
    pub masking_field_stride: usize,
    pub masking1x1_field_stride: usize,
    pub mask1x1_xsize: usize,
    pub src_stride: usize,
    pub info_loss_multiplier: f32,
    pub cost_delta: f32,
    pub zeros_mul: f32,
}

/// The owners whose rows the source ACSConfig points to: the source image and
/// its rect, the quant field, the masking fields and the dequant matrices.
#[derive(Clone, Copy)]
pub struct AcsPlanes<'a> {
    pub src: &'a Image3F,
    pub rect_in: Rect,
    pub quant_field: &'a ImageF,
    pub mask: &'a ImageF,
    pub mask1x1: &'a ImageF,
    pub dequant: &'a DequantMatrices,
}

/// The source ACSConfig, with each row pointer as a slice from that row on.
pub struct AcsConfig<'a> {
    pub dequant: &'a DequantMatrices,
    pub quant_field_row: &'a [f32],
    pub quant_field_stride: usize,
    pub masking_field_row: Option<&'a [f32]>,
    pub masking_field_stride: usize,
    pub masking1x1_field_row: Option<&'a [f32]>,
    pub masking1x1_field_stride: usize,
    pub mask1x1_xsize: usize,
    pub src_rows: [&'a [f32]; 3],
    pub src_stride: usize,
    pub info_loss_multiplier: f32,
    pub cost_delta: f32,
    pub zeros_mul: f32,
}

impl<'a> AcsConfig<'a> {
    pub fn new(scalars: &AcsConfigScalars, planes: &AcsPlanes<'a>) -> Self {
        let rows_from = |plane: &'a ImageF, x0: usize, y0: usize| -> Option<&'a [f32]> {
            if plane.xsize() > 0 && plane.ysize() > 0 {
                let stride = plane.pixels_per_row() as usize;
                plane.pixels().map(|pixels| &pixels[y0 * stride + x0..])
            } else {
                None
            }
        };
        let rect = planes.rect_in;
        let src_row =
            |c: usize| rows_from(planes.src.plane(c), rect.x0(), rect.y0()).unwrap_or(&[]);
        Self {
            dequant: planes.dequant,
            quant_field_row: rows_from(planes.quant_field, 0, 0).unwrap_or(&[]),
            quant_field_stride: scalars.quant_field_stride,
            masking_field_row: rows_from(planes.mask, 0, 0),
            masking_field_stride: scalars.masking_field_stride,
            masking1x1_field_row: rows_from(planes.mask1x1, 0, 0),
            masking1x1_field_stride: scalars.masking1x1_field_stride,
            mask1x1_xsize: scalars.mask1x1_xsize,
            src_rows: [src_row(0), src_row(1), src_row(2)],
            src_stride: scalars.src_stride,
            info_loss_multiplier: scalars.info_loss_multiplier,
            cost_delta: scalars.cost_delta,
            zeros_mul: scalars.zeros_mul,
        }
    }
    /// `&config.Pixel(c, x, y)` and the source row from there on.
    pub fn pixel(&self, c: usize, x: usize, y: usize) -> &'a [f32] {
        &self.src_rows[c][y * self.src_stride + x..]
    }
    pub fn masking(&self, bx: usize, by: usize) -> f32 {
        let row = self.masking_field_row.expect("a masking field");
        debug_assert!(row[by * self.masking_field_stride + bx] > 0.0);
        row[by * self.masking_field_stride + bx]
    }
    pub fn masking_ptr1x1(&self, bx: usize, by: usize) -> &'a [f32] {
        let row = self.masking1x1_field_row.expect("a pixel masking field");
        debug_assert!(row[by * self.masking1x1_field_stride + bx] > 0.0);
        &row[by * self.masking1x1_field_stride + bx..]
    }
    pub fn quant(&self, bx: usize, by: usize) -> f32 {
        debug_assert!(self.quant_field_row[by * self.quant_field_stride + bx] > 0.0);
        self.quant_field_row[by * self.quant_field_stride + bx]
    }
}

#[expect(clippy::manual_is_multiple_of, reason = "keeps the source's % tests")]
pub fn multi_block_transform_crosses_horizontal_boundary(
    ac_strategy: &AcStrategyImage,
    mut start_x: usize,
    y: usize,
    mut end_x: usize,
) -> bool {
    if start_x >= ac_strategy.xsize() || y >= ac_strategy.ysize() {
        return false;
    }
    if y % 8 == 0 {
        // Nothing crosses 64x64 boundaries, and the memory on the other side
        // of the 64x64 block may still uninitialized.
        return false;
    }
    end_x = end_x.min(ac_strategy.xsize());
    // The first multiblock might be before the start_x, let's adjust it
    // to point to the first IsFirstBlock() == true block we find by backward
    // tracing.
    let row = ac_strategy.const_row(y, 0);
    let start_x_limit = start_x & !7;
    while start_x != start_x_limit && !row.get(start_x).is_first_block() {
        start_x -= 1;
    }
    let mut x = start_x;
    while x < end_x {
        if row.get(x).is_first_block() {
            x += row.get(x).covered_blocks_x();
        } else {
            return true;
        }
    }
    false
}

#[expect(clippy::manual_is_multiple_of, reason = "keeps the source's % tests")]
pub fn multi_block_transform_crosses_vertical_boundary(
    ac_strategy: &AcStrategyImage,
    x: usize,
    mut start_y: usize,
    mut end_y: usize,
) -> bool {
    if x >= ac_strategy.xsize() || start_y >= ac_strategy.ysize() {
        return false;
    }
    if x % 8 == 0 {
        // Nothing crosses 64x64 boundaries, and the memory on the other side
        // of the 64x64 block may still uninitialized.
        return false;
    }
    end_y = end_y.min(ac_strategy.ysize());
    // The first multiblock might be before the start_y, let's adjust it
    // to point to the first IsFirstBlock() == true block we find by backward
    // tracing.
    let start_y_limit = start_y & !7;
    while start_y != start_y_limit && !ac_strategy.const_row(start_y, 0).get(x).is_first_block() {
        start_y -= 1;
    }

    let mut y = start_y;
    while y < end_y {
        let row = ac_strategy.const_row(y, 0);
        if row.get(x).is_first_block() {
            y += row.get(x).covered_blocks_y();
        } else {
            return true;
        }
    }
    false
}

#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(
    clippy::assign_op_pattern,
    clippy::int_plus_one,
    reason = "keeps Highway's Add(Sqrt(q), entropy_v) and the source's lane bound"
)]
pub fn estimate_entropy(
    acs: &AcStrategy,
    entropy_mul: f32,
    x: usize,
    y: usize,
    config: &AcsConfig<'_>,
    cmap_factors: &[f32; 3],
    block: &mut [f32],
    full_scratch_space: &mut [f32],
    _quantized: &mut [u32],
    entropy: &mut f32,
) -> Status {
    *entropy = 0.0;
    let (mem, scratch_space) = full_scratch_space.split_at_mut(MAX_COEFF_AREA);
    let size = (1usize << acs.log2_covered_blocks()) * DCT_BLOCK_SIZE;

    // Apply transform.
    for c in 0..3 {
        let block_c = &mut block[size * c..];
        transform_from_pixels(
            acs.strategy(),
            config.pixel(c, x, y),
            config.src_stride,
            block_c,
            scratch_space,
        );
    }

    let num_blocks = acs.covered_blocks_x() * acs.covered_blocks_y();
    // avoid large blocks when there is a lot going on in red-green.
    let mut quant_norm16 = 0.0f32;
    if num_blocks == 1 {
        // When it is only one 8x8, we don't need aggregation of values.
        quant_norm16 = config.quant(x / 8, y / 8);
    } else if num_blocks == 2 {
        // Taking max instead of 8th norm seems to work
        // better for smallest blocks up to 16x8. Jyrki couldn't get
        // improvements in trying the same for 16x16 blocks.
        if acs.covered_blocks_y() == 2 {
            quant_norm16 = max_f32(config.quant(x / 8, y / 8), config.quant(x / 8, y / 8 + 1));
        } else {
            quant_norm16 = max_f32(config.quant(x / 8, y / 8), config.quant(x / 8 + 1, y / 8));
        }
    } else {
        // Load QF value, calculate empirical heuristic on masking field
        // for weighting the information loss. Information loss manifests
        // itself as ringing, and masking could hide it.
        for iy in 0..acs.covered_blocks_y() {
            for ix in 0..acs.covered_blocks_x() {
                let mut qval = config.quant(x / 8 + ix, y / 8 + iy);
                qval *= qval;
                qval *= qval;
                qval *= qval;
                quant_norm16 = mul_add_f32(qval, qval, quant_norm16);
            }
        }
        quant_norm16 /= num_blocks as f32;
        quant_norm16 = fast_powf(quant_norm16, 1.0f32 / 16.0);
    }
    let quant = quant_norm16;

    // Compute entropy.
    let mut loss = 0.0f32;
    for c in 0..3 {
        let inv_matrix = config.dequant.inv_matrix(acs.strategy(), c);
        let matrix = config.dequant.matrix(acs.strategy(), c);
        let cmap_factor = cmap_factors[c];

        let mut entropy_v = 0.0f32;
        let mut nzeros_v = 0.0f32;
        for i in 0..num_blocks * DCT_BLOCK_SIZE {
            let input = block[c * size + i];
            let in_y = block[size + i] * cmap_factor;
            let im = inv_matrix[i];
            let val = (input - in_y) * (im * quant);
            let rval = round(val);
            let diff = val - rval;
            let m = matrix[i];
            mem[i] = m * diff;
            let q = rval.abs();
            let q_is_zero = q == 0.0;
            // We used to have q * C here, but that cost model seems to
            // be punishing large values more than necessary. Sqrt tries
            // to avoid large values less aggressively.
            entropy_v = q.sqrt() + entropy_v;
            nzeros_v += if q_is_zero { 0.0 } else { 1.0 };
        }

        {
            let masku_lut = [12.0f32, 0.0, 4.0];
            let masku_off = masku_lut[c];
            let mut lossc = 0.0f32;
            transform_to_pixels(
                acs.strategy(),
                mem,
                block,
                acs.covered_blocks_x() * 8,
                scratch_space,
            );

            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..acs.covered_blocks_x() {
                    for dy in 0..BLOCK_DIM {
                        for dx in 0..BLOCK_DIM {
                            let mut input = block[(iy * BLOCK_DIM + dy)
                                * (acs.covered_blocks_x() * BLOCK_DIM)
                                + ix * BLOCK_DIM
                                + dx];
                            if x + ix * 8 + dx + 1 <= config.mask1x1_xsize {
                                let masku = config.masking_ptr1x1(x + ix * 8 + dx, y + iy * 8 + dy)
                                    [0]
                                    + masku_off;
                                input *= masku;
                                input *= input;
                                input *= input;
                                input *= input;
                                lossc += input;
                            }
                        }
                    }
                }
            }
            let channel_mul = [8.2f64.powf(8.0), 1.0f64.powf(8.0), 1.03f64.powf(8.0)];
            lossc *= channel_mul[c] as f32;
            loss += lossc;
        }
        *entropy = mul_add_f32(config.cost_delta, entropy_v, *entropy);
        let num_nzeros = nzeros_v as usize;
        // Add #bit of num_nonzeros, as an estimate of the cost for encoding the
        // number of non-zeros of the block.
        let nbits = ceil_log2_nonzero(num_nzeros + 1) + 1;
        // Also add #bit of #bit of num_nonzeros, to estimate the ANS cost, with a
        // bias.
        *entropy = mul_add_f32(
            config.zeros_mul,
            (ceil_log2_nonzero(nbits + 17) + nbits) as f32,
            *entropy,
        );
        if c == 0 && num_blocks >= 2 {
            // It is X channel (red-green) and we often see ringing
            // in the large blocks. Let's punish that more here.
            let w = (1.0 + 3.0f64.min(num_blocks as f64 / 8.0)) as f32;
            *entropy *= w;
            loss *= w;
        }
    }
    let loss_scalar = (loss / (num_blocks * DCT_BLOCK_SIZE) as f32).powf(1.0f32 / 8.0)
        * (num_blocks * DCT_BLOCK_SIZE) as f32
        / quant_norm16;
    *entropy *= entropy_mul;
    *entropy = mul_add_f32(config.info_loss_multiplier, loss_scalar, *entropy);
    Ok(())
}

struct TransformTry8x8 {
    kind: AcStrategyType,
    encoding_speed_tier_max_limit: i32,
    entropy_mul: f64,
}

#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
pub fn find_best_8x8_transform(
    x: usize,
    y: usize,
    encoding_speed_tier: i32,
    butteraugli_target: f32,
    config: &AcsConfig<'_>,
    cmap_factors: &[f32; 3],
    _ac_strategy: &mut AcStrategyImage,
    block: &mut [f32],
    scratch_space: &mut [f32],
    quantized: &mut [u32],
    entropy_out: &mut f32,
    best_tx: &mut AcStrategyType,
) -> Status {
    const TRANSFORMS_8X8: [TransformTry8x8; 10] = [
        TransformTry8x8 {
            kind: AcStrategyType::DCT,
            encoding_speed_tier_max_limit: 9,
            entropy_mul: 0.8,
        },
        TransformTry8x8 {
            kind: AcStrategyType::DCT4X4,
            encoding_speed_tier_max_limit: 5,
            entropy_mul: 1.08,
        },
        TransformTry8x8 {
            kind: AcStrategyType::DCT2X2,
            encoding_speed_tier_max_limit: 5,
            entropy_mul: 0.95,
        },
        TransformTry8x8 {
            kind: AcStrategyType::DCT4X8,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: 0.85931637428340035,
        },
        TransformTry8x8 {
            kind: AcStrategyType::DCT8X4,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: 0.85931637428340035,
        },
        TransformTry8x8 {
            kind: AcStrategyType::IDENTITY,
            encoding_speed_tier_max_limit: 5,
            entropy_mul: 1.0427542510634957,
        },
        TransformTry8x8 {
            kind: AcStrategyType::AFV0,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: 0.81779489591359944,
        },
        TransformTry8x8 {
            kind: AcStrategyType::AFV1,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: 0.81779489591359944,
        },
        TransformTry8x8 {
            kind: AcStrategyType::AFV2,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: 0.81779489591359944,
        },
        TransformTry8x8 {
            kind: AcStrategyType::AFV3,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: 0.81779489591359944,
        },
    ];
    let mut best = 1e30f64;
    *best_tx = TRANSFORMS_8X8[0].kind;
    for tx in &TRANSFORMS_8X8 {
        if tx.encoding_speed_tier_max_limit < encoding_speed_tier {
            continue;
        }
        let acs = AcStrategy::from_raw_strategy(tx.kind);
        let mut entropy_mul = (tx.entropy_mul / TRANSFORMS_8X8[0].entropy_mul) as f32;
        if (tx.kind == AcStrategyType::DCT2X2 || tx.kind == AcStrategyType::IDENTITY)
            && butteraugli_target < 5.0
        {
            let favor_2x2_at_high_quality = 0.4f64 as f32;
            // pow(x, 2.0f) is the product x * x in the original object.
            let base = (5.0f32 - butteraugli_target) / 5.0f32;
            let weight = base * base;
            entropy_mul = mul_add_f32(-favor_2x2_at_high_quality, weight, entropy_mul);
        }
        if (tx.kind != AcStrategyType::DCT
            && tx.kind != AcStrategyType::DCT2X2
            && tx.kind != AcStrategyType::IDENTITY)
            && butteraugli_target > 4.0
        {
            let avoid_entropy_of_transforms = 0.5f32;
            let mut mul = 1.0f32;
            if butteraugli_target < 12.0 {
                mul = (f64::from(mul) * ((12.0 - 4.0) / (f64::from(butteraugli_target) - 4.0)))
                    as f32;
            }
            entropy_mul = mul_add_f32(avoid_entropy_of_transforms, mul, entropy_mul);
        }
        let mut entropy = 0.0f32;
        estimate_entropy(
            &acs,
            entropy_mul,
            x,
            y,
            config,
            cmap_factors,
            block,
            scratch_space,
            quantized,
            &mut entropy,
        )?;
        if f64::from(entropy) < best {
            *best_tx = tx.kind;
            best = f64::from(entropy);
        }
    }
    *entropy_out = best as f32;
    Ok(())
}

// bx, by addresses the 64x64 block at 8x8 subresolution
// cx, cy addresses the left, upper 8x8 block position of the candidate
// transform.
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
pub fn try_merge_acs(
    acs_raw: AcStrategyType,
    bx: usize,
    by: usize,
    cx: usize,
    cy: usize,
    config: &AcsConfig<'_>,
    cmap_factors: &[f32; 3],
    ac_strategy: &mut AcStrategyImage,
    entropy_mul: f32,
    candidate_priority: u8,
    priority: &mut [u8; 64],
    entropy_estimate: &mut [f32; 64],
    block: &mut [f32],
    scratch_space: &mut [f32],
    quantized: &mut [u32],
) -> Status {
    let acs = AcStrategy::from_raw_strategy(acs_raw);
    let mut entropy_current = 0.0f32;
    for iy in 0..acs.covered_blocks_y() {
        for ix in 0..acs.covered_blocks_x() {
            if priority[(cy + iy) * 8 + (cx + ix)] >= candidate_priority {
                // Transform would reuse already allocated blocks and
                // lead to invalid overlaps, for example DCT64X32 vs.
                // DCT32X64.
                return Ok(());
            }
            entropy_current += entropy_estimate[(cy + iy) * 8 + (cx + ix)];
        }
    }
    let mut entropy_candidate = 0.0f32;
    estimate_entropy(
        &acs,
        entropy_mul,
        (bx + cx) * 8,
        (by + cy) * 8,
        config,
        cmap_factors,
        block,
        scratch_space,
        quantized,
        &mut entropy_candidate,
    )?;
    if entropy_candidate >= entropy_current {
        return Ok(());
    }
    // Accept the candidate.
    for iy in 0..acs.covered_blocks_y() {
        for ix in 0..acs.covered_blocks_x() {
            entropy_estimate[(cy + iy) * 8 + cx + ix] = 0.0;
            priority[(cy + iy) * 8 + cx + ix] = candidate_priority;
        }
    }
    ac_strategy.set(bx + cx, by + cy, acs_raw)?;
    entropy_estimate[cy * 8 + cx] = entropy_candidate;
    Ok(())
}

fn set_entropy_for_transform(
    cx: usize,
    cy: usize,
    acs_raw: AcStrategyType,
    entropy: f32,
    entropy_estimate: &mut [f32; 64],
) {
    let acs = AcStrategy::from_raw_strategy(acs_raw);
    for dy in 0..acs.covered_blocks_y() {
        for dx in 0..acs.covered_blocks_x() {
            entropy_estimate[(cy + dy) * 8 + cx + dx] = 0.0;
        }
    }
    entropy_estimate[cy * 8 + cx] = entropy;
}

fn acs_square(blocks: usize) -> AcStrategyType {
    if blocks == 2 {
        AcStrategyType::DCT16X16
    } else if blocks == 4 {
        AcStrategyType::DCT32X32
    } else {
        AcStrategyType::DCT64X64
    }
}

fn acs_vertical_split(blocks: usize) -> AcStrategyType {
    if blocks == 2 {
        AcStrategyType::DCT16X8
    } else if blocks == 4 {
        AcStrategyType::DCT32X16
    } else {
        AcStrategyType::DCT64X32
    }
}

fn acs_horizontal_split(blocks: usize) -> AcStrategyType {
    if blocks == 2 {
        AcStrategyType::DCT8X16
    } else if blocks == 4 {
        AcStrategyType::DCT16X32
    } else {
        AcStrategyType::DCT32X64
    }
}

// The following function tries to merge smaller transforms into
// squares and the rectangles originating from a single middle division
// (horizontal or vertical) fairly.
//
// This is now generalized to concern about squares
// of blocks X blocks size, where a block is 8x8 pixels.
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(non_snake_case, reason = "keeps the source's JXK/KXJ/JXJ names")]
pub fn find_best_first_level_division_for_square(
    blocks: usize,
    allow_square_transform: bool,
    bx: usize,
    by: usize,
    cx: usize,
    cy: usize,
    config: &AcsConfig<'_>,
    cmap_factors: &[f32; 3],
    ac_strategy: &mut AcStrategyImage,
    entropy_mul_JXK: f32,
    entropy_mul_JXJ: f32,
    entropy_estimate: &mut [f32; 64],
    block: &mut [f32],
    scratch_space: &mut [f32],
    quantized: &mut [u32],
) -> Status {
    // We denote J for the larger dimension here, and K for the smaller.
    // For example, for 32x32 block splitting, J would be 32, K 16.
    let blocks_half = blocks / 2;
    let acs_rawJXK = acs_vertical_split(blocks);
    let acs_rawKXJ = acs_horizontal_split(blocks);
    let acs_rawJXJ = acs_square(blocks);
    let acsJXK = AcStrategy::from_raw_strategy(acs_rawJXK);
    let acsKXJ = AcStrategy::from_raw_strategy(acs_rawKXJ);
    let acsJXJ = AcStrategy::from_raw_strategy(acs_rawJXJ);
    let row0_y = by + cy;
    let row1_y = by + cy + blocks_half;
    // Let's check if we can consider a JXJ block here at all.
    // This is not necessary in the basic use of hierarchically merging
    // blocks in the simplest possible way, but is needed when we try other
    // 'floating' options of merging, possibly after a simple hierarchical
    // merge has been explored.
    if multi_block_transform_crosses_horizontal_boundary(
        ac_strategy,
        bx + cx,
        by + cy,
        bx + cx + blocks,
    ) || multi_block_transform_crosses_horizontal_boundary(
        ac_strategy,
        bx + cx,
        by + cy + blocks,
        bx + cx + blocks,
    ) || multi_block_transform_crosses_vertical_boundary(
        ac_strategy,
        bx + cx,
        by + cy,
        by + cy + blocks,
    ) || multi_block_transform_crosses_vertical_boundary(
        ac_strategy,
        bx + cx + blocks,
        by + cy,
        by + cy + blocks,
    ) {
        return Ok(()); // not suitable for JxJ analysis, some transforms leak out.
    }
    // For floating transforms there may be
    // already blocks selected that make either or both JXK and
    // KXJ not feasible for this location.
    let allow_JXK = !multi_block_transform_crosses_vertical_boundary(
        ac_strategy,
        bx + cx + blocks_half,
        by + cy,
        by + cy + blocks,
    );
    let allow_KXJ = !multi_block_transform_crosses_horizontal_boundary(
        ac_strategy,
        bx + cx,
        by + cy + blocks_half,
        bx + cx + blocks,
    );
    // Current entropies aggregated on NxN resolution.
    let mut entropy = [[0.0f32; 2]; 2];
    for dy in 0..blocks {
        for dx in 0..blocks {
            entropy[dy / blocks_half][dx / blocks_half] +=
                entropy_estimate[(cy + dy) * 8 + (cx + dx)];
        }
    }
    let mut entropy_JXK_left = f32::MAX;
    let mut entropy_JXK_right = f32::MAX;
    let mut entropy_KXJ_top = f32::MAX;
    let mut entropy_KXJ_bottom = f32::MAX;
    let mut entropy_JXJ = f32::MAX;
    if allow_JXK {
        if ac_strategy.const_row(row0_y, 0).get(bx + cx).strategy() != acs_rawJXK {
            estimate_entropy(
                &acsJXK,
                entropy_mul_JXK,
                (bx + cx) * 8,
                (by + cy) * 8,
                config,
                cmap_factors,
                block,
                scratch_space,
                quantized,
                &mut entropy_JXK_left,
            )?;
        }
        if ac_strategy
            .const_row(row0_y, 0)
            .get(bx + cx + blocks_half)
            .strategy()
            != acs_rawJXK
        {
            estimate_entropy(
                &acsJXK,
                entropy_mul_JXK,
                (bx + cx + blocks_half) * 8,
                (by + cy) * 8,
                config,
                cmap_factors,
                block,
                scratch_space,
                quantized,
                &mut entropy_JXK_right,
            )?;
        }
    }
    if allow_KXJ {
        if ac_strategy.const_row(row0_y, 0).get(bx + cx).strategy() != acs_rawKXJ {
            estimate_entropy(
                &acsKXJ,
                entropy_mul_JXK,
                (bx + cx) * 8,
                (by + cy) * 8,
                config,
                cmap_factors,
                block,
                scratch_space,
                quantized,
                &mut entropy_KXJ_top,
            )?;
        }
        if ac_strategy.const_row(row1_y, 0).get(bx + cx).strategy() != acs_rawKXJ {
            estimate_entropy(
                &acsKXJ,
                entropy_mul_JXK,
                (bx + cx) * 8,
                (by + cy + blocks_half) * 8,
                config,
                cmap_factors,
                block,
                scratch_space,
                quantized,
                &mut entropy_KXJ_bottom,
            )?;
        }
    }
    if allow_square_transform {
        // We control the exploration of the square transform separately so that
        // we can turn it off at high decoding speeds for 32x32, but still allow
        // exploring 16x32 and 32x16.
        estimate_entropy(
            &acsJXJ,
            entropy_mul_JXJ,
            (bx + cx) * 8,
            (by + cy) * 8,
            config,
            cmap_factors,
            block,
            scratch_space,
            quantized,
            &mut entropy_JXJ,
        )?;
    }

    // Test if this block should have JXK or KXJ transforms,
    // because it can have only one or the other.
    let cost_JxN = min_cpp(entropy_JXK_left, entropy[0][0] + entropy[1][0])
        + min_cpp(entropy_JXK_right, entropy[0][1] + entropy[1][1]);
    let cost_NxJ = min_cpp(entropy_KXJ_top, entropy[0][0] + entropy[0][1])
        + min_cpp(entropy_KXJ_bottom, entropy[1][0] + entropy[1][1]);
    if entropy_JXJ < cost_JxN && entropy_JXJ < cost_NxJ {
        ac_strategy.set(bx + cx, by + cy, acs_rawJXJ)?;
        set_entropy_for_transform(cx, cy, acs_rawJXJ, entropy_JXJ, entropy_estimate);
    } else if cost_JxN < cost_NxJ {
        if entropy_JXK_left < entropy[0][0] + entropy[1][0] {
            ac_strategy.set(bx + cx, by + cy, acs_rawJXK)?;
            set_entropy_for_transform(cx, cy, acs_rawJXK, entropy_JXK_left, entropy_estimate);
        }
        if entropy_JXK_right < entropy[0][1] + entropy[1][1] {
            ac_strategy.set(bx + cx + blocks_half, by + cy, acs_rawJXK)?;
            set_entropy_for_transform(
                cx + blocks_half,
                cy,
                acs_rawJXK,
                entropy_JXK_right,
                entropy_estimate,
            );
        }
    } else {
        if entropy_KXJ_top < entropy[0][0] + entropy[0][1] {
            ac_strategy.set(bx + cx, by + cy, acs_rawKXJ)?;
            set_entropy_for_transform(cx, cy, acs_rawKXJ, entropy_KXJ_top, entropy_estimate);
        }
        if entropy_KXJ_bottom < entropy[1][0] + entropy[1][1] {
            ac_strategy.set(bx + cx, by + cy + blocks_half, acs_rawKXJ)?;
            set_entropy_for_transform(
                cx,
                cy + blocks_half,
                acs_rawKXJ,
                entropy_KXJ_bottom,
                entropy_estimate,
            );
        }
    }
    Ok(())
}

// std::min(a, b): b < a ? b : a.
fn min_cpp(a: f32, b: f32) -> f32 {
    if b < a { b } else { a }
}

struct MergeTry {
    kind: AcStrategyType,
    priority: u8,
    decoding_speed_tier_max_limit: u8,
    #[expect(dead_code, reason = "the source declares and never reads this limit")]
    encoding_speed_tier_max_limit: u8,
    entropy_mul: f32,
}

fn process_rect_acs(
    cparams: &CompressParams<'_>,
    config: &AcsConfig<'_>,
    rect: Rect,
    cmap: &ColorCorrelationMap,
    memory: &mut [f32],
    quantized: &mut [u32],
    ac_strategy: &mut AcStrategyImage,
) -> Status {
    // Main philosophy here:
    // 1. First find best 8x8 transform for each area.
    // 2. Merging them into larger transforms where possibly, but
    // starting from the smallest transforms (16x8 and 8x16).
    // Additional complication: 16x8 and 8x16 are considered
    // simultaneously and fairly against each other.
    // We are looking at 64x64 squares since the Y-to-X and Y-to-B
    // maps happen to be at that resolution, and having
    // integral transforms cross these boundaries leads to
    // additional complications.
    let butteraugli_target = cparams.butteraugli_distance;
    let (block, scratch_space) = memory.split_at_mut(3 * MAX_COEFF_AREA);
    let bx = rect.x0();
    let by = rect.y0();
    if rect.xsize() > 8 || rect.ysize() > 8 {
        return Err(StatusCode::GenericError);
    }
    let tx = bx / COLOR_TILE_DIM_IN_BLOCKS;
    let ty = by / COLOR_TILE_DIM_IN_BLOCKS;
    let cmap_factors = [
        cmap.base()
            .y_to_x_ratio(i32::from(cmap.ytox_map.row(ty)[tx])),
        0.0f32,
        cmap.base()
            .y_to_b_ratio(i32::from(cmap.ytob_map.row(ty)[tx])),
    ];
    if cparams.speed_tier > SpeedTier::Hare {
        return Ok(());
    }
    // First compute the best 8x8 transform for each square. Later, we do not
    // experiment with different combinations, but only use the best of the 8x8s
    // when DCT8X8 is specified in the tree search.
    // 8x8 transforms have 10 variants, but every larger transform is just a DCT.
    let mut entropy_estimate = [0.0f32; 64];
    // Favor all 8x8 transforms (against 16x8 and larger transforms)) at
    // low butteraugli_target distances.
    let k8x8mul1 = -0.4f64 as f32;
    let k8x8mul2 = 1.0f32;
    let k8x8base = 1.4f64 as f32;
    let mul8x8 = k8x8mul2 + k8x8mul1 / (butteraugli_target + k8x8base);
    for iy in 0..rect.ysize() {
        for ix in 0..rect.xsize() {
            let mut entropy = 0.0f32;
            let mut best_of_8x8s = AcStrategyType::DCT;
            find_best_8x8_transform(
                8 * (bx + ix),
                8 * (by + iy),
                cparams.speed_tier as i32,
                butteraugli_target,
                config,
                &cmap_factors,
                ac_strategy,
                block,
                scratch_space,
                quantized,
                &mut entropy,
                &mut best_of_8x8s,
            )?;
            ac_strategy.set(bx + ix, by + iy, best_of_8x8s)?;
            entropy_estimate[iy * 8 + ix] = entropy * mul8x8;
        }
    }
    // Merge when a larger transform is better than the previously
    // searched best combination of 8x8 transforms.
    // These numbers need to be figured out manually and looking at
    // ringing next to sky etc. Optimization will find smaller numbers
    // and produce more ringing than is ideal. Larger numbers will
    // help stop ringing.
    let entropy_mul16x8 = 1.21f64 as f32;
    let entropy_mul16x16 = 1.34f64 as f32;
    let entropy_mul16x32 = 1.49f64 as f32;
    let entropy_mul32x32 = 1.48f64 as f32;
    let entropy_mul64x32 = 2.25f64 as f32;
    let entropy_mul64x64 = 2.25f64 as f32;
    // TODO(jyrki): Consider this feedback in further changes:
    // Also effectively when the multipliers for smaller blocks are
    // below 1, this raises the bar for the bigger blocks even higher
    // in that sense these constants are not independent (e.g. changing
    // the constant for DCT16x32 by -5% (making it more likely) also
    // means that DCT32x32 becomes harder to do when starting from
    // two DCT16x32s). It might be better to make them more independent,
    // e.g. by not applying the multiplier when storing the new entropy
    // estimates in TryMergeToACSCandidate().
    let transforms_for_merge: [MergeTry; 6] = [
        MergeTry {
            kind: AcStrategyType::DCT16X8,
            priority: 2,
            decoding_speed_tier_max_limit: 4,
            encoding_speed_tier_max_limit: 5,
            entropy_mul: entropy_mul16x8,
        },
        MergeTry {
            kind: AcStrategyType::DCT8X16,
            priority: 2,
            decoding_speed_tier_max_limit: 4,
            encoding_speed_tier_max_limit: 5,
            entropy_mul: entropy_mul16x8,
        },
        // FindBestFirstLevelDivisionForSquare looks for DCT16X16 and its
        // subdivisions. {AcStrategyType::DCT16X16, 3, entropy_mul16X16},
        MergeTry {
            kind: AcStrategyType::DCT16X32,
            priority: 4,
            decoding_speed_tier_max_limit: 4,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: entropy_mul16x32,
        },
        MergeTry {
            kind: AcStrategyType::DCT32X16,
            priority: 4,
            decoding_speed_tier_max_limit: 4,
            encoding_speed_tier_max_limit: 4,
            entropy_mul: entropy_mul16x32,
        },
        // FindBestFirstLevelDivisionForSquare looks for DCT32X32 and its
        // subdivisions. {AcStrategyType::DCT32X32, 5, 1, 5,
        // 0.9822994906548809f},
        MergeTry {
            kind: AcStrategyType::DCT64X32,
            priority: 6,
            decoding_speed_tier_max_limit: 1,
            encoding_speed_tier_max_limit: 3,
            entropy_mul: entropy_mul64x32,
        },
        MergeTry {
            kind: AcStrategyType::DCT32X64,
            priority: 6,
            decoding_speed_tier_max_limit: 1,
            encoding_speed_tier_max_limit: 3,
            entropy_mul: entropy_mul64x32,
        },
        // {AcStrategyType::DCT64X64, 8, 1, 3, 2.0846542128012948f},
    ];
    // The source declares the array with nine elements; the remaining three
    // are value-initialized: DCT, priority 0, limits 0 and multiplier 0.
    let zero_try = || MergeTry {
        kind: AcStrategyType::DCT,
        priority: 0,
        decoding_speed_tier_max_limit: 0,
        encoding_speed_tier_max_limit: 0,
        entropy_mul: 0.0,
    };
    let merge_tries = transforms_for_merge
        .into_iter()
        .chain([zero_try(), zero_try(), zero_try()]);
    // These sizes not yet included in merge heuristic:
    // set(AcStrategyType::DCT32X8, 0.0f, 2.261390410971102f);
    // set(AcStrategyType::DCT8X32, 0.0f, 2.261390410971102f);
    // set(AcStrategyType::DCT128X128, 0.0f, 1.0f);
    // set(AcStrategyType::DCT128X64, 0.0f, 0.73f);
    // set(AcStrategyType::DCT64X128, 0.0f, 0.73f);
    // set(AcStrategyType::DCT256X256, 0.0f, 1.0f);
    // set(AcStrategyType::DCT256X128, 0.0f, 0.73f);
    // set(AcStrategyType::DCT128X256, 0.0f, 0.73f);

    // Priority is a tricky kludge to avoid collisions so that transforms
    // don't overlap.
    let mut priority = [0u8; 64];
    let enable_32x32 = cparams.decoding_speed_tier < 4;
    for mt in merge_tries {
        if usize::from(mt.decoding_speed_tier_max_limit) < cparams.decoding_speed_tier {
            continue;
        }
        let acs = AcStrategy::from_raw_strategy(mt.kind);

        let mut cy = 0;
        while cy + acs.covered_blocks_y() - 1 < rect.ysize() {
            let mut cx = 0;
            while cx + acs.covered_blocks_x() - 1 < rect.xsize() {
                let next_cx = cx + acs.covered_blocks_x();
                'candidate: {
                    if cy + 7 < rect.ysize() && cx + 7 < rect.xsize() {
                        if cparams.decoding_speed_tier < 4 && mt.kind == AcStrategyType::DCT32X64 {
                            // We handle both DCT8X16 and DCT16X8 at the same time.
                            if (cy | cx) % 8 == 0 {
                                find_best_first_level_division_for_square(
                                    8,
                                    true,
                                    bx,
                                    by,
                                    cx,
                                    cy,
                                    config,
                                    &cmap_factors,
                                    ac_strategy,
                                    mt.entropy_mul,
                                    entropy_mul64x64,
                                    &mut entropy_estimate,
                                    block,
                                    scratch_space,
                                    quantized,
                                )?;
                            }
                            break 'candidate;
                        } else if mt.kind == AcStrategyType::DCT32X16 {
                            // We handled both DCT8X16 and DCT16X8 at the same time,
                            // and that is above. The last column and last row,
                            // when the last column or last row is odd numbered,
                            // are still handled by TryMergeAcs.
                            break 'candidate;
                        }
                    }
                    if (mt.kind == AcStrategyType::DCT16X32 && cy % 4 != 0)
                        || (mt.kind == AcStrategyType::DCT32X16 && cx % 4 != 0)
                    {
                        // already covered by FindBest32X32
                        break 'candidate;
                    }

                    if cy + 3 < rect.ysize() && cx + 3 < rect.xsize() {
                        if mt.kind == AcStrategyType::DCT16X32 {
                            // We handle both DCT8X16 and DCT16X8 at the same time.
                            if (cy | cx) % 4 == 0 {
                                find_best_first_level_division_for_square(
                                    4,
                                    enable_32x32,
                                    bx,
                                    by,
                                    cx,
                                    cy,
                                    config,
                                    &cmap_factors,
                                    ac_strategy,
                                    mt.entropy_mul,
                                    entropy_mul32x32,
                                    &mut entropy_estimate,
                                    block,
                                    scratch_space,
                                    quantized,
                                )?;
                            }
                            break 'candidate;
                        } else if mt.kind == AcStrategyType::DCT32X16 {
                            // We handled both DCT8X16 and DCT16X8 at the same time,
                            // and that is above. The last column and last row,
                            // when the last column or last row is odd numbered,
                            // are still handled by TryMergeAcs.
                            break 'candidate;
                        }
                    }
                    if (mt.kind == AcStrategyType::DCT16X32 && cy % 4 != 0)
                        || (mt.kind == AcStrategyType::DCT32X16 && cx % 4 != 0)
                    {
                        // already covered by FindBest32X32
                        break 'candidate;
                    }
                    if cy + 1 < rect.ysize() && cx + 1 < rect.xsize() {
                        if mt.kind == AcStrategyType::DCT8X16 {
                            // We handle both DCT8X16 and DCT16X8 at the same time.
                            if (cy | cx) % 2 == 0 {
                                find_best_first_level_division_for_square(
                                    2,
                                    true,
                                    bx,
                                    by,
                                    cx,
                                    cy,
                                    config,
                                    &cmap_factors,
                                    ac_strategy,
                                    mt.entropy_mul,
                                    entropy_mul16x16,
                                    &mut entropy_estimate,
                                    block,
                                    scratch_space,
                                    quantized,
                                )?;
                            }
                            break 'candidate;
                        } else if mt.kind == AcStrategyType::DCT16X8 {
                            // We handled both DCT8X16 and DCT16X8 at the same time,
                            // and that is above. The last column and last row,
                            // when the last column or last row is odd numbered,
                            // are still handled by TryMergeAcs.
                            break 'candidate;
                        }
                    }
                    if (mt.kind == AcStrategyType::DCT8X16 && cy % 2 == 1)
                        || (mt.kind == AcStrategyType::DCT16X8 && cx % 2 == 1)
                    {
                        // already covered by FindBestFirstLevelDivisionForSquare
                        break 'candidate;
                    }
                    // All other merge sizes are handled here.
                    // Some of the DCT16X8s and DCT8X16s will still leak through here
                    // when there is an odd number of 8x8 blocks, then the last row
                    // and column will get their DCT16X8s and DCT8X16s through the
                    // normal integral transform merging process.
                    try_merge_acs(
                        mt.kind,
                        bx,
                        by,
                        cx,
                        cy,
                        config,
                        &cmap_factors,
                        ac_strategy,
                        mt.entropy_mul,
                        mt.priority,
                        &mut priority,
                        &mut entropy_estimate,
                        block,
                        scratch_space,
                        quantized,
                    )?;
                }
                cx = next_cx;
            }
            cy += acs.covered_blocks_y();
        }
    }
    if cparams.speed_tier >= SpeedTier::Hare {
        return Ok(());
    }
    // Here we still try to do some non-aligned matching, find a few more
    // 16X8, 8X16 and 16X16s between the non-2-aligned blocks.
    let mut cy = 0;
    while cy + 1 < rect.ysize() {
        let mut cx = 0;
        while cx + 1 < rect.xsize() {
            if (cy | cx) % 2 != 0 {
                find_best_first_level_division_for_square(
                    2,
                    true,
                    bx,
                    by,
                    cx,
                    cy,
                    config,
                    &cmap_factors,
                    ac_strategy,
                    entropy_mul16x8,
                    entropy_mul16x16,
                    &mut entropy_estimate,
                    block,
                    scratch_space,
                    quantized,
                )?;
            }
            cx += 1;
        }
        cy += 1;
    }
    // Non-aligned matching for 32X32, 16X32 and 32X16.
    let step = if cparams.speed_tier >= SpeedTier::Tortoise {
        2
    } else {
        1
    };
    let mut cy = 0;
    while cy + 3 < rect.ysize() {
        let mut cx = 0;
        while cx + 3 < rect.xsize() {
            if (cy | cx) % 4 != 0 {
                find_best_first_level_division_for_square(
                    4,
                    enable_32x32,
                    bx,
                    by,
                    cx,
                    cy,
                    config,
                    &cmap_factors,
                    ac_strategy,
                    entropy_mul16x32,
                    entropy_mul32x32,
                    &mut entropy_estimate,
                    block,
                    scratch_space,
                    quantized,
                )?;
            }
            cx += step;
        }
        cy += step;
    }
    Ok(())
}

pub struct AcStrategyHeuristics<'c, 'p> {
    pub memory_manager: MemoryManagerPtr,
    pub cparams: &'c CompressParams<'p>,
    pub config: AcsConfigScalars,
    pub mem_per_thread: usize,
    pub(crate) mem: Option<AlignedMemory>,
    pub qmem_per_thread: usize,
    pub(crate) qmem: Option<AlignedMemory>,
}

impl<'c, 'p> AcStrategyHeuristics<'c, 'p> {
    pub fn new(memory_manager: MemoryManagerPtr, cparams: &'c CompressParams<'p>) -> Self {
        Self {
            memory_manager,
            cparams,
            config: AcsConfigScalars::default(),
            mem_per_thread: 0,
            mem: None,
            qmem_per_thread: 0,
            qmem: None,
        }
    }

    #[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
    /// Init. The source keeps row pointers to `src`, `quant_field`, `mask`,
    /// `mask1x1` and `matrices`; ProcessRect receives those owners again.
    pub fn init(
        &mut self,
        src: &Image3F,
        _rect_in: Rect,
        quant_field: &ImageF,
        mask: &ImageF,
        mask1x1: &ImageF,
        matrices: &mut DequantMatrices,
    ) -> Status {
        if self.cparams.speed_tier >= SpeedTier::Cheetah {
            matrices.ensure_computed_in(self.memory_manager, 1)?; // DCT8 only
        } else {
            let mut acs_mask = 0u32;
            // All transforms up to 64x64.
            for i in 0..AcStrategyType::DCT128X128.0 {
                acs_mask |= 1 << i;
            }
            matrices.ensure_computed_in(self.memory_manager, acs_mask)?;
        }

        // Image row pointers and strides.
        self.config.quant_field_stride = quant_field.pixels_per_row() as usize;
        if mask.xsize() > 0 && mask.ysize() > 0 {
            self.config.masking_field_stride = mask.pixels_per_row() as usize;
        }
        self.config.mask1x1_xsize = mask1x1.xsize();
        if mask1x1.xsize() > 0 && mask1x1.ysize() > 0 {
            self.config.masking1x1_field_stride = mask1x1.pixels_per_row() as usize;
        }

        self.config.src_stride = src.plane(0).pixels_per_row() as usize;

        // Entropy estimate is composed of two factors:
        //  - estimate of the number of bits that will be used by the block
        //  - information loss due to quantization
        // The following constant controls the relative weights of these
        // components.
        self.config.info_loss_multiplier = 1.2f64 as f32;
        self.config.zeros_mul = 9.3089059022677905f64 as f32;
        self.config.cost_delta = 10.833273317067883f64 as f32;

        let bias = 0.13731742964354549f64 as f32;
        let ratio = (self.cparams.butteraugli_distance + bias) / (1.0f32 + bias);

        let pow1 = 0.33677806662454718f64 as f32;
        let pow2 = 0.50990926717963703f64 as f32;
        let pow3 = 0.36702940662370243f64 as f32;
        self.config.info_loss_multiplier *= ratio.powf(pow1);
        self.config.zeros_mul *= ratio.powf(pow2);
        self.config.cost_delta *= ratio.powf(pow3);
        Ok(())
    }

    pub fn prepare_for_threads(&mut self, num_threads: usize) -> Status {
        let dct_scratch_size = 3 * (MAX_VECTOR_SIZE / size_of::<f32>()) * MAX_BLOCK_DIM;
        self.mem_per_thread = 6 * MAX_COEFF_AREA + dct_scratch_size;
        let mem_bytes = num_threads * self.mem_per_thread * size_of::<f32>();
        self.mem = Some(AlignedMemory::create(self.memory_manager, mem_bytes, 0)?);
        self.qmem_per_thread = MAX_COEFF_AREA;
        let qmem_bytes = num_threads * self.qmem_per_thread * size_of::<u32>();
        self.qmem = Some(AlignedMemory::create(self.memory_manager, qmem_bytes, 0)?);
        Ok(())
    }

    /// ProcessRect over the owners that Init received.
    pub fn process_rect(
        &mut self,
        rect: Rect,
        cmap: &ColorCorrelationMap,
        ac_strategy: &mut AcStrategyImage,
        thread: usize,
        planes: &AcsPlanes<'_>,
    ) -> Status {
        // In Cheetah mode, use DCT8 everywhere and uniform quantization.
        if self.cparams.speed_tier >= SpeedTier::Cheetah {
            ac_strategy.fill_dct8(rect);
            return Ok(());
        }
        let config = AcsConfig::new(&self.config, planes);
        let mem_per_thread = self.mem_per_thread;
        let qmem_per_thread = self.qmem_per_thread;
        let mem = self
            .mem
            .as_mut()
            .ok_or(StatusCode::GenericError)?
            .f32s_mut();
        let qmem = self
            .qmem
            .as_mut()
            .ok_or(StatusCode::GenericError)?
            .u32s_mut();
        process_rect_acs(
            self.cparams,
            &config,
            rect,
            cmap,
            &mut mem[thread * mem_per_thread..(thread + 1) * mem_per_thread],
            &mut qmem[thread * qmem_per_thread..(thread + 1) * qmem_per_thread],
            ac_strategy,
        )
    }

    pub fn finalize(
        &self,
        _frame_dim: &FrameDimensions,
        ac_strategy: &AcStrategyImage,
        aux_out: Option<&mut AuxOut>,
    ) -> Status {
        // Accounting and debug output.
        if let Some(aux_out) = aux_out {
            aux_out.num_small_blocks = ac_strategy.count_blocks(AcStrategyType::IDENTITY)
                + ac_strategy.count_blocks(AcStrategyType::DCT2X2)
                + ac_strategy.count_blocks(AcStrategyType::DCT4X4);
            aux_out.num_dct4x8_blocks = ac_strategy.count_blocks(AcStrategyType::DCT4X8)
                + ac_strategy.count_blocks(AcStrategyType::DCT8X4);
            aux_out.num_afv_blocks = ac_strategy.count_blocks(AcStrategyType::AFV0)
                + ac_strategy.count_blocks(AcStrategyType::AFV1)
                + ac_strategy.count_blocks(AcStrategyType::AFV2)
                + ac_strategy.count_blocks(AcStrategyType::AFV3);
            aux_out.num_dct8_blocks = ac_strategy.count_blocks(AcStrategyType::DCT);
            aux_out.num_dct8x16_blocks = ac_strategy.count_blocks(AcStrategyType::DCT8X16)
                + ac_strategy.count_blocks(AcStrategyType::DCT16X8);
            aux_out.num_dct8x32_blocks = ac_strategy.count_blocks(AcStrategyType::DCT8X32)
                + ac_strategy.count_blocks(AcStrategyType::DCT32X8);
            aux_out.num_dct16_blocks = ac_strategy.count_blocks(AcStrategyType::DCT16X16);
            aux_out.num_dct16x32_blocks = ac_strategy.count_blocks(AcStrategyType::DCT16X32)
                + ac_strategy.count_blocks(AcStrategyType::DCT32X16);
            aux_out.num_dct32_blocks = ac_strategy.count_blocks(AcStrategyType::DCT32X32);
            aux_out.num_dct32x64_blocks = ac_strategy.count_blocks(AcStrategyType::DCT32X64)
                + ac_strategy.count_blocks(AcStrategyType::DCT64X32);
            aux_out.num_dct64_blocks = ac_strategy.count_blocks(AcStrategyType::DCT64X64);
        }
        // JXL_DEBUG_AC_STRATEGY is 0 at the pinned revision.
        Ok(())
    }
}
