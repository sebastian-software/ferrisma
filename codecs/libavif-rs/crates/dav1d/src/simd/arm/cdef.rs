// Copyright © 2018, VideoLAN and dav1d authors
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `cdef_dsp_init_arm` of `upstream/dav1d/src/arm/cdef.h`: under
//! `DAV1D_ARM_CPU_FLAG_NEON` the four slots (`dir`, `fb[0..2]`) get the
//! kernels written from the algorithm (`simd/cdef/`, at 8 bpc and at 16 bpc
//! for every high bit depth, as the original assigns its NEON kernels); with
//! the flag masked off the C kernels stay.
//!
//! The original's `DEFINE_FILTER` wrappers `cdef_filter_{8x8,4x8,4x4}_neon`
//! pad the block into a stack buffer and call the assembly filter; the
//! rewrite's slots do the same in one function (`simd/cdef/mod.rs`: the C
//! `padding`, then the filter), so no wrapper is left here.
use core::ffi::{c_int, c_uint, c_void};

use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_ARM_CPU_FLAG_NEON};

/// `cdef_dir_fn` (src/cdef.h) at 8 bpc.
pub type CdefDirFn = unsafe extern "C" fn(*const u8, isize, *mut c_uint) -> c_int;
/// `cdef_fn` (src/cdef.h) at 8 bpc.
pub type CdefFn =
    unsafe extern "C" fn(*mut u8, isize, *const [u8; 2], *const u8, *const u8, c_int, c_int, c_int, c_int, c_uint);

/// `Dav1dCdefDSPContext` (src/cdef.h): `dir`, `fb[3]`.
#[repr(C)]
pub struct Dav1dCdefDSPContext {
    pub dir: Option<CdefDirFn>,
    pub fb: [Option<CdefFn>; 3],
}

/// `cdef_dir_fn` at 16 bpc (with `bitdepth_max`).
pub type CdefDirFn16 = unsafe extern "C" fn(*const u16, isize, *mut c_uint, c_int) -> c_int;
/// `cdef_fn` at 16 bpc (with `bitdepth_max`).
pub type CdefFn16 = unsafe extern "C" fn(
    *mut u16,
    isize,
    *const [u16; 2],
    *const u16,
    *const u16,
    c_int,
    c_int,
    c_int,
    c_int,
    c_uint,
    c_int,
);

/// `Dav1dCdefDSPContext` at 16 bpc.
#[repr(C)]
pub struct Dav1dCdefDSPContext16 {
    pub dir: Option<CdefDirFn16>,
    pub fb: [Option<CdefFn16>; 3],
}

/// `cdef_dsp_init_arm` at 8 bpc.
fn cdef_dsp_init_arm(c: &mut Dav1dCdefDSPContext) {
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    // c->dir = dav1d_cdef_find_dir_8bpc_neon; c->fb[0..2] =
    // cdef_filter_{8x8,4x8,4x4}_8bpc_neon.
    crate::simd::cdef::install_8bpc(c);
}

/// `cdef_dsp_init_arm` at 16 bpc.
fn cdef_dsp_init_arm16(c: &mut Dav1dCdefDSPContext16) {
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
        return;
    }

    // The same slots, `_16bpc_neon`.
    crate::simd::cdef::install_16bpc(c);
}

/// Called at the end of the translated `dav1d_cdef_dsp_init_8bpc`, where
/// the original calls `cdef_dsp_init_arm(c)`.
///
/// # Safety
/// `c` points to the translated `Dav1dCdefDSPContext` being initialized
/// (same layout, checked by a test), not otherwise accessed during the
/// call.
pub unsafe fn cdef_dsp_init_arm_8bpc(c: *mut c_void) {
    cdef_dsp_init_arm(&mut *(c as *mut Dav1dCdefDSPContext))
}

/// Called at the end of the translated `dav1d_cdef_dsp_init_16bpc`.
///
/// # Safety
/// As for `cdef_dsp_init_arm_8bpc`.
pub unsafe fn cdef_dsp_init_arm_16bpc(c: *mut c_void) {
    cdef_dsp_init_arm16(&mut *(c as *mut Dav1dCdefDSPContext16))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_layout_matches_the_translation() {
        use core::mem::{offset_of, size_of};
        use crate::b8::cdef_tmpl::Dav1dCdefDSPContext as Translated;
        assert_eq!(size_of::<Translated>(), size_of::<Dav1dCdefDSPContext>());
        assert_eq!(offset_of!(Translated, fb), offset_of!(Dav1dCdefDSPContext, fb));
        use crate::b16::cdef_tmpl::Dav1dCdefDSPContext as Translated16;
        assert_eq!(size_of::<Translated16>(), size_of::<Dav1dCdefDSPContext16>());
        assert_eq!(offset_of!(Translated16, fb), offset_of!(Dav1dCdefDSPContext16, fb));
        assert_eq!(size_of::<Dav1dCdefDSPContext>(), 4 * size_of::<usize>());
    }

    /// The four slots after the translated init under `mask`, and whether
    /// they differ from the C kernels (`dav1d_set_cpu_flags_mask(0)`).
    fn changed8(mask: u32) -> [bool; 4] {
        use crate::b8::cdef_tmpl::{dav1d_cdef_dsp_init_8bpc, Dav1dCdefDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let slots = |c: &Translated| {
            let fb = c.fb.map(|f| f.map(|f| f as usize));
            [c.dir.map(|f| f as usize), fb[0], fb[1], fb[2]]
        };
        unsafe {
            dav1d_init_cpu();
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_set_cpu_flags_mask(mask);
            dav1d_cdef_dsp_init_8bpc(&mut c);
            dav1d_set_cpu_flags_mask(0);
            dav1d_cdef_dsp_init_8bpc(&mut reference);
            dav1d_set_cpu_flags_mask(!0);
            core::array::from_fn(|i| slots(&c)[i] != slots(&reference)[i])
        }
    }

    fn changed16(mask: u32) -> [bool; 4] {
        use crate::b16::cdef_tmpl::{dav1d_cdef_dsp_init_16bpc, Dav1dCdefDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let slots = |c: &Translated| {
            let fb = c.fb.map(|f| f.map(|f| f as usize));
            [c.dir.map(|f| f as usize), fb[0], fb[1], fb[2]]
        };
        unsafe {
            dav1d_init_cpu();
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_set_cpu_flags_mask(mask);
            dav1d_cdef_dsp_init_16bpc(&mut c);
            dav1d_set_cpu_flags_mask(0);
            dav1d_cdef_dsp_init_16bpc(&mut reference);
            dav1d_set_cpu_flags_mask(!0);
            core::array::from_fn(|i| slots(&c)[i] != slots(&reference)[i])
        }
    }

    /// With NEON allowed all four slots hold the kernels of `simd/cdef/` at
    /// 8 and 16 bpc, and with `dav1d_set_cpu_flags_mask(0)` none does.
    #[test]
    fn neon_selects_the_simd_kernels_and_mask_zero_keeps_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(changed8(!0), [true; 4]);
        assert_eq!(changed16(!0), [true; 4]);
        assert_eq!(changed8(0), [false; 4]);
        assert_eq!(changed16(0), [false; 4]);
    }
}
