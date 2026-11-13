// Supplementary transcription of upstream/aom/test/fwht4x4_test.cc
// (Trans4x4WHT, NEON instantiation: av1_fwht4x4_neon at AOM_BITS_10 and
// AOM_BITS_12) with the exact-match checks of test/transform_test_base.h:
// CoeffCheck (RunCoeffCheck) and MemCheck (RunMemCheck), against
// fwht4x4_ref, i.e. the translated av1_fwht4x4_c. AccuracyCheck and
// InvAccuracyCheck are round-trip error bounds through the inverse
// transform, not a comparison with _c, and are not transcribed; the
// coefficient-range EXPECT of MemCheck is kept.
#![cfg(target_arch = "aarch64")]

use aom::simd::av1__encoder__arm__hybrid_fwd_txfm_neon::av1_fwht4x4_neon;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_fwht4x4_c(input: *const i16, output: *mut i32, stride: i32);
}

type FdctFunc = unsafe extern "C-unwind" fn(*const i16, *mut i32, i32);

const NUM_COEFFS: usize = 16;
const PITCH: usize = 4;
const HEIGHT: usize = 4;
// test/transform_test_base.h
const K_DCT_MAX_VALUE: i32 = 16384;

/// TransformTestBase::FindRowLength for pitch 4 and height 4.
fn find_row_length() -> i32 {
    4
}

fn run_coeff_check(fwd: FdctFunc, bit_depth: i32) {
    let mask = (1i32 << bit_depth) - 1;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let count_test_block = 5000;
    // Use a stride value which is not the width of any transform, to catch
    // cases where the transforms use the stride incorrectly.
    let stride = 96usize;
    let mut input_block = vec![0i16; stride * HEIGHT];
    let mut output_ref_block = [0i32; NUM_COEFFS];
    let mut output_block = [0i32; NUM_COEFFS];
    for i in 0..count_test_block {
        for j in 0..HEIGHT {
            for k in 0..PITCH {
                let in_idx = j * stride + k;
                let out_idx = j * PITCH + k;
                let a = rnd.rand16() as i32 & mask;
                let b = rnd.rand16() as i32 & mask;
                input_block[in_idx] = (a - b) as i16;
                // bit_depth != AOM_BITS_8
                let v = rnd.rand16() as i32 & mask;
                output_block[out_idx] = v;
                output_ref_block[out_idx] = v;
            }
        }
        // SAFETY: the input holds four rows at `stride`; the outputs hold 16
        // coefficients.
        unsafe {
            av1_fwht4x4_c(input_block.as_ptr(), output_ref_block.as_mut_ptr(), stride as i32);
            fwd(input_block.as_ptr(), output_block.as_mut_ptr(), stride as i32);
        }
        for out_idx in 0..NUM_COEFFS {
            assert_eq!(output_block[out_idx], output_ref_block[out_idx],
                       "Error: not bit-exact result at index: {out_idx} at test block: {i}");
        }
    }
}

fn run_mem_check(fwd: FdctFunc, bit_depth: i32) {
    let mask = ((1i32 << bit_depth) - 1) as i16;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let count_test_block = 5000;
    let mut input_extreme_block = [0i16; NUM_COEFFS];
    let mut output_ref_block = [0i32; NUM_COEFFS];
    let mut output_block = [0i32; NUM_COEFFS];
    for i in 0..count_test_block {
        // Initialize a test block with input range [-mask_, mask_].
        for j in 0..NUM_COEFFS {
            input_extreme_block[j] = if rnd.rand8() % 2 != 0 { mask } else { -mask };
        }
        if i == 0 {
            input_extreme_block.fill(mask);
        } else if i == 1 {
            input_extreme_block.fill(-mask);
        }
        // SAFETY: 16 inputs at pitch 4 and 16 outputs.
        unsafe {
            av1_fwht4x4_c(input_extreme_block.as_ptr(), output_ref_block.as_mut_ptr(), PITCH as i32);
            fwd(input_extreme_block.as_ptr(), output_block.as_mut_ptr(), PITCH as i32);
        }
        let row_length = find_row_length();
        for j in 0..NUM_COEFFS {
            assert_eq!(output_block[j], output_ref_block[j],
                       "Not bit-exact at test index: {i}, j = {j}");
            assert!((row_length * K_DCT_MAX_VALUE) << (bit_depth - 8) >= output_block[j].abs(),
                    "Error: NxN FDCT has coefficient larger than N*DCT_MAX_VALUE");
        }
    }
}

#[test]
fn fwht4x4_coeff_check() {
    for bd in [10, 12] {
        run_coeff_check(av1_fwht4x4_neon, bd);
    }
}

#[test]
fn fwht4x4_mem_check() {
    for bd in [10, 12] {
        run_mem_check(av1_fwht4x4_neon, bd);
    }
}
