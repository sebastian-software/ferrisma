// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2020, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/cdef16.S` (16 bpc) and the 16 bpc
//! instances of `cdef_tmpl.S` with NEON intrinsics (engine ADR-0044 D2,
//! libavif-rs ADR 0062).
//!
//! From `cdef16.S`: the padding entry points
//! `dav1d_cdef_padding{8,4}_16bpc_neon` (macro `padding_func_16`, with
//! `pad_top_bot_16` and `load_n_incr_16`); there is no fully edged variant
//! at 16 bpc. The template instances `filter 8, 16`, `filter 4, 16` and
//! `find_dir 16` are named functions here that instantiate the generic
//! macros of `cdef.rs` with `BPC` = 16.
//!
//! The correspondence with the assembly is that of `cdef.rs` (register
//! file `V` by register number, byte pointers, post-increments as explicit
//! offsets, a branch to another function as a call and `return`). The
//! `reg` of the padding macros is `2 * W` bytes (`q` for w 8, `d` for w 4).
//! No deviation.
//!
//! Safety (argued once for the family): as for 8 bpc (`cdef.rs`), with
//! 16-bit pixels and strides in bytes: the padding reads `src` rows `0..h`
//! (columns `0..w`, and `w..w+2` with `CDEF_HAVE_RIGHT`), `left[0..h]`
//! with `CDEF_HAVE_LEFT`, two rows at `top` and at `bottom` with the
//! respective flags (columns `-2..w+2` as the flags allow), and writes
//! rows `-2..h+2`, columns `-2..w+2` of the 16-bit `tmp` of the
//! `DEFINE_FILTER` wrapper; the filters read `tmp` within two rows and
//! columns of the block and write `dst` rows `0..h`, columns `0..w`;
//! `find_dir` reads 8 rows of 8 pixels and writes `*var`.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::{c_int, c_uint};

use super::cdef::{filter_func, find_dir, ld1_s, ldr, st1_x2, str_, P};
use super::util::{regs, V};

/// The macro `pad_top_bot_16 s1, s2, w, stride, reg, ret`: the two rows at
/// `s1` and `s2` into `tmp` at `x0` (`reg` = `2 * W` bytes). Returns `x0`
/// after the rows; with `RET` the caller returns.
#[inline(always)]
unsafe fn pad_top_bot_16<const W: usize, const STRIDE: usize, const RET: bool>(
    v: &mut V,
    mut x0: P,
    mut s1: *const u8,
    mut s2: *const u8,
    w7: c_uint,
) -> P {
    if w7 & 1 != 0 {
        // CDEF_HAVE_LEFT
        s1 = s1.sub(4);
        s2 = s2.sub(4);
        if w7 & 2 != 0 {
            // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
            v[0] = ldr(s1, 2 * W); // ldr reg0, [s1]
            v[1] = ldr(s1.add(2 * W), 8); // ldr d1, [s1, #2*w]
            v[2] = ldr(s2, 2 * W);
            v[3] = ldr(s2.add(2 * W), 8);
            str_(x0, 2 * W, v[0]);
            str_(x0.add(2 * W), 8, v[1]);
            x0 = x0.add(2 * STRIDE);
            str_(x0, 2 * W, v[2]);
            str_(x0.add(2 * W), 8, v[3]);
        } else {
            // 1: CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
            v[0] = ldr(s1, 2 * W);
            v[1] = ldr(s1.add(2 * W), 4); // ldr s1, [s1, #2*w]
            v[2] = ldr(s2, 2 * W);
            v[3] = ldr(s2.add(2 * W), 4);
            str_(x0, 2 * W, v[0]);
            str_(x0.add(2 * W), 4, v[1]);
            str_(x0.add(2 * W + 4), 4, v[31]);
            x0 = x0.add(2 * STRIDE);
            str_(x0, 2 * W, v[2]);
            str_(x0.add(2 * W), 4, v[3]);
            str_(x0.add(2 * W + 4), 4, v[31]);
        }
    } else if w7 & 2 != 0 {
        // 2: !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        v[0] = ldr(s1, 2 * W);
        v[1] = ldr(s1.add(2 * W), 4);
        v[2] = ldr(s2, 2 * W);
        v[3] = ldr(s2.add(2 * W), 4);
        str_(x0, 4, v[31]);
        str_(x0.add(4), 2 * W, v[0]); // stur
        str_(x0.add(4 + 2 * W), 4, v[1]);
        x0 = x0.add(2 * STRIDE);
        str_(x0, 4, v[31]);
        str_(x0.add(4), 2 * W, v[2]);
        str_(x0.add(4 + 2 * W), 4, v[3]);
    } else {
        // 1: !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        v[0] = ldr(s1, 2 * W);
        v[1] = ldr(s2, 2 * W);
        str_(x0, 4, v[31]);
        str_(x0.add(4), 2 * W, v[0]);
        str_(x0.add(4 + 2 * W), 4, v[31]);
        x0 = x0.add(2 * STRIDE);
        str_(x0, 4, v[31]);
        str_(x0.add(4), 2 * W, v[1]);
        str_(x0.add(4 + 2 * W), 4, v[31]);
    }
    if RET {
        return x0; // ret
    }
    x0.add(2 * STRIDE)
}

/// The macro `load_n_incr_16 dst, src, incr, w`: `ld1 {dst.4h}` (w 4) or
/// `ld1 {dst.8h}` (w 8), then `src += incr`.
#[inline(always)]
unsafe fn load_n_incr_16<const W: usize>(src: &mut *const u8, incr: isize) -> int16x8_t {
    let r = ldr(*src, 2 * W);
    *src = src.offset(incr);
    r
}

/// The macro `padding_func_16 w, stride, reg`: `dav1d_cdef_padding\w\()_16bpc_neon`.
#[inline(always)]
unsafe fn padding_func_16<const W: usize, const STRIDE: usize>(
    tmp: *mut u16,
    src: *const u16,
    src_stride: isize,
    left: *const [u16; 2],
    top: *const u16,
    bottom: *const u16,
    h: c_int,
    edges: c_uint,
) {
    let (mut x1, x2, mut x3, x4, x5, mut w6, w7) =
        (src as *const u8, src_stride, left as *const u8, top as *const u8, bottom as *const u8, h, edges);
    let mut v = regs();
    v[30] = vreinterpretq_s16_u16(vdupq_n_u16(0x8000)); // movi v30.8h, #0x80, lsl #8
    v[31] = v[30];
    let mut x0 = (tmp as P).sub(2 * (2 * STRIDE + 2));
    if w7 & 4 == 0 {
        // !CDEF_HAVE_TOP
        st1_x2(x0, v[30], v[31]);
        x0 = x0.add(32);
        if W == 8 {
            st1_x2(x0, v[30], v[31]);
            x0 = x0.add(32);
        }
    } else {
        // 1: CDEF_HAVE_TOP
        let x9 = x4.offset(x2);
        x0 = pad_top_bot_16::<W, STRIDE, false>(&mut v, x0, x4, x9, w7);
    }

    // 3: Middle section
    if w7 & 1 != 0 {
        // CDEF_HAVE_LEFT
        if w7 & 2 != 0 {
            // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
            loop {
                v[0] = ld1_s::<0>(v[0], x3);
                x3 = x3.add(4);
                v[2] = ldr(x1.add(2 * W), 4);
                v[1] = load_n_incr_16::<W>(&mut x1, x2);
                w6 -= 1;
                str_(x0, 4, v[0]);
                str_(x0.add(4), 2 * W, v[1]);
                str_(x0.add(4 + 2 * W), 4, v[2]);
                x0 = x0.add(2 * STRIDE);
                if w6 <= 0 {
                    break;
                }
            }
        } else {
            // 1: CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
            loop {
                v[0] = ld1_s::<0>(v[0], x3);
                x3 = x3.add(4);
                v[1] = load_n_incr_16::<W>(&mut x1, x2);
                w6 -= 1;
                str_(x0, 4, v[0]);
                str_(x0.add(4), 2 * W, v[1]);
                str_(x0.add(4 + 2 * W), 4, v[31]);
                x0 = x0.add(2 * STRIDE);
                if w6 <= 0 {
                    break;
                }
            }
        }
    } else if w7 & 2 != 0 {
        // 2: !CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
        loop {
            v[1] = ldr(x1.add(2 * W), 4);
            v[0] = load_n_incr_16::<W>(&mut x1, x2);
            w6 -= 1;
            str_(x0, 4, v[31]);
            str_(x0.add(4), 2 * W, v[0]);
            str_(x0.add(4 + 2 * W), 4, v[1]);
            x0 = x0.add(2 * STRIDE);
            if w6 <= 0 {
                break;
            }
        }
    } else {
        // 1: !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        loop {
            v[0] = load_n_incr_16::<W>(&mut x1, x2);
            w6 -= 1;
            str_(x0, 4, v[31]);
            str_(x0.add(4), 2 * W, v[0]);
            str_(x0.add(4 + 2 * W), 4, v[31]);
            x0 = x0.add(2 * STRIDE);
            if w6 <= 0 {
                break;
            }
        }
    }

    // 3:
    if w7 & 8 == 0 {
        // !CDEF_HAVE_BOTTOM
        st1_x2(x0, v[30], v[31]);
        x0 = x0.add(32);
        if W == 8 {
            st1_x2(x0, v[30], v[31]);
        }
        return;
    }
    // 1: CDEF_HAVE_BOTTOM
    let x9 = x5.offset(x2);
    pad_top_bot_16::<W, STRIDE, true>(&mut v, x0, x5, x9, w7);
}

/// `void dav1d_cdef_padding8_16bpc_neon(uint16_t *tmp, const pixel *src,
/// ptrdiff_t src_stride, const pixel (*left)[2], const pixel *const top,
/// const pixel *const bottom, int h, enum CdefEdgeFlags edges)`
pub unsafe extern "C" fn dav1d_cdef_padding8_16bpc_neon(
    tmp: *mut u16,
    src: *const u16,
    src_stride: isize,
    left: *const [u16; 2],
    top: *const u16,
    bottom: *const u16,
    h: c_int,
    edges: c_uint,
) {
    padding_func_16::<8, 16>(tmp, src, src_stride, left, top, bottom, h, edges)
}

/// `dav1d_cdef_padding4_16bpc_neon`
pub unsafe extern "C" fn dav1d_cdef_padding4_16bpc_neon(
    tmp: *mut u16,
    src: *const u16,
    src_stride: isize,
    left: *const [u16; 2],
    top: *const u16,
    bottom: *const u16,
    h: c_int,
    edges: c_uint,
) {
    padding_func_16::<4, 8>(tmp, src, src_stride, left, top, bottom, h, edges)
}

/// A 16 bpc instance of `filter_func`.
macro_rules! filter_instance {
    ($name:ident, $w:literal, $pri:literal, $sec:literal, $min:literal) => {
        #[doc = concat!("`", stringify!($name), "` (`filter_func`)")]
        unsafe fn $name(
            dst: *mut u16,
            dst_stride: isize,
            tmp: *const u16,
            pri_strength: c_int,
            sec_strength: c_int,
            dir: c_int,
            damping: c_int,
            h: c_int,
            edges: usize,
            bitdepth_max: c_int,
        ) {
            filter_func::<$w, 16, $pri, $sec, $min>(
                dst as P,
                dst_stride,
                tmp as *const u8,
                pri_strength,
                sec_strength,
                dir,
                damping,
                h,
                edges,
                bitdepth_max,
                None,
            )
        }
    };
}
// filter 8, 16
filter_instance!(cdef_filter8_pri_16bpc_neon, 8, true, false, false);
filter_instance!(cdef_filter8_sec_16bpc_neon, 8, false, true, false);
filter_instance!(cdef_filter8_pri_sec_16bpc_neon, 8, true, true, true);
// filter 4, 16
filter_instance!(cdef_filter4_pri_16bpc_neon, 4, true, false, false);
filter_instance!(cdef_filter4_sec_16bpc_neon, 4, false, true, false);
filter_instance!(cdef_filter4_pri_sec_16bpc_neon, 4, true, true, true);

/// `void dav1d_cdef_filter8_16bpc_neon(pixel *dst, ptrdiff_t dst_stride,
/// const uint16_t *tmp, int pri_strength, int sec_strength, int dir,
/// int damping, int h, size_t edges, int bitdepth_max)` (the function of
/// the macro `filter`)
pub unsafe extern "C" fn dav1d_cdef_filter8_16bpc_neon(
    dst: *mut u16,
    dst_stride: isize,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
    bitdepth_max: c_int,
) {
    let a = (dst, dst_stride, tmp, pri_strength, sec_strength, dir, damping, h, edges, bitdepth_max);
    if pri_strength == 0 {
        // only sec
        return cdef_filter8_sec_16bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9);
    }
    if sec_strength == 0 {
        // only pri
        return cdef_filter8_pri_16bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9);
    }
    // both pri and sec
    cdef_filter8_pri_sec_16bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9)
}

/// `dav1d_cdef_filter4_16bpc_neon`
pub unsafe extern "C" fn dav1d_cdef_filter4_16bpc_neon(
    dst: *mut u16,
    dst_stride: isize,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
    bitdepth_max: c_int,
) {
    let a = (dst, dst_stride, tmp, pri_strength, sec_strength, dir, damping, h, edges, bitdepth_max);
    if pri_strength == 0 {
        return cdef_filter4_sec_16bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9);
    }
    if sec_strength == 0 {
        return cdef_filter4_pri_16bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9);
    }
    cdef_filter4_pri_sec_16bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9)
}

/// `int dav1d_cdef_find_dir_16bpc_neon(const pixel *img, const ptrdiff_t
/// stride, unsigned *const var, int bitdepth_max)` (`find_dir 16`)
pub unsafe extern "C" fn dav1d_cdef_find_dir_16bpc_neon(
    img: *const u16,
    stride: isize,
    var: *mut c_uint,
    bitdepth_max: c_int,
) -> c_int {
    find_dir::<16>(img as *const u8, stride, var, bitdepth_max)
}
