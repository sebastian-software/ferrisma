// Supplementary transcription of upstream/aom/test/variance_test.cc for the
// NEON instantiations of MseWxHTest (aom_mse_wxh_16bit_neon), Mse16xHTest
// (aom_mse_16xh_16bit_neon: RefMse and RefMseExtreme) and MseHBDWxHTest
// (aom_mse_wxh_16bit_highbd_neon at 10 bits), with the original seeds,
// buffer sizes, strides (equal to the width) and exact checks. The speed
// tests are not transcribed. The SSE2 and AVX2 instantiations (MseWxHTest,
// Mse16xHTest, MseHBDWxHTest at 10 bits) use the same parameters.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

#[cfg(target_arch = "aarch64")]
use aom::simd::aom_dsp__arm__highbd_variance_neon::aom_mse_wxh_16bit_highbd_neon;
#[cfg(target_arch = "aarch64")]
use aom::simd::aom_dsp__arm__variance_neon::{aom_mse_16xh_16bit_neon, aom_mse_wxh_16bit_neon};

use super::acm_random::ACMRandom;
#[cfg(target_arch = "x86_64")]
use aom::simd::aom_dsp__x86__highbd_variance_avx2::aom_mse_wxh_16bit_highbd_avx2;
#[cfg(target_arch = "x86_64")]
use aom::simd::aom_dsp__x86__highbd_variance_sse2::aom_mse_wxh_16bit_highbd_sse2;
#[cfg(target_arch = "x86_64")]
use aom::simd::aom_dsp__x86__variance_avx2::{aom_mse_16xh_16bit_avx2, aom_mse_wxh_16bit_avx2};
#[cfg(target_arch = "x86_64")]
use aom::simd::aom_dsp__x86__variance_sse2::{aom_mse_16xh_16bit_sse2, aom_mse_wxh_16bit_sse2};

extern "C-unwind" {
    fn aom_mse_wxh_16bit_c(dst: *mut u8, dstride: i32, src: *mut u16, sstride: i32, w: i32,
                           h: i32) -> u64;
    fn aom_mse_16xh_16bit_c(dst: *mut u8, dstride: i32, src: *mut u16, w: i32, h: i32) -> u64;
    fn aom_mse_wxh_16bit_highbd_c(dst: *mut u16, dstride: i32, src: *mut u16, sstride: i32,
                                  w: i32, h: i32) -> u64;
}

// av1/common/cdef_block.h
const CDEF_VERY_LARGE: u16 = 0x4000;
// (log2width, log2height) of the NEON parameter lists.
const PARAMS: [(i32, i32); 4] = [(3, 3), (3, 2), (2, 3), (2, 2)];

#[cfg(target_arch = "aarch64")]
#[test]
fn mse_wxh_16bit_ref_mse() {
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let block_size = (w * h) as usize;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u8; block_size];
        let mut src = vec![0u16; block_size];
        for _ in 0..10 {
            for k in 0..block_size {
                dst[k] = rnd.rand8();
                src[k] = rnd.rand8() as u16;
            }
            // SAFETY: w x h samples at stride w.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_wxh_16bit_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h),
                 aom_mse_wxh_16bit_neon(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

type Mse16xHFunc = unsafe extern "C-unwind" fn(*mut u8, i32, *mut u16, i32, i32) -> u64;

fn mse_16xh(extreme: bool, func: Mse16xHFunc) {
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let mem_size = 16 * 8;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u8; mem_size];
        let mut src = vec![0u16; mem_size];
        for _ in 0..10 {
            for k in 0..mem_size {
                if extreme {
                    dst[k] = if rnd.rand8() & 1 != 0 { 0 } else { 255 };
                    src[k] = if rnd.rand8() & 1 != 0 { 0 } else { CDEF_VERY_LARGE };
                } else {
                    dst[k] = rnd.rand8();
                    // Right shift by 6 is done to generate more input in range
                    // of [0,255] than CDEF_VERY_LARGE
                    let rnd_i10 = (rnd.rand16() >> 6) as i32;
                    src[k] = if rnd_i10 < 256 { rnd_i10 as u16 } else { CDEF_VERY_LARGE };
                }
            }
            // SAFETY: 16 / w blocks of w x h samples, as in the original.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_16xh_16bit_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, h),
                 func(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn mse_16xh_16bit_ref_mse() {
    mse_16xh(false, aom_mse_16xh_16bit_neon);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn mse_16xh_16bit_ref_mse_extreme() {
    mse_16xh(true, aom_mse_16xh_16bit_neon);
}

#[cfg(target_arch = "aarch64")]
#[test]
fn mse_wxh_16bit_highbd_ref_mse() {
    let mask: u16 = (1 << 10) - 1;
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let block_size = (w * h) as usize;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u16; block_size];
        let mut src = vec![0u16; block_size];
        for _ in 0..10 {
            for k in 0..block_size {
                dst[k] = rnd.rand16() & mask;
                src[k] = rnd.rand16() & mask;
            }
            // SAFETY: w x h samples at stride w.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_wxh_16bit_highbd_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h),
                 aom_mse_wxh_16bit_highbd_neon(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

// x86-64 instantiations of the same tests (upstream SSE2, SSE4_1 and AVX2).
#[cfg(target_arch = "x86_64")]
#[test]
fn mse_wxh_16bit_ref_mse_avx2() {
    if !std::is_x86_feature_detected!("avx2") {
        return;
    }
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let block_size = (w * h) as usize;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u8; block_size];
        let mut src = vec![0u16; block_size];
        for _ in 0..10 {
            for k in 0..block_size {
                dst[k] = rnd.rand8();
                src[k] = rnd.rand8() as u16;
            }
            // SAFETY: w x h samples at stride w.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_wxh_16bit_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h),
                 aom_mse_wxh_16bit_avx2(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_wxh_16bit_ref_mse_sse2() {
    if !std::is_x86_feature_detected!("sse2") {
        return;
    }
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let block_size = (w * h) as usize;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u8; block_size];
        let mut src = vec![0u16; block_size];
        for _ in 0..10 {
            for k in 0..block_size {
                dst[k] = rnd.rand8();
                src[k] = rnd.rand8() as u16;
            }
            // SAFETY: w x h samples at stride w.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_wxh_16bit_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h),
                 aom_mse_wxh_16bit_sse2(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_wxh_16bit_highbd_ref_mse_avx2() {
    if !std::is_x86_feature_detected!("avx2") {
        return;
    }
    let mask: u16 = (1 << 10) - 1;
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let block_size = (w * h) as usize;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u16; block_size];
        let mut src = vec![0u16; block_size];
        for _ in 0..10 {
            for k in 0..block_size {
                dst[k] = rnd.rand16() & mask;
                src[k] = rnd.rand16() & mask;
            }
            // SAFETY: w x h samples at stride w.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_wxh_16bit_highbd_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h),
                 aom_mse_wxh_16bit_highbd_avx2(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_wxh_16bit_highbd_ref_mse_sse2() {
    if !std::is_x86_feature_detected!("sse2") {
        return;
    }
    let mask: u16 = (1 << 10) - 1;
    for (lw, lh) in PARAMS {
        let (w, h) = (1 << lw, 1 << lh);
        let block_size = (w * h) as usize;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        let mut dst = vec![0u16; block_size];
        let mut src = vec![0u16; block_size];
        for _ in 0..10 {
            for k in 0..block_size {
                dst[k] = rnd.rand16() & mask;
                src[k] = rnd.rand16() & mask;
            }
            // SAFETY: w x h samples at stride w.
            let (mse_ref, mse_mod) = unsafe {
                (aom_mse_wxh_16bit_highbd_c(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h),
                 aom_mse_wxh_16bit_highbd_sse2(dst.as_mut_ptr(), w, src.as_mut_ptr(), w, w, h))
            };
            assert_eq!(mse_ref, mse_mod, "ref mse: {mse_ref} mod mse: {mse_mod}");
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_16xh_16bit_ref_mse_sse2() {
    if std::is_x86_feature_detected!("sse2") {
        mse_16xh(false, aom_mse_16xh_16bit_sse2);
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_16xh_16bit_ref_mse_extreme_sse2() {
    if std::is_x86_feature_detected!("sse2") {
        mse_16xh(true, aom_mse_16xh_16bit_sse2);
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_16xh_16bit_ref_mse_avx2() {
    if std::is_x86_feature_detected!("avx2") {
        mse_16xh(false, aom_mse_16xh_16bit_avx2);
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn mse_16xh_16bit_ref_mse_extreme_avx2() {
    if std::is_x86_feature_detected!("avx2") {
        mse_16xh(true, aom_mse_16xh_16bit_avx2);
    }
}
