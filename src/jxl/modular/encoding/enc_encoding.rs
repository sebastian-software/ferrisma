// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Given-tree residual encoding and predefined trees from modular/encoding/
//! enc_encoding.cc. Learning and ModularGenericCompress remain absent.
//! Token-pointer positions become indices into the caller's resized storage.
//! Preserve kWantDebug=true's predictor-image allocation and writes, even though
//! its commented-out diagnostic output is not exposed by the source.

use super::context_predict::*;
use super::dec_ma::{PropertyDecisionNode as Node, Tree};
use super::encoding::{GroupHeader, PROP_RANGE_FAST, TreeLut, filter_tree, tree_to_lookup_table};
use crate::jxl::base::bits::{ceil_log2_nonzero, floor_log2_nonzero};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::enc_ans::Token;
use crate::jxl::fields::bundle;
use crate::jxl::image::Image3F;
use crate::jxl::image_ops::fill_image;
use crate::jxl::modular::modular_image::{Channel, Image};
use crate::jxl::modular::options::{ModularOptions, Predictor, TreeKind};
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
