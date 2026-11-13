// Supplementary transcription of upstream/aom/test/sse_sum_test.cc for the NEON
// instantiation: SumSSETest (aom_sum_sse_2d_i16_neon against
// aom_sum_sse_2d_i16_c: OperationCheck, ExtremeValues,
// ArbitraryWidthAndHeight). Each original TEST_P starts from a fresh fixture
// seeded with ACMRandom::DeterministicSeed(); the buffer size, strides, value
// ranges, iteration count and the exact-equality checks of the SSE and the
// sum follow the original. The speed test is not transcribed. C++ evaluates
// `rnd_(2) ? rnd_(limit) : -rnd_(limit)` and the other draws in source order,
// as transcribed here.
#![cfg(target_arch = "aarch64")]

use aom::simd::aom_dsp__arm__sum_squares_neon::aom_sum_sse_2d_i16_neon;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn aom_sum_sse_2d_i16_c(src: *const i16, src_stride: i32, width: i32, height: i32,
                            sum: *mut i32) -> u64;
}

type SSI16Func = unsafe extern "C-unwind" fn(*const i16, i32, i32, i32, *mut i32) -> u64;

const K_NUM_ITERATIONS: usize = 10000;

fn run_test(ref_func: SSI16Func, tst_func: SSI16Func, is_random: bool, multiple_of_4: bool) {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    // aom_memalign(16, 256 * 256 * 2), left uninitialized by the original;
    // every position a call reads is written first.
    let mut src = vec![0i16; 256 * 256];
    for _ in 0..K_NUM_ITERATIONS {
        // Up to 128x128
        let width = if multiple_of_4 { 4 * (rnd.pseudo_uniform(31) + 1) } else {
            rnd.pseudo_uniform(127) + 1
        };
        // Up to 128x128
        let height = if multiple_of_4 { 4 * (rnd.pseudo_uniform(31) + 1) } else {
            rnd.pseudo_uniform(127) + 1
        };
        let mut stride = 4 << rnd.pseudo_uniform(7); // Up to 256 stride
        while stride < width {
            // Make sure it's valid
            stride = 4 << rnd.pseudo_uniform(7);
        }
        let msb = 11; // Up to 12 bit input
        let limit = 1 << (msb + 1);
        if is_random {
            // GenRandomData
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
            // GenExtremeData
            let val = if rnd.pseudo_uniform(2) != 0 { limit - 1 } else { -(limit - 1) };
            for ii in 0..height {
                for jj in 0..width {
                    src[(ii * stride + jj) as usize] = val as i16;
                }
            }
        }
        let (mut sum_ref, mut sum_tst) = (0i32, 0i32);
        // SAFETY: the 256x256 buffer covers `height` rows at `stride`.
        let sse_ref = unsafe { ref_func(src.as_ptr(), stride, width, height, &mut sum_ref) };
        // SAFETY: as above.
        let sse_tst = unsafe { tst_func(src.as_ptr(), stride, width, height, &mut sum_tst) };
        assert_eq!(sse_ref, sse_tst,
                   "Error: SumSSETest [{width}x{height}] C SSE does not match optimized output.");
        assert_eq!(sum_ref, sum_tst,
                   "Error: SumSSETest [{width}x{height}] C Sum does not match optimized output.");
    }
}

#[test]
fn sum_sse_2d_i16_operation_check() {
    // GenRandomData, width and height multiple of 4
    run_test(aom_sum_sse_2d_i16_c, aom_sum_sse_2d_i16_neon, true, true);
}

#[test]
fn sum_sse_2d_i16_extreme_values() {
    // GenExtremeData, width and height multiple of 4
    run_test(aom_sum_sse_2d_i16_c, aom_sum_sse_2d_i16_neon, false, true);
}

#[test]
fn sum_sse_2d_i16_arbitrary_width_and_height() {
    // GenRandomData, width and height not necessarily multiple of 4
    run_test(aom_sum_sse_2d_i16_c, aom_sum_sse_2d_i16_neon, true, false);
}
