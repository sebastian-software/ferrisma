// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Histogram storage and ANS strategy from `lib/jxl/enc_ans_params.h`.
//! Clustering, hybrid/LZ77 strategy selection, and entropy cache evaluation
//! remain outside this boundary until their dependencies are translated.

use super::ans_common::create_flat_histogram;

pub type ANSHistBin = i32;

#[derive(Clone, Copy, Debug)]
pub enum ANSHistogramStrategy {
    Fast,
    Approximate,
    Precise,
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
