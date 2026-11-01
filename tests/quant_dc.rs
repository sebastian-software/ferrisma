// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_fields::f16_coder;
use jxl::jxl::enc_quant_weights::dequant_matrices_encode_dc;
use jxl::jxl::quant_weights::{DC_QUANT, DequantMatrices};

#[test]
fn dc_setter_preserves_signaling_nan_bits_in_its_unmodified_inverse_copy() {
    let bits = std::hint::black_box(0x7fa2_3456u32);
    let mut matrices = DequantMatrices::default();
    matrices.set_dc_quant(&[f32::from_bits(bits), 512.0, 256.0]);
    assert_eq!(matrices.inv_dc_quant(0).to_bits(), bits);
    assert_eq!(matrices.dc_quant(0).to_bits(), bits | 0x0040_0000);
}

#[test]
fn default_dc_output_is_one_bit_and_default_input_retains_reused_values() {
    let matrices = DequantMatrices::default();
    let mut writer = BitWriter::new();
    assert_eq!(matrices.dc_quants(), &DC_QUANT);
    dequant_matrices_encode_dc(&matrices, &mut writer, LayerType::Quant, None).unwrap();
    assert_eq!(writer.bits_written(), 1);
    writer.zero_pad_to_byte();
    assert_eq!(writer.get_span(), &[1]);
    let mut reused = DequantMatrices::default();
    reused.set_dc_quant(&[3.0, 7.0, 9.0]);
    let before = *reused.dc_quants();
    let mut reader = BitReader::new(writer.get_span());
    reused.decode_dc(&mut reader).unwrap();
    reader.close().unwrap();
    assert_eq!(reused.dc_quants(), &before);
}

#[test]
fn invalid_dc_coefficient_retains_prior_decoded_components_and_its_old_inverse() {
    let mut writer = BitWriter::new();
    writer
        .with_max_bits(
            64,
            LayerType::Quant,
            None,
            |w, _| {
                w.write(1, 0);
                for value in [1.0, -1.0, 1.0] {
                    f16_coder::write(value, w)?;
                }
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    let mut matrices = DequantMatrices::default();
    matrices.set_dc_quant(&[3.0, 7.0, 9.0]);
    let old_third = matrices.dc_quant(2);
    let mut reader = BitReader::new(writer.get_span());
    assert_eq!(
        matrices.decode_dc(&mut reader),
        Err(StatusCode::GenericError)
    );
    assert_eq!(reader.total_bits_consumed(), 33);
    reader.close().unwrap();
    assert_eq!(matrices.dc_quant(0), 1.0 / 128.0);
    assert_eq!(matrices.inv_dc_quant(0), 128.0);
    assert_eq!(matrices.dc_quant(1), -1.0 / 128.0);
    assert_eq!(matrices.inv_dc_quant(1), 7.0);
    assert_eq!(matrices.dc_quant(2), old_third);
    assert_eq!(matrices.inv_dc_quant(2), 9.0);
}
