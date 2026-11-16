// Copyright © 2019, VideoLAN and dav1d authors
// Copyright © 2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! `upstream/dav1d/src/arm/msac.h`: the original renames the seven entropy
//! decoder calls of `src/msac.h` to the NEON functions at compile time
//! (`#define dav1d_msac_decode_symbol_adapt4 dav1d_msac_decode_symbol_adapt4_neon`,
//! …), in every unit that includes the header. The translation is made
//! without assembly, so its call sites call the C functions by name
//! (`dav1d_msac_decode_symbol_adapt_c` for all three symbol sizes);
//! `tools/enable_dav1d_dispatch.py` (entry `msac_arm`) renames each call
//! site to the function below that the original's macro names there.
//!
//! Each function calls the rebuilt NEON kernel (`simd/arm64/msac.rs`); the
//! choice is made at compile time, as the original makes it (NEON is
//! baseline on AArch64), so a call site costs a direct call and nothing
//! else. A run-time check of `dav1d_get_cpu_flags()` here (the form until
//! 2026-11-16) cost two loads and a branch at every symbol and kept both
//! paths in the callers: 81 million instructions of the 1.44 G of the 8-bit
//! 4:2:0 still, and it doubled the self time of the inlined callers
//! (`decode_coefs`) on the profile (ADR 0062).
//!
//! The original's portable build (`-Denable_asm=false`) calls the C
//! functions. The port's counterpart is the build configuration
//! `--cfg dav1d_portable` (`tools/build_dav1d_consumers.py --portable`),
//! which the whole-image scalar gate decodes with; there each function
//! calls the translated C function the call site called before.
//! The functions take the call site's own translated `MsacContext` type
//! (each translated unit declares one; all have the original's layout, see
//! the test below) and hand it on as the kernel's context.
#![allow(clippy::all)]

use core::ffi::c_uint;

use super::super::arm64::msac::*;
use crate::common::msac as c;

/// The original's `HAVE_ASM` for `src/arm/msac.h`: a constant, so that the
/// other branch is not compiled into the call sites.
#[inline(always)]
const fn neon() -> bool {
    !cfg!(dav1d_portable)
}

// Safety of the seven functions below: as for the C functions they replace
// (src/msac.h): `s` points to the decoder's live, initialized MsacContext
// and `cdf` to the CDF array the original passes at that call site (see
// simd/arm64/msac.rs for what the kernels read and write).

/// `dav1d_msac_decode_symbol_adapt4` (n_symbols 1–3)
#[inline(always)]
pub unsafe fn dav1d_msac_decode_symbol_adapt4<S>(s: *mut S, cdf: *mut u16, n_symbols: usize) -> c_uint {
    if neon() {
        dav1d_msac_decode_symbol_adapt4_neon(s.cast(), cdf, n_symbols)
    } else {
        c::dav1d_msac_decode_symbol_adapt_c(s.cast(), cdf, n_symbols)
    }
}

/// `dav1d_msac_decode_symbol_adapt8` (n_symbols 1–7)
#[inline(always)]
pub unsafe fn dav1d_msac_decode_symbol_adapt8<S>(s: *mut S, cdf: *mut u16, n_symbols: usize) -> c_uint {
    if neon() {
        dav1d_msac_decode_symbol_adapt8_neon(s.cast(), cdf, n_symbols)
    } else {
        c::dav1d_msac_decode_symbol_adapt_c(s.cast(), cdf, n_symbols)
    }
}

/// `dav1d_msac_decode_symbol_adapt16` (n_symbols 3–15)
#[inline(always)]
pub unsafe fn dav1d_msac_decode_symbol_adapt16<S>(s: *mut S, cdf: *mut u16, n_symbols: usize) -> c_uint {
    if neon() {
        dav1d_msac_decode_symbol_adapt16_neon(s.cast(), cdf, n_symbols)
    } else {
        c::dav1d_msac_decode_symbol_adapt_c(s.cast(), cdf, n_symbols)
    }
}

/// `dav1d_msac_decode_hi_tok`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_hi_tok<S>(s: *mut S, cdf: *mut u16) -> c_uint {
    if neon() {
        dav1d_msac_decode_hi_tok_neon(s.cast(), cdf)
    } else {
        c::dav1d_msac_decode_hi_tok_c(s.cast(), cdf)
    }
}

/// `dav1d_msac_decode_bool_adapt`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_bool_adapt<S>(s: *mut S, cdf: *mut u16) -> c_uint {
    if neon() {
        dav1d_msac_decode_bool_adapt_neon(s.cast(), cdf)
    } else {
        c::dav1d_msac_decode_bool_adapt_c(s.cast(), cdf)
    }
}

/// `dav1d_msac_decode_bool_equi`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_bool_equi<S>(s: *mut S) -> c_uint {
    if neon() {
        dav1d_msac_decode_bool_equi_neon(s.cast())
    } else {
        c::dav1d_msac_decode_bool_equi_c(s.cast())
    }
}

/// `dav1d_msac_decode_bool`
#[inline(always)]
pub unsafe fn dav1d_msac_decode_bool<S>(s: *mut S, f: c_uint) -> c_uint {
    if neon() {
        dav1d_msac_decode_bool_neon(s.cast(), f)
    } else {
        c::dav1d_msac_decode_bool_c(s.cast(), f)
    }
}

/// The seven kernels by their assembly names, for the per-kernel gate and
/// the instruction counts (`tools/check_dav1d_dsp.py --simd neon`,
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
        port_table_dav1d_msac_decode_symbol_adapt4_neon => dav1d_msac_decode_symbol_adapt4_neon,
        port_table_dav1d_msac_decode_symbol_adapt8_neon => dav1d_msac_decode_symbol_adapt8_neon,
        port_table_dav1d_msac_decode_symbol_adapt16_neon => dav1d_msac_decode_symbol_adapt16_neon,
        port_table_dav1d_msac_decode_hi_tok_neon => dav1d_msac_decode_hi_tok_neon,
        port_table_dav1d_msac_decode_bool_adapt_neon => dav1d_msac_decode_bool_adapt_neon,
        port_table_dav1d_msac_decode_bool_equi_neon => dav1d_msac_decode_bool_equi_neon,
        port_table_dav1d_msac_decode_bool_neon => dav1d_msac_decode_bool_neon,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The kernels' context and every translated unit's `MsacContext`
    /// have the same layout (the offsets `BUF_POS` … `ALLOW_UPDATE_CDF`
    /// of msac.S).
    #[test]
    fn context_layout_matches_the_assembly_and_the_translation() {
        use core::mem::{offset_of, size_of};
        assert_eq!(offset_of!(MsacContext, buf_pos), 0);
        assert_eq!(offset_of!(MsacContext, buf_end), 8);
        assert_eq!(offset_of!(MsacContext, dif), 16);
        assert_eq!(offset_of!(MsacContext, rng), 24);
        assert_eq!(offset_of!(MsacContext, cnt), 28);
        assert_eq!(offset_of!(MsacContext, allow_update_cdf), 32);
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
    /// against the assembly; this is the quick in-crate version).
    #[test]
    fn kernels_follow_the_c_functions() {
        use crate::common::msac::{dav1d_msac_init, MsacContext as C};
        let mut seed = 0x1234_5678u32;
        let mut rnd = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed >> 8
        };
        let buf: [u8; 128] = core::array::from_fn(|_| rnd() as u8);
        unsafe {
            for kind in 0..7 {
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
                        // randomize_cdf of the original test: cdf[n] is the count (0).
                        let n = if kind >= 4 { 1 } else { ns };
                        let mut cdf = [0u16; 16];
                        for i in (1..=n).rev() {
                            let next = cdf[i] as u32;
                            cdf[i - 1] = (next + rnd() % (32768 - next - i as u32) + 1) as u16;
                        }
                        let mut cdf_b = cdf;
                        let mut f = 16384u32;
                        while a.cnt >= 0 {
                            f = (f * 7 + 1) & 0x7fff;
                            let (ra, rb) = match kind {
                                0 => (
                                    c::dav1d_msac_decode_symbol_adapt_c(&mut a, cdf.as_mut_ptr(), ns),
                                    dav1d_msac_decode_symbol_adapt4_neon((&mut b as *mut C).cast(), cdf_b.as_mut_ptr(), ns),
                                ),
                                1 => (
                                    c::dav1d_msac_decode_symbol_adapt_c(&mut a, cdf.as_mut_ptr(), ns),
                                    dav1d_msac_decode_symbol_adapt8_neon((&mut b as *mut C).cast(), cdf_b.as_mut_ptr(), ns),
                                ),
                                2 => (
                                    c::dav1d_msac_decode_symbol_adapt_c(&mut a, cdf.as_mut_ptr(), ns),
                                    dav1d_msac_decode_symbol_adapt16_neon((&mut b as *mut C).cast(), cdf_b.as_mut_ptr(), ns),
                                ),
                                3 => (
                                    c::dav1d_msac_decode_hi_tok_c(&mut a, cdf.as_mut_ptr()),
                                    dav1d_msac_decode_hi_tok_neon((&mut b as *mut C).cast(), cdf_b.as_mut_ptr()),
                                ),
                                4 => (
                                    c::dav1d_msac_decode_bool_adapt_c(&mut a, cdf.as_mut_ptr()),
                                    dav1d_msac_decode_bool_adapt_neon((&mut b as *mut C).cast(), cdf_b.as_mut_ptr()),
                                ),
                                5 => (
                                    c::dav1d_msac_decode_bool_equi_c(&mut a),
                                    dav1d_msac_decode_bool_equi_neon((&mut b as *mut C).cast()),
                                ),
                                _ => (
                                    c::dav1d_msac_decode_bool_c(&mut a, f),
                                    dav1d_msac_decode_bool_neon((&mut b as *mut C).cast(), f),
                                ),
                            };
                            assert_eq!(ra, rb, "kind {kind} ns {ns}");
                            assert_eq!((a.buf_pos, a.rng, a.cnt), (b.buf_pos, b.rng, b.cnt), "kind {kind} ns {ns}");
                            // msac_cmp of the original checkasm test: the valid bits of dif.
                            let mask = !(!0usize >> (a.cnt.max(0) + 16));
                            assert_eq!((a.dif ^ b.dif) & mask, 0, "kind {kind} ns {ns}");
                            assert_eq!(cdf[..=n], cdf_b[..=n], "kind {kind} ns {ns}");
                        }
                    }
                }
            }
        }
    }
}
