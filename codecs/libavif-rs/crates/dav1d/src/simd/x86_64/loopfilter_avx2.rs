// Copyright © 2018-2021, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/loopfilter_avx2.asm` (8 bpc) with
//! AVX2 intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The four entry points `dav1d_lpf_{v,h}_sb_{y,uv}_8bpc_avx2`. Each is a
//! loop over groups of eight 4-pixel units (32 pixels along the edge, one
//! 256-bit register per pixel row or column across the edge) that picks
//! the widest filter the group needs from the group's byte of `vmask[2]`,
//! `vmask[1]` and `vmask[0]` and applies it to the whole group; inside
//! the filter the masks select per unit.
//!
//! Correspondence with the assembly:
//! - Each `cglobal` function is the `extern "C"` function of the same name.
//!   The macro `FILTER width, dir` is `filter::<WIDTH, DIR>` (`DIR` is `V`
//!   or `H`), with the `%if`s of the macro as `if`s on the two constants,
//!   so each of the eight expansions is one monomorphic copy; the local
//!   subroutines `.v4` and `.h4` (`FILTER 4, v` and `FILTER 4, h`, called
//!   from the luma and the chroma entry points) are `v4` and `h4`. The
//!   macros `ABSSUB`, `TRANSPOSE_16x4_AND_WRITE_4x32` and
//!   `TRANSPOSE_16X16B` are the `macro_rules!` of the same names, expanded
//!   in place with the register names as arguments, as in the assembly.
//! - The vector registers are locals `m0` … `m15` (`xm0` … where the
//!   assembly builds a register from 128-bit loads; the VEX encoding
//!   clears the upper half, which is `_mm256_zextsi128_si256`), the
//!   general-purpose registers locals named as in `DEFINE_ARGS` (`dst`,
//!   `stride`, `mask`, `l`, `l_stride`, `lut`, `w`, `stride3`, `mstride`,
//!   `l_stride3`, `tmp`). `SWAP` (x86inc's renaming of registers: a chain
//!   of pairwise exchanges of names, no instruction) is `swap!`, the same
//!   chain of exchanges of the locals.
//! - The stack area (`[rsp+n*32]`, 11 slots in `lpf_v_sb_y`, 21 in
//!   `lpf_h_sb_y`) holds spilled registers and, in `FILTER 16, h`, the
//!   transposed columns the filter does not keep in registers. Each slot
//!   is a local `s0` … `s20` of the same number; the compiler chooses
//!   which of them live in memory. There is no stack buffer.
//! - The constants of `SECTION_RODATA 32` are statics of the same names
//!   (`pb_128`, `pw_2048`, …), and an instruction's memory operand
//!   `[pb_128]` is `mem(&pb_128)`. `mem` loads the 32 bytes with a
//!   volatile read. The compiler then keeps every use as the memory
//!   operand the assembly has; a plain constant becomes one broadcast
//!   into a register per constant and path, and takes the registers the
//!   filter needs for pixels (see the deviations).
//! - `maskq` advances by one *byte* per group of eight units, and the
//!   masks are read from it as bytes (`cmp byte [maskq+8], 0`) and as
//!   doublewords (`vpbroadcastd m, [maskq+4]`, whose low byte is compared
//!   with `pb_mask` bit by bit, one bit per unit); `mask` is therefore a
//!   byte pointer here.
//!
//! Deviations from the assembly:
//! - Every vector access to caller memory is an unaligned move
//!   (`vmovdqu`), where the assembly uses `mova` for the pixel rows of the
//!   `v` filters (`dst` is aligned by 32 by contract): the same bytes and
//!   the same number of instructions, and a pointer that is not aligned
//!   is not undefined behavior.
//! - The volatile reads of the constants (`mem`) are not in the assembly;
//!   they are what makes the compiler emit the assembly's memory operands.
//!   With plain constants (`mem` returning the value) the outputs are the
//!   same and the kernels take more instructions.
//! - `movhps xm, [mem]` and `movhps [mem], xm` are written out with an
//!   unaligned read or write of the 8 bytes (they compile to `vmovhps`
//!   and `vpextrq [mem], xm, 1`: the same bytes, one instruction each).
//!   The intrinsics `_mm_loadh_pd` and `_mm_storeh_pd` access the memory
//!   through an aligned `f64` pointer, which a pixel row is not.
//! - Instruction scheduling, register allocation and the choice between a
//!   register and a stack slot are the compiler's. It moves more vectors
//!   to and from the stack than the assembly in the `h` filters of luma,
//!   and in `lpf_v_sb_y` it keeps the addresses of the pixel rows as
//!   separate pointers, computed before the loop and reloaded from the
//!   stack, where the assembly forms them with `lea` from `dstq` and
//!   `strideq`.
//! - `.v4` and `.h4` are functions the compiler may inline into their
//!   callers; the assembly `call`s one copy of each.
//! - `FILTER n, h` leaves `dstq` 32 rows and `lq` 8 rows further; here
//!   `filter` returns the two advanced pointers to the entry point's
//!   loop.
//!
//! Safety (argued once for the family): the translated caller
//! `lf_apply_tmpl` passes what the original passes to the assembly: `dst`
//! points into a picture plane with `stride` bytes per row ("dst/stride
//! are aligned by 32", src/loopfilter.h), `mask` to the `vmask`/`hmask`
//! array of the edge (4 words for luma, 3 for chroma, the last one 0),
//! `l` to the filter levels of the edge's blocks with `l_stride` blocks
//! per row (in an array that is allocated with 3 bytes to spare), `lut`
//! to the frame's `Av1FilterLUT`, and `w` is the number of 4-pixel units
//! along the edge, at most 32. Per group of eight units (`ceil(w / 8)`
//! groups, at least one) and only when the group's byte of one of the
//! masks is not 0, the kernels read and write what the assembly reads
//! and writes, which each entry point lists: always the whole group (32
//! pixels along the edge, 8 levels), also when `w` ends inside it or a
//! unit's mask bits are clear; such a unit's pixels are written back as
//! read. All caller memory is reached through raw pointers with
//! unaligned accesses; addresses are formed with wrapping arithmetic, so
//! a pointer the assembly advances past the last access (the `dst` and
//! `l` of the `h` filters, `mask`) is never claimed to be in bounds. The
//! only memory of the kernels' own is locals and the constants.
#![allow(clippy::all, non_snake_case, non_upper_case_globals, unused_assignments)]

use core::arch::x86_64::*;
use core::ffi::c_int;

/// `Av1FilterLUT` (src/lf_mask.h): `e[64]`, `i[64]`, `sharp[2]`.
#[repr(C)]
pub struct Av1FilterLUT {
    pub e: [u8; 64],
    pub i: [u8; 64],
    pub sharp: [u64; 2],
}

/// The `dir` argument of `FILTER`.
const V: bool = false;
const H: bool = true;

// --- SECTION_RODATA 32 ---------------------------------------------------------------------------------------

/// One 32-byte constant of `SECTION_RODATA 32`.
#[repr(C, align(32))]
struct Rodata([u8; 32]);

/// `times 16 db a, b`
const fn db2(a: u8, b: u8) -> Rodata {
    let mut bytes = [a; 32];
    let mut i = 1;
    while i < 32 {
        bytes[i] = b;
        i += 2;
    }
    Rodata(bytes)
}
/// `times 32 db a`
const fn db(a: u8) -> Rodata {
    Rodata([a; 32])
}
/// `times 16 dw a`
const fn dw(a: u16) -> Rodata {
    db2(a as u8, (a >> 8) as u8)
}

#[rustfmt::skip]
static pb_4x1_4x5_4x9_4x13: Rodata = Rodata([ // times 2 db 0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12
    0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12, 0, 0, 0, 0, 4, 4, 4, 4, 8, 8, 8, 8, 12, 12, 12, 12,
]);
static pb_7_1: Rodata = db2(7, 1);
static pb_3_1: Rodata = db2(3, 1);
static pb_2_1: Rodata = db2(2, 1);
static pb_m1_0: Rodata = db2(0xff, 0);
static pb_m1_1: Rodata = db2(0xff, 1);
static pb_m1_2: Rodata = db2(0xff, 2);
static pb_1: Rodata = db(1);
static pb_2: Rodata = db(2);
static pb_3: Rodata = db(3);
static pb_4: Rodata = db(4);
static pb_16: Rodata = db(16);
static pb_63: Rodata = db(63);
static pb_64: Rodata = db(64);
static pb_128: Rodata = db(0x80);
static pb_129: Rodata = db(0x81);
static pb_240: Rodata = db(0xf0);
static pb_248: Rodata = db(0xf8);
static pb_254: Rodata = db(0xfe);

static pw_2048: Rodata = dw(2048);
static pw_4096: Rodata = dw(4096);

#[rustfmt::skip]
static pb_mask: Rodata = Rodata([ // dd 1, 2, 4, 8, 16, 32, 64, 128
    1, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0, 16, 0, 0, 0, 32, 0, 0, 0, 64, 0, 0, 0, 128, 0, 0, 0,
]);

/// The memory operand `[c]` of an instruction, `c` one of the constants
/// above. A volatile read, so that every use stays a load the compiler
/// folds into the instruction as its memory operand, as in the assembly
/// (see the module documentation); the value is the constant's.
///
/// # Safety
/// The CPU supports AVX2. (`c` is a live, initialized, 32-byte aligned
/// static of 32 bytes: valid for the read.)
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn mem(c: &'static Rodata) -> __m256i {
    core::ptr::read_volatile((c as *const Rodata).cast::<__m256i>())
}

// --- addresses and the memory forms of the 128-bit instructions --------------------------------------------

/// `[p+off]`: the address only (wrapping, so forming it claims nothing).
#[inline(always)]
fn at(p: *mut u8, off: isize) -> *mut u8 {
    p.wrapping_offset(off)
}

/// `movd xm, [p]` (the upper bits of the register are cleared).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movd_ld(p: *const u8) -> __m128i {
    _mm_cvtsi32_si128(p.cast::<i32>().read_unaligned())
}
/// `pinsrd xm, [p], I`
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn pinsrd_ld<const I: i32>(xm: __m128i, p: *const u8) -> __m128i {
    _mm_insert_epi32::<I>(xm, p.cast::<i32>().read_unaligned())
}
/// `movq xm, [p]` (the upper bits of the register are cleared).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movq_ld(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}
/// `movhps xm, [p]`: what `_mm_loadh_pd` is (the low element of `xm` and
/// the 8 bytes at `p` as the high one), with an unaligned read where the
/// intrinsic reads through an aligned `*const f64`. The 8 bytes are only
/// moved.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movhps_ld(xm: __m128i, p: *const u8) -> __m128i {
    let xm = _mm_castsi128_pd(xm);
    _mm_castpd_si128(_mm_setr_pd(_mm_cvtsd_f64(xm), p.cast::<f64>().read_unaligned()))
}
/// `movd [p], xm`
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movd_st(p: *mut u8, xm: __m128i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(xm))
}
/// `pextrd [p], xm, I`
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn pextrd_st<const I: i32>(p: *mut u8, xm: __m128i) {
    p.cast::<i32>().write_unaligned(_mm_extract_epi32::<I>(xm))
}
/// `movq [p], xm`
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movq_st(p: *mut u8, xm: __m128i) {
    _mm_storel_epi64(p.cast(), xm)
}
/// `movhps [p], xm`: the high quadword, as the unaligned write of an
/// extracted element (`vpextrq [p], xm, 1`, the same 8 bytes in one
/// instruction; `_mm_storeh_pd` writes through an aligned `*mut f64` and,
/// on an integer register, compiles to `vpshufd` and `vmovq`).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movhps_st(p: *mut u8, xm: __m128i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(xm))
}
/// `movu xm, [p]` (the upper bits of the register are cleared by
/// `vinserti128_ld`'s `_mm256_zextsi128_si256`, as VEX does).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movu_ld(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}
/// `vinserti128 m, [p], 1`, `m` the register whose low half is `xm`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn vinserti128_ld(xm: __m128i, p: *const u8) -> __m256i {
    _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm), _mm_loadu_si128(p.cast()))
}
/// `movu [p], xm`, `xm` the low half of `m`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn movu_xm_st(p: *mut u8, m: __m256i) {
    _mm_storeu_si128(p.cast(), _mm256_castsi256_si128(m))
}
/// `vextracti128 [p], m, 1`
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn vextracti128_st(p: *mut u8, m: __m256i) {
    _mm_storeu_si128(p.cast(), _mm256_extracti128_si256::<1>(m))
}
/// `shufps m, a, b, IMM` on integer registers.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn shufps<const IMM: i32>(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castps_si256(_mm256_shuffle_ps::<IMM>(_mm256_castsi256_ps(a), _mm256_castsi256_ps(b)))
}
/// x86inc's `q3131` and `q2020` (base-4 digits, highest first).
const Q3131: i32 = 0b11_01_11_01;
const Q2020: i32 = 0b10_00_10_00;
/// `vpbroadcastd m, [p]`
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn vpbroadcastd_ld(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `SWAP a, b, c, …` of x86inc.asm: exchanges the names `a` and `b`, then
/// `b` and `c`, and so on. No instruction.
macro_rules! swap {
    ($a:ident, $b:ident) => {
        core::mem::swap(&mut $a, &mut $b);
    };
    ($a:ident, $b:ident, $($rest:ident),+) => {
        core::mem::swap(&mut $a, &mut $b);
        swap!($b, $($rest),+);
    };
}

/// `%macro ABSSUB 4 ; dst, a, b, tmp`
macro_rules! ABSSUB {
    ($dst:ident, $a:ident, $b:ident, $tmp:ident) => {
        $dst = _mm256_subs_epu8($a, $b); // psubusb %1, %2, %3
        $tmp = _mm256_subs_epu8($b, $a); // psubusb %4, %3, %2
        $dst = _mm256_or_si256($dst, $tmp); // por %1, %4
    };
}

/// `%macro TRANSPOSE_16x4_AND_WRITE_4x32 5`: the four registers `%1` … `%4`
/// hold the pixel columns -2 … +1 of 32 rows (row `r` in byte `r`, with the
/// rows of the two lanes as the loads of `FILTER n, h` leave them);
/// transposes them and writes 4 pixels at `dst - 2` in each of the 32 rows.
/// `%5` is a scratch register. Advances `dst` by 32 rows.
macro_rules! TRANSPOSE_16x4_AND_WRITE_4x32 {
    ($dst:ident, $stride:ident, $stride3:ident; $m1:ident, $m2:ident, $m3:ident, $m4:ident, $m5:ident) => {
        // transpose 16x4
        $m5 = _mm256_unpacklo_epi8($m1, $m2); // punpcklbw m%5, m%1, m%2
        $m1 = _mm256_unpackhi_epi8($m1, $m2); // punpckhbw m%1, m%2
        $m2 = _mm256_unpacklo_epi8($m3, $m4); // punpcklbw m%2, m%3, m%4
        $m3 = _mm256_unpackhi_epi8($m3, $m4); // punpckhbw m%3, m%4
        $m4 = _mm256_unpacklo_epi16($m5, $m2); // punpcklwd m%4, m%5, m%2
        $m5 = _mm256_unpackhi_epi16($m5, $m2); // punpckhwd m%5, m%2
        $m2 = _mm256_unpacklo_epi16($m1, $m3); // punpcklwd m%2, m%1, m%3
        $m1 = _mm256_unpackhi_epi16($m1, $m3); // punpckhwd m%1, m%3

        // write out
        let mut xm4 = _mm256_castsi256_si128($m4);
        let mut xm5 = _mm256_castsi256_si128($m5);
        let mut xm2 = _mm256_castsi256_si128($m2);
        let mut xm1 = _mm256_castsi256_si128($m1);
        movd_st(at($dst, -2), xm4); // movd [dstq+strideq*0-2], xm%4
        pextrd_st::<1>(at($dst, $stride - 2), xm4); // pextrd [dstq+strideq*1-2], xm%4, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm4); // pextrd [dstq+strideq*2-2], xm%4, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm4); // pextrd [dstq+stride3q-2], xm%4, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
        movd_st(at($dst, -2), xm5); // movd [dstq+strideq*0-2], xm%5
        pextrd_st::<1>(at($dst, $stride - 2), xm5); // pextrd [dstq+strideq*1-2], xm%5, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm5); // pextrd [dstq+strideq*2-2], xm%5, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm5); // pextrd [dstq+stride3q-2], xm%5, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
        movd_st(at($dst, -2), xm2); // movd [dstq+strideq*0-2], xm%2
        pextrd_st::<1>(at($dst, $stride - 2), xm2); // pextrd [dstq+strideq*1-2], xm%2, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm2); // pextrd [dstq+strideq*2-2], xm%2, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm2); // pextrd [dstq+stride3q-2], xm%2, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
        movd_st(at($dst, -2), xm1); // movd [dstq+strideq*0-2], xm%1
        pextrd_st::<1>(at($dst, $stride - 2), xm1); // pextrd [dstq+strideq*1-2], xm%1, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm1); // pextrd [dstq+strideq*2-2], xm%1, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm1); // pextrd [dstq+stride3q-2], xm%1, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]

        xm4 = _mm256_extracti128_si256::<1>($m4); // vextracti128 xm%4, m%4, 1
        xm5 = _mm256_extracti128_si256::<1>($m5); // vextracti128 xm%5, m%5, 1
        xm2 = _mm256_extracti128_si256::<1>($m2); // vextracti128 xm%2, m%2, 1
        xm1 = _mm256_extracti128_si256::<1>($m1); // vextracti128 xm%1, m%1, 1

        movd_st(at($dst, -2), xm4); // movd [dstq+strideq*0-2], xm%4
        pextrd_st::<1>(at($dst, $stride - 2), xm4); // pextrd [dstq+strideq*1-2], xm%4, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm4); // pextrd [dstq+strideq*2-2], xm%4, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm4); // pextrd [dstq+stride3q-2], xm%4, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
        movd_st(at($dst, -2), xm5); // movd [dstq+strideq*0-2], xm%5
        pextrd_st::<1>(at($dst, $stride - 2), xm5); // pextrd [dstq+strideq*1-2], xm%5, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm5); // pextrd [dstq+strideq*2-2], xm%5, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm5); // pextrd [dstq+stride3q-2], xm%5, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
        movd_st(at($dst, -2), xm2); // movd [dstq+strideq*0-2], xm%2
        pextrd_st::<1>(at($dst, $stride - 2), xm2); // pextrd [dstq+strideq*1-2], xm%2, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm2); // pextrd [dstq+strideq*2-2], xm%2, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm2); // pextrd [dstq+stride3q-2], xm%2, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
        movd_st(at($dst, -2), xm1); // movd [dstq+strideq*0-2], xm%1
        pextrd_st::<1>(at($dst, $stride - 2), xm1); // pextrd [dstq+strideq*1-2], xm%1, 1
        pextrd_st::<2>(at($dst, $stride * 2 - 2), xm1); // pextrd [dstq+strideq*2-2], xm%1, 2
        pextrd_st::<3>(at($dst, $stride3 - 2), xm1); // pextrd [dstq+stride3q-2], xm%1, 3
        $dst = at($dst, $stride * 4); // lea dstq, [dstq+strideq*4]
    };
}

/// `%macro TRANSPOSE_16X16B 3 ; in_load_15_from_mem, out_store_0_in_mem, mem`
/// on the registers `m0` … `m15` (passed by name, in order). `mem` is the
/// stack slot that stands in for the seventeenth register the transpose
/// needs: with `in_load_15_from_mem` it holds input 15 on entry, with
/// `out_store_0_in_mem` output 0 on exit.
macro_rules! TRANSPOSE_16X16B {
    ($in_load_15_from_mem:literal, $out_store_0_in_mem:literal, $mem:ident;
     $m0:ident, $m1:ident, $m2:ident, $m3:ident, $m4:ident, $m5:ident, $m6:ident, $m7:ident,
     $m8:ident, $m9:ident, $m10:ident, $m11:ident, $m12:ident, $m13:ident, $m14:ident, $m15:ident) => {
        if $in_load_15_from_mem == 0 {
            $mem = $m15; // mova %3, m15
        }

        // input in m0-15
        $m15 = _mm256_unpacklo_epi8($m0, $m1); // punpcklbw m15, m0, m1
        $m0 = _mm256_unpackhi_epi8($m0, $m1); // punpckhbw m0, m1
        $m1 = _mm256_unpacklo_epi8($m2, $m3); // punpcklbw m1, m2, m3
        $m2 = _mm256_unpackhi_epi8($m2, $m3); // punpckhbw m2, m3
        $m3 = _mm256_unpacklo_epi8($m4, $m5); // punpcklbw m3, m4, m5
        $m4 = _mm256_unpackhi_epi8($m4, $m5); // punpckhbw m4, m5
        $m5 = _mm256_unpacklo_epi8($m6, $m7); // punpcklbw m5, m6, m7
        $m6 = _mm256_unpackhi_epi8($m6, $m7); // punpckhbw m6, m7
        $m7 = _mm256_unpacklo_epi8($m8, $m9); // punpcklbw m7, m8, m9
        $m8 = _mm256_unpackhi_epi8($m8, $m9); // punpckhbw m8, m9
        $m9 = _mm256_unpacklo_epi8($m10, $m11); // punpcklbw m9, m10, m11
        $m10 = _mm256_unpackhi_epi8($m10, $m11); // punpckhbw m10, m11
        $m11 = _mm256_unpacklo_epi8($m12, $m13); // punpcklbw m11, m12, m13
        $m12 = _mm256_unpackhi_epi8($m12, $m13); // punpckhbw m12, m13
        $m13 = $mem; // mova m13, %3
        $mem = $m12; // mova %3, m12
        $m12 = _mm256_unpacklo_epi8($m14, $m13); // punpcklbw m12, m14, m13
        $m13 = _mm256_unpackhi_epi8($m14, $m13); // punpckhbw m13, m14, m13

        // interleaved in m15,0,1,2,3,4,5,6,7,8,9,10,11,rsp%3,12,13
        $m14 = _mm256_unpacklo_epi16($m15, $m1); // punpcklwd m14, m15, m1
        $m15 = _mm256_unpackhi_epi16($m15, $m1); // punpckhwd m15, m1
        $m1 = _mm256_unpacklo_epi16($m0, $m2); // punpcklwd m1, m0, m2
        $m0 = _mm256_unpackhi_epi16($m0, $m2); // punpckhwd m0, m2
        $m2 = _mm256_unpacklo_epi16($m3, $m5); // punpcklwd m2, m3, m5
        $m3 = _mm256_unpackhi_epi16($m3, $m5); // punpckhwd m3, m5
        $m5 = _mm256_unpacklo_epi16($m4, $m6); // punpcklwd m5, m4, m6
        $m4 = _mm256_unpackhi_epi16($m4, $m6); // punpckhwd m4, m6
        $m6 = _mm256_unpacklo_epi16($m7, $m9); // punpcklwd m6, m7, m9
        $m7 = _mm256_unpackhi_epi16($m7, $m9); // punpckhwd m7, m9
        $m9 = _mm256_unpacklo_epi16($m8, $m10); // punpcklwd m9, m8, m10
        $m8 = _mm256_unpackhi_epi16($m8, $m10); // punpckhwd m8, m10
        $m10 = _mm256_unpacklo_epi16($m11, $m12); // punpcklwd m10, m11, m12
        $m11 = _mm256_unpackhi_epi16($m11, $m12); // punpckhwd m11, m12
        $m12 = $mem; // mova m12, %3
        $mem = $m11; // mova %3, m11
        $m11 = _mm256_unpacklo_epi16($m12, $m13); // punpcklwd m11, m12, m13
        $m12 = _mm256_unpackhi_epi16($m12, $m13); // punpckhwd m12, m13

        // interleaved in m14,15,1,0,2,3,5,4,6,7,9,8,10,rsp%3,11,12
        $m13 = _mm256_unpacklo_epi32($m14, $m2); // punpckldq m13, m14, m2
        $m14 = _mm256_unpackhi_epi32($m14, $m2); // punpckhdq m14, m2
        $m2 = _mm256_unpacklo_epi32($m15, $m3); // punpckldq m2, m15, m3
        $m15 = _mm256_unpackhi_epi32($m15, $m3); // punpckhdq m15, m3
        $m3 = _mm256_unpacklo_epi32($m1, $m5); // punpckldq m3, m1, m5
        $m1 = _mm256_unpackhi_epi32($m1, $m5); // punpckhdq m1, m5
        $m5 = _mm256_unpacklo_epi32($m0, $m4); // punpckldq m5, m0, m4
        $m0 = _mm256_unpackhi_epi32($m0, $m4); // punpckhdq m0, m4
        $m4 = _mm256_unpacklo_epi32($m6, $m10); // punpckldq m4, m6, m10
        $m6 = _mm256_unpackhi_epi32($m6, $m10); // punpckhdq m6, m10
        $m10 = _mm256_unpacklo_epi32($m9, $m11); // punpckldq m10, m9, m11
        $m9 = _mm256_unpackhi_epi32($m9, $m11); // punpckhdq m9, m11
        $m11 = _mm256_unpacklo_epi32($m8, $m12); // punpckldq m11, m8, m12
        $m8 = _mm256_unpackhi_epi32($m8, $m12); // punpckhdq m8, m12
        $m12 = $mem; // mova m12, %3
        $mem = $m8; // mova %3, m8
        $m8 = _mm256_unpacklo_epi32($m7, $m12); // punpckldq m8, m7, m12
        $m7 = _mm256_unpackhi_epi32($m7, $m12); // punpckhdq m7, m12

        // interleaved in m13,14,2,15,3,1,5,0,4,6,8,7,10,9,11,rsp%3
        $m12 = _mm256_unpacklo_epi64($m13, $m4); // punpcklqdq m12, m13, m4
        $m13 = _mm256_unpackhi_epi64($m13, $m4); // punpckhqdq m13, m4
        $m4 = _mm256_unpacklo_epi64($m14, $m6); // punpcklqdq m4, m14, m6
        $m14 = _mm256_unpackhi_epi64($m14, $m6); // punpckhqdq m14, m6
        $m6 = _mm256_unpacklo_epi64($m2, $m8); // punpcklqdq m6, m2, m8
        $m2 = _mm256_unpackhi_epi64($m2, $m8); // punpckhqdq m2, m8
        $m8 = _mm256_unpacklo_epi64($m15, $m7); // punpcklqdq m8, m15, m7
        $m15 = _mm256_unpackhi_epi64($m15, $m7); // punpckhqdq m15, m7
        $m7 = _mm256_unpacklo_epi64($m3, $m10); // punpcklqdq m7, m3, m10
        $m3 = _mm256_unpackhi_epi64($m3, $m10); // punpckhqdq m3, m10
        $m10 = _mm256_unpacklo_epi64($m1, $m9); // punpcklqdq m10, m1, m9
        $m1 = _mm256_unpackhi_epi64($m1, $m9); // punpckhqdq m1, m9
        $m9 = _mm256_unpacklo_epi64($m5, $m11); // punpcklqdq m9, m5, m11
        $m5 = _mm256_unpackhi_epi64($m5, $m11); // punpckhqdq m5, m11
        $m11 = $mem; // mova m11, %3
        $mem = $m12; // mova %3, m12
        $m12 = _mm256_unpacklo_epi64($m0, $m11); // punpcklqdq m12, m0, m11
        $m0 = _mm256_unpackhi_epi64($m0, $m11); // punpckhqdq m0, m11
        if $out_store_0_in_mem == 0 {
            $m11 = $mem; // mova m11, %3
        }

        // interleaved m11,13,4,14,6,2,8,15,7,3,10,1,9,5,12,0
        swap!($m0, $m11, $m1, $m13, $m5, $m2, $m4, $m6, $m8, $m7, $m15); // SWAP 0, 11, 1, 13, 5, 2, 4, 6, 8, 7, 15
        swap!($m3, $m14, $m12, $m9); // SWAP 3, 14, 12, 9
    };
}

/// `%macro FILTER 2 ; width [4/6/8/16], dir [h/v]`: filters one group of
/// eight units, 32 pixels along the edge. `mstride_or_l_stride3` is the
/// register r8 of the entry points: `mstrideq` for `v`, `l_stride3q` for
/// `h`. Returns `dstq` and `lq` as the macro leaves them: unchanged for
/// `v` (the loop advances them), 32 pixel rows and 8 level rows further
/// for `h`.
///
/// # Safety
/// As for the entry points (which state what each width reads and
/// writes); the CPU supports AVX2.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn filter<const WD: usize, const DIR_H: bool>(
    dst: *mut u8,
    stride: isize,
    mask: *const u8,
    l: *const u8,
    l_stride: isize,
    lut: *const Av1FilterLUT,
    stride3: isize,
    mstride_or_l_stride3: isize,
) -> (*mut u8, *const u8) {
    let (mut dst, mut l) = (dst, l as *mut u8);
    let (mstride, l_stride3) = (mstride_or_l_stride3, mstride_or_l_stride3);
    let mut tmp = dst;

    // The registers and the stack slots `[rsp+n*32]`. Each is assigned before
    // it is read in every expansion; the zero is what makes that a property
    // of the language instead of an argument, and the compiler drops it.
    let z = _mm256_setzero_si256();
    let (mut m0, mut m1, mut m2, mut m3, mut m4, mut m5, mut m6, mut m7) = (z, z, z, z, z, z, z, z);
    let (mut m8, mut m9, mut m10, mut m11, mut m12, mut m13, mut m14, mut m15) = (z, z, z, z, z, z, z, z);
    let (mut s0, mut s1, mut s2, mut s3, mut s4, mut s5, mut s6, mut s7, mut s8, mut s9, mut s10) =
        (z, z, z, z, z, z, z, z, z, z, z);
    let (mut s11, mut s12, mut s13, mut s14, mut s15, mut s16, mut s17, mut s18, mut s19, mut s20) =
        (z, z, z, z, z, z, z, z, z, z);

    // load data
    if !DIR_H {
        if WD == 4 {
            tmp = at(dst, mstride * 2); // lea tmpq, [dstq+mstrideq*2]
            m3 = _mm256_loadu_si256(tmp.cast()); // mova m3, [tmpq+strideq*0] ; p1
            m4 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m4, [tmpq+strideq*1] ; p0
            m5 = _mm256_loadu_si256(at(tmp, stride * 2).cast()); // mova m5, [tmpq+strideq*2] ; q0
            m6 = _mm256_loadu_si256(at(tmp, stride3).cast()); // mova m6, [tmpq+stride3q] ; q1
        } else {
            // load 6-8 pixels, remainder (for wd=16) will be read inline
            tmp = at(dst, mstride * 4); // lea tmpq, [dstq+mstrideq*4]
            if WD != 6 {
                m12 = _mm256_loadu_si256(tmp.cast()); // mova m12, [tmpq+strideq*0]
            }
            m13 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m13, [tmpq+strideq*1]
            m3 = _mm256_loadu_si256(at(tmp, stride * 2).cast()); // mova m3, [tmpq+strideq*2]
            m4 = _mm256_loadu_si256(at(tmp, stride3).cast()); // mova m4, [tmpq+stride3q]
            m5 = _mm256_loadu_si256(dst.cast()); // mova m5, [dstq+strideq*0]
            m6 = _mm256_loadu_si256(at(dst, stride).cast()); // mova m6, [dstq+strideq*1]
            m14 = _mm256_loadu_si256(at(dst, stride * 2).cast()); // mova m14, [dstq+strideq*2]
            if WD != 6 {
                m15 = _mm256_loadu_si256(at(dst, stride3).cast()); // mova m15, [dstq+stride3q]
            }
        }
    } else {
        // load lines
        if WD == 4 {
            let mut xm3 = movd_ld(at(dst, -2)); // movd xm3, [dstq+strideq*0-2]
            let mut xm4 = movd_ld(at(dst, stride - 2)); // movd xm4, [dstq+strideq*1-2]
            let mut xm5 = movd_ld(at(dst, stride * 2 - 2)); // movd xm5, [dstq+strideq*2-2]
            let mut xm6 = movd_ld(at(dst, stride3 - 2)); // movd xm6, [dstq+stride3q -2]
            tmp = at(dst, stride * 4); // lea tmpq, [dstq+strideq*4]
            xm3 = pinsrd_ld::<2>(xm3, at(tmp, -2)); // pinsrd xm3, [tmpq+strideq*0-2], 2
            xm4 = pinsrd_ld::<2>(xm4, at(tmp, stride - 2)); // pinsrd xm4, [tmpq+strideq*1-2], 2
            xm5 = pinsrd_ld::<2>(xm5, at(tmp, stride * 2 - 2)); // pinsrd xm5, [tmpq+strideq*2-2], 2
            xm6 = pinsrd_ld::<2>(xm6, at(tmp, stride3 - 2)); // pinsrd xm6, [tmpq+stride3q -2], 2
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            xm3 = pinsrd_ld::<1>(xm3, at(tmp, -2)); // pinsrd xm3, [tmpq+strideq*0-2], 1
            xm4 = pinsrd_ld::<1>(xm4, at(tmp, stride - 2)); // pinsrd xm4, [tmpq+strideq*1-2], 1
            xm5 = pinsrd_ld::<1>(xm5, at(tmp, stride * 2 - 2)); // pinsrd xm5, [tmpq+strideq*2-2], 1
            xm6 = pinsrd_ld::<1>(xm6, at(tmp, stride3 - 2)); // pinsrd xm6, [tmpq+stride3q -2], 1
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            xm3 = pinsrd_ld::<3>(xm3, at(tmp, -2)); // pinsrd xm3, [tmpq+strideq*0-2], 3
            xm4 = pinsrd_ld::<3>(xm4, at(tmp, stride - 2)); // pinsrd xm4, [tmpq+strideq*1-2], 3
            xm5 = pinsrd_ld::<3>(xm5, at(tmp, stride * 2 - 2)); // pinsrd xm5, [tmpq+strideq*2-2], 3
            xm6 = pinsrd_ld::<3>(xm6, at(tmp, stride3 - 2)); // pinsrd xm6, [tmpq+stride3q -2], 3
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            let mut xm12 = movd_ld(at(tmp, -2)); // movd xm12, [tmpq+strideq*0-2]
            let mut xm13 = movd_ld(at(tmp, stride - 2)); // movd xm13, [tmpq+strideq*1-2]
            let mut xm14 = movd_ld(at(tmp, stride * 2 - 2)); // movd xm14, [tmpq+strideq*2-2]
            let mut xm15 = movd_ld(at(tmp, stride3 - 2)); // movd xm15, [tmpq+stride3q -2]
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            xm12 = pinsrd_ld::<2>(xm12, at(tmp, -2)); // pinsrd xm12, [tmpq+strideq*0-2], 2
            xm13 = pinsrd_ld::<2>(xm13, at(tmp, stride - 2)); // pinsrd xm13, [tmpq+strideq*1-2], 2
            xm14 = pinsrd_ld::<2>(xm14, at(tmp, stride * 2 - 2)); // pinsrd xm14, [tmpq+strideq*2-2], 2
            xm15 = pinsrd_ld::<2>(xm15, at(tmp, stride3 - 2)); // pinsrd xm15, [tmpq+stride3q -2], 2
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            xm12 = pinsrd_ld::<1>(xm12, at(tmp, -2)); // pinsrd xm12, [tmpq+strideq*0-2], 1
            xm13 = pinsrd_ld::<1>(xm13, at(tmp, stride - 2)); // pinsrd xm13, [tmpq+strideq*1-2], 1
            xm14 = pinsrd_ld::<1>(xm14, at(tmp, stride * 2 - 2)); // pinsrd xm14, [tmpq+strideq*2-2], 1
            xm15 = pinsrd_ld::<1>(xm15, at(tmp, stride3 - 2)); // pinsrd xm15, [tmpq+stride3q -2], 1
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            xm12 = pinsrd_ld::<3>(xm12, at(tmp, -2)); // pinsrd xm12, [tmpq+strideq*0-2], 3
            xm13 = pinsrd_ld::<3>(xm13, at(tmp, stride - 2)); // pinsrd xm13, [tmpq+strideq*1-2], 3
            xm14 = pinsrd_ld::<3>(xm14, at(tmp, stride * 2 - 2)); // pinsrd xm14, [tmpq+strideq*2-2], 3
            xm15 = pinsrd_ld::<3>(xm15, at(tmp, stride3 - 2)); // pinsrd xm15, [tmpq+stride3q -2], 3
            m3 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm3), xm12); // vinserti128 m3, xm12, 1
            m4 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm4), xm13); // vinserti128 m4, xm13, 1
            m5 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm5), xm14); // vinserti128 m5, xm14, 1
            m6 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm6), xm15); // vinserti128 m6, xm15, 1

            // transpose 4x16
            // xm3: A-D0,A-D8,A-D4,A-D12
            // xm4: A-D1,A-D9,A-D5,A-D13
            // xm5: A-D2,A-D10,A-D6,A-D14
            // xm6: A-D3,A-D11,A-D7,A-D15
            m7 = _mm256_unpacklo_epi8(m3, m4); // punpcklbw m7, m3, m4
            m3 = _mm256_unpackhi_epi8(m3, m4); // punpckhbw m3, m4
            m4 = _mm256_unpacklo_epi8(m5, m6); // punpcklbw m4, m5, m6
            m5 = _mm256_unpackhi_epi8(m5, m6); // punpckhbw m5, m6

            // xm7: A0-1,B0-1,C0-1,D0-1,A8-9,B8-9,C8-9,D8-9
            // xm3: A4-5,B4-5,C4-5,D4-5,A12-13,B12-13,C12-13,D12-13
            // xm4: A2-3,B2-3,C2-3,D2-3,A10-11,B10-11,C10-11,D10-11
            // xm5: A6-7,B6-7,C6-7,D6-7,A14-15,B14-15,C14-15,D14-15
            m6 = _mm256_unpacklo_epi16(m7, m4); // punpcklwd m6, m7, m4
            m7 = _mm256_unpackhi_epi16(m7, m4); // punpckhwd m7, m4
            m4 = _mm256_unpacklo_epi16(m3, m5); // punpcklwd m4, m3, m5
            m3 = _mm256_unpackhi_epi16(m3, m5); // punpckhwd m3, m5

            // xm6: A0-3,B0-3,C0-3,D0-3
            // xm7: A8-11,B8-11,C8-11,D8-11
            // xm4: A4-7,B4-7,C4-7,D4-7
            // xm3: A12-15,B12-15,C12-15,D12-15
            m5 = _mm256_unpacklo_epi32(m6, m4); // punpckldq m5, m6, m4
            m6 = _mm256_unpackhi_epi32(m6, m4); // punpckhdq m6, m4
            m4 = _mm256_unpacklo_epi32(m7, m3); // punpckldq m4, m7, m3
            m7 = _mm256_unpackhi_epi32(m7, m3); // punpckhdq m7, m3

            // xm5: A0-7,B0-7
            // xm6: C0-7,D0-7
            // xm4: A8-15,B8-15
            // xm7: C8-15,D8-15
            m3 = _mm256_unpacklo_epi64(m5, m4); // punpcklqdq m3, m5, m4
            m4 = _mm256_unpackhi_epi64(m5, m4); // punpckhqdq m4, m5, m4
            m5 = _mm256_unpacklo_epi64(m6, m7); // punpcklqdq m5, m6, m7
            m6 = _mm256_unpackhi_epi64(m6, m7); // punpckhqdq m6, m7

        // xm3: A0-15
        // xm5: B0-15
        // xm4: C0-15
        // xm6: D0-15
        } else if WD == 6 || WD == 8 {
            let o = -((WD / 2) as isize); // -%1/2
            let mut xm3 = movq_ld(at(dst, o)); // movq xm3, [dstq+strideq*0-%1/2]
            let mut xm4 = movq_ld(at(dst, stride + o)); // movq xm4, [dstq+strideq*1-%1/2]
            let mut xm5 = movq_ld(at(dst, stride * 2 + o)); // movq xm5, [dstq+strideq*2-%1/2]
            let mut xm6 = movq_ld(at(dst, stride3 + o)); // movq xm6, [dstq+stride3q -%1/2]
            tmp = at(dst, stride * 8); // lea tmpq, [dstq+strideq*8]
            xm3 = movhps_ld(xm3, at(tmp, o)); // movhps xm3, [tmpq+strideq*0-%1/2]
            xm4 = movhps_ld(xm4, at(tmp, stride + o)); // movhps xm4, [tmpq+strideq*1-%1/2]
            xm5 = movhps_ld(xm5, at(tmp, stride * 2 + o)); // movhps xm5, [tmpq+strideq*2-%1/2]
            xm6 = movhps_ld(xm6, at(tmp, stride3 + o)); // movhps xm6, [tmpq+stride3q -%1/2]
            tmp = at(tmp, stride * 8); // lea tmpq, [tmpq+strideq*8]
            let mut xm7 = movq_ld(at(tmp, o)); // movq xm7, [tmpq+strideq*0-%1/2]
            let mut xm8 = movq_ld(at(tmp, stride + o)); // movq xm8, [tmpq+strideq*1-%1/2]
            let mut xm9 = movq_ld(at(tmp, stride * 2 + o)); // movq xm9, [tmpq+strideq*2-%1/2]
            let mut xm11 = movq_ld(at(tmp, stride3 + o)); // movq xm11, [tmpq+stride3q -%1/2]
            tmp = at(tmp, stride * 8); // lea tmpq, [tmpq+strideq*8]
            xm7 = movhps_ld(xm7, at(tmp, o)); // movhps xm7, [tmpq+strideq*0-%1/2]
            xm8 = movhps_ld(xm8, at(tmp, stride + o)); // movhps xm8, [tmpq+strideq*1-%1/2]
            xm9 = movhps_ld(xm9, at(tmp, stride * 2 + o)); // movhps xm9, [tmpq+strideq*2-%1/2]
            xm11 = movhps_ld(xm11, at(tmp, stride3 + o)); // movhps xm11, [tmpq+stride3q -%1/2]
            m3 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm3), xm7); // vinserti128 m3, xm7, 1
            m4 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm4), xm8); // vinserti128 m4, xm8, 1
            m5 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm5), xm9); // vinserti128 m5, xm9, 1
            m6 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm6), xm11); // vinserti128 m6, xm11, 1
            tmp = at(dst, stride * 4); // lea tmpq, [dstq+strideq*4]
            let mut xm12 = movq_ld(at(tmp, o)); // movq xm12, [tmpq+strideq*0-%1/2]
            let mut xm13 = movq_ld(at(tmp, stride + o)); // movq xm13, [tmpq+strideq*1-%1/2]
            let mut xm14 = movq_ld(at(tmp, stride * 2 + o)); // movq xm14, [tmpq+strideq*2-%1/2]
            let mut xm15 = movq_ld(at(tmp, stride3 + o)); // movq xm15, [tmpq+stride3q -%1/2]
            tmp = at(tmp, stride * 8); // lea tmpq, [tmpq+strideq*8]
            xm12 = movhps_ld(xm12, at(tmp, o)); // movhps xm12, [tmpq+strideq*0-%1/2]
            xm13 = movhps_ld(xm13, at(tmp, stride + o)); // movhps xm13, [tmpq+strideq*1-%1/2]
            xm14 = movhps_ld(xm14, at(tmp, stride * 2 + o)); // movhps xm14, [tmpq+strideq*2-%1/2]
            xm15 = movhps_ld(xm15, at(tmp, stride3 + o)); // movhps xm15, [tmpq+stride3q -%1/2]
            tmp = at(tmp, stride * 8); // lea tmpq, [tmpq+strideq*8]
            xm7 = movq_ld(at(tmp, o)); // movq xm7, [tmpq+strideq*0-%1/2]
            xm8 = movq_ld(at(tmp, stride + o)); // movq xm8, [tmpq+strideq*1-%1/2]
            xm9 = movq_ld(at(tmp, stride * 2 + o)); // movq xm9, [tmpq+strideq*2-%1/2]
            xm11 = movq_ld(at(tmp, stride3 + o)); // movq xm11, [tmpq+stride3q -%1/2]
            tmp = at(tmp, stride * 8); // lea tmpq, [tmpq+strideq*8]
            xm7 = movhps_ld(xm7, at(tmp, o)); // movhps xm7, [tmpq+strideq*0-%1/2]
            xm8 = movhps_ld(xm8, at(tmp, stride + o)); // movhps xm8, [tmpq+strideq*1-%1/2]
            xm9 = movhps_ld(xm9, at(tmp, stride * 2 + o)); // movhps xm9, [tmpq+strideq*2-%1/2]
            xm11 = movhps_ld(xm11, at(tmp, stride3 + o)); // movhps xm11, [tmpq+stride3q -%1/2]
            m12 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm12), xm7); // vinserti128 m12, xm7, 1
            m13 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm13), xm8); // vinserti128 m13, xm8, 1
            m14 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm14), xm9); // vinserti128 m14, xm9, 1
            m15 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm15), xm11); // vinserti128 m15, xm11, 1

            // transpose 8x16
            // xm3: A-H0,A-H8
            // xm4: A-H1,A-H9
            // xm5: A-H2,A-H10
            // xm6: A-H3,A-H11
            // xm12: A-H4,A-H12
            // xm13: A-H5,A-H13
            // xm14: A-H6,A-H14
            // xm15: A-H7,A-H15
            m7 = _mm256_unpacklo_epi8(m3, m4); // punpcklbw m7, m3, m4
            m3 = _mm256_unpackhi_epi8(m3, m4); // punpckhbw m3, m4
            m4 = _mm256_unpacklo_epi8(m5, m6); // punpcklbw m4, m5, m6
            m5 = _mm256_unpackhi_epi8(m5, m6); // punpckhbw m5, m6
            m6 = _mm256_unpacklo_epi8(m12, m13); // punpcklbw m6, m12, m13
            m12 = _mm256_unpackhi_epi8(m12, m13); // punpckhbw m12, m13
            m13 = _mm256_unpacklo_epi8(m14, m15); // punpcklbw m13, m14, m15
            m14 = _mm256_unpackhi_epi8(m14, m15); // punpckhbw m14, m15

            // xm7: A0-1,B0-1,C0-1,D0-1,E0-1,F0-1,G0-1,H0-1
            // xm3: A8-9,B8-9,C8-9,D8-9,E8-9,F8-9,G8-9,H8-9
            // xm4: A2-3,B2-3,C2-3,D2-3,E2-3,F2-3,G2-3,H2-3
            // xm5: A10-11,B10-11,C10-11,D10-11,E10-11,F10-11,G10-11,H10-11
            // xm6: A4-5,B4-5,C4-5,D4-5,E4-5,F4-5,G4-5,H4-5
            // xm12: A12-13,B12-13,C12-13,D12-13,E12-13,F12-13,G12-13,H12-13
            // xm13: A6-7,B6-7,C6-7,D6-7,E6-7,F6-7,G6-7,H6-7
            // xm14: A14-15,B14-15,C14-15,D14-15,E14-15,F14-15,G14-15,H14-15
            m15 = _mm256_unpacklo_epi16(m7, m4); // punpcklwd m15, m7, m4
            m7 = _mm256_unpackhi_epi16(m7, m4); // punpckhwd m7, m4
            m4 = _mm256_unpacklo_epi16(m3, m5); // punpcklwd m4, m3, m5
            m3 = _mm256_unpackhi_epi16(m3, m5); // punpckhwd m3, m5
            m5 = _mm256_unpacklo_epi16(m6, m13); // punpcklwd m5, m6, m13
            m6 = _mm256_unpackhi_epi16(m6, m13); // punpckhwd m6, m13
            m13 = _mm256_unpacklo_epi16(m12, m14); // punpcklwd m13, m12, m14
            m12 = _mm256_unpackhi_epi16(m12, m14); // punpckhwd m12, m14

            // xm15: A0-3,B0-3,C0-3,D0-3
            // xm7: E0-3,F0-3,G0-3,H0-3
            // xm4: A8-11,B8-11,C8-11,D8-11
            // xm3: E8-11,F8-11,G8-11,H8-11
            // xm5: A4-7,B4-7,C4-7,D4-7
            // xm6: E4-7,F4-7,G4-7,H4-7
            // xm13: A12-15,B12-15,C12-15,D12-15
            // xm12: E12-15,F12-15,G12-15,H12-15
            m14 = _mm256_unpacklo_epi32(m15, m5); // punpckldq m14, m15, m5
            m15 = _mm256_unpackhi_epi32(m15, m5); // punpckhdq m15, m5
            m5 = _mm256_unpacklo_epi32(m7, m6); // punpckldq m5, m7, m6
            if WD != 6 {
                m7 = _mm256_unpackhi_epi32(m7, m6); // punpckhdq m7, m6
            }
            m6 = _mm256_unpacklo_epi32(m4, m13); // punpckldq m6, m4, m13
            m4 = _mm256_unpackhi_epi32(m4, m13); // punpckhdq m4, m13
            m13 = _mm256_unpacklo_epi32(m3, m12); // punpckldq m13, m3, m12
            if WD != 6 {
                m12 = _mm256_unpackhi_epi32(m3, m12); // punpckhdq m12, m3, m12
            }
            // xm14: A0-7,B0-7
            // xm15: C0-7,D0-7
            // xm5: E0-7,F0-7
            // xm7: G0-7,H0-7
            // xm6: A8-15,B8-15
            // xm4: C8-15,D8-15
            // xm13: E8-15,F8-15
            // xm12: G8-15,H8-15
            m3 = _mm256_unpacklo_epi64(m14, m6); // punpcklqdq m3, m14, m6
            m14 = _mm256_unpackhi_epi64(m14, m6); // punpckhqdq m14, m6
            m6 = _mm256_unpackhi_epi64(m15, m4); // punpckhqdq m6, m15, m4
            m15 = _mm256_unpacklo_epi64(m15, m4); // punpcklqdq m15, m4
            m4 = _mm256_unpacklo_epi64(m5, m13); // punpcklqdq m4, m5, m13
            m13 = _mm256_unpackhi_epi64(m5, m13); // punpckhqdq m13, m5, m13
            if WD == 8 {
                m5 = _mm256_unpacklo_epi64(m7, m12); // punpcklqdq m5, m7, m12
                m12 = _mm256_unpackhi_epi64(m7, m12); // punpckhqdq m12, m7, m12

                // xm3: A0-15
                // xm14: B0-15
                // xm15: C0-15
                // xm6: D0-15
                // xm4: E0-15
                // xm13: F0-15
                // xm5: G0-15
                // xm12: H0-15
                swap!(m12, m3, m15); // SWAP 12, 3, 15
                swap!(m13, m14, m5, m4, m6); // SWAP 13, 14, 5, 4, 6

            // 3,14,15,6,4,13,5,12 -> 12,13,3,4,5,6,14,15
            } else {
                swap!(m13, m3, m14); // SWAP 13, 3, 14
                swap!(m6, m4, m15, m5); // SWAP 6, 4, 15, 5

                // 3,14,15,6,4,13 -> 13,3,4,5,6,14
            }
        } else {
            // load and 16x16 transpose. We only use 14 pixels but we'll need the
            // remainder at the end for the second transpose
            let mut xm0 = movu_ld(at(dst, -8)); // movu xm0, [dstq+strideq*0-8]
            let mut xm1 = movu_ld(at(dst, stride - 8)); // movu xm1, [dstq+strideq*1-8]
            let mut xm2 = movu_ld(at(dst, stride * 2 - 8)); // movu xm2, [dstq+strideq*2-8]
            let mut xm3 = movu_ld(at(dst, stride3 - 8)); // movu xm3, [dstq+stride3q -8]
            tmp = at(dst, stride * 4); // lea tmpq, [dstq+strideq*4]
            let mut xm4 = movu_ld(at(tmp, -8)); // movu xm4, [tmpq+strideq*0-8]
            let mut xm5 = movu_ld(at(tmp, stride - 8)); // movu xm5, [tmpq+strideq*1-8]
            let mut xm6 = movu_ld(at(tmp, stride * 2 - 8)); // movu xm6, [tmpq+strideq*2-8]
            let mut xm7 = movu_ld(at(tmp, stride3 - 8)); // movu xm7, [tmpq+stride3q -8]
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            let mut xm8 = movu_ld(at(tmp, -8)); // movu xm8, [tmpq+strideq*0-8]
            let mut xm9 = movu_ld(at(tmp, stride - 8)); // movu xm9, [tmpq+strideq*1-8]
            let mut xm10 = movu_ld(at(tmp, stride * 2 - 8)); // movu xm10, [tmpq+strideq*2-8]
            let mut xm11 = movu_ld(at(tmp, stride3 - 8)); // movu xm11, [tmpq+stride3q -8]
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            let mut xm12 = movu_ld(at(tmp, -8)); // movu xm12, [tmpq+strideq*0-8]
            let mut xm13 = movu_ld(at(tmp, stride - 8)); // movu xm13, [tmpq+strideq*1-8]
            let mut xm14 = movu_ld(at(tmp, stride * 2 - 8)); // movu xm14, [tmpq+strideq*2-8]
            let mut xm15 = movu_ld(at(tmp, stride3 - 8)); // movu xm15, [tmpq+stride3q -8]
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            m0 = vinserti128_ld(xm0, at(tmp, -8)); // vinserti128 m0, [tmpq+strideq*0-8], 1
            m1 = vinserti128_ld(xm1, at(tmp, stride - 8)); // vinserti128 m1, [tmpq+strideq*1-8], 1
            m2 = vinserti128_ld(xm2, at(tmp, stride * 2 - 8)); // vinserti128 m2, [tmpq+strideq*2-8], 1
            m3 = vinserti128_ld(xm3, at(tmp, stride3 - 8)); // vinserti128 m3, [tmpq+stride3q -8], 1
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            m4 = vinserti128_ld(xm4, at(tmp, -8)); // vinserti128 m4, [tmpq+strideq*0-8], 1
            m5 = vinserti128_ld(xm5, at(tmp, stride - 8)); // vinserti128 m5, [tmpq+strideq*1-8], 1
            m6 = vinserti128_ld(xm6, at(tmp, stride * 2 - 8)); // vinserti128 m6, [tmpq+strideq*2-8], 1
            m7 = vinserti128_ld(xm7, at(tmp, stride3 - 8)); // vinserti128 m7, [tmpq+stride3q -8], 1
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            m8 = vinserti128_ld(xm8, at(tmp, -8)); // vinserti128 m8, [tmpq+strideq*0-8], 1
            m9 = vinserti128_ld(xm9, at(tmp, stride - 8)); // vinserti128 m9, [tmpq+strideq*1-8], 1
            m10 = vinserti128_ld(xm10, at(tmp, stride * 2 - 8)); // vinserti128 m10, [tmpq+strideq*2-8], 1
            m11 = vinserti128_ld(xm11, at(tmp, stride3 - 8)); // vinserti128 m11, [tmpq+stride3q -8], 1
            tmp = at(tmp, stride * 4); // lea tmpq, [tmpq+strideq*4]
            m12 = vinserti128_ld(xm12, at(tmp, -8)); // vinserti128 m12, [tmpq+strideq*0-8], 1
            m13 = vinserti128_ld(xm13, at(tmp, stride - 8)); // vinserti128 m13, [tmpq+strideq*1-8], 1
            m14 = vinserti128_ld(xm14, at(tmp, stride * 2 - 8)); // vinserti128 m14, [tmpq+strideq*2-8], 1
            m15 = vinserti128_ld(xm15, at(tmp, stride3 - 8)); // vinserti128 m15, [tmpq+stride3q -8], 1

            // TRANSPOSE_16X16B 0, 1, [rsp+11*32]
            TRANSPOSE_16X16B!(0, 1, s11; m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15);
            s12 = m1; // mova [rsp+12*32], m1
            s13 = m2; // mova [rsp+13*32], m2
            s14 = m3; // mova [rsp+14*32], m3
            s15 = m12; // mova [rsp+15*32], m12
            s16 = m13; // mova [rsp+16*32], m13
            s17 = m14; // mova [rsp+17*32], m14
            s18 = m15; // mova [rsp+18*32], m15

            // 4,5,6,7,8,9,10,11 -> 12,13,3,4,5,6,14,15
            swap!(m12, m4, m7); // SWAP 12, 4, 7
            swap!(m13, m5, m8); // SWAP 13, 5, 8
            swap!(m3, m6, m9); // SWAP 3, 6, 9
            swap!(m10, m14); // SWAP 10, 14
            swap!(m11, m15); // SWAP 11, 15
        }
    }

    // load L/E/I/H
    if !DIR_H {
        m1 = _mm256_loadu_si256(l.cast()); // movu m1, [lq]
        m0 = _mm256_loadu_si256(at(l, l_stride).cast()); // movu m0, [lq+l_strideq]
    } else {
        let mut xm1 = movq_ld(l); // movq xm1, [lq]
        let mut xm2 = movq_ld(at(l, l_stride * 2)); // movq xm2, [lq+l_strideq*2]
        xm1 = movhps_ld(xm1, at(l, l_stride)); // movhps xm1, [lq+l_strideq]
        xm2 = movhps_ld(xm2, at(l, l_stride3)); // movhps xm2, [lq+l_stride3q]
        l = at(l, l_stride * 4); // lea lq, [lq+l_strideq*4]
        let mut xm10 = movq_ld(l); // movq xm10, [lq]
        let mut xm0 = movq_ld(at(l, l_stride * 2)); // movq xm0, [lq+l_strideq*2]
        xm10 = movhps_ld(xm10, at(l, l_stride)); // movhps xm10, [lq+l_strideq]
        xm0 = movhps_ld(xm0, at(l, l_stride3)); // movhps xm0, [lq+l_stride3q]
        l = at(l, l_stride * 4); // lea lq, [lq+l_strideq*4]
        m1 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm1), xm10); // vinserti128 m1, xm10, 1
        m2 = _mm256_inserti128_si256::<1>(_mm256_zextsi128_si256(xm2), xm0); // vinserti128 m2, xm0, 1
        m0 = shufps::<Q3131>(m1, m2); // shufps m0, m1, m2, q3131
        m1 = shufps::<Q2020>(m1, m2); // shufps m1, m2, q2020
    }
    m2 = _mm256_setzero_si256(); // pxor m2, m2
    m10 = _mm256_cmpeq_epi8(m2, m0); // pcmpeqb m10, m2, m0
    m1 = _mm256_and_si256(m1, m10); // pand m1, m10
    m0 = _mm256_or_si256(m0, m1); // por m0, m1 ; l[x][] ? l[x][] : l[x-stride][]
    m0 = _mm256_shuffle_epi8(m0, mem(&pb_4x1_4x5_4x9_4x13)); // pshufb m0, [pb_4x1_4x5_4x9_4x13] ; l[x][1]
    m10 = _mm256_cmpeq_epi8(m2, m0); // pcmpeqb m10, m2, m0 ; !L
    m2 = _mm256_srl_epi64(m0, _mm_loadu_si128((lut as *const u8).add(128).cast())); // psrlq m2, m0, [lutq+128]
    m2 = _mm256_and_si256(m2, mem(&pb_63)); // pand m2, [pb_63]
    m1 = _mm256_set1_epi8((lut as *const i8).add(136).read()); // vpbroadcastb m1, [lutq+136]
    m2 = _mm256_min_epu8(m2, m1); // pminub m2, m1
    m2 = _mm256_max_epu8(m2, mem(&pb_1)); // pmaxub m2, [pb_1] ; I
    m1 = _mm256_and_si256(m0, mem(&pb_240)); // pand m1, m0, [pb_240]
    m1 = _mm256_srli_epi64::<4>(m1); // psrlq m1, 4 ; H
    m0 = _mm256_add_epi8(m0, mem(&pb_2)); // paddb m0, [pb_2]
    m0 = _mm256_add_epi8(m0, m0); // paddb m0, m0
    m0 = _mm256_add_epi8(m0, m2); // paddb m0, m2 ; E
    m1 = _mm256_xor_si256(m1, mem(&pb_128)); // pxor m1, [pb_128]
    m2 = _mm256_xor_si256(m2, mem(&pb_128)); // pxor m2, [pb_128]
    m0 = _mm256_xor_si256(m0, mem(&pb_128)); // pxor m0, [pb_128]

    ABSSUB!(m8, m3, m4, m9); // abs(p1-p0)
    m8 = _mm256_max_epu8(m8, m10); // pmaxub m8, m10
    ABSSUB!(m9, m5, m6, m10); // abs(q1-q0)
    m8 = _mm256_max_epu8(m8, m9); // pmaxub m8, m9
    if WD == 4 {
        m8 = _mm256_xor_si256(m8, mem(&pb_128)); // pxor m8, [pb_128]
        m7 = _mm256_cmpgt_epi8(m8, m1); // pcmpgtb m7, m8, m1 ; hev
    } else {
        m7 = _mm256_xor_si256(m8, mem(&pb_128)); // pxor m7, m8, [pb_128]
        m7 = _mm256_cmpgt_epi8(m7, m1); // pcmpgtb m7, m1 ; hev

        if WD == 6 {
            ABSSUB!(m9, m13, m4, m10); // abs(p2-p0)
            m9 = _mm256_max_epu8(m9, m8); // pmaxub m9, m8
        } else {
            ABSSUB!(m9, m12, m4, m10); // abs(p3-p0)
            m9 = _mm256_max_epu8(m9, m8); // pmaxub m9, m8
            ABSSUB!(m10, m13, m4, m11); // abs(p2-p0)
            m9 = _mm256_max_epu8(m9, m10); // pmaxub m9, m10
        }
        ABSSUB!(m10, m5, m14, m11); // abs(q2-q0)
        m9 = _mm256_max_epu8(m9, m10); // pmaxub m9, m10
        if WD != 6 {
            ABSSUB!(m10, m5, m15, m11); // abs(q3-q0)
            m9 = _mm256_max_epu8(m9, m10); // pmaxub m9, m10
        }
        m9 = _mm256_xor_si256(m9, mem(&pb_128)); // pxor m9, [pb_128]
        m9 = _mm256_cmpgt_epi8(m9, mem(&pb_129)); // pcmpgtb m9, [pb_129] ; !flat8in

        if WD == 6 {
            ABSSUB!(m10, m13, m3, m1); // abs(p2-p1)
        } else {
            ABSSUB!(m10, m12, m13, m11); // abs(p3-p2)
            ABSSUB!(m11, m13, m3, m1); // abs(p2-p1)
            m10 = _mm256_max_epu8(m10, m11); // pmaxub m10, m11
            ABSSUB!(m11, m14, m15, m1); // abs(q3-q2)
            m10 = _mm256_max_epu8(m10, m11); // pmaxub m10, m11
        }
        ABSSUB!(m11, m14, m6, m1); // abs(q2-q1)
        m10 = _mm256_max_epu8(m10, m11); // pmaxub m10, m11
        if WD == 16 {
            m11 = vpbroadcastd_ld(mask.add(8)); // vpbroadcastd m11, [maskq+8]
            m1 = vpbroadcastd_ld(mask.add(4)); // vpbroadcastd m1, [maskq+4]
            m11 = _mm256_or_si256(m11, m1); // por m11, m1
            m11 = _mm256_and_si256(m11, mem(&pb_mask)); // pand m11, [pb_mask]
            m11 = _mm256_cmpeq_epi32(m11, mem(&pb_mask)); // pcmpeqd m11, [pb_mask]
            m10 = _mm256_and_si256(m10, m11); // pand m10, m11
        } else {
            m11 = vpbroadcastd_ld(mask.add(4)); // vpbroadcastd m11, [maskq+4]
            m11 = _mm256_and_si256(m11, mem(&pb_mask)); // pand m11, [pb_mask]
            m11 = _mm256_cmpeq_epi32(m11, mem(&pb_mask)); // pcmpeqd m11, [pb_mask]
            m10 = _mm256_and_si256(m10, m11); // pand m10, m11 ; only apply fm-wide to wd>4 blocks
        }
        m8 = _mm256_max_epu8(m8, m10); // pmaxub m8, m10

        m8 = _mm256_xor_si256(m8, mem(&pb_128)); // pxor m8, [pb_128]
    }
    m8 = _mm256_cmpgt_epi8(m8, m2); // pcmpgtb m8, m2

    ABSSUB!(m10, m3, m6, m11); // abs(p1-q1)
    ABSSUB!(m11, m4, m5, m2); // abs(p0-q0)
    m11 = _mm256_adds_epu8(m11, m11); // paddusb m11, m11
    m10 = _mm256_and_si256(m10, mem(&pb_254)); // pand m10, [pb_254]
    m10 = _mm256_srli_epi64::<1>(m10); // psrlq m10, 1
    m10 = _mm256_adds_epu8(m10, m11); // paddusb m10, m11 ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    m10 = _mm256_xor_si256(m10, mem(&pb_128)); // pxor m10, [pb_128]
    m10 = _mm256_cmpgt_epi8(m10, m0); // pcmpgtb m10, m0 ; abs(p0-q0)*2+(abs(p1-q1)>>1) > E
    m8 = _mm256_or_si256(m8, m10); // por m8, m10

    if WD == 16 {
        if !DIR_H {
            tmp = at(dst, mstride * 8); // lea tmpq, [dstq+mstrideq*8]
            m0 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m0, [tmpq+strideq*1]
        } else {
            m0 = s12; // mova m0, [rsp+12*32]
        }
        ABSSUB!(m1, m0, m4, m2);
        if !DIR_H {
            m0 = _mm256_loadu_si256(at(tmp, stride * 2).cast()); // mova m0, [tmpq+strideq*2]
        } else {
            m0 = s13; // mova m0, [rsp+13*32]
        }
        ABSSUB!(m2, m0, m4, m10);
        m1 = _mm256_max_epu8(m1, m2); // pmaxub m1, m2
        if !DIR_H {
            m0 = _mm256_loadu_si256(at(tmp, stride3).cast()); // mova m0, [tmpq+stride3q]
        } else {
            m0 = s14; // mova m0, [rsp+14*32]
        }
        ABSSUB!(m2, m0, m4, m10);
        m1 = _mm256_max_epu8(m1, m2); // pmaxub m1, m2
        if !DIR_H {
            tmp = at(dst, stride * 4); // lea tmpq, [dstq+strideq*4]
            m0 = _mm256_loadu_si256(tmp.cast()); // mova m0, [tmpq+strideq*0]
        } else {
            m0 = s15; // mova m0, [rsp+15*32]
        }
        ABSSUB!(m2, m0, m5, m10);
        m1 = _mm256_max_epu8(m1, m2); // pmaxub m1, m2
        if !DIR_H {
            m0 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m0, [tmpq+strideq*1]
        } else {
            m0 = s16; // mova m0, [rsp+16*32]
        }
        ABSSUB!(m2, m0, m5, m10);
        m1 = _mm256_max_epu8(m1, m2); // pmaxub m1, m2
        if !DIR_H {
            m0 = _mm256_loadu_si256(at(tmp, stride * 2).cast()); // mova m0, [tmpq+strideq*2]
        } else {
            m0 = s17; // mova m0, [rsp+17*32]
        }
        ABSSUB!(m2, m0, m5, m10);
        m1 = _mm256_max_epu8(m1, m2); // pmaxub m1, m2
        m1 = _mm256_xor_si256(m1, mem(&pb_128)); // pxor m1, [pb_128]
        m1 = _mm256_cmpgt_epi8(m1, mem(&pb_129)); // pcmpgtb m1, [pb_129] ; !flat8out
        m1 = _mm256_or_si256(m1, m9); // por m1, m9 ; !flat8in | !flat8out
        m2 = vpbroadcastd_ld(mask.add(8)); // vpbroadcastd m2, [maskq+8]
        m10 = _mm256_and_si256(m2, mem(&pb_mask)); // pand m10, m2, [pb_mask]
        m10 = _mm256_cmpeq_epi32(m10, mem(&pb_mask)); // pcmpeqd m10, [pb_mask]
        m1 = _mm256_andnot_si256(m1, m10); // pandn m1, m10 ; flat16
        m1 = _mm256_andnot_si256(m8, m1); // pandn m1, m8, m1 ; flat16 & fm

        m10 = vpbroadcastd_ld(mask.add(4)); // vpbroadcastd m10, [maskq+4]
        m10 = _mm256_or_si256(m10, m2); // por m10, m2
        m2 = _mm256_and_si256(m10, mem(&pb_mask)); // pand m2, m10, [pb_mask]
        m2 = _mm256_cmpeq_epi32(m2, mem(&pb_mask)); // pcmpeqd m2, [pb_mask]
        m9 = _mm256_andnot_si256(m9, m2); // pandn m9, m2 ; flat8in
        m9 = _mm256_andnot_si256(m8, m9); // pandn m9, m8, m9
        m2 = vpbroadcastd_ld(mask); // vpbroadcastd m2, [maskq+0]
        m2 = _mm256_or_si256(m2, m10); // por m2, m10
        m2 = _mm256_and_si256(m2, mem(&pb_mask)); // pand m2, [pb_mask]
        m2 = _mm256_cmpeq_epi32(m2, mem(&pb_mask)); // pcmpeqd m2, [pb_mask]
        m8 = _mm256_andnot_si256(m8, m2); // pandn m8, m2
        m8 = _mm256_andnot_si256(m9, m8); // pandn m8, m9, m8 ; fm & !flat8 & !flat16
        m9 = _mm256_andnot_si256(m1, m9); // pandn m9, m1, m9 ; flat8 & !flat16
    } else if WD != 4 {
        m0 = vpbroadcastd_ld(mask.add(4)); // vpbroadcastd m0, [maskq+4]
        m2 = _mm256_and_si256(m0, mem(&pb_mask)); // pand m2, m0, [pb_mask]
        m2 = _mm256_cmpeq_epi32(m2, mem(&pb_mask)); // pcmpeqd m2, [pb_mask]
        m9 = _mm256_andnot_si256(m9, m2); // pandn m9, m2
        m9 = _mm256_andnot_si256(m8, m9); // pandn m9, m8, m9 ; flat8 & fm
        m2 = vpbroadcastd_ld(mask); // vpbroadcastd m2, [maskq+0]
        m0 = _mm256_or_si256(m0, m2); // por m0, m2
        m0 = _mm256_and_si256(m0, mem(&pb_mask)); // pand m0, [pb_mask]
        m0 = _mm256_cmpeq_epi32(m0, mem(&pb_mask)); // pcmpeqd m0, [pb_mask]
        m8 = _mm256_andnot_si256(m8, m0); // pandn m8, m0
        m8 = _mm256_andnot_si256(m9, m8); // pandn m8, m9, m8 ; fm & !flat8
    } else {
        m0 = vpbroadcastd_ld(mask); // vpbroadcastd m0, [maskq+0]
        m0 = _mm256_and_si256(m0, mem(&pb_mask)); // pand m0, [pb_mask]
        m0 = _mm256_cmpeq_epi32(m0, mem(&pb_mask)); // pcmpeqd m0, [pb_mask]
        m8 = _mm256_andnot_si256(m8, m0); // pandn m8, m0 ; fm
    }

    // short filter

    m3 = _mm256_xor_si256(m3, mem(&pb_128)); // pxor m3, [pb_128]
    m6 = _mm256_xor_si256(m6, mem(&pb_128)); // pxor m6, [pb_128]
    m10 = _mm256_subs_epi8(m3, m6); // psubsb m10, m3, m6 ; iclip_diff(p1-q1)
    m10 = _mm256_and_si256(m10, m7); // pand m10, m7 ; f=iclip_diff(p1-q1)&hev
    m4 = _mm256_xor_si256(m4, mem(&pb_128)); // pxor m4, [pb_128]
    m5 = _mm256_xor_si256(m5, mem(&pb_128)); // pxor m5, [pb_128]
    m11 = _mm256_subs_epi8(m5, m4); // psubsb m11, m5, m4
    m10 = _mm256_adds_epi8(m10, m11); // paddsb m10, m11
    m10 = _mm256_adds_epi8(m10, m11); // paddsb m10, m11
    m10 = _mm256_adds_epi8(m10, m11); // paddsb m10, m11 ; f=iclip_diff(3*(q0-p0)+f)
    m8 = _mm256_and_si256(m8, m10); // pand m8, m10 ; f&=fm
    m10 = _mm256_adds_epi8(m8, mem(&pb_3)); // paddsb m10, m8, [pb_3]
    m8 = _mm256_adds_epi8(m8, mem(&pb_4)); // paddsb m8, [pb_4]
    m10 = _mm256_and_si256(m10, mem(&pb_248)); // pand m10, [pb_248]
    m8 = _mm256_and_si256(m8, mem(&pb_248)); // pand m8, [pb_248]
    m10 = _mm256_srli_epi64::<3>(m10); // psrlq m10, 3
    m8 = _mm256_srli_epi64::<3>(m8); // psrlq m8, 3
    m10 = _mm256_xor_si256(m10, mem(&pb_16)); // pxor m10, [pb_16]
    m8 = _mm256_xor_si256(m8, mem(&pb_16)); // pxor m8, [pb_16]
    m10 = _mm256_sub_epi8(m10, mem(&pb_16)); // psubb m10, [pb_16] ; f2
    m8 = _mm256_sub_epi8(m8, mem(&pb_16)); // psubb m8, [pb_16] ; f1
    m4 = _mm256_adds_epi8(m4, m10); // paddsb m4, m10
    m5 = _mm256_subs_epi8(m5, m8); // psubsb m5, m8
    m4 = _mm256_xor_si256(m4, mem(&pb_128)); // pxor m4, [pb_128]
    m5 = _mm256_xor_si256(m5, mem(&pb_128)); // pxor m5, [pb_128]

    m8 = _mm256_xor_si256(m8, mem(&pb_128)); // pxor m8, [pb_128]
    m10 = _mm256_setzero_si256(); // pxor m10, m10
    m8 = _mm256_avg_epu8(m8, m10); // pavgb m8, m10 ; f=(f1+1)>>1
    m8 = _mm256_sub_epi8(m8, mem(&pb_64)); // psubb m8, [pb_64]
    m8 = _mm256_andnot_si256(m7, m8); // pandn m8, m7, m8 ; f&=!hev
    m3 = _mm256_adds_epi8(m3, m8); // paddsb m3, m8
    m6 = _mm256_subs_epi8(m6, m8); // psubsb m6, m8
    m3 = _mm256_xor_si256(m3, mem(&pb_128)); // pxor m3, [pb_128]
    m6 = _mm256_xor_si256(m6, mem(&pb_128)); // pxor m6, [pb_128]

    if WD == 16 {
        // flat16 filter
        if !DIR_H {
            tmp = at(dst, mstride * 8); // lea tmpq, [dstq+mstrideq*8]
            m0 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m0, [tmpq+strideq*1] ; p6
            m2 = _mm256_loadu_si256(at(tmp, stride * 2).cast()); // mova m2, [tmpq+strideq*2] ; p5
            m7 = _mm256_loadu_si256(at(tmp, stride3).cast()); // mova m7, [tmpq+stride3q] ; p4
        } else {
            m0 = s12; // mova m0, [rsp+12*32]
            m2 = s13; // mova m2, [rsp+13*32]
            m7 = s14; // mova m7, [rsp+14*32]
        }

        s0 = m9; // mova [rsp+0*32], m9
        s1 = m14; // mova [rsp+1*32], m14
        s2 = m15; // mova [rsp+2*32], m15

        // p6*7+p5*2+p4*2+p3+p2+p1+p0+q0 [p5/p4/p2/p1/p0/q0][p6/p3] A
        // write -6
        m14 = _mm256_unpacklo_epi8(m0, m12); // punpcklbw m14, m0, m12
        m15 = _mm256_unpackhi_epi8(m0, m12); // punpckhbw m15, m0, m12
        m10 = _mm256_maddubs_epi16(m14, mem(&pb_7_1)); // pmaddubsw m10, m14, [pb_7_1]
        m11 = _mm256_maddubs_epi16(m15, mem(&pb_7_1)); // pmaddubsw m11, m15, [pb_7_1] ; p6*7+p3
        m8 = _mm256_unpacklo_epi8(m2, m7); // punpcklbw m8, m2, m7
        m9 = _mm256_unpackhi_epi8(m2, m7); // punpckhbw m9, m2, m7
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_2)); // pmaddubsw m8, [pb_2]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_2)); // pmaddubsw m9, [pb_2]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*7+p5*2+p4*2+p3
        m8 = _mm256_unpacklo_epi8(m13, m3); // punpcklbw m8, m13, m3
        m9 = _mm256_unpackhi_epi8(m13, m3); // punpckhbw m9, m13, m3
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_1)); // pmaddubsw m8, [pb_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_1)); // pmaddubsw m9, [pb_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*7+p5*2+p4*2+p3+p2+p1
        m8 = _mm256_unpacklo_epi8(m4, m5); // punpcklbw m8, m4, m5
        m9 = _mm256_unpackhi_epi8(m4, m5); // punpckhbw m9, m4, m5
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_1)); // pmaddubsw m8, [pb_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_1)); // pmaddubsw m9, [pb_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_and_si256(m8, m1); // pand m8, m1
        m9 = _mm256_andnot_si256(m1, m2); // pandn m9, m1, m2
        m8 = _mm256_or_si256(m8, m9); // por m8, m9
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride * 2).cast(), m8); // mova [tmpq+strideq*2], m8 ; p5
        } else {
            s13 = m8; // mova [rsp+13*32], m8
        }

        // sub p6*2, add p3/q1 [reuse p6/p3 from A][-p6,+q1|save] B
        // write -5
        m14 = _mm256_maddubs_epi16(m14, mem(&pb_m1_1)); // pmaddubsw m14, [pb_m1_1]
        m15 = _mm256_maddubs_epi16(m15, mem(&pb_m1_1)); // pmaddubsw m15, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m14); // paddw m10, m14
        m11 = _mm256_add_epi16(m11, m15); // paddw m11, m15 ; p6*6+p5*2+p4*2+p3*2+p2+p1+p0+q0
        m8 = _mm256_unpacklo_epi8(m0, m6); // punpcklbw m8, m0, m6
        m9 = _mm256_unpackhi_epi8(m0, m6); // punpckhbw m9, m0, m6
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        s3 = m8; // mova [rsp+3*32], m8
        s4 = m9; // mova [rsp+4*32], m9
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*5+p5*2+p4*2+p3*2+p2+p1+p0+q0+q1
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_blendv_epi8(m7, m8, m1); // vpblendvb m8, m7, m8, m1
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride3).cast(), m8); // mova [tmpq+stride3q], m8 ; p4
        } else {
            s14 = m8; // mova [rsp+14*32], m8
        }

        // sub p6/p5, add p2/q2 [-p6,+p2][-p5,+q2|save] C
        // write -4
        m14 = s1; // mova m14, [rsp+1*32]
        m8 = _mm256_unpacklo_epi8(m0, m13); // punpcklbw m8, m0, m13
        m9 = _mm256_unpackhi_epi8(m0, m13); // punpckhbw m9, m0, m13
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*4+p5*2+p4*2+p3*2+p2*2+p1+p0+q0+q1
        m8 = _mm256_unpacklo_epi8(m2, m14); // punpcklbw m8, m2, m14
        m2 = _mm256_unpackhi_epi8(m2, m14); // punpckhbw m2, m14
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m2 = _mm256_maddubs_epi16(m2, mem(&pb_m1_1)); // pmaddubsw m2, [pb_m1_1]
        s1 = m8; // mova [rsp+1*32], m8
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m2); // paddw m11, m2 ; p6*4+p5+p4*2+p3*2+p2*2+p1+p0+q0+q1+q2
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_blendv_epi8(m12, m8, m1); // vpblendvb m8, m12, m8, m1
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride * 4).cast(), m8); // mova [tmpq+strideq*4], m8 ; p3
        } else {
            s19 = m8; // mova [rsp+19*32], m8
        }

        // sub p6/p4, add p1/q3 [-p6,+p1][-p4,+q3|save] D
        // write -3
        m15 = s2; // mova m15, [rsp+2*32]
        m8 = _mm256_unpacklo_epi8(m0, m3); // punpcklbw m8, m0, m3
        m9 = _mm256_unpackhi_epi8(m0, m3); // punpckhbw m9, m0, m3
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*3+p5+p4*2+p3*2+p2*2+p1*2+p0+q0+q1+q2
        m8 = _mm256_unpacklo_epi8(m7, m15); // punpcklbw m8, m7, m15
        m7 = _mm256_unpackhi_epi8(m7, m15); // punpckhbw m7, m15
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m7 = _mm256_maddubs_epi16(m7, mem(&pb_m1_1)); // pmaddubsw m7, [pb_m1_1]
        s2 = m8; // mova [rsp+2*32], m8
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m7); // paddw m11, m7 ; p6*3+p5+p4+p3*2+p2*2+p1*2+p0+q0+q1+q2+q3
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_blendv_epi8(m13, m8, m1); // vpblendvb m8, m13, m8, m1
        s6 = m8; // mova [rsp+6*32], m8 ; don't clobber p2/m13 since we need it in F

        // sub p6/p3, add p0/q4 [-p6,+p0][-p3,+q4|save] E
        // write -2
        if !DIR_H {
            tmp = at(dst, stride * 4); // lea tmpq, [dstq+strideq*4]
        }
        m8 = _mm256_unpacklo_epi8(m0, m4); // punpcklbw m8, m0, m4
        m9 = _mm256_unpackhi_epi8(m0, m4); // punpckhbw m9, m0, m4
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*2+p5+p4+p3*2+p2*2+p1*2+p0*2+q0+q1+q2+q3
        if !DIR_H {
            m9 = _mm256_loadu_si256(tmp.cast()); // mova m9, [tmpq+strideq*0] ; q4
        } else {
            m9 = s15; // mova m9, [rsp+15*32]
        }
        m8 = _mm256_unpacklo_epi8(m12, m9); // punpcklbw m8, m12, m9
        m9 = _mm256_unpackhi_epi8(m12, m9); // punpckhbw m9, m12, m9
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        s7 = m8; // mova [rsp+7*32], m8
        s5 = m9; // mova [rsp+5*32], m9
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6*2+p5+p4+p3+p2*2+p1*2+p0*2+q0+q1+q2+q3+q4
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_blendv_epi8(m3, m8, m1); // vpblendvb m8, m3, m8, m1
        s8 = m8; // mova [rsp+8*32], m8 ; don't clobber p1/m3 since we need it in G

        // sub p6/p2, add q0/q5 [-p6,+q0][-p2,+q5|save] F
        // write -1
        if !DIR_H {
            m9 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m9, [tmpq+strideq*1] ; q5
        } else {
            m9 = s16; // mova m9, [rsp+16*32]
        }
        m8 = _mm256_unpacklo_epi8(m0, m5); // punpcklbw m8, m0, m5
        m0 = _mm256_unpackhi_epi8(m0, m5); // punpckhbw m0, m5
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m0 = _mm256_maddubs_epi16(m0, mem(&pb_m1_1)); // pmaddubsw m0, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m0); // paddw m11, m0 ; p6+p5+p4+p3+p2*2+p1*2+p0*2+q0*2+q1+q2+q3+q4
        m0 = _mm256_unpacklo_epi8(m13, m9); // punpcklbw m0, m13, m9
        m9 = _mm256_unpackhi_epi8(m13, m9); // punpckhbw m9, m13, m9
        m13 = s6; // mova m13, [rsp+6*32]
        m0 = _mm256_maddubs_epi16(m0, mem(&pb_m1_1)); // pmaddubsw m0, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        s9 = m0; // mova [rsp+ 9*32], m0
        s10 = m9; // mova [rsp+10*32], m9
        m10 = _mm256_add_epi16(m10, m0); // paddw m10, m0
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p6+p5+p4+p3+p2+p1*2+p0*2+q0*2+q1+q2+q3+q4+q5
        m0 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m0, m10, [pw_2048]
        m8 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m8, m11, [pw_2048]
        m0 = _mm256_packus_epi16(m0, m8); // packuswb m0, m8
        m0 = _mm256_blendv_epi8(m4, m0, m1); // vpblendvb m0, m4, m0, m1
        s6 = m0; // mova [rsp+6*32], m0 ; don't clobber p0/m4 since we need it in H

        // sub p6/p1, add q1/q6 [reuse -p6,+q1 from B][-p1,+q6|save] G
        // write +0
        if !DIR_H {
            m0 = _mm256_loadu_si256(at(tmp, stride * 2).cast()); // mova m0, [tmpq+strideq*2] ; q6
        } else {
            m0 = s17; // mova m0, [rsp+17*32]
        }
        m10 = _mm256_add_epi16(m10, s3); // paddw m10, [rsp+3*32]
        m11 = _mm256_add_epi16(m11, s4); // paddw m11, [rsp+4*32] ; p5+p4+p3+p2+p1*2+p0*2+q0*2+q1*2+q2+q3+q4+q5
        m8 = _mm256_unpacklo_epi8(m3, m0); // punpcklbw m8, m3, m0
        m9 = _mm256_unpackhi_epi8(m3, m0); // punpckhbw m9, m3, m0
        m3 = s8; // mova m3, [rsp+8*32]
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        s3 = m8; // mova [rsp+3*32], m8
        s4 = m9; // mova [rsp+4*32], m9
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p5+p4+p3+p2+p1+p0*2+q0*2+q1*2+q2+q3+q4+q5+q6
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_blendv_epi8(m5, m8, m1); // vpblendvb m8, m5, m8, m1
        s8 = m8; // mova [rsp+8*32], m8 ; don't clobber q0/m5 since we need it in I

        // sub p5/p0, add q2/q6 [reuse -p5,+q2 from C][-p0,+q6] H
        // write +1
        m10 = _mm256_add_epi16(m10, s1); // paddw m10, [rsp+1*32]
        m11 = _mm256_add_epi16(m11, m2); // paddw m11, m2 ; p4+p3+p2+p1+p0*2+q0*2+q1*2+q2*2+q3+q4+q5+q6
        m8 = _mm256_unpacklo_epi8(m4, m0); // punpcklbw m8, m4, m0
        m2 = _mm256_unpackhi_epi8(m4, m0); // punpckhbw m2, m4, m0
        m4 = s6; // mova m4, [rsp+6*32]
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m2 = _mm256_maddubs_epi16(m2, mem(&pb_m1_1)); // pmaddubsw m2, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m2); // paddw m11, m2 ; p4+p3+p2+p1+p0+q0*2+q1*2+q2*2+q3+q4+q5+q6*2
        m2 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m2, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m2 = _mm256_packus_epi16(m2, m9); // packuswb m2, m9
        m2 = _mm256_blendv_epi8(m6, m2, m1); // vpblendvb m2, m6, m2, m1 ; don't clobber q1/m6 since we need it in K

        // sub p4/q0, add q3/q6 [reuse -p4,+q3 from D][-q0,+q6] I
        // write +2
        m10 = _mm256_add_epi16(m10, s2); // paddw m10, [rsp+2*32]
        m11 = _mm256_add_epi16(m11, m7); // paddw m11, m7 ; p3+p2+p1+p0+q0*2+q1*2+q2*2+q3*2+q4+q5+q6*2
        m8 = _mm256_unpacklo_epi8(m5, m0); // punpcklbw m8, m5, m0
        m9 = _mm256_unpackhi_epi8(m5, m0); // punpckhbw m9, m5, m0
        m5 = s8; // mova m5, [rsp+8*32]
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p3+p2+p1+p0+q0+q1*2+q2*2+q3*2+q4+q5+q6*3
        m7 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m7, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m7 = _mm256_packus_epi16(m7, m9); // packuswb m7, m9
        m7 = _mm256_blendv_epi8(m14, m7, m1); // vpblendvb m7, m14, m7, m1 ; don't clobber q2/m14 since we need it in K

        // sub p3/q1, add q4/q6 [reuse -p3,+q4 from E][-q1,+q6] J
        // write +3
        m10 = _mm256_add_epi16(m10, s7); // paddw m10, [rsp+7*32]
        m11 = _mm256_add_epi16(m11, s5); // paddw m11, [rsp+5*32] ; p2+p1+p0+q0+q1*2+q2*2+q3*2+q4*2+q5+q6*3
        m8 = _mm256_unpacklo_epi8(m6, m0); // punpcklbw m8, m6, m0
        m9 = _mm256_unpackhi_epi8(m6, m0); // punpckhbw m9, m6, m0
        swap!(m2, m6); // SWAP 2, 6
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p2+p1+p0+q0+q1+q2*2+q3*2+q4*2+q5+q6*4
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        m8 = _mm256_blendv_epi8(m15, m8, m1); // vpblendvb m8, m15, m8, m1
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, mstride).cast(), m8); // mova [tmpq+mstrideq], m8 ; q3
        } else {
            s20 = m8; // mova [rsp+20*32], m8
        }

        // sub p2/q2, add q5/q6 [reuse -p2,+q5 from F][-q2,+q6] K
        // write +4
        m10 = _mm256_add_epi16(m10, s9); // paddw m10, [rsp+ 9*32]
        m11 = _mm256_add_epi16(m11, s10); // paddw m11, [rsp+10*32] ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
        m8 = _mm256_unpacklo_epi8(m14, m0); // punpcklbw m8, m14, m0
        m9 = _mm256_unpackhi_epi8(m14, m0); // punpckhbw m9, m14, m0
        swap!(m14, m7); // SWAP 14, 7
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
        m8 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m8, m10, [pw_2048]
        m9 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m9, m11, [pw_2048]
        m8 = _mm256_packus_epi16(m8, m9); // packuswb m8, m9
        if !DIR_H {
            m9 = _mm256_loadu_si256(tmp.cast()); // mova m9, [tmpq+strideq*0]
        } else {
            m9 = s15; // mova m9, [rsp+15*32]
        }
        m8 = _mm256_blendv_epi8(m9, m8, m1); // vpblendvb m8, m9, m8, m1
        if !DIR_H {
            _mm256_storeu_si256(tmp.cast(), m8); // mova [tmpq+strideq*0], m8 ; q4
        } else {
            s15 = m8; // mova [rsp+15*32], m8
        }

        // sub p1/q3, add q6*2 [reuse -p1,+q6 from G][-q3,+q6] L
        // write +5
        m10 = _mm256_add_epi16(m10, s3); // paddw m10, [rsp+3*32]
        m11 = _mm256_add_epi16(m11, s4); // paddw m11, [rsp+4*32] ; p1+p0+q0+q1+q2*2+q3*2+q4*2+q5*2+q6*4
        m8 = _mm256_unpacklo_epi8(m15, m0); // punpcklbw m8, m15, m0
        m9 = _mm256_unpackhi_epi8(m15, m0); // punpckhbw m9, m15, m0
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m9 = _mm256_maddubs_epi16(m9, mem(&pb_m1_1)); // pmaddubsw m9, [pb_m1_1]
        m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
        m11 = _mm256_add_epi16(m11, m9); // paddw m11, m9 ; p1+p0+q0+q1+q2+q3*2+q4*2+q5*2+q6*5
        m10 = _mm256_mulhrs_epi16(m10, mem(&pw_2048)); // pmulhrsw m10, [pw_2048]
        m11 = _mm256_mulhrs_epi16(m11, mem(&pw_2048)); // pmulhrsw m11, [pw_2048]
        m10 = _mm256_packus_epi16(m10, m11); // packuswb m10, m11
        if !DIR_H {
            m11 = _mm256_loadu_si256(at(tmp, stride).cast()); // mova m11, [tmpq+strideq*1]
        } else {
            m11 = s16; // mova m11, [rsp+16*32]
        }
        m10 = _mm256_blendv_epi8(m11, m10, m1); // vpblendvb m10, m11, m10, m1
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride).cast(), m10); // mova [tmpq+strideq*1], m10 ; q5
        } else {
            s16 = m10; // mova [rsp+16*32], m10
        }

        m9 = s0; // mova m9, [rsp+0*32]
        if !DIR_H {
            tmp = at(dst, mstride * 4); // lea tmpq, [dstq+mstrideq*4]
        }
    }
    if WD >= 8 {
        // flat8 filter
        m0 = _mm256_unpacklo_epi8(m12, m3); // punpcklbw m0, m12, m3
        m1 = _mm256_unpackhi_epi8(m12, m3); // punpckhbw m1, m12, m3
        m2 = _mm256_maddubs_epi16(m0, mem(&pb_3_1)); // pmaddubsw m2, m0, [pb_3_1]
        m7 = _mm256_maddubs_epi16(m1, mem(&pb_3_1)); // pmaddubsw m7, m1, [pb_3_1] ; 3 * p3 + p1
        m8 = _mm256_unpacklo_epi8(m13, m4); // punpcklbw m8, m13, m4
        m11 = _mm256_unpackhi_epi8(m13, m4); // punpckhbw m11, m13, m4
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_2_1)); // pmaddubsw m8, [pb_2_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_2_1)); // pmaddubsw m11, [pb_2_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m11); // paddw m7, m11 ; 3 * p3 + 2 * p2 + p1 + p0
        m8 = _mm256_unpacklo_epi8(m5, mem(&pb_4)); // punpcklbw m8, m5, [pb_4]
        m11 = _mm256_unpackhi_epi8(m5, mem(&pb_4)); // punpckhbw m11, m5, [pb_4]
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_1)); // pmaddubsw m8, [pb_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_1)); // pmaddubsw m11, [pb_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m11); // paddw m7, m11 ; 3 * p3 + 2 * p2 + p1 + p0 + q0 + 4
        m8 = _mm256_srli_epi16::<3>(m2); // psrlw m8, m2, 3
        m11 = _mm256_srli_epi16::<3>(m7); // psrlw m11, m7, 3
        m8 = _mm256_packus_epi16(m8, m11); // packuswb m8, m11
        m10 = _mm256_blendv_epi8(m13, m8, m9); // vpblendvb m10, m13, m8, m9 ; p2
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride).cast(), m10); // mova [tmpq+strideq*1], m10 ; p2
        }

        m8 = _mm256_maddubs_epi16(m0, mem(&pb_m1_1)); // pmaddubsw m8, m0, [pb_m1_1]
        m11 = _mm256_maddubs_epi16(m1, mem(&pb_m1_1)); // pmaddubsw m11, m1, [pb_m1_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m11); // paddw m7, m11
        m8 = _mm256_unpacklo_epi8(m13, m6); // punpcklbw m8, m13, m6
        m11 = _mm256_unpackhi_epi8(m13, m6); // punpckhbw m11, m13, m6
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_m1_1)); // pmaddubsw m11, [pb_m1_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m11); // paddw m7, m11 ; 2 * p3 + p2 + 2 * p1 + p0 + q0 + q1 + 4
        m8 = _mm256_srli_epi16::<3>(m2); // psrlw m8, m2, 3
        m11 = _mm256_srli_epi16::<3>(m7); // psrlw m11, m7, 3
        m8 = _mm256_packus_epi16(m8, m11); // packuswb m8, m11
        m8 = _mm256_blendv_epi8(m3, m8, m9); // vpblendvb m8, m3, m8, m9 ; p1
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride * 2).cast(), m8); // mova [tmpq+strideq*2], m8 ; p1
        } else {
            s0 = m8; // mova [rsp+0*32], m8
        }

        m0 = _mm256_maddubs_epi16(m0, mem(&pb_1)); // pmaddubsw m0, [pb_1]
        m1 = _mm256_maddubs_epi16(m1, mem(&pb_1)); // pmaddubsw m1, [pb_1]
        m2 = _mm256_sub_epi16(m2, m0); // psubw m2, m0
        m7 = _mm256_sub_epi16(m7, m1); // psubw m7, m1
        m8 = _mm256_unpacklo_epi8(m4, m14); // punpcklbw m8, m4, m14
        m11 = _mm256_unpackhi_epi8(m4, m14); // punpckhbw m11, m4, m14
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_1)); // pmaddubsw m8, [pb_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_1)); // pmaddubsw m11, [pb_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m11); // paddw m7, m11 ; p3 + p2 + p1 + 2 * p0 + q0 + q1 + q2 + 4
        m8 = _mm256_srli_epi16::<3>(m2); // psrlw m8, m2, 3
        m11 = _mm256_srli_epi16::<3>(m7); // psrlw m11, m7, 3
        m8 = _mm256_packus_epi16(m8, m11); // packuswb m8, m11
        m8 = _mm256_blendv_epi8(m4, m8, m9); // vpblendvb m8, m4, m8, m9 ; p0
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride3).cast(), m8); // mova [tmpq+stride3q ], m8 ; p0
        } else {
            s1 = m8; // mova [rsp+1*32], m8
        }

        m0 = _mm256_unpacklo_epi8(m5, m15); // punpcklbw m0, m5, m15
        m1 = _mm256_unpackhi_epi8(m5, m15); // punpckhbw m1, m5, m15
        m8 = _mm256_maddubs_epi16(m0, mem(&pb_1)); // pmaddubsw m8, m0, [pb_1]
        m11 = _mm256_maddubs_epi16(m1, mem(&pb_1)); // pmaddubsw m11, m1, [pb_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m11); // paddw m7, m11
        m8 = _mm256_unpacklo_epi8(m4, m12); // punpcklbw m8, m4, m12
        m11 = _mm256_unpackhi_epi8(m4, m12); // punpckhbw m11, m4, m12
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_1)); // pmaddubsw m8, [pb_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_1)); // pmaddubsw m11, [pb_1]
        m2 = _mm256_sub_epi16(m2, m8); // psubw m2, m8
        m7 = _mm256_sub_epi16(m7, m11); // psubw m7, m11 ; p2 + p1 + p0 + 2 * q0 + q1 + q2 + q3 + 4
        m8 = _mm256_srli_epi16::<3>(m2); // psrlw m8, m2, 3
        m11 = _mm256_srli_epi16::<3>(m7); // psrlw m11, m7, 3
        m8 = _mm256_packus_epi16(m8, m11); // packuswb m8, m11
        m11 = _mm256_blendv_epi8(m5, m8, m9); // vpblendvb m11, m5, m8, m9 ; q0
        if !DIR_H {
            _mm256_storeu_si256(dst.cast(), m11); // mova [dstq+strideq*0], m11 ; q0
        }

        m0 = _mm256_maddubs_epi16(m0, mem(&pb_m1_1)); // pmaddubsw m0, [pb_m1_1]
        m1 = _mm256_maddubs_epi16(m1, mem(&pb_m1_1)); // pmaddubsw m1, [pb_m1_1]
        m2 = _mm256_add_epi16(m2, m0); // paddw m2, m0
        m7 = _mm256_add_epi16(m7, m1); // paddw m7, m1
        m8 = _mm256_unpacklo_epi8(m13, m6); // punpcklbw m8, m13, m6
        m13 = _mm256_unpackhi_epi8(m13, m6); // punpckhbw m13, m6
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m13 = _mm256_maddubs_epi16(m13, mem(&pb_m1_1)); // pmaddubsw m13, [pb_m1_1]
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m7 = _mm256_add_epi16(m7, m13); // paddw m7, m13 ; p1 + p0 + q0 + 2 * q1 + q2 + 2 * q3 + 4
        m8 = _mm256_srli_epi16::<3>(m2); // psrlw m8, m2, 3
        m13 = _mm256_srli_epi16::<3>(m7); // psrlw m13, m7, 3
        m8 = _mm256_packus_epi16(m8, m13); // packuswb m8, m13
        m13 = _mm256_blendv_epi8(m6, m8, m9); // vpblendvb m13, m6, m8, m9 ; q1
        if !DIR_H {
            _mm256_storeu_si256(at(dst, stride).cast(), m13); // mova [dstq+strideq*1], m13 ; q1
        }

        m0 = _mm256_unpacklo_epi8(m3, m6); // punpcklbw m0, m3, m6
        m1 = _mm256_unpackhi_epi8(m3, m6); // punpckhbw m1, m3, m6
        m0 = _mm256_maddubs_epi16(m0, mem(&pb_1)); // pmaddubsw m0, [pb_1]
        m1 = _mm256_maddubs_epi16(m1, mem(&pb_1)); // pmaddubsw m1, [pb_1]
        m2 = _mm256_sub_epi16(m2, m0); // psubw m2, m0
        m7 = _mm256_sub_epi16(m7, m1); // psubw m7, m1
        m0 = _mm256_unpacklo_epi8(m14, m15); // punpcklbw m0, m14, m15
        m1 = _mm256_unpackhi_epi8(m14, m15); // punpckhbw m1, m14, m15
        m0 = _mm256_maddubs_epi16(m0, mem(&pb_1)); // pmaddubsw m0, [pb_1]
        m1 = _mm256_maddubs_epi16(m1, mem(&pb_1)); // pmaddubsw m1, [pb_1]
        m2 = _mm256_add_epi16(m2, m0); // paddw m2, m0
        m7 = _mm256_add_epi16(m7, m1); // paddw m7, m1 ; p0 + q0 + q1 + q2 + 2 * q2 + 3 * q3 + 4
        m2 = _mm256_srli_epi16::<3>(m2); // psrlw m2, 3
        m7 = _mm256_srli_epi16::<3>(m7); // psrlw m7, 3
        m2 = _mm256_packus_epi16(m2, m7); // packuswb m2, m7
        m2 = _mm256_blendv_epi8(m14, m2, m9); // vpblendvb m2, m14, m2, m9 ; q2
        if !DIR_H {
            _mm256_storeu_si256(at(dst, stride * 2).cast(), m2); // mova [dstq+strideq*2], m2 ; q2
        } else {
            m0 = s0; // mova m0, [rsp+0*32]
            m1 = s1; // mova m1, [rsp+1*32]
            if WD == 8 {
                // 16x8 transpose
                m3 = _mm256_unpacklo_epi8(m12, m10); // punpcklbw m3, m12, m10
                m12 = _mm256_unpackhi_epi8(m12, m10); // punpckhbw m12, m10
                m10 = _mm256_unpacklo_epi8(m0, m1); // punpcklbw m10, m0, m1
                m0 = _mm256_unpackhi_epi8(m0, m1); // punpckhbw m0, m1
                m1 = _mm256_unpacklo_epi8(m11, m13); // punpcklbw m1, m11, m13
                m11 = _mm256_unpackhi_epi8(m11, m13); // punpckhbw m11, m13
                m13 = _mm256_unpacklo_epi8(m2, m15); // punpcklbw m13, m2, m15
                m2 = _mm256_unpackhi_epi8(m2, m15); // punpckhbw m2, m15

                m15 = _mm256_unpacklo_epi16(m3, m10); // punpcklwd m15, m3, m10
                m3 = _mm256_unpackhi_epi16(m3, m10); // punpckhwd m3, m10
                m10 = _mm256_unpacklo_epi16(m12, m0); // punpcklwd m10, m12, m0
                m12 = _mm256_unpackhi_epi16(m12, m0); // punpckhwd m12, m0
                m0 = _mm256_unpacklo_epi16(m1, m13); // punpcklwd m0, m1, m13
                m1 = _mm256_unpackhi_epi16(m1, m13); // punpckhwd m1, m13
                m13 = _mm256_unpacklo_epi16(m11, m2); // punpcklwd m13, m11, m2
                m11 = _mm256_unpackhi_epi16(m11, m2); // punpckhwd m11, m2

                m2 = _mm256_unpacklo_epi32(m15, m0); // punpckldq m2, m15, m0
                m15 = _mm256_unpackhi_epi32(m15, m0); // punpckhdq m15, m0
                m0 = _mm256_unpacklo_epi32(m3, m1); // punpckldq m0, m3, m1
                m3 = _mm256_unpackhi_epi32(m3, m1); // punpckhdq m3, m1
                m1 = _mm256_unpacklo_epi32(m10, m13); // punpckldq m1, m10, m13
                m10 = _mm256_unpackhi_epi32(m10, m13); // punpckhdq m10, m13
                m13 = _mm256_unpacklo_epi32(m12, m11); // punpckldq m13, m12, m11
                m12 = _mm256_unpackhi_epi32(m12, m11); // punpckhdq m12, m11

                // write 8x32
                let mut xm2 = _mm256_castsi256_si128(m2);
                let mut xm15 = _mm256_castsi256_si128(m15);
                let mut xm0 = _mm256_castsi256_si128(m0);
                let mut xm3 = _mm256_castsi256_si128(m3);
                let mut xm1 = _mm256_castsi256_si128(m1);
                let mut xm10 = _mm256_castsi256_si128(m10);
                let mut xm13 = _mm256_castsi256_si128(m13);
                let mut xm12 = _mm256_castsi256_si128(m12);
                movq_st(at(dst, -4), xm2); // movq [dstq+strideq*0-4], xm2
                movhps_st(at(dst, stride - 4), xm2); // movhps [dstq+strideq*1-4], xm2
                movq_st(at(dst, stride * 2 - 4), xm15); // movq [dstq+strideq*2-4], xm15
                movhps_st(at(dst, stride3 - 4), xm15); // movhps [dstq+stride3q -4], xm15
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movq_st(at(dst, -4), xm0); // movq [dstq+strideq*0-4], xm0
                movhps_st(at(dst, stride - 4), xm0); // movhps [dstq+strideq*1-4], xm0
                movq_st(at(dst, stride * 2 - 4), xm3); // movq [dstq+strideq*2-4], xm3
                movhps_st(at(dst, stride3 - 4), xm3); // movhps [dstq+stride3q -4], xm3
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movq_st(at(dst, -4), xm1); // movq [dstq+strideq*0-4], xm1
                movhps_st(at(dst, stride - 4), xm1); // movhps [dstq+strideq*1-4], xm1
                movq_st(at(dst, stride * 2 - 4), xm10); // movq [dstq+strideq*2-4], xm10
                movhps_st(at(dst, stride3 - 4), xm10); // movhps [dstq+stride3q -4], xm10
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movq_st(at(dst, -4), xm13); // movq [dstq+strideq*0-4], xm13
                movhps_st(at(dst, stride - 4), xm13); // movhps [dstq+strideq*1-4], xm13
                movq_st(at(dst, stride * 2 - 4), xm12); // movq [dstq+strideq*2-4], xm12
                movhps_st(at(dst, stride3 - 4), xm12); // movhps [dstq+stride3q -4], xm12
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]

                xm2 = _mm256_extracti128_si256::<1>(m2); // vextracti128 xm2, m2, 1
                xm15 = _mm256_extracti128_si256::<1>(m15); // vextracti128 xm15, m15, 1
                xm0 = _mm256_extracti128_si256::<1>(m0); // vextracti128 xm0, m0, 1
                xm3 = _mm256_extracti128_si256::<1>(m3); // vextracti128 xm3, m3, 1
                xm1 = _mm256_extracti128_si256::<1>(m1); // vextracti128 xm1, m1, 1
                xm10 = _mm256_extracti128_si256::<1>(m10); // vextracti128 xm10, m10, 1
                xm13 = _mm256_extracti128_si256::<1>(m13); // vextracti128 xm13, m13, 1
                xm12 = _mm256_extracti128_si256::<1>(m12); // vextracti128 xm12, m12, 1

                movq_st(at(dst, -4), xm2); // movq [dstq+strideq*0-4], xm2
                movhps_st(at(dst, stride - 4), xm2); // movhps [dstq+strideq*1-4], xm2
                movq_st(at(dst, stride * 2 - 4), xm15); // movq [dstq+strideq*2-4], xm15
                movhps_st(at(dst, stride3 - 4), xm15); // movhps [dstq+stride3q -4], xm15
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movq_st(at(dst, -4), xm0); // movq [dstq+strideq*0-4], xm0
                movhps_st(at(dst, stride - 4), xm0); // movhps [dstq+strideq*1-4], xm0
                movq_st(at(dst, stride * 2 - 4), xm3); // movq [dstq+strideq*2-4], xm3
                movhps_st(at(dst, stride3 - 4), xm3); // movhps [dstq+stride3q -4], xm3
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movq_st(at(dst, -4), xm1); // movq [dstq+strideq*0-4], xm1
                movhps_st(at(dst, stride - 4), xm1); // movhps [dstq+strideq*1-4], xm1
                movq_st(at(dst, stride * 2 - 4), xm10); // movq [dstq+strideq*2-4], xm10
                movhps_st(at(dst, stride3 - 4), xm10); // movhps [dstq+stride3q -4], xm10
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movq_st(at(dst, -4), xm13); // movq [dstq+strideq*0-4], xm13
                movhps_st(at(dst, stride - 4), xm13); // movhps [dstq+strideq*1-4], xm13
                movq_st(at(dst, stride * 2 - 4), xm12); // movq [dstq+strideq*2-4], xm12
                movhps_st(at(dst, stride3 - 4), xm12); // movhps [dstq+stride3q -4], xm12
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
            } else {
                // 16x16 transpose and store
                swap!(m5, m10, m2); // SWAP 5, 10, 2
                swap!(m6, m0); // SWAP 6, 0
                swap!(m7, m1); // SWAP 7, 1
                swap!(m8, m11); // SWAP 8, 11
                swap!(m9, m13); // SWAP 9, 13
                m0 = s11; // mova m0, [rsp+11*32]
                m1 = s12; // mova m1, [rsp+12*32]
                m2 = s13; // mova m2, [rsp+13*32]
                m3 = s14; // mova m3, [rsp+14*32]
                m4 = s19; // mova m4, [rsp+19*32]
                m11 = s20; // mova m11, [rsp+20*32]
                m12 = s15; // mova m12, [rsp+15*32]
                m13 = s16; // mova m13, [rsp+16*32]
                m14 = s17; // mova m14, [rsp+17*32]

                // TRANSPOSE_16X16B 1, 0, [rsp+18*32]
                TRANSPOSE_16X16B!(1, 0, s18; m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15);
                movu_xm_st(at(dst, -8), m0); // movu [dstq+strideq*0-8], xm0
                movu_xm_st(at(dst, stride - 8), m1); // movu [dstq+strideq*1-8], xm1
                movu_xm_st(at(dst, stride * 2 - 8), m2); // movu [dstq+strideq*2-8], xm2
                movu_xm_st(at(dst, stride3 - 8), m3); // movu [dstq+stride3q -8], xm3
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movu_xm_st(at(dst, -8), m4); // movu [dstq+strideq*0-8], xm4
                movu_xm_st(at(dst, stride - 8), m5); // movu [dstq+strideq*1-8], xm5
                movu_xm_st(at(dst, stride * 2 - 8), m6); // movu [dstq+strideq*2-8], xm6
                movu_xm_st(at(dst, stride3 - 8), m7); // movu [dstq+stride3q -8], xm7
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movu_xm_st(at(dst, -8), m8); // movu [dstq+strideq*0-8], xm8
                movu_xm_st(at(dst, stride - 8), m9); // movu [dstq+strideq*1-8], xm9
                movu_xm_st(at(dst, stride * 2 - 8), m10); // movu [dstq+strideq*2-8], xm10
                movu_xm_st(at(dst, stride3 - 8), m11); // movu [dstq+stride3q -8], xm11
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                movu_xm_st(at(dst, -8), m12); // movu [dstq+strideq*0-8], xm12
                movu_xm_st(at(dst, stride - 8), m13); // movu [dstq+strideq*1-8], xm13
                movu_xm_st(at(dst, stride * 2 - 8), m14); // movu [dstq+strideq*2-8], xm14
                movu_xm_st(at(dst, stride3 - 8), m15); // movu [dstq+stride3q -8], xm15
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                vextracti128_st(at(dst, -8), m0); // vextracti128 [dstq+strideq*0-8], m0, 1
                vextracti128_st(at(dst, stride - 8), m1); // vextracti128 [dstq+strideq*1-8], m1, 1
                vextracti128_st(at(dst, stride * 2 - 8), m2); // vextracti128 [dstq+strideq*2-8], m2, 1
                vextracti128_st(at(dst, stride3 - 8), m3); // vextracti128 [dstq+stride3q -8], m3, 1
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                vextracti128_st(at(dst, -8), m4); // vextracti128 [dstq+strideq*0-8], m4, 1
                vextracti128_st(at(dst, stride - 8), m5); // vextracti128 [dstq+strideq*1-8], m5, 1
                vextracti128_st(at(dst, stride * 2 - 8), m6); // vextracti128 [dstq+strideq*2-8], m6, 1
                vextracti128_st(at(dst, stride3 - 8), m7); // vextracti128 [dstq+stride3q -8], m7, 1
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                vextracti128_st(at(dst, -8), m8); // vextracti128 [dstq+strideq*0-8], m8, 1
                vextracti128_st(at(dst, stride - 8), m9); // vextracti128 [dstq+strideq*1-8], m9, 1
                vextracti128_st(at(dst, stride * 2 - 8), m10); // vextracti128 [dstq+strideq*2-8], m10, 1
                vextracti128_st(at(dst, stride3 - 8), m11); // vextracti128 [dstq+stride3q -8], m11, 1
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
                vextracti128_st(at(dst, -8), m12); // vextracti128 [dstq+strideq*0-8], m12, 1
                vextracti128_st(at(dst, stride - 8), m13); // vextracti128 [dstq+strideq*1-8], m13, 1
                vextracti128_st(at(dst, stride * 2 - 8), m14); // vextracti128 [dstq+strideq*2-8], m14, 1
                vextracti128_st(at(dst, stride3 - 8), m15); // vextracti128 [dstq+stride3q -8], m15, 1
                dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
            }
        }
    } else if WD == 6 {
        // flat6 filter

        m8 = _mm256_unpacklo_epi8(m13, m5); // punpcklbw m8, m13, m5
        m11 = _mm256_unpackhi_epi8(m13, m5); // punpckhbw m11, m13, m5
        m0 = _mm256_maddubs_epi16(m8, mem(&pb_3_1)); // pmaddubsw m0, m8, [pb_3_1]
        m1 = _mm256_maddubs_epi16(m11, mem(&pb_3_1)); // pmaddubsw m1, m11, [pb_3_1]
        m7 = _mm256_unpacklo_epi8(m4, m3); // punpcklbw m7, m4, m3
        m10 = _mm256_unpackhi_epi8(m4, m3); // punpckhbw m10, m4, m3
        m2 = _mm256_maddubs_epi16(m7, mem(&pb_2)); // pmaddubsw m2, m7, [pb_2]
        m12 = _mm256_maddubs_epi16(m10, mem(&pb_2)); // pmaddubsw m12, m10, [pb_2]
        m0 = _mm256_add_epi16(m0, m2); // paddw m0, m2
        m1 = _mm256_add_epi16(m1, m12); // paddw m1, m12
        m2 = _mm256_mulhrs_epi16(m0, mem(&pw_4096)); // pmulhrsw m2, m0, [pw_4096]
        m12 = _mm256_mulhrs_epi16(m1, mem(&pw_4096)); // pmulhrsw m12, m1, [pw_4096]
        m2 = _mm256_packus_epi16(m2, m12); // packuswb m2, m12
        m2 = _mm256_blendv_epi8(m3, m2, m9); // vpblendvb m2, m3, m2, m9
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride * 2).cast(), m2); // mova [tmpq+strideq*2], m2 ; p1
        }

        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_m1_1)); // pmaddubsw m11, [pb_m1_1]
        m0 = _mm256_add_epi16(m0, m8); // paddw m0, m8
        m1 = _mm256_add_epi16(m1, m11); // paddw m1, m11
        m8 = _mm256_unpacklo_epi8(m13, m6); // punpcklbw m8, m13, m6
        m11 = _mm256_unpackhi_epi8(m13, m6); // punpckhbw m11, m13, m6
        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m8, [pb_m1_1]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_m1_1)); // pmaddubsw m11, [pb_m1_1]
        m0 = _mm256_add_epi16(m0, m8); // paddw m0, m8
        m1 = _mm256_add_epi16(m1, m11); // paddw m1, m11
        m12 = _mm256_mulhrs_epi16(m0, mem(&pw_4096)); // pmulhrsw m12, m0, [pw_4096]
        m13 = _mm256_mulhrs_epi16(m1, mem(&pw_4096)); // pmulhrsw m13, m1, [pw_4096]
        m12 = _mm256_packus_epi16(m12, m13); // packuswb m12, m13
        m12 = _mm256_blendv_epi8(m4, m12, m9); // vpblendvb m12, m4, m12, m9
        if !DIR_H {
            _mm256_storeu_si256(at(tmp, stride3).cast(), m12); // mova [tmpq+stride3q], m12 ; p0
        }

        m0 = _mm256_add_epi16(m0, m8); // paddw m0, m8
        m1 = _mm256_add_epi16(m1, m11); // paddw m1, m11
        m8 = _mm256_unpacklo_epi8(m3, m14); // punpcklbw m8, m3, m14
        m11 = _mm256_unpackhi_epi8(m3, m14); // punpckhbw m11, m3, m14
        m14 = _mm256_maddubs_epi16(m8, mem(&pb_m1_1)); // pmaddubsw m14, m8, [pb_m1_1]
        m13 = _mm256_maddubs_epi16(m11, mem(&pb_m1_1)); // pmaddubsw m13, m11, [pb_m1_1]
        m0 = _mm256_add_epi16(m0, m14); // paddw m0, m14
        m1 = _mm256_add_epi16(m1, m13); // paddw m1, m13
        m14 = _mm256_mulhrs_epi16(m0, mem(&pw_4096)); // pmulhrsw m14, m0, [pw_4096]
        m13 = _mm256_mulhrs_epi16(m1, mem(&pw_4096)); // pmulhrsw m13, m1, [pw_4096]
        m14 = _mm256_packus_epi16(m14, m13); // packuswb m14, m13
        m14 = _mm256_blendv_epi8(m5, m14, m9); // vpblendvb m14, m5, m14, m9
        if !DIR_H {
            _mm256_storeu_si256(dst.cast(), m14); // mova [dstq+strideq*0], m14 ; q0
        }

        m8 = _mm256_maddubs_epi16(m8, mem(&pb_m1_2)); // pmaddubsw m8, [pb_m1_2]
        m11 = _mm256_maddubs_epi16(m11, mem(&pb_m1_2)); // pmaddubsw m11, [pb_m1_2]
        m0 = _mm256_add_epi16(m0, m8); // paddw m0, m8
        m1 = _mm256_add_epi16(m1, m11); // paddw m1, m11
        m7 = _mm256_maddubs_epi16(m7, mem(&pb_m1_0)); // pmaddubsw m7, [pb_m1_0]
        m10 = _mm256_maddubs_epi16(m10, mem(&pb_m1_0)); // pmaddubsw m10, [pb_m1_0]
        m0 = _mm256_add_epi16(m0, m7); // paddw m0, m7
        m1 = _mm256_add_epi16(m1, m10); // paddw m1, m10
        m0 = _mm256_mulhrs_epi16(m0, mem(&pw_4096)); // pmulhrsw m0, [pw_4096]
        m1 = _mm256_mulhrs_epi16(m1, mem(&pw_4096)); // pmulhrsw m1, [pw_4096]
        m0 = _mm256_packus_epi16(m0, m1); // packuswb m0, m1
        m0 = _mm256_blendv_epi8(m6, m0, m9); // vpblendvb m0, m6, m0, m9
        if !DIR_H {
            _mm256_storeu_si256(at(dst, stride).cast(), m0); // mova [dstq+strideq*1], m0 ; q1
        } else {
            // TRANSPOSE_16x4_AND_WRITE_4x32 2, 12, 14, 0, 1
            TRANSPOSE_16x4_AND_WRITE_4x32!(dst, stride, stride3; m2, m12, m14, m0, m1);
        }
    } else {
        if !DIR_H {
            _mm256_storeu_si256(tmp.cast(), m3); // mova [tmpq+strideq*0], m3 ; p1
            _mm256_storeu_si256(at(tmp, stride).cast(), m4); // mova [tmpq+strideq*1], m4 ; p0
            _mm256_storeu_si256(at(tmp, stride * 2).cast(), m5); // mova [tmpq+strideq*2], m5 ; q0
            _mm256_storeu_si256(at(tmp, stride3).cast(), m6); // mova [tmpq+stride3q ], m6 ; q1
        } else {
            // TRANSPOSE_16x4_AND_WRITE_4x32 3, 4, 5, 6, 7
            TRANSPOSE_16x4_AND_WRITE_4x32!(dst, stride, stride3; m3, m4, m5, m6, m7);
        }
    }
    (dst, l as *const u8)
}

/// `.v4` of `lpf_v_sb_y_8bpc` (`FILTER 4, v`; also called from
/// `lpf_v_sb_uv_8bpc`).
///
/// # Safety
/// As for the entry points; the CPU supports AVX2.
#[target_feature(enable = "avx2")]
unsafe fn v4(
    dst: *mut u8,
    stride: isize,
    mask: *const u8,
    l: *const u8,
    l_stride: isize,
    lut: *const Av1FilterLUT,
    stride3: isize,
    mstride: isize,
) {
    filter::<4, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride); // FILTER 4, v
}

/// `.h4` of `lpf_h_sb_y_8bpc` (`FILTER 4, h`; also called from
/// `lpf_h_sb_uv_8bpc`). Returns `dstq` and `lq`.
///
/// # Safety
/// As for the entry points; the CPU supports AVX2.
#[target_feature(enable = "avx2")]
unsafe fn h4(
    dst: *mut u8,
    stride: isize,
    mask: *const u8,
    l: *const u8,
    l_stride: isize,
    lut: *const Av1FilterLUT,
    stride3: isize,
    l_stride3: isize,
) -> (*mut u8, *const u8) {
    filter::<4, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3) // FILTER 4, h
}

/// `cglobal lpf_v_sb_y_8bpc` (AVX2): `void dav1d_lpf_v_sb_y_8bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int w)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// Per group `g` of eight units whose mask bytes are not all 0 (`x` =
/// `32 * g`): reads the 32 bytes `l[8 * g - l_stride ..][..32]` and
/// `l[8 * g ..][..32]` (the levels of the eight blocks above the edge and
/// below it), `lut.sharp` (16 bytes) and 4 bytes at each of
/// `(u8*)mask + g`, `+ 4 + g` (wd8, wd16) and `+ 8 + g` (wd16); reads the
/// pixels `dst[x .. x + 32]` of the rows -2 … +1 (wd4), -4 … +3 (wd8) or
/// -7 … +6 (wd16) and writes the rows -2 … +1 (wd4), -3 … +2 (wd8) or
/// -6 … +5 (wd16), all 32 pixels of each.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_v_sb_y_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    w: c_int,
) {
    let (mut dst, mut mask, mut l, mut w) = (dst, mask as *const u8, l as *const u8, w);
    let l_stride = l_stride << 2; // shl l_strideq, 2
    l = l.wrapping_offset(-l_stride); // sub lq, l_strideq
    let mstride = stride.wrapping_neg(); // mov mstrideq, strideq ; neg mstrideq
    let stride3 = stride * 3; // lea stride3q, [strideq*3]

    // .loop:
    loop {
        if *mask.add(8) != 0 {
            // cmp byte [maskq+8], 0 ; vmask[2] ; je .no_flat16
            filter::<16, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride);
        // FILTER 16, v
        } else if *mask.add(4) != 0 {
            // .no_flat16: cmp byte [maskq+4], 0 ; vmask[1] ; je .no_flat
            filter::<8, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride);
        // FILTER 8, v
        } else if *mask != 0 {
            // .no_flat: cmp byte [maskq+0], 0 ; vmask[0] ; je .end
            v4(dst, stride, mask, l, l_stride, lut, stride3, mstride); // call .v4
        }

        // .end:
        l = l.wrapping_add(32); // add lq, 32
        dst = dst.wrapping_add(32); // add dstq, 32
        mask = mask.wrapping_add(1); // add maskq, 1
        w -= 8; // sub wd, 8
        if w <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal lpf_h_sb_y_8bpc` (AVX2): `void dav1d_lpf_h_sb_y_8bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int h)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// Per group `g` of eight units whose mask bytes are not all 0 (rows
/// `y` = `32 * g .. 32 * g + 32`): reads 8 bytes at `l[(8 * g + i) *
/// l_stride - 1]` for `i` in `0..8` (the levels of the block left of the
/// edge and right of it in each of the eight block rows), `lut.sharp` and
/// the mask bytes as `lpf_v_sb_y`; in each of the 32 rows reads the 4
/// (wd4), 8 (wd8) or 16 (wd16) pixels that start at `dst[y * stride - 2]`,
/// `- 4` or `- 8`, and writes the same pixels (wd4, wd8, wd16: all of
/// them, the outer ones with the value read).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_h_sb_y_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    h: c_int,
) {
    let (mut dst, mut mask, mut l, mut h) = (dst, mask as *const u8, l as *const u8, h);
    let l_stride = l_stride << 2; // shl l_strideq, 2
    l = l.wrapping_sub(4); // sub lq, 4
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    let l_stride3 = l_stride * 3; // lea l_stride3q, [l_strideq*3]

    // .loop:
    loop {
        if *mask.add(8) != 0 {
            // cmp byte [maskq+8], 0 ; vmask[2] ; je .no_flat16
            (dst, l) = filter::<16, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3);
        // FILTER 16, h
        } else if *mask.add(4) != 0 {
            // .no_flat16: cmp byte [maskq+4], 0 ; vmask[1] ; je .no_flat
            (dst, l) = filter::<8, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3);
        // FILTER 8, h
        } else if *mask != 0 {
            // .no_flat: cmp byte [maskq+0], 0 ; vmask[0] ; je .no_filter
            (dst, l) = h4(dst, stride, mask, l, l_stride, lut, stride3, l_stride3);
        // call .h4
        } else {
            // .no_filter:
            dst = dst.wrapping_offset(stride3 * 8); // lea dstq, [dstq+stride3q*8]
            l = l.wrapping_offset(l_stride * 8); // lea lq, [lq+l_strideq*8]
            dst = dst.wrapping_offset(stride * 8); // lea dstq, [dstq+strideq*8]
        }

        // .end:
        mask = mask.wrapping_add(1); // add maskq, 1
        h -= 8; // sub hd, 8
        if h <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal lpf_v_sb_uv_8bpc` (AVX2): `void dav1d_lpf_v_sb_uv_8bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int w)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// Per group `g` of eight units whose mask bytes are not both 0: the
/// level, `lut` and mask bytes of `lpf_v_sb_y` without `+ 8 + g`; reads
/// the pixels `dst[32 * g ..][..32]` of the rows -2 … +1 (wd4) or
/// -3 … +2 (wd6) and writes the rows -2 … +1, all 32 pixels of each.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_v_sb_uv_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    w: c_int,
) {
    let (mut dst, mut mask, mut l, mut w) = (dst, mask as *const u8, l as *const u8, w);
    let l_stride = l_stride << 2; // shl l_strideq, 2
    l = l.wrapping_offset(-l_stride); // sub lq, l_strideq
    let mstride = stride.wrapping_neg(); // mov mstrideq, strideq ; neg mstrideq
    let stride3 = stride * 3; // lea stride3q, [strideq*3]

    // .loop:
    loop {
        if *mask.add(4) != 0 {
            // cmp byte [maskq+4], 0 ; vmask[1] ; je .no_flat
            filter::<6, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride);
        // FILTER 6, v
        } else if *mask != 0 {
            // .no_flat: cmp byte [maskq+0], 0 ; vmask[0] ; je .end
            // call mangle(private_prefix %+ _lpf_v_sb_y_8bpc_avx2).v4
            v4(dst, stride, mask, l, l_stride, lut, stride3, mstride);
        }

        // .end:
        l = l.wrapping_add(32); // add lq, 32
        dst = dst.wrapping_add(32); // add dstq, 32
        mask = mask.wrapping_add(1); // add maskq, 1
        w -= 8; // sub wd, 8
        if w <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal lpf_h_sb_uv_8bpc` (AVX2): `void dav1d_lpf_h_sb_uv_8bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int h)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// Per group `g` of eight units whose mask bytes are not both 0: the
/// level, `lut` and mask bytes of `lpf_h_sb_y` without `+ 8 + g`; in each
/// of the 32 rows reads the 4 pixels at `dst[y * stride - 2]` (wd4) or
/// the 8 pixels at `dst[y * stride - 3]` (wd6: the six of the filter and
/// the two behind them, which the transpose drops) and writes the 4
/// pixels at `dst[y * stride - 2]`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_h_sb_uv_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    h: c_int,
) {
    let (mut dst, mut mask, mut l, mut h) = (dst, mask as *const u8, l as *const u8, h);
    let l_stride = l_stride << 2; // shl l_strideq, 2
    l = l.wrapping_sub(4); // sub lq, 4
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    let l_stride3 = l_stride * 3; // lea l_stride3q, [l_strideq*3]

    // .loop:
    loop {
        if *mask.add(4) != 0 {
            // cmp byte [maskq+4], 0 ; vmask[1] ; je .no_flat
            (dst, l) = filter::<6, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3);
        // FILTER 6, h
        } else if *mask != 0 {
            // .no_flat: cmp byte [maskq+0], 0 ; vmask[0] ; je .no_filter
            // call mangle(private_prefix %+ _lpf_h_sb_y_8bpc_avx2).h4
            (dst, l) = h4(dst, stride, mask, l, l_stride, lut, stride3, l_stride3);
        } else {
            // .no_filter:
            dst = dst.wrapping_offset(stride3 * 8); // lea dstq, [dstq+stride3q*8]
            l = l.wrapping_offset(l_stride * 8); // lea lq, [lq+l_strideq*8]
            dst = dst.wrapping_offset(stride * 8); // lea dstq, [dstq+strideq*8]
        }

        // .end:
        mask = mask.wrapping_add(1); // add maskq, 1
        h -= 8; // sub hd, 8
        if h <= 0 {
            break; // jg .loop
        }
    }
}
