// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_frame::decode_vardct;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

#[test]
fn captured_original_vardct_files_decode_without_cpp() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    // Original pinned encoder, effort 7, distance 1, progressive AC, Gaborish,
    // EPF/patches/dots/noise off. Expected samples come from its decoder.
    for (bytes, expected, depth, alpha) in [
        (
            include_bytes!("data/vardct-rgb8.jxl").as_slice(),
            include_bytes!("data/vardct-rgb8.pixels").as_slice(),
            8,
            false,
        ),
        (
            include_bytes!("data/vardct-gray-alpha16.jxl").as_slice(),
            include_bytes!("data/vardct-gray-alpha16.pixels").as_slice(),
            16,
            true,
        ),
    ] {
        let result = decode_vardct(bytes);
        assert_eq!(result.status, Ok(()));
        assert_eq!(result.frame_size, (3, 8));
        assert_eq!(result.reader_bits, bytes.len() * 8);
        assert_eq!(result.metadata.m.bit_depth.bits_per_sample, depth);
        assert_eq!(result.metadata.m.has_alpha(), alpha);
        assert_eq!(result.pixels, expected);
        assert_eq!(result.float_pixels.len(), 3 * 8 * 3);
    }
}

#[test]
fn vardct_entry_retains_its_explicit_profile_boundary() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut config = LosslessThunderConfig::new(1, 1, false, LosslessSampleDepth::Uint8).unwrap();
    let modular = config.encode_pixels(&[13, 27, 41], None).unwrap();
    assert_eq!(
        decode_vardct(modular.as_slice()).status,
        Err(StatusCode::Unsupported)
    );
    assert_eq!(decode_vardct(&[]).status, Err(StatusCode::GenericError));
}
