// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! CDEF written from the algorithm (`libavif-dav1d-kernels`, ADR 0062,
//! "Kernels written from the algorithm"): the installation into the DSP
//! table and the tests against the translated C. The init mirrors
//! (`simd/arm/cdef.rs`, `simd/x86/cdef.rs`) call `install_8bpc` and
//! `install_16bpc` where the original assigns its NEON or AVX2 kernels; on
//! a CPU without the kernels' level (x86-64-v3 on x86-64) the C kernels
//! stay.

pub use dav1d_kernels::cdef::{kernels_16bpc, kernels_8bpc};

#[cfg(target_arch = "aarch64")]
use super::arm::cdef::{Dav1dCdefDSPContext, Dav1dCdefDSPContext16};
#[cfg(target_arch = "x86_64")]
use super::x86::cdef::{Dav1dCdefDSPContext, Dav1dCdefDSPContext16};

/// Called by `cdef_dsp_init_*_8bpc` where the original assigns its NEON or
/// AVX2 kernels: the four slots get the kernels if the CPU has their level;
/// otherwise the C kernels stay.
pub fn install_8bpc(c: &mut Dav1dCdefDSPContext) {
    if let Some(k) = kernels_8bpc() {
        c.dir = Some(k.dir);
        c.fb = k.fb.map(Some);
    }
}

/// As `install_8bpc`, at 16 bpc (every high bit depth).
pub fn install_16bpc(c: &mut Dav1dCdefDSPContext16) {
    if let Some(k) = kernels_16bpc() {
        c.dir = Some(k.dir);
        c.fb = k.fb.map(Some);
    }
}

#[cfg(test)]
mod tests;
