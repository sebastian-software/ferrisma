// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_cache::{DecoderShared, PassesDecoderState};
use jxl::jxl::frame_header::{FLAG_USE_DC_FRAME, FrameEncoding, FrameHeader};
use jxl::jxl::image::Image3;
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::passes_state::{DcSource, PassesSharedState, initialize_passes_shared_state};
use std::sync::atomic::Ordering;

#[test]
fn missing_dc_frame_error_keeps_new_binding_and_successful_binding_survives_owner_move() {
    let mut metadata = CodecMetadata::default();
    metadata.size.set(17, 9).unwrap();
    metadata.m.xyb_encoded = false;
    let mut header = FrameHeader::new(Some(&metadata));
    header.encoding = FrameEncoding::MODULAR;
    header.flags = FLAG_USE_DC_FRAME;
    header.dc_level = 2;
    let mut shared = PassesSharedState::new();
    shared.dc_storage = Image3::create(4, 3).unwrap();
    assert_eq!(
        initialize_passes_shared_state(&header, &mut shared, false),
        Err(StatusCode::GenericError)
    );
    assert_eq!(shared.dc_source, DcSource::Frame(2));
    assert_eq!(shared.dc_storage.xsize(), 0);
    shared.dc_frames[2] = Image3::create(2, 1).unwrap();
    shared.dc_frames[2].plane_row_mut(0, 0)[0] = 7.0;
    initialize_passes_shared_state(&header, &mut shared, false).unwrap();
    for y in 0..shared.quant_dc.ysize() {
        assert!(
            shared.quant_dc.row(y)[..shared.quant_dc.xsize()]
                .iter()
                .all(|v| *v == 0)
        );
    }
    let mut moved = Box::new(shared);
    assert!(std::ptr::eq(moved.dc(), &moved.dc_frames[2]));
    assert_eq!(moved.dc().plane_row(0, 0)[0], 7.0);
    // InitializePassesSharedState uses Create's original xyb=true default even
    // for the selected lossless profile's non-XYB metadata.
    assert_eq!(moved.cmap.base().base_correlation_b(), 1.0);
    header.flags = 0;
    header.dc_level = 0;
    initialize_passes_shared_state(&header, &mut moved, false).unwrap();
    let moved = *moved;
    assert!(std::ptr::eq(moved.dc(), &moved.dc_storage));
    assert_eq!(moved.dc_frames[2].plane_row(0, 0)[0], 7.0);
}

#[test]
fn borrowed_shared_state_sizes_sigma_while_ac_orders_mutate_own_storage() {
    let mut metadata = CodecMetadata::default();
    metadata.size.set(3, 2).unwrap();
    let mut ext_metadata = CodecMetadata::default();
    ext_metadata.size.set(65, 33).unwrap();
    let mut header = FrameHeader::new(Some(&metadata));
    header.loop_filter.epf_iters = 1;
    let ext_header = FrameHeader::new(Some(&ext_metadata));
    let mut ext = PassesSharedState::new();
    ext.metadata = Some(&ext_metadata);
    ext.frame_dim = ext_header.to_frame_dimensions();
    ext.coeff_order_size = 333;
    ext.coeff_orders = vec![9, 2];
    let mut state = PassesDecoderState::new();
    state.shared_storage.metadata = Some(&metadata);
    state.shared_storage.frame_dim = header.to_frame_dimensions();
    state.shared = DecoderShared::External(&ext);
    state.init(&header).unwrap();
    assert_eq!(
        (state.sigma.xsize(), state.sigma.ysize()),
        (
            ext.frame_dim.xsize_blocks + 4,
            ext.frame_dim.ysize_blocks + 4
        )
    );
    state.used_acs.store(1 << 26, Ordering::SeqCst);
    state.init_for_ac(2).unwrap();
    assert_eq!(
        state.shared_storage.coeff_orders.len(),
        2 * jxl::jxl::coeff_order::COEFF_ORDER_MAX_SIZE
    );
    assert_eq!(state.shared().coeff_order_size, 333);
    assert_eq!(ext.coeff_orders, [9, 2]);
}
