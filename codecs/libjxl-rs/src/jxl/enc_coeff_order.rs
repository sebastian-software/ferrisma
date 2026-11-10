// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Permutation tokenization/encoding from lib/jxl/enc_coeff_order.cc.
//! AC-strategy coefficient ordering remains unported.

use super::base::status::Status;
use super::coeff_order::{PERMUTATION_CONTEXTS, coeff_order_context};
use super::coeff_order_fwd::CoeffOrder;
use super::enc_ans::{EntropyEncodingData, Token, write_tokens_with_allotment};
use super::enc_ans_params::HistogramParams;
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_entropy::build_and_encode_histograms;
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
