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
#[path = "simd/lowbd_inv_txfm.rs"]
mod lowbd_inv_txfm;
#[path = "simd/quantize.rs"]
mod quantize;
#[path = "simd/error_block.rs"]
mod error_block;
#[path = "simd/subtract.rs"]
mod subtract;
#[path = "simd/highbd_fwd_txfm.rs"]
mod highbd_fwd_txfm;
#[path = "simd/lowbd_fwd_txfm.rs"]
mod lowbd_fwd_txfm;
#[path = "simd/fwht4x4.rs"]
mod fwht4x4;
#[path = "simd/sum_squares.rs"]
mod sum_squares;
#[path = "simd/sse_sum.rs"]
mod sse_sum;
#[path = "simd/horver_correlation.rs"]
mod horver_correlation;
#[path = "simd/encodetxb.rs"]
mod encodetxb;
#[path = "simd/mse_16bit.rs"]
mod mse_16bit;
#[path = "simd/blk_sse_sum.rs"]
mod blk_sse_sum;
#[path = "simd/round_shift_array.rs"]
mod round_shift_array;
#[path = "simd/intrapred.rs"]
mod intrapred;
#[path = "simd/lowbd_intrapred.rs"]
mod lowbd_intrapred;
#[path = "simd/dr_prediction.rs"]
mod dr_prediction;
#[path = "simd/filterintra.rs"]
mod filterintra;
#[path = "simd/intra_edge.rs"]
mod intra_edge;
#[path = "simd/loopfilter.rs"]
mod loopfilter;
#[path = "simd/restoration.rs"]
mod restoration;
#[path = "simd/cfl.rs"]
mod cfl;

/// The original's C references (`av1_fwd_txfm2d_*_c`, the C inverse 2D
/// transforms) call `av1_round_shift_array` through its RTCD pointer, which
/// `av1_rtcd()` sets up in the original's test main. The transcriptions call
/// those references without that setup, so they select the `_c` round shift,
/// as `AOM_SIMD_CAPS_MASK=0` does, and keep the reference scalar throughout.
/// The store is idempotent, so concurrent tests may repeat it. The x86-64
/// overlay does not dispatch this function, so there is nothing to select.
pub(crate) fn scalar_round_shift_reference() {
    #[cfg(target_arch = "aarch64")]
    {
        extern "C-unwind" {
            fn av1_round_shift_array_c(arr: *mut i32, size: i32, bit: i32);
        }
        // SAFETY: a plain store of the `_c` function into its RTCD pointer.
        unsafe {
            aom::av1__common__av1_rtcd::av1_round_shift_array = Some(av1_round_shift_array_c);
        }
    }
}
