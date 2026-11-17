// Copyright © 2018, VideoLAN and dav1d authors
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `upstream/dav1d/src/arm/cdef.h`: the `DEFINE_FILTER` wrappers
//! `cdef_filter_{8x8,4x8,4x4}_neon` (a 16-byte aligned `uint16_t` buffer of
//! `12 * tmp_stride + 8` elements, `dav1d_cdef_padding*` then
//! `dav1d_cdef_filter*`) and `cdef_dsp_init_arm`: under
//! `DAV1D_ARM_CPU_FLAG_NEON` the four slots get the rebuilt kernels
//! (`simd/arm64/cdef.rs` at 8 bpc, `cdef16.rs` at 16 bpc, for every high
//! bit depth as in the original); with the flag masked off the C kernels
//! stay. The wrappers' buffer is left uninitialized, as `ALIGN_STK_16`
//! leaves it; the padding writes every element the filter reads
//! (`simd/arm64/cdef.rs`, safety).
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

/// `ALIGN_STK_16`
#[repr(C, align(16))]
struct Align16<T>(T);

/// `DEFINE_FILTER(w, h, tmp_stride)`
macro_rules! define_filter {
    ($name:ident, $w:literal, $h:literal, $tmp_stride:literal, $padding:path, $filter:path, u8) => {
        unsafe extern "C" fn $name(
            dst: *mut u8,
            stride: isize,
            left: *const [u8; 2],
            top: *const u8,
            bottom: *const u8,
            pri_strength: c_int,
            sec_strength: c_int,
            dir: c_int,
            damping: c_int,
            edges: c_uint,
        ) {
            let mut tmp_buf = core::mem::MaybeUninit::<Align16<[u16; 12 * $tmp_stride + 8]>>::uninit();
            let tmp = (tmp_buf.as_mut_ptr() as *mut u16).add(2 * $tmp_stride + 8);
            $padding(tmp, dst, stride, left, top, bottom, $h, edges);
            $filter(dst, stride, tmp, pri_strength, sec_strength, dir, damping, $h, edges as usize);
        }
    };
    ($name:ident, $w:literal, $h:literal, $tmp_stride:literal, $padding:path, $filter:path, u16) => {
        unsafe extern "C" fn $name(
            dst: *mut u16,
            stride: isize,
            left: *const [u16; 2],
            top: *const u16,
            bottom: *const u16,
            pri_strength: c_int,
            sec_strength: c_int,
            dir: c_int,
            damping: c_int,
            edges: c_uint,
            bitdepth_max: c_int,
        ) {
            let mut tmp_buf = core::mem::MaybeUninit::<Align16<[u16; 12 * $tmp_stride + 8]>>::uninit();
            let tmp = (tmp_buf.as_mut_ptr() as *mut u16).add(2 * $tmp_stride + 8);
            $padding(tmp, dst, stride, left, top, bottom, $h, edges);
            $filter(dst, stride, tmp, pri_strength, sec_strength, dir, damping, $h, edges as usize, bitdepth_max);
        }
    };
}

/// The 8 bpc compilation of `src/arm/cdef.h`.
mod bpc8 {
    use super::super::super::arm64::cdef::*;
    use super::*;

    define_filter!(cdef_filter_8x8_neon, 8, 8, 16, dav1d_cdef_padding8_8bpc_neon, dav1d_cdef_filter8_8bpc_neon, u8);
    define_filter!(cdef_filter_4x8_neon, 4, 8, 8, dav1d_cdef_padding4_8bpc_neon, dav1d_cdef_filter4_8bpc_neon, u8);
    define_filter!(cdef_filter_4x4_neon, 4, 4, 8, dav1d_cdef_padding4_8bpc_neon, dav1d_cdef_filter4_8bpc_neon, u8);

    /// `cdef_dsp_init_arm` at 8 bpc.
    pub(super) unsafe fn cdef_dsp_init_arm(c: &mut Dav1dCdefDSPContext) {
        let flags = dav1d_get_cpu_flags();

        if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
            return;
        }

        c.dir = Some(dav1d_cdef_find_dir_8bpc_neon);
        c.fb[0] = Some(cdef_filter_8x8_neon);
        c.fb[1] = Some(cdef_filter_4x8_neon);
        c.fb[2] = Some(cdef_filter_4x4_neon);
    }
}

/// The 16 bpc compilation of `src/arm/cdef.h`.
mod bpc16 {
    use super::super::super::arm64::cdef16::*;
    use super::*;

    define_filter!(cdef_filter_8x8_neon, 8, 8, 16, dav1d_cdef_padding8_16bpc_neon, dav1d_cdef_filter8_16bpc_neon, u16);
    define_filter!(cdef_filter_4x8_neon, 4, 8, 8, dav1d_cdef_padding4_16bpc_neon, dav1d_cdef_filter4_16bpc_neon, u16);
    define_filter!(cdef_filter_4x4_neon, 4, 4, 8, dav1d_cdef_padding4_16bpc_neon, dav1d_cdef_filter4_16bpc_neon, u16);

    /// `cdef_dsp_init_arm` at 16 bpc.
    pub(super) unsafe fn cdef_dsp_init_arm(c: &mut Dav1dCdefDSPContext16) {
        let flags = dav1d_get_cpu_flags();

        if flags & DAV1D_ARM_CPU_FLAG_NEON == 0 {
            return;
        }

        c.dir = Some(dav1d_cdef_find_dir_16bpc_neon);
        c.fb[0] = Some(cdef_filter_8x8_neon);
        c.fb[1] = Some(cdef_filter_4x8_neon);
        c.fb[2] = Some(cdef_filter_4x4_neon);
    }
}

/// Called at the end of the translated `dav1d_cdef_dsp_init_8bpc`, where
/// the original calls `cdef_dsp_init_arm(c)`.
///
/// # Safety
/// `c` points to the translated `Dav1dCdefDSPContext` being initialized
/// (same layout, checked by a test), not otherwise accessed during the
/// call.
pub unsafe fn cdef_dsp_init_arm_8bpc(c: *mut c_void) {
    bpc8::cdef_dsp_init_arm(&mut *(c as *mut Dav1dCdefDSPContext))
}

/// Called at the end of the translated `dav1d_cdef_dsp_init_16bpc`.
///
/// # Safety
/// As for `cdef_dsp_init_arm_8bpc`.
pub unsafe fn cdef_dsp_init_arm_16bpc(c: *mut c_void) {
    bpc16::cdef_dsp_init_arm(&mut *(c as *mut Dav1dCdefDSPContext16))
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

    /// With NEON allowed the rebuilt slots are selected, and with
    /// `dav1d_set_cpu_flags_mask(0)` none is.
    #[test]
    fn neon_selects_the_rebuilt_kernels_and_mask_zero_keeps_c() {
        use crate::b8::cdef_tmpl::{dav1d_cdef_dsp_init_8bpc, Dav1dCdefDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let rebuilt = crate::simd::arm64::cdef::dav1d_cdef_find_dir_8bpc_neon as CdefDirFn as usize;
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let slots = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_cdef_dsp_init_8bpc(&mut c);
            dav1d_set_cpu_flags_mask(0);
            dav1d_cdef_dsp_init_8bpc(&mut reference);
            dav1d_set_cpu_flags_mask(!0);
            let fb = |c: &Translated| c.fb.map(|f| f.map(|f| f as usize));
            (c.dir.map(|f| f as usize), fb(&c) != fb(&reference))
        };
        assert_eq!(slots(!0), (Some(rebuilt), true));
        let (dir, changed) = slots(0);
        assert_ne!(dir, Some(rebuilt));
        assert!(!changed);
    }

    /// The same at 16 bpc.
    #[test]
    fn neon_selects_the_rebuilt_16bpc_kernels_and_mask_zero_keeps_c() {
        use crate::b16::cdef_tmpl::{dav1d_cdef_dsp_init_16bpc, Dav1dCdefDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        let rebuilt = crate::simd::arm64::cdef16::dav1d_cdef_find_dir_16bpc_neon as CdefDirFn16 as usize;
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let slots = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            let mut reference: Translated = core::mem::zeroed();
            dav1d_cdef_dsp_init_16bpc(&mut c);
            dav1d_set_cpu_flags_mask(0);
            dav1d_cdef_dsp_init_16bpc(&mut reference);
            dav1d_set_cpu_flags_mask(!0);
            let fb = |c: &Translated| c.fb.map(|f| f.map(|f| f as usize));
            (c.dir.map(|f| f as usize), fb(&c) != fb(&reference))
        };
        assert_eq!(slots(!0), (Some(rebuilt), true));
        let (dir, changed) = slots(0);
        assert_ne!(dir, Some(rebuilt));
        assert!(!changed);
    }
}
