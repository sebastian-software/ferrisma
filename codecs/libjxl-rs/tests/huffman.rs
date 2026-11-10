// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Local regression cases for tree ordering, depth limiting, and parser state.

use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_huffman::HuffmanDecodingData;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_huffman::build_and_store_huffman_tree;
use jxl::jxl::enc_huffman_tree::{convert_bit_depths_to_symbols, create_huffman_tree};
use jxl::jxl::huffman_table::HuffmanCode;

#[test]
fn equal_counts_keep_upstream_symbol_order() {
    let mut depth = [0; 5];
    let mut bits = [0; 5];
    create_huffman_tree(&[1; 5], 15, &mut depth);
    convert_bit_depths_to_symbols(&depth, &mut bits);
    assert_eq!(depth, [2, 2, 2, 3, 3]);
    assert_eq!(bits, [0, 2, 1, 3, 7]);
}

#[test]
fn skewed_histogram_limits_depth_and_decodes_through_second_level() {
    let mut histogram = [0; 32];
    let (mut a, mut b) = (1, 1);
    for value in &mut histogram {
        *value = a;
        (a, b) = (b, a + b);
    }
    for offset in 0..8 {
        let mut depth = [0; 32];
        let mut bits = [0; 32];
        let mut writer = BitWriter::new();
        writer
            .with_max_bits(
                4096,
                LayerType::Header,
                None,
                |w, _| {
                    w.write(offset, (1 << offset) - 1);
                    build_and_store_huffman_tree(&histogram, &mut depth, &mut bits, w)?;
                    assert_eq!(depth.iter().copied().max(), Some(15));
                    for i in (0..32).chain((0..32).rev()) {
                        w.write(usize::from(depth[i]), u64::from(bits[i]));
                    }
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        let mut reader = BitReader::new(writer.get_span());
        reader.skip_bits(offset);
        let mut data = HuffmanDecodingData::default();
        assert!(data.read_from_bit_stream(32, &mut reader));
        assert!(data.table.len() > 256);
        for expected in (0u16..32).chain((0u16..32).rev()) {
            reader.refill();
            assert_eq!(data.read_symbol(&mut reader), expected);
        }
        assert_eq!(reader.jump_to_byte_boundary(), Ok(()));
        assert_eq!(reader.total_bits_consumed(), writer.bits_written());
        assert_eq!(reader.close(), Ok(()));
    }
}

#[test]
fn failed_reads_preserve_the_source_table_and_consumption_order() {
    let seed = HuffmanCode { bits: 3, value: 17 };
    let mut data = HuffmanDecodingData {
        table: vec![seed; 512],
    };
    let mut oversized = BitReader::new(&[]);
    assert!(!data.read_from_bit_stream(32769, &mut oversized));
    assert_eq!(oversized.total_bits_consumed(), 0);
    assert_eq!(data.table, vec![seed; 512]);
    assert_eq!(oversized.close(), Ok(()));

    let mut duplicate = BitReader::new(&[0x05]);
    assert!(!data.read_from_bit_stream(2, &mut duplicate));
    assert_eq!(duplicate.total_bits_consumed(), 6);
    assert_eq!(data.table, vec![seed; 256]);
    assert_eq!(duplicate.close(), Ok(()));

    let mut truncated = BitReader::new(&[]);
    assert!(!data.read_from_bit_stream(32, &mut truncated));
    assert_eq!(truncated.total_bits_consumed(), 38);
    assert_eq!(data.table, vec![seed; 256]);
    assert!(truncated.close().is_err());
}
