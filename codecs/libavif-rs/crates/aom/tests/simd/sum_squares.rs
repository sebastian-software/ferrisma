// Supplementary transcription of upstream/aom/test/sum_squares_test.cc for
// the NEON instantiations: SumSquaresTest (aom_sum_squares_2d_i16_neon:
// OperationCheck, ExtremeValues, ArbitraryWidthAndHeight), SumSquares1DTest
// (aom_sum_squares_i16_neon: RandomValues, ExtremeValues), SSETest
// (aom_sse_neon and aom_highbd_sse_neon over widths 4..128 step 4:
// OperationCheck, ExtremeValues), Lowbd2dVarTest (aom_var_2d_u8_neon) and
// Highbd2dVarTest (aom_var_2d_u16_neon). Each original TEST_P starts from a
// fresh fixture seeded with ACMRandom::DeterministicSeed(); buffer sizes,
// strides, value ranges, iteration counts and the exact-equality checks
// follow the original. The speed tests are not transcribed. C++ evaluates
// `rnd_(2) ? rnd_(limit) : -rnd_(limit)` and the other draws in source order,
// as transcribed here.
#![cfg(target_arch = "aarch64")]

use aom::simd::aom_dsp__arm__highbd_sse_neon::aom_highbd_sse_neon;
use aom::simd::aom_dsp__arm__sse_neon::aom_sse_neon;
use aom::simd::aom_dsp__arm__sum_squares_neon::*;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn aom_sum_squares_2d_i16_c(src: *const i16, stride: i32, width: i32, height: i32) -> u64;
    fn aom_sum_squares_i16_c(src: *const i16, n: u32) -> u64;
    fn aom_sse_c(a: *const u8, a_stride: i32, b: *const u8, b_stride: i32, width: i32,
                 height: i32) -> i64;
    fn aom_highbd_sse_c(a: *const u8, a_stride: i32, b: *const u8, b_stride: i32, width: i32,
                        height: i32) -> i64;
    fn aom_var_2d_u8_c(src: *mut u8, src_stride: i32, width: i32, height: i32) -> u64;
    fn aom_var_2d_u16_c(src: *mut u8, src_stride: i32, width: i32, height: i32) -> u64;
}

type SSI16Func = unsafe extern "C-unwind" fn(*const i16, i32, i32, i32) -> u64;
type F1D = unsafe extern "C-unwind" fn(*const i16, u32) -> u64;
type SSEFunc = unsafe extern "C-unwind" fn(*const u8, i32, *const u8, i32, i32, i32) -> i64;
type Var2DFunc = unsafe extern "C-unwind" fn(*mut u8, i32, i32, i32) -> u64;

const K_NUM_ITERATIONS: usize = 10000;
const K_INT13_MAX: i16 = (1 << 12) - 1;

#[repr(C, align(32))]
struct Aligned<T>(T);

fn convert_to_byteptr<T>(p: *mut T) -> *mut u8 {
    ((p as usize) >> 1) as *mut u8
}

// SumSquaresTest
fn sum_squares_run_test(ref_func: SSI16Func, tst_func: SSI16Func, is_random: bool,
                        multiple_of_4: bool) {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = vec![0i16; 256 * 256];
    for _ in 0..K_NUM_ITERATIONS {
        // Up to 128x128
        let width = if multiple_of_4 { 4 * (rnd.pseudo_uniform(31) + 1) } else {
            rnd.pseudo_uniform(127) + 1
        };
        let height = if multiple_of_4 { 4 * (rnd.pseudo_uniform(31) + 1) } else {
            rnd.pseudo_uniform(127) + 1
        };
        let mut stride = 4 << rnd.pseudo_uniform(7); // Up to 256 stride
        while stride < width {
            stride = 4 << rnd.pseudo_uniform(7);
        }
        let limit = 1 << 12;
        if is_random {
            for ii in 0..height {
                for jj in 0..width {
                    src[(ii * stride + jj) as usize] = if rnd.pseudo_uniform(2) != 0 {
                        rnd.pseudo_uniform(limit) as i16
                    } else {
                        -rnd.pseudo_uniform(limit) as i16
                    };
                }
            }
        } else {
            let val = if rnd.pseudo_uniform(2) != 0 { limit - 1 } else { -(limit - 1) };
            for ii in 0..height {
                for jj in 0..width {
                    src[(ii * stride + jj) as usize] = val as i16;
                }
            }
        }
        // SAFETY: the 256x256 buffer covers height rows at `stride`.
        let (res_ref, res_tst) = unsafe {
            (ref_func(src.as_ptr(), stride, width, height), tst_func(src.as_ptr(), stride, width,
                                                                     height))
        };
        assert_eq!(res_ref, res_tst,
                   "Error: Sum Squares Test [{width}x{height}] C output does not match optimized \
                    output.");
    }
}

#[test]
fn sum_squares_2d_i16_operation_check() {
    sum_squares_run_test(aom_sum_squares_2d_i16_c, aom_sum_squares_2d_i16_neon, true, true);
}

#[test]
fn sum_squares_2d_i16_extreme_values() {
    sum_squares_run_test(aom_sum_squares_2d_i16_c, aom_sum_squares_2d_i16_neon, false, true);
}

#[test]
fn sum_squares_2d_i16_arbitrary_width_and_height() {
    sum_squares_run_test(aom_sum_squares_2d_i16_c, aom_sum_squares_2d_i16_neon, true, false);
}

// SumSquares1DTest
const K_ITERATIONS_1D: usize = 1000;
const K_MAX_SIZE: usize = 256;

fn sum_squares_1d(ref_func: F1D, tst_func: F1D, random: bool) {
    let mut rng = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = Box::new(Aligned([0i16; K_MAX_SIZE * K_MAX_SIZE]));
    for _ in 0..K_ITERATIONS_1D {
        if random {
            for i in 0..K_MAX_SIZE * K_MAX_SIZE {
                src.0[i] = (rng.pseudo_uniform(K_INT13_MAX as i32 * 2 + 1) - K_INT13_MAX as i32)
                    as i16;
            }
        } else if rng.pseudo_uniform(2) != 0 {
            src.0.fill(K_INT13_MAX);
        } else {
            src.0.fill(-K_INT13_MAX);
        }
        // Block size is between 64 and 128 * 128 and is always a multiple of 64.
        let n = ((rng.pseudo_uniform(255) + 1) * 64) as u32;
        // SAFETY: n <= 256 * 64 elements of the buffer are read.
        let (ref_res, tst_res) = unsafe { (ref_func(src.0.as_ptr(), n), tst_func(src.0.as_ptr(), n)) };
        assert_eq!(ref_res, tst_res);
    }
}

#[test]
fn sum_squares_i16_random_values() {
    sum_squares_1d(aom_sum_squares_i16_c, aom_sum_squares_i16_neon, true);
}

#[test]
fn sum_squares_i16_extreme_values() {
    sum_squares_1d(aom_sum_squares_i16_c, aom_sum_squares_i16_neon, false);
}

// SSETest
struct SseFixture {
    is_hbd: bool,
    src: Box<Aligned<[u8; 256 * 256 * 2]>>,
    ref_: Box<Aligned<[u8; 256 * 256 * 2]>>,
    rnd: ACMRandom,
}

impl SseFixture {
    fn gen_random_data(&mut self, width: i32, height: i32, stride: i32) {
        let limit = 1 << 12;
        for ii in 0..height {
            for jj in 0..width {
                let idx = (ii * stride + jj) as usize;
                if !self.is_hbd {
                    self.src.0[idx] = self.rnd.rand8();
                    self.ref_.0[idx] = self.rnd.rand8();
                } else {
                    let s = self.rnd.pseudo_uniform(limit) as u16;
                    let r = self.rnd.pseudo_uniform(limit) as u16;
                    self.src.0[2 * idx..2 * idx + 2].copy_from_slice(&s.to_ne_bytes());
                    self.ref_.0[2 * idx..2 * idx + 2].copy_from_slice(&r.to_ne_bytes());
                }
            }
        }
    }

    fn gen_extreme_data(is_hbd: bool, width: i32, height: i32, stride: i32, data: &mut [u8],
                        val: i16) {
        for ii in 0..height {
            for jj in 0..width {
                let idx = (ii * stride + jj) as usize;
                if !is_hbd {
                    data[idx] = val as u8;
                } else {
                    data[2 * idx..2 * idx + 2].copy_from_slice(&(val as u16).to_ne_bytes());
                }
            }
        }
    }

    fn run_test(&mut self, ref_func: SSEFunc, tst_func: SSEFunc, is_random: bool, width: i32,
                height: i32) {
        for k in 0..3 {
            let mut stride = 4 << self.rnd.pseudo_uniform(7);
            while stride < width {
                stride = 4 << self.rnd.pseudo_uniform(7);
            }
            if is_random {
                self.gen_random_data(width, height, stride);
            } else {
                let msb = if self.is_hbd { 12 } else { 8 };
                let limit = ((1 << msb) - 1) as i16;
                if k == 0 {
                    Self::gen_extreme_data(self.is_hbd, width, height, stride, &mut self.src.0, 0);
                    Self::gen_extreme_data(self.is_hbd, width, height, stride, &mut self.ref_.0,
                                           limit);
                } else {
                    Self::gen_extreme_data(self.is_hbd, width, height, stride, &mut self.src.0,
                                           limit);
                    Self::gen_extreme_data(self.is_hbd, width, height, stride, &mut self.ref_.0, 0);
                }
            }
            let mut src = self.src.0.as_mut_ptr();
            let mut ref_ = self.ref_.0.as_mut_ptr();
            if self.is_hbd {
                src = convert_to_byteptr(src as *mut u16);
                ref_ = convert_to_byteptr(ref_ as *mut u16);
            }
            // SAFETY: the buffers hold 256x256 samples of the selected depth;
            // height rows of width samples at `stride` lie within them.
            let (res_ref, res_tst) = unsafe {
                (ref_func(src, stride, ref_, stride, width, height),
                 tst_func(src, stride, ref_, stride, width, height))
            };
            assert_eq!(res_ref, res_tst,
                       "Error:{}{k} SSE Test [{width}x{height}] C output does not match optimized \
                        output.", if self.is_hbd { "hbd " } else { " " });
        }
    }
}

fn sse_test(ref_func: SSEFunc, tst_func: SSEFunc, is_hbd: bool, is_random: bool) {
    // Range(4, 129, 4)
    for width in (4..129).step_by(4) {
        let mut f = SseFixture {
            is_hbd,
            src: Box::new(Aligned([0; 256 * 256 * 2])),
            ref_: Box::new(Aligned([0; 256 * 256 * 2])),
            rnd: ACMRandom::with_seed(ACMRandom::deterministic_seed()),
        };
        for height in (4..=128).step_by(4) {
            f.run_test(ref_func, tst_func, is_random, width, height);
        }
    }
}

#[test]
fn sse_operation_check() {
    sse_test(aom_sse_c, aom_sse_neon, false, true);
}

#[test]
fn sse_extreme_values() {
    sse_test(aom_sse_c, aom_sse_neon, false, false);
}

#[test]
fn highbd_sse_operation_check() {
    sse_test(aom_highbd_sse_c, aom_highbd_sse_neon, true, true);
}

#[test]
fn highbd_sse_extreme_values() {
    sse_test(aom_highbd_sse_c, aom_highbd_sse_neon, true, false);
}

// Lowbd2dVarTest
fn lowbd_2d_var_run_test(ref_func: Var2DFunc, tst_func: Var2DFunc, is_random: bool) {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = Box::new(Aligned([0u8; 512 * 512]));
    for _ in 0..K_NUM_ITERATIONS {
        let width = 4 * (rnd.pseudo_uniform(63) + 1); // Up to 256x256
        let height = 4 * (rnd.pseudo_uniform(63) + 1); // Up to 256x256
        let mut stride = 4 << rnd.pseudo_uniform(8); // Up to 512 stride
        while stride < width {
            stride = 4 << rnd.pseudo_uniform(8);
        }
        let limit = 1 << 8;
        if is_random {
            for ii in 0..height {
                for jj in 0..width {
                    src.0[(ii * stride + jj) as usize] = rnd.pseudo_uniform(limit) as u8;
                }
            }
        } else {
            let val = if rnd.pseudo_uniform(2) != 0 { limit - 1 } else { 0 };
            for ii in 0..height {
                for jj in 0..width {
                    src.0[(ii * stride + jj) as usize] = val as u8;
                }
            }
        }
        // SAFETY: the 512x512 buffer covers height rows at `stride`.
        let (res_ref, res_tst) = unsafe {
            (ref_func(src.0.as_mut_ptr(), stride, width, height),
             tst_func(src.0.as_mut_ptr(), stride, width, height))
        };
        assert_eq!(res_ref, res_tst,
                   "Error: Sum Squares Test [{width}x{height}] C output does not match optimized \
                    output.");
    }
}

#[test]
fn var_2d_u8_operation_check() {
    lowbd_2d_var_run_test(aom_var_2d_u8_c, aom_var_2d_u8_neon, true);
}

#[test]
fn var_2d_u8_extreme_values() {
    lowbd_2d_var_run_test(aom_var_2d_u8_c, aom_var_2d_u8_neon, false);
}

// Highbd2dVarTest
fn highbd_2d_var_run_test(ref_func: Var2DFunc, tst_func: Var2DFunc, is_random: bool,
                          multiple_of_4: bool) {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = Box::new(Aligned([0u16; 512 * 512]));
    for _ in 0..K_NUM_ITERATIONS {
        // Up to 256x256
        let width = if multiple_of_4 { 4 * (rnd.pseudo_uniform(63) + 1) } else {
            rnd.pseudo_uniform(255) + 1
        };
        let height = if multiple_of_4 { 4 * (rnd.pseudo_uniform(63) + 1) } else {
            rnd.pseudo_uniform(255) + 1
        };
        let mut stride = 4 << rnd.pseudo_uniform(8); // Up to 512 stride
        while stride < width {
            stride = 4 << rnd.pseudo_uniform(8);
        }
        let limit = 1 << 12;
        if is_random {
            for ii in 0..height {
                for jj in 0..width {
                    src.0[(ii * stride + jj) as usize] = rnd.pseudo_uniform(limit) as u16;
                }
            }
        } else {
            let val = if rnd.pseudo_uniform(2) != 0 { limit - 1 } else { 0 };
            for ii in 0..height {
                for jj in 0..width {
                    src.0[(ii * stride + jj) as usize] = val as u16;
                }
            }
        }
        let p = convert_to_byteptr(src.0.as_mut_ptr());
        // SAFETY: the 512x512 buffer covers height rows at `stride`.
        let (res_ref, res_tst) = unsafe {
            (ref_func(p, stride, width, height), tst_func(p, stride, width, height))
        };
        assert_eq!(res_ref, res_tst,
                   "Error: Sum Squares Test [{width}x{height}] C output does not match optimized \
                    output.");
    }
}

#[test]
fn var_2d_u16_operation_check() {
    highbd_2d_var_run_test(aom_var_2d_u16_c, aom_var_2d_u16_neon, true, true);
}

#[test]
fn var_2d_u16_extreme_values() {
    highbd_2d_var_run_test(aom_var_2d_u16_c, aom_var_2d_u16_neon, false, true);
}

#[test]
fn var_2d_u16_arbitrary_width_and_height() {
    highbd_2d_var_run_test(aom_var_2d_u16_c, aom_var_2d_u16_neon, true, false);
}
