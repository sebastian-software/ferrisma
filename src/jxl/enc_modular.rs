// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Sample conversion and transform/tree helpers from lib/jxl/enc_modular.cc.
//! Frame encoding is unported. The transform helpers use the null-pool path.
//! Disjoint slices replace the source's row pointers. Source-undefined floating
//! conversions and shifts remain caller preconditions, checked with assertions.
//! A failed custom-float conversion retains its already written output prefix.

use super::base::bits::ceil_log2_nonzero;
use super::base::float::{mul_add_f32, mul_add_f64};
use super::base::status::{Status, StatusCode, StatusOr};
use super::common::SpeedTier;
use super::dec_ans::HybridUintConfig;
use super::enc_ans_params::Histogram;
use super::enc_modular_simd::estimate_cost;
use super::enc_params::CompressParams;
use super::modular::encoding::context_predict::weighted;
use super::modular::encoding::dec_ma::{PropertyDecisionNode, Tree};
use super::modular::modular_image::{Channel, Image};
use super::modular::options::Predictor;
use super::modular::transform::enc_transform::{compute_minmax, transform_forward};
use super::modular::transform::transform::{Transform, TransformId};
use super::pack_signed::pack_signed;

fn source_i32(value: f64) -> i32 {
    assert!(
        value.trunc() >= i32::MIN as f64 && value.trunc() <= i32::MAX as f64,
        "source palette float-to-int conversion must be defined"
    );
    value as i32
}

pub fn merge_trees(
    trees: &[Tree],
    tree_splits: &[usize],
    begin: usize,
    end: usize,
    tree: &mut Tree,
) -> Status {
    if trees.len().wrapping_add(1) != tree_splits.len() || end <= begin || end > trees.len() {
        return Err(StatusCode::GenericError);
    }
    if end == begin.wrapping_add(1) {
        let sz = tree.len();
        tree.extend_from_slice(&trees[begin]);
        for node in &mut tree[sz..] {
            node.lchild = (node.lchild as usize).wrapping_add(sz) as u32;
            node.rchild = (node.rchild as usize).wrapping_add(sz) as u32;
        }
        return Ok(());
    }
    let mid = begin.wrapping_add(end) / 2;
    let splitval = tree_splits[mid].wrapping_sub(1);
    let cur = tree.len();
    tree.push(PropertyDecisionNode::new(
        1,
        splitval as i32,
        0,
        0,
        Predictor::ZERO,
        0,
        1,
    ));
    tree[cur].lchild = tree.len() as u32;
    merge_trees(trees, tree_splits, mid, end, tree)?;
    tree[cur].rchild = tree.len() as u32;
    merge_trees(trees, tree_splits, begin, mid, tree)?;
    Ok(())
}

/// Quantize the complete plane, preserving source division and valid signed
/// arithmetic preconditions, even when the channel's logical dimensions differ.
pub fn quantize_channel(ch: &mut Channel, q: i32) {
    if q == 1 {
        return;
    }
    for y in 0..ch.plane.ysize() {
        for x in 0..ch.plane.xsize() {
            let value = &mut ch.plane.row_mut(y)[x];
            *value = if *value < 0 {
                -((-*value + q / 2) / q) * q
            } else {
                ((*value + q / 2) / q) * q
            };
        }
    }
}

pub fn estimate_wp_cost(img: &Image, i: usize) -> f32 {
    let mut extra_bits = 0usize;
    let mut histo_cost = 0.0f32;
    let config = HybridUintConfig::default();
    const CUTOFFS: [i32; 33] = [
        -500, -392, -255, -191, -127, -95, -63, -47, -31, -23, -15, -11, -7, -4, -3, -1, 0, 1, 3,
        5, 7, 11, 15, 23, 31, 47, 63, 95, 127, 191, 255, 392, 500,
    ];
    let mut histo: [Histogram; 34] = std::array::from_fn(|_| Histogram::default());
    let mut wp_header = weighted::Header::default();
    weighted::predictor_mode(i as i32, &mut wp_header);
    for ch in &img.channel {
        let mut wp_state = weighted::State::new(&wp_header, ch.w, ch.h);
        let mut properties = vec![0];
        let mut unhealthy = false;
        for y in 0..ch.h {
            let row = ch.row(y);
            for x in 0..ch.w {
                let left = if x != 0 {
                    row[x - 1]
                } else if y != 0 {
                    ch.row(y - 1)[x]
                } else {
                    0
                } as i64;
                let top = if y != 0 {
                    ch.row(y - 1)[x] as i64
                } else {
                    left
                };
                let topleft = if x != 0 && y != 0 {
                    ch.row(y - 1)[x - 1] as i64
                } else {
                    left
                };
                let topright = if x + 1 < ch.w && y != 0 {
                    ch.row(y - 1)[x + 1] as i64
                } else {
                    top
                };
                let toptop = if y > 1 { ch.row(y - 2)[x] as i64 } else { top };
                let guess = wp_state.predict::<true>(
                    x,
                    y,
                    ch.w,
                    top,
                    left,
                    topright,
                    topleft,
                    toptop,
                    Some(&mut properties),
                    0,
                ) as i32;
                let mut ctx = 0;
                for c in CUTOFFS {
                    ctx += usize::from(c >= properties[0]);
                }
                let (res, overflow) = row[x].overflowing_sub(guess);
                unhealthy |= overflow;
                let (token, nbits, _) = config.encode(pack_signed(res));
                histo[ctx].add(token as usize);
                extra_bits = extra_bits.wrapping_add(nbits as usize);
                wp_state.update_errors(row[x] as i64, x, y, ch.w);
            }
        }
        if unhealthy {
            return f32::MAX;
        }
        for h in &mut histo {
            histo_cost += h.shannon_entropy();
            h.clear();
        }
    }
    histo_cost + extra_bits as f32
}

pub fn do_transform(
    image: &mut Image,
    tr: &Transform,
    wp_header: &weighted::Header,
    force_jxlart: bool,
) -> bool {
    let mut t = tr.clone();
    let did_it = if force_jxlart {
        if t.meta_apply(image).is_err() {
            return false;
        }
        true
    } else {
        transform_forward(&mut t, image, wp_header).is_ok()
    };
    if did_it {
        image.transform.push(t);
    }
    did_it
}

pub fn maybe_do_transform(
    image: &mut Image,
    tr: &Transform,
    cparams: &CompressParams<'_>,
    wp_header: &weighted::Header,
    cost_before: f32,
    force_jxlart: bool,
) -> StatusOr<bool> {
    if force_jxlart || cparams.speed_tier >= SpeedTier::Squirrel {
        return Ok(do_transform(image, tr, wp_header, force_jxlart));
    }
    let mut did_it = do_transform(image, tr, wp_header, false);
    if did_it {
        let cost_after = estimate_cost(image)?;
        if cost_after > cost_before {
            let t = image
                .transform
                .last()
                .expect("successful source transform pushed an entry")
                .clone();
            if t.inverse(image, wp_header).is_err() {
                return Ok(false);
            }
            image.transform.pop();
            did_it = false;
        }
    }
    Ok(did_it)
}

pub fn try_palettes(
    gi: &mut Image,
    max_bitdepth: &mut i32,
    maxval: &mut i32,
    cparams: &CompressParams<'_>,
    channel_colors_percent: f32,
) -> Status {
    let mut cost_before = 0.0f32;
    let mut did_palette = 0usize;
    let nb_pixels = gi.channel[0].w.wrapping_mul(gi.channel[0].h) as f32;
    let nb_chans = gi.channel.len().wrapping_sub(gi.nb_meta_channels) as i32;
    let arbitrary_bpp_estimate = 0.2f32 * gi.bitdepth as f32 * nb_chans as f32;
    if cparams.palette_colors != 0 || cparams.lossy_palette {
        cost_before = if cparams.speed_tier <= SpeedTier::Squirrel {
            estimate_cost(gi)?
        } else {
            nb_pixels * arbitrary_bpp_estimate
        };
        if nb_chans > 1 {
            let mut t = Transform::new(TransformId::PALETTE);
            t.begin_c = gi.nb_meta_channels as u32;
            t.num_c = nb_chans as u32;
            assert_ne!(
                cparams.palette_colors,
                i32::MIN,
                "source palette abs must be defined"
            );
            t.nb_colors =
                source_i32((mul_add_f32(cost_before, 0.0005, nb_pixels / 128.0) + 128.0) as f64)
                    .min(cparams.palette_colors.abs()) as u32;
            t.ordered_palette = cparams.palette_colors >= 0;
            t.lossy_palette = cparams.lossy_palette && t.num_c == 3;
            if t.lossy_palette {
                t.predictor = Predictor::AVERAGE4;
            }
            did_palette = usize::from(maybe_do_transform(
                gi,
                &t,
                cparams,
                &weighted::Header::default(),
                cost_before,
                cparams.options.zero_tokens,
            )?);
        }
        if did_palette == 0 && nb_chans > 3 {
            let mut t = Transform::new(TransformId::PALETTE);
            t.begin_c = gi.nb_meta_channels as u32;
            t.num_c = (nb_chans - 1) as u32;
            assert_ne!(
                cparams.palette_colors,
                i32::MIN,
                "source palette abs must be defined"
            );
            t.nb_colors =
                source_i32((mul_add_f32(cost_before, 0.0005, nb_pixels / 128.0) + 128.0) as f64)
                    .min(cparams.palette_colors.abs()) as u32;
            t.ordered_palette = cparams.palette_colors >= 0;
            t.lossy_palette = cparams.lossy_palette;
            if t.lossy_palette {
                t.predictor = Predictor::AVERAGE4;
            }
            did_palette = usize::from(maybe_do_transform(
                gi,
                &t,
                cparams,
                &weighted::Header::default(),
                cost_before,
                cparams.options.zero_tokens,
            )?);
        }
    }
    if channel_colors_percent > 0.0 {
        let nb_channels = gi
            .channel
            .len()
            .wrapping_sub(gi.nb_meta_channels)
            .wrapping_sub(did_palette);
        let orig_bitdepth = *max_bitdepth;
        *max_bitdepth = 0;
        if nb_channels > 0 && (did_palette != 0 || cost_before == 0.0) {
            cost_before = if cparams.speed_tier < SpeedTier::Squirrel {
                estimate_cost(gi)?
            } else {
                0.0
            };
        }
        for i in did_palette..nb_channels.wrapping_add(did_palette) {
            let (mut min, mut max) = (0, 0);
            compute_minmax(
                &gi.channel[gi.nb_meta_channels + i],
                Some(&mut min),
                Some(&mut max),
            );
            let colors = max as i64 - min as i64 + 1;
            let mut t = Transform::new(TransformId::PALETTE);
            t.begin_c = i.wrapping_add(gi.nb_meta_channels) as u32;
            t.num_c = 1;
            t.nb_colors = source_i32((nb_pixels / 16.0) as f64).min(source_i32(
                channel_colors_percent as f64 / 100.0 * colors as f64,
            )) as u32;
            let did_ch_palette = maybe_do_transform(
                gi,
                &t,
                cparams,
                &weighted::Header::default(),
                cost_before,
                false,
            )?;
            if did_ch_palette {
                compute_minmax(
                    &gi.channel[gi.nb_meta_channels + i],
                    Some(&mut min),
                    Some(&mut max),
                );
                if max < *maxval {
                    *maxval = max;
                }
                let ch_bitdepth = if max > 0 {
                    ceil_log2_nonzero(max as u32) as i32
                } else {
                    0
                };
                if ch_bitdepth > *max_bitdepth {
                    *max_bitdepth = ch_bitdepth;
                }
            } else {
                *max_bitdepth = orig_bitdepth;
            }
        }
    }
    Ok(())
}

/// Convert normalized integer samples or exactly representable custom floats.
///
/// Both rows must contain `xsize` samples. Integer conversion requires finite
/// products whose rounded, truncated result fits i32. Custom floats other than
/// binary32 require 1..=8 exponent bits and 0..=23 mantissa bits.
pub fn float_to_int(
    row_in: &[f32],
    row_out: &mut [i32],
    xsize: usize,
    bits: u32,
    exp_bits: u32,
    fp: bool,
    dfactor: f64,
) -> Status {
    if bits > 32 {
        return Err(StatusCode::GenericError);
    }
    if !fp {
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        if bits > 22 {
            for x in 0..xsize {
                let value = mul_add_f64(
                    row_in[x] as f64,
                    dfactor,
                    if row_in[x] < 0.0 { -0.5 } else { 0.5 },
                );
                assert!(value.trunc() >= i32::MIN as f64 && value.trunc() <= i32::MAX as f64);
                row_out[x] = value as i32;
            }
        } else {
            let factor = dfactor as f32;
            for x in 0..xsize {
                let value =
                    mul_add_f32(row_in[x], factor, if row_in[x] < 0.0 { -0.5 } else { 0.5 });
                assert!(
                    value.trunc() as f64 >= i32::MIN as f64
                        && value.trunc() as f64 <= i32::MAX as f64
                );
                row_out[x] = value as i32;
            }
        }
        return Ok(());
    }
    if bits == 32 {
        if exp_bits != 8 {
            return Err(StatusCode::GenericError);
        }
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        for x in 0..xsize {
            row_out[x] = row_in[x].to_bits() as i32;
        }
        return Ok(());
    }
    if bits == 0 {
        return Err(StatusCode::GenericError);
    }
    assert!((1..=8).contains(&exp_bits) && bits > exp_bits && bits - exp_bits - 1 <= 23);
    let exp_bias = (1i32 << (exp_bits - 1)) - 1;
    let max_exp = (1i32 << exp_bits) - 1;
    let sign = 1u32 << (bits - 1);
    let mant_bits = (bits - exp_bits - 1) as i32;
    let mant_shift = 23 - mant_bits;
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let mut f = row_in[x].to_bits();
        let signbit = f >> 31;
        f &= 0x7fff_ffff;
        if f == 0 {
            row_out[x] = if signbit != 0 { sign as i32 } else { 0 };
            continue;
        }
        let mut exp = (f >> 23) as i32 - 127;
        let mut mantissa = (f & 0x007f_ffff) as i32;
        if exp == 128 {
            f = if signbit != 0 { sign } else { 0 };
            f |= (((1i32 << exp_bits) - 1) << mant_bits) as u32;
            f |= (mantissa >> mant_shift) as u32;
            row_out[x] = f as i32;
            continue;
        }
        exp += exp_bias;
        if exp <= 0 {
            mantissa |= 0x0080_0000;
            if exp < -mant_bits {
                return Err(StatusCode::GenericError);
            }
            mantissa >>= 1 - exp;
            exp = 0;
        }
        if exp >= max_exp {
            return Err(StatusCode::GenericError);
        }
        if mantissa & ((1 << mant_shift) - 1) != 0 {
            return Err(StatusCode::GenericError);
        }
        mantissa >>= mant_shift;
        f = if signbit != 0 { sign } else { 0 };
        f |= (exp << mant_bits) as u32;
        f |= mantissa as u32;
        row_out[x] = f as i32;
    }
    Ok(())
}
