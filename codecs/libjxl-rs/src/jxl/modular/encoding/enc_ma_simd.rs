// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! FindBestSplit of lib/jxl/modular/encoding/enc_ma.cc for the NEON and AVX2
//! targets (port ADR 0004, "Encoder path"), with its in-namespace
//! EstimateBits and Padded written once against [`SimdF32`]
//! (`HWY_FULL(float)` and `HWY_FULL(int32_t)`: four NEON, eight AVX2 lanes).
//! `enc_ma` keeps the one-lane translation and calls [`find_best_split_simd`]
//! when the encoder selected a SIMD target, as
//! `HWY_DYNAMIC_DISPATCH(FindBestSplit)` does.
//!
//! The vector parts are EstimateBits (the integer total by wrapping `Add`
//! and `SumOfLanes`, exact in any order; the float bits by the target's
//! `SumOfLanes`) and `Padded`, which rounds the histogram length up to the
//! lane count, so the padded zero counts and the reduction order follow the
//! target. The rest is scalar code inside `HWY_NAMESPACE`, compiled with
//! the target's FMA: it has no product added to or subtracted from another
//! term (`fast_decode_multiplier * best->Cost()` is compared, and `base_bits
//! / 2 - threshold` divides), so it is `enc_ma`'s scalar control flow,
//! statement for statement, with the helpers shared.

use super::context_predict::WP_PROP;
use super::dec_ma::Tree;
use super::enc_ma::{
    CostInfo, IntersectionType, SplitInfo, TreeSamples, box_intersects,
    collect_extra_bits_increase, make_split_node, split_tree_samples,
};
use crate::jxl::ans_params::ANS_TAB_SIZE;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};
use crate::jxl::modular::options::Predictor;
use crate::jxl::modular::options::{ModularMultiplierInfo, NUM_STATIC_PROPERTIES, StaticPropRange};
use crate::jxl::render_pipeline::simd_transfer::fast_log2f;

/// EstimateBits(counts, num_symbols): `num_symbols` is a multiple of the
/// lane count (`Padded`).
#[inline(always)]
pub fn estimate_bits_d<D: SimdF32>(d: D, counts: &[i32], num_symbols: usize) -> f32 {
    let mut total_v = d.set_i32(0);
    let mut i = 0;
    while i < num_symbols {
        let counts_iv = d.load_i32(&counts[i..]);
        total_v = d.add_i32(total_v, counts_iv);
        i += D::LANES;
    }
    // SumOfLanes(di, total_v): the wrapping sum in every lane.
    let mut lanes = [0i32; 16];
    d.store_i32(total_v, &mut lanes[..D::LANES]);
    let total = lanes[..D::LANES]
        .iter()
        .fold(0i32, |sum, &x| sum.wrapping_add(x));
    let total_v = d.set_i32(total);

    let minprob = d.set(1.0f32 / ANS_TAB_SIZE as f32);
    let inv_total = d.set(1.0f32 / total as f32);
    let mut bits_lanes = d.zero();
    let mut i = 0;
    while i < num_symbols {
        let counts_iv = d.load_i32(&counts[i..]);
        let counts_fv = d.convert_to_f32(counts_iv);
        let probs = d.mul(counts_fv, inv_total);
        let mprobs = d.max(probs, minprob);
        let nbps = d.if_then_zero_else_i32(
            d.eq_i32(counts_iv, total_v),
            d.bitcast_to_i32(fast_log2f(d, mprobs)),
        );
        bits_lanes = d.sub(bits_lanes, d.mul(counts_fv, d.bitcast_to_f32(nbps)));
        i += D::LANES;
    }
    d.get_lane(d.sum_of_lanes(bits_lanes))
}

#[inline(always)]
pub fn find_best_split_d<D: SimdF32>(
    d: D,
    samples: &mut TreeSamples,
    threshold: f32,
    mul_info: &[ModularMultiplierInfo],
    initial_range: StaticPropRange,
    fast_decode_multiplier: f32,
    tree: &mut Tree,
) {
    struct NodeInfo {
        pos: usize,
        begin: usize,
        end: usize,
        range: StaticPropRange,
    }
    let mut nodes = vec![NodeInfo {
        pos: 0,
        begin: 0,
        end: samples.num_distinct_samples(),
        range: initial_range,
    }];
    let num_predictors = samples.num_predictors();
    let num_properties = samples.num_properties();
    while let Some(info) = nodes.pop() {
        let (pos, begin, end, static_range) = (info.pos, info.begin, info.end, info.range);
        if begin == end {
            continue;
        }
        const STATIC_CONSTANT: usize = 0;
        const STATIC: usize = 1;
        const NONSTATIC: usize = 2;
        const NOWP: usize = 3;
        let mut splits = [SplitInfo::default(); 4];
        let mut max_symbols = 0usize;
        for pred in 0..num_predictors {
            for i in begin..end {
                let tok = samples.token(pred, i) as u32;
                max_symbols = max_symbols.max(tok.wrapping_add(1) as usize);
            }
        }
        // Padded(max_symbols): RoundUpTo(max_symbols, Lanes(df)).
        let max_symbols = max_symbols.div_ceil(D::LANES) * D::LANES;
        let mut counts = vec![0i32; max_symbols * num_predictors];
        let mut tot_extra_bits = vec![0u32; num_predictors];
        for (pred, total) in tot_extra_bits.iter_mut().enumerate() {
            let mut extra_bits = 0usize;
            let rtokens = samples.rtokens(pred);
            for (i, rt) in rtokens.iter().enumerate().take(end).skip(begin) {
                let count = samples.count(i);
                let eb = usize::from(rt.nbits).wrapping_mul(count);
                let index = pred * max_symbols + usize::from(rt.tok);
                counts[index] = counts[index].wrapping_add(count as i32);
                extra_bits = extra_bits.wrapping_add(eb);
            }
            *total = extra_bits as u32;
        }
        let pred = samples.predictor_index(tree[pos].predictor);
        let base_bits = estimate_bits_d(d, &counts[pred * max_symbols..], max_symbols)
            + tot_extra_bits[pred] as f32;
        let mut best = NONSTATIC;
        let mut forced = SplitInfo::default();
        let mut use_forced = false;
        for mmi in mul_info {
            let (mut axis, mut val) = (0, 0);
            let t = box_intersects(static_range, mmi.range, &mut axis, &mut val);
            if t == IntersectionType::None {
                continue;
            }
            if t == IntersectionType::Inside {
                tree[pos].multiplier = mmi.multiplier;
                break;
            }
            if t == IntersectionType::Partial {
                forced.val = samples.quantize_static_property(axis, val as i32);
                forced.prop = axis as usize;
                forced.lcost = base_bits / 2.0 - threshold;
                forced.rcost = forced.lcost;
                forced.lpred = tree[pos].predictor;
                forced.rpred = tree[pos].predictor;
                use_forced = true;
                forced.pos = begin;
                if forced.prop < samples.num_static_props() {
                    for x in begin..end {
                        if samples.property::<true>(forced.prop, x) <= forced.val as usize {
                            forced.pos += 1;
                        }
                    }
                } else {
                    let prop = forced.prop - samples.num_static_props();
                    for x in begin..end {
                        if samples.property::<false>(prop, x) <= forced.val as usize {
                            forced.pos += 1;
                        }
                    }
                }
                break;
            }
        }
        if !use_forced {
            let mut prop_value_used_count = Vec::new();
            let mut count_increase = Vec::new();
            let mut extra_bits_increase = Vec::new();
            let (mut costs_l, mut costs_r) = (Vec::<CostInfo>::new(), Vec::<CostInfo>::new());
            let mut counts_above = vec![0i32; max_symbols];
            let mut counts_below = vec![0i32; max_symbols];
            let change_pred_penalty = 800.0f32 / (100.0f32 + threshold);
            let mut prop = 0;
            while prop < num_properties && base_bits > threshold {
                costs_l.clear();
                costs_r.clear();
                let prop_size = samples.num_property_values(prop);
                if extra_bits_increase.len() < prop_size {
                    count_increase.resize(prop_size * max_symbols, 0i32);
                    extra_bits_increase.resize(prop_size, 0usize);
                }
                prop_value_used_count.clear();
                prop_value_used_count.resize(prop_size, 0i32);
                let mut first_used = prop_size;
                let mut last_used = 0;
                if prop < samples.num_static_props() {
                    for i in begin..end {
                        let p = samples.property::<true>(prop, i);
                        prop_value_used_count[p] = prop_value_used_count[p].wrapping_add(1);
                        last_used = last_used.max(p);
                        first_used = first_used.min(p);
                    }
                } else {
                    let prop_idx = prop - samples.num_static_props();
                    for i in begin..end {
                        let p = samples.property::<false>(prop_idx, i);
                        prop_value_used_count[p] = prop_value_used_count[p].wrapping_add(1);
                        last_used = last_used.max(p);
                        first_used = first_used.min(p);
                    }
                }
                costs_l.resize(last_used - first_used, CostInfo::default());
                costs_r.resize(last_used - first_used, CostInfo::default());
                for pred in 0..num_predictors {
                    let rtokens = samples.rtokens(pred);
                    if prop < samples.num_static_props() {
                        collect_extra_bits_increase::<true>(
                            samples,
                            rtokens,
                            &mut count_increase,
                            &mut extra_bits_increase,
                            begin,
                            end,
                            prop,
                            max_symbols,
                        );
                    } else {
                        collect_extra_bits_increase::<false>(
                            samples,
                            rtokens,
                            &mut count_increase,
                            &mut extra_bits_increase,
                            begin,
                            end,
                            prop - samples.num_static_props(),
                            max_symbols,
                        );
                    }
                    counts_above
                        .copy_from_slice(&counts[pred * max_symbols..(pred + 1) * max_symbols]);
                    counts_below.fill(0);
                    let mut extra_bits_below = 0usize;
                    for i in first_used..last_used {
                        if prop_value_used_count[i] == 0 {
                            continue;
                        }
                        extra_bits_below = extra_bits_below.wrapping_add(extra_bits_increase[i]);
                        extra_bits_increase[i] = 0;
                        for sym in 0..max_symbols {
                            let index = i * max_symbols + sym;
                            counts_above[sym] =
                                counts_above[sym].wrapping_sub(count_increase[index]);
                            counts_below[sym] =
                                counts_below[sym].wrapping_add(count_increase[index]);
                            count_increase[index] = 0;
                        }
                        let rcost = estimate_bits_d(d, &counts_above, max_symbols)
                            + tot_extra_bits[pred] as f32
                            - extra_bits_below as f32;
                        let lcost = estimate_bits_d(d, &counts_below, max_symbols)
                            + extra_bits_below as f32;
                        let mut penalty = 0.0f32;
                        if samples.predictor_from_index(pred) != tree[pos].predictor
                            && tree[pos].predictor != Predictor::WEIGHTED
                        {
                            penalty = change_pred_penalty;
                        }
                        if samples.predictor_from_index(pred) == Predictor::WEIGHTED {
                            penalty = (f64::from(penalty) + 1e-8) as f32;
                        }
                        if samples.predictor_from_index(pred) == Predictor::ZERO {
                            penalty = (f64::from(penalty) - 1e-8) as f32;
                        }
                        if rcost + penalty < costs_r[i - first_used].cost() {
                            costs_r[i - first_used] = CostInfo {
                                cost: rcost,
                                extra_cost: penalty,
                                pred: samples.predictor_from_index(pred),
                            };
                        }
                        if lcost + penalty < costs_l[i - first_used].cost() {
                            costs_l[i - first_used] = CostInfo {
                                cost: lcost,
                                extra_cost: penalty,
                                pred: samples.predictor_from_index(pred),
                            };
                        }
                    }
                }
                let mut split = begin;
                for i in first_used..last_used {
                    if prop_value_used_count[i] == 0 {
                        continue;
                    }
                    split = split.wrapping_add(prop_value_used_count[i] as usize);
                    let rcost = costs_r[i - first_used].cost;
                    let lcost = costs_l[i - first_used].cost;
                    let uses_wp = samples.property_from_index(prop) == WP_PROP
                        || costs_l[i - first_used].pred == Predictor::WEIGHTED
                        || costs_r[i - first_used].pred == Predictor::WEIGHTED;
                    let zero_entropy_side = rcost == 0.0 || lcost == 0.0;
                    let best_ref =
                        if samples.property_from_index(prop) < NUM_STATIC_PROPERTIES as usize {
                            if zero_entropy_side {
                                STATIC_CONSTANT
                            } else {
                                STATIC
                            }
                        } else if uses_wp {
                            NONSTATIC
                        } else {
                            NOWP
                        };
                    if lcost + rcost < splits[best_ref].cost() {
                        splits[best_ref] = SplitInfo {
                            prop,
                            val: i as u32,
                            pos: split,
                            lcost,
                            lpred: costs_l[i - first_used].pred,
                            rcost,
                            rpred: costs_r[i - first_used].pred,
                        };
                    }
                }
                extra_bits_increase[last_used] = 0;
                for sym in 0..max_symbols {
                    count_increase[last_used * max_symbols + sym] = 0;
                }
                prop += 1;
            }
            if splits[NOWP].cost() + threshold < base_bits
                && splits[NOWP].cost() <= fast_decode_multiplier * splits[best].cost()
            {
                best = NOWP;
            }
            if splits[STATIC].cost() + threshold < base_bits
                && splits[STATIC].cost() <= fast_decode_multiplier * splits[best].cost()
            {
                best = STATIC;
            }
            if splits[STATIC_CONSTANT].cost() + threshold < base_bits {
                best = STATIC_CONSTANT;
            }
        }
        let best = if use_forced { forced } else { splits[best] };
        if best.cost() + threshold < base_bits {
            let p = samples.property_from_index(best.prop) as u32;
            let dequant = samples.unquantize_property(best.prop, best.val);
            make_split_node(pos, p as i32, dequant, best.lpred, 0, best.rpred, 0, tree);
            if best.prop < samples.num_static_props() {
                split_tree_samples::<true>(samples, begin, best.pos, end, best.prop, best.val);
            } else {
                split_tree_samples::<false>(
                    samples,
                    begin,
                    best.pos,
                    end,
                    best.prop - samples.num_static_props(),
                    best.val,
                );
            }
            let mut new_range = static_range;
            if p < NUM_STATIC_PROPERTIES as u32 {
                new_range[p as usize][1] = dequant.wrapping_add(1) as u32;
            }
            nodes.push(NodeInfo {
                pos: tree[pos].rchild as usize,
                begin,
                end: best.pos,
                range: new_range,
            });
            new_range = static_range;
            if p < NUM_STATIC_PROPERTIES as u32 {
                new_range[p as usize][0] = dequant.wrapping_add(1) as u32;
            }
            nodes.push(NodeInfo {
                pos: tree[pos].lchild as usize,
                begin: best.pos,
                end,
                range: new_range,
            });
        }
    }
}

struct FindBestSplit<'a> {
    samples: &'a mut TreeSamples,
    threshold: f32,
    mul_info: &'a [ModularMultiplierInfo],
    initial_range: StaticPropRange,
    fast_decode_multiplier: f32,
    tree: &'a mut Tree,
}
impl Kernel for FindBestSplit<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        find_best_split_d(
            d,
            self.samples,
            self.threshold,
            self.mul_info,
            self.initial_range,
            self.fast_decode_multiplier,
            self.tree,
        );
    }
}

/// `HWY_DYNAMIC_DISPATCH(FindBestSplit)` at a SIMD target.
pub fn find_best_split_simd(
    target: Target,
    samples: &mut TreeSamples,
    threshold: f32,
    mul_info: &[ModularMultiplierInfo],
    initial_range: StaticPropRange,
    fast_decode_multiplier: f32,
    tree: &mut Tree,
) {
    dispatch(
        target,
        FindBestSplit {
            samples,
            threshold,
            mul_info,
            initial_range,
            fast_decode_multiplier,
            tree,
        },
    );
}

struct EstimateBits<'a>(&'a [i32], usize);
impl Kernel for EstimateBits<'_> {
    type Output = f32;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> f32 {
        estimate_bits_d(d, self.0, self.1)
    }
}

/// The target namespace's EstimateBits (not an export; for tests).
pub fn estimate_bits_simd(target: Target, counts: &[i32], num_symbols: usize) -> f32 {
    dispatch(target, EstimateBits(counts, num_symbols))
}
