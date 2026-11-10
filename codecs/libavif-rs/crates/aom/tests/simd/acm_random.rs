// Rust transcription of upstream/aom/test/acm_random.h and the GoogleTest
// `testing::internal::Random` generator it wraps, so the supplementary
// SIMD-versus-C comparisons draw the same input sequences as the original
// libaom tests for a given seed.
#![allow(dead_code)]

const K_MAX_RANGE: u32 = 1 << 31;

pub struct ACMRandom {
    state: u32,
}

impl ACMRandom {
    pub const fn deterministic_seed() -> u32 {
        0xbaba
    }

    pub fn new() -> Self {
        Self::with_seed(Self::deterministic_seed())
    }

    pub fn with_seed(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn reset(&mut self, seed: u32) {
        self.state = seed;
    }

    fn generate(&mut self, range: u32) -> u32 {
        // These constants are the same as are used in glibc's rand(3).
        self.state = ((1103515245u64 * self.state as u64 + 12345) as u32) % K_MAX_RANGE;
        assert!(range > 0 && range <= K_MAX_RANGE);
        self.state % range
    }

    pub fn rand31(&mut self) -> u32 {
        self.generate(K_MAX_RANGE)
    }

    pub fn rand16(&mut self) -> u16 {
        ((self.generate(K_MAX_RANGE) >> 15) & 0xffff) as u16
    }

    pub fn rand16_signed(&mut self) -> i16 {
        self.rand16() as i16
    }

    pub fn rand15(&mut self) -> u16 {
        ((self.generate(K_MAX_RANGE) >> 16) & 0x7fff) as u16
    }

    pub fn rand15_signed(&mut self) -> i16 {
        (self.rand15() as i16) - (1 << 14)
    }

    pub fn rand12(&mut self) -> u16 {
        ((self.generate(K_MAX_RANGE) >> 19) & 0xfff) as u16
    }

    pub fn rand8(&mut self) -> u8 {
        ((self.generate(K_MAX_RANGE) >> 23) & 0xff) as u8
    }

    pub fn rand8_extremes(&mut self) -> u8 {
        let r = self.rand8();
        if r < 128 { r << 4 } else { r >> 4 }
    }

    pub fn pseudo_uniform(&mut self, range: i32) -> i32 {
        self.generate(range as u32) as i32
    }
}
