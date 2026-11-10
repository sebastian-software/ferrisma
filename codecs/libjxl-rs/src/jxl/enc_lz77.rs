// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/enc_lz77.{h,cc}`. Owned vectors and borrowed input
//! slices replace pointers. Keep hash wrapping, insertion/traversal order,
//! distance-code collision rules, and cost comparisons from the source.

use super::ans_params::ANS_LOG_TAB_SIZE;
use super::base::bits::ceil_log2_nonzero;
use super::base::fast_math::fast_log2f;
use super::dec_ans::{
    HybridUintConfig, LZ77Params, NUM_SPECIAL_DISTANCES, WINDOW_SIZE, special_distance,
};
use super::enc_ans::Token;
use super::enc_ans_params::{Histogram, HistogramParams, LZ77Method};
use super::enc_lz77_costs::{DIST_COST, LEN_COST};
use std::collections::BTreeMap;

fn cost_mul_add(mul: f32, x: f32, add: f32) -> f32 {
    // Upstream disables Clang auto-vectorization. Its ordinary scalar cost
    // expressions contract on AArch64; the x86_64 baseline has no FMA target.
    // Highway's explicitly non-fused polynomial operations remain separate.
    #[cfg(target_arch = "aarch64")]
    {
        mul.mul_add(x, add)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        mul * x + add
    }
}

/// The source's local cost-model helper. Counts must stay within its signed
/// histogram range, and token contexts must address the supplied context count.
pub struct SymbolCostEstimator {
    max_alphabet_size: usize,
    bits: Vec<f32>,
    add_symbol_cost: Vec<f32>,
}
impl SymbolCostEstimator {
    pub fn new(
        num_contexts: usize,
        force_huffman: bool,
        tokens: &[Vec<Token>],
        lz77: &LZ77Params,
    ) -> Self {
        let mut builder = vec![Histogram::default(); num_contexts];
        let uint_config = HybridUintConfig::default();
        for stream in tokens {
            for token in stream {
                let cfg = if token.is_lz77_length() {
                    lz77.length_uint_config
                } else {
                    uint_config
                };
                let (mut tok, _, _) = cfg.encode(token.value);
                tok = tok.wrapping_add(if token.is_lz77_length() {
                    lz77.min_symbol
                } else {
                    0
                });
                builder[token.context() as usize].add(tok as usize);
            }
        }
        let max_alphabet_size = builder.iter().map(|h| h.counts.len()).max().unwrap_or(0);
        let mut result = Self {
            max_alphabet_size,
            bits: vec![0.0; num_contexts * max_alphabet_size],
            add_symbol_cost: vec![0.0; num_contexts],
        };
        for (i, histogram) in builder.iter().enumerate() {
            let inv_total = 1.0f32 / (histogram.total_count as f32 + 1e-8_f32);
            let mut total_cost = 0.0;
            for (j, &count) in histogram.counts.iter().enumerate() {
                let cnt = count as usize;
                let mut cost = 0.0;
                if cnt != 0 && cnt != histogram.total_count {
                    cost = -fast_log2f(cnt as f32 * inv_total);
                    if force_huffman {
                        cost = cost.ceil();
                    }
                } else if cnt == 0 {
                    cost = ANS_LOG_TAB_SIZE as f32;
                }
                result.bits[i * max_alphabet_size + j] = cost;
                total_cost = cost_mul_add(cost, count as f32, total_cost);
            }
            let penalty = cost_mul_add(-total_cost, inv_total, 6.0);
            result.add_symbol_cost[i] = if 0.0 < penalty { penalty } else { 0.0 };
        }
        result
    }
    pub fn bits(&self, ctx: usize, sym: usize) -> f32 {
        self.bits[ctx * self.max_alphabet_size + sym]
    }
    pub fn len_cost(&self, ctx: usize, len: usize, lz77: &LZ77Params) -> f32 {
        let (tok, nbits, _) = lz77.length_uint_config.encode(len as u32);
        nbits as f32 + self.bits(ctx, tok.wrapping_add(lz77.min_symbol) as usize)
    }
    pub fn dist_cost(&self, len: usize, lz77: &LZ77Params) -> f32 {
        let (tok, nbits, _) = HybridUintConfig::default().encode(len as u32);
        nbits as f32 + self.bits(lz77.nonserialized_distance_context, tok as usize)
    }
    pub fn add_symbol_cost(&self, idx: usize) -> f32 {
        self.add_symbol_cost[idx]
    }
}

fn apply_rle(
    params: &HistogramParams,
    num_contexts: usize,
    tokens: &[Vec<Token>],
    lz77: &LZ77Params,
) -> Vec<Vec<Token>> {
    let mut result = vec![Vec::new(); tokens.len()];
    let sce = SymbolCostEstimator::new(num_contexts, params.force_huffman, tokens, lz77);
    let mut bit_decrease = 0.0f32;
    let mut total_symbols = 0usize;
    let mut sym_cost = Vec::new();
    let uint_config = HybridUintConfig::default();
    for (stream, input) in tokens.iter().enumerate() {
        let multiplier = params.image_widths.get(stream).copied().unwrap_or(0);
        let out = &mut result[stream];
        total_symbols = total_symbols.wrapping_add(input.len());
        sym_cost.resize(input.len() + 1, 0.0f32);
        for i in 0..input.len() {
            let (tok, nbits, _) = uint_config.encode(input[i].value);
            sym_cost[i + 1] =
                sce.bits(input[i].context() as usize, tok as usize) + nbits as f32 + sym_cost[i];
        }
        out.reserve(input.len());
        let mut i = 0;
        while i < input.len() {
            let mut num_to_copy = 0;
            let distance_symbol = usize::from(multiplier != 0);
            if i > 0 {
                while i + num_to_copy < input.len() {
                    if input[i + num_to_copy].value != input[i - 1].value {
                        break;
                    }
                    num_to_copy += 1;
                }
            }
            if num_to_copy == 0 {
                out.push(input[i]);
                i += 1;
                continue;
            }
            let cost = sym_cost[i + num_to_copy] - sym_cost[i];
            let len = num_to_copy.wrapping_sub(lz77.min_length as usize);
            let lz77_cost = if num_to_copy >= lz77.min_length as usize {
                (ceil_log2_nonzero(len.wrapping_add(1)) + 1) as f32
            } else {
                0.0
            };
            if num_to_copy < lz77.min_length as usize || cost <= lz77_cost {
                out.extend_from_slice(&input[i..i + num_to_copy]);
                i += num_to_copy;
                continue;
            }
            let mut length = Token::new(input[i].context(), len as u32);
            length.set_is_lz77_length(true);
            out.push(length);
            i += num_to_copy;
            bit_decrease += cost - lz77_cost;
            out.push(Token::new(
                lz77.nonserialized_distance_context as u32,
                distance_symbol as u32,
            ));
        }
    }
    if f64::from(bit_decrease) > total_symbols as f64 * 0.2 + 16.0 {
        result
    } else {
        Vec::new()
    }
}

fn get_hash<const HASH: usize>(pos: usize, data: &[u32]) -> u32 {
    let mut h = 0u32;
    if pos + HASH <= data.len() {
        for i in 0..HASH {
            h ^= data[pos + i]
                .wrapping_add(0x9e3779b9)
                .wrapping_add(h << 6)
                .wrapping_add(h >> 2);
        }
        h ^= h >> 16;
        h = h.wrapping_mul(0x85ebca6b);
        h ^= h >> 13;
        h = h.wrapping_mul(0xc2b2ae35);
        h ^= h >> 16;
    } else {
        return 0;
    }
    h
}

pub fn len_cost(len: usize) -> f32 {
    let (tok, nbits, _) = HybridUintConfig::new(1, 0, 0).encode(len as u32);
    LEN_COST[(tok as usize).min(LEN_COST.len() - 1)] + nbits as f32
}
pub fn dist_cost(dist: usize) -> f32 {
    let (tok, nbits, _) = HybridUintConfig::new(7, 0, 0).encode(dist as u32);
    DIST_COST[(tok as usize).min(DIST_COST.len() - 1)] + nbits as f32
}

#[derive(Clone)]
struct HashBucket<const BUCKET: usize> {
    data: [u32; BUCKET],
    idx: u16,
    size: u16,
}
struct LZ77HashMap<'a, const BUCKET: usize, const HASH: usize> {
    data: &'a [u32],
    hash_mask: u32,
    hash_table: Vec<HashBucket<BUCKET>>,
    special_dist_table: Vec<i8>,
    num_special_distances: usize,
}
impl<'a, const BUCKET: usize, const HASH: usize> LZ77HashMap<'a, BUCKET, HASH> {
    fn new(data: &'a [u32], multiplier: usize) -> Self {
        let hash_mask = (1u32 << 14) - 1;
        let mut result = Self {
            data,
            hash_mask,
            hash_table: vec![
                HashBucket {
                    data: [0; BUCKET],
                    idx: 0,
                    size: 0
                };
                hash_mask as usize + 1
            ],
            special_dist_table: Vec::new(),
            num_special_distances: 0,
        };
        if multiplier != 0 {
            result.num_special_distances = NUM_SPECIAL_DISTANCES;
            result.special_dist_table.resize(multiplier * 8 + 8, -1);
            // This table traverses upward: collisions keep the largest code.
            for i in 0..NUM_SPECIAL_DISTANCES {
                result.special_dist_table[special_distance(i, multiplier as i32) as usize] =
                    i as i8;
            }
        }
        result
    }
    fn find_match(&mut self, pos: usize, min_len: usize) -> (usize, usize) {
        let h = (get_hash::<HASH>(pos, self.data) & self.hash_mask) as usize;
        let mut len = 0;
        let mut dist_symbol = 0;
        for i in 0..usize::from(self.hash_table[h].size) {
            let candidate = self.hash_table[h].data[i] as usize;
            let dist = pos - candidate;
            let cur_length = self.match_length(candidate, pos);
            if cur_length < len || cur_length < min_len {
                continue;
            }
            let mut cur_dist_symbol = self.num_special_distances + dist - 1;
            if dist < self.special_dist_table.len() {
                let lookup = self.special_dist_table[dist];
                if lookup >= 0 {
                    cur_dist_symbol = lookup as usize;
                }
            }
            if cur_length == len && cur_dist_symbol >= dist_symbol {
                continue;
            }
            len = cur_length;
            dist_symbol = cur_dist_symbol;
        }
        self.insert(h, pos);
        (len, dist_symbol)
    }
    fn update(&mut self, pos: usize) {
        let h = (get_hash::<HASH>(pos, self.data) & self.hash_mask) as usize;
        self.insert(h, pos);
    }
    fn insert(&mut self, h: usize, pos: usize) {
        let bucket = &mut self.hash_table[h];
        bucket.size = bucket.size.wrapping_add(1).min(BUCKET as u16);
        bucket.data[bucket.idx as usize] = pos as u32;
        let next = bucket.idx.wrapping_add(1);
        bucket.idx = next - u16::from(next >= BUCKET as u16) * BUCKET as u16;
    }
    fn match_length(&self, a: usize, b: usize) -> usize {
        debug_assert!(a < b);
        let mut len = 0;
        while b + len < self.data.len() && self.data[a + len] == self.data[b + len] {
            len += 1;
        }
        len
    }
}

fn apply_greedy<const BUCKET: usize, const HASH: usize, const RUNTIME_COST: bool>(
    params: &HistogramParams,
    num_contexts: usize,
    tokens: &[Vec<Token>],
    lz77: &LZ77Params,
) -> Vec<Vec<Token>> {
    let mut result = vec![Vec::new(); tokens.len()];
    let sce = SymbolCostEstimator::new(num_contexts, params.force_huffman, tokens, lz77);
    let mut bit_decrease = 0.0f32;
    let mut total_symbols = 0usize;
    let uint_config = HybridUintConfig::default();
    let mut sym_cost = Vec::new();
    for (stream, input) in tokens.iter().enumerate() {
        let multiplier = params.image_widths.get(stream).copied().unwrap_or(0);
        let out = &mut result[stream];
        total_symbols = total_symbols.wrapping_add(input.len());
        let mut data = vec![0u32; input.len()];
        sym_cost.resize(input.len() + 1, 0.0f32);
        for pos in 0..input.len() {
            let (tok, nbits, _) = uint_config.encode(input[pos].value);
            sym_cost[pos + 1] = sce.bits(input[pos].context() as usize, tok as usize)
                + nbits as f32
                + sym_cost[pos];
            data[pos] = input[pos].value;
        }
        out.reserve(input.len());
        let min_length = lz77.min_length as usize;
        debug_assert!(min_length >= 3);
        let mut map = LZ77HashMap::<BUCKET, HASH>::new(&data, multiplier);
        let mut pos = 0;
        while pos < input.len() {
            out.push(input[pos]);
            if pos + HASH >= input.len() {
                pos += 1;
                continue;
            }
            let (len, dist_symbol) = map.find_match(pos, min_length);
            if len < min_length {
                pos += 1;
                continue;
            }
            let cost = sym_cost[pos + len] - sym_cost[pos];
            let lz77_len = len - min_length;
            let lz77_cost = len_cost(lz77_len) + dist_cost(dist_symbol);
            if RUNTIME_COST && lz77_cost > cost {
                for offset in 1..len {
                    out.push(input[pos + offset]);
                    map.update(pos + offset);
                }
                pos += len;
                continue;
            }
            bit_decrease +=
                cost - lz77_cost - sce.add_symbol_cost(out.last().unwrap().context() as usize);
            let token = out.last_mut().unwrap();
            token.value = lz77_len as u32;
            token.set_is_lz77_length(true);
            out.push(Token::new(
                lz77.nonserialized_distance_context as u32,
                dist_symbol as u32,
            ));
            for offset in 1..len {
                map.update(pos + offset);
            }
            pos += len;
        }
    }
    if f64::from(bit_decrease) > total_symbols as f64 * 0.2 + 16.0 {
        result
    } else {
        Vec::new()
    }
}

struct HashChain<const MAX_CHAIN: u32> {
    data: Vec<u32>,
    head: Vec<i32>,
    chain: Vec<u32>,
    val: Vec<i32>,
    headz: Vec<i32>,
    chainz: Vec<u32>,
    zeros: Vec<u32>,
    numzeros: u32,
    window_size: usize,
    window_mask: usize,
    min_length: usize,
    max_length: usize,
    special_dist_table: BTreeMap<i32, i32>,
    num_special_distances: usize,
    max_chain_length: u32,
}
impl<const MAX_CHAIN: u32> HashChain<MAX_CHAIN> {
    fn new(
        tokens: &[Token],
        window_size: usize,
        min_length: usize,
        max_length: usize,
        multiplier: usize,
    ) -> Self {
        let mut result = Self {
            data: tokens.iter().map(|t| t.value).collect(),
            head: vec![-1; 32768],
            val: vec![-1; window_size],
            chain: (0..window_size as u32).collect(),
            chainz: (0..window_size as u32).collect(),
            zeros: vec![0; window_size],
            headz: vec![-1; window_size + 1],
            numzeros: 0,
            window_size,
            window_mask: window_size - 1,
            min_length,
            max_length,
            special_dist_table: BTreeMap::new(),
            num_special_distances: 0,
            max_chain_length: MAX_CHAIN,
        };
        if multiplier != 0 {
            // Unlike the greedy ring map, this source loop counts downward.
            for i in (0..NUM_SPECIAL_DISTANCES).rev() {
                result
                    .special_dist_table
                    .insert(special_distance(i, multiplier as i32), i as i32);
            }
            result.num_special_distances = NUM_SPECIAL_DISTANCES;
        }
        result
    }
    fn count_zeros(&self, pos: usize, prevzeros: u32) -> u32 {
        let end = (pos + self.window_size).min(self.data.len());
        if prevzeros > 0 {
            if prevzeros as usize >= self.window_mask
                && self.data[end - 1] == 0
                && end == pos + self.window_size
            {
                return prevzeros;
            }
            return prevzeros - 1;
        }
        let mut num = 0;
        while pos + (num as usize) < end && self.data[pos + num as usize] == 0 {
            num += 1;
        }
        num
    }
    fn update(&mut self, pos: usize) {
        let hashval = (get_hash::<3>(pos, &self.data) & 32767) as usize;
        let wpos = pos & self.window_mask;
        self.val[wpos] = hashval as i32;
        if self.head[hashval] != -1 {
            self.chain[wpos] = self.head[hashval] as u32;
        }
        self.head[hashval] = wpos as i32;
        if pos > 0 && self.data[pos] != self.data[pos - 1] {
            self.numzeros = 0;
        }
        self.numzeros = self.count_zeros(pos, self.numzeros);
        self.zeros[wpos] = self.numzeros;
        if self.headz[self.numzeros as usize] != -1 {
            self.chainz[wpos] = self.headz[self.numzeros as usize] as u32;
        }
        self.headz[self.numzeros as usize] = wpos as i32;
    }
    fn find_matches(&self, pos: usize, _max_dist: i32, mut found_match: impl FnMut(usize, usize)) {
        let wpos = (pos & self.window_mask) as u32;
        let hashval = get_hash::<3>(pos, &self.data) & 32767;
        let mut hashpos = self.chain[wpos as usize];
        let mut prev_dist = 0i32;
        let end = (pos.wrapping_add(self.max_length) as i32).min(self.data.len() as i32);
        let mut chainlength = 0u32;
        let mut best_len = 0u32;
        loop {
            let dist = if hashpos <= wpos {
                (wpos - hashpos) as i32
            } else {
                (wpos.wrapping_sub(hashpos) as usize + self.window_mask + 1) as i32
            };
            if dist < prev_dist {
                break;
            }
            prev_dist = dist;
            let mut len = 0u32;
            if dist > 0 {
                let mut i = pos as i32;
                let mut j = pos.wrapping_sub(dist as usize) as i32;
                if self.numzeros > 3 {
                    let mut r =
                        ((self.numzeros - 1) as i32).min(self.zeros[hashpos as usize] as i32);
                    if i + r >= end {
                        r = end - i - 1;
                    }
                    i += r;
                    j += r;
                }
                while i < end && self.data[i as usize] == self.data[j as usize] {
                    i += 1;
                    j += 1;
                }
                len = (i as usize).wrapping_sub(pos) as u32;
                if len as usize >= self.min_length && len.wrapping_add(2) >= best_len {
                    let dist_symbol = self
                        .special_dist_table
                        .get(&dist)
                        .copied()
                        .unwrap_or((self.num_special_distances + dist as usize - 1) as i32);
                    found_match(len as usize, dist_symbol as usize);
                    if len > best_len {
                        best_len = len;
                    }
                }
            }
            chainlength = chainlength.wrapping_add(1);
            if chainlength >= self.max_chain_length {
                break;
            }
            if self.numzeros >= 3 && len > self.numzeros {
                if hashpos == self.chainz[hashpos as usize] {
                    break;
                }
                hashpos = self.chainz[hashpos as usize];
                if self.zeros[hashpos as usize] != self.numzeros {
                    break;
                }
            } else {
                if hashpos == self.chain[hashpos as usize] {
                    break;
                }
                hashpos = self.chain[hashpos as usize];
                if self.val[hashpos as usize] != hashval as i32 {
                    break;
                }
            }
        }
    }
}

fn apply_optimal<const MAX_CHAIN: u32>(
    params: &HistogramParams,
    num_contexts: usize,
    tokens: &[Vec<Token>],
    lz77: &LZ77Params,
) -> Vec<Vec<Token>> {
    let estimates = apply_greedy::<7, 3, false>(params, num_contexts, tokens, lz77);
    if estimates.is_empty() {
        return Vec::new();
    }
    let sce = SymbolCostEstimator::new(num_contexts + 1, params.force_huffman, &estimates, lz77);
    let mut result = vec![Vec::new(); tokens.len()];
    let uint_config = HybridUintConfig::default();
    let mut sym_cost = Vec::new();
    let mut dist_symbols = Vec::new();
    for (stream, input) in tokens.iter().enumerate() {
        let multiplier = params.image_widths.get(stream).copied().unwrap_or(0);
        let out = &mut result[stream];
        sym_cost.resize(input.len() + 1, 0.0f32);
        for i in 0..input.len() {
            let (tok, nbits, _) = uint_config.encode(input[i].value);
            sym_cost[i + 1] =
                sce.bits(input[i].context() as usize, tok as usize) + nbits as f32 + sym_cost[i];
        }
        out.reserve(input.len());
        let max_distance = input.len();
        let min_length = lz77.min_length as usize;
        debug_assert!(min_length >= 3);
        let max_length = input.len();
        let mut window_size = 1;
        while window_size < max_distance && window_size < WINDOW_SIZE {
            window_size <<= 1;
        }
        let mut chain =
            HashChain::<MAX_CHAIN>::new(input, window_size, min_length, max_length, multiplier);
        struct MatchInfo {
            len: u32,
            dist_symbol: u32,
            ctx: u32,
            total_cost: f32,
        }
        let mut prefix_costs: Vec<_> = (0..input.len() + 1)
            .map(|_| MatchInfo {
                len: 0,
                dist_symbol: 0,
                ctx: 0,
                total_cost: f32::MAX,
            })
            .collect();
        prefix_costs[0].total_cost = 0.0;
        let mut rle_length = 0usize;
        let mut skip_lz77 = 0usize;
        for i in 0..input.len() {
            chain.update(i);
            let lit_cost = prefix_costs[i].total_cost + sym_cost[i + 1] - sym_cost[i];
            if prefix_costs[i + 1].total_cost > lit_cost {
                prefix_costs[i + 1] = MatchInfo {
                    dist_symbol: 0,
                    len: 1,
                    ctx: input[i].context(),
                    total_cost: lit_cost,
                };
            }
            if skip_lz77 > 0 {
                skip_lz77 -= 1;
                continue;
            }
            dist_symbols.clear();
            chain.find_matches(i, max_distance as i32, |len, dist_symbol| {
                if dist_symbols.len() <= len {
                    dist_symbols.resize(len + 1, dist_symbol as u32);
                }
                if dist_symbol < dist_symbols[len] as usize {
                    dist_symbols[len] = dist_symbol as u32;
                }
            });
            if dist_symbols.len() <= min_length {
                continue;
            }
            let mut best_cost = *dist_symbols.last().unwrap() as usize;
            for j in (min_length..dist_symbols.len()).rev() {
                if (dist_symbols[j] as usize) < best_cost {
                    best_cost = dist_symbols[j] as usize;
                }
                dist_symbols[j] = best_cost as u32;
            }
            for j in min_length..dist_symbols.len() {
                let lz77_cost = sce.len_cost(input[i].context() as usize, j - min_length, lz77)
                    + sce.dist_cost(dist_symbols[j] as usize, lz77);
                let cost = prefix_costs[i].total_cost + lz77_cost;
                if prefix_costs[i + j].total_cost > cost {
                    prefix_costs[i + j] = MatchInfo {
                        len: j as u32,
                        dist_symbol: dist_symbols[j].wrapping_add(1),
                        ctx: input[i].context(),
                        total_cost: cost,
                    };
                }
            }
            if (*dist_symbols.last().unwrap() == 0 && multiplier == 0)
                || (*dist_symbols.last().unwrap() == 1 && multiplier != 0)
            {
                rle_length += 1;
            } else {
                rle_length = 0;
            }
            if rle_length >= 8 && dist_symbols.len() > 9 {
                skip_lz77 = dist_symbols.len() - 10;
                rle_length = 0;
            }
        }
        let mut pos = input.len();
        while pos > 0 {
            let is_length = prefix_costs[pos].dist_symbol != 0;
            if is_length {
                out.push(Token::new(
                    lz77.nonserialized_distance_context as u32,
                    prefix_costs[pos].dist_symbol - 1,
                ));
            }
            let val = if is_length {
                prefix_costs[pos].len.wrapping_sub(min_length as u32)
            } else {
                input[pos - 1].value
            };
            let mut token = Token::new(prefix_costs[pos].ctx, val);
            token.set_is_lz77_length(is_length);
            out.push(token);
            pos -= prefix_costs[pos].len as usize;
        }
        if !out.is_empty() {
            out.reverse();
        }
    }
    result
}

/// Returns no streams when the source's savings threshold is not met.
pub fn apply_lz77(
    params: &HistogramParams,
    num_contexts: usize,
    tokens: &[Vec<Token>],
    lz77: &LZ77Params,
) -> Vec<Vec<Token>> {
    match params.lz77_method {
        LZ77Method::None => Vec::new(),
        LZ77Method::RLE => apply_rle(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b1w3f => apply_greedy::<1, 3, false>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b3w3f => apply_greedy::<3, 3, false>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b7w3f => apply_greedy::<7, 3, false>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b15w3f => apply_greedy::<15, 3, false>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b31w3f => apply_greedy::<31, 3, false>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b1w3t => apply_greedy::<1, 3, true>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b3w3t => apply_greedy::<3, 3, true>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b7w3t => apply_greedy::<7, 3, true>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b15w3t => apply_greedy::<15, 3, true>(params, num_contexts, tokens, lz77),
        LZ77Method::LZ77b31w3t => apply_greedy::<31, 3, true>(params, num_contexts, tokens, lz77),
        LZ77Method::Optc1 => apply_optimal::<1>(params, num_contexts, tokens, lz77),
        LZ77Method::Optc3 => apply_optimal::<3>(params, num_contexts, tokens, lz77),
        LZ77Method::Optc8 => apply_optimal::<8>(params, num_contexts, tokens, lz77),
        LZ77Method::Optc256 => apply_optimal::<256>(params, num_contexts, tokens, lz77),
    }
}
