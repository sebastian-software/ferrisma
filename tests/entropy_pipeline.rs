// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Complete native entropy entry points, without test-supplied framing/tables.

use jxl::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_ans::{
    EntropyEncodingData, Token, build_and_encode_histograms, encode_histograms,
    write_tokens_with_allotment,
};
use jxl::jxl::enc_ans_params::{HistogramParams, LZ77Method};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;

#[test]
fn complete_encoder_round_trips_ans_prefix_lz77_and_streaming_headers() {
    let original = vec![
        (0..1024)
            .map(|i| Token::new(i % 3, (i / 17) % 13))
            .collect::<Vec<_>>(),
        (0..1024)
            .map(|i| Token::new((i + 1) % 3, i * 65537))
            .collect(),
    ];
    for streaming in [false, true] {
        for force_huffman in [false, true] {
            for lz77_method in [LZ77Method::None, LZ77Method::RLE, LZ77Method::Optc8] {
                let params = HistogramParams {
                    streaming_mode: streaming,
                    force_huffman,
                    lz77_method,
                    image_widths: vec![7; original.len()],
                    ..HistogramParams::default()
                };
                let mut tokens = original.clone();
                let mut codes = EntropyEncodingData::default();
                let mut writer = BitWriter::new();
                build_and_encode_histograms(
                    &params,
                    3,
                    &mut tokens,
                    &mut codes,
                    if streaming { None } else { Some(&mut writer) },
                    LayerType::Ac,
                    None,
                )
                .unwrap();
                if streaming {
                    encode_histograms(&mut codes, &mut writer, LayerType::Ac, None).unwrap();
                }
                for stream in &tokens {
                    write_tokens_with_allotment(
                        stream,
                        &codes,
                        0,
                        &mut writer,
                        LayerType::Ac,
                        None,
                    )
                    .unwrap();
                }
                let bits = writer.bits_written();
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
                let mut br = BitReader::new(writer.get_span());
                let mut code = ANSCode::default();
                let mut map = Vec::new();
                decode_histograms(
                    jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
                    &mut br,
                    3,
                    &mut code,
                    &mut map,
                    false,
                )
                .unwrap();
                assert_eq!(code.use_prefix_code, force_huffman);
                assert_eq!(code.lz77.enabled, lz77_method != LZ77Method::None);
                for stream in &original {
                    let mut reader = ANSSymbolReader::create(&code, &mut br, 7).unwrap();
                    for token in stream {
                        assert_eq!(
                            reader.read_hybrid_uint(token.context() as usize, &mut br, &map),
                            token.value as usize
                        );
                    }
                    assert!(reader.check_ans_final_state());
                }
                assert_eq!(br.total_bits_consumed(), bits);
                assert!(br.close().is_ok());
            }
        }
    }
}
