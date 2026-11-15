// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The Highway exports of lib/jxl/enc_cluster.cc for the NEON and AVX2
//! targets (port ADR 0004, "Encoder path"): `HistogramCondition`,
//! `HistogramEntropy` and `FastClusterHistograms` with the in-namespace
//! `Entropy`, `HistogramDistance`, `ClusterDistance` and
//! `HistogramKLDivergence`, written once against [`SimdF32`].
//! `HWY_CAPPED(float, Histogram::kRounding)` and its `int32_t` twin are the
//! full vectors on both targets (four NEON, eight AVX2 lanes), and
//! `FastLog2f` is fast_math-inl.h's vector form (`simd_transfer`'s). Compare
//! them only with the original's SIMD build at the same target; the scalar
//! (one-lane) translation stays in `enc_cluster`.
//!
//! The vector loops load whole vectors (`LoadU`) at `i < counts.size()`, as
//! in the source, so a histogram's length must be a multiple of the lane
//! count: `Add` and `EnsureCapacity` keep it a multiple of `kRounding` (8)
//! and `HistogramCondition` cuts it to a multiple of the lane count. Where
//! the original would read past the vector's end, the port's bounds-checked
//! load panics instead. Integer counts are added with wrapping `Add`,
//! `AllTrue` and the integer `SumOfLanes` are formed from the lanes (both are
//! exact on wrapping 32-bit words in any order); the float `SumOfLanes` is
//! the target's own reduction order. No scalar statement of these functions
//! has a contractible product and sum, so the scalar statements are those
//! of `enc_cluster` (`NumSymbols` is the shared scalar function).
use super::base::status::{Status, StatusCode};
use super::enc_ans_params::Histogram;
use super::enc_cluster::num_symbols;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::render_pipeline::simd_transfer::fast_log2f;

/// `Entropy(count, inv_total, total)`: `IfThenZeroElse(Eq(count, total),
/// Sub(zero, Mul(count, FastLog2f(d, Mul(inv_total, count)))))`.
#[inline(always)]
fn entropy<D: SimdF32>(d: D, count: D::V, inv_total: D::V, total: D::V) -> D::V {
    let zero = d.set(0.0);
    d.if_then_zero_else(
        d.eq(count, total),
        d.sub(zero, d.mul(count, fast_log2f(d, d.mul(inv_total, count)))),
    )
}

/// `AllTrue(di, Eq(counts, kZero))`.
#[inline(always)]
fn all_zero<D: SimdF32>(di: D, counts: D::I) -> bool {
    let mut lanes = [0i32; 16];
    di.store_i32(di.eq_i32(counts, di.set_i32(0)), &mut lanes[..D::LANES]);
    lanes[..D::LANES].iter().all(|&m| m != 0)
}

/// `GetLane(SumOfLanes(di, v))` of 32-bit lanes: wrapping, so exact in any
/// order.
#[inline(always)]
fn sum_of_lanes_i32<D: SimdF32>(di: D, v: D::I) -> i32 {
    let mut lanes = [0i32; 16];
    di.store_i32(v, &mut lanes[..D::LANES]);
    lanes[..D::LANES]
        .iter()
        .fold(0i32, |sum, &x| sum.wrapping_add(x))
}

#[inline(always)]
pub fn histogram_condition_d<D: SimdF32>(di: D, a: &mut Histogram) {
    let k_zero = di.set_i32(0);
    let mut total = k_zero;
    let mut nz_pos = -(D::LANES as i32);
    let mut i = 0;
    while i < a.counts.len() {
        let counts = di.load_i32(&a.counts[i..]);
        let nz = !all_zero(di, counts);
        total = di.add_i32(total, counts);
        if nz {
            nz_pos = i as i32;
        }
        i += D::LANES;
    }
    // resize(nz_pos + Lanes(di)): the int sum converts to size_t.
    a.counts
        .resize(nz_pos.wrapping_add(D::LANES as i32) as isize as usize, 0);
    // The int32_t lane converts to size_t (sign extension).
    a.total_count = sum_of_lanes_i32(di, total) as isize as usize;
}

#[inline(always)]
pub fn histogram_entropy_d<D: SimdF32>(df: D, a: &mut Histogram) {
    a.entropy = 0.0;
    if a.total_count == 0 {
        return;
    }
    let inv_tot = df.set(1.0f32 / a.total_count as f32);
    let mut entropy_lanes = df.zero();
    let total = df.set(a.total_count as f32);
    let mut i = 0;
    while i < a.counts.len() {
        let counts = df.load_i32(&a.counts[i..]);
        entropy_lanes = df.add(
            entropy_lanes,
            entropy(df, df.convert_to_f32(counts), inv_tot, total),
        );
        i += D::LANES;
    }
    a.entropy += df.get_lane(df.sum_of_lanes(entropy_lanes));
}

#[inline(always)]
pub fn histogram_distance_d<D: SimdF32>(df: D, a: &Histogram, b: &Histogram) -> f32 {
    if a.total_count == 0 || b.total_count == 0 {
        return 0.0;
    }
    let inv_tot = df.set(1.0f32 / a.total_count.wrapping_add(b.total_count) as f32);
    let mut distance_lanes = df.zero();
    let total = df.set(a.total_count.wrapping_add(b.total_count) as f32);
    let mut i = 0;
    while i < a.counts.len().max(b.counts.len()) {
        let a_counts = if a.counts.len() > i {
            df.load_i32(&a.counts[i..])
        } else {
            df.set_i32(0)
        };
        let b_counts = if b.counts.len() > i {
            df.load_i32(&b.counts[i..])
        } else {
            df.set_i32(0)
        };
        let counts = df.convert_to_f32(df.add_i32(a_counts, b_counts));
        distance_lanes = df.add(distance_lanes, entropy(df, counts, inv_tot, total));
        i += D::LANES;
    }
    let total_distance = df.get_lane(df.sum_of_lanes(distance_lanes));
    total_distance - a.entropy - b.entropy
}

#[inline(always)]
pub fn cluster_distance_d<D: SimdF32>(
    df: D,
    a: &Histogram,
    b: &Histogram,
    prefix_codes: bool,
) -> f32 {
    let distance = histogram_distance_d(df, a, b);
    if !prefix_codes || a.total_count == 0 || b.total_count == 0 || num_symbols(a, Some(b)) <= 1 {
        return distance;
    }
    let mut single = 0.0;
    if num_symbols(a, None) <= 1 {
        single += a.total_count as f32;
    }
    if num_symbols(b, None) <= 1 {
        single += b.total_count as f32;
    }
    // std::max returns its first argument on equality or unordered comparison.
    if distance < single { single } else { distance }
}

#[inline(always)]
pub fn histogram_kl_divergence_d<D: SimdF32>(df: D, actual: &Histogram, coding: &Histogram) -> f32 {
    if actual.total_count == 0 {
        return 0.0;
    }
    if coding.total_count == 0 {
        return f32::INFINITY;
    }
    let coding_inv = df.set(1.0f32 / coding.total_count as f32);
    let mut cost_lanes = df.zero();
    let zero_i = df.set_i32(0);
    let mut i = 0;
    while i < actual.counts.len() {
        let counts = df.load_i32(&actual.counts[i..]);
        let coding_counts = if coding.counts.len() > i {
            df.load_i32(&coding.counts[i..])
        } else {
            zero_i
        };
        let coding_probs = df.mul(df.convert_to_f32(coding_counts), coding_inv);
        let neg_coding_cost = df.bitcast_to_f32(df.if_then_zero_else_i32(
            df.eq_i32(counts, zero_i),
            df.if_then_else_i32(
                df.eq_i32(coding_counts, zero_i),
                df.bitcast_to_i32(df.set(f32::NEG_INFINITY)),
                df.bitcast_to_i32(fast_log2f(df, coding_probs)),
            ),
        ));
        cost_lanes = df.neg_mul_add(df.convert_to_f32(counts), neg_coding_cost, cost_lanes);
        i += D::LANES;
    }
    let total_cost = df.get_lane(df.sum_of_lanes(cost_lanes));
    total_cost - actual.entropy
}

/// `FastClusterHistograms`: the control flow of `enc_cluster`'s scalar
/// translation with the vector helpers above.
#[inline(always)]
pub fn fast_cluster_histograms_d<D: SimdF32>(
    d: D,
    input: &mut [Histogram],
    max_histograms: usize,
    prefix_codes: bool,
    out: &mut Vec<Histogram>,
    symbols: &mut Vec<u32>,
) -> Status {
    let prev_histograms = out.len();
    out.reserve(max_histograms.saturating_sub(out.len()));
    symbols.clear();
    symbols.resize(input.len(), max_histograms as u32);
    let mut dists = vec![f32::MAX; input.len()];
    let mut largest_idx = 0;
    for i in 0..input.len() {
        if input[i].total_count == 0 {
            symbols[i] = 0;
            dists[i] = 0.0;
            continue;
        }
        histogram_entropy_d(d, &mut input[i]);
        if input[i].total_count > input[largest_idx].total_count {
            largest_idx = i;
        }
    }
    if prev_histograms > 0 {
        for histo in out.iter_mut().take(prev_histograms) {
            histogram_entropy_d(d, histo);
        }
        for i in 0..input.len() {
            if dists[i] == 0.0 {
                continue;
            }
            for previous in out.iter().take(prev_histograms) {
                let value = histogram_kl_divergence_d(d, &input[i], previous);
                // C++ std::min(value, dists[i]) preserves the first equal value.
                dists[i] = if dists[i] < value { dists[i] } else { value };
            }
        }
        if !dists.is_empty() {
            // std::max_element: the first of the largest.
            let mut max_dist = 0;
            for i in 1..dists.len() {
                if dists[max_dist] < dists[i] {
                    max_dist = i;
                }
            }
            if dists[max_dist] > 0.0 {
                largest_idx = max_dist;
            }
        }
    }
    const MIN_DISTANCE_FOR_DISTINCT: f32 = 48.0;
    while out.len() < max_histograms {
        symbols[largest_idx] = out.len() as u32;
        out.push(input[largest_idx].clone());
        dists[largest_idx] = 0.0;
        largest_idx = 0;
        for i in 0..input.len() {
            if dists[i] == 0.0 {
                continue;
            }
            let value = cluster_distance_d(d, &input[i], out.last().unwrap(), prefix_codes);
            dists[i] = if dists[i] < value { dists[i] } else { value };
            if dists[i] > dists[largest_idx] {
                largest_idx = i;
            }
        }
        if dists[largest_idx] < MIN_DISTANCE_FOR_DISTINCT {
            break;
        }
    }
    for i in 0..input.len() {
        if symbols[i] != max_histograms as u32 {
            continue;
        }
        let mut best = 0;
        let mut best_dist = f32::MAX;
        for (j, histo) in out.iter().enumerate() {
            let dist = if j < prev_histograms {
                histogram_kl_divergence_d(d, &input[i], histo)
            } else {
                cluster_distance_d(d, &input[i], histo, prefix_codes)
            };
            if dist < best_dist {
                best = j;
                best_dist = dist;
            }
        }
        // JXL_ENSURE(best_dist < std::numeric_limits<float>::max()).
        if best_dist >= f32::MAX || best_dist.is_nan() {
            return Err(StatusCode::GenericError);
        }
        if best >= prev_histograms {
            out[best].add_histogram(&input[i]);
            histogram_entropy_d(d, &mut out[best]);
        }
        symbols[i] = best as u32;
    }
    Ok(())
}

struct Condition<'a>(&'a mut Histogram);
impl Kernel for Condition<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        histogram_condition_d(d, self.0);
    }
}

struct Entropy<'a>(&'a mut Histogram);
impl Kernel for Entropy<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        histogram_entropy_d(d, self.0);
    }
}

struct FastCluster<'a> {
    input: &'a mut [Histogram],
    max_histograms: usize,
    prefix_codes: bool,
    out: &'a mut Vec<Histogram>,
    symbols: &'a mut Vec<u32>,
}
impl Kernel for FastCluster<'_> {
    type Output = Status;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> Status {
        fast_cluster_histograms_d(
            d,
            self.input,
            self.max_histograms,
            self.prefix_codes,
            self.out,
            self.symbols,
        )
    }
}

struct Distance<'a> {
    a: &'a Histogram,
    b: &'a Histogram,
    kl: bool,
}
impl Kernel for Distance<'_> {
    type Output = f32;
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) -> f32 {
        if self.kl {
            histogram_kl_divergence_d(d, self.a, self.b)
        } else {
            histogram_distance_d(d, self.a, self.b)
        }
    }
}

/// `HWY_DYNAMIC_DISPATCH(HistogramCondition)` at a SIMD target.
pub fn histogram_condition_simd(target: Target, a: &mut Histogram) {
    dispatch(target, Condition(a));
}

/// `HWY_DYNAMIC_DISPATCH(HistogramEntropy)` at a SIMD target.
pub fn histogram_entropy_simd(target: Target, a: &mut Histogram) {
    dispatch(target, Entropy(a));
}

/// `HWY_DYNAMIC_DISPATCH(FastClusterHistograms)` at a SIMD target.
pub fn fast_cluster_histograms_simd(
    target: Target,
    input: &mut [Histogram],
    max_histograms: usize,
    prefix_codes: bool,
    out: &mut Vec<Histogram>,
    symbols: &mut Vec<u32>,
) -> Status {
    dispatch(
        target,
        FastCluster {
            input,
            max_histograms,
            prefix_codes,
            out,
            symbols,
        },
    )
}

/// The target namespace's `HistogramDistance` (not an export; for tests).
pub fn histogram_distance_simd(target: Target, a: &Histogram, b: &Histogram) -> f32 {
    dispatch(target, Distance { a, b, kl: false })
}

/// The target namespace's `HistogramKLDivergence` (not an export; for tests).
pub fn histogram_kl_divergence_simd(target: Target, actual: &Histogram, coding: &Histogram) -> f32 {
    dispatch(
        target,
        Distance {
            a: actual,
            b: coding,
            kl: true,
        },
    )
}
