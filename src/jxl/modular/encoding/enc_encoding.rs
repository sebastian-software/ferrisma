// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Given-tree residual encoding and predefined trees from modular/encoding/
//! enc_encoding.cc, including tree learning and ModularGenericCompress.
//! Token-pointer positions become indices into the caller's resized storage.
//! Preserve kWantDebug=true's predictor-image allocation and writes, even though
//! its commented-out diagnostic output is not exposed by the source.

use super::context_predict::*;
use super::dec_ma::{PropertyDecisionNode as Node, Tree};
use super::enc_ma::{TreeSamples, collect_pixel_samples, compute_best_tree, tokenize_tree};
use super::encoding::{GroupHeader, PROP_RANGE_FAST, TreeLut, filter_tree, tree_to_lookup_table};
use super::ma_common::NUM_TREE_CONTEXTS;
use crate::jxl::base::bits::{ceil_log2_nonzero, floor_log2_nonzero};
use crate::jxl::base::float::{max_f64, min_f32, mul_add_f64};
use crate::jxl::base::random::Rng;
use crate::jxl::base::status::{Status, StatusCode, StatusOr};
use crate::jxl::enc_ans::{
    EntropyEncodingData, Token, build_and_encode_histograms, write_tokens_with_allotment,
};
use crate::jxl::enc_aux_out::{AuxOut, LayerType};
use crate::jxl::enc_bit_writer::BitWriter;
use crate::jxl::fields::bundle;
use crate::jxl::image::Image3F;
use crate::jxl::image_ops::fill_image;
use crate::jxl::modular::modular_image::{Channel, Image};
use crate::jxl::modular::options::{
    ModularMultiplierInfo, ModularOptions, NUM_MODULAR_PREDICTORS, Predictor, StaticPropRange,
    TreeKind, UNDEFINED_PREDICTOR,
};
use crate::jxl::pack_signed::pack_signed;
use std::collections::VecDeque;

fn predictor_color(p: Predictor) -> [u8; 3] {
    match p {
        Predictor::ZERO => [0, 0, 0],
        Predictor::LEFT => [255, 0, 0],
        Predictor::TOP => [0, 255, 0],
        Predictor::AVERAGE0 => [0, 0, 255],
        Predictor::AVERAGE4 => [192, 128, 128],
        Predictor::SELECT => [255, 255, 0],
        Predictor::GRADIENT => [255, 0, 255],
        Predictor::WEIGHTED => [0, 255, 255],
        _ => [255, 255, 255],
    }
}
fn make_fixed_tree(
    property: i32,
    cutoffs: &[i32],
    pred: Predictor,
    num_pixels: usize,
    bitdepth: i32,
) -> Tree {
    let log_px = ceil_log2_nonzero(num_pixels);
    let mut min_gap = 0;
    if log_px < 14 {
        min_gap = 8 * (14 - log_px);
    }
    let shift = if bitdepth > 11 {
        (bitdepth - 11).min(4)
    } else {
        0
    };
    let mul = 1 << shift;
    struct NodeInfo {
        begin: usize,
        end: usize,
        pos: usize,
    }
    let mut tree = vec![Node::leaf(pred, 0, 1)];
    let mut q = VecDeque::from([NodeInfo {
        begin: 0,
        end: cutoffs.len(),
        pos: 0,
    }]);
    while let Some(info) = q.pop_front() {
        if info.begin + min_gap >= info.end {
            continue;
        }
        let split = ((info.begin + info.end) / 2) as u32;
        let cutoff = cutoffs[split as usize] * mul;
        tree[info.pos] = Node::split(property, cutoff, tree.len() as i32, -1);
        q.push_back(NodeInfo {
            begin: split as usize + 1,
            end: info.end,
            pos: tree.len(),
        });
        tree.push(Node::leaf(pred, 0, 1));
        q.push_back(NodeInfo {
            begin: info.begin,
            end: split as usize,
            pos: tree.len(),
        });
        tree.push(Node::leaf(pred, 0, 1));
    }
    tree
}
pub fn predefined_tree(kind: TreeKind, total_pixels: usize, bitdepth: i32, prevprop: i32) -> Tree {
    match kind {
        TreeKind::JpegTranscodeACMeta | TreeKind::TrivialTreeNoPredictor => {
            vec![Node::leaf(Predictor::ZERO, 0, 1)]
        }
        TreeKind::FalconACMeta => vec![Node::leaf(Predictor::LEFT, 0, 1)],
        TreeKind::ACMeta => {
            if total_pixels < 1024 {
                return vec![Node::leaf(Predictor::LEFT, 0, 1)];
            }
            vec![
                Node::split(0, 1, 1, -1),
                Node::split(0, 2, 3, -1),
                Node::split(0, 0, 5, -1),
                Node::split(6, 3, 21, -1),
                Node::split(2, 0, 7, -1),
                Node::leaf(Predictor::GRADIENT, 0, 1),
                Node::leaf(Predictor::GRADIENT, 0, 1),
                Node::split(7, 5, 9, -1),
                Node::split(7, 5, 15, -1),
                Node::split(7, 11, 11, -1),
                Node::split(7, 3, 13, -1),
                Node::leaf(Predictor::LEFT, 0, 1),
                Node::leaf(Predictor::LEFT, 0, 1),
                Node::leaf(Predictor::LEFT, 0, 1),
                Node::leaf(Predictor::LEFT, 0, 1),
                Node::split(7, 11, 17, -1),
                Node::split(7, 3, 19, -1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::split(7, 3, 23, -1),
                Node::split(7, 3, 25, -1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::leaf(Predictor::ZERO, 0, 1),
                Node::leaf(Predictor::ZERO, 0, 1),
            ]
        }
        TreeKind::WPFixedDC => {
            let cutoffs = [
                -500, -392, -255, -191, -127, -95, -63, -47, -31, -23, -15, -11, -7, -4, -3, -1, 0,
                1, 3, 5, 7, 11, 15, 23, 31, 47, 63, 95, 127, 191, 255, 392, 500,
            ];
            make_fixed_tree(
                WP_PROP as i32,
                &cutoffs,
                Predictor::WEIGHTED,
                total_pixels,
                bitdepth,
            )
        }
        TreeKind::GradientFixedDC => {
            let cutoffs = [
                -500, -392, -255, -191, -127, -95, -63, -47, -31, -23, -15, -11, -7, -4, -3, -1, 0,
                1, 3, 5, 7, 11, 15, 23, 31, 47, 63, 95, 127, 191, 255, 392, 500,
            ];
            make_fixed_tree(
                if prevprop > 0 {
                    (NUM_NONREF_PROPERTIES + 2) as i32
                } else {
                    GRADIENT_PROP as i32
                },
                &cutoffs,
                Predictor::GRADIENT,
                total_pixels,
                bitdepth,
            )
        }
        // PredefinedTree rejects learning as a programmer error; the release
        // reference's disabled JXL_DEBUG_ABORT then returns an empty tree.
        TreeKind::Learn => Vec::new(),
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source image/tree/token boundaries"
)]
fn encode_modular_channel_maans(
    image: &Image,
    chan: i32,
    wp_header: &weighted::Header,
    global_tree: &Tree,
    tokens: &mut [Token],
    tokenpp: &mut usize,
    group_id: usize,
    skip_encoder_fast_path: bool,
) -> Status {
    let channel = &image.channel[chan as usize];
    let mut tokenp = *tokenpp;
    if channel.w == 0 || channel.h == 0 {
        return Err(StatusCode::GenericError);
    }
    let mut predictor_img = Image3F::create(channel.w, channel.h)?;
    let mut static_props = [chan, group_id as i32];
    let (mut use_wp, mut is_wp_only, mut is_gradient_only) = (false, false, false);
    let mut num_props = 0;
    let tree = filter_tree(
        global_tree,
        &mut static_props,
        &mut num_props,
        &mut use_wp,
        &mut is_wp_only,
        &mut is_gradient_only,
    );
    let tree_lookup = MATreeLookup::new(&tree);
    let mut tree_lut = Box::new(TreeLut::<u16, false, false>::default());
    if is_wp_only {
        is_wp_only = tree_to_lookup_table(&tree, &mut tree_lut);
    }
    if is_gradient_only {
        is_gradient_only = tree_to_lookup_table(&tree, &mut tree_lut);
    }
    if is_wp_only && !skip_encoder_fast_path {
        for (c, color) in predictor_color(Predictor::WEIGHTED).into_iter().enumerate() {
            fill_image(f32::from(color), predictor_img.plane_mut(c));
        }
        let onerow = channel.plane.pixels_per_row();
        let mut wp_state = weighted::State::new(wp_header, channel.w, channel.h);
        let mut properties = vec![0; 1];
        let mut unhealthy = false;
        for y in 0..channel.h {
            let r = channel.row(y);
            for x in 0..channel.w {
                let p = PixelPosition::new(
                    channel.plane.pixels().unwrap(),
                    y * onerow as usize + x,
                    onerow,
                );
                let left = i64::from(if x != 0 {
                    r[x - 1]
                } else if y != 0 {
                    p.read_pixel(-onerow)
                } else {
                    0
                });
                let top = if y != 0 {
                    i64::from(p.read_pixel(-onerow))
                } else {
                    left
                };
                let topleft = if x != 0 && y != 0 {
                    i64::from(p.read_pixel(-1 - onerow))
                } else {
                    left
                };
                let topright = if x + 1 < channel.w && y != 0 {
                    i64::from(p.read_pixel(1 - onerow))
                } else {
                    top
                };
                let toptop = if y > 1 {
                    i64::from(p.read_pixel(-onerow - onerow))
                } else {
                    top
                };
                let guess = wp_state.predict::<true>(
                    x,
                    y,
                    channel.w,
                    top,
                    left,
                    topright,
                    topleft,
                    toptop,
                    Some(&mut properties),
                    0,
                ) as i32;
                let pos = (PROP_RANGE_FAST
                    + properties[0].clamp(-PROP_RANGE_FAST, PROP_RANGE_FAST - 1))
                    as u32;
                let ctx_id = u32::from(tree_lut.context_lookup[pos as usize]);
                let (residual, overflow) = r[x].overflowing_sub(guess);
                unhealthy |= overflow;
                tokens[tokenp] = Token::new(ctx_id, pack_signed(residual));
                tokenp += 1;
                wp_state.update_errors(i64::from(r[x]), x, y, channel.w);
            }
        }
        if unhealthy {
            return Err(StatusCode::GenericError);
        }
    } else if tree.len() == 1
        && tree[0].predictor() == Predictor::GRADIENT
        && tree[0].multiplier() == 1
        && tree[0].predictor_offset() == 0
        && !skip_encoder_fast_path
    {
        for (c, color) in predictor_color(Predictor::GRADIENT).into_iter().enumerate() {
            fill_image(f32::from(color), predictor_img.plane_mut(c));
        }
        let onerow = channel.plane.pixels_per_row();
        let mut unhealthy = false;
        for y in 0..channel.h {
            let r = channel.row(y);
            for x in 0..channel.w {
                let p = PixelPosition::new(
                    channel.plane.pixels().unwrap(),
                    y * onerow as usize + x,
                    onerow,
                );
                let left = if x != 0 {
                    r[x - 1]
                } else if y != 0 {
                    p.read_pixel(-onerow)
                } else {
                    0
                };
                let top = if y != 0 { p.read_pixel(-onerow) } else { left };
                let topleft = if x != 0 && y != 0 {
                    p.read_pixel(-1 - onerow)
                } else {
                    left
                };
                let guess = clamped_gradient(top, left, topleft);
                let (residual, overflow) = r[x].overflowing_sub(guess);
                unhealthy |= overflow;
                tokens[tokenp] = Token::new(tree[0].child_id, pack_signed(residual));
                tokenp += 1;
            }
        }
        if unhealthy {
            return Err(StatusCode::GenericError);
        }
    } else if is_gradient_only && !skip_encoder_fast_path {
        for (c, color) in predictor_color(Predictor::GRADIENT).into_iter().enumerate() {
            fill_image(f32::from(color), predictor_img.plane_mut(c));
        }
        let onerow = channel.plane.pixels_per_row();
        let mut unhealthy = false;
        for y in 0..channel.h {
            let r = channel.row(y);
            for x in 0..channel.w {
                let p = PixelPosition::new(
                    channel.plane.pixels().unwrap(),
                    y * onerow as usize + x,
                    onerow,
                );
                let left = i64::from(if x != 0 {
                    r[x - 1]
                } else if y != 0 {
                    p.read_pixel(-onerow)
                } else {
                    0
                });
                let top = if y != 0 {
                    i64::from(p.read_pixel(-onerow))
                } else {
                    left
                };
                let topleft = if x != 0 && y != 0 {
                    i64::from(p.read_pixel(-1 - onerow))
                } else {
                    left
                };
                let guess = clamped_gradient(top as i32, left as i32, topleft as i32);
                let pos = (i64::from(PROP_RANGE_FAST)
                    + (top + left - topleft)
                        .clamp(i64::from(-PROP_RANGE_FAST), i64::from(PROP_RANGE_FAST - 1)))
                    as u32;
                let ctx_id = u32::from(tree_lut.context_lookup[pos as usize]);
                let (residual, overflow) = r[x].overflowing_sub(guess);
                unhealthy |= overflow;
                tokens[tokenp] = Token::new(ctx_id, pack_signed(residual));
                tokenp += 1;
            }
        }
        if unhealthy {
            return Err(StatusCode::GenericError);
        }
    } else if tree.len() == 1
        && tree[0].predictor() == Predictor::ZERO
        && tree[0].multiplier() == 1
        && tree[0].predictor_offset() == 0
        && !skip_encoder_fast_path
    {
        for (c, color) in predictor_color(Predictor::ZERO).into_iter().enumerate() {
            fill_image(f32::from(color), predictor_img.plane_mut(c));
        }
        for y in 0..channel.h {
            for &v in &channel.row(y)[..channel.w] {
                tokens[tokenp] = Token::new(tree[0].child_id, pack_signed(v));
                tokenp += 1;
            }
        }
    } else if tree.len() == 1
        && tree[0].predictor() != Predictor::WEIGHTED
        && (tree[0].multiplier() & tree[0].multiplier().wrapping_sub(1)) == 0
        && tree[0].predictor_offset() == 0
        && !skip_encoder_fast_path
    {
        for (c, color) in predictor_color(tree[0].predictor()).into_iter().enumerate() {
            fill_image(f32::from(color), predictor_img.plane_mut(c));
        }
        let mul_shift = floor_log2_nonzero(tree[0].multiplier() as u32);
        let onerow = channel.plane.pixels_per_row();
        for y in 0..channel.h {
            let r = channel.row(y);
            for (x, &v) in r.iter().take(channel.w).enumerate() {
                let p = PixelPosition::new(
                    channel.plane.pixels().unwrap(),
                    y * onerow as usize + x,
                    onerow,
                );
                let pred =
                    predict_no_tree_no_wp(channel.w, p, x as i32, y as i32, tree[0].predictor());
                let residual = i64::from(v) - pred.guess;
                tokens[tokenp] = Token::new(
                    tree[0].child_id,
                    pack_signed((residual >> mul_shift) as i32),
                );
                tokenp += 1;
            }
        }
    } else if !use_wp && !skip_encoder_fast_path {
        let onerow = channel.plane.pixels_per_row();
        let mut properties = vec![0; num_props];
        let mut references = Channel::create(properties.len() - NUM_NONREF_PROPERTIES, channel.w)?;
        for y in 0..channel.h {
            let p = channel.row(y);
            precompute_references(channel, y, image, chan as u32, &mut references);
            init_props_row(&mut properties, &static_props, y as i32);
            for (x, &v) in p.iter().take(channel.w).enumerate() {
                let pp = PixelPosition::new(
                    channel.plane.pixels().unwrap(),
                    y * onerow as usize + x,
                    onerow,
                );
                let res = predict_tree_no_wp(
                    &mut properties,
                    channel.w,
                    pp,
                    x as i32,
                    y as i32,
                    &tree_lookup,
                    &references,
                );
                for (i, color) in predictor_color(res.predictor).into_iter().enumerate() {
                    predictor_img.plane_row_mut(i, y)[x] = f32::from(color);
                }
                let residual = i64::from(v) - res.guess;
                tokens[tokenp] = Token::new(
                    res.context as u32,
                    pack_signed((residual / i64::from(res.multiplier)) as i32),
                );
                tokenp += 1;
            }
        }
    } else {
        let onerow = channel.plane.pixels_per_row();
        let mut properties = vec![0; num_props];
        let mut references = Channel::create(properties.len() - NUM_NONREF_PROPERTIES, channel.w)?;
        let mut wp_state = weighted::State::new(wp_header, channel.w, channel.h);
        for y in 0..channel.h {
            let p = channel.row(y);
            precompute_references(channel, y, image, chan as u32, &mut references);
            init_props_row(&mut properties, &static_props, y as i32);
            for (x, &v) in p.iter().take(channel.w).enumerate() {
                let pp = PixelPosition::new(
                    channel.plane.pixels().unwrap(),
                    y * onerow as usize + x,
                    onerow,
                );
                let res = predict_tree_wp(
                    &mut properties,
                    channel.w,
                    pp,
                    x as i32,
                    y as i32,
                    &tree_lookup,
                    &references,
                    &mut wp_state,
                );
                for (i, color) in predictor_color(res.predictor).into_iter().enumerate() {
                    predictor_img.plane_row_mut(i, y)[x] = f32::from(color);
                }
                let residual = i64::from(v) - res.guess;
                tokens[tokenp] = Token::new(
                    res.context as u32,
                    pack_signed((residual / i64::from(res.multiplier)) as i32),
                );
                tokenp += 1;
                wp_state.update_errors(i64::from(v), x, y, channel.w);
            }
        }
    }
    *tokenpp = tokenp;
    Ok(())
}
pub fn modular_compress(
    image: &Image,
    options: &ModularOptions,
    group_id: usize,
    tree: &Tree,
    header: &mut GroupHeader,
    tokens: &mut Vec<Token>,
    width: &mut usize,
) -> Status {
    let nb_channels = image.channel.len();
    if image.w == 0 || image.h == 0 || nb_channels < 1 {
        return Ok(());
    }
    if image.error {
        return Err(StatusCode::GenericError);
    }
    bundle::init(header);
    if predictor_has_weighted(options.predictor) {
        weighted::predictor_mode(options.wp_mode, &mut header.wp_header);
    }
    header.transforms = image.transform.clone();
    header.use_global_tree = true;
    let mut image_width = 0;
    let mut total_tokens = 0;
    for (i, c) in image.channel.iter().enumerate() {
        if i >= image.nb_meta_channels
            && (c.w > options.max_chan_size || c.h > options.max_chan_size)
        {
            break;
        }
        if c.w > image_width {
            image_width = c.w;
        }
        total_tokens += c.w * c.h;
    }
    if options.zero_tokens {
        tokens.resize(tokens.len() + total_tokens, Token::new(0, 0));
    } else {
        let pos = tokens.len();
        tokens.resize(pos + total_tokens, Token::default());
        let mut tokenp = pos;
        for (i, c) in image.channel.iter().enumerate() {
            if i >= image.nb_meta_channels
                && (c.w > options.max_chan_size || c.h > options.max_chan_size)
            {
                break;
            }
            if c.w == 0 || c.h == 0 {
                continue;
            }
            encode_modular_channel_maans(
                image,
                i as i32,
                &header.wp_header,
                tree,
                tokens,
                &mut tokenp,
                group_id,
                options.skip_encoder_fast_path,
            )?;
        }
        if tokenp != tokens.len() {
            return Err(StatusCode::GenericError);
        }
    }
    *width = image_width;
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source learning state, sample decision, and pixel inputs"
)]
fn gather_sample<const NEC: bool>(
    properties: &mut Vec<i32>,
    channel: &Channel,
    x: usize,
    y: usize,
    references: &Channel,
    wp_state: &mut weighted::State<'_>,
    samples: &mut TreeSamples,
    total_pixels: &mut usize,
    rng: &mut Rng,
    threshold: u64,
    multiple_predictors: bool,
) {
    let onerow = channel.plane.pixels_per_row();
    let pp = PixelPosition::new(
        channel.plane.pixels().unwrap(),
        y * onerow as usize + x,
        onerow,
    );
    let mut predictions = [0i64; NUM_MODULAR_PREDICTORS];
    if multiple_predictors {
        if NEC {
            predict_learn_all_nec(
                properties,
                channel.w,
                pp,
                x as i32,
                y as i32,
                references,
                wp_state,
                &mut predictions,
            );
        } else {
            predict_learn_all(
                properties,
                channel.w,
                pp,
                x as i32,
                y as i32,
                references,
                wp_state,
                &mut predictions,
            );
        }
    } else {
        let pred = samples.predictor_from_index(0);
        predictions[pred.0 as usize] = if NEC {
            predict_learn_nec(
                properties, channel.w, pp, x as i32, y as i32, pred, references, wp_state,
            )
            .guess
        } else {
            predict_learn(
                properties, channel.w, pp, x as i32, y as i32, pred, references, wp_state,
            )
            .guess
        };
    }
    *total_pixels = total_pixels.wrapping_add(1);
    if rng.next_u64() >> 32 <= threshold {
        samples.add_sample(i64::from(channel.row(y)[x]), properties, &predictions);
    }
    wp_state.update_errors(i64::from(channel.row(y)[x]), x, y, channel.w);
}
fn gather_tree_data(
    image: &Image,
    chan: i32,
    group_id: usize,
    wp_header: &weighted::Header,
    options: &ModularOptions,
    samples: &mut TreeSamples,
    total_pixels: &mut usize,
) -> Status {
    let channel = &image.channel[chan as usize];
    let static_props = [chan, group_id as i32];
    let mut properties = vec![
        0;
        NUM_NONREF_PROPERTIES.wrapping_add(
            EXTRA_PROPS_PER_CHANNEL.wrapping_mul(options.max_properties as usize)
        )
    ];
    let mut fraction = f64::from(min_f32(1.0, options.nb_repeats));
    if fraction > 0.0 {
        let ratio = 1024.0 / channel.w.wrapping_mul(channel.h) as f64;
        let bound = if ratio < 1.0 { ratio } else { 1.0 };
        fraction = max_f64(fraction, bound);
    }
    let threshold = ((u64::MAX >> 32) as f64 * fraction) as u64;
    // The source's local use_sample lambda has precisely Rng(0)'s two words.
    let mut rng = Rng::new(0);
    let mut references = Channel::create(
        properties.len().wrapping_sub(NUM_NONREF_PROPERTIES),
        channel.w,
    )?;
    let mut wp_state = weighted::State::new(wp_header, channel.w, channel.h);
    let reserve = mul_add_f64(fraction * channel.h as f64, channel.w as f64, 64.0) as usize;
    samples.prepare_for_samples(reserve);
    let multiple_predictors = samples.num_predictors() != 1;
    for y in 0..channel.h {
        precompute_references(channel, y, image, chan as u32, &mut references);
        init_props_row(&mut properties, &static_props, y as i32);
        if y > 1 && channel.w > 8 && references.w == 0 {
            for x in 0..2 {
                gather_sample::<false>(
                    &mut properties,
                    channel,
                    x,
                    y,
                    &references,
                    &mut wp_state,
                    samples,
                    total_pixels,
                    &mut rng,
                    threshold,
                    multiple_predictors,
                );
            }
            for x in 2..channel.w - 2 {
                gather_sample::<true>(
                    &mut properties,
                    channel,
                    x,
                    y,
                    &references,
                    &mut wp_state,
                    samples,
                    total_pixels,
                    &mut rng,
                    threshold,
                    multiple_predictors,
                );
            }
            for x in channel.w - 2..channel.w {
                gather_sample::<false>(
                    &mut properties,
                    channel,
                    x,
                    y,
                    &references,
                    &mut wp_state,
                    samples,
                    total_pixels,
                    &mut rng,
                    threshold,
                    multiple_predictors,
                );
            }
        } else {
            for x in 0..channel.w {
                gather_sample::<false>(
                    &mut properties,
                    channel,
                    x,
                    y,
                    &references,
                    &mut wp_state,
                    samples,
                    total_pixels,
                    &mut rng,
                    threshold,
                    multiple_predictors,
                );
            }
        }
    }
    Ok(())
}
fn learn_tree_from_samples(
    mut samples: TreeSamples,
    total_pixels: usize,
    options: &ModularOptions,
    multiplier_info: &[ModularMultiplierInfo],
    mut range: StaticPropRange,
) -> StatusOr<Tree> {
    let mut tree = Vec::new();
    for r in &mut range {
        if r[1] == 0 {
            r[1] = u32::MAX;
        }
    }
    if !samples.has_samples() {
        tree.push(Node::leaf(samples.predictor_from_index(0), 0, 1));
        return Ok(tree);
    }
    let fraction = samples.num_samples() as f32 * 1.0 / total_pixels as f32;
    let required_cost = mul_add_f64(f64::from(fraction), 0.9, 0.1) as f32;
    samples.all_samples_done();
    compute_best_tree(
        &mut samples,
        options.splitting_heuristics_node_threshold * required_cost,
        multiplier_info,
        range,
        options.fast_decode_multiplier,
        &mut tree,
    )?;
    Ok(tree)
}
pub fn learn_tree(
    images: &[Image],
    options: &[ModularOptions],
    start: u32,
    stop: u32,
    multiplier_info: &[ModularMultiplierInfo],
) -> StatusOr<Tree> {
    let mut samples = TreeSamples::default();
    samples.set_predictor(
        options[start as usize].predictor,
        options[start as usize].wp_tree_mode,
    )?;
    samples.set_properties(
        &options[start as usize].splitting_heuristics_properties,
        options[start as usize].wp_tree_mode,
    )?;
    let mut max_c = 0u32;
    let mut pixel_samples = Vec::new();
    let mut diff_samples = Vec::new();
    let mut group_pixel_count = Vec::new();
    let mut channel_pixel_count = Vec::new();
    for i in start..stop {
        max_c = max_c.max(images[i as usize].channel.len() as u32);
        collect_pixel_samples(
            &images[i as usize],
            &options[i as usize],
            i,
            &mut group_pixel_count,
            &mut channel_pixel_count,
            &mut pixel_samples,
            &mut diff_samples,
        );
    }
    let range = [[0, max_c], [start, stop]];
    samples.pre_quantize_properties(
        &range,
        multiplier_info,
        &group_pixel_count,
        &channel_pixel_count,
        &mut pixel_samples,
        &mut diff_samples,
        options[start as usize].max_property_values,
    );
    let mut total_pixels = 0usize;
    for (i, ch) in images[start as usize].channel.iter().enumerate() {
        if i >= images[start as usize].nb_meta_channels
            && (ch.w > options[start as usize].max_chan_size
                || ch.h > options[start as usize].max_chan_size)
        {
            break;
        }
        total_pixels = total_pixels.wrapping_add(ch.w.wrapping_mul(ch.h));
    }
    total_pixels = total_pixels.max(1);
    let mut wp_header = weighted::Header::default();
    for i in start as usize..stop as usize {
        let image = &images[i];
        let opts = &options[i];
        let nb_channels = image.channel.len();
        if image.w == 0 || image.h == 0 || nb_channels < 1 {
            continue;
        }
        if image.error {
            return Err(StatusCode::GenericError);
        }
        if opts.tree_kind != TreeKind::Learn {
            return Err(StatusCode::GenericError);
        }
        bundle::init(&mut wp_header);
        if predictor_has_weighted(opts.predictor) {
            weighted::predictor_mode(opts.wp_mode, &mut wp_header);
        }
        for (c, ch) in image.channel.iter().enumerate() {
            if c >= image.nb_meta_channels
                && (ch.w > opts.max_chan_size || ch.h > opts.max_chan_size)
            {
                break;
            }
            if ch.w == 0 || ch.h == 0 {
                continue;
            }
            gather_tree_data(
                image,
                c as i32,
                i,
                &wp_header,
                opts,
                &mut samples,
                &mut total_pixels,
            )?;
        }
    }
    learn_tree_from_samples(
        samples,
        total_pixels,
        &options[start as usize],
        multiplier_info,
        range,
    )
}
pub fn modular_generic_compress(
    image: &Image,
    opts: &ModularOptions,
    writer: &mut BitWriter,
    mut aux: Option<&mut AuxOut>,
    layer: LayerType,
    group_id: usize,
) -> Status {
    let nb_channels = image.channel.len();
    if image.w == 0 || image.h == 0 || nb_channels < 1 {
        return Ok(());
    }
    if image.error {
        return Err(StatusCode::GenericError);
    }
    let mut options = opts.clone();
    if options.predictor == UNDEFINED_PREDICTOR {
        options.predictor = Predictor::GRADIENT;
    }
    let _bits = writer.bits_written();
    let mut header = GroupHeader::default();
    bundle::init(&mut header);
    if predictor_has_weighted(options.predictor) {
        weighted::predictor_mode(options.wp_mode, &mut header.wp_header);
    }
    header.transforms = image.transform.clone();
    bundle::write(&mut header, writer, layer, aux.as_deref_mut())?;
    let mut tree = if options.tree_kind == TreeKind::Learn {
        learn_tree(
            std::slice::from_ref(image),
            std::slice::from_ref(&options),
            0,
            1,
            &[],
        )?
    } else {
        let mut total_pixels = 0usize;
        for (i, ch) in image.channel.iter().enumerate() {
            if i >= image.nb_meta_channels
                && (ch.w > options.max_chan_size || ch.h > options.max_chan_size)
            {
                break;
            }
            total_pixels = total_pixels.wrapping_add(ch.w.wrapping_mul(ch.h));
        }
        total_pixels = total_pixels.max(1);
        predefined_tree(
            options.tree_kind,
            total_pixels,
            image.bitdepth,
            options.max_properties,
        )
    };
    let mut decoded_tree = Vec::new();
    let mut tree_tokens = vec![Vec::new()];
    tokenize_tree(&tree, &mut tree_tokens[0], &mut decoded_tree)?;
    if tree.len() != decoded_tree.len() {
        return Err(StatusCode::GenericError);
    }
    tree = decoded_tree;
    let mut code = EntropyEncodingData::default();
    build_and_encode_histograms(
        &options.histogram_params,
        NUM_TREE_CONTEXTS,
        &mut tree_tokens,
        &mut code,
        Some(writer),
        LayerType::ModularTree,
        aux.as_deref_mut(),
    )?;
    write_tokens_with_allotment(
        &tree_tokens[0],
        &code,
        0,
        writer,
        LayerType::ModularTree,
        aux.as_deref_mut(),
    )?;
    let mut image_width = 0;
    let mut tokens = vec![Vec::new()];
    modular_compress(
        image,
        &options,
        group_id,
        &tree,
        &mut header,
        &mut tokens[0],
        &mut image_width,
    )?;
    code = EntropyEncodingData::default();
    let mut params = options.histogram_params.clone();
    params.image_widths.push(image_width);
    build_and_encode_histograms(
        &params,
        tree.len().div_ceil(2),
        &mut tokens,
        &mut code,
        Some(writer),
        layer,
        aux.as_deref_mut(),
    )?;
    write_tokens_with_allotment(&tokens[0], &code, 0, writer, layer, aux)?;
    Ok(())
}
