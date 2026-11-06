//! Supplementary SIMD-versus-C comparisons for the hand-translated kernels in
//! `aom::simd` (ADR 0056). Each module transcribes the exact-match cases of the
//! original libaom test that instantiates the corresponding Neon functions.
//! Run with `cargo test --release -p libavif-aom --test simd`.
#![allow(non_snake_case, non_upper_case_globals)]

#[path = "simd/acm_random.rs"]
mod acm_random;
#[path = "simd/cdef.rs"]
mod cdef;
#[path = "simd/highbd_inv_txfm.rs"]
mod highbd_inv_txfm;
#[path = "simd/quantize.rs"]
mod quantize;
#[path = "simd/error_block.rs"]
mod error_block;
#[path = "simd/subtract.rs"]
mod subtract;
#[path = "simd/highbd_fwd_txfm.rs"]
mod highbd_fwd_txfm;
#[path = "simd/intrapred.rs"]
mod intrapred;
#[path = "simd/dr_prediction.rs"]
mod dr_prediction;
#[path = "simd/filterintra.rs"]
mod filterintra;
#[path = "simd/intra_edge.rs"]
mod intra_edge;
