//! Shared helpers of the WP20 parity tests: the SplitMix64 generator that the
//! WP4, WP6 and WP10 tests use for their seeded inputs, and the value sets
//! that reach the saturation and sign edges of the NEON arithmetic.
//!
//! Include with `#[path = "support/wp20.rs"] mod support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

/// SplitMix64, the deterministic generator of the earlier packages' tests.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A value in `0..bound`.
    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    pub fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }

    pub fn bytes(&mut self, count: usize) -> Vec<u8> {
        (0..count).map(|_| self.byte()).collect()
    }

    /// Bytes that are an edge value (0, 1, 127, 128, 254, 255) a quarter of
    /// the time and random otherwise.
    pub fn edge_bytes(&mut self, count: usize) -> Vec<u8> {
        const EDGES: [u8; 6] = [0, 1, 127, 128, 254, 255];
        (0..count)
            .map(|_| {
                if self.below(4) == 0 {
                    EDGES[self.below(EDGES.len() as u64) as usize]
                } else {
                    self.byte()
                }
            })
            .collect()
    }

    pub fn words(&mut self, count: usize) -> Vec<u32> {
        (0..count).map(|_| self.next_u64() as u32).collect()
    }

    /// `i16` values in `-range..=range`, with the two bounds a quarter of the
    /// time.
    pub fn i16s(&mut self, count: usize, range: i32) -> Vec<i16> {
        (0..count)
            .map(|_| match self.below(8) {
                0 => range as i16,
                1 => (-range) as i16,
                _ => (self.below(2 * range as u64 + 1) as i32 - range) as i16,
            })
            .collect()
    }
}

/// The widths that cover every remainder of the 8- and 16-lane loops and
/// the lengths around the 32 and 64 boundaries.
pub fn widths() -> impl Iterator<Item = usize> {
    (0..=40).chain(63..=66).chain([127, 128, 129])
}
