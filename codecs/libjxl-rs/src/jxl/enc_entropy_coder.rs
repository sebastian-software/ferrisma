// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! NumNonZeroExceptLLF, NumNonZero8x8ExceptDC and TokenizeCoefficients from
//! lib/jxl/enc_entropy_coder.cc, for the one-lane (HWY_SCALAR) target. The
//! vector masks count zero coefficients exactly as the lane loop does; integer
//! arithmetic only, so there is no contraction site. Row pointers become
//! slices and owner-relative offsets.

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
use super::image::{Image3, Plane};
use super::pack_signed::pack_signed;

/// Returns the number of non-zero coefficients, skipping the LLF, and writes
/// the per-8x8 block counts starting at `nzeros_pos` in `nzeros`.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
fn num_non_zero_except_llf(
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
    let mut zeros = 0i32;
    // Rows with LLF: mask out the LLF, which counts as zero. The source's
    // mask array has kMaxCoeffBlocks * (1 + kBlockDim) entries of which only
    // the first four are set, read from kMaxCoeffBlocks - cx onwards.
    const MASK_SET: usize = 4;
    for y in 0..cy {
        for x in 0..cx * BLOCK_DIM {
            let masked = MAX_COEFF_BLOCKS - cx + x < MASK_SET;
            let coef = if masked {
                0
            } else {
                block[y * cx * BLOCK_DIM + x]
            };
            if coef == 0 {
                zeros += 1;
            }
        }
    }
    // Remaining rows: no mask
    for y in cy..cy * BLOCK_DIM {
        for x in 0..cx * BLOCK_DIM {
            if block[y * cx * BLOCK_DIM + x] == 0 {
                zeros += 1;
            }
        }
    }
    let count = (cx * cy * DCT_BLOCK_SIZE) as i32 - zeros;
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
fn num_non_zero_8x8_except_dc(block: &[i32], nzeros_pos: &mut i32) -> i32 {
    let mut zeros = 0i32;
    // First row has DC, so mask
    for (x, &value) in block.iter().enumerate().take(BLOCK_DIM) {
        let coef = if x == 0 { 0 } else { value };
        if coef == 0 {
            zeros += 1;
        }
    }
    for &value in &block[BLOCK_DIM..DCT_BLOCK_SIZE] {
        if value == 0 {
            zeros += 1;
        }
    }
    let nzeros = DCT_BLOCK_SIZE as i32 - zeros;
    *nzeros_pos = nzeros;
    nzeros
}

/// TokenizeCoefficients. `ac_rows` are the group's three coefficient rows;
/// `orders` is one pass's order table. The number of nonzeros of each block
/// is predicted from the top and the left blocks.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
#[expect(
    clippy::needless_range_loop,
    reason = "Keep the source block index loop."
)]
pub fn tokenize_coefficients(
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
                    num_non_zero_8x8_except_dc(block, &mut nzeros_all[row_start + sbx[c]])
                } else {
                    num_non_zero_except_llf(
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
                if nzeros != 0 {
                    return Err(StatusCode::GenericError);
                }
                offset[c] += size;
            }
        }
    }
    Ok(())
}
