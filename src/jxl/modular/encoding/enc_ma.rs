// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Tree sampling, quantization, learning, and tokenization from
//! lib/jxl/modular/encoding/enc_ma.{h,cc}. VecDeque preserves the source BFS;
//! owned vectors retain its sample workspaces and fixed-width hashes/counts.

use super::context_predict::{GRADIENT_PROP, NUM_NONREF_PROPERTIES, WP_PROP};
use super::dec_ma::{PropertyDecisionNode, Tree};
use super::ma_common::*;
use crate::jxl::ans_params::ANS_TAB_SIZE;
use crate::jxl::base::bits::ceil_log2_nonzero;
use crate::jxl::base::fast_math::fast_log2f;
use crate::jxl::base::random::Rng;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::dec_ans::HybridUintConfig;
use crate::jxl::enc_ans::Token;
use crate::jxl::modular::modular_image::Image;
use crate::jxl::modular::options::Predictor;
use crate::jxl::modular::options::{
    ModularMultiplierInfo, ModularOptions, NUM_MODULAR_PREDICTORS, NUM_STATIC_PROPERTIES,
    Properties, StaticPropRange, TreeMode,
};
use crate::jxl::pack_signed::pack_signed;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResidualToken {
    pub tok: u8,
    pub nbits: u8,
}
#[derive(Default)]
pub struct TreeSamples {
    residuals: Vec<Vec<ResidualToken>>,
    sample_counts: Vec<u16>,
    // Source initializes this only in SetProperties. Extra Rust initialization
    // gives it no meaning before that call.
    num_static_props: usize,
    static_props: [Vec<u32>; NUM_STATIC_PROPERTIES as usize],
    props: Vec<Vec<u8>>,
    compact_properties: Vec<Vec<i32>>,
    props_to_use: Vec<u32>,
    predictors: Vec<Predictor>,
    static_property_mapping: [Vec<u16>; NUM_STATIC_PROPERTIES as usize],
    property_mapping: Vec<Vec<u8>>,
    num_samples: usize,
    dedup_table: Vec<u32>,
}
impl TreeSamples {
    const PROPERTY_RANGE: i32 = 511;
    const DEDUP_ENTRY_UNUSED: u32 = u32::MAX;
    pub fn has_samples(&self) -> bool {
        !self.residuals.is_empty() && !self.residuals[0].is_empty()
    }
    pub fn num_distinct_samples(&self) -> usize {
        self.sample_counts.len()
    }
    pub fn num_samples(&self) -> usize {
        self.num_samples
    }
    pub fn rtokens(&self, pred: usize) -> &[ResidualToken] {
        &self.residuals[pred]
    }
    pub fn rtoken(&self, pred: usize, i: usize) -> &ResidualToken {
        &self.residuals[pred][i]
    }
    pub fn token(&self, pred: usize, i: usize) -> usize {
        usize::from(self.residuals[pred][i].tok)
    }
    pub fn count(&self, i: usize) -> usize {
        usize::from(self.sample_counts[i])
    }
    pub fn predictor_index(&self, p: Predictor) -> usize {
        self.predictors
            .iter()
            .position(|&v| v == p)
            .unwrap_or(self.predictors.len())
    }
    pub fn property_index(&self, p: usize) -> usize {
        self.props_to_use
            .iter()
            .position(|&v| v as usize == p)
            .unwrap_or(self.props_to_use.len())
    }
    pub fn num_property_values(&self, p: usize) -> usize {
        self.compact_properties[p].len() + 1
    }
    pub fn num_static_props(&self) -> usize {
        self.num_static_props
    }
    pub fn property<const S: bool>(&self, p: usize, i: usize) -> usize {
        if S {
            self.static_props[p][i] as usize
        } else {
            usize::from(self.props[p][i])
        }
    }
    pub fn unquantize_property(&self, p: usize, q: u32) -> i32 {
        self.compact_properties[p][q as usize]
    }
    pub fn predictor_from_index(&self, i: usize) -> Predictor {
        self.predictors[i]
    }
    pub fn property_from_index(&self, i: usize) -> usize {
        self.props_to_use[i] as usize
    }
    pub fn num_predictors(&self) -> usize {
        self.predictors.len()
    }
    pub fn num_properties(&self) -> usize {
        self.props_to_use.len()
    }
    pub fn all_samples_done(&mut self) {
        self.dedup_table = Vec::new();
    }
    pub fn quantize_property(&self, prop: u32, v: i32) -> u32 {
        let v = v.clamp(-Self::PROPERTY_RANGE, Self::PROPERTY_RANGE) + Self::PROPERTY_RANGE;
        u32::from(self.property_mapping[prop as usize - self.num_static_props][v as usize])
    }
    pub fn quantize_static_property(&self, prop: u32, v: i32) -> u32 {
        let v = v.clamp(-Self::PROPERTY_RANGE, Self::PROPERTY_RANGE) + Self::PROPERTY_RANGE;
        u32::from(self.static_property_mapping[prop as usize][v as usize])
    }
    pub fn set_predictor(&mut self, predictor: Predictor, mode: TreeMode) -> Status {
        if mode == TreeMode::WPOnly {
            self.predictors = vec![Predictor::WEIGHTED];
            self.residuals.resize_with(1, Vec::new);
            return Ok(());
        }
        if mode == TreeMode::NoWP && predictor == Predictor::WEIGHTED {
            return Err(StatusCode::GenericError);
        }
        if predictor == Predictor::VARIABLE {
            for i in 0..NUM_MODULAR_PREDICTORS {
                self.predictors.push(Predictor(i as u32));
            }
            self.predictors.swap(0, Predictor::WEIGHTED.0 as usize);
            self.predictors.swap(1, Predictor::GRADIENT.0 as usize);
        } else if predictor == Predictor::BEST {
            self.predictors = vec![Predictor::WEIGHTED, Predictor::GRADIENT];
        } else {
            self.predictors = vec![predictor];
        }
        if mode == TreeMode::NoWP {
            self.predictors.retain(|&p| p != Predictor::WEIGHTED);
        }
        self.residuals.resize_with(self.predictors.len(), Vec::new);
        Ok(())
    }
    pub fn set_properties(&mut self, properties: &[u32], mode: TreeMode) -> Status {
        self.props_to_use = properties.to_vec();
        if mode == TreeMode::WPOnly {
            self.props_to_use = vec![WP_PROP as u32];
        }
        if mode == TreeMode::GradientOnly {
            self.props_to_use = vec![GRADIENT_PROP as u32];
        }
        if mode == TreeMode::NoWP {
            self.props_to_use.retain(|&p| p != WP_PROP as u32);
        }
        if self.props_to_use.is_empty() {
            return Err(StatusCode::GenericError);
        }
        self.num_static_props = 0;
        for &p in &self.props_to_use {
            if p < NUM_STATIC_PROPERTIES as u32 {
                self.num_static_props += 1;
            }
        }
        self.props
            .resize_with(self.props_to_use.len() - self.num_static_props, Vec::new);
        Ok(())
    }
    fn init_table(&mut self, log_size: usize) {
        let size = 1usize << log_size;
        if self.dedup_table.len() == size {
            return;
        }
        // resize intentionally retains old entries, as the source does.
        self.dedup_table.resize(size, Self::DEDUP_ENTRY_UNUSED);
        for i in 0..self.num_distinct_samples() {
            if self.sample_counts[i] != u16::MAX {
                self.add_to_table(i);
            }
        }
    }
    fn add_to_table_and_merge(&mut self, a: usize) -> bool {
        let pos1 = self.hash1(a);
        let pos2 = self.hash2(a);
        if self.dedup_table[pos1] != Self::DEDUP_ENTRY_UNUSED
            && self.is_same_sample(a, self.dedup_table[pos1] as usize)
        {
            let b = self.dedup_table[pos1] as usize;
            self.sample_counts[b] = self.sample_counts[b].wrapping_add(1);
            if self.sample_counts[b] == u16::MAX {
                self.dedup_table[pos1] = Self::DEDUP_ENTRY_UNUSED;
            }
            return true;
        }
        if self.dedup_table[pos2] != Self::DEDUP_ENTRY_UNUSED
            && self.is_same_sample(a, self.dedup_table[pos2] as usize)
        {
            let b = self.dedup_table[pos2] as usize;
            self.sample_counts[b] = self.sample_counts[b].wrapping_add(1);
            if self.sample_counts[b] == u16::MAX {
                self.dedup_table[pos2] = Self::DEDUP_ENTRY_UNUSED;
            }
            return true;
        }
        self.add_to_table(a);
        false
    }
    fn add_to_table(&mut self, a: usize) {
        let p1 = self.hash1(a);
        let p2 = self.hash2(a);
        if self.dedup_table[p1] == Self::DEDUP_ENTRY_UNUSED {
            self.dedup_table[p1] = a as u32;
        } else if self.dedup_table[p2] == Self::DEDUP_ENTRY_UNUSED {
            self.dedup_table[p2] = a as u32;
        }
    }
    pub fn prepare_for_samples(&mut self, extra_num_samples: usize) {
        for r in &mut self.residuals {
            r.reserve(extra_num_samples);
        }
        for p in self.static_props.iter_mut().take(self.num_static_props) {
            p.reserve(extra_num_samples);
        }
        for p in &mut self.props {
            p.reserve(extra_num_samples);
        }
        let total = extra_num_samples.wrapping_add(self.sample_counts.len());
        let next_size = ceil_log2_nonzero(total.wrapping_mul(3) / 2);
        self.init_table(next_size);
    }
    fn hash1(&self, a: usize) -> usize {
        const C: u64 = 0x1e35a7bd;
        let mut h = C;
        for r in &self.residuals {
            h = h.wrapping_mul(C).wrapping_add(u64::from(r[a].tok));
            h = h.wrapping_mul(C).wrapping_add(u64::from(r[a].nbits));
        }
        for p in self.static_props.iter().take(self.num_static_props) {
            h = h.wrapping_mul(C).wrapping_add(u64::from(p[a]));
        }
        for p in &self.props {
            h = h.wrapping_mul(C).wrapping_add(u64::from(p[a]));
        }
        ((h >> 16) as usize) & (self.dedup_table.len() - 1)
    }
    fn hash2(&self, a: usize) -> usize {
        const C: u64 = 0x1e35a7bd1e35a7bd;
        let mut h = C;
        for p in self.static_props.iter().take(self.num_static_props) {
            h = h.wrapping_mul(C) ^ u64::from(p[a]);
        }
        for p in &self.props {
            h = h.wrapping_mul(C) ^ u64::from(p[a]);
        }
        for r in &self.residuals {
            h = h.wrapping_mul(C) ^ u64::from(r[a].tok);
            h = h.wrapping_mul(C) ^ u64::from(r[a].nbits);
        }
        ((h >> 16) as usize) & (self.dedup_table.len() - 1)
    }
    fn is_same_sample(&self, a: usize, b: usize) -> bool {
        let mut ret = true;
        for r in &self.residuals {
            if r[a].tok != r[b].tok {
                ret = false;
            }
            if r[a].nbits != r[b].nbits {
                ret = false;
            }
        }
        for p in self.static_props.iter().take(self.num_static_props) {
            if p[a] != p[b] {
                ret = false;
            }
        }
        for p in &self.props {
            if p[a] != p[b] {
                ret = false;
            }
        }
        ret
    }
    pub fn add_sample(&mut self, pixel: i64, properties: &Properties, predictions: &[i64]) {
        for (i, p) in self.predictors.iter().enumerate() {
            let v = (pixel - predictions[p.0 as usize]) as i32;
            let (tok, nbits, _) = HybridUintConfig::new(4, 1, 2).encode(pack_signed(v));
            self.residuals[i].push(ResidualToken {
                tok: tok as u8,
                nbits: nbits as u8,
            });
        }
        for (i, &property) in properties.iter().enumerate().take(self.num_static_props) {
            let q = self.quantize_static_property(i as u32, property);
            self.static_props[i].push(q);
        }
        for i in self.num_static_props..self.props_to_use.len() {
            let q = self.quantize_property(i as u32, properties[self.props_to_use[i] as usize]);
            self.props[i - self.num_static_props].push(q as u8);
        }
        self.sample_counts.push(1);
        self.num_samples = self.num_samples.wrapping_add(1);
        if self.add_to_table_and_merge(self.sample_counts.len() - 1) {
            for r in &mut self.residuals {
                r.pop();
            }
            for p in self.static_props.iter_mut().take(self.num_static_props) {
                p.pop();
            }
            for p in &mut self.props {
                p.pop();
            }
            self.sample_counts.pop();
        }
    }
    pub fn swap(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        for r in &mut self.residuals {
            r.swap(a, b);
        }
        for p in self.static_props.iter_mut().take(self.num_static_props) {
            p.swap(a, b);
        }
        for p in &mut self.props {
            p.swap(a, b);
        }
        self.sample_counts.swap(a, b);
    }
}

fn quantize_histogram(histogram: &[u32], num_chunks: usize) -> Vec<i32> {
    if histogram.is_empty() || num_chunks == 0 {
        return Vec::new();
    }
    let sum_long = histogram.iter().fold(0 as std::os::raw::c_ulong, |a, &n| {
        a.wrapping_add(n as std::os::raw::c_ulong)
    });
    // std::accumulate starts from 0LU before assignment to uint64_t.
    #[cfg(any(windows, target_pointer_width = "32"))]
    let sum = u64::from(sum_long);
    #[cfg(all(not(windows), not(target_pointer_width = "32")))]
    let sum = sum_long;
    if sum == 0 {
        return Vec::new();
    }
    let mut thresholds = Vec::new();
    let mut cumsum = 0u64;
    let mut threshold = 1u64;
    for (i, &n) in histogram.iter().enumerate() {
        cumsum = cumsum.wrapping_add(u64::from(n));
        if cumsum.wrapping_mul(num_chunks as u64) >= threshold.wrapping_mul(sum) {
            thresholds.push(i as i32);
            while cumsum.wrapping_mul(num_chunks as u64) >= threshold.wrapping_mul(sum) {
                threshold = threshold.wrapping_add(1);
            }
        }
    }
    thresholds.pop();
    thresholds
}
fn quantize_samples(samples: &[i32], num_chunks: usize) -> Vec<i32> {
    if samples.is_empty() {
        return Vec::new();
    }
    let min = (*samples.iter().min().unwrap()).clamp(-512, 512);
    let mut counts = vec![0u32; 1025];
    for &s in samples {
        let offset = (s.clamp(-512, 512) - min) as u32;
        counts[offset as usize] = counts[offset as usize].wrapping_add(1);
    }
    let mut thresholds = quantize_histogram(&counts, num_chunks);
    for v in &mut thresholds {
        *v += min;
    }
    thresholds
}
trait QuantWord: Copy {
    fn mapped(v: usize) -> Self;
}
impl QuantWord for u8 {
    fn mapped(v: usize) -> Self {
        v as Self
    }
}
impl QuantWord for u16 {
    fn mapped(v: usize) -> Self {
        v as Self
    }
}
fn quant_map<T: QuantWord>(from: &[i32], to: &mut Vec<T>, num_pegs: usize, bias: i32) {
    to.resize(num_pegs, T::mapped(0));
    let mut mapped = 0;
    for (i, v) in to.iter_mut().enumerate() {
        while mapped < from.len() && (i as i32) - bias > from[mapped] {
            mapped += 1;
        }
        *v = T::mapped(mapped);
    }
}
impl TreeSamples {
    #[expect(
        clippy::too_many_arguments,
        reason = "Preserve source property/multiplier/sample inputs"
    )]
    #[expect(
        clippy::manual_is_multiple_of,
        reason = "Mirror the source modulo test."
    )]
    pub fn pre_quantize_properties(
        &mut self,
        range: &StaticPropRange,
        multiplier_info: &[ModularMultiplierInfo],
        group_pixel_count: &[u32],
        channel_pixel_count: &[u32],
        pixel_samples: &mut [i32],
        diff_samples: &mut [i32],
        max_property_values: usize,
    ) {
        let mut group_thresholds = Vec::new();
        let mut channel_thresholds = Vec::new();
        for v in multiplier_info {
            if v.range[0][0] != range[0][0] {
                channel_thresholds.push(v.range[0][0].wrapping_sub(1) as i32);
            }
            if v.range[0][1] != range[0][1] {
                channel_thresholds.push(v.range[0][1].wrapping_sub(1) as i32);
            }
            if v.range[1][0] != range[1][0] {
                group_thresholds.push(v.range[1][0].wrapping_sub(1) as i32);
            }
            if v.range[1][1] != range[1][1] {
                group_thresholds.push(v.range[1][1].wrapping_sub(1) as i32);
            }
        }
        channel_thresholds.sort_unstable();
        channel_thresholds.dedup();
        group_thresholds.sort_unstable();
        group_thresholds.dedup();
        self.compact_properties
            .resize_with(self.props_to_use.len(), Vec::new);
        let (
            mut abs_pixel_thresholds,
            mut pixel_thresholds,
            mut abs_diff_thresholds,
            mut diff_thresholds,
        ) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        self.property_mapping
            .resize_with(self.props_to_use.len() - self.num_static_props, Vec::new);
        for i in 0..self.props_to_use.len() {
            let p = self.props_to_use[i] as usize;
            let thresholds = if p == 0 {
                if channel_thresholds.is_empty() {
                    quantize_histogram(channel_pixel_count, max_property_values)
                } else {
                    channel_thresholds.clone()
                }
            } else if p == 1 {
                if group_thresholds.is_empty() {
                    quantize_histogram(group_pixel_count, max_property_values)
                } else {
                    group_thresholds.clone()
                }
            } else if p == 2 || p == 3 {
                let mut q = Vec::with_capacity(max_property_values.wrapping_sub(1));
                let mut j = 0usize;
                while j + 1 < max_property_values {
                    q.push(
                        ((j + 1).wrapping_mul(256) / max_property_values).wrapping_sub(1) as i32,
                    );
                    j += 1;
                }
                q
            } else if p == 6
                || p == 7
                || p == 8
                || (p >= NUM_NONREF_PROPERTIES && (p - NUM_NONREF_PROPERTIES) % 4 == 1)
            {
                if pixel_thresholds.is_empty() {
                    pixel_thresholds = quantize_samples(pixel_samples, max_property_values);
                }
                pixel_thresholds.clone()
            } else if p == 4
                || p == 5
                || (p >= NUM_NONREF_PROPERTIES && (p - NUM_NONREF_PROPERTIES) % 4 == 0)
            {
                if abs_pixel_thresholds.is_empty() {
                    if pixel_thresholds.is_empty() {
                        pixel_thresholds = quantize_samples(pixel_samples, max_property_values);
                    }
                    for v in pixel_samples.iter_mut() {
                        *v = v.abs();
                    }
                    abs_pixel_thresholds = quantize_samples(pixel_samples, max_property_values);
                }
                abs_pixel_thresholds.clone()
            } else if p >= NUM_NONREF_PROPERTIES && (p - NUM_NONREF_PROPERTIES) % 4 == 2 {
                if abs_diff_thresholds.is_empty() {
                    if diff_thresholds.is_empty() {
                        diff_thresholds = quantize_samples(diff_samples, max_property_values);
                    }
                    for v in diff_samples.iter_mut() {
                        *v = v.abs();
                    }
                    abs_diff_thresholds = quantize_samples(diff_samples, max_property_values);
                }
                abs_diff_thresholds.clone()
            } else if p == WP_PROP {
                if max_property_values < 32 {
                    vec![-127, -63, -31, -15, -7, -3, -1, 0, 1, 3, 7, 15, 31, 63, 127]
                } else if max_property_values < 64 {
                    vec![
                        -255, -191, -127, -95, -63, -47, -31, -23, -15, -11, -7, -5, -3, -1, 0, 1,
                        3, 5, 7, 11, 15, 23, 31, 47, 63, 95, 127, 191, 255,
                    ]
                } else {
                    vec![
                        -255, -223, -191, -159, -127, -111, -95, -79, -63, -55, -47, -39, -31, -27,
                        -23, -19, -15, -13, -11, -9, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5,
                        6, 7, 9, 11, 13, 15, 19, 23, 27, 31, 39, 47, 55, 63, 79, 95, 111, 127, 159,
                        191, 223, 255,
                    ]
                }
            } else {
                if diff_thresholds.is_empty() {
                    diff_thresholds = quantize_samples(diff_samples, max_property_values);
                }
                diff_thresholds.clone()
            };
            self.compact_properties[i] = thresholds;
            if i < self.num_static_props {
                quant_map(
                    &self.compact_properties[i],
                    &mut self.static_property_mapping[i],
                    (Self::PROPERTY_RANGE * 2 + 1) as usize,
                    Self::PROPERTY_RANGE,
                );
            } else {
                quant_map(
                    &self.compact_properties[i],
                    &mut self.property_mapping[i - self.num_static_props],
                    (Self::PROPERTY_RANGE * 2 + 1) as usize,
                    Self::PROPERTY_RANGE,
                );
            }
        }
    }
}

pub fn collect_pixel_samples(
    image: &Image,
    options: &ModularOptions,
    group_id: u32,
    group_pixel_count: &mut Vec<u32>,
    channel_pixel_count: &mut Vec<u32>,
    pixel_samples: &mut Vec<i32>,
    diff_samples: &mut Vec<i32>,
) {
    if options.nb_repeats == 0.0 {
        return;
    }
    if group_pixel_count.len() <= group_id as usize {
        group_pixel_count.resize(group_id.wrapping_add(1) as usize, 0);
    }
    if channel_pixel_count.len() < image.channel.len() {
        channel_pixel_count.resize(image.channel.len(), 0);
    }
    let mut rng = Rng::new(u64::from(group_id));
    let fraction = f64::from(options.nb_repeats) * 0.1;
    let fraction = if 0.99 < fraction { 0.99 } else { fraction } as f32;
    let dist = Rng::make_geometric(fraction);
    let mut total_pixels = 0usize;
    let mut ids = Vec::new();
    for (i, ch) in image.channel.iter().enumerate() {
        if i >= image.nb_meta_channels
            && (ch.w > options.max_chan_size || ch.h > options.max_chan_size)
        {
            break;
        }
        if ch.w <= 1 || ch.h == 0 {
            continue;
        }
        ids.push(i);
        let count = ch.w.wrapping_mul(ch.h);
        group_pixel_count[group_id as usize] =
            group_pixel_count[group_id as usize].wrapping_add(count as u32);
        channel_pixel_count[i] = channel_pixel_count[i].wrapping_add(count as u32);
        total_pixels = total_pixels.wrapping_add(count);
    }
    if ids.is_empty() {
        return;
    }
    let target = crate::jxl::base::float::mul_add_f32(
        fraction,
        total_pixels as f32,
        pixel_samples.len() as f32,
    ) as usize;
    if target > pixel_samples.len() {
        pixel_samples.reserve(target - pixel_samples.len());
    }
    let target = crate::jxl::base::float::mul_add_f32(
        fraction,
        total_pixels as f32,
        diff_samples.len() as f32,
    ) as usize;
    if target > diff_samples.len() {
        diff_samples.reserve(target - diff_samples.len());
    }
    let (mut i, mut y, mut x) = (0usize, 0usize, 0usize);
    let advance = |amount: usize, i: &mut usize, y: &mut usize, x: &mut usize| {
        *x = x.wrapping_add(amount);
        while *x >= image.channel[ids[*i]].w {
            *x -= image.channel[ids[*i]].w;
            *y += 1;
            if *y == image.channel[ids[*i]].h {
                *i += 1;
                *y = 0;
                if *i >= ids.len() {
                    return;
                }
            }
        }
    };
    advance(rng.geometric(&dist) as usize, &mut i, &mut y, &mut x);
    while i < ids.len() {
        let row = image.channel[ids[i]].row(y);
        pixel_samples.push(row[x]);
        let xp = if x == 0 { 1 } else { x - 1 };
        diff_samples.push((i64::from(row[x]) - i64::from(row[xp])) as i32);
        advance(
            rng.geometric(&dist).wrapping_add(1) as usize,
            &mut i,
            &mut y,
            &mut x,
        );
    }
}

/// Original one-lane Highway EstimateBits. Its Mul/Sub and FastLog2f operations
/// remain separate; the scalar Padded value equals the symbol count.
pub fn estimate_bits(counts: &[i32], num_symbols: usize) -> f32 {
    let mut total = 0i32;
    for &n in &counts[..num_symbols] {
        total = total.wrapping_add(n);
    }
    let minprob = 1.0f32 / ANS_TAB_SIZE as f32;
    let inv_total = 1.0f32 / total as f32;
    let mut bits = 0.0f32;
    for &n in &counts[..num_symbols] {
        let nf = n as f32;
        let probs = nf * inv_total;
        let mprobs = probs.max(minprob);
        let nbps = fast_log2f(mprobs);
        let nbps = if n == total { 0.0 } else { nbps };
        bits -= nf * nbps;
    }
    bits
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source split-node children and predictor fields"
)]
fn make_split_node(
    pos: usize,
    property: i32,
    splitval: i32,
    lpred: Predictor,
    loff: i64,
    rpred: Predictor,
    roff: i64,
    tree: &mut Tree,
) {
    tree[pos].lchild = tree.len() as u32;
    tree[pos].rchild = tree.len().wrapping_add(1) as u32;
    tree[pos].splitval = splitval;
    tree[pos].property = property as i16;
    tree.push(PropertyDecisionNode::leaf(rpred, roff, 1));
    tree.push(PropertyDecisionNode::leaf(lpred, loff, 1));
}
#[derive(Clone, Copy, Eq, PartialEq)]
enum IntersectionType {
    None,
    Partial,
    Inside,
}
fn box_intersects(
    needle: StaticPropRange,
    haystack: StaticPropRange,
    axis: &mut u32,
    value: &mut u32,
) -> IntersectionType {
    let mut partial = false;
    for i in 0..NUM_STATIC_PROPERTIES as usize {
        if haystack[i][0] >= needle[i][1] {
            return IntersectionType::None;
        }
        if haystack[i][1] <= needle[i][0] {
            return IntersectionType::None;
        }
        if haystack[i][0] <= needle[i][0] && haystack[i][1] >= needle[i][1] {
            continue;
        }
        partial = true;
        *axis = i as u32;
        *value = if haystack[i][0] > needle[i][0] && haystack[i][0] < needle[i][1] {
            haystack[i][0].wrapping_sub(1)
        } else {
            haystack[i][1].wrapping_sub(1)
        };
    }
    if partial {
        IntersectionType::Partial
    } else {
        IntersectionType::Inside
    }
}
fn split_tree_samples<const S: bool>(
    samples: &mut TreeSamples,
    begin: usize,
    pos: usize,
    end: usize,
    prop: usize,
    val: u32,
) {
    let mut begin_pos = begin;
    let mut end_pos = pos;
    loop {
        while begin_pos < pos && samples.property::<S>(prop, begin_pos) <= val as usize {
            begin_pos += 1;
        }
        while end_pos < end && samples.property::<S>(prop, end_pos) > val as usize {
            end_pos += 1;
        }
        if begin_pos < pos && end_pos < end {
            samples.swap(begin_pos, end_pos);
        }
        begin_pos = begin_pos.wrapping_add(1);
        end_pos = end_pos.wrapping_add(1);
        if !(begin_pos < pos && end_pos < end) {
            break;
        }
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve source residual/property histogram workspaces"
)]
fn collect_extra_bits_increase<const S: bool>(
    samples: &TreeSamples,
    rtokens: &[ResidualToken],
    count_increase: &mut [i32],
    extra_bits_increase: &mut [usize],
    begin: usize,
    end: usize,
    prop_idx: usize,
    max_symbols: usize,
) {
    for (i2, rt) in rtokens.iter().enumerate().take(end).skip(begin) {
        let cnt = samples.count(i2);
        let p = samples.property::<S>(prop_idx, i2);
        let sym = usize::from(rt.tok);
        let ebi = usize::from(rt.nbits).wrapping_mul(cnt);
        let index = p * max_symbols + sym;
        count_increase[index] = count_increase[index].wrapping_add(cnt as i32);
        extra_bits_increase[p] = extra_bits_increase[p].wrapping_add(ebi);
    }
}
#[derive(Clone, Copy)]
struct SplitInfo {
    prop: usize,
    val: u32,
    pos: usize,
    lcost: f32,
    rcost: f32,
    lpred: Predictor,
    rpred: Predictor,
}
impl Default for SplitInfo {
    fn default() -> Self {
        Self {
            prop: 0,
            val: 0,
            pos: 0,
            lcost: f32::MAX,
            rcost: f32::MAX,
            lpred: Predictor::ZERO,
            rpred: Predictor::ZERO,
        }
    }
}
impl SplitInfo {
    fn cost(self) -> f32 {
        self.lcost + self.rcost
    }
}
#[derive(Clone, Copy)]
struct CostInfo {
    cost: f32,
    extra_cost: f32,
    pred: Predictor,
}
impl Default for CostInfo {
    fn default() -> Self {
        Self {
            cost: f32::MAX,
            extra_cost: 0.0,
            pred: Predictor::ZERO,
        }
    }
}
impl CostInfo {
    fn cost(self) -> f32 {
        self.cost + self.extra_cost
    }
}
fn find_best_split(
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
        // Padded(max_symbols) uses one lane in the original scalar target.
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
        let base_bits =
            estimate_bits(&counts[pred * max_symbols..], max_symbols) + tot_extra_bits[pred] as f32;
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
                        let rcost = estimate_bits(&counts_above, max_symbols)
                            + tot_extra_bits[pred] as f32
                            - extra_bits_below as f32;
                        let lcost =
                            estimate_bits(&counts_below, max_symbols) + extra_bits_below as f32;
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
pub fn compute_best_tree(
    samples: &mut TreeSamples,
    threshold: f32,
    mul_info: &[ModularMultiplierInfo],
    range: StaticPropRange,
    fast_decode_multiplier: f32,
    tree: &mut Tree,
) -> Status {
    tree.push(PropertyDecisionNode::leaf(
        samples.predictor_from_index(0),
        0,
        1,
    ));
    if samples.num_properties() >= 64 {
        return Err(StatusCode::GenericError);
    }
    if samples.num_distinct_samples() > u32::MAX as usize {
        return Err(StatusCode::GenericError);
    }
    find_best_split(
        samples,
        threshold,
        mul_info,
        range,
        fast_decode_multiplier,
        tree,
    );
    Ok(())
}

pub fn tokenize_tree(
    tree: &[PropertyDecisionNode],
    tokens: &mut Vec<Token>,
    decoder_tree: &mut Tree,
) -> Status {
    if tree.len() > MAX_TREE_SIZE {
        return Err(StatusCode::GenericError);
    }
    let mut queue = VecDeque::from([0i32]);
    let mut leaf_id = 0usize;
    decoder_tree.clear();
    while let Some(cur) = queue.pop_front() {
        let node = &tree[cur as usize];
        if node.property < -1 {
            return Err(StatusCode::GenericError);
        }
        tokens.push(Token::new(
            PROPERTY_CONTEXT as u32,
            (i32::from(node.property) + 1) as u32,
        ));
        if node.property == -1 {
            tokens.push(Token::new(PREDICTOR_CONTEXT as u32, node.predictor.0));
            tokens.push(Token::new(
                OFFSET_CONTEXT as u32,
                pack_signed(node.predictor_offset as i32),
            ));
            // Num0BitsBelowLS1Bit_Nonzero requires a nonzero multiplier.
            assert_ne!(node.multiplier, 0);
            let mul_log = node.multiplier.trailing_zeros();
            let mul_bits = (node.multiplier >> mul_log) - 1;
            tokens.push(Token::new(MULTIPLIER_LOG_CONTEXT as u32, mul_log));
            tokens.push(Token::new(MULTIPLIER_BITS_CONTEXT as u32, mul_bits));
            if node.predictor.0 >= Predictor::BEST.0 {
                return Err(StatusCode::GenericError);
            }
            decoder_tree.push(PropertyDecisionNode::new(
                -1,
                0,
                leaf_id as i32,
                0,
                node.predictor,
                node.predictor_offset,
                node.multiplier,
            ));
            leaf_id += 1;
            continue;
        }
        decoder_tree.push(PropertyDecisionNode::new(
            i32::from(node.property),
            node.splitval,
            (decoder_tree.len() + queue.len() + 1) as i32,
            (decoder_tree.len() + queue.len() + 2) as i32,
            Predictor::ZERO,
            0,
            1,
        ));
        queue.push_back(node.lchild as i32);
        queue.push_back(node.rchild as i32);
        tokens.push(Token::new(
            SPLIT_VAL_CONTEXT as u32,
            pack_signed(node.splitval),
        ));
    }
    Ok(())
}
