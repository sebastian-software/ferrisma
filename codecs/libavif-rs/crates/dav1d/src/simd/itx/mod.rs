// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! The inverse transforms written from the algorithm
//! (`libavif-dav1d-kernels`, ADR 0062, "Kernels written from the
//! algorithm"): the kernels' table for the init mirrors (`simd/arm/itx.rs`,
//! `simd/x86/itx.rs`), which fill the slots where the original assigns its
//! NEON or AVX2 kernels, and the tests against the translated C.

pub use dav1d_kernels::itx::*;

/// The scan order of each block size: `dav1d_scans` (scan.c).
fn scans() -> Scans {
    core::array::from_fn(|tx| {
        let (w, h) = SIZES[tx];
        // SAFETY: `dav1d_scans` is a constant table of `min(w, 32) *
        // min(h, 32)` entries per size (scan.c); nothing writes it.
        unsafe {
            let scans = *core::ptr::addr_of!(crate::common::scan::dav1d_scans);
            core::slice::from_raw_parts(scans[tx], w.min(32) * h.min(32))
        }
    })
}

/// The 8 bpc kernels, if the CPU has their level (on x86-64 x86-64-v3;
/// without it the C kernels stay, with `all_simd` 0).
pub fn kernels_8bpc() -> Option<Kernels<ItxFn>> {
    Kernels::new_8bpc(&scans())
}

/// The 16 bpc kernels (10 and 12 bits), if the CPU has their level.
pub fn kernels_16bpc() -> Option<Kernels<ItxFn16>> {
    Kernels::new_16bpc(&scans())
}

#[cfg(test)]
mod tests;
