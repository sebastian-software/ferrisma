// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Known answers, original stack semantics, and sample/ownership integration.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::transform::enc_rct::fwd_rct;
use jxl::jxl::modular::transform::enc_squeeze::fwd_squeeze;
use jxl::jxl::modular::transform::palette::{inv_palette, palette_internal::get_palette_value};
use jxl::jxl::modular::transform::squeeze::{
    default_squeeze_parameters, inv_squeeze, meta_squeeze, smooth_tendency,
};
use jxl::jxl::modular::transform::squeeze_params::SqueezeParams;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};

#[test]
fn implicit_palette_known_colors_and_empty_single_palette_keep_clamp_order() {
    assert_eq!(get_palette_value(&[], 0, 0, 0, 0, 8), 32);
    assert_eq!(get_palette_value(&[], 4, 1, 0, 0, 8), 95);
    assert_eq!(get_palette_value(&[], 16, 2, 0, 0, 8), 95);
    assert_eq!(get_palette_value(&[], 64, 0, 0, 0, 8), 0);
    assert_eq!(get_palette_value(&[], 68, 0, 0, 0, 8), 255);
    assert_eq!(get_palette_value(&[], -2, 0, 0, 0, 8), 4);
    assert_eq!(get_palette_value(&[], -3, 0, 0, 0, 8), -4);
    assert_eq!(get_palette_value(&[], -145, 0, 0, 0, 8), 4);
    let mut image = Image::create(4, 1, 8, 0).unwrap();
    image.nb_meta_channels = 1;
    image.channel.push(Channel::create(0, 1).unwrap());
    image.channel.push(Channel::create(4, 1).unwrap());
    image.channel[1].row_mut(0)[..4].copy_from_slice(&[i32::MIN, -1, 0, i32::MAX]);
    inv_palette(
        &mut image,
        0,
        0,
        0,
        jxl::jxl::modular::options::Predictor::ZERO,
        &weighted::Header::default(),
        None,
    )
    .unwrap();
    assert_eq!(&image.channel[0].row(0)[..4], &[32, 32, 0, 0]);
    assert_eq!(image.nb_meta_channels, 0);
}
#[test]
fn palette_rows_expand_rgb_indices_and_keep_original_index_channel_tag() {
    let mut image = Image::create(3, 1, 8, 0).unwrap();
    image.nb_meta_channels = 1;
    let mut palette = Channel::create(2, 3).unwrap();
    for (y, colors) in [[10, 20], [30, 40], [50, 60]].into_iter().enumerate() {
        palette.row_mut(y)[..2].copy_from_slice(&colors);
    }
    image.channel.push(palette);
    let mut index = Channel::create(3, 1).unwrap();
    index.component = 7;
    index.row_mut(0)[..3].copy_from_slice(&[1, 0, 1]);
    image.channel.push(index);
    inv_palette(
        &mut image,
        0,
        2,
        0,
        jxl::jxl::modular::options::Predictor::ZERO,
        &weighted::Header::default(),
        None,
    )
    .unwrap();
    for (c, expected) in [[20, 10, 20], [40, 30, 40], [60, 50, 60]]
        .into_iter()
        .enumerate()
    {
        assert_eq!(&image.channel[c].row(0)[..3], &expected);
    }
    assert_eq!(
        image
            .channel
            .iter()
            .map(|c| c.component)
            .collect::<Vec<_>>(),
        [7, -1, -1]
    );
}
#[test]
fn squeeze_signed_rounding_and_odd_rasters_round_trip_through_default_schedule() {
    assert_eq!(smooth_tendency(3, 2, 1), 1);
    assert_eq!(smooth_tendency(1, 2, 3), -1);
    assert_eq!(smooth_tendency(3, 1, 2), 0);
    let mut image = Image::create(17, 9, 16, 3).unwrap();
    for (c, ch) in image.channel.iter_mut().enumerate() {
        for y in 0..9 {
            for x in 0..17 {
                ch.row_mut(y)[x] = ((x * 17 + y * 137 + c * 377) % 65536) as i32 - 32768;
            }
        }
    }
    let original = Image::clone_image(&image).unwrap();
    let mut parameters = Vec::new();
    default_squeeze_parameters(&mut parameters, &image);
    assert!(parameters.len() > 2);
    fwd_squeeze(&mut image, parameters.clone()).unwrap();
    assert!(image.channel.len() > 3);
    inv_squeeze(&mut image, &parameters, None).unwrap();
    assert_eq!(image.channel.len(), 3);
    for c in 0..3 {
        for y in 0..9 {
            assert_eq!(
                &image.channel[c].row(y)[..17],
                &original.channel[c].row(y)[..17]
            );
        }
    }
    let mut one = Image::create(1, 1, 8, 1).unwrap();
    one.channel[0].row_mut(0)[0] = -1;
    assert_eq!(
        fwd_squeeze(&mut one, Vec::new()),
        Err(StatusCode::GenericError)
    );
    assert_eq!(one.channel[0].row(0)[0], -1);
}
#[test]
fn metadata_failure_keeps_incremented_meta_count_and_completed_channel_change() {
    let mut image = Image::create(3, 1, 8, 1).unwrap();
    let mut empty = Channel::create(0, 1).unwrap();
    empty.component = 1;
    image.channel.push(empty);
    image.nb_meta_channels = 2;
    let mut p = vec![SqueezeParams {
        horizontal: true,
        in_place: true,
        begin_c: 0,
        num_c: 2,
    }];
    assert_eq!(
        meta_squeeze(&mut image, &mut p),
        Err(StatusCode::GenericError)
    );
    assert_eq!(image.nb_meta_channels, 4);
    assert_eq!(image.channel.len(), 3);
    assert_eq!((image.channel[0].w, image.channel[0].hshift), (2, 1));
    assert_eq!((image.channel[1].w, image.channel[2].w), (0, 1));
}
#[test]
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn inverse_stack_pops_successes_retains_failure_and_restores_complete_rct_squeeze_chain() {
    let mut image = Image::create(17, 9, 16, 3).unwrap();
    for c in 0..3 {
        for y in 0..9 {
            for x in 0..17 {
                image.channel[c].row_mut(y)[x] = (c * 4096 + y * 31 + x) as i32 - 32768;
            }
        }
    }
    let original = Image::clone_image(&image).unwrap();
    let rct = Transform::new(TransformId::RCT);
    fwd_rct(&mut image, 0, rct.rct_type as usize).unwrap();
    image.transform.push(rct);
    let mut squeeze = Transform::new(TransformId::SQUEEZE);
    default_squeeze_parameters(&mut squeeze.squeezes, &image);
    fwd_squeeze(&mut image, squeeze.squeezes.clone()).unwrap();
    image.transform.push(squeeze);
    image.undo_transforms(&weighted::Header::default(), None);
    assert!(!image.error);
    assert!(image.transform.is_empty());
    for c in 0..3 {
        for y in 0..9 {
            assert_eq!(
                &image.channel[c].row(y)[..17],
                &original.channel[c].row(y)[..17]
            );
        }
    }
    image.transform.push(Transform::new(TransformId::INVALID));
    image.transform.push(Transform {
        rct_type: 0,
        ..Transform::new(TransformId::RCT)
    });
    image.undo_transforms(&weighted::Header::default(), None);
    assert!(image.error);
    assert_eq!(image.transform.len(), 1);
    assert_eq!(image.transform[0].id, TransformId::INVALID);
}
