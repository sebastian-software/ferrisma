// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/cdef_avx2.asm` (8 bpc) with AVX2
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The four entry points `dav1d_cdef_filter_{8x8,4x8,4x4}_8bpc_avx2` and
//! `dav1d_cdef_dir_8bpc_avx2`. A filter with all four neighbors present
//! (`edges == 0xf`, the fast path) reads the pixels around the block from the
//! picture, `top`, `bot` and `left` directly and filters bytes; otherwise
//! (`.border_block`) it copies the block and the neighbors the edge flags
//! allow into a 16-bit buffer `px` on its stack, marks the missing ones with
//! 0x8000 and filters words.
//!
//! Correspondence with the assembly:
//! - Each `cglobal` is the `extern "C"` function of the same name. The
//!   vector registers are the register file `M` (`m[5]` is `m5`; an `xmN`
//!   operand is `lo(m[N])`, an `xmm` result is stored zero-extended with
//!   `zx`), the general-purpose registers are locals (fast path: the struct
//!   `GprP1`) named as `DEFINE_ARGS` names them. Every function but the
//!   exported ones is `#[inline(always)]` without `#[target_feature]`, so each
//!   entry point is one function of AVX2 code; the kernels contain no call.
//! - `SECTION_RODATA` is one byte array, `RODATA`, with a constant per label
//!   (`ro(label, off)`), because the assembly reads across its labels
//!   (`[blend_4x4+16]` loads 32 bytes, 12 of them from `blend_4x8_0`); its
//!   bytes are the original object's.
//! - Fast path: the `cglobal` and `CDEF_FILTER w, h` up to `.border_block`
//!   are `cdef_filter_8bpc::<W, H, F>` (`F` the frame size), with the
//!   macro's `%if`s as `if`s on the constants; the main
//!   path, `.pri_only` and `.sec_only` are the branches on `pri` and `sec`,
//!   and the bodies of `.k_loop`, `.pri_k_loop` and `.sec_k_loop` are
//!   `cdef_filter_8bpc_{,pri_,sec_}k_loop`, called for k = 1 and then k = 0.
//!   `PREP_REGS`, `LOAD_BLOCK`, `ACCUMULATE_TAP_BYTE` and `ADJUST_PIXEL` are
//!   `prep_regs`, `load_block`, `accumulate_tap_byte` and `adjust_pixel`
//!   (register arguments as register numbers); the end of the 8x8
//!   `.v_loop` is `cdef_filter_8bpc_next_rows`. Each tap loader `.dNkK`,
//!   which the assembly `call`s through `cdef_filter_WxH_8bpc_jmptable`, is
//!   `cdef_filter_8bpc_dNkK::<W, H>` (labels that share code are one
//!   function: `d1k0` also serves `.d2k0` and `.d3k0`, `d5k0` `.d6k0` and
//!   `.d7k0`); `cdef_filter_8bpc_tap` is the table: a `match` on
//!   `dir*2 + k + tap*2` in the table's order. The loaders' `[rsp+gprsize+N]`
//!   is the entry's frame at `N` (`FrameP1`).
//! - Border path: `.border_block` is `cdef_filter_8bpc_border_block::<W, H>`,
//!   called through `cdef_filter_{8x8,4x8,4x4}_8bpc_border_block`, which own
//!   the `ALLOC_STACK` frame. The padding labels (`.no_right`, `.top_*`,
//!   `.no_top`, `.no_left`, `.bottom_*`, `.no_bottom`) are the branches of
//!   `if edge & N` chains in the assembly's order; `.border_pri_only` and
//!   `.border_sec_only` are early returns; the `_v_loop`s and `_k_loop`s are
//!   loops with the assembly's counters. `BORDER_LOAD_BLOCK`,
//!   `ACCUMULATE_TAP_WORD` and `BORDER_ADJUST_PIXEL` are
//!   `border_load_block::<W, INIT>`, `accumulate_tap_word::<W, CLIP>` and
//!   `border_adjust_pixel::<W, CLIP>`; `BORDER_PREP_REGS` is written out at
//!   its three sites.
//! - `cdef_dir_8bpc` is `dav1d_cdef_dir_8bpc_avx2` (the loads) and
//!   `cdef_dir_8bpc_main` (from `cglobal_label .main` to its `RET`), which
//!   `cdef_dir_16bpc` (`cdef16_avx2.rs`) also runs.
//! - The memory-operand helpers carry the suffix of the path that uses them
//!   (`_p1` fast path, `_p2` border path and `dir`).
//!
//! Deviations from the assembly, and whether they change the count:
//! - The fast path's jump table is a `match` on the run-time index
//!   (`cdef_filter_8bpc_tap`), with `dir & 7` (`dir` itself inside the
//!   contract, 0..=7); the compiler decides how to compile the choice.
//! - The fast path reads `left` at offsets -16 to +20 (32-byte `pblendw`
//!   operands at `[leftq-16+…]`, `pmovzxbw [leftq-7]`, 16 bytes at
//!   `[leftq+5]`), beyond the `h` rows of two pixels the caller hands over.
//!   In the decoder these are `lr_bak[bit][pl]`, 8 rows inside a stack array
//!   of 96 bytes, so the assembly reads before its start or behind its end
//!   for the first and the last plane; the blends and shifts that follow
//!   discard those bytes. The rebuild copies exactly the `h` rows (the C
//!   kernel's set) into a zeroed local with 16 bytes before and 32 behind
//!   them and runs the assembly's accesses on the copy (memory safety; the
//!   same value in every lane that reaches the output; about five
//!   instructions).
//! - Each frame (`ALLOC_STACK`) is a local zeroed whole: some loader blends
//!   read frame bytes that the assembly never writes and discard them (8x8
//!   `.d3k1` 0x18..0x20; 4x8 `.d3k1`/`.d4k1` 0x08..0x10 when a shift slot is
//!   unused, `.d0k1`/`.d1k1` 0x30..0x38, which is why the 4x8 frame is 8
//!   bytes larger than the assembly's), and the border path's padding writes
//!   every element of `px` the filter reads only by a case analysis over the
//!   edge flags. 10 to 14 stores per call, counted in the figures of ADR 0062.
//! - The shift counts `[rsp+0]`/`[rsp+8]` (memory operands of `psrlw`) and
//!   the 8x8 copy of `bot` (`[rsp+16]`) are locals.
//! - `lzcnt` is `leading_zeros` (the same value for every operand; the
//!   original selects these kernels on AVX2 alone, so no BMI instruction is
//!   used: `bsr`/`xor`, one instruction more), `cmovs` an `if`.
//! - Every vector access to caller memory is an unaligned move where the
//!   assembly uses `mova`; `movhps`, `pinsr*`, `pextrd`, `movd` to and from
//!   memory are unaligned scalar accesses of the same bytes. Same count.
//! - The read-only constants are plain loads; `tap_table` is indexed with
//!   the run-time shifts and directions as in the assembly, within its 42
//!   bytes by the contract (`dir` 0..=7, `damping` at most 6 at 8 bpc).
//! - Instruction scheduling and register allocation are the compiler's (it
//!   unrolls the two passes of some `k` loops).
//!
//! Reads beyond what the C kernel reads (rows -2..h+1, columns -2..w+1 of
//! the block, each neighbor only with its edge flag), kept as the assembly
//! makes them, on the original's buffers (the bounds are the original's;
//! derived from the code, not checked with guard pages): the fast path
//! reads block rows from column -2 (4x4 -4) to 17 (4x8 rows 4 to 7 to 15),
//! at 4x4 the two rows below from column -20 to 17 and the two above from
//! -2 to 29; the border path reads 16 bytes of each block row with
//! `CDEF_HAVE_RIGHT` (8 bytes at w 4) and the rows above and below from
//! column -8 to 15 depending on the flags. The picture's rows are the width
//! rounded up to 128 pixels (`picture.c`) and the CDEF line buffers have 32
//! bytes before and behind each row (`decode.c`), in the original and in
//! the translation alike. Writes: exactly the block.
//!
//! Safety (argued once for the family): the caller (`cdef_apply_tmpl`, or
//! the test) passes the block's first pixel and the picture's stride in
//! bytes, `left` with the block's rows of two pixels left of it, `top` and
//! `bot` pointing to the two rows above and below (picture or line buffer),
//! strengths, `dir` in 0..=7, `damping` and the edge flags; the kernels read
//! and write what the assembly reads and writes, except `left` (above).
#![allow(clippy::all, non_snake_case, non_upper_case_globals)]

use core::arch::x86_64::*;
use core::ffi::{c_int, c_uint};

// ---- registers, memory operands, read-only data ----

/// The vector registers `m0` … `m15` (`ymm`); `m[5]` is `m5`. An `xmN` operand is `lo(m[N])`; an instruction
/// with an `xmm` destination stores `zx(…)` (VEX clears the upper half).
pub(crate) type M = [__m256i; 16];

/// A fresh register file (all zero; the compiler drops the zeros of registers written before they are read).
#[inline(always)]
pub(crate) unsafe fn regs() -> M {
    [_mm256_setzero_si256(); 16]
}

/// `xmN`: the low 128 bits.
#[inline(always)]
pub(crate) unsafe fn lo(x: __m256i) -> __m128i {
    _mm256_castsi256_si128(x)
}

/// An `xmm` result written to `mN`: the upper half is zero.
#[inline(always)]
pub(crate) unsafe fn zx(x: __m128i) -> __m256i {
    _mm256_zextsi128_si256(x)
}

/// `p + off` bytes, wrapping (no claim that the address is in bounds until it is accessed).
#[inline(always)]
pub(crate) fn at<T>(p: *mut T, off: isize) -> *mut T {
    (p as *mut u8).wrapping_offset(off) as *mut T
}

/// `SECTION_RODATA 32` of `cdef_avx2.asm`, byte for byte (checked against the original's object file): the
/// assembly addresses its tables at offsets from their labels that reach into the next table
/// (`[blend_4x4+16]` loads 32 bytes, 12 of them from `blend_4x8_0`), so the tables are one array and each
/// label is an offset into it (the constants below). The jump tables that follow `tap_table` in the
/// original are code addresses and not part of it.
#[repr(C, align(32))]
pub(crate) struct Rodata(pub(crate) [u8; 466]);
pub(crate) static RODATA: Rodata = Rodata([
    0x04, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, // 0x000
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, // 0x010
    0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x020
    0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x030
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x040
    0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, // 0x050
    0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x060
    0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, // 0x070
    0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x080
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, // 0x090
    0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, // 0x0a0
    0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0b0
    0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, // 0x0c0
    0x80, 0x80, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0d0
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0e0
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x0f0
    0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x100
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x110
    0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x120
    0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x130
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x140
    0x48, 0x03, 0x00, 0x00, 0xa4, 0x01, 0x00, 0x00, 0x18, 0x01, 0x00, 0x00, 0xd2, 0x00, 0x00, 0x00, // 0x150
    0xa8, 0x00, 0x00, 0x00, 0x8c, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x69, 0x00, 0x00, 0x00, // 0x160
    0xa4, 0x01, 0x00, 0x00, 0xd2, 0x00, 0x00, 0x00, 0x8c, 0x00, 0x00, 0x00, 0x69, 0x00, 0x00, 0x00, // 0x170
    0x0c, 0x0d, 0x0a, 0x0b, 0x08, 0x09, 0x06, 0x07, 0x04, 0x05, 0x02, 0x03, 0x00, 0x01, 0x0e, 0x0f, // 0x180
    0x00, 0x08, 0x01, 0x09, 0x02, 0x0a, 0x03, 0x0b, 0x04, 0x0c, 0x05, 0x0d, 0x06, 0x0e, 0x07, 0x0f, // 0x190
    0x80, 0x00, 0x80, 0x00, 0x00, 0x08, 0x00, 0x08, 0xff, 0x7f, 0x3f, 0x1f, 0x0f, 0x07, 0x03, 0x01, // 0x1a0
    0x04, 0x02, 0x03, 0x03, 0x02, 0x01, 0xf1, 0xe2, 0x01, 0xf2, 0x01, 0x02, 0x01, 0x12, 0x11, 0x22, // 0x1b0
    0x10, 0x21, 0x10, 0x20, 0x10, 0x1f, 0xf1, 0xe2, 0x01, 0xf2, 0x01, 0x02, 0x01, 0x12, 0x11, 0x22, // 0x1c0
    0x10, 0x21, // 0x1d0
]);
pub(crate) const pd_47130256: usize = 0x000;
pub(crate) const blend_4x4: usize = 0x020;
pub(crate) const blend_4x8_0: usize = 0x04c;
pub(crate) const blend_4x8_1: usize = 0x06c;
pub(crate) const blend_4x8_2: usize = 0x094;
pub(crate) const blend_4x8_3: usize = 0x0b8;
pub(crate) const blend_8x8_0: usize = 0x0e0;
pub(crate) const blend_8x8_1: usize = 0x110;
pub(crate) const div_table: usize = 0x150;
pub(crate) const shufw_6543210x: usize = 0x180;
pub(crate) const shufb_lohi: usize = 0x190;
pub(crate) const pw_128: usize = 0x1a0;
pub(crate) const pw_2048: usize = 0x1a4;
pub(crate) const tap_table: usize = 0x1a8;

/// `[label+off]`: the address of a read-only operand. Every access through it must lie inside `RODATA`
/// (466 bytes); say at the use why it does when the offset is not a constant.
#[inline(always)]
pub(crate) fn ro(label: usize, off: isize) -> *const u8 {
    RODATA.0.as_ptr().wrapping_add(label).wrapping_offset(off)
}

// ---- part 1: the entry points `cdef_filter_{8x8,4x8,4x4}_8bpc` and their fast path (edges == 0xf):
// `cdef_avx2.asm` lines 94–296 (PREP_REGS, LOAD_BLOCK, ACCUMULATE_TAP_BYTE, ADJUST_PIXEL) and 397–1194 ----

// Memory operands of part 1 (suffix `_p1`: candidates for the prelude). Every access to caller memory is an
// unaligned access of exactly the bytes the instruction's memory operand covers.

/// `p + off` bytes for a read (wrapping: no claim that the address is in bounds until it is accessed).
#[inline(always)]
fn a_p1(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `movd xm, [p]` (4 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movd_p1(p: *const u8) -> __m128i {
    _mm_cvtsi32_si128(p.cast::<i32>().read_unaligned())
}

/// `movq xm, [p]` (8 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movq_p1(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}

/// A 16-byte memory operand (`movu xm, [p]`, `vinserti128 m, [p], 1`, `pmovzxbw m, [p]`, …).
#[inline(always)]
unsafe fn x_p1(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// A 32-byte memory operand (`movu m, [p]`, `vpblendvb m, [p], …`, `vpblendd m, [p], …`, `pblendw m, [p], …`).
#[inline(always)]
unsafe fn y_p1(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `vinserti128 m, [p], 1`: `m`'s low half and the 16 bytes at `p` as the high half.
#[inline(always)]
unsafe fn vinserti128_p1(m: __m256i, p: *const u8) -> __m256i {
    _mm256_inserti128_si256::<1>(m, x_p1(p))
}

/// `vinserti128 m, xmN, 1`.
#[inline(always)]
unsafe fn vinserti128_r_p1(m: __m256i, x: __m256i) -> __m256i {
    _mm256_inserti128_si256::<1>(m, lo(x))
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn vbroadcasti128_p1(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(x_p1(p))
}

/// `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastq_p1(p: *const u8) -> __m256i {
    _mm256_set1_epi64x(p.cast::<i64>().read_unaligned())
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_p1(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vpbroadcastb m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastb_p1(p: *const u8) -> __m256i {
    _mm256_set1_epi8(p.cast::<i8>().read())
}

/// `pinsrb xm, [p], I`.
#[inline(always)]
unsafe fn pinsrb_p1<const I: i32>(x: __m256i, p: *const u8) -> __m256i {
    zx(_mm_insert_epi8::<I>(lo(x), p.read() as i32))
}

/// `pinsrw xm, [p], I`.
#[inline(always)]
unsafe fn pinsrw_p1<const I: i32>(x: __m256i, p: *const u8) -> __m256i {
    zx(_mm_insert_epi16::<I>(lo(x), p.cast::<u16>().read_unaligned() as i32))
}

/// `pinsrd xm, [p], I`.
#[inline(always)]
unsafe fn pinsrd_p1<const I: i32>(x: __m256i, p: *const u8) -> __m256i {
    zx(_mm_insert_epi32::<I>(lo(x), p.cast::<i32>().read_unaligned()))
}

/// `movhps xm, [p]`: the low quadword of `xm` and the 8 bytes at `p` as the high one (an unaligned read where
/// `_mm_loadh_pd` reads through an aligned `*const f64`; the 8 bytes are only moved).
#[inline(always)]
unsafe fn movhps_p1(x: __m256i, p: *const u8) -> __m256i {
    let x = _mm_castsi128_pd(lo(x));
    zx(_mm_castpd_si128(_mm_setr_pd(
        _mm_cvtsd_f64(x),
        p.cast::<f64>().read_unaligned(),
    )))
}

/// `shufps xmD, xmA, xmB, IMM` on integer registers.
#[inline(always)]
unsafe fn shufps_x_p1<const IMM: i32>(a: __m256i, b: __m256i) -> __m256i {
    zx(_mm_castps_si128(_mm_shuffle_ps::<IMM>(
        _mm_castsi128_ps(lo(a)),
        _mm_castsi128_ps(lo(b)),
    )))
}

/// `shufps mD, mA, mB, IMM` on integer registers.
#[inline(always)]
unsafe fn shufps_y_p1<const IMM: i32>(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castps_si256(_mm256_shuffle_ps::<IMM>(_mm256_castsi256_ps(a), _mm256_castsi256_ps(b)))
}

/// `psrldq xmD, xmA, N`.
#[inline(always)]
unsafe fn psrldq_x_p1<const N: i32>(a: __m256i) -> __m256i {
    zx(_mm_bsrli_si128::<N>(lo(a)))
}

/// `movd [p], xm`.
#[inline(always)]
unsafe fn movd_st_p1(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(lo(x)))
}

/// `pextrd [p], xm, I`.
#[inline(always)]
unsafe fn pextrd_st_p1<const I: i32>(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_extract_epi32::<I>(lo(x)))
}

/// `movq [p], xm`.
#[inline(always)]
unsafe fn movq_st_p1(p: *mut u8, x: __m256i) {
    _mm_storel_epi64(p.cast(), lo(x))
}

/// `movhps [p], xm`: the high quadword, as the unaligned write of an extracted element (`vpextrq [p], xm, 1`,
/// the same 8 bytes in one instruction).
#[inline(always)]
unsafe fn movhps_st_p1(p: *mut u8, x: __m256i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(lo(x)))
}

// x86inc's shuffle immediates `qABCD` = `A<<6|B<<4|C<<2|D`.
const Q0110_P1: i32 = 0 << 6 | 1 << 4 | 1 << 2 | 0;
const Q1010_P1: i32 = 1 << 6 | 0 << 4 | 1 << 2 | 0;
const Q1020_P1: i32 = 1 << 6 | 0 << 4 | 2 << 2 | 0;
const Q1031_P1: i32 = 1 << 6 | 0 << 4 | 3 << 2 | 1;
const Q2010_P1: i32 = 2 << 6 | 0 << 4 | 1 << 2 | 0;
const Q2020_P1: i32 = 2 << 6 | 0 << 4 | 2 << 2 | 0;
const Q2121_P1: i32 = 2 << 6 | 1 << 4 | 2 << 2 | 1;
const Q3110_P1: i32 = 3 << 6 | 1 << 4 | 1 << 2 | 0;
const Q3131_P1: i32 = 3 << 6 | 1 << 4 | 3 << 2 | 1;

/// The stack frame of the fast path (`ALLOC_STACK`), addressed by the assembly's offsets from `rsp`; a tap loader
/// that the assembly `call`s reads it at `[rsp+gprsize+off]`, which is the entry's `[rsp+off]`. Zero-initialized
/// whole (see `cdef_filter_8bpc`): the tap loaders read bytes of it that the assembly never writes (masked out by
/// their blends), and the 4x8 loaders read up to 8 bytes beyond the assembly's `ALLOC_STACK` area, so the Rust frame
/// is that much larger.
#[repr(C, align(32))]
struct FrameP1<const N: usize>([u8; N]);

/// The general-purpose registers the tap loaders use, named as in the `DEFINE_ARGS` of `PREP_REGS`: `top` is
/// `topq` (4xN) or `top1q` (8x8), `h` is `hq` (8x8 only), `dst4` (4x8) and `stride3` (4xN; at 8x8 the entry's
/// `r11`, which is `stride3q` there).
struct GprP1 {
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    top: *const u8,
    bot: *const u8,
    dst4: *mut u8,
    stride3: isize,
    top2: *const u8,
    h: isize,
}

// ---- the tap loaders `.d0k0` … `.d7k1` (`call`ed through the jump table; they load p0 into m5 and p1 into m6) ----

/// `.d0k0`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d0k0<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = vpbroadcastq_p1(a_p1(dst, s - 1)); // vpbroadcastq m6, [dstq+strideq*1-1]
            m[10] = vpbroadcastq_p1(a_p1(dst, s * 2 - 1)); // vpbroadcastq m10, [dstq+strideq*2-1]
            m[5] = zx(movd_p1(a_p1(g.top, s + 1))); // movd xm5, [topq+strideq*1+1]
            m[9] = zx(movd_p1(a_p1(dst, 1))); // movd xm9, [dstq+strideq*0+1]
            m[11] = _mm256_bsrli_epi128::<2>(m[6]); // psrldq m11, m6, 2
            m[12] = _mm256_bsrli_epi128::<2>(m[10]); // psrldq m12, m10, 2
            m[6] = vinserti128_p1(m[6], a_p1(dst, s3 - 1)); // vinserti128 m6, [dstq+stride3q -1], 1
            m[10] = vinserti128_p1(m[10], a_p1(g.bot, -1)); // vinserti128 m10, [botq -1], 1
            m[5] = _mm256_blend_epi32::<0x10>(m[5], m[11]); // vpblendd m5, m11, 0x10
            m[9] = _mm256_blend_epi32::<0x10>(m[9], m[12]); // vpblendd m9, m12, 0x10
            m[11] = y_p1(ro(blend_4x4, 16)); // movu m11, [blend_4x4+16]
            m[6] = _mm256_unpacklo_epi32(m[6], m[10]); // punpckldq m6, m10
            m[5] = _mm256_unpacklo_epi32(m[5], m[9]); // punpckldq m5, m9
            m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x28)), m[11]); // vpblendvb m6, [rsp+gprsize+0x28], m11
        } else {
            let dst4 = g.dst4 as *const u8;
            m[5] = zx(movd_p1(a_p1(g.top, s + 1))); // movd xm5, [topq +strideq*1+1]
            m[6] = zx(movq_p1(a_p1(dst, s - 1))); // movq xm6, [dstq +strideq*1-1]
            m[10] = zx(movq_p1(a_p1(dst, s3 - 1))); // movq xm10, [dstq +stride3q -1]
            m[11] = zx(movq_p1(a_p1(dst4, s - 1))); // movq xm11, [dst4q+strideq*1-1]
            m[5] = pinsrd_p1::<1>(m[5], a_p1(dst, 1)); // pinsrd xm5, [dstq +strideq*0+1], 1
            m[6] = movhps_p1(m[6], a_p1(dst, s * 2 - 1)); // movhps xm6, [dstq +strideq*2-1]
            m[10] = movhps_p1(m[10], a_p1(dst4, -1)); // movhps xm10, [dst4q+strideq*0-1]
            m[11] = movhps_p1(m[11], a_p1(dst4, s * 2 - 1)); // movhps xm11, [dst4q+strideq*2-1]
            m[9] = psrldq_x_p1::<2>(m[6]); // psrldq xm9, xm6, 2
            m[5] = shufps_x_p1::<Q2010_P1>(m[5], m[9]); // shufps xm5, xm9, q2010 ; -1 +0 +1 +2
            m[6] = shufps_x_p1::<Q2020_P1>(m[6], m[10]); // shufps xm6, xm10, q2020 ; +1 +2 +3 +4
            m[9] = psrldq_x_p1::<2>(m[11]); // psrldq xm9, xm11, 2
            m[10] = psrldq_x_p1::<2>(m[10]); // psrldq xm10, 2
            m[10] = shufps_x_p1::<Q2020_P1>(m[10], m[9]); // shufps xm10, xm9, q2020 ; +3 +4 +5 +6
            m[9] = zx(movd_p1(a_p1(dst4, s3 - 1))); // movd xm9, [dst4q+stride3q -1]
            m[9] = pinsrd_p1::<1>(m[9], a_p1(g.bot, -1)); // pinsrd xm9, [botq -1], 1
            m[11] = shufps_x_p1::<Q1020_P1>(m[11], m[9]); // shufps xm11, xm9, q1020 ; +5 +6 +7 +8
            m[9] = _mm256_cvtepu8_epi16(x_p1(a_p1(g.left, 3))); // pmovzxbw m9, [leftq+3]
            m[6] = vinserti128_r_p1(m[6], m[11]); // vinserti128 m6, xm11, 1
            m[11] = y_p1(ro(blend_4x8_0, 4)); // movu m11, [blend_4x8_0+4]
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
            m[6] = _mm256_blendv_epi8(m[6], m[9], m[11]); // vpblendvb m6, m9, m11
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_0+16]
        m[5] = zx(movq_p1(a_p1(g.top2, 1))); // movq xm5, [top2q +1]
        m[10] = vbroadcasti128_p1(a_p1(dst, s - 1)); // vbroadcasti128 m10, [dstq+strideq*1-1]
        m[11] = vbroadcasti128_p1(a_p1(dst, s * 2 - 1)); // vbroadcasti128 m11, [dstq+strideq*2-1]
        m[5] = movhps_p1(m[5], a_p1(dst, 1)); // movhps xm5, [dstq+strideq*0+1]
        m[6] = vinserti128_p1(m[10], a_p1(dst, s3 - 1)); // vinserti128 m6, m10, [dstq+stride3q-1], 1
        m[9] = vinserti128_p1(m[11], a_p1(g.bot, -1)); // vinserti128 m9, m11, [botq -1], 1
        m[10] = _mm256_bsrli_epi128::<2>(m[10]); // psrldq m10, 2
        m[11] = _mm256_bsrli_epi128::<2>(m[11]); // psrldq m11, 2
        m[6] = _mm256_unpacklo_epi64(m[6], m[9]); // punpcklqdq m6, m9
        m[9] = y_p1(ro(blend_8x8_0, 16 + h * 2 * 1 + 16 * 1)); // movu m9, [r13+hq*2*1+16*1]
        m[10] = _mm256_unpacklo_epi64(m[10], m[11]); // punpcklqdq m10, m11
        m[5] = _mm256_blend_epi32::<0xF0>(m[5], m[10]); // vpblendd m5, m10, 0xF0
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x60 + h * 8 + 64 + 8 * 1)), m[9]);
        // vpblendvb m6, [rsp+gprsize+0x60+hq*8+64+8*1], m9
    }
}

/// `.d1k0`, `.d2k0`, `.d3k0` (one code).
#[inline(always)]
unsafe fn cdef_filter_8bpc_d1k0<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = zx(movq_p1(a_p1(dst, -1))); // movq xm6, [dstq+strideq*0-1]
            m[9] = zx(movq_p1(a_p1(dst, s - 1))); // movq xm9, [dstq+strideq*1-1]
            m[6] = vinserti128_p1(m[6], a_p1(dst, s * 2 - 1)); // vinserti128 m6, [dstq+strideq*2-1], 1
            m[9] = vinserti128_p1(m[9], a_p1(dst, s3 - 1)); // vinserti128 m9, [dstq+stride3q -1], 1
            m[11] = y_p1(a_p1(fr, 0x10)); // movu m11, [rsp+gprsize+0x10]
            m[12] = _mm256_cmpeq_epi32(m[12], m[12]); // pcmpeqd m12, m12
            m[5] = _mm256_bsrli_epi128::<2>(m[6]); // psrldq m5, m6, 2
            m[10] = _mm256_bsrli_epi128::<2>(m[9]); // psrldq m10, m9, 2
            m[12] = _mm256_srli_epi32::<24>(m[12]); // psrld m12, 24
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
            m[6] = _mm256_blendv_epi8(m[6], m[11], m[12]); // vpblendvb m6, m11, m12
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movq_p1(a_p1(dst, -1))); // movq xm6, [dstq +strideq*0-1]
            m[9] = zx(movq_p1(a_p1(dst, s * 2 - 1))); // movq xm9, [dstq +strideq*2-1]
            m[6] = movhps_p1(m[6], a_p1(dst, s - 1)); // movhps xm6, [dstq +strideq*1-1]
            m[9] = movhps_p1(m[9], a_p1(dst, s3 - 1)); // movhps xm9, [dstq +stride3q -1]
            m[10] = zx(movq_p1(a_p1(dst4, -1))); // movq xm10, [dst4q+strideq*0-1]
            m[10] = movhps_p1(m[10], a_p1(dst4, s - 1)); // movhps xm10, [dst4q+strideq*1-1]
            m[5] = psrldq_x_p1::<2>(m[6]); // psrldq xm5, xm6, 2
            m[11] = psrldq_x_p1::<2>(m[9]); // psrldq xm11, xm9, 2
            m[5] = shufps_x_p1::<Q2020_P1>(m[5], m[11]); // shufps xm5, xm11, q2020
            m[11] = zx(movq_p1(a_p1(dst4, s * 2 - 1))); // movq xm11, [dst4q+strideq*2-1]
            m[11] = movhps_p1(m[11], a_p1(dst4, s3 - 1)); // movhps xm11, [dst4q+stride3q -1]
            m[6] = shufps_x_p1::<Q2020_P1>(m[6], m[9]); // shufps xm6, xm9, q2020
            m[9] = shufps_x_p1::<Q2020_P1>(m[10], m[11]); // shufps xm9, xm10, xm11, q2020
            m[6] = vinserti128_r_p1(m[6], m[9]); // vinserti128 m6, xm9, 1
            m[9] = _mm256_cvtepu8_epi16(x_p1(a_p1(g.left, 1))); // pmovzxbw m9, [leftq+1]
            m[10] = psrldq_x_p1::<2>(m[10]); // psrldq xm10, 2
            m[11] = psrldq_x_p1::<2>(m[11]); // psrldq xm11, 2
            m[10] = shufps_x_p1::<Q2020_P1>(m[10], m[11]); // shufps xm10, xm11, q2020
            m[11] = vpbroadcastd_p1(ro(blend_4x8_0, 4)); // vpbroadcastd m11, [blend_4x8_0+4]
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
            m[6] = _mm256_blendv_epi8(m[6], m[9], m[11]); // vpblendvb m6, m9, m11
        }
    } else {
        let h = g.h;
        m[5] = zx(x_p1(a_p1(dst, -1))); // movu xm5, [dstq+strideq*0-1]
        m[9] = zx(x_p1(a_p1(dst, s - 1))); // movu xm9, [dstq+strideq*1-1]
        m[5] = vinserti128_p1(m[5], a_p1(dst, s * 2 - 1)); // vinserti128 m5, [dstq+strideq*2-1], 1
        m[9] = vinserti128_p1(m[9], a_p1(dst, s3 - 1)); // vinserti128 m9, [dstq+stride3q -1], 1
        m[10] = y_p1(ro(blend_8x8_0, 16)); // movu m10, [blend_8x8_0+16]
        m[6] = _mm256_unpacklo_epi64(m[5], m[9]); // punpcklqdq m6, m5, m9
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x60 + h * 8 + 64)), m[10]); // vpblendvb m6, [rsp+gprsize+0x60+hq*8+64], m10
        m[5] = _mm256_bsrli_epi128::<2>(m[5]); // psrldq m5, 2
        m[9] = _mm256_bsrli_epi128::<2>(m[9]); // psrldq m9, 2
        m[5] = _mm256_unpacklo_epi64(m[5], m[9]); // punpcklqdq m5, m9
    }
}

/// `.d4k0`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d4k0<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[10] = vpbroadcastq_p1(a_p1(dst, s - 1)); // vpbroadcastq m10, [dstq+strideq*1-1]
            m[11] = vpbroadcastq_p1(a_p1(dst, s * 2 - 1)); // vpbroadcastq m11, [dstq+strideq*2-1]
            m[6] = zx(movd_p1(a_p1(g.top, s - 1))); // movd xm6, [topq+strideq*1-1]
            m[9] = zx(movd_p1(a_p1(dst, -1))); // movd xm9, [dstq+strideq*0-1]
            m[5] = _mm256_bsrli_epi128::<2>(m[10]); // psrldq m5, m10, 2
            m[12] = _mm256_bsrli_epi128::<2>(m[11]); // psrldq m12, m11, 2
            m[6] = _mm256_blend_epi32::<0x10>(m[6], m[10]); // vpblendd m6, m10, 0x10
            m[9] = _mm256_blend_epi32::<0x10>(m[9], m[11]); // vpblendd m9, m11, 0x10
            m[10] = y_p1(ro(blend_4x4, 0)); // movu m10, [blend_4x4]
            m[5] = vinserti128_p1(m[5], a_p1(dst, s3 + 1)); // vinserti128 m5, [dstq+stride3q +1], 1
            m[12] = vinserti128_p1(m[12], a_p1(g.bot, 1)); // vinserti128 m12, [botq +1], 1
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[5] = _mm256_unpacklo_epi32(m[5], m[12]); // punpckldq m5, m12
            m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x40)), m[10]); // vpblendvb m6, [rsp+gprsize+0x40], m10
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movd_p1(a_p1(g.top, s - 1))); // movd xm6, [topq +strideq*1-1]
            m[9] = zx(movq_p1(a_p1(dst, s - 1))); // movq xm9, [dstq +strideq*1-1]
            m[10] = zx(movq_p1(a_p1(dst, s3 - 1))); // movq xm10, [dstq +stride3q -1]
            m[11] = zx(movq_p1(a_p1(dst4, s - 1))); // movq xm11, [dst4q+strideq*1-1]
            m[6] = pinsrd_p1::<1>(m[6], a_p1(dst, -1)); // pinsrd xm6, [dstq +strideq*0-1], 1
            m[9] = movhps_p1(m[9], a_p1(dst, s * 2 - 1)); // movhps xm9, [dstq +strideq*2-1]
            m[10] = movhps_p1(m[10], a_p1(dst4, -1)); // movhps xm10, [dst4q+strideq*0-1]
            m[11] = movhps_p1(m[11], a_p1(dst4, s * 2 - 1)); // movhps xm11, [dst4q+strideq*2-1]
            m[5] = psrldq_x_p1::<2>(m[9]); // psrldq xm5, xm9, 2
            m[6] = shufps_x_p1::<Q2010_P1>(m[6], m[9]); // shufps xm6, xm9, q2010
            m[9] = psrldq_x_p1::<2>(m[10]); // psrldq xm9, xm10, 2
            m[5] = shufps_x_p1::<Q2020_P1>(m[5], m[9]); // shufps xm5, xm9, q2020
            m[10] = shufps_x_p1::<Q2020_P1>(m[10], m[11]); // shufps xm10, xm11, q2020
            m[9] = zx(movd_p1(a_p1(dst4, s3 + 1))); // movd xm9, [dst4q+stride3q +1]
            m[6] = vinserti128_r_p1(m[6], m[10]); // vinserti128 m6, xm10, 1
            m[9] = pinsrd_p1::<1>(m[9], a_p1(g.bot, 1)); // pinsrd xm9, [botq +1], 1
            m[11] = psrldq_x_p1::<2>(m[11]); // psrldq xm11, 2
            m[10] = _mm256_cvtepu8_epi16(x_p1(a_p1(g.left, -1))); // pmovzxbw m10, [leftq-1]
            m[11] = shufps_x_p1::<Q1020_P1>(m[11], m[9]); // shufps xm11, xm9, q1020
            m[9] = y_p1(ro(blend_4x8_0, 0)); // movu m9, [blend_4x8_0]
            m[5] = vinserti128_r_p1(m[5], m[11]); // vinserti128 m5, xm11, 1
            m[6] = _mm256_blendv_epi8(m[6], m[10], m[9]); // vpblendvb m6, m10, m9
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_0+8]
        m[6] = zx(movq_p1(a_p1(g.top2, -1))); // movq xm6, [top2q -1]
        m[5] = vbroadcasti128_p1(a_p1(dst, s - 1)); // vbroadcasti128 m5, [dstq+strideq*1-1]
        m[9] = vbroadcasti128_p1(a_p1(dst, s * 2 - 1)); // vbroadcasti128 m9, [dstq+strideq*2-1]
        m[6] = movhps_p1(m[6], a_p1(dst, -1)); // movhps xm6, [dstq+strideq*0-1]
        m[11] = y_p1(ro(blend_8x8_0, 8 + h * 2 * 1 + 16 * 1)); // movu m11, [r13+hq*2*1+16*1]
        m[10] = _mm256_unpacklo_epi64(m[5], m[9]); // punpcklqdq m10, m5, m9
        m[5] = vinserti128_p1(m[5], a_p1(dst, s3 - 1)); // vinserti128 m5, [dstq+stride3q -1], 1
        m[9] = vinserti128_p1(m[9], a_p1(g.bot, -1)); // vinserti128 m9, [botq -1], 1
        m[6] = _mm256_blend_epi32::<0xF0>(m[6], m[10]); // vpblendd m6, m10, 0xF0
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x60 + h * 8 + 64 - 8 * 1)), m[11]); // vpblendvb m6, [rsp+gprsize+0x60+hq*8+64-8*1], m11
        m[5] = _mm256_bsrli_epi128::<2>(m[5]); // psrldq m5, 2
        m[9] = _mm256_bsrli_epi128::<2>(m[9]); // psrldq m9, 2
        m[5] = _mm256_unpacklo_epi64(m[5], m[9]); // punpcklqdq m5, m9
    }
}

/// `.d5k0`, `.d6k0`, `.d7k0` (one code).
#[inline(always)]
unsafe fn cdef_filter_8bpc_d5k0<const W: usize, const H: usize>(m: &mut M, g: &GprP1, _fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = zx(movd_p1(a_p1(g.top, s))); // movd xm6, [topq+strideq*1 ]
            m[5] = vpbroadcastd_p1(a_p1(dst, s)); // vpbroadcastd m5, [dstq+strideq*1 ]
            m[9] = vpbroadcastd_p1(a_p1(dst, s * 2)); // vpbroadcastd m9, [dstq+strideq*2 ]
            m[6] = zx(_mm_blend_epi32::<0x2>(lo(m[6]), x_p1(a_p1(dst, -4)))); // vpblendd xm6, [dstq+strideq*0-4], 0x2
            m[5] = _mm256_blend_epi32::<0x22>(m[5], m[9]); // vpblendd m5, m9, 0x22
            m[6] = _mm256_blend_epi32::<0x30>(m[6], m[5]); // vpblendd m6, m5, 0x30
            m[5] = vinserti128_p1(m[5], a_p1(dst, s3)); // vinserti128 m5, [dstq+stride3q ], 1
            m[5] = _mm256_blend_epi32::<0x20>(m[5], y_p1(a_p1(g.bot, -20))); // vpblendd m5, [botq -20], 0x20
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movd_p1(a_p1(g.top, s))); // movd xm6, [topq +strideq*1]
            m[5] = zx(movd_p1(a_p1(dst, s))); // movd xm5, [dstq +strideq*1]
            m[9] = zx(movd_p1(a_p1(dst, s3))); // movd xm9, [dstq +stride3q ]
            m[10] = zx(movd_p1(a_p1(dst4, s))); // movd xm10, [dst4q+strideq*1]
            m[11] = zx(movd_p1(a_p1(dst4, s3))); // movd xm11, [dst4q+stride3q ]
            m[6] = pinsrd_p1::<1>(m[6], dst); // pinsrd xm6, [dstq +strideq*0], 1
            m[5] = pinsrd_p1::<1>(m[5], a_p1(dst, s * 2)); // pinsrd xm5, [dstq +strideq*2], 1
            m[9] = pinsrd_p1::<1>(m[9], dst4); // pinsrd xm9, [dst4q+strideq*0], 1
            m[10] = pinsrd_p1::<1>(m[10], a_p1(dst4, s * 2)); // pinsrd xm10, [dst4q+strideq*2], 1
            m[11] = pinsrd_p1::<1>(m[11], g.bot); // pinsrd xm11, [botq ], 1
            m[6] = zx(_mm_unpacklo_epi64(lo(m[6]), lo(m[5]))); // punpcklqdq xm6, xm5
            m[5] = zx(_mm_unpacklo_epi64(lo(m[5]), lo(m[9]))); // punpcklqdq xm5, xm9
            m[9] = zx(_mm_unpacklo_epi64(lo(m[9]), lo(m[10]))); // punpcklqdq xm9, xm10
            m[10] = zx(_mm_unpacklo_epi64(lo(m[10]), lo(m[11]))); // punpcklqdq xm10, xm11
            m[6] = vinserti128_r_p1(m[6], m[9]); // vinserti128 m6, xm9, 1
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
        }
    } else {
        m[6] = zx(movq_p1(g.top2)); // movq xm6, [top2q ]
        m[5] = zx(movq_p1(a_p1(dst, s))); // movq xm5, [dstq+strideq*1]
        m[9] = zx(movq_p1(a_p1(dst, s3))); // movq xm9, [dstq+stride3q ]
        m[6] = movhps_p1(m[6], dst); // movhps xm6, [dstq+strideq*0]
        m[5] = movhps_p1(m[5], a_p1(dst, s * 2)); // movhps xm5, [dstq+strideq*2]
        m[9] = movhps_p1(m[9], g.bot); // movhps xm9, [botq ]
        m[6] = vinserti128_r_p1(m[6], m[5]); // vinserti128 m6, xm5, 1
        m[5] = vinserti128_r_p1(m[5], m[9]); // vinserti128 m5, xm9, 1
    }
}

/// `.d0k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d0k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = zx(movd_p1(a_p1(dst, s * 2 - 2))); // movd xm6, [dstq+strideq*2-2]
            m[9] = zx(movd_p1(a_p1(dst, s3 - 2))); // movd xm9, [dstq+stride3q -2]
            m[5] = zx(movd_p1(a_p1(g.top, 2))); // movd xm5, [topq+strideq*0+2]
            m[10] = zx(movd_p1(a_p1(g.top, s + 2))); // movd xm10, [topq+strideq*1+2]
            m[6] = pinsrw_p1::<0>(m[6], a_p1(g.left, 4)); // pinsrw xm6, [leftq+4], 0
            m[9] = pinsrw_p1::<0>(m[9], a_p1(g.left, 6)); // pinsrw xm9, [leftq+6], 0
            m[5] = vinserti128_p1(m[5], a_p1(dst, 2)); // vinserti128 m5, [dstq+strideq*0+2], 1
            m[10] = vinserti128_p1(m[10], a_p1(dst, s + 2)); // vinserti128 m10, [dstq+strideq*1+2], 1
            m[6] = vinserti128_p1(m[6], a_p1(g.bot, -2)); // vinserti128 m6, [botq+strideq*0-2], 1
            m[9] = vinserti128_p1(m[9], a_p1(g.bot, s - 2)); // vinserti128 m9, [botq+strideq*1-2], 1
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movq_p1(a_p1(dst, s * 2 - 2))); // movq xm6, [dstq +strideq*2-2]
            m[10] = zx(movd_p1(a_p1(dst4, s * 2 - 2))); // movd xm10, [dst4q+strideq*2-2]
            m[5] = zx(movd_p1(a_p1(g.top, 2))); // movd xm5, [topq +strideq*0+2]
            m[9] = zx(movq_p1(a_p1(dst4, -2))); // movq xm9, [dst4q+strideq*0-2]
            m[6] = movhps_p1(m[6], a_p1(dst, s3 - 2)); // movhps xm6, [dstq +stride3q -2]
            m[10] = pinsrw_p1::<3>(m[10], a_p1(dst4, s3)); // pinsrw xm10, [dst4q+stride3q ], 3
            m[5] = pinsrd_p1::<1>(m[5], a_p1(g.top, s + 2)); // pinsrd xm5, [topq +strideq*1+2], 1
            m[9] = movhps_p1(m[9], a_p1(dst4, s - 2)); // movhps xm9, [dst4q+strideq*1-2]
            m[10] = pinsrd_p1::<2>(m[10], a_p1(g.bot, -2)); // pinsrd xm10, [botq +strideq*0-2], 2
            m[5] = pinsrd_p1::<2>(m[5], a_p1(dst, 2)); // pinsrd xm5, [dstq +strideq*0+2], 2
            m[10] = pinsrd_p1::<3>(m[10], a_p1(g.bot, s - 2)); // pinsrd xm10, [botq +strideq*1-2], 3
            m[5] = pinsrd_p1::<3>(m[5], a_p1(dst, s + 2)); // pinsrd xm5, [dstq +strideq*1+2], 3
            m[11] = shufps_x_p1::<Q3131_P1>(m[6], m[9]); // shufps xm11, xm6, xm9, q3131
            m[6] = shufps_x_p1::<Q2020_P1>(m[6], m[9]); // shufps xm6, xm9, q2020
            m[9] = y_p1(ro(blend_4x8_3, 8)); // movu m9, [blend_4x8_3+8]
            m[6] = vinserti128_r_p1(m[6], m[10]); // vinserti128 m6, xm10, 1
            m[5] = vinserti128_r_p1(m[5], m[11]); // vinserti128 m5, xm11, 1
            m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x10 + 8)), m[9]); // vpblendvb m6, [rsp+gprsize+0x10+8], m9
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_1+16]
        m[6] = zx(movq_p1(a_p1(dst, s * 2 - 2))); // movq xm6, [dstq+strideq*2-2]
        m[9] = zx(movq_p1(a_p1(dst, s3 - 2))); // movq xm9, [dstq+stride3q -2]
        m[5] = zx(movq_p1(a_p1(g.top, 2))); // movq xm5, [top1q +2]
        m[10] = zx(movq_p1(a_p1(g.top2, 2))); // movq xm10, [top2q +2]
        m[11] = y_p1(ro(blend_8x8_1, 16 + h * 2 * 2 + 16 * 2)); // movu m11, [r13+hq*2*2+16*2]
        m[6] = vinserti128_p1(m[6], a_p1(g.bot, -2)); // vinserti128 m6, [botq+strideq*0-2], 1
        m[9] = vinserti128_p1(m[9], a_p1(g.bot, s - 2)); // vinserti128 m9, [botq+strideq*1-2], 1
        m[5] = vinserti128_p1(m[5], a_p1(dst, 2)); // vinserti128 m5, [dstq+strideq*0+2], 1
        m[10] = vinserti128_p1(m[10], a_p1(dst, s + 2)); // vinserti128 m10, [dstq+strideq*1+2], 1
        m[6] = _mm256_unpacklo_epi64(m[6], m[9]); // punpcklqdq m6, m9
        m[5] = _mm256_unpacklo_epi64(m[5], m[10]); // punpcklqdq m5, m10
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x20 + h * 8 + 64 + 8 * 2)), m[11]);
        // vpblendvb m6, [rsp+gprsize+0x20+hq*8+64+8*2], m11
    }
}

/// `.d1k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d1k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = vpbroadcastq_p1(a_p1(dst, s - 2)); // vpbroadcastq m6, [dstq+strideq*1-2]
            m[9] = vpbroadcastq_p1(a_p1(dst, s * 2 - 2)); // vpbroadcastq m9, [dstq+strideq*2-2]
            m[5] = zx(movd_p1(a_p1(g.top, s + 2))); // movd xm5, [topq+strideq*1+2]
            m[10] = zx(movd_p1(a_p1(dst, 2))); // movd xm10, [dstq+strideq*0+2]
            m[11] = _mm256_bsrli_epi128::<4>(m[6]); // psrldq m11, m6, 4
            m[12] = _mm256_bsrli_epi128::<4>(m[9]); // psrldq m12, m9, 4
            m[5] = _mm256_blend_epi32::<0x10>(m[5], m[11]); // vpblendd m5, m11, 0x10
            m[11] = zx(movq_p1(a_p1(g.left, 2))); // movq xm11, [leftq+2]
            m[6] = vinserti128_p1(m[6], a_p1(dst, s3 - 2)); // vinserti128 m6, [dstq+stride3q-2], 1
            m[11] = zx(_mm_unpacklo_epi32(lo(m[11]), lo(m[11]))); // punpckldq xm11, xm11
            m[10] = _mm256_blend_epi32::<0x10>(m[10], m[12]); // vpblendd m10, m12, 0x10
            m[12] = _mm256_cmpeq_epi32(m[12], m[12]); // pcmpeqd m12, m12
            m[11] = _mm256_cvtepu16_epi32(lo(m[11])); // pmovzxwd m11, xm11
            m[12] = _mm256_srli_epi32::<16>(m[12]); // psrld m12, 16
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[9] = vpbroadcastd_p1(a_p1(g.bot, -2)); // vpbroadcastd m9, [botq-2]
            m[6] = _mm256_blendv_epi8(m[6], m[11], m[12]); // vpblendvb m6, m11, m12
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
            m[6] = _mm256_blend_epi32::<0x20>(m[6], m[9]); // vpblendd m6, m9, 0x20
        } else {
            let dst4 = g.dst4 as *const u8;
            m[5] = zx(movd_p1(a_p1(g.top, s + 2))); // movd xm5, [topq +strideq*1+2]
            m[6] = zx(movq_p1(a_p1(dst, s - 2))); // movq xm6, [dstq +strideq*1-2]
            m[9] = zx(movq_p1(a_p1(dst, s3 - 2))); // movq xm9, [dstq +stride3q -2]
            m[10] = zx(movq_p1(a_p1(dst4, s - 2))); // movq xm10, [dst4q+strideq*1-2]
            m[11] = zx(movd_p1(a_p1(dst4, s3 - 2))); // movd xm11, [dst4q+stride3q -2]
            m[5] = pinsrd_p1::<1>(m[5], a_p1(dst, 2)); // pinsrd xm5, [dstq +strideq*0+2], 1
            m[6] = movhps_p1(m[6], a_p1(dst, s * 2 - 2)); // movhps xm6, [dstq +strideq*2-2]
            m[9] = movhps_p1(m[9], a_p1(dst4, -2)); // movhps xm9, [dst4q+strideq*0-2]
            m[10] = movhps_p1(m[10], a_p1(dst4, s * 2 - 2)); // movhps xm10, [dst4q+strideq*2-2]
            m[11] = pinsrd_p1::<1>(m[11], a_p1(g.bot, -2)); // pinsrd xm11, [botq -2], 1
            m[5] = shufps_x_p1::<Q3110_P1>(m[5], m[6]); // shufps xm5, xm6, q3110
            m[6] = shufps_x_p1::<Q2020_P1>(m[6], m[9]); // shufps xm6, xm9, q2020
            m[9] = shufps_x_p1::<Q3131_P1>(m[9], m[10]); // shufps xm9, xm10, q3131
            m[10] = shufps_x_p1::<Q1020_P1>(m[10], m[11]); // shufps xm10, xm11, q1020
            m[11] = y_p1(ro(blend_4x8_2, 4)); // movu m11, [blend_4x8_2+4]
            m[6] = vinserti128_r_p1(m[6], m[10]); // vinserti128 m6, xm10, 1
            m[5] = vinserti128_r_p1(m[5], m[9]); // vinserti128 m5, xm9, 1
            m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x10 + 4)), m[11]); // vpblendvb m6, [rsp+gprsize+0x10+4], m11
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_1+16]
        m[5] = zx(movq_p1(a_p1(g.top2, 2))); // movq xm5, [top2q +2]
        m[6] = vbroadcasti128_p1(a_p1(dst, s - 2)); // vbroadcasti128 m6, [dstq+strideq*1-2]
        m[9] = vbroadcasti128_p1(a_p1(dst, s * 2 - 2)); // vbroadcasti128 m9, [dstq+strideq*2-2]
        m[5] = movhps_p1(m[5], a_p1(dst, 2)); // movhps xm5, [dstq+strideq*0+2]
        m[10] = shufps_y_p1::<Q2121_P1>(m[6], m[9]); // shufps m10, m6, m9, q2121
        m[6] = vinserti128_p1(m[6], a_p1(dst, s3 - 2)); // vinserti128 m6, [dstq+stride3q -2], 1
        m[9] = vinserti128_p1(m[9], a_p1(g.bot, -2)); // vinserti128 m9, [botq -2], 1
        m[11] = y_p1(ro(blend_8x8_1, 16 + h * 2 * 1 + 16 * 1)); // movu m11, [r13+hq*2*1+16*1]
        m[5] = _mm256_blend_epi32::<0xF0>(m[5], m[10]); // vpblendd m5, m10, 0xF0
        m[6] = _mm256_unpacklo_epi64(m[6], m[9]); // punpcklqdq m6, m9
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x20 + h * 8 + 64 + 8 * 1)), m[11]);
        // vpblendvb m6, [rsp+gprsize+0x20+hq*8+64+8*1], m11
    }
}

/// `.d2k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d2k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[11] = zx(movq_p1(g.left)); // movq xm11, [leftq]
            m[6] = zx(movq_p1(a_p1(dst, -2))); // movq xm6, [dstq+strideq*0-2]
            m[9] = zx(movq_p1(a_p1(dst, s - 2))); // movq xm9, [dstq+strideq*1-2]
            m[6] = vinserti128_p1(m[6], a_p1(dst, s * 2 - 2)); // vinserti128 m6, [dstq+strideq*2-2], 1
            m[9] = vinserti128_p1(m[9], a_p1(dst, s3 - 2)); // vinserti128 m9, [dstq+stride3q -2], 1
            m[11] = zx(_mm_unpacklo_epi32(lo(m[11]), lo(m[11]))); // punpckldq xm11, xm11
            m[5] = _mm256_bsrli_epi128::<4>(m[6]); // psrldq m5, m6, 4
            m[10] = _mm256_bsrli_epi128::<4>(m[9]); // psrldq m10, m9, 4
            m[11] = _mm256_cvtepu16_epi32(lo(m[11])); // pmovzxwd m11, xm11
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
            m[6] = _mm256_blend_epi16::<0x05>(m[6], m[11]); // pblendw m6, m11, 0x05
        } else {
            let dst4 = g.dst4 as *const u8;
            m[5] = zx(movq_p1(a_p1(dst, -2))); // movq xm5, [dstq +strideq*0-2]
            m[9] = zx(movq_p1(a_p1(dst, s * 2 - 2))); // movq xm9, [dstq +strideq*2-2]
            m[10] = zx(movq_p1(a_p1(dst4, -2))); // movq xm10, [dst4q+strideq*0-2]
            m[11] = zx(movq_p1(a_p1(dst4, s * 2 - 2))); // movq xm11, [dst4q+strideq*2-2]
            m[5] = movhps_p1(m[5], a_p1(dst, s - 2)); // movhps xm5, [dstq +strideq*1-2]
            m[9] = movhps_p1(m[9], a_p1(dst, s3 - 2)); // movhps xm9, [dstq +stride3q -2]
            m[10] = movhps_p1(m[10], a_p1(dst4, s - 2)); // movhps xm10, [dst4q+strideq*1-2]
            m[11] = movhps_p1(m[11], a_p1(dst4, s3 - 2)); // movhps xm11, [dst4q+stride3q -2]
            m[6] = shufps_x_p1::<Q2020_P1>(m[5], m[9]); // shufps xm6, xm5, xm9, q2020
            m[5] = shufps_x_p1::<Q3131_P1>(m[5], m[9]); // shufps xm5, xm9, q3131
            m[9] = shufps_x_p1::<Q2020_P1>(m[10], m[11]); // shufps xm9, xm10, xm11, q2020
            m[10] = shufps_x_p1::<Q3131_P1>(m[10], m[11]); // shufps xm10, xm11, q3131
            m[11] = _mm256_cvtepu16_epi32(x_p1(g.left)); // pmovzxwd m11, [leftq]
            m[6] = vinserti128_r_p1(m[6], m[9]); // vinserti128 m6, xm9, 1
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
            m[6] = _mm256_blend_epi16::<0x55>(m[6], m[11]); // pblendw m6, m11, 0x55
        }
    } else {
        let h = g.h;
        m[11] = y_p1(a_p1(fr, 0x20 + h * 8 + 64)); // mova m11, [rsp+gprsize+0x20+hq*8+64]
        m[5] = zx(x_p1(a_p1(dst, -2))); // movu xm5, [dstq+strideq*0-2]
        m[9] = zx(x_p1(a_p1(dst, s - 2))); // movu xm9, [dstq+strideq*1-2]
        m[5] = vinserti128_p1(m[5], a_p1(dst, s * 2 - 2)); // vinserti128 m5, [dstq+strideq*2-2], 1
        m[9] = vinserti128_p1(m[9], a_p1(dst, s3 - 2)); // vinserti128 m9, [dstq+stride3q -2], 1
        m[6] = shufps_y_p1::<Q1010_P1>(m[5], m[9]); // shufps m6, m5, m9, q1010
        m[5] = shufps_y_p1::<Q2121_P1>(m[5], m[9]); // shufps m5, m9, q2121
        m[6] = _mm256_blend_epi16::<0x11>(m[6], m[11]); // pblendw m6, m11, 0x11
    }
}

/// `.d3k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d3k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[11] = vpbroadcastq_p1(a_p1(dst, s - 2)); // vpbroadcastq m11, [dstq+strideq*1-2]
            m[12] = vpbroadcastq_p1(a_p1(dst, s * 2 - 2)); // vpbroadcastq m12, [dstq+strideq*2-2]
            m[6] = zx(movd_p1(a_p1(g.top, s - 2))); // movd xm6, [topq+strideq*1-2]
            m[9] = zx(movd_p1(a_p1(dst, -2))); // movd xm9, [dstq+strideq*0-2]
            m[11] = _mm256_blend_epi16::<0x01>(m[11], y_p1(a_p1(g.left, -16 + 2))); // pblendw m11, [leftq-16+2], 0x01
            m[12] = _mm256_blend_epi16::<0x01>(m[12], y_p1(a_p1(g.left, -16 + 4))); // pblendw m12, [leftq-16+4], 0x01
            m[9] = pinsrw_p1::<0>(m[9], a_p1(g.left, -0 + 0)); // pinsrw xm9, [leftq- 0+0], 0
            m[5] = _mm256_bsrli_epi128::<4>(m[11]); // psrldq m5, m11, 4
            m[10] = _mm256_bsrli_epi128::<4>(m[12]); // psrldq m10, m12, 4
            m[5] = vinserti128_p1(m[5], a_p1(dst, s3 + 2)); // vinserti128 m5, [dstq+stride3q +2], 1
            m[10] = vinserti128_p1(m[10], a_p1(g.bot, 2)); // vinserti128 m10, [botq +2], 1
            m[6] = _mm256_blend_epi32::<0x10>(m[6], m[11]); // vpblendd m6, m11, 0x10
            m[9] = _mm256_blend_epi32::<0x10>(m[9], m[12]); // vpblendd m9, m12, 0x10
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movd_p1(a_p1(g.top, s - 2))); // movd xm6, [topq +strideq*1-2]
            m[5] = zx(movq_p1(a_p1(dst, s - 2))); // movq xm5, [dstq +strideq*1-2]
            m[9] = zx(movq_p1(a_p1(dst, s3 - 2))); // movq xm9, [dstq +stride3q -2]
            m[10] = zx(movq_p1(a_p1(dst4, s - 2))); // movq xm10, [dst4q+strideq*1-2]
            m[11] = zx(movd_p1(a_p1(dst4, s3 + 2))); // movd xm11, [dst4q+stride3q +2]
            m[6] = pinsrw_p1::<3>(m[6], dst); // pinsrw xm6, [dstq +strideq*0 ], 3
            m[5] = movhps_p1(m[5], a_p1(dst, s * 2 - 2)); // movhps xm5, [dstq +strideq*2-2]
            m[9] = movhps_p1(m[9], a_p1(dst4, -2)); // movhps xm9, [dst4q+strideq*0-2]
            m[10] = movhps_p1(m[10], a_p1(dst4, s * 2 - 2)); // movhps xm10, [dst4q+strideq*2-2]
            m[11] = pinsrd_p1::<1>(m[11], a_p1(g.bot, 2)); // pinsrd xm11, [botq +2], 1
            m[6] = shufps_x_p1::<Q2010_P1>(m[6], m[5]); // shufps xm6, xm5, q2010
            m[5] = shufps_x_p1::<Q3131_P1>(m[5], m[9]); // shufps xm5, xm9, q3131
            m[9] = shufps_x_p1::<Q2020_P1>(m[9], m[10]); // shufps xm9, xm10, q2020
            m[10] = shufps_x_p1::<Q1031_P1>(m[10], m[11]); // shufps xm10, xm11, q1031
            m[11] = y_p1(ro(blend_4x8_2, 0)); // movu m11, [blend_4x8_2]
            m[6] = vinserti128_r_p1(m[6], m[9]); // vinserti128 m6, xm9, 1
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
            m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x10 - 4)), m[11]); // vpblendvb m6, [rsp+gprsize+0x10-4], m11
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_1+8]
        m[6] = zx(movq_p1(a_p1(g.top2, -2))); // movq xm6, [top2q -2]
        m[5] = vbroadcasti128_p1(a_p1(dst, s - 2)); // vbroadcasti128 m5, [dstq+strideq*1-2]
        m[10] = vbroadcasti128_p1(a_p1(dst, s * 2 - 2)); // vbroadcasti128 m10, [dstq+strideq*2-2]
        m[6] = movhps_p1(m[6], a_p1(dst, -2)); // movhps xm6, [dstq+strideq*0-2]
        m[9] = _mm256_unpacklo_epi64(m[5], m[10]); // punpcklqdq m9, m5, m10
        m[5] = vinserti128_p1(m[5], a_p1(dst, s3 - 2)); // vinserti128 m5, [dstq+stride3q -2], 1
        m[10] = vinserti128_p1(m[10], a_p1(g.bot, -2)); // vinserti128 m10, [botq -2], 1
        m[11] = y_p1(ro(blend_8x8_1, 8 + h * 2 * 1 + 16 * 1)); // movu m11, [r13+hq*2*1+16*1]
        m[6] = _mm256_blend_epi32::<0xF0>(m[6], m[9]); // vpblendd m6, m9, 0xF0
        m[5] = shufps_y_p1::<Q2121_P1>(m[5], m[10]); // shufps m5, m10, q2121
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x20 + h * 8 + 64 - 8 * 1)), m[11]);
        // vpblendvb m6, [rsp+gprsize+0x20+hq*8+64-8*1], m11
    }
}

/// `.d4k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d4k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            // The low halves of m6 and m9 keep what the registers held before (as in the assembly); only their
            // dwords 0 (replaced below) reach the accumulation, which reads the low 8 bytes of each lane.
            m[6] = vinserti128_p1(m[6], dst.wrapping_offset(-2)); // vinserti128 m6, [dstq+strideq*0-2], 1
            m[9] = vinserti128_p1(m[9], a_p1(dst, s - 2)); // vinserti128 m9, [dstq+strideq*1-2], 1
            m[5] = zx(movd_p1(a_p1(dst, s * 2 + 2))); // movd xm5, [dstq+strideq*2+2]
            m[10] = zx(movd_p1(a_p1(dst, s3 + 2))); // movd xm10, [dstq+stride3q +2]
            m[6] = _mm256_blend_epi16::<0x01>(m[6], y_p1(a_p1(g.left, -16 + 0))); // pblendw m6, [leftq-16+0], 0x01
            m[9] = _mm256_blend_epi16::<0x01>(m[9], y_p1(a_p1(g.left, -16 + 2))); // pblendw m9, [leftq-16+2], 0x01
            m[5] = vinserti128_p1(m[5], a_p1(g.bot, 2)); // vinserti128 m5, [botq+strideq*0+2], 1
            m[10] = vinserti128_p1(m[10], a_p1(g.bot, s + 2)); // vinserti128 m10, [botq+strideq*1+2], 1
            m[6] = _mm256_blend_epi32::<0x01>(m[6], y_p1(a_p1(g.top, -2))); // vpblendd m6, [topq+strideq*0-2], 0x01
            m[9] = _mm256_blend_epi32::<0x01>(m[9], y_p1(a_p1(g.top, s - 2))); // vpblendd m9, [topq+strideq*1-2], 0x01
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movd_p1(a_p1(g.top, -2))); // movd xm6, [topq +strideq*0-2]
            m[5] = zx(movq_p1(a_p1(dst, s * 2 - 2))); // movq xm5, [dstq +strideq*2-2]
            m[9] = zx(movq_p1(a_p1(dst4, -2))); // movq xm9, [dst4q+strideq*0-2]
            m[10] = zx(movd_p1(a_p1(dst4, s * 2 + 2))); // movd xm10, [dst4q+strideq*2+2]
            m[6] = pinsrd_p1::<1>(m[6], a_p1(g.top, s - 2)); // pinsrd xm6, [topq +strideq*1-2], 1
            m[5] = movhps_p1(m[5], a_p1(dst, s3 - 2)); // movhps xm5, [dstq +stride3q -2]
            m[9] = movhps_p1(m[9], a_p1(dst4, s - 2)); // movhps xm9, [dst4q+strideq*1-2]
            m[10] = pinsrd_p1::<1>(m[10], a_p1(dst4, s3 + 2)); // pinsrd xm10, [dst4q+stride3q +2], 1
            m[6] = pinsrd_p1::<2>(m[6], a_p1(dst, -2)); // pinsrd xm6, [dstq +strideq*0-2], 2
            m[10] = pinsrd_p1::<2>(m[10], a_p1(g.bot, 2)); // pinsrd xm10, [botq +strideq*0+2], 2
            m[6] = pinsrd_p1::<3>(m[6], a_p1(dst, s - 2)); // pinsrd xm6, [dstq +strideq*1-2], 3
            m[10] = pinsrd_p1::<3>(m[10], a_p1(g.bot, s + 2)); // pinsrd xm10, [botq +strideq*1+2], 3
            m[11] = shufps_x_p1::<Q2020_P1>(m[5], m[9]); // shufps xm11, xm5, xm9, q2020
            m[5] = shufps_x_p1::<Q3131_P1>(m[5], m[9]); // shufps xm5, xm9, q3131
            m[9] = y_p1(ro(blend_4x8_3, 0)); // movu m9, [blend_4x8_3]
            m[6] = vinserti128_r_p1(m[6], m[11]); // vinserti128 m6, xm11, 1
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
            m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x10 - 8)), m[9]); // vpblendvb m6, [rsp+gprsize+0x10-8], m9
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_1]
        m[11] = y_p1(ro(blend_8x8_1, h * 2 * 2 + 16 * 2)); // movu m11, [r13+hq*2*2+16*2]
        m[6] = zx(movq_p1(a_p1(g.top, -2))); // movq xm6, [top1q -2]
        m[9] = zx(movq_p1(a_p1(g.top2, -2))); // movq xm9, [top2q -2]
        m[5] = zx(movq_p1(a_p1(dst, s * 2 + 2))); // movq xm5, [dstq+strideq*2+2]
        m[10] = zx(movq_p1(a_p1(dst, s3 + 2))); // movq xm10, [dstq+stride3q +2]
        m[6] = vinserti128_p1(m[6], a_p1(dst, -2)); // vinserti128 m6, [dstq+strideq*0-2], 1
        m[9] = vinserti128_p1(m[9], a_p1(dst, s - 2)); // vinserti128 m9, [dstq+strideq*1-2], 1
        m[5] = vinserti128_p1(m[5], a_p1(g.bot, 2)); // vinserti128 m5, [botq+strideq*0+2], 1
        m[10] = vinserti128_p1(m[10], a_p1(g.bot, s + 2)); // vinserti128 m10, [botq+strideq*1+2], 1
        m[6] = _mm256_unpacklo_epi64(m[6], m[9]); // punpcklqdq m6, m9
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x20 + h * 8 + 64 - 8 * 2)), m[11]); // vpblendvb m6, [rsp+gprsize+0x20+hq*8+64-8*2], m11
        m[5] = _mm256_unpacklo_epi64(m[5], m[10]); // punpcklqdq m5, m10
    }
}

/// `.d5k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d5k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = zx(movd_p1(a_p1(g.top, -1))); // movd xm6, [topq+strideq*0-1]
            m[9] = zx(movd_p1(a_p1(g.top, s - 1))); // movd xm9, [topq+strideq*1-1]
            m[5] = zx(movd_p1(a_p1(dst, s * 2 + 1))); // movd xm5, [dstq+strideq*2+1]
            m[10] = zx(movd_p1(a_p1(dst, s3 + 1))); // movd xm10, [dstq+stride3q +1]
            m[12] = _mm256_cmpeq_epi32(m[12], m[12]); // pcmpeqd m12, m12
            m[11] = _mm256_cvtepu8_epi16(x_p1(a_p1(g.left, -8 + 1))); // pmovzxbw m11, [leftq-8+1]
            m[12] = _mm256_srli_epi32::<24>(m[12]); // psrld m12, 24
            m[6] = vinserti128_p1(m[6], a_p1(dst, -1)); // vinserti128 m6, [dstq+strideq*0-1], 1
            m[9] = vinserti128_p1(m[9], a_p1(dst, s - 1)); // vinserti128 m9, [dstq+strideq*1-1], 1
            m[5] = vinserti128_p1(m[5], a_p1(g.bot, 1)); // vinserti128 m5, [botq+strideq*0+1], 1
            m[10] = vinserti128_p1(m[10], a_p1(g.bot, s + 1)); // vinserti128 m10, [botq+strideq*1+1], 1
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[9] = _mm256_setzero_si256(); // pxor m9, m9
            m[12] = _mm256_blend_epi32::<0x0F>(m[12], m[9]); // vpblendd m12, m9, 0x0F
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
            m[6] = _mm256_blendv_epi8(m[6], m[11], m[12]); // vpblendvb m6, m11, m12
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movd_p1(a_p1(g.top, -1))); // movd xm6, [topq +strideq*0-1]
            m[5] = zx(movq_p1(a_p1(dst, s * 2 - 1))); // movq xm5, [dstq +strideq*2-1]
            m[9] = zx(movq_p1(a_p1(dst4, -1))); // movq xm9, [dst4q+strideq*0-1]
            m[10] = zx(movd_p1(a_p1(dst4, s * 2 + 1))); // movd xm10, [dst4q+strideq*2+1]
            m[6] = pinsrd_p1::<1>(m[6], a_p1(g.top, s - 1)); // pinsrd xm6, [topq +strideq*1-1], 1
            m[5] = movhps_p1(m[5], a_p1(dst, s3 - 1)); // movhps xm5, [dstq +stride3q -1]
            m[9] = movhps_p1(m[9], a_p1(dst4, s - 1)); // movhps xm9, [dst4q+strideq*1-1]
            m[10] = pinsrd_p1::<1>(m[10], a_p1(dst4, s3 + 1)); // pinsrd xm10, [dst4q+stride3q +1], 1
            m[6] = pinsrd_p1::<2>(m[6], a_p1(dst, -1)); // pinsrd xm6, [dstq +strideq*0-1], 2
            m[10] = pinsrd_p1::<2>(m[10], a_p1(g.bot, 1)); // pinsrd xm10, [botq +strideq*0+1], 2
            m[6] = pinsrd_p1::<3>(m[6], a_p1(dst, s - 1)); // pinsrd xm6, [dstq +strideq*1-1], 3
            m[10] = pinsrd_p1::<3>(m[10], a_p1(g.bot, s + 1)); // pinsrd xm10, [botq +strideq*1+1], 3
            m[11] = shufps_x_p1::<Q2020_P1>(m[5], m[9]); // shufps xm11, xm5, xm9, q2020
            m[6] = vinserti128_r_p1(m[6], m[11]); // vinserti128 m6, xm11, 1
            m[11] = _mm256_cvtepu8_epi16(x_p1(a_p1(g.left, -3))); // pmovzxbw m11, [leftq-3]
            m[5] = psrldq_x_p1::<2>(m[5]); // psrldq xm5, 2
            m[9] = psrldq_x_p1::<2>(m[9]); // psrldq xm9, 2
            m[5] = shufps_x_p1::<Q2020_P1>(m[5], m[9]); // shufps xm5, xm9, q2020
            m[9] = y_p1(ro(blend_4x8_1, 0)); // movu m9, [blend_4x8_1]
            m[5] = vinserti128_r_p1(m[5], m[10]); // vinserti128 m5, xm10, 1
            m[6] = _mm256_blendv_epi8(m[6], m[11], m[9]); // vpblendvb m6, m11, m9
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_0]
        m[11] = y_p1(ro(blend_8x8_0, h * 2 * 2 + 16 * 2)); // movu m11, [r13+hq*2*2+16*2]
        m[6] = zx(movq_p1(a_p1(g.top, -1))); // movq xm6, [top1q -1]
        m[9] = zx(movq_p1(a_p1(g.top2, -1))); // movq xm9, [top2q -1]
        m[5] = zx(movq_p1(a_p1(dst, s * 2 + 1))); // movq xm5, [dstq+strideq*2+1]
        m[10] = zx(movq_p1(a_p1(dst, s3 + 1))); // movq xm10, [dstq+stride3q +1]
        m[6] = vinserti128_p1(m[6], a_p1(dst, -1)); // vinserti128 m6, [dstq+strideq*0-1], 1
        m[9] = vinserti128_p1(m[9], a_p1(dst, s - 1)); // vinserti128 m9, [dstq+strideq*1-1], 1
        m[5] = vinserti128_p1(m[5], a_p1(g.bot, 1)); // vinserti128 m5, [botq+strideq*0+1], 1
        m[10] = vinserti128_p1(m[10], a_p1(g.bot, s + 1)); // vinserti128 m10, [botq+strideq*1+1], 1
        m[6] = _mm256_unpacklo_epi64(m[6], m[9]); // punpcklqdq m6, m9
        m[5] = _mm256_unpacklo_epi64(m[5], m[10]); // punpcklqdq m5, m10
        m[6] = _mm256_blendv_epi8(m[6], y_p1(a_p1(fr, 0x60 + h * 8 + 64 - 8 * 2)), m[11]);
        // vpblendvb m6, [rsp+gprsize+0x60+hq*8+64-8*2], m11
    }
}

/// `.d6k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d6k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, _fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[6] = zx(movd_p1(g.top)); // movd xm6, [topq+strideq*0]
            m[9] = zx(movd_p1(a_p1(g.top, s))); // movd xm9, [topq+strideq*1]
            m[5] = zx(movd_p1(a_p1(dst, s * 2))); // movd xm5, [dstq+strideq*2]
            m[10] = zx(movd_p1(a_p1(dst, s3))); // movd xm10, [dstq+stride3q ]
            m[6] = vinserti128_p1(m[6], dst); // vinserti128 m6, [dstq+strideq*0], 1
            m[9] = vinserti128_p1(m[9], a_p1(dst, s)); // vinserti128 m9, [dstq+strideq*1], 1
            m[5] = vinserti128_p1(m[5], g.bot); // vinserti128 m5, [botq+strideq*0], 1
            m[10] = vinserti128_p1(m[10], a_p1(g.bot, s)); // vinserti128 m10, [botq+strideq*1], 1
            m[6] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m6, m9
            m[5] = _mm256_unpacklo_epi32(m[5], m[10]); // punpckldq m5, m10
        } else {
            let dst4 = g.dst4 as *const u8;
            m[5] = zx(movd_p1(a_p1(dst, s * 2))); // movd xm5, [dstq +strideq*2]
            m[6] = zx(movd_p1(g.top)); // movd xm6, [topq +strideq*0]
            m[9] = zx(movd_p1(a_p1(dst4, s * 2))); // movd xm9, [dst4q+strideq*2]
            m[5] = pinsrd_p1::<1>(m[5], a_p1(dst, s3)); // pinsrd xm5, [dstq +stride3q ], 1
            m[6] = pinsrd_p1::<1>(m[6], a_p1(g.top, s)); // pinsrd xm6, [topq +strideq*1], 1
            m[9] = pinsrd_p1::<1>(m[9], a_p1(dst4, s3)); // pinsrd xm9, [dst4q+stride3q ], 1
            m[5] = pinsrd_p1::<2>(m[5], dst4); // pinsrd xm5, [dst4q+strideq*0], 2
            m[6] = pinsrd_p1::<2>(m[6], dst); // pinsrd xm6, [dstq +strideq*0], 2
            m[9] = pinsrd_p1::<2>(m[9], g.bot); // pinsrd xm9, [botq +strideq*0], 2
            m[5] = pinsrd_p1::<3>(m[5], a_p1(dst4, s)); // pinsrd xm5, [dst4q+strideq*1], 3
            m[6] = pinsrd_p1::<3>(m[6], a_p1(dst, s)); // pinsrd xm6, [dstq +strideq*1], 3
            m[9] = pinsrd_p1::<3>(m[9], a_p1(g.bot, s)); // pinsrd xm9, [botq +strideq*1], 3
            m[6] = vinserti128_r_p1(m[6], m[5]); // vinserti128 m6, xm5, 1
            m[5] = vinserti128_r_p1(m[5], m[9]); // vinserti128 m5, xm9, 1
        }
    } else {
        m[5] = zx(movq_p1(a_p1(dst, s * 2))); // movq xm5, [dstq+strideq*2]
        m[9] = zx(movq_p1(g.bot)); // movq xm9, [botq+strideq*0]
        m[6] = zx(movq_p1(g.top)); // movq xm6, [top1q ]
        m[10] = zx(movq_p1(dst)); // movq xm10, [dstq+strideq*0]
        m[5] = movhps_p1(m[5], a_p1(dst, s3)); // movhps xm5, [dstq+stride3q ]
        m[9] = movhps_p1(m[9], a_p1(g.bot, s)); // movhps xm9, [botq+strideq*1]
        m[6] = movhps_p1(m[6], g.top2); // movhps xm6, [top2q ]
        m[10] = movhps_p1(m[10], a_p1(dst, s)); // movhps xm10, [dstq+strideq*1]
        m[5] = vinserti128_r_p1(m[5], m[9]); // vinserti128 m5, xm9, 1
        m[6] = vinserti128_r_p1(m[6], m[10]); // vinserti128 m6, xm10, 1
    }
}

/// `.d7k1`.
#[inline(always)]
unsafe fn cdef_filter_8bpc_d7k1<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8) {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    if W == 4 {
        if H == 4 {
            m[5] = zx(movd_p1(a_p1(dst, s * 2 - 1))); // movd xm5, [dstq+strideq*2-1]
            m[9] = zx(movd_p1(a_p1(dst, s3 - 1))); // movd xm9, [dstq+stride3q -1]
            m[6] = zx(movd_p1(a_p1(g.top, 1))); // movd xm6, [topq+strideq*0+1]
            m[10] = zx(movd_p1(a_p1(g.top, s + 1))); // movd xm10, [topq+strideq*1+1]
            m[5] = pinsrb_p1::<0>(m[5], a_p1(g.left, 5)); // pinsrb xm5, [leftq+ 5], 0
            m[9] = pinsrb_p1::<0>(m[9], a_p1(g.left, 7)); // pinsrb xm9, [leftq+ 7], 0
            m[6] = vinserti128_p1(m[6], a_p1(dst, 1)); // vinserti128 m6, [dstq+strideq*0+1], 1
            m[10] = vinserti128_p1(m[10], a_p1(dst, s + 1)); // vinserti128 m10, [dstq+strideq*1+1], 1
            m[5] = vinserti128_p1(m[5], a_p1(g.bot, -1)); // vinserti128 m5, [botq+strideq*0-1], 1
            m[9] = vinserti128_p1(m[9], a_p1(g.bot, s - 1)); // vinserti128 m9, [botq+strideq*1-1], 1
            m[6] = _mm256_unpacklo_epi32(m[6], m[10]); // punpckldq m6, m10
            m[5] = _mm256_unpacklo_epi32(m[5], m[9]); // punpckldq m5, m9
        } else {
            let dst4 = g.dst4 as *const u8;
            m[6] = zx(movd_p1(a_p1(g.top, 1))); // movd xm6, [topq +strideq*0+1]
            m[9] = zx(movq_p1(a_p1(dst, s * 2 - 1))); // movq xm9, [dstq +strideq*2-1]
            m[10] = zx(movq_p1(a_p1(dst4, -1))); // movq xm10, [dst4q+strideq*0-1]
            m[11] = zx(movd_p1(a_p1(dst4, s * 2 - 1))); // movd xm11, [dst4q+strideq*2-1]
            m[6] = pinsrd_p1::<1>(m[6], a_p1(g.top, s + 1)); // pinsrd xm6, [topq +strideq*1+1], 1
            m[9] = movhps_p1(m[9], a_p1(dst, s3 - 1)); // movhps xm9, [dstq +stride3q -1]
            m[10] = movhps_p1(m[10], a_p1(dst4, s - 1)); // movhps xm10, [dst4q+strideq*1-1]
            m[11] = pinsrd_p1::<1>(m[11], a_p1(dst4, s3 - 1)); // pinsrd xm11, [dst4q+stride3q -1], 1
            m[6] = pinsrd_p1::<2>(m[6], a_p1(dst, 1)); // pinsrd xm6, [dstq +strideq*0+1], 2
            m[11] = pinsrd_p1::<2>(m[11], a_p1(g.bot, -1)); // pinsrd xm11, [botq +strideq*0-1], 2
            m[6] = pinsrd_p1::<3>(m[6], a_p1(dst, s + 1)); // pinsrd xm6, [dstq +strideq*1+1], 3
            m[11] = pinsrd_p1::<3>(m[11], a_p1(g.bot, s - 1)); // pinsrd xm11, [botq +strideq*1-1], 3
            m[5] = shufps_x_p1::<Q2020_P1>(m[9], m[10]); // shufps xm5, xm9, xm10, q2020
            m[5] = vinserti128_r_p1(m[5], m[11]); // vinserti128 m5, xm11, 1
            m[11] = _mm256_cvtepu8_epi16(x_p1(a_p1(g.left, 5))); // pmovzxbw m11, [leftq+5]
            m[9] = psrldq_x_p1::<2>(m[9]); // psrldq xm9, 2
            m[10] = psrldq_x_p1::<2>(m[10]); // psrldq xm10, 2
            m[9] = shufps_x_p1::<Q2020_P1>(m[9], m[10]); // shufps xm9, xm10, q2020
            m[10] = y_p1(ro(blend_4x8_1, 8)); // movu m10, [blend_4x8_1+8]
            m[6] = vinserti128_r_p1(m[6], m[9]); // vinserti128 m6, xm9, 1
            m[5] = _mm256_blendv_epi8(m[5], m[11], m[10]); // vpblendvb m5, m11, m10
        }
    } else {
        let h = g.h;
        // lea r13, [blend_8x8_0+16]
        m[5] = zx(movq_p1(a_p1(dst, s * 2 - 1))); // movq xm5, [dstq+strideq*2-1]
        m[9] = zx(movq_p1(a_p1(g.bot, -1))); // movq xm9, [botq+strideq*0-1]
        m[6] = zx(movq_p1(a_p1(g.top, 1))); // movq xm6, [top1q +1]
        m[10] = zx(movq_p1(a_p1(dst, 1))); // movq xm10, [dstq+strideq*0+1]
        m[5] = movhps_p1(m[5], a_p1(dst, s3 - 1)); // movhps xm5, [dstq+stride3q -1]
        m[9] = movhps_p1(m[9], a_p1(g.bot, s - 1)); // movhps xm9, [botq+strideq*1-1]
        m[6] = movhps_p1(m[6], a_p1(g.top2, 1)); // movhps xm6, [top2q +1]
        m[10] = movhps_p1(m[10], a_p1(dst, s + 1)); // movhps xm10, [dstq+strideq*1+1]
        m[11] = y_p1(ro(blend_8x8_0, 16 + h * 2 * 2 + 16 * 2)); // movu m11, [r13+hq*2*2+16*2]
        m[5] = vinserti128_r_p1(m[5], m[9]); // vinserti128 m5, xm9, 1
        m[6] = vinserti128_r_p1(m[6], m[10]); // vinserti128 m6, xm10, 1
        m[5] = _mm256_blendv_epi8(m[5], y_p1(a_p1(fr, 0x60 + h * 8 + 64 + 8 * 2)), m[11]);
        // vpblendvb m5, [rsp+gprsize+0x60+hq*8+64+8*2], m11
    }
}

/// The computed `call` of `ACCUMULATE_TAP_BYTE` (`movsxd dirjmpq, [dirq+kq*4+tap*2*4]`, `add dirjmpq, tableq`,
/// `call dirjmpq`): `idx` = `dir*2 + k + tap*2` is the entry of `cdef_filter_WxH_8bpc_jmptable` (`d6k0, d6k1, d7k0,
/// d7k1, d0k0, … d7k1, d0k0, d0k1, d1k0, d1k1`), and each arm calls the loader that entry names.
#[inline(always)]
unsafe fn cdef_filter_8bpc_tap<const W: usize, const H: usize>(m: &mut M, g: &GprP1, fr: *const u8, idx: usize) {
    match idx {
        0 => cdef_filter_8bpc_d5k0::<W, H>(m, g, fr),  // d6k0
        1 => cdef_filter_8bpc_d6k1::<W, H>(m, g, fr),  // d6k1
        2 => cdef_filter_8bpc_d5k0::<W, H>(m, g, fr),  // d7k0
        3 => cdef_filter_8bpc_d7k1::<W, H>(m, g, fr),  // d7k1
        4 => cdef_filter_8bpc_d0k0::<W, H>(m, g, fr),  // d0k0
        5 => cdef_filter_8bpc_d0k1::<W, H>(m, g, fr),  // d0k1
        6 => cdef_filter_8bpc_d1k0::<W, H>(m, g, fr),  // d1k0
        7 => cdef_filter_8bpc_d1k1::<W, H>(m, g, fr),  // d1k1
        8 => cdef_filter_8bpc_d1k0::<W, H>(m, g, fr),  // d2k0
        9 => cdef_filter_8bpc_d2k1::<W, H>(m, g, fr),  // d2k1
        10 => cdef_filter_8bpc_d1k0::<W, H>(m, g, fr), // d3k0
        11 => cdef_filter_8bpc_d3k1::<W, H>(m, g, fr), // d3k1
        12 => cdef_filter_8bpc_d4k0::<W, H>(m, g, fr), // d4k0
        13 => cdef_filter_8bpc_d4k1::<W, H>(m, g, fr), // d4k1
        14 => cdef_filter_8bpc_d5k0::<W, H>(m, g, fr), // d5k0
        15 => cdef_filter_8bpc_d5k1::<W, H>(m, g, fr), // d5k1
        16 => cdef_filter_8bpc_d5k0::<W, H>(m, g, fr), // d6k0
        17 => cdef_filter_8bpc_d6k1::<W, H>(m, g, fr), // d6k1
        18 => cdef_filter_8bpc_d5k0::<W, H>(m, g, fr), // d7k0
        19 => cdef_filter_8bpc_d7k1::<W, H>(m, g, fr), // d7k1
        20 => cdef_filter_8bpc_d0k0::<W, H>(m, g, fr), // d0k0
        21 => cdef_filter_8bpc_d0k1::<W, H>(m, g, fr), // d0k1
        22 => cdef_filter_8bpc_d1k0::<W, H>(m, g, fr), // d1k0
        23 => cdef_filter_8bpc_d1k1::<W, H>(m, g, fr), // d1k1
        // Not reached: `dir` is `dir & 7`, so idx <= 7*2 + 1 + 4*2 = 23.
        _ => unreachable!(),
    }
}

// ---- the macros of lines 94–296 ----

/// `PREP_REGS w, h` (the jump table and `dirq` are the `idx` arithmetic of `accumulate_tap_byte`).
#[inline(always)]
unsafe fn prep_regs<const W: usize, const H: usize>(g: &mut GprP1, top: *const u8) {
    // mov dird, r7m ; lea tableq, [cdef_filter_WxH_8bpc_jmptable] ; lea dirq, [tableq+dirq*2*4]
    if W == 4 {
        if H != 4 {
            g.dst4 = at(g.dst, g.stride * 4); // lea dst4q, [dstq+strideq*4]
        }
    } else {
        g.h = -8; // mov hq, -8
        g.top = top; // lea top1q, [top1q+strideq*0]
        g.top2 = a_p1(top, g.stride); // lea top2q, [top1q+strideq*1]
    }
    if W == 4 {
        g.stride3 = g.stride * 3; // lea stride3q, [strideq*3]
    }
}

/// `LOAD_BLOCK w, h, init_min_max`; returns `k`.
#[inline(always)]
unsafe fn load_block<const W: usize, const H: usize, const INIT_MIN_MAX: bool>(m: &mut M, g: &GprP1) -> isize {
    let (dst, s, s3) = (g.dst as *const u8, g.stride, g.stride3);
    let k = 1; // mov kd, 1
    m[15] = _mm256_setzero_si256(); // pxor m15, m15 ; sum
    if H == 8 {
        m[12] = _mm256_setzero_si256(); // pxor m12, m12
        if W == 4 {
            let dst4 = g.dst4 as *const u8;
            m[4] = zx(movd_p1(dst)); // movd xm4, [dstq +strideq*0]
            m[6] = zx(movd_p1(a_p1(dst, s))); // movd xm6, [dstq +strideq*1]
            m[5] = zx(movd_p1(a_p1(dst, s * 2))); // movd xm5, [dstq +strideq*2]
            m[7] = zx(movd_p1(a_p1(dst, s3))); // movd xm7, [dstq +stride3q ]
            m[4] = vinserti128_p1(m[4], dst4); // vinserti128 m4, [dst4q+strideq*0], 1
            m[6] = vinserti128_p1(m[6], a_p1(dst4, s)); // vinserti128 m6, [dst4q+strideq*1], 1
            m[5] = vinserti128_p1(m[5], a_p1(dst4, s * 2)); // vinserti128 m5, [dst4q+strideq*2], 1
            m[7] = vinserti128_p1(m[7], a_p1(dst4, s3)); // vinserti128 m7, [dst4q+stride3q ], 1
            m[4] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m4, m6
            m[5] = _mm256_unpacklo_epi32(m[5], m[7]); // punpckldq m5, m7
        } else {
            m[4] = zx(movq_p1(dst)); // movq xm4, [dstq+strideq*0]
            m[5] = zx(movq_p1(a_p1(dst, s))); // movq xm5, [dstq+strideq*1]
            m[4] = vinserti128_p1(m[4], a_p1(dst, s * 2)); // vinserti128 m4, [dstq+strideq*2], 1
            m[5] = vinserti128_p1(m[5], a_p1(dst, s3)); // vinserti128 m5, [dstq+stride3q ], 1
        }
        m[4] = _mm256_unpacklo_epi64(m[4], m[5]); // punpcklqdq m4, m5
    } else {
        m[4] = zx(movd_p1(dst)); // movd xm4, [dstq+strideq*0]
        m[5] = zx(movd_p1(a_p1(dst, s))); // movd xm5, [dstq+strideq*1]
        m[4] = vinserti128_p1(m[4], a_p1(dst, s * 2)); // vinserti128 m4, [dstq+strideq*2], 1
        m[5] = vinserti128_p1(m[5], a_p1(dst, s3)); // vinserti128 m5, [dstq+stride3q ], 1
        m[4] = _mm256_unpacklo_epi32(m[4], m[5]); // punpckldq m4, m5
    }
    if INIT_MIN_MAX {
        m[7] = m[4]; // mova m7, m4 ; min
        m[8] = m[4]; // mova m8, m4 ; max
    }
    k
}

/// `ACCUMULATE_TAP_BYTE tap_offset, shift, mask, strength, mul_tap, w, h, clip`: `shift` is the frame offset of
/// the shift count (`[rsp+0]` or `[rsp+8]`), `mask`, `strength` and `mul_tap` are register numbers.
#[inline(always)]
unsafe fn accumulate_tap_byte<const W: usize, const H: usize, const CLIP: bool>(
    m: &mut M,
    g: &GprP1,
    fr: *const u8,
    dir: usize,
    k: isize,
    tap_offset: usize,
    shift: isize,
    mask: usize,
    strength: usize,
    mul_tap: usize,
) {
    // load p0/p1
    // movsxd dirjmpq, [dirq+kq*4+%1*2*4] ; add dirjmpq, tableq ; call dirjmpq
    cdef_filter_8bpc_tap::<W, H>(m, g, fr, dir * 2 + k as usize + tap_offset * 2);
    let sh = _mm_loadl_epi64(a_p1(fr, shift).cast()); // the 64-bit count of `psrlw m, m, [rsp+shift]`

    if CLIP {
        m[7] = _mm256_max_epu8(m[7], m[5]); // pmaxub m7, m5
        m[8] = _mm256_min_epu8(m[8], m[5]); // pminub m8, m5
        m[7] = _mm256_max_epu8(m[7], m[6]); // pmaxub m7, m6
        m[8] = _mm256_min_epu8(m[8], m[6]); // pminub m8, m6
    }

    // accumulate sum[m15] over p0/p1
    if H == 4 {
        m[5] = _mm256_unpacklo_epi8(m[5], m[6]); // punpcklbw m5, m6
        m[6] = _mm256_unpacklo_epi8(m[4], m[4]); // punpcklbw m6, m4, m4
        m[9] = _mm256_subs_epu8(m[5], m[6]); // psubusb m9, m5, m6
        m[5] = _mm256_subs_epu8(m[6], m[5]); // psubusb m5, m6, m5
        m[9] = _mm256_or_si256(m[9], m[5]); // por m9, m5 ; abs_diff_p01(p01 - px)
        m[5] = _mm256_cmpeq_epi8(m[5], m[9]); // pcmpeqb m5, m9
        m[5] = _mm256_or_si256(m[5], m[mul_tap]); // por m5, %5
        m[6] = _mm256_sign_epi8(m[mul_tap], m[5]); // psignb m6, %5, m5
        m[5] = _mm256_srl_epi16(m[9], sh); // psrlw m5, m9, %2 ; emulate 8-bit shift
        m[5] = _mm256_and_si256(m[5], m[mask]); // pand m5, %3
        m[5] = _mm256_subs_epu8(m[strength], m[5]); // psubusb m5, %4, m5
        m[5] = _mm256_min_epu8(m[5], m[9]); // pminub m5, m9
        m[5] = _mm256_maddubs_epi16(m[5], m[6]); // pmaddubsw m5, m6
        m[15] = _mm256_add_epi16(m[15], m[5]); // paddw m15, m5
    } else {
        m[9] = _mm256_subs_epu8(m[5], m[4]); // psubusb m9, m5, m4
        m[5] = _mm256_subs_epu8(m[4], m[5]); // psubusb m5, m4, m5
        m[11] = _mm256_subs_epu8(m[6], m[4]); // psubusb m11, m6, m4
        m[6] = _mm256_subs_epu8(m[4], m[6]); // psubusb m6, m4, m6
        m[9] = _mm256_or_si256(m[9], m[5]); // por m9, m5 ; abs_diff_p0(p0 - px)
        m[11] = _mm256_or_si256(m[11], m[6]); // por m11, m6 ; abs_diff_p1(p1 - px)
        m[5] = _mm256_cmpeq_epi8(m[5], m[9]); // pcmpeqb m5, m9
        m[6] = _mm256_cmpeq_epi8(m[6], m[11]); // pcmpeqb m6, m11
        m[10] = _mm256_unpackhi_epi8(m[9], m[11]); // punpckhbw m10, m9, m11
        m[9] = _mm256_unpacklo_epi8(m[9], m[11]); // punpcklbw m9, m11
        m[5] = _mm256_or_si256(m[5], m[mul_tap]); // por m5, %5
        m[11] = _mm256_or_si256(m[6], m[mul_tap]); // por m11, m6, %5
        m[6] = _mm256_unpackhi_epi8(m[5], m[11]); // punpckhbw m6, m5, m11
        m[5] = _mm256_unpacklo_epi8(m[5], m[11]); // punpcklbw m5, m11
        m[11] = _mm256_sign_epi8(m[mul_tap], m[6]); // psignb m11, %5, m6
        m[6] = _mm256_srl_epi16(m[10], sh); // psrlw m6, m10, %2 ; emulate 8-bit shift
        m[6] = _mm256_and_si256(m[6], m[mask]); // pand m6, %3
        m[6] = _mm256_subs_epu8(m[strength], m[6]); // psubusb m6, %4, m6
        m[6] = _mm256_min_epu8(m[6], m[10]); // pminub m6, m10
        m[6] = _mm256_maddubs_epi16(m[6], m[11]); // pmaddubsw m6, m11
        m[12] = _mm256_add_epi16(m[12], m[6]); // paddw m12, m6
        m[11] = _mm256_sign_epi8(m[mul_tap], m[5]); // psignb m11, %5, m5
        m[5] = _mm256_srl_epi16(m[9], sh); // psrlw m5, m9, %2 ; emulate 8-bit shift
        m[5] = _mm256_and_si256(m[5], m[mask]); // pand m5, %3
        m[5] = _mm256_subs_epu8(m[strength], m[5]); // psubusb m5, %4, m5
        m[5] = _mm256_min_epu8(m[5], m[9]); // pminub m5, m9
        m[5] = _mm256_maddubs_epi16(m[5], m[11]); // pmaddubsw m5, m11
        m[15] = _mm256_add_epi16(m[15], m[5]); // paddw m15, m5
    }
}

/// `ADJUST_PIXEL w, h, zero, pw_2048, clip` (`zero` and `pw_2048` are register numbers).
#[inline(always)]
unsafe fn adjust_pixel<const W: usize, const H: usize, const CLIP: bool>(
    m: &mut M,
    g: &GprP1,
    zero: usize,
    pw_2048_: usize,
) {
    let (dst, s, s3) = (g.dst, g.stride, g.stride3);
    if H == 4 {
        if CLIP {
            m[4] = _mm256_unpacklo_epi8(m[4], m[zero]); // punpcklbw m4, %3
        }
        m[zero] = _mm256_cmpgt_epi16(m[zero], m[15]); // pcmpgtw %3, m15
        m[15] = _mm256_add_epi16(m[15], m[zero]); // paddw m15, %3
        m[15] = _mm256_mulhrs_epi16(m[15], m[pw_2048_]); // pmulhrsw m15, %4
        if !CLIP {
            m[15] = _mm256_packs_epi16(m[15], m[15]); // packsswb m15, m15
            m[4] = _mm256_add_epi8(m[4], m[15]); // paddb m4, m15
        } else {
            m[4] = _mm256_add_epi16(m[4], m[15]); // paddw m4, m15
            m[4] = _mm256_packus_epi16(m[4], m[4]); // packuswb m4, m4 ; clip px in [0x0,0xff]
            m[4] = _mm256_min_epu8(m[4], m[7]); // pminub m4, m7
            m[4] = _mm256_max_epu8(m[4], m[8]); // pmaxub m4, m8
        }
        m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
        movd_st_p1(dst, m[4]); // movd [dstq+strideq*0], xm4
        movd_st_p1(at(dst, s * 2), m[5]); // movd [dstq+strideq*2], xm5
        pextrd_st_p1::<1>(at(dst, s), m[4]); // pextrd [dstq+strideq*1], xm4, 1
        pextrd_st_p1::<1>(at(dst, s3), m[5]); // pextrd [dstq+stride3q ], xm5, 1
    } else {
        m[6] = _mm256_cmpgt_epi16(m[zero], m[12]); // pcmpgtw m6, %3, m12
        m[5] = _mm256_cmpgt_epi16(m[zero], m[15]); // pcmpgtw m5, %3, m15
        m[12] = _mm256_add_epi16(m[12], m[6]); // paddw m12, m6
        m[15] = _mm256_add_epi16(m[15], m[5]); // paddw m15, m5
        if CLIP {
            m[5] = _mm256_unpackhi_epi8(m[4], m[zero]); // punpckhbw m5, m4, %3
            m[4] = _mm256_unpacklo_epi8(m[4], m[zero]); // punpcklbw m4, %3
        }
        m[12] = _mm256_mulhrs_epi16(m[12], m[pw_2048_]); // pmulhrsw m12, %4
        m[15] = _mm256_mulhrs_epi16(m[15], m[pw_2048_]); // pmulhrsw m15, %4
        if !CLIP {
            m[15] = _mm256_packs_epi16(m[15], m[12]); // packsswb m15, m12
            m[4] = _mm256_add_epi8(m[4], m[15]); // paddb m4, m15
        } else {
            m[5] = _mm256_add_epi16(m[5], m[12]); // paddw m5, m12
            m[4] = _mm256_add_epi16(m[4], m[15]); // paddw m4, m15
            m[4] = _mm256_packus_epi16(m[4], m[5]); // packuswb m4, m5 ; clip px in [0x0,0xff]
            m[4] = _mm256_min_epu8(m[4], m[7]); // pminub m4, m7
            m[4] = _mm256_max_epu8(m[4], m[8]); // pmaxub m4, m8
        }
        m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
        if W == 4 {
            let dst4 = g.dst4;
            movd_st_p1(dst, m[4]); // movd [dstq +strideq*0], xm4
            movd_st_p1(dst4, m[5]); // movd [dst4q+strideq*0], xm5
            pextrd_st_p1::<1>(at(dst, s), m[4]); // pextrd [dstq +strideq*1], xm4, 1
            pextrd_st_p1::<1>(at(dst4, s), m[5]); // pextrd [dst4q+strideq*1], xm5, 1
            pextrd_st_p1::<2>(at(dst, s * 2), m[4]); // pextrd [dstq +strideq*2], xm4, 2
            pextrd_st_p1::<2>(at(dst4, s * 2), m[5]); // pextrd [dst4q+strideq*2], xm5, 2
            pextrd_st_p1::<3>(at(dst, s3), m[4]); // pextrd [dstq +stride3q ], xm4, 3
            pextrd_st_p1::<3>(at(dst4, s3), m[5]); // pextrd [dst4q+stride3q ], xm5, 3
        } else {
            movq_st_p1(dst, m[4]); // movq [dstq+strideq*0], xm4
            movq_st_p1(at(dst, s * 2), m[5]); // movq [dstq+strideq*2], xm5
            movhps_st_p1(at(dst, s), m[4]); // movhps [dstq+strideq*1], xm4
            movhps_st_p1(at(dst, s3), m[5]); // movhps [dstq+stride3q ], xm5
        }
    }
}

// ---- `CDEF_FILTER w, h` from its start to `.border_block` ----

/// The body of `.k_loop` (main path): the taps of `k` for the primary and the secondary direction. The entry point
/// runs it for k = 1 and k = 0, the two passes of `dec kq ; jge .k_loop`: with the pass written out, each
/// `ACCUMULATE_TAP_BYTE`'s table index `dir*2 + k + tap*2` is a constant and the computed `call` becomes the loader's
/// code in place. (With the loop kept, the compiler keeps the dispatch at run time and carries the loaded pixels
/// from one tap to the next in general-purpose registers and the stack: about 1.3-1.7x the assembly's
/// instructions.)
#[inline(always)]
unsafe fn cdef_filter_8bpc_k_loop<const W: usize, const H: usize>(
    m: &mut M,
    g: &GprP1,
    fr: *const u8,
    dir: usize,
    k: isize,
    pri_taps: *const u8,
    sec_taps: *const u8,
) {
    m[2] = vpbroadcastb_p1(pri_taps.wrapping_offset(k)); // vpbroadcastb m2, [priq+kq] ; pri_taps
    m[3] = vpbroadcastb_p1(sec_taps.wrapping_offset(k)); // vpbroadcastb m3, [secq+kq] ; sec_taps
    accumulate_tap_byte::<W, H, true>(m, g, fr, dir, k, 2, 0, 13, 0, 2); // ACCUMULATE_TAP_BYTE 2, [rsp+0], m13, m0, m2, %1, %2, 1 ; dir + 0
    accumulate_tap_byte::<W, H, true>(m, g, fr, dir, k, 4, 8, 14, 1, 3); // ACCUMULATE_TAP_BYTE 4, [rsp+8], m14, m1, m3, %1, %2, 1 ; dir + 2
    accumulate_tap_byte::<W, H, true>(m, g, fr, dir, k, 0, 8, 14, 1, 3); // ACCUMULATE_TAP_BYTE 0, [rsp+8], m14, m1, m3, %1, %2, 1 ; dir - 2
}

/// The body of `.pri_k_loop` (run for k = 1 and k = 0, as `cdef_filter_8bpc_k_loop`).
#[inline(always)]
unsafe fn cdef_filter_8bpc_pri_k_loop<const W: usize, const H: usize>(
    m: &mut M,
    g: &GprP1,
    fr: *const u8,
    dir: usize,
    k: isize,
    pri_taps: *const u8,
) {
    m[2] = vpbroadcastb_p1(pri_taps.wrapping_offset(k)); // vpbroadcastb m2, [priq+kq] ; pri_taps
    accumulate_tap_byte::<W, H, false>(m, g, fr, dir, k, 2, 0, 13, 0, 2); // ACCUMULATE_TAP_BYTE 2, [rsp+0], m13, m0, m2, %1, %2 ; dir + 0
}

/// The body of `.sec_k_loop` (run for k = 1 and k = 0, as `cdef_filter_8bpc_k_loop`).
#[inline(always)]
unsafe fn cdef_filter_8bpc_sec_k_loop<const W: usize, const H: usize>(
    m: &mut M,
    g: &GprP1,
    fr: *const u8,
    dir: usize,
    k: isize,
    sec_taps: *const u8,
) {
    m[3] = vpbroadcastb_p1(sec_taps.wrapping_offset(k)); // vpbroadcastb m3, [secq+kq] ; sec_taps
    accumulate_tap_byte::<W, H, false>(m, g, fr, dir, k, 4, 8, 14, 1, 3); // ACCUMULATE_TAP_BYTE 4, [rsp+8], m14, m1, m3, %1, %2 ; dir + 2
    accumulate_tap_byte::<W, H, false>(m, g, fr, dir, k, 0, 8, 14, 1, 3); // ACCUMULATE_TAP_BYTE 0, [rsp+8], m14, m1, m3, %1, %2 ; dir - 2
}

/// The end of a `.v_loop` iteration at 8x8 (`%if %1*%2 > mmsize`): the next four rows, the copies of rows 2 and 3
/// (pre-filter) in the frame as the rows above, and the caller's `bot`. Returns whether to loop.
#[inline(always)]
unsafe fn cdef_filter_8bpc_next_rows(g: &mut GprP1, fr: *const u8, bot: *const u8) -> bool {
    g.dst = at(g.dst, g.stride * 4); // lea dstq, [dstq+strideq*4]
    g.top = a_p1(fr, 0xa0); // lea top1q, [rsp+0xa0]
    g.top2 = a_p1(fr, 0xb0); // lea top2q, [rsp+0xb0]
    g.bot = bot; // mov botq, [rsp+16]
    g.h += 4; // add hq, 4
    g.h < 0 // jl .v_loop
}

/// `cdef_filter_WxH_8bpc` with `edges == 0xf` (after `jne .border_block`): the entry, the main path (pri and sec),
/// `.pri_only` and `.sec_only`. `F` is the size of the frame in bytes.
#[inline(always)]
unsafe fn cdef_filter_8bpc<const W: usize, const H: usize, const F: usize>(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: usize,
    damping: c_int,
) {
    // The frame (`ALLOC_STACK`), zeroed whole: the tap loaders' blends read bytes of it that the assembly never
    // writes (at 8x8 [rsp+0x18..0x20], at 4x8 [rsp+0x0..0x10] when a shift slot is unused and [rsp+0x30..0x38]
    // beyond its `ALLOC_STACK` area) and discard them; with the zeros no load reads uninitialized memory.
    let mut frame = FrameP1::<F>([0; F]);
    let fr: *mut u8 = frame.0.as_mut_ptr();
    let mut m = regs();
    // Deviation (memory safety): the tap loaders and the entry read `left` at offsets -16 to +20 (32-byte
    // `pblendw` operands at `[leftq-16+…]`, `pmovzxbw [leftq-7]`, 16 bytes at `[leftq+5]`, …), beyond the `h`
    // rows of two pixels the caller hands over (the decoder's `lr_bak[bit][pl]`, 8 rows inside a stack array of
    // 96 bytes, so before its start or behind its end for the first and the last plane). The bytes outside the
    // rows are discarded by the blends and shifts that follow. The rebuild reads exactly the `h` rows (the C
    // kernel's set) into a zeroed local with 16 bytes before and 32 behind them and runs the assembly's
    // accesses on that copy: same values in every lane that reaches the output, five instructions more.
    let mut left_buf = [0u8; 64];
    core::ptr::copy_nonoverlapping(left as *const u8, left_buf.as_mut_ptr().add(16), H * 2);
    let left = left_buf.as_ptr().add(16);
    let mut g = GprP1 {
        dst,
        stride,
        left,
        top,
        bot,
        dst4: dst,
        stride3: 0,
        top2: top,
        h: 0,
    };
    // `mov [rsp+16], botq` (8x8): kept in a local instead of the frame (the frame's bytes 0x10..0x18 stay zero;
    // the one loader that reads them, `.d4k1` at the first rows, discards them in its blend).
    let mut bot_saved = bot;
    if H == 4 {
        // PUSH r11 ; PUSH r12 ; ALLOC_STACK 0x60, 16
        m[0] = zx(_mm_cvtepu8_epi16(movq_p1(a_p1(left, 1)))); // pmovzxbw xm0, [leftq+1]
        m[0] = _mm256_permute4x64_epi64::<Q0110_P1>(m[0]); // vpermq m0, m0, q0110
        m[1] = _mm256_bsrli_epi128::<4>(m[0]); // psrldq m1, m0, 4
        m[2] = _mm256_alignr_epi8::<12>(m[0], m[0]); // vpalignr m2, m0, m0, 12
        _mm256_storeu_si256(at(fr, 0x10).cast(), m[0]); // movu [rsp+0x10], m0
        _mm256_storeu_si256(at(fr, 0x28).cast(), m[1]); // movu [rsp+0x28], m1
        _mm256_storeu_si256(at(fr, 0x40).cast(), m[2]); // movu [rsp+0x40], m2
    } else if W == 4 {
        // PUSH r13 ; ALLOC_STACK 8*2+%1*%2*1, 16
        m[0] = _mm256_cvtepu16_epi32(x_p1(left)); // pmovzxwd m0, [leftq]
        _mm256_storeu_si256(at(fr, 0x10).cast(), m[0]); // mova [rsp+0x10], m0
    } else {
        // PUSH r13 ; PUSH r14 ; ALLOC_STACK 8*4+%1*%2*2+32, 16
        let r11 = stride * 3; // lea r11, [strideq*3]
        m[4] = zx(x_p1(a_p1(dst, stride * 2))); // movu xm4, [dstq+strideq*2]
        m[0] = _mm256_cvtepu16_epi64(movq_p1(a_p1(left, 0))); // pmovzxwq m0, [leftq+0]
        m[1] = _mm256_cvtepu16_epi64(movq_p1(a_p1(left, 8))); // pmovzxwq m1, [leftq+8]
        m[4] = vinserti128_p1(m[4], a_p1(dst, r11)); // vinserti128 m4, [dstq+r11], 1
        m[2] = _mm256_cvtepu8_epi32(movq_p1(a_p1(left, 1))); // pmovzxbd m2, [leftq+1]
        m[3] = _mm256_cvtepu8_epi32(movq_p1(a_p1(left, 9))); // pmovzxbd m3, [leftq+9]
        bot_saved = bot; // mov [rsp+16], botq
        _mm256_storeu_si256(at(fr, 0x20).cast(), m[0]); // mova [rsp+0x20], m0
        _mm256_storeu_si256(at(fr, 0x40).cast(), m[1]); // mova [rsp+0x40], m1
        _mm256_storeu_si256(at(fr, 0x60).cast(), m[2]); // mova [rsp+0x60], m2
        _mm256_storeu_si256(at(fr, 0x80).cast(), m[3]); // mova [rsp+0x80], m3
        _mm256_storeu_si256(at(fr, 0xa0).cast(), m[4]); // mova [rsp+0xa0], m4
        g.bot = at(dst, stride * 4); // lea botq, [dstq+strideq*4]
        g.stride3 = r11; // r11 is stride3q in PREP_REGS' DEFINE_ARGS
    }

    // DEFINE_ARGS dst, stride, left, top, bot, pri, secdmp, zero, pridmp, damping
    let damping = damping.wrapping_sub(31); // mov dampingd, r8m ; sub dampingd, 31
    let zero = 0; // xor zerod, zerod
    let secdmp = sec; // movifnidn secdmpd, secdmpm

    // mov dird, r7m (PREP_REGS): `dir` is the argument.
    if pri != 0 {
        // test prid, prid ; jz .sec_only
        m[0] = zx(_mm_cvtsi32_si128(pri)); // movd xm0, prid

        // lzcnt: `u32::leading_zeros` is lzcnt's value for every operand (32 for zero), here pri != 0.
        let mut pridmp = (pri as u32).leading_zeros() as i32; // lzcnt pridmpd, prid
        pridmp = pridmp.wrapping_add(damping); // add pridmpd, dampingd
        if pridmp < 0 {
            pridmp = zero; // cmovs pridmpd, zerod
        }
        fr.cast::<u64>().write(pridmp as u32 as u64); // mov [rsp+0], pridmpq ; pri_shift
        if secdmp != 0 {
            // test secdmpd, secdmpd ; jz .pri_only
            m[1] = zx(_mm_cvtsi32_si128(secdmp)); // movd xm1, secdmpd
            let secdmp = ((secdmp as u32).leading_zeros() as i32).wrapping_add(damping); // lzcnt ; add secdmpd, dampingd
            at(fr, 8).cast::<u64>().write(secdmp as u32 as u64); // mov [rsp+8], secdmpq ; sec_shift

            // lea tableq, [tap_table]. The shifts are damping - ulog2(strength) <= 6 by contract (damping 3..6),
            // so the byte read lies in `tap_table` (42 bytes).
            m[13] = vpbroadcastb_p1(ro(tap_table, pridmp as u32 as isize)); // vpbroadcastb m13, [tableq+pridmpq] ; pri_shift_mask
            m[14] = vpbroadcastb_p1(ro(tap_table, secdmp as u32 as isize)); // vpbroadcastb m14, [tableq+secdmpq] ; sec_shift_mask

            // pri/sec_taps[k] [4 total]
            m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0 ; pri_strength
            m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1 ; sec_strength
            let pri = pri & 1; // and prid, 1
            let pri_taps = ro(tap_table, pri as isize * 2 + 8); // lea priq, [tableq+priq*2+8] ; pri_taps
            let sec_taps = ro(tap_table, 12); // lea secq, [tableq+12] ; sec_taps

            prep_regs::<W, H>(&mut g, top); // PREP_REGS %1, %2
            loop {
                // .v_loop:
                let k = load_block::<W, H, true>(&mut m, &g); // LOAD_BLOCK %1, %2, 1

                // `.k_loop:` … `dec kq ; jge .k_loop` runs its body for k = 1 and k = 0; the two
                // passes are written out, so that every jump-table index is a constant (see `cdef_filter_8bpc_k_loop`).
                cdef_filter_8bpc_k_loop::<W, H>(&mut m, &g, fr, dir, k, pri_taps, sec_taps); // k = 1
                cdef_filter_8bpc_k_loop::<W, H>(&mut m, &g, fr, dir, k - 1, pri_taps, sec_taps); // dec kq ; jge .k_loop: k = 0

                m[10] = vpbroadcastd_p1(ro(pw_2048, 0)); // vpbroadcastd m10, [pw_2048]
                m[9] = _mm256_setzero_si256(); // pxor m9, m9
                adjust_pixel::<W, H, true>(&mut m, &g, 9, 10); // ADJUST_PIXEL %1, %2, m9, m10, 1
                if !(W * H > 32 && cdef_filter_8bpc_next_rows(&mut g, fr, bot_saved)) {
                    break;
                }
            }
            return; // RET
        }

        // .pri_only:
        // DEFINE_ARGS dst, stride, left, top, bot, pri, _, table, pridmp
        m[13] = vpbroadcastb_p1(ro(tap_table, pridmp as u32 as isize)); // lea tableq, [tap_table] ; vpbroadcastb m13, [tableq+pridmpq] ; pri_shift_mask

        // pri/sec_taps[k] [4 total]
        m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0 ; pri_strength
        let pri = pri & 1; // and prid, 1
        let pri_taps = ro(tap_table, pri as isize * 2 + 8); // lea priq, [tableq+priq*2+8] ; pri_taps
        prep_regs::<W, H>(&mut g, top); // PREP_REGS %1, %2
        m[3] = vpbroadcastd_p1(ro(pw_2048, 0)); // vpbroadcastd m3, [pw_2048]
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        loop {
            // .pri_v_loop:
            let k = load_block::<W, H, false>(&mut m, &g); // LOAD_BLOCK %1, %2

            // `.pri_k_loop:` … `dec kq ; jge .pri_k_loop` runs its body for k = 1 and k = 0; the two
            // passes are written out, so that every jump-table index is a constant (see `cdef_filter_8bpc_k_loop`).
            cdef_filter_8bpc_pri_k_loop::<W, H>(&mut m, &g, fr, dir, k, pri_taps); // k = 1
            cdef_filter_8bpc_pri_k_loop::<W, H>(&mut m, &g, fr, dir, k - 1, pri_taps); // dec kq ; jge .pri_k_loop: k = 0
            adjust_pixel::<W, H, false>(&mut m, &g, 1, 3); // ADJUST_PIXEL %1, %2, m1, m3
            if !(W * H > 32 && cdef_filter_8bpc_next_rows(&mut g, fr, bot_saved)) {
                break;
            }
        }
        return; // RET
    }

    // .sec_only:
    // DEFINE_ARGS dst, stride, left, top, bot, _, secdmp, zero, _, damping
    m[1] = zx(_mm_cvtsi32_si128(secdmp)); // movd xm1, secdmpd

    // lzcnt: `u32::leading_zeros` is lzcnt's value for every operand (32 for zero).
    let secdmp = ((secdmp as u32).leading_zeros() as i32).wrapping_add(damping); // lzcnt secdmpd, secdmpd ; add secdmpd, dampingd
    at(fr, 8).cast::<u64>().write(secdmp as u32 as u64); // mov [rsp+8], secdmpq ; sec_shift

    // DEFINE_ARGS dst, stride, left, top, bot, _, secdmp, table
    m[14] = vpbroadcastb_p1(ro(tap_table, secdmp as u32 as isize)); // lea tableq, [tap_table] ; vpbroadcastb m14, [tableq+secdmpq] ; sec_shift_mask

    // pri/sec_taps[k] [4 total]
    m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1 ; sec_strength
    let sec_taps = ro(tap_table, 12); // lea secq, [tableq+12] ; sec_taps
    prep_regs::<W, H>(&mut g, top); // PREP_REGS %1, %2
    m[2] = vpbroadcastd_p1(ro(pw_2048, 0)); // vpbroadcastd m2, [pw_2048]
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    loop {
        // .sec_v_loop:
        let k = load_block::<W, H, false>(&mut m, &g); // LOAD_BLOCK %1, %2

        // `.sec_k_loop:` … `dec kq ; jge .sec_k_loop` runs its body for k = 1 and k = 0; the two
        // passes are written out, so that every jump-table index is a constant (see `cdef_filter_8bpc_k_loop`).
        cdef_filter_8bpc_sec_k_loop::<W, H>(&mut m, &g, fr, dir, k, sec_taps); // k = 1
        cdef_filter_8bpc_sec_k_loop::<W, H>(&mut m, &g, fr, dir, k - 1, sec_taps); // dec kq ; jge .sec_k_loop: k = 0
        adjust_pixel::<W, H, false>(&mut m, &g, 0, 2); // ADJUST_PIXEL %1, %2, m0, m2
        if !(W * H > 32 && cdef_filter_8bpc_next_rows(&mut g, fr, bot_saved)) {
            break;
        }
    }
    // RET
}

/// `cdef_filter_8x8_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_cdef_filter_8x8_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    if edges != 0xf {
        // mov edged, edgem ; cmp edged, 0xf ; jne .border_block
        return cdef_filter_8x8_8bpc_border_block(dst, stride, left, top, bot, pri, sec, dir, damping, edges);
    }
    const F: usize = 8 * 4 + 8 * 8 * 2 + 32;
    // `dir & 7`: the contract keeps `dir` in 0..=7, where it is `dir`; outside it the assembly would jump through
    // bytes beyond its table.
    cdef_filter_8bpc::<8, 8, F>(dst, stride, left, top, bot, pri, sec, (dir & 7) as usize, damping)
}

/// `cdef_filter_4x8_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_cdef_filter_4x8_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    if edges != 0xf {
        // mov edged, edgem ; cmp edged, 0xf ; jne .border_block
        return cdef_filter_4x8_8bpc_border_block(dst, stride, left, top, bot, pri, sec, dir, damping, edges);
    }
    // ALLOC_STACK 8*2+4*8*1 = 0x30 bytes, plus 8: `.d0k1` reads [rsp+0x18..0x38] (blend discards the last 8).
    const F: usize = 8 * 2 + 4 * 8 * 1 + 8;
    // `dir & 7`: the contract keeps `dir` in 0..=7, where it is `dir`; outside it the assembly would jump through
    // bytes beyond its table.
    cdef_filter_8bpc::<4, 8, F>(dst, stride, left, top, bot, pri, sec, (dir & 7) as usize, damping)
}

/// `cdef_filter_4x4_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_cdef_filter_4x4_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    if edges != 0xf {
        // mov edged, edgem ; cmp edged, 0xf ; jne .border_block
        return cdef_filter_4x4_8bpc_border_block(dst, stride, left, top, bot, pri, sec, dir, damping, edges);
    }
    const F: usize = 0x60;
    // `dir & 7`: the contract keeps `dir` in 0..=7, where it is `dir`; outside it the assembly would jump through
    // bytes beyond its table.
    cdef_filter_8bpc::<4, 4, F>(dst, stride, left, top, bot, pri, sec, (dir & 7) as usize, damping)
}

// ---- part 2: `.border_block` of `cdef_filter_{8x8,4x8,4x4}_8bpc` (edges != 0xf): `cdef_avx2.asm` lines
// 280–395 (BORDER_PREP_REGS, BORDER_LOAD_BLOCK, ACCUMULATE_TAP_WORD, BORDER_ADJUST_PIXEL) and 1195–1562; and
// `cdef_dir_8bpc` with its label `.main` (lines 1566–1771) ----

// ---- part 2 helpers (candidates for the prelude; suffix `_p2` until the lead moves them) ----

/// `vpbroadcastd m, [label+off]`: a read-only dword broadcast (the 4 bytes lie inside `RODATA`: constant
/// offsets, checked at each use).
#[inline(always)]
unsafe fn vpbroadcastd_ro_p2(label: usize, off: isize) -> __m256i {
    _mm256_set1_epi32(ro(label, off).cast::<i32>().read_unaligned())
}

/// `vbroadcasti128 m, [label+off]`: 16 read-only bytes in both lanes (inside `RODATA`, constant offsets).
#[inline(always)]
unsafe fn vbroadcasti128_ro_p2(label: usize, off: isize) -> __m256i {
    _mm256_broadcastsi128_si256(_mm_loadu_si128(ro(label, off).cast()))
}

/// `vpbroadcastq m, [p]`: 8 bytes of caller memory in all four quadwords.
#[inline(always)]
unsafe fn vpbroadcastq_ld_p2(p: *const u8) -> __m256i {
    _mm256_set1_epi64x(p.cast::<i64>().read_unaligned())
}

/// `movq xm, [p]`: 8 bytes of caller memory, the rest of the register zero.
#[inline(always)]
unsafe fn movq_ld_p2(p: *const u8) -> __m256i {
    zx(_mm_loadl_epi64(p.cast()))
}

// ---- `.border_block` (edges != 0xf) ----

/// `pmovzxbw xm, [p]`: 8 bytes widened to 8 words, the upper half of the register zero.
#[inline(always)]
unsafe fn pmovzxbw_x_p2(p: *const u8) -> __m256i {
    zx(_mm_cvtepu8_epi16(_mm_loadl_epi64(p.cast())))
}

/// `pmovzxbw m, [p]` (`ymm`): 16 bytes widened to 16 words.
#[inline(always)]
unsafe fn pmovzxbw_y_p2(p: *const u8) -> __m256i {
    _mm256_cvtepu8_epi16(_mm_loadu_si128(p.cast()))
}

/// `pmovzxbw m, [p]` in the padding code, where `m` is `xmm` for w 4 (`INIT_XMM`) and `ymm` for w 8.
#[inline(always)]
unsafe fn pmovzxbw_m_p2<const W: usize>(p: *const u8) -> __m256i {
    if W == 4 {
        pmovzxbw_x_p2(p)
    } else {
        pmovzxbw_y_p2(p)
    }
}

/// `movu [p], m` / `mova [p], m` in the padding code (`xmm` for w 4, `ymm` for w 8).
#[inline(always)]
unsafe fn movu_st_m_p2<const W: usize>(p: *mut u8, x: __m256i) {
    if W == 4 {
        _mm_storeu_si128(p.cast(), lo(x))
    } else {
        _mm256_storeu_si256(p.cast(), x)
    }
}

/// `mova [p], xm` / `movu [p], xm`: 16 bytes.
#[inline(always)]
unsafe fn movu_st_x_p2(p: *mut u8, x: __m256i) {
    _mm_storeu_si128(p.cast(), lo(x))
}

/// `movq [p], xm`: 8 bytes.
#[inline(always)]
unsafe fn movq_st_p2(p: *mut u8, x: __m256i) {
    _mm_storel_epi64(p.cast(), lo(x))
}

/// `movd xm, [p]`: 4 bytes, the rest of the register zero.
#[inline(always)]
unsafe fn movd_ld_p2(p: *const u8) -> __m256i {
    zx(_mm_cvtsi32_si128(p.cast::<i32>().read_unaligned()))
}

/// `movd [p], xm`: 4 bytes.
#[inline(always)]
unsafe fn movd_st_p2(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(lo(x)))
}

/// `pinsrd xm, [p], I`: 4 bytes into dword `I`, the rest kept.
#[inline(always)]
unsafe fn pinsrd_ld_p2<const I: i32>(x: __m128i, p: *const u8) -> __m128i {
    _mm_insert_epi32::<I>(x, p.cast::<i32>().read_unaligned())
}

/// `pextrd [p], xm, I`: dword `I`, 4 bytes.
#[inline(always)]
unsafe fn pextrd_st_p2<const I: i32>(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_extract_epi32::<I>(lo(x)))
}

/// `movhps xm, [p]`: 8 bytes into the upper half of the `xmm` register, the lower half kept. (Written with an
/// unaligned 8-byte read: `_mm_loadh_pd` reads through an `f64` pointer, which `px` at a word offset is not;
/// this form compiles to `vmovhps`, where `_mm_unpacklo_epi64` with `_mm_loadl_epi64` gives two instructions.)
#[inline(always)]
unsafe fn movhps_ld_p2(x: __m128i, p: *const u8) -> __m128i {
    let x = _mm_castsi128_pd(x);
    _mm_castpd_si128(_mm_setr_pd(_mm_cvtsd_f64(x), p.cast::<f64>().read_unaligned()))
}

/// `movhps [p], xm`: the upper 8 bytes of the `xmm` register.
#[inline(always)]
unsafe fn movhps_st_p2(p: *mut u8, x: __m256i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(lo(x)))
}

/// `p + off` bytes of a read-only pointer, wrapping (the prelude's `at` for `*const`).
#[inline(always)]
fn atc_p2(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `[tap_table+idx]`, one byte. In bounds by the contract: `idx` is a shift (`0..=7`: `pri_shift` and
/// `sec_shift` are at most `damping`, which `cdef_apply` keeps at most 6 at 8 bpc), a tap index
/// (`8..=11`, `12..=13`) or a direction offset (`14 + dir*2 + k + 0/4/12 <= 41` for `dir` in `0..=7`); the
/// table is 42 bytes long. The assembly reads the same byte.
#[inline(always)]
unsafe fn tap_p2(idx: usize) -> u8 {
    debug_assert!(idx < 42);
    ro(tap_table, idx as isize).read()
}

/// `%macro BORDER_LOAD_BLOCK w, h, init_min_max`: `m4` = the rows at `stk` (2 rows of 8 or 4 rows of 4 words),
/// `m15` (sum) = 0, with `INIT` `m7` (max) = `m8` (min) = `m4`. Sets `k` = 1.
#[inline(always)]
unsafe fn border_load_block<const W: usize, const INIT: bool>(m: &mut M, stk: *mut u8, k: &mut isize) {
    *k = 1; // mov kd, 1
    if W == 4 {
        m[4] = movq_ld_p2(at(stk, 32 * 0)); // movq xm4, [stkq+32*0]
        m[4] = zx(movhps_ld_p2(lo(m[4]), at(stk, 32 * 1))); // movhps xm4, [stkq+32*1]
        m[5] = movq_ld_p2(at(stk, 32 * 2)); // movq xm5, [stkq+32*2]
        m[5] = zx(movhps_ld_p2(lo(m[5]), at(stk, 32 * 3))); // movhps xm5, [stkq+32*3]
        m[4] = _mm256_inserti128_si256::<1>(m[4], lo(m[5])); // vinserti128 m4, xm5, 1
    } else {
        m[4] = zx(_mm_loadu_si128(at(stk, 32 * 0).cast())); // mova xm4, [stkq+32*0] ; px
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(stk, 32 * 1).cast()));
        // vinserti128 m4, [stkq+32*1], 1
    }
    m[15] = _mm256_setzero_si256(); // pxor m15, m15 ; sum
    if INIT {
        m[7] = m[4]; // mova m7, m4 ; max
        m[8] = m[4]; // mova m8, m4 ; min
    }
}

/// `%macro ACCUMULATE_TAP_WORD tap_offset, shift, mask, strength, mul_tap, w, clip`: `shift` is the
/// shift count operand (`[rsp+0]` or `[rsp+8]`: only its low quadword counts), `mask`, `strength` and
/// `mul_tap` are register numbers. `dir` is `dirq` (`tap_table + dir*2 + 14`).
#[inline(always)]
unsafe fn accumulate_tap_word<const W: usize, const CLIP: bool>(
    m: &mut M,
    stk: *mut u8,
    dir: usize,
    k: isize,
    tap_offset: usize,
    shift: __m128i,
    mask: usize,
    strength: usize,
    mul_tap: usize,
) {
    // load p0/p1
    let mut off = tap_p2(dir + k as usize + tap_offset) as i8 as isize; // movsx offq, byte [dirq+kq+%1] ; off1
    if W == 4 {
        m[5] = movq_ld_p2(at(stk, off * 2 + 32 * 0)); // movq xm5, [stkq+offq*2+32*0] ; p0
        m[6] = movq_ld_p2(at(stk, off * 2 + 32 * 2)); // movq xm6, [stkq+offq*2+32*2]
        m[5] = zx(movhps_ld_p2(lo(m[5]), at(stk, off * 2 + 32 * 1))); // movhps xm5, [stkq+offq*2+32*1]
        m[6] = zx(movhps_ld_p2(lo(m[6]), at(stk, off * 2 + 32 * 3))); // movhps xm6, [stkq+offq*2+32*3]
        m[5] = _mm256_inserti128_si256::<1>(m[5], lo(m[6])); // vinserti128 m5, xm6, 1
    } else {
        m[5] = zx(_mm_loadu_si128(at(stk, off * 2 + 32 * 0).cast())); // movu xm5, [stkq+offq*2+32*0] ; p0
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(stk, off * 2 + 32 * 1).cast()));
        // vinserti128 m5, [stkq+offq*2+32*1], 1
    }
    off = -off; // neg offq ; -off1
    if W == 4 {
        m[6] = movq_ld_p2(at(stk, off * 2 + 32 * 0)); // movq xm6, [stkq+offq*2+32*0] ; p1
        m[9] = movq_ld_p2(at(stk, off * 2 + 32 * 2)); // movq xm9, [stkq+offq*2+32*2]
        m[6] = zx(movhps_ld_p2(lo(m[6]), at(stk, off * 2 + 32 * 1))); // movhps xm6, [stkq+offq*2+32*1]
        m[9] = zx(movhps_ld_p2(lo(m[9]), at(stk, off * 2 + 32 * 3))); // movhps xm9, [stkq+offq*2+32*3]
        m[6] = _mm256_inserti128_si256::<1>(m[6], lo(m[9])); // vinserti128 m6, xm9, 1
    } else {
        m[6] = zx(_mm_loadu_si128(at(stk, off * 2 + 32 * 0).cast())); // movu xm6, [stkq+offq*2+32*0] ; p1
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(stk, off * 2 + 32 * 1).cast()));
        // vinserti128 m6, [stkq+offq*2+32*1], 1
    }
    if CLIP {
        // out of bounds values are set to a value that is a both a large unsigned value and a negative
        // signed value. use signed max and unsigned min to remove them
        m[7] = _mm256_max_epi16(m[7], m[5]); // pmaxsw m7, m5 ; max after p0
        m[8] = _mm256_min_epu16(m[8], m[5]); // pminuw m8, m5 ; min after p0
        m[7] = _mm256_max_epi16(m[7], m[6]); // pmaxsw m7, m6 ; max after p1
        m[8] = _mm256_min_epu16(m[8], m[6]); // pminuw m8, m6 ; min after p1
    }

    // accumulate sum[m15] over p0/p1; calculate difference before converting
    m[5] = _mm256_sub_epi16(m[5], m[4]); // psubw m5, m4 ; diff_p0(p0 - px)
    m[6] = _mm256_sub_epi16(m[6], m[4]); // psubw m6, m4 ; diff_p1(p1 - px)

    // convert to 8-bits with signed saturation; saturating to large diffs has no impact on the results
    m[5] = _mm256_packs_epi16(m[5], m[6]); // packsswb m5, m6

    // group into pairs so we can accumulate using maddubsw
    m[5] = _mm256_shuffle_epi8(m[5], m[12]); // pshufb m5, m12
    m[9] = _mm256_abs_epi8(m[5]); // pabsb m9, m5
    m[10] = _mm256_sign_epi8(m[mul_tap], m[5]); // psignb m10, %5, m5
    m[5] = _mm256_srl_epi16(m[9], shift); // psrlw m5, m9, %2 ; emulate 8-bit shift
    m[5] = _mm256_and_si256(m[5], m[mask]); // pand m5, %3
    m[5] = _mm256_subs_epu8(m[strength], m[5]); // psubusb m5, %4, m5

    // use unsigned min since abs diff can equal 0x80
    m[5] = _mm256_min_epu8(m[5], m[9]); // pminub m5, m9
    m[5] = _mm256_maddubs_epi16(m[5], m[10]); // pmaddubsw m5, m10
    m[15] = _mm256_add_epi16(m[15], m[5]); // paddw m15, m5
}

/// `%macro BORDER_ADJUST_PIXEL w, pw_2048, clip`: `pw_2048` is a register number. Writes 2 rows of 8 or 4 rows
/// of 4 pixels at `dst`.
#[inline(always)]
unsafe fn border_adjust_pixel<const W: usize, const CLIP: bool>(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    stride3: isize,
    pw_2048_: usize,
) {
    m[9] = _mm256_cmpgt_epi16(m[11], m[15]); // pcmpgtw m9, m11, m15
    m[15] = _mm256_add_epi16(m[15], m[9]); // paddw m15, m9
    m[15] = _mm256_mulhrs_epi16(m[15], m[pw_2048_]); // pmulhrsw m15, %2
    m[4] = _mm256_add_epi16(m[4], m[15]); // paddw m4, m15
    if CLIP {
        m[4] = _mm256_min_epi16(m[4], m[7]); // pminsw m4, m7
        m[4] = _mm256_max_epi16(m[4], m[8]); // pmaxsw m4, m8
    }
    m[4] = _mm256_packus_epi16(m[4], m[4]); // packuswb m4, m4
    m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
    if W == 4 {
        movd_st_p2(at(dst, stride * 0), m[4]); // movd [dstq+strideq*0], xm4
        pextrd_st_p2::<1>(at(dst, stride * 1), m[4]); // pextrd [dstq+strideq*1], xm4, 1
        movd_st_p2(at(dst, stride * 2), m[5]); // movd [dstq+strideq*2], xm5
        pextrd_st_p2::<1>(at(dst, stride3), m[5]); // pextrd [dstq+stride3q ], xm5, 1
    } else {
        movq_st_p2(at(dst, stride * 0), m[4]); // movq [dstq+strideq*0], xm4
        movq_st_p2(at(dst, stride * 1), m[5]); // movq [dstq+strideq*1], xm5
    }
}

/// `.border_block` of `CDEF_FILTER w, h` (`W`, `H`), from the label to the three `RET`s. `frame` is the
/// `ALLOC_STACK 2*16+(h+4)*32` area (32-byte aligned, zeroed by the caller); `px` is `frame + 2*16+2*32`, the
/// block's padded copy as words, 16 per row (32 bytes; columns -2 and -1 of a row are words 14 and 15 of the
/// row before), rows -2..h+1. `[rsp+0]` and `[rsp+8]` (`pri_shift`, `sec_shift`) are locals here.
#[inline(always)]
unsafe fn cdef_filter_8bpc_border_block<const W: usize, const H: usize>(
    m: &mut M,
    frame: *mut u8,
    mut dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edge: c_uint,
) {
    let w = W as isize;
    let h = H as isize;
    let px = at(frame, 2 * 16 + 2 * 32); // %define px rsp+2*16+2*32
    let left = left as *const u8;

    m[14] = _mm256_cmpeq_epi16(m[14], m[14]); // pcmpeqw m14, m14
    m[14] = _mm256_slli_epi16::<15>(m[14]); // psllw m14, 15 ; 0x8000

    // prepare pixel buffers - body/right (INIT_XMM for w 4: `m` is `xmm` until the filter)
    let dst4 = if H == 8 { at(dst, stride * 4) } else { dst }; // lea dst4q, [dstq+strideq*4] (h 8 only)
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    if edge & 2 != 0 {
        // test edgeb, 2 ; have_right
        m[1] = pmovzxbw_m_p2::<W>(at(dst, stride * 0)); // pmovzxbw m1, [dstq+strideq*0]
        m[2] = pmovzxbw_m_p2::<W>(at(dst, stride * 1)); // pmovzxbw m2, [dstq+strideq*1]
        m[3] = pmovzxbw_m_p2::<W>(at(dst, stride * 2)); // pmovzxbw m3, [dstq+strideq*2]
        m[4] = pmovzxbw_m_p2::<W>(at(dst, stride3)); // pmovzxbw m4, [dstq+stride3q]
        movu_st_m_p2::<W>(at(px, 0 * 32), m[1]); // mova [px+0*32], m1
        movu_st_m_p2::<W>(at(px, 1 * 32), m[2]); // mova [px+1*32], m2
        movu_st_m_p2::<W>(at(px, 2 * 32), m[3]); // mova [px+2*32], m3
        movu_st_m_p2::<W>(at(px, 3 * 32), m[4]); // mova [px+3*32], m4
        if H == 8 {
            m[1] = pmovzxbw_m_p2::<W>(at(dst4, stride * 0)); // pmovzxbw m1, [dst4q+strideq*0]
            m[2] = pmovzxbw_m_p2::<W>(at(dst4, stride * 1)); // pmovzxbw m2, [dst4q+strideq*1]
            m[3] = pmovzxbw_m_p2::<W>(at(dst4, stride * 2)); // pmovzxbw m3, [dst4q+strideq*2]
            m[4] = pmovzxbw_m_p2::<W>(at(dst4, stride3)); // pmovzxbw m4, [dst4q+stride3q]
            movu_st_m_p2::<W>(at(px, 4 * 32), m[1]); // mova [px+4*32], m1
            movu_st_m_p2::<W>(at(px, 5 * 32), m[2]); // mova [px+5*32], m2
            movu_st_m_p2::<W>(at(px, 6 * 32), m[3]); // mova [px+6*32], m3
            movu_st_m_p2::<W>(at(px, 7 * 32), m[4]); // mova [px+7*32], m4
        }
        // jmp .body_done
    } else {
        // .no_right:
        if W == 4 {
            m[1] = movd_ld_p2(at(dst, stride * 0)); // movd xm1, [dstq+strideq*0]
            m[2] = movd_ld_p2(at(dst, stride * 1)); // movd xm2, [dstq+strideq*1]
            m[3] = movd_ld_p2(at(dst, stride * 2)); // movd xm3, [dstq+strideq*2]
            m[4] = movd_ld_p2(at(dst, stride3)); // movd xm4, [dstq+stride3q]
            m[1] = zx(_mm_cvtepu8_epi16(lo(m[1]))); // pmovzxbw xm1, xm1
            m[2] = zx(_mm_cvtepu8_epi16(lo(m[2]))); // pmovzxbw xm2, xm2
            m[3] = zx(_mm_cvtepu8_epi16(lo(m[3]))); // pmovzxbw xm3, xm3
            m[4] = zx(_mm_cvtepu8_epi16(lo(m[4]))); // pmovzxbw xm4, xm4
            movq_st_p2(at(px, 0 * 32), m[1]); // movq [px+0*32], xm1
            movq_st_p2(at(px, 1 * 32), m[2]); // movq [px+1*32], xm2
            movq_st_p2(at(px, 2 * 32), m[3]); // movq [px+2*32], xm3
            movq_st_p2(at(px, 3 * 32), m[4]); // movq [px+3*32], xm4
        } else {
            m[1] = pmovzxbw_x_p2(at(dst, stride * 0)); // pmovzxbw xm1, [dstq+strideq*0]
            m[2] = pmovzxbw_x_p2(at(dst, stride * 1)); // pmovzxbw xm2, [dstq+strideq*1]
            m[3] = pmovzxbw_x_p2(at(dst, stride * 2)); // pmovzxbw xm3, [dstq+strideq*2]
            m[4] = pmovzxbw_x_p2(at(dst, stride3)); // pmovzxbw xm4, [dstq+stride3q]
            movu_st_x_p2(at(px, 0 * 32), m[1]); // mova [px+0*32], xm1
            movu_st_x_p2(at(px, 1 * 32), m[2]); // mova [px+1*32], xm2
            movu_st_x_p2(at(px, 2 * 32), m[3]); // mova [px+2*32], xm3
            movu_st_x_p2(at(px, 3 * 32), m[4]); // mova [px+3*32], xm4
        }
        movd_st_p2(at(px, 0 * 32 + w * 2), m[14]); // movd [px+0*32+%1*2], xm14
        movd_st_p2(at(px, 1 * 32 + w * 2), m[14]); // movd [px+1*32+%1*2], xm14
        movd_st_p2(at(px, 2 * 32 + w * 2), m[14]); // movd [px+2*32+%1*2], xm14
        movd_st_p2(at(px, 3 * 32 + w * 2), m[14]); // movd [px+3*32+%1*2], xm14
        if H == 8 {
            if W == 4 {
                m[1] = movd_ld_p2(at(dst4, stride * 0)); // movd xm1, [dst4q+strideq*0]
                m[2] = movd_ld_p2(at(dst4, stride * 1)); // movd xm2, [dst4q+strideq*1]
                m[3] = movd_ld_p2(at(dst4, stride * 2)); // movd xm3, [dst4q+strideq*2]
                m[4] = movd_ld_p2(at(dst4, stride3)); // movd xm4, [dst4q+stride3q]
                m[1] = zx(_mm_cvtepu8_epi16(lo(m[1]))); // pmovzxbw xm1, xm1
                m[2] = zx(_mm_cvtepu8_epi16(lo(m[2]))); // pmovzxbw xm2, xm2
                m[3] = zx(_mm_cvtepu8_epi16(lo(m[3]))); // pmovzxbw xm3, xm3
                m[4] = zx(_mm_cvtepu8_epi16(lo(m[4]))); // pmovzxbw xm4, xm4
                movq_st_p2(at(px, 4 * 32), m[1]); // movq [px+4*32], xm1
                movq_st_p2(at(px, 5 * 32), m[2]); // movq [px+5*32], xm2
                movq_st_p2(at(px, 6 * 32), m[3]); // movq [px+6*32], xm3
                movq_st_p2(at(px, 7 * 32), m[4]); // movq [px+7*32], xm4
            } else {
                m[1] = pmovzxbw_x_p2(at(dst4, stride * 0)); // pmovzxbw xm1, [dst4q+strideq*0]
                m[2] = pmovzxbw_x_p2(at(dst4, stride * 1)); // pmovzxbw xm2, [dst4q+strideq*1]
                m[3] = pmovzxbw_x_p2(at(dst4, stride * 2)); // pmovzxbw xm3, [dst4q+strideq*2]
                m[4] = pmovzxbw_x_p2(at(dst4, stride3)); // pmovzxbw xm4, [dst4q+stride3q]
                movu_st_x_p2(at(px, 4 * 32), m[1]); // mova [px+4*32], xm1
                movu_st_x_p2(at(px, 5 * 32), m[2]); // mova [px+5*32], xm2
                movu_st_x_p2(at(px, 6 * 32), m[3]); // mova [px+6*32], xm3
                movu_st_x_p2(at(px, 7 * 32), m[4]); // mova [px+7*32], xm4
            }
            movd_st_p2(at(px, 4 * 32 + w * 2), m[14]); // movd [px+4*32+%1*2], xm14
            movd_st_p2(at(px, 5 * 32 + w * 2), m[14]); // movd [px+5*32+%1*2], xm14
            movd_st_p2(at(px, 6 * 32 + w * 2), m[14]); // movd [px+6*32+%1*2], xm14
            movd_st_p2(at(px, 7 * 32 + w * 2), m[14]); // movd [px+7*32+%1*2], xm14
        }
    }
    // .body_done:

    // top
    if edge & 4 != 0 {
        // test edgeb, 4 ; have_top
        if edge & 1 != 0 {
            // test edgeb, 1 ; have_left
            if edge & 2 != 0 {
                // test edgeb, 2 ; have_right
                m[1] = pmovzxbw_m_p2::<W>(atc_p2(top, stride * 0 - (w / 2))); // pmovzxbw m1, [topq+strideq*0-(%1/2)]
                m[2] = pmovzxbw_m_p2::<W>(atc_p2(top, stride * 1 - (w / 2))); // pmovzxbw m2, [topq+strideq*1-(%1/2)]
                movu_st_m_p2::<W>(at(px, -2 * 32 - w), m[1]); // movu [px-2*32-%1], m1
                movu_st_m_p2::<W>(at(px, -1 * 32 - w), m[2]); // movu [px-1*32-%1], m2
                                                              // jmp .top_done
            } else {
                // .top_no_right:
                m[1] = pmovzxbw_m_p2::<W>(atc_p2(top, stride * 0 - w)); // pmovzxbw m1, [topq+strideq*0-%1]
                m[2] = pmovzxbw_m_p2::<W>(atc_p2(top, stride * 1 - w)); // pmovzxbw m2, [topq+strideq*1-%1]
                movu_st_m_p2::<W>(at(px, -2 * 32 - w * 2), m[1]); // movu [px-2*32-%1*2], m1
                movu_st_m_p2::<W>(at(px, -1 * 32 - w * 2), m[2]); // movu [px-1*32-%1*2], m2
                movd_st_p2(at(px, -2 * 32 + w * 2), m[14]); // movd [px-2*32+%1*2], xm14
                movd_st_p2(at(px, -1 * 32 + w * 2), m[14]); // movd [px-1*32+%1*2], xm14
                                                            // jmp .top_done
            }
        } else if edge & 2 != 0 {
            // .top_no_left: test edgeb, 2 ; have_right
            m[1] = pmovzxbw_m_p2::<W>(atc_p2(top, stride * 0)); // pmovzxbw m1, [topq+strideq*0]
            m[2] = pmovzxbw_m_p2::<W>(atc_p2(top, stride * 1)); // pmovzxbw m2, [topq+strideq*1]
            movu_st_m_p2::<W>(at(px, -2 * 32 + 0), m[1]); // mova [px-2*32+0], m1
            movu_st_m_p2::<W>(at(px, -1 * 32 + 0), m[2]); // mova [px-1*32+0], m2
            movd_st_p2(at(px, -2 * 32 - 4), m[14]); // movd [px-2*32-4], xm14
            movd_st_p2(at(px, -1 * 32 - 4), m[14]); // movd [px-1*32-4], xm14
                                                    // jmp .top_done
        } else {
            // .top_no_left_right:
            if W == 4 {
                m[1] = movd_ld_p2(atc_p2(top, stride * 0)); // movd xm1, [topq+strideq*0]
                m[1] = zx(pinsrd_ld_p2::<1>(lo(m[1]), atc_p2(top, stride * 1))); // pinsrd xm1, [topq+strideq*1], 1
                m[1] = zx(_mm_cvtepu8_epi16(lo(m[1]))); // pmovzxbw xm1, xm1
                movq_st_p2(at(px, -2 * 32 + 0), m[1]); // movq [px-2*32+0], xm1
                movhps_st_p2(at(px, -1 * 32 + 0), m[1]); // movhps [px-1*32+0], xm1
            } else {
                m[1] = pmovzxbw_x_p2(atc_p2(top, stride * 0)); // pmovzxbw xm1, [topq+strideq*0]
                m[2] = pmovzxbw_x_p2(atc_p2(top, stride * 1)); // pmovzxbw xm2, [topq+strideq*1]
                movu_st_x_p2(at(px, -2 * 32 + 0), m[1]); // mova [px-2*32+0], xm1
                movu_st_x_p2(at(px, -1 * 32 + 0), m[2]); // mova [px-1*32+0], xm2
            }
            movd_st_p2(at(px, -2 * 32 - 4), m[14]); // movd [px-2*32-4], xm14
            movd_st_p2(at(px, -1 * 32 - 4), m[14]); // movd [px-1*32-4], xm14
            movd_st_p2(at(px, -2 * 32 + w * 2), m[14]); // movd [px-2*32+%1*2], xm14
            movd_st_p2(at(px, -1 * 32 + w * 2), m[14]); // movd [px-1*32+%1*2], xm14
                                                        // jmp .top_done
        }
    } else {
        // .no_top:
        movu_st_m_p2::<W>(at(px, -2 * 32 - w), m[14]); // movu [px-2*32-%1], m14
        movu_st_m_p2::<W>(at(px, -1 * 32 - w), m[14]); // movu [px-1*32-%1], m14
    }
    // .top_done:

    // left
    if edge & 1 != 0 {
        // test edgeb, 1 ; have_left
        m[1] = pmovzxbw_x_p2(atc_p2(left, 0)); // pmovzxbw xm1, [leftq+ 0]
        if H == 8 {
            m[2] = pmovzxbw_x_p2(atc_p2(left, 8)); // pmovzxbw xm2, [leftq+ 8]
        }
        movd_st_p2(at(px, 0 * 32 - 4), m[1]); // movd [px+0*32-4], xm1
        pextrd_st_p2::<1>(at(px, 1 * 32 - 4), m[1]); // pextrd [px+1*32-4], xm1, 1
        pextrd_st_p2::<2>(at(px, 2 * 32 - 4), m[1]); // pextrd [px+2*32-4], xm1, 2
        pextrd_st_p2::<3>(at(px, 3 * 32 - 4), m[1]); // pextrd [px+3*32-4], xm1, 3
        if H == 8 {
            movd_st_p2(at(px, 4 * 32 - 4), m[2]); // movd [px+4*32-4], xm2
            pextrd_st_p2::<1>(at(px, 5 * 32 - 4), m[2]); // pextrd [px+5*32-4], xm2, 1
            pextrd_st_p2::<2>(at(px, 6 * 32 - 4), m[2]); // pextrd [px+6*32-4], xm2, 2
            pextrd_st_p2::<3>(at(px, 7 * 32 - 4), m[2]); // pextrd [px+7*32-4], xm2, 3
        }
        // jmp .left_done
    } else {
        // .no_left:
        movd_st_p2(at(px, 0 * 32 - 4), m[14]); // movd [px+0*32-4], xm14
        movd_st_p2(at(px, 1 * 32 - 4), m[14]); // movd [px+1*32-4], xm14
        movd_st_p2(at(px, 2 * 32 - 4), m[14]); // movd [px+2*32-4], xm14
        movd_st_p2(at(px, 3 * 32 - 4), m[14]); // movd [px+3*32-4], xm14
        if H == 8 {
            movd_st_p2(at(px, 4 * 32 - 4), m[14]); // movd [px+4*32-4], xm14
            movd_st_p2(at(px, 5 * 32 - 4), m[14]); // movd [px+5*32-4], xm14
            movd_st_p2(at(px, 6 * 32 - 4), m[14]); // movd [px+6*32-4], xm14
            movd_st_p2(at(px, 7 * 32 - 4), m[14]); // movd [px+7*32-4], xm14
        }
    }
    // .left_done:

    // bottom
    if edge & 8 != 0 {
        // test edgeb, 8 ; have_bottom
        if edge & 1 != 0 {
            // test edgeb, 1 ; have_left
            if edge & 2 != 0 {
                // test edgeb, 2 ; have_right
                m[1] = pmovzxbw_m_p2::<W>(atc_p2(bot, stride * 0 - (w / 2))); // pmovzxbw m1, [botq+strideq*0-(%1/2)]
                m[2] = pmovzxbw_m_p2::<W>(atc_p2(bot, stride * 1 - (w / 2))); // pmovzxbw m2, [botq+strideq*1-(%1/2)]
                movu_st_m_p2::<W>(at(px, (h + 0) * 32 - w), m[1]); // movu [px+(%2+0)*32-%1], m1
                movu_st_m_p2::<W>(at(px, (h + 1) * 32 - w), m[2]); // movu [px+(%2+1)*32-%1], m2
                                                                   // jmp .bottom_done
            } else {
                // .bottom_no_right:
                m[1] = pmovzxbw_m_p2::<W>(atc_p2(bot, stride * 0 - w)); // pmovzxbw m1, [botq+strideq*0-%1]
                m[2] = pmovzxbw_m_p2::<W>(atc_p2(bot, stride * 1 - w)); // pmovzxbw m2, [botq+strideq*1-%1]
                movu_st_m_p2::<W>(at(px, (h + 0) * 32 - w * 2), m[1]); // movu [px+(%2+0)*32-%1*2], m1
                movu_st_m_p2::<W>(at(px, (h + 1) * 32 - w * 2), m[2]); // movu [px+(%2+1)*32-%1*2], m2
                if W == 8 {
                    movd_st_p2(at(px, (h - 1) * 32 + w * 2), m[14]); // movd [px+(%2-1)*32+%1*2], xm14 ; overwritten by previous movu
                }
                movd_st_p2(at(px, (h + 0) * 32 + w * 2), m[14]); // movd [px+(%2+0)*32+%1*2], xm14
                movd_st_p2(at(px, (h + 1) * 32 + w * 2), m[14]); // movd [px+(%2+1)*32+%1*2], xm14
                                                                 // jmp .bottom_done
            }
        } else if edge & 2 != 0 {
            // .bottom_no_left: test edgeb, 2 ; have_right
            m[1] = pmovzxbw_m_p2::<W>(atc_p2(bot, stride * 0)); // pmovzxbw m1, [botq+strideq*0]
            m[2] = pmovzxbw_m_p2::<W>(atc_p2(bot, stride * 1)); // pmovzxbw m2, [botq+strideq*1]
            movu_st_m_p2::<W>(at(px, (h + 0) * 32 + 0), m[1]); // mova [px+(%2+0)*32+0], m1
            movu_st_m_p2::<W>(at(px, (h + 1) * 32 + 0), m[2]); // mova [px+(%2+1)*32+0], m2
            movd_st_p2(at(px, (h + 0) * 32 - 4), m[14]); // movd [px+(%2+0)*32-4], xm14
            movd_st_p2(at(px, (h + 1) * 32 - 4), m[14]); // movd [px+(%2+1)*32-4], xm14
                                                         // jmp .bottom_done
        } else {
            // .bottom_no_left_right:
            if W == 4 {
                m[1] = movd_ld_p2(atc_p2(bot, stride * 0)); // movd xm1, [botq+strideq*0]
                m[1] = zx(pinsrd_ld_p2::<1>(lo(m[1]), atc_p2(bot, stride * 1))); // pinsrd xm1, [botq+strideq*1], 1
                m[1] = zx(_mm_cvtepu8_epi16(lo(m[1]))); // pmovzxbw xm1, xm1
                movq_st_p2(at(px, (h + 0) * 32 + 0), m[1]); // movq [px+(%2+0)*32+0], xm1
                movhps_st_p2(at(px, (h + 1) * 32 + 0), m[1]); // movhps [px+(%2+1)*32+0], xm1
            } else {
                m[1] = pmovzxbw_x_p2(atc_p2(bot, stride * 0)); // pmovzxbw xm1, [botq+strideq*0]
                m[2] = pmovzxbw_x_p2(atc_p2(bot, stride * 1)); // pmovzxbw xm2, [botq+strideq*1]
                movu_st_x_p2(at(px, (h + 0) * 32 + 0), m[1]); // mova [px+(%2+0)*32+0], xm1
                movu_st_x_p2(at(px, (h + 1) * 32 + 0), m[2]); // mova [px+(%2+1)*32+0], xm2
            }
            movd_st_p2(at(px, (h + 0) * 32 - 4), m[14]); // movd [px+(%2+0)*32-4], xm14
            movd_st_p2(at(px, (h + 1) * 32 - 4), m[14]); // movd [px+(%2+1)*32-4], xm14
            movd_st_p2(at(px, (h + 0) * 32 + w * 2), m[14]); // movd [px+(%2+0)*32+%1*2], xm14
            movd_st_p2(at(px, (h + 1) * 32 + w * 2), m[14]); // movd [px+(%2+1)*32+%1*2], xm14
                                                             // jmp .bottom_done
        }
    } else {
        // .no_bottom:
        movu_st_m_p2::<W>(at(px, (h + 0) * 32 - w), m[14]); // movu [px+(%2+0)*32-%1], m14
        movu_st_m_p2::<W>(at(px, (h + 1) * 32 - w), m[14]); // movu [px+(%2+1)*32-%1], m14
    }
    // .bottom_done:

    // actual filter (INIT_YMM avx2)
    // register to shuffle values into after packing
    m[12] = vbroadcasti128_ro_p2(shufb_lohi, 0); // vbroadcasti128 m12, [shufb_lohi]

    // `dampingd` - 31 (32-bit arithmetic, as the assembly's); `lzcnt` operands below are never zero (the
    // strength was tested nonzero on each path), where `leading_zeros` gives the same value
    let damping = (damping as u32).wrapping_sub(31); // mov dampingd, r8m / sub dampingd, 31
    let zero = 0u32; // xor zerod, zerod
                     // vloop: `%1*%2*2/mmsize > 1` (8x8: 4 passes of 2 rows; 4x8: 2 passes of 4 rows; 4x4: one pass)
    const MMSIZE: usize = 32;
    let vloop = W * H * 2 / MMSIZE > 1;
    let vloop_lines = (MMSIZE / (W * 2)) as isize; // %define vloop_lines (mmsize/(%1*2))
    if pri == 0 {
        // test prid, prid / jz .border_sec_only
        // .border_sec_only:
        m[1] = zx(_mm_cvtsi32_si128(sec)); // movd xm1, secdmpd
        let secdmp = (sec as u32).leading_zeros().wrapping_add(damping); // lzcnt secdmpd, secdmpd / add secdmpd, dampingd
        let sec_shift = _mm_cvtsi64_si128(secdmp as u64 as i64); // mov [rsp+8], secdmpq ; sec_shift
        m[14] = _mm256_set1_epi8(tap_p2(secdmp as usize) as i8); // vpbroadcastb m14, [tableq+secdmpq] ; sec_shift_mask
        m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1 ; sec_strength
        let sec_taps = 12; // lea secq, [tableq+12] ; sec_taps
                           // BORDER_PREP_REGS %1, %2
        let dir = (dir as usize) * 2 + 14; // mov dird, r7m / lea dirq, [tableq+dirq*2+14]
        let mut hd = W * H * 2 / MMSIZE; // mov hd, %1*%2*2/mmsize
        let mut stk = px; // lea stkq, [px]
        m[11] = _mm256_setzero_si256(); // pxor m11, m11
        m[0] = vpbroadcastd_ro_p2(pw_2048, 0); // vpbroadcastd m0, [pw_2048]
        let mut k = 0isize;
        loop {
            // .border_sec_v_loop:
            border_load_block::<W, false>(m, stk, &mut k); // BORDER_LOAD_BLOCK %1, %2
            loop {
                // .border_sec_k_loop:
                m[3] = _mm256_set1_epi8(tap_p2(sec_taps + k as usize) as i8); // vpbroadcastb m3, [secq+kq] ; sec_taps
                accumulate_tap_word::<W, false>(m, stk, dir, k, 2 * 2, sec_shift, 14, 1, 3); // ACCUMULATE_TAP_WORD 2*2, [rsp+8], m14, m1, m3, %1
                accumulate_tap_word::<W, false>(m, stk, dir, k, 6 * 2, sec_shift, 14, 1, 3); // ACCUMULATE_TAP_WORD 6*2, [rsp+8], m14, m1, m3, %1
                k -= 1; // dec kq
                if k < 0 {
                    break; // jge .border_sec_k_loop
                }
            }
            border_adjust_pixel::<W, false>(m, dst, stride, stride3, 0); // BORDER_ADJUST_PIXEL %1, m0
            if !vloop {
                break;
            }
            dst = at(dst, stride * vloop_lines); // lea dstq, [dstq+strideq*vloop_lines]
            stk = at(stk, 32 * vloop_lines); // add stkq, 32*vloop_lines
            hd -= 1; // dec hd
            if hd == 0 {
                break; // jg .border_sec_v_loop
            }
        }
        return; // RET
    }
    m[0] = zx(_mm_cvtsi32_si128(pri)); // movd xm0, prid
    let mut pridmp = (pri as u32).leading_zeros().wrapping_add(damping); // lzcnt pridmpd, prid / add pridmpd, dampingd
    if (pridmp as i32) < 0 {
        pridmp = zero; // cmovs pridmpd, zerod
    }
    let pri_shift = _mm_cvtsi64_si128(pridmp as u64 as i64); // mov [rsp+0], pridmpq ; pri_shift
    if sec == 0 {
        // test secdmpd, secdmpd / jz .border_pri_only
        // .border_pri_only:
        m[13] = _mm256_set1_epi8(tap_p2(pridmp as usize) as i8); // vpbroadcastb m13, [tableq+pridmpq] ; pri_shift_mask
        m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0 ; pri_strength
        let pri_taps = (pri as usize & 1) * 2 + 8; // and prid, 1 / lea priq, [tableq+priq*2+8] ; pri_taps
                                                   // BORDER_PREP_REGS %1, %2
        let dir = (dir as usize) * 2 + 14; // mov dird, r7m / lea dirq, [tableq+dirq*2+14]
        let mut hd = W * H * 2 / MMSIZE; // mov hd, %1*%2*2/mmsize
        let mut stk = px; // lea stkq, [px]
        m[11] = _mm256_setzero_si256(); // pxor m11, m11
        m[1] = vpbroadcastd_ro_p2(pw_2048, 0); // vpbroadcastd m1, [pw_2048]
        let mut k = 0isize;
        loop {
            // .border_pri_v_loop:
            border_load_block::<W, false>(m, stk, &mut k); // BORDER_LOAD_BLOCK %1, %2
            loop {
                // .border_pri_k_loop:
                m[2] = _mm256_set1_epi8(tap_p2(pri_taps + k as usize) as i8); // vpbroadcastb m2, [priq+kq] ; pri_taps
                accumulate_tap_word::<W, false>(m, stk, dir, k, 0 * 2, pri_shift, 13, 0, 2); // ACCUMULATE_TAP_WORD 0*2, [rsp+0], m13, m0, m2, %1
                k -= 1; // dec kq
                if k < 0 {
                    break; // jge .border_pri_k_loop
                }
            }
            border_adjust_pixel::<W, false>(m, dst, stride, stride3, 1); // BORDER_ADJUST_PIXEL %1, m1
            if !vloop {
                break;
            }
            dst = at(dst, stride * vloop_lines); // lea dstq, [dstq+strideq*vloop_lines]
            stk = at(stk, 32 * vloop_lines); // add stkq, 32*vloop_lines
            hd -= 1; // dec hd
            if hd == 0 {
                break; // jg .border_pri_v_loop
            }
        }
        return; // RET
    }
    m[1] = zx(_mm_cvtsi32_si128(sec)); // movd xm1, secdmpd
    let secdmp = (sec as u32).leading_zeros().wrapping_add(damping); // lzcnt secdmpd, secdmpd / add secdmpd, dampingd
    let sec_shift = _mm_cvtsi64_si128(secdmp as u64 as i64); // mov [rsp+8], secdmpq ; sec_shift
    m[13] = _mm256_set1_epi8(tap_p2(pridmp as usize) as i8); // vpbroadcastb m13, [tableq+pridmpq] ; pri_shift_mask
    m[14] = _mm256_set1_epi8(tap_p2(secdmp as usize) as i8); // vpbroadcastb m14, [tableq+secdmpq] ; sec_shift_mask

    // pri/sec_taps[k] [4 total]
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0 ; pri_strength
    m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1 ; sec_strength
    let pri_taps = (pri as usize & 1) * 2 + 8; // and prid, 1 / lea priq, [tableq+priq*2+8] ; pri_taps
    let sec_taps = 12; // lea secq, [tableq+12] ; sec_taps

    // BORDER_PREP_REGS %1, %2
    let dir = (dir as usize) * 2 + 14; // mov dird, r7m / lea dirq, [tableq+dirq*2+14]
    let mut hd = W * H * 2 / MMSIZE; // mov hd, %1*%2*2/mmsize
    let mut stk = px; // lea stkq, [px]
    m[11] = _mm256_setzero_si256(); // pxor m11, m11
    let mut k = 0isize;
    loop {
        // .border_v_loop:
        border_load_block::<W, true>(m, stk, &mut k); // BORDER_LOAD_BLOCK %1, %2, 1
        loop {
            // .border_k_loop:
            m[2] = _mm256_set1_epi8(tap_p2(pri_taps + k as usize) as i8); // vpbroadcastb m2, [priq+kq] ; pri_taps
            m[3] = _mm256_set1_epi8(tap_p2(sec_taps + k as usize) as i8); // vpbroadcastb m3, [secq+kq] ; sec_taps
            accumulate_tap_word::<W, true>(m, stk, dir, k, 0 * 2, pri_shift, 13, 0, 2); // ACCUMULATE_TAP_WORD 0*2, [rsp+0], m13, m0, m2, %1, 1
            accumulate_tap_word::<W, true>(m, stk, dir, k, 2 * 2, sec_shift, 14, 1, 3); // ACCUMULATE_TAP_WORD 2*2, [rsp+8], m14, m1, m3, %1, 1
            accumulate_tap_word::<W, true>(m, stk, dir, k, 6 * 2, sec_shift, 14, 1, 3); // ACCUMULATE_TAP_WORD 6*2, [rsp+8], m14, m1, m3, %1, 1
            k -= 1; // dec kq
            if k < 0 {
                break; // jge .border_k_loop
            }
        }
        m[10] = vpbroadcastd_ro_p2(pw_2048, 0); // vpbroadcastd m10, [pw_2048]
        border_adjust_pixel::<W, true>(m, dst, stride, stride3, 10); // BORDER_ADJUST_PIXEL %1, m10, 1
        if !vloop {
            break;
        }
        dst = at(dst, stride * vloop_lines); // lea dstq, [dstq+strideq*vloop_lines]
        stk = at(stk, 32 * vloop_lines); // add stkq, 32*vloop_lines
        hd -= 1; // dec hd
        if hd == 0 {
            break; // jg .border_v_loop
        }
    }
    // RET
}

/// `.border_block` of `cdef_filter_8x8_8bpc`. Called by the entry point when `edges != 0xf`.
#[inline(always)]
unsafe fn cdef_filter_8x8_8bpc_border_block(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    // ALLOC_STACK 2*16+(%2+4)*32: zero-initialized whole. The padding writes every element the filter reads
    // (rows -2..h+1, columns -2..w+1 of `px`), but only by a case analysis over the sixteen edge combinations.
    let mut frame = [_mm256_setzero_si256(); (2 * 16 + (8 + 4) * 32) / 32];
    let m = &mut regs();
    cdef_filter_8bpc_border_block::<8, 8>(
        m,
        frame.as_mut_ptr().cast(),
        dst,
        stride,
        left,
        top,
        bot,
        pri,
        sec,
        dir,
        damping,
        edges,
    )
}

/// `.border_block` of `cdef_filter_4x8_8bpc`.
#[inline(always)]
unsafe fn cdef_filter_4x8_8bpc_border_block(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    // ALLOC_STACK 2*16+(%2+4)*32: zero-initialized whole (see the 8x8 wrapper).
    let mut frame = [_mm256_setzero_si256(); (2 * 16 + (8 + 4) * 32) / 32];
    let m = &mut regs();
    cdef_filter_8bpc_border_block::<4, 8>(
        m,
        frame.as_mut_ptr().cast(),
        dst,
        stride,
        left,
        top,
        bot,
        pri,
        sec,
        dir,
        damping,
        edges,
    )
}

/// `.border_block` of `cdef_filter_4x4_8bpc`.
#[inline(always)]
unsafe fn cdef_filter_4x4_8bpc_border_block(
    dst: *mut u8,
    stride: isize,
    left: *const [u8; 2],
    top: *const u8,
    bot: *const u8,
    pri: c_int,
    sec: c_int,
    dir: c_int,
    damping: c_int,
    edges: c_uint,
) {
    // ALLOC_STACK 2*16+(%2+4)*32: zero-initialized whole (see the 8x8 wrapper).
    let mut frame = [_mm256_setzero_si256(); (2 * 16 + (4 + 4) * 32) / 32];
    let m = &mut regs();
    cdef_filter_8bpc_border_block::<4, 4>(
        m,
        frame.as_mut_ptr().cast(),
        dst,
        stride,
        left,
        top,
        bot,
        pri,
        sec,
        dir,
        damping,
        edges,
    )
}

/// `cglobal cdef_dir_8bpc, 3, 4, 6, src, stride, var, stride3`: loads the 8x8 block (8 bytes of each of the
/// rows 0..7 at `src`, nothing else), widens it to words and runs `.main`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_cdef_dir_8bpc_avx2(src: *const u8, stride: isize, var: *mut c_uint) -> c_int {
    let m = &mut regs();
    let mut src = src as *mut u8;
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    m[0] = movq_ld_p2(at(src, stride * 0)); // movq xm0, [srcq+strideq*0]
    m[1] = movq_ld_p2(at(src, stride * 1)); // movq xm1, [srcq+strideq*1]
    m[2] = movq_ld_p2(at(src, stride * 2)); // movq xm2, [srcq+strideq*2]
    m[3] = movq_ld_p2(at(src, stride3)); // movq xm3, [srcq+stride3q ]
    src = at(src, stride * 4); // lea srcq, [srcq+strideq*4]
    m[4] = vpbroadcastq_ld_p2(at(src, stride3)); // vpbroadcastq m4, [srcq+stride3q ]
    m[5] = vpbroadcastq_ld_p2(at(src, stride * 2)); // vpbroadcastq m5, [srcq+strideq*2]
    m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[4]); // vpblendd m0, m4, 0xf0
    m[1] = _mm256_blend_epi32::<0xf0>(m[1], m[5]); // vpblendd m1, m5, 0xf0
    m[4] = vpbroadcastq_ld_p2(at(src, stride * 1)); // vpbroadcastq m4, [srcq+strideq*1]
    m[5] = vpbroadcastq_ld_p2(at(src, stride * 0)); // vpbroadcastq m5, [srcq+strideq*0]
    m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[4]); // vpblendd m2, m4, 0xf0
    m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[5]); // vpblendd m3, m5, 0xf0
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    m[0] = _mm256_unpacklo_epi8(m[0], m[4]); // punpcklbw m0, m4
    m[1] = _mm256_unpacklo_epi8(m[1], m[4]); // punpcklbw m1, m4
    m[2] = _mm256_unpacklo_epi8(m[2], m[4]); // punpcklbw m2, m4
    m[3] = _mm256_unpacklo_epi8(m[3], m[4]); // punpcklbw m3, m4
    cdef_dir_8bpc_main(m, var) // cglobal_label .main (falls through)
}

/// `cglobal_label .main` of `cdef_dir_8bpc` to its `RET`: `m0`–`m3` hold the 8x8 block as words (rows 0..3 in
/// the low lanes, rows 7..4 in the high lanes, as the entry builds them); writes `*var`, returns the direction
/// (`eax`). `cdef_dir_16bpc` jumps here too.
#[inline(always)]
pub(crate) unsafe fn cdef_dir_8bpc_main(m: &mut M, var: *mut c_uint) -> c_int {
    m[4] = vpbroadcastd_ro_p2(pw_128, 0); // vpbroadcastd m4, [pw_128]
                                          // PROLOGUE 3, 4, 15 (frame bookkeeping)
    m[0] = _mm256_sub_epi16(m[0], m[4]); // psubw m0, m4
    m[1] = _mm256_sub_epi16(m[1], m[4]); // psubw m1, m4
    m[2] = _mm256_sub_epi16(m[2], m[4]); // psubw m2, m4
    m[3] = _mm256_sub_epi16(m[3], m[4]); // psubw m3, m4

    // shuffle registers to generate partial_sum_diag[0-1] together
    m[7] = _mm256_permute2x128_si256::<0x01>(m[0], m[0]); // vperm2i128 m7, m0, m0, 0x01
    m[6] = _mm256_permute2x128_si256::<0x01>(m[1], m[1]); // vperm2i128 m6, m1, m1, 0x01
    m[5] = _mm256_permute2x128_si256::<0x01>(m[2], m[2]); // vperm2i128 m5, m2, m2, 0x01
    m[4] = _mm256_permute2x128_si256::<0x01>(m[3], m[3]); // vperm2i128 m4, m3, m3, 0x01

    // start with partial_sum_hv[0-1]
    m[8] = _mm256_add_epi16(m[0], m[1]); // paddw m8, m0, m1
    m[9] = _mm256_add_epi16(m[2], m[3]); // paddw m9, m2, m3
    m[10] = _mm256_hadd_epi16(m[0], m[1]); // phaddw m10, m0, m1
    m[11] = _mm256_hadd_epi16(m[2], m[3]); // phaddw m11, m2, m3
    m[8] = _mm256_add_epi16(m[8], m[9]); // paddw m8, m9
    m[10] = _mm256_hadd_epi16(m[10], m[11]); // phaddw m10, m11
    m[9] = zx(_mm256_extracti128_si256::<1>(m[8])); // vextracti128 xm9, m8, 1
    m[11] = zx(_mm256_extracti128_si256::<1>(m[10])); // vextracti128 xm11, m10, 1
    m[8] = zx(_mm_add_epi16(lo(m[8]), lo(m[9]))); // paddw xm8, xm9 ; partial_sum_hv[1]
    m[10] = zx(_mm_hadd_epi16(lo(m[10]), lo(m[11]))); // phaddw xm10, xm11 ; partial_sum_hv[0]
    m[8] = _mm256_inserti128_si256::<1>(m[8], lo(m[10])); // vinserti128 m8, xm10, 1
    m[9] = vpbroadcastd_ro_p2(div_table, 44); // vpbroadcastd m9, [div_table+44]
    m[8] = _mm256_madd_epi16(m[8], m[8]); // pmaddwd m8, m8
    m[8] = _mm256_mullo_epi32(m[8], m[9]); // pmulld m8, m9 ; cost6[2a-d] | cost2[a-d]

    // create aggregates (see the assembly's comment), then shuffle m11 [shufw_6543210x], unpcklwd, pmaddwd,
    // pmulld, paddd
    m[9] = _mm256_bslli_epi128::<2>(m[1]); // pslldq m9, m1, 2
    m[10] = _mm256_bsrli_epi128::<14>(m[1]); // psrldq m10, m1, 14
    m[11] = _mm256_bslli_epi128::<4>(m[2]); // pslldq m11, m2, 4
    m[12] = _mm256_bsrli_epi128::<12>(m[2]); // psrldq m12, m2, 12
    m[13] = _mm256_bslli_epi128::<6>(m[3]); // pslldq m13, m3, 6
    m[14] = _mm256_bsrli_epi128::<10>(m[3]); // psrldq m14, m3, 10
    m[9] = _mm256_add_epi16(m[9], m[11]); // paddw m9, m11
    m[10] = _mm256_add_epi16(m[10], m[12]); // paddw m10, m12
    m[9] = _mm256_add_epi16(m[9], m[13]); // paddw m9, m13
    m[10] = _mm256_add_epi16(m[10], m[14]); // paddw m10, m14
    m[11] = _mm256_bslli_epi128::<8>(m[4]); // pslldq m11, m4, 8
    m[12] = _mm256_bsrli_epi128::<8>(m[4]); // psrldq m12, m4, 8
    m[13] = _mm256_bslli_epi128::<10>(m[5]); // pslldq m13, m5, 10
    m[14] = _mm256_bsrli_epi128::<6>(m[5]); // psrldq m14, m5, 6
    m[9] = _mm256_add_epi16(m[9], m[11]); // paddw m9, m11
    m[10] = _mm256_add_epi16(m[10], m[12]); // paddw m10, m12
    m[9] = _mm256_add_epi16(m[9], m[13]); // paddw m9, m13
    m[10] = _mm256_add_epi16(m[10], m[14]); // paddw m10, m14
    m[11] = _mm256_bslli_epi128::<12>(m[6]); // pslldq m11, m6, 12
    m[12] = _mm256_bsrli_epi128::<4>(m[6]); // psrldq m12, m6, 4
    m[13] = _mm256_bslli_epi128::<14>(m[7]); // pslldq m13, m7, 14
    m[14] = _mm256_bsrli_epi128::<2>(m[7]); // psrldq m14, m7, 2
    m[9] = _mm256_add_epi16(m[9], m[11]); // paddw m9, m11
    m[10] = _mm256_add_epi16(m[10], m[12]); // paddw m10, m12
    m[9] = _mm256_add_epi16(m[9], m[13]); // paddw m9, m13
    m[10] = _mm256_add_epi16(m[10], m[14]); // paddw m10, m14 ; partial_sum_diag[0/1][8-14,zero]
    m[14] = vbroadcasti128_ro_p2(shufw_6543210x, 0); // vbroadcasti128 m14, [shufw_6543210x]
    m[13] = vbroadcasti128_ro_p2(div_table, 16); // vbroadcasti128 m13, [div_table+16]
    m[12] = vbroadcasti128_ro_p2(div_table, 0); // vbroadcasti128 m12, [div_table+0]
    m[9] = _mm256_add_epi16(m[9], m[0]); // paddw m9, m0 ; partial_sum_diag[0/1][0-7]
    m[10] = _mm256_shuffle_epi8(m[10], m[14]); // pshufb m10, m14
    m[11] = _mm256_unpackhi_epi16(m[9], m[10]); // punpckhwd m11, m9, m10
    m[9] = _mm256_unpacklo_epi16(m[9], m[10]); // punpcklwd m9, m10
    m[11] = _mm256_madd_epi16(m[11], m[11]); // pmaddwd m11, m11
    m[9] = _mm256_madd_epi16(m[9], m[9]); // pmaddwd m9, m9
    m[11] = _mm256_mullo_epi32(m[11], m[13]); // pmulld m11, m13
    m[9] = _mm256_mullo_epi32(m[9], m[12]); // pmulld m9, m12
    m[9] = _mm256_add_epi32(m[9], m[11]); // paddd m9, m11 ; cost0[a-d] | cost4[a-d]

    // merge horizontally and vertically for partial_sum_alt[0-3]
    m[10] = _mm256_add_epi16(m[0], m[1]); // paddw m10, m0, m1
    m[11] = _mm256_add_epi16(m[2], m[3]); // paddw m11, m2, m3
    m[12] = _mm256_add_epi16(m[4], m[5]); // paddw m12, m4, m5
    m[13] = _mm256_add_epi16(m[6], m[7]); // paddw m13, m6, m7
    m[0] = _mm256_hadd_epi16(m[0], m[4]); // phaddw m0, m4
    m[1] = _mm256_hadd_epi16(m[1], m[5]); // phaddw m1, m5
    m[2] = _mm256_hadd_epi16(m[2], m[6]); // phaddw m2, m6
    m[3] = _mm256_hadd_epi16(m[3], m[7]); // phaddw m3, m7

    // create aggregates (see the assembly's comment), then pshuflw m11 3012, unpcklwd, pmaddwd, pmulld, paddd
    m[4] = _mm256_bslli_epi128::<2>(m[11]); // pslldq m4, m11, 2
    m[11] = _mm256_bsrli_epi128::<14>(m[11]); // psrldq m11, 14
    m[5] = _mm256_bslli_epi128::<4>(m[12]); // pslldq m5, m12, 4
    m[12] = _mm256_bsrli_epi128::<12>(m[12]); // psrldq m12, 12
    m[6] = _mm256_bslli_epi128::<6>(m[13]); // pslldq m6, m13, 6
    m[13] = _mm256_bsrli_epi128::<10>(m[13]); // psrldq m13, 10
    m[4] = _mm256_add_epi16(m[4], m[10]); // paddw m4, m10
    m[11] = _mm256_add_epi16(m[11], m[12]); // paddw m11, m12
    m[12] = vpbroadcastd_ro_p2(div_table, 44); // vpbroadcastd m12, [div_table+44]
    m[5] = _mm256_add_epi16(m[5], m[6]); // paddw m5, m6
    m[11] = _mm256_add_epi16(m[11], m[13]); // paddw m11, m13 ; partial_sum_alt[3/2] right
    m[13] = vbroadcasti128_ro_p2(div_table, 32); // vbroadcasti128 m13, [div_table+32]
    m[4] = _mm256_add_epi16(m[4], m[5]); // paddw m4, m5 ; partial_sum_alt[3/2] left
    m[5] = _mm256_shufflelo_epi16::<0xc6>(m[11]); // pshuflw m5, m11, q3012
    m[6] = _mm256_unpackhi_epi16(m[11], m[4]); // punpckhwd m6, m11, m4
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
    m[6] = _mm256_madd_epi16(m[6], m[6]); // pmaddwd m6, m6
    m[4] = _mm256_madd_epi16(m[4], m[4]); // pmaddwd m4, m4
    m[6] = _mm256_mullo_epi32(m[6], m[12]); // pmulld m6, m12
    m[4] = _mm256_mullo_epi32(m[4], m[13]); // pmulld m4, m13
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6 ; cost7[a-d] | cost5[a-d]

    // create aggregates (see the assembly's comment), then pshuflw m1 3012, unpcklwd, pmaddwd, pmulld, paddd
    m[5] = _mm256_bslli_epi128::<2>(m[1]); // pslldq m5, m1, 2
    m[1] = _mm256_bsrli_epi128::<14>(m[1]); // psrldq m1, 14
    m[6] = _mm256_bslli_epi128::<4>(m[2]); // pslldq m6, m2, 4
    m[2] = _mm256_bsrli_epi128::<12>(m[2]); // psrldq m2, 12
    m[7] = _mm256_bslli_epi128::<6>(m[3]); // pslldq m7, m3, 6
    m[3] = _mm256_bsrli_epi128::<10>(m[3]); // psrldq m3, 10
    m[5] = _mm256_add_epi16(m[5], m[0]); // paddw m5, m0
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3 ; partial_sum_alt[0/1] right
    m[5] = _mm256_add_epi16(m[5], m[6]); // paddw m5, m6 ; partial_sum_alt[0/1] left
    m[0] = _mm256_shufflelo_epi16::<0xc6>(m[1]); // pshuflw m0, m1, q3012
    m[1] = _mm256_unpackhi_epi16(m[1], m[5]); // punpckhwd m1, m5
    m[5] = _mm256_unpacklo_epi16(m[5], m[0]); // punpcklwd m5, m0
    m[1] = _mm256_madd_epi16(m[1], m[1]); // pmaddwd m1, m1
    m[5] = _mm256_madd_epi16(m[5], m[5]); // pmaddwd m5, m5
    m[1] = _mm256_mullo_epi32(m[1], m[12]); // pmulld m1, m12
    m[5] = _mm256_mullo_epi32(m[5], m[13]); // pmulld m5, m13
    m[5] = _mm256_add_epi32(m[5], m[1]); // paddd m5, m1 ; cost1[a-d] | cost3[a-d]

    m[0] = zx(_mm_loadu_si128(ro(pd_47130256, 16).cast())); // mova xm0, [pd_47130256+ 16]
    m[1] = _mm256_loadu_si256(ro(pd_47130256, 0).cast()); // mova m1, [pd_47130256]
    m[9] = _mm256_hadd_epi32(m[9], m[8]); // phaddd m9, m8
    m[5] = _mm256_hadd_epi32(m[5], m[4]); // phaddd m5, m4
    m[9] = _mm256_hadd_epi32(m[9], m[5]); // phaddd m9, m5
    m[0] = _mm256_permutevar8x32_epi32(m[9], m[0]); // vpermd m0, m9 ; cost[0-3]
    m[1] = _mm256_permutevar8x32_epi32(m[9], m[1]); // vpermd m1, m9 ; cost[4-7] | cost[0-3]

    // now find the best cost
    m[2] = zx(_mm_max_epi32(lo(m[0]), lo(m[1]))); // pmaxsd xm2, xm0, xm1
    m[3] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[2]))); // pshufd xm3, xm2, q1032
    m[2] = zx(_mm_max_epi32(lo(m[2]), lo(m[3]))); // pmaxsd xm2, xm3
    m[3] = zx(_mm_shuffle_epi32::<0xb1>(lo(m[2]))); // pshufd xm3, xm2, q2301
    m[2] = zx(_mm_max_epi32(lo(m[2]), lo(m[3]))); // pmaxsd xm2, xm3 ; best cost

    // find the idx using minpos: make everything other than the best cost negative via subtraction, find the
    // min of unsigned 16-bit ints to sort out the negative values
    m[4] = zx(_mm_sub_epi32(lo(m[1]), lo(m[2]))); // psubd xm4, xm1, xm2
    m[3] = zx(_mm_sub_epi32(lo(m[0]), lo(m[2]))); // psubd xm3, xm0, xm2
    m[3] = zx(_mm_packs_epi32(lo(m[3]), lo(m[4]))); // packssdw xm3, xm4
    m[3] = zx(_mm_minpos_epu16(lo(m[3]))); // phminposuw xm3, xm3

    // convert idx to 32-bits
    m[3] = zx(_mm_srli_epi32::<16>(lo(m[3]))); // psrld xm3, 16
    let eax = _mm_cvtsi128_si32(lo(m[3])); // movd eax, xm3

    // get idx^4 complement
    m[3] = _mm256_permutevar8x32_epi32(m[1], m[3]); // vpermd m3, m1
    m[2] = zx(_mm_sub_epi32(lo(m[2]), lo(m[3]))); // psubd xm2, xm3
    m[2] = zx(_mm_srli_epi32::<10>(lo(m[2]))); // psrld xm2, 10
    var.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(lo(m[2]))); // movd [varq], xm2
    eax // RET
}
