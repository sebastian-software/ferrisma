// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The `HWY_NAMESPACE` part of lib/jxl/enc_ac_strategy.cc (the
//! `ProcessRectACS` export) for the NEON and AVX2 targets (port ADR 0004,
//! "Encoder path"), written once against [`SimdF32`]: EstimateEntropy,
//! FindBest8x8Transform, TryMergeAcs, FindBestFirstLevelDivisionForSquare
//! and ProcessRectACS. MultiBlockTransformCrossesHorizontalBoundary,
//! MultiBlockTransformCrossesVerticalBoundary, SetEntropyForTransform and
//! AcsSquare/AcsVerticalSplit/AcsHorizontalSplit have no floating-point or
//! vector code; they are the scalar translation's (`enc_ac_strategy`).
//! Inside `HWY_NAMESPACE` the transforms are the in-namespace SIMD
//! TransformFromPixels (`enc_transforms_simd`) and TransformToPixels
//! (`dec_transforms_simd`), and FastPowf is fast_math-inl.h's one-lane
//! vector form (`simd_transfer`). Compare these functions only with the
//! original's SIMD build at the same target; the scalar (HWY_SCALAR)
//! translation stays in `enc_ac_strategy`.
//!
//! `HWY_FULL(float)` and `HWY_CAPPED(float, 8)` are the same descriptor on
//! both targets (four NEON, eight AVX2 lanes), so EstimateEntropy's masking
//! loop uses `D` as well; its bound `x + ix * 8 + dx + Lanes(df8) <=
//! mask1x1_xsize` and the vector accumulators with their `SumOfLanes`
//! follow the lane count, so NEON and AVX2 give different words by design.
//!
//! The scalar statements inside `HWY_NAMESPACE` carry FMA on both targets
//! (the target attributes include it), so the contraction sites that the
//! scalar translation fuses only on AArch64 (`mul_add_f32`) are fused here
//! on x86-64 too: EstimateEntropy's `quant_norm16 += qval * qval` and its
//! three `entropy += a * b` statements, FindBest8x8Transform's two
//! `entropy_mul` adjustments, and the inlined `YtoXRatio`/`YtoBRatio` of
//! ProcessRectACS (`y_to_x_ratio_simd`, as in the SIMD ComputeCoefficients).
//! `pow(float, float)` is the host `powf` and the static `pow(double,
//! double)` channel weights the host `pow`, as in the scalar translation.
use super::ac_strategy::{AcStrategy, AcStrategyImage, AcStrategyType, MAX_COEFF_AREA};
use super::base::bits::ceil_log2_nonzero;
use super::base::float::max_f32;
use super::base::status::Status;
use super::dec_transforms_simd::transform_to_pixels_d;
use super::enc_ac_strategy::{
    AcsConfig, acs_horizontal_split, acs_square, acs_vertical_split, min_cpp,
    multi_block_transform_crosses_horizontal_boundary,
    multi_block_transform_crosses_vertical_boundary, set_entropy_for_transform,
};
use super::enc_transforms_simd::transform_from_pixels_d;
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::render_pipeline::simd_transfer::fast_powf;

/// EstimateEntropy; the contract of
/// [`super::enc_ac_strategy::estimate_entropy`].
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[inline(always)]
pub fn estimate_entropy_d<D: SimdF32>(
    d: D,
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
        transform_from_pixels_d(
            d,
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
                // `quant_norm16 += qval * qval`, contracted on both targets.
                quant_norm16 = qval.mul_add(qval, quant_norm16);
            }
        }
        quant_norm16 /= num_blocks as f32;
        // The in-namespace FastPowf(float, float): HWY_CAPPED(float, 1).
        quant_norm16 = d.get_lane(fast_powf(d, d.set(quant_norm16), d.set(1.0f32 / 16.0f32)));
    }
    let quant = d.set(quant_norm16);

    // Compute entropy.
    // HWY_CAPPED(float, 8) is `d` on both targets (see the module comment).
    let mut loss = d.zero();
    for c in 0..3 {
        let inv_matrix = config.dequant.inv_matrix(acs.strategy(), c);
        let matrix = config.dequant.matrix(acs.strategy(), c);
        let cmap_factor = d.set(cmap_factors[c]);

        let mut entropy_v = d.zero();
        let mut nzeros_v = d.zero();
        let mut i = 0;
        while i < num_blocks * DCT_BLOCK_SIZE {
            let input = d.load(&block[c * size + i..]);
            let in_y = d.mul(d.load(&block[size + i..]), cmap_factor);
            let im = d.load(&inv_matrix[i..]);
            let val = d.mul(d.sub(input, in_y), d.mul(im, quant));
            let rval = d.round(val);
            let diff = d.sub(val, rval);
            let m = d.load(&matrix[i..]);
            d.store(d.mul(m, diff), &mut mem[i..]);
            let q = d.abs(rval);
            let q_is_zero = d.eq(q, d.zero());
            // We used to have q * C here, but that cost model seems to
            // be punishing large values more than necessary. Sqrt tries
            // to avoid large values less aggressively.
            entropy_v = d.add(d.sqrt(q), entropy_v);
            nzeros_v = d.add(nzeros_v, d.if_then_zero_else(q_is_zero, d.set(1.0)));
            i += D::LANES;
        }

        {
            let masku_lut = [12.0f32, 0.0, 4.0];
            let masku_off = d.set(masku_lut[c]);
            let mut lossc = d.zero();
            transform_to_pixels_d(
                d,
                acs.strategy(),
                mem,
                block,
                acs.covered_blocks_x() * 8,
                scratch_space,
            );

            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..acs.covered_blocks_x() {
                    for dy in 0..BLOCK_DIM {
                        let mut dx = 0;
                        while dx < BLOCK_DIM {
                            let mut input = d.load(
                                &block[(iy * BLOCK_DIM + dy)
                                    * (acs.covered_blocks_x() * BLOCK_DIM)
                                    + ix * BLOCK_DIM
                                    + dx..],
                            );
                            if x + ix * 8 + dx + D::LANES <= config.mask1x1_xsize {
                                let masku = d.add(
                                    d.load(config.masking_ptr1x1(x + ix * 8 + dx, y + iy * 8 + dy)),
                                    masku_off,
                                );
                                input = d.mul(masku, input);
                                input = d.mul(input, input);
                                input = d.mul(input, input);
                                input = d.mul(input, input);
                                lossc = d.add(lossc, input);
                            }
                            dx += D::LANES;
                        }
                    }
                }
            }
            let channel_mul = [8.2f64.powf(8.0), 1.0f64.powf(8.0), 1.03f64.powf(8.0)];
            lossc = d.mul(d.set(channel_mul[c] as f32), lossc);
            loss = d.add(loss, lossc);
        }
        // `entropy += config.cost_delta * GetLane(...)`, contracted.
        *entropy = config
            .cost_delta
            .mul_add(d.get_lane(d.sum_of_lanes(entropy_v)), *entropy);
        let num_nzeros = d.get_lane(d.sum_of_lanes(nzeros_v)) as usize;
        // Add #bit of num_nonzeros, as an estimate of the cost for encoding the
        // number of non-zeros of the block.
        let nbits = ceil_log2_nonzero(num_nzeros + 1) + 1;
        // Also add #bit of #bit of num_nonzeros, to estimate the ANS cost, with a
        // bias.
        *entropy = config
            .zeros_mul
            .mul_add((ceil_log2_nonzero(nbits + 17) + nbits) as f32, *entropy);
        if c == 0 && num_blocks >= 2 {
            // It is X channel (red-green) and we often see ringing
            // in the large blocks. Let's punish that more here.
            let w = (1.0 + 3.0f64.min(num_blocks as f64 / 8.0)) as f32;
            *entropy *= w;
            loss = d.mul(loss, d.set(w));
        }
    }
    let loss_scalar = (d.get_lane(d.sum_of_lanes(loss)) / (num_blocks * DCT_BLOCK_SIZE) as f32)
        .powf(1.0f32 / 8.0)
        * (num_blocks * DCT_BLOCK_SIZE) as f32
        / quant_norm16;
    *entropy *= entropy_mul;
    *entropy = config.info_loss_multiplier.mul_add(loss_scalar, *entropy);
    Ok(())
}

/// The arguments of EstimateEntropy after the strategy.
pub struct EntropyArgs<'a, 'c> {
    pub entropy_mul: f32,
    pub x: usize,
    pub y: usize,
    pub config: &'a AcsConfig<'c>,
    pub cmap_factors: &'a [f32; 3],
    pub block: &'a mut [f32],
    pub full_scratch_space: &'a mut [f32],
    pub quantized: &'a mut [u32],
    pub entropy: &'a mut f32,
}
struct Entropy<'a, 'c> {
    acs: &'a AcStrategy,
    args: EntropyArgs<'a, 'c>,
}
impl Kernel for Entropy<'_, '_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        let a = self.args;
        estimate_entropy_d(
            d,
            self.acs,
            a.entropy_mul,
            a.x,
            a.y,
            a.config,
            a.cmap_factors,
            a.block,
            a.full_scratch_space,
            a.quantized,
            a.entropy,
        )
    }
}

/// `EstimateEntropy` of the target namespace (external linkage in the
/// original, not an export) instantiated for one SIMD target.
pub fn estimate_entropy_simd(
    target: Target,
    acs: &AcStrategy,
    args: EntropyArgs<'_, '_>,
) -> Status {
    dispatch(target, Entropy { acs, args })
}

struct TransformTry8x8 {
    kind: AcStrategyType,
    encoding_speed_tier_max_limit: i32,
    entropy_mul: f64,
}

#[expect(clippy::excessive_precision, reason = "the source's decimal literals")]
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

/// FindBest8x8Transform; the contract of
/// [`super::enc_ac_strategy::find_best_8x8_transform`].
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[inline(always)]
pub fn find_best_8x8_transform_d<D: SimdF32>(
    d: D,
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
            // `entropy_mul -= kFavor2X2AtHighQuality * weight`, contracted on
            // both targets.
            entropy_mul = (-favor_2x2_at_high_quality).mul_add(weight, entropy_mul);
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
            // `entropy_mul += kAvoidEntropyOfTransforms * mul`, contracted.
            entropy_mul = avoid_entropy_of_transforms.mul_add(mul, entropy_mul);
        }
        let mut entropy = 0.0f32;
        estimate_entropy_d(
            d,
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

/// The arguments of FindBest8x8Transform.
pub struct Best8x8Args<'a, 'c> {
    pub x: usize,
    pub y: usize,
    pub encoding_speed_tier: i32,
    pub butteraugli_target: f32,
    pub config: &'a AcsConfig<'c>,
    pub cmap_factors: &'a [f32; 3],
    pub ac_strategy: &'a mut AcStrategyImage,
    pub block: &'a mut [f32],
    pub scratch_space: &'a mut [f32],
    pub quantized: &'a mut [u32],
    pub entropy_out: &'a mut f32,
    pub best_tx: &'a mut AcStrategyType,
}
struct Best8x8<'a, 'c>(Best8x8Args<'a, 'c>);
impl Kernel for Best8x8<'_, '_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        let a = self.0;
        find_best_8x8_transform_d(
            d,
            a.x,
            a.y,
            a.encoding_speed_tier,
            a.butteraugli_target,
            a.config,
            a.cmap_factors,
            a.ac_strategy,
            a.block,
            a.scratch_space,
            a.quantized,
            a.entropy_out,
            a.best_tx,
        )
    }
}

/// `FindBest8x8Transform` of the target namespace instantiated for one
/// SIMD target.
pub fn find_best_8x8_transform_simd(target: Target, args: Best8x8Args<'_, '_>) -> Status {
    dispatch(target, Best8x8(args))
}

// bx, by addresses the 64x64 block at 8x8 subresolution
// cx, cy addresses the left, upper 8x8 block position of the candidate
// transform.
/// TryMergeAcs; the contract of [`super::enc_ac_strategy::try_merge_acs`].
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[inline(always)]
pub fn try_merge_acs_d<D: SimdF32>(
    d: D,
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
    estimate_entropy_d(
        d,
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

// The following function tries to merge smaller transforms into
// squares and the rectangles originating from a single middle division
// (horizontal or vertical) fairly.
//
// This is now generalized to concern about squares
// of blocks X blocks size, where a block is 8x8 pixels.
/// FindBestFirstLevelDivisionForSquare; the contract of
/// [`super::enc_ac_strategy::find_best_first_level_division_for_square`].
#[expect(clippy::too_many_arguments, reason = "mirrors the source signature")]
#[expect(non_snake_case, reason = "keeps the source's JXK/KXJ/JXJ names")]
#[inline(always)]
pub fn find_best_first_level_division_for_square_d<D: SimdF32>(
    d: D,
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
            estimate_entropy_d(
                d,
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
            estimate_entropy_d(
                d,
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
            estimate_entropy_d(
                d,
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
            estimate_entropy_d(
                d,
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
        estimate_entropy_d(
            d,
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

/// The block-level state the merge steps of ProcessRectACS update: the
/// tile's origin, its entropy estimates and the strategy image.
pub struct MergeState<'a, 'c> {
    pub bx: usize,
    pub by: usize,
    pub config: &'a AcsConfig<'c>,
    pub cmap_factors: &'a [f32; 3],
    pub ac_strategy: &'a mut AcStrategyImage,
    pub entropy_estimate: &'a mut [f32; 64],
    pub block: &'a mut [f32],
    pub scratch_space: &'a mut [f32],
    pub quantized: &'a mut [u32],
}

/// The arguments of TryMergeAcs besides the [`MergeState`].
pub struct TryMergeArgs<'a> {
    pub acs_raw: AcStrategyType,
    pub cx: usize,
    pub cy: usize,
    pub entropy_mul: f32,
    pub candidate_priority: u8,
    pub priority: &'a mut [u8; 64],
}
struct TryMerge<'a, 'c>(MergeState<'a, 'c>, TryMergeArgs<'a>);
impl Kernel for TryMerge<'_, '_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        let (s, a) = (self.0, self.1);
        try_merge_acs_d(
            d,
            a.acs_raw,
            s.bx,
            s.by,
            a.cx,
            a.cy,
            s.config,
            s.cmap_factors,
            s.ac_strategy,
            a.entropy_mul,
            a.candidate_priority,
            a.priority,
            s.entropy_estimate,
            s.block,
            s.scratch_space,
            s.quantized,
        )
    }
}

/// `TryMergeAcs` of the target namespace instantiated for one SIMD target.
pub fn try_merge_acs_simd(
    target: Target,
    state: MergeState<'_, '_>,
    args: TryMergeArgs<'_>,
) -> Status {
    dispatch(target, TryMerge(state, args))
}

/// The arguments of FindBestFirstLevelDivisionForSquare besides the
/// [`MergeState`].
#[expect(non_snake_case, reason = "keeps the source's JXK/JXJ names")]
pub struct SquareArgs {
    pub blocks: usize,
    pub allow_square_transform: bool,
    pub cx: usize,
    pub cy: usize,
    pub entropy_mul_JXK: f32,
    pub entropy_mul_JXJ: f32,
}
struct Square<'a, 'c>(MergeState<'a, 'c>, SquareArgs);
impl Kernel for Square<'_, '_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        let (s, a) = (self.0, self.1);
        find_best_first_level_division_for_square_d(
            d,
            a.blocks,
            a.allow_square_transform,
            s.bx,
            s.by,
            a.cx,
            a.cy,
            s.config,
            s.cmap_factors,
            s.ac_strategy,
            a.entropy_mul_JXK,
            a.entropy_mul_JXJ,
            s.entropy_estimate,
            s.block,
            s.scratch_space,
            s.quantized,
        )
    }
}

/// `FindBestFirstLevelDivisionForSquare` of the target namespace
/// instantiated for one SIMD target.
pub fn find_best_first_level_division_for_square_simd(
    target: Target,
    state: MergeState<'_, '_>,
    args: SquareArgs,
) -> Status {
    dispatch(target, Square(state, args))
}
