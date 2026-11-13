// Supplementary transcription of upstream/aom/test/cfl_test.cc for its NEON
// instantiations: CFLSubAvgTest.SubAvgTest, CFLSubsampleLBDTest
// (SubsampleLBD420/422/444Test), CFLSubsampleHBDTest
// (SubsampleHBD420/422/444Test), CFLPredictTest.PredictTest and
// CFLPredictHBDTest.PredictHBDTest. Each parameter resets the generator to
// the deterministic seed (CFLTest::init), draws in the original order, runs
// NUM_ITERATIONS (100) iterations, and compares the getter results of
// `cfl_get_*_neon` against those of `cfl_get_*_c` exactly over the original
// width x height region at stride CFL_BUF_LINE. The DISABLED_*Speed tests are
// not transcribed.
#![cfg(target_arch = "aarch64")]
use aom::av1__common__cfl::{
    cfl_get_luma_subsampling_420_hbd_c, cfl_get_luma_subsampling_420_lbd_c,
    cfl_get_luma_subsampling_422_hbd_c, cfl_get_luma_subsampling_422_lbd_c,
    cfl_get_luma_subsampling_444_hbd_c, cfl_get_luma_subsampling_444_lbd_c,
    cfl_get_predict_hbd_fn_c, cfl_get_predict_lbd_fn_c, cfl_get_subtract_average_fn_c,
    cfl_predict_hbd_fn, cfl_predict_lbd_fn, cfl_subsample_hbd_fn, cfl_subsample_lbd_fn,
    cfl_subtract_average_fn,
};
use aom::simd::av1__common__arm__cfl_neon::*;

use super::acm_random::ACMRandom;

const NUM_ITERATIONS: usize = 100;
const CFL_BUF_LINE: usize = 32;
const CFL_BUF_SQUARE: usize = CFL_BUF_LINE * CFL_BUF_LINE;

// TX sizes of ALL_CFL_TX_SIZES, in the original order.
const TX_4X4: u8 = 0;
const TX_8X8: u8 = 1;
const TX_16X16: u8 = 2;
const TX_32X32: u8 = 3;
const TX_4X8: u8 = 5;
const TX_8X4: u8 = 6;
const TX_8X16: u8 = 7;
const TX_16X8: u8 = 8;
const TX_16X32: u8 = 9;
const TX_32X16: u8 = 10;
const TX_4X16: u8 = 13;
const TX_16X4: u8 = 14;
const TX_8X32: u8 = 15;
const TX_32X8: u8 = 16;
const ALL_CFL_TX_SIZES: [u8; 14] = [
    TX_4X4, TX_4X8, TX_4X16, TX_8X4, TX_8X8, TX_8X16, TX_8X32, TX_16X4, TX_16X8, TX_16X16,
    TX_16X32, TX_32X8, TX_32X16, TX_32X32,
];

// av1/common/common_data.h
const TX_SIZE_WIDE: [usize; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const TX_SIZE_HIGH: [usize; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

#[repr(C, align(32))]
struct Aligned<T>(T);

fn assert_eq_region<A: PartialEq + core::fmt::Debug>(a: &[A], b: &[A], width: usize, height: usize,
                                                      what: &str, tx: u8, it: usize) {
    for j in 0..height {
        for i in 0..width {
            assert_eq!(a[j * CFL_BUF_LINE + i], b[j * CFL_BUF_LINE + i],
                       "{what} tx_size {tx} iteration {it} at ({i},{j})");
        }
    }
}

/// TEST_P(CFLSubAvgTest, SubAvgTest)
#[test]
fn cfl_sub_avg_neon() {
    for tx in ALL_CFL_TX_SIZES {
        let (width, height) = (TX_SIZE_WIDE[tx as usize], TX_SIZE_HIGH[tx as usize]);
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        // SAFETY: plain table lookups.
        let (sub_avg, sub_avg_ref): (cfl_subtract_average_fn, cfl_subtract_average_fn) =
            unsafe { (cfl_get_subtract_average_fn_neon(tx), cfl_get_subtract_average_fn_c(tx)) };
        let (sub_avg, sub_avg_ref) = (sub_avg.unwrap(), sub_avg_ref.unwrap());
        let mut data = Box::new([0u16; CFL_BUF_SQUARE]);
        let mut data_ref = Box::new([0u16; CFL_BUF_SQUARE]);
        let mut dst = Box::new([0i16; CFL_BUF_SQUARE]);
        let mut dst_ref = Box::new([0i16; CFL_BUF_SQUARE]);
        for it in 0..NUM_ITERATIONS {
            for j in 0..height {
                for i in 0..width {
                    let d = rnd.rand15();
                    data[j * CFL_BUF_LINE + i] = d;
                    data_ref[j * CFL_BUF_LINE + i] = d;
                }
            }
            // SAFETY: CFL_BUF_SQUARE buffers at stride CFL_BUF_LINE.
            unsafe {
                sub_avg(data.as_ptr(), dst.as_mut_ptr());
                sub_avg_ref(data_ref.as_ptr(), dst_ref.as_mut_ptr());
            }
            assert_eq_region(&dst[..], &dst_ref[..], width, height, "SubAvg", tx, it);
        }
    }
}

/// CFLSubsampleTest::subsampleTest for one function pair.
fn subsample_test<I: Copy + Default>(fun: unsafe extern "C-unwind" fn(*const I, i32, *mut u16),
                                     fun_ref: unsafe extern "C-unwind" fn(*const I, i32, *mut u16),
                                     tx: u8, sub_width: usize, sub_height: usize,
                                     random: fn(&mut ACMRandom) -> I, what: &str) {
    let (width, height) = (TX_SIZE_WIDE[tx as usize], TX_SIZE_HIGH[tx as usize]);
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut data = vec![I::default(); CFL_BUF_SQUARE];
    let mut data_ref = vec![I::default(); CFL_BUF_SQUARE];
    let mut sub_luma_pels = Box::new([0u16; CFL_BUF_SQUARE]);
    let mut sub_luma_pels_ref = Box::new([0u16; CFL_BUF_SQUARE]);
    for it in 0..NUM_ITERATIONS {
        for j in 0..height {
            for i in 0..width {
                let d = random(&mut rnd);
                data[j * CFL_BUF_LINE + i] = d;
                data_ref[j * CFL_BUF_LINE + i] = d;
            }
        }
        // SAFETY: CFL_BUF_SQUARE buffers at stride CFL_BUF_LINE.
        unsafe {
            fun(data.as_ptr(), CFL_BUF_LINE as i32, sub_luma_pels.as_mut_ptr());
            fun_ref(data_ref.as_ptr(), CFL_BUF_LINE as i32, sub_luma_pels_ref.as_mut_ptr());
        }
        assert_eq_region(&sub_luma_pels[..], &sub_luma_pels_ref[..], sub_width, sub_height, what,
                         tx, it);
    }
}

type GetLbd = unsafe extern "C-unwind" fn(u8) -> cfl_subsample_lbd_fn;
type GetHbd = unsafe extern "C-unwind" fn(u8) -> cfl_subsample_hbd_fn;

/// CFLSubsampleLBDTest.SubsampleLBD{420,422,444}Test
#[test]
fn cfl_subsample_lbd_neon() {
    let cases: [(GetLbd, GetLbd, &str, usize, usize); 3] = [
        (cfl_get_luma_subsampling_420_lbd_neon, cfl_get_luma_subsampling_420_lbd_c, "LBD420", 1, 1),
        (cfl_get_luma_subsampling_422_lbd_neon, cfl_get_luma_subsampling_422_lbd_c, "LBD422", 1, 0),
        (cfl_get_luma_subsampling_444_lbd_neon, cfl_get_luma_subsampling_444_lbd_c, "LBD444", 0, 0),
    ];
    for tx in ALL_CFL_TX_SIZES {
        let (width, height) = (TX_SIZE_WIDE[tx as usize], TX_SIZE_HIGH[tx as usize]);
        for (get, get_ref, what, sw, sh) in cases {
            // SAFETY: plain table lookups.
            let (fun, fun_ref) = unsafe { (get(tx).unwrap(), get_ref(tx).unwrap()) };
            subsample_test::<u8>(fun, fun_ref, tx, width >> sw, height >> sh, ACMRandom::rand8,
                                 what);
        }
    }
}

/// CFLSubsampleHBDTest.SubsampleHBD{420,422,444}Test
#[test]
fn cfl_subsample_hbd_neon() {
    let cases: [(GetHbd, GetHbd, &str, usize, usize); 3] = [
        (cfl_get_luma_subsampling_420_hbd_neon, cfl_get_luma_subsampling_420_hbd_c, "HBD420", 1, 1),
        (cfl_get_luma_subsampling_422_hbd_neon, cfl_get_luma_subsampling_422_hbd_c, "HBD422", 1, 0),
        (cfl_get_luma_subsampling_444_hbd_neon, cfl_get_luma_subsampling_444_hbd_c, "HBD444", 0, 0),
    ];
    for tx in ALL_CFL_TX_SIZES {
        let (width, height) = (TX_SIZE_WIDE[tx as usize], TX_SIZE_HIGH[tx as usize]);
        for (get, get_ref, what, sw, sh) in cases {
            // SAFETY: plain table lookups.
            let (fun, fun_ref) = unsafe { (get(tx).unwrap(), get_ref(tx).unwrap()) };
            subsample_test::<u16>(fun, fun_ref, tx, width >> sw, height >> sh, ACMRandom::rand12,
                                  what);
        }
    }
}

/// CFLTestWithAlignedData<I>::randData: returns alpha_q3.
fn rand_data<I: Copy>(rnd: &mut ACMRandom, bd: i32, width: usize, height: usize,
                      chroma: &mut [I], chroma_ref: &mut [I], luma: &mut [i16],
                      luma_ref: &mut [i16], to_i: fn(i32) -> I) -> i32 {
    let alpha_q3 = rnd.pseudo_uniform(33) - 16;
    let dc = to_i(rnd.pseudo_uniform(1 << bd));
    for j in 0..height {
        for i in 0..width {
            chroma[j * CFL_BUF_LINE + i] = dc;
            chroma_ref[j * CFL_BUF_LINE + i] = dc;
            let v = rnd.pseudo_uniform(1 << (bd + 3)) as i16;
            luma_ref[j * CFL_BUF_LINE + i] = v;
            luma[j * CFL_BUF_LINE + i] = v;
        }
    }
    alpha_q3
}

/// TEST_P(CFLPredictTest, PredictTest)
#[test]
fn cfl_predict_lbd_neon() {
    for tx in ALL_CFL_TX_SIZES {
        let (width, height) = (TX_SIZE_WIDE[tx as usize], TX_SIZE_HIGH[tx as usize]);
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        // SAFETY: plain table lookups.
        let (predict, predict_ref): (cfl_predict_lbd_fn, cfl_predict_lbd_fn) =
            unsafe { (cfl_get_predict_lbd_fn_neon(tx), cfl_get_predict_lbd_fn_c(tx)) };
        let (predict, predict_ref) = (predict.unwrap(), predict_ref.unwrap());
        let mut chroma = Box::new(Aligned([0u8; CFL_BUF_SQUARE]));
        let mut chroma_ref = Box::new(Aligned([0u8; CFL_BUF_SQUARE]));
        let mut luma = Box::new(Aligned([0i16; CFL_BUF_SQUARE]));
        let mut luma_ref = Box::new(Aligned([0i16; CFL_BUF_SQUARE]));
        for it in 0..NUM_ITERATIONS {
            let alpha_q3 = rand_data(&mut rnd, 8, width, height, &mut chroma.0, &mut chroma_ref.0,
                                     &mut luma.0, &mut luma_ref.0, |v| v as u8);
            // SAFETY: CFL_BUF_SQUARE buffers at stride CFL_BUF_LINE.
            unsafe {
                predict(luma.0.as_ptr(), chroma.0.as_mut_ptr(), CFL_BUF_LINE as i32, alpha_q3);
                predict_ref(luma_ref.0.as_ptr(), chroma_ref.0.as_mut_ptr(), CFL_BUF_LINE as i32,
                            alpha_q3);
            }
            assert_eq_region(&chroma.0[..], &chroma_ref.0[..], width, height, "PredictLBD", tx, it);
        }
    }
}

/// TEST_P(CFLPredictHBDTest, PredictHBDTest)
#[test]
fn cfl_predict_hbd_neon() {
    let bd = 12;
    for tx in ALL_CFL_TX_SIZES {
        let (width, height) = (TX_SIZE_WIDE[tx as usize], TX_SIZE_HIGH[tx as usize]);
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        // SAFETY: plain table lookups.
        let (predict, predict_ref): (cfl_predict_hbd_fn, cfl_predict_hbd_fn) =
            unsafe { (cfl_get_predict_hbd_fn_neon(tx), cfl_get_predict_hbd_fn_c(tx)) };
        let (predict, predict_ref) = (predict.unwrap(), predict_ref.unwrap());
        let mut chroma = Box::new(Aligned([0u16; CFL_BUF_SQUARE]));
        let mut chroma_ref = Box::new(Aligned([0u16; CFL_BUF_SQUARE]));
        let mut luma = Box::new(Aligned([0i16; CFL_BUF_SQUARE]));
        let mut luma_ref = Box::new(Aligned([0i16; CFL_BUF_SQUARE]));
        for it in 0..NUM_ITERATIONS {
            let alpha_q3 = rand_data(&mut rnd, bd, width, height, &mut chroma.0,
                                     &mut chroma_ref.0, &mut luma.0, &mut luma_ref.0,
                                     |v| v as u16);
            // SAFETY: CFL_BUF_SQUARE buffers at stride CFL_BUF_LINE.
            unsafe {
                predict(luma.0.as_ptr(), chroma.0.as_mut_ptr(), CFL_BUF_LINE as i32, alpha_q3, bd);
                predict_ref(luma_ref.0.as_ptr(), chroma_ref.0.as_mut_ptr(), CFL_BUF_LINE as i32,
                            alpha_q3, bd);
            }
            assert_eq_region(&chroma.0[..], &chroma_ref.0[..], width, height, "PredictHBD", tx, it);
        }
    }
}
