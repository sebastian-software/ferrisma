//! Hand-translated SIMD specializations of libaom's run-time CPU dispatch
//! (RTCD) functions, ADR 0056.
//!
//! Each file mirrors one original `*_neon.c` (or helper `*_neon.h`) under the
//! same relative path below `upstream/aom`, and keeps its function names,
//! helper structure, and intrinsic sequence. The translated scalar code calls
//! these kernels only through the RTCD function pointers that
//! `aom_dsp_rtcd()` and `av1_rtcd()` assign from `aom_arm_cpu_caps()`, so the
//! `_c` implementations stay selectable at run time with
//! `AOM_SIMD_CAPS_MASK=0`. Every exported kernel must produce the same output
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
#[path = "aom_dsp/arm/highbd_quantize_neon.rs"]
pub mod aom_dsp__arm__highbd_quantize_neon;

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
#[path = "av1/common/arm/cdef_block_neon.rs"]
pub mod av1__common__arm__cdef_block_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/highbd_inv_txfm_neon.rs"]
pub mod av1__common__arm__highbd_inv_txfm_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/highbd_reconintra_neon.rs"]
pub mod av1__common__arm__highbd_reconintra_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/common/arm/reconintra_neon.rs"]
pub mod av1__common__arm__reconintra_neon;

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
#[path = "av1/encoder/arm/shift_neon.rs"]
pub mod av1__encoder__arm__shift_neon;

#[cfg(target_arch = "aarch64")]
#[path = "av1/encoder/arm/txfm_neon.rs"]
pub mod av1__encoder__arm__txfm_neon;
