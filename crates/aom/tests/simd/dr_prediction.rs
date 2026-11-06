// Supplementary transcription of upstream/aom/test/dr_prediction_test.cc
// (HighbdDrPredTest.SaturatedValues and HighbdDrPredTest.OperationCheck, NEON
// instantiation). Angles, dx/dy derivation, edge-upsample selection, buffer
// layout and padding, the Rand8() fill order, bit depths, and the exact-match
// check follow the original. DISABLED_Speed is not transcribed. The lowbd
// NEON instantiation (av1_dr_prediction_z{1,2,3}_neon) belongs to
// aom_dsp/arm/intrapred_neon.c and is not part of this port.
#![cfg(target_arch = "aarch64")]
use aom::simd::aom_dsp__arm__highbd_intrapred_neon::{
    av1_highbd_dr_prediction_z1_neon, av1_highbd_dr_prediction_z2_neon, av1_highbd_dr_prediction_z3_neon,
};

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_highbd_dr_prediction_z1_c(dst: *mut u16, stride: isize, bw: i32, bh: i32, above: *const u16,
                                     left: *const u16, upsample_above: i32, dx: i32, dy: i32, bd: i32);
    fn av1_highbd_dr_prediction_z2_c(dst: *mut u16, stride: isize, bw: i32, bh: i32, above: *const u16,
                                     left: *const u16, upsample_above: i32, upsample_left: i32, dx: i32,
                                     dy: i32, bd: i32);
    fn av1_highbd_dr_prediction_z3_c(dst: *mut u16, stride: isize, bw: i32, bh: i32, above: *const u16,
                                     left: *const u16, upsample_left: i32, dx: i32, dy: i32, bd: i32);
}

type Z1Hbd = unsafe extern "C-unwind" fn(*mut u16, isize, i32, i32, *const u16, *const u16, i32, i32, i32, i32);
type Z2Hbd =
    unsafe extern "C-unwind" fn(*mut u16, isize, i32, i32, *const u16, *const u16, i32, i32, i32, i32, i32);
type Z3Hbd = unsafe extern "C-unwind" fn(*mut u16, isize, i32, i32, *const u16, *const u16, i32, i32, i32, i32);

/// The z{1,2,3}_wrapper_hbd adapters of the original.
#[derive(Clone, Copy)]
enum DrPredHbd {
    Z1(Z1Hbd),
    Z2(Z2Hbd),
    Z3(Z3Hbd),
}

impl DrPredHbd {
    #[allow(clippy::too_many_arguments)]
    unsafe fn call(self, dst: *mut u16, stride: isize, bw: i32, bh: i32, above: *const u16, left: *const u16,
                   upsample_above: i32, upsample_left: i32, dx: i32, dy: i32, bd: i32) {
        match self {
            DrPredHbd::Z1(f) => f(dst, stride, bw, bh, above, left, upsample_above, dx, dy, bd),
            DrPredHbd::Z2(f) => f(dst, stride, bw, bh, above, left, upsample_above, upsample_left, dx, dy, bd),
            DrPredHbd::Z3(f) => f(dst, stride, bw, bh, above, left, upsample_left, dx, dy, bd),
        }
    }
}

const MAX_TX_SIZE: usize = 64;
const K_NUM_INTRA_NEIGHBOUR_PIXELS: usize = MAX_TX_SIZE * 2 + 32;
const K_INTRA_PRED_INPUT_PADDING: usize = 16;
const K_Z1_START: i32 = 0;
const K_Z2_START: i32 = 90;
const K_Z3_START: i32 = 180;
const TX_SIZES_ALL: usize = 19;

// kTxSize lists TX_4X4..TX_64X16 in enum order; tx_size_wide / tx_size_high
// from av1/common/common_data.h.
const TX_SIZE_WIDE: [i32; TX_SIZES_ALL] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const TX_SIZE_HIGH: [i32; TX_SIZES_ALL] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

// dr_intra_derivative from av1/common/reconintra.h.
#[rustfmt::skip]
const DR_INTRA_DERIVATIVE: [i32; 90] = [
    0,    0, 0,
    1023, 0, 0,
    547,  0, 0,
    372,  0, 0, 0, 0,
    273,  0, 0,
    215,  0, 0,
    178,  0, 0,
    151,  0, 0,
    132,  0, 0,
    116,  0, 0,
    102,  0, 0, 0,
    90,   0, 0,
    80,   0, 0,
    71,   0, 0,
    64,   0, 0,
    57,   0, 0,
    51,   0, 0,
    45,   0, 0, 0,
    40,   0, 0,
    35,   0, 0,
    31,   0, 0,
    27,   0, 0,
    23,   0, 0,
    19,   0, 0,
    15,   0, 0, 0, 0,
    11,   0, 0,
    7,    0, 0,
    3,    0, 0,
];

// av1_get_dx / av1_get_dy / av1_use_intra_edge_upsample from
// av1/common/reconintra.h.
fn av1_get_dx(angle: i32) -> i32 {
    if angle > 0 && angle < 90 {
        DR_INTRA_DERIVATIVE[angle as usize]
    } else if angle > 90 && angle < 180 {
        DR_INTRA_DERIVATIVE[(180 - angle) as usize]
    } else {
        1
    }
}

fn av1_get_dy(angle: i32) -> i32 {
    if angle > 90 && angle < 180 {
        DR_INTRA_DERIVATIVE[(angle - 90) as usize]
    } else if angle > 180 && angle < 270 {
        DR_INTRA_DERIVATIVE[(270 - angle) as usize]
    } else {
        1
    }
}

fn av1_use_intra_edge_upsample(bs0: i32, bs1: i32, delta: i32, type_: i32) -> i32 {
    let d = delta.abs();
    let blk_wh = bs0 + bs1;
    if d == 0 || d >= 40 {
        return 0;
    }
    if type_ != 0 { (blk_wh <= 8) as i32 } else { (blk_wh <= 16) as i32 }
}

struct DrPredTest {
    ref_fn: DrPredHbd,
    tst_fn: DrPredHbd,
    bit_depth: i32,
    start_angle: i32,
    stop_angle: i32,
    enable_upsample: i32,
    upsample_above: i32,
    upsample_left: i32,
    bw: i32,
    bh: i32,
    dx: i32,
    dy: i32,
    rng: ACMRandom,
    blocks: usize,
}

impl DrPredTest {
    fn new(ref_fn: DrPredHbd, tst_fn: DrPredHbd, bit_depth: i32, start_angle: i32) -> Self {
        Self {
            ref_fn,
            tst_fn,
            bit_depth,
            start_angle,
            stop_angle: start_angle + 90,
            enable_upsample: 0,
            upsample_above: 0,
            upsample_left: 0,
            bw: 0,
            bh: 0,
            dx: 1,
            dy: 1,
            rng: ACMRandom::new(),
            blocks: 0,
        }
    }

    fn run_test(&mut self, needsaturation: bool, p_angle: i32) {
        let bd = self.bit_depth;
        for tx in 0..TX_SIZES_ALL {
            self.bw = TX_SIZE_WIDE[tx];
            self.bh = TX_SIZE_HIGH[tx];
            if self.enable_upsample != 0 {
                self.upsample_above = av1_use_intra_edge_upsample(self.bw, self.bh, p_angle - 90, 0);
                self.upsample_left = av1_use_intra_edge_upsample(self.bw, self.bh, p_angle - 180, 0);
            } else {
                self.upsample_above = 0;
                self.upsample_left = 0;
            }

            let mut left_data = [0u16; K_NUM_INTRA_NEIGHBOUR_PIXELS];
            let mut above_data = [0u16; K_NUM_INTRA_NEIGHBOUR_PIXELS];
            if needsaturation {
                let sat = ((1 << bd) - 1) as u16;
                for i in 0..K_NUM_INTRA_NEIGHBOUR_PIXELS {
                    left_data[i] = sat;
                    above_data[i] = sat;
                }
            } else {
                for i in 0..K_NUM_INTRA_NEIGHBOUR_PIXELS {
                    left_data[i] = self.rng.rand8() as u16;
                    above_data[i] = self.rng.rand8() as u16;
                }
            }
            // We need to allow reading some previous bytes from the input pointers.
            let above = above_data[K_INTRA_PRED_INPUT_PADDING..].as_ptr();
            let left = left_data[K_INTRA_PRED_INPUT_PADDING..].as_ptr();

            // Add additional padding to allow detection of over reads/writes when
            // the transform width is equal to MAX_TX_SIZE.
            let dst_stride = MAX_TX_SIZE + 16;
            let mut dst_ref = vec![0u16; dst_stride * self.bh as usize];
            let mut dst_tst = vec![0u16; dst_stride * self.bh as usize];
            // SAFETY: the edge buffers and destinations have the extents of the
            // original test.
            unsafe {
                self.ref_fn.call(dst_ref.as_mut_ptr(), dst_stride as isize, self.bw, self.bh, above, left,
                                 self.upsample_above, self.upsample_left, self.dx, self.dy, bd);
                self.tst_fn.call(dst_tst.as_mut_ptr(), dst_stride as isize, self.bw, self.bh, above, left,
                                 self.upsample_above, self.upsample_left, self.dx, self.dy, bd);
            }
            for r in 0..self.bh as usize {
                for c in 0..self.bw as usize {
                    assert_eq!(dst_ref[r * dst_stride + c], dst_tst[r * dst_stride + c],
                               "{}x{} r: {} c: {} dx: {} dy: {} upsample_above: {} upsample_left: {} bd: {}",
                               self.bw, self.bh, r, c, self.dx, self.dy, self.upsample_above,
                               self.upsample_left, bd);
                }
            }
            self.blocks += 1;
        }
    }

    /// TEST_P(HighbdDrPredTest, SaturatedValues) and OperationCheck.
    fn run(&mut self, needsaturation: bool) {
        self.enable_upsample = 0;
        while self.enable_upsample < 2 {
            for angle in self.start_angle..self.stop_angle {
                self.dx = av1_get_dx(angle);
                self.dy = av1_get_dy(angle);
                if self.dx != 0 && self.dy != 0 {
                    self.run_test(needsaturation, angle);
                }
            }
            self.enable_upsample += 1;
        }
    }
}

fn neon_params() -> Vec<(DrPredHbd, DrPredHbd, i32, i32)> {
    let mut v = Vec::new();
    for bd in [8, 10, 12] {
        v.push((DrPredHbd::Z1(av1_highbd_dr_prediction_z1_c), DrPredHbd::Z1(av1_highbd_dr_prediction_z1_neon), bd,
                K_Z1_START));
    }
    for bd in [8, 10, 12] {
        v.push((DrPredHbd::Z2(av1_highbd_dr_prediction_z2_c), DrPredHbd::Z2(av1_highbd_dr_prediction_z2_neon), bd,
                K_Z2_START));
    }
    for bd in [8, 10, 12] {
        v.push((DrPredHbd::Z3(av1_highbd_dr_prediction_z3_c), DrPredHbd::Z3(av1_highbd_dr_prediction_z3_neon), bd,
                K_Z3_START));
    }
    v
}

fn run_all(needsaturation: bool) {
    let mut blocks = 0;
    for (r, t, bd, start) in neon_params() {
        let mut test = DrPredTest::new(r, t, bd, start);
        test.run(needsaturation);
        blocks += test.blocks;
    }
    eprintln!("9 parameter sets, {} blocks compared", blocks);
}

#[test]
fn highbd_dr_pred_neon_saturated_values() {
    run_all(true);
}

#[test]
fn highbd_dr_pred_neon_operation_check() {
    run_all(false);
}
