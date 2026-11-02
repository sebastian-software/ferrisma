// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Original scalar Xorshift128Plus from lib/jxl/xorshift128plus-inl.h.
//! The source adapts the MIT-licensed vpxyz/xorshift generator. Eight independent
//! generators and their original seed schedules are retained; integer arithmetic
//! is explicitly modulo 2^64. Other Highway targets remain separate work.

pub struct Xorshift128Plus {
    s0: [u64; Self::N],
    s1: [u64; Self::N],
}
impl Xorshift128Plus {
    pub const N: usize = 8;
    pub fn new(seed: u64) -> Self {
        let mut state = Self {
            s0: [0; Self::N],
            s1: [0; Self::N],
        };
        state.s0[0] = Self::split_mix64(seed.wrapping_add(0x9e3779b97f4a7c15));
        state.s1[0] = Self::split_mix64(state.s0[0]);
        for i in 1..Self::N {
            state.s0[i] = Self::split_mix64(state.s1[i - 1]);
            state.s1[i] = Self::split_mix64(state.s0[i]);
        }
        state
    }
    pub fn from_seeds(seed1: u32, seed2: u32, seed3: u32, seed4: u32) -> Self {
        let mut state = Self {
            s0: [0; Self::N],
            s1: [0; Self::N],
        };
        state.s0[0] = Self::split_mix64(
            ((u64::from(seed1) << 32) + u64::from(seed2)).wrapping_add(0x9e3779b97f4a7c15),
        );
        state.s1[0] = Self::split_mix64(
            ((u64::from(seed3) << 32) + u64::from(seed4)).wrapping_add(0x9e3779b97f4a7c15),
        );
        for i in 1..Self::N {
            state.s0[i] = Self::split_mix64(state.s0[i - 1]);
            state.s1[i] = Self::split_mix64(state.s1[i - 1]);
        }
        state
    }
    pub fn fill(&mut self) -> [u64; Self::N] {
        let mut bits = [0; Self::N];
        for (i, value) in bits.iter_mut().enumerate() {
            let mut s1 = self.s0[i];
            let s0 = self.s1[i];
            *value = s1.wrapping_add(s0);
            self.s0[i] = s0;
            s1 ^= s1 << 23;
            s1 ^= s0 ^ (s1 >> 18) ^ (s0 >> 5);
            self.s1[i] = s1;
        }
        bits
    }
    fn split_mix64(mut z: u64) -> u64 {
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
