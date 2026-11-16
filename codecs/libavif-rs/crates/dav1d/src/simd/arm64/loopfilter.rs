// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/loopfilter.S` (8 bpc) with NEON
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The four entry points `dav1d_lpf_{v,h}_sb_{y,uv}_8bpc_neon` (the macro
//! `lpf_func`), the column filters `lpf_{v,h}_{4,6,8,16}_16_neon` and the
//! filter cores `lpf_16_wd{4,6,8,16}_neon` (the macro `loop_filter`).
//!
//! Correspondence with the assembly:
//! - The vector registers are the array `v` of `uint8x16_t`, indexed by
//!   register number; the filter core and the column filters work on it
//!   as the assembly does on `v0`–`v31` (`.8h` instructions reinterpret
//!   the register). `.8b` results (`sqxtn`, `rshrn`) clear the high half.
//! - The core returns `x14` (`1 << 0`: nothing to store, `1 << 4`: the
//!   inner 4 pixels, `1 << 6`: the inner 6, 0: all); the macros
//!   `lpf_16_wd*` that branch on it are `match`es in the column filters,
//!   `ret x15` a `return`.
//! - Post-incremented row pointers (`x16`, `x0`) are explicit row offsets.
//!   The horizontal column filters leave `x0` 16 rows further and the
//!   vertical ones where it was; the entry point's loop advances its own
//!   `x0` by the same amount after each call, as the assembly does.
//! - Lane loads and stores of 4 and 8 pixels (`ld1 {vN.s}[i]`,
//!   `ld1 {vN.d}[i]`) are unaligned reads and writes.
//!
//! Safety (argued once for the family): the translated caller
//! `lf_apply_tmpl` passes what the original passes to the assembly:
//! `dst` points into a picture plane with `stride` bytes per row, `vmask`
//! to 2 (`uv`) or 3 (`y`) masks, `l` to the filter levels of the edge's
//! blocks with `b4_stride` blocks per row, and `lut` to the frame's
//! `Av1FilterLUT`. The original's C equivalent (`loop_filter_*_sb128*_c`)
//! reads and writes, for each 4-pixel unit `i` whose bit is set in the
//! combined mask, the rows (`h`) or columns (`v`) `4i..4i+4` and up to 7
//! pixels on either side of the edge (8 to the left for `h` at wd16, as
//! the assembly loads), which the caller guarantees are inside the
//! plane; the assembly accesses the same pixels (it loads all 16 lines of
//! a group of four units only when one of them is set, and all four
//! units of a group lie in the plane) and the `l` entries `0..4n` of the
//! row (and the row above for `v`, the column to the left for `h`).
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::c_int;

/// The vector register file `v0`–`v31` as bytes.
type R = [uint8x16_t; 32];
type Dst = *mut u8;

/// `Av1FilterLUT` (src/lf_mask.h): `e[64]`, `i[64]`, `sharp[2]`.
#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [u8; 64],
    pub i: [u8; 64],
    pub sharp: [u64; 2],
}

// --- instructions -------------------------------------------------------------
// SAFETY of the value intrinsics below: NEON is baseline on AArch64.

#[inline(always)]
fn h(a: uint8x16_t) -> uint16x8_t {
    unsafe { vreinterpretq_u16_u8(a) }
}
#[inline(always)]
fn b(a: uint16x8_t) -> uint8x16_t {
    unsafe { vreinterpretq_u8_u16(a) }
}
#[inline(always)]
fn s(a: uint8x16_t) -> int8x16_t {
    unsafe { vreinterpretq_s8_u8(a) }
}
#[inline(always)]
fn u(a: int8x16_t) -> uint8x16_t {
    unsafe { vreinterpretq_u8_s8(a) }
}
/// `uaddl vd.8h, vn.8b, vm.8b`
#[inline(always)]
fn uaddl(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vaddl_u8(vget_low_u8(a), vget_low_u8(c))) }
}
/// `uaddl2 vd.8h, vn.16b, vm.16b`
#[inline(always)]
fn uaddl2(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vaddl_high_u8(a, c)) }
}
/// `uaddw vd.8h, vn.8h, vm.8b`
#[inline(always)]
fn uaddw(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vaddw_u8(h(a), vget_low_u8(c))) }
}
/// `uaddw2 vd.8h, vn.8h, vm.16b`
#[inline(always)]
fn uaddw2(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vaddw_high_u8(h(a), c)) }
}
/// `add vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn add8h(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vaddq_u16(h(a), h(c))) }
}
/// `sub vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn sub8h(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vsubq_u16(h(a), h(c))) }
}
/// `rshrn vd.8b, vn.8h, #N` (clears the high half)
#[inline(always)]
fn rshrn<const N: i32>(a: uint8x16_t) -> uint8x16_t {
    unsafe { vcombine_u8(vrshrn_n_u16::<N>(h(a)), vdup_n_u8(0)) }
}
/// `rshrn2 vd.16b, vn.8h, #N`
#[inline(always)]
fn rshrn2<const N: i32>(d: uint8x16_t, a: uint8x16_t) -> uint8x16_t {
    unsafe { vrshrn_high_n_u16::<N>(vget_low_u8(d), h(a)) }
}
/// `bit vd, vn, vm`: `vd = vm ? vn : vd` bitwise
#[inline(always)]
fn bit(d: uint8x16_t, n: uint8x16_t, m: uint8x16_t) -> uint8x16_t {
    unsafe { vbslq_u8(m, n, d) }
}
/// `bif vd, vn, vm`: `vd = vm ? vd : vn` bitwise
#[inline(always)]
fn bif(d: uint8x16_t, n: uint8x16_t, m: uint8x16_t) -> uint8x16_t {
    unsafe { vbslq_u8(m, d, n) }
}
/// `mov x16, vN.d[0] ; mov x17, vN.d[1] ; adds x16, x16, x17`: Z clear
#[inline(always)]
fn any(a: uint8x16_t) -> bool {
    unsafe {
        let a = vreinterpretq_u64_u8(a);
        vgetq_lane_u64::<0>(a).wrapping_add(vgetq_lane_u64::<1>(a)) != 0
    }
}
#[inline(always)]
fn movi(x: u8) -> uint8x16_t {
    unsafe { vdupq_n_u8(x) }
}

// --- memory --------------------------------------------------------------------

#[inline(always)]
unsafe fn row(p: Dst, stride: isize, r: isize) -> Dst {
    p.offset(r * stride)
}
/// `ld1 {vN.16b}, [p]`
#[inline(always)]
unsafe fn ld16(p: Dst) -> uint8x16_t {
    vld1q_u8(p)
}
/// `st1 {vN.16b}, [p]`
#[inline(always)]
unsafe fn st16(p: Dst, a: uint8x16_t) {
    vst1q_u8(p, a)
}
/// `ld1 {vN.s}[L], [p]`
#[inline(always)]
unsafe fn ld_s<const L: i32>(a: uint8x16_t, p: Dst) -> uint8x16_t {
    let x = core::ptr::read_unaligned(p.cast::<u32>());
    vreinterpretq_u8_u32(vsetq_lane_u32::<L>(x, vreinterpretq_u32_u8(a)))
}
/// `st1 {vN.s}[L], [p]`
#[inline(always)]
unsafe fn st_s<const L: i32>(p: Dst, a: uint8x16_t) {
    core::ptr::write_unaligned(p.cast::<u32>(), vgetq_lane_u32::<L>(vreinterpretq_u32_u8(a)))
}
/// `ld1 {vN.d}[L], [p]`
#[inline(always)]
unsafe fn ld_d<const L: i32>(a: uint8x16_t, p: Dst) -> uint8x16_t {
    let x = core::ptr::read_unaligned(p.cast::<u64>());
    vreinterpretq_u8_u64(vsetq_lane_u64::<L>(x, vreinterpretq_u64_u8(a)))
}
/// `st1 {vN.d}[L], [p]`
#[inline(always)]
unsafe fn st_d<const L: i32>(p: Dst, a: uint8x16_t) {
    core::ptr::write_unaligned(p.cast::<u64>(), vgetq_lane_u64::<L>(vreinterpretq_u64_u8(a)))
}

// --- util.S transposes -----------------------------------------------------------

#[inline(always)]
fn trn1_16b(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { vtrn1q_u8(a, c) }
}
#[inline(always)]
fn trn2_16b(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { vtrn2q_u8(a, c) }
}
#[inline(always)]
fn trn1_8h(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vtrn1q_u16(h(a), h(c))) }
}
#[inline(always)]
fn trn2_8h(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { b(vtrn2q_u16(h(a), h(c))) }
}
#[inline(always)]
fn trn1_4s(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { vreinterpretq_u8_u32(vtrn1q_u32(vreinterpretq_u32_u8(a), vreinterpretq_u32_u8(c))) }
}
#[inline(always)]
fn trn2_4s(a: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    unsafe { vreinterpretq_u8_u32(vtrn2q_u32(vreinterpretq_u32_u8(a), vreinterpretq_u32_u8(c))) }
}

/// `transpose_8x16b r0, …, r7, t8, t9`
#[inline(always)]
fn transpose_8x16b(v: &mut R, r: [usize; 8], t8: usize, t9: usize) {
    let [r0, r1, r2, r3, r4, r5, r6, r7] = r;
    v[t8] = trn1_16b(v[r0], v[r1]);
    v[t9] = trn2_16b(v[r0], v[r1]);
    v[r1] = trn1_16b(v[r2], v[r3]);
    v[r3] = trn2_16b(v[r2], v[r3]);
    v[r0] = trn1_16b(v[r4], v[r5]);
    v[r5] = trn2_16b(v[r4], v[r5]);
    v[r2] = trn1_16b(v[r6], v[r7]);
    v[r7] = trn2_16b(v[r6], v[r7]);

    v[r4] = trn1_8h(v[r0], v[r2]);
    v[r2] = trn2_8h(v[r0], v[r2]);
    v[r6] = trn1_8h(v[r5], v[r7]);
    v[r7] = trn2_8h(v[r5], v[r7]);
    v[r5] = trn1_8h(v[t9], v[r3]);
    v[t9] = trn2_8h(v[t9], v[r3]);
    v[r3] = trn1_8h(v[t8], v[r1]);
    v[t8] = trn2_8h(v[t8], v[r1]);

    v[r0] = trn1_4s(v[r3], v[r4]);
    v[r4] = trn2_4s(v[r3], v[r4]);
    v[r1] = trn1_4s(v[r5], v[r6]);
    v[r5] = trn2_4s(v[r5], v[r6]);
    v[r6] = trn2_4s(v[t8], v[r2]);
    v[r2] = trn1_4s(v[t8], v[r2]);
    v[r3] = trn1_4s(v[t9], v[r7]);
    v[r7] = trn2_4s(v[t9], v[r7]);
}

/// `transpose_4x16b r0, r1, r2, r3, t4, t5, t6, t7`
#[inline(always)]
fn transpose_4x16b(v: &mut R, r: [usize; 4], t: [usize; 4]) {
    let [r0, r1, r2, r3] = r;
    let [t4, t5, t6, t7] = t;
    v[t4] = trn1_16b(v[r0], v[r1]);
    v[t5] = trn2_16b(v[r0], v[r1]);
    v[t6] = trn1_16b(v[r2], v[r3]);
    v[t7] = trn2_16b(v[r2], v[r3]);

    v[r0] = trn1_8h(v[t4], v[t6]);
    v[r2] = trn2_8h(v[t4], v[t6]);
    v[r1] = trn1_8h(v[t5], v[t7]);
    v[r3] = trn2_8h(v[t5], v[t7]);
}

// --- loop_filter wd: lpf_16_wd{4,6,8,16}_neon ---------------------------------------

/// The macro `loop_filter wd` (`lpf_16_wd\wd\()_neon`). Inputs: `v17`–`v30`
/// the pixels p6…q6 (as many as `WD` needs), `v10` E, `v11` I, `v12` H,
/// `v13` the mask of wd >= 4, `v14` of wd > 4, `v15` of wd == 16. Returns
/// `x14`.
#[inline(always)]
fn lpf_16_wd<const WD: u32>(v: &mut R) -> u64 {
    // SAFETY: NEON value intrinsics only.
    unsafe {
        v[0] = vabdq_u8(v[22], v[23]); // abs(p1 - p0)
        v[1] = vabdq_u8(v[25], v[24]); // abs(q1 - q0)
        v[2] = vabdq_u8(v[23], v[24]); // abs(p0 - q0)
        v[3] = vabdq_u8(v[22], v[25]); // abs(p1 - q1)
        if WD >= 6 {
            v[4] = vabdq_u8(v[21], v[22]); // abs(p2 - p1)
            v[5] = vabdq_u8(v[26], v[25]); // abs(q2 - q1)
        }
        if WD >= 8 {
            v[6] = vabdq_u8(v[20], v[21]); // abs(p3 - p2)
            v[7] = vabdq_u8(v[27], v[26]); // abs(q3 - q3)
        }
        if WD >= 6 {
            v[4] = vmaxq_u8(v[4], v[5]);
        }
        v[2] = vqaddq_u8(v[2], v[2]); // abs(p0 - q0) * 2
        if WD >= 8 {
            v[6] = vmaxq_u8(v[6], v[7]);
        }
        v[3] = vshrq_n_u8::<1>(v[3]);
        if WD >= 8 {
            v[4] = vmaxq_u8(v[4], v[6]);
        }
        if WD >= 6 {
            v[4] = vandq_u8(v[4], v[14]);
        }
        v[0] = vmaxq_u8(v[0], v[1]); // max(abs(p1 - p0), abs(q1 - q0))
        v[2] = vqaddq_u8(v[2], v[3]); // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1
        if WD >= 6 {
            v[4] = vmaxq_u8(v[0], v[4]);
            v[1] = vcgeq_u8(v[11], v[4]); // max(abs(p1 - p0), abs(q1 - q0), abs(), abs(), ...) <= I
        } else {
            v[1] = vcgeq_u8(v[11], v[0]); // max(abs(p1 - p0), abs(q1 - q0)) <= I
        }
        v[2] = vcgeq_u8(v[10], v[2]); // abs(p0 - q0) * 2 + abs(p1 - q1) >> 1 <= E
        v[1] = vandq_u8(v[1], v[2]); // fm
        v[1] = vandq_u8(v[1], v[13]); // fm && wd >= 4
        if WD >= 6 {
            v[14] = vandq_u8(v[14], v[1]); // fm && wd > 4
        }
        if WD >= 16 {
            v[15] = vandq_u8(v[15], v[1]); // fm && wd == 16
        }

        if !any(v[1]) {
            // if (!fm || wd < 4) return;
            return 1 << 0;
        }
        // 9:
        let mut skip_wd4 = false;
        if WD >= 6 {
            v[10] = movi(1);
            v[2] = vabdq_u8(v[21], v[23]); // abs(p2 - p0)
            v[3] = vabdq_u8(v[22], v[23]); // abs(p1 - p0)
            v[4] = vabdq_u8(v[25], v[24]); // abs(q1 - q0)
            v[5] = vabdq_u8(v[26], v[24]); // abs(q2 - q0)
            if WD >= 8 {
                v[6] = vabdq_u8(v[20], v[23]); // abs(p3 - p0)
                v[7] = vabdq_u8(v[27], v[24]); // abs(q3 - q0)
            }
            v[2] = vmaxq_u8(v[2], v[3]);
            v[4] = vmaxq_u8(v[4], v[5]);
            if WD >= 8 {
                v[6] = vmaxq_u8(v[6], v[7]);
            }
            v[2] = vmaxq_u8(v[2], v[4]);
            if WD >= 8 {
                v[2] = vmaxq_u8(v[2], v[6]);
            }

            if WD == 16 {
                v[3] = vabdq_u8(v[17], v[23]); // abs(p6 - p0)
                v[4] = vabdq_u8(v[18], v[23]); // abs(p5 - p0)
                v[5] = vabdq_u8(v[19], v[23]); // abs(p4 - p0)
            }
            v[2] = vcgeq_u8(v[10], v[2]); // flat8in
            if WD == 16 {
                v[6] = vabdq_u8(v[28], v[24]); // abs(q4 - q0)
                v[7] = vabdq_u8(v[29], v[24]); // abs(q5 - q0)
                v[8] = vabdq_u8(v[30], v[24]); // abs(q6 - q0)
            }
            v[14] = vandq_u8(v[2], v[14]); // flat8in && fm && wd > 4
            v[1] = vbicq_u8(v[1], v[14]); // fm && wd >= 4 && !flat8in
            if WD == 16 {
                v[3] = vmaxq_u8(v[3], v[4]);
                v[5] = vmaxq_u8(v[5], v[6]);
            }
            let nz = any(v[1]);
            if WD == 16 {
                v[7] = vmaxq_u8(v[7], v[8]);
                v[3] = vmaxq_u8(v[3], v[5]);
                v[3] = vmaxq_u8(v[3], v[7]);
                v[3] = vcgeq_u8(v[10], v[3]); // flat8out
            }
            if WD == 16 {
                v[15] = vandq_u8(v[15], v[3]); // flat8out && fm && wd == 16
                v[15] = vandq_u8(v[15], v[14]); // flat8out && flat8in && fm && wd == 16
                v[14] = vbicq_u8(v[14], v[15]); // flat8in && fm && wd >= 4 && !flat8out
            }
            // b.eq 1f  // skip wd == 4 case
            skip_wd4 = !nz;
        }
        if !skip_wd4 {
            v[3] = movi(128);
            v[2] = veorq_u8(v[22], v[3]); // p1 - 128
            v[3] = veorq_u8(v[25], v[3]); // q1 - 128
            v[0] = vcgtq_u8(v[0], v[12]); // hev
            v[2] = u(vqsubq_s8(s(v[2]), s(v[3]))); // iclip_diff(p1 - q1)
            v[4] = vandq_u8(v[2], v[0]); // if (hev) iclip_diff(p1 - q1)
            v[0] = vbicq_u8(v[1], v[0]); // (fm && wd >= 4 && !hev)
            v[2] = b(vsubl_u8(vget_low_u8(v[24]), vget_low_u8(v[23])));
            v[5] = b(vdupq_n_u16(3));
            v[3] = b(vsubl_high_u8(v[24], v[23]));
            v[2] = b(vmulq_u16(h(v[2]), h(v[5])));
            v[3] = b(vmulq_u16(h(v[3]), h(v[5])));
            v[6] = movi(4);
            v[2] = vreinterpretq_u8_s16(vaddw_s8(vreinterpretq_s16_u8(v[2]), vget_low_s8(s(v[4]))));
            v[3] = vreinterpretq_u8_s16(vaddw_high_s8(vreinterpretq_s16_u8(v[3]), s(v[4])));
            v[7] = movi(3);
            v[2] = u(vcombine_s8(vqmovn_s16(vreinterpretq_s16_u8(v[2])), vdup_n_s8(0))); // f
            v[2] = u(vqmovn_high_s16(vget_low_s8(s(v[2])), vreinterpretq_s16_u8(v[3])));
            v[4] = u(vqaddq_s8(s(v[6]), s(v[2]))); // imin(f + 4, 127)
            v[5] = u(vqaddq_s8(s(v[7]), s(v[2]))); // imin(f + 3, 127)
            v[4] = u(vshrq_n_s8::<3>(s(v[4]))); // f1
            v[5] = u(vshrq_n_s8::<3>(s(v[5]))); // f2
            v[2] = v[23]; // p0
            v[3] = v[24]; // q0
            v[6] = u(vnegq_s8(s(v[4]))); // -f1
            v[4] = u(vrshrq_n_s8::<1>(s(v[4]))); // (f1 + 1) >> 1
            // p0 + f2, q0 - f1
            v[2] = vsqaddq_u8(v[2], s(v[5])); // out p0
            v[3] = vsqaddq_u8(v[3], s(v[6])); // out q0
            v[6] = u(vnegq_s8(s(v[4]))); // -((f1 + 1) >> 1)
            v[23] = bit(v[23], v[2], v[1]); // if (fm && wd >= 4)
            v[24] = bit(v[24], v[3], v[1]); // if (fm && wd >= 4)
            v[2] = v[22]; // p1
            v[3] = v[25]; // q1
            // p1 + ((f1 + 1) >> 1), q1 - ((f1 + 1) >> 1)
            v[2] = vsqaddq_u8(v[2], s(v[4])); // out p1
            v[3] = vsqaddq_u8(v[3], s(v[6])); // out q1
            v[22] = bit(v[22], v[2], v[0]); // if (fm && wd >= 4 && !hev)
            v[25] = bit(v[25], v[3], v[0]); // if (fm && wd >= 4 && !hev)
        }
        // 1:

        if WD == 6 {
            if any(v[14]) {
                v[0] = uaddl(v[21], v[21]); // p2 * 2
                v[1] = uaddl2(v[21], v[21]);
                v[2] = uaddl(v[21], v[22]); // p2 + p1
                v[3] = uaddl2(v[21], v[22]);
                v[4] = uaddl(v[22], v[23]); // p1 + p0
                v[5] = uaddl2(v[22], v[23]);
                v[6] = uaddl(v[23], v[24]); // p0 + q0
                v[7] = uaddl2(v[23], v[24]);
                v[8] = add8h(v[0], v[2]);
                v[9] = add8h(v[1], v[3]);
                v[10] = add8h(v[4], v[6]);
                v[11] = add8h(v[5], v[7]);
                v[12] = uaddl(v[24], v[25]); // q0 + q1
                v[13] = uaddl2(v[24], v[25]);
                v[8] = add8h(v[8], v[10]);
                v[9] = add8h(v[9], v[11]);
                v[12] = sub8h(v[12], v[0]);
                v[13] = sub8h(v[13], v[1]);
                v[10] = uaddl(v[25], v[26]); // q1 + q2
                v[11] = uaddl2(v[25], v[26]);
                v[0] = rshrn::<3>(v[8]); // out p1
                v[0] = rshrn2::<3>(v[0], v[9]);

                v[8] = add8h(v[8], v[12]);
                v[9] = add8h(v[9], v[13]);
                v[10] = sub8h(v[10], v[2]);
                v[11] = sub8h(v[11], v[3]);
                v[12] = uaddl(v[26], v[26]); // q2 + q2
                v[13] = uaddl2(v[26], v[26]);
                v[1] = rshrn::<3>(v[8]); // out p0
                v[1] = rshrn2::<3>(v[1], v[9]);

                v[8] = add8h(v[8], v[10]);
                v[9] = add8h(v[9], v[11]);
                v[12] = sub8h(v[12], v[4]);
                v[13] = sub8h(v[13], v[5]);
                v[2] = rshrn::<3>(v[8]); // out q0
                v[2] = rshrn2::<3>(v[2], v[9]);

                v[22] = bit(v[22], v[0], v[14]); // p1 if (flat8in)
                v[8] = add8h(v[8], v[12]);
                v[9] = add8h(v[9], v[13]);
                v[23] = bit(v[23], v[1], v[14]); // p0 if (flat8in)
                v[3] = rshrn::<3>(v[8]); // out q1
                v[3] = rshrn2::<3>(v[3], v[9]);
                v[24] = bit(v[24], v[2], v[14]); // q0 if (flat8in)
                v[25] = bit(v[25], v[3], v[14]); // q1 if (flat8in)
            }
            // 2:
        } else if WD >= 8 {
            if !any(v[14]) {
                if WD == 8 {
                    // b.eq 8f  // skip if there's no flat8in
                    return 1 << 4;
                }
                // b.eq 2f
            } else {
                v[0] = uaddl(v[20], v[21]); // p3 + p2
                v[1] = uaddl2(v[20], v[21]);
                v[2] = uaddl(v[22], v[25]); // p1 + q1
                v[3] = uaddl2(v[22], v[25]);
                v[4] = uaddl(v[20], v[22]); // p3 + p1
                v[5] = uaddl2(v[20], v[22]);
                v[6] = uaddl(v[23], v[26]); // p0 + q2
                v[7] = uaddl2(v[23], v[26]);
                v[8] = add8h(v[0], v[0]); // 2 * (p3 + p2)
                v[9] = add8h(v[1], v[1]);
                v[8] = uaddw(v[8], v[23]); // + p0
                v[9] = uaddw2(v[9], v[23]);
                v[8] = uaddw(v[8], v[24]); // + q0
                v[9] = uaddw2(v[9], v[24]);
                v[8] = add8h(v[8], v[4]);
                v[9] = add8h(v[9], v[5]); // + p3 + p1
                v[2] = sub8h(v[2], v[0]); // p1 + q1 - p3 - p2
                v[3] = sub8h(v[3], v[1]);
                v[6] = sub8h(v[6], v[4]); // p0 + q2 - p3 - p1
                v[7] = sub8h(v[7], v[5]);
                v[10] = rshrn::<3>(v[8]); // out p2
                v[10] = rshrn2::<3>(v[10], v[9]);

                v[8] = add8h(v[8], v[2]);
                v[9] = add8h(v[9], v[3]);
                v[0] = uaddl(v[20], v[23]); // p3 + p0
                v[1] = uaddl2(v[20], v[23]);
                v[2] = uaddl(v[24], v[27]); // q0 + q3
                v[3] = uaddl2(v[24], v[27]);
                v[11] = rshrn::<3>(v[8]); // out p1
                v[11] = rshrn2::<3>(v[11], v[9]);

                v[8] = add8h(v[8], v[6]);
                v[9] = add8h(v[9], v[7]);
                v[2] = sub8h(v[2], v[0]); // q0 + q3 - p3 - p0
                v[3] = sub8h(v[3], v[1]);
                v[4] = uaddl(v[21], v[24]); // p2 + q0
                v[5] = uaddl2(v[21], v[24]);
                v[6] = uaddl(v[25], v[27]); // q1 + q3
                v[7] = uaddl2(v[25], v[27]);
                v[12] = rshrn::<3>(v[8]); // out p0
                v[12] = rshrn2::<3>(v[12], v[9]);

                v[8] = add8h(v[8], v[2]);
                v[9] = add8h(v[9], v[3]);
                v[6] = sub8h(v[6], v[4]); // q1 + q3 - p2 - q0
                v[7] = sub8h(v[7], v[5]);
                v[0] = uaddl(v[22], v[25]); // p1 + q1
                v[1] = uaddl2(v[22], v[25]);
                v[2] = uaddl(v[26], v[27]); // q2 + q3
                v[3] = uaddl2(v[26], v[27]);
                v[13] = rshrn::<3>(v[8]); // out q0
                v[13] = rshrn2::<3>(v[13], v[9]);

                v[8] = add8h(v[8], v[6]);
                v[9] = add8h(v[9], v[7]);
                v[2] = sub8h(v[2], v[0]); // q2 + q3 - p1 - q1
                v[3] = sub8h(v[3], v[1]);
                v[0] = rshrn::<3>(v[8]); // out q1
                v[0] = rshrn2::<3>(v[0], v[9]);

                v[8] = add8h(v[8], v[2]);
                v[9] = add8h(v[9], v[3]);

                v[21] = bit(v[21], v[10], v[14]);
                v[22] = bit(v[22], v[11], v[14]);
                v[23] = bit(v[23], v[12], v[14]);
                v[1] = rshrn::<3>(v[8]); // out q2
                v[1] = rshrn2::<3>(v[1], v[9]);
                v[24] = bit(v[24], v[13], v[14]);
                v[25] = bit(v[25], v[0], v[14]);
                v[26] = bit(v[26], v[1], v[14]);
            }
        }
        // 2:
        if WD == 16 {
            if !any(v[15]) {
                // b.ne 1f  // check if flat8out is needed
                if !any(v[14]) {
                    return 1 << 4; // 8: if there was no flat8in, just write the inner 4 pixels
                }
                return 1 << 6; // 7: if flat8in was used, write the inner 6 pixels
            }
            // 1:
            v[2] = uaddl(v[17], v[17]); // p6 + p6
            v[3] = uaddl2(v[17], v[17]);
            v[4] = uaddl(v[17], v[18]); // p6 + p5
            v[5] = uaddl2(v[17], v[18]);
            v[6] = uaddl(v[17], v[19]); // p6 + p4
            v[7] = uaddl2(v[17], v[19]);
            v[8] = uaddl(v[17], v[20]); // p6 + p3
            v[9] = uaddl2(v[17], v[20]);
            v[12] = add8h(v[2], v[4]);
            v[13] = add8h(v[3], v[5]);
            v[10] = add8h(v[6], v[8]);
            v[11] = add8h(v[7], v[9]);
            v[6] = uaddl(v[17], v[21]); // p6 + p2
            v[7] = uaddl2(v[17], v[21]);
            v[12] = add8h(v[12], v[10]);
            v[13] = add8h(v[13], v[11]);
            v[8] = uaddl(v[17], v[22]); // p6 + p1
            v[9] = uaddl2(v[17], v[22]);
            v[10] = uaddl(v[18], v[23]); // p5 + p0
            v[11] = uaddl2(v[18], v[23]);
            v[6] = add8h(v[6], v[8]);
            v[7] = add8h(v[7], v[9]);
            v[8] = uaddl(v[19], v[24]); // p4 + q0
            v[9] = uaddl2(v[19], v[24]);
            v[12] = add8h(v[12], v[6]);
            v[13] = add8h(v[13], v[7]);
            v[10] = add8h(v[10], v[8]);
            v[11] = add8h(v[11], v[9]);
            v[6] = uaddl(v[20], v[25]); // p3 + q1
            v[7] = uaddl2(v[20], v[25]);
            v[12] = add8h(v[12], v[10]);
            v[13] = add8h(v[13], v[11]);
            v[6] = sub8h(v[6], v[2]);
            v[7] = sub8h(v[7], v[3]);
            v[2] = uaddl(v[21], v[26]); // p2 + q2
            v[3] = uaddl2(v[21], v[26]);
            v[0] = rshrn::<4>(v[12]); // out p5
            v[0] = rshrn2::<4>(v[0], v[13]);
            v[12] = add8h(v[12], v[6]); // - (p6 + p6) + (p3 + q1)
            v[13] = add8h(v[13], v[7]);
            v[2] = sub8h(v[2], v[4]);
            v[3] = sub8h(v[3], v[5]);
            v[4] = uaddl(v[22], v[27]); // p1 + q3
            v[5] = uaddl2(v[22], v[27]);
            v[6] = uaddl(v[17], v[19]); // p6 + p4
            v[7] = uaddl2(v[17], v[19]);
            v[1] = rshrn::<4>(v[12]); // out p4
            v[1] = rshrn2::<4>(v[1], v[13]);
            v[12] = add8h(v[12], v[2]); // - (p6 + p5) + (p2 + q2)
            v[13] = add8h(v[13], v[3]);
            v[4] = sub8h(v[4], v[6]);
            v[5] = sub8h(v[5], v[7]);
            v[6] = uaddl(v[23], v[28]); // p0 + q4
            v[7] = uaddl2(v[23], v[28]);
            v[8] = uaddl(v[17], v[20]); // p6 + p3
            v[9] = uaddl2(v[17], v[20]);
            v[2] = rshrn::<4>(v[12]); // out p3
            v[2] = rshrn2::<4>(v[2], v[13]);
            v[12] = add8h(v[12], v[4]); // - (p6 + p4) + (p1 + q3)
            v[13] = add8h(v[13], v[5]);
            v[6] = sub8h(v[6], v[8]);
            v[7] = sub8h(v[7], v[9]);
            v[8] = uaddl(v[24], v[29]); // q0 + q5
            v[9] = uaddl2(v[24], v[29]);
            v[4] = uaddl(v[17], v[21]); // p6 + p2
            v[5] = uaddl2(v[17], v[21]);
            v[3] = rshrn::<4>(v[12]); // out p2
            v[3] = rshrn2::<4>(v[3], v[13]);
            v[12] = add8h(v[12], v[6]); // - (p6 + p3) + (p0 + q4)
            v[13] = add8h(v[13], v[7]);
            v[8] = sub8h(v[8], v[4]);
            v[9] = sub8h(v[9], v[5]);
            v[6] = uaddl(v[25], v[30]); // q1 + q6
            v[7] = uaddl2(v[25], v[30]);
            v[10] = uaddl(v[17], v[22]); // p6 + p1
            v[11] = uaddl2(v[17], v[22]);
            v[4] = rshrn::<4>(v[12]); // out p1
            v[4] = rshrn2::<4>(v[4], v[13]);
            v[12] = add8h(v[12], v[8]); // - (p6 + p2) + (q0 + q5)
            v[13] = add8h(v[13], v[9]);
            v[6] = sub8h(v[6], v[10]);
            v[7] = sub8h(v[7], v[11]);
            v[8] = uaddl(v[26], v[30]); // q2 + q6
            v[9] = uaddl2(v[26], v[30]);
            v[0] = bif(v[0], v[18], v[15]); // out p5
            v[10] = uaddl(v[18], v[23]); // p5 + p0
            v[11] = uaddl2(v[18], v[23]);
            v[5] = rshrn::<4>(v[12]); // out p0
            v[5] = rshrn2::<4>(v[5], v[13]);
            v[12] = add8h(v[12], v[6]); // - (p6 + p1) + (q1 + q6)
            v[13] = add8h(v[13], v[7]);
            v[8] = sub8h(v[8], v[10]);
            v[9] = sub8h(v[9], v[11]);
            v[10] = uaddl(v[27], v[30]); // q3 + q6
            v[11] = uaddl2(v[27], v[30]);
            v[1] = bif(v[1], v[19], v[15]); // out p4
            v[18] = uaddl(v[19], v[24]); // p4 + q0
            v[19] = uaddl2(v[19], v[24]);
            v[6] = rshrn::<4>(v[12]); // out q0
            v[6] = rshrn2::<4>(v[6], v[13]);
            v[12] = add8h(v[12], v[8]); // - (p5 + p0) + (q2 + q6)
            v[13] = add8h(v[13], v[9]);
            v[10] = sub8h(v[10], v[18]);
            v[11] = sub8h(v[11], v[19]);
            v[8] = uaddl(v[28], v[30]); // q4 + q6
            v[9] = uaddl2(v[28], v[30]);
            v[2] = bif(v[2], v[20], v[15]); // out p3
            v[18] = uaddl(v[20], v[25]); // p3 + q1
            v[19] = uaddl2(v[20], v[25]);
            v[7] = rshrn::<4>(v[12]); // out q1
            v[7] = rshrn2::<4>(v[7], v[13]);
            v[12] = add8h(v[12], v[10]); // - (p4 + q0) + (q3 + q6)
            v[13] = add8h(v[13], v[11]);
            v[18] = sub8h(v[8], v[18]);
            v[19] = sub8h(v[9], v[19]);
            v[10] = uaddl(v[29], v[30]); // q5 + q6
            v[11] = uaddl2(v[29], v[30]);
            v[3] = bif(v[3], v[21], v[15]); // out p2
            v[20] = uaddl(v[21], v[26]); // p2 + q2
            v[21] = uaddl2(v[21], v[26]);
            v[8] = rshrn::<4>(v[12]); // out q2
            v[8] = rshrn2::<4>(v[8], v[13]);
            v[12] = add8h(v[12], v[18]); // - (p3 + q1) + (q4 + q6)
            v[13] = add8h(v[13], v[19]);
            v[10] = sub8h(v[10], v[20]);
            v[11] = sub8h(v[11], v[21]);
            v[18] = uaddl(v[30], v[30]); // q6 + q6
            v[19] = uaddl2(v[30], v[30]);
            v[4] = bif(v[4], v[22], v[15]); // out p1
            v[20] = uaddl(v[22], v[27]); // p1 + q3
            v[21] = uaddl2(v[22], v[27]);
            v[9] = rshrn::<4>(v[12]); // out q3
            v[9] = rshrn2::<4>(v[9], v[13]);
            v[12] = add8h(v[12], v[10]); // - (p2 + q2) + (q5 + q6)
            v[13] = add8h(v[13], v[11]);
            v[18] = sub8h(v[18], v[20]);
            v[19] = sub8h(v[19], v[21]);
            v[5] = bif(v[5], v[23], v[15]); // out p0
            v[10] = rshrn::<4>(v[12]); // out q4
            v[10] = rshrn2::<4>(v[10], v[13]);
            v[12] = add8h(v[12], v[18]); // - (p1 + q3) + (q6 + q6)
            v[13] = add8h(v[13], v[19]);
            v[11] = rshrn::<4>(v[12]); // out q5
            v[11] = rshrn2::<4>(v[11], v[13]);
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

// --- the column filters lpf_{v,h}_{4,6,8,16}_16_neon --------------------------------

/// Store the 4 inner rows p1, p0, q0, q1 (`v22`–`v25`) of a vertical edge.
#[inline(always)]
unsafe fn store_v4(v: &R, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -2);
    st16(row(x16, x1, 0), v[22]); // p1
    st16(row(x0, x1, 0), v[24]); // q0
    st16(row(x16, x1, 1), v[23]); // p0
    st16(row(x0, x1, 1), v[25]); // q1
}

/// Store p2 … q2 (`v21`–`v26`) of a vertical edge.
#[inline(always)]
unsafe fn store_v6(v: &R, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -3);
    st16(row(x16, x1, 0), v[21]); // p2
    st16(row(x0, x1, 0), v[24]); // q0
    st16(row(x16, x1, 1), v[22]); // p1
    st16(row(x0, x1, 1), v[25]); // q1
    st16(row(x16, x1, 2), v[23]); // p0
    st16(row(x0, x1, 2), v[26]); // q2
}

/// Transpose `v22`–`v25` back and store the 4 inner columns of a
/// horizontal edge (`sub x16, x0, x1, lsl #4 ; sub x16, x16, #2` with
/// `x0` 16 rows below the edge's first row).
#[inline(always)]
unsafe fn store_h4(v: &mut R, x0: Dst, x1: isize) {
    let x16 = x0.offset(-2);
    transpose_4x16b(v, [22, 23, 24, 25], [26, 27, 28, 29]);
    let x0 = row(x16, x1, 8);
    st_s::<0>(row(x16, x1, 0), v[22]);
    st_s::<2>(row(x0, x1, 0), v[22]);
    st_s::<0>(row(x16, x1, 1), v[23]);
    st_s::<2>(row(x0, x1, 1), v[23]);
    st_s::<0>(row(x16, x1, 2), v[24]);
    st_s::<2>(row(x0, x1, 2), v[24]);
    st_s::<0>(row(x16, x1, 3), v[25]);
    st_s::<2>(row(x0, x1, 3), v[25]);
    st_s::<1>(row(x16, x1, 4), v[22]);
    st_s::<3>(row(x0, x1, 4), v[22]);
    st_s::<1>(row(x16, x1, 5), v[23]);
    st_s::<3>(row(x0, x1, 5), v[23]);
    st_s::<1>(row(x16, x1, 6), v[24]);
    st_s::<3>(row(x0, x1, 6), v[24]);
    st_s::<1>(row(x16, x1, 7), v[25]);
    st_s::<3>(row(x0, x1, 7), v[25]);
}

/// Transpose `v20`–`v27` back and store 8 columns (p3 … q3) of a
/// horizontal edge.
#[inline(always)]
unsafe fn store_h8(v: &mut R, x0: Dst, x1: isize) {
    let x16 = x0.offset(-4);
    transpose_8x16b(v, [20, 21, 22, 23, 24, 25, 26, 27], 28, 29);
    let x0 = row(x16, x1, 8);
    for (i, r) in [20usize, 21, 22, 23, 24, 25, 26, 27].into_iter().enumerate() {
        st_d::<0>(row(x16, x1, i as isize), v[r]);
        st_d::<1>(row(x0, x1, i as isize), v[r]);
    }
}

/// Load 8 columns (`x0 - 4` …) of 16 rows into `v20`–`v27` and transpose.
#[inline(always)]
unsafe fn load_h8(v: &mut R, x0: Dst, x1: isize) {
    let x16 = x0.offset(-4);
    let x0 = row(x16, x1, 8);
    for (i, r) in [20usize, 21, 22, 23, 24, 25, 26, 27].into_iter().enumerate() {
        v[r] = ld_d::<0>(v[r], row(x16, x1, i as isize));
        v[r] = ld_d::<1>(v[r], row(x0, x1, i as isize));
    }
    transpose_8x16b(v, [20, 21, 22, 23, 24, 25, 26, 27], 28, 29);
}

/// `lpf_v_4_16_neon`
#[inline(always)]
unsafe fn lpf_v_4_16(v: &mut R, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -2);
    v[22] = ld16(row(x16, x1, 0)); // p1
    v[24] = ld16(row(x0, x1, 0)); // q0
    v[23] = ld16(row(x16, x1, 1)); // p0
    v[25] = ld16(row(x0, x1, 1)); // q1

    // lpf_16_wd4
    if lpf_16_wd::<4>(v) != 0 {
        return;
    }

    store_v4(v, x0, x1);
}

/// `lpf_h_4_16_neon`
#[inline(always)]
unsafe fn lpf_h_4_16(v: &mut R, x0: Dst, x1: isize) {
    let x16 = x0.offset(-2);
    let x0b = row(x16, x1, 8);
    v[22] = ld_s::<0>(v[22], row(x16, x1, 0));
    v[22] = ld_s::<2>(v[22], row(x0b, x1, 0));
    v[23] = ld_s::<0>(v[23], row(x16, x1, 1));
    v[23] = ld_s::<2>(v[23], row(x0b, x1, 1));
    v[24] = ld_s::<0>(v[24], row(x16, x1, 2));
    v[24] = ld_s::<2>(v[24], row(x0b, x1, 2));
    v[25] = ld_s::<0>(v[25], row(x16, x1, 3));
    v[25] = ld_s::<2>(v[25], row(x0b, x1, 3));
    v[22] = ld_s::<1>(v[22], row(x16, x1, 4));
    v[22] = ld_s::<3>(v[22], row(x0b, x1, 4));
    v[23] = ld_s::<1>(v[23], row(x16, x1, 5));
    v[23] = ld_s::<3>(v[23], row(x0b, x1, 5));
    v[24] = ld_s::<1>(v[24], row(x16, x1, 6));
    v[24] = ld_s::<3>(v[24], row(x0b, x1, 6));
    v[25] = ld_s::<1>(v[25], row(x16, x1, 7));
    v[25] = ld_s::<3>(v[25], row(x0b, x1, 7));

    transpose_4x16b(v, [22, 23, 24, 25], [26, 27, 28, 29]);

    // lpf_16_wd4
    if lpf_16_wd::<4>(v) != 0 {
        return;
    }

    store_h4(v, x0, x1);
}

/// `lpf_v_6_16_neon`
#[inline(always)]
unsafe fn lpf_v_6_16(v: &mut R, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -3);
    v[21] = ld16(row(x16, x1, 0)); // p2
    v[24] = ld16(row(x0, x1, 0)); // q0
    v[22] = ld16(row(x16, x1, 1)); // p1
    v[25] = ld16(row(x0, x1, 1)); // q1
    v[23] = ld16(row(x16, x1, 2)); // p0
    v[26] = ld16(row(x0, x1, 2)); // q2

    // lpf_16_wd6
    if lpf_16_wd::<6>(v) != 0 {
        return;
    }

    store_v4(v, x0, x1);
}

/// `lpf_h_6_16_neon`
#[inline(always)]
unsafe fn lpf_h_6_16(v: &mut R, x0: Dst, x1: isize) {
    load_h8(v, x0, x1);

    // lpf_16_wd6
    if lpf_16_wd::<6>(v) != 0 {
        return;
    }

    store_h4(v, x0, x1);
}

/// `lpf_v_8_16_neon`
#[inline(always)]
unsafe fn lpf_v_8_16(v: &mut R, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -4);
    v[20] = ld16(row(x16, x1, 0)); // p3
    v[24] = ld16(row(x0, x1, 0)); // q0
    v[21] = ld16(row(x16, x1, 1)); // p2
    v[25] = ld16(row(x0, x1, 1)); // q1
    v[22] = ld16(row(x16, x1, 2)); // p1
    v[26] = ld16(row(x0, x1, 2)); // q2
    v[23] = ld16(row(x16, x1, 3)); // p0
    v[27] = ld16(row(x0, x1, 3)); // q3

    // lpf_16_wd8
    match lpf_16_wd::<8>(v) {
        0 => store_v6(v, x0, x1),
        x14 if x14 & (1 << 4) != 0 => store_v4(v, x0, x1), // 8:
        _ => {}
    }
}

/// `lpf_h_8_16_neon`
#[inline(always)]
unsafe fn lpf_h_8_16(v: &mut R, x0: Dst, x1: isize) {
    load_h8(v, x0, x1);

    // lpf_16_wd8
    match lpf_16_wd::<8>(v) {
        0 => store_h8(v, x0, x1),
        x14 if x14 & (1 << 4) != 0 => store_h4(v, x0, x1), // 8:
        _ => {}
    }
}

/// `lpf_v_16_16_neon`
#[inline(always)]
unsafe fn lpf_v_16_16(v: &mut R, x0: Dst, x1: isize) {
    let x16 = row(x0, x1, -7);
    v[17] = ld16(row(x16, x1, 0)); // p6
    v[24] = ld16(row(x0, x1, 0)); // q0
    v[18] = ld16(row(x16, x1, 1)); // p5
    v[25] = ld16(row(x0, x1, 1)); // q1
    v[19] = ld16(row(x16, x1, 2)); // p4
    v[26] = ld16(row(x0, x1, 2)); // q2
    v[20] = ld16(row(x16, x1, 3)); // p3
    v[27] = ld16(row(x0, x1, 3)); // q3
    v[21] = ld16(row(x16, x1, 4)); // p2
    v[28] = ld16(row(x0, x1, 4)); // q4
    v[22] = ld16(row(x16, x1, 5)); // p1
    v[29] = ld16(row(x0, x1, 5)); // q5
    v[23] = ld16(row(x16, x1, 6)); // p0
    v[30] = ld16(row(x0, x1, 6)); // q6

    // lpf_16_wd16
    match lpf_16_wd::<16>(v) {
        0 => {
            let x16 = row(x0, x1, -6);
            st16(row(x16, x1, 0), v[0]); // p5
            st16(row(x0, x1, 0), v[6]); // q0
            st16(row(x16, x1, 1), v[1]); // p4
            st16(row(x0, x1, 1), v[7]); // q1
            st16(row(x16, x1, 2), v[2]); // p3
            st16(row(x0, x1, 2), v[8]); // q2
            st16(row(x16, x1, 3), v[3]); // p2
            st16(row(x0, x1, 3), v[9]); // q3
            st16(row(x16, x1, 4), v[4]); // p1
            st16(row(x0, x1, 4), v[10]); // q4
            st16(row(x16, x1, 5), v[5]); // p0
            st16(row(x0, x1, 5), v[11]); // q5
        }
        x14 if x14 & (1 << 6) != 0 => store_v6(v, x0, x1), // 7:
        x14 if x14 & (1 << 4) != 0 => store_v4(v, x0, x1), // 8:
        _ => {}
    }
}

/// `lpf_h_16_16_neon`
#[inline(always)]
unsafe fn lpf_h_16_16(v: &mut R, x0: Dst, x1: isize) {
    let x16 = x0.offset(-8);
    for (i, (p, q)) in [(16usize, 24usize), (17, 25), (18, 26), (19, 27), (20, 28), (21, 29), (22, 30), (23, 31)]
        .into_iter()
        .enumerate()
    {
        v[p] = ld_d::<0>(v[p], row(x16, x1, i as isize));
        v[q] = ld_d::<0>(v[q], row(x0, x1, i as isize));
    }
    for (i, (p, q)) in [(16usize, 24usize), (17, 25), (18, 26), (19, 27), (20, 28), (21, 29), (22, 30), (23, 31)]
        .into_iter()
        .enumerate()
    {
        v[p] = ld_d::<1>(v[p], row(x16, x1, 8 + i as isize));
        v[q] = ld_d::<1>(v[q], row(x0, x1, 8 + i as isize));
    }

    transpose_8x16b(v, [16, 17, 18, 19, 20, 21, 22, 23], 0, 1);
    transpose_8x16b(v, [24, 25, 26, 27, 28, 29, 30, 31], 0, 1);

    // lpf_16_wd16
    match lpf_16_wd::<16>(v) {
        0 => {
            let x16 = x0.offset(-8);

            transpose_8x16b(v, [16, 17, 0, 1, 2, 3, 4, 5], 18, 19);
            transpose_8x16b(v, [6, 7, 8, 9, 10, 11, 30, 31], 18, 19);

            for (i, (p, q)) in [(16usize, 6usize), (17, 7), (0, 8), (1, 9), (2, 10), (3, 11), (4, 30), (5, 31)]
                .into_iter()
                .enumerate()
            {
                st_d::<0>(row(x16, x1, i as isize), v[p]);
                st_d::<0>(row(x0, x1, i as isize), v[q]);
            }
            for (i, (p, q)) in [(16usize, 6usize), (17, 7), (0, 8), (1, 9), (2, 10), (3, 11), (4, 30), (5, 31)]
                .into_iter()
                .enumerate()
            {
                st_d::<1>(row(x16, x1, 8 + i as isize), v[p]);
                st_d::<1>(row(x0, x1, 8 + i as isize), v[q]);
            }
        }
        x14 if x14 & (1 << 6) != 0 => store_h8(v, x0, x1), // 7:
        x14 if x14 & (1 << 4) != 0 => store_h4(v, x0, x1), // 8:
        _ => {}
    }
}

// --- lpf_func dir, type ---------------------------------------------------------------

/// `word_1248`
static WORD_1248: [u32; 4] = [1, 2, 4, 8];

/// The macro `lpf_func dir, type`: `V` for `v` (row edges), `Y` for `y`.
#[inline(always)]
unsafe fn lpf_func<const V: bool, const Y: bool>(
    dst: Dst,
    stride: isize,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: isize,
    lut: *const Av1FilterLUT,
) {
    let mut v: R = [vdupq_n_u8(0); 32];
    let mut x0 = dst;
    let x1 = stride;
    // ldp w6, w7, [x2]  // vmask[0], vmask[1]
    let mut w6 = core::ptr::read(vmask);
    let mut w7 = core::ptr::read(vmask.add(1));
    // .ifc \type, y: ldr w2, [x2, #8]  // vmask[2]
    let mut w2 = if Y { core::ptr::read(vmask.add(2)) } else { 0 };
    // add x5, x5, #128  // Move to sharp part of lut
    let x5 = (lut as *const u8).add(128);
    if Y {
        w7 |= w2; // vmask[1] |= vmask[2]
    }
    // .ifc \dir, v: sub x4, x3, x4, lsl #2
    // .else: sub x3, x3, #4 ; lsl x4, x4, #2
    let mut x3 = l as *const u8;
    let mut x4: *const u8 = core::ptr::null();
    let x4s = b4_stride * 4;
    if V {
        x4 = x3.offset(-x4s);
    } else {
        x3 = x3.offset(-4);
    }
    w6 |= w7; // vmask[0] |= vmask[1]

    loop {
        // 1:
        let skip = w6 & 0x0f == 0;
        if V {
            v[0] = vld1q_u8(x4);
            x4 = x4.add(16);
            v[1] = vld1q_u8(x3);
            x3 = x3.add(16);
        } else {
            // ld2 {v0.s, v1.s}[i], [x3], x4
            let mut a = vreinterpretq_u32_u8(v[0]);
            let mut c = vreinterpretq_u32_u8(v[1]);
            let p = core::ptr::read_unaligned(x3.cast::<[u32; 2]>());
            a = vsetq_lane_u32::<0>(p[0], a);
            c = vsetq_lane_u32::<0>(p[1], c);
            x3 = x3.offset(x4s);
            let p = core::ptr::read_unaligned(x3.cast::<[u32; 2]>());
            a = vsetq_lane_u32::<1>(p[0], a);
            c = vsetq_lane_u32::<1>(p[1], c);
            x3 = x3.offset(x4s);
            let p = core::ptr::read_unaligned(x3.cast::<[u32; 2]>());
            a = vsetq_lane_u32::<2>(p[0], a);
            c = vsetq_lane_u32::<2>(p[1], c);
            x3 = x3.offset(x4s);
            let p = core::ptr::read_unaligned(x3.cast::<[u32; 2]>());
            a = vsetq_lane_u32::<3>(p[0], a);
            c = vsetq_lane_u32::<3>(p[1], c);
            x3 = x3.offset(x4s);
            v[0] = vreinterpretq_u8_u32(a);
            v[1] = vreinterpretq_u8_u32(c);
        }
        // b.eq 7f  // if (!(vm & bits)) continue;
        if !skip {
            v[5] = vld1q_dup_u8(x5); // sharp[0]
            v[2] = vreinterpretq_u8_u32(vdupq_n_u32(0xff));
            v[13] = vreinterpretq_u8_u32(vdupq_n_u32(w6)); // vmask[0]

            v[0] = vandq_u8(v[0], v[2]); // Keep only lowest byte in each 32 bit word
            v[1] = vandq_u8(v[1], v[2]);
            v[3] = vtstq_u8(v[1], v[2]); // Check for nonzero values in l[0][0]
            v[4] = movi(1);
            v[6] = vld1q_dup_u8(x5.add(8)); // sharp[1]
            v[1] = bif(v[1], v[0], v[3]); // if (!l[0][0]) L = l[offset][0]
            v[2] = vreinterpretq_u8_u32(vtstq_u32(vreinterpretq_u32_u8(v[1]), vreinterpretq_u32_u8(v[2]))); // L != 0
            v[1] = vreinterpretq_u8_u32(vmulq_u32(vreinterpretq_u32_u8(v[1]), vreinterpretq_u32_u8(v[4]))); // L
            if Y {
                v[15] = vreinterpretq_u8_u32(vdupq_n_u32(w2)); // vmask[2]
            }
            v[14] = vreinterpretq_u8_u32(vdupq_n_u32(w7)); // vmask[1]
            // b.eq 7f  // if (!L) continue;
            if any(v[2]) {
                v[5] = u(vnegq_s8(s(v[5]))); // -sharp[0]
                v[16] = vreinterpretq_u8_u32(vld1q_u32(WORD_1248.as_ptr()));
                v[12] = vshrq_n_u8::<4>(v[1]); // H
                v[3] = u(vshlq_s8(s(v[1]), s(v[5]))); // L >> sharp[0]
                if Y {
                    v[15] = vreinterpretq_u8_u32(vtstq_u32(vreinterpretq_u32_u8(v[15]), vreinterpretq_u32_u8(v[16]))); // if (vmask[2] & bits)
                }
                v[7] = movi(2);
                v[3] = vminq_u8(v[3], v[6]); // imin(L >> sharp[0], sharp[1])
                v[0] = vaddq_u8(v[1], v[7]); // L + 2
                v[11] = vmaxq_u8(v[3], v[4]); // imax(imin(), 1) = limit = I
                v[0] = vaddq_u8(v[0], v[0]); // 2*(L + 2)
                v[14] = vreinterpretq_u8_u32(vtstq_u32(vreinterpretq_u32_u8(v[14]), vreinterpretq_u32_u8(v[16]))); // if (vmask[1] & bits)
                v[10] = vaddq_u8(v[0], v[11]); // 2*(L + 2) + limit = E
                v[13] = vreinterpretq_u8_u32(vtstq_u32(vreinterpretq_u32_u8(v[13]), vreinterpretq_u32_u8(v[16]))); // if (vmask[0] & bits)
                v[13] = vandq_u8(v[13], v[2]); // vmask[0] &= L != 0

                if Y && w2 & 0x0f != 0 {
                    // wd16
                    if V { lpf_v_16_16(&mut v, x0, x1) } else { lpf_h_16_16(&mut v, x0, x1) }
                } else if w7 & 0x0f != 0 {
                    if Y {
                        // wd8
                        if V { lpf_v_8_16(&mut v, x0, x1) } else { lpf_h_8_16(&mut v, x0, x1) }
                    } else {
                        // wd6
                        if V { lpf_v_6_16(&mut v, x0, x1) } else { lpf_h_6_16(&mut v, x0, x1) }
                    }
                } else {
                    // wd4
                    if V { lpf_v_4_16(&mut v, x0, x1) } else { lpf_h_4_16(&mut v, x0, x1) }
                }
            }
        }
        // 7: for dir h, the column filters advance x0 by 16 rows; if the
        // whole function is skipped, it is advanced here instead.
        if !V {
            x0 = row(x0, x1, 16);
        }
        // 8:
        w6 >>= 4; // vmask[0] >>= 4
        w7 >>= 4; // vmask[1] >>= 4
        if Y {
            w2 >>= 4; // vmask[2] >>= 4
        }
        if V {
            x0 = x0.add(16);
        }
        // cbnz w6, 1b
        if w6 == 0 {
            break;
        }
    }
}

/// `void dav1d_lpf_v_sb_y_8bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const uint32_t *const vmask, const uint8_t (*l)[4], ptrdiff_t b4_stride,
/// const Av1FilterLUT *lut, const int w)`
pub unsafe extern "C" fn dav1d_lpf_v_sb_y_8bpc_neon(
    dst: Dst,
    stride: isize,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: isize,
    lut: *const Av1FilterLUT,
    _w: c_int,
) {
    lpf_func::<true, true>(dst, stride, vmask, l, b4_stride, lut)
}

/// `dav1d_lpf_h_sb_y_8bpc_neon`
pub unsafe extern "C" fn dav1d_lpf_h_sb_y_8bpc_neon(
    dst: Dst,
    stride: isize,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: isize,
    lut: *const Av1FilterLUT,
    _w: c_int,
) {
    lpf_func::<false, true>(dst, stride, vmask, l, b4_stride, lut)
}

/// `dav1d_lpf_v_sb_uv_8bpc_neon`
pub unsafe extern "C" fn dav1d_lpf_v_sb_uv_8bpc_neon(
    dst: Dst,
    stride: isize,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: isize,
    lut: *const Av1FilterLUT,
    _w: c_int,
) {
    lpf_func::<true, false>(dst, stride, vmask, l, b4_stride, lut)
}

/// `dav1d_lpf_h_sb_uv_8bpc_neon`
pub unsafe extern "C" fn dav1d_lpf_h_sb_uv_8bpc_neon(
    dst: Dst,
    stride: isize,
    vmask: *const u32,
    l: *const [u8; 4],
    b4_stride: isize,
    lut: *const Av1FilterLUT,
    _w: c_int,
) {
    lpf_func::<false, false>(dst, stride, vmask, l, b4_stride, lut)
}
