// Supplementary transcription of upstream/aom/test/sum_squares_test.cc
// (SSE_Sum_Test, NEON instantiation aom_get_blk_sse_sum_neon over
// kValidBlockSize: OperationCheck and ExtremeValues). Each parameter starts
// from a fresh fixture seeded with ACMRandom::DeterministicSeed(); strides,
// value ranges and the exact checks follow the original. The speed test is
// not transcribed.
#![cfg(target_arch = "aarch64")]

use aom::simd::aom_dsp__arm__blk_sse_sum_neon::aom_get_blk_sse_sum_neon;

use super::acm_random::ACMRandom;

extern "C-unwind" {
    fn aom_get_blk_sse_sum_c(data: *const i16, stride: i32, bw: i32, bh: i32, x_sum: *mut i32,
                             x2_sum: *mut i64);
}

type SseSumFunc = unsafe extern "C-unwind" fn(*const i16, i32, i32, i32, *mut i32, *mut i64);

#[repr(C, align(32))]
struct Aligned<T>(T);

const TX_SIZE_WIDE: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const TX_SIZE_HIGH: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
// kValidBlockSize as TX_SIZE values (av1/common/enums.h): TX_4X4 .. TX_32X32,
// TX_64X64, TX_4X8, TX_8X4, TX_8X16, TX_16X8, TX_16X32, TX_32X16, TX_64X32,
// TX_32X64, TX_4X16, TX_16X4, TX_8X32, TX_32X8, TX_16X64, TX_64X16.
const K_VALID_BLOCK_SIZE: [usize; 19] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 11, 13, 14, 15, 16,
                                         17, 18];

fn sse_sum_run_test(ref_func: SseSumFunc, tst_func: SseSumFunc, is_random: bool, tx_size: usize) {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = Box::new(Aligned([0i16; 256 * 256]));
    let width = TX_SIZE_WIDE[tx_size];
    let height = TX_SIZE_HIGH[tx_size];
    for k in 0..3 {
        let mut stride = 4 << rnd.pseudo_uniform(7); // Up to 256 stride
        while stride < width {
            stride = 4 << rnd.pseudo_uniform(7);
        }
        if is_random {
            let limit = 1 << 12;
            for ii in 0..height {
                for jj in 0..width {
                    src.0[(ii * stride + jj) as usize] = rnd.pseudo_uniform(limit) as i16;
                }
            }
        } else {
            let limit: i16 = (1 << 12) - 1;
            let val = if k == 0 { limit } else { -limit };
            for ii in 0..height {
                for jj in 0..width {
                    src.0[(ii * stride + jj) as usize] = val;
                }
            }
        }
        let (mut sum_c, mut sse_c, mut sum_intr, mut sse_intr) = (0i32, 0i64, 0i32, 0i64);
        // SAFETY: the buffer covers height rows of width samples at `stride`.
        unsafe {
            ref_func(src.0.as_ptr(), stride, width, height, &mut sum_c, &mut sse_c);
            tst_func(src.0.as_ptr(), stride, width, height, &mut sum_intr, &mut sse_intr);
        }
        assert_eq!(sum_c, sum_intr,
                   "Error:{k} SSE Sum Test [{width}x{height}] C output does not match optimized \
                    output.");
        assert_eq!(sse_c, sse_intr,
                   "Error:{k} SSE Sum Test [{width}x{height}] C output does not match optimized \
                    output.");
    }
}

#[test]
fn get_blk_sse_sum_operation_check() {
    for tx_size in K_VALID_BLOCK_SIZE {
        sse_sum_run_test(aom_get_blk_sse_sum_c, aom_get_blk_sse_sum_neon, true, tx_size);
    }
}

#[test]
fn get_blk_sse_sum_extreme_values() {
    for tx_size in K_VALID_BLOCK_SIZE {
        sse_sum_run_test(aom_get_blk_sse_sum_c, aom_get_blk_sse_sum_neon, false, tx_size);
    }
}
