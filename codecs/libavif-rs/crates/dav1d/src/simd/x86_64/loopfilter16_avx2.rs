// Copyright © 2021, VideoLAN and dav1d authors
// Copyright © 2021, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/loopfilter16_avx2.asm` (16 bpc, used
//! at 10 and 12 bit) with AVX2 intrinsics (engine ADR-0044 D2, libavif-rs
//! ADR 0062).
//!
//! The four entry points `dav1d_lpf_{v,h}_sb_{y,uv}_16bpc_avx2`; 16 pixels
//! of 16 bits per vector, so one iteration of an entry point's loop filters
//! four 4-pixel units along the edge (32 bytes of a row for `v`, 16 rows
//! for `h`).
//!
//! Correspondence with the assembly:
//! - Each `cglobal` function is the `extern "C"` function of the same name;
//!   its loop tests `vmask[2]`, `vmask[1]`, `vmask[0]` against the four
//!   mask bits of the iteration (`mask_bits`) and runs the widest filter
//!   any of the four units asks for, as the assembly does.
//! - The macro `FILTER width, dir` is `filter::<WD, DIR>` (`WD` 4, 6, 8 or
//!   16, `DIR` `V` or `H`); its `%if`/`%ifidn` blocks are `if` blocks on
//!   the two constants. The local subroutines `.v4` and `.h4` of the luma
//!   functions, which the chroma functions call too, are `v4` and `h4`.
//!   The macros `TRANSPOSE4X4W`, `TRANSPOSE8X8W` and
//!   `TRANSPOSE_8x4_AND_WRITE_4x16` are the `macro_rules!` of the same
//!   names in lowercase; `SWAP` (a renaming of registers in the assembly,
//!   no instruction) is `swap!`, which exchanges the locals.
//! - The vector registers are the locals `m0` … `m15` (an `xmN` operand is
//!   written through the 128-bit intrinsic and zero-extended, as the
//!   VEX-encoded instruction does), the general-purpose registers locals
//!   named as `DEFINE_ARGS` names them (`dst`, `stride`, `mask`, `l`,
//!   `l_stride`, `lut`, `w`/`h`, `stride3`, `mstride`/`l_stride3`, `tmp`,
//!   `mask_bits`) and `r11`. `dst` and `l` are byte pointers, since the
//!   assembly offsets them in bytes.
//! - The stack frame (`32 * 5` in `lpf_v_sb_y`, `32 * 15` in `lpf_h_sb_y`)
//!   is the local array `rsp` of fifteen vectors: `rsp[N]` is
//!   `[rsp+N*32]`. Slots 0–4 hold spilled registers of the wd16 filter,
//!   slot 5 `p3` and slots 6–13 the transposed columns `p7` … `p4` and
//!   `q4` … `q7` of the `h` filters, slot 14 the filtered `q3`.
//! - The tables `pw_4`, `clip_max` and `clip_min` are one array, as the
//!   assembly addresses all three through `r11` (`pw_4` for 10 bpc,
//!   4 bytes further for 12 bpc); the other constants are vector
//!   constants at their use.
//!
//! Deviations from the assembly:
//! - Every vector access to caller memory is an unaligned move of the same
//!   bytes. The assembly uses aligned moves (`mova`) for the pixel rows of
//!   the `v` filters (32 bytes at `dst` and the rows above and below) and
//!   for the two 16-byte halves of each row of the wd16 `h` filter
//!   (`dst-16` and `dst`). Same number of instructions.
//! - The stack slots are elements of a local array of vectors that is
//!   initialized to zero, and the registers a width does not load (`m0`,
//!   `m11`, `m13`, `m14`, `m15`) start as zero, so that every local has a
//!   value on every path whatever the two constants are. The array is
//!   never addressed by a variable index, so the compiler keeps the slots
//!   that a width uses in registers or in its own spill slots and emits no
//!   store of the initial zeros.
//! - `stride3`, `mstride` (`v`) and `l_stride3` (`h`), which the assembly
//!   keeps in `r7` and `r8`, are parameters of `filter`, as are `r11` and
//!   `m12`; `filter` returns `dst`, which the `h` filters advance.
//! - `movhps [mem], xm` (the store of a row's four pixels from the high
//!   quadword in `TRANSPOSE_8x4_AND_WRITE_4x16`) is the extraction of that
//!   quadword to memory (`vpextrq`), one instruction as well.
//! - `.v4` and `.h4` are inlined into both of their callers; the assembly
//!   calls them.
//!
//! Safety (argued once for the family): the caller `lf_apply_tmpl` (and
//! the test) passes a pointer into a picture plane of 16-bit pixels and
//! its stride in bytes, three (`y`) or two (`uv`) mask words, the level
//! entries of the edge's blocks with `l_stride` entries per row of
//! blocks, the frame's `Av1FilterLUT`, the number of 4-pixel units along
//! the edge, and `bitdepth_max` (1023 or 4095; `bitdepth_max >> 11`
//! selects the row of the constant table, so no other value is allowed).
//! The kernels read and write what the assembly reads and writes; the
//! entry points state it, including the accesses that go beyond what the
//! C equivalent touches (whole groups of four units, the 8 bytes behind
//! `q1` in the rows of the second half in `h4`, `p7`/`q7` and `p3`/`q3`
//! of the wd16 and wd8 `h` filters, the bytes behind the level a unit
//! uses).
#![allow(clippy::all, non_snake_case)]

use core::arch::x86_64::*;
use core::ffi::c_int;

use super::loopfilter_avx2::Av1FilterLUT;

/// The `dir` argument of `FILTER`: `v`, an edge between rows.
const V: bool = false;
/// The `dir` argument of `FILTER`: `h`, an edge between columns.
const H: bool = true;

/// `pw_4`, `clip_max`, `clip_min` of the `SECTION_RODATA`, contiguous: the
/// 10 bpc value of each (two words) is followed by its 12 bpc value, and
/// `r11` points to `pw_4` or 4 bytes behind it.
#[repr(C, align(8))]
struct Rodata([i16; 12]);
static PW_4: Rodata = Rodata([
    4, 4, 16, 16, // pw_4
    511, 511, 2047, 2047, // clip_max
    -512, -512, -2048, -2048, // clip_min
]);

// --- memory operands -------------------------------------------------------------
// SAFETY (all of them): `p + off` is readable or writable for the size of the
// move; the callers say why. `wrapping_offset`, so that an address is only
// required to be in bounds where it is accessed.

/// `mova m, [p+off]` (32 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn load(p: *const u8, off: isize) -> __m256i {
    _mm256_loadu_si256(p.wrapping_offset(off).cast())
}
/// `mova [p+off], m` (32 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn store(p: *mut u8, off: isize, m: __m256i) {
    _mm256_storeu_si256(p.wrapping_offset(off).cast(), m)
}
/// `movu xm, [p+off]` (16 bytes), also the memory operand of `vinserti128`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn loadx(p: *const u8, off: isize) -> __m128i {
    _mm_loadu_si128(p.wrapping_offset(off).cast())
}
/// `movu [p+off], xm` (16 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn storex(p: *mut u8, off: isize, xm: __m128i) {
    _mm_storeu_si128(p.wrapping_offset(off).cast(), xm)
}
/// `vextracti128 [p+off], m, 1` (16 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn storex1(p: *mut u8, off: isize, m: __m256i) {
    _mm_storeu_si128(p.wrapping_offset(off).cast(), _mm256_extracti128_si256::<1>(m))
}
/// `movq xm, [p+off]` (8 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn loadq(p: *const u8, off: isize) -> __m128i {
    _mm_loadl_epi64(p.wrapping_offset(off).cast())
}
/// `movq [p+off], xm` (8 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn storeq(p: *mut u8, off: isize, xm: __m128i) {
    _mm_storel_epi64(p.wrapping_offset(off).cast(), xm)
}
/// `movhps [p+off], xm` (8 bytes): the high quadword, written as its
/// extraction, which compiles to one store (`vpextrq [p+off], xm, 1`); the
/// store intrinsic of the floating-point domain (`_mm_storeh_pd`) compiled
/// to a shuffle and a `vmovq`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn storeh(p: *mut u8, off: isize, xm: __m128i) {
    p.wrapping_offset(off)
        .cast::<i64>()
        .write_unaligned(_mm_extract_epi64::<1>(xm))
}
/// `vpbroadcastd m, [p]` (4 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn broadcastd<T>(p: *const T) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}
/// `vpbroadcastq m, [p+off]` (8 bytes).
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn broadcastq(p: *const u8, off: isize) -> __m256i {
    _mm256_set1_epi64x(p.wrapping_offset(off).cast::<i64>().read_unaligned())
}

/// `SWAP a, b, ...` of x86inc: exchanges the names of `a` and `b`, then of
/// `b` and the next, and so on. No instruction.
macro_rules! swap {
    ($a:ident, $b:ident) => {
        core::mem::swap(&mut $a, &mut $b);
    };
    ($a:ident, $b:ident, $($rest:ident),+) => {
        core::mem::swap(&mut $a, &mut $b);
        swap!($b, $($rest),+);
    };
}

/// `TRANSPOSE4X4W`: per 128-bit lane,
/// ```text
///        in:            out:
/// mm%1   a b c d        a e i m
/// mm%2   e f g h        b f j n
/// mm%3   i j k l   ->   c g k o
/// mm%4   m n o p        d h l p
/// ```
/// of the low four words (the high four words alike, into the high
/// quadwords); `%5` is scratch.
macro_rules! transpose4x4w {
    ($m1:ident, $m2:ident, $m3:ident, $m4:ident, $m5:ident) => {
        $m5 = _mm256_unpacklo_epi16($m1, $m2); // punpcklwd m%5, m%1, m%2
        $m1 = _mm256_unpackhi_epi16($m1, $m2); // punpckhwd m%1, m%2
        $m2 = _mm256_unpacklo_epi16($m3, $m4); // punpcklwd m%2, m%3, m%4
        $m3 = _mm256_unpackhi_epi16($m3, $m4); // punpckhwd m%3, m%4
        $m4 = _mm256_unpacklo_epi32($m5, $m2); // punpckldq m%4, m%5, m%2
        $m5 = _mm256_unpackhi_epi32($m5, $m2); // punpckhdq m%5, m%2
        $m2 = _mm256_unpacklo_epi32($m1, $m3); // punpckldq m%2, m%1, m%3
        $m1 = _mm256_unpackhi_epi32($m1, $m3); // punpckhdq m%1, m%3

        swap!($m1, $m4); // SWAP %1, %4
        swap!($m2, $m5, $m3); // SWAP %2, %5, %3
    };
}

/// `TRANSPOSE8X8W`: transposes the 8x8 words of `%1` … `%8` per 128-bit
/// lane; `%9` is scratch.
macro_rules! transpose8x8w {
    ($m1:ident, $m2:ident, $m3:ident, $m4:ident, $m5:ident, $m6:ident, $m7:ident, $m8:ident, $m9:ident) => {
        transpose4x4w!($m1, $m2, $m3, $m4, $m9); // TRANSPOSE4X4W %1, %2, %3, %4, %9
        transpose4x4w!($m5, $m6, $m7, $m8, $m9); // TRANSPOSE4X4W %5, %6, %7, %8, %9

        $m9 = _mm256_unpackhi_epi64($m1, $m5); // punpckhqdq m%9, m%1, m%5
        $m1 = _mm256_unpacklo_epi64($m1, $m5); // punpcklqdq m%1, m%5
        $m5 = _mm256_unpackhi_epi64($m2, $m6); // punpckhqdq m%5, m%2, m%6
        $m2 = _mm256_unpacklo_epi64($m2, $m6); // punpcklqdq m%2, m%6
        $m6 = _mm256_unpackhi_epi64($m3, $m7); // punpckhqdq m%6, m%3, m%7
        $m3 = _mm256_unpacklo_epi64($m3, $m7); // punpcklqdq m%3, m%7
        $m7 = _mm256_unpackhi_epi64($m4, $m8); // punpckhqdq m%7, m%4, m%8
        $m4 = _mm256_unpacklo_epi64($m4, $m8); // punpcklqdq m%4, m%8

        swap!($m8, $m7, $m4, $m5, $m3, $m2, $m9); // SWAP %8, %7, %4, %5, %3, %2, %9
    };
}

/// `TRANSPOSE_8x4_AND_WRITE_4x16`: transposes `m3` … `m6` (`p1`, `p0`,
/// `q0`, `q1` of sixteen rows) and writes the four pixels of each row at
/// `dst-4` (bytes); advances `dst` by sixteen rows. `m0` is scratch.
macro_rules! transpose_8x4_and_write_4x16 {
    ($dst:ident, $stride:ident, $stride3:ident, $m0:ident, $m3:ident, $m4:ident, $m5:ident, $m6:ident) => {
        // transpose 8x4
        $m0 = _mm256_unpacklo_epi16($m3, $m4); // punpcklwd m0, m3, m4
        $m3 = _mm256_unpackhi_epi16($m3, $m4); // punpckhwd m3, m4
        $m4 = _mm256_unpacklo_epi16($m5, $m6); // punpcklwd m4, m5, m6
        $m5 = _mm256_unpackhi_epi16($m5, $m6); // punpckhwd m5, m6
        $m6 = _mm256_unpacklo_epi32($m0, $m4); // punpckldq m6, m0, m4
        $m0 = _mm256_unpackhi_epi32($m0, $m4); // punpckhdq m0, m4
        $m4 = _mm256_unpacklo_epi32($m3, $m5); // punpckldq m4, m3, m5
        $m3 = _mm256_unpackhi_epi32($m3, $m5); // punpckhdq m3, m5

        // write out
        storeq($dst, -4, _mm256_castsi256_si128($m6)); // movq   [dstq+strideq*0-4], xm6
        storeh($dst, $stride - 4, _mm256_castsi256_si128($m6)); // movhps [dstq+strideq*1-4], xm6
        storeq($dst, $stride * 2 - 4, _mm256_castsi256_si128($m0)); // movq   [dstq+strideq*2-4], xm0
        storeh($dst, $stride3 - 4, _mm256_castsi256_si128($m0)); // movhps [dstq+stride3q -4], xm0
        $dst = $dst.wrapping_offset($stride * 4); // lea dstq, [dstq+strideq*4]
        storeq($dst, -4, _mm256_castsi256_si128($m4)); // movq   [dstq+strideq*0-4], xm4
        storeh($dst, $stride - 4, _mm256_castsi256_si128($m4)); // movhps [dstq+strideq*1-4], xm4
        storeq($dst, $stride * 2 - 4, _mm256_castsi256_si128($m3)); // movq   [dstq+strideq*2-4], xm3
        storeh($dst, $stride3 - 4, _mm256_castsi256_si128($m3)); // movhps [dstq+stride3q -4], xm3
        $dst = $dst.wrapping_offset($stride * 4); // lea dstq, [dstq+strideq*4]

        let xm6 = _mm256_extracti128_si256::<1>($m6); // vextracti128 xm6, m6, 1
        let xm0 = _mm256_extracti128_si256::<1>($m0); // vextracti128 xm0, m0, 1
        let xm4 = _mm256_extracti128_si256::<1>($m4); // vextracti128 xm4, m4, 1
        let xm3 = _mm256_extracti128_si256::<1>($m3); // vextracti128 xm3, m3, 1

        storeq($dst, -4, xm6); // movq   [dstq+strideq*0-4], xm6
        storeh($dst, $stride - 4, xm6); // movhps [dstq+strideq*1-4], xm6
        storeq($dst, $stride * 2 - 4, xm0); // movq   [dstq+strideq*2-4], xm0
        storeh($dst, $stride3 - 4, xm0); // movhps [dstq+stride3q -4], xm0
        $dst = $dst.wrapping_offset($stride * 4); // lea dstq, [dstq+strideq*4]
        storeq($dst, -4, xm4); // movq   [dstq+strideq*0-4], xm4
        storeh($dst, $stride - 4, xm4); // movhps [dstq+strideq*1-4], xm4
        storeq($dst, $stride * 2 - 4, xm3); // movq   [dstq+strideq*2-4], xm3
        storeh($dst, $stride3 - 4, xm3); // movhps [dstq+stride3q -4], xm3
        $dst = $dst.wrapping_offset($stride * 4); // lea dstq, [dstq+strideq*4]
    };
}

/// `FILTER width, dir`: filters the four 4-pixel units at `dst` (`V`: 16
/// pixels of the rows around the edge; `H`: 16 rows of the columns around
/// the edge) with the filter of width `WD` or, per unit, the narrower one
/// its mask bits ask for, or none. `r8` is `mstride` for `V` and
/// `l_stride3` for `H`; `m12` holds the unit's mask bit in its four
/// words. Returns `dst`, which the `H` filters advance by sixteen rows.
///
/// # Safety
/// As the entry points state for the group of four units; the CPU
/// supports AVX2.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn filter<const WD: usize, const DIR: bool>(
    mut dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const u8,
    l_stride: isize,
    lut: *const Av1FilterLUT,
    stride3: isize,
    r8: isize,
    r11: *const i16,
    m12: __m256i,
) -> *mut u8 {
    let mstride = r8; // DEFINE_ARGS of the v functions
    let l_stride3 = r8; // DEFINE_ARGS of the h functions
    let zero = _mm256_setzero_si256();
    let (mut m1, mut m2, mut m3, mut m4, mut m5, mut m6, mut m7): (
        __m256i,
        __m256i,
        __m256i,
        __m256i,
        __m256i,
        __m256i,
        __m256i,
    );
    let (mut m8, mut m9, mut m10): (__m256i, __m256i, __m256i);
    // The registers that only some widths load before the common part (module documentation).
    let (mut m0, mut m11, mut m13, mut m14, mut m15) = (zero, zero, zero, zero, zero);
    // The stack frame: rsp[N] is [rsp+N*32]. Zero-initialized whole (module documentation).
    let mut rsp = [zero; 15];
    let mut tmp = dst;

    // load data
    if DIR == V {
        if WD == 4 {
            tmp = dst.wrapping_offset(mstride * 2); // lea tmpq, [dstq+mstrideq*2]
            m3 = load(tmp, 0); // mova m3, [tmpq+strideq*0] ; p1
            m4 = load(tmp, stride); // mova m4, [tmpq+strideq*1] ; p0
            m5 = load(tmp, stride * 2); // mova m5, [tmpq+strideq*2] ; q0
            m6 = load(tmp, stride3); // mova m6, [tmpq+stride3q] ; q1
        } else {
            // load 6-8 pixels, remainder (for wd=16) will be read inline
            tmp = dst.wrapping_offset(mstride * 4); // lea tmpq, [dstq+mstrideq*4]
                                                    // we load p3 later
            m13 = load(tmp, stride); // mova m13, [tmpq+strideq*1]
            m3 = load(tmp, stride * 2); // mova m3, [tmpq+strideq*2]
            m4 = load(tmp, stride3); // mova m4, [tmpq+stride3q]
            m5 = load(dst, 0); // mova m5, [dstq+strideq*0]
            m6 = load(dst, stride); // mova m6, [dstq+strideq*1]
            m14 = load(dst, stride * 2); // mova m14, [dstq+strideq*2]
            if WD != 6 {
                m15 = load(dst, stride3); // mova m15, [dstq+stride3q]
            }
        }
    } else if WD == 4 {
        // load lines
        m3 = _mm256_zextsi128_si256(loadq(dst, -4)); // movq xm3, [dstq+strideq*0-4]
        m4 = _mm256_zextsi128_si256(loadq(dst, stride - 4)); // movq xm4, [dstq+strideq*1-4]
        m5 = _mm256_zextsi128_si256(loadq(dst, stride * 2 - 4)); // movq xm5, [dstq+strideq*2-4]
        m6 = _mm256_zextsi128_si256(loadq(dst, stride3 - 4)); // movq xm6, [dstq+stride3q -4]
        tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
        m11 = _mm256_zextsi128_si256(loadq(tmp, -4)); // movq xm11, [tmpq+strideq*0-4]
        m13 = _mm256_zextsi128_si256(loadq(tmp, stride - 4)); // movq xm13, [tmpq+strideq*1-4]
        m14 = _mm256_zextsi128_si256(loadq(tmp, stride * 2 - 4)); // movq xm14, [tmpq+strideq*2-4]
        m15 = _mm256_zextsi128_si256(loadq(tmp, stride3 - 4)); // movq xm15, [tmpq+stride3q -4]
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
                                               // this overreads by 8 bytes but the buffers are padded
                                               // so that should be ok
        m3 = _mm256_inserti128_si256::<1>(m3, loadx(tmp, -4)); // vinserti128 m3, [tmpq+strideq*0-4], 1
        m4 = _mm256_inserti128_si256::<1>(m4, loadx(tmp, stride - 4)); // vinserti128 m4, [tmpq+strideq*1-4], 1
        m5 = _mm256_inserti128_si256::<1>(m5, loadx(tmp, stride * 2 - 4)); // vinserti128 m5, [tmpq+strideq*2-4], 1
        m6 = _mm256_inserti128_si256::<1>(m6, loadx(tmp, stride3 - 4)); // vinserti128 m6, [tmpq+stride3q -4], 1
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m11 = _mm256_inserti128_si256::<1>(m11, loadx(tmp, -4)); // vinserti128 m11, [tmpq+strideq*0-4], 1
        m13 = _mm256_inserti128_si256::<1>(m13, loadx(tmp, stride - 4)); // vinserti128 m13, [tmpq+strideq*1-4], 1
        m14 = _mm256_inserti128_si256::<1>(m14, loadx(tmp, stride * 2 - 4)); // vinserti128 m14, [tmpq+strideq*2-4], 1
        m15 = _mm256_inserti128_si256::<1>(m15, loadx(tmp, stride3 - 4)); // vinserti128 m15, [tmpq+stride3q -4], 1

        // transpose 4x8
        // xm3: A-D0,A-D4
        // xm4: A-D1,A-D5
        // xm5: A-D2,A-D6
        // xm6: A-D3,A-D7
        m7 = _mm256_unpacklo_epi16(m3, m4); // punpcklwd m7, m3, m4
        m3 = _mm256_unpacklo_epi16(m11, m13); // punpcklwd m3, m11, m13
        m4 = _mm256_unpacklo_epi16(m5, m6); // punpcklwd m4, m5, m6
        m5 = _mm256_unpacklo_epi16(m14, m15); // punpcklwd m5, m14, m15
                                              // xm7: A0-1,B0-1,C0-1,D0-1
                                              // xm3: A4-5,B4-5,C4-5,D4-5
                                              // xm4: A2-3,B2-3,C2-3,D2-3
                                              // xm5: A6-7,B6-7,C6-7,D6-7
        m6 = _mm256_unpacklo_epi32(m7, m4); // punpckldq m6, m7, m4
        m7 = _mm256_unpackhi_epi32(m7, m4); // punpckhdq m7, m4
        m8 = _mm256_unpacklo_epi32(m3, m5); // punpckldq m8, m3, m5
        m5 = _mm256_unpackhi_epi32(m3, m5); // punpckhdq m5, m3, m5
                                            // xm6: A0-3,B0-3
                                            // xm7: C0-3,D0-3
                                            // xm8: A4-7,B4-7
                                            // xm5: C4-7,D4-7
        m3 = _mm256_unpacklo_epi64(m6, m8); // punpcklqdq m3, m6, m8
        m4 = _mm256_unpackhi_epi64(m6, m8); // punpckhqdq m4, m6, m8
        m6 = _mm256_unpackhi_epi64(m7, m5); // punpckhqdq m6, m7, m5
        m5 = _mm256_unpacklo_epi64(m7, m5); // punpcklqdq m5, m7, m5
                                            // xm3: A0-7
                                            // xm4: B0-7
                                            // xm5: C0-7
                                            // xm6: D0-7
    } else if WD == 6 || WD == 8 {
        m3 = _mm256_zextsi128_si256(loadx(dst, -8)); // movu xm3, [dstq+strideq*0-8]
        m4 = _mm256_zextsi128_si256(loadx(dst, stride - 8)); // movu xm4, [dstq+strideq*1-8]
        m5 = _mm256_zextsi128_si256(loadx(dst, stride * 2 - 8)); // movu xm5, [dstq+strideq*2-8]
        m6 = _mm256_zextsi128_si256(loadx(dst, stride3 - 8)); // movu xm6, [dstq+stride3q -8]
        tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
        m11 = _mm256_zextsi128_si256(loadx(tmp, -8)); // movu xm11, [tmpq+strideq*0-8]
        m13 = _mm256_zextsi128_si256(loadx(tmp, stride - 8)); // movu xm13, [tmpq+strideq*1-8]
        m14 = _mm256_zextsi128_si256(loadx(tmp, stride * 2 - 8)); // movu xm14, [tmpq+strideq*2-8]
        m15 = _mm256_zextsi128_si256(loadx(tmp, stride3 - 8)); // movu xm15, [tmpq+stride3q -8]
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m3 = _mm256_inserti128_si256::<1>(m3, loadx(tmp, -8)); // vinserti128 m3, [tmpq+strideq*0-8], 1
        m4 = _mm256_inserti128_si256::<1>(m4, loadx(tmp, stride - 8)); // vinserti128 m4, [tmpq+strideq*1-8], 1
        m5 = _mm256_inserti128_si256::<1>(m5, loadx(tmp, stride * 2 - 8)); // vinserti128 m5, [tmpq+strideq*2-8], 1
        m6 = _mm256_inserti128_si256::<1>(m6, loadx(tmp, stride3 - 8)); // vinserti128 m6, [tmpq+stride3q -8], 1
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m11 = _mm256_inserti128_si256::<1>(m11, loadx(tmp, -8)); // vinserti128 m11, [tmpq+strideq*0-8], 1
        m13 = _mm256_inserti128_si256::<1>(m13, loadx(tmp, stride - 8)); // vinserti128 m13, [tmpq+strideq*1-8], 1
        m14 = _mm256_inserti128_si256::<1>(m14, loadx(tmp, stride * 2 - 8)); // vinserti128 m14, [tmpq+strideq*2-8], 1
        m15 = _mm256_inserti128_si256::<1>(m15, loadx(tmp, stride3 - 8)); // vinserti128 m15, [tmpq+stride3q -8], 1

        // transpose 8x16
        // xm3: A-H0,A-H8
        // xm4: A-H1,A-H9
        // xm5: A-H2,A-H10
        // xm6: A-H3,A-H11
        // xm11: A-H4,A-H12
        // xm13: A-H5,A-H13
        // xm14: A-H6,A-H14
        // xm15: A-H7,A-H15
        m7 = _mm256_unpacklo_epi16(m3, m4); // punpcklwd m7, m3, m4
        m3 = _mm256_unpackhi_epi16(m3, m4); // punpckhwd m3, m4
        m4 = _mm256_unpacklo_epi16(m5, m6); // punpcklwd m4, m5, m6
        m5 = _mm256_unpackhi_epi16(m5, m6); // punpckhwd m5, m6
        m6 = _mm256_unpacklo_epi16(m11, m13); // punpcklwd m6, m11, m13
        m11 = _mm256_unpackhi_epi16(m11, m13); // punpckhwd m11, m13
        m13 = _mm256_unpacklo_epi16(m14, m15); // punpcklwd m13, m14, m15
        m14 = _mm256_unpackhi_epi16(m14, m15); // punpckhwd m14, m15
                                               // xm7: A0-1,B0-1,C0-1,D0-1
                                               // xm3: E0-1,F0-1,G0-1,H0-1
                                               // xm4: A2-3,B2-3,C2-3,D2-3
                                               // xm5: E2-3,F2-3,G2-3,H2-3
                                               // xm6: A4-5,B4-5,C4-5,D4-5
                                               // xm11: E4-5,F4-5,G4-5,H4-5
                                               // xm13: A6-7,B6-7,C6-7,D6-7
                                               // xm14: E6-7,F6-7,G6-7,H6-7
        m15 = _mm256_unpacklo_epi32(m7, m4); // punpckldq m15, m7, m4
        m7 = _mm256_unpackhi_epi32(m7, m4); // punpckhdq m7, m4
        m9 = _mm256_unpacklo_epi32(m3, m5); // punpckldq m9, m3, m5
        m8 = _mm256_unpackhi_epi32(m3, m5); // punpckhdq m8, m3, m5
        m3 = _mm256_unpacklo_epi32(m6, m13); // punpckldq m3, m6, m13
        m6 = _mm256_unpackhi_epi32(m6, m13); // punpckhdq m6, m13
        m10 = _mm256_unpacklo_epi32(m11, m14); // punpckldq m10, m11, m14
        m11 = _mm256_unpackhi_epi32(m11, m14); // punpckhdq m11, m14
                                               // xm15: A0-3,B0-3
                                               // xm7: C0-3,D0-3
                                               // xm9: E0-3,F0-3
                                               // xm8: G0-3,H0-3
                                               // xm3: A4-7,B4-7
                                               // xm6: C4-7,D4-7
                                               // xm10: E4-7,F4-7
                                               // xm11: G4-7,H4-7
        if WD != 6 {
            m0 = _mm256_unpacklo_epi64(m15, m3); // punpcklqdq m0, m15, m3
        }
        m13 = _mm256_unpackhi_epi64(m15, m3); // punpckhqdq m13, m15, m3
        m3 = _mm256_unpacklo_epi64(m7, m6); // punpcklqdq m3, m7, m6
        m4 = _mm256_unpackhi_epi64(m7, m6); // punpckhqdq m4, m7, m6
        m5 = _mm256_unpacklo_epi64(m9, m10); // punpcklqdq m5, m9, m10
        m6 = _mm256_unpackhi_epi64(m9, m10); // punpckhqdq m6, m9, m10
        m14 = _mm256_unpacklo_epi64(m8, m11); // punpcklqdq m14, m8, m11
        if WD != 6 {
            m15 = _mm256_unpackhi_epi64(m8, m11); // punpckhqdq m15, m8, m11
            rsp[5] = m0; // mova [rsp+5*32], m0
        }
    } else {
        // We only use 14 pixels but we'll need the remainder at the end for
        // the second transpose
        m0 = _mm256_zextsi128_si256(loadx(dst, -16)); // mova xm0, [dstq+strideq*0-16]
        m1 = _mm256_zextsi128_si256(loadx(dst, stride - 16)); // mova xm1, [dstq+strideq*1-16]
        m2 = _mm256_zextsi128_si256(loadx(dst, stride * 2 - 16)); // mova xm2, [dstq+strideq*2-16]
        m3 = _mm256_zextsi128_si256(loadx(dst, stride3 - 16)); // mova xm3, [dstq+stride3q -16]
        tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
        m4 = _mm256_zextsi128_si256(loadx(tmp, -16)); // mova xm4, [tmpq+strideq*0-16]
        m5 = _mm256_zextsi128_si256(loadx(tmp, stride - 16)); // mova xm5, [tmpq+strideq*1-16]
        m6 = _mm256_zextsi128_si256(loadx(tmp, stride * 2 - 16)); // mova xm6, [tmpq+strideq*2-16]
        m7 = _mm256_zextsi128_si256(loadx(tmp, stride3 - 16)); // mova xm7, [tmpq+stride3q -16]
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m0 = _mm256_inserti128_si256::<1>(m0, loadx(tmp, -16)); // vinserti128 m0, m0, [tmpq+strideq*0-16], 1
        m1 = _mm256_inserti128_si256::<1>(m1, loadx(tmp, stride - 16)); // vinserti128 m1, m1, [tmpq+strideq*1-16], 1
        m2 = _mm256_inserti128_si256::<1>(m2, loadx(tmp, stride * 2 - 16)); // vinserti128 m2, m2, [tmpq+strideq*2-16], 1
        m3 = _mm256_inserti128_si256::<1>(m3, loadx(tmp, stride3 - 16)); // vinserti128 m3, m3, [tmpq+stride3q -16], 1
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m4 = _mm256_inserti128_si256::<1>(m4, loadx(tmp, -16)); // vinserti128 m4, m4, [tmpq+strideq*0-16], 1
        m5 = _mm256_inserti128_si256::<1>(m5, loadx(tmp, stride - 16)); // vinserti128 m5, m5, [tmpq+strideq*1-16], 1
        m6 = _mm256_inserti128_si256::<1>(m6, loadx(tmp, stride * 2 - 16)); // vinserti128 m6, m6, [tmpq+strideq*2-16], 1
        m7 = _mm256_inserti128_si256::<1>(m7, loadx(tmp, stride3 - 16)); // vinserti128 m7, m7, [tmpq+stride3q -16], 1

        transpose8x8w!(m0, m1, m2, m3, m4, m5, m6, m7, m8); // TRANSPOSE8X8W 0, 1, 2, 3, 4, 5, 6, 7, 8

        rsp[6] = m0; // mova [rsp+6*32], m0
        rsp[7] = m1; // mova [rsp+7*32], m1
        rsp[8] = m2; // mova [rsp+8*32], m2
        rsp[9] = m3; // mova [rsp+9*32], m3
        rsp[5] = m4; // mova [rsp+5*32], m4

        m0 = _mm256_zextsi128_si256(loadx(dst, 0)); // mova xm0, [dstq+strideq*0]
        m1 = _mm256_zextsi128_si256(loadx(dst, stride)); // mova xm1, [dstq+strideq*1]
        m2 = _mm256_zextsi128_si256(loadx(dst, stride * 2)); // mova xm2, [dstq+strideq*2]
        m3 = _mm256_zextsi128_si256(loadx(dst, stride3)); // mova xm3, [dstq+stride3q ]
        tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
        m8 = _mm256_zextsi128_si256(loadx(tmp, 0)); // mova xm8, [tmpq+strideq*0]
        m9 = _mm256_zextsi128_si256(loadx(tmp, stride)); // mova xm9, [tmpq+strideq*1]
        m10 = _mm256_zextsi128_si256(loadx(tmp, stride * 2)); // mova xm10, [tmpq+strideq*2]
        m11 = _mm256_zextsi128_si256(loadx(tmp, stride3)); // mova xm11, [tmpq+stride3q ]
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m0 = _mm256_inserti128_si256::<1>(m0, loadx(tmp, 0)); // vinserti128 m0, m0, [tmpq+strideq*0], 1
        m1 = _mm256_inserti128_si256::<1>(m1, loadx(tmp, stride)); // vinserti128 m1, m1, [tmpq+strideq*1], 1
        m2 = _mm256_inserti128_si256::<1>(m2, loadx(tmp, stride * 2)); // vinserti128 m2, m2, [tmpq+strideq*2], 1
        m3 = _mm256_inserti128_si256::<1>(m3, loadx(tmp, stride3)); // vinserti128 m3, m3, [tmpq+stride3q ], 1
        tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
        m8 = _mm256_inserti128_si256::<1>(m8, loadx(tmp, 0)); // vinserti128 m8, m8, [tmpq+strideq*0], 1
        m9 = _mm256_inserti128_si256::<1>(m9, loadx(tmp, stride)); // vinserti128 m9, m9, [tmpq+strideq*1], 1
        m10 = _mm256_inserti128_si256::<1>(m10, loadx(tmp, stride * 2)); // vinserti128 m10, m10, [tmpq+strideq*2], 1
        m11 = _mm256_inserti128_si256::<1>(m11, loadx(tmp, stride3)); // vinserti128 m11, m11, [tmpq+stride3q ], 1

        transpose8x8w!(m0, m1, m2, m3, m8, m9, m10, m11, m4); // TRANSPOSE8X8W 0, 1, 2, 3, 8, 9, 10, 11, 4

        rsp[10] = m8; // mova [rsp+10*32], m8
        rsp[11] = m9; // mova [rsp+11*32], m9
        rsp[12] = m10; // mova [rsp+12*32], m10
        rsp[13] = m11; // mova [rsp+13*32], m11

        // 5,6,7,0,1,2,3 -> 13,3,4,5,6,14,15
        swap!(m13, m5, m0); // SWAP 13, 5, 0
        swap!(m3, m6, m1, m15); // SWAP 3, 6, 1, 15
        swap!(m4, m7); // SWAP 4, 7
        swap!(m2, m14); // SWAP 2, 14
    }

    // load L/E/I/H
    if DIR == V {
        m1 = _mm256_cvtepu8_epi16(loadx(l, 0)); // pmovzxbw m1, [lq]
        m0 = _mm256_cvtepu8_epi16(loadx(l, l_stride)); // pmovzxbw m0, [lq+l_strideq]
        m2 = zero; // pxor m2, m2
    } else {
        m0 = broadcastq(l, 0); // vpbroadcastq m0, [lq] ; l0, l1
        m1 = broadcastq(l, l_stride); // vpbroadcastq m1, [lq+l_strideq] ; l2, l3
        m2 = broadcastq(l, l_stride * 2); // vpbroadcastq m2, [lq+l_strideq*2] ; l4, l5
        m10 = broadcastq(l, l_stride3); // vpbroadcastq m10, [lq+l_stride3q] ; l6, l7
        m0 = _mm256_unpacklo_epi32(m0, m1); // punpckldq m0, m1 ; l0, l2, l1, l3 [2x]
        m2 = _mm256_unpacklo_epi32(m2, m10); // punpckldq m2, m10 ; l4, l6, l5, l7 [2x]
        m0 = _mm256_blend_epi32::<0b11110000>(m0, m2); // vpblendd m0, m0, m2, 11110000b ; l0, l2, l1, l3, l4, l6, l5, l7
        m2 = zero; // pxor m2, m2
        m1 = _mm256_unpacklo_epi8(m0, m2); // punpcklbw m1, m0, m2 ; l0, l2, l4, l6
        m0 = _mm256_unpackhi_epi8(m0, m2); // punpckhbw m0, m2 ; l1, l3, l5, l7
    }
    m10 = _mm256_cmpeq_epi16(m2, m0); // pcmpeqw m10, m2, m0
    m1 = _mm256_and_si256(m1, m10); // pand m1, m10
    m0 = _mm256_or_si256(m0, m1); // por m0, m1 ; l[x][] ? l[x][] : l[x-stride][]
    let pb_4x1_4x5_4x9_4x13 = _mm256_setr_epi8(
        0, 1, 0, 1, 0, 1, 0, 1, 8, 9, 8, 9, 8, 9, 8, 9, //
        0, 1, 0, 1, 0, 1, 0, 1, 8, 9, 8, 9, 8, 9, 8, 9,
    );
    m0 = _mm256_shuffle_epi8(m0, pb_4x1_4x5_4x9_4x13); // pshufb m0, [pb_4x1_4x5_4x9_4x13] ; l[x][1]
    m10 = _mm256_cmpeq_epi16(m2, m0); // pcmpeqw m10, m2, m0 ; !L
    m10 = _mm256_srli_epi16::<1>(m10); // psrlw m10, 1
                                       // The count is the low quadword (sharp[0]) of the 16 bytes sharp[0..2] the instruction loads.
    m2 = _mm256_srl_epi16(m0, loadx(lut.cast(), 128)); // psrlw m2, m0, [lutq+128]
    m1 = _mm256_set1_epi16(lut.cast::<u8>().add(136).cast::<i16>().read_unaligned()); // vpbroadcastw m1, [lutq+136]
    m2 = _mm256_min_epu16(m2, m1); // pminuw m2, m1
    m2 = _mm256_max_epu16(m2, _mm256_set1_epi16(1)); // pmaxuw m2, [pw_1] ; I
    m1 = _mm256_srli_epi16::<4>(m0); // psrlw m1, m0, 4 ; H
    m0 = _mm256_add_epi16(m0, _mm256_set1_epi16(2)); // paddw m0, [pw_2]
    m8 = broadcastd(r11); // vpbroadcastd m8, [r11]
    m0 = _mm256_add_epi16(m0, m0); // paddw m0, m0
    m0 = _mm256_add_epi16(m0, m2); // paddw m0, m2 ; E
    m0 = _mm256_mullo_epi16(m0, m8); // REPX {pmullw x, m8}, m0, m1, m2
    m1 = _mm256_mullo_epi16(m1, m8);
    m2 = _mm256_mullo_epi16(m2, m8);

    m8 = _mm256_sub_epi16(m3, m4); // psubw m8, m3, m4 ; p1-p0
    m9 = _mm256_sub_epi16(m5, m6); // psubw m9, m5, m6 ; q1-q0
    m8 = _mm256_abs_epi16(m8); // REPX {pabsw x, x}, m8, m9
    m9 = _mm256_abs_epi16(m9);
    m8 = _mm256_max_epu16(m8, m10); // pmaxuw m8, m10
    m8 = _mm256_max_epu16(m8, m9); // pmaxuw m8, m9
    m7 = _mm256_cmpgt_epi16(m8, m1); // pcmpgtw m7, m8, m1 ; hev
    if WD != 4 {
        m9 = _mm256_sub_epi16(m13, m4); // psubw m9, m13, m4 ; p2-p0
        m9 = _mm256_abs_epi16(m9); // pabsw m9, m9
        m9 = _mm256_max_epu16(m9, m8); // pmaxuw m9, m8
        if WD != 6 {
            if DIR == V {
                m11 = load(tmp, 0); // mova m11, [tmpq+strideq*0] ; p3
            } else {
                m11 = rsp[5]; // mova m11, [rsp+5*32] ; p3
            }
            m10 = _mm256_sub_epi16(m11, m4); // psubw m10, m11, m4 ; p3-p0
            m10 = _mm256_abs_epi16(m10); // pabsw m10, m10
            m9 = _mm256_max_epu16(m9, m10); // pmaxuw m9, m10
        }
        m10 = _mm256_sub_epi16(m5, m14); // psubw m10, m5, m14 ; q2-q0
        m10 = _mm256_abs_epi16(m10); // pabsw m10, m10
        m9 = _mm256_max_epu16(m9, m10); // pmaxuw m9, m10
        if WD != 6 {
            m10 = _mm256_sub_epi16(m5, m15); // psubw m10, m5, m15 ; q3-q0
            m10 = _mm256_abs_epi16(m10); // pabsw m10, m10
            m9 = _mm256_max_epu16(m9, m10); // pmaxuw m9, m10
        }
        m10 = broadcastd(r11); // vpbroadcastd m10, [r11]
        m9 = _mm256_cmpgt_epi16(m9, m10); // pcmpgtw m9, m10 ; !flat8in

        m10 = _mm256_sub_epi16(m13, m3); // psubw m10, m13, m3 ; p2-p1
        m10 = _mm256_abs_epi16(m10); // pabsw m10, m10
        if WD != 6 {
            m11 = _mm256_sub_epi16(m11, m13); // psubw m11, m13 ; p3-p2
            m11 = _mm256_abs_epi16(m11); // pabsw m11, m11
            m10 = _mm256_max_epu16(m10, m11); // pmaxuw m10, m11
            m11 = _mm256_sub_epi16(m14, m15); // psubw m11, m14, m15 ; q3-q2
            m11 = _mm256_abs_epi16(m11); // pabsw m11, m11
            m10 = _mm256_max_epu16(m10, m11); // pmaxuw m10, m11
        }
        m11 = _mm256_sub_epi16(m14, m6); // psubw m11, m14, m6 ; q2-q1
        m11 = _mm256_abs_epi16(m11); // pabsw m11, m11
        m10 = _mm256_max_epu16(m10, m11); // pmaxuw m10, m11

        if WD == 16 {
            m11 = broadcastd(mask.add(2)); // vpbroadcastd m11, [maskq+8]
            m1 = broadcastd(mask.add(1)); // vpbroadcastd m1, [maskq+4]
            m11 = _mm256_or_si256(m11, m1); // por m11, m1
            m11 = _mm256_and_si256(m11, m12); // pand m11, m12
            m11 = _mm256_cmpeq_epi32(m11, m12); // pcmpeqd m11, m12
            m10 = _mm256_and_si256(m10, m11); // pand m10, m11
        } else {
            m11 = broadcastd(mask.add(1)); // vpbroadcastd m11, [maskq+4]
            m11 = _mm256_and_si256(m11, m12); // pand m11, m12
            m11 = _mm256_cmpeq_epi32(m11, m12); // pcmpeqd m11, m12
            m10 = _mm256_and_si256(m10, m11); // pand m10, m11 ; only apply fm-wide to wd>4 blocks
        }
        m8 = _mm256_max_epu16(m8, m10); // pmaxuw m8, m10
    }
    m8 = _mm256_cmpgt_epi16(m8, m2); // pcmpgtw m8, m2

    m10 = _mm256_sub_epi16(m3, m6); // psubw m10, m3, m6 ; p1-q1
    m11 = _mm256_sub_epi16(m4, m5); // psubw m11, m4, m5 ; p0-q0
    m10 = _mm256_abs_epi16(m10); // REPX {pabsw x, x}, m10, m11
    m11 = _mm256_abs_epi16(m11);
    m11 = _mm256_add_epi16(m11, m11); // paddw m11, m11
    m10 = _mm256_srli_epi16::<1>(m10); // psrlw m10, 1
    m10 = _mm256_add_epi16(m10, m11); // paddw m10, m11 ; abs(p0-q0)*2+(abs(p1-q1)>>1)
    m10 = _mm256_cmpgt_epi16(m10, m0); // pcmpgtw m10, m0 ; abs(p0-q0)*2+(abs(p1-q1)>>1) > E
    m8 = _mm256_or_si256(m8, m10); // por m8, m10

    if WD == 16 {
        if DIR == V {
            tmp = dst.wrapping_offset(mstride * 8); // lea tmpq, [dstq+mstrideq*8]
            m0 = load(tmp, stride); // mova m0, [tmpq+strideq*1]
            m1 = load(tmp, stride * 2); // mova m1, [tmpq+strideq*2]
            m2 = load(tmp, stride3); // mova m2, [tmpq+stride3q]
        } else {
            m0 = rsp[7]; // mova m0, [rsp+7*32]
            m1 = rsp[8]; // mova m1, [rsp+8*32]
            m2 = rsp[9]; // mova m2, [rsp+9*32]
        }
        m0 = _mm256_sub_epi16(m0, m4); // REPX {psubw x, m4}, m0, m1, m2
        m1 = _mm256_sub_epi16(m1, m4);
        m2 = _mm256_sub_epi16(m2, m4);
        m0 = _mm256_abs_epi16(m0); // REPX {pabsw x, x}, m0, m1, m2
        m1 = _mm256_abs_epi16(m1);
        m2 = _mm256_abs_epi16(m2);
        m1 = _mm256_max_epu16(m1, m0); // pmaxuw m1, m0
        m1 = _mm256_max_epu16(m1, m2); // pmaxuw m1, m2
        if DIR == V {
            tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
            m0 = load(tmp, 0); // mova m0, [tmpq+strideq*0]
            m2 = load(tmp, stride); // mova m2, [tmpq+strideq*1]
            m10 = load(tmp, stride * 2); // mova m10, [tmpq+strideq*2]
        } else {
            m0 = rsp[10]; // mova m0, [rsp+10*32]
            m2 = rsp[11]; // mova m2, [rsp+11*32]
            m10 = rsp[12]; // mova m10, [rsp+12*32]
        }
        m0 = _mm256_sub_epi16(m0, m5); // REPX {psubw x, m5}, m0, m2, m10
        m2 = _mm256_sub_epi16(m2, m5);
        m10 = _mm256_sub_epi16(m10, m5);
        m0 = _mm256_abs_epi16(m0); // REPX {pabsw x, x}, m0, m2, m10
        m2 = _mm256_abs_epi16(m2);
        m10 = _mm256_abs_epi16(m10);
        m0 = _mm256_max_epu16(m0, m2); // pmaxuw m0, m2
        m1 = _mm256_max_epu16(m1, m10); // pmaxuw m1, m10
        m1 = _mm256_max_epu16(m1, m0); // pmaxuw m1, m0
        m0 = broadcastd(r11); // vpbroadcastd m0, [r11]
        m1 = _mm256_cmpgt_epi16(m1, m0); // pcmpgtw m1, m0 ; !flat8out
        m1 = _mm256_or_si256(m1, m9); // por m1, m9 ; !flat8in | !flat8out
        m2 = broadcastd(mask.add(2)); // vpbroadcastd m2, [maskq+8]
        m10 = _mm256_and_si256(m2, m12); // pand m10, m2, m12
        m10 = _mm256_cmpeq_epi32(m10, m12); // pcmpeqd m10, m12
        m1 = _mm256_andnot_si256(m1, m10); // pandn m1, m10 ; flat16
        m1 = _mm256_andnot_si256(m8, m1); // pandn m1, m8, m1 ; flat16 & fm

        m10 = broadcastd(mask.add(1)); // vpbroadcastd m10, [maskq+4]
        m10 = _mm256_or_si256(m10, m2); // por m10, m2
        m2 = _mm256_and_si256(m10, m12); // pand m2, m10, m12
        m2 = _mm256_cmpeq_epi32(m2, m12); // pcmpeqd m2, m12
        m9 = _mm256_andnot_si256(m9, m2); // pandn m9, m2 ; flat8in
        m9 = _mm256_andnot_si256(m8, m9); // pandn m9, m8, m9
        m2 = broadcastd(mask); // vpbroadcastd m2, [maskq+0]
        m2 = _mm256_or_si256(m2, m10); // por m2, m10
        m2 = _mm256_and_si256(m2, m12); // pand m2, m12
        m2 = _mm256_cmpeq_epi32(m2, m12); // pcmpeqd m2, m12
        m8 = _mm256_andnot_si256(m8, m2); // pandn m8, m2
        m8 = _mm256_andnot_si256(m9, m8); // pandn m8, m9, m8 ; fm & !flat8 & !flat16
        m9 = _mm256_andnot_si256(m1, m9); // pandn m9, m1, m9 ; flat8 & !flat16
    } else if WD != 4 {
        m0 = broadcastd(mask.add(1)); // vpbroadcastd m0, [maskq+4]
        m2 = _mm256_and_si256(m0, m12); // pand m2, m0, m12
        m2 = _mm256_cmpeq_epi32(m2, m12); // pcmpeqd m2, m12
        m9 = _mm256_andnot_si256(m9, m2); // pandn m9, m2
        m9 = _mm256_andnot_si256(m8, m9); // pandn m9, m8, m9 ; flat8 & fm
        m2 = broadcastd(mask); // vpbroadcastd m2, [maskq+0]
        m0 = _mm256_or_si256(m0, m2); // por m0, m2
        m0 = _mm256_and_si256(m0, m12); // pand m0, m12
        m0 = _mm256_cmpeq_epi32(m0, m12); // pcmpeqd m0, m12
        m8 = _mm256_andnot_si256(m8, m0); // pandn m8, m0
        m8 = _mm256_andnot_si256(m9, m8); // pandn m8, m9, m8 ; fm & !flat8
    } else {
        m0 = broadcastd(mask); // vpbroadcastd m0, [maskq+0]
        m0 = _mm256_and_si256(m0, m12); // pand m0, m12
        m0 = _mm256_cmpeq_epi32(m0, m12); // pcmpeqd m0, m12
        m8 = _mm256_andnot_si256(m8, m0); // pandn m8, m0 ; fm
    }

    // short filter
    m0 = broadcastd(r11.add(4)); // vpbroadcastd m0, [r11+8*1] ; 511 or 2047
    m2 = broadcastd(r11.add(8)); // vpbroadcastd m2, [r11+8*2] ; -512 or -2048
    m10 = _mm256_sub_epi16(m5, m4); // psubw m10, m5, m4
    m11 = _mm256_add_epi16(m10, m10); // paddw m11, m10, m10
    m11 = _mm256_add_epi16(m11, m10); // paddw m11, m10
    m10 = _mm256_sub_epi16(m3, m6); // psubw m10, m3, m6 ; iclip_diff(p1-q1)
    m10 = _mm256_min_epi16(m10, m0); // pminsw m10, m0
    m10 = _mm256_max_epi16(m10, m2); // pmaxsw m10, m2
    m10 = _mm256_and_si256(m10, m7); // pand m10, m7 ; f=iclip_diff(p1-q1)&hev
    m10 = _mm256_add_epi16(m10, m11); // paddw m10, m11 ; f=iclip_diff(3*(q0-p0)+f)
    m10 = _mm256_min_epi16(m10, m0); // pminsw m10, m0
    m10 = _mm256_max_epi16(m10, m2); // pmaxsw m10, m2
    m8 = _mm256_and_si256(m8, m10); // pand m8, m10 ; f&=fm
    m10 = _mm256_set1_epi16(4); // vpbroadcastd m10, [pw_4]
    m10 = _mm256_add_epi16(m10, m8); // paddw m10, m8
    m8 = _mm256_add_epi16(m8, _mm256_set1_epi16(3)); // paddw m8, [pw_3]
    m10 = _mm256_min_epi16(m10, m0); // REPX {pminsw x, m0}, m10, m8
    m8 = _mm256_min_epi16(m8, m0);
    m10 = _mm256_srai_epi16::<3>(m10); // psraw m10, 3 ; f2
    m8 = _mm256_srai_epi16::<3>(m8); // psraw m8, 3 ; f1
    m5 = _mm256_sub_epi16(m5, m10); // psubw m5, m10
    m4 = _mm256_add_epi16(m4, m8); // paddw m4, m8

    m10 = _mm256_add_epi16(m10, _mm256_set1_epi16(1)); // paddw m10, [pw_1]
    m10 = _mm256_srai_epi16::<1>(m10); // psraw m10, 1 ; f=(f1+1)>>1
    m8 = _mm256_andnot_si256(m7, m10); // pandn m8, m7, m10 ; f&=!hev
    m3 = _mm256_add_epi16(m3, m8); // paddw m3, m8
    m6 = _mm256_sub_epi16(m6, m8); // psubw m6, m8
    m8 = zero; // pxor m8, m8
    m0 = _mm256_sub_epi16(m0, m2); // psubw m0, m2 ; 1023 or 4095
    m3 = _mm256_min_epi16(m3, m0); // REPX {pminsw x, m0}, m3, m4, m5, m6
    m4 = _mm256_min_epi16(m4, m0);
    m5 = _mm256_min_epi16(m5, m0);
    m6 = _mm256_min_epi16(m6, m0);
    m3 = _mm256_max_epi16(m3, m8); // REPX {pmaxsw x, m8}, m3, m4, m5, m6
    m4 = _mm256_max_epi16(m4, m8);
    m5 = _mm256_max_epi16(m5, m8);
    m6 = _mm256_max_epi16(m6, m8);

    if WD == 16 {
        // m3-6 = p1/p0/q0/q1, m9=flat8, m1=flat16
        // m12=filter bits mask
        // m13-15=p2/q2/q3
        // m0,2,7-8,10-11 = free

        // flat16 filter
        if DIR == V {
            tmp = dst.wrapping_offset(mstride * 8); // lea tmpq, [dstq+mstrideq*8]
            m0 = load(tmp, stride); // mova m0, [tmpq+strideq*1] ; p6
            m2 = load(tmp, stride * 2); // mova m2, [tmpq+strideq*2] ; p5
            m7 = load(tmp, stride3); // mova m7, [tmpq+stride3q] ; p4
            m11 = load(tmp, stride * 4); // mova m11, [tmpq+strideq*4] ; p3
        } else {
            m0 = rsp[7]; // mova m0, [rsp+7*32]
            m2 = rsp[8]; // mova m2, [rsp+8*32]
            m7 = rsp[9]; // mova m7, [rsp+9*32]
            m11 = rsp[5]; // mova m11, [rsp+5*32]
        }

        rsp[0] = m9; // mova [rsp+ 0*32], m9

        // p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
        m8 = _mm256_add_epi16(m0, _mm256_set1_epi16(1)); // paddw m8, m0, [pw_1]
        m8 = _mm256_slli_epi16::<3>(m8); // psllw m8, 3 ; p6*8+8
        m10 = _mm256_add_epi16(m2, m7); // paddw m10, m2, m7 ; p5+p4
        m8 = _mm256_sub_epi16(m8, m0); // psubw m8, m0
        m10 = _mm256_add_epi16(m10, m10); // paddw m10, m10 ; (p5+p4)*2
        m8 = _mm256_add_epi16(m8, m11); // paddw m8, m11 ; p6*7+p3
        m10 = _mm256_add_epi16(m10, m13); // paddw m10, m13 ; (p5+p4)*2+p2
        m8 = _mm256_add_epi16(m8, m3); // paddw m8, m3 ; p6*7+p3+p1
        m10 = _mm256_add_epi16(m10, m4); // paddw m10, m4 ; (p5+p4)*2+p2+p0
        m8 = _mm256_add_epi16(m8, m5); // paddw m8, m5 ; p6*7+p3+p1+q0
        m8 = _mm256_add_epi16(m8, m10); // paddw m8, m10 ; p6*7+p5*2+p4*2+p3+p2+p1+p0+q0
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m2, m10, m1); // vpblendvb m10, m2, m10, m1
        if DIR == V {
            store(tmp, stride * 2, m10); // mova [tmpq+strideq*2], m10 ; p5
        } else {
            rsp[8] = m10; // mova [rsp+8*32], m10
        }

        // sub p6*2, add p3/q1
        m8 = _mm256_add_epi16(m8, m11); // paddw m8, m11
        m10 = _mm256_add_epi16(m0, m0); // paddw m10, m0, m0
        m8 = _mm256_add_epi16(m8, m6); // paddw m8, m6
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m7, m10, m1); // vpblendvb m10, m7, m10, m1
        if DIR == V {
            store(tmp, stride3, m10); // mova [tmpq+stride3q], m10 ; p4
        } else {
            rsp[9] = m10; // mova [rsp+9*32], m10
        }

        // sub p6/p5, add p2/q2
        m8 = _mm256_sub_epi16(m8, m0); // psubw m8, m0
        m10 = _mm256_add_epi16(m13, m14); // paddw m10, m13, m14
        m8 = _mm256_sub_epi16(m8, m2); // psubw m8, m2
        m8 = _mm256_add_epi16(m8, m10); // paddw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m11, m10, m1); // vpblendvb m10, m11, m10, m1
        if DIR == V {
            store(tmp, stride * 4, m10); // mova [tmpq+strideq*4], m10 ; p3
            tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
        } else {
            rsp[5] = m10; // mova [rsp+5*32], m10
        }

        // sub p6/p4, add p1/q3
        m8 = _mm256_add_epi16(m8, m3); // paddw m8, m3
        m10 = _mm256_add_epi16(m0, m7); // paddw m10, m0, m7
        m8 = _mm256_add_epi16(m8, m15); // paddw m8, m15
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m13, m10, m1); // vpblendvb m10, m13, m10, m1
        rsp[1] = m10; // mova [rsp+1*32], m10 ; don't clobber p2/m13

        // sub p6/p3, add p0/q4
        m8 = _mm256_add_epi16(m8, m4); // paddw m8, m4
        m10 = _mm256_add_epi16(m0, m11); // paddw m10, m0, m11
        if DIR == V {
            m8 = _mm256_add_epi16(m8, load(tmp, 0)); // paddw m8, [tmpq+strideq*0]
        } else {
            m8 = _mm256_add_epi16(m8, rsp[10]); // paddw m8, [rsp+10*32]
        }
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m3, m10, m1); // vpblendvb m10, m3, m10, m1
        rsp[2] = m10; // mova [rsp+2*32], m10 ; don't clobber p1/m3

        // sub p6/p2, add q0/q5
        m8 = _mm256_add_epi16(m8, m5); // paddw m8, m5
        m10 = _mm256_add_epi16(m0, m13); // paddw m10, m0, m13
        if DIR == V {
            m8 = _mm256_add_epi16(m8, load(tmp, stride)); // paddw m8, [tmpq+strideq*1]
        } else {
            m8 = _mm256_add_epi16(m8, rsp[11]); // paddw m8, [rsp+11*32]
        }
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m4, m10, m1); // vpblendvb m10, m4, m10, m1
        rsp[3] = m10; // mova [rsp+3*32], m10 ; don't clobber p0/m4

        // sub p6/p1, add q1/q6
        m8 = _mm256_add_epi16(m8, m6); // paddw m8, m6
        m10 = _mm256_add_epi16(m0, m3); // paddw m10, m0, m3
        if DIR == V {
            m0 = load(tmp, stride * 2); // mova m0, [tmpq+strideq*2] ; q6
        } else {
            m0 = rsp[12]; // mova m0, [rsp+12*32] ; q6
        }
        m8 = _mm256_add_epi16(m8, m0); // paddw m8, m0
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m5, m10, m1); // vpblendvb m10, m5, m10, m1
        rsp[4] = m10; // mova [rsp+4*32], m10 ; don't clobber q0/m5

        // sub p5/p0, add q2/q6
        m8 = _mm256_add_epi16(m8, m14); // paddw m8, m14
        m10 = _mm256_add_epi16(m2, m4); // paddw m10, m2, m4
        m8 = _mm256_add_epi16(m8, m0); // paddw m8, m0
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m2 = _mm256_blendv_epi8(m6, m10, m1); // vpblendvb m2, m6, m10, m1 ; don't clobber q1/m6

        // sub p4/q0, add q3/q6
        m8 = _mm256_add_epi16(m8, m15); // paddw m8, m15
        m10 = _mm256_add_epi16(m7, m5); // paddw m10, m7, m5
        m8 = _mm256_add_epi16(m8, m0); // paddw m8, m0
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m7 = _mm256_blendv_epi8(m14, m10, m1); // vpblendvb m7, m14, m10, m1 ; don't clobber q2/m14

        // sub p3/q1, add q4/q6
        if DIR == V {
            m8 = _mm256_add_epi16(m8, load(tmp, 0)); // paddw m8, [tmpq+strideq*0]
        } else {
            m8 = _mm256_add_epi16(m8, rsp[10]); // paddw m8, [rsp+10*32]
        }
        m10 = _mm256_add_epi16(m11, m6); // paddw m10, m11, m6
        m8 = _mm256_add_epi16(m8, m0); // paddw m8, m0
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        m10 = _mm256_blendv_epi8(m15, m10, m1); // vpblendvb m10, m15, m10, m1
        if DIR == V {
            store(tmp, mstride, m10); // mova [tmpq+mstrideq], m10 ; q3
        } else {
            rsp[14] = m10; // mova [rsp+14*32], m10
        }

        // sub p2/q2, add q5/q6
        if DIR == V {
            m8 = _mm256_add_epi16(m8, load(tmp, stride)); // paddw m8, [tmpq+strideq*1]
        } else {
            m8 = _mm256_add_epi16(m8, rsp[11]); // paddw m8, [rsp+11*32]
        }
        m10 = _mm256_add_epi16(m13, m14); // paddw m10, m13, m14
        m8 = _mm256_add_epi16(m8, m0); // paddw m8, m0
        m8 = _mm256_sub_epi16(m8, m10); // psubw m8, m10
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        if DIR == V {
            m9 = load(tmp, 0); // mova m9, [tmpq+strideq*0]
        } else {
            m9 = rsp[10]; // mova m9, [rsp+10*32]
        }
        m10 = _mm256_blendv_epi8(m9, m10, m1); // vpblendvb m10, m9, m10, m1
        if DIR == V {
            store(tmp, 0, m10); // mova [tmpq+strideq*0], m10 ; q4
        } else {
            rsp[10] = m10; // mova [rsp+10*32], m10
        }

        // sub p1/q3, add q6*2
        m8 = _mm256_sub_epi16(m8, m3); // psubw m8, m3
        m0 = _mm256_add_epi16(m0, m0); // paddw m0, m0
        m8 = _mm256_sub_epi16(m8, m15); // psubw m8, m15
        m8 = _mm256_add_epi16(m8, m0); // paddw m8, m0
        m10 = _mm256_srli_epi16::<4>(m8); // psrlw m10, m8, 4
        if DIR == V {
            m9 = load(tmp, stride); // mova m9, [tmpq+strideq*1]
        } else {
            m9 = rsp[11]; // mova m9, [rsp+11*32]
        }
        m10 = _mm256_blendv_epi8(m9, m10, m1); // vpblendvb m10, m9, m10, m1
        if DIR == V {
            store(tmp, stride, m10); // mova [tmpq+strideq*1], m10 ; q5
        } else {
            rsp[11] = m10; // mova [rsp+11*32], m10
        }

        m9 = rsp[0]; // mova m9, [rsp+0*32]
        m13 = rsp[1]; // mova m13, [rsp+1*32]
        m3 = rsp[2]; // mova m3, [rsp+2*32]
        m4 = rsp[3]; // mova m4, [rsp+3*32]
        m5 = rsp[4]; // mova m5, [rsp+4*32]
        swap!(m2, m6); // SWAP 2, 6
        swap!(m7, m14); // SWAP 7, 14
        if DIR == V {
            tmp = dst.wrapping_offset(mstride * 4); // lea tmpq, [dstq+mstrideq*4]
        } else {
            m15 = rsp[14]; // mova m15, [rsp+14*32]
        }
    }

    if WD >= 8 {
        // flat8 filter
        m7 = _mm256_set1_epi16(4096); // vpbroadcastd m7, [pw_4096]
        if DIR == V {
            m0 = load(tmp, 0); // mova m0, [tmpq+strideq*0] ; p3
        } else {
            m0 = rsp[5]; // mova m0, [rsp+5*32] ; p3
        }
        m1 = _mm256_add_epi16(m0, m13); // paddw m1, m0, m13 ; p3+p2
        m2 = _mm256_add_epi16(m3, m4); // paddw m2, m3, m4 ; p1+p0
        m8 = _mm256_add_epi16(m1, m1); // paddw m8, m1, m1 ; 2*(p3+p2)
        m2 = _mm256_add_epi16(m2, m0); // paddw m2, m0 ; p1+p0+p3
        m8 = _mm256_add_epi16(m8, m5); // paddw m8, m5 ; 2*(p3+p2)+q0
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8 ; 3*p3+2*p2+p1+p0+q0
        m10 = _mm256_mulhrs_epi16(m2, m7); // pmulhrsw m10, m2, m7

        m8 = _mm256_add_epi16(m3, m6); // paddw m8, m3, m6
        m2 = _mm256_sub_epi16(m2, m1); // psubw m2, m1
        m2 = _mm256_add_epi16(m2, m8); // paddw m2, m8
        m8 = _mm256_mulhrs_epi16(m2, m7); // pmulhrsw m8, m2, m7

        m11 = _mm256_add_epi16(m0, m3); // paddw m11, m0, m3
        m1 = _mm256_add_epi16(m4, m14); // paddw m1, m4, m14
        m2 = _mm256_sub_epi16(m2, m11); // psubw m2, m11
        m2 = _mm256_add_epi16(m2, m1); // paddw m2, m1
        m1 = _mm256_mulhrs_epi16(m2, m7); // pmulhrsw m1, m2, m7

        m11 = _mm256_add_epi16(m0, m4); // paddw m11, m0, m4
        m4 = _mm256_blendv_epi8(m4, m1, m9); // pblendvb m4, m1, m9
        m1 = _mm256_add_epi16(m5, m15); // paddw m1, m5, m15
        m2 = _mm256_sub_epi16(m2, m11); // psubw m2, m11
        m2 = _mm256_add_epi16(m2, m1); // paddw m2, m1
        m11 = _mm256_mulhrs_epi16(m2, m7); // pmulhrsw m11, m2, m7

        m2 = _mm256_add_epi16(m2, m6); // paddw m2, m6
        m2 = _mm256_add_epi16(m2, m15); // paddw m2, m15
        m1 = _mm256_add_epi16(m13, m5); // paddw m1, m13, m5
        m5 = _mm256_blendv_epi8(m5, m11, m9); // pblendvb m5, m11, m9
        m13 = _mm256_blendv_epi8(m13, m10, m9); // pblendvb m13, m10, m9
        m2 = _mm256_sub_epi16(m2, m1); // psubw m2, m1
        m1 = _mm256_mulhrs_epi16(m2, m7); // pmulhrsw m1, m2, m7

        m2 = _mm256_sub_epi16(m2, m3); // psubw m2, m3
        m3 = _mm256_blendv_epi8(m3, m8, m9); // pblendvb m3, m8, m9
        m2 = _mm256_sub_epi16(m2, m6); // psubw m2, m6
        m6 = _mm256_blendv_epi8(m6, m1, m9); // pblendvb m6, m1, m9
        m1 = _mm256_add_epi16(m15, m14); // paddw m1, m15, m14
        m2 = _mm256_add_epi16(m2, m1); // paddw m2, m1
        m2 = _mm256_mulhrs_epi16(m2, m7); // pmulhrsw m2, m7

        m14 = _mm256_blendv_epi8(m14, m2, m9); // pblendvb m14, m2, m9

        if DIR == V {
            store(tmp, stride, m13); // mova [tmpq+strideq*1], m13 ; p2
            store(tmp, stride * 2, m3); // mova [tmpq+strideq*2], m3 ; p1
            store(tmp, stride3, m4); // mova [tmpq+stride3q ], m4 ; p0
            store(dst, 0, m5); // mova [dstq+strideq*0], m5 ; q0
            store(dst, stride, m6); // mova [dstq+strideq*1], m6 ; q1
            store(dst, stride * 2, m14); // mova [dstq+strideq*2], m14 ; q2
        } else if WD == 8 {
            transpose8x8w!(m0, m13, m3, m4, m5, m6, m14, m15, m1); // TRANSPOSE8X8W 0, 13, 3, 4, 5, 6, 14, 15, 1

            // write 8x16
            storex(dst, -8, _mm256_castsi256_si128(m0)); // movu [dstq+strideq*0-8], xm0
            storex(dst, stride - 8, _mm256_castsi256_si128(m13)); // movu [dstq+strideq*1-8], xm13
            storex(dst, stride * 2 - 8, _mm256_castsi256_si128(m3)); // movu [dstq+strideq*2-8], xm3
            storex(dst, stride3 - 8, _mm256_castsi256_si128(m4)); // movu [dstq+stride3q -8], xm4
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            storex(dst, -8, _mm256_castsi256_si128(m5)); // movu [dstq+strideq*0-8], xm5
            storex(dst, stride - 8, _mm256_castsi256_si128(m6)); // movu [dstq+strideq*1-8], xm6
            storex(dst, stride * 2 - 8, _mm256_castsi256_si128(m14)); // movu [dstq+strideq*2-8], xm14
            storex(dst, stride3 - 8, _mm256_castsi256_si128(m15)); // movu [dstq+stride3q -8], xm15
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            storex1(dst, -8, m0); // vextracti128 [dstq+strideq*0-8], m0, 1
            storex1(dst, stride - 8, m13); // vextracti128 [dstq+strideq*1-8], m13, 1
            storex1(dst, stride * 2 - 8, m3); // vextracti128 [dstq+strideq*2-8], m3, 1
            storex1(dst, stride3 - 8, m4); // vextracti128 [dstq+stride3q -8], m4, 1
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            storex1(dst, -8, m5); // vextracti128 [dstq+strideq*0-8], m5, 1
            storex1(dst, stride - 8, m6); // vextracti128 [dstq+strideq*1-8], m6, 1
            storex1(dst, stride * 2 - 8, m14); // vextracti128 [dstq+strideq*2-8], m14, 1
            storex1(dst, stride3 - 8, m15); // vextracti128 [dstq+stride3q -8], m15, 1
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
        } else {
            m8 = rsp[6]; // mova m8, [rsp+6*32]
            m1 = rsp[7]; // mova m1, [rsp+7*32]
            m2 = rsp[8]; // mova m2, [rsp+8*32]
            m7 = rsp[9]; // mova m7, [rsp+9*32]
            transpose8x8w!(m8, m1, m2, m7, m0, m13, m3, m4, m9); // TRANSPOSE8X8W 8, 1, 2, 7, 0, 13, 3, 4, 9

            storex(dst, -16, _mm256_castsi256_si128(m8)); // mova [dstq+strideq*0-16], xm8
            storex(dst, stride - 16, _mm256_castsi256_si128(m1)); // mova [dstq+strideq*1-16], xm1
            storex(dst, stride * 2 - 16, _mm256_castsi256_si128(m2)); // mova [dstq+strideq*2-16], xm2
            storex(dst, stride3 - 16, _mm256_castsi256_si128(m7)); // mova [dstq+stride3q -16], xm7
            tmp = dst.wrapping_offset(stride * 4); // lea tmpq, [dstq+strideq*4]
            storex(tmp, -16, _mm256_castsi256_si128(m0)); // mova [tmpq+strideq*0-16], xm0
            storex(tmp, stride - 16, _mm256_castsi256_si128(m13)); // mova [tmpq+strideq*1-16], xm13
            storex(tmp, stride * 2 - 16, _mm256_castsi256_si128(m3)); // mova [tmpq+strideq*2-16], xm3
            storex(tmp, stride3 - 16, _mm256_castsi256_si128(m4)); // mova [tmpq+stride3q -16], xm4
            tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
            storex1(tmp, -16, m8); // vextracti128 [tmpq+strideq*0-16], m8, 1
            storex1(tmp, stride - 16, m1); // vextracti128 [tmpq+strideq*1-16], m1, 1
            storex1(tmp, stride * 2 - 16, m2); // vextracti128 [tmpq+strideq*2-16], m2, 1
            storex1(tmp, stride3 - 16, m7); // vextracti128 [tmpq+stride3q -16], m7, 1
            tmp = tmp.wrapping_offset(stride * 4); // lea tmpq, [tmpq+strideq*4]
            storex1(tmp, -16, m0); // vextracti128 [tmpq+strideq*0-16], m0, 1
            storex1(tmp, stride - 16, m13); // vextracti128 [tmpq+strideq*1-16], m13, 1
            storex1(tmp, stride * 2 - 16, m3); // vextracti128 [tmpq+strideq*2-16], m3, 1
            storex1(tmp, stride3 - 16, m4); // vextracti128 [tmpq+stride3q -16], m4, 1

            m0 = rsp[10]; // mova m0, [rsp+10*32]
            m1 = rsp[11]; // mova m1, [rsp+11*32]
            m2 = rsp[12]; // mova m2, [rsp+12*32]
            m3 = rsp[13]; // mova m3, [rsp+13*32]
            transpose8x8w!(m5, m6, m14, m15, m0, m1, m2, m3, m4); // TRANSPOSE8X8W 5, 6, 14, 15, 0, 1, 2, 3, 4
            storex(dst, 0, _mm256_castsi256_si128(m5)); // mova [dstq+strideq*0], xm5
            storex(dst, stride, _mm256_castsi256_si128(m6)); // mova [dstq+strideq*1], xm6
            storex(dst, stride * 2, _mm256_castsi256_si128(m14)); // mova [dstq+strideq*2], xm14
            storex(dst, stride3, _mm256_castsi256_si128(m15)); // mova [dstq+stride3q ], xm15
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            storex(dst, 0, _mm256_castsi256_si128(m0)); // mova [dstq+strideq*0], xm0
            storex(dst, stride, _mm256_castsi256_si128(m1)); // mova [dstq+strideq*1], xm1
            storex(dst, stride * 2, _mm256_castsi256_si128(m2)); // mova [dstq+strideq*2], xm2
            storex(dst, stride3, _mm256_castsi256_si128(m3)); // mova [dstq+stride3q ], xm3
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            storex1(dst, 0, m5); // vextracti128 [dstq+strideq*0], m5, 1
            storex1(dst, stride, m6); // vextracti128 [dstq+strideq*1], m6, 1
            storex1(dst, stride * 2, m14); // vextracti128 [dstq+strideq*2], m14, 1
            storex1(dst, stride3, m15); // vextracti128 [dstq+stride3q ], m15, 1
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            storex1(dst, 0, m0); // vextracti128 [dstq+strideq*0], m0, 1
            storex1(dst, stride, m1); // vextracti128 [dstq+strideq*1], m1, 1
            storex1(dst, stride * 2, m2); // vextracti128 [dstq+strideq*2], m2, 1
            storex1(dst, stride3, m3); // vextracti128 [dstq+stride3q ], m3, 1
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
        }
    } else if WD == 6 {
        // flat6 filter
        m7 = _mm256_set1_epi16(4096); // vpbroadcastd m7, [pw_4096]
        m8 = _mm256_add_epi16(m3, m4); // paddw m8, m3, m4
        m8 = _mm256_add_epi16(m8, m13); // paddw m8, m13 ; p2+p1+p0
        m11 = _mm256_add_epi16(m13, m5); // paddw m11, m13, m5
        m8 = _mm256_add_epi16(m8, m8); // paddw m8, m8
        m8 = _mm256_add_epi16(m8, m11); // paddw m8, m11 ; p2+2*(p2+p1+p0)+q0
        m2 = _mm256_mulhrs_epi16(m8, m7); // pmulhrsw m2, m8, m7

        m8 = _mm256_add_epi16(m8, m5); // paddw m8, m5
        m11 = _mm256_add_epi16(m13, m13); // paddw m11, m13, m13
        m8 = _mm256_add_epi16(m8, m6); // paddw m8, m6
        m8 = _mm256_sub_epi16(m8, m11); // psubw m8, m11
        m10 = _mm256_mulhrs_epi16(m8, m7); // pmulhrsw m10, m8, m7

        m8 = _mm256_add_epi16(m8, m6); // paddw m8, m6
        m11 = _mm256_add_epi16(m13, m3); // paddw m11, m13, m3
        m8 = _mm256_add_epi16(m8, m14); // paddw m8, m14
        m8 = _mm256_sub_epi16(m8, m11); // psubw m8, m11
        m11 = _mm256_mulhrs_epi16(m8, m7); // pmulhrsw m11, m8, m7

        m8 = _mm256_sub_epi16(m8, m3); // psubw m8, m3
        m14 = _mm256_add_epi16(m14, m14); // paddw m14, m14
        m8 = _mm256_sub_epi16(m8, m4); // psubw m8, m4
        m8 = _mm256_add_epi16(m8, m14); // paddw m8, m14
        m8 = _mm256_mulhrs_epi16(m8, m7); // pmulhrsw m8, m7

        m3 = _mm256_blendv_epi8(m3, m2, m9); // pblendvb m3, m2, m9
        m4 = _mm256_blendv_epi8(m4, m10, m9); // pblendvb m4, m10, m9
        m5 = _mm256_blendv_epi8(m5, m11, m9); // pblendvb m5, m11, m9
        m6 = _mm256_blendv_epi8(m6, m8, m9); // pblendvb m6, m8, m9

        if DIR == V {
            store(tmp, stride * 2, m3); // mova [tmpq+strideq*2], m3 ; p1
            store(tmp, stride3, m4); // mova [tmpq+stride3q ], m4 ; p0
            store(dst, 0, m5); // mova [dstq+strideq*0], m5 ; q0
            store(dst, stride, m6); // mova [dstq+strideq*1], m6 ; q1
        } else {
            transpose_8x4_and_write_4x16!(dst, stride, stride3, m0, m3, m4, m5, m6);
            // TRANSPOSE_8x4_AND_WRITE_4x16
        }
    } else if DIR == V {
        store(tmp, 0, m3); // mova [tmpq+strideq*0], m3 ; p1
        store(tmp, stride, m4); // mova [tmpq+strideq*1], m4 ; p0
        store(tmp, stride * 2, m5); // mova [tmpq+strideq*2], m5 ; q0
        store(tmp, stride3, m6); // mova [tmpq+stride3q ], m6 ; q1
    } else {
        transpose_8x4_and_write_4x16!(dst, stride, stride3, m0, m3, m4, m5, m6);
        // TRANSPOSE_8x4_AND_WRITE_4x16
    }
    dst
}

/// `.v4` of `lpf_v_sb_y_16bpc`, which `lpf_v_sb_uv_16bpc` calls too:
/// `FILTER 4, v`.
///
/// # Safety
/// As `filter`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn v4(
    dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const u8,
    l_stride: isize,
    lut: *const Av1FilterLUT,
    stride3: isize,
    mstride: isize,
    r11: *const i16,
    m12: __m256i,
) {
    filter::<4, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride, r11, m12);
    // FILTER 4, v
}

/// `.h4` of `lpf_h_sb_y_16bpc`, which `lpf_h_sb_uv_16bpc` calls too:
/// `FILTER 4, h`. Returns `dst`, advanced by sixteen rows.
///
/// # Safety
/// As `filter`.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn h4(
    dst: *mut u8,
    stride: isize,
    mask: *const u32,
    l: *const u8,
    l_stride: isize,
    lut: *const Av1FilterLUT,
    stride3: isize,
    l_stride3: isize,
    r11: *const i16,
    m12: __m256i,
) -> *mut u8 {
    filter::<4, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3, r11, m12)
    // FILTER 4, h
}

/// `mova m12, [pb_mask]`: the mask bit of each of the four units of the
/// first iteration, in the two doublewords of the unit's four pixels.
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn pb_mask() -> __m256i {
    _mm256_setr_epi32(1, 1, 2, 2, 4, 4, 8, 8)
}

/// `cglobal lpf_v_sb_y_16bpc` (AVX2): `void dav1d_lpf_v_sb_y_16bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int w, int bitdepth_max)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// The loop runs for the groups `g` = 0, 1, … of four units (16 pixels,
/// 32 bytes of a row at `dst + 32 * g`) while `4 * g < w`, at least once:
/// up to three units beyond `w` when `w` is not a multiple of 4. Per
/// group:
/// - reads `mask[0..3]`; for a group with a mask bit set, `lut.sharp[0..2]`
///   (the 16 bytes at offset 128; `sharp[0]` and the low word of
///   `sharp[1]` are used), the 16 bytes at `l + 16 * g` and the 16 bytes
///   `4 * l_stride` before them (bytes 0, 4, 8 and 12 are used: the level
///   of each unit in the row of blocks and in the row above; `l` points
///   to byte 1 of a level entry, so the load ends one byte behind the
///   fourth entry);
/// - for a group with a bit set in `mask[2]`: reads the rows `-7` … `6`
///   (`p6` … `q6`) and writes the rows `-6` … `5`; otherwise with a bit
///   set in `mask[1]`: reads the rows `-4` … `3` and writes the rows
///   `-3` … `2`; otherwise with a bit set in `mask[0]`: reads and writes
///   the rows `-2` … `1`. All 16 pixels of the group in each row are read
///   and written, the pixels of a unit that is not filtered (or filtered
///   by a narrower filter) with the values they had.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_v_sb_y_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    let mut dst = dst.cast::<u8>();
    let r6 = (bitdepth_max as u32 >> 11) as usize; // mov r6d, r7m ; shr r6d, 11 ; is_12bpc
    debug_assert!(r6 <= 1);
    // SAFETY: bitdepth_max is 1023 or 4095, so r11 is pw_4 or pw_4 + 4 bytes.
    let r11 = PW_4.0.as_ptr().add(r6 * 2); // lea r11, [pw_4] ; lea r11, [r11+r6*4]
    let mut w = w; // mov wd, wm
    let l_stride = l_stride << 2; // shl l_strideq, 2
    let mut l = l.cast::<u8>().wrapping_offset(-l_stride); // sub lq, l_strideq
    let mstride = stride.wrapping_neg(); // mov mstrideq, strideq ; neg mstrideq
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    let mut mask_bits: u32 = 0xf; // mov mask_bitsd, 0xf
    let mut m12 = pb_mask(); // mova m12, [pb_mask]

    loop {
        // test [maskq+8], mask_bitsd ; vmask[2]
        if mask.add(2).read_unaligned() & mask_bits != 0 {
            filter::<16, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride, r11, m12);
        // FILTER 16, v
        } else if mask.add(1).read_unaligned() & mask_bits != 0 {
            // .no_flat16: test [maskq+4], mask_bitsd ; vmask[1]
            filter::<8, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride, r11, m12);
        // FILTER 8, v
        } else if mask.read_unaligned() & mask_bits != 0 {
            // .no_flat: test [maskq+0], mask_bitsd ; vmask[0]
            v4(dst, stride, mask, l, l_stride, lut, stride3, mstride, r11, m12);
            // call .v4
        }

        // .end:
        m12 = _mm256_slli_epi32::<4>(m12); // pslld m12, 4
        l = l.wrapping_add(16); // add lq, 16
        dst = dst.wrapping_add(32); // add dstq, 32
        mask_bits <<= 4; // shl mask_bitsd, 4
        w -= 4; // sub wd, 4
        if w <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal lpf_h_sb_y_16bpc` (AVX2): `void dav1d_lpf_h_sb_y_16bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int w, int bitdepth_max)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// `w` is the number of 4-row units (`h` in the assembly). The loop runs
/// for the groups `g` = 0, 1, … of four units (the 16 rows from
/// `dst + 16 * g * stride`) while `4 * g < w`, at least once. Per group:
/// - reads `mask[0..3]`; for a group with a mask bit set, `lut.sharp[0..2]`
///   as `lpf_v_sb_y`, and 8 bytes at `l - 4` in each of the four rows of
///   level entries, `4 * l_stride` bytes apart (bytes 0 and 4 are used:
///   the level of the block to the left and of the unit; the load ends
///   as many bytes behind the unit's entry as `l` points into it);
/// - for a group with a bit set in `mask[2]`: reads and writes the 16
///   pixels `-8` … `7` (`p7` … `q7`) of each of the 16 rows; otherwise
///   with a bit set in `mask[1]`: reads and writes the 8 pixels `-4` … `3`
///   of each row; otherwise with a bit set in `mask[0]`: reads the 4
///   pixels `-2` … `1` of the rows 0–7 and the 8 pixels `-2` … `5` of the
///   rows 8–15 (the assembly's 16-byte load for the upper half of the
///   vector), and writes the 4 pixels `-2` … `1` of each row. Pixels that
///   a unit's filter does not change are written with the values they
///   had.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_h_sb_y_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    let mut dst = dst.cast::<u8>();
    let r6 = (bitdepth_max as u32 >> 11) as usize; // mov r6d, r7m ; shr r6d, 11 ; is_12bpc
    debug_assert!(r6 <= 1);
    // SAFETY: bitdepth_max is 1023 or 4095, so r11 is pw_4 or pw_4 + 4 bytes.
    let r11 = PW_4.0.as_ptr().add(r6 * 2); // lea r11, [pw_4] ; lea r11, [r11+r6*4]
    let mut h = w; // mov hd, hm
    let l_stride = l_stride << 2; // shl l_strideq, 2
    let mut l = l.cast::<u8>().wrapping_sub(4); // sub lq, 4
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    let l_stride3 = l_stride * 3; // lea l_stride3q, [l_strideq*3]
    let mut mask_bits: u32 = 0xf; // mov mask_bitsd, 0xf
    let mut m12 = pb_mask(); // mova m12, [pb_mask]

    loop {
        // test [maskq+8], mask_bitsd ; vmask[2]
        if mask.add(2).read_unaligned() & mask_bits != 0 {
            dst = filter::<16, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3, r11, m12);
        // FILTER 16, h
        } else if mask.add(1).read_unaligned() & mask_bits != 0 {
            // .no_flat16: test [maskq+4], mask_bitsd ; vmask[1]
            dst = filter::<8, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3, r11, m12);
        // FILTER 8, h
        } else if mask.read_unaligned() & mask_bits != 0 {
            // .no_flat: test [maskq+0], mask_bitsd ; vmask[0]
            dst = h4(dst, stride, mask, l, l_stride, lut, stride3, l_stride3, r11, m12);
        // call .h4
        } else {
            // .no_filter:
            dst = dst.wrapping_offset(stride * 8); // lea dstq, [dstq+strideq*8]
            dst = dst.wrapping_offset(stride * 8); // lea dstq, [dstq+strideq*8]
        }

        // .end:
        m12 = _mm256_slli_epi32::<4>(m12); // pslld m12, 4
        l = l.wrapping_offset(l_stride * 4); // lea lq, [lq+l_strideq*4]
        mask_bits <<= 4; // shl mask_bitsd, 4
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal lpf_v_sb_uv_16bpc` (AVX2): `void dav1d_lpf_v_sb_uv_16bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int w, int bitdepth_max)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// As `lpf_v_sb_y` with two mask words (`mask[0..2]`): for a group with a
/// bit set in `mask[1]` reads the rows `-3` … `2` and writes the rows
/// `-2` … `1`; otherwise with a bit set in `mask[0]` reads and writes the
/// rows `-2` … `1`. `l` points to byte 2 or 3 of a level entry, so the
/// 16-byte loads end two or three bytes behind the fourth entry.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_v_sb_uv_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    let mut dst = dst.cast::<u8>();
    let r6 = (bitdepth_max as u32 >> 11) as usize; // mov r6d, r7m ; shr r6d, 11 ; is_12bpc
    debug_assert!(r6 <= 1);
    // SAFETY: bitdepth_max is 1023 or 4095, so r11 is pw_4 or pw_4 + 4 bytes.
    let r11 = PW_4.0.as_ptr().add(r6 * 2); // lea r11, [pw_4] ; lea r11, [r11+r6*4]
    let mut w = w; // mov wd, wm
    let l_stride = l_stride << 2; // shl l_strideq, 2
    let mut l = l.cast::<u8>().wrapping_offset(-l_stride); // sub lq, l_strideq
    let mstride = stride.wrapping_neg(); // mov mstrideq, strideq ; neg mstrideq
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    let mut mask_bits: u32 = 0xf; // mov mask_bitsd, 0xf
    let mut m12 = pb_mask(); // mova m12, [pb_mask]

    loop {
        // test [maskq+4], mask_bitsd ; vmask[1]
        if mask.add(1).read_unaligned() & mask_bits != 0 {
            filter::<6, V>(dst, stride, mask, l, l_stride, lut, stride3, mstride, r11, m12);
        // FILTER 6, v
        } else if mask.read_unaligned() & mask_bits != 0 {
            // .no_flat: test [maskq+0], mask_bitsd ; vmask[0]
            v4(dst, stride, mask, l, l_stride, lut, stride3, mstride, r11, m12);
            // call mangle(private_prefix %+ _lpf_v_sb_y_16bpc_avx2).v4
        }

        // .end:
        m12 = _mm256_slli_epi32::<4>(m12); // pslld m12, 4
        l = l.wrapping_add(16); // add lq, 16
        dst = dst.wrapping_add(32); // add dstq, 32
        mask_bits <<= 4; // shl mask_bitsd, 4
        w -= 4; // sub wd, 4
        if w <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal lpf_h_sb_uv_16bpc` (AVX2): `void dav1d_lpf_h_sb_uv_16bpc_avx2(pixel *dst, ptrdiff_t stride,
/// const uint32_t *mask, const uint8_t (*lvl)[4], ptrdiff_t lvl_stride, const Av1FilterLUT *lut, int w, int bitdepth_max)`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
/// As `lpf_h_sb_y` with two mask words (`mask[0..2]`): for a group with a
/// bit set in `mask[1]` reads the 8 pixels `-4` … `3` of each of the 16
/// rows and writes the 4 pixels `-2` … `1`; otherwise with a bit set in
/// `mask[0]` as `lpf_h_sb_y` for `mask[0]`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_lpf_h_sb_uv_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    mask: *const u32,
    l: *const [u8; 4],
    l_stride: isize,
    lut: *const Av1FilterLUT,
    w: c_int,
    bitdepth_max: c_int,
) {
    let mut dst = dst.cast::<u8>();
    let r6 = (bitdepth_max as u32 >> 11) as usize; // mov r6d, r7m ; shr r6d, 11 ; is_12bpc
    debug_assert!(r6 <= 1);
    // SAFETY: bitdepth_max is 1023 or 4095, so r11 is pw_4 or pw_4 + 4 bytes.
    let r11 = PW_4.0.as_ptr().add(r6 * 2); // lea r11, [pw_4] ; lea r11, [r11+r6*4]
    let mut h = w; // mov hd, hm
    let l_stride = l_stride << 2; // shl l_strideq, 2
    let mut l = l.cast::<u8>().wrapping_sub(4); // sub lq, 4
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    let l_stride3 = l_stride * 3; // lea l_stride3q, [l_strideq*3]
    let mut mask_bits: u32 = 0xf; // mov mask_bitsd, 0xf
    let mut m12 = pb_mask(); // mova m12, [pb_mask]

    loop {
        // test [maskq+4], mask_bitsd ; vmask[1]
        if mask.add(1).read_unaligned() & mask_bits != 0 {
            dst = filter::<6, H>(dst, stride, mask, l, l_stride, lut, stride3, l_stride3, r11, m12);
        // FILTER 6, h
        } else if mask.read_unaligned() & mask_bits != 0 {
            // .no_flat: test [maskq+0], mask_bitsd ; vmask[0]
            dst = h4(dst, stride, mask, l, l_stride, lut, stride3, l_stride3, r11, m12);
        // call mangle(private_prefix %+ _lpf_h_sb_y_16bpc_avx2).h4
        } else {
            // .no_filter:
            dst = dst.wrapping_offset(stride * 8); // lea dstq, [dstq+strideq*8]
            dst = dst.wrapping_offset(stride * 8); // lea dstq, [dstq+strideq*8]
        }

        // .end:
        m12 = _mm256_slli_epi32::<4>(m12); // pslld m12, 4
        l = l.wrapping_offset(l_stride * 4); // lea lq, [lq+l_strideq*4]
        mask_bits <<= 4; // shl mask_bitsd, 4
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .loop
        }
    }
}
