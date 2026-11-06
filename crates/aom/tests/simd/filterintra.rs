// Supplementary transcription of upstream/aom/test/filterintra_test.cc
// (AV1FilterIntraPredTest.BitExactCheck, NEON instantiation). Modes, transform
// sizes, buffer layout, the per-iteration reseeded Rand8() fill, iteration
// count, and the exact-match check follow the original. DISABLED_Speed is not
// transcribed.
#![cfg(target_arch = "aarch64")]
use aom::simd::av1__common__arm__reconintra_neon::av1_filter_intra_predictor_neon;

use super::acm_random::ACMRandom;

type Predictor = unsafe extern "C-unwind" fn(*mut u8, isize, u8, *const u8, *const u8, i32);

extern "C-unwind" {
    fn av1_filter_intra_predictor_c(dst: *mut u8, stride: isize, tx_size: u8, above: *const u8, left: *const u8,
                                    mode: i32);
}

const MAX_TX_SIZE: usize = 32;
const MAX_TEST_NUM: usize = 100;

// FILTER_DC_PRED .. FILTER_PAETH_PRED
const FILTER_MODES: [i32; 5] = [0, 1, 2, 3, 4];

// TX_SIZE values from av1/common/enums.h.
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

const K_TX_SIZE_NEON: [u8; 14] = [
    TX_4X4, TX_8X8, TX_16X16, TX_32X32, TX_4X8, TX_8X4, TX_8X16, TX_16X8, TX_16X32, TX_32X16, TX_4X16, TX_16X4,
    TX_8X32, TX_32X8,
];

// tx_size_wide / tx_size_high from av1/common/common_data.h.
const TX_SIZE_WIDE: [usize; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const TX_SIZE_HIGH: [usize; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

fn prepare_buffer(alloc: &mut [u8]) {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    for v in alloc.iter_mut().take(2 * MAX_TX_SIZE + 1) {
        *v = rnd.rand8();
    }
}

/// AV1FilterIntraPredTest::RunTest for one (mode, tx size) parameter.
fn run_test(pred_func_ref: Predictor, pred_func: Predictor, mode: i32, tx_size: u8) -> usize {
    let mut alloc = vec![0u8; 2 * MAX_TX_SIZE + 1];
    let mut pred_ref = vec![0u8; MAX_TX_SIZE * MAX_TX_SIZE];
    let mut pred = vec![0u8; MAX_TX_SIZE * MAX_TX_SIZE];
    let w = TX_SIZE_WIDE[tx_size as usize];
    let h = TX_SIZE_HIGH[tx_size as usize];
    let stride = w as isize;
    for tst_index in 0..MAX_TEST_NUM {
        prepare_buffer(&mut alloc);
        let left = alloc.as_ptr();
        // SAFETY: above = alloc + MaxTxSize and &above[1] is passed, as in the
        // original; both predictors stay within the 2 * 32 + 1 byte edge buffer.
        unsafe {
            let above = left.add(MAX_TX_SIZE);
            pred_func_ref(pred_ref.as_mut_ptr(), stride, tx_size, above.add(1), left, mode);
            pred_func(pred.as_mut_ptr(), stride, tx_size, above.add(1), left, mode);
        }
        for i in 0..w * h {
            assert_eq!(pred_ref[i], pred[i], "Error at position: {} Tx size: {}x{} mode: {} Test number: {}", i, w,
                       h, mode, tst_index);
        }
    }
    MAX_TEST_NUM
}

#[test]
fn filter_intra_pred_neon_bitexact() {
    let mut blocks = 0;
    for mode in FILTER_MODES {
        for tx_size in K_TX_SIZE_NEON {
            blocks += run_test(av1_filter_intra_predictor_c, av1_filter_intra_predictor_neon, mode, tx_size);
        }
    }
    eprintln!("70 parameter sets, {} blocks compared", blocks);
}
