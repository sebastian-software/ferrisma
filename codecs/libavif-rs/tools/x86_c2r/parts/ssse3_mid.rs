
// From av1/common/enums.h.
type TX_TYPE = u8;

pub const TX_4X4: i32 = 0;
pub const TX_8X8: i32 = 1;
pub const TX_16X16: i32 = 2;
pub const TX_32X32: i32 = 3;
pub const TX_64X64: i32 = 4;
pub const TX_4X8: i32 = 5;
pub const TX_8X4: i32 = 6;
pub const TX_8X16: i32 = 7;
pub const TX_16X8: i32 = 8;
pub const TX_16X32: i32 = 9;
pub const TX_32X16: i32 = 10;
pub const TX_32X64: i32 = 11;
pub const TX_64X32: i32 = 12;
pub const TX_4X16: i32 = 13;
pub const TX_16X4: i32 = 14;
pub const TX_8X32: i32 = 15;
pub const TX_32X8: i32 = 16;
pub const TX_16X64: i32 = 17;
pub const TX_64X16: i32 = 18;
pub const TX_SIZES_ALL: i32 = 19;
pub const TX_SIZES: i32 = TX_4X8;

pub const DCT_DCT: i32 = 0;
pub const ADST_DCT: i32 = 1;
pub const DCT_ADST: i32 = 2;
pub const ADST_ADST: i32 = 3;
pub const FLIPADST_DCT: i32 = 4;
pub const DCT_FLIPADST: i32 = 5;
pub const FLIPADST_FLIPADST: i32 = 6;
pub const ADST_FLIPADST: i32 = 7;
pub const FLIPADST_ADST: i32 = 8;
pub const IDTX: i32 = 9;
pub const V_DCT: i32 = 10;
pub const H_DCT: i32 = 11;
pub const V_ADST: i32 = 12;
pub const H_ADST: i32 = 13;
pub const V_FLIPADST: i32 = 14;
pub const H_FLIPADST: i32 = 15;
pub const TX_TYPES: i32 = 16;

// From av1/common/av1_inv_txfm1d_cfg.h.
pub const INV_COS_BIT: i32 = 12;

// From av1/common/av1_txfm.h.
const cos_bit_min: i32 = 10;
pub const NewSqrt2Bits: i32 = 12;
// 2^12 * sqrt(2)
pub const NewSqrt2: i32 = 5793;
// 2^12 / sqrt(2)
pub const NewInvSqrt2: i32 = 2896;

#[inline]
pub unsafe fn cospi_arr(n: i32) -> *const i32 {
    (&raw const av1_cospi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline]
pub unsafe fn sinpi_arr(n: i32) -> *const i32 {
    (&raw const av1_sinpi_arr_data[(n - cos_bit_min) as usize]).cast::<i32>()
}

#[inline]
pub unsafe fn get_flip_cfg(tx_type: TX_TYPE, ud_flip: *mut i32, lr_flip: *mut i32) {
    match tx_type as i32 {
        DCT_DCT | ADST_DCT | DCT_ADST | ADST_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        IDTX | V_DCT | H_DCT | V_ADST | H_ADST => {
            *ud_flip = 0;
            *lr_flip = 0;
        }
        FLIPADST_DCT | FLIPADST_ADST | V_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 0;
        }
        DCT_FLIPADST | ADST_FLIPADST | H_FLIPADST => {
            *ud_flip = 0;
            *lr_flip = 1;
        }
        FLIPADST_FLIPADST => {
            *ud_flip = 1;
            *lr_flip = 1;
        }
        _ => {
            *ud_flip = 0;
            *lr_flip = 0;
            debug_assert!(false);
        }
    }
}

// Utility function that returns the log of the ratio of the col and row
// sizes.
#[inline]
pub fn get_rect_tx_log_ratio(col: i32, row: i32) -> i32 {
    if col == row {
        return 0;
    }
    if col > row {
        if col == row * 2 {
            return 1;
        }
        if col == row * 4 {
            return 2;
        }
        debug_assert!(false, "Unsupported transform size");
    } else {
        if row == col * 2 {
            return -1;
        }
        if row == col * 4 {
            return -2;
        }
        debug_assert!(false, "Unsupported transform size");
    }
    0 // Invalid
}

// From av1/common/common_data.h.
// Transform block width in log2
static tx_size_wide_log2: [i32; 19] = [2, 3, 4, 5, 6, 2, 3, 3, 4, 4, 5, 5, 6, 2, 4, 3, 5, 4, 6];
// Transform block height in log2
static tx_size_high_log2: [i32; 19] = [2, 3, 4, 5, 6, 3, 2, 4, 3, 5, 4, 6, 5, 4, 2, 5, 3, 6, 4];

#[inline]
pub fn get_txw_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_wide_log2[tx_size as usize] - tx_size_wide_log2[0]
}

#[inline]
pub fn get_txh_idx(tx_size: TX_SIZE) -> i32 {
    tx_size_high_log2[tx_size as usize] - tx_size_high_log2[0]
}

// From av1/common/x86/av1_inv_txfm_ssse3.h.
macro_rules! btf_16_ssse3 {
    ($w0:expr, $w1:expr, $in:expr, $out0:expr, $out1:expr) => {{
        let _w0: __m128i = _mm_set1_epi16(($w0 * 8) as i16);
        let _w1: __m128i = _mm_set1_epi16(($w1 * 8) as i16);
        let _in: __m128i = $in;
        $out0 = _mm_mulhrs_epi16(_in, _w0);
        $out1 = _mm_mulhrs_epi16(_in, _w1);
    }};
}
pub(crate) use btf_16_ssse3;

macro_rules! btf_16_adds_subs_sse2 {
    ($in0:expr, $in1:expr) => {{
        let _in0: __m128i = $in0;
        let _in1: __m128i = $in1;
        $in0 = _mm_adds_epi16(_in0, _in1);
        $in1 = _mm_subs_epi16(_in0, _in1);
    }};
}
pub(crate) use btf_16_adds_subs_sse2;

macro_rules! btf_16_subs_adds_sse2 {
    ($in0:expr, $in1:expr) => {{
        let _in0: __m128i = $in0;
        let _in1: __m128i = $in1;
        $in1 = _mm_subs_epi16(_in0, _in1);
        $in0 = _mm_adds_epi16(_in0, _in1);
    }};
}
pub(crate) use btf_16_subs_adds_sse2;

macro_rules! btf_16_adds_subs_out_sse2 {
    ($out0:expr, $out1:expr, $in0:expr, $in1:expr) => {{
        let _in0: __m128i = $in0;
        let _in1: __m128i = $in1;
        $out0 = _mm_adds_epi16(_in0, _in1);
        $out1 = _mm_subs_epi16(_in0, _in1);
    }};
}
pub(crate) use btf_16_adds_subs_out_sse2;

#[inline]
#[target_feature(enable = "ssse3")]
pub unsafe fn round_shift_16bit_ssse3(in_: *mut __m128i, size: i32, bit: i32) {
    if bit < 0 {
        let scale: __m128i = _mm_set1_epi16((1 << (15 + bit)) as i16);
        for i in 0..size {
            *in_.offset(i as isize) = _mm_mulhrs_epi16(*in_.offset(i as isize), scale);
        }
    } else if bit > 0 {
        for i in 0..size {
            *in_.offset(i as isize) =
                _mm_sll_epi16(*in_.offset(i as isize), _mm_cvtsi32_si128(bit));
        }
    }
}

pub type transform_1d_ssse3 = Option<unsafe fn(*const __m128i, *mut __m128i)>;

// ---------------------------------------------------------------------------
// av1/common/x86/av1_inv_txfm_ssse3.c

