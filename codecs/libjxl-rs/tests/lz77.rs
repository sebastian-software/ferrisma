// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Exercise the source's LZ77 token representation independently of its encoder.

use jxl::jxl::dec_ans::{LZ77Params, NUM_SPECIAL_DISTANCES, special_distance};
use jxl::jxl::enc_ans::Token;
use jxl::jxl::enc_ans_params::{HistogramParams, LZ77Method};
use jxl::jxl::enc_lz77::apply_lz77;

#[test]
fn every_compressing_method_expands_to_the_input_across_streams() {
    let methods = [
        LZ77Method::RLE,
        LZ77Method::LZ77b1w3f,
        LZ77Method::LZ77b3w3f,
        LZ77Method::LZ77b7w3f,
        LZ77Method::LZ77b15w3f,
        LZ77Method::LZ77b31w3f,
        LZ77Method::LZ77b1w3t,
        LZ77Method::LZ77b3w3t,
        LZ77Method::LZ77b7w3t,
        LZ77Method::LZ77b15w3t,
        LZ77Method::LZ77b31w3t,
        LZ77Method::Optc1,
        LZ77Method::Optc3,
        LZ77Method::Optc8,
        LZ77Method::Optc256,
    ];
    let input: Vec<_> = (0..4096)
        .map(|i| Token::new(i % 3, (i / 17) % 13))
        .collect();
    let streams = vec![input.clone(), Vec::new(), input];
    let lz77 = LZ77Params {
        nonserialized_distance_context: 3,
        ..LZ77Params::default()
    };
    for method in methods {
        for force_huffman in [false, true] {
            for width in [0, 1, 7] {
                let params = HistogramParams {
                    force_huffman,
                    lz77_method: method,
                    image_widths: vec![width; streams.len()],
                    ..HistogramParams::default()
                };
                let compressed = apply_lz77(&params, 3, &streams, &lz77);
                assert_eq!(compressed.len(), streams.len(), "{method:?}");
                assert!(compressed[0].len() < streams[0].len(), "{method:?}");
                for (encoded, original) in compressed.iter().zip(&streams) {
                    let mut decoded = Vec::new();
                    let mut tokens = encoded.iter();
                    while let Some(token) = tokens.next() {
                        if !token.is_lz77_length() {
                            decoded.push(token.value);
                            continue;
                        }
                        let dist = tokens.next().expect("copy distance follows length");
                        assert!(!dist.is_lz77_length());
                        assert_eq!(dist.context(), 3);
                        let distance = if width != 0 && dist.value < NUM_SPECIAL_DISTANCES as u32 {
                            special_distance(dist.value as usize, width as i32) as usize
                        } else {
                            dist.value as usize + 1
                                - if width == 0 { 0 } else { NUM_SPECIAL_DISTANCES }
                        };
                        assert!(distance != 0 && distance <= decoded.len());
                        for _ in 0..token.value + lz77.min_length {
                            decoded.push(decoded[decoded.len() - distance]);
                        }
                    }
                    assert_eq!(
                        decoded,
                        original.iter().map(|t| t.value).collect::<Vec<_>>()
                    );
                }
            }
        }
    }
}
