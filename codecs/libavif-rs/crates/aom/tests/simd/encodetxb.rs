// Supplementary transcription of upstream/aom/test/encodetxb_test.cc for the
// NEON instantiations: EncodeTxbTest.GetNzMapContexts
// (av1_get_nz_map_contexts_neon) and EncodeTxbInitLevelTest.match
// (av1_txb_init_levels_neon over every TX_SIZE). Seeds, loops, data
// generation and the checks follow the original. Note the original's
// GetNzMapContexts contract: Compare() reports a mismatch only at the scan
// positions below eob, so positions the C function leaves untouched may
// differ; this transcription keeps that check unchanged.
#![cfg(target_arch = "aarch64")]

use aom::av1__common__scan::av1_scan_orders;
use aom::simd::av1__encoder__arm__encodetxb_neon::*;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_get_nz_map_contexts_c(levels: *const u8, scan: *const i16, eob: i32, tx_size: u8,
                                 tx_class: u8, coeff_contexts: *mut i8);
    fn av1_txb_init_levels_c(coeff: *const i32, width: i32, height: i32, levels: *mut u8);
}

type GetNzMapContextsFunc = unsafe extern "C-unwind" fn(*const u8, *const i16, i32, u8, u8, *mut i8);
type TxbInitLevelsFunc = unsafe extern "C-unwind" fn(*const i32, i32, i32, *mut u8);

const TX_SIZES_ALL: usize = 19;
const TX_TYPES: usize = 16;
const MAX_TX_SQUARE: usize = 64 * 64;
const TX_PAD_HOR_LOG2: i32 = 2;
const TX_PAD_HOR: i32 = 4;
const TX_PAD_TOP: i32 = 0;
const TX_PAD_VER: i32 = 4;
const TX_PAD_2D: usize = ((32 + TX_PAD_HOR) * (32 + TX_PAD_VER) + 16) as usize;

// av1/common/txb_common.h tx_type_to_class
const tx_type_to_class: [u8; TX_TYPES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2, 1, 2, 1];
const tx_size_wide: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const tx_size_high: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
const tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

fn adjusted(tx_size: usize) -> usize {
    match tx_size {
        4 | 11 | 12 => 3,
        18 => 10,
        17 => 9,
        _ => tx_size,
    }
}
fn get_txb_bhl(tx_size: usize) -> i32 {
    tx_size_high_log2[adjusted(tx_size)]
}
fn get_txb_wide(tx_size: usize) -> i32 {
    tx_size_wide[adjusted(tx_size)]
}
fn get_txb_high(tx_size: usize) -> i32 {
    tx_size_high[adjusted(tx_size)]
}
fn set_levels_offset(height: i32) -> usize {
    (TX_PAD_TOP * (height + TX_PAD_HOR)) as usize
}
fn get_padded_idx(idx: i32, bhl: i32) -> i32 {
    idx + ((idx >> bhl) << TX_PAD_HOR_LOG2)
}

#[repr(C, align(16))]
struct Aligned16<T>(T);

struct EncodeTxbTest {
    func: GetNzMapContextsFunc,
    rnd: ACMRandom,
    levels_buf: Box<[u8; TX_PAD_2D]>,
    levels: usize,
    coeff_contexts_ref: Box<Aligned16<[i8; MAX_TX_SQUARE]>>,
    coeff_contexts: Box<Aligned16<[i8; MAX_TX_SQUARE]>>,
}

impl EncodeTxbTest {
    fn init_data_with_eob(&mut self, scan: *const i16, bhl: i32, eob: i32) {
        self.levels_buf.fill(0);
        self.coeff_contexts.0.fill(0);
        for c in 0..eob as usize {
            // SAFETY: scan covers the eob coefficients of the transform.
            let pos = unsafe { *scan.add(c) } as i32;
            self.levels_buf[self.levels + get_padded_idx(pos, bhl) as usize] =
                (self.rnd.rand8() as i32).clamp(0, i8::MAX as i32) as u8;
            self.coeff_contexts.0[pos as usize] = (self.rnd.rand16() >> 1) as i8;
        }
        self.coeff_contexts_ref.0 = self.coeff_contexts.0;
    }

    fn compare(&self, scan: *const i16, eob: i32) -> bool {
        if self.coeff_contexts.0 != self.coeff_contexts_ref.0 {
            for i in 0..eob as usize {
                // SAFETY: as in init_data_with_eob.
                let pos = unsafe { *scan.add(i) } as usize;
                if self.coeff_contexts_ref.0[pos] != self.coeff_contexts.0[pos] {
                    println!("coeff_contexts_[{pos}] diff:{:6} (ref),{:6} (opt)",
                             self.coeff_contexts_ref.0[pos], self.coeff_contexts.0[pos]);
                    return true;
                }
            }
        }
        false
    }

    fn get_nz_map_contexts_run(&mut self) {
        let k_num_tests = 10;
        let mut result = false;
        for _is_inter in 0..2 {
            for tx_type in 0..TX_TYPES {
                let tx_class = tx_type_to_class[tx_type];
                for tx_size in 0..TX_SIZES_ALL {
                    let bhl = get_txb_bhl(tx_size);
                    let width = get_txb_wide(tx_size);
                    let height = get_txb_high(tx_size);
                    // SAFETY: av1_scan_orders is immutable after initialization.
                    let scan = unsafe { av1_scan_orders[tx_size][tx_type].scan };
                    self.levels = set_levels_offset(height);
                    let mut i = 0;
                    while i < k_num_tests && !result {
                        let mut eob = 1;
                        while eob <= width * height && !result {
                            self.init_data_with_eob(scan, bhl, eob);
                            // SAFETY: the levels buffer is TX_PAD_2D bytes with the
                            // original's padding; the contexts hold MAX_TX_SQUARE.
                            unsafe {
                                av1_get_nz_map_contexts_c(self.levels_buf.as_ptr().add(self.levels),
                                                          scan, eob, tx_size as u8, tx_class,
                                                          self.coeff_contexts_ref.0.as_mut_ptr());
                                (self.func)(self.levels_buf.as_ptr().add(self.levels), scan, eob,
                                            tx_size as u8, tx_class,
                                            self.coeff_contexts.0.as_mut_ptr());
                            }
                            result = self.compare(scan, eob);
                            assert!(!result, " tx_class {tx_class} width {} height {} eob {eob}",
                                    tx_size_wide[tx_size], tx_size_high[tx_size]);
                            eob += 1;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
}

#[test]
fn get_nz_map_contexts() {
    let mut t = EncodeTxbTest {
        func: av1_get_nz_map_contexts_neon,
        rnd: ACMRandom::new(),
        levels_buf: Box::new([0; TX_PAD_2D]),
        levels: 0,
        coeff_contexts_ref: Box::new(Aligned16([0; MAX_TX_SQUARE])),
        coeff_contexts: Box::new(Aligned16([0; MAX_TX_SQUARE])),
    };
    t.get_nz_map_contexts_run();
}

/// EncodeTxbInitLevelTest::RunTest without the speed branch.
fn txb_init_level_run_test(test_func: TxbInitLevelsFunc, tx_size: usize) {
    let width = get_txb_wide(tx_size);
    let height = get_txb_high(tx_size);
    let mut coeff = vec![0i32; MAX_TX_SQUARE];
    let mut levels_buf = [[0u8; TX_PAD_2D]; 2];
    let off = set_levels_offset(height);

    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    for i in 0..(width * height) as usize {
        coeff[i] = rnd.rand16_signed() as i32;
    }
    for i in 0..TX_PAD_2D {
        levels_buf[0][i] = rnd.rand8();
        levels_buf[1][i] = rnd.rand8();
    }
    // SAFETY: each levels buffer is TX_PAD_2D bytes, as in the original.
    unsafe {
        av1_txb_init_levels_c(coeff.as_ptr(), width, height, levels_buf[0].as_mut_ptr().add(off));
        test_func(coeff.as_ptr(), width, height, levels_buf[1].as_mut_ptr().add(off));
    }
    let stride = width + TX_PAD_HOR;
    for r in 0..height + TX_PAD_VER {
        for c in 0..stride {
            let k = (c + r * stride) as usize;
            assert_eq!(levels_buf[0][k], levels_buf[1][k], "[{r},{c}] 1{width}x{height}");
        }
    }
}

#[test]
fn txb_init_levels_match() {
    // Range(0, TX_SIZES_ALL)
    for tx_size in 0..TX_SIZES_ALL {
        txb_init_level_run_test(av1_txb_init_levels_neon, tx_size);
    }
}
