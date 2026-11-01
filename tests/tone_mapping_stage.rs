// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::cms::tone_mapping_scalar::gamut_map;
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::render_pipeline::stage_tone_mapping::get_tone_mapping_stage;
#[test]
fn fixed_lossless_metadata_selects_original_no_tone_mapping_path() {
    for gray in [false, true] {
        for depth in [LosslessSampleDepth::Uint8, LosslessSampleDepth::Uint16] {
            let config = LosslessThunderConfig::new(17, 9, gray, depth).unwrap();
            let mut info = OutputEncodingInfo::default();
            info.set_from_metadata(config.metadata()).unwrap();
            assert_eq!(
                info.orig_intensity_target.to_bits(),
                info.desired_intensity_target.to_bits()
            );
            assert!(get_tone_mapping_stage(&info).is_none());
        }
    }
}

#[test]
fn gamut_blend_keeps_original_add_operand_nan_payload() {
    let mut rgb = [-0.0, f32::from_bits(0x7fa12345), f32::from_bits(0x7fc98765)];
    gamut_map(&mut rgb, [0.2126, 0.7152, 0.0722], 0.0);
    // Captured from the original one-lane GamutMap, independently of the
    // differential fixture: the blue quiet-NaN payload survives its blend.
    assert_eq!(rgb.map(f32::to_bits), [0x7fe12345, 0x7fe12345, 0x7fc98765]);
}
