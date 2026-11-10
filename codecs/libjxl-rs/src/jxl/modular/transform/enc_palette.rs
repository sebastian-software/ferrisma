// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Forward palette selection from modular/transform/enc_palette.{h,cc}.
//! Ordered maps/sets preserve lexicographic iteration. Palette sorting uses the
//! reference libc++ specialization, including its equivalent-element order.
//! Row pointers become short borrows; load all source components before writing
//! an index. Lossy reconstruction owns the source's separate quantized image.

use super::enc_transform::compute_minmax;
use super::palette::palette_internal::{
    IMPLICIT_PALETTE_SIZE, LARGE_CUBE, LARGE_CUBE_OFFSET, MAX_PALETTE_LOOKUP_TABLE_SIZE,
    SMALL_CUBE, get_palette_value,
};
use super::transform::check_equal_channels;
use crate::jxl::base::float::{min_f32, mul_add_f32, mul_add_f64};
use crate::jxl::base::libcpp_sort;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::image_ops::copy_image_to;
use crate::jxl::modular::encoding::context_predict::{PixelPosition, predict_no_tree_wp, weighted};
use crate::jxl::modular::modular_image::{Channel, Image};
use crate::jxl::modular::options::Predictor;
use std::collections::{BTreeMap, BTreeSet};

pub mod palette_internal {
    use super::*;
    pub const MIN_IMPLICIT_PALETTE_INDEX: i32 = -(2 * 72 - 1);
    pub fn color_distance(a: &[f32], b: &[i32]) -> f32 {
        let mut distance = 0.0;
        let mut ave3 = 0.0;
        if a.len() >= 3 {
            ave3 =
                (a[0] + b[0] as f32 + a[1] + b[1] as f32 + a[2] + b[2] as f32) * (1.21f32 / 3.0f32);
        }
        let (mut sum_a, mut sum_b) = (0.0, 0.0);
        for c in 0..a.len() {
            let difference = a[c] - b[c] as f32;
            let mut weight = if c == 0 {
                3.0f32
            } else if c == 1 {
                5.0
            } else {
                2.0
            };
            if c < 3 && a[c] + b[c] as f32 >= ave3 {
                let add_w = [1.15f32, 1.15, 1.12];
                weight += add_w[c];
                if c == 2 && f64::from(a[2] + b[2] as f32) < 1.22 * f64::from(ave3) {
                    weight = (f64::from(weight) - 0.5) as f32;
                }
            }
            distance = mul_add_f32(difference * difference * weight, weight, distance);
            let sum_weight = if c == 0 {
                3
            } else if c == 1 {
                5
            } else {
                1
            };
            sum_a = mul_add_f32(a[c], sum_weight as f32, sum_a);
            sum_b += (b[c] * sum_weight) as f32;
        }
        distance *= 4.0;
        let sum_difference = sum_a - sum_b;
        mul_add_f32(sum_difference, sum_difference, distance)
    }
    pub fn quantize_color_to_implicit_palette_index(
        color: &[i32],
        palette_size: i32,
        bit_depth: i32,
        high_quality: bool,
    ) -> i32 {
        let mut index = 0;
        let quant = (1 << bit_depth) - 1;
        let half = if bit_depth > 1 {
            1 << (bit_depth - 1)
        } else {
            0
        };
        if high_quality {
            let mut multiplier = 1;
            for &value in color {
                let quantized = ((LARGE_CUBE - 1) * value + half) / quant;
                index += quantized * multiplier;
                multiplier *= LARGE_CUBE;
            }
            index + palette_size + LARGE_CUBE_OFFSET
        } else {
            let mut multiplier = 1;
            for &value in color {
                let value = (value - (1 << (bit_depth - 3).max(0))).max(0);
                let mut quantized = ((LARGE_CUBE - 1) * value + half) / quant;
                if quantized > SMALL_CUBE - 1 {
                    quantized = SMALL_CUBE - 1;
                }
                index += quantized * multiplier;
                multiplier *= SMALL_CUBE;
            }
            index + palette_size
        }
    }
}
pub fn round_int(value: i32, div: i32) -> i32 {
    if value < 0 {
        return -round_int(-value, div);
    }
    (value + div / 2) / div
}
#[derive(Default)]
struct PaletteIterationData {
    final_run: bool,
    deltas: [Vec<i32>; 3],
    delta_distances: Vec<f64>,
    frequent_deltas: [Vec<i32>; 3],
}
impl PaletteIterationData {
    fn find_frequent_color_deltas(&mut self, num_pixels: i32, bitdepth: i32) {
        let mut frequencies = BTreeMap::<[i32; 3], f64>::new();
        let bucket_size = 3 << (bitdepth - 8).max(0);
        for i in 0..self.deltas[0].len() {
            let delta = std::array::from_fn(|c| round_int(self.deltas[c][i], bucket_size));
            if delta == [0; 3] {
                continue;
            }
            *frequencies.entry(delta).or_default() += self.delta_distances[i].sqrt().sqrt();
        }
        let distance_multiplier = 1.0f32 / num_pixels as f32;
        for (delta, frequency) in &mut frequencies {
            let distance = palette_internal::color_distance(&[0.0; 3], delta).sqrt() + 1.0;
            *frequency *= f64::from(distance) * f64::from(distance_multiplier);
        }
        let mut sorted: Vec<_> = frequencies.into_iter().collect();
        libcpp_sort::sort_by(&mut sorted, |a, b| a.1 > b.1);
        for (delta, frequency) in sorted {
            if self.frequent_deltas[0].len() >= 128 {
                break;
            }
            if frequency < 17.0 {
                break;
            }
            for (c, value) in delta.into_iter().enumerate() {
                self.frequent_deltas[c].push(value * bucket_size);
            }
        }
    }
}
fn ordered_luma(color: &[i32], count: usize, threshold: f32) -> f32 {
    let y = mul_add_f32(0.587f32, color[1] as f32, 0.299f32 * color[0] as f32);
    let mut y = mul_add_f32(0.114f32, color[2] as f32, y) + 0.1f32;
    if color.len() > 3 {
        y *= 1.0f32 + color[3] as f32;
    }
    if count as f32 > threshold { -y } else { y }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source palette fields and iteration state"
)]
fn fwd_palette_iteration(
    input: &mut Image,
    begin_c: u32,
    end_c: u32,
    nb_colors: &mut u32,
    nb_deltas: &mut u32,
    ordered: bool,
    lossy: bool,
    predictor: &mut Predictor,
    wp_header: &weighted::Header,
    iteration: &mut PaletteIterationData,
) -> Status {
    check_equal_channels(input, begin_c, end_c)?;
    if (begin_c as usize) < input.nb_meta_channels {
        return Err(StatusCode::GenericError);
    }
    let nb = end_c.wrapping_sub(begin_c).wrapping_add(1) as usize;
    let begin = begin_c as usize;
    let (w, h) = (input.channel[begin].w, input.channel[begin].h);
    if input.bitdepth >= 32 {
        return Err(StatusCode::GenericError);
    }
    if !lossy && *nb_colors < 2 {
        return Err(StatusCode::GenericError);
    }
    if !lossy && nb == 1 {
        if *nb_colors == 0 {
            return Err(StatusCode::GenericError);
        }
        let (mut min, mut max) = (0, 0);
        compute_minmax(&input.channel[begin], Some(&mut min), Some(&mut max));
        let size = (i64::from(max) - i64::from(min) + 1) as usize;
        if size > MAX_PALETTE_LOOKUP_TABLE_SIZE as usize {
            let mut palette = BTreeSet::new();
            let mut idx = 0i32;
            for y in 0..h {
                for &value in &input.channel[begin].row(y)[..w] {
                    if palette.insert(value) {
                        idx += 1;
                        if idx > *nb_colors as i32 {
                            return Err(StatusCode::GenericError);
                        }
                    }
                }
            }
            let mut pch = Channel::create(idx as usize, 1)?;
            pch.hshift = -1;
            pch.vshift = -1;
            *nb_colors = idx as u32;
            for (idx, value) in palette.into_iter().enumerate() {
                pch.row_mut(0)[idx] = value;
            }
            for y in 0..h {
                for pixel in &mut input.channel[begin].row_mut(y)[..w] {
                    let mut idx = 0usize;
                    while *pixel != pch.row(0)[idx] && idx < *nb_colors as usize {
                        idx += 1;
                    }
                    *pixel = idx as i32;
                }
            }
            *predictor = Predictor::ZERO;
            input.nb_meta_channels += 1;
            input.channel.insert(0, pch);
            return Ok(());
        }
        let mut lookup = vec![0i32; size];
        let mut idx = 0i32;
        for y in 0..h {
            for &value in &input.channel[begin].row(y)[..w] {
                if lookup[(value - min) as usize] == 0 {
                    lookup[(value - min) as usize] = 1;
                    idx += 1;
                    if idx > *nb_colors as i32 {
                        return Err(StatusCode::GenericError);
                    }
                }
            }
        }
        let mut pch = Channel::create(idx as usize, 1)?;
        pch.hshift = -1;
        pch.vshift = -1;
        *nb_colors = idx as u32;
        idx = 0;
        for (i, v) in lookup.iter_mut().enumerate() {
            if *v != 0 {
                pch.row_mut(0)[idx as usize] = i.wrapping_add(min as usize) as i32;
                *v = idx;
                idx += 1;
            }
        }
        for y in 0..h {
            for pixel in &mut input.channel[begin].row_mut(y)[..w] {
                *pixel = lookup[(*pixel - min) as usize];
            }
        }
        *predictor = Predictor::ZERO;
        input.nb_meta_channels += 1;
        input.channel.insert(0, pch);
        return Ok(());
    }
    let mut quantized_input = Image::create(0, 0, 8, 0)?;
    if lossy {
        quantized_input = Image::create(w, h, input.bitdepth, nb as i32)?;
        for c in 0..nb {
            copy_image_to(
                &input.channel[begin + c].plane,
                &mut quantized_input.channel[c].plane,
            )?;
        }
    }
    *nb_deltas = 0;
    let mut delta_used = false;
    let mut candidates = BTreeSet::<Vec<i32>>::new();
    let mut image_order = Vec::<Vec<i32>>::new();
    let mut color = vec![0i32; nb];
    let mut color_with_error = vec![0f32; nb];
    let mut inverse = BTreeMap::<Vec<i32>, usize>::new();
    if lossy {
        iteration.find_frequent_color_deltas(w.wrapping_mul(h) as i32, input.bitdepth);
        *nb_deltas = iteration.frequent_deltas[0].len() as u32;
        let mut frequency = BTreeMap::<Vec<i32>, usize>::new();
        for y in 1..h.saturating_sub(1) {
            for x in 1..w.saturating_sub(1) {
                for (c, value) in color.iter_mut().enumerate() {
                    *value = input.channel[begin + c].row(y)[x];
                }
                let mut cross = true;
                for [dx, dy] in [[1isize, 0], [-1, 0], [0, 1], [0, -1]] {
                    if !cross {
                        break;
                    }
                    for (c, &value) in color.iter().enumerate() {
                        if !cross {
                            break;
                        }
                        if input.channel[begin + c].row((y as isize + dy) as usize)
                            [(x as isize + dx) as usize]
                            != value
                        {
                            cross = false;
                        }
                    }
                }
                if cross {
                    *frequency.entry(color.clone()).or_default() += 1;
                }
            }
        }
        let bound = mul_add_f32(input.h.wrapping_mul(input.w) as f32, 0.01f32, 5.0) as usize;
        for (color, frequency) in frequency {
            if frequency > bound {
                candidates.insert(color.clone());
                image_order.push(color);
            }
        }
    }
    let mut implicit = BTreeMap::<Vec<i32>, bool>::new();
    let mut implicit_colors = Vec::with_capacity(IMPLICIT_PALETTE_SIZE as usize);
    for k in 0..IMPLICIT_PALETTE_SIZE {
        for (c, value) in color.iter_mut().enumerate() {
            *value = get_palette_value(&[], k, c, 0, 0, input.bitdepth);
        }
        implicit.insert(color.clone(), true);
        implicit_colors.push(color.clone());
    }
    let mut frequency = BTreeMap::<Vec<i32>, usize>::new();
    let mut implicit_used = 0u32;
    for y in 0..h {
        for x in 0..w {
            if lossy && candidates.len() >= *nb_colors as usize {
                break;
            }
            for (c, value) in color.iter_mut().enumerate() {
                *value = input.channel[begin + c].row(y)[x];
            }
            if candidates.insert(color.clone()) {
                if *implicit.entry(color.clone()).or_default() {
                    implicit_used = implicit_used.wrapping_add(1);
                } else {
                    image_order.push(color.clone());
                    if image_order.len() > *nb_colors as usize {
                        return Err(StatusCode::GenericError);
                    }
                }
            }
            *frequency.entry(color.clone()).or_default() += 1;
        }
    }
    *nb_colors = (*nb_deltas as usize).wrapping_add(image_order.len()) as u32;
    if !lossy && nb_colors.wrapping_add(implicit_used) == 1 {
        return Err(StatusCode::GenericError);
    }
    for color in &implicit_colors {
        if *frequency.entry(color.clone()).or_default() > 10 {
            *nb_colors = nb_colors.wrapping_add(1);
            image_order.push(color.clone());
        }
    }
    for (k, color) in implicit_colors.into_iter().enumerate() {
        inverse.insert(color, (*nb_colors as usize).wrapping_add(k));
    }
    let mut pch = Channel::create(*nb_colors as usize, nb)?;
    pch.hshift = -1;
    pch.vshift = -1;
    let onerow = pch.plane.pixels_per_row();
    let onerow_image = input.channel[begin].plane.pixels_per_row();
    let bit_depth = input.bitdepth.min(24);
    if lossy {
        for i in 0..*nb_deltas as usize {
            for c in 0..3 {
                pch.row_mut(c)[i] = iteration.frequent_deltas[c][i];
            }
        }
    }
    if ordered && nb >= 3 {
        libcpp_sort::sort_by(&mut image_order, |a, b| {
            ordered_luma(a, frequency[a], 4.0) < ordered_luma(b, frequency[b], 4.0)
        });
    }
    for (clr, color) in image_order.into_iter().enumerate() {
        for (c, &value) in color.iter().enumerate() {
            pch.row_mut(c)[*nb_deltas as usize + clr] = value;
        }
        inverse.insert(color, clr);
    }
    let mut wp_states: Vec<_> = (0..nb)
        .map(|_| weighted::State::new(wp_header, w, h))
        .collect();
    let mut error_row: [Vec<Vec<f32>>; 3] = std::array::from_fn(|_| Vec::new());
    if lossy {
        for row in &mut error_row {
            row.resize_with(nb, || vec![0.0; w + 4]);
        }
    }
    for y in 0..h {
        for x in 0..w {
            let index;
            if !lossy {
                for (c, value) in color.iter_mut().enumerate() {
                    *value = input.channel[begin + c].row(y)[x];
                }
                index = *inverse.entry(color.clone()).or_default() as i32;
            } else {
                let mut best_index = 0;
                let mut best_is_delta = false;
                let mut best_distance = f32::INFINITY;
                let mut best_val = vec![0i32; nb];
                let mut ideal_residual = vec![0i32; nb];
                let mut quantized_val = vec![0i32; nb];
                let mut predictions = vec![0i32; nb];
                for diffusion_multiplier in [0.55f64, 0.75] {
                    for c in 0..nb {
                        color_with_error[c] = mul_add_f64(
                            if iteration.final_run {
                                diffusion_multiplier
                            } else {
                                0.0
                            },
                            f64::from(error_row[0][c][x + 2]),
                            f64::from(input.channel[begin + c].row(y)[x]),
                        ) as f32;
                        color[c] = (color_with_error[c].round() as i64)
                            .clamp(0, (1i64 << input.bitdepth) - 1)
                            as i32;
                    }
                    for c in 0..nb {
                        let q = &quantized_input.channel[c];
                        let pos = PixelPosition::new(
                            q.plane.pixels().unwrap(),
                            y * q.plane.pixels_per_row() as usize + x,
                            onerow_image,
                        );
                        predictions[c] = predict_no_tree_wp(
                            w,
                            pos,
                            x as i32,
                            y as i32,
                            *predictor,
                            &mut wp_states[c],
                        )
                        .guess as i32;
                    }
                    let mut try_index = |idx: i32| {
                        for c in 0..nb {
                            quantized_val[c] = get_palette_value(
                                pch.plane.pixels().unwrap_or(&[]),
                                idx,
                                c,
                                *nb_colors as i32,
                                onerow as i32,
                                bit_depth,
                            );
                            if idx < *nb_deltas as i32 {
                                quantized_val[c] += predictions[c];
                            }
                        }
                        let color_distance = (32.0f64
                            / (1i64 << (2 * (bit_depth - 8)).max(0)) as f64
                            * f64::from(palette_internal::color_distance(
                                &color_with_error,
                                &quantized_val,
                            ))) as f32;
                        let penalty = if idx == -1 {
                            -124
                        } else if idx < 0 {
                            -2 * idx
                        } else if idx < *nb_deltas as i32 {
                            250
                        } else if idx < *nb_colors as i32 {
                            150
                        } else if idx < (*nb_colors as i32) + LARGE_CUBE_OFFSET {
                            70
                        } else {
                            256
                        };
                        let distance = color_distance + penalty as f32;
                        if distance < best_distance {
                            best_distance = distance;
                            best_index = idx;
                            best_is_delta = idx < *nb_deltas as i32;
                            std::mem::swap(&mut best_val, &mut quantized_val);
                            for c in 0..nb {
                                ideal_residual[c] =
                                    (color_with_error[c] - predictions[c] as f32) as i32;
                            }
                        }
                    };
                    for idx in palette_internal::MIN_IMPLICIT_PALETTE_INDEX..*nb_colors as i32 {
                        try_index(idx);
                    }
                    try_index(palette_internal::quantize_color_to_implicit_palette_index(
                        &color,
                        *nb_colors as i32,
                        bit_depth,
                        false,
                    ));
                    try_index(palette_internal::quantize_color_to_implicit_palette_index(
                        &color,
                        *nb_colors as i32,
                        bit_depth,
                        true,
                    ));
                }
                index = best_index;
                delta_used |= best_is_delta;
                if !iteration.final_run {
                    for (c, &value) in ideal_residual.iter().enumerate().take(3) {
                        iteration.deltas[c].push(value);
                    }
                    iteration.delta_distances.push(f64::from(best_distance));
                }
                for c in 0..nb {
                    wp_states[c].update_errors(i64::from(best_val[c]), x, y, w);
                    quantized_input.channel[c].row_mut(y)[x] = best_val[c];
                }
                let mut len_error = 0.0;
                for c in 0..nb {
                    let local_error = color_with_error[c] - best_val[c] as f32;
                    len_error = mul_add_f32(local_error, local_error, len_error);
                }
                len_error = len_error.sqrt();
                let mut modulate = 1.0f32;
                let len_limit = 38 << (bit_depth - 8).max(0);
                if len_error > len_limit as f32 {
                    modulate *= len_limit as f32 / len_error;
                }
                for c in 0..nb {
                    let mut total_error = color_with_error[c] - best_val[c] as f32;
                    const OFFSETS: [[usize; 2]; 12] = [
                        [1, 2],
                        [0, 3],
                        [0, 4],
                        [1, 1],
                        [1, 3],
                        [2, 2],
                        [1, 0],
                        [1, 4],
                        [2, 1],
                        [2, 3],
                        [2, 0],
                        [2, 4],
                    ];
                    let mut total_available = 0.0;
                    for &[row, col] in &OFFSETS[..11] {
                        if error_row[row][c][x + col].is_sign_negative()
                            != total_error.is_sign_negative()
                        {
                            total_available += error_row[row][c][x + col];
                        }
                    }
                    let weight = (f64::from(total_error.abs())
                        / (f64::from(total_available.abs()) + 1e-3))
                        as f32;
                    let weight = min_f32(weight, 1.0);
                    for &[row, col] in &OFFSETS[..11] {
                        let error = &mut error_row[row][c][x + col];
                        if error.is_sign_negative() != total_error.is_sign_negative() {
                            total_error = mul_add_f32(weight, *error, total_error);
                            *error *= 1.0f32 - weight;
                        }
                    }
                    total_error *= modulate;
                    let remaining_error = ((1.0f64 / 14.0) * f64::from(total_error)) as f32;
                    error_row[0][c][x + 3] =
                        mul_add_f32(2.0, remaining_error, error_row[0][c][x + 3]);
                    error_row[0][c][x + 4] += remaining_error;
                    error_row[1][c][x] += remaining_error;
                    for i in 0..5 {
                        error_row[1][c][x + i] += remaining_error;
                        error_row[2][c][x + i] += remaining_error;
                    }
                }
            }
            if iteration.final_run {
                input.channel[begin].row_mut(y)[x] = index;
            }
        }
        if lossy {
            for c in 0..nb {
                let [a, b, d] = &mut error_row;
                std::mem::swap(&mut a[c], &mut b[c]);
                std::mem::swap(&mut b[c], &mut d[c]);
                d[c].fill(0.0);
            }
        }
    }
    if !delta_used {
        *predictor = Predictor::ZERO;
    }
    if iteration.final_run {
        input.nb_meta_channels += 1;
        input.channel.drain(begin + 1..end_c as usize + 1);
        input.channel.insert(0, pch);
    }
    *nb_colors = nb_colors.wrapping_sub(*nb_deltas);
    Ok(())
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the original mutable palette parameters"
)]
pub fn fwd_palette(
    input: &mut Image,
    begin_c: u32,
    end_c: u32,
    nb_colors: &mut u32,
    nb_deltas: &mut u32,
    ordered: bool,
    lossy: bool,
    predictor: &mut Predictor,
    wp_header: &weighted::Header,
) -> Status {
    let mut iteration = PaletteIterationData::default();
    let (mut colors_orig, mut deltas_orig) = (*nb_colors, *nb_deltas);
    if lossy && input.bitdepth >= 8 {
        fwd_palette_iteration(
            input,
            begin_c,
            end_c,
            &mut colors_orig,
            &mut deltas_orig,
            ordered,
            lossy,
            predictor,
            wp_header,
            &mut iteration,
        )?;
    }
    iteration.final_run = true;
    fwd_palette_iteration(
        input,
        begin_c,
        end_c,
        nb_colors,
        nb_deltas,
        ordered,
        lossy,
        predictor,
        wp_header,
        &mut iteration,
    )
}
