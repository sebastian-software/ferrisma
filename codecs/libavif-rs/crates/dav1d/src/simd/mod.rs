//! Kernels rebuilt from dav1d's assembly with `core::arch` intrinsics
//! (engine ADR-0044 D2; libavif-rs ADR 0062).
//!
//! dav1d 1.5.4 has its DSP kernels only in assembly. The translation is made
//! with `-Denable_asm=false`, so its `*_dsp_init` functions assign the C
//! kernels and `dav1d_init_cpu` reports nothing. `tools/enable_dav1d_dispatch.py`
//! makes two reproducible edits in the translated units (each with a check
//! mode): `dav1d_init_cpu` stores the flags of `cpu::dav1d_get_cpu_flags_native`,
//! and each enabled `dav1d_<family>_dsp_init_<bpc>` calls the matching
//! `<family>_dsp_init_<arch>_<bpc>` here where the original calls its
//! `*_dsp_init_arm/_x86`. The C kernels stay selectable with
//! `dav1d_set_cpu_flags_mask(0)`.
//!
//! Layout below `simd/`: `arm64/<file>.rs` rebuilds `src/arm/64/<file>.S`,
//! `arm/<family>.rs` the init header `src/arm/<family>.h`. The original keeps
//! its x86 headers and assembly in one directory (`src/x86/`); here
//! `x86/<family>.rs` is the header `src/x86/<family>.h` and
//! `x86_64/<file>.rs` rebuilds `src/x86/<file>.asm` in its x86-64 form.
#![allow(clippy::all)]

pub mod cpu;

#[cfg(target_arch = "aarch64")]
pub mod arm64 {
    pub mod cdef;
    pub mod cdef16;
    pub mod ipred;
    pub mod ipred16;
    pub mod itx;
    pub mod itx16;
    pub mod loopfilter;
    pub mod loopfilter16;
    pub mod looprestoration;
    pub mod looprestoration16;
    pub mod looprestoration_common;
    pub mod looprestoration_tmpl;
    pub mod msac;
    pub mod util;
}

#[cfg(target_arch = "aarch64")]
pub mod arm {
    pub mod cdef;
    pub mod ipred;
    pub mod itx;
    pub mod loopfilter;
    pub mod looprestoration;
    pub mod msac;
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub mod cdef16_avx2;
    pub mod cdef_avx2;
    pub mod itx16_avx2;
    pub mod itx_avx2;
    pub mod loopfilter16_avx2;
    pub mod loopfilter_avx2;
    pub mod msac;
    pub mod util;
}

#[cfg(target_arch = "x86_64")]
pub mod x86 {
    pub mod cdef;
    pub mod itx;
    pub mod loopfilter;
    pub mod msac;
}
