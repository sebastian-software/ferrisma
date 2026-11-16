// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! EstimateCost of lib/jxl/enc_modular_simd.cc for the NEON and AVX2 targets
//! (port ADR 0004, "Encoder path"): the source's vector branch (`HWY_TARGET
//! != HWY_SCALAR`) with `estimate_cost_detail::ContextMap()`, written once
//! against [`SimdF32`] (`HWY_FULL(int32_t)`, `HWY_FULL(uint32_t)` and
//! `HWY_FULL(float)`: four NEON, eight AVX2 lanes). It is a different
//! algorithm from the `HWY_SCALAR` branch (`enc_modular_simd`, which stays
//! the scalar path): a clamped-gradient residual per lane, the context from
//! the capped `max_diff` through the context map, the token of the default
//! HybridUintConfig formed from the float exponent, and `Condition` plus
//! `ShannonEntropy` per histogram (the in-namespace SIMD ones of
//! `enc_cluster_simd`).
//!
//! Integer lanes only, except the `ConvertTo(df, packed_fixed)` of values
//! below 2^22, which every conversion gives exactly. The unsigned `Gt`,
//! `Ge`, `Lt` and `Min` are formed exactly as in `enc_ans_simd`. The rows
//! are loaded in whole vectors up to `RoundUpTo(w, Lanes)` inside the row
//! padding; lanes past `w` never reach a histogram (`FastAdd` runs over
//! `x < w`) or the extra bits (`tail_mask`). The source's scratch buffer lets
//! `primer` alias `max_diff_row` and `top_primer` alias `token_row`; every
//! vector step loads the primers before it stores those rows, so separate
//! buffers give the same words.

use super::base::status::StatusOr;
use super::enc_ans_params::Histogram;
use super::enc_ans_simd::{ge_u32, gt_u32, lt_u32, store_u32};
use super::enc_cluster_simd::{histogram_condition_d, histogram_entropy_d};
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::modular::modular_image::Image;

/// `estimate_cost_detail::kLastThreshold`.
pub const LAST_THRESHOLD: usize = 501;
/// `estimate_cost_detail::kLastCtx`.
pub const LAST_CTX: usize = 16;

/// `estimate_cost_detail::ContextMap()`: the source's literal table.
pub const CONTEXT_MAP: [u8; LAST_THRESHOLD] = [
    0, 1, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 8,
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 10, 10, 10, 10, 10, 10, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12,
    12, 12, 12, 12, 12, 12, 12, 12, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 16,
];

#[inline(always)]
fn min_u32<D: SimdF32>(d: D, a: D::I, b: D::I) -> D::I {
    let msb = d.set_i32(i32::MIN);
    d.xor_i32(d.min_i32(d.xor_i32(a, msb), d.xor_i32(b, msb)), msb)
}
#[inline(always)]
fn not_i32<D: SimdF32>(d: D, v: D::I) -> D::I {
    d.xor_i32(v, d.set_i32(-1))
}
#[inline(always)]
fn sum_of_lanes_u32<D: SimdF32>(d: D, v: D::I) -> u32 {
    let mut lanes = [0i32; 16];
    d.store_i32(v, &mut lanes[..D::LANES]);
    lanes[..D::LANES]
        .iter()
        .fold(0u32, |sum, &x| sum.wrapping_add(x as u32))
}

/// The packed residual's token and `eb_fixed`, shared by both row loops.
#[inline(always)]
fn token_of<D: SimdF32>(du: D, ures: D::I) -> (D::I, D::I) {
    let k_one = du.set_i32(1);
    let k_split = du.set_i32(16);
    let k_exp_offset2 = du.set_i32(129); // 127 + 2
    let k_token_bias = du.set_i32(8);
    let k_token_mul = du.set_i32(4);
    let k_msb_mask = du.set_i32(3);
    let k_large_threshold = du.set_i32((1 << 22) - 1);
    const LARGE_SHIFT_VAL: i32 = 10;
    let k_large_shift = du.set_i32(LARGE_SHIFT_VAL);
    let packed = du.xor_i32(
        du.shift_left_i32::<1>(ures),
        du.sub_i32(du.shift_right_u32::<31>(not_i32(du, ures)), k_one),
    );
    let is_large = gt_u32(du, packed, k_large_threshold);
    let packed_shifted = du.shift_right_u32::<LARGE_SHIFT_VAL>(packed);
    let not_literal = ge_u32(du, packed, k_split);
    let packed_fixed = du.if_then_else_i32(is_large, packed_shifted, packed);
    let v = du.bitcast_to_i32(du.convert_to_f32(packed_fixed));
    let eb_raw = du.sub_i32(du.shift_right_u32::<23>(v), k_exp_offset2);
    let eb = du.if_then_else_i32(is_large, du.add_i32(eb_raw, k_large_shift), eb_raw);
    let token = du.add_i32(
        du.add_i32(k_token_bias, du.mul_i32(eb, k_token_mul)),
        du.and_i32(du.shift_right_u32::<21>(v), k_msb_mask),
    );
    let eb_fixed = du.if_then_else_i32(not_literal, eb, du.set_i32(0));
    let token_fixed = du.if_then_else_i32(not_literal, token, packed);
    (token_fixed, eb_fixed)
}

#[inline(always)]
pub fn estimate_cost_d<D: SimdF32>(d: D, img: &Image) -> StatusOr<f32> {
    let lanes = D::LANES;
    let mut histo_cost = 0usize;
    let mut histo_cost_frac = 0.0f32;
    let ctx_map = &CONTEXT_MAP;
    let k_max_diff_cap = d.set_i32((LAST_THRESHOLD - 1) as i32);
    let k_lanes = d.set_i32(lanes as i32);
    let mut iota = [0i32; 16];
    for (k, lane) in iota[..lanes].iter_mut().enumerate() {
        *lane = k as i32;
    }
    let k_iota = d.load_i32(&iota[..lanes]);

    let mut max_w = 0usize;
    for ch in &img.channel {
        if ch.h == 0 {
            continue;
        }
        max_w = max_w.max(ch.w);
    }
    max_w = max_w.div_ceil(lanes) * lanes;
    max_w = max_w.max(2 * lanes);
    let mut max_diff_row = vec![0u32; max_w];
    let mut token_row = vec![0u32; max_w];
    let mut primer = vec![0i32; max_w];
    let mut top_primer = vec![0i32; max_w];

    let mut histo: [Histogram; LAST_CTX + 1] = std::array::from_fn(|_| Histogram::default());
    let mut extra_bits_lanes = d.set_i32(0);
    for ch in &img.channel {
        if ch.h == 0 || ch.w == 0 {
            continue;
        }
        for h in &mut histo {
            h.ensure_capacity(32 * 4);
        }
        let r = ch.row(0);
        primer[0] = 0;
        d.store_i32(d.load_i32(r), &mut primer[1..]);
        let mut pos = k_iota;
        let last_pos = d.set_i32(ch.w as u32 as i32);
        // `last` is `primer` for the first step, then `r + x + Lanes - 1`.
        let mut x = 0;
        while x < ch.w {
            let left = if x == 0 {
                d.load_i32(&primer)
            } else {
                d.load_i32(&r[x - 1..])
            };
            let central = d.load_i32(&r[x..]);
            let ures = d.sub_i32(central, left);
            let (token_fixed, eb_fixed) = token_of(d, ures);
            let tail_mask = lt_u32(d, pos, last_pos);
            extra_bits_lanes = d.add_i32(
                extra_bits_lanes,
                d.if_then_else_i32(tail_mask, eb_fixed, d.set_i32(0)),
            );
            store_u32(d, token_fixed, &mut token_row[x..]);
            pos = d.add_i32(pos, k_lanes);
            x += lanes;
        }
        for &token in &token_row[..ch.w] {
            histo[0].fast_add(token as usize);
        }
        for y in 1..ch.h {
            let r = ch.row(y);
            let t = ch.row(y - 1);
            primer[0] = t[0];
            d.store_i32(d.load_i32(r), &mut primer[1..]);
            top_primer[0] = t[0];
            d.store_i32(d.load_i32(t), &mut top_primer[1..]);
            pos = k_iota;
            let mut x = 0;
            while x < ch.w {
                let (left, topleft) = if x == 0 {
                    (d.load_i32(&primer), d.load_i32(&top_primer))
                } else {
                    (d.load_i32(&r[x - 1..]), d.load_i32(&t[x - 1..]))
                };
                let central = d.load_i32(&r[x..]);
                let top = d.load_i32(&t[x..]);
                let l_ge_t = d.xor_i32(d.lt_i32(left, top), d.set_i32(-1));
                let m = d.if_then_else_i32(l_ge_t, top, left);
                let mm = d.if_then_else_i32(l_ge_t, left, top);
                let maxx = d.max_i32(topleft, mm);
                let minn = d.min_i32(topleft, m);
                let max_diff = d.sub_i32(maxx, minn);
                store_u32(
                    d,
                    min_u32(d, max_diff, k_max_diff_cap),
                    &mut max_diff_row[x..],
                );
                let overshoot = d.lt_i32(topleft, m);
                let undershoot = d.gt_i32(topleft, mm);
                let grad = d.sub_i32(d.add_i32(top, left), topleft);
                let prediction =
                    d.if_then_else_i32(undershoot, m, d.if_then_else_i32(overshoot, mm, grad));
                let ures = d.sub_i32(central, prediction);
                let (token_fixed, eb_fixed) = token_of(d, ures);
                let tail_mask = lt_u32(d, pos, last_pos);
                extra_bits_lanes = d.add_i32(
                    extra_bits_lanes,
                    d.if_then_else_i32(tail_mask, eb_fixed, d.set_i32(0)),
                );
                store_u32(d, token_fixed, &mut token_row[x..]);
                pos = d.add_i32(pos, k_lanes);
                x += lanes;
            }
            for x in 0..ch.w {
                let ctx = usize::from(ctx_map[max_diff_row[x] as usize]);
                histo[ctx].fast_add(token_row[x] as usize);
            }
        }
        for h in &mut histo {
            histogram_condition_d(d, h);
            histogram_entropy_d(d, h);
            let f_cost = h.entropy;
            let i_cost = f_cost as usize;
            histo_cost = histo_cost.wrapping_add(i_cost);
            histo_cost_frac += f_cost - i_cost as f32;
            h.clear();
        }
    }
    let extra_bits = sum_of_lanes_u32(d, extra_bits_lanes) as usize;
    let total_cost = extra_bits
        .wrapping_add(histo_cost)
        .wrapping_add(histo_cost_frac as usize);
    Ok(total_cost as f32)
}

struct EstimateCost<'a>(&'a Image);
impl Kernel for EstimateCost<'_> {
    type Output = StatusOr<f32>;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> StatusOr<f32> {
        estimate_cost_d(d, self.0)
    }
}

/// `HWY_DYNAMIC_DISPATCH(EstimateCost)` at a SIMD target.
pub fn estimate_cost_simd(target: Target, img: &Image) -> StatusOr<f32> {
    dispatch(target, EstimateCost(img))
}
