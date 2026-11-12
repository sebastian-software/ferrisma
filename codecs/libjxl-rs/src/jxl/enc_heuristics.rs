// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! FindBestBlockEntropyModel from lib/jxl/enc_heuristics.cc. The remaining
//! heuristics of that file (LossyFrameHeuristics, ComputeARHeuristics with
//! ReconstructImage, the downsampling helpers) are not translated yet.
//!
//! The cluster merge's std::sort uses the translated libc++ introsort
//! (`base/libcpp_sort.rs`): its comparator orders equal counts arbitrarily,
//! so the reference library's order decides the merged pair.

use super::ac_context::BlockCtxMap;
use super::ac_strategy::AcStrategyImage;
use super::coeff_order::STRATEGY_ORDER;
use super::coeff_order_fwd::NUM_ORDERS;
use super::common::SpeedTier;
use super::enc_params::CompressParams;
use super::image::Plane;

const N_ORDERS: usize = NUM_ORDERS as usize;

fn clamp1(v: i32, lo: i32, hi: i32) -> i32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

/// FindBestBlockEntropyModel.
#[expect(clippy::needless_range_loop, reason = "Keep the source index loops.")]
pub fn find_best_block_entropy_model(
    cparams: &CompressParams<'_>,
    rqf: &Plane<i32>,
    ac_strategy: &AcStrategyImage,
    block_ctx_map: &mut BlockCtxMap,
) {
    if cparams.decoding_speed_tier >= 1 {
        // The source assigns the simple map to a local copy of the block
        // context map and returns, leaving the map unchanged.
        return;
    }
    if cparams.speed_tier >= SpeedTier::Falcon {
        return;
    }
    // No need to change context modeling for small images.
    let tot = rqf.xsize() * rqf.ysize();
    let size_for_ctx_model = (1024.0f32 * cparams.butteraugli_distance) as usize;
    if tot < size_for_ctx_model {
        return;
    }

    // count the occurrences of each qf value and each strategy type.
    let mut qf_counts = vec![0usize; 256];
    let mut qf_ord_counts = vec![[0usize; 256]; N_ORDERS];
    let mut ord_counts = [0usize; N_ORDERS];
    for y in 0..rqf.ysize() {
        let qf_row = rqf.row(y);
        let acs_row = ac_strategy.const_row(y, 0);
        for x in 0..rqf.xsize() {
            let ord = usize::from(STRATEGY_ORDER[usize::from(acs_row.get(x).raw_strategy())]);
            let qf = (qf_row[x] - 1) as usize;
            qf_counts[qf] += 1;
            qf_ord_counts[ord][qf] += 1;
            ord_counts[ord] += 1;
        }
    }

    // Splitting the context model according to the quantization field seems to
    // mostly benefit only large images.
    let size_for_qf_split = (8192.0f32 * cparams.butteraugli_distance) as usize;
    let num_qf_segments = if tot < size_for_qf_split { 1 } else { 2 };
    let qft = &mut block_ctx_map.qf_thresholds;
    qft.clear();
    // Divide the quant field in up to num_qf_segments segments.
    let mut cumsum = 0usize;
    let mut next = 1usize;
    let mut last_cut = 256usize;
    let mut cut = tot * next / num_qf_segments;
    for j in 0..256u32 {
        cumsum += qf_counts[j as usize];
        if cumsum > cut {
            if j != 0 {
                qft.push(j);
            }
            last_cut = j as usize;
            while cumsum > cut {
                next += 1;
                cut = tot * next / num_qf_segments;
            }
        } else if next > qft.len() + 1 && (j as usize).wrapping_sub(1) == last_cut && j != 0 {
            qft.push(j);
        }
    }

    // Count the occurrences of each segment.
    let mut counts = vec![0usize; N_ORDERS * (qft.len() + 1)];
    let mut qft_pos = 0usize;
    for j in 0..256usize {
        if qft_pos < qft.len() && j == qft[qft_pos] as usize {
            qft_pos += 1;
        }
        for i in 0..N_ORDERS {
            counts[qft_pos + i * (qft.len() + 1)] += qf_ord_counts[i][j];
        }
    }

    // Repeatedly merge the lowest-count pair.
    let mut remap: Vec<u8> = (0..((qft.len() + 1) * N_ORDERS) as u8).collect();
    let mut clusters = remap.clone();
    let nb_clusters = clamp1((tot / size_for_ctx_model / 2) as i32, 2, 9) as usize;
    let nb_clusters_chroma = clamp1((tot / size_for_ctx_model / 3) as i32, 1, 5) as usize;
    // This is O(n^2 log n), but n is small.
    while clusters.len() > nb_clusters {
        super::base::libcpp_sort::sort_by(&mut clusters, |&a, &b| {
            counts[usize::from(a)] > counts[usize::from(b)]
        });
        let n = clusters.len();
        let back = usize::from(clusters[n - 1]);
        let second = usize::from(clusters[n - 2]);
        counts[second] += counts[back];
        counts[back] = 0;
        remap[back] = clusters[n - 2];
        clusters.pop();
    }
    for i in 0..remap.len() {
        while remap[usize::from(remap[i])] != remap[i] {
            remap[i] = remap[usize::from(remap[i])];
        }
    }
    // Relabel starting from 0.
    let mut remap_remap = vec![remap.len() as u8; remap.len()];
    let mut num = 0usize;
    for i in 0..remap.len() {
        if usize::from(remap_remap[usize::from(remap[i])]) == remap.len() {
            remap_remap[usize::from(remap[i])] = num as u8;
            num += 1;
        }
        remap[i] = remap_remap[usize::from(remap[i])];
    }
    // Write the block context map.
    let len = remap.len();
    let ctx_map = &mut block_ctx_map.ctx_map;
    *ctx_map = remap;
    ctx_map.resize(len * 3, 0);
    // for chroma, only use up to nb_clusters_chroma separate block contexts
    // (those for the biggest clusters)
    for i in len..len * 3 {
        ctx_map[i] = (num as i32
            + clamp1(
                i32::from(ctx_map[i % len]),
                0,
                nb_clusters_chroma as i32 - 1,
            )) as u8;
    }
    let mut max = ctx_map[0];
    for &v in ctx_map.iter() {
        if max < v {
            max = v;
        }
    }
    block_ctx_map.num_ctxs = usize::from(max) + 1;
}
