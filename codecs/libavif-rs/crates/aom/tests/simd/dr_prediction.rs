// Supplementary transcription of upstream/aom/test/dr_prediction_test.cc
// (HighbdDrPredTest.SaturatedValues and HighbdDrPredTest.OperationCheck, NEON
// and AVX2 instantiations). Angles, dx/dy derivation, edge-upsample selection,
// buffer layout and padding, the Rand8() fill order, bit depths, and the
// exact-match check follow the original. DISABLED_Speed is not transcribed.
// LowbdDrPredTest.SaturatedValues and LowbdDrPredTest.OperationCheck are
// transcribed for the NEON instantiation (av1_dr_prediction_z{1,2,3}_neon of
// aom_dsp/arm/intrapred_neon.c, AOM_BITS_8). The lowbd SSE4_1 and AVX2
// instantiations belong to x86 files that are not part of this port.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
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

/// The nine DrPredFunc<DrPred_Hbd> values of an instantiation: Z1, Z2, Z3,
/// each at AOM_BITS_8, AOM_BITS_10 and AOM_BITS_12.
fn params(z1: Z1Hbd, z2: Z2Hbd, z3: Z3Hbd) -> Vec<(DrPredHbd, DrPredHbd, i32, i32)> {
    let mut v = Vec::new();
    for bd in [8, 10, 12] {
        v.push((DrPredHbd::Z1(av1_highbd_dr_prediction_z1_c), DrPredHbd::Z1(z1), bd, K_Z1_START));
    }
    for bd in [8, 10, 12] {
        v.push((DrPredHbd::Z2(av1_highbd_dr_prediction_z2_c), DrPredHbd::Z2(z2), bd, K_Z2_START));
    }
    for bd in [8, 10, 12] {
        v.push((DrPredHbd::Z3(av1_highbd_dr_prediction_z3_c), DrPredHbd::Z3(z3), bd, K_Z3_START));
    }
    v
}

fn run_all(params: Vec<(DrPredHbd, DrPredHbd, i32, i32)>, needsaturation: bool) {
    let mut blocks = 0;
    for (r, t, bd, start) in params {
        let mut test = DrPredTest::new(r, t, bd, start);
        test.run(needsaturation);
        blocks += test.blocks;
    }
    eprintln!("9 parameter sets, {} blocks compared", blocks);
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::aom_dsp__arm__highbd_intrapred_neon::{
        av1_highbd_dr_prediction_z1_neon, av1_highbd_dr_prediction_z2_neon,
        av1_highbd_dr_prediction_z3_neon,
    };

    fn neon_params() -> Vec<(DrPredHbd, DrPredHbd, i32, i32)> {
        params(av1_highbd_dr_prediction_z1_neon, av1_highbd_dr_prediction_z2_neon,
               av1_highbd_dr_prediction_z3_neon)
    }

    #[test]
    fn highbd_dr_pred_neon_saturated_values() {
        run_all(neon_params(), true);
    }

    #[test]
    fn highbd_dr_pred_neon_operation_check() {
        run_all(neon_params(), false);
    }
}

#[cfg(target_arch = "aarch64")]
mod lowbd_neon {
    use super::*;
    use aom::simd::aom_dsp__arm__intrapred_neon::{
        av1_dr_prediction_z1_neon, av1_dr_prediction_z2_neon, av1_dr_prediction_z3_neon,
    };

    extern "C-unwind" {
        fn av1_dr_prediction_z1_c(dst: *mut u8, stride: isize, bw: i32, bh: i32, above: *const u8,
                                  left: *const u8, upsample_above: i32, dx: i32, dy: i32);
        fn av1_dr_prediction_z2_c(dst: *mut u8, stride: isize, bw: i32, bh: i32, above: *const u8,
                                  left: *const u8, upsample_above: i32, upsample_left: i32, dx: i32,
                                  dy: i32);
        fn av1_dr_prediction_z3_c(dst: *mut u8, stride: isize, bw: i32, bh: i32, above: *const u8,
                                  left: *const u8, upsample_left: i32, dx: i32, dy: i32);
    }

    type Z1Lbd = unsafe extern "C-unwind" fn(*mut u8, isize, i32, i32, *const u8, *const u8, i32, i32, i32);
    type Z2Lbd =
        unsafe extern "C-unwind" fn(*mut u8, isize, i32, i32, *const u8, *const u8, i32, i32, i32, i32);
    type Z3Lbd = unsafe extern "C-unwind" fn(*mut u8, isize, i32, i32, *const u8, *const u8, i32, i32, i32);

    /// The z{1,2,3}_wrapper adapters of the original.
    #[derive(Clone, Copy)]
    enum DrPred {
        Z1(Z1Lbd),
        Z2(Z2Lbd),
        Z3(Z3Lbd),
    }

    impl DrPred {
        #[allow(clippy::too_many_arguments)]
        unsafe fn call(self, dst: *mut u8, stride: isize, bw: i32, bh: i32, above: *const u8, left: *const u8,
                       upsample_above: i32, upsample_left: i32, dx: i32, dy: i32) {
            match self {
                DrPred::Z1(f) => f(dst, stride, bw, bh, above, left, upsample_above, dx, dy),
                DrPred::Z2(f) => f(dst, stride, bw, bh, above, left, upsample_above, upsample_left, dx, dy),
                DrPred::Z3(f) => f(dst, stride, bw, bh, above, left, upsample_left, dx, dy),
            }
        }
    }

    /// DrPredTest<uint8_t, DrPred> (LowbdDrPredTest) at AOM_BITS_8.
    struct LowbdDrPredTest {
        ref_fn: DrPred,
        tst_fn: DrPred,
        start_angle: i32,
        stop_angle: i32,
        rng: ACMRandom,
        blocks: usize,
    }

    impl LowbdDrPredTest {
        fn new(ref_fn: DrPred, tst_fn: DrPred, start_angle: i32) -> Self {
            Self { ref_fn, tst_fn, start_angle, stop_angle: start_angle + 90, rng: ACMRandom::new(), blocks: 0 }
        }

        fn run_test(&mut self, needsaturation: bool, p_angle: i32, enable_upsample: i32, dx: i32, dy: i32) {
            let bd = 8;
            for tx in 0..TX_SIZES_ALL {
                let bw = TX_SIZE_WIDE[tx];
                let bh = TX_SIZE_HIGH[tx];
                let (upsample_above, upsample_left) = if enable_upsample != 0 {
                    (av1_use_intra_edge_upsample(bw, bh, p_angle - 90, 0),
                     av1_use_intra_edge_upsample(bw, bh, p_angle - 180, 0))
                } else {
                    (0, 0)
                };

                let mut left_data = [0u8; K_NUM_INTRA_NEIGHBOUR_PIXELS];
                let mut above_data = [0u8; K_NUM_INTRA_NEIGHBOUR_PIXELS];
                if needsaturation {
                    let sat = ((1 << bd) - 1) as u8;
                    for i in 0..K_NUM_INTRA_NEIGHBOUR_PIXELS {
                        left_data[i] = sat;
                        above_data[i] = sat;
                    }
                } else {
                    for i in 0..K_NUM_INTRA_NEIGHBOUR_PIXELS {
                        left_data[i] = self.rng.rand8();
                        above_data[i] = self.rng.rand8();
                    }
                }
                // We need to allow reading some previous bytes from the input pointers.
                let above = above_data[K_INTRA_PRED_INPUT_PADDING..].as_ptr();
                let left = left_data[K_INTRA_PRED_INPUT_PADDING..].as_ptr();

                // Add additional padding to allow detection of over reads/writes when
                // the transform width is equal to MAX_TX_SIZE.
                let dst_stride = MAX_TX_SIZE + 16;
                let mut dst_ref = vec![0u8; dst_stride * bh as usize];
                let mut dst_tst = vec![0u8; dst_stride * bh as usize];
                // SAFETY: the edge buffers and destinations have the extents of the
                // original test.
                unsafe {
                    self.ref_fn.call(dst_ref.as_mut_ptr(), dst_stride as isize, bw, bh, above, left,
                                     upsample_above, upsample_left, dx, dy);
                    self.tst_fn.call(dst_tst.as_mut_ptr(), dst_stride as isize, bw, bh, above, left,
                                     upsample_above, upsample_left, dx, dy);
                }
                for r in 0..bh as usize {
                    for c in 0..bw as usize {
                        assert_eq!(dst_ref[r * dst_stride + c], dst_tst[r * dst_stride + c],
                                   "{}x{} r: {} c: {} dx: {} dy: {} upsample_above: {} upsample_left: {}",
                                   bw, bh, r, c, dx, dy, upsample_above, upsample_left);
                    }
                }
                self.blocks += 1;
            }
        }

        /// TEST_P(LowbdDrPredTest, SaturatedValues) and OperationCheck
        /// (RundrPredTest(0)).
        fn run(&mut self, needsaturation: bool) {
            for enable_upsample in 0..2 {
                for angle in self.start_angle..self.stop_angle {
                    let dx = av1_get_dx(angle);
                    let dy = av1_get_dy(angle);
                    if dx != 0 && dy != 0 {
                        self.run_test(needsaturation, angle, enable_upsample, dx, dy);
                    }
                }
            }
        }
    }

    fn run_lowbd(needsaturation: bool) {
        let params = [
            (DrPred::Z1(av1_dr_prediction_z1_c), DrPred::Z1(av1_dr_prediction_z1_neon), K_Z1_START),
            (DrPred::Z2(av1_dr_prediction_z2_c), DrPred::Z2(av1_dr_prediction_z2_neon), K_Z2_START),
            (DrPred::Z3(av1_dr_prediction_z3_c), DrPred::Z3(av1_dr_prediction_z3_neon), K_Z3_START),
        ];
        let mut blocks = 0;
        for (r, t, start) in params {
            let mut test = LowbdDrPredTest::new(r, t, start);
            test.run(needsaturation);
            blocks += test.blocks;
        }
        eprintln!("3 parameter sets, {} blocks compared", blocks);
    }

    #[test]
    fn lowbd_dr_pred_neon_saturated_values() {
        run_lowbd(true);
    }

    #[test]
    fn lowbd_dr_pred_neon_operation_check() {
        run_lowbd(false);
    }
}

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use super::*;
    use aom::simd::aom_dsp__x86__intrapred_avx2::{
        av1_highbd_dr_prediction_z1_avx2, av1_highbd_dr_prediction_z2_avx2,
        av1_highbd_dr_prediction_z3_avx2,
    };

    fn avx2_params() -> Vec<(DrPredHbd, DrPredHbd, i32, i32)> {
        params(av1_highbd_dr_prediction_z1_avx2, av1_highbd_dr_prediction_z2_avx2,
               av1_highbd_dr_prediction_z3_avx2)
    }

    #[test]
    fn highbd_dr_pred_avx2_saturated_values() {
        assert!(std::arch::is_x86_feature_detected!("avx2"), "AVX2 instantiation needs an AVX2 CPU");
        run_all(avx2_params(), true);
    }

    #[test]
    fn highbd_dr_pred_avx2_operation_check() {
        assert!(std::arch::is_x86_feature_detected!("avx2"), "AVX2 instantiation needs an AVX2 CPU");
        run_all(avx2_params(), false);
    }
}
