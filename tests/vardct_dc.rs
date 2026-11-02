// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

#[cfg(target_arch = "aarch64")]
#[test]
fn smoothing_retains_the_selected_original_nan_payload() {
    use jxl::jxl::compressed_dc::adaptive_dc_smoothing;
    use jxl::jxl::frame_header::YCbCrChromaSubsampling;
    use jxl::jxl::image::Image3;
    // The selected pristine scalar object retains each source operator in its
    // own boundary at O0 with all sanitizers active. Independent ordinary and
    // ASan/UBSan captures give the neighbor payload at the center in all planes.
    let input = [
        0x3a01_2345,
        0x3a01_2345,
        0x7fc1_2345,
        0x3a01_2345,
        0xffc9_8765,
        0x7fc1_2345,
        0x3a01_2345,
        0x3a01_2345,
        0x7fc1_2345,
    ];
    let mut image = Image3::create(3, 3).unwrap();
    for c in 0..3 {
        for y in 0..3 {
            for x in 0..3 {
                image.plane_row_mut(c, y)[x] = f32::from_bits(input[y * 3 + x]);
            }
        }
    }
    adaptive_dc_smoothing(
        &[0.125, 0.5, 2.0],
        &mut image,
        &YCbCrChromaSubsampling::default(),
    )
    .unwrap();
    for c in 0..3 {
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(
                    image.plane_row(c, y)[x].to_bits(),
                    if x == 1 && y == 1 {
                        0x7fc1_2345
                    } else {
                        input[y * 3 + x]
                    }
                );
            }
        }
    }
}
