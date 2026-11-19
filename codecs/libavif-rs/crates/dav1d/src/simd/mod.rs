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

/// Not in the original: copies the pixels `topleft[from..=to]` of an intra prediction edge into `buf`, at the
/// same offsets from `buf[at]`, and returns the copy's `topleft`. `buf` is zero-initialized by the caller.
///
/// The rebuilt kernels load whole vectors around the edge, beyond the pixels the C kernel reads and, for some
/// block sizes, beyond the array the caller's pointer belongs to (the decoder's `edge[257]`, with `topleft` at
/// index 128, or at 32 for inter-intra prediction). The values of those pixels never reach the output, so a
/// kernel that can read outside the array runs on such a copy: `from..=to` is what the C kernel reads, and `buf`
/// is large enough for every load the kernel makes around `buf[at]`.
///
/// # Safety
/// `topleft[from..=to]` is readable. Panics if that range does not fit into `buf` around `at`.
#[inline(always)]
pub(crate) unsafe fn copy_edge<P: Copy, const N: usize>(
    buf: &mut [P; N],
    at: usize,
    topleft: *const P,
    from: isize,
    to: isize,
) -> *const P {
    let start = at.checked_add_signed(from).expect("edge copy starts before the buffer");
    let len = (to - from + 1) as usize;
    buf[start..start + len].copy_from_slice(core::slice::from_raw_parts(topleft.offset(from), len));
    buf.as_mut_ptr().add(at)
}

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
    pub mod ipred16_avx2;
    pub mod ipred_avx2;
    pub mod itx16_avx2;
    pub mod itx_avx2;
    pub mod loopfilter16_avx2;
    pub mod loopfilter_avx2;
    pub mod msac;
}

#[cfg(target_arch = "x86_64")]
pub mod x86 {
    pub mod cdef;
    pub mod ipred;
    pub mod itx;
    pub mod loopfilter;
    pub mod msac;
}

/// Test support for the edge contract of the intra prediction kernels (`copy_edge`): the slots run with the edge
/// array placed against memory that faults on access.
#[cfg(test)]
pub(crate) mod edge_test {
    use core::ffi::c_int;

    // `enum IntraPredMode` (src/levels.h): the slots `intra_pred` has.
    const DC_PRED: usize = 0;
    const VERT_PRED: usize = 1;
    const HOR_PRED: usize = 2;
    const Z1_PRED: usize = 6;
    const Z2_PRED: usize = 7;
    const Z3_PRED: usize = 8;
    const SMOOTH_PRED: usize = 9;
    const FILTER_PRED: usize = 13;

    /// The block sizes of the intra prediction slots (the transform sizes).
    const SIZES: [(c_int, c_int); 19] = [
        (4, 4),
        (4, 8),
        (4, 16),
        (8, 4),
        (8, 8),
        (8, 16),
        (8, 32),
        (16, 4),
        (16, 8),
        (16, 16),
        (16, 32),
        (16, 64),
        (32, 8),
        (32, 16),
        (32, 32),
        (32, 64),
        (64, 16),
        (64, 32),
        (64, 64),
    ];

    /// One page of readable and writable memory between two pages without access.
    struct Guarded {
        base: *mut u8,
        page: usize,
    }

    impl Guarded {
        fn new() -> Self {
            // SAFETY: an anonymous private mapping of three pages; the outer two lose all access.
            unsafe {
                let page = libc::sysconf(libc::_SC_PAGESIZE) as usize;
                let base = libc::mmap(
                    core::ptr::null_mut(),
                    3 * page,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_PRIVATE | libc::MAP_ANON,
                    -1,
                    0,
                );
                assert_ne!(base, libc::MAP_FAILED);
                assert_eq!(libc::mprotect(base, page, libc::PROT_NONE), 0);
                assert_eq!(
                    libc::mprotect(base.cast::<u8>().add(2 * page).cast(), page, libc::PROT_NONE),
                    0
                );
                Self {
                    base: base.cast(),
                    page,
                }
            }
        }
    }

    impl Drop for Guarded {
        fn drop(&mut self) {
            // SAFETY: the mapping of `new`.
            unsafe { libc::munmap(self.base.cast(), 3 * self.page) };
        }
    }

    /// Runs every intra prediction slot on every block size with a 257-pixel edge array placed directly after an
    /// inaccessible page and directly before one: with `topleft` at index 128 (intra prediction, all fourteen
    /// modes) and at index 32 (inter-intra prediction: DC, vertical, horizontal and smooth, blocks of at most
    /// 32x32), Z1 to Z3 over their angles with the flag sets 0, 0x400 and 0x600, the filter predictor over its five
    /// filters (blocks of at most 32x32, as the decoder uses it). A load outside the array faults; `slot(port, …)`
    /// calls the kernel under test (`port`) or the C kernel, and where `compare(mode)` holds the two blocks must
    /// be equal.
    ///
    /// `slot(port, mode, dst, stride, topleft, width, height, angle)`; `pixel(v)` makes a pixel of the bit depth
    /// from random bits.
    pub(crate) fn check<P: Copy + PartialEq + core::fmt::Debug>(
        pixel: impl Fn(u32) -> P,
        compare: impl Fn(usize) -> bool,
        slot: impl Fn(bool, usize, *mut P, isize, *const P, c_int, c_int, c_int),
    ) {
        let guarded = Guarded::new();
        let size = 257 * core::mem::size_of::<P>();
        let mut seed = 0x2545_f491u32;
        let mut random = move || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            seed
        };
        let mut port = vec![pixel(0); 64 * 64];
        let mut c = vec![pixel(0); 64 * 64];
        let stride = (64 * core::mem::size_of::<P>()) as isize;
        for at_end in [false, true] {
            // SAFETY: inside the readable page.
            let edge = unsafe {
                if at_end {
                    guarded.base.add(2 * guarded.page - size)
                } else {
                    guarded.base.add(guarded.page)
                }
            } as *mut P;
            for i in 0..257 {
                // SAFETY: the 257 pixels of the array.
                unsafe { edge.add(i).write(pixel(random())) };
            }
            for (index, modes) in [
                (128usize, &[0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13][..]),
                (32, &[DC_PRED, VERT_PRED, HOR_PRED, SMOOTH_PRED][..]),
            ] {
                // SAFETY: inside the array.
                let topleft = unsafe { edge.add(index) } as *const P;
                for &(w, h) in &SIZES {
                    for &mode in modes {
                        if (index == 32 || mode == FILTER_PRED) && (w > 32 || h > 32) {
                            continue;
                        }
                        let angles: Vec<c_int> = match mode {
                            Z1_PRED | Z2_PRED | Z3_PRED => {
                                let first = 3 + 90 * (mode - Z1_PRED) as c_int;
                                (0..15)
                                    .flat_map(|i| [0, 0x400, 0x600].map(|flags| (first + 6 * i) | flags))
                                    .collect()
                            }
                            FILTER_PRED => (0..5).collect(),
                            _ => vec![0],
                        };
                        for angle in angles {
                            port.fill(pixel(0));
                            c.fill(pixel(0));
                            slot(true, mode, port.as_mut_ptr(), stride, topleft, w, h, angle);
                            slot(false, mode, c.as_mut_ptr(), stride, topleft, w, h, angle);
                            assert!(
                                !compare(mode) || port == c,
                                "mode {mode} {w}x{h} angle {angle:#x}, topleft at {index}, array at the {} guard",
                                if at_end { "upper" } else { "lower" }
                            );
                        }
                    }
                }
            }
        }
    }
}
