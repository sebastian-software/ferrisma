// Supplementary transcription of upstream/aom/test/av1_round_shift_array_test.cc
// (AV1CompRoundShiftTest.CheckOutput, NEON and SSE4_1 instantiations
// av1_round_shift_array_neon and av1_round_shift_array_sse4_1 over
// txsize_to_bsize and kValidBitCheck; the SSE4_1 test skips on a CPU without
// the feature). Each
// parameter starts from a fresh fixture seeded with
// ACMRandom::DeterministicSeed(); the buffer, the input range
// (Rand31() / 16) and the exact check follow the original. The speed test is
// not transcribed.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

#[cfg(target_arch = "aarch64")]
use aom::simd::av1__common__arm__av1_txfm_neon::av1_round_shift_array_neon;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_array_sse4_1;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn av1_round_shift_array_c(arr: *mut i32, size: i32, bit: i32);
}

#[repr(C, align(32))]
struct Aligned<T>(T);

// block_size_wide[txsize_to_bsize[tx_size]] for every TX_SIZE, i.e. the
// transform widths (av1/common/common_data.h).
const TXSIZE_TO_BSIZE_WIDE: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8,
                                         32, 16, 64];
const K_VALID_BIT_CHECK: [i32; 9] = [-4, -3, -2, -1, 0, 1, 2, 3, 4];

/// AV1CompRoundShiftTest.CheckOutput for one instantiation.
fn check_output(func: unsafe extern "C-unwind" fn(*mut i32, i32, i32)) {
    for w in TXSIZE_TO_BSIZE_WIDE {
        for bit in K_VALID_BIT_CHECK {
            let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
            let blk_wd = 64;
            let mut pred = Aligned([0i32; 64]);
            let mut ref_buffer = Aligned([0i32; 64]);
            for i in 0..blk_wd {
                let v = (rnd.rand31() / 16) as i32;
                ref_buffer.0[i] = v;
                pred.0[i] = v;
            }
            // SAFETY: both buffers hold 64 >= w elements.
            unsafe {
                av1_round_shift_array_c(ref_buffer.0.as_mut_ptr(), w, bit);
                func(pred.0.as_mut_ptr(), w, bit);
            }
            for x in 0..w as usize {
                assert_eq!(ref_buffer.0[x], pred.0[x], "{w} mismatch @({x}) bit {bit}");
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn round_shift_array_check_output() {
    check_output(av1_round_shift_array_neon);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn round_shift_array_check_output_sse4_1() {
    if !std::is_x86_feature_detected!("sse4.1") {
        return;
    }
    check_output(av1_round_shift_array_sse4_1);
}
