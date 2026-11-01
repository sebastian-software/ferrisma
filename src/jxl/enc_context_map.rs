// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Context-map output from `lib/jxl/enc_context_map.{h,cc}`.
//! Preserve cost ties and MTF order.

use super::ac_context::{BlockCtxMap, DEFAULT_CTX_MAP};
use super::base::bits::ceil_log2_nonzero;
use super::base::status::Status;
use super::enc_ans::{EntropyEncodingData, Token, build_and_encode_histograms, write_tokens};
use super::enc_ans_params::{HistogramParams, HybridUintMethod};
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::entropy_coder::{DC_THRESHOLD_DIST, QF_THRESHOLD_DIST};
use super::fields::u32_coder;
use super::pack_signed::pack_signed;

pub const CLUSTERS_LIMIT: usize = 128;

pub fn encode_block_ctx_map(
    block_ctx_map: &BlockCtxMap,
    writer: &mut BitWriter,
    aux: Option<&mut AuxOut>,
) -> Status {
    let dct = &block_ctx_map.dc_thresholds;
    let qft = &block_ctx_map.qf_thresholds;
    let ctx_map = &block_ctx_map.ctx_map;
    writer.with_max_bits(
        (dct[0].len() + dct[1].len() + dct[2].len() + qft.len()) * 34
            + 1
            + 4
            + 4
            + ctx_map.len() * 10
            + 1024,
        LayerType::Ac,
        aux,
        |writer, aux| {
            if dct.iter().all(Vec::is_empty) && qft.is_empty() && ctx_map == &DEFAULT_CTX_MAP {
                writer.write(1, 1);
                return Ok(());
            }
            writer.write(1, 0);
            for thresholds in dct {
                writer.write(4, thresholds.len() as u64);
                for &i in thresholds {
                    u32_coder::write(DC_THRESHOLD_DIST, pack_signed(i), writer)?;
                }
            }
            writer.write(4, qft.len() as u64);
            for &i in qft {
                u32_coder::write(QF_THRESHOLD_DIST, i.wrapping_sub(1), writer)?;
            }
            encode_context_map(ctx_map, block_ctx_map.num_ctxs, writer, LayerType::Ac, aux)
        },
        false,
    )
}

fn move_to_front_transform(values: &[u8]) -> Vec<u8> {
    if values.is_empty() {
        return values.to_vec();
    }
    let max_value = *values.iter().max().unwrap();
    let mut mtf: Vec<_> = (0..=max_value).collect();
    let mut result = Vec::with_capacity(values.len());
    for &value in values {
        let index = mtf.iter().position(|&entry| entry == value).unwrap();
        result.push(index as u8);
        let value = mtf[index];
        for i in (1..=index).rev() {
            mtf[i] = mtf[i - 1];
        }
        mtf[0] = value;
    }
    result
}

/// A single-histogram map uses the caller's active writer allotment.
pub fn encode_context_map(
    context_map: &[u8],
    num_histograms: usize,
    writer: &mut BitWriter,
    layer: LayerType,
    aux: Option<&mut AuxOut>,
) -> Status {
    if num_histograms == 1 {
        writer.write(1, 1);
        writer.write(2, 0);
        return Ok(());
    }
    let transformed = move_to_front_transform(context_map);
    let mut tokens = vec![
        context_map
            .iter()
            .map(|&ctx| Token::new(0, u32::from(ctx)))
            .collect(),
    ];
    let mut mtf_tokens = vec![
        transformed
            .iter()
            .map(|&sym| Token::new(0, u32::from(sym)))
            .collect(),
    ];
    let params = HistogramParams {
        uint_method: HybridUintMethod::ContextMap,
        ..HistogramParams::default()
    };
    let ans_cost = build_and_encode_histograms(
        &params,
        1,
        &mut tokens,
        &mut EntropyEncodingData::default(),
        None,
        LayerType::Header,
        None,
    )?;
    let mtf_cost = build_and_encode_histograms(
        &params,
        1,
        &mut mtf_tokens,
        &mut EntropyEncodingData::default(),
        None,
        LayerType::Header,
        None,
    )?;
    let use_mtf = mtf_cost < ans_cost;
    tokens[0].clear();
    for i in 0..transformed.len() {
        tokens[0].push(Token::new(
            0,
            u32::from(if use_mtf {
                transformed[i]
            } else {
                context_map[i]
            }),
        ));
    }
    let entry_bits = ceil_log2_nonzero(num_histograms);
    let simple_cost = entry_bits * context_map.len();
    if entry_bits < 4 && simple_cost < ans_cost && simple_cost < mtf_cost {
        writer.with_max_bits(
            3 + simple_cost,
            layer,
            aux,
            |w, _| {
                w.write(1, 1);
                w.write(2, entry_bits as u64);
                for &entry in context_map {
                    w.write(entry_bits, u64::from(entry));
                }
                Ok(())
            },
            false,
        )?;
    } else {
        writer.with_max_bits(
            2 + tokens[0].len() * 24,
            layer,
            aux,
            |w, aux| {
                w.write(1, 0);
                w.write(1, u64::from(use_mtf));
                let mut codes = EntropyEncodingData::default();
                build_and_encode_histograms(
                    &params,
                    1,
                    &mut tokens,
                    &mut codes,
                    Some(w),
                    layer,
                    aux,
                )?;
                write_tokens(&tokens[0], &codes, 0, w);
                Ok(())
            },
            false,
        )?;
    }
    Ok(())
}
