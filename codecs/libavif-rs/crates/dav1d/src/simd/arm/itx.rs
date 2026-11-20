// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `itx_dsp_init_arm` of `upstream/dav1d/src/arm/itx.h` for 8 and 16 bpc,
//! statement by statement: under `DAV1D_ARM_CPU_FLAG_NEON` the slots that
//! the original assigns its NEON kernels get the inverse transforms written
//! from the algorithm (`simd/itx/`): at 8 bpc the 156 slots of the header,
//! and `*all_simd = 1`, so the C kernels' eob tables are not initialized, as in the original;
//! at 16 bpc WHT at every bit depth and every other slot (and `all_simd`)
//! only at 10 bits. With the flag masked off the C kernels stay and
//! `all_simd` stays 0.
use core::ffi::{c_int, c_void};

pub use super::super::itx::{ItxFn, ItxFn16};
use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};
use super::super::itx::*;

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

    let Some(k) = Kernels::new_8bpc() else {
        return;
    };
    let t = &mut c.itxfm_add;
    // assign_itx_fn( , 4, 4, wht_wht, WHT_WHT, neon);
    k.assign_wht(t);

    // assign_itx16_fn( ,  4,  4, neon) … assign_itx1_fn ( , 64, 64, neon),
    // one call per statement of src/arm/itx.h.
    k.assign(t, TX_4X4, ITX16);
    k.assign(t, RTX_4X8, ITX16);
    k.assign(t, RTX_4X16, ITX16);
    k.assign(t, RTX_8X4, ITX16);
    k.assign(t, TX_8X8, ITX16);
    k.assign(t, RTX_8X16, ITX16);
    k.assign(t, RTX_8X32, ITX2);
    k.assign(t, RTX_16X4, ITX16);
    k.assign(t, RTX_16X8, ITX16);
    k.assign(t, TX_16X16, ITX12);
    k.assign(t, RTX_16X32, ITX2);
    k.assign(t, RTX_16X64, ITX1);
    k.assign(t, RTX_32X8, ITX2);
    k.assign(t, RTX_32X16, ITX2);
    k.assign(t, TX_32X32, ITX2);
    k.assign(t, RTX_32X64, ITX1);
    k.assign(t, RTX_64X16, ITX1);
    k.assign(t, RTX_64X32, ITX1);
    k.assign(t, TX_64X64, ITX1);
    *all_simd = 1;
}

/// `Dav1dInvTxfmDSPContext` (src/itx.h) at 16 bpc.
#[repr(C)]
pub struct Dav1dInvTxfmDSPContext16 {
    pub itxfm_add: [[Option<ItxFn16>; 17]; 19],
}

/// Called by the translated `dav1d_itx_dsp_init_16bpc` where the original
/// calls `itx_dsp_init_arm(c, bpc, &all_simd)` at `BITDEPTH == 16`: WHT at
/// every bit depth, every other slot and `*all_simd = 1` only at 10 bits, as
/// in the original.
///
/// # Safety
/// `c` points to the translated 16 bpc `Dav1dInvTxfmDSPContext` being
/// initialized (same layout, checked by a test) and `all_simd` to the
/// init's local flag; neither is otherwise accessed during the call.
pub unsafe fn itx_dsp_init_arm_16bpc(c: *mut c_void, bpc: c_int, all_simd: *mut c_int) {
    let c = &mut *(c as *mut Dav1dInvTxfmDSPContext16);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    let Some(k) = Kernels::new_16bpc() else {
        return;
    };
    let t = &mut c.itxfm_add;
    // assign_itx_fn( , 4, 4, wht_wht, WHT_WHT, neon);
    k.assign_wht(t);

    // if (BITDEPTH == 16 && bpc != 10) return;
    if bpc != 10 {
        return;
    }

    // assign_itx16_fn( ,  4,  4, neon) … assign_itx1_fn ( , 64, 64, neon).
    k.assign(t, TX_4X4, ITX16);
    k.assign(t, RTX_4X8, ITX16);
    k.assign(t, RTX_4X16, ITX16);
    k.assign(t, RTX_8X4, ITX16);
    k.assign(t, TX_8X8, ITX16);
    k.assign(t, RTX_8X16, ITX16);
    k.assign(t, RTX_8X32, ITX2);
    k.assign(t, RTX_16X4, ITX16);
    k.assign(t, RTX_16X8, ITX16);
    k.assign(t, TX_16X16, ITX12);
    k.assign(t, RTX_16X32, ITX2);
    k.assign(t, RTX_16X64, ITX1);
    k.assign(t, RTX_32X8, ITX2);
    k.assign(t, RTX_32X16, ITX2);
    k.assign(t, TX_32X32, ITX2);
    k.assign(t, RTX_32X64, ITX1);
    k.assign(t, RTX_64X16, ITX1);
    k.assign(t, RTX_64X32, ITX1);
    k.assign(t, TX_64X64, ITX1);
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

    /// The slots of the translated 8 bpc init under `mask` that differ from
    /// the C kernels (`dav1d_set_cpu_flags_mask(0)`).
    fn changed8(mask: u32) -> Vec<(usize, usize)> {
        use crate::b8::itx_tmpl::{dav1d_itx_dsp_init_8bpc, Dav1dInvTxfmDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let table = |mask: u32| unsafe {
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_itx_dsp_init_8bpc(&mut c, 8);
            c.itxfm_add.map(|row| row.map(|f| f.map(|f| f as usize)))
        };
        unsafe { dav1d_init_cpu() };
        let (simd, scalar) = (table(mask), table(0));
        unsafe { dav1d_set_cpu_flags_mask(!0) };
        (0..19)
            .flat_map(|tx| (0..17).map(move |ty| (tx, ty)))
            .filter(|&(tx, ty)| simd[tx][ty] != scalar[tx][ty])
            .collect()
    }

    /// The slots of the original's NEON block (src/arm/itx.h) at 8 bpc and
    /// at 10 bits: WHT and the `assign_itx*_fn` statements.
    fn header_slots() -> Vec<(usize, usize)> {
        let mut slots = vec![(TX_4X4, WHT_WHT)];
        for (tx, types) in [
            (TX_4X4, ITX16),
            (RTX_4X8, ITX16),
            (RTX_4X16, ITX16),
            (RTX_8X4, ITX16),
            (TX_8X8, ITX16),
            (RTX_8X16, ITX16),
            (RTX_8X32, ITX2),
            (RTX_16X4, ITX16),
            (RTX_16X8, ITX16),
            (TX_16X16, ITX12),
            (RTX_16X32, ITX2),
            (RTX_16X64, ITX1),
            (RTX_32X8, ITX2),
            (RTX_32X16, ITX2),
            (TX_32X32, ITX2),
            (RTX_32X64, ITX1),
            (RTX_64X16, ITX1),
            (RTX_64X32, ITX1),
            (TX_64X64, ITX1),
        ] {
            slots.extend(types.iter().map(|&ty| (tx, ty)));
        }
        slots.sort();
        slots
    }

    /// With NEON allowed the kernels of `simd/itx/` are selected in the 156
    /// slots of the original's header, and with `dav1d_set_cpu_flags_mask(0)`
    /// none is (the scalar path).
    #[test]
    fn neon_selects_the_simd_kernels_and_mask_zero_keeps_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(header_slots().len(), 156);
        assert_eq!(changed8(!0), header_slots());
        assert!(changed8(0).is_empty());
    }

    /// The 16 bpc mirror has the translated 16 bpc context's layout.
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
        assert_eq!(
            core::mem::size_of::<Dav1dInvTxfmDSPContext16>(),
            19 * 17 * core::mem::size_of::<usize>()
        );
    }

    /// The slots of the translated 16 bpc init at `bpc` under `mask` that
    /// differ from the C kernels.
    fn changed16(mask: u32, bpc: c_int) -> Vec<(usize, usize)> {
        use crate::b16::itx_tmpl::{dav1d_itx_dsp_init_16bpc, Dav1dInvTxfmDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let table = |mask: u32| unsafe {
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_itx_dsp_init_16bpc(&mut c, bpc);
            c.itxfm_add.map(|row| row.map(|f| f.map(|f| f as usize)))
        };
        unsafe { dav1d_init_cpu() };
        let (simd, scalar) = (table(mask), table(0));
        unsafe { dav1d_set_cpu_flags_mask(!0) };
        (0..19)
            .flat_map(|tx| (0..17).map(move |ty| (tx, ty)))
            .filter(|&(tx, ty)| simd[tx][ty] != scalar[tx][ty])
            .collect()
    }

    /// At 10 bits NEON selects the kernels of `simd/itx/` in the slots of the
    /// header, at 12 bits only WHT, and `dav1d_set_cpu_flags_mask(0)` keeps C.
    #[test]
    fn neon_selects_the_simd_16bpc_kernels_and_mask_zero_keeps_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(changed16(!0, 10), header_slots());
        assert_eq!(changed16(!0, 12), vec![(TX_4X4, WHT_WHT)]);
        assert!(changed16(0, 10).is_empty());
        assert!(changed16(0, 12).is_empty());
    }
}
