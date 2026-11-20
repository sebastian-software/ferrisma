// Copyright © 2018-2023, VideoLAN and dav1d authors
// Copyright © 2018-2023, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `itx_dsp_init_x86` of `upstream/dav1d/src/x86/itx.h` for x86-64, statement
//! by statement: under `DAV1D_X86_CPU_FLAG_AVX2` every slot that the
//! original assigns an AVX2 kernel gets the inverse transform written from
//! the algorithm (`simd/itx/`, at 8 bpc and at 10 and 12 bits), if the CPU
//! has `fearless_simd`'s `Avx2` level (x86-64-v3; `simd/itx/mod.rs`);
//! otherwise, and with the flags masked off, the C kernels stay.
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

pub use super::super::itx::{ItxFn, ItxFn16};
use super::super::cpu::{
    dav1d_get_cpu_flags, DAV1D_X86_CPU_FLAG_AVX2, DAV1D_X86_CPU_FLAG_SSE2, DAV1D_X86_CPU_FLAG_SSE41,
    DAV1D_X86_CPU_FLAG_SSSE3,
};
use super::super::itx::*;

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

    let Some(k) = Kernels::new_8bpc() else {
        return;
    };
    let t = &mut c.itxfm_add;
    // assign_itx_fn(, 4, 4, wht_wht, WHT_WHT, avx2);
    k.assign_wht(t);

    // assign_itx16_fn( ,  4,  4, avx2);
    k.assign(t, TX_4X4, ITX16);
    // assign_itx16_fn(R,  4,  8, avx2);
    k.assign(t, RTX_4X8, ITX16);
    // assign_itx16_fn(R,  4, 16, avx2);
    k.assign(t, RTX_4X16, ITX16);
    // assign_itx16_fn(R,  8,  4, avx2);
    k.assign(t, RTX_8X4, ITX16);
    // assign_itx16_fn( ,  8,  8, avx2);
    k.assign(t, TX_8X8, ITX16);
    // assign_itx16_fn(R,  8, 16, avx2);
    k.assign(t, RTX_8X16, ITX16);
    // assign_itx2_fn(R,  8, 32, avx2);
    k.assign(t, RTX_8X32, ITX2);
    // assign_itx16_fn(R, 16,  4, avx2);
    k.assign(t, RTX_16X4, ITX16);
    // assign_itx16_fn(R, 16,  8, avx2);
    k.assign(t, RTX_16X8, ITX16);
    // assign_itx12_fn( , 16, 16, avx2);
    k.assign(t, TX_16X16, ITX12);
    // assign_itx2_fn(R, 16, 32, avx2);
    k.assign(t, RTX_16X32, ITX2);
    // assign_itx1_fn(R, 16, 64, avx2);
    k.assign(t, RTX_16X64, ITX1);
    // assign_itx2_fn(R, 32,  8, avx2);
    k.assign(t, RTX_32X8, ITX2);
    // assign_itx2_fn(R, 32, 16, avx2);
    k.assign(t, RTX_32X16, ITX2);
    // assign_itx2_fn( , 32, 32, avx2);
    k.assign(t, TX_32X32, ITX2);
    // assign_itx1_fn(R, 32, 64, avx2);
    k.assign(t, RTX_32X64, ITX1);
    // assign_itx1_fn(R, 64, 16, avx2);
    k.assign(t, RTX_64X16, ITX1);
    // assign_itx1_fn(R, 64, 32, avx2);
    k.assign(t, RTX_64X32, ITX1);
    // assign_itx1_fn( , 64, 64, avx2);
    k.assign(t, TX_64X64, ITX1);
    // The original's `*all_simd = 1` of the SSSE3 block (module
    // documentation): every slot now holds a kernel of `simd/itx/`.
    *all_simd = 1;

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

    let Some(k) = Kernels::new_16bpc() else {
        return;
    };
    let t = &mut c.itxfm_add;
    // assign_itx_fn(, 4, 4, wht_wht, WHT_WHT, avx2);
    k.assign_wht(t);

    if bpc == 10 {
        // assign_itx16_bpc_fn( ,  4,  4, 10, avx2);
        k.assign(t, TX_4X4, ITX16);
        // assign_itx16_bpc_fn(R,  4,  8, 10, avx2);
        k.assign(t, RTX_4X8, ITX16);
        // assign_itx16_bpc_fn(R,  4, 16, 10, avx2);
        k.assign(t, RTX_4X16, ITX16);
        // assign_itx16_bpc_fn(R,  8,  4, 10, avx2);
        k.assign(t, RTX_8X4, ITX16);
        // assign_itx16_bpc_fn( ,  8,  8, 10, avx2);
        k.assign(t, TX_8X8, ITX16);
        // assign_itx16_bpc_fn(R,  8, 16, 10, avx2);
        k.assign(t, RTX_8X16, ITX16);
        // assign_itx2_bpc_fn(R,  8, 32, 10, avx2);
        k.assign(t, RTX_8X32, ITX2);
        // assign_itx16_bpc_fn(R, 16,  4, 10, avx2);
        k.assign(t, RTX_16X4, ITX16);
        // assign_itx16_bpc_fn(R, 16,  8, 10, avx2);
        k.assign(t, RTX_16X8, ITX16);
        // assign_itx12_bpc_fn( , 16, 16, 10, avx2);
        k.assign(t, TX_16X16, ITX12);
        // assign_itx2_bpc_fn(R, 16, 32, 10, avx2);
        k.assign(t, RTX_16X32, ITX2);
        // assign_itx1_bpc_fn(R, 16, 64, 10, avx2);
        k.assign(t, RTX_16X64, ITX1);
        // assign_itx2_bpc_fn(R, 32,  8, 10, avx2);
        k.assign(t, RTX_32X8, ITX2);
        // assign_itx2_bpc_fn(R, 32, 16, 10, avx2);
        k.assign(t, RTX_32X16, ITX2);
        // assign_itx2_bpc_fn( , 32, 32, 10, avx2);
        k.assign(t, TX_32X32, ITX2);
        // assign_itx1_bpc_fn(R, 32, 64, 10, avx2);
        k.assign(t, RTX_32X64, ITX1);
        // assign_itx1_bpc_fn(R, 64, 16, 10, avx2);
        k.assign(t, RTX_64X16, ITX1);
        // assign_itx1_bpc_fn(R, 64, 32, 10, avx2);
        k.assign(t, RTX_64X32, ITX1);
        // assign_itx1_bpc_fn( , 64, 64, 10, avx2);
        k.assign(t, TX_64X64, ITX1);

        // The original's `*all_simd = 1` of the SSE4.1 block at 10 bits
        // (module documentation): every slot now holds a kernel of `simd/itx/`.
        *all_simd = 1;
    } else {
        // assign_itx16_bpc_fn( ,  4,  4, 12, avx2);
        k.assign(t, TX_4X4, ITX16);
        // assign_itx16_bpc_fn(R,  4,  8, 12, avx2);
        k.assign(t, RTX_4X8, ITX16);
        // assign_itx16_bpc_fn(R,  4, 16, 12, avx2);
        k.assign(t, RTX_4X16, ITX16);
        // assign_itx16_bpc_fn(R,  8,  4, 12, avx2);
        k.assign(t, RTX_8X4, ITX16);
        // assign_itx16_bpc_fn( ,  8,  8, 12, avx2);
        k.assign(t, TX_8X8, ITX16);
        // assign_itx16_bpc_fn(R,  8, 16, 12, avx2);
        k.assign(t, RTX_8X16, ITX16);
        // assign_itx2_bpc_fn(R,  8, 32, 12, avx2);
        k.assign(t, RTX_8X32, ITX2);
        // assign_itx16_bpc_fn(R, 16,  4, 12, avx2);
        k.assign(t, RTX_16X4, ITX16);
        // assign_itx16_bpc_fn(R, 16,  8, 12, avx2);
        k.assign(t, RTX_16X8, ITX16);
        // assign_itx12_bpc_fn( , 16, 16, 12, avx2);
        k.assign(t, TX_16X16, ITX12);
        // assign_itx2_bpc_fn(R, 32,  8, 12, avx2);
        k.assign(t, RTX_32X8, ITX2);
        // assign_itx_bpc_fn(R, 16, 32, identity_identity, IDTX, 12, avx2);
        k.assign(t, RTX_16X32, IDENTITY);
        // assign_itx_bpc_fn(R, 32, 16, identity_identity, IDTX, 12, avx2);
        k.assign(t, RTX_32X16, IDENTITY);
        // assign_itx_bpc_fn(, 32, 32, identity_identity, IDTX, 12, avx2);
        k.assign(t, TX_32X32, IDENTITY);
    }

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

    /// Whether the CPU has AVX2 and `fearless_simd`'s `Avx2` level.
    fn level() -> bool {
        std::arch::is_x86_feature_detected!("avx2") && fearless_simd::Level::new().as_avx2().is_some()
    }

    /// The slots in which two tables differ.
    fn changed(a: &[[Option<usize>; 17]; 19], b: &[[Option<usize>; 17]; 19]) -> Vec<(usize, usize)> {
        (0..19)
            .flat_map(|tx| (0..17).map(move |ty| (tx, ty)))
            .filter(|&(tx, ty)| a[tx][ty] != b[tx][ty])
            .collect()
    }

    /// The slots of the original's AVX2 block (src/x86/itx.h) at 8 bpc and
    /// at 10 bits (`assign_itx*_fn` / `assign_itx*_bpc_fn`, and WHT); at 12
    /// bits the same without the DCT_DCT slots of 16x32 and larger.
    fn header_slots(bpc: c_int) -> Vec<(usize, usize)> {
        let large = if bpc == 12 { IDENTITY } else { ITX2 };
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
            (RTX_16X32, large),
            (RTX_32X8, ITX2),
            (RTX_32X16, large),
            (TX_32X32, large),
        ] {
            slots.extend(types.iter().map(|&ty| (tx, ty)));
        }
        if bpc != 12 {
            for tx in [RTX_16X64, RTX_32X64, RTX_64X16, RTX_64X32, TX_64X64] {
                slots.push((tx, DCT_DCT));
            }
        }
        slots.sort();
        slots
    }

    /// With AVX2 allowed (and the level detected) the kernels of `simd/itx/`
    /// are selected in all 156 slots that the original assigns; a mask below
    /// AVX2 keeps the C kernels (also where the original has SSE2 or SSSE3
    /// kernels), the same ones as `dav1d_set_cpu_flags_mask(0)`.
    #[test]
    fn avx2_selects_the_simd_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let scalar = table8(0);
        let expected = if level() { header_slots(8) } else { vec![] };
        assert_eq!(header_slots(8).len(), 156);
        assert_eq!(changed(&table8(!0), &scalar), expected);
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | DAV1D_X86_CPU_FLAG_SSE41;
        assert_eq!(table8(below), scalar);
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

    /// With AVX2 allowed (and the level detected) the kernels of
    /// `simd/itx/` are selected in the 156 slots that the original assigns
    /// at 10 bits and the 148 at 12 bits (the eight DCT_DCT slots of 16x32
    /// and larger keep C); a mask below AVX2 keeps the C kernels (also where
    /// the original has SSE2 or SSE4.1 kernels), the same ones as
    /// `dav1d_set_cpu_flags_mask(0)`.
    #[test]
    fn avx2_selects_the_simd_16bpc_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | DAV1D_X86_CPU_FLAG_SSE41;
        for (bpc, count) in [(10, 156), (12, 148)] {
            let scalar = table16(0, bpc);
            let expected = if level() { header_slots(bpc) } else { vec![] };
            assert_eq!(header_slots(bpc).len(), count);
            assert_eq!(changed(&table16(!0, bpc), &scalar), expected);
            assert_eq!(table16(below, bpc), scalar);
        }
    }
}
