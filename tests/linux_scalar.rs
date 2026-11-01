// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Exact observations from the original scalar reference configurations.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod reference_targets {
    use jxl::jxl::base::status::StatusCode;
    use jxl::jxl::enc_quant_weights::dequant_matrices_scale_dc;
    use jxl::jxl::image::Plane;
    use jxl::jxl::image_ops::downsample_image;
    use jxl::jxl::quant_weights::DequantMatrices;

    #[test]
    fn original_downsampling_accumulator_keeps_target_nan_payloads() {
        let values = [
            0, 0x80000000, 1, 0x80000001, 0x3f800000, 0xbf800000, 0x7f7fffff, 0xff7fffff,
            0x7f800000, 0xff800000, 0x7fc12345, 0x7fa23456,
        ];
        let mut image = Plane::create(7, 9).unwrap();
        for y in 0..9 {
            for x in 0..7 {
                image.row_mut(y)[x] = f32::from_bits(values[(x + y * 7) % 12]);
            }
        }
        let down = downsample_image(&image, 2).unwrap();
        // Captured from image-downsample 0 7 9 2 1 in the pinned C++ oracle.
        // x86 ADDSS retains the accumulated quiet payload when a later sample
        // is a signaling NaN. AArch64's FADD follows its original priority.
        let (invalid, mixed) = if cfg!(target_arch = "x86_64") {
            (0xffc00000, 0x7fc12345)
        } else {
            (0x7fc00000, 0x7fe23456)
        };
        let expected = [
            0x7f800000, 0x7fc12345, 0x7fe23456, 0x7effffff, 0x7fc12345, 0x7fe23456, 0, 0x7f800000,
            0x7fe23456, 0, invalid, 0x7fc12345, 0, invalid, mixed, 0xfeffffff, invalid, mixed, 0,
            1,
        ];
        assert_eq!((down.xsize(), down.ysize()), (4, 5));
        for y in 0..5 {
            for x in 0..4 {
                assert_eq!(down.row(y)[x].to_bits(), expected[y * 4 + x]);
            }
        }
    }

    #[test]
    fn original_dc_scaling_retains_failed_roundtrip_nan_state() {
        let mut matrices = DequantMatrices::default();
        matrices.set_dc_quant(&[4096.0, 512.0, f32::from_bits(0x7fb04f21)]);
        let status = dequant_matrices_scale_dc(&mut matrices, f32::from_bits(0x7fc12345));
        assert_eq!(status, Err(StatusCode::GenericError));
        // Source observations from the corresponding qdc-chain Set/Scale
        // sequence: the rejected half-float writer retains SetDCQuant's state.
        let third = if cfg!(target_arch = "x86_64") {
            0x7fc12345
        } else {
            0x7ff04f21
        };
        for (c, expected) in [0x7fc12345, 0x7fc12345, third].into_iter().enumerate() {
            assert_eq!(matrices.dc_quant(c).to_bits(), expected);
            assert_eq!(matrices.inv_dc_quant(c).to_bits(), expected);
        }
    }
}
