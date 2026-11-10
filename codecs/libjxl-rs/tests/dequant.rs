// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::quantizer_inl::adjust_quant_bias;

#[test]
fn quant_bias_retains_captured_original_rounding_and_integer_boundaries() {
    // Captured from both complete ordinary and ASan/UBSan original graphs.
    let biases = [
        1.0 - f32::from_bits(0x3d5fd8c1),
        1.0 - f32::from_bits(0x3d8f78bc),
        1.0 - f32::from_bits(0x3d4c88c0),
        0.145,
    ];
    let values = [0, 1, -1, 2, -2, i32::MIN, i32::MAX];
    let expected = [
        0, 1064436340, 3211919988, 1073133650, 3220617298, 3472883712, 1325400064,
    ];
    assert_eq!(
        values.map(|v| adjust_quant_bias(0, v, &biases).to_bits()),
        expected
    );
}

#[test]
fn quant_bias_zero_and_one_keep_original_select_and_sign_xor() {
    let biases = [
        f32::from_bits(0x7fa12345),
        f32::from_bits(0xffc98765),
        -0.0,
        0.145,
    ];
    for c in 0..3 {
        assert_eq!(adjust_quant_bias(c, 0, &biases).to_bits(), 0);
        assert_eq!(
            adjust_quant_bias(c, 1, &biases).to_bits(),
            biases[c].to_bits()
        );
        assert_eq!(
            adjust_quant_bias(c, -1, &biases).to_bits(),
            biases[c].to_bits() ^ 0x80000000
        );
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn dequant_x_correlation_retains_captured_native_x86_nan_operand() {
    use jxl::jxl::dct_util::ConstAcPtr;
    use jxl::jxl::dec_group_render::dequant_lane;
    // Independent original Linux capture, PR #13 run 41410474499, lane 7.
    // Ordinary and fully instrumented original observers agree on these words.
    let matrices = [
        [f32::from_bits(0x7fa12345)],
        [f32::from_bits(0xffa98765)],
        [0.0],
    ];
    let quantized = [[32767i16], [-32768], [0]];
    let biases = [1064436340, 1064177896, 1064515444, 1041529569].map(f32::from_bits);
    let mut block = [0.0; 3];
    dequant_lane(
        [0.125 * 0.8, 0.125, 0.125 * 1.25],
        matrices.each_ref().map(|v| &v[..]),
        1,
        0,
        -0.125,
        0.375,
        &biases,
        &quantized.each_ref().map(|v| ConstAcPtr::I16(v)),
        &mut block,
    );
    assert_eq!(block.map(f32::to_bits), [0xffe98765; 3]);
}
