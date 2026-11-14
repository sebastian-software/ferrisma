// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_external_image::convert_uint_to_buffer;
use jxl::jxl::dec_frame::SequenceDecoder;
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::types::JxlEndianness;
use std::path::PathBuf;

// Each stored byte buffer was independently captured from actual ordinary and
// ASan/UBSan originals. decoder-geometry-fixtures.json retains both source
// requests and observations; decoder_geometry_reference.py reproduces them.
#[test]
fn complete_source_geometry_files_keep_the_independently_captured_rgba_bytes() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/decoder-geometry");
    let mut tested = 0;
    for row in include_str!("data/decoder-geometry-native.requests").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        assert_eq!(fields.len(), 3);
        let bytes = std::fs::read(data.join(fields[1])).unwrap();
        let expected = std::fs::read(data.join(fields[2])).unwrap();
        let mut metadata = CodecMetadata::default();
        let mut reader = BitReader::new(&bytes);
        assert_eq!(reader.read_fixed_bits::<16>(), 0x0aff);
        jxl::jxl::headers::read_size_header(&mut reader, &mut metadata.size).unwrap();
        jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut metadata.m).unwrap();
        metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
        jxl::jxl::fields::bundle::read(&mut reader, &mut metadata.transform_data).unwrap();
        assert!(!metadata.m.color_encoding.want_icc());
        assert!(!metadata.m.have_preview);
        reader.jump_to_byte_boundary().unwrap();
        reader.all_reads_within_bounds().unwrap();
        let mut offset = reader.total_bits_consumed() / 8;
        reader.close().unwrap();
        let mut decoder = SequenceDecoder::new(&metadata).unwrap();
        let mut frames = 0;
        loop {
            assert!(frames < 64);
            frames += 1;
            let decoded = decoder.decode_frame(&bytes[offset..], false);
            decoded.status.unwrap();
            offset += decoder.output().decoded_bytes();
            if decoded.header.unwrap().is_last {
                break;
            }
        }
        let image = decoder.output();
        assert!(std::ptr::eq(image.metadata().unwrap(), &metadata.m));
        let (width, height) = (
            metadata.oriented_xsize(false),
            metadata.oriented_ysize(false),
        );
        let stride = width * 4 + 16;
        let mut pixels = vec![0xa5; stride * height + 7];
        let capacity = pixels.len();
        convert_uint_to_buffer(
            &image,
            8,
            4,
            JxlEndianness::LITTLE,
            stride,
            &mut pixels,
            capacity,
            metadata.m.get_orientation(),
            true,
        )
        .unwrap();
        assert_eq!(pixels, expected, "{}", fields[0]);
        tested += 1;
    }
    assert_eq!(tested, 154);
}
