// Copyright © 2018-2021, VideoLAN and dav1d authors
// Copyright © 2018, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/itx_avx2.asm` (8 bpc) with AVX2
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The 156 entry points `dav1d_inv_txfm_add_<type1>_<type2>_<w>x<h>_8bpc_avx2`
//! of the 8 bpc inverse transforms: every size from 4x4 to 64x64, the 16
//! transform-type pairs of the sizes up to 16 in both directions, dct_dct and
//! identity_identity of the 32-point sizes, dct_dct of the 64-point sizes, and
//! WHT 4x4. `simd/x86/itx.rs` assigns them under `DAV1D_X86_CPU_FLAG_AVX2`.
//!
//! Correspondence with the assembly:
//! - The vector registers are one array `m: M = [__m256i; 16]` of the entry
//!   point, passed as `&mut M` to everything it inlines; `m[5]` is `m5`. An
//!   `xmN` operand is `lo(m[N])`; an instruction with an `xmm` destination
//!   (`INIT_XMM`, `WRAP_XMM`, `xmN`) uses the `_mm_*` intrinsic and stores
//!   `zx(…)`, because the VEX encoding clears the upper half. One Rust line
//!   per instruction, with the assembly as its trailing comment, in the
//!   assembly's order; scheduling and register allocation are the compiler's.
//! - `INV_TXFM_FN type1, type2, size` (load `tx2q` with the second pass,
//!   jump to the first) is, per size, one `inv_txfm_add_<size>(dst, stride,
//!   c, eob, t1: K, t2: K)`: the dct_dct DC-only test and path where the
//!   size's `INV_TXFM_<SIZE>_FN` has one, then `i<t1>_<size>_internal_pass1`
//!   (the code up to `jmp tx2q`) and `i<t2>_<size>_internal_pass2` (from
//!   `.pass2:` to its `RET`). The exported functions are made by
//!   `inv_txfm_fn!`, one line each, and are the only items with
//!   `#[target_feature(enable = "avx2")]`; everything else is
//!   `#[inline(always)]`, so the kinds are constants and each entry point is
//!   its own straight-line code, as in the assembly. The sizes that are their
//!   own `cglobal` (8x32 and up) are one function each.
//! - A `%macro` is a function of the macro's name in lower case whose
//!   register arguments are register numbers (`idct8_1d(m, [0, …, 7], [8,
//!   9], 10)`); a macro that the assembly expands at both widths has a
//!   256-bit form under its name and a 128-bit form with the suffix `_x`
//!   (`itx_mul2x_pack_x`, `idct4_1d_packed_x`, `idct8_1d_packed_x`,
//!   `iadst8_1d_packed_x`, `idct16_1d_packed_x`). A label that is `call`ed
//!   or jumped to from elsewhere is a function `<function>_<label>` without
//!   `_8bpc` (`idct_16x16_internal_main`, `iadst_4x16_internal_main2`,
//!   `inv_txfm_add_dct_dct_16x32_main_oddhalf`, …), with the cross calls of
//!   the assembly: 4x16 and 16x4 share their `.main`s, 8x4 uses 4x8's,
//!   16x16's `.main` serves the 32- and 64-point sizes, 16x32's odd half
//!   (`.main_oddhalf`, `.main_oddhalf_fast`, `.main2`), `.transpose_2x8x8_round`
//!   and `.pass2_end` and 32x16's `.pass1_end` serve 32x32 and the 64-point
//!   sizes, and the idct64 subroutines of 16x64 (`.main_part1`,
//!   `.main_part2_pass1`, `.main_part2_internal`, `.main_part2_pass2`,
//!   `IDCT64_PART2_END`) serve the other 64-point sizes. The DC-only tails
//!   (`.dconly` of 8x8, 16x4, 32x8 and 64x16) are functions the other sizes
//!   end in, as they jump into them.
//! - Constants: `[o(pw_A_B)]` is `pw2(A, B)`, `[o(pw_N)]` `pw(N)`,
//!   `[o(pd_N)]` `pd(N)`, `movd xm, [o(pw_N)]` `movd_pw(N)`: broadcasts the
//!   compiler folds; tables loaded whole or indexed (`deint_shuf`,
//!   `idct64_mul`) are statics with the assembly's name and layout.
//! - The general-purpose registers are locals named as in `DEFINE_ARGS`
//!   (`dst`, `stride`, `cq`, `eobd`, `tmp1q`, `r3`, …). The spill slots
//!   `[rsp+gprsize+32*0..2]` that subroutines share with their caller are a
//!   `[__m256i; 3]` passed explicitly. The intermediate rows of the 32- and
//!   64-point sizes (`cglobal …, 32*35` and the like) are a frame of 32-byte
//!   slots reached only through raw pointers with the assembly's byte
//!   offsets (`tmp1q`, `tmp2q`, `r2`, …; `at`, a wrapping offset, so that no
//!   out-of-bounds pointer is formed).
//!
//! Memory: `dst` points to the block's `h` rows of `w` pixels, `stride`
//! bytes apart; `c` to the block's `min(w, 32) * min(h, 32)` coefficients,
//! which each kernel leaves zero where the assembly zeroes them. Every
//! access to `dst` and `c` touches exactly the bytes the assembly touches:
//! rows of 4 (`movd`/`pinsrd`), 8 (`movq`/`movhps`), 16 or 32 bytes inside the
//! block, coefficient loads and stores inside the buffer, and the DC-only
//! paths' 16-byte load and 4-byte store at `c`. None reaches beyond the
//! block or the coefficients.
//!
//! Deviations from the assembly:
//! - Caller memory is accessed with unaligned moves where the assembly uses
//!   `mova` (the same bytes and instruction count; a misaligned pointer is
//!   then not undefined behavior); 4-byte rows with unaligned `i32` reads
//!   and writes (`movd`/`pinsrd`/`pextrd`), `movhps` with an unaligned
//!   8-byte read or write (`movhps_ld`, `movhps_st`).
//! - The frames of the 32- and 64-point sizes are zeroed whole before their
//!   first use (`zeroed_frame`), where the assembly leaves them
//!   uninitialized: which slots a later loop reads depends on the eob path,
//!   so no load can find a slot without a value. Cost: one 32-byte store per
//!   slot (16 to 199 per call; ADR 0062 has the instructions per call). The
//!   zero goes through `core::hint::black_box`, or the compiler calls
//!   `memset` for the larger frames.
//! - Flag arithmetic on general-purpose registers is written as Rust on the
//!   same bit widths (`add eobd, -279 / adc r4b, al` as `overflowing_add`
//!   plus the carry, the `add …, 0x80000000 / jnc` loop counters, `test r3,
//!   r3` on a pointer as `is_null`), and jumps between labels are
//!   `loop`/`break`/`if`; the order of loads, stores, eob branches,
//!   coefficient clearing and loops is the assembly's.
//! - 16x64's `.main_part1` is called twice, the second time with `r6` eight
//!   bytes further, which moves each of its `o(pw_…)` operands two entries
//!   on in the read-only data (`pw_401_4076` becomes `pw_1931_3612`, …): the
//!   function takes `r6` (0 or 2) and selects the pair per call; the
//!   instruction count is the same.
//! - 64x64 sets `r3` to `r2` on its fast path, where the assembly leaves it
//!   stale and adds `32*8` to it without using it.
//! - `vpblendd xm, …, 0xcc` under `WRAP_XMM` is `_mm_blend_epi32::<0xc>`
//!   (only the low four bits of the immediate apply to an `xmm` operand).
//! - Labels that only one function reaches are folded into it
//!   (`iadst_16x4`'s `.end2`/`.end3`, `iadst_8x4`'s `.end3`); x86inc's
//!   `WIN64_RESTORE_XMM`, `PROLOGUE`, `lea r6, [o_base]` and `ALIGN` have no
//!   code.
//! - The compiler rewrites some shuffle sequences (16x4 identity_identity:
//!   two rounds of `punpck{l,h}wd` become `pshufb` and `vpblendd`, and
//!   `packuswb` plus `vpermq` in `WRITE_16X2` two `vextracti128` and
//!   `packuswb`), and the DC-only paths of the 64-point sizes share the
//!   large functions' prologue (stack probe above 4 KiB).
//!
//! # Safety
//! Each entry point is called by the decoder through the DSP table with
//! the contract above, on a CPU with AVX2 (the init assigns it only under
//! `DAV1D_X86_CPU_FLAG_AVX2`). Raw pointers reach `dst`, `c` and the frames;
//! no reference to caller memory is formed.
#![allow(
    clippy::all,
    non_snake_case,
    unused_assignments,
    dead_code,
    unused_variables,
    unused_macros
)]

use core::arch::x86_64::*;
use core::ffi::c_int;

// --- what every part shares ------------------------------------------

/// The vector registers `m0` … `m15` of the assembly, indexed by register number. An `xmN` operand is the low
/// half of `m[N]` (`lo`); a VEX-encoded instruction with an `xmm` destination clears the upper half (`zx`).
pub(crate) type M = [__m256i; 16];
type Dst = *mut u8;
type Coef = *mut i16;

/// The 1-D transform of a pass: `type1`/`type2` of `INV_TXFM_FN` (`dct`, `adst`, `flipadst`, `identity`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum K {
    Dct,
    Adst,
    FlipAdst,
    Identity,
}

/// The register file before the first instruction. (The hardware registers hold whatever the caller left; no
/// kernel reads one before writing it, and zero makes that a non-issue: the compiler drops the stores.)
#[inline(always)]
pub(crate) unsafe fn regs() -> M {
    [_mm256_setzero_si256(); 16]
}

/// `xmN`: the low half of a register.
#[inline(always)]
pub(crate) unsafe fn lo(x: __m256i) -> __m128i {
    _mm256_castsi256_si128(x)
}

/// The register after a VEX-encoded instruction wrote `xmN`: the upper half is zero.
#[inline(always)]
pub(crate) unsafe fn zx(x: __m128i) -> __m256i {
    _mm256_zextsi128_si256(x)
}

/// `[p+off]`: the address only (wrapping, so forming it claims nothing).
#[inline(always)]
pub(crate) fn at<T>(p: *mut T, off: isize) -> *mut T {
    p.wrapping_byte_offset(off)
}

// SECTION_RODATA: a constant is written where the assembly names it. `vpbroadcastd m, [o(pw_A_B)]` is
// `pw2(A, B)` (the words A, B in every doubleword; `pw_mA_B` is `pw2(-A, B)`), `vpbroadcastd m, [o(pw_N)]` and
// `vpbroadcastw` of such a constant are `pw(N)` (`pw_2896x8` is `pw(2896 * 8)`), `[o(pd_2048)]` is `pd(2048)`.
// The compiler places the values in read-only data and broadcasts them as the assembly does.

/// `vpbroadcastd m, [o(pw_A_B)]`
#[inline(always)]
pub(crate) unsafe fn pw2(a: i32, b: i32) -> __m256i {
    _mm256_set1_epi32((((b as u16 as u32) << 16) | (a as u16 as u32)) as i32)
}

/// `vpbroadcastd m, [o(pw_N)]` (`times 2 dw N`)
#[inline(always)]
pub(crate) unsafe fn pw(n: i32) -> __m256i {
    _mm256_set1_epi16(n as i16)
}

/// `vpbroadcastd m, [o(pd_N)]`
#[inline(always)]
pub(crate) unsafe fn pd(n: i32) -> __m256i {
    _mm256_set1_epi32(n)
}

/// `movd xmN, [o(pw_N)]`: the doubleword `N, N` in the low lane, the rest zero.
#[inline(always)]
pub(crate) unsafe fn movd_pw(n: i32) -> __m256i {
    zx(_mm_cvtsi32_si128(
        (((n as u16 as u32) << 16) | (n as u16 as u32)) as i32,
    ))
}

/// `movhps xm, [p]`: the 8 bytes at `p` into the upper half of the `xmm` register, the lower half kept. (Written
/// with an unaligned 8-byte load: `_mm_loadh_pd` reads through an `f64` pointer, which a pixel row is not.)
#[inline(always)]
pub(crate) unsafe fn movhps_ld(x: __m128i, p: *const u8) -> __m128i {
    _mm_unpacklo_epi64(x, _mm_loadl_epi64(p.cast()))
}

/// `movhps [p], xm`: the upper 8 bytes of the register to `p`.
#[inline(always)]
pub(crate) unsafe fn movhps_st(p: *mut u8, x: __m128i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(x))
}

/// `%macro ITX_MUL2X_PACK dst/src, tmp[1-2], rnd, coef[1-2], flags` at `mmsize` 32 (flags: 1 = swap,
/// 2 = interleave, 4 = `coef[1-2]` are registers, then given as their numbers).
#[inline(always)]
unsafe fn itx_mul2x_pack(m: &mut M, r: usize, t1: usize, t2: usize, rnd: usize, c1: i32, c2: i32, flags: u32) {
    if flags & 4 != 0 {
        m[t1] = _mm256_madd_epi16(m[c1 as usize], m[r]); // pmaddwd m%2, m%5, m%1
        m[r] = _mm256_madd_epi16(m[r], m[c2 as usize]); // pmaddwd m%1, m%6
    } else {
        if flags & 1 != 0 {
            m[t1] = pw2(c1, c2); // vpbroadcastd m%2, [o(pw_%5_%6)]
            m[t2] = pw2(-c2, c1); // vpbroadcastd m%3, [o(pw_m%6_%5)]
        } else {
            m[t1] = pw2(-c2, c1); // vpbroadcastd m%2, [o(pw_m%6_%5)]
            m[t2] = pw2(c1, c2); // vpbroadcastd m%3, [o(pw_%5_%6)]
        }
        m[t1] = _mm256_madd_epi16(m[t1], m[r]); // pmaddwd m%2, m%1
        m[r] = _mm256_madd_epi16(m[r], m[t2]); // pmaddwd m%1, m%3
    }
    m[t1] = _mm256_add_epi32(m[t1], m[rnd]); // paddd m%2, m%4
    m[r] = _mm256_add_epi32(m[r], m[rnd]); // paddd m%1, m%4
    if flags & 2 != 0 {
        m[t1] = _mm256_slli_epi32::<4>(m[t1]); // pslld m%2, 4
        m[r] = _mm256_srli_epi32::<12>(m[r]); // psrld m%1, 12
        m[r] = _mm256_blend_epi16::<0xaa>(m[r], m[t1]); // pblendw m%1, m%2, 0xaa
    } else {
        m[t1] = _mm256_srai_epi32::<12>(m[t1]); // psrad m%2, 12
        m[r] = _mm256_srai_epi32::<12>(m[r]); // psrad m%1, 12
        m[r] = _mm256_packs_epi32(m[r], m[t1]); // packssdw m%1, m%2
    }
}

/// `%macro ITX_MUL4X_PACK dst/src, tmp[1-3], rnd, coef[1-4], flags` (flags: 1 = swap, 2 = interleave).
#[inline(always)]
unsafe fn itx_mul4x_pack(m: &mut M, r: usize, t1: usize, t2: usize, t3: usize, rnd: usize, c: [i32; 4], flags: u32) {
    if flags & 1 != 0 {
        m[t2] = pw2(c[2], c[3]); // vpbroadcastd m%3, [o(pw_%8_%9)]
        m[t3] = pw2(-c[3], c[2]); // vpbroadcastd m%4, [o(pw_m%9_%8)]
        m[t1] = zx(lo(pw2(c[0], c[1]))); // vpbroadcastd xm%2, [o(pw_%6_%7)]
        m[t1] = _mm256_blend_epi32::<0xf0>(m[t1], m[t2]); // vpblendd m%2, m%3, 0xf0
        m[t2] = zx(lo(pw2(-c[1], c[0]))); // vpbroadcastd xm%3, [o(pw_m%7_%6)]
    } else {
        m[t2] = pw2(-c[3], c[2]); // vpbroadcastd m%3, [o(pw_m%9_%8)]
        m[t3] = pw2(c[2], c[3]); // vpbroadcastd m%4, [o(pw_%8_%9)]
        m[t1] = zx(lo(pw2(-c[1], c[0]))); // vpbroadcastd xm%2, [o(pw_m%7_%6)]
        m[t1] = _mm256_blend_epi32::<0xf0>(m[t1], m[t2]); // vpblendd m%2, m%3, 0xf0
        m[t2] = zx(lo(pw2(c[0], c[1]))); // vpbroadcastd xm%3, [o(pw_%6_%7)]
    }
    m[t2] = _mm256_blend_epi32::<0xf0>(m[t2], m[t3]); // vpblendd m%3, m%4, 0xf0
    itx_mul2x_pack(m, r, t3, usize::MAX, rnd, t1 as i32, t2 as i32, 4 | flags); // ITX_MUL2X_PACK %1, %4, _, %5, %2, %3, (4|%10)
}

/// `%macro ITX_MULSUB_2W dst/src[1-2], tmp[1-2], rnd, coef[1-2], dst2`:
/// `dst1 = (src1 * coef1 - src2 * coef2 + rnd) >> 12`, `dst2 = (src1 * coef2 + src2 * coef1 + rnd) >> 12`.
/// `rnd` is the number of the register that holds `pd_2048`. A `coef2` below 32 means that both coefficients are
/// registers (`%if %7 < 32`), given as their numbers; otherwise they are the constants of `pw_m<coef2>_<coef1>`
/// and `pw_<coef1>_<coef2>` (a negative `coef1` is the assembly's `m<N>`). `dst2` is the optional eighth
/// argument: the register that receives the second result instead of `src2`.
#[inline(always)]
unsafe fn itx_mulsub_2w(
    m: &mut M,
    r1: usize,
    r2: usize,
    t1: usize,
    t2: usize,
    rnd: usize,
    c1: i32,
    c2: i32,
    dst2: Option<usize>,
) {
    let regs = (0..32).contains(&c2);
    m[t1] = _mm256_unpackhi_epi16(m[r2], m[r1]); // punpckhwd m%3, m%2, m%1
    m[r2] = _mm256_unpacklo_epi16(m[r2], m[r1]); // punpcklwd m%2, m%1
    if regs {
        m[r1] = _mm256_madd_epi16(m[c2 as usize], m[r2]); // pmaddwd m%1, m%7, m%2
        m[t2] = _mm256_madd_epi16(m[c2 as usize], m[t1]); // pmaddwd m%4, m%7, m%3
    } else {
        m[r1] = pw2(-c2, c1); // vpbroadcastd m%1, [o(pw_m%7_%6)]
        m[t2] = _mm256_madd_epi16(m[t1], m[r1]); // pmaddwd m%4, m%3, m%1
        m[r1] = _mm256_madd_epi16(m[r1], m[r2]); // pmaddwd m%1, m%2
    }
    m[t2] = _mm256_add_epi32(m[t2], m[rnd]); // paddd m%4, m%5
    m[r1] = _mm256_add_epi32(m[r1], m[rnd]); // paddd m%1, m%5
    m[t2] = _mm256_srai_epi32::<12>(m[t2]); // psrad m%4, 12
    m[r1] = _mm256_srai_epi32::<12>(m[r1]); // psrad m%1, 12
    m[r1] = _mm256_packs_epi32(m[r1], m[t2]); // packssdw m%1, m%4
    if regs {
        m[t1] = _mm256_madd_epi16(m[t1], m[c1 as usize]); // pmaddwd m%3, m%6
        m[r2] = _mm256_madd_epi16(m[r2], m[c1 as usize]); // pmaddwd m%2, m%6
    } else {
        m[t2] = pw2(c1, c2); // vpbroadcastd m%4, [o(pw_%6_%7)]
        m[t1] = _mm256_madd_epi16(m[t1], m[t2]); // pmaddwd m%3, m%4
        m[r2] = _mm256_madd_epi16(m[r2], m[t2]); // pmaddwd m%2, m%4
    }
    m[t1] = _mm256_add_epi32(m[t1], m[rnd]); // paddd m%3, m%5
    m[r2] = _mm256_add_epi32(m[r2], m[rnd]); // paddd m%2, m%5
    m[t1] = _mm256_srai_epi32::<12>(m[t1]); // psrad m%3, 12
    m[r2] = _mm256_srai_epi32::<12>(m[r2]); // psrad m%2, 12
    match dst2 {
        Some(d) => m[d] = _mm256_packs_epi32(m[r2], m[t1]), // packssdw m%8, m%2, m%3
        None => m[r2] = _mm256_packs_epi32(m[r2], m[t1]),   // packssdw m%2, m%3
    }
}

/// `%macro IDCT4_1D src[1-4], tmp[1-2], pd_2048` (register numbers)
#[inline(always)]
unsafe fn idct4_1d(m: &mut M, r: [usize; 4], t: [usize; 2], rnd: usize) {
    let [r1, r2, r3, r4] = r;
    let [r5, r6] = t;
    itx_mulsub_2w(m, r2, r4, r5, r6, rnd, 1567, 3784, Some(r5)); // t2, t3
    itx_mulsub_2w(m, r1, r3, r4, r6, rnd, 2896, 2896, Some(r4)); // t1, t0
    m[r3] = _mm256_subs_epi16(m[r1], m[r2]); // psubsw m%3, m%1, m%2
    m[r2] = _mm256_adds_epi16(m[r2], m[r1]); // paddsw m%2, m%1
    m[r1] = _mm256_adds_epi16(m[r4], m[r5]); // paddsw m%1, m%4, m%5
    m[r4] = _mm256_subs_epi16(m[r4], m[r5]); // psubsw m%4, m%5
}

/// `%macro IDCT8_1D src[1-8], tmp[1-2], pd_2048` (register numbers)
#[inline(always)]
unsafe fn idct8_1d(m: &mut M, r: [usize; 8], t: [usize; 2], rnd: usize) {
    let [r1, r2, r3, r4, r5, r6, r7, r8] = r;
    let [r9, r10] = t;
    itx_mulsub_2w(m, r6, r4, r9, r10, rnd, 3406, 2276, None); // t5a, t6a
    itx_mulsub_2w(m, r2, r8, r9, r10, rnd, 799, 4017, None); // t4a, t7a
    itx_mulsub_2w(m, r3, r7, r9, r10, rnd, 1567, 3784, None); // t2, t3
    m[r9] = _mm256_adds_epi16(m[r2], m[r6]); // paddsw m%9, m%2, m%6 ; t4
    m[r2] = _mm256_subs_epi16(m[r2], m[r6]); // psubsw m%2, m%6 ; t5a
    m[r10] = _mm256_adds_epi16(m[r8], m[r4]); // paddsw m%10, m%8, m%4 ; t7
    m[r8] = _mm256_subs_epi16(m[r8], m[r4]); // psubsw m%8, m%4 ; t6a
    itx_mulsub_2w(m, r1, r5, r4, r6, rnd, 2896, 2896, None); // t1, t0
    itx_mulsub_2w(m, r8, r2, r4, r6, rnd, 2896, 2896, None); // t5, t6
    m[r6] = _mm256_subs_epi16(m[r1], m[r3]); // psubsw m%6, m%1, m%3 ; dct4 out2
    m[r3] = _mm256_adds_epi16(m[r3], m[r1]); // paddsw m%3, m%1 ; dct4 out1
    m[r1] = _mm256_adds_epi16(m[r5], m[r7]); // paddsw m%1, m%5, m%7 ; dct4 out0
    m[r5] = _mm256_subs_epi16(m[r5], m[r7]); // psubsw m%5, m%7 ; dct4 out3
    m[r7] = _mm256_subs_epi16(m[r3], m[r2]); // psubsw m%7, m%3, m%2 ; out6
    m[r2] = _mm256_adds_epi16(m[r2], m[r3]); // paddsw m%2, m%3 ; out1
    m[r3] = _mm256_adds_epi16(m[r6], m[r8]); // paddsw m%3, m%6, m%8 ; out2
    m[r6] = _mm256_subs_epi16(m[r6], m[r8]); // psubsw m%6, m%8 ; out5
    m[r8] = _mm256_subs_epi16(m[r1], m[r10]); // psubsw m%8, m%1, m%10 ; out7
    m[r1] = _mm256_adds_epi16(m[r1], m[r10]); // paddsw m%1, m%10 ; out0
    m[r4] = _mm256_adds_epi16(m[r5], m[r9]); // paddsw m%4, m%5, m%9 ; out3
    m[r5] = _mm256_subs_epi16(m[r5], m[r9]); // psubsw m%5, m%9 ; out4
}

/// `%macro IDCT16_1D_ODDHALF src[1-8], tmp[1-2], pd_2048` (register numbers; in1 = %1, in3 = %2, in5 = %3,
/// in7 = %4, in9 = %5, in11 = %6, in13 = %7, in15 = %8)
#[inline(always)]
unsafe fn idct16_1d_oddhalf(m: &mut M, r: [usize; 8], t: [usize; 2], rnd: usize) {
    let [r1, r2, r3, r4, r5, r6, r7, r8] = r;
    let [r9, r10] = t;
    itx_mulsub_2w(m, r1, r8, r9, r10, rnd, 401, 4076, None); // t8a, t15a
    itx_mulsub_2w(m, r5, r4, r9, r10, rnd, 3166, 2598, None); // t9a, t14a
    itx_mulsub_2w(m, r3, r6, r9, r10, rnd, 1931, 3612, None); // t10a, t13a
    itx_mulsub_2w(m, r7, r2, r9, r10, rnd, 3920, 1189, None); // t11a, t12a
    m[r9] = _mm256_subs_epi16(m[r2], m[r6]); // psubsw m%9, m%2, m%6 ; t13
    m[r6] = _mm256_adds_epi16(m[r6], m[r2]); // paddsw m%6, m%2 ; t12
    m[r2] = _mm256_subs_epi16(m[r8], m[r4]); // psubsw m%2, m%8, m%4 ; t14
    m[r8] = _mm256_adds_epi16(m[r8], m[r4]); // paddsw m%8, m%4 ; t15
    m[r4] = _mm256_subs_epi16(m[r7], m[r3]); // psubsw m%4, m%7, m%3 ; t10
    m[r3] = _mm256_adds_epi16(m[r3], m[r7]); // paddsw m%3, m%7 ; t11
    m[r7] = _mm256_subs_epi16(m[r1], m[r5]); // psubsw m%7, m%1, m%5 ; t9
    m[r1] = _mm256_adds_epi16(m[r1], m[r5]); // paddsw m%1, m%5 ; t8
    itx_mulsub_2w(m, r2, r7, r5, r10, rnd, 1567, 3784, None); // t9a, t14a
    itx_mulsub_2w(m, r9, r4, r5, r10, rnd, -3784, 1567, None); // t10a, t13a
    m[r5] = _mm256_subs_epi16(m[r1], m[r3]); // psubsw m%5, m%1, m%3 ; t11a
    m[r1] = _mm256_adds_epi16(m[r1], m[r3]); // paddsw m%1, m%3 ; t8a
    m[r3] = _mm256_subs_epi16(m[r7], m[r4]); // psubsw m%3, m%7, m%4 ; t13
    m[r7] = _mm256_adds_epi16(m[r7], m[r4]); // paddsw m%7, m%4 ; t14
    m[r4] = _mm256_subs_epi16(m[r8], m[r6]); // psubsw m%4, m%8, m%6 ; t12a
    m[r8] = _mm256_adds_epi16(m[r8], m[r6]); // paddsw m%8, m%6 ; t15a
    m[r6] = _mm256_subs_epi16(m[r2], m[r9]); // psubsw m%6, m%2, m%9 ; t10
    m[r2] = _mm256_adds_epi16(m[r2], m[r9]); // paddsw m%2, m%9 ; t9
    itx_mulsub_2w(m, r3, r6, r9, r10, rnd, 2896, 2896, None); // t10a, t13a
    itx_mulsub_2w(m, r4, r5, r9, r10, rnd, 2896, 2896, None); // t11, t12
}

/// `%macro WRITE_8X4 coefs[1-2], tmp[1-2], off[1-3]` (default offsets `strideq*1, strideq*2, r3`): adds the two
/// coefficient vectors (a register's value or the memory operand's) to four rows of 8 pixels at `dst`,
/// `dst + off1`, `dst + off2` and `dst + off3`.
#[inline(always)]
unsafe fn write_8x4(m: &mut M, dst: Dst, c1: __m256i, c2: __m256i, t1: usize, t2: usize, off: [isize; 3]) {
    let mut x3 = _mm_loadl_epi64(dst.cast()); // movq xm%3, [dstq]
    x3 = movhps_ld(x3, at(dst, off[0])); // movhps xm%3, [dstq+%5]
    let mut x4 = _mm_loadl_epi64(at(dst, off[1]).cast()); // movq xm%4, [dstq+%6]
    x4 = movhps_ld(x4, at(dst, off[2])); // movhps xm%4, [dstq+%7]
    m[t1] = _mm256_cvtepu8_epi16(x3); // pmovzxbw m%3, xm%3
    m[t2] = _mm256_cvtepu8_epi16(x4); // pmovzxbw m%4, xm%4
    m[t1] = _mm256_add_epi16(m[t1], c1); // paddw m%3, m%1
    m[t2] = _mm256_add_epi16(m[t2], c2); // paddw m%4, m%2
    m[t1] = _mm256_packus_epi16(m[t1], m[t2]); // packuswb m%3, m%4
    m[t2] = zx(_mm256_extracti128_si256::<1>(m[t1])); // vextracti128 xm%4, m%3, 1
    _mm_storel_epi64(dst.cast(), lo(m[t1])); // movq [dstq], xm%3
    movhps_st(at(dst, off[1]), lo(m[t1])); // movhps [dstq+%6], xm%3
    _mm_storel_epi64(at(dst, off[0]).cast(), lo(m[t2])); // movq [dstq+%5], xm%4
    movhps_st(at(dst, off[2]), lo(m[t2])); // movhps [dstq+%7], xm%4
}

/// `%macro WRITE_16X2 coefs[1-2], tmp[1-2], offset[1-2]`: adds the two coefficient vectors to the rows of 16
/// pixels at `dst + off1` and `dst + off2`.
#[inline(always)]
unsafe fn write_16x2(m: &mut M, dst: Dst, c1: __m256i, c2: __m256i, t1: usize, t2: usize, off: [isize; 2]) {
    m[t1] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(dst, off[0]).cast())); // pmovzxbw m%3, [dstq+%5]
    m[t1] = _mm256_add_epi16(m[t1], c1); // paddw m%3, m%1
    m[t2] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(dst, off[1]).cast())); // pmovzxbw m%4, [dstq+%6]
    m[t2] = _mm256_add_epi16(m[t2], c2); // paddw m%4, m%2
    m[t1] = _mm256_packus_epi16(m[t1], m[t2]); // packuswb m%3, m%4
    m[t1] = _mm256_permute4x64_epi64::<0xd8>(m[t1]); // vpermq m%3, m%3, q3120
    _mm_storeu_si128(at(dst, off[0]).cast(), lo(m[t1])); // mova [dstq+%5], xm%3
    _mm_storeu_si128(at(dst, off[1]).cast(), _mm256_extracti128_si256::<1>(m[t1]));
    // vextracti128 [dstq+%6], m%3, 1
}

// The DC-only tails that several sizes jump into. On entry `xm0` is the DC coefficient after its first
// multiplication, `xm1` is `pw_2896x8`, `xm2` the size's rounding factor and `r3d` the number of rows (`eobd`,
// which is 0 on this path, with the height `or`ed in).

/// `.dconly` of `inv_txfm_add_dct_dct_8x8_8bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x8_dconly(m: &mut M, dst: Dst, stride: isize, r3d: c_int) {
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    inv_txfm_add_dct_dct_8x8_dconly2(m, dst, stride, r3d)
}

/// `.dconly2` of `inv_txfm_add_dct_dct_8x8_8bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x8_dconly2(m: &mut M, mut dst: Dst, stride: isize, mut r3d: c_int) {
    m[2] = movd_pw(2048); // movd xm2, [pw_2048]
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
    let r2 = stride * 3; // lea r2, [strideq*3]
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    loop {
        // .dconly_loop:
        write_8x4(m, dst, m[0], m[0], 1, 2, [stride, stride * 2, r2]); // WRITE_8X4 0, 0, 1, 2, strideq*1, strideq*2, r2
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        r3d -= 4; // sub r3d, 4
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `.dconly` of `inv_txfm_add_dct_dct_16x4_8bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x4_dconly(m: &mut M, mut dst: Dst, stride: isize, mut r3d: c_int) {
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[2] = movd_pw(2048); // movd xm2, [pw_2048] ; intentionally rip-relative
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[3] = _mm256_setzero_si256(); // pxor m3, m3
    loop {
        // .dconly_loop:
        m[1] = zx(_mm_loadu_si128(dst.cast())); // mova xm1, [dstq+strideq*0]
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(dst, stride).cast())); // vinserti128 m1, [dstq+strideq*1], 1
        m[2] = _mm256_unpackhi_epi8(m[1], m[3]); // punpckhbw m2, m1, m3
        m[1] = _mm256_unpacklo_epi8(m[1], m[3]); // punpcklbw m1, m3
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
        m[1] = _mm256_packus_epi16(m[1], m[2]); // packuswb m1, m2
        _mm_storeu_si128(dst.cast(), lo(m[1])); // mova [dstq+strideq*0], xm1
        _mm_storeu_si128(at(dst, stride).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+strideq*1], m1, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        r3d -= 2; // sub r3d, 2
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `.dconly` of `inv_txfm_add_dct_dct_32x8_8bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_dconly(m: &mut M, mut dst: Dst, stride: isize, mut r3d: c_int) {
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[2] = movd_pw(2048); // movd xm2, [pw_2048] ; intentionally rip-relative
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[3] = _mm256_setzero_si256(); // pxor m3, m3
    loop {
        // .dconly_loop:
        m[1] = _mm256_loadu_si256(dst.cast()); // mova m1, [dstq]
        m[2] = _mm256_unpackhi_epi8(m[1], m[3]); // punpckhbw m2, m1, m3
        m[1] = _mm256_unpacklo_epi8(m[1], m[3]); // punpcklbw m1, m3
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
        m[1] = _mm256_packus_epi16(m[1], m[2]); // packuswb m1, m2
        _mm256_storeu_si256(dst.cast(), m[1]); // mova [dstq], m1
        dst = at(dst, stride); // add dstq, strideq
        r3d -= 1; // dec r3d
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `cglobal inv_txfm_add_<type1>_<type2>_<size>_8bpc` (`INV_TXFM_FN type1, type2, size` and the sizes' own
/// entry points): the exported function `$name`, which runs `$body(dst, stride, c, eob, $args…)`, the size's
/// function that the kinds of the two passes (or nothing) select at compile time.
macro_rules! inv_txfm_fn {
    ($name:ident, $body:ident $(, $arg:expr)*) => {
        /// # Safety
        /// As for the family (module documentation), and the CPU supports AVX2 (the init selects this function
        /// under `DAV1D_X86_CPU_FLAG_AVX2`).
        #[target_feature(enable = "avx2")]
        pub unsafe extern "C" fn $name(dst: *mut u8, stride: isize, c: *mut i16, eob: c_int) {
            $body(dst, stride, c, eob $(, $arg)*)
        }
    };
}

/// Not in the original: writes zero to every slot of the frame `*buf` (the 32- and 64-point sizes) and returns
/// the pointer to its first slot, as `super::util::zeroed`, with one difference: the zero vector goes through
/// `core::hint::black_box`, so the compiler keeps the 32-byte stores instead of replacing them by a call of
/// `memset` (which it does for these frame sizes with `zeroed`; glibc's `memset` then runs `rep stosb`, which
/// Callgrind counts once per byte: 4096 instructions for the frame of 64x32). The frame is reached only through
/// the returned pointer afterwards.
#[inline(always)]
pub(crate) unsafe fn zeroed_frame<const N: usize>(buf: &mut core::mem::MaybeUninit<[__m256i; N]>) -> *mut __m256i {
    let p = buf.as_mut_ptr().cast::<__m256i>();
    let zero = core::hint::black_box(_mm256_setzero_si256());
    for i in 0..N {
        _mm256_store_si256(p.add(i), zero);
    }
    p
}

// --- part 1: itx_avx2.asm lines 290-1331 and 1870-2147 (ITX4_END, WHT, 4x4, 4x8, 4x16, 8x4, 16x4) --------------------

/// `%macro ITX4_END row[1-4], rnd` (`INIT_XMM`; ends in `ret`): rounds `m0`, `m1` with `pw_<rnd>` unless `rnd`
/// is 0 and adds their four rows of four coefficients to the pixel rows `rows[0..4]` (row numbers; the
/// assembly forms `dstq + strideq * n` from `dstq` and `r2 = dstq + strideq * 2`).
#[inline(always)]
unsafe fn itx4_end(m: &mut M, dst: Dst, stride: isize, rows: [isize; 4], rnd: i32) {
    if rnd != 0 {
        m[2] = zx(lo(pw(rnd))); // vpbroadcastd m2, [o(pw_%5)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw m0, m2
        m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[2]))); // pmulhrsw m1, m2
    }
    // lea r2, [dstq+strideq*2]
    let adr = |i: usize| at(dst, stride * rows[i]).cast::<i32>();
    let mut x2 = _mm_cvtsi32_si128(adr(0).read_unaligned()); // movd m2, [row_adr1]
    x2 = _mm_insert_epi32::<1>(x2, adr(1).read_unaligned()); // pinsrd m2, [row_adr2], 1
    let mut x3 = _mm_cvtsi32_si128(adr(2).read_unaligned()); // movd m3, [row_adr3]
    x3 = _mm_insert_epi32::<1>(x3, adr(3).read_unaligned()); // pinsrd m3, [row_adr4], 1
    x2 = _mm_cvtepu8_epi16(x2); // pmovzxbw m2, m2
    x3 = _mm_cvtepu8_epi16(x3); // pmovzxbw m3, m3
    let x0 = _mm_add_epi16(lo(m[0]), x2); // paddw m0, m2
    let x1 = _mm_add_epi16(lo(m[1]), x3); // paddw m1, m3
    let x0 = _mm_packus_epi16(x0, x1); // packuswb m0, m1
    adr(0).write_unaligned(_mm_cvtsi128_si32(x0)); // movd [row_adr1], m0
    adr(1).write_unaligned(_mm_extract_epi32::<1>(x0)); // pextrd [row_adr2], m0, 1
    adr(2).write_unaligned(_mm_extract_epi32::<2>(x0)); // pextrd [row_adr3], m0, 2
    adr(3).write_unaligned(_mm_extract_epi32::<3>(x0)); // pextrd [row_adr4], m0, 3
    (m[0], m[1], m[2], m[3]) = (zx(x0), zx(x1), zx(x2), zx(x3));
}

/// `%macro IWHT4_1D_PACKED` (`INIT_XMM`)
#[inline(always)]
unsafe fn iwht4_1d_packed(m: &mut M) {
    let (mut m0, mut m1) = (lo(m[0]), lo(m[1]));
    let m3 = _mm_unpackhi_epi64(m0, m1); // punpckhqdq m3, m0, m1 ; in1 in3
    m0 = _mm_unpacklo_epi64(m0, m1); // punpcklqdq m0, m1 ; in0 in2
    let mut m2 = _mm_sub_epi16(m0, m3); // psubw m2, m0, m3
    m0 = _mm_add_epi16(m0, m3); // paddw m0, m3
    m2 = _mm_unpackhi_epi64(m2, m2); // punpckhqdq m2, m2 ; t2 t2
    m0 = _mm_unpacklo_epi64(m0, m0); // punpcklqdq m0, m0 ; t0 t0
    m1 = _mm_sub_epi16(m0, m2); // psubw m1, m0, m2
    m1 = _mm_srai_epi16::<1>(m1); // psraw m1, 1
    m1 = _mm_sub_epi16(m1, m3); // psubw m1, m3 ; t1 t3
    m0 = _mm_sub_epi16(m0, m1); // psubw m0, m1 ; ____ out0
    m2 = _mm_add_epi16(m2, m1); // paddw m2, m1 ; out3 ____
    (m[0], m[1], m[2], m[3]) = (zx(m0), zx(m1), zx(m2), zx(m3));
}

/// `cglobal inv_txfm_add_wht_wht_4x4_8bpc, 3, 3, 4, dst, stride, c` (`INIT_XMM avx2`)
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_inv_txfm_add_wht_wht_4x4_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    c: *mut i16,
    _eob: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    m[0] = zx(_mm_loadu_si128(c.cast())); // mova m0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(c.add(8).cast())); // mova m1, [cq+16*1]
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    _mm_storeu_si128(c.cast(), lo(m[2])); // mova [cq+16*0], m2
    _mm_storeu_si128(c.add(8).cast(), lo(m[2])); // mova [cq+16*1], m2
    m[0] = zx(_mm_srai_epi16::<2>(lo(m[0]))); // psraw m0, 2
    m[1] = zx(_mm_srai_epi16::<2>(lo(m[1]))); // psraw m1, 2
    iwht4_1d_packed(m); // IWHT4_1D_PACKED
    m[0] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[1]))); // punpckhwd m0, m1
    m[3] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[2]))); // punpcklwd m3, m1, m2
    m[1] = zx(_mm_unpackhi_epi32(lo(m[0]), lo(m[3]))); // punpckhdq m1, m0, m3
    m[0] = zx(_mm_unpacklo_epi32(lo(m[0]), lo(m[3]))); // punpckldq m0, m3
    iwht4_1d_packed(m); // IWHT4_1D_PACKED
    m[0] = zx(_mm_blend_epi32::<0x03>(lo(m[0]), lo(m[2]))); // vpblendd m0, m2, 0x03
    itx4_end(m, dst, stride, [3, 0, 2, 1], 0) // ITX4_END 3, 0, 2, 1, 0
}

/// `deint_shuf` (`SECTION_RODATA`)
#[allow(non_upper_case_globals)]
pub(crate) static deint_shuf: [u8; 16] = [0, 1, 4, 5, 8, 9, 12, 13, 2, 3, 6, 7, 10, 11, 14, 15];

// The macros that the assembly expands at both widths have a form per width: the 256-bit form carries the
// macro's name, the 128-bit form (`INIT_XMM` code and `WRAP_XMM …`) the suffix `_x`. In a 128-bit form every
// instruction has an `xmm` destination, so every result is stored with `zx`.

/// `%macro ITX_MUL2X_PACK dst/src, tmp[1-2], rnd, coef[1-2], flags` at `mmsize` 16 (the prelude's
/// `itx_mul2x_pack` is the 256-bit form; same arguments).
#[inline(always)]
unsafe fn itx_mul2x_pack_x(m: &mut M, r: usize, t1: usize, t2: usize, rnd: usize, c1: i32, c2: i32, flags: u32) {
    if flags & 4 != 0 {
        m[t1] = zx(_mm_madd_epi16(lo(m[c1 as usize]), lo(m[r]))); // pmaddwd m%2, m%5, m%1
        m[r] = zx(_mm_madd_epi16(lo(m[r]), lo(m[c2 as usize]))); // pmaddwd m%1, m%6
    } else {
        if flags & 1 != 0 {
            m[t1] = zx(lo(pw2(c1, c2))); // vpbroadcastd m%2, [o(pw_%5_%6)]
            m[t2] = zx(lo(pw2(-c2, c1))); // vpbroadcastd m%3, [o(pw_m%6_%5)]
        } else {
            m[t1] = zx(lo(pw2(-c2, c1))); // vpbroadcastd m%2, [o(pw_m%6_%5)]
            m[t2] = zx(lo(pw2(c1, c2))); // vpbroadcastd m%3, [o(pw_%5_%6)]
        }
        m[t1] = zx(_mm_madd_epi16(lo(m[t1]), lo(m[r]))); // pmaddwd m%2, m%1
        m[r] = zx(_mm_madd_epi16(lo(m[r]), lo(m[t2]))); // pmaddwd m%1, m%3
    }
    m[t1] = zx(_mm_add_epi32(lo(m[t1]), lo(m[rnd]))); // paddd m%2, m%4
    m[r] = zx(_mm_add_epi32(lo(m[r]), lo(m[rnd]))); // paddd m%1, m%4
    if flags & 2 != 0 {
        m[t1] = zx(_mm_slli_epi32::<4>(lo(m[t1]))); // pslld m%2, 4
        m[r] = zx(_mm_srli_epi32::<12>(lo(m[r]))); // psrld m%1, 12
        m[r] = zx(_mm_blend_epi16::<0xaa>(lo(m[r]), lo(m[t1]))); // pblendw m%1, m%2, 0xaa
    } else {
        m[t1] = zx(_mm_srai_epi32::<12>(lo(m[t1]))); // psrad m%2, 12
        m[r] = zx(_mm_srai_epi32::<12>(lo(m[r]))); // psrad m%1, 12
        m[r] = zx(_mm_packs_epi32(lo(m[r]), lo(m[t1]))); // packssdw m%1, m%2
    }
}

/// `%macro IDCT4_1D_PACKED` at `mmsize` 32
#[inline(always)]
unsafe fn idct4_1d_packed(m: &mut M) {
    m[4] = pd(2048); // vpbroadcastd m4, [o(pd_2048)]
    m[2] = _mm256_unpackhi_epi16(m[1], m[0]); // punpckhwd m2, m1, m0
    m[1] = _mm256_unpacklo_epi16(m[1], m[0]); // punpcklwd m1, m0
    itx_mul2x_pack(m, 2, 0, 3, 4, 1567, 3784, 0); // ITX_MUL2X_PACK 2, 0, 3, 4, 1567, 3784
    itx_mul2x_pack(m, 1, 0, 3, 4, 2896, 2896, 0); // ITX_MUL2X_PACK 1, 0, 3, 4, 2896, 2896
    m[0] = _mm256_adds_epi16(m[1], m[2]); // paddsw m0, m1, m2 ; out0 out1
    m[1] = _mm256_subs_epi16(m[1], m[2]); // psubsw m1, m2 ; out3 out2
}

/// `%macro IDCT4_1D_PACKED` at `mmsize` 16
#[inline(always)]
unsafe fn idct4_1d_packed_x(m: &mut M) {
    m[4] = zx(lo(pd(2048))); // vpbroadcastd m4, [o(pd_2048)]
    m[2] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[0]))); // punpckhwd m2, m1, m0
    m[1] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[0]))); // punpcklwd m1, m0
    itx_mul2x_pack_x(m, 2, 0, 3, 4, 1567, 3784, 0); // ITX_MUL2X_PACK 2, 0, 3, 4, 1567, 3784
    itx_mul2x_pack_x(m, 1, 0, 3, 4, 2896, 2896, 0); // ITX_MUL2X_PACK 1, 0, 3, 4, 2896, 2896
    m[0] = zx(_mm_adds_epi16(lo(m[1]), lo(m[2]))); // paddsw m0, m1, m2 ; out0 out1
    m[1] = zx(_mm_subs_epi16(lo(m[1]), lo(m[2]))); // psubsw m1, m2 ; out3 out2
}

/// `%macro IADST4_1D_PACKED` at `mmsize` 32
#[inline(always)]
unsafe fn iadst4_1d_packed(m: &mut M) {
    m[2] = _mm256_unpacklo_epi16(m[1], m[0]); // punpcklwd m2, m1, m0
    m[3] = _mm256_unpackhi_epi16(m[1], m[0]); // punpckhwd m3, m1, m0
    m[5] = pw2(-3344, 3344); // vpbroadcastd m5, [o(pw_m3344_3344)]
    m[0] = pw2(3803, 1321); // vpbroadcastd m0, [o(pw_3803_1321)]
    m[4] = pw2(-1321, 2482); // vpbroadcastd m4, [o(pw_m1321_2482)]
    m[1] = _mm256_madd_epi16(m[5], m[2]); // pmaddwd m1, m5, m2 ; 3344*in3 - 3344*in2
    m[5] = _mm256_srli_epi32::<16>(m[5]); // psrld m5, 16
    m[0] = _mm256_madd_epi16(m[0], m[2]); // pmaddwd m0, m2
    m[2] = _mm256_madd_epi16(m[2], m[4]); // pmaddwd m2, m4
    m[5] = _mm256_madd_epi16(m[5], m[3]); // pmaddwd m5, m3 ; 3344*in0
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; 3344*in0 - 3344*in2 + 3344*in3
    m[4] = pw2(2482, 3344); // vpbroadcastd m4, [o(pw_2482_3344)]
    m[5] = pw2(-3803, 3344); // vpbroadcastd m5, [o(pw_m3803_3344)]
    m[4] = _mm256_madd_epi16(m[4], m[3]); // pmaddwd m4, m3
    m[5] = _mm256_madd_epi16(m[5], m[3]); // pmaddwd m5, m3
    m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0 ; 1321*in0 + 3344*in1 + 3803*in2 + 2482*in3
    m[0] = pw2(-3803, -6688); // vpbroadcastd m0, [o(pw_m3803_m6688)]
    m[3] = _mm256_madd_epi16(m[3], m[0]); // pmaddwd m3, m0
    m[0] = pd(2048); // vpbroadcastd m0, [o(pd_2048)]
    m[2] = _mm256_add_epi32(m[2], m[0]); // paddd m2, m0
    m[1] = _mm256_add_epi32(m[1], m[0]); // paddd m1, m0
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4
    m[5] = _mm256_add_epi32(m[5], m[2]); // paddd m5, m2 ; 2482*in0 + 3344*in1 - 1321*in2 - 3803*in3
    m[2] = _mm256_add_epi32(m[2], m[4]); // paddd m2, m4
    m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3
    m[1] = _mm256_srai_epi32::<12>(m[1]); // REPX {psrad x, 12}, m1, m2, m0, m5
    m[2] = _mm256_srai_epi32::<12>(m[2]);
    m[0] = _mm256_srai_epi32::<12>(m[0]);
    m[5] = _mm256_srai_epi32::<12>(m[5]);
    m[0] = _mm256_packs_epi32(m[0], m[5]); // packssdw m0, m5 ; out0 out1
    m[1] = _mm256_packs_epi32(m[1], m[2]); // packssdw m1, m2 ; out2 out3
}

/// `%macro IADST4_1D_PACKED` at `mmsize` 16
#[inline(always)]
unsafe fn iadst4_1d_packed_x(m: &mut M) {
    m[2] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[0]))); // punpcklwd m2, m1, m0
    m[3] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[0]))); // punpckhwd m3, m1, m0
    m[5] = zx(lo(pw2(-3344, 3344))); // vpbroadcastd m5, [o(pw_m3344_3344)]
    m[0] = zx(lo(pw2(3803, 1321))); // vpbroadcastd m0, [o(pw_3803_1321)]
    m[4] = zx(lo(pw2(-1321, 2482))); // vpbroadcastd m4, [o(pw_m1321_2482)]
    m[1] = zx(_mm_madd_epi16(lo(m[5]), lo(m[2]))); // pmaddwd m1, m5, m2 ; 3344*in3 - 3344*in2
    m[5] = zx(_mm_srli_epi32::<16>(lo(m[5]))); // psrld m5, 16
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[2]))); // pmaddwd m0, m2
    m[2] = zx(_mm_madd_epi16(lo(m[2]), lo(m[4]))); // pmaddwd m2, m4
    m[5] = zx(_mm_madd_epi16(lo(m[5]), lo(m[3]))); // pmaddwd m5, m3 ; 3344*in0
    m[1] = zx(_mm_add_epi32(lo(m[1]), lo(m[5]))); // paddd m1, m5 ; 3344*in0 - 3344*in2 + 3344*in3
    m[4] = zx(lo(pw2(2482, 3344))); // vpbroadcastd m4, [o(pw_2482_3344)]
    m[5] = zx(lo(pw2(-3803, 3344))); // vpbroadcastd m5, [o(pw_m3803_3344)]
    m[4] = zx(_mm_madd_epi16(lo(m[4]), lo(m[3]))); // pmaddwd m4, m3
    m[5] = zx(_mm_madd_epi16(lo(m[5]), lo(m[3]))); // pmaddwd m5, m3
    m[4] = zx(_mm_add_epi32(lo(m[4]), lo(m[0]))); // paddd m4, m0 ; 1321*in0 + 3344*in1 + 3803*in2 + 2482*in3
    m[0] = zx(lo(pw2(-3803, -6688))); // vpbroadcastd m0, [o(pw_m3803_m6688)]
    m[3] = zx(_mm_madd_epi16(lo(m[3]), lo(m[0]))); // pmaddwd m3, m0
    m[0] = zx(lo(pd(2048))); // vpbroadcastd m0, [o(pd_2048)]
    m[2] = zx(_mm_add_epi32(lo(m[2]), lo(m[0]))); // paddd m2, m0
    m[1] = zx(_mm_add_epi32(lo(m[1]), lo(m[0]))); // paddd m1, m0
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[4]))); // paddd m0, m4
    m[5] = zx(_mm_add_epi32(lo(m[5]), lo(m[2]))); // paddd m5, m2 ; 2482*in0 + 3344*in1 - 1321*in2 - 3803*in3
    m[2] = zx(_mm_add_epi32(lo(m[2]), lo(m[4]))); // paddd m2, m4
    m[2] = zx(_mm_add_epi32(lo(m[2]), lo(m[3]))); // paddd m2, m3
    m[1] = zx(_mm_srai_epi32::<12>(lo(m[1]))); // REPX {psrad x, 12}, m1, m2, m0, m5
    m[2] = zx(_mm_srai_epi32::<12>(lo(m[2])));
    m[0] = zx(_mm_srai_epi32::<12>(lo(m[0])));
    m[5] = zx(_mm_srai_epi32::<12>(lo(m[5])));
    m[0] = zx(_mm_packs_epi32(lo(m[0]), lo(m[5]))); // packssdw m0, m5 ; out0 out1
    m[1] = zx(_mm_packs_epi32(lo(m[1]), lo(m[2]))); // packssdw m1, m2 ; out2 out3
}

// --- 4x4 (INIT_XMM avx2) ---------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X4_FN type1, type2` (`INV_TXFM_FN type1, type2, 4x4` and the DC-only path of dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x4(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[0] = zx(_mm_set1_epi16(c.read_unaligned())); // vpbroadcastw m0, [cq]
        m[1] = zx(lo(pw(2896 * 8))); // vpbroadcastd m1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw m0, m1
        c.cast::<c_int>().write_unaligned(eob); // mov [cq], eobd ; 0
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw m0, m1
        m[1] = m[0]; // mova m1, m0
        return iadst_4x4_internal_end2(m, dst, stride); // jmp m(iadst_4x4_internal_8bpc).end2
    }
    match t1 {
        K::Dct => idct_4x4_internal_pass1(m, c),
        K::Adst => iadst_4x4_internal_pass1(m, c),
        K::FlipAdst => iflipadst_4x4_internal_pass1(m, c),
        K::Identity => iidentity_4x4_internal_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x4_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_4x4_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x4_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x4_internal_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_4x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_4x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova m0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova m1, [cq+16*1]
    idct4_1d_packed_x(m); // IDCT4_1D_PACKED
    m[2] = zx(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // mova m2, [o(deint_shuf)]
    m[3] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x7d>(
        _mm_castsi128_ps(lo(m[0])),
        _mm_castsi128_ps(lo(m[1])),
    ))); // shufps m3, m0, m1, q1331
    m[0] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x28>(
        _mm_castsi128_ps(lo(m[0])),
        _mm_castsi128_ps(lo(m[1])),
    ))); // shufps m0, m1, q0220
    m[0] = zx(_mm_shuffle_epi8(lo(m[0]), lo(m[2]))); // pshufb m0, m2
    m[1] = zx(_mm_shuffle_epi8(lo(m[3]), lo(m[2]))); // pshufb m1, m3, m2
}

/// `.pass2` of `idct_4x4_internal_8bpc`
#[inline(always)]
unsafe fn idct_4x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct4_1d_packed_x(m); // IDCT4_1D_PACKED
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    _mm_storeu_si128(c.byte_add(16 * 0).cast(), lo(m[2])); // mova [cq+16*0], m2
    _mm_storeu_si128(c.byte_add(16 * 1).cast(), lo(m[2])); // mova [cq+16*1], m2
    itx4_end(m, dst, stride, [0, 1, 3, 2], 2048) // ITX4_END 0, 1, 3, 2
}

/// `cglobal iadst_4x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_4x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova m0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova m1, [cq+16*1]
    iadst_4x4_internal_main(m); // call .main
    m[3] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[1]))); // punpckhwd m3, m0, m1
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[1]))); // punpcklwd m0, m1
    m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[3]))); // punpckhwd m1, m0, m3
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); // punpcklwd m0, m3
}

/// `.pass2` of `iadst_4x4_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x4_internal_main(m); // call .main
    iadst_4x4_internal_end(m, dst, stride, c)
}

/// `.end` of `iadst_4x4_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x4_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    _mm_storeu_si128(c.byte_add(16 * 0).cast(), lo(m[2])); // mova [cq+16*0], m2
    _mm_storeu_si128(c.byte_add(16 * 1).cast(), lo(m[2])); // mova [cq+16*1], m2
    iadst_4x4_internal_end2(m, dst, stride)
}

/// `.end2` of `iadst_4x4_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x4_internal_end2(m: &mut M, dst: Dst, stride: isize) {
    itx4_end(m, dst, stride, [0, 1, 2, 3], 2048) // ITX4_END 0, 1, 2, 3
}

/// `.main` of `iadst_4x4_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_4x4_internal_main(m: &mut M) {
    iadst4_1d_packed_x(m) // IADST4_1D_PACKED
}

/// `cglobal iflipadst_4x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova m0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova m1, [cq+16*1]
    iadst_4x4_internal_main(m); // call m(iadst_4x4_internal_8bpc).main
    m[2] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[0]))); // punpcklwd m2, m1, m0
    m[1] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[0]))); // punpckhwd m1, m0
    m[0] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[2]))); // punpcklwd m0, m1, m2
    m[1] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[2]))); // punpckhwd m1, m2
}

/// `.pass2` of `iflipadst_4x4_internal_8bpc` (with its `.end` and `.end2`)
#[inline(always)]
unsafe fn iflipadst_4x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x4_internal_main(m); // call m(iadst_4x4_internal_8bpc).main
                                // .end:
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    _mm_storeu_si128(c.byte_add(16 * 0).cast(), lo(m[2])); // mova [cq+16*0], m2
    _mm_storeu_si128(c.byte_add(16 * 1).cast(), lo(m[2])); // mova [cq+16*1], m2
                                                           // .end2:
    itx4_end(m, dst, stride, [3, 2, 1, 0], 2048) // ITX4_END 3, 2, 1, 0
}

/// `cglobal iidentity_4x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova m0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova m1, [cq+16*1]
    m[3] = zx(lo(pw(1697 * 8))); // vpbroadcastd m3, [o(pw_1697x8)]
    m[2] = zx(_mm_mulhrs_epi16(lo(m[3]), lo(m[0]))); // pmulhrsw m2, m3, m0
    m[3] = zx(_mm_mulhrs_epi16(lo(m[3]), lo(m[1]))); // pmulhrsw m3, m1
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw m0, m2
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[3]))); // paddsw m1, m3
    m[2] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[1]))); // punpckhwd m2, m0, m1
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[1]))); // punpcklwd m0, m1
    m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[2]))); // punpckhwd m1, m0, m2
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[2]))); // punpcklwd m0, m2
}

/// `.pass2` of `iidentity_4x4_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_4x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[3] = zx(lo(pw(1697 * 8))); // vpbroadcastd m3, [o(pw_1697x8)]
    m[2] = zx(_mm_mulhrs_epi16(lo(m[3]), lo(m[0]))); // pmulhrsw m2, m3, m0
    m[3] = zx(_mm_mulhrs_epi16(lo(m[3]), lo(m[1]))); // pmulhrsw m3, m1
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw m0, m2
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[3]))); // paddsw m1, m3
    iadst_4x4_internal_end(m, dst, stride, c) // jmp m(iadst_4x4_internal_8bpc).end
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x4_8bpc_avx2,
    inv_txfm_add_4x4,
    K::Identity,
    K::Identity
);

// --- 4x8 (INIT_YMM avx2) ---------------------------------------------------------------------------------------------

/// `%macro WRITE_4X8 coefs[1-2]`: adds the two coefficient registers to eight rows of 4 pixels, the rows
/// `dst + strideq * {0, 1, 2}`, `dst + r3` and the same four from `r2`. Uses `m4` and `m5`.
#[inline(always)]
unsafe fn write_4x8(m: &mut M, dst: Dst, stride: isize, r2: Dst, r3: isize, c1: usize, c2: usize) {
    let ld = |p: Dst| p.cast::<i32>().read_unaligned();
    let st = |p: Dst, v: i32| p.cast::<i32>().write_unaligned(v);
    m[4] = zx(_mm_cvtsi32_si128(ld(dst))); // movd xm4, [dstq+strideq*0]
    m[4] = zx(_mm_insert_epi32::<1>(lo(m[4]), ld(at(dst, stride)))); // pinsrd xm4, [dstq+strideq*1], 1
    m[5] = zx(_mm_cvtsi32_si128(ld(at(dst, stride * 2)))); // movd xm5, [dstq+strideq*2]
    m[5] = zx(_mm_insert_epi32::<1>(lo(m[5]), ld(at(dst, r3)))); // pinsrd xm5, [dstq+r3], 1
    m[4] = zx(_mm_insert_epi32::<2>(lo(m[4]), ld(r2))); // pinsrd xm4, [r2+strideq*0], 2
    m[4] = zx(_mm_insert_epi32::<3>(lo(m[4]), ld(at(r2, stride)))); // pinsrd xm4, [r2+strideq*1], 3
    m[5] = zx(_mm_insert_epi32::<2>(lo(m[5]), ld(at(r2, stride * 2)))); // pinsrd xm5, [r2+strideq*2], 2
    m[5] = zx(_mm_insert_epi32::<3>(lo(m[5]), ld(at(r2, r3)))); // pinsrd xm5, [r2+r3], 3
    m[4] = _mm256_cvtepu8_epi16(lo(m[4])); // pmovzxbw m4, xm4
    m[5] = _mm256_cvtepu8_epi16(lo(m[5])); // pmovzxbw m5, xm5
    m[4] = _mm256_add_epi16(m[4], m[c1]); // paddw m4, m%1
    m[5] = _mm256_add_epi16(m[5], m[c2]); // paddw m5, m%2
    m[4] = _mm256_packus_epi16(m[4], m[5]); // packuswb m4, m5
    m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
    st(dst, _mm_cvtsi128_si32(lo(m[4]))); // movd [dstq+strideq*0], xm4
    st(at(dst, stride), _mm_extract_epi32::<1>(lo(m[4]))); // pextrd [dstq+strideq*1], xm4, 1
    st(at(dst, stride * 2), _mm_extract_epi32::<2>(lo(m[4]))); // pextrd [dstq+strideq*2], xm4, 2
    st(at(dst, r3), _mm_extract_epi32::<3>(lo(m[4]))); // pextrd [dstq+r3], xm4, 3
    st(r2, _mm_cvtsi128_si32(lo(m[5]))); // movd [r2+strideq*0], xm5
    st(at(r2, stride), _mm_extract_epi32::<1>(lo(m[5]))); // pextrd [r2+strideq*1], xm5, 1
    st(at(r2, stride * 2), _mm_extract_epi32::<2>(lo(m[5]))); // pextrd [r2+strideq*2], xm5, 2
    st(at(r2, r3), _mm_extract_epi32::<3>(lo(m[5]))); // pextrd [r2+r3], xm5, 3
}

/// `%macro INV_TXFM_4X8_FN type1, type2` (`INV_TXFM_FN type1, type2, 4x8` and the DC-only path of dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x8(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(2048); // movd xm2, [o(pw_2048)]
        c.cast::<c_int>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
        m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
        m[1] = m[0]; // mova m1, m0
        return iadst_4x8_internal_end3(m, dst, stride); // jmp m(iadst_4x8_internal_8bpc).end3
    }
    match t1 {
        K::Dct => idct_4x8_internal_pass1(m, c),
        K::Adst => iadst_4x8_internal_pass1(m, c),
        K::FlipAdst => iflipadst_4x8_internal_pass1(m, c),
        K::Identity => iidentity_4x8_internal_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x8_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_4x8_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x8_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x8_internal_pass2(m, dst, stride, c),
    }
}

/// `%macro IDCT8_1D_PACKED` at `mmsize` 16 (`WRAP_XMM IDCT8_1D_PACKED`)
#[inline(always)]
unsafe fn idct8_1d_packed_x(m: &mut M) {
    m[6] = zx(lo(pd(2048))); // vpbroadcastd m6, [o(pd_2048)]
    m[5] = zx(_mm_unpackhi_epi16(lo(m[3]), lo(m[0]))); // punpckhwd m5, m3, m0 ; in7 in1
    m[4] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[2]))); // punpckhwd m4, m1, m2 ; in3 in5
    m[3] = zx(_mm_unpacklo_epi16(lo(m[3]), lo(m[1]))); // punpcklwd m3, m1 ; in6 in2
    m[2] = zx(_mm_unpacklo_epi16(lo(m[2]), lo(m[0]))); // punpcklwd m2, m0 ; in4 in0
    itx_mul2x_pack_x(m, 5, 0, 1, 6, 799, 4017, 3); // ITX_MUL2X_PACK 5, 0, 1, 6, 799, 4017, 3 ; t4a t7a
    itx_mul2x_pack_x(m, 4, 0, 1, 6, 3406, 2276, 3); // ITX_MUL2X_PACK 4, 0, 1, 6, 3406, 2276, 3 ; t5a t6a
    itx_mul2x_pack_x(m, 3, 0, 1, 6, 1567, 3784, 0); // ITX_MUL2X_PACK 3, 0, 1, 6, 1567, 3784 ; t3 t2
    m[0] = zx(_mm_subs_epi16(lo(m[5]), lo(m[4]))); // psubsw m0, m5, m4 ; t5a t6a (interleaved)
    m[4] = zx(_mm_adds_epi16(lo(m[4]), lo(m[5]))); // paddsw m4, m5 ; t4 t7 (interleaved)
    itx_mul2x_pack_x(m, 2, 1, 5, 6, 2896, 2896, 0); // ITX_MUL2X_PACK 2, 1, 5, 6, 2896, 2896 ; t0 t1
    m[1] = zx(lo(pw2(-2896, 2896))); // vpbroadcastd m1, [o(pw_m2896_2896)]
    itx_mul2x_pack_x(m, 0, 1, usize::MAX, 6, 1, 5, 4); // ITX_MUL2X_PACK 0, 1, _, 6, 1, 5, 4 ; t6 t5
    m[4] = zx(_mm_shuffle_epi8(lo(m[4]), _mm_loadu_si128(deint_shuf.as_ptr().cast()))); // pshufb m4, [o(deint_shuf)]
    m[1] = zx(_mm_subs_epi16(lo(m[2]), lo(m[3]))); // psubsw m1, m2, m3 ; tmp3 tmp2
    m[3] = zx(_mm_adds_epi16(lo(m[3]), lo(m[2]))); // paddsw m3, m2 ; tmp0 tmp1
    m[2] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x4e>(
        _mm_castsi128_ps(lo(m[4])),
        _mm_castsi128_ps(lo(m[0])),
    ))); // shufps m2, m4, m0, q1032 ; t7 t6
    m[4] = zx(_mm_blend_epi32::<0xc>(lo(m[4]), lo(m[0]))); // vpblendd m4, m0, 0xcc ; t4 t5
    m[0] = zx(_mm_adds_epi16(lo(m[3]), lo(m[2]))); // paddsw m0, m3, m2 ; out0 out1
    m[3] = zx(_mm_subs_epi16(lo(m[3]), lo(m[2]))); // psubsw m3, m2 ; out7 out6
    m[2] = zx(_mm_subs_epi16(lo(m[1]), lo(m[4]))); // psubsw m2, m1, m4 ; out4 out5
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[4]))); // paddsw m1, m4 ; out3 out2
}

/// `%macro IADST8_1D_PACKED pass` at `mmsize` 16 (`WRAP_XMM IADST8_1D_PACKED 1` and `2`)
#[inline(always)]
unsafe fn iadst8_1d_packed_x(m: &mut M, pass: u32) {
    m[6] = zx(lo(pd(2048))); // vpbroadcastd m6, [o(pd_2048)]
    m[0] = zx(_mm_unpackhi_epi16(lo(m[4]), lo(m[3]))); // punpckhwd m0, m4, m3 ; 0 7
    m[1] = zx(_mm_unpackhi_epi16(lo(m[5]), lo(m[2]))); // punpckhwd m1, m5, m2 ; 2 5
    m[2] = zx(_mm_unpacklo_epi16(lo(m[2]), lo(m[5]))); // punpcklwd m2, m5 ; 4 3
    m[3] = zx(_mm_unpacklo_epi16(lo(m[3]), lo(m[4]))); // punpcklwd m3, m4 ; 6 1
    if pass == 1 {
        itx_mul2x_pack_x(m, 0, 4, 5, 6, 401, 4076, 3); // ITX_MUL2X_PACK 0, 4, 5, 6, 401, 4076, 3 ; t1a t0a
        itx_mul2x_pack_x(m, 1, 4, 5, 6, 1931, 3612, 2); // ITX_MUL2X_PACK 1, 4, 5, 6, 1931, 3612, 2 ; t2a t3a
        itx_mul2x_pack_x(m, 2, 4, 5, 6, 3166, 2598, 3); // ITX_MUL2X_PACK 2, 4, 5, 6, 3166, 2598, 3 ; t5a t4a
        itx_mul2x_pack_x(m, 3, 4, 5, 6, 3920, 1189, 2); // ITX_MUL2X_PACK 3, 4, 5, 6, 3920, 1189, 2 ; t6a t7a
        m[4] = zx(_mm_subs_epi16(lo(m[0]), lo(m[2]))); // psubsw m4, m0, m2 ; t5 t4
        m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw m0, m2 ; t1 t0
        m[5] = zx(_mm_subs_epi16(lo(m[1]), lo(m[3]))); // psubsw m5, m1, m3 ; t6 t7
        m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[3]))); // paddsw m1, m3 ; t2 t3
        itx_mul2x_pack_x(m, 4, 2, 3, 6, 1567, 3784, 3); // ITX_MUL2X_PACK 4, 2, 3, 6, 1567, 3784, 3 ; t5a t4a
        itx_mul2x_pack_x(m, 5, 2, 3, 6, 3784, 1567, 2); // ITX_MUL2X_PACK 5, 2, 3, 6, 3784, 1567, 2 ; t7a t6a
        m[2] = zx(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // mova m2, [o(deint_shuf)]
        m[1] = zx(_mm_shufflelo_epi16::<0xb1>(lo(m[1]))); // pshuflw m1, m1, q2301
        m[1] = zx(_mm_shufflehi_epi16::<0xb1>(lo(m[1]))); // pshufhw m1, m1, q2301
        m[3] = zx(_mm_subs_epi16(lo(m[0]), lo(m[1]))); // psubsw m3, m0, m1 ; t3 t2
        m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[1]))); // paddsw m0, m1 ; -out7 out0
        m[1] = zx(_mm_subs_epi16(lo(m[4]), lo(m[5]))); // psubsw m1, m4, m5 ; t7 t6
        m[4] = zx(_mm_adds_epi16(lo(m[4]), lo(m[5]))); // paddsw m4, m5 ; out6 -out1
        m[0] = zx(_mm_shuffle_epi8(lo(m[0]), lo(m[2]))); // pshufb m0, m2
        m[4] = zx(_mm_shuffle_epi8(lo(m[4]), lo(m[2]))); // pshufb m4, m2
        m[5] = zx(lo(pw2(-2896, 2896))); // vpbroadcastd m5, [o(pw_m2896_2896)]
        m[2] = zx(_mm_madd_epi16(lo(m[5]), lo(m[3]))); // pmaddwd m2, m5, m3
        m[5] = zx(_mm_madd_epi16(lo(m[5]), lo(m[1]))); // pmaddwd m5, m1
        m[2] = zx(_mm_add_epi32(lo(m[2]), lo(m[6]))); // paddd m2, m6
        m[5] = zx(_mm_add_epi32(lo(m[5]), lo(m[6]))); // paddd m5, m6
        m[2] = zx(_mm_srai_epi32::<12>(lo(m[2]))); // psrad m2, 12
        m[5] = zx(_mm_srai_epi32::<12>(lo(m[5]))); // psrad m5, 12
        m[2] = zx(_mm_packs_epi32(lo(m[2]), lo(m[5]))); // packssdw m2, m5 ; out4 -out5
        m[5] = zx(lo(pw2(2896, 2896))); // vpbroadcastd m5, [o(pw_2896_2896)]
        m[3] = zx(_mm_madd_epi16(lo(m[3]), lo(m[5]))); // pmaddwd m3, m5
        m[1] = zx(_mm_madd_epi16(lo(m[1]), lo(m[5]))); // pmaddwd m1, m5
        m[3] = zx(_mm_add_epi32(lo(m[3]), lo(m[6]))); // paddd m3, m6
        m[1] = zx(_mm_add_epi32(lo(m[1]), lo(m[6]))); // paddd m1, m6
        m[3] = zx(_mm_srai_epi32::<12>(lo(m[3]))); // psrad m3, 12
        m[1] = zx(_mm_srai_epi32::<12>(lo(m[1]))); // psrad m1, 12
        m[1] = zx(_mm_packs_epi32(lo(m[1]), lo(m[3]))); // packssdw m1, m3 ; out2 -out3
        m[3] = zx(_mm_unpacklo_epi64(lo(m[4]), lo(m[0]))); // punpcklqdq m3, m4, m0 ; out6 -out7
        m[0] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[4]))); // punpckhqdq m0, m4 ; out0 -out1
    } else {
        itx_mul2x_pack_x(m, 0, 4, 5, 6, 401, 4076, 0); // ITX_MUL2X_PACK 0, 4, 5, 6, 401, 4076 ; t0a t1a
        itx_mul2x_pack_x(m, 1, 4, 5, 6, 1931, 3612, 0); // ITX_MUL2X_PACK 1, 4, 5, 6, 1931, 3612 ; t2a t3a
        itx_mul2x_pack_x(m, 2, 4, 5, 6, 3166, 2598, 0); // ITX_MUL2X_PACK 2, 4, 5, 6, 3166, 2598 ; t4a t5a
        itx_mul2x_pack_x(m, 3, 4, 5, 6, 3920, 1189, 0); // ITX_MUL2X_PACK 3, 4, 5, 6, 3920, 1189 ; t6a t7a
        m[4] = zx(_mm_subs_epi16(lo(m[0]), lo(m[2]))); // psubsw m4, m0, m2 ; t4 t5
        m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw m0, m2 ; t0 t1
        m[5] = zx(_mm_subs_epi16(lo(m[1]), lo(m[3]))); // psubsw m5, m1, m3 ; t6 t7
        m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[3]))); // paddsw m1, m3 ; t2 t3
        m[2] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x4e>(
            _mm_castsi128_ps(lo(m[5])),
            _mm_castsi128_ps(lo(m[4])),
        ))); // shufps m2, m5, m4, q1032
        m[4] = zx(_mm_unpackhi_epi16(lo(m[4]), lo(m[2]))); // punpckhwd m4, m2
        m[5] = zx(_mm_unpacklo_epi16(lo(m[5]), lo(m[2]))); // punpcklwd m5, m2
        itx_mul2x_pack_x(m, 4, 2, 3, 6, 1567, 3784, 1); // ITX_MUL2X_PACK 4, 2, 3, 6, 1567, 3784, 1 ; t5a t4a
        itx_mul2x_pack_x(m, 5, 2, 3, 6, 3784, 1567, 0); // ITX_MUL2X_PACK 5, 2, 3, 6, 3784, 1567 ; t7a t6a
        m[2] = zx(_mm_subs_epi16(lo(m[0]), lo(m[1]))); // psubsw m2, m0, m1 ; t2 t3
        m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[1]))); // paddsw m0, m1 ; out0 -out7
        m[1] = zx(_mm_subs_epi16(lo(m[4]), lo(m[5]))); // psubsw m1, m4, m5 ; t7 t6
        m[4] = zx(_mm_adds_epi16(lo(m[4]), lo(m[5]))); // paddsw m4, m5 ; out6 -out1
        m[5] = zx(lo(pw(2896 * 8))); // vpbroadcastd m5, [o(pw_2896x8)]
        m[3] = zx(_mm_blend_epi32::<0x3>(lo(m[0]), lo(m[4]))); // vpblendd m3, m0, m4, 0x33 ; out6 -out7
        m[0] = zx(_mm_blend_epi32::<0xc>(lo(m[0]), lo(m[4]))); // vpblendd m0, m4, 0xcc ; out0 -out1
        m[4] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x4e>(
            _mm_castsi128_ps(lo(m[2])),
            _mm_castsi128_ps(lo(m[1])),
        ))); // shufps m4, m2, m1, q1032 ; t3 t7
        m[1] = zx(_mm_blend_epi32::<0x3>(lo(m[1]), lo(m[2]))); // vpblendd m1, m2, 0x33 ; t2 t6
        m[2] = zx(_mm_subs_epi16(lo(m[1]), lo(m[4]))); // psubsw m2, m1, m4 ; t2-t3 t6-t7
        m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[4]))); // paddsw m1, m4 ; t2+t3 t6+t7
        m[2] = zx(_mm_mulhrs_epi16(lo(m[2]), lo(m[5]))); // pmulhrsw m2, m5 ; out4 -out5
        m[1] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[1]))); // pshufd m1, m1, q1032
        m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[5]))); // pmulhrsw m1, m5 ; out2 -out3
    }
}

/// `cglobal idct_4x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_4x8_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 0).cast())); // vpermq m0, [cq+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 1).cast())); // vpermq m1, [cq+32*1], q3120
    m[2] = pw(2896 * 8); // vpbroadcastd m2, [o(pw_2896x8)]
    m[0] = _mm256_mulhrs_epi16(m[0], m[2]); // pmulhrsw m0, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
    idct4_1d_packed(m); // IDCT4_1D_PACKED
    m[2] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m2, [o(deint_shuf)]
    m[3] = _mm256_castps_si256(_mm256_shuffle_ps::<0x7d>(
        _mm256_castsi256_ps(m[0]),
        _mm256_castsi256_ps(m[1]),
    )); // shufps m3, m0, m1, q1331
    m[0] = _mm256_castps_si256(_mm256_shuffle_ps::<0x28>(
        _mm256_castsi256_ps(m[0]),
        _mm256_castsi256_ps(m[1]),
    )); // shufps m0, m1, q0220
    m[0] = _mm256_shuffle_epi8(m[0], m[2]); // pshufb m0, m2
    m[1] = _mm256_shuffle_epi8(m[3], m[2]); // pshufb m1, m3, m2
}

/// `.pass2` of `idct_4x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_4x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1
    idct_4x8_internal_main(m); // call .main
    m[4] = pw(2048); // vpbroadcastd m4, [o(pw_2048)]
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[3])); // vinserti128 m1, xm3, 1
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
    iadst_4x8_internal_end2(m, dst, stride, c) // jmp m(iadst_4x8_internal_8bpc).end2
}

/// `.main` of `idct_4x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn idct_4x8_internal_main(m: &mut M) {
    idct8_1d_packed_x(m) // WRAP_XMM IDCT8_1D_PACKED
}

/// `cglobal iadst_4x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_4x8_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 0).cast())); // vpermq m0, [cq+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 1).cast())); // vpermq m1, [cq+32*1], q3120
    m[2] = pw(2896 * 8); // vpbroadcastd m2, [o(pw_2896x8)]
    m[0] = _mm256_mulhrs_epi16(m[0], m[2]); // pmulhrsw m0, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
    iadst_8x4_internal_main(m); // call m(iadst_8x4_internal_8bpc).main
    m[3] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m3, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[0], m[3]); // punpckhwd m1, m0, m3
    m[0] = _mm256_unpacklo_epi16(m[0], m[3]); // punpcklwd m0, m3
}

/// `.pass2` of `iadst_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1
    m[4] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[0]))); // pshufd xm4, xm0, q1032
    m[5] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[1]))); // pshufd xm5, xm1, q1032
    iadst_4x8_internal_main_pass2(m); // call .main_pass2
    m[4] = pw(2048); // vpbroadcastd m4, [o(pw_2048)]
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[3])); // vinserti128 m1, xm3, 1
    m[5] = _mm256_setzero_si256(); // pxor m5, m5
    m[5] = _mm256_sub_epi16(m[5], m[4]); // psubw m5, m4
    iadst_4x8_internal_end(m, dst, stride, c)
}

/// `.end` of `iadst_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x8_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = _mm256_blend_epi32::<0xcc>(m[4], m[5]); // vpblendd m4, m5, 0xcc
    iadst_4x8_internal_end2(m, dst, stride, c)
}

/// `.end2` of `iadst_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x8_internal_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    _mm256_storeu_si256(c.byte_add(32 * 0).cast(), m[2]); // mova [cq+32*0], m2
    _mm256_storeu_si256(c.byte_add(32 * 1).cast(), m[2]); // mova [cq+32*1], m2
    iadst_4x8_internal_end3(m, dst, stride)
}

/// `.end3` of `iadst_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x8_internal_end3(m: &mut M, dst: Dst, stride: isize) {
    let r2 = at(dst, stride * 4); // lea r2, [dstq+strideq*4]
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_4x8(m, dst, stride, r2, r3, 0, 1) // WRITE_4X8 0, 1
}

/// `.main_pass1` of `iadst_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x8_internal_main_pass1(m: &mut M) {
    iadst8_1d_packed_x(m, 1) // WRAP_XMM IADST8_1D_PACKED 1
}

/// `.main_pass2` of `iadst_4x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_4x8_internal_main_pass2(m: &mut M) {
    iadst8_1d_packed_x(m, 2) // WRAP_XMM IADST8_1D_PACKED 2
}

/// `cglobal iflipadst_4x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x8_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 0).cast())); // vpermq m0, [cq+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 1).cast())); // vpermq m1, [cq+32*1], q3120
    m[2] = pw(2896 * 8); // vpbroadcastd m2, [o(pw_2896x8)]
    m[0] = _mm256_mulhrs_epi16(m[0], m[2]); // pmulhrsw m0, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
    iadst_8x4_internal_main(m); // call m(iadst_8x4_internal_8bpc).main
    m[3] = _mm256_unpacklo_epi16(m[1], m[0]); // punpcklwd m3, m1, m0
    m[1] = _mm256_unpackhi_epi16(m[1], m[0]); // punpckhwd m1, m0
    m[0] = _mm256_unpacklo_epi16(m[1], m[3]); // punpcklwd m0, m1, m3
    m[1] = _mm256_unpackhi_epi16(m[1], m[3]); // punpckhwd m1, m3
}

/// `.pass2` of `iflipadst_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_4x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1
    m[4] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[0]))); // pshufd xm4, xm0, q1032
    m[5] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[1]))); // pshufd xm5, xm1, q1032
    iadst_4x8_internal_main_pass2(m); // call m(iadst_4x8_internal_8bpc).main_pass2
    m[5] = pw(2048); // vpbroadcastd m5, [o(pw_2048)]
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[1])); // vinserti128 m3, xm1, 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[0])); // vinserti128 m2, xm0, 1
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5
    m[0] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m0, m3, q1032
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m1, m2, q1032
    iadst_4x8_internal_end(m, dst, stride, c) // jmp m(iadst_4x8_internal_8bpc).end
}

/// `cglobal iidentity_4x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x8_internal_pass1(m: &mut M, c: Coef) {
    m[2] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 0).cast())); // vpermq m2, [cq+32*0], q3120
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.byte_add(32 * 1).cast())); // vpermq m0, [cq+32*1], q3120
    m[3] = pw(2896 * 8); // vpbroadcastd m3, [o(pw_2896x8)]
    m[4] = pw(1697 * 8); // vpbroadcastd m4, [o(pw_1697x8)]
    m[1] = _mm256_unpacklo_epi16(m[2], m[0]); // punpcklwd m1, m2, m0
    m[2] = _mm256_unpackhi_epi16(m[2], m[0]); // punpckhwd m2, m0
    m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
    m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
    m[0] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m0, m1, m2
    m[1] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m1, m2
    m[2] = _mm256_mulhrs_epi16(m[4], m[0]); // pmulhrsw m2, m4, m0
    m[4] = _mm256_mulhrs_epi16(m[4], m[1]); // pmulhrsw m4, m1
    m[0] = _mm256_adds_epi16(m[0], m[2]); // paddsw m0, m2
    m[1] = _mm256_adds_epi16(m[1], m[4]); // paddsw m1, m4
}

/// `.pass2` of `iidentity_4x8_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_4x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(4096); // vpbroadcastd m4, [o(pw_4096)]
    iadst_4x8_internal_end2(m, dst, stride, c) // jmp m(iadst_4x8_internal_8bpc).end2
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x8_8bpc_avx2,
    inv_txfm_add_4x8,
    K::Identity,
    K::Identity
);

// --- 8x4 -------------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_8X4_FN type1, type2` (`INV_TXFM_FN type1, type2, 8x4` and the DC-only path of dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_8x4(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        c.cast::<c_int>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        return inv_txfm_add_dct_dct_8x8_dconly2(m, dst, stride, eob); // jmp m(inv_txfm_add_dct_dct_8x8_8bpc).dconly2
    }
    match t1 {
        K::Dct => idct_8x4_internal_pass1(m, c),
        K::Adst => iadst_8x4_internal_pass1(m, c),
        K::FlipAdst => iflipadst_8x4_internal_pass1(m, c),
        K::Identity => iidentity_8x4_internal_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x4_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_8x4_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x4_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x4_internal_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_8x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_8x4_internal_pass1(m: &mut M, c: Coef) {
    m[3] = zx(lo(pw(2896 * 8))); // vpbroadcastd xm3, [o(pw_2896x8)]
    m[0] = zx(_mm_mulhrs_epi16(lo(m[3]), _mm_loadu_si128(c.byte_add(16 * 0).cast()))); // pmulhrsw xm0, xm3, [cq+16*0]
    m[1] = zx(_mm_mulhrs_epi16(lo(m[3]), _mm_loadu_si128(c.byte_add(16 * 1).cast()))); // pmulhrsw xm1, xm3, [cq+16*1]
    m[2] = zx(_mm_mulhrs_epi16(lo(m[3]), _mm_loadu_si128(c.byte_add(16 * 2).cast()))); // pmulhrsw xm2, xm3, [cq+16*2]
    m[3] = zx(_mm_mulhrs_epi16(lo(m[3]), _mm_loadu_si128(c.byte_add(16 * 3).cast()))); // pmulhrsw xm3, [cq+16*3]
    idct_4x8_internal_main(m); // call m(idct_4x8_internal_8bpc).main
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m4, [o(deint_shuf)]
    m[3] = _mm256_inserti128_si256::<1>(m[1], lo(m[3])); // vinserti128 m3, m1, xm3, 1
    m[1] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m1, m0, xm2, 1
    m[0] = _mm256_castps_si256(_mm256_shuffle_ps::<0x28>(
        _mm256_castsi256_ps(m[1]),
        _mm256_castsi256_ps(m[3]),
    )); // shufps m0, m1, m3, q0220
    m[1] = _mm256_castps_si256(_mm256_shuffle_ps::<0x7d>(
        _mm256_castsi256_ps(m[1]),
        _mm256_castsi256_ps(m[3]),
    )); // shufps m1, m3, q1331
    m[0] = _mm256_shuffle_epi8(m[0], m[4]); // pshufb m0, m4
    m[1] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m1, m4
}

/// `.pass2` of `idct_8x4_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct4_1d_packed(m); // IDCT4_1D_PACKED
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    iadst_8x4_internal_end2(m, dst, stride, c) // jmp m(iadst_8x4_internal_8bpc).end2
}

/// `cglobal iadst_8x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_8x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(lo(pw(2896 * 8))); // vpbroadcastd xm0, [o(pw_2896x8)]
    m[4] = zx(_mm_shuffle_epi32::<0x4e>(_mm_loadu_si128(c.byte_add(16 * 0).cast()))); // pshufd xm4, [cq+16*0], q1032
    m[3] = zx(_mm_mulhrs_epi16(lo(m[0]), _mm_loadu_si128(c.byte_add(16 * 3).cast()))); // pmulhrsw xm3, xm0, [cq+16*3]
    m[5] = zx(_mm_shuffle_epi32::<0x4e>(_mm_loadu_si128(c.byte_add(16 * 1).cast()))); // pshufd xm5, [cq+16*1], q1032
    m[2] = zx(_mm_mulhrs_epi16(lo(m[0]), _mm_loadu_si128(c.byte_add(16 * 2).cast()))); // pmulhrsw xm2, xm0, [cq+16*2]
    m[4] = zx(_mm_mulhrs_epi16(lo(m[4]), lo(m[0]))); // pmulhrsw xm4, xm0
    m[5] = zx(_mm_mulhrs_epi16(lo(m[5]), lo(m[0]))); // pmulhrsw xm5, xm0
    iadst_4x8_internal_main_pass1(m); // call m(iadst_4x8_internal_8bpc).main_pass1
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[3])); // vinserti128 m1, xm3, 1
    m[2] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m2, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[3] = _mm256_setzero_si256(); // pxor m3, m3
    m[3] = _mm256_subs_epi16(m[3], m[2]); // psubsw m3, m2
    m[1] = _mm256_unpackhi_epi16(m[0], m[3]); // punpckhwd m1, m0, m3
    m[0] = _mm256_unpacklo_epi16(m[0], m[3]); // punpcklwd m0, m3
}

/// `.pass2` of `iadst_8x4_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x4_internal_main(m); // call .main
    iadst_8x4_internal_end(m, dst, stride, c)
}

/// `.end` of `iadst_8x4_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x4_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
    iadst_8x4_internal_end2(m, dst, stride, c)
}

/// `.end2` of `iadst_8x4_internal_8bpc` (with its `.end3`)
#[inline(always)]
unsafe fn iadst_8x4_internal_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = pw(2048); // vpbroadcastd m2, [o(pw_2048)]
    m[0] = _mm256_mulhrs_epi16(m[0], m[2]); // pmulhrsw m0, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
                                            // .end3:
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    _mm256_storeu_si256(c.byte_add(32 * 0).cast(), m[2]); // mova [cq+32*0], m2
    _mm256_storeu_si256(c.byte_add(32 * 1).cast(), m[2]); // mova [cq+32*1], m2
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_8x4(m, dst, m[0], m[1], 4, 5, [stride, stride * 2, r3]) // WRITE_8X4 0, 1, 4, 5
}

/// `.main` of `iadst_8x4_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_8x4_internal_main(m: &mut M) {
    iadst4_1d_packed(m) // IADST4_1D_PACKED
}

/// `cglobal iflipadst_8x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_8x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(lo(pw(2896 * 8))); // vpbroadcastd xm0, [o(pw_2896x8)]
    m[4] = zx(_mm_shuffle_epi32::<0x4e>(_mm_loadu_si128(c.byte_add(16 * 0).cast()))); // pshufd xm4, [cq+16*0], q1032
    m[3] = zx(_mm_mulhrs_epi16(lo(m[0]), _mm_loadu_si128(c.byte_add(16 * 3).cast()))); // pmulhrsw xm3, xm0, [cq+16*3]
    m[5] = zx(_mm_shuffle_epi32::<0x4e>(_mm_loadu_si128(c.byte_add(16 * 1).cast()))); // pshufd xm5, [cq+16*1], q1032
    m[2] = zx(_mm_mulhrs_epi16(lo(m[0]), _mm_loadu_si128(c.byte_add(16 * 2).cast()))); // pmulhrsw xm2, xm0, [cq+16*2]
    m[4] = zx(_mm_mulhrs_epi16(lo(m[4]), lo(m[0]))); // pmulhrsw xm4, xm0
    m[5] = zx(_mm_mulhrs_epi16(lo(m[5]), lo(m[0]))); // pmulhrsw xm5, xm0
    iadst_4x8_internal_main_pass1(m); // call m(iadst_4x8_internal_8bpc).main_pass1
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[1])); // vinserti128 m3, xm1, 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[0])); // vinserti128 m2, xm0, 1
    m[1] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m1, m3, m2
    m[3] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m3, m2
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    m[0] = _mm256_subs_epi16(m[0], m[1]); // psubsw m0, m1
    m[1] = _mm256_unpackhi_epi16(m[0], m[3]); // punpckhwd m1, m0, m3
    m[0] = _mm256_unpacklo_epi16(m[0], m[3]); // punpcklwd m0, m3
}

/// `.pass2` of `iflipadst_8x4_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_8x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x4_internal_main(m); // call m(iadst_8x4_internal_8bpc).main
    m[2] = m[1]; // mova m2, m1
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[0]); // vpermq m1, m0, q2031
    m[0] = _mm256_permute4x64_epi64::<0x8d>(m[2]); // vpermq m0, m2, q2031
    iadst_8x4_internal_end2(m, dst, stride, c) // jmp m(iadst_8x4_internal_8bpc).end2
}

/// `cglobal iidentity_8x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_8x4_internal_pass1(m: &mut M, c: Coef) {
    m[2] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova xm2, [cq+16*0]
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova xm0, [cq+16*1]
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(c.byte_add(16 * 2).cast())); // vinserti128 m2, [cq+16*2], 1
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(c.byte_add(16 * 3).cast())); // vinserti128 m0, [cq+16*3], 1
    m[3] = pw(2896 * 8); // vpbroadcastd m3, [o(pw_2896x8)]
    m[1] = _mm256_unpacklo_epi16(m[2], m[0]); // punpcklwd m1, m2, m0
    m[2] = _mm256_unpackhi_epi16(m[2], m[0]); // punpckhwd m2, m0
    m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
    m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
    m[0] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m0, m1, m2
    m[1] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m1, m2
    m[0] = _mm256_adds_epi16(m[0], m[0]); // paddsw m0, m0
    m[1] = _mm256_adds_epi16(m[1], m[1]); // paddsw m1, m1
}

/// `.pass2` of `iidentity_8x4_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_8x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[3] = pw(1697 * 8); // vpbroadcastd m3, [o(pw_1697x8)]
    m[2] = _mm256_mulhrs_epi16(m[3], m[0]); // pmulhrsw m2, m3, m0
    m[3] = _mm256_mulhrs_epi16(m[3], m[1]); // pmulhrsw m3, m1
    m[0] = _mm256_adds_epi16(m[0], m[2]); // paddsw m0, m2
    m[1] = _mm256_adds_epi16(m[1], m[3]); // paddsw m1, m3
    iadst_8x4_internal_end(m, dst, stride, c) // jmp m(iadst_8x4_internal_8bpc).end
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x4_8bpc_avx2,
    inv_txfm_add_8x4,
    K::Identity,
    K::Identity
);

// --- 4x16 ------------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X16_FN type1, type2` (`INV_TXFM_FN type1, type2, 4x16` and the DC-only path of dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x16(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        m[3] = movd_pw(2048); // movd xm3, [o(pw_2048)]
        c.cast::<c_int>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[3]))); // pmulhrsw xm0, xm3
        m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
        m[1] = m[0]; // mova m1, m0
        m[2] = m[0]; // mova m2, m0
        m[3] = m[0]; // mova m3, m0
        return iadst_4x16_internal_end3(m, dst, stride); // jmp m(iadst_4x16_internal_8bpc).end3
    }
    match t1 {
        K::Dct => idct_4x16_internal_pass1(m, c),
        K::Adst => iadst_4x16_internal_pass1(m, c),
        K::FlipAdst => iflipadst_4x16_internal_pass1(m, c),
        K::Identity => iidentity_4x16_internal_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x16_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_4x16_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x16_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x16_internal_pass2(m, dst, stride, c),
    }
}

/// `%macro IDCT16_1D_PACKED` at `mmsize` 16 (`WRAP_XMM IDCT16_1D_PACKED`). (The macro's label `.main2`, after
/// the first instruction, has no caller in the 8 bpc file for this expansion.)
#[inline(always)]
unsafe fn idct16_1d_packed_x(m: &mut M) {
    m[10] = zx(lo(pd(2048))); // vpbroadcastd m10, [o(pd_2048)]
    m[8] = zx(_mm_unpackhi_epi16(lo(m[7]), lo(m[0]))); // punpckhwd m8, m7, m0 ; dct16 in15 in1
    m[9] = zx(_mm_unpacklo_epi16(lo(m[4]), lo(m[0]))); // punpcklwd m9, m4, m0 ; dct4 in2 in0
    m[0] = zx(_mm_unpackhi_epi16(lo(m[3]), lo(m[4]))); // punpckhwd m0, m3, m4 ; dct16 in7 in9
    m[7] = zx(_mm_unpacklo_epi16(lo(m[7]), lo(m[1]))); // punpcklwd m7, m1 ; dct8 in7 in1
    m[1] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[6]))); // punpckhwd m1, m6 ; dct16 in3 in13
    m[3] = zx(_mm_unpacklo_epi16(lo(m[3]), lo(m[5]))); // punpcklwd m3, m5 ; dct8 in3 in5
    m[5] = zx(_mm_unpackhi_epi16(lo(m[5]), lo(m[2]))); // punpckhwd m5, m2 ; dct16 in11 in5
    m[6] = zx(_mm_unpacklo_epi16(lo(m[6]), lo(m[2]))); // punpcklwd m6, m2 ; dct4 in3 in1
    itx_mul2x_pack_x(m, 8, 2, 4, 10, 401, 4076, 3); // ITX_MUL2X_PACK 8, 2, 4, 10, 401, 4076, 3 ; t8a t15a
    itx_mul2x_pack_x(m, 0, 2, 4, 10, 3166, 2598, 3); // ITX_MUL2X_PACK 0, 2, 4, 10, 3166, 2598, 3 ; t9a t14a
    itx_mul2x_pack_x(m, 1, 2, 4, 10, 3920, 1189, 3); // ITX_MUL2X_PACK 1, 2, 4, 10, 3920, 1189, 3 ; t11a t12a
    itx_mul2x_pack_x(m, 5, 2, 4, 10, 1931, 3612, 3); // ITX_MUL2X_PACK 5, 2, 4, 10, 1931, 3612, 3 ; t10a t13a
    itx_mul2x_pack_x(m, 7, 2, 4, 10, 799, 4017, 3); // ITX_MUL2X_PACK 7, 2, 4, 10, 799, 4017, 3 ; t4a t7a
    itx_mul2x_pack_x(m, 3, 2, 4, 10, 3406, 2276, 3); // ITX_MUL2X_PACK 3, 2, 4, 10, 3406, 2276, 3 ; t5a t6a
    itx_mul2x_pack_x(m, 6, 2, 4, 10, 1567, 3784, 0); // ITX_MUL2X_PACK 6, 2, 4, 10, 1567, 3784 ; t3 t2
    m[2] = zx(_mm_subs_epi16(lo(m[8]), lo(m[0]))); // psubsw m2, m8, m0 ; t9 t14
    m[8] = zx(_mm_adds_epi16(lo(m[8]), lo(m[0]))); // paddsw m8, m0 ; t8 t15
    m[0] = zx(_mm_subs_epi16(lo(m[1]), lo(m[5]))); // psubsw m0, m1, m5 ; t10 t13
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[5]))); // paddsw m1, m5 ; t11 t12
    m[5] = zx(lo(pw2(-3784, 1567))); // vpbroadcastd m5, [o(pw_m3784_1567)] ; reuse pw_1567_3784
    itx_mul2x_pack_x(m, 2, 4, usize::MAX, 10, 4, 5, 6); // ITX_MUL2X_PACK 2, 4, _, 10, 4, 5, 6 ; t9a t14a
    m[4] = zx(lo(pw2(-1567, -3784))); // vpbroadcastd m4, [o(pw_m1567_m3784)] ; reuse pw_m3784_1567
    itx_mul2x_pack_x(m, 0, 5, usize::MAX, 10, 5, 4, 6); // ITX_MUL2X_PACK 0, 5, _, 10, 5, 4, 6 ; t10a t13a
    m[4] = zx(_mm_subs_epi16(lo(m[8]), lo(m[1]))); // psubsw m4, m8, m1 ; t11a t12a
    m[8] = zx(_mm_adds_epi16(lo(m[8]), lo(m[1]))); // paddsw m8, m1 ; t8a t15a
    m[1] = zx(_mm_subs_epi16(lo(m[7]), lo(m[3]))); // psubsw m1, m7, m3 ; t5a t6a
    m[7] = zx(_mm_adds_epi16(lo(m[7]), lo(m[3]))); // paddsw m7, m3 ; t4 t7
    m[3] = zx(_mm_adds_epi16(lo(m[2]), lo(m[0]))); // paddsw m3, m2, m0 ; t9 t14
    m[2] = zx(_mm_subs_epi16(lo(m[2]), lo(m[0]))); // psubsw m2, m0 ; t10 t13
    m[0] = zx(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // mova m0, [o(deint_shuf)]
    m[8] = zx(_mm_shuffle_epi8(lo(m[8]), lo(m[0]))); // pshufb m8, m0
    m[7] = zx(_mm_shuffle_epi8(lo(m[7]), lo(m[0]))); // pshufb m7, m0
    m[3] = zx(_mm_shuffle_epi8(lo(m[3]), lo(m[0]))); // pshufb m3, m0
    itx_mul2x_pack_x(m, 9, 0, 5, 10, 2896, 2896, 0); // ITX_MUL2X_PACK 9, 0, 5, 10, 2896, 2896 ; t0 t1
    m[0] = zx(lo(pw2(-2896, 2896))); // vpbroadcastd m0, [o(pw_m2896_2896)]
    itx_mul2x_pack_x(m, 4, 5, usize::MAX, 10, 5, 0, 4); // ITX_MUL2X_PACK 4, 5, _, 10, 5, 0, 4 ; t11 t12
    m[5] = zx(lo(pw2(2896, 2896))); // vpbroadcastd m5, [o(pw_2896_2896)]
    itx_mul2x_pack_x(m, 1, 0, usize::MAX, 10, 0, 5, 4); // ITX_MUL2X_PACK 1, 0, _, 10, 0, 5, 4 ; t6 t5
    m[0] = zx(lo(pw2(-2896, 2896))); // vpbroadcastd m0, [o(pw_m2896_2896)]
    itx_mul2x_pack_x(m, 2, 0, usize::MAX, 10, 0, 5, 4); // ITX_MUL2X_PACK 2, 0, _, 10, 0, 5, 4 ; t13a t10a
    m[0] = zx(_mm_unpackhi_epi64(lo(m[8]), lo(m[3]))); // punpckhqdq m0, m8, m3 ; t15a t14
    m[8] = zx(_mm_unpacklo_epi64(lo(m[8]), lo(m[3]))); // punpcklqdq m8, m3 ; t8a t9
    m[5] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x4e>(
        _mm_castsi128_ps(lo(m[4])),
        _mm_castsi128_ps(lo(m[2])),
    ))); // shufps m5, m4, m2, q1032 ; t12 t13a
    m[4] = zx(_mm_blend_epi32::<0xc>(lo(m[4]), lo(m[2]))); // vpblendd m4, m2, 0xcc ; t11 t10a
    m[2] = zx(_mm_castps_si128(_mm_shuffle_ps::<0x4e>(
        _mm_castsi128_ps(lo(m[7])),
        _mm_castsi128_ps(lo(m[1])),
    ))); // shufps m2, m7, m1, q1032 ; t7 t6
    m[7] = zx(_mm_blend_epi32::<0xc>(lo(m[7]), lo(m[1]))); // vpblendd m7, m1, 0xcc ; t4 t5
    m[1] = zx(_mm_subs_epi16(lo(m[9]), lo(m[6]))); // psubsw m1, m9, m6 ; dct4 out3 out2
    m[9] = zx(_mm_adds_epi16(lo(m[9]), lo(m[6]))); // paddsw m9, m6 ; dct4 out0 out1
    m[3] = zx(_mm_subs_epi16(lo(m[9]), lo(m[2]))); // psubsw m3, m9, m2 ; dct8 out7 out6
    m[9] = zx(_mm_adds_epi16(lo(m[9]), lo(m[2]))); // paddsw m9, m2 ; dct8 out0 out1
    m[2] = zx(_mm_subs_epi16(lo(m[1]), lo(m[7]))); // psubsw m2, m1, m7 ; dct8 out4 out5
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[7]))); // paddsw m1, m7 ; dct8 out3 out2
    m[7] = zx(_mm_subs_epi16(lo(m[9]), lo(m[0]))); // psubsw m7, m9, m0 ; out15 out14
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[9]))); // paddsw m0, m9 ; out0 out1
    m[6] = zx(_mm_subs_epi16(lo(m[1]), lo(m[5]))); // psubsw m6, m1, m5 ; out12 out13
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[5]))); // paddsw m1, m5 ; out3 out2
    m[5] = zx(_mm_subs_epi16(lo(m[2]), lo(m[4]))); // psubsw m5, m2, m4 ; out11 out10
    m[2] = zx(_mm_adds_epi16(lo(m[2]), lo(m[4]))); // paddsw m2, m4 ; out4 out5
    m[4] = zx(_mm_subs_epi16(lo(m[3]), lo(m[8]))); // psubsw m4, m3, m8 ; out8 out9
    m[3] = zx(_mm_adds_epi16(lo(m[3]), lo(m[8]))); // paddsw m3, m8 ; out7 out6
}

/// `cglobal idct_4x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_4x16_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_loadu_si256(c.byte_add(32 * 0).cast()); // mova m0, [cq+32*0]
    m[1] = _mm256_loadu_si256(c.byte_add(32 * 1).cast()); // mova m1, [cq+32*1]
    m[2] = _mm256_loadu_si256(c.byte_add(32 * 2).cast()); // mova m2, [cq+32*2]
    m[3] = _mm256_loadu_si256(c.byte_add(32 * 3).cast()); // mova m3, [cq+32*3]
    idct_16x4_internal_main(m); // call m(idct_16x4_internal_8bpc).main
    m[5] = pw(16384); // vpbroadcastd m5, [o(pw_16384)]
    m[4] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m4, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m3, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[0] = _mm256_mulhrs_epi16(m[0], m[5]); // REPX {pmulhrsw x, m5}, m0, m4, m2, m3
    m[4] = _mm256_mulhrs_epi16(m[4], m[5]);
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]);
    m[3] = _mm256_mulhrs_epi16(m[3], m[5]);
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
}

/// `.pass2` of `idct_4x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_4x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm4, m0, 1
    m[5] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm5, m1, 1
    m[6] = zx(_mm256_extracti128_si256::<1>(m[2])); // vextracti128 xm6, m2, 1
    m[7] = zx(_mm256_extracti128_si256::<1>(m[3])); // vextracti128 xm7, m3, 1
    idct_4x16_internal_main(m); // call .main
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m0, xm4, 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[5])); // vinserti128 m1, xm5, 1
    m[5] = pw(2048); // vpbroadcastd m5, [o(pw_2048)]
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[6])); // vinserti128 m2, xm6, 1
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[7])); // vinserti128 m3, xm7, 1
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
    m[3] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m3, m3, q1032
    iadst_4x16_internal_end2(m, dst, stride, c) // jmp m(iadst_4x16_internal_8bpc).end2
}

/// `.main` of `idct_4x16_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn idct_4x16_internal_main(m: &mut M) {
    idct16_1d_packed_x(m) // WRAP_XMM IDCT16_1D_PACKED
}

/// `cglobal iadst_4x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_4x16_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_loadu_si256(c.byte_add(32 * 0).cast()); // mova m0, [cq+32*0]
    m[1] = _mm256_loadu_si256(c.byte_add(32 * 1).cast()); // mova m1, [cq+32*1]
    m[2] = _mm256_loadu_si256(c.byte_add(32 * 2).cast()); // mova m2, [cq+32*2]
    m[3] = _mm256_loadu_si256(c.byte_add(32 * 3).cast()); // mova m3, [cq+32*3]
    iadst_16x4_internal_main(m); // call m(iadst_16x4_internal_8bpc).main
    m[5] = pw(16384); // vpbroadcastd m5, [o(pw_16384)]
    m[4] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m4, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m3, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[4] = _mm256_mulhrs_epi16(m[4], m[5]); // REPX {pmulhrsw x, m5}, m4, m2, m3, m0
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]);
    m[3] = _mm256_mulhrs_epi16(m[3], m[5]);
    m[0] = _mm256_mulhrs_epi16(m[0], m[5]);
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
}

/// `.pass2` of `iadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x16_internal_main(m); // call .main
    m[5] = pw(2896 * 8); // vpbroadcastd m5, [o(pw_2896x8)]
    m[1] = _mm256_adds_epi16(m[2], m[4]); // paddsw m1, m2, m4
    m[2] = _mm256_subs_epi16(m[2], m[4]); // psubsw m2, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5 ; -out7 out4 out6 -out5
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]); // pmulhrsw m2, m5 ; out8 -out11 -out9 out10
    m[5] = pw(2048); // vpbroadcastd m5, [o(pw_2048)]
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
    m[4] = _mm256_blend_epi32::<0x33>(m[1], m[0]); // vpblendd m4, m1, m0, 0x33
    m[0] = _mm256_blend_epi32::<0x33>(m[0], m[2]); // vpblendd m0, m2, 0x33
    m[2] = _mm256_blend_epi32::<0x33>(m[2], m[3]); // vpblendd m2, m3, 0x33
    m[3] = _mm256_blend_epi32::<0x33>(m[3], m[1]); // vpblendd m3, m1, 0x33
    m[0] = _mm256_permute4x64_epi64::<0x8d>(m[0]); // vpermq m0, m0, q2031
    m[1] = _mm256_permute4x64_epi64::<0x72>(m[2]); // vpermq m1, m2, q1302
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[3]); // vpermq m2, m3, q3120
    m[3] = _mm256_permute4x64_epi64::<0x27>(m[4]); // vpermq m3, m4, q0213
    m[6] = _mm256_sub_epi16(m[7], m[5]); // psubw m6, m7, m5
    iadst_4x16_internal_end(m, dst, stride, c)
}

/// `.end` of `iadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[5] = _mm256_blend_epi32::<0xcc>(m[5], m[6]); // vpblendd m5, m6, 0xcc
    iadst_4x16_internal_end2(m, dst, stride, c)
}

/// `.end2` of `iadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_mulhrs_epi16(m[0], m[5]); // REPX {pmulhrsw x, m5}, m0, m1, m2, m3
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]);
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]);
    m[3] = _mm256_mulhrs_epi16(m[3], m[5]);
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.byte_add(32 * 0).cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(c.byte_add(32 * 1).cast(), m[4]); // mova [cq+32*1], m4
    _mm256_storeu_si256(c.byte_add(32 * 2).cast(), m[4]); // mova [cq+32*2], m4
    _mm256_storeu_si256(c.byte_add(32 * 3).cast(), m[4]); // mova [cq+32*3], m4
    iadst_4x16_internal_end3(m, dst, stride)
}

/// `.end3` of `iadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_end3(m: &mut M, mut dst: Dst, stride: isize) {
    let mut r2 = at(dst, stride * 8); // lea r2, [dstq+strideq*8]
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_4x8(m, dst, stride, r2, r3, 0, 1); // WRITE_4X8 0, 1
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    r2 = at(r2, stride * 4); // lea r2, [r2+strideq*4]
    write_4x8(m, dst, stride, r2, r3, 2, 3) // WRITE_4X8 2, 3
}

/// `.main` of `iadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_main(m: &mut M) {
    m[4] = _mm256_blend_epi32::<0xcc>(m[1], m[0]); // vpblendd m4, m1, m0, 0xcc
    m[1] = _mm256_blend_epi32::<0x33>(m[1], m[0]); // vpblendd m1, m0, 0x33
    m[5] = _mm256_blend_epi32::<0xcc>(m[2], m[3]); // vpblendd m5, m2, m3, 0xcc
    m[2] = _mm256_blend_epi32::<0x33>(m[2], m[3]); // vpblendd m2, m3, 0x33
    m[3] = _mm256_permute2x128_si256::<0x31>(m[5], m[2]); // vperm2i128 m3, m5, m2, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[1], lo(m[4])); // vinserti128 m0, m1, xm4, 1 ; in0 in3 in2 in1
    m[4] = _mm256_permute2x128_si256::<0x31>(m[1], m[4]); // vperm2i128 m4, m1, m4, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[5], lo(m[2])); // vinserti128 m1, m5, xm2, 1 ; in4 in7 in6 in5
    m[3] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m3, m3, q1032 ; in15 in12 in13 in14
    m[2] = _mm256_shuffle_epi32::<0x4e>(m[4]); // pshufd m2, m4, q1032 ; in11 in8 in9 in10
    iadst_4x16_internal_main2(m)
}

/// `.main2` of `iadst_4x16_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_4x16_internal_main2(m: &mut M) {
    m[8] = pd(2048); // vpbroadcastd m8, [o(pd_2048)]
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    m[4] = _mm256_unpackhi_epi16(m[3], m[0]); // punpckhwd m4, m3, m0 ; in12 in3 in14 in1
    m[0] = _mm256_unpacklo_epi16(m[0], m[3]); // punpcklwd m0, m3 ; in0 in15 in2 in13
    m[3] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m3, m2, m1 ; in8 in7 in10 in5
    m[1] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m1, m2 ; in4 in11 in6 in9
    itx_mul4x_pack(m, 0, 2, 5, 6, 8, [201, 4091, 995, 3973], 3); // ITX_MUL4X_PACK 0, 2, 5, 6, 8, 201, 4091, 995, 3973, 3
    itx_mul4x_pack(m, 1, 2, 5, 6, 8, [1751, 3703, 2440, 3290], 3); // ITX_MUL4X_PACK 1, 2, 5, 6, 8, 1751, 3703, 2440, 3290, 3
    itx_mul4x_pack(m, 3, 2, 5, 6, 8, [3035, 2751, 3513, 2106], 3); // ITX_MUL4X_PACK 3, 2, 5, 6, 8, 3035, 2751, 3513, 2106, 3
    itx_mul4x_pack(m, 4, 2, 5, 6, 8, [3857, 1380, 4052, 601], 3); // ITX_MUL4X_PACK 4, 2, 5, 6, 8, 3857, 1380, 4052, 601, 3
    m[2] = _mm256_subs_epi16(m[0], m[3]); // psubsw m2, m0, m3 ; t9a t8a t11a t10a
    m[0] = _mm256_adds_epi16(m[0], m[3]); // paddsw m0, m3 ; t1a t0a t3a t2a
    m[3] = _mm256_subs_epi16(m[1], m[4]); // psubsw m3, m1, m4 ; t13a t12a t15a t14a
    m[1] = _mm256_adds_epi16(m[1], m[4]); // paddsw m1, m4 ; t5a t4a t7a t6a
    itx_mul4x_pack(m, 2, 4, 5, 6, 8, [799, 4017, 3406, 2276], 3); // ITX_MUL4X_PACK 2, 4, 5, 6, 8, 799, 4017, 3406, 2276, 3
    m[6] = _mm256_sub_epi16(m[7], m[5]); // psubw m6, m7, m5
    itx_mul2x_pack(m, 3, 5, usize::MAX, 8, 6, 4, 6); // ITX_MUL2X_PACK 3, 5, _, 8, 6, 4, 6
    m[6] = pw2(-3784, 1567); // vpbroadcastd m6, [o(pw_m3784_1567)]
    m[5] = pw2(1567, 3784); // vpbroadcastd m5, [o(pw_1567_3784)]
    m[4] = _mm256_subs_epi16(m[0], m[1]); // psubsw m4, m0, m1 ; t5 t4 t7 t6
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; t1 t0 t3 t2
    m[1] = _mm256_subs_epi16(m[2], m[3]); // psubsw m1, m2, m3 ; t13a t12a t15a t14a
    m[2] = _mm256_adds_epi16(m[2], m[3]); // paddsw m2, m3 ; t9a t8a t11a t10a
    m[3] = _mm256_sub_epi16(m[7], m[6]); // psubw m3, m7, m6 ; pw_3784_m1567
    m[6] = _mm256_blend_epi32::<0xf0>(m[6], m[3]); // vpblendd m6, m3, 0xf0
    itx_mul2x_pack(m, 4, 3, usize::MAX, 8, 6, 5, 4); // ITX_MUL2X_PACK 4, 3, _, 8, 6, 5, 4 ; t4a t5a t7a t6a
    itx_mul2x_pack(m, 1, 3, usize::MAX, 8, 6, 5, 4); // ITX_MUL2X_PACK 1, 3, _, 8, 6, 5, 4 ; t12 t13 t15 t14
    m[5] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m5, [o(deint_shuf)]
    m[0] = _mm256_shuffle_epi8(m[0], m[5]); // pshufb m0, m5
    m[2] = _mm256_shuffle_epi8(m[2], m[5]); // pshufb m2, m5
    m[3] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m3, m0, m2, 0x31 ; t3 t2 t11a t10a
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1 ; t1 t0 t9a t8a
    m[2] = _mm256_permute2x128_si256::<0x31>(m[4], m[1]); // vperm2i128 m2, m4, m1, 0x31 ; t7a t6a t15 t14
    m[4] = _mm256_inserti128_si256::<1>(m[4], lo(m[1])); // vinserti128 m4, xm1, 1 ; t4a t5a t12 t13
    m[2] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m2, m2, q1032 ; t6a t7a t14 t15
    m[1] = _mm256_subs_epi16(m[0], m[3]); // psubsw m1, m0, m3 ; t3a t2a t11 t10
    m[0] = _mm256_adds_epi16(m[0], m[3]); // paddsw m0, m3 ; -out15 out0 out14 -out1
    m[3] = _mm256_adds_epi16(m[4], m[2]); // paddsw m3, m4, m2 ; -out3 out12 out2 -out13
    m[4] = _mm256_subs_epi16(m[4], m[2]); // psubsw m4, m2 ; t6 t7 t14a t15a
    m[2] = _mm256_castps_si256(_mm256_shuffle_ps::<0x4e>(
        _mm256_castsi256_ps(m[1]),
        _mm256_castsi256_ps(m[4]),
    )); // shufps m2, m1, m4, q1032 ; t2a t6 t10 t14a
    m[4] = _mm256_blend_epi32::<0x33>(m[4], m[1]); // vpblendd m4, m1, 0x33 ; t3a t7 t11 t15a
}

/// `.main_pass1_end` of `iadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_main_pass1_end(m: &mut M) {
    m[5] = pw2(-2896, 2896); // vpbroadcastd m5, [o(pw_m2896_2896)]
    m[6] = pw2(2896, 2896); // vpbroadcastd m6, [o(pw_2896_2896)]
    m[1] = _mm256_unpacklo_epi16(m[4], m[2]); // punpcklwd m1, m4, m2
    m[4] = _mm256_unpackhi_epi16(m[4], m[2]); // punpckhwd m4, m2
    m[2] = _mm256_madd_epi16(m[5], m[4]); // pmaddwd m2, m5, m4
    m[4] = _mm256_madd_epi16(m[4], m[6]); // pmaddwd m4, m6
    m[5] = _mm256_madd_epi16(m[5], m[1]); // pmaddwd m5, m1
    m[1] = _mm256_madd_epi16(m[1], m[6]); // pmaddwd m1, m6
    m[5] = _mm256_add_epi32(m[5], m[8]); // REPX {paddd x, m8}, m5, m1, m2, m4
    m[1] = _mm256_add_epi32(m[1], m[8]);
    m[2] = _mm256_add_epi32(m[2], m[8]);
    m[4] = _mm256_add_epi32(m[4], m[8]);
    m[5] = _mm256_srai_epi32::<12>(m[5]); // REPX {psrad x, 12}, m5, m2, m1, m4
    m[2] = _mm256_srai_epi32::<12>(m[2]);
    m[1] = _mm256_srai_epi32::<12>(m[1]);
    m[4] = _mm256_srai_epi32::<12>(m[4]);
    m[2] = _mm256_packs_epi32(m[2], m[5]); // packssdw m2, m5 ; -out11 out8 out10 -out9
    m[1] = _mm256_packs_epi32(m[1], m[4]); // packssdw m1, m4 ; -out7 out4 out6 -out5
}

/// `cglobal iflipadst_4x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x16_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_loadu_si256(c.byte_add(32 * 0).cast()); // mova m0, [cq+32*0]
    m[1] = _mm256_loadu_si256(c.byte_add(32 * 1).cast()); // mova m1, [cq+32*1]
    m[2] = _mm256_loadu_si256(c.byte_add(32 * 2).cast()); // mova m2, [cq+32*2]
    m[3] = _mm256_loadu_si256(c.byte_add(32 * 3).cast()); // mova m3, [cq+32*3]
    iadst_16x4_internal_main(m); // call m(iadst_16x4_internal_8bpc).main
    m[5] = pw(16384); // vpbroadcastd m5, [o(pw_16384)]
    m[4] = _mm256_unpacklo_epi16(m[1], m[0]); // punpcklwd m4, m1, m0
    m[1] = _mm256_unpackhi_epi16(m[1], m[0]); // punpckhwd m1, m0
    m[0] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m0, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[4] = _mm256_mulhrs_epi16(m[4], m[5]); // REPX {pmulhrsw x, m5}, m4, m1, m0, m3
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]);
    m[0] = _mm256_mulhrs_epi16(m[0], m[5]);
    m[3] = _mm256_mulhrs_epi16(m[3], m[5]);
    m[2] = _mm256_unpacklo_epi32(m[3], m[1]); // punpckldq m2, m3, m1
    m[3] = _mm256_unpackhi_epi32(m[3], m[1]); // punpckhdq m3, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[4]); // punpckhdq m1, m0, m4
    m[0] = _mm256_unpacklo_epi32(m[0], m[4]); // punpckldq m0, m4
}

/// `.pass2` of `iflipadst_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_4x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x16_internal_main(m); // call m(iadst_4x16_internal_8bpc).main
    m[5] = pw(2896 * 8); // vpbroadcastd m5, [o(pw_2896x8)]
    m[1] = _mm256_adds_epi16(m[2], m[4]); // paddsw m1, m2, m4
    m[2] = _mm256_subs_epi16(m[2], m[4]); // psubsw m2, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5 ; -out7 out4 out6 -out5
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]); // pmulhrsw m2, m5 ; out8 -out11 -out9 out10
    m[6] = pw(2048); // vpbroadcastd m6, [o(pw_2048)]
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
    m[4] = _mm256_blend_epi32::<0x33>(m[0], m[2]); // vpblendd m4, m0, m2, 0x33
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[1]); // vpblendd m0, m1, 0xcc
    m[1] = _mm256_blend_epi32::<0xcc>(m[1], m[3]); // vpblendd m1, m3, 0xcc
    m[2] = _mm256_blend_epi32::<0x33>(m[2], m[3]); // vpblendd m2, m3, 0x33
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x27>(m[1]); // vpermq m1, m1, q0213
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[2]); // vpermq m2, m2, q2031
    m[3] = _mm256_permute4x64_epi64::<0x72>(m[4]); // vpermq m3, m4, q1302
    m[5] = _mm256_sub_epi16(m[7], m[6]); // psubw m5, m7, m6
    iadst_4x16_internal_end(m, dst, stride, c) // jmp m(iadst_4x16_internal_8bpc).end
}

/// `cglobal iidentity_4x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x16_internal_pass1(m: &mut M, c: Coef) {
    m[3] = _mm256_loadu_si256(c.byte_add(32 * 0).cast()); // mova m3, [cq+32*0]
    m[2] = _mm256_loadu_si256(c.byte_add(32 * 1).cast()); // mova m2, [cq+32*1]
    m[4] = _mm256_loadu_si256(c.byte_add(32 * 2).cast()); // mova m4, [cq+32*2]
    m[5] = _mm256_loadu_si256(c.byte_add(32 * 3).cast()); // mova m5, [cq+32*3]
    m[8] = pw(1697 * 8); // vpbroadcastd m8, [o(pw_1697x8)]
    m[0] = _mm256_cmpeq_epi16(m[0], m[0]); // pcmpeqw m0, m0 ; -1
    m[1] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m1, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[2] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m2, m4, m5
    m[4] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m4, m5
    m[5] = _mm256_mulhrs_epi16(m[8], m[1]); // pmulhrsw m5, m8, m1
    m[6] = _mm256_mulhrs_epi16(m[8], m[2]); // pmulhrsw m6, m8, m2
    m[7] = _mm256_mulhrs_epi16(m[8], m[3]); // pmulhrsw m7, m8, m3
    m[8] = _mm256_mulhrs_epi16(m[8], m[4]); // pmulhrsw m8, m4
    m[9] = _mm256_cmpeq_epi16(m[0], m[1]); // pcmpeqw m9, m0, m1 ; we want to do a signed avg, but pavgw is
    m[1] = _mm256_xor_si256(m[1], m[9]); // pxor m1, m9 ; unsigned. as long as both signs are equal
    m[9] = _mm256_cmpeq_epi16(m[0], m[2]); // pcmpeqw m9, m0, m2 ; it still works, but if the input is -1 the
    m[2] = _mm256_xor_si256(m[2], m[9]); // pxor m2, m9 ; pmulhrsw result will become 0 which causes
    m[9] = _mm256_cmpeq_epi16(m[0], m[3]); // pcmpeqw m9, m0, m3 ; pavgw to output -32768 instead of 0 unless
    m[3] = _mm256_xor_si256(m[3], m[9]); // pxor m3, m9 ; we explicitly deal with that case here.
    m[0] = _mm256_cmpeq_epi16(m[0], m[4]); // pcmpeqw m0, m4
    m[4] = _mm256_xor_si256(m[4], m[0]); // pxor m4, m0
    m[1] = _mm256_avg_epu16(m[1], m[5]); // pavgw m1, m5
    m[2] = _mm256_avg_epu16(m[2], m[6]); // pavgw m2, m6
    m[3] = _mm256_avg_epu16(m[3], m[7]); // pavgw m3, m7
    m[4] = _mm256_avg_epu16(m[4], m[8]); // pavgw m4, m8
    m[0] = _mm256_unpacklo_epi32(m[1], m[2]); // punpckldq m0, m1, m2
    m[1] = _mm256_unpackhi_epi32(m[1], m[2]); // punpckhdq m1, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
}

/// `.pass2` of `iidentity_4x16_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_4x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pw(1697 * 16); // vpbroadcastd m8, [o(pw_1697x16)]
    m[5] = pw(2048); // vpbroadcastd m5, [o(pw_2048)]
    m[4] = _mm256_mulhrs_epi16(m[8], m[0]); // pmulhrsw m4, m8, m0
    m[6] = _mm256_mulhrs_epi16(m[8], m[1]); // pmulhrsw m6, m8, m1
    m[7] = _mm256_mulhrs_epi16(m[8], m[2]); // pmulhrsw m7, m8, m2
    m[8] = _mm256_mulhrs_epi16(m[8], m[3]); // pmulhrsw m8, m3
    m[0] = _mm256_adds_epi16(m[0], m[0]); // REPX {paddsw x, x}, m0, m1, m2, m3
    m[1] = _mm256_adds_epi16(m[1], m[1]);
    m[2] = _mm256_adds_epi16(m[2], m[2]);
    m[3] = _mm256_adds_epi16(m[3], m[3]);
    m[0] = _mm256_adds_epi16(m[0], m[4]); // paddsw m0, m4
    m[1] = _mm256_adds_epi16(m[1], m[6]); // paddsw m1, m6
    m[2] = _mm256_adds_epi16(m[2], m[7]); // paddsw m2, m7
    m[3] = _mm256_adds_epi16(m[3], m[8]); // paddsw m3, m8
    iadst_4x16_internal_end2(m, dst, stride, c) // jmp m(iadst_4x16_internal_8bpc).end2
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x16_8bpc_avx2,
    inv_txfm_add_4x16,
    K::Identity,
    K::Identity
);

// --- 16x4 (INIT_YMM avx2) --------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_16X4_FN type1, type2` (`INV_TXFM_FN type1, type2, 16x4` and the DC-only path of dct_dct; its
/// `.dconly` is the prelude's `inv_txfm_add_dct_dct_16x4_dconly`)
#[inline(always)]
unsafe fn inv_txfm_add_16x4(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<c_int>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 4; // or r3d, 4
        return inv_txfm_add_dct_dct_16x4_dconly(m, dst, stride, r3d); // .dconly:
    }
    match t1 {
        K::Dct => idct_16x4_internal_pass1(m, c),
        K::Adst => iadst_16x4_internal_pass1(m, c),
        K::FlipAdst => iflipadst_16x4_internal_pass1(m, c),
        K::Identity => iidentity_16x4_internal_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x4_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_16x4_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_16x4_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_16x4_internal_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_16x4_internal_8bpc` up to `jmp tx2q` (which it reaches through `iadst_16x4_internal_8bpc`'s
/// `.pass1_end`)
#[inline(always)]
unsafe fn idct_16x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova xm0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova xm1, [cq+16*1]
    m[2] = zx(_mm_loadu_si128(c.byte_add(16 * 2).cast())); // mova xm2, [cq+16*2]
    m[3] = zx(_mm_loadu_si128(c.byte_add(16 * 3).cast())); // mova xm3, [cq+16*3]
    m[4] = zx(_mm_loadu_si128(c.byte_add(16 * 4).cast())); // mova xm4, [cq+16*4]
    m[5] = zx(_mm_loadu_si128(c.byte_add(16 * 5).cast())); // mova xm5, [cq+16*5]
    m[6] = zx(_mm_loadu_si128(c.byte_add(16 * 6).cast())); // mova xm6, [cq+16*6]
    m[7] = zx(_mm_loadu_si128(c.byte_add(16 * 7).cast())); // mova xm7, [cq+16*7]
    idct_4x16_internal_main(m); // call m(idct_4x16_internal_8bpc).main
    m[6] = _mm256_inserti128_si256::<1>(m[2], lo(m[6])); // vinserti128 m6, m2, xm6, 1
    m[2] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m2, m0, xm4, 1
    m[0] = _mm256_inserti128_si256::<1>(m[1], lo(m[5])); // vinserti128 m0, m1, xm5, 1
    m[1] = _mm256_inserti128_si256::<1>(m[3], lo(m[7])); // vinserti128 m1, m3, xm7, 1
    m[3] = _mm256_unpacklo_epi16(m[2], m[6]); // punpcklwd m3, m2, m6
    m[2] = _mm256_unpackhi_epi16(m[2], m[6]); // punpckhwd m2, m6
    m[6] = pw(16384); // vpbroadcastd m6, [o(pw_16384)]
    m[4] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m4, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = m[6]; // mova m1, m6
    iadst_16x4_internal_pass1_end(m) // jmp m(iadst_16x4_internal_8bpc).pass1_end
}

/// `.pass2` of `idct_16x4_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x4_internal_main(m); // call .main
    iadst_16x4_internal_end(m, dst, stride, c) // jmp m(iadst_16x4_internal_8bpc).end
}

/// `.main` of `idct_16x4_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn idct_16x4_internal_main(m: &mut M) {
    m[6] = pd(2048); // vpbroadcastd m6, [o(pd_2048)]
    idct4_1d(m, [0, 1, 2, 3], [4, 5], 6); // IDCT4_1D 0, 1, 2, 3, 4, 5, 6
}

/// `cglobal iadst_16x4_internal_8bpc` up to `.pass1_end`
#[inline(always)]
unsafe fn iadst_16x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0x6c>(_mm256_loadu_si256(c.byte_add(32 * 0).cast())); // vpermq m0, [cq+32*0], q1230
    m[3] = _mm256_permute4x64_epi64::<0x93>(_mm256_loadu_si256(c.byte_add(32 * 3).cast())); // vpermq m3, [cq+32*3], q2103
    m[1] = _mm256_permute4x64_epi64::<0x6c>(_mm256_loadu_si256(c.byte_add(32 * 1).cast())); // vpermq m1, [cq+32*1], q1230
    m[2] = _mm256_permute4x64_epi64::<0x93>(_mm256_loadu_si256(c.byte_add(32 * 2).cast())); // vpermq m2, [cq+32*2], q2103
    iadst_4x16_internal_main2(m); // call m(iadst_4x16_internal_8bpc).main2
    iadst_4x16_internal_main_pass1_end(m); // call m(iadst_4x16_internal_8bpc).main_pass1_end
    m[4] = _mm256_unpacklo_epi16(m[3], m[1]); // punpcklwd m4, m3, m1
    m[5] = _mm256_unpacklo_epi16(m[2], m[0]); // punpcklwd m5, m2, m0
    m[0] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m0, m1
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[1] = pw(16384); // vpbroadcastd m1, [o(pw_16384)]
    m[3] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m3, m0, xm2, 1
    m[2] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m2, m0, m2, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[5])); // vinserti128 m0, m4, xm5, 1
    m[4] = _mm256_permute2x128_si256::<0x31>(m[4], m[5]); // vperm2i128 m4, m4, m5, 0x31
    m[6] = _mm256_sub_epi16(m[7], m[1]); // psubw m6, m7, m1
    iadst_16x4_internal_pass1_end(m)
}

/// `.pass1_end` of `iadst_16x4_internal_8bpc` (up to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_16x4_internal_pass1_end(m: &mut M) {
    m[3] = _mm256_mulhrs_epi16(m[3], m[1]); // pmulhrsw m3, m1
    m[2] = _mm256_mulhrs_epi16(m[2], m[6]); // pmulhrsw m2, m6
    m[4] = _mm256_mulhrs_epi16(m[4], m[1]); // pmulhrsw m4, m1
    m[0] = _mm256_mulhrs_epi16(m[0], m[6]); // pmulhrsw m0, m6
    m[1] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m1, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[2] = _mm256_unpacklo_epi16(m[4], m[0]); // punpcklwd m2, m4, m0
    m[4] = _mm256_unpackhi_epi16(m[4], m[0]); // punpckhwd m4, m0
    m[0] = _mm256_unpacklo_epi32(m[1], m[2]); // punpckldq m0, m1, m2
    m[1] = _mm256_unpackhi_epi32(m[1], m[2]); // punpckhdq m1, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
}

/// `.pass2` of `iadst_16x4_internal_8bpc`
#[inline(always)]
unsafe fn iadst_16x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_16x4_internal_main(m); // call .main
    iadst_16x4_internal_end(m, dst, stride, c)
}

/// `.end` of `iadst_16x4_internal_8bpc` (with its `.end2` and `.end3`, which have no other caller)
#[inline(always)]
unsafe fn iadst_16x4_internal_end(m: &mut M, mut dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(2048); // vpbroadcastd m4, [o(pw_2048)]
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // REPX {pmulhrsw x, m4}, m0, m1, m2, m3
    m[1] = _mm256_mulhrs_epi16(m[1], m[4]);
    m[2] = _mm256_mulhrs_epi16(m[2], m[4]);
    m[3] = _mm256_mulhrs_epi16(m[3], m[4]);
    // .end2:
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.byte_add(32 * 0).cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(c.byte_add(32 * 1).cast(), m[4]); // mova [cq+32*1], m4
    _mm256_storeu_si256(c.byte_add(32 * 2).cast(), m[4]); // mova [cq+32*2], m4
    _mm256_storeu_si256(c.byte_add(32 * 3).cast(), m[4]); // mova [cq+32*3], m4
                                                          // .end3:
    write_16x2(m, dst, m[0], m[1], 4, 5, [0, stride]); // WRITE_16X2 0, 1, 4, 5, strideq*0, strideq*1
    dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
    write_16x2(m, dst, m[2], m[3], 4, 5, [0, stride]); // WRITE_16X2 2, 3, 4, 5, strideq*0, strideq*1
}

/// `.main` of `iadst_16x4_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_16x4_internal_main(m: &mut M) {
    m[6] = pw2(-3344, 3344); // vpbroadcastd m6, [o(pw_m3344_3344)]
    m[7] = pw2(3803, 1321); // vpbroadcastd m7, [o(pw_3803_1321)]
    m[8] = pw2(-1321, 2482); // vpbroadcastd m8, [o(pw_m1321_2482)]
    m[9] = pw2(2482, 3344); // vpbroadcastd m9, [o(pw_2482_3344)]
    m[4] = _mm256_unpacklo_epi16(m[2], m[0]); // punpcklwd m4, m2, m0 ; in2 in0 l
    m[2] = _mm256_unpackhi_epi16(m[2], m[0]); // punpckhwd m2, m0 ; in2 in0 h
    m[5] = _mm256_srli_epi32::<16>(m[6]); // psrld m5, m6, 16
    m[10] = _mm256_madd_epi16(m[6], m[4]); // pmaddwd m10, m6, m4 ; t2:02 l
    m[6] = _mm256_madd_epi16(m[6], m[2]); // pmaddwd m6, m2 ; t2:02 h
    m[0] = _mm256_madd_epi16(m[7], m[4]); // pmaddwd m0, m7, m4 ; t0:02 l
    m[7] = _mm256_madd_epi16(m[7], m[2]); // pmaddwd m7, m2 ; t0:02 h
    m[4] = _mm256_madd_epi16(m[4], m[8]); // pmaddwd m4, m8 ; t1:02 l
    m[8] = _mm256_madd_epi16(m[8], m[2]); // pmaddwd m8, m2 ; t1:02 h
    m[2] = _mm256_unpackhi_epi16(m[3], m[1]); // punpckhwd m2, m3, m1 ; in3 in1 h
    m[3] = _mm256_unpacklo_epi16(m[3], m[1]); // punpcklwd m3, m1 ; in3 in1 l
    m[1] = _mm256_madd_epi16(m[5], m[2]); // pmaddwd m1, m5, m2 ; t2:3 h
    m[5] = _mm256_madd_epi16(m[5], m[3]); // pmaddwd m5, m3 ; t2:3 l
    m[6] = _mm256_add_epi32(m[6], m[1]); // paddd m6, m1
    m[1] = pd(2048); // vpbroadcastd m1, [o(pd_2048)]
    m[10] = _mm256_add_epi32(m[10], m[5]); // paddd m10, m5
    m[5] = _mm256_madd_epi16(m[9], m[3]); // pmaddwd m5, m9, m3
    m[9] = _mm256_madd_epi16(m[9], m[2]); // pmaddwd m9, m2
    m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1
    m[7] = _mm256_add_epi32(m[7], m[1]); // paddd m7, m1
    m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5 ; t0 + t3 + 2048 l
    m[7] = _mm256_add_epi32(m[7], m[9]); // paddd m7, m9 ; t0 + t3 + 2048 h
    m[9] = pw2(-3803, 3344); // vpbroadcastd m9, [o(pw_m3803_3344)]
    m[5] = _mm256_madd_epi16(m[9], m[2]); // pmaddwd m5, m9, m2
    m[9] = _mm256_madd_epi16(m[9], m[3]); // pmaddwd m9, m3
    m[10] = _mm256_add_epi32(m[10], m[1]); // paddd m10, m1 ; t2 + 2048 l
    m[6] = _mm256_add_epi32(m[6], m[1]); // paddd m6, m1 ; t2 + 2048 h
    m[5] = _mm256_add_epi32(m[5], m[1]); // paddd m5, m1 ; t1:13 + 2048 h
    m[1] = _mm256_add_epi32(m[1], m[9]); // paddd m1, m9 ; t1:13 + 2048 l
    m[9] = pw2(-3803, -6688); // vpbroadcastd m9, [o(pw_m3803_m6688)]
    m[2] = _mm256_madd_epi16(m[2], m[9]); // pmaddwd m2, m9
    m[3] = _mm256_madd_epi16(m[3], m[9]); // pmaddwd m3, m9
    m[5] = _mm256_add_epi32(m[5], m[8]); // paddd m5, m8 ; t1 + t3 + 2048 h
    m[1] = _mm256_add_epi32(m[1], m[4]); // paddd m1, m4 ; t1 + t3 + 2048 l
    m[8] = _mm256_add_epi32(m[8], m[7]); // paddd m8, m7
    m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
    m[2] = _mm256_add_epi32(m[2], m[8]); // paddd m2, m8 ; t0 + t1 - t3 + 2048 h
    m[3] = _mm256_add_epi32(m[3], m[4]); // paddd m3, m4 ; t0 + t1 - t3 + 2048 l
    m[10] = _mm256_srai_epi32::<12>(m[10]); // REPX {psrad x, 12}, m10, m6, m0, m7, m5, m1, m2, m3
    m[6] = _mm256_srai_epi32::<12>(m[6]);
    m[0] = _mm256_srai_epi32::<12>(m[0]);
    m[7] = _mm256_srai_epi32::<12>(m[7]);
    m[5] = _mm256_srai_epi32::<12>(m[5]);
    m[1] = _mm256_srai_epi32::<12>(m[1]);
    m[2] = _mm256_srai_epi32::<12>(m[2]);
    m[3] = _mm256_srai_epi32::<12>(m[3]);
    m[0] = _mm256_packs_epi32(m[0], m[7]); // packssdw m0, m7
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[3] = _mm256_packs_epi32(m[3], m[2]); // packssdw m3, m2
    m[2] = _mm256_packs_epi32(m[10], m[6]); // packssdw m2, m10, m6
}

/// `cglobal iflipadst_16x4_internal_8bpc` up to `jmp tx2q` (which it reaches through `iadst_16x4_internal_8bpc`'s
/// `.pass1_end`)
#[inline(always)]
unsafe fn iflipadst_16x4_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0x6c>(_mm256_loadu_si256(c.byte_add(32 * 0).cast())); // vpermq m0, [cq+32*0], q1230
    m[3] = _mm256_permute4x64_epi64::<0x93>(_mm256_loadu_si256(c.byte_add(32 * 3).cast())); // vpermq m3, [cq+32*3], q2103
    m[1] = _mm256_permute4x64_epi64::<0x6c>(_mm256_loadu_si256(c.byte_add(32 * 1).cast())); // vpermq m1, [cq+32*1], q1230
    m[2] = _mm256_permute4x64_epi64::<0x93>(_mm256_loadu_si256(c.byte_add(32 * 2).cast())); // vpermq m2, [cq+32*2], q2103
    iadst_4x16_internal_main2(m); // call m(iadst_4x16_internal_8bpc).main2
    iadst_4x16_internal_main_pass1_end(m); // call m(iadst_4x16_internal_8bpc).main_pass1_end
    m[4] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m4, m3, m2
    m[5] = _mm256_unpackhi_epi16(m[1], m[0]); // punpckhwd m5, m1, m0
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    m[1] = _mm256_unpacklo_epi16(m[1], m[3]); // punpcklwd m1, m3
    m[6] = pw(16384); // vpbroadcastd m6, [o(pw_16384)]
    m[3] = _mm256_inserti128_si256::<1>(m[0], lo(m[1])); // vinserti128 m3, m0, xm1, 1
    m[2] = _mm256_permute2x128_si256::<0x31>(m[0], m[1]); // vperm2i128 m2, m0, m1, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[5])); // vinserti128 m0, m4, xm5, 1
    m[4] = _mm256_permute2x128_si256::<0x31>(m[4], m[5]); // vperm2i128 m4, m4, m5, 0x31
    m[1] = _mm256_sub_epi16(m[7], m[6]); // psubw m1, m7, m6
    iadst_16x4_internal_pass1_end(m) // jmp m(iadst_16x4_internal_8bpc).pass1_end
}

/// `.pass2` of `iflipadst_16x4_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_16x4_internal_pass2(m: &mut M, mut dst: Dst, stride: isize, c: Coef) {
    iadst_16x4_internal_main(m); // call m(iadst_16x4_internal_8bpc).main
    m[4] = pw(2048); // vpbroadcastd m4, [o(pw_2048)]
    m[3] = _mm256_mulhrs_epi16(m[3], m[4]); // REPX {pmulhrsw x, m4}, m3, m2, m1, m0
    m[2] = _mm256_mulhrs_epi16(m[2], m[4]);
    m[1] = _mm256_mulhrs_epi16(m[1], m[4]);
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]);
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.byte_add(32 * 0).cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(c.byte_add(32 * 1).cast(), m[4]); // mova [cq+32*1], m4
    _mm256_storeu_si256(c.byte_add(32 * 2).cast(), m[4]); // mova [cq+32*2], m4
    _mm256_storeu_si256(c.byte_add(32 * 3).cast(), m[4]); // mova [cq+32*3], m4
    write_16x2(m, dst, m[3], m[2], 4, 5, [0, stride]); // WRITE_16X2 3, 2, 4, 5, strideq*0, strideq*1
    dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
    write_16x2(m, dst, m[1], m[0], 4, 5, [0, stride]); // WRITE_16X2 1, 0, 4, 5, strideq*0, strideq*1
}

/// `cglobal iidentity_16x4_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_16x4_internal_pass1(m: &mut M, c: Coef) {
    m[2] = zx(_mm_loadu_si128(c.byte_add(16 * 0).cast())); // mova xm2, [cq+16*0]
    m[4] = zx(_mm_loadu_si128(c.byte_add(16 * 1).cast())); // mova xm4, [cq+16*1]
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(c.byte_add(16 * 4).cast())); // vinserti128 m2, [cq+16*4], 1
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(c.byte_add(16 * 5).cast())); // vinserti128 m4, [cq+16*5], 1
    m[0] = zx(_mm_loadu_si128(c.byte_add(16 * 2).cast())); // mova xm0, [cq+16*2]
    m[1] = zx(_mm_loadu_si128(c.byte_add(16 * 3).cast())); // mova xm1, [cq+16*3]
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(c.byte_add(16 * 6).cast())); // vinserti128 m0, [cq+16*6], 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(c.byte_add(16 * 7).cast())); // vinserti128 m1, [cq+16*7], 1
    m[7] = pw(1697 * 16); // vpbroadcastd m7, [o(pw_1697x16)]
    m[8] = pw(16384); // vpbroadcastd m8, [o(pw_16384)]
    m[3] = _mm256_unpacklo_epi16(m[2], m[4]); // punpcklwd m3, m2, m4
    m[2] = _mm256_unpackhi_epi16(m[2], m[4]); // punpckhwd m2, m4
    m[4] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m4, m0, m1
    m[0] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m0, m1
    m[1] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m1, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[2] = _mm256_unpacklo_epi16(m[4], m[0]); // punpcklwd m2, m4, m0
    m[4] = _mm256_unpackhi_epi16(m[4], m[0]); // punpckhwd m4, m0
    m[0] = _mm256_mulhrs_epi16(m[7], m[1]); // pmulhrsw m0, m7, m1
    m[5] = _mm256_mulhrs_epi16(m[7], m[2]); // pmulhrsw m5, m7, m2
    m[6] = _mm256_mulhrs_epi16(m[7], m[3]); // pmulhrsw m6, m7, m3
    m[7] = _mm256_mulhrs_epi16(m[7], m[4]); // pmulhrsw m7, m4
    m[0] = _mm256_mulhrs_epi16(m[0], m[8]); // REPX {pmulhrsw x, m8}, m0, m5, m6, m7
    m[5] = _mm256_mulhrs_epi16(m[5], m[8]);
    m[6] = _mm256_mulhrs_epi16(m[6], m[8]);
    m[7] = _mm256_mulhrs_epi16(m[7], m[8]);
    m[1] = _mm256_adds_epi16(m[1], m[0]); // paddsw m1, m0
    m[2] = _mm256_adds_epi16(m[2], m[5]); // paddsw m2, m5
    m[3] = _mm256_adds_epi16(m[3], m[6]); // paddsw m3, m6
    m[4] = _mm256_adds_epi16(m[4], m[7]); // paddsw m4, m7
    m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
    m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
}

/// `.pass2` of `iidentity_16x4_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_16x4_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[7] = pw(1697 * 8); // vpbroadcastd m7, [o(pw_1697x8)]
    m[4] = _mm256_mulhrs_epi16(m[7], m[0]); // pmulhrsw m4, m7, m0
    m[5] = _mm256_mulhrs_epi16(m[7], m[1]); // pmulhrsw m5, m7, m1
    m[6] = _mm256_mulhrs_epi16(m[7], m[2]); // pmulhrsw m6, m7, m2
    m[7] = _mm256_mulhrs_epi16(m[7], m[3]); // pmulhrsw m7, m3
    m[0] = _mm256_adds_epi16(m[0], m[4]); // paddsw m0, m4
    m[1] = _mm256_adds_epi16(m[1], m[5]); // paddsw m1, m5
    m[2] = _mm256_adds_epi16(m[2], m[6]); // paddsw m2, m6
    m[3] = _mm256_adds_epi16(m[3], m[7]); // paddsw m3, m7
    iadst_16x4_internal_end(m, dst, stride, c) // jmp m(iadst_16x4_internal_8bpc).end
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x4_8bpc_avx2,
    inv_txfm_add_16x4,
    K::Identity,
    K::Identity
);

// --- part 2: itx_avx2.asm lines 1332-1869 and 2148-2459 (8x8, 8x16, 16x8) --------------------

/// `vbroadcasti128 m, [o(table)]`
#[inline(always)]
unsafe fn vbroadcasti128_p2(table: &[u8; 16]) -> __m256i {
    _mm256_broadcastsi128_si256(_mm_loadu_si128(table.as_ptr().cast()))
}

/// `shufps dst, a, b, imm` on integer registers.
#[inline(always)]
unsafe fn shufps_p2<const IMM: i32>(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castps_si256(_mm256_shuffle_ps::<IMM>(_mm256_castsi256_ps(a), _mm256_castsi256_ps(b)))
}

// The 1-D macros of part 1's range at `mmsize` 32 (part 1 has the `mmsize` 16 forms, suffix `_x`).

/// `%macro IDCT8_1D_PACKED 0` at `mmsize` 32
#[inline(always)]
unsafe fn idct8_1d_packed(m: &mut M) {
    m[6] = pd(2048); // vpbroadcastd m6, [o(pd_2048)]
    m[5] = _mm256_unpackhi_epi16(m[3], m[0]); // punpckhwd m5, m3, m0 ; in7 in1
    m[4] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m4, m1, m2 ; in3 in5
    m[3] = _mm256_unpacklo_epi16(m[3], m[1]); // punpcklwd m3, m1 ; in6 in2
    m[2] = _mm256_unpacklo_epi16(m[2], m[0]); // punpcklwd m2, m0 ; in4 in0
    itx_mul2x_pack(m, 5, 0, 1, 6, 799, 4017, 3); // ITX_MUL2X_PACK 5, 0, 1, 6, 799, 4017, 3 ; t4a t7a
    itx_mul2x_pack(m, 4, 0, 1, 6, 3406, 2276, 3); // ITX_MUL2X_PACK 4, 0, 1, 6, 3406, 2276, 3 ; t5a t6a
    itx_mul2x_pack(m, 3, 0, 1, 6, 1567, 3784, 0); // ITX_MUL2X_PACK 3, 0, 1, 6, 1567, 3784 ; t3 t2
    m[0] = _mm256_subs_epi16(m[5], m[4]); // psubsw m0, m5, m4 ; t5a t6a (interleaved)
    m[4] = _mm256_adds_epi16(m[4], m[5]); // paddsw m4, m5 ; t4 t7 (interleaved)
    itx_mul2x_pack(m, 2, 1, 5, 6, 2896, 2896, 0); // ITX_MUL2X_PACK 2, 1, 5, 6, 2896, 2896 ; t0 t1
    m[1] = pw2(-2896, 2896); // vpbroadcastd m1, [o(pw_m2896_2896)]
    itx_mul2x_pack(m, 0, 1, usize::MAX, 6, 1, 5, 4); // ITX_MUL2X_PACK 0, 1, _, 6, 1, 5, 4 ; t6 t5
    m[1] = vbroadcasti128_p2(&deint_shuf); // vbroadcasti128 m1, [o(deint_shuf)]
    m[4] = _mm256_shuffle_epi8(m[4], m[1]); // pshufb m4, m1
    m[1] = _mm256_subs_epi16(m[2], m[3]); // psubsw m1, m2, m3 ; tmp3 tmp2
    m[3] = _mm256_adds_epi16(m[3], m[2]); // paddsw m3, m2 ; tmp0 tmp1
    m[2] = shufps_p2::<0x4e>(m[4], m[0]); // shufps m2, m4, m0, q1032 ; t7 t6
    m[4] = _mm256_blend_epi32::<0xcc>(m[4], m[0]); // vpblendd m4, m0, 0xcc ; t4 t5
    m[0] = _mm256_adds_epi16(m[3], m[2]); // paddsw m0, m3, m2 ; out0 out1
    m[3] = _mm256_subs_epi16(m[3], m[2]); // psubsw m3, m2 ; out7 out6
    m[2] = _mm256_subs_epi16(m[1], m[4]); // psubsw m2, m1, m4 ; out4 out5
    m[1] = _mm256_adds_epi16(m[1], m[4]); // paddsw m1, m4 ; out3 out2
}

/// `%macro IADST8_1D_PACKED 1 ; pass` at `mmsize` 32
#[inline(always)]
unsafe fn iadst8_1d_packed(m: &mut M, pass: u32) {
    m[6] = pd(2048); // vpbroadcastd m6, [o(pd_2048)]
    m[0] = _mm256_unpackhi_epi16(m[4], m[3]); // punpckhwd m0, m4, m3 ; 0 7
    m[1] = _mm256_unpackhi_epi16(m[5], m[2]); // punpckhwd m1, m5, m2 ; 2 5
    m[2] = _mm256_unpacklo_epi16(m[2], m[5]); // punpcklwd m2, m5 ; 4 3
    m[3] = _mm256_unpacklo_epi16(m[3], m[4]); // punpcklwd m3, m4 ; 6 1
    if pass == 1 {
        itx_mul2x_pack(m, 0, 4, 5, 6, 401, 4076, 3); // ITX_MUL2X_PACK 0, 4, 5, 6, 401, 4076, 3 ; t1a t0a
        itx_mul2x_pack(m, 1, 4, 5, 6, 1931, 3612, 2); // ITX_MUL2X_PACK 1, 4, 5, 6, 1931, 3612, 2 ; t2a t3a
        itx_mul2x_pack(m, 2, 4, 5, 6, 3166, 2598, 3); // ITX_MUL2X_PACK 2, 4, 5, 6, 3166, 2598, 3 ; t5a t4a
        itx_mul2x_pack(m, 3, 4, 5, 6, 3920, 1189, 2); // ITX_MUL2X_PACK 3, 4, 5, 6, 3920, 1189, 2 ; t6a t7a
        m[4] = _mm256_subs_epi16(m[0], m[2]); // psubsw m4, m0, m2 ; t5 t4
        m[0] = _mm256_adds_epi16(m[0], m[2]); // paddsw m0, m2 ; t1 t0
        m[5] = _mm256_subs_epi16(m[1], m[3]); // psubsw m5, m1, m3 ; t6 t7
        m[1] = _mm256_adds_epi16(m[1], m[3]); // paddsw m1, m3 ; t2 t3
        itx_mul2x_pack(m, 4, 2, 3, 6, 1567, 3784, 3); // ITX_MUL2X_PACK 4, 2, 3, 6, 1567, 3784, 3 ; t5a t4a
        itx_mul2x_pack(m, 5, 2, 3, 6, 3784, 1567, 2); // ITX_MUL2X_PACK 5, 2, 3, 6, 3784, 1567, 2 ; t7a t6a
        m[2] = vbroadcasti128_p2(&deint_shuf); // vbroadcasti128 m2, [o(deint_shuf)]
        m[1] = _mm256_shufflelo_epi16::<0xb1>(m[1]); // pshuflw m1, m1, q2301
        m[1] = _mm256_shufflehi_epi16::<0xb1>(m[1]); // pshufhw m1, m1, q2301
        m[3] = _mm256_subs_epi16(m[0], m[1]); // psubsw m3, m0, m1 ; t3 t2
        m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; -out7 out0
        m[1] = _mm256_subs_epi16(m[4], m[5]); // psubsw m1, m4, m5 ; t7 t6
        m[4] = _mm256_adds_epi16(m[4], m[5]); // paddsw m4, m5 ; out6 -out1
        m[0] = _mm256_shuffle_epi8(m[0], m[2]); // pshufb m0, m2
        m[4] = _mm256_shuffle_epi8(m[4], m[2]); // pshufb m4, m2
        m[5] = pw2(-2896, 2896); // vpbroadcastd m5, [o(pw_m2896_2896)]
        m[2] = _mm256_madd_epi16(m[5], m[3]); // pmaddwd m2, m5, m3
        m[5] = _mm256_madd_epi16(m[5], m[1]); // pmaddwd m5, m1
        m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6
        m[5] = _mm256_add_epi32(m[5], m[6]); // paddd m5, m6
        m[2] = _mm256_srai_epi32::<12>(m[2]); // psrad m2, 12
        m[5] = _mm256_srai_epi32::<12>(m[5]); // psrad m5, 12
        m[2] = _mm256_packs_epi32(m[2], m[5]); // packssdw m2, m5 ; out4 -out5
        m[5] = pw2(2896, 2896); // vpbroadcastd m5, [o(pw_2896_2896)]
        m[3] = _mm256_madd_epi16(m[3], m[5]); // pmaddwd m3, m5
        m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
        m[3] = _mm256_add_epi32(m[3], m[6]); // paddd m3, m6
        m[1] = _mm256_add_epi32(m[1], m[6]); // paddd m1, m6
        m[3] = _mm256_srai_epi32::<12>(m[3]); // psrad m3, 12
        m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
        m[1] = _mm256_packs_epi32(m[1], m[3]); // packssdw m1, m3 ; out2 -out3
        m[3] = _mm256_unpacklo_epi64(m[4], m[0]); // punpcklqdq m3, m4, m0 ; out6 -out7
        m[0] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m0, m4 ; out0 -out1
    } else {
        itx_mul2x_pack(m, 0, 4, 5, 6, 401, 4076, 0); // ITX_MUL2X_PACK 0, 4, 5, 6, 401, 4076 ; t0a t1a
        itx_mul2x_pack(m, 1, 4, 5, 6, 1931, 3612, 0); // ITX_MUL2X_PACK 1, 4, 5, 6, 1931, 3612 ; t2a t3a
        itx_mul2x_pack(m, 2, 4, 5, 6, 3166, 2598, 0); // ITX_MUL2X_PACK 2, 4, 5, 6, 3166, 2598 ; t4a t5a
        itx_mul2x_pack(m, 3, 4, 5, 6, 3920, 1189, 0); // ITX_MUL2X_PACK 3, 4, 5, 6, 3920, 1189 ; t6a t7a
        m[4] = _mm256_subs_epi16(m[0], m[2]); // psubsw m4, m0, m2 ; t4 t5
        m[0] = _mm256_adds_epi16(m[0], m[2]); // paddsw m0, m2 ; t0 t1
        m[5] = _mm256_subs_epi16(m[1], m[3]); // psubsw m5, m1, m3 ; t6 t7
        m[1] = _mm256_adds_epi16(m[1], m[3]); // paddsw m1, m3 ; t2 t3
        m[2] = shufps_p2::<0x4e>(m[5], m[4]); // shufps m2, m5, m4, q1032
        m[4] = _mm256_unpackhi_epi16(m[4], m[2]); // punpckhwd m4, m2
        m[5] = _mm256_unpacklo_epi16(m[5], m[2]); // punpcklwd m5, m2
        itx_mul2x_pack(m, 4, 2, 3, 6, 1567, 3784, 1); // ITX_MUL2X_PACK 4, 2, 3, 6, 1567, 3784, 1 ; t5a t4a
        itx_mul2x_pack(m, 5, 2, 3, 6, 3784, 1567, 0); // ITX_MUL2X_PACK 5, 2, 3, 6, 3784, 1567 ; t7a t6a
        m[2] = _mm256_subs_epi16(m[0], m[1]); // psubsw m2, m0, m1 ; t2 t3
        m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; out0 -out7
        m[1] = _mm256_subs_epi16(m[4], m[5]); // psubsw m1, m4, m5 ; t7 t6
        m[4] = _mm256_adds_epi16(m[4], m[5]); // paddsw m4, m5 ; out6 -out1
        m[5] = pw(2896 * 8); // vpbroadcastd m5, [o(pw_2896x8)]
        m[3] = _mm256_blend_epi32::<0x33>(m[0], m[4]); // vpblendd m3, m0, m4, 0x33 ; out6 -out7
        m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[4]); // vpblendd m0, m4, 0xcc ; out0 -out1
        m[4] = shufps_p2::<0x4e>(m[2], m[1]); // shufps m4, m2, m1, q1032 ; t3 t7
        m[1] = _mm256_blend_epi32::<0x33>(m[1], m[2]); // vpblendd m1, m2, 0x33 ; t2 t6
        m[2] = _mm256_subs_epi16(m[1], m[4]); // psubsw m2, m1, m4 ; t2-t3 t6-t7
        m[1] = _mm256_adds_epi16(m[1], m[4]); // paddsw m1, m4 ; t2+t3 t6+t7
        m[2] = _mm256_mulhrs_epi16(m[2], m[5]); // pmulhrsw m2, m5 ; out4 -out5
        m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
        m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5 ; out2 -out3
    }
}

/// `%macro IDCT16_1D_PACKED 0` at `mmsize` 32 (the macro's label `.main2`, after the first instruction, has no
/// caller in the 8 bpc file)
#[inline(always)]
unsafe fn idct16_1d_packed(m: &mut M) {
    m[10] = pd(2048); // vpbroadcastd m10, [o(pd_2048)]
                      // .main2:
    m[8] = _mm256_unpackhi_epi16(m[7], m[0]); // punpckhwd m8, m7, m0 ; dct16 in15 in1
    m[9] = _mm256_unpacklo_epi16(m[4], m[0]); // punpcklwd m9, m4, m0 ; dct4 in2 in0
    m[0] = _mm256_unpackhi_epi16(m[3], m[4]); // punpckhwd m0, m3, m4 ; dct16 in7 in9
    m[7] = _mm256_unpacklo_epi16(m[7], m[1]); // punpcklwd m7, m1 ; dct8 in7 in1
    m[1] = _mm256_unpackhi_epi16(m[1], m[6]); // punpckhwd m1, m6 ; dct16 in3 in13
    m[3] = _mm256_unpacklo_epi16(m[3], m[5]); // punpcklwd m3, m5 ; dct8 in3 in5
    m[5] = _mm256_unpackhi_epi16(m[5], m[2]); // punpckhwd m5, m2 ; dct16 in11 in5
    m[6] = _mm256_unpacklo_epi16(m[6], m[2]); // punpcklwd m6, m2 ; dct4 in3 in1
    itx_mul2x_pack(m, 8, 2, 4, 10, 401, 4076, 3); // ITX_MUL2X_PACK 8, 2, 4, 10, 401, 4076, 3 ; t8a t15a
    itx_mul2x_pack(m, 0, 2, 4, 10, 3166, 2598, 3); // ITX_MUL2X_PACK 0, 2, 4, 10, 3166, 2598, 3 ; t9a t14a
    itx_mul2x_pack(m, 1, 2, 4, 10, 3920, 1189, 3); // ITX_MUL2X_PACK 1, 2, 4, 10, 3920, 1189, 3 ; t11a t12a
    itx_mul2x_pack(m, 5, 2, 4, 10, 1931, 3612, 3); // ITX_MUL2X_PACK 5, 2, 4, 10, 1931, 3612, 3 ; t10a t13a
    itx_mul2x_pack(m, 7, 2, 4, 10, 799, 4017, 3); // ITX_MUL2X_PACK 7, 2, 4, 10, 799, 4017, 3 ; t4a t7a
    itx_mul2x_pack(m, 3, 2, 4, 10, 3406, 2276, 3); // ITX_MUL2X_PACK 3, 2, 4, 10, 3406, 2276, 3 ; t5a t6a
    itx_mul2x_pack(m, 6, 2, 4, 10, 1567, 3784, 0); // ITX_MUL2X_PACK 6, 2, 4, 10, 1567, 3784 ; t3 t2
    m[2] = _mm256_subs_epi16(m[8], m[0]); // psubsw m2, m8, m0 ; t9 t14
    m[8] = _mm256_adds_epi16(m[8], m[0]); // paddsw m8, m0 ; t8 t15
    m[0] = _mm256_subs_epi16(m[1], m[5]); // psubsw m0, m1, m5 ; t10 t13
    m[1] = _mm256_adds_epi16(m[1], m[5]); // paddsw m1, m5 ; t11 t12
    m[5] = pw2(-3784, 1567); // vpbroadcastd m5, [o(pw_m3784_1567)] ; reuse pw_1567_3784
    itx_mul2x_pack(m, 2, 4, usize::MAX, 10, 4, 5, 6); // ITX_MUL2X_PACK 2, 4, _, 10, 4, 5, 6 ; t9a t14a
    m[4] = pw2(-1567, -3784); // vpbroadcastd m4, [o(pw_m1567_m3784)] ; reuse pw_m3784_1567
    itx_mul2x_pack(m, 0, 5, usize::MAX, 10, 5, 4, 6); // ITX_MUL2X_PACK 0, 5, _, 10, 5, 4, 6 ; t10a t13a
    m[4] = _mm256_subs_epi16(m[8], m[1]); // psubsw m4, m8, m1 ; t11a t12a
    m[8] = _mm256_adds_epi16(m[8], m[1]); // paddsw m8, m1 ; t8a t15a
    m[1] = _mm256_subs_epi16(m[7], m[3]); // psubsw m1, m7, m3 ; t5a t6a
    m[7] = _mm256_adds_epi16(m[7], m[3]); // paddsw m7, m3 ; t4 t7
    m[3] = _mm256_adds_epi16(m[2], m[0]); // paddsw m3, m2, m0 ; t9 t14
    m[2] = _mm256_subs_epi16(m[2], m[0]); // psubsw m2, m0 ; t10 t13
    m[0] = vbroadcasti128_p2(&deint_shuf); // vbroadcasti128 m0, [o(deint_shuf)]
    m[8] = _mm256_shuffle_epi8(m[8], m[0]); // pshufb m8, m0
    m[7] = _mm256_shuffle_epi8(m[7], m[0]); // pshufb m7, m0
    m[3] = _mm256_shuffle_epi8(m[3], m[0]); // pshufb m3, m0
    itx_mul2x_pack(m, 9, 0, 5, 10, 2896, 2896, 0); // ITX_MUL2X_PACK 9, 0, 5, 10, 2896, 2896 ; t0 t1
    m[0] = pw2(-2896, 2896); // vpbroadcastd m0, [o(pw_m2896_2896)]
    itx_mul2x_pack(m, 4, 5, usize::MAX, 10, 5, 0, 4); // ITX_MUL2X_PACK 4, 5, _, 10, 5, 0, 4 ; t11 t12
    m[5] = pw2(2896, 2896); // vpbroadcastd m5, [o(pw_2896_2896)]
    itx_mul2x_pack(m, 1, 0, usize::MAX, 10, 0, 5, 4); // ITX_MUL2X_PACK 1, 0, _, 10, 0, 5, 4 ; t6 t5
    m[0] = pw2(-2896, 2896); // vpbroadcastd m0, [o(pw_m2896_2896)]
    itx_mul2x_pack(m, 2, 0, usize::MAX, 10, 0, 5, 4); // ITX_MUL2X_PACK 2, 0, _, 10, 0, 5, 4 ; t13a t10a
    m[0] = _mm256_unpackhi_epi64(m[8], m[3]); // punpckhqdq m0, m8, m3 ; t15a t14
    m[8] = _mm256_unpacklo_epi64(m[8], m[3]); // punpcklqdq m8, m3 ; t8a t9
    m[5] = shufps_p2::<0x4e>(m[4], m[2]); // shufps m5, m4, m2, q1032 ; t12 t13a
    m[4] = _mm256_blend_epi32::<0xcc>(m[4], m[2]); // vpblendd m4, m2, 0xcc ; t11 t10a
    m[2] = shufps_p2::<0x4e>(m[7], m[1]); // shufps m2, m7, m1, q1032 ; t7 t6
    m[7] = _mm256_blend_epi32::<0xcc>(m[7], m[1]); // vpblendd m7, m1, 0xcc ; t4 t5
    m[1] = _mm256_subs_epi16(m[9], m[6]); // psubsw m1, m9, m6 ; dct4 out3 out2
    m[9] = _mm256_adds_epi16(m[9], m[6]); // paddsw m9, m6 ; dct4 out0 out1
    m[3] = _mm256_subs_epi16(m[9], m[2]); // psubsw m3, m9, m2 ; dct8 out7 out6
    m[9] = _mm256_adds_epi16(m[9], m[2]); // paddsw m9, m2 ; dct8 out0 out1
    m[2] = _mm256_subs_epi16(m[1], m[7]); // psubsw m2, m1, m7 ; dct8 out4 out5
    m[1] = _mm256_adds_epi16(m[1], m[7]); // paddsw m1, m7 ; dct8 out3 out2
    m[7] = _mm256_subs_epi16(m[9], m[0]); // psubsw m7, m9, m0 ; out15 out14
    m[0] = _mm256_adds_epi16(m[0], m[9]); // paddsw m0, m9 ; out0 out1
    m[6] = _mm256_subs_epi16(m[1], m[5]); // psubsw m6, m1, m5 ; out12 out13
    m[1] = _mm256_adds_epi16(m[1], m[5]); // paddsw m1, m5 ; out3 out2
    m[5] = _mm256_subs_epi16(m[2], m[4]); // psubsw m5, m2, m4 ; out11 out10
    m[2] = _mm256_adds_epi16(m[2], m[4]); // paddsw m2, m4 ; out4 out5
    m[4] = _mm256_subs_epi16(m[3], m[8]); // psubsw m4, m3, m8 ; out8 out9
    m[3] = _mm256_adds_epi16(m[3], m[8]); // paddsw m3, m8 ; out7 out6
}

// 8x8

/// `%macro INV_TXFM_8X8_FN 2 ; type1, type2`: the DC-only path of dct_dct, then the two passes.
#[inline(always)]
unsafe fn inv_txfm_add_8x8(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 8; // or r3d, 8
        return inv_txfm_add_dct_dct_8x8_dconly(m, dst, stride, r3d); // .dconly:
    }
    match t1 {
        K::Dct => idct_8x8_internal_pass1(m, c),
        K::Adst => iadst_8x8_internal_pass1(m, c),
        K::FlipAdst => iflipadst_8x8_internal_pass1(m, c),
        K::Identity => iidentity_8x8_internal_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x8_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_8x8_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x8_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x8_internal_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_8x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_8x8_internal_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 0).cast())); // vpermq m0, [cq+32*0], q3120 ; 0 1
    m[3] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 3).cast())); // vpermq m3, [cq+32*3], q3120 ; 6 7
    m[2] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 2).cast())); // vpermq m2, [cq+32*2], q3120 ; 4 5
    m[1] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 1).cast())); // vpermq m1, [cq+32*1], q3120 ; 2 3
    idct_8x8_internal_main(m); // call .main
    m[4] = shufps_p2::<0x28>(m[0], m[1]); // shufps m4, m0, m1, q0220
    m[5] = shufps_p2::<0x7d>(m[0], m[1]); // shufps m5, m0, m1, q1331
    m[1] = shufps_p2::<0x28>(m[2], m[3]); // shufps m1, m2, m3, q0220
    m[3] = shufps_p2::<0x7d>(m[2], m[3]); // shufps m3, m2, m3, q1331
    m[0] = vbroadcasti128_p2(&deint_shuf); // vbroadcasti128 m0, [o(deint_shuf)]
    m[2] = pw(16384); // vpbroadcastd m2, [o(pw_16384)]
    for i in [4, 5, 1, 3] {
        m[i] = _mm256_shuffle_epi8(m[i], m[0]); // REPX {pshufb x, m0}, m4, m5, m1, m3
    }
    for i in [4, 5, 1, 3] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[2]); // REPX {pmulhrsw x, m2}, m4, m5, m1, m3
    }
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[1])); // vinserti128 m0, m4, xm1, 1
    m[2] = _mm256_permute2x128_si256::<0x31>(m[4], m[1]); // vperm2i128 m2, m4, m1, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[5], lo(m[3])); // vinserti128 m1, m5, xm3, 1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[5], m[3]); // vperm2i128 m3, m5, m3, 0x31
}

/// `.pass2` of `idct_8x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_8x8_internal_main(m); // call .main
    m[4] = pw(2048); // vpbroadcastd m4, [o(pw_2048)]
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m3, m3, q2031
    iadst_8x8_internal_end2(m, dst, stride, c) // jmp m(iadst_8x8_internal_8bpc).end2
}

/// `.main` of `idct_8x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn idct_8x8_internal_main(m: &mut M) {
    idct8_1d_packed(m) // IDCT8_1D_PACKED
}

/// `cglobal iadst_8x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_8x8_internal_pass1(m: &mut M, c: Coef) {
    m[4] = _mm256_permute4x64_epi64::<0x72>(_mm256_loadu_si256(at(c, 32 * 0).cast())); // vpermq m4, [cq+32*0], q1302 ; 1 0
    m[3] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 3).cast())); // vpermq m3, [cq+32*3], q3120 ; 6 7
    m[5] = _mm256_permute4x64_epi64::<0x72>(_mm256_loadu_si256(at(c, 32 * 1).cast())); // vpermq m5, [cq+32*1], q1302 ; 3 2
    m[2] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 2).cast())); // vpermq m2, [cq+32*2], q3120 ; 4 5
    iadst_8x8_internal_main_pass1(m); // call .main_pass1
    m[5] = pw(16384); // vpbroadcastd m5, [o(pw_16384)]
    m[4] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m4, m0, m1
    m[0] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m0, m1
    m[1] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m1, m2, m3
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[3] = _mm256_setzero_si256(); // pxor m3, m3
    m[3] = _mm256_sub_epi16(m[3], m[5]); // psubw m3, m5 ; negate odd elements during rounding
    m[4] = _mm256_mulhrs_epi16(m[4], m[5]); // pmulhrsw m4, m5
    m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5
    m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
    m[3] = _mm256_unpacklo_epi16(m[4], m[0]); // punpcklwd m3, m4, m0
    m[4] = _mm256_unpackhi_epi16(m[4], m[0]); // punpckhwd m4, m0
    m[0] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m0, m1, m2
    m[1] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m1, m2
    m[2] = _mm256_permute2x128_si256::<0x31>(m[3], m[0]); // vperm2i128 m2, m3, m0, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[3], lo(m[0])); // vinserti128 m0, m3, xm0, 1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[4], m[1]); // vperm2i128 m3, m4, m1, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[4], lo(m[1])); // vinserti128 m1, m4, xm1, 1
}

/// `.pass2` of `iadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = _mm256_shuffle_epi32::<0x4e>(m[0]); // pshufd m4, m0, q1032
    m[5] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m5, m1, q1032
    iadst_8x8_internal_main_pass2(m); // call .main_pass2
    m[5] = pw(2048); // vpbroadcastd m5, [o(pw_2048)]
    m[4] = zx(lo(pw(4096))); // vpbroadcastd xm4, [o(pw_4096)]
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5 ; lower half = 2048, upper half = -2048
    iadst_8x8_internal_end(m, dst, stride, c)
}

/// `.end` of `iadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    for i in [0, 1, 2, 3] {
        m[i] = _mm256_permute4x64_epi64::<0xd8>(m[i]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    }
    iadst_8x8_internal_end2(m, dst, stride, c)
}

/// `.end2` of `iadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
    iadst_8x8_internal_end3(m, dst, stride, c)
}

/// `.end3` of `iadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_end3(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = _mm256_mulhrs_epi16(m[2], m[4]); // pmulhrsw m2, m4
    m[3] = _mm256_mulhrs_epi16(m[3], m[4]); // pmulhrsw m3, m4
    iadst_8x8_internal_end4(m, dst, stride, c) // WIN64_RESTORE_XMM
}

/// `.end4` of `iadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_end4(m: &mut M, mut dst: Dst, stride: isize, c: Coef) {
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(at(c, 32 * 0).cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[4]); // mova [cq+32*1], m4
    _mm256_storeu_si256(at(c, 32 * 2).cast(), m[4]); // mova [cq+32*2], m4
    _mm256_storeu_si256(at(c, 32 * 3).cast(), m[4]); // mova [cq+32*3], m4
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_8x4(m, dst, m[0], m[1], 4, 5, [stride, stride * 2, r3]); // WRITE_8X4 0, 1, 4, 5
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[2], m[3], 4, 5, [stride, stride * 2, r3]); // WRITE_8X4 2, 3, 4, 5
}

/// `.main_pass1` of `iadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_main_pass1(m: &mut M) {
    iadst8_1d_packed(m, 1) // IADST8_1D_PACKED 1
}

/// `.main_pass2` of `iadst_8x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_8x8_internal_main_pass2(m: &mut M) {
    iadst8_1d_packed(m, 2) // IADST8_1D_PACKED 2
}

/// `cglobal iflipadst_8x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_8x8_internal_pass1(m: &mut M, c: Coef) {
    m[4] = _mm256_permute4x64_epi64::<0x72>(_mm256_loadu_si256(at(c, 32 * 0).cast())); // vpermq m4, [cq+32*0], q1302 ; 1 0
    m[3] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 3).cast())); // vpermq m3, [cq+32*3], q3120 ; 6 7
    m[5] = _mm256_permute4x64_epi64::<0x72>(_mm256_loadu_si256(at(c, 32 * 1).cast())); // vpermq m5, [cq+32*1], q1302 ; 3 2
    m[2] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 2).cast())); // vpermq m2, [cq+32*2], q3120 ; 4 5
    iadst_8x8_internal_main_pass1(m); // call m(iadst_8x8_internal_8bpc).main_pass1
    m[5] = pw(16384); // vpbroadcastd m5, [o(pw_16384)]
    m[4] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m4, m3, m2
    m[3] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m3, m2
    m[2] = _mm256_unpackhi_epi16(m[1], m[0]); // punpckhwd m2, m1, m0
    m[1] = _mm256_unpacklo_epi16(m[1], m[0]); // punpcklwd m1, m0
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    m[0] = _mm256_sub_epi16(m[0], m[5]); // psubw m0, m5
    m[4] = _mm256_mulhrs_epi16(m[4], m[0]); // pmulhrsw m4, m0
    m[3] = _mm256_mulhrs_epi16(m[3], m[5]); // pmulhrsw m3, m5
    m[2] = _mm256_mulhrs_epi16(m[2], m[0]); // pmulhrsw m2, m0
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5
    m[0] = _mm256_unpackhi_epi16(m[4], m[3]); // punpckhwd m0, m4, m3
    m[4] = _mm256_unpacklo_epi16(m[4], m[3]); // punpcklwd m4, m3
    m[3] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m3, m2, m1
    m[2] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m2, m1
    m[1] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m1, m0, xm3, 1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m3, m0, m3, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[2])); // vinserti128 m0, m4, xm2, 1
    m[2] = _mm256_permute2x128_si256::<0x31>(m[4], m[2]); // vperm2i128 m2, m4, m2, 0x31
}

/// `.pass2` of `iflipadst_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_8x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = _mm256_shuffle_epi32::<0x4e>(m[0]); // pshufd m4, m0, q1032
    m[5] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m5, m1, q1032
    iadst_8x8_internal_main_pass2(m); // call m(iadst_8x8_internal_8bpc).main_pass2
    m[4] = pw(2048); // vpbroadcastd m4, [o(pw_2048)]
    m[5] = zx(lo(pw(4096))); // vpbroadcastd xm5, [o(pw_4096)]
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5 ; lower half = -2048, upper half = 2048
    m[5] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m5, m3, q2031
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[0]); // vpermq m3, m0, q2031
    m[0] = _mm256_permute4x64_epi64::<0x8d>(m[2]); // vpermq m0, m2, q2031
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m2, m1, q2031
    m[1] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m1, m0, m4
    m[0] = _mm256_mulhrs_epi16(m[5], m[4]); // pmulhrsw m0, m5, m4
    iadst_8x8_internal_end3(m, dst, stride, c) // jmp m(iadst_8x8_internal_8bpc).end3
}

/// `cglobal iidentity_8x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_8x8_internal_pass1(m: &mut M, c: Coef) {
    m[3] = zx(_mm_loadu_si128(at(c, 16 * 0).cast())); // mova xm3, [cq+16*0]
    m[2] = zx(_mm_loadu_si128(at(c, 16 * 1).cast())); // mova xm2, [cq+16*1]
    m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, 16 * 4).cast())); // vinserti128 m3, [cq+16*4], 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, 16 * 5).cast())); // vinserti128 m2, [cq+16*5], 1
    m[4] = zx(_mm_loadu_si128(at(c, 16 * 2).cast())); // mova xm4, [cq+16*2]
    m[0] = zx(_mm_loadu_si128(at(c, 16 * 3).cast())); // mova xm0, [cq+16*3]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 16 * 6).cast())); // vinserti128 m4, [cq+16*6], 1
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 16 * 7).cast())); // vinserti128 m0, [cq+16*7], 1
    m[1] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m1, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[2] = _mm256_unpacklo_epi16(m[4], m[0]); // punpcklwd m2, m4, m0
    m[4] = _mm256_unpackhi_epi16(m[4], m[0]); // punpckhwd m4, m0
    m[0] = _mm256_unpacklo_epi32(m[1], m[2]); // punpckldq m0, m1, m2
    m[1] = _mm256_unpackhi_epi32(m[1], m[2]); // punpckhdq m1, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
}

/// `.pass2` of `iidentity_8x8_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_8x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(4096); // vpbroadcastd m4, [o(pw_4096)]
    iadst_8x8_internal_end(m, dst, stride, c) // jmp m(iadst_8x8_internal_8bpc).end
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x8_8bpc_avx2,
    inv_txfm_add_8x8,
    K::Identity,
    K::Identity
);

// 8x16
//
// Every first pass of 8x16 and of 16x8 advances `cq` by `32*4` (to the middle of the coefficient buffer) and the
// second passes clear the buffer relative to that: the `_pass1` functions take `c: &mut Coef` and leave it
// advanced, the `_pass2` functions and the `.end*` tails take that advanced pointer.

/// `%macro INV_TXFM_8X16_FN 2 ; type1, type2`: the DC-only path of dct_dct, then the two passes.
#[inline(always)]
unsafe fn inv_txfm_add_8x16(dst: Dst, stride: isize, mut c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        let r3d = eob | 16; // or r3d, 16
        return inv_txfm_add_dct_dct_8x8_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_8x8_8bpc).dconly
    }
    match t1 {
        K::Dct => idct_8x16_internal_pass1(m, &mut c),
        K::Adst => iadst_8x16_internal_pass1(m, &mut c),
        K::FlipAdst => iflipadst_8x16_internal_pass1(m, &mut c),
        K::Identity => iidentity_8x16_internal_pass1(m, &mut c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x16_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_8x16_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x16_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x16_internal_pass2(m, dst, stride, c),
    }
}

/// `%macro ITX_8X16_LOAD_COEFS 0`
#[inline(always)]
unsafe fn itx_8x16_load_coefs(m: &mut M, c: &mut Coef) {
    m[4] = pw(2896 * 8); // vpbroadcastd m4, [o(pw_2896x8)]
    m[0] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, 32 * 0).cast())); // pmulhrsw m0, m4, [cq+32*0]
    *c = at(*c, 32 * 4); // add cq, 32*4
    m[7] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, 32 * 3).cast())); // pmulhrsw m7, m4, [cq+32*3]
    m[1] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, -32 * 3).cast())); // pmulhrsw m1, m4, [cq-32*3]
    m[6] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, 32 * 2).cast())); // pmulhrsw m6, m4, [cq+32*2]
    m[2] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, -32 * 2).cast())); // pmulhrsw m2, m4, [cq-32*2]
    m[5] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, 32 * 1).cast())); // pmulhrsw m5, m4, [cq+32*1]
    m[3] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, -32 * 1).cast())); // pmulhrsw m3, m4, [cq-32*1]
    m[4] = _mm256_mulhrs_epi16(m[4], _mm256_loadu_si256(at(*c, 32 * 0).cast()));
    // pmulhrsw m4, [cq+32*0]
}

/// `cglobal idct_8x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_8x16_internal_pass1(m: &mut M, c: &mut Coef) {
    itx_8x16_load_coefs(m, c); // ITX_8X16_LOAD_COEFS
    idct_16x8_internal_main(m); // call m(idct_16x8_internal_8bpc).main
    m[10] = pw(16384); // vpbroadcastd m10, [o(pw_16384)]
    idct_8x16_internal_pass1_end(m)
}

/// `.pass1_end` of `idct_8x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_pass1_end(m: &mut M) {
    m[9] = _mm256_permute2x128_si256::<0x31>(m[3], m[7]); // vperm2i128 m9, m3, m7, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[7])); // vinserti128 m3, xm7, 1
    m[8] = _mm256_permute2x128_si256::<0x31>(m[2], m[6]); // vperm2i128 m8, m2, m6, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[6])); // vinserti128 m2, xm6, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[1], m[5]); // vperm2i128 m6, m1, m5, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[5])); // vinserti128 m1, xm5, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[0], m[4]); // vperm2i128 m5, m0, m4, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m0, xm4, 1
    m[4] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m4, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m3, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    idct_8x16_internal_pass1_end2(m)
}

/// `.pass1_end2` of `idct_8x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_pass1_end2(m: &mut M) {
    m[7] = _mm256_unpackhi_epi16(m[5], m[6]); // punpckhwd m7, m5, m6
    m[5] = _mm256_unpacklo_epi16(m[5], m[6]); // punpcklwd m5, m6
    m[6] = _mm256_unpacklo_epi16(m[8], m[9]); // punpcklwd m6, m8, m9
    m[8] = _mm256_unpackhi_epi16(m[8], m[9]); // punpckhwd m8, m9
    for i in [2, 0, 4, 3, 5, 6, 7, 8] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[10]); // REPX {pmulhrsw x, m10}, m2, m0, m4, m3, m5, m6, m7, m8
    }
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
    m[4] = _mm256_unpacklo_epi32(m[5], m[6]); // punpckldq m4, m5, m6
    m[5] = _mm256_unpackhi_epi32(m[5], m[6]); // punpckhdq m5, m6
    m[6] = _mm256_unpacklo_epi32(m[7], m[8]); // punpckldq m6, m7, m8
    m[7] = _mm256_unpackhi_epi32(m[7], m[8]); // punpckhdq m7, m8
}

/// `.pass2` of `idct_8x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_8x16_internal_main(m); // call .main
    for i in [0, 2, 4, 6] {
        m[i] = _mm256_permute4x64_epi64::<0xd8>(m[i]); // REPX {vpermq x, x, q3120}, m0, m2, m4, m6
    }
    for i in [1, 3, 5, 7] {
        m[i] = _mm256_permute4x64_epi64::<0x8d>(m[i]); // REPX {vpermq x, x, q2031}, m1, m3, m5, m7
    }
    idct_8x16_internal_end(m, dst, stride, c)
}

/// `.end` of `idct_8x16_internal_8bpc` (`c` is `cq` as the first pass left it: the buffer's start + `32*4`)
#[inline(always)]
unsafe fn idct_8x16_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pw(2048); // vpbroadcastd m8, [o(pw_2048)]
    idct_8x16_internal_end2(m, dst, stride, c)
}

/// `.end2` of `idct_8x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    for i in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[8]); // REPX {pmulhrsw x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x16_internal_end3(m, dst, stride, c)
}

/// `.end3` of `idct_8x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_end3(m: &mut M, mut dst: Dst, stride: isize, c: Coef) {
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[8]); // REPX {mova [cq+32*x], m8}, -4, -3, -2, -1, 0, 1, 2, 3
    }
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_8x4(m, dst, m[0], m[1], 8, 9, [stride, stride * 2, r3]); // WRITE_8X4 0, 1, 8, 9
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[2], m[3], 0, 1, [stride, stride * 2, r3]); // WRITE_8X4 2, 3, 0, 1
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[4], m[5], 0, 1, [stride, stride * 2, r3]); // WRITE_8X4 4, 5, 0, 1
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[6], m[7], 0, 1, [stride, stride * 2, r3]); // WRITE_8X4 6, 7, 0, 1
}

/// `.main` of `idct_8x16_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn idct_8x16_internal_main(m: &mut M) {
    idct16_1d_packed(m) // IDCT16_1D_PACKED
}

/// `cglobal iadst_8x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_8x16_internal_pass1(m: &mut M, c: &mut Coef) {
    itx_8x16_load_coefs(m, c); // ITX_8X16_LOAD_COEFS
    iadst_16x8_internal_main(m); // call m(iadst_16x8_internal_8bpc).main
    iadst_16x8_internal_main_pass1_end(m); // call m(iadst_16x8_internal_8bpc).main_pass1_end
    m[10] = pw(16384); // vpbroadcastd m10, [o(pw_16384)]
    m[9] = _mm256_slli_epi32::<17>(m[10]); // pslld m9, m10, 17
    m[10] = _mm256_sub_epi16(m[10], m[9]); // psubw m10, m9 ; 16384, -16384
    idct_8x16_internal_pass1_end(m) // jmp m(idct_8x16_internal_8bpc).pass1_end
}

/// `.pass2` of `iadst_8x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x16_internal_main(m); // call .main
    iadst_8x16_internal_main_pass2_end(m); // call .main_pass2_end
    m[9] = pw(2048); // vpbroadcastd m9, [o(pw_2048)]
    m[8] = zx(lo(pw(4096))); // vpbroadcastd xm8, [o(pw_4096)]
    m[8] = _mm256_sub_epi16(m[8], m[9]); // psubw m8, m9
    for i in [0, 1, 2, 3] {
        m[i] = _mm256_permute4x64_epi64::<0x8d>(m[i]); // REPX {vpermq x, x, q2031}, m0, m1, m2, m3
    }
    for i in [4, 5, 6, 7] {
        m[i] = _mm256_permute4x64_epi64::<0xd8>(m[i]); // REPX {vpermq x, x, q3120}, m4, m5, m6, m7
    }
    idct_8x16_internal_end2(m, dst, stride, c) // jmp m(idct_8x16_internal_8bpc).end2
}

/// `.main` of `iadst_8x16_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_8x16_internal_main(m: &mut M) {
    for i in [7, 1, 5, 3] {
        m[i] = _mm256_shuffle_epi32::<0x4e>(m[i]); // REPX {pshufd x, x, q1032}, m7, m1, m5, m3
    }
    iadst_8x16_internal_main2(m)
}

/// `.main2` of `iadst_8x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x16_internal_main2(m: &mut M) {
    const X: usize = usize::MAX; // the `_` of an unused macro argument
    m[10] = pd(2048); // vpbroadcastd m10, [o(pd_2048)]
    m[8] = _mm256_unpackhi_epi16(m[7], m[0]); // punpckhwd m8, m7, m0 ; in14 in1
    m[0] = _mm256_unpacklo_epi16(m[0], m[7]); // punpcklwd m0, m7 ; in0 in15
    m[7] = _mm256_unpacklo_epi16(m[6], m[1]); // punpcklwd m7, m6, m1 ; in12 in3
    m[1] = _mm256_unpackhi_epi16(m[1], m[6]); // punpckhwd m1, m6 ; in2 in13
    m[6] = _mm256_unpackhi_epi16(m[5], m[2]); // punpckhwd m6, m5, m2 ; in10 in5
    m[2] = _mm256_unpacklo_epi16(m[2], m[5]); // punpcklwd m2, m5 ; in4 in11
    m[5] = _mm256_unpacklo_epi16(m[4], m[3]); // punpcklwd m5, m4, m3 ; in8 in7
    m[3] = _mm256_unpackhi_epi16(m[3], m[4]); // punpckhwd m3, m4 ; in6 in9
    itx_mul2x_pack(m, 0, 4, 9, 10, 201, 4091, 3); // ITX_MUL2X_PACK 0, 4, 9, 10, 201, 4091, 3 ; t0 t1
    itx_mul2x_pack(m, 1, 4, 9, 10, 995, 3973, 3); // ITX_MUL2X_PACK 1, 4, 9, 10, 995, 3973, 3 ; t2 t3
    itx_mul2x_pack(m, 2, 4, 9, 10, 1751, 3703, 3); // ITX_MUL2X_PACK 2, 4, 9, 10, 1751, 3703, 3 ; t4 t5
    itx_mul2x_pack(m, 3, 4, 9, 10, 2440, 3290, 3); // ITX_MUL2X_PACK 3, 4, 9, 10, 2440, 3290, 3 ; t6 t7
    itx_mul2x_pack(m, 5, 4, 9, 10, 3035, 2751, 3); // ITX_MUL2X_PACK 5, 4, 9, 10, 3035, 2751, 3 ; t8 t9
    itx_mul2x_pack(m, 6, 4, 9, 10, 3513, 2106, 3); // ITX_MUL2X_PACK 6, 4, 9, 10, 3513, 2106, 3 ; t10 t11
    itx_mul2x_pack(m, 7, 4, 9, 10, 3857, 1380, 3); // ITX_MUL2X_PACK 7, 4, 9, 10, 3857, 1380, 3 ; t12 t13
    itx_mul2x_pack(m, 8, 4, 9, 10, 4052, 601, 3); // ITX_MUL2X_PACK 8, 4, 9, 10, 4052, 601, 3 ; t14 t15
    m[4] = _mm256_subs_epi16(m[0], m[5]); // psubsw m4, m0, m5 ; t9a t8a
    m[0] = _mm256_adds_epi16(m[0], m[5]); // paddsw m0, m5 ; t1a t0a
    m[5] = _mm256_subs_epi16(m[1], m[6]); // psubsw m5, m1, m6 ; t11a t10a
    m[1] = _mm256_adds_epi16(m[1], m[6]); // paddsw m1, m6 ; t3a t2a
    m[6] = _mm256_subs_epi16(m[2], m[7]); // psubsw m6, m2, m7 ; t13a t12a
    m[2] = _mm256_adds_epi16(m[2], m[7]); // paddsw m2, m7 ; t5a t4a
    m[7] = _mm256_subs_epi16(m[3], m[8]); // psubsw m7, m3, m8 ; t15a t14a
    m[3] = _mm256_adds_epi16(m[3], m[8]); // paddsw m3, m8 ; t7a t6a
    m[11] = pw2(-4017, 799); // vpbroadcastd m11, [o(pw_m4017_799)]
    m[12] = pw2(799, 4017); // vpbroadcastd m12, [o(pw_799_4017)]
    m[9] = _mm256_setzero_si256(); // pxor m9, m9
    itx_mul2x_pack(m, 4, 8, X, 10, 11, 12, 6); // ITX_MUL2X_PACK 4, 8, _, 10, 11, 12, 6 ; t8 t9
    m[8] = _mm256_sub_epi16(m[9], m[11]); // psubw m8, m9, m11 ; pw_4017_m799
    itx_mul2x_pack(m, 6, 12, X, 10, 12, 8, 6); // ITX_MUL2X_PACK 6, 12, _, 10, 12, 8, 6 ; t12 t13
    m[11] = pw2(-2276, 3406); // vpbroadcastd m11, [o(pw_m2276_3406)]
    m[12] = pw2(3406, 2276); // vpbroadcastd m12, [o(pw_3406_2276)]
    itx_mul2x_pack(m, 5, 8, X, 10, 11, 12, 6); // ITX_MUL2X_PACK 5, 8, _, 10, 11, 12, 6 ; t10 t11
    m[8] = _mm256_sub_epi16(m[9], m[11]); // psubw m8, m9, m11 ; pw_2276_m3406
    itx_mul2x_pack(m, 7, 12, X, 10, 12, 8, 6); // ITX_MUL2X_PACK 7, 12, _, 10, 12, 8, 6 ; t14 t15
    m[8] = _mm256_subs_epi16(m[1], m[3]); // psubsw m8, m1, m3 ; t7 t6
    m[1] = _mm256_adds_epi16(m[1], m[3]); // paddsw m1, m3 ; t3 t2
    m[3] = _mm256_subs_epi16(m[0], m[2]); // psubsw m3, m0, m2 ; t5 t4
    m[0] = _mm256_adds_epi16(m[0], m[2]); // paddsw m0, m2 ; t1 t0
    m[2] = _mm256_subs_epi16(m[5], m[7]); // psubsw m2, m5, m7 ; t14a t15a
    m[7] = _mm256_adds_epi16(m[7], m[5]); // paddsw m7, m5 ; t10a t11a
    m[5] = _mm256_subs_epi16(m[4], m[6]); // psubsw m5, m4, m6 ; t12a t13a
    m[4] = _mm256_adds_epi16(m[4], m[6]); // paddsw m4, m6 ; t8a t9a
    m[11] = pw2(-3784, 1567); // vpbroadcastd m11, [o(pw_m3784_1567)]
    m[12] = pw2(1567, 3784); // vpbroadcastd m12, [o(pw_1567_3784)]
    itx_mul2x_pack(m, 3, 6, X, 10, 12, 11, 6); // ITX_MUL2X_PACK 3, 6, _, 10, 12, 11, 6 ; t5a t4a
    m[6] = _mm256_sub_epi16(m[9], m[11]); // psubw m6, m9, m11 ; pw_3784_m1567
    itx_mul2x_pack(m, 8, 6, X, 10, 6, 12, 6); // ITX_MUL2X_PACK 8, 6, _, 10, 6, 12, 6 ; t7a t6a
    m[11] = pw2(-1567, 3784); // vpbroadcastd m11, [o(pw_m1567_3784)]
    m[12] = pw2(3784, 1567); // vpbroadcastd m12, [o(pw_3784_1567)]
    itx_mul2x_pack(m, 2, 6, X, 10, 11, 12, 6); // ITX_MUL2X_PACK 2, 6, _, 10, 11, 12, 6 ; t15 t14
    m[6] = _mm256_sub_epi16(m[9], m[11]); // psubw m6, m9, m11 ; pw_1567_m3784
    itx_mul2x_pack(m, 5, 12, X, 10, 12, 6, 6); // ITX_MUL2X_PACK 5, 12, _, 10, 12, 6, 6 ; t13 t12
    m[12] = vbroadcasti128_p2(&deint_shuf); // vbroadcasti128 m12, [o(deint_shuf)]
    m[6] = _mm256_adds_epi16(m[4], m[7]); // paddsw m6, m4, m7 ; -out1 out14
    m[4] = _mm256_subs_epi16(m[4], m[7]); // psubsw m4, m7 ; t10 t11
    m[11] = _mm256_subs_epi16(m[3], m[8]); // psubsw m11, m3, m8 ; t7 t6
    m[8] = _mm256_adds_epi16(m[8], m[3]); // paddsw m8, m3 ; out12 -out3
    m[3] = _mm256_subs_epi16(m[0], m[1]); // psubsw m3, m0, m1 ; t3a t2a
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; -out15 out0
    m[1] = _mm256_adds_epi16(m[2], m[5]); // paddsw m1, m2, m5 ; -out13 out2
    m[5] = _mm256_subs_epi16(m[5], m[2]); // psubsw m5, m2 ; t15a t14a
    m[0] = _mm256_shuffle_epi8(m[0], m[12]); // pshufb m0, m12
    m[6] = _mm256_shuffle_epi8(m[6], m[12]); // pshufb m6, m12
    m[8] = _mm256_shuffle_epi8(m[8], m[12]); // pshufb m8, m12
    m[1] = _mm256_shuffle_epi8(m[1], m[12]); // pshufb m1, m12
    m[7] = shufps_p2::<0x4e>(m[6], m[0]); // shufps m7, m6, m0, q1032 ; out14 -out15
    m[0] = _mm256_blend_epi32::<0x33>(m[0], m[6]); // vpblendd m0, m6, 0x33 ; -out1 out0
    m[6] = _mm256_unpacklo_epi64(m[8], m[1]); // punpcklqdq m6, m8, m1 ; out12 -out13
    m[1] = _mm256_unpackhi_epi64(m[8], m[1]); // punpckhqdq m1, m8, m1 ; -out3 out2
}

/// `.main_pass1_end` of `iadst_8x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_8x16_internal_main_pass1_end(m: &mut M) {
    m[8] = pw2(-2896, 2896); // vpbroadcastd m8, [o(pw_m2896_2896)]
    m[12] = pw2(2896, 2896); // vpbroadcastd m12, [o(pw_2896_2896)]
    m[9] = _mm256_madd_epi16(m[8], m[11]); // pmaddwd m9, m8, m11 ; -out11
    m[2] = _mm256_madd_epi16(m[12], m[5]); // pmaddwd m2, m12, m5 ; -out5
    m[5] = _mm256_madd_epi16(m[5], m[8]); // pmaddwd m5, m8 ; out10
    m[11] = _mm256_madd_epi16(m[11], m[12]); // pmaddwd m11, m12 ; out4
    for i in [9, 5, 2, 11] {
        m[i] = _mm256_add_epi32(m[i], m[10]); // REPX {paddd x, m10}, m9, m5, m2, m11
    }
    for i in [9, 5, 2, 11] {
        m[i] = _mm256_srai_epi32::<12>(m[i]); // REPX {psrad x, 12}, m9, m5, m2, m11
    }
    m[5] = _mm256_packs_epi32(m[5], m[9]); // packssdw m5, m9 ; out10 -out11
    m[2] = _mm256_packs_epi32(m[2], m[11]); // packssdw m2, m11 ; -out5 out4
    m[11] = _mm256_madd_epi16(m[8], m[3]); // pmaddwd m11, m8, m3 ; out8
    m[8] = pw2(2896, -2896); // vpbroadcastd m8, [o(pw_2896_m2896)]
    m[3] = _mm256_madd_epi16(m[3], m[12]); // pmaddwd m3, m12 ; -out7
    m[8] = _mm256_madd_epi16(m[8], m[4]); // pmaddwd m8, m4 ; -out9
    m[4] = _mm256_madd_epi16(m[4], m[12]); // pmaddwd m4, m12 ; out6
    for i in [11, 3, 8, 4] {
        m[i] = _mm256_add_epi32(m[i], m[10]); // REPX {paddd x, m10}, m11, m3, m8, m4
    }
    for i in [11, 3, 8, 4] {
        m[i] = _mm256_srai_epi32::<12>(m[i]); // REPX {psrad x, 12}, m11, m3, m8, m4
    }
    m[3] = _mm256_packs_epi32(m[3], m[4]); // packssdw m3, m4 ; -out7 out6
    m[4] = _mm256_packs_epi32(m[11], m[8]); // packssdw m4, m11, m8 ; out8 -out9
    m[10] = pw(16384); // vpbroadcastd m10, [o(pw_16384)]
    m[9] = _mm256_setzero_si256(); // pxor m9, m9
}

/// `.main_pass2_end` of `iadst_8x16_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_8x16_internal_main_pass2_end(m: &mut M) {
    m[8] = pw(2896 * 8); // vpbroadcastd m8, [o(pw_2896x8)]
    m[2] = _mm256_shuffle_epi8(m[11], m[12]); // pshufb m2, m11, m12
    m[5] = _mm256_shuffle_epi8(m[5], m[12]); // pshufb m5, m12
    m[3] = _mm256_shuffle_epi8(m[3], m[12]); // pshufb m3, m12
    m[4] = _mm256_shuffle_epi8(m[4], m[12]); // pshufb m4, m12
    m[11] = _mm256_unpacklo_epi64(m[5], m[2]); // punpcklqdq m11, m5, m2 ; t15a t7
    m[5] = _mm256_unpackhi_epi64(m[5], m[2]); // punpckhqdq m5, m2 ; t14a t6
    m[2] = shufps_p2::<0x4e>(m[3], m[4]); // shufps m2, m3, m4, q1032 ; t2a t10
    m[3] = _mm256_blend_epi32::<0xcc>(m[3], m[4]); // vpblendd m3, m4, 0xcc ; t3a t11
    m[4] = _mm256_subs_epi16(m[2], m[3]); // psubsw m4, m2, m3 ; out8 -out9
    m[3] = _mm256_adds_epi16(m[3], m[2]); // paddsw m3, m2 ; -out7 out6
    m[2] = _mm256_adds_epi16(m[5], m[11]); // paddsw m2, m5, m11 ; -out5 out4
    m[5] = _mm256_subs_epi16(m[5], m[11]); // psubsw m5, m11 ; out10 -out11
    for i in [2, 3, 4, 5] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[8]); // REPX {pmulhrsw x, m8}, m2, m3, m4, m5
    }
}

/// `cglobal iflipadst_8x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_8x16_internal_pass1(m: &mut M, c: &mut Coef) {
    itx_8x16_load_coefs(m, c); // ITX_8X16_LOAD_COEFS
    iadst_16x8_internal_main(m); // call m(iadst_16x8_internal_8bpc).main
    iadst_16x8_internal_main_pass1_end(m); // call m(iadst_16x8_internal_8bpc).main_pass1_end
    m[9] = pw(16384); // vpbroadcastd m9, [o(pw_16384)]
    m[10] = _mm256_slli_epi32::<17>(m[9]); // pslld m10, m9, 17
    m[10] = _mm256_sub_epi16(m[10], m[9]); // psubw m10, m9 ; -16384, 16384
    m[9] = _mm256_permute2x128_si256::<0x31>(m[4], m[0]); // vperm2i128 m9, m4, m0, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[0])); // vinserti128 m0, m4, xm0, 1
    m[8] = _mm256_permute2x128_si256::<0x31>(m[5], m[1]); // vperm2i128 m8, m5, m1, 0x31
    m[4] = _mm256_inserti128_si256::<1>(m[5], lo(m[1])); // vinserti128 m4, m5, xm1, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[7], m[3]); // vperm2i128 m5, m7, m3, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[7], lo(m[3])); // vinserti128 m3, m7, xm3, 1
    m[1] = _mm256_inserti128_si256::<1>(m[6], lo(m[2])); // vinserti128 m1, m6, xm2, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[6], m[2]); // vperm2i128 m6, m6, m2, 0x31
    m[2] = _mm256_unpacklo_epi16(m[4], m[0]); // punpcklwd m2, m4, m0
    m[4] = _mm256_unpackhi_epi16(m[4], m[0]); // punpckhwd m4, m0
    m[0] = _mm256_unpacklo_epi16(m[3], m[1]); // punpcklwd m0, m3, m1
    m[3] = _mm256_unpackhi_epi16(m[3], m[1]); // punpckhwd m3, m1
    idct_8x16_internal_pass1_end2(m) // jmp m(idct_8x16_internal_8bpc).pass1_end2
}

/// `.pass2` of `iflipadst_8x16_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_8x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x16_internal_main(m); // call m(iadst_8x16_internal_8bpc).main
    iadst_8x16_internal_main_pass2_end(m); // call m(iadst_8x16_internal_8bpc).main_pass2_end
    m[8] = pw(2048); // vpbroadcastd m8, [o(pw_2048)]
    m[9] = zx(lo(pw(4096))); // vpbroadcastd xm9, [o(pw_4096)]
    m[8] = _mm256_sub_epi16(m[8], m[9]); // psubw m8, m9
    m[9] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m9, m0, q3120
    m[0] = _mm256_permute4x64_epi64::<0x8d>(m[7]); // vpermq m0, m7, q2031
    m[7] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m7, m1, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[6]); // vpermq m1, m6, q2031
    m[6] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m6, m2, q3120
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[5]); // vpermq m2, m5, q2031
    m[5] = _mm256_permute4x64_epi64::<0xd8>(m[3]); // vpermq m5, m3, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[4]); // vpermq m3, m4, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[8]); // pmulhrsw m0, m8
    m[1] = _mm256_mulhrs_epi16(m[1], m[8]); // pmulhrsw m1, m8
    m[2] = _mm256_mulhrs_epi16(m[2], m[8]); // pmulhrsw m2, m8
    m[3] = _mm256_mulhrs_epi16(m[3], m[8]); // pmulhrsw m3, m8
    m[4] = _mm256_mulhrs_epi16(m[5], m[8]); // pmulhrsw m4, m5, m8
    m[5] = _mm256_mulhrs_epi16(m[6], m[8]); // pmulhrsw m5, m6, m8
    m[6] = _mm256_mulhrs_epi16(m[7], m[8]); // pmulhrsw m6, m7, m8
    m[7] = _mm256_mulhrs_epi16(m[9], m[8]); // pmulhrsw m7, m9, m8
    idct_8x16_internal_end3(m, dst, stride, c) // jmp m(idct_8x16_internal_8bpc).end3
}

/// `%macro IDTX16 3-4 ; src/dst, tmp, pw_1697x16, [pw_16394]` (register numbers)
#[inline(always)]
unsafe fn idtx16(m: &mut M, r: usize, tmp: usize, c1697: usize, rnd: Option<usize>) {
    m[tmp] = _mm256_mulhrs_epi16(m[c1697], m[r]); // pmulhrsw m%2, m%3, m%1
    match rnd {
        Some(rnd) => m[tmp] = _mm256_mulhrs_epi16(m[tmp], m[rnd]), // pmulhrsw m%2, m%4 (if downshifting by 1)
        None => m[r] = _mm256_adds_epi16(m[r], m[r]),              // paddsw m%1, m%1
    }
    m[r] = _mm256_adds_epi16(m[r], m[tmp]); // paddsw m%1, m%2
}

/// `cglobal iidentity_8x16_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_8x16_internal_pass1(m: &mut M, c: &mut Coef) {
    m[3] = zx(_mm_loadu_si128(at(*c, 16 * 0).cast())); // mova xm3, [cq+16*0]
    m[2] = zx(_mm_loadu_si128(at(*c, 16 * 2).cast())); // mova xm2, [cq+16*2]
    *c = at(*c, 16 * 8); // add cq, 16*8
    m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(*c, 16 * 0).cast())); // vinserti128 m3, [cq+16*0], 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(*c, 16 * 2).cast())); // vinserti128 m2, [cq+16*2], 1
    m[9] = pw(2896 * 8); // vpbroadcastd m9, [o(pw_2896x8)]
    m[4] = zx(_mm_loadu_si128(at(*c, -16 * 4).cast())); // mova xm4, [cq-16*4]
    m[5] = zx(_mm_loadu_si128(at(*c, -16 * 2).cast())); // mova xm5, [cq-16*2]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(*c, 16 * 4).cast())); // vinserti128 m4, [cq+16*4], 1
    m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(*c, 16 * 6).cast())); // vinserti128 m5, [cq+16*6], 1
    m[7] = zx(_mm_loadu_si128(at(*c, -16 * 7).cast())); // mova xm7, [cq-16*7]
    m[6] = zx(_mm_loadu_si128(at(*c, -16 * 5).cast())); // mova xm6, [cq-16*5]
    m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(*c, 16 * 1).cast())); // vinserti128 m7, [cq+16*1], 1
    m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(*c, 16 * 3).cast())); // vinserti128 m6, [cq+16*3], 1
    m[8] = zx(_mm_loadu_si128(at(*c, -16 * 3).cast())); // mova xm8, [cq-16*3]
    m[0] = zx(_mm_loadu_si128(at(*c, -16 * 1).cast())); // mova xm0, [cq-16*1]
    m[8] = _mm256_inserti128_si256::<1>(m[8], _mm_loadu_si128(at(*c, 16 * 5).cast())); // vinserti128 m8, [cq+16*5], 1
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(*c, 16 * 7).cast())); // vinserti128 m0, [cq+16*7], 1
    m[1] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m1, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[2] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m2, m4, m5
    m[4] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m4, m5
    m[5] = _mm256_unpacklo_epi16(m[7], m[6]); // punpcklwd m5, m7, m6
    m[7] = _mm256_unpackhi_epi16(m[7], m[6]); // punpckhwd m7, m6
    m[6] = _mm256_unpacklo_epi16(m[8], m[0]); // punpcklwd m6, m8, m0
    m[8] = _mm256_unpackhi_epi16(m[8], m[0]); // punpckhwd m8, m0
    for i in [1, 2, 3, 4, 5, 6, 7, 8] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[9]); // REPX {pmulhrsw x, m9}, m1, m2, m3, m4, m5, m6, m7, m8
    }
    m[0] = _mm256_unpacklo_epi32(m[1], m[2]); // punpckldq m0, m1, m2
    m[1] = _mm256_unpackhi_epi32(m[1], m[2]); // punpckhdq m1, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m2, m3, m4
    m[3] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m3, m4
    m[4] = _mm256_unpacklo_epi32(m[5], m[6]); // punpckldq m4, m5, m6
    m[5] = _mm256_unpackhi_epi32(m[5], m[6]); // punpckhdq m5, m6
    m[6] = _mm256_unpacklo_epi32(m[7], m[8]); // punpckldq m6, m7, m8
    m[7] = _mm256_unpackhi_epi32(m[7], m[8]); // punpckhdq m7, m8
}

/// `.pass2` of `iidentity_8x16_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_8x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pw(1697 * 16); // vpbroadcastd m8, [o(pw_1697x16)]
    for i in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[i] = _mm256_permute4x64_epi64::<0xd8>(m[i]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for i in [0, 1, 2, 3, 4, 5, 6, 7] {
        idtx16(m, i, 9, 8, None); // REPX {IDTX16 x, 9, 8}, 0, 1, 2, 3, 4, 5, 6, 7
    }
    idct_8x16_internal_end(m, dst, stride, c) // jmp m(idct_8x16_internal_8bpc).end
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x16_8bpc_avx2,
    inv_txfm_add_8x16,
    K::Identity,
    K::Identity
);

// 16x8

/// `%macro INV_TXFM_16X8_FN 2 ; type1, type2`: the DC-only path of dct_dct, then the two passes.
#[inline(always)]
unsafe fn inv_txfm_add_16x8(dst: Dst, stride: isize, mut c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        let r3d = eob | 8; // or r3d, 8
        return inv_txfm_add_dct_dct_16x4_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_16x4_8bpc).dconly
    }
    match t1 {
        K::Dct => idct_16x8_internal_pass1(m, &mut c),
        K::Adst => iadst_16x8_internal_pass1(m, &mut c),
        K::FlipAdst => iflipadst_16x8_internal_pass1(m, &mut c),
        K::Identity => iidentity_16x8_internal_pass1(m, &mut c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x8_internal_pass2(m, dst, stride, c),
        K::Adst => iadst_16x8_internal_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_16x8_internal_pass2(m, dst, stride, c),
        K::Identity => iidentity_16x8_internal_pass2(m, dst, stride, c),
    }
}

/// `%macro ITX_16X8_LOAD_COEFS 1 ; shuf_odd` (`SHUF_ODD` is the `vpermq` immediate `q%1`)
#[inline(always)]
unsafe fn itx_16x8_load_coefs<const SHUF_ODD: i32>(m: &mut M, c: &mut Coef) {
    m[8] = pw(2896 * 8); // vpbroadcastd m8, [o(pw_2896x8)]
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(*c, 32 * 0).cast())); // vpermq m0, [cq+32*0], q3120
    *c = at(*c, 32 * 4); // add cq, 32*4
    m[7] = _mm256_permute4x64_epi64::<SHUF_ODD>(_mm256_loadu_si256(at(*c, 32 * 3).cast())); // vpermq m7, [cq+32*3], q%1
    m[1] = _mm256_permute4x64_epi64::<SHUF_ODD>(_mm256_loadu_si256(at(*c, -32 * 3).cast())); // vpermq m1, [cq-32*3], q%1
    m[6] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(*c, 32 * 2).cast())); // vpermq m6, [cq+32*2], q3120
    m[2] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(*c, -32 * 2).cast())); // vpermq m2, [cq-32*2], q3120
    m[5] = _mm256_permute4x64_epi64::<SHUF_ODD>(_mm256_loadu_si256(at(*c, 32 * 1).cast())); // vpermq m5, [cq+32*1], q%1
    m[3] = _mm256_permute4x64_epi64::<SHUF_ODD>(_mm256_loadu_si256(at(*c, -32 * 1).cast())); // vpermq m3, [cq-32*1], q%1
    m[4] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(*c, 32 * 0).cast())); // vpermq m4, [cq+32*0], q3120
    for i in [0, 7, 1, 6, 2, 5, 3, 4] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[8]); // REPX {pmulhrsw x, m8}, m0, m7, m1, m6, m2, m5, m3, m4
    }
}

/// `cglobal idct_16x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_16x8_internal_pass1(m: &mut M, c: &mut Coef) {
    itx_16x8_load_coefs::<0xd8>(m, c); // ITX_16X8_LOAD_COEFS 3120
    idct_8x16_internal_main(m); // call m(idct_8x16_internal_8bpc).main
    m[10] = pw(16384); // vpbroadcastd m10, [o(pw_16384)]
    m[8] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m8, m0, m2
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    m[2] = _mm256_unpackhi_epi16(m[1], m[3]); // punpckhwd m2, m1, m3
    m[1] = _mm256_unpacklo_epi16(m[1], m[3]); // punpcklwd m1, m3
    m[9] = _mm256_unpacklo_epi16(m[4], m[6]); // punpcklwd m9, m4, m6
    m[4] = _mm256_unpackhi_epi16(m[4], m[6]); // punpckhwd m4, m6
    m[6] = _mm256_unpacklo_epi16(m[5], m[7]); // punpcklwd m6, m5, m7
    m[5] = _mm256_unpackhi_epi16(m[5], m[7]); // punpckhwd m5, m7
    for i in [8, 1, 4, 6] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[10]); // REPX {pmulhrsw x, m10}, m8, m1, m4, m6
    }
    idct_16x8_internal_pass1_end(m)
}

/// `.pass1_end` of `idct_16x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_pass1_end(m: &mut M) {
    for i in [0, 2, 9, 5] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[10]); // REPX {pmulhrsw x, m10}, m0, m2, m9, m5
    }
    m[3] = _mm256_unpackhi_epi16(m[0], m[8]); // punpckhwd m3, m0, m8
    m[0] = _mm256_unpacklo_epi16(m[0], m[8]); // punpcklwd m0, m8
    m[8] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m8, m2, m1
    m[2] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m2, m1
    m[7] = _mm256_unpacklo_epi16(m[9], m[4]); // punpcklwd m7, m9, m4
    m[9] = _mm256_unpackhi_epi16(m[9], m[4]); // punpckhwd m9, m4
    m[4] = _mm256_unpacklo_epi16(m[5], m[6]); // punpcklwd m4, m5, m6
    m[5] = _mm256_unpackhi_epi16(m[5], m[6]); // punpckhwd m5, m6
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[8]); // punpckldq m2, m3, m8
    m[3] = _mm256_unpackhi_epi32(m[3], m[8]); // punpckhdq m3, m8
    m[6] = _mm256_unpacklo_epi32(m[7], m[4]); // punpckldq m6, m7, m4
    m[7] = _mm256_unpackhi_epi32(m[7], m[4]); // punpckhdq m7, m4
    m[8] = _mm256_unpacklo_epi32(m[9], m[5]); // punpckldq m8, m9, m5
    m[9] = _mm256_unpackhi_epi32(m[9], m[5]); // punpckhdq m9, m5
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[6]); // vperm2i128 m4, m0, m6, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[6])); // vinserti128 m0, xm6, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[7]); // vperm2i128 m5, m1, m7, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[7])); // vinserti128 m1, xm7, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[2], m[8]); // vperm2i128 m6, m2, m8, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[8])); // vinserti128 m2, xm8, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[3], m[9]); // vperm2i128 m7, m3, m9, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[9])); // vinserti128 m3, xm9, 1
}

/// `.pass2` of `idct_16x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x8_internal_main(m); // call .main
    m[8] = pw(2048); // vpbroadcastd m8, [o(pw_2048)]
    idct_16x8_internal_end(m, dst, stride, c)
}

/// `.end` of `idct_16x8_internal_8bpc` (`c` is `cq` as the first pass left it: the buffer's start + `32*4`)
#[inline(always)]
unsafe fn idct_16x8_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    for i in [0, 2, 4, 6] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[8]); // REPX {pmulhrsw x, m8}, m0, m2, m4, m6
    }
    idct_16x8_internal_end2(m, dst, stride, c)
}

/// `.end2` of `idct_16x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    for i in [1, 3, 5, 7] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[8]); // REPX {pmulhrsw x, m8}, m1, m3, m5, m7
    }
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
    write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r3]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r3
    idct_16x8_internal_end3(m, dst, stride, c, r3)
}

/// `.end3` of `idct_16x8_internal_8bpc` (`r3` is `strideq*3`)
#[inline(always)]
unsafe fn idct_16x8_internal_end3(m: &mut M, dst: Dst, stride: isize, c: Coef, r3: isize) {
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[0]); // REPX {mova [cq+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
    }
    idct_16x8_internal_end4(m, dst, stride, r3)
}

/// `.end4` of `idct_16x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_end4(m: &mut M, mut dst: Dst, stride: isize, r3: isize) {
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_16x2(m, dst, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
    write_16x2(m, dst, m[6], m[7], 0, 1, [stride * 2, r3]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r3
}

/// `.main` of `idct_16x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_main(m: &mut M) {
    m[10] = pd(2048); // vpbroadcastd m10, [o(pd_2048)]
    idct_16x8_internal_main2(m)
}

/// `.main2` of `idct_16x8_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_main2(m: &mut M) {
    idct8_1d(m, [0, 1, 2, 3, 4, 5, 6, 7], [8, 9], 10) // IDCT8_1D 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
}

/// `cglobal iadst_16x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_16x8_internal_pass1(m: &mut M, c: &mut Coef) {
    itx_16x8_load_coefs::<0x72>(m, c); // ITX_16X8_LOAD_COEFS 1302
    iadst_8x16_internal_main2(m); // call m(iadst_8x16_internal_8bpc).main2
    iadst_8x16_internal_main_pass1_end(m); // call m(iadst_8x16_internal_8bpc).main_pass1_end
    m[11] = _mm256_sub_epi16(m[9], m[10]); // psubw m11, m9, m10
    m[8] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m8, m0, m2
    m[0] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m0, m2
    m[2] = _mm256_unpackhi_epi16(m[1], m[3]); // punpckhwd m2, m1, m3
    m[1] = _mm256_unpacklo_epi16(m[1], m[3]); // punpcklwd m1, m3
    m[9] = _mm256_unpacklo_epi16(m[4], m[6]); // punpcklwd m9, m4, m6
    m[4] = _mm256_unpackhi_epi16(m[4], m[6]); // punpckhwd m4, m6
    m[6] = _mm256_unpackhi_epi16(m[5], m[7]); // punpckhwd m6, m5, m7
    m[5] = _mm256_unpacklo_epi16(m[5], m[7]); // punpcklwd m5, m7
    for i in [8, 1, 4, 6] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[11]); // REPX {pmulhrsw x, m11}, m8, m1, m4, m6
    }
    idct_16x8_internal_pass1_end(m) // jmp m(idct_16x8_internal_8bpc).pass1_end
}

/// `.pass2` of `iadst_16x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_16x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_16x8_internal_main(m); // call .main
    iadst_16x8_internal_main_pass2_end(m); // call .main_pass2_end
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    m[8] = _mm256_sub_epi16(m[8], m[9]); // psubw m8, m9
    for i in [0, 2, 4, 6] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[9]); // REPX {pmulhrsw x, m9}, m0, m2, m4, m6
    }
    idct_16x8_internal_end2(m, dst, stride, c) // jmp m(idct_16x8_internal_8bpc).end2
}

/// `.main` of `iadst_16x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_16x8_internal_main(m: &mut M) {
    m[10] = pd(2048); // vpbroadcastd m10, [o(pd_2048)]
    itx_mulsub_2w(m, 7, 0, 8, 9, 10, 401, 4076, None); // ITX_MULSUB_2W 7, 0, 8, 9, 10, 401, 4076 ; t1a, t0a
    itx_mulsub_2w(m, 3, 4, 8, 9, 10, 3166, 2598, None); // ITX_MULSUB_2W 3, 4, 8, 9, 10, 3166, 2598 ; t5a, t4a
    itx_mulsub_2w(m, 1, 6, 8, 9, 10, 3920, 1189, None); // ITX_MULSUB_2W 1, 6, 8, 9, 10, 3920, 1189 ; t7a, t6a
    itx_mulsub_2w(m, 5, 2, 8, 9, 10, 1931, 3612, None); // ITX_MULSUB_2W 5, 2, 8, 9, 10, 1931, 3612 ; t3a, t2a
    m[8] = _mm256_subs_epi16(m[2], m[6]); // psubsw m8, m2, m6 ; t6
    m[2] = _mm256_adds_epi16(m[2], m[6]); // paddsw m2, m6 ; t2
    m[6] = _mm256_subs_epi16(m[0], m[4]); // psubsw m6, m0, m4 ; t4
    m[0] = _mm256_adds_epi16(m[0], m[4]); // paddsw m0, m4 ; t0
    m[4] = _mm256_subs_epi16(m[5], m[1]); // psubsw m4, m5, m1 ; t7
    m[5] = _mm256_adds_epi16(m[5], m[1]); // paddsw m5, m1 ; t3
    m[1] = _mm256_subs_epi16(m[7], m[3]); // psubsw m1, m7, m3 ; t5
    m[7] = _mm256_adds_epi16(m[7], m[3]); // paddsw m7, m3 ; t1
    itx_mulsub_2w(m, 6, 1, 3, 9, 10, 1567, 3784, None); // ITX_MULSUB_2W 6, 1, 3, 9, 10, 1567, 3784 ; t5a, t4a
    itx_mulsub_2w(m, 4, 8, 3, 9, 10, 3784, 1567, None); // ITX_MULSUB_2W 4, 8, 3, 9, 10, 3784, 1567 ; t6a, t7a
    m[9] = _mm256_subs_epi16(m[6], m[8]); // psubsw m9, m6, m8 ; t7
    m[6] = _mm256_adds_epi16(m[6], m[8]); // paddsw m6, m8 ; out6
    m[3] = _mm256_subs_epi16(m[7], m[5]); // psubsw m3, m7, m5 ; t3
    m[7] = _mm256_adds_epi16(m[7], m[5]); // paddsw m7, m5 ; -out7
    m[5] = _mm256_subs_epi16(m[0], m[2]); // psubsw m5, m0, m2 ; t2
    m[0] = _mm256_adds_epi16(m[0], m[2]); // paddsw m0, m2 ; out0
    m[2] = _mm256_subs_epi16(m[1], m[4]); // psubsw m2, m1, m4 ; t6
    m[1] = _mm256_adds_epi16(m[1], m[4]); // paddsw m1, m4 ; -out1
}

/// `.main_pass1_end` of `iadst_16x8_internal_8bpc`
#[inline(always)]
unsafe fn iadst_16x8_internal_main_pass1_end(m: &mut M) {
    m[11] = pw2(-2896, 2896); // vpbroadcastd m11, [o(pw_m2896_2896)]
    m[12] = pw2(2896, 2896); // vpbroadcastd m12, [o(pw_2896_2896)]
    m[4] = _mm256_unpackhi_epi16(m[3], m[5]); // punpckhwd m4, m3, m5
    m[3] = _mm256_unpacklo_epi16(m[3], m[5]); // punpcklwd m3, m5
    m[5] = _mm256_madd_epi16(m[11], m[4]); // pmaddwd m5, m11, m4
    m[4] = _mm256_madd_epi16(m[4], m[12]); // pmaddwd m4, m12
    m[8] = _mm256_madd_epi16(m[11], m[3]); // pmaddwd m8, m11, m3
    m[3] = _mm256_madd_epi16(m[3], m[12]); // pmaddwd m3, m12
    for i in [5, 4, 8, 3] {
        m[i] = _mm256_add_epi32(m[i], m[10]); // REPX {paddd x, m10}, m5, m4, m8, m3
    }
    for i in [5, 8, 4, 3] {
        m[i] = _mm256_srai_epi32::<12>(m[i]); // REPX {psrad x, 12}, m5, m8, m4, m3
    }
    m[3] = _mm256_packs_epi32(m[3], m[4]); // packssdw m3, m4 ; -out3
    m[4] = _mm256_packs_epi32(m[8], m[5]); // packssdw m4, m8, m5 ; out4
    m[5] = _mm256_unpacklo_epi16(m[9], m[2]); // punpcklwd m5, m9, m2
    m[9] = _mm256_unpackhi_epi16(m[9], m[2]); // punpckhwd m9, m2
    m[2] = _mm256_madd_epi16(m[12], m[5]); // pmaddwd m2, m12, m5
    m[5] = _mm256_madd_epi16(m[5], m[11]); // pmaddwd m5, m11
    m[12] = _mm256_madd_epi16(m[12], m[9]); // pmaddwd m12, m9
    m[11] = _mm256_madd_epi16(m[11], m[9]); // pmaddwd m11, m9
    for i in [2, 5, 12, 11] {
        m[i] = _mm256_add_epi32(m[i], m[10]); // REPX {paddd x, m10}, m2, m5, m12, m11
    }
    for i in [2, 12, 5, 11] {
        m[i] = _mm256_srai_epi32::<12>(m[i]); // REPX {psrad x, 12}, m2, m12, m5, m11
    }
    m[2] = _mm256_packs_epi32(m[2], m[12]); // packssdw m2, m12 ; out2
    m[5] = _mm256_packs_epi32(m[5], m[11]); // packssdw m5, m11 ; -out5
}

/// `.main_pass2_end` of `iadst_16x8_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_16x8_internal_main_pass2_end(m: &mut M) {
    m[8] = pw(2896 * 8); // vpbroadcastd m8, [o(pw_2896x8)]
    m[4] = _mm256_subs_epi16(m[5], m[3]); // psubsw m4, m5, m3
    m[3] = _mm256_adds_epi16(m[3], m[5]); // paddsw m3, m5
    m[5] = _mm256_subs_epi16(m[2], m[9]); // psubsw m5, m2, m9
    m[2] = _mm256_adds_epi16(m[2], m[9]); // paddsw m2, m9
    m[2] = _mm256_mulhrs_epi16(m[2], m[8]); // pmulhrsw m2, m8 ; out2
    m[3] = _mm256_mulhrs_epi16(m[3], m[8]); // pmulhrsw m3, m8 ; -out3
    m[4] = _mm256_mulhrs_epi16(m[4], m[8]); // pmulhrsw m4, m8 ; out4
    m[5] = _mm256_mulhrs_epi16(m[5], m[8]); // pmulhrsw m5, m8 ; -out5
    m[9] = pw(2048); // vpbroadcastd m9, [o(pw_2048)]
}

/// `cglobal iflipadst_16x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_16x8_internal_pass1(m: &mut M, c: &mut Coef) {
    itx_16x8_load_coefs::<0x72>(m, c); // ITX_16X8_LOAD_COEFS 1302
    iadst_8x16_internal_main2(m); // call m(iadst_8x16_internal_8bpc).main2
    iadst_8x16_internal_main_pass1_end(m); // call m(iadst_8x16_internal_8bpc).main_pass1_end
    m[9] = _mm256_sub_epi16(m[9], m[10]); // psubw m9, m10
    m[8] = _mm256_unpacklo_epi16(m[6], m[4]); // punpcklwd m8, m6, m4
    m[6] = _mm256_unpackhi_epi16(m[6], m[4]); // punpckhwd m6, m4
    m[4] = _mm256_unpacklo_epi16(m[7], m[5]); // punpcklwd m4, m7, m5
    m[7] = _mm256_unpackhi_epi16(m[7], m[5]); // punpckhwd m7, m5
    m[5] = _mm256_unpackhi_epi16(m[3], m[1]); // punpckhwd m5, m3, m1
    m[3] = _mm256_unpacklo_epi16(m[3], m[1]); // punpcklwd m3, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[0]); // punpckhwd m1, m2, m0
    m[2] = _mm256_unpacklo_epi16(m[2], m[0]); // punpcklwd m2, m0
    for i in [8, 4, 5, 1] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[10]); // REPX {pmulhrsw x, m10}, m8, m4, m5, m1
    }
    for i in [6, 7, 3, 2] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[9]); // REPX {pmulhrsw x, m9}, m6, m7, m3, m2
    }
    m[0] = _mm256_unpacklo_epi16(m[7], m[4]); // punpcklwd m0, m7, m4
    m[7] = _mm256_unpackhi_epi16(m[7], m[4]); // punpckhwd m7, m4
    m[4] = _mm256_unpackhi_epi16(m[6], m[8]); // punpckhwd m4, m6, m8
    m[6] = _mm256_unpacklo_epi16(m[6], m[8]); // punpcklwd m6, m8
    m[8] = _mm256_unpackhi_epi16(m[3], m[5]); // punpckhwd m8, m3, m5
    m[3] = _mm256_unpacklo_epi16(m[3], m[5]); // punpcklwd m3, m5
    m[5] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m5, m2, m1
    m[2] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m2, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[6]); // punpckhdq m1, m0, m6
    m[0] = _mm256_unpacklo_epi32(m[0], m[6]); // punpckldq m0, m6
    m[6] = _mm256_unpacklo_epi32(m[7], m[4]); // punpckldq m6, m7, m4
    m[7] = _mm256_unpackhi_epi32(m[7], m[4]); // punpckhdq m7, m4
    m[4] = _mm256_unpackhi_epi32(m[3], m[5]); // punpckhdq m4, m3, m5
    m[3] = _mm256_unpacklo_epi32(m[3], m[5]); // punpckldq m3, m5
    m[5] = _mm256_unpacklo_epi32(m[8], m[2]); // punpckldq m5, m8, m2
    m[8] = _mm256_unpackhi_epi32(m[8], m[2]); // punpckhdq m8, m2
    m[2] = _mm256_inserti128_si256::<1>(m[6], lo(m[5])); // vinserti128 m2, m6, xm5, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[6], m[5]); // vperm2i128 m6, m5, 0x31
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[4]); // vperm2i128 m5, m1, m4, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[4])); // vinserti128 m1, xm4, 1
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m4, m0, m3, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m0, xm3, 1
    m[3] = _mm256_inserti128_si256::<1>(m[7], lo(m[8])); // vinserti128 m3, m7, xm8, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[7], m[8]); // vperm2i128 m7, m8, 0x31
}

/// `.pass2` of `iflipadst_16x8_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_16x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_16x8_internal_main(m); // call m(iadst_16x8_internal_8bpc).main
    iadst_16x8_internal_main_pass2_end(m); // call m(iadst_16x8_internal_8bpc).main_pass2_end
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    m[8] = _mm256_sub_epi16(m[8], m[9]); // psubw m8, m9
    m[10] = _mm256_mulhrs_epi16(m[7], m[8]); // pmulhrsw m10, m7, m8
    m[7] = _mm256_mulhrs_epi16(m[0], m[9]); // pmulhrsw m7, m0, m9
    m[0] = _mm256_mulhrs_epi16(m[6], m[9]); // pmulhrsw m0, m6, m9
    m[6] = _mm256_mulhrs_epi16(m[1], m[8]); // pmulhrsw m6, m1, m8
    m[1] = _mm256_mulhrs_epi16(m[5], m[8]); // pmulhrsw m1, m5, m8
    m[5] = _mm256_mulhrs_epi16(m[2], m[9]); // pmulhrsw m5, m2, m9
    m[2] = _mm256_mulhrs_epi16(m[4], m[9]); // pmulhrsw m2, m4, m9
    m[4] = _mm256_mulhrs_epi16(m[3], m[8]); // pmulhrsw m4, m3, m8
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_16x2(m, dst, m[10], m[0], 8, 9, [0, stride]); // WRITE_16X2 10, 0, 8, 9, strideq*0, strideq*1
    write_16x2(m, dst, m[1], m[2], 0, 1, [stride * 2, r3]); // WRITE_16X2 1, 2, 0, 1, strideq*2, r3
    idct_16x8_internal_end3(m, dst, stride, c, r3) // jmp m(idct_16x8_internal_8bpc).end3
}

/// `cglobal iidentity_16x8_internal_8bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_16x8_internal_pass1(m: &mut M, c: &mut Coef) {
    m[7] = zx(_mm_loadu_si128(at(*c, 16 * 0).cast())); // mova xm7, [cq+16*0]
    m[2] = zx(_mm_loadu_si128(at(*c, 16 * 1).cast())); // mova xm2, [cq+16*1]
    *c = at(*c, 16 * 8); // add cq, 16*8
    m[3] = pw(2896 * 8); // vpbroadcastd m3, [o(pw_2896x8)]
    m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(*c, 16 * 0).cast())); // vinserti128 m7, [cq+16*0], 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(*c, 16 * 1).cast())); // vinserti128 m2, [cq+16*1], 1
    m[6] = zx(_mm_loadu_si128(at(*c, -16 * 6).cast())); // mova xm6, [cq-16*6]
    m[4] = zx(_mm_loadu_si128(at(*c, -16 * 5).cast())); // mova xm4, [cq-16*5]
    m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(*c, 16 * 2).cast())); // vinserti128 m6, [cq+16*2], 1
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(*c, 16 * 3).cast())); // vinserti128 m4, [cq+16*3], 1
    m[8] = zx(_mm_loadu_si128(at(*c, -16 * 4).cast())); // mova xm8, [cq-16*4]
    m[5] = zx(_mm_loadu_si128(at(*c, -16 * 3).cast())); // mova xm5, [cq-16*3]
    m[8] = _mm256_inserti128_si256::<1>(m[8], _mm_loadu_si128(at(*c, 16 * 4).cast())); // vinserti128 m8, [cq+16*4], 1
    m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(*c, 16 * 5).cast())); // vinserti128 m5, [cq+16*5], 1
    m[0] = zx(_mm_loadu_si128(at(*c, -16 * 2).cast())); // mova xm0, [cq-16*2]
    m[1] = zx(_mm_loadu_si128(at(*c, -16 * 1).cast())); // mova xm1, [cq-16*1]
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(*c, 16 * 6).cast())); // vinserti128 m0, [cq+16*6], 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(*c, 16 * 7).cast())); // vinserti128 m1, [cq+16*7], 1
    m[10] = pw(1697 * 16); // vpbroadcastd m10, [o(pw_1697x16)]
    m[11] = pw(16384); // vpbroadcastd m11, [o(pw_16384)]
    for i in [7, 2, 6, 4, 8, 5, 0, 1] {
        m[i] = _mm256_mulhrs_epi16(m[i], m[3]); // REPX {pmulhrsw x, m3}, m7, m2, m6, m4, m8, m5, m0, m1
    }
    m[3] = _mm256_unpacklo_epi16(m[7], m[2]); // punpcklwd m3, m7, m2
    m[7] = _mm256_unpackhi_epi16(m[7], m[2]); // punpckhwd m7, m2
    m[2] = _mm256_unpacklo_epi16(m[6], m[4]); // punpcklwd m2, m6, m4
    m[6] = _mm256_unpackhi_epi16(m[6], m[4]); // punpckhwd m6, m4
    m[4] = _mm256_unpacklo_epi16(m[8], m[5]); // punpcklwd m4, m8, m5
    m[8] = _mm256_unpackhi_epi16(m[8], m[5]); // punpckhwd m8, m5
    m[5] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m5, m0, m1
    m[0] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m0, m1
    m[1] = _mm256_unpacklo_epi32(m[3], m[2]); // punpckldq m1, m3, m2
    m[3] = _mm256_unpackhi_epi32(m[3], m[2]); // punpckhdq m3, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[5]); // punpckldq m2, m4, m5
    m[4] = _mm256_unpackhi_epi32(m[4], m[5]); // punpckhdq m4, m5
    m[5] = _mm256_unpacklo_epi32(m[7], m[6]); // punpckldq m5, m7, m6
    m[7] = _mm256_unpackhi_epi32(m[7], m[6]); // punpckhdq m7, m6
    m[6] = _mm256_unpacklo_epi32(m[8], m[0]); // punpckldq m6, m8, m0
    m[8] = _mm256_unpackhi_epi32(m[8], m[0]); // punpckhdq m8, m0
    for i in [1, 3, 2, 4, 5, 7, 6, 8] {
        idtx16(m, i, 0, 10, Some(11)); // REPX {IDTX16 x, 0, 10, 11}, 1, 3, 2, 4, 5, 7, 6, 8
    }
    m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
    m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
    m[4] = _mm256_unpacklo_epi64(m[5], m[6]); // punpcklqdq m4, m5, m6
    m[5] = _mm256_unpackhi_epi64(m[5], m[6]); // punpckhqdq m5, m6
    m[6] = _mm256_unpacklo_epi64(m[7], m[8]); // punpcklqdq m6, m7, m8
    m[7] = _mm256_unpackhi_epi64(m[7], m[8]); // punpckhqdq m7, m8
}

/// `.pass2` of `iidentity_16x8_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_16x8_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pw(4096); // vpbroadcastd m8, [o(pw_4096)]
    idct_16x8_internal_end(m, dst, stride, c) // jmp m(idct_16x8_internal_8bpc).end
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x8_8bpc_avx2,
    inv_txfm_add_16x8,
    K::Identity,
    K::Identity
);

// --- part 3: itx_avx2.asm lines 2460-3545 (16x16, 8x32, 32x8) --------------------

/// `REPX {op x, …}, a, b, …`: the statement once per listed register number (or offset), in the list's order.
macro_rules! repx {
    ([$($i:expr),* $(,)?], $x:ident => $body:expr) => {
        $({
            let $x = $i;
            $body;
        })*
    };
}

// 16x16

/// `%macro ITX_16X16_LOAD_COEFS 0`: the sixteen rows into `m0`-`m15`, `m15` also into the frame's first
/// slot; `cq` advances by eight rows.
#[inline(always)]
unsafe fn itx_16x16_load_coefs(m: &mut M, cq: &mut Coef, s: &mut [__m256i; 3]) {
    let mut c = *cq;
    m[0] = _mm256_loadu_si256(at(c, 32 * 0).cast()); // mova m0, [cq+32*0]
    m[1] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m1, [cq+32*1]
    m[2] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m2, [cq+32*2]
    m[3] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m3, [cq+32*3]
    c = at(c, 32 * 8); // add cq, 32*8
    m[4] = _mm256_loadu_si256(at(c, -32 * 4).cast()); // mova m4, [cq-32*4]
    m[5] = _mm256_loadu_si256(at(c, -32 * 3).cast()); // mova m5, [cq-32*3]
    m[6] = _mm256_loadu_si256(at(c, -32 * 2).cast()); // mova m6, [cq-32*2]
    m[7] = _mm256_loadu_si256(at(c, -32 * 1).cast()); // mova m7, [cq-32*1]
    m[8] = _mm256_loadu_si256(at(c, 32 * 0).cast()); // mova m8, [cq+32*0]
    m[9] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m9, [cq+32*1]
    m[10] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m10, [cq+32*2]
    m[11] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m11, [cq+32*3]
    m[12] = _mm256_loadu_si256(at(c, 32 * 4).cast()); // mova m12, [cq+32*4]
    m[13] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m13, [cq+32*5]
    m[14] = _mm256_loadu_si256(at(c, 32 * 6).cast()); // mova m14, [cq+32*6]
    m[15] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m15, [cq+32*7]
    s[0] = m[15]; // mova [rsp], m15
    *cq = c;
}

/// `cglobal idct_16x16_internal_8bpc, 0, 5, 16, 32*3, dst, stride, c, eob, tx2` up to `jmp tx2q`. `s` is the
/// function's frame of three 32-byte slots (`[rsp+32*0..2]`), shared by both passes.
#[inline(always)]
unsafe fn idct_16x16_internal_pass1(m: &mut M, cq: &mut Coef, s: &mut [__m256i; 3]) {
    itx_16x16_load_coefs(m, cq, s); // ITX_16X16_LOAD_COEFS
    idct_16x16_internal_main(m, s); // call .main
    idct_16x16_internal_pass1_end(m, s)
}

/// `.pass1_end` of `idct_16x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x16_internal_pass1_end(m: &mut M, s: &mut [__m256i; 3]) {
    m[1] = pw(8192); // vpbroadcastd m1, [o(pw_8192)]
    repx!([0, 2, 4, 6, 8, 10, 12, 14], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m0, m2, m4, m6, m8, m10, m12, m14
    _mm_storeu_si128(
        s.as_mut_ptr().cast::<__m128i>().add(5),
        _mm256_extracti128_si256::<1>(m[8]),
    ); // vextracti128 [rsp+16*5], m8, 1
    _mm_storeu_si128(s.as_mut_ptr().cast::<__m128i>().add(1), lo(m[8])); // mova [rsp+16*1], xm8
    idct_16x16_internal_pass1_end2(m, s)
}

/// `.pass1_end2` of `idct_16x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x16_internal_pass1_end2(m: &mut M, s: &mut [__m256i; 3]) {
    _mm_storeu_si128(
        s.as_mut_ptr().cast::<__m128i>().add(4),
        _mm256_extracti128_si256::<1>(m[0]),
    ); // vextracti128 [rsp+16*4], m0, 1
    _mm_storeu_si128(s.as_mut_ptr().cast::<__m128i>().add(0), lo(m[0])); // mova [rsp+16*0], xm0
    repx!([3, 5, 7, 9, 11, 13, 15], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m3, m5, m7, m9, m11, m13, m15
    m[1] = _mm256_mulhrs_epi16(m[1], s[1]); // pmulhrsw m1, [rsp+32*1]
    m[8] = _mm256_permute2x128_si256::<0x31>(m[1], m[9]); // vperm2i128 m8, m1, m9, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[9])); // vinserti128 m1, xm9, 1
    m[9] = _mm256_permute2x128_si256::<0x31>(m[2], m[10]); // vperm2i128 m9, m2, m10, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[10])); // vinserti128 m2, xm10, 1
    m[10] = _mm256_permute2x128_si256::<0x31>(m[3], m[11]); // vperm2i128 m10, m3, m11, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[11])); // vinserti128 m3, xm11, 1
    m[11] = _mm256_permute2x128_si256::<0x31>(m[4], m[12]); // vperm2i128 m11, m4, m12, 0x31
    m[4] = _mm256_inserti128_si256::<1>(m[4], lo(m[12])); // vinserti128 m4, xm12, 1
    m[12] = _mm256_permute2x128_si256::<0x31>(m[5], m[13]); // vperm2i128 m12, m5, m13, 0x31
    m[5] = _mm256_inserti128_si256::<1>(m[5], lo(m[13])); // vinserti128 m5, xm13, 1
    m[13] = _mm256_permute2x128_si256::<0x31>(m[6], m[14]); // vperm2i128 m13, m6, m14, 0x31
    m[6] = _mm256_inserti128_si256::<1>(m[6], lo(m[14])); // vinserti128 m6, xm14, 1
    m[14] = _mm256_permute2x128_si256::<0x31>(m[7], m[15]); // vperm2i128 m14, m7, m15, 0x31
    m[7] = _mm256_inserti128_si256::<1>(m[7], lo(m[15])); // vinserti128 m7, xm15, 1
    m[15] = s[2]; // mova m15, [rsp+32*2]
    idct_16x16_internal_pass1_end3(m, s)
}

/// `.pass1_end3` of `idct_16x16_internal_8bpc`: the transposition; ends in `jmp tx2q`
#[inline(always)]
unsafe fn idct_16x16_internal_pass1_end3(m: &mut M, s: &mut [__m256i; 3]) {
    m[0] = _mm256_unpacklo_epi16(m[9], m[10]); // punpcklwd m0, m9, m10
    m[9] = _mm256_unpackhi_epi16(m[9], m[10]); // punpckhwd m9, m10
    m[10] = _mm256_unpacklo_epi16(m[15], m[8]); // punpcklwd m10, m15, m8
    m[15] = _mm256_unpackhi_epi16(m[15], m[8]); // punpckhwd m15, m8
    m[8] = _mm256_unpackhi_epi16(m[11], m[12]); // punpckhwd m8, m11, m12
    m[11] = _mm256_unpacklo_epi16(m[11], m[12]); // punpcklwd m11, m12
    m[12] = _mm256_unpackhi_epi16(m[13], m[14]); // punpckhwd m12, m13, m14
    m[13] = _mm256_unpacklo_epi16(m[13], m[14]); // punpcklwd m13, m14
    m[14] = _mm256_unpackhi_epi32(m[11], m[13]); // punpckhdq m14, m11, m13
    m[11] = _mm256_unpacklo_epi32(m[11], m[13]); // punpckldq m11, m13
    m[13] = _mm256_unpacklo_epi32(m[15], m[9]); // punpckldq m13, m15, m9
    m[15] = _mm256_unpackhi_epi32(m[15], m[9]); // punpckhdq m15, m9
    m[9] = _mm256_unpacklo_epi32(m[10], m[0]); // punpckldq m9, m10, m0
    m[10] = _mm256_unpackhi_epi32(m[10], m[0]); // punpckhdq m10, m0
    m[0] = _mm256_unpackhi_epi32(m[8], m[12]); // punpckhdq m0, m8, m12
    m[8] = _mm256_unpacklo_epi32(m[8], m[12]); // punpckldq m8, m12
    m[12] = _mm256_unpacklo_epi64(m[13], m[8]); // punpcklqdq m12, m13, m8
    m[13] = _mm256_unpackhi_epi64(m[13], m[8]); // punpckhqdq m13, m8
    m[8] = _mm256_unpacklo_epi64(m[9], m[11]); // punpcklqdq m8, m9, m11
    m[9] = _mm256_unpackhi_epi64(m[9], m[11]); // punpckhqdq m9, m11
    m[11] = _mm256_unpackhi_epi64(m[10], m[14]); // punpckhqdq m11, m10, m14
    m[10] = _mm256_unpacklo_epi64(m[10], m[14]); // punpcklqdq m10, m14
    m[14] = _mm256_unpacklo_epi64(m[15], m[0]); // punpcklqdq m14, m15, m0
    m[15] = _mm256_unpackhi_epi64(m[15], m[0]); // punpckhqdq m15, m0
    m[0] = s[0]; // mova m0, [rsp]
    s[0] = m[15]; // mova [rsp], m15
    m[15] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m15, m4, m5
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
    m[5] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m5, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m1, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m7, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m2, m4, m6
    m[4] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m4, m6
    m[6] = _mm256_unpackhi_epi32(m[5], m[7]); // punpckhdq m6, m5, m7
    m[5] = _mm256_unpacklo_epi32(m[5], m[7]); // punpckldq m5, m7
    m[7] = _mm256_unpacklo_epi32(m[15], m[1]); // punpckldq m7, m15, m1
    m[15] = _mm256_unpackhi_epi32(m[15], m[1]); // punpckhdq m15, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
    m[4] = _mm256_unpacklo_epi64(m[5], m[7]); // punpcklqdq m4, m5, m7
    m[5] = _mm256_unpackhi_epi64(m[5], m[7]); // punpckhqdq m5, m7
    m[7] = _mm256_unpackhi_epi64(m[6], m[15]); // punpckhqdq m7, m6, m15
    m[6] = _mm256_unpacklo_epi64(m[6], m[15]); // punpcklqdq m6, m15
}

/// `.pass2` of `idct_16x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef, s: &mut [__m256i; 3]) {
    idct_16x16_internal_main(m, s); // call .main
    idct_16x16_internal_end(m, dst, stride, c, s)
}

/// `.end` of `idct_16x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x16_internal_end(m: &mut M, dst: Dst, stride: isize, c: Coef, s: &mut [__m256i; 3]) {
    m[1] = pw(2048); // vpbroadcastd m1, [o(pw_2048)]
    repx!([0, 2, 4, 6, 8, 10, 12, 14], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m0, m2, m4, m6, m8, m10, m12, m14
    s[0] = m[6]; // mova [rsp], m6
    idct_16x16_internal_end2(m, dst, stride, c, s)
}

/// `.end2` of `idct_16x16_internal_8bpc`
#[inline(always)]
unsafe fn idct_16x16_internal_end2(m: &mut M, mut dst: Dst, stride: isize, c: Coef, s: &mut [__m256i; 3]) {
    repx!([3, 5, 7, 9, 11, 13, 15], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m3, m5, m7, m9, m11, m13, m15
    m[1] = _mm256_mulhrs_epi16(m[1], s[1]); // pmulhrsw m1, [rsp+32*1]
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_16x2(m, dst, m[0], m[1], 6, 0, [0, stride]); // WRITE_16X2 0, 1, 6, 0, strideq*0, strideq*1
    write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r3]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r3
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_16x2(m, dst, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
    write_16x2(m, dst, s[0], m[7], 0, 1, [stride * 2, r3]); // WRITE_16X2 [rsp], 7, 0, 1, strideq*2, r3
    idct_16x16_internal_end3(m, dst, stride, c, r3)
}

/// `.end3` of `idct_16x16_internal_8bpc`: the lower eight rows and the clearing of the coefficients (`cq` is
/// the pointer that the first pass advanced by eight rows); `r3` is `strideq*3`.
#[inline(always)]
unsafe fn idct_16x16_internal_end3(m: &mut M, mut dst: Dst, stride: isize, c: Coef, r3: isize) {
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    repx!([-8, -7, -6, -5, -4, -3, -2, -1], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[2])); // REPX {mova [cq+32*x], m2}, -8, -7, -6, -5, -4, -3, -2, -1
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_16x2(m, dst, m[8], m[9], 0, 1, [0, stride]); // WRITE_16X2 8, 9, 0, 1, strideq*0, strideq*1
    write_16x2(m, dst, m[10], m[11], 0, 1, [stride * 2, r3]); // WRITE_16X2 10, 11, 0, 1, strideq*2, r3
    repx!([0, 1, 2, 3, 4, 5, 6, 7], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[2])); // REPX {mova [cq+32*x], m2}, 0, 1, 2, 3, 4, 5, 6, 7
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_16x2(m, dst, m[12], m[13], 0, 1, [0, stride]); // WRITE_16X2 12, 13, 0, 1, strideq*0, strideq*1
    write_16x2(m, dst, m[14], m[15], 0, 1, [stride * 2, r3]); // WRITE_16X2 14, 15, 0, 1, strideq*2, r3
}

/// `.main` of `idct_16x16_internal_8bpc`: `s` is the caller's frame (`[rsp+gprsize+32*0..2]`); slot 0 holds
/// in15 on entry, and out1 is returned in slot 1.
#[inline(always)]
pub(crate) unsafe fn idct_16x16_internal_main(m: &mut M, s: &mut [__m256i; 3]) {
    m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
    s[1] = m[1]; // mova [rsp+gprsize+32*1], m1
    s[2] = m[9]; // mova [rsp+gprsize+32*2], m9
    idct8_1d(m, [0, 2, 4, 6, 8, 10, 12, 14], [1, 9], 15); // IDCT8_1D 0, 2, 4, 6, 8, 10, 12, 14, 1, 9, 15
    m[1] = s[2]; // mova m1, [rsp+gprsize+32*2] ; in9
    s[2] = m[14]; // mova [rsp+gprsize+32*2], m14 ; tmp7
    m[9] = s[1]; // mova m9, [rsp+gprsize+32*1] ; in1
    s[1] = m[10]; // mova [rsp+gprsize+32*1], m10 ; tmp5
    m[14] = s[0]; // mova m14, [rsp+gprsize+32*0] ; in15
    s[0] = m[6]; // mova [rsp+gprsize+32*0], m6 ; tmp3
    idct16_1d_oddhalf(m, [9, 3, 5, 7, 1, 11, 13, 14], [6, 10], 15); // IDCT16_1D_ODDHALF 9, 3, 5, 7, 1, 11, 13, 14, 6, 10, 15
    m[6] = s[1]; // mova m6, [rsp+gprsize+32*1] ; tmp5
    m[15] = _mm256_subs_epi16(m[0], m[14]); // psubsw m15, m0, m14 ; out15
    m[0] = _mm256_adds_epi16(m[0], m[14]); // paddsw m0, m14 ; out0
    m[14] = _mm256_subs_epi16(m[2], m[13]); // psubsw m14, m2, m13 ; out14
    m[2] = _mm256_adds_epi16(m[2], m[13]); // paddsw m2, m13 ; out1
    s[1] = m[2]; // mova [rsp+gprsize+32*1], m2
    m[13] = _mm256_subs_epi16(m[4], m[11]); // psubsw m13, m4, m11 ; out13
    m[2] = _mm256_adds_epi16(m[4], m[11]); // paddsw m2, m4, m11 ; out2
    m[11] = _mm256_subs_epi16(m[8], m[7]); // psubsw m11, m8, m7 ; out11
    m[4] = _mm256_adds_epi16(m[8], m[7]); // paddsw m4, m8, m7 ; out4
    m[7] = s[2]; // mova m7, [rsp+gprsize+32*2] ; tmp7
    m[10] = _mm256_subs_epi16(m[6], m[5]); // psubsw m10, m6, m5 ; out10
    m[5] = _mm256_adds_epi16(m[5], m[6]); // paddsw m5, m6 ; out5
    m[8] = _mm256_subs_epi16(m[7], m[9]); // psubsw m8, m7, m9 ; out8
    m[7] = _mm256_adds_epi16(m[7], m[9]); // paddsw m7, m9 ; out7
    m[9] = _mm256_subs_epi16(m[12], m[3]); // psubsw m9, m12, m3 ; out9
    m[6] = _mm256_adds_epi16(m[12], m[3]); // paddsw m6, m12, m3 ; out6
    m[3] = s[0]; // mova m3, [rsp+gprsize+32*0] ; tmp3
    m[12] = _mm256_subs_epi16(m[3], m[1]); // psubsw m12, m3, m1 ; out12
    m[3] = _mm256_adds_epi16(m[3], m[1]); // paddsw m3, m1 ; out3
}

/// `cglobal iadst_16x16_internal_8bpc, 0, 5, 16, 32*3, dst, stride, c, eob, tx2` up to the jump into the
/// first pass's end of the DCT
#[inline(always)]
unsafe fn iadst_16x16_internal_pass1(m: &mut M, cq: &mut Coef, s: &mut [__m256i; 3]) {
    itx_16x16_load_coefs(m, cq, s); // ITX_16X16_LOAD_COEFS
    let c = *cq;
    iadst_16x16_internal_main(m, s); // call .main
    iadst_16x16_internal_main_pass1_end(m, c); // call .main_pass1_end
    m[0] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmulhrsw m0, m1, [cq+32*0]
    m[2] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulhrsw m2, m1, [cq+32*1]
    repx!([4, 6, 8, 10], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m4, m6, m8, m10
    m[12] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulhrsw m12, m1, [cq+32*2]
    m[14] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulhrsw m14, m1, [cq+32*3]
    _mm_storeu_si128(
        s.as_mut_ptr().cast::<__m128i>().add(5),
        _mm256_extracti128_si256::<1>(m[8]),
    ); // vextracti128 [rsp+16*5], m8, 1
    _mm_storeu_si128(s.as_mut_ptr().cast::<__m128i>().add(1), lo(m[8])); // mova [rsp+16*1], xm8
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    m[1] = _mm256_sub_epi16(m[8], m[1]); // psubw m1, m8, m1
    idct_16x16_internal_pass1_end2(m, s) // jmp m(idct_16x16_internal_8bpc).pass1_end2
}

/// `.pass2` of `iadst_16x16_internal_8bpc`
#[inline(always)]
unsafe fn iadst_16x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef, s: &mut [__m256i; 3]) {
    iadst_16x16_internal_main(m, s); // call .main
    iadst_16x16_internal_main_pass2_end(m); // call .main_pass2_end
    repx!([0, 2, 4, 6, 8, 10, 12, 14], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m0, m2, m4, m6, m8, m10, m12, m14
    s[0] = m[6]; // mova [rsp+32*0], m6
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    m[1] = _mm256_sub_epi16(m[6], m[1]); // psubw m1, m6, m1
    idct_16x16_internal_end2(m, dst, stride, c, s) // jmp m(idct_16x16_internal_8bpc).end2
}

/// `.main` of `iadst_16x16_internal_8bpc`: `s` is the caller's frame; slot 0 holds in15 on entry, and
/// -out1 is returned in slot 1.
#[inline(always)]
pub(crate) unsafe fn iadst_16x16_internal_main(m: &mut M, s: &mut [__m256i; 3]) {
    m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
    s[1] = m[0]; // mova [rsp+gprsize+32*1], m0
    s[2] = m[4]; // mova [rsp+gprsize+32*2], m4
    itx_mulsub_2w(m, 13, 2, 0, 4, 15, 995, 3973, None); // ITX_MULSUB_2W 13, 2, 0, 4, 15, 995, 3973 ; t3,  t2
    itx_mulsub_2w(m, 9, 6, 0, 4, 15, 2440, 3290, None); // ITX_MULSUB_2W 9, 6, 0, 4, 15, 2440, 3290 ; t7,  t6
    itx_mulsub_2w(m, 5, 10, 0, 4, 15, 3513, 2106, None); // ITX_MULSUB_2W 5, 10, 0, 4, 15, 3513, 2106 ; t11, t10
    itx_mulsub_2w(m, 1, 14, 0, 4, 15, 4052, 601, None); // ITX_MULSUB_2W 1, 14, 0, 4, 15, 4052, 601 ; t15, t14
    m[0] = _mm256_subs_epi16(m[2], m[10]); // psubsw m0, m2, m10 ; t10a
    m[2] = _mm256_adds_epi16(m[2], m[10]); // paddsw m2, m10 ; t2a
    m[10] = _mm256_subs_epi16(m[13], m[5]); // psubsw m10, m13, m5 ; t11a
    m[13] = _mm256_adds_epi16(m[13], m[5]); // paddsw m13, m5 ; t3a
    m[5] = _mm256_subs_epi16(m[6], m[14]); // psubsw m5, m6, m14 ; t14a
    m[6] = _mm256_adds_epi16(m[6], m[14]); // paddsw m6, m14 ; t6a
    m[14] = _mm256_subs_epi16(m[9], m[1]); // psubsw m14, m9, m1 ; t15a
    m[9] = _mm256_adds_epi16(m[9], m[1]); // paddsw m9, m1 ; t7a
    itx_mulsub_2w(m, 0, 10, 1, 4, 15, 3406, 2276, None); // ITX_MULSUB_2W 0, 10, 1, 4, 15, 3406, 2276 ; t11, t10
    itx_mulsub_2w(m, 14, 5, 1, 4, 15, 2276, 3406, None); // ITX_MULSUB_2W 14, 5, 1, 4, 15, 2276, 3406 ; t14, t15
    m[1] = _mm256_subs_epi16(m[10], m[14]); // psubsw m1, m10, m14 ; t14a
    m[10] = _mm256_adds_epi16(m[10], m[14]); // paddsw m10, m14 ; t10a
    m[14] = _mm256_subs_epi16(m[0], m[5]); // psubsw m14, m0, m5 ; t15a
    m[0] = _mm256_adds_epi16(m[0], m[5]); // paddsw m0, m5 ; t11a
    m[5] = _mm256_subs_epi16(m[2], m[6]); // psubsw m5, m2, m6 ; t6
    m[2] = _mm256_adds_epi16(m[2], m[6]); // paddsw m2, m6 ; t2
    m[6] = _mm256_subs_epi16(m[13], m[9]); // psubsw m6, m13, m9 ; t7
    m[13] = _mm256_adds_epi16(m[13], m[9]); // paddsw m13, m9 ; t3
    itx_mulsub_2w(m, 6, 5, 4, 9, 15, 3784, 1567, None); // ITX_MULSUB_2W 6, 5, 4, 9, 15, 3784, 1567 ; t6a, t7a
    itx_mulsub_2w(m, 14, 1, 4, 9, 15, 3784, 1567, None); // ITX_MULSUB_2W 14, 1, 4, 9, 15, 3784, 1567 ; t14, t15
    m[9] = s[0]; // mova m9, [rsp+gprsize+32*0] ; in15
    s[0] = m[10]; // mova [rsp+gprsize+32*0], m10 ; t10a
    m[4] = s[1]; // mova m4, [rsp+gprsize+32*1] ; in0
    s[1] = m[6]; // mova [rsp+gprsize+32*1], m6 ; t6a
    m[6] = s[2]; // mova m6, [rsp+gprsize+32*2] ; in4
    s[2] = m[2]; // mova [rsp+gprsize+32*2], m2 ; t2
    itx_mulsub_2w(m, 9, 4, 2, 10, 15, 201, 4091, None); // ITX_MULSUB_2W 9, 4, 2, 10, 15, 201, 4091 ; t1,  t0
    itx_mulsub_2w(m, 11, 6, 2, 10, 15, 1751, 3703, None); // ITX_MULSUB_2W 11, 6, 2, 10, 15, 1751, 3703 ; t5,  t4
    itx_mulsub_2w(m, 7, 8, 2, 10, 15, 3035, 2751, None); // ITX_MULSUB_2W 7, 8, 2, 10, 15, 3035, 2751 ; t9,  t8
    itx_mulsub_2w(m, 3, 12, 2, 10, 15, 3857, 1380, None); // ITX_MULSUB_2W 3, 12, 2, 10, 15, 3857, 1380 ; t13, t12
    m[10] = _mm256_subs_epi16(m[4], m[8]); // psubsw m10, m4, m8 ; t8a
    m[8] = _mm256_adds_epi16(m[8], m[4]); // paddsw m8, m4 ; t0a
    m[4] = _mm256_subs_epi16(m[9], m[7]); // psubsw m4, m9, m7 ; t9a
    m[9] = _mm256_adds_epi16(m[9], m[7]); // paddsw m9, m7 ; t1a
    m[7] = _mm256_subs_epi16(m[6], m[12]); // psubsw m7, m6, m12 ; t12a
    m[6] = _mm256_adds_epi16(m[6], m[12]); // paddsw m6, m12 ; t4a
    m[12] = _mm256_subs_epi16(m[11], m[3]); // psubsw m12, m11, m3 ; t13a
    m[11] = _mm256_adds_epi16(m[11], m[3]); // paddsw m11, m3 ; t5a
    itx_mulsub_2w(m, 10, 4, 2, 3, 15, 799, 4017, None); // ITX_MULSUB_2W 10, 4, 2, 3, 15, 799, 4017 ; t9,  t8
    itx_mulsub_2w(m, 12, 7, 2, 3, 15, 4017, 799, None); // ITX_MULSUB_2W 12, 7, 2, 3, 15, 4017, 799 ; t12, t13
    m[3] = _mm256_subs_epi16(m[9], m[11]); // psubsw m3, m9, m11 ; t5
    m[9] = _mm256_adds_epi16(m[9], m[11]); // paddsw m9, m11 ; t1
    m[11] = _mm256_subs_epi16(m[4], m[12]); // psubsw m11, m4, m12 ; t12a
    m[4] = _mm256_adds_epi16(m[4], m[12]); // paddsw m4, m12 ; t8a
    m[12] = _mm256_adds_epi16(m[8], m[6]); // paddsw m12, m8, m6 ; t0
    m[8] = _mm256_subs_epi16(m[8], m[6]); // psubsw m8, m6 ; t4
    m[6] = _mm256_adds_epi16(m[10], m[7]); // paddsw m6, m10, m7 ; t9a
    m[10] = _mm256_subs_epi16(m[10], m[7]); // psubsw m10, m7 ; t13a
    itx_mulsub_2w(m, 8, 3, 2, 7, 15, 1567, 3784, None); // ITX_MULSUB_2W 8, 3, 2, 7, 15, 1567, 3784 ; t5a, t4a
    itx_mulsub_2w(m, 11, 10, 2, 7, 15, 1567, 3784, None); // ITX_MULSUB_2W 11, 10, 2, 7, 15, 1567, 3784 ; t13, t12
    m[7] = s[0]; // mova m7, [rsp+gprsize+32*0] ; t10a
    m[2] = s[1]; // mova m2, [rsp+gprsize+32*1] ; t6a
    m[15] = _mm256_adds_epi16(m[9], m[13]); // paddsw m15, m9, m13 ; -out15
    m[9] = _mm256_subs_epi16(m[9], m[13]); // psubsw m9, m13 ; t3a
    m[13] = _mm256_adds_epi16(m[11], m[1]); // paddsw m13, m11, m1 ; -out13
    m[11] = _mm256_subs_epi16(m[11], m[1]); // psubsw m11, m1 ; t15a
    m[1] = _mm256_subs_epi16(m[4], m[7]); // psubsw m1, m4, m7 ; t10
    m[7] = _mm256_adds_epi16(m[7], m[4]); // paddsw m7, m4 ; -out1
    m[4] = _mm256_subs_epi16(m[3], m[2]); // psubsw m4, m3, m2 ; t6
    m[3] = _mm256_adds_epi16(m[3], m[2]); // paddsw m3, m2 ; -out3
    m[2] = _mm256_adds_epi16(m[10], m[14]); // paddsw m2, m10, m14 ; out2
    m[10] = _mm256_subs_epi16(m[10], m[14]); // psubsw m10, m14 ; t14a
    m[14] = _mm256_adds_epi16(m[6], m[0]); // paddsw m14, m6, m0 ; out14
    m[6] = _mm256_subs_epi16(m[6], m[0]); // psubsw m6, m0 ; t11
    m[0] = s[2]; // mova m0, [rsp+gprsize+32*2] ; t2
    s[1] = m[7]; // mova [rsp+gprsize+32*1], m7
    m[7] = _mm256_subs_epi16(m[12], m[0]); // psubsw m7, m12, m0 ; t2a
    m[0] = _mm256_adds_epi16(m[0], m[12]); // paddsw m0, m12 ; out0
    m[12] = _mm256_adds_epi16(m[8], m[5]); // paddsw m12, m8, m5 ; out12
    m[8] = _mm256_subs_epi16(m[8], m[5]); // psubsw m8, m5 ; t7
}

/// `.main_pass1_end` of `iadst_16x16_internal_8bpc`: parks out0, out2, out12 and out14 in the coefficient
/// rows 8-11 (`[cq+32*0..3]` of the advanced pointer).
#[inline(always)]
unsafe fn iadst_16x16_internal_main_pass1_end(m: &mut M, c: Coef) {
    _mm256_storeu_si256(at(c, 32 * 0).cast(), m[0]); // mova [cq+32*0], m0
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[2]); // mova [cq+32*1], m2
    _mm256_storeu_si256(at(c, 32 * 2).cast(), m[12]); // mova [cq+32*2], m12
    _mm256_storeu_si256(at(c, 32 * 3).cast(), m[14]); // mova [cq+32*3], m14
    m[14] = pw2(-2896, 2896); // vpbroadcastd m14, [pw_m2896_2896]
    m[12] = pw2(2896, 2896); // vpbroadcastd m12, [pw_2896_2896]
    m[2] = pd(2048); // vpbroadcastd m2, [pd_2048]
    m[5] = _mm256_unpacklo_epi16(m[11], m[10]); // punpcklwd m5, m11, m10
    m[11] = _mm256_unpackhi_epi16(m[11], m[10]); // punpckhwd m11, m10
    m[10] = _mm256_madd_epi16(m[14], m[5]); // pmaddwd m10, m14, m5
    m[0] = _mm256_madd_epi16(m[14], m[11]); // pmaddwd m0, m14, m11
    m[5] = _mm256_madd_epi16(m[5], m[12]); // pmaddwd m5, m12
    m[11] = _mm256_madd_epi16(m[11], m[12]); // pmaddwd m11, m12
    repx!([10, 0, 5, 11], i => m[i] = _mm256_add_epi32(m[i], m[2])); // REPX {paddd x, m2}, m10, m0, m5, m11
    repx!([10, 0, 5, 11], i => m[i] = _mm256_srai_epi32::<12>(m[i])); // REPX {psrad x, 12}, m10, m0, m5, m11
    m[10] = _mm256_packs_epi32(m[10], m[0]); // packssdw m10, m0 ; out10
    m[5] = _mm256_packs_epi32(m[5], m[11]); // packssdw m5, m11 ; -out5
    m[11] = _mm256_unpacklo_epi16(m[8], m[4]); // punpcklwd m11, m8, m4
    m[8] = _mm256_unpackhi_epi16(m[8], m[4]); // punpckhwd m8, m4
    m[4] = _mm256_madd_epi16(m[12], m[11]); // pmaddwd m4, m12, m11
    m[0] = _mm256_madd_epi16(m[12], m[8]); // pmaddwd m0, m12, m8
    m[11] = _mm256_madd_epi16(m[11], m[14]); // pmaddwd m11, m14
    m[8] = _mm256_madd_epi16(m[8], m[14]); // pmaddwd m8, m14
    repx!([4, 0, 11, 8], i => m[i] = _mm256_add_epi32(m[i], m[2])); // REPX {paddd x, m2}, m4, m0, m11, m8
    repx!([4, 0, 11, 8], i => m[i] = _mm256_srai_epi32::<12>(m[i])); // REPX {psrad x, 12}, m4, m0, m11, m8
    m[4] = _mm256_packs_epi32(m[4], m[0]); // packssdw m4, m0 ; out4
    m[11] = _mm256_packs_epi32(m[11], m[8]); // packssdw m11, m8 ; -out11
    m[8] = _mm256_unpacklo_epi16(m[9], m[7]); // punpcklwd m8, m9, m7
    m[9] = _mm256_unpackhi_epi16(m[9], m[7]); // punpckhwd m9, m7
    m[7] = _mm256_madd_epi16(m[12], m[8]); // pmaddwd m7, m12, m8
    m[0] = _mm256_madd_epi16(m[12], m[9]); // pmaddwd m0, m12, m9
    m[8] = _mm256_madd_epi16(m[8], m[14]); // pmaddwd m8, m14
    m[9] = _mm256_madd_epi16(m[9], m[14]); // pmaddwd m9, m14
    repx!([7, 0, 8, 9], i => m[i] = _mm256_add_epi32(m[i], m[2])); // REPX {paddd x, m2}, m7, m0, m8, m9
    repx!([7, 0, 8, 9], i => m[i] = _mm256_srai_epi32::<12>(m[i])); // REPX {psrad x, 12}, m7, m0, m8, m9
    m[7] = _mm256_packs_epi32(m[7], m[0]); // packssdw m7, m0 ; -out7
    m[8] = _mm256_packs_epi32(m[8], m[9]); // packssdw m8, m9 ; out8
    m[0] = _mm256_unpackhi_epi16(m[6], m[1]); // punpckhwd m0, m6, m1
    m[6] = _mm256_unpacklo_epi16(m[6], m[1]); // punpcklwd m6, m1
    m[1] = _mm256_madd_epi16(m[14], m[0]); // pmaddwd m1, m14, m0
    m[9] = _mm256_madd_epi16(m[14], m[6]); // pmaddwd m9, m14, m6
    m[0] = _mm256_madd_epi16(m[0], m[12]); // pmaddwd m0, m12
    m[6] = _mm256_madd_epi16(m[6], m[12]); // pmaddwd m6, m12
    repx!([1, 9, 0, 6], i => m[i] = _mm256_add_epi32(m[i], m[2])); // REPX {paddd x, m2}, m1, m9, m0, m6
    repx!([1, 9, 0, 6], i => m[i] = _mm256_srai_epi32::<12>(m[i])); // REPX {psrad x, 12}, m1, m9, m0, m6
    m[9] = _mm256_packs_epi32(m[9], m[1]); // packssdw m9, m1 ; -out7
    m[6] = _mm256_packs_epi32(m[6], m[0]); // packssdw m6, m0 ; out8
    m[1] = pw(8192); // vpbroadcastd m1, [o(pw_8192)]
}

/// `.main_pass2_end` of `iadst_16x16_internal_8bpc`
#[inline(always)]
pub(crate) unsafe fn iadst_16x16_internal_main_pass2_end(m: &mut M) {
    // In pass 2 we're going to clip to pixels afterwards anyway, so clipping to
    // 16-bit here will produce the same result as using 32-bit intermediates.
    m[5] = _mm256_adds_epi16(m[10], m[11]); // paddsw m5, m10, m11 ; -out5
    m[10] = _mm256_subs_epi16(m[10], m[11]); // psubsw m10, m11 ; out10
    m[11] = _mm256_subs_epi16(m[4], m[8]); // psubsw m11, m4, m8 ; -out11
    m[4] = _mm256_adds_epi16(m[4], m[8]); // paddsw m4, m8 ; out4
    m[8] = _mm256_subs_epi16(m[7], m[9]); // psubsw m8, m7, m9 ; out8
    m[7] = _mm256_adds_epi16(m[7], m[9]); // paddsw m7, m9 ; -out7
    m[9] = _mm256_subs_epi16(m[1], m[6]); // psubsw m9, m1, m6 ; -out9
    m[6] = _mm256_adds_epi16(m[6], m[1]); // paddsw m6, m1 ; out6
    m[1] = pw(2896 * 8); // vpbroadcastd m1, [o(pw_2896x8)]
    repx!([4, 5, 6, 7, 8, 9, 10, 11], i => m[i] = _mm256_mulhrs_epi16(m[i], m[1])); // REPX {pmulhrsw x, m1}, m4, m5, m6, m7, m8, m9, m10, m11
    m[1] = pw(2048); // vpbroadcastd m1, [o(pw_2048)]
}

/// `cglobal iflipadst_16x16_internal_8bpc, 0, 5, 16, 32*3, dst, stride, c, eob, tx2` up to the jump into the
/// first pass's end of the DCT
#[inline(always)]
unsafe fn iflipadst_16x16_internal_pass1(m: &mut M, cq: &mut Coef, s: &mut [__m256i; 3]) {
    itx_16x16_load_coefs(m, cq, s); // ITX_16X16_LOAD_COEFS
    let c = *cq;
    iadst_16x16_internal_main(m, s); // call m(iadst_16x16_internal_8bpc).main
    iadst_16x16_internal_main_pass1_end(m, c); // call m(iadst_16x16_internal_8bpc).main_pass1_end
    m[6] = _mm256_mulhrs_epi16(m[6], m[1]); // pmulhrsw m6, m1
    m[2] = _mm256_mulhrs_epi16(m[1], m[8]); // pmulhrsw m2, m1, m8
    s[2] = m[6]; // mova [rsp+32*2], m6
    m[6] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m6, m1, m4
    m[4] = _mm256_mulhrs_epi16(m[1], m[10]); // pmulhrsw m4, m1, m10
    m[8] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulhrsw m8, m1, [cq+32*3]
    m[10] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulhrsw m10, m1, [cq+32*2]
    m[12] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulhrsw m12, m1, [cq+32*1]
    m[14] = _mm256_mulhrs_epi16(m[1], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmulhrsw m14, m1, [cq+32*0]
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    m[0] = _mm256_sub_epi16(m[0], m[1]); // psubw m0, m1
    repx!([3, 5, 7, 11, 15], i => m[i] = _mm256_mulhrs_epi16(m[i], m[0])); // REPX {pmulhrsw x, m0}, m3, m5, m7, m11, m15
    m[1] = _mm256_mulhrs_epi16(m[0], m[9]); // pmulhrsw m1, m0, m9
    m[9] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m9, m0, m13
    m[0] = _mm256_mulhrs_epi16(m[0], s[1]); // pmulhrsw m0, [rsp+32*1]
    _mm_storeu_si128(s.as_mut_ptr().cast::<__m128i>().add(0), lo(m[15])); // mova [rsp+16*0], xm15
    _mm_storeu_si128(s.as_mut_ptr().cast::<__m128i>().add(1), lo(m[7])); // mova [rsp+16*1], xm7
    m[15] = _mm256_permute2x128_si256::<0x31>(m[15], m[7]); // vperm2i128 m15, m15, m7, 0x31
    m[7] = _mm256_inserti128_si256::<1>(m[2], lo(m[14])); // vinserti128 m7, m2, xm14, 1
    m[14] = _mm256_permute2x128_si256::<0x31>(m[2], m[14]); // vperm2i128 m14, m2, m14, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[9], lo(m[5])); // vinserti128 m2, m9, xm5, 1
    m[9] = _mm256_permute2x128_si256::<0x31>(m[9], m[5]); // vperm2i128 m9, m9, m5, 0x31
    m[5] = _mm256_inserti128_si256::<1>(m[4], lo(m[12])); // vinserti128 m5, m4, xm12, 1
    m[12] = _mm256_permute2x128_si256::<0x31>(m[4], m[12]); // vperm2i128 m12, m4, m12, 0x31
    m[4] = _mm256_inserti128_si256::<1>(m[11], lo(m[3])); // vinserti128 m4, m11, xm3, 1
    m[11] = _mm256_permute2x128_si256::<0x31>(m[11], m[3]); // vperm2i128 m11, m11, m3, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[10], lo(m[6])); // vinserti128 m3, m10, xm6, 1
    m[10] = _mm256_permute2x128_si256::<0x31>(m[10], m[6]); // vperm2i128 m10, m10, m6, 0x31
    m[6] = _mm256_inserti128_si256::<1>(m[1], lo(m[0])); // vinserti128 m6, m1, xm0, 1
    m[13] = _mm256_permute2x128_si256::<0x31>(m[1], m[0]); // vperm2i128 m13, m1, m0, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[8], lo(s[2])); // vinserti128 m1, m8, [rsp+32*2], 1
    m[8] = _mm256_permute2x128_si256::<0x31>(m[8], s[2]); // vperm2i128 m8, m8, [rsp+32*2], 0x31
    idct_16x16_internal_pass1_end3(m, s) // jmp m(idct_16x16_internal_8bpc).pass1_end3
}

/// `.pass2` of `iflipadst_16x16_internal_8bpc`
#[inline(always)]
unsafe fn iflipadst_16x16_internal_pass2(m: &mut M, mut dst: Dst, stride: isize, c: Coef, s: &mut [__m256i; 3]) {
    iadst_16x16_internal_main(m, s); // call m(iadst_16x16_internal_8bpc).main
    iadst_16x16_internal_main_pass2_end(m); // call m(iadst_16x16_internal_8bpc).main_pass2_end
    m[0] = _mm256_mulhrs_epi16(m[0], m[1]); // pmulhrsw m0, m1
    m[8] = _mm256_mulhrs_epi16(m[8], m[1]); // pmulhrsw m8, m1
    s[0] = m[0]; // mova [rsp+32*0], m0
    s[2] = m[8]; // mova [rsp+32*2], m8
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    m[0] = _mm256_sub_epi16(m[0], m[1]); // psubw m0, m1
    m[8] = _mm256_mulhrs_epi16(m[0], m[7]); // pmulhrsw m8, m0, m7
    m[7] = _mm256_mulhrs_epi16(m[0], m[9]); // pmulhrsw m7, m0, m9
    m[9] = _mm256_mulhrs_epi16(m[1], m[6]); // pmulhrsw m9, m1, m6
    m[6] = _mm256_mulhrs_epi16(m[1], m[10]); // pmulhrsw m6, m1, m10
    m[10] = _mm256_mulhrs_epi16(m[0], m[5]); // pmulhrsw m10, m0, m5
    m[5] = _mm256_mulhrs_epi16(m[0], m[11]); // pmulhrsw m5, m0, m11
    m[11] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m11, m1, m4
    m[4] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m4, m1, m12
    m[12] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m12, m0, m3
    m[3] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m3, m0, m13
    m[13] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m13, m1, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[14]); // pmulhrsw m1, m14
    m[14] = _mm256_mulhrs_epi16(m[0], s[1]); // pmulhrsw m14, m0, [rsp+32*1]
    m[0] = _mm256_mulhrs_epi16(m[0], m[15]); // pmulhrsw m0, m15
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_16x2(m, dst, m[0], m[1], 2, 0, [0, stride]); // WRITE_16X2 0, 1, 2, 0, strideq*0, strideq*1
    m[15] = s[0]; // mova m15, [rsp+32*0]
    write_16x2(m, dst, m[3], m[4], 0, 1, [stride * 2, r3]); // WRITE_16X2 3, 4, 0, 1, strideq*2, r3
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_16x2(m, dst, m[5], m[6], 0, 1, [0, stride]); // WRITE_16X2 5, 6, 0, 1, strideq*0, strideq*1
    write_16x2(m, dst, m[7], s[2], 0, 1, [stride * 2, r3]); // WRITE_16X2 7, [rsp+32*2], 0, 1, strideq*2, r3
    idct_16x16_internal_end3(m, dst, stride, c, r3) // jmp m(idct_16x16_internal_8bpc).end3
}

/// `%macro IDTX16B src/dst, tmp, pw_1697x16`
#[inline(always)]
unsafe fn idtx16b(m: &mut M, r: usize, t: usize, c: usize) {
    m[t] = _mm256_mulhrs_epi16(m[c], m[r]); // pmulhrsw m%2, m%3, m%1
    m[t] = _mm256_srai_epi16::<1>(m[t]); // psraw m%2, 1
    m[r] = _mm256_avg_epu16(m[r], m[t]); // pavgw m%1, m%2 ; signs are guaranteed to be equal
}

/// `cglobal iidentity_16x16_internal_8bpc, 0, 5, 16, 32*3, dst, stride, c, eob, tx2` up to the jump into
/// the first pass's end of the DCT
#[inline(always)]
unsafe fn iidentity_16x16_internal_pass1(m: &mut M, cq: &mut Coef, s: &mut [__m256i; 3]) {
    let mut c = *cq;
    m[7] = pw(1697 * 16); // vpbroadcastd m7, [o(pw_1697x16)]
    m[0] = zx(_mm_loadu_si128(at(c, 16 * 0).cast())); // mova xm0, [cq+16* 0]
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 16 * 16).cast())); // vinserti128 m0, [cq+16*16], 1
    m[15] = zx(_mm_loadu_si128(at(c, 16 * 1).cast())); // mova xm15, [cq+16* 1]
    m[15] = _mm256_inserti128_si256::<1>(m[15], _mm_loadu_si128(at(c, 16 * 17).cast())); // vinserti128 m15, [cq+16*17], 1
    m[1] = zx(_mm_loadu_si128(at(c, 16 * 2).cast())); // mova xm1, [cq+16* 2]
    m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 16 * 18).cast())); // vinserti128 m1, [cq+16*18], 1
    m[8] = zx(_mm_loadu_si128(at(c, 16 * 3).cast())); // mova xm8, [cq+16* 3]
    m[8] = _mm256_inserti128_si256::<1>(m[8], _mm_loadu_si128(at(c, 16 * 19).cast())); // vinserti128 m8, [cq+16*19], 1
    m[2] = zx(_mm_loadu_si128(at(c, 16 * 4).cast())); // mova xm2, [cq+16* 4]
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, 16 * 20).cast())); // vinserti128 m2, [cq+16*20], 1
    m[9] = zx(_mm_loadu_si128(at(c, 16 * 5).cast())); // mova xm9, [cq+16* 5]
    m[9] = _mm256_inserti128_si256::<1>(m[9], _mm_loadu_si128(at(c, 16 * 21).cast())); // vinserti128 m9, [cq+16*21], 1
    m[3] = zx(_mm_loadu_si128(at(c, 16 * 6).cast())); // mova xm3, [cq+16* 6]
    m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, 16 * 22).cast())); // vinserti128 m3, [cq+16*22], 1
    m[10] = zx(_mm_loadu_si128(at(c, 16 * 7).cast())); // mova xm10, [cq+16* 7]
    c = at(c, 16 * 16); // add cq, 16*16
    m[10] = _mm256_inserti128_si256::<1>(m[10], _mm_loadu_si128(at(c, 16 * 7).cast())); // vinserti128 m10, [cq+16* 7], 1
    m[4] = zx(_mm_loadu_si128(at(c, -16 * 8).cast())); // mova xm4, [cq-16* 8]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 16 * 8).cast())); // vinserti128 m4, [cq+16* 8], 1
    m[11] = zx(_mm_loadu_si128(at(c, -16 * 7).cast())); // mova xm11, [cq-16* 7]
    m[11] = _mm256_inserti128_si256::<1>(m[11], _mm_loadu_si128(at(c, 16 * 9).cast())); // vinserti128 m11, [cq+16* 9], 1
    m[5] = zx(_mm_loadu_si128(at(c, -16 * 6).cast())); // mova xm5, [cq-16* 6]
    m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(c, 16 * 10).cast())); // vinserti128 m5, [cq+16*10], 1
    m[12] = zx(_mm_loadu_si128(at(c, -16 * 5).cast())); // mova xm12, [cq-16* 5]
    m[12] = _mm256_inserti128_si256::<1>(m[12], _mm_loadu_si128(at(c, 16 * 11).cast())); // vinserti128 m12, [cq+16*11], 1
    m[13] = zx(_mm_loadu_si128(at(c, -16 * 3).cast())); // mova xm13, [cq-16* 3]
    m[13] = _mm256_inserti128_si256::<1>(m[13], _mm_loadu_si128(at(c, 16 * 13).cast())); // vinserti128 m13, [cq+16*13], 1
    m[14] = zx(_mm_loadu_si128(at(c, -16 * 1).cast())); // mova xm14, [cq-16* 1]
    m[14] = _mm256_inserti128_si256::<1>(m[14], _mm_loadu_si128(at(c, 16 * 15).cast())); // vinserti128 m14, [cq+16*15], 1
    repx!([0, 15, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 13, 14], i => idtx16b(m, i, 6, 7)); // REPX {IDTX16B x, 6, 7}, 0, 15, 1, 8, 2, 9, 3, 10, 4, 11, 5, 12, 13, 14
    m[6] = zx(_mm_loadu_si128(at(c, -16 * 4).cast())); // mova xm6, [cq-16* 4]
    m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(c, 16 * 12).cast())); // vinserti128 m6, [cq+16*12], 1
    s[0] = m[0]; // mova [rsp], m0
    idtx16b(m, 6, 0, 7); // IDTX16B 6, 0, 7
    m[0] = zx(_mm_loadu_si128(at(c, -16 * 2).cast())); // mova xm0, [cq-16* 2]
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 16 * 14).cast())); // vinserti128 m0, [cq+16*14], 1
    m[7] = _mm256_mulhrs_epi16(m[7], m[0]); // pmulhrsw m7, m0
    m[7] = _mm256_srai_epi16::<1>(m[7]); // psraw m7, 1
    m[7] = _mm256_avg_epu16(m[7], m[0]); // pavgw m7, m0
    *cq = c;
    idct_16x16_internal_pass1_end3(m, s) // jmp m(idct_16x16_internal_8bpc).pass1_end3
}

/// `.pass2` of `iidentity_16x16_internal_8bpc`
#[inline(always)]
unsafe fn iidentity_16x16_internal_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef, s: &mut [__m256i; 3]) {
    m[15] = pw(1697 * 16); // vpbroadcastd m15, [o(pw_1697x16)]
    s[1] = m[0]; // mova [rsp+32*1], m0
    repx!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14], i => idtx16(m, i, 0, 15, None)); // REPX {IDTX16 x, 0, 15}, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
    m[0] = s[1]; // mova m0, [rsp+32*1]
    s[1] = m[1]; // mova [rsp+32*1], m1
    idtx16(m, 0, 1, 15, None); // IDTX16 0, 1, 15
    m[1] = s[0]; // mova m1, [rsp+32*0]
    m[15] = _mm256_mulhrs_epi16(m[15], m[1]); // pmulhrsw m15, m1
    m[1] = _mm256_adds_epi16(m[1], m[1]); // paddsw m1, m1
    m[15] = _mm256_adds_epi16(m[15], m[1]); // paddsw m15, m1
    idct_16x16_internal_end(m, dst, stride, c, s) // jmp m(idct_16x16_internal_8bpc).end
}

/// `%macro INV_TXFM_16X16_FN type1, type2`: `INV_TXFM_FN type1, type2, 16x16` with the DC-only path of
/// dct_dct, then the first pass and (`jmp tx2q`) the second.
#[inline(always)]
unsafe fn inv_txfm_add_16x16(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 16; // or r3d, 16
        return inv_txfm_add_dct_dct_16x4_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_16x4_8bpc).dconly
    }
    // The frame of the `_internal` functions (`cglobal …, 0, 5, 16, 32*3`): three 32-byte slots that both
    // passes use. Zeroed whole; every path writes a slot before it reads it.
    let mut s = [_mm256_setzero_si256(); 3];
    let mut c = c;
    match t1 {
        K::Dct => idct_16x16_internal_pass1(m, &mut c, &mut s),
        K::Adst => iadst_16x16_internal_pass1(m, &mut c, &mut s),
        K::FlipAdst => iflipadst_16x16_internal_pass1(m, &mut c, &mut s),
        K::Identity => iidentity_16x16_internal_pass1(m, &mut c, &mut s),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x16_internal_pass2(m, dst, stride, c, &mut s),
        K::Adst => iadst_16x16_internal_pass2(m, dst, stride, c, &mut s),
        K::FlipAdst => iflipadst_16x16_internal_pass2(m, dst, stride, c, &mut s),
        K::Identity => iidentity_16x16_internal_pass2(m, dst, stride, c, &mut s),
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x16_8bpc_avx2,
    inv_txfm_add_16x16,
    K::Identity,
    K::Identity
);

// 8x32 and 32x8

/// `%macro LOAD_8ROWS src, stride, is_rect2`: eight rows, `stride` bytes apart, from `src` into `m0`-`m7`
/// (with `is_rect2` scaled by `pw_2896x8`, which stays in `m15`).
#[inline(always)]
unsafe fn load_8rows<T>(m: &mut M, src: *mut T, stride: isize, is_rect2: u32) {
    if is_rect2 != 0 {
        m[15] = pw(2896 * 8); // vpbroadcastd m15, [o(pw_2896x8)]
        m[0] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 0).cast())); // pmulhrsw m0, m15, [%1+%2*0]
        m[1] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 1).cast())); // pmulhrsw m1, m15, [%1+%2*1]
        m[2] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 2).cast())); // pmulhrsw m2, m15, [%1+%2*2]
        m[3] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 3).cast())); // pmulhrsw m3, m15, [%1+%2*3]
        m[4] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 4).cast())); // pmulhrsw m4, m15, [%1+%2*4]
        m[5] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 5).cast())); // pmulhrsw m5, m15, [%1+%2*5]
        m[6] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 6).cast())); // pmulhrsw m6, m15, [%1+%2*6]
        m[7] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 7).cast()));
    // pmulhrsw m7, m15, [%1+%2*7]
    } else {
        m[0] = _mm256_loadu_si256(at(src, stride * 0).cast()); // mova m0, [%1+%2*0]
        m[1] = _mm256_loadu_si256(at(src, stride * 1).cast()); // mova m1, [%1+%2*1]
        m[2] = _mm256_loadu_si256(at(src, stride * 2).cast()); // mova m2, [%1+%2*2]
        m[3] = _mm256_loadu_si256(at(src, stride * 3).cast()); // mova m3, [%1+%2*3]
        m[4] = _mm256_loadu_si256(at(src, stride * 4).cast()); // mova m4, [%1+%2*4]
        m[5] = _mm256_loadu_si256(at(src, stride * 5).cast()); // mova m5, [%1+%2*5]
        m[6] = _mm256_loadu_si256(at(src, stride * 6).cast()); // mova m6, [%1+%2*6]
        m[7] = _mm256_loadu_si256(at(src, stride * 7).cast()); // mova m7, [%1+%2*7]
    }
}

/// `%macro LOAD_8ROWS_H src, stride, is_rect2`: eight rows, `stride` bytes apart, from `src` into `m8`-`m15`
/// (with `is_rect2` scaled by `pw_2896x8`: 1 loads the constant into `m15`, any other nonzero value uses
/// the one that is there).
#[inline(always)]
unsafe fn load_8rows_h<T>(m: &mut M, src: *mut T, stride: isize, is_rect2: u32) {
    if is_rect2 != 0 {
        if is_rect2 == 1 {
            m[15] = pw(2896 * 8); // vpbroadcastd m15, [o(pw_2896x8)]
        }
        m[8] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 0).cast())); // pmulhrsw m8, m15, [%1+%2*0]
        m[9] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 1).cast())); // pmulhrsw m9, m15, [%1+%2*1]
        m[10] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 2).cast())); // pmulhrsw m10, m15, [%1+%2*2]
        m[11] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 3).cast())); // pmulhrsw m11, m15, [%1+%2*3]
        m[12] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 4).cast())); // pmulhrsw m12, m15, [%1+%2*4]
        m[13] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 5).cast())); // pmulhrsw m13, m15, [%1+%2*5]
        m[14] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 6).cast())); // pmulhrsw m14, m15, [%1+%2*6]
        m[15] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 7).cast()));
    // pmulhrsw m15, [%1+%2*7]
    } else {
        m[8] = _mm256_loadu_si256(at(src, stride * 0).cast()); // mova m8, [%1+%2*0]
        m[9] = _mm256_loadu_si256(at(src, stride * 1).cast()); // mova m9, [%1+%2*1]
        m[10] = _mm256_loadu_si256(at(src, stride * 2).cast()); // mova m10, [%1+%2*2]
        m[11] = _mm256_loadu_si256(at(src, stride * 3).cast()); // mova m11, [%1+%2*3]
        m[12] = _mm256_loadu_si256(at(src, stride * 4).cast()); // mova m12, [%1+%2*4]
        m[13] = _mm256_loadu_si256(at(src, stride * 5).cast()); // mova m13, [%1+%2*5]
        m[14] = _mm256_loadu_si256(at(src, stride * 6).cast()); // mova m14, [%1+%2*6]
        m[15] = _mm256_loadu_si256(at(src, stride * 7).cast()); // mova m15, [%1+%2*7]
    }
}

/// `.transpose8x8` of `inv_txfm_add_identity_identity_8x32_8bpc`: transposes the 8x8 words of each lane of
/// `m0`-`m7` (temporary: `m8`).
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32_transpose8x8(m: &mut M) {
    m[8] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m8, m4, m5
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
    m[5] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m5, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m1, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m7, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m2, m4, m6
    m[4] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m4, m6
    m[6] = _mm256_unpackhi_epi32(m[5], m[7]); // punpckhdq m6, m5, m7
    m[5] = _mm256_unpacklo_epi32(m[5], m[7]); // punpckldq m5, m7
    m[7] = _mm256_unpacklo_epi32(m[8], m[1]); // punpckldq m7, m8, m1
    m[8] = _mm256_unpackhi_epi32(m[8], m[1]); // punpckhdq m8, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
    m[4] = _mm256_unpacklo_epi64(m[5], m[7]); // punpcklqdq m4, m5, m7
    m[5] = _mm256_unpackhi_epi64(m[5], m[7]); // punpckhqdq m5, m7
    m[7] = _mm256_unpackhi_epi64(m[6], m[8]); // punpckhqdq m7, m6, m8
    m[6] = _mm256_unpacklo_epi64(m[6], m[8]); // punpcklqdq m6, m8
}

/// `%macro ITX_UNPACK_MULHRSW dst1, dst2/src, tmp, coef[1-4]` (the constants are `pw_<coef1>_<coef2>x8` and
/// `pw_<coef3>_<coef4>x8`, which the assembly addresses through `r5`)
#[inline(always)]
unsafe fn itx_unpack_mulhrsw(m: &mut M, d1: usize, d2: usize, t: usize, c1: i32, c2: i32, c3: i32, c4: i32) {
    m[t] = pw2(c1 * 8, c2 * 8); // vpbroadcastd m%3, [r5-pw_201_4091x8+pw_%4_%5x8]
    m[d1] = _mm256_unpacklo_epi16(m[d2], m[d2]); // punpcklwd m%1, m%2, m%2
    m[d1] = _mm256_mulhrs_epi16(m[d1], m[t]); // pmulhrsw m%1, m%3
    m[t] = pw2(c3 * 8, c4 * 8); // vpbroadcastd m%3, [r5-pw_201_4091x8+pw_%6_%7x8]
    m[d2] = _mm256_unpackhi_epi16(m[d2], m[d2]); // punpckhwd m%2, m%2
    m[d2] = _mm256_mulhrs_epi16(m[d2], m[t]); // pmulhrsw m%2, m%3
}

/// `cglobal inv_txfm_add_dct_dct_8x32_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 32; // or r3d, 32
        return inv_txfm_add_dct_dct_8x8_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_8x8_8bpc).dconly
    }
    // PROLOGUE 0, 4, 16, 32*3, dst, stride, c, eob: a frame of three 32-byte slots. Zeroed whole; both the
    // path of a large eob and `.fast` store all three before `.main`/`.main_fast` read them.
    let mut s = [_mm256_setzero_si256(); 3];
    let s = &mut s;
    if eob > 106 {
        // cmp eobd, 106 / jle .fast
        load_8rows(m, at(c, 32 * 1), 32 * 2, 0); // LOAD_8ROWS cq+32*1, 32*2
        idct_16x8_internal_main(m); // call m(idct_16x8_internal_8bpc).main
        m[11] = _mm256_permute2x128_si256::<0x31>(m[0], m[4]); // vperm2i128 m11, m0, m4, 0x31
        m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m0, xm4, 1
        m[4] = _mm256_permute2x128_si256::<0x31>(m[1], m[5]); // vperm2i128 m4, m1, m5, 0x31
        m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[5])); // vinserti128 m1, xm5, 1
        m[5] = _mm256_permute2x128_si256::<0x31>(m[2], m[6]); // vperm2i128 m5, m2, m6, 0x31
        m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[6])); // vinserti128 m2, xm6, 1
        m[6] = _mm256_permute2x128_si256::<0x31>(m[3], m[7]); // vperm2i128 m6, m3, m7, 0x31
        m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[7])); // vinserti128 m3, xm7, 1
        m[7] = _mm256_setzero_si256(); // pxor m7, m7
        repx!([1, 3, 5, 7, 9, 11, 13, 15], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[7])); // REPX {mova [cq+32*x], m7}, 1, 3, 5, 7, 9, 11, 13, 15
        m[7] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m7, m0, m1
        m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
        m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
        m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
        m[3] = _mm256_unpacklo_epi16(m[11], m[4]); // punpcklwd m3, m11, m4
        m[11] = _mm256_unpackhi_epi16(m[11], m[4]); // punpckhwd m11, m4
        m[4] = _mm256_unpackhi_epi16(m[5], m[6]); // punpckhwd m4, m5, m6
        m[5] = _mm256_unpacklo_epi16(m[5], m[6]); // punpcklwd m5, m6
        m[6] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m6, m0, m2
        m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
        m[2] = _mm256_unpacklo_epi32(m[3], m[5]); // punpckldq m2, m3, m5
        m[3] = _mm256_unpackhi_epi32(m[3], m[5]); // punpckhdq m3, m5
        m[5] = _mm256_unpackhi_epi32(m[11], m[4]); // punpckhdq m5, m11, m4
        m[11] = _mm256_unpacklo_epi32(m[11], m[4]); // punpckldq m11, m4
        m[4] = _mm256_unpacklo_epi32(m[7], m[1]); // punpckldq m4, m7, m1
        m[7] = _mm256_unpackhi_epi32(m[7], m[1]); // punpckhdq m7, m1
        m[12] = _mm256_unpackhi_epi64(m[6], m[0]); // punpckhqdq m12, m6, m0
        m[0] = _mm256_unpacklo_epi64(m[0], m[6]); // punpcklqdq m0, m6 ; out4
        m[13] = _mm256_unpackhi_epi64(m[7], m[4]); // punpckhqdq m13, m7, m4
        m[4] = _mm256_unpacklo_epi64(m[4], m[7]); // punpcklqdq m4, m7 ; out5
        m[14] = _mm256_unpackhi_epi64(m[3], m[2]); // punpckhqdq m14, m3, m2
        m[2] = _mm256_unpacklo_epi64(m[2], m[3]); // punpcklqdq m2, m3 ; out6
        m[15] = _mm256_unpackhi_epi64(m[5], m[11]); // punpckhqdq m15, m5, m11
        m[11] = _mm256_unpacklo_epi64(m[11], m[5]); // punpcklqdq m11, m5 ; out7
        s[0] = m[0]; // mova [rsp+32*0], m0
        s[1] = m[4]; // mova [rsp+32*1], m4
        s[2] = m[2]; // mova [rsp+32*2], m2
    }
    // .fast:
    load_8rows(m, at(c, 32 * 0), 32 * 2, 0); // LOAD_8ROWS cq+32*0, 32*2
    idct_16x8_internal_main(m); // call m(idct_16x8_internal_8bpc).main
    m[8] = _mm256_permute2x128_si256::<0x31>(m[0], m[4]); // vperm2i128 m8, m0, m4, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m0, xm4, 1
    m[4] = _mm256_permute2x128_si256::<0x31>(m[1], m[5]); // vperm2i128 m4, m1, m5, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[5])); // vinserti128 m1, xm5, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[2], m[6]); // vperm2i128 m5, m2, m6, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[6])); // vinserti128 m2, xm6, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[3], m[7]); // vperm2i128 m6, m3, m7, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[7])); // vinserti128 m3, xm7, 1
    m[9] = pw(8192); // vpbroadcastd m9, [o(pw_8192)]
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    repx!([0, 2, 4, 6, 8, 10, 12, 14], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[7])); // REPX {mova [cq+32*x], m7}, 0, 2, 4, 6, 8, 10, 12, 14
    m[7] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m7, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[8], m[4]); // punpckhwd m3, m8, m4
    m[8] = _mm256_unpacklo_epi16(m[8], m[4]); // punpcklwd m8, m4
    m[4] = _mm256_unpackhi_epi16(m[5], m[6]); // punpckhwd m4, m5, m6
    m[5] = _mm256_unpacklo_epi16(m[5], m[6]); // punpcklwd m5, m6
    m[6] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m6, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[8], m[5]); // punpckldq m2, m8, m5
    m[8] = _mm256_unpackhi_epi32(m[8], m[5]); // punpckhdq m8, m5
    m[5] = _mm256_unpackhi_epi32(m[3], m[4]); // punpckhdq m5, m3, m4
    m[3] = _mm256_unpacklo_epi32(m[3], m[4]); // punpckldq m3, m4
    m[4] = _mm256_unpackhi_epi32(m[7], m[1]); // punpckhdq m4, m7, m1
    m[7] = _mm256_unpacklo_epi32(m[7], m[1]); // punpckldq m7, m1
    m[1] = _mm256_unpacklo_epi64(m[7], m[4]); // punpcklqdq m1, m7, m4
    m[7] = _mm256_unpackhi_epi64(m[7], m[4]); // punpckhqdq m7, m4 ; out9
    m[4] = _mm256_unpackhi_epi64(m[2], m[8]); // punpckhqdq m4, m2, m8 ; out10
    m[2] = _mm256_unpacklo_epi64(m[2], m[8]); // punpcklqdq m2, m8
    m[8] = _mm256_unpackhi_epi64(m[3], m[5]); // punpckhqdq m8, m3, m5
    m[3] = _mm256_unpacklo_epi64(m[3], m[5]); // punpcklqdq m3, m5
    m[5] = _mm256_unpackhi_epi64(m[0], m[6]); // punpckhqdq m5, m0, m6 ; out8
    m[0] = _mm256_unpacklo_epi64(m[0], m[6]); // punpcklqdq m0, m6
    repx!([0, 1, 2, 3, 4, 5, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[9])); // REPX {pmulhrsw x, m9}, m0, m1, m2, m3, m4, m5, m7
    if eob > 106 {
        // cmp eobd, 106 / jg .full
        // .full:
        repx!([12, 13, 14, 15], i => m[i] = _mm256_mulhrs_epi16(m[i], m[9])); // REPX {pmulhrsw x, m9}, m12, m13, m14, m15
        m[6] = _mm256_mulhrs_epi16(m[9], s[2]); // pmulhrsw m6, m9, [rsp+32*2]
        s[2] = m[4]; // mova [rsp+32*2], m4
        m[4] = _mm256_mulhrs_epi16(m[9], s[0]); // pmulhrsw m4, m9, [rsp+32*0]
        s[0] = m[5]; // mova [rsp+32*0], m5
        m[5] = _mm256_mulhrs_epi16(m[9], s[1]); // pmulhrsw m5, m9, [rsp+32*1]
        s[1] = m[7]; // mova [rsp+32*1], m7
        m[7] = _mm256_mulhrs_epi16(m[9], m[11]); // pmulhrsw m7, m9, m11
        m[11] = _mm256_mulhrs_epi16(m[9], m[8]); // pmulhrsw m11, m9, m8
        inv_txfm_add_dct_dct_8x32_main(m, s); // call .main
    } else {
        s[0] = m[5]; // mova [rsp+32*0], m5
        s[1] = m[7]; // mova [rsp+32*1], m7
        s[2] = m[4]; // mova [rsp+32*2], m4
        m[11] = _mm256_mulhrs_epi16(m[9], m[8]); // pmulhrsw m11, m9, m8
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        repx!([5, 6, 7], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, m7
        inv_txfm_add_dct_dct_8x32_main_fast(m, s); // call .main_fast
                                                   // jmp .pass2
    }
    // .pass2:
    m[12] = pw(2048); // vpbroadcastd m12, [o(pw_2048)]
    repx!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15], i => m[i] = _mm256_mulhrs_epi16(m[i], m[12])); // REPX {pmulhrsw x, m12}, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m13, m14, m15
    m[12] = _mm256_mulhrs_epi16(m[12], s[0]); // pmulhrsw m12, [rsp]
    repx!([0, 2, 4, 6, 8, 10, 12, 14], i => m[i] = _mm256_permute4x64_epi64::<0xd8>(m[i])); // REPX {vpermq x, x, q3120}, m0, m2, m4, m6, m8, m10, m12, m14
    repx!([1, 3, 5, 7, 9, 11, 13, 15], i => m[i] = _mm256_permute4x64_epi64::<0x8d>(m[i])); // REPX {vpermq x, x, q2031}, m1, m3, m5, m7, m9, m11, m13, m15
    s[0] = m[4]; // mova [rsp+32*0], m4
    s[1] = m[6]; // mova [rsp+32*1], m6
    let r3 = stride * 3; // lea r3, [strideq*3]
    write_8x4(m, dst, m[0], m[1], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 0, 1, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[2], m[3], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 2, 3, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, s[0], m[5], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 [rsp+32*0], 5, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, s[1], m[7], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 [rsp+32*1], 7, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[8], m[9], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 8, 9, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[10], m[11], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 10, 11, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[12], m[13], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 12, 13, 4, 6
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    write_8x4(m, dst, m[14], m[15], 4, 6, [stride, stride * 2, r3]); // WRITE_8X4 14, 15, 4, 6
}

/// `.main_fast` of `inv_txfm_add_dct_dct_8x32_8bpc` (the bottom half is zero): `s` is the caller's frame
/// (`[rsp+gprsize+32*0..2]`), which holds in1/in3 … on entry as for `.main`.
#[inline(always)]
pub(crate) unsafe fn inv_txfm_add_dct_dct_8x32_main_fast(m: &mut M, s: &mut [__m256i; 3]) {
    idct_8x16_internal_main(m); // call m(idct_8x16_internal_8bpc).main
    m[8] = s[0]; // mova m8, [rsp+gprsize+0*32]
    s[0] = m[0]; // mova [rsp+gprsize+0*32], m0
    m[9] = s[1]; // mova m9, [rsp+gprsize+1*32]
    s[1] = m[1]; // mova [rsp+gprsize+1*32], m1
    m[0] = s[2]; // mova m0, [rsp+gprsize+2*32]
    s[2] = m[6]; // mova [rsp+gprsize+2*32], m6
                 // lea r5, [r6-(o_base)+pw_201_4091x8]
    itx_unpack_mulhrsw(m, 1, 8, 6, 201, 4091, -601, 4052); // ITX_UNPACK_MULHRSW 1, 8, 6, 201, 4091, m601, 4052 ; t16a, t31a, t23a, t24a
    itx_unpack_mulhrsw(m, 15, 9, 6, 995, 3973, -1380, 3857); // ITX_UNPACK_MULHRSW 15, 9, 6, 995, 3973, m1380, 3857 ; t20a, t27a, t19a, t28a
    itx_unpack_mulhrsw(m, 14, 0, 6, 1751, 3703, -2106, 3513); // ITX_UNPACK_MULHRSW 14, 0, 6, 1751, 3703, m2106, 3513 ; t18a, t29a, t21a, t26a
    itx_unpack_mulhrsw(m, 13, 11, 6, 2440, 3290, -2751, 3035); // ITX_UNPACK_MULHRSW 13, 11, 6, 2440, 3290, m2751, 3035 ; t22a, t25a, t17a, t30a
    inv_txfm_add_dct_dct_8x32_main2(m, s) // jmp .main2
}

/// `.main` of `inv_txfm_add_dct_dct_8x32_8bpc`: the 32-point DCT of packed rows; `s` is the caller's frame
/// (`[rsp+gprsize+32*0..2]`), whose slots hold three of the inputs on entry and out24/out25 (slot 0) on
/// return.
#[inline(always)]
pub(crate) unsafe fn inv_txfm_add_dct_dct_8x32_main(m: &mut M, s: &mut [__m256i; 3]) {
    idct_8x16_internal_main(m); // call m(idct_8x16_internal_8bpc).main
    m[8] = s[0]; // mova m8, [rsp+gprsize+0*32]
    s[0] = m[0]; // mova [rsp+gprsize+0*32], m0
    m[9] = s[1]; // mova m9, [rsp+gprsize+1*32]
    s[1] = m[1]; // mova [rsp+gprsize+1*32], m1
    m[0] = s[2]; // mova m0, [rsp+gprsize+2*32]
    s[2] = m[6]; // mova [rsp+gprsize+2*32], m6
    m[1] = _mm256_unpacklo_epi16(m[15], m[8]); // punpcklwd m1, m15, m8 ; in31 in1
    m[8] = _mm256_unpackhi_epi16(m[8], m[15]); // punpckhwd m8, m15 ; in3  in29
    m[15] = _mm256_unpacklo_epi16(m[14], m[9]); // punpcklwd m15, m14, m9 ; in27 in5
    m[9] = _mm256_unpackhi_epi16(m[9], m[14]); // punpckhwd m9, m14 ; in7  in25
    m[14] = _mm256_unpacklo_epi16(m[13], m[0]); // punpcklwd m14, m13, m0 ; in23 in9
    m[0] = _mm256_unpackhi_epi16(m[0], m[13]); // punpckhwd m0, m13 ; in11 in21
    m[13] = _mm256_unpacklo_epi16(m[12], m[11]); // punpcklwd m13, m12, m11 ; in19 in13
    m[11] = _mm256_unpackhi_epi16(m[11], m[12]); // punpckhwd m11, m12 ; in15 in17
    itx_mul2x_pack(m, 1, 6, 12, 10, 201, 4091, 3); // ITX_MUL2X_PACK 1, 6, 12, 10, 201, 4091, 3 ; t16a, t31a
    itx_mul2x_pack(m, 8, 6, 12, 10, 4052, 601, 3); // ITX_MUL2X_PACK 8, 6, 12, 10, 4052, 601, 3 ; t23a, t24a
    itx_mul2x_pack(m, 15, 6, 12, 10, 995, 3973, 3); // ITX_MUL2X_PACK 15, 6, 12, 10, 995, 3973, 3 ; t20a, t27a
    itx_mul2x_pack(m, 9, 6, 12, 10, 3857, 1380, 3); // ITX_MUL2X_PACK 9, 6, 12, 10, 3857, 1380, 3 ; t19a, t28a
    itx_mul2x_pack(m, 14, 6, 12, 10, 1751, 3703, 3); // ITX_MUL2X_PACK 14, 6, 12, 10, 1751, 3703, 3 ; t18a, t29a
    itx_mul2x_pack(m, 0, 6, 12, 10, 3513, 2106, 3); // ITX_MUL2X_PACK 0, 6, 12, 10, 3513, 2106, 3 ; t21a, t26a
    itx_mul2x_pack(m, 13, 6, 12, 10, 2440, 3290, 3); // ITX_MUL2X_PACK 13, 6, 12, 10, 2440, 3290, 3 ; t22a, t25a
    itx_mul2x_pack(m, 11, 6, 12, 10, 3035, 2751, 3); // ITX_MUL2X_PACK 11, 6, 12, 10, 3035, 2751, 3 ; t17a, t30a
    inv_txfm_add_dct_dct_8x32_main2(m, s)
}

/// `.main2` of `inv_txfm_add_dct_dct_8x32_8bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_main2(m: &mut M, s: &mut [__m256i; 3]) {
    m[6] = _mm256_subs_epi16(m[1], m[11]); // psubsw m6, m1, m11 ; t17 t30
    m[1] = _mm256_adds_epi16(m[1], m[11]); // paddsw m1, m11 ; t16 t31
    m[11] = _mm256_subs_epi16(m[9], m[14]); // psubsw m11, m9, m14 ; t18 t29
    m[9] = _mm256_adds_epi16(m[9], m[14]); // paddsw m9, m14 ; t19 t28
    m[14] = _mm256_subs_epi16(m[15], m[0]); // psubsw m14, m15, m0 ; t21 t26
    m[15] = _mm256_adds_epi16(m[15], m[0]); // paddsw m15, m0 ; t20 t27
    m[0] = _mm256_subs_epi16(m[8], m[13]); // psubsw m0, m8, m13 ; t22 t25
    m[8] = _mm256_adds_epi16(m[8], m[13]); // paddsw m8, m13 ; t23 t24
    itx_mul2x_pack(m, 6, 12, 13, 10, 799, 4017, 3); // ITX_MUL2X_PACK 6, 12, 13, 10, 799, 4017, 3 ; t17a t30a
    itx_mul2x_pack(m, 11, 12, 13, 10, -4017, 799, 3); // ITX_MUL2X_PACK 11, 12, 13, 10, m4017, 799, 3 ; t18a t29a
    itx_mul2x_pack(m, 14, 12, 13, 10, 3406, 2276, 3); // ITX_MUL2X_PACK 14, 12, 13, 10, 3406, 2276, 3 ; t21a t26a
    itx_mul2x_pack(m, 0, 12, 13, 10, -2276, 3406, 3); // ITX_MUL2X_PACK 0, 12, 13, 10, m2276, 3406, 3 ; t22a t25a
    m[13] = _mm256_subs_epi16(m[1], m[9]); // psubsw m13, m1, m9 ; t19a t28a
    m[1] = _mm256_adds_epi16(m[1], m[9]); // paddsw m1, m9 ; t16a t31a
    m[9] = _mm256_subs_epi16(m[8], m[15]); // psubsw m9, m8, m15 ; t20a t27a
    m[8] = _mm256_adds_epi16(m[8], m[15]); // paddsw m8, m15 ; t23a t24a
    m[15] = _mm256_subs_epi16(m[6], m[11]); // psubsw m15, m6, m11 ; t18  t29
    m[6] = _mm256_adds_epi16(m[6], m[11]); // paddsw m6, m11 ; t17  t30
    m[11] = _mm256_subs_epi16(m[0], m[14]); // psubsw m11, m0, m14 ; t21  t26
    m[0] = _mm256_adds_epi16(m[0], m[14]); // paddsw m0, m14 ; t22  t25
    itx_mul2x_pack(m, 15, 12, 14, 10, 1567, 3784, 3); // ITX_MUL2X_PACK 15, 12, 14, 10, 1567, 3784, 3 ; t18a t29a
    itx_mul2x_pack(m, 13, 12, 14, 10, 1567, 3784, 3); // ITX_MUL2X_PACK 13, 12, 14, 10, 1567, 3784, 3 ; t19  t28
    itx_mul2x_pack(m, 9, 12, 14, 10, -3784, 1567, 3); // ITX_MUL2X_PACK 9, 12, 14, 10, m3784, 1567, 3 ; t20  t27
    itx_mul2x_pack(m, 11, 12, 14, 10, -3784, 1567, 3); // ITX_MUL2X_PACK 11, 12, 14, 10, m3784, 1567, 3 ; t21a t26a
    m[12] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m12, [o(deint_shuf)]
    m[14] = _mm256_subs_epi16(m[1], m[8]); // psubsw m14, m1, m8 ; t23  t24
    m[1] = _mm256_adds_epi16(m[1], m[8]); // paddsw m1, m8 ; t16  t31
    m[8] = _mm256_subs_epi16(m[6], m[0]); // psubsw m8, m6, m0 ; t22a t25a
    m[6] = _mm256_adds_epi16(m[6], m[0]); // paddsw m6, m0 ; t17a t30a
    m[0] = _mm256_subs_epi16(m[15], m[11]); // psubsw m0, m15, m11 ; t21  t26
    m[15] = _mm256_adds_epi16(m[15], m[11]); // paddsw m15, m11 ; t18  t29
    m[11] = _mm256_subs_epi16(m[13], m[9]); // psubsw m11, m13, m9 ; t20a t27a
    m[13] = _mm256_adds_epi16(m[13], m[9]); // paddsw m13, m9 ; t19a t28a
    repx!([1, 6, 15, 13], i => m[i] = _mm256_shuffle_epi8(m[i], m[12])); // REPX {pshufb x, m12}, m1, m6, m15, m13
    itx_mul2x_pack(m, 14, 9, 12, 10, 2896, 2896, 0); // ITX_MUL2X_PACK 14, 9, 12, 10, 2896, 2896 ; t24a t23a
    m[9] = pw2(-2896, 2896); // vpbroadcastd m9, [o(pw_m2896_2896)]
    itx_mul2x_pack(m, 8, 12, usize::MAX, 10, 12, 9, 4); // ITX_MUL2X_PACK 8, 12, _, 10, 12, 9, 4 ; t22  t25
    m[12] = pw2(2896, 2896); // vpbroadcastd m12, [o(pw_2896_2896)]
    itx_mul2x_pack(m, 0, 12, usize::MAX, 10, 12, 9, 4); // ITX_MUL2X_PACK 0, 12, _, 10, 12, 9, 4 ; t21a t26a
    m[12] = pw2(2896, 2896); // vpbroadcastd m12, [o(pw_2896_2896)]
    itx_mul2x_pack(m, 11, 9, usize::MAX, 10, 9, 12, 4); // ITX_MUL2X_PACK 11, 9, _, 10, 9, 12, 4 ; t27  t20
    m[9] = _mm256_castps_si256(_mm256_shuffle_ps::<0x4e>(
        _mm256_castsi256_ps(m[14]),
        _mm256_castsi256_ps(m[8]),
    )); // shufps m9, m14, m8, q1032 ; t23a t22
    m[14] = _mm256_blend_epi32::<0xcc>(m[14], m[8]); // vpblendd m14, m8, 0xcc ; t24a t25
    m[8] = _mm256_castps_si256(_mm256_shuffle_ps::<0x4e>(
        _mm256_castsi256_ps(m[11]),
        _mm256_castsi256_ps(m[0]),
    )); // shufps m8, m11, m0, q1032 ; t20  t21a
    m[11] = _mm256_blend_epi32::<0xcc>(m[11], m[0]); // vpblendd m11, m0, 0xcc ; t27  t26a
    m[0] = _mm256_unpacklo_epi64(m[1], m[6]); // punpcklqdq m0, m1, m6 ; t16  t17a
    m[1] = _mm256_unpackhi_epi64(m[1], m[6]); // punpckhqdq m1, m6 ; t31  t30a
    m[10] = _mm256_subs_epi16(m[5], m[8]); // psubsw m10, m5, m8 ; out20 out21
    m[5] = _mm256_adds_epi16(m[5], m[8]); // paddsw m5, m8 ; out11 out10
    m[6] = _mm256_subs_epi16(m[3], m[14]); // psubsw m6, m3, m14 ; out24 out25
    m[3] = _mm256_adds_epi16(m[3], m[14]); // paddsw m3, m14 ; out7  out6
    m[8] = _mm256_subs_epi16(m[7], m[0]); // psubsw m8, m7, m0 ; out16 out17
    m[7] = _mm256_adds_epi16(m[7], m[0]); // paddsw m7, m0 ; out15 out14
    m[0] = s[0]; // mova m0, [rsp+gprsize+0*32]
    m[12] = _mm256_unpacklo_epi64(m[13], m[15]); // punpcklqdq m12, m13, m15 ; t19a t18
    m[13] = _mm256_unpackhi_epi64(m[13], m[15]); // punpckhqdq m13, m15 ; t28a t29
    m[15] = _mm256_subs_epi16(m[0], m[1]); // psubsw m15, m0, m1 ; out31 out30
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; out0  out1
    m[1] = s[1]; // mova m1, [rsp+gprsize+1*32]
    s[0] = m[6]; // mova [rsp+gprsize+0*32], m6
    m[6] = s[2]; // mova m6, [rsp+gprsize+2*32]
    m[14] = _mm256_subs_epi16(m[1], m[13]); // psubsw m14, m1, m13 ; out28 out29
    m[1] = _mm256_adds_epi16(m[1], m[13]); // paddsw m1, m13 ; out3  out2
    m[13] = _mm256_subs_epi16(m[2], m[11]); // psubsw m13, m2, m11 ; out27 out26
    m[2] = _mm256_adds_epi16(m[2], m[11]); // paddsw m2, m11 ; out4  out5
    m[11] = _mm256_subs_epi16(m[4], m[9]); // psubsw m11, m4, m9 ; out23 out22
    m[4] = _mm256_adds_epi16(m[4], m[9]); // paddsw m4, m9 ; out8  out9
    m[9] = _mm256_subs_epi16(m[6], m[12]); // psubsw m9, m6, m12 ; out19 out18
    m[6] = _mm256_adds_epi16(m[6], m[12]); // paddsw m6, m12 ; out12 out13
}

/// `%macro LOAD_PACKED_16X2 dst, tmp, row[1-2]`
#[inline(always)]
unsafe fn load_packed_16x2(m: &mut M, c: Coef, d: usize, t: usize, r1: isize, r2: isize) {
    m[d] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * r1).cast())); // vbroadcasti128 m%1, [cq+16*%3]
    m[t] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * r2).cast())); // vbroadcasti128 m%2, [cq+16*%4]
    m[d] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[d]),
        _mm256_castsi256_pd(m[t]),
    )); // shufpd m%1, m%2, 0x0c
}

/// `cglobal inv_txfm_add_dct_dct_32x8_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 8; // or r3d, 8
        return inv_txfm_add_dct_dct_32x8_dconly(m, dst, stride, r3d); // .dconly: (in the prelude)
    }
    // .normal:
    // PROLOGUE 0, 4, 16, 32*3, dst, stride, c, eob: a frame of three 32-byte slots. Zeroed whole; all three
    // are stored below before `.main`/`.main_fast` read them.
    let mut s = [_mm256_setzero_si256(); 3];
    let s = &mut s;
    load_packed_16x2(m, c, 0, 7, 0, 2); // LOAD_PACKED_16X2 0, 7, 0, 2 ; in0  in2
    load_packed_16x2(m, c, 4, 7, 1, 3); // LOAD_PACKED_16X2 4, 7, 1, 3 ; in1  in3
    load_packed_16x2(m, c, 1, 7, 4, 6); // LOAD_PACKED_16X2 1, 7, 4, 6 ; in4  in6
    load_packed_16x2(m, c, 5, 7, 5, 7); // LOAD_PACKED_16X2 5, 7, 5, 7 ; in5  in7
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    repx!([0, 1, 2, 3], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, 0, 1, 2, 3
    c = at(c, 16 * 16); // add cq, 16*16
    load_packed_16x2(m, c, 2, 7, -8, -6); // LOAD_PACKED_16X2 2, 7, -8, -6 ; in8  in10
    load_packed_16x2(m, c, 6, 7, -7, -5); // LOAD_PACKED_16X2 6, 7, -7, -5 ; in9  in11
    load_packed_16x2(m, c, 3, 7, -4, -2); // LOAD_PACKED_16X2 3, 7, -4, -2 ; in12 in14
    load_packed_16x2(m, c, 11, 7, -3, -1); // LOAD_PACKED_16X2 11, 7, -3, -1 ; in13 in15
    repx!([-4, -3, -2, -1], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, -4, -3, -2, -1
    s[0] = m[4]; // mova [rsp+32*0], m4
    s[1] = m[5]; // mova [rsp+32*1], m5
    s[2] = m[6]; // mova [rsp+32*2], m6
    if eob > 106 {
        // cmp eobd, 106 / jg .full
        // .full:
        load_packed_16x2(m, c, 4, 7, 0, 2); // LOAD_PACKED_16X2 4, 7, 0, 2 ; in16 in18
        load_packed_16x2(m, c, 12, 7, 3, 1); // LOAD_PACKED_16X2 12, 7, 3, 1 ; in19 in17
        load_packed_16x2(m, c, 5, 7, 4, 6); // LOAD_PACKED_16X2 5, 7, 4, 6 ; in20 in22
        load_packed_16x2(m, c, 13, 7, 7, 5); // LOAD_PACKED_16X2 13, 7, 7, 5 ; in23 in21
        repx!([0, 1, 2, 3], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, 0, 1, 2, 3
        c = at(c, 16 * 8); // add cq, 16*8
        load_packed_16x2(m, c, 6, 7, 0, 2); // LOAD_PACKED_16X2 6, 7, 0, 2 ; in24 in26
        load_packed_16x2(m, c, 14, 7, 3, 1); // LOAD_PACKED_16X2 14, 7, 3, 1 ; in27 in25
        load_packed_16x2(m, c, 7, 8, 4, 6); // LOAD_PACKED_16X2 7, 8, 4, 6 ; in28 in30
        load_packed_16x2(m, c, 15, 8, 7, 5); // LOAD_PACKED_16X2 15, 8, 7, 5 ; in31 in29
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([0, 1, 2, 3], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, 0, 1, 2, 3
        inv_txfm_add_dct_dct_8x32_main(m, s); // call m(inv_txfm_add_dct_dct_8x32_8bpc).main
    } else {
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        repx!([5, 6, 7], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, m7
        inv_txfm_add_dct_dct_8x32_main_fast(m, s); // call m(inv_txfm_add_dct_dct_8x32_8bpc).main_fast
                                                   // jmp .pass2
    }
    // .pass2:
    m[12] = pw(8192); // vpbroadcastd m12, [o(pw_8192)]
    repx!([8, 9, 10, 11, 13, 14, 15], i => m[i] = _mm256_mulhrs_epi16(m[i], m[12])); // REPX {pmulhrsw x, m12}, m8, m9, m10, m11, m13, m14, m15
    s[1] = m[9]; // mova [rsp+32*1], m9
    s[2] = m[10]; // mova [rsp+32*2], m10
    m[9] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m9, m0, m2
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    m[2] = _mm256_unpackhi_epi16(m[1], m[3]); // punpckhwd m2, m1, m3
    m[1] = _mm256_unpacklo_epi16(m[1], m[3]); // punpcklwd m1, m3
    m[10] = _mm256_unpacklo_epi16(m[4], m[6]); // punpcklwd m10, m4, m6
    m[4] = _mm256_unpackhi_epi16(m[4], m[6]); // punpckhwd m4, m6
    m[6] = _mm256_unpacklo_epi16(m[5], m[7]); // punpcklwd m6, m5, m7
    m[5] = _mm256_unpackhi_epi16(m[5], m[7]); // punpckhwd m5, m7
    m[3] = _mm256_unpackhi_epi16(m[0], m[9]); // punpckhwd m3, m0, m9
    m[0] = _mm256_unpacklo_epi16(m[0], m[9]); // punpcklwd m0, m9
    m[9] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m9, m2, m1
    m[2] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m2, m1
    m[7] = _mm256_unpacklo_epi16(m[10], m[4]); // punpcklwd m7, m10, m4
    m[10] = _mm256_unpackhi_epi16(m[10], m[4]); // punpckhwd m10, m4
    m[4] = _mm256_unpacklo_epi16(m[5], m[6]); // punpcklwd m4, m5, m6
    m[5] = _mm256_unpackhi_epi16(m[5], m[6]); // punpckhwd m5, m6
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[9]); // punpckldq m2, m3, m9
    m[3] = _mm256_unpackhi_epi32(m[3], m[9]); // punpckhdq m3, m9
    m[6] = _mm256_unpacklo_epi32(m[7], m[4]); // punpckldq m6, m7, m4
    m[7] = _mm256_unpackhi_epi32(m[7], m[4]); // punpckhdq m7, m4
    m[9] = _mm256_unpacklo_epi32(m[10], m[5]); // punpckldq m9, m10, m5
    m[10] = _mm256_unpackhi_epi32(m[10], m[5]); // punpckhdq m10, m5
    repx!([0, 1, 2, 3, 6, 7, 9, 10], i => m[i] = _mm256_mulhrs_epi16(m[i], m[12])); // REPX {pmulhrsw x, m12}, m0, m1, m2, m3, m6, m7, m9, m10
    m[12] = _mm256_mulhrs_epi16(m[12], s[0]); // pmulhrsw m12, [rsp+32*0]
    s[0] = m[8]; // mova [rsp+32*0], m8
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[6]); // vperm2i128 m4, m0, m6, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[6])); // vinserti128 m0, xm6, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[7]); // vperm2i128 m5, m1, m7, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[7])); // vinserti128 m1, xm7, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[2], m[9]); // vperm2i128 m6, m2, m9, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[9])); // vinserti128 m2, xm9, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[3], m[10]); // vperm2i128 m7, m3, m10, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[10])); // vinserti128 m3, xm10, 1
    idct_16x8_internal_main(m); // call m(idct_16x8_internal_8bpc).main
    m[8] = pw(2048); // vpbroadcastd m8, [o(pw_2048)]
    repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[8])); // REPX {pmulhrsw x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    let r2 = stride * 3; // lea r2, [strideq*3]
    write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
    write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r2]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r2
    let mut r3 = at(dst, stride * 4); // lea r3, [dstq+strideq*4]
                                      // %define dstq r3
    write_16x2(m, r3, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
    write_16x2(m, r3, m[6], m[7], 0, 1, [stride * 2, r2]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r2
    m[0] = s[0]; // mova m0, [rsp+32*0]
    m[1] = s[1]; // mova m1, [rsp+32*1]
    m[2] = s[2]; // mova m2, [rsp+32*2]
    m[7] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m7, m0, m2
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    m[2] = _mm256_unpackhi_epi16(m[1], m[11]); // punpckhwd m2, m1, m11
    m[1] = _mm256_unpacklo_epi16(m[1], m[11]); // punpcklwd m1, m11
    m[4] = _mm256_unpackhi_epi16(m[12], m[14]); // punpckhwd m4, m12, m14
    m[12] = _mm256_unpacklo_epi16(m[12], m[14]); // punpcklwd m12, m14
    m[5] = _mm256_unpackhi_epi16(m[13], m[15]); // punpckhwd m5, m13, m15
    m[13] = _mm256_unpacklo_epi16(m[13], m[15]); // punpcklwd m13, m15
    m[3] = _mm256_unpackhi_epi16(m[0], m[7]); // punpckhwd m3, m0, m7
    m[0] = _mm256_unpacklo_epi16(m[0], m[7]); // punpcklwd m0, m7
    m[9] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m9, m2, m1
    m[2] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m2, m1
    m[7] = _mm256_unpacklo_epi16(m[12], m[4]); // punpcklwd m7, m12, m4
    m[12] = _mm256_unpackhi_epi16(m[12], m[4]); // punpckhwd m12, m4
    m[4] = _mm256_unpacklo_epi16(m[5], m[13]); // punpcklwd m4, m5, m13
    m[5] = _mm256_unpackhi_epi16(m[5], m[13]); // punpckhwd m5, m13
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[3], m[9]); // punpckldq m2, m3, m9
    m[3] = _mm256_unpackhi_epi32(m[3], m[9]); // punpckhdq m3, m9
    m[6] = _mm256_unpacklo_epi32(m[7], m[4]); // punpckldq m6, m7, m4
    m[7] = _mm256_unpackhi_epi32(m[7], m[4]); // punpckhdq m7, m4
    m[9] = _mm256_unpacklo_epi32(m[12], m[5]); // punpckldq m9, m12, m5
    m[12] = _mm256_unpackhi_epi32(m[12], m[5]); // punpckhdq m12, m5
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[6]); // vperm2i128 m4, m0, m6, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[6])); // vinserti128 m0, xm6, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[7]); // vperm2i128 m5, m1, m7, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[7])); // vinserti128 m1, xm7, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[2], m[9]); // vperm2i128 m6, m2, m9, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[9])); // vinserti128 m2, xm9, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[3], m[12]); // vperm2i128 m7, m3, m12, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[12])); // vinserti128 m3, xm12, 1
    idct_16x8_internal_main2(m); // call m(idct_16x8_internal_8bpc).main2
    m[8] = pw(2048); // vpbroadcastd m8, [o(pw_2048)]
    repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[8])); // REPX {pmulhrsw x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    dst = at(dst, 16); // add r0, 16
    r3 = at(r3, 16); // add r3, 16
                     // %define dstq r0
    write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
    write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r2]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r2
                                                            // %define dstq r3
    write_16x2(m, r3, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
    write_16x2(m, r3, m[6], m[7], 0, 1, [stride * 2, r2]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r2
}

/// `cglobal inv_txfm_add_identity_identity_8x32_8bpc, 4, 5, 11, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[9] = pw(5); // vpbroadcastd m9, [pw_5]
    let r4 = stride * 3; // lea r4, [strideq*3]
    let mut eobd = (eob as u32).wrapping_sub(107); // sub eobd, 107 ; loop_iterations = 1 + (eobd >= 107)
    loop {
        // .loop:
        m[0] = zx(_mm_loadu_si128(at(c, 16 * 0).cast())); // mova xm0,[cq+16* 0]
        m[1] = zx(_mm_loadu_si128(at(c, 16 * 4).cast())); // mova xm1, [cq+16* 4]
        m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 16 * 1).cast())); // vinserti128 m0, [cq+16* 1], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 16 * 5).cast())); // vinserti128 m1, [cq+16* 5], 1
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        _mm256_storeu_si256(at(c, 32 * 0).cast(), m[8]); // mova [cq+32*0], m8
        _mm256_storeu_si256(at(c, 32 * 2).cast(), m[8]); // mova [cq+32*2], m8
        c = at(c, 16 * 16); // add cq, 16*16
        m[2] = zx(_mm_loadu_si128(at(c, -16 * 8).cast())); // mova xm2, [cq-16* 8]
        m[3] = zx(_mm_loadu_si128(at(c, -16 * 4).cast())); // mova xm3, [cq-16* 4]
        m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, -16 * 7).cast())); // vinserti128 m2, [cq-16* 7], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, -16 * 3).cast())); // vinserti128 m3, [cq-16* 3], 1
        m[4] = zx(_mm_loadu_si128(at(c, 16 * 0).cast())); // mova xm4, [cq+16* 0]
        m[5] = zx(_mm_loadu_si128(at(c, 16 * 4).cast())); // mova xm5, [cq+16* 4]
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 16 * 1).cast())); // vinserti128 m4, [cq+16* 1], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(c, 16 * 5).cast())); // vinserti128 m5, [cq+16* 5], 1
        m[6] = zx(_mm_loadu_si128(at(c, 16 * 8).cast())); // mova xm6, [cq+16* 8]
        m[7] = zx(_mm_loadu_si128(at(c, 16 * 12).cast())); // mova xm7, [cq+16*12]
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(c, 16 * 9).cast())); // vinserti128 m6, [cq+16* 9], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(c, 16 * 13).cast())); // vinserti128 m7, [cq+16*13], 1
        repx!([-4, -2, 0, 2, 4, 6], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, -4, -2, 0, 2, 4, 6
        repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_adds_epi16(m[i], m[9])); // REPX {paddsw x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call .transpose8x8
        repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_srai_epi16::<3>(m[i])); // REPX {psraw x, 3 }, m0, m1, m2, m3, m4, m5, m6, m7
        write_8x4(m, dst, m[0], m[4], 8, 10, [stride * 8, stride * 4, r4 * 4]); // WRITE_8X4 0, 4, 8, 10, strideq*8, strideq*4, r4*4
        dst = at(dst, stride); // add dstq, strideq
        write_8x4(m, dst, m[1], m[5], 0, 4, [stride * 8, stride * 4, r4 * 4]); // WRITE_8X4 1, 5, 0, 4, strideq*8, strideq*4, r4*4
        dst = at(dst, stride); // add dstq, strideq
        write_8x4(m, dst, m[2], m[6], 0, 4, [stride * 8, stride * 4, r4 * 4]); // WRITE_8X4 2, 6, 0, 4, strideq*8, strideq*4, r4*4
        dst = at(dst, stride); // add dstq, strideq
        write_8x4(m, dst, m[3], m[7], 0, 4, [stride * 8, stride * 4, r4 * 4]); // WRITE_8X4 3, 7, 0, 4, strideq*8, strideq*4, r4*4
        dst = at(dst, stride); // add dstq, strideq
        c = at(c, -(16 * 16 - 32)); // sub cq, 16*16-32
        dst = at(dst, r4 * 4); // lea dstq, [dstq+r4*4]
        let carry;
        (eobd, carry) = eobd.overflowing_add(0x8000_0000); // add eobd, 0x80000000
        if carry {
            break; // jnc .loop
        }
    }
}

/// `cglobal inv_txfm_add_identity_identity_32x8_8bpc, 4, 6, 10, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x8(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    c = at(c, 16 * 8); // add cq, 16*8
    m[9] = pw(4096); // vpbroadcastd m9, [pw_4096]
    let r4 = stride * 3; // lea r4, [strideq*3]
    let mut r5 = at(dst, stride * 4); // lea r5, [dstq+strideq*4]
    let mut eobd = (eob as u32).wrapping_sub(107); // sub eobd, 107
    loop {
        // .loop:
        m[0] = zx(_mm_loadu_si128(at(c, -16 * 8).cast())); // mova xm0, [cq-16*8]
        m[1] = zx(_mm_loadu_si128(at(c, -16 * 7).cast())); // mova xm1, [cq-16*7]
        m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 16 * 0).cast())); // vinserti128 m0, [cq+16*0], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 16 * 1).cast())); // vinserti128 m1, [cq+16*1], 1
        m[2] = zx(_mm_loadu_si128(at(c, -16 * 6).cast())); // mova xm2, [cq-16*6]
        m[3] = zx(_mm_loadu_si128(at(c, -16 * 5).cast())); // mova xm3, [cq-16*5]
        m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, 16 * 2).cast())); // vinserti128 m2, [cq+16*2], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, 16 * 3).cast())); // vinserti128 m3, [cq+16*3], 1
        m[4] = zx(_mm_loadu_si128(at(c, -16 * 4).cast())); // mova xm4, [cq-16*4]
        m[5] = zx(_mm_loadu_si128(at(c, -16 * 3).cast())); // mova xm5, [cq-16*3]
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 16 * 4).cast())); // vinserti128 m4, [cq+16*4], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(c, 16 * 5).cast())); // vinserti128 m5, [cq+16*5], 1
        m[6] = zx(_mm_loadu_si128(at(c, -16 * 2).cast())); // mova xm6, [cq-16*2]
        m[7] = zx(_mm_loadu_si128(at(c, -16 * 1).cast())); // mova xm7, [cq-16*1]
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(c, 16 * 6).cast())); // vinserti128 m6, [cq+16*6], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(c, 16 * 7).cast())); // vinserti128 m7, [cq+16*7], 1
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([-4, -3, -2, -1, 0, 1, 2, 3], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, -4, -3, -2, -1, 0, 1, 2, 3
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[9])); // REPX {pmulhrsw x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
        write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
        write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r4]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r4
                                                                // %define dstq r5
        write_16x2(m, r5, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
        write_16x2(m, r5, m[6], m[7], 0, 1, [stride * 2, r4]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r4
        c = at(c, 16 * 16); // add cq, 16*16
        dst = at(dst, 16); // add r0, 16
        r5 = at(r5, 16); // add r5, 16
        let carry;
        (eobd, carry) = eobd.overflowing_add(0x8000_0000); // add eobd, 0x80000000
        if carry {
            break; // jnc .loop
        }
    }
}

inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_8x32_8bpc_avx2, inv_txfm_add_dct_dct_8x32);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x32_8bpc_avx2,
    inv_txfm_add_identity_identity_8x32
);
inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_32x8_8bpc_avx2, inv_txfm_add_dct_dct_32x8);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x8_8bpc_avx2,
    inv_txfm_add_identity_identity_32x8
);

// --- part 4: itx_avx2.asm lines 3546-4412 (16x32, 32x16, 32x32) --------------------

// The stack frames of this part. `[rsp+32*0]`, `[rsp+32*1]` and `[rsp+32*2]` (`[rsp+gprsize+32*n]` inside a
// subroutine) are the three spill slots that the subroutines share with their caller: `stack: &mut [__m256i; 3]`.
// The rest of a frame (`[rsp+32*3]` and up) holds the intermediate rows; it is the local array `frame`
// (`frame[0]` is `[rsp+32*3]`), reached only through the raw pointers `tmp1q`, `tmp2q`, `tmp3q` as in the
// assembly, with the assembly's byte offsets. Every `frame` is zero-initialized whole before its first use.

/// `%macro LOAD_16ROWS src, stride, is_rect2 = 0, zero_coefs = 1`: sixteen rows, `stride` bytes apart, from
/// `src` into `m0`-`m15` (with `is_rect2` scaled by `pw_2896x8`), `m15` also into `[rsp]` (`stack[0]`), then
/// with `zero_coefs` the sixteen rows cleared.
#[inline(always)]
unsafe fn load_16rows<T>(
    m: &mut M,
    stack: &mut [__m256i; 3],
    src: *mut T,
    stride: isize,
    is_rect2: u32,
    zero_coefs: u32,
) {
    if is_rect2 != 0 {
        m[15] = pw(2896 * 8); // vpbroadcastd m15, [o(pw_2896x8)]
        m[0] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 0).cast())); // pmulhrsw m0, m15, [%1+%2* 0]
        m[1] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 1).cast())); // pmulhrsw m1, m15, [%1+%2* 1]
        m[2] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 2).cast())); // pmulhrsw m2, m15, [%1+%2* 2]
        m[3] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 3).cast())); // pmulhrsw m3, m15, [%1+%2* 3]
        m[4] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 4).cast())); // pmulhrsw m4, m15, [%1+%2* 4]
        m[5] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 5).cast())); // pmulhrsw m5, m15, [%1+%2* 5]
        m[6] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 6).cast())); // pmulhrsw m6, m15, [%1+%2* 6]
        m[7] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 7).cast())); // pmulhrsw m7, m15, [%1+%2* 7]
        m[8] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 8).cast())); // pmulhrsw m8, m15, [%1+%2* 8]
        m[9] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 9).cast())); // pmulhrsw m9, m15, [%1+%2* 9]
        m[10] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 10).cast())); // pmulhrsw m10, m15, [%1+%2*10]
        m[11] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 11).cast())); // pmulhrsw m11, m15, [%1+%2*11]
        m[12] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 12).cast())); // pmulhrsw m12, m15, [%1+%2*12]
        m[13] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 13).cast())); // pmulhrsw m13, m15, [%1+%2*13]
        m[14] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 14).cast())); // pmulhrsw m14, m15, [%1+%2*14]
        m[15] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(src, stride * 15).cast()));
    // pmulhrsw m15, [%1+%2*15]
    } else {
        m[0] = _mm256_loadu_si256(at(src, stride * 0).cast()); // mova m0, [%1+%2* 0]
        m[1] = _mm256_loadu_si256(at(src, stride * 1).cast()); // mova m1, [%1+%2* 1]
        m[2] = _mm256_loadu_si256(at(src, stride * 2).cast()); // mova m2, [%1+%2* 2]
        m[3] = _mm256_loadu_si256(at(src, stride * 3).cast()); // mova m3, [%1+%2* 3]
        m[4] = _mm256_loadu_si256(at(src, stride * 4).cast()); // mova m4, [%1+%2* 4]
        m[5] = _mm256_loadu_si256(at(src, stride * 5).cast()); // mova m5, [%1+%2* 5]
        m[6] = _mm256_loadu_si256(at(src, stride * 6).cast()); // mova m6, [%1+%2* 6]
        m[7] = _mm256_loadu_si256(at(src, stride * 7).cast()); // mova m7, [%1+%2* 7]
        m[8] = _mm256_loadu_si256(at(src, stride * 8).cast()); // mova m8, [%1+%2* 8]
        m[9] = _mm256_loadu_si256(at(src, stride * 9).cast()); // mova m9, [%1+%2* 9]
        m[10] = _mm256_loadu_si256(at(src, stride * 10).cast()); // mova m10, [%1+%2*10]
        m[11] = _mm256_loadu_si256(at(src, stride * 11).cast()); // mova m11, [%1+%2*11]
        m[12] = _mm256_loadu_si256(at(src, stride * 12).cast()); // mova m12, [%1+%2*12]
        m[13] = _mm256_loadu_si256(at(src, stride * 13).cast()); // mova m13, [%1+%2*13]
        m[14] = _mm256_loadu_si256(at(src, stride * 14).cast()); // mova m14, [%1+%2*14]
        m[15] = _mm256_loadu_si256(at(src, stride * 15).cast()); // mova m15, [%1+%2*15]
    }
    stack[0] = m[15]; // mova [rsp], m15
    if zero_coefs != 0 {
        m[15] = _mm256_setzero_si256(); // pxor m15, m15
        repx!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], x => _mm256_storeu_si256(at(src, stride * x).cast(), m[15]));
        // REPX {mova [%1+%2*x], m15}, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    }
}

/// `%macro IDCT32_PASS2_END coefs[1-2], tmp[1-2], rnd, offset[1-2]`: `coefs[1]`, `tmp[1-2]` and `rnd` are
/// register numbers, `coefs[2]` is the address of the memory operand; the two rows are `[dstq+offset1]` and
/// `[r2+offset2]`.
#[inline(always)]
unsafe fn idct32_pass2_end(
    m: &mut M,
    dst: Dst,
    r2: Dst,
    c1: usize,
    c2: *mut __m256i,
    t1: usize,
    t2: usize,
    rnd: usize,
    off1: isize,
    off2: isize,
) {
    m[t2] = _mm256_loadu_si256(c2); // mova m%4, [%2]
    m[t1] = _mm256_adds_epi16(m[c1], m[t2]); // paddsw m%3, m%1, m%4
    m[c1] = _mm256_subs_epi16(m[c1], m[t2]); // psubsw m%1, m%4
    m[t2] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(dst, off1).cast())); // pmovzxbw m%4, [dstq+%6]
    m[t1] = _mm256_mulhrs_epi16(m[t1], m[rnd]); // pmulhrsw m%3, m%5
    m[c1] = _mm256_mulhrs_epi16(m[c1], m[rnd]); // pmulhrsw m%1, m%5
    m[t1] = _mm256_add_epi16(m[t1], m[t2]); // paddw m%3, m%4
    m[t2] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(r2, off2).cast())); // pmovzxbw m%4, [r2+%7]
    m[c1] = _mm256_add_epi16(m[c1], m[t2]); // paddw m%1, m%4
    m[t1] = _mm256_packus_epi16(m[t1], m[c1]); // packuswb m%3, m%1
    m[t1] = _mm256_permute4x64_epi64::<0xd8>(m[t1]); // vpermq m%3, m%3, q3120
    _mm_storeu_si128(at(dst, off1).cast(), lo(m[t1])); // mova [dstq+%6], xm%3
    _mm_storeu_si128(at(r2, off2).cast(), _mm256_extracti128_si256::<1>(m[t1]));
    // vextracti128 [r2+%7], m%3, 1
}

/// `cglobal inv_txfm_add_dct_dct_16x32_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        let r3d = eob | 32; // or r3d, 32
        return inv_txfm_add_dct_dct_16x4_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_16x4_8bpc).dconly
    }
    // PROLOGUE 0, 8, 16, 32*35, dst, stride, c, eob, tmp1, tmp2, base, tmp3
    let mut stack = [_mm256_setzero_si256(); 3]; // [rsp+32*0..3]
    let stack = &mut stack;
    let mut frame = core::mem::MaybeUninit::<[__m256i; 32]>::uninit(); // [rsp+32*3..35]
    let frame0: *mut __m256i = zeroed_frame(&mut frame); // zeroed whole (see above)
    load_16rows(m, stack, c, 64, 1, 1); // LOAD_16ROWS cq, 64, 1
    idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
    let tmp1 = at(frame0, 32 * 4); // lea tmp1q, [rsp+32*7]
    let tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
    let tmp3 = at(tmp1, 32 * 16); // lea tmp3q, [tmp1q+32*16]
    m[1] = stack[1]; // mova m1, [rsp+32*1]
    stack[0] = m[6]; // mova [rsp+32*0], m6
    stack[1] = m[7]; // mova [rsp+32*1], m7
    m[7] = pw(16384); // vpbroadcastd m7, [o(pw_16384)]
    inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, stack); // call .transpose_2x8x8_round
    m[15] = stack[0]; // mova m15, [rsp+32*0]
    _mm_storeu_si128(at(tmp3, -32 * 4).cast(), lo(m[0])); // mova [tmp3q-32*4+ 0], xm0
    _mm_storeu_si128(at(tmp3, 32 * 0).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [tmp3q+32*0+ 0], m0, 1
    _mm_storeu_si128(at(tmp3, -32 * 3).cast(), lo(m[2])); // mova [tmp3q-32*3+ 0], xm2
    _mm_storeu_si128(at(tmp3, 32 * 1).cast(), _mm256_extracti128_si256::<1>(m[2])); // vextracti128 [tmp3q+32*1+ 0], m2, 1
    _mm_storeu_si128(at(tmp3, -32 * 2).cast(), lo(m[4])); // mova [tmp3q-32*2+ 0], xm4
    _mm_storeu_si128(at(tmp3, 32 * 2).cast(), _mm256_extracti128_si256::<1>(m[4])); // vextracti128 [tmp3q+32*2+ 0], m4, 1
    _mm_storeu_si128(at(tmp3, -32 * 1).cast(), lo(m[6])); // mova [tmp3q-32*1+ 0], xm6
    _mm_storeu_si128(at(tmp3, 32 * 3).cast(), _mm256_extracti128_si256::<1>(m[6])); // vextracti128 [tmp3q+32*3+ 0], m6, 1
    _mm_storeu_si128(at(tmp3, -32 * 4 + 16).cast(), lo(m[8])); // mova [tmp3q-32*4+16], xm8
    _mm_storeu_si128(at(tmp3, 32 * 0 + 16).cast(), _mm256_extracti128_si256::<1>(m[8])); // vextracti128 [tmp3q+32*0+16], m8, 1
    _mm_storeu_si128(at(tmp3, -32 * 3 + 16).cast(), lo(m[10])); // mova [tmp3q-32*3+16], xm10
    _mm_storeu_si128(at(tmp3, 32 * 1 + 16).cast(), _mm256_extracti128_si256::<1>(m[10])); // vextracti128 [tmp3q+32*1+16], m10, 1
    _mm_storeu_si128(at(tmp3, -32 * 2 + 16).cast(), lo(m[12])); // mova [tmp3q-32*2+16], xm12
    _mm_storeu_si128(at(tmp3, 32 * 2 + 16).cast(), _mm256_extracti128_si256::<1>(m[12])); // vextracti128 [tmp3q+32*2+16], m12, 1
    _mm_storeu_si128(at(tmp3, -32 * 1 + 16).cast(), lo(m[14])); // mova [tmp3q-32*1+16], xm14
    _mm_storeu_si128(at(tmp3, 32 * 3 + 16).cast(), _mm256_extracti128_si256::<1>(m[14])); // vextracti128 [tmp3q+32*3+16], m14, 1
    if eob <= 150 {
        // cmp eobd, 150 / jg .full
        m[0] = _mm256_inserti128_si256::<1>(m[1], lo(m[9])); // vinserti128 m0, m1, xm9, 1
        m[4] = _mm256_permute2x128_si256::<0x31>(m[1], m[9]); // vperm2i128 m4, m1, m9, 0x31
        m[2] = _mm256_inserti128_si256::<1>(m[5], lo(m[13])); // vinserti128 m2, m5, xm13, 1
        m[6] = _mm256_permute2x128_si256::<0x31>(m[5], m[13]); // vperm2i128 m6, m5, m13, 0x31
        m[1] = _mm256_inserti128_si256::<1>(m[3], lo(m[11])); // vinserti128 m1, m3, xm11, 1
        m[5] = _mm256_permute2x128_si256::<0x31>(m[3], m[11]); // vperm2i128 m5, m3, m11, 0x31
        m[3] = _mm256_inserti128_si256::<1>(m[7], lo(m[15])); // vinserti128 m3, m7, xm15, 1
        m[7] = _mm256_permute2x128_si256::<0x31>(m[7], m[15]); // vperm2i128 m7, m7, m15, 0x31
        inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, stack, tmp1, tmp2); // call .main_oddhalf_fast
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([9, 10, 11, 12, 13, 14, 15], r => m[r] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
                                                              // jmp .idct16
    } else {
        // .full:
        at(tmp1, -32 * 4).write(m[1]); // mova [tmp1q-32*4], m1
        at(tmp1, -32 * 3).write(m[3]); // mova [tmp1q-32*3], m3
        at(tmp1, -32 * 2).write(m[5]); // mova [tmp1q-32*2], m5
        at(tmp1, -32 * 1).write(m[7]); // mova [tmp1q-32*1], m7
        at(tmp1, 32 * 0).write(m[9]); // mova [tmp1q+32*0], m9
        at(tmp1, 32 * 1).write(m[11]); // mova [tmp1q+32*1], m11
        at(tmp1, 32 * 2).write(m[13]); // mova [tmp1q+32*2], m13
        at(tmp1, 32 * 3).write(m[15]); // mova [tmp1q+32*3], m15
        load_16rows(m, stack, at(c, 32), 64, 1, 1); // LOAD_16ROWS cq+32, 64, 1
        idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
        let r2 = at(tmp3, 32 * 8); // lea r2, [tmp3q+32*8]
        m[1] = stack[1]; // mova m1, [rsp+32*1]
        stack[0] = m[6]; // mova [rsp+32*0], m6
        stack[1] = m[7]; // mova [rsp+32*1], m7
        m[7] = pw(16384); // vpbroadcastd m7, [o(pw_16384)]
        inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, stack); // call .transpose_2x8x8_round
        m[15] = stack[0]; // mova m15, [rsp+32*0]
        _mm_storeu_si128(at(r2, -32 * 4).cast(), lo(m[0])); // mova [r2-32*4+ 0], xm0
        _mm_storeu_si128(at(r2, 32 * 0).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [r2+32*0+ 0], m0, 1
        _mm_storeu_si128(at(r2, -32 * 3).cast(), lo(m[2])); // mova [r2-32*3+ 0], xm2
        _mm_storeu_si128(at(r2, 32 * 1).cast(), _mm256_extracti128_si256::<1>(m[2])); // vextracti128 [r2+32*1+ 0], m2, 1
        _mm_storeu_si128(at(r2, -32 * 2).cast(), lo(m[4])); // mova [r2-32*2+ 0], xm4
        _mm_storeu_si128(at(r2, 32 * 2).cast(), _mm256_extracti128_si256::<1>(m[4])); // vextracti128 [r2+32*2+ 0], m4, 1
        _mm_storeu_si128(at(r2, -32 * 1).cast(), lo(m[6])); // mova [r2-32*1+ 0], xm6
        _mm_storeu_si128(at(r2, 32 * 3).cast(), _mm256_extracti128_si256::<1>(m[6])); // vextracti128 [r2+32*3+ 0], m6, 1
        _mm_storeu_si128(at(r2, -32 * 4 + 16).cast(), lo(m[8])); // mova [r2-32*4+16], xm8
        _mm_storeu_si128(at(r2, 32 * 0 + 16).cast(), _mm256_extracti128_si256::<1>(m[8])); // vextracti128 [r2+32*0+16], m8, 1
        _mm_storeu_si128(at(r2, -32 * 3 + 16).cast(), lo(m[10])); // mova [r2-32*3+16], xm10
        _mm_storeu_si128(at(r2, 32 * 1 + 16).cast(), _mm256_extracti128_si256::<1>(m[10])); // vextracti128 [r2+32*1+16], m10, 1
        _mm_storeu_si128(at(r2, -32 * 2 + 16).cast(), lo(m[12])); // mova [r2-32*2+16], xm12
        _mm_storeu_si128(at(r2, 32 * 2 + 16).cast(), _mm256_extracti128_si256::<1>(m[12])); // vextracti128 [r2+32*2+16], m12, 1
        _mm_storeu_si128(at(r2, -32 * 1 + 16).cast(), lo(m[14])); // mova [r2-32*1+16], xm14
        _mm_storeu_si128(at(r2, 32 * 3 + 16).cast(), _mm256_extracti128_si256::<1>(m[14])); // vextracti128 [r2+32*3+16], m14, 1
        m[8] = _mm256_inserti128_si256::<1>(m[1], lo(m[9])); // vinserti128 m8, m1, xm9, 1
        m[12] = _mm256_permute2x128_si256::<0x31>(m[1], m[9]); // vperm2i128 m12, m1, m9, 0x31
        m[0] = zx(_mm_loadu_si128(at(tmp1, -32 * 4).cast())); // mova xm0, [tmp1q-32*4]
        m[1] = zx(_mm_loadu_si128(at(tmp1, -32 * 3).cast())); // mova xm1, [tmp1q-32*3]
        m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(tmp1, 32 * 0).cast())); // vinserti128 m0, [tmp1q+32*0], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(tmp1, 32 * 1).cast())); // vinserti128 m1, [tmp1q+32*1], 1
        m[10] = _mm256_inserti128_si256::<1>(m[5], lo(m[13])); // vinserti128 m10, m5, xm13, 1
        m[14] = _mm256_permute2x128_si256::<0x31>(m[5], m[13]); // vperm2i128 m14, m5, m13, 0x31
        m[4] = zx(_mm_loadu_si128(at(tmp1, -32 * 4 + 16).cast())); // mova xm4, [tmp1q-32*4+16]
        m[5] = zx(_mm_loadu_si128(at(tmp1, -32 * 3 + 16).cast())); // mova xm5, [tmp1q-32*3+16]
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(tmp1, 32 * 0 + 16).cast())); // vinserti128 m4, [tmp1q+32*0+16], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(tmp1, 32 * 1 + 16).cast())); // vinserti128 m5, [tmp1q+32*1+16], 1
        m[9] = _mm256_inserti128_si256::<1>(m[3], lo(m[11])); // vinserti128 m9, m3, xm11, 1
        m[13] = _mm256_permute2x128_si256::<0x31>(m[3], m[11]); // vperm2i128 m13, m3, m11, 0x31
        m[2] = zx(_mm_loadu_si128(at(tmp1, -32 * 2).cast())); // mova xm2, [tmp1q-32*2]
        m[3] = zx(_mm_loadu_si128(at(tmp1, -32 * 1).cast())); // mova xm3, [tmp1q-32*1]
        m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(tmp1, 32 * 2).cast())); // vinserti128 m2, [tmp1q+32*2], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(tmp1, 32 * 3).cast())); // vinserti128 m3, [tmp1q+32*3], 1
        m[11] = _mm256_inserti128_si256::<1>(m[7], lo(m[15])); // vinserti128 m11, m7, xm15, 1
        m[15] = _mm256_permute2x128_si256::<0x31>(m[7], m[15]); // vperm2i128 m15, m7, m15, 0x31
        m[6] = zx(_mm_loadu_si128(at(tmp1, -32 * 2 + 16).cast())); // mova xm6, [tmp1q-32*2+16]
        m[7] = zx(_mm_loadu_si128(at(tmp1, -32 * 1 + 16).cast())); // mova xm7, [tmp1q-32*1+16]
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(tmp1, 32 * 2 + 16).cast())); // vinserti128 m6, [tmp1q+32*2+16], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(tmp1, 32 * 3 + 16).cast())); // vinserti128 m7, [tmp1q+32*3+16], 1
        inv_txfm_add_dct_dct_16x32_main_oddhalf(m, stack, tmp1, tmp2); // call .main_oddhalf
        load_8rows_h(m, at(r2, -32 * 4), 32, 0); // LOAD_8ROWS_H r2-32*4, 32
    }
    // .idct16:
    load_8rows(m, at(tmp3, -32 * 4), 32, 0); // LOAD_8ROWS tmp3q-32*4, 32
    stack[0] = m[15]; // mova [rsp], m15
    idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
    let mut r2 = at(dst, stride * 19); // imul r2, strideq, 19 / add r2, dstq
    let r3 = stride * 3; // lea r3, [strideq*3]
    inv_txfm_add_dct_dct_16x32_pass2_end(m, stack, &mut dst, stride, &mut r2, r3, tmp1, tmp2);
    // call .pass2_end
}

/// `.main_oddhalf_fast` of `inv_txfm_add_dct_dct_16x32_8bpc` ("lower half is zero"): the odd half of the
/// 32-point DCT for in1, in3, …, in15 in `m0`-`m7`; falls into `.main2`.
#[inline(always)]
pub(crate) unsafe fn inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(
    m: &mut M,
    stack: &mut [__m256i; 3],
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
) {
    stack[1] = m[7]; // mova [rsp+gprsize+32*1], m7
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    stack[0] = m[7]; // mova [rsp+gprsize+32*0], m7
    stack[2] = m[7]; // mova [rsp+gprsize+32*2], m7
    m[11] = pw(3703 * 8); // vpbroadcastd m11, [o(pw_3703x8)]
    m[7] = pw(1751 * 8); // vpbroadcastd m7, [o(pw_1751x8)]
    m[12] = pw(-1380 * 8); // vpbroadcastd m12, [o(pw_m1380x8)]
    m[8] = pw(3857 * 8); // vpbroadcastd m8, [o(pw_3857x8)]
    m[13] = pw(3973 * 8); // vpbroadcastd m13, [o(pw_3973x8)]
    m[15] = pw(995 * 8); // vpbroadcastd m15, [o(pw_995x8)]
    m[11] = _mm256_mulhrs_epi16(m[11], m[4]); // pmulhrsw m11, m4 ; t29a
    m[4] = _mm256_mulhrs_epi16(m[4], m[7]); // pmulhrsw m4, m7 ; t18a
    m[12] = _mm256_mulhrs_epi16(m[12], m[3]); // pmulhrsw m12, m3 ; t19a
    m[3] = _mm256_mulhrs_epi16(m[3], m[8]); // pmulhrsw m3, m8 ; t28a
    m[13] = _mm256_mulhrs_epi16(m[13], m[2]); // pmulhrsw m13, m2 ; t27a
    m[2] = _mm256_mulhrs_epi16(m[2], m[15]); // pmulhrsw m2, m15 ; t20a
    m[10] = pw(-2106 * 8); // vpbroadcastd m10, [o(pw_m2106x8)]
    m[7] = pw(3513 * 8); // vpbroadcastd m7, [o(pw_3513x8)]
    m[9] = pw(3290 * 8); // vpbroadcastd m9, [o(pw_3290x8)]
    m[8] = pw(2440 * 8); // vpbroadcastd m8, [o(pw_2440x8)]
    m[14] = pw(-601 * 8); // vpbroadcastd m14, [o(pw_m601x8)]
    m[15] = pw(4052 * 8); // vpbroadcastd m15, [o(pw_4052x8)]
    m[10] = _mm256_mulhrs_epi16(m[10], m[5]); // pmulhrsw m10, m5 ; t21a
    m[5] = _mm256_mulhrs_epi16(m[5], m[7]); // pmulhrsw m5, m7 ; t26a
    m[9] = _mm256_mulhrs_epi16(m[9], m[6]); // pmulhrsw m9, m6 ; t25a
    m[6] = _mm256_mulhrs_epi16(m[6], m[8]); // pmulhrsw m6, m8 ; t22a
    m[14] = _mm256_mulhrs_epi16(m[14], m[1]); // pmulhrsw m14, m1 ; t23a
    m[1] = _mm256_mulhrs_epi16(m[1], m[15]); // pmulhrsw m1, m15 ; t24a
    m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
    inv_txfm_add_dct_dct_16x32_main2(m, stack, tmp1, tmp2) // jmp .main2
}

/// `.main_oddhalf` of `inv_txfm_add_dct_dct_16x32_8bpc`: the odd half of the 32-point DCT for in1, in3, …, in31
/// in `m0`-`m15`. Uses the three spill slots `[rsp+gprsize+32*0..3]` (`stack`) and stores t16 … t31 to
/// `[tmp1q-32*4]` … `[tmp1q+32*3]` and `[tmp2q-32*4]` … `[tmp2q+32*3]` (all sixteen, on both entries).
#[inline(always)]
pub(crate) unsafe fn inv_txfm_add_dct_dct_16x32_main_oddhalf(
    m: &mut M,
    stack: &mut [__m256i; 3],
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
) {
    stack[0] = m[15]; // mova [rsp+gprsize+32*0], m15
    stack[1] = m[7]; // mova [rsp+gprsize+32*1], m7
    stack[2] = m[8]; // mova [rsp+gprsize+32*2], m8
    m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
    itx_mulsub_2w(m, 4, 11, 7, 8, 15, 1751, 3703, None); // ITX_MULSUB_2W 4, 11, 7, 8, 15, 1751, 3703 ; t18a, t29a
    itx_mulsub_2w(m, 12, 3, 7, 8, 15, 3857, 1380, None); // ITX_MULSUB_2W 12, 3, 7, 8, 15, 3857, 1380 ; t19a, t28a
    itx_mulsub_2w(m, 2, 13, 7, 8, 15, 995, 3973, None); // ITX_MULSUB_2W 2, 13, 7, 8, 15, 995, 3973 ; t20a, t27a
    itx_mulsub_2w(m, 10, 5, 7, 8, 15, 3513, 2106, None); // ITX_MULSUB_2W 10, 5, 7, 8, 15, 3513, 2106 ; t21a, t26a
    itx_mulsub_2w(m, 6, 9, 7, 8, 15, 2440, 3290, None); // ITX_MULSUB_2W 6, 9, 7, 8, 15, 2440, 3290 ; t22a, t25a
    itx_mulsub_2w(m, 14, 1, 7, 8, 15, 4052, 601, None); // ITX_MULSUB_2W 14, 1, 7, 8, 15, 4052, 601 ; t23a, t24a
    inv_txfm_add_dct_dct_16x32_main2(m, stack, tmp1, tmp2)
}

/// `.main2` of `inv_txfm_add_dct_dct_16x32_8bpc` (the common tail of `.main_oddhalf` and `.main_oddhalf_fast`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32_main2(
    m: &mut M,
    stack: &mut [__m256i; 3],
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
) {
    m[7] = _mm256_subs_epi16(m[12], m[4]); // psubsw m7, m12, m4 ; t18
    m[12] = _mm256_adds_epi16(m[12], m[4]); // paddsw m12, m4 ; t19
    m[4] = _mm256_subs_epi16(m[2], m[10]); // psubsw m4, m2, m10 ; t21
    m[2] = _mm256_adds_epi16(m[2], m[10]); // paddsw m2, m10 ; t20
    m[10] = _mm256_subs_epi16(m[14], m[6]); // psubsw m10, m14, m6 ; t22
    m[14] = _mm256_adds_epi16(m[14], m[6]); // paddsw m14, m6 ; t23
    m[6] = _mm256_subs_epi16(m[1], m[9]); // psubsw m6, m1, m9 ; t25
    m[1] = _mm256_adds_epi16(m[1], m[9]); // paddsw m1, m9 ; t24
    m[9] = _mm256_subs_epi16(m[13], m[5]); // psubsw m9, m13, m5 ; t26
    m[13] = _mm256_adds_epi16(m[13], m[5]); // paddsw m13, m5 ; t27
    m[5] = _mm256_subs_epi16(m[3], m[11]); // psubsw m5, m3, m11 ; t29
    m[3] = _mm256_adds_epi16(m[3], m[11]); // paddsw m3, m11 ; t28
    itx_mulsub_2w(m, 5, 7, 8, 11, 15, -4017, 799, None); // ITX_MULSUB_2W 5, 7, 8, 11, 15, m4017, 799 ; t18a, t29a
    itx_mulsub_2w(m, 9, 4, 8, 11, 15, 3406, 2276, None); // ITX_MULSUB_2W 9, 4, 8, 11, 15, 3406, 2276 ; t21a, t26a
    itx_mulsub_2w(m, 6, 10, 8, 11, 15, -2276, 3406, None); // ITX_MULSUB_2W 6, 10, 8, 11, 15, m2276, 3406 ; t22a, t25a
    m[8] = _mm256_subs_epi16(m[14], m[2]); // psubsw m8, m14, m2 ; t20a
    m[14] = _mm256_adds_epi16(m[14], m[2]); // paddsw m14, m2 ; t23a
    m[2] = _mm256_subs_epi16(m[1], m[13]); // psubsw m2, m1, m13 ; t27a
    m[1] = _mm256_adds_epi16(m[1], m[13]); // paddsw m1, m13 ; t24a
    m[13] = _mm256_subs_epi16(m[6], m[9]); // psubsw m13, m6, m9 ; t21
    m[6] = _mm256_adds_epi16(m[6], m[9]); // paddsw m6, m9 ; t22
    m[9] = _mm256_subs_epi16(m[10], m[4]); // psubsw m9, m10, m4 ; t26
    m[10] = _mm256_adds_epi16(m[10], m[4]); // paddsw m10, m4 ; t25
    itx_mulsub_2w(m, 2, 8, 4, 11, 15, -3784, 1567, None); // ITX_MULSUB_2W 2, 8, 4, 11, 15, m3784, 1567 ; t20, t27
    itx_mulsub_2w(m, 9, 13, 4, 11, 15, -3784, 1567, None); // ITX_MULSUB_2W 9, 13, 4, 11, 15, m3784, 1567 ; t21a, t26a
    m[4] = stack[0]; // mova m4, [rsp+gprsize+32*0] ; in31
    stack[0] = m[6]; // mova [rsp+gprsize+32*0], m6 ; t22
    m[6] = stack[1]; // mova m6, [rsp+gprsize+32*1] ; in15
    stack[1] = m[14]; // mova [rsp+gprsize+32*1], m14 ; t23a
    m[14] = stack[2]; // mova m14, [rsp+gprsize+32*2] ; in17
    stack[2] = m[1]; // mova [rsp+gprsize+32*2], m1 ; t24a
    itx_mulsub_2w(m, 0, 4, 1, 11, 15, 201, 4091, None); // ITX_MULSUB_2W 0, 4, 1, 11, 15, 201, 4091 ; t16a, t31a
    itx_mulsub_2w(m, 14, 6, 1, 11, 15, 3035, 2751, None); // ITX_MULSUB_2W 14, 6, 1, 11, 15, 3035, 2751 ; t17a, t30a
    m[1] = _mm256_subs_epi16(m[0], m[14]); // psubsw m1, m0, m14 ; t17
    m[0] = _mm256_adds_epi16(m[0], m[14]); // paddsw m0, m14 ; t16
    m[14] = _mm256_subs_epi16(m[4], m[6]); // psubsw m14, m4, m6 ; t30
    m[4] = _mm256_adds_epi16(m[4], m[6]); // paddsw m4, m6 ; t31
    itx_mulsub_2w(m, 14, 1, 6, 11, 15, 799, 4017, None); // ITX_MULSUB_2W 14, 1, 6, 11, 15, 799, 4017 ; t17a, t30a
    m[6] = _mm256_subs_epi16(m[0], m[12]); // psubsw m6, m0, m12 ; t19a
    m[0] = _mm256_adds_epi16(m[0], m[12]); // paddsw m0, m12 ; t16a
    m[12] = _mm256_subs_epi16(m[4], m[3]); // psubsw m12, m4, m3 ; t28a
    m[4] = _mm256_adds_epi16(m[4], m[3]); // paddsw m4, m3 ; t31a
    m[3] = _mm256_subs_epi16(m[14], m[5]); // psubsw m3, m14, m5 ; t18
    m[14] = _mm256_adds_epi16(m[14], m[5]); // paddsw m14, m5 ; t17
    m[5] = _mm256_subs_epi16(m[1], m[7]); // psubsw m5, m1, m7 ; t29
    m[1] = _mm256_adds_epi16(m[1], m[7]); // paddsw m1, m7 ; t30
    itx_mulsub_2w(m, 5, 3, 7, 11, 15, 1567, 3784, None); // ITX_MULSUB_2W 5, 3, 7, 11, 15, 1567, 3784 ; t18a, t29a
    itx_mulsub_2w(m, 12, 6, 7, 11, 15, 1567, 3784, None); // ITX_MULSUB_2W 12, 6, 7, 11, 15, 1567, 3784 ; t19, t28
    m[7] = _mm256_subs_epi16(m[1], m[10]); // psubsw m7, m1, m10 ; t25a
    m[1] = _mm256_adds_epi16(m[1], m[10]); // paddsw m1, m10 ; t30a
    m[10] = _mm256_subs_epi16(m[5], m[9]); // psubsw m10, m5, m9 ; t21
    m[5] = _mm256_adds_epi16(m[5], m[9]); // paddsw m5, m9 ; t18
    m[9] = _mm256_subs_epi16(m[12], m[2]); // psubsw m9, m12, m2 ; t20a
    m[12] = _mm256_adds_epi16(m[12], m[2]); // paddsw m12, m2 ; t19a
    m[2] = _mm256_subs_epi16(m[3], m[13]); // psubsw m2, m3, m13 ; t26
    m[3] = _mm256_adds_epi16(m[3], m[13]); // paddsw m3, m13 ; t29
    m[13] = _mm256_subs_epi16(m[6], m[8]); // psubsw m13, m6, m8 ; t27a
    m[6] = _mm256_adds_epi16(m[6], m[8]); // paddsw m6, m8 ; t28a
    at(tmp1, -32 * 2).write(m[5]); // mova [tmp1q-32*2], m5
    at(tmp1, -32 * 1).write(m[12]); // mova [tmp1q-32*1], m12
    at(tmp2, 32 * 0).write(m[6]); // mova [tmp2q+32*0], m6
    at(tmp2, 32 * 1).write(m[3]); // mova [tmp2q+32*1], m3
    at(tmp2, 32 * 2).write(m[1]); // mova [tmp2q+32*2], m1
    m[5] = stack[0]; // mova m5, [rsp+gprsize+32*0] ; t22
    m[6] = stack[1]; // mova m6, [rsp+gprsize+32*1] ; t23
    m[3] = stack[2]; // mova m3, [rsp+gprsize+32*2] ; t24a
    m[1] = _mm256_subs_epi16(m[14], m[5]); // psubsw m1, m14, m5 ; t22a
    m[14] = _mm256_adds_epi16(m[14], m[5]); // paddsw m14, m5 ; t17a
    m[5] = _mm256_subs_epi16(m[0], m[6]); // psubsw m5, m0, m6 ; t23
    m[0] = _mm256_adds_epi16(m[0], m[6]); // paddsw m0, m6 ; t16
    m[6] = _mm256_subs_epi16(m[4], m[3]); // psubsw m6, m4, m3 ; t24
    m[4] = _mm256_adds_epi16(m[4], m[3]); // paddsw m4, m3 ; t31
    m[8] = pw2(-2896, 2896); // vpbroadcastd m8, [o(pw_m2896_2896)]
    m[3] = pw2(2896, 2896); // vpbroadcastd m3, [o(pw_2896_2896)]
    at(tmp1, -32 * 4).write(m[0]); // mova [tmp1q-32*4], m0
    at(tmp1, -32 * 3).write(m[14]); // mova [tmp1q-32*3], m14
    at(tmp2, 32 * 3).write(m[4]); // mova [tmp2q+32*3], m4
    itx_mulsub_2w(m, 13, 9, 0, 4, 15, 3, 8, None); // ITX_MULSUB_2W 13, 9, 0, 4, 15, 3, 8 ; t20, t27
    itx_mulsub_2w(m, 2, 10, 0, 4, 15, 3, 8, None); // ITX_MULSUB_2W 2, 10, 0, 4, 15, 3, 8 ; t21a, t26a
    itx_mulsub_2w(m, 7, 1, 0, 4, 15, 3, 8, None); // ITX_MULSUB_2W 7, 1, 0, 4, 15, 3, 8 ; t22, t25
    itx_mulsub_2w(m, 6, 5, 0, 4, 15, 3, 8, None); // ITX_MULSUB_2W 6, 5, 0, 4, 15, 3, 8 ; t23a, t24a
    at(tmp1, 32 * 0).write(m[13]); // mova [tmp1q+32*0], m13
    at(tmp1, 32 * 1).write(m[2]); // mova [tmp1q+32*1], m2
    at(tmp1, 32 * 2).write(m[7]); // mova [tmp1q+32*2], m7
    at(tmp1, 32 * 3).write(m[6]); // mova [tmp1q+32*3], m6
    at(tmp2, -32 * 4).write(m[5]); // mova [tmp2q-32*4], m5
    at(tmp2, -32 * 3).write(m[1]); // mova [tmp2q-32*3], m1
    at(tmp2, -32 * 2).write(m[10]); // mova [tmp2q-32*2], m10
    at(tmp2, -32 * 1).write(m[9]); // mova [tmp2q-32*1], m9
}

/// `.transpose_2x8x8_round` of `inv_txfm_add_dct_dct_16x32_8bpc`: `m7` is the rounding factor, the values of
/// `m6` and `m7` are in `[rsp+gprsize+32*0]` and `[rsp+gprsize+32*1]`; on return `m15` is in
/// `[rsp+gprsize+32*0]`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m: &mut M, stack: &mut [__m256i; 3]) {
    m[6] = _mm256_unpackhi_epi16(m[12], m[13]); // punpckhwd m6, m12, m13
    m[12] = _mm256_unpacklo_epi16(m[12], m[13]); // punpcklwd m12, m13
    m[13] = _mm256_unpackhi_epi16(m[8], m[9]); // punpckhwd m13, m8, m9
    m[8] = _mm256_unpacklo_epi16(m[8], m[9]); // punpcklwd m8, m9
    m[9] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m9, m14, m15
    m[14] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m14, m15
    m[15] = _mm256_unpackhi_epi16(m[10], m[11]); // punpckhwd m15, m10, m11
    m[10] = _mm256_unpacklo_epi16(m[10], m[11]); // punpcklwd m10, m11
    repx!([0, 1, 2, 3, 4, 5], r => m[r] = _mm256_mulhrs_epi16(m[r], m[7])); // REPX {pmulhrsw x, m7}, m0, m1, m2, m3, m4, m5
    m[11] = _mm256_unpackhi_epi32(m[8], m[10]); // punpckhdq m11, m8, m10
    m[8] = _mm256_unpacklo_epi32(m[8], m[10]); // punpckldq m8, m10
    m[10] = _mm256_unpacklo_epi32(m[12], m[14]); // punpckldq m10, m12, m14
    m[12] = _mm256_unpackhi_epi32(m[12], m[14]); // punpckhdq m12, m14
    m[14] = _mm256_unpackhi_epi32(m[13], m[15]); // punpckhdq m14, m13, m15
    m[13] = _mm256_unpacklo_epi32(m[13], m[15]); // punpckldq m13, m15
    m[15] = _mm256_unpacklo_epi32(m[6], m[9]); // punpckldq m15, m6, m9
    m[6] = _mm256_unpackhi_epi32(m[6], m[9]); // punpckhdq m6, m9
    m[9] = _mm256_unpackhi_epi64(m[8], m[10]); // punpckhqdq m9, m8, m10
    m[8] = _mm256_unpacklo_epi64(m[8], m[10]); // punpcklqdq m8, m10
    m[10] = _mm256_unpacklo_epi64(m[11], m[12]); // punpcklqdq m10, m11, m12
    m[11] = _mm256_unpackhi_epi64(m[11], m[12]); // punpckhqdq m11, m12
    m[12] = _mm256_unpacklo_epi64(m[13], m[15]); // punpcklqdq m12, m13, m15
    m[13] = _mm256_unpackhi_epi64(m[13], m[15]); // punpckhqdq m13, m15
    m[15] = _mm256_unpackhi_epi64(m[14], m[6]); // punpckhqdq m15, m14, m6
    m[14] = _mm256_unpacklo_epi64(m[14], m[6]); // punpcklqdq m14, m6
    m[6] = _mm256_mulhrs_epi16(m[7], stack[0]); // pmulhrsw m6, m7, [rsp+gprsize+32*0]
    repx!([8, 9, 10, 11, 12, 13, 14, 15], r => m[r] = _mm256_mulhrs_epi16(m[r], m[7])); // REPX {pmulhrsw x, m7}, m8, m9, m10, m11, m12, m13, m14, m15
    m[7] = _mm256_mulhrs_epi16(m[7], stack[1]); // pmulhrsw m7, [rsp+gprsize+32*1]
    stack[0] = m[15]; // mova [rsp+gprsize+32*0], m15
    m[15] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m15, m4, m5
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
    m[5] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m5, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m1, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m7, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m2, m4, m6
    m[4] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m4, m6
    m[6] = _mm256_unpackhi_epi32(m[5], m[7]); // punpckhdq m6, m5, m7
    m[5] = _mm256_unpacklo_epi32(m[5], m[7]); // punpckldq m5, m7
    m[7] = _mm256_unpacklo_epi32(m[15], m[1]); // punpckldq m7, m15, m1
    m[15] = _mm256_unpackhi_epi32(m[15], m[1]); // punpckhdq m15, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
    m[4] = _mm256_unpacklo_epi64(m[5], m[7]); // punpcklqdq m4, m5, m7
    m[5] = _mm256_unpackhi_epi64(m[5], m[7]); // punpckhqdq m5, m7
    m[7] = _mm256_unpackhi_epi64(m[6], m[15]); // punpckhqdq m7, m6, m15
    m[6] = _mm256_unpacklo_epi64(m[6], m[15]); // punpcklqdq m6, m15
}

/// `.pass2_end` of `inv_txfm_add_dct_dct_16x32_8bpc`: out1 is in `[rsp+gprsize+32*1]`; `r2` is
/// `dstq + strideq * 19` and `r3` is `strideq * 3`. `dstq` is advanced by three rows and `r2` moved back by
/// three, as the assembly leaves them (the 32-pixel-wide callers continue from there).
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32_pass2_end(
    m: &mut M,
    stack: &mut [__m256i; 3],
    dst: &mut Dst,
    stride: isize,
    r2: &mut Dst,
    r3: isize,
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
) {
    stack[0] = m[7]; // mova [rsp+gprsize+32*0], m7
    stack[2] = m[15]; // mova [rsp+gprsize+32*2], m15
    m[15] = pw(2048); // vpbroadcastd m15, [o(pw_2048)]
    idct32_pass2_end(m, *dst, *r2, 0, at(tmp2, 32 * 3), 1, 7, 15, 0, r3 * 4); // IDCT32_PASS2_END 0, tmp2q+32*3, 1, 7, 15, strideq*0, r3*4
    idct32_pass2_end(m, *dst, *r2, 4, at(tmp2, -32 * 1), 0, 7, 15, stride * 4, stride * 8); // IDCT32_PASS2_END 4, tmp2q-32*1, 0, 7, 15, strideq*4, strideq*8
    idct32_pass2_end(m, *dst, *r2, 8, at(tmp1, 32 * 3), 0, 4, 15, stride * 8, stride * 4); // IDCT32_PASS2_END 8, tmp1q+32*3, 0, 4, 15, strideq*8, strideq*4
    idct32_pass2_end(m, *dst, *r2, 12, at(tmp1, -32 * 1), 0, 4, 15, r3 * 4, 0); // IDCT32_PASS2_END 12, tmp1q-32*1, 0, 4, 15, r3*4, strideq*0
    *dst = at(*dst, stride); // add dstq, strideq
    *r2 = at(*r2, -stride); // sub r2, strideq
    m[1] = stack[1]; // mova m1, [rsp+gprsize+32*1]
    idct32_pass2_end(m, *dst, *r2, 1, at(tmp2, 32 * 2), 0, 4, 15, 0, r3 * 4); // IDCT32_PASS2_END 1, tmp2q+32*2, 0, 4, 15, strideq*0, r3*4
    idct32_pass2_end(m, *dst, *r2, 5, at(tmp2, -32 * 2), 0, 4, 15, stride * 4, stride * 8); // IDCT32_PASS2_END 5, tmp2q-32*2, 0, 4, 15, strideq*4, strideq*8
    idct32_pass2_end(m, *dst, *r2, 9, at(tmp1, 32 * 2), 0, 4, 15, stride * 8, stride * 4); // IDCT32_PASS2_END 9, tmp1q+32*2, 0, 4, 15, strideq*8, strideq*4
    idct32_pass2_end(m, *dst, *r2, 13, at(tmp1, -32 * 2), 0, 4, 15, r3 * 4, 0); // IDCT32_PASS2_END 13, tmp1q-32*2, 0, 4, 15, r3*4, strideq*0
    *dst = at(*dst, stride); // add dstq, strideq
    *r2 = at(*r2, -stride); // sub r2, strideq
    idct32_pass2_end(m, *dst, *r2, 2, at(tmp2, 32 * 1), 0, 4, 15, 0, r3 * 4); // IDCT32_PASS2_END 2, tmp2q+32*1, 0, 4, 15, strideq*0, r3*4
    idct32_pass2_end(m, *dst, *r2, 6, at(tmp2, -32 * 3), 0, 4, 15, stride * 4, stride * 8); // IDCT32_PASS2_END 6, tmp2q-32*3, 0, 4, 15, strideq*4, strideq*8
    idct32_pass2_end(m, *dst, *r2, 10, at(tmp1, 32 * 1), 0, 4, 15, stride * 8, stride * 4); // IDCT32_PASS2_END 10, tmp1q+32*1, 0, 4, 15, strideq*8, strideq*4
    idct32_pass2_end(m, *dst, *r2, 14, at(tmp1, -32 * 3), 0, 4, 15, r3 * 4, 0); // IDCT32_PASS2_END 14, tmp1q-32*3, 0, 4, 15, r3*4, strideq*0
    *dst = at(*dst, stride); // add dstq, strideq
    *r2 = at(*r2, -stride); // sub r2, strideq
    m[7] = stack[0]; // mova m7, [rsp+gprsize+32*0]
    m[1] = stack[2]; // mova m1, [rsp+gprsize+32*2]
    idct32_pass2_end(m, *dst, *r2, 3, at(tmp2, 32 * 0), 0, 4, 15, 0, r3 * 4); // IDCT32_PASS2_END 3, tmp2q+32*0, 0, 4, 15, strideq*0, r3*4
    idct32_pass2_end(m, *dst, *r2, 7, at(tmp2, -32 * 4), 0, 4, 15, stride * 4, stride * 8); // IDCT32_PASS2_END 7, tmp2q-32*4, 0, 4, 15, strideq*4, strideq*8
    idct32_pass2_end(m, *dst, *r2, 11, at(tmp1, 32 * 0), 0, 4, 15, stride * 8, stride * 4); // IDCT32_PASS2_END 11, tmp1q+32*0, 0, 4, 15, strideq*8, strideq*4
    idct32_pass2_end(m, *dst, *r2, 1, at(tmp1, -32 * 4), 0, 4, 15, r3 * 4, 0); // IDCT32_PASS2_END 1, tmp1q-32*4, 0, 4, 15, r3*4, strideq*0
}

/// `%macro IDCT32_PASS1_END row[1-2], tmp[1-2]` ("Perform the final sumsub step and YMM lane shuffling";
/// register numbers)
#[inline(always)]
unsafe fn idct32_pass1_end(
    m: &mut M,
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
    r1: usize,
    r2: usize,
    t1: usize,
    t2: usize,
) {
    let a = at(tmp2, 32 * (3 - r1 as isize)); // [tmp2q+32*( 3-%1)]
    let b = at(tmp1, 32 * (11 - r2 as isize)); // [tmp1q+32*(11-%2)]
    m[t1] = a.read(); // mova m%3, [tmp2q+32*( 3-%1)]
    m[t2] = _mm256_subs_epi16(m[r1], m[t1]); // psubsw m%4, m%1, m%3
    m[r1] = _mm256_adds_epi16(m[r1], m[t1]); // paddsw m%1, m%3
    m[t1] = b.read(); // mova m%3, [tmp1q+32*(11-%2)]
    _mm_storeu_si128(at(b, 16).cast(), lo(m[t2])); // mova [tmp1q+32*(11-%2)+16], xm%4
    _mm_storeu_si128(at(a, 16).cast(), _mm256_extracti128_si256::<1>(m[t2])); // vextracti128 [tmp2q+32*( 3-%1)+16], m%4, 1
    m[t2] = _mm256_adds_epi16(m[r2], m[t1]); // paddsw m%4, m%2, m%3
    m[r2] = _mm256_subs_epi16(m[r2], m[t1]); // psubsw m%2, m%3
    _mm_storeu_si128(b.cast(), lo(m[r2])); // mova [tmp1q+32*(11-%2)], xm%2
    _mm_storeu_si128(a.cast(), _mm256_extracti128_si256::<1>(m[r2])); // vextracti128 [tmp2q+32*( 3-%1)], m%2, 1
    m[r2] = _mm256_permute2x128_si256::<0x31>(m[r1], m[t2]); // vperm2i128 m%2, m%1, m%4, 0x31
    m[r1] = _mm256_inserti128_si256::<1>(m[r1], lo(m[t2])); // vinserti128 m%1, xm%4, 1
}

/// `cglobal inv_txfm_add_dct_dct_32x16_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        let r3d = eob | 16; // or r3d, 16
        return inv_txfm_add_dct_dct_32x8_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_32x8_8bpc).dconly
    }
    // .normal:
    // PROLOGUE 0, 6, 16, 32*19, dst, stride, c, eob, tmp1, tmp2
    let mut stack = [_mm256_setzero_si256(); 3]; // [rsp+32*0..3]
    let stack = &mut stack;
    let mut frame = core::mem::MaybeUninit::<[__m256i; 16]>::uninit(); // [rsp+32*3..19]
    let frame0: *mut __m256i = zeroed_frame(&mut frame); // zeroed whole (see above)
    m[15] = pw(2896 * 8); // vpbroadcastd m15, [o(pw_2896x8)]
    m[0] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulhrsw m0, m15, [cq+32* 1]
    m[1] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulhrsw m1, m15, [cq+32* 3]
    m[2] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 5).cast())); // pmulhrsw m2, m15, [cq+32* 5]
    m[3] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 7).cast())); // pmulhrsw m3, m15, [cq+32* 7]
    m[4] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 9).cast())); // pmulhrsw m4, m15, [cq+32* 9]
    m[5] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 11).cast())); // pmulhrsw m5, m15, [cq+32*11]
    m[6] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 13).cast())); // pmulhrsw m6, m15, [cq+32*13]
    m[7] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 15).cast())); // pmulhrsw m7, m15, [cq+32*15]
    m[8] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 17).cast())); // pmulhrsw m8, m15, [cq+32*17]
    m[9] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 19).cast())); // pmulhrsw m9, m15, [cq+32*19]
    m[10] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 21).cast())); // pmulhrsw m10, m15, [cq+32*21]
    m[11] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 23).cast())); // pmulhrsw m11, m15, [cq+32*23]
    m[12] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 25).cast())); // pmulhrsw m12, m15, [cq+32*25]
    m[13] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 27).cast())); // pmulhrsw m13, m15, [cq+32*27]
    m[14] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 29).cast())); // pmulhrsw m14, m15, [cq+32*29]
    m[15] = _mm256_mulhrs_epi16(m[15], _mm256_loadu_si256(at(c, 32 * 31).cast())); // pmulhrsw m15, [cq+32*31]
    let tmp1 = at(frame0, 32 * 4); // lea tmp1q, [rsp+32*7]
    let tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
    inv_txfm_add_dct_dct_16x32_main_oddhalf(m, stack, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
    load_16rows(m, stack, c, 32 * 2, 1, 0); // LOAD_16ROWS cq+32*0, 32*2, 1, 0
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r3d = 8; // mov r3d, 8
    loop {
        // .zero_loop:
        _mm256_storeu_si256(at(c, 32 * 0).cast(), m[15]); // mova [cq+32*0], m15
        _mm256_storeu_si256(at(c, 32 * 1).cast(), m[15]); // mova [cq+32*1], m15
        _mm256_storeu_si256(at(c, 32 * 2).cast(), m[15]); // mova [cq+32*2], m15
        _mm256_storeu_si256(at(c, 32 * 3).cast(), m[15]); // mova [cq+32*3], m15
        c = at(c, 32 * 4); // add cq, 32*4
        r3d -= 1; // dec r3d
        if r3d <= 0 {
            break; // jg .zero_loop
        }
    }
    idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
    inv_txfm_add_dct_dct_32x16_pass1_end(m, stack, tmp1, tmp2); // call .pass1_end
    let r2 = stride * 3; // lea r2, [strideq*3]
    let mut r3 = dst; // mov r3, dstq
    loop {
        // .pass2:
        m[7] = pw(16384); // vpbroadcastd m7, [o(pw_16384)]
        inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, stack); // call m(inv_txfm_add_dct_dct_16x32_8bpc).transpose_2x8x8_round
        idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
        stack[2] = m[15]; // mova [rsp+32*2], m15
        m[15] = pw(2048); // vpbroadcastd m15, [o(pw_2048)]
        repx!([2, 3, 0], r => m[r] = _mm256_mulhrs_epi16(m[r], m[15])); // REPX {pmulhrsw x, m15}, m2, m3, m0
        write_16x2(m, dst, m[2], m[3], 1, 2, [stride * 2, r2]); // WRITE_16X2 2, 3, 1, 2, strideq*2, r2
        m[1] = _mm256_mulhrs_epi16(m[15], stack[1]); // pmulhrsw m1, m15, [rsp+32*1]
        write_16x2(m, dst, m[0], m[1], 2, 3, [0, stride]); // WRITE_16X2 0, 1, 2, 3, strideq*0, strideq*1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        repx!([4, 5, 6, 7], r => m[r] = _mm256_mulhrs_epi16(m[r], m[15])); // REPX {pmulhrsw x, m15}, m4, m5, m6, m7
        write_16x2(m, dst, m[4], m[5], 2, 3, [0, stride]); // WRITE_16X2 4, 5, 2, 3, strideq*0, strideq*1
        write_16x2(m, dst, m[6], m[7], 2, 3, [stride * 2, r2]); // WRITE_16X2 6, 7, 2, 3, strideq*2, r2
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        repx!([8, 9, 10, 11], r => m[r] = _mm256_mulhrs_epi16(m[r], m[15])); // REPX {pmulhrsw x, m15}, m8, m9, m10, m11
        write_16x2(m, dst, m[8], m[9], 2, 3, [0, stride]); // WRITE_16X2 8, 9, 2, 3, strideq*0, strideq*1
        write_16x2(m, dst, m[10], m[11], 2, 3, [stride * 2, r2]); // WRITE_16X2 10, 11, 2, 3, strideq*2, r2
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        repx!([11, 12, 13, 14], r => m[r] = _mm256_mulhrs_epi16(m[r], m[15])); // REPX {pmulhrsw x, m15}, m11, m12, m13, m14
        m[15] = _mm256_mulhrs_epi16(m[15], stack[2]); // pmulhrsw m15, [rsp+32*2]
        write_16x2(m, dst, m[12], m[13], 2, 3, [0, stride]); // WRITE_16X2 12, 13, 2, 3, strideq*0, strideq*1
        write_16x2(m, dst, m[14], m[15], 2, 3, [stride * 2, r2]); // WRITE_16X2 14, 15, 2, 3, strideq*2, r2
        if r3.is_null() {
            break; // test r3, r3 / jnz .right_half / RET
        }
        // .right_half:
        load_8rows(m, at(tmp1, -32 * 4), 32, 0); // LOAD_8ROWS tmp1q-32*4, 32
        load_8rows_h(m, at(tmp2, -32 * 4), 32, 0); // LOAD_8ROWS_H tmp2q-32*4, 32
        dst = at(r3, 16); // lea dstq, [r3+16]
        r3 = core::ptr::null_mut(); // xor r3d, r3d
        stack[0] = m[6]; // mova [rsp+32*0], m6
        stack[1] = m[7]; // mova [rsp+32*1], m7
                         // jmp .pass2
    }
}

/// `.pass1_end` of `inv_txfm_add_dct_dct_32x16_8bpc`: out1 is in `[rsp+gprsize+32*1]`; on return the values
/// of `m6` and `m7` are in `[rsp+gprsize+32*0]` and `[rsp+gprsize+32*1]` (as `.transpose_2x8x8_round` takes
/// them).
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16_pass1_end(
    m: &mut M,
    stack: &mut [__m256i; 3],
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
) {
    stack[0] = m[9]; // mova [rsp+gprsize+32*0], m9
    idct32_pass1_end(m, tmp1, tmp2, 0, 8, 1, 9); // IDCT32_PASS1_END 0, 8, 1, 9
    idct32_pass1_end(m, tmp1, tmp2, 2, 10, 1, 9); // IDCT32_PASS1_END 2, 10, 1, 9
    idct32_pass1_end(m, tmp1, tmp2, 3, 11, 1, 9); // IDCT32_PASS1_END 3, 11, 1, 9
    idct32_pass1_end(m, tmp1, tmp2, 4, 12, 1, 9); // IDCT32_PASS1_END 4, 12, 1, 9
    idct32_pass1_end(m, tmp1, tmp2, 5, 13, 1, 9); // IDCT32_PASS1_END 5, 13, 1, 9
    idct32_pass1_end(m, tmp1, tmp2, 6, 14, 1, 9); // IDCT32_PASS1_END 6, 14, 1, 9
    idct32_pass1_end(m, tmp1, tmp2, 7, 15, 1, 9); // IDCT32_PASS1_END 7, 15, 1, 9
    m[1] = stack[1]; // mova m1, [rsp+gprsize+32*1]
    m[9] = stack[0]; // mova m9, [rsp+gprsize+32*0]
    stack[0] = m[6]; // mova [rsp+gprsize+32*0], m6
    stack[1] = m[7]; // mova [rsp+gprsize+32*1], m7
    idct32_pass1_end(m, tmp1, tmp2, 1, 9, 6, 7); // IDCT32_PASS1_END 1, 9, 6, 7
}

/// `cglobal inv_txfm_add_identity_identity_16x32_8bpc, 4, 5, 13, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_16x32(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[9] = pw(2896 * 8); // vpbroadcastd m9, [o(pw_2896x8)]
    m[10] = pw(1697 * 16); // vpbroadcastd m10, [o(pw_1697x16)]
    m[12] = pw(8192); // vpbroadcastd m12, [o(pw_8192)]
    let mut r4b = (eob > 43) as i8; // cmp eobd, 43 / setg r4b ; if (eob > 43) iteration_count++
    let al = (eob > 150) as i8; // cmp eobd, 150 / setg al ; if (eob > 150) iteration_count++
    let (_, cf) = (eob as u32).overflowing_add(-279i32 as u32); // add eobd, -279 ; if (eob > 278)
    r4b = r4b + al + cf as i8; // adc r4b, al ; iteration_count++
    let r3 = stride * 3; // lea r3, [strideq*3]
    let mut r6 = c; // mov r6, cq
    m[11] = _mm256_add_epi16(m[12], m[12]); // paddw m11, m12, m12 ; pw_16384
    loop {
        // .loop:
        m[0] = zx(_mm_loadu_si128(at(c, 64 * 0).cast())); // mova xm0, [cq+64* 0]
        m[1] = zx(_mm_loadu_si128(at(c, 64 * 1).cast())); // mova xm1, [cq+64* 1]
        m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 64 * 8).cast())); // vinserti128 m0, [cq+64* 8], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 64 * 9).cast())); // vinserti128 m1, [cq+64* 9], 1
        m[2] = zx(_mm_loadu_si128(at(c, 64 * 2).cast())); // mova xm2, [cq+64* 2]
        m[3] = zx(_mm_loadu_si128(at(c, 64 * 3).cast())); // mova xm3, [cq+64* 3]
        m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, 64 * 10).cast())); // vinserti128 m2, [cq+64*10], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, 64 * 11).cast())); // vinserti128 m3, [cq+64*11], 1
        m[4] = zx(_mm_loadu_si128(at(c, 64 * 4).cast())); // mova xm4, [cq+64* 4]
        m[5] = zx(_mm_loadu_si128(at(c, 64 * 5).cast())); // mova xm5, [cq+64* 5]
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 64 * 12).cast())); // vinserti128 m4, [cq+64*12], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(c, 64 * 13).cast())); // vinserti128 m5, [cq+64*13], 1
        m[6] = zx(_mm_loadu_si128(at(c, 64 * 6).cast())); // mova xm6, [cq+64* 6]
        m[7] = zx(_mm_loadu_si128(at(c, 64 * 7).cast())); // mova xm7, [cq+64* 7]
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(c, 64 * 14).cast())); // vinserti128 m6, [cq+64*14], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(c, 64 * 15).cast())); // vinserti128 m7, [cq+64*15], 1
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => m[r] = _mm256_mulhrs_epi16(m[r], m[9])); // REPX {pmulhrsw x, m9 }, m0, m1, m2, m3, m4, m5, m6, m7
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => idtx16(m, r, 8, 10, Some(11))); // REPX {IDTX16 x, 8, 10, 11}, 0, 1, 2, 3, 4, 5, 6, 7
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => m[r] = _mm256_mulhrs_epi16(m[r], m[12])); // REPX {pmulhrsw x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
        write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
        write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r3]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        write_16x2(m, dst, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
        write_16x2(m, dst, m[6], m[7], 0, 1, [stride * 2, r3]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        c = at(c, 16); // add cq, 16
        r4b -= 1; // dec r4b
        if r4b < 0 {
            break; // jge .loop
        }
    }
    c = at(c, -32); // sub cq, 32
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    let mut r0d = 8; // mov r0d, 8
    if c > r6 {
        // cmp cq, r6 / ja .zero_loop
        loop {
            // .zero_loop:
            _mm256_storeu_si256(at(r6, 32 * 0).cast(), m[0]); // mova [r6+32*0], m0
            _mm256_storeu_si256(at(r6, 32 * 1).cast(), m[0]); // mova [r6+32*1], m0
            _mm256_storeu_si256(at(r6, 32 * 2).cast(), m[0]); // mova [r6+32*2], m0
            _mm256_storeu_si256(at(r6, 32 * 3).cast(), m[0]); // mova [r6+32*3], m0
            r6 = at(r6, 32 * 4); // add r6, 32*4
            r0d -= 1; // dec r0d
            if r0d <= 0 {
                break; // jg .zero_loop
            }
        }
    } else {
        loop {
            // .zero_loop_half:
            _mm256_storeu_si256(at(r6, 64 * 0).cast(), m[0]); // mova [r6+64*0], m0
            _mm256_storeu_si256(at(r6, 64 * 1).cast(), m[0]); // mova [r6+64*1], m0
            r6 = at(r6, 64 * 4); // add r6, 64*4
            _mm256_storeu_si256(at(r6, -64 * 2).cast(), m[0]); // mova [r6-64*2], m0
            _mm256_storeu_si256(at(r6, -64 * 1).cast(), m[0]); // mova [r6-64*1], m0
            r0d -= 2; // sub r0d, 2
            if r0d <= 0 {
                break; // jg .zero_loop_half
            }
        }
    }
}

/// `cglobal inv_txfm_add_identity_identity_32x16_8bpc, 4, 6, 12, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x16(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[9] = pw(2896 * 8); // vpbroadcastd m9, [o(pw_2896x8)]
    m[10] = pw(1697 * 16); // vpbroadcastd m10, [o(pw_1697x16)]
    m[11] = pw(2048); // vpbroadcastd m11, [o(pw_2048)]
    let mut r4b = (eob > 35) as i8; // cmp eobd, 35 / setg r4b ; if (eob > 35) iteration_count++
    let r3b = (eob > 150) as i8; // cmp eobd, 150 / setg r3b ; if (eob > 150) iteration_count += 2
    r4b += r3b * 2; // lea r4d, [r4+r3*2] (only r4b is used: the upper bits of r4 and r3 do not reach it)
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r5 = dst; // mov r5, dstq
    let mut r6 = c; // mov r6, cq
    loop {
        // .loop:
        m[0] = zx(_mm_loadu_si128(at(c, 32 * 0).cast())); // mova xm0, [cq+32* 0]
        m[1] = zx(_mm_loadu_si128(at(c, 32 * 1).cast())); // mova xm1, [cq+32* 1]
        m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 32 * 8).cast())); // vinserti128 m0, [cq+32* 8], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 32 * 9).cast())); // vinserti128 m1, [cq+32* 9], 1
        m[2] = zx(_mm_loadu_si128(at(c, 32 * 2).cast())); // mova xm2, [cq+32* 2]
        m[3] = zx(_mm_loadu_si128(at(c, 32 * 3).cast())); // mova xm3, [cq+32* 3]
        m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, 32 * 10).cast())); // vinserti128 m2, [cq+32*10], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, 32 * 11).cast())); // vinserti128 m3, [cq+32*11], 1
        m[4] = zx(_mm_loadu_si128(at(c, 32 * 4).cast())); // mova xm4, [cq+32* 4]
        m[5] = zx(_mm_loadu_si128(at(c, 32 * 5).cast())); // mova xm5, [cq+32* 5]
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 32 * 12).cast())); // vinserti128 m4, [cq+32*12], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(c, 32 * 13).cast())); // vinserti128 m5, [cq+32*13], 1
        m[6] = zx(_mm_loadu_si128(at(c, 32 * 6).cast())); // mova xm6, [cq+32* 6]
        m[7] = zx(_mm_loadu_si128(at(c, 32 * 7).cast())); // mova xm7, [cq+32* 7]
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(c, 32 * 14).cast())); // vinserti128 m6, [cq+32*14], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(c, 32 * 15).cast())); // vinserti128 m7, [cq+32*15], 1
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => m[r] = _mm256_mulhrs_epi16(m[r], m[9])); // REPX {pmulhrsw x, m9 }, m0, m1, m2, m3, m4, m5, m6, m7
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => m[r] = _mm256_adds_epi16(m[r], m[r])); // REPX {paddsw x, x }, m0, m1, m2, m3, m4, m5, m6, m7
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => idtx16(m, r, 8, 10, None)); // REPX {IDTX16 x, 8, 10}, 0, 1, 2, 3, 4, 5, 6, 7
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => m[r] = _mm256_mulhrs_epi16(m[r], m[11])); // REPX {pmulhrsw x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
        write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
        write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r3]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        write_16x2(m, dst, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
        write_16x2(m, dst, m[6], m[7], 0, 1, [stride * 2, r3]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        c = at(c, 16); // add cq, 16
        r4b -= 1; // dec r4b
        if r4b < 0 {
            break; // jl .ret
        }
        if r4b & 1 == 0 {
            continue; // test r4b, 1 / jz .loop
        }
        c = at(c, 32 * 15); // add cq, 32*15
        dst = at(r5, 16); // lea dstq, [r5+16]
                          // jmp .loop
    }
    // .ret:
    let mut cd = (c as usize as u32).wrapping_sub(r6 as usize as u32) as i32; // sub cd, eax (eax is r6d)
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    cd += 384; // add cd, 384
    loop {
        // .zero_loop:
        _mm256_storeu_si256(at(r6, 32 * 0).cast(), m[0]); // mova [r6+32*0], m0
        _mm256_storeu_si256(at(r6, 32 * 1).cast(), m[0]); // mova [r6+32*1], m0
        _mm256_storeu_si256(at(r6, 32 * 2).cast(), m[0]); // mova [r6+32*2], m0
        _mm256_storeu_si256(at(r6, 32 * 3).cast(), m[0]); // mova [r6+32*3], m0
        r6 = at(r6, 32 * 4); // add r6, 32*4
        cd -= 128; // sub cd, 128
        if cd < 0 {
            break; // jge .zero_loop
        }
    }
}

/// `cglobal inv_txfm_add_dct_dct_32x32_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x32(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 32; // or r3d, 32
        return inv_txfm_add_dct_dct_32x8_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_32x8_8bpc).dconly
    }
    // .normal:
    // PROLOGUE 0, 9, 16, 32*67, dst, stride, c, eob, tmp1, tmp2, base, tmp3, tmp4
    let mut stack = [_mm256_setzero_si256(); 3]; // [rsp+32*0..3]
    let stack = &mut stack;
    let mut frame = core::mem::MaybeUninit::<[__m256i; 64]>::uninit(); // [rsp+32*3..67]
    let frame0: *mut __m256i = zeroed_frame(&mut frame); // zeroed whole (see above)
    let mut tmp1 = at(frame0, 32 * 4); // lea tmp1q, [rsp+32*7]
    let mut tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
    let mut tmp3;
    let mut eobd = eob.wrapping_sub(136) as u32; // sub eobd, 136
    let tmp4d = eobd as i32; // mov tmp4d, eobd
    loop {
        // .pass1_loop:
        load_8rows(m, at(c, 64 * 1), 64 * 2, 0); // LOAD_8ROWS cq+64*1, 64*2
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([1, 3, 5, 7, 9, 11, 13, 15], x => _mm256_storeu_si256(at(c, 64 * x).cast(), m[8])); // REPX {mova [cq+64*x], m8}, 1, 3, 5, 7, 9, 11, 13, 15
        if tmp4d >= 0 {
            // test tmp4d, tmp4d / jl .fast
            load_8rows_h(m, at(c, 64 * 17), 64 * 2, 0); // LOAD_8ROWS_H cq+64*17, 64*2
            inv_txfm_add_dct_dct_16x32_main_oddhalf(m, stack, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
            load_8rows_h(m, at(c, 64 * 16), 64 * 2, 0); // LOAD_8ROWS_H cq+64*16, 64*2
            m[0] = _mm256_setzero_si256(); // pxor m0, m0
            repx!([16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31], x => _mm256_storeu_si256(at(c, 64 * x).cast(), m[0])); // REPX {mova [cq+64*x], m0}, 16, 17, …, 31
            stack[0] = m[15]; // mova [rsp], m15
                              // jmp .idct16
        } else {
            // .fast:
            inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, stack, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
            m[8] = _mm256_setzero_si256(); // pxor m8, m8
            repx!([9, 10, 11, 12, 13, 14], r => m[r] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
            stack[0] = m[8]; // mova [rsp], m8
        }
        // .idct16:
        load_8rows(m, at(c, 64 * 0), 64 * 2, 0); // LOAD_8ROWS cq+64*0, 64*2
        m[15] = _mm256_setzero_si256(); // pxor m15, m15
        repx!([0, 2, 4, 6, 8, 10, 12, 14], x => _mm256_storeu_si256(at(c, 64 * x).cast(), m[15])); // REPX {mova [cq+64*x], m15}, 0, 2, 4, 6, 8, 10, 12, 14
        idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_32x16_pass1_end(m, stack, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_32x16_8bpc).pass1_end
        m[7] = pw(8192); // vpbroadcastd m7, [o(pw_8192)]
        inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, stack); // call m(inv_txfm_add_dct_dct_16x32_8bpc).transpose_2x8x8_round
        tmp3 = at(tmp1, 32 * 32); // lea tmp3q, [tmp1q+32*32]
        m[15] = stack[0]; // mova m15, [rsp]
        at(tmp3, -32 * 4).write(m[0]); // mova [tmp3q-32*4], m0
        at(tmp3, -32 * 3).write(m[2]); // mova [tmp3q-32*3], m2
        at(tmp3, -32 * 2).write(m[4]); // mova [tmp3q-32*2], m4
        at(tmp3, -32 * 1).write(m[6]); // mova [tmp3q-32*1], m6
        at(tmp3, 32 * 0).write(m[8]); // mova [tmp3q+32*0], m8
        at(tmp3, 32 * 1).write(m[10]); // mova [tmp3q+32*1], m10
        at(tmp3, 32 * 2).write(m[12]); // mova [tmp3q+32*2], m12
        at(tmp3, 32 * 3).write(m[14]); // mova [tmp3q+32*3], m14
        tmp3 = at(tmp3, 32 * 8); // add tmp3q, 32*8
        at(tmp3, -32 * 4).write(m[1]); // mova [tmp3q-32*4], m1
        at(tmp3, -32 * 3).write(m[3]); // mova [tmp3q-32*3], m3
        at(tmp3, -32 * 2).write(m[5]); // mova [tmp3q-32*2], m5
        at(tmp3, -32 * 1).write(m[7]); // mova [tmp3q-32*1], m7
        at(tmp3, 32 * 0).write(m[9]); // mova [tmp3q+32*0], m9
        at(tmp3, 32 * 1).write(m[11]); // mova [tmp3q+32*1], m11
        at(tmp3, 32 * 2).write(m[13]); // mova [tmp3q+32*2], m13
        at(tmp3, 32 * 3).write(m[15]); // mova [tmp3q+32*3], m15
        m[9] = pw(8192); // vpbroadcastd m9, [o(pw_8192)]
        m[0] = _mm256_mulhrs_epi16(m[9], at(tmp1, -32 * 4).read()); // pmulhrsw m0, m9, [tmp1q-32*4]
        m[1] = _mm256_mulhrs_epi16(m[9], at(tmp1, -32 * 3).read()); // pmulhrsw m1, m9, [tmp1q-32*3]
        m[2] = _mm256_mulhrs_epi16(m[9], at(tmp1, -32 * 2).read()); // pmulhrsw m2, m9, [tmp1q-32*2]
        m[3] = _mm256_mulhrs_epi16(m[9], at(tmp1, -32 * 1).read()); // pmulhrsw m3, m9, [tmp1q-32*1]
        m[4] = _mm256_mulhrs_epi16(m[9], at(tmp1, 32 * 0).read()); // pmulhrsw m4, m9, [tmp1q+32*0]
        m[5] = _mm256_mulhrs_epi16(m[9], at(tmp1, 32 * 1).read()); // pmulhrsw m5, m9, [tmp1q+32*1]
        m[6] = _mm256_mulhrs_epi16(m[9], at(tmp1, 32 * 2).read()); // pmulhrsw m6, m9, [tmp1q+32*2]
        m[7] = _mm256_mulhrs_epi16(m[9], at(tmp1, 32 * 3).read()); // pmulhrsw m7, m9, [tmp1q+32*3]
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        at(tmp1, -32 * 4).write(m[0]); // mova [tmp1q-32*4], m0
        m[0] = _mm256_mulhrs_epi16(m[9], at(tmp2, -32 * 4).read()); // pmulhrsw m0, m9, [tmp2q-32*4]
        at(tmp2, -32 * 4).write(m[1]); // mova [tmp2q-32*4], m1
        m[1] = _mm256_mulhrs_epi16(m[9], at(tmp2, -32 * 3).read()); // pmulhrsw m1, m9, [tmp2q-32*3]
        at(tmp1, -32 * 3).write(m[2]); // mova [tmp1q-32*3], m2
        m[2] = _mm256_mulhrs_epi16(m[9], at(tmp2, -32 * 2).read()); // pmulhrsw m2, m9, [tmp2q-32*2]
        at(tmp2, -32 * 3).write(m[3]); // mova [tmp2q-32*3], m3
        m[3] = _mm256_mulhrs_epi16(m[9], at(tmp2, -32 * 1).read()); // pmulhrsw m3, m9, [tmp2q-32*1]
        at(tmp1, -32 * 2).write(m[4]); // mova [tmp1q-32*2], m4
        m[4] = _mm256_mulhrs_epi16(m[9], at(tmp2, 32 * 0).read()); // pmulhrsw m4, m9, [tmp2q+32*0]
        at(tmp2, -32 * 2).write(m[5]); // mova [tmp2q-32*2], m5
        m[5] = _mm256_mulhrs_epi16(m[9], at(tmp2, 32 * 1).read()); // pmulhrsw m5, m9, [tmp2q+32*1]
        at(tmp1, -32 * 1).write(m[6]); // mova [tmp1q-32*1], m6
        m[6] = _mm256_mulhrs_epi16(m[9], at(tmp2, 32 * 2).read()); // pmulhrsw m6, m9, [tmp2q+32*2]
        at(tmp2, -32 * 1).write(m[7]); // mova [tmp2q-32*1], m7
        m[7] = _mm256_mulhrs_epi16(m[9], at(tmp2, 32 * 3).read()); // pmulhrsw m7, m9, [tmp2q+32*3]
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        at(tmp1, 32 * 0).write(m[0]); // mova [tmp1q+32*0], m0
        at(tmp2, 32 * 0).write(m[1]); // mova [tmp2q+32*0], m1
        at(tmp1, 32 * 1).write(m[2]); // mova [tmp1q+32*1], m2
        at(tmp2, 32 * 1).write(m[3]); // mova [tmp2q+32*1], m3
        at(tmp1, 32 * 2).write(m[4]); // mova [tmp1q+32*2], m4
        at(tmp2, 32 * 2).write(m[5]); // mova [tmp2q+32*2], m5
        at(tmp1, 32 * 3).write(m[6]); // mova [tmp1q+32*3], m6
        at(tmp2, 32 * 3).write(m[7]); // mova [tmp2q+32*3], m7
        c = at(c, 32); // add cq, 32
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        tmp2 = at(tmp2, 32 * 16); // add tmp2q, 32*16
        let (sum, cf) = eobd.overflowing_add(0x8000_0000); // add eobd, 0x80000000
        eobd = sum;
        if cf {
            break; // jnc .pass1_loop
        }
    }
    tmp1 = at(tmp1, 32 * 24); // add tmp1q, 32*24
    let mut r2 = at(dst, stride * 19); // imul r2, strideq, 19 / add r2, dstq
    let r3 = stride * 3; // lea r3, [strideq*3]
    if tmp4d < 0 {
        // test tmp4d, tmp4d / jge .pass2_loop
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        tmp2 = at(tmp2, 32 * 16); // add tmp2q, 32*16
        tmp3 = at(tmp3, 32 * 16); // add tmp3q, 32*16
    }
    loop {
        // .pass2_loop:
        load_8rows(m, at(tmp2, -32 * 4), 32, 0); // LOAD_8ROWS tmp2q-32*4, 32
        if tmp4d >= 0 {
            // test tmp4d, tmp4d / jl .fast2
            load_8rows_h(m, at(tmp3, -32 * 4), 32, 0); // LOAD_8ROWS_H tmp3q-32*4, 32
            inv_txfm_add_dct_dct_16x32_main_oddhalf(m, stack, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
            tmp3 = at(tmp3, -32 * 8); // sub tmp3q, 32*8
            load_8rows_h(m, at(tmp3, -32 * 4), 32, 0); // LOAD_8ROWS_H tmp3q-32*4, 32
            tmp3 = at(tmp3, -32 * 16); // sub tmp3q, 32*16
                                       // jmp .pass2_loop_end
        } else {
            // .fast2:
            inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, stack, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
            tmp3 = at(tmp3, -32 * 24); // sub tmp3q, 32*24
            m[8] = _mm256_setzero_si256(); // pxor m8, m8
            repx!([9, 10, 11, 12, 13, 14, 15], r => m[r] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
        }
        // .pass2_loop_end:
        load_8rows(m, at(tmp3, -32 * 4), 32, 0); // LOAD_8ROWS tmp3q-32*4, 32
        stack[0] = m[15]; // mova [rsp], m15
        idct_16x16_internal_main(m, stack); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_16x32_pass2_end(m, stack, &mut dst, stride, &mut r2, r3, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).pass2_end
        tmp3 = at(tmp1, -32 * 32); // lea tmp3q, [tmp1q-32*32]
        if tmp2 < tmp3 {
            break; // cmp tmp2q, tmp3q / jb .ret
        }
        tmp2 = at(tmp2, -32 * 32); // sub tmp2q, 32*32
        dst = at(dst, -r3); // sub dstq, r3
        r2 = at(r2, r3 + 16); // lea r2, [r2+r3+16]
        dst = at(dst, 16); // add dstq, 16
                           // jmp .pass2_loop
    }
    // .ret:
}

/// `cglobal inv_txfm_add_identity_identity_32x32_8bpc, 4, 6, 10, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x32(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[9] = pw(8192); // vpbroadcastd m9, [pw_8192]
    let mut eobd = eob.wrapping_sub(136) as u32; // sub eobd, 136 ; if (eob < 136)
    eobd >>= 30; // shr eobd, 30 ; topleft 16x16 only
    eobd = eobd.wrapping_mul(2).wrapping_sub(8); // lea eobd, [eobq*2-8]
    let r4 = stride * 3; // lea r4, [strideq*3]
    let r5 = dst; // mov r5, dstq
    let mut r6 = at(c, 32); // lea r6, [cq+32]
    loop {
        // .loop:
        m[0] = zx(_mm_loadu_si128(at(c, 64 * 0).cast())); // mova xm0, [cq+64* 0]
        m[1] = zx(_mm_loadu_si128(at(c, 64 * 1).cast())); // mova xm1, [cq+64* 1]
        m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 64 * 8).cast())); // vinserti128 m0, [cq+64* 8], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 64 * 9).cast())); // vinserti128 m1, [cq+64* 9], 1
        m[2] = zx(_mm_loadu_si128(at(c, 64 * 2).cast())); // mova xm2, [cq+64* 2]
        m[3] = zx(_mm_loadu_si128(at(c, 64 * 3).cast())); // mova xm3, [cq+64* 3]
        m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(c, 64 * 10).cast())); // vinserti128 m2, [cq+64*10], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(c, 64 * 11).cast())); // vinserti128 m3, [cq+64*11], 1
        m[4] = zx(_mm_loadu_si128(at(c, 64 * 4).cast())); // mova xm4, [cq+64* 4]
        m[5] = zx(_mm_loadu_si128(at(c, 64 * 5).cast())); // mova xm5, [cq+64* 5]
        m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(c, 64 * 12).cast())); // vinserti128 m4, [cq+64*12], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(c, 64 * 13).cast())); // vinserti128 m5, [cq+64*13], 1
        m[6] = zx(_mm_loadu_si128(at(c, 64 * 6).cast())); // mova xm6, [cq+64* 6]
        m[7] = zx(_mm_loadu_si128(at(c, 64 * 7).cast())); // mova xm7, [cq+64* 7]
        m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_loadu_si128(at(c, 64 * 14).cast())); // vinserti128 m6, [cq+64*14], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_loadu_si128(at(c, 64 * 15).cast())); // vinserti128 m7, [cq+64*15], 1
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        repx!([0, 1, 2, 3, 4, 5, 6, 7], r => m[r] = _mm256_mulhrs_epi16(m[r], m[9])); // REPX {pmulhrsw x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
        write_16x2(m, dst, m[0], m[1], 8, 0, [0, stride]); // WRITE_16X2 0, 1, 8, 0, strideq*0, strideq*1
        write_16x2(m, dst, m[2], m[3], 0, 1, [stride * 2, r4]); // WRITE_16X2 2, 3, 0, 1, strideq*2, r4
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        write_16x2(m, dst, m[4], m[5], 0, 1, [0, stride]); // WRITE_16X2 4, 5, 0, 1, strideq*0, strideq*1
        write_16x2(m, dst, m[6], m[7], 0, 1, [stride * 2, r4]); // WRITE_16X2 6, 7, 0, 1, strideq*2, r4
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        c = at(c, 16); // add cq, 16
        eobd = eobd.wrapping_add(1); // inc eobd
        if eobd == 0 {
            break; // jz .ret
        }
        if eobd & 3 != 0 {
            continue; // test eobd, 3 / jnz .loop
        }
        c = at(c, 64 * 15); // add cq, 64*15
        dst = at(r5, 16); // lea dstq, [r5+16]
                          // jmp .loop
    }
    // .ret:
    m[0] = _mm256_setzero_si256(); // pxor m0, m0
    let mut r0d = 16; // mov r0d, 16
    if c != r6 {
        // cmp cq, r6 / jne .zero_loop
        loop {
            // .zero_loop:
            _mm256_storeu_si256(at(r6, -32 * 1).cast(), m[0]); // mova [r6-32*1], m0
            _mm256_storeu_si256(at(r6, 32 * 0).cast(), m[0]); // mova [r6+32*0], m0
            _mm256_storeu_si256(at(r6, 32 * 1).cast(), m[0]); // mova [r6+32*1], m0
            _mm256_storeu_si256(at(r6, 32 * 2).cast(), m[0]); // mova [r6+32*2], m0
            r6 = at(r6, 32 * 4); // add r6, 32*4
            r0d -= 1; // dec r0d
            if r0d <= 0 {
                break; // jg .zero_loop
            }
        }
    } else {
        loop {
            // .zero_loop_topleft:
            _mm256_storeu_si256(at(r6, -32 * 1).cast(), m[0]); // mova [r6-32*1], m0
            _mm256_storeu_si256(at(r6, 32 * 1).cast(), m[0]); // mova [r6+32*1], m0
            _mm256_storeu_si256(at(r6, 32 * 3).cast(), m[0]); // mova [r6+32*3], m0
            _mm256_storeu_si256(at(r6, 32 * 5).cast(), m[0]); // mova [r6+32*5], m0
            r6 = at(r6, 64 * 4); // add r6, 64*4
            r0d -= 4; // sub r0d, 4
            if r0d <= 0 {
                break; // jg .zero_loop_topleft
            }
        }
    }
}

// 16x32
inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_16x32_8bpc_avx2, inv_txfm_add_dct_dct_16x32);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x32_8bpc_avx2,
    inv_txfm_add_identity_identity_16x32
);

// 32x16
inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_32x16_8bpc_avx2, inv_txfm_add_dct_dct_32x16);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x16_8bpc_avx2,
    inv_txfm_add_identity_identity_32x16
);

// 32x32
inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_32x32_8bpc_avx2, inv_txfm_add_dct_dct_32x32);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x32_8bpc_avx2,
    inv_txfm_add_identity_identity_32x32
);

// --- part 5: itx_avx2.asm lines 4413-5542 (16x64, 64x16, 32x64, 64x32, 64x64) --------------------

// The frames of these sizes (`PROLOGUE …, 32*N`) are arrays of `N` 32-byte slots, reached through the raw pointer
// `rsp` to their first slot and the pointers the assembly derives from it (`tmp1q`, `tmp2q`, `r2`, `r3`). Each
// frame is zeroed whole before its first use (`zeroed_frame`), so no load from it can find a slot without a
// value, whichever eob path wrote it. The subroutines that the assembly `call`s address the caller's first slots
// as `[rsp+gprsize+32*n]`; they receive them as `&mut [__m256i; 3]` (`s3_p5(rsp)`), which no other pointer
// touches while the subroutine runs (`tmp1q`/`tmp2q` never point below slot 3).

/// `mova m, [p+off]` in a frame (32-byte aligned; `off` in bytes)
#[inline(always)]
unsafe fn ld_p5(p: *mut __m256i, off: isize) -> __m256i {
    _mm256_load_si256(at(p, off))
}

/// `mova [p+off], m` in a frame
#[inline(always)]
unsafe fn st_p5(p: *mut __m256i, off: isize, v: __m256i) {
    _mm256_store_si256(at(p, off), v)
}

/// `mova xm, [p+off]` in a frame (16-byte aligned)
#[inline(always)]
unsafe fn ldx_p5(p: *mut __m256i, off: isize) -> __m128i {
    _mm_load_si128(at(p, off).cast())
}

/// The frame's first three slots, `[rsp+gprsize+32*0..2]` of a called subroutine.
#[inline(always)]
unsafe fn s3_p5<'a>(rsp: *mut __m256i) -> &'a mut [__m256i; 3] {
    &mut *rsp.cast::<[__m256i; 3]>()
}

/// `COEF_X8 x`: the doubleword of the two words `x*8`.
const fn coef_x8_p5(x: i32) -> i32 {
    let w = (x * 8) as u16 as u32;
    ((w << 16) | w) as i32
}

/// `idct64_mul` (`SECTION_RODATA`): `COEF_X8` of the idct64 step-1 multipliers. `.main_part1` reads the doublewords
/// at `r6 + 4*k` for the `k` it names; its caller sets `r6` to the table (`r6` = 0 here) and then 8 bytes on (2).
#[allow(non_upper_case_globals)]
pub(crate) static idct64_mul: [i32; 32] = [
    coef_x8_p5(4095),
    coef_x8_p5(101),
    coef_x8_p5(4065),
    coef_x8_p5(501),
    coef_x8_p5(2967),
    coef_x8_p5(-2824),
    coef_x8_p5(3229),
    coef_x8_p5(-2520),
    coef_x8_p5(3745),
    coef_x8_p5(1660),
    coef_x8_p5(3564),
    coef_x8_p5(2019),
    coef_x8_p5(3822),
    coef_x8_p5(-1474),
    coef_x8_p5(3948),
    coef_x8_p5(-1092),
    coef_x8_p5(3996),
    coef_x8_p5(897),
    coef_x8_p5(3889),
    coef_x8_p5(1285),
    coef_x8_p5(3461),
    coef_x8_p5(-2191),
    coef_x8_p5(3659),
    coef_x8_p5(-1842),
    coef_x8_p5(3349),
    coef_x8_p5(2359),
    coef_x8_p5(3102),
    coef_x8_p5(2675),
    coef_x8_p5(4036),
    coef_x8_p5(-700),
    coef_x8_p5(4085),
    coef_x8_p5(-301),
];

// --- the idct64 subroutines of 16x64 -------------------------------------------------------------------------

/// `%macro IDCT64_PART2_END out, src[1-2], tmp[1-3], (offset[1-4])`: the last sums and differences of the
/// 64-point DCT for the outputs `out`, `31-out`, `32+out` and `63-out`. Without `d` (the six-argument form, pass
/// 1) the four rows go back to the frame; with `d = (dstq, r2, offsets)` (pass 2) they are rounded by `m14` and
/// added to the four pixel rows of 16 at the offsets from `dstq` and `r2`.
#[inline(always)]
unsafe fn idct64_part2_end(
    m: &mut M,
    n: isize,
    s1: usize,
    s2: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
    d: Option<(Dst, Dst, [isize; 4])>,
) {
    if n & 1 != 0 {
        m[t2] = ld_p5(tmp2, -32 * (51 - n)); // mova m%5, [tmp2q-32*(51-%1)] ; idct16 out 0+n
        m[t1] = ld_p5(tmp1, -32 * (14 + n)); // mova m%4, [tmp1q-32*(14+%1)] ; idct32 out31-n
    } else {
        m[t2] = ld_p5(tmp1, -32 * (45 - n)); // mova m%5, [tmp1q-32*(45-%1)]
        m[t1] = ld_p5(tmp2, -32 * (20 + n)); // mova m%4, [tmp2q-32*(20+%1)]
    }
    m[t3] = _mm256_subs_epi16(m[t2], m[t1]); // psubsw m%6, m%5, m%4 ; idct32 out31-n
    m[t2] = _mm256_adds_epi16(m[t2], m[t1]); // paddsw m%5, m%4 ; idct32 out 0+n
    m[t1] = _mm256_subs_epi16(m[t3], m[s2]); // psubsw m%4, m%6, m%3 ; out32+n
    m[t3] = _mm256_adds_epi16(m[t3], m[s2]); // paddsw m%6, m%3 ; out31-n
    m[s2] = _mm256_subs_epi16(m[t2], m[s1]); // psubsw m%3, m%5, m%2 ; out63-n
    m[t2] = _mm256_adds_epi16(m[t2], m[s1]); // paddsw m%5, m%2 ; out 0+n
    match d {
        None => {
            // pass 1
            if n & 1 != 0 {
                st_p5(tmp2, -32 * (19 - n), m[t1]); // mova [tmp2q-32*(19-%1)], m%4
                st_p5(tmp1, -32 * (14 + n), m[t3]); // mova [tmp1q-32*(14+%1)], m%6
                st_p5(tmp1, 32 * (18 - n), m[s2]); // mova [tmp1q+32*(18-%1)], m%3
                st_p5(tmp2, -32 * (51 - n), m[t2]); // mova [tmp2q-32*(51-%1)], m%5
            } else {
                st_p5(tmp1, -32 * (13 - n), m[t1]); // mova [tmp1q-32*(13-%1)], m%4
                st_p5(tmp2, -32 * (20 + n), m[t3]); // mova [tmp2q-32*(20+%1)], m%6
                st_p5(tmp2, 32 * (12 - n), m[s2]); // mova [tmp2q+32*(12-%1)], m%3
                st_p5(tmp1, -32 * (45 - n), m[t2]); // mova [tmp1q-32*(45-%1)], m%5
            }
        }
        Some((dst, r2, o)) => {
            // pass 2
            repx!([t1, t3, s2, t2], i => m[i] = _mm256_mulhrs_epi16(m[i], m[14])); // REPX {pmulhrsw x, m14}, m%4, m%6, m%3, m%5
            let (d0, d1) = if n & 1 != 0 { (r2, dst) } else { (dst, r2) };
            m[s1] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(d0, o[2]).cast())); // pmovzxbw m%2, [%%d0+%9]
            m[s1] = _mm256_add_epi16(m[s1], m[t1]); // paddw m%2, m%4
            m[t1] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(d1, o[1]).cast())); // pmovzxbw m%4, [%%d1+%8]
            m[t1] = _mm256_add_epi16(m[t1], m[t3]); // paddw m%4, m%6
            m[t3] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(d1, o[3]).cast())); // pmovzxbw m%6, [%%d1+%10]
            m[s2] = _mm256_add_epi16(m[s2], m[t3]); // paddw m%3, m%6
            m[t3] = _mm256_cvtepu8_epi16(_mm_loadu_si128(at(d0, o[0]).cast())); // pmovzxbw m%6, [%%d0+%7]
            m[t2] = _mm256_add_epi16(m[t2], m[t3]); // paddw m%5, m%6
            m[s1] = _mm256_packus_epi16(m[s1], m[t1]); // packuswb m%2, m%4
            m[s2] = _mm256_packus_epi16(m[s2], m[t2]); // packuswb m%3, m%5
            m[s1] = _mm256_permute4x64_epi64::<0xd8>(m[s1]); // vpermq m%2, m%2, q3120
            m[s2] = _mm256_permute4x64_epi64::<0xd8>(m[s2]); // vpermq m%3, m%3, q3120
            _mm_storeu_si128(at(d0, o[2]).cast(), lo(m[s1])); // mova [%%d0+%9], xm%2
            _mm_storeu_si128(at(d1, o[1]).cast(), _mm256_extracti128_si256::<1>(m[s1])); // vextracti128 [%%d1+%8], m%2, 1
            _mm_storeu_si128(at(d1, o[3]).cast(), lo(m[s2])); // mova [%%d1+%10], xm%3
            _mm_storeu_si128(at(d0, o[0]).cast(), _mm256_extracti128_si256::<1>(m[s2]));
            // vextracti128 [%%d0+%7], m%3, 1
        }
    }
}

/// `vpbroadcastd m, [o(idct64_mul+4*k)]` with `r6` set to the table plus `4*r6` bytes
#[inline(always)]
unsafe fn idct64_mul_p5(r6: usize, k: usize) -> __m256i {
    _mm256_set1_epi32(idct64_mul[r6 + k])
}

/// `.main_part1` of `inv_txfm_add_dct_dct_16x64_8bpc`: idct64 steps 1-5. in1/31/17/15/9/23/25/7 (with `r6` = 0) or
/// in5/27/21/11/13/19/29/3 (with `r6` = 2, the assembly's `add r6, 8`) in `m0`-`m7` become t32a…t39a and
/// t56a…t63a (or t40a…t55a) in `[tmp1q-32*4..+3]` and `[tmp2q-32*4..+3]`. `m15` holds `pd_2048`.
///
/// Every `o(…)` constant here is relative to `r6`, so the second call (`r6` 8 bytes on) reads each pair constant
/// two entries later in the assembly's rodata (`pw_401_4076` → `pw_1931_3612`, `pw_m4017_799` → `pw_m2276_3406`,
/// `pw_799_4017` → `pw_3406_2276`, `pw_3166_2598` → `pw_3920_1189`, `pw_m799_m4017` → `pw_m3406_m2276`, and in
/// `ITX_MULSUB_2W` `pw_m401_m4076`/`pw_m4076_401` → `pw_m1931_m3612`/`pw_m3612_1931`,
/// `pw_m3166_m2598`/`pw_m2598_3166` → `pw_m3920_m1189`/`pw_m1189_3920`): those are written out per call.
#[inline(always)]
pub(crate) unsafe fn inv_txfm_add_dct_dct_16x64_main_part1(
    m: &mut M,
    r6: usize,
    tmp1: *mut __m256i,
    tmp2: *mut __m256i,
) {
    m[11] = idct64_mul_p5(r6, 0); // vpbroadcastd m11, [o(idct64_mul+4* 0)]
    m[13] = idct64_mul_p5(r6, 1); // vpbroadcastd m13, [o(idct64_mul+4* 1)]
    m[10] = idct64_mul_p5(r6, 4); // vpbroadcastd m10, [o(idct64_mul+4* 4)]
    m[12] = idct64_mul_p5(r6, 5); // vpbroadcastd m12, [o(idct64_mul+4* 5)]
    m[11] = _mm256_mulhrs_epi16(m[11], m[0]); // pmulhrsw m11, m0 ; t63a
    m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13 ; t32a
    m[10] = _mm256_mulhrs_epi16(m[10], m[1]); // pmulhrsw m10, m1 ; t62a
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12 ; t33a
    m[9] = idct64_mul_p5(r6, 8); // vpbroadcastd m9, [o(idct64_mul+4* 8)]
    m[13] = idct64_mul_p5(r6, 9); // vpbroadcastd m13, [o(idct64_mul+4* 9)]
    m[8] = idct64_mul_p5(r6, 12); // vpbroadcastd m8, [o(idct64_mul+4*12)]
    m[12] = idct64_mul_p5(r6, 13); // vpbroadcastd m12, [o(idct64_mul+4*13)]
    m[9] = _mm256_mulhrs_epi16(m[9], m[2]); // pmulhrsw m9, m2 ; t61a
    m[2] = _mm256_mulhrs_epi16(m[2], m[13]); // pmulhrsw m2, m13 ; t34a
    m[8] = _mm256_mulhrs_epi16(m[8], m[3]); // pmulhrsw m8, m3 ; t60a
    m[3] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m3, m12 ; t35a
    m[12] = _mm256_subs_epi16(m[0], m[1]); // psubsw m12, m0, m1 ; t33
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; t32
    m[1] = _mm256_subs_epi16(m[3], m[2]); // psubsw m1, m3, m2 ; t34
    m[3] = _mm256_adds_epi16(m[3], m[2]); // paddsw m3, m2 ; t35
    m[2] = _mm256_subs_epi16(m[8], m[9]); // psubsw m2, m8, m9 ; t61
    m[8] = _mm256_adds_epi16(m[8], m[9]); // paddsw m8, m9 ; t60
    m[9] = _mm256_subs_epi16(m[11], m[10]); // psubsw m9, m11, m10 ; t62
    m[11] = _mm256_adds_epi16(m[11], m[10]); // paddsw m11, m10 ; t63
    let (c1, c2) = if r6 == 0 { (-4076, 401) } else { (-3612, 1931) }; // (r6 = 2: pw_m1931_m3612, pw_m3612_1931)
    itx_mulsub_2w(m, 2, 1, 10, 13, 15, c1, c2, None); // ITX_MULSUB_2W 2, 1, 10, 13, 15, m4076, 401 ; t34a, t61a
    m[14] = if r6 == 0 { pw2(401, 4076) } else { pw2(1931, 3612) }; // vpbroadcastd m14, [o(pw_401_4076)]
    itx_mulsub_2w(m, 9, 12, 10, 13, 15, 14, 13, None); // t33a, t62a
    m[10] = _mm256_subs_epi16(m[0], m[3]); // psubsw m10, m0, m3 ; t35a
    m[0] = _mm256_adds_epi16(m[0], m[3]); // paddsw m0, m3 ; t32a
    m[3] = _mm256_subs_epi16(m[11], m[8]); // psubsw m3, m11, m8 ; t60a
    m[11] = _mm256_adds_epi16(m[11], m[8]); // paddsw m11, m8 ; t63a
    m[8] = _mm256_subs_epi16(m[9], m[2]); // psubsw m8, m9, m2 ; t34
    m[9] = _mm256_adds_epi16(m[9], m[2]); // paddsw m9, m2 ; t33
    m[2] = _mm256_subs_epi16(m[12], m[1]); // psubsw m2, m12, m1 ; t61
    m[12] = _mm256_adds_epi16(m[12], m[1]); // paddsw m12, m1 ; t62
    st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
    st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
    st_p5(tmp2, 32 * 2, m[12]); // mova [tmp2q+32*2], m12
    st_p5(tmp2, 32 * 3, m[11]); // mova [tmp2q+32*3], m11
    m[13] = if r6 == 0 { pw2(-4017, 799) } else { pw2(-2276, 3406) }; // vpbroadcastd m13, [o(pw_m4017_799)]
    m[14] = if r6 == 0 { pw2(799, 4017) } else { pw2(3406, 2276) }; // vpbroadcastd m14, [o(pw_799_4017)]
    itx_mulsub_2w(m, 2, 8, 0, 1, 15, 14, 13, None); // t34a, t61a
    itx_mulsub_2w(m, 3, 10, 0, 1, 15, 14, 13, None); // t35, t60
    st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
    st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
    st_p5(tmp2, 32 * 0, m[10]); // mova [tmp2q+32*0], m10
    st_p5(tmp2, 32 * 1, m[8]); // mova [tmp2q+32*1], m8
    m[3] = idct64_mul_p5(r6, 16); // vpbroadcastd m3, [o(idct64_mul+4*16)]
    m[11] = idct64_mul_p5(r6, 17); // vpbroadcastd m11, [o(idct64_mul+4*17)]
    m[2] = idct64_mul_p5(r6, 20); // vpbroadcastd m2, [o(idct64_mul+4*20)]
    m[10] = idct64_mul_p5(r6, 21); // vpbroadcastd m10, [o(idct64_mul+4*21)]
    m[1] = idct64_mul_p5(r6, 24); // vpbroadcastd m1, [o(idct64_mul+4*24)]
    m[9] = idct64_mul_p5(r6, 25); // vpbroadcastd m9, [o(idct64_mul+4*25)]
    m[0] = idct64_mul_p5(r6, 28); // vpbroadcastd m0, [o(idct64_mul+4*28)]
    m[8] = idct64_mul_p5(r6, 29); // vpbroadcastd m8, [o(idct64_mul+4*29)]
    m[3] = _mm256_mulhrs_epi16(m[3], m[4]); // pmulhrsw m3, m4 ; t59a
    m[4] = _mm256_mulhrs_epi16(m[4], m[11]); // pmulhrsw m4, m11 ; t36a
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]); // pmulhrsw m2, m5 ; t58a
    m[5] = _mm256_mulhrs_epi16(m[5], m[10]); // pmulhrsw m5, m10 ; t37a
    m[1] = _mm256_mulhrs_epi16(m[1], m[6]); // pmulhrsw m1, m6 ; t57a
    m[6] = _mm256_mulhrs_epi16(m[6], m[9]); // pmulhrsw m6, m9 ; t38a
    m[0] = _mm256_mulhrs_epi16(m[0], m[7]); // pmulhrsw m0, m7 ; t56a
    m[7] = _mm256_mulhrs_epi16(m[7], m[8]); // pmulhrsw m7, m8 ; t39a
    m[8] = _mm256_subs_epi16(m[4], m[5]); // psubsw m8, m4, m5 ; t37
    m[4] = _mm256_adds_epi16(m[4], m[5]); // paddsw m4, m5 ; t36
    m[5] = _mm256_subs_epi16(m[7], m[6]); // psubsw m5, m7, m6 ; t38
    m[7] = _mm256_adds_epi16(m[7], m[6]); // paddsw m7, m6 ; t39
    m[6] = _mm256_subs_epi16(m[0], m[1]); // psubsw m6, m0, m1 ; t57
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1 ; t56
    m[1] = _mm256_subs_epi16(m[3], m[2]); // psubsw m1, m3, m2 ; t58
    m[3] = _mm256_adds_epi16(m[3], m[2]); // paddsw m3, m2 ; t59
    let (c1, c2) = if r6 == 0 { (-2598, 3166) } else { (-1189, 3920) }; // (r6 = 2: pw_m3920_m1189, pw_m1189_3920)
    itx_mulsub_2w(m, 6, 5, 2, 9, 15, c1, c2, None); // ITX_MULSUB_2W 6, 5, 2, 9, 15, m2598, 3166 ; t38a, t57a
    m[10] = if r6 == 0 { pw2(3166, 2598) } else { pw2(3920, 1189) }; // vpbroadcastd m10, [o(pw_3166_2598)]
    itx_mulsub_2w(m, 1, 8, 2, 9, 15, 10, 9, None); // t37a, t58a
    m[2] = _mm256_subs_epi16(m[7], m[4]); // psubsw m2, m7, m4 ; t36a
    m[7] = _mm256_adds_epi16(m[7], m[4]); // paddsw m7, m4 ; t39a
    m[4] = _mm256_subs_epi16(m[0], m[3]); // psubsw m4, m0, m3 ; t59a
    m[0] = _mm256_adds_epi16(m[0], m[3]); // paddsw m0, m3 ; t56a
    m[3] = _mm256_subs_epi16(m[6], m[1]); // psubsw m3, m6, m1 ; t37
    m[6] = _mm256_adds_epi16(m[6], m[1]); // paddsw m6, m1 ; t38
    m[1] = _mm256_subs_epi16(m[5], m[8]); // psubsw m1, m5, m8 ; t58
    m[5] = _mm256_adds_epi16(m[5], m[8]); // paddsw m5, m8 ; t57
    st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
    st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
    st_p5(tmp2, -32 * 4, m[0]); // mova [tmp2q-32*4], m0
    st_p5(tmp2, -32 * 3, m[5]); // mova [tmp2q-32*3], m5
    m[6] = if r6 == 0 { pw2(-799, -4017) } else { pw2(-3406, -2276) }; // vpbroadcastd m6, [o(pw_m799_m4017)]
    m[7] = if r6 == 0 { pw2(-4017, 799) } else { pw2(-2276, 3406) }; // vpbroadcastd m7, [o(pw_m4017_799)]
    itx_mulsub_2w(m, 4, 2, 0, 5, 15, 7, 6, None); // t36, t59
    itx_mulsub_2w(m, 1, 3, 0, 5, 15, 7, 6, None); // t37a, t58a
    st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
    st_p5(tmp1, 32 * 1, m[1]); // mova [tmp1q+32*1], m1
    st_p5(tmp2, -32 * 2, m[3]); // mova [tmp2q-32*2], m3
    st_p5(tmp2, -32 * 1, m[2]); // mova [tmp2q-32*1], m2
}

/// `.main_part2_pass1` of `inv_txfm_add_dct_dct_16x64_8bpc`: idct64 steps 6-9 and the idct16/32/64 sums and
/// differences, back into the frame; `tmp1q` and `tmp2q` move towards each other until they meet.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_main_part2_pass1(m: &mut M, tmp1: &mut *mut __m256i, tmp2: &mut *mut __m256i) {
    // sub r6, o_idct64_offset + 8
    m[11] = pw2(1567, 3784); // vpbroadcastd m11, [o(pw_1567_3784)]
    m[12] = pw2(-3784, 1567); // vpbroadcastd m12, [o(pw_m3784_1567)]
    m[13] = pw2(2896, 2896); // vpbroadcastd m13, [o(pw_2896_2896)]
    m[14] = pw2(-2896, 2896); // vpbroadcastd m14, [o(pw_m2896_2896)]
    loop {
        // .main_part2_pass1_loop:
        inv_txfm_add_dct_dct_16x64_main_part2_internal(m, tmp1, tmp2); // call .main_part2_internal
        idct64_part2_end(m, 0, 7, 0, 6, 9, 10, *tmp1, *tmp2, None); // IDCT64_PART2_END 0, 7, 0, 6, 9, 10
        idct64_part2_end(m, 7, 8, 5, 0, 6, 7, *tmp1, *tmp2, None); // IDCT64_PART2_END 7, 8, 5, 0, 6, 7
        idct64_part2_end(m, 8, 2, 1, 0, 6, 7, *tmp1, *tmp2, None); // IDCT64_PART2_END 8, 2, 1, 0, 6, 7
        idct64_part2_end(m, 15, 3, 4, 0, 6, 7, *tmp1, *tmp2, None); // IDCT64_PART2_END 15, 3, 4, 0, 6, 7
        if *tmp1 == *tmp2 {
            break; // cmp tmp1q, tmp2q / jne .main_part2_pass1_loop
        }
    }
}

/// `.main_part2_internal` of `inv_txfm_add_dct_dct_16x64_8bpc` (`m11`-`m14` hold its constants, `m15`
/// `pd_2048`); advances `tmp1q` and steps `tmp2q` back by one slot.
#[inline(always)]
pub(crate) unsafe fn inv_txfm_add_dct_dct_16x64_main_part2_internal(
    m: &mut M,
    tmp1: &mut *mut __m256i,
    tmp2: &mut *mut __m256i,
) {
    m[0] = ld_p5(*tmp1, -32 * 12); // mova m0, [tmp1q-32*12] ; t32a
    m[6] = ld_p5(*tmp2, -32 * 13); // mova m6, [tmp2q-32*13] ; t39a
    m[1] = ld_p5(*tmp1, -32 * 4); // mova m1, [tmp1q-32* 4] ; t40a
    m[5] = ld_p5(*tmp2, 32 * 3); // mova m5, [tmp2q+32* 3] ; t55a
    *tmp1 = at(*tmp1, 32); // add tmp1q, 32
    *tmp2 = at(*tmp2, -32); // sub tmp2q, 32
    m[2] = ld_p5(*tmp1, 32 * 3); // mova m2, [tmp1q+32* 3] ; t48a
    m[4] = ld_p5(*tmp2, -32 * 4); // mova m4, [tmp2q-32* 4] ; t47a
    m[3] = ld_p5(*tmp1, 32 * 11); // mova m3, [tmp1q+32*11] ; t56a
    m[7] = ld_p5(*tmp2, 32 * 12); // mova m7, [tmp2q+32*12] ; t63a
    m[8] = _mm256_subs_epi16(m[0], m[6]); // psubsw m8, m0, m6 ; t39
    m[0] = _mm256_adds_epi16(m[0], m[6]); // paddsw m0, m6 ; t32
    m[6] = _mm256_subs_epi16(m[4], m[1]); // psubsw m6, m4, m1 ; t40
    m[4] = _mm256_adds_epi16(m[4], m[1]); // paddsw m4, m1 ; t47
    m[1] = _mm256_subs_epi16(m[2], m[5]); // psubsw m1, m2, m5 ; t55
    m[2] = _mm256_adds_epi16(m[2], m[5]); // paddsw m2, m5 ; t48
    m[5] = _mm256_subs_epi16(m[7], m[3]); // psubsw m5, m7, m3 ; t56
    m[7] = _mm256_adds_epi16(m[7], m[3]); // paddsw m7, m3 ; t63
    itx_mulsub_2w(m, 5, 8, 3, 9, 15, 11, 12, None); // t39a, t56a
    m[9] = pw2(-1567, -3784); // vpbroadcastd m9, [o(pw_m1567_m3784)]
    itx_mulsub_2w(m, 1, 6, 3, 9, 15, 12, 9, None); // t40a, t55a
    m[3] = _mm256_subs_epi16(m[0], m[4]); // psubsw m3, m0, m4 ; t47a
    m[0] = _mm256_adds_epi16(m[0], m[4]); // paddsw m0, m4 ; t32a
    m[4] = _mm256_subs_epi16(m[7], m[2]); // psubsw m4, m7, m2 ; t48a
    m[7] = _mm256_adds_epi16(m[7], m[2]); // paddsw m7, m2 ; t63a
    m[2] = _mm256_subs_epi16(m[5], m[1]); // psubsw m2, m5, m1 ; t40
    m[5] = _mm256_adds_epi16(m[5], m[1]); // paddsw m5, m1 ; t39
    m[1] = _mm256_subs_epi16(m[8], m[6]); // psubsw m1, m8, m6 ; t55
    m[8] = _mm256_adds_epi16(m[8], m[6]); // paddsw m8, m6 ; t56
    itx_mulsub_2w(m, 4, 3, 6, 9, 15, 13, 14, None); // t47, t48
    itx_mulsub_2w(m, 1, 2, 6, 9, 15, 13, 14, None); // t40a, t55a
}

/// `.main_part2_pass2` of `inv_txfm_add_dct_dct_16x64_8bpc`: as `.main_part2_pass1`, but the 64 output rows are
/// added to the pixels of 16 columns at `dstq`, which is left four rows on (the loop's `add dstq, strideq`; the
/// caller 32x64 continues from there).
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_main_part2_pass2(
    m: &mut M,
    stride: isize,
    dst: &mut Dst,
    tmp1: &mut *mut __m256i,
    tmp2: &mut *mut __m256i,
) {
    // sub r6, o_idct64_offset + 8
    m[11] = pw2(1567, 3784); // vpbroadcastd m11, [o(pw_1567_3784)]
    m[12] = pw2(-3784, 1567); // vpbroadcastd m12, [o(pw_m3784_1567)]
    m[13] = pw2(2896, 2896); // vpbroadcastd m13, [o(pw_2896_2896)]
    let r9 = stride * 5; // lea r9, [strideq*5] ; stride*5
    let r3 = r9 + stride; // lea r3, [r9+strideq*1] ; stride*6
    let r7 = r9 + stride * 2; // lea r7, [r9+strideq*2] ; stride*7
    let r8 = r3 + stride * 2; // lea r8, [r3+strideq*2] ; stride*8
    let mut r2 = at(*dst, r7); // lea r2, [dstq+r7]
    loop {
        // .main_part2_pass2_loop:
        m[14] = pw2(-2896, 2896); // vpbroadcastd m14, [o(pw_m2896_2896)]
        inv_txfm_add_dct_dct_16x64_main_part2_internal(m, tmp1, tmp2); // call .main_part2_internal
        m[14] = pw(2048); // vpbroadcastd m14, [o(pw_2048)]
        let o1 = [0, r3 * 4, r8 * 4, r7 * 8];
        let o2 = [stride * 8, r8 * 2, r9 * 8, r3 * 8];
        idct64_part2_end(m, 0, 7, 0, 6, 9, 10, *tmp1, *tmp2, Some((*dst, r2, o1))); // IDCT64_PART2_END 0, 7, 0, 6, 9, 10, strideq*0, r3*4, r8*4, r7*8
        idct64_part2_end(m, 7, 8, 5, 0, 6, 7, *tmp1, *tmp2, Some((*dst, r2, o1))); // IDCT64_PART2_END 7, 8, 5, 0, 6, 7, strideq*0, r3*4, r8*4, r7*8
        idct64_part2_end(m, 8, 2, 1, 0, 6, 7, *tmp1, *tmp2, Some((*dst, r2, o2))); // IDCT64_PART2_END 8, 2, 1, 0, 6, 7, strideq*8, r8*2, r9*8, r3*8
        idct64_part2_end(m, 15, 3, 4, 0, 6, 7, *tmp1, *tmp2, Some((*dst, r2, o2))); // IDCT64_PART2_END 15, 3, 4, 0, 6, 7, strideq*8, r8*2, r9*8, r3*8
        *dst = at(*dst, stride); // add dstq, strideq
        r2 = at(r2, -stride); // sub r2, strideq
        if *tmp1 == *tmp2 {
            break; // cmp tmp1q, tmp2q / jne .main_part2_pass2_loop
        }
    }
}

// 16x64

/// `cglobal inv_txfm_add_dct_dct_16x64_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 64; // or r3d, 64
        return inv_txfm_add_dct_dct_16x4_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_16x4_8bpc).dconly
    }
    // .normal: PROLOGUE 0, 10, 16, 32*67, dst, stride, c, eob, tmp1, tmp2. The frame is zeroed whole (see the top
    // of this part).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 67]>::uninit();
    let rsp: *mut __m256i = zeroed_frame(&mut frame);
    let mut cq = c;
    let mut tmp1 = at(rsp, 32 * 23); // lea tmp1q, [rsp+32*23]
    let mut tmp2 = at(tmp1, 32 * 24); // lea tmp2q, [tmp1q+32*24]
    let mut eobd = eob.wrapping_sub(151); // sub eobd, 151
    let r7d = eobd; // mov r7d, eobd
    loop {
        // .pass1_loop:
        load_16rows(m, s3_p5(rsp), cq, 64, 0, 1); // LOAD_16ROWS cq, 64
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
        st_p5(rsp, 32 * 0, m[6]); // mova [rsp+32*0], m6
        st_p5(rsp, 32 * 1, m[7]); // mova [rsp+32*1], m7
        m[7] = pw(8192); // vpbroadcastd m7, [o(pw_8192)]
        inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, s3_p5(rsp)); // call m(inv_txfm_add_dct_dct_16x32_8bpc).transpose_2x8x8_round
        m[15] = ld_p5(rsp, 32 * 0); // mova m15, [rsp+32*0]
        st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        st_p5(tmp1, -32 * 3, m[2]); // mova [tmp1q-32*3], m2
        st_p5(tmp1, -32 * 2, m[4]); // mova [tmp1q-32*2], m4
        st_p5(tmp1, -32 * 1, m[6]); // mova [tmp1q-32*1], m6
        st_p5(tmp1, 32 * 0, m[8]); // mova [tmp1q+32*0], m8
        st_p5(tmp1, 32 * 1, m[10]); // mova [tmp1q+32*1], m10
        st_p5(tmp1, 32 * 2, m[12]); // mova [tmp1q+32*2], m12
        st_p5(tmp1, 32 * 3, m[14]); // mova [tmp1q+32*3], m14
        st_p5(tmp2, -32 * 4, m[1]); // mova [tmp2q-32*4], m1
        st_p5(tmp2, -32 * 3, m[3]); // mova [tmp2q-32*3], m3
        st_p5(tmp2, -32 * 2, m[5]); // mova [tmp2q-32*2], m5
        st_p5(tmp2, -32 * 1, m[7]); // mova [tmp2q-32*1], m7
        st_p5(tmp2, 32 * 0, m[9]); // mova [tmp2q+32*0], m9
        st_p5(tmp2, 32 * 1, m[11]); // mova [tmp2q+32*1], m11
        st_p5(tmp2, 32 * 2, m[13]); // mova [tmp2q+32*2], m13
        st_p5(tmp2, 32 * 3, m[15]); // mova [tmp2q+32*3], m15
        cq = at(cq, 32); // add cq, 32
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        tmp2 = at(tmp2, 32 * 8); // add tmp2q, 32*8
        let (e, carry) = (eobd as u32).overflowing_add(0x8000_0000); // add eobd, 0x80000000
        eobd = e as i32;
        if carry {
            break; // jnc .pass1_loop
        }
    }
    let mut r2 = at(rsp, 32 * 23); // lea r2, [rsp+32*23]
    m[0] = zx(ldx_p5(r2, -32 * 4 + 0)); // mova xm0, [r2-32*4+ 0]
    m[1] = zx(ldx_p5(r2, -32 * 2 + 0)); // mova xm1, [r2-32*2+ 0]
    m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_p5(r2, 32 * 0 + 0)); // vinserti128 m0, [r2+32*0+ 0], 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_p5(r2, 32 * 2 + 0)); // vinserti128 m1, [r2+32*2+ 0], 1
    m[2] = zx(ldx_p5(r2, -32 * 4 + 16)); // mova xm2, [r2-32*4+16]
    m[3] = zx(ldx_p5(r2, -32 * 2 + 16)); // mova xm3, [r2-32*2+16]
    m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(r2, 32 * 0 + 16)); // vinserti128 m2, [r2+32*0+16], 1
    m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(r2, 32 * 2 + 16)); // vinserti128 m3, [r2+32*2+16], 1
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    repx!([5, 6, 7, 8, 9, 10, 11, 12, 13, 14], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, …, m14
    let mut r3 = r2;
    if r7d >= 0 {
        // test r7d, r7d / jl .fast
        r3 = at(r2, 32 * 8); // lea r3, [r2+32*8]
        m[4] = zx(ldx_p5(r3, -32 * 4 + 0)); // mova xm4, [r3-32*4+ 0]
        m[5] = zx(ldx_p5(r3, -32 * 2 + 0)); // mova xm5, [r3-32*2+ 0]
        m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(r3, 32 * 0 + 0)); // vinserti128 m4, [r3+32*0+ 0], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(r3, 32 * 2 + 0)); // vinserti128 m5, [r3+32*2+ 0], 1
        m[6] = zx(ldx_p5(r3, -32 * 4 + 16)); // mova xm6, [r3-32*4+16]
        m[7] = zx(ldx_p5(r3, -32 * 2 + 16)); // mova xm7, [r3-32*2+16]
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(r3, 32 * 0 + 16)); // vinserti128 m6, [r3+32*0+16], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(r3, 32 * 2 + 16)); // vinserti128 m7, [r3+32*2+16], 1
    }
    // .fast:
    st_p5(rsp, 0, m[8]); // mova [rsp], m8
    tmp1 = at(rsp, 32 * 7); // lea tmp1q, [rsp+32*7]
    idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
    m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
    st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
    st_p5(tmp1, -32 * 3, m[1]); // mova [tmp1q-32*3], m1
    st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
    st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
    st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
    st_p5(tmp1, 32 * 1, m[5]); // mova [tmp1q+32*1], m5
    st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
    st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
    tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
    st_p5(tmp1, -32 * 4, m[8]); // mova [tmp1q-32*4], m8
    st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
    st_p5(tmp1, -32 * 2, m[10]); // mova [tmp1q-32*2], m10
    st_p5(tmp1, -32 * 1, m[11]); // mova [tmp1q-32*1], m11
    st_p5(tmp1, 32 * 0, m[12]); // mova [tmp1q+32*0], m12
    st_p5(tmp1, 32 * 1, m[13]); // mova [tmp1q+32*1], m13
    st_p5(tmp1, 32 * 2, m[14]); // mova [tmp1q+32*2], m14
    st_p5(tmp1, 32 * 3, m[15]); // mova [tmp1q+32*3], m15
    m[0] = zx(ldx_p5(r2, -32 * 3 + 0)); // mova xm0, [r2-32*3+ 0]
    m[1] = zx(ldx_p5(r2, -32 * 1 + 0)); // mova xm1, [r2-32*1+ 0]
    m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_p5(r2, 32 * 1 + 0)); // vinserti128 m0, [r2+32*1+ 0], 1
    m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_p5(r2, 32 * 3 + 0)); // vinserti128 m1, [r2+32*3+ 0], 1
    m[2] = zx(ldx_p5(r2, -32 * 3 + 16)); // mova xm2, [r2-32*3+16]
    m[3] = zx(ldx_p5(r2, -32 * 1 + 16)); // mova xm3, [r2-32*1+16]
    m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(r2, 32 * 1 + 16)); // vinserti128 m2, [r2+32*1+16], 1
    m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(r2, 32 * 3 + 16)); // vinserti128 m3, [r2+32*3+16], 1
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    repx!([5, 6, 7], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, m7
    if r7d >= 0 {
        // test r7d, r7d / jl .fast2
        m[4] = zx(ldx_p5(r3, -32 * 3 + 0)); // mova xm4, [r3-32*3+ 0]
        m[5] = zx(ldx_p5(r3, -32 * 1 + 0)); // mova xm5, [r3-32*1+ 0]
        m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(r3, 32 * 1 + 0)); // vinserti128 m4, [r3+32*1+ 0], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(r3, 32 * 3 + 0)); // vinserti128 m5, [r3+32*3+ 0], 1
        m[6] = zx(ldx_p5(r3, -32 * 3 + 16)); // mova xm6, [r3-32*3+16]
        m[7] = zx(ldx_p5(r3, -32 * 1 + 16)); // mova xm7, [r3-32*1+16]
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(r3, 32 * 1 + 16)); // vinserti128 m6, [r3+32*1+16], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(r3, 32 * 3 + 16)); // vinserti128 m7, [r3+32*3+16], 1
    }
    // .fast2:
    tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
    tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
    inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    r2 = at(r2, 32 * 24); // add r2, 32*24
    m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
    tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
    tmp2 = at(tmp2, 32 * 32); // add tmp2q, 32*32
    m[0] = zx(ldx_p5(r2, -32 * 4 + 0)); // mova xm0, [r2-32*4+ 0]
    m[3] = zx(ldx_p5(r2, -32 * 1 + 16)); // mova xm3, [r2-32*1+16]
    m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_p5(r2, 32 * 0 + 0)); // vinserti128 m0, [r2+32*0+ 0], 1
    m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(r2, 32 * 3 + 16)); // vinserti128 m3, [r2+32*3+16], 1
    m[4] = zx(ldx_p5(r2, -32 * 4 + 16)); // mova xm4, [r2-32*4+16]
    m[7] = zx(ldx_p5(r2, -32 * 1 + 0)); // mova xm7, [r2-32*1+ 0]
    m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(r2, 32 * 0 + 16)); // vinserti128 m4, [r2+32*0+16], 1
    m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(r2, 32 * 3 + 0)); // vinserti128 m7, [r2+32*3+ 0], 1
    m[1] = _mm256_setzero_si256(); // pxor m1, m1
    repx!([2, 5, 6], i => m[i] = m[1]); // REPX {mova x, m1}, m2, m5, m6
    if r7d >= 0 {
        // test r7d, r7d / jl .fast3
        r3 = at(r3, 32 * 24); // add r3, 32*24
        m[1] = zx(ldx_p5(r3, -32 * 1 + 16)); // mova xm1, [r3-32*1+16]
        m[2] = zx(ldx_p5(r3, -32 * 4 + 0)); // mova xm2, [r3-32*4+ 0]
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_p5(r3, 32 * 3 + 16)); // vinserti128 m1, [r3+32*3+16], 1
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(r3, 32 * 0 + 0)); // vinserti128 m2, [r3+32*0+ 0], 1
        m[5] = zx(ldx_p5(r3, -32 * 1 + 0)); // mova xm5, [r3-32*1+ 0]
        m[6] = zx(ldx_p5(r3, -32 * 4 + 16)); // mova xm6, [r3-32*4+16]
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(r3, 32 * 3 + 0)); // vinserti128 m5, [r3+32*3+ 0], 1
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(r3, 32 * 0 + 16)); // vinserti128 m6, [r3+32*0+16], 1
    }
    // .fast3:
    // add r6, o_idct64_offset
    inv_txfm_add_dct_dct_16x64_main_part1(m, 0, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
                                                             // add r6, 8
    tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
    tmp2 = at(tmp2, -32 * 8); // sub tmp2q, 32*8
    m[0] = zx(ldx_p5(r2, -32 * 2 + 0)); // mova xm0, [r2-32*2+ 0]
    m[3] = zx(ldx_p5(r2, -32 * 3 + 16)); // mova xm3, [r2-32*3+16]
    m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_p5(r2, 32 * 2 + 0)); // vinserti128 m0, [r2+32*2+ 0], 1
    m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(r2, 32 * 1 + 16)); // vinserti128 m3, [r2+32*1+16], 1
    m[4] = zx(ldx_p5(r2, -32 * 2 + 16)); // mova xm4, [r2-32*2+16]
    m[7] = zx(ldx_p5(r2, -32 * 3 + 0)); // mova xm7, [r2-32*3+ 0]
    m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(r2, 32 * 2 + 16)); // vinserti128 m4, [r2+32*2+16], 1
    m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(r2, 32 * 1 + 0)); // vinserti128 m7, [r2+32*1+ 0], 1
    m[1] = _mm256_setzero_si256(); // pxor m1, m1
    repx!([2, 5, 6], i => m[i] = m[1]); // REPX {mova x, m1}, m2, m5, m6
    if r7d >= 0 {
        // test r7d, r7d / jl .fast4
        m[1] = zx(ldx_p5(r3, -32 * 3 + 16)); // mova xm1, [r3-32*3+16]
        m[2] = zx(ldx_p5(r3, -32 * 2 + 0)); // mova xm2, [r3-32*2+ 0]
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_p5(r3, 32 * 1 + 16)); // vinserti128 m1, [r3+32*1+16], 1
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(r3, 32 * 2 + 0)); // vinserti128 m2, [r3+32*2+ 0], 1
        m[5] = zx(ldx_p5(r3, -32 * 3 + 0)); // mova xm5, [r3-32*3+ 0]
        m[6] = zx(ldx_p5(r3, -32 * 2 + 16)); // mova xm6, [r3-32*2+16]
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(r3, 32 * 1 + 0)); // vinserti128 m5, [r3+32*1+ 0], 1
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(r3, 32 * 2 + 16)); // vinserti128 m6, [r3+32*2+16], 1
    }
    // .fast4:
    inv_txfm_add_dct_dct_16x64_main_part1(m, 2, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    inv_txfm_add_dct_dct_16x64_main_part2_pass2(m, stride, &mut dst, &mut tmp1, &mut tmp2);
    // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_pass2
}

inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_16x64_8bpc_avx2, inv_txfm_add_dct_dct_16x64);

// 64x16

/// `.dconly` of `inv_txfm_add_dct_dct_64x16_8bpc` (also the DC-only tail of 64x32 and 64x64)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_dconly(m: &mut M, mut dst: Dst, stride: isize, mut r3d: c_int) {
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[2] = movd_pw(2048); // movd xm2, [o(pw_2048)]
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[2]))); // pmulhrsw xm0, xm2
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[1] = _mm256_setzero_si256(); // pxor m1, m1
    loop {
        // .dconly_loop:
        m[2] = _mm256_loadu_si256(at(dst, 32 * 0).cast()); // mova m2, [dstq+32*0]
        m[3] = _mm256_loadu_si256(at(dst, 32 * 1).cast()); // mova m3, [dstq+32*1]
        m[4] = _mm256_unpackhi_epi8(m[2], m[1]); // punpckhbw m4, m2, m1
        m[2] = _mm256_unpacklo_epi8(m[2], m[1]); // punpcklbw m2, m1
        m[5] = _mm256_unpackhi_epi8(m[3], m[1]); // punpckhbw m5, m3, m1
        m[3] = _mm256_unpacklo_epi8(m[3], m[1]); // punpcklbw m3, m1
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[5] = _mm256_add_epi16(m[5], m[0]); // paddw m5, m0
        m[3] = _mm256_add_epi16(m[3], m[0]); // paddw m3, m0
        m[2] = _mm256_packus_epi16(m[2], m[4]); // packuswb m2, m4
        m[3] = _mm256_packus_epi16(m[3], m[5]); // packuswb m3, m5
        _mm256_storeu_si256(at(dst, 32 * 0).cast(), m[2]); // mova [dstq+32*0], m2
        _mm256_storeu_si256(at(dst, 32 * 1).cast(), m[3]); // mova [dstq+32*1], m3
        dst = at(dst, stride); // add dstq, strideq
        r3d -= 1; // dec r3d
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `mova m, [cq+32*row]` for the eight rows, then `pxor m8, m8` and `REPX {mova [cq+32*x], m8}` over the same rows
/// (the idct64 inputs of `.main_part1` in the first passes of 64x16, 64x32 and 64x64)
#[inline(always)]
unsafe fn load_clear_8rows_p5(m: &mut M, c: Coef, stride: isize, rows: [isize; 8]) {
    repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_loadu_si256(at(c, stride * rows[i]).cast())); // mova mi, [cq+32*row]
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    repx!([0, 1, 2, 3, 4, 5, 6, 7], i => _mm256_storeu_si256(at(c, stride * rows[i]).cast(), m[8]));
    // REPX {mova [cq+32*x], m8}, rows
}

/// `cglobal inv_txfm_add_dct_dct_64x16_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 16; // or r3d, 16
        return inv_txfm_add_dct_dct_64x16_dconly(m, dst, stride, r3d);
    }
    // .normal: PROLOGUE 0, 7, 16, 32*67, dst, stride, c, eob, tmp1, tmp2. The frame is zeroed whole (see the top
    // of this part).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 67]>::uninit();
    let rsp: *mut __m256i = zeroed_frame(&mut frame);
    load_8rows(m, at(c, 32 * 0), 32 * 4, 0); // LOAD_8ROWS cq+32*0, 32*4
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    repx!([0, 4, 8, 12, 16, 20, 24, 28], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, 0, 4, 8, 12, 16, 20, 24, 28
    repx!([9, 10, 11, 12, 13, 14], i => m[i] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
    st_p5(rsp, 0, m[8]); // mova [rsp], m8
    let mut tmp1 = at(rsp, 32 * 7); // lea tmp1q, [rsp+32*7]
    idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
    m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
    st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
    st_p5(tmp1, -32 * 3, m[1]); // mova [tmp1q-32*3], m1
    st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
    st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
    st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
    st_p5(tmp1, 32 * 1, m[5]); // mova [tmp1q+32*1], m5
    st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
    st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
    tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
    st_p5(tmp1, -32 * 4, m[8]); // mova [tmp1q-32*4], m8
    st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
    st_p5(tmp1, -32 * 2, m[10]); // mova [tmp1q-32*2], m10
    st_p5(tmp1, -32 * 1, m[11]); // mova [tmp1q-32*1], m11
    st_p5(tmp1, 32 * 0, m[12]); // mova [tmp1q+32*0], m12
    st_p5(tmp1, 32 * 1, m[13]); // mova [tmp1q+32*1], m13
    st_p5(tmp1, 32 * 2, m[14]); // mova [tmp1q+32*2], m14
    st_p5(tmp1, 32 * 3, m[15]); // mova [tmp1q+32*3], m15
    load_8rows(m, at(c, 32 * 2), 32 * 4, 0); // LOAD_8ROWS cq+32*2, 32*4
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    repx!([2, 6, 10, 14, 18, 22, 26, 30], i => _mm256_storeu_si256(at(c, 32 * i).cast(), m[8])); // REPX {mova [cq+32*x], m8}, 2, 6, 10, 14, 18, 22, 26, 30
    tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
    let mut tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
    inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
    tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
    tmp2 = at(tmp2, 32 * 32); // add tmp2q, 32*32
    load_clear_8rows_p5(m, c, 32, [1, 31, 17, 15, 9, 23, 25, 7]); // mova m0..m7, [cq+32*1, 31, 17, 15, 9, 23, 25, 7] / pxor m8, m8 / REPX {mova [cq+32*x], m8}, …
                                                                  // add r6, o_idct64_offset
    inv_txfm_add_dct_dct_16x64_main_part1(m, 0, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
                                                             // add r6, 8
    tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
    tmp2 = at(tmp2, -32 * 8); // sub tmp2q, 32*8
    load_clear_8rows_p5(m, c, 32, [5, 27, 21, 11, 13, 19, 29, 3]); // mova m0..m7, [cq+32*5, 27, 21, 11, 13, 19, 29, 3] / pxor m8, m8 / REPX {mova [cq+32*x], m8}, …
    inv_txfm_add_dct_dct_16x64_main_part1(m, 2, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    inv_txfm_add_dct_dct_16x64_main_part2_pass1(m, &mut tmp1, &mut tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_pass1
    tmp1 = at(tmp1, -32 * 36); // sub tmp1q, 32*36
    let r2 = stride * 3; // lea r2, [strideq*3]
    let mut tmp2d = 4; // mov tmp2d, 4
    loop {
        // .pass2_loop:
        let r3 = at(tmp1, -32 * 8); // lea r3, [tmp1q-32*8]
        m[0] = zx(ldx_p5(r3, -32 * 4)); // mova xm0, [r3 -32*4]
        m[1] = zx(ldx_p5(r3, -32 * 3)); // mova xm1, [r3 -32*3]
        m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_p5(tmp1, -32 * 4)); // vinserti128 m0, [tmp1q-32*4], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_p5(tmp1, -32 * 3)); // vinserti128 m1, [tmp1q-32*3], 1
        m[2] = zx(ldx_p5(r3, -32 * 2)); // mova xm2, [r3 -32*2]
        m[3] = zx(ldx_p5(r3, -32 * 1)); // mova xm3, [r3 -32*1]
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(tmp1, -32 * 2)); // vinserti128 m2, [tmp1q-32*2], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(tmp1, -32 * 1)); // vinserti128 m3, [tmp1q-32*1], 1
        m[4] = zx(ldx_p5(r3, 32 * 0)); // mova xm4, [r3 +32*0]
        m[5] = zx(ldx_p5(r3, 32 * 1)); // mova xm5, [r3 +32*1]
        m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(tmp1, 32 * 0)); // vinserti128 m4, [tmp1q+32*0], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(tmp1, 32 * 1)); // vinserti128 m5, [tmp1q+32*1], 1
        m[6] = zx(ldx_p5(r3, 32 * 2)); // mova xm6, [r3 +32*2]
        m[7] = zx(ldx_p5(r3, 32 * 3)); // mova xm7, [r3 +32*3]
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(tmp1, 32 * 2)); // vinserti128 m6, [tmp1q+32*2], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(tmp1, 32 * 3)); // vinserti128 m7, [tmp1q+32*3], 1
        m[8] = zx(ldx_p5(r3, -32 * 4 + 16)); // mova xm8, [r3 -32*4+16]
        m[9] = zx(ldx_p5(r3, -32 * 3 + 16)); // mova xm9, [r3 -32*3+16]
        m[8] = _mm256_inserti128_si256::<1>(m[8], ldx_p5(tmp1, -32 * 4 + 16)); // vinserti128 m8, [tmp1q-32*4+16], 1
        m[9] = _mm256_inserti128_si256::<1>(m[9], ldx_p5(tmp1, -32 * 3 + 16)); // vinserti128 m9, [tmp1q-32*3+16], 1
        m[10] = zx(ldx_p5(r3, -32 * 2 + 16)); // mova xm10, [r3 -32*2+16]
        m[11] = zx(ldx_p5(r3, -32 * 1 + 16)); // mova xm11, [r3 -32*1+16]
        m[10] = _mm256_inserti128_si256::<1>(m[10], ldx_p5(tmp1, -32 * 2 + 16)); // vinserti128 m10, [tmp1q-32*2+16], 1
        m[11] = _mm256_inserti128_si256::<1>(m[11], ldx_p5(tmp1, -32 * 1 + 16)); // vinserti128 m11, [tmp1q-32*1+16], 1
        m[12] = zx(ldx_p5(r3, 32 * 0 + 16)); // mova xm12, [r3 +32*0+16]
        m[13] = zx(ldx_p5(r3, 32 * 1 + 16)); // mova xm13, [r3 +32*1+16]
        m[12] = _mm256_inserti128_si256::<1>(m[12], ldx_p5(tmp1, 32 * 0 + 16)); // vinserti128 m12, [tmp1q+32*0+16], 1
        m[13] = _mm256_inserti128_si256::<1>(m[13], ldx_p5(tmp1, 32 * 1 + 16)); // vinserti128 m13, [tmp1q+32*1+16], 1
        m[14] = zx(ldx_p5(r3, 32 * 2 + 16)); // mova xm14, [r3 +32*2+16]
        m[15] = zx(ldx_p5(r3, 32 * 3 + 16)); // mova xm15, [r3 +32*3+16]
        m[14] = _mm256_inserti128_si256::<1>(m[14], ldx_p5(tmp1, 32 * 2 + 16)); // vinserti128 m14, [tmp1q+32*2+16], 1
        m[15] = _mm256_inserti128_si256::<1>(m[15], ldx_p5(tmp1, 32 * 3 + 16)); // vinserti128 m15, [tmp1q+32*3+16], 1
        st_p5(rsp, 32 * 0, m[6]); // mova [rsp+32*0], m6
        st_p5(rsp, 32 * 1, m[7]); // mova [rsp+32*1], m7
        m[7] = pw(8192); // vpbroadcastd m7, [o(pw_8192)]
        inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, s3_p5(rsp)); // call m(inv_txfm_add_dct_dct_16x32_8bpc).transpose_2x8x8_round
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        st_p5(rsp, 32 * 0, m[15]); // mova [rsp+32*0], m15
        m[15] = pw(2048); // vpbroadcastd m15, [o(pw_2048)]
        repx!([0, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[15])); // REPX {pmulhrsw x, m15}, m0, m2, m3, m4, m5, m6, m7
        write_16x2(m, dst, m[2], m[3], 1, 2, [stride * 2, r2]); // WRITE_16X2 2, 3, 1, 2, strideq*2, r2
        m[1] = _mm256_mulhrs_epi16(m[15], ld_p5(rsp, 32 * 1)); // pmulhrsw m1, m15, [rsp+32*1]
        write_16x2(m, dst, m[0], m[1], 2, 3, [0, stride]); // WRITE_16X2 0, 1, 2, 3, strideq*0, strideq*1
        let mut d = at(dst, stride * 4); // lea r3, [dstq+strideq*4] / %define dstq r3 (`d` is that r3)
        write_16x2(m, d, m[4], m[5], 2, 3, [0, stride]); // WRITE_16X2 4, 5, 2, 3, strideq*0, strideq*1
        write_16x2(m, d, m[6], m[7], 2, 3, [stride * 2, r2]); // WRITE_16X2 6, 7, 2, 3, strideq*2, r2
        repx!([8, 9, 10, 11, 12, 13, 14], i => m[i] = _mm256_mulhrs_epi16(m[i], m[15])); // REPX {pmulhrsw x, m15}, m8, m9, m10, m11, m12, m13, m14
        d = at(d, stride * 4); // lea r3, [r3+strideq*4]
        write_16x2(m, d, m[8], m[9], 2, 3, [0, stride]); // WRITE_16X2 8, 9, 2, 3, strideq*0, strideq*1
        write_16x2(m, d, m[10], m[11], 2, 3, [stride * 2, r2]); // WRITE_16X2 10, 11, 2, 3, strideq*2, r2
        m[15] = _mm256_mulhrs_epi16(m[15], ld_p5(rsp, 32 * 0)); // pmulhrsw m15, [rsp+32*0]
        d = at(d, stride * 4); // lea r3, [r3+strideq*4]
        write_16x2(m, d, m[12], m[13], 2, 3, [0, stride]); // WRITE_16X2 12, 13, 2, 3, strideq*0, strideq*1
        write_16x2(m, d, m[14], m[15], 2, 3, [stride * 2, r2]); // WRITE_16X2 14, 15, 2, 3, strideq*2, r2
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        dst = at(dst, 16); // add r0, 16
        tmp2d -= 1; // dec tmp2d
        if tmp2d <= 0 {
            break; // jg .pass2_loop
        }
    }
}

inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_64x16_8bpc_avx2, inv_txfm_add_dct_dct_64x16);

// 32x64

/// `cglobal inv_txfm_add_dct_dct_32x64_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x64(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        let r3d = eob | 64; // or r3d, 64
        return inv_txfm_add_dct_dct_32x8_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_32x8_8bpc).dconly
    }
    // .normal: PROLOGUE 0, 11, 16, 32*99, dst, stride, c, eob, tmp1, tmp2. The frame is zeroed whole (see the top of
    // this part).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 99]>::uninit();
    let rsp = zeroed_frame(&mut frame);
    let mut tmp1 = at(rsp, 32 * 7); // lea tmp1q, [rsp+32*7]
                                    // r10d is -1 (r10b nonzero: the fast paths) for an eob below 136, else 0; the loops count in its top bit.
    let mut r10d = (eob.wrapping_sub(136) >> 31) as u32; // lea r10d, [eobq-136] / sar r10d, 31
    loop {
        // .pass1_loop:
        let tmp2 = at(tmp1, 32 * 16); // lea tmp2q, [tmp1q+32*16]
        load_8rows(m, at(c, 64 * 1), 64 * 2, 1); // LOAD_8ROWS cq+64*1, 64*2, 1
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([1, 3, 5, 7, 9, 11, 13, 15], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[8])); // REPX {mova [cq+64*x], m8}, 1, 3, 5, 7, 9, 11, 13, 15
        if r10d as u8 == 0 {
            // test r10b, r10b / jnz .fast
            load_8rows_h(m, at(c, 64 * 17), 64 * 2, 2); // LOAD_8ROWS_H cq+64*17, 64*2, 2
            inv_txfm_add_dct_dct_16x32_main_oddhalf(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
            load_8rows_h(m, at(c, 64 * 16), 64 * 2, 1); // LOAD_8ROWS_H cq+64*16, 64*2, 1
            st_p5(rsp, 0, m[15]); // mova [rsp], m15
            m[15] = _mm256_setzero_si256(); // pxor m15, m15
            repx!([16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[15]));
        // REPX {mova [cq+64*x], m15}, 16, 17, …, 31
        // jmp .idct16
        } else {
            // .fast:
            inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
            m[8] = _mm256_setzero_si256(); // pxor m8, m8
            repx!([9, 10, 11, 12, 13, 14], i => m[i] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
            st_p5(rsp, 0, m[8]); // mova [rsp], m8
        }
        // .idct16:
        load_8rows(m, at(c, 64 * 0), 64 * 2, 1); // LOAD_8ROWS cq+64*0, 64*2, 1
        m[15] = _mm256_setzero_si256(); // pxor m15, m15
        repx!([0, 2, 4, 6, 8, 10, 12, 14], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[15])); // REPX {mova [cq+64*x], m15}, 0, 2, 4, 6, 8, 10, 12, 14
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_32x16_pass1_end(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_32x16_8bpc).pass1_end
        m[7] = pw(16384); // vpbroadcastd m7, [o(pw_16384)]
        inv_txfm_add_dct_dct_16x32_transpose_2x8x8_round(m, s3_p5(rsp)); // call m(inv_txfm_add_dct_dct_16x32_8bpc).transpose_2x8x8_round
        let mut r3 = at(tmp1, 32 * 48); // lea r3, [tmp1q+32*48]
        m[15] = ld_p5(rsp, 0); // mova m15, [rsp]
        st_p5(r3, -32 * 4, m[0]); // mova [r3-32*4], m0
        st_p5(r3, -32 * 3, m[2]); // mova [r3-32*3], m2
        st_p5(r3, -32 * 2, m[4]); // mova [r3-32*2], m4
        st_p5(r3, -32 * 1, m[6]); // mova [r3-32*1], m6
        st_p5(r3, 32 * 0, m[8]); // mova [r3+32*0], m8
        st_p5(r3, 32 * 1, m[10]); // mova [r3+32*1], m10
        st_p5(r3, 32 * 2, m[12]); // mova [r3+32*2], m12
        st_p5(r3, 32 * 3, m[14]); // mova [r3+32*3], m14
        r3 = at(r3, 32 * 24); // add r3, 32*24
        st_p5(r3, -32 * 4, m[1]); // mova [r3-32*4], m1
        st_p5(r3, -32 * 3, m[3]); // mova [r3-32*3], m3
        st_p5(r3, -32 * 2, m[5]); // mova [r3-32*2], m5
        st_p5(r3, -32 * 1, m[7]); // mova [r3-32*1], m7
        st_p5(r3, 32 * 0, m[9]); // mova [r3+32*0], m9
        st_p5(r3, 32 * 1, m[11]); // mova [r3+32*1], m11
        st_p5(r3, 32 * 2, m[13]); // mova [r3+32*2], m13
        st_p5(r3, 32 * 3, m[15]); // mova [r3+32*3], m15
        m[9] = pw(16384); // vpbroadcastd m9, [o(pw_16384)]
        m[0] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, -32 * 4)); // pmulhrsw m0, m9, [tmp1q-32*4]
        m[1] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, -32 * 3)); // pmulhrsw m1, m9, [tmp1q-32*3]
        m[2] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, -32 * 2)); // pmulhrsw m2, m9, [tmp1q-32*2]
        m[3] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, -32 * 1)); // pmulhrsw m3, m9, [tmp1q-32*1]
        m[4] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, 32 * 0)); // pmulhrsw m4, m9, [tmp1q+32*0]
        m[5] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, 32 * 1)); // pmulhrsw m5, m9, [tmp1q+32*1]
        m[6] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, 32 * 2)); // pmulhrsw m6, m9, [tmp1q+32*2]
        m[7] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp1, 32 * 3)); // pmulhrsw m7, m9, [tmp1q+32*3]
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        m[0] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, -32 * 4)); // pmulhrsw m0, m9, [tmp2q-32*4]
        st_p5(tmp2, -32 * 4, m[1]); // mova [tmp2q-32*4], m1
        m[1] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, -32 * 3)); // pmulhrsw m1, m9, [tmp2q-32*3]
        st_p5(tmp1, -32 * 3, m[2]); // mova [tmp1q-32*3], m2
        m[2] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, -32 * 2)); // pmulhrsw m2, m9, [tmp2q-32*2]
        st_p5(tmp2, -32 * 3, m[3]); // mova [tmp2q-32*3], m3
        m[3] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, -32 * 1)); // pmulhrsw m3, m9, [tmp2q-32*1]
        st_p5(tmp1, -32 * 2, m[4]); // mova [tmp1q-32*2], m4
        m[4] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, 32 * 0)); // pmulhrsw m4, m9, [tmp2q+32*0]
        st_p5(tmp2, -32 * 2, m[5]); // mova [tmp2q-32*2], m5
        m[5] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, 32 * 1)); // pmulhrsw m5, m9, [tmp2q+32*1]
        st_p5(tmp1, -32 * 1, m[6]); // mova [tmp1q-32*1], m6
        m[6] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, 32 * 2)); // pmulhrsw m6, m9, [tmp2q+32*2]
        st_p5(tmp2, -32 * 1, m[7]); // mova [tmp2q-32*1], m7
        m[7] = _mm256_mulhrs_epi16(m[9], ld_p5(tmp2, 32 * 3)); // pmulhrsw m7, m9, [tmp2q+32*3]
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        st_p5(tmp1, 32 * 0, m[0]); // mova [tmp1q+32*0], m0
        st_p5(tmp2, 32 * 0, m[1]); // mova [tmp2q+32*0], m1
        st_p5(tmp1, 32 * 1, m[2]); // mova [tmp1q+32*1], m2
        st_p5(tmp2, 32 * 1, m[3]); // mova [tmp2q+32*1], m3
        st_p5(tmp1, 32 * 2, m[4]); // mova [tmp1q+32*2], m4
        st_p5(tmp2, 32 * 2, m[5]); // mova [tmp2q+32*2], m5
        st_p5(tmp1, 32 * 3, m[6]); // mova [tmp1q+32*3], m6
        st_p5(tmp2, 32 * 3, m[7]); // mova [tmp2q+32*3], m7
        c = at(c, 32); // add cq, 32
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        let (r, carry) = r10d.overflowing_add(0x8000_0000); // add r10d, 0x80000000
        r10d = r;
        if carry {
            break; // jnc .pass1_loop
        }
    }
    let mut r2 = at(rsp, 32 * 55); // lea r2, [rsp+32*55]
    let mut r7 = at(r2, 32 * 24); // lea r7, [r2+32*24]
    loop {
        // .pass2_loop:
        let r3 = at(r2, 32 * 8); // lea r3, [r2+32*8]
        let r8 = at(r7, 32 * 8); // lea r8, [r7+32*8]
        m[0] = ld_p5(r2, -32 * 4); // mova m0, [r2-32*4]
        m[1] = ld_p5(r2, -32 * 2); // mova m1, [r2-32*2]
        m[2] = ld_p5(r2, 32 * 0); // mova m2, [r2+32*0]
        m[3] = ld_p5(r2, 32 * 2); // mova m3, [r2+32*2]
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        repx!([5, 6, 7, 8, 9, 10, 11, 12, 13, 14], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, …, m14
        if r10d as u8 == 0 {
            // test r10b, r10b / jnz .fast2
            m[4] = ld_p5(r3, -32 * 4); // mova m4, [r3-32*4]
            m[5] = ld_p5(r3, -32 * 2); // mova m5, [r3-32*2]
            m[6] = ld_p5(r3, 32 * 0); // mova m6, [r3+32*0]
            m[7] = ld_p5(r3, 32 * 2); // mova m7, [r3+32*2]
        }
        // .fast2:
        st_p5(rsp, 0, m[8]); // mova [rsp], m8
        let mut tmp1 = at(rsp, 32 * 39); // lea tmp1q, [rsp+32*39]
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
        st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        st_p5(tmp1, -32 * 3, m[1]); // mova [tmp1q-32*3], m1
        st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
        st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
        st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
        st_p5(tmp1, 32 * 1, m[5]); // mova [tmp1q+32*1], m5
        st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
        st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        st_p5(tmp1, -32 * 4, m[8]); // mova [tmp1q-32*4], m8
        st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
        st_p5(tmp1, -32 * 2, m[10]); // mova [tmp1q-32*2], m10
        st_p5(tmp1, -32 * 1, m[11]); // mova [tmp1q-32*1], m11
        st_p5(tmp1, 32 * 0, m[12]); // mova [tmp1q+32*0], m12
        st_p5(tmp1, 32 * 1, m[13]); // mova [tmp1q+32*1], m13
        st_p5(tmp1, 32 * 2, m[14]); // mova [tmp1q+32*2], m14
        st_p5(tmp1, 32 * 3, m[15]); // mova [tmp1q+32*3], m15
        m[0] = ld_p5(r2, -32 * 3); // mova m0, [r2-32*3]
        m[1] = ld_p5(r2, -32 * 1); // mova m1, [r2-32*1]
        m[2] = ld_p5(r2, 32 * 1); // mova m2, [r2+32*1]
        m[3] = ld_p5(r2, 32 * 3); // mova m3, [r2+32*3]
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        repx!([5, 6, 7], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, m7
        if r10d as u8 == 0 {
            // test r10b, r10b / jnz .fast3
            m[4] = ld_p5(r3, -32 * 3); // mova m4, [r3-32*3]
            m[5] = ld_p5(r3, -32 * 1); // mova m5, [r3-32*1]
            m[6] = ld_p5(r3, 32 * 1); // mova m6, [r3+32*1]
            m[7] = ld_p5(r3, 32 * 3); // mova m7, [r3+32*3]
        }
        // .fast3:
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        let mut tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
        inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        tmp2 = at(tmp2, 32 * 32); // add tmp2q, 32*32
        m[0] = ld_p5(r7, -32 * 4); // mova m0, [r7-32*4]
        m[3] = ld_p5(r7, 32 * 3); // mova m3, [r7+32*3]
        m[4] = ld_p5(r7, 32 * 0); // mova m4, [r7+32*0]
        m[7] = ld_p5(r7, -32 * 1); // mova m7, [r7-32*1]
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        repx!([2, 5, 6], i => m[i] = m[1]); // REPX {mova x, m1}, m2, m5, m6
        if r10d as u8 == 0 {
            // test r10b, r10b / jnz .fast4
            m[1] = ld_p5(r8, 32 * 3); // mova m1, [r8+32*3]
            m[2] = ld_p5(r8, -32 * 4); // mova m2, [r8-32*4]
            m[5] = ld_p5(r8, -32 * 1); // mova m5, [r8-32*1]
            m[6] = ld_p5(r8, 32 * 0); // mova m6, [r8+32*0]
        }
        // .fast4:
        // add r6, o_idct64_offset
        inv_txfm_add_dct_dct_16x64_main_part1(m, 0, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
                                                                 // add r6, 8
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        tmp2 = at(tmp2, -32 * 8); // sub tmp2q, 32*8
        m[0] = ld_p5(r7, -32 * 2); // mova m0, [r7-32*2]
        m[3] = ld_p5(r7, 32 * 1); // mova m3, [r7+32*1]
        m[4] = ld_p5(r7, 32 * 2); // mova m4, [r7+32*2]
        m[7] = ld_p5(r7, -32 * 3); // mova m7, [r7-32*3]
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        repx!([2, 5, 6], i => m[i] = m[1]); // REPX {mova x, m1}, m2, m5, m6
        if r10d as u8 == 0 {
            // test r10b, r10b / jnz .fast5
            m[1] = ld_p5(r8, 32 * 1); // mova m1, [r8+32*1]
            m[2] = ld_p5(r8, -32 * 2); // mova m2, [r8-32*2]
            m[5] = ld_p5(r8, -32 * 3); // mova m5, [r8-32*3]
            m[6] = ld_p5(r8, 32 * 2); // mova m6, [r8+32*2]
        }
        // .fast5:
        inv_txfm_add_dct_dct_16x64_main_part1(m, 2, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        inv_txfm_add_dct_dct_16x64_main_part2_pass2(m, stride, &mut dst, &mut tmp1, &mut tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_pass2
        let (r, carry) = r10d.overflowing_add(0x8000_0000); // add r10d, 0x80000000
        r10d = r;
        if carry {
            break; // jc .ret
        }
        r2 = at(rsp, 32 * 7); // lea r2, [rsp+32*7]
        r7 = at(r2, 32 * 16); // lea r7, [r2+32*16]
        dst = at(dst, -(stride * 8)); // sub dstq, r8 (r8 is strideq*8 after .main_part2_pass2)
        dst = at(dst, stride * 4 + 16); // lea dstq, [dstq+strideq*4+16]
                                        // jmp .pass2_loop
    }
    // .ret:
}

inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_32x64_8bpc_avx2, inv_txfm_add_dct_dct_32x64);

// 64x32

/// `pmulhrsw m0..m6, m7, [cq+stride*row]` and `pmulhrsw m7, [cq+stride*row]` for the eight rows (`m7` holds
/// `pw_2896x8`), then `pxor m8, m8` and `REPX {mova [cq+stride*x], m8}` over the same rows: the idct64 inputs of
/// `.main_part1` in the first passes of 64x32 and 64x64
#[inline(always)]
unsafe fn load_rect2_clear_8rows_p5(m: &mut M, c: Coef, stride: isize, rows: [isize; 8]) {
    repx!([0, 1, 2, 3, 4, 5, 6], i => m[i] = _mm256_mulhrs_epi16(m[7], _mm256_loadu_si256(at(c, stride * rows[i]).cast()))); // pmulhrsw mi, m7, [cq+64*row]
    m[7] = _mm256_mulhrs_epi16(m[7], _mm256_loadu_si256(at(c, stride * rows[7]).cast())); // pmulhrsw m7, [cq+64*row]
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    repx!([0, 1, 2, 3, 4, 5, 6, 7], i => _mm256_storeu_si256(at(c, stride * rows[i]).cast(), m[8]));
    // REPX {mova [cq+64*x], m8}, rows
}

/// `.transpose_round_interleave` of `inv_txfm_add_dct_dct_64x32_8bpc`: rounds by `m10` and transposes four times
/// 16 rows of the frame at `tmp1q` in place (8x8 blocks, the two halves of each row interleaved); `tmp1q` ends
/// 64 slots on.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x32_transpose_round_interleave(m: &mut M, tmp1: &mut *mut __m256i) {
    let mut tmp3d = 4; // mov tmp3d, 4
    loop {
        // .loop:
        let t1 = *tmp1;
        let tmp2 = at(t1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
        m[0] = zx(ldx_p5(t1, -32 * 4)); // mova xm0, [tmp1q-32*4]
        m[1] = zx(ldx_p5(t1, -32 * 3)); // mova xm1, [tmp1q-32*3]
        m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_p5(tmp2, -32 * 4)); // vinserti128 m0, [tmp2q-32*4], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_p5(tmp2, -32 * 3)); // vinserti128 m1, [tmp2q-32*3], 1
        m[2] = zx(ldx_p5(t1, -32 * 2)); // mova xm2, [tmp1q-32*2]
        m[3] = zx(ldx_p5(t1, -32 * 1)); // mova xm3, [tmp1q-32*1]
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(tmp2, -32 * 2)); // vinserti128 m2, [tmp2q-32*2], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(tmp2, -32 * 1)); // vinserti128 m3, [tmp2q-32*1], 1
        m[4] = zx(ldx_p5(t1, 32 * 0)); // mova xm4, [tmp1q+32*0]
        m[5] = zx(ldx_p5(t1, 32 * 1)); // mova xm5, [tmp1q+32*1]
        m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(tmp2, 32 * 0)); // vinserti128 m4, [tmp2q+32*0], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(tmp2, 32 * 1)); // vinserti128 m5, [tmp2q+32*1], 1
        m[6] = zx(ldx_p5(t1, 32 * 2)); // mova xm6, [tmp1q+32*2]
        m[7] = zx(ldx_p5(t1, 32 * 3)); // mova xm7, [tmp1q+32*3]
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(tmp2, 32 * 2)); // vinserti128 m6, [tmp2q+32*2], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(tmp2, 32 * 3)); // vinserti128 m7, [tmp2q+32*3], 1
        repx!([0, 1, 2, 3, 4, 5, 6, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[10])); // REPX {pmulhrsw x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        m[8] = zx(ldx_p5(t1, -32 * 4 + 16)); // mova xm8, [tmp1q-32*4+16]
        m[9] = zx(ldx_p5(t1, -32 * 3 + 16)); // mova xm9, [tmp1q-32*3+16]
        m[8] = _mm256_inserti128_si256::<1>(m[8], ldx_p5(tmp2, -32 * 4 + 16)); // vinserti128 m8, [tmp2q-32*4+16], 1
        m[9] = _mm256_inserti128_si256::<1>(m[9], ldx_p5(tmp2, -32 * 3 + 16)); // vinserti128 m9, [tmp2q-32*3+16], 1
        st_p5(t1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        st_p5(tmp2, -32 * 4, m[1]); // mova [tmp2q-32*4], m1
        st_p5(t1, -32 * 3, m[2]); // mova [tmp1q-32*3], m2
        st_p5(tmp2, -32 * 3, m[3]); // mova [tmp2q-32*3], m3
        m[2] = zx(ldx_p5(t1, -32 * 2 + 16)); // mova xm2, [tmp1q-32*2+16]
        m[3] = zx(ldx_p5(t1, -32 * 1 + 16)); // mova xm3, [tmp1q-32*1+16]
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_p5(tmp2, -32 * 2 + 16)); // vinserti128 m2, [tmp2q-32*2+16], 1
        m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_p5(tmp2, -32 * 1 + 16)); // vinserti128 m3, [tmp2q-32*1+16], 1
        st_p5(t1, -32 * 2, m[4]); // mova [tmp1q-32*2], m4
        st_p5(tmp2, -32 * 2, m[5]); // mova [tmp2q-32*2], m5
        st_p5(t1, -32 * 1, m[6]); // mova [tmp1q-32*1], m6
        st_p5(tmp2, -32 * 1, m[7]); // mova [tmp2q-32*1], m7
        m[4] = zx(ldx_p5(t1, 32 * 0 + 16)); // mova xm4, [tmp1q+32*0+16]
        m[5] = zx(ldx_p5(t1, 32 * 1 + 16)); // mova xm5, [tmp1q+32*1+16]
        m[4] = _mm256_inserti128_si256::<1>(m[4], ldx_p5(tmp2, 32 * 0 + 16)); // vinserti128 m4, [tmp2q+32*0+16], 1
        m[5] = _mm256_inserti128_si256::<1>(m[5], ldx_p5(tmp2, 32 * 1 + 16)); // vinserti128 m5, [tmp2q+32*1+16], 1
        m[6] = zx(ldx_p5(t1, 32 * 2 + 16)); // mova xm6, [tmp1q+32*2+16]
        m[7] = zx(ldx_p5(t1, 32 * 3 + 16)); // mova xm7, [tmp1q+32*3+16]
        m[6] = _mm256_inserti128_si256::<1>(m[6], ldx_p5(tmp2, 32 * 2 + 16)); // vinserti128 m6, [tmp2q+32*2+16], 1
        m[7] = _mm256_inserti128_si256::<1>(m[7], ldx_p5(tmp2, 32 * 3 + 16)); // vinserti128 m7, [tmp2q+32*3+16], 1
        m[0] = _mm256_mulhrs_epi16(m[8], m[10]); // pmulhrsw m0, m8, m10
        m[1] = _mm256_mulhrs_epi16(m[9], m[10]); // pmulhrsw m1, m9, m10
        repx!([2, 3, 4, 5, 6, 7], i => m[i] = _mm256_mulhrs_epi16(m[i], m[10])); // REPX {pmulhrsw x, m10}, m2, m3, m4, m5, m6, m7
        inv_txfm_add_identity_identity_8x32_transpose8x8(m); // call m(inv_txfm_add_identity_identity_8x32_8bpc).transpose8x8
        st_p5(t1, 32 * 0, m[0]); // mova [tmp1q+32*0], m0
        st_p5(tmp2, 32 * 0, m[1]); // mova [tmp2q+32*0], m1
        st_p5(t1, 32 * 1, m[2]); // mova [tmp1q+32*1], m2
        st_p5(tmp2, 32 * 1, m[3]); // mova [tmp2q+32*1], m3
        st_p5(t1, 32 * 2, m[4]); // mova [tmp1q+32*2], m4
        st_p5(tmp2, 32 * 2, m[5]); // mova [tmp2q+32*2], m5
        st_p5(t1, 32 * 3, m[6]); // mova [tmp1q+32*3], m6
        st_p5(tmp2, 32 * 3, m[7]); // mova [tmp2q+32*3], m7
        *tmp1 = at(t1, 32 * 16); // add tmp1q, 32*16
        tmp3d -= 1; // dec tmp3d
        if tmp3d <= 0 {
            break; // jg .loop
        }
    }
}

/// `cglobal inv_txfm_add_dct_dct_64x32_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x32(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(16384); // movd xm2, [o(pw_16384)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[1]))); // pmulhrsw xm0, xm1
        let r3d = eob | 32; // or r3d, 32
        return inv_txfm_add_dct_dct_64x16_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_64x16_8bpc).dconly
    }
    // .normal: PROLOGUE 0, 9, 16, 32*131, dst, stride, c, eob, tmp1, tmp2, base, tmp3, tmp4. The frame is zeroed
    // whole (see the top of this part).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 131]>::uninit();
    let rsp: *mut __m256i = zeroed_frame(&mut frame);
    let mut tmp1 = at(rsp, 32 * 7); // lea tmp1q, [rsp+32*7]
    let mut tmp4d = eob.wrapping_sub(136) as u32; // lea tmp4d, [eobq-136]
    loop {
        // .pass1_loop:
        load_8rows(m, at(c, 64 * 0), 64 * 4, 1); // LOAD_8ROWS cq+64*0, 64*4, 1
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([0, 4, 8, 12, 16, 20, 24, 28], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[8])); // REPX {mova [cq+64*x], m8}, 0, 4, 8, 12, 16, 20, 24, 28
        repx!([9, 10, 11, 12, 13, 14], i => m[i] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
        st_p5(rsp, 0, m[8]); // mova [rsp], m8
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
        st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        st_p5(tmp1, -32 * 3, m[1]); // mova [tmp1q-32*3], m1
        st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
        st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
        st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
        st_p5(tmp1, 32 * 1, m[5]); // mova [tmp1q+32*1], m5
        st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
        st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        st_p5(tmp1, -32 * 4, m[8]); // mova [tmp1q-32*4], m8
        st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
        st_p5(tmp1, -32 * 2, m[10]); // mova [tmp1q-32*2], m10
        st_p5(tmp1, -32 * 1, m[11]); // mova [tmp1q-32*1], m11
        st_p5(tmp1, 32 * 0, m[12]); // mova [tmp1q+32*0], m12
        st_p5(tmp1, 32 * 1, m[13]); // mova [tmp1q+32*1], m13
        st_p5(tmp1, 32 * 2, m[14]); // mova [tmp1q+32*2], m14
        st_p5(tmp1, 32 * 3, m[15]); // mova [tmp1q+32*3], m15
        load_8rows(m, at(c, 64 * 2), 64 * 4, 1); // LOAD_8ROWS cq+64*2, 64*4, 1
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([2, 6, 10, 14, 18, 22, 26, 30], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[8])); // REPX {mova [cq+64*x], m8}, 2, 6, 10, 14, 18, 22, 26, 30
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        let mut tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
        inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        tmp2 = at(tmp2, 32 * 32); // add tmp2q, 32*32
        m[7] = pw(2896 * 8); // vpbroadcastd m7, [o(pw_2896x8)]
        load_rect2_clear_8rows_p5(m, c, 64, [1, 31, 17, 15, 9, 23, 25, 7]); // pmulhrsw m0..m7, m7, [cq+64*1, 31, 17, 15, 9, 23, 25, 7] / pxor m8, m8 / REPX {mova [cq+64*x], m8}, …
                                                                            // add r6, o_idct64_offset
        inv_txfm_add_dct_dct_16x64_main_part1(m, 0, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        m[7] = pw(2896 * 8); // vpbroadcastd m7, [o(pw_2896x8-(o_idct64_offset))]
                             // add r6, 8
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        tmp2 = at(tmp2, -32 * 8); // sub tmp2q, 32*8
        load_rect2_clear_8rows_p5(m, c, 64, [5, 27, 21, 11, 13, 19, 29, 3]); // pmulhrsw m0..m7, m7, [cq+64*5, 27, 21, 11, 13, 19, 29, 3] / pxor m8, m8 / REPX {mova [cq+64*x], m8}, …
        inv_txfm_add_dct_dct_16x64_main_part1(m, 2, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        inv_txfm_add_dct_dct_16x64_main_part2_pass1(m, &mut tmp1, &mut tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_pass1
        tmp1 = at(tmp1, -32 * 44); // sub tmp1q, 32*44
        m[10] = pw(16384); // vpbroadcastd m10, [o(pw_16384)]
        inv_txfm_add_dct_dct_64x32_transpose_round_interleave(m, &mut tmp1); // call m(inv_txfm_add_dct_dct_64x32_8bpc).transpose_round_interleave
        c = at(c, 32); // add cq, 32
        let (t, carry) = tmp4d.overflowing_add(0x8000_0000); // add tmp4d, 0x80000000
        tmp4d = t;
        if carry {
            break; // jnc .pass1_loop
        }
    }
    // After the loop, bit 30 of tmp4d is set exactly when it ran once (eob < 136); the pass-2 loop counts in the
    // low byte (`tmp4b`) and tests that bit.
    tmp1 = at(rsp, 32 * 15); // lea tmp1q, [rsp+32*15]
    let mut r2 = at(dst, stride * 19); // imul r2, strideq, 19 / add r2, dstq
    let r3 = stride * 3; // lea r3, [strideq*3]
    tmp4d = (tmp4d & !0xff) | 4; // mov tmp4b, 4
    loop {
        // .pass2_loop:
        let tmp2 = at(tmp1, 32 * 64); // lea tmp2q, [tmp1q+32*64]
        load_8rows(m, at(tmp1, -32 * 4), 32, 0); // LOAD_8ROWS tmp1q-32*4, 32
        if tmp4d & 0x4000_0000 == 0 {
            // test tmp4d, 0x40000000 / jnz .fast
            load_8rows_h(m, at(tmp2, -32 * 4), 32, 0); // LOAD_8ROWS_H tmp2q-32*4, 32
            inv_txfm_add_dct_dct_16x32_main_oddhalf(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
            let tmp3 = at(tmp2, -32 * 8); // lea tmp3q, [tmp2q-32*8]
            load_8rows_h(m, at(tmp3, -32 * 4), 32, 0); // LOAD_8ROWS_H tmp3q-32*4, 32
            st_p5(rsp, 0, m[15]); // mova [rsp], m15
                                  // jmp .idct16
        } else {
            // .fast:
            inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
            m[8] = _mm256_setzero_si256(); // pxor m8, m8
            repx!([9, 10, 11, 12, 13, 14], i => m[i] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
            st_p5(rsp, 0, m[8]); // mova [rsp], m8
        }
        // .idct16:
        let tmp3 = at(tmp1, -32 * 8); // lea tmp3q, [tmp1q-32*8]
        load_8rows(m, at(tmp3, -32 * 4), 32, 0); // LOAD_8ROWS tmp3q-32*4, 32
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_16x32_pass2_end(m, s3_p5(rsp), &mut dst, stride, &mut r2, r3, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).pass2_end
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        dst = at(dst, -r3); // sub dstq, r3
        r2 = at(r2, r3 + 16); // lea r2, [r2+r3+16]
        dst = at(dst, 16); // add dstq, 16
        let b = (tmp4d as u8).wrapping_sub(1); // dec tmp4b
        tmp4d = (tmp4d & !0xff) | b as u32;
        if b as i8 <= 0 {
            break; // jg .pass2_loop
        }
    }
}

inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_64x32_8bpc_avx2, inv_txfm_add_dct_dct_64x32);

// 64x64

/// `cglobal inv_txfm_add_dct_dct_64x64_8bpc, 4, 4, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x64(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    // lea r6, [o_base]
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        m[1] = movd_pw(2896 * 8); // movd xm1, [o(pw_2896x8)]
        m[0] = zx(_mm_mulhrs_epi16(lo(m[1]), _mm_loadu_si128(c.cast()))); // pmulhrsw xm0, xm1, [cq]
        m[2] = movd_pw(8192); // movd xm2, [o(pw_8192)]
        c.cast::<i32>().write_unaligned(eob); // mov [cq], eobd
        let r3d = eob | 64; // or r3d, 64
        return inv_txfm_add_dct_dct_64x16_dconly(m, dst, stride, r3d); // jmp m(inv_txfm_add_dct_dct_64x16_8bpc).dconly
    }
    // .normal: PROLOGUE 0, 11, 16, 32*199, dst, stride, c, eob, tmp1, tmp2. The frame is zeroed whole (see the top
    // of this part).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 199]>::uninit();
    let rsp = zeroed_frame(&mut frame);
    let mut tmp1 = at(rsp, 32 * 71); // lea tmp1q, [rsp+32*71]
    let mut r10d = eob.wrapping_sub(136) as u32; // lea r10d, [eobq-136]
    loop {
        // .pass1_loop:
        load_8rows(m, at(c, 64 * 0), 64 * 4, 0); // LOAD_8ROWS cq+64*0, 64*4
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([0, 4, 8, 12, 16, 20, 24, 28], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[8])); // REPX {mova [cq+64*x], m8}, 0, 4, 8, 12, 16, 20, 24, 28
        repx!([9, 10, 11, 12, 13, 14], i => m[i] = m[8]); // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
        st_p5(rsp, 0, m[8]); // mova [rsp], m8
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
        st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        st_p5(tmp1, -32 * 3, m[1]); // mova [tmp1q-32*3], m1
        st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
        st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
        st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
        st_p5(tmp1, 32 * 1, m[5]); // mova [tmp1q+32*1], m5
        st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
        st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        st_p5(tmp1, -32 * 4, m[8]); // mova [tmp1q-32*4], m8
        st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
        st_p5(tmp1, -32 * 2, m[10]); // mova [tmp1q-32*2], m10
        st_p5(tmp1, -32 * 1, m[11]); // mova [tmp1q-32*1], m11
        st_p5(tmp1, 32 * 0, m[12]); // mova [tmp1q+32*0], m12
        st_p5(tmp1, 32 * 1, m[13]); // mova [tmp1q+32*1], m13
        st_p5(tmp1, 32 * 2, m[14]); // mova [tmp1q+32*2], m14
        st_p5(tmp1, 32 * 3, m[15]); // mova [tmp1q+32*3], m15
        load_8rows(m, at(c, 64 * 2), 64 * 4, 0); // LOAD_8ROWS cq+64*2, 64*4
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        repx!([2, 6, 10, 14, 18, 22, 26, 30], i => _mm256_storeu_si256(at(c, 64 * i).cast(), m[8])); // REPX {mova [cq+64*x], m8}, 2, 6, 10, 14, 18, 22, 26, 30
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        let mut tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
        inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        tmp2 = at(tmp2, 32 * 32); // add tmp2q, 32*32
        load_clear_8rows_p5(m, c, 64, [1, 31, 17, 15, 9, 23, 25, 7]); // mova m0..m7, [cq+64*1, 31, 17, 15, 9, 23, 25, 7] / pxor m8, m8 / REPX {mova [cq+64*x], m8}, …
                                                                      // add r6, o_idct64_offset
        inv_txfm_add_dct_dct_16x64_main_part1(m, 0, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
                                                                 // add r6, 8
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        tmp2 = at(tmp2, -32 * 8); // sub tmp2q, 32*8
        load_clear_8rows_p5(m, c, 64, [5, 27, 21, 11, 13, 19, 29, 3]); // mova m0..m7, [cq+64*5, 27, 21, 11, 13, 19, 29, 3] / pxor m8, m8 / REPX {mova [cq+64*x], m8}, …
        inv_txfm_add_dct_dct_16x64_main_part1(m, 2, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        inv_txfm_add_dct_dct_16x64_main_part2_pass1(m, &mut tmp1, &mut tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_pass1
        tmp1 = at(tmp1, -32 * 44); // sub tmp1q, 32*44
        m[10] = pw(8192); // vpbroadcastd m10, [o(pw_8192)]
        inv_txfm_add_dct_dct_64x32_transpose_round_interleave(m, &mut tmp1); // call m(inv_txfm_add_dct_dct_64x32_8bpc).transpose_round_interleave
        c = at(c, 32); // add cq, 32
        let (r, carry) = r10d.overflowing_add(0x8000_0000); // add r10d, 0x80000000
        r10d = r;
        if carry {
            break; // jnc .pass1_loop
        }
    }
    // After the loop, bit 30 of r10d is set exactly when it ran once (eob < 136); the pass-2 loop counts in the
    // low byte (`r10b`) and tests that bit.
    tmp1 = at(rsp, 32 * 7); // lea tmp1q, [rsp+32*7]
    r10d = (r10d & !0xff) | 4; // mov r10b, 4
    loop {
        // .pass2_loop:
        let mut r2 = at(tmp1, 32 * 64); // lea r2, [tmp1q+32*64]
        m[0] = ld_p5(r2, -32 * 4); // mova m0, [r2-32*4]
        m[1] = ld_p5(r2, -32 * 2); // mova m1, [r2-32*2]
        m[2] = ld_p5(r2, 32 * 0); // mova m2, [r2+32*0]
        m[3] = ld_p5(r2, 32 * 2); // mova m3, [r2+32*2]
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        repx!([5, 6, 7, 8, 9, 10, 11, 12, 13, 14], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, …, m14
        st_p5(rsp, 0, m[4]); // mova [rsp], m4
                             // r3 is set only on the full path (`lea r3, [r2+32*64]`); the fast path never loads through it.
        let mut r3 = r2;
        if r10d & 0x4000_0000 == 0 {
            // test r10d, 0x40000000 / jnz .fast
            r3 = at(r2, 32 * 64); // lea r3, [r2+32*64]
            m[4] = ld_p5(r3, -32 * 4); // mova m4, [r3-32*4]
            m[5] = ld_p5(r3, -32 * 2); // mova m5, [r3-32*2]
            m[6] = ld_p5(r3, 32 * 0); // mova m6, [r3+32*0]
            m[7] = ld_p5(r3, 32 * 2); // mova m7, [r3+32*2]
        }
        // .fast:
        idct_16x16_internal_main(m, s3_p5(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p5(rsp, 32 * 1); // mova m1, [rsp+32*1]
        st_p5(tmp1, -32 * 4, m[0]); // mova [tmp1q-32*4], m0
        st_p5(tmp1, -32 * 3, m[1]); // mova [tmp1q-32*3], m1
        st_p5(tmp1, -32 * 2, m[2]); // mova [tmp1q-32*2], m2
        st_p5(tmp1, -32 * 1, m[3]); // mova [tmp1q-32*1], m3
        st_p5(tmp1, 32 * 0, m[4]); // mova [tmp1q+32*0], m4
        st_p5(tmp1, 32 * 1, m[5]); // mova [tmp1q+32*1], m5
        st_p5(tmp1, 32 * 2, m[6]); // mova [tmp1q+32*2], m6
        st_p5(tmp1, 32 * 3, m[7]); // mova [tmp1q+32*3], m7
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        st_p5(tmp1, -32 * 4, m[8]); // mova [tmp1q-32*4], m8
        st_p5(tmp1, -32 * 3, m[9]); // mova [tmp1q-32*3], m9
        st_p5(tmp1, -32 * 2, m[10]); // mova [tmp1q-32*2], m10
        st_p5(tmp1, -32 * 1, m[11]); // mova [tmp1q-32*1], m11
        st_p5(tmp1, 32 * 0, m[12]); // mova [tmp1q+32*0], m12
        st_p5(tmp1, 32 * 1, m[13]); // mova [tmp1q+32*1], m13
        st_p5(tmp1, 32 * 2, m[14]); // mova [tmp1q+32*2], m14
        st_p5(tmp1, 32 * 3, m[15]); // mova [tmp1q+32*3], m15
        m[0] = ld_p5(r2, -32 * 3); // mova m0, [r2-32*3]
        m[1] = ld_p5(r2, -32 * 1); // mova m1, [r2-32*1]
        m[2] = ld_p5(r2, 32 * 1); // mova m2, [r2+32*1]
        m[3] = ld_p5(r2, 32 * 3); // mova m3, [r2+32*3]
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        repx!([5, 6, 7], i => m[i] = m[4]); // REPX {mova x, m4}, m5, m6, m7
        if r10d & 0x4000_0000 == 0 {
            // test r10d, 0x40000000 / jnz .fast2
            m[4] = ld_p5(r3, -32 * 3); // mova m4, [r3-32*3]
            m[5] = ld_p5(r3, -32 * 1); // mova m5, [r3-32*1]
            m[6] = ld_p5(r3, 32 * 1); // mova m6, [r3+32*1]
            m[7] = ld_p5(r3, 32 * 3); // mova m7, [r3+32*3]
        }
        // .fast2:
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        let mut tmp2 = at(tmp1, 32 * 8); // lea tmp2q, [tmp1q+32*8]
        inv_txfm_add_dct_dct_16x32_main_oddhalf_fast(m, s3_p5(rsp), tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[15] = pd(2048); // vpbroadcastd m15, [o(pd_2048)]
        r2 = at(r2, 32 * 8); // add r2, 32*8
        r3 = at(r3, 32 * 8); // add r3, 32*8
        tmp1 = at(tmp1, 32 * 16); // add tmp1q, 32*16
        tmp2 = at(tmp2, 32 * 32); // add tmp2q, 32*32
        m[0] = ld_p5(r2, -32 * 4); // mova m0, [r2-32*4] ; 1
        m[3] = ld_p5(r2, 32 * 3); // mova m3, [r2+32*3] ; 15
        m[4] = ld_p5(r2, 32 * 0); // mova m4, [r2+32*0] ; 9
        m[7] = ld_p5(r2, -32 * 1); // mova m7, [r2-32*1] ; 7
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        repx!([2, 5, 6], i => m[i] = m[1]); // REPX {mova x, m1}, m2, m5, m6
        if r10d & 0x4000_0000 == 0 {
            // test r10d, 0x40000000 / jnz .fast3
            m[1] = ld_p5(r3, 32 * 3); // mova m1, [r3+32*3] ; 31
            m[2] = ld_p5(r3, -32 * 4); // mova m2, [r3-32*4] ; 17
            m[5] = ld_p5(r3, -32 * 1); // mova m5, [r3-32*1] ; 23
            m[6] = ld_p5(r3, 32 * 0); // mova m6, [r3+32*0] ; 25
        }
        // .fast3:
        // add r6, o_idct64_offset
        inv_txfm_add_dct_dct_16x64_main_part1(m, 0, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
                                                                 // add r6, 8
        tmp1 = at(tmp1, 32 * 8); // add tmp1q, 32*8
        tmp2 = at(tmp2, -32 * 8); // sub tmp2q, 32*8
        m[0] = ld_p5(r2, -32 * 2); // mova m0, [r2-32*2] ; 5
        m[3] = ld_p5(r2, 32 * 1); // mova m3, [r2+32*1] ; 11
        m[4] = ld_p5(r2, 32 * 2); // mova m4, [r2+32*2] ; 13
        m[7] = ld_p5(r2, -32 * 3); // mova m7, [r2-32*3] ; 3
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        repx!([2, 5, 6], i => m[i] = m[1]); // REPX {mova x, m1}, m2, m5, m6
        if r10d & 0x4000_0000 == 0 {
            // test r10d, 0x40000000 / jnz .fast4
            m[1] = ld_p5(r3, 32 * 1); // mova m1, [r3+32*1] ; 27
            m[2] = ld_p5(r3, -32 * 2); // mova m2, [r3-32*2] ; 21
            m[5] = ld_p5(r3, -32 * 3); // mova m5, [r3-32*3] ; 19
            m[6] = ld_p5(r3, 32 * 2); // mova m6, [r3+32*2] ; 29
        }
        // .fast4:
        inv_txfm_add_dct_dct_16x64_main_part1(m, 2, tmp1, tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        inv_txfm_add_dct_dct_16x64_main_part2_pass2(m, stride, &mut dst, &mut tmp1, &mut tmp2); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_pass2
        tmp1 = at(tmp1, -32 * 28); // sub tmp1q, 32*28
        dst = at(dst, -(stride * 8)); // sub dstq, r8 (r8 is strideq*8 after .main_part2_pass2)
        dst = at(dst, stride * 4 + 16); // lea dstq, [dstq+strideq*4+16]
        let b = (r10d as u8).wrapping_sub(1); // dec r10b
        r10d = (r10d & !0xff) | b as u32;
        if b as i8 <= 0 {
            break; // jg .pass2_loop
        }
    }
}

inv_txfm_fn!(dav1d_inv_txfm_add_dct_dct_64x64_8bpc_avx2, inv_txfm_add_dct_dct_64x64);
