// Supplementary transcription of upstream/aom/test/av1_inv_txfm2d_test.cc
// (AV1LbdInvTxfm2d: `match` and `gt_int16`) for the NEON instantiation
// av1_lowbd_inv_txfm2d_add_neon. Inputs come from the forward transforms in
// libaom_test::fwd_txfm_func_ls and the reference is
// libaom_test::inv_txfm_func_ls (av1_inv_txfm2d_add_<W>x<H>_c with 16-bit
// output, test/av1_txfm_test.h), with the original seed, block layout,
// 500 extra iterations, eob truncation, and exact-match assertions.
// DISABLED_Speed is not transcribed.
//
// A supplementary check, not in the original suite, runs the same inputs
// through the function the RTCD table dispatches, av1_inv_txfm_add_neon,
// against av1_inv_txfm_add_c (TxfmParam with is_hbd = 0, bd = 8), including
// the lossless 4x4 path that both route to the C implementation.
#![cfg(target_arch = "aarch64")]
use aom::av1__common__idct::TxfmParam;
use aom::av1__common__scan::av1_scan_orders;
use aom::simd::av1__common__arm__av1_inv_txfm_neon::{
    av1_inv_txfm_add_neon, av1_lowbd_inv_txfm2d_add_neon,
};

use super::acm_random::ACMRandom;

type FwdTxfm2dFunc = unsafe extern "C-unwind" fn(*const i16, *mut i32, i32, u8, i32);
type InvTxfm2dFunc = unsafe extern "C-unwind" fn(*const i32, *mut u16, i32, u8, i32);
type LbdInvTxfm2dFunc = unsafe extern "C-unwind" fn(*const i32, *mut u8, i32, u8, u8, i32);
type InvTxfmAddFunc = unsafe extern "C-unwind" fn(*const i32, *mut u8, i32, *const TxfmParam);

macro_rules! extern_txfm_pairs {
    ($(($fwd:ident, $inv:ident)),* $(,)?) => {
        extern "C-unwind" {
            $(
                fn $fwd(input: *const i16, output: *mut i32, stride: i32, tx_type: u8, bd: i32);
                fn $inv(input: *const i32, output: *mut u16, stride: i32, tx_type: u8, bd: i32);
            )*
            fn av1_highbd_inv_txfm_add_c(dqcoeff: *const i32, dst: *mut u8, stride: i32,
                                         txfm_param: *const TxfmParam);
            fn av1_inv_txfm_add_c(dqcoeff: *const i32, dst: *mut u8, stride: i32,
                                  txfm_param: *const TxfmParam);
        }
    };
}

extern_txfm_pairs!(
    (av1_fwd_txfm2d_4x4_c, av1_inv_txfm2d_add_4x4_c),
    (av1_fwd_txfm2d_8x8_c, av1_inv_txfm2d_add_8x8_c),
    (av1_fwd_txfm2d_16x16_c, av1_inv_txfm2d_add_16x16_c),
    (av1_fwd_txfm2d_32x32_c, av1_inv_txfm2d_add_32x32_c),
    (av1_fwd_txfm2d_64x64_c, av1_inv_txfm2d_add_64x64_c),
    (av1_fwd_txfm2d_4x8_c, av1_inv_txfm2d_add_4x8_c),
    (av1_fwd_txfm2d_8x4_c, av1_inv_txfm2d_add_8x4_c),
    (av1_fwd_txfm2d_8x16_c, av1_inv_txfm2d_add_8x16_c),
    (av1_fwd_txfm2d_16x8_c, av1_inv_txfm2d_add_16x8_c),
    (av1_fwd_txfm2d_16x32_c, av1_inv_txfm2d_add_16x32_c),
    (av1_fwd_txfm2d_32x16_c, av1_inv_txfm2d_add_32x16_c),
    (av1_fwd_txfm2d_32x64_c, av1_inv_txfm2d_add_32x64_c),
    (av1_fwd_txfm2d_64x32_c, av1_inv_txfm2d_add_64x32_c),
    (av1_fwd_txfm2d_4x16_c, av1_inv_txfm2d_add_4x16_c),
    (av1_fwd_txfm2d_16x4_c, av1_inv_txfm2d_add_16x4_c),
    (av1_fwd_txfm2d_8x32_c, av1_inv_txfm2d_add_8x32_c),
    (av1_fwd_txfm2d_32x8_c, av1_inv_txfm2d_add_32x8_c),
    (av1_fwd_txfm2d_16x64_c, av1_inv_txfm2d_add_16x64_c),
    (av1_fwd_txfm2d_64x16_c, av1_inv_txfm2d_add_64x16_c),
);

const TX_SIZES_ALL: usize = 19;
const TX_TYPES: usize = 16;
const TX_4X4: u8 = 0;
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

/// libaom_test::inv_txfm_func_ls.
const INV_TXFM_FUNC_LS: [InvTxfm2dFunc; TX_SIZES_ALL] = [
    av1_inv_txfm2d_add_4x4_c, av1_inv_txfm2d_add_8x8_c, av1_inv_txfm2d_add_16x16_c,
    av1_inv_txfm2d_add_32x32_c, av1_inv_txfm2d_add_64x64_c, av1_inv_txfm2d_add_4x8_c,
    av1_inv_txfm2d_add_8x4_c, av1_inv_txfm2d_add_8x16_c, av1_inv_txfm2d_add_16x8_c,
    av1_inv_txfm2d_add_16x32_c, av1_inv_txfm2d_add_32x16_c, av1_inv_txfm2d_add_32x64_c,
    av1_inv_txfm2d_add_64x32_c, av1_inv_txfm2d_add_4x16_c, av1_inv_txfm2d_add_16x4_c,
    av1_inv_txfm2d_add_8x32_c, av1_inv_txfm2d_add_32x8_c, av1_inv_txfm2d_add_16x64_c,
    av1_inv_txfm2d_add_64x16_c,
];

#[repr(C, align(32))]
struct Aligned<T>(T);

const BLK_WIDTH: usize = 64;
const BLK_SIZE: usize = BLK_WIDTH * BLK_WIDTH;

/// What the target is compared against.
#[derive(Clone, Copy)]
enum Target {
    /// The original test: av1_lowbd_inv_txfm2d_add_neon against
    /// inv_txfm_func_ls[tx_size] with 16-bit output.
    Lbd(LbdInvTxfm2dFunc),
    /// Supplementary: the dispatched entry against av1_inv_txfm_add_c.
    Add { neon: InvTxfmAddFunc, lossless: i32 },
}

/// AV1LbdInvTxfm2d::RunAV1InvTxfm2dTest with run_times = 1; returns the
/// number of compared blocks.
fn run_av1_inv_txfm2d_test(target: Target, tx_type: u8, tx_size: u8, gt_int16: bool) -> usize {
    let fwd_func = FWD_TXFM_FUNC_LS[tx_size as usize];
    let ref_func = INV_TXFM_FUNC_LS[tx_size as usize];
    let bd = 8;
    let mut input = Box::new(Aligned([0i16; BLK_SIZE]));
    let mut inv_input = Box::new(Aligned([0i32; BLK_SIZE]));
    let mut output = Box::new(Aligned([0u8; BLK_SIZE]));
    let mut ref_output = Box::new(Aligned([0u16; BLK_SIZE]));
    let mut ref_output8 = Box::new(Aligned([0u8; BLK_SIZE]));
    let stride = BLK_WIDTH as i32;
    let rows = TX_SIZE_HIGH[tx_size as usize];
    let cols = TX_SIZE_WIDE[tx_size as usize];
    let rows_nonezero = rows.min(32);
    let cols_nonezero = cols.min(32);
    // SAFETY: av1_scan_orders is an immutable table after initialization.
    let scan = unsafe { av1_scan_orders[tx_size as usize][tx_type as usize].scan };
    let eobmax = rows_nonezero * cols_nonezero;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let rand_times = eobmax + 500;

    let mut compared = 0;
    for cnt in 0..rand_times {
        let max_in: i16 = (1 << bd) - 1;
        for r in 0..BLK_WIDTH {
            for c in 0..BLK_WIDTH {
                input.0[r * cols as usize + c] =
                    if cnt == 0 { max_in } else { rnd.rand8_extremes() as i16 };
                output.0[r * BLK_WIDTH + c] = if cnt == 0 { 128 } else { rnd.rand8() };
                ref_output.0[r * BLK_WIDTH + c] = output.0[r * BLK_WIDTH + c] as u16;
                ref_output8.0[r * BLK_WIDTH + c] = output.0[r * BLK_WIDTH + c];
            }
        }
        // SAFETY: 64x64 input and output blocks cover every transform size.
        unsafe { fwd_func(input.0.as_ptr(), inv_input.0.as_mut_ptr(), stride, tx_type, bd) };

        // produce eob input by setting high freq coeffs to zero
        let eob = (cnt + 1).min(eobmax);
        for i in eob..eobmax {
            // SAFETY: the scan table has eobmax entries for this size.
            inv_input.0[unsafe { *scan.add(i as usize) } as usize] = 0;
        }
        if gt_int16 {
            // SAFETY: as above.
            inv_input.0[unsafe { *scan.add((eob - 1) as usize) } as usize] =
                i16::MAX as i32 * 100 / 141;
        }
        match target {
            Target::Lbd(target_func) => {
                // SAFETY: 64x64 blocks with stride 64.
                unsafe {
                    ref_func(inv_input.0.as_ptr(), ref_output.0.as_mut_ptr(), stride, tx_type, bd);
                    target_func(inv_input.0.as_ptr(), output.0.as_mut_ptr(), stride, tx_type,
                                tx_size, eob);
                }
                for r in 0..rows as usize {
                    for c in 0..cols as usize {
                        let ref_value = ref_output.0[r * BLK_WIDTH + c] as u8;
                        assert_eq!(ref_value, output.0[r * BLK_WIDTH + c],
                                   "[{r},{c}] {cnt} tx_size: {cols}x{rows} tx_type: {} eob {eob}",
                                   TX_TYPE_NAME[tx_type as usize]);
                    }
                }
            }
            Target::Add { neon, lossless } => {
                let txfm_param = TxfmParam {
                    tx_type,
                    tx_size,
                    lossless,
                    bd,
                    is_hbd: 0,
                    tx_set_type: EXT_TX_SET_ALL16,
                    eob,
                };
                // SAFETY: 64x64 blocks with stride 64.
                unsafe {
                    av1_inv_txfm_add_c(inv_input.0.as_ptr(), ref_output8.0.as_mut_ptr(), stride,
                                       &txfm_param);
                    neon(inv_input.0.as_ptr(), output.0.as_mut_ptr(), stride, &txfm_param);
                }
                for r in 0..rows as usize {
                    for c in 0..cols as usize {
                        assert_eq!(ref_output8.0[r * BLK_WIDTH + c], output.0[r * BLK_WIDTH + c],
                                   "[{r},{c}] {cnt} tx_size: {cols}x{rows} tx_type: {} eob {eob} \
                                    lossless {lossless}",
                                   TX_TYPE_NAME[tx_type as usize]);
                    }
                }
            }
        }
        compared += 1;
    }
    compared
}

/// TEST_P(AV1LbdInvTxfm2d, match)
fn av1_lbd_inv_txfm2d_match(target: Target) -> usize {
    let mut compared = 0;
    for j in 0..TX_SIZES_ALL as u8 {
        for i in 0..TX_TYPES as u8 {
            if is_tx_size_type_valid(j, i) {
                compared += run_av1_inv_txfm2d_test(target, i, j, false);
            }
        }
    }
    compared
}

/// TEST_P(AV1LbdInvTxfm2d, gt_int16)
fn av1_lbd_inv_txfm2d_gt_int16(target: Target) -> usize {
    let types = [DCT_DCT, ADST_DCT, FLIPADST_DCT, IDTX, V_DCT, H_DCT, H_ADST, H_FLIPADST];
    let mut compared = 0;
    for j in 0..TX_SIZES_ALL as u8 {
        for tp in types {
            if is_tx_size_type_valid(j, tp) {
                compared += run_av1_inv_txfm2d_test(target, tp, j, true);
            }
        }
    }
    compared
}

#[test]
fn lowbd_inv_txfm2d_match() {
    let compared = av1_lbd_inv_txfm2d_match(Target::Lbd(av1_lowbd_inv_txfm2d_add_neon));
    eprintln!("NEON/AV1LbdInvTxfm2d.match: {compared} blocks compared");
}

#[test]
fn lowbd_inv_txfm2d_gt_int16() {
    let compared = av1_lbd_inv_txfm2d_gt_int16(Target::Lbd(av1_lowbd_inv_txfm2d_add_neon));
    eprintln!("NEON/AV1LbdInvTxfm2d.gt_int16: {compared} blocks compared");
}

/// Supplementary: the RTCD-dispatched entry against av1_inv_txfm_add_c.
#[test]
fn lowbd_inv_txfm_add_dispatched_entry() {
    // av1_inv_txfm_add_c reaches av1_highbd_inv_txfm_add through its RTCD
    // pointer. Select the `_c` implementation, as AOM_SIMD_CAPS_MASK=0 does,
    // so the reference is scalar throughout. No other test reads this
    // pointer.
    // SAFETY: a plain store of a function pointer; the RTCD unit declares
    // its own nominal copy of the same C record type `TxfmParam`, hence the
    // pointer-type transmute.
    unsafe {
        aom::av1__common__av1_rtcd::av1_highbd_inv_txfm_add =
            Some(core::mem::transmute(av1_highbd_inv_txfm_add_c as InvTxfmAddFunc));
    }
    let target = Target::Add { neon: av1_inv_txfm_add_neon, lossless: 0 };
    let compared = av1_lbd_inv_txfm2d_match(target) + av1_lbd_inv_txfm2d_gt_int16(target);
    // Lossless coding uses only TX_4X4 with WHT_WHT (signalled as DCT_DCT);
    // both functions route it to av1_inv_txfm_add_c.
    let lossless = Target::Add { neon: av1_inv_txfm_add_neon, lossless: 1 };
    let lossless_compared = run_av1_inv_txfm2d_test(lossless, DCT_DCT, TX_4X4, false);
    eprintln!("av1_inv_txfm_add_neon: {compared} lossy and {lossless_compared} lossless blocks \
               compared");
}
