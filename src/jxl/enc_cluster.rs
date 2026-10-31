// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/enc_cluster.{h,cc}`, using its one-lane Highway
//! specialization. Mutable borrows expose the source's mutable entropy cache.
//! The priority queue retains the original reversed tuple ordering.

use super::base::fast_math::fast_log2f;
use super::base::status::{Status, StatusCode};
use super::enc_ans_params::{ClusteringType, Histogram, HistogramParams};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

fn entropy(count: f32, inv_total: f32, total: f32) -> f32 {
    // Source IfThenZeroElse evaluates both arguments, even for count == total.
    let value = 0.0 - count * fast_log2f(inv_total * count);
    if count == total { 0.0 } else { value }
}

impl Histogram {
    pub fn condition(&mut self) {
        let mut total = 0i32;
        let mut new_len = 0;
        for (i, &count) in self.counts.iter().enumerate() {
            total = total.wrapping_add(count);
            if count != 0 {
                new_len = i + 1;
            }
        }
        self.counts.resize(new_len, 0);
        self.total_count = total as usize;
    }
    pub fn shannon_entropy(&mut self) -> f32 {
        histogram_entropy(self);
        self.entropy
    }
}

fn histogram_entropy(a: &mut Histogram) {
    a.entropy = 0.0;
    if a.total_count == 0 {
        return;
    }
    let inv_tot = 1.0f32 / a.total_count as f32;
    let mut entropy_lanes = 0.0;
    let total = a.total_count as f32;
    for &count in &a.counts {
        entropy_lanes += entropy(count as f32, inv_tot, total);
    }
    a.entropy += entropy_lanes;
}

pub fn histogram_distance(a: &Histogram, b: &Histogram) -> f32 {
    if a.total_count == 0 || b.total_count == 0 {
        return 0.0;
    }
    let total = a.total_count.wrapping_add(b.total_count) as f32;
    let inv_tot = 1.0f32 / total;
    let mut distance_lanes = 0.0;
    for i in 0..a.counts.len().max(b.counts.len()) {
        let ac = a.counts.get(i).copied().unwrap_or(0);
        let bc = b.counts.get(i).copied().unwrap_or(0);
        distance_lanes += entropy(ac.wrapping_add(bc) as f32, inv_tot, total);
    }
    distance_lanes - a.entropy - b.entropy
}

fn num_symbols(a: &Histogram, b: Option<&Histogram>) -> usize {
    let mut n = 0;
    for i in 0..a.counts.len().max(b.map_or(0, |b| b.counts.len())) {
        let count = a.counts.get(i).copied().unwrap_or(0)
            + b.and_then(|b| b.counts.get(i)).copied().unwrap_or(0);
        n += usize::from(count > 0);
    }
    n
}

fn cluster_distance(a: &Histogram, b: &Histogram, prefix_codes: bool) -> f32 {
    let distance = histogram_distance(a, b);
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

pub fn histogram_kl_divergence(actual: &Histogram, coding: &Histogram) -> f32 {
    if actual.total_count == 0 {
        return 0.0;
    }
    if coding.total_count == 0 {
        return f32::INFINITY;
    }
    let coding_inv = 1.0f32 / coding.total_count as f32;
    let mut cost_lanes = 0.0;
    for (i, &count) in actual.counts.iter().enumerate() {
        let coding_count = coding.counts.get(i).copied().unwrap_or(0);
        let coding_probs = coding_count as f32 * coding_inv;
        let log = fast_log2f(coding_probs);
        let neg_coding_cost = if count == 0 {
            0.0
        } else if coding_count == 0 {
            f32::NEG_INFINITY
        } else {
            log
        };
        cost_lanes -= count as f32 * neg_coding_cost;
    }
    cost_lanes - actual.entropy
}

fn fast_cluster_histograms(
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
        histogram_entropy(&mut input[i]);
        if input[i].total_count > input[largest_idx].total_count {
            largest_idx = i;
        }
    }
    if prev_histograms > 0 {
        for histo in out.iter_mut().take(prev_histograms) {
            histogram_entropy(histo);
        }
        for i in 0..input.len() {
            if dists[i] == 0.0 {
                continue;
            }
            for previous in out.iter().take(prev_histograms) {
                let value = histogram_kl_divergence(&input[i], previous);
                // C++ std::min(value, dists[i]) preserves the first equal value.
                dists[i] = if dists[i] < value { dists[i] } else { value };
            }
        }
        if !dists.is_empty() {
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
    while out.len() < max_histograms {
        symbols[largest_idx] = out.len() as u32;
        out.push(input[largest_idx].clone());
        dists[largest_idx] = 0.0;
        largest_idx = 0;
        for i in 0..input.len() {
            if dists[i] == 0.0 {
                continue;
            }
            let value = cluster_distance(&input[i], out.last().unwrap(), prefix_codes);
            dists[i] = if dists[i] < value { dists[i] } else { value };
            if dists[i] > dists[largest_idx] {
                largest_idx = i;
            }
        }
        if dists[largest_idx] < 48.0 {
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
                histogram_kl_divergence(&input[i], histo)
            } else {
                cluster_distance(&input[i], histo, prefix_codes)
            };
            if dist < best_dist {
                best = j;
                best_dist = dist;
            }
        }
        if best_dist >= f32::MAX || best_dist.is_nan() {
            return Err(StatusCode::GenericError);
        }
        if best >= prev_histograms {
            out[best].add_histogram(&input[i]);
            histogram_entropy(&mut out[best]);
        }
        symbols[i] = best as u32;
    }
    Ok(())
}

fn histogram_reindex(out: &mut Vec<Histogram>, prev_histograms: usize, symbols: &mut [u32]) {
    let tmp = out.clone();
    let mut new_index = BTreeMap::new();
    for i in 0..prev_histograms {
        new_index.insert(i as u32, i);
    }
    let mut next_index = prev_histograms;
    for &symbol in symbols.iter() {
        if let std::collections::btree_map::Entry::Vacant(entry) = new_index.entry(symbol) {
            entry.insert(next_index);
            out[next_index] = tmp[symbol as usize].clone();
            next_index += 1;
        }
    }
    out.truncate(next_index);
    for symbol in symbols {
        *symbol = new_index[symbol] as u32;
    }
}

struct HistogramPair {
    cost: f32,
    first: u32,
    second: u32,
    version: u32,
}
impl PartialEq for HistogramPair {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
            && (self.first, self.second, self.version) == (other.first, other.second, other.version)
    }
}
impl Eq for HistogramPair {}
impl PartialOrd for HistogramPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HistogramPair {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .partial_cmp(&self.cost)
            .expect("defined histogram pair cost")
            .then_with(|| {
                (other.first, other.second, other.version).cmp(&(
                    self.first,
                    self.second,
                    self.version,
                ))
            })
    }
}

pub fn cluster_histograms(
    params: &HistogramParams,
    input: &mut [Histogram],
    max_histograms: usize,
    out: &mut Vec<Histogram>,
    symbols: &mut Vec<u32>,
) -> Status {
    let prev_histograms = out.len();
    let mut max_histograms = max_histograms.min(params.max_histograms).min(input.len());
    if params.clustering == ClusteringType::Fastest {
        max_histograms = max_histograms.min(4);
    }
    fast_cluster_histograms(
        input,
        prev_histograms + max_histograms,
        params.force_huffman,
        out,
        symbols,
    )?;
    if prev_histograms == 0 && params.clustering == ClusteringType::Best {
        for histo in out.iter_mut() {
            histo.entropy = histo.ans_population_cost()?;
        }
        let mut next_version = 2u32;
        let mut version = vec![1u32; out.len()];
        let mut renumbering: Vec<u32> = (0..out.len() as u32).collect();
        let mut pairs = BinaryHeap::new();
        for i in 0..out.len() {
            for j in i + 1..out.len() {
                let mut histo = Histogram::default();
                histo.add_histogram(&out[i]);
                histo.add_histogram(&out[j]);
                let cost = histo.ans_population_cost()? - (out[i].entropy + out[j].entropy);
                if cost >= 0.0 {
                    continue;
                }
                pairs.push(HistogramPair {
                    cost,
                    first: i as u32,
                    second: j as u32,
                    version: version[i].max(version[j]),
                });
            }
        }
        while let Some(pair) = pairs.pop() {
            let (first, second, ver) = (pair.first as usize, pair.second as usize, pair.version);
            if ver != version[first].max(version[second])
                || version[first] == 0
                || version[second] == 0
            {
                continue;
            }
            let (before_second, from_second) = out.split_at_mut(second);
            before_second[first].add_histogram(&from_second[0]);
            out[first].entropy = out[first].ans_population_cost()?;
            for item in &mut renumbering {
                if *item == second as u32 {
                    *item = first as u32;
                }
            }
            version[second] = 0;
            version[first] = next_version;
            next_version = next_version.wrapping_add(1);
            for j in 0..out.len() {
                if j == first || version[j] == 0 {
                    continue;
                }
                let mut histo = Histogram::default();
                histo.add_histogram(&out[first]);
                histo.add_histogram(&out[j]);
                let cost = histo.ans_population_cost()? - (out[first].entropy + out[j].entropy);
                if cost >= 0.0 {
                    continue;
                }
                pairs.push(HistogramPair {
                    cost,
                    first: first.min(j) as u32,
                    second: first.max(j) as u32,
                    version: version[first].max(version[j]),
                });
            }
        }
        let mut reverse = vec![u32::MAX; out.len()];
        let mut num_alive = 0;
        for i in 0..out.len() {
            if version[i] == 0 {
                continue;
            }
            out[num_alive] = out[i].clone();
            num_alive += 1;
            reverse[i] = num_alive as u32 - 1;
        }
        out.truncate(num_alive);
        for item in symbols.iter_mut() {
            *item = reverse[renumbering[*item as usize] as usize];
        }
    }
    histogram_reindex(out, prev_histograms, symbols);
    Ok(())
}
