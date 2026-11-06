// Supplementary transcription of upstream/aom/test/av1_fwd_txfm2d_test.cc
// (AV1HighbdFwdTxfm2dTest.match, NEON instantiation over
// Highbd_fwd_txfm_for_neon). The original target is av1_highbd_fwd_txfm()
// with a zeroed TxfmParam (lossless = 0), which for every listed size calls
// the RTCD av1_fwd_txfm2d_<W>x<H>(input, output, stride, tx_type, bd); the
// 16x64, 64x16, and 64x64 wrappers pass DCT_DCT, the only type
// IsTxSizeTypeValid() admits for them. This transcription therefore calls the
// Neon kernel directly where the original reached it through RTCD, and the
// reference is fwd_txfm_func_ls[tx_size], the translated av1_fwd_txfm2d_*_c.
// Bit depths, tx types, seeds, iteration counts, input generation, and the
// compared coefficient region follow the original; the speed test is omitted.

use super::acm_random::ACMRandom;

pub type FwdTxfm2dFunc = unsafe extern "C-unwind" fn(*const i16, *mut i32, i32, u8, i32);

// TX_SIZE values (av1/common/enums.h).
pub const TX_4X4: usize = 0;
pub const TX_8X8: usize = 1;
pub const TX_16X16: usize = 2;
pub const TX_32X32: usize = 3;
pub const TX_64X64: usize = 4;
pub const TX_4X8: usize = 5;
pub const TX_8X4: usize = 6;
pub const TX_8X16: usize = 7;
pub const TX_16X8: usize = 8;
pub const TX_16X32: usize = 9;
pub const TX_32X16: usize = 10;
pub const TX_32X64: usize = 11;
pub const TX_64X32: usize = 12;
pub const TX_4X16: usize = 13;
pub const TX_16X4: usize = 14;
pub const TX_8X32: usize = 15;
pub const TX_32X8: usize = 16;
pub const TX_16X64: usize = 17;
pub const TX_64X16: usize = 18;

const TX_TYPES: usize = 16;

// tx_size_wide / tx_size_high (av1/common/common_data.h).
const tx_size_wide: [i32; 19] = [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const tx_size_high: [i32; 19] = [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];

// txsize_sqr_up_map (av1/common/common_data.h).
const txsize_sqr_up_map: [usize; 19] = [
    TX_4X4, TX_8X8, TX_16X16, TX_32X32, TX_64X64, TX_8X8, TX_8X8, TX_16X16, TX_16X16, TX_32X32,
    TX_32X32, TX_64X64, TX_64X64, TX_16X16, TX_16X16, TX_32X32, TX_32X32, TX_64X64, TX_64X64,
];

// TxSetType values (aom_dsp/txfm_common.h).
const EXT_TX_SET_DCTONLY: usize = 0;
const EXT_TX_SET_DCT_IDTX: usize = 1;
const EXT_TX_SET_ALL16: usize = 5;

// av1_ext_tx_used (av1/common/blockd.h).
const av1_ext_tx_used: [[i32; TX_TYPES]; 6] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

// libaom_test::IsTxSizeTypeValid (test/av1_txfm_test.h).
fn is_tx_size_type_valid(tx_size: usize, tx_type: usize) -> bool {
    let tx_size_sqr_up = txsize_sqr_up_map[tx_size];
    let tx_set_type = if tx_size_sqr_up > TX_32X32 {
        EXT_TX_SET_DCTONLY
    } else if tx_size_sqr_up == TX_32X32 {
        EXT_TX_SET_DCT_IDTX
    } else {
        EXT_TX_SET_ALL16
    };
    av1_ext_tx_used[tx_set_type][tx_type] != 0
}

extern "C-unwind" {
    fn av1_fwd_txfm2d_4x4_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8, bd: i32);
    fn av1_fwd_txfm2d_8x8_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8, bd: i32);
    fn av1_fwd_txfm2d_16x16_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_32x32_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_64x64_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_4x8_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8, bd: i32);
    fn av1_fwd_txfm2d_8x4_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8, bd: i32);
    fn av1_fwd_txfm2d_8x16_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                             bd: i32);
    fn av1_fwd_txfm2d_16x8_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                             bd: i32);
    fn av1_fwd_txfm2d_16x32_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_32x16_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_32x64_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_64x32_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_4x16_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                             bd: i32);
    fn av1_fwd_txfm2d_16x4_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                             bd: i32);
    fn av1_fwd_txfm2d_8x32_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                             bd: i32);
    fn av1_fwd_txfm2d_32x8_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                             bd: i32);
    fn av1_fwd_txfm2d_16x64_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
    fn av1_fwd_txfm2d_64x16_c(input: *const i16, output: *mut i32, stride: i32, tx_type: u8,
                              bd: i32);
}

// libaom_test::fwd_txfm_func_ls (test/av1_txfm_test.h, !CONFIG_REALTIME_ONLY).
pub const fwd_txfm_func_ls: [FwdTxfm2dFunc; 19] = [
    av1_fwd_txfm2d_4x4_c, av1_fwd_txfm2d_8x8_c, av1_fwd_txfm2d_16x16_c, av1_fwd_txfm2d_32x32_c,
    av1_fwd_txfm2d_64x64_c, av1_fwd_txfm2d_4x8_c, av1_fwd_txfm2d_8x4_c, av1_fwd_txfm2d_8x16_c,
    av1_fwd_txfm2d_16x8_c, av1_fwd_txfm2d_16x32_c, av1_fwd_txfm2d_32x16_c, av1_fwd_txfm2d_32x64_c,
    av1_fwd_txfm2d_64x32_c, av1_fwd_txfm2d_4x16_c, av1_fwd_txfm2d_16x4_c, av1_fwd_txfm2d_8x32_c,
    av1_fwd_txfm2d_32x8_c, av1_fwd_txfm2d_16x64_c, av1_fwd_txfm2d_64x16_c,
];

#[repr(C, align(32))]
struct Aligned<T>(T);

/// AV1HighbdFwdTxfm2dMatchTest; returns the number of compared coefficients.
pub fn av1_highbd_fwd_txfm2d_match_test(tx_size: usize, target_func: FwdTxfm2dFunc) -> usize {
    let bd_ar: [i32; 2] = [10, 12];
    let rows = tx_size_high[tx_size];
    let cols = tx_size_wide[tx_size];
    let mut compared = 0usize;
    for i in 0..2 {
        let bd = bd_ar[i];
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
                    for r in 0..rows {
                        for c in 0..cols {
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

                // SAFETY: the 64x64 input and output buffers cover every
                // transform size with stride 64.
                unsafe {
                    ref_func(input.0.as_ptr(), ref_output.0.as_mut_ptr(), input_stride,
                             tx_type as u8, bd);
                    target_func(input.0.as_ptr(), output.0.as_mut_ptr(), input_stride,
                                tx_type as u8, bd);
                }
                let check_cols = 32.min(cols);
                let check_rows = 32.min(rows * cols / check_cols);
                for r in 0..check_rows {
                    for c in 0..check_cols {
                        let k = (c * check_rows + r) as usize;
                        assert_eq!(ref_output.0[k], output.0[k],
                                   "[{r},{c}] cnt:{cnt} tx_size: {cols}x{rows} tx_type: {tx_type} \
                                    bd: {bd}");
                        compared += 1;
                    }
                }
            }
        }
    }
    compared
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::av1__encoder__arm__highbd_fwd_txfm_neon::*;

    fn run(tx_size: usize, target_func: FwdTxfm2dFunc) {
        let compared = av1_highbd_fwd_txfm2d_match_test(tx_size, target_func);
        assert!(compared > 0);
        println!("tx_size {tx_size}: {compared} coefficients compared");
    }

    macro_rules! match_tests {
        ($($test:ident: $tx_size:expr, $func:ident;)*) => {
            $(
                #[test]
                fn $test() {
                    run($tx_size, $func);
                }
            )*
        };
    }

    // Highbd_fwd_txfm_for_neon (CONFIG_REALTIME_ONLY is 0).
    match_tests! {
        highbd_fwd_txfm2d_match_4x4: TX_4X4, av1_fwd_txfm2d_4x4_neon;
        highbd_fwd_txfm2d_match_8x8: TX_8X8, av1_fwd_txfm2d_8x8_neon;
        highbd_fwd_txfm2d_match_16x16: TX_16X16, av1_fwd_txfm2d_16x16_neon;
        highbd_fwd_txfm2d_match_32x32: TX_32X32, av1_fwd_txfm2d_32x32_neon;
        highbd_fwd_txfm2d_match_64x64: TX_64X64, av1_fwd_txfm2d_64x64_neon;
        highbd_fwd_txfm2d_match_4x8: TX_4X8, av1_fwd_txfm2d_4x8_neon;
        highbd_fwd_txfm2d_match_8x4: TX_8X4, av1_fwd_txfm2d_8x4_neon;
        highbd_fwd_txfm2d_match_8x16: TX_8X16, av1_fwd_txfm2d_8x16_neon;
        highbd_fwd_txfm2d_match_16x8: TX_16X8, av1_fwd_txfm2d_16x8_neon;
        highbd_fwd_txfm2d_match_16x32: TX_16X32, av1_fwd_txfm2d_16x32_neon;
        highbd_fwd_txfm2d_match_32x16: TX_32X16, av1_fwd_txfm2d_32x16_neon;
        highbd_fwd_txfm2d_match_32x64: TX_32X64, av1_fwd_txfm2d_32x64_neon;
        highbd_fwd_txfm2d_match_64x32: TX_64X32, av1_fwd_txfm2d_64x32_neon;
        highbd_fwd_txfm2d_match_4x16: TX_4X16, av1_fwd_txfm2d_4x16_neon;
        highbd_fwd_txfm2d_match_16x4: TX_16X4, av1_fwd_txfm2d_16x4_neon;
        highbd_fwd_txfm2d_match_8x32: TX_8X32, av1_fwd_txfm2d_8x32_neon;
        highbd_fwd_txfm2d_match_32x8: TX_32X8, av1_fwd_txfm2d_32x8_neon;
        highbd_fwd_txfm2d_match_16x64: TX_16X64, av1_fwd_txfm2d_16x64_neon;
        highbd_fwd_txfm2d_match_64x16: TX_64X16, av1_fwd_txfm2d_64x16_neon;
    }
}
