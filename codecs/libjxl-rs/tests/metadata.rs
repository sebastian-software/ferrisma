// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Coordinate/gamma scenarios adapt color_encoding_internal_test.cc. Header
// integration and reused-state regressions are local test infrastructure.

use jxl::jxl::cms::color_encoding_cms::{CieXy, ColorEncoding as ColorStorage, PrimariesCieXy};
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_fields::write_codestream_headers;
use jxl::jxl::fields::bundle;
use jxl::jxl::image_metadata::{
    CodecMetadata, ExtraChannel, ExtraChannelInfo, ImageMetadata, read_image_metadata,
};

#[test]
fn custom_coordinates_and_gamma_keep_source_round_trip_and_failure_behavior() {
    let mut source = ColorStorage::default();
    assert!(source.set_white_point(CieXy { x: 0.8, y: 0.01 }).is_ok());
    assert!(
        source
            .set_primaries(PrimariesCieXy {
                r: CieXy { x: -0.01, y: 0.2 },
                g: CieXy { x: 0.4, y: 0.401 },
                b: CieXy { x: 1.1, y: -1.2 }
            })
            .is_ok()
    );
    for value in [0.0, -1e-6, 1.001] {
        assert!(source.tf.set_gamma(value).is_err());
    }
    source.tf.set_gamma(1.0).unwrap();
    assert!(source.tf.is_linear());
    source.tf.set_gamma(0.123).unwrap();
    assert!(source.tf.have_gamma);
    let mut destination = ColorStorage::default();
    destination.from_external(&source.to_external()).unwrap();
    assert!(source.same_color_encoding(&destination));
    // A failure after the x conversion must retain the newly assigned x and
    // the previous y, rather than rolling the whole setter back.
    let before = source.white.y;
    assert!(source.white.set_value(CieXy { x: 3.0, y: 0.1 }).is_err());
    assert_eq!(source.white.x, 3_000_000);
    assert_eq!(source.white.y, before);
}

#[test]
fn complete_rgb_and_gray_codestream_headers_round_trip_in_rust() {
    for gray in [false, true] {
        for bits in [8, 12, 16] {
            let mut metadata = CodecMetadata::default();
            metadata.size.set(257, 129).unwrap();
            metadata.m.xyb_encoded = false;
            metadata.m.set_uint_samples(bits);
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
            let end = writer.bits_written();
            writer
                .with_max_bits(
                    7,
                    jxl::jxl::enc_aux_out::LayerType::Header,
                    None,
                    |w, _| {
                        w.zero_pad_to_byte();
                        Ok(())
                    },
                    false,
                )
                .unwrap();
            let mut br = BitReader::new(writer.get_span());
            assert_eq!(br.read_bits(8), 255);
            assert_eq!(br.read_bits(8), 10);
            let mut decoded = CodecMetadata::default();
            bundle::read(&mut br, &mut decoded.size).unwrap();
            read_image_metadata(&mut br, &mut decoded.m).unwrap();
            decoded.transform_data.nonserialized_xyb_encoded = decoded.m.xyb_encoded;
            bundle::read(&mut br, &mut decoded.transform_data).unwrap();
            assert_eq!((decoded.xsize(), decoded.ysize()), (257, 129));
            assert_eq!(decoded.m.bit_depth.bits_per_sample, bits);
            assert_eq!(decoded.m.color_encoding.is_gray(), gray);
            assert!(!decoded.m.xyb_encoded);
            assert_eq!(br.total_bits_consumed(), end);
            assert!(br.close().is_ok());
        }
    }
}

#[test]
fn default_image_read_keeps_existing_extra_channel_names_like_the_source() {
    let info = ExtraChannelInfo {
        channel_type: ExtraChannel::DEPTH,
        name: vec![0xff, 0, b'x'],
        ..ExtraChannelInfo::default()
    };
    let mut metadata = ImageMetadata {
        extra_channel_info: vec![info.clone(), info],
        orientation: 8,
        ..ImageMetadata::default()
    };
    let bytes = [1u8];
    let mut br = BitReader::new(&bytes);
    read_image_metadata(&mut br, &mut metadata).unwrap();
    assert_eq!(metadata.num_extra_channels, 0);
    assert_eq!(metadata.extra_channel_info.len(), 2);
    assert_eq!(metadata.orientation, 1);
    for info in &metadata.extra_channel_info {
        assert_eq!(info.name, [0xff, 0, b'x']);
        assert_eq!(info.channel_type, ExtraChannel::ALPHA);
    }
    assert!(br.close().is_ok());
}
