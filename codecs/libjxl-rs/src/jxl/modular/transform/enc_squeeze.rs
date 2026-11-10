// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Forward squeeze from lib/jxl/modular/transform/enc_squeeze.{h,cc}.
//! Preserve signed i32 intermediate arithmetic and the source's nonoverflowing
//! input requirement. The unused pool parameter is the null-pool boundary.

use super::squeeze::{check_meta_squeeze_params, default_squeeze_parameters, smooth_tendency};
use super::squeeze_params::SqueezeParams;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::modular::modular_image::{Channel, Image};
fn average(x: i32, y: i32) -> i32 {
    (x + y + i32::from(x > y)) >> 1
}
pub fn fwd_h_squeeze(input: &mut Image, c: i32, rc: i32) -> Status {
    let chin = &input.channel[c as usize];
    let mut chout = Channel::create_with_shifts(
        chin.w.wrapping_add(1) / 2,
        chin.h,
        chin.hshift + 1,
        chin.vshift,
    )?;
    let mut residual =
        Channel::create_with_shifts(chin.w - chout.w, chout.h, chin.hshift + 1, chin.vshift)?;
    chout.component = chin.component;
    residual.component = chin.component;
    for y in 0..chout.h {
        let p_in = chin.row(y);
        let cw = chout.w;
        let rw = residual.w;
        let p_out = chout.row_mut(y);
        let p_res = residual.row_mut(y);
        for x in 0..rw {
            let a = p_in[x * 2];
            let b = p_in[x * 2 + 1];
            let avg = average(a, b);
            p_out[x] = avg;
            let diff = a - b;
            let mut next_avg = avg;
            if x + 1 < rw {
                let c = p_in[x * 2 + 2];
                let d = p_in[x * 2 + 3];
                next_avg = average(c, d);
            } else if chin.w & 1 != 0 {
                next_avg = p_in[x * 2 + 2];
            }
            let left = if x > 0 { p_in[x * 2 - 1] } else { avg };
            let tendency =
                smooth_tendency(i64::from(left), i64::from(avg), i64::from(next_avg)) as i32;
            p_res[x] = diff - tendency;
        }
        if chin.w & 1 != 0 {
            let x = (cw - 1) as i32;
            p_out[x as usize] = p_in[x as usize * 2];
        }
    }
    input.channel[c as usize] = chout;
    input.channel.insert(rc as usize, residual);
    Ok(())
}
pub fn fwd_v_squeeze(input: &mut Image, c: i32, rc: i32) -> Status {
    let chin = &input.channel[c as usize];
    let mut chout = Channel::create_with_shifts(
        chin.w,
        chin.h.wrapping_add(1) / 2,
        chin.hshift,
        chin.vshift + 1,
    )?;
    let mut residual =
        Channel::create_with_shifts(chin.w, chin.h - chout.h, chin.hshift, chin.vshift + 1)?;
    chout.component = chin.component;
    residual.component = chin.component;
    for y in 0..residual.h {
        let cw = chout.w;
        let rh = residual.h;
        let p_out = chout.row_mut(y);
        let p_res = residual.row_mut(y);
        for x in 0..cw {
            let a = chin.row(y * 2)[x];
            let b = chin.row(y * 2 + 1)[x];
            let avg = average(a, b);
            p_out[x] = avg;
            let diff = a - b;
            let mut next_avg = avg;
            if y + 1 < rh {
                let c = chin.row(y * 2 + 2)[x];
                let d = chin.row(y * 2 + 3)[x];
                next_avg = average(c, d);
            } else if chin.h & 1 != 0 {
                next_avg = chin.row(y * 2 + 2)[x];
            }
            let top = if y > 0 { chin.row(y * 2 - 1)[x] } else { avg };
            let tendency =
                smooth_tendency(i64::from(top), i64::from(avg), i64::from(next_avg)) as i32;
            p_res[x] = diff - tendency;
        }
    }
    if chin.h & 1 != 0 {
        let y = chout.h - 1;
        let cw = chout.w;
        chout.row_mut(y)[..cw].copy_from_slice(&chin.row(y * 2)[..cw]);
    }
    input.channel[c as usize] = chout;
    input.channel.insert(rc as usize, residual);
    Ok(())
}
pub fn fwd_squeeze(input: &mut Image, mut parameters: Vec<SqueezeParams>) -> Status {
    if parameters.is_empty() {
        default_squeeze_parameters(&mut parameters, input);
    }
    if parameters.is_empty() {
        return Err(StatusCode::GenericError);
    }
    for p in parameters {
        check_meta_squeeze_params(&p, input.channel.len() as i32)?;
        let begin = p.begin_c;
        let end = begin.wrapping_add(p.num_c).wrapping_sub(1);
        let offset = if p.in_place {
            end.wrapping_add(1)
        } else {
            input.channel.len() as u32
        };
        for c in begin..=end {
            let rc = offset.wrapping_add(c).wrapping_sub(begin);
            if p.horizontal {
                fwd_h_squeeze(input, c as i32, rc as i32)?;
            } else {
                fwd_v_squeeze(input, c as i32, rc as i32)?;
            }
        }
    }
    Ok(())
}
