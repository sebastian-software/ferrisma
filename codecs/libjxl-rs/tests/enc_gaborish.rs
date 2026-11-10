// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// lib/jxl/enc_gaborish_test.cc with its original tolerances, plus the
// VerifyRelativeError check from lib/jxl/image_test_utils.h. TestDirac is
// disabled upstream (#if JXL_FALSE) and stays disabled here; the parity
// corpus compares its input as exact words. The exact original words for
// TestZero and TestFlat are compared in tests/parity/encoder_transforms.rs.
use jxl::jxl::base::rect::Rect;
use jxl::jxl::convolve::WeightsSymmetric3;
use jxl::jxl::convolve_slow::slow_symmetric3;
use jxl::jxl::enc_gaborish::gaborish_inverse;
use jxl::jxl::image::{Image3F, ImageF};
use jxl::jxl::image_ops::{fill_image3, zero_fill_image3};

fn gaborish_kernel(weight1: f32, weight2: f32) -> WeightsSymmetric3 {
    let weight0 = 1.0f32;

    // Normalize
    let mul = 1.0f32 / (weight0 + 4.0 * (weight1 + weight2));
    let w0 = weight0 * mul;
    let w1 = weight1 * mul;
    let w2 = weight2 * mul;

    WeightsSymmetric3 {
        c: [w0; 4],
        r: [w1; 4],
        d: [w2; 4],
    }
}

fn convolve_gaborish(input: &ImageF, weight1: f32, weight2: f32, out: &mut ImageF) {
    assert_eq!((input.xsize(), input.ysize()), (out.xsize(), out.ysize()));
    let rect = Rect::new(0, 0, input.xsize(), input.ysize());
    slow_symmetric3(input, rect, &gaborish_kernel(weight1, weight2), None, out).unwrap();
}

fn verify_relative_error(
    expected: &ImageF,
    actual: &ImageF,
    threshold_l1: f64,
    threshold_relative: f64,
) -> bool {
    assert_eq!(
        (expected.xsize(), expected.ysize()),
        (actual.xsize(), actual.ysize())
    );
    let mut any_bad = false;
    for y in 0..expected.ysize() {
        for x in 0..expected.xsize() {
            let e = expected.row(y)[x];
            let l1 = f64::from((e - actual.row(y)[x]).abs());
            // Cannot compute relative, only check/update L1.
            if e.abs() < 1E-10 {
                if l1 > threshold_l1 {
                    any_bad = true;
                }
            } else {
                let relative = l1 / f64::from(e).abs();
                if l1 > threshold_l1 && relative > threshold_relative {
                    any_bad = true;
                }
            }
        }
    }
    !any_bad
}

fn test_round_trip(input: &Image3F, max_l1: f32) {
    let mut fwd = Image3F::create(input.xsize(), input.ysize()).unwrap();
    for c in 0..3 {
        convolve_gaborish(input.plane(c), 0.0, 0.0, fwd.plane_mut(c));
    }
    let w = 0.927_189_3_f32;
    let weights = [w, w, w];
    let rect = Rect::new(0, 0, fwd.xsize(), fwd.ysize());
    gaborish_inverse(&mut fwd, rect, &weights, None).unwrap();
    for c in 0..3 {
        // The Image3 overload narrows both thresholds to float.
        assert!(verify_relative_error(
            input.plane(c),
            fwd.plane(c),
            f64::from(max_l1),
            f64::from(1E-4f32)
        ));
    }
}

#[test]
fn gaborish_test_zero() {
    let mut input = Image3F::create(20, 20).unwrap();
    zero_fill_image3(&mut input);
    test_round_trip(&input, 0.0);
}

#[test]
fn gaborish_test_flat() {
    let mut input = Image3F::create(20, 20).unwrap();
    fill_image3(1.0f32, &mut input);
    test_round_trip(&input, 1E-5);
}
