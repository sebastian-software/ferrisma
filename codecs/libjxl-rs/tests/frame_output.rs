// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::enc_aux_out::AuxOut;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::image::Image3;

#[test]
fn zero_gray_frame_sections_include_the_original_header_toc_and_group_payload() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let config = LosslessThunderConfig::new(1, 1, true, LosslessSampleDepth::Uint8).unwrap();
    let mut frame = config.create_frame().unwrap();
    frame
        .prepare_modular(&Image3::create(1, 1).unwrap())
        .unwrap();
    frame.compute_modular_tokens().unwrap();
    let mut groups = Vec::new();
    let mut aux = AuxOut::default();
    frame.encode_groups(&mut groups, Some(&mut aux)).unwrap();
    let writer = frame.finalize_groups(&groups, Some(&mut aux)).unwrap();
    // Captured independently from the original header/TOC/append stages.
    assert_eq!(
        writer.get_span(),
        &[
            0x08, 0x00, 0x01, 0x00, 0x1c, 0x00, 0x4b, 0x12, 0xa5, 0x42, 0x85, 0x24, 0x0c
        ]
    );
    assert!(writer.get_span().ends_with(groups[0].get_span()));
    assert_eq!(writer.bits_written(), aux.total_bits());
}

#[test]
fn unaligned_group_failure_keeps_the_header_mutation_and_its_accounting() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let config = LosslessThunderConfig::new(17, 9, false, LosslessSampleDepth::Uint16).unwrap();
    let mut frame = config.create_frame().unwrap();
    let mut code = BitWriter::new();
    code.with_max_bits(
        1,
        jxl::jxl::enc_aux_out::LayerType::Header,
        None,
        |w, _| {
            w.write(1, 1);
            Ok(())
        },
        false,
    )
    .unwrap();
    let mut aux = AuxOut::default();
    assert!(frame.header().all_default);
    assert!(matches!(
        frame.finalize_groups(&[code], Some(&mut aux)),
        Err(StatusCode::GenericError)
    ));
    assert!(!frame.header().all_default);
    assert!(aux.total_bits() > 0);
    assert_eq!(
        aux.layers[jxl::jxl::enc_aux_out::LayerType::Toc as usize].total_bits,
        0
    );
}
