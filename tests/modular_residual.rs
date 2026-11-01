// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual sample/token/entropy/channel integration, without image framing.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_ans::{
    EntropyEncodingData, Token, build_and_encode_histograms, write_tokens_with_allotment,
};
use jxl::jxl::enc_ans_params::{HistogramParams, HybridUintMethod, LZ77Method};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::encoding::dec_ma::{PropertyDecisionNode as Node, Tree};
use jxl::jxl::modular::encoding::enc_encoding::modular_compress;
use jxl::jxl::modular::encoding::enc_ma::tokenize_tree;
use jxl::jxl::modular::encoding::encoding::{GroupHeader, TreeLut, decode_modular_channel_maans};
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::{ModularOptions, Predictor};

fn round_trip(image: &Image, t: &Tree, skip: bool, force: bool, rle: bool) -> (Vec<Token>, bool) {
    let mut tree = Vec::new();
    tokenize_tree(t, &mut Vec::new(), &mut tree).unwrap();
    let options = ModularOptions {
        predictor: Predictor::WEIGHTED,
        wp_mode: 2,
        skip_encoder_fast_path: skip,
        ..Default::default()
    };
    let mut header = GroupHeader::default();
    let mut streams = vec![Vec::new()];
    let mut width = 0;
    modular_compress(
        image,
        &options,
        13,
        &tree,
        &mut header,
        &mut streams[0],
        &mut width,
    )
    .unwrap();
    assert_eq!(width, image.w);
    let original_tokens = streams[0].clone();
    let params = HistogramParams {
        force_huffman: force,
        lz77_method: if rle {
            LZ77Method::RLE
        } else {
            LZ77Method::None
        },
        uint_method: HybridUintMethod::Uint000,
        image_widths: vec![width],
        ..Default::default()
    };
    let mut code = EntropyEncodingData::default();
    let mut writer = BitWriter::new();
    build_and_encode_histograms(
        &params,
        tree.len().div_ceil(2),
        &mut streams,
        &mut code,
        Some(&mut writer),
        LayerType::ModularGlobal,
        None,
    )
    .unwrap();
    write_tokens_with_allotment(
        &streams[0],
        &code,
        0,
        &mut writer,
        LayerType::ModularGlobal,
        None,
    )
    .unwrap();
    let bits = writer.bits_written();
    writer
        .with_max_bits(
            8,
            LayerType::ModularGlobal,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    let mut decoded =
        Image::create(image.w, image.h, image.bitdepth, image.channel.len() as i32).unwrap();
    for c in &mut decoded.channel {
        c.plane.bytes_mut().unwrap().fill(0xa5);
    }
    let mut br = BitReader::new(writer.get_span());
    let mut dc = ANSCode::default();
    let mut map = Vec::new();
    decode_histograms(&mut br, tree.len().div_ceil(2), &mut dc, &mut map, false).unwrap();
    let mut reader = ANSSymbolReader::create(&dc, &mut br, width).unwrap();
    let huff_rle = reader.is_huff_rle_only();
    let mut lut = Box::new(TreeLut::<u8, false, false>::default());
    let (mut run, mut value) = (0, 0);
    for c in 0..image.channel.len() {
        decode_modular_channel_maans(
            &mut br,
            &mut reader,
            &map,
            &tree,
            &header.wp_header,
            c as i32,
            13,
            &mut lut,
            &mut decoded,
            &mut run,
            &mut value,
        )
        .unwrap();
        br.all_reads_within_bounds().unwrap();
        if c == 0
            && image.w == 256
            && image.h == 1
            && tree.len() == 1
            && tree[0].predictor == Predictor::GRADIENT
            && huff_rle
        {
            assert!(run > 0, "The RLE run must continue in the next channel");
        }
    }
    assert!(reader.check_ans_final_state());
    assert_eq!(br.total_bits_consumed(), bits);
    br.close().unwrap();
    for c in 0..image.channel.len() {
        for y in 0..image.h {
            assert_eq!(
                &decoded.channel[c].row(y)[..image.w],
                &image.channel[c].row(y)[..image.w]
            );
            let p = &decoded.channel[c].plane;
            let from = y * p.bytes_per_row() + image.w * 4;
            let to = (y + 1) * p.bytes_per_row();
            assert!(p.bytes().unwrap()[from..to].iter().all(|&v| v == 0xa5));
        }
    }
    (original_tokens, huff_rle)
}
#[test]
fn every_predictor_round_trips_raster_samples_and_fast_paths_keep_tokens() {
    let mut image = Image::create(17, 5, 16, 3).unwrap();
    for (c, ch) in image.channel.iter_mut().enumerate() {
        for y in 0..5 {
            for x in 0..17 {
                ch.row_mut(y)[x] = ((x * 192 + y * 873 + c * 355) % 65536) as i32 - 32768;
            }
        }
    }
    for p in 0..14 {
        let t = vec![Node::leaf(Predictor(p), 0, 1)];
        for (force, rle) in [(false, false), (true, false), (false, true), (true, true)] {
            let (fast, _) = round_trip(&image, &t, false, force, rle);
            let (slow, _) = round_trip(&image, &t, true, force, rle);
            assert_eq!(
                fast.iter()
                    .map(|v| (v.context(), v.value))
                    .collect::<Vec<_>>(),
                slow.iter()
                    .map(|v| (v.context(), v.value))
                    .collect::<Vec<_>>()
            );
        }
    }
    for (property, predictor) in [
        (9, Predictor::GRADIENT),
        (15, Predictor::WEIGHTED),
        (18, Predictor::AVERAGE4),
    ] {
        let t = vec![
            Node::split(property, 0, 1, 2),
            Node::leaf(predictor, 0, 1),
            Node::leaf(predictor, 0, 1),
        ];
        for force in [false, true] {
            round_trip(&image, &t, false, force, false);
        }
    }
}
#[test]
fn gradient_huffman_rle_run_state_spans_channels_and_static_mixtures_stay_correct() {
    let mut image = Image::create(256, 1, 24, 3).unwrap();
    for (c, ch) in image.channel.iter_mut().enumerate() {
        let mut value = if c == 0 { 0 } else { 31 * 16 };
        for x in 0..256 {
            value += 8192 + (x as i32 / 8) * 16;
            ch.row_mut(0)[x] = value;
        }
    }
    let t = vec![Node::leaf(Predictor::GRADIENT, 0, 1)];
    let (_, specialized) = round_trip(&image, &t, false, true, true);
    assert!(specialized);
    let t = vec![
        Node::split(0, 0, 1, 2),
        Node::leaf(Predictor::LEFT, 0, 1),
        Node::leaf(Predictor::GRADIENT, 0, 1),
    ];
    let (_, specialized) = round_trip(&image, &t, false, true, true);
    assert!(specialized);
}
#[test]
fn gradient_known_tokens_and_overflow_preserve_header_width_and_written_prefix() {
    let mut image = Image::create(3, 2, 16, 1).unwrap();
    for y in 0..2 {
        image.channel[0].row_mut(y)[..3].copy_from_slice(&if y == 0 {
            [10, 20, 30]
        } else {
            [15, 25, 35]
        });
    }
    let options = ModularOptions {
        predictor: Predictor::GRADIENT,
        ..Default::default()
    };
    let t = vec![Node::leaf(Predictor::GRADIENT, 0, 1)];
    let mut h = GroupHeader::default();
    weighted::predictor_mode(2, &mut h.wp_header);
    let weights = h.wp_header.w;
    let mut tokens = vec![Token::new(5, 0xdeadbeef)];
    let mut width = 999;
    modular_compress(&image, &options, 0, &t, &mut h, &mut tokens, &mut width).unwrap();
    assert_eq!(
        tokens.iter().map(|t| t.value).collect::<Vec<_>>(),
        [0xdeadbeef, 20, 20, 20, 10, 10, 10]
    );
    assert_eq!(h.wp_header.w, weights);
    assert!(h.use_global_tree);
    assert_eq!(width, 3);
    image.channel[0].row_mut(0)[..3].copy_from_slice(&[i32::MIN, i32::MAX, i32::MIN]);
    width = 999;
    tokens.truncate(1);
    assert_eq!(
        modular_compress(&image, &options, 0, &t, &mut h, &mut tokens, &mut width),
        Err(StatusCode::GenericError)
    );
    assert_eq!(width, 999);
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].value, 0xdeadbeef);
    assert_eq!(
        [tokens[1].value, tokens[2].value, tokens[3].value],
        [u32::MAX, 1, 2]
    );
}
