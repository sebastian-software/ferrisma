// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::{
    dec_bit_reader::BitReader,
    dec_frame::SequenceDecoder,
    fields::bundle,
    frame_header::ColorTransform,
    headers::read_size_header,
    image_metadata::{CodecMetadata, read_image_metadata},
};
struct Capture {
    wire: &'static [u8],
    pixels: &'static [u8],
    width: usize,
    height: usize,
    gray: bool,
}
macro_rules! capture {
    ($name:literal,$w:literal,$h:literal,$gray:literal) => {
        Capture {
            wire: include_bytes!(concat!("data/", $name, ".jxl")),
            pixels: include_bytes!(concat!("data/", $name, ".float-pixels")),
            width: $w,
            height: $h,
            gray: $gray,
        }
    };
}
#[test]
fn captured_original_jpeg_chroma_files_decode_without_cpp() {
    // Genuine baseline JPEG inputs were encoded by original public JxlEncoder
    // entry points. Both original graphs independently agree on all captured
    // initialized output words, stored in planar little-endian binary32 order.
    for capture in [
        capture!("ycbcr-jpeg-444", 65, 33, false),
        capture!("ycbcr-jpeg-422", 65, 33, false),
        capture!("ycbcr-jpeg-420", 65, 33, false),
        capture!("ycbcr-jpeg-440", 65, 33, false),
        capture!("ycbcr-jpeg-gray", 65, 33, true),
        capture!("ycbcr-jpeg-444-1x1", 1, 1, false),
        capture!("ycbcr-jpeg-422-1x1", 1, 1, false),
        capture!("ycbcr-jpeg-420-1x1", 1, 1, false),
        capture!("ycbcr-jpeg-440-1x1", 1, 1, false),
        capture!("ycbcr-jpeg-gray-1x1", 1, 1, true),
        capture!("ycbcr-jpeg-444-9x7", 9, 7, false),
        capture!("ycbcr-jpeg-422-9x7", 9, 7, false),
        capture!("ycbcr-jpeg-420-9x7", 9, 7, false),
        capture!("ycbcr-jpeg-440-9x7", 9, 7, false),
        capture!("ycbcr-jpeg-gray-9x7", 9, 7, true),
        capture!("ycbcr-jpeg-444-257x3", 257, 3, false),
        capture!("ycbcr-jpeg-422-257x3", 257, 3, false),
        capture!("ycbcr-jpeg-420-257x3", 257, 3, false),
        capture!("ycbcr-jpeg-440-257x3", 257, 3, false),
        capture!("ycbcr-jpeg-gray-257x3", 257, 3, true),
        capture!("ycbcr-jpeg-444-3x257", 3, 257, false),
        capture!("ycbcr-jpeg-422-3x257", 3, 257, false),
        capture!("ycbcr-jpeg-420-3x257", 3, 257, false),
        capture!("ycbcr-jpeg-440-3x257", 3, 257, false),
        capture!("ycbcr-jpeg-gray-3x257", 3, 257, true),
    ] {
        let mut metadata = CodecMetadata::default();
        let mut bits = BitReader::new(capture.wire);
        assert_eq!(bits.read_fixed_bits::<16>(), 0x0aff);
        read_size_header(&mut bits, &mut metadata.size).unwrap();
        read_image_metadata(&mut bits, &mut metadata.m).unwrap();
        metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
        bundle::read(&mut bits, &mut metadata.transform_data).unwrap();
        assert!(!metadata.m.color_encoding.want_icc());
        assert_eq!(metadata.m.color_encoding.is_gray(), capture.gray);
        bits.jump_to_byte_boundary().unwrap();
        let offset = bits.total_bits_consumed() / 8;
        bits.close().unwrap();
        let mut decoder = SequenceDecoder::new(&metadata).unwrap();
        let frame = decoder.decode_frame(&capture.wire[offset..], false);
        assert_eq!(frame.status, Ok(()));
        assert_eq!(frame.header.unwrap().color_transform, ColorTransform::YCBCR);
        let image = decoder.output();
        assert_eq!(
            (image.color().xsize(), image.color().ysize()),
            (capture.width, capture.height)
        );
        let words: Vec<_> = (0..3)
            .flat_map(|c| {
                (0..capture.height).flat_map(move |y| (0..capture.width).map(move |x| (c, x, y)))
            })
            .map(|(c, x, y)| image.color().plane_row(c, y)[x].to_bits())
            .collect();
        let expected: Vec<_> = capture
            .pixels
            .chunks_exact(4)
            .map(|v| u32::from_le_bytes(v.try_into().unwrap()))
            .collect();
        assert_eq!(words, expected);
        assert!(image.extra_channels().is_empty());
    }
}
