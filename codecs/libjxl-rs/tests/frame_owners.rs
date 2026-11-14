// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::dec_patch_dictionary::ReferenceFrameBinding;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_cache::PassesEncoderState;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

#[test]
fn moved_shared_state_keeps_its_matrix_dc_and_reference_bindings_in_the_same_owner() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut state = PassesEncoderState::new();
    state.shared.bind_own_reference_frames();
    state.shared.matrices.set_dc_quant(&[2.0, 4.0, 8.0]);
    let mut state = std::convert::identity(state);
    state
        .shared
        .quantizer
        .recompute_from_global_scale(&state.shared.matrices);
    assert_eq!(state.shared.quantizer.mul_dc()[..3], [0.5, 0.25, 0.125]);
    assert!(std::ptr::eq(&*state.shared.dc(), &state.shared.dc_storage));
    assert_eq!(
        state
            .shared
            .image_features
            .patches
            .borrow()
            .reference_frame_binding(),
        Some(ReferenceFrameBinding::SharedState)
    );
    assert!(
        state.shared.reference_frames.borrow().iter().all(|r| r
            .frame
            .as_ref()
            .is_some_and(|b| b.metadata().is_none())
            && !r.ib_is_in_xyb)
    );
    assert_eq!(state.shared.block_ctx_map.ctx_map.len(), 39);
}

#[test]
fn take_bytes_transfers_the_existing_padded_allocation_without_copying() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut writer = BitWriter::new();
    writer
        .with_max_bits(
            24,
            LayerType::Header,
            None,
            |w, _| {
                w.write(24, 0x563412);
                Ok(())
            },
            false,
        )
        .unwrap();
    let address = writer.get_span().as_ptr();
    let bytes = writer.take_bytes();
    assert_eq!(bytes.as_slice(), &[0x12, 0x34, 0x56]);
    assert_eq!(bytes.as_slice().as_ptr(), address);
    assert_eq!(bytes.len(), 3);
    assert!(!bytes.is_empty());
}

#[test]
fn complete_encoding_reuses_metadata_and_rejects_short_input_before_writing_statistics() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut config = LosslessThunderConfig::new(17, 9, false, LosslessSampleDepth::Uint16).unwrap();
    let data: Vec<_> = (0..17 * 9 * 3)
        .flat_map(|i| ((i * 1931) as u16).to_ne_bytes())
        .collect();
    let first = config.encode_pixels(&data, None).unwrap();
    let second = config.encode_pixels(&data, None).unwrap();
    assert_eq!(first.as_slice(), second.as_slice());
    let mut aux = jxl::jxl::enc_aux_out::AuxOut::default();
    assert!(
        config
            .encode_pixels(&data[..data.len() - 1], Some(&mut aux))
            .is_err()
    );
    assert_eq!(aux.layer(LayerType::Header).total_bits, 0);
}
