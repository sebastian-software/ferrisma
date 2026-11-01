// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Permutation context and decoder from lib/jxl/coeff_order.{h,cc}.
//! AC-strategy coefficient orders remain a separate, unported boundary.

use super::base::status::{Status, StatusCode};
use super::coeff_order_fwd::CoeffOrder;
use super::dec_ans::{ANSCode, ANSSymbolReader, HybridUintConfig, decode_histograms};
use super::dec_bit_reader::BitReader;
use super::lehmer_code::decode_lehmer_code;
pub const PERMUTATION_CONTEXTS: u32 = 8;

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
