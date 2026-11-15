// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! NumNonZeroExceptLLF, NumNonZero8x8ExceptDC and TokenizeCoefficients of
//! lib/jxl/enc_entropy_coder.cc for the NEON and AVX2 targets (port ADR 0004,
//! "Encoder path"), written once against [`SimdF32`]'s signed lanes
//! (`HWY_CAPPED(int32_t, kBlockDim)`: four NEON, eight AVX2 lanes, the full
//! vectors). `enc_entropy_coder` keeps the scalar (one-lane) translation and
//! calls [`tokenize_coefficients_simd`] when the encoder selected a SIMD
//! target, as `HWY_DYNAMIC_DISPATCH(TokenizeCoefficients)` does.
//!
//! The counters add `VecFromMask(Eq(coef, zero))` (all-ones, -1, per zero
//! coefficient) lane by lane and reduce with `SumOfLanes`; 32-bit wrapping
//! integer sums are exact in any order, so the integer `SumOfLanes` is formed
//! from the lanes. The LLF mask is the source's `HWY_ALIGN` array of
//! `kMaxCoeffBlocks * (1 + kBlockDim)` words of which only the first four are
//! -1, read from `kMaxCoeffBlocks - cx` onwards, and the DC mask is
//! `{-1, 0, ...}`. The control flow of TokenizeCoefficients is scalar code
//! inside `HWY_NAMESPACE` with integer arithmetic only (no contraction site);
//! it is `enc_entropy_coder`'s.

use super::ac_context::{BlockCtxMap, zero_density_context};
use super::ac_strategy::{AcStrategy, AcStrategyImage, MAX_COEFF_BLOCKS};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::coeff_order::{STRATEGY_ORDER, coeff_order_offset};
use super::coeff_order_fwd::{CoeffOrder, coefficient_layout};
use super::enc_ans::Token;
use super::entropy_coder::predict_from_top_and_left;
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE};
use super::frame_header::YCbCrChromaSubsampling;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::{Image3, Plane};
use super::pack_signed::pack_signed;

/// `GetLane(SumOfLanes(di, v))` of 32-bit lanes (wrapping, exact in any
/// order).
#[inline(always)]
fn sum_of_lanes_i32<D: SimdF32>(di: D, v: D::I) -> i32 {
    let mut lanes = [0i32; 16];
    di.store_i32(v, &mut lanes[..D::LANES]);
    lanes[..D::LANES]
        .iter()
        .fold(0i32, |sum, &x| sum.wrapping_add(x))
}

/// Returns the number of non-zero coefficients, skipping the LLF, and writes
/// the per-8x8 block counts starting at `nzeros_pos` in `nzeros`.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn num_non_zero_except_llf_d<D: SimdF32>(
    di: D,
    cx: usize,
    cy: usize,
    acs: AcStrategy,
    covered_blocks: usize,
    log2_covered_blocks: usize,
    block: &[i32],
    nzeros_stride: usize,
    nzeros: &mut [i32],
    nzeros_pos: usize,
) -> i32 {
    let zero = di.set_i32(0);
    // Add FF..FF for every zero coefficient, negate to get #zeros.
    let mut neg_sum_zero = zero;
    {
        // Mask sufficient for one row of coefficients.
        let mut llf_mask_lanes = [0i32; MAX_COEFF_BLOCKS * (1 + BLOCK_DIM)];
        llf_mask_lanes[..4].fill(-1);
        // First cx=1,2,4 elements are FF..FF, others 0.
        let llf_mask_pos = MAX_COEFF_BLOCKS - cx;

        // Rows with LLF: mask out the LLF
        for y in 0..cy {
            let mut x = 0;
            while x < cx * BLOCK_DIM {
                let llf_mask = di.load_i32(&llf_mask_lanes[llf_mask_pos + x..]);
                // LLF counts as zero so we don't include it in nzeros.
                let coef = di.bitcast_to_i32(di.and_not(
                    di.bitcast_to_f32(llf_mask),
                    di.bitcast_to_f32(di.load_i32(&block[y * cx * BLOCK_DIM + x..])),
                ));
                neg_sum_zero = di.add_i32(neg_sum_zero, di.eq_i32(coef, zero));
                x += D::LANES;
            }
        }
    }
    // Remaining rows: no mask
    for y in cy..cy * BLOCK_DIM {
        let mut x = 0;
        while x < cx * BLOCK_DIM {
            let coef = di.load_i32(&block[y * cx * BLOCK_DIM + x..]);
            neg_sum_zero = di.add_i32(neg_sum_zero, di.eq_i32(coef, zero));
            x += D::LANES;
        }
    }
    // We want area - sum_zero, add because neg_sum_zero is already negated.
    let count =
        ((cx * cy * DCT_BLOCK_SIZE) as i32).wrapping_add(sum_of_lanes_i32(di, neg_sum_zero));
    let shifted_nzeros =
        ((count as usize).wrapping_add(covered_blocks - 1) >> log2_covered_blocks) as i32;
    // Need non-canonicalized dimensions!
    for y in 0..acs.covered_blocks_y() {
        for x in 0..acs.covered_blocks_x() {
            nzeros[nzeros_pos + x + y * nzeros_stride] = shifted_nzeros;
        }
    }
    count
}

/// Specialization for 8x8, where only top-left is LLF/DC.
pub fn num_non_zero_8x8_except_dc_d<D: SimdF32>(di: D, block: &[i32], nzeros_pos: &mut i32) -> i32 {
    let zero = di.set_i32(0);
    let mut neg_sum_zero = zero;
    {
        // First row has DC, so mask
        let mut dc_mask_lanes = [0i32; BLOCK_DIM];
        dc_mask_lanes[0] = -1;
        let mut x = 0;
        while x < BLOCK_DIM {
            let dc_mask = di.load_i32(&dc_mask_lanes[x..]);
            // DC counts as zero so we don't include it in nzeros.
            let coef = di.bitcast_to_i32(di.and_not(
                di.bitcast_to_f32(dc_mask),
                di.bitcast_to_f32(di.load_i32(&block[x..])),
            ));
            neg_sum_zero = di.add_i32(neg_sum_zero, di.eq_i32(coef, zero));
            x += D::LANES;
        }
    }
    // Remaining rows: no mask
    for y in 1..BLOCK_DIM {
        let mut x = 0;
        while x < BLOCK_DIM {
            let coef = di.load_i32(&block[y * BLOCK_DIM + x..]);
            neg_sum_zero = di.add_i32(neg_sum_zero, di.eq_i32(coef, zero));
            x += D::LANES;
        }
    }
    // We want 64 - sum_zero, add because neg_sum_zero is already negated.
    let nzeros = (DCT_BLOCK_SIZE as i32).wrapping_add(sum_of_lanes_i32(di, neg_sum_zero));
    *nzeros_pos = nzeros;
    nzeros
}

/// TokenizeCoefficients at a SIMD target: `enc_entropy_coder`'s control
/// flow with the vector counters above.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
#[expect(
    clippy::needless_range_loop,
    reason = "Keep the source block index loop."
)]
pub fn tokenize_coefficients_d<D: SimdF32>(
    di: D,
    orders: &[CoeffOrder],
    rect: Rect,
    ac_rows: [&[i32]; 3],
    ac_strategy: &AcStrategyImage,
    cs: &YCbCrChromaSubsampling,
    tmp_num_nzeroes: &mut Image3<i32>,
    output: &mut Vec<Token>,
    qdc: &Plane<u8>,
    qf: &Plane<i32>,
    block_ctx_map: &BlockCtxMap,
) -> Status {
    let xsize_blocks = rect.xsize();
    let ysize_blocks = rect.ysize();
    output.clear();
    // TODO(user): update the estimate: usually less coefficients are used.
    output.reserve(3 * xsize_blocks * ysize_blocks * DCT_BLOCK_SIZE);

    let mut offset = [0usize; 3];
    let nzeros_stride = tmp_num_nzeroes.pixels_per_row() as usize;
    for by in 0..ysize_blocks {
        let sby = [
            by >> cs.v_shift(0),
            by >> cs.v_shift(1),
            by >> cs.v_shift(2),
        ];
        let row_qdc = &qdc.row(rect.y0() + by)[rect.x0()..];
        let row_qf = &qf.row(rect.y0() + by)[rect.x0()..];
        let acs_row = ac_strategy.const_rect_row(rect, by);
        for bx in 0..xsize_blocks {
            let acs = acs_row.get(bx);
            if !acs.is_first_block() {
                continue;
            }
            let sbx = [
                bx >> cs.h_shift(0),
                bx >> cs.h_shift(1),
                bx >> cs.h_shift(2),
            ];
            let mut cx = acs.covered_blocks_x();
            let mut cy = acs.covered_blocks_y();
            let covered_blocks = cx * cy; // = #LLF coefficients
            let log2_covered_blocks = covered_blocks.trailing_zeros() as usize;
            let size = covered_blocks * DCT_BLOCK_SIZE;

            coefficient_layout(&mut cy, &mut cx); // swap cx/cy to canonical order

            for c in [1usize, 0, 2] {
                if sbx[c] << cs.h_shift(c) != bx {
                    continue;
                }
                if sby[c] << cs.v_shift(c) != by {
                    continue;
                }
                let block = &ac_rows[c][offset[c]..];

                let plane = tmp_num_nzeroes.plane_mut(c);
                let nzeros_all = plane.pixels_mut().ok_or(StatusCode::GenericError)?;
                let row_start = sby[c] * nzeros_stride;
                let mut nzeros = if covered_blocks == 1 {
                    num_non_zero_8x8_except_dc_d(di, block, &mut nzeros_all[row_start + sbx[c]])
                } else {
                    num_non_zero_except_llf_d(
                        di,
                        cx,
                        cy,
                        acs,
                        covered_blocks,
                        log2_covered_blocks,
                        block,
                        nzeros_stride,
                        nzeros_all,
                        row_start + sbx[c],
                    )
                };

                let ord = usize::from(STRATEGY_ORDER[usize::from(acs.raw_strategy())]);
                let order = &orders[coeff_order_offset(ord, c)..];

                let predicted_nzeros = {
                    let (top, row) = nzeros_all.split_at(row_start);
                    let row_top = if sby[c] == 0 {
                        None
                    } else {
                        Some(&top[row_start - nzeros_stride..])
                    };
                    predict_from_top_and_left(row_top, row, sbx[c], 32)
                };
                let block_ctx =
                    block_ctx_map.context(i32::from(row_qdc[bx]), row_qf[sbx[c]] as u32, ord, c);
                let nzero_ctx =
                    block_ctx_map.non_zero_context(predicted_nzeros as u32, block_ctx as u32);

                output.push(Token::new(nzero_ctx, nzeros as u32));
                let histo_offset = block_ctx_map.zero_density_contexts_offset(block_ctx as u32);
                // Skip LLF.
                let mut prev = if nzeros as isize > (size / 16) as isize {
                    0
                } else {
                    1
                };
                let mut k = covered_blocks;
                while k < size && nzeros != 0 {
                    let coeff = block[order[k] as usize];
                    let ctx = histo_offset as usize
                        + zero_density_context(
                            nzeros as usize,
                            k,
                            covered_blocks,
                            log2_covered_blocks,
                            prev,
                        );
                    let u_coeff = pack_signed(coeff);
                    output.push(Token::new(ctx as u32, u_coeff));
                    prev = usize::from(coeff != 0);
                    nzeros -= prev as i32;
                    k += 1;
                }
                // JXL_ENSURE(nzeros == 0).
                if nzeros != 0 {
                    return Err(StatusCode::GenericError);
                }
                offset[c] += size;
            }
        }
    }
    Ok(())
}

struct Tokenize<'a> {
    orders: &'a [CoeffOrder],
    rect: Rect,
    ac_rows: [&'a [i32]; 3],
    ac_strategy: &'a AcStrategyImage,
    cs: &'a YCbCrChromaSubsampling,
    tmp_num_nzeroes: &'a mut Image3<i32>,
    output: &'a mut Vec<Token>,
    qdc: &'a Plane<u8>,
    qf: &'a Plane<i32>,
    block_ctx_map: &'a BlockCtxMap,
}
impl Kernel for Tokenize<'_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        tokenize_coefficients_d(
            d,
            self.orders,
            self.rect,
            self.ac_rows,
            self.ac_strategy,
            self.cs,
            self.tmp_num_nzeroes,
            self.output,
            self.qdc,
            self.qf,
            self.block_ctx_map,
        )
    }
}

/// `HWY_DYNAMIC_DISPATCH(TokenizeCoefficients)` at a SIMD target.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn tokenize_coefficients_simd(
    target: Target,
    orders: &[CoeffOrder],
    rect: Rect,
    ac_rows: [&[i32]; 3],
    ac_strategy: &AcStrategyImage,
    cs: &YCbCrChromaSubsampling,
    tmp_num_nzeroes: &mut Image3<i32>,
    output: &mut Vec<Token>,
    qdc: &Plane<u8>,
    qf: &Plane<i32>,
    block_ctx_map: &BlockCtxMap,
) -> Status {
    dispatch(
        target,
        Tokenize {
            orders,
            rect,
            ac_rows,
            ac_strategy,
            cs,
            tmp_num_nzeroes,
            output,
            qdc,
            qf,
            block_ctx_map,
        },
    )
}
