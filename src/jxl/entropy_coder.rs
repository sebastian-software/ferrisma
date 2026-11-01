// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar prediction and BlockCtxMap decoding from `lib/jxl/entropy_coder.{h,cc}`.

use super::ac_context::BlockCtxMap;
use super::base::status::{Status, StatusCode};
use super::coeff_order_fwd::NUM_ORDERS;
use super::dec_bit_reader::BitReader;
use super::dec_context_map::decode_context_map;
use super::field_encodings::{U32Enc, bits, bits_offset};
use super::fields::u32_coder;
use super::pack_signed::unpack_signed;

/// Neighbor slices cover each reached index. As in the source, the reached
/// signed sum `top + left + 1` must be representable as i32 at both additions.
pub fn predict_from_top_and_left(
    row_top: Option<&[i32]>,
    row: &[i32],
    x: usize,
    default_val: i32,
) -> i32 {
    if x == 0 {
        return row_top.map_or(default_val, |top| top[x]);
    }
    let Some(top) = row_top else {
        return row[x - 1];
    };
    (top[x] + row[x - 1] + 1) / 2
}

pub const DC_THRESHOLD_DIST: U32Enc = U32Enc::new(
    bits(4),
    bits_offset(8, 16),
    bits_offset(16, 272),
    bits_offset(32, 65808),
);
pub const QF_THRESHOLD_DIST: U32Enc = U32Enc::new(
    bits(2),
    bits_offset(3, 4),
    bits_offset(5, 12),
    bits_offset(8, 44),
);

pub fn decode_block_ctx_map(br: &mut BitReader<'_>, block_ctx_map: &mut BlockCtxMap) -> Status {
    let is_default = br.read_fixed_bits::<1>() != 0;
    if is_default {
        *block_ctx_map = BlockCtxMap::default();
        return Ok(());
    }
    block_ctx_map.num_dc_ctxs = 1;
    for dct in &mut block_ctx_map.dc_thresholds {
        dct.resize(br.read_fixed_bits::<4>() as usize, 0);
        block_ctx_map.num_dc_ctxs *= dct.len() + 1;
        for i in dct {
            *i = unpack_signed(u32_coder::read(DC_THRESHOLD_DIST, br) as usize);
        }
    }
    block_ctx_map
        .qf_thresholds
        .resize(br.read_fixed_bits::<4>() as usize, 0);
    for i in &mut block_ctx_map.qf_thresholds {
        *i = u32_coder::read(QF_THRESHOLD_DIST, br).wrapping_add(1);
    }
    if block_ctx_map.num_dc_ctxs * (block_ctx_map.qf_thresholds.len() + 1) > 64 {
        return Err(StatusCode::GenericError);
    }
    block_ctx_map.ctx_map.resize(
        3 * usize::from(NUM_ORDERS)
            * block_ctx_map.num_dc_ctxs
            * (block_ctx_map.qf_thresholds.len() + 1),
        0,
    );
    decode_context_map(&mut block_ctx_map.ctx_map, &mut block_ctx_map.num_ctxs, br)?;
    if block_ctx_map.num_ctxs > 16 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
