// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! dav1d's CDEF (`cdef_tmpl.c`) and inverse transforms (`itx_tmpl.c`,
//! `itx_1d.c`) written from the algorithm on the portable vector types of
//! `fearless_simd`: one implementation for NEON on AArch64 and AVX2
//! (x86-64-v3) on x86-64 (libavif-rs ADR 0062, "Kernels written from the
//! algorithm"). `libavif-dav1d` installs them in its DSP tables, where the
//! original assigns its NEON or AVX2 assembly, and holds their tests against
//! the translated C.
//!
//! A crate of its own so that it can be compiled optimized in the `dev`
//! profile while the translated decoder stays at opt-level 0 (workspace
//! `Cargo.toml`): `fearless_simd`'s generic code relies on inlining, and
//! unoptimized it needs stack frames of megabytes.

// The row loops walk fixed-size chunks with `chunks_exact`, as the gated
// kernels were written; `as_chunks` would restate them without a functional
// difference.
#![allow(clippy::chunks_exact_to_as_chunks)]

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
compile_error!("The dav1d kernels target AArch64 NEON and x86-64 AVX2");

pub mod cdef;
pub mod itx;
