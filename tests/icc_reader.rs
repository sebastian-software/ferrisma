// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::{
    base::status::StatusCode, dec_bit_reader::BitReader, icc_codec::ICCReader,
    padded_bytes::PaddedBytes,
};

fn bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

// Captured using pristine WriteICC in both ordinary and ASan/UBSan graphs.
// Both returned 708 bits. Profile bytes use the captured 128-byte header and
// captured 64-byte body period; bytes 128..132 are the zero tag count.
fn fixture() -> (Vec<u8>, Vec<u8>) {
    let stream = bytes(
        "13400103d2d2d9e0daf95d1705839d0f868ff9b6bb30c07080e1cbeec2c7300cc3f0672e9b8c76398c0275dfe0e301f3763d2b4e582437544186acd1beeaa87b4d28f9b308243e2457442b99d73e1396d890ca3972f9e7fc0e",
    );
    let header = bytes(
        "0000200000000000040000006d6e74725247422058595a2000000000000000000000000061637370000000000000000000000000000000000000000000000000000000000000f6d6000100000000d32d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    );
    let period = bytes(
        "135babfb2ba3e323639bab7b2b937333d39b4bfb4b83c303439bcb7bcb937333d39b2bfbaba3e32363db2b7babd37333935b0bfb8b43034383db0b7b8bd373b3",
    );
    let mut profile: Vec<u8> = (0..8192).map(|i| period[i % period.len()]).collect();
    profile[..128].copy_from_slice(&header);
    profile[128..132].fill(0);
    (stream, profile)
}

#[test]
fn original_process_before_init_clears_output_only_after_successful_eoi() {
    let mut reader = ICCReader::new();
    let mut output = PaddedBytes::default();
    output.append(&[0x12, 0x34]).unwrap();
    let mut bits = BitReader::new(&[]);
    bits.skip_bits(1);
    assert_eq!(
        reader.process(&mut bits, &mut output),
        Err(StatusCode::NotEnoughBytes)
    );
    assert_eq!(bits.total_bits_consumed(), 1);
    assert_eq!(output.as_slice(), [0x12, 0x34]);
    bits.close().unwrap();
    reader.reset();
    let mut bits = BitReader::new(&[]);
    assert_eq!(
        reader.process(&mut bits, &mut output),
        Err(StatusCode::GenericError)
    );
    assert!(output.is_empty());
    bits.close().unwrap();
}

#[test]
fn original_writeicc_stream_survives_reader_owner_moves_and_reset_reuse() {
    let (stream, profile) = fixture();
    let mut reader = ICCReader::new();
    for _ in 0..2 {
        let mut bits = BitReader::new(&stream);
        reader.init(&mut bits).unwrap();
        // Move every owning field after Init has bound the entropy code.
        let mut owner = Box::new(reader);
        let mut output = PaddedBytes::default();
        owner.process(&mut bits, &mut output).unwrap();
        assert_eq!(bits.total_bits_consumed(), 708);
        assert_eq!(output.as_slice(), profile);
        bits.close().unwrap();
        reader = *owner;
        reader.reset();
    }
}

#[test]
fn original_partial_lz77_decode_restores_checkpoint_and_resumes_after_owner_move() {
    let (stream, profile) = fixture();
    let mut reader = ICCReader::new();
    let mut output = PaddedBytes::default();
    output.append(&[0x12, 0x34]).unwrap();
    let mut partial = BitReader::new(&stream[..47]);
    reader.init(&mut partial).unwrap();
    // Original: i restores to 20, while the still-defined written prefix is 512.
    assert_eq!(
        reader.process(&mut partial, &mut output),
        Err(StatusCode::NotEnoughBytes)
    );
    assert_eq!(partial.total_bits_consumed(), 1877);
    assert_eq!(output.as_slice(), [0x12, 0x34]);
    partial.close().unwrap();
    let mut owners = Vec::from([reader]);
    let mut reader = owners.pop().unwrap();
    let mut full = BitReader::new(&stream);
    reader.init(&mut full).unwrap();
    assert_eq!(full.total_bits_consumed(), 373);
    reader.process(&mut full, &mut output).unwrap();
    assert_eq!(full.total_bits_consumed(), 708);
    assert_eq!(output.as_slice(), profile);
    full.close().unwrap();
}

#[test]
fn original_failed_histogram_update_retains_cached_huffman_mode_and_live_tables() {
    let request = include_str!("data/icc-reader-cache.requests")
        .lines()
        .next()
        .unwrap();
    let mut words = request.split_whitespace();
    assert_eq!(words.next(), Some("ir-run"));
    let mut output = PaddedBytes::default();
    output.append(&bytes(words.next().unwrap())).unwrap();
    assert_eq!(words.next(), Some("3"));
    let mut reader = ICCReader::new();
    // Actual ordinary and ASan original captures: two fatal Init calls leave
    // the old cached Huffman/config views valid. Direct Process is defined,
    // consumes 163 bits, then clears the seed before UnpredictICC fails.
    for (expected_op, expected_bits) in [(0, 407), (0, 67), (1, 163)] {
        let op: usize = words.next().unwrap().parse().unwrap();
        assert_eq!(op, expected_op);
        let skip = words.next().unwrap().parse().unwrap();
        let data = bytes(words.next().unwrap());
        let mut bits = BitReader::new(&data);
        bits.skip_bits(skip);
        let status = if op == 0 {
            reader.init(&mut bits)
        } else {
            reader.process(&mut bits, &mut output)
        };
        assert_eq!(status, Err(StatusCode::GenericError));
        assert_eq!(bits.total_bits_consumed(), expected_bits);
        if op == 0 {
            assert_eq!(output.as_slice(), [0x12, 0x34]);
        } else {
            assert!(output.is_empty());
        }
        bits.close().unwrap();
    }
}
