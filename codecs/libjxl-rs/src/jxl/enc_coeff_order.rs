// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Coefficient-order computation (ComputeUsedOrders, ComputeCoeffOrder) and
//! permutation tokenization/encoding (EncodePermutation, EncodeCoeffOrders)
//! from lib/jxl/enc_coeff_order.cc.
//!
//! Contraction map: ComputeCoeffOrder's `num_zeros * inv_sqrt_sz + 0.1f`
//! (binary32) is one front-end fmuladd site, fused on AArch64 and separate on
//! x86-64. `std::sqrt(sz)` with an integer argument is the double overload.
//! The order sort's keys are unique (the index fills the low 16 bits), so the
//! libc++ order is fully determined by the keys.

use super::ac_strategy::{
    AcStrategy, AcStrategyImage, AcStrategyType, MAX_COEFF_AREA, NUM_VALID_STRATEGIES,
};
use super::base::float::mul_add_f32;
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
use super::coeff_order::{
    COEFF_ORDER_MAX_SIZE, PERMUTATION_CONTEXTS, STRATEGY_ORDER, coeff_order_context,
    coeff_order_offset,
};
use super::coeff_order_fwd::{CoeffOrder, coefficient_layout};
use super::common::SpeedTier;
use super::dct_util::{AcImage, ConstAcPtr};
use super::enc_ans::{EntropyEncodingData, Token, write_tokens_with_allotment};
use super::enc_ans_params::HistogramParams;
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_entropy::build_and_encode_histograms;
use super::frame_dimensions::{BLOCK_DIM, DCT_BLOCK_SIZE, FrameDimensions, GROUP_DIM_IN_BLOCKS};
use super::lehmer_code::compute_lehmer_code;

fn tokenize_permutation(
    order: &[CoeffOrder],
    skip: usize,
    size: usize,
    tokens: &mut Vec<Token>,
) -> Status {
    let mut lehmer = vec![0; size];
    let mut temp = vec![0; size + 1];
    compute_lehmer_code(order, &mut temp, size, &mut lehmer)?;
    let mut end = size;
    while end > skip && lehmer[end - 1] == 0 {
        end -= 1;
    }
    tokens.push(Token::new(
        coeff_order_context(size as u32),
        end.wrapping_sub(skip) as u32,
    ));
    let mut last = 0;
    for &value in lehmer.iter().take(end).skip(skip) {
        tokens.push(Token::new(coeff_order_context(last), value));
        last = value;
    }
    Ok(())
}
pub fn encode_permutation(
    order: &[CoeffOrder],
    skip: usize,
    size: usize,
    writer: &mut BitWriter,
    layer: LayerType,
    mut aux: Option<&mut AuxOut>,
) -> Status {
    let mut tokens = vec![Vec::new()];
    tokenize_permutation(order, skip, size, &mut tokens[0])?;
    let mut codes = EntropyEncodingData::default();
    let _cost = build_and_encode_histograms(
        &HistogramParams::default(),
        PERMUTATION_CONTEXTS as usize,
        &mut tokens,
        &mut codes,
        Some(writer),
        layer,
        aux.as_deref_mut(),
    )?;
    write_tokens_with_allotment(&tokens[0], &codes, 0, writer, layer, aux)
}

/// ComputeUsedOrders: the bit set of used orders and of customizable orders.
pub fn compute_used_orders(
    speed: SpeedTier,
    ac_strategy: &AcStrategyImage,
    rect: Rect,
) -> (u32, u32) {
    // No coefficient reordering in Falcon or faster.
    // Only uses DCT8 = 0, so bitfield = 1.
    if speed >= SpeedTier::Falcon {
        return (1, 1);
    }
    let mut ret = 0u32;
    let mut ret_customize = 0u32;
    let xsize_blocks = rect.xsize();
    let ysize_blocks = rect.ysize();
    // TODO(veluca): precompute when doing DCT.
    for by in 0..ysize_blocks {
        let acs_row = ac_strategy.const_rect_row(rect, by);
        for bx in 0..xsize_blocks {
            let ord = u32::from(STRATEGY_ORDER[usize::from(acs_row.get(bx).raw_strategy())]);
            // Do not customize coefficient orders for blocks bigger than 32x32.
            ret |= 1u32 << ord;
            if ord > 6 {
                continue;
            }
            ret_customize |= 1u32 << ord;
        }
    }
    // Use default orders for small images.
    if ac_strategy.xsize() < 5 && ac_strategy.ysize() < 5 {
        return (ret, 0);
    }
    (ret, ret_customize)
}

#[derive(Clone, Copy, Default)]
struct PosAndCount {
    pos: u32,
    // Saving index breaks the ties for non-stable sort
    count_and_idx: u64,
}

/// ComputeCoeffOrder. `order` is one pass's order table.
#[allow(clippy::too_many_arguments)] // Preserve the original function boundary.
pub fn compute_coeff_order(
    speed: SpeedTier,
    ac_image: &AcImage,
    ac_strategy: &AcStrategyImage,
    frame_dim: &FrameDimensions,
    all_used_orders: &mut u32,
    prev_used_acs: u32,
    current_used_acs: u32,
    mut current_used_orders: u32,
    order: &mut [CoeffOrder],
) -> Status {
    let mut num_zeros = vec![0i64; COEFF_ORDER_MAX_SIZE];
    // If compressing at high speed and only using 8x8 DCTs, only consider a
    // subset of blocks.
    let mut block_fraction = 1.0f64;
    // TODO(veluca): figure out why sampling blocks if non-8x8s are used makes
    // encoding significantly less dense.
    if speed >= SpeedTier::Squirrel && current_used_orders == 1 {
        block_fraction = f64::from(0.5f32);
    }
    // No need to compute number of zero coefficients if all orders are the
    // default.
    if current_used_orders != 0 {
        let threshold = ((u64::MAX >> 32) as f64 * block_fraction) as u64;
        let mut s: [u64; 2] = [0x94D049BB133111EB, 0xBF58476D1CE4E5B9];
        // Xorshift128+ adapted from xorshift128+-inl.h
        let mut use_sample = || {
            let mut s1 = s[0];
            let s0 = s[1];
            let bits = s1.wrapping_add(s0); // b, c
            s[0] = s0;
            s1 ^= s1 << 23;
            s1 ^= s0 ^ (s1 >> 18) ^ (s0 >> 5);
            s[1] = s1;
            (bits >> 32) <= threshold
        };

        // Count number of zero coefficients, separately for each DCT band.
        // TODO(veluca): precompute when doing DCT.
        for group_index in 0..frame_dim.num_groups {
            let gx = group_index % frame_dim.xsize_groups;
            let gy = group_index / frame_dim.xsize_groups;
            let rect = Rect::new_clamped(
                gx * GROUP_DIM_IN_BLOCKS,
                gy * GROUP_DIM_IN_BLOCKS,
                GROUP_DIM_IN_BLOCKS,
                GROUP_DIM_IN_BLOCKS,
                frame_dim.xsize_blocks,
                frame_dim.ysize_blocks,
            );
            let rows: [ConstAcPtr<'_>; 3] =
                std::array::from_fn(|c| ac_image.plane_row(c, group_index, 0));
            let mut ac_offset = 0usize;

            // TODO(veluca): SIMDfy.
            for by in 0..rect.ysize() {
                let acs_row = ac_strategy.const_rect_row(rect, by);
                for bx in 0..rect.xsize() {
                    let acs = acs_row.get(bx);
                    if !acs.is_first_block() {
                        continue;
                    }
                    if !use_sample() {
                        continue;
                    }
                    let size = DCT_BLOCK_SIZE << acs.log2_covered_blocks();
                    for (c, row) in rows.iter().enumerate() {
                        let order_offset = coeff_order_offset(
                            usize::from(STRATEGY_ORDER[usize::from(acs.raw_strategy())]),
                            c,
                        );
                        match row {
                            ConstAcPtr::I16(row) => {
                                for k in 0..size {
                                    let is_zero = row[ac_offset + k] == 0;
                                    num_zeros[order_offset + k] += i64::from(is_zero);
                                }
                            }
                            ConstAcPtr::I32(row) => {
                                for k in 0..size {
                                    let is_zero = row[ac_offset + k] == 0;
                                    num_zeros[order_offset + k] += i64::from(is_zero);
                                }
                            }
                        }
                        // Ensure LLFs are first in the order.
                        let mut cx = acs.covered_blocks_x();
                        let mut cy = acs.covered_blocks_y();
                        coefficient_layout(&mut cy, &mut cx);
                        for iy in 0..cy {
                            for ix in 0..cx {
                                num_zeros[order_offset + iy * BLOCK_DIM * cx + ix] = -1;
                            }
                        }
                    }
                    ac_offset += size;
                }
            }
        }
    }
    let mut mem = vec![PosAndCount::default(); MAX_COEFF_AREA];

    let mut natural_order_buffer: Vec<CoeffOrder> = Vec::new();

    let mut computed = 0u16;
    for o in 0..NUM_VALID_STRATEGIES {
        let ord = STRATEGY_ORDER[usize::from(o)];
        if computed & (1 << ord) != 0 {
            continue;
        }
        computed |= 1 << ord;
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(u32::from(o)));
        let sz = DCT_BLOCK_SIZE * acs.covered_blocks_x() * acs.covered_blocks_y();

        // Do nothing for transforms that don't appear.
        if (1u32 << ord) & !current_used_acs != 0 {
            continue;
        }

        // Do nothing if we already committed to this custom order previously.
        if (1u32 << ord) & prev_used_acs != 0 {
            continue;
        }
        if (1u32 << ord) & *all_used_orders != 0 {
            continue;
        }

        if natural_order_buffer.len() < sz {
            natural_order_buffer.resize(sz, 0);
        }
        acs.compute_natural_coeff_order(&mut natural_order_buffer);

        // Ensure natural coefficient order is not permuted if the order is
        // not transmitted.
        if (1u32 << ord) & !current_used_orders != 0 {
            for c in 0..3 {
                let offset = coeff_order_offset(usize::from(ord), c);
                if coeff_order_offset(usize::from(ord), c + 1) - offset != sz {
                    return Err(StatusCode::GenericError);
                }
                order[offset..offset + sz].copy_from_slice(&natural_order_buffer[..sz]);
            }
            continue;
        }

        let mut is_nondefault = false;
        for c in 0..3 {
            // Apply zig-zag order.
            let pos_and_val = &mut mem[..sz];
            let offset = coeff_order_offset(usize::from(ord), c);
            if coeff_order_offset(usize::from(ord), c + 1) - offset != sz {
                return Err(StatusCode::GenericError);
            }
            let inv_sqrt_sz = (1.0f64 / (sz as f64).sqrt()) as f32;
            for i in 0..sz {
                let pos = natural_order_buffer[i];
                pos_and_val[i].pos = pos;
                // We don't care for the exact number -> quantize number of zeros,
                // to get less permuted order. `num_zeros * inv_sqrt_sz + 0.1f`
                // is one contracted expression.
                let count_f =
                    mul_add_f32(num_zeros[offset + pos as usize] as f32, inv_sqrt_sz, 0.1f32);
                let count = float_to_u64(count_f);
                pos_and_val[i].count_and_idx = (count << 16) | i as u64;
            }

            // Stable-sort -> elements with same number of zeros will preserve their
            // order.
            super::base::libcpp_sort::sort_by(pos_and_val, |a, b| {
                a.count_and_idx < b.count_and_idx
            });

            // Grab indices.
            for i in 0..sz {
                order[offset + i] = pos_and_val[i].pos;
                is_nondefault |= natural_order_buffer[i] != pos_and_val[i].pos;
            }
        }
        if !is_nondefault {
            current_used_orders &= !(1u32 << ord);
        }
    }
    *all_used_orders |= current_used_orders;
    Ok(())
}

/// The float-to-uint64_t conversion of the source. Values in (-1, 0]
/// truncate to zero; the source is undefined below that, and the reached
/// values (the -1 LLF markers give 0.1f - inv_sqrt_sz) are not.
fn float_to_u64(v: f32) -> u64 {
    debug_assert!(v > -1.0 && v < 18446744073709551616.0);
    if v <= 0.0 { 0 } else { v as u64 }
}

fn encode_coeff_order(
    order: &[CoeffOrder],
    acs: AcStrategy,
    tokens: &mut Vec<Token>,
    order_zigzag: &mut [CoeffOrder],
    natural_order_lut: &[CoeffOrder],
) -> Status {
    let llf = acs.covered_blocks_x() * acs.covered_blocks_y();
    let size = DCT_BLOCK_SIZE * llf;
    for i in 0..size {
        order_zigzag[i] = natural_order_lut[order[i] as usize];
    }
    tokenize_permutation(order_zigzag, llf, size, tokens)
}

/// EncodeCoeffOrders. `order` is one pass's order table.
pub fn encode_coeff_orders(
    used_orders: u16,
    order: &[CoeffOrder],
    writer: &mut BitWriter,
    layer: LayerType,
    mut aux: Option<&mut AuxOut>,
) -> Status {
    let mut mem = vec![0 as CoeffOrder; MAX_COEFF_AREA];
    let mut computed = 0u16;
    let mut tokens = vec![Vec::new()];
    let mut natural_order_lut: Vec<CoeffOrder> = Vec::new();
    for o in 0..NUM_VALID_STRATEGIES {
        let ord = STRATEGY_ORDER[usize::from(o)];
        if computed & (1 << ord) != 0 {
            continue;
        }
        computed |= 1 << ord;
        if used_orders & (1 << ord) == 0 {
            continue;
        }
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(u32::from(o)));
        let llf = acs.covered_blocks_x() * acs.covered_blocks_y();
        let size = DCT_BLOCK_SIZE * llf;
        if natural_order_lut.len() < size {
            natural_order_lut.resize(size, 0);
        }
        acs.compute_natural_coeff_order_lut(&mut natural_order_lut);
        for c in 0..3 {
            encode_coeff_order(
                &order[coeff_order_offset(usize::from(ord), c)..],
                acs,
                &mut tokens[0],
                &mut mem,
                &natural_order_lut,
            )?;
        }
    }
    // Do not write anything if no order is used.
    if used_orders != 0 {
        let mut codes = EntropyEncodingData::default();
        let _cost = build_and_encode_histograms(
            &HistogramParams::default(),
            PERMUTATION_CONTEXTS as usize,
            &mut tokens,
            &mut codes,
            Some(writer),
            layer,
            aux.as_deref_mut(),
        )?;
        write_tokens_with_allotment(&tokens[0], &codes, 0, writer, layer, aux)?;
    }
    Ok(())
}
