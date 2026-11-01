// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::image_metadata::CodecMetadata;

#[test]
fn non_xyb_metadata_keeps_previous_opsin_matrix_and_default_flag() {
    let mut metadata = CodecMetadata::default();
    metadata
        .m
        .color_encoding
        .set_srgb(ColorSpace::RGB, RenderingIntent::RELATIVE)
        .unwrap();
    let mut info = OutputEncodingInfo::default();
    info.set_from_metadata(&metadata).unwrap();
    assert!(info.xyb_encoded);
    assert!(info.all_default_opsin);
    let previous = info.opsin_params.inverse_opsin_matrix.map(f32::to_bits);
    metadata.m.xyb_encoded = false;
    metadata.m.tone_mapping.intensity_target = 1000.0;
    metadata
        .m
        .color_encoding
        .set_srgb(ColorSpace::GRAY, RenderingIntent::RELATIVE)
        .unwrap();
    info.set_from_metadata(&metadata).unwrap();
    assert!(!info.xyb_encoded);
    assert!(info.all_default_opsin);
    assert_eq!(
        info.opsin_params.inverse_opsin_matrix.map(f32::to_bits),
        previous
    );
    assert!(info.color_encoding.is_gray());
    assert!(info.color_encoding_is_original);
    assert_eq!(info.desired_intensity_target, 1000.0);
}
