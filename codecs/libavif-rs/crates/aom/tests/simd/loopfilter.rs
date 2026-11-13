// Supplementary transcription of upstream/aom/test/lpf_test.cc
// (Loop8Test6Param_lbd, Loop8Test9Param_lbd, Loop8Test6Param_hbd and
// Loop8Test9Param_hbd: OperationCheck and ValueCheck) for the NEON
// instantiations of aom_dsp/arm/loopfilter_neon.c and
// aom_dsp/arm/highbd_loopfilter_neon.c. Seed (one ACMRandom per test case),
// iteration count (10000), buffer layout (32x32, filtered at s + 8 + 8 * p),
// threshold draws, InitInput, parameter lists and bit depths, and the
// exact-match check over the whole buffer follow the original. DISABLED_Speed
// is not transcribed.
#![cfg(target_arch = "aarch64")]
use aom::simd::aom_dsp__arm__highbd_loopfilter_neon::*;
use aom::simd::aom_dsp__arm__loopfilter_neon::*;

use super::acm_random::ACMRandom;

const K_NUM_COEFFS: usize = 1024;
const NUMBER_OF_ITERATIONS: usize = 10000;
// av1/common/av1_loopfilter.h
const MAX_LOOP_FILTER: i32 = 63;

type LoopOp = unsafe extern "C-unwind" fn(*mut u8, i32, *const u8, *const u8, *const u8);
type DualLoopOp = unsafe extern "C-unwind" fn(*mut u8, i32, *const u8, *const u8, *const u8,
                                              *const u8, *const u8, *const u8);
type HbdLoopOp = unsafe extern "C-unwind" fn(*mut u16, i32, *const u8, *const u8, *const u8, i32);
type HbdDualLoopOp = unsafe extern "C-unwind" fn(*mut u16, i32, *const u8, *const u8, *const u8,
                                                 *const u8, *const u8, *const u8, i32);

extern "C-unwind" {
    fn aom_lpf_vertical_14_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_vertical_8_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_vertical_6_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_vertical_4_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_14_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_8_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_6_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_4_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_4_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_vertical_4_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_6_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_vertical_6_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_8_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_vertical_8_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);
    fn aom_lpf_horizontal_14_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8,
                                    t: *const u8);
    fn aom_lpf_vertical_14_quad_c(s: *mut u8, p: i32, b: *const u8, l: *const u8, t: *const u8);

    fn aom_lpf_horizontal_4_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                   t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_horizontal_6_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                   t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_horizontal_8_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                   t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_horizontal_14_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                    t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_vertical_4_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                 t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_vertical_6_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                 t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_vertical_8_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                 t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);
    fn aom_lpf_vertical_14_dual_c(s: *mut u8, p: i32, b0: *const u8, l0: *const u8,
                                  t0: *const u8, b1: *const u8, l1: *const u8, t1: *const u8);

    fn aom_highbd_lpf_horizontal_4_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                     t: *const u8, bd: i32);
    fn aom_highbd_lpf_horizontal_6_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                     t: *const u8, bd: i32);
    fn aom_highbd_lpf_horizontal_8_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                     t: *const u8, bd: i32);
    fn aom_highbd_lpf_horizontal_14_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                      t: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_4_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                   t: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_6_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                   t: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_8_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                   t: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_14_c(s: *mut u16, p: i32, b: *const u8, l: *const u8,
                                    t: *const u8, bd: i32);

    fn aom_highbd_lpf_horizontal_4_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                          t0: *const u8, b1: *const u8, l1: *const u8,
                                          t1: *const u8, bd: i32);
    fn aom_highbd_lpf_horizontal_6_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                          t0: *const u8, b1: *const u8, l1: *const u8,
                                          t1: *const u8, bd: i32);
    fn aom_highbd_lpf_horizontal_8_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                          t0: *const u8, b1: *const u8, l1: *const u8,
                                          t1: *const u8, bd: i32);
    fn aom_highbd_lpf_horizontal_14_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                           t0: *const u8, b1: *const u8, l1: *const u8,
                                           t1: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_4_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                        t0: *const u8, b1: *const u8, l1: *const u8,
                                        t1: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_6_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                        t0: *const u8, b1: *const u8, l1: *const u8,
                                        t1: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_8_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                        t0: *const u8, b1: *const u8, l1: *const u8,
                                        t1: *const u8, bd: i32);
    fn aom_highbd_lpf_vertical_14_dual_c(s: *mut u16, p: i32, b0: *const u8, l0: *const u8,
                                         t0: *const u8, b1: *const u8, l1: *const u8,
                                         t1: *const u8, bd: i32);
}

#[repr(C, align(16))]
struct Aligned16<T>(T);

/// The 16-byte threshold vector the original declares per draw.
fn thresh_vec(v: u8) -> Aligned16<[u8; 16]> {
    Aligned16([v; 16])
}

fn get_outer_thresh(rnd: &mut ACMRandom) -> u8 {
    rnd.pseudo_uniform(3 * MAX_LOOP_FILTER + 5) as u8
}

fn get_inner_thresh(rnd: &mut ACMRandom) -> u8 {
    rnd.pseudo_uniform(MAX_LOOP_FILTER + 1) as u8
}

fn get_hev_thresh(rnd: &mut ACMRandom) -> u8 {
    (rnd.pseudo_uniform(MAX_LOOP_FILTER + 1) >> 4) as u8
}

/// Pixel storage of the original's Pixel_t (uint8_t or uint16_t).
trait Pixel: Copy + PartialEq + Default {
    fn from_masked(v: u16, mask: i32) -> Self;
}

impl Pixel for u8 {
    fn from_masked(v: u16, mask: i32) -> Self {
        ((v as i32) & mask) as u8
    }
}

impl Pixel for u16 {
    fn from_masked(v: u16, mask: i32) -> Self {
        ((v as i32) & mask) as u16
    }
}

/// InitInput<Pixel_t, PIXEL_WIDTH_t>.
fn init_input<P: Pixel>(s: &mut [P], ref_s: &mut [P], rnd: &mut ACMRandom, limit: u8, mask: i32,
                        p: i32, i: usize) {
    let mut tmp_s = [0u16; K_NUM_COEFFS];
    let limit = limit as i32;
    let n = K_NUM_COEFFS as i32;

    let mut j: i32 = 0;
    while j < n {
        let val = rnd.rand8();
        if val & 0x80 != 0 {
            // 50% chance to choose a new value.
            tmp_s[j as usize] = rnd.rand16();
            j += 1;
        } else {
            // 50% chance to repeat previous value in row X times.
            let mut k = 0;
            loop {
                let cont = k < ((val & 0x1f) as i32 + 1) && j < n;
                k += 1;
                if !cont {
                    break;
                }
                if j < 1 {
                    tmp_s[j as usize] = rnd.rand16();
                } else if val & 0x20 != 0 {
                    // Increment by a value within the limit.
                    tmp_s[j as usize] = (tmp_s[(j - 1) as usize] as i32 + (limit - 1)) as u16;
                } else {
                    // Decrement by a value within the limit.
                    tmp_s[j as usize] = (tmp_s[(j - 1) as usize] as i32 - (limit - 1)) as u16;
                }
                j += 1;
            }
        }
    }

    j = 0;
    while j < n {
        let val = rnd.rand8();
        if val & 0x80 != 0 {
            j += 1;
        } else {
            // 50% chance to repeat previous value in column X times.
            let mut k = 0;
            loop {
                let cont = k < ((val & 0x1f) as i32 + 1) && j < n;
                k += 1;
                if !cont {
                    break;
                }
                if j < 1 {
                    tmp_s[j as usize] = rnd.rand16();
                } else if val & 0x20 != 0 {
                    // Increment by a value within the limit.
                    tmp_s[((j % 32) * 32 + j / 32) as usize] =
                        (tmp_s[(((j - 1) % 32) * 32 + (j - 1) / 32) as usize] as i32
                            + (limit - 1)) as u16;
                } else {
                    // Decrement by a value within the limit.
                    tmp_s[((j % 32) * 32 + j / 32) as usize] =
                        (tmp_s[(((j - 1) % 32) * 32 + (j - 1) / 32) as usize] as i32
                            - (limit - 1)) as u16;
                }
                j += 1;
            }
        }
    }

    for j in 0..K_NUM_COEFFS {
        if i % 2 != 0 {
            s[j] = P::from_masked(tmp_s[j], mask);
        } else {
            let jj = j as i32;
            s[j] = P::from_masked(tmp_s[(p * (jj % p) + jj / p) as usize], mask);
        }
        ref_s[j] = s[j];
    }
}

/// One filter call on a buffer with `p` stride, through either single or dual
/// signature; `th` holds blimit0, limit0, thresh0, blimit1, limit1, thresh1.
trait Op<P> {
    fn call(&self, s: *mut P, p: i32, th: &[Aligned16<[u8; 16]>; 6], bd: i32);
    fn dual(&self) -> bool;
}

impl Op<u8> for LoopOp {
    fn call(&self, s: *mut u8, p: i32, th: &[Aligned16<[u8; 16]>; 6], _bd: i32) {
        // SAFETY: `s` points 8 rows and 8 columns into a 32x32 buffer, the
        // extent the original test provides for every loop filter.
        unsafe { self(s, p, th[0].0.as_ptr(), th[1].0.as_ptr(), th[2].0.as_ptr()) }
    }
    fn dual(&self) -> bool {
        false
    }
}

impl Op<u8> for DualLoopOp {
    fn call(&self, s: *mut u8, p: i32, th: &[Aligned16<[u8; 16]>; 6], _bd: i32) {
        // SAFETY: as above.
        unsafe {
            self(s, p, th[0].0.as_ptr(), th[1].0.as_ptr(), th[2].0.as_ptr(), th[3].0.as_ptr(),
                 th[4].0.as_ptr(), th[5].0.as_ptr())
        }
    }
    fn dual(&self) -> bool {
        true
    }
}

impl Op<u16> for HbdLoopOp {
    fn call(&self, s: *mut u16, p: i32, th: &[Aligned16<[u8; 16]>; 6], bd: i32) {
        // SAFETY: as above.
        unsafe { self(s, p, th[0].0.as_ptr(), th[1].0.as_ptr(), th[2].0.as_ptr(), bd) }
    }
    fn dual(&self) -> bool {
        false
    }
}

impl Op<u16> for HbdDualLoopOp {
    fn call(&self, s: *mut u16, p: i32, th: &[Aligned16<[u8; 16]>; 6], bd: i32) {
        // SAFETY: as above.
        unsafe {
            self(s, p, th[0].0.as_ptr(), th[1].0.as_ptr(), th[2].0.as_ptr(), th[3].0.as_ptr(),
                 th[4].0.as_ptr(), th[5].0.as_ptr(), bd)
        }
    }
    fn dual(&self) -> bool {
        true
    }
}

/// OPCHECK / OPCHECKd (`value_check == false`) and VALCHECK / VALCHECKd
/// (`value_check == true`). Returns the number of compared blocks.
fn check<P: Pixel, F: Op<P>>(op: &F, ref_op: &F, bit_depth: i32, value_check: bool,
                             name: &str) -> usize {
    let mask = (1i32 << bit_depth) - 1;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let p: i32 = (K_NUM_COEFFS / 32) as i32;
    let mut s = Box::new(Aligned16([P::default(); K_NUM_COEFFS]));
    let mut ref_s = Box::new(Aligned16([P::default(); K_NUM_COEFFS]));
    let mut err_count_total = 0usize;
    let mut first_failure: i64 = -1;
    for i in 0..NUMBER_OF_ITERATIONS {
        let blimit0 = thresh_vec(get_outer_thresh(&mut rnd));
        let limit0 = thresh_vec(get_inner_thresh(&mut rnd));
        let thresh0 = thresh_vec(get_hev_thresh(&mut rnd));
        let (blimit1, limit1, thresh1) = if op.dual() {
            let b = thresh_vec(get_outer_thresh(&mut rnd));
            let l = thresh_vec(get_inner_thresh(&mut rnd));
            let t = thresh_vec(get_hev_thresh(&mut rnd));
            (b, l, t)
        } else {
            (thresh_vec(0), thresh_vec(0), thresh_vec(0))
        };
        if value_check {
            for j in 0..K_NUM_COEFFS {
                s.0[j] = P::from_masked(rnd.rand16(), mask);
                ref_s.0[j] = s.0[j];
            }
        } else {
            let limit = if op.dual() {
                if limit0.0[0] < limit1.0[0] { limit0.0[0] } else { limit1.0[0] }
            } else {
                limit0.0[0]
            };
            init_input(&mut s.0, &mut ref_s.0, &mut rnd, limit, mask, p, i);
        }
        let th = [blimit0, limit0, thresh0, blimit1, limit1, thresh1];
        let offset = (8 + p * 8) as usize;
        ref_op.call(ref_s.0[offset..].as_mut_ptr(), p, &th, bit_depth);
        op.call(s.0[offset..].as_mut_ptr(), p, &th, bit_depth);
        let err_count = (0..K_NUM_COEFFS).filter(|&j| ref_s.0[j] != s.0[j]).count();
        if err_count != 0 && err_count_total == 0 {
            first_failure = i as i64;
        }
        err_count_total += err_count;
    }
    assert_eq!(0, err_count_total,
               "{name} bd {bit_depth} ({}): C output doesn't match SIMD loopfilter output. \
                First failed at test case {first_failure}",
               if value_check { "ValueCheck" } else { "OperationCheck" });
    NUMBER_OF_ITERATIONS
}

fn run_all<P: Pixel, F: Op<P>>(params: &[(F, F, i32, &str)]) -> usize {
    let mut blocks = 0;
    for (op, ref_op, bd, name) in params {
        blocks += check(op, ref_op, *bd, false, name);
        blocks += check(op, ref_op, *bd, true, name);
    }
    blocks
}

macro_rules! lbd {
    ($neon:ident, $c:ident) => {
        ($neon as LoopOp, $c as LoopOp, 8, stringify!($neon))
    };
}

macro_rules! lbd_dual {
    ($neon:ident, $c:ident) => {
        ($neon as DualLoopOp, $c as DualLoopOp, 8, stringify!($neon))
    };
}

macro_rules! hbd {
    ($neon:ident, $c:ident, $bd:literal) => {
        ($neon as HbdLoopOp, $c as HbdLoopOp, $bd, stringify!($neon))
    };
}

macro_rules! hbd_dual {
    ($neon:ident, $c:ident, $bd:literal) => {
        ($neon as HbdDualLoopOp, $c as HbdDualLoopOp, $bd, stringify!($neon))
    };
}

/// INSTANTIATE_TEST_SUITE_P(NEON, Loop8Test6Param_lbd, kLoop8Test6).
#[test]
fn loopfilter_loop8_test6_param_lbd_neon() {
    let params = [
        lbd!(aom_lpf_vertical_14_neon, aom_lpf_vertical_14_c),
        lbd!(aom_lpf_vertical_8_neon, aom_lpf_vertical_8_c),
        lbd!(aom_lpf_vertical_6_neon, aom_lpf_vertical_6_c),
        lbd!(aom_lpf_vertical_4_neon, aom_lpf_vertical_4_c),
        lbd!(aom_lpf_horizontal_14_neon, aom_lpf_horizontal_14_c),
        lbd!(aom_lpf_horizontal_8_neon, aom_lpf_horizontal_8_c),
        lbd!(aom_lpf_horizontal_6_neon, aom_lpf_horizontal_6_c),
        lbd!(aom_lpf_horizontal_4_neon, aom_lpf_horizontal_4_c),
        lbd!(aom_lpf_horizontal_4_quad_neon, aom_lpf_horizontal_4_quad_c),
        lbd!(aom_lpf_vertical_4_quad_neon, aom_lpf_vertical_4_quad_c),
        lbd!(aom_lpf_horizontal_6_quad_neon, aom_lpf_horizontal_6_quad_c),
        lbd!(aom_lpf_vertical_6_quad_neon, aom_lpf_vertical_6_quad_c),
        lbd!(aom_lpf_horizontal_8_quad_neon, aom_lpf_horizontal_8_quad_c),
        lbd!(aom_lpf_vertical_8_quad_neon, aom_lpf_vertical_8_quad_c),
        lbd!(aom_lpf_horizontal_14_quad_neon, aom_lpf_horizontal_14_quad_c),
        lbd!(aom_lpf_vertical_14_quad_neon, aom_lpf_vertical_14_quad_c),
    ];
    let blocks = run_all(&params);
    assert_eq!(blocks, 16 * 2 * NUMBER_OF_ITERATIONS);
}

/// INSTANTIATE_TEST_SUITE_P(NEON, Loop8Test9Param_lbd, kLoop8Test9).
#[test]
fn loopfilter_loop8_test9_param_lbd_neon() {
    let params = [
        lbd_dual!(aom_lpf_horizontal_4_dual_neon, aom_lpf_horizontal_4_dual_c),
        lbd_dual!(aom_lpf_horizontal_6_dual_neon, aom_lpf_horizontal_6_dual_c),
        lbd_dual!(aom_lpf_horizontal_8_dual_neon, aom_lpf_horizontal_8_dual_c),
        lbd_dual!(aom_lpf_horizontal_14_dual_neon, aom_lpf_horizontal_14_dual_c),
        lbd_dual!(aom_lpf_vertical_4_dual_neon, aom_lpf_vertical_4_dual_c),
        lbd_dual!(aom_lpf_vertical_6_dual_neon, aom_lpf_vertical_6_dual_c),
        lbd_dual!(aom_lpf_vertical_8_dual_neon, aom_lpf_vertical_8_dual_c),
        lbd_dual!(aom_lpf_vertical_14_dual_neon, aom_lpf_vertical_14_dual_c),
    ];
    let blocks = run_all(&params);
    assert_eq!(blocks, 8 * 2 * NUMBER_OF_ITERATIONS);
}

/// INSTANTIATE_TEST_SUITE_P(NEON, Loop8Test6Param_hbd, kHbdLoop8Test6).
#[test]
fn loopfilter_loop8_test6_param_hbd_neon() {
    let params = [
        hbd!(aom_highbd_lpf_horizontal_4_neon, aom_highbd_lpf_horizontal_4_c, 8),
        hbd!(aom_highbd_lpf_horizontal_4_neon, aom_highbd_lpf_horizontal_4_c, 10),
        hbd!(aom_highbd_lpf_horizontal_4_neon, aom_highbd_lpf_horizontal_4_c, 12),
        hbd!(aom_highbd_lpf_horizontal_6_neon, aom_highbd_lpf_horizontal_6_c, 8),
        hbd!(aom_highbd_lpf_horizontal_6_neon, aom_highbd_lpf_horizontal_6_c, 10),
        hbd!(aom_highbd_lpf_horizontal_6_neon, aom_highbd_lpf_horizontal_6_c, 12),
        hbd!(aom_highbd_lpf_horizontal_8_neon, aom_highbd_lpf_horizontal_8_c, 8),
        hbd!(aom_highbd_lpf_horizontal_8_neon, aom_highbd_lpf_horizontal_8_c, 10),
        hbd!(aom_highbd_lpf_horizontal_8_neon, aom_highbd_lpf_horizontal_8_c, 12),
        hbd!(aom_highbd_lpf_horizontal_14_neon, aom_highbd_lpf_horizontal_14_c, 8),
        hbd!(aom_highbd_lpf_horizontal_14_neon, aom_highbd_lpf_horizontal_14_c, 10),
        hbd!(aom_highbd_lpf_horizontal_14_neon, aom_highbd_lpf_horizontal_14_c, 12),
        hbd!(aom_highbd_lpf_vertical_4_neon, aom_highbd_lpf_vertical_4_c, 8),
        hbd!(aom_highbd_lpf_vertical_4_neon, aom_highbd_lpf_vertical_4_c, 10),
        hbd!(aom_highbd_lpf_vertical_4_neon, aom_highbd_lpf_vertical_4_c, 12),
        hbd!(aom_highbd_lpf_vertical_6_neon, aom_highbd_lpf_vertical_6_c, 8),
        hbd!(aom_highbd_lpf_vertical_6_neon, aom_highbd_lpf_vertical_6_c, 10),
        hbd!(aom_highbd_lpf_vertical_6_neon, aom_highbd_lpf_vertical_6_c, 12),
        hbd!(aom_highbd_lpf_vertical_8_neon, aom_highbd_lpf_vertical_8_c, 8),
        hbd!(aom_highbd_lpf_vertical_8_neon, aom_highbd_lpf_vertical_8_c, 10),
        hbd!(aom_highbd_lpf_vertical_8_neon, aom_highbd_lpf_vertical_8_c, 12),
        hbd!(aom_highbd_lpf_vertical_14_neon, aom_highbd_lpf_vertical_14_c, 8),
        hbd!(aom_highbd_lpf_vertical_14_neon, aom_highbd_lpf_vertical_14_c, 10),
        hbd!(aom_highbd_lpf_vertical_14_neon, aom_highbd_lpf_vertical_14_c, 12),
    ];
    let blocks = run_all(&params);
    assert_eq!(blocks, 24 * 2 * NUMBER_OF_ITERATIONS);
}

/// INSTANTIATE_TEST_SUITE_P(NEON, Loop8Test9Param_hbd, kHbdLoop8Test9).
#[test]
fn loopfilter_loop8_test9_param_hbd_neon() {
    let params = [
        hbd_dual!(aom_highbd_lpf_horizontal_4_dual_neon, aom_highbd_lpf_horizontal_4_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_horizontal_6_dual_neon, aom_highbd_lpf_horizontal_6_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_horizontal_8_dual_neon, aom_highbd_lpf_horizontal_8_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_horizontal_14_dual_neon, aom_highbd_lpf_horizontal_14_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_vertical_4_dual_neon, aom_highbd_lpf_vertical_4_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_vertical_6_dual_neon, aom_highbd_lpf_vertical_6_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_vertical_8_dual_neon, aom_highbd_lpf_vertical_8_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_vertical_14_dual_neon, aom_highbd_lpf_vertical_14_dual_c, 8),
        hbd_dual!(aom_highbd_lpf_horizontal_4_dual_neon, aom_highbd_lpf_horizontal_4_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_horizontal_6_dual_neon, aom_highbd_lpf_horizontal_6_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_horizontal_8_dual_neon, aom_highbd_lpf_horizontal_8_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_horizontal_14_dual_neon, aom_highbd_lpf_horizontal_14_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_vertical_4_dual_neon, aom_highbd_lpf_vertical_4_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_vertical_6_dual_neon, aom_highbd_lpf_vertical_6_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_vertical_8_dual_neon, aom_highbd_lpf_vertical_8_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_vertical_14_dual_neon, aom_highbd_lpf_vertical_14_dual_c, 10),
        hbd_dual!(aom_highbd_lpf_horizontal_4_dual_neon, aom_highbd_lpf_horizontal_4_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_horizontal_6_dual_neon, aom_highbd_lpf_horizontal_6_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_horizontal_8_dual_neon, aom_highbd_lpf_horizontal_8_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_horizontal_14_dual_neon, aom_highbd_lpf_horizontal_14_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_vertical_4_dual_neon, aom_highbd_lpf_vertical_4_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_vertical_6_dual_neon, aom_highbd_lpf_vertical_6_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_vertical_8_dual_neon, aom_highbd_lpf_vertical_8_dual_c, 12),
        hbd_dual!(aom_highbd_lpf_vertical_14_dual_neon, aom_highbd_lpf_vertical_14_dual_c, 12),
    ];
    let blocks = run_all(&params);
    assert_eq!(blocks, 24 * 2 * NUMBER_OF_ITERATIONS);
}
