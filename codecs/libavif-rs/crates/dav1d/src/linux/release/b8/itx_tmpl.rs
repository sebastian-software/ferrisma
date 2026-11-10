/*
 * Copyright © 2018-2019, VideoLAN and dav1d authors
 * Copyright © 2018-2019, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Mechanically translated from upstream/dav1d/src/itx_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    static dav1d_tx1d_fns: [[itx_1d_fn; 4]; 5];
    static dav1d_tx1d_types: [[uint8_t; 2]; 16];
    fn dav1d_inv_wht4_1d_c(c: *mut int32_t, stride: ptrdiff_t);
    static dav1d_last_nonzero_col_from_eob: [*const uint8_t; 19];
    fn dav1d_init_last_nonzero_col_from_eob_tables();
    static dav1d_txfm_dimensions: [TxfmInfo; 19];
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __int32_t = i32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type pixel = uint8_t;
pub type coef = int16_t;
pub type TxfmSize = core::ffi::c_uint;
pub const N_TX_SIZES: TxfmSize = 5;
pub const TX_64X64: TxfmSize = 4;
pub const TX_32X32: TxfmSize = 3;
pub const TX_16X16: TxfmSize = 2;
pub const TX_8X8: TxfmSize = 1;
pub const TX_4X4: TxfmSize = 0;
pub type RectTxfmSize = core::ffi::c_uint;
pub const N_RECT_TX_SIZES: RectTxfmSize = 19;
pub const RTX_64X16: RectTxfmSize = 18;
pub const RTX_16X64: RectTxfmSize = 17;
pub const RTX_32X8: RectTxfmSize = 16;
pub const RTX_8X32: RectTxfmSize = 15;
pub const RTX_16X4: RectTxfmSize = 14;
pub const RTX_4X16: RectTxfmSize = 13;
pub const RTX_64X32: RectTxfmSize = 12;
pub const RTX_32X64: RectTxfmSize = 11;
pub const RTX_32X16: RectTxfmSize = 10;
pub const RTX_16X32: RectTxfmSize = 9;
pub const RTX_16X8: RectTxfmSize = 8;
pub const RTX_8X16: RectTxfmSize = 7;
pub const RTX_8X4: RectTxfmSize = 6;
pub const RTX_4X8: RectTxfmSize = 5;
pub type TxfmType = core::ffi::c_uint;
pub const N_TX_TYPES_PLUS_LL: TxfmType = 17;
pub const WHT_WHT: TxfmType = 16;
pub const N_TX_TYPES: TxfmType = 16;
pub const H_FLIPADST: TxfmType = 15;
pub const V_FLIPADST: TxfmType = 14;
pub const H_ADST: TxfmType = 13;
pub const V_ADST: TxfmType = 12;
pub const H_DCT: TxfmType = 11;
pub const V_DCT: TxfmType = 10;
pub const IDTX: TxfmType = 9;
pub const FLIPADST_ADST: TxfmType = 8;
pub const ADST_FLIPADST: TxfmType = 7;
pub const FLIPADST_FLIPADST: TxfmType = 6;
pub const DCT_FLIPADST: TxfmType = 5;
pub const FLIPADST_DCT: TxfmType = 4;
pub const ADST_ADST: TxfmType = 3;
pub const DCT_ADST: TxfmType = 2;
pub const ADST_DCT: TxfmType = 1;
pub const DCT_DCT: TxfmType = 0;
pub type itxfm_fn = Option<
    unsafe extern "C" fn(*mut pixel, ptrdiff_t, *mut coef, core::ffi::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[itxfm_fn; 17]; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TxfmInfo {
    pub w: uint8_t,
    pub h: uint8_t,
    pub lw: uint8_t,
    pub lh: uint8_t,
    pub min: uint8_t,
    pub max: uint8_t,
    pub sub: uint8_t,
    pub ctx: uint8_t,
}
pub type itx_1d_fn = Option<
    unsafe extern "C" fn(
        *mut int32_t,
        ptrdiff_t,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub const IDENTITY: Tx1dType = 2;
pub type Tx1dType = core::ffi::c_uint;
pub const N_TX_1D_TYPES: Tx1dType = 4;
pub const FLIPADST: Tx1dType = 3;
pub const ADST: Tx1dType = 1;
pub const DCT: Tx1dType = 0;
pub const INT16_MIN: core::ffi::c_int = -(32767 as core::ffi::c_int)
    - 1 as core::ffi::c_int;
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
#[inline]
unsafe extern "C" fn iclip(
    v: core::ffi::c_int,
    min: core::ffi::c_int,
    max: core::ffi::c_int,
) -> core::ffi::c_int {
    return if v < min { min } else if v > max { max } else { v };
}
#[inline]
unsafe extern "C" fn iclip_u8(v: core::ffi::c_int) -> core::ffi::c_int {
    return iclip(v, 0 as core::ffi::c_int, 255 as core::ffi::c_int);
}
#[inline(never)]
unsafe extern "C" fn inv_txfm_add_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
    tx: core::ffi::c_int,
    shift: core::ffi::c_int,
    txtp: TxfmType,
) {
    let t_dim: *const TxfmInfo = &*dav1d_txfm_dimensions.as_ptr().offset(tx as isize)
        as *const TxfmInfo;
    let w: core::ffi::c_int = 4 as core::ffi::c_int * (*t_dim).w as core::ffi::c_int;
    let h: core::ffi::c_int = 4 as core::ffi::c_int * (*t_dim).h as core::ffi::c_int;
    let has_dconly: core::ffi::c_int = (txtp as core::ffi::c_uint
        == DCT_DCT as core::ffi::c_int as core::ffi::c_uint) as core::ffi::c_int;
    if !(w >= 4 as core::ffi::c_int && w <= 64 as core::ffi::c_int) {
        unreachable!();
    }
    if !(h >= 4 as core::ffi::c_int && h <= 64 as core::ffi::c_int) {
        unreachable!();
    }
    if !(eob >= 0 as core::ffi::c_int) {
        unreachable!();
    }
    let is_rect2: core::ffi::c_int = (w * 2 as core::ffi::c_int == h
        || h * 2 as core::ffi::c_int == w) as core::ffi::c_int;
    let rnd: core::ffi::c_int = (1 as core::ffi::c_int) << shift
        >> 1 as core::ffi::c_int;
    if eob < has_dconly {
        let mut dc: core::ffi::c_int = *coeff.offset(0 as core::ffi::c_int as isize)
            as core::ffi::c_int;
        *coeff.offset(0 as core::ffi::c_int as isize) = 0 as coef;
        if is_rect2 != 0 {
            dc = dc * 181 as core::ffi::c_int + 128 as core::ffi::c_int
                >> 8 as core::ffi::c_int;
        }
        dc = dc * 181 as core::ffi::c_int + 128 as core::ffi::c_int
            >> 8 as core::ffi::c_int;
        dc = dc + rnd >> shift;
        dc = dc * 181 as core::ffi::c_int + 128 as core::ffi::c_int
            + 2048 as core::ffi::c_int >> 12 as core::ffi::c_int;
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < h {
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < w {
                *dst.offset(x as isize) = iclip_u8(
                    *dst.offset(x as isize) as core::ffi::c_int + dc,
                ) as pixel;
                x += 1;
            }
            y += 1;
            dst = dst.offset(stride as isize);
        }
        return;
    }
    let txtps: *const uint8_t = (dav1d_tx1d_types[txtp as usize]).as_ptr();
    let first_1d_fn: itx_1d_fn = dav1d_tx1d_fns[(*t_dim).lw
        as usize][*txtps.offset(0 as core::ffi::c_int as isize) as usize];
    let second_1d_fn: itx_1d_fn = dav1d_tx1d_fns[(*t_dim).lh
        as usize][*txtps.offset(1 as core::ffi::c_int as isize) as usize];
    let sh: core::ffi::c_int = imin(h, 32 as core::ffi::c_int) as core::ffi::c_int;
    let sw: core::ffi::c_int = imin(w, 32 as core::ffi::c_int) as core::ffi::c_int;
    let row_clip_min: core::ffi::c_int = INT16_MIN;
    let col_clip_min: core::ffi::c_int = INT16_MIN;
    let row_clip_max: core::ffi::c_int = !row_clip_min;
    let col_clip_max: core::ffi::c_int = !col_clip_min;
    let mut tmp: [int32_t; 4096] = [0; 4096];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut last_nonzero_col: core::ffi::c_int = 0;
    if *txtps.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
        == IDENTITY as core::ffi::c_int
        && *txtps.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            != IDENTITY as core::ffi::c_int
    {
        last_nonzero_col = imin(sh - 1 as core::ffi::c_int, eob);
    } else if *txtps.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
        == IDENTITY as core::ffi::c_int
        && *txtps.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            != IDENTITY as core::ffi::c_int
    {
        last_nonzero_col = eob
            >> (*t_dim).lw as core::ffi::c_int + 2 as core::ffi::c_int;
    } else {
        last_nonzero_col = *(dav1d_last_nonzero_col_from_eob[tx as usize])
            .offset(eob as isize) as core::ffi::c_int;
    }
    if !(last_nonzero_col < sh) {
        unreachable!();
    }
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 <= last_nonzero_col {
        if is_rect2 != 0 {
            let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_0 < sw {
                *c.offset(x_0 as isize) = (*coeff.offset((y_0 + x_0 * sh) as isize)
                    as core::ffi::c_int * 181 as core::ffi::c_int
                    + 128 as core::ffi::c_int >> 8 as core::ffi::c_int) as int32_t;
                x_0 += 1;
            }
        } else {
            let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
            while x_1 < sw {
                *c.offset(x_1 as isize) = *coeff.offset((y_0 + x_1 * sh) as isize)
                    as int32_t;
                x_1 += 1;
            }
        }
        first_1d_fn
            .expect(
                "non-null function pointer",
            )(c, 1 as ptrdiff_t, row_clip_min, row_clip_max);
        y_0 += 1;
        c = c.offset(w as isize);
    }
    if (last_nonzero_col + 1 as core::ffi::c_int) < sh {
        memset(
            c as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (::core::mem::size_of::<int32_t>() as size_t)
                .wrapping_mul((sh - last_nonzero_col - 1 as core::ffi::c_int) as size_t)
                .wrapping_mul(w as size_t),
        );
    }
    memset(
        coeff as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<coef>() as size_t)
            .wrapping_mul(sw as size_t)
            .wrapping_mul(sh as size_t),
    );
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < w * sh {
        tmp[i as usize] = iclip(
            tmp[i as usize] + rnd >> shift,
            col_clip_min,
            col_clip_max,
        ) as int32_t;
        i += 1;
    }
    let mut x_2: core::ffi::c_int = 0 as core::ffi::c_int;
    while x_2 < w {
        second_1d_fn
            .expect(
                "non-null function pointer",
            )(
            &raw mut *tmp.as_mut_ptr().offset(x_2 as isize),
            w as ptrdiff_t,
            col_clip_min,
            col_clip_max,
        );
        x_2 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_1: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_1 < h {
        let mut x_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_3 < w {
            let fresh0 = c;
            c = c.offset(1);
            *dst.offset(x_3 as isize) = iclip_u8(
                *dst.offset(x_3 as isize) as core::ffi::c_int
                    + (*fresh0 + 8 as core::ffi::c_int >> 4 as core::ffi::c_int),
            ) as pixel;
            x_3 += 1;
        }
        y_1 += 1;
        dst = dst.offset(stride as isize);
    }
}
unsafe extern "C" fn inv_txfm_add_dct_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_4X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_4x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X8 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_4x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_4X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X4 as core::ffi::c_int,
        0 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_8x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_8X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_8x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_8x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X32 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_8x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_8X32 as core::ffi::c_int,
        2 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_16x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X4 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_flipadst_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        H_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_identity_16x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X8 as core::ffi::c_int,
        1 as core::ffi::c_int,
        V_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_adst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        ADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        H_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        ADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_adst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_adst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        FLIPADST_ADST,
    );
}
unsafe extern "C" fn inv_txfm_add_adst_flipadst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        ADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_dct_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        FLIPADST_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_flipadst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_flipadst_flipadst_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        FLIPADST_FLIPADST,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_identity_16x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_16X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        V_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X32 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_16x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X32 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_16x64_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_16X64 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_32X8 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_32x8_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_32X8 as core::ffi::c_int,
        2 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_32X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_32x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_32X16 as core::ffi::c_int,
        1 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_32X32 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_identity_identity_32x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_32X32 as core::ffi::c_int,
        2 as core::ffi::c_int,
        IDTX,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_32x64_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_32X64 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_64x16_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_64X16 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_64x32_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        RTX_64X32 as core::ffi::c_int,
        1 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_dct_dct_64x64_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    inv_txfm_add_c(
        dst,
        stride,
        coeff,
        eob,
        TX_64X64 as core::ffi::c_int,
        2 as core::ffi::c_int,
        DCT_DCT,
    );
}
unsafe extern "C" fn inv_txfm_add_wht_wht_4x4_c(
    mut dst: *mut pixel,
    stride: ptrdiff_t,
    coeff: *mut coef,
    eob: core::ffi::c_int,
) {
    let mut tmp: [int32_t; 16] = [0; 16];
    let mut c: *mut int32_t = tmp.as_mut_ptr();
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 4 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < 4 as core::ffi::c_int {
            *c.offset(x as isize) = (*coeff
                .offset((y + x * 4 as core::ffi::c_int) as isize) as core::ffi::c_int
                >> 2 as core::ffi::c_int) as int32_t;
            x += 1;
        }
        dav1d_inv_wht4_1d_c(c, 1 as ptrdiff_t);
        y += 1;
        c = c.offset(4 as core::ffi::c_int as isize);
    }
    memset(
        coeff as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (::core::mem::size_of::<coef>() as size_t)
            .wrapping_mul(4 as size_t)
            .wrapping_mul(4 as size_t),
    );
    let mut x_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while x_0 < 4 as core::ffi::c_int {
        dav1d_inv_wht4_1d_c(&raw mut *tmp.as_mut_ptr().offset(x_0 as isize), 4 as ptrdiff_t);
        x_0 += 1;
    }
    c = tmp.as_mut_ptr();
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    while y_0 < 4 as core::ffi::c_int {
        let mut x_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while x_1 < 4 as core::ffi::c_int {
            let fresh1 = c;
            c = c.offset(1);
            *dst.offset(x_1 as isize) = iclip_u8(
                *dst.offset(x_1 as isize) as core::ffi::c_int + *fresh1,
            ) as pixel;
            x_1 += 1;
        }
        y_0 += 1;
        dst = dst.offset(stride as isize);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_itx_dsp_init_8bpc(
    c: *mut Dav1dInvTxfmDSPContext,
    mut bpc: core::ffi::c_int,
) {
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][WHT_WHT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_wht_wht_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_4X4 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_4x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X8 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_4x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_4X16 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_4x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X4 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_8x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_8X8 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_8x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X16 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_8x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X32 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_8x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_8X32 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_8x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X4 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_16x4_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][H_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_identity_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][V_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_flipadst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][H_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_identity_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X8 as core::ffi::c_int
        as usize][V_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_adst_16x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][DCT_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_dct_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][ADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_adst_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][ADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_adst_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][ADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_adst_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][FLIPADST_ADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_adst_flipadst_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][DCT_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_dct_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][FLIPADST_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_flipadst_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][FLIPADST_FLIPADST as core::ffi::c_int as usize] = Some(
        inv_txfm_add_flipadst_flipadst_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][H_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_identity_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_16X16 as core::ffi::c_int
        as usize][V_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_dct_16x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X32 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_16x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X32 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_16x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_16X64 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_16x64_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_32X8 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_32x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_32X8 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_32x8_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_32X16 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_32x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_32X16 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_32x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_32X32 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_32x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_32X32 as core::ffi::c_int
        as usize][IDTX as core::ffi::c_int as usize] = Some(
        inv_txfm_add_identity_identity_32x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_32X64 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_32x64_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_64X16 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_64x16_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[RTX_64X32 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_64x32_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    (*c)
        .itxfm_add[TX_64X64 as core::ffi::c_int
        as usize][DCT_DCT as core::ffi::c_int as usize] = Some(
        inv_txfm_add_dct_dct_64x64_c
            as unsafe extern "C" fn(
                *mut pixel,
                ptrdiff_t,
                *mut coef,
                core::ffi::c_int,
            ) -> (),
    ) as itxfm_fn;
    let mut all_simd: core::ffi::c_int = 0 as core::ffi::c_int;
    if all_simd == 0 {
        dav1d_init_last_nonzero_col_from_eob_tables();
    }
}
