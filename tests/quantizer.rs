// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::ac_strategy::AcStrategyType;
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::{AuxOut, LayerType};
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_fields::write_quantizer_params;
use jxl::jxl::fields::bundle;
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::{DEFAULT_QUANT_BIAS, Quantizer, QuantizerParams};

fn snapshot(q: &Quantizer) -> Vec<u32> {
    let mut words = vec![
        q.global_scale() as u32,
        q.raw_dc() as u32,
        q.scale().to_bits(),
        q.inv_global_scale().to_bits(),
        q.inv_quant_dc().to_bits(),
    ];
    words.extend(
        q.mul_dc()
            .iter()
            .chain(q.inv_mul_dc())
            .chain(q.zero_bias())
            .map(|v| v.to_bits()),
    );
    words
}
fn write(global_scale: u32, quant_dc: u32, prefix: usize) -> (BitWriter, AuxOut) {
    let mut writer = BitWriter::new();
    writer
        .with_max_bits(
            prefix,
            LayerType::Header,
            None,
            |w, _| {
                if prefix != 0 {
                    w.write(prefix, (1 << prefix) - 1);
                }
                Ok(())
            },
            false,
        )
        .unwrap();
    let mut aux = AuxOut::default();
    write_quantizer_params(
        &mut QuantizerParams {
            global_scale,
            quant_dc,
        },
        &mut writer,
        LayerType::Quant,
        Some(&mut aux),
    )
    .unwrap();
    (writer, aux)
}
fn bytes(writer: &mut BitWriter) -> Vec<u8> {
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
    writer.get_span().to_vec()
}

#[test]
fn quantizer_parameter_defaults_and_original_serialized_known_answers() {
    let defaults = QuantizerParams::default();
    assert_eq!((defaults.global_scale, defaults.quant_dc), (1, 1));
    assert_eq!(
        DEFAULT_QUANT_BIAS.map(f32::to_bits),
        [1064436340, 1064177896, 1064515444, 1041529569]
    );
    // Captured independently from the pristine C++ WriteQuantizerParams.
    for (global, dc, prefix, bits, expected) in [
        (1, 1, 0, 20, vec![0x00, 0x20, 0x00]),
        (1024, 64, 0, 23, vec![0xfc, 0xcf, 0x1f]),
        (73728, 65536, 0, 36, vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
        (4097, 16, 3, 19, vec![0x17, 0x00, 0x00]),
    ] {
        let (mut writer, aux) = write(global, dc, prefix);
        assert_eq!(writer.bits_written(), bits);
        assert_eq!(aux.layer(LayerType::Quant).total_bits, bits - prefix);
        assert_eq!(bytes(&mut writer), expected);
        let mut reader = BitReader::new(&expected);
        reader.skip_bits(prefix);
        let mut parsed = QuantizerParams::default();
        bundle::read(&mut reader, &mut parsed).unwrap();
        assert_eq!((parsed.global_scale, parsed.quant_dc), (global, dc));
        assert_eq!(reader.total_bits_consumed(), bits);
        reader.close().unwrap();
    }
}

#[test]
fn quantizer_parameter_writer_rejects_field_limits_before_writing_or_accounting() {
    for (global_scale, quant_dc) in [(0, 1), (73729, 1), (1, 0), (1, 65537), (u32::MAX, u32::MAX)] {
        let mut writer = BitWriter::new();
        let mut aux = AuxOut::default();
        let mut p = QuantizerParams {
            global_scale,
            quant_dc,
        };
        assert_eq!(
            write_quantizer_params(&mut p, &mut writer, LayerType::Quant, Some(&mut aux)),
            Err(StatusCode::GenericError)
        );
        assert_eq!(writer.bits_written(), 0);
        assert_eq!(aux.layer(LayerType::Quant).total_bits, 0);
    }
}

#[test]
fn quantizer_decode_retains_all_owner_fields_on_failure_but_bundle_retains_partial_fields() {
    let matrices = DequantMatrices::default();
    let mut q = Quantizer::with_values(&matrices, 123, 54321);
    q.clear_dc_mul();
    let before = snapshot(&q);
    for bytes in [
        &[][..],
        &[0xff][..],
        &[0xff, 0xff][..],
        &[0xff, 0xff, 0xff][..],
    ] {
        let mut reader = BitReader::new(bytes);
        assert_eq!(
            q.decode(&matrices, &mut reader),
            Err(StatusCode::NotEnoughBytes)
        );
        assert_eq!(snapshot(&q), before);
        assert!(reader.total_bits_consumed() > bytes.len() * 8);
        // The failed bounds check was already accounted for by Bundle::Read.
        reader.close().unwrap();
    }
    let mut p = QuantizerParams {
        global_scale: 54321,
        quant_dc: 123,
    };
    let mut reader = BitReader::new(&[]);
    assert_eq!(
        bundle::read(&mut reader, &mut p),
        Err(StatusCode::NotEnoughBytes)
    );
    assert_eq!((p.global_scale, p.quant_dc), (1, 123));
    assert_eq!(reader.total_bits_consumed(), 13);
    reader.close().unwrap();
}

#[test]
fn quantizer_decode_roundtrips_update_derived_fields_and_keep_cleared_fourth_lanes() {
    let mut matrices = DequantMatrices::default();
    let mut q = Quantizer::with_values(&matrices, 123, 54321);
    q.clear_dc_mul();
    matrices.set_dc_quant(&[2.0, 4.0, 8.0]);
    let (mut writer, _) = write(1024, 64, 5);
    let payload = bytes(&mut writer);
    let mut reader = BitReader::new(&payload);
    reader.skip_bits(5);
    q.decode(&matrices, &mut reader).unwrap();
    assert_eq!(
        (q.get_params().global_scale, q.get_params().quant_dc),
        (1024, 64)
    );
    assert_eq!(
        q.mul_dc().map(f32::to_bits),
        [
            0.5f32.to_bits(),
            0.25f32.to_bits(),
            0.125f32.to_bits(),
            1.0f32.to_bits()
        ]
    );
    assert_eq!(
        q.inv_mul_dc().map(f32::to_bits),
        [
            2.0f32.to_bits(),
            4.0f32.to_bits(),
            8.0f32.to_bits(),
            1.0f32.to_bits()
        ]
    );
    assert_eq!(reader.total_bits_consumed(), 28);
    reader.close().unwrap();
}

#[test]
fn quantizer_clamp_preserves_literal_source_min_max_and_scaling_rounds_away_from_zero() {
    for word in [0x7fc12345, 0xffc23456] {
        assert_eq!(Quantizer::clamp_val(f32::from_bits(word)), 1);
    }
    for (input, expected) in [
        (f32::NEG_INFINITY, 1),
        (f32::INFINITY, 256),
        (-0.0, 1),
        (1.999, 1),
        (255.999, 255),
        (256.0, 256),
    ] {
        assert_eq!(Quantizer::clamp_val(input), expected);
    }
    let matrices = DequantMatrices::default();
    for global in [-3, -1, 1, 3] {
        let mut q = Quantizer::with_values(&matrices, 64, global);
        q.clear_dc_mul();
        let scale = q.scale_global_scale(&matrices, 0.5);
        let expected = if global < 0 {
            (global - 1) / 2
        } else {
            (global + 1) / 2
        };
        assert_eq!(q.global_scale(), expected);
        assert_eq!(scale.to_bits(), (expected as f32 / global as f32).to_bits());
        assert_eq!(q.mul_dc()[3].to_bits(), 1.0f32.to_bits());
        assert_eq!(q.inv_mul_dc()[3].to_bits(), 1.0f32.to_bits());
    }
}

#[test]
fn quantizer_matrix_accessors_return_the_original_matrix_owner_slices() {
    let mut matrices = DequantMatrices::default();
    let quantizer = Quantizer::new(&matrices);
    matrices
        .ensure_computed(1 << AcStrategyType::DCT16X8.0)
        .unwrap();
    for c in 0..3 {
        assert!(std::ptr::eq(
            quantizer.dequant_matrix(&matrices, AcStrategyType::DCT16X8, c),
            matrices.matrix(AcStrategyType::DCT16X8, c)
        ));
        assert!(std::ptr::eq(
            quantizer.inv_dequant_matrix(&matrices, AcStrategyType::DCT16X8, c),
            matrices.inv_matrix(AcStrategyType::DCT16X8, c)
        ));
    }
}

#[cfg(all(target_pointer_width = "64", not(target_os = "windows")))]
#[test]
fn quantizer_scaling_keeps_original_c_long_rounding_before_int_narrowing() {
    assert_eq!(std::ffi::c_long::BITS, 64);
    let matrices = DequantMatrices::default();
    // Independently captured from the ordinary and fully instrumented pristine
    // original on the supported 64-bit C-long targets. No Rust output supplies
    // the global-scale, returned ratio, or derived-state expected words.
    for (global, scale_word, expected_global, ratio, derived) in [
        (
            2147483647,
            0x3f800000,
            -2147483648,
            3212836864,
            [3338665984, 3087007744, 3036676096],
        ),
        (
            -2147483648,
            0xbf800000,
            -2147483648,
            1065353216,
            [3338665984, 3087007744, 3036676096],
        ),
        (
            -2147483648,
            0x40000000,
            0,
            2147483648,
            [0, 2139095040, 2139095040],
        ),
        (
            1,
            0x4f7fffff,
            -256,
            3279945728,
            [3145728000, 3279945728, 3229614080],
        ),
        (
            1,
            0x4f800001,
            512,
            1140850688,
            [1006632960, 1124073472, 1073741824],
        ),
        (
            1,
            0xcf000001,
            2147483392,
            1325400062,
            [1191182334, 939524097, 889192449],
        ),
        (
            3,
            0x4e800000,
            -1073741824,
            3450514091,
            [3330277376, 3095396352, 3045064704],
        ),
        (1, 0x5effffff, 0, 0, [0, 2139095040, 2139095040]),
        (1, 0xdf000000, 0, 0, [0, 2139095040, 2139095040]),
    ] {
        let mut q = Quantizer::with_values(&matrices, 64, global);
        q.clear_dc_mul();
        let result = q.scale_global_scale(&matrices, f32::from_bits(scale_word));
        assert_eq!(q.global_scale(), expected_global);
        assert_eq!(result.to_bits(), ratio);
        assert_eq!(
            [
                q.scale().to_bits(),
                q.inv_global_scale().to_bits(),
                q.inv_quant_dc().to_bits()
            ],
            derived
        );
        assert_eq!(q.mul_dc()[3].to_bits(), 1.0f32.to_bits());
        assert_eq!(q.inv_mul_dc()[3].to_bits(), 1.0f32.to_bits());
    }
}
