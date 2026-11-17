// Copyright © 2019, VideoLAN and dav1d authors
// Copyright © 2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `upstream/dav1d/src/x86/msac.h` for x86-64: the original renames six of
//! the seven entropy decoder calls of `src/msac.h` to the SSE2 functions at
//! compile time (`#define dav1d_msac_decode_symbol_adapt4
//! dav1d_msac_decode_symbol_adapt4_sse2`, …; SSE2 is part of x86-64), in
//! every unit that includes the header, and calls the 16-symbol function
//! through a pointer in the context, `(ctx)->symbol_adapt16(ctx, cdf,
//! symb)`, which `dav1d_msac_init` sets for every tile from the CPU flags:
//! the C function, then under `DAV1D_X86_CPU_FLAG_SSE2` the SSE2 function,
//! then under `DAV1D_X86_CPU_FLAG_AVX2` the AVX2 function (`msac_init_x86`).
//! The port has the AVX2 function and, below AVX2, the C function: the SSE2
//! 16-symbol function is selected by the original only on a CPU without
//! AVX2 and is not rebuilt (engine ADR-0041 section 4; ADR 0062, "Levels"),
//! as the SSSE3 and SSE4.1 kernels of the DSP families are not; there the
//! port runs its C kernel, as every DSP init mirror does.
//! The translation is made without assembly, so its call sites call the C
//! functions by name (`dav1d_msac_decode_symbol_adapt_c` for all three
//! symbol sizes); `tools/enable_dav1d_dispatch.py` (entry `msac_x86`)
//! renames each call site to the function below that the original's macro
//! names there.
//!
//! The six functions with a compile-time choice call the rebuilt SSE2
//! kernel (`simd/x86_64/msac.rs`) directly, as in the original; a call site
//! costs a direct call and nothing else, and `dav1d_set_cpu_flags_mask`
//! does not reach them, as it does not in the original.
//!
//! One more deviation, in `dav1d_msac_decode_symbol_adapt16`: the translated
//! `MsacContext` keeps the portable layout, without the `symbol_adapt16`
//! pointer that the original adds under `ARCH_X86_64 && HAVE_ASM` (the
//! layout is that of the portable original, which the ABI gate compares;
//! the context is embedded in `Dav1dTileState`). The function therefore
//! makes `msac_init_x86`'s choice from `dav1d_get_cpu_flags()` at every
//! call instead of once per tile: two loads, a test and a branch where the
//! original loads the pointer and calls through it. The flags do not change
//! while a decoder runs (`dav1d_set_cpu_flags_mask` is to be called before
//! a decoder is opened), so on an AVX2 machine the same kernel runs as in
//! the original. Decided 2026-11-17: the per-call choice and the portable
//! context layout stay (ADR 0062).
//!
//! The original's portable build (`-Denable_asm=false`) calls the C
//! functions. The port's counterpart is the build configuration
//! `--cfg dav1d_portable` (`tools/build_dav1d_consumers.py --portable`),
//! which the whole-image scalar gate decodes with; there each function
//! calls the translated C function the call site called before.
//! The functions take the call site's own translated `MsacContext` type
//! (each translated unit declares one; all have the original's portable
//! layout, see the test below) and hand it on as the kernel's context.
#![allow(clippy::all)]

use core::ffi::c_uint;

use super::super::cpu::{dav1d_get_cpu_flags, DAV1D_X86_CPU_FLAG_AVX2};
use super::super::x86_64::msac::*;
use crate::common::msac as c;

/// The original's `HAVE_ASM` for `src/x86/msac.h`: a constant, so that the
/// other branch is not compiled into the call sites.
#[inline(always)]
const fn asm() -> bool {
    !cfg!(dav1d_portable)
}

// Safety of the seven functions below: as for the C functions they replace
// (src/msac.h): `s` points to the decoder's live, initialized MsacContext
// and `cdf` to the CDF array the original passes at that call site (see
// simd/x86_64/msac.rs for what the kernels read and write).

/// `dav1d_msac_decode_symbol_adapt4` (n_symbols 1–3)
#[inline(always)]
pub unsafe fn dav1d_msac_decode_symbol_adapt4<S>(s: *mut S, cdf: *mut u16, n_symbols: usize) -> c_uint {
    if asm() {
        dav1d_msac_decode_symbol_adapt4_sse2(s.cast(), cdf, n_symbols)
    } else {
        c::dav1d_msac_decode_symbol_adapt_c(s.cast(), cdf, n_symbols)
    }
}

/// `dav1d_msac_decode_symbol_adapt8` (n_symbols 1–7)
#[inline(always)]
pub unsafe fn dav1d_msac_decode_symbol_adapt8<S>(s: *mut S, cdf: *mut u16, n_symbols: usize) -> c_uint {
    if asm() {
        dav1d_msac_decode_symbol_adapt8_sse2(s.cast(), cdf, n_symbols)
    } else {
        c::dav1d_msac_decode_symbol_adapt_c(s.cast(), cdf, n_symbols)
    }
}

/// `dav1d_msac_decode_symbol_adapt16` (n_symbols 3–15): the function
/// `msac_init_x86` stores in the context, chosen here (see the module
/// documentation): the AVX2 kernel under `DAV1D_X86_CPU_FLAG_AVX2`, the C
/// function otherwise (where the original has its SSE2 function).
#[inline(always)]
pub unsafe fn dav1d_msac_decode_symbol_adapt16<S>(s: *mut S, cdf: *mut u16, n_symbols: usize) -> c_uint {
    if asm() {
        let flags = dav1d_get_cpu_flags();
        if flags & DAV1D_X86_CPU_FLAG_AVX2 != 0 {
            // SAFETY: AVX2 is available (the flag is only set when CPUID
            // and XCR0 report it, simd/cpu.rs).
            return dav1d_msac_decode_symbol_adapt16_avx2(s.cast(), cdf, n_symbols);
        }
        // if (flags & DAV1D_X86_CPU_FLAG_SSE2) s->symbol_adapt16 =
        // dav1d_msac_decode_symbol_adapt16_sse2: not rebuilt, the C function
        // stays (module documentation).
    }
    c::dav1d_msac_decode_symbol_adapt_c(s.cast(), cdf, n_symbols)
}

/// `dav1d_msac_decode_hi_tok`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_hi_tok<S>(s: *mut S, cdf: *mut u16) -> c_uint {
    if asm() {
        dav1d_msac_decode_hi_tok_sse2(s.cast(), cdf)
    } else {
        c::dav1d_msac_decode_hi_tok_c(s.cast(), cdf)
    }
}

/// `dav1d_msac_decode_bool_adapt`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_bool_adapt<S>(s: *mut S, cdf: *mut u16) -> c_uint {
    if asm() {
        dav1d_msac_decode_bool_adapt_sse2(s.cast(), cdf)
    } else {
        c::dav1d_msac_decode_bool_adapt_c(s.cast(), cdf)
    }
}

/// `dav1d_msac_decode_bool_equi`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_bool_equi<S>(s: *mut S) -> c_uint {
    if asm() {
        dav1d_msac_decode_bool_equi_sse2(s.cast())
    } else {
        c::dav1d_msac_decode_bool_equi_c(s.cast())
    }
}

/// `dav1d_msac_decode_bool`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_bool<S>(s: *mut S, f: c_uint) -> c_uint {
    if asm() {
        dav1d_msac_decode_bool_sse2(s.cast(), f)
    } else {
        c::dav1d_msac_decode_bool_c(s.cast(), f)
    }
}

/// The seven kernels by their assembly names, for the per-kernel gate and
/// the instruction counts (`tools/check_dav1d_dsp.py --simd avx2`,
/// `tools/measure_dav1d_kernels.py`). The original's are hidden symbols of
/// its library; both sides reach them through an accessor
/// `port_table_<name>` (the C side's is compiled by the gate tool). Built
/// only with the test feature, so the library's exported symbols do not
/// change.
#[cfg(feature = "port-test-bridges")]
mod bridges {
    use super::*;
    use core::ffi::c_void;

    macro_rules! bridge {
        ($($accessor:ident => $kernel:ident),* $(,)?) => {$(
            #[no_mangle]
            pub extern "C" fn $accessor() -> *const c_void {
                $kernel as *const c_void
            }
        )*};
    }
    bridge! {
        port_table_dav1d_msac_decode_symbol_adapt4_sse2 => dav1d_msac_decode_symbol_adapt4_sse2,
        port_table_dav1d_msac_decode_symbol_adapt8_sse2 => dav1d_msac_decode_symbol_adapt8_sse2,
        port_table_dav1d_msac_decode_symbol_adapt16_avx2 => dav1d_msac_decode_symbol_adapt16_avx2,
        port_table_dav1d_msac_decode_hi_tok_sse2 => dav1d_msac_decode_hi_tok_sse2,
        port_table_dav1d_msac_decode_bool_adapt_sse2 => dav1d_msac_decode_bool_adapt_sse2,
        port_table_dav1d_msac_decode_bool_equi_sse2 => dav1d_msac_decode_bool_equi_sse2,
        port_table_dav1d_msac_decode_bool_sse2 => dav1d_msac_decode_bool_sse2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The kernels' context and every translated unit's `MsacContext`
    /// have the same layout (`struc msac` of msac.asm).
    #[test]
    fn context_layout_matches_the_assembly_and_the_translation() {
        use core::mem::{offset_of, size_of};
        assert_eq!(offset_of!(MsacContext, buf), 0);
        assert_eq!(offset_of!(MsacContext, end), 8);
        assert_eq!(offset_of!(MsacContext, dif), 16);
        assert_eq!(offset_of!(MsacContext, rng), 24);
        assert_eq!(offset_of!(MsacContext, cnt), 28);
        assert_eq!(offset_of!(MsacContext, update_cdf), 32);
        macro_rules! same {
            ($($t:ty),*) => {$(
                assert_eq!(size_of::<$t>(), size_of::<MsacContext>());
                assert_eq!(offset_of!($t, buf_pos), 0);
                assert_eq!(offset_of!($t, buf_end), 8);
                assert_eq!(offset_of!($t, dif), 16);
                assert_eq!(offset_of!($t, rng), 24);
                assert_eq!(offset_of!($t, cnt), 28);
                assert_eq!(offset_of!($t, allow_update_cdf), 32);
            )*};
        }
        same!(
            crate::common::msac::MsacContext,
            crate::common::decode::MsacContext,
            crate::b8::recon_tmpl::MsacContext,
            crate::b16::recon_tmpl::MsacContext
        );
    }

    /// Every kernel decodes what the C function decodes from the same
    /// stream, CDF and context (the original checkasm test does this
    /// against the assembly; this is the quick in-crate version). The
    /// streams end inside the test, so the refill near the end of the
    /// buffer runs for every kernel.
    #[test]
    fn kernels_follow_the_c_functions() {
        use crate::common::msac::{dav1d_msac_init, MsacContext as C};
        let avx2 = std::arch::is_x86_feature_detected!("avx2");
        let mut seed = 0x1234_5678u32;
        let mut rnd = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed >> 8
        };
        // 32-byte alignment, as the original's CDF arrays have.
        #[repr(C, align(32))]
        struct Cdf([u16; 16]);
        for len in [128usize, 61, 9, 5] {
            let buf: Vec<u8> = (0..len).map(|_| rnd() as u8).collect();
            unsafe {
                for kind in 0..7 {
                    if kind == 2 && !avx2 {
                        continue;
                    }
                    for update in 0..2 {
                        for ns in 1..=15usize {
                            let ok = match kind {
                                0 => ns <= 3,
                                1 => ns <= 7,
                                2 => ns >= 3,
                                _ => ns == 3,
                            };
                            if !ok {
                                continue;
                            }
                            let mut a: C = core::mem::zeroed();
                            dav1d_msac_init(&mut a, buf.as_ptr(), buf.len(), (update == 0) as i32);
                            let mut b: C = core::ptr::read(&a);
                            let pb = (&mut b as *mut C).cast::<MsacContext>();
                            // randomize_cdf of the original test: cdf[n] is the count (0).
                            let n = if (4..7).contains(&kind) { 1 } else { ns };
                            let mut cdf = Cdf([0u16; 16]);
                            for i in (1..=n).rev() {
                                let next = cdf.0[i] as u32;
                                cdf.0[i - 1] = (next + rnd() % (32768 - next - i as u32) + 1) as u16;
                            }
                            let mut cdf_b = Cdf(cdf.0);
                            let (pa, pc) = (cdf.0.as_mut_ptr(), cdf_b.0.as_mut_ptr());
                            let mut f = 16384u32;
                            while a.cnt >= 0 {
                                f = (f * 7 + 1) & 0x7fff;
                                let (ra, rb) = match kind {
                                    0 => (
                                        c::dav1d_msac_decode_symbol_adapt_c(&mut a, pa, ns),
                                        dav1d_msac_decode_symbol_adapt4_sse2(pb, pc, ns),
                                    ),
                                    1 => (
                                        c::dav1d_msac_decode_symbol_adapt_c(&mut a, pa, ns),
                                        dav1d_msac_decode_symbol_adapt8_sse2(pb, pc, ns),
                                    ),
                                    2 => (
                                        c::dav1d_msac_decode_symbol_adapt_c(&mut a, pa, ns),
                                        dav1d_msac_decode_symbol_adapt16_avx2(pb, pc, ns),
                                    ),
                                    3 => (
                                        c::dav1d_msac_decode_hi_tok_c(&mut a, pa),
                                        dav1d_msac_decode_hi_tok_sse2(pb, pc),
                                    ),
                                    4 => (
                                        c::dav1d_msac_decode_bool_adapt_c(&mut a, pa),
                                        dav1d_msac_decode_bool_adapt_sse2(pb, pc),
                                    ),
                                    5 => (
                                        c::dav1d_msac_decode_bool_equi_c(&mut a),
                                        dav1d_msac_decode_bool_equi_sse2(pb),
                                    ),
                                    _ => (
                                        c::dav1d_msac_decode_bool_c(&mut a, f),
                                        dav1d_msac_decode_bool_sse2(pb, f),
                                    ),
                                };
                                assert_eq!(ra, rb, "kind {kind} ns {ns} len {len}");
                                assert_eq!(
                                    (a.buf_pos, a.rng, a.cnt),
                                    (b.buf_pos, b.rng, b.cnt),
                                    "kind {kind} ns {ns} len {len}"
                                );
                                // msac_cmp of the original checkasm test: the valid bits of dif.
                                let mask = !(!0usize >> (a.cnt.max(0) + 16));
                                assert_eq!((a.dif ^ b.dif) & mask, 0, "kind {kind} ns {ns} len {len}");
                                assert_eq!(cdf.0[..=n], cdf_b.0[..=n], "kind {kind} ns {ns} len {len}");
                            }
                        }
                    }
                }
            }
        }
    }

    /// `dav1d_msac_decode_symbol_adapt16` follows the CPU flags: the AVX2
    /// kernel when the flags allow it, and the C function below AVX2 (a
    /// mask of SSE2 alone, where the original has its SSE2 function, and
    /// with every flag masked off, where the original's `dav1d_msac_init`
    /// leaves the C pointer as well). Both give the same symbols; the test
    /// checks that each choice decodes.
    #[test]
    fn symbol_adapt16_follows_the_cpu_flags() {
        use crate::common::cpu::{dav1d_init_cpu, dav1d_set_cpu_flags_mask};
        use crate::common::msac::{dav1d_msac_init, MsacContext as C};
        use crate::simd::cpu::DAV1D_X86_CPU_FLAG_SSE2;
        let _guard = crate::simd::cpu::CPU_MASK_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        #[repr(C, align(32))]
        struct Cdf([u16; 16]);
        let buf: [u8; 64] = core::array::from_fn(|i| (i * 37 + 11) as u8);
        let decode = |mask: u32| unsafe {
            dav1d_init_cpu();
            dav1d_set_cpu_flags_mask(mask);
            let mut s: C = core::mem::zeroed();
            dav1d_msac_init(&mut s, buf.as_ptr(), buf.len(), 0);
            let mut cdf = Cdf([0u16; 16]);
            for i in 0..15 {
                cdf.0[i] = (32768 - 2048 * (i as u32 + 1)) as u16;
            }
            let symbols: Vec<u32> = (0..40)
                .map(|_| dav1d_msac_decode_symbol_adapt16(&mut s, cdf.0.as_mut_ptr(), 15))
                .collect();
            dav1d_set_cpu_flags_mask(!0);
            (symbols, cdf.0)
        };
        let all = decode(!0);
        assert_eq!(all, decode(DAV1D_X86_CPU_FLAG_SSE2));
        assert_eq!(all, decode(0));
    }
}
