// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2020, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/loopfilter16.S` (16 bpc) with
//! NEON intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The four entry points `dav1d_lpf_{v,h}_sb_{y,uv}_16bpc_neon` (the macro
//! `lpf_func`), the column filters `lpf_{v,h}_{4,6,8,16}_8_neon` and the
//! filter cores `lpf_8_wd{4,6,8,16}_neon` (the macro `loop_filter`); 8
//! pixels of 16 bits per vector, two 4-pixel units per step.
//!
//! The correspondence with the assembly is that of the 8 bpc file
//! (`loopfilter.rs`): the register file is `V` of `util.rs` (`int16x8_t`
//! by register number; unsigned instructions reinterpret it), `x14` is the
//! core's return value, the column filters' stores follow its value, and
//! the entry point's loop advances `x0` as the column filters leave it.
//! `w8` (bitdepth_max) and `w9` (bitdepth_min_8), which the entry point
//! sets for the core, are parameters of the core.
//!
//! Safety (argued once for the family): as for 8 bpc (`loopfilter.rs`),
//! with 16-bit pixels: the caller `lf_apply_tmpl` passes a plane pointer
//! and its stride in bytes, the masks, the levels and the LUT; the
//! assembly, and this rebuild, read and write the pixels the C
//! equivalent filters for each pair of 4-pixel units with a set bit, and
//! the level entries of the row (and the row above or the column to the
//! left).
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::c_int;

use super::loopfilter::Av1FilterLUT;
use super::util::{transpose_4x8h, transpose_8x8h, V};

type Dst = *mut u16;

// --- instructions (SAFETY: NEON value intrinsics; NEON is baseline) ------------

#[inline(always)]
fn uu(a: int16x8_t) -> uint16x8_t {
    unsafe { vreinterpretq_u16_s16(a) }
}
#[inline(always)]
fn ss(a: uint16x8_t) -> int16x8_t {
    unsafe { vreinterpretq_s16_u16(a) }
}
#[inline(always)]
fn uabd(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vabdq_u16(uu(a), uu(c))) }
}
#[inline(always)]
fn umax(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vmaxq_u16(uu(a), uu(c))) }
}
#[inline(always)]
fn uqadd(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vqaddq_u16(uu(a), uu(c))) }
}
#[inline(always)]
fn cmhs(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vcgeq_u16(uu(a), uu(c))) }
}
#[inline(always)]
fn cmhi(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vcgtq_u16(uu(a), uu(c))) }
}
#[inline(always)]
fn and(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vandq_s16(a, c) }
}
#[inline(always)]
fn bic(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vbicq_s16(a, c) }
}
#[inline(always)]
fn add(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vaddq_s16(a, c) }
}
#[inline(always)]
fn sub(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vsubq_s16(a, c) }
}
/// `urshr vd.8h, vn.8h, #N`
#[inline(always)]
fn urshr<const N: i32>(a: int16x8_t) -> int16x8_t {
    unsafe { ss(vrshrq_n_u16::<N>(uu(a))) }
}
/// `bit vd, vn, vm`: `vd = vm ? vn : vd` bitwise
#[inline(always)]
fn bit(d: int16x8_t, n: int16x8_t, m: int16x8_t) -> int16x8_t {
    unsafe { vbslq_s16(uu(m), n, d) }
}
/// `bif vd, vn, vm`: `vd = vm ? vd : vn` bitwise
#[inline(always)]
fn bif(d: int16x8_t, n: int16x8_t, m: int16x8_t) -> int16x8_t {
    unsafe { vbslq_s16(uu(m), d, n) }
}
/// `mov x16, vN.d[0] ; mov x17, vN.d[1] ; adds x16, x16, x17`: Z clear
#[inline(always)]
fn any(a: int16x8_t) -> bool {
    unsafe {
        let a = vreinterpretq_u64_s16(a);
        vgetq_lane_u64::<0>(a).wrapping_add(vgetq_lane_u64::<1>(a)) != 0
    }
}
#[inline(always)]
fn dup(x: u16) -> int16x8_t {
    unsafe { vdupq_n_s16(x as i16) }
}

// --- memory --------------------------------------------------------------------

/// Row `r` below `p` (`stride` in bytes).
#[inline(always)]
unsafe fn row(p: Dst, stride: isize, r: isize) -> Dst {
    (p as *mut u8).offset(r * stride) as Dst
}
/// `ld1 {vN.8h}, [p]`
#[inline(always)]
unsafe fn ld8(p: Dst) -> int16x8_t {
    vld1q_s16(p as *const i16)
}
/// `st1 {vN.8h}, [p]`
#[inline(always)]
unsafe fn st8(p: Dst, a: int16x8_t) {
    vst1q_s16(p as *mut i16, a)
}
/// `ld1 {vN.d}[L], [p]`
#[inline(always)]
unsafe fn ld_d<const L: i32>(a: int16x8_t, p: Dst) -> int16x8_t {
    let x = core::ptr::read_unaligned(p.cast::<u64>());
    vreinterpretq_s16_u64(vsetq_lane_u64::<L>(x, vreinterpretq_u64_s16(a)))
}
/// `st1 {vN.d}[L], [p]`
#[inline(always)]
unsafe fn st_d<const L: i32>(p: Dst, a: int16x8_t) {
    core::ptr::write_unaligned(p.cast::<u64>(), vgetq_lane_u64::<L>(vreinterpretq_u64_s16(a)))
}

// --- loop_filter wd: lpf_8_wd{4,6,8,16}_neon ---------------------------------------

/// The macro `loop_filter wd` (`lpf_8_wd\wd\()_neon`); registers as in
/// `loopfilter.rs`, `w8` bitdepth_max, `w9` bitdepth_min_8. Returns `x14`.
#[inline(always)]
fn lpf_8_wd<const WD: u32>(v: &mut V, w8: u32, w9: u32) -> u64 {
    // SAFETY: NEON value intrinsics only.
    unsafe {
        v[0] = uabd(v[22], v[23]); // abs(p1 - p0)
        v[1] = uabd(v[25], v[24]); // abs(q1 - q0)
        v[2] = uabd(v[23], v[24]); // abs(p0 - q0)
        v[3] = uabd(v[22], v[25]); // abs(p1 - q1)
        if WD >= 6 {
            v[4] = uabd(v[21], v[22]); // abs(p2 - p1)
            v[5] = uabd(v[26], v[25]); // abs(q2 - q1)
        }
        if WD >= 8 {
            v[6] = uabd(v[20], v[21]); // abs(p3 - p2)
            v[7] = uabd(v[27], v[26]); // abs(q3 - q3)
        }
        if WD >= 6 {
            v[4] = umax(v[4], v[5]);
        }
        v[2] = uqadd(v[2], v[2]); // abs(p0 - q0) * 2
        if WD >= 8 {
            v[6] = umax(v[6], v[7]);
        }
        v[3] = ss(vshrq_n_u16::<1>(uu(v[3])));
        if WD >= 8 {
            v[4] = umax(v[4], v[6]);
        }
        if WD >= 6 {
            v[4] = and(v[4], v[14]);
        }
        v[0] = umax(v[0], v[1]); // max(abs(p1 - p0), abs(q1 - q0))
        v[2] = uqadd(v[2], v[3]); // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1
        if WD >= 6 {
            v[4] = umax(v[0], v[4]);
            v[1] = cmhs(v[11], v[4]); // max(abs(p1 - p0), abs(q1 - q0), abs(), abs(), ...) <= I
        } else {
            v[1] = cmhs(v[11], v[0]); // max(abs(p1 - p0), abs(q1 - q0)) <= I
        }
        v[2] = cmhs(v[10], v[2]); // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1 <= E
        v[1] = and(v[1], v[2]); // fm
        v[1] = and(v[1], v[13]); // fm && wd >= 4
        if WD >= 6 {
            v[14] = and(v[14], v[1]); // fm && wd > 4
        }
        if WD >= 16 {
            v[15] = and(v[15], v[1]); // fm && wd == 16
        }

        if !any(v[1]) {
            return 1 << 0; // if (!fm || wd < 4) return;
        }
        // 9:
        let mut skip_wd4 = false;
        if WD >= 6 {
            v[10] = dup(1);
            v[2] = uabd(v[21], v[23]); // abs(p2 - p0)
            v[3] = uabd(v[22], v[23]); // abs(p1 - p0)
            v[4] = uabd(v[25], v[24]); // abs(q1 - q0)
            v[5] = uabd(v[26], v[24]); // abs(q2 - q0)
            v[9] = dup(w9 as u16); // bitdepth_min_8
            if WD >= 8 {
                v[6] = uabd(v[20], v[23]); // abs(p3 - p0)
                v[7] = uabd(v[27], v[24]); // abs(q3 - q0)
            }
            v[2] = umax(v[2], v[3]);
            v[4] = umax(v[4], v[5]);
            if WD >= 8 {
                v[6] = umax(v[6], v[7]);
            }
            v[2] = umax(v[2], v[4]);
            v[10] = ss(vshlq_u16(uu(v[10]), v[9])); // F = 1 << bitdepth_min_8
            if WD >= 8 {
                v[2] = umax(v[2], v[6]);
            }

            if WD == 16 {
                v[3] = uabd(v[17], v[23]); // abs(p6 - p0)
                v[4] = uabd(v[18], v[23]); // abs(p5 - p0)
                v[5] = uabd(v[19], v[23]); // abs(p4 - p0)
            }
            v[2] = cmhs(v[10], v[2]); // flat8in
            if WD == 16 {
                v[6] = uabd(v[28], v[24]); // abs(q4 - q0)
                v[7] = uabd(v[29], v[24]); // abs(q5 - q0)
                v[8] = uabd(v[30], v[24]); // abs(q6 - q0)
            }
            v[14] = and(v[2], v[14]); // flat8in && fm && wd > 4
            v[1] = bic(v[1], v[14]); // fm && wd >= 4 && !flat8in
            if WD == 16 {
                v[3] = umax(v[3], v[4]);
                v[5] = umax(v[5], v[6]);
            }
            let nz = any(v[1]);
            if WD == 16 {
                v[7] = umax(v[7], v[8]);
                v[3] = umax(v[3], v[5]);
                v[3] = umax(v[3], v[7]);
                v[3] = cmhs(v[10], v[3]); // flat8out
            }
            if WD == 16 {
                v[15] = and(v[15], v[3]); // flat8out && fm && wd == 16
                v[15] = and(v[15], v[14]); // flat8out && flat8in && fm && wd == 16
                v[14] = bic(v[14], v[15]); // flat8in && fm && wd >= 4 && !flat8out
            }
            skip_wd4 = !nz; // b.eq 1f  // skip wd == 4 case
        }
        if !skip_wd4 {
            v[3] = dup(w8 as u16); // bitdepth_max
            v[2] = sub(v[22], v[25]); // p1 - q1
            v[3] = ss(vshrq_n_u16::<1>(uu(v[3]))); // 128 << bitdepth_min_8 - 1
            v[0] = cmhi(v[0], v[12]); // hev
            v[9] = vmvnq_s16(v[3]); // - 128 * (1 << bitdepth_min_8)
            v[2] = vminq_s16(v[2], v[3]); // iclip_diff(p1 - q1)
            v[2] = vmaxq_s16(v[2], v[9]); // iclip_diff(p1 - q1)
            v[4] = and(v[2], v[0]); // if (hev) iclip_diff(p1 - q1)
            v[2] = sub(v[24], v[23]);
            v[5] = dup(3);
            v[0] = bic(v[1], v[0]); // (fm && wd >= 4 && !hev)
            v[2] = vmulq_s16(v[2], v[5]);
            v[6] = dup(4);
            v[2] = add(v[2], v[4]);
            v[2] = vminq_s16(v[2], v[3]); // f = iclip_diff()
            v[2] = vmaxq_s16(v[2], v[9]); // f = iclip_diff()
            v[4] = vqaddq_s16(v[6], v[2]); // f + 4
            v[5] = vqaddq_s16(v[5], v[2]); // f + 3
            v[4] = vminq_s16(v[4], v[3]); // imin(f + 4, 128 << bitdepth_min_8 - 1)
            v[5] = vminq_s16(v[5], v[3]); // imin(f + 3, 128 << bitdepth_min_8 - 1)
            v[4] = vshrq_n_s16::<3>(v[4]); // f1
            v[5] = vshrq_n_s16::<3>(v[5]); // f2
            v[9] = dup(0);
            v[3] = dup(w8 as u16); // bitdepth_max
            v[2] = vqaddq_s16(v[23], v[5]); // p0 + f2
            v[6] = vqsubq_s16(v[24], v[4]); // q0 - f1
            v[4] = vrshrq_n_s16::<1>(v[4]); // (f1 + 1) >> 1
            v[2] = vminq_s16(v[2], v[3]); // out p0 = iclip_pixel()
            v[6] = vminq_s16(v[6], v[3]); // out q0 = iclip_pixel()
            v[2] = vmaxq_s16(v[2], v[9]); // out p0 = iclip_pixel()
            v[6] = vmaxq_s16(v[6], v[9]); // out q0 = iclip_pixel()
            v[23] = bit(v[23], v[2], v[1]); // if (fm && wd >= 4)
            v[24] = bit(v[24], v[6], v[1]); // if (fm && wd >= 4)
            v[2] = vqaddq_s16(v[22], v[4]); // p1 + f
            v[6] = vqsubq_s16(v[25], v[4]); // q1 - f
            v[2] = vminq_s16(v[2], v[3]); // out p1 = iclip_pixel()
            v[6] = vminq_s16(v[6], v[3]); // out q1 = iclip_pixel()
            v[2] = vmaxq_s16(v[2], v[9]); // out p1 = iclip_pixel()
            v[6] = vmaxq_s16(v[6], v[9]); // out q1 = iclip_pixel()
            v[22] = bit(v[22], v[2], v[0]); // if (fm && wd >= 4 && !hev)
            v[25] = bit(v[25], v[6], v[0]); // if (fm && wd >= 4 && !hev)
        }
        // 1:

        if WD == 6 {
            if any(v[14]) {
                v[0] = add(v[21], v[21]); // p2 * 2
                v[2] = add(v[21], v[22]); // p2 + p1
                v[4] = add(v[22], v[23]); // p1 + p0
                v[6] = add(v[23], v[24]); // p0 + q0
                v[8] = add(v[0], v[2]);
                v[10] = add(v[4], v[6]);
                v[12] = add(v[24], v[25]); // q0 + q1
                v[8] = add(v[8], v[10]);
                v[12] = sub(v[12], v[0]);
                v[10] = add(v[25], v[26]); // q1 + q2
                v[0] = urshr::<3>(v[8]); // out p1

                v[8] = add(v[8], v[12]);
                v[10] = sub(v[10], v[2]);
                v[12] = add(v[26], v[26]); // q2 + q2
                v[1] = urshr::<3>(v[8]); // out p0

                v[8] = add(v[8], v[10]);
                v[12] = sub(v[12], v[4]);
                v[2] = urshr::<3>(v[8]); // out q0

                v[22] = bit(v[22], v[0], v[14]); // p1 if (flat8in)
                v[8] = add(v[8], v[12]);
                v[23] = bit(v[23], v[1], v[14]); // p0 if (flat8in)
                v[3] = urshr::<3>(v[8]); // out q1
                v[24] = bit(v[24], v[2], v[14]); // q0 if (flat8in)
                v[25] = bit(v[25], v[3], v[14]); // q1 if (flat8in)
            }
        } else if WD >= 8 {
            if !any(v[14]) {
                if WD == 8 {
                    return 1 << 4; // b.eq 8f  // skip if there's no flat8in
                }
            } else {
                v[0] = add(v[20], v[21]); // p3 + p2
                v[2] = add(v[22], v[25]); // p1 + q1
                v[4] = add(v[20], v[22]); // p3 + p1
                v[6] = add(v[23], v[26]); // p0 + q2
                v[8] = add(v[0], v[0]); // 2 * (p3 + p2)
                v[9] = add(v[23], v[24]); // p0 + q0
                v[8] = add(v[8], v[4]); // + p3 + p1
                v[2] = sub(v[2], v[0]); // p1 + q1 - p3 - p2
                v[8] = add(v[8], v[9]); // + p0 + q0
                v[6] = sub(v[6], v[4]); // p0 + q2 - p3 - p1
                v[10] = urshr::<3>(v[8]); // out p2

                v[8] = add(v[8], v[2]);
                v[0] = add(v[20], v[23]); // p3 + p0
                v[2] = add(v[24], v[27]); // q0 + q3
                v[11] = urshr::<3>(v[8]); // out p1

                v[8] = add(v[8], v[6]);
                v[2] = sub(v[2], v[0]); // q0 + q3 - p3 - p0
                v[4] = add(v[21], v[24]); // p2 + q0
                v[6] = add(v[25], v[27]); // q1 + q3
                v[12] = urshr::<3>(v[8]); // out p0

                v[8] = add(v[8], v[2]);
                v[6] = sub(v[6], v[4]); // q1 + q3 - p2 - q0
                v[0] = add(v[22], v[25]); // p1 + q1
                v[2] = add(v[26], v[27]); // q2 + q3
                v[13] = urshr::<3>(v[8]); // out q0

                v[8] = add(v[8], v[6]);
                v[2] = sub(v[2], v[0]); // q2 + q3 - p1 - q1
                v[0] = urshr::<3>(v[8]); // out q1

                v[8] = add(v[8], v[2]);

                v[21] = bit(v[21], v[10], v[14]);
                v[22] = bit(v[22], v[11], v[14]);
                v[23] = bit(v[23], v[12], v[14]);
                v[1] = urshr::<3>(v[8]); // out q2
                v[24] = bit(v[24], v[13], v[14]);
                v[25] = bit(v[25], v[0], v[14]);
                v[26] = bit(v[26], v[1], v[14]);
            }
        }
        // 2:
        if WD == 16 {
            if !any(v[15]) {
                if !any(v[14]) {
                    return 1 << 4; // 8: just write the inner 4 pixels
                }
                return 1 << 6; // 7: write the inner 6 pixels
            }
            // 1:
            v[2] = add(v[17], v[17]); // p6 + p6
            v[4] = add(v[17], v[18]); // p6 + p5
            v[6] = add(v[17], v[19]); // p6 + p4
            v[8] = add(v[17], v[20]); // p6 + p3
            v[12] = add(v[2], v[4]);
            v[10] = add(v[6], v[8]);
            v[6] = add(v[17], v[21]); // p6 + p2
            v[12] = add(v[12], v[10]);
            v[8] = add(v[17], v[22]); // p6 + p1
            v[10] = add(v[18], v[23]); // p5 + p0
            v[6] = add(v[6], v[8]);
            v[8] = add(v[19], v[24]); // p4 + q0
            v[12] = add(v[12], v[6]);
            v[10] = add(v[10], v[8]);
            v[6] = add(v[20], v[25]); // p3 + q1
            v[12] = add(v[12], v[10]);
            v[6] = sub(v[6], v[2]);
            v[2] = add(v[21], v[26]); // p2 + q2
            v[0] = urshr::<4>(v[12]); // out p5
            v[12] = add(v[12], v[6]); // - (p6 + p6) + (p3 + q1)
            v[2] = sub(v[2], v[4]);
            v[4] = add(v[22], v[27]); // p1 + q3
            v[6] = add(v[17], v[19]); // p6 + p4
            v[1] = urshr::<4>(v[12]); // out p4
            v[12] = add(v[12], v[2]); // - (p6 + p5) + (p2 + q2)
            v[4] = sub(v[4], v[6]);
            v[6] = add(v[23], v[28]); // p0 + q4
            v[8] = add(v[17], v[20]); // p6 + p3
            v[2] = urshr::<4>(v[12]); // out p3
            v[12] = add(v[12], v[4]); // - (p6 + p4) + (p1 + q3)
            v[6] = sub(v[6], v[8]);
            v[8] = add(v[24], v[29]); // q0 + q5
            v[4] = add(v[17], v[21]); // p6 + p2
            v[3] = urshr::<4>(v[12]); // out p2
            v[12] = add(v[12], v[6]); // - (p6 + p3) + (p0 + q4)
            v[8] = sub(v[8], v[4]);
            v[6] = add(v[25], v[30]); // q1 + q6
            v[10] = add(v[17], v[22]); // p6 + p1
            v[4] = urshr::<4>(v[12]); // out p1
            v[12] = add(v[12], v[8]); // - (p6 + p2) + (q0 + q5)
            v[6] = sub(v[6], v[10]);
            v[8] = add(v[26], v[30]); // q2 + q6
            v[0] = bif(v[0], v[18], v[15]); // out p5
            v[10] = add(v[18], v[23]); // p5 + p0
            v[5] = urshr::<4>(v[12]); // out p0
            v[12] = add(v[12], v[6]); // - (p6 + p1) + (q1 + q6)
            v[8] = sub(v[8], v[10]);
            v[10] = add(v[27], v[30]); // q3 + q6
            v[1] = bif(v[1], v[19], v[15]); // out p4
            v[18] = add(v[19], v[24]); // p4 + q0
            v[6] = urshr::<4>(v[12]); // out q0
            v[12] = add(v[12], v[8]); // - (p5 + p0) + (q2 + q6)
            v[10] = sub(v[10], v[18]);
            v[8] = add(v[28], v[30]); // q4 + q6
            v[2] = bif(v[2], v[20], v[15]); // out p3
            v[18] = add(v[20], v[25]); // p3 + q1
            v[7] = urshr::<4>(v[12]); // out q1
            v[12] = add(v[12], v[10]); // - (p4 + q0) + (q3 + q6)
            v[18] = sub(v[8], v[18]);
            v[10] = add(v[29], v[30]); // q5 + q6
            v[3] = bif(v[3], v[21], v[15]); // out p2
            v[20] = add(v[21], v[26]); // p2 + q2
            v[8] = urshr::<4>(v[12]); // out q2
            v[12] = add(v[12], v[18]); // - (p3 + q1) + (q4 + q6)
            v[10] = sub(v[10], v[20]);
            v[18] = add(v[30], v[30]); // q6 + q6
            v[4] = bif(v[4], v[22], v[15]); // out p1
            v[20] = add(v[22], v[27]); // p1 + q3
            v[9] = urshr::<4>(v[12]); // out q3
            v[12] = add(v[12], v[10]); // - (p2 + q2) + (q5 + q6)
            v[18] = sub(v[18], v[20]);
            v[5] = bif(v[5], v[23], v[15]); // out p0
            v[10] = urshr::<4>(v[12]); // out q4
            v[12] = add(v[12], v[18]); // - (p1 + q3) + (q6 + q6)
            v[11] = urshr::<4>(v[12]); // out q5
            v[6] = bif(v[6], v[24], v[15]); // out q0
            v[7] = bif(v[7], v[25], v[15]); // out q1
            v[8] = bif(v[8], v[26], v[15]); // out q2
            v[9] = bif(v[9], v[27], v[15]); // out q3
            v[10] = bif(v[10], v[28], v[15]); // out q4
            v[11] = bif(v[11], v[29], v[15]); // out q5
        }
        0
    }
}

// --- the column filters lpf_{v,h}_{4,6,8,16}_8_neon -----------------------------------

#[inline(always)]
unsafe fn store_v4(v: &V, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -2);
    st8(row(x16, x1, 0), v[22]); // p1
    st8(row(x0, x1, 0), v[24]); // q0
    st8(row(x16, x1, 1), v[23]); // p0
    st8(row(x0, x1, 1), v[25]); // q1
}

#[inline(always)]
unsafe fn store_v6(v: &V, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -3);
    st8(row(x16, x1, 0), v[21]); // p2
    st8(row(x0, x1, 0), v[24]); // q0
    st8(row(x16, x1, 1), v[22]); // p1
    st8(row(x0, x1, 1), v[25]); // q1
    st8(row(x16, x1, 2), v[23]); // p0
    st8(row(x0, x1, 2), v[26]); // q2
}

/// Transpose `v22`–`v25` back and store the 4 inner columns of 8 rows.
#[inline(always)]
unsafe fn store_h4(v: &mut V, x0: Dst, x1: isize) {
    let x16 = x0.offset(-2);
    transpose_4x8h(v, [22, 23, 24, 25], [26, 27, 28, 29]);
    let x0 = row(x16, x1, 4);
    for (i, r) in [22usize, 23, 24, 25].into_iter().enumerate() {
        st_d::<0>(row(x16, x1, i as isize), v[r]);
        st_d::<1>(row(x0, x1, i as isize), v[r]);
    }
}

/// Transpose `v20`–`v27` back and store 8 columns of 8 rows.
#[inline(always)]
unsafe fn store_h8(v: &mut V, x0: Dst, x1: isize) {
    let x16 = x0.offset(-4);
    transpose_8x8h(v, [20, 21, 22, 23, 24, 25, 26, 27], 28, 29);
    let x0 = row(x16, x1, 4);
    for (i, (p, q)) in [(20usize, 24usize), (21, 25), (22, 26), (23, 27)].into_iter().enumerate() {
        st8(row(x16, x1, i as isize), v[p]);
        st8(row(x0, x1, i as isize), v[q]);
    }
}

/// Load 8 columns (`x0 - 4` …) of 8 rows into `v20`–`v27` and transpose.
#[inline(always)]
unsafe fn load_h8(v: &mut V, x0: Dst, x1: isize) {
    let x16 = x0.offset(-4);
    let x0 = row(x16, x1, 4);
    for (i, (p, q)) in [(20usize, 24usize), (21, 25), (22, 26), (23, 27)].into_iter().enumerate() {
        v[p] = ld8(row(x16, x1, i as isize));
        v[q] = ld8(row(x0, x1, i as isize));
    }
    transpose_8x8h(v, [20, 21, 22, 23, 24, 25, 26, 27], 28, 29);
}

/// `lpf_v_4_8_neon`
#[inline(always)]
unsafe fn lpf_v_4_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    let x16 = row(x0, x1, -2);
    v[22] = ld8(row(x16, x1, 0)); // p1
    v[24] = ld8(row(x0, x1, 0)); // q0
    v[23] = ld8(row(x16, x1, 1)); // p0
    v[25] = ld8(row(x0, x1, 1)); // q1

    if lpf_8_wd::<4>(v, w8, w9) != 0 {
        return;
    }

    store_v4(v, x0, x1);
}

/// `lpf_h_4_8_neon`
#[inline(always)]
unsafe fn lpf_h_4_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    let x16 = x0.offset(-2);
    let x0b = row(x16, x1, 4);
    for (i, r) in [22usize, 23, 24, 25].into_iter().enumerate() {
        v[r] = ld_d::<0>(v[r], row(x16, x1, i as isize));
        v[r] = ld_d::<1>(v[r], row(x0b, x1, i as isize));
    }

    transpose_4x8h(v, [22, 23, 24, 25], [26, 27, 28, 29]);

    if lpf_8_wd::<4>(v, w8, w9) != 0 {
        return;
    }

    store_h4(v, x0, x1);
}

/// `lpf_v_6_8_neon`
#[inline(always)]
unsafe fn lpf_v_6_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    let x16 = row(x0, x1, -3);
    v[21] = ld8(row(x16, x1, 0)); // p2
    v[24] = ld8(row(x0, x1, 0)); // q0
    v[22] = ld8(row(x16, x1, 1)); // p1
    v[25] = ld8(row(x0, x1, 1)); // q1
    v[23] = ld8(row(x16, x1, 2)); // p0
    v[26] = ld8(row(x0, x1, 2)); // q2

    if lpf_8_wd::<6>(v, w8, w9) != 0 {
        return;
    }

    store_v4(v, x0, x1);
}

/// `lpf_h_6_8_neon`
#[inline(always)]
unsafe fn lpf_h_6_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    load_h8(v, x0, x1);

    if lpf_8_wd::<6>(v, w8, w9) != 0 {
        return;
    }

    store_h4(v, x0, x1);
}

/// `lpf_v_8_8_neon`
#[inline(always)]
unsafe fn lpf_v_8_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    let x16 = row(x0, x1, -4);
    v[20] = ld8(row(x16, x1, 0)); // p3
    v[24] = ld8(row(x0, x1, 0)); // q0
    v[21] = ld8(row(x16, x1, 1)); // p2
    v[25] = ld8(row(x0, x1, 1)); // q1
    v[22] = ld8(row(x16, x1, 2)); // p1
    v[26] = ld8(row(x0, x1, 2)); // q2
    v[23] = ld8(row(x16, x1, 3)); // p0
    v[27] = ld8(row(x0, x1, 3)); // q3

    match lpf_8_wd::<8>(v, w8, w9) {
        0 => store_v6(v, x0, x1),
        x14 if x14 & (1 << 4) != 0 => store_v4(v, x0, x1), // 8:
        _ => {}
    }
}

/// `lpf_h_8_8_neon`
#[inline(always)]
unsafe fn lpf_h_8_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    load_h8(v, x0, x1);

    match lpf_8_wd::<8>(v, w8, w9) {
        0 => store_h8(v, x0, x1),
        x14 if x14 & (1 << 4) != 0 => store_h4(v, x0, x1), // 8:
        _ => {}
    }
}

/// `lpf_v_16_8_neon`
#[inline(always)]
unsafe fn lpf_v_16_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    let x16 = row(x0, x1, -7);
    for (i, (p, q)) in [(17usize, 24usize), (18, 25), (19, 26), (20, 27), (21, 28), (22, 29), (23, 30)]
        .into_iter()
        .enumerate()
    {
        v[p] = ld8(row(x16, x1, i as isize)); // p6 … p0
        v[q] = ld8(row(x0, x1, i as isize)); // q0 … q6
    }

    match lpf_8_wd::<16>(v, w8, w9) {
        0 => {
            let x16 = row(x0, x1, -6);
            for (i, (p, q)) in [(0usize, 6usize), (1, 7), (2, 8), (3, 9), (4, 10), (5, 11)].into_iter().enumerate() {
                st8(row(x16, x1, i as isize), v[p]); // p5 … p0
                st8(row(x0, x1, i as isize), v[q]); // q0 … q5
            }
        }
        x14 if x14 & (1 << 6) != 0 => store_v6(v, x0, x1), // 7:
        x14 if x14 & (1 << 4) != 0 => store_v4(v, x0, x1), // 8:
        _ => {}
    }
}

/// `lpf_h_16_8_neon`
#[inline(always)]
unsafe fn lpf_h_16_8(v: &mut V, x0: Dst, x1: isize, w8: u32, w9: u32) {
    let x16 = x0.offset(-8);
    for (i, (p, q)) in [(16usize, 24usize), (17, 25), (18, 26), (19, 27), (20, 28), (21, 29), (22, 30), (23, 31)]
        .into_iter()
        .enumerate()
    {
        v[p] = ld8(row(x16, x1, i as isize));
        v[q] = ld8(row(x0, x1, i as isize));
    }

    transpose_8x8h(v, [16, 17, 18, 19, 20, 21, 22, 23], 0, 1);
    transpose_8x8h(v, [24, 25, 26, 27, 28, 29, 30, 31], 0, 1);

    match lpf_8_wd::<16>(v, w8, w9) {
        0 => {
            let x16 = x0.offset(-8);

            transpose_8x8h(v, [16, 17, 0, 1, 2, 3, 4, 5], 18, 19);
            transpose_8x8h(v, [6, 7, 8, 9, 10, 11, 30, 31], 18, 19);

            for (i, (p, q)) in [(16usize, 6usize), (17, 7), (0, 8), (1, 9), (2, 10), (3, 11), (4, 30), (5, 31)]
                .into_iter()
                .enumerate()
            {
                st8(row(x16, x1, i as isize), v[p]);
                st8(row(x0, x1, i as isize), v[q]);
            }
        }
        x14 if x14 & (1 << 6) != 0 => store_h8(v, x0, x1), // 7:
        x14 if x14 & (1 << 4) != 0 => store_h4(v, x0, x1), // 8:
        _ => {}
    }
}

// --- lpf_func dir, type ---------------------------------------------------------------

/// `word_12`
static WORD_12: [u32; 2] = [1, 2];

/// The macro `lpf_func dir, type` at 16 bpc.
#[inline(always)]
unsafe fn lpf_func<const VD: bool, const Y: bool>(
    dst: Dst,
    stride: isize,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: isize,
    lut: *const Av1FilterLUT,
    bitdepth_max: c_int,
) {
    let mut v: V = [vdupq_n_s16(0); 32];
    let mut x0 = dst;
    let x1 = stride;
    // mov w8, w7  // bitdepth_max ; clz w9, w8 ; mov w10, #24 ; sub w9, w10, w9
    let w8 = bitdepth_max as u32;
    let w9 = 24u32.wrapping_sub(w8.leading_zeros()); // bitdepth_min_8
    // ldp w6, w7, [x2]  // vmask[0], vmask[1]
    let mut w6 = core::ptr::read(vmask);
    let mut w7 = core::ptr::read(vmask.add(1));
    let mut w2 = if Y { core::ptr::read(vmask.add(2)) } else { 0 }; // vmask[2]
    // add x5, x5, #128  // Move to sharp part of lut
    let x5 = (lut as *const u8).add(128);
    if Y {
        w7 |= w2; // vmask[1] |= vmask[2]
    }
    let mut x3 = l as *const u8;
    let mut x4: *const u8 = core::ptr::null();
    let x4s = b4_stride * 4;
    if VD {
        x4 = x3.offset(-x4s);
    } else {
        x3 = x3.offset(-4);
    }
    w6 |= w7; // vmask[0] |= vmask[1]

    // v0/v1/v2… of the level computation are 64-bit (`.8b`, `.2s`).
    let d8 = |x: uint8x8_t| vreinterpretq_s16_u8(vcombine_u8(x, vdup_n_u8(0)));
    let l8 = |x: int16x8_t| vget_low_u8(vreinterpretq_u8_s16(x));
    let l32 = |x: int16x8_t| vget_low_u32(vreinterpretq_u32_s16(x));
    let d32 = |x: uint32x2_t| vreinterpretq_s16_u32(vcombine_u32(x, vdup_n_u32(0)));
    loop {
        // 1:
        let skip = w6 & 0x03 == 0;
        if VD {
            // ld1 {v0.8b}, [x4], #8 ; ld1 {v1.8b}, [x3], #8
            v[0] = d8(vld1_u8(x4));
            x4 = x4.add(8);
            v[1] = d8(vld1_u8(x3));
            x3 = x3.add(8);
        } else {
            // ld2 {v0.s, v1.s}[i], [x3], x4
            let mut a = vreinterpretq_u32_s16(v[0]);
            let mut c = vreinterpretq_u32_s16(v[1]);
            let p = core::ptr::read_unaligned(x3.cast::<[u32; 2]>());
            a = vsetq_lane_u32::<0>(p[0], a);
            c = vsetq_lane_u32::<0>(p[1], c);
            x3 = x3.offset(x4s);
            let p = core::ptr::read_unaligned(x3.cast::<[u32; 2]>());
            a = vsetq_lane_u32::<1>(p[0], a);
            c = vsetq_lane_u32::<1>(p[1], c);
            x3 = x3.offset(x4s);
            v[0] = vreinterpretq_s16_u32(a);
            v[1] = vreinterpretq_s16_u32(c);
        }
        // b.eq 7f  // if (!(vm & bits)) continue;
        if !skip {
            v[5] = d8(vld1_dup_u8(x5)); // sharp[0]
            v[2] = d32(vdup_n_u32(0xff));
            v[13] = d32(vdup_n_u32(w6)); // vmask[0]
            v[31] = dup(w9 as u16); // bitdepth_min_8

            v[0] = d8(vand_u8(l8(v[0]), l8(v[2]))); // Keep only lowest byte in each 32 bit word
            v[1] = d8(vand_u8(l8(v[1]), l8(v[2])));
            v[3] = d8(vtst_u8(l8(v[1]), l8(v[2]))); // Check for nonzero values in l[0][0]
            v[4] = d8(vdup_n_u8(1));
            v[6] = d8(vld1_dup_u8(x5.add(8))); // sharp[1]
            v[1] = d8(vbsl_u8(l8(v[3]), l8(v[1]), l8(v[0]))); // if (!l[0][0]) L = l[offset][0]
            v[2] = d32(vtst_u32(l32(v[1]), l32(v[2]))); // L != 0
            v[1] = d32(vmul_u32(l32(v[1]), l32(v[4]))); // L
            if Y {
                v[15] = d32(vdup_n_u32(w2)); // vmask[2]
            }
            v[14] = d32(vdup_n_u32(w7)); // vmask[1]
            // mov x16, v2.d[0] ; cmp x16, #0 ; b.eq 7f  // if (!L) continue;
            if vgetq_lane_u64::<0>(vreinterpretq_u64_s16(v[2])) != 0 {
                v[5] = d8(vreinterpret_u8_s8(vneg_s8(vreinterpret_s8_u8(l8(v[5]))))); // -sharp[0]
                v[16] = d32(vld1_u32(WORD_12.as_ptr()));
                v[12] = d8(vshr_n_u8::<4>(l8(v[1]))); // H
                v[3] = d8(vreinterpret_u8_s8(vshl_s8(vreinterpret_s8_u8(l8(v[1])), vreinterpret_s8_u8(l8(v[5]))))); // L >> sharp[0]
                if Y {
                    v[15] = d32(vtst_u32(l32(v[15]), l32(v[16]))); // if (vmask[2] & bits)
                }
                v[7] = d8(vdup_n_u8(2));
                v[3] = d8(vmin_u8(l8(v[3]), l8(v[6]))); // imin(L >> sharp[0], sharp[1])
                v[0] = d8(vadd_u8(l8(v[1]), l8(v[7]))); // L + 2
                v[11] = d8(vmax_u8(l8(v[3]), l8(v[4]))); // imax(imin(), 1) = limit = I
                v[0] = d8(vadd_u8(l8(v[0]), l8(v[0]))); // 2*(L + 2)
                v[14] = d32(vtst_u32(l32(v[14]), l32(v[16]))); // if (vmask[1] & bits)
                v[12] = vreinterpretq_s16_u16(vmovl_u8(l8(v[12])));
                v[10] = d8(vadd_u8(l8(v[0]), l8(v[11]))); // 2*(L + 2) + limit = E
                v[13] = d32(vtst_u32(l32(v[13]), l32(v[16]))); // if (vmask[0] & bits)
                v[11] = vreinterpretq_s16_u16(vmovl_u8(l8(v[11])));
                v[10] = vreinterpretq_s16_u16(vmovl_u8(l8(v[10])));
                v[13] = d8(vand_u8(l8(v[13]), l8(v[2]))); // vmask[0] &= L != 0
                v[14] = vmovl_s8(vreinterpret_s8_u8(l8(v[14])));
                v[13] = vmovl_s8(vreinterpret_s8_u8(l8(v[13])));
                if Y {
                    v[15] = vmovl_s8(vreinterpret_s8_u8(l8(v[15])));
                }
                v[12] = ss(vshlq_u16(uu(v[12]), v[31]));
                v[11] = ss(vshlq_u16(uu(v[11]), v[31]));
                v[10] = ss(vshlq_u16(uu(v[10]), v[31]));

                if Y && w2 & 0x03 != 0 {
                    // wd16
                    if VD { lpf_v_16_8(&mut v, x0, x1, w8, w9) } else { lpf_h_16_8(&mut v, x0, x1, w8, w9) }
                } else if w7 & 0x03 != 0 {
                    if Y {
                        // wd8
                        if VD { lpf_v_8_8(&mut v, x0, x1, w8, w9) } else { lpf_h_8_8(&mut v, x0, x1, w8, w9) }
                    } else {
                        // wd6
                        if VD { lpf_v_6_8(&mut v, x0, x1, w8, w9) } else { lpf_h_6_8(&mut v, x0, x1, w8, w9) }
                    }
                } else {
                    // wd4
                    if VD { lpf_v_4_8(&mut v, x0, x1, w8, w9) } else { lpf_h_4_8(&mut v, x0, x1, w8, w9) }
                }
            }
        }
        // 7: for dir h, the column filters advance x0 by 8 rows; if the
        // whole function is skipped, it is advanced here instead.
        if !VD {
            x0 = row(x0, x1, 8);
        }
        // 8:
        w6 >>= 2; // vmask[0] >>= 2
        w7 >>= 2; // vmask[1] >>= 2
        if Y {
            w2 >>= 2; // vmask[2] >>= 2
        }
        if VD {
            x0 = x0.add(8); // add x0, x0, #16 (bytes)
        }
        // cbnz w6, 1b
        if w6 == 0 {
            break;
        }
    }
}

macro_rules! entry {
    ($($name:ident: $v:expr, $y:expr;)*) => {$(
        /// `void dav1d_lpf_{v,h}_sb_{y,uv}_16bpc_neon(pixel *dst, const ptrdiff_t stride,
        /// const uint32_t *const vmask, const uint8_t (*l)[4], ptrdiff_t b4_stride,
        /// const Av1FilterLUT *lut, const int w, const int bitdepth_max)`
        pub unsafe extern "C" fn $name(
            dst: Dst,
            stride: isize,
            vmask: *const u32,
            l: *const [u8; 4],
            b4_stride: isize,
            lut: *const Av1FilterLUT,
            _w: c_int,
            bitdepth_max: c_int,
        ) {
            lpf_func::<$v, $y>(dst, stride, vmask, l, b4_stride, lut, bitdepth_max)
        }
    )*};
}
entry! {
    dav1d_lpf_v_sb_y_16bpc_neon: true, true;
    dav1d_lpf_h_sb_y_16bpc_neon: false, true;
    dav1d_lpf_v_sb_uv_16bpc_neon: true, false;
    dav1d_lpf_h_sb_uv_16bpc_neon: false, false;
}
