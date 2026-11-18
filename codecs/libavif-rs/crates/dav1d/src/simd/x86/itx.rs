// Copyright © 2018-2023, VideoLAN and dav1d authors
// Copyright © 2018-2023, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `itx_dsp_init_x86` of `upstream/dav1d/src/x86/itx.h` for x86-64, statement
//! by statement: under `DAV1D_X86_CPU_FLAG_AVX2` every slot that the
//! original assigns an AVX2 kernel gets the rebuilt one
//! (`simd/x86_64/itx_avx2.rs` at 8 bpc, `simd/x86_64/itx16_avx2.rs` at 10
//! and 12 bits); with the flags masked off the C kernels stay.
//!
//! On an AVX2 machine no slot keeps a lower level in the original: the AVX2
//! block of the header assigns every slot that its SSE2 (WHT), SSSE3 (8 bpc)
//! and SSE4.1 (10 bit) blocks assign, so the header preprocessed with
//! assembly leaves only AVX2 and C kernels there (ADR 0062 has the list).
//! The SSE2, SSSE3 and SSE4.1 kernels, which the original runs only on a CPU
//! without AVX2, and the AVX-512 kernels have no counterpart (ADR 0062,
//! "Levels"; engine ADR-0041 section 4): below AVX2 the C kernels stay.
//!
//! One consequence for `all_simd`: the original sets it with its SSSE3
//! assignments (8 bpc), which cover every slot. Here the C kernels stay below
//! AVX2, and they need the tables that `dav1d_itx_dsp_init` only builds while
//! `all_simd` is 0, so `*all_simd = 1` follows the AVX2 assignments instead.
//! On an AVX2 machine the value is the original's. At 16 bpc the original
//! sets it in its SSE4.1 block, at 10 bits only; here it follows the 10-bit
//! AVX2 assignments, which cover every slot, and stays 0 at 12 bits, where
//! the eight DCT_DCT slots of 16x32 and larger keep the C kernels (as in
//! the original).
use core::ffi::{c_int, c_void};

use super::super::cpu::{
    dav1d_get_cpu_flags, DAV1D_X86_CPU_FLAG_AVX2, DAV1D_X86_CPU_FLAG_SSE2, DAV1D_X86_CPU_FLAG_SSE41,
    DAV1D_X86_CPU_FLAG_SSSE3,
};
use super::super::x86_64::itx16_avx2::*;
use super::super::x86_64::itx_avx2::*;

/// `itxfm_fn` (src/itx.h) at 8 bpc.
pub type ItxFn = unsafe extern "C" fn(*mut u8, isize, *mut i16, c_int);
/// `itxfm_fn` (src/itx.h) at 16 bpc: `pixel` is `uint16_t`, `coef` is
/// `int32_t`, and the last argument is `bitdepth_max`.
pub type ItxFn16 = unsafe extern "C" fn(*mut u16, isize, *mut i32, c_int, c_int);

// enum RectTxfmSize and enum TxfmType (src/levels.h)
const TX_4X4: usize = 0;
const TX_8X8: usize = 1;
const TX_16X16: usize = 2;
const TX_32X32: usize = 3;
const TX_64X64: usize = 4;
const RTX_4X8: usize = 5;
const RTX_8X4: usize = 6;
const RTX_8X16: usize = 7;
const RTX_16X8: usize = 8;
const RTX_16X32: usize = 9;
const RTX_32X16: usize = 10;
const RTX_32X64: usize = 11;
const RTX_64X32: usize = 12;
const RTX_4X16: usize = 13;
const RTX_16X4: usize = 14;
const RTX_8X32: usize = 15;
const RTX_32X8: usize = 16;
const RTX_16X64: usize = 17;
const RTX_64X16: usize = 18;
const DCT_DCT: usize = 0;
const ADST_DCT: usize = 1;
const DCT_ADST: usize = 2;
const ADST_ADST: usize = 3;
const FLIPADST_DCT: usize = 4;
const DCT_FLIPADST: usize = 5;
const FLIPADST_FLIPADST: usize = 6;
const ADST_FLIPADST: usize = 7;
const FLIPADST_ADST: usize = 8;
const IDTX: usize = 9;
const V_DCT: usize = 10;
const H_DCT: usize = 11;
const V_ADST: usize = 12;
const H_ADST: usize = 13;
const V_FLIPADST: usize = 14;
const H_FLIPADST: usize = 15;
const WHT_WHT: usize = 16;

/// `Dav1dInvTxfmDSPContext` (src/itx.h): `itxfm_add[N_RECT_TX_SIZES][N_TX_TYPES_PLUS_LL]`.
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext {
    pub itxfm_add: [[Option<ItxFn>; 17]; 19],
}

/// `Dav1dInvTxfmDSPContext` (src/itx.h) at 16 bpc.
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext16 {
    pub itxfm_add: [[Option<ItxFn16>; 17]; 19],
}

/// Called by the translated `dav1d_itx_dsp_init_8bpc` where the original
/// calls `itx_dsp_init_x86(c, bpc, &all_simd)`.
///
/// # Safety
/// `c` points to the translated `Dav1dInvTxfmDSPContext` being initialized
/// (same layout, checked by a test) and `all_simd` to the init's local flag;
/// neither is otherwise accessed during the call.
pub unsafe fn itx_dsp_init_x86_8bpc(c: *mut c_void, _bpc: c_int, all_simd: *mut c_int) {
    let c = &mut *(c as *mut Dav1dInvTxfmDSPContext);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    // assign_itx_fn(, 4, 4, wht_wht, WHT_WHT, sse2): not rebuilt, the C
    // kernel stays (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    // BITDEPTH == 8: assign_itx16_fn(, 4, 4, ssse3) … assign_itx1_fn( , 64,
    // 64, ssse3) and `*all_simd = 1`: not rebuilt, the C kernels stay and
    // `all_simd` stays 0 (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    // BITDEPTH == 16 only.

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    #[cfg(dav1d_itx_pilot)]
    let before = c.itxfm_add;
    let t = &mut c.itxfm_add;
    // assign_itx_fn(, 4, 4, wht_wht, WHT_WHT, avx2);
    t[TX_4X4][WHT_WHT] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_avx2);

    // assign_itx16_fn( ,  4,  4, avx2);
    t[TX_4X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx2);
    t[TX_4X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx2);
    t[TX_4X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx2);
    t[TX_4X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx2);
    t[TX_4X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx2);
    t[TX_4X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx2);
    t[TX_4X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx2);
    t[TX_4X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx2);
    t[TX_4X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx2);
    t[TX_4X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx2);
    t[TX_4X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx2);
    t[TX_4X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx2);
    t[TX_4X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx2);
    t[TX_4X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx2);
    t[TX_4X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx2);
    t[TX_4X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx2);
    // assign_itx16_fn(R,  4,  8, avx2);
    t[RTX_4X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx2);
    t[RTX_4X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx2);
    t[RTX_4X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx2);
    t[RTX_4X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx2);
    t[RTX_4X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx2);
    t[RTX_4X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx2);
    t[RTX_4X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2);
    t[RTX_4X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx2);
    t[RTX_4X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx2);
    t[RTX_4X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx2);
    t[RTX_4X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx2);
    t[RTX_4X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx2);
    t[RTX_4X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx2);
    t[RTX_4X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx2);
    t[RTX_4X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx2);
    t[RTX_4X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx2);
    // assign_itx16_fn(R,  4, 16, avx2);
    t[RTX_4X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx2);
    t[RTX_4X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx2);
    t[RTX_4X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx2);
    t[RTX_4X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx2);
    t[RTX_4X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx2);
    t[RTX_4X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx2);
    t[RTX_4X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx2);
    t[RTX_4X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx2);
    t[RTX_4X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx2);
    t[RTX_4X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx2);
    t[RTX_4X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx2);
    t[RTX_4X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx2);
    t[RTX_4X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx2);
    t[RTX_4X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx2);
    t[RTX_4X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx2);
    t[RTX_4X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx2);
    // assign_itx16_fn(R,  8,  4, avx2);
    t[RTX_8X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx2);
    t[RTX_8X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx2);
    t[RTX_8X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx2);
    t[RTX_8X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx2);
    t[RTX_8X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx2);
    t[RTX_8X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx2);
    t[RTX_8X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx2);
    t[RTX_8X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx2);
    t[RTX_8X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx2);
    t[RTX_8X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx2);
    t[RTX_8X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx2);
    t[RTX_8X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx2);
    t[RTX_8X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx2);
    t[RTX_8X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx2);
    t[RTX_8X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx2);
    t[RTX_8X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx2);
    // assign_itx16_fn( ,  8,  8, avx2);
    t[TX_8X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx2);
    t[TX_8X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx2);
    t[TX_8X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx2);
    t[TX_8X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx2);
    t[TX_8X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx2);
    t[TX_8X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx2);
    t[TX_8X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx2);
    t[TX_8X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx2);
    t[TX_8X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx2);
    t[TX_8X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx2);
    t[TX_8X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx2);
    t[TX_8X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx2);
    t[TX_8X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx2);
    t[TX_8X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx2);
    t[TX_8X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx2);
    t[TX_8X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx2);
    // assign_itx16_fn(R,  8, 16, avx2);
    t[RTX_8X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx2);
    t[RTX_8X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx2);
    t[RTX_8X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx2);
    t[RTX_8X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx2);
    t[RTX_8X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx2);
    t[RTX_8X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx2);
    t[RTX_8X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx2);
    t[RTX_8X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx2);
    t[RTX_8X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx2);
    t[RTX_8X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx2);
    t[RTX_8X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx2);
    t[RTX_8X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx2);
    t[RTX_8X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx2);
    t[RTX_8X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx2);
    t[RTX_8X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx2);
    t[RTX_8X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx2);
    // assign_itx2_fn(R,  8, 32, avx2);
    t[RTX_8X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx2);
    t[RTX_8X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx2);
    // assign_itx16_fn(R, 16,  4, avx2);
    t[RTX_16X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx2);
    t[RTX_16X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx2);
    t[RTX_16X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx2);
    t[RTX_16X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx2);
    t[RTX_16X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx2);
    t[RTX_16X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx2);
    t[RTX_16X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx2);
    t[RTX_16X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx2);
    t[RTX_16X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx2);
    t[RTX_16X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx2);
    t[RTX_16X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx2);
    t[RTX_16X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx2);
    t[RTX_16X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx2);
    t[RTX_16X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx2);
    t[RTX_16X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx2);
    t[RTX_16X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx2);
    // assign_itx16_fn(R, 16,  8, avx2);
    t[RTX_16X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx2);
    t[RTX_16X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx2);
    t[RTX_16X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx2);
    t[RTX_16X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx2);
    t[RTX_16X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx2);
    t[RTX_16X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx2);
    t[RTX_16X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx2);
    t[RTX_16X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx2);
    t[RTX_16X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx2);
    t[RTX_16X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx2);
    t[RTX_16X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx2);
    t[RTX_16X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx2);
    t[RTX_16X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx2);
    t[RTX_16X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx2);
    t[RTX_16X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx2);
    t[RTX_16X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx2);
    // assign_itx12_fn( , 16, 16, avx2);
    t[TX_16X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx2);
    t[TX_16X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx2);
    t[TX_16X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx2);
    t[TX_16X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx2);
    t[TX_16X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx2);
    t[TX_16X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx2);
    t[TX_16X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx2);
    t[TX_16X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx2);
    t[TX_16X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx2);
    t[TX_16X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx2);
    t[TX_16X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx2);
    t[TX_16X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx2);
    // assign_itx2_fn(R, 16, 32, avx2);
    t[RTX_16X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx2);
    t[RTX_16X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx2);
    // assign_itx1_fn(R, 16, 64, avx2);
    t[RTX_16X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx2);
    // assign_itx2_fn(R, 32,  8, avx2);
    t[RTX_32X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx2);
    t[RTX_32X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx2);
    // assign_itx2_fn(R, 32, 16, avx2);
    t[RTX_32X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx2);
    t[RTX_32X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx2);
    // assign_itx2_fn( , 32, 32, avx2);
    t[TX_32X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx2);
    t[TX_32X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx2);
    // assign_itx1_fn(R, 32, 64, avx2);
    t[RTX_32X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx2);
    // assign_itx1_fn(R, 64, 16, avx2);
    t[RTX_64X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx2);
    // assign_itx1_fn(R, 64, 32, avx2);
    t[RTX_64X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx2);
    // assign_itx1_fn( , 64, 64, avx2);
    t[TX_64X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx2);
    // The original's `*all_simd = 1` of the SSSE3 block (module
    // documentation): every slot now holds a rebuilt kernel.
    *all_simd = 1;

    #[cfg(dav1d_itx_pilot)]
    crate::simd::itx_pilot::install_8bpc(&mut c.itxfm_add, &before);

    // if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return; and the
    // avx512icl assignments: above the scope (module documentation).
}

/// Called by the translated `dav1d_itx_dsp_init_16bpc` where the original
/// calls `itx_dsp_init_x86(c, bpc, &all_simd)`.
///
/// # Safety
/// `c` points to the translated 16 bpc `Dav1dInvTxfmDSPContext` being
/// initialized (same layout, checked by a test) and `all_simd` to the
/// init's local flag; neither is otherwise accessed during the call.
pub unsafe fn itx_dsp_init_x86_16bpc(c: *mut c_void, bpc: c_int, all_simd: *mut c_int) {
    let c = &mut *(c as *mut Dav1dInvTxfmDSPContext16);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    // assign_itx_fn(, 4, 4, wht_wht, WHT_WHT, sse2): not rebuilt, the C
    // kernel stays (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    // BITDEPTH == 8 only.

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    // BITDEPTH == 16, bpc == 10: assign_itx16_fn(, 4, 4, sse4) …
    // assign_itx1_fn(, 64, 64, sse4) and `*all_simd = 1`: not rebuilt, the C
    // kernels stay and `all_simd` stays 0 (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    #[cfg(dav1d_itx_pilot)]
    let before = c.itxfm_add;
    let t = &mut c.itxfm_add;
    // assign_itx_fn(, 4, 4, wht_wht, WHT_WHT, avx2);
    t[TX_4X4][WHT_WHT] = Some(dav1d_inv_txfm_add_wht_wht_4x4_16bpc_avx2);

    if bpc == 10 {
        // assign_itx16_bpc_fn( ,  4,  4, 10, avx2);
        t[TX_4X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x4_10bpc_avx2);
        t[TX_4X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x4_10bpc_avx2);
        t[TX_4X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x4_10bpc_avx2);
        t[TX_4X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_10bpc_avx2);
        t[TX_4X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x4_10bpc_avx2);
        t[TX_4X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x4_10bpc_avx2);
        t[TX_4X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x4_10bpc_avx2);
        t[TX_4X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_10bpc_avx2);
        t[TX_4X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_10bpc_avx2);
        t[TX_4X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_10bpc_avx2);
        t[TX_4X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_10bpc_avx2);
        t[TX_4X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x4_10bpc_avx2);
        t[TX_4X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x4_10bpc_avx2);
        t[TX_4X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_10bpc_avx2);
        t[TX_4X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x4_10bpc_avx2);
        t[TX_4X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_10bpc_avx2);
        // assign_itx16_bpc_fn(R,  4,  8, 10, avx2);
        t[RTX_4X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x8_10bpc_avx2);
        t[RTX_4X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x8_10bpc_avx2);
        t[RTX_4X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x8_10bpc_avx2);
        t[RTX_4X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_10bpc_avx2);
        t[RTX_4X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x8_10bpc_avx2);
        t[RTX_4X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x8_10bpc_avx2);
        t[RTX_4X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x8_10bpc_avx2);
        t[RTX_4X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_10bpc_avx2);
        t[RTX_4X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_10bpc_avx2);
        t[RTX_4X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_10bpc_avx2);
        t[RTX_4X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_10bpc_avx2);
        t[RTX_4X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x8_10bpc_avx2);
        t[RTX_4X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x8_10bpc_avx2);
        t[RTX_4X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_10bpc_avx2);
        t[RTX_4X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x8_10bpc_avx2);
        t[RTX_4X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_10bpc_avx2);
        // assign_itx16_bpc_fn(R,  4, 16, 10, avx2);
        t[RTX_4X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x16_10bpc_avx2);
        t[RTX_4X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x16_10bpc_avx2);
        t[RTX_4X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x16_10bpc_avx2);
        t[RTX_4X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_10bpc_avx2);
        t[RTX_4X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x16_10bpc_avx2);
        t[RTX_4X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x16_10bpc_avx2);
        t[RTX_4X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x16_10bpc_avx2);
        t[RTX_4X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_10bpc_avx2);
        t[RTX_4X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_10bpc_avx2);
        t[RTX_4X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_10bpc_avx2);
        t[RTX_4X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_10bpc_avx2);
        t[RTX_4X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x16_10bpc_avx2);
        t[RTX_4X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x16_10bpc_avx2);
        t[RTX_4X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_10bpc_avx2);
        t[RTX_4X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x16_10bpc_avx2);
        t[RTX_4X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_10bpc_avx2);
        // assign_itx16_bpc_fn(R,  8,  4, 10, avx2);
        t[RTX_8X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x4_10bpc_avx2);
        t[RTX_8X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x4_10bpc_avx2);
        t[RTX_8X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x4_10bpc_avx2);
        t[RTX_8X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_10bpc_avx2);
        t[RTX_8X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x4_10bpc_avx2);
        t[RTX_8X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x4_10bpc_avx2);
        t[RTX_8X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x4_10bpc_avx2);
        t[RTX_8X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_10bpc_avx2);
        t[RTX_8X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_10bpc_avx2);
        t[RTX_8X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_10bpc_avx2);
        t[RTX_8X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_10bpc_avx2);
        t[RTX_8X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x4_10bpc_avx2);
        t[RTX_8X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x4_10bpc_avx2);
        t[RTX_8X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_10bpc_avx2);
        t[RTX_8X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x4_10bpc_avx2);
        t[RTX_8X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_10bpc_avx2);
        // assign_itx16_bpc_fn( ,  8,  8, 10, avx2);
        t[TX_8X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x8_10bpc_avx2);
        t[TX_8X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x8_10bpc_avx2);
        t[TX_8X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x8_10bpc_avx2);
        t[TX_8X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_10bpc_avx2);
        t[TX_8X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x8_10bpc_avx2);
        t[TX_8X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x8_10bpc_avx2);
        t[TX_8X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x8_10bpc_avx2);
        t[TX_8X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_10bpc_avx2);
        t[TX_8X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_10bpc_avx2);
        t[TX_8X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_10bpc_avx2);
        t[TX_8X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_10bpc_avx2);
        t[TX_8X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x8_10bpc_avx2);
        t[TX_8X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x8_10bpc_avx2);
        t[TX_8X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_10bpc_avx2);
        t[TX_8X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x8_10bpc_avx2);
        t[TX_8X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_10bpc_avx2);
        // assign_itx16_bpc_fn(R,  8, 16, 10, avx2);
        t[RTX_8X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x16_10bpc_avx2);
        t[RTX_8X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x16_10bpc_avx2);
        t[RTX_8X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x16_10bpc_avx2);
        t[RTX_8X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_10bpc_avx2);
        t[RTX_8X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x16_10bpc_avx2);
        t[RTX_8X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x16_10bpc_avx2);
        t[RTX_8X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x16_10bpc_avx2);
        t[RTX_8X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_10bpc_avx2);
        t[RTX_8X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_10bpc_avx2);
        t[RTX_8X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_10bpc_avx2);
        t[RTX_8X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_10bpc_avx2);
        t[RTX_8X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x16_10bpc_avx2);
        t[RTX_8X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x16_10bpc_avx2);
        t[RTX_8X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_10bpc_avx2);
        t[RTX_8X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x16_10bpc_avx2);
        t[RTX_8X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_10bpc_avx2);
        // assign_itx2_bpc_fn(R,  8, 32, 10, avx2);
        t[RTX_8X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x32_10bpc_avx2);
        t[RTX_8X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x32_10bpc_avx2);
        // assign_itx16_bpc_fn(R, 16,  4, 10, avx2);
        t[RTX_16X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x4_10bpc_avx2);
        t[RTX_16X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x4_10bpc_avx2);
        t[RTX_16X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x4_10bpc_avx2);
        t[RTX_16X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_10bpc_avx2);
        t[RTX_16X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x4_10bpc_avx2);
        t[RTX_16X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x4_10bpc_avx2);
        t[RTX_16X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x4_10bpc_avx2);
        t[RTX_16X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_10bpc_avx2);
        t[RTX_16X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_10bpc_avx2);
        t[RTX_16X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_10bpc_avx2);
        t[RTX_16X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_10bpc_avx2);
        t[RTX_16X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x4_10bpc_avx2);
        t[RTX_16X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x4_10bpc_avx2);
        t[RTX_16X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_10bpc_avx2);
        t[RTX_16X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x4_10bpc_avx2);
        t[RTX_16X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_10bpc_avx2);
        // assign_itx16_bpc_fn(R, 16,  8, 10, avx2);
        t[RTX_16X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x8_10bpc_avx2);
        t[RTX_16X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x8_10bpc_avx2);
        t[RTX_16X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x8_10bpc_avx2);
        t[RTX_16X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_10bpc_avx2);
        t[RTX_16X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x8_10bpc_avx2);
        t[RTX_16X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x8_10bpc_avx2);
        t[RTX_16X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x8_10bpc_avx2);
        t[RTX_16X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_10bpc_avx2);
        t[RTX_16X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_10bpc_avx2);
        t[RTX_16X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_10bpc_avx2);
        t[RTX_16X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_10bpc_avx2);
        t[RTX_16X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x8_10bpc_avx2);
        t[RTX_16X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x8_10bpc_avx2);
        t[RTX_16X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_10bpc_avx2);
        t[RTX_16X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x8_10bpc_avx2);
        t[RTX_16X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_10bpc_avx2);
        // assign_itx12_bpc_fn( , 16, 16, 10, avx2);
        t[TX_16X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x16_10bpc_avx2);
        t[TX_16X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x16_10bpc_avx2);
        t[TX_16X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x16_10bpc_avx2);
        t[TX_16X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_10bpc_avx2);
        t[TX_16X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x16_10bpc_avx2);
        t[TX_16X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x16_10bpc_avx2);
        t[TX_16X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x16_10bpc_avx2);
        t[TX_16X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_10bpc_avx2);
        t[TX_16X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_10bpc_avx2);
        t[TX_16X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_10bpc_avx2);
        t[TX_16X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_10bpc_avx2);
        t[TX_16X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x16_10bpc_avx2);
        // assign_itx2_bpc_fn(R, 16, 32, 10, avx2);
        t[RTX_16X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x32_10bpc_avx2);
        t[RTX_16X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x32_10bpc_avx2);
        // assign_itx1_bpc_fn(R, 16, 64, 10, avx2);
        t[RTX_16X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x64_10bpc_avx2);
        // assign_itx2_bpc_fn(R, 32,  8, 10, avx2);
        t[RTX_32X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x8_10bpc_avx2);
        t[RTX_32X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x8_10bpc_avx2);
        // assign_itx2_bpc_fn(R, 32, 16, 10, avx2);
        t[RTX_32X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x16_10bpc_avx2);
        t[RTX_32X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x16_10bpc_avx2);
        // assign_itx2_bpc_fn( , 32, 32, 10, avx2);
        t[TX_32X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x32_10bpc_avx2);
        t[TX_32X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x32_10bpc_avx2);
        // assign_itx1_bpc_fn(R, 32, 64, 10, avx2);
        t[RTX_32X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x64_10bpc_avx2);
        // assign_itx1_bpc_fn(R, 64, 16, 10, avx2);
        t[RTX_64X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x16_10bpc_avx2);
        // assign_itx1_bpc_fn(R, 64, 32, 10, avx2);
        t[RTX_64X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x32_10bpc_avx2);
        // assign_itx1_bpc_fn( , 64, 64, 10, avx2);
        t[TX_64X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x64_10bpc_avx2);

        // The original's `*all_simd = 1` of the SSE4.1 block at 10 bits
        // (module documentation): every slot now holds a rebuilt kernel.
        *all_simd = 1;
    } else {
        // assign_itx16_bpc_fn( ,  4,  4, 12, avx2);
        t[TX_4X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x4_12bpc_avx2);
        t[TX_4X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x4_12bpc_avx2);
        t[TX_4X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x4_12bpc_avx2);
        t[TX_4X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_12bpc_avx2);
        t[TX_4X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x4_12bpc_avx2);
        t[TX_4X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x4_12bpc_avx2);
        t[TX_4X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x4_12bpc_avx2);
        t[TX_4X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_12bpc_avx2);
        t[TX_4X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_12bpc_avx2);
        t[TX_4X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_12bpc_avx2);
        t[TX_4X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_12bpc_avx2);
        t[TX_4X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x4_12bpc_avx2);
        t[TX_4X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x4_12bpc_avx2);
        t[TX_4X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_12bpc_avx2);
        t[TX_4X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x4_12bpc_avx2);
        t[TX_4X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_12bpc_avx2);
        // assign_itx16_bpc_fn(R,  4,  8, 12, avx2);
        t[RTX_4X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x8_12bpc_avx2);
        t[RTX_4X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x8_12bpc_avx2);
        t[RTX_4X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x8_12bpc_avx2);
        t[RTX_4X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_12bpc_avx2);
        t[RTX_4X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x8_12bpc_avx2);
        t[RTX_4X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x8_12bpc_avx2);
        t[RTX_4X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x8_12bpc_avx2);
        t[RTX_4X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_12bpc_avx2);
        t[RTX_4X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_12bpc_avx2);
        t[RTX_4X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_12bpc_avx2);
        t[RTX_4X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_12bpc_avx2);
        t[RTX_4X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x8_12bpc_avx2);
        t[RTX_4X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x8_12bpc_avx2);
        t[RTX_4X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_12bpc_avx2);
        t[RTX_4X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x8_12bpc_avx2);
        t[RTX_4X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_12bpc_avx2);
        // assign_itx16_bpc_fn(R,  4, 16, 12, avx2);
        t[RTX_4X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x16_12bpc_avx2);
        t[RTX_4X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x16_12bpc_avx2);
        t[RTX_4X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x16_12bpc_avx2);
        t[RTX_4X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_12bpc_avx2);
        t[RTX_4X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x16_12bpc_avx2);
        t[RTX_4X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x16_12bpc_avx2);
        t[RTX_4X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x16_12bpc_avx2);
        t[RTX_4X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_12bpc_avx2);
        t[RTX_4X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_12bpc_avx2);
        t[RTX_4X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_12bpc_avx2);
        t[RTX_4X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_12bpc_avx2);
        t[RTX_4X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x16_12bpc_avx2);
        t[RTX_4X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x16_12bpc_avx2);
        t[RTX_4X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_12bpc_avx2);
        t[RTX_4X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x16_12bpc_avx2);
        t[RTX_4X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_12bpc_avx2);
        // assign_itx16_bpc_fn(R,  8,  4, 12, avx2);
        t[RTX_8X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x4_12bpc_avx2);
        t[RTX_8X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x4_12bpc_avx2);
        t[RTX_8X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x4_12bpc_avx2);
        t[RTX_8X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_12bpc_avx2);
        t[RTX_8X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x4_12bpc_avx2);
        t[RTX_8X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x4_12bpc_avx2);
        t[RTX_8X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x4_12bpc_avx2);
        t[RTX_8X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_12bpc_avx2);
        t[RTX_8X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_12bpc_avx2);
        t[RTX_8X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_12bpc_avx2);
        t[RTX_8X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_12bpc_avx2);
        t[RTX_8X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x4_12bpc_avx2);
        t[RTX_8X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x4_12bpc_avx2);
        t[RTX_8X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_12bpc_avx2);
        t[RTX_8X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x4_12bpc_avx2);
        t[RTX_8X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_12bpc_avx2);
        // assign_itx16_bpc_fn( ,  8,  8, 12, avx2);
        t[TX_8X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x8_12bpc_avx2);
        t[TX_8X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x8_12bpc_avx2);
        t[TX_8X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x8_12bpc_avx2);
        t[TX_8X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_12bpc_avx2);
        t[TX_8X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x8_12bpc_avx2);
        t[TX_8X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x8_12bpc_avx2);
        t[TX_8X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x8_12bpc_avx2);
        t[TX_8X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_12bpc_avx2);
        t[TX_8X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_12bpc_avx2);
        t[TX_8X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_12bpc_avx2);
        t[TX_8X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_12bpc_avx2);
        t[TX_8X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x8_12bpc_avx2);
        t[TX_8X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x8_12bpc_avx2);
        t[TX_8X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_12bpc_avx2);
        t[TX_8X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x8_12bpc_avx2);
        t[TX_8X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_12bpc_avx2);
        // assign_itx16_bpc_fn(R,  8, 16, 12, avx2);
        t[RTX_8X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x16_12bpc_avx2);
        t[RTX_8X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x16_12bpc_avx2);
        t[RTX_8X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x16_12bpc_avx2);
        t[RTX_8X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_12bpc_avx2);
        t[RTX_8X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x16_12bpc_avx2);
        t[RTX_8X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x16_12bpc_avx2);
        t[RTX_8X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x16_12bpc_avx2);
        t[RTX_8X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_12bpc_avx2);
        t[RTX_8X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_12bpc_avx2);
        t[RTX_8X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_12bpc_avx2);
        t[RTX_8X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_12bpc_avx2);
        t[RTX_8X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x16_12bpc_avx2);
        t[RTX_8X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x16_12bpc_avx2);
        t[RTX_8X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_12bpc_avx2);
        t[RTX_8X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x16_12bpc_avx2);
        t[RTX_8X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_12bpc_avx2);
        // assign_itx2_bpc_fn(R,  8, 32, 12, avx2);
        t[RTX_8X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x32_12bpc_avx2);
        t[RTX_8X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x32_12bpc_avx2);
        // assign_itx16_bpc_fn(R, 16,  4, 12, avx2);
        t[RTX_16X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x4_12bpc_avx2);
        t[RTX_16X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x4_12bpc_avx2);
        t[RTX_16X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x4_12bpc_avx2);
        t[RTX_16X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_12bpc_avx2);
        t[RTX_16X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x4_12bpc_avx2);
        t[RTX_16X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x4_12bpc_avx2);
        t[RTX_16X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x4_12bpc_avx2);
        t[RTX_16X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_12bpc_avx2);
        t[RTX_16X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_12bpc_avx2);
        t[RTX_16X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_12bpc_avx2);
        t[RTX_16X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_12bpc_avx2);
        t[RTX_16X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x4_12bpc_avx2);
        t[RTX_16X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x4_12bpc_avx2);
        t[RTX_16X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_12bpc_avx2);
        t[RTX_16X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x4_12bpc_avx2);
        t[RTX_16X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_12bpc_avx2);
        // assign_itx16_bpc_fn(R, 16,  8, 12, avx2);
        t[RTX_16X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x8_12bpc_avx2);
        t[RTX_16X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x8_12bpc_avx2);
        t[RTX_16X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x8_12bpc_avx2);
        t[RTX_16X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_12bpc_avx2);
        t[RTX_16X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x8_12bpc_avx2);
        t[RTX_16X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x8_12bpc_avx2);
        t[RTX_16X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x8_12bpc_avx2);
        t[RTX_16X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_12bpc_avx2);
        t[RTX_16X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_12bpc_avx2);
        t[RTX_16X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_12bpc_avx2);
        t[RTX_16X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_12bpc_avx2);
        t[RTX_16X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x8_12bpc_avx2);
        t[RTX_16X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x8_12bpc_avx2);
        t[RTX_16X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_12bpc_avx2);
        t[RTX_16X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x8_12bpc_avx2);
        t[RTX_16X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_12bpc_avx2);
        // assign_itx12_bpc_fn( , 16, 16, 12, avx2);
        t[TX_16X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x16_12bpc_avx2);
        t[TX_16X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x16_12bpc_avx2);
        t[TX_16X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x16_12bpc_avx2);
        t[TX_16X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_12bpc_avx2);
        t[TX_16X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x16_12bpc_avx2);
        t[TX_16X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x16_12bpc_avx2);
        t[TX_16X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x16_12bpc_avx2);
        t[TX_16X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_12bpc_avx2);
        t[TX_16X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_12bpc_avx2);
        t[TX_16X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_12bpc_avx2);
        t[TX_16X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_12bpc_avx2);
        t[TX_16X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x16_12bpc_avx2);
        // assign_itx2_bpc_fn(R, 32,  8, 12, avx2);
        t[RTX_32X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x8_12bpc_avx2);
        t[RTX_32X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x8_12bpc_avx2);
        // assign_itx_bpc_fn(R, 16, 32, identity_identity, IDTX, 12, avx2);
        t[RTX_16X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x32_12bpc_avx2);
        // assign_itx_bpc_fn(R, 32, 16, identity_identity, IDTX, 12, avx2);
        t[RTX_32X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x16_12bpc_avx2);
        // assign_itx_bpc_fn(, 32, 32, identity_identity, IDTX, 12, avx2);
        t[TX_32X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x32_12bpc_avx2);
    }

    #[cfg(dav1d_itx_pilot)]
    crate::simd::itx_pilot::install_16bpc(&mut c.itxfm_add, &before);

    // if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return; and the
    // avx512icl assignments: above the scope (module documentation).
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owned mirror and the translated context have the same layout,
    /// so the init can write the translated table through it.
    #[test]
    fn context_layout_matches_the_translation() {
        use crate::b8::itx_tmpl::Dav1dInvTxfmDSPContext as Translated;
        assert_eq!(
            core::mem::size_of::<Translated>(),
            core::mem::size_of::<Dav1dInvTxfmDSPContext>()
        );
        assert_eq!(
            core::mem::align_of::<Translated>(),
            core::mem::align_of::<Dav1dInvTxfmDSPContext>()
        );
        assert_eq!(
            core::mem::size_of::<Dav1dInvTxfmDSPContext>(),
            19 * 17 * core::mem::size_of::<usize>()
        );
    }

    /// The table after the translated init under a CPU mask.
    fn table8(mask: u32) -> [[Option<usize>; 17]; 19] {
        use crate::b8::itx_tmpl::{dav1d_itx_dsp_init_8bpc, Dav1dInvTxfmDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_itx_dsp_init_8bpc(&mut c, 8);
            dav1d_set_cpu_flags_mask(!0);
            c.itxfm_add.map(|row| row.map(|f| f.map(|f| f as usize)))
        }
    }

    /// With AVX2 allowed (and detected) the rebuilt kernels are selected,
    /// in all 156 slots that the original assigns; a mask below AVX2 keeps
    /// the C kernels (also where the original has SSE2 or SSSE3 kernels),
    /// the same ones as `dav1d_set_cpu_flags_mask(0)`.
    #[test]
    fn avx2_selects_the_rebuilt_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        let rebuilt = dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx2 as ItxFn as usize;
        let wht = dav1d_inv_txfm_add_wht_wht_4x4_8bpc_avx2 as ItxFn as usize;
        let all = table8(!0);
        let scalar = table8(0);
        assert_eq!(all[TX_8X8][DCT_DCT] == Some(rebuilt), avx2);
        assert_eq!(all[TX_4X4][WHT_WHT] == Some(wht), avx2);
        if avx2 {
            let changed = (0..19)
                .flat_map(|tx| (0..17).map(move |ty| (tx, ty)))
                .filter(|&(tx, ty)| all[tx][ty] != scalar[tx][ty])
                .count();
            assert_eq!(changed, 156);
        }
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | DAV1D_X86_CPU_FLAG_SSE41;
        assert_eq!(table8(below), scalar);
        assert_ne!(scalar[TX_8X8][DCT_DCT], Some(rebuilt));
    }

    /// The 16 bpc mirror and the translated context have the same layout.
    #[test]
    fn context16_layout_matches_the_translation() {
        use crate::b16::itx_tmpl::Dav1dInvTxfmDSPContext as Translated;
        assert_eq!(
            core::mem::size_of::<Translated>(),
            core::mem::size_of::<Dav1dInvTxfmDSPContext16>()
        );
        assert_eq!(
            core::mem::align_of::<Translated>(),
            core::mem::align_of::<Dav1dInvTxfmDSPContext16>()
        );
    }

    /// The 16 bpc table after the translated init at `bpc` under a CPU mask.
    fn table16(mask: u32, bpc: c_int) -> [[Option<usize>; 17]; 19] {
        use crate::b16::itx_tmpl::{dav1d_itx_dsp_init_16bpc, Dav1dInvTxfmDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_itx_dsp_init_16bpc(&mut c, bpc);
            dav1d_set_cpu_flags_mask(!0);
            c.itxfm_add.map(|row| row.map(|f| f.map(|f| f as usize)))
        }
    }

    /// With AVX2 allowed (and detected) the rebuilt 16 bpc kernels are
    /// selected, in the 156 slots that the original assigns at 10 bits and
    /// the 148 at 12 bits (the eight DCT_DCT slots of 16x32 and larger keep
    /// C); a mask below AVX2 keeps the C kernels (also where the original
    /// has SSE2 or SSE4.1 kernels), the same ones as
    /// `dav1d_set_cpu_flags_mask(0)`.
    #[test]
    fn avx2_selects_the_rebuilt_16bpc_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | DAV1D_X86_CPU_FLAG_SSE41;
        for (bpc, rebuilt, count) in [
            (10, dav1d_inv_txfm_add_dct_dct_8x8_10bpc_avx2 as ItxFn16 as usize, 156),
            (12, dav1d_inv_txfm_add_dct_dct_8x8_12bpc_avx2 as ItxFn16 as usize, 148),
        ] {
            let wht = dav1d_inv_txfm_add_wht_wht_4x4_16bpc_avx2 as ItxFn16 as usize;
            let all = table16(!0, bpc);
            let scalar = table16(0, bpc);
            assert_eq!(all[TX_8X8][DCT_DCT] == Some(rebuilt), avx2);
            assert_eq!(all[TX_4X4][WHT_WHT] == Some(wht), avx2);
            if avx2 {
                let changed = (0..19)
                    .flat_map(|tx| (0..17).map(move |ty| (tx, ty)))
                    .filter(|&(tx, ty)| all[tx][ty] != scalar[tx][ty])
                    .count();
                assert_eq!(changed, count);
                assert_eq!(all[TX_32X32][DCT_DCT] == scalar[TX_32X32][DCT_DCT], bpc == 12);
            }
            assert_eq!(table16(below, bpc), scalar);
            assert_ne!(scalar[TX_8X8][DCT_DCT], Some(rebuilt));
        }
    }
}
