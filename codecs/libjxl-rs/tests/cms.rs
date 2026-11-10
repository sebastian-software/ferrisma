// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Native profile-layout checks and independent published MD5 known answers.

use jxl::jxl::cms::icc::{icc_compute_md5, maybe_create_profile};
use jxl::jxl::color_encoding::*;

#[test]
fn icc_md5_matches_known_answers_at_padding_boundaries() {
    for (data, expected) in [
        ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ("a", "0cc175b9c0f1b6a831c399e269772661"),
        ("abc", "900150983cd24fb0d6963f7d28e17f72"),
        ("message digest", "f96b697d7cb7938d525a2f31aaf161d0"),
        (
            "abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b",
        ),
        (
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "d174ab98d277d9f5a5611c2c9f419d9f",
        ),
        (
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
            "57edf4a22be3c955ac49da2e2107b67a",
        ),
    ] {
        let actual = icc_compute_md5(data.as_bytes())
            .iter()
            .map(|v| format!("{v:02x}"))
            .collect::<String>();
        assert_eq!(actual, expected);
    }
}

#[test]
fn generated_profiles_have_bounded_aligned_tags_and_correct_profile_ids() {
    for space in [ColorSpace::RGB, ColorSpace::GRAY, ColorSpace::XYB] {
        for tf in [
            TransferFunction::SRGB,
            TransferFunction::PQ,
            TransferFunction::HLG,
        ] {
            let c = JxlColorEncoding {
                color_space: space,
                white_point: WhitePoint::D65,
                white_point_xy: [0.3127, 0.3290],
                primaries: Primaries::SRGB,
                primaries_red_xy: [0.639998686, 0.330010138],
                primaries_green_xy: [0.300003784, 0.600003357],
                primaries_blue_xy: [0.150002046, 0.059997204],
                transfer_function: tf,
                gamma: 0.0,
                rendering_intent: if space == ColorSpace::XYB {
                    RenderingIntent::PERCEPTUAL
                } else {
                    RenderingIntent::RELATIVE
                },
            };
            let mut profile = Vec::new();
            maybe_create_profile(&c, &mut profile).unwrap();
            let read = |offset| {
                u32::from_be_bytes(profile[offset..offset + 4].try_into().unwrap()) as usize
            };
            assert_eq!(read(0), profile.len());
            assert_eq!(&profile[36..40], b"acsp");
            let tags = read(128);
            let data_start = 132 + tags * 12;
            for tag in 0..tags {
                let offset = read(132 + tag * 12 + 4);
                let size = read(132 + tag * 12 + 8);
                assert!(offset >= data_start);
                assert_eq!(offset % 4, 0);
                assert!(offset + size <= profile.len());
            }
            let id = profile[84..100].to_vec();
            profile[44..48].fill(0);
            profile[64..68].fill(0);
            profile[84..100].fill(0);
            assert_eq!(id, icc_compute_md5(&profile));
        }
    }
}
