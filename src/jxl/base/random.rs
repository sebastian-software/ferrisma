// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Rng and distributions from lib/jxl/base/random.h. Preserve the source's
//! xorshift state, float bit construction, logf rounding, and scalar Clang
//! contraction. Distribution arguments retain the source's valid preconditions.

use super::float::mul_add_f32;
pub type GeometricDistribution = f32;
pub struct Rng {
    s: [u64; 2],
}
impl Rng {
    pub const fn new(seed: u64) -> Self {
        Self {
            s: [0x94D049BB133111EB, 0xBF58476D1CE4E5B9u64.wrapping_add(seed)],
        }
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut s1 = self.s[0];
        let s0 = self.s[1];
        let bits = s1.wrapping_add(s0);
        self.s[0] = s0;
        s1 ^= s1 << 23;
        s1 ^= s0 ^ (s1 >> 18) ^ (s0 >> 5);
        self.s[1] = s1;
        bits
    }
    pub fn uniform_i(&mut self, begin: i64, end: i64) -> i64 {
        (self.next_u64() % ((end - begin) as u64)) as i64 + begin
    }
    pub fn uniform_u(&mut self, begin: u64, end: u64) -> u64 {
        (self.next_u64() % end.wrapping_sub(begin)).wrapping_add(begin)
    }
    pub fn uniform_f(&mut self, begin: f32, end: f32) -> f32 {
        let u = (self.next_u64() >> (64 - 23)) as u32 | 0x3f800000;
        let f = f32::from_bits(u);
        mul_add_f32(end - begin, f - 1.0, begin)
    }
    pub fn bernoulli(&mut self, p: f32) -> bool {
        self.uniform_f(0.0, 1.0) < p
    }
    pub fn make_geometric(p: f32) -> GeometricDistribution {
        (1.0f64 / f64::from((1.0f32 - p).ln())) as f32
    }
    pub fn geometric(&mut self, dist: &GeometricDistribution) -> u32 {
        let f = self.uniform_f(0.0, 1.0);
        let log = (1.0f32 - f).ln() * *dist;
        // C++ float-to-uint outside this range is undefined.
        assert!(log.is_finite() && log >= 0.0 && f64::from(log) < 4294967296.0);
        log as u32
    }
    pub fn shuffle<T>(&mut self, t: &mut [T], n: usize) {
        let mut i = 0;
        while i + 1 < n {
            let a = self.uniform_u(i as u64, n as u64) as usize;
            t.swap(a, i);
            i += 1;
        }
    }
}
