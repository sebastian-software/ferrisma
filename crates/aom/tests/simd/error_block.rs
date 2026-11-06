// Supplementary transcription of upstream/aom/test/error_block_test.cc
// (ErrorBlockTest OperationCheck and ExtremeValues) for its Neon
// instantiation. Inputs, seeds, iteration counts, and exact comparisons follow
// the original.
use super::acm_random::ACMRandom;

type ErrorBlockFunc = unsafe fn(*const i32, *const i32, isize, *mut i64, i32) -> i64;

extern "C-unwind" {
    fn av1_highbd_block_error_c(coeff: *const i32, dqcoeff: *const i32, block_size: isize,
                                ssz: *mut i64, bd: i32) -> i64;
    fn av1_block_error_c(coeff: *const i32, dqcoeff: *const i32, block_size: isize,
                         ssz: *mut i64) -> i64;
    fn av1_block_error_lp_c(coeff: *const i16, dqcoeff: *const i16, block_size: isize) -> i64;
}

const K_NUM_ITERATIONS: i32 = 1000;

#[repr(C, align(16))]
struct Buffer([i32; 4096]);

fn operation_check(error_block_op: ErrorBlockFunc, ref_error_block_op: ErrorBlockFunc, bit_depth: i32) {
    let mut rnd = ACMRandom::new();
    let mut coeff = Buffer([0; 4096]);
    let mut dqcoeff = Buffer([0; 4096]);
    let mut err_count_total = 0;
    let mut first_failure = -1;
    let msb = bit_depth + 8 - 1;
    for i in 0..K_NUM_ITERATIONS {
        let block_size: isize = 16 << (i % 9); // All block sizes from 4x4, 8x4 ..64x64
        for j in 0..block_size as usize {
            if rnd.pseudo_uniform(2) != 0 {
                coeff.0[j] = rnd.pseudo_uniform(1 << msb);
                dqcoeff.0[j] = rnd.pseudo_uniform(1 << msb);
            } else {
                coeff.0[j] = -rnd.pseudo_uniform(1 << msb);
                dqcoeff.0[j] = -rnd.pseudo_uniform(1 << msb);
            }
        }
        let (mut ssz, mut ref_ssz) = (0i64, 0i64);
        // SAFETY: both buffers hold block_size coefficients.
        let ref_ret = unsafe { ref_error_block_op(coeff.0.as_ptr(), dqcoeff.0.as_ptr(), block_size,
                                                  &mut ref_ssz, bit_depth) };
        let ret = unsafe { error_block_op(coeff.0.as_ptr(), dqcoeff.0.as_ptr(), block_size,
                                          &mut ssz, bit_depth) };
        let err_count = ((ref_ret != ret) | (ref_ssz != ssz)) as i32;
        if err_count != 0 && err_count_total == 0 {
            first_failure = i;
        }
        err_count_total += err_count;
    }
    assert_eq!(0, err_count_total,
               "Error: Error Block Test, C output doesn't match optimized output. First failed at test case {first_failure}");
}

fn extreme_values(error_block_op: ErrorBlockFunc, ref_error_block_op: ErrorBlockFunc, bit_depth: i32) {
    let mut rnd = ACMRandom::new();
    let mut coeff = Buffer([0; 4096]);
    let mut dqcoeff = Buffer([0; 4096]);
    let mut err_count_total = 0;
    let mut first_failure = -1;
    let msb = bit_depth + 8 - 1;
    let mut max_val = (1 << msb) - 1;
    for i in 0..K_NUM_ITERATIONS {
        let k = (i / 9) % 9;
        if k == 8 && (i % 9) == 0 {
            max_val >>= 1;
        }
        let block_size: isize = 16 << (i % 9); // All block sizes from 4x4, 8x4 ..64x64
        for j in 0..block_size as usize {
            if k < 4 {
                coeff.0[j] = if k % 2 != 0 { max_val } else { 0 };
                dqcoeff.0[j] = if (k >> 1) % 2 != 0 { max_val } else { 0 };
            } else if k < 8 {
                coeff.0[j] = if k % 2 != 0 { -max_val } else { 0 };
                dqcoeff.0[j] = if (k >> 1) % 2 != 0 { -max_val } else { 0 };
            } else if rnd.pseudo_uniform(2) != 0 {
                coeff.0[j] = rnd.pseudo_uniform(1 << 14);
                dqcoeff.0[j] = rnd.pseudo_uniform(1 << 14);
            } else {
                coeff.0[j] = -rnd.pseudo_uniform(1 << 14);
                dqcoeff.0[j] = -rnd.pseudo_uniform(1 << 14);
            }
        }
        let (mut ssz, mut ref_ssz) = (0i64, 0i64);
        // SAFETY: both buffers hold block_size coefficients.
        let ref_ret = unsafe { ref_error_block_op(coeff.0.as_ptr(), dqcoeff.0.as_ptr(), block_size,
                                                  &mut ref_ssz, bit_depth) };
        let ret = unsafe { error_block_op(coeff.0.as_ptr(), dqcoeff.0.as_ptr(), block_size,
                                          &mut ssz, bit_depth) };
        let err_count = ((ref_ret != ret) | (ref_ssz != ssz)) as i32;
        if err_count != 0 && err_count_total == 0 {
            first_failure = i;
        }
        err_count_total += err_count;
    }
    assert_eq!(0, err_count_total,
               "Error: Error Block Test, C output doesn't match optimized output. First failed at test case {first_failure}");
}

unsafe fn highbd_c(c: *const i32, d: *const i32, n: isize, ssz: *mut i64, bd: i32) -> i64 {
    av1_highbd_block_error_c(c, d, n, ssz, bd)
}
unsafe fn block_error_8bit_c(c: *const i32, d: *const i32, n: isize, ssz: *mut i64, bps: i32) -> i64 {
    assert_eq!(bps, 8);
    av1_block_error_c(c, d, n, ssz)
}
unsafe fn block_error_lp_c(c: *const i32, d: *const i32, n: isize, ssz: *mut i64, bps: i32) -> i64 {
    assert_eq!(bps, 8);
    *ssz = -1;
    av1_block_error_lp_c(c as *const i16, d as *const i16, n)
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::av1__encoder__arm__av1_error_neon::*;
    use aom::simd::av1__encoder__arm__highbd_rdopt_neon::*;

    unsafe fn highbd_neon(c: *const i32, d: *const i32, n: isize, ssz: *mut i64, bd: i32) -> i64 {
        av1_highbd_block_error_neon(c, d, n, ssz, bd)
    }
    unsafe fn block_error_8bit_neon(c: *const i32, d: *const i32, n: isize, ssz: *mut i64,
                                    bps: i32) -> i64 {
        assert_eq!(bps, 8);
        av1_block_error_neon(c, d, n, ssz)
    }
    unsafe fn block_error_lp_neon(c: *const i32, d: *const i32, n: isize, ssz: *mut i64,
                                  bps: i32) -> i64 {
        assert_eq!(bps, 8);
        *ssz = -1;
        av1_block_error_lp_neon(c as *const i16, d as *const i16, n)
    }

    // kErrorBlockTestParamsNeon
    const PARAMS: [(ErrorBlockFunc, ErrorBlockFunc, i32); 5] = [
        (highbd_neon, highbd_c, 10),
        (highbd_neon, highbd_c, 12),
        (highbd_neon, highbd_c, 8),
        (block_error_8bit_neon, block_error_8bit_c, 8),
        (block_error_lp_neon, block_error_lp_c, 8),
    ];

    #[test]
    fn operation_check_all() {
        for (op, reference, bd) in PARAMS {
            operation_check(op, reference, bd);
        }
    }

    #[test]
    fn extreme_values_all() {
        for (op, reference, bd) in PARAMS {
            extreme_values(op, reference, bd);
        }
    }
}
