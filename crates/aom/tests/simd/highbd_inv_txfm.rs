// Supplementary transcription of upstream/aom/test/av1_highbd_iht_test.cc
// (AV1HighbdInvTxfm2d: `match` and `gt_int16`) for the NEON instantiation
// av1_highbd_inv_txfm_add_neon. Inputs come from the forward transforms in
// libaom_test::fwd_txfm_func_ls (test/av1_txfm_test.h), with the original
// seed, block layout, eob truncation, and exact-match assertions. No original
// test instantiates the av1_inv_txfm2d_add_<W>x<H>_neon kernels directly.
use aom::av1__common__idct::TxfmParam;
use aom::av1__common__scan::av1_scan_orders;

use super::acm_random::ACMRandom;

type FwdTxfm2dFunc = unsafe extern "C-unwind" fn(*const i16, *mut i32, i32, u8, i32);
type HighbdInvTxfm2dFunc = unsafe extern "C-unwind" fn(*const i32, *mut u8, i32, *const TxfmParam);

extern "C-unwind" {
    fn av1_highbd_inv_txfm_add_c(input: *const i32, dest: *mut u8, stride: i32,
                                 txfm_param: *const TxfmParam);
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

const TX_SIZES_ALL: usize = 19;
const TX_TYPES: usize = 16;
const TX_32X32: u8 = 3;
const EXT_TX_SET_DCTONLY: usize = 0;
const EXT_TX_SET_DCT_IDTX: usize = 1;
const EXT_TX_SET_ALL16: u8 = 5;

const DCT_DCT: u8 = 0;
const ADST_DCT: u8 = 1;
const FLIPADST_DCT: u8 = 4;
const IDTX: u8 = 9;
const V_DCT: u8 = 10;
const H_DCT: u8 = 11;
const H_ADST: u8 = 13;
const H_FLIPADST: u8 = 15;

const TX_TYPE_NAME: [&str; TX_TYPES] = [
    "DCT_DCT", "ADST_DCT", "DCT_ADST", "ADST_ADST", "FLIPADST_DCT", "DCT_FLIPADST",
    "FLIPADST_FLIPADST", "ADST_FLIPADST", "FLIPADST_ADST", "IDTX", "V_DCT", "H_DCT", "V_ADST",
    "H_ADST", "V_FLIPADST", "H_FLIPADST",
];

// av1/common/common_data.h
const TX_SIZE_WIDE: [i32; TX_SIZES_ALL] =
    [4, 8, 16, 32, 64, 4, 8, 8, 16, 16, 32, 32, 64, 4, 16, 8, 32, 16, 64];
const TX_SIZE_HIGH: [i32; TX_SIZES_ALL] =
    [4, 8, 16, 32, 64, 8, 4, 16, 8, 32, 16, 64, 32, 16, 4, 32, 8, 64, 16];
const TXSIZE_SQR_UP_MAP: [u8; TX_SIZES_ALL] =
    [0, 1, 2, 3, 4, 1, 1, 2, 2, 3, 3, 4, 4, 2, 2, 3, 3, 4, 4];

// av1/common/blockd.h
const AV1_EXT_TX_USED: [[i32; TX_TYPES]; 6] = [
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

/// libaom_test::IsTxSizeTypeValid (test/av1_txfm_test.h).
fn is_tx_size_type_valid(tx_size: u8, tx_type: u8) -> bool {
    let tx_size_sqr_up = TXSIZE_SQR_UP_MAP[tx_size as usize];
    let tx_set_type = if tx_size_sqr_up > TX_32X32 {
        EXT_TX_SET_DCTONLY
    } else if tx_size_sqr_up == TX_32X32 {
        EXT_TX_SET_DCT_IDTX
    } else {
        EXT_TX_SET_ALL16 as usize
    };
    AV1_EXT_TX_USED[tx_set_type][tx_type as usize] != 0
}

/// libaom_test::fwd_txfm_func_ls (CONFIG_AV1_ENCODER, !CONFIG_REALTIME_ONLY).
const FWD_TXFM_FUNC_LS: [FwdTxfm2dFunc; TX_SIZES_ALL] = [
    av1_fwd_txfm2d_4x4_c, av1_fwd_txfm2d_8x8_c, av1_fwd_txfm2d_16x16_c, av1_fwd_txfm2d_32x32_c,
    av1_fwd_txfm2d_64x64_c, av1_fwd_txfm2d_4x8_c, av1_fwd_txfm2d_8x4_c, av1_fwd_txfm2d_8x16_c,
    av1_fwd_txfm2d_16x8_c, av1_fwd_txfm2d_16x32_c, av1_fwd_txfm2d_32x16_c, av1_fwd_txfm2d_32x64_c,
    av1_fwd_txfm2d_64x32_c, av1_fwd_txfm2d_4x16_c, av1_fwd_txfm2d_16x4_c, av1_fwd_txfm2d_8x32_c,
    av1_fwd_txfm2d_32x8_c, av1_fwd_txfm2d_16x64_c, av1_fwd_txfm2d_64x16_c,
];

/// CONVERT_TO_BYTEPTR (aom_ports/mem.h).
fn convert_to_byteptr(p: *mut u16) -> *mut u8 {
    ((p as usize) >> 1) as *mut u8
}

#[repr(C, align(32))]
struct Aligned<T>(T);

const BLK_WIDTH: usize = 64;
const BLK_SIZE: usize = BLK_WIDTH * BLK_WIDTH;

/// AV1HighbdInvTxfm2d::RunAV1InvTxfm2dTest; returns the number of compared
/// blocks.
fn run_av1_inv_txfm2d_test(target_func: HighbdInvTxfm2dFunc, tx_type: u8, tx_size: u8,
                           run_times: i32, bit_depth: i32, gt_int16: bool) -> usize {
    let fwd_func = FWD_TXFM_FUNC_LS[tx_size as usize];
    let mut input = Aligned([0i16; BLK_SIZE]);
    let mut inv_input = Aligned([0i32; BLK_SIZE]);
    let mut output = Aligned([0u16; BLK_SIZE]);
    let mut ref_output = Aligned([0u16; BLK_SIZE]);
    let stride = BLK_WIDTH as i32;
    let rows = TX_SIZE_HIGH[tx_size as usize];
    let cols = TX_SIZE_WIDE[tx_size as usize];
    let rows_nonezero = rows.min(32);
    let cols_nonezero = cols.min(32);
    let mask = ((1i32 << bit_depth) - 1) as u16;
    let run_times = (run_times / (rows * cols)).max(1);
    // SAFETY: av1_scan_orders is an immutable table after initialization.
    let scan = unsafe { av1_scan_orders[tx_size as usize][tx_type as usize].scan };
    let eobmax = rows_nonezero * cols_nonezero;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let rand_times = if run_times == 1 { eobmax } else { 1 };
    assert_eq!(run_times, 1, "only the exact-match configuration is transcribed");

    let mut txfm_param = TxfmParam {
        tx_type,
        tx_size,
        lossless: 0,
        bd: bit_depth,
        is_hbd: 1,
        tx_set_type: EXT_TX_SET_ALL16,
        eob: 0,
    };

    let mut compared = 0;
    for cnt in 0..rand_times {
        for r in 0..BLK_WIDTH {
            for c in 0..BLK_WIDTH {
                let a = (rnd.rand16() & mask) as i32;
                let b = (rnd.rand16() & mask) as i32;
                input.0[r * cols as usize + c] = (a - b) as i16;
                output.0[r * BLK_WIDTH + c] = rnd.rand16() & mask;
                ref_output.0[r * BLK_WIDTH + c] = output.0[r * BLK_WIDTH + c];
            }
        }
        // SAFETY: 64x64 input and output blocks cover every transform size.
        unsafe { fwd_func(input.0.as_ptr(), inv_input.0.as_mut_ptr(), stride, tx_type, bit_depth) };

        // produce eob input by setting high freq coeffs to zero
        let eob = (cnt + 1).min(eobmax);
        for i in eob..eobmax {
            // SAFETY: the scan table has eobmax entries for this size.
            inv_input.0[unsafe { *scan.add(i as usize) } as usize] = 0;
        }
        txfm_param.eob = eob;
        if gt_int16 {
            let inv_input_mask = ((1i32 << (bit_depth + 7)) - 1) as u16;
            for i in 0..eob {
                // SAFETY: as above.
                inv_input.0[unsafe { *scan.add(i as usize) } as usize] =
                    (rnd.rand31() & inv_input_mask as u32) as i32;
            }
        }

        // SAFETY: both destinations are 64x64 u16 blocks with stride 64,
        // passed in libaom's CONVERT_TO_BYTEPTR form.
        unsafe {
            av1_highbd_inv_txfm_add_c(inv_input.0.as_ptr(),
                                      convert_to_byteptr(ref_output.0.as_mut_ptr()), stride,
                                      &txfm_param);
            target_func(inv_input.0.as_ptr(), convert_to_byteptr(output.0.as_mut_ptr()), stride,
                        &txfm_param);
        }
        for r in 0..rows as usize {
            for c in 0..cols as usize {
                assert_eq!(ref_output.0[r * BLK_WIDTH + c], output.0[r * BLK_WIDTH + c],
                           "[{r},{c}] {cnt} tx_size: {cols}x{rows} bit_depth_: {bit_depth} \
                            tx_type: {} eob {eob}",
                           TX_TYPE_NAME[tx_type as usize]);
            }
        }
        compared += 1;
    }
    compared
}

/// TEST_P(AV1HighbdInvTxfm2d, match)
fn av1_highbd_inv_txfm2d_match(target_func: HighbdInvTxfm2dFunc) -> usize {
    let bitdepth_ar = [8, 10, 12];
    let mut compared = 0;
    for bd in bitdepth_ar {
        for j in 0..TX_SIZES_ALL as u8 {
            for i in 0..TX_TYPES as u8 {
                if is_tx_size_type_valid(j, i) {
                    compared += run_av1_inv_txfm2d_test(target_func, i, j, 1, bd, false);
                }
            }
        }
    }
    compared
}

/// TEST_P(AV1HighbdInvTxfm2d, gt_int16)
fn av1_highbd_inv_txfm2d_gt_int16(target_func: HighbdInvTxfm2dFunc) -> usize {
    let bitdepth_ar = [8, 10, 12];
    let types = [DCT_DCT, ADST_DCT, FLIPADST_DCT, IDTX, V_DCT, H_DCT, H_ADST, H_FLIPADST];
    let mut compared = 0;
    for bd in bitdepth_ar {
        for j in 0..TX_SIZES_ALL as u8 {
            for tp in types {
                if is_tx_size_type_valid(j, tp) {
                    compared += run_av1_inv_txfm2d_test(target_func, tp, j, 1, bd, true);
                }
            }
        }
    }
    compared
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::av1__common__arm__highbd_inv_txfm_neon::av1_highbd_inv_txfm_add_neon;

    #[test]
    fn highbd_inv_txfm2d_match() {
        let compared = av1_highbd_inv_txfm2d_match(av1_highbd_inv_txfm_add_neon);
        eprintln!("AV1HighbdInvTxfm2d.match: {compared} blocks compared");
    }

    #[test]
    fn highbd_inv_txfm2d_gt_int16() {
        let compared = av1_highbd_inv_txfm2d_gt_int16(av1_highbd_inv_txfm_add_neon);
        eprintln!("AV1HighbdInvTxfm2d.gt_int16: {compared} blocks compared");
    }
}
