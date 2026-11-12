// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Histogram storage and ANS strategy from `lib/jxl/enc_ans_params.h`.
//! Encoder parameter fields/defaults and histogram storage are translated;
//! ForModular follows the original policy in enc_ans.cc.

use super::ans_common::create_flat_histogram;
use super::common::SpeedTier;
use super::dec_ans::HybridUintConfig;
use super::enc_params::CompressParams;
use super::modular::options::Predictor;

pub type ANSHistBin = i32;

#[derive(Clone, Copy, Debug)]
pub enum ANSHistogramStrategy {
    Fast,
    Approximate,
    Precise,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClusteringType {
    Fastest,
    Fast,
    Best,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HybridUintMethod {
    None,
    Uint000,
    Fast,
    ContextMap,
    Best,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LZ77Method {
    None,
    RLE,
    LZ77b1w3f,
    LZ77b3w3f,
    LZ77b7w3f,
    LZ77b15w3f,
    LZ77b31w3f,
    LZ77b1w3t,
    LZ77b3w3t,
    LZ77b7w3t,
    LZ77b15w3t,
    LZ77b31w3t,
    Optc1,
    Optc3,
    Optc8,
    Optc256,
}

#[derive(Clone)]
pub struct HistogramParams {
    pub clustering: ClusteringType,
    pub uint_method: HybridUintMethod,
    pub lz77_method: LZ77Method,
    pub ans_histogram_strategy: ANSHistogramStrategy,
    pub image_widths: Vec<usize>,
    pub max_histograms: usize,
    pub force_huffman: bool,
    pub initialize_global_state: bool,
    pub streaming_mode: bool,
    pub add_missing_symbols: bool,
    pub add_fixed_histograms: bool,
}

impl Default for HistogramParams {
    fn default() -> Self {
        Self {
            clustering: ClusteringType::Best,
            uint_method: HybridUintMethod::Best,
            lz77_method: LZ77Method::RLE,
            ans_histogram_strategy: ANSHistogramStrategy::Precise,
            image_widths: Vec::new(),
            max_histograms: usize::MAX,
            force_huffman: false,
            initialize_global_state: true,
            streaming_mode: false,
            add_missing_symbols: false,
            add_fixed_histograms: false,
        }
    }
}
impl HistogramParams {
    /// HistogramParams(SpeedTier tier, size_t num_ctx); `num_ctx` is unused.
    pub fn with_tier(tier: SpeedTier, _num_ctx: usize) -> Self {
        let mut params = Self::default();
        if tier > SpeedTier::Falcon {
            params.clustering = ClusteringType::Fastest;
            params.lz77_method = LZ77Method::None;
        } else if tier > SpeedTier::Tortoise {
            params.clustering = ClusteringType::Fast;
        } else {
            params.clustering = ClusteringType::Best;
        }
        if tier > SpeedTier::Tortoise {
            params.uint_method = HybridUintMethod::None;
        }
        if tier >= SpeedTier::Squirrel {
            params.ans_histogram_strategy = ANSHistogramStrategy::Approximate;
        }
        params
    }
    pub fn for_modular(
        cparams: &CompressParams<'_>,
        extra_dc_precision: &[u8],
        streaming_mode: bool,
    ) -> Self {
        let mut params = Self {
            streaming_mode,
            ..Self::default()
        };
        if cparams.speed_tier > SpeedTier::Kitten {
            params.clustering = ClusteringType::Fast;
            params.ans_histogram_strategy = if cparams.speed_tier > SpeedTier::Thunder {
                ANSHistogramStrategy::Fast
            } else {
                ANSHistogramStrategy::Approximate
            };
            params.lz77_method = if cparams.modular_mode && cparams.speed_tier <= SpeedTier::Hare {
                LZ77Method::RLE
            } else {
                LZ77Method::None
            };
            if (!extra_dc_precision.is_empty() && extra_dc_precision[0] != 0)
                || (cparams.modular_mode && cparams.speed_tier < SpeedTier::Cheetah)
            {
                params.uint_method = HybridUintMethod::Fast;
            } else {
                params.uint_method = HybridUintMethod::None;
            }
        } else if cparams.speed_tier <= SpeedTier::Tortoise {
            params.lz77_method = LZ77Method::Optc256;
        } else {
            params.lz77_method = LZ77Method::LZ77b3w3f;
        }
        if cparams.decoding_speed_tier >= 2 {
            params.max_histograms = 12;
        }
        if cparams.options.predictor == Predictor::ZERO && cparams.modular_mode {
            params.lz77_method = if cparams.speed_tier >= SpeedTier::Falcon {
                LZ77Method::None
            } else if cparams.speed_tier >= SpeedTier::Hare {
                LZ77Method::RLE
            } else if cparams.speed_tier >= SpeedTier::Kitten {
                LZ77Method::LZ77b3w3f
            } else {
                LZ77Method::Optc256
            };
        }
        params
    }
    pub fn new(tier: SpeedTier, _num_ctx: usize) -> Self {
        let mut result = Self::default();
        if tier > SpeedTier::Falcon {
            result.clustering = ClusteringType::Fastest;
            result.lz77_method = LZ77Method::None;
        } else if tier > SpeedTier::Tortoise {
            result.clustering = ClusteringType::Fast;
        } else {
            result.clustering = ClusteringType::Best;
        }
        if tier > SpeedTier::Tortoise {
            result.uint_method = HybridUintMethod::None;
        }
        if tier >= SpeedTier::Squirrel {
            result.ans_histogram_strategy = ANSHistogramStrategy::Approximate;
        }
        result
    }
    pub fn uint_config(&self) -> HybridUintConfig {
        match self.uint_method {
            HybridUintMethod::ContextMap => HybridUintConfig::new(2, 0, 1),
            HybridUintMethod::Uint000 => HybridUintConfig::new(0, 0, 0),
            _ => HybridUintConfig::default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct Histogram {
    pub counts: Vec<ANSHistBin>,
    pub total_count: usize,
    /// Source cache, deliberately not kept up to date by every operation.
    pub entropy: f32,
}

impl Histogram {
    pub const ROUNDING: usize = 8;

    pub fn new(length: usize) -> Self {
        let mut result = Self::default();
        result.ensure_capacity(length);
        result
    }
    pub fn flat(length: i32, total_count: i32) -> Self {
        Self {
            counts: create_flat_histogram(length, total_count),
            total_count: total_count as usize,
            entropy: 0.0,
        }
    }
    pub fn clear(&mut self) {
        self.counts.clear();
        self.total_count = 0;
        self.entropy = 0.0;
    }
    pub fn add(&mut self, symbol: usize) {
        if self.counts.len() <= symbol {
            self.ensure_capacity(symbol + 1);
        }
        self.counts[symbol] += 1;
        self.total_count = self.total_count.wrapping_add(1);
    }
    pub fn ensure_capacity(&mut self, length: usize) {
        self.counts.resize(
            length.wrapping_add(Self::ROUNDING - 1) / Self::ROUNDING * Self::ROUNDING,
            0,
        );
    }
    pub fn fast_add(&mut self, symbol: usize) {
        self.counts[symbol] += 1;
    }
    pub fn add_histogram(&mut self, other: &Self) {
        if other.counts.len() > self.counts.len() {
            self.counts.resize(other.counts.len(), 0);
        }
        for (count, &value) in self.counts.iter_mut().zip(&other.counts) {
            *count += value;
        }
        self.total_count = self.total_count.wrapping_add(other.total_count);
    }
    pub fn alphabet_size(&self) -> usize {
        for i in (0..self.counts.len()).rev() {
            if self.counts[i] > 0 {
                return i + 1;
            }
        }
        0
    }
    pub fn max_symbol(&self) -> usize {
        if self.total_count == 0 {
            return 0;
        }
        for i in (1..self.counts.len()).rev() {
            if self.counts[i] != 0 {
                return i;
            }
        }
        0
    }
    pub fn swap(&mut self, other: &mut Self) {
        std::mem::swap(self, other);
    }
}
