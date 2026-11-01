// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

// Captured independently from the actual AppleClang 21 scalar TransformToPixels
// objects (ordinary and instrumented, native AArch64) with the documented
// source-scoped -fno-unroll-loops, not a substitute arithmetic probe.
#[cfg(target_arch = "aarch64")]
#[test]
fn selected_original_identity_keeps_nan_words_and_fused_subnormal_rounding() {
    use jxl::jxl::ac_strategy::AcStrategyType;
    use jxl::jxl::dec_transforms::transform_to_pixels;
    let mut coefficients = [0.125; 64];
    coefficients[21] = f32::from_bits(std::hint::black_box(0x7fc1_2345));
    let mut pixels = [-123.25; 108];
    let mut scratch = [-123.25; 308];
    transform_to_pixels(
        AcStrategyType::IDENTITY,
        &mut coefficients,
        &mut pixels,
        13,
        &mut scratch,
    );
    assert_eq!(pixels[4].to_bits(), 0x7fc1_2345);
    assert_eq!(pixels[19].to_bits(), 0x7fc1_2345);
    assert_eq!(coefficients[21].to_bits(), 0x7fc1_2345);
    assert!(scratch.iter().all(|v| *v == -123.25));
    coefficients.fill(0.0);
    coefficients[0] = f32::from_bits(1);
    for i in [2, 4, 6, 16, 18, 20, 22, 32] {
        coefficients[i] = f32::from_bits(1);
    }
    transform_to_pixels(
        AcStrategyType::IDENTITY,
        &mut coefficients,
        &mut pixels,
        13,
        &mut scratch,
    );
    assert_eq!(pixels[14].to_bits(), 0);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn selected_original_dct_and_afv_keep_competing_payloads_and_length_one_raw_copies() {
    use jxl::jxl::ac_strategy::AcStrategyType;
    use jxl::jxl::dct::compute_scaled_dct;
    use jxl::jxl::dec_transforms::{lowest_frequencies_from_dc, transform_to_pixels};
    // Captured from both complete original graphs at the final configuration.
    for (kind, expected) in [
        (AcStrategyType::DCT, 0x7fc1_2345),
        (AcStrategyType::AFV0, 0xffc9_8765),
    ] {
        let mut coefficients = [0.125; 64];
        coefficients[0] = f32::from_bits(std::hint::black_box(0x7fc1_2345));
        coefficients[1] = f32::from_bits(std::hint::black_box(0xffc9_8765));
        let mut pixels = [0.0; 88];
        let mut scratch = [0.0; 308];
        transform_to_pixels(kind, &mut coefficients, &mut pixels, 11, &mut scratch);
        assert_eq!(pixels[0].to_bits(), expected);
    }
    let dc = [
        f32::from_bits(0xffc9_8765),
        f32::from_bits(std::hint::black_box(0x7fa1_2345)),
        0.125,
        0.125,
        0.125,
    ];
    let mut llf = [0.0; 128];
    let mut scratch = [0.0; 32];
    lowest_frequencies_from_dc(AcStrategyType::DCT8X16, &dc, 5, &mut llf, &mut scratch);
    assert_eq!(llf[0].to_bits(), 0x7fe1_2345);
    let input = [f32::from_bits(std::hint::black_box(0x7fa1_2345))];
    let mut output = [0.0];
    compute_scaled_dct(1, 1, &input, 1, &mut output, &mut scratch);
    assert_eq!(output[0].to_bits(), 0x7fa1_2345);
}
