// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::float::{load_float_row, load_float16};
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::enc_external_image::convert_from_external;
use jxl::jxl::image::Plane;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

#[test]
fn binary16_load_preserves_negative_zero_subnormals_and_signaling_nan_payloads() {
    for (input, expected) in [
        (0x8000, 0x80000000),
        (1, 0x33800000),
        (0x0400, 0x38800000),
        (0x7d01, 0x7fa02000),
        (0xfd01, 0xffa02000),
    ] {
        assert_eq!(load_float16(input).to_bits(), expected);
    }
    assert_eq!(
        load_float_row(
            &[],
            17,
            usize::MAX,
            JxlDataType(7),
            true,
            f32::NAN,
            |_, _| panic!("unsupported types must not call the callback")
        ),
        Err(StatusCode::GenericError)
    );
}

#[test]
fn external_rgb16_plane_uses_channel_offset_padded_stride_and_optional_last_row_padding() {
    let format = JxlPixelFormat {
        num_channels: 3,
        data_type: JxlDataType::UINT16,
        endianness: JxlEndianness::BIG,
        align: 16,
    };
    let mut data = [0x69u8; 32];
    let samples = [0u16, 1001, 12345, 65535];
    for y in 0..2 {
        for x in 0..2 {
            data[y * 16 + x * 6 + 2..y * 16 + x * 6 + 4]
                .copy_from_slice(&samples[y * 2 + x].to_be_bytes());
        }
    }
    let mut plane = Plane::<f32>::create(2, 2).unwrap();
    convert_from_external(&data, 28, 2, 2, 16, format, 1, &mut plane).unwrap();
    let values: Vec<_> = (0..2)
        .flat_map(|y| plane.row(y)[..2].iter().copied())
        .collect();
    assert_eq!(values, samples.map(|v| f32::from(v) * (1.0f32 / 65535.0)));
    convert_from_external(&data, 32, 2, 2, 16, format, 1, &mut plane).unwrap();
    assert_eq!(
        convert_from_external(&data, 27, 2, 2, 16, format, 1, &mut plane),
        Err(StatusCode::GenericError)
    );
    assert_eq!(
        convert_from_external(&data, 33, 2, 2, 16, format, 1, &mut plane),
        Err(StatusCode::GenericError)
    );
    assert_eq!(plane.row(1)[1], 1.0);
}

#[test]
fn unchecked_size_zero_and_custom_integer_depth_preserve_original_unclamped_scaling() {
    let format = JxlPixelFormat {
        num_channels: 1,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    let mut plane = Plane::<f32>::create(2, 1).unwrap();
    convert_from_external(&[15, 255], 0, 2, 1, 4, format, 0, &mut plane).unwrap();
    assert_eq!(&plane.row(0)[..2], &[1.0, 17.0]);
}
