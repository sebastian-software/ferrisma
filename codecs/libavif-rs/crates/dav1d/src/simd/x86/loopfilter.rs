// Copyright © 2018-2021, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `loop_filter_dsp_init_x86` of `upstream/dav1d/src/x86/loopfilter.h` for
//! x86-64, statement by statement: under `DAV1D_X86_CPU_FLAG_AVX2` (which
//! the original tests after `DAV1D_X86_CPU_FLAG_SSSE3`) the four slots get
//! the rebuilt AVX2 kernels (`simd/x86_64/loopfilter_avx2.rs` at 8 bpc,
//! `loopfilter16_avx2.rs` at 16 bpc, for every high bit depth as in the
//! original); with the flags masked off the C kernels stay.
//!
//! Two of the original's assignments have no counterpart (ADR 0062,
//! "Levels"): the SSSE3 kernels, which the original selects only on a CPU
//! without AVX2 (stated gap: the C kernels stay there), and the AVX-512
//! kernels, which are above the v1.0 scope (engine ADR-0041 section 4) and
//! which the gate masks off in the original.
use core::ffi::{c_int, c_void};

use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_X86_CPU_FLAG_AVX2, DAV1D_X86_CPU_FLAG_SSSE3};
use super::super::x86_64::loopfilter16_avx2::*;
pub use super::super::x86_64::loopfilter_avx2::Av1FilterLUT;
use super::super::x86_64::loopfilter_avx2::*;

/// `loopfilter_sb_fn` (src/loopfilter.h) at 8 bpc.
pub type LoopFilterSbFn =
    unsafe extern "C" fn(*mut u8, isize, *const u32, *const [u8; 4], isize, *const Av1FilterLUT, c_int);

/// `Dav1dLoopFilterDSPContext` (src/loopfilter.h): `loop_filter_sb[2][2]`.
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[Option<LoopFilterSbFn>; 2]; 2],
}

/// Called at the end of the translated `dav1d_loop_filter_dsp_init_8bpc`,
/// where the original calls `loop_filter_dsp_init_x86(c)`.
///
/// # Safety
/// `c` points to the translated `Dav1dLoopFilterDSPContext` being
/// initialized (same layout, checked by a test), not otherwise accessed
/// during the call.
pub unsafe fn loop_filter_dsp_init_x86_8bpc(c: *mut c_void) {
    let c = &mut *(c as *mut Dav1dLoopFilterDSPContext);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    // c->loop_filter_sb[..][..] = BF(dav1d_lpf_*, ssse3): not rebuilt, the
    // C kernels stay (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    c.loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_avx2);
    c.loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_avx2);
    c.loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_avx2);
    c.loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_avx2);

    // if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return; and the
    // avx512icl assignments: above the scope (module documentation).
}

/// `loopfilter_sb_fn` at 16 bpc (with `bitdepth_max`).
pub type LoopFilterSbFn16 =
    unsafe extern "C" fn(*mut u16, isize, *const u32, *const [u8; 4], isize, *const Av1FilterLUT, c_int, c_int);

/// `Dav1dLoopFilterDSPContext` at 16 bpc.
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext16 {
    pub loop_filter_sb: [[Option<LoopFilterSbFn16>; 2]; 2],
}

/// Called at the end of the translated `dav1d_loop_filter_dsp_init_16bpc`.
///
/// # Safety
/// As for `loop_filter_dsp_init_x86_8bpc`.
pub unsafe fn loop_filter_dsp_init_x86_16bpc(c: *mut c_void) {
    let c = &mut *(c as *mut Dav1dLoopFilterDSPContext16);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    // c->loop_filter_sb[..][..] = BF(dav1d_lpf_*, ssse3): not rebuilt, the
    // C kernels stay (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    c.loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_16bpc_avx2);
    c.loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_16bpc_avx2);
    c.loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_16bpc_avx2);
    c.loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_16bpc_avx2);

    // if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return; and the
    // avx512icl assignments: above the scope (module documentation).
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simd::cpu::DAV1D_X86_CPU_FLAG_SSE2;

    #[test]
    fn context_layout_matches_the_translation() {
        use crate::b8::loopfilter_tmpl::Dav1dLoopFilterDSPContext as Translated;
        assert_eq!(
            core::mem::size_of::<Translated>(),
            core::mem::size_of::<Dav1dLoopFilterDSPContext>()
        );
        use crate::b16::loopfilter_tmpl::Dav1dLoopFilterDSPContext as Translated16;
        assert_eq!(
            core::mem::size_of::<Translated16>(),
            core::mem::size_of::<Dav1dLoopFilterDSPContext16>()
        );
        assert_eq!(
            core::mem::size_of::<Dav1dLoopFilterDSPContext>(),
            4 * core::mem::size_of::<usize>()
        );
        use crate::common::lf_mask::Av1FilterLUT as Lut;
        assert_eq!(core::mem::size_of::<Lut>(), core::mem::size_of::<Av1FilterLUT>());
        assert_eq!(core::mem::offset_of!(Lut, sharp), 128);
    }

    /// The slots after the translated init under a CPU mask: with AVX2
    /// allowed (and detected) the rebuilt kernel, otherwise not: below AVX2
    /// the C kernel stays, also where the original has an SSSE3 kernel.
    fn slot8(mask: u32) -> Option<usize> {
        use crate::b8::loopfilter_tmpl::{dav1d_loop_filter_dsp_init_8bpc, Dav1dLoopFilterDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_loop_filter_dsp_init_8bpc(&mut c);
            dav1d_set_cpu_flags_mask(!0);
            c.loop_filter_sb[0][1].map(|f| f as usize)
        }
    }

    fn slot16(mask: u32) -> Option<usize> {
        use crate::b16::loopfilter_tmpl::{dav1d_loop_filter_dsp_init_16bpc, Dav1dLoopFilterDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_loop_filter_dsp_init_16bpc(&mut c);
            dav1d_set_cpu_flags_mask(!0);
            c.loop_filter_sb[1][0].map(|f| f as usize)
        }
    }

    #[test]
    fn avx2_selects_the_rebuilt_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        let rebuilt = dav1d_lpf_v_sb_y_8bpc_avx2 as LoopFilterSbFn as usize;
        assert_eq!(slot8(!0) == Some(rebuilt), avx2);
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | crate::simd::cpu::DAV1D_X86_CPU_FLAG_SSE41;
        assert_ne!(slot8(below), Some(rebuilt));
        assert_ne!(slot8(0), Some(rebuilt));
        assert_eq!(slot8(below), slot8(0));
    }

    #[test]
    fn avx2_selects_the_rebuilt_16bpc_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        let rebuilt = dav1d_lpf_h_sb_uv_16bpc_avx2 as LoopFilterSbFn16 as usize;
        assert_eq!(slot16(!0) == Some(rebuilt), avx2);
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | crate::simd::cpu::DAV1D_X86_CPU_FLAG_SSE41;
        assert_ne!(slot16(below), Some(rebuilt));
        assert_eq!(slot16(below), slot16(0));
    }
}
