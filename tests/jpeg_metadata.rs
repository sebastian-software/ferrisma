// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Golden words and states captured from the pinned original ordinary and
// ASan/UBSan JPEGData bundle, not calculated from the Rust implementation.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::fields::bundle;
use jxl::jxl::jpeg::jpeg_data::{AppMarkerType, JpegData};
use jxl::jxl::jpeg::jpeg_data_fields::set_jpeg_data_from_icc;

fn captured() -> Vec<u8> {
    let h = include_str!("data/jpeg-metadata/gray.bundle.hex").trim();
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
        .collect()
}
#[test]
fn jpeg_metadata_original_unaligned_gray_bundle() {
    let bytes = captured();
    let mut reader = BitReader::new(&bytes);
    reader.skip_bits(3);
    let mut j = JpegData::default();
    assert_eq!(bundle::read(&mut reader, &mut j), Ok(()));
    assert_eq!(reader.total_bits_consumed(), 1844);
    assert_eq!(reader.close(), Ok(()));
    assert_eq!(j.marker_order, [0xe0, 0xdb, 0xc0, 0xc4, 0xc4, 0xda, 0xd9]);
    assert_eq!(j.components.len(), 1);
    assert_eq!(j.components[0].id, 1);
    assert_eq!(j.quant.len(), 1);
    assert_eq!(j.quant[0].values, [0; 64]);
    assert_eq!(j.huffman_code.len(), 2);
    assert_eq!(j.huffman_code[0].slot_id, 0);
    assert_eq!(j.huffman_code[1].slot_id, 16);
    assert_eq!(j.scan_info.len(), 1);
    assert_eq!(j.scan_info[0].num_components, 1);
    assert_eq!(j.scan_info[0].se, 63);
    assert_eq!(j.app_data[0], [0; 17]);
    assert!(j.tail_data.is_empty());
}
#[test]
fn jpeg_metadata_original_truncated_read_retains_initialized_owners() {
    let bytes = captured();
    let mut reader = BitReader::new(&bytes[..10]);
    reader.skip_bits(3);
    let mut j = JpegData::default();
    assert_eq!(
        bundle::read(&mut reader, &mut j),
        Err(StatusCode::NotEnoughBytes)
    );
    assert_eq!(reader.total_bits_consumed(), 82);
    assert_eq!(reader.close(), Ok(()));
    assert_eq!(j.marker_order, [0xe0, 0xdb, 0xc0, 0xc4, 0xc4, 0xda, 0xd9]);
    assert_eq!(j.app_data, [vec![0; 17]]);
    assert_eq!(j.app_marker_type, [AppMarkerType::UNKNOWN]);
    assert_eq!(j.quant.len(), 1);
    assert_eq!(j.huffman_code.len(), 2);
    assert_eq!(j.huffman_code[0].counts, [0; 17]);
    assert_eq!(j.components.len(), 1);
    assert_eq!(j.components[0].id, 1);
    assert_eq!(j.scan_info.len(), 1);
    assert_eq!(j.scan_info[0].num_components, 0);
}
#[test]
fn jpeg_metadata_icc_error_keeps_completed_marker_write() {
    let mut j = JpegData {
        app_data: vec![vec![0xa5; 18], vec![0x5a; 20]],
        app_marker_type: vec![AppMarkerType::ICC; 2],
        ..Default::default()
    };
    assert_eq!(
        set_jpeg_data_from_icc(&[7, 8], &mut j),
        Err(StatusCode::GenericError)
    );
    assert_eq!(j.app_data[0][17], 7);
    assert_eq!(j.app_data[1], [0x5a; 20]);
}
