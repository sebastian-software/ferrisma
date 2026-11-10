// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Adapted from the primitive and dimension-header cases in fields_test.cc.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::field_encodings::{U32Enc, bits, val};
use jxl::jxl::fields::{bundle, f16_coder, u32_coder, u64_coder};
use jxl::jxl::headers::{AnimationHeader, PreviewHeader, SizeHeader};

#[path = "common/bundles.rs"]
mod bundles;

#[test]
fn u32_source_boundaries() {
    let enc = U32Enc::new(val(0), bits(4), val(0x7FFF_FFFF), bits(32));
    for (value, expected_bits) in [
        (0, 2),
        (1, 6),
        (15, 6),
        (0x7FFF_FFFF, 2),
        (128, 34),
        (0x7FFF_FFFE, 34),
        (0x8000_0000, 34),
        (u32::MAX, 34),
    ] {
        let mut encoded_bits = 0;
        u32_coder::can_encode(enc, value, &mut encoded_bits).unwrap();
        assert_eq!(encoded_bits, expected_bits);
        let mut writer = BitWriter::new();
        writer
            .with_max_bits(
                40,
                LayerType::Header,
                None,
                |w, _| {
                    u32_coder::write(enc, value, w)?;
                    assert_eq!(w.bits_written(), expected_bits);
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        let mut reader = BitReader::new(writer.get_span());
        assert_eq!(u32_coder::read(enc, &mut reader), value);
        reader.close().unwrap();
    }
}

#[test]
fn u64_source_boundaries() {
    let mut cases = vec![
        (0, 2),
        (1, 6),
        (2, 6),
        (8, 6),
        (15, 6),
        (16, 6),
        (17, 10),
        (18, 10),
        (100, 10),
        (271, 10),
        (272, 10),
        (273, 15),
        (274, 15),
        (1000, 15),
        (4094, 15),
        (4095, 15),
    ];
    for (power, width) in [
        (12, 24),
        (20, 33),
        (28, 42),
        (36, 51),
        (44, 60),
        (52, 69),
        (60, 73),
    ] {
        cases.push((1u64 << power, width));
        cases.push(((1u64 << power) + 1, width));
        let top = if power == 60 {
            u64::MAX
        } else {
            (1u64 << (power + 8)) - 1
        };
        cases.push((top - 1, width));
        cases.push((top, width));
    }
    for (value, expected_bits) in cases {
        let mut encoded_bits = 0;
        u64_coder::can_encode(value, &mut encoded_bits).unwrap();
        assert_eq!(encoded_bits, expected_bits);
        let mut writer = BitWriter::new();
        writer
            .with_max_bits(
                80,
                LayerType::Header,
                None,
                |w, _| {
                    u64_coder::write(value, w)?;
                    assert_eq!(w.bits_written(), expected_bits);
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        let mut reader = BitReader::new(writer.get_span());
        assert_eq!(u64_coder::read(&mut reader), value);
        reader.close().unwrap();
    }
}

#[test]
fn f16_source_values_and_range() {
    for sign in [-1.0f32, 1.0] {
        for mag in [
            0.0,
            0.5,
            1.0,
            2.0,
            2.5,
            16.015625,
            1.0 / 4096.0,
            1.0 / 16384.0,
            65504.0,
        ] {
            let value = sign * mag;
            let mut encoded = 0;
            f16_coder::can_encode(value, &mut encoded).unwrap();
            assert_eq!(encoded, 16);
            let mut writer = BitWriter::new();
            writer
                .with_max_bits(
                    16,
                    LayerType::Header,
                    None,
                    |w, _| f16_coder::write(value, w),
                    false,
                )
                .unwrap();
            let mut reader = BitReader::new(writer.get_span());
            let mut decoded = 0.0;
            f16_coder::read(&mut reader, &mut decoded).unwrap();
            assert_eq!(decoded, value);
            reader.close().unwrap();
        }
    }
    for value in [
        65504.01,
        -65505.0,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ] {
        assert_eq!(
            f16_coder::can_encode(value, &mut 0),
            Err(StatusCode::GenericError)
        );
    }
}

#[test]
fn roundtrip_size_source_cases() {
    for i in 0..8 {
        let mut size = SizeHeader::default();
        size.set(123 + 77 * i, 7 + i).unwrap();
        let (ext, total) = bundle::can_encode(&mut size).unwrap();
        assert_eq!(ext, 0);
        let mut writer = BitWriter::new();
        bundle::write(&mut size, &mut writer, LayerType::Header, None).unwrap();
        assert_eq!(writer.bits_written(), total);
        writer.zero_pad_to_byte();
        let mut size2 = SizeHeader::default();
        let mut reader = BitReader::new(writer.get_span());
        bundle::read(&mut reader, &mut size2).unwrap();
        assert_eq!(reader.total_bits_consumed(), total);
        reader.close().unwrap();
        assert_eq!((size.xsize(), size.ysize()), (size2.xsize(), size2.ysize()));
    }
}

#[test]
fn preview_source_range() {
    for i in 1..4360 {
        let mut header = PreviewHeader::default();
        header.set(i, i).unwrap();
        let (ext, total) = bundle::can_encode(&mut header).unwrap();
        assert_eq!(ext, 0);
        assert!(total >= 6);
    }
}

#[test]
fn dimensions_set_and_encoding_have_distinct_limits() {
    let mut header = SizeHeader::default();
    header.set(u32::MAX as usize, u32::MAX as usize).unwrap();
    assert_eq!(
        bundle::can_encode(&mut header),
        Err(StatusCode::GenericError)
    );
    assert_eq!(header.set(0, 1), Err(StatusCode::GenericError));
    #[cfg(target_pointer_width = "64")]
    assert_eq!(
        header.set(u32::MAX as usize + 1, 1),
        Err(StatusCode::GenericError)
    );
}

#[test]
fn truncated_header_is_retryable_and_close_is_acknowledged() {
    let mut header = SizeHeader::default();
    let mut reader = BitReader::new(&[]);
    assert_eq!(
        bundle::read(&mut reader, &mut header),
        Err(StatusCode::NotEnoughBytes)
    );
    // ReadVisitor already checked bounds, preserving the source's Close rule.
    assert_eq!(reader.close(), Ok(()));
}

#[test]
fn animation_defaults_are_initialized_by_the_visitor() {
    let mut header = AnimationHeader::default();
    assert_eq!(
        (
            header.tps_numerator,
            header.tps_denominator,
            header.num_loops,
            header.have_timecodes
        ),
        (1, 1, 0, false)
    );
    assert!(bundle::all_default(&mut header));
    header.tps_numerator = 1000;
    assert!(!bundle::all_default(&mut header));
    bundle::set_default(&mut header);
    assert!(bundle::all_default(&mut header));
}

#[test]
fn new_decoder_reads_old_bundle_with_default_extension_fields() {
    let mut old = bundles::OldBundle {
        old_large: 123,
        old_f: 3.75,
        ..Default::default()
    };
    let (ext, bits) = bundle::can_encode(&mut old).unwrap();
    assert_eq!(ext, 0);
    let mut writer = BitWriter::new();
    bundle::write(&mut old, &mut writer, LayerType::Header, None).unwrap();
    writer
        .with_max_bits(
            32,
            LayerType::Header,
            None,
            |w, _| {
                w.write(20, 0xA55A);
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    let mut reader = BitReader::new(writer.get_span());
    let mut new = bundles::NewBundle::default();
    bundle::read(&mut reader, &mut new).unwrap();
    assert_eq!(reader.total_bits_consumed(), bits);
    assert_eq!(reader.read_bits(20), 0xA55A);
    reader.close().unwrap();
    assert_eq!(
        (new.old_small, new.old_f, new.old_large, new.extensions),
        (old.old_small, old.old_f, old.old_large, old.extensions)
    );
    assert_eq!((new.new_small, new.new_f, new.new_large), (2, -2.0, 0));
}

#[test]
fn old_decoder_skips_new_bundle_extension_payload() {
    let mut new = bundles::NewBundle {
        old_large: 123,
        extensions: 3,
        new_f: 999.0,
        new_large: 456,
        ..Default::default()
    };
    let (ext, bits) = bundle::can_encode(&mut new).unwrap();
    assert_ne!(ext, 0);
    let mut writer = BitWriter::new();
    bundle::write(&mut new, &mut writer, LayerType::Header, None).unwrap();
    writer
        .with_max_bits(
            32,
            LayerType::Header,
            None,
            |w, _| {
                w.write(20, 0xA55A);
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    let mut reader = BitReader::new(writer.get_span());
    let mut old = bundles::OldBundle::default();
    bundle::read(&mut reader, &mut old).unwrap();
    assert_eq!(reader.total_bits_consumed(), bits);
    assert_eq!(reader.read_bits(20), 0xA55A);
    reader.close().unwrap();
    assert_eq!(
        (old.old_small, old.old_f, old.old_large, old.extensions),
        (new.old_small, new.old_f, new.old_large, new.extensions)
    );
}

#[test]
fn all_default_branch_recursively_resets_nested_fields() {
    let mut fields = bundles::DefaultedBundle::default();
    assert_eq!(fields.value, 7);
    let (ext, bits) = bundle::can_encode(&mut fields).unwrap();
    assert_eq!((ext, bits), (0, 1));
    let mut writer = BitWriter::new();
    bundle::write(&mut fields, &mut writer, LayerType::Header, None).unwrap();
    writer.zero_pad_to_byte();
    let mut decoded = bundles::DefaultedBundle {
        value: 5,
        ..Default::default()
    };
    decoded.size.set(1920, 1080).unwrap();
    let mut reader = BitReader::new(writer.get_span());
    bundle::read(&mut reader, &mut decoded).unwrap();
    assert_eq!(reader.total_bits_consumed(), 1);
    reader.close().unwrap();
    assert_eq!(
        (decoded.value, decoded.size.xsize(), decoded.size.ysize()),
        (7, 1, 1)
    );
}
