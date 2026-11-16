// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `itx_dsp_init_arm` of `upstream/dav1d/src/arm/itx.h` for 8 bpc, limited to
//! the entry points rebuilt so far (`simd/arm64/itx.rs`). The original
//! assigns, under `DAV1D_ARM_CPU_FLAG_NEON`, every transform of every size
//! and then sets `*all_simd = 1`; until every one is rebuilt the remaining
//! slots keep their C kernels and `all_simd` stays 0, so the C kernels'
//! `dav1d_init_last_nonzero_col_from_eob_tables` still runs.
use core::ffi::{c_int, c_void};

use super::super::arm64::itx::*;
use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};

/// `itxfm_fn` (src/itx.h) at 8 bpc.
pub type ItxFn = unsafe extern "C" fn(*mut u8, isize, *mut i16, c_int);

// enum RectTxfmSize and enum TxfmType (src/levels.h)
const TX_4X4: usize = 0;
const TX_8X8: usize = 1;
const TX_16X16: usize = 2;
const RTX_4X8: usize = 5;
const RTX_8X4: usize = 6;
const RTX_8X16: usize = 7;
const RTX_16X8: usize = 8;
const RTX_4X16: usize = 13;
const RTX_16X4: usize = 14;
const DCT_DCT: usize = 0;
const IDTX: usize = 9;

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
/// (same layout, checked by a test) and is not otherwise accessed during the
/// call.
pub unsafe fn itx_dsp_init_arm_8bpc(c: *mut c_void, _bpc: c_int, _all_simd: *mut c_int) {
    let c = &mut *(c as *mut Dav1dInvTxfmDSPContext);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    // assign_itx16_fn( ,  4,  4, neon) … assign_itx12_fn( , 16, 16, neon):
    // the DCT_DCT and IDTX members rebuilt so far.
    let t = &mut c.itxfm_add;
    t[TX_4X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon);
    t[TX_4X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon);
    t[RTX_4X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon);
    t[RTX_4X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon);
    t[RTX_4X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon);
    t[RTX_4X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon);
    t[RTX_8X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon);
    t[RTX_8X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon);
    t[TX_8X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon);
    t[TX_8X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon);
    t[RTX_8X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon);
    t[RTX_8X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon);
    t[RTX_16X4][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon);
    t[RTX_16X4][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon);
    t[RTX_16X8][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon);
    t[RTX_16X8][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon);
    t[TX_16X16][DCT_DCT] = Some(dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon);
    t[TX_16X16][IDTX] = Some(dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The owned mirror and the translated context have the same layout,
    /// so the init can write the translated table through it.
    #[test]
    fn context_layout_matches_the_translation() {
        use crate::b8::itx_tmpl::Dav1dInvTxfmDSPContext as Translated;
        assert_eq!(core::mem::size_of::<Translated>(), core::mem::size_of::<Dav1dInvTxfmDSPContext>());
        assert_eq!(core::mem::align_of::<Translated>(), core::mem::align_of::<Dav1dInvTxfmDSPContext>());
        assert_eq!(core::mem::size_of::<Dav1dInvTxfmDSPContext>(), 19 * 17 * core::mem::size_of::<usize>());
    }

    /// With NEON allowed the rebuilt slots are selected, and with
    /// `dav1d_set_cpu_flags_mask(0)` none is (the scalar path).
    #[test]
    fn neon_selects_the_rebuilt_kernels_and_mask_zero_keeps_c() {
        use crate::b8::itx_tmpl::{dav1d_itx_dsp_init_8bpc, Dav1dInvTxfmDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let rebuilt = dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon as ItxFn as usize;
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
