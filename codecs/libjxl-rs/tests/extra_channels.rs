// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_frame::SequenceDecoder;
use jxl::jxl::image_metadata::CodecMetadata;
fn metadata(bytes: &[u8]) -> (CodecMetadata, usize) {
    let mut m = CodecMetadata::default();
    let mut reader = BitReader::new(bytes);
    assert_eq!(reader.read_fixed_bits::<16>(), 0x0aff);
    jxl::jxl::headers::read_size_header(&mut reader, &mut m.size).unwrap();
    jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut m.m).unwrap();
    m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
    jxl::jxl::fields::bundle::read(&mut reader, &mut m.transform_data).unwrap();
    reader.jump_to_byte_boundary().unwrap();
    reader.all_reads_within_bounds().unwrap();
    let position = reader.total_bits_consumed() / 8;
    reader.close().unwrap();
    (m, position)
}
#[test]
fn all_extra_channel_words_match_the_independent_pristine_native_dyadic_capture() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let bytes = include_bytes!("data/extra-channel-all-types-dyadic.jxl");
    let (m, offset) = metadata(bytes);
    assert_eq!(m.m.num_extra_channels, 10);
    assert_eq!(
        m.m.extra_channel_info
            .iter()
            .map(|ec| ec.channel_type.0)
            .collect::<Vec<_>>(),
        [1, 0, 2, 3, 4, 5, 6, 16, 0, 2]
    );
    let mut decoder = SequenceDecoder::new(&m).unwrap();
    decoder
        .decode_frame(&bytes[offset..], false)
        .status
        .unwrap();
    let output = decoder.output();
    assert!(std::ptr::eq(output.metadata().unwrap(), &m.m));
    assert!(output.has_alpha());
    assert!(output.alpha_is_premultiplied());
    assert!(std::ptr::eq(
        output.alpha().unwrap(),
        &output.extra_channels()[1]
    ));
    let mut captured = Vec::new();
    for y in 0..output.ysize() {
        for x in 0..output.xsize() {
            for c in 0..3 {
                captured
                    .extend_from_slice(&output.color().plane_row(c, y)[x].to_bits().to_le_bytes());
            }
        }
    }
    for ec in output.extra_channels() {
        assert_eq!((ec.xsize(), ec.ysize()), (9, 7));
        for y in 0..ec.ysize() {
            for x in 0..ec.xsize() {
                captured.extend_from_slice(&ec.row(y)[x].to_bits().to_le_bytes());
            }
        }
    }
    assert_eq!(
        captured,
        include_bytes!("data/extra-channel-all-types-dyadic.float-pixels").as_slice()
    );
}
#[test]
fn moving_a_multichannel_sequence_retains_the_metadata_and_reference_owners() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let bytes = include_bytes!("data/extra-channel-reference-0-2.jxl");
    let (m, mut offset) = metadata(bytes);
    let mut decoder = SequenceDecoder::new(&m).unwrap();
    let first = decoder.decode_frame(&bytes[offset..], false);
    first.status.unwrap();
    assert!(!first.header.unwrap().is_last);
    offset += decoder.output().decoded_bytes();
    let owner = std::rc::Rc::downgrade(&decoder.state().frame_storage_for_referencing);
    let mut decoder = std::convert::identity(decoder);
    assert!(owner.upgrade().is_some());
    let second = decoder.decode_frame(&bytes[offset..], false);
    second.status.unwrap();
    assert!(second.header.unwrap().is_last);
    assert!(std::ptr::eq(decoder.output().metadata().unwrap(), &m.m));
    assert_eq!(decoder.output().extra_channels().len(), 10);
    drop(decoder);
    assert!(owner.upgrade().is_none());
}
