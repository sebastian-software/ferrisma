// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Ordinary Rust builds execute this complete path without the C++ feature.
use jxl::jxl::dec_frame::decode_lossless_thunder;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
#[test]
fn pixel_buffers_round_trip_complete_files_with_parsed_metadata_and_integer_output() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for (w, h) in [(1, 1), (17, 9), (513, 3), (2049, 2)] {
        for gray in [false, true] {
            for depth in [8, 16] {
                let channels = if gray { 1 } else { 3 };
                let mut seed = 0x11224488abcdefu64;
                let input: Vec<_> = (0..w * h * channels)
                    .flat_map(|_| {
                        seed ^= seed << 13;
                        seed ^= seed >> 7;
                        seed ^= seed << 17;
                        let value = seed as u16;
                        if depth == 8 {
                            vec![value as u8]
                        } else {
                            value.to_ne_bytes().to_vec()
                        }
                    })
                    .collect();
                let mut config = LosslessThunderConfig::new(
                    w,
                    h,
                    gray,
                    if depth == 8 {
                        LosslessSampleDepth::Uint8
                    } else {
                        LosslessSampleDepth::Uint16
                    },
                )
                .unwrap();
                let encoded = config.encode_pixels(&input, None).unwrap();
                let decoded = decode_lossless_thunder(encoded.as_slice());
                decoded.status.unwrap();
                assert_eq!(decoded.pixels, input);
                assert_eq!((decoded.metadata.xsize(), decoded.metadata.ysize()), (w, h));
                assert_eq!(decoded.metadata.m.color_encoding.channels(), channels);
                assert_eq!(decoded.metadata.m.bit_depth.bits_per_sample, depth);
                assert_eq!(decoded.reader_bits, encoded.len() * 8);
                assert_eq!(decoded.frame_size, (w, h));
                assert!(
                    decoded.frame_decoded_bytes > 0 && decoded.frame_decoded_bytes < encoded.len()
                );
            }
        }
    }
}
#[test]
fn complete_uint16_image_recovers_every_integer_sample() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let input: Vec<_> = (0..=u16::MAX).flat_map(u16::to_ne_bytes).collect();
    let mut config =
        LosslessThunderConfig::new(256, 256, true, LosslessSampleDepth::Uint16).unwrap();
    let encoded = config.encode_pixels(&input, None).unwrap();
    let decoded = decode_lossless_thunder(encoded.as_slice());
    decoded.status.unwrap();
    assert_eq!(decoded.pixels, input);
    assert_eq!(decoded.metadata.m.bit_depth.bits_per_sample, 16);
    assert_eq!(decoded.reader_bits, encoded.len() * 8);
}

#[test]
fn lossless_alpha_files_preserve_all_channels_including_hidden_colors() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for (w, h) in [
        (1, 1),
        (17, 9),
        (257, 3),
        (513, 3),
        (2049, 2),
        (3, 257),
        (3, 2049),
        (257, 257),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                let channels = if gray { 2 } else { 4 };
                let input: Vec<u8> = (0..w * h * channels)
                    .flat_map(|i| {
                        let value = if i % channels == channels - 1 {
                            [0, u16::MAX, 1, 32768, 129][(i / channels) % 5]
                        } else {
                            (i * 173 + i * i * 29 + 61) as u16
                        };
                        if depth == 8 {
                            vec![value as u8]
                        } else {
                            value.to_ne_bytes().to_vec()
                        }
                    })
                    .collect();
                let mut config = LosslessThunderConfig::new_with_alpha(
                    w,
                    h,
                    gray,
                    if depth == 8 {
                        LosslessSampleDepth::Uint8
                    } else {
                        LosslessSampleDepth::Uint16
                    },
                )
                .unwrap();
                let encoded = config.encode_pixels(&input, None).unwrap();
                let decoded = decode_lossless_thunder(encoded.as_slice());
                decoded.status.unwrap();
                assert_eq!(decoded.pixels, input, "{w}x{h} gray={gray} depth={depth}");
                assert_eq!(decoded.metadata.m.get_alpha_bits(), depth);
                assert_eq!(decoded.metadata.m.num_extra_channels, 1);
                assert!(!decoded.metadata.m.extra_channel_info[0].alpha_associated);
                assert_eq!(decoded.frame_size, (w, h));
                assert_eq!(decoded.reader_bits, encoded.len() * 8);
            }
        }
    }
}

#[test]
fn complete_uint16_alpha_image_recovers_every_alpha_sample_and_hidden_rgb() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let input: Vec<u8> = (0..=u16::MAX)
        .flat_map(|alpha| {
            [
                alpha.wrapping_add(1),
                alpha ^ 0x55aa,
                u16::MAX - alpha,
                alpha,
            ]
            .into_iter()
            .flat_map(u16::to_ne_bytes)
        })
        .collect();
    let mut config =
        LosslessThunderConfig::new_with_alpha(256, 256, false, LosslessSampleDepth::Uint16)
            .unwrap();
    let encoded = config.encode_pixels(&input, None).unwrap();
    let decoded = decode_lossless_thunder(encoded.as_slice());
    decoded.status.unwrap();
    assert_eq!(decoded.pixels, input);
    assert_eq!(decoded.metadata.m.get_alpha_bits(), 16);
}
#[test]
fn alpha_profile_rejects_untranslated_metadata_before_frame_decoding() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    use jxl::jxl::base::status::StatusCode;
    use jxl::jxl::enc_bit_writer::BitWriter;
    use jxl::jxl::enc_fields::write_codestream_headers;
    use jxl::jxl::image_metadata::CodecMetadata;
    for mode in 0..5 {
        let mut metadata = CodecMetadata::default();
        metadata.size.set(1, 1).unwrap();
        metadata.m.set_uint_samples(8);
        metadata.m.xyb_encoded = false;
        metadata
            .m
            .color_encoding
            .set_srgb(
                jxl::jxl::color_encoding::ColorSpace::RGB,
                jxl::jxl::color_encoding::RenderingIntent::RELATIVE,
            )
            .unwrap();
        metadata.m.set_alpha_bits(8, false);
        match mode {
            0 => metadata.m.extra_channel_info[0].alpha_associated = true,
            1 => metadata.m.extra_channel_info[0].dim_shift = 1,
            2 => metadata.m.extra_channel_info[0].bit_depth.bits_per_sample = 16,
            3 => {
                metadata.m.extra_channel_info[0].channel_type =
                    jxl::jxl::image_metadata::ExtraChannel::DEPTH
            }
            _ => {
                metadata
                    .m
                    .extra_channel_info
                    .push(metadata.m.extra_channel_info[0].clone());
                metadata.m.num_extra_channels = 2;
            }
        }
        let mut writer = BitWriter::new();
        write_codestream_headers(&mut metadata, &mut writer, None).unwrap();
        writer
            .with_max_bits(
                8,
                jxl::jxl::enc_aux_out::LayerType::Header,
                None,
                |w, _| {
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        let decoded = decode_lossless_thunder(writer.take_bytes().as_slice());
        assert_eq!(decoded.status, Err(StatusCode::Unsupported));
        assert_eq!(
            decoded.metadata.m.num_extra_channels,
            metadata.m.num_extra_channels
        );
        assert!(decoded.pixels.is_empty());
        assert_eq!(decoded.frame_size, (0, 0));
    }
}
