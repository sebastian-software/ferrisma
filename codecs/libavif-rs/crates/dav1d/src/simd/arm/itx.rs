// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `itx_dsp_init_arm` of `upstream/dav1d/src/arm/itx.h` for 8 bpc: under
//! `DAV1D_ARM_CPU_FLAG_NEON` every slot gets its rebuilt kernel
//! (`simd/arm64/itx.rs`) and `*all_simd = 1`, so the C kernels' eob tables
//! are not initialized, as in the original. With the flag masked off the C
//! kernels stay and `all_simd` stays 0.
use core::ffi::{c_int, c_void};

use super::super::arm64::itx::*;
use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};

/// `itxfm_fn` (src/itx.h) at 8 bpc.
pub type ItxFn = unsafe extern "C" fn(*mut u8, isize, *mut i16, c_int);

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

/// Called by the translated `dav1d_itx_dsp_init_8bpc` where the original
/// calls `itx_dsp_init_arm(c, bpc, &all_simd)`.
///
/// # Safety
/// `c` points to the translated `Dav1dInvTxfmDSPContext` being initialized
/// (same layout, checked by a test) and `all_simd` to the init's local flag;
/// neither is otherwise accessed during the call.
pub unsafe fn itx_dsp_init_arm_8bpc(c: *mut c_void, _bpc: c_int, all_simd: *mut c_int) {
    let c = &mut *(c as *mut Dav1dInvTxfmDSPContext);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    // assign_itx_fn( , 4, 4, wht_wht, WHT_WHT, neon) and, for 8 bpc,
    // assign_itx16_fn … assign_itx1_fn ( , 64, 64, neon) (src/arm/itx.h).
    let t = &mut c.itxfm_add;
    t[TX_4X4][WHT_WHT] = Some(dav1d_inv_txfm_add_wht_wht_4x4_8bpc_neon);
    t[TX_4X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon);
    t[TX_4X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon);
    t[TX_4X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x4_8bpc_neon);
    t[TX_4X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_neon);
    t[TX_4X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x4_8bpc_neon);
    t[TX_4X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x4_8bpc_neon);
    t[TX_4X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x4_8bpc_neon);
    t[TX_4X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_neon);
    t[TX_4X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_neon);
    t[TX_4X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_neon);
    t[TX_4X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_neon);
    t[TX_4X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x4_8bpc_neon);
    t[TX_4X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x4_8bpc_neon);
    t[TX_4X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_neon);
    t[TX_4X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x4_8bpc_neon);
    t[TX_4X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_neon);
    t[RTX_4X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon);
    t[RTX_4X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon);
    t[RTX_4X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x8_8bpc_neon);
    t[RTX_4X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_neon);
    t[RTX_4X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x8_8bpc_neon);
    t[RTX_4X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x8_8bpc_neon);
    t[RTX_4X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x8_8bpc_neon);
    t[RTX_4X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_neon);
    t[RTX_4X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_neon);
    t[RTX_4X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_neon);
    t[RTX_4X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_neon);
    t[RTX_4X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x8_8bpc_neon);
    t[RTX_4X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x8_8bpc_neon);
    t[RTX_4X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_neon);
    t[RTX_4X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x8_8bpc_neon);
    t[RTX_4X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_neon);
    t[RTX_8X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon);
    t[RTX_8X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon);
    t[RTX_8X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x4_8bpc_neon);
    t[RTX_8X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_neon);
    t[RTX_8X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x4_8bpc_neon);
    t[RTX_8X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x4_8bpc_neon);
    t[RTX_8X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x4_8bpc_neon);
    t[RTX_8X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_neon);
    t[RTX_8X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_neon);
    t[RTX_8X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_neon);
    t[RTX_8X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_neon);
    t[RTX_8X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x4_8bpc_neon);
    t[RTX_8X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x4_8bpc_neon);
    t[RTX_8X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_neon);
    t[RTX_8X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x4_8bpc_neon);
    t[RTX_8X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_neon);
    t[TX_8X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon);
    t[TX_8X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon);
    t[TX_8X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x8_8bpc_neon);
    t[TX_8X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_neon);
    t[TX_8X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x8_8bpc_neon);
    t[TX_8X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x8_8bpc_neon);
    t[TX_8X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x8_8bpc_neon);
    t[TX_8X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_neon);
    t[TX_8X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_neon);
    t[TX_8X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_neon);
    t[TX_8X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_neon);
    t[TX_8X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x8_8bpc_neon);
    t[TX_8X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x8_8bpc_neon);
    t[TX_8X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_neon);
    t[TX_8X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x8_8bpc_neon);
    t[TX_8X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_neon);
    t[RTX_4X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon);
    t[RTX_4X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon);
    t[RTX_4X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_4x16_8bpc_neon);
    t[RTX_4X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_neon);
    t[RTX_4X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_4x16_8bpc_neon);
    t[RTX_4X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_4x16_8bpc_neon);
    t[RTX_4X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_4x16_8bpc_neon);
    t[RTX_4X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_neon);
    t[RTX_4X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_neon);
    t[RTX_4X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_neon);
    t[RTX_4X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_neon);
    t[RTX_4X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_4x16_8bpc_neon);
    t[RTX_4X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_4x16_8bpc_neon);
    t[RTX_4X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_neon);
    t[RTX_4X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_4x16_8bpc_neon);
    t[RTX_4X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_neon);
    t[RTX_16X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon);
    t[RTX_16X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon);
    t[RTX_16X4][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x4_8bpc_neon);
    t[RTX_16X4][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_neon);
    t[RTX_16X4][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x4_8bpc_neon);
    t[RTX_16X4][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x4_8bpc_neon);
    t[RTX_16X4][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x4_8bpc_neon);
    t[RTX_16X4][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_neon);
    t[RTX_16X4][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_neon);
    t[RTX_16X4][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_neon);
    t[RTX_16X4][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_neon);
    t[RTX_16X4][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x4_8bpc_neon);
    t[RTX_16X4][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x4_8bpc_neon);
    t[RTX_16X4][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_neon);
    t[RTX_16X4][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x4_8bpc_neon);
    t[RTX_16X4][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_neon);
    t[RTX_8X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon);
    t[RTX_8X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon);
    t[RTX_8X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_8x16_8bpc_neon);
    t[RTX_8X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_neon);
    t[RTX_8X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_8x16_8bpc_neon);
    t[RTX_8X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_8x16_8bpc_neon);
    t[RTX_8X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_8x16_8bpc_neon);
    t[RTX_8X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_neon);
    t[RTX_8X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_neon);
    t[RTX_8X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_neon);
    t[RTX_8X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_neon);
    t[RTX_8X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_8x16_8bpc_neon);
    t[RTX_8X16][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_8x16_8bpc_neon);
    t[RTX_8X16][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_neon);
    t[RTX_8X16][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_8x16_8bpc_neon);
    t[RTX_8X16][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_neon);
    t[RTX_16X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon);
    t[RTX_16X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon);
    t[RTX_16X8][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x8_8bpc_neon);
    t[RTX_16X8][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_neon);
    t[RTX_16X8][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x8_8bpc_neon);
    t[RTX_16X8][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x8_8bpc_neon);
    t[RTX_16X8][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x8_8bpc_neon);
    t[RTX_16X8][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_neon);
    t[RTX_16X8][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_neon);
    t[RTX_16X8][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_neon);
    t[RTX_16X8][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_neon);
    t[RTX_16X8][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x8_8bpc_neon);
    t[RTX_16X8][H_ADST] = Some(dav1d_inv_txfm_add_adst_identity_16x8_8bpc_neon);
    t[RTX_16X8][H_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_neon);
    t[RTX_16X8][V_ADST] = Some(dav1d_inv_txfm_add_identity_adst_16x8_8bpc_neon);
    t[RTX_16X8][V_FLIPADST] = Some(dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_neon);
    t[TX_16X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon);
    t[TX_16X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon);
    t[TX_16X16][ADST_DCT] = Some(dav1d_inv_txfm_add_dct_adst_16x16_8bpc_neon);
    t[TX_16X16][FLIPADST_DCT] = Some(dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_neon);
    t[TX_16X16][H_DCT] = Some(dav1d_inv_txfm_add_dct_identity_16x16_8bpc_neon);
    t[TX_16X16][DCT_ADST] = Some(dav1d_inv_txfm_add_adst_dct_16x16_8bpc_neon);
    t[TX_16X16][ADST_ADST] = Some(dav1d_inv_txfm_add_adst_adst_16x16_8bpc_neon);
    t[TX_16X16][FLIPADST_ADST] = Some(dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_neon);
    t[TX_16X16][DCT_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_neon);
    t[TX_16X16][ADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_neon);
    t[TX_16X16][FLIPADST_FLIPADST] = Some(dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_neon);
    t[TX_16X16][V_DCT] = Some(dav1d_inv_txfm_add_identity_dct_16x16_8bpc_neon);
    t[RTX_8X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_neon);
    t[RTX_8X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x32_8bpc_neon);
    t[RTX_32X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_neon);
    t[RTX_32X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x8_8bpc_neon);
    t[RTX_16X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_neon);
    t[RTX_16X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x32_8bpc_neon);
    t[RTX_32X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_neon);
    t[RTX_32X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x16_8bpc_neon);
    t[TX_32X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_neon);
    t[TX_32X32][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_32x32_8bpc_neon);
    t[RTX_16X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_neon);
    t[RTX_32X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_neon);
    t[RTX_64X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_neon);
    t[RTX_64X32][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_neon);
    t[TX_64X64][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_neon);
    *all_simd = 1;
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

    /// With NEON allowed the rebuilt slots are selected, and with
    /// `dav1d_set_cpu_flags_mask(0)` none is (the scalar path).
    #[test]
    fn neon_selects_the_rebuilt_kernels_and_mask_zero_keeps_c() {
        use crate::b8::itx_tmpl::{dav1d_itx_dsp_init_8bpc, Dav1dInvTxfmDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let rebuilt = dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon as ItxFn as usize;
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let slot = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_itx_dsp_init_8bpc(&mut c, 8);
            dav1d_set_cpu_flags_mask(!0);
            c.itxfm_add[TX_8X8][DCT_DCT].map(|f| f as usize)
        };
        assert_eq!(slot(!0), Some(rebuilt));
        assert_ne!(slot(0), Some(rebuilt));
    }
}
