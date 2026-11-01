// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::float::load_float16;
use jxl::jxl::dec_external_image::{convert_float_channels_to_buffer, float_to_f16};
use jxl::jxl::image::Plane;
use jxl::jxl::image_metadata::Orientation;
use jxl::jxl::types::JxlEndianness;

#[test]
fn all_half_words_round_trip_with_native_cast_nan_quieting_and_unchanged_suffix() {
    let input: Vec<_> = (0..=u16::MAX).map(load_float16).collect();
    let mut output = vec![0xdead; input.len() + 7];
    float_to_f16(&input, &mut output, input.len());
    for word in 0..=u16::MAX {
        let expected = if word & 0x7c00 == 0x7c00 && word & 0x3ff != 0 {
            word | 0x200
        } else {
            word
        };
        assert_eq!(output[usize::from(word)], expected, "half word {word:04x}");
    }
    assert_eq!(&output[input.len()..], &[0xdead; 7]);
    float_to_f16(&[], &mut output, 0);
    assert_eq!(&output[input.len()..], &[0xdead; 7]);
}

#[test]
fn float32_export_preserves_signaling_nan_words_and_interleaved_ones() {
    let words = [
        0x7fa12345u32,
        0xffa12345,
        0x80000000,
        1,
        0x7fc98765,
        0x3f800000,
    ];
    let mut plane = Plane::create(3, 2).unwrap();
    for y in 0..2 {
        for x in 0..3 {
            plane.row_mut(y)[x] = f32::from_bits(words[y * 3 + x]);
        }
    }
    for (endian, little) in [(JxlEndianness::LITTLE, true), (JxlEndianness::BIG, false)] {
        let mut output = [0xa7; 80];
        convert_float_channels_to_buffer(
            &[Some(&plane), None],
            32,
            endian,
            32,
            &mut output,
            56,
            Orientation::FLIP_HORIZONTAL,
        )
        .unwrap();
        for y in 0..2 {
            for x in 0..3 {
                let value = words[y * 3 + 2 - x];
                let expected = if little {
                    value.to_le_bytes()
                } else {
                    value.to_be_bytes()
                };
                let one = if little {
                    1.0f32.to_le_bytes()
                } else {
                    1.0f32.to_be_bytes()
                };
                let at = y * 32 + x * 8;
                assert_eq!(&output[at..at + 4], &expected);
                assert_eq!(&output[at + 4..at + 8], &one);
            }
        }
        assert_eq!(&output[24..32], &[0xa7; 8]);
        assert_eq!(&output[56..], &[0xa7; 24]);
    }
}

#[test]
fn half_buffer_guards_precede_writes_and_valid_export_keeps_padding() {
    let mut plane = Plane::create(3, 1).unwrap();
    plane.row_mut(0)[..3].copy_from_slice(&[1.0, -0.0, f32::from_bits(0x7fa12345)]);
    let mut output = [0xa7; 16];
    for (bits, stride, size) in [(15, 8, 16), (16, 5, 16), (16, 8, 5)] {
        assert!(
            convert_float_channels_to_buffer(
                &[Some(&plane)],
                bits,
                JxlEndianness::LITTLE,
                stride,
                &mut output,
                size,
                Orientation::IDENTITY,
            )
            .is_err()
        );
        assert_eq!(output, [0xa7; 16]);
    }
    convert_float_channels_to_buffer(
        &[Some(&plane)],
        16,
        JxlEndianness::LITTLE,
        8,
        &mut output,
        6,
        Orientation::IDENTITY,
    )
    .unwrap();
    assert_eq!(&output[..6], &[0, 0x3c, 0, 0x80, 9, 0x7f]);
    assert_eq!(&output[6..], &[0xa7; 10]);
}
