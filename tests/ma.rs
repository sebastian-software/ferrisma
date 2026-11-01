// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Integration of real tree tokens, entropy framing, parsing, and prediction.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_ans::{
    EntropyEncodingData, build_and_encode_histograms, write_tokens_with_allotment,
};
use jxl::jxl::enc_ans_params::{HistogramParams, LZ77Method};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::modular::encoding::context_predict::MATreeLookup;
use jxl::jxl::modular::encoding::dec_ma::{PropertyDecisionNode as Node, Tree, decode_tree};
use jxl::jxl::modular::encoding::enc_ma::tokenize_tree;
use jxl::jxl::modular::encoding::encoding::{
    GroupHeader, TreeLut, filter_tree, tree_to_lookup_table,
};
use jxl::jxl::modular::encoding::ma_common::{MAX_TREE_SIZE, NUM_TREE_CONTEXTS};
use jxl::jxl::modular::options::Predictor;

fn encode(tree: &Tree, force_huffman: bool) -> (BitWriter, Tree) {
    let mut tokens = vec![Vec::new()];
    let mut decoder = Vec::new();
    tokenize_tree(tree, &mut tokens[0], &mut decoder).unwrap();
    let params = HistogramParams {
        force_huffman,
        lz77_method: LZ77Method::None,
        ..Default::default()
    };
    let mut code = EntropyEncodingData::default();
    let mut writer = BitWriter::new();
    build_and_encode_histograms(
        &params,
        NUM_TREE_CONTEXTS,
        &mut tokens,
        &mut code,
        Some(&mut writer),
        LayerType::ModularTree,
        None,
    )
    .unwrap();
    write_tokens_with_allotment(
        &tokens[0],
        &code,
        0,
        &mut writer,
        LayerType::ModularTree,
        None,
    )
    .unwrap();
    writer
        .with_max_bits(
            8,
            LayerType::ModularTree,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    (writer, decoder)
}
#[test]
fn serialized_tree_selects_original_contexts_after_static_pruning() {
    let input = vec![
        Node::split(0, 0, 1, 2),
        Node::split(15, 4, 3, 4),
        Node::split(9, -2, 5, 6),
        Node::leaf(Predictor::WEIGHTED, -3, 2),
        Node::leaf(Predictor::WEIGHTED, 0, 1),
        Node::leaf(Predictor::GRADIENT, 5, 3),
        Node::leaf(Predictor::GRADIENT, 0, 1),
    ];
    for force in [false, true] {
        let (w, expected) = encode(&input, force);
        let mut br = BitReader::new(w.get_span());
        let mut tree = Vec::new();
        decode_tree(&mut br, &mut tree, MAX_TREE_SIZE).unwrap();
        br.close().unwrap();
        assert_eq!(tree, expected);
        for channel in [0, 1] {
            let (mut n, mut wp, mut only, mut gradient) = (0, false, false, false);
            let flat = filter_tree(
                &tree,
                &mut [channel, 17],
                &mut n,
                &mut wp,
                &mut only,
                &mut gradient,
            );
            assert_eq!(
                (n, wp, only, gradient),
                (16, channel == 1, channel == 1, channel == 0)
            );
            let lookup = MATreeLookup::new(&flat);
            for (v, context, offset, multiplier) in if channel == 0 {
                vec![(-3, 3, 0, 1), (-2, 3, 0, 1), (-1, 2, 5, 3)]
            } else {
                vec![(4, 1, 0, 1), (5, 0, -3, 2)]
            } {
                let mut properties = vec![0; 16];
                properties[if channel == 0 { 9 } else { 15 }] = v;
                let result = lookup.lookup(&properties);
                assert_eq!(
                    (result.context, result.offset, result.multiplier),
                    (context, offset, multiplier)
                );
                assert_eq!(
                    result.predictor,
                    if channel == 0 {
                        Predictor::GRADIENT
                    } else {
                        Predictor::WEIGHTED
                    }
                );
            }
            if channel == 0 {
                let mut lut = TreeLut::<u8, true, true>::default();
                assert!(tree_to_lookup_table(&flat, &mut lut));
                for value in -8192..8192 {
                    let i = (value + 8192) as usize;
                    assert_eq!(
                        (lut.context_lookup[i], lut.offsets[i], lut.multipliers[i]),
                        if value > -2 { (2, 5, 3) } else { (3, 0, 1) }
                    );
                }
            }
        }
    }
}
#[test]
fn decoder_size_checkpoint_accepts_final_leaf_but_retains_unfinished_prefix() {
    let (w, expected) = encode(&vec![Node::leaf(Predictor::LEFT, 0, 1)], false);
    let mut br = BitReader::new(w.get_span());
    let mut tree = Vec::new();
    decode_tree(&mut br, &mut tree, 0).unwrap();
    br.close().unwrap();
    assert_eq!(tree, expected);
    let input = vec![
        Node::split(9, 0, 1, 2),
        Node::leaf(Predictor::LEFT, 0, 1),
        Node::leaf(Predictor::TOP, 0, 1),
    ];
    let (w, expected) = encode(&input, false);
    let mut br = BitReader::new(w.get_span());
    assert_eq!(
        decode_tree(&mut br, &mut tree, 0),
        Err(StatusCode::GenericError)
    );
    br.close().unwrap();
    assert_eq!(tree, &expected[..1]);
}
#[test]
fn invalid_nested_split_keeps_parsed_nodes_and_reused_group_defaults_keep_transforms() {
    let input = vec![
        Node::split(9, 0, 1, 2),
        Node::split(9, -1, 3, 4),
        Node::leaf(Predictor::ZERO, 0, 1),
        Node::leaf(Predictor::ZERO, 0, 1),
        Node::leaf(Predictor::ZERO, 0, 1),
    ];
    let (w, expected) = encode(&input, true);
    let mut br = BitReader::new(w.get_span());
    let mut tree = Vec::new();
    assert_eq!(
        decode_tree(&mut br, &mut tree, MAX_TREE_SIZE),
        Err(StatusCode::GenericError)
    );
    br.close().unwrap();
    assert_eq!(tree, expected);
    let mut header = GroupHeader::default();
    header.transforms.push(Default::default());
    jxl::jxl::fields::bundle::set_default(&mut header);
    assert_eq!(header.transforms.len(), 1); // Source resizes only when reading.
}
