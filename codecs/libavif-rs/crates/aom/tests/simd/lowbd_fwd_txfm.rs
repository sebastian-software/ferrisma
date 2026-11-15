// Supplementary transcription of upstream/aom/test/av1_fwd_txfm2d_test.cc
// (AV1FwdTxfm2dTest.match, the NEON instantiation over fwd_txfm_for_neon with
// av1_lowbd_fwd_txfm_neon, and on x86-64 the SSE4_1 and AVX2 ones over
// fwd_txfm_for_sse41 and fwd_txfm_for_avx2, all 19 sizes). The reference is fwd_txfm_func_ls[tx_size], the
// translated av1_fwd_txfm2d_*_c, and the target receives a zeroed TxfmParam
// with tx_type, tx_size, tx_set_type = EXT_TX_SET_ALL16 and bd = 8, as in the
// original. Seeds, iteration counts, input generation and the compared
// coefficient region follow AV1FwdTxfm2dMatchTest; the speed test and the
// DCT/Hadamard scale tests (which use tolerances) are not transcribed.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]

use aom::av1__common__idct::TxfmParam;
#[cfg(target_arch = "aarch64")]
use aom::simd::av1__encoder__arm__av1_fwd_txfm2d_neon::av1_lowbd_fwd_txfm_neon;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__av1_fwd_txfm2d_avx2::av1_lowbd_fwd_txfm_avx2;
#[cfg(target_arch = "x86_64")]
use aom::simd::av1__encoder__x86__av1_fwd_txfm2d_sse4::av1_lowbd_fwd_txfm_sse4_1;

use super::acm_random::ACMRandom;
use super::highbd_fwd_txfm::*;

type LowbdFwdTxfmFunc = unsafe extern "C-unwind" fn(*const i16, *mut i32, i32, *mut TxfmParam);

const TX_TYPES: usize = 16;
const EXT_TX_SET_ALL16: u8 = 5;
const tx_size_wide: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const tx_size_high: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

#[repr(C, align(32))]
struct Aligned<T>(T);

/// AV1FwdTxfm2dMatchTest; returns the number of compared coefficients.
fn av1_fwd_txfm2d_match_test(tx_size: usize, target_func: LowbdFwdTxfmFunc) -> usize {
    super::scalar_round_shift_reference();
    let bd = 8;
    // SAFETY: TxfmParam is a plain C record; zero is the memset of the original.
    let mut param: TxfmParam = unsafe { core::mem::zeroed() };
    let rows = tx_size_high[tx_size];
    let cols = tx_size_wide[tx_size];
    let mut compared = 0usize;
    for tx_type in 0..TX_TYPES {
        if !is_tx_size_type_valid(tx_size, tx_type) {
            continue;
        }
        let ref_func = fwd_txfm_func_ls[tx_size];
        let mut input = Box::new(Aligned([0i16; 64 * 64]));
        let mut output = Box::new(Aligned([0i32; 64 * 64]));
        let mut ref_output = Box::new(Aligned([0i32; 64 * 64]));
        let input_stride: i32 = 64;
        let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
        for cnt in 0..500 {
            if cnt == 0 {
                for c in 0..cols {
                    for r in 0..rows {
                        input.0[(r * input_stride + c) as usize] = ((1 << bd) - 1) as i16;
                    }
                }
            } else {
                for r in 0..rows {
                    for c in 0..cols {
                        input.0[(r * input_stride + c) as usize] =
                            (rnd.rand16() as i32 % (1 << bd)) as i16;
                    }
                }
            }
            param.tx_type = tx_type as u8;
            param.tx_size = tx_size as u8;
            param.tx_set_type = EXT_TX_SET_ALL16 as _;
            param.bd = bd;
            // SAFETY: the 64x64 buffers cover every transform size at stride 64.
            unsafe {
                ref_func(input.0.as_ptr(), ref_output.0.as_mut_ptr(), input_stride, tx_type as u8,
                         bd);
                target_func(input.0.as_ptr(), output.0.as_mut_ptr(), input_stride, &mut param);
            }
            let check_cols = 32.min(cols);
            let check_rows = 32.min(rows * cols / check_cols);
            for r in 0..check_rows {
                for c in 0..check_cols {
                    let k = (r * check_cols + c) as usize;
                    assert_eq!(ref_output.0[k], output.0[k],
                               "[{r},{c}] cnt:{cnt} tx_size: {cols}x{rows} tx_type: {tx_type}");
                    compared += 1;
                }
            }
        }
    }
    compared
}

macro_rules! match_tests {
    ($arch:literal, $feature:tt, $func:ident; $($test:ident: $tx_size:expr;)*) => {
        $(
            #[cfg(target_arch = $arch)]
            #[test]
            fn $test() {
                #[cfg(target_arch = "x86_64")]
                if !std::is_x86_feature_detected!($feature) {
                    return;
                }
                let compared = av1_fwd_txfm2d_match_test($tx_size, $func);
                assert!(compared > 0);
            }
        )*
    };
}

// fwd_txfm_for_neon
match_tests! {
    "aarch64", "neon", av1_lowbd_fwd_txfm_neon;
    lowbd_fwd_txfm2d_match_4x4: TX_4X4;
    lowbd_fwd_txfm2d_match_8x8: TX_8X8;
    lowbd_fwd_txfm2d_match_16x16: TX_16X16;
    lowbd_fwd_txfm2d_match_32x32: TX_32X32;
    lowbd_fwd_txfm2d_match_64x64: TX_64X64;
    lowbd_fwd_txfm2d_match_4x8: TX_4X8;
    lowbd_fwd_txfm2d_match_8x4: TX_8X4;
    lowbd_fwd_txfm2d_match_8x16: TX_8X16;
    lowbd_fwd_txfm2d_match_16x8: TX_16X8;
    lowbd_fwd_txfm2d_match_16x32: TX_16X32;
    lowbd_fwd_txfm2d_match_32x16: TX_32X16;
    lowbd_fwd_txfm2d_match_32x64: TX_32X64;
    lowbd_fwd_txfm2d_match_64x32: TX_64X32;
    lowbd_fwd_txfm2d_match_4x16: TX_4X16;
    lowbd_fwd_txfm2d_match_16x4: TX_16X4;
    lowbd_fwd_txfm2d_match_8x32: TX_8X32;
    lowbd_fwd_txfm2d_match_32x8: TX_32X8;
    lowbd_fwd_txfm2d_match_16x64: TX_16X64;
    lowbd_fwd_txfm2d_match_64x16: TX_64X16;
}

// fwd_txfm_for_sse41
match_tests! {
    "x86_64", "sse4.1", av1_lowbd_fwd_txfm_sse4_1;
    lowbd_fwd_txfm2d_sse4_1_match_4x4: TX_4X4;
    lowbd_fwd_txfm2d_sse4_1_match_8x8: TX_8X8;
    lowbd_fwd_txfm2d_sse4_1_match_16x16: TX_16X16;
    lowbd_fwd_txfm2d_sse4_1_match_32x32: TX_32X32;
    lowbd_fwd_txfm2d_sse4_1_match_64x64: TX_64X64;
    lowbd_fwd_txfm2d_sse4_1_match_4x8: TX_4X8;
    lowbd_fwd_txfm2d_sse4_1_match_8x4: TX_8X4;
    lowbd_fwd_txfm2d_sse4_1_match_8x16: TX_8X16;
    lowbd_fwd_txfm2d_sse4_1_match_16x8: TX_16X8;
    lowbd_fwd_txfm2d_sse4_1_match_16x32: TX_16X32;
    lowbd_fwd_txfm2d_sse4_1_match_32x16: TX_32X16;
    lowbd_fwd_txfm2d_sse4_1_match_32x64: TX_32X64;
    lowbd_fwd_txfm2d_sse4_1_match_64x32: TX_64X32;
    lowbd_fwd_txfm2d_sse4_1_match_4x16: TX_4X16;
    lowbd_fwd_txfm2d_sse4_1_match_16x4: TX_16X4;
    lowbd_fwd_txfm2d_sse4_1_match_8x32: TX_8X32;
    lowbd_fwd_txfm2d_sse4_1_match_32x8: TX_32X8;
    lowbd_fwd_txfm2d_sse4_1_match_16x64: TX_16X64;
    lowbd_fwd_txfm2d_sse4_1_match_64x16: TX_64X16;
}

// fwd_txfm_for_avx2
match_tests! {
    "x86_64", "avx2", av1_lowbd_fwd_txfm_avx2;
    lowbd_fwd_txfm2d_avx2_match_4x4: TX_4X4;
    lowbd_fwd_txfm2d_avx2_match_8x8: TX_8X8;
    lowbd_fwd_txfm2d_avx2_match_16x16: TX_16X16;
    lowbd_fwd_txfm2d_avx2_match_32x32: TX_32X32;
    lowbd_fwd_txfm2d_avx2_match_64x64: TX_64X64;
    lowbd_fwd_txfm2d_avx2_match_4x8: TX_4X8;
    lowbd_fwd_txfm2d_avx2_match_8x4: TX_8X4;
    lowbd_fwd_txfm2d_avx2_match_8x16: TX_8X16;
    lowbd_fwd_txfm2d_avx2_match_16x8: TX_16X8;
    lowbd_fwd_txfm2d_avx2_match_16x32: TX_16X32;
    lowbd_fwd_txfm2d_avx2_match_32x16: TX_32X16;
    lowbd_fwd_txfm2d_avx2_match_32x64: TX_32X64;
    lowbd_fwd_txfm2d_avx2_match_64x32: TX_64X32;
    lowbd_fwd_txfm2d_avx2_match_4x16: TX_4X16;
    lowbd_fwd_txfm2d_avx2_match_16x4: TX_16X4;
    lowbd_fwd_txfm2d_avx2_match_8x32: TX_8X32;
    lowbd_fwd_txfm2d_avx2_match_32x8: TX_32X8;
    lowbd_fwd_txfm2d_avx2_match_16x64: TX_16X64;
    lowbd_fwd_txfm2d_avx2_match_64x16: TX_64X16;
}
