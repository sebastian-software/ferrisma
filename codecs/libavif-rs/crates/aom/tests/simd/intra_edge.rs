// Supplementary transcription of upstream/aom/test/intra_edge_test.cc
// (UpsampleTest8B, FilterEdgeTest8B, UpsampleTestHB, and FilterEdgeTestHB
// RandomValues, NEON and SSE4_1 instantiations). The FunctionEquivalenceTest
// generator seed, iteration counts, buffer sizes and offsets, the random draw
// order, and the exact-match ranges follow the original. DISABLED_Speed is not
// transcribed.
//
// The SSE4.1 8-bit edge filter reads up to 7 bytes past the end of the
// kBufSize test buffer (the 16-byte `in + 16` load of its last iteration). In
// the original those bytes belong to the fixture members declared after
// `edge_tst_data_[]`, so the FilterEdgeTest buffers live here in one
// #[repr(C)] struct with the original member order and the read stays inside
// the same object.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
use super::acm_random::ACMRandom;

type Up8B = unsafe extern "C-unwind" fn(*mut u8, i32);
type Fe8B = unsafe extern "C-unwind" fn(*mut u8, i32, i32);
type UpHB = unsafe extern "C-unwind" fn(*mut u16, i32, i32);
type FeHB = unsafe extern "C-unwind" fn(*mut u16, i32, i32);

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

/// The data members of FilterEdgeTest<F, T> in declaration order.
#[repr(C)]
struct FilterEdgeData<T> {
    edge_ref_data: [T; FE_K_BUF_SIZE],
    edge_tst_data: [T; FE_K_BUF_SIZE],
    edge_ref: *mut T,
    edge_tst: *mut T,
    size: i32,
    strength: i32,
}

impl<T: Copy + Default> FilterEdgeData<T> {
    fn new() -> Box<Self> {
        Box::new(FilterEdgeData {
            edge_ref_data: [T::default(); FE_K_BUF_SIZE],
            edge_tst_data: [T::default(); FE_K_BUF_SIZE],
            edge_ref: core::ptr::null_mut(),
            edge_tst: core::ptr::null_mut(),
            size: 0,
            strength: 0,
        })
    }
}

fn upsample_8b_random_values(tst_func: Up8B) {
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
            tst_func(edge_tst_data.as_mut_ptr().add(UP_K_OFFSET), size);
        }
        let max_idx = (size - 1) * 2;
        for r in -2..=max_idx {
            let k = (UP_K_OFFSET as i32 + r) as usize;
            assert_eq!(edge_ref_data[k], edge_tst_data[k], "iter {} size {} r {}", iter, size, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

fn filter_edge_8b_random_values(tst_func: Fe8B) {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut t = FilterEdgeData::<u8>::new();
    for iter in 0..K_ITERATIONS {
        t.strength = rng.pseudo_uniform(4);
        t.size = 4 * (rng.pseudo_uniform(128 / 4) + 1) + 1;
        for i in 0..FE_K_OFFSET + t.size as usize {
            let pix = rng.rand8();
            t.edge_ref_data[i] = pix;
            t.edge_tst_data[i] = pix;
        }
        t.edge_ref = t.edge_ref_data.as_mut_ptr().wrapping_add(FE_K_OFFSET);
        t.edge_tst = t.edge_tst_data.as_mut_ptr().wrapping_add(FE_K_OFFSET);
        // SAFETY: edge pointers sit kOffset into kBufSize buffers of the
        // fixture, as in the original.
        unsafe {
            av1_filter_intra_edge_c(t.edge_ref, t.size, t.strength);
            tst_func(t.edge_tst, t.size, t.strength);
        }
        for r in 0..t.size as usize {
            assert_eq!(t.edge_ref_data[FE_K_OFFSET + r], t.edge_tst_data[FE_K_OFFSET + r],
                       "iter {} size {} strength {} r {}", iter, t.size, t.strength, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

fn upsample_hbd_random_values(tst_func: UpHB) {
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
            tst_func(edge_tst_data.as_mut_ptr().add(UP_K_OFFSET), size, bit_depth);
        }
        let max_idx = (size - 1) * 2;
        for r in -2..=max_idx {
            let k = (UP_K_OFFSET as i32 + r) as usize;
            assert_eq!(edge_ref_data[k], edge_tst_data[k], "iter {} bd {} size {} r {}", iter, bit_depth, size, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

fn filter_edge_hbd_random_values(tst_func: FeHB) {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut t = FilterEdgeData::<u16>::new();
    for iter in 0..K_ITERATIONS {
        let bit_depth = match rng.pseudo_uniform(3) {
            0 => 8,
            1 => 10,
            _ => 12,
        };
        let hi = 1 << bit_depth;
        t.strength = rng.pseudo_uniform(4);
        t.size = 4 * (rng.pseudo_uniform(128 / 4) + 1) + 1;
        for i in 0..FE_K_OFFSET + t.size as usize {
            let pix = rng.pseudo_uniform(hi) as u16;
            t.edge_ref_data[i] = pix;
            t.edge_tst_data[i] = pix;
        }
        t.edge_ref = t.edge_ref_data.as_mut_ptr().wrapping_add(FE_K_OFFSET);
        t.edge_tst = t.edge_tst_data.as_mut_ptr().wrapping_add(FE_K_OFFSET);
        // SAFETY: edge pointers sit kOffset into kBufSize buffers of the
        // fixture, as in the original.
        unsafe {
            av1_highbd_filter_intra_edge_c(t.edge_ref, t.size, t.strength);
            tst_func(t.edge_tst, t.size, t.strength);
        }
        for r in 0..t.size as usize {
            assert_eq!(t.edge_ref_data[FE_K_OFFSET + r], t.edge_tst_data[FE_K_OFFSET + r],
                       "iter {} bd {} size {} strength {} r {}", iter, bit_depth, t.size, t.strength, r);
        }
    }
    eprintln!("{} edges compared", K_ITERATIONS);
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::av1__common__arm__highbd_reconintra_neon::{
        av1_highbd_filter_intra_edge_neon, av1_highbd_upsample_intra_edge_neon,
    };
    use aom::simd::av1__common__arm__reconintra_neon::{av1_filter_intra_edge_neon, av1_upsample_intra_edge_neon};

    #[test]
    fn upsample_intra_edge_8b_neon_random_values() {
        upsample_8b_random_values(av1_upsample_intra_edge_neon);
    }

    #[test]
    fn filter_intra_edge_8b_neon_random_values() {
        filter_edge_8b_random_values(av1_filter_intra_edge_neon);
    }

    #[test]
    fn upsample_intra_edge_hbd_neon_random_values() {
        upsample_hbd_random_values(av1_highbd_upsample_intra_edge_neon);
    }

    #[test]
    fn filter_intra_edge_hbd_neon_random_values() {
        filter_edge_hbd_random_values(av1_highbd_filter_intra_edge_neon);
    }
}

#[cfg(target_arch = "x86_64")]
mod sse4_1 {
    use super::*;
    use aom::simd::av1__common__x86__intra_edge_sse4::*;

    fn assert_sse4_1() {
        assert!(std::arch::is_x86_feature_detected!("sse4.1"), "SSE4_1 instantiation needs an SSE4.1 CPU");
    }

    #[test]
    fn upsample_intra_edge_8b_sse4_1_random_values() {
        assert_sse4_1();
        upsample_8b_random_values(av1_upsample_intra_edge_sse4_1);
    }

    #[test]
    fn filter_intra_edge_8b_sse4_1_random_values() {
        assert_sse4_1();
        filter_edge_8b_random_values(av1_filter_intra_edge_sse4_1);
    }

    #[test]
    fn upsample_intra_edge_hbd_sse4_1_random_values() {
        assert_sse4_1();
        upsample_hbd_random_values(av1_highbd_upsample_intra_edge_sse4_1);
    }

    #[test]
    fn filter_intra_edge_hbd_sse4_1_random_values() {
        assert_sse4_1();
        filter_edge_hbd_random_values(av1_highbd_filter_intra_edge_sse4_1);
    }
}
