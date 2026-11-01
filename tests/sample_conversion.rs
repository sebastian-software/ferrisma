// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::dec_modular::{
    int_to_float, multiply_sum, rgb_from_single, single_from_single, single_from_single_accurate,
};
use jxl::jxl::enc_modular::float_to_int;

#[test]
fn gray_and_rgb_integer_rows_recover_every_eight_and_sixteen_bit_sample() {
    for bits in [8, 16] {
        let max = (1i32 << bits) - 1;
        let input: Vec<_> = (0..=max).collect();
        let factor = 1.0 / max as f64;
        let mut gray = vec![0.0; input.len()];
        single_from_single(input.len(), &input, factor as f32, &mut gray);
        let mut r = gray.clone();
        let mut g = gray.clone();
        let mut b = gray.clone();
        rgb_from_single(input.len(), &input, factor as f32, &mut r, &mut g, &mut b);
        assert_eq!(r, gray);
        assert_eq!(g, gray);
        assert_eq!(b, gray);
        let mut reconstructed = vec![-1; input.len()];
        float_to_int(
            &gray,
            &mut reconstructed,
            input.len(),
            bits,
            0,
            false,
            max as f64,
        )
        .unwrap();
        assert_eq!(reconstructed, input);
    }
}

#[test]
fn every_binary16_pattern_survives_conversion_including_signed_zero_and_nan_payloads() {
    let input: Vec<_> = (0..65536).collect();
    let mut decoded = vec![0.0; input.len()];
    int_to_float(&input, &mut decoded, input.len(), 16, 5).unwrap();
    assert_eq!(decoded[0].to_bits(), 0);
    assert_eq!(decoded[32768].to_bits(), 1 << 31);
    assert_eq!(decoded[1].to_bits(), (2.0f32).powi(-24).to_bits());
    assert_eq!(decoded[0x3c00].to_bits(), 1.0f32.to_bits());
    let mut reconstructed = vec![-1; input.len()];
    float_to_int(&decoded, &mut reconstructed, input.len(), 16, 5, true, 1.0).unwrap();
    assert_eq!(reconstructed, input);
}

#[test]
fn binary32_copy_preserves_signaling_nan_bits_and_invalid_width_preserves_storage() {
    let input = [
        0,
        i32::MIN,
        0x7f80_0000,
        0xff80_0000u32 as i32,
        0x7f80_0001,
        -1,
    ];
    let mut decoded = [42.0; 7];
    int_to_float(&input, &mut decoded, input.len(), 32, 8).unwrap();
    assert_eq!(
        decoded[..input.len()]
            .iter()
            .map(|v| v.to_bits() as i32)
            .collect::<Vec<_>>(),
        input
    );
    assert_eq!(decoded[6], 42.0);
    let mut output = [-7; 7];
    float_to_int(&decoded, &mut output, input.len(), 32, 8, true, 1.0).unwrap();
    assert_eq!(&output[..input.len()], &input);
    assert_eq!(output[6], -7);
    for (bits, exp) in [(32, 7), (33, 8), (0, 5)] {
        let previous = output;
        assert_eq!(
            float_to_int(&decoded, &mut output, 6, bits, exp, true, 1.0),
            Err(StatusCode::GenericError)
        );
        assert_eq!(output, previous);
    }
    let previous = decoded.map(f32::to_bits);
    assert_eq!(
        int_to_float(&input, &mut decoded, 6, 32, 7),
        Err(StatusCode::GenericError)
    );
    assert_eq!(decoded.map(f32::to_bits), previous);
}

#[test]
fn custom_float_error_preserves_completed_prefix_and_untouched_tail() {
    let input = [0.0, -0.0, 1.0, f32::from_bits(0x3f80_0001), 2.0];
    let mut output = [-17; 6];
    assert_eq!(
        float_to_int(&input, &mut output, 5, 16, 5, true, 1.0),
        Err(StatusCode::GenericError)
    );
    assert_eq!(output, [0, 0x8000, 0x3c00, -17, -17, -17]);
    float_to_int(&[0.5, -0.5], &mut output, 2, 8, 0, false, 1.0).unwrap();
    assert_eq!(&output[..2], &[1, -1]);
}

#[test]
fn format_errors_precede_any_buffer_access_or_length_precondition() {
    for bits in [0, 32, 33, u32::MAX] {
        assert_eq!(
            float_to_int(&[], &mut [], usize::MAX, bits, 7, true, 1.0),
            Err(StatusCode::GenericError)
        );
    }
    assert_eq!(
        int_to_float(&[], &mut [], usize::MAX, 32, 7),
        Err(StatusCode::GenericError)
    );
}

#[test]
fn scalar_sum_wraps_before_conversion_and_double_product_retains_extra_precision() {
    let mut output = [7.0; 4];
    multiply_sum(2, &[i32::MAX, i32::MIN], &[1, -1], 1.0, &mut output);
    assert_eq!(output, [i32::MIN as f32, i32::MAX as f32, 7.0, 7.0]);
    let input = [16_777_217, 16_777_219];
    let factor = 1.0 / 16_777_217.0;
    single_from_single_accurate(2, &input, factor, &mut output);
    assert_eq!(output[0].to_bits(), 1.0f32.to_bits());
    assert_eq!(output[1].to_bits(), 0x3f80_0001);
    assert_eq!(output[2..], [7.0, 7.0]);
}
