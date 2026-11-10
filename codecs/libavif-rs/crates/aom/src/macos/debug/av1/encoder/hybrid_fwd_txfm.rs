/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/av1/encoder/hybrid_fwd_txfm.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static mut av1_fwd_txfm2d_16x16: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_16x32: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_16x4: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_16x64: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_16x8: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_32x16: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_32x32: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_32x64: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_32x8: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_4x16: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_4x4: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_4x8: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_64x16: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_64x32: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_64x64: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_8x16: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_8x32: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_8x4: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut av1_fwd_txfm2d_8x8: Option<
        unsafe extern "C-unwind" fn(
            *const int16_t,
            *mut int32_t,
            core::ffi::c_int,
            TX_TYPE,
            core::ffi::c_int,
        ) -> (),
    >;
    fn aom_hadamard_16x16_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
    fn aom_hadamard_32x32_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
    fn aom_hadamard_4x4_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
    fn aom_hadamard_8x8_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
    fn aom_highbd_hadamard_16x16_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
    fn aom_highbd_hadamard_32x32_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
    fn aom_highbd_hadamard_8x8_c(
        src_diff: *const int16_t,
        src_stride: ptrdiff_t,
        coeff: *mut tran_low_t,
    );
}
pub type ptrdiff_t = isize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type tran_high_t = int64_t;
pub type tran_low_t = int32_t;
pub type C2RustUnnamed = core::ffi::c_uint;
pub const TX_INVALID: C2RustUnnamed = 255;
pub const TX_SIZES_LARGEST: C2RustUnnamed = 4;
pub const TX_SIZES: C2RustUnnamed = 5;
pub const TX_SIZES_ALL: C2RustUnnamed = 19;
pub const TX_64X16: C2RustUnnamed = 18;
pub const TX_16X64: C2RustUnnamed = 17;
pub const TX_32X8: C2RustUnnamed = 16;
pub const TX_8X32: C2RustUnnamed = 15;
pub const TX_16X4: C2RustUnnamed = 14;
pub const TX_4X16: C2RustUnnamed = 13;
pub const TX_64X32: C2RustUnnamed = 12;
pub const TX_32X64: C2RustUnnamed = 11;
pub const TX_32X16: C2RustUnnamed = 10;
pub const TX_16X32: C2RustUnnamed = 9;
pub const TX_16X8: C2RustUnnamed = 8;
pub const TX_8X16: C2RustUnnamed = 7;
pub const TX_8X4: C2RustUnnamed = 6;
pub const TX_4X8: C2RustUnnamed = 5;
pub const TX_64X64: C2RustUnnamed = 4;
pub const TX_32X32: C2RustUnnamed = 3;
pub const TX_16X16: C2RustUnnamed = 2;
pub const TX_8X8: C2RustUnnamed = 1;
pub const TX_4X4: C2RustUnnamed = 0;
pub type TX_SIZE = uint8_t;
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const TX_TYPE_INVALID: C2RustUnnamed_0 = 255;
pub const DCT_ADST_TX_MASK: C2RustUnnamed_0 = 15;
pub const TX_TYPES: C2RustUnnamed_0 = 16;
pub const H_FLIPADST: C2RustUnnamed_0 = 15;
pub const V_FLIPADST: C2RustUnnamed_0 = 14;
pub const H_ADST: C2RustUnnamed_0 = 13;
pub const V_ADST: C2RustUnnamed_0 = 12;
pub const H_DCT: C2RustUnnamed_0 = 11;
pub const V_DCT: C2RustUnnamed_0 = 10;
pub const IDTX: C2RustUnnamed_0 = 9;
pub const FLIPADST_ADST: C2RustUnnamed_0 = 8;
pub const ADST_FLIPADST: C2RustUnnamed_0 = 7;
pub const FLIPADST_FLIPADST: C2RustUnnamed_0 = 6;
pub const DCT_FLIPADST: C2RustUnnamed_0 = 5;
pub const FLIPADST_DCT: C2RustUnnamed_0 = 4;
pub const ADST_ADST: C2RustUnnamed_0 = 3;
pub const DCT_ADST: C2RustUnnamed_0 = 2;
pub const ADST_DCT: C2RustUnnamed_0 = 1;
pub const DCT_DCT: C2RustUnnamed_0 = 0;
pub type TX_TYPE = uint8_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const EXT_TX_SET_TYPES: C2RustUnnamed_1 = 6;
pub const EXT_TX_SET_ALL16: C2RustUnnamed_1 = 5;
pub const EXT_TX_SET_DTT9_IDTX_1DDCT: C2RustUnnamed_1 = 4;
pub const EXT_TX_SET_DTT4_IDTX_1DDCT: C2RustUnnamed_1 = 3;
pub const EXT_TX_SET_DTT4_IDTX: C2RustUnnamed_1 = 2;
pub const EXT_TX_SET_DCT_IDTX: C2RustUnnamed_1 = 1;
pub const EXT_TX_SET_DCTONLY: C2RustUnnamed_1 = 0;
pub type TxSetType = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct txfm_param {
    pub tx_type: TX_TYPE,
    pub tx_size: TX_SIZE,
    pub lossless: core::ffi::c_int,
    pub bd: core::ffi::c_int,
    pub is_hbd: core::ffi::c_int,
    pub tx_set_type: TxSetType,
    pub eob: core::ffi::c_int,
}
pub type TxfmParam = txfm_param;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BitDepthInfo {
    pub bit_depth: core::ffi::c_int,
    pub use_highbitdepth_buf: core::ffi::c_int,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const UNIT_QUANT_SHIFT: core::ffi::c_int = 2 as core::ffi::c_int;
pub const UNIT_QUANT_FACTOR: core::ffi::c_int = (1 as core::ffi::c_int)
    << UNIT_QUANT_SHIFT;
static mut av1_ext_tx_used: [[core::ffi::c_int; 16]; 6] = [
    [
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwht4x4_c(
    mut input: *const int16_t,
    mut output: *mut tran_low_t,
    mut stride: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut a1: tran_high_t = 0;
    let mut b1: tran_high_t = 0;
    let mut c1: tran_high_t = 0;
    let mut d1: tran_high_t = 0;
    let mut e1: tran_high_t = 0;
    let mut ip_pass0: *const int16_t = input;
    let mut ip: *const tran_low_t = 0 as *const tran_low_t;
    let mut op: *mut tran_low_t = output;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        a1 = *ip_pass0.offset((0 as core::ffi::c_int * stride) as isize) as tran_high_t;
        b1 = *ip_pass0.offset((1 as core::ffi::c_int * stride) as isize) as tran_high_t;
        c1 = *ip_pass0.offset((2 as core::ffi::c_int * stride) as isize) as tran_high_t;
        d1 = *ip_pass0.offset((3 as core::ffi::c_int * stride) as isize) as tran_high_t;
        a1 += b1;
        d1 = d1 - c1;
        e1 = a1 - d1 >> 1 as core::ffi::c_int;
        b1 = e1 - b1;
        c1 = e1 - c1;
        a1 -= c1;
        d1 += b1;
        *op.offset(0 as core::ffi::c_int as isize) = a1 as tran_low_t;
        *op.offset(1 as core::ffi::c_int as isize) = c1 as tran_low_t;
        *op.offset(2 as core::ffi::c_int as isize) = d1 as tran_low_t;
        *op.offset(3 as core::ffi::c_int as isize) = b1 as tran_low_t;
        ip_pass0 = ip_pass0.offset(1);
        op = op.offset(4 as core::ffi::c_int as isize);
        i += 1;
    }
    ip = output;
    op = output;
    i = 0 as core::ffi::c_int;
    while i < 4 as core::ffi::c_int {
        a1 = *ip.offset((4 as core::ffi::c_int * 0 as core::ffi::c_int) as isize)
            as tran_high_t;
        b1 = *ip.offset((4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize)
            as tran_high_t;
        c1 = *ip.offset((4 as core::ffi::c_int * 2 as core::ffi::c_int) as isize)
            as tran_high_t;
        d1 = *ip.offset((4 as core::ffi::c_int * 3 as core::ffi::c_int) as isize)
            as tran_high_t;
        a1 += b1;
        d1 -= c1;
        e1 = a1 - d1 >> 1 as core::ffi::c_int;
        b1 = e1 - b1;
        c1 = e1 - c1;
        a1 -= c1;
        d1 += b1;
        *op.offset((4 as core::ffi::c_int * 0 as core::ffi::c_int) as isize) = (a1
            * UNIT_QUANT_FACTOR as tran_high_t) as tran_low_t;
        *op.offset((4 as core::ffi::c_int * 1 as core::ffi::c_int) as isize) = (c1
            * UNIT_QUANT_FACTOR as tran_high_t) as tran_low_t;
        *op.offset((4 as core::ffi::c_int * 2 as core::ffi::c_int) as isize) = (d1
            * UNIT_QUANT_FACTOR as tran_high_t) as tran_low_t;
        *op.offset((4 as core::ffi::c_int * 3 as core::ffi::c_int) as isize) = (b1
            * UNIT_QUANT_FACTOR as tran_high_t) as tran_low_t;
        ip = ip.offset(1);
        op = op.offset(1);
        i += 1;
    }
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_4x4(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    if (*txfm_param).lossless != 0 {
        if !(tx_type as core::ffi::c_int == DCT_DCT as core::ffi::c_int)
            as core::ffi::c_int as core::ffi::c_long != 0
        {
            crate::platform::assertion(
                (::core::mem::transmute::<
                    [u8; 20],
                    [core::ffi::c_char; 20],
                >(*b"highbd_fwd_txfm_4x4\0"))
                    .as_ptr(),
                b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
                84 as core::ffi::c_int,
                b"tx_type == DCT_DCT\0" as *const u8 as *const core::ffi::c_char,
            );
        } else {};
        av1_fwht4x4_c(src_diff, coeff, diff_stride);
        return;
    }
    av1_fwd_txfm2d_4x4
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_4x8(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_4x8
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_8x4(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_8x4
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_8x16(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_8x16
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_16x8(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_16x8
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_16x32(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_16x32
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_32x16(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_32x16
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_16x4(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_16x4
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_4x16(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_4x16
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_32x8(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_32x8
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_8x32(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    av1_fwd_txfm2d_8x32
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, (*txfm_param).bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_8x8(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_8x8
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_16x16(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_16x16
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_32x32(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let tx_type: TX_TYPE = (*txfm_param).tx_type;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_32x32
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_32x64(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if !((*txfm_param).tx_type as core::ffi::c_int == DCT_DCT as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"highbd_fwd_txfm_32x64\0"))
                .as_ptr(),
            b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
            191 as core::ffi::c_int,
            b"txfm_param->tx_type == DCT_DCT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_32x64
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_64x32(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if !((*txfm_param).tx_type as core::ffi::c_int == DCT_DCT as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"highbd_fwd_txfm_64x32\0"))
                .as_ptr(),
            b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
            200 as core::ffi::c_int,
            b"txfm_param->tx_type == DCT_DCT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_64x32
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, (*txfm_param).tx_type, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_16x64(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if !((*txfm_param).tx_type as core::ffi::c_int == DCT_DCT as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"highbd_fwd_txfm_16x64\0"))
                .as_ptr(),
            b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
            210 as core::ffi::c_int,
            b"txfm_param->tx_type == DCT_DCT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_16x64
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, DCT_DCT as core::ffi::c_int as TX_TYPE, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_64x16(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if !((*txfm_param).tx_type as core::ffi::c_int == DCT_DCT as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"highbd_fwd_txfm_64x16\0"))
                .as_ptr(),
            b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
            218 as core::ffi::c_int,
            b"txfm_param->tx_type == DCT_DCT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_64x16
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, DCT_DCT as core::ffi::c_int as TX_TYPE, bd);
}
unsafe extern "C-unwind" fn highbd_fwd_txfm_64x64(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if !((*txfm_param).tx_type as core::ffi::c_int == DCT_DCT as core::ffi::c_int)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 22],
                [core::ffi::c_char; 22],
            >(*b"highbd_fwd_txfm_64x64\0"))
                .as_ptr(),
            b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
            227 as core::ffi::c_int,
            b"txfm_param->tx_type == DCT_DCT\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let mut dst_coeff: *mut int32_t = coeff as *mut int32_t;
    let bd: core::ffi::c_int = (*txfm_param).bd;
    av1_fwd_txfm2d_64x64
        .expect(
            "non-null function pointer",
        )(src_diff, dst_coeff, diff_stride, DCT_DCT as core::ffi::c_int as TX_TYPE, bd);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_fwd_txfm(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if (*txfm_param).bd == 8 as core::ffi::c_int {
        av1_lowbd_fwd_txfm_c(src_diff, coeff, diff_stride, txfm_param);
    } else {
        av1_highbd_fwd_txfm(src_diff, coeff, diff_stride, txfm_param);
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_lowbd_fwd_txfm_c(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    av1_highbd_fwd_txfm(src_diff, coeff, diff_stride, txfm_param);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_highbd_fwd_txfm(
    mut src_diff: *const int16_t,
    mut coeff: *mut tran_low_t,
    mut diff_stride: core::ffi::c_int,
    mut txfm_param: *mut TxfmParam,
) {
    if (av1_ext_tx_used[(*txfm_param).tx_set_type
        as usize][(*txfm_param).tx_type as usize] == 0) as core::ffi::c_int
        as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 20],
                [core::ffi::c_char; 20],
            >(*b"av1_highbd_fwd_txfm\0"))
                .as_ptr(),
            b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
            248 as core::ffi::c_int,
            b"av1_ext_tx_used[txfm_param->tx_set_type][txfm_param->tx_type]\0"
                as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let tx_size: TX_SIZE = (*txfm_param).tx_size;
    match tx_size as core::ffi::c_int {
        4 => {
            highbd_fwd_txfm_64x64(src_diff, coeff, diff_stride, txfm_param);
        }
        11 => {
            highbd_fwd_txfm_32x64(src_diff, coeff, diff_stride, txfm_param);
        }
        12 => {
            highbd_fwd_txfm_64x32(src_diff, coeff, diff_stride, txfm_param);
        }
        3 => {
            highbd_fwd_txfm_32x32(src_diff, coeff, diff_stride, txfm_param);
        }
        2 => {
            highbd_fwd_txfm_16x16(src_diff, coeff, diff_stride, txfm_param);
        }
        1 => {
            highbd_fwd_txfm_8x8(src_diff, coeff, diff_stride, txfm_param);
        }
        5 => {
            highbd_fwd_txfm_4x8(src_diff, coeff, diff_stride, txfm_param);
        }
        6 => {
            highbd_fwd_txfm_8x4(src_diff, coeff, diff_stride, txfm_param);
        }
        7 => {
            highbd_fwd_txfm_8x16(src_diff, coeff, diff_stride, txfm_param);
        }
        8 => {
            highbd_fwd_txfm_16x8(src_diff, coeff, diff_stride, txfm_param);
        }
        9 => {
            highbd_fwd_txfm_16x32(src_diff, coeff, diff_stride, txfm_param);
        }
        10 => {
            highbd_fwd_txfm_32x16(src_diff, coeff, diff_stride, txfm_param);
        }
        0 => {
            highbd_fwd_txfm_4x4(src_diff, coeff, diff_stride, txfm_param);
        }
        13 => {
            highbd_fwd_txfm_4x16(src_diff, coeff, diff_stride, txfm_param);
        }
        14 => {
            highbd_fwd_txfm_16x4(src_diff, coeff, diff_stride, txfm_param);
        }
        15 => {
            highbd_fwd_txfm_8x32(src_diff, coeff, diff_stride, txfm_param);
        }
        16 => {
            highbd_fwd_txfm_32x8(src_diff, coeff, diff_stride, txfm_param);
        }
        17 => {
            highbd_fwd_txfm_16x64(src_diff, coeff, diff_stride, txfm_param);
        }
        18 => {
            highbd_fwd_txfm_64x16(src_diff, coeff, diff_stride, txfm_param);
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"av1_highbd_fwd_txfm\0"))
                        .as_ptr(),
                    b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
                    311 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn highbd_wht_fwd_txfm(
    mut tx_size: TX_SIZE,
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    match tx_size as core::ffi::c_int {
        0 => {
            aom_hadamard_4x4_c(src_diff, src_stride, coeff);
        }
        1 => {
            aom_highbd_hadamard_8x8_c(src_diff, src_stride, coeff);
        }
        2 => {
            aom_highbd_hadamard_16x16_c(src_diff, src_stride, coeff);
        }
        3 => {
            aom_highbd_hadamard_32x32_c(src_diff, src_stride, coeff);
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 20],
                        [core::ffi::c_char; 20],
                    >(*b"highbd_wht_fwd_txfm\0"))
                        .as_ptr(),
                    b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
                    331 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    };
}
#[inline]
unsafe extern "C-unwind" fn wht_fwd_txfm(
    mut tx_size: TX_SIZE,
    mut src_diff: *const int16_t,
    mut src_stride: ptrdiff_t,
    mut coeff: *mut tran_low_t,
) {
    match tx_size as core::ffi::c_int {
        0 => {
            aom_hadamard_4x4_c(src_diff, src_stride, coeff);
        }
        1 => {
            aom_hadamard_8x8_c(src_diff, src_stride, coeff);
        }
        2 => {
            aom_hadamard_16x16_c(src_diff, src_stride, coeff);
        }
        3 => {
            aom_hadamard_32x32_c(src_diff, src_stride, coeff);
        }
        _ => {
            if (0 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long != 0
            {
                crate::platform::assertion(
                    (::core::mem::transmute::<
                        [u8; 13],
                        [core::ffi::c_char; 13],
                    >(*b"wht_fwd_txfm\0"))
                        .as_ptr(),
                    b"hybrid_fwd_txfm.c\0" as *const u8 as *const core::ffi::c_char,
                    343 as core::ffi::c_int,
                    b"0\0" as *const u8 as *const core::ffi::c_char,
                );
            } else {};
        }
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_quick_txfm(
    mut use_hadamard: core::ffi::c_int,
    mut tx_size: TX_SIZE,
    mut bd_info: BitDepthInfo,
    mut src_diff: *const int16_t,
    mut src_stride: core::ffi::c_int,
    mut coeff: *mut tran_low_t,
) {
    if use_hadamard != 0 {
        if bd_info.use_highbitdepth_buf != 0 {
            highbd_wht_fwd_txfm(tx_size, src_diff, src_stride as ptrdiff_t, coeff);
        } else {
            wht_fwd_txfm(tx_size, src_diff, src_stride as ptrdiff_t, coeff);
        }
    } else {
        let mut txfm_param: TxfmParam = txfm_param {
            tx_type: 0,
            tx_size: 0,
            lossless: 0,
            bd: 0,
            is_hbd: 0,
            tx_set_type: 0,
            eob: 0,
        };
        txfm_param.tx_type = DCT_DCT as core::ffi::c_int as TX_TYPE;
        txfm_param.tx_size = tx_size;
        txfm_param.lossless = 0 as core::ffi::c_int;
        txfm_param.bd = bd_info.bit_depth;
        txfm_param.is_hbd = bd_info.use_highbitdepth_buf;
        txfm_param.tx_set_type = EXT_TX_SET_ALL16 as core::ffi::c_int as TxSetType;
        av1_fwd_txfm(src_diff, coeff, src_stride, &raw mut txfm_param);
    };
}
