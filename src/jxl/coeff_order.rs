// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Permutation and AC coefficient-order decoding from lib/jxl/coeff_order.{h,cc}.
//! Strategy-order buckets, natural-order selection, serialized permutations,
//! optional unused outputs, and coefficient offsets retain the original loops.

use super::ac_strategy::{AcStrategy, AcStrategyType, NUM_VALID_STRATEGIES};
use super::base::status::{Status, StatusCode};
use super::coeff_order_fwd::CoeffOrder;
use super::dec_ans::{ANSCode, ANSSymbolReader, HybridUintConfig, decode_histograms};
use super::dec_bit_reader::BitReader;
use super::lehmer_code::decode_lehmer_code;
pub const PERMUTATION_CONTEXTS: u32 = 8;
pub const COEFF_ORDER_LIMIT: usize = 6156;
pub const COEFF_ORDER_MAX_SIZE: usize = COEFF_ORDER_LIMIT * super::frame_dimensions::DCT_BLOCK_SIZE;
pub const COEFF_ORDER_OFFSET: [usize; 40] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    10,
    14,
    18,
    34,
    50,
    66,
    68,
    70,
    72,
    76,
    80,
    84,
    92,
    100,
    108,
    172,
    236,
    300,
    332,
    364,
    396,
    652,
    908,
    1164,
    1292,
    1420,
    1548,
    2572,
    3596,
    4620,
    5132,
    5644,
    COEFF_ORDER_LIMIT,
];
pub const STRATEGY_ORDER: [u8; 27] = [
    0, 1, 1, 1, 2, 3, 4, 4, 5, 5, 6, 6, 1, 1, 1, 1, 1, 1, 7, 8, 8, 9, 10, 10, 11, 12, 12,
];

pub const fn coeff_order_offset(order: usize, channel: usize) -> usize {
    COEFF_ORDER_OFFSET[3 * order + channel] * super::frame_dimensions::DCT_BLOCK_SIZE
}

pub fn coeff_order_context(value: u32) -> u32 {
    let (token, _, _) = HybridUintConfig::new(0, 0, 0).encode(value);
    token.min(PERMUTATION_CONTEXTS - 1)
}
fn read_permutation(
    skip: usize,
    size: usize,
    order: Option<&mut [CoeffOrder]>,
    br: &mut BitReader<'_>,
    reader: &mut ANSSymbolReader<'_>,
    context_map: &[u8],
) -> Status {
    let mut lehmer = vec![0; size];
    let mut temp = vec![0; size * 2];
    let end = reader
        .read_hybrid_uint(coeff_order_context(size as u32) as usize, br, context_map)
        .wrapping_add(skip) as u32;
    if end as usize > size {
        return Err(StatusCode::GenericError);
    }
    let mut last = 0;
    for (i, value) in lehmer.iter_mut().enumerate().take(end as usize).skip(skip) {
        *value =
            reader.read_hybrid_uint(coeff_order_context(last) as usize, br, context_map) as u32;
        last = *value;
        if *value as usize >= size - i {
            return Err(StatusCode::GenericError);
        }
    }
    if let Some(order) = order {
        decode_lehmer_code(&lehmer, &mut temp, size, order)?;
    }
    Ok(())
}
pub fn decode_permutation(
    skip: usize,
    size: usize,
    order: Option<&mut [CoeffOrder]>,
    br: &mut BitReader<'_>,
) -> Status {
    let mut context_map = Vec::new();
    let mut code = ANSCode::default();
    decode_histograms(
        br,
        PERMUTATION_CONTEXTS as usize,
        &mut code,
        &mut context_map,
        false,
    )?;
    let mut reader = ANSSymbolReader::create(&code, br, 0)?;
    read_permutation(skip, size, order, br, &mut reader, &context_map)?;
    if !reader.check_ans_final_state() {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}

fn decode_coeff_order(
    acs: AcStrategy,
    mut order: Option<&mut [CoeffOrder]>,
    br: &mut BitReader<'_>,
    reader: &mut ANSSymbolReader<'_>,
    natural_order: &[CoeffOrder],
    context_map: &[u8],
) -> Status {
    let llf = acs.covered_blocks_x() * acs.covered_blocks_y();
    let size = super::frame_dimensions::DCT_BLOCK_SIZE * llf;
    read_permutation(llf, size, order.as_deref_mut(), br, reader, context_map)?;
    if let Some(order) = order {
        for value in &mut order[..size] {
            *value = natural_order[*value as usize];
        }
    }
    Ok(())
}

/// The output is the original packed per-order/per-channel storage. Only
/// selected strategies' ranges are accessed; unused encoded orders still
/// consume their permutations and validate the final entropy state.
#[expect(
    clippy::collapsible_if,
    reason = "Mirror the source nested conditions."
)]
pub fn decode_coeff_orders(
    used_orders: u16,
    used_acs: u32,
    order: &mut [CoeffOrder],
    br: &mut BitReader<'_>,
) -> Status {
    let mut computed = 0u16;
    let mut context_map = Vec::new();
    let mut code = ANSCode::default();
    let mut natural_order = Vec::new();
    let mut reader = if used_orders != 0 {
        decode_histograms(
            br,
            PERMUTATION_CONTEXTS as usize,
            &mut code,
            &mut context_map,
            false,
        )?;
        Some(ANSSymbolReader::create(&code, br, 0)?)
    } else {
        None
    };
    let mut acs_mask = 0u32;
    for (raw, &ord) in STRATEGY_ORDER
        .iter()
        .enumerate()
        .take(NUM_VALID_STRATEGIES as usize)
    {
        if used_acs & (1 << raw) == 0 {
            continue;
        }
        acs_mask |= 1 << ord;
    }
    for (raw, &ord) in STRATEGY_ORDER
        .iter()
        .enumerate()
        .take(NUM_VALID_STRATEGIES as usize)
    {
        let ord = ord as usize;
        if computed & (1 << ord) != 0 {
            continue;
        }
        computed |= 1 << ord;
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw as u32));
        let used = acs_mask & (1 << ord) != 0;
        let size = super::frame_dimensions::DCT_BLOCK_SIZE
            * acs.covered_blocks_x()
            * acs.covered_blocks_y();
        if used || used_orders & (1 << ord) != 0 {
            if natural_order.len() < size {
                natural_order.resize(size, 0);
            }
            acs.compute_natural_coeff_order(&mut natural_order);
        }
        if used_orders & (1 << ord) == 0 {
            if used {
                for c in 0..3 {
                    let offset = coeff_order_offset(ord, c);
                    order[offset..offset + size].copy_from_slice(&natural_order[..size]);
                }
            }
        } else {
            for c in 0..3 {
                let dest = if used {
                    let offset = coeff_order_offset(ord, c);
                    Some(&mut order[offset..offset + size])
                } else {
                    None
                };
                decode_coeff_order(
                    acs,
                    dest,
                    br,
                    reader
                        .as_mut()
                        .expect("nonzero used-orders mask initializes the reader"),
                    &natural_order,
                    &context_map,
                )?;
            }
        }
    }
    if let Some(reader) = reader {
        if !reader.check_ans_final_state() {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}
