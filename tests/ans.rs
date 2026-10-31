// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Local entropy regression cases; cross-language evidence is in parity/ans.rs.

use jxl::jxl::dec_ans::{ANSSymbolReader, HybridUintConfig, decode_uint_configs};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_ans::{Token, encode_uint_configs};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;

#[test]
fn hybrid_uint_extremes_roundtrip_with_exact_extra_bit_counts() {
    for cfg in [
        HybridUintConfig::new(0, 0, 0),
        HybridUintConfig::new(2, 0, 1),
        HybridUintConfig::default(),
        HybridUintConfig::new(15, 7, 8),
    ] {
        for value in [0, 1, 15, 16, 17, 255, 256, 65535, 1 << 31, u32::MAX] {
            let (token, nbits, bits) = cfg.encode(value);
            let mut writer = BitWriter::new();
            writer
                .with_max_bits(
                    40,
                    LayerType::Header,
                    None,
                    |w, _| {
                        w.write(nbits as usize, u64::from(bits));
                        w.zero_pad_to_byte();
                        Ok(())
                    },
                    false,
                )
                .unwrap();
            let mut reader = BitReader::new(writer.get_span());
            reader.refill();
            assert_eq!(
                ANSSymbolReader::read_hybrid_uint_config(cfg, token as usize, &mut reader),
                value
            );
            assert_eq!(reader.total_bits_consumed(), nbits as usize);
            assert_eq!(reader.close(), Ok(()));
        }
    }
}

#[test]
fn hybrid_config_vector_covers_all_ans_alphabet_widths() {
    for log_alpha in 5..=8 {
        let mut configs = Vec::new();
        for e in 0..log_alpha {
            for m in 0..=e {
                for l in 0..=e - m {
                    configs.push(HybridUintConfig::new(e, m, l));
                }
            }
        }
        configs.push(HybridUintConfig::new(log_alpha, 0, 0));
        let mut writer = BitWriter::new();
        writer
            .with_max_bits(
                10 * configs.len() + 7,
                LayerType::Header,
                None,
                |w, _| {
                    encode_uint_configs(&configs, w, log_alpha as usize);
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        let mut reader = BitReader::new(writer.get_span());
        let mut decoded = vec![HybridUintConfig::default(); configs.len()];
        assert_eq!(
            decode_uint_configs(log_alpha as usize, &mut decoded, &mut reader),
            Ok(())
        );
        assert_eq!(decoded, configs);
        assert_eq!(reader.close(), Ok(()));
    }
}

#[test]
fn token_context_retains_the_source_bitfield_truncation() {
    let mut token = Token::new(u32::MAX, 42);
    assert_eq!(token.context(), 0x7fff_ffff);
    assert!(!token.is_lz77_length());
    token.set_is_lz77_length(true);
    token.set_context(0x8000_0003);
    assert_eq!(token.context(), 3);
    assert!(token.is_lz77_length());
    assert_eq!(token.value, 42);
    assert_eq!(std::mem::size_of::<Token>(), 8);
}
