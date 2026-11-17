// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// Copyright © 2020, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/cdef.S` (8 bpc) and the shared
//! `cdef_tmpl.S` (both bit depths) with NEON intrinsics (engine ADR-0044
//! D2, libavif-rs ADR 0062).
//!
//! From `cdef.S`: the padding entry points `dav1d_cdef_padding{8,4}_8bpc_neon`
//! (macro `padding_func`, with `pad_top_bottom` and `load_n_incr`), their
//! fully edged variants `cdef_padding{8,4}_edged_8bpc_neon`
//! (`padding_func_edged`), and the fully edged filters
//! `cdef_filter{8,4}{_pri,_sec,_pri_sec}_edged_8bpc_neon` (`filter_func_8`
//! with `load_px_8` and `handle_pixel_8`), which work on an 8-bit `tmp`.
//! From `cdef_tmpl.S`: the tables (`directions8`, `directions4`,
//! `pri_taps`, `div_table`, `alt_fact`), the filters `filter_func` (with
//! `load_px` and `handle_pixel`) and their dispatcher `filter`, and
//! `find_dir` (with `cost_alt`, `find_best`, `dir_load_step{1,2,3}`). The
//! template's macros are generic functions here with the bit depth as the
//! const parameter `BPC`; each assembly instance is a named function that
//! instantiates them (the 8 bpc instances in this file, the 16 bpc ones in
//! `cdef16.rs`).
//!
//! Correspondence with the assembly:
//! - The vector registers are the array `v` (`V` of `util.rs`,
//!   `int16x8_t` by register number); byte instructions (`.16b`, `.8b`)
//!   and word instructions (`.4s`) reinterpret the register. A 64-bit
//!   arrangement and a scalar `ldr`/`addv` clear the rest of the register.
//! - General registers keep their assembly names (`x0`, `w7`, …);
//!   post-incremented addresses are explicit offsets after the access.
//!   Pointers are byte pointers, as the assembly's address arithmetic.
//! - A branch to another function (`b.eq cdef_padding8_edged_8bpc_neon`,
//!   `b cdef_filter8_sec_8bpc_neon`) is a call followed by `return`; the
//!   branch target of a template instance is passed by the instance as a
//!   parameter (`edged`), which is constant after inlining.
//! - The `.irpc i` body of `find_dir` is `find_dir_row::<I>`; the
//!   immediates of its `ext` instructions are folded from `I`.
//! - The stack slots of `find_dir` (`cost[8]`) are a local array; the save
//!   of `d8` at 16 bpc is the compiler's.
//!
//! Deviations: none that changes the instruction count of the arithmetic
//! or of the loads and stores. `mov w3, #8` of `find_dir` (dead: `w3` is
//! overwritten before it is read) has no counterpart.
//!
//! Safety (argued once for the family): the callers are the `DEFINE_FILTER`
//! wrappers of `simd/arm/cdef.rs` and the `dir` slot, called by the
//! translated `cdef_apply_tmpl` as the original calls them. The padding
//! functions get `tmp` = `tmp_buf + 2 * TS + 8` in a 16-byte aligned
//! `uint16_t tmp_buf[12 * TS + 8]` (`TS` = 16 for w 8, 8 for w 4) and
//! write its rows `-2..h+2`, columns `-2..w+2` (16-bit), or for the fully
//! edged 8 bpc case the bytes at `tmp` with `TS` bytes per row, rows
//! `-2..h+2`, columns `-2..w+2`; the largest store of a missing edge (two
//! full rows of `0x8000`) ends inside the buffer. They read `src` rows
//! `0..h`, columns `0..w` and the right columns `w..w+2` with
//! `CDEF_HAVE_RIGHT`; `left[0..h]` with `CDEF_HAVE_LEFT`; the rows `top`,
//! `top + stride` with `CDEF_HAVE_TOP` and `bottom`, `bottom + stride`
//! with `CDEF_HAVE_BOTTOM`, columns `-2..w+2` as the flags allow, which
//! the original's contract (`cdef_apply_tmpl`, the C `padding`) makes
//! readable. The filters read `tmp` at the block's pixels and at the
//! direction offsets of at most two rows and two columns around them,
//! all written by the padding, and write `dst` rows `0..h`, columns
//! `0..w`. `find_dir` reads 8 rows of 8 pixels of `img` and writes `*var`.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::{c_int, c_uint};
use core::ptr::{read_unaligned, write_unaligned};

use super::util::{d, lo, regs, srshr, V};

pub(super) type P = *mut u8;

/// A fully edged 8 bpc filter (`cdef_filter\w\suffix\()_edged_8bpc_neon`).
pub(super) type EdgedFn = unsafe fn(P, isize, *const u8, c_int, c_int, c_int, c_int, c_int);

// --- instructions ---------------------------------------------------------------
// SAFETY of the value intrinsics below: NEON is baseline on AArch64.

#[inline(always)]
fn uu(a: int16x8_t) -> uint16x8_t {
    unsafe { vreinterpretq_u16_s16(a) }
}
#[inline(always)]
fn ss(a: uint16x8_t) -> int16x8_t {
    unsafe { vreinterpretq_s16_u16(a) }
}
#[inline(always)]
fn b(a: int16x8_t) -> uint8x16_t {
    unsafe { vreinterpretq_u8_s16(a) }
}
#[inline(always)]
fn hb(a: uint8x16_t) -> int16x8_t {
    unsafe { vreinterpretq_s16_u8(a) }
}
#[inline(always)]
fn w(a: int16x8_t) -> int32x4_t {
    unsafe { vreinterpretq_s32_s16(a) }
}
#[inline(always)]
fn hw(a: int32x4_t) -> int16x8_t {
    unsafe { vreinterpretq_s16_s32(a) }
}
/// `uxtl vd.8h, vn.8b`
#[inline(always)]
pub(super) fn uxtl(a: int16x8_t) -> int16x8_t {
    unsafe { ss(vmovl_u8(vget_low_u8(b(a)))) }
}
/// `uabd vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn uabd(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vabdq_u16(uu(a), uu(c))) }
}
/// `ushl vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn ushl(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vshlq_u16(uu(a), c)) }
}
/// `uqsub vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn uqsub(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vqsubq_u16(uu(a), uu(c))) }
}
/// `umin vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn umin(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { ss(vminq_u16(uu(a), uu(c))) }
}
#[inline(always)]
fn smin(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vminq_s16(a, c) }
}
#[inline(always)]
fn smax(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vmaxq_s16(a, c) }
}
#[inline(always)]
fn add(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vaddq_s16(a, c) }
}
/// `add vd.4h, vn.4h, vm.4h`
#[inline(always)]
fn add4h(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { d(vadd_s16(lo(a), lo(c))) }
}
#[inline(always)]
fn sub(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vsubq_s16(a, c) }
}
#[inline(always)]
fn neg(a: int16x8_t) -> int16x8_t {
    unsafe { vnegq_s16(a) }
}
/// `mla vd.8h, vn.8h, vm.8h`
#[inline(always)]
fn mla(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vmlaq_s16(acc, a, c) }
}
/// `cmlt vd.8h, vn.8h, #0`
#[inline(always)]
fn cmlt0(a: int16x8_t) -> int16x8_t {
    unsafe { ss(vcltzq_s16(a)) }
}
/// `xtn vd.8b, vn.8h` (clears the high half)
#[inline(always)]
fn xtn(a: int16x8_t) -> int16x8_t {
    unsafe { hb(vcombine_u8(vmovn_u16(uu(a)), vdup_n_u8(0))) }
}
/// `ext vd.16b, vn.16b, vm.16b, #n` (`n` even, 2..=14, constant after inlining)
#[inline(always)]
fn ext(a: int16x8_t, c: int16x8_t, n: usize) -> int16x8_t {
    unsafe {
        let (a, c) = (b(a), b(c));
        hb(match n {
            2 => vextq_u8::<2>(a, c),
            4 => vextq_u8::<4>(a, c),
            6 => vextq_u8::<6>(a, c),
            8 => vextq_u8::<8>(a, c),
            10 => vextq_u8::<10>(a, c),
            12 => vextq_u8::<12>(a, c),
            14 => vextq_u8::<14>(a, c),
            _ => unreachable!(),
        })
    }
}
/// `ins vd.h[i], vn.h[0]` (`i` constant after inlining)
#[inline(always)]
fn ins_h(a: int16x8_t, i: usize, c: int16x8_t) -> int16x8_t {
    unsafe {
        match i {
            0 => vcopyq_laneq_s16::<0, 0>(a, c),
            1 => vcopyq_laneq_s16::<1, 0>(a, c),
            2 => vcopyq_laneq_s16::<2, 0>(a, c),
            3 => vcopyq_laneq_s16::<3, 0>(a, c),
            4 => vcopyq_laneq_s16::<4, 0>(a, c),
            5 => vcopyq_laneq_s16::<5, 0>(a, c),
            6 => vcopyq_laneq_s16::<6, 0>(a, c),
            7 => vcopyq_laneq_s16::<7, 0>(a, c),
            _ => unreachable!(),
        }
    }
}
/// `smull vd.4s, vn.4h, vm.4h`
#[inline(always)]
fn smull(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hw(vmull_s16(lo(a), lo(c))) }
}
/// `smull2 vd.4s, vn.8h, vm.8h`
#[inline(always)]
fn smull2(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hw(vmull_high_s16(a, c)) }
}
/// `smlal vd.4s, vn.4h, vm.4h`
#[inline(always)]
fn smlal(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hw(vmlal_s16(w(acc), lo(a), lo(c))) }
}
/// `smlal2 vd.4s, vn.8h, vm.8h`
#[inline(always)]
fn smlal2(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hw(vmlal_high_s16(w(acc), a, c)) }
}
/// `mul vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn mul4s(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hw(vmulq_s32(w(a), w(c))) }
}
/// `mla vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn mla4s(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hw(vmlaq_s32(w(acc), w(a), w(c))) }
}
/// `uxtl vd.4s, vn.4h`
#[inline(always)]
fn uxtl4s(a: int16x8_t) -> int16x8_t {
    unsafe { hw(vreinterpretq_s32_u32(vmovl_u16(vget_low_u16(uu(a))))) }
}
/// `uxtl2 vd.4s, vn.8h`
#[inline(always)]
fn uxtl2_4s(a: int16x8_t) -> int16x8_t {
    unsafe { hw(vreinterpretq_s32_u32(vmovl_high_u16(uu(a)))) }
}
/// `addv sd, vn.4s` (the rest of the register cleared)
#[inline(always)]
fn addv4s(a: int16x8_t) -> int16x8_t {
    unsafe { hw(vsetq_lane_s32::<0>(vaddvq_s32(w(a)), vdupq_n_s32(0))) }
}
/// `mov wd, vn.s[0]`
#[inline(always)]
fn s0(a: int16x8_t) -> u32 {
    unsafe { vgetq_lane_u32::<0>(vreinterpretq_u32_s16(a)) }
}

// --- byte instructions of the fully edged 8 bpc filter -----------------------------

/// `uabd vd.16b, vn.16b, vm.16b`
#[inline(always)]
fn uabd_b(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vabdq_u8(b(a), b(c))) }
}
/// `ushl vd.16b, vn.16b, vm.16b`
#[inline(always)]
fn ushl_b(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vshlq_u8(b(a), vreinterpretq_s8_s16(c))) }
}
/// `uqsub vd.16b, vn.16b, vm.16b`
#[inline(always)]
fn uqsub_b(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vqsubq_u8(b(a), b(c))) }
}
/// `cmhi vd.16b, vn.16b, vm.16b`
#[inline(always)]
fn cmhi_b(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vcgtq_u8(b(a), b(c))) }
}
#[inline(always)]
fn umin_b(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vminq_u8(b(a), b(c))) }
}
#[inline(always)]
fn umax_b(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vmaxq_u8(b(a), b(c))) }
}
#[inline(always)]
fn neg_b(a: int16x8_t) -> int16x8_t {
    unsafe { hb(vreinterpretq_u8_s8(vnegq_s8(vreinterpretq_s8_s16(a)))) }
}
/// `bsl vd.16b, vn.16b, vm.16b`: `vd = vd ? vn : vm` bitwise
#[inline(always)]
fn bsl(d: int16x8_t, n: int16x8_t, m: int16x8_t) -> int16x8_t {
    unsafe { vbslq_s16(uu(d), n, m) }
}
/// `mla vd.16b, vn.16b, vm.16b`
#[inline(always)]
fn mla_b(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { hb(vmlaq_u8(b(acc), b(a), b(c))) }
}
#[inline(always)]
fn dup_b(x: u32) -> int16x8_t {
    unsafe { hb(vdupq_n_u8(x as u8)) }
}
/// `vd.8b` result of a 64-bit byte instruction: the high half cleared.
#[inline(always)]
fn d8(x: uint8x8_t) -> int16x8_t {
    unsafe { hb(vcombine_u8(x, vdup_n_u8(0))) }
}
#[inline(always)]
fn lo8(a: int16x8_t) -> uint8x8_t {
    unsafe { vget_low_u8(b(a)) }
}

// --- memory ------------------------------------------------------------------------

/// `ldr hN/sN/dN/qN, [p]` (`n` = 2, 4, 8, 16 bytes; the rest cleared)
#[inline(always)]
pub(super) unsafe fn ldr(p: *const u8, n: usize) -> int16x8_t {
    match n {
        2 => ss(vsetq_lane_u16::<0>(read_unaligned(p.cast::<u16>()), vdupq_n_u16(0))),
        4 => vreinterpretq_s16_u32(vsetq_lane_u32::<0>(read_unaligned(p.cast::<u32>()), vdupq_n_u32(0))),
        8 => vreinterpretq_s16_u64(vsetq_lane_u64::<0>(read_unaligned(p.cast::<u64>()), vdupq_n_u64(0))),
        16 => hb(vld1q_u8(p)),
        _ => unreachable!(),
    }
}
/// `str hN/sN/dN/qN, [p]` and `stur` (`n` = 2, 4, 8, 16 bytes)
#[inline(always)]
pub(super) unsafe fn str_(p: P, n: usize, a: int16x8_t) {
    match n {
        2 => write_unaligned(p.cast::<u16>(), vgetq_lane_u16::<0>(uu(a))),
        4 => write_unaligned(p.cast::<u32>(), vgetq_lane_u32::<0>(vreinterpretq_u32_s16(a))),
        8 => write_unaligned(p.cast::<u64>(), vgetq_lane_u64::<0>(vreinterpretq_u64_s16(a))),
        16 => vst1q_u8(p, b(a)),
        _ => unreachable!(),
    }
}
/// `ld1 {vN.h}[0], [p]`
#[inline(always)]
unsafe fn ld1_h0(a: int16x8_t, p: *const u8) -> int16x8_t {
    ss(vsetq_lane_u16::<0>(read_unaligned(p.cast::<u16>()), uu(a)))
}
/// `ld1 {vN.s}[L], [p]`
#[inline(always)]
pub(super) unsafe fn ld1_s<const L: i32>(a: int16x8_t, p: *const u8) -> int16x8_t {
    let x = read_unaligned(p.cast::<u32>());
    vreinterpretq_s16_u32(vsetq_lane_u32::<L>(x, vreinterpretq_u32_s16(a)))
}
/// `ld1 {vN.d}[L], [p]`
#[inline(always)]
unsafe fn ld1_d<const L: i32>(a: int16x8_t, p: *const u8) -> int16x8_t {
    let x = read_unaligned(p.cast::<u64>());
    vreinterpretq_s16_u64(vsetq_lane_u64::<L>(x, vreinterpretq_u64_s16(a)))
}
/// `st1 {vN.s}[L], [p]`
#[inline(always)]
unsafe fn st1_s<const L: i32>(p: P, a: int16x8_t) {
    write_unaligned(p.cast::<u32>(), vgetq_lane_u32::<L>(vreinterpretq_u32_s16(a)))
}
/// `st1 {vN.d}[L], [p]`
#[inline(always)]
unsafe fn st1_d<const L: i32>(p: P, a: int16x8_t) {
    write_unaligned(p.cast::<u64>(), vgetq_lane_u64::<L>(vreinterpretq_u64_s16(a)))
}
/// `st1 {vA.8h, vB.8h}, [p]`
#[inline(always)]
pub(super) unsafe fn st1_x2(p: P, a: int16x8_t, c: int16x8_t) {
    vst1q_u8_x2(p, uint8x16x2_t(b(a), b(c)))
}
/// `st1 {vA.8b, vB.8b}, [p]`
#[inline(always)]
unsafe fn st1_8b_x2(p: P, a: int16x8_t, c: int16x8_t) {
    vst1_u8_x2(p, uint8x8x2_t(lo8(a), lo8(c)))
}

// --- tables (cdef_tmpl.S) ----------------------------------------------------------

/// The macro `dir_table w, stride`: `directions\w`.
macro_rules! dir_table {
    ($s:expr) => {
        [
            -1 * $s + 1, -2 * $s + 2, //
            0 * $s + 1, -1 * $s + 2, //
            0 * $s + 1, 0 * $s + 2, //
            0 * $s + 1, 1 * $s + 2, //
            1 * $s + 1, 2 * $s + 2, //
            1 * $s + 0, 2 * $s + 1, //
            1 * $s + 0, 2 * $s + 0, //
            1 * $s + 0, 2 * $s - 1, //
            // Repeated, to avoid & 7
            -1 * $s + 1, -2 * $s + 2, //
            0 * $s + 1, -1 * $s + 2, //
            0 * $s + 1, 0 * $s + 2, //
            0 * $s + 1, 1 * $s + 2, //
            1 * $s + 1, 2 * $s + 2, //
            1 * $s + 0, 2 * $s + 1, //
        ]
    };
}
/// `directions8` (`dir_table 8, 16`)
pub(super) static DIRECTIONS8: [i8; 28] = dir_table!(16);
/// `directions4` (`dir_table 4, 8`)
pub(super) static DIRECTIONS4: [i8; 28] = dir_table!(8);
/// `pri_taps`
pub(super) static PRI_TAPS: [u8; 4] = [4, 2, 3, 3];
/// `div_table`
static DIV_TABLE: [u16; 8] = [840, 420, 280, 210, 168, 140, 120, 105];
/// `alt_fact`
static ALT_FACT: [u16; 12] = [420, 210, 140, 105, 105, 105, 105, 105, 140, 210, 420, 0];

// --- padding (cdef.S) --------------------------------------------------------------

/// The macro `pad_top_bottom s1, s2, w, stride, rn, rw, ret`: the two rows
/// at `s1` and `s2` into `tmp` at `x0` (`rn` = `W` bytes, `rw` = `2 * W`
/// bytes). Returns `x0` after the rows; with `RET` the caller returns.
#[inline(always)]
unsafe fn pad_top_bottom<const W: usize, const STRIDE: usize, const RET: bool>(
    v: &mut V,
    mut x0: P,
    mut s1: *const u8,
    mut s2: *const u8,
    w7: c_uint,
) -> P {
    if w7 & 1 != 0 {
        // CDEF_HAVE_LEFT
        s1 = s1.sub(2);
        s2 = s2.sub(2);
        if w7 & 2 != 0 {
            // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
            v[0] = ldr(s1, W); // ldr rn0, [s1]
            v[1] = ldr(s1.add(W), 4); // ldr s1, [s1, #w]
            v[2] = ldr(s2, W);
            v[3] = ldr(s2.add(W), 4);
            v[0] = uxtl(v[0]);
            v[1] = uxtl(v[1]);
            v[2] = uxtl(v[2]);
            v[3] = uxtl(v[3]);
            str_(x0, 2 * W, v[0]);
            str_(x0.add(2 * W), 8, v[1]);
            x0 = x0.add(2 * STRIDE);
            str_(x0, 2 * W, v[2]);
            str_(x0.add(2 * W), 8, v[3]);
        } else {
            // 1: CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
            v[0] = ldr(s1, W);
            v[1] = ldr(s1.add(W), 2); // ldr h1, [s1, #w]
            v[2] = ldr(s2, W);
            v[3] = ldr(s2.add(W), 2);
            v[0] = uxtl(v[0]);
            v[1] = uxtl(v[1]);
            v[2] = uxtl(v[2]);
            v[3] = uxtl(v[3]);
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
        v[0] = ldr(s1, W);
        v[1] = ldr(s1.add(W), 2);
        v[2] = ldr(s2, W);
        v[3] = ldr(s2.add(W), 2);
        v[0] = uxtl(v[0]);
        v[1] = uxtl(v[1]);
        v[2] = uxtl(v[2]);
        v[3] = uxtl(v[3]);
        str_(x0, 4, v[31]);
        str_(x0.add(4), 2 * W, v[0]); // stur
        str_(x0.add(4 + 2 * W), 4, v[1]);
        x0 = x0.add(2 * STRIDE);
        str_(x0, 4, v[31]);
        str_(x0.add(4), 2 * W, v[2]);
        str_(x0.add(4 + 2 * W), 4, v[3]);
    } else {
        // 1: !CDEF_HAVE_LEFT+!CDEF_HAVE_RIGHT
        v[0] = ldr(s1, W);
        v[1] = ldr(s2, W);
        v[0] = uxtl(v[0]);
        v[1] = uxtl(v[1]);
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

/// The macro `load_n_incr dst, src, incr, w`: `ld1 {dst.s}[0]` (w 4) or
/// `ld1 {dst.8b}` (w 8), then `src += incr`.
#[inline(always)]
unsafe fn load_n_incr<const W: usize>(dst: int16x8_t, src: &mut *const u8, incr: isize) -> int16x8_t {
    let r = if W == 4 { ld1_s::<0>(dst, *src) } else { ldr(*src, 8) };
    *src = src.offset(incr);
    r
}

/// The macro `padding_func w, stride, rn, rw`: `dav1d_cdef_padding\w\()_8bpc_neon`.
#[inline(always)]
unsafe fn padding_func<const W: usize, const STRIDE: usize>(
    tmp: *mut u16,
    src: *const u8,
    src_stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    h: c_int,
    edges: c_uint,
    edged: unsafe fn(*mut u16, *const u8, isize, *const [u8; 2], *const u8, *const u8, c_int),
) {
    let (mut x1, x2, mut x3, x4, x5, mut w6, w7) = (src, src_stride, left as *const u8, top, bottom, h, edges);
    if w7 == 0xf {
        // fully edged
        return edged(tmp, src, src_stride, left, top, bottom, h);
    }
    let mut v = regs();
    v[30] = ss(vdupq_n_u16(0x8000)); // movi v30.8h, #0x80, lsl #8
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
        x0 = pad_top_bottom::<W, STRIDE, false>(&mut v, x0, x4, x9, w7);
    }

    // 3: Middle section
    if w7 & 1 != 0 {
        // CDEF_HAVE_LEFT
        if w7 & 2 != 0 {
            // CDEF_HAVE_LEFT+CDEF_HAVE_RIGHT
            loop {
                v[0] = ld1_h0(v[0], x3);
                x3 = x3.add(2);
                v[2] = ldr(x1.add(W), 2);
                v[1] = load_n_incr::<W>(v[1], &mut x1, x2);
                w6 -= 1;
                v[0] = uxtl(v[0]);
                v[1] = uxtl(v[1]);
                v[2] = uxtl(v[2]);
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
                v[0] = ld1_h0(v[0], x3);
                x3 = x3.add(2);
                v[1] = load_n_incr::<W>(v[1], &mut x1, x2);
                w6 -= 1;
                v[0] = uxtl(v[0]);
                v[1] = uxtl(v[1]);
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
            v[1] = ldr(x1.add(W), 2);
            v[0] = load_n_incr::<W>(v[0], &mut x1, x2);
            w6 -= 1;
            v[0] = uxtl(v[0]);
            v[1] = uxtl(v[1]);
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
            v[0] = load_n_incr::<W>(v[0], &mut x1, x2);
            w6 -= 1;
            v[0] = uxtl(v[0]);
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
    pad_top_bottom::<W, STRIDE, true>(&mut v, x0, x5, x9, w7);
}

/// `void dav1d_cdef_padding8_8bpc_neon(uint16_t *tmp, const pixel *src,
/// ptrdiff_t src_stride, const pixel (*left)[2], const pixel *const top,
/// const pixel *const bottom, int h, enum CdefEdgeFlags edges)`
pub unsafe extern "C" fn dav1d_cdef_padding8_8bpc_neon(
    tmp: *mut u16,
    src: *const u8,
    src_stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    h: c_int,
    edges: c_uint,
) {
    padding_func::<8, 16>(tmp, src, src_stride, left, top, bottom, h, edges, cdef_padding8_edged_8bpc_neon)
}

/// `dav1d_cdef_padding4_8bpc_neon`
pub unsafe extern "C" fn dav1d_cdef_padding4_8bpc_neon(
    tmp: *mut u16,
    src: *const u8,
    src_stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    h: c_int,
    edges: c_uint,
) {
    padding_func::<4, 8>(tmp, src, src_stride, left, top, bottom, h, edges, cdef_padding4_edged_8bpc_neon)
}

/// The macro `padding_func_edged w, stride, reg`: an 8-bit `tmp` with
/// `STRIDE` bytes per row (`reg` = `W` bytes).
#[inline(always)]
unsafe fn padding_func_edged<const W: usize, const STRIDE: usize>(
    tmp: *mut u16,
    src: *const u8,
    src_stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    h: c_int,
) {
    let (mut x1, x2, mut x3, mut w6) = (src, src_stride, left as *const u8, h);
    let mut v = regs();
    let x4 = top.sub(2);
    let x5 = bottom.sub(2);
    let mut x0 = (tmp as P).sub(2 * STRIDE + 2);

    if W == 4 {
        v[0] = ldr(x4, 8);
        v[1] = ldr(x4.offset(x2), 8);
        st1_8b_x2(x0, v[0], v[1]);
        x0 = x0.add(16);
    } else {
        let x9 = x4.offset(x2);
        v[0] = ldr(x4, 8);
        v[1] = ldr(x4.add(8), 4);
        v[2] = ldr(x9, 8);
        v[3] = ldr(x9.add(8), 4);
        str_(x0, 8, v[0]);
        str_(x0.add(8), 4, v[1]);
        str_(x0.add(STRIDE), 8, v[2]);
        str_(x0.add(STRIDE + 8), 4, v[3]);
        x0 = x0.add(2 * STRIDE);
    }

    // 0:
    loop {
        v[0] = ld1_h0(v[0], x3);
        x3 = x3.add(2);
        v[2] = ldr(x1.add(W), 2);
        v[1] = load_n_incr::<W>(v[1], &mut x1, x2);
        w6 -= 1;
        str_(x0, 2, v[0]);
        str_(x0.add(2), W, v[1]); // stur reg1, [x0, #2]
        str_(x0.add(2 + W), 2, v[2]);
        x0 = x0.add(STRIDE);
        if w6 <= 0 {
            break;
        }
    }

    if W == 4 {
        v[0] = ldr(x5, 8);
        v[1] = ldr(x5.offset(x2), 8);
        st1_8b_x2(x0, v[0], v[1]);
    } else {
        let x9 = x5.offset(x2);
        v[0] = ldr(x5, 8);
        v[1] = ldr(x5.add(8), 4);
        v[2] = ldr(x9, 8);
        v[3] = ldr(x9.add(8), 4);
        str_(x0, 8, v[0]);
        str_(x0.add(8), 4, v[1]);
        str_(x0.add(STRIDE), 8, v[2]);
        str_(x0.add(STRIDE + 8), 4, v[3]);
    }
}

/// `cdef_padding8_edged_8bpc_neon`
unsafe fn cdef_padding8_edged_8bpc_neon(
    tmp: *mut u16,
    src: *const u8,
    src_stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    h: c_int,
) {
    padding_func_edged::<8, 16>(tmp, src, src_stride, left, top, bottom, h)
}

/// `cdef_padding4_edged_8bpc_neon`
unsafe fn cdef_padding4_edged_8bpc_neon(
    tmp: *mut u16,
    src: *const u8,
    src_stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bottom: *const u8,
    h: c_int,
) {
    padding_func_edged::<4, 8>(tmp, src, src_stride, left, top, bottom, h)
}

// --- filter (cdef_tmpl.S) ----------------------------------------------------------

/// The macro `load_px d1, d2, w`: the pixels at `x2 ± off` (`w9`).
#[inline(always)]
unsafe fn load_px<const W: usize>(v: &mut V, d1: usize, d2: usize, x2: *const u8, w9: u8) {
    let off = (w9 as i8 as isize) << 1;
    let mut x6 = x2.offset(off); // x + off
    let mut x9 = x2.offset(-off); // x - off
    if W == 8 {
        v[d1] = ldr(x6, 16); // p0
        v[d2] = ldr(x9, 16); // p1
    } else {
        v[d1] = ldr(x6, 8); // ld1 {d1.4h}, [x6]: p0
        x6 = x6.add(2 * 8); // += stride
        v[d2] = ldr(x9, 8); // p1
        x9 = x9.add(2 * 8); // += stride
        v[d1] = ld1_d::<1>(v[d1], x6); // p0
        v[d2] = ld1_d::<1>(v[d2], x9); // p1
    }
}

/// The macro `handle_pixel s1, s2, thresh_vec, shift, tap, min`.
#[inline(always)]
fn handle_pixel<const MIN: bool>(v: &mut V, s1: usize, s2: usize, thresh: usize, shift: usize, tap: u32) {
    // SAFETY: NEON value intrinsics only.
    unsafe {
        if MIN {
            v[2] = umin(v[2], v[s1]);
            v[3] = smax(v[3], v[s1]);
            v[2] = umin(v[2], v[s2]);
            v[3] = smax(v[3], v[s2]);
        }
        v[16] = uabd(v[0], v[s1]); // abs(diff)
        v[20] = uabd(v[0], v[s2]); // abs(diff)
        v[17] = ushl(v[16], v[shift]); // abs(diff) >> shift
        v[21] = ushl(v[20], v[shift]); // abs(diff) >> shift
        v[17] = uqsub(v[thresh], v[17]); // clip = imax(0, threshold - (abs(diff) >> shift))
        v[21] = uqsub(v[thresh], v[21]); // clip = imax(0, threshold - (abs(diff) >> shift))
        v[18] = sub(v[s1], v[0]); // diff = p0 - px
        v[22] = sub(v[s2], v[0]); // diff = p1 - px
        v[16] = neg(v[17]); // -clip
        v[20] = neg(v[21]); // -clip
        v[18] = smin(v[18], v[17]); // imin(diff, clip)
        v[22] = smin(v[22], v[21]); // imin(diff, clip)
        v[19] = vdupq_n_s16(tap as i16); // taps[k]
        v[18] = smax(v[18], v[16]); // constrain() = imax(imin(diff, clip), -clip)
        v[22] = smax(v[22], v[20]); // constrain() = imax(imin(diff, clip), -clip)
        v[1] = mla(v[1], v[18], v[19]); // sum += taps[k] * constrain()
        v[1] = mla(v[1], v[22], v[19]); // sum += taps[k] * constrain()
    }
}

/// The macro `filter_func w, bpc, pri, sec, min, suffix`:
/// `cdef_filter\w\suffix\()_\bpc\()bpc_neon`. `dst` and `tmp` are byte
/// pointers; `edged` is the 8 bpc branch target for fully edged blocks.
#[inline(always)]
pub(super) unsafe fn filter_func<const W: usize, const BPC: u32, const PRI: bool, const SEC: bool, const MIN: bool>(
    dst: P,
    dst_stride: isize,
    tmp: *const u8,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
    bitdepth_max: c_int,
    edged: Option<EdgedFn>,
) {
    let (mut x0, x1, mut x2) = (dst, dst_stride, tmp);
    let (w3, w4, w5, w6, mut w7) = (pri_strength as u32, sec_strength as u32, dir as u32, damping as u32, h);
    if BPC == 8 {
        let w8 = edges as u32; // ldr w8, [sp]: edges
        if w8 == 0xf {
            if let Some(f) = edged {
                return f(dst, dst_stride, tmp, pri_strength, sec_strength, dir, damping, h);
            }
        }
    }
    let mut v = regs();
    let mut x8: *const u8 = core::ptr::null();
    if PRI {
        let mut w9 = 0u32;
        if BPC == 16 {
            w9 = (bitdepth_max as u32).leading_zeros(); // clz w9, w9 (bitdepth_max)
            w9 = w9.wrapping_sub(24); // -bitdepth_min_8
            w9 = w9.wrapping_neg(); // bitdepth_min_8
        }
        x8 = PRI_TAPS.as_ptr();
        if BPC == 16 {
            w9 = w3.wrapping_shr(w9); // pri_strength >> bitdepth_min_8
            w9 &= 1; // (pri_strength >> bitdepth_min_8) & 1
        } else {
            w9 = w3 & 1;
        }
        x8 = x8.add((w9 as usize) << 1);
    }
    let x9 = if W == 8 { DIRECTIONS8.as_ptr() } else { DIRECTIONS4.as_ptr() };
    let mut x5 = (x9 as *const u8).add((w5 as usize) << 1);
    v[30] = d(vdup_n_s16(15)); // movi v30.4h, #15
    v[28] = d(vdup_n_s16(w6 as i16)); // damping

    if PRI {
        v[25] = vdupq_n_s16(w3 as i16); // threshold
    }
    if SEC {
        v[27] = vdupq_n_s16(w4 as i16); // threshold
    }
    v[24] = d(vtrn1_s16(lo(v[25]), lo(v[27])));
    v[24] = d(vclz_s16(lo(v[24]))); // clz(threshold)
    v[24] = d(vsub_s16(lo(v[30]), lo(v[24]))); // ulog2(threshold)
    v[24] = d(vreinterpret_s16_u16(vqsub_u16(vreinterpret_u16_s16(lo(v[28])), vreinterpret_u16_s16(lo(v[24]))))); // shift = imax(0, damping - ulog2(threshold))
    v[24] = d(vneg_s16(lo(v[24]))); // -shift
    if SEC {
        v[26] = vdupq_laneq_s16::<1>(v[24]);
    }
    if PRI {
        v[24] = vdupq_laneq_s16::<0>(v[24]);
    }

    loop {
        // 1:
        if W == 8 {
            v[0] = ldr(x2, 16); // px
        } else {
            let x12 = x2.add(2 * 8);
            v[0] = ldr(x2, 8); // ld1 {v0.4h}, [x2]: px
            v[0] = ld1_d::<1>(v[0], x12); // px
        }

        v[1] = vdupq_n_s16(0); // sum
        if MIN {
            v[2] = v[0]; // min
            v[3] = v[0]; // max
        }

        // Instead of loading sec_taps 2, 1 from memory, just set it
        // to 2 initially and decrease for the second round.
        // This is also used as loop counter.
        let mut w11: u32 = 2; // sec_taps[0]

        loop {
            // 2:
            if PRI {
                let w9 = *x5; // off1
                load_px::<W>(&mut v, 4, 5, x2, w9);
            }
            if SEC {
                x5 = x5.add(4); // +2*2
                let w9 = *x5; // off2
                load_px::<W>(&mut v, 6, 7, x2, w9);
            }
            if PRI {
                let w10 = *x8 as u32; // *pri_taps
                handle_pixel::<MIN>(&mut v, 4, 5, 25, 24, w10);
            }
            if SEC {
                x5 = x5.add(8); // +2*4
                let w9 = *x5; // off3
                load_px::<W>(&mut v, 4, 5, x2, w9);

                handle_pixel::<MIN>(&mut v, 6, 7, 27, 26, w11);

                handle_pixel::<MIN>(&mut v, 4, 5, 27, 26, w11);

                x5 = x5.sub(11); // x5 -= 2*(2+4); x5 += 1;
            } else {
                x5 = x5.add(1); // x5 += 1
            }
            w11 -= 1; // sec_tap-- (value)
            if PRI {
                x8 = x8.add(1); // pri_taps++ (pointer)
            }
            if w11 == 0 {
                break;
            }
        }

        v[4] = cmlt0(v[1]); // -(sum < 0)
        v[1] = add(v[1], v[4]); // sum - (sum < 0)
        v[1] = srshr::<4>(v[1]); // (8 + sum - (sum < 0)) >> 4
        v[0] = add(v[0], v[1]); // px + (8 + sum ...) >> 4
        if MIN {
            v[0] = smin(v[0], v[3]);
            v[0] = smax(v[0], v[2]); // iclip(px + .., min, max)
        }
        if BPC == 8 {
            v[0] = xtn(v[0]);
        }
        if W == 8 {
            x2 = x2.add(2 * 16); // tmp += tmp_stride
            w7 -= 1; // h--
            if BPC == 8 {
                str_(x0, 8, v[0]); // st1 {v0.8b}, [x0], x1
            } else {
                str_(x0, 16, v[0]); // st1 {v0.8h}, [x0], x1
            }
            x0 = x0.offset(x1);
        } else {
            if BPC == 8 {
                st1_s::<0>(x0, v[0]);
            } else {
                st1_d::<0>(x0, v[0]);
            }
            x0 = x0.offset(x1);
            x2 = x2.add(2 * 16); // tmp += 2*tmp_stride
            w7 -= 2; // h -= 2
            if BPC == 8 {
                st1_s::<1>(x0, v[0]);
            } else {
                st1_d::<1>(x0, v[0]);
            }
            x0 = x0.offset(x1);
        }

        // Reset pri_taps and directions back to the original point
        x5 = x5.sub(2);
        if PRI {
            x8 = x8.sub(2);
        }

        if w7 <= 0 {
            break;
        }
    }
}

// --- fully edged 8 bpc filter (cdef.S) ----------------------------------------------

/// The macro `load_px_8 d1, d2, w`: the 8-bit pixels at `x2 ± off` (`w9`).
#[inline(always)]
unsafe fn load_px_8<const W: usize>(v: &mut V, d1: usize, d2: usize, x2: *const u8, w9: u8) {
    let off = w9 as i8 as isize;
    let mut x6 = x2.offset(off); // x + off
    let mut x9 = x2.offset(-off); // x - off
    if W == 8 {
        v[d1] = ld1_d::<0>(v[d1], x6); // p0
        x6 = x6.add(16); // += stride
        v[d2] = ld1_d::<0>(v[d2], x9); // p1
        x9 = x9.add(16); // += stride
        v[d1] = ld1_d::<1>(v[d1], x6); // p0
        v[d2] = ld1_d::<1>(v[d2], x9); // p0
    } else {
        v[d1] = ld1_s::<0>(v[d1], x6); // p0
        x6 = x6.add(8); // += stride
        v[d2] = ld1_s::<0>(v[d2], x9); // p1
        x9 = x9.add(8); // += stride
        v[d1] = ld1_s::<1>(v[d1], x6); // p0
        x6 = x6.add(8); // += stride
        v[d2] = ld1_s::<1>(v[d2], x9); // p1
        x9 = x9.add(8); // += stride
        v[d1] = ld1_s::<2>(v[d1], x6); // p0
        x6 = x6.add(8); // += stride
        v[d2] = ld1_s::<2>(v[d2], x9); // p1
        x9 = x9.add(8); // += stride
        v[d1] = ld1_s::<3>(v[d1], x6); // p0
        v[d2] = ld1_s::<3>(v[d2], x9); // p1
    }
}

/// The macro `handle_pixel_8 s1, s2, thresh_vec, shift, tap, min`.
#[inline(always)]
fn handle_pixel_8<const MIN: bool>(v: &mut V, s1: usize, s2: usize, thresh: usize, shift: usize, tap: u32) {
    if MIN {
        v[3] = umin_b(v[3], v[s1]);
        v[4] = umax_b(v[4], v[s1]);
        v[3] = umin_b(v[3], v[s2]);
        v[4] = umax_b(v[4], v[s2]);
    }
    v[16] = uabd_b(v[0], v[s1]); // abs(diff)
    v[20] = uabd_b(v[0], v[s2]); // abs(diff)
    v[17] = ushl_b(v[16], v[shift]); // abs(diff) >> shift
    v[21] = ushl_b(v[20], v[shift]); // abs(diff) >> shift
    v[17] = uqsub_b(v[thresh], v[17]); // clip = imax(0, threshold - (abs(diff) >> shift))
    v[21] = uqsub_b(v[thresh], v[21]); // clip = imax(0, threshold - (abs(diff) >> shift))
    v[18] = cmhi_b(v[0], v[s1]); // px > p0
    v[22] = cmhi_b(v[0], v[s2]); // px > p1
    v[17] = umin_b(v[17], v[16]); // imin(abs(diff), clip)
    v[21] = umin_b(v[21], v[20]); // imin(abs(diff), clip)
    v[19] = dup_b(tap); // taps[k]
    v[16] = neg_b(v[17]); // -imin()
    v[20] = neg_b(v[21]); // -imin()
    v[18] = bsl(v[18], v[16], v[17]); // constrain() = apply_sign()
    v[22] = bsl(v[22], v[20], v[21]); // constrain() = apply_sign()
    v[1] = mla_b(v[1], v[18], v[19]); // sum += taps[k] * constrain()
    v[2] = mla_b(v[2], v[22], v[19]); // sum += taps[k] * constrain()
}

/// The macro `filter_func_8 w, pri, sec, min, suffix`:
/// `cdef_filter\w\suffix\()_edged_8bpc_neon`, on the 8-bit `tmp` of
/// `padding_func_edged`.
#[inline(always)]
unsafe fn filter_func_8<const W: usize, const PRI: bool, const SEC: bool, const MIN: bool>(
    dst: P,
    dst_stride: isize,
    tmp: *const u8,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
) {
    let (mut x0, x1, mut x2) = (dst, dst_stride, tmp);
    let (w3, w4, w5, w6, mut w7) = (pri_strength as u32, sec_strength as u32, dir as u32, damping as u32, h);
    let mut v = regs();
    let mut x8: *const u8 = core::ptr::null();
    if PRI {
        x8 = PRI_TAPS.as_ptr();
        let w9 = w3 & 1;
        x8 = x8.add((w9 as usize) << 1);
    }
    let x9 = if W == 8 { DIRECTIONS8.as_ptr() } else { DIRECTIONS4.as_ptr() };
    let mut x5 = (x9 as *const u8).add((w5 as usize) << 1);
    v[30] = d8(vdup_n_u8(7)); // movi v30.8b, #7
    v[28] = d8(vdup_n_u8(w6 as u8)); // damping

    if PRI {
        v[25] = dup_b(w3); // threshold
    }
    if SEC {
        v[27] = dup_b(w4); // threshold
    }
    v[24] = d8(vtrn1_u8(lo8(v[25]), lo8(v[27])));
    v[24] = d8(vclz_u8(lo8(v[24]))); // clz(threshold)
    v[24] = d8(vsub_u8(lo8(v[30]), lo8(v[24]))); // ulog2(threshold)
    v[24] = d8(vqsub_u8(lo8(v[28]), lo8(v[24]))); // shift = imax(0, damping - ulog2(threshold))
    v[24] = d8(vreinterpret_u8_s8(vneg_s8(vreinterpret_s8_u8(lo8(v[24]))))); // -shift
    if SEC {
        v[26] = hb(vdupq_laneq_u8::<1>(b(v[24])));
    }
    if PRI {
        v[24] = hb(vdupq_laneq_u8::<0>(b(v[24])));
    }

    loop {
        // 1:
        if W == 8 {
            let x12 = x2.add(16);
            v[0] = ld1_d::<0>(v[0], x2); // px
            v[0] = ld1_d::<1>(v[0], x12); // px
        } else {
            let x12 = x2.add(1 * 8);
            let x13 = x2.add(2 * 8);
            let x14 = x2.add(3 * 8);
            v[0] = ld1_s::<0>(v[0], x2); // px
            v[0] = ld1_s::<1>(v[0], x12); // px
            v[0] = ld1_s::<2>(v[0], x13); // px
            v[0] = ld1_s::<3>(v[0], x14); // px
        }

        // We need 9-bits or two 8-bit accululators to fit the sum.
        // Max of |sum| > 15*2*6(pri) + 4*4*3(sec) = 228.
        // Start sum at -1 instead of 0 to help handle rounding later.
        v[1] = dup_b(255); // sum
        v[2] = dup_b(0); // sum
        if MIN {
            v[3] = v[0]; // min
            v[4] = v[0]; // max
        }

        // Instead of loading sec_taps 2, 1 from memory, just set it
        // to 2 initially and decrease for the second round.
        // This is also used as loop counter.
        let mut w11: u32 = 2; // sec_taps[0]

        loop {
            // 2:
            if PRI {
                let w9 = *x5; // off1
                load_px_8::<W>(&mut v, 5, 6, x2, w9);
            }
            if SEC {
                x5 = x5.add(4); // +2*2
                let w9 = *x5; // off2
                load_px_8::<W>(&mut v, 28, 29, x2, w9);
            }
            if PRI {
                let w10 = *x8 as u32; // *pri_taps
                handle_pixel_8::<MIN>(&mut v, 5, 6, 25, 24, w10);
            }
            if SEC {
                x5 = x5.add(8); // +2*4
                let w9 = *x5; // off3
                load_px_8::<W>(&mut v, 5, 6, x2, w9);

                handle_pixel_8::<MIN>(&mut v, 28, 29, 27, 26, w11);

                handle_pixel_8::<MIN>(&mut v, 5, 6, 27, 26, w11);

                x5 = x5.sub(11); // x5 -= 2*(2+4); x5 += 1;
            } else {
                x5 = x5.add(1); // x5 += 1
            }
            w11 -= 1; // sec_tap-- (value)
            if PRI {
                x8 = x8.add(1); // pri_taps++ (pointer)
            }
            if w11 == 0 {
                break;
            }
        }

        // Perform halving adds since the value won't fit otherwise.
        // To handle the offset for negative values, use both halving w/ and w/o rounding.
        let (s1, s2) = (vreinterpretq_s8_s16(v[1]), vreinterpretq_s8_s16(v[2]));
        v[5] = vreinterpretq_s16_s8(vrhaddq_s8(s1, s2)); // sum >> 1
        v[6] = vreinterpretq_s16_s8(vhaddq_s8(s1, s2)); // (sum - 1) >> 1
        v[1] = hb(vcltzq_s8(vreinterpretq_s8_s16(v[5]))); // sum < 0
        v[1] = bsl(v[1], v[6], v[5]); // (sum - (sum < 0)) >> 1

        v[1] = vreinterpretq_s16_s8(vrshrq_n_s8::<3>(vreinterpretq_s8_s16(v[1]))); // (8 + sum - (sum < 0)) >> 4

        v[0] = hb(vsqaddq_u8(b(v[0]), vreinterpretq_s8_s16(v[1]))); // px + (8 + sum ...) >> 4
        if MIN {
            v[0] = umin_b(v[0], v[4]);
            v[0] = umax_b(v[0], v[3]); // iclip(px + .., min, max)
        }
        if W == 8 {
            st1_d::<0>(x0, v[0]);
            x0 = x0.offset(x1);
            x2 = x2.add(2 * 16); // tmp += 2*tmp_stride
            w7 -= 2; // h -= 2
            st1_d::<1>(x0, v[0]);
            x0 = x0.offset(x1);
        } else {
            st1_s::<0>(x0, v[0]);
            x0 = x0.offset(x1);
            x2 = x2.add(4 * 8); // tmp += 4*tmp_stride
            st1_s::<1>(x0, v[0]);
            x0 = x0.offset(x1);
            w7 -= 4; // h -= 4
            st1_s::<2>(x0, v[0]);
            x0 = x0.offset(x1);
            st1_s::<3>(x0, v[0]);
            x0 = x0.offset(x1);
        }

        // Reset pri_taps and directions back to the original point
        x5 = x5.sub(2);
        if PRI {
            x8 = x8.sub(2);
        }

        if w7 <= 0 {
            break;
        }
    }
}

macro_rules! edged_instance {
    ($name:ident, $w:literal, $pri:literal, $sec:literal, $min:literal) => {
        #[doc = concat!("`", stringify!($name), "` (`filter_func_8`)")]
        unsafe fn $name(
            dst: P,
            dst_stride: isize,
            tmp: *const u8,
            pri_strength: c_int,
            sec_strength: c_int,
            dir: c_int,
            damping: c_int,
            h: c_int,
        ) {
            filter_func_8::<$w, $pri, $sec, $min>(dst, dst_stride, tmp, pri_strength, sec_strength, dir, damping, h)
        }
    };
}
// filter_8 8
edged_instance!(cdef_filter8_pri_edged_8bpc_neon, 8, true, false, false);
edged_instance!(cdef_filter8_sec_edged_8bpc_neon, 8, false, true, false);
edged_instance!(cdef_filter8_pri_sec_edged_8bpc_neon, 8, true, true, true);
// filter_8 4
edged_instance!(cdef_filter4_pri_edged_8bpc_neon, 4, true, false, false);
edged_instance!(cdef_filter4_sec_edged_8bpc_neon, 4, false, true, false);
edged_instance!(cdef_filter4_pri_sec_edged_8bpc_neon, 4, true, true, true);

/// An 8 bpc instance of `filter_func`, branching to its edged variant.
macro_rules! filter_instance {
    ($name:ident, $w:literal, $pri:literal, $sec:literal, $min:literal, $edged:ident) => {
        #[doc = concat!("`", stringify!($name), "` (`filter_func`)")]
        unsafe fn $name(
            dst: *mut u8,
            dst_stride: isize,
            tmp: *const u16,
            pri_strength: c_int,
            sec_strength: c_int,
            dir: c_int,
            damping: c_int,
            h: c_int,
            edges: usize,
        ) {
            filter_func::<$w, 8, $pri, $sec, $min>(
                dst,
                dst_stride,
                tmp as *const u8,
                pri_strength,
                sec_strength,
                dir,
                damping,
                h,
                edges,
                0,
                Some($edged),
            )
        }
    };
}
// filter 8, 8
filter_instance!(cdef_filter8_pri_8bpc_neon, 8, true, false, false, cdef_filter8_pri_edged_8bpc_neon);
filter_instance!(cdef_filter8_sec_8bpc_neon, 8, false, true, false, cdef_filter8_sec_edged_8bpc_neon);
filter_instance!(cdef_filter8_pri_sec_8bpc_neon, 8, true, true, true, cdef_filter8_pri_sec_edged_8bpc_neon);
// filter 4, 8
filter_instance!(cdef_filter4_pri_8bpc_neon, 4, true, false, false, cdef_filter4_pri_edged_8bpc_neon);
filter_instance!(cdef_filter4_sec_8bpc_neon, 4, false, true, false, cdef_filter4_sec_edged_8bpc_neon);
filter_instance!(cdef_filter4_pri_sec_8bpc_neon, 4, true, true, true, cdef_filter4_pri_sec_edged_8bpc_neon);

/// `void dav1d_cdef_filter8_8bpc_neon(pixel *dst, ptrdiff_t dst_stride,
/// const uint16_t *tmp, int pri_strength, int sec_strength, int dir,
/// int damping, int h, size_t edges)` (the function of the macro `filter`)
pub unsafe extern "C" fn dav1d_cdef_filter8_8bpc_neon(
    dst: *mut u8,
    dst_stride: isize,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
) {
    let a = (dst, dst_stride, tmp, pri_strength, sec_strength, dir, damping, h, edges);
    if pri_strength == 0 {
        // only sec
        return cdef_filter8_sec_8bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8);
    }
    if sec_strength == 0 {
        // only pri
        return cdef_filter8_pri_8bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8);
    }
    // both pri and sec
    cdef_filter8_pri_sec_8bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8)
}

/// `dav1d_cdef_filter4_8bpc_neon`
pub unsafe extern "C" fn dav1d_cdef_filter4_8bpc_neon(
    dst: *mut u8,
    dst_stride: isize,
    tmp: *const u16,
    pri_strength: c_int,
    sec_strength: c_int,
    dir: c_int,
    damping: c_int,
    h: c_int,
    edges: usize,
) {
    let a = (dst, dst_stride, tmp, pri_strength, sec_strength, dir, damping, h, edges);
    if pri_strength == 0 {
        return cdef_filter4_sec_8bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8);
    }
    if sec_strength == 0 {
        return cdef_filter4_pri_8bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8);
    }
    cdef_filter4_pri_sec_8bpc_neon(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8)
}

// --- find_dir (cdef_tmpl.S) ----------------------------------------------------------

/// The macro `cost_alt d1, d2, s1, s2, s3, s4`.
#[inline(always)]
fn cost_alt(v: &mut V, d1: usize, d2: usize, s1: usize, s2: usize, s3: usize, s4: usize) {
    v[22] = smull(v[s1], v[s1]); // sum_alt[n]*sum_alt[n]
    v[23] = smull2(v[s1], v[s1]);
    v[24] = smull(v[s2], v[s2]);
    v[25] = smull(v[s3], v[s3]); // sum_alt[n]*sum_alt[n]
    v[26] = smull2(v[s3], v[s3]);
    v[27] = smull(v[s4], v[s4]);
    v[22] = mul4s(v[22], v[29]); // sum_alt[n]^2*fact
    v[22] = mla4s(v[22], v[23], v[30]);
    v[22] = mla4s(v[22], v[24], v[31]);
    v[25] = mul4s(v[25], v[29]); // sum_alt[n]^2*fact
    v[25] = mla4s(v[25], v[26], v[30]);
    v[25] = mla4s(v[25], v[27], v[31]);
    v[d1] = addv4s(v[22]); // *cost_ptr
    v[d2] = addv4s(v[25]); // *cost_ptr
}

/// The general registers of `find_best`.
struct Best {
    w0: u32,
    w1: u32,
    w3: u32,
    w4: u32,
    w5: u32,
}

/// The macro `find_best s1, s2, s3` (`S2`: `s2` and `s3` given).
#[inline(always)]
fn find_best<const S2: bool>(v: &V, r: &mut Best, s2: usize, s3: usize) {
    if S2 {
        r.w5 = s0(v[s2]);
    }
    let gt = r.w4 as i32 > r.w1 as i32; // cost[n] > best_cost
    r.w0 = if gt { r.w3 } else { r.w0 }; // best_dir = n
    r.w1 = if gt { r.w4 } else { r.w1 }; // best_cost = cost[n]
    if S2 {
        r.w3 += 1; // n++
        let gt = r.w5 as i32 > r.w1 as i32; // cost[n] > best_cost
        r.w4 = s0(v[s3]);
        r.w0 = if gt { r.w3 } else { r.w0 }; // best_dir = n
        r.w1 = if gt { r.w5 } else { r.w1 }; // best_cost = cost[n]
        r.w3 += 1; // n++
    }
}

/// The macro `dir_load_step1 s1, bpc`: the next row, `x0 += x1`.
#[inline(always)]
unsafe fn dir_load_step1<const BPC: u32>(v: &mut V, s1: usize, x0: &mut *const u8, x1: isize) {
    if BPC == 8 {
        v[s1] = ldr(*x0, 8); // ld1 {s1.8b}
    } else {
        v[s1] = ldr(*x0, 16); // ld1 {s1.8h}
    }
    *x0 = x0.offset(x1);
}

/// The macro `dir_load_step2 s1, bpc`.
#[inline(always)]
fn dir_load_step2<const BPC: u32>(v: &mut V, s1: usize) {
    // SAFETY: NEON value intrinsics only.
    unsafe {
        if BPC == 8 {
            v[s1] = ss(vsubl_u8(lo8(v[s1]), lo8(v[31]))); // usubl s1.8h, s1.8b, v31.8b
        } else {
            v[s1] = ushl(v[s1], v[8]);
        }
    }
}

/// The macro `dir_load_step3 s1, bpc` (nothing at 8 bpc).
#[inline(always)]
fn dir_load_step3<const BPC: u32>(v: &mut V, s1: usize) {
    if BPC != 8 {
        v[s1] = sub(v[s1], v[31]);
    }
}

/// The body of `.irpc i, 01234567` in `find_dir`, for `i` = `I`.
#[inline(always)]
unsafe fn find_dir_row<const I: usize, const BPC: u32>(v: &mut V, x0: &mut *const u8, x1: isize) {
    v[25] = ss(vsetq_lane_u16::<0>(vaddvq_u16(uu(v[26])), vdupq_n_u16(0))); // addv h25, v26.8h: [y]
    v[27] = vrev64q_s16(v[26]);
    v[28] = vpaddq_s16(v[26], v[30]); // [(x >> 1)]
    v[5] = add(v[5], v[26]); // sum_hv[1]
    v[27] = ext(v[27], v[27], 8); // [-x]
    v[29] = d(vrev64_s16(lo(v[28]))); // [-(x >> 1)]
    v[4] = ins_h(v[4], I, v[25]); // sum_hv[0]
    if I < 6 {
        let n = 16 - 2 * (3 - (I / 2));
        v[22] = ext(v[30], v[26], n);
        v[23] = ext(v[26], v[30], n);
        v[18] = add(v[18], v[22]); // sum_alt[2]
        v[19] = add4h(v[19], v[23]); // sum_alt[2]
    } else {
        v[18] = add(v[18], v[26]); // sum_alt[2]
    }
    if I == 0 {
        v[20] = v[26]; // sum_alt[3]
    } else if I == 1 {
        v[20] = add(v[20], v[26]); // sum_alt[3]
    } else {
        let n = 16 - 2 * (I / 2);
        v[24] = ext(v[30], v[26], n);
        v[25] = ext(v[26], v[30], n);
        v[20] = add(v[20], v[24]); // sum_alt[3]
        v[21] = add4h(v[21], v[25]); // sum_alt[3]
    }
    if I == 0 {
        v[0] = v[26]; // sum_diag[0]
        dir_load_step1::<BPC>(v, 26, x0, x1);
        v[2] = v[27]; // sum_diag[1]
        dir_load_step2::<BPC>(v, 26);
        v[6] = v[28]; // sum_alt[0]
        dir_load_step3::<BPC>(v, 26);
        v[16] = v[29]; // sum_alt[1]
    } else {
        let n = 16 - 2 * I;
        v[22] = ext(v[30], v[26], n);
        v[23] = ext(v[26], v[30], n);
        v[24] = ext(v[30], v[27], n);
        v[25] = ext(v[27], v[30], n);
        if I != 7 {
            // Nothing to load for the final row
            dir_load_step1::<BPC>(v, 26, x0, x1); // Start setting up the next row early.
        }
        v[0] = add(v[0], v[22]); // sum_diag[0]
        v[1] = add(v[1], v[23]); // sum_diag[0]
        v[2] = add(v[2], v[24]); // sum_diag[1]
        v[3] = add(v[3], v[25]); // sum_diag[1]
        if I != 7 {
            dir_load_step2::<BPC>(v, 26);
        }
        v[22] = ext(v[30], v[28], n);
        v[23] = ext(v[28], v[30], n);
        v[24] = ext(v[30], v[29], n);
        v[25] = ext(v[29], v[30], n);
        if I != 7 {
            dir_load_step3::<BPC>(v, 26);
        }
        v[6] = add(v[6], v[22]); // sum_alt[0]
        v[7] = add4h(v[7], v[23]); // sum_alt[0]
        v[16] = add(v[16], v[24]); // sum_alt[1]
        v[17] = add4h(v[17], v[25]); // sum_alt[1]
    }
}

/// The macro `find_dir bpc`: `int dav1d_cdef_find_dir_\bpc\()bpc_neon(const
/// pixel *img, const ptrdiff_t stride, unsigned *const var
/// HIGHBD_DECL_SUFFIX)`; `img` is a byte pointer.
#[inline(always)]
pub(super) unsafe fn find_dir<const BPC: u32>(img: *const u8, stride: isize, var: *mut c_uint, bitdepth_max: c_int) -> c_int {
    let (mut x0, x1, x2) = (img, stride, var);
    let mut v = regs();
    if BPC == 16 {
        let w3 = (bitdepth_max as u32).leading_zeros(); // clz(bitdepth_max)
        let w3 = w3.wrapping_sub(24); // -bitdepth_min_8
        v[8] = vdupq_n_s16(w3 as i16);
    }
    let mut cost = [0u32; 8]; // sub sp, sp, #32
    if BPC == 8 {
        v[31] = dup_b(128);
    } else {
        v[31] = vdupq_n_s16(128);
    }
    v[30] = dup_b(0);
    v[1] = vdupq_n_s16(0); // v0-v1 sum_diag[0]
    v[3] = vdupq_n_s16(0); // v2-v3 sum_diag[1]
    v[5] = vdupq_n_s16(0); // v4-v5 sum_hv[0-1]
    v[7] = vdupq_n_s16(0); // v6-v7 sum_alt[0]
    dir_load_step1::<BPC>(&mut v, 26, &mut x0, x1); // Setup first row early
    v[17] = vdupq_n_s16(0); // v16-v17 sum_alt[1]
    v[18] = vdupq_n_s16(0); // v18-v19 sum_alt[2]
    dir_load_step2::<BPC>(&mut v, 26);
    v[19] = vdupq_n_s16(0);
    dir_load_step3::<BPC>(&mut v, 26);
    v[21] = vdupq_n_s16(0); // v20-v21 sum_alt[3]

    find_dir_row::<0, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<1, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<2, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<3, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<4, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<5, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<6, BPC>(&mut v, &mut x0, x1);
    find_dir_row::<7, BPC>(&mut v, &mut x0, x1);

    v[31] = hw(vdupq_n_s32(105));

    v[26] = smull(v[4], v[4]); // sum_hv[0]*sum_hv[0]
    v[26] = smlal2(v[26], v[4], v[4]);
    v[27] = smull(v[5], v[5]); // sum_hv[1]*sum_hv[1]
    v[27] = smlal2(v[27], v[5], v[5]);
    v[26] = mul4s(v[26], v[31]); // cost[2] *= 105
    v[27] = mul4s(v[27], v[31]); // cost[6] *= 105
    v[4] = addv4s(v[26]); // cost[2]
    v[5] = addv4s(v[27]); // cost[6]

    v[1] = vrev64q_s16(v[1]);
    v[3] = vrev64q_s16(v[3]);
    v[1] = ext(v[1], v[1], 10); // sum_diag[0][14-n]
    v[3] = ext(v[3], v[3], 10); // sum_diag[1][14-n]

    cost[2] = s0(v[4]); // cost[2]
    cost[6] = s0(v[5]); // cost[6]

    v[31] = ldr(DIV_TABLE.as_ptr().cast(), 16); // ldrel q31, x4, div_table

    v[22] = smull(v[0], v[0]); // sum_diag[0]*sum_diag[0]
    v[23] = smull2(v[0], v[0]);
    v[22] = smlal(v[22], v[1], v[1]);
    v[23] = smlal2(v[23], v[1], v[1]);
    v[24] = smull(v[2], v[2]); // sum_diag[1]*sum_diag[1]
    v[25] = smull2(v[2], v[2]);
    v[24] = smlal(v[24], v[3], v[3]);
    v[25] = smlal2(v[25], v[3], v[3]);
    v[30] = uxtl4s(v[31]); // div_table
    v[31] = uxtl2_4s(v[31]);
    v[22] = mul4s(v[22], v[30]); // cost[0]
    v[22] = mla4s(v[22], v[23], v[31]); // cost[0]
    v[24] = mul4s(v[24], v[30]); // cost[4]
    v[24] = mla4s(v[24], v[25], v[31]); // cost[4]
    v[0] = addv4s(v[22]); // cost[0]
    v[2] = addv4s(v[24]); // cost[4]

    let x5 = ALT_FACT.as_ptr();
    let t = vld1_u16_x3(x5); // div_table[2*m+1] + 105
    v[29] = d(vreinterpret_s16_u16(t.0));
    v[30] = d(vreinterpret_s16_u16(t.1));
    v[31] = d(vreinterpret_s16_u16(t.2));

    cost[0] = s0(v[0]); // cost[0]
    cost[4] = s0(v[2]); // cost[4]

    v[29] = uxtl4s(v[29]); // div_table[2*m+1] + 105
    v[30] = uxtl4s(v[30]);
    v[31] = uxtl4s(v[31]);

    cost_alt(&mut v, 6, 16, 6, 7, 16, 17); // cost[1], cost[3]
    cost_alt(&mut v, 18, 20, 18, 19, 20, 21); // cost[5], cost[7]
    cost[1] = s0(v[6]); // cost[1]
    cost[3] = s0(v[16]); // cost[3]

    let mut r = Best { w0: 0, w1: s0(v[0]), w3: 1, w4: 0, w5: 0 }; // best_dir, best_cost, n

    cost[5] = s0(v[18]); // cost[5]
    cost[7] = s0(v[20]); // cost[7]

    r.w4 = s0(v[6]);

    find_best::<true>(&v, &mut r, 4, 16);
    find_best::<true>(&v, &mut r, 2, 18);
    find_best::<true>(&v, &mut r, 5, 20);
    find_best::<false>(&v, &mut r, 0, 0);

    let w3 = r.w0 ^ 4; // best_dir ^4
    let w4 = *cost.as_ptr().add(w3 as usize); // ldr w4, [sp, w3, uxtw #2]
    let mut w1 = r.w1.wrapping_sub(w4); // best_cost - cost[best_dir ^ 4]
    w1 >>= 10;
    *x2 = w1; // *var

    r.w0 as c_int
}

/// `int dav1d_cdef_find_dir_8bpc_neon(const pixel *img, const ptrdiff_t
/// stride, unsigned *const var)` (`find_dir 8`)
pub unsafe extern "C" fn dav1d_cdef_find_dir_8bpc_neon(img: *const u8, stride: isize, var: *mut c_uint) -> c_int {
    find_dir::<8>(img, stride, var, 0)
}
