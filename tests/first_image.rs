// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Ordinary Rust builds execute this complete path without the C++ feature.
use jxl::jxl::dec_frame::decode_lossless_thunder;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
#[test]
fn pixel_buffers_round_trip_complete_files_with_parsed_metadata_and_integer_output() {
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
