// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::ac_context::{BlockCtxMap, DEFAULT_CTX_MAP, zero_density_context};
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::{AuxOut, LayerType};
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_context_map::encode_block_ctx_map;
use jxl::jxl::entropy_coder::{decode_block_ctx_map, predict_from_top_and_left};

fn encoded(m: &BlockCtxMap, offset: usize) -> (Vec<u8>, usize) {
    let mut w = BitWriter::new();
    w.with_max_bits(
        offset,
        LayerType::Header,
        None,
        |w, _| {
            if offset != 0 {
                w.write(offset, (1 << offset) - 1);
            }
            Ok(())
        },
        false,
    )
    .unwrap();
    encode_block_ctx_map(m, &mut w, None).unwrap();
    let bits = w.bits_written();
    w.with_max_bits(
        7,
        LayerType::Header,
        None,
        |w, _| {
            w.zero_pad_to_byte();
            Ok(())
        },
        false,
    )
    .unwrap();
    (w.get_span().to_vec(), bits)
}
#[test]
fn zero_density_buckets_cover_valid_and_decoder_invalid_sums() {
    let mut max_valid = 0;
    let mut max_all = 0;
    for nz in 1..64 {
        for k in 1..64 {
            for prev in 0..2 {
                let ctx = zero_density_context(nz, k, 1, 0, prev);
                max_all = max_all.max(ctx);
                if nz + k < 64 {
                    max_valid = max_valid.max(ctx);
                }
                assert_eq!(
                    ctx,
                    zero_density_context(nz * 16 - 15, k * 16 + 15, 16, 4, prev)
                );
            }
        }
    }
    assert_eq!(max_valid + 1, 458);
    assert_eq!(max_all + 1, 474);
}
#[test]
fn context_lookup_uses_channel_swap_strict_thresholds_and_dc_fastest() {
    let m = BlockCtxMap {
        dc_thresholds: [vec![], vec![], vec![]],
        qf_thresholds: vec![5],
        ctx_map: (0..156).collect(),
        num_ctxs: 156,
        num_dc_ctxs: 2,
    };
    assert_eq!(m.context(0, 5, 0, 1), 0);
    assert_eq!(m.context(1, 5, 0, 1), 1);
    assert_eq!(m.context(0, 6, 0, 1), 2);
    assert_eq!(m.context(0, 5, 1, 1), 4);
    assert_eq!(m.context(0, 5, 0, 0), 52);
    assert_eq!(m.context(0, 5, 0, 2), 104);
    assert_eq!(m.non_zero_context(7, 2), 7 * 156 + 2);
    assert_eq!(m.non_zero_context(8, 2), 8 * 156 + 2);
    assert_eq!(m.non_zero_context(63, 2), 35 * 156 + 2);
    assert_eq!(m.non_zero_context(1008, 2), 36 * 156 + 2);
    assert_eq!(m.zero_density_contexts_offset(2), 156 * 37 + 458 * 2);
    assert_eq!(m.num_ac_contexts(), 156 * 495);
}
#[test]
fn prediction_retains_edge_access_and_truncation_toward_zero() {
    assert_eq!(predict_from_top_and_left(None, &[], 0, -71), -71);
    assert_eq!(predict_from_top_and_left(Some(&[19]), &[], 0, -71), 19);
    assert_eq!(predict_from_top_and_left(None, &[-19], 1, -71), -19);
    assert_eq!(predict_from_top_and_left(Some(&[0, -4]), &[0], 1, 0), -1);
    assert_eq!(predict_from_top_and_left(Some(&[0, -3]), &[0], 1, 0), -1);
}
#[test]
fn default_decoding_replaces_every_reused_field_and_encoding_ignores_counts() {
    let m = BlockCtxMap {
        num_ctxs: 991,
        num_dc_ctxs: 997,
        ..BlockCtxMap::default()
    };
    let (bytes, bits) = encoded(&m, 7);
    assert_eq!(bits, 8);
    assert_eq!(bytes, [255]);
    let mut state = BlockCtxMap {
        dc_thresholds: [vec![3], vec![5], vec![7]],
        qf_thresholds: vec![11],
        ctx_map: vec![13],
        num_ctxs: 17,
        num_dc_ctxs: 19,
    };
    let mut br = BitReader::new(&bytes);
    br.skip_bits(7);
    decode_block_ctx_map(
        jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
        &mut br,
        &mut state,
    )
    .unwrap();
    br.close().unwrap();
    assert!(state.dc_thresholds.iter().all(Vec::is_empty));
    assert!(state.qf_thresholds.is_empty());
    assert_eq!(state.ctx_map, DEFAULT_CTX_MAP);
    assert_eq!(state.num_ctxs, 15);
    assert_eq!(state.num_dc_ctxs, 1);
}
#[test]
fn custom_maps_roundtrip_at_each_bit_offset_and_charge_ac() {
    let m = BlockCtxMap {
        dc_thresholds: [vec![-7, 17], vec![5], vec![]],
        qf_thresholds: vec![1, 45],
        ctx_map: (0..702).map(|i| (i % 16) as u8).collect(),
        num_ctxs: 16,
        num_dc_ctxs: 6,
    };
    for offset in 0..8 {
        let (bytes, bits) = encoded(&m, offset);
        let mut br = BitReader::new(&bytes);
        br.skip_bits(offset);
        let mut decoded = BlockCtxMap::default();
        decode_block_ctx_map(
            jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
            &mut br,
            &mut decoded,
        )
        .unwrap();
        assert_eq!(br.total_bits_consumed(), bits);
        br.close().unwrap();
        assert_eq!(decoded.dc_thresholds, m.dc_thresholds);
        assert_eq!(decoded.qf_thresholds, m.qf_thresholds);
        assert_eq!(decoded.ctx_map, m.ctx_map);
        assert_eq!(decoded.num_ctxs, 16);
        assert_eq!(decoded.num_dc_ctxs, 6);
    }
    let mut w = BitWriter::new();
    let mut aux = AuxOut::default();
    encode_block_ctx_map(&m, &mut w, Some(&mut aux)).unwrap();
    assert_eq!(aux.total_bits(), w.bits_written());
    assert_eq!(aux.layer(LayerType::Ac).total_bits, w.bits_written());
}
#[test]
fn oversized_thresholds_and_unencodable_qf_preserve_reached_mutations() {
    let m = BlockCtxMap {
        dc_thresholds: [vec![1; 4], vec![2; 4], vec![3]],
        qf_thresholds: vec![4],
        ctx_map: vec![0; 39],
        num_ctxs: 1,
        num_dc_ctxs: 50,
    };
    let (bytes, _) = encoded(&m, 0);
    let mut br = BitReader::new(&bytes);
    let mut decoded = BlockCtxMap {
        ctx_map: vec![99, 17],
        num_ctxs: 101,
        ..BlockCtxMap::default()
    };
    assert_eq!(
        decode_block_ctx_map(
            jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
            &mut br,
            &mut decoded
        ),
        Err(StatusCode::GenericError)
    );
    br.close().unwrap();
    assert_eq!(decoded.dc_thresholds, m.dc_thresholds);
    assert_eq!(decoded.qf_thresholds, m.qf_thresholds);
    assert_eq!(decoded.num_dc_ctxs, 50);
    assert_eq!(decoded.ctx_map, [99, 17]);
    assert_eq!(decoded.num_ctxs, 101);
    let bad = BlockCtxMap {
        qf_thresholds: vec![0],
        ..BlockCtxMap::default()
    };
    let mut w = BitWriter::new();
    let mut aux = AuxOut::default();
    assert_eq!(
        encode_block_ctx_map(&bad, &mut w, Some(&mut aux)),
        Err(StatusCode::GenericError)
    );
    assert_eq!(w.bits_written(), 17);
    assert_eq!(aux.layer(LayerType::Ac).total_bits, 17);
}
