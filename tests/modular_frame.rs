// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

use jxl::jxl::common::SpeedTier;
use jxl::jxl::enc_modular::{
    do_transform, estimate_wp_cost, maybe_do_transform, merge_trees, quantize_channel, try_palettes,
};
use jxl::jxl::enc_modular_simd::estimate_cost;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::encoding::dec_ma::PropertyDecisionNode;
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::options::Predictor;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};

fn samples(image: &Image) -> Vec<i32> {
    image
        .channel
        .iter()
        .flat_map(|c| (0..c.h).flat_map(|y| c.row(y)[..c.w].iter().copied()))
        .collect()
}
fn patterned_rgb() -> Image {
    let mut im = Image::create(17, 9, 8, 3).unwrap();
    for c in 0..3 {
        for y in 0..9 {
            for x in 0..17 {
                im.channel[c].row_mut(y)[x] = ((x + 3 * y) % 4 * (c + 1) * 7) as i32;
            }
        }
    }
    im
}

#[test]
fn merge_appends_original_stream_decisions_and_offsets_even_leaf_child_fields() {
    let trees = vec![
        vec![PropertyDecisionNode::leaf(Predictor::LEFT, 17, 1)],
        vec![PropertyDecisionNode::leaf(Predictor::TOP, -3, 2)],
    ];
    let seed = PropertyDecisionNode::leaf(Predictor::ZERO, 29, 1);
    let mut tree = vec![seed];
    merge_trees(&trees, &[0, 8, 30], 0, 2, &mut tree).unwrap();
    assert_eq!(tree.len(), 4);
    assert_eq!(tree[0], seed);
    assert_eq!(
        (
            tree[1].property,
            tree[1].splitval,
            tree[1].lchild,
            tree[1].rchild
        ),
        (1, 7, 2, 3)
    );
    assert_eq!(
        (tree[2].predictor, tree[2].lchild, tree[2].rchild),
        (Predictor::TOP, 2, 2)
    );
    assert_eq!(
        (tree[3].predictor, tree[3].lchild, tree[3].rchild),
        (Predictor::LEFT, 3, 3)
    );
    let previous = tree.clone();
    assert!(merge_trees(&trees, &[0, 8], 0, 2, &mut tree).is_err());
    assert_eq!(tree, previous);
}

#[test]
fn transform_cost_rejection_restores_pixels_while_fast_selection_keeps_the_stack() {
    let mut image = patterned_rgb();
    let original = samples(&image);
    let mut transform = Transform::new(TransformId::RCT);
    transform.rct_type = 6;
    let mut p = CompressParams {
        speed_tier: SpeedTier::Kitten,
        ..Default::default()
    };
    let header = weighted::Header::default();
    assert!(!maybe_do_transform(&mut image, &transform, &p, &header, -1.0, false).unwrap());
    assert_eq!(samples(&image), original);
    assert!(image.transform.is_empty());
    p.speed_tier = SpeedTier::Thunder;
    assert!(maybe_do_transform(&mut image, &transform, &p, &header, -1.0, false).unwrap());
    assert_eq!(image.transform.len(), 1);
    assert_ne!(samples(&image), original);
    assert_eq!(transform.rct_type, 6);
    image.undo_transforms(&header);
    assert_eq!(samples(&image), original);
    assert!(!do_transform(
        &mut image,
        &Transform::new(TransformId::INVALID),
        &header,
        false
    ));
    assert!(image.transform.is_empty());
}

#[test]
fn palette_selection_and_transform_undo_recover_the_original_rgb_image() {
    let mut image = patterned_rgb();
    let original = samples(&image);
    let p = CompressParams {
        speed_tier: SpeedTier::Thunder,
        ..Default::default()
    };
    let (mut depth, mut max) = (9, 255);
    try_palettes(&mut image, &mut depth, &mut max, &p, 95.0).unwrap();
    assert_eq!(image.nb_meta_channels, 1);
    assert_eq!(image.channel.len(), 2);
    assert_eq!(image.transform.len(), 1);
    assert_eq!(image.transform[0].id, TransformId::PALETTE);
    assert_eq!(image.transform[0].nb_colors, 4);
    image.undo_transforms(&weighted::Header::default());
    assert!(!image.error);
    assert_eq!(samples(&image), original);
}

#[test]
fn quantization_uses_the_plane_and_costs_reject_unrepresentable_weighted_residuals() {
    let mut c = Channel::create(7, 1).unwrap();
    c.row_mut(0)[..7].copy_from_slice(&[-8, -5, -2, 0, 2, 5, 8]);
    c.w = 2;
    c.h = 0;
    quantize_channel(&mut c, 3);
    assert_eq!(&c.plane.row(0)[..7], &[-9, -6, -3, 0, 3, 6, 9]);
    let mut zero = Image::create(11, 7, 8, 3).unwrap();
    for ch in &mut zero.channel {
        for y in 0..ch.h {
            ch.row_mut(y)[..11].fill(0);
        }
    }
    assert_eq!(estimate_cost(&zero).unwrap().to_bits(), 0.0f32.to_bits());
    assert_eq!(estimate_wp_cost(&zero, 0).to_bits(), 0.0f32.to_bits());
    let mut extremes = Image::create(2, 1, 32, 1).unwrap();
    extremes.channel[0].row_mut(0)[..2].copy_from_slice(&[i32::MIN, i32::MAX]);
    assert_eq!(estimate_wp_cost(&extremes, 0).to_bits(), f32::MAX.to_bits());
}
