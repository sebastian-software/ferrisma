// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Independent known answers and round-trip/ownership checks for scalar RCT.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::transform::enc_rct::{fwd_rct, fwd_rct_channels};
use jxl::jxl::modular::transform::rct::inv_rct;

#[test]
fn all_42_color_transforms_round_trip_samples_and_keep_padding() {
    for kind in 0..42 {
        let mut image = Image::create(17, 3, 16, 6).unwrap();
        for c in 0..6 {
            image.channel[c].plane.bytes_mut().unwrap().fill(0xa5);
            for y in 0..3 {
                for x in 0..17 {
                    image.channel[c].row_mut(y)[x] =
                        (((c * 4096 + y * 257 + x * 3) % 65536) as i32) - 32768;
                }
            }
        }
        let original = Image::clone_image(&image).unwrap();
        let forward = fwd_rct(&mut image, 3, kind);
        assert_eq!(
            forward,
            if kind == 0 {
                Err(StatusCode::GenericError)
            } else {
                Ok(())
            }
        );
        inv_rct(&mut image, 3, kind).unwrap();
        for c in 0..6 {
            for y in 0..3 {
                assert_eq!(
                    &image.channel[c].row(y)[..17],
                    &original.channel[c].row(y)[..17],
                    "type {kind}, channel {c}"
                );
                let p = &image.channel[c].plane;
                let start = y * p.bytes_per_row() + 17 * 4;
                let end = (y + 1) * p.bytes_per_row();
                assert!(p.bytes().unwrap()[start..end].iter().all(|&v| v == 0xa5));
            }
        }
    }
}
#[test]
fn ycocg_known_answer_and_inverse_full_width_wrap_match_source_math() {
    let mut image = Image::create(1, 1, 16, 3).unwrap();
    for (c, v) in [10, 20, 30].into_iter().enumerate() {
        image.channel[c].row_mut(0)[0] = v;
    }
    fwd_rct(&mut image, 0, 6).unwrap();
    assert_eq!(
        [
            image.channel[0].row(0)[0],
            image.channel[1].row(0)[0],
            image.channel[2].row(0)[0]
        ],
        [20, -20, 0]
    );
    inv_rct(&mut image, 0, 6).unwrap();
    assert_eq!(image.channel[2].row(0)[0], 30);
    for (c, v) in [i32::MIN, i32::MAX, i32::MIN].into_iter().enumerate() {
        image.channel[c].row_mut(0)[0] = v;
    }
    inv_rct(&mut image, 0, 6).unwrap();
    assert_eq!(
        [
            image.channel[0].row(0)[0],
            image.channel[1].row(0)[0],
            image.channel[2].row(0)[0]
        ],
        [0, 1073741824, -2147483647]
    );
}
#[test]
fn pure_inverse_permutation_moves_channels_but_separate_forward_zero_copies() {
    let mut from = Image::create(1, 1, 8, 3).unwrap();
    let mut to = Image::create(1, 1, 8, 3).unwrap();
    for (c, v) in [10, 20, 30].into_iter().enumerate() {
        from.channel[c].row_mut(0)[0] = v;
    }
    let [a, b, c] = &mut to.channel[..] else {
        unreachable!()
    };
    fwd_rct_channels(
        [&from.channel[0], &from.channel[1], &from.channel[2]],
        [a, b, c],
        0,
    )
    .unwrap();
    inv_rct(&mut to, 0, 7).unwrap();
    assert_eq!(
        [
            to.channel[0].component,
            to.channel[1].component,
            to.channel[2].component
        ],
        [2, 0, 1]
    );
    assert_eq!(
        [
            to.channel[0].row(0)[0],
            to.channel[1].row(0)[0],
            to.channel[2].row(0)[0]
        ],
        [30, 10, 20]
    );
}
