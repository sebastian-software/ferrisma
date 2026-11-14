// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::ac_strategy::{AcStrategyImage, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::Quantizer;

#[test]
fn color_map_default_has_uncomputed_dc_factors_and_creation_uses_original_tile_rounding() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let default = ColorCorrelationMap::default();
    assert_eq!(default.base().base_correlation_b(), 1.0);
    assert_eq!(default.base().dc_factors(), &[0.0; 4]);
    let xyb = ColorCorrelationMap::create(65, 129, true).unwrap();
    assert_eq!((xyb.ytox_map.xsize(), xyb.ytox_map.ysize()), (2, 3));
    assert_eq!(xyb.base().dc_factors(), &[0.0, 0.0, 1.0, 0.0]);
    let rgb = ColorCorrelationMap::create(65, 129, false).unwrap();
    assert!(rgb.base().is_jpeg_compatible());
    assert_eq!(rgb.base().dc_factors(), &[0.0; 4]);
    for y in 0..3 {
        assert_eq!(&rgb.ytox_map.row(y)[..2], &[0, 0]);
        assert_eq!(&rgb.ytob_map.row(y)[..2], &[0, 0]);
    }
}

#[test]
fn quantizer_recomputation_reads_the_original_matrix_owner_and_retains_the_cleared_fourth_lanes() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut matrices = DequantMatrices::default();
    let mut quantizer = Quantizer::new(&matrices);
    assert_eq!(quantizer.scale(), 1.0 / 64.0);
    assert_eq!(quantizer.inv_global_scale(), 64.0);
    assert_eq!(quantizer.inv_quant_dc(), 1.0);
    let old = quantizer.mul_dc()[0];
    matrices.set_dc_quant(&[2.0, 4.0, 8.0]);
    assert_eq!(quantizer.mul_dc()[0], old);
    assert_eq!(quantizer.dc_step(&matrices, 0), 0.5);
    quantizer.clear_dc_mul();
    quantizer.recompute_from_global_scale(&matrices);
    assert_eq!(quantizer.mul_dc(), &[0.5, 0.25, 0.125, 1.0]);
    assert_eq!(quantizer.inv_mul_dc(), &[2.0, 4.0, 8.0, 1.0]);
}

#[test]
fn default_quantizer_keeps_the_original_arithmetic_that_quiets_a_signaling_nan() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let nan = f32::from_bits(std::hint::black_box(0x7fa1_2345));
    let mut matrices = DequantMatrices::default();
    matrices.set_dc_quant(&[1.0, 1.0, nan]);
    let quantizer = Quantizer::new(&matrices);
    assert_eq!(matrices.inv_dc_quant(2).to_bits(), 0x7fa1_2345);
    // The stored matrix value is a copy and stays exact; the recomputed
    // products are arithmetic, whose NaN payload (quieting included) is not
    // gated under decision D8 unless JXL_PARITY_STRICT=1.
    if std::env::var_os("JXL_PARITY_STRICT").is_some_and(|v| v == "1") {
        assert_eq!(quantizer.mul_dc()[2].to_bits(), 0x7fe1_2345);
        assert_eq!(quantizer.inv_mul_dc()[2].to_bits(), 0x7fe1_2345);
    } else {
        assert!(quantizer.mul_dc()[2].is_nan() && quantizer.inv_mul_dc()[2].is_nan());
    }
}

#[test]
fn strategy_counts_keep_the_original_byte_truncation_then_integer_promotion_for_unnamed_types() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut image = AcStrategyImage::create(3, 2).unwrap();
    image.fill_invalid();
    assert_eq!(image.count_blocks(AcStrategyType(127)), 6);
    image.fill_dct8(Rect::new(0, 0, 2, 1));
    let image = std::convert::identity(image);
    assert_eq!(image.count_blocks(AcStrategyType::DCT), 2);
    assert_eq!(image.count_blocks(AcStrategyType(256)), 2);
    assert_eq!(image.count_blocks(AcStrategyType(127)), 4);
    assert_eq!(image.count_blocks(AcStrategyType(128)), 0);
    assert_eq!(image.count_blocks(AcStrategyType(255)), 0);
    assert!(image.is_valid(0, 0));
    assert!(!image.is_valid(2, 1));
}

#[test]
fn modular_field_allocation_retains_the_original_xyb_map_default_and_block_storage_dimensions() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let config = LosslessThunderConfig::new(513, 3, false, LosslessSampleDepth::Uint16).unwrap();
    let mut frame = config.create_frame().unwrap();
    assert_eq!(frame.fields().cmap.base().dc_factors()[2], 0.0);
    frame.allocate_frame_fields().unwrap();
    let fields = frame.fields();
    assert_eq!(
        (fields.ac_strategy.xsize(), fields.ac_strategy.ysize()),
        (65, 1)
    );
    assert_eq!(
        (
            fields.raw_quant_field.xsize(),
            fields.raw_quant_field.ysize()
        ),
        (65, 1)
    );
    assert_eq!(
        (fields.dc_storage.xsize(), fields.dc_storage.ysize()),
        (65, 1)
    );
    assert_eq!(
        (fields.cmap.ytox_map.xsize(), fields.cmap.ytox_map.ysize()),
        (9, 1)
    );
    assert_eq!(fields.cmap.base().base_correlation_b(), 1.0);
    assert_eq!(fields.cmap.base().dc_factors()[2], 1.0);
    assert_eq!(fields.coeff_order_size, 6156 * 64);
    assert_eq!(frame.quantizer().scale(), 1.0 / 64.0);
}
