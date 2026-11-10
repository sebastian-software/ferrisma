// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Standalone modular fixtures captured from the pinned scalar C++ encoder.
// They contain actual source image streams, not JPEG XL codestream framing.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::fields::bundle;
use jxl::jxl::modular::encoding::encoding::{
    GroupHeader, modular_decode, modular_generic_decompress,
};
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::ModularOptions;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};

fn bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}
fn pad(writer: &mut BitWriter) {
    writer
        .with_max_bits(
            8,
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
fn original_standalone_gray_and_palette_squeeze_streams_restore_samples() {
    // md-source 6 5 0 0 0 0 0 0: GradientFixedDC, Gradient, no transforms.
    let gray = bytes("22612c36483251297a00");
    let mut image = Image::create(3, 2, 8, 1).unwrap();
    let mut br = BitReader::new(&gray);
    modular_generic_decompress(
        &mut br,
        &mut image,
        None,
        0,
        &mut ModularOptions::default(),
        true,
        None,
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(br.total_bits_consumed(), 74);
    br.close().unwrap();
    assert_eq!(&image.channel[0].row(0)[..3], &[10, 20, 30]);
    assert_eq!(&image.channel[0].row(1)[..3], &[15, 25, 35]);
    // md-source 6 6 2 0 1 5 0 0: source lossless palette followed by squeeze,
    // GradientFixedDC, Weighted preset 2, and RLE entropy selection.
    let rgb = bytes("a8740080a9b959088400603041c2586c10ca8c6dd6c4460f8c1e18818240412038d038d000");
    let mut image = Image::create(9, 5, 8, 3).unwrap();
    let mut header = GroupHeader::default();
    let mut br = BitReader::new(&rgb);
    modular_generic_decompress(
        &mut br,
        &mut image,
        Some(&mut header),
        0,
        &mut ModularOptions::default(),
        true,
        None,
        None,
        None,
        false,
    )
    .unwrap();
    assert_eq!(br.total_bits_consumed(), 291);
    br.close().unwrap();
    assert_eq!(image.channel.len(), 3);
    assert_eq!(image.nb_meta_channels, 0);
    assert!(image.transform.is_empty());
    assert!(!image.error);
    assert_eq!(header.transforms.len(), 2);
    for c in 0..3 {
        for y in 0..5 {
            for x in 0..9 {
                assert_eq!(
                    image.channel[c].row(y)[x],
                    (c * 32 + ((x + y) % 2) * 10) as i32
                );
            }
        }
    }
}
#[test]
fn missing_global_tables_zero_the_current_tail_only_when_the_guard_is_armed() {
    let mut encoded = GroupHeader {
        use_global_tree: true,
        ..GroupHeader::default()
    };
    let mut writer = BitWriter::new();
    bundle::write(&mut encoded, &mut writer, LayerType::Header, None).unwrap();
    let bits = writer.bits_written();
    pad(&mut writer);
    for allow in [false, true] {
        let mut image = Image::create(2, 2, 8, 2).unwrap();
        for c in &mut image.channel {
            for y in 0..2 {
                c.row_mut(y)[..2].fill(37);
            }
        }
        let mut br = BitReader::new(writer.get_span());
        let mut header = GroupHeader::default();
        assert_eq!(
            modular_decode(
                &mut br,
                &mut image,
                &mut header,
                0,
                &mut ModularOptions::default(),
                None,
                None,
                None,
                allow
            ),
            Err(StatusCode::GenericError)
        );
        assert_eq!(br.total_bits_consumed(), bits);
        br.close().unwrap();
        assert!(!image.error);
        for c in &image.channel {
            for y in 0..2 {
                assert_eq!(&c.row(y)[..2], if allow { &[37, 37] } else { &[0, 0] });
            }
        }
    }
}
#[test]
fn incomplete_header_preserves_strict_state_but_clears_and_zeroes_allowed_truncation() {
    for allow in [false, true] {
        let mut image = Image::create(2, 1, 8, 3).unwrap();
        for c in &mut image.channel {
            c.row_mut(0)[..2].fill(17);
        }
        let t = Transform {
            rct_type: 0,
            ..Transform::new(TransformId::RCT)
        };
        image.transform.push(t.clone());
        let mut header = GroupHeader::default();
        header.transforms.push(t);
        let mut br = BitReader::new(&[]);
        assert_eq!(
            modular_generic_decompress(
                &mut br,
                &mut image,
                Some(&mut header),
                0,
                &mut ModularOptions::default(),
                true,
                None,
                None,
                None,
                allow
            ),
            Err(StatusCode::NotEnoughBytes)
        );
        // Bundle::Read already acknowledged the overread at its explicit
        // bounds checkpoint; Close succeeds in the original implementation.
        assert_eq!(br.close(), Ok(()));
        assert!(!image.error);
        assert_eq!(image.transform.len(), usize::from(!allow));
        assert_eq!(header.transforms.len(), usize::from(!allow));
        for c in &image.channel {
            assert_eq!(&c.row(0)[..2], if allow { &[0, 0] } else { &[17, 17] });
        }
    }
}
