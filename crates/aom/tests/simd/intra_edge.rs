// Supplementary transcription of upstream/aom/test/intra_edge_test.cc
// (UpsampleTest8B, FilterEdgeTest8B, UpsampleTestHB, and FilterEdgeTestHB
// RandomValues, NEON instantiations). The FunctionEquivalenceTest generator
// seed, iteration counts, buffer sizes and offsets, the random draw order, and
// the exact-match ranges follow the original. DISABLED_Speed is not
// transcribed.
#![cfg(target_arch = "aarch64")]
use aom::simd::av1__common__arm__highbd_reconintra_neon::{
    av1_highbd_filter_intra_edge_neon, av1_highbd_upsample_intra_edge_neon,
};
use aom::simd::av1__common__arm__reconintra_neon::{av1_filter_intra_edge_neon, av1_upsample_intra_edge_neon};

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_upsample_intra_edge_c(p: *mut u8, sz: i32);
    fn av1_filter_intra_edge_c(p: *mut u8, sz: i32, strength: i32);
    fn av1_highbd_upsample_intra_edge_c(p: *mut u16, sz: i32, bd: i32);
    fn av1_highbd_filter_intra_edge_c(p: *mut u16, sz: i32, strength: i32);
}

const K_ITERATIONS: usize = 1000000;

// UpsampleTest
const UP_K_BUF_SIZE: usize = 2 * 64 + 32;
const UP_K_OFFSET: usize = 16;

// FilterEdgeTest
const FE_K_MAX_EDGE: usize = 2 * 64;
const FE_K_BUF_SIZE: usize = FE_K_MAX_EDGE + 32;
const FE_K_OFFSET: usize = 15;

#[test]
fn upsample_intra_edge_8b_neon_random_values() {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut edge_ref_data = [0u8; UP_K_BUF_SIZE];
    let mut edge_tst_data = [0u8; UP_K_BUF_SIZE];
    for iter in 0..K_ITERATIONS {
        let size = 4 * (rng.pseudo_uniform(4) + 1);
        let mut i = 0;
        let mut pix = 0u8;
        while i < UP_K_OFFSET + size as usize {
            pix = rng.rand8();
            edge_ref_data[i] = pix;
            edge_tst_data[i] = edge_ref_data[i];
            i += 1;
        }
        // Extend final sample
        while i < UP_K_BUF_SIZE {
            edge_ref_data[i] = pix;
            edge_tst_data[i] = pix;
            i += 1;
        }
        // SAFETY: edge pointers sit kOffset into kBufSize buffers, as in the
        // original.
        unsafe {
            av1_upsample_intra_edge_c(edge_ref_data.as_mut_ptr().add(UP_K_OFFSET), size);
            av1_upsample_intra_edge_neon(edge_tst_data.as_mut_ptr().add(UP_K_OFFSET), size);
        }
        let max_idx = (size - 1) * 2;
        for r in -2..=max_idx {
            let k = (UP_K_OFFSET as i32 + r) as usize;
            assert_eq!(edge_ref_data[k], edge_tst_data[k], "iter {} size {} r {}", iter, size, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

#[test]
fn filter_intra_edge_8b_neon_random_values() {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut edge_ref_data = [0u8; FE_K_BUF_SIZE];
    let mut edge_tst_data = [0u8; FE_K_BUF_SIZE];
    for iter in 0..K_ITERATIONS {
        let strength = rng.pseudo_uniform(4);
        let size = 4 * (rng.pseudo_uniform(128 / 4) + 1) + 1;
        for i in 0..FE_K_OFFSET + size as usize {
            let pix = rng.rand8();
            edge_ref_data[i] = pix;
            edge_tst_data[i] = pix;
        }
        // SAFETY: edge pointers sit kOffset into kBufSize buffers, as in the
        // original.
        unsafe {
            av1_filter_intra_edge_c(edge_ref_data.as_mut_ptr().add(FE_K_OFFSET), size, strength);
            av1_filter_intra_edge_neon(edge_tst_data.as_mut_ptr().add(FE_K_OFFSET), size, strength);
        }
        for r in 0..size as usize {
            assert_eq!(edge_ref_data[FE_K_OFFSET + r], edge_tst_data[FE_K_OFFSET + r],
                       "iter {} size {} strength {} r {}", iter, size, strength, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

#[test]
fn upsample_intra_edge_hbd_neon_random_values() {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut edge_ref_data = [0u16; UP_K_BUF_SIZE];
    let mut edge_tst_data = [0u16; UP_K_BUF_SIZE];
    for iter in 0..K_ITERATIONS {
        let bit_depth = match rng.pseudo_uniform(3) {
            0 => 8,
            1 => 10,
            _ => 12,
        };
        let hi = 1 << bit_depth;
        let size = 4 * (rng.pseudo_uniform(4) + 1);
        let mut i = 0;
        let mut pix = 0u16;
        while i < UP_K_OFFSET + size as usize {
            pix = rng.pseudo_uniform(hi) as u16;
            edge_ref_data[i] = pix;
            edge_tst_data[i] = pix;
            i += 1;
        }
        // Extend final sample
        while i < UP_K_BUF_SIZE {
            edge_ref_data[i] = pix;
            edge_tst_data[i] = pix;
            i += 1;
        }
        // SAFETY: edge pointers sit kOffset into kBufSize buffers, as in the
        // original.
        unsafe {
            av1_highbd_upsample_intra_edge_c(edge_ref_data.as_mut_ptr().add(UP_K_OFFSET), size, bit_depth);
            av1_highbd_upsample_intra_edge_neon(edge_tst_data.as_mut_ptr().add(UP_K_OFFSET), size, bit_depth);
        }
        let max_idx = (size - 1) * 2;
        for r in -2..=max_idx {
            let k = (UP_K_OFFSET as i32 + r) as usize;
            assert_eq!(edge_ref_data[k], edge_tst_data[k], "iter {} bd {} size {} r {}", iter, bit_depth, size, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

#[test]
fn filter_intra_edge_hbd_neon_random_values() {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut edge_ref_data = [0u16; FE_K_BUF_SIZE];
    let mut edge_tst_data = [0u16; FE_K_BUF_SIZE];
    for iter in 0..K_ITERATIONS {
        let bit_depth = match rng.pseudo_uniform(3) {
            0 => 8,
            1 => 10,
            _ => 12,
        };
        let hi = 1 << bit_depth;
        let strength = rng.pseudo_uniform(4);
        let size = 4 * (rng.pseudo_uniform(128 / 4) + 1) + 1;
        for i in 0..FE_K_OFFSET + size as usize {
            let pix = rng.pseudo_uniform(hi) as u16;
            edge_ref_data[i] = pix;
            edge_tst_data[i] = pix;
        }
        // SAFETY: edge pointers sit kOffset into kBufSize buffers, as in the
        // original.
        unsafe {
            av1_highbd_filter_intra_edge_c(edge_ref_data.as_mut_ptr().add(FE_K_OFFSET), size, strength);
            av1_highbd_filter_intra_edge_neon(edge_tst_data.as_mut_ptr().add(FE_K_OFFSET), size, strength);
        }
        for r in 0..size as usize {
            assert_eq!(edge_ref_data[FE_K_OFFSET + r], edge_tst_data[FE_K_OFFSET + r],
                       "iter {} bd {} size {} strength {} r {}", iter, bit_depth, size, strength, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}
