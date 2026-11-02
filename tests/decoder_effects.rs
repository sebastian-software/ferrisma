// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::dec_frame::decode_vardct;

#[test]
fn original_epf_noise_and_spline_files_decode_without_a_cpp_runtime() {
    // Independently captured original raw one-shot encoder files and original
    // scalar decoder output, with EPF=3, Gaborish, progressive AC, nonzero
    // manual noise, one genuine custom spline, and full-resolution alpha.
    for (file, pixels, floats) in [
        (
            include_bytes!("data/effects_rgb_alpha8.jxl").as_slice(),
            include_bytes!("data/effects_rgb_alpha8.pixels").as_slice(),
            include_bytes!("data/effects_rgb_alpha8.f32le").as_slice(),
        ),
        (
            include_bytes!("data/effects_gray_alpha16.jxl").as_slice(),
            include_bytes!("data/effects_gray_alpha16.pixels").as_slice(),
            include_bytes!("data/effects_gray_alpha16.f32le").as_slice(),
        ),
    ] {
        let decoded = decode_vardct(file);
        assert_eq!(decoded.status, Ok(()));
        assert_eq!(decoded.frame_size, (3, 8));
        assert_eq!(decoded.reader_bits, file.len() * 8);
        assert_eq!(decoded.pixels, pixels);
        let expected: Vec<_> = floats
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(
            decoded
                .float_pixels
                .iter()
                .map(|f| f.to_bits())
                .collect::<Vec<_>>(),
            expected
        );
    }
}
