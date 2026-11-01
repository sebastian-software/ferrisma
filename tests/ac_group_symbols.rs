// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::ac_context::BlockCtxMap;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use jxl::jxl::dct_util::AcPtrMut;
use jxl::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_group::decode_ac_var_block;
use jxl::jxl::enc_ans::{
    EntropyEncodingData, Token, build_and_encode_histograms, write_tokens_with_allotment,
};
use jxl::jxl::enc_ans_params::{HistogramParams, LZ77Method};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;

fn fixture(values: &[u32], prefix: bool) -> Vec<u8> {
    let mut streams = vec![values.iter().map(|&v| Token::new(0, v)).collect()];
    let params = HistogramParams {
        force_huffman: prefix,
        initialize_global_state: false,
        lz77_method: LZ77Method::None,
        ..HistogramParams::default()
    };
    let mut codes = EntropyEncodingData {
        use_prefix_code: prefix,
        ..EntropyEncodingData::default()
    };
    let mut writer = BitWriter::new();
    build_and_encode_histograms(
        &params,
        1,
        &mut streams,
        &mut codes,
        Some(&mut writer),
        LayerType::Ac,
        None,
    )
    .unwrap();
    write_tokens_with_allotment(&streams[0], &codes, 0, &mut writer, LayerType::Ac, None).unwrap();
    writer
        .with_max_bits(
            7,
            LayerType::Ac,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    writer.get_span().to_vec()
}
fn order(acs: AcStrategy) -> (Vec<u32>, usize) {
    let offset =
        COEFF_ORDER_OFFSET[3 * usize::from(STRATEGY_ORDER[usize::from(acs.raw_strategy())])] * 64;
    let mut order = vec![0; offset + (64 << acs.log2_covered_blocks())];
    acs.compute_natural_coeff_order(&mut order[offset..]);
    (order, offset)
}
#[test]
fn signed_coefficients_accumulate_with_shift_and_i16_narrowing_without_touching_llf() {
    for prefix in [false, true] {
        let bytes = fixture(&[3, 2, 0, 3, 4], prefix);
        let mut br = BitReader::new(&bytes);
        let mut code = ANSCode::default();
        let mut map = Vec::new();
        decode_histograms(&mut br, 1, &mut code, &mut map, false).unwrap();
        let mut reader = ANSSymbolReader::create(&code, &mut br, 0).unwrap();
        let acs = AcStrategy::from_raw_strategy(AcStrategyType::DCT);
        let (order, offset) = order(acs);
        let mut block = vec![32767; 64];
        let mut row = vec![17; 4];
        let ctx = BlockCtxMap::default();
        decode_ac_var_block::<false>(
            0,
            0,
            &mut row,
            None,
            4,
            0,
            0,
            0,
            0,
            acs,
            &order,
            &mut br,
            &mut reader,
            &vec![map[0]; ctx.num_ac_contexts() as usize + 16],
            &[0],
            &[1],
            &ctx,
            AcPtrMut::I16(&mut block),
            4,
        )
        .unwrap();
        assert_eq!(row, [3, 17, 17, 17]);
        assert_eq!(block[order[offset] as usize], 32767);
        assert_eq!(block[order[offset + 1] as usize], -32753);
        assert_eq!(block[order[offset + 2] as usize], 32767);
        assert_eq!(block[order[offset + 3] as usize], 32735);
        assert_eq!(block[order[offset + 4] as usize], -32737);
        assert!(reader.check_ans_final_state());
        br.close().unwrap();
    }
}
#[test]
fn invalid_count_preserves_rows_but_outstanding_count_keeps_scatter_and_coefficient_work() {
    for values in [vec![64], {
        let mut v = vec![2, 2];
        v.extend([0; 62]);
        v
    }] {
        let bytes = fixture(&values, false);
        let mut br = BitReader::new(&bytes);
        let mut code = ANSCode::default();
        let mut map = Vec::new();
        decode_histograms(&mut br, 1, &mut code, &mut map, false).unwrap();
        let mut reader = ANSSymbolReader::create(&code, &mut br, 0).unwrap();
        let acs = AcStrategy::from_raw_strategy(AcStrategyType::DCT);
        let (order, offset) = order(acs);
        let mut block = vec![10; 64];
        let mut row = vec![17; 4];
        let ctx = BlockCtxMap::default();
        let status = decode_ac_var_block::<false>(
            0,
            0,
            &mut row,
            None,
            4,
            0,
            0,
            0,
            0,
            acs,
            &order,
            &mut br,
            &mut reader,
            &vec![map[0]; ctx.num_ac_contexts() as usize + 16],
            &[0],
            &[1],
            &ctx,
            AcPtrMut::I32(&mut block),
            0,
        );
        assert_eq!(status, Err(StatusCode::GenericError));
        assert_eq!(row[0], if values[0] == 64 { 17 } else { 2 });
        assert_eq!(
            block[order[offset + 1] as usize],
            if values[0] == 64 { 10 } else { 11 }
        );
        assert_eq!(block[order[offset] as usize], 10);
        assert!(reader.check_ans_final_state());
        br.close().unwrap();
    }
}
#[test]
fn multiblock_scatter_retains_neighbor_rows_and_zero_count_retains_coefficients() {
    let bytes = fixture(&[0], true);
    let mut br = BitReader::new(&bytes);
    let mut code = ANSCode::default();
    let mut map = Vec::new();
    decode_histograms(&mut br, 1, &mut code, &mut map, false).unwrap();
    let mut reader = ANSSymbolReader::create(&code, &mut br, 0).unwrap();
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(4));
    let (order, _) = order(acs);
    let mut row = vec![19; 16];
    let mut block = vec![-20; 256];
    let ctx = BlockCtxMap::default();
    decode_ac_var_block::<false>(
        0,
        2,
        &mut row,
        Some(&[15; 4]),
        4,
        0,
        1,
        5,
        0,
        acs,
        &order,
        &mut br,
        &mut reader,
        &vec![map[0]; ctx.num_ac_contexts() as usize + 16],
        &[0],
        &[0, 2],
        &ctx,
        AcPtrMut::I32(&mut block),
        0,
    )
    .unwrap();
    for (i, &value) in row.iter().enumerate() {
        assert_eq!(value, if [1, 2, 5, 6].contains(&i) { 0 } else { 19 });
    }
    assert_eq!(block, vec![-20; 256]);
    assert!(reader.check_ans_final_state());
    br.close().unwrap();
}
