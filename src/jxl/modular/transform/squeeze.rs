// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Original scalar inverse/metadata/defaults from modular/transform/squeeze.h
//! and squeeze.cc. Null-pool tasks retain eight-row / 64-column spans; SIMD
//! specializations and custom runners remain unported. Owned output channels
//! replace pointer aliases and move into the image at the source checkpoints.

use super::squeeze_params::SqueezeParams;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::modular::modular_image::{Channel, Image};
pub const MAX_FIRST_PREVIEW_SIZE: usize = 8;

pub fn smooth_tendency(b: i64, a: i64, n: i64) -> i64 {
    let mut diff = 0;
    if b >= a && a >= n {
        diff = (4 * b - 3 * n - a + 6) / 12;
        if diff - (diff & 1) > 2 * (b - a) {
            diff = 2 * (b - a) + 1;
        }
        if diff + (diff & 1) > 2 * (a - n) {
            diff = 2 * (a - n);
        }
    } else if b <= a && a <= n {
        diff = (4 * b - 3 * n - a - 6) / 12;
        if diff + (diff & 1) < 2 * (b - a) {
            diff = 2 * (b - a) - 1;
        }
        if diff - (diff & 1) < 2 * (a - n) {
            diff = 2 * (a - n);
        }
    }
    diff
}
pub fn inv_h_squeeze(input: &mut Image, c: u32, rc: u32) -> Status {
    if c as usize >= input.channel.len() || rc as usize >= input.channel.len() {
        return Err(StatusCode::GenericError);
    }
    let ci = c as usize;
    let ri = rc as usize;
    let chin = &input.channel[ci];
    let residual = &input.channel[ri];
    if chin.w != chin.w.wrapping_add(residual.w).wrapping_add(1) / 2 || chin.h != residual.h {
        return Err(StatusCode::GenericError);
    }
    if residual.w == 0 {
        input.channel[ci].hshift -= 1;
        return Ok(());
    }
    let mut chout = Channel::create_with_shifts(
        chin.w.wrapping_add(residual.w),
        chin.h,
        chin.hshift - 1,
        chin.vshift,
    )?;
    if residual.h == 0 {
        input.channel[ci] = chout;
        return Ok(());
    }
    const ROWS_PER_THREAD: usize = 8;
    for task in 0..chin
        .h
        .wrapping_add(ROWS_PER_THREAD - 1)
        .wrapping_div(ROWS_PER_THREAD) as u32
    {
        let y0 = task as usize * ROWS_PER_THREAD;
        let rows = ROWS_PER_THREAD.min(chin.h - y0);
        for y in y0..y0 + rows {
            let p_residual = residual.row(y);
            let p_avg = chin.row(y);
            let outw = chout.w;
            let p_out = chout.row_mut(y);
            for x in 0..residual.w {
                let diff_minus_tendency = i64::from(p_residual[x]);
                let avg = i64::from(p_avg[x]);
                let next_avg = if x + 1 < chin.w {
                    i64::from(p_avg[x + 1])
                } else {
                    avg
                };
                let left = if x != 0 {
                    i64::from(p_out[(x << 1) - 1])
                } else {
                    avg
                };
                let tendency = smooth_tendency(left, avg, next_avg);
                let diff = diff_minus_tendency + tendency;
                let a = avg + diff / 2;
                p_out[x << 1] = a as i32;
                let b = a - diff;
                p_out[(x << 1) + 1] = b as i32;
            }
            if outw & 1 != 0 {
                p_out[outw - 1] = p_avg[chin.w - 1];
            }
        }
    }
    input.channel[ci] = chout;
    Ok(())
}
pub fn inv_v_squeeze(input: &mut Image, c: u32, rc: u32) -> Status {
    if c as usize >= input.channel.len() || rc as usize >= input.channel.len() {
        return Err(StatusCode::GenericError);
    }
    let ci = c as usize;
    let ri = rc as usize;
    let chin = &input.channel[ci];
    let residual = &input.channel[ri];
    if chin.h != chin.h.wrapping_add(residual.h).wrapping_add(1) / 2 || chin.w != residual.w {
        return Err(StatusCode::GenericError);
    }
    if residual.h == 0 {
        input.channel[ci].vshift -= 1;
        return Ok(());
    }
    let mut chout = Channel::create_with_shifts(
        chin.w,
        chin.h.wrapping_add(residual.h),
        chin.hshift,
        chin.vshift - 1,
    )?;
    if residual.w == 0 {
        input.channel[ci] = chout;
        return Ok(());
    }
    const COLS_PER_THREAD: u32 = 64;
    for task in 0..chin
        .w
        .wrapping_add(COLS_PER_THREAD as usize - 1)
        .wrapping_div(COLS_PER_THREAD as usize) as u32
    {
        let x0 = task.wrapping_mul(COLS_PER_THREAD) as usize;
        let x1 = ((task.wrapping_add(1) as usize) * COLS_PER_THREAD as usize).min(chin.w);
        let w = x1 - x0;
        for y in 0..residual.h {
            let p_residual = residual.row(y);
            let p_avg = chin.row(y);
            let p_navg = chin.row(if y + 1 < chin.h { y + 1 } else { y });
            for x in 0..w {
                let j = x + x0;
                let avg = i64::from(p_avg[j]);
                let next_avg = i64::from(p_navg[j]);
                let top = if y > 0 {
                    i64::from(chout.row((y << 1) - 1)[j])
                } else {
                    avg
                };
                let tendency = smooth_tendency(top, avg, next_avg);
                let diff_minus_tendency = i64::from(p_residual[j]);
                let diff = diff_minus_tendency + tendency;
                let out = avg + diff / 2;
                chout.row_mut(y << 1)[j] = out as i32;
                chout.row_mut((y << 1) + 1)[j] = (out - diff) as i32;
            }
        }
    }
    if chout.h & 1 != 0 {
        let y = chin.h - 1;
        let p_avg = chin.row(y);
        let p_out = chout.row_mut(y << 1);
        p_out[..chin.w].copy_from_slice(&p_avg[..chin.w]);
    }
    input.channel[ci] = chout;
    Ok(())
}
pub fn inv_squeeze(input: &mut Image, parameters: &[SqueezeParams]) -> Status {
    let mut i = parameters.len().wrapping_sub(1) as i32;
    while i >= 0 {
        let parameter = &parameters[i as usize];
        check_meta_squeeze_params(parameter, input.channel.len() as i32)?;
        let begin = parameter.begin_c;
        let end = begin.wrapping_add(parameter.num_c).wrapping_sub(1);
        let offset = if parameter.in_place {
            end.wrapping_add(1)
        } else {
            input
                .channel
                .len()
                .wrapping_add(begin as usize)
                .wrapping_sub(end as usize)
                .wrapping_sub(1) as u32
        };
        if (begin as usize) < input.nb_meta_channels {
            if input.nb_meta_channels <= parameter.num_c as usize {
                return Err(StatusCode::GenericError);
            }
            input.nb_meta_channels -= parameter.num_c as usize;
        }
        for c in begin..=end {
            let rc = offset.wrapping_add(c).wrapping_sub(begin);
            if rc as usize >= input.channel.len() {
                return Err(StatusCode::GenericError);
            }
            if input.channel[c as usize].w < input.channel[rc as usize].w
                || input.channel[c as usize].h < input.channel[rc as usize].h
            {
                return Err(StatusCode::GenericError);
            }
            if parameter.horizontal {
                inv_h_squeeze(input, c, rc)?;
            } else {
                inv_v_squeeze(input, c, rc)?;
            }
        }
        input.channel.drain(
            offset as usize..offset as usize + end.wrapping_sub(begin).wrapping_add(1) as usize,
        );
        i -= 1;
    }
    Ok(())
}
pub fn default_squeeze_parameters(parameters: &mut Vec<SqueezeParams>, image: &Image) {
    let nb_channels = image.channel.len().wrapping_sub(image.nb_meta_channels) as i32;
    parameters.clear();
    let mut w = image.channel[image.nb_meta_channels].w;
    let mut h = image.channel[image.nb_meta_channels].h;
    let wide = w > h;
    if nb_channels > 2
        && image.channel[image.nb_meta_channels + 1].w == w
        && image.channel[image.nb_meta_channels + 1].h == h
    {
        let mut p = SqueezeParams {
            horizontal: true,
            in_place: false,
            begin_c: image.nb_meta_channels.wrapping_add(1) as u32,
            num_c: 2,
        };
        parameters.push(p.clone());
        p.horizontal = false;
        parameters.push(p);
    }
    let mut p = SqueezeParams {
        begin_c: image.nb_meta_channels as u32,
        num_c: nb_channels as u32,
        in_place: true,
        ..Default::default()
    };
    if !wide && h > MAX_FIRST_PREVIEW_SIZE {
        p.horizontal = false;
        parameters.push(p.clone());
        h = h.wrapping_add(1) / 2;
    }
    while w > MAX_FIRST_PREVIEW_SIZE || h > MAX_FIRST_PREVIEW_SIZE {
        if w > MAX_FIRST_PREVIEW_SIZE {
            p.horizontal = true;
            parameters.push(p.clone());
            w = w.wrapping_add(1) / 2;
        }
        if h > MAX_FIRST_PREVIEW_SIZE {
            p.horizontal = false;
            parameters.push(p.clone());
            h = h.wrapping_add(1) / 2;
        }
    }
}
pub fn check_meta_squeeze_params(parameter: &SqueezeParams, num_channels: i32) -> Status {
    let c1 = parameter.begin_c as i32;
    let c2 = parameter
        .begin_c
        .wrapping_add(parameter.num_c)
        .wrapping_sub(1) as i32;
    if c1 < 0 || c1 >= num_channels || c2 < 0 || c2 >= num_channels || c2 < c1 {
        return Err(StatusCode::GenericError);
    }
    Ok(())
}
pub fn meta_squeeze(image: &mut Image, parameters: &mut Vec<SqueezeParams>) -> Status {
    if parameters.is_empty() {
        default_squeeze_parameters(parameters, image);
    }
    for p in parameters {
        check_meta_squeeze_params(p, image.channel.len() as i32)?;
        let begin = p.begin_c;
        let end = begin.wrapping_add(p.num_c).wrapping_sub(1);
        if (begin as usize) < image.nb_meta_channels {
            if end as usize >= image.nb_meta_channels || !p.in_place {
                return Err(StatusCode::GenericError);
            }
            image.nb_meta_channels = image.nb_meta_channels.wrapping_add(p.num_c as usize);
        }
        let offset = if p.in_place {
            end.wrapping_add(1)
        } else {
            image.channel.len() as u32
        };
        for c in begin..=end {
            let channel = &mut image.channel[c as usize];
            if channel.hshift > 30 || channel.vshift > 30 {
                return Err(StatusCode::GenericError);
            }
            let (mut w, mut h) = (channel.w, channel.h);
            if w == 0 || h == 0 {
                return Err(StatusCode::GenericError);
            }
            if p.horizontal {
                channel.w = w.wrapping_add(1) / 2;
                if channel.hshift >= 0 {
                    channel.hshift += 1;
                }
                w = w.wrapping_sub(w.wrapping_add(1) / 2);
            } else {
                channel.h = h.wrapping_add(1) / 2;
                if channel.vshift >= 0 {
                    channel.vshift += 1;
                }
                h = h.wrapping_sub(h.wrapping_add(1) / 2);
            }
            channel.shrink()?;
            let mut placeholder = Channel::create(w, h)?;
            placeholder.hshift = channel.hshift;
            placeholder.vshift = channel.vshift;
            placeholder.component = channel.component;
            image.channel.insert(
                offset as usize + c.wrapping_sub(begin) as usize,
                placeholder,
            );
        }
    }
    Ok(())
}
