// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Native integration of the translated standalone modular encoder/decoder.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_ans_params::LZ77Method;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::modular::encoding::enc_encoding::modular_generic_compress;
use jxl::jxl::modular::encoding::encoding::{GroupHeader, modular_generic_decompress};
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::{ModularOptions, Predictor, TreeKind, UNDEFINED_PREDICTOR};
use jxl::jxl::modular::transform::enc_rct::fwd_rct;
use jxl::jxl::modular::transform::enc_squeeze::fwd_squeeze;
use jxl::jxl::modular::transform::squeeze::default_squeeze_parameters;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};

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
fn generic_gray_encoder_reproduces_the_original_fixed_decoder_fixture() {
    let mut image = Image::create(3, 2, 8, 1).unwrap();
    image.channel[0].row_mut(0)[..3].copy_from_slice(&[10, 20, 30]);
    image.channel[0].row_mut(1)[..3].copy_from_slice(&[15, 25, 35]);
    let mut options = ModularOptions {
        tree_kind: TreeKind::GradientFixedDC,
        predictor: Predictor::GRADIENT,
        ..ModularOptions::default()
    };
    options.histogram_params.lz77_method = LZ77Method::None;
    let mut writer = BitWriter::new();
    modular_generic_compress(
        &image,
        &options,
        &mut writer,
        None,
        LayerType::ModularGlobal,
        0,
    )
    .unwrap();
    assert_eq!(writer.bits_written(), 74);
    pad(&mut writer);
    // Same pinned C++ fixture as tests/modular_decode.rs, independently of
    // whether the translated decoder accepts the encoder's output.
    assert_eq!(
        writer.get_span(),
        &[0x22, 0x61, 0x2c, 0x36, 0x48, 0x32, 0x51, 0x29, 0x7a, 0x00]
    );
}

#[test]
fn learned_rct_squeeze_streams_restore_odd_sized_gray_and_rgb_rasters() {
    for (width, height, channels) in [(17, 9, 1), (65, 33, 3)] {
        for predictor in [UNDEFINED_PREDICTOR, Predictor::VARIABLE] {
            let mut image = Image::create(width, height, 16, channels).unwrap();
            for (c, channel) in image.channel.iter_mut().enumerate() {
                for y in 0..height {
                    for x in 0..width {
                        channel.row_mut(y)[x] =
                            ((x * 1237 + y * 31 + c * 71 + x * y * 53) & 65535) as i32;
                    }
                }
            }
            let original = Image::clone_image(&image).unwrap();
            if channels == 3 {
                fwd_rct(&mut image, 0, 6).unwrap();
                image.transform.push(Transform {
                    rct_type: 6,
                    ..Transform::new(TransformId::RCT)
                });
            }
            let mut squeeze = Transform::new(TransformId::SQUEEZE);
            default_squeeze_parameters(&mut squeeze.squeezes, &image);
            fwd_squeeze(&mut image, squeeze.squeezes.clone()).unwrap();
            image.transform.push(squeeze);
            let mut options = ModularOptions {
                predictor,
                nb_repeats: 0.25,
                ..ModularOptions::default()
            };
            if predictor == Predictor::VARIABLE {
                options.max_properties = 1;
                options
                    .splitting_heuristics_properties
                    .extend([16, 17, 18, 19]);
            }
            let mut writer = BitWriter::new();
            modular_generic_compress(
                &image,
                &options,
                &mut writer,
                None,
                LayerType::ModularGlobal,
                0,
            )
            .unwrap();
            // The source normalizes an undefined predictor in a local copy.
            assert_eq!(options.predictor, predictor);
            let bits = writer.bits_written();
            pad(&mut writer);
            let mut decoded = Image::create(width, height, 16, channels).unwrap();
            let mut header = GroupHeader::default();
            let mut br = BitReader::new(writer.get_span());
            modular_generic_decompress(
                &mut br,
                &mut decoded,
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
            assert_eq!(br.total_bits_consumed(), bits);
            br.close().unwrap();
            assert_eq!(header.transforms.len(), if channels == 3 { 2 } else { 1 });
            assert!(decoded.transform.is_empty());
            assert_eq!(decoded.nb_meta_channels, 0);
            assert!(!decoded.error);
            assert_eq!(decoded.channel.len(), channels as usize);
            for (actual, expected) in decoded.channel.iter().zip(&original.channel) {
                assert_eq!((actual.w, actual.h), (width, height));
                for y in 0..height {
                    assert_eq!(&actual.row(y)[..width], &expected.row(y)[..width]);
                }
            }
        }
    }
}

#[test]
fn failed_learning_retains_the_written_header_and_leaves_caller_options_intact() {
    let image = Image::create(1, 1, 8, 1).unwrap();
    let options = ModularOptions {
        splitting_heuristics_properties: Vec::new(),
        ..ModularOptions::default()
    };
    let mut writer = BitWriter::new();
    assert_eq!(
        modular_generic_compress(
            &image,
            &options,
            &mut writer,
            None,
            LayerType::ModularGlobal,
            0
        ),
        Err(StatusCode::GenericError)
    );
    assert_eq!(writer.bits_written(), 4);
    pad(&mut writer);
    assert_eq!(writer.get_span(), &[2]);
    assert_eq!(options.predictor, UNDEFINED_PREDICTOR);
    assert!(options.splitting_heuristics_properties.is_empty());
}
