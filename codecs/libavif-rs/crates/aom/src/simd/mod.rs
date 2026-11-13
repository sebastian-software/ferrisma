//! Hand-translated SIMD specializations of libaom's run-time CPU dispatch
//! (RTCD) functions, ADR 0056.
//!
//! Each file mirrors one original `*_neon.c`, `*_sse2.c`, `*_sse4.c` or
//! `*_avx2.c` (or a helper header) under the same relative path below
//! `upstream/aom`, and keeps its function names, helper structure, and
//! intrinsic sequence. The translated scalar code calls these kernels only
//! through the RTCD function pointers that `aom_dsp_rtcd()` and `av1_rtcd()`
//! assign from `aom_arm_cpu_caps()` on AArch64 and `x86_simd_caps()` on
//! x86-64 (ADR 0058), so the `_c` implementations stay selectable at run time
//! with `AOM_SIMD_CAPS_MASK=0`. x86 functions carry the `target_feature` of the
//! compiler flag upstream builds their file with. Every exported kernel must produce the same output
//! as its `_c` counterpart for all inputs the original tests accept.
#![allow(clippy::all)]

/// C scalar types used in the original kernel signatures.
pub mod types {
    pub type tran_low_t = i32;
    pub type qm_val_t = u8;
}

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/highbd_intrapred_neon.rs"]
pub mod aom_dsp__arm__highbd_intrapred_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/highbd_loopfilter_neon.rs"]
pub mod aom_dsp__arm__highbd_loopfilter_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/highbd_quantize_neon.rs"]
pub mod aom_dsp__arm__highbd_quantize_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/loopfilter_neon.rs"]
pub mod aom_dsp__arm__loopfilter_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/intrapred_neon.rs"]
pub mod aom_dsp__arm__intrapred_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/mem_neon.rs"]
pub mod aom_dsp__arm__mem_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/subtract_neon.rs"]
pub mod aom_dsp__arm__subtract_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/sum_neon.rs"]
pub mod aom_dsp__arm__sum_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_dsp/arm/transpose_neon.rs"]
pub mod aom_dsp__arm__transpose_neon;

#[cfg(target_arch = "aarch64")]
#[path = "aom_ports/aarch64_cpudetect.rs"]
pub mod aom_ports__aarch64_cpudetect;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/av1_inv_txfm_neon.rs"]
pub mod av1__common__arm__av1_inv_txfm_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/cdef_block_neon.rs"]
pub mod av1__common__arm__cdef_block_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/highbd_inv_txfm_neon.rs"]
pub mod av1__common__arm__highbd_inv_txfm_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/highbd_reconintra_neon.rs"]
pub mod av1__common__arm__highbd_reconintra_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/highbd_wiener_convolve_neon.rs"]
pub mod av1__common__arm__highbd_wiener_convolve_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/reconintra_neon.rs"]
pub mod av1__common__arm__reconintra_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/selfguided_neon.rs"]
pub mod av1__common__arm__selfguided_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/wiener_convolve_neon.rs"]
pub mod av1__common__arm__wiener_convolve_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/av1_txfm.rs"]
pub mod av1__common__av1_txfm;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/av1_error_neon.rs"]
pub mod av1__encoder__arm__av1_error_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/av1_highbd_quantize_neon.rs"]
pub mod av1__encoder__arm__av1_highbd_quantize_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/highbd_fwd_txfm_neon.rs"]
pub mod av1__encoder__arm__highbd_fwd_txfm_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/highbd_rdopt_neon.rs"]
pub mod av1__encoder__arm__highbd_rdopt_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/quantize_neon.rs"]
pub mod av1__encoder__arm__quantize_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/shift_neon.rs"]
pub mod av1__encoder__arm__shift_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/txfm_neon.rs"]
pub mod av1__encoder__arm__txfm_neon;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/simd/v128_intrinsics_x86.rs"]
pub mod aom_dsp__simd__v128_intrinsics_x86;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/simd/v256_intrinsics_x86.rs"]
pub mod aom_dsp__simd__v256_intrinsics_x86;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/simd/v64_intrinsics_x86.rs"]
pub mod aom_dsp__simd__v64_intrinsics_x86;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/highbd_adaptive_quantize_avx2.rs"]
pub mod aom_dsp__x86__highbd_adaptive_quantize_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/highbd_adaptive_quantize_sse2.rs"]
pub mod aom_dsp__x86__highbd_adaptive_quantize_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/highbd_intrapred_sse2.rs"]
pub mod aom_dsp__x86__highbd_intrapred_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/highbd_quantize_intrin_avx2.rs"]
pub mod aom_dsp__x86__highbd_quantize_intrin_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/highbd_subtract_sse2.rs"]
pub mod aom_dsp__x86__highbd_subtract_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/intrapred_avx2.rs"]
pub mod aom_dsp__x86__intrapred_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/lpf_common_sse2.rs"]
pub mod aom_dsp__x86__lpf_common_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/quantize_x86.rs"]
pub mod aom_dsp__x86__quantize_x86;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/subtract_avx2.rs"]
pub mod aom_dsp__x86__subtract_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/synonyms.rs"]
pub mod aom_dsp__x86__synonyms;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/synonyms_avx2.rs"]
pub mod aom_dsp__x86__synonyms_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/transpose_sse2.rs"]
pub mod aom_dsp__x86__transpose_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/txfm_common_avx2.rs"]
pub mod aom_dsp__x86__txfm_common_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "aom_dsp/x86/txfm_common_sse2.rs"]
pub mod aom_dsp__x86__txfm_common_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/av1_inv_txfm_ssse3.rs"]
pub mod av1__common__x86__av1_inv_txfm_ssse3;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/av1_txfm_sse2.rs"]
pub mod av1__common__x86__av1_txfm_sse2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/av1_txfm_sse4.rs"]
pub mod av1__common__x86__av1_txfm_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/cdef_block_avx2.rs"]
pub mod av1__common__x86__cdef_block_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/filterintra_sse4.rs"]
pub mod av1__common__x86__filterintra_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/highbd_inv_txfm_avx2.rs"]
pub mod av1__common__x86__highbd_inv_txfm_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/highbd_inv_txfm_sse4.rs"]
pub mod av1__common__x86__highbd_inv_txfm_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/highbd_txfm_utility_sse4.rs"]
pub mod av1__common__x86__highbd_txfm_utility_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/common/x86/intra_edge_sse4.rs"]
pub mod av1__common__x86__intra_edge_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/av1_fwd_txfm1d_sse4.rs"]
pub mod av1__encoder__x86__av1_fwd_txfm1d_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/av1_fwd_txfm2d_sse4.rs"]
pub mod av1__encoder__x86__av1_fwd_txfm2d_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/av1_highbd_quantize_avx2.rs"]
pub mod av1__encoder__x86__av1_highbd_quantize_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/av1_txfm1d_sse4.rs"]
pub mod av1__encoder__x86__av1_txfm1d_sse4;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/error_intrin_avx2.rs"]
pub mod av1__encoder__x86__error_intrin_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/highbd_block_error_intrin_avx2.rs"]
pub mod av1__encoder__x86__highbd_block_error_intrin_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/highbd_fwd_txfm_avx2.rs"]
pub mod av1__encoder__x86__highbd_fwd_txfm_avx2;

#[cfg(target_arch = "x86_64")]
#[path = "av1/encoder/x86/highbd_fwd_txfm_sse4.rs"]
pub mod av1__encoder__x86__highbd_fwd_txfm_sse4;
