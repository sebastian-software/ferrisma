// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2020, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/itx16.S` (16 bpc) with NEON
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! All 156 entry points that `itx_dsp_init_arm` assigns at 16 bpc (the
//! whole file). Lines 1 to 1698 of the assembly: the constant tables, the macros
//! (`mul_mla`, `mul_mls`, `scale_input`, `smin_4s`/`smax_4s`, the
//! `load_add_store*` family, `idct_dc`, `iwht4`, `idct_4`, `iadst_4x4`,
//! `idct_8`, `iadst_8`, `iadst_16`, `identity_4x16*`, `def_horz_16`), the
//! functions `idct_dc_w4..w64_neon`, the 32-bit first-pass transforms
//! `inv_{dct,adst,flipadst,identity}_4s_x{4,8,16}_neon`,
//! `inv_txfm_horz{,_scale}_16x4_neon`, `inv_txfm_add_vert_8x16_neon`,
//! `inv_txfm_add_{4x4,8x8,4x8,8x4,16x16,16x4,4x16}_neon`, and the exported
//! entry points of sizes 4x4 (with `wht_wht`), 8x8, 4x8, 8x4, 16x16, 4x16
//! and 16x4 with their eob thresholds and eob tables. Lines 1699 to the end
//! (from the section marked below): 16x8 and 8x16 (every type), the
//! identity and DCT 32-point sizes (32x32, 16x32, 32x16, 8x32, 32x8) and
//! the 64-point DCT sizes (64x64, 64x32, 32x64, 64x16, 16x64), reusing
//! `idct32_odd` and `dct_8h_x64` of `itx.rs` for the 16-bit passes the
//! assembly takes from `itx.S`. The assembly is the
//! 10-bit path; `itx_dsp_init_arm` assigns it only for `bpc == 10` (WHT at
//! every bit depth).
//!
//! Correspondence with the assembly:
//! - Each exported `function` is one `extern "C"` function of the same name;
//!   each internal `function` and each `.macro` is a Rust function of the
//!   same name without `_neon`. `blr x4`/`blr x5`/`blr x6` become calls of
//!   the 1-D transform the entry point loaded into that register:
//!   `call_4s` for the 32-bit first-pass transforms of this file, and
//!   `super::itx::call` for the 16-bit second-pass transforms
//!   (`inv_*_8h_x*`/`inv_*_4h_x*`), which the assembly takes from `itx.S`.
//! - The register file is `v: V` (`util.rs`), indexed by register number,
//!   one 128-bit value per register; an instruction on `.4s` views it as
//!   four 32-bit lanes (`s`/`h` reinterpret, no conversion). The 1-D
//!   transforms are transcribed instruction by instruction on that file,
//!   including the scratch registers `v2`–`v7` and the coefficient and
//!   clip-constant registers `v0`, `v1`, `v4`, `v5`, `v7` they leave behind
//!   (`inv_dct_4s_x16` leaves the row clip constants in `v5`/`v4`, which
//!   callers of the 32-point part rely on). Coefficient lanes are read from
//!   the register (`mul_mla::<3, 2>(…, v[0], v[0])` is `v0.s[3], v0.s[2]`).
//!   A 64-bit arrangement (`.4h`, `.2s`) writes the low half and clears the
//!   high half.
//! - The `load_add_store*` macros are computed per row: load, `srshr`,
//!   `usqadd`, `smin` against `0x3ff` (`bitdepth_max` in
//!   `L(itx_4x4_end)`), store. Their interleaving of rows only schedules
//!   independent lanes (the instructions per row are the assembly's); the
//!   registers they reuse as load targets are not written back (no caller
//!   reads them afterwards). `idct_dc_w*` keep the assembly's rows per
//!   step, and `w16`–`w64` its multi-register `ld1`/`st1`
//!   (`vld1q_s16_x2`/`_x4`).
//! - The dct_dct entry points of the 32- and 64-point sizes test the DC
//!   case and then call `inv_txfm_add_dct_dct_<w>x<h>_16bpc_coefs`
//!   (not inlined): the assembly sets up its stack frame only after the
//!   `idct_dc` macro, and one Rust function would set up the frame of the
//!   full path (registers saved, stack probe) before the DC test.
//! - Pointer updates that can step past the block (the assembly's
//!   post-increments after the last row, the 64-point first pass pointing
//!   `x9` past the coefficients) use wrapping arithmetic or element
//!   offsets; no access is made there.
//! - The stack buffers (`sub sp, sp, #512` of `inv_txfm_add_16x16_neon`,
//!   and those of the later sizes) are zero-initialized local arrays;
//!   every element is written before it is read, as in the assembly.
//! - The general-purpose registers (`x0` dst, `x1` stride in bytes, `x2`
//!   coeff, `w3` eob, `w4` bitdepth_max, `x13` eob threshold or table) are
//!   function arguments; strides of the helpers (`x8`, `x11`) are counted in
//!   elements, post-increment addressing becomes explicit offsets.
//!
//! Safety (argued once for the family): the translated caller `recon_tmpl`
//! passes what the original passes to the assembly: `dst` points to a
//! block of `h` rows of `w` writable 16-bit pixels, `stride` bytes apart,
//! and `coeff` to the block's coefficient buffer of at least `w * h`
//! (capped at 32x32) 32-bit values, both live for the call and not
//! otherwise accessed during it. Every load and store below stays inside
//! those two regions: pixel accesses are rows `0..h` and columns `0..w` of
//! `dst`, coefficient accesses are offsets `0..min(w, 32) * min(h, 32)` of
//! `coeff` (the DC-only paths touch offset 0 only). The remaining accesses are to local arrays.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::c_int;

use super::itx::{call, K};
use super::util::*;

type Dst = *mut u16;
type Coef = *mut i32;

// --- constant tables ---------------------------------------------------------

/// `idct_coeffs`: idct4, idct8, idct16, idct32.
const IDCT_COEFFS: [i32; 32] = [
    // idct4
    2896, 2896 * 8 * (1 << 16), 1567, 3784, //
    // idct8
    799, 4017, 3406, 2276, //
    // idct16
    401, 4076, 3166, 2598, //
    1931, 3612, 3920, 1189, //
    // idct32
    201, 4091, 3035, 2751, //
    1751, 3703, 3857, 1380, //
    995, 3973, 3513, 2106, //
    2440, 3290, 4052, 601,
];

/// `idct64_coeffs`
const IDCT64_COEFFS: [i32; 48] = [
    101 * 8 * (1 << 16), 4095 * 8 * (1 << 16), 2967 * 8 * (1 << 16), -2824 * 8 * (1 << 16), //
    1660 * 8 * (1 << 16), 3745 * 8 * (1 << 16), 3822 * 8 * (1 << 16), -1474 * 8 * (1 << 16), //
    4076, 401, 4017, 799, //
    //
    4036 * 8 * (1 << 16), -700 * 8 * (1 << 16), 2359 * 8 * (1 << 16), 3349 * 8 * (1 << 16), //
    3461 * 8 * (1 << 16), -2191 * 8 * (1 << 16), 897 * 8 * (1 << 16), 3996 * 8 * (1 << 16), //
    -3166, -2598, -799, -4017, //
    //
    501 * 8 * (1 << 16), 4065 * 8 * (1 << 16), 3229 * 8 * (1 << 16), -2520 * 8 * (1 << 16), //
    2019 * 8 * (1 << 16), 3564 * 8 * (1 << 16), 3948 * 8 * (1 << 16), -1092 * 8 * (1 << 16), //
    3612, 1931, 2276, 3406, //
    //
    4085 * 8 * (1 << 16), -301 * 8 * (1 << 16), 2675 * 8 * (1 << 16), 3102 * 8 * (1 << 16), //
    3659 * 8 * (1 << 16), -1842 * 8 * (1 << 16), 1285 * 8 * (1 << 16), 3889 * 8 * (1 << 16), //
    -3920, -1189, -3406, -2276,
];

/// `iadst4_coeffs`
const IADST4_COEFFS: [i32; 4] = [1321, 3803, 2482, 3344];

/// `iadst8_coeffs` (with the idct_coeffs tail)
const IADST8_COEFFS: [i32; 12] = [
    4076, 401, 3612, 1931, //
    2598, 3166, 1189, 3920, //
    // idct_coeffs
    2896, 0, 1567, 3784,
];

/// `iadst16_coeffs`
const IADST16_COEFFS: [i32; 16] = [
    4091, 201, 3973, 995, //
    3703, 1751, 3290, 2440, //
    2751, 3035, 2106, 3513, //
    1380, 3857, 601, 4052,
];

/// `movi vd.4s, #1, msl #16`: row_clip_max = ~(~bdmax << 7), 0x1ffff
const ROW_CLIP_MAX: i32 = 0x1ffff;
/// `mvni vd.4s, #1, msl #16`: row_clip_min = (~bdmax << 7), 0xfffe0000
const ROW_CLIP_MIN: i32 = !0x1ffff;
/// `movz w16, #2896*8, lsl #16`
const W2896: i32 = 2896 * 8 * (1 << 16);

/// `eob_16x16`
const EOB_16X16: [u16; 4] = [10, 36, 78, 256];
/// `eob_16x16_identity`
const EOB_16X16_IDENTITY: [u16; 4] = [4, 8, 12, 256];
/// `eob_4x16`
const EOB_4X16: [u16; 4] = [13, 29, 45, 64];
/// `eob_4x16_identity1`
const EOB_4X16_IDENTITY1: [u16; 4] = [16, 32, 48, 64];
/// `eob_4x16_identity2`
const EOB_4X16_IDENTITY2: [u16; 4] = [4, 8, 12, 64];

// --- instructions on the register file (SAFETY of the `unsafe` blocks in
// this section: NEON value intrinsics; NEON is baseline on AArch64) ---------

/// The four 32-bit lanes of a register.
#[inline(always)]
fn s(x: int16x8_t) -> int32x4_t {
    unsafe { vreinterpretq_s32_s16(x) }
}

/// A register holding four 32-bit lanes.
#[inline(always)]
fn h(x: int32x4_t) -> int16x8_t {
    unsafe { vreinterpretq_s16_s32(x) }
}

/// `movi`/`mvni vd.4s, …`: every 32-bit lane `x`.
#[inline(always)]
fn dup_4s(x: i32) -> int16x8_t {
    unsafe { h(vdupq_n_s32(x)) }
}

/// `dup vd.2s, wN`: the low two 32-bit lanes `x`, the high half cleared.
#[inline(always)]
fn dup_2s(x: i32) -> int16x8_t {
    unsafe { h(vcombine_s32(vdup_n_s32(x), vdup_n_s32(0))) }
}

/// `mvni vd.8h, #0xfc, lsl #8`: every 16-bit lane `0x3ff`.
#[inline(always)]
fn mvni_3ff() -> int16x8_t {
    unsafe { vdupq_n_s16(0x3ff) }
}

/// `ld1 {vd.4s}` from a constant table at element `k` (`ldrel`, `movrel` +
/// `ld1`/`ldp`).
#[inline(always)]
fn coeffs(t: &[i32], k: usize) -> int16x8_t {
    let a = [t[k], t[k + 1], t[k + 2], t[k + 3]];
    // SAFETY: `a` is a local array of four 32-bit values.
    unsafe { h(vld1q_s32(a.as_ptr())) }
}

/// The `mul_mla d, s0, s1, c0, c1` macro: `mul d.4s, s0.4s, c0;
/// mla d.4s, s1.4s, c1` with `c0 = r0.s[L0]`, `c1 = r1.s[L1]` (wrapping).
#[inline(always)]
fn mul_mla<const L0: i32, const L1: i32>(s0: int16x8_t, s1: int16x8_t, r0: int16x8_t, r1: int16x8_t) -> int16x8_t {
    unsafe { h(vmlaq_laneq_s32::<L1>(vmulq_laneq_s32::<L0>(s(s0), s(r0)), s(s1), s(r1))) }
}

/// The `mul_mls d, s0, s1, c0, c1` macro: `mul d.4s, s0.4s, c0;
/// mls d.4s, s1.4s, c1` with `c0 = r0.s[L0]`, `c1 = r1.s[L1]` (wrapping).
#[inline(always)]
fn mul_mls<const L0: i32, const L1: i32>(s0: int16x8_t, s1: int16x8_t, r0: int16x8_t, r1: int16x8_t) -> int16x8_t {
    unsafe { h(vmlsq_laneq_s32::<L1>(vmulq_laneq_s32::<L0>(s(s0), s(r0)), s(s1), s(r1))) }
}

/// `mul vd.4s, vn.4s, vm.s[L]`
#[inline(always)]
fn mul_4s<const L: i32>(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { h(vmulq_laneq_s32::<L>(s(a), s(c))) }
}

/// `mla vd.4s, vn.4s, vm.s[L]`
#[inline(always)]
fn mla_4s<const L: i32>(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { h(vmlaq_laneq_s32::<L>(s(acc), s(a), s(c))) }
}

/// `mls vd.4s, vn.4s, vm.s[L]`
#[inline(always)]
fn mls_4s<const L: i32>(acc: int16x8_t, a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { h(vmlsq_laneq_s32::<L>(s(acc), s(a), s(c))) }
}

/// `add vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn add_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { h(vaddq_s32(s(a), s(b))) }
}

/// `sub vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn sub_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { h(vsubq_s32(s(a), s(b))) }
}

/// `srshr vd.4s, vn.4s, #N`
#[inline(always)]
fn srshr_4s<const N: i32>(a: int16x8_t) -> int16x8_t {
    unsafe { h(vrshrq_n_s32::<N>(s(a))) }
}

/// `sshr vd.4s, vn.4s, #N`
#[inline(always)]
fn sshr_4s<const N: i32>(a: int16x8_t) -> int16x8_t {
    unsafe { h(vshrq_n_s32::<N>(s(a))) }
}

/// `sqadd vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn sqadd_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { h(vqaddq_s32(s(a), s(b))) }
}

/// `sqsub vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn sqsub_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { h(vqsubq_s32(s(a), s(b))) }
}

/// `sqneg vd.4s, vn.4s`
#[inline(always)]
fn sqneg_4s(a: int16x8_t) -> int16x8_t {
    unsafe { h(vqnegq_s32(s(a))) }
}

/// `neg vd.4s, vn.4s` (wrapping)
#[inline(always)]
fn neg_4s(a: int16x8_t) -> int16x8_t {
    unsafe { h(vnegq_s32(s(a))) }
}

/// `sqshl vd.4s, vn.4s, #1`
#[inline(always)]
fn sqshl1_4s(a: int16x8_t) -> int16x8_t {
    unsafe { h(vqshlq_n_s32::<1>(s(a))) }
}

/// `sqrdmulh vd.4s, vn.4s, vm.s[L]`
#[inline(always)]
fn sqrdmulh_4s<const L: i32>(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { h(vqrdmulhq_laneq_s32::<L>(s(a), s(c))) }
}

/// `sqrdmulh vd.8h, vn.8h, vm.h[L]`
#[inline(always)]
fn sqrdmulh_8h<const L: i32>(a: int16x8_t, c: int16x8_t) -> int16x8_t {
    unsafe { vqrdmulhq_laneq_s16::<L>(a, c) }
}

/// The `smin_4s r0, r1, r2` macro: `smin r0.4s, r1.4s, r2.4s`
#[inline(always)]
fn smin_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { h(vminq_s32(s(a), s(b))) }
}

/// The `smax_4s r0, r1, r2` macro: `smax r0.4s, r1.4s, r2.4s`
#[inline(always)]
fn smax_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { h(vmaxq_s32(s(a), s(b))) }
}

/// `sqxtn vd.4h, vn.4s` (clears the high half)
#[inline(always)]
fn sqxtn(a: int16x8_t) -> int16x8_t {
    unsafe { d(vqmovn_s32(s(a))) }
}

/// `sqxtn2 vd.8h, vn.4s` (keeps the low half of `vd`)
#[inline(always)]
fn sqxtn2(rd: int16x8_t, a: int16x8_t) -> int16x8_t {
    unsafe { vqmovn_high_s32(lo(rd), s(a)) }
}

/// `sqrshrn vd.4h, vn.4s, #N` (clears the high half)
#[inline(always)]
fn sqrshrn<const N: i32>(a: int16x8_t) -> int16x8_t {
    unsafe { d(vqrshrn_n_s32::<N>(s(a))) }
}

/// `sqrshrn2 vd.8h, vn.4s, #N` (keeps the low half of `vd`)
#[inline(always)]
fn sqrshrn2<const N: i32>(rd: int16x8_t, a: int16x8_t) -> int16x8_t {
    unsafe { vqrshrn_high_n_s32::<N>(lo(rd), s(a)) }
}

/// `usqadd vd.8h, vn.8h`: the unsigned pixels `vd` plus the signed `vn`,
/// saturated to `0..=0xffff`.
#[inline(always)]
fn usqadd(rd: int16x8_t, a: int16x8_t) -> int16x8_t {
    unsafe { vreinterpretq_s16_u16(vsqaddq_u16(vreinterpretq_u16_s16(rd), a)) }
}

/// `smin vd.8h, vn.8h, vm.8h` (signed)
#[inline(always)]
fn smin_8h(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    unsafe { vminq_s16(a, b) }
}

// --- memory access -------------------------------------------------------------

/// `ld1 {vd.4s}, [p]`
#[inline(always)]
unsafe fn ld_4s(p: *const i32) -> int16x8_t {
    h(vld1q_s32(p))
}

/// `st1 {vs.4s}, [p]`
#[inline(always)]
unsafe fn st_4s(p: *mut i32, x: int16x8_t) {
    vst1q_s32(p, s(x));
}

/// Row `r` below `p` (`stride` in bytes).
#[inline(always)]
unsafe fn row(p: Dst, stride: isize, r: isize) -> Dst {
    (p as *mut u8).offset(r * stride) as Dst
}

/// `ld1 {vd.8h}, [p]`: eight pixels.
#[inline(always)]
unsafe fn ld_8h(p: Dst) -> int16x8_t {
    vld1q_s16(p as *const i16)
}

/// `st1 {vs.8h}, [p]`
#[inline(always)]
unsafe fn st_8h(p: Dst, x: int16x8_t) {
    vst1q_s16(p as *mut i16, x);
}

/// `ld1 {vd.d}[0], [p], x1; ld1 {vd.d}[1], [p], x1`: two rows of four
/// pixels.
#[inline(always)]
unsafe fn ld_d2(p: Dst, stride: isize) -> int16x8_t {
    vcombine_s16(vld1_s16(p as *const i16), vld1_s16(row(p, stride, 1) as *const i16))
}

/// `st1 {vs.d}[0], [p], x1; st1 {vs.d}[1], [p], x1`
#[inline(always)]
unsafe fn st_d2(p: Dst, stride: isize, x: int16x8_t) {
    vst1_s16(p as *mut i16, vget_low_s16(x));
    vst1_s16(row(p, stride, 1) as *mut i16, vget_high_s16(x));
}

// --- load_add_store --------------------------------------------------------------

/// One row of the `load_add_store load, shift, addsrc, adddst, min, store`
/// macro: `ld1 {vL.8h}`, `srshr x, x, #SHIFT`, `usqadd vL, x`,
/// `smin vL, vL, v7` (0x3ff), `st1 {vL.8h}`.
#[inline(always)]
unsafe fn load_add_store<const SHIFT: i32>(p: Dst, x: int16x8_t) {
    let l = ld_8h(p);
    let x = srshr::<SHIFT>(x);
    let l = smin_8h(usqadd(l, x), mvni_3ff());
    st_8h(p, l);
}

/// `load_add_store_8x16 dst, src`: rows `0..16` from v16–v31.
#[inline(always)]
unsafe fn load_add_store_8x16(dst: Dst, stride: isize, v: &V) {
    for i in 0..16 {
        load_add_store::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// `load_add_store_8x8 dst, src, shiftbits`: rows `0..8` from v16–v23.
#[inline(always)]
unsafe fn load_add_store_8x8<const SHIFT: i32>(dst: Dst, stride: isize, v: &V) {
    for i in 0..8 {
        load_add_store::<SHIFT>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// `load_add_store_8x4 dst, src, shiftbits`: rows `0..4` from v16–v19.
#[inline(always)]
unsafe fn load_add_store_8x4<const SHIFT: i32>(dst: Dst, stride: isize, v: &V) {
    for i in 0..4 {
        load_add_store::<SHIFT>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// Two rows of the `load_add_store4` macro: `ld1 {vL.d}[0..1]`,
/// `ins ra.d[1], rb.d[0]`, `srshr #4`, `usqadd`, `smin` against `v7`
/// (0x3ff), `st1 {vL.d}[0..1]`; rows `p`, `p + stride` from the low halves
/// of `ra`, `rb`.
#[inline(always)]
unsafe fn load_add_store4(p: Dst, stride: isize, ra: int16x8_t, rb: int16x8_t) {
    let l = ld_d2(p, stride);
    let x = srshr::<4>(ins_hi_lo(ra, rb));
    let l = smin_8h(usqadd(l, x), mvni_3ff());
    st_d2(p, stride, l);
}

/// `load_add_store_4x16 dst, src`: rows `2k`, `2k+1` from v(16+2k), v(17+2k).
#[inline(always)]
unsafe fn load_add_store_4x16(dst: Dst, stride: isize, v: &V) {
    for k in 0..8 {
        load_add_store4(row(dst, stride, 2 * k as isize), stride, v[16 + 2 * k], v[17 + 2 * k]);
    }
}

/// `load_add_store_4x8 dst, src`: rows `2k`, `2k+1` from v(16+2k), v(17+2k).
#[inline(always)]
unsafe fn load_add_store_4x8(dst: Dst, stride: isize, v: &V) {
    for k in 0..4 {
        load_add_store4(row(dst, stride, 2 * k as isize), stride, v[16 + 2 * k], v[17 + 2 * k]);
    }
}

// --- idct_dc -------------------------------------------------------------------

/// The `idct_dc w, h, shift` macro when `eob == 0` (the caller tests
/// `cbnz w3`), followed by `idct_dc_w<w>_neon` with `w4 = h`.
#[inline(always)]
unsafe fn idct_dc<const W: usize, const H: usize, const SHIFT: i32>(dst: Dst, stride: isize, coeff: Coef) {
    // movz w16, #2896*8, lsl #16; ld1r {v16.4s}, [x2]; dup v0.2s, w16
    let v16 = h(vld1q_dup_s32(coeff));
    let v0 = dup_2s(W2896);
    let mut v20 = sqrdmulh_4s::<0>(v16, v0);
    // str wzr, [x2]
    *coeff = 0;
    if W == 2 * H || 2 * W == H {
        v20 = sqrdmulh_4s::<0>(v20, v0);
    }
    let mut v16 = match SHIFT {
        // sqxtn v16.4h, v20.4s; sqxtn2 v16.8h, v20.4s
        0 => sqxtn2(sqxtn(v20), v20),
        // sqrshrn v16.4h, v20.4s, #shift; sqrshrn2 v16.8h, v20.4s, #shift
        1 => sqrshrn2::<1>(sqrshrn::<1>(v20), v20),
        2 => sqrshrn2::<2>(sqrshrn::<2>(v20), v20),
        _ => unreachable!(),
    };
    // sqrdmulh v16.8h, v16.8h, v0.h[1]; srshr v16.8h, v16.8h, #4
    v16 = sqrdmulh_8h::<1>(v16, v0);
    v16 = srshr::<4>(v16);
    // mov w4, #h; b idct_dc_w<w>_neon
    let w4 = H as i32;
    match W {
        4 => idct_dc_w4(dst, stride, w4, v16),
        8 => idct_dc_w8(dst, stride, w4, v16),
        16 => idct_dc_w16(dst, stride, w4, v16),
        32 => idct_dc_w32(dst, stride, w4, v16),
        _ => idct_dc_w64(dst, stride, w4, v16),
    }
}

/// `idct_dc_w4_neon`: four rows of four pixels per step.
#[inline(always)]
unsafe fn idct_dc_w4(mut x0: Dst, x1: isize, mut w4: i32, v16: int16x8_t) {
    let v31 = mvni_3ff();
    loop {
        let v0 = ld_d2(x0, x1);
        let v1 = ld_d2(row(x0, x1, 2), x1);
        w4 -= 4;
        let v0 = smin_8h(usqadd(v0, v16), v31);
        let v1 = smin_8h(usqadd(v1, v16), v31);
        st_d2(x0, x1, v0);
        st_d2(row(x0, x1, 2), x1, v1);
        x0 = row(x0, x1, 4);
        // b.gt 1b
        if w4 <= 0 {
            break;
        }
    }
}

/// `idct_dc_w8_neon`: four rows of eight pixels per step.
#[inline(always)]
unsafe fn idct_dc_w8(mut x0: Dst, x1: isize, mut w4: i32, v16: int16x8_t) {
    let v31 = mvni_3ff();
    loop {
        let v = [0, 1, 2, 3].map(|r| ld_8h(row(x0, x1, r)));
        w4 -= 4;
        let v = v.map(|x| smin_8h(usqadd(x, v16), v31));
        for r in 0..4 {
            st_8h(row(x0, x1, r as isize), v[r]);
        }
        x0 = row(x0, x1, 4);
        if w4 <= 0 {
            break;
        }
    }
}

/// `idct_dc_w16_neon`: two rows of sixteen pixels per step; each row is one
/// two-register `ld1`/`st1`, as in the assembly.
#[inline(always)]
unsafe fn idct_dc_w16(mut x0: Dst, x1: isize, mut w4: i32, v16: int16x8_t) {
    let v31 = mvni_3ff();
    loop {
        // ld1 {v0.8h, v1.8h}, [x0], x1; ld1 {v2.8h, v3.8h}, [x0], x1
        let r1 = row(x0, x1, 1);
        let a = vld1q_s16_x2(x0 as *const i16);
        let b = vld1q_s16_x2(r1 as *const i16);
        w4 -= 2;
        let f = |x| smin_8h(usqadd(x, v16), v31);
        // st1 {v0.8h, v1.8h}, [x0], x1; st1 {v2.8h, v3.8h}, [x0], x1
        vst1q_s16_x2(x0 as *mut i16, int16x8x2_t(f(a.0), f(a.1)));
        vst1q_s16_x2(r1 as *mut i16, int16x8x2_t(f(b.0), f(b.1)));
        x0 = row(x0, x1, 2);
        if w4 <= 0 {
            break;
        }
    }
}

/// One `ld1 {v0.8h-v3.8h}`, `usqadd`, `smin` and `st1 {v0.8h-v3.8h}` on 32
/// pixels at `p`.
#[inline(always)]
unsafe fn dc_add32(p: Dst, v16: int16x8_t, v31: int16x8_t) {
    let a = vld1q_s16_x4(p as *const i16);
    let f = |x| smin_8h(usqadd(x, v16), v31);
    vst1q_s16_x4(p as *mut i16, int16x8x4_t(f(a.0), f(a.1), f(a.2), f(a.3)));
}

/// `idct_dc_w32_neon`: one row of 32 pixels per step (one four-register
/// `ld1` and `st1`).
#[inline(always)]
unsafe fn idct_dc_w32(mut x0: Dst, x1: isize, mut w4: i32, v16: int16x8_t) {
    let v31 = mvni_3ff();
    loop {
        w4 -= 1;
        dc_add32(x0, v16, v31);
        x0 = row(x0, x1, 1);
        if w4 <= 0 {
            break;
        }
    }
}

/// `idct_dc_w64_neon`: one row of 64 pixels per step (two four-register
/// `ld1` and `st1`).
#[inline(always)]
unsafe fn idct_dc_w64(mut x0: Dst, x1: isize, mut w4: i32, v16: int16x8_t) {
    let v31 = mvni_3ff();
    loop {
        w4 -= 1;
        dc_add32(x0, v16, v31);
        dc_add32(x0.add(32), v16, v31);
        x0 = row(x0, x1, 1);
        if w4 <= 0 {
            break;
        }
    }
}

// --- 1-D transforms, first pass (32 bits) ----------------------------------------

/// The `scale_input .4s, c, r0, …` macro: `sqrdmulh r.4s, r.4s, c` with
/// `c = rc.s[L]`.
#[inline(always)]
fn scale_input<const L: i32>(v: &mut V, rc: int16x8_t, regs: &[usize]) {
    for &r in regs {
        v[r] = sqrdmulh_4s::<L>(v[r], rc);
    }
}

/// `iwht4` on v16–v19 (`.4s`, wrapping `add`/`sub`, `sshr #1`).
#[inline(always)]
fn iwht4(v: &mut V) {
    v[16] = add_4s(v[16], v[17]);
    v[21] = sub_4s(v[18], v[19]);
    v[20] = sub_4s(v[16], v[21]);
    v[20] = sshr_4s::<1>(v[20]);
    v[18] = sub_4s(v[20], v[17]);
    v[17] = sub_4s(v[20], v[19]);
    v[19] = add_4s(v[21], v[18]);
    v[16] = sub_4s(v[16], v[17]);
}

/// `idct_4 r0, r1, r2, r3` (`v0` holds idct_coeffs)
#[inline(always)]
fn idct_4(v: &mut V, r0: usize, r1: usize, r2: usize, r3: usize) {
    v[6] = mul_mla::<3, 2>(v[r1], v[r3], v[0], v[0]);
    v[2] = mul_mla::<0, 0>(v[r0], v[r2], v[0], v[0]);
    v[4] = mul_mls::<2, 3>(v[r1], v[r3], v[0], v[0]);
    v[3] = mul_mls::<0, 0>(v[r0], v[r2], v[0], v[0]);
    v[6] = srshr_4s::<12>(v[6]);
    v[2] = srshr_4s::<12>(v[2]);
    v[7] = srshr_4s::<12>(v[4]);
    v[3] = srshr_4s::<12>(v[3]);
    v[r0] = sqadd_4s(v[2], v[6]);
    v[r3] = sqsub_4s(v[2], v[6]);
    v[r1] = sqadd_4s(v[3], v[7]);
    v[r2] = sqsub_4s(v[3], v[7]);
}

/// `inv_dct_4s_x4_neon`
#[inline(always)]
fn inv_dct_4s_x4(v: &mut V) {
    // ldrel q0, x16, idct_coeffs
    v[0] = coeffs(&IDCT_COEFFS, 0);
    idct_4(v, 16, 17, 18, 19);
}

/// `iadst_4x4 o0, o1, o2, o3`
#[inline(always)]
fn iadst_4x4(v: &mut V, o: [usize; 4]) {
    let [o0, o1, o2, o3] = o;
    // ldrel q0, x16, iadst4_coeffs
    v[0] = coeffs(&IADST4_COEFFS, 0);

    v[3] = sub_4s(v[16], v[18]);
    v[4] = mul_4s::<0>(v[16], v[0]);
    v[4] = mla_4s::<1>(v[4], v[18], v[0]);
    v[4] = mla_4s::<2>(v[4], v[19], v[0]);
    v[7] = mul_4s::<3>(v[17], v[0]);
    v[3] = add_4s(v[3], v[19]);
    v[5] = mul_4s::<2>(v[16], v[0]);
    v[5] = mls_4s::<0>(v[5], v[18], v[0]);
    v[5] = mls_4s::<1>(v[5], v[19], v[0]);

    v[o3] = add_4s(v[4], v[5]);
    v[o2] = mul_4s::<3>(v[3], v[0]);
    v[o0] = add_4s(v[4], v[7]);
    v[o1] = add_4s(v[5], v[7]);
    v[o3] = sub_4s(v[o3], v[7]);

    v[o0] = srshr_4s::<12>(v[o0]);
    v[o2] = srshr_4s::<12>(v[o2]);
    v[o1] = srshr_4s::<12>(v[o1]);
    v[o3] = srshr_4s::<12>(v[o3]);
}

/// `inv_adst_4s_x4_neon`
#[inline(always)]
fn inv_adst_4s_x4(v: &mut V) {
    iadst_4x4(v, [16, 17, 18, 19]);
}

/// `inv_flipadst_4s_x4_neon`
#[inline(always)]
fn inv_flipadst_4s_x4(v: &mut V) {
    iadst_4x4(v, [19, 18, 17, 16]);
}

/// `inv_identity_4s_x4_neon`
#[inline(always)]
fn inv_identity_4s_x4(v: &mut V) {
    // movz w16, #(5793-4096)*8, lsl #16; dup v0.2s, w16
    v[0] = dup_2s((5793 - 4096) * 8 * (1 << 16));
    v[4] = sqrdmulh_4s::<0>(v[16], v[0]);
    v[5] = sqrdmulh_4s::<0>(v[17], v[0]);
    v[6] = sqrdmulh_4s::<0>(v[18], v[0]);
    v[7] = sqrdmulh_4s::<0>(v[19], v[0]);
    v[16] = sqadd_4s(v[16], v[4]);
    v[17] = sqadd_4s(v[17], v[5]);
    v[18] = sqadd_4s(v[18], v[6]);
    v[19] = sqadd_4s(v[19], v[7]);
}

/// `idct_8 r0, …, r7` (`v0`, `v1` hold idct_coeffs); leaves row_clip_max
/// in `v5` and row_clip_min in `v4`.
#[inline(always)]
fn idct_8(v: &mut V, r: [usize; 8]) {
    let [r0, r1, r2, r3, r4, r5, r6, r7] = r;
    idct_4(v, r0, r2, r4, r6);

    v[5] = dup_4s(ROW_CLIP_MAX);
    v[4] = dup_4s(ROW_CLIP_MIN);
    for x in [r0, r2, r4, r6] {
        v[x] = smin_4s(v[x], v[5]);
    }
    for x in [r0, r2, r4, r6] {
        v[x] = smax_4s(v[x], v[4]);
    }

    v[2] = mul_mls::<0, 1>(v[r1], v[r7], v[1], v[1]); // -> t4a
    v[3] = mul_mla::<1, 0>(v[r1], v[r7], v[1], v[1]); // -> t7a
    v[6] = mul_mls::<2, 3>(v[r5], v[r3], v[1], v[1]); // -> t5a
    v[7] = mul_mla::<3, 2>(v[r5], v[r3], v[1], v[1]); // -> t6a
    v[r1] = srshr_4s::<12>(v[2]); // t4a
    v[r7] = srshr_4s::<12>(v[3]); // t7a
    v[r3] = srshr_4s::<12>(v[6]); // t5a
    v[r5] = srshr_4s::<12>(v[7]); // t6a

    v[2] = sqadd_4s(v[r1], v[r3]); // t4
    v[r1] = sqsub_4s(v[r1], v[r3]); // t5a
    v[3] = sqadd_4s(v[r7], v[r5]); // t7
    v[r3] = sqsub_4s(v[r7], v[r5]); // t6a

    for x in [2, r1, 3, r3] {
        v[x] = smin_4s(v[x], v[5]);
    }
    for x in [2, r1, 3, r3] {
        v[x] = smax_4s(v[x], v[4]);
    }

    v[7] = mul_mls::<0, 0>(v[r3], v[r1], v[0], v[0]); // -> t5
    v[6] = mul_mla::<0, 0>(v[r3], v[r1], v[0], v[0]); // -> t6
    v[7] = srshr_4s::<12>(v[7]); // t5
    v[6] = srshr_4s::<12>(v[6]); // t6

    v[r7] = sqsub_4s(v[r0], v[3]); // out7
    v[r0] = sqadd_4s(v[r0], v[3]); // out0
    v[r1] = sqadd_4s(v[r2], v[6]); // out1
    v[6] = sqsub_4s(v[r2], v[6]); // out6
    v[r2] = sqadd_4s(v[r4], v[7]); // out2
    v[r5] = sqsub_4s(v[r4], v[7]); // out5
    v[r3] = sqadd_4s(v[r6], v[2]); // out3
    v[r4] = sqsub_4s(v[r6], v[2]); // out4
    v[r6] = v[6]; // out6
}

/// `inv_dct_4s_x8_neon`
#[inline(always)]
fn inv_dct_4s_x8(v: &mut V) {
    // movrel x16, idct_coeffs; ld1 {v0.4s, v1.4s}, [x16]
    v[0] = coeffs(&IDCT_COEFFS, 0);
    v[1] = coeffs(&IDCT_COEFFS, 4);
    idct_8(v, [16, 17, 18, 19, 20, 21, 22, 23]);
}

/// `iadst_8 o0, …, o7`
#[inline(always)]
fn iadst_8(v: &mut V, o: [usize; 8]) {
    let [o0, o1, o2, o3, o4, o5, o6, o7] = o;
    // movrel x16, iadst8_coeffs; ld1 {v0.4s, v1.4s}, [x16], #32
    v[0] = coeffs(&IADST8_COEFFS, 0);
    v[1] = coeffs(&IADST8_COEFFS, 4);

    v[2] = mul_mla::<0, 1>(v[23], v[16], v[0], v[0]);
    v[4] = mul_mls::<1, 0>(v[23], v[16], v[0], v[0]);
    v[6] = mul_mla::<2, 3>(v[21], v[18], v[0], v[0]);
    v[16] = srshr_4s::<12>(v[2]); // t0a
    v[23] = srshr_4s::<12>(v[4]); // t1a
    v[2] = mul_mls::<3, 2>(v[21], v[18], v[0], v[0]);
    v[4] = mul_mla::<0, 1>(v[19], v[20], v[1], v[1]);
    v[18] = srshr_4s::<12>(v[6]); // t2a
    v[21] = srshr_4s::<12>(v[2]); // t3a
    v[6] = mul_mls::<1, 0>(v[19], v[20], v[1], v[1]);
    v[2] = mul_mla::<2, 3>(v[17], v[22], v[1], v[1]);
    v[20] = srshr_4s::<12>(v[4]); // t4a
    v[19] = srshr_4s::<12>(v[6]); // t5a
    v[4] = mul_mls::<3, 2>(v[17], v[22], v[1], v[1]);
    v[22] = srshr_4s::<12>(v[2]); // t6a
    v[17] = srshr_4s::<12>(v[4]); // t7a

    // ld1 {v0.4s}, [x16]
    v[0] = coeffs(&IADST8_COEFFS, 8);

    v[1] = dup_4s(ROW_CLIP_MAX);

    v[2] = sqadd_4s(v[16], v[20]); // t0
    v[3] = sqsub_4s(v[16], v[20]); // t4
    v[20] = dup_4s(ROW_CLIP_MIN);
    v[4] = sqadd_4s(v[23], v[19]); // t1
    v[5] = sqsub_4s(v[23], v[19]); // t5
    v[6] = sqadd_4s(v[18], v[22]); // t2
    v[7] = sqsub_4s(v[18], v[22]); // t6
    v[18] = sqadd_4s(v[21], v[17]); // t3
    v[19] = sqsub_4s(v[21], v[17]); // t7

    for r in [2, 3, 4, 5, 6, 7, 18, 19] {
        v[r] = smin_4s(v[r], v[1]);
    }
    for r in [2, 3, 4, 5, 6, 7, 18, 19] {
        v[r] = smax_4s(v[r], v[20]);
    }

    v[16] = mul_mla::<3, 2>(v[3], v[5], v[0], v[0]);
    v[20] = mul_mls::<2, 3>(v[3], v[5], v[0], v[0]);
    v[22] = mul_mls::<3, 2>(v[19], v[7], v[0], v[0]);

    v[3] = srshr_4s::<12>(v[16]); // t4a
    v[5] = srshr_4s::<12>(v[20]); // t5a

    v[16] = mul_mla::<2, 3>(v[19], v[7], v[0], v[0]);

    v[7] = srshr_4s::<12>(v[22]); // t6a
    v[19] = srshr_4s::<12>(v[16]); // t7a

    v[o0] = sqadd_4s(v[2], v[6]); // out0
    v[2] = sqsub_4s(v[2], v[6]); // t2
    v[o7] = sqadd_4s(v[4], v[18]); // out7
    v[4] = sqsub_4s(v[4], v[18]); // t3

    v[18] = dup_4s(ROW_CLIP_MIN);

    v[o1] = sqadd_4s(v[3], v[7]); // out1
    v[3] = sqsub_4s(v[3], v[7]); // t6
    v[o6] = sqadd_4s(v[5], v[19]); // out6
    v[5] = sqsub_4s(v[5], v[19]); // t7

    // Not clipping the output registers, as they will be downshifted and
    // narrowed afterwards anyway.
    for r in [2, 4, 3, 5] {
        v[r] = smin_4s(v[r], v[1]);
    }
    for r in [2, 4, 3, 5] {
        v[r] = smax_4s(v[r], v[18]);
    }

    v[o7] = sqneg_4s(v[o7]); // out7
    v[o1] = sqneg_4s(v[o1]); // out1

    v[18] = mul_mla::<0, 0>(v[2], v[4], v[0], v[0]); // -> out3 (v19 or v20)
    v[6] = mul_mls::<0, 0>(v[2], v[4], v[0], v[0]); // -> out4 (v20 or v19)
    v[20] = mul_mls::<0, 0>(v[3], v[5], v[0], v[0]); // -> out5 (v21 or v18)
    v[2] = srshr_4s::<12>(v[18]); // out3
    v[18] = mul_mla::<0, 0>(v[3], v[5], v[0], v[0]); // -> out2 (v18 or v21)
    v[3] = srshr_4s::<12>(v[20]); // out5
    v[o2] = srshr_4s::<12>(v[18]); // out2 (v18 or v21)
    v[o4] = srshr_4s::<12>(v[6]); // out4 (v20 or v19)

    v[o3] = sqneg_4s(v[2]); // out3
    v[o5] = sqneg_4s(v[3]); // out5
}

/// `inv_adst_4s_x8_neon`
#[inline(always)]
fn inv_adst_4s_x8(v: &mut V) {
    iadst_8(v, [16, 17, 18, 19, 20, 21, 22, 23]);
}

/// `inv_flipadst_4s_x8_neon`
#[inline(always)]
fn inv_flipadst_4s_x8(v: &mut V) {
    iadst_8(v, [23, 22, 21, 20, 19, 18, 17, 16]);
}

/// `inv_identity_4s_x8_neon`
#[inline(always)]
fn inv_identity_4s_x8(v: &mut V) {
    for r in 16..24 {
        v[r] = sqshl1_4s(v[r]);
    }
}

/// `inv_dct_4s_x16_neon`; leaves row_clip_max in `v5`, row_clip_min in
/// `v4` and idct_coeffs[0..4] in `v0`.
#[inline(always)]
fn inv_dct_4s_x16(v: &mut V) {
    // movrel x16, idct_coeffs; ld1 {v0.4s, v1.4s}, [x16], #32
    v[0] = coeffs(&IDCT_COEFFS, 0);
    v[1] = coeffs(&IDCT_COEFFS, 4);

    idct_8(v, [16, 18, 20, 22, 24, 26, 28, 30]);

    // idct_8 leaves the row_clip_max/min constants in v5 and v4
    for r in [16, 18, 20, 22, 24, 26, 28, 30] {
        v[r] = smin_4s(v[r], v[5]);
    }
    for r in [16, 18, 20, 22, 24, 26, 28, 30] {
        v[r] = smax_4s(v[r], v[4]);
    }

    // ldp q0, q1, [x16], #-32
    v[0] = coeffs(&IDCT_COEFFS, 8);
    v[1] = coeffs(&IDCT_COEFFS, 12);

    v[2] = mul_mls::<0, 1>(v[17], v[31], v[0], v[0]); // -> t8a
    v[3] = mul_mla::<1, 0>(v[17], v[31], v[0], v[0]); // -> t15a
    v[6] = mul_mls::<2, 3>(v[25], v[23], v[0], v[0]); // -> t9a
    v[17] = srshr_4s::<12>(v[2]); // t8a
    v[31] = srshr_4s::<12>(v[3]); // t15a
    v[2] = mul_mla::<3, 2>(v[25], v[23], v[0], v[0]); // -> t14a
    v[3] = mul_mls::<0, 1>(v[21], v[27], v[1], v[1]); // -> t10a
    v[23] = srshr_4s::<12>(v[6]); // t9a
    v[25] = srshr_4s::<12>(v[2]); // t14a
    v[6] = mul_mla::<1, 0>(v[21], v[27], v[1], v[1]); // -> t13a
    v[2] = mul_mls::<2, 3>(v[29], v[19], v[1], v[1]); // -> t11a
    v[21] = srshr_4s::<12>(v[3]); // t10a
    v[27] = srshr_4s::<12>(v[6]); // t13a
    v[3] = mul_mla::<3, 2>(v[29], v[19], v[1], v[1]); // -> t12a
    v[19] = srshr_4s::<12>(v[2]); // t11a
    v[29] = srshr_4s::<12>(v[3]); // t12a

    // ld1 {v0.4s}, [x16]
    v[0] = coeffs(&IDCT_COEFFS, 0);

    v[2] = sqsub_4s(v[17], v[23]); // t9
    v[17] = sqadd_4s(v[17], v[23]); // t8
    v[3] = sqsub_4s(v[31], v[25]); // t14
    v[31] = sqadd_4s(v[31], v[25]); // t15
    v[23] = sqsub_4s(v[19], v[21]); // t10
    v[19] = sqadd_4s(v[19], v[21]); // t11
    v[25] = sqadd_4s(v[29], v[27]); // t12
    v[29] = sqsub_4s(v[29], v[27]); // t13

    for r in [2, 17, 3, 31, 23, 19, 25, 29] {
        v[r] = smin_4s(v[r], v[5]);
    }
    for r in [2, 17, 3, 31, 23, 19, 25, 29] {
        v[r] = smax_4s(v[r], v[4]);
    }

    v[7] = mul_mls::<2, 3>(v[3], v[2], v[0], v[0]); // -> t9a
    v[6] = mul_mla::<3, 2>(v[3], v[2], v[0], v[0]); // -> t14a
    v[21] = srshr_4s::<12>(v[7]); // t9a
    v[27] = srshr_4s::<12>(v[6]); // t14a

    v[7] = mul_mls::<2, 3>(v[29], v[23], v[0], v[0]); // -> t13a
    v[6] = mul_mla::<3, 2>(v[29], v[23], v[0], v[0]); // -> t10a
    v[29] = srshr_4s::<12>(v[7]); // t13a
    v[6] = neg_4s(v[6]);
    v[23] = srshr_4s::<12>(v[6]); // t10a

    v[2] = sqsub_4s(v[17], v[19]); // t11a
    v[17] = sqadd_4s(v[17], v[19]); // t8a
    v[3] = sqsub_4s(v[31], v[25]); // t12a
    v[31] = sqadd_4s(v[31], v[25]); // t15a
    v[19] = sqadd_4s(v[21], v[23]); // t9
    v[21] = sqsub_4s(v[21], v[23]); // t10
    v[25] = sqsub_4s(v[27], v[29]); // t13
    v[27] = sqadd_4s(v[27], v[29]); // t14

    for r in [2, 17, 3, 31, 19, 21, 25, 27] {
        v[r] = smin_4s(v[r], v[5]);
    }
    for r in [2, 17, 3, 31, 19, 21, 25, 27] {
        v[r] = smax_4s(v[r], v[4]);
    }

    v[7] = mul_mls::<0, 0>(v[3], v[2], v[0], v[0]); // -> t11
    v[6] = mul_mla::<0, 0>(v[3], v[2], v[0], v[0]); // -> t12
    v[2] = mul_mls::<0, 0>(v[25], v[21], v[0], v[0]); // -> t10a

    v[7] = srshr_4s::<12>(v[7]); // t11
    v[6] = srshr_4s::<12>(v[6]); // t12
    v[3] = mul_mla::<0, 0>(v[25], v[21], v[0], v[0]); // -> t13a
    v[2] = srshr_4s::<12>(v[2]); // t10a
    v[3] = srshr_4s::<12>(v[3]); // t13a

    v[1] = sqadd_4s(v[16], v[31]); // out0
    v[31] = sqsub_4s(v[16], v[31]); // out15
    v[16] = v[1];
    v[23] = sqadd_4s(v[30], v[17]); // out7
    v[1] = sqsub_4s(v[30], v[17]); // out8
    v[17] = sqadd_4s(v[18], v[27]); // out1
    v[30] = sqsub_4s(v[18], v[27]); // out14
    v[18] = sqadd_4s(v[20], v[3]); // out2
    v[29] = sqsub_4s(v[20], v[3]); // out13
    v[3] = sqadd_4s(v[28], v[19]); // out6
    v[25] = sqsub_4s(v[28], v[19]); // out9
    v[19] = sqadd_4s(v[22], v[6]); // out3
    v[28] = sqsub_4s(v[22], v[6]); // out12
    v[20] = sqadd_4s(v[24], v[7]); // out4
    v[27] = sqsub_4s(v[24], v[7]); // out11
    v[21] = sqadd_4s(v[26], v[2]); // out5
    v[26] = sqsub_4s(v[26], v[2]); // out10
    v[24] = v[1];
    v[22] = v[3];
}

/// `iadst_16 o0, …, o15` (`.ifc \o0, v16` and `.ifc \o8, v23` select the
/// register moves of the flipped form).
#[inline(always)]
fn iadst_16(v: &mut V, o: [usize; 16]) {
    // movrel x16, iadst16_coeffs; ld1 {v0.4s, v1.4s}, [x16], #32
    v[0] = coeffs(&IADST16_COEFFS, 0);
    v[1] = coeffs(&IADST16_COEFFS, 4);

    v[2] = mul_mla::<0, 1>(v[31], v[16], v[0], v[0]); // -> t0
    v[4] = mul_mls::<1, 0>(v[31], v[16], v[0], v[0]); // -> t1
    v[6] = mul_mla::<2, 3>(v[29], v[18], v[0], v[0]); // -> t2
    v[16] = srshr_4s::<12>(v[2]); // t0
    v[31] = srshr_4s::<12>(v[4]); // t1
    v[2] = mul_mls::<3, 2>(v[29], v[18], v[0], v[0]); // -> t3
    v[4] = mul_mla::<0, 1>(v[27], v[20], v[1], v[1]); // -> t4
    v[18] = srshr_4s::<12>(v[6]); // t2
    v[29] = srshr_4s::<12>(v[2]); // t3
    v[6] = mul_mls::<1, 0>(v[27], v[20], v[1], v[1]); // -> t5
    v[2] = mul_mla::<2, 3>(v[25], v[22], v[1], v[1]); // -> t6
    v[20] = srshr_4s::<12>(v[4]); // t4
    v[27] = srshr_4s::<12>(v[6]); // t5
    v[4] = mul_mls::<3, 2>(v[25], v[22], v[1], v[1]); // -> t7
    // ld1 {v0.4s, v1.4s}, [x16]; movrel x16, idct_coeffs
    v[0] = coeffs(&IADST16_COEFFS, 8);
    v[1] = coeffs(&IADST16_COEFFS, 12);
    v[6] = mul_mla::<0, 1>(v[23], v[24], v[0], v[0]); // -> t8
    v[22] = srshr_4s::<12>(v[2]); // t6
    v[25] = srshr_4s::<12>(v[4]); // t7
    v[2] = mul_mls::<1, 0>(v[23], v[24], v[0], v[0]); // -> t9
    v[4] = mul_mla::<2, 3>(v[21], v[26], v[0], v[0]); // -> t10
    v[23] = srshr_4s::<12>(v[6]); // t8
    v[24] = srshr_4s::<12>(v[2]); // t9
    v[6] = mul_mls::<3, 2>(v[21], v[26], v[0], v[0]); // -> t11
    v[2] = mul_mla::<0, 1>(v[19], v[28], v[1], v[1]); // -> t12
    v[21] = srshr_4s::<12>(v[4]); // t10
    v[26] = srshr_4s::<12>(v[6]); // t11
    v[4] = mul_mls::<1, 0>(v[19], v[28], v[1], v[1]); // -> t13
    v[6] = mul_mla::<2, 3>(v[17], v[30], v[1], v[1]); // -> t14
    v[19] = srshr_4s::<12>(v[2]); // t12
    v[28] = srshr_4s::<12>(v[4]); // t13
    v[2] = mul_mls::<3, 2>(v[17], v[30], v[1], v[1]); // -> t15
    v[17] = srshr_4s::<12>(v[6]); // t14
    v[30] = srshr_4s::<12>(v[2]); // t15

    // ld1 {v0.4s, v1.4s}, [x16]
    v[0] = coeffs(&IDCT_COEFFS, 0);
    v[1] = coeffs(&IDCT_COEFFS, 4);

    v[5] = dup_4s(ROW_CLIP_MAX);
    v[7] = dup_4s(ROW_CLIP_MIN);

    v[2] = sqsub_4s(v[16], v[23]); // t8a
    v[16] = sqadd_4s(v[16], v[23]); // t0a
    v[3] = sqsub_4s(v[31], v[24]); // t9a
    v[31] = sqadd_4s(v[31], v[24]); // t1a
    v[23] = sqadd_4s(v[18], v[21]); // t2a
    v[18] = sqsub_4s(v[18], v[21]); // t10a
    v[24] = sqadd_4s(v[29], v[26]); // t3a
    v[29] = sqsub_4s(v[29], v[26]); // t11a
    v[21] = sqadd_4s(v[20], v[19]); // t4a
    v[20] = sqsub_4s(v[20], v[19]); // t12a
    v[26] = sqadd_4s(v[27], v[28]); // t5a
    v[27] = sqsub_4s(v[27], v[28]); // t13a
    v[19] = sqadd_4s(v[22], v[17]); // t6a
    v[22] = sqsub_4s(v[22], v[17]); // t14a
    v[28] = sqadd_4s(v[25], v[30]); // t7a
    v[25] = sqsub_4s(v[25], v[30]); // t15a

    let clip1 = [2, 16, 3, 31, 23, 18, 24, 29, 21, 20, 26, 27, 19, 22, 28, 25];
    for r in clip1 {
        v[r] = smin_4s(v[r], v[5]);
    }
    for r in clip1 {
        v[r] = smax_4s(v[r], v[7]);
    }

    v[4] = mul_mla::<1, 0>(v[2], v[3], v[1], v[1]); // -> t8
    v[6] = mul_mls::<0, 1>(v[2], v[3], v[1], v[1]); // -> t9
    v[2] = mul_mla::<3, 2>(v[18], v[29], v[1], v[1]); // -> t10
    v[17] = srshr_4s::<12>(v[4]); // t8
    v[30] = srshr_4s::<12>(v[6]); // t9
    v[4] = mul_mls::<2, 3>(v[18], v[29], v[1], v[1]); // -> t11
    v[6] = mul_mls::<1, 0>(v[27], v[20], v[1], v[1]); // -> t12
    v[18] = srshr_4s::<12>(v[2]); // t10
    v[29] = srshr_4s::<12>(v[4]); // t11
    v[2] = mul_mla::<0, 1>(v[27], v[20], v[1], v[1]); // -> t13
    v[4] = mul_mls::<3, 2>(v[25], v[22], v[1], v[1]); // -> t14
    v[27] = srshr_4s::<12>(v[6]); // t12
    v[20] = srshr_4s::<12>(v[2]); // t13
    v[6] = mul_mla::<2, 3>(v[25], v[22], v[1], v[1]); // -> t15
    v[25] = srshr_4s::<12>(v[4]); // t14
    v[22] = srshr_4s::<12>(v[6]); // t15

    v[2] = sqsub_4s(v[16], v[21]); // t4
    v[16] = sqadd_4s(v[16], v[21]); // t0
    v[3] = sqsub_4s(v[31], v[26]); // t5
    v[31] = sqadd_4s(v[31], v[26]); // t1
    v[21] = sqadd_4s(v[23], v[19]); // t2
    v[23] = sqsub_4s(v[23], v[19]); // t6
    v[26] = sqadd_4s(v[24], v[28]); // t3
    v[24] = sqsub_4s(v[24], v[28]); // t7
    v[19] = sqadd_4s(v[17], v[27]); // t8a
    v[17] = sqsub_4s(v[17], v[27]); // t12a
    v[28] = sqadd_4s(v[30], v[20]); // t9a
    v[30] = sqsub_4s(v[30], v[20]); // t13a
    v[27] = sqadd_4s(v[18], v[25]); // t10a
    v[18] = sqsub_4s(v[18], v[25]); // t14a
    v[20] = sqadd_4s(v[29], v[22]); // t11a
    v[29] = sqsub_4s(v[29], v[22]); // t15a

    let clip2 = [2, 16, 3, 31, 21, 23, 26, 24, 19, 17, 28, 30, 27, 18, 20, 29];
    for r in clip2 {
        v[r] = smin_4s(v[r], v[5]);
    }
    for r in clip2 {
        v[r] = smax_4s(v[r], v[7]);
    }

    v[4] = mul_mla::<3, 2>(v[2], v[3], v[0], v[0]); // -> t4a
    v[6] = mul_mls::<2, 3>(v[2], v[3], v[0], v[0]); // -> t5a
    v[2] = mul_mls::<3, 2>(v[24], v[23], v[0], v[0]); // -> t6a
    v[22] = srshr_4s::<12>(v[4]); // t4a
    v[25] = srshr_4s::<12>(v[6]); // t5a
    v[4] = mul_mla::<2, 3>(v[24], v[23], v[0], v[0]); // -> t7a
    v[6] = mul_mla::<3, 2>(v[17], v[30], v[0], v[0]); // -> t12
    v[24] = srshr_4s::<12>(v[2]); // t6a
    v[23] = srshr_4s::<12>(v[4]); // t7a
    v[2] = mul_mls::<2, 3>(v[17], v[30], v[0], v[0]); // -> t13
    v[4] = mul_mls::<3, 2>(v[29], v[18], v[0], v[0]); // -> t14
    v[17] = srshr_4s::<12>(v[6]); // t12
    v[6] = mul_mla::<2, 3>(v[29], v[18], v[0], v[0]); // -> t15
    v[29] = srshr_4s::<12>(v[2]); // t13
    v[30] = srshr_4s::<12>(v[4]); // t14
    v[18] = srshr_4s::<12>(v[6]); // t15

    v[2] = sqsub_4s(v[16], v[21]); // t2a
    if o[0] == 16 {
        v[o[0]] = sqadd_4s(v[16], v[21]); // out0
        v[21] = sqsub_4s(v[31], v[26]); // t3a
        v[o[15]] = sqadd_4s(v[31], v[26]); // out15
    } else {
        v[4] = sqadd_4s(v[16], v[21]); // out0
        v[21] = sqsub_4s(v[31], v[26]); // t3a
        v[o[15]] = sqadd_4s(v[31], v[26]); // out15
        v[o[0]] = v[4];
    }

    v[3] = sqsub_4s(v[29], v[18]); // t15a
    v[o[13]] = sqadd_4s(v[29], v[18]); // out13
    v[o[2]] = sqadd_4s(v[17], v[30]); // out2
    v[26] = sqsub_4s(v[17], v[30]); // t14a

    v[o[1]] = sqadd_4s(v[19], v[27]); // out1
    v[27] = sqsub_4s(v[19], v[27]); // t10
    v[o[14]] = sqadd_4s(v[28], v[20]); // out14
    v[20] = sqsub_4s(v[28], v[20]); // t11

    v[o[3]] = sqadd_4s(v[22], v[24]); // out3
    v[22] = sqsub_4s(v[22], v[24]); // t6
    v[o[12]] = sqadd_4s(v[25], v[23]); // out12
    v[23] = sqsub_4s(v[25], v[23]); // t7

    // Not clipping the output registers, as they will be downshifted and
    // narrowed afterwards anyway.
    let clip3 = [2, 21, 3, 26, 27, 20, 22, 23];
    for r in clip3 {
        v[r] = smin_4s(v[r], v[5]);
    }
    for r in clip3 {
        v[r] = smax_4s(v[r], v[7]);
    }

    v[o[15]] = sqneg_4s(v[o[15]]); // out15
    v[o[13]] = sqneg_4s(v[o[13]]); // out13
    v[o[1]] = sqneg_4s(v[o[1]]); // out1
    v[o[3]] = sqneg_4s(v[o[3]]); // out3

    v[24] = mul_mls::<0, 0>(v[2], v[21], v[0], v[0]); // -> out8 (v24 or v23)
    v[4] = mul_mla::<0, 0>(v[2], v[21], v[0], v[0]); // -> out7 (v23 or v24)
    v[6] = mul_mla::<0, 0>(v[26], v[3], v[0], v[0]); // -> out5 (v21 or v26)

    v[24] = srshr_4s::<12>(v[24]); // out8
    v[4] = srshr_4s::<12>(v[4]); // out7
    v[5] = srshr_4s::<12>(v[6]); // out5
    v[6] = mul_mls::<0, 0>(v[26], v[3], v[0], v[0]); // -> out10 (v26 or v21)
    v[2] = mul_mla::<0, 0>(v[22], v[23], v[0], v[0]); // -> out4 (v20 or v27)
    v[26] = srshr_4s::<12>(v[6]); // out10

    v[6] = mul_mls::<0, 0>(v[22], v[23], v[0], v[0]); // -> out11 (v27 or v20)
    v[22] = mul_mla::<0, 0>(v[27], v[20], v[0], v[0]); // -> out6 (v22 or v25)
    v[21] = mul_mls::<0, 0>(v[27], v[20], v[0], v[0]); // -> out9 (v25 or v22)

    v[o[4]] = srshr_4s::<12>(v[2]); // out4
    v[6] = srshr_4s::<12>(v[6]); // out11
    v[7] = srshr_4s::<12>(v[21]); // out9
    v[o[6]] = srshr_4s::<12>(v[22]); // out6

    if o[8] == 23 {
        v[o[8]] = v[24];
        v[o[10]] = v[26];
    }

    v[o[7]] = sqneg_4s(v[4]); // out7
    v[o[5]] = sqneg_4s(v[5]); // out5
    v[o[11]] = sqneg_4s(v[6]); // out11
    v[o[9]] = sqneg_4s(v[7]); // out9
}

/// `inv_adst_4s_x16_neon`
#[inline(always)]
fn inv_adst_4s_x16(v: &mut V) {
    iadst_16(v, [16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);
}

/// `inv_flipadst_4s_x16_neon`
#[inline(always)]
fn inv_flipadst_4s_x16(v: &mut V) {
    iadst_16(v, [31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16]);
}

/// `inv_identity_4s_x16_neon`
#[inline(always)]
fn inv_identity_4s_x16(v: &mut V) {
    // movz w16, #2*(5793-4096)*8, lsl #16; dup v0.2s, w16
    v[0] = dup_2s(2 * (5793 - 4096) * 8 * (1 << 16));
    for i in 16..32 {
        v[2] = sqrdmulh_4s::<0>(v[i], v[0]);
        v[i] = sqadd_4s(v[i], v[i]);
        v[i] = sqadd_4s(v[i], v[2]);
    }
}

/// `identity_4x16_shift1 c` with `c = rc.s[L]`, on v16–v31.
#[inline(always)]
fn identity_4x16_shift1<const L: i32>(v: &mut V, rc: int16x8_t) {
    for i in 16..32 {
        v[3] = sqrdmulh_4s::<L>(v[i], rc);
        v[3] = srshr_4s::<1>(v[3]);
        v[i] = sqadd_4s(v[i], v[3]);
    }
}

/// `identity_4x16 c` with `c = rc.s[L]`, on v16–v31.
#[inline(always)]
fn identity_4x16<const L: i32>(v: &mut V, rc: int16x8_t) {
    for i in 16..32 {
        v[3] = sqrdmulh_4s::<L>(v[i], rc);
        v[i] = sqadd_4s(v[i], v[i]);
        v[i] = sqadd_4s(v[i], v[3]);
    }
}

/// `blr` to `inv_<k>_4s_x<n>_neon` (the first-pass transform an entry point
/// loaded into `x4`/`x5`) on v16 onwards.
#[inline(always)]
fn call_4s(k: K, n: usize, v: &mut V) {
    match (k, n) {
        (K::Dct, 4) => inv_dct_4s_x4(v),
        (K::Adst, 4) => inv_adst_4s_x4(v),
        (K::Flip, 4) => inv_flipadst_4s_x4(v),
        (K::Id, 4) => inv_identity_4s_x4(v),
        (K::Dct, 8) => inv_dct_4s_x8(v),
        (K::Adst, 8) => inv_adst_4s_x8(v),
        (K::Flip, 8) => inv_flipadst_4s_x8(v),
        (K::Id, 8) => inv_identity_4s_x8(v),
        (K::Dct, _) => inv_dct_4s_x16(v),
        (K::Adst, _) => inv_adst_4s_x16(v),
        (K::Flip, _) => inv_flipadst_4s_x16(v),
        (K::Id, _) => inv_identity_4s_x16(v),
    }
}

// --- 4x4 -----------------------------------------------------------------------

/// `L(itx_4x4_end)`: `v0` holds rows 0, 1 and `v1` rows 2, 3 of the
/// destination, `v16`/`v18` the residuals; clamped against `bitdepth_max`
/// (`w4`).
#[inline(always)]
unsafe fn itx_4x4_end(x0: Dst, x1: isize, w4: c_int, v0: int16x8_t, v1: int16x8_t, v16: int16x8_t, v18: int16x8_t) {
    // dup v31.8h, w4
    let v31 = vdupq_n_s16(w4 as i16);
    let v0 = usqadd(v0, v16);
    let v1 = usqadd(v1, v18);
    let v0 = smin_8h(v0, v31);
    st_d2(x0, x1, v0);
    let v1 = smin_8h(v1, v31);
    st_d2(row(x0, x1, 2), x1, v1);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_wht_wht_4x4_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
    bitdepth_max: c_int,
) {
    let mut v = regs();
    v[30] = dup_4s(0);
    v[31] = dup_4s(0);
    for i in 0..4 {
        v[16 + i] = ld_4s(coeff.add(4 * i));
    }
    // st1 {v30.4s, v31.4s}, [x2], #32
    st_4s(coeff, v[30]);
    st_4s(coeff.add(4), v[31]);

    for i in 16..20 {
        v[i] = sshr_4s::<2>(v[i]);
    }

    iwht4(&mut v);

    st_4s(coeff.add(8), v[30]);
    st_4s(coeff.add(12), v[31]);
    transpose_4x4s(&mut v, [16, 17, 18, 19], [20, 21, 22, 23]);

    iwht4(&mut v);

    let v0 = ld_d2(dst, stride);
    v[16] = sqxtn(v[16]);
    v[16] = sqxtn2(v[16], v[17]);
    let v1 = ld_d2(row(dst, stride, 2), stride);
    v[18] = sqxtn(v[18]);
    v[18] = sqxtn2(v[18], v[19]);

    // b L(itx_4x4_end)
    itx_4x4_end(dst, stride, bitdepth_max, v0, v1, v[16], v[18]);
}

/// `inv_txfm_add_4x4_neon` (`x4` bitdepth_max, `x5` the first, `x6` the
/// second transform).
#[inline(always)]
unsafe fn inv_txfm_add_4x4(dst: Dst, stride: isize, coeff: Coef, w4: c_int, x5: K, x6: K) {
    let mut v = regs();
    v[30] = dup_4s(0);
    v[31] = dup_4s(0);
    for i in 0..4 {
        v[16 + i] = ld_4s(coeff.add(4 * i));
    }
    st_4s(coeff, v[30]);
    st_4s(coeff.add(4), v[31]);

    call_4s(x5, 4, &mut v);

    st_4s(coeff.add(8), v[30]);
    st_4s(coeff.add(12), v[31]);
    for i in 16..20 {
        v[i] = sqxtn(v[i]);
    }
    transpose_4x4h(&mut v, [16, 17, 18, 19], [20, 21, 22, 23]);

    call::<false>(x6, 4, &mut v);

    let v0 = ld_d2(dst, stride);
    v[16] = ins_hi_lo(v[16], v[17]);
    v[18] = ins_hi_lo(v[18], v[19]);
    let v1 = ld_d2(row(dst, stride, 2), stride);
    v[16] = srshr::<4>(v[16]);
    v[18] = srshr::<4>(v[18]);

    itx_4x4_end(dst, stride, w4, v0, v1, v[16], v[18]);
}

/// The DC-only path of `def_fn_4x4 dct, dct` (`eob == 0`).
#[inline(always)]
unsafe fn idct_dc_4x4(dst: Dst, stride: isize, coeff: Coef, w4: c_int) {
    // movz w16, #2896*8, lsl #16; ld1r {v16.4s}, [x2]; dup v4.2s, w16
    let v16 = h(vld1q_dup_s32(coeff));
    let v4 = dup_2s(W2896);
    // str wzr, [x2]
    *coeff = 0;
    let v16 = sqrdmulh_4s::<0>(v16, v4);
    let v0 = ld_d2(dst, stride);
    let v20 = sqxtn2(sqxtn(v16), v16);
    let v1 = ld_d2(row(dst, stride, 2), stride);
    let v20 = sqrdmulh_8h::<1>(v20, v4);
    let v16 = srshr::<4>(v20);
    let v18 = srshr::<4>(v20);
    // movi v30.8h, #0; b L(itx_4x4_end)
    itx_4x4_end(dst, stride, w4, v0, v1, v16, v18);
}

/// `def_fn_4x4 txfm1, txfm2`
#[inline(always)]
unsafe fn itx_4x4(dst: Dst, stride: isize, coeff: Coef, eob: c_int, bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc_4x4(dst, stride, coeff, bdmax);
    }
    inv_txfm_add_4x4(dst, stride, coeff, bdmax, t1, t2);
}

// --- 8x8 -----------------------------------------------------------------------

/// `inv_txfm_add_8x8_neon` (`x4` first, `x5` second transform, `w13` eob
/// threshold).
#[inline(always)]
unsafe fn inv_txfm_add_8x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, x4: K, x5: K, w13: c_int) {
    let mut v = regs();
    v[31] = dup_4s(0);

    // cmp w3, w13; mov x11, #32; b.lt 1f
    let x11 = 8;
    if eob >= w13 {
        let mut x6 = coeff.add(4);
        for i in 16..24 {
            v[i] = ld_4s(x6);
            st_4s(x6, v[31]);
            x6 = x6.add(x11);
        }

        call_4s(x4, 8, &mut v);

        v[24] = sqrshrn::<1>(v[16]);
        v[25] = sqrshrn::<1>(v[17]);
        v[26] = sqrshrn::<1>(v[18]);
        v[27] = sqrshrn::<1>(v[19]);
        v[24] = sqrshrn2::<1>(v[24], v[20]);
        v[25] = sqrshrn2::<1>(v[25], v[21]);
        v[26] = sqrshrn2::<1>(v[26], v[22]);
        v[27] = sqrshrn2::<1>(v[27], v[23]);

        transpose_4x8h(&mut v, [24, 25, 26, 27], [2, 3, 4, 5]);
    } else {
        // 1:
        for i in 24..28 {
            v[i] = dup_4s(0);
        }
    }

    // 2:
    let mut x2 = coeff;
    for i in 16..24 {
        v[i] = ld_4s(x2);
        st_4s(x2, v[31]);
        x2 = x2.add(x11);
    }

    call_4s(x4, 8, &mut v);

    v[16] = sqrshrn::<1>(v[16]);
    v[17] = sqrshrn::<1>(v[17]);
    v[18] = sqrshrn::<1>(v[18]);
    v[19] = sqrshrn::<1>(v[19]);
    v[16] = sqrshrn2::<1>(v[16], v[20]);
    v[17] = sqrshrn2::<1>(v[17], v[21]);
    v[18] = sqrshrn2::<1>(v[18], v[22]);
    v[19] = sqrshrn2::<1>(v[19], v[23]);

    transpose_4x8h(&mut v, [16, 17, 18, 19], [20, 21, 22, 23]);

    v[20] = v[24];
    v[21] = v[25];
    v[22] = v[26];
    v[23] = v[27];

    call::<true>(x5, 8, &mut v);

    load_add_store_8x8::<4>(dst, stride, &v);
}

/// `def_fn_8x8 txfm1, txfm2, eob_half`: the threshold.
#[inline(always)]
fn eob_half_8x8(t1: K, t2: K) -> c_int {
    match (t1, t2) {
        (K::Id, K::Id) => 10,
        (K::Id, _) | (_, K::Id) => 4,
        _ => 10,
    }
}

/// `def_fn_8x8 txfm1, txfm2, eob_half`
#[inline(always)]
unsafe fn itx_8x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<8, 8, 1>(dst, stride, coeff);
    }
    inv_txfm_add_8x8(dst, stride, coeff, eob, t1, t2, eob_half_8x8(t1, t2));
}

// --- 8x4, 4x8 ------------------------------------------------------------------

/// `inv_txfm_add_8x4_neon`: `x4` is `inv_*_4s_x8`, `x5` `inv_*_8h_x4`.
#[inline(always)]
unsafe fn inv_txfm_add_8x4(dst: Dst, stride: isize, coeff: Coef, x4: K, x5: K) {
    let mut v = regs();
    for i in 28..32 {
        v[i] = dup_4s(0);
    }
    for i in 0..4 {
        v[16 + i] = ld_4s(coeff.add(4 * i));
    }
    for i in 0..4 {
        st_4s(coeff.add(4 * i), v[28 + i]);
    }
    // movz w16, #2896*8, lsl #16; dup v0.2s, w16
    v[0] = dup_2s(W2896);
    for i in 0..4 {
        v[20 + i] = ld_4s(coeff.add(16 + 4 * i));
    }
    for i in 0..4 {
        st_4s(coeff.add(16 + 4 * i), v[28 + i]);
    }

    let c = v[0];
    scale_input::<0>(&mut v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);

    call_4s(x4, 8, &mut v);

    for i in 16..24 {
        v[i] = sqxtn(v[i]);
    }

    transpose_4x4h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);
    transpose_4x4h(&mut v, [20, 21, 22, 23], [4, 5, 6, 7]);
    v[16] = ins_hi_lo(v[16], v[20]);
    v[17] = ins_hi_lo(v[17], v[21]);
    v[18] = ins_hi_lo(v[18], v[22]);
    v[19] = ins_hi_lo(v[19], v[23]);

    call::<true>(x5, 4, &mut v);

    load_add_store_8x4::<4>(dst, stride, &v);
}

/// `inv_txfm_add_4x8_neon`: `x4` is `inv_*_4s_x4`, `x5` `inv_*_4h_x8`,
/// `w13` the eob threshold.
#[inline(always)]
unsafe fn inv_txfm_add_4x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, x4: K, x5: K, w13: c_int) {
    let mut v = regs();
    // movz w16, #2896*8, lsl #16; movi v31.4s, #0; dup v30.2s, w16
    v[31] = dup_4s(0);
    v[30] = dup_2s(W2896);

    // cmp w3, w13; mov x11, #32; b.lt 1f
    let x11 = 8;
    if eob >= w13 {
        let mut x6 = coeff.add(4);
        for i in 16..20 {
            v[i] = ld_4s(x6);
            st_4s(x6, v[31]);
            x6 = x6.add(x11);
        }
        let c = v[30];
        scale_input::<0>(&mut v, c, &[16, 17, 18, 19]);
        call_4s(x4, 4, &mut v);
        v[20] = sqxtn(v[16]);
        v[21] = sqxtn(v[17]);
        v[22] = sqxtn(v[18]);
        v[23] = sqxtn(v[19]);
        transpose_4x4h(&mut v, [20, 21, 22, 23], [4, 5, 6, 7]);
    } else {
        // 1: movi v20.4h–v23.4h, #0
        for i in 20..24 {
            v[i] = d(vdup_n_s16(0));
        }
    }

    // 2:
    let mut x2 = coeff;
    for i in 16..20 {
        v[i] = ld_4s(x2);
        st_4s(x2, v[31]);
        x2 = x2.add(x11);
    }
    let c = v[30];
    scale_input::<0>(&mut v, c, &[16, 17, 18, 19]);
    call_4s(x4, 4, &mut v);
    for i in 16..20 {
        v[i] = sqxtn(v[i]);
    }
    transpose_4x4h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);

    call::<false>(x5, 8, &mut v);

    load_add_store_4x8(dst, stride, &v);
}

/// `def_fns_48 w, h`: the `eob_half` of each type (used by 4x8 only).
#[inline(always)]
fn eob_half_48(t1: K, t2: K) -> c_int {
    match (t1, t2) {
        (K::Id, K::Id) => 13,
        (K::Id, _) => 16,
        (_, K::Id) => 4,
        _ => 13,
    }
}

/// `def_fn_48 4, 8, txfm1, txfm2, eob_half`
#[inline(always)]
unsafe fn itx_4x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<4, 8, 0>(dst, stride, coeff);
    }
    inv_txfm_add_4x8(dst, stride, coeff, eob, t1, t2, eob_half_48(t1, t2));
}

/// `def_fn_48 8, 4, txfm1, txfm2, eob_half` (no threshold for 8x4)
#[inline(always)]
unsafe fn itx_8x4(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<8, 4, 0>(dst, stride, coeff);
    }
    inv_txfm_add_8x4(dst, stride, coeff, t1, t2);
}

// --- 16x16 and the shared 16-point passes --------------------------------------

/// `inv_txfm_horz_16x4_neon` (`def_horz_16 scale=0, shift=2`) and
/// `inv_txfm_horz_scale_16x4_neon` (`SCALE`, shift 1): four columns of
/// coefficients from `x7` (16 rows, `x8` values apart, cleared after the
/// load) through `x4` into 64 values at `x6`.
#[inline(always)]
unsafe fn inv_txfm_horz_16x4<const SCALE: bool>(v: &mut V, x6: *mut i16, x7: Coef, x8: usize, x4: K) {
    v[7] = dup_4s(0);
    if SCALE {
        // movz w16, #2896*8, lsl #16; dup v0.2s, w16
        v[0] = dup_2s(W2896);
    }
    let mut x7 = x7;
    for i in 16..32 {
        v[i] = ld_4s(x7);
        st_4s(x7, v[7]);
        x7 = x7.add(x8);
    }
    if SCALE {
        let c = v[0];
        scale_input::<0>(v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
        scale_input::<0>(v, c, &[24, 25, 26, 27, 28, 29, 30, 31]);
    }
    call_4s(x4, 16, v);
    if SCALE {
        horz_16x4_narrow::<1>(v);
    } else {
        horz_16x4_narrow::<2>(v);
    }
    // L(horz_16x4_epilog):
    transpose_4x8h(v, [16, 17, 18, 19], [4, 5, 6, 7]);
    transpose_4x8h(v, [20, 21, 22, 23], [4, 5, 6, 7]);

    let order = [16, 20, 17, 21, 18, 22, 19, 23];
    for (k, &r) in order.iter().enumerate() {
        // st1 {vr.8h}, [x6], #16
        vst1q_s16(x6.add(8 * k), v[r]);
    }
}

/// The `sqrshrn`/`sqrshrn2 #shift` block of `def_horz_16`: v16–v31 (`.4s`)
/// into v16–v23 (`.8h`).
#[inline(always)]
fn horz_16x4_narrow<const SHIFT: i32>(v: &mut V) {
    v[16] = sqrshrn::<SHIFT>(v[16]);
    v[17] = sqrshrn::<SHIFT>(v[17]);
    v[18] = sqrshrn::<SHIFT>(v[18]);
    v[19] = sqrshrn::<SHIFT>(v[19]);
    v[16] = sqrshrn2::<SHIFT>(v[16], v[20]);
    v[17] = sqrshrn2::<SHIFT>(v[17], v[21]);
    v[18] = sqrshrn2::<SHIFT>(v[18], v[22]);
    v[19] = sqrshrn2::<SHIFT>(v[19], v[23]);
    v[20] = sqrshrn::<SHIFT>(v[24]);
    v[21] = sqrshrn::<SHIFT>(v[25]);
    v[22] = sqrshrn::<SHIFT>(v[26]);
    v[23] = sqrshrn::<SHIFT>(v[27]);
    v[20] = sqrshrn2::<SHIFT>(v[20], v[28]);
    v[21] = sqrshrn2::<SHIFT>(v[21], v[29]);
    v[22] = sqrshrn2::<SHIFT>(v[22], v[30]);
    v[23] = sqrshrn2::<SHIFT>(v[23], v[31]);
}

/// `inv_txfm_add_vert_8x16_neon`: 16 rows of 8 values from `x7` (`x8`
/// values apart), `x5` (`inv_*_8h_x16`), `load_add_store_8x16` to `x6`.
#[inline(always)]
unsafe fn inv_txfm_add_vert_8x16(v: &mut V, x6: Dst, stride: isize, x7: *const i16, x8: usize, x5: K) {
    for i in 0..16 {
        v[16 + i] = vld1q_s16(x7.add(x8 * i));
    }
    call::<true>(x5, 16, v);
    load_add_store_8x16(x6, stride, v);
}

/// `inv_txfm_add_16x16_neon` with `x4`/`x5` the transforms and `x13` the eob
/// table.
#[inline(always)]
unsafe fn inv_txfm_add_16x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, x4: K, x5: K, x13: &[u16; 4]) {
    let mut v = regs();
    // sub sp, sp, #512
    let mut buf = [0i16; 256];
    let sp = buf.as_mut_ptr();
    // ldrh w12, [x13], #2
    let mut x13i = 0;
    let mut w12 = x13[x13i] as c_int;
    x13i += 1;
    'horz: {
        for i in [0usize, 4, 8, 12] {
            let x6 = sp.add(i * 16);
            if i > 0 {
                let w8 = (16 - i) as i32;
                if eob < w12 {
                    // 1: movi v4.8h–v7.8h, #0
                    // 2: subs w8, w8, #4; st1 ×2 (64 values); b.gt 2b
                    let mut x6 = x6;
                    let mut w8 = w8;
                    loop {
                        w8 -= 4;
                        for _ in 0..8 {
                            vst1q_s16(x6, vdupq_n_s16(0));
                            x6 = x6.add(8);
                        }
                        if w8 <= 0 {
                            break;
                        }
                    }
                    break 'horz;
                }
                if i < 12 {
                    w12 = x13[x13i] as c_int;
                    x13i += 1;
                }
            }
            // add x7, x2, #(i*4); mov x8, #16*4
            inv_txfm_horz_16x4::<false>(&mut v, x6, coeff.add(i), 16, x4);
        }
    }
    // 3:
    for i in [0usize, 8] {
        inv_txfm_add_vert_8x16(&mut v, dst.add(i), stride, sp.add(i), 16, x5);
    }
    // add sp, sp, #512
}

/// `def_fn_16x16 txfm1, txfm2`
#[inline(always)]
unsafe fn itx_16x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<16, 16, 2>(dst, stride, coeff);
    }
    let x13 = match (t1, t2) {
        (K::Id, K::Id) => &EOB_16X16,
        (K::Id, _) | (_, K::Id) => &EOB_16X16_IDENTITY,
        _ => &EOB_16X16,
    };
    inv_txfm_add_16x16(dst, stride, coeff, eob, t1, t2, x13);
}

// --- 16x4, 4x16 ----------------------------------------------------------------

/// `inv_txfm_add_16x4_neon`: `x4` is `inv_*_4s_x16`, `x5` `inv_*_8h_x4`.
#[inline(always)]
unsafe fn inv_txfm_add_16x4(dst: Dst, stride: isize, coeff: Coef, x4: K, x5: K) {
    let mut v = regs();
    v[4] = dup_4s(0);

    for i in 0..16 {
        v[16 + i] = ld_4s(coeff.add(4 * i));
        st_4s(coeff.add(4 * i), v[4]);
    }

    call_4s(x4, 16, &mut v);

    v[16] = sqrshrn::<1>(v[16]);
    v[17] = sqrshrn::<1>(v[17]);
    v[18] = sqrshrn::<1>(v[18]);
    v[19] = sqrshrn::<1>(v[19]);
    v[16] = sqrshrn2::<1>(v[16], v[20]);
    v[17] = sqrshrn2::<1>(v[17], v[21]);
    v[18] = sqrshrn2::<1>(v[18], v[22]);
    v[19] = sqrshrn2::<1>(v[19], v[23]);
    transpose_4x8h(&mut v, [16, 17, 18, 19], [2, 3, 4, 5]);
    call::<true>(x5, 4, &mut v);
    // mov x6, x0
    load_add_store_8x4::<4>(dst, stride, &v);

    v[16] = sqrshrn::<1>(v[24]);
    v[17] = sqrshrn::<1>(v[25]);
    v[18] = sqrshrn::<1>(v[26]);
    v[19] = sqrshrn::<1>(v[27]);
    v[16] = sqrshrn2::<1>(v[16], v[28]);
    v[17] = sqrshrn2::<1>(v[17], v[29]);
    v[18] = sqrshrn2::<1>(v[18], v[30]);
    v[19] = sqrshrn2::<1>(v[19], v[31]);
    transpose_4x8h(&mut v, [16, 17, 18, 19], [2, 3, 4, 5]);
    call::<true>(x5, 4, &mut v);
    // add x6, x0, #16
    load_add_store_8x4::<4>(dst.add(8), stride, &v);
}

/// One column group of `inv_txfm_add_4x16_neon`: four rows of four
/// coefficients from `x6` (16 values apart, cleared), `x4`, `sqrshrn #1`
/// into `o`, `transpose_4x4h o, v4–v7`.
#[inline(always)]
unsafe fn inv_txfm_add_4x16_part(v: &mut V, x6: Coef, x4: K, o: [usize; 4]) {
    // movi v2.4s, #0
    v[2] = dup_4s(0);
    let x11 = 16;
    let mut x6 = x6;
    for i in 16..20 {
        v[i] = ld_4s(x6);
        st_4s(x6, v[2]);
        x6 = x6.add(x11);
    }
    call_4s(x4, 4, v);
    for k in 0..4 {
        v[o[k]] = sqrshrn::<1>(v[16 + k]);
    }
    transpose_4x4h(v, o, [4, 5, 6, 7]);
}

/// `inv_txfm_add_4x16_neon`: `x4` is `inv_*_4s_x4`, `x5` `inv_*_4h_x16`,
/// `x13` the eob table.
#[inline(always)]
unsafe fn inv_txfm_add_4x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, x4: K, x5: K, x13: &[u16; 4]) {
    let mut v = regs();
    // ldrh w12, [x13, #4]; mov x11, #64; cmp w3, w12; ldrh w12, [x13, #2]
    let mut w12 = x13[2] as c_int;
    let lt = eob < w12;
    w12 = x13[1] as c_int;
    if !lt {
        inv_txfm_add_4x16_part(&mut v, coeff.add(12), x4, [28, 29, 30, 31]);
    } else {
        // 1: movi v28.4h–v31.4h, #0
        for i in 28..32 {
            v[i] = d(vdup_n_s16(0));
        }
    }
    // 2: cmp w3, w12; ldrh w12, [x13, #0]
    let lt = eob < w12;
    w12 = x13[0] as c_int;
    if !lt {
        inv_txfm_add_4x16_part(&mut v, coeff.add(8), x4, [24, 25, 26, 27]);
    } else {
        for i in 24..28 {
            v[i] = d(vdup_n_s16(0));
        }
    }
    // 2: cmp w3, w12
    if eob >= w12 {
        inv_txfm_add_4x16_part(&mut v, coeff.add(4), x4, [20, 21, 22, 23]);
    } else {
        for i in 20..24 {
            v[i] = d(vdup_n_s16(0));
        }
    }
    // 2:
    v[2] = dup_4s(0);
    let mut x2 = coeff;
    for i in 16..20 {
        v[i] = ld_4s(x2);
        st_4s(x2, v[2]);
        x2 = x2.add(16);
    }
    call_4s(x4, 4, &mut v);
    for i in 16..20 {
        v[i] = sqrshrn::<1>(v[i]);
    }
    transpose_4x8h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);

    call::<false>(x5, 16, &mut v);

    load_add_store_4x16(dst, stride, &v);
}

/// `def_fn_416 4, 16, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_4x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<4, 16, 1>(dst, stride, coeff);
    }
    let x13 = match (t1, t2) {
        (K::Id, K::Id) => &EOB_4X16,
        (K::Id, _) => &EOB_4X16_IDENTITY1,
        (_, K::Id) => &EOB_4X16_IDENTITY2,
        _ => &EOB_4X16,
    };
    inv_txfm_add_4x16(dst, stride, coeff, eob, t1, t2, x13);
}

/// `def_fn_416 16, 4, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_16x4(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<16, 4, 1>(dst, stride, coeff);
    }
    inv_txfm_add_16x4(dst, stride, coeff, t1, t2);
}

// --- entry points (the def_fn_* lists of itx16.S) --------------------------------

macro_rules! def_fns {
    ($($name:ident = $itx:ident($t1:ident, $t2:ident);)*) => {$(
        pub unsafe extern "C" fn $name(
            dst: Dst,
            stride: isize,
            coeff: Coef,
            eob: c_int,
            bitdepth_max: c_int,
        ) {
            $itx(dst, stride, coeff, eob, bitdepth_max, K::$t1, K::$t2)
        }
    )*};
}

def_fns! {
    dav1d_inv_txfm_add_dct_dct_4x4_16bpc_neon = itx_4x4(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_4x4_16bpc_neon = itx_4x4(Id, Id);
    dav1d_inv_txfm_add_dct_adst_4x4_16bpc_neon = itx_4x4(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_4x4_16bpc_neon = itx_4x4(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_4x4_16bpc_neon = itx_4x4(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_4x4_16bpc_neon = itx_4x4(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_4x4_16bpc_neon = itx_4x4(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_4x4_16bpc_neon = itx_4x4(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_4x4_16bpc_neon = itx_4x4(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_4x4_16bpc_neon = itx_4x4(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_4x4_16bpc_neon = itx_4x4(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_4x4_16bpc_neon = itx_4x4(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_4x4_16bpc_neon = itx_4x4(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_4x4_16bpc_neon = itx_4x4(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_4x4_16bpc_neon = itx_4x4(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_4x4_16bpc_neon = itx_4x4(Id, Flip);

    dav1d_inv_txfm_add_dct_dct_8x8_16bpc_neon = itx_8x8(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_8x8_16bpc_neon = itx_8x8(Id, Id);
    dav1d_inv_txfm_add_dct_adst_8x8_16bpc_neon = itx_8x8(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_8x8_16bpc_neon = itx_8x8(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_8x8_16bpc_neon = itx_8x8(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_8x8_16bpc_neon = itx_8x8(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_8x8_16bpc_neon = itx_8x8(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_8x8_16bpc_neon = itx_8x8(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_8x8_16bpc_neon = itx_8x8(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_8x8_16bpc_neon = itx_8x8(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_8x8_16bpc_neon = itx_8x8(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_8x8_16bpc_neon = itx_8x8(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_8x8_16bpc_neon = itx_8x8(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_8x8_16bpc_neon = itx_8x8(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_8x8_16bpc_neon = itx_8x8(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_8x8_16bpc_neon = itx_8x8(Id, Flip);

    dav1d_inv_txfm_add_dct_dct_4x8_16bpc_neon = itx_4x8(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_4x8_16bpc_neon = itx_4x8(Id, Id);
    dav1d_inv_txfm_add_dct_adst_4x8_16bpc_neon = itx_4x8(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_4x8_16bpc_neon = itx_4x8(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_4x8_16bpc_neon = itx_4x8(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_4x8_16bpc_neon = itx_4x8(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_4x8_16bpc_neon = itx_4x8(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_4x8_16bpc_neon = itx_4x8(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_4x8_16bpc_neon = itx_4x8(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_4x8_16bpc_neon = itx_4x8(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_4x8_16bpc_neon = itx_4x8(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_4x8_16bpc_neon = itx_4x8(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_4x8_16bpc_neon = itx_4x8(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_4x8_16bpc_neon = itx_4x8(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_4x8_16bpc_neon = itx_4x8(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_4x8_16bpc_neon = itx_4x8(Id, Flip);

    dav1d_inv_txfm_add_dct_dct_8x4_16bpc_neon = itx_8x4(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_8x4_16bpc_neon = itx_8x4(Id, Id);
    dav1d_inv_txfm_add_dct_adst_8x4_16bpc_neon = itx_8x4(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_8x4_16bpc_neon = itx_8x4(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_8x4_16bpc_neon = itx_8x4(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_8x4_16bpc_neon = itx_8x4(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_8x4_16bpc_neon = itx_8x4(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_8x4_16bpc_neon = itx_8x4(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_8x4_16bpc_neon = itx_8x4(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_8x4_16bpc_neon = itx_8x4(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_8x4_16bpc_neon = itx_8x4(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_8x4_16bpc_neon = itx_8x4(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_8x4_16bpc_neon = itx_8x4(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_8x4_16bpc_neon = itx_8x4(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_8x4_16bpc_neon = itx_8x4(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_8x4_16bpc_neon = itx_8x4(Id, Flip);

    dav1d_inv_txfm_add_dct_dct_16x16_16bpc_neon = itx_16x16(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_16x16_16bpc_neon = itx_16x16(Id, Id);
    dav1d_inv_txfm_add_dct_adst_16x16_16bpc_neon = itx_16x16(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_16x16_16bpc_neon = itx_16x16(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_16x16_16bpc_neon = itx_16x16(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_16x16_16bpc_neon = itx_16x16(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_16x16_16bpc_neon = itx_16x16(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_16x16_16bpc_neon = itx_16x16(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_16x16_16bpc_neon = itx_16x16(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_16x16_16bpc_neon = itx_16x16(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_16x16_16bpc_neon = itx_16x16(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_16x16_16bpc_neon = itx_16x16(Id, Dct);

    dav1d_inv_txfm_add_dct_dct_4x16_16bpc_neon = itx_4x16(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_4x16_16bpc_neon = itx_4x16(Id, Id);
    dav1d_inv_txfm_add_dct_adst_4x16_16bpc_neon = itx_4x16(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_4x16_16bpc_neon = itx_4x16(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_4x16_16bpc_neon = itx_4x16(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_4x16_16bpc_neon = itx_4x16(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_4x16_16bpc_neon = itx_4x16(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_4x16_16bpc_neon = itx_4x16(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_4x16_16bpc_neon = itx_4x16(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_4x16_16bpc_neon = itx_4x16(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_4x16_16bpc_neon = itx_4x16(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_4x16_16bpc_neon = itx_4x16(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_4x16_16bpc_neon = itx_4x16(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_4x16_16bpc_neon = itx_4x16(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_4x16_16bpc_neon = itx_4x16(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_4x16_16bpc_neon = itx_4x16(Id, Flip);

    dav1d_inv_txfm_add_dct_dct_16x4_16bpc_neon = itx_16x4(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_16x4_16bpc_neon = itx_16x4(Id, Id);
    dav1d_inv_txfm_add_dct_adst_16x4_16bpc_neon = itx_16x4(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_16x4_16bpc_neon = itx_16x4(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_16x4_16bpc_neon = itx_16x4(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_16x4_16bpc_neon = itx_16x4(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_16x4_16bpc_neon = itx_16x4(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_16x4_16bpc_neon = itx_16x4(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_16x4_16bpc_neon = itx_16x4(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_16x4_16bpc_neon = itx_16x4(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_16x4_16bpc_neon = itx_16x4(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_16x4_16bpc_neon = itx_16x4(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_16x4_16bpc_neon = itx_16x4(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_16x4_16bpc_neon = itx_16x4(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_16x4_16bpc_neon = itx_16x4(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_16x4_16bpc_neon = itx_16x4(Id, Flip);
}

// =============================================================================
// Lines 1699 to 3652 of itx16.S: 16x8, 8x16, the 32- and 64-point transforms.
//
// Additional notes on the correspondence (on top of the module header):
// - The stack buffers (`sub sp`, `sub_sp`) are local `i32` arrays, so that the
//   `.4s` and `.8h` views of the same bytes (`def_horz_32` stores its first
//   half as `.4s` and overwrites it as `.8h`; the 64-point scratch is first
//   64 `.4s` vectors, then 64 `.8h` vectors of `inv_txfm_dct_8h_x64_neon`)
//   are both aligned. Every element is written before it is read.
// - Pointer registers that the assembly moves past the end of a block between
//   accesses (the identity loops, `inv_txfm_horz_dct_64x4_neon`,
//   `inv_txfm_add_vert_dct_8x64_neon`) advance with wrapping arithmetic; the
//   64-point loads keep `x7`, `x9`, `x10`, `x11` as element offsets from the
//   coefficient pointer for the same reason. Accesses stay where the module
//   header states.
// - Macros defined inside a function (`store1`, `store2`, `combine`,
//   `store_addsub`, `add_dest_addsub`) are Rust functions prefixed with the
//   function that defines them where two share a name.
// =============================================================================

/// `eob_8x16`
const EOB_8X16: [u16; 4] = [10, 43, 75, 128];
/// `eob_8x16_identity1`
const EOB_8X16_IDENTITY1: [u16; 4] = [4, 64, 96, 128];
/// `eob_8x16_identity2`
const EOB_8X16_IDENTITY2: [u16; 4] = [4, 8, 12, 128];
/// `eob_32x32`
const EOB_32X32: [u16; 8] = [10, 36, 78, 136, 210, 300, 406, 1024];
/// `eob_16x32`
const EOB_16X32: [u16; 8] = [10, 36, 78, 151, 215, 279, 343, 512];
/// `eob_16x32_shortside`
const EOB_16X32_SHORTSIDE: [u16; 4] = [10, 36, 78, 512];
/// `eob_8x32`
const EOB_8X32: [u16; 8] = [10, 43, 75, 107, 139, 171, 203, 256];

/// `movz w17, #2*(5793-4096)*8, lsl #16`
const W17_IDENTITY16: i32 = 2 * (5793 - 4096) * 8 * (1 << 16);

// --- helpers for instruction blocks -----------------------------------------------

/// `rev64 vd.8h, vn.8h`
#[inline(always)]
fn rev64_8h(a: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsic; NEON is baseline on AArch64.
    unsafe { vrev64q_s16(a) }
}

/// `srshl vd.4s, vn.4s, vm.4s`
#[inline(always)]
fn srshl_4s(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsic; NEON is baseline on AArch64.
    unsafe { h(vrshlq_s32(s(a), s(b))) }
}

/// The `.irp r, … smin \r.4s, \r.4s, v<max>.4s` / `.irp r, … smax \r.4s,
/// \r.4s, v<min>.4s` pair (also `smin_4s`/`smax_4s`).
#[inline(always)]
fn clip_4s(v: &mut V, regs: &[usize], max: usize, min: usize) {
    for &r in regs {
        v[r] = smin_4s(v[r], v[max]);
    }
    for &r in regs {
        v[r] = smax_4s(v[r], v[min]);
    }
}

/// Four `sqrshrn vD.4h, vL.4s, #SHIFT` followed by four
/// `sqrshrn2 vD.8h, vH.4s, #SHIFT` (no `hi` register is a destination).
#[inline(always)]
fn sqrshrn_4x8h<const SHIFT: i32>(v: &mut V, dst: [usize; 4], lo: [usize; 4], hi: [usize; 4]) {
    for k in 0..4 {
        v[dst[k]] = sqrshrn::<SHIFT>(v[lo[k]]);
    }
    for k in 0..4 {
        v[dst[k]] = sqrshrn2::<SHIFT>(v[dst[k]], v[hi[k]]);
    }
}

/// The `sqxtn`/`sqxtn2` block (`SQXTN`) or the `sqrshrn`/`sqrshrn2 #1` block
/// of the identity 32 entry points: v(16+2k), v(17+2k) (`.4s`) into v(16+k)
/// (`.8h`), in register order.
#[inline(always)]
fn narrow_pairs<const SQXTN: bool>(v: &mut V) {
    for k in 0..8 {
        if SQXTN {
            v[16 + k] = sqxtn(v[16 + 2 * k]);
            v[16 + k] = sqxtn2(v[16 + k], v[17 + 2 * k]);
        } else {
            v[16 + k] = sqrshrn::<1>(v[16 + 2 * k]);
            v[16 + k] = sqrshrn2::<1>(v[16 + k], v[17 + 2 * k]);
        }
    }
}

/// `p` moved by `r` rows of `stride` bytes, as a register update that may
/// leave the block (wrapping, no access).
#[inline(always)]
fn wrow(p: Dst, stride: isize, r: isize) -> Dst {
    (p as *mut u8).wrapping_offset(r * stride) as Dst
}

/// Label `1:`/`2:` of the 32- and 64-point entry points: `movi v4.8h–v7.8h,
/// #0`, then per step `subs w8, w8, #SUB` and `REPT` times
/// `st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], #64`, while `w8 > 0`.
#[inline(always)]
unsafe fn zero_rest<const SUB: i32, const REPT: usize>(v: &mut V, x6: *mut i16, w8: i32) {
    for r in 4..8 {
        v[r] = vdupq_n_s16(0);
    }
    let mut x6 = x6;
    let mut w8 = w8;
    loop {
        w8 -= SUB;
        for _ in 0..REPT {
            for r in 4..8 {
                st_8h(x6 as Dst, v[r]);
                x6 = x6.wrapping_add(8);
            }
        }
        if w8 <= 0 {
            break;
        }
    }
}

// --- 16x8, 8x16 ----------------------------------------------------------------

/// `inv_txfm_add_16x8_neon`: `x4` is `inv_*_4s_x16`, `x5` `inv_*_8h_x8`,
/// `w13` the eob threshold. The spilled `d8`–`d15` are registers of the local
/// file.
#[inline(always)]
unsafe fn inv_txfm_add_16x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, x4: K, x5: K, w13: c_int) {
    let mut v = regs();
    // cmp w3, w13; mov x11, #32; b.lt 1f
    let x11 = 8;
    if eob >= w13 {
        // movi v4.4s, #0; movz w16, #2896*8, lsl #16; dup v0.2s, w16
        v[4] = dup_4s(0);
        v[0] = dup_2s(W2896);
        // add x6, x2, #16
        let mut x6 = coeff.add(4);
        for i in 16..32 {
            v[i] = ld_4s(x6);
            st_4s(x6, v[4]);
            x6 = x6.add(x11);
        }
        let c = v[0];
        scale_input::<0>(&mut v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
        scale_input::<0>(&mut v, c, &[24, 25, 26, 27, 28, 29, 30, 31]);
        call_4s(x4, 16, &mut v);

        sqrshrn_4x8h::<1>(&mut v, [8, 9, 10, 11], [16, 17, 18, 19], [20, 21, 22, 23]);
        sqrshrn_4x8h::<1>(&mut v, [12, 13, 14, 15], [24, 25, 26, 27], [28, 29, 30, 31]);

        transpose_4x8h(&mut v, [8, 9, 10, 11], [2, 3, 4, 5]);
        transpose_4x8h(&mut v, [12, 13, 14, 15], [2, 3, 4, 5]);
    } else {
        // 1: movi v8.8h–v15.8h, #0
        for i in 8..16 {
            v[i] = vdupq_n_s16(0);
        }
    }
    // 2: movz w16, #2896*8, lsl #16; dup v0.2s, w16; movi v4.4s, #0
    v[0] = dup_2s(W2896);
    v[4] = dup_4s(0);
    let mut x2 = coeff;
    for i in 16..32 {
        v[i] = ld_4s(x2);
        st_4s(x2, v[4]);
        x2 = x2.add(x11);
    }
    let c = v[0];
    scale_input::<0>(&mut v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
    scale_input::<0>(&mut v, c, &[24, 25, 26, 27, 28, 29, 30, 31]);
    call_4s(x4, 16, &mut v);

    sqrshrn_4x8h::<1>(&mut v, [16, 17, 18, 19], [16, 17, 18, 19], [20, 21, 22, 23]);

    v[20] = v[8];
    v[21] = v[9];
    v[22] = v[10];
    v[23] = v[11];

    transpose_4x8h(&mut v, [16, 17, 18, 19], [2, 3, 4, 5]);

    sqrshrn_4x8h::<1>(&mut v, [8, 9, 10, 11], [24, 25, 26, 27], [28, 29, 30, 31]);

    transpose_4x8h(&mut v, [8, 9, 10, 11], [2, 3, 4, 5]);

    call::<true>(x5, 8, &mut v);

    // mov x6, x0; load_add_store_8x8 x6, x7
    load_add_store_8x8::<4>(dst, stride, &v);

    for i in 0..8 {
        v[16 + i] = v[8 + i];
    }

    call::<true>(x5, 8, &mut v);

    // add x0, x0, #16; load_add_store_8x8 x0, x7
    load_add_store_8x8::<4>(dst.add(8), stride, &v);
}

/// One column group of `inv_txfm_add_8x16_neon`: eight rows of four
/// coefficients from `x6` (16 values apart, cleared), scaled, `x4`,
/// `sqrshrn`/`sqrshrn2 #1` into `o`, `transpose_4x8h o, v2–v5`.
#[inline(always)]
unsafe fn inv_txfm_add_8x16_part(v: &mut V, x6: Coef, x4: K, o: [usize; 4]) {
    // movi v4.4s, #0; movz w16, #2896*8, lsl #16; dup v0.2s, w16
    v[4] = dup_4s(0);
    v[0] = dup_2s(W2896);
    let x11 = 16;
    let mut x6 = x6;
    for i in 16..24 {
        v[i] = ld_4s(x6);
        st_4s(x6, v[4]);
        x6 = x6.add(x11);
    }
    let c = v[0];
    scale_input::<0>(v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
    call_4s(x4, 8, v);
    sqrshrn_4x8h::<1>(v, o, [16, 17, 18, 19], [20, 21, 22, 23]);
    transpose_4x8h(v, o, [2, 3, 4, 5]);
}

/// `inv_txfm_add_8x16_neon`: `x4` is `inv_*_4s_x8`, `x5` `inv_*_8h_x16`,
/// `x13` the eob table.
#[inline(always)]
unsafe fn inv_txfm_add_8x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, x4: K, x5: K, x13: &[u16; 4]) {
    let mut v = regs();
    // ldrh w12, [x13, #4]; mov x11, #64; cmp w3, w12; ldrh w12, [x13, #2]
    let mut w12 = x13[2] as c_int;
    let lt = eob < w12;
    w12 = x13[1] as c_int;
    if !lt {
        // add x6, x2, #48
        inv_txfm_add_8x16_part(&mut v, coeff.add(12), x4, [28, 29, 30, 31]);
    } else {
        // 1: movi v28.8h–v31.8h, #0
        for i in 28..32 {
            v[i] = vdupq_n_s16(0);
        }
    }
    // 2: cmp w3, w12; ldrh w12, [x13, #0]
    let lt = eob < w12;
    w12 = x13[0] as c_int;
    if !lt {
        // add x6, x2, #32
        inv_txfm_add_8x16_part(&mut v, coeff.add(8), x4, [24, 25, 26, 27]);
    } else {
        for i in 24..28 {
            v[i] = vdupq_n_s16(0);
        }
    }
    // 2: cmp w3, w12
    if eob >= w12 {
        // add x6, x2, #16
        inv_txfm_add_8x16_part(&mut v, coeff.add(4), x4, [8, 9, 10, 11]);
    } else {
        for i in 8..12 {
            v[i] = vdupq_n_s16(0);
        }
    }
    // 2: (the same block from x2)
    inv_txfm_add_8x16_part(&mut v, coeff, x4, [16, 17, 18, 19]);

    v[20] = v[8];
    v[21] = v[9];
    v[22] = v[10];
    v[23] = v[11];

    call::<true>(x5, 16, &mut v);

    load_add_store_8x16(dst, stride, &v);
}

/// `def_fn_816 w, h, txfm1, txfm2`: the eob table (`x13`).
#[inline(always)]
fn eob_816(t1: K, t2: K) -> &'static [u16; 4] {
    match (t1, t2) {
        (K::Id, K::Id) => &EOB_8X16,
        (K::Id, _) => &EOB_8X16_IDENTITY1,
        (_, K::Id) => &EOB_8X16_IDENTITY2,
        _ => &EOB_8X16,
    }
}

/// `def_fn_816 16, 8, txfm1, txfm2` (`ldrh w13, [x13]`)
#[inline(always)]
unsafe fn itx_16x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<16, 8, 1>(dst, stride, coeff);
    }
    inv_txfm_add_16x8(dst, stride, coeff, eob, t1, t2, eob_816(t1, t2)[0] as c_int);
}

/// `def_fn_816 8, 16, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_8x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, _bdmax: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<8, 16, 1>(dst, stride, coeff);
    }
    inv_txfm_add_8x16(dst, stride, coeff, eob, t1, t2, eob_816(t1, t2));
}

def_fns! {
    dav1d_inv_txfm_add_dct_dct_8x16_16bpc_neon = itx_8x16(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_8x16_16bpc_neon = itx_8x16(Id, Id);
    dav1d_inv_txfm_add_dct_adst_8x16_16bpc_neon = itx_8x16(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_8x16_16bpc_neon = itx_8x16(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_8x16_16bpc_neon = itx_8x16(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_8x16_16bpc_neon = itx_8x16(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_8x16_16bpc_neon = itx_8x16(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_8x16_16bpc_neon = itx_8x16(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_8x16_16bpc_neon = itx_8x16(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_8x16_16bpc_neon = itx_8x16(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_8x16_16bpc_neon = itx_8x16(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_8x16_16bpc_neon = itx_8x16(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_8x16_16bpc_neon = itx_8x16(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_8x16_16bpc_neon = itx_8x16(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_8x16_16bpc_neon = itx_8x16(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_8x16_16bpc_neon = itx_8x16(Id, Flip);

    dav1d_inv_txfm_add_dct_dct_16x8_16bpc_neon = itx_16x8(Dct, Dct);
    dav1d_inv_txfm_add_identity_identity_16x8_16bpc_neon = itx_16x8(Id, Id);
    dav1d_inv_txfm_add_dct_adst_16x8_16bpc_neon = itx_16x8(Dct, Adst);
    dav1d_inv_txfm_add_dct_flipadst_16x8_16bpc_neon = itx_16x8(Dct, Flip);
    dav1d_inv_txfm_add_dct_identity_16x8_16bpc_neon = itx_16x8(Dct, Id);
    dav1d_inv_txfm_add_adst_dct_16x8_16bpc_neon = itx_16x8(Adst, Dct);
    dav1d_inv_txfm_add_adst_adst_16x8_16bpc_neon = itx_16x8(Adst, Adst);
    dav1d_inv_txfm_add_adst_flipadst_16x8_16bpc_neon = itx_16x8(Adst, Flip);
    dav1d_inv_txfm_add_flipadst_dct_16x8_16bpc_neon = itx_16x8(Flip, Dct);
    dav1d_inv_txfm_add_flipadst_adst_16x8_16bpc_neon = itx_16x8(Flip, Adst);
    dav1d_inv_txfm_add_flipadst_flipadst_16x8_16bpc_neon = itx_16x8(Flip, Flip);
    dav1d_inv_txfm_add_identity_dct_16x8_16bpc_neon = itx_16x8(Id, Dct);
    dav1d_inv_txfm_add_adst_identity_16x8_16bpc_neon = itx_16x8(Adst, Id);
    dav1d_inv_txfm_add_flipadst_identity_16x8_16bpc_neon = itx_16x8(Flip, Id);
    dav1d_inv_txfm_add_identity_adst_16x8_16bpc_neon = itx_16x8(Id, Adst);
    dav1d_inv_txfm_add_identity_flipadst_16x8_16bpc_neon = itx_16x8(Id, Flip);
}

// --- 32-point, first pass (32 bits) ------------------------------------------------

/// `inv_dct32_odd_4s_x16_neon`: the odd half of a 32-point DCT on v16–v31;
/// leaves row_clip_max in `v5`, row_clip_min in `v4` and idct_coeffs[0..8]
/// in `v0`, `v1`.
#[inline(always)]
fn inv_dct32_odd_4s_x16(v: &mut V) {
    // movrel x16, idct_coeffs, 4*16; ld1 {v0.4s, v1.4s}, [x16], #32
    v[0] = coeffs(&IDCT_COEFFS, 16);
    v[1] = coeffs(&IDCT_COEFFS, 20);

    v[2] = mul_mls::<0, 1>(v[16], v[31], v[0], v[0]); // -> t16a
    v[4] = mul_mla::<1, 0>(v[16], v[31], v[0], v[0]); // -> t31a
    v[6] = mul_mls::<2, 3>(v[24], v[23], v[0], v[0]); // -> t17a
    v[16] = srshr_4s::<12>(v[2]); // t16a
    v[31] = srshr_4s::<12>(v[4]); // t31a
    v[2] = mul_mla::<3, 2>(v[24], v[23], v[0], v[0]); // -> t30a
    v[4] = mul_mls::<0, 1>(v[20], v[27], v[1], v[1]); // -> t18a
    v[24] = srshr_4s::<12>(v[6]); // t17a
    v[23] = srshr_4s::<12>(v[2]); // t30a
    v[6] = mul_mla::<1, 0>(v[20], v[27], v[1], v[1]); // -> t29a
    v[2] = mul_mls::<2, 3>(v[28], v[19], v[1], v[1]); // -> t19a
    v[20] = srshr_4s::<12>(v[4]); // t18a
    v[27] = srshr_4s::<12>(v[6]); // t29a
    v[4] = mul_mla::<3, 2>(v[28], v[19], v[1], v[1]); // -> t28a
    // ldp q0, q1, [x16], #-4*24
    v[0] = coeffs(&IDCT_COEFFS, 24);
    v[1] = coeffs(&IDCT_COEFFS, 28);
    v[6] = mul_mls::<0, 1>(v[18], v[29], v[0], v[0]); // -> t20a
    v[28] = srshr_4s::<12>(v[2]); // t19a
    v[19] = srshr_4s::<12>(v[4]); // t28a
    v[2] = mul_mla::<1, 0>(v[18], v[29], v[0], v[0]); // -> t27a
    v[4] = mul_mls::<2, 3>(v[26], v[21], v[0], v[0]); // -> t21a
    v[18] = srshr_4s::<12>(v[6]); // t20a
    v[29] = srshr_4s::<12>(v[2]); // t27a
    v[6] = mul_mla::<3, 2>(v[26], v[21], v[0], v[0]); // -> t26a
    v[2] = mul_mls::<0, 1>(v[22], v[25], v[1], v[1]); // -> t22a
    v[26] = srshr_4s::<12>(v[4]); // t21a
    v[21] = srshr_4s::<12>(v[6]); // t26a
    v[4] = mul_mla::<1, 0>(v[22], v[25], v[1], v[1]); // -> t25a
    v[6] = mul_mls::<2, 3>(v[30], v[17], v[1], v[1]); // -> t23a
    v[22] = srshr_4s::<12>(v[2]); // t22a
    v[25] = srshr_4s::<12>(v[4]); // t25a
    v[2] = mul_mla::<3, 2>(v[30], v[17], v[1], v[1]); // -> t24a
    v[30] = srshr_4s::<12>(v[6]); // t23a
    v[17] = srshr_4s::<12>(v[2]); // t24a

    // ld1 {v0.4s, v1.4s}, [x16]
    v[0] = coeffs(&IDCT_COEFFS, 0);
    v[1] = coeffs(&IDCT_COEFFS, 4);

    v[5] = dup_4s(ROW_CLIP_MAX);
    v[4] = dup_4s(ROW_CLIP_MIN);

    v[2] = sqsub_4s(v[16], v[24]); // t17
    v[16] = sqadd_4s(v[16], v[24]); // t16
    v[3] = sqsub_4s(v[31], v[23]); // t30
    v[31] = sqadd_4s(v[31], v[23]); // t31
    v[24] = sqsub_4s(v[28], v[20]); // t18
    v[28] = sqadd_4s(v[28], v[20]); // t19
    v[23] = sqadd_4s(v[18], v[26]); // t20
    v[18] = sqsub_4s(v[18], v[26]); // t21
    v[20] = sqsub_4s(v[30], v[22]); // t22
    v[30] = sqadd_4s(v[30], v[22]); // t23
    v[26] = sqadd_4s(v[17], v[25]); // t24
    v[17] = sqsub_4s(v[17], v[25]); // t25
    v[22] = sqsub_4s(v[29], v[21]); // t26
    v[29] = sqadd_4s(v[29], v[21]); // t27
    v[25] = sqadd_4s(v[19], v[27]); // t28
    v[19] = sqsub_4s(v[19], v[27]); // t29

    clip_4s(v, &[2, 16, 3, 31, 24, 28, 23, 18, 20, 30, 26, 17, 22, 29, 25, 19], 5, 4);

    v[7] = mul_mls::<0, 1>(v[3], v[2], v[1], v[1]); // -> t17a
    v[6] = mul_mla::<1, 0>(v[3], v[2], v[1], v[1]); // -> t30a
    v[2] = mul_mla::<1, 0>(v[19], v[24], v[1], v[1]); // -> t18a
    v[21] = srshr_4s::<12>(v[7]); // t17a
    v[27] = srshr_4s::<12>(v[6]); // t30a
    v[2] = neg_4s(v[2]); // -> t18a
    v[7] = mul_mls::<0, 1>(v[19], v[24], v[1], v[1]); // -> t29a
    v[6] = mul_mls::<2, 3>(v[22], v[18], v[1], v[1]); // -> t21a
    v[19] = srshr_4s::<12>(v[2]); // t18a
    v[24] = srshr_4s::<12>(v[7]); // t29a
    v[2] = mul_mla::<3, 2>(v[22], v[18], v[1], v[1]); // -> t26a
    v[7] = mul_mla::<3, 2>(v[17], v[20], v[1], v[1]); // -> t22a
    v[22] = srshr_4s::<12>(v[6]); // t21a
    v[18] = srshr_4s::<12>(v[2]); // t26a
    v[7] = neg_4s(v[7]); // -> t22a
    v[6] = mul_mls::<2, 3>(v[17], v[20], v[1], v[1]); // -> t25a
    v[17] = srshr_4s::<12>(v[7]); // t22a
    v[20] = srshr_4s::<12>(v[6]); // t25a

    v[2] = sqsub_4s(v[27], v[24]); // t29
    v[27] = sqadd_4s(v[27], v[24]); // t30
    v[3] = sqsub_4s(v[21], v[19]); // t18
    v[21] = sqadd_4s(v[21], v[19]); // t17
    v[24] = sqsub_4s(v[16], v[28]); // t19a
    v[16] = sqadd_4s(v[16], v[28]); // t16a
    v[19] = sqsub_4s(v[30], v[23]); // t20a
    v[30] = sqadd_4s(v[30], v[23]); // t23a
    v[28] = sqsub_4s(v[17], v[22]); // t21
    v[17] = sqadd_4s(v[17], v[22]); // t22
    v[23] = sqadd_4s(v[26], v[29]); // t24a
    v[26] = sqsub_4s(v[26], v[29]); // t27a
    v[22] = sqadd_4s(v[20], v[18]); // t25
    v[20] = sqsub_4s(v[20], v[18]); // t26
    v[29] = sqsub_4s(v[31], v[25]); // t28a
    v[31] = sqadd_4s(v[31], v[25]); // t31a

    clip_4s(v, &[2, 27, 3, 21, 24, 16, 19, 30, 28, 17, 23, 26, 22, 20, 29, 31], 5, 4);

    v[7] = mul_mls::<2, 3>(v[2], v[3], v[0], v[0]); // -> t18a
    v[6] = mul_mla::<3, 2>(v[2], v[3], v[0], v[0]); // -> t29a
    v[2] = mul_mls::<2, 3>(v[29], v[24], v[0], v[0]); // -> t19
    v[18] = srshr_4s::<12>(v[7]); // t18a
    v[25] = srshr_4s::<12>(v[6]); // t29a
    v[7] = mul_mla::<3, 2>(v[29], v[24], v[0], v[0]); // -> t28
    v[6] = mul_mla::<3, 2>(v[26], v[19], v[0], v[0]); // -> t20
    v[29] = srshr_4s::<12>(v[2]); // t19
    v[24] = srshr_4s::<12>(v[7]); // t28
    v[6] = neg_4s(v[6]); // -> t20
    v[2] = mul_mls::<2, 3>(v[26], v[19], v[0], v[0]); // -> t27
    v[7] = mul_mla::<3, 2>(v[20], v[28], v[0], v[0]); // -> t21a
    v[26] = srshr_4s::<12>(v[6]); // t20
    v[19] = srshr_4s::<12>(v[2]); // t27
    v[7] = neg_4s(v[7]); // -> t21a
    v[6] = mul_mls::<2, 3>(v[20], v[28], v[0], v[0]); // -> t26a
    v[20] = srshr_4s::<12>(v[7]); // t21a
    v[28] = srshr_4s::<12>(v[6]); // t26a

    v[2] = sqsub_4s(v[16], v[30]); // t23
    v[16] = sqadd_4s(v[16], v[30]); // t16 = out16
    v[3] = sqsub_4s(v[31], v[23]); // t24
    v[31] = sqadd_4s(v[31], v[23]); // t31 = out31
    v[23] = sqsub_4s(v[21], v[17]); // t22a
    v[17] = sqadd_4s(v[21], v[17]); // t17a = out17
    v[30] = sqadd_4s(v[27], v[22]); // t30a = out30
    v[21] = sqsub_4s(v[27], v[22]); // t25a
    v[27] = sqsub_4s(v[18], v[20]); // t21
    v[18] = sqadd_4s(v[18], v[20]); // t18 = out18
    v[7] = sqadd_4s(v[29], v[26]); // t19a = out19
    v[26] = sqsub_4s(v[29], v[26]); // t20a
    v[29] = sqadd_4s(v[25], v[28]); // t29 = out29
    v[25] = sqsub_4s(v[25], v[28]); // t26
    v[28] = sqadd_4s(v[24], v[19]); // t28a = out28
    v[24] = sqsub_4s(v[24], v[19]); // t27a
    v[19] = v[7]; // out19

    clip_4s(v, &[2, 16, 3, 31, 23, 17, 30, 21, 27, 18, 19, 26, 29, 25, 28, 24], 5, 4);

    v[7] = mul_mls::<0, 0>(v[24], v[26], v[0], v[0]); // -> t20
    v[6] = mul_mla::<0, 0>(v[24], v[26], v[0], v[0]); // -> t27
    v[20] = srshr_4s::<12>(v[7]); // t20
    v[22] = srshr_4s::<12>(v[6]); // t27

    v[7] = mul_mla::<0, 0>(v[25], v[27], v[0], v[0]); // -> t26a
    v[6] = mul_mls::<0, 0>(v[25], v[27], v[0], v[0]); // -> t21a
    v[27] = v[22]; // t27
    v[26] = srshr_4s::<12>(v[7]); // t26a

    v[24] = mul_mls::<0, 0>(v[21], v[23], v[0], v[0]); // -> t22
    v[7] = mul_mla::<0, 0>(v[21], v[23], v[0], v[0]); // -> t25
    v[21] = srshr_4s::<12>(v[6]); // t21a
    v[22] = srshr_4s::<12>(v[24]); // t22
    v[25] = srshr_4s::<12>(v[7]); // t25

    v[7] = mul_mls::<0, 0>(v[3], v[2], v[0], v[0]); // -> t23a
    v[6] = mul_mla::<0, 0>(v[3], v[2], v[0], v[0]); // -> t24a
    v[23] = srshr_4s::<12>(v[7]); // t23a
    v[24] = srshr_4s::<12>(v[6]); // t24a
}

/// The `store2 r0, r1, r2, r3, shift` macro of `def_horz_32`: the 16 `.4s`
/// values at `x6` plus and minus `r`, narrowed by `#SHIFT`; the sums as the
/// first 16, the reversed differences as the last 16 `.8h` values of the row
/// (64 bytes, the same bytes `store1` filled).
#[inline(always)]
unsafe fn horz_32_store2<const SHIFT: i32>(v: &mut V, x6: *mut i16, r: [usize; 4]) {
    let [r0, r1, r2, r3] = r;
    let p = x6 as *mut i32;
    // ld1 {v0.4s, v1.4s, v2.4s, v3.4s}, [x6]
    v[0] = ld_4s(p);
    v[1] = ld_4s(p.add(4));
    v[2] = ld_4s(p.add(8));
    v[3] = ld_4s(p.add(12));
    v[4] = sqsub_4s(v[0], v[r0]);
    v[0] = sqadd_4s(v[0], v[r0]);
    v[5] = sqsub_4s(v[1], v[r1]);
    v[1] = sqadd_4s(v[1], v[r1]);
    v[6] = sqsub_4s(v[2], v[r2]);
    v[2] = sqadd_4s(v[2], v[r2]);
    v[7] = sqsub_4s(v[3], v[r3]);
    v[3] = sqadd_4s(v[3], v[r3]);
    v[0] = sqrshrn::<SHIFT>(v[0]);
    v[0] = sqrshrn2::<SHIFT>(v[0], v[1]);
    v[1] = sqrshrn::<SHIFT>(v[2]);
    v[1] = sqrshrn2::<SHIFT>(v[1], v[3]);
    v[2] = sqrshrn::<SHIFT>(v[7]);
    v[2] = sqrshrn2::<SHIFT>(v[2], v[6]);
    v[3] = sqrshrn::<SHIFT>(v[5]);
    v[3] = sqrshrn2::<SHIFT>(v[3], v[4]);
    // st1 {v0.8h, v1.8h}, [x6], #32
    st_8h(x6 as Dst, v[0]);
    st_8h(x6.add(8) as Dst, v[1]);
    v[2] = rev64_8h(v[2]);
    v[3] = rev64_8h(v[3]);
    // st1 {v2.8h, v3.8h}, [x6], #32
    st_8h(x6.add(16) as Dst, v[2]);
    st_8h(x6.add(24) as Dst, v[3]);
}

/// `inv_txfm_horz_dct_32x4_neon` (`def_horz_32 scale=0, shift=2`) and
/// `inv_txfm_horz_scale_dct_32x4_neon` (`SCALE`, shift 1): four columns of
/// coefficients from `x7` (32 rows, `x8` values apart, cleared after the
/// load) into four rows of 32 values at `x6` (64 bytes apart).
#[inline(always)]
unsafe fn inv_txfm_horz_dct_32x4<const SCALE: bool>(v: &mut V, x6: *mut i16, x7: Coef, x8: usize) {
    v[7] = dup_4s(0);
    // lsl x8, x8, #1
    let x8 = 2 * x8;
    if SCALE {
        // movz w16, #2896*8, lsl #16; dup v0.2s, w16
        v[0] = dup_2s(W2896);
    }
    for i in 0..16 {
        v[16 + i] = ld_4s(x7.add(x8 * i));
        st_4s(x7.add(x8 * i), v[7]);
    }
    // sub x7, x7, x8, lsl #4; add x7, x7, x8, lsr #1
    let x7 = x7.add(x8 / 2);
    if SCALE {
        let c = v[0];
        scale_input::<0>(v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
        scale_input::<0>(v, c, &[24, 25, 26, 27, 28, 29, 30, 31]);
    }
    inv_dct_4s_x16(v);

    // idct_16 leaves the row_clip_max/min constants in v5 and v4
    clip_4s(
        v,
        &[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        5,
        4,
    );

    transpose_4x4s(v, [16, 17, 18, 19], [2, 3, 4, 5]);
    transpose_4x4s(v, [20, 21, 22, 23], [2, 3, 4, 5]);
    transpose_4x4s(v, [24, 25, 26, 27], [2, 3, 4, 5]);
    transpose_4x4s(v, [28, 29, 30, 31], [2, 3, 4, 5]);

    // store1 v16.4s, v20.4s, v24.4s, v28.4s … v19.4s, v23.4s, v27.4s, v31.4s
    let p = x6 as *mut i32;
    for j in 0..4 {
        for (k, r) in [16 + j, 20 + j, 24 + j, 28 + j].into_iter().enumerate() {
            st_4s(p.add(16 * j + 4 * k), v[r]);
        }
    }
    // sub x6, x6, #64*4

    v[7] = dup_4s(0);
    for i in 0..16 {
        v[16 + i] = ld_4s(x7.add(x8 * i));
        st_4s(x7.add(x8 * i), v[7]);
    }
    if SCALE {
        // This relies on the fact that the idct also leaves the right coeff
        // in v0.s[1]
        let c = v[0];
        scale_input::<1>(v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
        scale_input::<1>(v, c, &[24, 25, 26, 27, 28, 29, 30, 31]);
    }
    inv_dct32_odd_4s_x16(v);
    transpose_4x4s(v, [31, 30, 29, 28], [2, 3, 4, 5]);
    transpose_4x4s(v, [27, 26, 25, 24], [2, 3, 4, 5]);
    transpose_4x4s(v, [23, 22, 21, 20], [2, 3, 4, 5]);
    transpose_4x4s(v, [19, 18, 17, 16], [2, 3, 4, 5]);

    for j in 0..4 {
        let r = [31 - j, 27 - j, 23 - j, 19 - j];
        if SCALE {
            horz_32_store2::<1>(v, x6.add(32 * j), r);
        } else {
            horz_32_store2::<2>(v, x6.add(32 * j), r);
        }
    }
}

// --- 32-point, second pass (16 bits) -----------------------------------------------

/// `inv_txfm_add_vert_dct_8x32_neon`: a 32-point DCT down eight columns of
/// `x7` (32 rows, `x8` values apart; the even half is written back to the
/// even rows), added to 32 rows of 8 pixels at `x6`.
#[inline(always)]
unsafe fn inv_txfm_add_vert_dct_8x32(v: &mut V, x6: Dst, stride: isize, x7: *mut i16, x8: usize) {
    // lsl x8, x8, #1
    let x8 = 2 * x8;

    for i in 0..16 {
        v[16 + i] = vld1q_s16(x7.add(x8 * i));
    }
    // sub x7, x7, x8, lsl #4

    // bl X(inv_dct_8h_x16_neon)
    call::<true>(K::Dct, 16, v);

    for i in 0..16 {
        vst1q_s16(x7.add(x8 * i), v[16 + i]);
    }
    // sub x7, x7, x8, lsl #4; add x7, x7, x8, lsr #1
    let odd = x7.add(x8 / 2);

    for i in 0..16 {
        v[16 + i] = vld1q_s16(odd.add(x8 * i));
    }
    // sub x7, x7, x8, lsl #4; sub x7, x7, x8, lsr #1 (x7 is back at row 0)

    // bl X(inv_dct32_odd_8h_x16_neon)
    super::itx::idct32_odd(v);

    // neg x9, x8; mov x10, x6; mvni v1.8h, #0xfc, lsl #8
    v[1] = mvni_3ff();
    // combine v31.8h … v16.8h, sqadd, x8: row m from even row m (x7 ascending)
    let mut y = 0isize;
    for m in 0..16 {
        let e = vld1q_s16(x7.add(x8 * m));
        load_add_store::<4>(row(x6, stride, y), sqadd::<true>(e, v[31 - m]));
        y += 1;
    }
    // sub x7, x7, x8; combine v16.8h … v31.8h, sqsub, x9: x7 descending
    for m in 0..16 {
        let e = vld1q_s16(x7.add(x8 * (15 - m)));
        load_add_store::<4>(row(x6, stride, y), sqsub::<true>(e, v[16 + m]));
        y += 1;
    }
}

// --- identity 32 -------------------------------------------------------------------

/// The two loads of one row of 8 coefficients (`ld1 {va.4s, vb.4s}, [x2]`)
/// and its clearing (`st1 {v0.4s, v1.4s}, [x2], x8`) of the identity 32
/// entry points: rows to v16–v31 in pairs, `x2` advanced by eight rows.
#[inline(always)]
unsafe fn identity_load8(v: &mut V, x2: &mut Coef, x8: usize) {
    for k in 0..8 {
        v[16 + 2 * k] = ld_4s(*x2);
        v[17 + 2 * k] = ld_4s(x2.add(4));
        st_4s(*x2, v[0]);
        st_4s(x2.add(4), v[1]);
        *x2 = x2.wrapping_add(x8);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_32x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    let mut v = regs();
    v[0] = vdupq_n_s16(0);
    v[1] = vdupq_n_s16(0);
    // movrel x13, eob_32x32, 2
    let mut x13 = 1;

    // mov x8, #4*32
    let x8 = 32;
    let mut x0 = dst;
    let mut x2 = coeff;
    loop {
        // 1: mov w9, #0; movrel x12, eob_32x32, 2
        let mut w9 = 0usize;
        let mut x12 = 1;
        loop {
            // 2:
            w9 += 8;
            identity_load8(&mut v, &mut x2, x8);
            narrow_pairs::<true>(&mut v);
            transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);

            load_add_store_8x8::<2>(x0, stride, &v);
            // ldrh w11, [x12], #4
            let w11 = EOB_32X32[x12] as c_int;
            x12 += 2;
            // sub x0, x0, x1, lsl #3 (undoes the macro's eight rows);
            // add x0, x0, #2*8
            x0 = x0.wrapping_add(8);
            // cmp w3, w11; b.ge 2b
            if eob < w11 {
                break;
            }
        }

        // ldrh w11, [x13], #4; cmp w3, w11; b.lt 9f
        let w11 = EOB_32X32[x13] as c_int;
        x13 += 2;
        if eob < w11 {
            return;
        }

        // sub x0, x0, w9, uxtw #1; add x0, x0, x1, lsl #3
        x0 = wrow(x0.wrapping_sub(w9), stride, 8);
        // msub x2, x8, x9, x2; add x2, x2, #4*8
        x2 = x2.wrapping_sub(x8 * w9).wrapping_add(8);
    }
}

/// `def_identity_1632 w, h, wshort, hshort`: `inv_txfm_add_identity_identity_
/// 16x32_16bpc_neon` (`W == 16`) and `…_32x16_16bpc_neon`.
#[inline(always)]
unsafe fn identity_1632<const W: usize, const H: usize>(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    // eob_16x32\hshort (x13) and eob_16x32\wshort (x12)
    let (outer, inner): (&[u16], &[u16]) = if W == 16 {
        (&EOB_16X32, &EOB_16X32_SHORTSIDE)
    } else {
        (&EOB_16X32_SHORTSIDE, &EOB_16X32)
    };
    let mut v = regs();
    // movz w16, #2896*8, lsl #16; movz w17, #2*(5793-4096)*8, lsl #16
    v[0] = dup_4s(0);
    v[1] = dup_4s(0);
    // movrel x13, eob_16x32\hshort, 2
    let mut x13 = 1;

    // mov x8, #4*\h
    let x8 = H;
    let mut x0 = dst;
    let mut x2 = coeff;
    loop {
        // 1: mov w9, #0; movrel x12, eob_16x32\wshort, 2
        let mut w9 = 0usize;
        let mut x12 = 1;
        loop {
            // 2:
            w9 += 8;
            identity_load8(&mut v, &mut x2, x8);
            // dup v2.2s, w16; mov v2.s[1], w17 (between the loads)
            v[2] = dup_2s(W2896);
            v[2] = h(vsetq_lane_s32::<1>(W17_IDENTITY16, s(v[2])));
            let c = v[2];
            scale_input::<0>(&mut v, c, &[16, 17, 18, 19, 20, 21, 22, 23]);
            scale_input::<0>(&mut v, c, &[24, 25, 26, 27, 28, 29, 30, 31]);

            if W == 16 {
                // 16x32
                identity_4x16_shift1::<1>(&mut v, c);
            } else {
                // 32x16: shift_16_regs sqshl, 1; identity_4x16 v2.s[1]
                for i in 16..32 {
                    v[i] = sqshl1_4s(v[i]);
                }
                identity_4x16::<1>(&mut v, c);
            }
            narrow_pairs::<true>(&mut v);

            transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);

            if W == 16 {
                load_add_store_8x8::<2>(x0, stride, &v);
            } else {
                load_add_store_8x8::<4>(x0, stride, &v);
            }
            // ldrh w11, [x12], #4
            let w11 = inner[x12] as c_int;
            x12 += 2;
            // sub x0, x0, x1, lsl #3; add x0, x0, #16
            x0 = x0.wrapping_add(8);
            // cmp w3, w11; b.ge 2b
            if eob < w11 {
                break;
            }
        }

        // ldrh w11, [x13], #4; cmp w3, w11; b.lt 9f
        let w11 = outer[x13] as c_int;
        x13 += 2;
        if eob < w11 {
            return;
        }

        // sub x0, x0, w9, uxtw #1; add x0, x0, x1, lsl #3
        x0 = wrow(x0.wrapping_sub(w9), stride, 8);
        // msub x2, x8, x9, x2; add x2, x2, #4*8
        x2 = x2.wrapping_sub(x8 * w9).wrapping_add(8);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    identity_1632::<16, 32>(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_32x16_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    identity_1632::<32, 16>(dst, stride, coeff, eob)
}

/// `def_identity_832 w, h`: `inv_txfm_add_identity_identity_8x32_16bpc_neon`
/// (`W == 8`) and `…_32x8_16bpc_neon`.
#[inline(always)]
unsafe fn identity_832<const W: usize, const H: usize>(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    v[0] = dup_4s(0);
    v[1] = dup_4s(0);
    // Working on 8x8 blocks, read every other entry from eob_8x32
    // movrel x13, eob_8x32, 2
    let mut x13 = 1;

    // mov w8, #4*\h
    let x8 = H;
    let mut x0 = dst;
    let mut x2 = coeff;
    loop {
        // 1: ldrh w12, [x13], #4
        let w12 = EOB_8X32[x13] as c_int;
        x13 += 2;
        identity_load8(&mut v, &mut x2, x8);

        if W == 8 {
            narrow_pairs::<false>(&mut v);
        } else {
            narrow_pairs::<true>(&mut v);
        }

        transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);

        // cmp w3, w12
        let lt = eob < w12;
        if W == 8 {
            load_add_store_8x8::<2>(x0, stride, &v);
        } else {
            load_add_store_8x8::<3>(x0, stride, &v);
        }
        // the macro leaves x0 eight rows down
        x0 = wrow(x0, stride, 8);

        // b.lt 9f
        if lt {
            return;
        }
        if W == 8 {
            // sub x2, x2, x8, lsl #3; add x2, x2, #4*8
            x2 = x2.wrapping_sub(8 * x8).wrapping_add(8);
        } else {
            // sub x0, x0, x1, lsl #3; add x0, x0, #2*8
            x0 = wrow(x0, stride, -8).wrapping_add(8);
        }
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    identity_832::<8, 32>(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_32x8_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    identity_832::<32, 8>(dst, stride, coeff, eob)
}

// --- dct_dct 32 ---------------------------------------------------------------------

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<32, 32, 2>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_32x32_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_32x32_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_32x32_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub sp, sp, #2048
    let mut buf = [0i32; 2048 / 4];
    let sp = buf.as_mut_ptr() as *mut i16;
    // movrel x13, eob_32x32; ldrh w12, [x13], #2
    let mut x13 = 0;
    let mut w12 = EOB_32X32[x13] as c_int;
    x13 += 1;

    for i in [0usize, 4, 8, 12, 16, 20, 24, 28] {
        // add x6, sp, #(\i*32*2)
        let x6 = sp.add(i * 32);
        if i > 0 {
            let w8 = (32 - i) as i32;
            if eob < w12 {
                // 1:, 2:
                zero_rest::<4, 4>(&mut v, x6, w8);
                break;
            }
            if i < 28 {
                w12 = EOB_32X32[x13] as c_int;
                x13 += 1;
            }
        }
        // add x7, x2, #(\i*4); mov x8, #32*4
        inv_txfm_horz_dct_32x4::<false>(&mut v, x6, coeff.add(i), 32);
    }

    // 3:
    for i in [0usize, 8, 16, 24] {
        // add x6, x0, #(\i*2); add x7, sp, #(\i*2); mov x8, #32*2
        inv_txfm_add_vert_dct_8x32(&mut v, dst.add(i), stride, sp.add(i), 32);
    }
    // add sp, sp, #2048
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<16, 32, 1>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_16x32_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_16x32_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_16x32_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub sp, sp, #1024
    let mut buf = [0i32; 1024 / 4];
    let sp = buf.as_mut_ptr() as *mut i16;
    // movrel x13, eob_16x32; ldrh w12, [x13], #2; adr x4, inv_dct_4s_x16_neon
    let mut x13 = 0;
    let mut w12 = EOB_16X32[x13] as c_int;
    x13 += 1;
    let x4 = K::Dct;

    for i in [0usize, 4, 8, 12, 16, 20, 24, 28] {
        // add x6, sp, #(\i*16*2); add x7, x2, #(\i*4)
        let x6 = sp.add(i * 16);
        if i > 0 {
            let w8 = (32 - i) as i32;
            if eob < w12 {
                zero_rest::<4, 2>(&mut v, x6, w8);
                break;
            }
            if i < 28 {
                w12 = EOB_16X32[x13] as c_int;
                x13 += 1;
            }
        }
        // mov x8, #4*32
        inv_txfm_horz_16x4::<true>(&mut v, x6, coeff.add(i), 32, x4);
    }

    // 3:
    for i in [0usize, 8] {
        // add x6, x0, #(\i*2); add x7, sp, #(\i*2); mov x8, #16*2
        inv_txfm_add_vert_dct_8x32(&mut v, dst.add(i), stride, sp.add(i), 16);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x16_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<32, 16, 1>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_32x16_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_32x16_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_32x16_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub sp, sp, #1024
    let mut buf = [0i32; 1024 / 4];
    let sp = buf.as_mut_ptr() as *mut i16;
    // movrel x13, eob_16x32; movrel x5, X(inv_dct_8h_x16_neon); ldrh w12, [x13], #2
    let mut x13 = 0;
    let x5 = K::Dct;
    let mut w12 = EOB_16X32[x13] as c_int;
    x13 += 1;

    for i in [0usize, 4, 8, 12] {
        // add x6, sp, #(\i*32*2); add x7, x2, #(\i*4)
        let x6 = sp.add(i * 32);
        if i > 0 {
            let w8 = (16 - i) as i32;
            if eob < w12 {
                zero_rest::<4, 4>(&mut v, x6, w8);
                break;
            }
            if i < 12 {
                w12 = EOB_16X32[x13] as c_int;
                x13 += 1;
            }
        }
        // mov x8, #4*16
        inv_txfm_horz_dct_32x4::<true>(&mut v, x6, coeff.add(i), 16);
    }

    // 3:
    for i in [0usize, 8, 16, 24] {
        // add x6, x0, #(\i*2); add x7, sp, #(\i*2); mov x8, #32*2
        inv_txfm_add_vert_8x16(&mut v, dst.add(i), stride, sp.add(i), 32, x5);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<8, 32, 2>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_8x32_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_8x32_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_8x32_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub sp, sp, #512
    let mut buf = [0i32; 512 / 4];
    let sp = buf.as_mut_ptr() as *mut i16;

    // movrel x13, eob_8x32
    let mut x13 = 0;

    // movi v28.4s, #0; mov x8, #4*32; mov w9, #32; mov x6, sp; mov x7, x2
    v[28] = dup_4s(0);
    let x8 = 32;
    let mut w9 = 32i32;
    let mut x6 = sp;
    let mut x7 = coeff;
    loop {
        // 1:
        for i in 0..8 {
            v[16 + i] = ld_4s(x7.add(x8 * i));
            st_4s(x7.add(x8 * i), v[28]);
        }
        // ldrh w12, [x13], #2
        let w12 = EOB_8X32[x13] as c_int;
        x13 += 1;
        w9 -= 4;
        // sub x7, x7, x8, lsl #3; add x7, x7, #4*4
        x7 = x7.wrapping_add(4);

        inv_dct_4s_x8(&mut v);

        sqrshrn_4x8h::<2>(&mut v, [16, 17, 18, 19], [16, 17, 18, 19], [20, 21, 22, 23]);

        transpose_4x8h(&mut v, [16, 17, 18, 19], [2, 3, 4, 5]);

        // cmp w3, w12; st1 {v16.8h, v17.8h, v18.8h, v19.8h}, [x6], #64
        for k in 0..4 {
            st_8h(x6.add(8 * k) as Dst, v[16 + k]);
        }
        x6 = x6.wrapping_add(32);

        // b.ge 1b
        if eob < w12 {
            break;
        }
    }
    // cbz w9, 3f
    if w9 != 0 {
        v[29] = vdupq_n_s16(0);
        v[30] = vdupq_n_s16(0);
        v[31] = vdupq_n_s16(0);
        loop {
            // 2: subs w9, w9, #4; st1 {v28.8h, v29.8h, v30.8h, v31.8h}, [x6], #64
            w9 -= 4;
            for k in 0..4 {
                st_8h(x6.add(8 * k) as Dst, v[28 + k]);
            }
            x6 = x6.wrapping_add(32);
            if w9 <= 0 {
                break;
            }
        }
    }

    // 3: mov x6, x0; mov x7, sp; mov x8, #8*2
    inv_txfm_add_vert_dct_8x32(&mut v, dst, stride, sp, 8);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x8_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<32, 8, 2>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_32x8_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_32x8_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_32x8_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub sp, sp, #512
    let mut buf = [0i32; 512 / 4];
    let sp = buf.as_mut_ptr() as *mut i16;

    for i in [0usize, 4] {
        // add x6, sp, #(\i*32*2); add x7, x2, #(\i*4)
        let x6 = sp.add(i * 32);
        // cmp w3, #10; b.lt 1f
        if i > 0 && eob < 10 {
            // 1: movi v4.8h–v7.8h, #0; .rept 4: st1 {v4–v7}, [x6], #64
            for r in 4..8 {
                v[r] = vdupq_n_s16(0);
            }
            for k in 0..4 {
                for r in 4..8 {
                    st_8h(x6.add(32 * k + 8 * (r - 4)) as Dst, v[r]);
                }
            }
            break;
        }
        // mov x8, #8*4
        inv_txfm_horz_dct_32x4::<false>(&mut v, x6, coeff.add(i), 8);
    }

    // 2: mov x8, #2*32; mov w9, #0
    let x8 = 32;
    let mut w9 = 0usize;
    loop {
        // 1: add x6, x0, x9, lsl #1; add x7, sp, x9, lsl #1
        let x6 = dst.add(w9);
        let x7 = sp.add(w9);
        for i in 0..8 {
            v[16 + i] = vld1q_s16(x7.add(x8 * i));
        }
        w9 += 8;

        // bl X(inv_dct_8h_x8_neon)
        call::<true>(K::Dct, 8, &mut v);

        // cmp w9, #32
        load_add_store_8x8::<4>(x6, stride, &v);

        // b.lt 1b
        if w9 >= 32 {
            break;
        }
    }
    // add sp, sp, #512
}

// --- 64-point, first pass (32 bits) ------------------------------------------------

/// `inv_dct64_step1_neon`: four inputs (v16–v19) to the eight values
/// t32a/33/34a/35/60/61a/62/63a (or the corresponding ones of the other
/// three groups) at `x6`, which advances by eight vectors; `x17` walks
/// `idct64_coeffs` (12 values per call). `v5`/`v4` hold the row clip
/// constants.
#[inline(always)]
unsafe fn inv_dct64_step1(v: &mut V, x6: &mut *mut i32, x17: &mut usize) {
    // in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
    // in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
    // in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
    // in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a

    // ld1 {v0.4s, v1.4s}, [x17], #32
    v[0] = coeffs(&IDCT64_COEFFS, *x17);
    v[1] = coeffs(&IDCT64_COEFFS, *x17 + 4);
    *x17 += 8;

    v[23] = sqrdmulh_4s::<1>(v[16], v[0]); // t63a
    v[16] = sqrdmulh_4s::<0>(v[16], v[0]); // t32a
    v[22] = sqrdmulh_4s::<2>(v[17], v[0]); // t62a
    v[17] = sqrdmulh_4s::<3>(v[17], v[0]); // t33a
    v[21] = sqrdmulh_4s::<1>(v[18], v[1]); // t61a
    v[18] = sqrdmulh_4s::<0>(v[18], v[1]); // t34a
    v[20] = sqrdmulh_4s::<2>(v[19], v[1]); // t60a
    v[19] = sqrdmulh_4s::<3>(v[19], v[1]); // t35a

    // ld1 {v0.4s}, [x17], #16
    v[0] = coeffs(&IDCT64_COEFFS, *x17);
    *x17 += 4;

    v[24] = sqadd_4s(v[16], v[17]); // t32
    v[25] = sqsub_4s(v[16], v[17]); // t33
    v[26] = sqsub_4s(v[19], v[18]); // t34
    v[27] = sqadd_4s(v[19], v[18]); // t35
    v[28] = sqadd_4s(v[20], v[21]); // t60
    v[29] = sqsub_4s(v[20], v[21]); // t61
    v[30] = sqsub_4s(v[23], v[22]); // t62
    v[31] = sqadd_4s(v[23], v[22]); // t63

    clip_4s(v, &[24, 25, 26, 27, 28, 29, 30, 31], 5, 4);

    v[2] = mul_mla::<0, 1>(v[29], v[26], v[0], v[0]); // -> t34a
    v[7] = mul_mls::<1, 0>(v[29], v[26], v[0], v[0]); // -> t61a
    v[2] = neg_4s(v[2]); // t34a
    v[6] = mul_mls::<1, 0>(v[30], v[25], v[0], v[0]); // -> t33a
    v[26] = srshr_4s::<12>(v[2]); // t34a
    v[2] = mul_mla::<0, 1>(v[30], v[25], v[0], v[0]); // -> t62a
    v[29] = srshr_4s::<12>(v[7]); // t61a
    v[25] = srshr_4s::<12>(v[6]); // t33a
    v[30] = srshr_4s::<12>(v[2]); // t62a

    v[16] = sqadd_4s(v[24], v[27]); // t32a
    v[19] = sqsub_4s(v[24], v[27]); // t35a
    v[17] = sqadd_4s(v[25], v[26]); // t33
    v[18] = sqsub_4s(v[25], v[26]); // t34
    v[20] = sqsub_4s(v[31], v[28]); // t60a
    v[23] = sqadd_4s(v[31], v[28]); // t63a
    v[21] = sqsub_4s(v[30], v[29]); // t61
    v[22] = sqadd_4s(v[30], v[29]); // t62

    clip_4s(v, &[16, 19, 17, 18, 20, 23, 21, 22], 5, 4);

    v[2] = mul_mla::<2, 3>(v[21], v[18], v[0], v[0]); // -> t61a
    v[7] = mul_mls::<3, 2>(v[21], v[18], v[0], v[0]); // -> t34a
    v[6] = mul_mla::<2, 3>(v[20], v[19], v[0], v[0]); // -> t60
    v[21] = srshr_4s::<12>(v[2]); // t61a
    v[18] = srshr_4s::<12>(v[7]); // t34a
    v[2] = mul_mls::<3, 2>(v[20], v[19], v[0], v[0]); // -> t35
    v[20] = srshr_4s::<12>(v[6]); // t60
    v[19] = srshr_4s::<12>(v[2]); // t35

    // st1 {v16.4s, v17.4s, v18.4s, v19.4s}, [x6], #64
    // st1 {v20.4s, v21.4s, v22.4s, v23.4s}, [x6], #64
    for k in 0..8 {
        st_4s(x6.add(4 * k), v[16 + k]);
    }
    *x6 = x6.wrapping_add(32);
}

/// `inv_dct64_step2_neon`: the butterflies of t32..t63 in place, from the
/// outside in (`x6` upwards from t32a, `x9` downwards from t39a, both in
/// vectors of four values, groups eight vectors apart). `v5`/`v4` hold the
/// row clip constants.
#[inline(always)]
unsafe fn inv_dct64_step2(v: &mut V, x6: *mut i32, x9: *mut i32) {
    // ldrel q0, x16, idct_coeffs
    v[0] = coeffs(&IDCT_COEFFS, 0);
    let mut x6 = x6;
    let mut x9 = x9;
    loop {
        // 1:
        // t32a/33/34a/35/60/61a/62/63a
        // t56a/57/58a/59/36/37a/38/39a
        // t40a/41/42a/43/52/53a/54/55a
        // t48a/49/50a/51/44/45a/46/47a
        v[16] = ld_4s(x6); // t32a
        v[17] = ld_4s(x9.add(32)); // t39a
        v[18] = ld_4s(x9); // t63a
        v[19] = ld_4s(x6.add(32)); // t56a
        v[20] = ld_4s(x6.add(64)); // t40a
        v[21] = ld_4s(x9.add(96)); // t47a
        v[22] = ld_4s(x9.add(64)); // t55a
        v[23] = ld_4s(x6.add(96)); // t48a

        v[24] = sqadd_4s(v[16], v[17]); // t32
        v[25] = sqsub_4s(v[16], v[17]); // t39
        v[26] = sqadd_4s(v[18], v[19]); // t63
        v[27] = sqsub_4s(v[18], v[19]); // t56
        v[28] = sqsub_4s(v[21], v[20]); // t40
        v[29] = sqadd_4s(v[21], v[20]); // t47
        v[30] = sqadd_4s(v[23], v[22]); // t48
        v[31] = sqsub_4s(v[23], v[22]); // t55

        clip_4s(v, &[24, 25, 26, 27, 28, 29, 30, 31], 5, 4);

        v[2] = mul_mla::<3, 2>(v[27], v[25], v[0], v[0]); // -> t56a
        v[7] = mul_mls::<2, 3>(v[27], v[25], v[0], v[0]); // -> t39a
        v[6] = mul_mla::<3, 2>(v[31], v[28], v[0], v[0]); // -> t40a
        v[25] = srshr_4s::<12>(v[2]); // t56a
        v[27] = srshr_4s::<12>(v[7]); // t39a
        v[6] = neg_4s(v[6]); // t40a
        v[2] = mul_mls::<2, 3>(v[31], v[28], v[0], v[0]); // -> t55a
        v[31] = srshr_4s::<12>(v[6]); // t40a
        v[28] = srshr_4s::<12>(v[2]); // t55a

        v[16] = sqadd_4s(v[24], v[29]); // t32a
        v[19] = sqsub_4s(v[24], v[29]); // t47a
        v[17] = sqadd_4s(v[27], v[31]); // t39
        v[18] = sqsub_4s(v[27], v[31]); // t40
        v[20] = sqsub_4s(v[26], v[30]); // t48a
        v[23] = sqadd_4s(v[26], v[30]); // t63a
        v[21] = sqsub_4s(v[25], v[28]); // t55
        v[22] = sqadd_4s(v[25], v[28]); // t56

        clip_4s(v, &[16, 19, 17, 18, 20, 23, 21, 22], 5, 4);

        v[2] = mul_mls::<0, 0>(v[21], v[18], v[0], v[0]); // -> t40a
        v[7] = mul_mla::<0, 0>(v[21], v[18], v[0], v[0]); // -> t55a
        v[6] = mul_mls::<0, 0>(v[20], v[19], v[0], v[0]); // -> t47
        v[18] = srshr_4s::<12>(v[2]); // t40a
        v[21] = srshr_4s::<12>(v[7]); // t55a
        v[2] = mul_mla::<0, 0>(v[20], v[19], v[0], v[0]); // -> t48
        v[19] = srshr_4s::<12>(v[6]); // t47
        v[20] = srshr_4s::<12>(v[2]); // t48

        st_4s(x6, v[16]); // t32a
        st_4s(x9, v[17]); // t39
        st_4s(x6.add(32), v[18]); // t40a
        st_4s(x9.add(32), v[19]); // t47
        st_4s(x6.add(64), v[20]); // t48
        st_4s(x9.add(64), v[21]); // t55a
        st_4s(x6.add(96), v[22]); // t56
        st_4s(x9.add(96), v[23]); // t63a

        // add x6, x6, #4*4; sub x9, x9, #4*4; cmp x6, x9; b.lt 1b
        x6 = x6.add(4);
        x9 = x9.sub(4);
        if x6 >= x9 {
            break;
        }
    }
}

/// The `store_addsub r0, r1, r2, r3` macro of `def_dct64_func`: the four
/// even vectors at `x6` plus `r` (clipped against `v1`/`v0`) back to `x6`,
/// minus `r` (clipped) downwards from `x10` (`x9` = -16 bytes).
#[inline(always)]
unsafe fn dct64_store_addsub(v: &mut V, x6: &mut *mut i32, x10: &mut *mut i32, x9: isize, r: [usize; 4]) {
    let [r0, r1, r2, r3] = r;
    v[2] = ld_4s(*x6);
    *x6 = x6.add(4);
    v[3] = ld_4s(*x6);
    *x6 = x6.add(4);
    v[6] = sqadd_4s(v[2], v[r0]);
    v[r0] = sqsub_4s(v[2], v[r0]);
    v[4] = ld_4s(*x6);
    *x6 = x6.add(4);
    v[7] = sqadd_4s(v[3], v[r1]);
    v[r1] = sqsub_4s(v[3], v[r1]);
    v[6] = smin_4s(v[6], v[1]);
    v[r0] = smin_4s(v[r0], v[1]);
    v[5] = ld_4s(*x6);
    *x6 = x6.add(4);
    v[2] = sqadd_4s(v[4], v[r2]);
    // sub x6, x6, #16*4
    *x6 = x6.sub(16);
    v[6] = smax_4s(v[6], v[0]);
    v[r0] = smax_4s(v[r0], v[0]);
    v[r2] = sqsub_4s(v[4], v[r2]);
    v[7] = smin_4s(v[7], v[1]);
    v[r1] = smin_4s(v[r1], v[1]);
    st_4s(*x6, v[6]);
    *x6 = x6.add(4);
    st_4s(*x10, v[r0]);
    *x10 = x10.wrapping_offset(x9);
    v[2] = smin_4s(v[2], v[1]);
    v[r2] = smin_4s(v[r2], v[1]);
    v[7] = smax_4s(v[7], v[0]);
    v[r1] = smax_4s(v[r1], v[0]);
    v[3] = sqadd_4s(v[5], v[r3]);
    v[r3] = sqsub_4s(v[5], v[r3]);
    v[2] = smax_4s(v[2], v[0]);
    v[r2] = smax_4s(v[r2], v[0]);
    v[3] = smin_4s(v[3], v[1]);
    v[r3] = smin_4s(v[r3], v[1]);
    st_4s(*x6, v[7]);
    *x6 = x6.add(4);
    st_4s(*x10, v[r1]);
    *x10 = x10.wrapping_offset(x9);
    v[3] = smax_4s(v[3], v[0]);
    v[r3] = smax_4s(v[r3], v[0]);
    st_4s(*x6, v[2]);
    *x6 = x6.add(4);
    st_4s(*x10, v[r2]);
    *x10 = x10.wrapping_offset(x9);
    st_4s(*x6, v[3]);
    *x6 = x6.add(4);
    st_4s(*x10, v[r3]);
    *x10 = x10.wrapping_offset(x9);
}

/// The `load8 src, strd, zero, clear` macro: v16–v23 from `x7` (an element
/// offset from `c`), `x8` values apart; with `CLEAR` each is replaced by
/// `v7` (zero).
#[inline(always)]
unsafe fn load8<const CLEAR: bool>(v: &mut V, c: Coef, x7: &mut isize, x8: isize) {
    for i in 16..24 {
        let p = c.offset(*x7);
        v[i] = ld_4s(p);
        if CLEAR {
            st_4s(p, v[7]);
        }
        *x7 += x8;
    }
}

/// The `clear_upper8` macro: `movi v24.4s–v31.4s, #0`.
#[inline(always)]
fn clear_upper8(v: &mut V) {
    for i in 24..32 {
        v[i] = dup_4s(0);
    }
}

/// `movz16dup_if v0.2s, w16, #2896*8, scale`; `movi_if v7, #0, clear`.
#[inline(always)]
fn dct64_setup<const CLEAR: bool, const SCALE: bool>(v: &mut V) {
    if SCALE {
        v[0] = dup_2s(W2896);
    }
    if CLEAR {
        v[7] = dup_4s(0);
    }
}

/// `scale_if scale, v0.s[0], r…`
#[inline(always)]
fn scale_if<const SCALE: bool>(v: &mut V, regs: &[usize]) {
    if SCALE {
        let c = v[0];
        scale_input::<0>(v, c, regs);
    }
}

/// `inv_txfm_dct_clear_4s_x64_neon` (`CLEAR`) and
/// `inv_txfm_dct_clear_scale_4s_x64_neon` (`CLEAR`, `SCALE`) of
/// `def_dct64_func` (the assembly instantiates no other form): a 64-point
/// DCT of four columns of `x7` (32 rows, `x8` values apart; the upper 32
/// inputs are zero) into 64 vectors of four values at `sp`.
#[inline(always)]
unsafe fn inv_txfm_dct_4s_x64<const CLEAR: bool, const SCALE: bool>(v: &mut V, sp: *mut i32, x7: Coef, x8: usize) {
    // mov x6, sp; lsl x8, x8, #2
    let mut x6 = sp;
    let mut x8 = 4 * x8 as isize;
    // x7 as an element offset from the coefficient pointer passed in
    let c = x7;
    let mut x7: isize = 0;

    dct64_setup::<CLEAR, SCALE>(v);
    load8::<CLEAR>(v, c, &mut x7, x8);
    clear_upper8(v);
    // sub x7, x7, x8, lsl #3; add x7, x7, x8, lsr #1
    x7 = x7 - 8 * x8 + x8 / 2;
    scale_if::<SCALE>(v, &[16, 17, 18, 19, 20, 21, 22, 23]);

    inv_dct_4s_x16(v);

    // idct_16 leaves the row_clip_max/min constants in v5 and v4
    clip_4s(
        v,
        &[16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31],
        5,
        4,
    );

    // store16 x6
    for i in 0..16 {
        st_4s(x6.add(4 * i), v[16 + i]);
    }
    x6 = x6.add(64);

    dct64_setup::<CLEAR, SCALE>(v);
    load8::<CLEAR>(v, c, &mut x7, x8);
    clear_upper8(v);
    // sub x7, x7, x8, lsl #3; lsr x8, x8, #1; sub x7, x7, x8, lsr #1
    x7 -= 8 * x8;
    x8 /= 2;
    x7 -= x8 / 2;
    scale_if::<SCALE>(v, &[16, 17, 18, 19, 20, 21, 22, 23]);

    inv_dct32_odd_4s_x16(v);

    // add x10, x6, #16*15; sub x6, x6, #16*16; mov x9, #-16
    let mut x10 = x6.add(60);
    x6 = x6.sub(64);
    let x9: isize = -4;

    v[1] = dup_4s(ROW_CLIP_MAX);
    v[0] = dup_4s(ROW_CLIP_MIN);

    dct64_store_addsub(v, &mut x6, &mut x10, x9, [31, 30, 29, 28]);
    dct64_store_addsub(v, &mut x6, &mut x10, x9, [27, 26, 25, 24]);
    dct64_store_addsub(v, &mut x6, &mut x10, x9, [23, 22, 21, 20]);
    dct64_store_addsub(v, &mut x6, &mut x10, x9, [19, 18, 17, 16]);

    // add x6, x6, #4*4*16
    x6 = x6.add(64);

    // movrel x17, idct64_coeffs
    let mut x17 = 0usize;
    v[5] = dup_4s(ROW_CLIP_MAX);
    v[4] = dup_4s(ROW_CLIP_MIN);
    dct64_setup::<CLEAR, SCALE>(v);
    let at = |o: isize| c.offset(o);
    let mut x9 = x7 + 16 * x8; // offset 16
    let mut x10 = x7 + 8 * x8; // offset 8
    x9 -= x8; // offset 15
    let mut x11 = x10 - x8; // offset 7
    v[16] = ld_4s(at(x7)); // in1  (offset 0)
    v[17] = ld_4s(at(x9)); // in31 (offset 15)
    v[18] = ld_4s(at(x10)); // in17 (offset 8)
    v[19] = ld_4s(at(x11)); // in15 (offset 7)
    if CLEAR {
        st_4s(at(x7), v[7]);
        st_4s(at(x9), v[7]);
        st_4s(at(x10), v[7]);
        st_4s(at(x11), v[7]);
    }
    scale_if::<SCALE>(v, &[16, 17, 18, 19]);
    inv_dct64_step1(v, &mut x6, &mut x17);

    dct64_setup::<CLEAR, SCALE>(v);
    x7 += 4 * x8; // offset 4
    x9 -= 4 * x8; // offset 11
    x10 = x7 - x8; // offset 3
    x11 = x9 + x8; // offset 12
    v[16] = ld_4s(at(x10)); // in7  (offset 3)
    v[17] = ld_4s(at(x11)); // in25 (offset 12)
    v[18] = ld_4s(at(x9)); // in23 (offset 11)
    v[19] = ld_4s(at(x7)); // in9  (offset 4)
    if CLEAR {
        st_4s(at(x7), v[7]);
        st_4s(at(x9), v[7]);
        st_4s(at(x10), v[7]);
        st_4s(at(x11), v[7]);
    }
    scale_if::<SCALE>(v, &[16, 17, 18, 19]);
    inv_dct64_step1(v, &mut x6, &mut x17);

    dct64_setup::<CLEAR, SCALE>(v);
    x10 -= 2 * x8; // offset 1
    x9 -= 2 * x8; // offset 9
    x7 += x8; // offset 5
    x11 += x8; // offset 13
    v[16] = ld_4s(at(x10 + x8)); // in5  (offset 2)
    v[17] = ld_4s(at(x11)); // in27 (offset 13)
    v[18] = ld_4s(at(x9 + x8)); // in21 (offset 10)
    v[19] = ld_4s(at(x7)); // in11 (offset 5)
    if CLEAR {
        st_4s(at(x10 + x8), v[7]);
        st_4s(at(x11), v[7]);
        st_4s(at(x9 + x8), v[7]);
        st_4s(at(x7), v[7]);
    }
    scale_if::<SCALE>(v, &[16, 17, 18, 19]);
    inv_dct64_step1(v, &mut x6, &mut x17);

    dct64_setup::<CLEAR, SCALE>(v);
    v[16] = ld_4s(at(x10)); // in3  (offset 1)
    v[17] = ld_4s(at(x11 + x8)); // in29 (offset 14)
    v[18] = ld_4s(at(x9)); // in19 (offset 9)
    v[19] = ld_4s(at(x7 + x8)); // in13 (offset 6)
    if CLEAR {
        st_4s(at(x10), v[7]);
        st_4s(at(x11 + x8), v[7]);
        st_4s(at(x9), v[7]);
        st_4s(at(x7 + x8), v[7]);
    }
    scale_if::<SCALE>(v, &[16, 17, 18, 19]);
    inv_dct64_step1(v, &mut x6, &mut x17);

    // sub x6, x6, #4*4*32; add x9, x6, #4*4*7
    let x6 = x6.wrapping_sub(128);
    let x9 = x6.add(28);

    inv_dct64_step2(v, x6, x9);
}

/// The `store_addsub src0, src1, src2, src3` macro of
/// `inv_txfm_horz_dct_64x4_neon`: `src0 ± src1` and `src2 ± src3`, `srshl`
/// by `v7`, narrowed; the sums to row `x6`, the reversed differences to row
/// `x9`, both rows advanced by `x10` (64 values).
#[inline(always)]
unsafe fn horz_64x4_store_addsub(v: &mut V, x6: &mut *mut i16, x9: &mut *mut i16, r: [usize; 4]) {
    let [s0, s1, s2, s3] = r;
    v[1] = sqsub_4s(v[s0], v[s1]);
    v[0] = sqadd_4s(v[s0], v[s1]);
    v[3] = sqsub_4s(v[s2], v[s3]);
    v[1] = srshl_4s(v[1], v[7]);
    v[2] = sqadd_4s(v[s2], v[s3]);
    v[3] = srshl_4s(v[3], v[7]);
    v[0] = srshl_4s(v[0], v[7]);
    v[2] = srshl_4s(v[2], v[7]);
    v[3] = sqxtn(v[3]);
    v[3] = sqxtn2(v[3], v[1]);
    v[0] = sqxtn(v[0]);
    v[0] = sqxtn2(v[0], v[2]);
    v[3] = rev64_8h(v[3]);
    // st1 {v0.8h}, [x6], x10; st1 {v3.8h}, [x9], x10
    st_8h(*x6 as Dst, v[0]);
    *x6 = x6.wrapping_add(64);
    st_8h(*x9 as Dst, v[3]);
    *x9 = x9.wrapping_add(64);
}

/// `inv_txfm_horz_dct_64x4_neon`: the last butterflies of the 64-point DCT
/// at `sp` (64 vectors of four values), transposed into four rows of 64
/// values at `x6`, with `srshl` by `w12` (-2 or -1).
#[inline(always)]
unsafe fn inv_txfm_horz_dct_64x4(v: &mut V, sp: *const i32, x6: *mut i16, w12: i32) {
    // mov x7, sp; add x8, sp, #4*4*(64 - 4); add x9, x6, #2*56;
    // mov x10, #2*64; mov x11, #-4*4*4
    let mut x7 = sp;
    let mut x8 = sp.add(4 * (64 - 4));
    let mut x6 = x6;
    let mut x9 = x6.add(56);

    // dup v7.4s, w12
    v[7] = dup_4s(w12);
    loop {
        // 1:
        for k in 0..4 {
            v[16 + k] = ld_4s(x7.add(4 * k));
        }
        x7 = x7.add(16);
        for k in 0..4 {
            v[28 + k] = ld_4s(x8.add(4 * k));
        }
        x8 = x8.wrapping_sub(16);
        for k in 0..4 {
            v[20 + k] = ld_4s(x7.add(4 * k));
        }
        x7 = x7.add(16);
        for k in 0..4 {
            v[24 + k] = ld_4s(x8.add(4 * k));
        }
        x8 = x8.wrapping_sub(16);
        transpose_4x4s(v, [16, 17, 18, 19], [2, 3, 4, 5]);
        transpose_4x4s(v, [20, 21, 22, 23], [2, 3, 4, 5]);
        transpose_4x4s(v, [31, 30, 29, 28], [2, 3, 4, 5]);
        transpose_4x4s(v, [27, 26, 25, 24], [2, 3, 4, 5]);

        horz_64x4_store_addsub(v, &mut x6, &mut x9, [16, 31, 20, 27]);
        horz_64x4_store_addsub(v, &mut x6, &mut x9, [17, 30, 21, 26]);
        horz_64x4_store_addsub(v, &mut x6, &mut x9, [18, 29, 22, 25]);
        horz_64x4_store_addsub(v, &mut x6, &mut x9, [19, 28, 23, 24]);
        // sub x6, x6, x10, lsl #2; sub x9, x9, x10, lsl #2;
        // add x6, x6, #16; sub x9, x9, #16
        x6 = x6.wrapping_sub(4 * 64).wrapping_add(8);
        x9 = x9.wrapping_sub(4 * 64).wrapping_sub(8);

        // cmp x7, x8; b.lt 1b
        if x7 >= x8 {
            break;
        }
    }
}

// --- 64-point, second pass (16 bits) -----------------------------------------------

/// The `add_dest_addsub src0, src1, src2, src3` macro: `src0 ± src1` added
/// to rows `x6` and `x9`, `src2 ± src3` to rows `x6 + 1` and `x9 - 1`; both
/// pointers advance by two rows towards each other.
#[inline(always)]
unsafe fn add_dest_addsub(v: &mut V, x6: &mut Dst, x9: &mut Dst, stride: isize, r: [usize; 4]) {
    let [s0, s1, s2, s3] = r;
    // x10 = -x1
    v[0] = ld_8h(*x6);
    *x6 = wrow(*x6, stride, 1);
    v[1] = ld_8h(*x9);
    *x9 = wrow(*x9, stride, -1);
    v[4] = sqadd::<true>(v[s0], v[s1]);
    v[2] = ld_8h(*x6);
    v[s0] = sqsub::<true>(v[s0], v[s1]);
    v[3] = ld_8h(*x9);
    v[5] = sqadd::<true>(v[s2], v[s3]);
    v[s2] = sqsub::<true>(v[s2], v[s3]);
    // sub x6, x6, x1; sub x9, x9, x10
    *x6 = wrow(*x6, stride, -1);
    *x9 = wrow(*x9, stride, 1);
    v[4] = srshr::<4>(v[4]);
    v[5] = srshr::<4>(v[5]);
    v[s0] = srshr::<4>(v[s0]);
    v[0] = usqadd(v[0], v[4]);
    v[s2] = srshr::<4>(v[s2]);
    v[1] = usqadd(v[1], v[s0]);
    v[2] = usqadd(v[2], v[5]);
    v[0] = smin_8h(v[0], v[7]);
    v[3] = usqadd(v[3], v[s2]);
    v[1] = smin_8h(v[1], v[7]);
    st_8h(*x6, v[0]);
    *x6 = wrow(*x6, stride, 1);
    v[2] = smin_8h(v[2], v[7]);
    st_8h(*x9, v[1]);
    *x9 = wrow(*x9, stride, -1);
    v[3] = smin_8h(v[3], v[7]);
    st_8h(*x6, v[2]);
    *x6 = wrow(*x6, stride, 1);
    st_8h(*x9, v[3]);
    *x9 = wrow(*x9, stride, -1);
}

/// `inv_txfm_add_vert_dct_8x64_neon`: the last butterflies of the 64-point
/// DCT that `inv_txfm_dct_8h_x64_neon` left at `sp` (64 vectors of eight
/// values), added to 64 rows of 8 pixels at `x6`, top and bottom halves
/// meeting in the middle.
#[inline(always)]
unsafe fn inv_txfm_add_vert_dct_8x64(v: &mut V, sp: *const i16, x6: Dst, stride: isize) {
    // mov x7, sp; add x8, sp, #2*8*(64 - 4); add x9, x6, x1, lsl #6;
    // sub x9, x9, x1; neg x10, x1; mov x11, #-2*8*4
    let mut x7 = sp;
    let mut x8 = sp.add(8 * (64 - 4));
    let mut x6 = x6;
    let mut x9 = row(x6, stride, 63);

    loop {
        // 1:
        for k in 0..4 {
            v[16 + k] = vld1q_s16(x7.add(8 * k));
        }
        x7 = x7.add(32);
        for k in 0..4 {
            v[28 + k] = vld1q_s16(x8.add(8 * k));
        }
        x8 = x8.wrapping_sub(32);
        for k in 0..4 {
            v[20 + k] = vld1q_s16(x7.add(8 * k));
        }
        x7 = x7.add(32);
        for k in 0..4 {
            v[24 + k] = vld1q_s16(x8.add(8 * k));
        }
        x8 = x8.wrapping_sub(32);

        // mvni v7.8h, #0xfc, lsl #8
        v[7] = mvni_3ff();
        add_dest_addsub(v, &mut x6, &mut x9, stride, [16, 31, 17, 30]);
        add_dest_addsub(v, &mut x6, &mut x9, stride, [18, 29, 19, 28]);
        add_dest_addsub(v, &mut x6, &mut x9, stride, [20, 27, 21, 26]);
        add_dest_addsub(v, &mut x6, &mut x9, stride, [22, 25, 23, 24]);

        // cmp x7, x8; b.lt 1b
        if x7 >= x8 {
            break;
        }
    }
}

// --- dct_dct 64 ---------------------------------------------------------------------

/// The 64-point scratch at the bottom of the stack frame (`64*4*4` or
/// `64*8*2` bytes): 64 `.4s` vectors for `inv_txfm_dct_*_4s_x64_neon`, 64
/// `.8h` vectors for `inv_txfm_dct_8h_x64_neon`.
type Scratch64 = [i32; 64 * 4];

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_64x64_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<64, 64, 2>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_64x64_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_64x64_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_64x64_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub_sp 64*32*2+64*4*4; add x5, sp, #64*4*4
    let mut scratch: Scratch64 = [0; 64 * 4];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i32; 64 * 32 * 2 / 4];
    let x5 = buf.as_mut_ptr() as *mut i16;

    // movrel x13, eob_32x32 (w12 is first read after the first ldrh)
    let mut x13 = 0;
    let mut w12: c_int = 0;

    for i in [0usize, 4, 8, 12, 16, 20, 24, 28] {
        // add x6, x5, #(\i*64*2)
        let x6 = x5.add(i * 64);
        if i > 0 {
            let w8 = (32 - i) as i32;
            if eob < w12 {
                zero_rest::<2, 4>(&mut v, x6, w8);
                break;
            }
        }
        // add x7, x2, #(\i*4); mov x8, #32*4; mov x12, #-2 // shift
        inv_txfm_dct_4s_x64::<true, false>(&mut v, sp, coeff.add(i), 32);
        // add x6, x5, #(\i*64*2)
        inv_txfm_horz_dct_64x4(&mut v, sp, x6, -2);
        if i < 28 {
            // ldrh w12, [x13], #2
            w12 = EOB_32X32[x13] as c_int;
            x13 += 1;
        }
    }

    // 3:
    for i in [0usize, 8, 16, 24, 32, 40, 48, 56] {
        // add x7, x5, #(\i*2); mov x8, #64*2; bl X(inv_txfm_dct_8h_x64_neon)
        super::itx::dct_8h_x64::<false, false>(&mut v, sp as *mut i16, x5.add(i), 64);
        // add x6, x0, #(\i*2)
        inv_txfm_add_vert_dct_8x64(&mut v, sp as *const i16, dst.add(i), stride);
    }
    // add sp, x5, #64*32*2
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_64x32_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<64, 32, 1>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_64x32_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_64x32_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_64x32_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub_sp 64*32*2+64*4*4; add x5, sp, #64*4*4
    let mut scratch: Scratch64 = [0; 64 * 4];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i32; 64 * 32 * 2 / 4];
    let x5 = buf.as_mut_ptr() as *mut i16;

    // movrel x13, eob_32x32
    let mut x13 = 0;
    let mut w12: c_int = 0;

    for i in [0usize, 4, 8, 12, 16, 20, 24, 28] {
        let x6 = x5.add(i * 64);
        if i > 0 {
            let w8 = (32 - i) as i32;
            if eob < w12 {
                zero_rest::<2, 4>(&mut v, x6, w8);
                break;
            }
        }
        // add x7, x2, #(\i*4); mov x8, #32*4; mov x12, #-1 // shift
        inv_txfm_dct_4s_x64::<true, true>(&mut v, sp, coeff.add(i), 32);
        inv_txfm_horz_dct_64x4(&mut v, sp, x6, -1);
        if i < 28 {
            w12 = EOB_32X32[x13] as c_int;
            x13 += 1;
        }
    }

    // 3:
    for i in [0usize, 8, 16, 24, 32, 40, 48, 56] {
        // add x6, x0, #(\i*2); add x7, x5, #(\i*2); mov x8, #64*2
        inv_txfm_add_vert_dct_8x32(&mut v, dst.add(i), stride, x5.add(i), 64);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x64_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<32, 64, 1>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_32x64_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_32x64_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_32x64_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub_sp 32*32*2+64*8*2; add x5, sp, #64*8*2
    let mut scratch: Scratch64 = [0; 64 * 4];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i32; 32 * 32 * 2 / 4];
    let x5 = buf.as_mut_ptr() as *mut i16;

    // movrel x13, eob_32x32; ldrh w12, [x13], #2
    let mut x13 = 0;
    let mut w12 = EOB_32X32[x13] as c_int;
    x13 += 1;

    for i in [0usize, 4, 8, 12, 16, 20, 24, 28] {
        // add x6, x5, #(\i*32*2)
        let x6 = x5.add(i * 32);
        if i > 0 {
            let w8 = (32 - i) as i32;
            if eob < w12 {
                zero_rest::<4, 4>(&mut v, x6, w8);
                break;
            }
            // ldrh w12, [x13], #2
            w12 = EOB_32X32[x13] as c_int;
            x13 += 1;
        }
        // add x7, x2, #(\i*4); mov x8, #32*4
        inv_txfm_horz_dct_32x4::<true>(&mut v, x6, coeff.add(i), 32);
    }

    // 3:
    for i in [0usize, 8, 16, 24] {
        // add x7, x5, #(\i*2); mov x8, #32*2; bl X(inv_txfm_dct_8h_x64_neon)
        super::itx::dct_8h_x64::<false, false>(&mut v, sp as *mut i16, x5.add(i), 32);
        // add x6, x0, #(\i*2)
        inv_txfm_add_vert_dct_8x64(&mut v, sp as *const i16, dst.add(i), stride);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_64x16_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<64, 16, 2>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_64x16_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_64x16_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_64x16_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub_sp 64*16*2+64*4*4; add x4, sp, #64*4*4
    let mut scratch: Scratch64 = [0; 64 * 4];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i32; 64 * 16 * 2 / 4];
    let x4 = buf.as_mut_ptr() as *mut i16;

    // movrel x13, eob_16x32
    let mut x13 = 0;
    let mut w12: c_int = 0;

    for i in [0usize, 4, 8, 12] {
        // add x6, x4, #(\i*64*2)
        let x6 = x4.add(i * 64);
        if i > 0 {
            let w8 = (16 - i) as i32;
            if eob < w12 {
                zero_rest::<2, 4>(&mut v, x6, w8);
                break;
            }
        }
        // add x7, x2, #(\i*4); mov x8, #16*4; mov x12, #-2 // shift
        inv_txfm_dct_4s_x64::<true, false>(&mut v, sp, coeff.add(i), 16);
        inv_txfm_horz_dct_64x4(&mut v, sp, x6, -2);
        if i < 12 {
            w12 = EOB_16X32[x13] as c_int;
            x13 += 1;
        }
    }

    // 3: movrel x5, X(inv_dct_8h_x16_neon)
    let x5 = K::Dct;
    for i in [0usize, 8, 16, 24, 32, 40, 48, 56] {
        // add x6, x0, #(\i*2); add x7, x4, #(\i*2); mov x8, #64*2
        inv_txfm_add_vert_8x16(&mut v, dst.add(i), stride, x4.add(i), 64, x5);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x64_16bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    _bitdepth_max: c_int,
) {
    if eob == 0 {
        return idct_dc::<16, 64, 2>(dst, stride, coeff);
    }
    inv_txfm_add_dct_dct_16x64_16bpc_coefs(dst, stride, coeff, eob)
}

/// The part of `dav1d_inv_txfm_add_dct_dct_16x64_16bpc_neon` after the `idct_dc` macro (`1:`).
unsafe fn inv_txfm_add_dct_dct_16x64_16bpc_coefs(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    // sub_sp 16*32*2+64*8*2; add x5, sp, #64*8*2
    let mut scratch: Scratch64 = [0; 64 * 4];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i32; 16 * 32 * 2 / 4];
    let x5 = buf.as_mut_ptr() as *mut i16;

    // movrel x13, eob_16x32; ldrh w12, [x13], #2
    let mut x13 = 0;
    let mut w12 = EOB_16X32[x13] as c_int;
    x13 += 1;

    // adr x4, inv_dct_4s_x16_neon
    let x4 = K::Dct;
    for i in [0usize, 4, 8, 12, 16, 20, 24, 28] {
        // add x6, x5, #(\i*16*2)
        let x6 = x5.add(i * 16);
        if i > 0 {
            let w8 = (32 - i) as i32;
            if eob < w12 {
                zero_rest::<4, 2>(&mut v, x6, w8);
                break;
            }
            if i < 28 {
                w12 = EOB_16X32[x13] as c_int;
                x13 += 1;
            }
        }
        // add x7, x2, #(\i*4); mov x8, #32*4
        inv_txfm_horz_16x4::<false>(&mut v, x6, coeff.add(i), 32, x4);
    }

    // 3:
    for i in [0usize, 8] {
        // add x7, x5, #(\i*2); mov x8, #16*2; bl X(inv_txfm_dct_8h_x64_neon)
        super::itx::dct_8h_x64::<false, false>(&mut v, sp as *mut i16, x5.add(i), 16);
        // add x6, x0, #(\i*2)
        inv_txfm_add_vert_dct_8x64(&mut v, sp as *const i16, dst.add(i), stride);
    }
}
