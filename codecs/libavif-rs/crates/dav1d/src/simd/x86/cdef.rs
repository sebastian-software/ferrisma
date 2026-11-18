// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `cdef_dsp_init_x86` of `upstream/dav1d/src/x86/cdef.h` for x86-64,
//! statement by statement: under `DAV1D_X86_CPU_FLAG_AVX2` the four slots
//! (`dir`, `fb[0..2]`) get the rebuilt AVX2 kernels
//! (`simd/x86_64/cdef_avx2.rs` at 8 bpc, `simd/x86_64/cdef16_avx2.rs` at 16
//! bpc, for every high bit depth as in the original); with the flags masked
//! off the C kernels stay.
//!
//! On x86 the original assigns the assembly to the slots directly: each
//! kernel pads the block itself (no `DEFINE_FILTER` wrapper as on ARM), so
//! the slots hold the rebuilt entry points.
//!
//! The original's SSE2 (8 bpc), SSSE3 and SSE4.1 assignments, which it
//! selects only on a CPU without AVX2, and its AVX-512 assignments have no
//! counterpart (ADR 0062, "Levels"; engine ADR-0041 section 4): below AVX2
//! the C kernels stay.
use core::ffi::{c_int, c_uint, c_void};

use super::super::cpu::{
    dav1d_get_cpu_flags, DAV1D_X86_CPU_FLAG_AVX2, DAV1D_X86_CPU_FLAG_SSE2, DAV1D_X86_CPU_FLAG_SSE41,
    DAV1D_X86_CPU_FLAG_SSSE3,
};
use super::super::x86_64::cdef16_avx2::*;
use super::super::x86_64::cdef_avx2::*;

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

/// Called at the end of the translated `dav1d_cdef_dsp_init_8bpc`, where
/// the original calls `cdef_dsp_init_x86(c)`.
///
/// # Safety
/// `c` points to the translated `Dav1dCdefDSPContext` being initialized
/// (same layout, checked by a test), not otherwise accessed during the call.
pub unsafe fn cdef_dsp_init_x86_8bpc(c: *mut c_void) {
    let c = &mut *(c as *mut Dav1dCdefDSPContext);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSE2 == 0 {
        return;
    }

    // c->fb[0..2] = BF(dav1d_cdef_filter_*, sse2): not rebuilt, the C
    // kernels stay (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    // c->dir and c->fb[0..2] = the ssse3 kernels: not rebuilt.

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    // c->dir and c->fb[0..2] = the sse4 kernels: not rebuilt.

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    c.dir = Some(dav1d_cdef_dir_8bpc_avx2);
    c.fb[0] = Some(dav1d_cdef_filter_8x8_8bpc_avx2);
    c.fb[1] = Some(dav1d_cdef_filter_4x8_8bpc_avx2);
    c.fb[2] = Some(dav1d_cdef_filter_4x4_8bpc_avx2);

    // if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return; and the
    // avx512icl assignments: above the scope (module documentation).
}

/// Called at the end of the translated `dav1d_cdef_dsp_init_16bpc`.
///
/// # Safety
/// As for `cdef_dsp_init_x86_8bpc`.
pub unsafe fn cdef_dsp_init_x86_16bpc(c: *mut c_void) {
    let c = &mut *(c as *mut Dav1dCdefDSPContext16);
    let flags = dav1d_get_cpu_flags();

    if flags & DAV1D_X86_CPU_FLAG_SSSE3 == 0 {
        return;
    }

    // c->dir and c->fb[0..2] = the ssse3 kernels: not rebuilt, the C
    // kernels stay (module documentation).

    if flags & DAV1D_X86_CPU_FLAG_SSE41 == 0 {
        return;
    }

    // c->dir = the sse4 kernel: not rebuilt.

    if flags & DAV1D_X86_CPU_FLAG_AVX2 == 0 {
        return;
    }

    c.dir = Some(dav1d_cdef_dir_16bpc_avx2);
    c.fb[0] = Some(dav1d_cdef_filter_8x8_16bpc_avx2);
    c.fb[1] = Some(dav1d_cdef_filter_4x8_16bpc_avx2);
    c.fb[2] = Some(dav1d_cdef_filter_4x4_16bpc_avx2);

    // if (!(flags & DAV1D_X86_CPU_FLAG_AVX512ICL)) return; and the
    // avx512icl assignments: above the scope (module documentation).
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_layout_matches_the_translation() {
        use crate::b8::cdef_tmpl::Dav1dCdefDSPContext as Translated;
        assert_eq!(
            core::mem::size_of::<Translated>(),
            core::mem::size_of::<Dav1dCdefDSPContext>()
        );
        use crate::b16::cdef_tmpl::Dav1dCdefDSPContext as Translated16;
        assert_eq!(
            core::mem::size_of::<Translated16>(),
            core::mem::size_of::<Dav1dCdefDSPContext16>()
        );
        assert_eq!(
            core::mem::size_of::<Dav1dCdefDSPContext>(),
            4 * core::mem::size_of::<usize>()
        );
        assert_eq!(
            core::mem::offset_of!(Translated, fb),
            core::mem::offset_of!(Dav1dCdefDSPContext, fb)
        );
    }

    /// The four slots after the translated init under a CPU mask.
    fn slots8(mask: u32) -> [Option<usize>; 4] {
        use crate::b8::cdef_tmpl::{dav1d_cdef_dsp_init_8bpc, Dav1dCdefDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_cdef_dsp_init_8bpc(&mut c);
            dav1d_set_cpu_flags_mask(!0);
            [
                c.dir.map(|f| f as usize),
                c.fb[0].map(|f| f as usize),
                c.fb[1].map(|f| f as usize),
                c.fb[2].map(|f| f as usize),
            ]
        }
    }

    fn slots16(mask: u32) -> [Option<usize>; 4] {
        use crate::b16::cdef_tmpl::{dav1d_cdef_dsp_init_16bpc, Dav1dCdefDSPContext as Translated};
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut c: Translated = core::mem::zeroed();
            dav1d_cdef_dsp_init_16bpc(&mut c);
            dav1d_set_cpu_flags_mask(!0);
            [
                c.dir.map(|f| f as usize),
                c.fb[0].map(|f| f as usize),
                c.fb[1].map(|f| f as usize),
                c.fb[2].map(|f| f as usize),
            ]
        }
    }

    /// With AVX2 allowed (and detected) every slot holds the rebuilt kernel;
    /// below AVX2 every slot keeps the C kernel, also where the original has
    /// an SSE2, SSSE3 or SSE4.1 kernel.
    #[test]
    fn avx2_selects_the_rebuilt_kernels_and_lower_masks_keep_c() {
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        let rebuilt8 = [
            Some(dav1d_cdef_dir_8bpc_avx2 as CdefDirFn as usize),
            Some(dav1d_cdef_filter_8x8_8bpc_avx2 as CdefFn as usize),
            Some(dav1d_cdef_filter_4x8_8bpc_avx2 as CdefFn as usize),
            Some(dav1d_cdef_filter_4x4_8bpc_avx2 as CdefFn as usize),
        ];
        let rebuilt16 = [
            Some(dav1d_cdef_dir_16bpc_avx2 as CdefDirFn16 as usize),
            Some(dav1d_cdef_filter_8x8_16bpc_avx2 as CdefFn16 as usize),
            Some(dav1d_cdef_filter_4x8_16bpc_avx2 as CdefFn16 as usize),
            Some(dav1d_cdef_filter_4x4_16bpc_avx2 as CdefFn16 as usize),
        ];
        assert_eq!(slots8(!0) == rebuilt8, avx2);
        assert_eq!(slots16(!0) == rebuilt16, avx2);
        let below = DAV1D_X86_CPU_FLAG_SSE2 | DAV1D_X86_CPU_FLAG_SSSE3 | DAV1D_X86_CPU_FLAG_SSE41;
        for (below8, c8) in slots8(below).iter().zip(slots8(0)) {
            assert_eq!(*below8, c8);
            assert!(!rebuilt8.contains(&c8));
        }
        for (below16, c16) in slots16(below).iter().zip(slots16(0)) {
            assert_eq!(*below16, c16);
            assert!(!rebuilt16.contains(&c16));
        }
    }
}
