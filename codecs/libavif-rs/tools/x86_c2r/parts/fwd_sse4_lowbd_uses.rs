use crate::av1__common__idct::TxfmParam;
use crate::av1__encoder__av1_fwd_txfm2d::{av1_fwd_cos_bit_col, av1_fwd_cos_bit_row, av1_fwd_txfm_shift_ls};
use crate::simd::aom_dsp__x86__transpose_sse2::*;
use crate::simd::av1__common__x86__av1_inv_txfm_ssse3::{
    get_flip_cfg, get_rect_tx_log_ratio, get_txh_idx, get_txw_idx, tx_size_high, AOMMIN, DCT_DCT, NewSqrt2,
    TX_32X64, TX_4X4, TX_64X32, TX_SIZES_ALL,
};
use crate::simd::av1__common__x86__av1_txfm_sse2::*;
use crate::simd::av1__encoder__x86__av1_fwd_txfm_sse2::*;
use crate::simd::av1__common__x86__av1_txfm_sse4::av1_round_shift_rect_array_32_sse4_1;

type FwdTxfm2dFunc = Option<unsafe fn(*const i16, *mut i32, i32, u8, i32)>;

extern "C-unwind" {
    // av1/encoder/hybrid_fwd_txfm.c, declared with this crate's TxfmParam as the Neon
    // translation does.
    fn av1_lowbd_fwd_txfm_c(src_diff: *const i16, coeff: *mut i32, diff_stride: i32,
                            txfm_param: *mut TxfmParam);
}
