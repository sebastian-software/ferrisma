// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `loop_filter_dsp_init_arm` of `upstream/dav1d/src/arm/loopfilter.h`:
//! under `DAV1D_ARM_CPU_FLAG_NEON` the four slots get the rebuilt kernels
//! (`simd/arm64/loopfilter.rs` at 8 bpc, `loopfilter16.rs` at 16 bpc, for
//! every high bit depth as in the original); with the flag masked off the
//! C kernels stay.
use core::ffi::{c_int, c_void};

use super::super::arm64::loopfilter::*;
use super::super::arm64::loopfilter16::*;
use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};

/// `loopfilter_sb_fn` (src/loopfilter.h) at 8 bpc.
pub type LoopFilterSbFn =
    unsafe extern "C" fn(*mut u8, isize, *const u32, *const [u8; 4], isize, *const Av1FilterLUT, c_int);

/// `Dav1dLoopFilterDSPContext` (src/loopfilter.h): `loop_filter_sb[2][2]`.
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext {
    pub loop_filter_sb: [[Option<LoopFilterSbFn>; 2]; 2],
}

/// Called at the end of the translated `dav1d_loop_filter_dsp_init_8bpc`,
/// where the original calls `loop_filter_dsp_init_arm(c)`.
///
/// # Safety
/// `c` points to the translated `Dav1dLoopFilterDSPContext` being
/// initialized (same layout, checked by a test), not otherwise accessed
/// during the call.
pub unsafe fn loop_filter_dsp_init_arm_8bpc(c: *mut c_void) {
    let c = &mut *(c as *mut Dav1dLoopFilterDSPContext);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    c.loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_8bpc_neon);
    c.loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_8bpc_neon);
    c.loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_8bpc_neon);
    c.loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_8bpc_neon);
}

/// `loopfilter_sb_fn` at 16 bpc (with `bitdepth_max`).
pub type LoopFilterSbFn16 = unsafe extern "C" fn(
    *mut u16,
    isize,
    *const u32,
    *const [u8; 4],
    isize,
    *const Av1FilterLUT,
    c_int,
    c_int,
);

/// `Dav1dLoopFilterDSPContext` at 16 bpc.
#[repr(C)]
pub struct Dav1dLoopFilterDSPContext16 {
    pub loop_filter_sb: [[Option<LoopFilterSbFn16>; 2]; 2],
}

/// Called at the end of the translated `dav1d_loop_filter_dsp_init_16bpc`.
///
/// # Safety
/// As for `loop_filter_dsp_init_arm_8bpc`.
pub unsafe fn loop_filter_dsp_init_arm_16bpc(c: *mut c_void) {
    let c = &mut *(c as *mut Dav1dLoopFilterDSPContext16);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    c.loop_filter_sb[0][0] = Some(dav1d_lpf_h_sb_y_16bpc_neon);
    c.loop_filter_sb[0][1] = Some(dav1d_lpf_v_sb_y_16bpc_neon);
    c.loop_filter_sb[1][0] = Some(dav1d_lpf_h_sb_uv_16bpc_neon);
    c.loop_filter_sb[1][1] = Some(dav1d_lpf_v_sb_uv_16bpc_neon);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_layout_matches_the_translation() {
        use crate::b8::loopfilter_tmpl::Dav1dLoopFilterDSPContext as Translated;
        assert_eq!(core::mem::size_of::<Translated>(), core::mem::size_of::<Dav1dLoopFilterDSPContext>());
        use crate::b16::loopfilter_tmpl::Dav1dLoopFilterDSPContext as Translated16;
        assert_eq!(core::mem::size_of::<Translated16>(), core::mem::size_of::<Dav1dLoopFilterDSPContext16>());
        assert_eq!(core::mem::size_of::<Dav1dLoopFilterDSPContext>(), 4 * core::mem::size_of::<usize>());
        use crate::common::lf_mask::Av1FilterLUT as Lut;
        assert_eq!(core::mem::size_of::<Lut>(), core::mem::size_of::<Av1FilterLUT>());
        assert_eq!(core::mem::offset_of!(Lut, sharp), 128);
    }

    /// With NEON allowed the rebuilt slots are selected, and with
    /// `dav1d_set_cpu_flags_mask(0)` none is.
    #[test]
    fn neon_selects_the_rebuilt_kernels_and_mask_zero_keeps_c() {
        use crate::b8::loopfilter_tmpl::{dav1d_loop_filter_dsp_init_8bpc, Dav1dLoopFilterDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let rebuilt = dav1d_lpf_v_sb_y_8bpc_neon as LoopFilterSbFn as usize;
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let slot = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_loop_filter_dsp_init_8bpc(&mut c);
            dav1d_set_cpu_flags_mask(!0);
            c.loop_filter_sb[0][1].map(|f| f as usize)
        };
        assert_eq!(slot(!0), Some(rebuilt));
        assert_ne!(slot(0), Some(rebuilt));
    }

    /// The same at 16 bpc.
    #[test]
    fn neon_selects_the_rebuilt_16bpc_kernels_and_mask_zero_keeps_c() {
        use crate::b16::loopfilter_tmpl::{
            dav1d_loop_filter_dsp_init_16bpc, Dav1dLoopFilterDSPContext as Translated,
        };
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let rebuilt = dav1d_lpf_h_sb_uv_16bpc_neon as LoopFilterSbFn16 as usize;
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let slot = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_loop_filter_dsp_init_16bpc(&mut c);
            dav1d_set_cpu_flags_mask(!0);
            c.loop_filter_sb[1][0].map(|f| f as usize)
        };
        assert_eq!(slot(!0), Some(rebuilt));
        assert_ne!(slot(0), Some(rebuilt));
    }
}
