// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! DecodeACVarBlock from lib/jxl/dec_group.cc. Group scheduling, dequantization,
//! and pixel rendering remain separate boundaries. Typed coefficient borrows
//! replace ACPtr and the source ACType template parameter.

use super::ac_context::{BlockCtxMap, zero_density_context};
use super::ac_strategy::AcStrategy;
use super::base::status::{Status, StatusCode};
use super::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use super::coeff_order_fwd::CoeffOrder;
use super::dct_util::AcPtrMut;
use super::dec_ans::ANSSymbolReader;
use super::dec_bit_reader::BitReader;
use super::entropy_coder::predict_from_top_and_left;
use super::frame_dimensions::DCT_BLOCK_SIZE;

/// Decodes one quantized AC block, retaining its LLF coefficients and adding
/// this pass to existing coefficients. All source array, context, strategy,
/// and predictor preconditions apply. `shift` must be less than usize::BITS;
/// each coefficient plus its existing value must fit source ptrdiff_t before
/// the source's implementation-defined signed narrowing (low bits on Clang).
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn decode_ac_var_block<const USES_LZ77: bool>(
    ctx_offset: usize,
    log2_covered_blocks: usize,
    row_nzeros: &mut [i32],
    row_nzeros_top: Option<&[i32]>,
    nzeros_stride: usize,
    c: usize,
    bx: usize,
    _by: usize,
    lbx: usize,
    acs: AcStrategy,
    coeff_order: &[CoeffOrder],
    br: &mut BitReader<'_>,
    decoder: &mut ANSSymbolReader<'_>,
    context_map: &[u8],
    qdc_row: &[u8],
    qf_row: &[i32],
    block_ctx_map: &BlockCtxMap,
    mut block: AcPtrMut<'_>,
    shift: usize,
) -> Status {
    let covered_blocks = 1usize << log2_covered_blocks;
    let size = covered_blocks * DCT_BLOCK_SIZE;
    let predicted_nzeros = predict_from_top_and_left(row_nzeros_top, row_nzeros, bx, 32);
    let ord = usize::from(STRATEGY_ORDER[usize::from(acs.raw_strategy())]);
    let order = &coeff_order[COEFF_ORDER_OFFSET[3 * ord + c] * DCT_BLOCK_SIZE..];
    let block_ctx = block_ctx_map.context(i32::from(qdc_row[lbx]), qf_row[bx] as u32, ord, c);
    // The source adds size_t ctx_offset, then narrows to int32_t before passing
    // the result back to the decoder's size_t context parameter.
    let nzero_ctx = (block_ctx_map.non_zero_context(predicted_nzeros as u32, block_ctx as u32)
        as usize)
        .wrapping_add(ctx_offset) as i32;
    let mut nzeros =
        decoder.read_hybrid_uint_inlined::<USES_LZ77>(nzero_ctx as usize, br, context_map);
    if nzeros > size - covered_blocks {
        return Err(StatusCode::GenericError);
    }
    for y in 0..acs.covered_blocks_y() {
        for x in 0..acs.covered_blocks_x() {
            row_nzeros[bx + x + y * nzeros_stride] =
                ((nzeros + covered_blocks - 1) >> log2_covered_blocks) as i32;
        }
    }
    let histo_offset = ctx_offset
        .wrapping_add(block_ctx_map.zero_density_contexts_offset(block_ctx as u32) as usize);
    let mut prev = usize::from(nzeros <= size / 16);
    for (k, &position) in order.iter().enumerate().take(size).skip(covered_blocks) {
        if nzeros == 0 {
            break;
        }
        let ctx = histo_offset.wrapping_add(zero_density_context(
            nzeros,
            k,
            covered_blocks,
            log2_covered_blocks,
            prev,
        ));
        let u_coeff = decoder.read_hybrid_uint_inlined::<USES_LZ77>(ctx, br, context_map);
        let magnitude = u_coeff >> 1;
        let neg_sign = (!u_coeff) & 1;
        let coeff = ((magnitude ^ neg_sign.wrapping_sub(1)) << shift) as isize;
        match &mut block {
            AcPtrMut::I16(values) => {
                let value = &mut values[position as usize];
                *value = (*value as isize).wrapping_add(coeff) as i16;
            }
            AcPtrMut::I32(values) => {
                let value = &mut values[position as usize];
                *value = (*value as isize).wrapping_add(coeff) as i32;
            }
        }
        prev = usize::from(u_coeff != 0);
        nzeros -= prev;
    }
    if nzeros != 0 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
