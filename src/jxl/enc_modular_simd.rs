// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! EstimateCost from lib/jxl/enc_modular_simd.cc, using the original one-lane
//! Highway configuration. SIMD kernels/context maps remain unported. Residual
//! subtraction retains the source's defined i32 arithmetic precondition.

use super::base::status::StatusOr;
use super::dec_ans::HybridUintConfig;
use super::enc_ans_params::Histogram;
use super::modular::encoding::context_predict::clamped_gradient;
use super::modular::modular_image::Image;
use super::pack_signed::pack_signed;

pub fn estimate_cost(img: &Image) -> StatusOr<f32> {
    let mut histo_cost = 0usize;
    let mut histo_cost_frac = 0.0f32;
    let mut extra_bits = 0usize;
    let config = HybridUintConfig::default();
    const CUTOFFS: [u32; 17] = [
        0, 1, 3, 5, 7, 11, 15, 23, 31, 47, 63, 95, 127, 191, 255, 392, 500,
    ];
    let mut histo: [Histogram; 18] = std::array::from_fn(|_| Histogram::default());
    for ch in &img.channel {
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
                let max_diff = (left.max(top).max(topleft) - left.min(top).min(topleft)) as usize;
                let mut ctx = 0;
                for c in CUTOFFS {
                    ctx += usize::from(max_diff < c as usize);
                }
                let res = row[x]
                    .checked_sub(clamped_gradient(top as i32, left as i32, topleft as i32))
                    .expect("source EstimateCost residual must fit i32");
                let (token, nbits, _) = config.encode(pack_signed(res));
                histo[ctx].add(token as usize);
                extra_bits = extra_bits.wrapping_add(nbits as usize);
            }
        }
        for h in &mut histo {
            let f_cost = h.shannon_entropy();
            let i_cost = f_cost as usize;
            histo_cost = histo_cost.wrapping_add(i_cost);
            histo_cost_frac += f_cost - i_cost as f32;
            h.clear();
        }
    }
    let total_cost = extra_bits
        .wrapping_add(histo_cost)
        .wrapping_add(histo_cost_frac as usize);
    Ok(total_cost as f32)
}
