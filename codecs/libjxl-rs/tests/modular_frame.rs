// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

use jxl::jxl::base::rect::Rect;
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::common::SpeedTier;
use jxl::jxl::dec_modular::ModularStreamId;
use jxl::jxl::enc_ans::Token;
use jxl::jxl::enc_aux_out::{AuxOut, LayerType};
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::enc_modular::{
    GroupParams, ModularFrameEncoder, do_transform, estimate_wp_cost, maybe_do_transform,
    merge_trees, quantize_channel, try_palettes,
};
use jxl::jxl::enc_modular_simd::estimate_cost;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::frame_header::{FrameEncoding, FrameHeader};
use jxl::jxl::image::Image3;
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::encoding::dec_ma::PropertyDecisionNode;
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::options::{Predictor, TreeMode};
use jxl::jxl::modular::transform::transform::{Transform, TransformId};

fn samples(image: &Image) -> Vec<i32> {
    image
        .channel
        .iter()
        .flat_map(|c| (0..c.h).flat_map(|y| c.row(y)[..c.w].iter().copied()))
        .collect()
}
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
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
    jxl::jxl::hwy::select_scalar_decoder_for_test();
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
    jxl::jxl::hwy::select_scalar_decoder_for_test();
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
    image.undo_transforms(&header, None);
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
    jxl::jxl::hwy::select_scalar_decoder_for_test();
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
    image.undo_transforms(&weighted::Header::default(), None);
    assert!(!image.error);
    assert_eq!(samples(&image), original);
}

#[test]
fn quantization_uses_the_plane_and_costs_reject_unrepresentable_weighted_residuals() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
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

fn metadata(w: usize, h: usize, gray: bool) -> CodecMetadata {
    let mut m = CodecMetadata::default();
    m.size.set(w, h).unwrap();
    m.m.color_encoding
        .set_srgb(
            if gray {
                ColorSpace::GRAY
            } else {
                ColorSpace::RGB
            },
            RenderingIntent::RELATIVE,
        )
        .unwrap();
    m
}
fn header(m: &CodecMetadata) -> FrameHeader<'_> {
    let mut h = FrameHeader::new(Some(m));
    h.encoding = FrameEncoding::MODULAR;
    h.group_size_shift = 0;
    h
}

#[test]
fn lossless_thunder_preparation_recovers_integer_pixels_through_the_selected_transforms() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for (gray, depth, bits) in [
        (true, LosslessSampleDepth::Uint8, 8),
        (false, LosslessSampleDepth::Uint8, 8),
        (true, LosslessSampleDepth::Uint16, 16),
        (false, LosslessSampleDepth::Uint16, 16),
    ] {
        let config = LosslessThunderConfig::new(17, 9, gray, depth).unwrap();
        assert_eq!(config.metadata().m.bit_depth.bits_per_sample, bits);
        assert!(!config.metadata().m.xyb_encoded);
        assert_eq!(
            config.metadata().m.modular_16_bit_buffer_sufficient,
            bits <= 12
        );
        let mut frame = config.create_frame().unwrap();
        assert_eq!(frame.header().encoding, FrameEncoding::MODULAR);
        assert!(!frame.header().loop_filter.gab);
        assert_eq!(frame.header().passes.num_passes, 1);
        let maxval = (1u32 << bits) - 1;
        let factor = 1.0f32 / maxval as f32;
        let mut color = Image3::<f32>::create(17, 9).unwrap();
        for c in 0..3 {
            for y in 0..9 {
                for x in 0..17 {
                    let sample = ((x + y * 37 + c * 311) * 997) as u32 & maxval;
                    color.plane_row_mut(c, y)[x] = sample as f32 * factor;
                }
            }
        }
        frame.prepare_modular(&color).unwrap();
        let e = frame.modular();
        assert_eq!(e.cparams.speed_tier, SpeedTier::Thunder);
        assert_eq!(e.cparams.buffering, 0);
        assert_eq!(e.cparams.level, 10);
        assert_eq!(e.cparams.responsive, 0);
        assert_eq!(e.stream_params.len(), 3);
        let mut restored = Image::clone_image(&e.stream_images[0]).unwrap();
        restored.undo_transforms(&weighted::Header::default(), None);
        assert!(!restored.error);
        assert_eq!(restored.channel.len(), if gray { 1 } else { 3 });
        for (c, channel) in restored.channel.iter().enumerate() {
            for y in 0..9 {
                for x in 0..17 {
                    let expected = ((x + y * 37 + c * 311) * 997) as u32 & maxval;
                    assert_eq!(channel.row(y)[x], expected as i32);
                }
            }
        }
    }
}

#[test]
fn lossless_thunder_preparation_releases_global_planes_and_keeps_edge_group_dimensions() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let config = LosslessThunderConfig::new(513, 3, true, LosslessSampleDepth::Uint16).unwrap();
    let mut frame = config.create_frame().unwrap();
    let mut color = Image3::<f32>::create(513, 3).unwrap();
    for y in 0..3 {
        for x in 0..513 {
            color.plane_row_mut(0, y)[x] = ((x + y * 997) * 137 % 65536) as f32 / 65535.0;
        }
    }
    frame.prepare_modular(&color).unwrap();
    let e = frame.modular();
    assert_eq!(e.frame_dim.num_groups, 3);
    let full = &e.stream_images[0].channel[0];
    assert_eq!((full.w, full.h), (513, 3));
    assert_eq!((full.plane.xsize(), full.plane.ysize()), (0, 0));
    for (group, width) in [256, 256, 1].into_iter().enumerate() {
        let stream = ModularStreamId::modular_ac(group, 0).id(&e.frame_dim);
        let channel = &e.stream_images[stream].channel[0];
        assert_eq!((channel.w, channel.h), (width, 3));
        assert_eq!(e.gi_channel[stream], [0]);
    }
}

#[test]
fn lossless_thunder_groups_keep_the_original_zero_gray_section_bytes_and_nonempty_guard() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let config = LosslessThunderConfig::new(1, 1, true, LosslessSampleDepth::Uint8).unwrap();
    let mut frame = config.create_frame().unwrap();
    let color = Image3::<f32>::create(1, 1).unwrap();
    frame.prepare_modular(&color).unwrap();
    frame.compute_modular_tokens().unwrap();
    let mut codes = Vec::new();
    let mut aux = AuxOut::default();
    frame.encode_groups(&mut codes, Some(&mut aux)).unwrap();
    assert_eq!(codes.len(), 1);
    assert_eq!(codes[0].bits_written(), 56);
    // Captured from the original EncodeGroups; this contains no frame/TOC.
    assert_eq!(
        codes[0].get_span(),
        &[0x4b, 0x12, 0xa5, 0x42, 0x85, 0x24, 0x0c]
    );
    assert_eq!(aux.total_bits(), 56);
    assert_eq!(
        frame.encode_groups(&mut codes, Some(&mut aux)),
        Err(StatusCode::GenericError)
    );
    assert_eq!(
        codes[0].get_span(),
        &[0x4b, 0x12, 0xa5, 0x42, 0x85, 0x24, 0x0c]
    );
    assert_eq!(aux.total_bits(), 56);
}

#[test]
fn lossless_thunder_group_writers_follow_dc_global_ac_and_edge_group_layout() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let config = LosslessThunderConfig::new(513, 3, true, LosslessSampleDepth::Uint16).unwrap();
    let mut frame = config.create_frame().unwrap();
    let mut color = Image3::<f32>::create(513, 3).unwrap();
    for y in 0..3 {
        for x in 0..513 {
            color.plane_row_mut(0, y)[x] = ((x + y * 997) * 137 % 65536) as f32 / 65535.0;
        }
    }
    frame.prepare_modular(&color).unwrap();
    frame.compute_modular_tokens().unwrap();
    let mut codes = Vec::new();
    let mut aux = AuxOut::default();
    frame.encode_groups(&mut codes, Some(&mut aux)).unwrap();
    assert_eq!(codes.len(), 6);
    assert!(codes[0].bits_written() > 0);
    assert_eq!(codes[1].bits_written(), 0);
    assert_eq!(codes[2].bits_written(), 0);
    assert!(codes[3..].iter().all(|w| w.bits_written() > 0));
    assert!(codes.iter().all(|w| w.bits_written() % 8 == 0));
    assert_eq!(
        codes.iter().map(BitWriter::bits_written).sum::<usize>(),
        aux.total_bits()
    );
}

#[test]
fn frame_constructor_copies_parameters_and_applies_original_progressive_lossless_policy() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let m = metadata(17, 9, false);
    let h = header(&m);
    let mut p = CompressParams {
        responsive: 1,
        decoding_speed_tier: 1,
        ..Default::default()
    };
    p.ec_distance = vec![0.5, -0.0];
    p.set_lossless();
    p.options.max_properties = 2;
    let mut e = ModularFrameEncoder::create(&h, &p, false).unwrap();
    assert_eq!(e.stream_images.len(), 22);
    assert!(e.stream_images.iter().all(Image::empty));
    assert_eq!(
        (
            e.cparams.responsive,
            e.cparams.decoding_speed_tier,
            e.cparams.colorspace
        ),
        (1, 2, 6)
    );
    assert_eq!(e.cparams.options.wp_tree_mode, TreeMode::NoWP);
    assert_eq!(e.cparams.options.predictor, Predictor::ZERO);
    assert_eq!(
        e.cparams.options.fast_decode_multiplier.to_bits(),
        1.015f32.to_bits()
    );
    assert_eq!(
        e.cparams.options.splitting_heuristics_properties,
        vec![0, 1, 4, 5, 6, 7, 8, 19, 23]
    );
    e.cparams.ec_distance[0] = 2.0;
    assert_eq!(p.ec_distance[0], 0.0);
    assert_eq!(p.decoding_speed_tier, 1);
    assert_eq!(p.colorspace, -1);
}

#[test]
fn frame_group_copy_preserves_crop_shift_selection_and_clear_retains_other_state() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let m = metadata(129, 33, false);
    let h = header(&m);
    let mut p = CompressParams {
        speed_tier: SpeedTier::Thunder,
        colorspace: 0,
        ..Default::default()
    };
    p.set_lossless();
    let mut e = ModularFrameEncoder::create(&h, &p, false).unwrap();
    let mut image = Image::create(129, 33, 16, 0).unwrap();
    image.nb_meta_channels = 1;
    for (c, (w, h, hs, vs)) in [(2, 1, 0, 0), (32, 9, 2, 2), (129, 33, 0, 0), (65, 17, 1, 1)]
        .into_iter()
        .enumerate()
    {
        let mut ch = Channel::create_with_shifts(w, h, hs, vs).unwrap();
        for y in 0..h {
            for x in 0..w {
                ch.row_mut(y)[x] = (c * 1000 + y * 100 + x) as i32;
            }
        }
        image.channel.push(ch);
    }
    e.stream_images[0] = image;
    e.gi_channel.resize_with(e.stream_images.len(), Vec::new);
    e.stream_options[0].wp_mode = 4;
    e.prepare_stream_params(
        &Rect::new(0, 0, 129, 33),
        0,
        1000,
        &ModularStreamId::global(),
        true,
        false,
    )
    .unwrap();
    assert_eq!(e.stream_options[0].wp_mode, 4);
    assert!(e.stream_images[0].transform.is_empty());
    let id = ModularStreamId::modular_ac(1, 0);
    let s = id.id(&e.frame_dim);
    let rect = Rect::new(128, 0, 1, 33);
    e.prepare_stream_params(&rect, 0, 1, &id, true, false)
        .unwrap();
    assert_eq!(e.gi_channel[s], vec![2]);
    assert_eq!(e.stream_images[s].channel.len(), 1);
    let ch = &e.stream_images[s].channel[0];
    assert_eq!(
        (ch.w, ch.h, ch.hshift, ch.vshift, ch.component),
        (1, 33, 0, 0, -1)
    );
    assert_eq!(ch.row(0)[0], 2128);
    assert_eq!(ch.row(32)[0], 5328);
    e.tokens = vec![vec![Token::new(3, 29)]];
    e.stream_params.push(GroupParams {
        rect,
        min_shift: 0,
        max_shift: 1,
        id,
    });
    e.clear_modular_stream_data();
    assert!(e.stream_params.is_empty());
    assert!(e.stream_images[s].channel.is_empty());
    assert!(e.stream_images[s].error);
    assert_eq!(e.gi_channel[s], vec![2]);
    assert_eq!(e.tokens[0][0].value, 29);
    assert_eq!(e.stream_images[0].channel.len(), 4);
}

#[test]
fn frame_token_error_retains_written_prefix_header_and_previous_width() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let m = metadata(3, 1, true);
    let h = header(&m);
    let mut p = CompressParams {
        speed_tier: SpeedTier::Lightning,
        ..Default::default()
    };
    p.set_lossless();
    let mut e = ModularFrameEncoder::create(&h, &p, false).unwrap();
    e.stream_images[0] = Image::create(3, 1, 32, 1).unwrap();
    e.stream_images[0].channel[0].row_mut(0)[..3].copy_from_slice(&[i32::MIN, i32::MAX, i32::MIN]);
    e.image_widths = vec![99; e.stream_images.len()];
    e.compute_tree().unwrap();
    assert_eq!(e.compute_tokens(), Err(StatusCode::GenericError));
    assert_eq!(e.tokens[0].len(), 3);
    assert_eq!(e.tokens[0][0].value, u32::MAX);
    assert_eq!(e.image_widths[0], 99);
    assert!(e.stream_headers[0].use_global_tree);
    assert_eq!(e.tree.len(), 1);
}

#[test]
fn frame_global_modular_sections_reproduce_independently_captured_cpp_zero_gray_bytes() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    // Original pinned ModularFrameEncoder, scalar Clang, 1x1 zero gray,
    // lossless TectonicPlate, group shift 0. These are modular sections,
    // without image/frame headers or a JPEG XL section table.
    let m = metadata(1, 1, true);
    let h = header(&m);
    let mut p = CompressParams {
        speed_tier: SpeedTier::TectonicPlate,
        ..Default::default()
    };
    p.set_lossless();
    let mut e = ModularFrameEncoder::create(&h, &p, false).unwrap();
    e.stream_images[0] = Image::create(1, 1, 8, 1).unwrap();
    e.stream_images[0].channel[0].row_mut(0)[0] = 0;
    e.gi_channel.resize_with(e.stream_images.len(), Vec::new);
    let rect = Rect::new(0, 0, 1, 1);
    let global = ModularStreamId::global();
    e.prepare_stream_params(&rect, -1, 30, &global, true, false)
        .unwrap();
    let ac = ModularStreamId::modular_ac(0, 0);
    let s = ac.id(&e.frame_dim);
    e.stream_options[s] = e.stream_options[0].clone();
    e.prepare_stream_params(&rect, -1, 30, &ac, true, false)
        .unwrap();
    for _ in 0..2 {
        e.compute_tree().unwrap();
        e.compute_tokens().unwrap();
    }
    let mut writer = BitWriter::new();
    e.encode_global_info(false, &mut writer, None).unwrap();
    e.encode_stream(&mut writer, None, LayerType::ModularGlobal, &global)
        .unwrap();
    assert_eq!(writer.bits_written(), 42);
    writer
        .with_max_bits(
            7,
            LayerType::Header,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    assert_eq!(writer.get_span(), &[0x25, 0x8c, 0xc5, 0x0a, 0xc1, 0x00]);
}
