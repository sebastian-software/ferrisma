//! Supplementary SIMD-versus-C comparisons for the hand-translated kernels in
//! `aom::simd` (ADR 0056). Each module transcribes the exact-match cases of the
//! original libaom test that instantiates the corresponding Neon functions.
//! Run with `cargo test --release -p libavif-aom --test simd`.
#![allow(non_snake_case, non_upper_case_globals)]

#[path = "simd/acm_random.rs"]
mod acm_random;
#[path = "simd/cdef.rs"]
mod cdef;
#[path = "simd/quantize.rs"]
mod quantize;
