// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Integration regressions for translated codestream and frame header writers.

use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_fields::{write_codestream_headers, write_frame_header};
use jxl::jxl::fields::bundle;
use jxl::jxl::frame_header::{
    ColorTransform, FrameEncoding, FrameHeader, FrameSize, FrameType, read_frame_header,
};
use jxl::jxl::image_metadata::{CodecMetadata, read_image_metadata};

fn pad(writer: &mut BitWriter) {
    writer
        .with_max_bits(
            7,
            LayerType::Header,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
}

#[test]
fn modular_frame_header_follows_real_codestream_metadata_for_rgb_and_gray() {
    for gray in [false, true] {
        for (x, y) in [(1, 1), (257, 129), (513, 257)] {
            let mut metadata = CodecMetadata::default();
            metadata.size.set(x, y).unwrap();
            metadata.m.xyb_encoded = false;
            metadata.m.set_uint_samples(8);
            metadata
                .m
                .color_encoding
                .set_srgb(
                    if gray {
                        ColorSpace::GRAY
                    } else {
                        ColorSpace::RGB
                    },
                    RenderingIntent::RELATIVE,
                )
                .unwrap();
            let mut writer = BitWriter::new();
            write_codestream_headers(&mut metadata, &mut writer, None).unwrap();
            pad(&mut writer);
            let mut frame = FrameHeader::new(Some(&metadata));
            frame.encoding = FrameEncoding::MODULAR;
            frame.color_transform = ColorTransform::NONE;
            frame.loop_filter.gab = false;
            frame.loop_filter.epf_iters = 0;
            write_frame_header(&mut frame, &mut writer, None).unwrap();
            let end = writer.bits_written();
            pad(&mut writer);

            let mut br = BitReader::new(writer.get_span());
            assert_eq!(br.read_bits(16), 0x0aff);
            let mut decoded_metadata = CodecMetadata::default();
            bundle::read(&mut br, &mut decoded_metadata.size).unwrap();
            read_image_metadata(&mut br, &mut decoded_metadata.m).unwrap();
            decoded_metadata.transform_data.nonserialized_xyb_encoded =
                decoded_metadata.m.xyb_encoded;
            bundle::read(&mut br, &mut decoded_metadata.transform_data).unwrap();
            br.jump_to_byte_boundary().unwrap();
            let mut decoded = FrameHeader::new(Some(&decoded_metadata));
            read_frame_header(&mut br, &mut decoded).unwrap();
            assert_eq!(br.total_bits_consumed(), end);
            br.close().unwrap();
            assert_eq!(decoded.encoding, FrameEncoding::MODULAR);
            assert_eq!(decoded.color_transform, ColorTransform::NONE);
            assert_eq!(decoded.frame_type, FrameType::REGULAR);
            assert!(decoded.is_last);
            assert!(!decoded.can_be_referenced());
            assert_eq!(decoded.loop_filter.padding(), 0);
            let d = decoded.to_frame_dimensions();
            assert_eq!((d.xsize, d.ysize), (x, y));
            assert_eq!((d.xsize_padded, d.ysize_padded), (x, y));
        }
    }
}

#[test]
fn custom_reference_frame_with_no_metadata_retains_the_source_optional_borrow_case() {
    // The original dereferences metadata only if size is not custom or the
    // reference is saved after the color transform. Neither applies here.
    let mut frame = FrameHeader::new(None);
    frame.frame_type = FrameType::REFERENCE_ONLY;
    frame.custom_size_or_origin = true;
    frame.frame_size = FrameSize {
        xsize: 19,
        ysize: 11,
    };
    frame.save_before_color_transform = true;
    let mut writer = BitWriter::new();
    write_frame_header(&mut frame, &mut writer, None).unwrap();
    pad(&mut writer);
    let mut br = BitReader::new(writer.get_span());
    let mut decoded = FrameHeader::new(None);
    read_frame_header(&mut br, &mut decoded).unwrap();
    br.close().unwrap();
    assert!(decoded.save_before_color_transform);
    assert!(!decoded.needs_color_transform());
    let d = decoded.to_frame_dimensions();
    assert_eq!((d.xsize, d.ysize), (19, 11));
}
