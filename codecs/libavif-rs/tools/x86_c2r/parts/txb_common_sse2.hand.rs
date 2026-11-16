// From av1/common/enums.h and av1/common/entropy.h.
const TX_PAD_HOR: i32 = 4;
const SIG_COEF_CONTEXTS_2D: i32 = 26;
const TX_CLASS_2D: u8 = 0;
const TX_CLASS_HORIZ: u8 = 1;
const TX_CLASS_VERT: u8 = 2;
const TX_32X32: u8 = 3;
const TX_64X64: u8 = 4;
const TX_32X16: u8 = 10;
const TX_16X32: u8 = 9;
const TX_32X64: u8 = 11;
const TX_64X32: u8 = 12;
const TX_16X64: u8 = 17;
const TX_64X16: u8 = 18;

// From av1/common/common_data.h (tx_size_wide and tx_size_high come from
// crate::simd::av1__common__x86__av1_inv_txfm_ssse3).
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

// From av1/common/blockd.h.
#[inline]
fn av1_get_adjusted_tx_size(tx_size: u8) -> u8 {
    match tx_size {
        TX_64X64 | TX_64X32 | TX_32X64 => TX_32X32,
        TX_64X16 => TX_32X16,
        TX_16X64 => TX_16X32,
        _ => tx_size,
    }
}

// From av1/common/txb_common.h.
#[inline]
fn get_txb_bhl(tx_size: u8) -> i32 {
    tx_size_high_log2[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[inline]
fn get_txb_wide(tx_size: u8) -> i32 {
    tx_size_wide[av1_get_adjusted_tx_size(tx_size) as usize]
}

#[inline]
fn get_txb_high(tx_size: u8) -> i32 {
    tx_size_high[av1_get_adjusted_tx_size(tx_size) as usize]
}
