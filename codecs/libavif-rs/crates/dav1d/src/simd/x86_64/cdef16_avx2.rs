// Copyright © 2021, VideoLAN and dav1d authors
// Copyright © 2021, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/cdef16_avx2.asm` (16 bpc, used at 10 and 12 bit) with AVX2
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The four entry points `dav1d_cdef_filter_{4x4,4x8,8x8}_16bpc_avx2` and `dav1d_cdef_dir_16bpc_avx2`.
//! Each filter copies the block and the neighbors the edge flags allow into a 16-bit buffer `px` on its
//! stack (missing neighbors are `pw_m16384`, 0xc000), then filters `px` in groups of two rows (8 pixels
//! wide) or four rows (4 pixels wide), one 256-bit register per group. The direction search scales the
//! pixels to 8 bits and runs the 8 bpc kernel's `.main`.
//!
//! Correspondence with the assembly:
//! - Each `cglobal` is the `extern "C"` function of the same name; its padding code (from the entry to
//!   `.padding_done`) is the `#[inline(always)]` function `cdef_filter_<W>x<H>_16bpc`, with the
//!   jumps between `.no_top`, `.top_no_left`, `.top_done` (and the same for the bottom and the left)
//!   as a labeled block and `break`.
//! - The macro `CDEF_FILTER w, h` is `cdef_filter::<W, H>` (its entry, `.pri_only`, `.pri_end`,
//!   `.sec_only`, `.end`), with the `%if`s on the macro's arguments as `if`s on the constants. The three
//!   subroutines that the macro defines in its square expansions (`%if %1 == %2`: in `cdef_filter_4x4`,
//!   which `cdef_filter_4x8` also `call`s, and in `cdef_filter_8x8`) are `cdef_filter_pri::<W>`,
//!   `cdef_filter_sec::<W>` and `cdef_filter_pri_sec::<W>`; the `%rep %1*%2/16` of `call`s is a loop
//!   with a constant count. The repeated `%if %1 == 4` pair of loads (`movu`+`punpcklqdq`, or
//!   `movu xm`+`vinserti128`) is `ld::<W>`, the repeated store at the end of each subroutine is
//!   `cdef_filter_store::<W>`.
//! - The vector registers are the register file `M` (`m[5]` is `m5`); the general-purpose registers are
//!   locals named as in `DEFINE_ARGS` (`dst`, `stride`, `dir`, `pridmp`, `pri`, `sec`, `tmp`, `off`,
//!   `stride3` for `r9`). The stack slots `pri_shift` and `sec_shift` (a 64-bit count that `psrlw` reads
//!   as its memory operand) are locals holding the count in the low 64 bits of an `xmm` value.
//! - The constants of `SECTION_RODATA` are statics of the same names (`dir_table4`, `dir_table8`,
//!   `pri_taps`, `dir_shift`); no read crosses a label. `pw_2048` and `pw_m16384` are broadcast constants.
//! - `cdef_dir_16bpc` ends in `jmp cdef_dir_8bpc_avx2.main`: here a call of the 8 bpc module's
//!   `cdef_dir_8bpc_main` with the register file.
//!
//! Deviations from the assembly:
//! - Every vector access to caller memory is an unaligned move where the assembly uses `mova` (`dst`
//!   rows of 8 pixels, the `src` rows of `cdef_dir_16bpc`): the same bytes and instruction count.
//! - `px` is a zero-initialized array: the padding writes every byte the filter uses on
//!   every edge combination, but the 4-pixel-wide loads (`movu m, [tmpq+offq+32*1]`, 32 bytes of which
//!   `punpcklqdq` uses the low 8 of each lane) reach 4 bytes past the last padded row, beyond the
//!   assembly's frame. The buffer has the assembly's frame size, starts 16 bytes later than the frame
//!   in the 4-pixel-wide kernels (so that those 4 bytes lie inside it), and costs 5, 7 and 13 32-byte
//!   stores (4x4, 4x8, 8x8).
//! - `lzcnt` and `tzcnt` (the strengths' logarithms) are `leading_zeros` and `trailing_zeros`, which
//!   give the same value for every input (32 for zero), and `rorx` (BMI2) is `rotate_right`: the
//!   original selects these kernels on AVX2 alone, so the port uses no BMI instruction; the compiler
//!   emits `bsr`/`bsf`/`ror` and a few more instructions.
//! - The shift counts stay in `xmm` registers where the assembly stores them to its frame and uses
//!   them as memory operands.
//! - `.pri`, `.sec` and `.pri_sec` are inlined into each entry point; the assembly `call`s one copy.
//! - Instruction scheduling and register allocation are the compiler's.
#![allow(clippy::all, non_snake_case, non_upper_case_globals)]

use core::arch::x86_64::*;
use core::ffi::{c_int, c_uint};

use super::cdef_avx2::{at, cdef_dir_8bpc_main, lo, regs, zx, M};

// ---- SECTION_RODATA ----

/// `DIR_TABLE 16`: byte offsets in `px` (rows 16 bytes apart) of the taps, two per direction, for the
/// directions 6, 7, 0 … 7, 0, 1 (the assembly's `db` lines).
static dir_table4: [i8; 24] = [
    16, 32, 16, 30, -14, -28, 2, -12, 2, 4, 2, 20, 18, 36, 16, 34, 16, 32, 16, 30, -14, -28, 2, -12,
];
/// `DIR_TABLE 32` (rows 32 bytes apart).
static dir_table8: [i8; 24] = [
    32, 64, 32, 62, -30, -60, 2, -28, 2, 4, 2, 36, 34, 68, 32, 66, 32, 64, 32, 62, -30, -60, 2, -28,
];
/// `pri_taps: dw 4, 4, 3, 3, 2, 2, 3, 3`.
static pri_taps: [u16; 8] = [4, 4, 3, 3, 2, 2, 3, 3];
/// `dir_shift: times 2 dw 0x4000; times 2 dw 0x1000`.
static dir_shift: [u16; 4] = [0x4000, 0x4000, 0x1000, 0x1000];
/// `pw_2048: times 2 dw 2048`.
const pw_2048: i16 = 2048;
/// `pw_m16384: times 2 dw -16384`.
const pw_m16384: i16 = -16384;

// ---- CDEF_FILTER ----

/// The `%if %1 == 4` pair of loads of `CDEF_FILTER`, rows at `p` and `q`: 4 pixels wide `movu m, [p]` +
/// `punpcklqdq m, [q]` (rows 0 2 | 1 3 of a group of four), 8 pixels wide `movu xm, [p]` +
/// `vinserti128 m, [q], 1`.
#[inline(always)]
unsafe fn ld<const W: usize>(p: *const u8, q: *const u8) -> __m256i {
    if W == 4 {
        let x = _mm256_loadu_si256(p.cast()); // movu m, [p]
        _mm256_unpacklo_epi64(x, _mm256_loadu_si256(q.cast())) // punpcklqdq m, [q]
    } else {
        let x = zx(_mm_loadu_si128(p.cast())); // movu xm, [p]
        _mm256_inserti128_si256::<1>(x, _mm_loadu_si128(q.cast())) // vinserti128 m, [q], 1
    }
}

/// The store at the end of `.pri`, `.sec` and `.pri_sec`: `m0` to four rows of 4 pixels or two rows of 8,
/// and `dstq` past them.
#[inline(always)]
unsafe fn cdef_filter_store<const W: usize>(m: &mut M, dst: &mut *mut u8, stride: isize, stride3: isize) {
    let d = *dst;
    if W == 4 {
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        _mm_storel_epi64(d.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
        _mm_storel_epi64(at(d, stride).cast(), lo(m[1])); // movq [dstq+strideq*1], xm1
        at(d, stride * 2)
            .cast::<i64>()
            .write_unaligned(_mm_extract_epi64::<1>(lo(m[0]))); // movhps [dstq+strideq*2], xm0
        at(d, stride3)
            .cast::<i64>()
            .write_unaligned(_mm_extract_epi64::<1>(lo(m[1]))); // movhps [dstq+r9], xm1
        *dst = at(d, stride * 4); // lea dstq, [dstq+strideq*4]
    } else {
        _mm_storeu_si128(d.cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
        _mm_storeu_si128(at(d, stride).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        *dst = at(d, stride * 2); // lea dstq, [dstq+strideq*2]
    }
}

/// `.pri` (primary taps only) of `cdef_filter_4x4_16bpc` (`W` = 4, also called by `cdef_filter_4x8`) and
/// `cdef_filter_8x8_16bpc` (`W` = 8): one group of rows from `px` at `tmp` to `dst`. In: `m6` the
/// strength, `m8` `pw_2048`, `m9`/`m10` the taps; `pri_shift` the frame slot.
#[inline(always)]
unsafe fn cdef_filter_pri<const W: usize>(
    m: &mut M,
    dir: *const i8,
    tmp: &mut *mut u8,
    pri_shift: __m128i,
    dst: &mut *mut u8,
    stride: isize,
    stride3: isize,
) {
    let t = *tmp;
    let sh = pri_shift;
    let mut off = *dir.add(4) as isize; // movsx offq, byte [dirq+4] ; off_k0
    m[1] = ld::<W>(t, at(t, 32)); // mova m1, [tmpq+32*0] ; punpcklqdq m1, [tmpq+32*1] (8: mova xm1 ; vinserti128)
    m[2] = ld::<W>(at(t, off), at(t, off + 32)); // movu m2, [tmpq+offq+32*0] ; punpcklqdq m2, [tmpq+offq+32*1] ; k0p0
    off = -off; // neg offq
    m[3] = ld::<W>(at(t, off), at(t, off + 32)); // movu m3, [tmpq+offq+32*0] ; punpcklqdq m3, [tmpq+offq+32*1] ; k0p1
    off = *dir.add(5) as isize; // movsx offq, byte [dirq+5] ; off_k1
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1 ; diff_k0p0
    m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1 ; diff_k0p1
    m[4] = _mm256_abs_epi16(m[2]); // pabsw m4, m2 ; adiff_k0p0
    m[5] = _mm256_srl_epi16(m[4], sh); // psrlw m5, m4, [pri_shift+gprsize]
    m[0] = _mm256_subs_epu16(m[6], m[5]); // psubusw m0, m6, m5
    m[5] = _mm256_abs_epi16(m[3]); // pabsw m5, m3 ; adiff_k0p1
    m[0] = _mm256_min_epi16(m[0], m[4]); // pminsw m0, m4
    m[4] = _mm256_srl_epi16(m[5], sh); // psrlw m4, m5, [pri_shift+gprsize]
    m[0] = _mm256_sign_epi16(m[0], m[2]); // psignw m0, m2 ; constrain(diff_k0p0)
    m[2] = _mm256_subs_epu16(m[6], m[4]); // psubusw m2, m6, m4
    m[2] = _mm256_min_epi16(m[2], m[5]); // pminsw m2, m5
    m[4] = ld::<W>(at(t, off), at(t, off + 32)); // movu m4, [tmpq+offq+32*0] ; punpcklqdq m4, [tmpq+offq+32*1] ; k1p0
    off = -off; // neg offq
    m[5] = ld::<W>(at(t, off), at(t, off + 32)); // movu m5, [tmpq+offq+32*0] ; punpcklqdq m5, [tmpq+offq+32*1] ; k1p1
    m[4] = _mm256_sub_epi16(m[4], m[1]); // psubw m4, m1 ; diff_k1p0
    m[5] = _mm256_sub_epi16(m[5], m[1]); // psubw m5, m1 ; diff_k1p1
    m[2] = _mm256_sign_epi16(m[2], m[3]); // psignw m2, m3 ; constrain(diff_k0p1)
    m[3] = _mm256_abs_epi16(m[4]); // pabsw m3, m4 ; adiff_k1p0
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2 ; constrain(diff_k0)
    m[2] = _mm256_srl_epi16(m[3], sh); // psrlw m2, m3, [pri_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[6], m[2]); // psubusw m7, m6, m2
    m[2] = _mm256_abs_epi16(m[5]); // pabsw m2, m5 ; adiff_k1p1
    m[7] = _mm256_min_epi16(m[7], m[3]); // pminsw m7, m3
    m[3] = _mm256_srl_epi16(m[2], sh); // psrlw m3, m2, [pri_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[4]); // psignw m7, m4 ; constrain(diff_k1p0)
    m[4] = _mm256_subs_epu16(m[6], m[3]); // psubusw m4, m6, m3
    m[4] = _mm256_min_epi16(m[4], m[2]); // pminsw m4, m2
    m[4] = _mm256_sign_epi16(m[4], m[5]); // psignw m4, m5 ; constrain(diff_k1p1)
    m[7] = _mm256_add_epi16(m[7], m[4]); // paddw m7, m4 ; constrain(diff_k1)
    m[0] = _mm256_mullo_epi16(m[0], m[9]); // pmullw m0, m9 ; pri_tap_k0
    m[7] = _mm256_mullo_epi16(m[7], m[10]); // pmullw m7, m10 ; pri_tap_k1
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7 ; sum
    m[2] = _mm256_srai_epi16::<15>(m[0]); // psraw m2, m0, 15
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[0] = _mm256_mulhrs_epi16(m[0], m[8]); // pmulhrsw m0, m8
    *tmp = at(t, 32 * 2); // add tmpq, 32*2
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    cdef_filter_store::<W>(m, dst, stride, stride3);
} // ret

/// `.sec` (secondary taps only) of `cdef_filter_4x4_16bpc` / `cdef_filter_8x8_16bpc`. In: `m6` the
/// strength, `m8` `pw_2048`; `sec_shift` the frame slot.
#[inline(always)]
unsafe fn cdef_filter_sec<const W: usize>(
    m: &mut M,
    dir: *const i8,
    tmp: &mut *mut u8,
    sec_shift: __m128i,
    dst: &mut *mut u8,
    stride: isize,
    stride3: isize,
) {
    let t = *tmp;
    let sh = sec_shift;
    let mut off = *dir.add(8) as isize; // movsx offq, byte [dirq+8] ; off1_k0
    m[1] = ld::<W>(t, at(t, 32)); // mova m1, [tmpq+32*0] ; punpcklqdq m1, [tmpq+32*1] (8: mova xm1 ; vinserti128)
    m[2] = ld::<W>(at(t, off), at(t, off + 32)); // movu m2, [tmpq+offq+32*0] ; punpcklqdq m2, [tmpq+offq+32*1] ; k0s0
    off = -off; // neg offq
    m[3] = ld::<W>(at(t, off), at(t, off + 32)); // movu m3, [tmpq+offq+32*0] ; punpcklqdq m3, [tmpq+offq+32*1] ; k0s1
    off = *dir.add(0) as isize; // movsx offq, byte [dirq+0] ; off2_k0
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1 ; diff_k0s0
    m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1 ; diff_k0s1
    m[4] = _mm256_abs_epi16(m[2]); // pabsw m4, m2 ; adiff_k0s0
    m[5] = _mm256_srl_epi16(m[4], sh); // psrlw m5, m4, [sec_shift+gprsize]
    m[0] = _mm256_subs_epu16(m[6], m[5]); // psubusw m0, m6, m5
    m[5] = _mm256_abs_epi16(m[3]); // pabsw m5, m3 ; adiff_k0s1
    m[0] = _mm256_min_epi16(m[0], m[4]); // pminsw m0, m4
    m[4] = _mm256_srl_epi16(m[5], sh); // psrlw m4, m5, [sec_shift+gprsize]
    m[0] = _mm256_sign_epi16(m[0], m[2]); // psignw m0, m2 ; constrain(diff_k0s0)
    m[2] = _mm256_subs_epu16(m[6], m[4]); // psubusw m2, m6, m4
    m[2] = _mm256_min_epi16(m[2], m[5]); // pminsw m2, m5
    m[4] = ld::<W>(at(t, off), at(t, off + 32)); // movu m4, [tmpq+offq+32*0] ; punpcklqdq m4, [tmpq+offq+32*1] ; k0s2
    off = -off; // neg offq
    m[5] = ld::<W>(at(t, off), at(t, off + 32)); // movu m5, [tmpq+offq+32*0] ; punpcklqdq m5, [tmpq+offq+32*1] ; k0s3
    off = *dir.add(9) as isize; // movsx offq, byte [dirq+9] ; off1_k1
    m[4] = _mm256_sub_epi16(m[4], m[1]); // psubw m4, m1 ; diff_k0s2
    m[5] = _mm256_sub_epi16(m[5], m[1]); // psubw m5, m1 ; diff_k0s3
    m[2] = _mm256_sign_epi16(m[2], m[3]); // psignw m2, m3 ; constrain(diff_k0s1)
    m[3] = _mm256_abs_epi16(m[4]); // pabsw m3, m4 ; adiff_k0s2
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[2] = _mm256_srl_epi16(m[3], sh); // psrlw m2, m3, [sec_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[6], m[2]); // psubusw m7, m6, m2
    m[2] = _mm256_abs_epi16(m[5]); // pabsw m2, m5 ; adiff_k0s3
    m[7] = _mm256_min_epi16(m[7], m[3]); // pminsw m7, m3
    m[3] = _mm256_srl_epi16(m[2], sh); // psrlw m3, m2, [sec_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[4]); // psignw m7, m4 ; constrain(diff_k0s2)
    m[4] = _mm256_subs_epu16(m[6], m[3]); // psubusw m4, m6, m3
    m[4] = _mm256_min_epi16(m[4], m[2]); // pminsw m4, m2
    m[2] = ld::<W>(at(t, off), at(t, off + 32)); // movu m2, [tmpq+offq+32*0] ; punpcklqdq m2, [tmpq+offq+32*1] ; k1s0
    off = -off; // neg offq
    m[3] = ld::<W>(at(t, off), at(t, off + 32)); // movu m3, [tmpq+offq+32*0] ; punpcklqdq m3, [tmpq+offq+32*1] ; k1s1
    off = *dir.add(1) as isize; // movsx offq, byte [dirq+1] ; off2_k1
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[4] = _mm256_sign_epi16(m[4], m[5]); // psignw m4, m5 ; constrain(diff_k0s3)
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4 ; constrain(diff_k0)
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1 ; diff_k1s0
    m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1 ; diff_k1s1
    m[0] = _mm256_add_epi16(m[0], m[0]); // paddw m0, m0 ; sec_tap_k0
    m[4] = _mm256_abs_epi16(m[2]); // pabsw m4, m2 ; adiff_k1s0
    m[5] = _mm256_srl_epi16(m[4], sh); // psrlw m5, m4, [sec_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[6], m[5]); // psubusw m7, m6, m5
    m[5] = _mm256_abs_epi16(m[3]); // pabsw m5, m3 ; adiff_k1s1
    m[7] = _mm256_min_epi16(m[7], m[4]); // pminsw m7, m4
    m[4] = _mm256_srl_epi16(m[5], sh); // psrlw m4, m5, [sec_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[2]); // psignw m7, m2 ; constrain(diff_k1s0)
    m[2] = _mm256_subs_epu16(m[6], m[4]); // psubusw m2, m6, m4
    m[2] = _mm256_min_epi16(m[2], m[5]); // pminsw m2, m5
    m[4] = ld::<W>(at(t, off), at(t, off + 32)); // movu m4, [tmpq+offq+32*0] ; punpcklqdq m4, [tmpq+offq+32*1] ; k1s2
    off = -off; // neg offq
    m[5] = ld::<W>(at(t, off), at(t, off + 32)); // movu m5, [tmpq+offq+32*0] ; punpcklqdq m5, [tmpq+offq+32*1] ; k1s3
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[4] = _mm256_sub_epi16(m[4], m[1]); // psubw m4, m1 ; diff_k1s2
    m[5] = _mm256_sub_epi16(m[5], m[1]); // psubw m5, m1 ; diff_k1s3
    m[2] = _mm256_sign_epi16(m[2], m[3]); // psignw m2, m3 ; constrain(diff_k1s1)
    m[3] = _mm256_abs_epi16(m[4]); // pabsw m3, m4 ; adiff_k1s2
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[2] = _mm256_srl_epi16(m[3], sh); // psrlw m2, m3, [sec_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[6], m[2]); // psubusw m7, m6, m2
    m[2] = _mm256_abs_epi16(m[5]); // pabsw m2, m5 ; adiff_k1s3
    m[7] = _mm256_min_epi16(m[7], m[3]); // pminsw m7, m3
    m[3] = _mm256_srl_epi16(m[2], sh); // psrlw m3, m2, [sec_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[4]); // psignw m7, m4 ; constrain(diff_k1s2)
    m[4] = _mm256_subs_epu16(m[6], m[3]); // psubusw m4, m6, m3
    m[4] = _mm256_min_epi16(m[4], m[2]); // pminsw m4, m2
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[4] = _mm256_sign_epi16(m[4], m[5]); // psignw m4, m5 ; constrain(diff_k1s3)
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4 ; sum
    m[2] = _mm256_srai_epi16::<15>(m[0]); // psraw m2, m0, 15
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[0] = _mm256_mulhrs_epi16(m[0], m[8]); // pmulhrsw m0, m8
    *tmp = at(t, 32 * 2); // add tmpq, 32*2
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    cdef_filter_store::<W>(m, dst, stride, stride3);
} // ret

/// `.pri_sec` (both) of `cdef_filter_4x4_16bpc` / `cdef_filter_8x8_16bpc`. In: `m6` the primary and `m13`
/// the secondary strength, `m8` `pw_2048`, `m9`/`m10` the primary taps; the two frame slots. `m11`/`m12`
/// collect the maximum and minimum of the taps for the clamp.
#[inline(always)]
unsafe fn cdef_filter_pri_sec<const W: usize>(
    m: &mut M,
    dir: *const i8,
    tmp: &mut *mut u8,
    dst: &mut *mut u8,
    stride: isize,
    stride3: isize,
    pri_shift: __m128i,
    sec_shift: __m128i,
) {
    let t = *tmp;
    let (psh, ssh) = (pri_shift, sec_shift);
    let mut off = *dir.add(8) as isize; // movsx offq, byte [dirq+8] ; off2_k0
    if W == 4 {
        m[1] = ld::<W>(t, at(t, 32)); // mova m1, [tmpq+32*0] ; punpcklqdq m1, [tmpq+32*1]
    } else {
        m[1] = ld::<W>(*dst, at(*dst, stride)); // mova xm1, [dstq+strideq*0] ; vinserti128 m1, [dstq+strideq*1], 1
    }
    m[2] = ld::<W>(at(t, off), at(t, off + 32)); // movu m2, [tmpq+offq+32*0] ; punpcklqdq m2, [tmpq+offq+32*1] ; k0s0
    off = -off; // neg offq
    m[3] = ld::<W>(at(t, off), at(t, off + 32)); // movu m3, [tmpq+offq+32*0] ; punpcklqdq m3, [tmpq+offq+32*1] ; k0s1
    off = *dir.add(0) as isize; // movsx offq, byte [dirq+0] ; off3_k0
    m[11] = _mm256_max_epi16(m[2], m[3]); // pmaxsw m11, m2, m3
    m[12] = _mm256_min_epu16(m[2], m[3]); // pminuw m12, m2, m3
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1 ; diff_k0s0
    m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1 ; diff_k0s1
    m[4] = _mm256_abs_epi16(m[2]); // pabsw m4, m2 ; adiff_k0s0
    m[5] = _mm256_srl_epi16(m[4], ssh); // psrlw m5, m4, [sec_shift+gprsize]
    m[0] = _mm256_subs_epu16(m[13], m[5]); // psubusw m0, m13, m5
    m[5] = _mm256_abs_epi16(m[3]); // pabsw m5, m3 ; adiff_k0s1
    m[0] = _mm256_min_epi16(m[0], m[4]); // pminsw m0, m4
    m[4] = _mm256_srl_epi16(m[5], ssh); // psrlw m4, m5, [sec_shift+gprsize]
    m[0] = _mm256_sign_epi16(m[0], m[2]); // psignw m0, m2 ; constrain(diff_k0s0)
    m[2] = _mm256_subs_epu16(m[13], m[4]); // psubusw m2, m13, m4
    m[2] = _mm256_min_epi16(m[2], m[5]); // pminsw m2, m5
    m[4] = ld::<W>(at(t, off), at(t, off + 32)); // movu m4, [tmpq+offq+32*0] ; punpcklqdq m4, [tmpq+offq+32*1] ; k0s2
    off = -off; // neg offq
    m[5] = ld::<W>(at(t, off), at(t, off + 32)); // movu m5, [tmpq+offq+32*0] ; punpcklqdq m5, [tmpq+offq+32*1] ; k0s3
    off = *dir.add(9) as isize; // movsx offq, byte [dirq+9] ; off2_k1
    m[2] = _mm256_sign_epi16(m[2], m[3]); // psignw m2, m3 ; constrain(diff_k0s1)
    m[11] = _mm256_max_epi16(m[11], m[4]); // pmaxsw m11, m4
    m[12] = _mm256_min_epu16(m[12], m[4]); // pminuw m12, m4
    m[11] = _mm256_max_epi16(m[11], m[5]); // pmaxsw m11, m5
    m[12] = _mm256_min_epu16(m[12], m[5]); // pminuw m12, m5
    m[4] = _mm256_sub_epi16(m[4], m[1]); // psubw m4, m1 ; diff_k0s2
    m[5] = _mm256_sub_epi16(m[5], m[1]); // psubw m5, m1 ; diff_k0s3
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[3] = _mm256_abs_epi16(m[4]); // pabsw m3, m4 ; adiff_k0s2
    m[2] = _mm256_srl_epi16(m[3], ssh); // psrlw m2, m3, [sec_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[13], m[2]); // psubusw m7, m13, m2
    m[2] = _mm256_abs_epi16(m[5]); // pabsw m2, m5 ; adiff_k0s3
    m[7] = _mm256_min_epi16(m[7], m[3]); // pminsw m7, m3
    m[3] = _mm256_srl_epi16(m[2], ssh); // psrlw m3, m2, [sec_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[4]); // psignw m7, m4 ; constrain(diff_k0s2)
    m[4] = _mm256_subs_epu16(m[13], m[3]); // psubusw m4, m13, m3
    m[4] = _mm256_min_epi16(m[4], m[2]); // pminsw m4, m2
    m[2] = ld::<W>(at(t, off), at(t, off + 32)); // movu m2, [tmpq+offq+32*0] ; punpcklqdq m2, [tmpq+offq+32*1] ; k1s0
    off = -off; // neg offq
    m[3] = ld::<W>(at(t, off), at(t, off + 32)); // movu m3, [tmpq+offq+32*0] ; punpcklqdq m3, [tmpq+offq+32*1] ; k1s1
    off = *dir.add(1) as isize; // movsx offq, byte [dirq+1] ; off3_k1
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[4] = _mm256_sign_epi16(m[4], m[5]); // psignw m4, m5 ; constrain(diff_k0s3)
    m[11] = _mm256_max_epi16(m[11], m[2]); // pmaxsw m11, m2
    m[12] = _mm256_min_epu16(m[12], m[2]); // pminuw m12, m2
    m[11] = _mm256_max_epi16(m[11], m[3]); // pmaxsw m11, m3
    m[12] = _mm256_min_epu16(m[12], m[3]); // pminuw m12, m3
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4 ; constrain(diff_k0)
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1 ; diff_k1s0
    m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1 ; diff_k1s1
    m[0] = _mm256_add_epi16(m[0], m[0]); // paddw m0, m0 ; sec_tap_k0
    m[4] = _mm256_abs_epi16(m[2]); // pabsw m4, m2 ; adiff_k1s0
    m[5] = _mm256_srl_epi16(m[4], ssh); // psrlw m5, m4, [sec_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[13], m[5]); // psubusw m7, m13, m5
    m[5] = _mm256_abs_epi16(m[3]); // pabsw m5, m3 ; adiff_k1s1
    m[7] = _mm256_min_epi16(m[7], m[4]); // pminsw m7, m4
    m[4] = _mm256_srl_epi16(m[5], ssh); // psrlw m4, m5, [sec_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[2]); // psignw m7, m2 ; constrain(diff_k1s0)
    m[2] = _mm256_subs_epu16(m[13], m[4]); // psubusw m2, m13, m4
    m[2] = _mm256_min_epi16(m[2], m[5]); // pminsw m2, m5
    m[4] = ld::<W>(at(t, off), at(t, off + 32)); // movu m4, [tmpq+offq+32*0] ; punpcklqdq m4, [tmpq+offq+32*1] ; k1s2
    off = -off; // neg offq
    m[5] = ld::<W>(at(t, off), at(t, off + 32)); // movu m5, [tmpq+offq+32*0] ; punpcklqdq m5, [tmpq+offq+32*1] ; k1s3
    off = *dir.add(4) as isize; // movsx offq, byte [dirq+4] ; off1_k0
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[2] = _mm256_sign_epi16(m[2], m[3]); // psignw m2, m3 ; constrain(diff_k1s1)
    m[11] = _mm256_max_epi16(m[11], m[4]); // pmaxsw m11, m4
    m[12] = _mm256_min_epu16(m[12], m[4]); // pminuw m12, m4
    m[11] = _mm256_max_epi16(m[11], m[5]); // pmaxsw m11, m5
    m[12] = _mm256_min_epu16(m[12], m[5]); // pminuw m12, m5
    m[4] = _mm256_sub_epi16(m[4], m[1]); // psubw m4, m1 ; diff_k1s2
    m[5] = _mm256_sub_epi16(m[5], m[1]); // psubw m5, m1 ; diff_k1s3
    m[3] = _mm256_abs_epi16(m[4]); // pabsw m3, m4 ; adiff_k1s2
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[2] = _mm256_srl_epi16(m[3], ssh); // psrlw m2, m3, [sec_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[13], m[2]); // psubusw m7, m13, m2
    m[2] = _mm256_abs_epi16(m[5]); // pabsw m2, m5 ; adiff_k1s3
    m[7] = _mm256_min_epi16(m[7], m[3]); // pminsw m7, m3
    m[3] = _mm256_srl_epi16(m[2], ssh); // psrlw m3, m2, [sec_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[4]); // psignw m7, m4 ; constrain(diff_k1s2)
    m[4] = _mm256_subs_epu16(m[13], m[3]); // psubusw m4, m13, m3
    m[4] = _mm256_min_epi16(m[4], m[2]); // pminsw m4, m2
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[2] = ld::<W>(at(t, off), at(t, off + 32)); // movu m2, [tmpq+offq+32*0] ; punpcklqdq m2, [tmpq+offq+32*1] ; k0p0
    off = -off; // neg offq
    m[3] = ld::<W>(at(t, off), at(t, off + 32)); // movu m3, [tmpq+offq+32*0] ; punpcklqdq m3, [tmpq+offq+32*1] ; k0p1
    off = *dir.add(5) as isize; // movsx offq, byte [dirq+5] ; off1_k1
    m[4] = _mm256_sign_epi16(m[4], m[5]); // psignw m4, m5 ; constrain(diff_k1s3)
    m[11] = _mm256_max_epi16(m[11], m[2]); // pmaxsw m11, m2
    m[12] = _mm256_min_epu16(m[12], m[2]); // pminuw m12, m2
    m[11] = _mm256_max_epi16(m[11], m[3]); // pmaxsw m11, m3
    m[12] = _mm256_min_epu16(m[12], m[3]); // pminuw m12, m3
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1 ; diff_k0p0
    m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1 ; diff_k0p1
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4
    m[4] = _mm256_abs_epi16(m[2]); // pabsw m4, m2 ; adiff_k0p0
    m[5] = _mm256_srl_epi16(m[4], psh); // psrlw m5, m4, [pri_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[6], m[5]); // psubusw m7, m6, m5
    m[5] = _mm256_abs_epi16(m[3]); // pabsw m5, m3 ; adiff_k0p1
    m[7] = _mm256_min_epi16(m[7], m[4]); // pminsw m7, m4
    m[4] = _mm256_srl_epi16(m[5], psh); // psrlw m4, m5, [pri_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[2]); // psignw m7, m2 ; constrain(diff_k0p0)
    m[2] = _mm256_subs_epu16(m[6], m[4]); // psubusw m2, m6, m4
    m[2] = _mm256_min_epi16(m[2], m[5]); // pminsw m2, m5
    m[4] = ld::<W>(at(t, off), at(t, off + 32)); // movu m4, [tmpq+offq+32*0] ; punpcklqdq m4, [tmpq+offq+32*1] ; k1p0
    off = -off; // neg offq
    m[5] = ld::<W>(at(t, off), at(t, off + 32)); // movu m5, [tmpq+offq+32*0] ; punpcklqdq m5, [tmpq+offq+32*1] ; k1p1
    m[2] = _mm256_sign_epi16(m[2], m[3]); // psignw m2, m3 ; constrain(diff_k0p1)
    m[7] = _mm256_add_epi16(m[7], m[2]); // paddw m7, m2 ; constrain(diff_k0)
    m[11] = _mm256_max_epi16(m[11], m[4]); // pmaxsw m11, m4
    m[12] = _mm256_min_epu16(m[12], m[4]); // pminuw m12, m4
    m[11] = _mm256_max_epi16(m[11], m[5]); // pmaxsw m11, m5
    m[12] = _mm256_min_epu16(m[12], m[5]); // pminuw m12, m5
    m[4] = _mm256_sub_epi16(m[4], m[1]); // psubw m4, m1 ; diff_k1p0
    m[5] = _mm256_sub_epi16(m[5], m[1]); // psubw m5, m1 ; diff_k1p1
    m[3] = _mm256_abs_epi16(m[4]); // pabsw m3, m4 ; adiff_k1p0
    m[7] = _mm256_mullo_epi16(m[7], m[9]); // pmullw m7, m9 ; pri_tap_k0
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
    m[2] = _mm256_srl_epi16(m[3], psh); // psrlw m2, m3, [pri_shift+gprsize]
    m[7] = _mm256_subs_epu16(m[6], m[2]); // psubusw m7, m6, m2
    m[2] = _mm256_abs_epi16(m[5]); // pabsw m2, m5 ; adiff_k1p1
    m[7] = _mm256_min_epi16(m[7], m[3]); // pminsw m7, m3
    m[3] = _mm256_srl_epi16(m[2], psh); // psrlw m3, m2, [pri_shift+gprsize]
    m[7] = _mm256_sign_epi16(m[7], m[4]); // psignw m7, m4 ; constrain(diff_k1p0)
    m[4] = _mm256_subs_epu16(m[6], m[3]); // psubusw m4, m6, m3
    m[4] = _mm256_min_epi16(m[4], m[2]); // pminsw m4, m2
    m[4] = _mm256_sign_epi16(m[4], m[5]); // psignw m4, m5 ; constrain(diff_k1p1)
    m[7] = _mm256_add_epi16(m[7], m[4]); // paddw m7, m4 ; constrain(diff_k1)
    m[7] = _mm256_mullo_epi16(m[7], m[10]); // pmullw m7, m10 ; pri_tap_k1
    m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7 ; sum
    m[2] = _mm256_srai_epi16::<15>(m[0]); // psraw m2, m0, 15
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[0] = _mm256_mulhrs_epi16(m[0], m[8]); // pmulhrsw m0, m8
    *tmp = at(t, 32 * 2); // add tmpq, 32*2
    m[11] = _mm256_max_epi16(m[11], m[1]); // pmaxsw m11, m1
    m[12] = _mm256_min_epu16(m[12], m[1]); // pminuw m12, m1
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[0] = _mm256_min_epi16(m[0], m[11]); // pminsw m0, m11
    m[0] = _mm256_max_epi16(m[0], m[12]); // pmaxsw m0, m12
    cdef_filter_store::<W>(m, dst, stride, stride3);
} // ret

/// `%macro CDEF_FILTER w, h` from its start to `.end`'s `RET` (the subroutines are the functions above).
/// `px` is the padded block of the entry point, `stride3` its `r9` (4 pixels wide only).
#[inline(always)]
unsafe fn cdef_filter<const W: usize, const H: usize>(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    stride3: isize,
    px: *mut u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    bitdepth_max: c_int,
) {
    // DEFINE_ARGS dst, stride, _, dir, pridmp, pri, sec, tmp
    let mut pri = pri as u32; // movifnidn prid, r5m
    let mut sec = sec as u32; // movifnidn secd, r6m
    let dir = dir as u32 as usize; // mov dird, r7m
    m[8] = _mm256_set1_epi16(pw_2048); // vpbroadcastd m8, [base+pw_2048]
                                       // `dir` is 0..7 (the contract of `cdef_fn`), so `[dirq+0]` … `[dirq+9]` lie inside the 24-byte table.
    let dirp = (if W == 4 {
        dir_table4.as_ptr()
    } else {
        dir_table8.as_ptr()
    })
    .add(dir * 2); // lea dirq, [base+dir_table%1+dirq*2]
    let mut tmp: *mut u8;
    if pri != 0 {
        // test prid, prid ; jz .sec_only
        m[6] = zx(_mm_cvtsi32_si128(pri as i32)); // movd xm6, prid
        m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
        let mut pridmp = pri.leading_zeros(); // lzcnt pridmpd, prid (the same value for every input)
        let mut t = pri.rotate_right(2); // rorx tmpd, prid, 2
        if bitdepth_max == 0xfff {
            // cmp dword r10m, 0xfff ; if (bpc == 12)
            pri = t; // cmove prid, tmpd ; pri >>= 2
        }
        t = damping as u32; // mov tmpd, r8m ; damping
        pri &= 4; // and prid, 4
        t = t.wrapping_sub(31); // sub tmpd, 31
                                // `pri` is 0 or 4: both reads lie inside the 16-byte `pri_taps`.
        let taps = pri_taps.as_ptr().cast::<u8>().add(pri as usize);
        m[9] = _mm256_set1_epi32(taps.cast::<i32>().read_unaligned()); // vpbroadcastd m9, [base+pri_taps+priq+8*0]
        m[10] = _mm256_set1_epi32(taps.add(8).cast::<i32>().read_unaligned()); // vpbroadcastd m10, [base+pri_taps+priq+8*1]
        if sec != 0 {
            // test secd, secd ; jz .pri_only
            m[0] = zx(_mm_cvtsi32_si128(sec as i32)); // movd xm0, secd
            m[13] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m13, xm0
            sec = sec.leading_zeros(); // lzcnt secd, secd (the same value for every input)
            pri = 0; // xor prid, prid
            pridmp = pridmp.wrapping_add(t); // add pridmpd, tmpd
            if (pridmp as i32) < 0 {
                pridmp = pri; // cmovs pridmpd, prid
            }
            sec = sec.wrapping_add(t); // add secd, tmpd
            tmp = px; // lea tmpq, [px]
            let pri_shift = _mm_cvtsi64_si128(pridmp as i64); // mov [pri_shift], pridmpq
            let sec_shift = _mm_cvtsi64_si128(sec as i64); // mov [sec_shift], secq
            for _ in 0..W * H / 16 {
                // %rep %1*%2/16: call .pri_sec
                cdef_filter_pri_sec::<W>(m, dirp, &mut tmp, &mut dst, stride, stride3, pri_shift, sec_shift);
            }
            // jmp .pri_end
        } else {
            // .pri_only:
            pridmp = pridmp.wrapping_add(t); // add pridmpd, tmpd
            if (pridmp as i32) < 0 {
                pridmp = sec; // cmovs pridmpd, secd
            }
            tmp = px; // lea tmpq, [px]
            let pri_shift = _mm_cvtsi64_si128(pridmp as i64); // mov [pri_shift], pridmpq
            for _ in 0..W * H / 16 {
                // %rep %1*%2/16: call .pri
                cdef_filter_pri::<W>(m, dirp, &mut tmp, pri_shift, &mut dst, stride, stride3);
            }
        }
        // .pri_end: .end: RET
    } else {
        // .sec_only:
        let mut t = damping as u32; // mov tmpd, r8m ; damping
        m[6] = zx(_mm_cvtsi32_si128(sec as i32)); // movd xm6, secd
        m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
        sec = sec.trailing_zeros(); // tzcnt secd, secd (the same value for every input)
        t = t.wrapping_sub(sec); // sub tmpd, secd
        let sec_shift = _mm_cvtsi64_si128(t as i64); // mov [sec_shift], tmpq
        tmp = px; // lea tmpq, [px]
        for _ in 0..W * H / 16 {
            // %rep %1*%2/16: call .sec
            cdef_filter_sec::<W>(m, dirp, &mut tmp, sec_shift, &mut dst, stride, stride3);
        }
        // jmp .end
    }
}

// ---- the entry points: padding ----

/// `movd xmN, [p]` (4 bytes, unaligned).
#[inline(always)]
unsafe fn movd_ld(p: *const u8) -> __m256i {
    zx(_mm_cvtsi32_si128(p.cast::<i32>().read_unaligned()))
}

/// `movd [p], xmN` (4 bytes, unaligned).
#[inline(always)]
unsafe fn movd_st(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(lo(x)))
}

/// `cglobal cdef_filter_4x4_16bpc` up to `.padding_done`, then `CDEF_FILTER 4, 4`. `px` rows are 16 bytes
/// apart: columns 0..5 at `px+16*y+0`, columns -2 and -1 at `px+16*y-4` (the last 4 bytes of the row
/// above), rows -2..5.
#[inline(always)]
unsafe fn cdef_filter_4x4_16bpc(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
    bitdepth_max: c_int,
) {
    let mut m = regs();
    // The frame, `16*10` bytes as in the assembly, but starting at `px-48` (the assembly: `px-64`), so
    // that the filter's 32-byte loads, which reach `px+100`, stay inside it; zeroed whole (the bytes
    // `px+96..px+100` are read and never written; everything else the filter reads is written below).
    let mut frame = [_mm_setzero_si128(); 10];
    let px = at(frame.as_mut_ptr().cast::<u8>(), 48);
    let edge = edges; // mov edged, r9m
                      // lea r8, [dir_table4]
    m[0] = zx(_mm_loadu_si128(dst.cast())); // movu xm0, [dstq+strideq*0]
    m[1] = zx(_mm_loadu_si128(at(dst, stride).cast())); // movu xm1, [dstq+strideq*1]
    let stride3 = stride * 3; // lea r9, [strideq*3]
    m[2] = zx(_mm_loadu_si128(at(dst, stride * 2).cast())); // movu xm2, [dstq+strideq*2]
    m[3] = zx(_mm_loadu_si128(at(dst, stride3).cast())); // movu xm3, [dstq+r9]
    m[7] = _mm256_set1_epi16(pw_m16384); // vpbroadcastd m7, [base+pw_m16384]
    _mm_storeu_si128(at(px, 16 * 0).cast(), lo(m[0])); // mova [px+16*0+0], xm0
    _mm_storeu_si128(at(px, 16 * 1).cast(), lo(m[1])); // mova [px+16*1+0], xm1
    _mm_storeu_si128(at(px, 16 * 2).cast(), lo(m[2])); // mova [px+16*2+0], xm2
    _mm_storeu_si128(at(px, 16 * 3).cast(), lo(m[3])); // mova [px+16*3+0], xm3
    'top: {
        if edge & 4 != 0 {
            // test edgeb, 4 ; HAVE_TOP ; jz .no_top
            m[0] = zx(_mm_loadu_si128(top.cast())); // movu xm0, [topq+strideq*0]
            m[1] = zx(_mm_loadu_si128(top.offset(stride).cast())); // movu xm1, [topq+strideq*1]
            _mm_storeu_si128(at(px, -16 * 2).cast(), lo(m[0])); // mova [px-16*2+0], xm0
            _mm_storeu_si128(at(px, -16 * 1).cast(), lo(m[1])); // mova [px-16*1+0], xm1
            if edge & 1 != 0 {
                // test edgeb, 1 ; HAVE_LEFT ; jz .top_no_left
                m[0] = movd_ld(top.offset(-4)); // movd xm0, [topq+strideq*0-4]
                m[1] = movd_ld(top.offset(stride - 4)); // movd xm1, [topq+strideq*1-4]
                movd_st(at(px, -16 * 2 - 4), m[0]); // movd [px-16*2-4], xm0
                movd_st(at(px, -16 * 1 - 4), m[1]); // movd [px-16*1-4], xm1
                break 'top; // jmp .top_done
            }
        } else {
            // .no_top:
            _mm256_storeu_si256(at(px, -16 * 2).cast(), m[7]); // mova [px-16*2+0], m7
        }
        // .top_no_left:
        movd_st(at(px, -16 * 2 - 4), m[7]); // movd [px-16*2-4], xm7
        movd_st(at(px, -16 * 1 - 4), m[7]); // movd [px-16*1-4], xm7
    }
    // .top_done:
    'bottom: {
        if edge & 8 != 0 {
            // test edgeb, 8 ; HAVE_BOTTOM ; jz .no_bottom
            m[0] = zx(_mm_loadu_si128(bot.cast())); // movu xm0, [botq+strideq*0]
            m[1] = zx(_mm_loadu_si128(bot.offset(stride).cast())); // movu xm1, [botq+strideq*1]
            _mm_storeu_si128(at(px, 16 * 4).cast(), lo(m[0])); // mova [px+16*4+0], xm0
            _mm_storeu_si128(at(px, 16 * 5).cast(), lo(m[1])); // mova [px+16*5+0], xm1
            if edge & 1 != 0 {
                // test edgeb, 1 ; HAVE_LEFT ; jz .bottom_no_left
                m[0] = movd_ld(bot.offset(-4)); // movd xm0, [botq+strideq*0-4]
                m[1] = movd_ld(bot.offset(stride - 4)); // movd xm1, [botq+strideq*1-4]
                movd_st(at(px, 16 * 4 - 4), m[0]); // movd [px+16*4-4], xm0
                movd_st(at(px, 16 * 5 - 4), m[1]); // movd [px+16*5-4], xm1
                break 'bottom; // jmp .bottom_done
            }
        } else {
            // .no_bottom:
            _mm256_storeu_si256(at(px, 16 * 4).cast(), m[7]); // mova [px+16*4+0], m7
        }
        // .bottom_no_left:
        movd_st(at(px, 16 * 4 - 4), m[7]); // movd [px+16*4-4], xm7
        movd_st(at(px, 16 * 5 - 4), m[7]); // movd [px+16*5-4], xm7
    }
    // .bottom_done:
    if edge & 1 != 0 {
        // test edgeb, 1 ; HAVE_LEFT ; jz .no_left
        m[0] = movd_ld(left.offset(4 * 0)); // movd xm0, [leftq+4*0]
        m[1] = movd_ld(left.offset(4 * 1)); // movd xm1, [leftq+4*1]
        m[2] = movd_ld(left.offset(4 * 2)); // movd xm2, [leftq+4*2]
        m[3] = movd_ld(left.offset(4 * 3)); // movd xm3, [leftq+4*3]
        movd_st(at(px, 16 * 0 - 4), m[0]); // movd [px+16*0-4], xm0
        movd_st(at(px, 16 * 1 - 4), m[1]); // movd [px+16*1-4], xm1
        movd_st(at(px, 16 * 2 - 4), m[2]); // movd [px+16*2-4], xm2
        movd_st(at(px, 16 * 3 - 4), m[3]); // movd [px+16*3-4], xm3
    } else {
        // .no_left:
        for x in 0..4 {
            movd_st(at(px, 16 * x - 4), m[7]); // REPX {movd [px+16*x-4], xm7}, 0, 1, 2, 3
        }
    }
    // .left_done:
    if edge & 2 == 0 {
        // test edgeb, 2 ; HAVE_RIGHT ; jnz .padding_done
        for x in -2..6 {
            movd_st(at(px, 16 * x + 8), m[7]); // REPX {movd [px+16*x+8], xm7}, -2, -1, 0, 1, 2, 3, 4, 5
        }
    }
    // .padding_done:
    cdef_filter::<4, 4>(&mut m, dst, stride, stride3, px, pri, sec, dir, damping, bitdepth_max);
}

/// `cglobal cdef_filter_4x8_16bpc` up to `.padding_done`, then `CDEF_FILTER 4, 8` (whose subroutines are
/// those of `cdef_filter_4x4_16bpc`). `px` as in 4x4, rows -2..9.
#[inline(always)]
unsafe fn cdef_filter_4x8_16bpc(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
    bitdepth_max: c_int,
) {
    let mut m = regs();
    // The frame, `16*14` bytes as in the assembly, but starting at `px-48` (the assembly: `px-64`), so
    // that the filter's 32-byte loads, which reach `px+164`, stay inside it; zeroed whole (the bytes
    // `px+160..px+164` are read and never written; everything else the filter reads is written below).
    let mut frame = [_mm_setzero_si128(); 14];
    let px = at(frame.as_mut_ptr().cast::<u8>(), 48);
    let edge = edges; // mov edged, r9m
    m[0] = zx(_mm_loadu_si128(dst.cast())); // movu xm0, [dstq+strideq*0]
    m[1] = zx(_mm_loadu_si128(at(dst, stride).cast())); // movu xm1, [dstq+strideq*1]
    let stride3 = stride * 3; // lea r9, [strideq*3]
    m[2] = zx(_mm_loadu_si128(at(dst, stride * 2).cast())); // movu xm2, [dstq+strideq*2]
    m[3] = zx(_mm_loadu_si128(at(dst, stride3).cast())); // movu xm3, [dstq+r9]
    let r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    m[4] = zx(_mm_loadu_si128(r6.cast())); // movu xm4, [r6+strideq*0]
    m[5] = zx(_mm_loadu_si128(at(r6, stride).cast())); // movu xm5, [r6+strideq*1]
    m[6] = zx(_mm_loadu_si128(at(r6, stride * 2).cast())); // movu xm6, [r6+strideq*2]
    m[7] = zx(_mm_loadu_si128(at(r6, stride3).cast())); // movu xm7, [r6+r9]
                                                        // lea r8, [dir_table4]
    for y in 0..8 {
        _mm_storeu_si128(at(px, 16 * y as isize).cast(), lo(m[y])); // mova [px+16*y+0], xmy (y = 0..7)
    }
    m[7] = _mm256_set1_epi16(pw_m16384); // vpbroadcastd m7, [base+pw_m16384]
    'top: {
        if edge & 4 != 0 {
            // test edgeb, 4 ; HAVE_TOP ; jz .no_top
            m[0] = zx(_mm_loadu_si128(top.cast())); // movu xm0, [topq+strideq*0]
            m[1] = zx(_mm_loadu_si128(top.offset(stride).cast())); // movu xm1, [topq+strideq*1]
            _mm_storeu_si128(at(px, -16 * 2).cast(), lo(m[0])); // mova [px-16*2+0], xm0
            _mm_storeu_si128(at(px, -16 * 1).cast(), lo(m[1])); // mova [px-16*1+0], xm1
            if edge & 1 != 0 {
                // test edgeb, 1 ; HAVE_LEFT ; jz .top_no_left
                m[0] = movd_ld(top.offset(-4)); // movd xm0, [topq+strideq*0-4]
                m[1] = movd_ld(top.offset(stride - 4)); // movd xm1, [topq+strideq*1-4]
                movd_st(at(px, -16 * 2 - 4), m[0]); // movd [px-16*2-4], xm0
                movd_st(at(px, -16 * 1 - 4), m[1]); // movd [px-16*1-4], xm1
                break 'top; // jmp .top_done
            }
        } else {
            // .no_top:
            _mm256_storeu_si256(at(px, -16 * 2).cast(), m[7]); // mova [px-16*2+0], m7
        }
        // .top_no_left:
        movd_st(at(px, -16 * 2 - 4), m[7]); // movd [px-16*2-4], xm7
        movd_st(at(px, -16 * 1 - 4), m[7]); // movd [px-16*1-4], xm7
    }
    // .top_done:
    'bottom: {
        if edge & 8 != 0 {
            // test edgeb, 8 ; HAVE_BOTTOM ; jz .no_bottom
            m[0] = zx(_mm_loadu_si128(bot.cast())); // movu xm0, [botq+strideq*0]
            m[1] = zx(_mm_loadu_si128(bot.offset(stride).cast())); // movu xm1, [botq+strideq*1]
            _mm_storeu_si128(at(px, 16 * 8).cast(), lo(m[0])); // mova [px+16*8+0], xm0
            _mm_storeu_si128(at(px, 16 * 9).cast(), lo(m[1])); // mova [px+16*9+0], xm1
            if edge & 1 != 0 {
                // test edgeb, 1 ; HAVE_LEFT ; jz .bottom_no_left
                m[0] = movd_ld(bot.offset(-4)); // movd xm0, [botq+strideq*0-4]
                m[1] = movd_ld(bot.offset(stride - 4)); // movd xm1, [botq+strideq*1-4]
                movd_st(at(px, 16 * 8 - 4), m[0]); // movd [px+16*8-4], xm0
                movd_st(at(px, 16 * 9 - 4), m[1]); // movd [px+16*9-4], xm1
                break 'bottom; // jmp .bottom_done
            }
        } else {
            // .no_bottom:
            _mm256_storeu_si256(at(px, 16 * 8).cast(), m[7]); // mova [px+16*8+0], m7
        }
        // .bottom_no_left:
        movd_st(at(px, 16 * 8 - 4), m[7]); // movd [px+16*8-4], xm7
        movd_st(at(px, 16 * 9 - 4), m[7]); // movd [px+16*9-4], xm7
    }
    // .bottom_done:
    if edge & 1 != 0 {
        // test edgeb, 1 ; HAVE_LEFT ; jz .no_left
        m[0] = movd_ld(left.offset(4 * 0)); // movd xm0, [leftq+4*0]
        m[1] = movd_ld(left.offset(4 * 1)); // movd xm1, [leftq+4*1]
        m[2] = movd_ld(left.offset(4 * 2)); // movd xm2, [leftq+4*2]
        m[3] = movd_ld(left.offset(4 * 3)); // movd xm3, [leftq+4*3]
        movd_st(at(px, 16 * 0 - 4), m[0]); // movd [px+16*0-4], xm0
        movd_st(at(px, 16 * 1 - 4), m[1]); // movd [px+16*1-4], xm1
        movd_st(at(px, 16 * 2 - 4), m[2]); // movd [px+16*2-4], xm2
        movd_st(at(px, 16 * 3 - 4), m[3]); // movd [px+16*3-4], xm3
        m[0] = movd_ld(left.offset(4 * 4)); // movd xm0, [leftq+4*4]
        m[1] = movd_ld(left.offset(4 * 5)); // movd xm1, [leftq+4*5]
        m[2] = movd_ld(left.offset(4 * 6)); // movd xm2, [leftq+4*6]
        m[3] = movd_ld(left.offset(4 * 7)); // movd xm3, [leftq+4*7]
        movd_st(at(px, 16 * 4 - 4), m[0]); // movd [px+16*4-4], xm0
        movd_st(at(px, 16 * 5 - 4), m[1]); // movd [px+16*5-4], xm1
        movd_st(at(px, 16 * 6 - 4), m[2]); // movd [px+16*6-4], xm2
        movd_st(at(px, 16 * 7 - 4), m[3]); // movd [px+16*7-4], xm3
    } else {
        // .no_left:
        for x in 0..8 {
            movd_st(at(px, 16 * x - 4), m[7]); // REPX {movd [px+16*x-4], xm7}, 0, 1, 2, 3, 4, 5, 6, 7
        }
    }
    // .left_done:
    if edge & 2 == 0 {
        // test edgeb, 2 ; HAVE_RIGHT ; jnz .padding_done
        for x in -2..10 {
            movd_st(at(px, 16 * x + 8), m[7]); // REPX {movd [px+16*x+8], xm7}, -2, -1, 0, … 9
        }
    }
    // .padding_done:
    cdef_filter::<4, 8>(&mut m, dst, stride, stride3, px, pri, sec, dir, damping, bitdepth_max);
}

/// `cglobal cdef_filter_8x8_16bpc` up to `.padding_done`, then `CDEF_FILTER 8, 8`. `px` rows are 32 bytes
/// apart: columns 0..9 at `px+32*y+0` (and the copy of columns 10..13 of the 32-byte row loads), columns
/// -2 and -1 at `px+32*y-4`, rows -2..9.
#[inline(always)]
unsafe fn cdef_filter_8x8_16bpc(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
    bitdepth_max: c_int,
) {
    let mut m = regs();
    // The frame, `32*13` bytes with `px` at `+32*3` as in the assembly (the filter reads `px-68` to
    // `px+308`); zeroed whole.
    let mut frame = [_mm256_setzero_si256(); 13];
    let px = at(frame.as_mut_ptr().cast::<u8>(), 32 * 3);
    let edge = edges; // mov edged, r9m
    m[0] = _mm256_loadu_si256(dst.cast()); // movu m0, [dstq+strideq*0]
    m[1] = _mm256_loadu_si256(at(dst, stride).cast()); // movu m1, [dstq+strideq*1]
    let mut r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[2] = _mm256_loadu_si256(r6.cast()); // movu m2, [r6+strideq*0]
    m[3] = _mm256_loadu_si256(at(r6, stride).cast()); // movu m3, [r6+strideq*1]
    r6 = at(r6, stride * 2); // lea r6, [r6+strideq*2]
    m[4] = _mm256_loadu_si256(r6.cast()); // movu m4, [r6+strideq*0]
    m[5] = _mm256_loadu_si256(at(r6, stride).cast()); // movu m5, [r6+strideq*1]
    r6 = at(r6, stride * 2); // lea r6, [r6+strideq*2]
    m[6] = _mm256_loadu_si256(r6.cast()); // movu m6, [r6+strideq*0]
    m[7] = _mm256_loadu_si256(at(r6, stride).cast()); // movu m7, [r6+strideq*1]
                                                      // lea r8, [dir_table8]
    for y in 0..8 {
        _mm256_storeu_si256(at(px, 32 * y as isize).cast(), m[y]); // mova [px+32*y+0], my (y = 0..7)
    }
    m[7] = _mm256_set1_epi16(pw_m16384); // vpbroadcastd m7, [base+pw_m16384]
    'top: {
        if edge & 4 != 0 {
            // test edgeb, 4 ; HAVE_TOP ; jz .no_top
            m[0] = _mm256_loadu_si256(top.cast()); // movu m0, [topq+strideq*0]
            m[1] = _mm256_loadu_si256(top.offset(stride).cast()); // movu m1, [topq+strideq*1]
            _mm256_storeu_si256(at(px, -32 * 2).cast(), m[0]); // mova [px-32*2+0], m0
            _mm256_storeu_si256(at(px, -32 * 1).cast(), m[1]); // mova [px-32*1+0], m1
            if edge & 1 != 0 {
                // test edgeb, 1 ; HAVE_LEFT ; jz .top_no_left
                m[0] = movd_ld(top.offset(-4)); // movd xm0, [topq+strideq*0-4]
                m[1] = movd_ld(top.offset(stride - 4)); // movd xm1, [topq+strideq*1-4]
                movd_st(at(px, -32 * 2 - 4), m[0]); // movd [px-32*2-4], xm0
                movd_st(at(px, -32 * 1 - 4), m[1]); // movd [px-32*1-4], xm1
                break 'top; // jmp .top_done
            }
        } else {
            // .no_top:
            _mm256_storeu_si256(at(px, -32 * 2).cast(), m[7]); // mova [px-32*2+0], m7
            _mm256_storeu_si256(at(px, -32 * 1).cast(), m[7]); // mova [px-32*1+0], m7
        }
        // .top_no_left:
        movd_st(at(px, -32 * 2 - 4), m[7]); // movd [px-32*2-4], xm7
        movd_st(at(px, -32 * 1 - 4), m[7]); // movd [px-32*1-4], xm7
    }
    // .top_done:
    'bottom: {
        if edge & 8 != 0 {
            // test edgeb, 8 ; HAVE_BOTTOM ; jz .no_bottom
            m[0] = _mm256_loadu_si256(bot.cast()); // movu m0, [botq+strideq*0]
            m[1] = _mm256_loadu_si256(bot.offset(stride).cast()); // movu m1, [botq+strideq*1]
            _mm256_storeu_si256(at(px, 32 * 8).cast(), m[0]); // mova [px+32*8+0], m0
            _mm256_storeu_si256(at(px, 32 * 9).cast(), m[1]); // mova [px+32*9+0], m1
            if edge & 1 != 0 {
                // test edgeb, 1 ; HAVE_LEFT ; jz .bottom_no_left
                m[0] = movd_ld(bot.offset(-4)); // movd xm0, [botq+strideq*0-4]
                m[1] = movd_ld(bot.offset(stride - 4)); // movd xm1, [botq+strideq*1-4]
                movd_st(at(px, 32 * 8 - 4), m[0]); // movd [px+32*8-4], xm0
                movd_st(at(px, 32 * 9 - 4), m[1]); // movd [px+32*9-4], xm1
                break 'bottom; // jmp .bottom_done
            }
        } else {
            // .no_bottom:
            _mm256_storeu_si256(at(px, 32 * 8).cast(), m[7]); // mova [px+32*8+0], m7
            _mm256_storeu_si256(at(px, 32 * 9).cast(), m[7]); // mova [px+32*9+0], m7
        }
        // .bottom_no_left:
        movd_st(at(px, 32 * 8 - 4), m[7]); // movd [px+32*8-4], xm7
        movd_st(at(px, 32 * 9 - 4), m[7]); // movd [px+32*9-4], xm7
    }
    // .bottom_done:
    if edge & 1 != 0 {
        // test edgeb, 1 ; HAVE_LEFT ; jz .no_left
        m[0] = movd_ld(left.offset(4 * 0)); // movd xm0, [leftq+4*0]
        m[1] = movd_ld(left.offset(4 * 1)); // movd xm1, [leftq+4*1]
        m[2] = movd_ld(left.offset(4 * 2)); // movd xm2, [leftq+4*2]
        m[3] = movd_ld(left.offset(4 * 3)); // movd xm3, [leftq+4*3]
        movd_st(at(px, 32 * 0 - 4), m[0]); // movd [px+32*0-4], xm0
        movd_st(at(px, 32 * 1 - 4), m[1]); // movd [px+32*1-4], xm1
        movd_st(at(px, 32 * 2 - 4), m[2]); // movd [px+32*2-4], xm2
        movd_st(at(px, 32 * 3 - 4), m[3]); // movd [px+32*3-4], xm3
        m[0] = movd_ld(left.offset(4 * 4)); // movd xm0, [leftq+4*4]
        m[1] = movd_ld(left.offset(4 * 5)); // movd xm1, [leftq+4*5]
        m[2] = movd_ld(left.offset(4 * 6)); // movd xm2, [leftq+4*6]
        m[3] = movd_ld(left.offset(4 * 7)); // movd xm3, [leftq+4*7]
        movd_st(at(px, 32 * 4 - 4), m[0]); // movd [px+32*4-4], xm0
        movd_st(at(px, 32 * 5 - 4), m[1]); // movd [px+32*5-4], xm1
        movd_st(at(px, 32 * 6 - 4), m[2]); // movd [px+32*6-4], xm2
        movd_st(at(px, 32 * 7 - 4), m[3]); // movd [px+32*7-4], xm3
    } else {
        // .no_left:
        for x in 0..8 {
            movd_st(at(px, 32 * x - 4), m[7]); // REPX {movd [px+32*x-4], xm7}, 0, 1, 2, 3, 4, 5, 6, 7
        }
    }
    // .left_done:
    if edge & 2 == 0 {
        // test edgeb, 2 ; HAVE_RIGHT ; jnz .padding_done
        for x in -2..10 {
            movd_st(at(px, 32 * x + 16), m[7]); // REPX {movd [px+32*x+16], xm7}, -2, -1, 0, … 9
        }
    }
    // .padding_done:
    cdef_filter::<8, 8>(&mut m, dst, stride, 0, px, pri, sec, dir, damping, bitdepth_max);
}

macro_rules! cdef_filter_fn {
    ($name:ident, $body:ident) => {
        /// The `cdef_fn` of `cdef16_avx2.asm` of the same name (`dst`, `top`, `bot`, `left` as `cdef.h` and
        /// `cdef_apply_tmpl.c` define them; `stride` in bytes).
        #[target_feature(enable = "avx2")]
        pub unsafe extern "C" fn $name(
            dst: *mut u16,
            stride: isize,
            left: *const [u16; 2],
            top: *const u16,
            bot: *const u16,
            pri: c_int,
            sec: c_int,
            dir: c_int,
            damping: c_int,
            edges: c_uint,
            bitdepth_max: c_int,
        ) {
            $body(
                dst.cast(),
                stride,
                left.cast(),
                top.cast(),
                bot.cast(),
                pri,
                sec,
                dir,
                damping,
                edges,
                bitdepth_max,
            )
        }
    };
}
cdef_filter_fn!(dav1d_cdef_filter_8x8_16bpc_avx2, cdef_filter_8x8_16bpc);
cdef_filter_fn!(dav1d_cdef_filter_4x8_16bpc_avx2, cdef_filter_4x8_16bpc);
cdef_filter_fn!(dav1d_cdef_filter_4x4_16bpc_avx2, cdef_filter_4x4_16bpc);

// ---- cdef_dir_16bpc ----

/// `cglobal cdef_dir_16bpc`: the 8x8 block scaled to 8 bits (`pmulhuw` by `dir_shift`: `>> 2` at 10 bit,
/// `>> 4` at 12 bit), rows 0 | 7, 1 | 6, 2 | 5, 3 | 4 in `m0`…`m3`, then `jmp cdef_dir_8bpc_avx2.main`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_cdef_dir_16bpc_avx2(
    src: *const u16,
    stride: isize,
    var: *mut c_uint,
    bitdepth_max: c_int,
) -> c_int {
    let mut m = regs();
    let mut src = src as *mut u8;
    // lea r6, [dir_shift]
    let bdmax = (bitdepth_max as u32 >> 11) as usize; // shr bdmaxd, 11 ; 0 for 10bpc, 1 for 12bpc
                                                      // `bdmax` is 0 or 1 (`bitdepth_max` is 0x3ff or 0xfff): the read lies inside the 8-byte `dir_shift`.
    let shift = dir_shift
        .as_ptr()
        .cast::<u8>()
        .add(bdmax * 4)
        .cast::<i32>()
        .read_unaligned();
    m[4] = _mm256_set1_epi32(shift); // vpbroadcastd m4, [r6+bdmaxq*4]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[0] = zx(_mm_loadu_si128(src.cast())); // mova xm0, [srcq+strideq*0]
    m[1] = zx(_mm_loadu_si128(at(src, stride).cast())); // mova xm1, [srcq+strideq*1]
    m[2] = zx(_mm_loadu_si128(at(src, stride * 2).cast())); // mova xm2, [srcq+strideq*2]
    m[3] = zx(_mm_loadu_si128(at(src, r6).cast())); // mova xm3, [srcq+r6]
    src = at(src, stride * 4); // lea srcq, [srcq+strideq*4]
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(src, r6).cast())); // vinserti128 m0, [srcq+r6], 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(src, stride * 2).cast())); // vinserti128 m1, [srcq+strideq*2], 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(src, stride).cast())); // vinserti128 m2, [srcq+strideq*1], 1
    m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(src.cast())); // vinserti128 m3, [srcq+strideq*0], 1
    for x in 0..4 {
        m[x] = _mm256_mulhi_epu16(m[x], m[4]); // REPX {pmulhuw x, m4}, m0, m1, m2, m3
    }
    cdef_dir_8bpc_main(&mut m, var) // jmp cdef_dir_8bpc_avx2.main
}
