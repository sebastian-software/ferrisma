// Copyright © 2021, VideoLAN and dav1d authors
// Copyright © 2021, Two Orioles, LLC
// Copyright © 2021, Matthias Dressel
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/itx16_avx2.asm` (10 and 12 bits) with
//! AVX2 intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The 303 entry points of the 16 bpc inverse transforms: 155
//! `dav1d_inv_txfm_add_<type1>_<type2>_<w>x<h>_10bpc_avx2` (every size from
//! 4x4 to 64x64 with the transform-type pairs the original has), 147
//! `…_12bpc_avx2` (the same without the 32-point dct_dct and the 64-point
//! sizes) and `dav1d_inv_txfm_add_wht_wht_4x4_16bpc_avx2`.
//! `simd/x86/itx.rs` assigns them under `DAV1D_X86_CPU_FLAG_AVX2`. The C
//! signature has a fifth argument, `bitdepth_max`; only WHT reads it.
//!
//! Correspondence with the assembly: as in `itx_avx2.rs` (its module
//! documentation), with these differences.
//! - The bit depth stays in every name: a label `.X` of the assembly
//!   function `F` (`idct_8x8_internal_10bpc`, `inv_txfm_add_dct_dct_8x32_12bpc`)
//!   is `F_X`; the two halves of an `_internal` function are `F_pass1` and
//!   `F_pass2`; per size and bit depth `inv_txfm_add_<size>_<bpc>bpc(dst,
//!   stride, c, eob, t1, t2)` does what `INV_TXFM_<SIZE>_FN` does. Where an
//!   `_internal` function loads its clipping constants before a `.pass1:`
//!   label that other functions jump to, those loads are in the size function
//!   and `F_pass1` is the label's code; the 12 bpc first passes that only
//!   jump to a 10 bpc `.pass1` call it directly.
//! - The 8 bpc labels that this file `call`s (`cextern …_8bpc_avx2.main` and
//!   the like) are the items of `itx_avx2.rs`, imported under names that keep
//!   their bit depth: `m(idct_8x8_internal_8bpc).main` is
//!   `idct_8x8_internal_8bpc_main`.
//! - `type Dst = *mut u16`, `type Coef = *mut i32`; offsets are bytes, as in
//!   the assembly (`at`).
//! - `ITX_MULSUB_2D` is `itx_mulsub_2d`, whose coefficients are an `i32` (a
//!   register number in 0..32, a `pd_N` constant otherwise) or `Q(A, B)`, the
//!   packed pair `pd_A_B` (`vbroadcasti128`, `pd_q`); `rnd` is `None` where
//!   the assembly passes `_`. The other shared macros: `idct4_1d_packed`,
//!   `idct4_1d_packed_word` and `iadst4_1d` (with `_x`, the `WRAP_XMM`
//!   forms), `idct4_1d`, `transpose_8x8_dword`, `iadst8_1d`, `idtx16`
//!   (`Idtx16Shift` for its optional fourth argument), `idtx16_12bpc`,
//!   `idct32_end::<SHIFT>`, `idct32_pass2_end`, `idct64_part2_end`,
//!   `idct64_shift_transpose::<SHIFT>`.
//! - The DC-only tails (`.dconly*` of 4x4, 4x8, 8x4, 8x8, 16x4, 32x8, 64x16)
//!   are functions; the 12 bpc paths load `dconly_12bpc` and end in the 10
//!   bpc tail, as the assembly jumps.
//! - Shared subroutines (the assembly's cross `call`s and `jmp`s): 8x4's
//!   `.main`s serve 4x8; 4x8's `.main`/`.main2`/`.main3` serve 8x4; 4x16's
//!   `.pass1_main*` and `.main` and 8x4's 12 bpc `.transpose_4x8` serve 16x4;
//!   16x4's `.pass1_main*` and `.main`/`.main_end` serve 4x16; 8x8's `.main`,
//!   `.main_rect2`, `.write_8x4*`, the 12 bpc `.round_shift4`/`.transpose_8x8`
//!   and 8x16's `.main_evenhalf` and `.main_oddhalf` family serve 16x8, 16x16,
//!   the 32- and the 64-point sizes; 16x8's `.main_part1`/`.main_part2`,
//!   `.pass1_rotations`, `.transpose*` and `.write_16x4*` serve 8x16, 16x16,
//!   the 32- and 64-point sizes; 8x32's `.main_oddhalf_part1`/`_part2` (with
//!   `_fast`/`_rect2`), `.main_end`, `.transpose`, `IDCT32_END` and 8x32
//!   identity's `.main_zero` serve 32x16, 32x32 and the 64-point sizes;
//!   16x32's `.pass2_end` and 32x16's `.main_end`/`.write_16x16` serve the
//!   64-point sizes; 16x64's `.main_part1`/`.main_part2` serve the other
//!   64-point sizes. All are `#[inline(always)]`; no entry point contains a
//!   `call`.
//!
//! Memory: `dst` points to the block's `h` rows of `w` pixels (`uint16_t`),
//! `stride` bytes apart; `c` to the block's `min(w, 32) * min(h, 32)`
//! coefficients (`int32_t`), which each kernel leaves zero where the
//! assembly zeroes them. Every access to `dst` and `c` touches exactly the
//! bytes the assembly touches: rows of 8 (`movq`/`movhps`), 16 or 32 bytes
//! inside the block, coefficient loads, stores and clears inside the buffer
//! (several sizes use it as scratch, as the assembly does), and the DC-only
//! paths' 4-byte read and write at `c`. None reaches beyond the block or the
//! coefficients.
//!
//! Deviations from the assembly:
//! - Caller memory is accessed with unaligned moves where the assembly uses
//!   `mova` (the same bytes and instruction count); `movhps` with
//!   `movhps_ld`/`movhps_st`.
//! - The frames of 16x16, the 32- and 64-point sizes (`cglobal …, 32*24` …
//!   `32*195`) and the 12 bpc 8x32 and 32x8 are zero-initialized arrays,
//!   where the assembly leaves them uninitialized: which slots a later step
//!   reads depends on the eob path. The compiler writes the zeros as it
//!   chooses (stores, or a call of `memset`). The small frames whose every read follows a
//!   write in the same pass (16x8's and the 12 bpc 8x16's `32*8`, 8x32 10
//!   bpc's `32*12`) are zero-initialized arrays or written whole before any
//!   read, as said at each site. The spill slots `[rsp+gprsize+32*0..2]` that
//!   subroutines share with their caller are a `[__m256i; 3]` (16x32's
//!   `.pass2_end` takes the fourth slot as `s3`).
//! - Flag arithmetic on general-purpose registers is Rust on the same widths
//!   (`add eobb, 21 / cmovc` as `overflowing_add` and the carry, `sub eobd,
//!   N / jl` as a signed compare), and jumps between labels are calls of the
//!   label's function, loops and branches in the assembly's order;
//!   subroutines that advance `dstq`, `cq` or `r6` take them as `&mut`.
//! - `lea r6, [deint_shuf+128]`, `lea r6, [pw_5+128]` (the 8 bpc tables'
//!   base) have no code; 16x64's `lea r6, [idct64_mul-8]` / `add r6, 8` is
//!   the 8 bpc `.main_part1`'s offset argument (0, 2), and the 64-point
//!   `.main`s' `idct64_mul_16bpc` pointer an index into that table.
//! - The compiler rewrites some shuffles: `vbroadcasti128` twice with
//!   `shufpd 0x0c` (16x4's DCT and 4x16's ADST first passes) becomes
//!   `vbroadcasti128`, `vpermq` twice and `vpblendd`; in 4x4 identity
//!   `vpermd`/`psrld`/`pshufb` become `pshufb`/`vpermq`/`pshufb`/`por`.
//!
//! # Safety
//! Each entry point is called by the decoder through the DSP table with the
//! contract above, on a CPU with AVX2 (the init assigns it only under
//! `DAV1D_X86_CPU_FLAG_AVX2`). Raw pointers reach `dst`, `c` and the frames;
//! no reference to caller memory is formed.
#![allow(
    clippy::all,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    dead_code,
    unused_variables,
    unused_macros,
    unused_imports
)]

use core::arch::x86_64::*;
use core::ffi::c_int;

// --- what every part shares (itx16_avx2.asm lines 1-350 and the shared macros) ---------------------------------

// The 8 bpc module's register file, helpers and the labels of `itx_avx2.asm` that this file `call`s
// (`cextern idct_8x8_internal_8bpc_avx2.main` and the like). An 8 bpc label keeps its bit depth here:
// `m(idct_8x8_internal_8bpc).main` is `idct_8x8_internal_8bpc_main`.
use super::itx_avx2::{
    at, deint_shuf, iadst_16x16_internal_main as iadst_16x16_internal_8bpc_main,
    iadst_16x16_internal_main_pass2_end as iadst_16x16_internal_8bpc_main_pass2_end,
    iadst_16x4_internal_main as iadst_16x4_internal_8bpc_main,
    iadst_16x8_internal_main as iadst_16x8_internal_8bpc_main,
    iadst_16x8_internal_main_pass2_end as iadst_16x8_internal_8bpc_main_pass2_end,
    iadst_4x16_internal_main2 as iadst_4x16_internal_8bpc_main2,
    iadst_4x4_internal_main as iadst_4x4_internal_8bpc_main,
    iadst_4x8_internal_main_pass2 as iadst_4x8_internal_8bpc_main_pass2,
    iadst_8x16_internal_main as iadst_8x16_internal_8bpc_main,
    iadst_8x16_internal_main_pass2_end as iadst_8x16_internal_8bpc_main_pass2_end,
    iadst_8x4_internal_main as iadst_8x4_internal_8bpc_main,
    iadst_8x8_internal_main_pass2 as iadst_8x8_internal_8bpc_main_pass2, idct64_mul,
    idct_16x16_internal_main as idct_16x16_internal_8bpc_main, idct_16x4_internal_main as idct_16x4_internal_8bpc_main,
    idct_16x8_internal_main as idct_16x8_internal_8bpc_main, idct_4x16_internal_main as idct_4x16_internal_8bpc_main,
    idct_4x8_internal_main as idct_4x8_internal_8bpc_main, idct_8x16_internal_main as idct_8x16_internal_8bpc_main,
    idct_8x8_internal_main as idct_8x8_internal_8bpc_main,
    inv_txfm_add_dct_dct_16x32_main_oddhalf as inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf,
    inv_txfm_add_dct_dct_16x32_main_oddhalf_fast as inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf_fast,
    inv_txfm_add_dct_dct_16x64_main_part1 as inv_txfm_add_dct_dct_16x64_8bpc_main_part1,
    inv_txfm_add_dct_dct_16x64_main_part2_internal as inv_txfm_add_dct_dct_16x64_8bpc_main_part2_internal,
    inv_txfm_add_dct_dct_8x32_main as inv_txfm_add_dct_dct_8x32_8bpc_main,
    inv_txfm_add_dct_dct_8x32_main_fast as inv_txfm_add_dct_dct_8x32_8bpc_main_fast, lo, movd_pw, movhps_ld, movhps_st,
    pd, pw, pw2, regs, zx, K, M,
};

/// `pixel *dst` at 16 bpc. Offsets are bytes, as in the assembly (`at(dst, stride * 2)`).
type Dst = *mut u16;
/// `coef *c` at 16 bpc (`int32_t`). Offsets are bytes, as in the assembly (`at(c, 32 * 3)`).
type Coef = *mut i32;

// SECTION_RODATA. Constants loaded by `vpbroadcastd` are written where the assembly names them: `[pd_N]` is
// `pd(N)` (`pd_mN` is `pd(-N)`, `clip_18b_min` is `pd(-0x20000)`, `clip_18b_max` `pd(0x1ffff)`, `clip_20b_min`
// `pd(-0x80000)`, `clip_20b_max` `pd(0x7ffff)`), `[pixel_10bpc_max]` is `pw(0x3ff)`, `[pixel_12bpc_max]`
// `pw(0xfff)`, `[dconly_10bpc]` `pw(0x7c00)`, `[dconly_12bpc]` `pw(0x7000)`; the `cextern` word constants of the
// 8 bpc file as there (`[pw_1697x8]` is `pw(1697 * 8)`, `[pw_m3784_1567]` `pw2(-3784, 1567)`). An `xmm`
// destination takes `zx(lo(…))`. The compiler places the values in read-only data and broadcasts them as the
// assembly does. `vbroadcasti128 m, [pd_A_B]` (`COEF_PAIR`: `dd A, A, B, B`) is `pd_q(A, B)`; `pd_B_mB` (the
// third argument of `COEF_PAIR`) is `pd_q(B, -B)`. Tables loaded whole or indexed are statics with the
// assembly's name and layout.

/// `vbroadcasti128 m, [pd_A_B]`: `A, A, B, B` in each lane.
#[inline(always)]
unsafe fn pd_q(a: i32, b: i32) -> __m256i {
    _mm256_setr_epi32(a, a, b, b, a, a, b, b)
}

#[repr(C, align(32))]
struct A32<T>(T);

static itx4_shuf: A32<[u32; 8]> = A32([
    0x50401600, 0xd0c09284, 0x70603422, 0xf0e0b0a6, 0x50401701, 0xd0c09385, 0x70603523, 0xf0e0b1a7,
]);
static idct4_12_shuf: A32<[i32; 8]> = A32([0, 2, 4, 6, 1, 3, 5, 7]);
static idct4_12_shuf2: A32<[i32; 8]> = A32([2, 0, 6, 4, 3, 1, 7, 5]);
static iadst8_12_shuf: A32<[i32; 8]> = A32([0, 4, 1, 5, 2, 6, 3, 7]);
static idct16_12_shuf: A32<[i32; 8]> = A32([0, 4, 1, 5, 3, 7, 2, 6]);
static iadst16_12_shuf: A32<[i32; 8]> = A32([3, 7, 0, 4, 2, 6, 1, 5]);
static pw_2048_m2048: A32<[i16; 8]> = A32([2048, 2048, 2048, 2048, -2048, -2048, -2048, -2048]);
static idct4_shuf: A32<[u8; 16]> = A32([0, 1, 4, 5, 12, 13, 8, 9, 2, 3, 6, 7, 14, 15, 10, 11]);
static idct32_shuf: A32<[u8; 16]> = A32([0, 1, 8, 9, 4, 5, 12, 13, 2, 3, 10, 11, 6, 7, 14, 15]);
/// `const idct64_mul_16bpc`
static idct64_mul_16bpc: A32<[i32; 48]> = A32([
    4095, 101, 2967, -2824, 3745, 1660, 3822, -1474, 401, 4076, 799, 4017, //
    -700, 4036, 2359, 3349, -2191, 3461, 897, 3996, -2598, -3166, -4017, -799, //
    4065, 501, 3229, -2520, 3564, 2019, 3948, -1092, 1931, 3612, 3406, 2276, //
    -301, 4085, 2675, 3102, -1842, 3659, 1285, 3889, -1189, -3920, -2276, -3406,
]);

/// `mova m, [table]` / `vbroadcasti128 m, [table]` of a static: the 32 (16) bytes at `p`.
#[inline(always)]
unsafe fn ld_tab256<T>(p: &A32<T>) -> __m256i {
    _mm256_load_si256((p as *const A32<T>).cast())
}
#[inline(always)]
unsafe fn ld_tab128<T>(p: &A32<T>) -> __m128i {
    _mm_load_si128((p as *const A32<T>).cast())
}

/// `cglobal inv_txfm_add_<type1>_<type2>_<size>_<bpc>bpc` (`INV_TXFM_FN type1, type2, eob_offset, size, bpc` and
/// the sizes' own entry points): the exported function `$name`, which runs `$body(dst, stride, c, eob, $args…)`,
/// the size's function that the kinds of the two passes (or nothing) select at compile time. `bitdepth_max`, the
/// fifth argument of the C signature, is not read by these kernels (only WHT reads it).
macro_rules! inv_txfm_fn {
    ($name:ident, $body:ident $(, $arg:expr)*) => {
        /// # Safety
        /// As for the family (module documentation), and the CPU supports AVX2 (the init selects this function
        /// under `DAV1D_X86_CPU_FLAG_AVX2`).
        #[target_feature(enable = "avx2")]
        pub unsafe extern "C" fn $name(dst: *mut u16, stride: isize, c: *mut i32, eob: c_int, _bdmax: c_int) {
            $body(dst, stride, c, eob $(, $arg)*)
        }
    };
}

/// A coefficient operand of `ITX_MULSUB_2D`: a register (`%if %N < 32`), a `pd_N` constant (`vpbroadcastd`), or a
/// `pd_A_B` pair (the packed form, flag 1: `vbroadcasti128`). An `i32` in 0..32 is a register number, any other
/// `i32` a constant; `Q(A, B)` is the pair `A_B`.
#[derive(Clone, Copy)]
enum Cf {
    R(usize),
    D(i32),
    P(i32, i32),
}
/// The pair `A_B` of `ITX_MULSUB_2D`'s packed form (`2896_1567` is `Q(2896, 1567)`).
#[derive(Clone, Copy)]
struct Q(i32, i32);
impl From<i32> for Cf {
    #[inline(always)]
    fn from(v: i32) -> Cf {
        if (0..32).contains(&v) {
            Cf::R(v as usize)
        } else {
            Cf::D(v)
        }
    }
}
impl From<Q> for Cf {
    #[inline(always)]
    fn from(q: Q) -> Cf {
        Cf::P(q.0, q.1)
    }
}
/// The broadcast of a constant coefficient (`vpbroadcastd m, [pd_N]` or, packed, `vbroadcasti128 m, [pd_A_B]`).
#[inline(always)]
unsafe fn cf_const(c: Cf) -> __m256i {
    match c {
        Cf::D(v) => pd(v),
        Cf::P(a, b) => pd_q(a, b),
        Cf::R(_) => unreachable!(),
    }
}

/// `%macro ITX_MULSUB_2D dst/src[1-2], tmp[1-3], rnd, coef[1-2], flags`:
/// `dst1 = (src1 * coef1 - src2 * coef2 + rnd) >> 12`, `dst2 = (src1 * coef2 + src2 * coef1 + rnd) >> 12`.
/// flags: 1 = packed (the coefficients are pairs, `Q(A, B)`), 2 = inv_dst2. `rnd` is the register that holds
/// the rounding constant, `None` where the assembly passes a non-number (`_`): no rounding and no shift.
#[inline(always)]
unsafe fn itx_mulsub_2d(
    m: &mut M,
    r1: usize,
    r2: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    rnd: Option<usize>,
    c1: impl Into<Cf>,
    c2: impl Into<Cf>,
    flags: u32,
) {
    let (c1, c2) = (c1.into(), c2.into());
    if let Cf::R(c) = c2 {
        m[t2] = _mm256_mullo_epi32(m[r1], m[c]); // pmulld m%4, m%1, m%8
        m[t1] = _mm256_mullo_epi32(m[r2], m[c]); // pmulld m%3, m%2, m%8
    } else {
        m[t1] = cf_const(c2); // vpbroadcastd m%3, [pd_%8] / vbroadcasti128 m%3, [pd_%8]
        m[t2] = _mm256_mullo_epi32(m[r1], m[t1]); // pmulld m%4, m%1, m%3
        m[t1] = _mm256_mullo_epi32(m[t1], m[r2]); // pmulld m%3, m%2
    }
    if let Cf::R(c) = c1 {
        m[r1] = _mm256_mullo_epi32(m[r1], m[c]); // pmulld m%1, m%7
        m[r2] = _mm256_mullo_epi32(m[r2], m[c]); // pmulld m%2, m%7
    } else {
        m[t3] = cf_const(c1); // vpbroadcastd m%5, [pd_%7] / vbroadcasti128 m%5, [pd_%7]
        m[r1] = _mm256_mullo_epi32(m[r1], m[t3]); // pmulld m%1, m%5
        m[r2] = _mm256_mullo_epi32(m[r2], m[t3]); // pmulld m%2, m%5
    }
    if flags & 2 != 0 {
        let rnd = rnd.unwrap();
        m[t2] = _mm256_sub_epi32(m[rnd], m[t2]); // psubd m%4, m%6, m%4
        m[r2] = _mm256_sub_epi32(m[t2], m[r2]); // psubd m%2, m%4, m%2
    } else {
        if let Some(rnd) = rnd {
            m[t2] = _mm256_add_epi32(m[t2], m[rnd]); // paddd m%4, m%6
        }
        m[r2] = _mm256_add_epi32(m[r2], m[t2]); // paddd m%2, m%4
    }
    if let Some(rnd) = rnd {
        m[r1] = _mm256_add_epi32(m[r1], m[rnd]); // paddd m%1, m%6
    }
    m[r1] = _mm256_sub_epi32(m[r1], m[t1]); // psubd m%1, m%3
    if rnd.is_some() {
        m[r2] = _mm256_srai_epi32::<12>(m[r2]); // psrad m%2, 12
        m[r1] = _mm256_srai_epi32::<12>(m[r1]); // psrad m%1, 12
    }
}

/// `%macro IDCT4_1D_PACKED dst/src[1-2], tmp[1-3], rnd`
#[inline(always)]
unsafe fn idct4_1d_packed(m: &mut M, r1: usize, r2: usize, t1: usize, t2: usize, t3: usize, rnd: usize) {
    itx_mulsub_2d(m, r1, r2, t1, t2, t3, Some(rnd), Q(2896, 1567), Q(2896, 3784), 1); // ITX_MULSUB_2D %1, %2, %3, %4, %5, %6, 2896_1567, 2896_3784, 1
    m[t1] = _mm256_unpackhi_epi64(m[r2], m[r1]); // punpckhqdq m%3, m%2, m%1 ; t3 t2
    m[r2] = _mm256_unpacklo_epi64(m[r2], m[r1]); // punpcklqdq m%2, m%1 ; t0 t1
    m[r1] = _mm256_add_epi32(m[r2], m[t1]); // paddd m%1, m%2, m%3 ; out0 out1
    m[r2] = _mm256_sub_epi32(m[r2], m[t1]); // psubd m%2, m%3 ; out3 out2
}

/// `%macro IDCT4_1D_PACKED_WORD dst/src[1-2], tmp[1-3], rnd` (`ymm`; the `WRAP_XMM` form is
/// `idct4_1d_packed_word_x`)
#[inline(always)]
unsafe fn idct4_1d_packed_word(m: &mut M, r1: usize, r2: usize, t1: usize, t2: usize, t3: usize, rnd: usize) {
    m[t3] = pw2(-3784, 1567); // vpbroadcastd m%5, [pw_m3784_1567]
    m[t1] = _mm256_unpackhi_epi16(m[r2], m[r1]); // punpckhwd m%3, m%2, m%1
    m[t2] = pw2(1567, 3784); // vpbroadcastd m%4, [pw_1567_3784]
    m[r2] = _mm256_unpacklo_epi16(m[r2], m[r1]); // punpcklwd m%2, m%1
    m[r1] = pw2(-2896, 2896); // vpbroadcastd m%1, [pw_m2896_2896]
    m[t3] = _mm256_madd_epi16(m[t3], m[t1]); // pmaddwd m%5, m%3
    m[t1] = _mm256_madd_epi16(m[t1], m[t2]); // pmaddwd m%3, m%4
    m[t2] = pw2(2896, 2896); // vpbroadcastd m%4, [pw_2896_2896]
    m[r1] = _mm256_madd_epi16(m[r1], m[r2]); // pmaddwd m%1, m%2
    m[r2] = _mm256_madd_epi16(m[r2], m[t2]); // pmaddwd m%2, m%4
    for x in [t3, t1, r1, r2] {
        m[x] = _mm256_add_epi32(m[x], m[rnd]); // REPX {paddd x, m%6}, m%5, m%3, m%1, m%2
    }
    for x in [t3, t1, r1, r2] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m%5, m%3, m%1, m%2
    }
    m[t1] = _mm256_packs_epi32(m[t1], m[t3]); // packssdw m%3, m%5 ; t3 t2
    m[r2] = _mm256_packs_epi32(m[r2], m[r1]); // packssdw m%2, m%1 ; t0 t1
    m[r1] = _mm256_adds_epi16(m[r2], m[t1]); // paddsw m%1, m%2, m%3 ; out0 out1
    m[r2] = _mm256_subs_epi16(m[r2], m[t1]); // psubsw m%2, m%3 ; out3 out2
}

/// `WRAP_XMM IDCT4_1D_PACKED_WORD dst/src[1-2], tmp[1-3], rnd` (`xmm`: every destination's upper half is zero)
#[inline(always)]
unsafe fn idct4_1d_packed_word_x(m: &mut M, r1: usize, r2: usize, t1: usize, t2: usize, t3: usize, rnd: usize) {
    m[t3] = zx(lo(pw2(-3784, 1567))); // vpbroadcastd xm%5, [pw_m3784_1567]
    m[t1] = zx(_mm_unpackhi_epi16(lo(m[r2]), lo(m[r1]))); // punpckhwd xm%3, xm%2, xm%1
    m[t2] = zx(lo(pw2(1567, 3784))); // vpbroadcastd xm%4, [pw_1567_3784]
    m[r2] = zx(_mm_unpacklo_epi16(lo(m[r2]), lo(m[r1]))); // punpcklwd xm%2, xm%1
    m[r1] = zx(lo(pw2(-2896, 2896))); // vpbroadcastd xm%1, [pw_m2896_2896]
    m[t3] = zx(_mm_madd_epi16(lo(m[t3]), lo(m[t1]))); // pmaddwd xm%5, xm%3
    m[t1] = zx(_mm_madd_epi16(lo(m[t1]), lo(m[t2]))); // pmaddwd xm%3, xm%4
    m[t2] = zx(lo(pw2(2896, 2896))); // vpbroadcastd xm%4, [pw_2896_2896]
    m[r1] = zx(_mm_madd_epi16(lo(m[r1]), lo(m[r2]))); // pmaddwd xm%1, xm%2
    m[r2] = zx(_mm_madd_epi16(lo(m[r2]), lo(m[t2]))); // pmaddwd xm%2, xm%4
    for x in [t3, t1, r1, r2] {
        m[x] = zx(_mm_add_epi32(lo(m[x]), lo(m[rnd]))); // REPX {paddd x, m%6}, m%5, m%3, m%1, m%2
    }
    for x in [t3, t1, r1, r2] {
        m[x] = zx(_mm_srai_epi32::<12>(lo(m[x]))); // REPX {psrad x, 12 }, m%5, m%3, m%1, m%2
    }
    m[t1] = zx(_mm_packs_epi32(lo(m[t1]), lo(m[t3]))); // packssdw xm%3, xm%5 ; t3 t2
    m[r2] = zx(_mm_packs_epi32(lo(m[r2]), lo(m[r1]))); // packssdw xm%2, xm%1 ; t0 t1
    m[r1] = zx(_mm_adds_epi16(lo(m[r2]), lo(m[t1]))); // paddsw xm%1, xm%2, xm%3 ; out0 out1
    m[r2] = zx(_mm_subs_epi16(lo(m[r2]), lo(m[t1]))); // psubsw xm%2, xm%3 ; out3 out2
}

/// `%macro IADST4_1D` (`ymm`; the `WRAP_XMM` form is `iadst4_1d_x`): `m0`-`m3` in, outputs (unrounded) in `m4`
/// (out0), `m6` (out1), `m2` (out2), `m3` (out3).
#[inline(always)]
unsafe fn iadst4_1d(m: &mut M) {
    m[5] = pd(1321); // vpbroadcastd m5, [pd_1321]
    m[7] = pd(2482); // vpbroadcastd m7, [pd_2482]
    m[4] = _mm256_mullo_epi32(m[0], m[5]); // pmulld m4, m0, m5 ; 1321*in0
    m[6] = _mm256_mullo_epi32(m[3], m[7]); // pmulld m6, m3, m7 ; 2482*in3
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6 ; 1321*in0 + 2482*in3
    m[6] = _mm256_mullo_epi32(m[0], m[7]); // pmulld m6, m0, m7 ; 2482*in0
    m[0] = _mm256_add_epi32(m[0], m[3]); // paddd m0, m3 ; in0 + in3
    m[7] = _mm256_add_epi32(m[7], m[5]); // paddd m7, m5 ; pd_3803
    m[5] = _mm256_mullo_epi32(m[5], m[2]); // pmulld m5, m2 ; 1321*in2
    m[3] = _mm256_mullo_epi32(m[3], m[7]); // pmulld m3, m7 ; 3803*in3
    m[7] = _mm256_mullo_epi32(m[7], m[2]); // pmulld m7, m2 ; 3803*in2
    m[2] = _mm256_sub_epi32(m[2], m[0]); // psubd m2, m0 ; in2 - in0 - in3
    m[0] = pd(-3344); // vpbroadcastd m0, [pd_m3344]
    m[1] = _mm256_mullo_epi32(m[1], m[0]); // pmulld m1, m0 ; -t3
    m[2] = _mm256_mullo_epi32(m[2], m[0]); // pmulld m2, m0 ; out2 (unrounded)
    m[6] = _mm256_sub_epi32(m[6], m[5]); // psubd m6, m5 ; 2482*in0 - 1321*in2
    m[4] = _mm256_add_epi32(m[4], m[7]); // paddd m4, m7 ;  t0
    m[6] = _mm256_sub_epi32(m[6], m[3]); // psubd m6, m3 ;  t1
    m[3] = _mm256_add_epi32(m[4], m[6]); // paddd m3, m4, m6
    m[4] = _mm256_sub_epi32(m[4], m[1]); // psubd m4, m1 ; out0 (unrounded)
    m[6] = _mm256_sub_epi32(m[6], m[1]); // psubd m6, m1 ; out1 (unrounded)
    m[3] = _mm256_add_epi32(m[3], m[1]); // paddd m3, m1 ; out3 (unrounded)
}

/// `WRAP_XMM IADST4_1D` (`xmm`: every destination's upper half is zero)
#[inline(always)]
unsafe fn iadst4_1d_x(m: &mut M) {
    m[5] = zx(lo(pd(1321))); // vpbroadcastd xm5, [pd_1321]
    m[7] = zx(lo(pd(2482))); // vpbroadcastd xm7, [pd_2482]
    m[4] = zx(_mm_mullo_epi32(lo(m[0]), lo(m[5]))); // pmulld xm4, xm0, xm5 ; 1321*in0
    m[6] = zx(_mm_mullo_epi32(lo(m[3]), lo(m[7]))); // pmulld xm6, xm3, xm7 ; 2482*in3
    m[4] = zx(_mm_add_epi32(lo(m[4]), lo(m[6]))); // paddd xm4, xm6 ; 1321*in0 + 2482*in3
    m[6] = zx(_mm_mullo_epi32(lo(m[0]), lo(m[7]))); // pmulld xm6, xm0, xm7 ; 2482*in0
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[3]))); // paddd xm0, xm3 ; in0 + in3
    m[7] = zx(_mm_add_epi32(lo(m[7]), lo(m[5]))); // paddd xm7, xm5 ; pd_3803
    m[5] = zx(_mm_mullo_epi32(lo(m[5]), lo(m[2]))); // pmulld xm5, xm2 ; 1321*in2
    m[3] = zx(_mm_mullo_epi32(lo(m[3]), lo(m[7]))); // pmulld xm3, xm7 ; 3803*in3
    m[7] = zx(_mm_mullo_epi32(lo(m[7]), lo(m[2]))); // pmulld xm7, xm2 ; 3803*in2
    m[2] = zx(_mm_sub_epi32(lo(m[2]), lo(m[0]))); // psubd xm2, xm0 ; in2 - in0 - in3
    m[0] = zx(lo(pd(-3344))); // vpbroadcastd xm0, [pd_m3344]
    m[1] = zx(_mm_mullo_epi32(lo(m[1]), lo(m[0]))); // pmulld xm1, xm0 ; -t3
    m[2] = zx(_mm_mullo_epi32(lo(m[2]), lo(m[0]))); // pmulld xm2, xm0 ; out2 (unrounded)
    m[6] = zx(_mm_sub_epi32(lo(m[6]), lo(m[5]))); // psubd xm6, xm5 ; 2482*in0 - 1321*in2
    m[4] = zx(_mm_add_epi32(lo(m[4]), lo(m[7]))); // paddd xm4, xm7 ;  t0
    m[6] = zx(_mm_sub_epi32(lo(m[6]), lo(m[3]))); // psubd xm6, xm3 ;  t1
    m[3] = zx(_mm_add_epi32(lo(m[4]), lo(m[6]))); // paddd xm3, xm4, xm6
    m[4] = zx(_mm_sub_epi32(lo(m[4]), lo(m[1]))); // psubd xm4, xm1 ; out0 (unrounded)
    m[6] = zx(_mm_sub_epi32(lo(m[6]), lo(m[1]))); // psubd xm6, xm1 ; out1 (unrounded)
    m[3] = zx(_mm_add_epi32(lo(m[3]), lo(m[1]))); // paddd xm3, xm1 ; out3 (unrounded)
}

/// `%macro IDCT4_1D src[1-4], tmp[1-3], rnd`
#[inline(always)]
unsafe fn idct4_1d(m: &mut M, r: [usize; 4], t: [usize; 3], rnd: usize) {
    let [r1, r2, r3, r4] = r;
    let [t1, t2, t3] = t;
    itx_mulsub_2d(m, r2, r4, t1, t2, t3, Some(rnd), 1567, 3784, 0); // ITX_MULSUB_2D %2, %4, %5, %6, %7, %8, 1567, 3784 ; t2, t3
    m[t1] = pd(2896); // vpbroadcastd m%5, [pd_2896]
    m[r1] = _mm256_mullo_epi32(m[r1], m[t1]); // pmulld m%1, m%5
    m[r3] = _mm256_mullo_epi32(m[r3], m[t1]); // pmulld m%3, m%5
    m[r1] = _mm256_add_epi32(m[r1], m[rnd]); // paddd m%1, m%8
    m[t1] = _mm256_add_epi32(m[r1], m[r3]); // paddd m%5, m%1, m%3
    m[r1] = _mm256_sub_epi32(m[r1], m[r3]); // psubd m%1, m%3
    m[t1] = _mm256_srai_epi32::<12>(m[t1]); // psrad m%5, 12 ; t0
    m[r1] = _mm256_srai_epi32::<12>(m[r1]); // psrad m%1, 12 ; t1
    m[r3] = _mm256_sub_epi32(m[r1], m[r2]); // psubd m%3, m%1, m%2
    m[r2] = _mm256_add_epi32(m[r2], m[r1]); // paddd m%2, m%1
    m[r1] = _mm256_add_epi32(m[t1], m[r4]); // paddd m%1, m%5, m%4
    m[r4] = _mm256_sub_epi32(m[t1], m[r4]); // psubd m%4, m%5, m%4
}

/// `%macro TRANSPOSE_8X8_DWORD src/dst[1-8], tmp[1-4]`
#[inline(always)]
unsafe fn transpose_8x8_dword(m: &mut M, r: [usize; 8], t: [usize; 4]) {
    let [r1, r2, r3, r4, r5, r6, r7, r8] = r;
    let [r9, r10, r11, r12] = t;
    m[r9] = _mm256_unpacklo_epi32(m[r1], m[r2]); // punpckldq m%9, m%1, m%2 ; aibj emfn
    m[r1] = _mm256_unpackhi_epi32(m[r1], m[r2]); // punpckhdq m%1, m%2 ; ckdl gohp
    m[r10] = _mm256_unpacklo_epi32(m[r3], m[r4]); // punpckldq m%10, m%3, m%4 ; qyrz uCvD
    m[r3] = _mm256_unpackhi_epi32(m[r3], m[r4]); // punpckhdq m%3, m%4 ; sAtB wExF
    m[r11] = _mm256_unpacklo_epi32(m[r5], m[r6]); // punpckldq m%11, m%5, m%6 ; GOHP KSLT
    m[r5] = _mm256_unpackhi_epi32(m[r5], m[r6]); // punpckhdq m%5, m%6 ; IQJR MUNV
    m[r12] = _mm256_unpacklo_epi32(m[r7], m[r8]); // punpckldq m%12, m%7, m%8 ; WeXf aibj
    m[r7] = _mm256_unpackhi_epi32(m[r7], m[r8]); // punpckhdq m%7, m%8 ; YgZh ckdl
    m[r2] = _mm256_unpacklo_epi64(m[r9], m[r10]); // punpcklqdq m%2, m%9, m%10 ; aiqy emuC
    m[r9] = _mm256_unpackhi_epi64(m[r9], m[r10]); // punpckhqdq m%9, m%10 ; bjrz fnvD
    m[r4] = _mm256_unpacklo_epi64(m[r1], m[r3]); // punpcklqdq m%4, m%1, m%3 ; cksA gowE
    m[r10] = _mm256_unpackhi_epi64(m[r1], m[r3]); // punpckhqdq m%10, m%1, m%3 ; dltB hpxF
    m[r6] = _mm256_unpacklo_epi64(m[r11], m[r12]); // punpcklqdq m%6, m%11, m%12 ; GOWe KSai
    m[r11] = _mm256_unpackhi_epi64(m[r11], m[r12]); // punpckhqdq m%11, m%12 ; HPXf LTbj
    m[r8] = _mm256_unpacklo_epi64(m[r5], m[r7]); // punpcklqdq m%8, m%5, m%7 ; IQYg MUck
    m[r12] = _mm256_unpackhi_epi64(m[r5], m[r7]); // punpckhqdq m%12, m%5, m%7 ; JRZh NVdl
    m[r1] = _mm256_permute2x128_si256::<0x20>(m[r2], m[r6]); // vperm2i128 m%1, m%2, m%6, 0x20 ; out0
    m[r5] = _mm256_permute2x128_si256::<0x31>(m[r2], m[r6]); // vperm2i128 m%5, m%2, m%6, 0x31 ; out4
    m[r2] = _mm256_permute2x128_si256::<0x20>(m[r9], m[r11]); // vperm2i128 m%2, m%9, m%11, 0x20 ; out1
    m[r6] = _mm256_permute2x128_si256::<0x31>(m[r9], m[r11]); // vperm2i128 m%6, m%9, m%11, 0x31 ; out5
    m[r3] = _mm256_permute2x128_si256::<0x20>(m[r4], m[r8]); // vperm2i128 m%3, m%4, m%8, 0x20 ; out2
    m[r7] = _mm256_permute2x128_si256::<0x31>(m[r4], m[r8]); // vperm2i128 m%7, m%4, m%8, 0x31 ; out6
    m[r4] = _mm256_permute2x128_si256::<0x20>(m[r10], m[r12]); // vperm2i128 m%4, m%10, m%12, 0x20 ; out3
    m[r8] = _mm256_permute2x128_si256::<0x31>(m[r10], m[r12]); // vperm2i128 m%8, m%10, m%12, 0x31 ; out7
}

// The DC-only tails that the sizes jump into (the `.dconly*` labels of the 10 bpc `INV_TXFM_<SIZE>_FN` macros and
// of 32x8; the 12 bpc paths load their own `dconly_12bpc` and jump to the 10 bpc label). On entry the size's
// `dconly_<bpc>bpc` constant is in `xm2` (4x4, 4x8), `m2` (8x4, 8x8, 8x16) or `m3` (16x4 and wider). `r6d` is
// the DC coefficient as it goes through the scalings, `r3d` the number of rows (`eobd`, which is 0 on this path,
// with the height `or`ed in). The general-purpose arithmetic is the assembly's 32-bit `imul`/`add`/`sar`
// (wrapping, arithmetic shift).

/// `.dconly` of `inv_txfm_add_dct_dct_4x4_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_4x4_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
    c.write_unaligned(eob); // mov [cq], eobd ; 0
    let r3d = eob | 4; // or r3d, 4
    inv_txfm_add_dct_dct_4x4_10bpc_dconly2(m, dst, stride, r6d, r3d)
}

/// `.dconly2` of `inv_txfm_add_dct_dct_4x4_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_4x4_10bpc_dconly2(m: &mut M, dst: Dst, stride: isize, mut r6d: i32, r3d: i32) {
    r6d = r6d.wrapping_add(128); // add r6d, 128
    r6d >>= 8; // sar r6d, 8
    inv_txfm_add_dct_dct_4x4_10bpc_dconly3(m, dst, stride, r6d, r3d)
}

/// `.dconly3` of `inv_txfm_add_dct_dct_4x4_10bpc` (`.dconly_loop`: two rows of four pixels per iteration)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_4x4_10bpc_dconly3(m: &mut M, mut dst: Dst, stride: isize, mut r6d: i32, mut r3d: i32) {
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    r6d = r6d.wrapping_add(2176); // add r6d, 2176
    r6d >>= 12; // sar r6d, 12
    m[0] = zx(_mm_cvtsi32_si128(r6d)); // movd xm0, r6d
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw xm0, xm2
    m[0] = zx(_mm_broadcastw_epi16(lo(m[0]))); // vpbroadcastw xm0, xm0
    loop {
        // .dconly_loop:
        m[1] = zx(_mm_loadl_epi64(dst.cast())); // movq xm1, [dstq+strideq*0]
        m[1] = zx(movhps_ld(lo(m[1]), at(dst, stride).cast())); // movhps xm1, [dstq+strideq*1]
        m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[0]))); // paddsw xm1, xm0
        m[1] = zx(_mm_subs_epu16(lo(m[1]), lo(m[2]))); // psubusw xm1, xm2
        _mm_storel_epi64(dst.cast(), lo(m[1])); // movq [dstq+strideq*0], xm1
        movhps_st(at(dst, stride).cast(), lo(m[1])); // movhps [dstq+strideq*1], xm1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        r3d -= 2; // sub r3d, 2
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
    // WRAP_XMM RET
}

/// `.dconly` of `inv_txfm_add_dct_dct_4x8_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_4x8_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
    c.write_unaligned(eob); // mov [cq], eobd ; 0
    let r3d = eob | 8; // or r3d, 8
    r6d = r6d.wrapping_add(128); // add r6d, 128
    r6d >>= 8; // sar r6d, 8
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    inv_txfm_add_dct_dct_4x4_10bpc_dconly2(m, dst, stride, r6d, r3d) // jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly2
}

/// `.dconly` of `inv_txfm_add_dct_dct_8x4_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x4_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
    c.write_unaligned(eob); // mov [cq], eobd ; 0
    let r3d = eob | 4; // or r3d, 4
    r6d = r6d.wrapping_add(128); // add r6d, 128
    r6d >>= 8; // sar r6d, 8
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    r6d = r6d.wrapping_add(128); // add r6d, 128
    r6d >>= 8; // sar r6d, 8
    inv_txfm_add_dct_dct_8x8_10bpc_dconly3(m, dst, stride, r6d, r3d) // jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly3
}

/// `.dconly` of `inv_txfm_add_dct_dct_8x8_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x8_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
    c.write_unaligned(eob); // mov [cq], eobd ; 0
    let r3d = eob | 8; // or r3d, 8
    inv_txfm_add_dct_dct_8x8_10bpc_dconly2(m, dst, stride, r6d, r3d)
}

/// `.dconly2` of `inv_txfm_add_dct_dct_8x8_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x8_10bpc_dconly2(m: &mut M, dst: Dst, stride: isize, mut r6d: i32, r3d: i32) {
    r6d = r6d.wrapping_add(384); // add r6d, 384
    r6d >>= 9; // sar r6d, 9
    inv_txfm_add_dct_dct_8x8_10bpc_dconly3(m, dst, stride, r6d, r3d)
}

/// `.dconly3` of `inv_txfm_add_dct_dct_8x8_10bpc` (`.dconly_loop`: two rows of eight pixels per iteration)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x8_10bpc_dconly3(m: &mut M, mut dst: Dst, stride: isize, mut r6d: i32, mut r3d: i32) {
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    r6d = r6d.wrapping_add(2176); // add r6d, 2176
    r6d >>= 12; // sar r6d, 12
    m[0] = zx(_mm_cvtsi32_si128(r6d)); // movd xm0, r6d
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw xm0, xm2
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    loop {
        // .dconly_loop:
        m[1] = zx(_mm_loadu_si128(dst.cast())); // mova xm1, [dstq+strideq*0]
        m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(dst, stride).cast())); // vinserti128 m1, [dstq+strideq*1], 1
        m[1] = _mm256_adds_epi16(m[1], m[0]); // paddsw m1, m0
        m[1] = _mm256_subs_epu16(m[1], m[2]); // psubusw m1, m2
        _mm_storeu_si128(dst.cast(), lo(m[1])); // mova [dstq+strideq*0], xm1
        _mm_storeu_si128(at(dst, stride).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+strideq*1], m1, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        r3d -= 2; // sub r3d, 2
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `.dconly` of `inv_txfm_add_dct_dct_16x4_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x4_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
    c.write_unaligned(eob); // mov [cq], eobd ; 0
    let r3d = eob | 4; // or r3d, 4
    inv_txfm_add_dct_dct_16x4_10bpc_dconly2(m, dst, stride, r6d, r3d)
}

/// `.dconly2` of `inv_txfm_add_dct_dct_16x4_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x4_10bpc_dconly2(m: &mut M, dst: Dst, stride: isize, mut r6d: i32, r3d: i32) {
    r6d = r6d.wrapping_add(384); // add r6d, 384
    r6d >>= 9; // sar r6d, 9
    inv_txfm_add_dct_dct_16x4_10bpc_dconly3(m, dst, stride, r6d, r3d)
}

/// `.dconly3` of `inv_txfm_add_dct_dct_16x4_10bpc` (`.dconly_loop`: two rows of 16 pixels per iteration)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x4_10bpc_dconly3(m: &mut M, mut dst: Dst, stride: isize, mut r6d: i32, mut r3d: i32) {
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    r6d = r6d.wrapping_add(2176); // add r6d, 2176
    r6d >>= 12; // sar r6d, 12
    m[0] = zx(_mm_cvtsi32_si128(r6d)); // movd xm0, r6d
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[3]))); // paddsw xm0, xm3
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    loop {
        // .dconly_loop:
        m[1] = _mm256_adds_epi16(m[0], _mm256_loadu_si256(dst.cast())); // paddsw m1, m0, [dstq+strideq*0]
        m[2] = _mm256_adds_epi16(m[0], _mm256_loadu_si256(at(dst, stride).cast())); // paddsw m2, m0, [dstq+strideq*1]
        m[1] = _mm256_subs_epu16(m[1], m[3]); // psubusw m1, m3
        m[2] = _mm256_subs_epu16(m[2], m[3]); // psubusw m2, m3
        _mm256_storeu_si256(dst.cast(), m[1]); // mova [dstq+strideq*0], m1
        _mm256_storeu_si256(at(dst, stride).cast(), m[2]); // mova [dstq+strideq*1], m2
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        r3d -= 2; // sub r3d, 2
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `.dconly` of `inv_txfm_add_dct_dct_32x8_10bpc` (entered after `imul r6d, [cq], 181`, `mov [cq], eobd` and
/// `or r3d, <rows>`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, mut r6d: i32, r3d: i32) {
    r6d = r6d.wrapping_add(640); // add r6d, 640
    r6d >>= 10; // sar r6d, 10
    inv_txfm_add_dct_dct_32x8_10bpc_dconly2(m, dst, stride, r6d, r3d)
}

/// `.dconly2` of `inv_txfm_add_dct_dct_32x8_10bpc` (`.dconly_loop`: one row of 32 pixels per iteration)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_10bpc_dconly2(m: &mut M, mut dst: Dst, stride: isize, mut r6d: i32, mut r3d: i32) {
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    r6d = r6d.wrapping_add(2176); // add r6d, 2176
    r6d >>= 12; // sar r6d, 12
    m[0] = zx(_mm_cvtsi32_si128(r6d)); // movd xm0, r6d
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[3]))); // paddsw xm0, xm3
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    loop {
        // .dconly_loop:
        m[1] = _mm256_adds_epi16(m[0], _mm256_loadu_si256(dst.cast())); // paddsw m1, m0, [dstq+32*0]
        m[2] = _mm256_adds_epi16(m[0], _mm256_loadu_si256(at(dst, 32).cast())); // paddsw m2, m0, [dstq+32*1]
        m[1] = _mm256_subs_epu16(m[1], m[3]); // psubusw m1, m3
        m[2] = _mm256_subs_epu16(m[2], m[3]); // psubusw m2, m3
        _mm256_storeu_si256(dst.cast(), m[1]); // mova [dstq+32*0], m1
        _mm256_storeu_si256(at(dst, 32).cast(), m[2]); // mova [dstq+32*1], m2
        dst = at(dst, stride); // add dstq, strideq
        r3d -= 1; // dec r3d
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

// --- WHT (the pilot: a whole entry point) ---------------------------------------------------------------------

/// `%macro IWHT4_1D_PACKED`: `m0` = in0 in2, `m1` = in1 in3
#[inline(always)]
unsafe fn iwht4_1d_packed(m: &mut M) {
    m[2] = _mm256_sub_epi32(m[0], m[1]); // psubd m2, m0, m1 ; t2
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1 ; t0
    m[2] = _mm256_permute4x64_epi64::<0xfa>(m[2]); // vpermq m2, m2, q3322
    m[0] = _mm256_permute4x64_epi64::<0x50>(m[0]); // vpermq m0, m0, q1100
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
    m[3] = _mm256_sub_epi32(m[0], m[2]); // psubd m3, m0, m2
    m[3] = _mm256_srai_epi32::<1>(m[3]); // psrad m3, 1
    m[3] = _mm256_sub_epi32(m[3], m[1]); // psubd m3, m1 ; t1 t3
    m[0] = _mm256_sub_epi32(m[0], m[3]); // psubd m0, m3 ; ____ out0
    m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3 ; out3 ____
}

/// `cglobal inv_txfm_add_wht_wht_4x4_16bpc, 3, 7, 6, dst, stride, c, eob, bdmax`
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2 (the init selects this function under
/// `DAV1D_X86_CPU_FLAG_AVX2`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_inv_txfm_add_wht_wht_4x4_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    c: *mut i32,
    _eob: c_int,
    bdmax: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    m[0] = zx(_mm_loadu_si128(c.cast())); // mova xm0, [cq+16*0]
    m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_loadu_si128(at(c, 16 * 2).cast())); // vinserti128 m0, [cq+16*2], 1
    m[1] = zx(_mm_loadu_si128(at(c, 16).cast())); // mova xm1, [cq+16*1]
    m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_loadu_si128(at(c, 16 * 3).cast())); // vinserti128 m1, [cq+16*3], 1
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(at(c, 32).cast(), m[4]); // mova [cq+32*1], m4
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[0] = _mm256_srai_epi32::<2>(m[0]); // psrad m0, 2
    m[1] = _mm256_srai_epi32::<2>(m[1]); // psrad m1, 2
    iwht4_1d_packed(m); // IWHT4_1D_PACKED
    m[0] = _mm256_unpackhi_epi32(m[0], m[3]); // punpckhdq m0, m3
    m[3] = _mm256_unpacklo_epi32(m[3], m[2]); // punpckldq m3, m2
    m[1] = _mm256_unpackhi_epi64(m[0], m[3]); // punpckhqdq m1, m0, m3
    m[0] = _mm256_unpacklo_epi64(m[0], m[3]); // punpcklqdq m0, m3
    iwht4_1d_packed(m); // IWHT4_1D_PACKED
    m[0] = _mm256_blend_epi32::<0x33>(m[0], m[2]); // vpblendd m0, m2, 0x33
    m[0] = _mm256_packs_epi32(m[0], m[3]); // packssdw m0, m3
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
    m[1] = zx(_mm_unpackhi_epi32(lo(m[0]), lo(m[2]))); // punpckhdq xm1, xm0, xm2 ; out2 out1
    m[0] = zx(_mm_unpacklo_epi32(lo(m[0]), lo(m[2]))); // punpckldq xm0, xm2 ; out3 out0
    m[2] = zx(_mm_loadl_epi64(at(r6, stride).cast())); // movq xm2, [r6  +strideq*1]
    m[2] = zx(movhps_ld(lo(m[2]), dst.cast())); // movhps xm2, [dstq+strideq*0]
    m[3] = zx(_mm_loadl_epi64(r6.cast())); // movq xm3, [r6  +strideq*0]
    m[3] = zx(movhps_ld(lo(m[3]), at(dst, stride).cast())); // movhps xm3, [dstq+strideq*1]
    m[5] = zx(_mm_cvtsi32_si128(bdmax)); // movd xm5, bdmaxd
    m[5] = zx(_mm_broadcastw_epi16(lo(m[5]))); // vpbroadcastw xm5, xm5
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[2]))); // paddsw xm0, xm2
    m[1] = zx(_mm_adds_epi16(lo(m[1]), lo(m[3]))); // paddsw xm1, xm3
    m[0] = zx(_mm_max_epi16(lo(m[0]), lo(m[4]))); // pmaxsw xm0, xm4
    m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[4]))); // pmaxsw xm1, xm4
    m[0] = zx(_mm_min_epi16(lo(m[0]), lo(m[5]))); // pminsw xm0, xm5
    m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[5]))); // pminsw xm1, xm5
    movhps_st(dst.cast(), lo(m[0])); // movhps [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[1])); // movhps [dstq+strideq*1], xm1
    _mm_storel_epi64(r6.cast(), lo(m[1])); // movq [r6  +strideq*0], xm1
    _mm_storel_epi64(at(r6, stride).cast(), lo(m[0])); // movq [r6  +strideq*1], xm0
}

// --- part 1: itx16_avx2.asm lines 351-1216 (4x4, 4x8) ---

// --- 4x4, 10 bpc --------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X4_FN type1, type2, 10` (`INV_TXFM_FN type1, type2, 0, 4x4, 10` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x4_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = zx(lo(pw(0x7c00))); // vpbroadcastd xm2, [dconly_10bpc]
        return inv_txfm_add_dct_dct_4x4_10bpc_dconly(m, dst, stride, c, eob);
    }
    match t1 {
        K::Dct => idct_4x4_internal_10bpc_pass1(m, c),
        K::Adst => iadst_4x4_internal_10bpc_pass1(m, c),
        K::FlipAdst => iflipadst_4x4_internal_10bpc_pass1(m, c),
        K::Identity => iidentity_4x4_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x4_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_4x4_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x4_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x4_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_4x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_4x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    idct_4x4_internal_10bpc_main(m, c); // call .main
    m[2] = _mm256_broadcastsi128_si256(ld_tab128(&idct4_shuf)); // vbroadcasti128 m2, [idct4_shuf]
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[0] = _mm256_shuffle_epi8(m[0], m[2]); // pshufb m0, m2
}

/// `.pass2` of `idct_4x4_internal_10bpc` (`m5` = `pd_2048` from the first pass)
#[inline(always)]
unsafe fn idct_4x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    idct4_1d_packed_word_x(m, 0, 1, 2, 3, 4, 5); // WRAP_XMM IDCT4_1D_PACKED_WORD 0, 1, 2, 3, 4, 5
    m[5] = zx(_mm_packs_epi32(lo(m[5]), lo(m[5]))); // packssdw xm5, xm5 ; pw_2048
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[5]))); // pmulhrsw xm0, xm5
    m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[5]))); // pmulhrsw xm1, xm5
    m[2] = zx(_mm_loadl_epi64(dst.cast())); // movq xm2, [dstq+strideq*0]
    m[2] = zx(movhps_ld(lo(m[2]), at(dst, stride).cast())); // movhps xm2, [dstq+strideq*1]
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[3] = zx(_mm_loadl_epi64(at(r6, stride).cast())); // movq xm3, [r6  +strideq*1]
    m[3] = zx(movhps_ld(lo(m[3]), r6.cast())); // movhps xm3, [r6  +strideq*0]
    m[5] = zx(lo(pw(0x3ff))); // vpbroadcastd xm5, [pixel_10bpc_max]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(at(c, 32).cast(), m[4]); // mova [cq+32*1], m4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
    m[0] = zx(_mm_max_epi16(lo(m[0]), lo(m[4]))); // pmaxsw xm0, xm4
    m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[4]))); // pmaxsw xm1, xm4
    m[0] = zx(_mm_min_epi16(lo(m[0]), lo(m[5]))); // pminsw xm0, xm5
    m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[5]))); // pminsw xm1, xm5
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    movhps_st(r6.cast(), lo(m[1])); // movhps [r6  +strideq*0], xm1
    _mm_storel_epi64(at(r6, stride).cast(), lo(m[1])); // movq [r6  +strideq*1], xm1
}

/// `.main` of `idct_4x4_internal_10bpc` (falls through into `.main2`)
#[inline(always)]
unsafe fn idct_4x4_internal_10bpc_main(m: &mut M, c: Coef) {
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.cast())); // vpermq m0, [cq+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32).cast())); // vpermq m1, [cq+32*1], q3120
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    idct_4x4_internal_10bpc_main2(m)
}

/// `.main2` of `idct_4x4_internal_10bpc`: `m0`, `m1` in, `m5` = `pd_2048`
#[inline(always)]
unsafe fn idct_4x4_internal_10bpc_main2(m: &mut M) {
    idct4_1d_packed(m, 0, 1, 2, 3, 4, 5); // IDCT4_1D_PACKED 0, 1, 2, 3, 4, 5
}

/// `cglobal iadst_4x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_4x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x4_internal_10bpc_main(m, c); // call .main
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[6])); // vinserti128 m0, m4, xm6, 1
    m[1] = _mm256_inserti128_si256::<1>(m[2], lo(m[3])); // vinserti128 m1, m2, xm3, 1
    iadst_4x4_internal_10bpc_pass1_end(m)
}

/// `.pass1_end` of `iadst_4x4_internal_10bpc`
#[inline(always)]
unsafe fn iadst_4x4_internal_10bpc_pass1_end(m: &mut M) {
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[2] = ld_tab256(&itx4_shuf); // mova m2, [itx4_shuf]
    m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[0] = _mm256_srai_epi32::<12>(m[0]); // psrad m0, 12
    m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[0] = _mm256_permutevar8x32_epi32(m[0], m[2]); // vpermd m0, m2, m0
    m[2] = _mm256_srli_epi32::<4>(m[2]); // psrld m2, 4
    m[0] = _mm256_shuffle_epi8(m[0], m[2]); // pshufb m0, m2
}

/// `.pass2` of `iadst_4x4_internal_10bpc` (with its `.end`)
#[inline(always)]
unsafe fn iadst_4x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    // lea r6, [deint_shuf+128] (the 8 bpc constants' base)
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    iadst_4x4_internal_8bpc_main(m); // call m(iadst_4x4_internal_8bpc).main
                                     // .end:
    m[4] = zx(lo(pw(2048))); // vpbroadcastd xm4, [pw_2048]
    m[2] = zx(_mm_loadl_epi64(dst.cast())); // movq xm2, [dstq+strideq*0]
    m[2] = zx(movhps_ld(lo(m[2]), at(dst, stride).cast())); // movhps xm2, [dstq+strideq*1]
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[3] = zx(_mm_loadl_epi64(r6.cast())); // movq xm3, [r6  +strideq*0]
    m[3] = zx(movhps_ld(lo(m[3]), at(r6, stride).cast())); // movhps xm3, [r6  +strideq*1]
    m[5] = zx(lo(pw(0x3ff))); // vpbroadcastd xm5, [pixel_10bpc_max]
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[4]))); // pmulhrsw xm0, xm4
    m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[4]))); // pmulhrsw xm1, xm4
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(at(c, 32).cast(), m[4]); // mova [cq+32*1], m4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
    m[0] = zx(_mm_max_epi16(lo(m[0]), lo(m[4]))); // pmaxsw xm0, xm4
    m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[4]))); // pmaxsw xm1, xm4
    m[0] = zx(_mm_min_epi16(lo(m[0]), lo(m[5]))); // pminsw xm0, xm5
    m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[5]))); // pminsw xm1, xm5
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    _mm_storel_epi64(r6.cast(), lo(m[1])); // movq [r6  +strideq*0], xm1
    movhps_st(at(r6, stride).cast(), lo(m[1])); // movhps [r6  +strideq*1], xm1
}

/// `.main` of `iadst_4x4_internal_10bpc` (falls through into `.main2`): outputs as `IADST4_1D` (`xmm`)
#[inline(always)]
unsafe fn iadst_4x4_internal_10bpc_main(m: &mut M, c: Coef) {
    m[0] = zx(_mm_loadu_si128(c.cast())); // mova xm0, [cq+16*0]
    m[1] = zx(_mm_loadu_si128(at(c, 16).cast())); // mova xm1, [cq+16*1]
    m[2] = zx(_mm_loadu_si128(at(c, 16 * 2).cast())); // mova xm2, [cq+16*2]
    m[3] = zx(_mm_loadu_si128(at(c, 16 * 3).cast())); // mova xm3, [cq+16*3]
    iadst_4x4_internal_10bpc_main2(m)
}

/// `.main2` of `iadst_4x4_internal_10bpc`: `xm0`-`xm3` in
#[inline(always)]
unsafe fn iadst_4x4_internal_10bpc_main2(m: &mut M) {
    iadst4_1d_x(m); // WRAP_XMM IADST4_1D
}

/// `cglobal iflipadst_4x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x4_internal_10bpc_main(m, c); // call m(iadst_4x4_internal_10bpc).main
    m[0] = _mm256_inserti128_si256::<1>(m[3], lo(m[2])); // vinserti128 m0, m3, xm2, 1
    m[1] = _mm256_inserti128_si256::<1>(m[6], lo(m[4])); // vinserti128 m1, m6, xm4, 1
    iadst_4x4_internal_10bpc_pass1_end(m) // jmp m(iadst_4x4_internal_10bpc).pass1_end
}

/// `.pass2` of `iflipadst_4x4_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_4x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    // lea r6, [deint_shuf+128] (the 8 bpc constants' base)
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    iadst_4x4_internal_8bpc_main(m); // call m(iadst_4x4_internal_8bpc).main
    m[4] = zx(lo(pw(2048))); // vpbroadcastd xm4, [pw_2048]
    m[3] = zx(_mm_loadl_epi64(at(dst, stride).cast())); // movq xm3, [dstq+strideq*1]
    m[3] = zx(movhps_ld(lo(m[3]), dst.cast())); // movhps xm3, [dstq+strideq*0]
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[2] = zx(_mm_loadl_epi64(at(r6, stride).cast())); // movq xm2, [r6  +strideq*1]
    m[2] = zx(movhps_ld(lo(m[2]), r6.cast())); // movhps xm2, [r6  +strideq*0]
    m[5] = zx(lo(pw(0x3ff))); // vpbroadcastd xm5, [pixel_10bpc_max]
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[4]))); // pmulhrsw xm0, xm4
    m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[4]))); // pmulhrsw xm1, xm4
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(at(c, 32).cast(), m[4]); // mova [cq+32*1], m4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
    m[0] = zx(_mm_max_epi16(lo(m[0]), lo(m[4]))); // pmaxsw xm0, xm4
    m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[4]))); // pmaxsw xm1, xm4
    m[0] = zx(_mm_min_epi16(lo(m[0]), lo(m[5]))); // pminsw xm0, xm5
    m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[5]))); // pminsw xm1, xm5
    movhps_st(dst.cast(), lo(m[1])); // movhps [dstq+strideq*0], xm1
    _mm_storel_epi64(at(dst, stride).cast(), lo(m[1])); // movq [dstq+strideq*1], xm1
    movhps_st(r6.cast(), lo(m[0])); // movhps [r6  +strideq*0], xm0
    _mm_storel_epi64(at(r6, stride).cast(), lo(m[0])); // movq [r6  +strideq*1], xm0
}

/// `cglobal iidentity_4x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[1] = pd(5793); // vpbroadcastd m1, [pd_5793]
    m[0] = _mm256_mullo_epi32(m[1], _mm256_loadu_si256(c.cast())); // pmulld m0, m1, [cq+32*0]
    m[1] = _mm256_mullo_epi32(m[1], _mm256_loadu_si256(at(c, 32).cast())); // pmulld m1, [cq+32*1]
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[3] = ld_tab256(&itx4_shuf); // mova m3, [itx4_shuf]
    m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[0] = _mm256_srai_epi32::<12>(m[0]); // psrad m0, 12
    m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[0] = _mm256_permutevar8x32_epi32(m[0], m[3]); // vpermd m0, m3, m0
    m[3] = _mm256_srli_epi32::<4>(m[3]); // psrld m3, 4
    m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
}

/// `.pass2` of `iidentity_4x4_internal_10bpc` (`m5` = `pd_2048` from the first pass)
#[inline(always)]
unsafe fn iidentity_4x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[1] = pw(1697 * 8); // vpbroadcastd m1, [pw_1697x8]
    m[2] = zx(_mm_loadl_epi64(dst.cast())); // movq xm2, [dstq+strideq*0]
    m[2] = zx(movhps_ld(lo(m[2]), at(dst, stride).cast())); // movhps xm2, [dstq+strideq*1]
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[1] = _mm256_mulhrs_epi16(m[1], m[0]); // pmulhrsw m1, m0
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1
    m[3] = zx(_mm_loadl_epi64(r6.cast())); // movq xm3, [r6  +strideq*0]
    m[3] = zx(movhps_ld(lo(m[3]), at(r6, stride).cast())); // movhps xm3, [r6  +strideq*1]
    m[4] = zx(lo(pw(0x3ff))); // vpbroadcastd xm4, [pixel_10bpc_max]
    m[5] = _mm256_packs_epi32(m[5], m[5]); // packssdw m5, m5 ; pw_2048
    m[0] = _mm256_mulhrs_epi16(m[0], m[5]); // pmulhrsw m0, m5
    m[5] = _mm256_setzero_si256(); // pxor m5, m5
    _mm256_storeu_si256(c.cast(), m[5]); // mova [cq+32*0], m5
    _mm256_storeu_si256(at(c, 32).cast(), m[5]); // mova [cq+32*1], m5
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
    m[0] = zx(_mm_max_epi16(lo(m[0]), lo(m[5]))); // pmaxsw xm0, xm5
    m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[5]))); // pmaxsw xm1, xm5
    m[0] = zx(_mm_min_epi16(lo(m[0]), lo(m[4]))); // pminsw xm0, xm4
    m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[4]))); // pminsw xm1, xm4
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    _mm_storel_epi64(r6.cast(), lo(m[1])); // movq [r6  +strideq*0], xm1
    movhps_st(at(r6, stride).cast(), lo(m[1])); // movhps [r6  +strideq*1], xm1
}

// --- 4x4, 12 bpc --------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X4_FN type1, type2, 12` (`INV_TXFM_FN type1, type2, 0, 4x4, 12` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x4_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = zx(lo(pw(0x7000))); // vpbroadcastd xm2, [dconly_12bpc]
        return inv_txfm_add_dct_dct_4x4_10bpc_dconly(m, dst, stride, c, eob); // jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly
    }
    match t1 {
        K::Dct => idct_4x4_internal_12bpc_pass1(m, c),
        K::Adst => iadst_4x4_internal_12bpc_pass1(m, c),
        K::FlipAdst => iflipadst_4x4_internal_12bpc_pass1(m, c),
        K::Identity => iidentity_4x4_internal_12bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x4_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_4x4_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x4_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x4_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_4x4_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_4x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    idct_4x4_internal_10bpc_main(m, c); // call m(idct_4x4_internal_10bpc).main
    m[3] = ld_tab256(&idct4_12_shuf); // mova m3, [idct4_12_shuf]
    m[4] = ld_tab256(&idct4_12_shuf2); // mova m4, [idct4_12_shuf2]
    m[2] = _mm256_permutevar8x32_epi32(m[1], m[4]); // vpermd m2, m4, m1
    m[1] = _mm256_permutevar8x32_epi32(m[0], m[3]); // vpermd m1, m3, m0
    iadst_4x4_internal_12bpc_pass1_end2(m) // jmp m(iadst_4x4_internal_12bpc).pass1_end2
}

/// `.pass2` of `idct_4x4_internal_12bpc`
#[inline(always)]
unsafe fn idct_4x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
    idct_4x4_internal_10bpc_main2(m); // call m(idct_4x4_internal_10bpc).main2
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    iadst_4x4_internal_12bpc_end(m, dst, stride, c) // jmp m(iadst_4x4_internal_12bpc).end
}

/// `cglobal iadst_4x4_internal_12bpc` up to `.pass1_end`
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x4_internal_10bpc_main(m, c); // call m(iadst_4x4_internal_10bpc).main
    m[1] = _mm256_inserti128_si256::<1>(m[4], lo(m[6])); // vinserti128 m1, m4, xm6, 1
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[3])); // vinserti128 m2, xm3, 1
    iadst_4x4_internal_12bpc_pass1_end(m)
}

/// `.pass1_end` of `iadst_4x4_internal_12bpc` (falls through into `.pass1_end2`)
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_pass1_end(m: &mut M) {
    m[3] = ld_tab256(&itx4_shuf); // mova m3, [itx4_shuf]
    m[5] = pd(1024); // vpbroadcastd m5, [pd_1024]
    m[1] = _mm256_srai_epi32::<1>(m[1]); // psrad m1, 1
    m[2] = _mm256_srai_epi32::<1>(m[2]); // psrad m2, 1
    m[1] = _mm256_permutevar8x32_epi32(m[1], m[3]); // vpermd m1, m3, m1
    m[2] = _mm256_permutevar8x32_epi32(m[2], m[3]); // vpermd m2, m3, m2
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[2] = _mm256_add_epi32(m[2], m[5]); // paddd m2, m5
    m[1] = _mm256_srai_epi32::<11>(m[1]); // psrad m1, 11
    m[2] = _mm256_srai_epi32::<11>(m[2]); // psrad m2, 11
    iadst_4x4_internal_12bpc_pass1_end2(m)
}

/// `.pass1_end2` of `iadst_4x4_internal_12bpc` (up to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_pass1_end2(m: &mut M) {
    m[3] = pd(-0x20000); // vpbroadcastd m3, [clip_18b_min]
    m[4] = pd(0x1ffff); // vpbroadcastd m4, [clip_18b_max]
    m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
    m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
    m[0] = _mm256_max_epi32(m[0], m[3]); // pmaxsd m0, m3
    m[1] = _mm256_max_epi32(m[1], m[3]); // pmaxsd m1, m3
    m[0] = _mm256_min_epi32(m[0], m[4]); // pminsd m0, m4
    m[1] = _mm256_min_epi32(m[1], m[4]); // pminsd m1, m4
}

/// `.pass2` of `iadst_4x4_internal_12bpc` (falls through into `.pass2_end`)
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x4_internal_12bpc_main_pass2(m); // call .main_pass2
    m[0] = _mm256_inserti128_si256::<1>(m[4], lo(m[6])); // vinserti128 m0, m4, xm6, 1
    m[1] = _mm256_inserti128_si256::<1>(m[2], lo(m[3])); // vinserti128 m1, m2, xm3, 1
    iadst_4x4_internal_12bpc_pass2_end(m, dst, stride, c)
}

/// `.pass2_end` of `iadst_4x4_internal_12bpc` (falls through into `.end`)
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_pass2_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[0] = _mm256_srai_epi32::<12>(m[0]); // psrad m0, 12
    m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
    iadst_4x4_internal_12bpc_end(m, dst, stride, c)
}

/// `.end` and `.end2` of `iadst_4x4_internal_12bpc` (`WIN64_RESTORE_XMM_INTERNAL` has no code here)
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    // .end2:
    m[4] = pw(16384); // vpbroadcastd m4, [pw_16384]
    m[2] = zx(_mm_loadl_epi64(dst.cast())); // movq xm2, [dstq+strideq*0]
    m[3] = zx(_mm_loadl_epi64(at(dst, stride).cast())); // movq xm3, [dstq+strideq*1]
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[2] = zx(movhps_ld(lo(m[2]), r6.cast())); // movhps xm2, [r6  +strideq*0] ; dst0 dst2
    m[3] = zx(movhps_ld(lo(m[3]), at(r6, stride).cast())); // movhps xm3, [r6  +strideq*1] ; dst1 dst3
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[3])); // vinserti128 m2, xm3, 1
    m[0] = _mm256_srai_epi32::<3>(m[0]); // psrad m0, 3
    m[1] = _mm256_srai_epi32::<3>(m[1]); // psrad m1, 3
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1 ; t0 t2 t1 t3
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    _mm256_storeu_si256(c.cast(), m[4]); // mova [cq+32*0], m4
    _mm256_storeu_si256(at(c, 32).cast(), m[4]); // mova [cq+32*1], m4
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2 ; out0 out2 out1 out3
    m[0] = _mm256_max_epi16(m[0], m[4]); // pmaxsw m0, m4
    m[0] = _mm256_min_epi16(m[0], m[5]); // pminsw m0, m5
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1 ; out1 out3
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    _mm_storel_epi64(at(dst, stride).cast(), lo(m[1])); // movq [dstq+strideq*1], xm1
    movhps_st(r6.cast(), lo(m[0])); // movhps [r6  +strideq*0], xm0
    movhps_st(at(r6, stride).cast(), lo(m[1])); // movhps [r6  +strideq*1], xm1
}

/// `.main_pass2` of `iadst_4x4_internal_12bpc` (jumps to `iadst_4x4_internal_10bpc`'s `.main2`)
#[inline(always)]
unsafe fn iadst_4x4_internal_12bpc_main_pass2(m: &mut M) {
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1
    m[2] = zx(lo(m[1])); // mova xm2, xm1
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    iadst_4x4_internal_10bpc_main2(m) // jmp m(iadst_4x4_internal_10bpc).main2
}

/// `cglobal iflipadst_4x4_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x4_internal_10bpc_main(m, c); // call m(iadst_4x4_internal_10bpc).main
    m[1] = _mm256_inserti128_si256::<1>(m[3], lo(m[2])); // vinserti128 m1, m3, xm2, 1
    m[2] = _mm256_inserti128_si256::<1>(m[6], lo(m[4])); // vinserti128 m2, m6, xm4, 1
    iadst_4x4_internal_12bpc_pass1_end(m) // jmp m(iadst_4x4_internal_12bpc).pass1_end
}

/// `.pass2` of `iflipadst_4x4_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_4x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x4_internal_12bpc_main_pass2(m); // call m(iadst_4x4_internal_12bpc).main_pass2
    m[0] = _mm256_inserti128_si256::<1>(m[3], lo(m[2])); // vinserti128 m0, m3, xm2, 1
    m[1] = _mm256_inserti128_si256::<1>(m[6], lo(m[4])); // vinserti128 m1, m6, xm4, 1
    iadst_4x4_internal_12bpc_pass2_end(m, dst, stride, c) // jmp m(iadst_4x4_internal_12bpc).pass2_end
}

/// `cglobal iidentity_4x4_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    m[2] = ld_tab256(&itx4_shuf); // mova m2, [itx4_shuf]
    m[3] = pd(1697); // vpbroadcastd m3, [pd_1697]
    m[0] = _mm256_permutevar8x32_epi32(_mm256_loadu_si256(c.cast()), m[2]); // vpermd m0, m2, [cq+32*0]
    m[2] = _mm256_permutevar8x32_epi32(_mm256_loadu_si256(at(c, 32).cast()), m[2]); // vpermd m2, m2, [cq+32*1]
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[1] = _mm256_mullo_epi32(m[3], m[0]); // pmulld m1, m3, m0
    m[3] = _mm256_mullo_epi32(m[3], m[2]); // pmulld m3, m2
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[3] = _mm256_add_epi32(m[3], m[5]); // paddd m3, m5
    m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
    m[3] = _mm256_srai_epi32::<12>(m[3]); // psrad m3, 12
    m[1] = _mm256_add_epi32(m[1], m[0]); // paddd m1, m0
    m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3
    iadst_4x4_internal_12bpc_pass1_end2(m) // jmp m(iadst_4x4_internal_12bpc).pass1_end2
}

/// `.pass2` of `iidentity_4x4_internal_12bpc`: `m0` = in0 in1, `m1` = in2 in3
#[inline(always)]
unsafe fn iidentity_4x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[3] = pd(5793); // vpbroadcastd m3, [pd_5793]
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_mullo_epi32(m[0], m[3]); // pmulld m0, m3
    m[1] = _mm256_mullo_epi32(m[1], m[3]); // pmulld m1, m3
    m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5 ; 2048
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[0] = _mm256_srai_epi32::<12>(m[0]); // psrad m0, 12
    m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
    iadst_4x4_internal_12bpc_end(m, dst, stride, c) // jmp m(iadst_4x4_internal_12bpc).end
}

// --- 4x8, 10 bpc --------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X8_FN type1, type2, 10` (`INV_TXFM_FN type1, type2, 0, 4x8, 10` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x8_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = zx(lo(pw(0x7c00))); // vpbroadcastd xm2, [dconly_10bpc]
        return inv_txfm_add_dct_dct_4x8_10bpc_dconly(m, dst, stride, c, eob);
    }
    match t1 {
        K::Dct => idct_4x8_internal_10bpc_pass1(m, c),
        K::Adst => iadst_4x8_internal_10bpc_pass1(m, c),
        K::FlipAdst => iflipadst_4x8_internal_10bpc_pass1(m, c),
        K::Identity => iidentity_4x8_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x8_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_4x8_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x8_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x8_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `cglobal idct_4x8_internal_10bpc` (its `.pass1`, which `idct_4x8_internal_12bpc` jumps to) up to `jmp tx2q`
#[inline(always)]
unsafe fn idct_4x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[3] = pd(2896); // vpbroadcastd m3, [pd_2896]
    m[0] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(c.cast())); // pmulld m0, m3, [cq+32*0]
    m[1] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(at(c, 32).cast())); // pmulld m1, m3, [cq+32*1]
    m[2] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m2, m3, [cq+32*2]
    m[3] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulld m3, m3, [cq+32*3]
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m1, m2, m3
    }
    idct4_1d(m, [0, 1, 2, 3], [4, 5, 6], 7); // IDCT4_1D 0, 1, 2, 3, 4, 5, 6, 7
}

/// `.pass2` of `idct_4x8_internal_10bpc`
#[inline(always)]
unsafe fn idct_4x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_packs_epi32(m[0], m[2]); // packssdw m0, m2
    m[1] = _mm256_packs_epi32(m[1], m[3]); // packssdw m1, m3
                                           // lea r6, [deint_shuf+128] (the 8 bpc constants' base)
    m[2] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m2, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2 ; 2 3
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2 ; 0 1
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1 ; 4 5
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1 ; 6 7
    idct_4x8_internal_8bpc_main(m); // call m(idct_4x8_internal_8bpc).main
    m[4] = zx(lo(pw(2048))); // vpbroadcastd xm4, [pw_2048]
    for x in [0, 1, 2, 3] {
        m[x] = zx(_mm_mulhrs_epi16(lo(m[x]), lo(m[4]))); // REPX {pmulhrsw x, xm4}, xm0, xm1, xm2, xm3
    }
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    m[4] = zx(_mm_loadl_epi64(dst.cast())); // movq xm4, [dstq+strideq*0]
    m[4] = zx(movhps_ld(lo(m[4]), at(dst, stride).cast())); // movhps xm4, [dstq+strideq*1]
    m[5] = zx(_mm_loadl_epi64(at(dst, r3).cast())); // movq xm5, [dstq+r3       ]
    m[5] = zx(movhps_ld(lo(m[5]), at(dst, stride * 2).cast())); // movhps xm5, [dstq+strideq*2]
    m[6] = zx(_mm_loadl_epi64(r6.cast())); // movq xm6, [r6  +strideq*0]
    m[6] = zx(movhps_ld(lo(m[6]), at(r6, stride).cast())); // movhps xm6, [r6  +strideq*1]
    m[7] = zx(_mm_loadl_epi64(at(r6, r3).cast())); // movq xm7, [r6  +r3       ]
    m[7] = zx(movhps_ld(lo(m[7]), at(r6, stride * 2).cast())); // movhps xm7, [r6  +strideq*2]
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); // paddw xm0, xm4 ; 0 1
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[5]))); // paddw xm1, xm5 ; 3 2
    m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[6]))); // paddw xm2, xm6 ; 4 5
    m[3] = zx(_mm_add_epi16(lo(m[3]), lo(m[7]))); // paddw xm3, xm7 ; 7 6
    m[5] = zx(lo(pw(0x3ff))); // vpbroadcastd xm5, [pixel_10bpc_max]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in 0..4 {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    for x in [0, 1, 2, 3] {
        m[x] = zx(_mm_max_epi16(lo(m[x]), lo(m[4]))); // REPX {pmaxsw x, xm4}, xm0, xm1, xm2, xm3
    }
    for x in [0, 1, 2, 3] {
        m[x] = zx(_mm_min_epi16(lo(m[x]), lo(m[5]))); // REPX {pminsw x, xm5}, xm0, xm1, xm2, xm3
    }
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    movhps_st(at(dst, stride * 2).cast(), lo(m[1])); // movhps [dstq+strideq*2], xm1
    _mm_storel_epi64(at(dst, r3).cast(), lo(m[1])); // movq [dstq+r3       ], xm1
    _mm_storel_epi64(r6.cast(), lo(m[2])); // movq [r6  +strideq*0], xm2
    movhps_st(at(r6, stride).cast(), lo(m[2])); // movhps [r6  +strideq*1], xm2
    movhps_st(at(r6, stride * 2).cast(), lo(m[3])); // movhps [r6  +strideq*2], xm3
    _mm_storel_epi64(at(r6, r3).cast(), lo(m[3])); // movq [r6  +r3       ], xm3
}

/// `cglobal iadst_4x8_internal_10bpc` up to `.pass1_end`
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_8x4_internal_10bpc_main(m, c); // call m(iadst_8x4_internal_10bpc).main
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_add_epi32(m[5], m[4]); // paddd m0, m5, m4
    m[1] = _mm256_add_epi32(m[5], m[6]); // paddd m1, m5, m6
    m[2] = _mm256_add_epi32(m[2], m[5]); // paddd m2, m5
    m[3] = _mm256_add_epi32(m[3], m[5]); // paddd m3, m5
    iadst_4x8_internal_10bpc_pass1_end(m)
}

/// `.pass1_end` of `iadst_4x8_internal_10bpc` (up to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_pass1_end(m: &mut M) {
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m1, m2, m3
    }
}

/// `.pass2` of `iadst_4x8_internal_10bpc` (with its `.end`)
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x8_internal_10bpc_pass2_main(m); // call .pass2_main
    m[4] = zx(ld_tab128(&pw_2048_m2048)); // mova xm4, [pw_2048_m2048]
    for x in [0, 1, 2, 3] {
        m[x] = zx(_mm_mulhrs_epi16(lo(m[x]), lo(m[4]))); // REPX {pmulhrsw x, xm4}, xm0, xm1, xm2, xm3
    }
    // .end:
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    m[4] = zx(_mm_loadl_epi64(dst.cast())); // movq xm4, [dstq+strideq*0]
    m[4] = zx(movhps_ld(lo(m[4]), at(dst, stride).cast())); // movhps xm4, [dstq+strideq*1]
    m[5] = zx(_mm_loadl_epi64(at(dst, stride * 2).cast())); // movq xm5, [dstq+strideq*2]
    m[5] = zx(movhps_ld(lo(m[5]), at(dst, r3).cast())); // movhps xm5, [dstq+r3       ]
    m[6] = zx(_mm_loadl_epi64(r6.cast())); // movq xm6, [r6  +strideq*0]
    m[6] = zx(movhps_ld(lo(m[6]), at(r6, stride).cast())); // movhps xm6, [r6  +strideq*1]
    m[7] = zx(_mm_loadl_epi64(at(r6, stride * 2).cast())); // movq xm7, [r6  +strideq*2]
    m[7] = zx(movhps_ld(lo(m[7]), at(r6, r3).cast())); // movhps xm7, [r6  +r3       ]
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); // paddw xm0, xm4 ; 0 1
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[5]))); // paddw xm1, xm5 ; 2 3
    m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[6]))); // paddw xm2, xm6 ; 4 5
    m[3] = zx(_mm_add_epi16(lo(m[3]), lo(m[7]))); // paddw xm3, xm7 ; 6 7
    m[5] = zx(lo(pw(0x3ff))); // vpbroadcastd xm5, [pixel_10bpc_max]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in 0..4 {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    for x in [0, 1, 2, 3] {
        m[x] = zx(_mm_max_epi16(lo(m[x]), lo(m[4]))); // REPX {pmaxsw x, xm4}, xm0, xm1, xm2, xm3
    }
    for x in [0, 1, 2, 3] {
        m[x] = zx(_mm_min_epi16(lo(m[x]), lo(m[5]))); // REPX {pminsw x, xm5}, xm0, xm1, xm2, xm3
    }
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    _mm_storel_epi64(at(dst, stride * 2).cast(), lo(m[1])); // movq [dstq+strideq*2], xm1
    movhps_st(at(dst, r3).cast(), lo(m[1])); // movhps [dstq+r3       ], xm1
    _mm_storel_epi64(r6.cast(), lo(m[2])); // movq [r6  +strideq*0], xm2
    movhps_st(at(r6, stride).cast(), lo(m[2])); // movhps [r6  +strideq*1], xm2
    _mm_storel_epi64(at(r6, stride * 2).cast(), lo(m[3])); // movq [r6  +strideq*2], xm3
    movhps_st(at(r6, r3).cast(), lo(m[3])); // movhps [r6  +r3       ], xm3
}

/// `.pass2_main` of `iadst_4x8_internal_10bpc` (jumps to `iadst_4x8_internal_8bpc`'s `.main_pass2`)
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_pass2_main(m: &mut M) {
    m[0] = _mm256_packs_epi32(m[0], m[2]); // packssdw m0, m2
    m[1] = _mm256_packs_epi32(m[1], m[3]); // packssdw m1, m3
                                           // lea r6, [deint_shuf+128] (the 8 bpc constants' base)
    m[4] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m4, m0, m1
    m[0] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m0, m1
    m[5] = _mm256_unpackhi_epi32(m[4], m[0]); // punpckhdq m5, m4, m0
    m[4] = _mm256_unpacklo_epi32(m[4], m[0]); // punpckldq m4, m0
    m[2] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm2, m4, 1 ; 4 5
    m[3] = zx(_mm256_extracti128_si256::<1>(m[5])); // vextracti128 xm3, m5, 1 ; 6 7
    m[4] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[4]))); // pshufd xm4, xm4, q1032 ; 1 0
    m[5] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[5]))); // pshufd xm5, xm5, q1032 ; 3 2
    iadst_4x8_internal_8bpc_main_pass2(m) // jmp m(iadst_4x8_internal_8bpc).main_pass2
}

/// `.main` of `iadst_4x8_internal_10bpc` (falls through into `.main2`): loads `m8`/`m9` with the 18-bit clip
/// bounds, then as `iadst_4x8_internal_10bpc_main2`.
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_main(m: &mut M, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    iadst_4x8_internal_10bpc_main2(m, c)
}

/// `.main2` of `iadst_4x8_internal_10bpc` (falls through into `.main3`): expects the clip bounds in `m8`/`m9`;
/// reads the eight 16-byte rows at `c` (`[cq+16*0..7]`), scales them by 2896 (rect2), sets `m6` = `pd_2896`
/// and `m7` = `pd_2048` and runs `.main3`: outputs `m0` (out0 out7), `m4` (out6 out1), `m2` (out4 -out5),
/// `m5` (-out3 out2), `m6` = `pw_2048_m2048`.
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_main2(m: &mut M, c: Coef) {
    m[0] = _mm256_broadcastsi128_si256(_mm_loadu_si128(c.cast())); // vbroadcasti128 m0, [cq+16*0]
    m[2] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 2).cast())); // vbroadcasti128 m2, [cq+16*2]
    m[3] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 5).cast())); // vbroadcasti128 m3, [cq+16*5]
    m[1] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 7).cast())); // vbroadcasti128 m1, [cq+16*7]
    m[6] = pd(2896); // vpbroadcastd m6, [pd_2896]
    m[0] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[0]),
        _mm256_castsi256_pd(m[2]),
    )); // shufpd m0, m2, 0x0c ; 0 2
    m[1] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[1]),
        _mm256_castsi256_pd(m[3]),
    )); // shufpd m1, m3, 0x0c ; 7 5
    m[2] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 4).cast())); // vbroadcasti128 m2, [cq+16*4]
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 6).cast())); // vbroadcasti128 m4, [cq+16*6]
    m[5] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16).cast())); // vbroadcasti128 m5, [cq+16*1]
    m[3] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 3).cast())); // vbroadcasti128 m3, [cq+16*3]
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    m[2] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[2]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m2, m4, 0x0c ; 4 6
    m[3] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[3]),
        _mm256_castsi256_pd(m[5]),
    )); // shufpd m3, m5, 0x0c ; 3 1
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mullo_epi32(m[x], m[6]); // REPX {pmulld x, m6}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd  x, m7}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad  x, 12}, m0, m1, m2, m3
    }
    iadst_4x8_internal_10bpc_main3(m)
}

/// `.main3` of `iadst_4x8_internal_10bpc`: the packed 8-point ADST of `m0` (0 2), `m1` (7 5), `m2` (4 6), `m3`
/// (3 1) with the clip bounds in `m8`/`m9` and `pd_2048` in `m7`; outputs `m0` (out0 out7), `m4` (out6 out1),
/// `m2` (out4 -out5), `m5` (-out3 out2), `m6` = `pw_2048_m2048` (the signs).
#[inline(always)]
unsafe fn iadst_4x8_internal_10bpc_main3(m: &mut M) {
    itx_mulsub_2d(m, 1, 0, 4, 5, 6, Some(7), Q(401, 1931), Q(4076, 3612), 1); // ITX_MULSUB_2D 1, 0, 4, 5, 6, 7, 401_1931, 4076_3612, 1
    itx_mulsub_2d(m, 3, 2, 4, 5, 6, Some(7), Q(3166, 3920), Q(2598, 1189), 1); // ITX_MULSUB_2D 3, 2, 4, 5, 6, 7, 3166_3920, 2598_1189, 1
    m[4] = _mm256_sub_epi32(m[0], m[2]); // psubd m4, m0, m2 ; t4  t6
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2 ; t0  t2
    m[2] = _mm256_sub_epi32(m[1], m[3]); // psubd m2, m1, m3 ; t5  t7
    m[1] = _mm256_add_epi32(m[1], m[3]); // paddd m1, m3 ; t1  t3
    for x in [4, 2, 0, 1] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m4, m2, m0, m1
    }
    for x in [4, 2, 0, 1] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m4, m2, m0, m1
    }
    m[5] = _mm256_setzero_si256(); // pxor m5, m5
    m[5] = _mm256_sub_epi32(m[5], m[4]); // psubd m5, m4
    m[4] = _mm256_blend_epi32::<0xcc>(m[4], m[2]); // vpblendd m4, m2, 0xcc ; t4  t7
    m[2] = _mm256_blend_epi32::<0xcc>(m[2], m[5]); // vpblendd m2, m5, 0xcc ; t5 -t6
    itx_mulsub_2d(m, 4, 2, 3, 5, 6, Some(7), 1567, 3784, 0); // ITX_MULSUB_2D 4, 2, 3, 5, 6, 7, 1567, 3784
    m[5] = pd(2896); // vpbroadcastd m5, [pd_2896]
    m[6] = _mm256_broadcastsi128_si256(ld_tab128(&pw_2048_m2048)); // vbroadcasti128 m6, [pw_2048_m2048] ; + + - -
    m[3] = _mm256_unpackhi_epi64(m[0], m[1]); // punpckhqdq m3, m0, m1
    m[0] = _mm256_unpacklo_epi64(m[0], m[1]); // punpcklqdq m0, m1
    m[1] = _mm256_sub_epi32(m[0], m[3]); // psubd m1, m0, m3 ; t2  t3
    m[0] = _mm256_add_epi32(m[0], m[3]); // paddd m0, m3 ;  out0 -out7
    m[3] = _mm256_unpackhi_epi64(m[4], m[2]); // punpckhqdq m3, m4, m2 ; t7a t6a
    m[4] = _mm256_unpacklo_epi64(m[4], m[2]); // punpcklqdq m4, m2 ; t5a t4a
    m[2] = _mm256_sub_epi32(m[4], m[3]); // psubd m2, m4, m3 ; t7  t6
    m[4] = _mm256_add_epi32(m[4], m[3]); // paddd m4, m3 ;  out6 -out1
    for x in [1, 2] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m1, m2
    }
    for x in [1, 2] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m1, m2
    }
    m[3] = _mm256_blend_epi32::<0xcc>(m[1], m[2]); // vpblendd m3, m1, m2, 0xcc
    m[1] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[1]),
        _mm256_castsi256_pd(m[2]),
    )); // shufpd m1, m2, 0x05
    m[3] = _mm256_mullo_epi32(m[3], m[5]); // pmulld m3, m5
    m[5] = _mm256_mullo_epi32(m[5], m[1]); // pmulld m5, m1
    m[0] = _mm256_sign_epi32(m[0], m[6]); // psignd m0, m6 ;  out0  out7
    m[4] = _mm256_sign_epi32(m[4], m[6]); // psignd m4, m6 ;  out6  out1
    m[3] = _mm256_add_epi32(m[3], m[7]); // paddd m3, m7
    m[2] = _mm256_sub_epi32(m[3], m[5]); // psubd m2, m3, m5
    m[5] = _mm256_add_epi32(m[5], m[3]); // paddd m5, m3
    m[2] = _mm256_srai_epi32::<12>(m[2]); // psrad m2, 12 ;  out4 -out5
    m[5] = _mm256_srai_epi32::<12>(m[5]); // psrad m5, 12 ; -out3  out2
}

/// `cglobal iflipadst_4x8_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_8x4_internal_10bpc_main(m, c); // call m(iadst_8x4_internal_10bpc).main
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_add_epi32(m[5], m[3]); // paddd m0, m5, m3
    m[1] = _mm256_add_epi32(m[5], m[2]); // paddd m1, m5, m2
    m[2] = _mm256_add_epi32(m[5], m[6]); // paddd m2, m5, m6
    m[3] = _mm256_add_epi32(m[5], m[4]); // paddd m3, m5, m4
    iadst_4x8_internal_10bpc_pass1_end(m) // jmp m(iadst_4x8_internal_10bpc).pass1_end
}

/// `.pass2` of `iflipadst_4x8_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_4x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_4x8_internal_10bpc_pass2_main(m); // call m(iadst_4x8_internal_10bpc).pass2_main
    m[4] = zx(ld_tab128(&pw_2048_m2048)); // mova xm4, [pw_2048_m2048]
    for x in [3, 2, 1, 0] {
        m[x] = zx(_mm_mulhrs_epi16(lo(m[x]), lo(m[4]))); // REPX {pmulhrsw x, xm4}, xm3, xm2, xm1, xm0
    }
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    m[4] = zx(_mm_loadl_epi64(at(dst, stride).cast())); // movq xm4, [dstq+strideq*1]
    m[4] = zx(movhps_ld(lo(m[4]), dst.cast())); // movhps xm4, [dstq+strideq*0]
    m[5] = zx(_mm_loadl_epi64(at(dst, r3).cast())); // movq xm5, [dstq+r3       ]
    m[5] = zx(movhps_ld(lo(m[5]), at(dst, stride * 2).cast())); // movhps xm5, [dstq+strideq*2]
    m[6] = zx(_mm_loadl_epi64(at(r6, stride).cast())); // movq xm6, [r6  +strideq*1]
    m[6] = zx(movhps_ld(lo(m[6]), r6.cast())); // movhps xm6, [r6  +strideq*0]
    m[7] = zx(_mm_loadl_epi64(at(r6, r3).cast())); // movq xm7, [r6  +r3       ]
    m[7] = zx(movhps_ld(lo(m[7]), at(r6, stride * 2).cast())); // movhps xm7, [r6  +strideq*2]
    m[3] = zx(_mm_add_epi16(lo(m[3]), lo(m[4]))); // paddw xm3, xm4 ; 1 0
    m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[5]))); // paddw xm2, xm5 ; 3 2
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[6]))); // paddw xm1, xm6 ; 5 4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[7]))); // paddw xm0, xm7 ; 7 6
    m[5] = zx(lo(pw(0x3ff))); // vpbroadcastd xm5, [pixel_10bpc_max]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in 0..4 {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    for x in [3, 2, 1, 0] {
        m[x] = zx(_mm_max_epi16(lo(m[x]), lo(m[4]))); // REPX {pmaxsw x, xm4}, xm3, xm2, xm1, xm0
    }
    for x in [3, 2, 1, 0] {
        m[x] = zx(_mm_min_epi16(lo(m[x]), lo(m[5]))); // REPX {pminsw x, xm5}, xm3, xm2, xm1, xm0
    }
    movhps_st(dst.cast(), lo(m[3])); // movhps [dstq+strideq*0], xm3
    _mm_storel_epi64(at(dst, stride).cast(), lo(m[3])); // movq [dstq+strideq*1], xm3
    movhps_st(at(dst, stride * 2).cast(), lo(m[2])); // movhps [dstq+strideq*2], xm2
    _mm_storel_epi64(at(dst, r3).cast(), lo(m[2])); // movq [dstq+r3       ], xm2
    movhps_st(r6.cast(), lo(m[1])); // movhps [r6  +strideq*0], xm1
    _mm_storel_epi64(at(r6, stride).cast(), lo(m[1])); // movq [r6  +strideq*1], xm1
    movhps_st(at(r6, stride * 2).cast(), lo(m[0])); // movhps [r6  +strideq*2], xm0
    _mm_storel_epi64(at(r6, r3).cast(), lo(m[0])); // movq [r6  +r3       ], xm0
}

/// `cglobal iidentity_4x8_internal_10bpc` (its `.pass1`, which `iidentity_4x8_internal_12bpc` jumps to) up to
/// `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[3] = pd(2896); // vpbroadcastd m3, [pd_2896]
    m[0] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(c.cast())); // pmulld m0, m3, [cq+32*0]
    m[1] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(at(c, 32).cast())); // pmulld m1, m3, [cq+32*1]
    m[2] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m2, m3, [cq+32*2]
    m[3] = _mm256_mullo_epi32(m[3], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulld m3, [cq+32*3]
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[4] = pd(5793); // vpbroadcastd m4, [pd_5793]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[5]); // REPX {paddd  x, m5}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad  x, 12}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mullo_epi32(m[x], m[4]); // REPX {pmulld x, m4}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[5]); // REPX {paddd  x, m5}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad  x, 12}, m0, m1, m2, m3
    }
}

/// `.pass2` of `iidentity_4x8_internal_10bpc`
#[inline(always)]
unsafe fn iidentity_4x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[6] = pw(0x3ff); // vpbroadcastd m6, [pixel_10bpc_max]
    iidentity_4x8_internal_10bpc_pass2_end(m, dst, stride, c); // call .pass2_end
}

/// `.pass2_end` of `iidentity_4x8_internal_10bpc`: `m0`-`m3` the first pass's rows, `m6` the pixel maximum
#[inline(always)]
unsafe fn iidentity_4x8_internal_10bpc_pass2_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(4096); // vpbroadcastd m4, [pw_4096]
    m[0] = _mm256_packs_epi32(m[0], m[2]); // packssdw m0, m2
    m[1] = _mm256_packs_epi32(m[1], m[3]); // packssdw m1, m3
    m[2] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m2, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[2] = _mm256_mulhrs_epi16(m[2], m[4]); // pmulhrsw m2, m4
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2 ; 2 3 6 7
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2 ; 0 1 4 5
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    m[2] = zx(_mm_loadl_epi64(dst.cast())); // movq xm2, [dstq+strideq*0]
    m[2] = zx(movhps_ld(lo(m[2]), at(dst, stride).cast())); // movhps xm2, [dstq+strideq*1]
    m[4] = _mm256_broadcastq_epi64(_mm_loadl_epi64(r6.cast())); // vpbroadcastq m4, [r6  +strideq*0]
    m[5] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(r6, stride).cast())); // vpbroadcastq m5, [r6  +strideq*1]
    m[3] = zx(_mm_loadl_epi64(at(dst, stride * 2).cast())); // movq xm3, [dstq+strideq*2]
    m[3] = zx(movhps_ld(lo(m[3]), at(dst, r3).cast())); // movhps xm3, [dstq+r3       ]
    m[2] = _mm256_blend_epi32::<0x30>(m[2], m[4]); // vpblendd m2, m4, 0x30
    m[2] = _mm256_blend_epi32::<0xc0>(m[2], m[5]); // vpblendd m2, m5, 0xc0
    m[4] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(r6, stride * 2).cast())); // vpbroadcastq m4, [r6  +strideq*2]
    m[5] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(r6, r3).cast())); // vpbroadcastq m5, [r6  +r3       ]
    m[3] = _mm256_blend_epi32::<0x30>(m[3], m[4]); // vpblendd m3, m4, 0x30
    m[3] = _mm256_blend_epi32::<0xc0>(m[3], m[5]); // vpblendd m3, m5, 0xc0
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in 0..4 {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2 ; out0 out1 out4 out5
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3 ; out2 out3 out6 out7
    m[0] = _mm256_max_epi16(m[0], m[4]); // pmaxsw m0, m4
    m[1] = _mm256_max_epi16(m[1], m[4]); // pmaxsw m1, m4
    m[0] = _mm256_min_epi16(m[0], m[6]); // pminsw m0, m6
    m[1] = _mm256_min_epi16(m[1], m[6]); // pminsw m1, m6
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1 ; out4 out5
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1 ; out6 out7
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    _mm_storel_epi64(at(dst, stride * 2).cast(), lo(m[1])); // movq [dstq+strideq*2], xm1
    movhps_st(at(dst, r3).cast(), lo(m[1])); // movhps [dstq+r3       ], xm1
    _mm_storel_epi64(r6.cast(), lo(m[2])); // movq [r6  +strideq*0], xm2
    movhps_st(at(r6, stride).cast(), lo(m[2])); // movhps [r6  +strideq*1], xm2
    _mm_storel_epi64(at(r6, stride * 2).cast(), lo(m[3])); // movq [r6  +strideq*2], xm3
    movhps_st(at(r6, r3).cast(), lo(m[3])); // movhps [r6  +r3       ], xm3
}

// --- 4x8, 12 bpc --------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X8_FN type1, type2, 12` (`INV_TXFM_FN type1, type2, 0, 4x8, 12` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x8_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = zx(lo(pw(0x7000))); // vpbroadcastd xm2, [dconly_12bpc]
        return inv_txfm_add_dct_dct_4x8_10bpc_dconly(m, dst, stride, c, eob); // jmp m(inv_txfm_add_dct_dct_4x8_10bpc).dconly
    }
    match t1 {
        K::Dct => idct_4x8_internal_10bpc_pass1(m, c), // jmp m(idct_4x8_internal_10bpc).pass1
        K::Adst => iadst_4x8_internal_12bpc_pass1(m, c),
        K::FlipAdst => iflipadst_4x8_internal_12bpc_pass1(m, c),
        K::Identity => iidentity_4x8_internal_10bpc_pass1(m, c), // jmp m(iidentity_4x8_internal_10bpc).pass1
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x8_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_4x8_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x8_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x8_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `.pass2` of `idct_4x8_internal_12bpc`
#[inline(always)]
unsafe fn idct_4x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    // transpose & interleave
    m[0] = _mm256_shuffle_epi32::<0x78>(m[0]); // pshufd m0, m0, q1320
    m[1] = _mm256_shuffle_epi32::<0x78>(m[1]); // pshufd m1, m1, q1320
    m[2] = _mm256_shuffle_epi32::<0x78>(m[2]); // pshufd m2, m2, q1320
    m[3] = _mm256_shuffle_epi32::<0x78>(m[3]); // pshufd m3, m3, q1320
    m[4] = _mm256_unpacklo_epi32(m[0], m[1]); // punpckldq m4, m0, m1
    m[0] = _mm256_unpackhi_epi32(m[0], m[1]); // punpckhdq m0, m1
    m[5] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m5, m2, m3
    m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3
    m[0] = _mm256_permute4x64_epi64::<0xd2>(m[0]); // vpermq m0, m0, q3102
    m[2] = _mm256_permute4x64_epi64::<0xd2>(m[2]); // vpermq m2, m2, q3102
    m[1] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x31 ; 1 5 (interleaved)
    m[3] = _mm256_permute2x128_si256::<0x20>(m[0], m[2]); // vperm2i128 m3, m0, m2, 0x20 ; 7 3 (interleaved)
    m[0] = _mm256_permute2x128_si256::<0x20>(m[4], m[5]); // vperm2i128 m0, m4, m5, 0x20 ; 0 2 (interleaved)
    m[2] = _mm256_permute2x128_si256::<0x31>(m[4], m[5]); // vperm2i128 m2, m4, m5, 0x31 ; 4 6 (interleaved)
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    idct_8x4_internal_10bpc_main(m); // call m(idct_8x4_internal_10bpc).main
    m[3] = _mm256_sub_epi32(m[0], m[4]); // psubd m3, m0, m4 ; out7 out6
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; out0 out1
    m[1] = _mm256_add_epi32(m[2], m[5]); // paddd m1, m2, m5 ; out3 out2
    m[2] = _mm256_sub_epi32(m[2], m[5]); // psubd m2, m5 ; out4 out5
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
    m[3] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m3, m3, q1032
    iadst_4x8_internal_12bpc_end(m, dst, stride, c) // jmp m(iadst_4x8_internal_12bpc).end
}

/// `cglobal iadst_4x8_internal_12bpc` up to `.pass1_end`
#[inline(always)]
unsafe fn iadst_4x8_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iadst_8x4_internal_10bpc_main(m, c); // call m(iadst_8x4_internal_10bpc).main
    m[0] = _mm256_srai_epi32::<1>(m[4]); // psrad m0, m4, 1
    m[1] = _mm256_srai_epi32::<1>(m[6]); // psrad m1, m6, 1
    m[2] = _mm256_srai_epi32::<1>(m[2]); // psrad m2, 1
    m[3] = _mm256_srai_epi32::<1>(m[3]); // psrad m3, 1
    iadst_4x8_internal_12bpc_pass1_end(m)
}

/// `.pass1_end` of `iadst_4x8_internal_12bpc` (up to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_4x8_internal_12bpc_pass1_end(m: &mut M) {
    m[5] = pd(1024); // vpbroadcastd m5, [pd_1024]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[5]); // REPX {paddd x, m5}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<11>(m[x]); // REPX {psrad x, 11}, m0, m1, m2, m3
    }
}

/// `.pass2` of `iadst_4x8_internal_12bpc` (falls through into `.end`)
#[inline(always)]
unsafe fn iadst_4x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    iadst_4x8_internal_12bpc_pass2_main(m); // call .pass2_main
    m[3] = _mm256_blend_epi32::<0x33>(m[0], m[4]); // vpblendd m3, m0, m4, 0x33 ; out6 out7
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[4]); // vpblendd m0, m4, 0xcc ; out0 out1
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[5]); // pshufd m1, m5, q1032
    m[2] = _mm256_sign_epi32(m[2], m[6]); // psignd m2, m6 ; out4 out5
    m[1] = _mm256_sign_epi32(m[1], m[6]); // psignd m1, m6 ; out2 out3
    iadst_4x8_internal_12bpc_end(m, dst, stride, c)
}

/// `.end` of `iadst_4x8_internal_12bpc`: `m0`-`m3` the 32-bit outputs (0 1, 2 3, 4 5, 6 7, interleaved)
#[inline(always)]
unsafe fn iadst_4x8_internal_12bpc_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(16384); // vpbroadcastd m4, [pw_16384]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<3>(m[x]); // REPX {psrad x, 3}, m0, m1, m2, m3
    }
    m[0] = _mm256_packs_epi32(m[0], m[2]); // packssdw m0, m2 ; 0 1 4 5 (interleaved)
    m[1] = _mm256_packs_epi32(m[1], m[3]); // packssdw m1, m3 ; 2 3 6 7 (interleaved)
    m[2] = ld_tab256(&iadst8_12_shuf); // mova m2, [iadst8_12_shuf]
    m[0] = _mm256_permutevar8x32_epi32(m[0], m[2]); // vpermd m0, m2, m0 ; 0 1 4 5
    m[1] = _mm256_permutevar8x32_epi32(m[1], m[2]); // vpermd m1, m2, m1 ; 2 3 6 7
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    m[4] = zx(_mm_loadl_epi64(dst.cast())); // movq xm4, [dstq+strideq*0]
    m[4] = zx(movhps_ld(lo(m[4]), at(dst, stride).cast())); // movhps xm4, [dstq+strideq*1]
    m[5] = zx(_mm_loadl_epi64(at(dst, stride * 2).cast())); // movq xm5, [dstq+strideq*2]
    m[5] = zx(movhps_ld(lo(m[5]), at(dst, r3).cast())); // movhps xm5, [dstq+r3       ]
    m[6] = zx(_mm_loadl_epi64(r6.cast())); // movq xm6, [r6  +strideq*0]
    m[6] = zx(movhps_ld(lo(m[6]), at(r6, stride).cast())); // movhps xm6, [r6  +strideq*1]
    m[4] = _mm256_inserti128_si256::<1>(m[4], lo(m[6])); // vinserti128 m4, xm6, 1
    m[7] = zx(_mm_loadl_epi64(at(r6, stride * 2).cast())); // movq xm7, [r6  +strideq*2]
    m[7] = zx(movhps_ld(lo(m[7]), at(r6, r3).cast())); // movhps xm7, [r6  +r3       ]
    m[5] = _mm256_inserti128_si256::<1>(m[5], lo(m[7])); // vinserti128 m5, xm7, 1
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4 ; 0 1 4 5
    m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5 ; 2 3 6 7
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in 0..4 {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    for x in [0, 1] {
        m[x] = _mm256_max_epi16(m[x], m[4]); // REPX {pmaxsw x,  m4}, m0, m1
    }
    for x in [0, 1] {
        m[x] = _mm256_min_epi16(m[x], m[5]); // REPX {pminsw x,  m5}, m0, m1
    }
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1 ; out4 out5
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1 ; out6 out7
    _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
    movhps_st(at(dst, stride).cast(), lo(m[0])); // movhps [dstq+strideq*1], xm0
    _mm_storel_epi64(at(dst, stride * 2).cast(), lo(m[1])); // movq [dstq+strideq*2], xm1
    movhps_st(at(dst, r3).cast(), lo(m[1])); // movhps [dstq+r3       ], xm1
    _mm_storel_epi64(r6.cast(), lo(m[2])); // movq [r6  +strideq*0], xm2
    movhps_st(at(r6, stride).cast(), lo(m[2])); // movhps [r6  +strideq*1], xm2
    _mm_storel_epi64(at(r6, stride * 2).cast(), lo(m[3])); // movq [r6  +strideq*2], xm3
    movhps_st(at(r6, r3).cast(), lo(m[3])); // movhps [r6  +r3       ], xm3
}

/// `.pass2_main` of `iadst_4x8_internal_12bpc` (jumps to `iadst_4x8_internal_10bpc`'s `.main3`)
#[inline(always)]
unsafe fn iadst_4x8_internal_12bpc_pass2_main(m: &mut M) {
    // transpose & interleave
    m[0] = _mm256_shuffle_epi32::<0x78>(m[0]); // pshufd m0, m0, q1320
    m[1] = _mm256_shuffle_epi32::<0x78>(m[1]); // pshufd m1, m1, q1320
    m[2] = _mm256_shuffle_epi32::<0x78>(m[2]); // pshufd m2, m2, q1320
    m[3] = _mm256_shuffle_epi32::<0x78>(m[3]); // pshufd m3, m3, q1320
    m[4] = _mm256_unpacklo_epi32(m[0], m[1]); // punpckldq m4, m0, m1
    m[0] = _mm256_unpackhi_epi32(m[0], m[1]); // punpckhdq m0, m1
    m[5] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m5, m2, m3
    m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3
    m[1] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x31 ; 7 5 (interleaved)
    m[3] = _mm256_permute2x128_si256::<0x20>(m[0], m[2]); // vperm2i128 m3, m0, m2, 0x20 ; 3 1 (interleaved)
    m[0] = _mm256_permute2x128_si256::<0x20>(m[4], m[5]); // vperm2i128 m0, m4, m5, 0x20 ; 0 2 (interleaved)
    m[2] = _mm256_permute2x128_si256::<0x31>(m[4], m[5]); // vperm2i128 m2, m4, m5, 0x31 ; 4 6 (interleaved)
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    iadst_4x8_internal_10bpc_main3(m) // jmp m(iadst_4x8_internal_10bpc).main3
}

/// `cglobal iflipadst_4x8_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x8_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iadst_8x4_internal_10bpc_main(m, c); // call m(iadst_8x4_internal_10bpc).main
    m[0] = _mm256_srai_epi32::<1>(m[3]); // psrad m0, m3, 1
    m[1] = _mm256_srai_epi32::<1>(m[2]); // psrad m1, m2, 1
    m[2] = _mm256_srai_epi32::<1>(m[6]); // psrad m2, m6, 1
    m[3] = _mm256_srai_epi32::<1>(m[4]); // psrad m3, m4, 1
    iadst_4x8_internal_12bpc_pass1_end(m) // jmp m(iadst_4x8_internal_12bpc).pass1_end
}

/// `.pass2` of `iflipadst_4x8_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_4x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    iadst_4x8_internal_12bpc_pass2_main(m); // call m(iadst_4x8_internal_12bpc).pass2_main
    m[3] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[4]),
        _mm256_castsi256_pd(m[0]),
    )); // shufpd m3, m4, m0, 0x05 ; out1 out0
    m[0] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[0]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m0, m4, 0x05 ; out7 out6
    m[2] = _mm256_sign_epi32(m[2], m[6]); // psignd m2, m6
    m[6] = _mm256_shuffle_epi32::<0x4e>(m[6]); // pshufd m6, m6, q1032
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m1, m2, q1032 ; out5 out4
    m[2] = _mm256_sign_epi32(m[5], m[6]); // psignd m2, m5, m6 ; out3 out2
    iadst_4x8_internal_12bpc_end(m, dst, stride, c) // jmp m(iadst_4x8_internal_12bpc).end
}

/// `.pass2` of `iidentity_4x8_internal_12bpc`: `m0` = in0 in1, `m1` = in2 in3, `m2` = in4 in5, `m3` = in6 in7
#[inline(always)]
unsafe fn iidentity_4x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[6] = pw(0xfff); // vpbroadcastd m6, [pixel_12bpc_max]
    iidentity_4x8_internal_10bpc_pass2_end(m, dst, stride, c); // call m(iidentity_4x8_internal_10bpc).pass2_end
}

// --- entry points -------------------------------------------------------------------------------------------

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x4_10bpc_avx2,
    inv_txfm_add_4x4_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x4_12bpc_avx2,
    inv_txfm_add_4x4_12bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x8_10bpc_avx2,
    inv_txfm_add_4x8_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x8_12bpc_avx2,
    inv_txfm_add_4x8_12bpc,
    K::Identity,
    K::Identity
);

// --- part 2: itx16_avx2.asm lines 1217-2261 (4x16, 8x4) ---

// --- 4x16 ---------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_4X16_FN type1, type2, 10` (`INV_TXFM_FN type1, type2, 0, 4x16, 10` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x16_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[2] = zx(lo(pw(0x7c00))); // vpbroadcastd xm2, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 16; // or r3d, 16
        r6d = r6d.wrapping_add(384); // add r6d, 384
        r6d >>= 9; // sar r6d, 9
        return inv_txfm_add_dct_dct_4x4_10bpc_dconly3(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly3
    }
    match t1 {
        K::Dct => idct_4x16_internal_10bpc_pass1(m, c),
        K::Adst => iadst_4x16_internal_10bpc_pass1(m, c),
        K::FlipAdst => iflipadst_4x16_internal_10bpc_pass1(m, c),
        K::Identity => iidentity_4x16_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x16_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_4x16_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x16_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x16_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro INV_TXFM_4X16_FN type1, type2, 12` (`INV_TXFM_FN type1, type2, 0, 4x16, 12` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_4x16_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[2] = zx(lo(pw(0x7000))); // vpbroadcastd xm2, [dconly_12bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 16; // or r3d, 16
        r6d = r6d.wrapping_add(384); // add r6d, 384
        r6d >>= 9; // sar r6d, 9
        return inv_txfm_add_dct_dct_4x4_10bpc_dconly3(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_4x4_10bpc).dconly3
    }
    match t1 {
        K::Dct => idct_4x16_internal_12bpc_pass1(m, c),
        K::Adst => iadst_4x16_internal_12bpc_pass1(m, c),
        K::FlipAdst => iflipadst_4x16_internal_12bpc_pass1(m, c),
        K::Identity => iidentity_4x16_internal_12bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_4x16_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_4x16_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_4x16_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_4x16_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `.pass1` of `idct_4x16_internal_10bpc` (the function's start) up to `jmp tx2q`; also the first pass of
/// `idct_4x16_internal_12bpc`
#[inline(always)]
unsafe fn idct_4x16_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[10] = pd(3072); // vpbroadcastd m10, [pd_3072]
    m[1] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m1, [cq+32*2]
    m[3] = _mm256_loadu_si256(at(c, 32 * 6).cast()); // mova m3, [cq+32*6]
    m[5] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m5, [cq+32*3]
    m[7] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m7, [cq+32*7]
    idct_4x16_internal_10bpc_pass1_main(m); // call .pass1_main
    m[0] = _mm256_mullo_epi32(m[6], _mm256_loadu_si256(c.cast())); // pmulld m0, m6, [cq+32*0]
    m[2] = _mm256_mullo_epi32(m[6], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmulld m2, m6, [cq+32*4]
    m[4] = _mm256_mullo_epi32(m[6], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulld m4, m6, [cq+32*1]
    m[6] = _mm256_mullo_epi32(m[6], _mm256_loadu_si256(at(c, 32 * 5).cast())); // pmulld m6, [cq+32*5]
    idct_4x16_internal_10bpc_pass1_main2(m); // call .pass1_main2
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass2` of `idct_4x16_internal_10bpc`
#[inline(always)]
unsafe fn idct_4x16_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7

    // (no code: the 8 bpc code's o_base) // lea r6, [deint_shuf+128]
    m[4] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m4, m2, m3
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[5] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m5, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[4]); // punpckhdq m1, m0, m4 ; 2 3
    m[0] = _mm256_unpacklo_epi32(m[0], m[4]); // punpckldq m0, m4 ; 0 1
    m[4] = _mm256_unpacklo_epi32(m[5], m[2]); // punpckldq m4, m5, m2 ; 8 9
    m[5] = _mm256_unpackhi_epi32(m[5], m[2]); // punpckhdq m5, m2 ; a b
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1 ; 4 5
    m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1 ; 6 7
    m[6] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm6, m4, 1 ; c d
    m[7] = zx(_mm256_extracti128_si256::<1>(m[5])); // vextracti128 xm7, m5, 1 ; e f
    idct_4x16_internal_8bpc_main(m); // call m(idct_4x16_internal_8bpc).main
    m[9] = pw(2048); // vpbroadcastd m9, [pw_2048]
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[1])); // vinserti128 m0, m0, xm1, 1 ; 0 1   3 2
    m[1] = _mm256_inserti128_si256::<1>(m[2], lo(m[3])); // vinserti128 m1, m2, xm3, 1 ; 4 5   7 6
    m[2] = _mm256_inserti128_si256::<1>(m[4], lo(m[5])); // vinserti128 m2, m4, xm5, 1 ; 8 9   b a
    m[3] = _mm256_inserti128_si256::<1>(m[6], lo(m[7])); // vinserti128 m3, m6, xm7, 1 ; c d   f e
    m[8] = pw(0x3ff); // vpbroadcastd m8, [pixel_10bpc_max]
    idct_4x16_internal_10bpc_pass2_end(m, dst, stride, c); // call .pass2_end
}

/// `.pass1_main` of `idct_4x16_internal_10bpc`: in `m1`, `m3`, `m5`, `m7` (rows 2, 6, 3, 7); out `m1`, `m3`,
/// `m5`, `m7` (t2l, t3l, t2h, t3h), `m6` = `pd_1448`, `m4`/`m8`/`m9` clobbered.
#[inline(always)]
pub(crate) unsafe fn idct_4x16_internal_10bpc_pass1_main(m: &mut M) {
    m[4] = pd(3784); // vpbroadcastd m4, [pd_3784]
    m[8] = pd(1567); // vpbroadcastd m8, [pd_1567]
    m[9] = pd(2048); // vpbroadcastd m9, [pd_2048]
    m[6] = pd(1448); // vpbroadcastd m6, [pd_1448]
    itx_mulsub_2d(m, 1, 3, 0, 2, usize::MAX, Some(9), 8, 4, 0); // ITX_MULSUB_2D 1, 3, 0, 2, _, 9, 8, 4 ; t2l, t3l
    itx_mulsub_2d(m, 5, 7, 4, 2, usize::MAX, Some(9), 8, 4, 0); // ITX_MULSUB_2D 5, 7, 4, 2, _, 9, 8, 4 ; t2h, t3h
}

/// `.pass1_main2` of `idct_4x16_internal_10bpc`: expects `m10` = rounding (`pd_3072` or the caller's), `m0`,
/// `m2`, `m4`, `m6` the products by 1448 of rows 0, 4, 1, 5 and `m1`, `m3`, `m5`, `m7` from `.pass1_main`;
/// out `m0`-`m7` (unshifted by the final `psrad`), `m8`/`m9` clobbered.
#[inline(always)]
pub(crate) unsafe fn idct_4x16_internal_10bpc_pass1_main2(m: &mut M) {
    m[0] = _mm256_add_epi32(m[0], m[10]); // paddd m0, m10
    m[4] = _mm256_add_epi32(m[4], m[10]); // paddd m4, m10
    m[8] = _mm256_add_epi32(m[0], m[2]); // paddd m8, m0, m2
    m[0] = _mm256_sub_epi32(m[0], m[2]); // psubd m0, m2
    m[9] = _mm256_add_epi32(m[4], m[6]); // paddd m9, m4, m6
    m[4] = _mm256_sub_epi32(m[4], m[6]); // psubd m4, m6
    for x in [8, 0, 9, 4] {
        m[x] = _mm256_srai_epi32::<11>(m[x]); // REPX {psrad x, 11}, m8, m0, m9, m4 ; t0l, t1l, t0h, t1h
    }
    m[2] = _mm256_sub_epi32(m[0], m[1]); // psubd m2, m0, m1
    m[1] = _mm256_add_epi32(m[1], m[0]); // paddd m1, m0
    m[6] = _mm256_sub_epi32(m[4], m[5]); // psubd m6, m4, m5
    m[5] = _mm256_add_epi32(m[5], m[4]); // paddd m5, m4
    m[0] = _mm256_add_epi32(m[8], m[3]); // paddd m0, m8, m3
    m[3] = _mm256_sub_epi32(m[8], m[3]); // psubd m3, m8, m3
    m[4] = _mm256_add_epi32(m[9], m[7]); // paddd m4, m9, m7
    m[7] = _mm256_sub_epi32(m[9], m[7]); // psubd m7, m9, m7
}

/// `.pass2_end` of `idct_4x16_internal_10bpc`: expects `m9` = the `pmulhrsw` factor, `m8` = the pixel maximum,
/// `m0`-`m3` the four 4x4 blocks
#[inline(always)]
unsafe fn idct_4x16_internal_10bpc_pass2_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    m[0] = _mm256_mulhrs_epi16(m[0], m[9]); // pmulhrsw m0, m9
    idct_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[1], m[9]); // pmulhrsw m0, m1, m9
    idct_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[2], m[9]); // pmulhrsw m0, m2, m9
    idct_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[3], m[9]); // pmulhrsw m0, m3, m9
    idct_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
}

/// `.write_4x4` of `idct_4x16_internal_10bpc` (advances `dstq` by four rows and `cq` by 64 bytes)
#[inline(always)]
unsafe fn idct_4x16_internal_10bpc_write_4x4(m: &mut M, pdst: &mut Dst, stride: isize, r6: isize, pc: &mut Coef) {
    let (mut dst, mut c) = (*pdst, *pc);
    m[4] = zx(_mm_loadl_epi64(dst.cast())); // movq xm4, [dstq+strideq*0]
    m[4] = zx(movhps_ld(lo(m[4]), at(dst, stride).cast())); // movhps xm4, [dstq+strideq*1]
    m[5] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, stride * 2).cast())); // vpbroadcastq m5, [dstq+strideq*2]
    m[6] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, r6).cast())); // vpbroadcastq m6, [dstq+r6 ]
    _mm256_storeu_si256(c.cast(), m[7]); // mova [cq+32*0], m7
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[7]); // mova [cq+32*1], m7
    c = at(c, 32 * 2); // add cq, 32*2
    m[4] = _mm256_blend_epi32::<0xc0>(m[4], m[5]); // vpblendd m4, m5, 0xc0
    m[4] = _mm256_blend_epi32::<0x30>(m[4], m[6]); // vpblendd m4, m6, 0x30
    m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
    m[4] = _mm256_max_epi16(m[4], m[7]); // pmaxsw m4, m7
    m[4] = _mm256_min_epi16(m[4], m[8]); // pminsw m4, m8
    m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
    _mm_storel_epi64(dst.cast(), lo(m[4])); // movq [dstq+strideq*0], xm4
    movhps_st(at(dst, stride).cast(), lo(m[4])); // movhps [dstq+strideq*1], xm4
    movhps_st(at(dst, stride * 2).cast(), lo(m[5])); // movhps [dstq+strideq*2], xm5
    _mm_storel_epi64(at(dst, r6).cast(), lo(m[5])); // movq [dstq+r6 ], xm5
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    (*pdst, *pc) = (dst, c);
}

/// `cglobal iadst_4x16_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_4x16_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_16x4_internal_10bpc_main(m, c); // call m(iadst_16x4_internal_10bpc).main
    m[6] = pd(6144); // vpbroadcastd m6, [pd_6144]
    iadst_16x4_internal_10bpc_main_end(m); // call m(iadst_16x4_internal_10bpc).main_end
    m[0] = _mm256_srai_epi32::<13>(m[4]); // psrad m0, m4, 13
    m[1] = _mm256_srai_epi32::<13>(m[5]); // psrad m1, m5, 13
    m[2] = _mm256_srai_epi32::<13>(m[2]); // psrad m2, 13
    m[3] = _mm256_srai_epi32::<13>(m[3]); // psrad m3, 13
    m[4] = _mm256_srai_epi32::<13>(m[8]); // psrad m4, m8, 13
    m[5] = _mm256_srai_epi32::<13>(m[9]); // psrad m5, m9, 13
    m[6] = _mm256_srai_epi32::<13>(m[6]); // psrad m6, 13
    m[7] = _mm256_srai_epi32::<13>(m[7]); // psrad m7, 13
}

/// `.pass2` of `iadst_4x16_internal_10bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    iadst_4x16_internal_10bpc_pass2_main(m); // call .pass2_main
    m[5] = pw(2048); // vpbroadcastd m5, [pw_2048]
    m[8] = pw(0x3ff); // vpbroadcastd m8, [pixel_10bpc_max]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[4] = _mm256_blend_epi32::<0xcc>(m[3], m[0]); // vpblendd m4, m3, m0, 0xcc ; -out3   out0   out2  -out1
    m[2] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m2, m2, q1032 ; -out11  out8   out10 -out9
    m[3] = _mm256_blend_epi32::<0x33>(m[3], m[0]); // vpblendd m3, m0, 0x33 ; -out15  out12  out14 -out13
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    m[9] = _mm256_sub_epi16(m[7], m[5]); // psubw m9, m7, m5
    m[9] = _mm256_blend_epi32::<0x3c>(m[9], m[5]); // vpblendd m9, m5, 0x3c ; -2048   2048   2048  -2048
    m[0] = _mm256_mulhrs_epi16(m[4], m[9]); // pmulhrsw m0, m4, m9
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[1], m[9]); // pmulhrsw m0, m1, m9
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[2], m[9]); // pmulhrsw m0, m2, m9
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[3], m[9]); // pmulhrsw m0, m3, m9
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
}

/// `.write_4x4` of `iadst_4x16_internal_10bpc` (advances `dstq` by four rows and `cq` by 64 bytes)
#[inline(always)]
unsafe fn iadst_4x16_internal_10bpc_write_4x4(m: &mut M, pdst: &mut Dst, stride: isize, r6: isize, pc: &mut Coef) {
    let (mut dst, mut c) = (*pdst, *pc);
    m[4] = zx(_mm_loadl_epi64(at(dst, r6).cast())); // movq xm4, [dstq+r6 ]
    m[4] = zx(movhps_ld(lo(m[4]), dst.cast())); // movhps xm4, [dstq+strideq*0]
    m[5] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, stride).cast())); // vpbroadcastq m5, [dstq+strideq*1]
    m[6] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, stride * 2).cast())); // vpbroadcastq m6, [dstq+strideq*2]
    _mm256_storeu_si256(c.cast(), m[7]); // mova [cq+32*0], m7
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[7]); // mova [cq+32*1], m7
    c = at(c, 32 * 2); // add cq, 32*2
    m[4] = _mm256_blend_epi32::<0xc0>(m[4], m[5]); // vpblendd m4, m5, 0xc0
    m[4] = _mm256_blend_epi32::<0x30>(m[4], m[6]); // vpblendd m4, m6, 0x30
    m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
    m[4] = _mm256_max_epi16(m[4], m[7]); // pmaxsw m4, m7
    m[4] = _mm256_min_epi16(m[4], m[8]); // pminsw m4, m8
    m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
    movhps_st(dst.cast(), lo(m[4])); // movhps [dstq+strideq*0], xm4
    movhps_st(at(dst, stride).cast(), lo(m[5])); // movhps [dstq+strideq*1], xm5
    _mm_storel_epi64(at(dst, stride * 2).cast(), lo(m[5])); // movq [dstq+strideq*2], xm5
    _mm_storel_epi64(at(dst, r6).cast(), lo(m[4])); // movq [dstq+r6 ], xm4
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    (*pdst, *pc) = (dst, c);
}

/// `.pass2_main` of `iadst_4x16_internal_10bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_10bpc_pass2_main(m: &mut M) {
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7

    // (no code: the 8 bpc code's o_base) // lea r6, [deint_shuf+128]
    m[4] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m4, m2, m3
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[5] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m5, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[4]); // punpckhdq m1, m0, m4
    m[0] = _mm256_unpacklo_epi32(m[0], m[4]); // punpckldq m0, m4
    m[4] = _mm256_unpacklo_epi32(m[5], m[2]); // punpckldq m4, m5, m2
    m[5] = _mm256_unpackhi_epi32(m[5], m[2]); // punpckhdq m5, m2
    m[3] = _mm256_blend_epi32::<0x33>(m[0], m[1]); // vpblendd m3, m0, m1, 0x33
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[1]); // vpblendd m0, m1, 0xcc
    m[2] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[5]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m2, m5, m4, 0x05
    m[4] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[4]),
        _mm256_castsi256_pd(m[5]),
    )); // shufpd m4, m5, 0x05
    m[1] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m1, m0, m3, 0x31 ; 4 7   6 5
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m0, xm3, 1 ; 0 3   2 1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[2], m[4]); // vperm2i128 m3, m2, m4, 0x31 ; c f   e d ; ????
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[4])); // vinserti128 m2, xm4, 1 ; b 8   9 a
    iadst_4x16_internal_8bpc_main2(m); // call m(iadst_4x16_internal_8bpc).main2
    m[5] = pw(2896 * 8); // vpbroadcastd m5, [pw_2896x8]
    m[1] = _mm256_adds_epi16(m[2], m[4]); // paddsw m1, m2, m4
    m[2] = _mm256_subs_epi16(m[2], m[4]); // psubsw m2, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5 ; -out7   out4   out6  -out5
    m[2] = _mm256_mulhrs_epi16(m[2], m[5]); // pmulhrsw m2, m5 ; out8  -out11 -out9   out10
}

/// `.main` of `iadst_4x16_internal_10bpc` (falls into `.main2`): loads the 16 rows of 4 coefficients at `c`;
/// expects `m12`/`m13` = clip minimum/maximum.
#[inline(always)]
pub(crate) unsafe fn iadst_4x16_internal_10bpc_main(m: &mut M, c: Coef) {
    m[0] = _mm256_broadcastsi128_si256(_mm_loadu_si128(c.cast())); // vbroadcasti128 m0, [cq+16* 0]
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 2).cast())); // vbroadcasti128 m4, [cq+16* 2]
    m[1] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 15).cast())); // vbroadcasti128 m1, [cq+16*15]
    m[5] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 13).cast())); // vbroadcasti128 m5, [cq+16*13]
    m[2] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 4).cast())); // vbroadcasti128 m2, [cq+16* 4]
    m[6] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 6).cast())); // vbroadcasti128 m6, [cq+16* 6]
    m[3] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 11).cast())); // vbroadcasti128 m3, [cq+16*11]
    m[7] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 9).cast())); // vbroadcasti128 m7, [cq+16* 9]
    m[0] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[0]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m0, m4, 0x0c ; 0  2
    m[1] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[1]),
        _mm256_castsi256_pd(m[5]),
    )); // shufpd m1, m5, 0x0c ; 15 13
    m[2] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[2]),
        _mm256_castsi256_pd(m[6]),
    )); // shufpd m2, m6, 0x0c ; 4  6
    m[3] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[3]),
        _mm256_castsi256_pd(m[7]),
    )); // shufpd m3, m7, 0x0c ; 11  9
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 8).cast())); // vbroadcasti128 m4, [cq+16* 8]
    m[6] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 10).cast())); // vbroadcasti128 m6, [cq+16*10]
    m[5] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 7).cast())); // vbroadcasti128 m5, [cq+16* 7]
    m[7] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 5).cast())); // vbroadcasti128 m7, [cq+16* 5]
    m[4] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[4]),
        _mm256_castsi256_pd(m[6]),
    )); // shufpd m4, m6, 0x0c ; 8 10
    m[5] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[5]),
        _mm256_castsi256_pd(m[7]),
    )); // shufpd m5, m7, 0x0c ; 7  5
    m[6] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 12).cast())); // vbroadcasti128 m6, [cq+16*12]
    m[7] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 14).cast())); // vbroadcasti128 m7, [cq+16*14]
    m[6] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[6]),
        _mm256_castsi256_pd(m[7]),
    )); // shufpd m6, m7, 0x0c ; 12 14
    m[7] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 3).cast())); // vbroadcasti128 m7, [cq+16* 3]
    m[8] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 1).cast())); // vbroadcasti128 m8, [cq+16* 1]
    m[7] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[7]),
        _mm256_castsi256_pd(m[8]),
    )); // shufpd m7, m8, 0x0c ; 3  1
    iadst_4x16_internal_10bpc_main2(m)
}

/// `.main2` of `iadst_4x16_internal_10bpc`: in `m0`-`m7` (rows 0 2, 15 13, 4 6, 11 9, 8 10, 7 5, 12 14, 3 1);
/// expects `m12`/`m13` = clip minimum/maximum; `m11` = `pd_2048` on return.
#[inline(always)]
pub(crate) unsafe fn iadst_4x16_internal_10bpc_main2(m: &mut M) {
    // expects: m12 = clip_min   m13 = clip_max
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    itx_mulsub_2d(m, 1, 0, 8, 9, 10, Some(11), Q(201, 995), Q(4091, 3973), 1); // ITX_MULSUB_2D 1, 0, 8, 9, 10, 11, 201_995, 4091_3973, 1
    itx_mulsub_2d(m, 3, 2, 8, 9, 10, Some(11), Q(1751, 2440), Q(3703, 3290), 1); // ITX_MULSUB_2D 3, 2, 8, 9, 10, 11, 1751_2440, 3703_3290, 1
    itx_mulsub_2d(m, 5, 4, 8, 9, 10, Some(11), Q(3035, 3513), Q(2751, 2106), 1); // ITX_MULSUB_2D 5, 4, 8, 9, 10, 11, 3035_3513, 2751_2106, 1
    itx_mulsub_2d(m, 7, 6, 8, 9, 10, Some(11), Q(3857, 4052), Q(1380, 601), 1); // ITX_MULSUB_2D 7, 6, 8, 9, 10, 11, 3857_4052, 1380_601, 1
    m[8] = _mm256_sub_epi32(m[0], m[4]); // psubd m8, m0, m4 ; t8a  t10a
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; t0a  t2a
    m[4] = _mm256_sub_epi32(m[1], m[5]); // psubd m4, m1, m5 ; t9a  t11a
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; t1a  t3a
    m[5] = _mm256_sub_epi32(m[2], m[6]); // psubd m5, m2, m6 ; t12a t14a
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6 ; t4a  t6a
    m[6] = _mm256_sub_epi32(m[3], m[7]); // psubd m6, m3, m7 ; t13a t15a
    m[3] = _mm256_add_epi32(m[3], m[7]); // paddd m3, m7 ; t5a  t7a
    for x in [0, 1, 2, 3, 4, 5, 6, 8] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m8
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 8] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m8
    }
    itx_mulsub_2d(m, 8, 4, 7, 9, 10, Some(11), Q(799, 3406), Q(4017, 2276), 1); // ITX_MULSUB_2D 8, 4, 7, 9, 10, 11, 799_3406, 4017_2276, 1
    itx_mulsub_2d(m, 6, 5, 7, 9, 10, Some(11), Q(4017, 2276), 10, 1); // ITX_MULSUB_2D 6, 5, 7, 9, 10, 11, 4017_2276, 10, 1
    m[7] = _mm256_sub_epi32(m[0], m[2]); // psubd m7, m0, m2 ; t4   t6
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2 ; t0   t2
    m[2] = _mm256_sub_epi32(m[1], m[3]); // psubd m2, m1, m3 ; t5   t7
    m[1] = _mm256_add_epi32(m[1], m[3]); // paddd m1, m3 ; t1   t3
    m[3] = _mm256_sub_epi32(m[4], m[6]); // psubd m3, m4, m6 ; t12a t14a
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6 ; t8a  t10a
    m[6] = _mm256_sub_epi32(m[8], m[5]); // psubd m6, m8, m5 ; t13a t15a
    m[8] = _mm256_add_epi32(m[8], m[5]); // paddd m8, m5 ; t9a  t11a
    for x in [0, 1, 2, 3, 4, 6, 7, 8] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m6, m7, m8
    }
    for x in [0, 1, 2, 3, 4, 6, 7, 8] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m6, m7, m8
    }
    m[5] = _mm256_unpacklo_epi64(m[3], m[7]); // punpcklqdq m5, m3, m7 ; t12a t4
    m[3] = _mm256_unpackhi_epi64(m[3], m[7]); // punpckhqdq m3, m7 ; t14a t6
    m[7] = _mm256_unpackhi_epi64(m[6], m[2]); // punpckhqdq m7, m6, m2 ; t15a t7
    m[6] = _mm256_unpacklo_epi64(m[6], m[2]); // punpcklqdq m6, m2 ; t13a t5
    itx_mulsub_2d(m, 7, 3, 2, 9, 10, Some(11), 3784, 1567, 0); // ITX_MULSUB_2D 7, 3, 2, 9, 10, 11, 3784, 1567
    itx_mulsub_2d(m, 5, 6, 2, 9, 10, Some(11), 1567, 10, 0); // ITX_MULSUB_2D 5, 6, 2, 9, 10, 11, 1567, 10
    m[10] = pd(2896); // vpbroadcastd m10, [pd_2896]
    m[9] = _mm256_broadcastsi128_si256(ld_tab128(&pw_2048_m2048)); // vbroadcasti128 m9, [pw_2048_m2048] ; + + - -
    m[2] = _mm256_unpackhi_epi64(m[4], m[0]); // punpckhqdq m2, m4, m0 ; t10a t2
    m[4] = _mm256_unpacklo_epi64(m[4], m[0]); // punpcklqdq m4, m0 ; t8a  t0
    m[0] = _mm256_unpackhi_epi64(m[8], m[1]); // punpckhqdq m0, m8, m1 ; t11a t3
    m[8] = _mm256_unpacklo_epi64(m[8], m[1]); // punpcklqdq m8, m1 ; t9a  t1
    m[1] = _mm256_add_epi32(m[6], m[7]); // paddd m1, m6, m7 ; out2   -out3
    m[6] = _mm256_sub_epi32(m[6], m[7]); // psubd m6, m7 ; t14a t6
    m[7] = _mm256_add_epi32(m[5], m[3]); // paddd m7, m5, m3 ; -out13  out12
    m[5] = _mm256_sub_epi32(m[5], m[3]); // psubd m5, m3 ; t15a t7
    m[3] = _mm256_sub_epi32(m[8], m[0]); // psubd m3, m8, m0 ; t11  t3a
    m[8] = _mm256_add_epi32(m[8], m[0]); // paddd m8, m0 ; out14  -out15
    m[0] = _mm256_add_epi32(m[4], m[2]); // paddd m0, m4, m2 ; -out1   out0
    m[4] = _mm256_sub_epi32(m[4], m[2]); // psubd m4, m2 ; t10  t2a
    for x in [6, 5, 3, 4] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m6, m5, m3, m4
    }
    for x in [6, 5, 3, 4] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m6, m5, m3, m4
    }
    for x in [6, 5, 3, 4] {
        m[x] = _mm256_mullo_epi32(m[x], m[10]); // REPX {pmulld x, m10}, m6, m5, m3, m4
    }
    m[6] = _mm256_add_epi32(m[6], m[11]); // paddd m6, m11
    m[4] = _mm256_add_epi32(m[4], m[11]); // paddd m4, m11
    m[2] = _mm256_add_epi32(m[6], m[5]); // paddd m2, m6, m5 ; -out5   out4
    m[6] = _mm256_sub_epi32(m[6], m[5]); // psubd m6, m5 ; out10 -out11
    m[5] = _mm256_sub_epi32(m[4], m[3]); // psubd m5, m4, m3 ; -out9   out8
    m[3] = _mm256_add_epi32(m[3], m[4]); // paddd m3, m4 ; out6  -out7
    for x in [2, 3, 5, 6] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m2, m3, m5, m6
    }
    for x in [1, 8, 3, 6] {
        m[x] = _mm256_sign_epi32(m[x], m[9]); // REPX {psignd x, m9}, m1, m8, m3, m6
    }
    m[9] = _mm256_shuffle_epi32::<0x4e>(m[9]); // pshufd m9, m9, q1032
    for x in [0, 7, 2, 5] {
        m[x] = _mm256_sign_epi32(m[x], m[9]); // REPX {psignd x, m9}, m0, m7, m2, m5
    }
}

/// `.pass1` of `iflipadst_4x16_internal_10bpc` (the function's start) up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x16_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_16x4_internal_10bpc_main(m, c); // call m(iadst_16x4_internal_10bpc).main
    m[6] = pd(6144); // vpbroadcastd m6, [pd_6144]
    iadst_16x4_internal_10bpc_main_end(m); // call m(iadst_16x4_internal_10bpc).main_end
    m[0] = _mm256_srai_epi32::<13>(m[3]); // psrad m0, m3, 13
    m[1] = _mm256_srai_epi32::<13>(m[2]); // psrad m1, m2, 13
    m[2] = _mm256_srai_epi32::<13>(m[5]); // psrad m2, m5, 13
    m[3] = _mm256_srai_epi32::<13>(m[4]); // psrad m3, m4, 13
    m[4] = _mm256_srai_epi32::<13>(m[7]); // psrad m4, m7, 13
    m[5] = _mm256_srai_epi32::<13>(m[6]); // psrad m5, m6, 13
    m[6] = _mm256_srai_epi32::<13>(m[9]); // psrad m6, m9, 13
    m[7] = _mm256_srai_epi32::<13>(m[8]); // psrad m7, m8, 13
}

/// `.pass2` of `iflipadst_4x16_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_4x16_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    iadst_4x16_internal_10bpc_pass2_main(m); // call m(iadst_4x16_internal_10bpc).pass2_main
    m[5] = pw(2048); // vpbroadcastd m5, [pw_2048]
    m[8] = pw(0x3ff); // vpbroadcastd m8, [pixel_10bpc_max]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[4] = _mm256_blend_epi32::<0x33>(m[3], m[0]); // vpblendd m4, m3, m0, 0x33 ; -out0   out3   out1  -out2
    m[2] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m2, m2, q1032 ; -out11  out8   out10 -out9
    m[3] = _mm256_blend_epi32::<0xcc>(m[3], m[0]); // vpblendd m3, m0, 0xcc ; -out12  out15  out13 -out14
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    m[9] = _mm256_sub_epi16(m[7], m[5]); // psubw m9, m7, m5
    m[9] = _mm256_blend_epi32::<0x3c>(m[9], m[5]); // vpblendd m9, m5, 0x3c ; -2048   2048   2048  -2048
    m[0] = _mm256_mulhrs_epi16(m[4], m[9]); // pmulhrsw m0, m4, m9
    iflipadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[2], m[9]); // pmulhrsw m0, m2, m9
    iflipadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[1], m[9]); // pmulhrsw m0, m1, m9
    iflipadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
    m[0] = _mm256_mulhrs_epi16(m[3], m[9]); // pmulhrsw m0, m3, m9
    iflipadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call .write_4x4
}

/// `.write_4x4` of `iflipadst_4x16_internal_10bpc` (advances `dstq` by four rows and `cq` by 64 bytes)
#[inline(always)]
unsafe fn iflipadst_4x16_internal_10bpc_write_4x4(m: &mut M, pdst: &mut Dst, stride: isize, r6: isize, pc: &mut Coef) {
    let (mut dst, mut c) = (*pdst, *pc);
    m[4] = zx(_mm_loadl_epi64(dst.cast())); // movq xm4, [dstq+strideq*0]
    m[4] = zx(movhps_ld(lo(m[4]), at(dst, r6).cast())); // movhps xm4, [dstq+r6 ]
    m[5] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, stride).cast())); // vpbroadcastq m5, [dstq+strideq*1]
    m[6] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, stride * 2).cast())); // vpbroadcastq m6, [dstq+strideq*2]
    _mm256_storeu_si256(c.cast(), m[7]); // mova [cq+32*0], m7
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[7]); // mova [cq+32*1], m7
    c = at(c, 32 * 2); // add cq, 32*2
    m[4] = _mm256_blend_epi32::<0x30>(m[4], m[5]); // vpblendd m4, m5, 0x30
    m[4] = _mm256_blend_epi32::<0xc0>(m[4], m[6]); // vpblendd m4, m6, 0xc0
    m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
    m[4] = _mm256_max_epi16(m[4], m[7]); // pmaxsw m4, m7
    m[4] = _mm256_min_epi16(m[4], m[8]); // pminsw m4, m8
    m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
    _mm_storel_epi64(dst.cast(), lo(m[4])); // movq [dstq+strideq*0], xm4
    _mm_storel_epi64(at(dst, stride).cast(), lo(m[5])); // movq [dstq+strideq*1], xm5
    movhps_st(at(dst, stride * 2).cast(), lo(m[5])); // movhps [dstq+strideq*2], xm5
    movhps_st(at(dst, r6).cast(), lo(m[4])); // movhps [dstq+r6 ], xm4
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    (*pdst, *pc) = (dst, c);
}

/// `cglobal iidentity_4x16_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x16_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[7] = pd(5793); // vpbroadcastd m7, [pd_5793]
    m[0] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(c.cast())); // pmulld m0, m7, [cq+32*0]
    m[4] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulld m4, m7, [cq+32*1]
    m[1] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m1, m7, [cq+32*2]
    m[5] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulld m5, m7, [cq+32*3]
    m[2] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmulld m2, m7, [cq+32*4]
    m[6] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 5).cast())); // pmulld m6, m7, [cq+32*5]
    m[3] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 6).cast())); // pmulld m3, m7, [cq+32*6]
    m[7] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(at(c, 32 * 7).cast())); // pmulld m7, [cq+32*7]
    m[8] = pd(6144); // vpbroadcastd m8, [pd_6144]
    for x in [0, 4, 1, 5, 2, 6, 3, 7] {
        m[x] = _mm256_add_epi32(m[x], m[8]); // REPX {paddd x, m8}, m0, m4, m1, m5, m2, m6, m3, m7
    }
    for x in [0, 4, 1, 5, 2, 6, 3, 7] {
        m[x] = _mm256_srai_epi32::<13>(m[x]); // REPX {psrad x, 13}, m0, m4, m1, m5, m2, m6, m3, m7
    }
}

/// `.pass2` of `iidentity_4x16_internal_10bpc`
#[inline(always)]
unsafe fn iidentity_4x16_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
    m[7] = pw(1697 * 16); // vpbroadcastd m7, [pw_1697x16]
    m[8] = pw(2048); // vpbroadcastd m8, [pw_2048]
    m[4] = _mm256_mulhrs_epi16(m[7], m[0]); // pmulhrsw m4, m7, m0
    m[5] = _mm256_mulhrs_epi16(m[7], m[1]); // pmulhrsw m5, m7, m1
    m[6] = _mm256_mulhrs_epi16(m[7], m[2]); // pmulhrsw m6, m7, m2
    m[7] = _mm256_mulhrs_epi16(m[7], m[3]); // pmulhrsw m7, m3
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_adds_epi16(m[x], m[x]); // REPX {paddsw x, x}, m0, m1, m2, m3
    }
    m[0] = _mm256_adds_epi16(m[0], m[4]); // paddsw m0, m4
    m[1] = _mm256_adds_epi16(m[1], m[5]); // paddsw m1, m5
    m[2] = _mm256_adds_epi16(m[2], m[6]); // paddsw m2, m6
    m[3] = _mm256_adds_epi16(m[3], m[7]); // paddsw m3, m7
    m[4] = pw(0x3ff); // vpbroadcastd m4, [pixel_10bpc_max]
    iidentity_4x16_internal_10bpc_pass2_end(m, dst, stride, c); // call .pass2_end
}

/// `.pass2_end` of `iidentity_4x16_internal_10bpc`: expects `m8` = the `pmulhrsw` factor, `m4` = the pixel
/// maximum, `m0`-`m3` the packed rows
#[inline(always)]
unsafe fn iidentity_4x16_internal_10bpc_pass2_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    m[7] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m7, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    let r6 = stride * 5; // lea r6, [strideq*5]
    m[3] = _mm256_setzero_si256(); // pxor m3, m3
    m[5] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m5, m0, m2 ; 2 3   6 7
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2 ; 0 1   4 5
    m[6] = _mm256_unpacklo_epi32(m[7], m[1]); // punpckldq m6, m7, m1 ; 8 9   c d
    m[7] = _mm256_unpackhi_epi32(m[7], m[1]); // punpckhdq m7, m1 ; a b   e f
    m[0] = _mm256_mulhrs_epi16(m[0], m[8]); // pmulhrsw m0, m8
    iidentity_4x16_internal_10bpc_write_2x4x2(m, &mut dst, stride, r6, &mut c); // call .write_2x4x2
    m[0] = _mm256_mulhrs_epi16(m[5], m[8]); // pmulhrsw m0, m5, m8
    iidentity_4x16_internal_10bpc_write_2x4x2(m, &mut dst, stride, r6, &mut c); // call .write_2x4x2
    m[0] = _mm256_mulhrs_epi16(m[6], m[8]); // pmulhrsw m0, m6, m8
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    iidentity_4x16_internal_10bpc_write_2x4x2(m, &mut dst, stride, r6, &mut c); // call .write_2x4x2
    m[0] = _mm256_mulhrs_epi16(m[7], m[8]); // pmulhrsw m0, m7, m8
    iidentity_4x16_internal_10bpc_write_2x4x2(m, &mut dst, stride, r6, &mut c); // call .write_2x4x2
}

/// `.write_2x4x2` of `iidentity_4x16_internal_10bpc` (advances `dstq` by two rows and `cq` by 64 bytes)
#[inline(always)]
unsafe fn iidentity_4x16_internal_10bpc_write_2x4x2(
    m: &mut M,
    pdst: &mut Dst,
    stride: isize,
    r6: isize,
    pc: &mut Coef,
) {
    let (mut dst, mut c) = (*pdst, *pc);
    m[1] = zx(_mm_loadl_epi64(dst.cast())); // movq xm1, [dstq+strideq*0]
    m[1] = zx(movhps_ld(lo(m[1]), at(dst, stride).cast())); // movhps xm1, [dstq+strideq*1]
    m[2] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, stride * 4).cast())); // vpbroadcastq m2, [dstq+strideq*4]
    m[1] = _mm256_blend_epi32::<0x30>(m[1], m[2]); // vpblendd m1, m2, 0x30
    m[2] = _mm256_broadcastq_epi64(_mm_loadl_epi64(at(dst, r6).cast())); // vpbroadcastq m2, [dstq+r6 ]
    m[1] = _mm256_blend_epi32::<0xc0>(m[1], m[2]); // vpblendd m1, m2, 0xc0
    _mm256_storeu_si256(c.cast(), m[3]); // mova [cq+32*0], m3
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[3]); // mova [cq+32*1], m3
    c = at(c, 32 * 2); // add cq, 32*2
    m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
    m[1] = _mm256_max_epi16(m[1], m[3]); // pmaxsw m1, m3
    m[1] = _mm256_min_epi16(m[1], m[4]); // pminsw m1, m4
    m[2] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm2, m1, 1
    _mm_storel_epi64(dst.cast(), lo(m[1])); // movq [dstq+strideq*0], xm1
    movhps_st(at(dst, stride).cast(), lo(m[1])); // movhps [dstq+strideq*1], xm1
    _mm_storel_epi64(at(dst, stride * 4).cast(), lo(m[2])); // movq [dstq+strideq*4], xm2
    movhps_st(at(dst, r6).cast(), lo(m[2])); // movhps [dstq+r6 ], xm2
    dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
    (*pdst, *pc) = (dst, c);
}

/// `cglobal idct_4x16_internal_12bpc` up to its jump into the 10 bpc first pass
#[inline(always)]
unsafe fn idct_4x16_internal_12bpc_pass1(m: &mut M, c: Coef) {
    idct_4x16_internal_10bpc_pass1(m, c); // jmp m(idct_4x16_internal_10bpc).pass1
}

/// `.pass2` of `idct_4x16_internal_12bpc`
#[inline(always)]
unsafe fn idct_4x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = _mm256_unpacklo_epi32(m[0], m[1]); // punpckldq m8, m0, m1
    m[0] = _mm256_unpackhi_epi32(m[0], m[1]); // punpckhdq m0, m1
    m[9] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m9, m2, m3
    m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3
    m[1] = _mm256_unpacklo_epi32(m[4], m[5]); // punpckldq m1, m4, m5
    m[4] = _mm256_unpackhi_epi32(m[4], m[5]); // punpckhdq m4, m5
    m[3] = _mm256_unpacklo_epi32(m[6], m[7]); // punpckldq m3, m6, m7
    m[6] = _mm256_unpackhi_epi32(m[6], m[7]); // punpckhdq m6, m7
    m[5] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m5, m0, m2 ; 2  6
    m[12] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m12, m0, m2 ; 3  7
    m[0] = _mm256_unpacklo_epi64(m[8], m[9]); // punpcklqdq m0, m8, m9 ; 0  4
    m[10] = _mm256_unpackhi_epi64(m[8], m[9]); // punpckhqdq m10, m8, m9 ; 1  5
    m[2] = _mm256_unpacklo_epi64(m[1], m[3]); // punpcklqdq m2, m1, m3 ; 8 12
    m[13] = _mm256_unpackhi_epi64(m[1], m[3]); // punpckhqdq m13, m1, m3 ; 9 13
    m[9] = _mm256_unpacklo_epi64(m[4], m[6]); // punpcklqdq m9, m4, m6 ; 10 14
    m[4] = _mm256_unpackhi_epi64(m[4], m[6]); // punpckhqdq m4, m6 ; 11 15
    m[1] = _mm256_permute2x128_si256::<0x20>(m[5], m[9]); // vperm2i128 m1, m5, m9, 0x20 ; 2 10
    m[3] = _mm256_permute2x128_si256::<0x31>(m[9], m[5]); // vperm2i128 m3, m9, m5, 0x31 ; 14  6
    m[11] = _mm256_permute4x64_epi64::<0x72>(m[4]); // vpermq m11, m4, q1302 ; 15 11

    // interleave
    for x in [0, 1, 2, 3, 10] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3, m10
    }
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3, 10, 11, 12, 13] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3, m10, m11, m12, m13
    }
    for x in [0, 1, 2, 3, 10, 11, 12, 13] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3, m10, m11, m12, m13
    }
    idct_16x4_internal_10bpc_pass1_main(m); // call m(idct_16x4_internal_10bpc).pass1_main
    m[6] = _mm256_permute4x64_epi64::<0x72>(m[12]); // vpermq m6, m12, q1302 ; 7  3
    m[5] = _mm256_permute4x64_epi64::<0xd8>(m[13]); // vpermq m5, m13, q3120 ; 9 13
    idct_16x4_internal_10bpc_pass1_main2(m); // call m(idct_16x4_internal_10bpc).pass1_main2
    idct_16x4_internal_10bpc_pass1_main3(m); // call m(idct_16x4_internal_10bpc).pass1_main3
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<3>(m[x]); // REPX {psrad x, 3}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[1] = _mm256_packs_epi32(m[2], m[3]); // packssdw m1, m2, m3
    m[2] = _mm256_packs_epi32(m[4], m[5]); // packssdw m2, m4, m5
    m[3] = _mm256_packs_epi32(m[6], m[7]); // packssdw m3, m6, m7
    m[4] = ld_tab256(&idct16_12_shuf); // mova m4, [idct16_12_shuf]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permutevar8x32_epi32(m[x], m[4]); // REPX {vpermd x, m4, x}, m0, m1, m2, m3
    }
    m[9] = pw(16384); // vpbroadcastd m9, [pw_16384]
    m[8] = pw(0xfff); // vpbroadcastd m8, [pixel_12bpc_max]
    idct_4x16_internal_10bpc_pass2_end(m, dst, stride, c); // call m(idct_4x16_internal_10bpc).pass2_end
}

/// `cglobal iadst_4x16_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_4x16_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x16_internal_12bpc_main_pass1(m, c); // call .main_pass1
    m[0] = _mm256_srai_epi32::<12>(m[4]); // psrad m0, m4, 12
    m[1] = _mm256_srai_epi32::<12>(m[5]); // psrad m1, m5, 12
    m[2] = _mm256_srai_epi32::<12>(m[2]); // psrad m2, 12
    m[3] = _mm256_srai_epi32::<12>(m[3]); // psrad m3, 12
    m[4] = _mm256_srai_epi32::<12>(m[8]); // psrad m4, m8, 12
    m[5] = _mm256_srai_epi32::<12>(m[9]); // psrad m5, m9, 12
    m[6] = _mm256_srai_epi32::<12>(m[6]); // psrad m6, 12
    m[7] = _mm256_srai_epi32::<12>(m[7]); // psrad m7, 12
}

/// `.pass2` of `iadst_4x16_internal_12bpc` (falls into `.pass2_end`)
#[inline(always)]
unsafe fn iadst_4x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_4x16_internal_12bpc_transpose_16x4(m); // call .transpose_16x4
    iadst_4x16_internal_10bpc_main2(m); // call m(iadst_4x16_internal_10bpc).main2
    m[4] = _mm256_shuffle_epi32::<0x4e>(m[5]); // pshufd m4, m5, q1032
    m[5] = _mm256_srai_epi32::<3>(m[6]); // psrad m5, m6, 3
    m[6] = _mm256_shuffle_epi32::<0x4e>(m[7]); // pshufd m6, m7, q1032
    m[7] = _mm256_srai_epi32::<3>(m[8]); // psrad m7, m8, 3
    for x in [0, 2] {
        m[x] = _mm256_shuffle_epi32::<0x4e>(m[x]); // REPX {pshufd x, x, q1032}, m0, m2
    }
    for x in [0, 1, 2, 3, 4, 6] {
        m[x] = _mm256_srai_epi32::<3>(m[x]); // REPX {psrad x, 3}, m0, m1, m2, m3, m4, m6
    }
    iadst_4x16_internal_12bpc_pass2_end(m, dst, stride, c)
}

/// `.pass2_end` of `iadst_4x16_internal_12bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_12bpc_pass2_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[1] = _mm256_packs_epi32(m[2], m[3]); // packssdw m1, m2, m3
    m[2] = _mm256_packs_epi32(m[4], m[5]); // packssdw m2, m4, m5
    m[3] = _mm256_packs_epi32(m[6], m[7]); // packssdw m3, m6, m7
    m[4] = ld_tab256(&iadst16_12_shuf); // mova m4, [iadst16_12_shuf]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permutevar8x32_epi32(m[x], m[4]); // REPX {vpermd x, m4, x}, m0, m1, m2, m3
    }
    m[9] = pw(16384); // vpbroadcastd m9, [pw_16384]
    m[8] = pw(0xfff); // vpbroadcastd m8, [pixel_12bpc_max]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[7] = _mm256_setzero_si256(); // pxor m7, m7
    m[0] = _mm256_mulhrs_epi16(m[0], m[9]); // pmulhrsw m0, m9
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call m(iadst_4x16_internal_10bpc).write_4x4
    m[0] = _mm256_mulhrs_epi16(m[9], m[1]); // pmulhrsw m0, m9, m1
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call m(iadst_4x16_internal_10bpc).write_4x4
    m[0] = _mm256_mulhrs_epi16(m[9], m[2]); // pmulhrsw m0, m9, m2
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call m(iadst_4x16_internal_10bpc).write_4x4
    m[0] = _mm256_mulhrs_epi16(m[9], m[3]); // pmulhrsw m0, m9, m3
    iadst_4x16_internal_10bpc_write_4x4(m, &mut dst, stride, r6, &mut c); // call m(iadst_4x16_internal_10bpc).write_4x4
}

/// `.transpose_16x4` of `iadst_4x16_internal_12bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_12bpc_transpose_16x4(m: &mut M) {
    // transpose & interleave
    m[8] = _mm256_unpacklo_epi32(m[0], m[1]); // punpckldq m8, m0, m1
    m[0] = _mm256_unpackhi_epi32(m[0], m[1]); // punpckhdq m0, m1
    m[9] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m9, m2, m3
    m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3
    m[1] = _mm256_unpacklo_epi32(m[4], m[5]); // punpckldq m1, m4, m5
    m[4] = _mm256_unpackhi_epi32(m[4], m[5]); // punpckhdq m4, m5
    m[3] = _mm256_unpacklo_epi32(m[6], m[7]); // punpckldq m3, m6, m7
    m[6] = _mm256_unpackhi_epi32(m[6], m[7]); // punpckhdq m6, m7
    m[10] = _mm256_unpacklo_epi64(m[8], m[0]); // punpcklqdq m10, m8, m0
    m[0] = _mm256_unpackhi_epi64(m[0], m[8]); // punpckhqdq m0, m8
    m[11] = _mm256_unpacklo_epi64(m[9], m[2]); // punpcklqdq m11, m9, m2
    m[2] = _mm256_unpackhi_epi64(m[2], m[9]); // punpckhqdq m2, m9
    m[8] = _mm256_unpacklo_epi64(m[1], m[4]); // punpcklqdq m8, m1, m4
    m[4] = _mm256_unpackhi_epi64(m[4], m[1]); // punpckhqdq m4, m1
    m[9] = _mm256_unpacklo_epi64(m[3], m[6]); // punpcklqdq m9, m3, m6
    m[6] = _mm256_unpackhi_epi64(m[6], m[3]); // punpckhqdq m6, m3
    m[5] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m5, m0, m2, 0x31 ; 7  5
    m[7] = _mm256_permute2x128_si256::<0x20>(m[0], m[2]); // vperm2i128 m7, m0, m2, 0x20 ; 3  1
    m[0] = _mm256_permute2x128_si256::<0x20>(m[10], m[11]); // vperm2i128 m0, m10, m11, 0x20 ; 0  2
    m[2] = _mm256_permute2x128_si256::<0x31>(m[10], m[11]); // vperm2i128 m2, m10, m11, 0x31 ; 4  6
    m[1] = _mm256_permute2x128_si256::<0x31>(m[4], m[6]); // vperm2i128 m1, m4, m6, 0x31 ; 15 13
    m[3] = _mm256_permute2x128_si256::<0x20>(m[4], m[6]); // vperm2i128 m3, m4, m6, 0x20 ; 11  9
    m[4] = _mm256_permute2x128_si256::<0x20>(m[8], m[9]); // vperm2i128 m4, m8, m9, 0x20 ; 8 10
    m[6] = _mm256_permute2x128_si256::<0x31>(m[8], m[9]); // vperm2i128 m6, m8, m9, 0x31 ; 12 14
}

/// `.main_pass1` of `iadst_4x16_internal_12bpc`
#[inline(always)]
unsafe fn iadst_4x16_internal_12bpc_main_pass1(m: &mut M, c: Coef) {
    iadst_16x4_internal_10bpc_main(m, c); // call m(iadst_16x4_internal_10bpc).main
    m[6] = pd(3072); // vpbroadcastd m6, [pd_3072]
    m[10] = _mm256_add_epi32(m[4], m[5]); // paddd m10, m4, m5
    m[4] = _mm256_sub_epi32(m[4], m[3]); // psubd m4, m3
    m[5] = _mm256_sub_epi32(m[5], m[3]); // psubd m5, m3
    m[3] = _mm256_add_epi32(m[3], m[10]); // paddd m3, m10
    m[8] = _mm256_sub_epi32(m[7], m[1]); // psubd m8, m7, m1
    m[7] = _mm256_add_epi32(m[7], m[9]); // paddd m7, m9
    m[9] = _mm256_sub_epi32(m[9], m[1]); // psubd m9, m1
    m[7] = _mm256_add_epi32(m[7], m[1]); // paddd m7, m1
    for x in [4, 5, 2, 3, 8, 9, 0, 7] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m4, m5, m2, m3, m8, m9, m0, m7
    }
    for x in [4, 5, 2, 3, 8, 9, 7] {
        m[x] = _mm256_add_epi32(m[x], m[6]); // REPX {paddd x, m6}, m4, m5, m2, m3, m8, m9, m7
    }
    m[6] = _mm256_add_epi32(m[6], m[0]); // paddd m6, m0
}

/// `cglobal iflipadst_4x16_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_4x16_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x16_internal_12bpc_main_pass1(m, c); // call m(iadst_4x16_internal_12bpc).main_pass1
    m[0] = _mm256_srai_epi32::<12>(m[3]); // psrad m0, m3, 12
    m[1] = _mm256_srai_epi32::<12>(m[2]); // psrad m1, m2, 12
    m[2] = _mm256_srai_epi32::<12>(m[5]); // psrad m2, m5, 12
    m[3] = _mm256_srai_epi32::<12>(m[4]); // psrad m3, m4, 12
    m[4] = _mm256_srai_epi32::<12>(m[7]); // psrad m4, m7, 12
    m[5] = _mm256_srai_epi32::<12>(m[6]); // psrad m5, m6, 12
    m[6] = _mm256_srai_epi32::<12>(m[9]); // psrad m6, m9, 12
    m[7] = _mm256_srai_epi32::<12>(m[8]); // psrad m7, m8, 12
}

/// `.pass2` of `iflipadst_4x16_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_4x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_4x16_internal_12bpc_transpose_16x4(m); // call m(iadst_4x16_internal_12bpc).transpose_16x4
    iadst_4x16_internal_10bpc_main2(m); // call m(iadst_4x16_internal_10bpc).main2
    m[4] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m4, m3, q1032
    m[3] = _mm256_srai_epi32::<3>(m[5]); // psrad m3, m5, 3
    m[5] = _mm256_srai_epi32::<3>(m[2]); // psrad m5, m2, 3
    m[2] = _mm256_shuffle_epi32::<0x4e>(m[6]); // pshufd m2, m6, q1032
    m[6] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m6, m1, q1032
    m[1] = _mm256_srai_epi32::<3>(m[7]); // psrad m1, m7, 3
    m[7] = _mm256_srai_epi32::<3>(m[0]); // psrad m7, m0, 3
    m[0] = _mm256_shuffle_epi32::<0x4e>(m[8]); // pshufd m0, m8, q1032
    for x in [0, 2, 4, 6] {
        m[x] = _mm256_srai_epi32::<3>(m[x]); // REPX {psrad x, 3}, m0, m2, m4, m6
    }
    iadst_4x16_internal_12bpc_pass2_end(m, dst, stride, c); // jmp m(iadst_4x16_internal_12bpc).pass2_end
}

/// `cglobal iidentity_4x16_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_4x16_internal_12bpc_pass1(m: &mut M, c: Coef) {
    m[8] = pd(1697); // vpbroadcastd m8, [pd_1697]
    m[0] = _mm256_loadu_si256(c.cast()); // mova m0, [cq+32*0]
    m[4] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m4, [cq+32*1]
    m[1] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m1, [cq+32*2]
    m[5] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m5, [cq+32*3]
    m[9] = pd(6144); // vpbroadcastd m9, [pd_6144]
    m[2] = _mm256_mullo_epi32(m[8], m[0]); // pmulld m2, m8, m0
    m[6] = _mm256_mullo_epi32(m[8], m[4]); // pmulld m6, m8, m4
    m[3] = _mm256_mullo_epi32(m[8], m[1]); // pmulld m3, m8, m1
    m[7] = _mm256_mullo_epi32(m[8], m[5]); // pmulld m7, m8, m5
    m[10] = _mm256_loadu_si256(at(c, 32 * 4).cast()); // mova m10, [cq+32*4]
    m[11] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m11, [cq+32*5]
    m[12] = _mm256_loadu_si256(at(c, 32 * 6).cast()); // mova m12, [cq+32*6]
    m[13] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m13, [cq+32*7]
    for x in [2, 6, 3, 7] {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m2, m6, m3, m7
    }
    for x in [2, 6, 3, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m2, m6, m3, m7
    }
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2
    m[2] = _mm256_mullo_epi32(m[8], m[10]); // pmulld m2, m8, m10
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6
    m[6] = _mm256_mullo_epi32(m[8], m[11]); // pmulld m6, m8, m11
    m[1] = _mm256_add_epi32(m[1], m[3]); // paddd m1, m3
    m[3] = _mm256_mullo_epi32(m[8], m[12]); // pmulld m3, m8, m12
    m[5] = _mm256_add_epi32(m[5], m[7]); // paddd m5, m7
    m[7] = _mm256_mullo_epi32(m[8], m[13]); // pmulld m7, m8, m13
    for x in [0, 4, 1, 5] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m0, m4, m1, m5
    }
    for x in [2, 6, 3, 7] {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m2, m6, m3, m7
    }
    for x in [2, 6, 3, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m2, m6, m3, m7
    }
    m[2] = _mm256_add_epi32(m[2], m[10]); // paddd m2, m10
    m[6] = _mm256_add_epi32(m[6], m[11]); // paddd m6, m11
    m[3] = _mm256_add_epi32(m[3], m[12]); // paddd m3, m12
    m[7] = _mm256_add_epi32(m[7], m[13]); // paddd m7, m13
    for x in [2, 6, 3, 7] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m2, m6, m3, m7
    }
}

/// `.pass2` of `iidentity_4x16_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_4x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[8] = pd(5793); // vpbroadcastd m8, [pd_5793]
    m[9] = pd(1024); // vpbroadcastd m9, [pd_1024]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_mullo_epi32(m[x], m[8]); // REPX {pmulld x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<14>(m[x]); // REPX {psrad x, 14}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
    m[8] = pw(16384); // vpbroadcastd m8, [pw_16384]
    m[4] = pw(0xfff); // vpbroadcastd m4, [pixel_12bpc_max]
    iidentity_4x16_internal_10bpc_pass2_end(m, dst, stride, c); // call m(iidentity_4x16_internal_10bpc).pass2_end
}

// --- 8x4 ----------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_8X4_FN type1, type2, 10` (`INV_TXFM_FN type1, type2, 0, 8x4, 10` and the DC-only path of
/// dct_dct); the two `vpbroadcastd` of `idct_8x4_internal_10bpc` before its `.pass1` are in the dct arm.
#[inline(always)]
unsafe fn inv_txfm_add_8x4_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = pw(0x7c00); // vpbroadcastd m2, [dconly_10bpc]
        return inv_txfm_add_dct_dct_8x4_10bpc_dconly(m, dst, stride, c, eob);
    }
    match t1 {
        K::Dct => {
            // cglobal idct_8x4_internal_10bpc
            m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
            m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
            idct_8x4_internal_10bpc_pass1(m, c)
        }
        K::Adst => iadst_8x4_internal_10bpc_pass1(m, c),
        K::FlipAdst => iflipadst_8x4_internal_10bpc_pass1(m, c),
        K::Identity => iidentity_8x4_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x4_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_8x4_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x4_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x4_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro INV_TXFM_8X4_FN type1, type2, 12` (`INV_TXFM_FN type1, type2, 0, 8x4, 12` and the DC-only path of
/// dct_dct)
#[inline(always)]
unsafe fn inv_txfm_add_8x4_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = pw(0x7000); // vpbroadcastd m2, [dconly_12bpc]
        return inv_txfm_add_dct_dct_8x4_10bpc_dconly(m, dst, stride, c, eob); // jmp m(inv_txfm_add_dct_dct_8x4_10bpc).dconly
    }
    match t1 {
        K::Dct => idct_8x4_internal_12bpc_pass1(m, c),
        K::Adst => iadst_8x4_internal_12bpc_pass1(m, c),
        K::FlipAdst => iflipadst_8x4_internal_12bpc_pass1(m, c),
        K::Identity => iidentity_8x4_internal_12bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x4_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_8x4_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x4_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x4_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `.pass1` of `idct_8x4_internal_10bpc` up to `jmp tx2q`: expects `m8`/`m9` = clip minimum/maximum (also the
/// first pass of `idct_8x4_internal_12bpc`)
#[inline(always)]
unsafe fn idct_8x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[1] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 1).cast())); // vbroadcasti128 m1, [cq+16*1]
    m[0] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 5).cast())); // vbroadcasti128 m0, [cq+16*5]
    m[2] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 3).cast())); // vbroadcasti128 m2, [cq+16*3]
    m[3] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 7).cast())); // vbroadcasti128 m3, [cq+16*7]
    m[6] = pd(2896); // vpbroadcastd m6, [pd_2896]
    m[1] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[1]),
        _mm256_castsi256_pd(m[0]),
    )); // shufpd m1, m0, 0x0c ; 1 5
    m[3] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[3]),
        _mm256_castsi256_pd(m[2]),
    )); // shufpd m3, m2, 0x0c ; 7 3
    m[0] = _mm256_broadcastsi128_si256(_mm_loadu_si128(c.cast())); // vbroadcasti128 m0, [cq+16*0]
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 2).cast())); // vbroadcasti128 m4, [cq+16*2]
    m[2] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 4).cast())); // vbroadcasti128 m2, [cq+16*4]
    m[5] = _mm256_broadcastsi128_si256(_mm_loadu_si128(at(c, 16 * 6).cast())); // vbroadcasti128 m5, [cq+16*6]
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    m[0] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[0]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m0, m4, 0x0c ; 0 2
    m[2] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(m[2]),
        _mm256_castsi256_pd(m[5]),
    )); // shufpd m2, m5, 0x0c ; 4 6
    for x in [1, 3, 0, 2] {
        m[x] = _mm256_mullo_epi32(m[x], m[6]); // REPX {pmulld x, m6}, m1, m3, m0, m2
    }
    for x in [1, 3, 0, 2] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m1, m3, m0, m2
    }
    for x in [1, 3, 0, 2] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m1, m3, m0, m2
    }
    idct_8x4_internal_10bpc_main(m); // call .main
    m[3] = _mm256_sub_epi32(m[0], m[4]); // psubd m3, m0, m4 ; out7 out6 (interleaved)
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; out0 out1 (interleaved)
    m[1] = _mm256_add_epi32(m[2], m[5]); // paddd m1, m2, m5 ; out3 out2 (interleaved)
    m[2] = _mm256_sub_epi32(m[2], m[5]); // psubd m2, m5 ; out4 out5 (interleaved)
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m1, m1, q1032
    m[3] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m3, m3, q1032
}

/// `.pass2` of `idct_8x4_internal_10bpc`
#[inline(always)]
unsafe fn idct_8x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m4, [deint_shuf]
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3
    m[1] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1
    m[0] = _mm256_shuffle_epi8(m[0], m[4]); // pshufb m0, m4
    m[1] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m1, m4
    idct4_1d_packed_word(m, 0, 1, 2, 3, 4, 7); // IDCT4_1D_PACKED_WORD 0, 1, 2, 3, 4, 7
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120 ; out0 out1
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m2, m1, q2031 ; out2 out3
    iadst_8x4_internal_10bpc_end(m, dst, stride, c); // jmp m(iadst_8x4_internal_10bpc).end
}

/// `.main` of `idct_8x4_internal_10bpc`: in `m0` (0 2), `m1` (1 5), `m2` (4 6), `m3` (7 3), rounded; expects
/// `m7` = `pd_2048`, `m8`/`m9` = clip minimum/maximum; out `m0`, `m2`, `m4`, `m5` (the caller's butterflies),
/// `m6` = `pd_2896`, `m1`/`m3` clobbered.
#[inline(always)]
pub(crate) unsafe fn idct_8x4_internal_10bpc_main(m: &mut M) {
    itx_mulsub_2d(m, 1, 3, 4, 5, 6, Some(7), Q(799, 3406), Q(4017, 2276), 1); // ITX_MULSUB_2D 1, 3, 4, 5, 6, 7, 799_3406, 4017_2276, 1
    idct4_1d_packed(m, 0, 2, 4, 5, 6, 7); // IDCT4_1D_PACKED 0, 2, 4, 5, 6, 7
    m[6] = pd(2896); // vpbroadcastd m6, [pd_2896]
    m[4] = _mm256_unpacklo_epi64(m[1], m[3]); // punpcklqdq m4, m1, m3 ; t4a  t7a
    m[1] = _mm256_unpackhi_epi64(m[1], m[3]); // punpckhqdq m1, m3 ; t5a  t6a
    m[3] = _mm256_sub_epi32(m[4], m[1]); // psubd m3, m4, m1 ; t5a  t6a
    m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1 ; t4   t7
    for x in [3, 4, 0, 2] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m3, m4, m0, m2
    }
    for x in [3, 4, 0, 2] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m3, m4, m0, m2
    }
    m[3] = _mm256_mullo_epi32(m[3], m[6]); // pmulld m3, m6
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m1, m3, q1032
    m[3] = _mm256_add_epi32(m[3], m[7]); // paddd m3, m7
    m[5] = _mm256_sub_epi32(m[3], m[1]); // psubd m5, m3, m1
    m[1] = _mm256_add_epi32(m[1], m[3]); // paddd m1, m3
    m[5] = _mm256_srai_epi32::<12>(m[5]); // psrad m5, 12
    m[1] = _mm256_srai_epi32::<12>(m[1]); // psrad m1, 12
    m[5] = _mm256_blend_epi32::<0x33>(m[5], m[4]); // vpblendd m5, m4, 0x33 ; t4   t5
    m[4] = _mm256_unpackhi_epi64(m[4], m[1]); // punpckhqdq m4, m1 ; t7   t6
}

/// `cglobal iadst_8x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_8x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x8_internal_10bpc_main(m, c); // call m(iadst_4x8_internal_10bpc).main
    m[3] = _mm256_blend_epi32::<0x33>(m[0], m[4]); // vpblendd m3, m0, m4, 0x33 ; out6 out7
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[4]); // vpblendd m0, m4, 0xcc ; out0 out1
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[5]); // pshufd m1, m5, q1032
    m[2] = _mm256_sign_epi32(m[2], m[6]); // psignd m2, m6 ; out4 out5
    m[1] = _mm256_sign_epi32(m[1], m[6]); // psignd m1, m6 ; out2 out3
}

/// `.pass2` of `iadst_8x4_internal_10bpc` (falls into `.end`)
#[inline(always)]
unsafe fn iadst_8x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x4_internal_10bpc_pass2_main(m); // call .pass2_main
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120 ; out0 out1
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m2, m1, q3120 ; out2 out3
    iadst_8x4_internal_10bpc_end(m, dst, stride, c)
}

/// `.end` of `iadst_8x4_internal_10bpc` (falls into `.end2`)
#[inline(always)]
unsafe fn iadst_8x4_internal_10bpc_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[1] = pw(2048); // vpbroadcastd m1, [pw_2048]
    m[0] = _mm256_mulhrs_epi16(m[0], m[1]); // pmulhrsw m0, m1
    m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
    m[5] = pw(0x3ff); // vpbroadcastd m5, [pixel_10bpc_max]
    iadst_8x4_internal_10bpc_end2(m, dst, stride, c)
}

/// `.end2` of `iadst_8x4_internal_10bpc`: expects `m5` = the pixel maximum, `m0`/`m1` the four rows
#[inline(always)]
unsafe fn iadst_8x4_internal_10bpc_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[2] = zx(_mm_loadu_si128(dst.cast())); // mova xm2, [dstq+strideq*0]
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(at(dst, stride).cast())); // vinserti128 m2, [dstq+strideq*1], 1
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[3] = zx(_mm_loadu_si128(r6.cast())); // mova xm3, [r6 +strideq*0]
    m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(r6, stride).cast())); // vinserti128 m3, [r6 +strideq*1], 1
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in [0, 1, 2, 3] {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
    m[0] = _mm256_max_epi16(m[0], m[4]); // pmaxsw m0, m4
    m[1] = _mm256_max_epi16(m[1], m[4]); // pmaxsw m1, m4
    m[0] = _mm256_min_epi16(m[0], m[5]); // pminsw m0, m5
    m[1] = _mm256_min_epi16(m[1], m[5]); // pminsw m1, m5
    _mm_storeu_si128(dst.cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
    _mm_storeu_si128(at(dst, stride).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
    _mm_storeu_si128(r6.cast(), lo(m[1])); // mova [r6 +strideq*0], xm1
    _mm_storeu_si128(at(r6, stride).cast(), _mm256_extracti128_si256::<1>(m[1]));
    // vextracti128 [r6 +strideq*1], m1, 1
}

/// `.pass2_main` of `iadst_8x4_internal_10bpc` (ends in `m(iadst_8x4_internal_8bpc).main`)
#[inline(always)]
unsafe fn iadst_8x4_internal_10bpc_pass2_main(m: &mut M) {
    m[4] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m4, [deint_shuf]
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3

    // (no code: the 8 bpc code's o_base) // lea r6, [deint_shuf+128]
    m[1] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1
    m[0] = _mm256_shuffle_epi8(m[0], m[4]); // pshufb m0, m4
    m[1] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m1, m4
    iadst_8x4_internal_8bpc_main(m); // jmp m(iadst_8x4_internal_8bpc).main
}

/// `.main` of `iadst_8x4_internal_10bpc` (falls into `.main2`): loads the four rows of 8 coefficients at `c`,
/// scales them by 2896 and runs `IADST4_1D`; out (unrounded) `m4`, `m6`, `m2`, `m3`.
#[inline(always)]
pub(crate) unsafe fn iadst_8x4_internal_10bpc_main(m: &mut M, c: Coef) {
    m[1] = pd(2896); // vpbroadcastd m1, [pd_2896]
    m[0] = _mm256_mullo_epi32(m[1], _mm256_loadu_si256(c.cast())); // pmulld m0, m1, [cq+32*0]
    m[3] = _mm256_mullo_epi32(m[1], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulld m3, m1, [cq+32*3]
    m[2] = _mm256_mullo_epi32(m[1], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m2, m1, [cq+32*2]
    m[1] = _mm256_mullo_epi32(m[1], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulld m1, [cq+32*1]
    m[4] = pd(2048); // vpbroadcastd m4, [pd_2048]
    for x in [0, 3, 2, 1] {
        m[x] = _mm256_add_epi32(m[x], m[4]); // REPX {paddd x, m4}, m0, m3, m2, m1
    }
    for x in [0, 3, 2, 1] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m3, m2, m1
    }
    iadst_8x4_internal_10bpc_main2(m)
}

/// `.main2` of `iadst_8x4_internal_10bpc` (`IADST4_1D` on `m0`-`m3`)
#[inline(always)]
pub(crate) unsafe fn iadst_8x4_internal_10bpc_main2(m: &mut M) {
    iadst4_1d(m); // IADST4_1D
}

/// `cglobal iflipadst_8x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_8x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x8_internal_10bpc_main(m, c); // call m(iadst_4x8_internal_10bpc).main
    m[3] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[4]),
        _mm256_castsi256_pd(m[0]),
    )); // shufpd m3, m4, m0, 0x05
    m[0] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[0]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m0, m4, 0x05
    m[2] = _mm256_sign_epi32(m[2], m[6]); // psignd m2, m6
    m[6] = _mm256_shuffle_epi32::<0x4e>(m[6]); // pshufd m6, m6, q1032
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m1, m2, q1032
    m[2] = _mm256_sign_epi32(m[5], m[6]); // psignd m2, m5, m6
}

/// `.pass2` of `iflipadst_8x4_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_8x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x4_internal_10bpc_pass2_main(m); // call m(iadst_8x4_internal_10bpc).pass2_main
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[0]); // vpermq m2, m0, q2031
    m[0] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m0, m1, q2031
    iadst_8x4_internal_10bpc_end(m, dst, stride, c); // jmp m(iadst_8x4_internal_10bpc).end
}

/// `.pass1` of `iidentity_8x4_internal_10bpc` (the function's start) up to `jmp tx2q`; also the first pass of
/// `iidentity_8x4_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_8x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[4] = pd(2896); // vpbroadcastd m4, [pd_2896]
    m[0] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(c.cast())); // vpermq m0, [cq+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 1).cast())); // vpermq m1, [cq+32*1], q3120
    m[2] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 2).cast())); // vpermq m2, [cq+32*2], q3120
    m[3] = _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(at(c, 32 * 3).cast())); // vpermq m3, [cq+32*3], q3120
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mullo_epi32(m[x], m[4]); // REPX {pmulld x, m4}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[x]); // REPX {paddd x, x }, m0, m1, m2, m3
    }
}

/// `.pass2` of `iidentity_8x4_internal_10bpc` (falls into `.pass2_end`)
#[inline(always)]
unsafe fn iidentity_8x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[5] = pw(0x3ff); // vpbroadcastd m5, [pixel_10bpc_max]
    m[4] = pw(1697 * 8); // vpbroadcastd m4, [pw_1697x8]
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3
    m[1] = _mm256_mulhrs_epi16(m[4], m[0]); // pmulhrsw m1, m4, m0
    m[4] = _mm256_mulhrs_epi16(m[4], m[2]); // pmulhrsw m4, m2
    m[0] = _mm256_adds_epi16(m[0], m[1]); // paddsw m0, m1
    m[2] = _mm256_adds_epi16(m[2], m[4]); // paddsw m2, m4
    m[7] = _mm256_packs_epi32(m[7], m[7]); // packssdw m7, m7 ; pw_2048
    iidentity_8x4_internal_10bpc_pass2_end(m, dst, stride, c)
}

/// `.pass2_end` of `iidentity_8x4_internal_10bpc`: expects `m7` = the `pmulhrsw` factor, `m5` = the pixel
/// maximum, `m0`/`m2` the packed rows
#[inline(always)]
unsafe fn iidentity_8x4_internal_10bpc_pass2_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[1] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m1, m0, m2
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[2] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m2, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[2] = _mm256_mulhrs_epi16(m[2], m[7]); // pmulhrsw m2, m7
    m[0] = _mm256_mulhrs_epi16(m[0], m[7]); // pmulhrsw m0, m7
    m[1] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m1, m0, m2
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    m[2] = zx(_mm_loadu_si128(dst.cast())); // mova xm2, [dstq+strideq*0]
    m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_loadu_si128(r6.cast())); // vinserti128 m2, [r6 +strideq*0], 1
    m[3] = zx(_mm_loadu_si128(at(dst, stride).cast())); // mova xm3, [dstq+strideq*1]
    m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_loadu_si128(at(r6, stride).cast())); // vinserti128 m3, [r6 +strideq*1], 1
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in [0, 1, 2, 3] {
        _mm256_storeu_si256(at(c, 32 * x).cast(), m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3
    }
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
    m[0] = _mm256_max_epi16(m[0], m[4]); // pmaxsw m0, m4
    m[1] = _mm256_max_epi16(m[1], m[4]); // pmaxsw m1, m4
    m[0] = _mm256_min_epi16(m[0], m[5]); // pminsw m0, m5
    m[1] = _mm256_min_epi16(m[1], m[5]); // pminsw m1, m5
    _mm_storeu_si128(dst.cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
    _mm_storeu_si128(at(dst, stride).cast(), lo(m[1])); // mova [dstq+strideq*1], xm1
    _mm_storeu_si128(r6.cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [r6 +strideq*0], m0, 1
    _mm_storeu_si128(at(r6, stride).cast(), _mm256_extracti128_si256::<1>(m[1]));
    // vextracti128 [r6 +strideq*1], m1, 1
}

/// `cglobal idct_8x4_internal_12bpc` up to its jump into the 10 bpc `.pass1`
#[inline(always)]
unsafe fn idct_8x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    m[8] = pd(-0x80000); // vpbroadcastd m8, [clip_20b_min]
    m[9] = pd(0x7ffff); // vpbroadcastd m9, [clip_20b_max]
    idct_8x4_internal_10bpc_pass1(m, c); // jmp m(idct_8x4_internal_10bpc).pass1
}

/// `.pass2` of `idct_8x4_internal_12bpc`
#[inline(always)]
unsafe fn idct_8x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    iadst_8x4_internal_12bpc_transpose_4x8(m); // call m(iadst_8x4_internal_12bpc).transpose_4x8
    idct4_1d(m, [0, 1, 2, 3], [4, 5, 6], 7); // IDCT4_1D 0, 1, 2, 3, 4, 5, 6, 7
    iadst_8x4_internal_12bpc_end(m, dst, stride, c); // jmp m(iadst_8x4_internal_12bpc).end
}

/// `cglobal iadst_8x4_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iadst_8x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    m[8] = pd(-0x80000); // vpbroadcastd m8, [clip_20b_min]
    m[9] = pd(0x7ffff); // vpbroadcastd m9, [clip_20b_max]
    iadst_4x8_internal_10bpc_main2(m, c); // call m(iadst_4x8_internal_10bpc).main2
    m[3] = _mm256_blend_epi32::<0x33>(m[0], m[4]); // vpblendd m3, m0, m4, 0x33 ; out6 out7
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[4]); // vpblendd m0, m4, 0xcc ; out0 out1
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[5]); // pshufd m1, m5, q1032
    m[2] = _mm256_sign_epi32(m[2], m[6]); // psignd m2, m6 ; out4 out5
    m[1] = _mm256_sign_epi32(m[1], m[6]); // psignd m1, m6 ; out2 out3
}

/// `.pass2` of `iadst_8x4_internal_12bpc` (falls into `.pass2_end`)
#[inline(always)]
unsafe fn iadst_8x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    iadst_8x4_internal_12bpc_pass2_main(m); // call .pass2_main
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_add_epi32(m[5], m[4]); // paddd m0, m5, m4
    m[1] = _mm256_add_epi32(m[5], m[6]); // paddd m1, m5, m6
    m[2] = _mm256_add_epi32(m[2], m[5]); // paddd m2, m5
    m[3] = _mm256_add_epi32(m[3], m[5]); // paddd m3, m5
    iadst_8x4_internal_12bpc_pass2_end(m, dst, stride, c)
}

/// `.pass2_end` of `iadst_8x4_internal_12bpc` (falls into `.end`)
#[inline(always)]
unsafe fn iadst_8x4_internal_12bpc_pass2_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m1, m2, m3
    }
    iadst_8x4_internal_12bpc_end(m, dst, stride, c)
}

/// `.end` of `iadst_8x4_internal_12bpc`
#[inline(always)]
unsafe fn iadst_8x4_internal_12bpc_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(16384); // vpbroadcastd m4, [pw_16384]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<3>(m[x]); // REPX {psrad x, 3}, m0, m1, m2, m3
    }
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[1] = _mm256_mulhrs_epi16(m[2], m[4]); // pmulhrsw m1, m2, m4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120 ; out0 out1
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120 ; out2 out3
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
    iadst_8x4_internal_10bpc_end2(m, dst, stride, c); // jmp m(iadst_8x4_internal_10bpc).end2
}

/// `.pass2_main` of `iadst_8x4_internal_12bpc` (ends in `m(iadst_8x4_internal_10bpc).main2`)
#[inline(always)]
unsafe fn iadst_8x4_internal_12bpc_pass2_main(m: &mut M) {
    iadst_8x4_internal_12bpc_transpose_4x8(m); // call .transpose_4x8
    iadst_8x4_internal_10bpc_main2(m); // jmp m(iadst_8x4_internal_10bpc).main2
}

/// `.transpose_4x8` of `iadst_8x4_internal_12bpc`: `m0`-`m3` (interleaved pairs of rows) in and out, `m4`/`m5`
/// clobbered.
#[inline(always)]
pub(crate) unsafe fn iadst_8x4_internal_12bpc_transpose_4x8(m: &mut M) {
    // deinterleave
    m[0] = _mm256_shuffle_epi32::<0xd8>(m[0]); // pshufd m0, m0, q3120
    m[1] = _mm256_shuffle_epi32::<0xd8>(m[1]); // pshufd m1, m1, q3120
    m[2] = _mm256_shuffle_epi32::<0xd8>(m[2]); // pshufd m2, m2, q3120
    m[3] = _mm256_shuffle_epi32::<0xd8>(m[3]); // pshufd m3, m3, q3120

    // transpose
    m[4] = _mm256_unpacklo_epi64(m[0], m[1]); // punpcklqdq m4, m0, m1
    m[0] = _mm256_unpackhi_epi64(m[0], m[1]); // punpckhqdq m0, m1
    m[5] = _mm256_unpacklo_epi64(m[2], m[3]); // punpcklqdq m5, m2, m3
    m[2] = _mm256_unpackhi_epi64(m[2], m[3]); // punpckhqdq m2, m3
    m[1] = _mm256_permute2x128_si256::<0x20>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x20 ; out1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m3, m0, m2, 0x31 ; out3
    m[2] = _mm256_permute2x128_si256::<0x31>(m[4], m[5]); // vperm2i128 m2, m4, m5, 0x31 ; out2
    m[0] = _mm256_permute2x128_si256::<0x20>(m[4], m[5]); // vperm2i128 m0, m4, m5, 0x20 ; out0
}

/// `cglobal iflipadst_8x4_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iflipadst_8x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    m[8] = pd(-0x80000); // vpbroadcastd m8, [clip_20b_min]
    m[9] = pd(0x7ffff); // vpbroadcastd m9, [clip_20b_max]
    iadst_4x8_internal_10bpc_main2(m, c); // call m(iadst_4x8_internal_10bpc).main2
    m[3] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[4]),
        _mm256_castsi256_pd(m[0]),
    )); // shufpd m3, m4, m0, 0x05
    m[0] = _mm256_castpd_si256(_mm256_shuffle_pd::<0x05>(
        _mm256_castsi256_pd(m[0]),
        _mm256_castsi256_pd(m[4]),
    )); // shufpd m0, m4, 0x05
    m[2] = _mm256_sign_epi32(m[2], m[6]); // psignd m2, m6
    m[6] = _mm256_shuffle_epi32::<0x4e>(m[6]); // pshufd m6, m6, q1032
    m[1] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m1, m2, q1032
    m[2] = _mm256_sign_epi32(m[5], m[6]); // psignd m2, m5, m6
}

/// `.pass2` of `iflipadst_8x4_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_8x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    iadst_8x4_internal_12bpc_pass2_main(m); // call m(iadst_8x4_internal_12bpc).pass2_main
    m[5] = pd(2048); // vpbroadcastd m5, [pd_2048]
    m[0] = _mm256_add_epi32(m[5], m[3]); // paddd m0, m5, m3
    m[1] = _mm256_add_epi32(m[5], m[2]); // paddd m1, m5, m2
    m[3] = _mm256_add_epi32(m[5], m[4]); // paddd m3, m5, m4
    m[2] = _mm256_add_epi32(m[5], m[6]); // paddd m2, m5, m6
    iadst_8x4_internal_12bpc_pass2_end(m, dst, stride, c); // jmp m(iadst_8x4_internal_12bpc).pass2_end
}

/// `cglobal iidentity_8x4_internal_12bpc` up to its jump into the 10 bpc `.pass1`
#[inline(always)]
unsafe fn iidentity_8x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    iidentity_8x4_internal_10bpc_pass1(m, c); // jmp m(iidentity_8x4_internal_10bpc).pass1
}

/// `.pass2` of `iidentity_8x4_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_8x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    // m0 = in0 in1 (interleaved)
    // m1 = in2 in3 (interleaved)
    // m2 = in4 in5 (interleaved)
    // m3 = in6 in7 (interleaved)
    m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
    m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m0, m1, m2, m3
    }
    m[4] = pd(5793); // vpbroadcastd m4, [pd_5793]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mullo_epi32(m[x], m[4]); // REPX {pmulld x, m4}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<15>(m[x]); // REPX {psrad x, 15}, m0, m1, m2, m3
    }
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
    m[7] = pw(16384); // vpbroadcastd m7, [pw_16384]
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3
    iidentity_8x4_internal_10bpc_pass2_end(m, dst, stride, c); // jmp m(iidentity_8x4_internal_10bpc).pass2_end
}

// --- the exported functions ---------------------------------------------------------------------------------------

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x16_10bpc_avx2,
    inv_txfm_add_4x16_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_4x16_12bpc_avx2,
    inv_txfm_add_4x16_12bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x4_10bpc_avx2,
    inv_txfm_add_8x4_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x4_12bpc_avx2,
    inv_txfm_add_8x4_12bpc,
    K::Identity,
    K::Identity
);

// --- part 3: itx16_avx2.asm lines 2262-3446 (8x8, 8x16) ---------------------------------------------------------

// --- 8x8 ---------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_8X8_FN type1, type2, 10`: the DC-only path of dct_dct (`.dconly`, in the prelude), then the
/// two passes. The `vpbroadcastd m12/m13, [clip_18b_*]` before each `.pass1:` label are here.
#[inline(always)]
unsafe fn inv_txfm_add_8x8_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = pw(0x7c00); // vpbroadcastd m2, [dconly_10bpc]
        return inv_txfm_add_dct_dct_8x8_10bpc_dconly(m, dst, stride, c, eob); // .dconly:
    }
    match t1 {
        K::Dct => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            idct_8x8_internal_10bpc_pass1(m, c)
        }
        K::Adst => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            iadst_8x8_internal_10bpc_pass1(m, c)
        }
        K::FlipAdst => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            iflipadst_8x8_internal_10bpc_pass1(m, c)
        }
        K::Identity => iidentity_8x8_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x8_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_8x8_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x8_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x8_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro INV_TXFM_8X8_FN type1, type2, 12`: the DC-only path of dct_dct (10 bpc's `.dconly` with
/// `dconly_12bpc`), then the two passes. The 12 bpc first passes are the 10 bpc ones (`jmp
/// m(i<type1>_8x8_internal_10bpc).pass1`) with the 20-bit clipping range.
#[inline(always)]
unsafe fn inv_txfm_add_8x8_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[2] = pw(0x7000); // vpbroadcastd m2, [dconly_12bpc]
        return inv_txfm_add_dct_dct_8x8_10bpc_dconly(m, dst, stride, c, eob); // jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly
    }
    match t1 {
        K::Dct => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            idct_8x8_internal_10bpc_pass1(m, c) // jmp m(idct_8x8_internal_10bpc).pass1
        }
        K::Adst => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            iadst_8x8_internal_10bpc_pass1(m, c) // jmp m(iadst_8x8_internal_10bpc).pass1
        }
        K::FlipAdst => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            iflipadst_8x8_internal_10bpc_pass1(m, c) // jmp m(iflipadst_8x8_internal_10bpc).pass1
        }
        K::Identity => iidentity_8x8_internal_10bpc_pass1(m, c), // jmp m(iidentity_8x8_internal_10bpc).pass1
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x8_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_8x8_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x8_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x8_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro IADST8_1D src[1-8], tmp[1-3], pd_2048, clip[1-2]`
#[inline(always)]
unsafe fn iadst8_1d(m: &mut M, r: [usize; 8], t: [usize; 3], rnd: usize, clip: [usize; 2]) {
    let [r1, r2, r3, r4, r5, r6, r7, r8] = r;
    let [t1, t2, t3] = t;
    let [cmin, cmax] = clip;
    itx_mulsub_2d(m, r8, r1, t1, t2, t3, Some(rnd), 401, 4076, 0); // ITX_MULSUB_2D %8, %1, %9, %10, %11, %12, 401, 4076 ; t1a, t0a
    itx_mulsub_2d(m, r2, r7, t1, t2, t3, Some(rnd), 3920, 1189, 0); // ITX_MULSUB_2D %2, %7, %9, %10, %11, %12, 3920, 1189 ; t7a, t6a
    itx_mulsub_2d(m, r6, r3, t1, t2, t3, Some(rnd), 1931, 3612, 0); // ITX_MULSUB_2D %6, %3, %9, %10, %11, %12, 1931, 3612 ; t3a, t2a
    itx_mulsub_2d(m, r4, r5, t1, t2, t3, Some(rnd), 3166, 2598, 0); // ITX_MULSUB_2D %4, %5, %9, %10, %11, %12, 3166, 2598 ; t5a, t4a
    m[t1] = _mm256_sub_epi32(m[r3], m[r7]); // psubd m%9, m%3, m%7 ; t6
    m[r3] = _mm256_add_epi32(m[r3], m[r7]); // paddd m%3, m%7 ; t2
    m[r7] = _mm256_sub_epi32(m[r1], m[r5]); // psubd m%7, m%1, m%5 ; t4
    m[r1] = _mm256_add_epi32(m[r1], m[r5]); // paddd m%1, m%5 ; t0
    m[r5] = _mm256_sub_epi32(m[r6], m[r2]); // psubd m%5, m%6, m%2 ; t7
    m[r6] = _mm256_add_epi32(m[r6], m[r2]); // paddd m%6, m%2 ; t3
    m[r2] = _mm256_sub_epi32(m[r8], m[r4]); // psubd m%2, m%8, m%4 ; t5
    m[r8] = _mm256_add_epi32(m[r8], m[r4]); // paddd m%8, m%4 ; t1
    for x in [r7, r2, t1, r5, r3, r1, r6, r8] {
        m[x] = _mm256_max_epi32(m[x], m[cmin]); // REPX {pmaxsd x, m%13}, m%7, m%2, m%9, m%5, m%3, m%1, m%6, m%8
    }
    for x in [r7, r2, t1, r5, r3, r1, r6, r8] {
        m[x] = _mm256_min_epi32(m[x], m[cmax]); // REPX {pminsd x, m%14}, m%7, m%2, m%9, m%5, m%3, m%1, m%6, m%8
    }
    itx_mulsub_2d(m, r7, r2, r4, t2, t3, Some(rnd), 1567, 3784, 0); // ITX_MULSUB_2D %7, %2, %4, %10, %11, %12, 1567, 3784 ; t5a, t4a
    itx_mulsub_2d(m, r5, t1, r4, t2, t3, Some(rnd), 3784, t3 as i32, 0); // ITX_MULSUB_2D %5, %9, %4, %10, %11, %12, 3784, %11 ; t6a, t7a
    m[t2] = _mm256_sub_epi32(m[r7], m[t1]); // psubd m%10, m%7, m%9 ; t7
    m[r7] = _mm256_add_epi32(m[r7], m[t1]); // paddd m%7, m%9 ; out6
    m[t1] = pd(1448); // vpbroadcastd m%9, [pd_1448]
    m[r4] = _mm256_sub_epi32(m[r8], m[r6]); // psubd m%4, m%8, m%6 ; t3
    m[r8] = _mm256_add_epi32(m[r8], m[r6]); // paddd m%8, m%6 ; -out7
    m[r6] = _mm256_sub_epi32(m[r1], m[r3]); // psubd m%6, m%1, m%3 ; t2
    m[r1] = _mm256_add_epi32(m[r1], m[r3]); // paddd m%1, m%3 ; out0
    m[r3] = _mm256_sub_epi32(m[r2], m[r5]); // psubd m%3, m%2, m%5 ; t6
    m[r2] = _mm256_add_epi32(m[r2], m[r5]); // paddd m%2, m%5 ; -out1
    for x in [r6, r4, r3, t2] {
        m[x] = _mm256_max_epi32(m[x], m[cmin]); // REPX {pmaxsd x, m%13}, m%6, m%4, m%3, m%10
    }
    for x in [r6, r4, r3, t2] {
        m[x] = _mm256_min_epi32(m[x], m[cmax]); // REPX {pminsd x, m%14}, m%6, m%4, m%3, m%10
    }
    for x in [r6, r4, r3, t2] {
        m[x] = _mm256_mullo_epi32(m[x], m[t1]); // REPX {pmulld x, m%9 }, m%6, m%4, m%3, m%10
    }
    m[r5] = _mm256_sub_epi32(m[r6], m[r4]); // psubd m%5, m%6, m%4 ; (t2 - t3) * 1448
    m[r4] = _mm256_add_epi32(m[r4], m[r6]); // paddd m%4, m%6 ; (t2 + t3) * 1448
    m[r6] = _mm256_sub_epi32(m[r3], m[t2]); // psubd m%6, m%3, m%10 ; (t6 - t7) * 1448
    m[r3] = _mm256_add_epi32(m[r3], m[t2]); // paddd m%3, m%10 ; (t6 + t7) * 1448
}

/// `.pass1` of `idct_8x8_internal_10bpc` up to `jmp tx2q` (expects `m12`/`m13` = the clipping range)
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_loadu_si256(at(c, 32 * 0).cast()); // mova m0, [cq+32*0]
    m[1] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m1, [cq+32*1]
    m[2] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m2, [cq+32*2]
    m[3] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m3, [cq+32*3]
    m[4] = _mm256_loadu_si256(at(c, 32 * 4).cast()); // mova m4, [cq+32*4]
    m[5] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m5, [cq+32*5]
    m[6] = _mm256_loadu_si256(at(c, 32 * 6).cast()); // mova m6, [cq+32*6]
    m[7] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m7, [cq+32*7]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    idct_8x8_internal_10bpc_main(m); // call .main
    idct_8x8_internal_10bpc_round_shift1(m); // call .round_shift1
}

/// `.pass2` of `idct_8x8_internal_10bpc`
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    idct_8x8_internal_10bpc_transpose_8x8_packed(m); // call .transpose_8x8_packed
    idct_8x8_internal_8bpc_main(m); // call m(idct_8x8_internal_8bpc).main
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m3, m3, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4_start(m, stride, &mut r6, &mut dst, &mut c); // call .write_8x4_start
    m[0] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m0, m2, m12
    m[1] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m1, m3, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call .write_8x4
}

/// `.write_8x4_start` of `idct_8x8_internal_10bpc` (falls into `.write_8x4`): sets `m11` = `pixel_10bpc_max`,
/// `r6` = `stride * 3`, `m10` = 0, then adds `m0`/`m1` (four rows of eight pixels) to `dst`, clears the
/// 128 bytes at `c` and advances `dst` by four rows and `c` by 128 bytes.
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_write_8x4_start(
    m: &mut M,
    stride: isize,
    r6: &mut isize,
    dst: &mut Dst,
    c: &mut Coef,
) {
    m[11] = pw(0x3ff); // vpbroadcastd m11, [pixel_10bpc_max]
    *r6 = stride * 3; // lea r6, [strideq*3]
    m[10] = _mm256_setzero_si256(); // pxor m10, m10
    idct_8x8_internal_10bpc_write_8x4(m, dst, stride, *r6, c)
}

/// `.write_8x4` of `idct_8x8_internal_10bpc`: expects `m10` = 0, `m11` = the pixel maximum, `r6` = `stride * 3`;
/// adds `m0` (rows 0, 1) and `m1` (rows 2, 3) to `dst`, clears the 128 bytes at `c`, advances `dst` by four
/// rows and `c` by 128 bytes (`m8`, `m9` are clobbered).
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_write_8x4(m: &mut M, dst: &mut Dst, stride: isize, r6: isize, c: &mut Coef) {
    m[8] = zx(_mm_loadu_si128((*dst).cast())); // mova xm8, [dstq+strideq*0]
    m[8] = _mm256_inserti128_si256::<1>(m[8], _mm_loadu_si128(at(*dst, stride).cast())); // vinserti128 m8, [dstq+strideq*1], 1
    m[9] = zx(_mm_loadu_si128(at(*dst, stride * 2).cast())); // mova xm9, [dstq+strideq*2]
    m[9] = _mm256_inserti128_si256::<1>(m[9], _mm_loadu_si128(at(*dst, r6).cast())); // vinserti128 m9, [dstq+r6 ], 1
    _mm256_storeu_si256(at(*c, 32 * 0).cast(), m[10]); // mova [cq+32*0], m10
    _mm256_storeu_si256(at(*c, 32 * 1).cast(), m[10]); // mova [cq+32*1], m10
    _mm256_storeu_si256(at(*c, 32 * 2).cast(), m[10]); // mova [cq+32*2], m10
    _mm256_storeu_si256(at(*c, 32 * 3).cast(), m[10]); // mova [cq+32*3], m10
    *c = at(*c, 32 * 4); // add cq, 32*4
    m[0] = _mm256_add_epi16(m[0], m[8]); // paddw m0, m8
    m[1] = _mm256_add_epi16(m[1], m[9]); // paddw m1, m9
    m[0] = _mm256_max_epi16(m[0], m[10]); // pmaxsw m0, m10
    m[1] = _mm256_max_epi16(m[1], m[10]); // pmaxsw m1, m10
    m[0] = _mm256_min_epi16(m[0], m[11]); // pminsw m0, m11
    m[1] = _mm256_min_epi16(m[1], m[11]); // pminsw m1, m11
    _mm_storeu_si128((*dst).cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
    _mm_storeu_si128(at(*dst, stride).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
    _mm_storeu_si128(at(*dst, stride * 2).cast(), lo(m[1])); // mova [dstq+strideq*2], xm1
    _mm_storeu_si128(at(*dst, r6).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+r6 ], m1, 1
    *dst = at(*dst, stride * 4); // lea dstq, [dstq+strideq*4]
}

/// `.transpose_8x8_packed` of `idct_8x8_internal_10bpc`
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_transpose_8x8_packed(m: &mut M) {
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
                                           // lea r6, [deint_shuf+128] (the 8 bpc file's `o_base`: no code here)
    m[4] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m4, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpackhi_epi32(m[4], m[1]); // punpckhdq m2, m4, m1
    m[4] = _mm256_unpacklo_epi32(m[4], m[1]); // punpckldq m4, m1
    m[1] = _mm256_inserti128_si256::<1>(m[3], lo(m[2])); // vinserti128 m1, m3, xm2, 1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[3], m[2]); // vperm2i128 m3, m2, 0x31
    m[2] = _mm256_permute2x128_si256::<0x31>(m[0], m[4]); // vperm2i128 m2, m0, m4, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m0, xm4, 1
}

/// `.main_rect2` of `idct_8x8_internal_10bpc` (falls into `.main`): expects `m11` = `pd_2048`, `m12`/`m13` =
/// the clipping range.
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_main_rect2(m: &mut M) {
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x8_internal_10bpc_main(m)
}

/// `.main` of `idct_8x8_internal_10bpc`: expects `m0`-`m7` = in0-in7, `m11` = `pd_2048`, `m12`/`m13` = the
/// clipping range; leaves the idct4 outputs in `m0`, `m6`, `m5`, `m3`, the odd half in `m7`, `m4`, `m9`, `m8`.
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_main(m: &mut M) {
    itx_mulsub_2d(m, 5, 3, 8, 9, 10, Some(11), 3406, 2276, 0); // ITX_MULSUB_2D 5, 3, 8, 9, 10, 11, 3406, 2276 ; t5a t6a
    itx_mulsub_2d(m, 1, 7, 8, 9, 10, Some(11), 799, 4017, 0); // ITX_MULSUB_2D 1, 7, 8, 9, 10, 11, 799, 4017 ; t4a t7a
    itx_mulsub_2d(m, 2, 6, 8, 9, 10, Some(11), 1567, 3784, 0); // ITX_MULSUB_2D 2, 6, 8, 9, 10, 11, 1567, 3784 ; t2  t3
    m[8] = _mm256_add_epi32(m[1], m[5]); // paddd m8, m1, m5 ; t4
    m[1] = _mm256_sub_epi32(m[1], m[5]); // psubd m1, m5 ; t5a
    m[9] = _mm256_add_epi32(m[7], m[3]); // paddd m9, m7, m3 ; t7
    m[7] = _mm256_sub_epi32(m[7], m[3]); // psubd m7, m3 ; t6a
    m[3] = pd(2896); // vpbroadcastd m3, [pd_2896]
    for x in [1, 8, 7, 9] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m1, m8, m7, m9
    }
    for x in [1, 8, 7, 9] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m1, m8, m7, m9
    }
    for x in [0, 4, 7, 1] {
        m[x] = _mm256_mullo_epi32(m[x], m[3]); // REPX {pmulld x, m3 }, m0, m4, m7, m1
    }
    m[0] = _mm256_add_epi32(m[0], m[11]); // paddd m0, m11
    m[7] = _mm256_add_epi32(m[7], m[11]); // paddd m7, m11
    m[5] = _mm256_sub_epi32(m[0], m[4]); // psubd m5, m0, m4
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4
    m[4] = _mm256_sub_epi32(m[7], m[1]); // psubd m4, m7, m1
    m[7] = _mm256_add_epi32(m[7], m[1]); // paddd m7, m1
    for x in [5, 0, 4, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m5, m0, m4, m7
    }
    m[3] = _mm256_sub_epi32(m[0], m[6]); // psubd m3, m0, m6 ; dct4 out3
    m[0] = _mm256_add_epi32(m[0], m[6]); // paddd m0, m6 ; dct4 out0
    m[6] = _mm256_add_epi32(m[5], m[2]); // paddd m6, m5, m2 ; dct4 out1
    m[5] = _mm256_sub_epi32(m[5], m[2]); // psubd m5, m2 ; dct4 out2
    for x in [0, 6, 5, 3] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m6, m5, m3
    }
    for x in [0, 6, 5, 3] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m6, m5, m3
    }
}

/// `.round_shift1` of `idct_8x8_internal_10bpc`: the butterflies after `.main` and `(x + 1) >> 1`.
#[inline(always)]
unsafe fn idct_8x8_internal_10bpc_round_shift1(m: &mut M) {
    m[1] = _mm256_set1_epi32(-1); // pcmpeqd m1, m1
    for x in [0, 6, 5, 3] {
        m[x] = _mm256_sub_epi32(m[x], m[1]); // REPX {psubd x, m1}, m0, m6, m5, m3
    }
    m[1] = _mm256_add_epi32(m[6], m[7]); // paddd m1, m6, m7 ; out1
    m[6] = _mm256_sub_epi32(m[6], m[7]); // psubd m6, m7 ; out6
    m[7] = _mm256_sub_epi32(m[0], m[9]); // psubd m7, m0, m9 ; out7
    m[0] = _mm256_add_epi32(m[0], m[9]); // paddd m0, m9 ; out0
    m[2] = _mm256_add_epi32(m[5], m[4]); // paddd m2, m5, m4 ; out2
    m[5] = _mm256_sub_epi32(m[5], m[4]); // psubd m5, m4 ; out5
    m[4] = _mm256_sub_epi32(m[3], m[8]); // psubd m4, m3, m8 ; out4
    m[3] = _mm256_add_epi32(m[3], m[8]); // paddd m3, m8 ; out3
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass1` of `iadst_8x8_internal_10bpc` up to `jmp tx2q` (expects `m12`/`m13` = the clipping range)
#[inline(always)]
unsafe fn iadst_8x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_8x8_internal_10bpc_main(m, c); // call .main
    iadst_8x8_internal_10bpc_main_end(m); // call .main_end
}

/// `.pass2` of `iadst_8x8_internal_10bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    idct_8x8_internal_10bpc_transpose_8x8_packed(m); // call m(idct_8x8_internal_10bpc).transpose_8x8_packed
    m[4] = _mm256_shuffle_epi32::<0x4e>(m[0]); // pshufd m4, m0, q1032
    m[5] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m5, m1, q1032
    iadst_8x8_internal_8bpc_main_pass2(m); // call m(iadst_8x8_internal_8bpc).main_pass2
    m[5] = pw(2048); // vpbroadcastd m5, [pw_2048]
    m[12] = zx(lo(pw(4096))); // vpbroadcastd xm12, [pw_4096]
    m[12] = _mm256_sub_epi16(m[12], m[5]); // psubw m12, m5
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    }
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4_start(m, stride, &mut r6, &mut dst, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4_start
    m[0] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m0, m2, m12
    m[1] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m1, m3, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `.main` of `iadst_8x8_internal_10bpc` (falls into `.main2`): loads `c`, expects `m12`/`m13` = the clipping
/// range.
#[inline(always)]
unsafe fn iadst_8x8_internal_10bpc_main(m: &mut M, c: Coef) {
    m[0] = _mm256_loadu_si256(at(c, 32 * 0).cast()); // mova m0, [cq+32*0]
    m[7] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m7, [cq+32*7]
    m[1] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m1, [cq+32*1]
    m[6] = _mm256_loadu_si256(at(c, 32 * 6).cast()); // mova m6, [cq+32*6]
    m[2] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m2, [cq+32*2]
    m[5] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m5, [cq+32*5]
    m[3] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m3, [cq+32*3]
    m[4] = _mm256_loadu_si256(at(c, 32 * 4).cast()); // mova m4, [cq+32*4]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    iadst_8x8_internal_10bpc_main2(m)
}

/// `.main2` of `iadst_8x8_internal_10bpc`: expects `m0`-`m7` = in0-in7, `m11` = `pd_2048`, `m12`/`m13` = the
/// clipping range; leaves `m8` = `pd_1`, `m9` = `pd_3072`.
#[inline(always)]
unsafe fn iadst_8x8_internal_10bpc_main2(m: &mut M) {
    iadst8_1d(m, [0, 1, 2, 3, 4, 5, 6, 7], [8, 9, 10], 11, [12, 13]); // IADST8_1D 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13
    m[8] = _mm256_srli_epi32::<10>(m[8]); // psrld m8, 10 ; pd_1
    m[9] = pd(3072); // vpbroadcastd m9, [pd_3072]
}

/// `.main_end` of `iadst_8x8_internal_10bpc` (expects `m8` = `pd_1`, `m9` = `pd_3072` from `.main2`)
#[inline(always)]
unsafe fn iadst_8x8_internal_10bpc_main_end(m: &mut M) {
    m[0] = _mm256_add_epi32(m[0], m[8]); // paddd m0, m8
    m[1] = _mm256_sub_epi32(m[8], m[1]); // psubd m1, m8, m1
    m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8
    m[7] = _mm256_sub_epi32(m[8], m[7]); // psubd m7, m8, m7
    for x in [0, 1, 6, 7] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m0, m1, m6, m7
    }
    // ; (1 + ((x + 1024) >> 11)) >> 1 = (3072 + x) >> 12
    // ; (1 - ((x + 1024) >> 11)) >> 1 = (3071 - x) >> 12
    m[8] = _mm256_sub_epi32(m[9], m[8]); // psubd m8, m9, m8 ; pd_3071
    m[2] = _mm256_add_epi32(m[2], m[9]); // paddd m2, m9
    m[3] = _mm256_sub_epi32(m[8], m[3]); // psubd m3, m8, m3
    m[4] = _mm256_add_epi32(m[4], m[9]); // paddd m4, m9
    m[5] = _mm256_sub_epi32(m[8], m[5]); // psubd m5, m8, m5
    for x in [2, 3, 4, 5] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m2, m3, m4, m5
    }
}

/// `.pass1` of `iflipadst_8x8_internal_10bpc` up to `jmp tx2q` (expects `m12`/`m13` = the clipping range)
#[inline(always)]
unsafe fn iflipadst_8x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_8x8_internal_10bpc_main(m, c); // call m(iadst_8x8_internal_10bpc).main
    iflipadst_8x8_internal_10bpc_main_end(m); // call .main_end
}

/// `.pass2` of `iflipadst_8x8_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_8x8_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    idct_8x8_internal_10bpc_transpose_8x8_packed(m); // call m(idct_8x8_internal_10bpc).transpose_8x8_packed
    m[4] = _mm256_shuffle_epi32::<0x4e>(m[0]); // pshufd m4, m0, q1032
    m[5] = _mm256_shuffle_epi32::<0x4e>(m[1]); // pshufd m5, m1, q1032
    iadst_8x8_internal_8bpc_main_pass2(m); // call m(iadst_8x8_internal_8bpc).main_pass2
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[5] = zx(lo(pw(4096))); // vpbroadcastd xm5, [pw_4096]
    m[12] = _mm256_sub_epi16(m[12], m[5]); // psubw m12, m5
    m[8] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m8, m3, q2031
    m[9] = _mm256_permute4x64_epi64::<0x8d>(m[2]); // vpermq m9, m2, q2031
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m2, m1, q2031
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[0]); // vpermq m3, m0, q2031
    m[0] = _mm256_mulhrs_epi16(m[8], m[12]); // pmulhrsw m0, m8, m12
    m[1] = _mm256_mulhrs_epi16(m[9], m[12]); // pmulhrsw m1, m9, m12
    idct_8x8_internal_10bpc_write_8x4_start(m, stride, &mut r6, &mut dst, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4_start
    m[0] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m0, m2, m12
    m[1] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m1, m3, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `.main_end` of `iflipadst_8x8_internal_10bpc` (expects `m8` = `pd_1`, `m9` = `pd_3072` from `.main2`)
#[inline(always)]
unsafe fn iflipadst_8x8_internal_10bpc_main_end(m: &mut M) {
    m[10] = _mm256_add_epi32(m[8], m[0]); // paddd m10, m8, m0
    m[0] = _mm256_sub_epi32(m[8], m[7]); // psubd m0, m8, m7
    m[7] = _mm256_sub_epi32(m[8], m[1]); // psubd m7, m8, m1
    m[1] = _mm256_add_epi32(m[8], m[6]); // paddd m1, m8, m6
    m[0] = _mm256_srai_epi32::<1>(m[0]); // psrad m0, 1
    m[1] = _mm256_srai_epi32::<1>(m[1]); // psrad m1, 1
    m[6] = _mm256_srai_epi32::<1>(m[7]); // psrad m6, m7, 1
    m[7] = _mm256_srai_epi32::<1>(m[10]); // psrad m7, m10, 1
    m[8] = _mm256_sub_epi32(m[9], m[8]); // psubd m8, m9, m8 ; pd_6143
    m[10] = _mm256_sub_epi32(m[8], m[5]); // psubd m10, m8, m5
    m[5] = _mm256_add_epi32(m[9], m[2]); // paddd m5, m9, m2
    m[2] = _mm256_sub_epi32(m[8], m[3]); // psubd m2, m8, m3
    m[3] = _mm256_add_epi32(m[9], m[4]); // paddd m3, m9, m4
    m[4] = _mm256_srai_epi32::<12>(m[2]); // psrad m4, m2, 12
    m[2] = _mm256_srai_epi32::<12>(m[10]); // psrad m2, m10, 12
    m[3] = _mm256_srai_epi32::<12>(m[3]); // psrad m3, 12
    m[5] = _mm256_srai_epi32::<12>(m[5]); // psrad m5, 12
}

/// `.pass1` of `iidentity_8x8_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_8x8_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[0] = _mm256_loadu_si256(at(c, 32 * 0).cast()); // mova m0, [cq+32*0]
    m[1] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m1, [cq+32*1]
    m[2] = _mm256_loadu_si256(at(c, 32 * 2).cast()); // mova m2, [cq+32*2]
    m[3] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m3, [cq+32*3]
    m[4] = _mm256_loadu_si256(at(c, 32 * 4).cast()); // mova m4, [cq+32*4]
    m[5] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m5, [cq+32*5]
    m[6] = _mm256_loadu_si256(at(c, 32 * 6).cast()); // mova m6, [cq+32*6]
    m[7] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m7, [cq+32*7]
}

/// `.pass2` of `iidentity_8x8_internal_10bpc`
#[inline(always)]
unsafe fn iidentity_8x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    iidentity_8x8_internal_10bpc_pass2_main(m, dst, stride, c)
}

/// `.pass2_main` of `iidentity_8x8_internal_10bpc` to its `RET`: expects `m0`-`m6` = the rows (32-bit), `m3`
/// already packed with `m7`, `m7` = the pixel maximum.
#[inline(always)]
unsafe fn iidentity_8x8_internal_10bpc_pass2_main(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[12] = pw(4096); // vpbroadcastd m12, [pw_4096]
    m[4] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m4, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[1]); // punpckldq m2, m4, m1
    m[4] = _mm256_unpackhi_epi32(m[4], m[1]); // punpckhdq m4, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2 ; 1 5
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2 ; 0 4
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4 ; 2 6
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4 ; 3 7
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    iidentity_8x8_internal_10bpc_write_2x8x2_start(m, stride, &mut r6, &mut c, &mut dst); // call .write_2x8x2_start
    m[0] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m0, m2, m12
    m[1] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m1, m3, m12
    iidentity_8x8_internal_10bpc_write_2x8x2_zero(m, &mut c, &mut dst, stride, r6);
    // call .write_2x8x2_zero
}

/// `.write_2x8x2_start` of `iidentity_8x8_internal_10bpc` (falls into `.write_2x8x2_zero`): sets `r6` =
/// `stride * 5`, `m6` = 0.
#[inline(always)]
unsafe fn iidentity_8x8_internal_10bpc_write_2x8x2_start(
    m: &mut M,
    stride: isize,
    r6: &mut isize,
    c: &mut Coef,
    dst: &mut Dst,
) {
    *r6 = stride * 5; // lea r6, [strideq*5]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    iidentity_8x8_internal_10bpc_write_2x8x2_zero(m, c, dst, stride, *r6)
}

/// `.write_2x8x2_zero` of `iidentity_8x8_internal_10bpc` (falls into `.write_2x8x2`): clears the 128 bytes at
/// `c` (with `m6` = 0) and advances `c` by 128 bytes.
#[inline(always)]
unsafe fn iidentity_8x8_internal_10bpc_write_2x8x2_zero(
    m: &mut M,
    c: &mut Coef,
    dst: &mut Dst,
    stride: isize,
    r6: isize,
) {
    _mm256_storeu_si256(at(*c, 32 * 0).cast(), m[6]); // mova [cq+32*0], m6
    _mm256_storeu_si256(at(*c, 32 * 1).cast(), m[6]); // mova [cq+32*1], m6
    _mm256_storeu_si256(at(*c, 32 * 2).cast(), m[6]); // mova [cq+32*2], m6
    _mm256_storeu_si256(at(*c, 32 * 3).cast(), m[6]); // mova [cq+32*3], m6
    *c = at(*c, 32 * 4); // add cq, 32*4
    iidentity_8x8_internal_10bpc_write_2x8x2(m, dst, stride, r6)
}

/// `.write_2x8x2` of `iidentity_8x8_internal_10bpc`: expects `m6` = 0, `m7` = the pixel maximum, `r6` =
/// `stride * 5`; adds `m0` (rows 0, 4) and `m1` (rows 1, 5) to `dst` and advances `dst` by two rows.
#[inline(always)]
unsafe fn iidentity_8x8_internal_10bpc_write_2x8x2(m: &mut M, dst: &mut Dst, stride: isize, r6: isize) {
    m[4] = zx(_mm_loadu_si128((*dst).cast())); // mova xm4, [dstq+strideq*0]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(*dst, stride * 4).cast())); // vinserti128 m4, [dstq+strideq*4], 1
    m[5] = zx(_mm_loadu_si128(at(*dst, stride).cast())); // mova xm5, [dstq+strideq*1]
    m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_loadu_si128(at(*dst, r6).cast())); // vinserti128 m5, [dstq+r6 ], 1
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4
    m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
    m[0] = _mm256_max_epi16(m[0], m[6]); // pmaxsw m0, m6
    m[1] = _mm256_max_epi16(m[1], m[6]); // pmaxsw m1, m6
    m[0] = _mm256_min_epi16(m[0], m[7]); // pminsw m0, m7
    m[1] = _mm256_min_epi16(m[1], m[7]); // pminsw m1, m7
    _mm_storeu_si128((*dst).cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
    _mm_storeu_si128(at(*dst, stride).cast(), lo(m[1])); // mova [dstq+strideq*1], xm1
    _mm_storeu_si128(at(*dst, stride * 4).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*4], m0, 1
    _mm_storeu_si128(at(*dst, r6).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+r6 ], m1, 1
    *dst = at(*dst, stride * 2); // lea dstq, [dstq+strideq*2]
}

/// `.pass2` of `idct_8x8_internal_12bpc`
#[inline(always)]
unsafe fn idct_8x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x8_internal_12bpc_transpose_8x8(m); // call .transpose_8x8
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x8_internal_12bpc_round_shift4(m); // call .round_shift4
    iadst_8x8_internal_12bpc_pass2_end(m, dst, stride, c) // jmp m(iadst_8x8_internal_12bpc).pass2_end
}

/// `.write_8x4_start` of `idct_8x8_internal_12bpc`: sets `m11` = `pixel_12bpc_max`, `r6` = `stride * 3`,
/// `m10` = 0 (then call `idct_8x8_internal_10bpc_write_8x4`).
#[inline(always)]
unsafe fn idct_8x8_internal_12bpc_write_8x4_start(m: &mut M, stride: isize, r6: &mut isize) {
    m[11] = pw(0xfff); // vpbroadcastd m11, [pixel_12bpc_max]
    *r6 = stride * 3; // lea r6, [strideq*3]
    m[10] = _mm256_setzero_si256(); // pxor m10, m10
}

/// `.transpose_8x8` of `idct_8x8_internal_12bpc`: `m0`-`m7` transposed as 8x8 dwords (`m8`-`m11` clobbered).
#[inline(always)]
unsafe fn idct_8x8_internal_12bpc_transpose_8x8(m: &mut M) {
    transpose_8x8_dword(m, [0, 1, 2, 3, 4, 5, 6, 7], [8, 9, 10, 11]); // TRANSPOSE_8X8_DWORD 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
}

/// `.round_shift4` of `idct_8x8_internal_12bpc`: the butterflies after `.main` and `(x + 8) >> 4`.
#[inline(always)]
unsafe fn idct_8x8_internal_12bpc_round_shift4(m: &mut M) {
    m[1] = pd(8); // vpbroadcastd m1, [pd_8]
    for x in [0, 6, 5, 3] {
        m[x] = _mm256_add_epi32(m[x], m[1]); // REPX {paddd x, m1}, m0, m6, m5, m3
    }
    m[1] = _mm256_add_epi32(m[6], m[7]); // paddd m1, m6, m7 ; out1
    m[6] = _mm256_sub_epi32(m[6], m[7]); // psubd m6, m7 ; out6
    m[7] = _mm256_sub_epi32(m[0], m[9]); // psubd m7, m0, m9 ; out7
    m[0] = _mm256_add_epi32(m[0], m[9]); // paddd m0, m9 ; out0
    m[2] = _mm256_add_epi32(m[5], m[4]); // paddd m2, m5, m4 ; out2
    m[5] = _mm256_sub_epi32(m[5], m[4]); // psubd m5, m4 ; out5
    m[4] = _mm256_sub_epi32(m[3], m[8]); // psubd m4, m3, m8 ; out4
    m[3] = _mm256_add_epi32(m[3], m[8]); // paddd m3, m8 ; out3
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4}, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass2` of `iadst_8x8_internal_12bpc`
#[inline(always)]
unsafe fn iadst_8x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_8x8_internal_12bpc_pass2_main(m); // call .pass2_main
    iadst_8x8_internal_12bpc_pass2_end(m, dst, stride, c)
}

/// `.pass2_end` of `iadst_8x8_internal_12bpc` to its `RET`: packs `m0`-`m7` (32-bit rows) and writes them.
#[inline(always)]
unsafe fn iadst_8x8_internal_12bpc_pass2_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[1] = _mm256_packs_epi32(m[2], m[3]); // packssdw m1, m2, m3
    for x in [0, 1] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1
    }
    idct_8x8_internal_12bpc_write_8x4_start(m, stride, &mut r6); // call m(idct_8x8_internal_12bpc).write_8x4_start
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_packs_epi32(m[4], m[5]); // packssdw m0, m4, m5
    m[1] = _mm256_packs_epi32(m[6], m[7]); // packssdw m1, m6, m7
    for x in [0, 1] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1
    }
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `.pass2_main` of `iadst_8x8_internal_12bpc` (falls into `.pass2_main2`)
#[inline(always)]
unsafe fn iadst_8x8_internal_12bpc_pass2_main(m: &mut M) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x8_internal_12bpc_transpose_8x8(m); // call m(idct_8x8_internal_12bpc).transpose_8x8
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    iadst_8x8_internal_12bpc_pass2_main2(m)
}

/// `.pass2_main2` of `iadst_8x8_internal_12bpc`: expects `m0`-`m7` = in0-in7, `m11` = `pd_2048`, `m12`/`m13` =
/// the 18-bit clipping range; the adst8 with the 12 bpc rounding (`>> 4`, `>> 15`).
#[inline(always)]
unsafe fn iadst_8x8_internal_12bpc_pass2_main2(m: &mut M) {
    iadst_8x8_internal_10bpc_main2(m); // call m(iadst_8x8_internal_10bpc).main2
    m[9] = _mm256_slli_epi32::<3>(m[8]); // pslld m9, m8, 3 ; pd_8
    m[0] = _mm256_add_epi32(m[0], m[9]); // paddd m0, m9
    m[1] = _mm256_sub_epi32(m[9], m[1]); // psubd m1, m9, m1 ; 8+x
    m[6] = _mm256_add_epi32(m[6], m[9]); // paddd m6, m9
    m[7] = _mm256_sub_epi32(m[9], m[7]); // psubd m7, m9, m7
    for x in [0, 1, 6, 7] {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4}, m0, m1, m6, m7
    }
    m[9] = pd(17408); // vpbroadcastd m9, [pd_17408]
    m[8] = _mm256_sub_epi32(m[9], m[8]); // psubd m8, m9, m8 ; 17407
    m[2] = _mm256_add_epi32(m[2], m[9]); // paddd m2, m9
    m[3] = _mm256_sub_epi32(m[8], m[3]); // psubd m3, m8, m3
    m[4] = _mm256_add_epi32(m[4], m[9]); // paddd m4, m9
    m[5] = _mm256_sub_epi32(m[8], m[5]); // psubd m5, m8, m5
    for x in [2, 3, 4, 5] {
        m[x] = _mm256_srai_epi32::<15>(m[x]); // REPX {psrad x, 15}, m2, m3, m4, m5
    }
}

/// `.pass2` of `iflipadst_8x8_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_8x8_internal_12bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    iadst_8x8_internal_12bpc_pass2_main(m); // call m(iadst_8x8_internal_12bpc).pass2_main
    m[7] = _mm256_packs_epi32(m[7], m[6]); // packssdw m7, m7, m6
    m[6] = _mm256_packs_epi32(m[1], m[0]); // packssdw m6, m1, m0
    m[1] = _mm256_packs_epi32(m[5], m[4]); // packssdw m1, m5, m4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[7]); // vpermq m0, m7, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
    idct_8x8_internal_12bpc_write_8x4_start(m, stride, &mut r6); // call m(idct_8x8_internal_12bpc).write_8x4_start
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_packs_epi32(m[3], m[2]); // packssdw m0, m3, m2
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[6]); // vpermq m1, m6, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `.pass2` of `iidentity_8x8_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_8x8_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    iidentity_8x8_internal_10bpc_pass2_main(m, dst, stride, c) // jmp m(iidentity_8x8_internal_10bpc).pass2_main
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x8_10bpc_avx2,
    inv_txfm_add_8x8_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x8_12bpc_avx2,
    inv_txfm_add_8x8_12bpc,
    K::Identity,
    K::Identity
);

// --- 8x16 --------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_8X16_FN type1, type2, eob_offset, 10`: the DC-only path of dct_dct (into 8x8's `.dconly2`),
/// the `eob_offset` 35 of dct/adst/flipadst with identity, then the two passes. The `vpbroadcastd m12/m13,
/// [clip_18b_*]` before each `.pass1:` label are here.
#[inline(always)]
unsafe fn inv_txfm_add_8x16_10bpc(dst: Dst, stride: isize, c: Coef, mut eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[2] = pw(0x7c00); // vpbroadcastd m2, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 16; // or r3d, 16
        r6d = r6d.wrapping_add(128); // add r6d, 128
        r6d >>= 8; // sar r6d, 8
        r6d = r6d.wrapping_mul(181); // imul r6d, 181
        return inv_txfm_add_dct_dct_8x8_10bpc_dconly2(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly2
    }
    if t2 == K::Identity && t1 != K::Identity {
        eob = eob.wrapping_add(35); // add eobd, 35
    }
    match t1 {
        K::Dct => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            idct_8x16_internal_10bpc_pass1(m, c, eob)
        }
        K::Adst => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            iadst_8x16_internal_10bpc_pass1(m, c, eob)
        }
        K::FlipAdst => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            iflipadst_8x16_internal_10bpc_pass1(m, c, eob)
        }
        K::Identity => iidentity_8x16_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x16_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_8x16_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x16_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x16_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro INV_TXFM_8X16_FN type1, type2, eob_offset, 12`: as at 10 bpc, with `dconly_12bpc` and the 20-bit
/// clipping range in the first passes (`jmp m(i<type1>_8x16_internal_10bpc).pass1`).
#[inline(always)]
unsafe fn inv_txfm_add_8x16_12bpc(dst: Dst, stride: isize, c: Coef, mut eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[2] = pw(0x7000); // vpbroadcastd m2, [dconly_12bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 16; // or r3d, 16
        r6d = r6d.wrapping_add(128); // add r6d, 128
        r6d >>= 8; // sar r6d, 8
        r6d = r6d.wrapping_mul(181); // imul r6d, 181
        return inv_txfm_add_dct_dct_8x8_10bpc_dconly2(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly2
    }
    if t2 == K::Identity && t1 != K::Identity {
        eob = eob.wrapping_add(35); // add eobd, 35
    }
    match t1 {
        K::Dct => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            idct_8x16_internal_10bpc_pass1(m, c, eob) // jmp m(idct_8x16_internal_10bpc).pass1
        }
        K::Adst => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            iadst_8x16_internal_10bpc_pass1(m, c, eob) // jmp m(iadst_8x16_internal_10bpc).pass1
        }
        K::FlipAdst => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            iflipadst_8x16_internal_10bpc_pass1(m, c, eob) // jmp m(iflipadst_8x16_internal_10bpc).pass1
        }
        K::Identity => iidentity_8x16_internal_10bpc_pass1(m, c), // jmp m(iidentity_8x16_internal_10bpc).pass1
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_8x16_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_8x16_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_8x16_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_8x16_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `.pass1` of `idct_8x16_internal_10bpc` up to `jmp tx2q` (expects `m12`/`m13` = the clipping range). The
/// first call of `.pass1_main` keeps its rows in the coefficient buffer (`[cq+32*1]`, …) and `m15`.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_pass1(m: &mut M, c: Coef, eob: c_int) {
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    if eob >= 43 {
        // cmp eobd, 43 / jl .fast
        idct_8x16_internal_10bpc_pass1_main(m, at(c, 32)); // add cq, 32 / call .pass1_main / sub cq, 32
        _mm256_storeu_si256(at(c, 32 * 1).cast(), m[0]); // mova [cq+32* 1], m0
        _mm256_storeu_si256(at(c, 32 * 3).cast(), m[1]); // mova [cq+32* 3], m1
        _mm256_storeu_si256(at(c, 32 * 5).cast(), m[2]); // mova [cq+32* 5], m2
        _mm256_storeu_si256(at(c, 32 * 7).cast(), m[3]); // mova [cq+32* 7], m3
        _mm256_storeu_si256(at(c, 32 * 9).cast(), m[4]); // mova [cq+32* 9], m4
        _mm256_storeu_si256(at(c, 32 * 11).cast(), m[5]); // mova [cq+32*11], m5
        _mm256_storeu_si256(at(c, 32 * 13).cast(), m[6]); // mova [cq+32*13], m6
        m[15] = m[7]; // mova m15, m7
        idct_8x16_internal_10bpc_pass1_main(m, c); // call .pass1_main
        m[8] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m8, [cq+32* 1]
        m[9] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m9, [cq+32* 3]
        m[10] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m10, [cq+32* 5]
        m[11] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m11, [cq+32* 7]
        m[12] = _mm256_loadu_si256(at(c, 32 * 9).cast()); // mova m12, [cq+32* 9]
        m[13] = _mm256_loadu_si256(at(c, 32 * 11).cast()); // mova m13, [cq+32*11]
        m[14] = _mm256_loadu_si256(at(c, 32 * 13).cast()); // mova m14, [cq+32*13]
    } else {
        // .fast:
        idct_8x16_internal_10bpc_pass1_main(m, c); // call .pass1_main
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14, 15] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
        }
    }
}

/// `.pass2` of `idct_8x16_internal_10bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_8x16_internal_10bpc_transpose(m); // call .transpose
    idct_8x16_internal_8bpc_main(m); // call m(idct_8x16_internal_8bpc).main
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    for x in [0, 2, 4, 6] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m2, m4, m6
    }
    for x in [1, 3, 5, 7] {
        m[x] = _mm256_permute4x64_epi64::<0x8d>(m[x]); // REPX {vpermq x, x, q2031}, m1, m3, m5, m7
    }
    idct_8x16_internal_10bpc_end(m, dst, stride, c)
}

/// `.end` of `idct_8x16_internal_10bpc` to its `RET`: expects `m0`-`m7` = the packed rows, `m12` = the
/// rounding multipliers.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4_start(m, stride, &mut r6, &mut dst, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4_start
    m[0] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m0, m2, m12
    m[1] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m1, m3, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_mulhrs_epi16(m[4], m[12]); // pmulhrsw m0, m4, m12
    m[1] = _mm256_mulhrs_epi16(m[5], m[12]); // pmulhrsw m1, m5, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_mulhrs_epi16(m[6], m[12]); // pmulhrsw m0, m6, m12
    m[1] = _mm256_mulhrs_epi16(m[7], m[12]); // pmulhrsw m1, m7, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `.transpose` of `idct_8x16_internal_10bpc`: packs `m0`-`m15` (32-bit) to words and transposes them.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_transpose(m: &mut M) {
    m[0] = _mm256_packs_epi32(m[0], m[8]); // packssdw m0, m8
    m[1] = _mm256_packs_epi32(m[1], m[9]); // packssdw m1, m9
    m[2] = _mm256_packs_epi32(m[2], m[10]); // packssdw m2, m10
    m[3] = _mm256_packs_epi32(m[3], m[11]); // packssdw m3, m11
    m[4] = _mm256_packs_epi32(m[4], m[12]); // packssdw m4, m12
    m[5] = _mm256_packs_epi32(m[5], m[13]); // packssdw m5, m13
    m[6] = _mm256_packs_epi32(m[6], m[14]); // packssdw m6, m14
    m[7] = _mm256_packs_epi32(m[7], m[15]); // packssdw m7, m15
                                            // lea r6, [deint_shuf+128] (the 8 bpc file's `o_base`: no code here)
    m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m3, m4, m5
    m[4] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m4, m5
    m[5] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m5, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpackhi_epi32(m[3], m[6]); // punpckhdq m7, m3, m6
    m[3] = _mm256_unpacklo_epi32(m[3], m[6]); // punpckldq m3, m6
    m[6] = _mm256_unpackhi_epi32(m[4], m[5]); // punpckhdq m6, m4, m5
    m[4] = _mm256_unpacklo_epi32(m[4], m[5]); // punpckldq m4, m5
    m[5] = _mm256_unpackhi_epi32(m[8], m[1]); // punpckhdq m5, m8, m1
    m[8] = _mm256_unpacklo_epi32(m[8], m[1]); // punpckldq m8, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m2, m0, m3, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m0, xm3, 1
    m[3] = _mm256_permute2x128_si256::<0x31>(m[1], m[7]); // vperm2i128 m3, m1, m7, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[7])); // vinserti128 m1, xm7, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[5], m[6]); // vperm2i128 m7, m5, m6, 0x31
    m[5] = _mm256_inserti128_si256::<1>(m[5], lo(m[6])); // vinserti128 m5, xm6, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[8], m[4]); // vperm2i128 m6, m8, m4, 0x31
    m[4] = _mm256_inserti128_si256::<1>(m[8], lo(m[4])); // vinserti128 m4, m8, xm4, 1
}

/// `.pass1_main` of `idct_8x16_internal_10bpc` (ends in `jmp m(idct_8x8_internal_10bpc).round_shift1`):
/// loads the even rows at `c`, expects `m14` = `pd_2896`, `m11` = `pd_2048`, `m12`/`m13` = the clipping range.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_pass1_main(m: &mut M, c: Coef) {
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmulld m0, m14, [cq+32* 0]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m1, m14, [cq+32* 2]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmulld m2, m14, [cq+32* 4]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 6).cast())); // pmulld m3, m14, [cq+32* 6]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 8).cast())); // pmulld m4, m14, [cq+32* 8]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 10).cast())); // pmulld m5, m14, [cq+32*10]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 12).cast())); // pmulld m6, m14, [cq+32*12]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 14).cast())); // pmulld m7, m14, [cq+32*14]
    idct_8x8_internal_10bpc_main_rect2(m); // call m(idct_8x8_internal_10bpc).main_rect2
    idct_8x8_internal_10bpc_round_shift1(m) // jmp m(idct_8x8_internal_10bpc).round_shift1
}

/// `.main_evenhalf` of `idct_8x16_internal_10bpc`: the butterflies after `idct_8x8_internal_10bpc_main`, clipped
/// to `m12`/`m13`.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_main_evenhalf(m: &mut M) {
    m[1] = _mm256_add_epi32(m[6], m[7]); // paddd m1, m6, m7 ; idct8 out1
    m[6] = _mm256_sub_epi32(m[6], m[7]); // psubd m6, m7 ; idct8 out6
    m[7] = _mm256_sub_epi32(m[0], m[9]); // psubd m7, m0, m9 ; idct8 out7
    m[0] = _mm256_add_epi32(m[0], m[9]); // paddd m0, m9 ; idct8 out0
    m[2] = _mm256_add_epi32(m[5], m[4]); // paddd m2, m5, m4 ; idct8 out2
    m[5] = _mm256_sub_epi32(m[5], m[4]); // psubd m5, m4 ; idct8 out5
    m[4] = _mm256_sub_epi32(m[3], m[8]); // psubd m4, m3, m8 ; idct8 out4
    m[3] = _mm256_add_epi32(m[3], m[8]); // paddd m3, m8 ; idct8 out3
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.main_oddhalf_fast_rect2` of `idct_8x16_internal_10bpc` (falls into `.main_oddhalf_fast`): as
/// `idct_8x16_internal_10bpc_main_oddhalf_fast`, after the `rect2` rounding of `m0`-`m3`.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_main_oddhalf_fast_rect2(m: &mut M, r6: *mut __m256i) {
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3
    }
    idct_8x16_internal_10bpc_main_oddhalf_fast(m, r6)
}

/// `.main_oddhalf_fast` of `idct_8x16_internal_10bpc` (the inputs 9-15 zero): as
/// `idct_8x16_internal_10bpc_main_oddhalf` with `m0`-`m3` = in1, in3, in5, in7.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_main_oddhalf_fast(m: &mut M, r6: *mut __m256i) {
    m[7] = pd(4076); // vpbroadcastd m7, [pd_4076]
    m[8] = pd(401); // vpbroadcastd m8, [pd_401]
    m[6] = pd(-1189); // vpbroadcastd m6, [pd_m1189]
    m[9] = pd(3920); // vpbroadcastd m9, [pd_3920]
    m[5] = pd(3612); // vpbroadcastd m5, [pd_3612]
    m[10] = pd(1931); // vpbroadcastd m10, [pd_1931]
    m[4] = pd(-2598); // vpbroadcastd m4, [pd_m2598]
    m[15] = pd(3166); // vpbroadcastd m15, [pd_3166]
    m[7] = _mm256_mullo_epi32(m[7], m[0]); // pmulld m7, m0
    m[0] = _mm256_mullo_epi32(m[0], m[8]); // pmulld m0, m8
    m[6] = _mm256_mullo_epi32(m[6], m[1]); // pmulld m6, m1
    m[1] = _mm256_mullo_epi32(m[1], m[9]); // pmulld m1, m9
    m[5] = _mm256_mullo_epi32(m[5], m[2]); // pmulld m5, m2
    m[2] = _mm256_mullo_epi32(m[2], m[10]); // pmulld m2, m10
    m[4] = _mm256_mullo_epi32(m[4], m[3]); // pmulld m4, m3
    m[3] = _mm256_mullo_epi32(m[3], m[15]); // pmulld m3, m15
    idct_8x16_internal_10bpc_main_oddhalf_fast2(m, r6) // jmp .main_oddhalf_fast2
}

/// `.main_oddhalf_rect2` of `idct_8x16_internal_10bpc` (falls into `.main_oddhalf`): as
/// `idct_8x16_internal_10bpc_main_oddhalf`, after the `rect2` rounding of `m0`-`m7`.
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_main_oddhalf_rect2(m: &mut M, r6: *mut __m256i) {
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x16_internal_10bpc_main_oddhalf(m, r6)
}

/// `.main_oddhalf` of `idct_8x16_internal_10bpc`: the odd half of the idct16. Expects `m0`-`m7` = in1, in3, …,
/// in15, `m11` = `pd_2048`, `m12`/`m13` = the clipping range, `m14` = `pd_2896`; writes t15a, t14, t13a, t12,
/// t11, t10a, t9, t8a to the eight 32-byte slots `[r6-32*4]` … `[r6+32*3]` (and leaves them in `m7`, `m6`, `m5`,
/// `m4`, `m3`, `m2`, `m1`, `m0`).
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_main_oddhalf(m: &mut M, r6: *mut __m256i) {
    itx_mulsub_2d(m, 0, 7, 8, 9, 10, None, 401, 4076, 0); // ITX_MULSUB_2D 0, 7, 8, 9, 10, _, 401, 4076 ; t8a,  t15a
    itx_mulsub_2d(m, 6, 1, 8, 9, 10, None, 3920, 1189, 0); // ITX_MULSUB_2D 6, 1, 8, 9, 10, _, 3920, 1189 ; t11a, t12a
    itx_mulsub_2d(m, 2, 5, 8, 9, 10, None, 1931, 3612, 0); // ITX_MULSUB_2D 2, 5, 8, 9, 10, _, 1931, 3612 ; t10a, t13a
    itx_mulsub_2d(m, 4, 3, 8, 9, 10, None, 3166, 2598, 0); // ITX_MULSUB_2D 4, 3, 8, 9, 10, _, 3166, 2598 ; t9a,  t14a
    idct_8x16_internal_10bpc_main_oddhalf_fast2(m, r6)
}

/// `.main_oddhalf_fast2` of `idct_8x16_internal_10bpc`
#[inline(always)]
unsafe fn idct_8x16_internal_10bpc_main_oddhalf_fast2(m: &mut M, r6: *mut __m256i) {
    const X: usize = usize::MAX; // the `_` of an unused macro argument
    for x in [0, 7, 6, 1, 2, 5, 4, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m7, m6, m1, m2, m5, m4, m3
    }
    for x in [0, 4, 6, 2, 1, 5, 7, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m4, m6, m2, m1, m5, m7, m3
    }
    m[8] = _mm256_sub_epi32(m[0], m[4]); // psubd m8, m0, m4 ; t9
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; t8
    m[4] = _mm256_sub_epi32(m[6], m[2]); // psubd m4, m6, m2 ; t10
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6 ; t11
    m[6] = _mm256_sub_epi32(m[1], m[5]); // psubd m6, m1, m5 ; t13
    m[5] = _mm256_add_epi32(m[5], m[1]); // paddd m5, m1 ; t12
    m[1] = _mm256_sub_epi32(m[7], m[3]); // psubd m1, m7, m3 ; t14
    m[7] = _mm256_add_epi32(m[7], m[3]); // paddd m7, m3 ; t15
    for x in [8, 1, 4, 6, 0, 2, 5, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m8, m1, m4, m6, m0, m2, m5, m7
    }
    for x in [8, 1, 4, 6, 0, 2, 5, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m8, m1, m4, m6, m0, m2, m5, m7
    }
    m[15] = pd(3784); // vpbroadcastd m15, [pd_3784]
    m[10] = pd(1567); // vpbroadcastd m10, [pd_1567]
    itx_mulsub_2d(m, 1, 8, 3, 9, X, Some(11), 10, 15, 0); // ITX_MULSUB_2D 1, 8, 3, 9, _, 11, 10, 15
    itx_mulsub_2d(m, 6, 4, 3, 9, X, Some(11), 10, 15, 2); // ITX_MULSUB_2D 6, 4, 3, 9, _, 11, 10, 15, 2
    m[3] = _mm256_sub_epi32(m[1], m[4]); // psubd m3, m1, m4 ; t10
    m[1] = _mm256_add_epi32(m[1], m[4]); // paddd m1, m4 ; t9
    m[4] = _mm256_sub_epi32(m[0], m[2]); // psubd m4, m0, m2 ; t11a
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2 ; t8a
    m[2] = _mm256_sub_epi32(m[8], m[6]); // psubd m2, m8, m6 ; t13
    m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8 ; t14
    m[8] = _mm256_sub_epi32(m[7], m[5]); // psubd m8, m7, m5 ; t12a
    m[7] = _mm256_add_epi32(m[7], m[5]); // paddd m7, m5 ; t15a
    for x in [2, 8, 3, 4, 0, 1, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m2, m8, m3, m4, m0, m1, m6, m7
    }
    for x in [2, 8, 3, 4, 0, 1, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m2, m8, m3, m4, m0, m1, m6, m7
    }
    for x in [2, 8, 3, 4] {
        m[x] = _mm256_mullo_epi32(m[x], m[14]); // REPX {pmulld x, m14}, m2, m8, m3, m4
    }
    m[2] = _mm256_add_epi32(m[2], m[11]); // paddd m2, m11
    m[8] = _mm256_add_epi32(m[8], m[11]); // paddd m8, m11
    m[5] = _mm256_add_epi32(m[2], m[3]); // paddd m5, m2, m3 ; t13a
    m[2] = _mm256_sub_epi32(m[2], m[3]); // psubd m2, m3 ; t10a
    m[3] = _mm256_sub_epi32(m[8], m[4]); // psubd m3, m8, m4 ; t11
    m[4] = _mm256_add_epi32(m[4], m[8]); // paddd m4, m8 ; t12
    for x in [5, 2, 3, 4] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m5, m2, m3, m4
    }
    _mm256_storeu_si256(at(r6, -32 * 4).cast(), m[7]); // mova [r6-32*4], m7
    _mm256_storeu_si256(at(r6, -32 * 3).cast(), m[6]); // mova [r6-32*3], m6
    _mm256_storeu_si256(at(r6, -32 * 2).cast(), m[5]); // mova [r6-32*2], m5
    _mm256_storeu_si256(at(r6, -32 * 1).cast(), m[4]); // mova [r6-32*1], m4
    _mm256_storeu_si256(at(r6, 32 * 0).cast(), m[3]); // mova [r6+32*0], m3
    _mm256_storeu_si256(at(r6, 32 * 1).cast(), m[2]); // mova [r6+32*1], m2
    _mm256_storeu_si256(at(r6, 32 * 2).cast(), m[1]); // mova [r6+32*2], m1
    _mm256_storeu_si256(at(r6, 32 * 3).cast(), m[0]); // mova [r6+32*3], m0
}

/// `.pass1` of `iadst_8x16_internal_10bpc` up to `jmp tx2q` (expects `m12`/`m13` = the clipping range)
#[inline(always)]
unsafe fn iadst_8x16_internal_10bpc_pass1(m: &mut M, c: Coef, eob: c_int) {
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    if eob >= 43 {
        // cmp eobd, 43 / jl .fast
        iadst_8x16_internal_10bpc_pass1_main(m, at(c, 32)); // add cq, 32 / call .pass1_main
        iadst_8x8_internal_10bpc_main_end(m); // call m(iadst_8x8_internal_10bpc).main_end / sub cq, 32
        _mm256_storeu_si256(at(c, 32 * 1).cast(), m[0]); // mova [cq+32* 1], m0
        _mm256_storeu_si256(at(c, 32 * 3).cast(), m[1]); // mova [cq+32* 3], m1
        _mm256_storeu_si256(at(c, 32 * 5).cast(), m[2]); // mova [cq+32* 5], m2
        _mm256_storeu_si256(at(c, 32 * 7).cast(), m[3]); // mova [cq+32* 7], m3
        _mm256_storeu_si256(at(c, 32 * 9).cast(), m[4]); // mova [cq+32* 9], m4
        _mm256_storeu_si256(at(c, 32 * 11).cast(), m[5]); // mova [cq+32*11], m5
        _mm256_storeu_si256(at(c, 32 * 13).cast(), m[6]); // mova [cq+32*13], m6
        m[15] = m[7]; // mova m15, m7
        iadst_8x16_internal_10bpc_pass1_main(m, c); // call .pass1_main
        iadst_8x8_internal_10bpc_main_end(m); // call m(iadst_8x8_internal_10bpc).main_end
        m[8] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m8, [cq+32* 1]
        m[9] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m9, [cq+32* 3]
        m[10] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m10, [cq+32* 5]
        m[11] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m11, [cq+32* 7]
        m[12] = _mm256_loadu_si256(at(c, 32 * 9).cast()); // mova m12, [cq+32* 9]
        m[13] = _mm256_loadu_si256(at(c, 32 * 11).cast()); // mova m13, [cq+32*11]
        m[14] = _mm256_loadu_si256(at(c, 32 * 13).cast()); // mova m14, [cq+32*13]
    } else {
        // .fast:
        iadst_8x16_internal_10bpc_pass1_main(m, c); // call .pass1_main
        iadst_8x8_internal_10bpc_main_end(m); // call m(iadst_8x8_internal_10bpc).main_end
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14, 15] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
        }
    }
}

/// `.pass2` of `iadst_8x16_internal_10bpc`
#[inline(always)]
unsafe fn iadst_8x16_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_8x16_internal_10bpc_transpose(m); // call m(idct_8x16_internal_10bpc).transpose
    iadst_8x16_internal_8bpc_main(m); // call m(iadst_8x16_internal_8bpc).main
    iadst_8x16_internal_8bpc_main_pass2_end(m); // call m(iadst_8x16_internal_8bpc).main_pass2_end
    m[8] = pw(2048); // vpbroadcastd m8, [pw_2048]
    m[12] = zx(lo(pw(4096))); // vpbroadcastd xm12, [pw_4096]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permute4x64_epi64::<0x8d>(m[x]); // REPX {vpermq x, x, q2031}, m0, m1, m2, m3
    }
    for x in [4, 5, 6, 7] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m4, m5, m6, m7
    }
    m[12] = _mm256_sub_epi16(m[12], m[8]); // psubw m12, m8
    idct_8x16_internal_10bpc_end(m, dst, stride, c) // jmp m(idct_8x16_internal_10bpc).end
}

/// `.pass1_main` of `iadst_8x16_internal_10bpc` (ends in `jmp m(iadst_8x8_internal_10bpc).main2`): loads the
/// even rows at `c`, expects `m14` = `pd_2896`, `m11` = `pd_2048`, `m12`/`m13` = the clipping range.
#[inline(always)]
unsafe fn iadst_8x16_internal_10bpc_pass1_main(m: &mut M, c: Coef) {
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmulld m0, m14, [cq+32* 0]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 14).cast())); // pmulld m7, m14, [cq+32*14]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m1, m14, [cq+32* 2]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 12).cast())); // pmulld m6, m14, [cq+32*12]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmulld m2, m14, [cq+32* 4]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 10).cast())); // pmulld m5, m14, [cq+32*10]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 6).cast())); // pmulld m3, m14, [cq+32* 6]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 32 * 8).cast())); // pmulld m4, m14, [cq+32* 8]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_8x8_internal_10bpc_main2(m) // jmp m(iadst_8x8_internal_10bpc).main2
}

/// `.pass1` of `iflipadst_8x16_internal_10bpc` up to `jmp tx2q` (expects `m12`/`m13` = the clipping range)
#[inline(always)]
unsafe fn iflipadst_8x16_internal_10bpc_pass1(m: &mut M, c: Coef, eob: c_int) {
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    if eob >= 43 {
        // cmp eobd, 43 / jl .fast
        iadst_8x16_internal_10bpc_pass1_main(m, at(c, 32)); // add cq, 32 / call m(iadst_8x16_internal_10bpc).pass1_main
        iflipadst_8x8_internal_10bpc_main_end(m); // call m(iflipadst_8x8_internal_10bpc).main_end / sub cq, 32
        _mm256_storeu_si256(at(c, 32 * 1).cast(), m[0]); // mova [cq+32* 1], m0
        _mm256_storeu_si256(at(c, 32 * 3).cast(), m[1]); // mova [cq+32* 3], m1
        _mm256_storeu_si256(at(c, 32 * 5).cast(), m[2]); // mova [cq+32* 5], m2
        _mm256_storeu_si256(at(c, 32 * 7).cast(), m[3]); // mova [cq+32* 7], m3
        _mm256_storeu_si256(at(c, 32 * 9).cast(), m[4]); // mova [cq+32* 9], m4
        _mm256_storeu_si256(at(c, 32 * 11).cast(), m[5]); // mova [cq+32*11], m5
        _mm256_storeu_si256(at(c, 32 * 13).cast(), m[6]); // mova [cq+32*13], m6
        m[15] = m[7]; // mova m15, m7
        iadst_8x16_internal_10bpc_pass1_main(m, c); // call m(iadst_8x16_internal_10bpc).pass1_main
        iflipadst_8x8_internal_10bpc_main_end(m); // call m(iflipadst_8x8_internal_10bpc).main_end
        m[8] = _mm256_loadu_si256(at(c, 32 * 1).cast()); // mova m8, [cq+32* 1]
        m[9] = _mm256_loadu_si256(at(c, 32 * 3).cast()); // mova m9, [cq+32* 3]
        m[10] = _mm256_loadu_si256(at(c, 32 * 5).cast()); // mova m10, [cq+32* 5]
        m[11] = _mm256_loadu_si256(at(c, 32 * 7).cast()); // mova m11, [cq+32* 7]
        m[12] = _mm256_loadu_si256(at(c, 32 * 9).cast()); // mova m12, [cq+32* 9]
        m[13] = _mm256_loadu_si256(at(c, 32 * 11).cast()); // mova m13, [cq+32*11]
        m[14] = _mm256_loadu_si256(at(c, 32 * 13).cast()); // mova m14, [cq+32*13]
    } else {
        // .fast:
        iadst_8x16_internal_10bpc_pass1_main(m, c); // call m(iadst_8x16_internal_10bpc).pass1_main
        iflipadst_8x8_internal_10bpc_main_end(m); // call m(iflipadst_8x8_internal_10bpc).main_end
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14, 15] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
        }
    }
}

/// `.pass2` of `iflipadst_8x16_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_8x16_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_8x16_internal_10bpc_transpose(m); // call m(idct_8x16_internal_10bpc).transpose
    iadst_8x16_internal_8bpc_main(m); // call m(iadst_8x16_internal_8bpc).main
    iadst_8x16_internal_8bpc_main_pass2_end(m); // call m(iadst_8x16_internal_8bpc).main_pass2_end
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[13] = zx(lo(pw(4096))); // vpbroadcastd xm13, [pw_4096]
    m[11] = m[0]; // mova m11, m0
    m[0] = _mm256_permute4x64_epi64::<0x8d>(m[7]); // vpermq m0, m7, q2031
    m[10] = m[1]; // mova m10, m1
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[6]); // vpermq m1, m6, q2031
    m[9] = m[2]; // mova m9, m2
    m[2] = _mm256_permute4x64_epi64::<0x8d>(m[5]); // vpermq m2, m5, q2031
    m[8] = m[3]; // mova m8, m3
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[4]); // vpermq m3, m4, q2031
    m[4] = _mm256_permute4x64_epi64::<0xd8>(m[8]); // vpermq m4, m8, q3120
    m[5] = _mm256_permute4x64_epi64::<0xd8>(m[9]); // vpermq m5, m9, q3120
    m[6] = _mm256_permute4x64_epi64::<0xd8>(m[10]); // vpermq m6, m10, q3120
    m[7] = _mm256_permute4x64_epi64::<0xd8>(m[11]); // vpermq m7, m11, q3120
    m[12] = _mm256_sub_epi16(m[12], m[13]); // psubw m12, m13
    idct_8x16_internal_10bpc_end(m, dst, stride, c) // jmp m(idct_8x16_internal_10bpc).end
}

/// The optional fourth argument of `IDTX16`.
#[derive(Clone, Copy)]
enum Idtx16Shift {
    /// Three arguments: no downshift.
    None,
    /// `pw_16384` in this register: downshift by 1 with rounding (`pmulhrsw`).
    Rnd(usize),
    /// `_`: downshift by 1 without rounding (`psraw`).
    Trunc,
}

/// `%macro IDTX16 src/dst, tmp, pw_1697x16, [pw_16384]`
#[inline(always)]
unsafe fn idtx16(m: &mut M, r1: usize, r2: usize, r3: usize, r4: Idtx16Shift) {
    m[r2] = _mm256_mulhrs_epi16(m[r3], m[r1]); // pmulhrsw m%2, m%3, m%1
    match r4 {
        Idtx16Shift::Rnd(r4) => m[r2] = _mm256_mulhrs_epi16(m[r2], m[r4]), // pmulhrsw m%2, m%4
        Idtx16Shift::Trunc => m[r2] = _mm256_srai_epi16::<1>(m[r2]),       // psraw m%2, 1
        Idtx16Shift::None => m[r1] = _mm256_adds_epi16(m[r1], m[r1]),      // paddsw m%1, m%1
    }
    m[r1] = _mm256_adds_epi16(m[r1], m[r2]); // paddsw m%1, m%2
}

/// `.pass1` of `iidentity_8x16_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_8x16_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    m[0] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmulld m0, m15, [cq+32* 0]
    m[8] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmulld m8, m15, [cq+32* 1]
    m[1] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmulld m1, m15, [cq+32* 2]
    m[9] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmulld m9, m15, [cq+32* 3]
    m[2] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmulld m2, m15, [cq+32* 4]
    m[10] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 5).cast())); // pmulld m10, m15, [cq+32* 5]
    m[3] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 6).cast())); // pmulld m3, m15, [cq+32* 6]
    m[11] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 7).cast())); // pmulld m11, m15, [cq+32* 7]
    m[4] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 8).cast())); // pmulld m4, m15, [cq+32* 8]
    m[12] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 9).cast())); // pmulld m12, m15, [cq+32* 9]
    m[5] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 10).cast())); // pmulld m5, m15, [cq+32*10]
    m[13] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 11).cast())); // pmulld m13, m15, [cq+32*11]
    m[6] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 12).cast())); // pmulld m6, m15, [cq+32*12]
    m[14] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 13).cast())); // pmulld m14, m15, [cq+32*13]
    m[7] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 14).cast())); // pmulld m7, m15, [cq+32*14]
    m[15] = _mm256_mullo_epi32(m[15], _mm256_loadu_si256(at(c, 32 * 15).cast())); // pmulld m15, [cq+32*15]
    _mm256_storeu_si256(c.cast(), m[7]); // mova [cq], m7
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m0, m1, m2, m3, m4, m5, m6, m8, m9, m10, m11, m12, m13, m14, m15
    }
    m[7] = _mm256_add_epi32(m[7], _mm256_loadu_si256(c.cast())); // paddd m7, [cq]
    for x in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15
    }
}

/// `.pass2` of `iidentity_8x16_internal_10bpc`
#[inline(always)]
unsafe fn iidentity_8x16_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_packs_epi32(m[0], m[8]); // packssdw m0, m8
    m[1] = _mm256_packs_epi32(m[1], m[9]); // packssdw m1, m9
    m[2] = _mm256_packs_epi32(m[2], m[10]); // packssdw m2, m10
    m[3] = _mm256_packs_epi32(m[3], m[11]); // packssdw m3, m11
    m[4] = _mm256_packs_epi32(m[4], m[12]); // packssdw m4, m12
    m[5] = _mm256_packs_epi32(m[5], m[13]); // packssdw m5, m13
    m[6] = _mm256_packs_epi32(m[6], m[14]); // packssdw m6, m14
    m[13] = _mm256_packs_epi32(m[7], m[15]); // packssdw m13, m7, m15
    m[8] = pw(1697 * 16); // vpbroadcastd m8, [pw_1697x16]
    for x in [0, 1, 2, 3, 4, 5, 6, 13] {
        idtx16(m, x, 9, 8, Idtx16Shift::None); // REPX {IDTX16 x, 9, 8}, 0, 1, 2, 3, 4, 5, 6, 13
    }
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    iidentity_8x16_internal_10bpc_pass2_end(m, dst, stride, c); // call .pass2_end
}

/// `.pass2_end` of `iidentity_8x16_internal_10bpc`: transposes the packed rows `m0`-`m6`, `m13` and writes them
/// (`m7` = the pixel maximum, `m12` = the rounding multiplier).
#[inline(always)]
unsafe fn iidentity_8x16_internal_10bpc_pass2_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    m[9] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m9, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[6], m[13]); // punpckhwd m1, m6, m13
    m[6] = _mm256_unpacklo_epi16(m[6], m[13]); // punpcklwd m6, m13
    m[13] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m13, m4, m5
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
    m[5] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m5, m2, m3
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[3] = _mm256_unpackhi_epi32(m[0], m[5]); // punpckhdq m3, m0, m5
    m[0] = _mm256_unpacklo_epi32(m[0], m[5]); // punpckldq m0, m5
    m[11] = _mm256_unpackhi_epi32(m[9], m[2]); // punpckhdq m11, m9, m2
    m[9] = _mm256_unpacklo_epi32(m[9], m[2]); // punpckldq m9, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m2, m4, m6
    m[4] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m4, m6
    m[6] = _mm256_unpacklo_epi32(m[13], m[1]); // punpckldq m6, m13, m1
    m[13] = _mm256_unpackhi_epi32(m[13], m[1]); // punpckhdq m13, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
    m[8] = _mm256_unpacklo_epi64(m[9], m[6]); // punpcklqdq m8, m9, m6
    m[9] = _mm256_unpackhi_epi64(m[9], m[6]); // punpckhqdq m9, m6
    m[10] = _mm256_unpacklo_epi64(m[11], m[13]); // punpcklqdq m10, m11, m13
    m[11] = _mm256_unpackhi_epi64(m[11], m[13]); // punpckhqdq m11, m13
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    iidentity_8x8_internal_10bpc_write_2x8x2_start(m, stride, &mut r6, &mut c, &mut dst); // call m(iidentity_8x8_internal_10bpc).write_2x8x2_start
    m[0] = _mm256_mulhrs_epi16(m[12], m[2]); // pmulhrsw m0, m12, m2
    m[1] = _mm256_mulhrs_epi16(m[12], m[3]); // pmulhrsw m1, m12, m3
    iidentity_8x8_internal_10bpc_write_2x8x2_zero(m, &mut c, &mut dst, stride, r6); // call m(iidentity_8x8_internal_10bpc).write_2x8x2_zero
    m[0] = _mm256_mulhrs_epi16(m[12], m[8]); // pmulhrsw m0, m12, m8
    m[1] = _mm256_mulhrs_epi16(m[12], m[9]); // pmulhrsw m1, m12, m9
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    iidentity_8x8_internal_10bpc_write_2x8x2_zero(m, &mut c, &mut dst, stride, r6); // call m(iidentity_8x8_internal_10bpc).write_2x8x2_zero
    m[0] = _mm256_mulhrs_epi16(m[12], m[10]); // pmulhrsw m0, m12, m10
    m[1] = _mm256_mulhrs_epi16(m[12], m[11]); // pmulhrsw m1, m12, m11
    iidentity_8x8_internal_10bpc_write_2x8x2_zero(m, &mut c, &mut dst, stride, r6);
    // call m(iidentity_8x8_internal_10bpc).write_2x8x2_zero
}

/// `.pass2` of `idct_8x16_internal_12bpc` (`cglobal …, 32*8`: the frame is eight 32-byte slots, `r6` its middle)
#[inline(always)]
unsafe fn idct_8x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    let mut frame = core::mem::MaybeUninit::<[__m256i; 8]>::uninit();
    // `idct_8x16_internal_10bpc_main_oddhalf` writes all eight slots `[r6-32*4]` … `[r6+32*3]` before
    // `idct_16x8_internal_10bpc_pass1_rotations` reads any of them.
    let r6 = at(frame.as_mut_ptr().cast::<__m256i>(), 32 * 4); // lea r6, [rsp+32*4]
    idct_8x16_internal_12bpc_transpose(m, c); // call .transpose
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    _mm256_storeu_si256(at(c, 32 * 8).cast(), m[0]); // mova [cq+32* 8], m0
    _mm256_storeu_si256(at(c, 32 * 10).cast(), m[2]); // mova [cq+32*10], m2
    _mm256_storeu_si256(at(c, 32 * 12).cast(), m[4]); // mova [cq+32*12], m4
    _mm256_storeu_si256(at(c, 32 * 14).cast(), m[6]); // mova [cq+32*14], m6
    m[0] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmaxsd m0, m12, [cq+32* 1]
    m[4] = _mm256_max_epi32(m[12], m[1]); // pmaxsd m4, m12, m1
    m[1] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmaxsd m1, m12, [cq+32* 3]
    m[2] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 5).cast())); // pmaxsd m2, m12, [cq+32* 5]
    m[6] = _mm256_max_epi32(m[12], m[5]); // pmaxsd m6, m12, m5
    m[5] = _mm256_max_epi32(m[12], m[3]); // pmaxsd m5, m12, m3
    m[3] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 7).cast())); // pmaxsd m3, m12, [cq+32* 7]
    m[7] = _mm256_max_epi32(m[7], m[12]); // pmaxsd m7, m12
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    idct_8x16_internal_10bpc_main_oddhalf(m, r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
    m[0] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmaxsd m0, m12, [cq+32* 0]
    m[1] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmaxsd m1, m12, [cq+32* 2]
    m[2] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmaxsd m2, m12, [cq+32* 4]
    m[3] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 6).cast())); // pmaxsd m3, m12, [cq+32* 6]
    m[4] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 8).cast())); // pmaxsd m4, m12, [cq+32* 8]
    m[5] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 10).cast())); // pmaxsd m5, m12, [cq+32*10]
    m[6] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 12).cast())); // pmaxsd m6, m12, [cq+32*12]
    m[7] = _mm256_max_epi32(m[12], _mm256_loadu_si256(at(c, 32 * 14).cast())); // pmaxsd m7, m12, [cq+32*14]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[11] = pd(8); // vpbroadcastd m11, [pd_8]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(idct_16x8_internal_10bpc).pass1_rotations
    for x in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4}, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15
    }
    idct_8x16_internal_12bpc_end(m, dst, stride, c)
}

/// `.end` of `idct_8x16_internal_12bpc` to its `RET`: packs and writes the 32-bit rows `m0`-`m15`.
#[inline(always)]
unsafe fn idct_8x16_internal_12bpc_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    let mut r6 = 0;
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[1] = _mm256_packs_epi32(m[2], m[3]); // packssdw m1, m2, m3
    m[2] = _mm256_packs_epi32(m[4], m[5]); // packssdw m2, m4, m5
    m[3] = _mm256_packs_epi32(m[6], m[7]); // packssdw m3, m6, m7
    m[4] = _mm256_packs_epi32(m[8], m[9]); // packssdw m4, m8, m9
    m[5] = _mm256_packs_epi32(m[10], m[11]); // packssdw m5, m10, m11
    m[6] = _mm256_packs_epi32(m[12], m[13]); // packssdw m6, m12, m13
    m[7] = _mm256_packs_epi32(m[14], m[15]); // packssdw m7, m14, m15
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
    idct_8x8_internal_12bpc_write_8x4_start(m, stride, &mut r6); // call m(idct_8x8_internal_12bpc).write_8x4_start
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m0, m2, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[3]); // vpermq m1, m3, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[4]); // vpermq m0, m4, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[5]); // vpermq m1, m5, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[6]); // vpermq m0, m6, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[7]); // vpermq m1, m7, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `.transpose` of `idct_8x16_internal_12bpc` (ends in `jmp m(idct_8x8_internal_12bpc).transpose_8x8`): the
/// 16x8 dwords in `m0`-`m15` transposed, with the coefficient buffer as scratch: rows 0-7 of the result in
/// `[cq+32*0]` … `[cq+32*7]`, rows 8-15 in `m0`-`m7`.
#[inline(always)]
unsafe fn idct_8x16_internal_12bpc_transpose(m: &mut M, c: Coef) {
    _mm256_storeu_si256(at(c, 32 * 8).cast(), m[8]); // mova [cq+32* 8], m8
    _mm256_storeu_si256(at(c, 32 * 9).cast(), m[9]); // mova [cq+32* 9], m9
    _mm256_storeu_si256(at(c, 32 * 10).cast(), m[10]); // mova [cq+32*10], m10
    _mm256_storeu_si256(at(c, 32 * 11).cast(), m[11]); // mova [cq+32*11], m11
    idct_8x8_internal_12bpc_transpose_8x8(m); // call m(idct_8x8_internal_12bpc).transpose_8x8
    _mm256_storeu_si256(at(c, 32 * 0).cast(), m[0]); // mova [cq+32* 0], m0
    _mm256_storeu_si256(at(c, 32 * 1).cast(), m[1]); // mova [cq+32* 1], m1
    _mm256_storeu_si256(at(c, 32 * 2).cast(), m[2]); // mova [cq+32* 2], m2
    _mm256_storeu_si256(at(c, 32 * 3).cast(), m[3]); // mova [cq+32* 3], m3
    _mm256_storeu_si256(at(c, 32 * 4).cast(), m[4]); // mova [cq+32* 4], m4
    _mm256_storeu_si256(at(c, 32 * 5).cast(), m[5]); // mova [cq+32* 5], m5
    _mm256_storeu_si256(at(c, 32 * 6).cast(), m[6]); // mova [cq+32* 6], m6
    _mm256_storeu_si256(at(c, 32 * 7).cast(), m[7]); // mova [cq+32* 7], m7
    m[0] = _mm256_loadu_si256(at(c, 32 * 8).cast()); // mova m0, [cq+32* 8]
    m[1] = _mm256_loadu_si256(at(c, 32 * 9).cast()); // mova m1, [cq+32* 9]
    m[2] = _mm256_loadu_si256(at(c, 32 * 10).cast()); // mova m2, [cq+32*10]
    m[3] = _mm256_loadu_si256(at(c, 32 * 11).cast()); // mova m3, [cq+32*11]
    m[4] = m[12]; // mova m4, m12
    m[5] = m[13]; // mova m5, m13
    m[6] = m[14]; // mova m6, m14
    m[7] = m[15]; // mova m7, m15
    idct_8x8_internal_12bpc_transpose_8x8(m) // jmp m(idct_8x8_internal_12bpc).transpose_8x8
}

/// `.pass2` of `iadst_8x16_internal_12bpc` (`cglobal …, 32*8`: the frame is eight 32-byte slots, `r6` its middle)
#[inline(always)]
unsafe fn iadst_8x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    let mut frame = core::mem::MaybeUninit::<[__m256i; 8]>::uninit();
    // `iadst_16x8_internal_10bpc_main_part1` (in `.pass2_main`) writes all eight slots `[r6-32*4]` …
    // `[r6+32*3]` before `main_part2` or `pass1_rotations` reads any of them.
    let r6 = at(frame.as_mut_ptr().cast::<__m256i>(), 32 * 4); // lea r6, [rsp+32*4]
    iadst_8x16_internal_12bpc_pass2_main(m, c, r6); // call .pass2_main
    iadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(iadst_16x8_internal_10bpc).pass1_rotations
    iadst_8x16_internal_12bpc_pass2_end(m, dst, stride, c)
}

/// `.pass2_end` of `iadst_8x16_internal_12bpc` (ends in `jmp m(idct_8x16_internal_12bpc).end`)
#[inline(always)]
unsafe fn iadst_8x16_internal_12bpc_pass2_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    for x in [0, 1, 2, 3, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4 }, m0, m1, m2, m3, m12, m13, m14, m15
    }
    for x in [4, 5, 6, 7, 8, 9, 10, 11] {
        m[x] = _mm256_srai_epi32::<15>(m[x]); // REPX {psrad x, 15}, m4, m5, m6, m7, m8, m9, m10, m11
    }
    idct_8x16_internal_12bpc_end(m, dst, stride, c) // jmp m(idct_8x16_internal_12bpc).end
}

/// `.pass2_main` of `iadst_8x16_internal_12bpc`: the adst16 of the columns, with `r6` the middle of an 8-slot
/// frame (written whole by `iadst_16x8_internal_10bpc_main_part1`); leaves `m13`-`m15` = `pd_17407`, `pd_17408`,
/// `pd_8` for the rotations.
#[inline(always)]
unsafe fn iadst_8x16_internal_12bpc_pass2_main(m: &mut M, c: Coef, r6: *mut __m256i) {
    idct_8x16_internal_12bpc_transpose(m, c); // call m(idct_8x16_internal_12bpc).transpose
    m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
    m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
    _mm256_storeu_si256(at(c, 32 * 8).cast(), m[0]); // mova [cq+32* 8], m0
    _mm256_storeu_si256(at(c, 32 * 11).cast(), m[3]); // mova [cq+32*11], m3
    _mm256_storeu_si256(at(c, 32 * 12).cast(), m[4]); // mova [cq+32*12], m4
    _mm256_storeu_si256(at(c, 32 * 15).cast(), m[7]); // mova [cq+32*15], m7
    m[0] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 2).cast())); // pmaxsd m0, m13, [cq+32* 2] ; 2
    m[3] = _mm256_max_epi32(m[13], m[1]); // pmaxsd m3, m13, m1 ; 9
    m[1] = _mm256_max_epi32(m[13], m[5]); // pmaxsd m1, m13, m5 ; 13
    m[4] = _mm256_max_epi32(m[13], m[2]); // pmaxsd m4, m13, m2 ; 10
    m[2] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 6).cast())); // pmaxsd m2, m13, [cq+32* 6] ; 6
    m[5] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 5).cast())); // pmaxsd m5, m13, [cq+32* 5] ; 5
    m[6] = _mm256_max_epi32(m[13], m[6]); // pmaxsd m6, m13, m6 ; 14
    m[7] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 1).cast())); // pmaxsd m7, m13, [cq+32* 1] ; 1
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[12] = pd(2048); // vpbroadcastd m12, [pd_2048]
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    iadst_16x8_internal_10bpc_main_part1(m, r6); // call m(iadst_16x8_internal_10bpc).main_part1
    m[0] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 0).cast())); // pmaxsd m0, m13, [cq+32* 0] ; 0
    m[1] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 15).cast())); // pmaxsd m1, m13, [cq+32*15] ; 15
    m[2] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 4).cast())); // pmaxsd m2, m13, [cq+32* 4] ; 4
    m[3] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 11).cast())); // pmaxsd m3, m13, [cq+32*11] ; 11
    m[4] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 8).cast())); // pmaxsd m4, m13, [cq+32* 8] ; 8
    m[5] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 7).cast())); // pmaxsd m5, m13, [cq+32* 7] ; 7
    m[6] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 12).cast())); // pmaxsd m6, m13, [cq+32*12] ; 12
    m[7] = _mm256_max_epi32(m[13], _mm256_loadu_si256(at(c, 32 * 3).cast())); // pmaxsd m7, m13, [cq+32* 3] ; 3
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_16x8_internal_10bpc_main_part2(m, r6); // call m(iadst_16x8_internal_10bpc).main_part2
    m[14] = pd(17408); // vpbroadcastd m14, [pd_17408]
    m[15] = _mm256_srli_epi32::<11>(m[15]); // psrld m15, 11 ; pd_1
    m[13] = _mm256_sub_epi32(m[14], m[15]); // psubd m13, m14, m15 ; pd_17407
    m[15] = _mm256_slli_epi32::<3>(m[15]); // pslld m15, 3 ; pd_8
}

/// `.pass2` of `iflipadst_8x16_internal_12bpc` (`cglobal …, 32*8`: the frame is eight 32-byte slots, `r6` its
/// middle)
#[inline(always)]
unsafe fn iflipadst_8x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    let mut frame = core::mem::MaybeUninit::<[__m256i; 8]>::uninit();
    // `iadst_16x8_internal_10bpc_main_part1` (in `iadst_8x16_internal_12bpc_pass2_main`) writes all eight slots
    // `[r6-32*4]` … `[r6+32*3]` before anything reads them.
    let r6 = at(frame.as_mut_ptr().cast::<__m256i>(), 32 * 4); // lea r6, [rsp+32*4]
    iadst_8x16_internal_12bpc_pass2_main(m, c, r6); // call m(iadst_8x16_internal_12bpc).pass2_main
    iflipadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(iflipadst_16x8_internal_10bpc).pass1_rotations
    iadst_8x16_internal_12bpc_pass2_end(m, dst, stride, c) // jmp m(iadst_8x16_internal_12bpc).pass2_end
}

/// `.pass2` of `iidentity_8x16_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_8x16_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iidentity_8x16_internal_12bpc_pass2_main(m, c); // call .pass2_main
    m[0] = _mm256_packs_epi32(m[0], m[8]); // packssdw m0, m8
    m[1] = _mm256_packs_epi32(m[1], m[9]); // packssdw m1, m9
    m[2] = _mm256_packs_epi32(m[2], m[10]); // packssdw m2, m10
    m[3] = _mm256_packs_epi32(m[3], m[11]); // packssdw m3, m11
    m[4] = _mm256_packs_epi32(m[4], m[12]); // packssdw m4, m12
    m[5] = _mm256_packs_epi32(m[5], m[13]); // packssdw m5, m13
    m[6] = _mm256_packs_epi32(m[6], m[14]); // packssdw m6, m14
    m[13] = _mm256_packs_epi32(m[7], m[15]); // packssdw m13, m7, m15
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    m[12] = pw(16384); // vpbroadcastd m12, [pw_16384]
    iidentity_8x16_internal_10bpc_pass2_end(m, dst, stride, c); // call m(iidentity_8x16_internal_10bpc).pass2_end
}

/// `.pass2_main` of `iidentity_8x16_internal_12bpc`: clips `m0`-`m15` to 18 bits and scales them by 5793 with
/// `(x + 1024) >> 14` (`[cq]` is the spill slot).
#[inline(always)]
unsafe fn iidentity_8x16_internal_12bpc_pass2_main(m: &mut M, c: Coef) {
    _mm256_storeu_si256(c.cast(), m[7]); // mova [cq], m7
    m[7] = pd(-0x20000); // vpbroadcastd m7, [clip_18b_min]
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_max_epi32(m[x], m[7]); // REPX {pmaxsd x, m7}, m0, m1, m2, m3, m4, m5, m6, m8, m9, m10, m11, m12, m13, m14, m15
    }
    m[7] = _mm256_max_epi32(m[7], _mm256_loadu_si256(c.cast())); // pmaxsd m7, [cq]
    _mm256_storeu_si256(c.cast(), m[15]); // mova [cq], m15
    m[15] = pd(0x1ffff); // vpbroadcastd m15, [clip_18b_max]
    for x in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] {
        m[x] = _mm256_min_epi32(m[x], m[15]); // REPX {pminsd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14
    }
    m[15] = _mm256_min_epi32(m[15], _mm256_loadu_si256(c.cast())); // pminsd m15, [cq]
    _mm256_storeu_si256(c.cast(), m[7]); // mova [cq], m7
    m[7] = pd(5793); // vpbroadcastd m7, [pd_5793]
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_mullo_epi32(m[x], m[7]); // REPX {pmulld x, m7}, m0, m1, m2, m3, m4, m5, m6, m8, m9, m10, m11, m12, m13, m14, m15
    }
    m[7] = _mm256_mullo_epi32(m[7], _mm256_loadu_si256(c.cast())); // pmulld m7, [cq]
    _mm256_storeu_si256(c.cast(), m[15]); // mova [cq], m15
    m[15] = pd(1024); // vpbroadcastd m15, [pd_1024]
    for x in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] {
        m[x] = _mm256_add_epi32(m[x], m[15]); // REPX {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14
    }
    m[15] = _mm256_add_epi32(m[15], _mm256_loadu_si256(c.cast())); // paddd m15, [cq]
    for x in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<14>(m[x]); // REPX {psrad x, 14}, m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x16_10bpc_avx2,
    inv_txfm_add_8x16_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x16_12bpc_avx2,
    inv_txfm_add_8x16_12bpc,
    K::Identity,
    K::Identity
);

// --- part 4: itx16_avx2.asm lines 3447-4603 (16x4, 16x8) ---

// Small helpers of this part (the memory operands of the assembly; byte offsets).

/// `vbroadcasti128 m, [p+off]`
#[inline(always)]
unsafe fn vbi128_p4<T>(p: *mut T, off: isize) -> __m256i {
    _mm256_broadcastsi128_si256(_mm_loadu_si128(at(p, off).cast()))
}

/// `shufpd m, m, 0x0c` on integer registers
#[inline(always)]
unsafe fn shufpd_0c_p4(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castpd_si256(_mm256_shuffle_pd::<0x0c>(
        _mm256_castsi256_pd(a),
        _mm256_castsi256_pd(b),
    ))
}

/// `mova m, [p+off]` (32 bytes)
#[inline(always)]
unsafe fn ld_p4<T>(p: *mut T, off: isize) -> __m256i {
    _mm256_loadu_si256(at(p, off).cast())
}

/// `mova [p+off], m` (32 bytes)
#[inline(always)]
unsafe fn st_p4<T>(p: *mut T, off: isize, v: __m256i) {
    _mm256_storeu_si256(at(p, off).cast(), v)
}

// --- 16x4 ---------------------------------------------------------------------------------------------------------

/// `%macro INV_TXFM_16X4_FN type1, type2, 10`: the DC-only path of dct_dct (`.dconly` is the prelude's), then the
/// two passes. The clipping constants that each `cglobal i<type1>_16x4_internal_10bpc` loads before its `.pass1`
/// are loaded here.
#[inline(always)]
unsafe fn inv_txfm_add_16x4_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        return inv_txfm_add_dct_dct_16x4_10bpc_dconly(m, dst, stride, c, eob);
    }
    match t1 {
        K::Dct => {
            m[8] = pd(-0x20000); // vpbroadcastd m8, [clip_18b_min]
            m[9] = pd(0x1ffff); // vpbroadcastd m9, [clip_18b_max]
            idct_16x4_internal_10bpc_pass1(m, c)
        }
        K::Adst => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            iadst_16x4_internal_10bpc_pass1(m, c)
        }
        K::FlipAdst => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            iflipadst_16x4_internal_10bpc_pass1(m, c)
        }
        K::Identity => iidentity_16x4_internal_10bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x4_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_16x4_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_16x4_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_16x4_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro INV_TXFM_16X4_FN type1, type2, 12` (the DC-only path jumps to the 10 bpc `.dconly`; the first passes
/// are the 10 bpc `.pass1`s with the 20-bit clipping constants)
#[inline(always)]
unsafe fn inv_txfm_add_16x4_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        m[3] = pw(0x7000); // vpbroadcastd m3, [dconly_12bpc]
        return inv_txfm_add_dct_dct_16x4_10bpc_dconly(m, dst, stride, c, eob); // jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly
    }
    match t1 {
        K::Dct => {
            m[8] = pd(-0x80000); // vpbroadcastd m8, [clip_20b_min]
            m[9] = pd(0x7ffff); // vpbroadcastd m9, [clip_20b_max]
            idct_16x4_internal_10bpc_pass1(m, c) // jmp m(idct_16x4_internal_10bpc).pass1
        }
        K::Adst => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            iadst_16x4_internal_10bpc_pass1(m, c) // jmp m(iadst_16x4_internal_10bpc).pass1
        }
        K::FlipAdst => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            iflipadst_16x4_internal_10bpc_pass1(m, c) // jmp m(iflipadst_16x4_internal_10bpc).pass1
        }
        K::Identity => iidentity_16x4_internal_12bpc_pass1(m, c),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x4_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_16x4_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_16x4_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_16x4_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// `.pass1` of `idct_16x4_internal_10bpc` (up to `jmp tx2q`; expects `m8`, `m9` = clip min, max)
#[inline(always)]
unsafe fn idct_16x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[0] = vbi128_p4(c, 16 * 0); // vbroadcasti128 m0, [cq+16* 0]
    m[4] = vbi128_p4(c, 16 * 4); // vbroadcasti128 m4, [cq+16* 4]
    m[1] = vbi128_p4(c, 16 * 2); // vbroadcasti128 m1, [cq+16* 2]
    m[7] = vbi128_p4(c, 16 * 6); // vbroadcasti128 m7, [cq+16* 6]
    m[5] = vbi128_p4(c, 16 * 10); // vbroadcasti128 m5, [cq+16*10]
    m[2] = vbi128_p4(c, 16 * 8); // vbroadcasti128 m2, [cq+16* 8]
    m[6] = vbi128_p4(c, 16 * 12); // vbroadcasti128 m6, [cq+16*12]
    m[3] = vbi128_p4(c, 16 * 14); // vbroadcasti128 m3, [cq+16*14]
    m[0] = shufpd_0c_p4(m[0], m[4]); // shufpd m0, m4, 0x0c ; 0 4
    m[1] = shufpd_0c_p4(m[1], m[5]); // shufpd m1, m5, 0x0c ; 2 10
    m[2] = shufpd_0c_p4(m[2], m[6]); // shufpd m2, m6, 0x0c ; 8 12
    m[3] = shufpd_0c_p4(m[3], m[7]); // shufpd m3, m7, 0x0c ; 14 6
    idct_16x4_internal_10bpc_pass1_main(m); // call .pass1_main
    m[10] = vbi128_p4(c, 16 * 1); // vbroadcasti128 m10, [cq+16* 1]
    m[4] = vbi128_p4(c, 16 * 5); // vbroadcasti128 m4, [cq+16* 5]
    m[11] = vbi128_p4(c, 16 * 15); // vbroadcasti128 m11, [cq+16*15]
    m[5] = vbi128_p4(c, 16 * 11); // vbroadcasti128 m5, [cq+16*11]
    m[10] = shufpd_0c_p4(m[10], m[4]); // shufpd m10, m4, 0x0c ; 1 5
    m[11] = shufpd_0c_p4(m[11], m[5]); // shufpd m11, m5, 0x0c ; 15 11
    m[5] = vbi128_p4(c, 16 * 9); // vbroadcasti128 m5, [cq+16* 9]
    m[4] = vbi128_p4(c, 16 * 13); // vbroadcasti128 m4, [cq+16*13]
    m[5] = shufpd_0c_p4(m[5], m[4]); // shufpd m5, m4, 0x0c ; 9 13
    m[6] = vbi128_p4(c, 16 * 7); // vbroadcasti128 m6, [cq+16* 7]
    m[4] = vbi128_p4(c, 16 * 3); // vbroadcasti128 m4, [cq+16* 3]
    m[6] = shufpd_0c_p4(m[6], m[4]); // shufpd m6, m4, 0x0c ; 7 3
    idct_16x4_internal_10bpc_pass1_main2(m); // call .pass1_main2
    m[4] = _mm256_cmpeq_epi32(m[4], m[4]); // pcmpeqd m4, m4
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_sub_epi32(m[x], m[4]); // REPX {psubd x, m4}, m0, m1, m2, m3
    }
    idct_16x4_internal_10bpc_pass1_main3(m); // call .pass1_main3
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass2` of `idct_16x4_internal_10bpc`
#[inline(always)]
unsafe fn idct_16x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x4_internal_10bpc_transpose_4x16_packed(m); // call .transpose_4x16_packed
                                                       // lea r6, [deint_shuf+128] (the 8 bpc `.main`'s `o()` base: no code here)
    idct_16x4_internal_8bpc_main(m); // call m(idct_16x4_internal_8bpc).main
    idct_16x4_internal_10bpc_end(m, dst, stride, c)
}

/// `.end` of `idct_16x4_internal_10bpc` (falls into `.end2` and `.end3`)
#[inline(always)]
unsafe fn idct_16x4_internal_10bpc_end(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[4] = pw(2048); // vpbroadcastd m4, [pw_2048]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[4]); // REPX {pmulhrsw x, m4}, m0, m1, m2, m3
    }
    m[5] = pw(0x3ff); // vpbroadcastd m5, [pixel_10bpc_max]
    idct_16x4_internal_10bpc_end2(m, dst, stride, c)
}

/// `.end2` of `idct_16x4_internal_10bpc` (expects `m5` = pixel max; falls into `.end3`)
#[inline(always)]
unsafe fn idct_16x4_internal_10bpc_end2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[0] = _mm256_add_epi16(m[0], ld_p4(dst, 0)); // paddw m0, [dstq+strideq*0]
    m[1] = _mm256_add_epi16(m[1], ld_p4(dst, stride)); // paddw m1, [dstq+strideq*1]
    idct_16x4_internal_10bpc_end3(m, dst, stride, c)
}

/// `.end3` of `idct_16x4_internal_10bpc` (expects `m5` = pixel max; ends in `RET`)
#[inline(always)]
unsafe fn idct_16x4_internal_10bpc_end3(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    let r6 = at(dst, stride * 2); // lea r6, [dstq+strideq*2]
    m[2] = _mm256_add_epi16(m[2], ld_p4(r6, 0)); // paddw m2, [r6 +strideq*0]
    m[3] = _mm256_add_epi16(m[3], ld_p4(r6, stride)); // paddw m3, [r6 +strideq*1]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in 0..8 {
        st_p4(c, 32 * x, m[4]); // REPX {mova [cq+32*x], m4}, 0, 1, 2, 3, 4, 5, 6, 7
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi16(m[x], m[4]); // REPX {pmaxsw x, m4}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi16(m[x], m[5]); // REPX {pminsw x, m5}, m0, m1, m2, m3
    }
    st_p4(dst, 0, m[0]); // mova [dstq+strideq*0], m0
    st_p4(dst, stride, m[1]); // mova [dstq+strideq*1], m1
    st_p4(r6, 0, m[2]); // mova [r6 +strideq*0], m2
    st_p4(r6, stride, m[3]); // mova [r6 +strideq*1], m3
}

/// `.pass1_main` of `idct_16x4_internal_10bpc`: the even half (`m0`-`m3` = in0 in4, in2 in10, in8 in12, in14 in6;
/// expects `m8`, `m9` = clip min, max; leaves `m7` = `pd_2048`)
#[inline(always)]
pub(crate) unsafe fn idct_16x4_internal_10bpc_pass1_main(m: &mut M) {
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    idct_8x4_internal_10bpc_main(m); // call m(idct_8x4_internal_10bpc).main
    m[3] = _mm256_sub_epi32(m[0], m[4]); // psubd m3, m0, m4 ; idct8 out7 out6
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; idct8 out0 out1
    m[1] = _mm256_add_epi32(m[2], m[5]); // paddd m1, m2, m5 ; idct8 out3 out2
    m[2] = _mm256_sub_epi32(m[2], m[5]); // psubd m2, m5 ; idct8 out4 out5
}

/// `.pass1_main2` of `idct_16x4_internal_10bpc`: the odd half (`m10`, `m11`, `m5`, `m6` = in1 in5, in15 in11, in9
/// in13, in7 in3; expects `m7` = `pd_2048`, `m8`, `m9` = clip min, max)
#[inline(always)]
pub(crate) unsafe fn idct_16x4_internal_10bpc_pass1_main2(m: &mut M) {
    itx_mulsub_2d(m, 10, 11, 4, 12, 13, Some(7), Q(401, 1931), Q(4076, 3612), 1); // ITX_MULSUB_2D 10, 11, 4, 12, 13, 7, 401_1931, 4076_3612, 1
    itx_mulsub_2d(m, 5, 6, 4, 12, 13, Some(7), Q(3166, 3920), Q(2598, 1189), 1); // ITX_MULSUB_2D 5, 6, 4, 12, 13, 7, 3166_3920, 2598_1189, 1
    m[12] = pd_q(3784, -3784); // vbroadcasti128 m12, [pd_3784_m3784]
    m[4] = _mm256_sub_epi32(m[10], m[5]); // psubd m4, m10, m5
    m[10] = _mm256_add_epi32(m[10], m[5]); // paddd m10, m5 ; t8 t11
    m[4] = _mm256_sign_epi32(m[4], m[12]); // psignd m4, m12 ; t9 t10
    m[5] = _mm256_sub_epi32(m[11], m[6]); // psubd m5, m11, m6
    m[11] = _mm256_add_epi32(m[11], m[6]); // paddd m11, m6 ; t15 t12
    m[5] = _mm256_sign_epi32(m[5], m[12]); // psignd m5, m12 ; t14 t13
    m[6] = pd(1567); // vpbroadcastd m6, [pd_1567]
    m[13] = pd(3784); // vpbroadcastd m13, [pd_3784]
    for x in [5, 4] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m5, m4
    }
    for x in [5, 4] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m5, m4
    }
    m[12] = _mm256_mullo_epi32(m[12], m[5]); // pmulld m12, m5
    m[5] = _mm256_mullo_epi32(m[5], m[6]); // pmulld m5, m6
    m[6] = pd_q(1567, -1567); // vbroadcasti128 m6, [pd_1567_m1567]
    m[13] = _mm256_mullo_epi32(m[13], m[4]); // pmulld m13, m4
    m[4] = _mm256_mullo_epi32(m[4], m[6]); // pmulld m4, m6
    for x in [10, 11, 0, 1] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m10, m11, m0, m1
    }
    for x in [10, 11, 0, 1] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m10, m11, m0, m1
    }
    m[12] = _mm256_add_epi32(m[12], m[7]); // paddd m12, m7
    m[5] = _mm256_add_epi32(m[5], m[7]); // paddd m5, m7
    m[4] = _mm256_add_epi32(m[4], m[12]); // paddd m4, m12
    m[5] = _mm256_sub_epi32(m[5], m[13]); // psubd m5, m13
    m[4] = _mm256_srai_epi32::<12>(m[4]); // psrad m4, 12 ; t14a t10a
    m[5] = _mm256_srai_epi32::<12>(m[5]); // psrad m5, 12 ; t9a t13a
    m[12] = pd(2896); // vpbroadcastd m12, [pd_2896]
    m[6] = _mm256_unpackhi_epi64(m[11], m[5]); // punpckhqdq m6, m11, m5
    m[11] = _mm256_unpacklo_epi64(m[11], m[4]); // punpcklqdq m11, m4
    m[4] = _mm256_unpackhi_epi64(m[10], m[4]); // punpckhqdq m4, m10, m4
    m[10] = _mm256_unpacklo_epi64(m[10], m[5]); // punpcklqdq m10, m5
    m[5] = _mm256_sub_epi32(m[11], m[6]); // psubd m5, m11, m6 ; t12a t13
    m[11] = _mm256_add_epi32(m[11], m[6]); // paddd m11, m6 ; t15a t14
    m[6] = _mm256_sub_epi32(m[10], m[4]); // psubd m6, m10, m4 ; t11a t10
    m[10] = _mm256_add_epi32(m[10], m[4]); // paddd m10, m4 ; t8a t9
    for x in [5, 6] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m5, m6
    }
    for x in [5, 6] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m5, m6
    }
    m[5] = _mm256_mullo_epi32(m[5], m[12]); // pmulld m5, m12
    m[6] = _mm256_mullo_epi32(m[6], m[12]); // pmulld m6, m12
    for x in [2, 3, 11, 10] {
        m[x] = _mm256_max_epi32(m[x], m[8]); // REPX {pmaxsd x, m8}, m2, m3, m11, m10
    }
    for x in [2, 3, 11, 10] {
        m[x] = _mm256_min_epi32(m[x], m[9]); // REPX {pminsd x, m9}, m2, m3, m11, m10
    }
}

/// `.pass1_main3` of `idct_16x4_internal_10bpc`: the final butterflies (expects `m7` = rounding)
#[inline(always)]
pub(crate) unsafe fn idct_16x4_internal_10bpc_pass1_main3(m: &mut M) {
    m[5] = _mm256_add_epi32(m[5], m[7]); // paddd m5, m7
    m[4] = _mm256_sub_epi32(m[5], m[6]); // psubd m4, m5, m6
    m[5] = _mm256_add_epi32(m[5], m[6]); // paddd m5, m6
    m[4] = _mm256_srai_epi32::<12>(m[4]); // psrad m4, 12 ; t11 t10a
    m[5] = _mm256_srai_epi32::<12>(m[5]); // psrad m5, 12 ; t12 t13a
    m[7] = _mm256_sub_epi32(m[0], m[11]); // psubd m7, m0, m11 ; out15 out14
    m[0] = _mm256_add_epi32(m[0], m[11]); // paddd m0, m11 ; out0 out1
    m[6] = _mm256_sub_epi32(m[1], m[5]); // psubd m6, m1, m5 ; out12 out13
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; out3 out2
    m[5] = _mm256_sub_epi32(m[2], m[4]); // psubd m5, m2, m4 ; out11 out10
    m[2] = _mm256_add_epi32(m[2], m[4]); // paddd m2, m4 ; out4 out5
    m[4] = _mm256_sub_epi32(m[3], m[10]); // psubd m4, m3, m10 ; out8 out9
    m[3] = _mm256_add_epi32(m[3], m[10]); // paddd m3, m10 ; out7 out6
    for x in [1, 3, 5, 7] {
        m[x] = _mm256_shuffle_epi32::<0x4e>(m[x]); // REPX {pshufd x, x, q1032}, m1, m3, m5, m7
    }
}

/// `.transpose_4x16_packed` of `idct_16x4_internal_10bpc` (`m0`-`m7` dwords in, `m0`-`m3` words out)
#[inline(always)]
unsafe fn idct_16x4_internal_10bpc_transpose_4x16_packed(m: &mut M) {
    m[8] = _mm256_broadcastsi128_si256(_mm_loadu_si128(deint_shuf.as_ptr().cast())); // vbroadcasti128 m8, [deint_shuf]
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3
    m[4] = _mm256_packs_epi32(m[4], m[5]); // packssdw m4, m5
    m[6] = _mm256_packs_epi32(m[6], m[7]); // packssdw m6, m7
    for x in [0, 2, 4, 6] {
        m[x] = _mm256_shuffle_epi8(m[x], m[8]); // REPX {pshufb x, m8}, m0, m2, m4, m6
    }
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2
    m[2] = _mm256_unpackhi_epi64(m[4], m[6]); // punpckhqdq m2, m4, m6
    m[4] = _mm256_unpacklo_epi64(m[4], m[6]); // punpcklqdq m4, m6
    m[3] = _mm256_permute2x128_si256::<0x31>(m[1], m[2]); // vperm2i128 m3, m1, m2, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[2])); // vinserti128 m1, xm2, 1
    m[2] = _mm256_permute2x128_si256::<0x31>(m[0], m[4]); // vperm2i128 m2, m0, m4, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[4])); // vinserti128 m0, xm4, 1
}

/// `.pass1` of `iadst_16x4_internal_10bpc` (up to `jmp tx2q`; expects `m12`, `m13` = clip min, max)
#[inline(always)]
unsafe fn iadst_16x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x16_internal_10bpc_main(m, c); // call m(iadst_4x16_internal_10bpc).main
    m[11] = _mm256_srai_epi32::<11>(m[11]); // psrad m11, 11 ; pd_1
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3
    }
    m[4] = _mm256_add_epi32(m[5], m[11]); // paddd m4, m5, m11
    m[5] = _mm256_add_epi32(m[6], m[11]); // paddd m5, m6, m11
    m[6] = _mm256_add_epi32(m[7], m[11]); // paddd m6, m7, m11
    m[7] = _mm256_add_epi32(m[8], m[11]); // paddd m7, m8, m11
    iadst_16x4_internal_10bpc_pass1_end(m)
}

/// `.pass1_end` of `iadst_16x4_internal_10bpc` (up to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_16x4_internal_10bpc_pass1_end(m: &mut M) {
    for x in [0, 2, 4, 6] {
        m[x] = _mm256_shuffle_epi32::<0x4e>(m[x]); // REPX {pshufd x, x, q1032}, m0, m2, m4, m6
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass2` of `iadst_16x4_internal_10bpc`
#[inline(always)]
unsafe fn iadst_16x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x4_internal_10bpc_transpose_4x16_packed(m); // call m(idct_16x4_internal_10bpc).transpose_4x16_packed
                                                       // lea r6, [deint_shuf+128] (no code here)
    iadst_16x4_internal_8bpc_main(m); // call m(iadst_16x4_internal_8bpc).main
    idct_16x4_internal_10bpc_end(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end
}

/// `.main` of `iadst_16x4_internal_10bpc`: the four-point adst of the rows of `cq` (`[cq+32*0..7]`), unrounded
/// (`m4`, `m5`, `m7`, `m9`, `m2`, `m0`, `m3`, `m1`, `m10`, `m8` as `.main_end` expects them)
#[inline(always)]
pub(crate) unsafe fn iadst_16x4_internal_10bpc_main(m: &mut M, c: Coef) {
    m[6] = pd(1321); // vpbroadcastd m6, [pd_1321]
    m[0] = ld_p4(c, 32 * 0); // mova m0, [cq+32*0]
    m[1] = ld_p4(c, 32 * 1); // mova m1, [cq+32*1]
    m[7] = pd(2482); // vpbroadcastd m7, [pd_2482]
    m[2] = ld_p4(c, 32 * 6); // mova m2, [cq+32*6]
    m[3] = ld_p4(c, 32 * 7); // mova m3, [cq+32*7]
    m[4] = _mm256_mullo_epi32(m[0], m[6]); // pmulld m4, m0, m6
    m[5] = _mm256_mullo_epi32(m[1], m[6]); // pmulld m5, m1, m6 ; 1321*in0
    m[9] = _mm256_mullo_epi32(m[2], m[7]); // pmulld m9, m2, m7
    m[8] = _mm256_mullo_epi32(m[3], m[7]); // pmulld m8, m3, m7 ; 2482*in3
    m[4] = _mm256_add_epi32(m[4], m[9]); // paddd m4, m9
    m[8] = _mm256_add_epi32(m[8], m[5]); // paddd m8, m5 ; 1321*in0 + 2482*in3
    m[5] = _mm256_mullo_epi32(m[0], m[7]); // pmulld m5, m0, m7
    m[9] = _mm256_mullo_epi32(m[1], m[7]); // pmulld m9, m1, m7 ; 2482*in0
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2
    m[1] = _mm256_add_epi32(m[1], m[3]); // paddd m1, m3 ; in0 + in3
    m[7] = _mm256_add_epi32(m[7], m[6]); // paddd m7, m6 ; pd_3803
    m[2] = _mm256_mullo_epi32(m[2], m[7]); // pmulld m2, m7
    m[3] = _mm256_mullo_epi32(m[3], m[7]); // pmulld m3, m7 ; 3803*in3
    m[5] = _mm256_sub_epi32(m[5], m[2]); // psubd m5, m2
    m[9] = _mm256_sub_epi32(m[9], m[3]); // psubd m9, m3 ; 2482*in0 - 3803*in3
    m[2] = ld_p4(c, 32 * 4); // mova m2, [cq+32*4]
    m[10] = _mm256_mullo_epi32(m[7], m[2]); // pmulld m10, m7, m2
    m[3] = _mm256_mullo_epi32(m[6], m[2]); // pmulld m3, m6, m2
    m[2] = _mm256_sub_epi32(m[2], m[0]); // psubd m2, m0
    m[0] = ld_p4(c, 32 * 5); // mova m0, [cq+32*5]
    m[7] = _mm256_mullo_epi32(m[7], m[0]); // pmulld m7, m0 ; 3803*in2
    m[6] = _mm256_mullo_epi32(m[6], m[0]); // pmulld m6, m0 ; 1321*in2
    m[0] = _mm256_sub_epi32(m[0], m[1]); // psubd m0, m1 ; in2 - in0 - in3
    m[1] = pd(-3344); // vpbroadcastd m1, [pd_m3344]
    m[4] = _mm256_add_epi32(m[4], m[10]); // paddd m4, m10
    m[7] = _mm256_add_epi32(m[7], m[8]); // paddd m7, m8 ; t0
    m[5] = _mm256_sub_epi32(m[5], m[3]); // psubd m5, m3
    m[9] = _mm256_sub_epi32(m[9], m[6]); // psubd m9, m6 ; t1
    m[2] = _mm256_mullo_epi32(m[2], m[1]); // pmulld m2, m1
    m[0] = _mm256_mullo_epi32(m[0], m[1]); // pmulld m0, m1 ; t2
    m[3] = _mm256_mullo_epi32(m[1], ld_p4(c, 32 * 2)); // pmulld m3, m1, [cq+32*2]
    m[1] = _mm256_mullo_epi32(m[1], ld_p4(c, 32 * 3)); // pmulld m1, [cq+32*3] ; -t3
}

/// `.main_end` of `iadst_16x4_internal_10bpc` (expects `m6` = rounding; out0-out7 unshifted in `m4`, `m5`, `m2`,
/// `m3`, `m8`, `m9`, `m6`, `m7`)
#[inline(always)]
pub(crate) unsafe fn iadst_16x4_internal_10bpc_main_end(m: &mut M) {
    m[5] = _mm256_add_epi32(m[5], m[6]); // paddd m5, m6
    m[9] = _mm256_add_epi32(m[9], m[6]); // paddd m9, m6
    m[10] = _mm256_add_epi32(m[4], m[5]); // paddd m10, m4, m5
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6
    m[8] = _mm256_add_epi32(m[7], m[6]); // paddd m8, m7, m6
    m[7] = _mm256_add_epi32(m[7], m[9]); // paddd m7, m9
    m[4] = _mm256_sub_epi32(m[4], m[3]); // psubd m4, m3 ; out0 (unshifted)
    m[5] = _mm256_sub_epi32(m[5], m[3]); // psubd m5, m3 ; out1 (unshifted)
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6 ; out2 (unshifted)
    m[3] = _mm256_add_epi32(m[3], m[10]); // paddd m3, m10 ; out3 (unshifted)
    m[8] = _mm256_sub_epi32(m[8], m[1]); // psubd m8, m1 ; out4 (unshifted)
    m[9] = _mm256_sub_epi32(m[9], m[1]); // psubd m9, m1 ; out5 (unshifted)
    m[6] = _mm256_add_epi32(m[6], m[0]); // paddd m6, m0 ; out6 (unshifted)
    m[7] = _mm256_add_epi32(m[7], m[1]); // paddd m7, m1 ; out7 (unshifted)
}

/// `.pass1` of `iflipadst_16x4_internal_10bpc` (up to `jmp tx2q`; expects `m12`, `m13` = clip min, max)
#[inline(always)]
unsafe fn iflipadst_16x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    iadst_4x16_internal_10bpc_main(m, c); // call m(iadst_4x16_internal_10bpc).main
    m[11] = _mm256_srai_epi32::<11>(m[11]); // psrad m11, 11 ; pd_1
    m[4] = _mm256_add_epi32(m[3], m[11]); // paddd m4, m3, m11
    m[3] = _mm256_add_epi32(m[5], m[11]); // paddd m3, m5, m11
    m[5] = _mm256_add_epi32(m[2], m[11]); // paddd m5, m2, m11
    m[2] = _mm256_add_epi32(m[6], m[11]); // paddd m2, m6, m11
    m[6] = _mm256_add_epi32(m[1], m[11]); // paddd m6, m1, m11
    m[1] = _mm256_add_epi32(m[7], m[11]); // paddd m1, m7, m11
    m[7] = _mm256_add_epi32(m[0], m[11]); // paddd m7, m0, m11
    m[0] = _mm256_add_epi32(m[8], m[11]); // paddd m0, m8, m11
    iadst_16x4_internal_10bpc_pass1_end(m) // jmp m(iadst_16x4_internal_10bpc).pass1_end
}

/// `.pass2` of `iflipadst_16x4_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_16x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x4_internal_10bpc_transpose_4x16_packed(m); // call m(idct_16x4_internal_10bpc).transpose_4x16_packed
                                                       // lea r6, [deint_shuf+128] (no code here)
    iadst_16x4_internal_8bpc_main(m); // call m(iadst_16x4_internal_8bpc).main
    m[4] = pw(2048); // vpbroadcastd m4, [pw_2048]
    m[5] = _mm256_mulhrs_epi16(m[3], m[4]); // pmulhrsw m5, m3, m4
    m[6] = _mm256_mulhrs_epi16(m[2], m[4]); // pmulhrsw m6, m2, m4
    m[2] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m2, m1, m4
    m[3] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m3, m0, m4
    m[0] = _mm256_add_epi16(m[5], ld_p4(dst, 0)); // paddw m0, m5, [dstq+strideq*0]
    m[1] = _mm256_add_epi16(m[6], ld_p4(dst, stride)); // paddw m1, m6, [dstq+strideq*1]
    m[5] = pw(0x3ff); // vpbroadcastd m5, [pixel_10bpc_max]
    idct_16x4_internal_10bpc_end3(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end3
}

/// `cglobal iidentity_16x4_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_16x4_internal_10bpc_pass1(m: &mut M, c: Coef) {
    m[8] = pd(5793); // vpbroadcastd m8, [pd_5793]
    for x in 0..8 {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * x as isize)); // vpermq mx, [cq+32*x], q3120
    }
    m[9] = pd(3072); // vpbroadcastd m9, [pd_3072]
    for x in 0..8 {
        m[x] = _mm256_mullo_epi32(m[x], m[8]); // REPX {pmulld x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass2` of `iidentity_16x4_internal_10bpc`
#[inline(always)]
unsafe fn iidentity_16x4_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x4_internal_10bpc_transpose_4x16_packed(m); // call m(idct_16x4_internal_10bpc).transpose_4x16_packed
    m[7] = pw(1697 * 8); // vpbroadcastd m7, [pw_1697x8]
    m[4] = _mm256_mulhrs_epi16(m[7], m[0]); // pmulhrsw m4, m7, m0
    m[5] = _mm256_mulhrs_epi16(m[7], m[1]); // pmulhrsw m5, m7, m1
    m[6] = _mm256_mulhrs_epi16(m[7], m[2]); // pmulhrsw m6, m7, m2
    m[7] = _mm256_mulhrs_epi16(m[7], m[3]); // pmulhrsw m7, m3
    m[0] = _mm256_adds_epi16(m[0], m[4]); // paddsw m0, m4
    m[1] = _mm256_adds_epi16(m[1], m[5]); // paddsw m1, m5
    m[2] = _mm256_adds_epi16(m[2], m[6]); // paddsw m2, m6
    m[3] = _mm256_adds_epi16(m[3], m[7]); // paddsw m3, m7
    idct_16x4_internal_10bpc_end(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end
}

/// `.pass2` of `idct_16x4_internal_12bpc`
#[inline(always)]
unsafe fn idct_16x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in 0..8 {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    // deinterleave
    for x in 0..8 {
        m[x] = _mm256_shuffle_epi32::<0xd8>(m[x]); // REPX {pshufd x, x, q3120}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    // transpose
    m[8] = _mm256_unpacklo_epi64(m[0], m[1]); // punpcklqdq m8, m0, m1
    m[0] = _mm256_unpackhi_epi64(m[0], m[1]); // punpckhqdq m0, m1
    m[9] = _mm256_unpacklo_epi64(m[2], m[3]); // punpcklqdq m9, m2, m3
    m[2] = _mm256_unpackhi_epi64(m[2], m[3]); // punpckhqdq m2, m3
    m[10] = _mm256_unpacklo_epi64(m[4], m[5]); // punpcklqdq m10, m4, m5
    m[4] = _mm256_unpackhi_epi64(m[4], m[5]); // punpckhqdq m4, m5
    m[11] = _mm256_unpacklo_epi64(m[6], m[7]); // punpcklqdq m11, m6, m7
    m[6] = _mm256_unpackhi_epi64(m[6], m[7]); // punpckhqdq m6, m7
    m[3] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m3, m0, m2, 0x31 ; out6
    m[1] = _mm256_permute2x128_si256::<0x20>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x20 ; out2
    m[7] = _mm256_permute2x128_si256::<0x31>(m[4], m[6]); // vperm2i128 m7, m4, m6, 0x31 ; out7
    m[5] = _mm256_permute2x128_si256::<0x20>(m[4], m[6]); // vperm2i128 m5, m4, m6, 0x20 ; out3
    m[13] = _mm256_permute2x128_si256::<0x31>(m[10], m[11]); // vperm2i128 m13, m10, m11, 0x31 ; out5
    m[12] = _mm256_permute2x128_si256::<0x20>(m[10], m[11]); // vperm2i128 m12, m10, m11, 0x20 ; out1
    m[11] = _mm256_permute2x128_si256::<0x31>(m[8], m[9]); // vperm2i128 m11, m8, m9, 0x31 ; out4
    m[10] = _mm256_permute2x128_si256::<0x20>(m[8], m[9]); // vperm2i128 m10, m8, m9, 0x20 ; out0
    idct_4x16_internal_10bpc_pass1_main(m); // call m(idct_4x16_internal_10bpc).pass1_main
    m[0] = _mm256_mullo_epi32(m[6], m[10]); // pmulld m0, m6, m10
    m[2] = _mm256_mullo_epi32(m[6], m[11]); // pmulld m2, m6, m11
    m[4] = _mm256_mullo_epi32(m[6], m[12]); // pmulld m4, m6, m12
    m[6] = _mm256_mullo_epi32(m[6], m[13]); // pmulld m6, m13
    m[10] = pd(17408); // vpbroadcastd m10, [pd_17408]
    idct_4x16_internal_10bpc_pass1_main2(m); // call m(idct_4x16_internal_10bpc).pass1_main2
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    }
    idct_16x4_internal_10bpc_end2(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end2
}

/// `.pass2` of `iadst_16x4_internal_12bpc`
#[inline(always)]
unsafe fn iadst_16x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_16x4_internal_12bpc_pass2_main(m, c); // call .pass2_main
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[4]); // REPX {pmulhrsw x, m4}, m0, m1, m2, m3
    }
    idct_16x4_internal_10bpc_end2(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end2
}

/// `.pass2_main` of `iadst_16x4_internal_12bpc` (uses `[cq+32*0..7]` as scratch; leaves `m4` = `pw_16384`, `m5` =
/// pixel max)
#[inline(always)]
unsafe fn iadst_16x4_internal_12bpc_pass2_main(m: &mut M, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in [0, 1, 2, 3, 6, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m6, m7
    }
    m[8] = _mm256_max_epi32(m[4], m[12]); // pmaxsd m8, m4, m12
    m[9] = _mm256_max_epi32(m[5], m[12]); // pmaxsd m9, m5, m12
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
    }
    iadst_8x4_internal_12bpc_transpose_4x8(m); // call m(iadst_8x4_internal_12bpc).transpose_4x8
    st_p4(c, 32 * 0, m[0]); // mova [cq+32*0], m0
    st_p4(c, 32 * 2, m[1]); // mova [cq+32*2], m1
    st_p4(c, 32 * 4, m[2]); // mova [cq+32*4], m2
    st_p4(c, 32 * 6, m[3]); // mova [cq+32*6], m3
    m[0] = _mm256_min_epi32(m[8], m[13]); // pminsd m0, m8, m13
    m[1] = _mm256_min_epi32(m[9], m[13]); // pminsd m1, m9, m13
    m[2] = _mm256_min_epi32(m[6], m[13]); // pminsd m2, m6, m13
    m[3] = _mm256_min_epi32(m[7], m[13]); // pminsd m3, m7, m13
    iadst_8x4_internal_12bpc_transpose_4x8(m); // call m(iadst_8x4_internal_12bpc).transpose_4x8
    st_p4(c, 32 * 1, m[0]); // mova [cq+32*1], m0
    st_p4(c, 32 * 3, m[1]); // mova [cq+32*3], m1
    st_p4(c, 32 * 5, m[2]); // mova [cq+32*5], m2
    st_p4(c, 32 * 7, m[3]); // mova [cq+32*7], m3
    iadst_16x4_internal_10bpc_main(m, c); // call m(iadst_16x4_internal_10bpc).main
    m[6] = pd(2048); // vpbroadcastd m6, [pd_2048]
    iadst_16x4_internal_10bpc_main_end(m); // call m(iadst_16x4_internal_10bpc).main_end
    m[0] = _mm256_srai_epi32::<15>(m[4]); // psrad m0, m4, 15
    m[1] = _mm256_srai_epi32::<15>(m[5]); // psrad m1, m5, 15
    m[2] = _mm256_srai_epi32::<15>(m[2]); // psrad m2, 15
    m[3] = _mm256_srai_epi32::<15>(m[3]); // psrad m3, 15
    m[4] = _mm256_srai_epi32::<15>(m[8]); // psrad m4, m8, 15
    m[5] = _mm256_srai_epi32::<15>(m[9]); // psrad m5, m9, 15
    m[6] = _mm256_srai_epi32::<15>(m[6]); // psrad m6, 15
    m[7] = _mm256_srai_epi32::<15>(m[7]); // psrad m7, 15
    m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
    m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
    m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
    m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
    m[4] = pw(16384); // vpbroadcastd m4, [pw_16384]
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
}

/// `.pass2` of `iflipadst_16x4_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_16x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    iadst_16x4_internal_12bpc_pass2_main(m, c); // call m(iadst_16x4_internal_12bpc).pass2_main
    m[7] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m7, m0, q3120
    m[6] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m6, m1, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m1, m2, q3120
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[3]); // vpermq m0, m3, q3120
    m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
    m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
    m[2] = _mm256_mulhrs_epi16(m[6], m[4]); // pmulhrsw m2, m6, m4
    m[3] = _mm256_mulhrs_epi16(m[7], m[4]); // pmulhrsw m3, m7, m4
    idct_16x4_internal_10bpc_end2(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end2
}

/// `cglobal iidentity_16x4_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_16x4_internal_12bpc_pass1(m: &mut M, c: Coef) {
    m[8] = pd(1697); // vpbroadcastd m8, [pd_1697]
    m[0] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 0)); // vpermq m0, [cq+32*0], q3120 ; 0 1
    m[1] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 1)); // vpermq m1, [cq+32*1], q3120 ; 2 3
    m[2] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 2)); // vpermq m2, [cq+32*2], q3120 ; 4 5
    m[3] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 3)); // vpermq m3, [cq+32*3], q3120 ; 6 7
    m[9] = pd(3072); // vpbroadcastd m9, [pd_3072]
    m[4] = _mm256_mullo_epi32(m[8], m[0]); // pmulld m4, m8, m0
    m[5] = _mm256_mullo_epi32(m[8], m[1]); // pmulld m5, m8, m1
    m[6] = _mm256_mullo_epi32(m[8], m[2]); // pmulld m6, m8, m2
    m[7] = _mm256_mullo_epi32(m[8], m[3]); // pmulld m7, m8, m3
    m[10] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 4)); // vpermq m10, [cq+32*4], q3120 ; 8 9
    m[11] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 5)); // vpermq m11, [cq+32*5], q3120 ; a b
    m[12] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 6)); // vpermq m12, [cq+32*6], q3120 ; c d
    m[13] = _mm256_permute4x64_epi64::<0xd8>(ld_p4(c, 32 * 7)); // vpermq m13, [cq+32*7], q3120 ; e f
    for x in [4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m4, m5, m6, m7
    }
    for x in [4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m4, m5, m6, m7
    }
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4
    m[4] = _mm256_mullo_epi32(m[8], m[10]); // pmulld m4, m8, m10
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5
    m[5] = _mm256_mullo_epi32(m[8], m[11]); // pmulld m5, m8, m11
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6
    m[6] = _mm256_mullo_epi32(m[8], m[12]); // pmulld m6, m8, m12
    m[3] = _mm256_add_epi32(m[3], m[7]); // paddd m3, m7
    m[7] = _mm256_mullo_epi32(m[8], m[13]); // pmulld m7, m8, m13
    for x in [4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m4, m5, m6, m7
    }
    for x in [4, 5, 6, 7] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m4, m5, m6, m7
    }
    m[4] = _mm256_add_epi32(m[4], m[10]); // paddd m4, m10
    m[5] = _mm256_add_epi32(m[5], m[11]); // paddd m5, m11
    m[6] = _mm256_add_epi32(m[6], m[12]); // paddd m6, m12
    m[7] = _mm256_add_epi32(m[7], m[13]); // paddd m7, m13
}

/// `.pass2` of `iidentity_16x4_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_16x4_internal_12bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    for x in 0..8 {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[8] = pd(5793); // vpbroadcastd m8, [pd_5793]
    m[9] = pd(2048); // vpbroadcastd m9, [pd_2048]
    for x in 0..8 {
        m[x] = _mm256_mullo_epi32(m[x], m[8]); // REPX {pmulld x, m8}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[9]); // REPX {paddd x, m9}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<15>(m[x]); // REPX {psrad x, 15}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_16x4_internal_10bpc_transpose_4x16_packed(m); // call m(idct_16x4_internal_10bpc).transpose_4x16_packed
    m[4] = pw(16384); // vpbroadcastd m4, [pw_16384]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[4]); // REPX {pmulhrsw x, m4}, m0, m1, m2, m3
    }
    m[5] = pw(0xfff); // vpbroadcastd m5, [pixel_12bpc_max]
    idct_16x4_internal_10bpc_end2(m, dst, stride, c) // jmp m(idct_16x4_internal_10bpc).end2
}

// --- 16x8 ---------------------------------------------------------------------------------------------------------
//
// The 16x8 `_internal` functions have a frame of 32*8 bytes (`cglobal …, 32*8`): `rsp` is the frame's start, and
// the first passes set `r6` to `rsp+32*4`. The frame is a `[__m256i; 8]` of the size function, zeroed whole
// (`[_mm256_setzero_si256(); 8]`); every load from it reads a slot that the same first pass wrote before.

/// `%macro INV_TXFM_16X8_FN type1, type2, 10`: the DC-only path of dct_dct, then the two passes. The clipping
/// constants that each `cglobal i<type1>_16x8_internal_10bpc` loads before its `.pass1` are loaded here.
#[inline(always)]
unsafe fn inv_txfm_add_16x8_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        return inv_txfm_add_16x8_dconly_p4(m, dst, stride, c, eob, pw(0x7c00)); // vpbroadcastd m3, [dconly_10bpc]
    }
    let mut frame = [_mm256_setzero_si256(); 8];
    let rsp = frame.as_mut_ptr();
    match t1 {
        K::Dct => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            idct_16x8_internal_10bpc_pass1(m, c, rsp)
        }
        K::Adst => {
            m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
            m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
            iadst_16x8_internal_10bpc_pass1(m, c, rsp)
        }
        K::FlipAdst => {
            m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
            m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
            iflipadst_16x8_internal_10bpc_pass1(m, c, rsp)
        }
        K::Identity => iidentity_16x8_internal_10bpc_pass1(m, c, rsp),
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x8_internal_10bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_16x8_internal_10bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_16x8_internal_10bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_16x8_internal_10bpc_pass2(m, dst, stride, c),
    }
}

/// `%macro INV_TXFM_16X8_FN type1, type2, 12` (the first passes are the 10 bpc `.pass1`s with the 20-bit clipping
/// constants; identity has none)
#[inline(always)]
unsafe fn inv_txfm_add_16x8_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        // test eobd, eobd / jnz %%p1
        return inv_txfm_add_16x8_dconly_p4(m, dst, stride, c, eob, pw(0x7000)); // vpbroadcastd m3, [dconly_12bpc]
    }
    let mut frame = [_mm256_setzero_si256(); 8];
    let rsp = frame.as_mut_ptr();
    match t1 {
        K::Dct => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            idct_16x8_internal_10bpc_pass1(m, c, rsp) // jmp m(idct_16x8_internal_10bpc).pass1
        }
        K::Adst => {
            m[13] = pd(-0x80000); // vpbroadcastd m13, [clip_20b_min]
            m[14] = pd(0x7ffff); // vpbroadcastd m14, [clip_20b_max]
            iadst_16x8_internal_10bpc_pass1(m, c, rsp) // jmp m(iadst_16x8_internal_10bpc).pass1
        }
        K::FlipAdst => {
            m[13] = pd(-0x80000); // vpbroadcastd m13, [clip_20b_min]
            m[14] = pd(0x7ffff); // vpbroadcastd m14, [clip_20b_max]
            iflipadst_16x8_internal_10bpc_pass1(m, c, rsp) // jmp m(iflipadst_16x8_internal_10bpc).pass1
        }
        K::Identity => iidentity_16x8_internal_10bpc_pass1(m, c, rsp), // jmp m(iidentity_16x8_internal_10bpc).pass1
    }
    // jmp tx2q
    match t2 {
        K::Dct => idct_16x8_internal_12bpc_pass2(m, dst, stride, c),
        K::Adst => iadst_16x8_internal_12bpc_pass2(m, dst, stride, c),
        K::FlipAdst => iflipadst_16x8_internal_12bpc_pass2(m, dst, stride, c),
        K::Identity => iidentity_16x8_internal_12bpc_pass2(m, dst, stride, c),
    }
}

/// The DC-only path of `INV_TXFM_16X8_FN dct, dct` (both bit depths; `dconly` is the bit depth's
/// `dconly_<bpc>bpc`)
#[inline(always)]
unsafe fn inv_txfm_add_16x8_dconly_p4(m: &mut M, dst: Dst, stride: isize, c: Coef, eob: c_int, dconly: __m256i) {
    let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
    m[3] = dconly; // vpbroadcastd m3, [dconly_%3bpc]
    c.write_unaligned(eob); // mov [cq], eobd ; 0
    let r3d = eob | 8; // or r3d, 8
    r6d = r6d.wrapping_add(128); // add r6d, 128
    r6d >>= 8; // sar r6d, 8
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    inv_txfm_add_dct_dct_16x4_10bpc_dconly2(m, dst, stride, r6d, r3d) // jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly2
}

/// `.pass1` of `idct_16x8_internal_10bpc` (up to `jmp tx2q`; expects `m12`, `m13` = clip min, max; `rsp` = the
/// frame)
#[inline(always)]
unsafe fn idct_16x8_internal_10bpc_pass1(m: &mut M, c: Coef, rsp: *mut __m256i) {
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    m[0] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 1)); // pmulld m0, m14, [cq+32* 1]
    m[1] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 3)); // pmulld m1, m14, [cq+32* 3]
    m[2] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 5)); // pmulld m2, m14, [cq+32* 5]
    m[3] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 7)); // pmulld m3, m14, [cq+32* 7]
    m[4] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 9)); // pmulld m4, m14, [cq+32* 9]
    m[5] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 11)); // pmulld m5, m14, [cq+32*11]
    m[6] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 13)); // pmulld m6, m14, [cq+32*13]
    m[7] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 15)); // pmulld m7, m14, [cq+32*15]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    let r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    idct_8x16_internal_10bpc_main_oddhalf_rect2(m, r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    m[0] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 0)); // pmulld m0, m14, [cq+32* 0]
    m[1] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 2)); // pmulld m1, m14, [cq+32* 2]
    m[2] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 4)); // pmulld m2, m14, [cq+32* 4]
    m[3] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 6)); // pmulld m3, m14, [cq+32* 6]
    m[4] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 8)); // pmulld m4, m14, [cq+32* 8]
    m[5] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 10)); // pmulld m5, m14, [cq+32*10]
    m[6] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 12)); // pmulld m6, m14, [cq+32*12]
    m[7] = _mm256_mullo_epi32(m[14], ld_p4(c, 32 * 14)); // pmulld m7, m14, [cq+32*14]
    idct_8x8_internal_10bpc_main_rect2(m); // call m(idct_8x8_internal_10bpc).main_rect2
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[11] = _mm256_srli_epi32::<11>(m[11]); // psrld m11, 11 ; pd_1
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_16x8_internal_10bpc_pass1_rotations(m, r6); // call .pass1_rotations
    for x in 0..16 {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m0, m1, m2, m3, m4, m5, m6, m7, m8, …, m15
    }
}

/// `.pass2` of `idct_16x8_internal_10bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x8_internal_10bpc_transpose(m); // call .transpose
    idct_16x8_internal_8bpc_main(m); // call m(idct_16x8_internal_8bpc).main
    m[10] = pw(2048); // vpbroadcastd m10, [pw_2048]
    idct_16x8_internal_10bpc_end(m, dst, stride, c)
}

/// `.end` of `idct_16x8_internal_10bpc` (expects `m10` = the rounding multiplier; falls into `.end2`)
#[inline(always)]
unsafe fn idct_16x8_internal_10bpc_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    m[0] = _mm256_mulhrs_epi16(m[0], m[10]); // pmulhrsw m0, m10
    m[1] = _mm256_mulhrs_epi16(m[1], m[10]); // pmulhrsw m1, m10
    m[2] = _mm256_mulhrs_epi16(m[2], m[10]); // pmulhrsw m2, m10
    m[3] = _mm256_mulhrs_epi16(m[3], m[10]); // pmulhrsw m3, m10
    let mut r3 = 0;
    idct_16x8_internal_10bpc_write_16x4_start(m, &mut dst, stride, &mut c, &mut r3); // call .write_16x4_start
    idct_16x8_internal_10bpc_end2(m, dst, stride, c, r3)
}

/// `.end2` of `idct_16x8_internal_10bpc` (expects `m10` = the rounding multiplier, `m8`, `m9` = 0, pixel max,
/// `r3` = `strideq*3`; ends in `RET`)
#[inline(always)]
unsafe fn idct_16x8_internal_10bpc_end2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef, r3: isize) {
    m[0] = _mm256_mulhrs_epi16(m[4], m[10]); // pmulhrsw m0, m4, m10
    m[1] = _mm256_mulhrs_epi16(m[5], m[10]); // pmulhrsw m1, m5, m10
    m[2] = _mm256_mulhrs_epi16(m[6], m[10]); // pmulhrsw m2, m6, m10
    m[3] = _mm256_mulhrs_epi16(m[7], m[10]); // pmulhrsw m3, m7, m10
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call .write_16x4_zero
}

/// `.pass1_rotations` of `idct_16x8_internal_10bpc`: the last butterflies of the 16-point dct, the even half in
/// `m0`-`m7` and the odd half in `[r6-32*4]` … `[r6+32*3]`; out0-out15 in `m0`-`m15`.
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_10bpc_pass1_rotations(m: &mut M, r6: *mut __m256i) {
    m[14] = ld_p4(r6, -32 * 4); // mova m14, [r6-32*4]
    m[13] = ld_p4(r6, -32 * 3); // mova m13, [r6-32*3]
    m[12] = ld_p4(r6, -32 * 2); // mova m12, [r6-32*2]
    m[11] = ld_p4(r6, -32 * 1); // mova m11, [r6-32*1]
    m[10] = ld_p4(r6, 32 * 0); // mova m10, [r6+32*0]
    m[9] = ld_p4(r6, 32 * 1); // mova m9, [r6+32*1]
    m[8] = ld_p4(r6, 32 * 2); // mova m8, [r6+32*2]
    m[15] = _mm256_sub_epi32(m[0], m[14]); // psubd m15, m0, m14 ; out15
    m[0] = _mm256_add_epi32(m[0], m[14]); // paddd m0, m14 ; out0
    m[14] = _mm256_sub_epi32(m[1], m[13]); // psubd m14, m1, m13 ; out14
    m[1] = _mm256_add_epi32(m[1], m[13]); // paddd m1, m13 ; out1
    m[13] = _mm256_sub_epi32(m[2], m[12]); // psubd m13, m2, m12 ; out13
    m[2] = _mm256_add_epi32(m[2], m[12]); // paddd m2, m12 ; out2
    m[12] = _mm256_sub_epi32(m[3], m[11]); // psubd m12, m3, m11 ; out12
    m[3] = _mm256_add_epi32(m[3], m[11]); // paddd m3, m11 ; out3
    m[11] = _mm256_sub_epi32(m[4], m[10]); // psubd m11, m4, m10 ; out11
    m[4] = _mm256_add_epi32(m[4], m[10]); // paddd m4, m10 ; out4
    m[10] = _mm256_sub_epi32(m[5], m[9]); // psubd m10, m5, m9 ; out10
    m[5] = _mm256_add_epi32(m[5], m[9]); // paddd m5, m9 ; out5
    m[9] = _mm256_sub_epi32(m[6], m[8]); // psubd m9, m6, m8 ; out9
    m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8 ; out6
    m[8] = _mm256_sub_epi32(m[7], ld_p4(r6, 32 * 3)); // psubd m8, m7, [r6+32*3] ; out8
    m[7] = _mm256_add_epi32(m[7], ld_p4(r6, 32 * 3)); // paddd m7, [r6+32*3] ; out7
}

/// `.transpose` of `idct_16x8_internal_10bpc` (`lea r6, [deint_shuf+128]`, the 8 bpc `.main`s' `o()` base, has
/// no code here; falls into `.transpose2`)
#[inline(always)]
unsafe fn idct_16x8_internal_10bpc_transpose(m: &mut M) {
    idct_16x8_internal_10bpc_transpose2(m)
}

/// `.transpose2` of `idct_16x8_internal_10bpc`: packs `m0`-`m15` (dwords) to words and transposes them (falls into
/// `.transpose3`); the 16x8 block in `m0`-`m7` (words)
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_10bpc_transpose2(m: &mut M) {
    m[0] = _mm256_packs_epi32(m[0], m[8]); // packssdw m0, m8
    m[1] = _mm256_packs_epi32(m[1], m[9]); // packssdw m1, m9
    m[2] = _mm256_packs_epi32(m[2], m[10]); // packssdw m2, m10
    m[3] = _mm256_packs_epi32(m[3], m[11]); // packssdw m3, m11
    m[4] = _mm256_packs_epi32(m[4], m[12]); // packssdw m4, m12
    m[5] = _mm256_packs_epi32(m[5], m[13]); // packssdw m5, m13
    m[6] = _mm256_packs_epi32(m[6], m[14]); // packssdw m6, m14
    m[7] = _mm256_packs_epi32(m[7], m[15]); // packssdw m7, m15
    idct_16x8_internal_10bpc_transpose3(m)
}

/// `.transpose3` of `idct_16x8_internal_10bpc`: transposes the words of `m0`-`m7` (uses `m8`)
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_10bpc_transpose3(m: &mut M) {
    m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m1, m2, m3
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m3, m4, m5
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
    m[5] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m5, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m7, m4, m6
    m[4] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m4, m6
    m[6] = _mm256_unpacklo_epi32(m[8], m[2]); // punpckldq m6, m8, m2
    m[8] = _mm256_unpackhi_epi32(m[8], m[2]); // punpckhdq m8, m2
    m[2] = _mm256_unpackhi_epi32(m[0], m[1]); // punpckhdq m2, m0, m1
    m[0] = _mm256_unpacklo_epi32(m[0], m[1]); // punpckldq m0, m1
    m[1] = _mm256_unpackhi_epi32(m[3], m[5]); // punpckhdq m1, m3, m5
    m[3] = _mm256_unpacklo_epi32(m[3], m[5]); // punpckldq m3, m5
    m[5] = _mm256_unpacklo_epi64(m[6], m[3]); // punpcklqdq m5, m6, m3
    m[6] = _mm256_unpackhi_epi64(m[6], m[3]); // punpckhqdq m6, m3
    m[3] = _mm256_unpackhi_epi64(m[2], m[7]); // punpckhqdq m3, m2, m7
    m[2] = _mm256_unpacklo_epi64(m[2], m[7]); // punpcklqdq m2, m7
    m[7] = _mm256_unpacklo_epi64(m[8], m[1]); // punpcklqdq m7, m8, m1
    m[8] = _mm256_unpackhi_epi64(m[8], m[1]); // punpckhqdq m8, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m1, m0, m4
    m[0] = _mm256_unpacklo_epi64(m[0], m[4]); // punpcklqdq m0, m4
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[5]); // vperm2i128 m4, m0, m5, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[5])); // vinserti128 m0, xm5, 1
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[6]); // vperm2i128 m5, m1, m6, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[6])); // vinserti128 m1, xm6, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[2], m[7]); // vperm2i128 m6, m2, m7, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[7])); // vinserti128 m2, xm7, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[3], m[8]); // vperm2i128 m7, m3, m8, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[8])); // vinserti128 m3, xm8, 1
}

/// `.write_16x4_start` of `idct_16x8_internal_10bpc` (falls into `.write_16x4_zero`): sets `m9` = 10 bpc pixel
/// max, `r3` = `strideq*3`, `m8` = 0, then clears `[cq+32*0..7]`, advances `cq` by `32*8`, adds `m0`-`m3` (words)
/// to four rows of 16 pixels at `dstq` and advances `dstq` by four rows.
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_10bpc_write_16x4_start(
    m: &mut M,
    dst: &mut Dst,
    stride: isize,
    c: &mut Coef,
    r3: &mut isize,
) {
    m[9] = pw(0x3ff); // vpbroadcastd m9, [pixel_10bpc_max]
    *r3 = stride * 3; // lea r3, [strideq*3]
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, *r3)
}

/// `.write_16x4_zero` of `idct_16x8_internal_10bpc` (falls into `.write_16x4`): clears `[cq+32*0..7]` with `m8`
/// (zero), advances `cq` by `32*8`, then `.write_16x4` (expects `m8`, `m9` = 0, pixel max, `r3` = `strideq*3`).
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_10bpc_write_16x4_zero(
    m: &mut M,
    dst: &mut Dst,
    stride: isize,
    c: &mut Coef,
    r3: isize,
) {
    for x in 0..8 {
        st_p4(*c, 32 * x, m[8]); // REPX {mova [cq+32*x], m8}, 0, 1, 2, 3, 4, 5, 6, 7
    }
    *c = at(*c, 32 * 8); // add cq, 32*8
    idct_16x8_internal_10bpc_write_16x4(m, dst, stride, r3)
}

/// `.write_16x4` of `idct_16x8_internal_10bpc`: adds `m0`-`m3` (words) to four rows of 16 pixels at `dstq`,
/// clamped to `m8`..`m9` (0, pixel max; `r3` = `strideq*3`), and advances `dstq` by four rows.
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_10bpc_write_16x4(m: &mut M, dst: &mut Dst, stride: isize, r3: isize) {
    let d = *dst;
    m[0] = _mm256_add_epi16(m[0], ld_p4(d, 0)); // paddw m0, [dstq+strideq*0]
    m[1] = _mm256_add_epi16(m[1], ld_p4(d, stride)); // paddw m1, [dstq+strideq*1]
    m[2] = _mm256_add_epi16(m[2], ld_p4(d, stride * 2)); // paddw m2, [dstq+strideq*2]
    m[3] = _mm256_add_epi16(m[3], ld_p4(d, r3)); // paddw m3, [dstq+r3 ]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_max_epi16(m[x], m[8]); // REPX {pmaxsw x, m8}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi16(m[x], m[9]); // REPX {pminsw x, m9}, m0, m1, m2, m3
    }
    st_p4(d, 0, m[0]); // mova [dstq+strideq*0], m0
    st_p4(d, stride, m[1]); // mova [dstq+strideq*1], m1
    st_p4(d, stride * 2, m[2]); // mova [dstq+strideq*2], m2
    st_p4(d, r3, m[3]); // mova [dstq+r3 ], m3
    *dst = at(d, stride * 4); // lea dstq, [dstq+strideq*4]
}

/// `.pass1` of `iadst_16x8_internal_10bpc` (up to `jmp tx2q`; expects `m13`, `m14` = clip min, max; `rsp` = the
/// frame)
#[inline(always)]
unsafe fn iadst_16x8_internal_10bpc_pass1(m: &mut M, c: Coef, rsp: *mut __m256i) {
    let r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    iadst_16x8_internal_10bpc_main(m, c, r6); // call .main
    m[14] = pd(3072); // vpbroadcastd m14, [pd_3072]
    m[15] = _mm256_srli_epi32::<11>(m[15]); // psrld m15, 11 ; pd_1
    m[13] = _mm256_sub_epi32(m[14], m[15]); // psubd m13, m14, m15 ; pd_3071
    iadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call .pass1_rotations
    iadst_16x8_internal_10bpc_pass1_end(m)
}

/// `.pass1_end` of `iadst_16x8_internal_10bpc` (up to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_16x8_internal_10bpc_pass1_end(m: &mut M) {
    for x in [0, 1, 2, 3, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1 }, m0, m1, m2, m3, m12, m13, m14, m15
    }
    for x in [4, 5, 6, 7, 8, 9, 10, 11] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m4, m5, m6, m7, m8, m9, m10, m11
    }
}

/// `.pass2` of `iadst_16x8_internal_10bpc`
#[inline(always)]
unsafe fn iadst_16x8_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    idct_16x8_internal_10bpc_transpose(m); // call m(idct_16x8_internal_10bpc).transpose
    iadst_16x8_internal_8bpc_main(m); // call m(iadst_16x8_internal_8bpc).main
    iadst_16x8_internal_8bpc_main_pass2_end(m); // call m(iadst_16x8_internal_8bpc).main_pass2_end
    m[10] = pw(2048); // vpbroadcastd m10, [pw_2048]
    m[11] = _mm256_setzero_si256(); // pxor m11, m11
    m[11] = _mm256_sub_epi16(m[11], m[10]); // psubw m11, m10
    m[0] = _mm256_mulhrs_epi16(m[0], m[10]); // pmulhrsw m0, m10
    m[1] = _mm256_mulhrs_epi16(m[1], m[11]); // pmulhrsw m1, m11
    m[2] = _mm256_mulhrs_epi16(m[2], m[10]); // pmulhrsw m2, m10
    m[3] = _mm256_mulhrs_epi16(m[3], m[11]); // pmulhrsw m3, m11
    let mut r3 = 0;
    idct_16x8_internal_10bpc_write_16x4_start(m, &mut dst, stride, &mut c, &mut r3); // call m(idct_16x8_internal_10bpc).write_16x4_start
    m[0] = _mm256_mulhrs_epi16(m[4], m[10]); // pmulhrsw m0, m4, m10
    m[1] = _mm256_mulhrs_epi16(m[5], m[11]); // pmulhrsw m1, m5, m11
    m[2] = _mm256_mulhrs_epi16(m[6], m[10]); // pmulhrsw m2, m6, m10
    m[3] = _mm256_mulhrs_epi16(m[7], m[11]); // pmulhrsw m3, m7, m11
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
}

/// `.pass1_rotations` of `iadst_16x8_internal_10bpc`: the rounding (`(x + 1) >> 1` with `m15` = `pd_1`, `(3072 ±
/// x) >> 12` with `m14` = `pd_3072`, `m13` = `pd_3071`) and signs of the 16-point adst outputs of `.main` (out0-out15
/// in `m0`-`m15`, still to be shifted; reads `[r6-32*4]` … `[r6-32*1]`)
#[inline(always)]
pub(crate) unsafe fn iadst_16x8_internal_10bpc_pass1_rotations(m: &mut M, r6: *mut __m256i) {
    m[0] = _mm256_add_epi32(m[0], m[15]); // paddd m0, m15
    m[1] = _mm256_sub_epi32(m[15], m[1]); // psubd m1, m15, m1
    m[2] = _mm256_add_epi32(m[2], m[15]); // paddd m2, m15
    m[3] = _mm256_sub_epi32(m[15], m[3]); // psubd m3, m15, m3
    m[4] = _mm256_add_epi32(m[4], m[14]); // paddd m4, m14
    m[5] = _mm256_sub_epi32(m[13], m[5]); // psubd m5, m13, m5
    m[6] = _mm256_add_epi32(m[6], m[14]); // paddd m6, m14
    m[7] = _mm256_sub_epi32(m[13], m[7]); // psubd m7, m13, m7
    m[8] = _mm256_add_epi32(m[14], m[9]); // paddd m8, m14, m9
    m[9] = _mm256_sub_epi32(m[13], m[10]); // psubd m9, m13, m10
    m[10] = _mm256_add_epi32(m[14], m[11]); // paddd m10, m14, m11
    m[11] = _mm256_sub_epi32(m[13], m[12]); // psubd m11, m13, m12
    m[12] = _mm256_add_epi32(m[15], ld_p4(r6, -32 * 1)); // paddd m12, m15, [r6-32*1]
    m[13] = _mm256_sub_epi32(m[15], ld_p4(r6, -32 * 2)); // psubd m13, m15, [r6-32*2]
    m[14] = _mm256_add_epi32(m[15], ld_p4(r6, -32 * 3)); // paddd m14, m15, [r6-32*3]
    m[15] = _mm256_sub_epi32(m[15], ld_p4(r6, -32 * 4)); // psubd m15, [r6-32*4]
}

/// `.main` of `iadst_16x8_internal_10bpc`: the 16-point adst of the rows of `cq` (`[cq+32*0..15]`, scaled by
/// 2896) through `.main_part1` and `.main_part2` (expects `m13`, `m14` = clip min, max; uses `[r6-32*4]` …
/// `[r6+32*3]`; leaves `m15` = `pd_2896`)
#[inline(always)]
unsafe fn iadst_16x8_internal_10bpc_main(m: &mut M, c: Coef, r6: *mut __m256i) {
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    m[0] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 2)); // pmulld m0, m15, [cq+32* 2]
    m[1] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 13)); // pmulld m1, m15, [cq+32*13]
    m[2] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 6)); // pmulld m2, m15, [cq+32* 6]
    m[3] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 9)); // pmulld m3, m15, [cq+32* 9]
    m[4] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 10)); // pmulld m4, m15, [cq+32*10]
    m[5] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 5)); // pmulld m5, m15, [cq+32* 5]
    m[6] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 14)); // pmulld m6, m15, [cq+32*14]
    m[7] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 1)); // pmulld m7, m15, [cq+32* 1]
    m[12] = pd(2048); // vpbroadcastd m12, [pd_2048]
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[12]); // REPX {paddd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_16x8_internal_10bpc_main_part1(m, r6); // call .main_part1
    m[0] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 0)); // pmulld m0, m15, [cq+32* 0]
    m[1] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 15)); // pmulld m1, m15, [cq+32*15]
    m[2] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 4)); // pmulld m2, m15, [cq+32* 4]
    m[3] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 11)); // pmulld m3, m15, [cq+32*11]
    m[4] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 8)); // pmulld m4, m15, [cq+32* 8]
    m[5] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 7)); // pmulld m5, m15, [cq+32* 7]
    m[6] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 12)); // pmulld m6, m15, [cq+32*12]
    m[7] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 3)); // pmulld m7, m15, [cq+32* 3]
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[12]); // REPX {paddd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_16x8_internal_10bpc_main_part2(m, r6)
}

/// `.main_part2` of `iadst_16x8_internal_10bpc`: in0, in15, in4, in11, in8, in7, in12, in3 in `m0`-`m7` (the other
/// half through `.main_part1` in `[r6-32*4]` … `[r6+32*3]`); expects `m12` = `pd_2048`, `m13`, `m14` = clip min,
/// max. Leaves the outputs (some negated, some unshifted, as commented) in `m0`-`m12` and `[r6-32*4]` …
/// `[r6-32*1]`.
#[inline(always)]
pub(crate) unsafe fn iadst_16x8_internal_10bpc_main_part2(m: &mut M, r6: *mut __m256i) {
    itx_mulsub_2d(m, 1, 0, 8, 9, 10, Some(12), 201, 4091, 0); // ITX_MULSUB_2D 1, 0, 8, 9, 10, 12, 201, 4091
    itx_mulsub_2d(m, 3, 2, 8, 9, 10, Some(12), 1751, 3703, 0); // ITX_MULSUB_2D 3, 2, 8, 9, 10, 12, 1751, 3703
    itx_mulsub_2d(m, 5, 4, 8, 9, 10, Some(12), 3035, 2751, 0); // ITX_MULSUB_2D 5, 4, 8, 9, 10, 12, 3035, 2751
    itx_mulsub_2d(m, 7, 6, 8, 9, 10, Some(12), 3857, 1380, 0); // ITX_MULSUB_2D 7, 6, 8, 9, 10, 12, 3857, 1380
    m[8] = _mm256_sub_epi32(m[0], m[4]); // psubd m8, m0, m4 ; t8a
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; t0a
    m[4] = _mm256_sub_epi32(m[1], m[5]); // psubd m4, m1, m5 ; t9a
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; t1a
    m[5] = _mm256_sub_epi32(m[2], m[6]); // psubd m5, m2, m6 ; t12a
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6 ; t4a
    m[6] = _mm256_sub_epi32(m[3], m[7]); // psubd m6, m3, m7 ; t13a
    m[7] = _mm256_add_epi32(m[7], m[3]); // paddd m7, m3 ; t5a
    for x in [8, 4, 5, 6, 0, 1, 2, 7] {
        m[x] = _mm256_max_epi32(m[x], m[13]); // REPX {pmaxsd x, m13}, m8, m4, m5, m6, m0, m1, m2, m7
    }
    for x in [8, 4, 5, 6, 0, 1, 2, 7] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m8, m4, m5, m6, m0, m1, m2, m7
    }
    m[11] = pd(4017); // vpbroadcastd m11, [pd_4017]
    m[10] = pd(799); // vpbroadcastd m10, [pd_799]
    itx_mulsub_2d(m, 8, 4, 3, 9, 0, Some(12), 10, 11, 0); // ITX_MULSUB_2D 8, 4, 3, 9, _, 12, 10, 11
    itx_mulsub_2d(m, 6, 5, 3, 9, 0, Some(12), 11, 10, 0); // ITX_MULSUB_2D 6, 5, 3, 9, _, 12, 11, 10
    m[3] = _mm256_sub_epi32(m[0], m[2]); // psubd m3, m0, m2 ; t4
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2 ; t0
    m[2] = _mm256_sub_epi32(m[1], m[7]); // psubd m2, m1, m7 ; t5
    m[1] = _mm256_add_epi32(m[1], m[7]); // paddd m1, m7 ; t1
    m[7] = _mm256_sub_epi32(m[4], m[6]); // psubd m7, m4, m6 ; t12a
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6 ; t8a
    m[6] = _mm256_sub_epi32(m[8], m[5]); // psubd m6, m8, m5 ; t13a
    m[5] = _mm256_add_epi32(m[5], m[8]); // paddd m5, m8 ; t9a
    for x in [3, 2, 7, 6, 0, 1, 4, 5] {
        m[x] = _mm256_max_epi32(m[x], m[13]); // REPX {pmaxsd x, m13}, m3, m2, m7, m6, m0, m1, m4, m5
    }
    for x in [3, 2, 7, 6, 0, 1, 4, 5] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m3, m2, m7, m6, m0, m1, m4, m5
    }
    m[11] = pd(3784); // vpbroadcastd m11, [pd_3784]
    m[10] = pd(1567); // vpbroadcastd m10, [pd_1567]
    itx_mulsub_2d(m, 3, 2, 8, 9, 0, Some(12), 10, 11, 0); // ITX_MULSUB_2D 3, 2, 8, 9, _, 12, 10, 11
    itx_mulsub_2d(m, 7, 6, 8, 9, 0, Some(12), 10, 11, 0); // ITX_MULSUB_2D 7, 6, 8, 9, _, 12, 10, 11
    m[10] = _mm256_min_epi32(m[14], ld_p4(r6, -32 * 4)); // pminsd m10, m14, [r6-32*4] ; t2
    m[8] = _mm256_min_epi32(m[14], ld_p4(r6, -32 * 3)); // pminsd m8, m14, [r6-32*3] ; t3
    m[9] = _mm256_sub_epi32(m[0], m[10]); // psubd m9, m0, m10 ; t2a
    m[0] = _mm256_add_epi32(m[0], m[10]); // paddd m0, m10 ; out0
    m[10] = _mm256_sub_epi32(m[1], m[8]); // psubd m10, m1, m8 ; t3a
    m[1] = _mm256_add_epi32(m[1], m[8]); // paddd m1, m8 ; -out15
    m[9] = _mm256_max_epi32(m[9], m[13]); // pmaxsd m9, m13
    m[10] = _mm256_max_epi32(m[10], m[13]); // pmaxsd m10, m13
    m[9] = _mm256_min_epi32(m[9], m[14]); // pminsd m9, m14
    m[10] = _mm256_min_epi32(m[10], m[14]); // pminsd m10, m14
    st_p4(r6, -32 * 4, m[1]); // mova [r6-32*4], m1
    m[11] = ld_p4(r6, -32 * 1); // mova m11, [r6-32*1] ; t7a
    m[1] = ld_p4(r6, -32 * 2); // mova m1, [r6-32*2] ; t6a
    m[8] = _mm256_sub_epi32(m[3], m[11]); // psubd m8, m3, m11 ; t7
    m[11] = _mm256_add_epi32(m[11], m[3]); // paddd m11, m3 ; out12
    m[3] = _mm256_add_epi32(m[2], m[1]); // paddd m3, m2, m1 ; -out3
    m[2] = _mm256_sub_epi32(m[2], m[1]); // psubd m2, m1 ; t6
    m[8] = _mm256_max_epi32(m[8], m[13]); // pmaxsd m8, m13
    m[2] = _mm256_max_epi32(m[2], m[13]); // pmaxsd m2, m13
    m[8] = _mm256_min_epi32(m[8], m[14]); // pminsd m8, m14
    m[2] = _mm256_min_epi32(m[2], m[14]); // pminsd m2, m14
    st_p4(r6, -32 * 1, m[11]); // mova [r6-32*1], m11
    st_p4(r6, -32 * 3, m[2]); // mova [r6-32*3], m2
    m[1] = ld_p4(r6, 32 * 3); // mova m1, [r6+32*3] ; t15
    m[2] = ld_p4(r6, 32 * 2); // mova m2, [r6+32*2] ; t14
    m[12] = _mm256_add_epi32(m[7], m[1]); // paddd m12, m7, m1 ; -out13
    m[7] = _mm256_sub_epi32(m[7], m[1]); // psubd m7, m1 ; t15a
    m[11] = _mm256_sub_epi32(m[6], m[2]); // psubd m11, m6, m2 ; t14a
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6 ; out2
    m[7] = _mm256_max_epi32(m[7], m[13]); // pmaxsd m7, m13
    m[11] = _mm256_max_epi32(m[11], m[13]); // pmaxsd m11, m13
    m[7] = _mm256_min_epi32(m[7], m[14]); // pminsd m7, m14
    m[11] = _mm256_min_epi32(m[11], m[14]); // pminsd m11, m14
    st_p4(r6, -32 * 2, m[12]); // mova [r6-32*2], m12
    m[1] = _mm256_min_epi32(m[14], ld_p4(r6, 32 * 0)); // pminsd m1, m14, [r6+32*0] ; t10a
    m[12] = _mm256_min_epi32(m[14], ld_p4(r6, 32 * 1)); // pminsd m12, m14, [r6+32*1] ; t11a
    m[6] = _mm256_sub_epi32(m[4], m[1]); // psubd m6, m4, m1 ; t10
    m[1] = _mm256_add_epi32(m[1], m[4]); // paddd m1, m4 ; -out1
    m[4] = _mm256_sub_epi32(m[5], m[12]); // psubd m4, m5, m12 ; t11
    m[5] = _mm256_add_epi32(m[5], m[12]); // paddd m5, m12 ; out14
    m[12] = pd(1448); // vpbroadcastd m12, [pd_1448]
    m[6] = _mm256_max_epi32(m[6], m[13]); // pmaxsd m6, m13
    m[4] = _mm256_max_epi32(m[4], m[13]); // pmaxsd m4, m13
    m[6] = _mm256_min_epi32(m[6], m[14]); // pminsd m6, m14
    m[4] = _mm256_min_epi32(m[4], m[14]); // pminsd m4, m14
    for x in [9, 10, 8, 7, 11, 6, 4] {
        m[x] = _mm256_mullo_epi32(m[x], m[12]); // REPX {pmulld x, m12}, m9, m10, m8, m7, m11, m6, m4
    }
    m[12] = _mm256_mullo_epi32(m[12], ld_p4(r6, -32 * 3)); // pmulld m12, [r6-32*3] ; t6
    st_p4(r6, -32 * 3, m[5]); // mova [r6-32*3], m5
    m[5] = _mm256_add_epi32(m[11], m[7]); // paddd m5, m11, m7 ; -out5 (unshifted)
    m[11] = _mm256_sub_epi32(m[11], m[7]); // psubd m11, m7 ; out10 (unshifted)
    m[7] = _mm256_add_epi32(m[9], m[10]); // paddd m7, m9, m10 ; -out7 (unshifted)
    m[9] = _mm256_sub_epi32(m[9], m[10]); // psubd m9, m10 ; out8 (unshifted)
    m[10] = _mm256_sub_epi32(m[6], m[4]); // psubd m10, m6, m4 ; -out9 (unshifted)
    m[6] = _mm256_add_epi32(m[6], m[4]); // paddd m6, m4 ; out6 (unshifted)
    m[4] = _mm256_add_epi32(m[12], m[8]); // paddd m4, m12, m8 ; out4 (unshifted)
    m[12] = _mm256_sub_epi32(m[12], m[8]); // psubd m12, m8 ; -out11 (unshifted)
}

/// `.main_part1` of `iadst_16x8_internal_10bpc`: in2, in13, in6, in9, in10, in5, in14, in1 in `m0`-`m7`; expects
/// `m12` = `pd_2048`, `m13`, `m14` = clip min, max; writes `[r6-32*4]` … `[r6+32*3]`.
#[inline(always)]
pub(crate) unsafe fn iadst_16x8_internal_10bpc_main_part1(m: &mut M, r6: *mut __m256i) {
    itx_mulsub_2d(m, 1, 0, 8, 9, 10, Some(12), 995, 3973, 0); // ITX_MULSUB_2D 1, 0, 8, 9, 10, 12, 995, 3973
    itx_mulsub_2d(m, 3, 2, 8, 9, 10, Some(12), 2440, 3290, 0); // ITX_MULSUB_2D 3, 2, 8, 9, 10, 12, 2440, 3290
    itx_mulsub_2d(m, 5, 4, 8, 9, 10, Some(12), 3513, 2106, 0); // ITX_MULSUB_2D 5, 4, 8, 9, 10, 12, 3513, 2106
    itx_mulsub_2d(m, 7, 6, 8, 9, 10, Some(12), 4052, 601, 0); // ITX_MULSUB_2D 7, 6, 8, 9, 10, 12, 4052, 601
    m[8] = _mm256_sub_epi32(m[0], m[4]); // psubd m8, m0, m4 ; t10a
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; t2a
    m[4] = _mm256_sub_epi32(m[1], m[5]); // psubd m4, m1, m5 ; t11a
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; t3a
    m[5] = _mm256_sub_epi32(m[2], m[6]); // psubd m5, m2, m6 ; t14a
    m[2] = _mm256_add_epi32(m[2], m[6]); // paddd m2, m6 ; t6a
    m[6] = _mm256_sub_epi32(m[3], m[7]); // psubd m6, m3, m7 ; t15a
    m[7] = _mm256_add_epi32(m[7], m[3]); // paddd m7, m3 ; t7a
    for x in [8, 4, 5, 6, 0, 1, 2, 7] {
        m[x] = _mm256_max_epi32(m[x], m[13]); // REPX {pmaxsd x, m13}, m8, m4, m5, m6, m0, m1, m2, m7
    }
    for x in [8, 4, 5, 6, 0, 1, 2, 7] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m8, m4, m5, m6, m0, m1, m2, m7
    }
    m[11] = pd(2276); // vpbroadcastd m11, [pd_2276]
    m[10] = pd(3406); // vpbroadcastd m10, [pd_3406]
    itx_mulsub_2d(m, 8, 4, 3, 9, 0, Some(12), 10, 11, 0); // ITX_MULSUB_2D 8, 4, 3, 9, _, 12, 10, 11
    itx_mulsub_2d(m, 6, 5, 3, 9, 0, Some(12), 11, 10, 0); // ITX_MULSUB_2D 6, 5, 3, 9, _, 12, 11, 10
    m[3] = _mm256_sub_epi32(m[0], m[2]); // psubd m3, m0, m2 ; t6
    m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2 ; t2
    m[2] = _mm256_sub_epi32(m[1], m[7]); // psubd m2, m1, m7 ; t7
    m[1] = _mm256_add_epi32(m[1], m[7]); // paddd m1, m7 ; t3
    m[7] = _mm256_sub_epi32(m[4], m[6]); // psubd m7, m4, m6 ; t14a
    m[4] = _mm256_add_epi32(m[4], m[6]); // paddd m4, m6 ; t10a
    m[6] = _mm256_sub_epi32(m[8], m[5]); // psubd m6, m8, m5 ; t15a
    m[5] = _mm256_add_epi32(m[5], m[8]); // paddd m5, m8 ; t11a
    for x in [3, 2, 7, 6, 0, 1, 4, 5] {
        m[x] = _mm256_max_epi32(m[x], m[13]); // REPX {pmaxsd x, m13}, m3, m2, m7, m6, m0, m1, m4, m5
    }
    for x in [3, 2, 7, 6] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m3, m2, m7, m6 ; clip the rest later
    }
    m[11] = pd(1567); // vpbroadcastd m11, [pd_1567]
    m[10] = pd(3784); // vpbroadcastd m10, [pd_3784]
    itx_mulsub_2d(m, 2, 3, 8, 9, 0, Some(12), 10, 11, 0); // ITX_MULSUB_2D 2, 3, 8, 9, _, 12, 10, 11
    itx_mulsub_2d(m, 6, 7, 8, 9, 0, Some(12), 10, 11, 0); // ITX_MULSUB_2D 6, 7, 8, 9, _, 12, 10, 11
    st_p4(r6, -32 * 4, m[0]); // mova [r6-32*4], m0
    st_p4(r6, -32 * 3, m[1]); // mova [r6-32*3], m1
    st_p4(r6, 32 * 0, m[4]); // mova [r6+32*0], m4
    st_p4(r6, 32 * 1, m[5]); // mova [r6+32*1], m5
    st_p4(r6, -32 * 2, m[2]); // mova [r6-32*2], m2
    st_p4(r6, -32 * 1, m[3]); // mova [r6-32*1], m3
    st_p4(r6, 32 * 2, m[6]); // mova [r6+32*2], m6
    st_p4(r6, 32 * 3, m[7]); // mova [r6+32*3], m7
}

/// `.pass1` of `iflipadst_16x8_internal_10bpc` (up to `jmp tx2q`; expects `m13`, `m14` = clip min, max; `rsp` =
/// the frame)
#[inline(always)]
unsafe fn iflipadst_16x8_internal_10bpc_pass1(m: &mut M, c: Coef, rsp: *mut __m256i) {
    let r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    iadst_16x8_internal_10bpc_main(m, c, r6); // call m(iadst_16x8_internal_10bpc).main
    m[14] = pd(3072); // vpbroadcastd m14, [pd_3072]
    m[15] = _mm256_srli_epi32::<11>(m[15]); // psrld m15, 11
    m[13] = _mm256_sub_epi32(m[14], m[15]); // psubd m13, m14, m15
    iflipadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call .pass1_rotations
    iadst_16x8_internal_10bpc_pass1_end(m) // jmp m(iadst_16x8_internal_10bpc).pass1_end
}

/// `.pass2` of `iflipadst_16x8_internal_10bpc`
#[inline(always)]
unsafe fn iflipadst_16x8_internal_10bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    idct_16x8_internal_10bpc_transpose(m); // call m(idct_16x8_internal_10bpc).transpose
    iadst_16x8_internal_8bpc_main(m); // call m(iadst_16x8_internal_8bpc).main
    iadst_16x8_internal_8bpc_main_pass2_end(m); // call m(iadst_16x8_internal_8bpc).main_pass2_end
    m[10] = pw(2048); // vpbroadcastd m10, [pw_2048]
    m[11] = _mm256_setzero_si256(); // pxor m11, m11
    m[11] = _mm256_sub_epi16(m[11], m[10]); // psubw m11, m10
    m[12] = m[0]; // mova m12, m0
    m[0] = _mm256_mulhrs_epi16(m[7], m[11]); // pmulhrsw m0, m7, m11
    m[7] = m[1]; // mova m7, m1
    m[1] = _mm256_mulhrs_epi16(m[6], m[10]); // pmulhrsw m1, m6, m10
    m[6] = m[2]; // mova m6, m2
    m[2] = _mm256_mulhrs_epi16(m[5], m[11]); // pmulhrsw m2, m5, m11
    m[5] = m[3]; // mova m5, m3
    m[3] = _mm256_mulhrs_epi16(m[4], m[10]); // pmulhrsw m3, m4, m10
    let mut r3 = 0;
    idct_16x8_internal_10bpc_write_16x4_start(m, &mut dst, stride, &mut c, &mut r3); // call m(idct_16x8_internal_10bpc).write_16x4_start
    m[0] = _mm256_mulhrs_epi16(m[5], m[11]); // pmulhrsw m0, m5, m11
    m[1] = _mm256_mulhrs_epi16(m[6], m[10]); // pmulhrsw m1, m6, m10
    m[2] = _mm256_mulhrs_epi16(m[7], m[11]); // pmulhrsw m2, m7, m11
    m[3] = _mm256_mulhrs_epi16(m[12], m[10]); // pmulhrsw m3, m12, m10
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
}

/// `.pass1_rotations` of `iflipadst_16x8_internal_10bpc`: as `iadst_16x8_internal_10bpc_pass1_rotations`, with the
/// outputs in reverse order (reads `[r6-32*4]` … `[r6-32*1]`)
#[inline(always)]
pub(crate) unsafe fn iflipadst_16x8_internal_10bpc_pass1_rotations(m: &mut M, r6: *mut __m256i) {
    m[8] = _mm256_sub_epi32(m[13], m[7]); // psubd m8, m13, m7
    m[7] = _mm256_add_epi32(m[14], m[9]); // paddd m7, m14, m9
    m[9] = _mm256_add_epi32(m[14], m[6]); // paddd m9, m14, m6
    m[6] = _mm256_sub_epi32(m[13], m[10]); // psubd m6, m13, m10
    m[10] = _mm256_sub_epi32(m[13], m[5]); // psubd m10, m13, m5
    m[5] = _mm256_add_epi32(m[14], m[11]); // paddd m5, m14, m11
    m[11] = _mm256_add_epi32(m[14], m[4]); // paddd m11, m14, m4
    m[4] = _mm256_sub_epi32(m[13], m[12]); // psubd m4, m13, m12
    m[12] = _mm256_sub_epi32(m[15], m[3]); // psubd m12, m15, m3
    m[3] = _mm256_add_epi32(m[15], ld_p4(r6, -32 * 1)); // paddd m3, m15, [r6-32*1]
    m[13] = _mm256_add_epi32(m[15], m[2]); // paddd m13, m15, m2
    m[2] = _mm256_sub_epi32(m[15], ld_p4(r6, -32 * 2)); // psubd m2, m15, [r6-32*2]
    m[14] = _mm256_sub_epi32(m[15], m[1]); // psubd m14, m15, m1
    m[1] = m[15]; // mova m1, m15
    m[15] = _mm256_add_epi32(m[15], m[0]); // paddd m15, m0
    m[0] = _mm256_sub_epi32(m[1], ld_p4(r6, -32 * 4)); // psubd m0, m1, [r6-32*4]
    m[1] = _mm256_add_epi32(m[1], ld_p4(r6, -32 * 3)); // paddd m1, [r6-32*3]
}

/// `.pass1` of `iidentity_16x8_internal_10bpc` (the whole first pass of both bit depths; `rsp` = the frame, whose
/// slot 0 is the spill slot `[rsp]`)
#[inline(always)]
unsafe fn iidentity_16x8_internal_10bpc_pass1(m: &mut M, c: Coef, rsp: *mut __m256i) {
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    for x in 0..15 {
        m[x] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * x as isize)); // pmulld mx, m15, [cq+32*x]
    }
    m[15] = _mm256_mullo_epi32(m[15], ld_p4(c, 32 * 15)); // pmulld m15, [cq+32*15]
    st_p4(rsp, 0, m[7]); // mova [rsp], m7
    m[7] = pd(2048); // vpbroadcastd m7, [pd_2048]
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7 }, m0, …, m6, m8, …, m15
    }
    m[7] = _mm256_add_epi32(m[7], ld_p4(rsp, 0)); // paddd m7, [rsp]
    for x in 0..16 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, …, m15
    }
    st_p4(rsp, 0, m[15]); // mova [rsp], m15
    m[15] = pd(5793); // vpbroadcastd m15, [pd_5793]
    for x in 0..15 {
        m[x] = _mm256_mullo_epi32(m[x], m[15]); // REPX {pmulld x, m15}, m0, …, m14
    }
    m[15] = _mm256_mullo_epi32(m[15], ld_p4(rsp, 0)); // pmulld m15, [rsp]
    st_p4(rsp, 0, m[7]); // mova [rsp], m7
    m[7] = pd(3072); // vpbroadcastd m7, [pd_3072]
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7 }, m0, …, m6, m8, …, m15
    }
    m[7] = _mm256_add_epi32(m[7], ld_p4(rsp, 0)); // paddd m7, [rsp]
    for x in 0..16 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, …, m15
    }
}

/// `.pass2` of `iidentity_16x8_internal_10bpc`
#[inline(always)]
unsafe fn iidentity_16x8_internal_10bpc_pass2(m: &mut M, dst: Dst, stride: isize, c: Coef) {
    idct_16x8_internal_10bpc_transpose(m); // call m(idct_16x8_internal_10bpc).transpose
    m[10] = pw(4096); // vpbroadcastd m10, [pw_4096]
    idct_16x8_internal_10bpc_end(m, dst, stride, c) // jmp m(idct_16x8_internal_10bpc).end
}

/// `.pass2` of `idct_16x8_internal_12bpc`
#[inline(always)]
unsafe fn idct_16x8_internal_12bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    idct_16x8_internal_12bpc_pass2_main(m, &mut dst, stride, &mut c); // call .pass2_main
}

/// `.pass2_main` of `idct_16x8_internal_12bpc` (falls into `.end`): the second pass of 8 rows of 16 dwords in
/// `m0`-`m15` (rows 0-7 of the left half in `m0`-`m7`, of the right half in `m8`-`m15`), written to eight rows at
/// `dstq` with the 12 bpc clamp; uses `[cq+32*0..15]` as scratch and clears it; advances `dstq` by eight rows and
/// `cq` by `32*16`.
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_12bpc_pass2_main(m: &mut M, dst: &mut Dst, stride: isize, c: &mut Coef) {
    idct_8x16_internal_12bpc_transpose(m, *c); // call m(idct_8x16_internal_12bpc).transpose
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    for x in 0..8 {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x8_internal_12bpc_round_shift4(m); // call m(idct_8x8_internal_12bpc).round_shift4
    for x in 0..8 {
        st_p4(*c, 32 * (8 + x as isize), m[x]); // mova [cq+32*(8+x)], mx
    }
    for x in 0..8 {
        m[x] = _mm256_max_epi32(m[12], ld_p4(*c, 32 * x as isize)); // pmaxsd mx, m12, [cq+32*x]
    }
    for x in 0..8 {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x8_internal_12bpc_round_shift4(m); // call m(idct_8x8_internal_12bpc).round_shift4
    idct_16x8_internal_12bpc_end(m, dst, stride, c)
}

/// `.end` of `idct_16x8_internal_12bpc`: packs `m0`-`m7` with `[cq+32*8..15]` and writes eight rows (as
/// `idct_16x8_internal_12bpc_pass2_main`)
#[inline(always)]
unsafe fn idct_16x8_internal_12bpc_end(m: &mut M, dst: &mut Dst, stride: isize, c: &mut Coef) {
    for x in 0..8 {
        m[x] = _mm256_packs_epi32(m[x], ld_p4(*c, 32 * (8 + x as isize))); // packssdw mx, [cq+32*(8+x)]
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    }
    let mut r3 = 0;
    idct_16x8_internal_12bpc_write_16x4_start(m, stride, &mut r3); // call .write_16x4_start
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[4]); // vpermq m0, m4, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[5]); // vpermq m1, m5, q3120
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[6]); // vpermq m2, m6, q3120
    m[3] = _mm256_permute4x64_epi64::<0xd8>(m[7]); // vpermq m3, m7, q3120
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3) // jmp m(idct_16x8_internal_10bpc).write_16x4_zero
}

/// `.write_16x4_start` of `idct_16x8_internal_12bpc`: `m9` = 12 bpc pixel max, `r3` = `strideq*3`, `m8` = 0 (no
/// write)
#[inline(always)]
pub(crate) unsafe fn idct_16x8_internal_12bpc_write_16x4_start(m: &mut M, stride: isize, r3: &mut isize) {
    m[9] = pw(0xfff); // vpbroadcastd m9, [pixel_12bpc_max]
    *r3 = stride * 3; // lea r3, [strideq*3]
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
}

/// `.pass2` of `iadst_16x8_internal_12bpc`
#[inline(always)]
unsafe fn iadst_16x8_internal_12bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    iadst_16x8_internal_12bpc_pass2_main(m, c); // call .pass2_main
    idct_16x8_internal_12bpc_end(m, &mut dst, stride, &mut c); // call m(idct_16x8_internal_12bpc).end
}

/// `.pass2_main` of `iadst_16x8_internal_12bpc` (uses `[cq+32*0..15]` as scratch; leaves the right half in
/// `[cq+32*8..15]`)
#[inline(always)]
unsafe fn iadst_16x8_internal_12bpc_pass2_main(m: &mut M, c: Coef) {
    idct_8x16_internal_12bpc_transpose(m, c); // call m(idct_8x16_internal_12bpc).transpose
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    for x in 0..8 {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_8x8_internal_12bpc_pass2_main2(m); // call m(iadst_8x8_internal_12bpc).pass2_main2
    for x in 0..8 {
        st_p4(c, 32 * (8 + x as isize), m[x]); // mova [cq+32*(8+x)], mx
    }
    for x in 0..8 {
        m[x] = _mm256_max_epi32(m[12], ld_p4(c, 32 * x as isize)); // pmaxsd mx, m12, [cq+32*x]
    }
    for x in 0..8 {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    iadst_8x8_internal_12bpc_pass2_main2(m); // call m(iadst_8x8_internal_12bpc).pass2_main2
}

/// `.pass2` of `iflipadst_16x8_internal_12bpc`
#[inline(always)]
unsafe fn iflipadst_16x8_internal_12bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    iadst_16x8_internal_12bpc_pass2_main(m, c); // call m(iadst_16x8_internal_12bpc).pass2_main
    m[13] = _mm256_packs_epi32(m[0], ld_p4(c, 32 * 8)); // packssdw m13, m0, [cq+32* 8]
    m[12] = _mm256_packs_epi32(m[1], ld_p4(c, 32 * 9)); // packssdw m12, m1, [cq+32* 9]
    m[11] = _mm256_packs_epi32(m[2], ld_p4(c, 32 * 10)); // packssdw m11, m2, [cq+32*10]
    m[10] = _mm256_packs_epi32(m[3], ld_p4(c, 32 * 11)); // packssdw m10, m3, [cq+32*11]
    m[3] = _mm256_packs_epi32(m[4], ld_p4(c, 32 * 12)); // packssdw m3, m4, [cq+32*12]
    m[2] = _mm256_packs_epi32(m[5], ld_p4(c, 32 * 13)); // packssdw m2, m5, [cq+32*13]
    m[1] = _mm256_packs_epi32(m[6], ld_p4(c, 32 * 14)); // packssdw m1, m6, [cq+32*14]
    m[0] = _mm256_packs_epi32(m[7], ld_p4(c, 32 * 15)); // packssdw m0, m7, [cq+32*15]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_permute4x64_epi64::<0xd8>(m[x]); // REPX {vpermq x, x, q3120}, m0, m1, m2, m3
    }
    let mut r3 = 0;
    idct_16x8_internal_12bpc_write_16x4_start(m, stride, &mut r3); // call m(idct_16x8_internal_12bpc).write_16x4_start
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[10]); // vpermq m0, m10, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[11]); // vpermq m1, m11, q3120
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[12]); // vpermq m2, m12, q3120
    m[3] = _mm256_permute4x64_epi64::<0xd8>(m[13]); // vpermq m3, m13, q3120
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
}

/// `.pass2` of `iidentity_16x8_internal_12bpc`
#[inline(always)]
unsafe fn iidentity_16x8_internal_12bpc_pass2(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef) {
    idct_16x8_internal_10bpc_transpose2(m); // call m(idct_16x8_internal_10bpc).transpose2
    m[10] = pw(4096); // vpbroadcastd m10, [pw_4096]
    m[0] = _mm256_mulhrs_epi16(m[0], m[10]); // pmulhrsw m0, m10
    m[1] = _mm256_mulhrs_epi16(m[1], m[10]); // pmulhrsw m1, m10
    m[2] = _mm256_mulhrs_epi16(m[2], m[10]); // pmulhrsw m2, m10
    m[3] = _mm256_mulhrs_epi16(m[3], m[10]); // pmulhrsw m3, m10
    let mut r3 = 0;
    idct_16x8_internal_12bpc_write_16x4_start(m, stride, &mut r3); // call m(idct_16x8_internal_12bpc).write_16x4_start
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    idct_16x8_internal_10bpc_end2(m, dst, stride, c, r3) // jmp m(idct_16x8_internal_10bpc).end2
}

// --- entry points -------------------------------------------------------------------------------------------------

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x4_10bpc_avx2,
    inv_txfm_add_16x4_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x4_12bpc_avx2,
    inv_txfm_add_16x4_12bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x8_10bpc_avx2,
    inv_txfm_add_16x8_10bpc,
    K::Identity,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_identity_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Adst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_identity_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::FlipAdst,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_adst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Identity,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_flipadst_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Identity,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x8_12bpc_avx2,
    inv_txfm_add_16x8_12bpc,
    K::Identity,
    K::Identity
);

// --- part 5: itx16_avx2.asm lines 4604-5617 (16x16) ---

// The frame of the 16x16 `_internal` functions (`cglobal …, 32*24`) is 24 32-byte slots, reached through raw
// pointers with the assembly's byte offsets: `rsp` is its first slot, `r6` (and `r5` at 12 bpc) point into it as
// in the assembly. `[rsp+gprsize+32*n]` inside a `call`ed subroutine is the caller's `[rsp+32*n]`: the same slot.
// The 8 bpc `.main`s take slots 0-2 as their `&mut [__m256i; 3]`.

/// `mova m, [p+off]` (32 bytes; caller memory or the frame)
#[inline(always)]
unsafe fn ld5<T>(p: *mut T, off: isize) -> __m256i {
    _mm256_loadu_si256(at(p, off).cast())
}

/// `mova [p+off], m` (32 bytes; caller memory or the frame)
#[inline(always)]
unsafe fn st5<T>(p: *mut T, off: isize, v: __m256i) {
    _mm256_storeu_si256(at(p, off).cast(), v)
}

/// The slots `[rsp+32*0..2]` as the spill slots of an 8 bpc `.main` (`[rsp+gprsize+32*0..2]` there). The
/// reference lives for that call only.
#[inline(always)]
unsafe fn spill5<'a>(rsp: *mut __m256i) -> &'a mut [__m256i; 3] {
    &mut *rsp.cast::<[__m256i; 3]>()
}

/// `INV_TXFM_16X16_FN type1, type2, eob_offset, 10` (the entry code and `jmp tx2q`) with the frame of
/// `i<type1>_16x16_internal_10bpc`.
#[inline(always)]
unsafe fn inv_txfm_add_16x16_10bpc(dst: Dst, stride: isize, c: Coef, mut eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct {
        if eob == 0 {
            // test eobd, eobd / jnz m(idct_16x16_internal_10bpc)
            let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
            m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
            c.write_unaligned(eob); // mov [cq], eobd ; 0
            let r3d = eob | 16; // or r3d, 16
            r6d = r6d.wrapping_add(640); // add r6d, 640
            r6d >>= 10; // sar r6d, 10
            return inv_txfm_add_dct_dct_16x4_10bpc_dconly3(m, dst, stride, r6d, r3d);
            // jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly3
        }
    } else if t1 == K::Dct && t2 == K::Identity {
        eob += 28; // add eobd, 28
    } else if t1 == K::Identity && t2 == K::Dct {
        eob += -92; // add eobd, -92
    }
    // Not in the original: the frame is zeroed whole (which slots the second pass reads depends on the eob path
    // of the first).
    let mut frame = [_mm256_setzero_si256(); 24];
    let rsp = frame.as_mut_ptr();
    let mut r6 = rsp;
    match t1 {
        K::Dct => {
            m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
            m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
            idct_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6);
        }
        K::Adst => {
            m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
            m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
            iadst_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6);
        }
        K::FlipAdst => {
            m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
            m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
            iflipadst_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6);
        }
        K::Identity => iidentity_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6),
    }
    match t2 {
        K::Dct => idct_16x16_internal_10bpc_pass2(m, dst, stride, c, eob, rsp, r6),
        K::Adst => iadst_16x16_internal_10bpc_pass2(m, dst, stride, c, eob, rsp, r6),
        K::FlipAdst => iflipadst_16x16_internal_10bpc_pass2(m, dst, stride, c, eob, rsp, r6),
        K::Identity => iidentity_16x16_internal_10bpc_pass2(m, dst, stride, c, eob, rsp, r6),
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x16_10bpc_avx2,
    inv_txfm_add_16x16_10bpc,
    K::Identity,
    K::Identity
);

/// `.pass1` of `idct_16x16_internal_10bpc` up to `jmp tx2q` (the clipping bounds in `m12`, `m13` on entry; the
/// code before the label, which sets them, is in the size functions): `eob` is `eobd` (36 is subtracted here),
/// `r6` is left where the second pass expects it.
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_pass1(
    m: &mut M,
    c: Coef,
    eob: &mut c_int,
    rsp: *mut __m256i,
    r6: &mut *mut __m256i,
) {
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    *r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    *eob = eob.wrapping_sub(36); // sub eobd, 36
    if *eob >= 0 {
        // jl .fast
        idct_16x16_internal_10bpc_main(m, at(c, 32), *r6); // add cq, 32 / call .main / sub cq, 32
        m[10] = ld5(*r6, -32 * 4); // mova m10, [r6-32*4]
        m[9] = ld5(*r6, -32 * 3); // mova m9, [r6-32*3]
        m[8] = ld5(*r6, -32 * 2); // mova m8, [r6-32*2]
        m[15] = _mm256_sub_epi32(m[0], m[10]); // psubd m15, m0, m10 ; out15
        m[0] = _mm256_add_epi32(m[0], m[10]); // paddd m0, m10 ; out0
        m[10] = _mm256_sub_epi32(m[1], m[9]); // psubd m10, m1, m9 ; out14
        m[1] = _mm256_add_epi32(m[1], m[9]); // paddd m1, m9 ; out1
        m[9] = _mm256_sub_epi32(m[2], m[8]); // psubd m9, m2, m8 ; out13
        m[2] = _mm256_add_epi32(m[2], m[8]); // paddd m2, m8 ; out2
        for x in [0, 1, 2] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m0, m1, m2
        }
        st5(*r6, -32 * 4, m[0]); // mova [r6-32*4], m0
        st5(*r6, -32 * 3, m[1]); // mova [r6-32*3], m1
        st5(*r6, -32 * 2, m[2]); // mova [r6-32*2], m2
        m[2] = ld5(*r6, -32); // mova m2, [r6-32*1]
        m[1] = ld5(*r6, 0); // mova m1, [r6+32*0]
        m[0] = ld5(*r6, 32); // mova m0, [r6+32*1]
        for x in [9, 10, 15] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m9, m10, m15
        }
        m[8] = _mm256_sub_epi32(m[3], m[2]); // psubd m8, m3, m2 ; out12
        m[3] = _mm256_add_epi32(m[3], m[2]); // paddd m3, m2 ; out3
        m[2] = _mm256_sub_epi32(m[4], m[1]); // psubd m2, m4, m1 ; out11
        m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1 ; out4
        m[1] = _mm256_sub_epi32(m[5], m[0]); // psubd m1, m5, m0 ; out10
        m[5] = _mm256_add_epi32(m[5], m[0]); // paddd m5, m0 ; out5
        for x in [3, 4, 5] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m3, m4, m5
        }
        st5(*r6, -32, m[3]); // mova [r6-32*1], m3
        st5(*r6, 0, m[4]); // mova [r6+32*0], m4
        st5(*r6, 32, m[5]); // mova [r6+32*1], m5
        m[4] = ld5(*r6, 32 * 2); // mova m4, [r6+32*2]
        m[3] = ld5(*r6, 32 * 3); // mova m3, [r6+32*3]
        for x in [1, 2, 8] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m1, m2, m8
        }
        m[5] = _mm256_sub_epi32(m[6], m[4]); // psubd m5, m6, m4 ; out9
        m[6] = _mm256_add_epi32(m[6], m[4]); // paddd m6, m4 ; out6
        m[4] = _mm256_sub_epi32(m[7], m[3]); // psubd m4, m7, m3 ; out8
        m[7] = _mm256_add_epi32(m[7], m[3]); // paddd m7, m3 ; out7
        for x in [6, 7, 4, 5] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m6, m7, m4, m5
        }
        st5(*r6, 32 * 2, m[6]); // mova [r6+32*2], m6
        st5(*r6, 32 * 3, m[7]); // mova [r6+32*3], m7
        *r6 = at(*r6, 32 * 8); // add r6, 32*8
        st5(*r6, -32 * 4, m[4]); // mova [r6-32*4], m4
        st5(*r6, -32 * 3, m[5]); // mova [r6-32*3], m5
        st5(*r6, -32 * 2, m[1]); // mova [r6-32*2], m1
        st5(*r6, -32, m[2]); // mova [r6-32*1], m2
        st5(*r6, 0, m[8]); // mova [r6+32*0], m8
        st5(*r6, 32, m[9]); // mova [r6+32*1], m9
        st5(*r6, 32 * 2, m[10]); // mova [r6+32*2], m10
        st5(*r6, 32 * 3, m[15]); // mova [r6+32*3], m15
    }
    // .fast:
    *r6 = at(*r6, 32 * 8); // add r6, 32*8
    idct_16x16_internal_10bpc_main(m, c, *r6); // call .main
    m[14] = ld5(*r6, -32 * 4); // mova m14, [r6-32*4]
    m[13] = ld5(*r6, -32 * 3); // mova m13, [r6-32*3]
    m[12] = ld5(*r6, -32 * 2); // mova m12, [r6-32*2]
    m[11] = ld5(*r6, -32); // mova m11, [r6-32*1]
    m[10] = ld5(*r6, 0); // mova m10, [r6+32*0]
    m[9] = ld5(*r6, 32); // mova m9, [r6+32*1]
    m[8] = ld5(*r6, 32 * 2); // mova m8, [r6+32*2]
    m[15] = _mm256_sub_epi32(m[0], m[14]); // psubd m15, m0, m14 ; out15
    m[0] = _mm256_add_epi32(m[0], m[14]); // paddd m0, m14 ; out0
    m[14] = _mm256_sub_epi32(m[1], m[13]); // psubd m14, m1, m13 ; out14
    m[1] = _mm256_add_epi32(m[1], m[13]); // paddd m1, m13 ; out1
    m[13] = _mm256_sub_epi32(m[2], m[12]); // psubd m13, m2, m12 ; out13
    m[2] = _mm256_add_epi32(m[2], m[12]); // paddd m2, m12 ; out2
    m[12] = _mm256_sub_epi32(m[3], m[11]); // psubd m12, m3, m11 ; out12
    m[3] = _mm256_add_epi32(m[3], m[11]); // paddd m3, m11 ; out3
    m[11] = _mm256_sub_epi32(m[4], m[10]); // psubd m11, m4, m10 ; out11
    m[4] = _mm256_add_epi32(m[4], m[10]); // paddd m4, m10 ; out4
    m[10] = _mm256_sub_epi32(m[5], m[9]); // psubd m10, m5, m9 ; out10
    m[5] = _mm256_add_epi32(m[5], m[9]); // paddd m5, m9 ; out5
    m[9] = _mm256_sub_epi32(m[6], m[8]); // psubd m9, m6, m8 ; out9
    m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8 ; out6
    m[8] = _mm256_sub_epi32(m[7], ld5(*r6, 32 * 3)); // psubd m8, m7, [r6+32*3] ; out8
    m[7] = _mm256_add_epi32(m[7], ld5(*r6, 32 * 3)); // paddd m7, [r6+32*3] ; out7
    *r6 = at(*r6, -32 * 8); // sub r6, 32*8
    for x in 0..16 {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m0, m1, …, m15
    }
    // jmp tx2q
}

/// `.pass2` of `idct_16x16_internal_10bpc` (to its `RET`)
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_pass2(
    m: &mut M,
    dst: Dst,
    stride: isize,
    c: Coef,
    eob: c_int,
    rsp: *mut __m256i,
    mut r6: *mut __m256i,
) {
    idct_16x16_internal_10bpc_transpose(m, eob, &mut r6); // call .transpose
                                                          // lea r6, [pw_5+128] (the base of the 8 bpc constants: no code)
    st5(rsp, 0, m[15]); // mova [rsp], m15
    idct_16x16_internal_8bpc_main(m, spill5(rsp)); // call m(idct_16x16_internal_8bpc).main
    m[1] = ld5(rsp, 32); // mova m1, [rsp+32*1]
    idct_16x16_internal_10bpc_end(m, dst, stride, c, rsp)
}

/// `.end` of `idct_16x16_internal_10bpc` (to its `RET`)
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef, rsp: *mut __m256i) {
    idct_16x16_internal_10bpc_write_16x16(m, &mut dst, stride, &mut c, rsp); // call .write_16x16
                                                                             // RET
}

/// `.write_16x16` of `idct_16x16_internal_10bpc`: rounds the 16 rows of 16 words in `m0`-`m15` with `pw_2048`,
/// adds them to the block and zeroes the coefficients (`[rsp+gprsize+32*0..2]` are the caller's frame slots 0-2).
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_write_16x16(
    m: &mut M,
    dst: &mut Dst,
    stride: isize,
    c: &mut Coef,
    rsp: *mut __m256i,
) {
    let mut r3: isize = 0; // r3: set by .write_16x4_start (lea r3, [strideq*3])
    st5(rsp, 0, m[8]); // mova [rsp+gprsize+32*0], m8
    st5(rsp, 32, m[9]); // mova [rsp+gprsize+32*1], m9
    st5(rsp, 32 * 2, m[12]); // mova [rsp+gprsize+32*2], m12
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    m[2] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m2, m12
    m[3] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m3, m12
    idct_16x8_internal_10bpc_write_16x4_start(m, dst, stride, c, &mut r3); // call m(idct_16x8_internal_10bpc).write_16x4_start
    idct_16x16_internal_10bpc_write_16x16_2(m, dst, stride, c, rsp, r3)
}

/// `.write_16x16_2` of `idct_16x16_internal_10bpc` (rows 4-15; `m12` holds the rounding multiplier)
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_write_16x16_2(
    m: &mut M,
    dst: &mut Dst,
    stride: isize,
    c: &mut Coef,
    rsp: *mut __m256i,
    r3: isize,
) {
    m[0] = _mm256_mulhrs_epi16(m[12], m[4]); // pmulhrsw m0, m12, m4
    m[1] = _mm256_mulhrs_epi16(m[12], m[5]); // pmulhrsw m1, m12, m5
    m[2] = _mm256_mulhrs_epi16(m[12], m[6]); // pmulhrsw m2, m12, m6
    m[3] = _mm256_mulhrs_epi16(m[12], m[7]); // pmulhrsw m3, m12, m7
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 0)); // pmulhrsw m0, m12, [rsp+gprsize+32*0]
    m[1] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 32)); // pmulhrsw m1, m12, [rsp+gprsize+32*1]
    m[2] = _mm256_mulhrs_epi16(m[12], m[10]); // pmulhrsw m2, m12, m10
    m[3] = _mm256_mulhrs_epi16(m[12], m[11]); // pmulhrsw m3, m12, m11
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 32 * 2)); // pmulhrsw m0, m12, [rsp+gprsize+32*2]
    m[1] = _mm256_mulhrs_epi16(m[12], m[13]); // pmulhrsw m1, m12, m13
    m[2] = _mm256_mulhrs_epi16(m[12], m[14]); // pmulhrsw m2, m12, m14
    m[3] = _mm256_mulhrs_epi16(m[12], m[15]); // pmulhrsw m3, m12, m15
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3) // jmp m(idct_16x8_internal_10bpc).write_16x4_zero
}

/// `.transpose` of `idct_16x16_internal_10bpc`: packs the first pass's 32-bit rows (`m0`-`m15` and the frame
/// slots at `r6`) to words and transposes the 16x16 block into `m0`-`m15`; `eob` < 0 is the fast path (the
/// right half is zero).
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_transpose(m: &mut M, eob: c_int, r6: &mut *mut __m256i) {
    if eob < 0 {
        // test eobd, eobd / jl .transpose_fast
        return idct_16x16_internal_10bpc_transpose_fast(m);
    }
    m[8] = _mm256_packs_epi32(m[8], ld5(*r6, -32 * 4)); // packssdw m8, [r6-32*4]
    m[9] = _mm256_packs_epi32(m[9], ld5(*r6, -32 * 3)); // packssdw m9, [r6-32*3]
    m[10] = _mm256_packs_epi32(m[10], ld5(*r6, -32 * 2)); // packssdw m10, [r6-32*2]
    m[11] = _mm256_packs_epi32(m[11], ld5(*r6, -32)); // packssdw m11, [r6-32*1]
    m[12] = _mm256_packs_epi32(m[12], ld5(*r6, 0)); // packssdw m12, [r6+32*0]
    m[13] = _mm256_packs_epi32(m[13], ld5(*r6, 32)); // packssdw m13, [r6+32*1]
    m[14] = _mm256_packs_epi32(m[14], ld5(*r6, 32 * 2)); // packssdw m14, [r6+32*2]
    m[15] = _mm256_packs_epi32(m[15], ld5(*r6, 32 * 3)); // packssdw m15, [r6+32*3]
    *r6 = at(*r6, -32 * 8); // sub r6, 32*8
    m[0] = _mm256_packs_epi32(m[0], ld5(*r6, -32 * 4)); // packssdw m0, [r6-32*4]
    m[1] = _mm256_packs_epi32(m[1], ld5(*r6, -32 * 3)); // packssdw m1, [r6-32*3]
    m[2] = _mm256_packs_epi32(m[2], ld5(*r6, -32 * 2)); // packssdw m2, [r6-32*2]
    m[3] = _mm256_packs_epi32(m[3], ld5(*r6, -32)); // packssdw m3, [r6-32*1]
    m[4] = _mm256_packs_epi32(m[4], ld5(*r6, 0)); // packssdw m4, [r6+32*0]
    m[5] = _mm256_packs_epi32(m[5], ld5(*r6, 32)); // packssdw m5, [r6+32*1]
    m[6] = _mm256_packs_epi32(m[6], ld5(*r6, 32 * 2)); // packssdw m6, [r6+32*2]
    m[7] = _mm256_packs_epi32(m[7], ld5(*r6, 32 * 3)); // packssdw m7, [r6+32*3]
    st5(*r6, 0, m[8]); // mova [r6], m8
    m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m1, m2, m3
    m[2] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m3, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m7, m4, m5
    m[4] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m4, m5
    m[5] = _mm256_unpacklo_epi32(m[8], m[2]); // punpckldq m5, m8, m2
    m[8] = _mm256_unpackhi_epi32(m[8], m[2]); // punpckhdq m8, m2
    m[2] = _mm256_unpackhi_epi32(m[0], m[1]); // punpckhdq m2, m0, m1
    m[0] = _mm256_unpacklo_epi32(m[0], m[1]); // punpckldq m0, m1
    m[1] = _mm256_unpackhi_epi32(m[7], m[6]); // punpckhdq m1, m7, m6
    m[7] = _mm256_unpacklo_epi32(m[7], m[6]); // punpckldq m7, m6
    m[6] = _mm256_unpackhi_epi32(m[4], m[3]); // punpckhdq m6, m4, m3
    m[4] = _mm256_unpacklo_epi32(m[4], m[3]); // punpckldq m4, m3
    m[3] = _mm256_unpackhi_epi64(m[2], m[1]); // punpckhqdq m3, m2, m1
    m[2] = _mm256_unpacklo_epi64(m[2], m[1]); // punpcklqdq m2, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[7]); // punpckhqdq m1, m0, m7
    m[0] = _mm256_unpacklo_epi64(m[0], m[7]); // punpcklqdq m0, m7
    m[7] = _mm256_unpacklo_epi64(m[8], m[6]); // punpcklqdq m7, m8, m6
    m[8] = _mm256_unpackhi_epi64(m[8], m[6]); // punpckhqdq m8, m6
    m[6] = _mm256_unpackhi_epi64(m[5], m[4]); // punpckhqdq m6, m5, m4
    m[5] = _mm256_unpacklo_epi64(m[5], m[4]); // punpcklqdq m5, m4
    m[4] = ld5(*r6, 0); // mova m4, [r6]
    st5(*r6, 0, m[8]); // mova [r6], m8
    m[8] = _mm256_unpacklo_epi16(m[4], m[9]); // punpcklwd m8, m4, m9
    m[4] = _mm256_unpackhi_epi16(m[4], m[9]); // punpckhwd m4, m9
    m[9] = _mm256_unpacklo_epi16(m[10], m[11]); // punpcklwd m9, m10, m11
    m[10] = _mm256_unpackhi_epi16(m[10], m[11]); // punpckhwd m10, m11
    m[11] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m11, m14, m15
    m[14] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m14, m15
    m[15] = _mm256_unpackhi_epi16(m[12], m[13]); // punpckhwd m15, m12, m13
    m[12] = _mm256_unpacklo_epi16(m[12], m[13]); // punpcklwd m12, m13
    m[13] = _mm256_unpacklo_epi32(m[4], m[10]); // punpckldq m13, m4, m10
    m[4] = _mm256_unpackhi_epi32(m[4], m[10]); // punpckhdq m4, m10
    m[10] = _mm256_unpackhi_epi32(m[8], m[9]); // punpckhdq m10, m8, m9
    m[8] = _mm256_unpacklo_epi32(m[8], m[9]); // punpckldq m8, m9
    m[9] = _mm256_unpackhi_epi32(m[12], m[14]); // punpckhdq m9, m12, m14
    m[12] = _mm256_unpacklo_epi32(m[12], m[14]); // punpckldq m12, m14
    m[14] = _mm256_unpackhi_epi32(m[15], m[11]); // punpckhdq m14, m15, m11
    m[15] = _mm256_unpacklo_epi32(m[15], m[11]); // punpckldq m15, m11
    m[11] = _mm256_unpackhi_epi64(m[10], m[9]); // punpckhqdq m11, m10, m9
    m[10] = _mm256_unpacklo_epi64(m[10], m[9]); // punpcklqdq m10, m9
    m[9] = _mm256_unpackhi_epi64(m[8], m[12]); // punpckhqdq m9, m8, m12
    m[8] = _mm256_unpacklo_epi64(m[8], m[12]); // punpcklqdq m8, m12
    m[12] = _mm256_unpacklo_epi64(m[13], m[15]); // punpcklqdq m12, m13, m15
    m[13] = _mm256_unpackhi_epi64(m[13], m[15]); // punpckhqdq m13, m15
    m[15] = _mm256_unpackhi_epi64(m[4], m[14]); // punpckhqdq m15, m4, m14
    m[14] = _mm256_unpacklo_epi64(m[4], m[14]); // punpcklqdq m14, m4, m14
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[8]); // vperm2i128 m4, m0, m8, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[8])); // vinserti128 m0, xm8, 1
    m[8] = _mm256_inserti128_si256::<1>(m[5], lo(m[12])); // vinserti128 m8, m5, xm12, 1
    m[12] = _mm256_permute2x128_si256::<0x13>(m[12], m[5]); // vperm2i128 m12, m5, 0x13
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[9]); // vperm2i128 m5, m1, m9, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[9])); // vinserti128 m1, xm9, 1
    m[9] = _mm256_inserti128_si256::<1>(m[6], lo(m[13])); // vinserti128 m9, m6, xm13, 1
    m[13] = _mm256_permute2x128_si256::<0x13>(m[13], m[6]); // vperm2i128 m13, m6, 0x13
    m[6] = _mm256_permute2x128_si256::<0x31>(m[2], m[10]); // vperm2i128 m6, m2, m10, 0x31
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[10])); // vinserti128 m2, xm10, 1
    m[10] = _mm256_inserti128_si256::<1>(m[7], lo(m[14])); // vinserti128 m10, m7, xm14, 1
    m[14] = _mm256_permute2x128_si256::<0x13>(m[14], m[7]); // vperm2i128 m14, m7, 0x13
    m[7] = _mm256_permute2x128_si256::<0x31>(m[3], m[11]); // vperm2i128 m7, m3, m11, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[11])); // vinserti128 m3, xm11, 1
    m[11] = zx(_mm_loadu_si128((*r6).cast())); // mova xm11, [r6]
    m[11] = _mm256_inserti128_si256::<1>(m[11], lo(m[15])); // vinserti128 m11, xm15, 1
    m[15] = _mm256_inserti128_si256::<0>(m[15], _mm_loadu_si128(at(*r6, 16).cast()));
    // vinserti128 m15, [r6+16], 0
}

/// `.transpose_fast` of `idct_16x16_internal_10bpc`: the left half only; `m8`-`m15` are zeroed.
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_transpose_fast(m: &mut M) {
    idct_16x8_internal_10bpc_transpose2(m); // call m(idct_16x8_internal_10bpc).transpose2
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    for x in [9, 10, 11, 12, 13, 14, 15] {
        m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
    }
}

/// `.main` of `idct_16x16_internal_10bpc`: the 16-point DCT of eight columns at `c` (`cq` as the caller set
/// it); out8-out15 of the odd half go to the frame slots at `r6` (`[r6-32*4..r6+32*3]`). Expects `m11` = pd_2048,
/// `m12`/`m13` the clipping bounds, `m14` = pd_2896.
#[inline(always)]
unsafe fn idct_16x16_internal_10bpc_main(m: &mut M, c: Coef, r6: *mut __m256i) {
    m[0] = ld5(c, 64 * 1); // mova m0, [cq+64* 1]
    m[1] = ld5(c, 64 * 3); // mova m1, [cq+64* 3]
    m[2] = ld5(c, 64 * 5); // mova m2, [cq+64* 5]
    m[3] = ld5(c, 64 * 7); // mova m3, [cq+64* 7]
    m[4] = ld5(c, 64 * 9); // mova m4, [cq+64* 9]
    m[5] = ld5(c, 64 * 11); // mova m5, [cq+64*11]
    m[6] = ld5(c, 64 * 13); // mova m6, [cq+64*13]
    m[7] = ld5(c, 64 * 15); // mova m7, [cq+64*15]
    idct_8x16_internal_10bpc_main_oddhalf(m, r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
    m[0] = ld5(c, 0); // mova m0, [cq+64* 0]
    m[1] = ld5(c, 64 * 2); // mova m1, [cq+64* 2]
    m[2] = ld5(c, 64 * 4); // mova m2, [cq+64* 4]
    m[3] = ld5(c, 64 * 6); // mova m3, [cq+64* 6]
    m[4] = ld5(c, 64 * 8); // mova m4, [cq+64* 8]
    m[5] = ld5(c, 64 * 10); // mova m5, [cq+64*10]
    m[6] = ld5(c, 64 * 12); // mova m6, [cq+64*12]
    m[7] = ld5(c, 64 * 14); // mova m7, [cq+64*14]
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[10] = _mm256_srli_epi32::<10>(m[11]); // psrld m10, m11, 10 ; pd_2
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[10]); // REPX {paddd x, m10}, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass1` of `iadst_16x16_internal_10bpc` up to `jmp tx2q` (through `.pass1_end`; the clipping bounds in
/// `m13`, `m14` on entry)
#[inline(always)]
unsafe fn iadst_16x16_internal_10bpc_pass1(
    m: &mut M,
    c: Coef,
    eob: &mut c_int,
    rsp: *mut __m256i,
    r6: &mut *mut __m256i,
) {
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    *r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    *eob = eob.wrapping_sub(36); // sub eobd, 36
    if *eob >= 0 {
        // jl .fast
        iadst_16x16_internal_10bpc_main(m, at(c, 32), *r6); // add cq, 32 / call .main / sub cq, 32
        m[8] = pd(5120); // vpbroadcastd m8, [pd_5120]
        m[4] = _mm256_add_epi32(m[4], m[8]); // paddd m4, m8
        m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8
        m[9] = _mm256_add_epi32(m[9], m[8]); // paddd m9, m8
        m[11] = _mm256_add_epi32(m[11], m[8]); // paddd m11, m8
        m[8] = pd(5119); // vpbroadcastd m8, [pd_5119]
        m[5] = _mm256_sub_epi32(m[8], m[5]); // psubd m5, m8, m5
        m[7] = _mm256_sub_epi32(m[8], m[7]); // psubd m7, m8, m7
        m[10] = _mm256_sub_epi32(m[8], m[10]); // psubd m10, m8, m10
        m[12] = _mm256_sub_epi32(m[8], m[12]); // psubd m12, m8, m12
        for x in [4, 5, 6, 7, 9, 10, 11, 12] {
            m[x] = _mm256_srai_epi32::<13>(m[x]); // REPX {psrad x, 13}, m4, m5, m6, m7, m9, m10, m11, m12
        }
        st5(*r6, 0, m[4]); // mova [r6+32*0], m4
        st5(*r6, 32, m[5]); // mova [r6+32*1], m5
        st5(*r6, 32 * 2, m[6]); // mova [r6+32*2], m6
        st5(*r6, 32 * 3, m[7]); // mova [r6+32*3], m7
        m[4] = _mm256_srli_epi32::<10>(m[15]); // psrld m4, m15, 10 ; pd_2
        m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4
        m[1] = _mm256_sub_epi32(m[4], m[1]); // psubd m1, m4, m1
        m[2] = _mm256_add_epi32(m[2], m[4]); // paddd m2, m4
        m[3] = _mm256_sub_epi32(m[4], m[3]); // psubd m3, m4, m3
        m[7] = _mm256_sub_epi32(m[4], ld5(*r6, -32 * 4)); // psubd m7, m4, [r6-32*4]
        m[6] = _mm256_add_epi32(m[4], ld5(*r6, -32 * 3)); // paddd m6, m4, [r6-32*3]
        m[5] = _mm256_sub_epi32(m[4], ld5(*r6, -32 * 2)); // psubd m5, m4, [r6-32*2]
        m[4] = _mm256_add_epi32(m[4], ld5(*r6, -32)); // paddd m4, [r6-32*1]
        for x in [0, 1, 2, 3, 4, 5, 6, 7] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2 }, m0, m1, m2, m3, m4, m5, m6, m7
        }
        st5(*r6, -32 * 4, m[0]); // mova [r6-32*4], m0
        st5(*r6, -32 * 3, m[1]); // mova [r6-32*3], m1
        st5(*r6, -32 * 2, m[2]); // mova [r6-32*2], m2
        st5(*r6, -32, m[3]); // mova [r6-32*1], m3
        *r6 = at(*r6, 32 * 8); // add r6, 32*8
        st5(*r6, -32 * 4, m[9]); // mova [r6-32*4], m9
        st5(*r6, -32 * 3, m[10]); // mova [r6-32*3], m10
        st5(*r6, -32 * 2, m[11]); // mova [r6-32*2], m11
        st5(*r6, -32, m[12]); // mova [r6-32*1], m12
        st5(*r6, 0, m[4]); // mova [r6+32*0], m4
        st5(*r6, 32, m[5]); // mova [r6+32*1], m5
        st5(*r6, 32 * 2, m[6]); // mova [r6+32*2], m6
        st5(*r6, 32 * 3, m[7]); // mova [r6+32*3], m7
    }
    // .fast:
    *r6 = at(*r6, 32 * 8); // add r6, 32*8
    iadst_16x16_internal_10bpc_main(m, c, *r6); // call .main
    m[14] = pd(5120); // vpbroadcastd m14, [pd_5120]
    m[13] = pd(5119); // vpbroadcastd m13, [pd_5119]
    m[15] = _mm256_srli_epi32::<10>(m[15]); // psrld m15, 10 ; pd_2
    m[0] = _mm256_add_epi32(m[0], m[15]); // paddd m0, m15
    m[1] = _mm256_sub_epi32(m[15], m[1]); // psubd m1, m15, m1
    m[2] = _mm256_add_epi32(m[2], m[15]); // paddd m2, m15
    m[3] = _mm256_sub_epi32(m[15], m[3]); // psubd m3, m15, m3
    m[4] = _mm256_add_epi32(m[4], m[14]); // paddd m4, m14
    m[5] = _mm256_sub_epi32(m[13], m[5]); // psubd m5, m13, m5
    m[6] = _mm256_add_epi32(m[6], m[14]); // paddd m6, m14
    m[7] = _mm256_sub_epi32(m[13], m[7]); // psubd m7, m13, m7
    m[8] = _mm256_add_epi32(m[14], m[9]); // paddd m8, m14, m9
    m[9] = _mm256_sub_epi32(m[13], m[10]); // psubd m9, m13, m10
    m[10] = _mm256_add_epi32(m[14], m[11]); // paddd m10, m14, m11
    m[11] = _mm256_sub_epi32(m[13], m[12]); // psubd m11, m13, m12
    m[12] = _mm256_add_epi32(m[15], ld5(*r6, -32)); // paddd m12, m15, [r6-32*1]
    m[13] = _mm256_sub_epi32(m[15], ld5(*r6, -32 * 2)); // psubd m13, m15, [r6-32*2]
    m[14] = _mm256_add_epi32(m[15], ld5(*r6, -32 * 3)); // paddd m14, m15, [r6-32*3]
    m[15] = _mm256_sub_epi32(m[15], ld5(*r6, -32 * 4)); // psubd m15, [r6-32*4]
    iadst_16x16_internal_10bpc_pass1_end(m, r6)
}

/// `.pass1_end` of `iadst_16x16_internal_10bpc` (to `jmp tx2q`)
#[inline(always)]
unsafe fn iadst_16x16_internal_10bpc_pass1_end(m: &mut M, r6: &mut *mut __m256i) {
    for x in [0, 1, 2, 3, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2 }, m0, m1, m2, m3, m12, m13, m14, m15
    }
    for x in [4, 5, 6, 7, 8, 9, 10, 11] {
        m[x] = _mm256_srai_epi32::<13>(m[x]); // REPX {psrad x, 13}, m4, m5, m6, m7, m8, m9, m10, m11
    }
    *r6 = at(*r6, -32 * 8); // sub r6, 32*8
                            // jmp tx2q
}

/// `.pass2` of `iadst_16x16_internal_10bpc` (to its `RET`)
#[inline(always)]
unsafe fn iadst_16x16_internal_10bpc_pass2(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    eob: c_int,
    rsp: *mut __m256i,
    mut r6: *mut __m256i,
) {
    let mut r3: isize = 0; // r3: set by .write_16x4_start (lea r3, [strideq*3])
    idct_16x16_internal_10bpc_transpose(m, eob, &mut r6); // call m(idct_16x16_internal_10bpc).transpose
                                                          // lea r6, [pw_5+128] (the base of the 8 bpc constants: no code)
    st5(rsp, 0, m[15]); // mova [rsp], m15
    iadst_16x16_internal_8bpc_main(m, spill5(rsp)); // call m(iadst_16x16_internal_8bpc).main
    iadst_16x16_internal_8bpc_main_pass2_end(m); // call m(iadst_16x16_internal_8bpc).main_pass2_end
    st5(rsp, 0, m[8]); // mova [rsp+32*0], m8
    st5(rsp, 32 * 2, m[12]); // mova [rsp+32*2], m12
    st5(rsp, 32 * 3, m[13]); // mova [rsp+32*3], m13
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[13] = _mm256_setzero_si256(); // pxor m13, m13
    m[13] = _mm256_sub_epi16(m[13], m[12]); // psubw m13, m12
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[13], ld5(rsp, 32)); // pmulhrsw m1, m13, [rsp+32*1]
    st5(rsp, 32, m[9]); // mova [rsp+32*1], m9
    m[2] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m2, m12
    m[3] = _mm256_mulhrs_epi16(m[3], m[13]); // pmulhrsw m3, m13
    idct_16x8_internal_10bpc_write_16x4_start(m, &mut dst, stride, &mut c, &mut r3); // call m(idct_16x8_internal_10bpc).write_16x4_start
    m[0] = _mm256_mulhrs_epi16(m[12], m[4]); // pmulhrsw m0, m12, m4
    m[1] = _mm256_mulhrs_epi16(m[13], m[5]); // pmulhrsw m1, m13, m5
    m[2] = _mm256_mulhrs_epi16(m[12], m[6]); // pmulhrsw m2, m12, m6
    m[3] = _mm256_mulhrs_epi16(m[13], m[7]); // pmulhrsw m3, m13, m7
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 0)); // pmulhrsw m0, m12, [rsp+32*0]
    m[1] = _mm256_mulhrs_epi16(m[13], ld5(rsp, 32)); // pmulhrsw m1, m13, [rsp+32*1]
    m[2] = _mm256_mulhrs_epi16(m[12], m[10]); // pmulhrsw m2, m12, m10
    m[3] = _mm256_mulhrs_epi16(m[13], m[11]); // pmulhrsw m3, m13, m11
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 32 * 2)); // pmulhrsw m0, m12, [rsp+32*2]
    m[1] = _mm256_mulhrs_epi16(m[13], ld5(rsp, 32 * 3)); // pmulhrsw m1, m13, [rsp+32*3]
    m[2] = _mm256_mulhrs_epi16(m[12], m[14]); // pmulhrsw m2, m12, m14
    m[3] = _mm256_mulhrs_epi16(m[13], m[15]); // pmulhrsw m3, m13, m15
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3);
    // call m(idct_16x8_internal_10bpc).write_16x4_zero
    // RET
}

/// `.main` of `iadst_16x16_internal_10bpc`: the 16-point ADST of eight columns at `c`; uses the frame slots at
/// `r6` (`[r6-32*4..r6+32*3]`). Expects `m13`/`m14` the clipping bounds, `m15` = pd_2896.
#[inline(always)]
unsafe fn iadst_16x16_internal_10bpc_main(m: &mut M, c: Coef, r6: *mut __m256i) {
    m[0] = ld5(c, 64 * 2); // mova m0, [cq+64* 2]
    m[1] = ld5(c, 64 * 13); // mova m1, [cq+64*13]
    m[2] = ld5(c, 64 * 6); // mova m2, [cq+64* 6]
    m[3] = ld5(c, 64 * 9); // mova m3, [cq+64* 9]
    m[4] = ld5(c, 64 * 10); // mova m4, [cq+64*10]
    m[5] = ld5(c, 64 * 5); // mova m5, [cq+64* 5]
    m[6] = ld5(c, 64 * 14); // mova m6, [cq+64*14]
    m[7] = ld5(c, 64 * 1); // mova m7, [cq+64* 1]
    m[12] = pd(2048); // vpbroadcastd m12, [pd_2048]
    iadst_16x8_internal_10bpc_main_part1(m, r6); // call m(iadst_16x8_internal_10bpc).main_part1
    m[0] = ld5(c, 0); // mova m0, [cq+64* 0]
    m[1] = ld5(c, 64 * 15); // mova m1, [cq+64*15]
    m[2] = ld5(c, 64 * 4); // mova m2, [cq+64* 4]
    m[3] = ld5(c, 64 * 11); // mova m3, [cq+64*11]
    m[4] = ld5(c, 64 * 8); // mova m4, [cq+64* 8]
    m[5] = ld5(c, 64 * 7); // mova m5, [cq+64* 7]
    m[6] = ld5(c, 64 * 12); // mova m6, [cq+64*12]
    m[7] = ld5(c, 64 * 3); // mova m7, [cq+64* 3]
    iadst_16x8_internal_10bpc_main_part2(m, r6) // jmp m(iadst_16x8_internal_10bpc).main_part2
}

/// `.pass1` of `iflipadst_16x16_internal_10bpc` up to `jmp tx2q` (through `iadst_16x16_internal_10bpc`'s
/// `.pass1_end`; the clipping bounds in `m13`, `m14` on entry)
#[inline(always)]
unsafe fn iflipadst_16x16_internal_10bpc_pass1(
    m: &mut M,
    c: Coef,
    eob: &mut c_int,
    rsp: *mut __m256i,
    r6: &mut *mut __m256i,
) {
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    *r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    *eob = eob.wrapping_sub(36); // sub eobd, 36
    if *eob >= 0 {
        // jl .fast
        iadst_16x16_internal_10bpc_main(m, at(c, 32), *r6); // add cq, 32 / call m(iadst_16x16_internal_10bpc).main / sub cq, 32
        m[8] = pd(5120); // vpbroadcastd m8, [pd_5120]
        m[11] = _mm256_add_epi32(m[11], m[8]); // paddd m11, m8
        m[9] = _mm256_add_epi32(m[9], m[8]); // paddd m9, m8
        m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8
        m[4] = _mm256_add_epi32(m[4], m[8]); // paddd m4, m8
        m[8] = pd(5119); // vpbroadcastd m8, [pd_5119]
        m[12] = _mm256_sub_epi32(m[8], m[12]); // psubd m12, m8, m12
        m[10] = _mm256_sub_epi32(m[8], m[10]); // psubd m10, m8, m10
        m[7] = _mm256_sub_epi32(m[8], m[7]); // psubd m7, m8, m7
        m[5] = _mm256_sub_epi32(m[8], m[5]); // psubd m5, m8, m5
        for x in [12, 11, 10, 9, 7, 6, 5, 4] {
            m[x] = _mm256_srai_epi32::<13>(m[x]); // REPX {psrad x, 13}, m12, m11, m10, m9, m7, m6, m5, m4
        }
        st5(*r6, 0, m[12]); // mova [r6+32*0], m12
        st5(*r6, 32, m[11]); // mova [r6+32*1], m11
        st5(*r6, 32 * 2, m[10]); // mova [r6+32*2], m10
        st5(*r6, 32 * 3, m[9]); // mova [r6+32*3], m9
        m[9] = _mm256_srli_epi32::<10>(m[15]); // psrld m9, m15, 10 ; pd_2
        m[3] = _mm256_sub_epi32(m[9], m[3]); // psubd m3, m9, m3
        m[2] = _mm256_add_epi32(m[2], m[9]); // paddd m2, m9
        m[1] = _mm256_sub_epi32(m[9], m[1]); // psubd m1, m9, m1
        m[0] = _mm256_add_epi32(m[0], m[9]); // paddd m0, m9
        m[12] = _mm256_sub_epi32(m[9], ld5(*r6, -32 * 4)); // psubd m12, m9, [r6-32*4]
        m[11] = _mm256_add_epi32(m[9], ld5(*r6, -32 * 3)); // paddd m11, m9, [r6-32*3]
        m[10] = _mm256_sub_epi32(m[9], ld5(*r6, -32 * 2)); // psubd m10, m9, [r6-32*2]
        m[9] = _mm256_add_epi32(m[9], ld5(*r6, -32)); // paddd m9, [r6-32*1]
        for x in [12, 11, 10, 9, 3, 2, 1, 0] {
            m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2 }, m12, m11, m10, m9, m3, m2, m1, m0
        }
        st5(*r6, -32 * 4, m[12]); // mova [r6-32*4], m12
        st5(*r6, -32 * 3, m[11]); // mova [r6-32*3], m11
        st5(*r6, -32 * 2, m[10]); // mova [r6-32*2], m10
        st5(*r6, -32, m[9]); // mova [r6-32*1], m9
        *r6 = at(*r6, 32 * 8); // add r6, 32*8
        st5(*r6, -32 * 4, m[7]); // mova [r6-32*4], m7
        st5(*r6, -32 * 3, m[6]); // mova [r6-32*3], m6
        st5(*r6, -32 * 2, m[5]); // mova [r6-32*2], m5
        st5(*r6, -32, m[4]); // mova [r6-32*1], m4
        st5(*r6, 0, m[3]); // mova [r6+32*0], m3
        st5(*r6, 32, m[2]); // mova [r6+32*1], m2
        st5(*r6, 32 * 2, m[1]); // mova [r6+32*2], m1
        st5(*r6, 32 * 3, m[0]); // mova [r6+32*3], m0
    }
    // .fast:
    *r6 = at(*r6, 32 * 8); // add r6, 32*8
    iadst_16x16_internal_10bpc_main(m, c, *r6); // call m(iadst_16x16_internal_10bpc).main
    m[14] = pd(5120); // vpbroadcastd m14, [pd_5120]
    m[13] = pd(5119); // vpbroadcastd m13, [pd_5119]
    m[15] = _mm256_srli_epi32::<10>(m[15]); // psrld m15, 10 ; pd_2
    m[8] = _mm256_sub_epi32(m[13], m[7]); // psubd m8, m13, m7
    m[7] = _mm256_add_epi32(m[14], m[9]); // paddd m7, m14, m9
    m[9] = _mm256_add_epi32(m[14], m[6]); // paddd m9, m14, m6
    m[6] = _mm256_sub_epi32(m[13], m[10]); // psubd m6, m13, m10
    m[10] = _mm256_sub_epi32(m[13], m[5]); // psubd m10, m13, m5
    m[5] = _mm256_add_epi32(m[14], m[11]); // paddd m5, m14, m11
    m[11] = _mm256_add_epi32(m[14], m[4]); // paddd m11, m14, m4
    m[4] = _mm256_sub_epi32(m[13], m[12]); // psubd m4, m13, m12
    m[12] = _mm256_sub_epi32(m[15], m[3]); // psubd m12, m15, m3
    m[3] = _mm256_add_epi32(m[15], ld5(*r6, -32)); // paddd m3, m15, [r6-32*1]
    m[13] = _mm256_add_epi32(m[15], m[2]); // paddd m13, m15, m2
    m[2] = _mm256_sub_epi32(m[15], ld5(*r6, -32 * 2)); // psubd m2, m15, [r6-32*2]
    m[14] = _mm256_sub_epi32(m[15], m[1]); // psubd m14, m15, m1
    m[1] = m[15]; // mova m1, m15
    m[15] = _mm256_add_epi32(m[15], m[0]); // paddd m15, m0
    m[0] = _mm256_sub_epi32(m[1], ld5(*r6, -32 * 4)); // psubd m0, m1, [r6-32*4]
    m[1] = _mm256_add_epi32(m[1], ld5(*r6, -32 * 3)); // paddd m1, [r6-32*3]
    iadst_16x16_internal_10bpc_pass1_end(m, r6) // jmp m(iadst_16x16_internal_10bpc).pass1_end
}

/// `.pass2` of `iflipadst_16x16_internal_10bpc` (to its `RET`)
#[inline(always)]
unsafe fn iflipadst_16x16_internal_10bpc_pass2(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    eob: c_int,
    rsp: *mut __m256i,
    mut r6: *mut __m256i,
) {
    let mut r3: isize = 0; // r3: set by .write_16x4_start (lea r3, [strideq*3])
    idct_16x16_internal_10bpc_transpose(m, eob, &mut r6); // call m(idct_16x16_internal_10bpc).transpose
                                                          // lea r6, [pw_5+128] (the base of the 8 bpc constants: no code)
    st5(rsp, 0, m[15]); // mova [rsp], m15
    iadst_16x16_internal_8bpc_main(m, spill5(rsp)); // call m(iadst_16x16_internal_8bpc).main
    iadst_16x16_internal_8bpc_main_pass2_end(m); // call m(iadst_16x16_internal_8bpc).main_pass2_end
    st5(rsp, 32 * 3, m[3]); // mova [rsp+32*3], m3
    st5(rsp, 32 * 2, m[2]); // mova [rsp+32*2], m2
    st5(rsp, 0, m[0]); // mova [rsp+32*0], m0
    m[2] = m[13]; // mova m2, m13
    m[3] = m[12]; // mova m3, m12
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[13] = _mm256_setzero_si256(); // pxor m13, m13
    m[13] = _mm256_sub_epi16(m[13], m[12]); // psubw m13, m12
    m[0] = _mm256_mulhrs_epi16(m[13], m[15]); // pmulhrsw m0, m13, m15
    m[1] = _mm256_mulhrs_epi16(m[12], m[14]); // pmulhrsw m1, m12, m14
    m[2] = _mm256_mulhrs_epi16(m[2], m[13]); // pmulhrsw m2, m13
    m[3] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m3, m12
    m[14] = m[8]; // mova m14, m8
    m[15] = m[9]; // mova m15, m9
    idct_16x8_internal_10bpc_write_16x4_start(m, &mut dst, stride, &mut c, &mut r3); // call m(idct_16x8_internal_10bpc).write_16x4_start
    m[0] = _mm256_mulhrs_epi16(m[13], m[11]); // pmulhrsw m0, m13, m11
    m[1] = _mm256_mulhrs_epi16(m[12], m[10]); // pmulhrsw m1, m12, m10
    m[2] = _mm256_mulhrs_epi16(m[13], m[15]); // pmulhrsw m2, m13, m15
    m[3] = _mm256_mulhrs_epi16(m[12], m[14]); // pmulhrsw m3, m12, m14
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_mulhrs_epi16(m[13], m[7]); // pmulhrsw m0, m13, m7
    m[1] = _mm256_mulhrs_epi16(m[12], m[6]); // pmulhrsw m1, m12, m6
    m[2] = _mm256_mulhrs_epi16(m[13], m[5]); // pmulhrsw m2, m13, m5
    m[3] = _mm256_mulhrs_epi16(m[12], m[4]); // pmulhrsw m3, m12, m4
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_mulhrs_epi16(m[13], ld5(rsp, 32 * 3)); // pmulhrsw m0, m13, [rsp+32*3]
    m[1] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 32 * 2)); // pmulhrsw m1, m12, [rsp+32*2]
    m[2] = _mm256_mulhrs_epi16(m[13], ld5(rsp, 32)); // pmulhrsw m2, m13, [rsp+32*1]
    m[3] = _mm256_mulhrs_epi16(m[12], ld5(rsp, 0)); // pmulhrsw m3, m12, [rsp+32*0]
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3);
    // call m(idct_16x8_internal_10bpc).write_16x4_zero
    // RET
}

/// `cglobal iidentity_16x16_internal_10bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_16x16_internal_10bpc_pass1(
    m: &mut M,
    c: Coef,
    eob: &mut c_int,
    rsp: *mut __m256i,
    r6: &mut *mut __m256i,
) {
    m[15] = pd(5793); // vpbroadcastd m15, [pd_5793]
    m[7] = pd(5120); // vpbroadcastd m7, [pd_5120]
    *r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    *eob = eob.wrapping_sub(36); // sub eobd, 36
    if *eob >= 0 {
        // jl .fast
        let mut r3: isize = -32 * 8 * 4; // mov r3, -32*8*4
        loop {
            // .righthalf:
            m[0] = _mm256_mullo_epi32(m[15], ld5(c, r3 + 32 * 33)); // pmulld m0, m15, [cq+r3+32*33]
            m[1] = _mm256_mullo_epi32(m[15], ld5(c, r3 + 32 * 35)); // pmulld m1, m15, [cq+r3+32*35]
            m[2] = _mm256_mullo_epi32(m[15], ld5(c, r3 + 32 * 37)); // pmulld m2, m15, [cq+r3+32*37]
            m[3] = _mm256_mullo_epi32(m[15], ld5(c, r3 + 32 * 39)); // pmulld m3, m15, [cq+r3+32*39]
            *r6 = at(*r6, 32 * 4); // add r6, 32*4
            for x in [0, 1, 2, 3] {
                m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m0, m1, m2, m3
            }
            for x in [0, 1, 2, 3] {
                m[x] = _mm256_srai_epi32::<13>(m[x]); // REPX {psrad x, 13}, m0, m1, m2, m3
            }
            st5(*r6, 0, m[0]); // mova [r6+32*0], m0
            st5(*r6, 32, m[1]); // mova [r6+32*1], m1
            st5(*r6, 32 * 2, m[2]); // mova [r6+32*2], m2
            st5(*r6, 32 * 3, m[3]); // mova [r6+32*3], m3
            r3 += 32 * 8; // add r3, 32*8
            if r3 >= 0 {
                break; // jl .righthalf
            }
        }
    }
    // .fast:
    m[0] = _mm256_mullo_epi32(m[15], ld5(c, 0)); // pmulld m0, m15, [cq+64* 0]
    m[1] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 1)); // pmulld m1, m15, [cq+64* 1]
    m[2] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 2)); // pmulld m2, m15, [cq+64* 2]
    m[3] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 3)); // pmulld m3, m15, [cq+64* 3]
    m[4] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 4)); // pmulld m4, m15, [cq+64* 4]
    m[5] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 5)); // pmulld m5, m15, [cq+64* 5]
    m[6] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 6)); // pmulld m6, m15, [cq+64* 6]
    m[8] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 7)); // pmulld m8, m15, [cq+64* 7]
    st5(c, 0, m[8]); // mova [cq], m8
    m[8] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 8)); // pmulld m8, m15, [cq+64* 8]
    m[9] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 9)); // pmulld m9, m15, [cq+64* 9]
    m[10] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 10)); // pmulld m10, m15, [cq+64*10]
    m[11] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 11)); // pmulld m11, m15, [cq+64*11]
    m[12] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 12)); // pmulld m12, m15, [cq+64*12]
    m[13] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 13)); // pmulld m13, m15, [cq+64*13]
    m[14] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 14)); // pmulld m14, m15, [cq+64*14]
    m[15] = _mm256_mullo_epi32(m[15], ld5(c, 64 * 15)); // pmulld m15, [cq+64*15]
    for x in [0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15] {
        m[x] = _mm256_add_epi32(m[x], m[7]); // REPX {paddd x, m7}, m0, …, m6, m8, …, m15
    }
    m[7] = _mm256_add_epi32(m[7], ld5(c, 0)); // paddd m7, [cq]
    for x in 0..16 {
        m[x] = _mm256_srai_epi32::<13>(m[x]); // REPX {psrad x, 13}, m0, m1, …, m15
    }
    // jmp tx2q
}

/// `.pass2` of `iidentity_16x16_internal_10bpc` (to the `RET` of `idct_16x16_internal_10bpc`'s `.end`)
#[inline(always)]
unsafe fn iidentity_16x16_internal_10bpc_pass2(
    m: &mut M,
    dst: Dst,
    stride: isize,
    c: Coef,
    eob: c_int,
    rsp: *mut __m256i,
    mut r6: *mut __m256i,
) {
    idct_16x16_internal_10bpc_transpose(m, eob, &mut r6); // call m(idct_16x16_internal_10bpc).transpose
    st5(c, 0, m[15]); // mova [cq+32*0], m15
    st5(c, 32, m[0]); // mova [cq+32*1], m0
    m[15] = pw(1697 * 16); // vpbroadcastd m15, [pw_1697x16]
    for x in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14] {
        idtx16(m, x, 0, 15, Idtx16Shift::None); // REPX {IDTX16 x, 0, 15}, 1, 2, …, 14
    }
    m[0] = ld5(c, 32); // mova m0, [cq+32*1]
    st5(c, 32, m[1]); // mova [cq+32*1], m1
    idtx16(m, 0, 1, 15, Idtx16Shift::None); // IDTX16 0, 1, 15
    m[1] = ld5(c, 0); // mova m1, [cq+32*0]
    m[15] = _mm256_mulhrs_epi16(m[15], m[1]); // pmulhrsw m15, m1
    m[1] = _mm256_adds_epi16(m[1], m[1]); // paddsw m1, m1
    m[15] = _mm256_adds_epi16(m[15], m[1]); // paddsw m15, m1
    m[1] = ld5(c, 32); // mova m1, [cq+32*1]
    idct_16x16_internal_10bpc_end(m, dst, stride, c, rsp) // jmp m(idct_16x16_internal_10bpc).end
}

// --- 12 bpc ---

/// `INV_TXFM_16X16_FN type1, type2, eob_offset, 12` (the entry code and `jmp tx2q`) with the frame of
/// `i<type1>_16x16_internal_12bpc`.
#[inline(always)]
unsafe fn inv_txfm_add_16x16_12bpc(dst: Dst, stride: isize, c: Coef, mut eob: c_int, t1: K, t2: K) {
    let mut m = regs();
    let m = &mut m;
    if t1 == K::Dct && t2 == K::Dct {
        if eob == 0 {
            // test eobd, eobd / jnz m(idct_16x16_internal_12bpc)
            let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
            m[3] = pw(0x7000); // vpbroadcastd m3, [dconly_12bpc]
            c.write_unaligned(eob); // mov [cq], eobd ; 0
            let r3d = eob | 16; // or r3d, 16
            r6d = r6d.wrapping_add(640); // add r6d, 640
            r6d >>= 10; // sar r6d, 10
            return inv_txfm_add_dct_dct_16x4_10bpc_dconly3(m, dst, stride, r6d, r3d);
            // jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly3
        }
    } else if t1 == K::Dct && t2 == K::Identity {
        eob += 28; // add eobd, 28
    } else if t1 == K::Identity && t2 == K::Dct {
        eob += -92; // add eobd, -92
    }
    // Not in the original: the frame is zeroed whole (which slots the second pass reads depends on the eob path
    // of the first).
    let mut frame = [_mm256_setzero_si256(); 24];
    let rsp = frame.as_mut_ptr();
    let mut r6 = rsp;
    match t1 {
        K::Dct => {
            m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
            m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
            idct_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6); // jmp m(idct_16x16_internal_10bpc).pass1
        }
        K::Adst => {
            m[13] = pd(-0x80000); // vpbroadcastd m13, [clip_20b_min]
            m[14] = pd(0x7ffff); // vpbroadcastd m14, [clip_20b_max]
            iadst_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6); // jmp m(iadst_16x16_internal_10bpc).pass1
        }
        K::FlipAdst => {
            m[13] = pd(-0x80000); // vpbroadcastd m13, [clip_20b_min]
            m[14] = pd(0x7ffff); // vpbroadcastd m14, [clip_20b_max]
            iflipadst_16x16_internal_10bpc_pass1(m, c, &mut eob, rsp, &mut r6); // jmp m(iflipadst_16x16_internal_10bpc).pass1
        }
        K::Identity => iidentity_16x16_internal_12bpc_pass1(m, c, &mut eob, rsp, &mut r6),
    }
    match t2 {
        K::Dct => idct_16x16_internal_12bpc_pass2(m, dst, stride, c, eob, r6),
        K::Adst => iadst_16x16_internal_12bpc_pass2(m, dst, stride, c, eob, r6),
        K::FlipAdst => iflipadst_16x16_internal_12bpc_pass2(m, dst, stride, c, eob, r6),
        K::Identity => iidentity_16x16_internal_12bpc_pass2(m, dst, stride, c, eob, rsp, r6),
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Dct,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_identity_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Dct,
    K::Identity
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_adst_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Dct,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_flipadst_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Dct,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_dct_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Adst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_adst_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Adst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_adst_flipadst_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Adst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_dct_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::FlipAdst,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_adst_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::FlipAdst,
    K::Adst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_flipadst_flipadst_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::FlipAdst,
    K::FlipAdst
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_dct_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Identity,
    K::Dct
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x16_12bpc_avx2,
    inv_txfm_add_16x16_12bpc,
    K::Identity,
    K::Identity
);

/// `.pass2` of `idct_16x16_internal_12bpc` (to the `RET` of `iadst_16x16_internal_12bpc`'s `.end`)
#[inline(always)]
unsafe fn idct_16x16_internal_12bpc_pass2(
    m: &mut M,
    dst: Dst,
    stride: isize,
    c: Coef,
    eob: c_int,
    mut r6: *mut __m256i,
) {
    for x in 8..16 {
        st5(c, 32 * x as isize, m[x]); // mova [cq+32* 8], m8 … mova [cq+32*15], m15
    }
    idct_16x16_internal_12bpc_pass2_main(m, c, eob, &mut r6); // call .pass2_main
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[1] = _mm256_packs_epi32(m[2], m[3]); // packssdw m1, m2, m3
    m[2] = _mm256_packs_epi32(m[4], m[5]); // packssdw m2, m4, m5
    m[3] = _mm256_packs_epi32(m[6], m[7]); // packssdw m3, m6, m7
    m[4] = _mm256_packs_epi32(m[8], m[9]); // packssdw m4, m8, m9
    m[5] = _mm256_packs_epi32(m[10], m[11]); // packssdw m5, m10, m11
    m[6] = _mm256_packs_epi32(m[12], m[13]); // packssdw m6, m12, m13
    m[7] = _mm256_packs_epi32(m[14], m[15]); // packssdw m7, m14, m15
    st5(r6, -32 * 4, m[0]); // mova [r6-32*4], m0
    st5(r6, -32 * 3, m[1]); // mova [r6-32*3], m1
    st5(r6, -32 * 2, m[2]); // mova [r6-32*2], m2
    st5(r6, -32, m[3]); // mova [r6-32*1], m3
    st5(r6, 0, m[4]); // mova [r6+32*0], m4
    st5(r6, 32, m[5]); // mova [r6+32*1], m5
    st5(r6, 32 * 2, m[6]); // mova [r6+32*2], m6
    st5(r6, 32 * 3, m[7]); // mova [r6+32*3], m7
    for x in 0..8 {
        m[x] = ld5(c, 32 * (x as isize + 8)); // mova m0, [cq+32* 8] … mova m7, [cq+32*15]
    }
    let r5 = r6; // mov r5, r6
    r6 = at(r6, 32 * 16); // add r6, 32*16
    idct_16x16_internal_12bpc_pass2_main(m, c, eob, &mut r6); // call .pass2_main
    iadst_16x16_internal_12bpc_end(m, dst, stride, c, r5) // jmp m(iadst_16x16_internal_12bpc).end
}

/// `.write_16x16` of `idct_16x16_internal_12bpc`: as the 10 bpc one with `pw_16384` and `pixel_12bpc_max`
#[inline(always)]
unsafe fn idct_16x16_internal_12bpc_write_16x16(
    m: &mut M,
    dst: &mut Dst,
    stride: isize,
    c: &mut Coef,
    rsp: *mut __m256i,
) {
    let mut r3: isize = 0; // r3: set by .write_16x4_start (lea r3, [strideq*3])
    st5(rsp, 0, m[8]); // mova [rsp+gprsize+32*0], m8
    st5(rsp, 32, m[9]); // mova [rsp+gprsize+32*1], m9
    st5(rsp, 32 * 2, m[12]); // mova [rsp+gprsize+32*2], m12
    m[12] = pw(16384); // vpbroadcastd m12, [pw_16384]
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    m[2] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m2, m12
    m[3] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m3, m12
    idct_16x8_internal_12bpc_write_16x4_start(m, stride, &mut r3); // call m(idct_16x8_internal_12bpc).write_16x4_start
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    idct_16x16_internal_10bpc_write_16x16_2(m, dst, stride, c, rsp, r3) // jmp m(idct_16x16_internal_10bpc).write_16x16_2
}

/// `.pass2_main` of `idct_16x16_internal_12bpc`: the 16-point DCT of eight columns, the left half's rows in
/// `m0`-`m7`, the right half's in the frame slots below `r6` (moved down 32*8 on the slow path); uses
/// `[cq+32*0..7]` as scratch.
#[inline(always)]
unsafe fn idct_16x16_internal_12bpc_pass2_main(m: &mut M, c: Coef, eob: c_int, r6: &mut *mut __m256i) {
    idct_8x8_internal_12bpc_transpose_8x8(m); // call m(idct_8x8_internal_12bpc).transpose_8x8
    st5(c, 0, m[0]); // mova [cq+32* 0], m0
    st5(c, 32, m[2]); // mova [cq+32* 1], m2
    st5(c, 32 * 2, m[4]); // mova [cq+32* 2], m4
    st5(c, 32 * 3, m[6]); // mova [cq+32* 3], m6
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[0] = _mm256_max_epi32(m[12], m[1]); // pmaxsd m0, m12, m1
    m[1] = _mm256_max_epi32(m[12], m[3]); // pmaxsd m1, m12, m3
    m[2] = _mm256_max_epi32(m[12], m[5]); // pmaxsd m2, m12, m5
    m[3] = _mm256_max_epi32(m[12], m[7]); // pmaxsd m3, m12, m7
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
    }
    if eob >= 0 {
        // test eobd, eobd / jge .pass2_slow
        // .pass2_slow:
        *r6 = at(*r6, -32 * 8); // sub r6, 32*8
        m[8] = ld5(*r6, -32 * 4); // mova m8, [r6-32*4]
        m[4] = ld5(*r6, -32 * 3); // mova m4, [r6-32*3]
        m[10] = ld5(*r6, -32 * 2); // mova m10, [r6-32*2]
        m[5] = ld5(*r6, -32); // mova m5, [r6-32*1]
        m[12] = ld5(*r6, 0); // mova m12, [r6+32*0]
        m[6] = ld5(*r6, 32); // mova m6, [r6+32*1]
        m[14] = ld5(*r6, 32 * 2); // mova m14, [r6+32*2]
        m[7] = ld5(*r6, 32 * 3); // mova m7, [r6+32*3]
        transpose_8x8_dword(m, [8, 4, 10, 5, 12, 6, 14, 7], [9, 11, 13, 15]); // TRANSPOSE_8X8_DWORD 8, 4, 10, 5, 12, 6, 14, 7, 9, 11, 13, 15
        st5(c, 32 * 4, m[8]); // mova [cq+32* 4], m8
        st5(c, 32 * 5, m[10]); // mova [cq+32* 5], m10
        st5(c, 32 * 6, m[12]); // mova [cq+32* 6], m12
        st5(c, 32 * 7, m[14]); // mova [cq+32* 7], m14
        m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
        m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
        for x in [4, 5, 6, 7] {
            m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m4, m5, m6, m7
        }
        for x in [4, 5, 6, 7] {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m4, m5, m6, m7
        }
    } else {
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        for x in [5, 6, 7] {
            m[x] = m[4]; // REPX {mova x, m4}, m5, m6, m7
        }
        // jmp .pass2_fast
    }
    // .pass2_fast:
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    idct_8x16_internal_10bpc_main_oddhalf(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
    m[0] = _mm256_max_epi32(m[12], ld5(c, 0)); // pmaxsd m0, m12, [cq+32* 0]
    m[1] = _mm256_max_epi32(m[12], ld5(c, 32)); // pmaxsd m1, m12, [cq+32* 1]
    m[2] = _mm256_max_epi32(m[12], ld5(c, 32 * 2)); // pmaxsd m2, m12, [cq+32* 2]
    m[3] = _mm256_max_epi32(m[12], ld5(c, 32 * 3)); // pmaxsd m3, m12, [cq+32* 3]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
    }
    if eob >= 0 {
        // test eobd, eobd / jge .pass2_slow2
        // .pass2_slow2:
        m[4] = _mm256_max_epi32(m[12], ld5(c, 32 * 4)); // pmaxsd m4, m12, [cq+32* 4]
        m[5] = _mm256_max_epi32(m[12], ld5(c, 32 * 5)); // pmaxsd m5, m12, [cq+32* 5]
        m[6] = _mm256_max_epi32(m[12], ld5(c, 32 * 6)); // pmaxsd m6, m12, [cq+32* 6]
        m[7] = _mm256_max_epi32(m[12], ld5(c, 32 * 7)); // pmaxsd m7, m12, [cq+32* 7]
        for x in [4, 5, 6, 7] {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m4, m5, m6, m7
        }
    } else {
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        for x in [5, 6, 7] {
            m[x] = m[4]; // REPX {mova x, m4}, m5, m6, m7
        }
        // jmp .pass2_fast2
    }
    // .pass2_fast2:
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[11] = _mm256_srai_epi32::<8>(m[11]); // psrad m11, 8 ; pd_8
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    idct_16x8_internal_10bpc_pass1_rotations(m, *r6); // call m(idct_16x8_internal_10bpc).pass1_rotations
    for x in 0..16 {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4}, m0, m1, …, m15
    }
}

/// `.pass2` of `iadst_16x16_internal_12bpc` (to its `RET`, through `.pass2_part3` and `.end`)
#[inline(always)]
unsafe fn iadst_16x16_internal_12bpc_pass2(
    m: &mut M,
    dst: Dst,
    stride: isize,
    c: Coef,
    eob: c_int,
    mut r6: *mut __m256i,
) {
    iadst_16x16_internal_12bpc_pass2_part1(m, c, eob, &mut r6); // call .pass2_part1
    iadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(iadst_16x8_internal_10bpc).pass1_rotations
    let mut r5 = r6;
    iadst_16x16_internal_12bpc_pass2_part2(m, c, eob, &mut r6, &mut r5); // call .pass2_part2
    iadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(iadst_16x8_internal_10bpc).pass1_rotations
    iadst_16x16_internal_12bpc_pass2_part3(m, dst, stride, c, r5)
}

/// `.pass2_part3` of `iadst_16x16_internal_12bpc` (to its `RET`, through `.end`)
#[inline(always)]
unsafe fn iadst_16x16_internal_12bpc_pass2_part3(m: &mut M, dst: Dst, stride: isize, c: Coef, r5: *mut __m256i) {
    for x in [0, 1, 2, 3, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4 }, m0, m1, m2, m3, m12, m13, m14, m15
    }
    for x in [4, 5, 6, 7, 8, 9, 10, 11] {
        m[x] = _mm256_srai_epi32::<15>(m[x]); // REPX {psrad x, 15}, m4, m5, m6, m7, m8, m9, m10, m11
    }
    iadst_16x16_internal_12bpc_end(m, dst, stride, c, r5)
}

/// `.end` of `iadst_16x16_internal_12bpc` (to its `RET`): packs the right half (`m0`-`m15`) to words, blends it
/// with the left half's words in the frame slots at `r5` and writes the block.
#[inline(always)]
unsafe fn iadst_16x16_internal_12bpc_end(m: &mut M, mut dst: Dst, stride: isize, mut c: Coef, r5: *mut __m256i) {
    let mut r3: isize = 0; // r3: set by .write_16x4_start (lea r3, [strideq*3])
    m[15] = _mm256_packs_epi32(m[15], m[14]); // packssdw m15, m14
    m[14] = _mm256_packs_epi32(m[13], m[12]); // packssdw m14, m13, m12
    m[13] = _mm256_packs_epi32(m[11], m[10]); // packssdw m13, m11, m10
    m[12] = _mm256_packs_epi32(m[9], m[8]); // packssdw m12, m9, m8
    m[11] = _mm256_packs_epi32(m[7], m[6]); // packssdw m11, m7, m6
    m[10] = _mm256_packs_epi32(m[5], m[4]); // packssdw m10, m5, m4
    m[7] = _mm256_packs_epi32(m[3], m[2]); // packssdw m7, m3, m2
    m[6] = _mm256_packs_epi32(m[1], m[0]); // packssdw m6, m1, m0
    m[0] = _mm256_blend_epi32::<0x33>(m[6], ld5(r5, -32 * 4)); // vpblendd m0, m6, [r5-32*4], 0x33
    m[1] = _mm256_blend_epi32::<0xcc>(m[6], ld5(r5, -32 * 4)); // vpblendd m1, m6, [r5-32*4], 0xcc
    m[2] = _mm256_blend_epi32::<0x33>(m[7], ld5(r5, -32 * 3)); // vpblendd m2, m7, [r5-32*3], 0x33
    m[3] = _mm256_blend_epi32::<0xcc>(m[7], ld5(r5, -32 * 3)); // vpblendd m3, m7, [r5-32*3], 0xcc
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m3, m3, q2031
    idct_16x8_internal_12bpc_write_16x4_start(m, stride, &mut r3); // call m(idct_16x8_internal_12bpc).write_16x4_start
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_blend_epi32::<0x33>(m[10], ld5(r5, -32 * 2)); // vpblendd m0, m10, [r5-32*2], 0x33
    m[1] = _mm256_blend_epi32::<0xcc>(m[10], ld5(r5, -32 * 2)); // vpblendd m1, m10, [r5-32*2], 0xcc
    m[2] = _mm256_blend_epi32::<0x33>(m[11], ld5(r5, -32)); // vpblendd m2, m11, [r5-32*1], 0x33
    m[3] = _mm256_blend_epi32::<0xcc>(m[11], ld5(r5, -32)); // vpblendd m3, m11, [r5-32*1], 0xcc
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m3, m3, q2031
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_blend_epi32::<0x33>(m[12], ld5(r5, 0)); // vpblendd m0, m12, [r5+32*0], 0x33
    m[1] = _mm256_blend_epi32::<0xcc>(m[12], ld5(r5, 0)); // vpblendd m1, m12, [r5+32*0], 0xcc
    m[2] = _mm256_blend_epi32::<0x33>(m[13], ld5(r5, 32)); // vpblendd m2, m13, [r5+32*1], 0x33
    m[3] = _mm256_blend_epi32::<0xcc>(m[13], ld5(r5, 32)); // vpblendd m3, m13, [r5+32*1], 0xcc
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m3, m3, q2031
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3); // call m(idct_16x8_internal_10bpc).write_16x4_zero
    m[0] = _mm256_blend_epi32::<0x33>(m[14], ld5(r5, 32 * 2)); // vpblendd m0, m14, [r5+32*2], 0x33
    m[1] = _mm256_blend_epi32::<0xcc>(m[14], ld5(r5, 32 * 2)); // vpblendd m1, m14, [r5+32*2], 0xcc
    m[2] = _mm256_blend_epi32::<0x33>(m[15], ld5(r5, 32 * 3)); // vpblendd m2, m15, [r5+32*3], 0x33
    m[3] = _mm256_blend_epi32::<0xcc>(m[15], ld5(r5, 32 * 3)); // vpblendd m3, m15, [r5+32*3], 0xcc
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
    m[3] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m3, m3, q2031
    idct_16x8_internal_10bpc_write_16x4_zero(m, &mut dst, stride, &mut c, r3);
    // call m(idct_16x8_internal_10bpc).write_16x4_zero
    // RET
}

/// `.pass2_part1` of `iadst_16x16_internal_12bpc` (falls into `.pass2_main`): keeps the second half's rows
/// `m8`-`m15` in `[cq+32*8..15]`.
#[inline(always)]
unsafe fn iadst_16x16_internal_12bpc_pass2_part1(m: &mut M, c: Coef, eob: c_int, r6: &mut *mut __m256i) {
    for x in 8..16 {
        st5(c, 32 * x as isize, m[x]); // mova [cq+32* 8], m8 … mova [cq+32*15], m15
    }
    iadst_16x16_internal_12bpc_pass2_main(m, c, eob, r6)
}

/// `.pass2_main` of `iadst_16x16_internal_12bpc`: the 16-point ADST of eight columns (as the DCT's
/// `.pass2_main`), up to the rounding constants of `iadst_16x8_internal_10bpc`'s `.pass1_rotations`.
#[inline(always)]
unsafe fn iadst_16x16_internal_12bpc_pass2_main(m: &mut M, c: Coef, eob: c_int, r6: &mut *mut __m256i) {
    idct_8x8_internal_12bpc_transpose_8x8(m); // call m(idct_8x8_internal_12bpc).transpose_8x8
    st5(c, 0, m[0]); // mova [cq+32* 0], m0
    st5(c, 32, m[3]); // mova [cq+32* 1], m3
    st5(c, 32 * 2, m[4]); // mova [cq+32* 2], m4
    st5(c, 32 * 3, m[7]); // mova [cq+32* 3], m7
    m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
    m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
    m[0] = _mm256_max_epi32(m[13], m[2]); // pmaxsd m0, m13, m2
    m[2] = _mm256_max_epi32(m[13], m[6]); // pmaxsd m2, m13, m6
    m[5] = _mm256_max_epi32(m[13], m[5]); // pmaxsd m5, m13, m5
    m[7] = _mm256_max_epi32(m[13], m[1]); // pmaxsd m7, m13, m1
    for x in [0, 2, 5, 7] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m0, m2, m5, m7
    }
    if eob >= 0 {
        // test eobd, eobd / jge .pass2_slow
        // .pass2_slow:
        *r6 = at(*r6, -32 * 8); // sub r6, 32*8
        m[8] = ld5(*r6, -32 * 4); // mova m8, [r6-32*4]
        m[3] = ld5(*r6, -32 * 3); // mova m3, [r6-32*3]
        m[4] = ld5(*r6, -32 * 2); // mova m4, [r6-32*2]
        m[11] = ld5(*r6, -32); // mova m11, [r6-32*1]
        m[12] = ld5(*r6, 0); // mova m12, [r6+32*0]
        m[1] = ld5(*r6, 32); // mova m1, [r6+32*1]
        m[6] = ld5(*r6, 32 * 2); // mova m6, [r6+32*2]
        m[15] = ld5(*r6, 32 * 3); // mova m15, [r6+32*3]
        transpose_8x8_dword(m, [8, 3, 4, 11, 12, 1, 6, 15], [13, 9, 10, 14]); // TRANSPOSE_8X8_DWORD 8, 3, 4, 11, 12, 1, 6, 15, 13, 9, 10, 14
        st5(c, 32 * 4, m[8]); // mova [cq+32* 4], m8
        st5(c, 32 * 5, m[11]); // mova [cq+32* 5], m11
        st5(c, 32 * 6, m[12]); // mova [cq+32* 6], m12
        st5(c, 32 * 7, m[15]); // mova [cq+32* 7], m15
        m[13] = pd(-0x20000); // vpbroadcastd m13, [clip_18b_min]
        m[14] = pd(0x1ffff); // vpbroadcastd m14, [clip_18b_max]
        for x in [1, 3, 4, 6] {
            m[x] = _mm256_max_epi32(m[x], m[13]); // REPX {pmaxsd x, m13}, m1, m3, m4, m6
        }
        for x in [1, 3, 4, 6] {
            m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m1, m3, m4, m6
        }
    } else {
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        for x in [3, 4, 6] {
            m[x] = m[1]; // REPX {mova x, m1}, m3, m4, m6
        }
        // jmp .pass2_fast
    }
    // .pass2_fast:
    m[12] = pd(2048); // vpbroadcastd m12, [pd_2048]
    m[15] = pd(2896); // vpbroadcastd m15, [pd_2896]
    iadst_16x8_internal_10bpc_main_part1(m, *r6); // call m(iadst_16x8_internal_10bpc).main_part1
    m[0] = _mm256_max_epi32(m[13], ld5(c, 0)); // pmaxsd m0, m13, [cq+32* 0] ;  0
    m[7] = _mm256_max_epi32(m[13], ld5(c, 32)); // pmaxsd m7, m13, [cq+32* 1] ;  3
    m[2] = _mm256_max_epi32(m[13], ld5(c, 32 * 2)); // pmaxsd m2, m13, [cq+32* 2] ;  4
    m[5] = _mm256_max_epi32(m[13], ld5(c, 32 * 3)); // pmaxsd m5, m13, [cq+32* 3] ;  7
    for x in [0, 2, 5, 7] {
        m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m0, m2, m5, m7
    }
    if eob >= 0 {
        // test eobd, eobd / jge .pass2_slow2
        // .pass2_slow2:
        m[4] = _mm256_max_epi32(m[13], ld5(c, 32 * 4)); // pmaxsd m4, m13, [cq+32* 4] ;  8
        m[3] = _mm256_max_epi32(m[13], ld5(c, 32 * 5)); // pmaxsd m3, m13, [cq+32* 5] ; 11
        m[6] = _mm256_max_epi32(m[13], ld5(c, 32 * 6)); // pmaxsd m6, m13, [cq+32* 6] ; 12
        m[1] = _mm256_max_epi32(m[13], ld5(c, 32 * 7)); // pmaxsd m1, m13, [cq+32* 7] ; 15
        for x in [1, 3, 4, 6] {
            m[x] = _mm256_min_epi32(m[x], m[14]); // REPX {pminsd x, m14}, m1, m3, m4, m6
        }
    } else {
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        for x in [3, 4, 6] {
            m[x] = m[1]; // REPX {mova x, m1}, m3, m4, m6
        }
        // jmp .pass2_fast2
    }
    // .pass2_fast2:
    iadst_16x8_internal_10bpc_main_part2(m, *r6); // call m(iadst_16x8_internal_10bpc).main_part2
    m[14] = pd(17408); // vpbroadcastd m14, [pd_17408]
    m[15] = _mm256_srli_epi32::<11>(m[15]); // psrld m15, 11 ; pd_1
    m[13] = _mm256_sub_epi32(m[14], m[15]); // psubd m13, m14, m15 ; pd_17407
    m[15] = _mm256_slli_epi32::<3>(m[15]); // pslld m15, 3 ; pd_8
}

/// `.pass2_part2` of `iadst_16x16_internal_12bpc` (into `.pass2_main`): rounds and packs the first eight rows to
/// the frame slots at `r6`, which become `r5`, and runs the second eight rows from `[cq+32*8..15]`.
#[inline(always)]
unsafe fn iadst_16x16_internal_12bpc_pass2_part2(
    m: &mut M,
    c: Coef,
    eob: c_int,
    r6: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    for x in [0, 1, 2, 3, 12, 13, 14, 15] {
        m[x] = _mm256_srai_epi32::<4>(m[x]); // REPX {psrad x, 4 }, m0, m1, m2, m3, m12, m13, m14, m15
    }
    for x in [4, 5, 6, 7, 8, 9, 10, 11] {
        m[x] = _mm256_srai_epi32::<15>(m[x]); // REPX {psrad x, 15}, m4, m5, m6, m7, m8, m9, m10, m11
    }
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[1] = _mm256_packs_epi32(m[2], m[3]); // packssdw m1, m2, m3
    m[2] = _mm256_packs_epi32(m[4], m[5]); // packssdw m2, m4, m5
    m[3] = _mm256_packs_epi32(m[6], m[7]); // packssdw m3, m6, m7
    m[4] = _mm256_packs_epi32(m[8], m[9]); // packssdw m4, m8, m9
    m[5] = _mm256_packs_epi32(m[10], m[11]); // packssdw m5, m10, m11
    m[6] = _mm256_packs_epi32(m[12], m[13]); // packssdw m6, m12, m13
    m[7] = _mm256_packs_epi32(m[14], m[15]); // packssdw m7, m14, m15
    st5(*r6, -32 * 4, m[0]); // mova [r6-32*4], m0
    st5(*r6, -32 * 3, m[1]); // mova [r6-32*3], m1
    st5(*r6, -32 * 2, m[2]); // mova [r6-32*2], m2
    st5(*r6, -32, m[3]); // mova [r6-32*1], m3
    st5(*r6, 0, m[4]); // mova [r6+32*0], m4
    st5(*r6, 32, m[5]); // mova [r6+32*1], m5
    st5(*r6, 32 * 2, m[6]); // mova [r6+32*2], m6
    st5(*r6, 32 * 3, m[7]); // mova [r6+32*3], m7
    for x in 0..8 {
        m[x] = ld5(c, 32 * (x as isize + 8)); // mova m0, [cq+32* 8] … mova m7, [cq+32*15]
    }
    *r5 = *r6; // mov r5, r6
    *r6 = at(*r6, 32 * 16); // add r6, 32*16
    iadst_16x16_internal_12bpc_pass2_main(m, c, eob, r6) // jmp .pass2_main
}

/// `.pass2` of `iflipadst_16x16_internal_12bpc` (to the `RET` of `iadst_16x16_internal_12bpc`)
#[inline(always)]
unsafe fn iflipadst_16x16_internal_12bpc_pass2(
    m: &mut M,
    dst: Dst,
    stride: isize,
    c: Coef,
    eob: c_int,
    mut r6: *mut __m256i,
) {
    iadst_16x16_internal_12bpc_pass2_part1(m, c, eob, &mut r6); // call m(iadst_16x16_internal_12bpc).pass2_part1
    iflipadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(iflipadst_16x8_internal_10bpc).pass1_rotations
    let mut r5 = r6;
    iadst_16x16_internal_12bpc_pass2_part2(m, c, eob, &mut r6, &mut r5); // call m(iadst_16x16_internal_12bpc).pass2_part2
    iflipadst_16x8_internal_10bpc_pass1_rotations(m, r6); // call m(iflipadst_16x8_internal_10bpc).pass1_rotations
    iadst_16x16_internal_12bpc_pass2_part3(m, dst, stride, c, r5) // jmp m(iadst_16x16_internal_12bpc).pass2_part3
}

/// `%macro IDTX16_12BPC src` (`m7` = pd_1697, `m15` = pd_5120; `m6` is the temporary)
#[inline(always)]
unsafe fn idtx16_12bpc(m: &mut M, src: usize) {
    m[6] = _mm256_mullo_epi32(m[7], m[src]); // pmulld m6, m7, m%1
    m[6] = _mm256_add_epi32(m[6], m[15]); // paddd m6, m15
    m[6] = _mm256_srai_epi32::<12>(m[6]); // psrad m6, 12
    m[6] = _mm256_add_epi32(m[6], m[src]); // paddd m6, m%1
    m[src] = _mm256_srai_epi32::<1>(m[6]); // psrad m%1, m6, 1
}

/// `cglobal iidentity_16x16_internal_12bpc` up to `jmp tx2q`
#[inline(always)]
unsafe fn iidentity_16x16_internal_12bpc_pass1(
    m: &mut M,
    c: Coef,
    eob: &mut c_int,
    rsp: *mut __m256i,
    r6: &mut *mut __m256i,
) {
    m[7] = pd(1697); // vpbroadcastd m7, [pd_1697]
    m[15] = pd(5120); // vpbroadcastd m15, [pd_5120]
    *r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    *eob = eob.wrapping_sub(36); // sub eobd, 36
    if *eob >= 0 {
        // jl .fast
        let mut r3: isize = -32 * 8 * 4; // mov r3, -32*8*4
        loop {
            // .righthalf:
            m[10] = ld5(c, r3 + 32 * 33); // mova m10, [cq+r3+32*33]
            m[11] = ld5(c, r3 + 32 * 35); // mova m11, [cq+r3+32*35]
            m[12] = ld5(c, r3 + 32 * 37); // mova m12, [cq+r3+32*37]
            m[13] = ld5(c, r3 + 32 * 39); // mova m13, [cq+r3+32*39]
            *r6 = at(*r6, 32 * 4); // add r6, 32*4
            m[0] = _mm256_mullo_epi32(m[7], m[10]); // pmulld m0, m7, m10
            m[1] = _mm256_mullo_epi32(m[7], m[11]); // pmulld m1, m7, m11
            m[2] = _mm256_mullo_epi32(m[7], m[12]); // pmulld m2, m7, m12
            m[3] = _mm256_mullo_epi32(m[7], m[13]); // pmulld m3, m7, m13
            for x in [0, 1, 2, 3] {
                m[x] = _mm256_add_epi32(m[x], m[15]); // REPX {paddd x, m15}, m0, m1, m2, m3
            }
            for x in [0, 1, 2, 3] {
                m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3
            }
            m[0] = _mm256_add_epi32(m[0], m[10]); // paddd m0, m10
            m[1] = _mm256_add_epi32(m[1], m[11]); // paddd m1, m11
            m[2] = _mm256_add_epi32(m[2], m[12]); // paddd m2, m12
            m[3] = _mm256_add_epi32(m[3], m[13]); // paddd m3, m13
            for x in [0, 1, 2, 3] {
                m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1  }, m0, m1, m2, m3
            }
            st5(*r6, 0, m[0]); // mova [r6+32*0], m0
            st5(*r6, 32, m[1]); // mova [r6+32*1], m1
            st5(*r6, 32 * 2, m[2]); // mova [r6+32*2], m2
            st5(*r6, 32 * 3, m[3]); // mova [r6+32*3], m3
            r3 += 32 * 8; // add r3, 32*8
            if r3 >= 0 {
                break; // jl .righthalf
            }
        }
    }
    // .fast:
    m[0] = ld5(c, 0); // mova m0, [cq+64* 0]
    m[1] = ld5(c, 64 * 1); // mova m1, [cq+64* 1]
    m[2] = ld5(c, 64 * 2); // mova m2, [cq+64* 2]
    m[3] = ld5(c, 64 * 3); // mova m3, [cq+64* 3]
    m[4] = ld5(c, 64 * 4); // mova m4, [cq+64* 4]
    m[5] = ld5(c, 64 * 5); // mova m5, [cq+64* 5]
    m[8] = ld5(c, 64 * 6); // mova m8, [cq+64* 6]
    m[9] = ld5(c, 64 * 7); // mova m9, [cq+64* 7]
    for x in [0, 1, 2, 3, 4, 5, 8, 9] {
        idtx16_12bpc(m, x); // REPX {IDTX16_12BPC x}, 0, 1, 2, 3, 4, 5, 8, 9
    }
    st5(c, 0, m[8]); // mova [cq+64*0], m8
    st5(c, 64, m[9]); // mova [cq+64*1], m9
    m[8] = ld5(c, 64 * 8); // mova m8, [cq+64* 8]
    m[9] = ld5(c, 64 * 9); // mova m9, [cq+64* 9]
    m[10] = ld5(c, 64 * 10); // mova m10, [cq+64*10]
    m[11] = ld5(c, 64 * 11); // mova m11, [cq+64*11]
    m[12] = ld5(c, 64 * 12); // mova m12, [cq+64*12]
    m[13] = ld5(c, 64 * 13); // mova m13, [cq+64*13]
    m[14] = ld5(c, 64 * 14); // mova m14, [cq+64*14]
    for x in [8, 9, 10, 11, 12, 13, 14] {
        idtx16_12bpc(m, x); // REPX {IDTX16_12BPC x}, 8, 9, 10, 11, 12, 13, 14
    }
    m[6] = ld5(c, 64 * 15); // mova m6, [cq+64*15]
    m[7] = _mm256_mullo_epi32(m[7], m[6]); // pmulld m7, m6
    m[7] = _mm256_add_epi32(m[7], m[15]); // paddd m7, m15
    m[7] = _mm256_srai_epi32::<12>(m[7]); // psrad m7, 12
    m[7] = _mm256_add_epi32(m[7], m[6]); // paddd m7, m6
    m[6] = ld5(c, 0); // mova m6, [cq+64*0]
    m[15] = _mm256_srai_epi32::<1>(m[7]); // psrad m15, m7, 1
    m[7] = ld5(c, 64); // mova m7, [cq+64*1]
                       // jmp tx2q
}

/// `.pass2` of `iidentity_16x16_internal_12bpc` (to its `RET`)
#[inline(always)]
unsafe fn iidentity_16x16_internal_12bpc_pass2(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    eob: c_int,
    rsp: *mut __m256i,
    mut r6: *mut __m256i,
) {
    iidentity_8x16_internal_12bpc_pass2_main(m, c); // call m(iidentity_8x16_internal_12bpc).pass2_main
    idct_16x16_internal_10bpc_transpose_fast(m); // call m(idct_16x16_internal_10bpc).transpose_fast
    if eob >= 0 {
        // test eobd, eobd / jl .pass2_fast
        for x in 0..8 {
            st5(c, 32 * (x as isize + 8), m[x]); // mova [cq+32* 8], m0 … mova [cq+32*15], m7
        }
        m[8] = ld5(r6, -32 * 4); // mova m8, [r6-32*4]
        m[9] = ld5(r6, -32 * 3); // mova m9, [r6-32*3]
        m[10] = ld5(r6, -32 * 2); // mova m10, [r6-32*2]
        m[11] = ld5(r6, -32); // mova m11, [r6-32*1]
        m[12] = ld5(r6, 0); // mova m12, [r6+32*0]
        m[13] = ld5(r6, 32); // mova m13, [r6+32*1]
        m[14] = ld5(r6, 32 * 2); // mova m14, [r6+32*2]
        m[15] = ld5(r6, 32 * 3); // mova m15, [r6+32*3]
        r6 = at(r6, -32 * 8); // sub r6, 32*8
        m[0] = ld5(r6, -32 * 4); // mova m0, [r6-32*4]
        m[1] = ld5(r6, -32 * 3); // mova m1, [r6-32*3]
        m[2] = ld5(r6, -32 * 2); // mova m2, [r6-32*2]
        m[3] = ld5(r6, -32); // mova m3, [r6-32*1]
        m[4] = ld5(r6, 0); // mova m4, [r6+32*0]
        m[5] = ld5(r6, 32); // mova m5, [r6+32*1]
        m[6] = ld5(r6, 32 * 2); // mova m6, [r6+32*2]
        m[7] = ld5(r6, 32 * 3); // mova m7, [r6+32*3]
        iidentity_8x16_internal_12bpc_pass2_main(m, c); // call m(iidentity_8x16_internal_12bpc).pass2_main
        idct_16x8_internal_10bpc_transpose2(m); // call m(idct_16x8_internal_10bpc).transpose2
        for x in 0..8 {
            m[x + 8] = m[x]; // mova m8, m0 … mova m15, m7
        }
        for x in 0..8 {
            m[x] = ld5(c, 32 * (x as isize + 8)); // mova m0, [cq+32* 8] … mova m7, [cq+32*15]
        }
    }
    // .pass2_fast:
    idct_16x16_internal_12bpc_write_16x16(m, &mut dst, stride, &mut c, rsp); // call m(idct_16x16_internal_12bpc).write_16x16
                                                                             // RET
}

// --- part 6: itx16_avx2.asm lines 5618-6552 (8x32, 32x8) ---

/// `[p+off]` of a frame (`p` a 32-byte slot pointer, `off` in bytes as in the assembly)
#[inline(always)]
unsafe fn ld_p6(p: *mut __m256i, off: isize) -> __m256i {
    _mm256_loadu_si256(at(p, off))
}

/// `mova [p+off], v` of a frame
#[inline(always)]
unsafe fn st_p6(p: *mut __m256i, off: isize, v: __m256i) {
    _mm256_storeu_si256(at(p, off), v)
}

/// `[cq+off]` (32 bytes)
#[inline(always)]
unsafe fn ldc_p6(c: Coef, off: isize) -> __m256i {
    _mm256_loadu_si256(at(c, off).cast())
}

/// `mova [cq+off], v`
#[inline(always)]
unsafe fn stc_p6(c: Coef, off: isize, v: __m256i) {
    _mm256_storeu_si256(at(c, off).cast(), v)
}

// --- the exported subroutines (parts 7 and 8 use them) ----------------------------------------------------------

/// `%macro IDCT32_END in/out1, out2, tmp[1-3], shift, pack`: the last butterflies of the 32-point DCT of row
/// `a`: `m<a>` holds idct16 out`a` on entry, the frames `[r6]` (idct16 odd half), `[r5]` (t16 … t23 side) and
/// `[r4]` the rest; `m11` the rounding, `m12`/`m13` the clips. Out: `m<a>` = out`a`, `m<t1>` = out`16+a`,
/// `m<b>` = out`15-a`, `m<t2>` = out`31-a` (shifted by `SHIFT`), packed into `m<a>` and `m<b>` if `pack`.
#[inline(always)]
unsafe fn idct32_end<const SHIFT: i32>(
    m: &mut M,
    r6: *mut __m256i,
    r5: *mut __m256i,
    r4: *mut __m256i,
    a: usize,
    b: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    pack: bool,
) {
    m[t2] = ld_p6(r6, 32 * (a as isize - 4)); // mova m%4, [r6+32*(%1-4)]
    m[b] = ld_p6(r5, 32 * (3 - a as isize)); // mova m%2, [r5+32*(3-%1)]
    m[t3] = ld_p6(r4, 32 * (a as isize - 4)); // mova m%5, [r4+32*(%1-4)]
    m[t1] = _mm256_sub_epi32(m[a], m[t2]); // psubd m%3, m%1, m%4 ; idct16 out15 - n
    m[a] = _mm256_add_epi32(m[a], m[t2]); // paddd m%1, m%4 ; idct16 out0  + n
    m[a] = _mm256_max_epi32(m[a], m[12]); // pmaxsd m%1, m12
    m[t1] = _mm256_max_epi32(m[t1], m[12]); // pmaxsd m%3, m12
    m[a] = _mm256_min_epi32(m[a], m[13]); // pminsd m%1, m13
    m[t1] = _mm256_min_epi32(m[t1], m[13]); // pminsd m%3, m13
    m[a] = _mm256_add_epi32(m[a], m[11]); // paddd m%1, m11
    m[t1] = _mm256_add_epi32(m[t1], m[11]); // paddd m%3, m11
    m[t2] = _mm256_sub_epi32(m[a], m[b]); // psubd m%4, m%1, m%2 ; out31 - n
    m[a] = _mm256_add_epi32(m[a], m[b]); // paddd m%1, m%2 ; out0  + n
    m[b] = _mm256_add_epi32(m[t1], m[t3]); // paddd m%2, m%3, m%5 ; out15 - n
    m[t1] = _mm256_sub_epi32(m[t1], m[t3]); // psubd m%3, m%5 ; out16 + n
    for x in [a, t1, b, t2] {
        m[x] = _mm256_srai_epi32::<SHIFT>(m[x]); // REPX {psrad x, %6}, m%1, m%3, m%2, m%4
    }
    if pack {
        m[a] = _mm256_packs_epi32(m[a], m[t1]); // packssdw m%1, m%3 ; out0  + n, out16 + n
        m[b] = _mm256_packs_epi32(m[b], m[t2]); // packssdw m%2, m%4 ; out15 - n, out31 - n
    }
}

/// `.main_oddhalf_part1_fast_rect2` of `inv_txfm_add_dct_dct_8x32_10bpc`: `m0`-`m3` = in1, in7, in9, in15
/// (lower half zero), scaled by 2896 (`m14`) by the caller; rounds them (`m11` = pd_2048), then as `_fast`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast_rect2(m: &mut M, r6: *mut __m256i) {
    for x in 0..4 {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3
    }
    for x in 0..4 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast(m, r6)
}

/// `.main_oddhalf_part1_fast` of `inv_txfm_add_dct_dct_8x32_10bpc`: in1, in7, in9, in15 in `m0`-`m3` (lower
/// half zero); `m11` = pd_2048, `m12`/`m13` the clips; writes t16a … t31a to `[r6-32*4]` … `[r6+32*3]`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast(m: &mut M, r6: *mut __m256i) {
    m[7] = pd(4091); // vpbroadcastd m7, [pd_4091]
    m[8] = pd(201); // vpbroadcastd m8, [pd_201]
    m[6] = pd(-1380); // vpbroadcastd m6, [pd_m1380]
    m[9] = pd(3857); // vpbroadcastd m9, [pd_3857]
    m[5] = pd(3703); // vpbroadcastd m5, [pd_3703]
    m[10] = pd(1751); // vpbroadcastd m10, [pd_1751]
    m[4] = pd(-2751); // vpbroadcastd m4, [pd_m2751]
    m[15] = pd(3035); // vpbroadcastd m15, [pd_3035]
    m[7] = _mm256_mullo_epi32(m[7], m[0]); // pmulld m7, m0
    m[0] = _mm256_mullo_epi32(m[0], m[8]); // pmulld m0, m8
    m[6] = _mm256_mullo_epi32(m[6], m[1]); // pmulld m6, m1
    m[1] = _mm256_mullo_epi32(m[1], m[9]); // pmulld m1, m9
    m[5] = _mm256_mullo_epi32(m[5], m[2]); // pmulld m5, m2
    m[2] = _mm256_mullo_epi32(m[2], m[10]); // pmulld m2, m10
    m[4] = _mm256_mullo_epi32(m[4], m[3]); // pmulld m4, m3
    m[3] = _mm256_mullo_epi32(m[3], m[15]); // pmulld m3, m15
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast2(m, r6) // jmp .main_oddhalf_part1_fast2
}

/// `.main_oddhalf_part1_rect2` of `inv_txfm_add_dct_dct_8x32_10bpc`: rounds `m0`-`m7` (`m11`), then as `.main_oddhalf_part1`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_rect2(m: &mut M, r6: *mut __m256i) {
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1(m, r6)
}

/// `.main_oddhalf_part1` of `inv_txfm_add_dct_dct_8x32_10bpc`: in1, in7, in9, in15, in17, in23, in25, in31 in
/// `m0`-`m7`; `m11` = pd_2048, `m12`/`m13` the clips; writes eight odd-half terms to `[r6-32*4]` … `[r6+32*3]`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1(m: &mut M, r6: *mut __m256i) {
    itx_mulsub_2d(m, 0, 7, 8, 9, 10, None, 201, 4091, 0); // ITX_MULSUB_2D 0, 7, 8, 9, 10, _,  201, 4091 ; t16a, t31a
    itx_mulsub_2d(m, 6, 1, 8, 9, 10, None, 3857, 1380, 0); // ITX_MULSUB_2D 6, 1, 8, 9, 10, _, 3857, 1380 ; t19a, t28a
    itx_mulsub_2d(m, 2, 5, 8, 9, 10, None, 1751, 3703, 0); // ITX_MULSUB_2D 2, 5, 8, 9, 10, _, 1751, 3703 ; t18a, t29a
    itx_mulsub_2d(m, 4, 3, 8, 9, 10, None, 3035, 2751, 0); // ITX_MULSUB_2D 4, 3, 8, 9, 10, _, 3035, 2751 ; t17a, t30a
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast2(m, r6)
}

/// `.main_oddhalf_part1_fast2` of `inv_txfm_add_dct_dct_8x32_10bpc`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast2(m: &mut M, r6: *mut __m256i) {
    for x in [0, 7, 6, 1, 2, 5, 4, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m7, m6, m1, m2, m5, m4, m3
    }
    for x in [0, 4, 6, 2, 1, 5, 7, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m4, m6, m2, m1, m5, m7, m3
    }
    m[8] = _mm256_sub_epi32(m[0], m[4]); // psubd m8, m0, m4 ; t17
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; t16
    m[4] = _mm256_sub_epi32(m[6], m[2]); // psubd m4, m6, m2 ; t18
    m[6] = _mm256_add_epi32(m[6], m[2]); // paddd m6, m2 ; t19
    m[2] = _mm256_sub_epi32(m[1], m[5]); // psubd m2, m1, m5 ; t29
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; t28
    m[5] = _mm256_sub_epi32(m[7], m[3]); // psubd m5, m7, m3 ; t30
    m[7] = _mm256_add_epi32(m[7], m[3]); // paddd m7, m3 ; t31
    for x in [8, 5, 4, 2, 0, 6, 1, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m8, m5, m4, m2, m0, m6, m1, m7
    }
    for x in [8, 5, 4, 2, 0, 6, 1, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m8, m5, m4, m2, m0, m6, m1, m7
    }
    m[15] = pd(4017); // vpbroadcastd m15, [pd_4017]
    m[10] = pd(799); // vpbroadcastd m10, [pd_799]
    itx_mulsub_2d(m, 5, 8, 3, 9, usize::MAX, Some(11), 10, 15, 0); // ITX_MULSUB_2D 5, 8, 3, 9, _, 11, 10, 15 ; t17a, t30a
    itx_mulsub_2d(m, 2, 4, 3, 9, usize::MAX, Some(11), 10, 15, 2); // ITX_MULSUB_2D 2, 4, 3, 9, _, 11, 10, 15, 2 ; t29a, t18a
    m[3] = _mm256_sub_epi32(m[0], m[6]); // psubd m3, m0, m6 ; t19a
    m[0] = _mm256_add_epi32(m[0], m[6]); // paddd m0, m6 ; t16a
    m[6] = _mm256_sub_epi32(m[7], m[1]); // psubd m6, m7, m1 ; t28a
    m[7] = _mm256_add_epi32(m[7], m[1]); // paddd m7, m1 ; t31a
    m[1] = _mm256_sub_epi32(m[5], m[4]); // psubd m1, m5, m4 ; t18
    m[5] = _mm256_add_epi32(m[5], m[4]); // paddd m5, m4 ; t17
    m[4] = _mm256_sub_epi32(m[8], m[2]); // psubd m4, m8, m2 ; t29
    m[8] = _mm256_add_epi32(m[8], m[2]); // paddd m8, m2 ; t30
    for x in [3, 6, 1, 4, 0, 7, 5, 8] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m3, m6, m1, m4, m0, m7, m5, m8
    }
    for x in [3, 6, 1, 4, 0, 7, 5, 8] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m3, m6, m1, m4, m0, m7, m5, m8
    }
    m[15] = pd(3784); // vpbroadcastd m15, [pd_3784]
    m[10] = pd(1567); // vpbroadcastd m10, [pd_1567]
    itx_mulsub_2d(m, 4, 1, 2, 9, usize::MAX, Some(11), 10, 15, 0); // ITX_MULSUB_2D 4, 1, 2, 9, _, 11, 10, 15 ; t18a, t29a
    itx_mulsub_2d(m, 6, 3, 2, 9, usize::MAX, Some(11), 10, 15, 0); // ITX_MULSUB_2D 6, 3, 2, 9, _, 11, 10, 15 ; t19,  t28
    st_p6(r6, -32 * 4, m[0]); // mova [r6-32*4], m0
    st_p6(r6, -32 * 3, m[5]); // mova [r6-32*3], m5
    st_p6(r6, -32 * 2, m[4]); // mova [r6-32*2], m4
    st_p6(r6, -32 * 1, m[6]); // mova [r6-32*1], m6
    st_p6(r6, 32 * 0, m[3]); // mova [r6+32*0], m3
    st_p6(r6, 32 * 1, m[1]); // mova [r6+32*1], m1
    st_p6(r6, 32 * 2, m[8]); // mova [r6+32*2], m8
    st_p6(r6, 32 * 3, m[7]); // mova [r6+32*3], m7
}

/// `.main_oddhalf_part2_fast_rect2` of `inv_txfm_add_dct_dct_8x32_10bpc`: rounds `m0`-`m3` (`m11`), then as `_fast`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast_rect2(
    m: &mut M,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    for x in 0..4 {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3
    }
    for x in 0..4 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast(m, r6, r4, r5)
}

/// `.main_oddhalf_part2_fast` of `inv_txfm_add_dct_dct_8x32_10bpc`: in3, in5, in11, in13 in `m0`-`m3` (lower
/// half zero); otherwise as `.main_oddhalf_part2`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast(
    m: &mut M,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    m[7] = pd(-601); // vpbroadcastd m7, [pd_m601]
    m[8] = pd(4052); // vpbroadcastd m8, [pd_4052]
    m[6] = pd(3973); // vpbroadcastd m6, [pd_3973]
    m[9] = pd(995); // vpbroadcastd m9, [pd_995]
    m[5] = pd(-2106); // vpbroadcastd m5, [pd_m2106]
    m[10] = pd(3513); // vpbroadcastd m10, [pd_3513]
    m[4] = pd(3290); // vpbroadcastd m4, [pd_3290]
    m[15] = pd(2440); // vpbroadcastd m15, [pd_2440]
    m[7] = _mm256_mullo_epi32(m[7], m[0]); // pmulld m7, m0
    m[0] = _mm256_mullo_epi32(m[0], m[8]); // pmulld m0, m8
    m[6] = _mm256_mullo_epi32(m[6], m[1]); // pmulld m6, m1
    m[1] = _mm256_mullo_epi32(m[1], m[9]); // pmulld m1, m9
    m[5] = _mm256_mullo_epi32(m[5], m[2]); // pmulld m5, m2
    m[2] = _mm256_mullo_epi32(m[2], m[10]); // pmulld m2, m10
    m[4] = _mm256_mullo_epi32(m[4], m[3]); // pmulld m4, m3
    m[3] = _mm256_mullo_epi32(m[3], m[15]); // pmulld m3, m15
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast2(m, r6, r4, r5) // jmp .main_oddhalf_part2_fast2
}

/// `.main_oddhalf_part2_rect2` of `inv_txfm_add_dct_dct_8x32_10bpc`: rounds `m0`-`m7` (`m11`), then as `.main_oddhalf_part2`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_rect2(
    m: &mut M,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    for x in 0..8 {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2(m, r6, r4, r5)
}

/// `.main_oddhalf_part2` of `inv_txfm_add_dct_dct_8x32_10bpc`: in3, in5, in11, in13, in19, in21, in27, in29 in
/// `m0`-`m7`, part 1's terms at `[r6-32*4]` … `[r6+32*3]`; `m11` = pd_2048, `m12`/`m13` the clips, `m14` =
/// pd_2896. Writes the 16 odd-half outputs to `[r6-32*4]` … `[r6+32*11]`, sets `r4` = `r6`, `r5` = `r6+32*8`
/// and advances `r6` by `32*16` (to the frame of the idct16 odd half).
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2(
    m: &mut M,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    itx_mulsub_2d(m, 7, 0, 8, 9, 10, None, 4052, 601, 0); // ITX_MULSUB_2D 7, 0, 8, 9, 10, _, 4052,  601 ; t23a, t24a
    itx_mulsub_2d(m, 1, 6, 8, 9, 10, None, 995, 3973, 0); // ITX_MULSUB_2D 1, 6, 8, 9, 10, _,  995, 3973 ; t20a, t27a
    itx_mulsub_2d(m, 5, 2, 8, 9, 10, None, 3513, 2106, 0); // ITX_MULSUB_2D 5, 2, 8, 9, 10, _, 3513, 2106 ; t21a, t26a
    itx_mulsub_2d(m, 3, 4, 8, 9, 10, None, 2440, 3290, 0); // ITX_MULSUB_2D 3, 4, 8, 9, 10, _, 2440, 3290 ; t22a, t25a
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast2(m, r6, r4, r5)
}

/// `.main_oddhalf_part2_fast2` of `inv_txfm_add_dct_dct_8x32_10bpc` (to its `ret`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast2(
    m: &mut M,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    for x in [0, 7, 6, 1, 2, 5, 4, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m7, m6, m1, m2, m5, m4, m3
    }
    for x in [0, 4, 6, 2, 1, 5, 7, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m4, m6, m2, m1, m5, m7, m3
    }
    m[8] = _mm256_sub_epi32(m[0], m[4]); // psubd m8, m0, m4 ; t25
    m[0] = _mm256_add_epi32(m[0], m[4]); // paddd m0, m4 ; t24
    m[4] = _mm256_sub_epi32(m[6], m[2]); // psubd m4, m6, m2 ; t26
    m[6] = _mm256_add_epi32(m[6], m[2]); // paddd m6, m2 ; t27
    m[2] = _mm256_sub_epi32(m[1], m[5]); // psubd m2, m1, m5 ; t21
    m[1] = _mm256_add_epi32(m[1], m[5]); // paddd m1, m5 ; t20
    m[5] = _mm256_sub_epi32(m[7], m[3]); // psubd m5, m7, m3 ; t22
    m[7] = _mm256_add_epi32(m[7], m[3]); // paddd m7, m3 ; t23
    for x in [8, 5, 4, 2, 0, 6, 1, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m8, m5, m4, m2, m0, m6, m1, m7
    }
    for x in [8, 5, 4, 2, 0, 6, 1, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m8, m5, m4, m2, m0, m6, m1, m7
    }
    m[15] = pd(2276); // vpbroadcastd m15, [pd_2276]
    m[10] = pd(3406); // vpbroadcastd m10, [pd_3406]
    itx_mulsub_2d(m, 4, 2, 3, 9, usize::MAX, Some(11), 10, 15, 0); // ITX_MULSUB_2D 4, 2, 3, 9, _, 11, 10, 15 ; t21a, t26a
    itx_mulsub_2d(m, 8, 5, 3, 9, usize::MAX, Some(11), 10, 15, 2); // ITX_MULSUB_2D 8, 5, 3, 9, _, 11, 10, 15, 2 ; t25a, t22a
    m[3] = _mm256_sub_epi32(m[0], m[6]); // psubd m3, m0, m6 ; t27a
    m[0] = _mm256_add_epi32(m[0], m[6]); // paddd m0, m6 ; t24a
    m[6] = _mm256_sub_epi32(m[7], m[1]); // psubd m6, m7, m1 ; t20a
    m[7] = _mm256_add_epi32(m[7], m[1]); // paddd m7, m1 ; t23a
    m[1] = _mm256_sub_epi32(m[5], m[4]); // psubd m1, m5, m4 ; t21
    m[5] = _mm256_add_epi32(m[5], m[4]); // paddd m5, m4 ; t22
    m[4] = _mm256_sub_epi32(m[8], m[2]); // psubd m4, m8, m2 ; t26
    m[8] = _mm256_add_epi32(m[8], m[2]); // paddd m8, m2 ; t25
    for x in [3, 6, 1, 4, 0, 7, 5, 8] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m3, m6, m1, m4, m0, m7, m5, m8
    }
    for x in [3, 6, 1, 4, 0, 7, 5, 8] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m3, m6, m1, m4, m0, m7, m5, m8
    }
    m[15] = pd(3784); // vpbroadcastd m15, [pd_3784]
    m[10] = pd(1567); // vpbroadcastd m10, [pd_1567]
    itx_mulsub_2d(m, 4, 1, 2, 9, usize::MAX, Some(11), 10, 15, 2); // ITX_MULSUB_2D 4, 1, 2, 9, _, 11, 10, 15, 2 ; t26a, t21a
    itx_mulsub_2d(m, 3, 6, 2, 9, usize::MAX, Some(11), 10, 15, 2); // ITX_MULSUB_2D 3, 6, 2, 9, _, 11, 10, 15, 2 ; t27,  t20
    let p = *r6;
    m[9] = ld_p6(p, -32 * 4); // mova m9, [r6-32*4] ; t16a
    m[10] = ld_p6(p, -32 * 3); // mova m10, [r6-32*3] ; t17
    m[2] = _mm256_sub_epi32(m[9], m[7]); // psubd m2, m9, m7 ; t23
    m[9] = _mm256_add_epi32(m[9], m[7]); // paddd m9, m7 ; t16
    m[7] = _mm256_sub_epi32(m[10], m[5]); // psubd m7, m10, m5 ; t22a
    m[10] = _mm256_add_epi32(m[10], m[5]); // paddd m10, m5 ; t17a
    for x in [9, 10, 2, 7] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m9, m10, m2, m7
    }
    for x in [9, 10, 2, 7] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m9, m10, m2, m7
    }
    st_p6(p, -32 * 4, m[9]); // mova [r6-32*4], m9
    st_p6(p, -32 * 3, m[10]); // mova [r6-32*3], m10
    m[9] = ld_p6(p, -32 * 2); // mova m9, [r6-32*2] ; t18a
    m[10] = ld_p6(p, -32 * 1); // mova m10, [r6-32*1] ; t19
    m[5] = _mm256_sub_epi32(m[9], m[1]); // psubd m5, m9, m1 ; t21
    m[9] = _mm256_add_epi32(m[9], m[1]); // paddd m9, m1 ; t18
    m[1] = _mm256_sub_epi32(m[10], m[6]); // psubd m1, m10, m6 ; t20a
    m[10] = _mm256_add_epi32(m[10], m[6]); // paddd m10, m6 ; t19a
    for x in [9, 10, 5, 1] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m9, m10, m5, m1
    }
    for x in [9, 10, 5, 1] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m9, m10, m5, m1
    }
    st_p6(p, -32 * 2, m[9]); // mova [r6-32*2], m9
    st_p6(p, -32 * 1, m[10]); // mova [r6-32*1], m10
    m[9] = ld_p6(p, 32 * 0); // mova m9, [r6+32*0] ; t28
    m[10] = ld_p6(p, 32 * 1); // mova m10, [r6+32*1] ; t29a
    m[6] = _mm256_sub_epi32(m[9], m[3]); // psubd m6, m9, m3 ; t27a
    m[9] = _mm256_add_epi32(m[9], m[3]); // paddd m9, m3 ; t28a
    m[3] = _mm256_sub_epi32(m[10], m[4]); // psubd m3, m10, m4 ; t26
    m[10] = _mm256_add_epi32(m[10], m[4]); // paddd m10, m4 ; t29
    for x in [9, 10, 6, 3] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m9, m10, m6, m3
    }
    for x in [9, 10, 6, 3] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m9, m10, m6, m3
    }
    for x in [6, 3, 1, 5] {
        m[x] = _mm256_mullo_epi32(m[x], m[14]); // REPX {pmulld x, m14}, m6, m3, m1, m5
    }
    m[6] = _mm256_add_epi32(m[6], m[11]); // paddd m6, m11
    m[3] = _mm256_add_epi32(m[3], m[11]); // paddd m3, m11
    m[4] = _mm256_sub_epi32(m[6], m[1]); // psubd m4, m6, m1 ; t20
    m[6] = _mm256_add_epi32(m[6], m[1]); // paddd m6, m1 ; t27
    m[1] = _mm256_sub_epi32(m[3], m[5]); // psubd m1, m3, m5 ; t21a
    m[3] = _mm256_add_epi32(m[3], m[5]); // paddd m3, m5 ; t26a
    for x in [4, 1, 3, 6] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad  x, 12 }, m4, m1, m3, m6
    }
    st_p6(p, 32 * 0, m[4]); // mova [r6+32*0], m4
    st_p6(p, 32 * 1, m[1]); // mova [r6+32*1], m1
    m[4] = ld_p6(p, 32 * 2); // mova m4, [r6+32*2] ; t30
    m[1] = ld_p6(p, 32 * 3); // mova m1, [r6+32*3] ; t31a
    m[5] = _mm256_sub_epi32(m[4], m[8]); // psubd m5, m4, m8 ; t25a
    m[4] = _mm256_add_epi32(m[4], m[8]); // paddd m4, m8 ; t30a
    m[8] = _mm256_sub_epi32(m[1], m[0]); // psubd m8, m1, m0 ; t24
    m[1] = _mm256_add_epi32(m[1], m[0]); // paddd m1, m0 ; t31
    for x in [8, 5, 4, 1] {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m8, m5, m4, m1
    }
    for x in [8, 5, 4, 1] {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m8, m5, m4, m1
    }
    for x in [5, 8, 7, 2] {
        m[x] = _mm256_mullo_epi32(m[x], m[14]); // REPX {pmulld x, m14}, m5, m8, m7, m2
    }
    m[5] = _mm256_add_epi32(m[5], m[11]); // paddd m5, m11
    m[8] = _mm256_add_epi32(m[8], m[11]); // paddd m8, m11
    m[0] = _mm256_sub_epi32(m[5], m[7]); // psubd m0, m5, m7 ; t22
    m[5] = _mm256_add_epi32(m[5], m[7]); // paddd m5, m7 ; t25
    m[7] = _mm256_sub_epi32(m[8], m[2]); // psubd m7, m8, m2 ; t23a
    m[2] = _mm256_add_epi32(m[2], m[8]); // paddd m2, m8 ; t24a
    for x in [0, 7, 2, 5] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad  x, 12 }, m0, m7, m2, m5
    }
    st_p6(p, 32 * 2, m[0]); // mova [r6+32*2], m0
    st_p6(p, 32 * 3, m[7]); // mova [r6+32*3], m7
    *r4 = p; // mov r4, r6
    let p = at(p, 32 * 8); // add r6, 32*8
    st_p6(p, -32 * 4, m[2]); // mova [r6-32*4], m2
    st_p6(p, -32 * 3, m[5]); // mova [r6-32*3], m5
    st_p6(p, -32 * 2, m[3]); // mova [r6-32*2], m3
    st_p6(p, -32 * 1, m[6]); // mova [r6-32*1], m6
    st_p6(p, 32 * 0, m[9]); // mova [r6+32*0], m9
    st_p6(p, 32 * 1, m[10]); // mova [r6+32*1], m10
    st_p6(p, 32 * 2, m[4]); // mova [r6+32*2], m4
    st_p6(p, 32 * 3, m[1]); // mova [r6+32*3], m1
    *r5 = p; // mov r5, r6
    *r6 = at(p, 32 * 8); // add r6, 32*8
}

/// `.main_end` of `inv_txfm_add_dct_dct_8x32_10bpc` (falls through into `.transpose`): `m0`-`m7` the idct16
/// even outputs, the frames `[r6]`, `[r5]`, `[r4]` as `.main_oddhalf_part2` and the idct16 odd half left them;
/// `m11` = pd_2048 (becomes pd_2), `m12`/`m13` the clips. Rounds by 2, packs to words; rows 16-31 of each
/// pair to `[r5-32*4]` … `[r5+32*3]`, rows 0-15 transposed in `m0`-`m7`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_main_end(m: &mut M, r6: *mut __m256i, r5: *mut __m256i, r4: *mut __m256i) {
    m[11] = _mm256_srli_epi32::<10>(m[11]); // psrld m11, 10 ; pd_2
    idct32_end::<2>(m, r6, r5, r4, 0, 15, 8, 9, 10, true); // IDCT32_END 0, 15, 8, 9, 10, 2
    idct32_end::<2>(m, r6, r5, r4, 1, 14, 8, 9, 10, true); // IDCT32_END 1, 14, 8, 9, 10, 2
    m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1 ; 16 17
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1 ;  0  1
    m[1] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m1, m14, m15 ; 14 15
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 30 31
    st_p6(r5, 32 * 3, m[8]); // mova [r5+32*3], m8
    st_p6(r5, 32 * 2, m[14]); // mova [r5+32*2], m14
    idct32_end::<2>(m, r6, r5, r4, 2, 15, 8, 9, 10, true); // IDCT32_END 2, 15, 8, 9, 10, 2
    idct32_end::<2>(m, r6, r5, r4, 3, 14, 8, 9, 10, true); // IDCT32_END 3, 14, 8, 9, 10, 2
    m[8] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m8, m2, m3 ; 18 19
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3 ;  2  3
    m[3] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m3, m14, m15 ; 12 13
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 28 29
    st_p6(r5, 32 * 1, m[8]); // mova [r5+32*1], m8
    st_p6(r5, 32 * 0, m[14]); // mova [r5+32*0], m14
    idct32_end::<2>(m, r6, r5, r4, 4, 15, 8, 9, 10, true); // IDCT32_END 4, 15, 8, 9, 10, 2
    idct32_end::<2>(m, r6, r5, r4, 5, 14, 8, 9, 10, true); // IDCT32_END 5, 14, 8, 9, 10, 2
    m[8] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m8, m4, m5 ; 20 21
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5 ;  4  5
    m[5] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m5, m14, m15 ; 10 11
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 26 27
    st_p6(r5, -32 * 1, m[8]); // mova [r5-32*1], m8
    st_p6(r5, -32 * 2, m[14]); // mova [r5-32*2], m14
    idct32_end::<2>(m, r6, r5, r4, 6, 15, 8, 9, 10, true); // IDCT32_END 6, 15, 8, 9, 10, 2
    idct32_end::<2>(m, r6, r5, r4, 7, 14, 8, 9, 10, true); // IDCT32_END 7, 14, 8, 9, 10, 2
    m[8] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m8, m6, m7 ; 22 23
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7 ;  6  7
    m[7] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m7, m14, m15 ;  8  9
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 24 25
    st_p6(r5, -32 * 3, m[8]); // mova [r5-32*3], m8
    st_p6(r5, -32 * 4, m[14]); // mova [r5-32*4], m14
    inv_txfm_add_dct_dct_8x32_10bpc_transpose(m)
}

/// `.transpose` of `inv_txfm_add_dct_dct_8x32_10bpc`: the word pairs of `m0`-`m7` (as `.main_end` leaves them)
/// to eight rows of eight words per lane.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_transpose(m: &mut M) {
    m[15] = _mm256_unpackhi_epi32(m[3], m[1]); // punpckhdq m15, m3, m1
    m[3] = _mm256_unpacklo_epi32(m[3], m[1]); // punpckldq m3, m1
    m[1] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m1, m4, m6
    m[4] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m4, m6
    m[6] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m6, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpackhi_epi32(m[7], m[5]); // punpckhdq m2, m7, m5
    m[7] = _mm256_unpacklo_epi32(m[7], m[5]); // punpckldq m7, m5
    m[5] = _mm256_unpacklo_epi64(m[2], m[15]); // punpcklqdq m5, m2, m15
    m[2] = _mm256_unpackhi_epi64(m[2], m[15]); // punpckhqdq m2, m15
    m[15] = _mm256_unpackhi_epi64(m[7], m[3]); // punpckhqdq m15, m7, m3
    m[7] = _mm256_unpacklo_epi64(m[7], m[3]); // punpcklqdq m7, m3
    m[3] = _mm256_unpackhi_epi64(m[6], m[1]); // punpckhqdq m3, m6, m1
    m[6] = _mm256_unpacklo_epi64(m[6], m[1]); // punpcklqdq m6, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m1, m0, m4
    m[0] = _mm256_unpacklo_epi64(m[0], m[4]); // punpcklqdq m0, m4
    m[4] = _mm256_permute2x128_si256::<0x31>(m[0], m[7]); // vperm2i128 m4, m0, m7, 0x31
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[7])); // vinserti128 m0, xm7, 1
    m[7] = _mm256_permute2x128_si256::<0x31>(m[3], m[2]); // vperm2i128 m7, m3, m2, 0x31
    m[3] = _mm256_inserti128_si256::<1>(m[3], lo(m[2])); // vinserti128 m3, xm2, 1
    m[2] = _mm256_inserti128_si256::<1>(m[6], lo(m[5])); // vinserti128 m2, m6, xm5, 1
    m[6] = _mm256_permute2x128_si256::<0x31>(m[6], m[5]); // vperm2i128 m6, m5, 0x31
    m[5] = _mm256_permute2x128_si256::<0x31>(m[1], m[15]); // vperm2i128 m5, m1, m15, 0x31
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[15])); // vinserti128 m1, xm15, 1
}

/// `.main_zero` of `inv_txfm_add_identity_identity_8x32_10bpc` (falls through into `.main`): clears the eight
/// coefficient rows `[cq+128*0..7]` (`m6` = 0), then adds the words of `m0`-`m3` (four rows of 2x8 pixels,
/// transposed here) to the 8x8 block at `dst`, clipped to 0 … `m7`. `r5`, `r6`, `r4` = `strideq*5`, `*3`, `*7`.
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32_10bpc_main_zero(
    m: &mut M,
    c: Coef,
    dst: Dst,
    stride: isize,
    r5: isize,
    r6: isize,
    r4: isize,
) {
    for i in 0..8 {
        stc_p6(c, 128 * i, m[6]); // REPX {mova [cq+128*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    }
    inv_txfm_add_identity_identity_8x32_10bpc_main(m, dst, stride, r5, r6, r4)
}

/// `.main` of `inv_txfm_add_identity_identity_8x32_10bpc` (see `.main_zero`)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32_10bpc_main(
    m: &mut M,
    dst: Dst,
    stride: isize,
    r5: isize,
    r6: isize,
    r4: isize,
) {
    m[4] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m4, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[0], m[4]); // punpckhwd m3, m0, m4
    m[0] = _mm256_unpacklo_epi16(m[0], m[4]); // punpcklwd m0, m4
    m[4] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m4, m2, m1
    m[2] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m2, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m0, m2
    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4
    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4
    m[4] = zx(_mm_loadu_si128(dst.cast())); // mova xm4, [dstq+strideq*0]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(dst, stride * 4).cast())); // vinserti128 m4, [dstq+strideq*4], 1
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4
    m[4] = zx(_mm_loadu_si128(at(dst, stride).cast())); // mova xm4, [dstq+strideq*1]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(dst, r5).cast())); // vinserti128 m4, [dstq+r5       ], 1
    m[1] = _mm256_add_epi16(m[1], m[4]); // paddw m1, m4
    m[4] = zx(_mm_loadu_si128(at(dst, stride * 2).cast())); // mova xm4, [dstq+strideq*2]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(dst, r6 * 2).cast())); // vinserti128 m4, [dstq+r6*2     ], 1
    m[2] = _mm256_add_epi16(m[2], m[4]); // paddw m2, m4
    m[4] = zx(_mm_loadu_si128(at(dst, r6).cast())); // mova xm4, [dstq+r6       ]
    m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_loadu_si128(at(dst, r4).cast())); // vinserti128 m4, [dstq+r4       ], 1
    m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
    for x in 0..4 {
        m[x] = _mm256_max_epi16(m[x], m[6]); // REPX {pmaxsw x, m6}, m0, m1, m2, m3
    }
    for x in 0..4 {
        m[x] = _mm256_min_epi16(m[x], m[7]); // REPX {pminsw x, m7}, m0, m1, m2, m3
    }
    _mm_storeu_si128(dst.cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
    _mm_storeu_si128(at(dst, stride * 4).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*4], m0, 1
    _mm_storeu_si128(at(dst, stride).cast(), lo(m[1])); // mova [dstq+strideq*1], xm1
    _mm_storeu_si128(at(dst, r5).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+r5       ], m1, 1
    _mm_storeu_si128(at(dst, stride * 2).cast(), lo(m[2])); // mova [dstq+strideq*2], xm2
    _mm_storeu_si128(at(dst, r6 * 2).cast(), _mm256_extracti128_si256::<1>(m[2])); // vextracti128 [dstq+r6*2     ], m2, 1
    _mm_storeu_si128(at(dst, r6).cast(), lo(m[3])); // mova [dstq+r6       ], xm3
    _mm_storeu_si128(at(dst, r4).cast(), _mm256_extracti128_si256::<1>(m[3])); // vextracti128 [dstq+r4       ], m3, 1
}

// --- 8x32 ---------------------------------------------------------------------------------------------------

/// `.pass1_main_part1` of `inv_txfm_add_dct_dct_8x32_10bpc`: the 8-point DCT of eight coefficient rows
/// `[cq+128*0..7]`, rounded by 2 (`m11` = pd_2048, `m12`/`m13` the clips)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_pass1_main_part1(m: &mut M, c: Coef) {
    for i in 0..8 {
        m[i] = ldc_p6(c, 128 * i as isize); // mova mi, [cq+128*i] (i = 0 … 7)
    }
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    m[1] = _mm256_srli_epi32::<10>(m[11]); // psrld m1, m11, 10 ; pd_2
    for x in [0, 6, 5, 3] {
        m[x] = _mm256_add_epi32(m[x], m[1]); // REPX {paddd x, m1}, m0, m6, m5, m3
    }
    m[1] = _mm256_add_epi32(m[6], m[7]); // paddd m1, m6, m7 ; out1
    m[6] = _mm256_sub_epi32(m[6], m[7]); // psubd m6, m7 ; out6
    m[7] = _mm256_sub_epi32(m[0], m[9]); // psubd m7, m0, m9 ; out7
    m[0] = _mm256_add_epi32(m[0], m[9]); // paddd m0, m9 ; out0
    m[2] = _mm256_add_epi32(m[5], m[4]); // paddd m2, m5, m4 ; out2
    m[5] = _mm256_sub_epi32(m[5], m[4]); // psubd m5, m4 ; out5
    m[4] = _mm256_sub_epi32(m[3], m[8]); // psubd m4, m3, m8 ; out4
    m[3] = _mm256_add_epi32(m[3], m[8]); // paddd m3, m8 ; out3
    for x in 0..8 {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2 }, m0, m1, m2, m3, m4, m5, m6, m7
    }
}

/// `.pass1_main` of `inv_txfm_add_dct_dct_8x32_10bpc`: `.pass1_main_part1`, advances `cq` by 32, packs to
/// words and shuffles (`m14` = idct32_shuf)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc_pass1_main(m: &mut M, c: &mut Coef) {
    inv_txfm_add_dct_dct_8x32_10bpc_pass1_main_part1(m, *c); // call .pass1_main_part1
    *c = at(*c, 32); // add cq, 32
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[2] = _mm256_packs_epi32(m[2], m[3]); // packssdw m2, m3
    m[4] = _mm256_packs_epi32(m[4], m[5]); // packssdw m4, m5
    m[6] = _mm256_packs_epi32(m[6], m[7]); // packssdw m6, m7
    m[0] = _mm256_shuffle_epi8(m[0], m[14]); // pshufb m0, m14
    m[2] = _mm256_shuffle_epi8(m[2], m[14]); // pshufb m2, m14
    m[4] = _mm256_shuffle_epi8(m[4], m[14]); // pshufb m4, m14
    m[6] = _mm256_shuffle_epi8(m[6], m[14]); // pshufb m6, m14
    m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m2, m4, m6
    m[4] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m4, m6
    m[1] = _mm256_permute2x128_si256::<0x31>(m[0], m[2]); // vperm2i128 m1, m0, m2, 0x31 ; 4 6
    m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[2])); // vinserti128 m0, xm2, 1 ; 0 2
    m[2] = _mm256_inserti128_si256::<1>(m[3], lo(m[4])); // vinserti128 m2, m3, xm4, 1 ; 1 3
    m[3] = _mm256_permute2x128_si256::<0x31>(m[3], m[4]); // vperm2i128 m3, m4, 0x31 ; 5 7
}

/// `cglobal inv_txfm_add_dct_dct_8x32_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_10bpc(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[2] = pw(0x7c00); // vpbroadcastd m2, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 32; // or r3d, 32
        r6d = r6d.wrapping_add(640); // add r6d, 640
        r6d >>= 10; // sar r6d, 10
        return inv_txfm_add_dct_dct_8x8_10bpc_dconly3(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly3
    }
    // PROLOGUE 0, 7, 16, 32*12: `s` is [rsp+32*0..2] (the 8 bpc `.main`'s `[rsp+gprsize+32*0..2]`), `f` is
    // [rsp+32*3..11] (`r6` = rsp+32*8, so `[r6-32*4]` is f[1] … `[r6+32*3]` f[8]). Both zeroed whole; indexed
    // with constants only, so they are values, not memory.
    let mut s = [_mm256_setzero_si256(); 3];
    let mut f = [_mm256_setzero_si256(); 9];
    let s = &mut s;
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[14] = _mm256_broadcastsi128_si256(ld_tab128(&idct32_shuf)); // vbroadcasti128 m14, [idct32_shuf]
    let r4 = c; // mov r4, cq
    inv_txfm_add_dct_dct_8x32_10bpc_pass1_main(m, &mut c); // call .pass1_main
    s[0] = m[2]; // mova [rsp+32*0], m2
    s[1] = m[3]; // mova [rsp+32*1], m3
    let fast = if eob < 43 {
        // cmp eobd, 43 / jge .eob43
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        s[2] = m[4]; // REPX {mova x, m4}, [rsp+32*2], m2, m3, m11
        m[2] = m[4];
        m[3] = m[4];
        m[11] = m[4];
        true // jmp .pass1_end_fast
    } else {
        // .eob43: lea r6, [rsp+32*8]
        f[1] = m[0]; // mova [r6-32*4], m0
        f[2] = m[1]; // mova [r6-32*3], m1
        inv_txfm_add_dct_dct_8x32_10bpc_pass1_main(m, &mut c); // call .pass1_main
        s[2] = m[2]; // mova [rsp+32*2], m2
        if eob < 107 {
            // cmp eobd, 107 / jge .eob107
            m[11] = m[3]; // mova m11, m3
            m[2] = m[0]; // mova m2, m0
            m[3] = m[1]; // mova m3, m1
            m[0] = f[1]; // mova m0, [r6-32*4]
            m[1] = f[2]; // mova m1, [r6-32*3]
            m[4] = _mm256_setzero_si256(); // pxor m4, m4
            true
        } else {
            // .eob107:
            f[0] = m[3]; // mova [rsp+32*3], m3
            f[3] = m[0]; // mova [r6-32*2], m0
            f[4] = m[1]; // mova [r6-32*1], m1
            inv_txfm_add_dct_dct_8x32_10bpc_pass1_main(m, &mut c); // call .pass1_main
            if eob < 171 {
                // cmp eobd, 171 / jge .eob171
                m[12] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m12, m2, q1032
                m[13] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m13, m3, q1032
                m[4] = m[0]; // mova m4, m0
                m[5] = m[1]; // mova m5, m1
                m[6] = _mm256_setzero_si256(); // pxor m6, m6
                m[7] = m[6]; // REPX {mova x, m6}, m7, m14, m15
                m[14] = m[6];
                m[15] = m[6];
                // jmp .pass1_end
            } else {
                // .eob171:
                f[5] = m[0]; // mova [r6+32*0], m0
                f[6] = m[1]; // mova [r6+32*1], m1
                f[7] = m[2]; // mova [r6+32*2], m2
                f[8] = m[3]; // mova [r6+32*3], m3
                inv_txfm_add_dct_dct_8x32_10bpc_pass1_main(m, &mut c); // call .pass1_main
                m[12] = _mm256_shuffle_epi32::<0x4e>(f[7]); // pshufd m12, [r6+32*2], q1032 ; out19 out17
                m[13] = _mm256_shuffle_epi32::<0x4e>(f[8]); // pshufd m13, [r6+32*3], q1032 ; out23 out21
                m[4] = f[5]; // mova m4, [r6+32*0] ; out16 out18
                m[5] = f[6]; // mova m5, [r6+32*1] ; out20 out22
                m[14] = _mm256_shuffle_epi32::<0x4e>(m[2]); // pshufd m14, m2, q1032 ; out27 out25
                m[15] = _mm256_shuffle_epi32::<0x4e>(m[3]); // pshufd m15, m3, q1032 ; out31 out29
                m[6] = m[0]; // mova m6, m0 ; out24 out26
                m[7] = m[1]; // mova m7, m1 ; out28 out30
            }
            // .pass1_end:
            m[0] = f[1]; // mova m0, [r6-32*4] ; out0  out2
            m[1] = f[2]; // mova m1, [r6-32*3] ; out4  out6
            m[2] = f[3]; // mova m2, [r6-32*2] ; out8  out10
            m[3] = f[4]; // mova m3, [r6-32*1] ; out12 out14
                         // lea r6, [deint_shuf+128]
            m[11] = f[0]; // mova m11, [rsp+32*3] ; out13 out15
            m[10] = pw(2048); // vpbroadcastd m10, [pw_2048]
            inv_txfm_add_dct_dct_8x32_8bpc_main(m, s); // call m(inv_txfm_add_dct_dct_8x32_8bpc).main
            false // (falls into .end)
        }
    };
    if fast {
        // .pass1_end_fast:
        m[10] = pw(2048); // vpbroadcastd m10, [pw_2048]
                          // lea r6, [deint_shuf+128]
        m[5] = m[4]; // REPX {mova x, m4}, m5, m6, m7
        m[6] = m[4];
        m[7] = m[4];
        inv_txfm_add_dct_dct_8x32_8bpc_main_fast(m, s); // call m(inv_txfm_add_dct_dct_8x32_8bpc).main_fast
                                                        // jmp .end
    }
    // .end: ; [rsp+0*32] = m12
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    c = r4; // mov cq, r4
    s[1] = m[8]; // mova [rsp+32*1], m8
    s[2] = m[9]; // mova [rsp+32*2], m9
    f[0] = m[10]; // mova [rsp+32*3], m10
    f[1] = m[11]; // mova [rsp+32*4], m11
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[1]); // vpermq m1, m1, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    let mut r6 = 0;
    idct_8x8_internal_10bpc_write_8x4_start(m, stride, &mut r6, &mut dst, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4_start
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m0, m2, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[3]); // vpermq m1, m3, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[4]); // vpermq m0, m4, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[5]); // vpermq m1, m5, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[6]); // vpermq m0, m6, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[7]); // vpermq m1, m7, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(s[1]); // vpermq m0, [rsp+32*1], q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(s[2]); // vpermq m1, [rsp+32*2], q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(f[0]); // vpermq m0, [rsp+32*3], q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(f[1]); // vpermq m1, [rsp+32*4], q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(s[0]); // vpermq m0, [rsp+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[13]); // vpermq m1, m13, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[14]); // vpermq m0, m14, q3120
    m[1] = _mm256_permute4x64_epi64::<0x8d>(m[15]); // vpermq m1, m15, q2031
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `cglobal inv_txfm_add_identity_identity_8x32_10bpc, 4, 7, 8, dst, stride, c, eob` from `.pass1` (`m7` =
/// pixel max, set by the 10 and 12 bpc entry points)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32_10bpc_pass1(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    eob: c_int,
) {
    m[5] = pw(5); // vpbroadcastd m5, [pw_5]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    let r6d = eob; // mov r6d, eobd
    let (b, carry) = (eob as u8).overflowing_add(21); // add eobb, 21
    let mut eobd = (eob & !0xff) | b as i32;
    if carry {
        eobd = r6d; // cmovc eobd, r6d ; 43, 107, 171 -> 64, 128, 192
    }
    let r6 = stride * 3; // lea r6, [strideq*3]
    let r5 = stride * 5; // lea r5, [strideq*5]
    let r4 = stride + r6 * 2; // lea r4, [strideq+r6*2] ; strideq*7
    loop {
        // .loop:
        m[0] = ldc_p6(c, 128 * 0); // mova m0, [cq+128*0]
        m[0] = _mm256_packs_epi32(m[0], ldc_p6(c, 128 * 1)); // packssdw m0, [cq+128*1]
        m[1] = ldc_p6(c, 128 * 2); // mova m1, [cq+128*2]
        m[1] = _mm256_packs_epi32(m[1], ldc_p6(c, 128 * 3)); // packssdw m1, [cq+128*3]
        m[2] = ldc_p6(c, 128 * 4); // mova m2, [cq+128*4]
        m[2] = _mm256_packs_epi32(m[2], ldc_p6(c, 128 * 5)); // packssdw m2, [cq+128*5]
        m[3] = ldc_p6(c, 128 * 6); // mova m3, [cq+128*6]
        m[3] = _mm256_packs_epi32(m[3], ldc_p6(c, 128 * 7)); // packssdw m3, [cq+128*7]
        for x in 0..4 {
            m[x] = _mm256_adds_epi16(m[x], m[5]); // REPX {paddsw x, m5}, m0, m1, m2, m3
        }
        for x in 0..4 {
            m[x] = _mm256_srai_epi16::<3>(m[x]); // REPX {psraw  x, 3 }, m0, m1, m2, m3
        }
        inv_txfm_add_identity_identity_8x32_10bpc_main_zero(m, c, dst, stride, r5, r6, r4); // call .main_zero
        c = at(c, 32); // add cq, 32
        dst = at(dst, stride * 8); // lea dstq, [dstq+strideq*8]
        eobd -= 64; // sub eobd, 64
        if eobd < 0 {
            break; // jge .loop
        }
    }
}

/// `cglobal inv_txfm_add_identity_identity_8x32_10bpc, 4, 7, 8, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    inv_txfm_add_identity_identity_8x32_10bpc_pass1(m, dst, stride, c, eob)
}

/// `.pass1_main` of `inv_txfm_add_dct_dct_8x32_12bpc`: `.pass1_main_part1` of 10 bpc, transposed back into
/// `[cq+128*0..7]`; advances `cq` by 32
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_12bpc_pass1_main(m: &mut M, c: &mut Coef) {
    inv_txfm_add_dct_dct_8x32_10bpc_pass1_main_part1(m, *c); // call m(inv_txfm_add_dct_dct_8x32_10bpc).pass1_main_part1
    transpose_8x8_dword(m, [0, 1, 2, 3, 4, 5, 6, 7], [8, 9, 10, 15]); // TRANSPOSE_8X8_DWORD 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15
    for i in 0..8 {
        stc_p6(*c, 128 * i as isize, m[i]); // mova [cq+128*i], mi (i = 0 … 7)
    }
    *c = at(*c, 32); // add cq, 32
}

/// `.main_end` of `inv_txfm_add_dct_dct_8x32_12bpc` (32x8 at 12 bpc): as the 10 bpc `.main_end` without the
/// packing; rows 16-31 to `[cq+32*16..31]`, rows 0-15 in `m0`-`m15` (`[cq+32*0..2]` and `[cq+32*12..14]` are
/// spill slots).
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_12bpc_main_end(
    m: &mut M,
    r6: *mut __m256i,
    r5: *mut __m256i,
    r4: *mut __m256i,
    c: Coef,
) {
    m[11] = _mm256_srli_epi32::<10>(m[11]); // psrld m11, 10 ; pd_2
    idct32_end::<2>(m, r6, r5, r4, 0, 15, 8, 9, 10, false); // IDCT32_END 0, 15, 8, 9, 10, 2, 0
    stc_p6(c, 32 * 16, m[8]); // mova [cq+32*16], m8
    stc_p6(c, 32 * 31, m[9]); // mova [cq+32*31], m9
    idct32_end::<2>(m, r6, r5, r4, 1, 14, 8, 9, 10, false); // IDCT32_END 1, 14, 8, 9, 10, 2, 0
    stc_p6(c, 32 * 17, m[8]); // mova [cq+32*17], m8
    stc_p6(c, 32 * 30, m[9]); // mova [cq+32*30], m9
    stc_p6(c, 32 * 14, m[14]); // mova [cq+32*14], m14
    idct32_end::<2>(m, r6, r5, r4, 2, 14, 8, 9, 10, false); // IDCT32_END 2, 14, 8, 9, 10, 2, 0
    stc_p6(c, 32 * 18, m[8]); // mova [cq+32*18], m8
    stc_p6(c, 32 * 29, m[9]); // mova [cq+32*29], m9
    stc_p6(c, 32 * 13, m[14]); // mova [cq+32*13], m14
    idct32_end::<2>(m, r6, r5, r4, 3, 14, 8, 9, 10, false); // IDCT32_END 3, 14, 8, 9, 10, 2, 0
    stc_p6(c, 32 * 19, m[8]); // mova [cq+32*19], m8
    stc_p6(c, 32 * 28, m[9]); // mova [cq+32*28], m9
    stc_p6(c, 32 * 12, m[14]); // mova [cq+32*12], m14
    idct32_end::<2>(m, r6, r5, r4, 4, 14, 8, 9, 10, false); // IDCT32_END 4, 14, 8, 9, 10, 2, 0
    stc_p6(c, 32 * 20, m[8]); // mova [cq+32*20], m8
    stc_p6(c, 32 * 27, m[9]); // mova [cq+32*27], m9
    stc_p6(c, 32 * 0, m[0]); // mova [cq+32* 0], m0
    stc_p6(c, 32 * 1, m[1]); // mova [cq+32* 1], m1
    stc_p6(c, 32 * 2, m[2]); // mova [cq+32* 2], m2
    idct32_end::<2>(m, r6, r5, r4, 5, 10, 0, 1, 2, false); // IDCT32_END 5, 10, 0, 1, 2, 2, 0
    stc_p6(c, 32 * 21, m[0]); // mova [cq+32*21], m0
    stc_p6(c, 32 * 26, m[1]); // mova [cq+32*26], m1
    idct32_end::<2>(m, r6, r5, r4, 6, 9, 0, 1, 2, false); // IDCT32_END 6, 9, 0, 1, 2, 2, 0
    stc_p6(c, 32 * 22, m[0]); // mova [cq+32*22], m0
    stc_p6(c, 32 * 25, m[1]); // mova [cq+32*25], m1
    idct32_end::<2>(m, r6, r5, r4, 7, 8, 0, 1, 2, false); // IDCT32_END 7, 8, 0, 1, 2, 2, 0
    stc_p6(c, 32 * 23, m[0]); // mova [cq+32*23], m0
    stc_p6(c, 32 * 24, m[1]); // mova [cq+32*24], m1
    m[0] = ldc_p6(c, 32 * 0); // mova m0, [cq+32* 0]
    m[1] = ldc_p6(c, 32 * 1); // mova m1, [cq+32* 1]
    m[2] = ldc_p6(c, 32 * 2); // mova m2, [cq+32* 2]
    m[11] = m[14]; // mova m11, m14
    m[12] = ldc_p6(c, 32 * 12); // mova m12, [cq+32*12]
    m[13] = ldc_p6(c, 32 * 13); // mova m13, [cq+32*13]
    m[14] = ldc_p6(c, 32 * 14); // mova m14, [cq+32*14]
}

/// `cglobal inv_txfm_add_dct_dct_8x32_12bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_8x32_12bpc(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[2] = pw(0x7000); // vpbroadcastd m2, [dconly_12bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 32; // or r3d, 32
        r6d = r6d.wrapping_add(640); // add r6d, 640
        r6d >>= 10; // sar r6d, 10
        return inv_txfm_add_dct_dct_8x8_10bpc_dconly3(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_8x8_10bpc).dconly3
    }
    // PROLOGUE 0, 7, 16, 32*24: the frame [rsp+32*0..23], zeroed whole (a zero-initialized array), reached through `r6`,
    // `r4`, `r5` only.
    let mut frame = [_mm256_setzero_si256(); 24];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
    m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
    let r4c = c; // mov r4, cq
    let mut r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    let mut r4: *mut __m256i = core::ptr::null_mut();
    let mut r5: *mut __m256i = core::ptr::null_mut();
    inv_txfm_add_dct_dct_8x32_12bpc_pass1_main(m, &mut c); // call .pass1_main
    let fast = if eob < 43 {
        // cmp eobd, 43 / jge .eob43 / jmp .pass2_fast
        true
    } else {
        // .eob43:
        inv_txfm_add_dct_dct_8x32_12bpc_pass1_main(m, &mut c); // call .pass1_main
        if eob < 107 {
            // cmp eobd, 107 / jge .eob107
            true
        } else {
            // .eob107:
            inv_txfm_add_dct_dct_8x32_12bpc_pass1_main(m, &mut c); // call .pass1_main
            if eob >= 171 {
                // cmp eobd, 171 / jge .eob171 / jmp .pass2
                // .eob171:
                inv_txfm_add_dct_dct_8x32_12bpc_pass1_main(m, &mut c); // call .pass1_main
            }
            false
        }
    };
    if fast {
        // .pass2_fast:
        c = r4c; // mov cq, r4
        m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
        m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
        m[0] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 1)); // pmaxsd m0, m12, [cq+128*1+ 0]
        m[1] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 7)); // pmaxsd m1, m12, [cq+128*7+ 0]
        m[2] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 1 + 32)); // pmaxsd m2, m12, [cq+128*1+32]
        m[3] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 7 + 32)); // pmaxsd m3, m12, [cq+128*7+32]
        for x in 0..4 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
        }
        m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
        inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast(m, r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast
        m[0] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 3)); // pmaxsd m0, m12, [cq+128*3+ 0]
        m[1] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 5)); // pmaxsd m1, m12, [cq+128*5+ 0]
        m[2] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 3 + 32)); // pmaxsd m2, m12, [cq+128*3+32]
        m[3] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 5 + 32)); // pmaxsd m3, m12, [cq+128*5+32]
        for x in 0..4 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
        }
        inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast(m, &mut r6, &mut r4, &mut r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast
        m[0] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 2)); // pmaxsd m0, m12, [cq+128*2+ 0]
        m[1] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 6)); // pmaxsd m1, m12, [cq+128*6+ 0]
        m[2] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 2 + 32)); // pmaxsd m2, m12, [cq+128*2+32]
        m[3] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 6 + 32)); // pmaxsd m3, m12, [cq+128*6+32]
        for x in 0..4 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
        }
        idct_8x16_internal_10bpc_main_oddhalf_fast(m, r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_fast
        m[0] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 0)); // pmaxsd m0, m12, [cq+128*0+ 0]
        m[1] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 4)); // pmaxsd m1, m12, [cq+128*4+ 0]
        m[2] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 0 + 32)); // pmaxsd m2, m12, [cq+128*0+32]
        m[3] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * 4 + 32)); // pmaxsd m3, m12, [cq+128*4+32]
        for x in 0..4 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3
        }
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        m[5] = m[4]; // REPX {mova x, m4}, m5, m6, m7
        m[6] = m[4];
        m[7] = m[4];
        idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
        idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
                                                   // jmp .pass2_end
    } else {
        // .pass2:
        c = r4c; // mov cq, r4
        m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
        m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
        for k in 0..4isize {
            for (i, row) in [1isize, 7].into_iter().enumerate() {
                // pmaxsd m(2k+i), m12, [cq+128*row+32*k]: m0 [1+0], m1 [7+0], m2 [1+32], m3 [7+32], …
                m[2 * k as usize + i] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * row + 32 * k));
            }
        }
        for x in 0..8 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
        }
        m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
        inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1(m, r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1
        for k in 0..4isize {
            for (i, row) in [3isize, 5].into_iter().enumerate() {
                // pmaxsd m(2k+i), m12, [cq+128*row+32*k]: m0 [3+0], m1 [5+0], m2 [3+32], m3 [5+32], …
                m[2 * k as usize + i] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * row + 32 * k));
            }
        }
        for x in 0..8 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
        }
        inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2(m, &mut r6, &mut r4, &mut r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2
        for k in 0..4isize {
            for (i, row) in [2isize, 6].into_iter().enumerate() {
                // pmaxsd m(2k+i), m12, [cq+128*row+32*k]: m0 [2+0], m1 [6+0], m2 [2+32], m3 [6+32], …
                m[2 * k as usize + i] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * row + 32 * k));
            }
        }
        for x in 0..8 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
        }
        idct_8x16_internal_10bpc_main_oddhalf(m, r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
        for k in 0..4isize {
            for (i, row) in [0isize, 4].into_iter().enumerate() {
                // pmaxsd m(2k+i), m12, [cq+128*row+32*k]: m0 [0+0], m1 [4+0], m2 [0+32], m3 [4+32], …
                m[2 * k as usize + i] = _mm256_max_epi32(m[12], ldc_p6(c, 128 * row + 32 * k));
            }
        }
        for x in 0..8 {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m1, m2, m3, m4, m5, m6, m7
        }
        idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
        idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    }
    // .pass2_end:
    m[11] = _mm256_srli_epi32::<8>(m[11]); // psrld m11, 8 ; pd_8
    idct32_end::<4>(m, r6, r5, r4, 0, 15, 8, 9, 10, true); // IDCT32_END 0, 15, 8, 9, 10, 4
    idct32_end::<4>(m, r6, r5, r4, 1, 14, 8, 9, 10, true); // IDCT32_END 1, 14, 8, 9, 10, 4
    m[8] = _mm256_unpackhi_epi64(m[0], m[1]); // punpckhqdq m8, m0, m1 ; 16 17 (interleaved)
    m[0] = _mm256_unpacklo_epi64(m[0], m[1]); // punpcklqdq m0, m1 ;  0  1 (interleaved)
    m[1] = _mm256_unpacklo_epi64(m[14], m[15]); // punpcklqdq m1, m14, m15 ; 14 15 (interleaved)
    m[14] = _mm256_unpackhi_epi64(m[14], m[15]); // punpckhqdq m14, m15 ; 30 31 (interleaved)
    st_p6(r5, 32 * 3, m[8]); // mova [r5+32*3], m8
    st_p6(r5, 32 * 2, m[14]); // mova [r5+32*2], m14
    idct32_end::<4>(m, r6, r5, r4, 2, 15, 8, 9, 10, true); // IDCT32_END 2, 15, 8, 9, 10, 4
    idct32_end::<4>(m, r6, r5, r4, 3, 14, 8, 9, 10, true); // IDCT32_END 3, 14, 8, 9, 10, 4
    m[8] = _mm256_unpackhi_epi64(m[2], m[3]); // punpckhqdq m8, m2, m3 ; 18 19 (interleaved)
    m[2] = _mm256_unpacklo_epi64(m[2], m[3]); // punpcklqdq m2, m3 ;  2  3 (interleaved)
    m[3] = _mm256_unpacklo_epi64(m[14], m[15]); // punpcklqdq m3, m14, m15 ; 12 13 (interleaved)
    m[14] = _mm256_unpackhi_epi64(m[14], m[15]); // punpckhqdq m14, m15 ; 28 29 (interleaved)
    st_p6(r5, 32 * 1, m[8]); // mova [r5+32*1], m8
    st_p6(r5, 32 * 0, m[14]); // mova [r5+32*0], m14
    idct32_end::<4>(m, r6, r5, r4, 4, 15, 8, 9, 10, true); // IDCT32_END 4, 15, 8, 9, 10, 4
    idct32_end::<4>(m, r6, r5, r4, 5, 14, 8, 9, 10, true); // IDCT32_END 5, 14, 8, 9, 10, 4
    m[8] = _mm256_unpackhi_epi64(m[4], m[5]); // punpckhqdq m8, m4, m5 ; 20 21 (interleaved)
    m[4] = _mm256_unpacklo_epi64(m[4], m[5]); // punpcklqdq m4, m5 ;  4  5 (interleaved)
    m[5] = _mm256_unpacklo_epi64(m[14], m[15]); // punpcklqdq m5, m14, m15 ; 10 11 (interleaved)
    m[14] = _mm256_unpackhi_epi64(m[14], m[15]); // punpckhqdq m14, m15 ; 26 27 (interleaved)
    st_p6(r5, -32 * 1, m[8]); // mova [r5-32*1], m8
    st_p6(r5, -32 * 2, m[14]); // mova [r5-32*2], m14
    idct32_end::<4>(m, r6, r5, r4, 6, 15, 8, 9, 10, true); // IDCT32_END 6, 15, 8, 9, 10, 4
    idct32_end::<4>(m, r6, r5, r4, 7, 14, 8, 9, 10, true); // IDCT32_END 7, 14, 8, 9, 10, 4
    m[8] = _mm256_unpackhi_epi64(m[6], m[7]); // punpckhqdq m8, m6, m7 ; 22 23 (interleaved)
    m[6] = _mm256_unpacklo_epi64(m[6], m[7]); // punpcklqdq m6, m7 ;  6  7 (interleaved)
    m[7] = _mm256_unpacklo_epi64(m[14], m[15]); // punpcklqdq m7, m14, m15 ;  8  9 (interleaved)
    m[14] = _mm256_unpackhi_epi64(m[14], m[15]); // punpckhqdq m14, m15 ; 24 25 (interleaved)
    st_p6(r5, -32 * 3, m[8]); // mova [r5-32*3], m8
    st_p6(r5, -32 * 4, m[14]); // mova [r5-32*4], m14
    m[15] = m[1]; // mova m15, m1
                  // .end:
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m1, m2, q3120
    let mut r6s = 0;
    idct_8x8_internal_12bpc_write_8x4_start(m, stride, &mut r6s); // call m(idct_8x8_internal_12bpc).write_8x4_start
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[4]); // vpermq m0, m4, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[6]); // vpermq m1, m6, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[7]); // vpermq m0, m7, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[5]); // vpermq m1, m5, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(m[3]); // vpermq m0, m3, q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(m[15]); // vpermq m1, m15, q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, 32 * 3)); // vpermq m0, [r5+32*3], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, 32 * 1)); // vpermq m1, [r5+32*1], q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, -32 * 1)); // vpermq m0, [r5-32*1], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, -32 * 3)); // vpermq m1, [r5-32*3], q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, -32 * 4)); // vpermq m0, [r5-32*4], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, -32 * 2)); // vpermq m1, [r5-32*2], q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
    m[0] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, 32 * 0)); // vpermq m0, [r5+32*0], q3120
    m[1] = _mm256_permute4x64_epi64::<0xd8>(ld_p6(r5, 32 * 2)); // vpermq m1, [r5+32*2], q3120
    idct_8x8_internal_10bpc_write_8x4(m, &mut dst, stride, r6s, &mut c); // call m(idct_8x8_internal_10bpc).write_8x4
}

/// `cglobal inv_txfm_add_identity_identity_8x32_12bpc, 4, 7, 8, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_8x32_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    inv_txfm_add_identity_identity_8x32_10bpc_pass1(m, dst, stride, c, eob) // jmp m(inv_txfm_add_identity_identity_8x32_10bpc).pass1
}

// --- 32x8 ---------------------------------------------------------------------------------------------------

/// `.pass2` of `inv_txfm_add_dct_dct_32x8_10bpc`: the 8 bpc 8-point DCT of sixteen columns of words in
/// `m0`-`m7` (`m11` = pw_2048), added to eight rows of 16 pixels; clears sixteen coefficient rows.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_10bpc_pass2(m: &mut M, dst: &mut Dst, stride: isize, c: &mut Coef) {
    idct_16x8_internal_8bpc_main(m); // call m(idct_16x8_internal_8bpc).main
    for x in 0..4 {
        m[x] = _mm256_mulhrs_epi16(m[x], m[11]); // REPX {pmulhrsw x, m11}, m0, m1, m2, m3
    }
    let mut r3 = 0;
    idct_16x8_internal_10bpc_write_16x4_start(m, dst, stride, c, &mut r3); // call m(idct_16x8_internal_10bpc).write_16x4_start
    m[0] = _mm256_mulhrs_epi16(m[11], m[4]); // pmulhrsw m0, m11, m4
    m[1] = _mm256_mulhrs_epi16(m[11], m[5]); // pmulhrsw m1, m11, m5
    m[2] = _mm256_mulhrs_epi16(m[11], m[6]); // pmulhrsw m2, m11, m6
    m[3] = _mm256_mulhrs_epi16(m[11], m[7]); // pmulhrsw m3, m11, m7
    idct_16x8_internal_10bpc_write_16x4_zero(m, dst, stride, c, r3); // jmp m(idct_16x8_internal_10bpc).write_16x4_zero
}

/// `.pass1` of `inv_txfm_add_dct_dct_32x8_10bpc`: the 32-point DCT of the 32 coefficient rows (eight columns),
/// `m12`/`m13` the clips (set by the caller), frame at `r6`; leaves `r6`, `r4`, `r5` and the registers as
/// `.main_end` expects them.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_10bpc_pass1(
    m: &mut M,
    c: Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    for (i, row) in [1isize, 7, 9, 15, 17, 23, 25, 31].into_iter().enumerate() {
        m[i] = ldc_p6(c, 32 * row); // mova mi, [cq+32*row]: m0 [cq+32* 1], m1 [cq+32* 7], …, m7 [cq+32*31]
    }
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1
    for (i, row) in [3isize, 5, 11, 13, 19, 21, 27, 29].into_iter().enumerate() {
        m[i] = ldc_p6(c, 32 * row); // mova mi, [cq+32*row]: m0 [cq+32* 3], m1 [cq+32* 5], …, m7 [cq+32*29]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2
    for (i, row) in [2isize, 6, 10, 14, 18, 22, 26, 30].into_iter().enumerate() {
        m[i] = ldc_p6(c, 32 * row); // mova mi, [cq+32*row]: m0 [cq+32* 2], m1 [cq+32* 6], …, m7 [cq+32*30]
    }
    idct_8x16_internal_10bpc_main_oddhalf(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
    for (i, row) in [0isize, 4, 8, 12, 16, 20, 24, 28].into_iter().enumerate() {
        m[i] = ldc_p6(c, 32 * row); // mova mi, [cq+32*row]: m0 [cq+32* 0], m1 [cq+32* 4], …, m7 [cq+32*28]
    }
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
}

/// `cglobal inv_txfm_add_dct_dct_32x8_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_10bpc(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jnz .full
        let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 8; // or r3d, 8
        return inv_txfm_add_dct_dct_32x8_10bpc_dconly(m, dst, stride, r6d, r3d);
        // .dconly
    }
    // .full: PROLOGUE 0, 7, 16, 32*24: the frame [rsp+32*0..23], zeroed whole (a zero-initialized array), reached through
    // `r6`, `r4`, `r5` only.
    let mut frame = [_mm256_setzero_si256(); 24];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    let mut r4: *mut __m256i = core::ptr::null_mut();
    let mut r5: *mut __m256i = core::ptr::null_mut();
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    inv_txfm_add_dct_dct_32x8_10bpc_pass1(m, c, &mut r6, &mut r4, &mut r5); // call .pass1
    inv_txfm_add_dct_dct_8x32_10bpc_main_end(m, r6, r5, r4); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_end
                                                             // lea r6, [deint_shuf+128]
    m[11] = pw(2048); // vpbroadcastd m11, [pw_2048]
    let r4d = dst; // mov r4, dstq
    inv_txfm_add_dct_dct_32x8_10bpc_pass2(m, &mut dst, stride, &mut c); // call .pass2
    m[0] = ld_p6(r5, 32 * 3); // mova m0, [r5+32*3] ; 16 17
    m[1] = ld_p6(r5, 32 * 2); // mova m1, [r5+32*2] ; 30 31
    m[2] = ld_p6(r5, 32 * 1); // mova m2, [r5+32*1] ; 18 19
    m[3] = ld_p6(r5, 32 * 0); // mova m3, [r5+32*0] ; 28 29
    m[4] = ld_p6(r5, -32 * 1); // mova m4, [r5-32*1] ; 20 21
    m[5] = ld_p6(r5, -32 * 2); // mova m5, [r5-32*2] ; 26 27
    m[6] = ld_p6(r5, -32 * 3); // mova m6, [r5-32*3] ; 22 23
    m[7] = ld_p6(r5, -32 * 4); // mova m7, [r5-32*4] ; 24 25
    inv_txfm_add_dct_dct_8x32_10bpc_transpose(m); // call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    dst = at(r4d, 32); // lea dstq, [r4+32]
    inv_txfm_add_dct_dct_32x8_10bpc_pass2(m, &mut dst, stride, &mut c); // call .pass2
}

/// `cglobal inv_txfm_add_identity_identity_32x8_10bpc, 4, 7, 8, dst, stride, c, eob` from `.pass1` (`m7` =
/// pixel max, set by the 10 and 12 bpc entry points)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x8_10bpc_pass1(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    eob: c_int,
) {
    m[5] = pw(4096); // vpbroadcastd m5, [pw_4096]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    let r6d = eob; // mov r6d, eobd
    let (b, carry) = (eob as u8).overflowing_add(21); // add eobb, 21
    let mut eobd = (eob & !0xff) | b as i32;
    if carry {
        eobd = r6d; // cmovc eobd, r6d
    }
    let r6 = stride * 3; // lea r6, [strideq*3]
    let r5 = stride * 5; // lea r5, [strideq*5]
    let r4 = stride + r6 * 2; // lea r4, [strideq+r6*2] ; strideq*7
    loop {
        // .loop:
        m[0] = ldc_p6(c, 32 * 0); // mova m0, [cq+32*0]
        m[0] = _mm256_packs_epi32(m[0], ldc_p6(c, 32 * 1)); // packssdw m0, [cq+32*1]
        m[1] = ldc_p6(c, 32 * 2); // mova m1, [cq+32*2]
        m[1] = _mm256_packs_epi32(m[1], ldc_p6(c, 32 * 3)); // packssdw m1, [cq+32*3]
        for i in 0..4 {
            stc_p6(c, 32 * i, m[6]); // REPX {mova [cq+32*x], m6}, 0, 1, 2, 3
        }
        c = at(c, 32 * 8); // add cq, 32*8
        m[2] = ldc_p6(c, -32 * 4); // mova m2, [cq-32*4]
        m[2] = _mm256_packs_epi32(m[2], ldc_p6(c, -32 * 3)); // packssdw m2, [cq-32*3]
        m[3] = ldc_p6(c, -32 * 2); // mova m3, [cq-32*2]
        m[3] = _mm256_packs_epi32(m[3], ldc_p6(c, -32 * 1)); // packssdw m3, [cq-32*1]
        for x in 0..4 {
            m[x] = _mm256_mulhrs_epi16(m[x], m[5]); // REPX {pmulhrsw x, m5}, m0, m1, m2, m3
        }
        for i in [-4isize, -3, -2, -1] {
            stc_p6(c, 32 * i, m[6]); // REPX {mova [cq+32*x], m6}, -4, -3, -2, -1
        }
        inv_txfm_add_identity_identity_8x32_10bpc_main(m, dst, stride, r5, r6, r4); // call m(inv_txfm_add_identity_identity_8x32_10bpc).main
        dst = at(dst, 16); // add dstq, 16
        eobd -= 64; // sub eobd, 64
        if eobd < 0 {
            break; // jge .loop
        }
    }
}

/// `cglobal inv_txfm_add_identity_identity_32x8_10bpc, 4, 7, 8, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x8_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    inv_txfm_add_identity_identity_32x8_10bpc_pass1(m, dst, stride, c, eob)
}

/// `cglobal inv_txfm_add_dct_dct_32x8_12bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x8_12bpc(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jnz .full
        let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7000); // vpbroadcastd m3, [dconly_12bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 8; // or r3d, 8
        return inv_txfm_add_dct_dct_32x8_10bpc_dconly(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly
    }
    // .full: PROLOGUE 0, 7, 16, 32*24: the frame [rsp+32*0..23], zeroed whole (a zero-initialized array), reached through
    // `r6`, `r4`, `r5` only.
    let mut frame = [_mm256_setzero_si256(); 24];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    let mut r4: *mut __m256i = core::ptr::null_mut();
    let mut r5: *mut __m256i = core::ptr::null_mut();
    m[12] = pd(-0x80000); // vpbroadcastd m12, [clip_20b_min]
    m[13] = pd(0x7ffff); // vpbroadcastd m13, [clip_20b_max]
    inv_txfm_add_dct_dct_32x8_10bpc_pass1(m, c, &mut r6, &mut r4, &mut r5); // call m(inv_txfm_add_dct_dct_32x8_10bpc).pass1
    inv_txfm_add_dct_dct_8x32_12bpc_main_end(m, r6, r5, r4, c); // call m(inv_txfm_add_dct_dct_8x32_12bpc).main_end
    let r4d = dst; // mov r4, dstq
    idct_16x8_internal_12bpc_pass2_main(m, &mut dst, stride, &mut c); // call m(idct_16x8_internal_12bpc).pass2_main
    for i in 0..16 {
        m[i] = ldc_p6(c, 32 * i as isize); // mova mi, [cq+32*i] ; 16 + i (i = 0 … 15)
    }
    dst = at(r4d, 32); // lea dstq, [r4+32]
    idct_16x8_internal_12bpc_pass2_main(m, &mut dst, stride, &mut c); // call m(idct_16x8_internal_12bpc).pass2_main
}

/// `cglobal inv_txfm_add_identity_identity_32x8_12bpc, 4, 7, 8, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x8_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    inv_txfm_add_identity_identity_32x8_10bpc_pass1(m, dst, stride, c, eob) // jmp m(inv_txfm_add_identity_identity_32x8_10bpc).pass1
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x32_10bpc_avx2,
    inv_txfm_add_dct_dct_8x32_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x32_10bpc_avx2,
    inv_txfm_add_identity_identity_8x32_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_8x32_12bpc_avx2,
    inv_txfm_add_dct_dct_8x32_12bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_8x32_12bpc_avx2,
    inv_txfm_add_identity_identity_8x32_12bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_32x8_10bpc_avx2,
    inv_txfm_add_dct_dct_32x8_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x8_10bpc_avx2,
    inv_txfm_add_identity_identity_32x8_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_32x8_12bpc_avx2,
    inv_txfm_add_dct_dct_32x8_12bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x8_12bpc_avx2,
    inv_txfm_add_identity_identity_32x8_12bpc
);

// --- part 7: itx16_avx2.asm lines 6553-7453 (16x32, 32x16, 32x32) ---

// --- 16x32 ---

/// `%macro IDCT32_PASS2_END coefs[1-2], tmp[1-2], offset[1-2]`: `c1` a register, `src` the address of the second
/// coefficient row; the two rows go to `[dstq+off1]` and `[r2+off2]` (16 pixels), clamped to 0..`m7`.
#[inline(always)]
unsafe fn idct32_pass2_end(
    m: &mut M,
    c1: usize,
    src: *mut __m256i,
    t1: usize,
    t2: usize,
    dst: Dst,
    r2: Dst,
    off1: isize,
    off2: isize,
) {
    m[t2] = src.read(); // mova m%4, [%2]
    m[t1] = _mm256_adds_epi16(m[c1], m[t2]); // paddsw m%3, m%1, m%4
    m[c1] = _mm256_subs_epi16(m[c1], m[t2]); // psubsw m%1, m%4
    if c1 == 0 {
        m[6] = _mm256_setzero_si256(); // pxor m6, m6
    }
    m[t1] = _mm256_mulhrs_epi16(m[t1], m[15]); // pmulhrsw m%3, m15
    m[c1] = _mm256_mulhrs_epi16(m[c1], m[15]); // pmulhrsw m%1, m15
    m[t1] = _mm256_add_epi16(m[t1], _mm256_loadu_si256(at(dst, off1).cast())); // paddw m%3, [dstq+%5]
    m[c1] = _mm256_add_epi16(m[c1], _mm256_loadu_si256(at(r2, off2).cast())); // paddw m%1, [r2+%6]
    m[t1] = _mm256_max_epi16(m[t1], m[6]); // pmaxsw m%3, m6
    m[c1] = _mm256_max_epi16(m[c1], m[6]); // pmaxsw m%1, m6
    m[t1] = _mm256_min_epi16(m[t1], m[7]); // pminsw m%3, m7
    m[c1] = _mm256_min_epi16(m[c1], m[7]); // pminsw m%1, m7
    _mm256_storeu_si256(at(dst, off1).cast(), m[t1]); // mova [dstq+%5], m%3
    _mm256_storeu_si256(at(r2, off2).cast(), m[c1]); // mova [r2+%6], m%1
}

/// `cglobal inv_txfm_add_dct_dct_16x32_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32_10bpc(mut dst: Dst, stride: isize, mut c: Coef, mut eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 32; // or r3d, 32
        r6d = r6d.wrapping_add(128); // add r6d, 128
        r6d >>= 8; // sar r6d, 8
        r6d = r6d.wrapping_mul(181); // imul r6d, 181
        return inv_txfm_add_dct_dct_16x4_10bpc_dconly2(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly2
    }
    // PROLOGUE 0, 8, 16, 32*36, dst, stride, c, eob
    // `[rsp+32*0..2]` (the spill slots the 8 bpc subroutines share) are `s`, `[rsp+32*3]` (`.pass2_end`'s fourth
    // slot) is `s3`; the frame `[rsp+32*0..35]` holds the rows (its first four slots are reached by `.main`'s last
    // call as scratch, never through `s`; no value crosses between the two).
    let mut s = [_mm256_setzero_si256(); 3];
    let s = &mut s;
    let mut s3 = _mm256_setzero_si256();
    let mut frame = [_mm256_setzero_si256(); 36];
    let rsp: *mut __m256i = frame.as_mut_ptr(); // zeroed whole (module documentation)
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    let mut r6 = at(rsp, 32 * 16); // lea r6, [rsp+32*16]
    let r4 = at(r6, 32 * 8); // lea r4, [r6+32*8]
    let r5 = at(r6, 32 * 16); // lea r5, [r6+32*16]
    inv_txfm_add_dct_dct_16x32_10bpc_main(m, &mut c, &mut r6); // call .main
    let fast = 'eob: {
        eob -= 44; // sub eobd, 44
        if eob < 0 {
            // jge .eob44
            m[2] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m2, m0, m3, 0x31 ; 5
            m[0] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m0, xm3, 1 ; 1
            m[3] = _mm256_permute2x128_si256::<0x31>(m[1], m[4]); // vperm2i128 m3, m1, m4, 0x31 ; 7
            m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[4])); // vinserti128 m1, xm4, 1 ; 3
            m[4] = _mm256_setzero_si256(); // pxor m4, m4
            for x in [5, 6, 7] {
                m[x] = m[4]; // REPX {mova x, m4}, m5, m6, m7
            }
            for x in [0, 1, 2, 3] {
                at(r6, 32 * x).write(m[4]); // REPX {mova [r6+32*x], m4}, 0, 1, 2, 3
            }
            break 'eob true; // jmp .fast
        }
        // .eob44:
        _mm_store_si128(at(r4, 16 * 0).cast(), lo(m[0])); // mova [r4+16*0], xm0
        _mm_store_si128(at(r4, 16 * 1).cast(), lo(m[3])); // mova [r4+16*1], xm3
        _mm_store_si128(at(r4, 16 * 2).cast(), lo(m[1])); // mova [r4+16*2], xm1
        _mm_store_si128(at(r4, 16 * 3).cast(), lo(m[4])); // mova [r4+16*3], xm4
        _mm_store_si128(at(r4, 16 * 4).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [r4+16*4], m0, 1
        _mm_store_si128(at(r4, 16 * 5).cast(), _mm256_extracti128_si256::<1>(m[3])); // vextracti128 [r4+16*5], m3, 1
        _mm_store_si128(at(r4, 16 * 6).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [r4+16*6], m1, 1
        _mm_store_si128(at(r4, 16 * 7).cast(), _mm256_extracti128_si256::<1>(m[4])); // vextracti128 [r4+16*7], m4, 1
        inv_txfm_add_dct_dct_16x32_10bpc_main(m, &mut c, &mut r6); // call .main
        eob -= 107; // sub eobd, 107
        if eob < 0 {
            // jge .eob151
            m[7] = _mm256_permute2x128_si256::<0x31>(m[1], m[4]); // vperm2i128 m7, m1, m4, 0x31 ; 15
            m[5] = _mm256_inserti128_si256::<1>(m[1], lo(m[4])); // vinserti128 m5, m1, xm4, 1 ; 11
            m[6] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m6, m0, m3, 0x31 ; 13
            m[4] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m4, m0, xm3, 1 ; 9
            m[0] = at(r4, 32 * 0).read(); // mova m0, [r4+32*0]
            m[1] = at(r4, 32 * 1).read(); // mova m1, [r4+32*1]
            m[2] = at(r4, 32 * 2).read(); // mova m2, [r4+32*2]
            m[3] = at(r4, 32 * 3).read(); // mova m3, [r4+32*3]
            break 'eob true;
        }
        // .eob151:
        _mm_store_si128(at(r4, -16 * 8).cast(), lo(m[0])); // mova [r4-16*8], xm0
        _mm_store_si128(at(r4, -16 * 7).cast(), lo(m[3])); // mova [r4-16*7], xm3
        _mm_store_si128(at(r4, -16 * 6).cast(), lo(m[1])); // mova [r4-16*6], xm1
        _mm_store_si128(at(r4, -16 * 5).cast(), lo(m[4])); // mova [r4-16*5], xm4
        _mm_store_si128(at(r4, -16 * 4).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [r4-16*4], m0, 1
        _mm_store_si128(at(r4, -16 * 3).cast(), _mm256_extracti128_si256::<1>(m[3])); // vextracti128 [r4-16*3], m3, 1
        _mm_store_si128(at(r4, -16 * 2).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [r4-16*2], m1, 1
        _mm_store_si128(at(r4, -16 * 1).cast(), _mm256_extracti128_si256::<1>(m[4])); // vextracti128 [r4-16*1], m4, 1
        inv_txfm_add_dct_dct_16x32_10bpc_main(m, &mut c, &mut r6); // call .main
        eob -= 128; // sub eobd, 128
        if eob < 0 {
            // jge .eob279
            m[10] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m10, m0, m3, 0x31 ; 21
            m[8] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m8, m0, xm3, 1 ; 17
            m[11] = _mm256_permute2x128_si256::<0x31>(m[1], m[4]); // vperm2i128 m11, m1, m4, 0x31 ; 23
            m[9] = _mm256_inserti128_si256::<1>(m[1], lo(m[4])); // vinserti128 m9, m1, xm4, 1 ; 19
            m[12] = _mm256_setzero_si256(); // pxor m12, m12
            for x in [13, 14, 15] {
                m[x] = m[12]; // REPX {mova x, m12}, m13, m14, m15
            }
            for x in [0, 1, 2, 3] {
                at(r6, 32 * x).write(m[12]); // REPX {mova [r6+32*x], m12}, 0, 1, 2, 3
            }
            break 'eob false; // jmp .full
        }
        // .eob279:
        _mm_store_si128(at(r5, 16 * 0).cast(), lo(m[0])); // mova [r5+16*0], xm0
        _mm_store_si128(at(r5, 16 * 1).cast(), lo(m[3])); // mova [r5+16*1], xm3
        _mm_store_si128(at(r5, 16 * 2).cast(), lo(m[1])); // mova [r5+16*2], xm1
        _mm_store_si128(at(r5, 16 * 3).cast(), lo(m[4])); // mova [r5+16*3], xm4
        _mm_store_si128(at(r5, 16 * 4).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [r5+16*4], m0, 1
        _mm_store_si128(at(r5, 16 * 5).cast(), _mm256_extracti128_si256::<1>(m[3])); // vextracti128 [r5+16*5], m3, 1
        _mm_store_si128(at(r5, 16 * 6).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [r5+16*6], m1, 1
        _mm_store_si128(at(r5, 16 * 7).cast(), _mm256_extracti128_si256::<1>(m[4])); // vextracti128 [r5+16*7], m4, 1
        inv_txfm_add_dct_dct_16x32_10bpc_main(m, &mut c, &mut r6); // call .main
        m[14] = _mm256_permute2x128_si256::<0x31>(m[0], m[3]); // vperm2i128 m14, m0, m3, 0x31 ; 29
        m[12] = _mm256_inserti128_si256::<1>(m[0], lo(m[3])); // vinserti128 m12, m0, xm3, 1 ; 25
        m[15] = _mm256_permute2x128_si256::<0x31>(m[1], m[4]); // vperm2i128 m15, m1, m4, 0x31 ; 31
        m[13] = _mm256_inserti128_si256::<1>(m[1], lo(m[4])); // vinserti128 m13, m1, xm4, 1 ; 27
        m[8] = at(r5, 32 * 0).read(); // mova m8, [r5+32*0]
        m[9] = at(r5, 32 * 1).read(); // mova m9, [r5+32*1]
        m[10] = at(r5, 32 * 2).read(); // mova m10, [r5+32*2]
        m[11] = at(r5, 32 * 3).read(); // mova m11, [r5+32*3]
        false
    };
    if fast {
        // .fast:
        // lea r6, [pw_5+128]
        inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf_fast(m, s, r4, r5); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14, 15] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, m15
        }
        // jmp .idct16
    } else {
        // .full:
        m[0] = at(r4, 32 * 0).read(); // mova m0, [r4+32*0]
        m[1] = at(r4, 32 * 1).read(); // mova m1, [r4+32*1]
        m[2] = at(r4, 32 * 2).read(); // mova m2, [r4+32*2]
        m[3] = at(r4, 32 * 3).read(); // mova m3, [r4+32*3]
        m[4] = at(r4, -32 * 4).read(); // mova m4, [r4-32*4]
        m[5] = at(r4, -32 * 3).read(); // mova m5, [r4-32*3]
        m[6] = at(r4, -32 * 2).read(); // mova m6, [r4-32*2]
        m[7] = at(r4, -32 * 1).read(); // mova m7, [r4-32*1]
                                       // lea r6, [pw_5 + 128]
        inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf(m, s, r4, r5); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
        let r3 = at(rsp, 32 * 8); // lea r3, [rsp+32*8]
        m[8] = at(r3, 32 * 0).read(); // mova m8, [r3+32*0]
        m[9] = at(r3, 32 * 1).read(); // mova m9, [r3+32*1]
        m[10] = at(r3, 32 * 2).read(); // mova m10, [r3+32*2]
        m[11] = at(r3, 32 * 3).read(); // mova m11, [r3+32*3]
        m[12] = at(r3, -32 * 4).read(); // mova m12, [r3-32*4]
        m[13] = at(r3, -32 * 3).read(); // mova m13, [r3-32*3]
        m[14] = at(r3, -32 * 2).read(); // mova m14, [r3-32*2]
        m[15] = at(r3, -32 * 1).read(); // mova m15, [r3-32*1]
    }
    // .idct16:
    let r3 = at(rsp, 32 * 16); // lea r3, [rsp+32*16]
    m[0] = at(r3, 32 * 0).read(); // mova m0, [r3+32*0]
    m[1] = at(r3, 32 * 1).read(); // mova m1, [r3+32*1]
    m[2] = at(r3, 32 * 2).read(); // mova m2, [r3+32*2]
    m[3] = at(r3, 32 * 3).read(); // mova m3, [r3+32*3]
    m[4] = at(r3, -32 * 4).read(); // mova m4, [r3-32*4]
    m[5] = at(r3, -32 * 3).read(); // mova m5, [r3-32*3]
    m[6] = at(r3, -32 * 2).read(); // mova m6, [r3-32*2]
    m[7] = at(r3, -32 * 1).read(); // mova m7, [r3-32*1]
    s[0] = m[15]; // mova [rsp], m15
    idct_16x16_internal_8bpc_main(m, s); // call m(idct_16x16_internal_8bpc).main
    let mut r2 = at(dst, stride * 19); // imul r2, strideq, 19 / add r2, dstq
    let r3 = stride * 3; // lea r3, [strideq*3]
    inv_txfm_add_dct_dct_16x32_10bpc_pass2_end(m, s, &mut s3, r5, &mut dst, stride, &mut r2, r3, r4);
    // call .pass2_end
}

/// `.main` of `inv_txfm_add_dct_dct_16x32_10bpc`: the first pass of eight columns (`[cq+128*0..15]`, rect2),
/// `m11` = `pd_2048`, `m12`/`m13` the clip bounds, `m14` = `pd_2896`. Clears the coefficients it read, advances
/// `cq` by 32, stores the 16-bit rows 0-3 and 8-11 to `[r6+16*0..7]`, leaves the others in `m0`, `m1`, `m3`,
/// `m4` and lowers `r6` by `32*4`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32_10bpc_main(m: &mut M, cq: &mut Coef, r6: &mut *mut __m256i) {
    let c = *cq;
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 1).cast())); // pmulld m0, m14, [cq+128* 1]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 3).cast())); // pmulld m1, m14, [cq+128* 3]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 5).cast())); // pmulld m2, m14, [cq+128* 5]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 7).cast())); // pmulld m3, m14, [cq+128* 7]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 9).cast())); // pmulld m4, m14, [cq+128* 9]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 11).cast())); // pmulld m5, m14, [cq+128*11]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 13).cast())); // pmulld m6, m14, [cq+128*13]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 15).cast())); // pmulld m7, m14, [cq+128*15]
    idct_8x16_internal_10bpc_main_oddhalf_rect2(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 0).cast())); // pmulld m0, m14, [cq+128* 0]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 2).cast())); // pmulld m1, m14, [cq+128* 2]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 4).cast())); // pmulld m2, m14, [cq+128* 4]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 6).cast())); // pmulld m3, m14, [cq+128* 6]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 8).cast())); // pmulld m4, m14, [cq+128* 8]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 10).cast())); // pmulld m5, m14, [cq+128*10]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 12).cast())); // pmulld m6, m14, [cq+128*12]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 128 * 14).cast())); // pmulld m7, m14, [cq+128*14]
    idct_8x8_internal_10bpc_main_rect2(m); // call m(idct_8x8_internal_10bpc).main_rect2
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[15] = _mm256_srli_epi32::<11>(m[11]); // psrld m15, m11, 11 ; pd_1
    m[8] = at(*r6, -32 * 4).read(); // mova m8, [r6-32*4]
    m[9] = at(*r6, -32 * 3).read(); // mova m9, [r6-32*3]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[15]); // REPX {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[10] = _mm256_sub_epi32(m[0], m[8]); // psubd m10, m0, m8 ; out15
    m[0] = _mm256_add_epi32(m[0], m[8]); // paddd m0, m8 ; out0
    m[8] = at(*r6, -32 * 2).read(); // mova m8, [r6-32*2]
    m[15] = _mm256_add_epi32(m[1], m[9]); // paddd m15, m1, m9 ; out1
    m[1] = _mm256_sub_epi32(m[1], m[9]); // psubd m1, m9 ; out14
    m[9] = at(*r6, -32 * 1).read(); // mova m9, [r6-32*1]
    for x in [0, 15, 10, 1] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m0, m15, m10, m1
    }
    m[0] = _mm256_packs_epi32(m[0], m[15]); // packssdw m0, m15
    m[1] = _mm256_packs_epi32(m[1], m[10]); // packssdw m1, m10
    m[10] = _mm256_sub_epi32(m[2], m[8]); // psubd m10, m2, m8 ; out13
    m[2] = _mm256_add_epi32(m[2], m[8]); // paddd m2, m8 ; out2
    m[8] = at(*r6, 32 * 0).read(); // mova m8, [r6+32*0]
    m[15] = _mm256_add_epi32(m[3], m[9]); // paddd m15, m3, m9 ; out3
    m[3] = _mm256_sub_epi32(m[3], m[9]); // psubd m3, m9 ; out12
    m[9] = at(*r6, 32 * 1).read(); // mova m9, [r6+32*1]
    for x in [2, 15, 10, 3] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m2, m15, m10, m3
    }
    m[2] = _mm256_packs_epi32(m[2], m[15]); // packssdw m2, m15
    m[3] = _mm256_packs_epi32(m[3], m[10]); // packssdw m3, m10
    m[10] = _mm256_sub_epi32(m[4], m[8]); // psubd m10, m4, m8 ; out11
    m[4] = _mm256_add_epi32(m[4], m[8]); // paddd m4, m8 ; out4
    m[8] = at(*r6, 32 * 2).read(); // mova m8, [r6+32*2]
    m[15] = _mm256_add_epi32(m[5], m[9]); // paddd m15, m5, m9 ; out5
    m[5] = _mm256_sub_epi32(m[5], m[9]); // psubd m5, m9 ; out10
    m[9] = at(*r6, 32 * 3).read(); // mova m9, [r6+32*3]
    for x in [4, 10, 15, 5] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m4, m10, m15, m5
    }
    m[4] = _mm256_packs_epi32(m[4], m[15]); // packssdw m4, m15
    m[5] = _mm256_packs_epi32(m[5], m[10]); // packssdw m5, m10
    m[10] = _mm256_sub_epi32(m[6], m[8]); // psubd m10, m6, m8 ; out9
    m[6] = _mm256_add_epi32(m[6], m[8]); // paddd m6, m8 ; out6
    m[15] = _mm256_add_epi32(m[7], m[9]); // paddd m15, m7, m9 ; out7
    m[7] = _mm256_sub_epi32(m[7], m[9]); // psubd m7, m9 ; out8
    for x in [6, 10, 15, 7] {
        m[x] = _mm256_srai_epi32::<1>(m[x]); // REPX {psrad x, 1}, m6, m10, m15, m7
    }
    m[6] = _mm256_packs_epi32(m[6], m[15]); // packssdw m6, m15
    m[7] = _mm256_packs_epi32(m[7], m[10]); // packssdw m7, m10
    m[8] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m8, m0, m2
    m[0] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m0, m2
    m[2] = _mm256_unpackhi_epi16(m[3], m[1]); // punpckhwd m2, m3, m1
    m[3] = _mm256_unpacklo_epi16(m[3], m[1]); // punpcklwd m3, m1
    m[1] = _mm256_unpackhi_epi16(m[4], m[6]); // punpckhwd m1, m4, m6
    m[4] = _mm256_unpacklo_epi16(m[4], m[6]); // punpcklwd m4, m6
    m[6] = _mm256_unpacklo_epi16(m[7], m[5]); // punpcklwd m6, m7, m5
    m[7] = _mm256_unpackhi_epi16(m[7], m[5]); // punpckhwd m7, m5
    m[5] = _mm256_setzero_si256(); // pxor m5, m5
    let mut r7d: i32 = 128 * 13; // mov r7d, 128*13
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        _mm256_storeu_si256(at(c, r7 - 128 * 1).cast(), m[5]); // mova [cq+r7-128*1], m5
        _mm256_storeu_si256(at(c, r7 + 128 * 0).cast(), m[5]); // mova [cq+r7+128*0], m5
        _mm256_storeu_si256(at(c, r7 + 128 * 1).cast(), m[5]); // mova [cq+r7+128*1], m5
        _mm256_storeu_si256(at(c, r7 + 128 * 2).cast(), m[5]); // mova [cq+r7+128*2], m5
        r7d -= 128 * 4; // sub r7d, 128*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    *cq = at(c, 32); // add cq, 32
    m[5] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m5, m3, m2
    m[3] = _mm256_unpackhi_epi16(m[3], m[2]); // punpckhwd m3, m2
    m[2] = _mm256_unpacklo_epi16(m[4], m[1]); // punpcklwd m2, m4, m1
    m[4] = _mm256_unpackhi_epi16(m[4], m[1]); // punpckhwd m4, m1
    m[1] = _mm256_unpackhi_epi16(m[0], m[8]); // punpckhwd m1, m0, m8
    m[0] = _mm256_unpacklo_epi16(m[0], m[8]); // punpcklwd m0, m8
    m[8] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m8, m6, m7
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
    m[7] = _mm256_unpacklo_epi64(m[1], m[4]); // punpcklqdq m7, m1, m4
    m[1] = _mm256_unpackhi_epi64(m[1], m[4]); // punpckhqdq m1, m4
    m[4] = _mm256_unpackhi_epi64(m[8], m[3]); // punpckhqdq m4, m8, m3
    m[8] = _mm256_unpacklo_epi64(m[8], m[3]); // punpcklqdq m8, m3
    m[3] = _mm256_unpackhi_epi64(m[6], m[5]); // punpckhqdq m3, m6, m5
    m[6] = _mm256_unpacklo_epi64(m[6], m[5]); // punpcklqdq m6, m5
    m[5] = _mm256_unpacklo_epi64(m[0], m[2]); // punpcklqdq m5, m0, m2
    m[0] = _mm256_unpackhi_epi64(m[0], m[2]); // punpckhqdq m0, m2
    _mm_store_si128(at(*r6, 16 * 0).cast(), lo(m[5])); // mova [r6+16*0], xm5
    _mm_store_si128(at(*r6, 16 * 1).cast(), lo(m[6])); // mova [r6+16*1], xm6
    _mm_store_si128(at(*r6, 16 * 2).cast(), lo(m[7])); // mova [r6+16*2], xm7
    _mm_store_si128(at(*r6, 16 * 3).cast(), lo(m[8])); // mova [r6+16*3], xm8
    _mm_store_si128(at(*r6, 16 * 4).cast(), _mm256_extracti128_si256::<1>(m[5])); // vextracti128 [r6+16*4], m5, 1
    _mm_store_si128(at(*r6, 16 * 5).cast(), _mm256_extracti128_si256::<1>(m[6])); // vextracti128 [r6+16*5], m6, 1
    _mm_store_si128(at(*r6, 16 * 6).cast(), _mm256_extracti128_si256::<1>(m[7])); // vextracti128 [r6+16*6], m7, 1
    _mm_store_si128(at(*r6, 16 * 7).cast(), _mm256_extracti128_si256::<1>(m[8])); // vextracti128 [r6+16*7], m8, 1
    *r6 = at(*r6, -32 * 4); // sub r6, 32*4
}

/// `.pass2_end` of `inv_txfm_add_dct_dct_16x32_10bpc`: adds the 32 rows of a 16-wide column, the sums and
/// differences of the even half in `m0`-`m15` (16-bit, with out1 in `s[1]`, as `m(idct_16x16_internal_8bpc).main`
/// leaves them) and the odd half at `[r5-32*4..+3]`/`[r4-32*4..+3]`, to `dst` (rows 0-15 from `dst`, rows 16-31
/// from `r2` = `dst+strideq*19` upwards, `r3` = `strideq*3`). `s` are `[rsp+gprsize+32*0..2]`, `s3`
/// `[rsp+gprsize+32*3]`; `dst` ends three rows further, `r2` three rows back.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x32_10bpc_pass2_end(
    m: &mut M,
    s: &mut [__m256i; 3],
    s3: &mut __m256i,
    r5: *mut __m256i,
    dst: &mut Dst,
    stride: isize,
    r2: &mut Dst,
    r3: isize,
    r4: *mut __m256i,
) {
    s[0] = m[6]; // mova [rsp+gprsize+32*0], m6
    s[2] = m[7]; // mova [rsp+gprsize+32*2], m7
    *s3 = m[15]; // mova [rsp+gprsize+32*3], m15
    m[15] = pw(2048); // vpbroadcastd m15, [pw_2048]
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    idct32_pass2_end(m, 0, at(r5, 32 * 3), 1, 6, *dst, *r2, stride * 0, r3 * 4); // IDCT32_PASS2_END 0, r5+32*3, 1, 6, strideq*0, r3*4
    idct32_pass2_end(m, 4, at(r5, -32 * 1), 0, 1, *dst, *r2, stride * 4, stride * 8); // IDCT32_PASS2_END 4, r5-32*1, 0, 1, strideq*4, strideq*8
    idct32_pass2_end(m, 8, at(r4, 32 * 3), 0, 4, *dst, *r2, stride * 8, stride * 4); // IDCT32_PASS2_END 8, r4+32*3, 0, 4, strideq*8, strideq*4
    idct32_pass2_end(m, 12, at(r4, -32 * 1), 0, 4, *dst, *r2, r3 * 4, stride * 0); // IDCT32_PASS2_END 12, r4-32*1, 0, 4, r3*4, strideq*0
    *dst = at(*dst, stride); // add dstq, strideq
    *r2 = at(*r2, -stride); // sub r2, strideq
    m[1] = s[1]; // mova m1, [rsp+gprsize+32*1]
    idct32_pass2_end(m, 1, at(r5, 32 * 2), 0, 4, *dst, *r2, stride * 0, r3 * 4); // IDCT32_PASS2_END 1, r5+32*2, 0, 4, strideq*0, r3*4
    idct32_pass2_end(m, 5, at(r5, -32 * 2), 0, 4, *dst, *r2, stride * 4, stride * 8); // IDCT32_PASS2_END 5, r5-32*2, 0, 4, strideq*4, strideq*8
    idct32_pass2_end(m, 9, at(r4, 32 * 2), 0, 4, *dst, *r2, stride * 8, stride * 4); // IDCT32_PASS2_END 9, r4+32*2, 0, 4, strideq*8, strideq*4
    idct32_pass2_end(m, 13, at(r4, -32 * 2), 0, 4, *dst, *r2, r3 * 4, stride * 0); // IDCT32_PASS2_END 13, r4-32*2, 0, 4, r3*4, strideq*0
    *dst = at(*dst, stride); // add dstq, strideq
    *r2 = at(*r2, -stride); // sub r2, strideq
    m[1] = s[0]; // mova m1, [rsp+gprsize+32*0]
    idct32_pass2_end(m, 2, at(r5, 32 * 1), 0, 4, *dst, *r2, stride * 0, r3 * 4); // IDCT32_PASS2_END 2, r5+32*1, 0, 4, strideq*0, r3*4
    idct32_pass2_end(m, 1, at(r5, -32 * 3), 0, 4, *dst, *r2, stride * 4, stride * 8); // IDCT32_PASS2_END 1, r5-32*3, 0, 4, strideq*4, strideq*8
    idct32_pass2_end(m, 10, at(r4, 32 * 1), 0, 4, *dst, *r2, stride * 8, stride * 4); // IDCT32_PASS2_END 10, r4+32*1, 0, 4, strideq*8, strideq*4
    idct32_pass2_end(m, 14, at(r4, -32 * 3), 0, 4, *dst, *r2, r3 * 4, stride * 0); // IDCT32_PASS2_END 14, r4-32*3, 0, 4, r3*4, strideq*0
    *dst = at(*dst, stride); // add dstq, strideq
    *r2 = at(*r2, -stride); // sub r2, strideq
    m[1] = s[2]; // mova m1, [rsp+gprsize+32*2]
    m[2] = *s3; // mova m2, [rsp+gprsize+32*3]
    idct32_pass2_end(m, 3, at(r5, 32 * 0), 0, 4, *dst, *r2, stride * 0, r3 * 4); // IDCT32_PASS2_END 3, r5+32*0, 0, 4, strideq*0, r3*4
    idct32_pass2_end(m, 1, at(r5, -32 * 4), 0, 4, *dst, *r2, stride * 4, stride * 8); // IDCT32_PASS2_END 1, r5-32*4, 0, 4, strideq*4, strideq*8
    idct32_pass2_end(m, 11, at(r4, 32 * 0), 0, 4, *dst, *r2, stride * 8, stride * 4); // IDCT32_PASS2_END 11, r4+32*0, 0, 4, strideq*8, strideq*4
    idct32_pass2_end(m, 2, at(r4, -32 * 4), 0, 4, *dst, *r2, r3 * 4, stride * 0);
    // IDCT32_PASS2_END 2, r4-32*4, 0, 4, r3*4, strideq*0
}

/// `cglobal inv_txfm_add_identity_identity_16x32_10bpc, 4, 7, 12, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_16x32_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    inv_txfm_add_identity_identity_16x32_10bpc_pass1(m, dst, stride, c, eob)
}

/// `cglobal inv_txfm_add_identity_identity_16x32_12bpc, 4, 7, 12, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_16x32_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    inv_txfm_add_identity_identity_16x32_10bpc_pass1(m, dst, stride, c, eob) // jmp m(inv_txfm_add_identity_identity_16x32_10bpc).pass1
}

/// `.pass1` of `inv_txfm_add_identity_identity_16x32_10bpc` (`m7` = the pixel maximum)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_16x32_10bpc_pass1(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    mut eob: c_int,
) {
    m[8] = pw(2896 * 8); // vpbroadcastd m8, [pw_2896x8]
    m[9] = pw(1697 * 16); // vpbroadcastd m9, [pw_1697x16]
    m[11] = pw(8192); // vpbroadcastd m11, [pw_8192]
    let r6 = stride * 5; // lea r6, [strideq*5]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    m[10] = _mm256_add_epi16(m[11], m[11]); // paddw m10, m11, m11 ; pw_16384
    let mut r5 = dst; // mov r5, dstq
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 36; // sub eobd, 36
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 128 * 8); // add cq, 128*8
    dst = at(r5, 16); // lea dstq, [r5+16]
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    c = at(c, -(128 * 8 - 32)); // sub cq, 128*8-32
    dst = at(r5, stride * 8); // lea dstq, [r5+strideq*8]
    r5 = dst; // mov r5, dstq
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 107; // sub eobd, 107 ; eob < 143
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 128 * 8); // add cq, 128*8
    dst = at(r5, 16); // lea dstq, [r5+16]
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    c = at(c, -(128 * 8 - 32)); // sub cq, 128*8-32
    dst = at(r5, stride * 8); // lea dstq, [r5+strideq*8]
    r5 = dst; // mov r5, dstq
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 128; // sub eobd, 128 ; eob < 271
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 128 * 8); // add cq, 128*8
    dst = at(r5, 16); // lea dstq, [r5+16]
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    c = at(c, -(128 * 8 - 32)); // sub cq, 128*8-32
    dst = at(r5, stride * 8); // lea dstq, [r5+strideq*8]
    r5 = dst; // mov r5, dstq
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 128; // sub eobd, 128 ; eob < 399
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 128 * 8); // add cq, 128*8
    dst = at(r5, 16); // lea dstq, [r5+16]
    inv_txfm_add_identity_identity_16x32_10bpc_main(m, c, &mut dst, stride, r6);
    // call .main
    // .ret:
}

/// `.main` of `inv_txfm_add_identity_identity_16x32_10bpc` (falls into `.main2`)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_16x32_10bpc_main(m: &mut M, c: Coef, dst: &mut Dst, stride: isize, r6: isize) {
    m[0] = _mm256_loadu_si256(at(c, 128 * 0).cast()); // mova m0, [cq+128*0]
    m[0] = _mm256_packs_epi32(m[0], _mm256_loadu_si256(at(c, 128 * 1).cast())); // packssdw m0, [cq+128*1]
    m[1] = _mm256_loadu_si256(at(c, 128 * 2).cast()); // mova m1, [cq+128*2]
    m[1] = _mm256_packs_epi32(m[1], _mm256_loadu_si256(at(c, 128 * 3).cast())); // packssdw m1, [cq+128*3]
    m[2] = _mm256_loadu_si256(at(c, 128 * 4).cast()); // mova m2, [cq+128*4]
    m[2] = _mm256_packs_epi32(m[2], _mm256_loadu_si256(at(c, 128 * 5).cast())); // packssdw m2, [cq+128*5]
    m[3] = _mm256_loadu_si256(at(c, 128 * 6).cast()); // mova m3, [cq+128*6]
    m[3] = _mm256_packs_epi32(m[3], _mm256_loadu_si256(at(c, 128 * 7).cast())); // packssdw m3, [cq+128*7]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[8]); // REPX {pmulhrsw x, m8 }, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        idtx16(m, x, 4, 9, Idtx16Shift::Rnd(10)); // REPX {IDTX16 x, 4, 9, 10}, 0, 1, 2, 3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[11]); // REPX {pmulhrsw x, m11}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        _mm256_storeu_si256(at(c, 128 * x).cast(), m[6]); // REPX {mova [cq+128*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    }
    inv_txfm_add_identity_identity_16x32_10bpc_main2(m, dst, stride, r6)
}

/// `.main2` of `inv_txfm_add_identity_identity_16x32_10bpc`: transposes the 16-bit `m0`-`m3` and adds them to
/// eight rows of eight pixels (two `write_2x8x2`), clamped to `m6`..`m7`; `dst` ends four rows further.
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_16x32_10bpc_main2(m: &mut M, dst: &mut Dst, stride: isize, r6: isize) {
    m[4] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m4, m0, m1
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
    m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
    m[3] = _mm256_unpackhi_epi16(m[0], m[4]); // punpckhwd m3, m0, m4
    m[0] = _mm256_unpacklo_epi16(m[0], m[4]); // punpcklwd m0, m4
    m[4] = _mm256_unpacklo_epi16(m[2], m[1]); // punpcklwd m4, m2, m1
    m[2] = _mm256_unpackhi_epi16(m[2], m[1]); // punpckhwd m2, m1
    m[1] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m1, m0, m4
    m[0] = _mm256_unpacklo_epi64(m[0], m[4]); // punpcklqdq m0, m4
    iidentity_8x8_internal_10bpc_write_2x8x2(m, dst, stride, r6); // call m(iidentity_8x8_internal_10bpc).write_2x8x2
    m[0] = _mm256_unpacklo_epi64(m[3], m[2]); // punpcklqdq m0, m3, m2
    m[1] = _mm256_unpackhi_epi64(m[3], m[2]); // punpckhqdq m1, m3, m2
    iidentity_8x8_internal_10bpc_write_2x8x2(m, dst, stride, r6) // jmp m(iidentity_8x8_internal_10bpc).write_2x8x2
}

// --- 32x16 ---

/// `cglobal inv_txfm_add_dct_dct_32x16_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16_10bpc(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 16; // or r3d, 16
        r6d = r6d.wrapping_add(128); // add r6d, 128
        r6d >>= 8; // sar r6d, 8
        r6d = r6d.wrapping_mul(181); // imul r6d, 181
        r6d = r6d.wrapping_add(384); // add r6d, 384
        r6d >>= 9; // sar r6d, 9
        return inv_txfm_add_dct_dct_32x8_10bpc_dconly2(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly2
    }
    // PROLOGUE 0, 8, 16, 32*40, dst, stride, c, eob
    // `[rsp+32*0..2]` as the spill slots of the subroutines (`[rsp]`, `[rsp+gprsize+32*n]`) are `s`; the frame
    // `[rsp+32*0..39]` holds the rows (`.main` and `[r4-32*4..-2]` reach its first three slots as rows; each value
    // is read back through the name it was stored with, so the two never exchange a value).
    let mut s = [_mm256_setzero_si256(); 3];
    let s = &mut s;
    let mut frame = [_mm256_setzero_si256(); 40];
    let rsp: *mut __m256i = frame.as_mut_ptr(); // zeroed whole (module documentation)
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    let mut r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    let mut r4 = rsp; // (set by .main)
    let mut r5 = rsp; // (set by .main)
    inv_txfm_add_dct_dct_32x16_10bpc_main(m, c, &mut r6, &mut r4, &mut r5); // call .main
    let r7;
    if eob < 36 {
        // cmp eobd, 36 / jge .full
        inv_txfm_add_dct_dct_8x32_10bpc_transpose(m); // call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, [rsp]
        }
        s[0] = m[8];
        // lea r6, [pw_5+128]
        r7 = dst; // mov r7, dstq
        idct_16x16_internal_8bpc_main(m, s); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_32x16_10bpc_write_16x16(m, s, &mut dst, stride); // call .write_16x16
        m[0] = at(r5, 32 * 3).read(); // mova m0, [r5+32*3]
        m[1] = at(r5, 32 * 2).read(); // mova m1, [r5+32*2]
        m[2] = at(r5, 32 * 1).read(); // mova m2, [r5+32*1]
        m[3] = at(r5, 32 * 0).read(); // mova m3, [r5+32*0]
        m[4] = at(r5, -32 * 1).read(); // mova m4, [r5-32*1]
        m[5] = at(r5, -32 * 2).read(); // mova m5, [r5-32*2]
        m[6] = at(r5, -32 * 3).read(); // mova m6, [r5-32*3]
        m[7] = at(r5, -32 * 4).read(); // mova m7, [r5-32*4]
        inv_txfm_add_dct_dct_8x32_10bpc_transpose(m); // call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14, [rsp]
        }
        s[0] = m[8];
        // jmp .end
    } else {
        // .full:
        c = at(c, 32); // add cq, 32
        at(r4, 32 * 3).write(m[0]); // mova [r4+32*3], m0
        at(r4, 32 * 2).write(m[1]); // mova [r4+32*2], m1
        at(r4, 32 * 1).write(m[2]); // mova [r4+32*1], m2
        at(r4, 32 * 0).write(m[3]); // mova [r4+32*0], m3
        at(r4, -32 * 1).write(m[4]); // mova [r4-32*1], m4
        at(r4, -32 * 2).write(m[5]); // mova [r4-32*2], m5
        at(r4, -32 * 3).write(m[6]); // mova [r4-32*3], m6
        at(r4, -32 * 4).write(m[7]); // mova [r4-32*4], m7
        inv_txfm_add_dct_dct_32x16_10bpc_main(m, c, &mut r6, &mut r4, &mut r5); // call .main
        r4 = at(r4, -32 * 16); // sub r4, 32*16 ; topleft 16x8
        inv_txfm_add_dct_dct_32x16_10bpc_transpose_16x16(m, r4, s); // call .transpose_16x16
                                                                    // lea r6, [pw_5+128]
        r7 = dst; // mov r7, dstq
        idct_16x16_internal_8bpc_main(m, s); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_32x16_10bpc_write_16x16(m, s, &mut dst, stride); // call .write_16x16
        m[0] = at(r5, 32 * 3).read(); // mova m0, [r5+32*3]
        m[1] = at(r5, 32 * 2).read(); // mova m1, [r5+32*2]
        m[2] = at(r5, 32 * 1).read(); // mova m2, [r5+32*1]
        m[3] = at(r5, 32 * 0).read(); // mova m3, [r5+32*0]
        m[4] = at(r5, -32 * 1).read(); // mova m4, [r5-32*1]
        m[5] = at(r5, -32 * 2).read(); // mova m5, [r5-32*2]
        m[6] = at(r5, -32 * 3).read(); // mova m6, [r5-32*3]
        m[7] = at(r5, -32 * 4).read(); // mova m7, [r5-32*4]
        r4 = at(r4, 32 * 8); // add r4, 32*8 ; bottomleft 16x8
        inv_txfm_add_dct_dct_32x16_10bpc_transpose_16x16(m, r4, s); // call .transpose_16x16
    }
    // .end:
    dst = at(r7, 32); // lea dstq, [r7+32]
    idct_16x16_internal_8bpc_main(m, s); // call m(idct_16x16_internal_8bpc).main
    inv_txfm_add_dct_dct_32x16_10bpc_write_16x16(m, s, &mut dst, stride); // call .write_16x16
}

/// `.transpose_16x16` of `inv_txfm_add_dct_dct_32x16_10bpc` (ends in `m(inv_txfm_add_dct_dct_8x32_10bpc)
/// .transpose`): the 16-bit rows in `m0`-`m7` and at `[r4-32*4..+3]`; `s[0]` (`[rsp+gprsize]`) receives `m15`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16_10bpc_transpose_16x16(m: &mut M, r4: *mut __m256i, s: &mut [__m256i; 3]) {
    m[8] = _mm256_unpackhi_epi32(m[3], m[1]); // punpckhdq m8, m3, m1
    m[3] = _mm256_unpacklo_epi32(m[3], m[1]); // punpckldq m3, m1
    m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2
    m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
    m[2] = _mm256_unpackhi_epi32(m[7], m[5]); // punpckhdq m2, m7, m5
    m[7] = _mm256_unpacklo_epi32(m[7], m[5]); // punpckldq m7, m5
    m[5] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m5, m4, m6
    m[4] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m4, m6
    m[6] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m6, m0, m4
    m[0] = _mm256_unpacklo_epi64(m[0], m[4]); // punpcklqdq m0, m4
    m[4] = _mm256_unpackhi_epi64(m[1], m[5]); // punpckhqdq m4, m1, m5
    m[1] = _mm256_unpacklo_epi64(m[1], m[5]); // punpcklqdq m1, m5
    m[5] = _mm256_unpackhi_epi64(m[7], m[3]); // punpckhqdq m5, m7, m3
    m[7] = _mm256_unpacklo_epi64(m[7], m[3]); // punpcklqdq m7, m3
    m[3] = _mm256_unpackhi_epi64(m[2], m[8]); // punpckhqdq m3, m2, m8
    m[2] = _mm256_unpacklo_epi64(m[2], m[8]); // punpcklqdq m2, m8
    m[8] = _mm256_inserti128_si256::<1>(m[0], lo(m[7])); // vinserti128 m8, m0, xm7, 1
    m[12] = _mm256_permute2x128_si256::<0x31>(m[0], m[7]); // vperm2i128 m12, m0, m7, 0x31
    m[9] = _mm256_inserti128_si256::<1>(m[6], lo(m[5])); // vinserti128 m9, m6, xm5, 1
    m[13] = _mm256_permute2x128_si256::<0x31>(m[6], m[5]); // vperm2i128 m13, m6, m5, 0x31
    m[10] = _mm256_inserti128_si256::<1>(m[1], lo(m[2])); // vinserti128 m10, m1, xm2, 1
    m[14] = _mm256_permute2x128_si256::<0x31>(m[1], m[2]); // vperm2i128 m14, m1, m2, 0x31
    m[11] = _mm256_inserti128_si256::<1>(m[4], lo(m[3])); // vinserti128 m11, m4, xm3, 1
    m[15] = _mm256_permute2x128_si256::<0x31>(m[4], m[3]); // vperm2i128 m15, m4, m3, 0x31
    m[0] = at(r4, 32 * 3).read(); // mova m0, [r4+32*3]
    m[1] = at(r4, 32 * 2).read(); // mova m1, [r4+32*2]
    m[2] = at(r4, 32 * 1).read(); // mova m2, [r4+32*1]
    m[3] = at(r4, 32 * 0).read(); // mova m3, [r4+32*0]
    m[4] = at(r4, -32 * 1).read(); // mova m4, [r4-32*1]
    m[5] = at(r4, -32 * 2).read(); // mova m5, [r4-32*2]
    m[6] = at(r4, -32 * 3).read(); // mova m6, [r4-32*3]
    m[7] = at(r4, -32 * 4).read(); // mova m7, [r4-32*4]
    s[0] = m[15]; // mova [rsp+gprsize], m15
    inv_txfm_add_dct_dct_8x32_10bpc_transpose(m) // jmp m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
}

/// `.main` of `inv_txfm_add_dct_dct_32x16_10bpc` (falls into `.main_end`): the first pass of eight rows
/// (`[cq+64*0..31]`, rect2) with `m12`/`m13` the clip bounds; clears the coefficients it read. `r6` is the frame
/// pointer the odd half works below (advanced by `32*16`); `r4`, `r5` are set as `.main_oddhalf_part2` sets them.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16_10bpc_main(
    m: &mut M,
    c: Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 1).cast())); // pmulld m0, m14, [cq+64* 1]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 7).cast())); // pmulld m1, m14, [cq+64* 7]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 9).cast())); // pmulld m2, m14, [cq+64* 9]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 15).cast())); // pmulld m3, m14, [cq+64*15]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 17).cast())); // pmulld m4, m14, [cq+64*17]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 23).cast())); // pmulld m5, m14, [cq+64*23]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 25).cast())); // pmulld m6, m14, [cq+64*25]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 31).cast())); // pmulld m7, m14, [cq+64*31]
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_rect2(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_rect2
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 3).cast())); // pmulld m0, m14, [cq+64* 3]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 5).cast())); // pmulld m1, m14, [cq+64* 5]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 11).cast())); // pmulld m2, m14, [cq+64*11]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 13).cast())); // pmulld m3, m14, [cq+64*13]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 19).cast())); // pmulld m4, m14, [cq+64*19]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 21).cast())); // pmulld m5, m14, [cq+64*21]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 27).cast())); // pmulld m6, m14, [cq+64*27]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 29).cast())); // pmulld m7, m14, [cq+64*29]
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_rect2(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_rect2
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 2).cast())); // pmulld m0, m14, [cq+64* 2]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 6).cast())); // pmulld m1, m14, [cq+64* 6]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 10).cast())); // pmulld m2, m14, [cq+64*10]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 14).cast())); // pmulld m3, m14, [cq+64*14]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 18).cast())); // pmulld m4, m14, [cq+64*18]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 22).cast())); // pmulld m5, m14, [cq+64*22]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 26).cast())); // pmulld m6, m14, [cq+64*26]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 30).cast())); // pmulld m7, m14, [cq+64*30]
    idct_8x16_internal_10bpc_main_oddhalf_rect2(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    m[0] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 0).cast())); // pmulld m0, m14, [cq+64* 0]
    m[1] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 4).cast())); // pmulld m1, m14, [cq+64* 4]
    m[2] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 8).cast())); // pmulld m2, m14, [cq+64* 8]
    m[3] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 12).cast())); // pmulld m3, m14, [cq+64*12]
    m[4] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 16).cast())); // pmulld m4, m14, [cq+64*16]
    m[5] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 20).cast())); // pmulld m5, m14, [cq+64*20]
    m[6] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 24).cast())); // pmulld m6, m14, [cq+64*24]
    m[7] = _mm256_mullo_epi32(m[14], _mm256_loadu_si256(at(c, 64 * 28).cast())); // pmulld m7, m14, [cq+64*28]
    idct_8x8_internal_10bpc_main_rect2(m); // call m(idct_8x8_internal_10bpc).main_rect2
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    let mut r7d: i32 = 64 * 30; // mov r7d, 64*30
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        _mm256_storeu_si256(at(c, r7 - 64 * 2).cast(), m[8]); // mova [cq+r7-64*2], m8
        _mm256_storeu_si256(at(c, r7 - 64 * 1).cast(), m[8]); // mova [cq+r7-64*1], m8
        _mm256_storeu_si256(at(c, r7 + 64 * 0).cast(), m[8]); // mova [cq+r7+64*0], m8
        _mm256_storeu_si256(at(c, r7 + 64 * 1).cast(), m[8]); // mova [cq+r7+64*1], m8
        r7d -= 64 * 4; // sub r7d, 64*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    inv_txfm_add_dct_dct_32x16_10bpc_main_end(m, *r6, *r5, *r4)
}

/// `.main_end` of `inv_txfm_add_dct_dct_32x16_10bpc`: the 32-point first pass's end (`IDCT32_END` with shift 1,
/// `m11` = `pd_2048` on entry, `pd_1` after): the even half in `m0`-`m7`, the odd half at `[r6-32*4..+3]`,
/// `[r5-32*4..+3]`, `[r4-32*4..+3]`; leaves eight 16-bit rows in `m0`-`m7` and eight at `[r5-32*4..+3]`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16_10bpc_main_end(m: &mut M, r6: *mut __m256i, r5: *mut __m256i, r4: *mut __m256i) {
    m[11] = _mm256_srli_epi32::<11>(m[11]); // psrld m11, 11 ; pd_1
    idct32_end::<1>(m, r6, r5, r4, 0, 15, 8, 9, 10, true); // IDCT32_END 0, 15, 8, 9, 10, 1
    idct32_end::<1>(m, r6, r5, r4, 1, 14, 8, 9, 10, true); // IDCT32_END 1, 14, 8, 9, 10, 1
    m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1 ; 16 17
    m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1 ; 0 1
    m[1] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m1, m14, m15 ; 14 15
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 30 31
    at(r5, 32 * 3).write(m[8]); // mova [r5+32*3], m8
    at(r5, 32 * 2).write(m[14]); // mova [r5+32*2], m14
    idct32_end::<1>(m, r6, r5, r4, 2, 15, 8, 9, 10, true); // IDCT32_END 2, 15, 8, 9, 10, 1
    idct32_end::<1>(m, r6, r5, r4, 3, 14, 8, 9, 10, true); // IDCT32_END 3, 14, 8, 9, 10, 1
    m[8] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m8, m2, m3 ; 18 19
    m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3 ; 2 3
    m[3] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m3, m14, m15 ; 12 13
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 28 29
    at(r5, 32 * 1).write(m[8]); // mova [r5+32*1], m8
    at(r5, 32 * 0).write(m[14]); // mova [r5+32*0], m14
    idct32_end::<1>(m, r6, r5, r4, 4, 15, 8, 9, 10, true); // IDCT32_END 4, 15, 8, 9, 10, 1
    idct32_end::<1>(m, r6, r5, r4, 5, 14, 8, 9, 10, true); // IDCT32_END 5, 14, 8, 9, 10, 1
    m[8] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m8, m4, m5 ; 20 21
    m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5 ; 4 5
    m[5] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m5, m14, m15 ; 10 11
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 26 27
    at(r5, -32 * 1).write(m[8]); // mova [r5-32*1], m8
    at(r5, -32 * 2).write(m[14]); // mova [r5-32*2], m14
    idct32_end::<1>(m, r6, r5, r4, 6, 15, 8, 9, 10, true); // IDCT32_END 6, 15, 8, 9, 10, 1
    idct32_end::<1>(m, r6, r5, r4, 7, 14, 8, 9, 10, true); // IDCT32_END 7, 14, 8, 9, 10, 1
    m[8] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m8, m6, m7 ; 22 23
    m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7 ; 6 7
    m[7] = _mm256_unpacklo_epi16(m[14], m[15]); // punpcklwd m7, m14, m15 ; 8 9
    m[14] = _mm256_unpackhi_epi16(m[14], m[15]); // punpckhwd m14, m15 ; 24 25
    at(r5, -32 * 3).write(m[8]); // mova [r5-32*3], m8
    at(r5, -32 * 4).write(m[14]); // mova [r5-32*4], m14
}

/// `.write_16x16` of `inv_txfm_add_dct_dct_32x16_10bpc`: rounds (`pw_2048`) and adds the sixteen 16-bit rows that
/// `m(idct_16x16_internal_8bpc).main` leaves (`m0`-`m15`, out1 in `s[1]`) to sixteen rows of 16 pixels, clamped
/// to 0..`pixel_10bpc_max`; `s` are `[rsp+gprsize+32*0..2]`; `dst` ends sixteen rows further.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x16_10bpc_write_16x16(m: &mut M, s: &mut [__m256i; 3], dst: &mut Dst, stride: isize) {
    m[1] = s[1]; // mova m1, [rsp+gprsize+32*1]
    s[0] = m[8]; // mova [rsp+gprsize+32*0], m8
    s[1] = m[9]; // mova [rsp+gprsize+32*1], m9
    s[2] = m[12]; // mova [rsp+gprsize+32*2], m12
    m[12] = pw(2048); // vpbroadcastd m12, [pw_2048]
    m[9] = pw(0x3ff); // vpbroadcastd m9, [pixel_10bpc_max]
    let r3 = stride * 3; // lea r3, [strideq*3]
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    m[0] = _mm256_mulhrs_epi16(m[0], m[12]); // pmulhrsw m0, m12
    m[1] = _mm256_mulhrs_epi16(m[1], m[12]); // pmulhrsw m1, m12
    m[2] = _mm256_mulhrs_epi16(m[2], m[12]); // pmulhrsw m2, m12
    m[3] = _mm256_mulhrs_epi16(m[3], m[12]); // pmulhrsw m3, m12
    idct_16x8_internal_10bpc_write_16x4(m, dst, stride, r3); // call m(idct_16x8_internal_10bpc).write_16x4
    m[0] = _mm256_mulhrs_epi16(m[12], m[4]); // pmulhrsw m0, m12, m4
    m[1] = _mm256_mulhrs_epi16(m[12], m[5]); // pmulhrsw m1, m12, m5
    m[2] = _mm256_mulhrs_epi16(m[12], m[6]); // pmulhrsw m2, m12, m6
    m[3] = _mm256_mulhrs_epi16(m[12], m[7]); // pmulhrsw m3, m12, m7
    idct_16x8_internal_10bpc_write_16x4(m, dst, stride, r3); // call m(idct_16x8_internal_10bpc).write_16x4
    m[0] = _mm256_mulhrs_epi16(m[12], s[0]); // pmulhrsw m0, m12, [rsp+gprsize+32*0]
    m[1] = _mm256_mulhrs_epi16(m[12], s[1]); // pmulhrsw m1, m12, [rsp+gprsize+32*1]
    m[2] = _mm256_mulhrs_epi16(m[12], m[10]); // pmulhrsw m2, m12, m10
    m[3] = _mm256_mulhrs_epi16(m[12], m[11]); // pmulhrsw m3, m12, m11
    idct_16x8_internal_10bpc_write_16x4(m, dst, stride, r3); // call m(idct_16x8_internal_10bpc).write_16x4
    m[0] = _mm256_mulhrs_epi16(m[12], s[2]); // pmulhrsw m0, m12, [rsp+gprsize+32*2]
    m[1] = _mm256_mulhrs_epi16(m[12], m[13]); // pmulhrsw m1, m12, m13
    m[2] = _mm256_mulhrs_epi16(m[12], m[14]); // pmulhrsw m2, m12, m14
    m[3] = _mm256_mulhrs_epi16(m[12], m[15]); // pmulhrsw m3, m12, m15
    idct_16x8_internal_10bpc_write_16x4(m, dst, stride, r3) // jmp m(idct_16x8_internal_10bpc).write_16x4
}

/// `cglobal inv_txfm_add_identity_identity_32x16_10bpc, 4, 7, 11, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x16_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    inv_txfm_add_identity_identity_32x16_10bpc_pass1(m, dst, stride, c, eob)
}

/// `cglobal inv_txfm_add_identity_identity_32x16_12bpc, 4, 7, 11, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x16_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    inv_txfm_add_identity_identity_32x16_10bpc_pass1(m, dst, stride, c, eob) // jmp m(inv_txfm_add_identity_identity_32x16_10bpc).pass1
}

/// `.pass1` of `inv_txfm_add_identity_identity_32x16_10bpc` (`m7` = the pixel maximum)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x16_10bpc_pass1(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    mut eob: c_int,
) {
    m[8] = pw(2896 * 8); // vpbroadcastd m8, [pw_2896x8]
    m[9] = pw(1697 * 16); // vpbroadcastd m9, [pw_1697x16]
    m[10] = pw(4096); // vpbroadcastd m10, [pw_4096]
    let r6 = stride * 5; // lea r6, [strideq*5]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    let r5 = dst; // mov r5, dstq
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 36; // sub eobd, 36
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 32); // add cq, 32
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    c = at(c, 64 * 8 - 32); // add cq, 64*8-32
    dst = at(r5, 16 * 1); // lea dstq, [r5+16*1]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 107; // sub eobd, 107 ; eob < 143
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 32); // add cq, 32
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    c = at(c, 64 * 8 - 32); // add cq, 64*8-32
    dst = at(r5, 16 * 2); // lea dstq, [r5+16*2]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 128; // sub eobd, 128 ; eob < 271
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 32); // add cq, 32
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    c = at(c, 64 * 8 - 32); // add cq, 64*8-32
    dst = at(r5, 16 * 3); // lea dstq, [r5+16*3]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6); // call .main
    eob -= 128; // sub eobd, 128 ; eob < 399
    if eob < 0 {
        return; // jl .ret
    }
    c = at(c, 32); // add cq, 32
    dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
    inv_txfm_add_identity_identity_32x16_10bpc_main(m, c, &mut dst, stride, r6);
    // call .main
    // .ret:
}

/// `.main` of `inv_txfm_add_identity_identity_32x16_10bpc` (ends in `m(inv_txfm_add_identity_identity_16x32_10bpc)
/// .main2`)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x16_10bpc_main(m: &mut M, c: Coef, dst: &mut Dst, stride: isize, r6: isize) {
    m[0] = _mm256_loadu_si256(at(c, 64 * 0).cast()); // mova m0, [cq+64*0]
    m[0] = _mm256_packs_epi32(m[0], _mm256_loadu_si256(at(c, 64 * 1).cast())); // packssdw m0, [cq+64*1]
    m[1] = _mm256_loadu_si256(at(c, 64 * 2).cast()); // mova m1, [cq+64*2]
    m[1] = _mm256_packs_epi32(m[1], _mm256_loadu_si256(at(c, 64 * 3).cast())); // packssdw m1, [cq+64*3]
    m[2] = _mm256_loadu_si256(at(c, 64 * 4).cast()); // mova m2, [cq+64*4]
    m[2] = _mm256_packs_epi32(m[2], _mm256_loadu_si256(at(c, 64 * 5).cast())); // packssdw m2, [cq+64*5]
    m[3] = _mm256_loadu_si256(at(c, 64 * 6).cast()); // mova m3, [cq+64*6]
    m[3] = _mm256_packs_epi32(m[3], _mm256_loadu_si256(at(c, 64 * 7).cast())); // packssdw m3, [cq+64*7]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[8]); // REPX {pmulhrsw x, m8 }, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_adds_epi16(m[x], m[x]); // REPX {paddsw x, x }, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        idtx16(m, x, 4, 9, Idtx16Shift::Trunc); // REPX {IDTX16 x, 4, 9, _ }, 0, 1, 2, 3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[10]); // REPX {pmulhrsw x, m10}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        _mm256_storeu_si256(at(c, 64 * x).cast(), m[6]); // REPX {mova [cq+64*x], m6}, 0, 1, 2, 3, 4, 5, 6, 7
    }
    inv_txfm_add_identity_identity_16x32_10bpc_main2(m, dst, stride, r6) // jmp m(inv_txfm_add_identity_identity_16x32_10bpc).main2
}

// --- 32x32 ---

/// `cglobal inv_txfm_add_dct_dct_32x32_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x32_10bpc(mut dst: Dst, stride: isize, mut c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 32; // or r3d, 32
        return inv_txfm_add_dct_dct_32x8_10bpc_dconly(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly
    }
    // PROLOGUE 0, 8, 16, 32*83, dst, stride, c, eob
    // `[rsp+32*0..2]` as the spill slots of the subroutines are `s`, `[rsp+32*3]` as `.pass2_end`'s fourth slot
    // `s3`; the frame `[rsp+32*0..82]` holds the rows (slot 3 is also the first row `.pass2_evenhalf` reads; each
    // value is read back through the name it was stored with).
    let mut s = [_mm256_setzero_si256(); 3];
    let s = &mut s;
    let mut s3 = _mm256_setzero_si256();
    let mut frame = [_mm256_setzero_si256(); 83];
    let rsp: *mut __m256i = frame.as_mut_ptr(); // zeroed whole (module documentation)
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    let mut r6 = at(rsp, 32 * 7); // lea r6, [rsp+32*7]
    let mut r4 = rsp; // (set by .main)
    let mut r5 = rsp; // (set by .main)
    let fast = 'm: {
        inv_txfm_add_dct_dct_32x32_10bpc_main(m, &mut c, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 36 {
            break 'm true; // cmp eobd, 36 / jl .fast
        }
        inv_txfm_add_dct_dct_32x32_10bpc_main(m, &mut c, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 136 {
            break 'm true; // cmp eobd, 136 / jl .fast
        }
        inv_txfm_add_dct_dct_32x32_10bpc_main(m, &mut c, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 300 {
            break 'm true; // cmp eobd, 300 / jl .fast
        }
        inv_txfm_add_dct_dct_32x32_10bpc_main(m, &mut c, &mut r6, &mut r4, &mut r5); // call .main
        false // jmp .pass2
    };
    if fast {
        // .fast:
        r4 = at(rsp, 32 * 71); // lea r4, [rsp+32*71]
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        loop {
            // .fast_loop:
            for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
                at(r6, 32 * x).write(m[0]); // REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
            }
            r6 = at(r6, 32 * 8); // add r6, 32*8
            if r6 >= r4 {
                break; // cmp r6, r4 / jl .fast_loop
            }
        }
    }
    // .pass2:
    let r3 = at(rsp, 32 * 3); // lea r3, [rsp+32*3]
    r4 = r6; // mov r4, r6
    r5 = at(r6, 32 * 8); // lea r5, [r6+32*8]
                         // lea r6, [pw_5+128]
    inv_txfm_add_dct_dct_32x32_10bpc_pass2_oddhalf(m, r3, s, r4, r5); // call .pass2_oddhalf
    inv_txfm_add_dct_dct_32x32_10bpc_pass2_evenhalf(m, r3, s); // call .pass2_evenhalf
    let mut r2 = at(dst, stride * 19); // imul r2, strideq, 19 / add r2, dstq
    let r3s = stride * 3; // lea r3, [strideq*3]
    inv_txfm_add_dct_dct_16x32_10bpc_pass2_end(m, s, &mut s3, r5, &mut dst, stride, &mut r2, r3s, r4); // call m(inv_txfm_add_dct_dct_16x32_10bpc).pass2_end
    dst = at(dst, -r3s); // sub dstq, r3
    r2 = at(r2, r3s + 32); // lea r2, [r2+r3+32]
    dst = at(dst, 32); // add dstq, 32
    let r3 = at(rsp, 32 * 11); // lea r3, [rsp+32*11]
    inv_txfm_add_dct_dct_32x32_10bpc_pass2_oddhalf(m, r3, s, r4, r5); // call .pass2_oddhalf
    inv_txfm_add_dct_dct_32x32_10bpc_pass2_evenhalf(m, r3, s); // call .pass2_evenhalf
    let r3s = stride * 3; // lea r3, [strideq*3]
    inv_txfm_add_dct_dct_16x32_10bpc_pass2_end(m, s, &mut s3, r5, &mut dst, stride, &mut r2, r3s, r4);
    // call m(inv_txfm_add_dct_dct_16x32_10bpc).pass2_end
}

/// `.main` of `inv_txfm_add_dct_dct_32x32_10bpc`: the first pass of eight rows (`[cq+128*0..31]`) with `m12`/`m13`
/// the clip bounds; clears the coefficients it read and advances `cq` by 32. Leaves the 16-bit rows at
/// `[r4-32*4..+3]` and `[r5-32*4..+3]`; `r6`, `r4`, `r5` as `.main_oddhalf_part2` sets them.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x32_10bpc_main(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    let c = *cq;
    m[0] = _mm256_loadu_si256(at(c, 128 * 1).cast()); // mova m0, [cq+128* 1]
    m[1] = _mm256_loadu_si256(at(c, 128 * 7).cast()); // mova m1, [cq+128* 7]
    m[2] = _mm256_loadu_si256(at(c, 128 * 9).cast()); // mova m2, [cq+128* 9]
    m[3] = _mm256_loadu_si256(at(c, 128 * 15).cast()); // mova m3, [cq+128*15]
    m[4] = _mm256_loadu_si256(at(c, 128 * 17).cast()); // mova m4, [cq+128*17]
    m[5] = _mm256_loadu_si256(at(c, 128 * 23).cast()); // mova m5, [cq+128*23]
    m[6] = _mm256_loadu_si256(at(c, 128 * 25).cast()); // mova m6, [cq+128*25]
    m[7] = _mm256_loadu_si256(at(c, 128 * 31).cast()); // mova m7, [cq+128*31]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1
    m[0] = _mm256_loadu_si256(at(c, 128 * 3).cast()); // mova m0, [cq+128* 3]
    m[1] = _mm256_loadu_si256(at(c, 128 * 5).cast()); // mova m1, [cq+128* 5]
    m[2] = _mm256_loadu_si256(at(c, 128 * 11).cast()); // mova m2, [cq+128*11]
    m[3] = _mm256_loadu_si256(at(c, 128 * 13).cast()); // mova m3, [cq+128*13]
    m[4] = _mm256_loadu_si256(at(c, 128 * 19).cast()); // mova m4, [cq+128*19]
    m[5] = _mm256_loadu_si256(at(c, 128 * 21).cast()); // mova m5, [cq+128*21]
    m[6] = _mm256_loadu_si256(at(c, 128 * 27).cast()); // mova m6, [cq+128*27]
    m[7] = _mm256_loadu_si256(at(c, 128 * 29).cast()); // mova m7, [cq+128*29]
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2
    m[0] = _mm256_loadu_si256(at(c, 128 * 2).cast()); // mova m0, [cq+128* 2]
    m[1] = _mm256_loadu_si256(at(c, 128 * 6).cast()); // mova m1, [cq+128* 6]
    m[2] = _mm256_loadu_si256(at(c, 128 * 10).cast()); // mova m2, [cq+128*10]
    m[3] = _mm256_loadu_si256(at(c, 128 * 14).cast()); // mova m3, [cq+128*14]
    m[4] = _mm256_loadu_si256(at(c, 128 * 18).cast()); // mova m4, [cq+128*18]
    m[5] = _mm256_loadu_si256(at(c, 128 * 22).cast()); // mova m5, [cq+128*22]
    m[6] = _mm256_loadu_si256(at(c, 128 * 26).cast()); // mova m6, [cq+128*26]
    m[7] = _mm256_loadu_si256(at(c, 128 * 30).cast()); // mova m7, [cq+128*30]
    idct_8x16_internal_10bpc_main_oddhalf(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
    m[0] = _mm256_loadu_si256(at(c, 128 * 0).cast()); // mova m0, [cq+128* 0]
    m[1] = _mm256_loadu_si256(at(c, 128 * 4).cast()); // mova m1, [cq+128* 4]
    m[2] = _mm256_loadu_si256(at(c, 128 * 8).cast()); // mova m2, [cq+128* 8]
    m[3] = _mm256_loadu_si256(at(c, 128 * 12).cast()); // mova m3, [cq+128*12]
    m[4] = _mm256_loadu_si256(at(c, 128 * 16).cast()); // mova m4, [cq+128*16]
    m[5] = _mm256_loadu_si256(at(c, 128 * 20).cast()); // mova m5, [cq+128*20]
    m[6] = _mm256_loadu_si256(at(c, 128 * 24).cast()); // mova m6, [cq+128*24]
    m[7] = _mm256_loadu_si256(at(c, 128 * 28).cast()); // mova m7, [cq+128*28]
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    inv_txfm_add_dct_dct_8x32_10bpc_main_end(m, *r6, *r5, *r4); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_end
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r7d: i32 = 128 * 29; // mov r7d, 128*29
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        _mm256_storeu_si256(at(c, r7 - 128 * 1).cast(), m[15]); // mova [cq+r7-128*1], m15
        _mm256_storeu_si256(at(c, r7 + 128 * 0).cast(), m[15]); // mova [cq+r7+128*0], m15
        _mm256_storeu_si256(at(c, r7 + 128 * 1).cast(), m[15]); // mova [cq+r7+128*1], m15
        _mm256_storeu_si256(at(c, r7 + 128 * 2).cast(), m[15]); // mova [cq+r7+128*2], m15
        r7d -= 128 * 4; // sub r7d, 128*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    *cq = at(c, 32); // add cq, 32
    let (r4, r5) = (*r4, *r5);
    at(r4, -32 * 4).write(m[0]); // mova [r4-32*4], m0
    at(r4, -32 * 3).write(m[1]); // mova [r4-32*3], m1
    at(r4, -32 * 2).write(m[2]); // mova [r4-32*2], m2
    at(r4, -32 * 1).write(m[3]); // mova [r4-32*1], m3
    at(r4, 32 * 0).write(m[4]); // mova [r4+32*0], m4
    at(r4, 32 * 1).write(m[5]); // mova [r4+32*1], m5
    at(r4, 32 * 2).write(m[6]); // mova [r4+32*2], m6
    at(r4, 32 * 3).write(m[7]); // mova [r4+32*3], m7
    m[0] = at(r5, 32 * 3).read(); // mova m0, [r5+32*3]
    m[1] = at(r5, 32 * 2).read(); // mova m1, [r5+32*2]
    m[2] = at(r5, 32 * 1).read(); // mova m2, [r5+32*1]
    m[3] = at(r5, 32 * 0).read(); // mova m3, [r5+32*0]
    m[4] = at(r5, -32 * 1).read(); // mova m4, [r5-32*1]
    m[5] = at(r5, -32 * 2).read(); // mova m5, [r5-32*2]
    m[6] = at(r5, -32 * 3).read(); // mova m6, [r5-32*3]
    m[7] = at(r5, -32 * 4).read(); // mova m7, [r5-32*4]
    inv_txfm_add_dct_dct_8x32_10bpc_transpose(m); // call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    at(r5, -32 * 4).write(m[0]); // mova [r5-32*4], m0
    at(r5, -32 * 3).write(m[1]); // mova [r5-32*3], m1
    at(r5, -32 * 2).write(m[2]); // mova [r5-32*2], m2
    at(r5, -32 * 1).write(m[3]); // mova [r5-32*1], m3
    at(r5, 32 * 0).write(m[4]); // mova [r5+32*0], m4
    at(r5, 32 * 1).write(m[5]); // mova [r5+32*1], m5
    at(r5, 32 * 2).write(m[6]); // mova [r5+32*2], m6
    at(r5, 32 * 3).write(m[7]); // mova [r5+32*3], m7
}

/// `.pass2_oddhalf` of `inv_txfm_add_dct_dct_32x32_10bpc` (ends in `m(inv_txfm_add_dct_dct_16x32_8bpc)
/// .main_oddhalf`, with `tmp1q` = `r4`, `tmp2q` = `r5`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x32_10bpc_pass2_oddhalf(
    m: &mut M,
    r3: *mut __m256i,
    s: &mut [__m256i; 3],
    r4: *mut __m256i,
    r5: *mut __m256i,
) {
    m[0] = at(r3, 32 * 1).read(); // mova m0, [r3+32* 1] ; 1
    m[1] = at(r3, 32 * 3).read(); // mova m1, [r3+32* 3] ; 3
    m[2] = at(r3, 32 * 5).read(); // mova m2, [r3+32* 5] ; 5
    m[3] = at(r3, 32 * 7).read(); // mova m3, [r3+32* 7] ; 7
    m[4] = at(r3, 32 * 17).read(); // mova m4, [r3+32*17] ; 9
    m[5] = at(r3, 32 * 19).read(); // mova m5, [r3+32*19] ; 11
    m[6] = at(r3, 32 * 21).read(); // mova m6, [r3+32*21] ; 13
    m[7] = at(r3, 32 * 23).read(); // mova m7, [r3+32*23] ; 15
    m[8] = at(r3, 32 * 33).read(); // mova m8, [r3+32*33] ; 17
    m[9] = at(r3, 32 * 35).read(); // mova m9, [r3+32*35] ; 19
    m[10] = at(r3, 32 * 37).read(); // mova m10, [r3+32*37] ; 21
    m[11] = at(r3, 32 * 39).read(); // mova m11, [r3+32*39] ; 23
    m[12] = at(r3, 32 * 49).read(); // mova m12, [r3+32*49] ; 25
    m[13] = at(r3, 32 * 51).read(); // mova m13, [r3+32*51] ; 27
    m[14] = at(r3, 32 * 53).read(); // mova m14, [r3+32*53] ; 29
    m[15] = at(r3, 32 * 55).read(); // mova m15, [r3+32*55] ; 31
    inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf(m, s, r4, r5) // jmp m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
}

/// `.pass2_evenhalf` of `inv_txfm_add_dct_dct_32x32_10bpc` (ends in `m(idct_16x16_internal_8bpc).main`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x32_10bpc_pass2_evenhalf(m: &mut M, r3: *mut __m256i, s: &mut [__m256i; 3]) {
    m[0] = at(r3, 32 * 0).read(); // mova m0, [r3+32* 0] ; 0
    m[1] = at(r3, 32 * 2).read(); // mova m1, [r3+32* 2] ; 2
    m[2] = at(r3, 32 * 4).read(); // mova m2, [r3+32* 4] ; 4
    m[3] = at(r3, 32 * 6).read(); // mova m3, [r3+32* 6] ; 6
    m[4] = at(r3, 32 * 16).read(); // mova m4, [r3+32*16] ; 8
    m[5] = at(r3, 32 * 18).read(); // mova m5, [r3+32*18] ; 10
    m[6] = at(r3, 32 * 20).read(); // mova m6, [r3+32*20] ; 12
    m[7] = at(r3, 32 * 22).read(); // mova m7, [r3+32*22] ; 14
    m[8] = at(r3, 32 * 32).read(); // mova m8, [r3+32*32] ; 16
    m[9] = at(r3, 32 * 34).read(); // mova m9, [r3+32*34] ; 18
    m[10] = at(r3, 32 * 36).read(); // mova m10, [r3+32*36] ; 20
    m[11] = at(r3, 32 * 38).read(); // mova m11, [r3+32*38] ; 22
    m[12] = at(r3, 32 * 48).read(); // mova m12, [r3+32*48] ; 24
    m[13] = at(r3, 32 * 50).read(); // mova m13, [r3+32*50] ; 26
    m[14] = at(r3, 32 * 52).read(); // mova m14, [r3+32*52] ; 28
    m[15] = at(r3, 32 * 54).read(); // mova m15, [r3+32*54] ; 30
    s[0] = m[15]; // mova [rsp+gprsize], m15
    idct_16x16_internal_8bpc_main(m, s) // jmp m(idct_16x16_internal_8bpc).main
}

/// `cglobal inv_txfm_add_identity_identity_32x32_10bpc, 4, 8, 8, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x32_10bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0x3ff); // vpbroadcastd m7, [pixel_10bpc_max]
    inv_txfm_add_identity_identity_32x32_10bpc_pass1(m, dst, stride, c, eob)
}

/// `cglobal inv_txfm_add_identity_identity_32x32_12bpc, 4, 8, 8, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x32_12bpc(dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    m[7] = pw(0xfff); // vpbroadcastd m7, [pixel_12bpc_max]
    inv_txfm_add_identity_identity_32x32_10bpc_pass1(m, dst, stride, c, eob) // jmp m(inv_txfm_add_identity_identity_32x32_10bpc).pass1
}

/// `.pass1` of `inv_txfm_add_identity_identity_32x32_10bpc` (`m7` = the pixel maximum)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x32_10bpc_pass1(
    m: &mut M,
    mut dst: Dst,
    stride: isize,
    mut c: Coef,
    eob: c_int,
) {
    m[5] = pw(8192); // vpbroadcastd m5, [pw_8192]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    let r6 = stride * 3; // lea r6, [strideq*3]
    let r5 = stride * 5; // lea r5, [strideq*5]
    let r4 = stride + r6 * 2; // lea r4, [strideq+r6*2] ; strideq*7
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4); // call .main ; 0
    if eob < 36 {
        return; // cmp eobd, 36 / jl .ret
    }
    c = at(c, 128 * 8); // add cq, 128*8 ; 0 1
    let mut r7 = dst; // mov r7, dstq ; 1
    dst = at(dst, 16); // add dstq, 16
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4); // call .main
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    if eob < 136 {
        return; // cmp eobd, 136 / jl .ret
    }
    c = at(c, 128 * 16 - 32); // add cq, 128*16-32 ; 0 1 2
    dst = at(r7, 16 * 2); // lea dstq, [r7+16*2] ; 1 2
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4); // call .main ; 2
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    if eob < 300 {
        return; // cmp eobd, 300 / jl .ret
    }
    c = at(c, 128 * 24 - 64); // add cq, 128*24-64 ; 0 1 2 3
    r7 = at(r7, 16 * 3); // add r7, 16*3 ; 1 2 3
    dst = r7; // mov dstq, r7 ; 2 3
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4); // call .main ; 3
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    if eob < 535 {
        return; // cmp eobd, 535 / jl .ret
    }
    c = at(c, 128 * 24 - 64); // add cq, 128*24-64 ; 0 1 2 3
    dst = at(r7, stride * 8); // lea dstq, [r7+strideq*8] ; 1 2 3 4
    r7 = dst; // mov r7, dstq ; 2 3 4
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4); // call .main ; 3 4
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2
    if eob < 755 {
        return; // cmp eobd, 755 / jl .ret
    }
    c = at(c, 128 * 16 - 32); // add cq, 128*16-32 ; 0 1 2 3
    dst = at(r7, stride * 8); // lea dstq, [r7+strideq*8] ; 1 2 3 4
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4); // call .main ; 2 3 4 5
    inv_txfm_add_identity_identity_32x32_10bpc_main2(m, &mut c, &mut dst, stride, r5, r6, r4); // call .main2 ; 3 4 5
    if eob < 911 {
        return; // cmp eobd, 911 / jl .ret
    }
    c = at(c, 128 * 8); // add cq, 128*8 ; 0 1 2 3
    dst = at(dst, 16); // add dstq, 16 ; 1 2 3 4
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, c, dst, stride, r5, r6, r4);
    // call .main ; 2 3 4 5
    // .ret:
}

/// `.main2` of `inv_txfm_add_identity_identity_32x32_10bpc` (falls into `.main`)
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x32_10bpc_main2(
    m: &mut M,
    c: &mut Coef,
    dst: &mut Dst,
    stride: isize,
    r5: isize,
    r6: isize,
    r4: isize,
) {
    *c = at(*c, -(128 * 8 - 32)); // sub cq, 128*8-32
    *dst = at(*dst, stride * 8 - 16); // lea dstq, [dstq+strideq*8-16]
    inv_txfm_add_identity_identity_32x32_10bpc_main(m, *c, *dst, stride, r5, r6, r4)
}

/// `.main` of `inv_txfm_add_identity_identity_32x32_10bpc` (ends in `m(inv_txfm_add_identity_identity_8x32_10bpc)
/// .main_zero`): eight rows of eight coefficients at `[cq+128*0..7]` to eight rows of eight pixels.
#[inline(always)]
unsafe fn inv_txfm_add_identity_identity_32x32_10bpc_main(
    m: &mut M,
    c: Coef,
    dst: Dst,
    stride: isize,
    r5: isize,
    r6: isize,
    r4: isize,
) {
    m[0] = _mm256_loadu_si256(at(c, 128 * 0).cast()); // mova m0, [cq+128*0]
    m[0] = _mm256_packs_epi32(m[0], _mm256_loadu_si256(at(c, 128 * 1).cast())); // packssdw m0, [cq+128*1]
    m[1] = _mm256_loadu_si256(at(c, 128 * 2).cast()); // mova m1, [cq+128*2]
    m[1] = _mm256_packs_epi32(m[1], _mm256_loadu_si256(at(c, 128 * 3).cast())); // packssdw m1, [cq+128*3]
    m[2] = _mm256_loadu_si256(at(c, 128 * 4).cast()); // mova m2, [cq+128*4]
    m[2] = _mm256_packs_epi32(m[2], _mm256_loadu_si256(at(c, 128 * 5).cast())); // packssdw m2, [cq+128*5]
    m[3] = _mm256_loadu_si256(at(c, 128 * 6).cast()); // mova m3, [cq+128*6]
    m[3] = _mm256_packs_epi32(m[3], _mm256_loadu_si256(at(c, 128 * 7).cast())); // packssdw m3, [cq+128*7]
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[5]); // REPX {pmulhrsw x, m5}, m0, m1, m2, m3
    }
    inv_txfm_add_identity_identity_8x32_10bpc_main_zero(m, c, dst, stride, r5, r6, r4)
    // jmp m(inv_txfm_add_identity_identity_8x32_10bpc).main_zero
}

// --- entry points ---

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x32_10bpc_avx2,
    inv_txfm_add_dct_dct_16x32_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x32_10bpc_avx2,
    inv_txfm_add_identity_identity_16x32_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_16x32_12bpc_avx2,
    inv_txfm_add_identity_identity_16x32_12bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_32x16_10bpc_avx2,
    inv_txfm_add_dct_dct_32x16_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x16_10bpc_avx2,
    inv_txfm_add_identity_identity_32x16_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x16_12bpc_avx2,
    inv_txfm_add_identity_identity_32x16_12bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_32x32_10bpc_avx2,
    inv_txfm_add_dct_dct_32x32_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x32_10bpc_avx2,
    inv_txfm_add_identity_identity_32x32_10bpc
);
inv_txfm_fn!(
    dav1d_inv_txfm_add_identity_identity_32x32_12bpc_avx2,
    inv_txfm_add_identity_identity_32x32_12bpc
);

// --- part 8: itx16_avx2.asm lines 7454-8599 (16x64, 32x64, 64x16, 64x32, 64x64) ---------------------------------

// The 64-point sizes have dct_dct only, and only at 10 bpc. Their frames (`PROLOGUE …, 32*N`) are arrays of `N`
// 32-byte slots, reached through the raw pointer `rsp` to their first slot and the pointers the assembly derives
// from it (`r4`, `r5`, `r6`, `r7`, `r10`). Each frame is zeroed whole before its first use (a zero-initialized array), so no
// load from it can find a slot without a value, whichever eob path wrote it. The subroutines that the assembly
// `call`s address the caller's first slots as `[rsp+gprsize+32*n]`; they receive them as `&mut [__m256i; 3]`
// (`s3_p8(rsp)`; 16x32's `.pass2_end` also takes the fourth slot, `[rsp+gprsize+32*3]`), which no other pointer
// touches while the subroutine runs. (The first row slots of 16x64, 32x64, 64x32 and 64x64 lie on slot 2 or 3, as
// in the assembly: those rows are loaded before the second pass's subroutines overwrite them.)

/// `mova m, [p+off]` in a frame (32-byte aligned; `off` in bytes)
#[inline(always)]
unsafe fn ld_p8(p: *mut __m256i, off: isize) -> __m256i {
    _mm256_load_si256(at(p, off))
}

/// `mova [p+off], m` in a frame
#[inline(always)]
unsafe fn st_p8(p: *mut __m256i, off: isize, v: __m256i) {
    _mm256_store_si256(at(p, off), v)
}

/// The frame's first three slots, `[rsp+gprsize+32*0..2]` of a called subroutine.
#[inline(always)]
unsafe fn s3_p8<'a>(rsp: *mut __m256i) -> &'a mut [__m256i; 3] {
    &mut *rsp.cast::<[__m256i; 3]>()
}

/// `mova m, [dstq+off]` / `[cq+off]`: 32 bytes of caller memory
#[inline(always)]
unsafe fn ldu_p8<T>(p: *mut T, off: isize) -> __m256i {
    _mm256_loadu_si256(at(p, off).cast())
}

/// `mova [dstq+off], m` / `[cq+off], m`: 32 bytes of caller memory
#[inline(always)]
unsafe fn stu_p8<T>(p: *mut T, off: isize, v: __m256i) {
    _mm256_storeu_si256(at(p, off).cast(), v)
}

/// `pmaxsd x, m12` / `pminsd x, m13` over the registers `r`, in that order (the two `REPX` lines that clip to
/// `clip_18b_*`, where both name the same registers in the same order)
#[inline(always)]
unsafe fn clip_p8<const N: usize>(m: &mut M, r: [usize; N]) {
    for x in r {
        m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, …
    }
    for x in r {
        m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, …
    }
}

// --- 16x64 and the idct64 subroutines that the other sizes call -------------------------------------------------

/// `%macro IDCT64_PART2_END out, src[1-2], tmp[1-3], offset[1-4]` (the 16 bpc form, pass 2 only): the last sums
/// and differences of the 64-point DCT for the outputs `out`, `31-out`, `32+out` and `63-out`, rounded by `m14`
/// and added to four rows of 16 pixels at the offsets `o` from `dstq` and `r2`.
#[inline(always)]
unsafe fn idct64_part2_end(
    m: &mut M,
    n: isize,
    r: [usize; 5],
    r4: *mut __m256i,
    r5: *mut __m256i,
    dst: Dst,
    r2: Dst,
    o: [isize; 4],
) {
    let [s1, s2, t1, t2, t3] = r;
    if n & 1 != 0 {
        m[t2] = ld_p8(r5, -32 * (51 - n)); // mova m%5, [r5-32*(51-%1)] ; idct16 out 0+n
        m[t1] = ld_p8(r4, -32 * (14 + n)); // mova m%4, [r4-32*(14+%1)] ; idct32 out31-n
    } else {
        m[t2] = ld_p8(r4, -32 * (45 - n)); // mova m%5, [r4-32*(45-%1)]
        m[t1] = ld_p8(r5, -32 * (20 + n)); // mova m%4, [r5-32*(20+%1)]
    }
    m[t3] = _mm256_adds_epi16(m[t2], m[t1]); // paddsw m%6, m%5, m%4 ; idct32 out 0+n
    m[t2] = _mm256_subs_epi16(m[t2], m[t1]); // psubsw m%5, m%4 ; idct32 out31-n
    m[t1] = _mm256_adds_epi16(m[t2], m[s2]); // paddsw m%4, m%5, m%3 ; out31-n
    m[t2] = _mm256_subs_epi16(m[t2], m[s2]); // psubsw m%5, m%3 ; out32+n
    m[s2] = _mm256_adds_epi16(m[t3], m[s1]); // paddsw m%3, m%6, m%2 ; out 0+n
    m[t3] = _mm256_subs_epi16(m[t3], m[s1]); // psubsw m%6, m%2 ; out63-n
    for x in [t2, t3, t1, s2] {
        m[x] = _mm256_mulhrs_epi16(m[x], m[14]); // REPX {pmulhrsw x, m14}, m%5, m%6, m%4, m%3
    }
    let (d0, d1) = if n & 1 != 0 { (r2, dst) } else { (dst, r2) };
    m[s2] = _mm256_add_epi16(m[s2], ldu_p8(d0, o[0])); // paddw m%3, [%%d0+%7 ]
    m[t1] = _mm256_add_epi16(m[t1], ldu_p8(d1, o[1])); // paddw m%4, [%%d1+%8 ]
    m[t2] = _mm256_add_epi16(m[t2], ldu_p8(d0, o[2])); // paddw m%5, [%%d0+%9 ]
    m[t3] = _mm256_add_epi16(m[t3], ldu_p8(d1, o[3])); // paddw m%6, [%%d1+%10]
    m[s1] = _mm256_setzero_si256(); // pxor m%2, m%2
    for x in [s2, t1, t2, t3] {
        m[x] = _mm256_max_epi16(m[x], m[s1]); // REPX {pmaxsw x, m%2}, m%3, m%4, m%5, m%6
    }
    m[s1] = pw(0x3ff); // vpbroadcastd m%2, [pixel_10bpc_max]
    for x in [s2, t1, t2, t3] {
        m[x] = _mm256_min_epi16(m[x], m[s1]); // REPX {pminsw x, m%2}, m%3, m%4, m%5, m%6
    }
    stu_p8(d0, o[0], m[s2]); // mova [%%d0+%7 ], m%3
    stu_p8(d1, o[1], m[t1]); // mova [%%d1+%8 ], m%4
    stu_p8(d0, o[2], m[t2]); // mova [%%d0+%9 ], m%5
    stu_p8(d1, o[3], m[t3]); // mova [%%d1+%10], m%6
}

/// `.main` of `inv_txfm_add_dct_dct_16x64_10bpc`: the 16-point DCT of eight columns of 16 coefficients
/// (`[cq+128*0..15]`, which it clears), rounded, packed to words and transposed into `[r6-32*4]` … `[r6+32*3]`;
/// moves `cq` 32 bytes and `r6` eight slots on.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_10bpc_main(m: &mut M, cq: &mut Coef, r6: &mut *mut __m256i) {
    let c = *cq;
    m[0] = ldu_p8(c, 128 * 1); // mova m0, [cq+128* 1]
    m[1] = ldu_p8(c, 128 * 3); // mova m1, [cq+128* 3]
    m[2] = ldu_p8(c, 128 * 5); // mova m2, [cq+128* 5]
    m[3] = ldu_p8(c, 128 * 7); // mova m3, [cq+128* 7]
    m[4] = ldu_p8(c, 128 * 9); // mova m4, [cq+128* 9]
    m[5] = ldu_p8(c, 128 * 11); // mova m5, [cq+128*11]
    m[6] = ldu_p8(c, 128 * 13); // mova m6, [cq+128*13]
    m[7] = ldu_p8(c, 128 * 15); // mova m7, [cq+128*15]
    idct_8x16_internal_10bpc_main_oddhalf(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf
    m[0] = ldu_p8(c, 128 * 0); // mova m0, [cq+128* 0]
    m[1] = ldu_p8(c, 128 * 2); // mova m1, [cq+128* 2]
    m[2] = ldu_p8(c, 128 * 4); // mova m2, [cq+128* 4]
    m[3] = ldu_p8(c, 128 * 6); // mova m3, [cq+128* 6]
    m[4] = ldu_p8(c, 128 * 8); // mova m4, [cq+128* 8]
    m[5] = ldu_p8(c, 128 * 10); // mova m5, [cq+128*10]
    m[6] = ldu_p8(c, 128 * 12); // mova m6, [cq+128*12]
    m[7] = ldu_p8(c, 128 * 14); // mova m7, [cq+128*14]
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r7d: i32 = 128 * 13; // mov r7d, 128*13
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        stu_p8(c, r7 - 128 * 1, m[15]); // mova [cq+r7-128*1], m15
        stu_p8(c, r7 + 128 * 0, m[15]); // mova [cq+r7+128*0], m15
        stu_p8(c, r7 + 128 * 1, m[15]); // mova [cq+r7+128*1], m15
        stu_p8(c, r7 + 128 * 2, m[15]); // mova [cq+r7+128*2], m15
        r7d -= 128 * 4; // sub r7d, 128*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    *cq = at(c, 32); // add cq, 32
    let p = *r6;
    m[15] = _mm256_srli_epi32::<10>(m[11]); // psrld m15, m11, 10 ; pd_2
    m[8] = ld_p8(p, -32 * 4); // mova m8, [r6-32*4]
    m[9] = ld_p8(p, 32 * 3); // mova m9, [r6+32*3]
    for x in [0, 1, 2, 3, 4, 5, 6, 7] {
        m[x] = _mm256_add_epi32(m[x], m[15]); // REPX {paddd x, m15}, m0, m1, m2, m3, m4, m5, m6, m7
    }
    m[10] = _mm256_sub_epi32(m[0], m[8]); // psubd m10, m0, m8 ; out15
    m[0] = _mm256_add_epi32(m[0], m[8]); // paddd m0, m8 ; out0
    m[8] = ld_p8(p, -32 * 3); // mova m8, [r6-32*3]
    m[15] = _mm256_sub_epi32(m[7], m[9]); // psubd m15, m7, m9 ; out8
    m[7] = _mm256_add_epi32(m[7], m[9]); // paddd m7, m9 ; out7
    m[9] = ld_p8(p, 32 * 2); // mova m9, [r6+32*2]
    for x in [0, 15, 10, 7] {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m0, m15, m10, m7
    }
    m[0] = _mm256_packs_epi32(m[0], m[15]); // packssdw m0, m15
    m[7] = _mm256_packs_epi32(m[7], m[10]); // packssdw m7, m10
    m[10] = _mm256_sub_epi32(m[1], m[8]); // psubd m10, m1, m8 ; out14
    m[1] = _mm256_add_epi32(m[1], m[8]); // paddd m1, m8 ; out1
    m[8] = ld_p8(p, -32 * 2); // mova m8, [r6-32*2]
    m[15] = _mm256_sub_epi32(m[6], m[9]); // psubd m15, m6, m9 ; out9
    m[6] = _mm256_add_epi32(m[6], m[9]); // paddd m6, m9 ; out6
    m[9] = ld_p8(p, 32 * 1); // mova m9, [r6+32*1]
    for x in [1, 15, 10, 6] {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m1, m15, m10, m6
    }
    m[1] = _mm256_packs_epi32(m[1], m[15]); // packssdw m1, m15
    m[6] = _mm256_packs_epi32(m[6], m[10]); // packssdw m6, m10
    m[10] = _mm256_sub_epi32(m[2], m[8]); // psubd m10, m2, m8 ; out13
    m[2] = _mm256_add_epi32(m[2], m[8]); // paddd m2, m8 ; out2
    m[8] = ld_p8(p, -32 * 1); // mova m8, [r6-32*1]
    m[15] = _mm256_sub_epi32(m[5], m[9]); // psubd m15, m5, m9 ; out10
    m[5] = _mm256_add_epi32(m[5], m[9]); // paddd m5, m9 ; out5
    m[9] = ld_p8(p, 32 * 0); // mova m9, [r6+32*0]
    for x in [2, 15, 10, 5] {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m2, m15, m10, m5
    }
    m[2] = _mm256_packs_epi32(m[2], m[15]); // packssdw m2, m15
    m[5] = _mm256_packs_epi32(m[5], m[10]); // packssdw m5, m10
    m[10] = _mm256_sub_epi32(m[3], m[8]); // psubd m10, m3, m8 ; out12
    m[3] = _mm256_add_epi32(m[3], m[8]); // paddd m3, m8 ; out3
    m[15] = _mm256_sub_epi32(m[4], m[9]); // psubd m15, m4, m9 ; out11
    m[4] = _mm256_add_epi32(m[4], m[9]); // paddd m4, m9 ; out4
    for x in [3, 15, 10, 4] {
        m[x] = _mm256_srai_epi32::<2>(m[x]); // REPX {psrad x, 2}, m3, m15, m10, m4
    }
    m[3] = _mm256_packs_epi32(m[3], m[15]); // packssdw m3, m15
    m[4] = _mm256_packs_epi32(m[4], m[10]); // packssdw m4, m10
    idct_16x8_internal_10bpc_transpose3(m); // call m(idct_16x8_internal_10bpc).transpose3
    st_p8(p, -32 * 4, m[0]); // mova [r6-32*4], m0
    st_p8(p, -32 * 3, m[1]); // mova [r6-32*3], m1
    st_p8(p, -32 * 2, m[2]); // mova [r6-32*2], m2
    st_p8(p, -32 * 1, m[3]); // mova [r6-32*1], m3
    st_p8(p, 32 * 0, m[4]); // mova [r6+32*0], m4
    st_p8(p, 32 * 1, m[5]); // mova [r6+32*1], m5
    st_p8(p, 32 * 2, m[6]); // mova [r6+32*2], m6
    st_p8(p, 32 * 3, m[7]); // mova [r6+32*3], m7
    *r6 = at(p, 32 * 8); // add r6, 32*8
}

/// `.main_part2_pass2` of `inv_txfm_add_dct_dct_16x64_10bpc`: idct64 steps 6-9 (`main_part2_internal` of 8 bpc)
/// and the sums of `IDCT64_PART2_END`, added to the 64 rows of 16 pixels at `dstq` (`r3`, `r7`, `r8`, `r9` =
/// `strideq` times 6, 7, 4, 5); `r4` and `r5` move towards each other until they meet, `dstq` four rows on.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_10bpc_main_part2_pass2(
    m: &mut M,
    dst: &mut Dst,
    r7: isize,
    stride: isize,
    r3: isize,
    r8: isize,
    r9: isize,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    m[11] = pw2(1567, 3784); // vpbroadcastd m11, [pw_1567_3784]
    m[12] = pw2(-3784, 1567); // vpbroadcastd m12, [pw_m3784_1567]
    m[13] = pw2(2896, 2896); // vpbroadcastd m13, [pw_2896_2896]
                             // lea r6, [pw_5+128] (the 8 bpc o_base)
    let mut r2 = at(*dst, r7); // lea r2, [dstq+r7]
    let o1 = [0, r3 * 4, r8 * 8, r7 * 8];
    let o2 = [stride * 8, r8 * 4, r9 * 8, r3 * 8];
    loop {
        // .main_part2_pass2_loop:
        m[14] = pw2(-2896, 2896); // vpbroadcastd m14, [pw_m2896_2896]
        inv_txfm_add_dct_dct_16x64_8bpc_main_part2_internal(m, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part2_internal
        m[14] = pw(2048); // vpbroadcastd m14, [pw_2048]
        idct64_part2_end(m, 0, [7, 0, 6, 9, 10], *r4, *r5, *dst, r2, o1); // IDCT64_PART2_END 0, 7, 0, 6, 9, 10, strideq*0, r3*4, r8*8, r7*8
        idct64_part2_end(m, 7, [8, 5, 0, 6, 7], *r4, *r5, *dst, r2, o1); // IDCT64_PART2_END 7, 8, 5, 0, 6, 7, strideq*0, r3*4, r8*8, r7*8
        idct64_part2_end(m, 8, [2, 1, 0, 6, 7], *r4, *r5, *dst, r2, o2); // IDCT64_PART2_END 8, 2, 1, 0, 6, 7, strideq*8, r8*4, r9*8, r3*8
        idct64_part2_end(m, 15, [3, 4, 0, 6, 7], *r4, *r5, *dst, r2, o2); // IDCT64_PART2_END 15, 3, 4, 0, 6, 7, strideq*8, r8*4, r9*8, r3*8
        *dst = at(*dst, stride); // add dstq, strideq
        r2 = at(r2, -stride); // sub r2, strideq
        if *r4 == *r5 {
            break; // cmp r4, r5 / jne .main_part2_pass2_loop
        }
    }
}

/// `[r5+4*k]` of `idct64_mul_16bpc`, with `r5` = `idct64_mul_16bpc + 4*r5k` (`vpbroadcastd m, [r5+4*k]`)
#[inline(always)]
unsafe fn idct64_mul_p8(r5k: usize, k: usize) -> __m256i {
    pd(idct64_mul_16bpc.0[r5k + k])
}

/// `.main_part1_rect2` of `inv_txfm_add_dct_dct_16x64_10bpc`: `.main_part1` after rounding `m0`-`m3` by `m11`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_10bpc_main_part1_rect2(m: &mut M, r5k: &mut usize, r6: &mut *mut __m256i) {
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3
    }
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, r5k, r6)
}

/// `.main_part1` of `inv_txfm_add_dct_dct_16x64_10bpc`: idct64 steps 1-5 for four inputs in `m0`-`m3` (in1, in31,
/// in17, in15 and the like) with the twelve multipliers at `r5` (`idct64_mul_16bpc`; `r5k` is the entry index,
/// moved 12 on), into `[r6-32*4]` … `[r6+32*3]`; moves `r6` eight slots on.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m: &mut M, r5k: &mut usize, r6: &mut *mut __m256i) {
    // in1/31/17/15 -> t32a/33/34a/35/60/61a/62/63a
    // in7/25/23/ 9 -> t56a/57/58a/59/36/37a/38/39a
    // in5/27/21/11 -> t40a/41/42a/43/52/53a/54/55a
    // in3/29/19/13 -> t48a/49/50a/51/44/45a/46/47a
    let k = *r5k;
    m[7] = idct64_mul_p8(k, 0); // vpbroadcastd m7, [r5+4*0]
    m[8] = idct64_mul_p8(k, 1); // vpbroadcastd m8, [r5+4*1]
    m[6] = idct64_mul_p8(k, 2); // vpbroadcastd m6, [r5+4*2]
    m[9] = idct64_mul_p8(k, 3); // vpbroadcastd m9, [r5+4*3]
    m[5] = idct64_mul_p8(k, 4); // vpbroadcastd m5, [r5+4*4]
    m[10] = idct64_mul_p8(k, 5); // vpbroadcastd m10, [r5+4*5]
    m[4] = idct64_mul_p8(k, 6); // vpbroadcastd m4, [r5+4*6]
    m[15] = idct64_mul_p8(k, 7); // vpbroadcastd m15, [r5+4*7]
    m[7] = _mm256_mullo_epi32(m[7], m[0]); // pmulld m7, m0 ; t63a
    m[0] = _mm256_mullo_epi32(m[0], m[8]); // pmulld m0, m8 ; t32a
    m[6] = _mm256_mullo_epi32(m[6], m[1]); // pmulld m6, m1 ; t62a
    m[1] = _mm256_mullo_epi32(m[1], m[9]); // pmulld m1, m9 ; t33a
    m[5] = _mm256_mullo_epi32(m[5], m[2]); // pmulld m5, m2 ; t61a
    m[2] = _mm256_mullo_epi32(m[2], m[10]); // pmulld m2, m10 ; t34a
    m[4] = _mm256_mullo_epi32(m[4], m[3]); // pmulld m4, m3 ; t60a
    m[3] = _mm256_mullo_epi32(m[3], m[15]); // pmulld m3, m15 ; t35a
    m[10] = idct64_mul_p8(k, 8); // vpbroadcastd m10, [r5+4*8]
    m[15] = idct64_mul_p8(k, 9); // vpbroadcastd m15, [r5+4*9]
    for x in [7, 0, 6, 1, 5, 2, 4, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m7, m0, m6, m1, m5, m2, m4, m3
    }
    for x in [0, 1, 7, 6, 2, 3, 5, 4] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m7, m6, m2, m3, m5, m4
    }
    m[8] = _mm256_sub_epi32(m[0], m[1]); // psubd m8, m0, m1 ; t33
    m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1 ; t32
    m[1] = _mm256_sub_epi32(m[7], m[6]); // psubd m1, m7, m6 ; t62
    m[7] = _mm256_add_epi32(m[7], m[6]); // paddd m7, m6 ; t63
    m[6] = _mm256_sub_epi32(m[3], m[2]); // psubd m6, m3, m2 ; t34
    m[3] = _mm256_add_epi32(m[3], m[2]); // paddd m3, m2 ; t35
    m[2] = _mm256_sub_epi32(m[4], m[5]); // psubd m2, m4, m5 ; t61
    m[4] = _mm256_add_epi32(m[4], m[5]); // paddd m4, m5 ; t60
    clip_p8(m, [8, 1, 6, 2]); // REPX {pmaxsd x, m12}, m8, m1, m6, m2 / REPX {pminsd x, m13}, m8, m1, m6, m2
    itx_mulsub_2d(m, 1, 8, 5, 9, 0, Some(11), 10, 15, 0); // ITX_MULSUB_2D 1, 8, 5, 9, _, 11, 10, 15 ; t33a, t62a
    itx_mulsub_2d(m, 2, 6, 5, 9, 0, Some(11), 10, 15, 2); // ITX_MULSUB_2D 2, 6, 5, 9, _, 11, 10, 15, 2 ; t61a, t34a
    clip_p8(m, [0, 3, 7, 4]); // REPX {pmaxsd x, m12}, m0, m3, m7, m4 / REPX {pminsd x, m13}, m0, m3, m7, m4
    m[10] = idct64_mul_p8(k, 10); // vpbroadcastd m10, [r5+4*10]
    m[15] = idct64_mul_p8(k, 11); // vpbroadcastd m15, [r5+4*11]
    m[5] = _mm256_sub_epi32(m[0], m[3]); // psubd m5, m0, m3 ; t35a
    m[0] = _mm256_add_epi32(m[0], m[3]); // paddd m0, m3 ; t32a
    m[3] = _mm256_sub_epi32(m[7], m[4]); // psubd m3, m7, m4 ; t60a
    m[7] = _mm256_add_epi32(m[7], m[4]); // paddd m7, m4 ; t63a
    m[4] = _mm256_sub_epi32(m[1], m[6]); // psubd m4, m1, m6 ; t34
    m[1] = _mm256_add_epi32(m[1], m[6]); // paddd m1, m6 ; t33
    m[6] = _mm256_sub_epi32(m[8], m[2]); // psubd m6, m8, m2 ; t61
    m[8] = _mm256_add_epi32(m[8], m[2]); // paddd m8, m2 ; t62
    clip_p8(m, [5, 3, 4, 6]); // REPX {pmaxsd x, m12}, m5, m3, m4, m6 / REPX {pminsd x, m13}, m5, m3, m4, m6
    itx_mulsub_2d(m, 3, 5, 2, 9, 0, Some(11), 10, 15, 0); // ITX_MULSUB_2D 3, 5, 2, 9, _, 11, 10, 15 ; t35,  t60
    itx_mulsub_2d(m, 6, 4, 2, 9, 0, Some(11), 10, 15, 0); // ITX_MULSUB_2D 6, 4, 2, 9, _, 11, 10, 15 ; t34a, t61a
    clip_p8(m, [0, 7, 1, 8]); // REPX {pmaxsd x, m12}, m0, m7, m1, m8 / REPX {pminsd x, m13}, m0, m7, m1, m8
    *r5k = k + 12; // add r5, 4*12
    let p = *r6;
    st_p8(p, -32 * 4, m[0]); // mova [r6-32*4], m0
    st_p8(p, 32 * 3, m[7]); // mova [r6+32*3], m7
    st_p8(p, -32 * 3, m[1]); // mova [r6-32*3], m1
    st_p8(p, 32 * 2, m[8]); // mova [r6+32*2], m8
    st_p8(p, -32 * 2, m[6]); // mova [r6-32*2], m6
    st_p8(p, 32 * 1, m[4]); // mova [r6+32*1], m4
    st_p8(p, -32 * 1, m[3]); // mova [r6-32*1], m3
    st_p8(p, 32 * 0, m[5]); // mova [r6+32*0], m5
    *r6 = at(p, 32 * 8); // add r6, 32*8
}

/// `.main_part2` of `inv_txfm_add_dct_dct_16x64_10bpc`: idct64 steps 6-9 over the 32 slots before `r6`, in place;
/// leaves `r6` where it was and `r5` one slot before it.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_10bpc_main_part2(m: &mut M, r6: &mut *mut __m256i, r5: &mut *mut __m256i) {
    *r5 = at(*r6, 32 * 3); // lea r5, [r6+32*3]
    *r6 = at(*r6, -32 * 4); // sub r6, 32*4
    m[10] = pd(1567); // vpbroadcastd m10, [pd_1567]
    m[15] = pd(3784); // vpbroadcastd m15, [pd_3784]
    loop {
        // .main_part2_loop:
        let (p6, p5) = (*r6, *r5);
        m[0] = ld_p8(p6, -32 * 32); // mova m0, [r6-32*32] ; t32a
        m[1] = ld_p8(p5, -32 * 24); // mova m1, [r5-32*24] ; t39a
        m[2] = ld_p8(p5, -32 * 32); // mova m2, [r5-32*32] ; t63a
        m[3] = ld_p8(p6, -32 * 24); // mova m3, [r6-32*24] ; t56a
        m[4] = ld_p8(p6, -32 * 16); // mova m4, [r6-32*16] ; t40a
        m[5] = ld_p8(p5, -32 * 8); // mova m5, [r5-32* 8] ; t47a
        m[6] = ld_p8(p5, -32 * 16); // mova m6, [r5-32*16] ; t55a
        m[7] = ld_p8(p6, -32 * 8); // mova m7, [r6-32* 8] ; t48a
        m[8] = _mm256_sub_epi32(m[0], m[1]); // psubd m8, m0, m1 ; t39
        m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1 ; t32
        m[1] = _mm256_sub_epi32(m[2], m[3]); // psubd m1, m2, m3 ; t56
        m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3 ; t63
        m[3] = _mm256_sub_epi32(m[5], m[4]); // psubd m3, m5, m4 ; t40
        m[5] = _mm256_add_epi32(m[5], m[4]); // paddd m5, m4 ; t47
        m[4] = _mm256_sub_epi32(m[7], m[6]); // psubd m4, m7, m6 ; t55
        m[7] = _mm256_add_epi32(m[7], m[6]); // paddd m7, m6 ; t48
        clip_p8(m, [8, 1, 3, 4]); // REPX {pmaxsd x, m12}, m8, m1, m3, m4 / REPX {pminsd x, m13}, m8, m1, m3, m4
        itx_mulsub_2d(m, 1, 8, 6, 9, 0, Some(11), 10, 15, 0); // ITX_MULSUB_2D 1, 8, 6, 9, _, 11, 10, 15 ; t39a, t56a
        itx_mulsub_2d(m, 4, 3, 6, 9, 0, Some(11), 10, 15, 2); // ITX_MULSUB_2D 4, 3, 6, 9, _, 11, 10, 15, 2 ; t55a, t40a
        for x in [0, 2, 5, 7] {
            m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m0, m2, m5, m7
        }
        for x in [0, 5, 2, 7] {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m0, m5, m2, m7
        }
        m[6] = _mm256_sub_epi32(m[2], m[7]); // psubd m6, m2, m7 ; t48a
        m[2] = _mm256_add_epi32(m[2], m[7]); // paddd m2, m7 ; t63a
        m[7] = _mm256_sub_epi32(m[0], m[5]); // psubd m7, m0, m5 ; t47a
        m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5 ; t32a
        m[5] = _mm256_sub_epi32(m[8], m[4]); // psubd m5, m8, m4 ; t55
        m[8] = _mm256_add_epi32(m[8], m[4]); // paddd m8, m4 ; t56
        m[4] = _mm256_sub_epi32(m[1], m[3]); // psubd m4, m1, m3 ; t40
        m[1] = _mm256_add_epi32(m[1], m[3]); // paddd m1, m3 ; t39
        clip_p8(m, [6, 7, 5, 4]); // REPX {pmaxsd x, m12}, m6, m7, m5, m4 / REPX {pminsd x, m13}, m6, m7, m5, m4
        for x in [6, 7, 5, 4] {
            m[x] = _mm256_mullo_epi32(m[x], m[14]); // REPX {pmulld x, m14}, m6, m7, m5, m4
        }
        clip_p8(m, [2, 0, 8, 1]); // REPX {pmaxsd x, m12}, m2, m0, m8, m1 / REPX {pminsd x, m13}, m2, m0, m8, m1
        m[6] = _mm256_add_epi32(m[6], m[11]); // paddd m6, m11
        m[5] = _mm256_add_epi32(m[5], m[11]); // paddd m5, m11
        m[3] = _mm256_sub_epi32(m[6], m[7]); // psubd m3, m6, m7 ; t47
        m[6] = _mm256_add_epi32(m[6], m[7]); // paddd m6, m7 ; t48
        m[7] = _mm256_sub_epi32(m[5], m[4]); // psubd m7, m5, m4 ; t40a
        m[5] = _mm256_add_epi32(m[5], m[4]); // paddd m5, m4 ; t55a
        for x in [3, 6, 7, 5] {
            m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12}, m3, m6, m7, m5
        }
        st_p8(p5, -32 * 8, m[2]); // mova [r5-32* 8], m2
        st_p8(p6, -32 * 32, m[0]); // mova [r6-32*32], m0
        st_p8(p6, -32 * 8, m[8]); // mova [r6-32* 8], m8
        st_p8(p5, -32 * 32, m[1]); // mova [r5-32*32], m1
        st_p8(p5, -32 * 24, m[3]); // mova [r5-32*24], m3
        st_p8(p6, -32 * 16, m[6]); // mova [r6-32*16], m6
        st_p8(p6, -32 * 24, m[7]); // mova [r6-32*24], m7
        st_p8(p5, -32 * 16, m[5]); // mova [r5-32*16], m5
        *r6 = at(p6, 32); // add r6, 32
        *r5 = at(p5, -32); // sub r5, 32
        if *r6 >= *r5 {
            break; // cmp r6, r5 / jl .main_part2_loop
        }
    }
}

/// `cglobal inv_txfm_add_dct_dct_16x64_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_16x64_10bpc(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 64; // or r3d, 64
        r6d = r6d.wrapping_add(640); // add r6d, 640
        r6d >>= 10; // sar r6d, 10
        return inv_txfm_add_dct_dct_16x4_10bpc_dconly3(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_16x4_10bpc).dconly3
    }
    // PROLOGUE 0, 10, 16, 32*98, dst, stride, c, eob. The frame is zeroed whole (see the top of this part).
    let mut frame = [_mm256_setzero_si256(); 98];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut cq = c;
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    let mut r6 = at(rsp, 32 * 6); // lea r6, [rsp+32*6]
    let mut eobd = eob;
    let fast = 'pass1: {
        inv_txfm_add_dct_dct_16x64_10bpc_main(m, &mut cq, &mut r6); // call .main
        eobd = eobd.wrapping_sub(44); // sub eobd, 44
        if eobd < 0 {
            break 'pass1 true; // jl .fast
        }
        inv_txfm_add_dct_dct_16x64_10bpc_main(m, &mut cq, &mut r6); // call .main
        eobd = eobd.wrapping_sub(107); // sub eobd, 107
        if eobd < 0 {
            break 'pass1 true; // jl .fast
        }
        inv_txfm_add_dct_dct_16x64_10bpc_main(m, &mut cq, &mut r6); // call .main
        eobd = eobd.wrapping_sub(128); // sub eobd, 128
        if eobd < 0 {
            break 'pass1 true; // jl .fast
        }
        inv_txfm_add_dct_dct_16x64_10bpc_main(m, &mut cq, &mut r6); // call .main
        false // jmp .pass2
    };
    if fast {
        // .fast:
        let r4 = at(rsp, 32 * 38); // lea r4, [rsp+32*38]
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        loop {
            // .fast_loop:
            for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
                st_p8(r6, 32 * x, m[0]); // REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
            }
            r6 = at(r6, 32 * 8); // add r6, 32*8
            if r6 >= r4 {
                break; // cmp r6, r4 / jl .fast_loop
            }
        }
    }
    // .pass2:
    // lea r6, [pw_5+128] (the 8 bpc o_base)
    m[0] = ld_p8(rsp, 32 * 2); // mova m0, [rsp+32* 2] ; in0
    m[1] = ld_p8(rsp, 32 * 6); // mova m1, [rsp+32* 6] ; in4
    m[2] = ld_p8(rsp, 32 * 10); // mova m2, [rsp+32*10] ; in8
    m[3] = ld_p8(rsp, 32 * 14); // mova m3, [rsp+32*14] ; in12
    m[4] = ld_p8(rsp, 32 * 18); // mova m4, [rsp+32*18] ; in16
    m[5] = ld_p8(rsp, 32 * 22); // mova m5, [rsp+32*22] ; in20
    m[6] = ld_p8(rsp, 32 * 26); // mova m6, [rsp+32*26] ; in24
    m[7] = ld_p8(rsp, 32 * 30); // mova m7, [rsp+32*30] ; in28
    m[8] = _mm256_setzero_si256(); // pxor m8, m8
    for x in [9, 10, 11, 12, 13, 14] {
        m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
    }
    st_p8(rsp, 0, m[8]); // mova [rsp], m8
    idct_16x16_internal_8bpc_main(m, s3_p8(rsp)); // call m(idct_16x16_internal_8bpc).main
    m[1] = ld_p8(rsp, 32 * 1); // mova m1, [rsp+32*1]
    let mut r4 = at(rsp, 32 * 38); // lea r4, [rsp+32*38]
    for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
        st_p8(r4, 32 * x, m[i]); // mova [r4-32*4], m0 … mova [r4+32*3], m7
    }
    r4 = at(r4, 32 * 8); // add r4, 32*8
    for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
        st_p8(r4, 32 * x, m[8 + i]); // mova [r4-32*4], m8 … mova [r4+32*3], m15
    }
    m[0] = ld_p8(rsp, 32 * 4); // mova m0, [rsp+32* 4] ; in2
    m[1] = ld_p8(rsp, 32 * 8); // mova m1, [rsp+32* 8] ; in6
    m[2] = ld_p8(rsp, 32 * 12); // mova m2, [rsp+32*12] ; in10
    m[3] = ld_p8(rsp, 32 * 16); // mova m3, [rsp+32*16] ; in14
    m[4] = ld_p8(rsp, 32 * 20); // mova m4, [rsp+32*20] ; in18
    m[5] = ld_p8(rsp, 32 * 24); // mova m5, [rsp+32*24] ; in22
    m[6] = ld_p8(rsp, 32 * 28); // mova m6, [rsp+32*28] ; in26
    m[7] = ld_p8(rsp, 32 * 32); // mova m7, [rsp+32*32] ; in30
    let mut r5 = at(r4, 32 * 16); // lea r5, [r4+32*16]
    r4 = at(r4, 32 * 8); // add r4, 32*8
    inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf_fast(m, s3_p8(rsp), r4, r5); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
    m[0] = ld_p8(rsp, 32 * 3); // mova m0, [rsp+32* 3] ; in1
    m[1] = ld_p8(rsp, 32 * 33); // mova m1, [rsp+32*33] ; in31
    m[2] = ld_p8(rsp, 32 * 19); // mova m2, [rsp+32*19] ; in17
    m[3] = ld_p8(rsp, 32 * 17); // mova m3, [rsp+32*17] ; in15
    m[4] = ld_p8(rsp, 32 * 11); // mova m4, [rsp+32*11] ; in9
    m[5] = ld_p8(rsp, 32 * 25); // mova m5, [rsp+32*25] ; in23
    m[6] = ld_p8(rsp, 32 * 27); // mova m6, [rsp+32*27] ; in25
    m[7] = ld_p8(rsp, 32 * 9); // mova m7, [rsp+32* 9] ; in7
                               // lea r6, [idct64_mul - 8]
    r4 = at(r4, 32 * 16); // add r4, 32*16
    r5 = at(r5, 32 * 32); // add r5, 32*32
    inv_txfm_add_dct_dct_16x64_8bpc_main_part1(m, 0, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    m[0] = ld_p8(rsp, 32 * 7); // mova m0, [rsp+32* 7] ; in5
    m[1] = ld_p8(rsp, 32 * 29); // mova m1, [rsp+32*29] ; in27
    m[2] = ld_p8(rsp, 32 * 23); // mova m2, [rsp+32*23] ; in21
    m[3] = ld_p8(rsp, 32 * 13); // mova m3, [rsp+32*13] ; in11
    m[4] = ld_p8(rsp, 32 * 15); // mova m4, [rsp+32*15] ; in13
    m[5] = ld_p8(rsp, 32 * 21); // mova m5, [rsp+32*21] ; in19
    m[6] = ld_p8(rsp, 32 * 31); // mova m6, [rsp+32*31] ; in29
    m[7] = ld_p8(rsp, 32 * 5); // mova m7, [rsp+32* 5] ; in3
                               // add r6, 8
    r4 = at(r4, 32 * 8); // add r4, 32*8
    r5 = at(r5, -32 * 8); // sub r5, 32*8
    inv_txfm_add_dct_dct_16x64_8bpc_main_part1(m, 2, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
    let r8 = stride * 4; // lea r8, [strideq*4]
    let r9 = stride * 5; // lea r9, [strideq*5]
    let r3 = r9 + stride; // lea r3, [r9+strideq*1] ; stride*6
    let r7 = r9 + stride * 2; // lea r7, [r9+strideq*2] ; stride*7
    inv_txfm_add_dct_dct_16x64_10bpc_main_part2_pass2(m, &mut dst, r7, stride, r3, r8, r9, &mut r4, &mut r5);
    // call .main_part2_pass2
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_16x64_10bpc_avx2,
    inv_txfm_add_dct_dct_16x64_10bpc
);

// --- 32x64 ------------------------------------------------------------------------------------------------------

/// `.main` of `inv_txfm_add_dct_dct_32x64_10bpc`: the 32-point DCT (rect2) of eight columns of 32 coefficients
/// (`[cq+128*0..31]`, which it clears), packed to words and transposed into the 16 slots at `r6` (moved 16 on);
/// `r4` and `r5` are left at their two halves.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x64_10bpc_main(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    let c = *cq;
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    for (i, k) in [1, 7, 9, 15, 17, 23, 25, 31].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 1] … pmulld m7, m14, [cq+128*31]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_rect2(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_rect2
    for (i, k) in [3, 5, 11, 13, 19, 21, 27, 29].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 3] … pmulld m7, m14, [cq+128*29]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_rect2(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_rect2
    for (i, k) in [2, 6, 10, 14, 18, 22, 26, 30].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 2] … pmulld m7, m14, [cq+128*30]
    }
    idct_8x16_internal_10bpc_main_oddhalf_rect2(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_rect2
    for (i, k) in [0, 4, 8, 12, 16, 20, 24, 28].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 0] … pmulld m7, m14, [cq+128*28]
    }
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r7d: i32 = 128 * 29; // mov r7d, 128*29
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        stu_p8(c, r7 - 128 * 1, m[15]); // mova [cq+r7-128*1], m15
        stu_p8(c, r7 + 128 * 0, m[15]); // mova [cq+r7+128*0], m15
        stu_p8(c, r7 + 128 * 1, m[15]); // mova [cq+r7+128*1], m15
        stu_p8(c, r7 + 128 * 2, m[15]); // mova [cq+r7+128*2], m15
        r7d -= 128 * 4; // sub r7d, 128*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    *cq = at(c, 32); // add cq, 32
    idct_8x8_internal_10bpc_main_rect2(m); // call m(idct_8x8_internal_10bpc).main_rect2
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    inv_txfm_add_dct_dct_32x16_10bpc_main_end(m, *r6, *r5, *r4); // call m(inv_txfm_add_dct_dct_32x16_10bpc).main_end
    inv_txfm_add_dct_dct_8x32_10bpc_transpose(m); // call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    let p4 = *r4;
    for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
        st_p8(p4, 32 * x, m[i]); // mova [r4-32*4], m0 … mova [r4+32*3], m7
    }
    let p5 = *r5;
    for (i, x) in [3, 2, 1, 0, -1, -2, -3, -4].into_iter().enumerate() {
        m[i] = ld_p8(p5, 32 * x); // mova m0, [r5+32*3] … mova m7, [r5-32*4]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_transpose(m); // call m(inv_txfm_add_dct_dct_8x32_10bpc).transpose
    for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
        st_p8(p5, 32 * x, m[i]); // mova [r5-32*4], m0 … mova [r5+32*3], m7
    }
}

/// `cglobal inv_txfm_add_dct_dct_32x64_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_32x64_10bpc(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        m[3] = pw(0x7c00); // vpbroadcastd m3, [dconly_10bpc]
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 64; // or r3d, 64
        r6d = r6d.wrapping_add(128); // add r6d, 128
        r6d >>= 8; // sar r6d, 8
        r6d = r6d.wrapping_mul(181); // imul r6d, 181
        r6d = r6d.wrapping_add(384); // add r6d, 384
        r6d >>= 9; // sar r6d, 9
        return inv_txfm_add_dct_dct_32x8_10bpc_dconly2(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_32x8_10bpc).dconly2
    }
    // PROLOGUE 0, 11, 16, 32*134, dst, stride, c, eob. The frame is zeroed whole (see the top of this part).
    let mut frame = [_mm256_setzero_si256(); 134];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut cq = c;
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    let mut r6 = at(rsp, 32 * 6); // lea r6, [rsp+32*6]
    let (mut r4, mut r5) = (rsp, rsp);
    let fast = 'pass1: {
        inv_txfm_add_dct_dct_32x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 36 {
            break 'pass1 true; // cmp eobd, 36 / jl .fast
        }
        inv_txfm_add_dct_dct_32x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 136 {
            break 'pass1 true; // cmp eobd, 136 / jl .fast
        }
        inv_txfm_add_dct_dct_32x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 300 {
            break 'pass1 true; // cmp eobd, 300 / jl .fast
        }
        inv_txfm_add_dct_dct_32x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        false // jmp .pass2
    };
    if fast {
        // .fast:
        r4 = at(rsp, 32 * 70); // lea r4, [rsp+32*70]
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        loop {
            // .fast_loop:
            for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
                st_p8(r6, 32 * x, m[0]); // REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
            }
            r6 = at(r6, 32 * 8); // add r6, 32*8
            if r6 >= r4 {
                break; // cmp r6, r4 / jl .fast_loop
            }
        }
    }
    // .pass2:
    // lea r6, [pw_5 + 128] (the 8 bpc o_base)
    let mut r10 = rsp; // mov r10, rsp
    let r8 = stride * 4; // lea r8, [strideq*4]
    let r9 = stride * 5; // lea r9, [strideq*5]
    let r3 = r9 + stride; // lea r3, [r9+strideq*1] ; stride*6
    let r7 = r9 + stride * 2; // lea r7, [r9+strideq*2] ; stride*7
    loop {
        // .pass2_loop:
        m[0] = ld_p8(r10, 32 * 2); // mova m0, [r10+32* 2] ; in0
        m[1] = ld_p8(r10, 32 * 6); // mova m1, [r10+32* 6] ; in4
        m[2] = ld_p8(r10, 32 * 18); // mova m2, [r10+32*18] ; in8
        m[3] = ld_p8(r10, 32 * 22); // mova m3, [r10+32*22] ; in12
        m[4] = ld_p8(r10, 32 * 34); // mova m4, [r10+32*34] ; in16
        m[5] = ld_p8(r10, 32 * 38); // mova m5, [r10+32*38] ; in20
        m[6] = ld_p8(r10, 32 * 50); // mova m6, [r10+32*50] ; in24
        m[7] = ld_p8(r10, 32 * 54); // mova m7, [r10+32*54] ; in28
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
        }
        st_p8(rsp, 0, m[8]); // mova [rsp], m8
        idct_16x16_internal_8bpc_main(m, s3_p8(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p8(rsp, 32 * 1); // mova m1, [rsp+32*1]
        r4 = at(rsp, 32 * 70); // lea r4, [rsp+32*70]
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            st_p8(r4, 32 * x, m[i]); // mova [r4-32*4], m0 … mova [r4+32*3], m7
        }
        r4 = at(r4, 32 * 8); // add r4, 32*8
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            st_p8(r4, 32 * x, m[8 + i]); // mova [r4-32*4], m8 … mova [r4+32*3], m15
        }
        m[0] = ld_p8(r10, 32 * 4); // mova m0, [r10+32* 4] ; in2
        m[1] = ld_p8(r10, 32 * 8); // mova m1, [r10+32* 8] ; in6
        m[2] = ld_p8(r10, 32 * 20); // mova m2, [r10+32*20] ; in10
        m[3] = ld_p8(r10, 32 * 24); // mova m3, [r10+32*24] ; in14
        m[4] = ld_p8(r10, 32 * 36); // mova m4, [r10+32*36] ; in18
        m[5] = ld_p8(r10, 32 * 40); // mova m5, [r10+32*40] ; in22
        m[6] = ld_p8(r10, 32 * 52); // mova m6, [r10+32*52] ; in26
        m[7] = ld_p8(r10, 32 * 56); // mova m7, [r10+32*56] ; in30
        r5 = at(r4, 32 * 16); // lea r5, [r4+32*16]
        r4 = at(r4, 32 * 8); // add r4, 32*8
        inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf_fast(m, s3_p8(rsp), r4, r5); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[0] = ld_p8(r10, 32 * 3); // mova m0, [r10+32* 3] ; in1
        m[1] = ld_p8(r10, 32 * 57); // mova m1, [r10+32*57] ; in31
        m[2] = ld_p8(r10, 32 * 35); // mova m2, [r10+32*35] ; in17
        m[3] = ld_p8(r10, 32 * 25); // mova m3, [r10+32*25] ; in15
        m[4] = ld_p8(r10, 32 * 19); // mova m4, [r10+32*19] ; in9
        m[5] = ld_p8(r10, 32 * 41); // mova m5, [r10+32*41] ; in23
        m[6] = ld_p8(r10, 32 * 51); // mova m6, [r10+32*51] ; in25
        m[7] = ld_p8(r10, 32 * 9); // mova m7, [r10+32* 9] ; in7
                                   // lea r6, [idct64_mul - 8]
        r4 = at(r4, 32 * 16); // add r4, 32*16
        r5 = at(r5, 32 * 32); // add r5, 32*32
        inv_txfm_add_dct_dct_16x64_8bpc_main_part1(m, 0, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        m[0] = ld_p8(r10, 32 * 7); // mova m0, [r10+32* 7] ; in5
        m[1] = ld_p8(r10, 32 * 53); // mova m1, [r10+32*53] ; in27
        m[2] = ld_p8(r10, 32 * 39); // mova m2, [r10+32*39] ; in21
        m[3] = ld_p8(r10, 32 * 21); // mova m3, [r10+32*21] ; in11
        m[4] = ld_p8(r10, 32 * 23); // mova m4, [r10+32*23] ; in13
        m[5] = ld_p8(r10, 32 * 37); // mova m5, [r10+32*37] ; in19
        m[6] = ld_p8(r10, 32 * 55); // mova m6, [r10+32*55] ; in29
        m[7] = ld_p8(r10, 32 * 5); // mova m7, [r10+32* 5] ; in3
                                   // add r6, 8
        r4 = at(r4, 32 * 8); // add r4, 32*8
        r5 = at(r5, -32 * 8); // sub r5, 32*8
        inv_txfm_add_dct_dct_16x64_8bpc_main_part1(m, 2, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        inv_txfm_add_dct_dct_16x64_10bpc_main_part2_pass2(m, &mut dst, r7, stride, r3, r8, r9, &mut r4, &mut r5);
        // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2_pass2
        r10 = at(r10, 32 * 8); // add r10, 32*8
        r4 = at(r4, -32 * 98); // sub r4, 32*98 ; rsp+32*16
        dst = at(dst, -r8); // sub dstq, r8
        dst = at(dst, 32); // add dstq, 32
        if r10 >= r4 {
            break; // cmp r10, r4 / jl .pass2_loop
        }
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_32x64_10bpc_avx2,
    inv_txfm_add_dct_dct_32x64_10bpc
);

// --- 64x16 ------------------------------------------------------------------------------------------------------

/// `.dconly` of `inv_txfm_add_dct_dct_64x16_10bpc` (also the DC-only tail of 64x64): entered after `imul r6d,
/// [cq], 181`, `mov [cq], eobd` and `or r3d, <rows>`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc_dconly(m: &mut M, dst: Dst, stride: isize, mut r6d: i32, r3d: i32) {
    r6d = r6d.wrapping_add(640); // add r6d, 640
    r6d >>= 10; // sar r6d, 10
    inv_txfm_add_dct_dct_64x16_10bpc_dconly2(m, dst, stride, r6d, r3d)
}

/// `.dconly2` of `inv_txfm_add_dct_dct_64x16_10bpc` (also the DC-only tail of 64x32; `.dconly_loop`: one row of
/// 64 pixels per iteration)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc_dconly2(m: &mut M, mut dst: Dst, stride: isize, mut r6d: i32, mut r3d: i32) {
    m[5] = pw(0x7c00); // vpbroadcastd m5, [dconly_10bpc]
    r6d = r6d.wrapping_mul(181); // imul r6d, 181
    r6d = r6d.wrapping_add(2176); // add r6d, 2176
    r6d >>= 12; // sar r6d, 12
    m[0] = zx(_mm_cvtsi32_si128(r6d)); // movd xm0, r6d
    m[0] = zx(_mm_adds_epi16(lo(m[0]), lo(m[5]))); // paddsw xm0, xm5
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    loop {
        // .dconly_loop:
        m[1] = _mm256_adds_epi16(m[0], ldu_p8(dst, 32 * 0)); // paddsw m1, m0, [dstq+32*0]
        m[2] = _mm256_adds_epi16(m[0], ldu_p8(dst, 32 * 1)); // paddsw m2, m0, [dstq+32*1]
        m[3] = _mm256_adds_epi16(m[0], ldu_p8(dst, 32 * 2)); // paddsw m3, m0, [dstq+32*2]
        m[4] = _mm256_adds_epi16(m[0], ldu_p8(dst, 32 * 3)); // paddsw m4, m0, [dstq+32*3]
        for x in [1, 2, 3, 4] {
            m[x] = _mm256_subs_epu16(m[x], m[5]); // REPX {psubusw x, m5}, m1, m2, m3, m4
        }
        stu_p8(dst, 32 * 0, m[1]); // mova [dstq+32*0], m1
        stu_p8(dst, 32 * 1, m[2]); // mova [dstq+32*1], m2
        stu_p8(dst, 32 * 2, m[3]); // mova [dstq+32*2], m3
        stu_p8(dst, 32 * 3, m[4]); // mova [dstq+32*3], m4
        dst = at(dst, stride); // add dstq, strideq
        r3d -= 1; // dec r3d
        if r3d <= 0 {
            break; // jg .dconly_loop
        }
    }
}

/// `.main` of `inv_txfm_add_dct_dct_64x16_10bpc`: the 64-point DCT of eight rows of 32 coefficients
/// (`[cq+64*0..31]`, which it clears) into the frame at `r6` (`.main_end` follows: the results stay 32-bit until
/// `.shift_transpose`). `r4` and `r5` are left as `.main_end` and `.shift_transpose` use them.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc_main(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    let c = *cq;
    let mut r5k = 0; // lea r5, [idct64_mul_16bpc]
    m[0] = ldu_p8(c, 64 * 1); // mova m0, [cq+64* 1]
    m[1] = ldu_p8(c, 64 * 31); // mova m1, [cq+64*31]
    m[2] = ldu_p8(c, 64 * 17); // mova m2, [cq+64*17]
    m[3] = ldu_p8(c, 64 * 15); // mova m3, [cq+64*15]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    m[0] = ldu_p8(c, 64 * 7); // mova m0, [cq+64* 7]
    m[1] = ldu_p8(c, 64 * 25); // mova m1, [cq+64*25]
    m[2] = ldu_p8(c, 64 * 23); // mova m2, [cq+64*23]
    m[3] = ldu_p8(c, 64 * 9); // mova m3, [cq+64* 9]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    m[0] = ldu_p8(c, 64 * 5); // mova m0, [cq+64* 5]
    m[1] = ldu_p8(c, 64 * 27); // mova m1, [cq+64*27]
    m[2] = ldu_p8(c, 64 * 21); // mova m2, [cq+64*21]
    m[3] = ldu_p8(c, 64 * 11); // mova m3, [cq+64*11]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    m[0] = ldu_p8(c, 64 * 3); // mova m0, [cq+64* 3]
    m[1] = ldu_p8(c, 64 * 29); // mova m1, [cq+64*29]
    m[2] = ldu_p8(c, 64 * 19); // mova m2, [cq+64*19]
    m[3] = ldu_p8(c, 64 * 13); // mova m3, [cq+64*13]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    inv_txfm_add_dct_dct_16x64_10bpc_main_part2(m, r6, r5); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2
    for (i, k) in [2, 14, 18, 30].into_iter().enumerate() {
        m[i] = ldu_p8(c, 64 * k); // mova m0, [cq+64* 2] … mova m3, [cq+64*30]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast
    for (i, k) in [6, 10, 22, 26].into_iter().enumerate() {
        m[i] = ldu_p8(c, 64 * k); // mova m0, [cq+64* 6] … mova m3, [cq+64*26]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast
    for (i, k) in [4, 12, 20, 28].into_iter().enumerate() {
        m[i] = ldu_p8(c, 64 * k); // mova m0, [cq+64* 4] … mova m3, [cq+64*28]
    }
    idct_8x16_internal_10bpc_main_oddhalf_fast(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_fast
    for (i, k) in [0, 8, 16, 24].into_iter().enumerate() {
        m[i] = ldu_p8(c, 64 * k); // mova m0, [cq+64* 0] … mova m3, [cq+64*24]
    }
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r7d: i32 = 64 * 30; // mov r7d, 64*30
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        stu_p8(c, r7 - 64 * 2, m[15]); // mova [cq+r7-64*2], m15
        stu_p8(c, r7 - 64 * 1, m[15]); // mova [cq+r7-64*1], m15
        stu_p8(c, r7 + 64 * 0, m[15]); // mova [cq+r7+64*0], m15
        stu_p8(c, r7 + 64 * 1, m[15]); // mova [cq+r7+64*1], m15
        r7d -= 64 * 4; // sub r7d, 64*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    inv_txfm_add_dct_dct_64x16_10bpc_main_end(m, cq, r6, r5)
}

/// `.main_end` of `inv_txfm_add_dct_dct_64x16_10bpc` (`m15` = `pd_2`, then `.main_end2`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc_main_end(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    m[15] = _mm256_srli_epi32::<10>(m[11]); // psrld m15, m11, 10 ; pd_2
    inv_txfm_add_dct_dct_64x16_10bpc_main_end2(m, cq, r6, r5)
}

/// `.main_end2` of `inv_txfm_add_dct_dct_64x16_10bpc`: the even quarter (in0, in8, in16, in24 in `m0`-`m3`), then
/// the idct16/32/64 sums and differences in the frame (`m15` the rounding added before the shift); moves `cq` 32
/// bytes on; `r5` and `r6` meet.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc_main_end2(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    *cq = at(*cq, 32); // add cq, 32
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    for x in [5, 6, 7] {
        m[x] = m[4]; // REPX {mova x, m4}, m5, m6, m7
    }
    idct_8x8_internal_10bpc_main(m); // call m(idct_8x8_internal_10bpc).main
    *r6 = at(*r6, 32 * 8); // add r6, 32*8
    idct_8x16_internal_10bpc_main_evenhalf(m); // call m(idct_8x16_internal_10bpc).main_evenhalf
    let p = *r6;
    st_p8(p, 32 * 2, m[1]); // mova [r6+32*2], m1
    st_p8(p, 32 * 1, m[2]); // mova [r6+32*1], m2
    st_p8(p, 32 * 0, m[3]); // mova [r6+32*0], m3
    st_p8(p, -32 * 1, m[4]); // mova [r6-32*1], m4
    st_p8(p, -32 * 2, m[5]); // mova [r6-32*2], m5
    st_p8(p, -32 * 3, m[6]); // mova [r6-32*3], m6
    st_p8(p, -32 * 4, m[7]); // mova [r6-32*4], m7
                             // jmp .main_end_loop_start
    loop {
        // .main_end_loop_start:
        let (p5, p6) = (*r5, *r6);
        m[1] = ld_p8(p5, 32 * 4); // mova m1, [r5+32* 4] ; idct16 15 - n
        m[2] = ld_p8(p5, -32 * 12); // mova m2, [r5-32*12] ; idct32 16 + n
        m[3] = ld_p8(p6, -32 * 13); // mova m3, [r6-32*13] ; idct32 31 - n
        m[4] = ld_p8(p6, -32 * 29); // mova m4, [r6-32*29] ; idct64 63 - n
        m[5] = ld_p8(p5, -32 * 28); // mova m5, [r5-32*28] ; idct64 48 + n
        m[6] = ld_p8(p6, -32 * 45); // mova m6, [r6-32*45] ; idct64 47 - n
        m[7] = ld_p8(p5, -32 * 44); // mova m7, [r5-32*44] ; idct64 32 + n
        m[8] = _mm256_add_epi32(m[0], m[1]); // paddd m8, m0, m1 ; idct16 out0  + n
        m[0] = _mm256_sub_epi32(m[0], m[1]); // psubd m0, m1 ; idct16 out15 - n
        clip_p8(m, [8, 0]); // REPX {pmaxsd x, m12}, m8, m0 / REPX {pminsd x, m13}, m8, m0
        m[1] = _mm256_add_epi32(m[8], m[3]); // paddd m1, m8, m3 ; idct32 out0  + n
        m[8] = _mm256_sub_epi32(m[8], m[3]); // psubd m8, m3 ; idct32 out31 - n
        m[3] = _mm256_add_epi32(m[0], m[2]); // paddd m3, m0, m2 ; idct32 out15 - n
        m[0] = _mm256_sub_epi32(m[0], m[2]); // psubd m0, m2 ; idct32 out16 + n
        for x in [1, 8, 3, 0] {
            m[x] = _mm256_max_epi32(m[x], m[12]); // REPX {pmaxsd x, m12}, m1, m8, m3, m0
        }
        for x in [1, 3, 8, 0] {
            m[x] = _mm256_min_epi32(m[x], m[13]); // REPX {pminsd x, m13}, m1, m3, m8, m0
        }
        for x in [1, 3, 0, 8] {
            m[x] = _mm256_add_epi32(m[x], m[15]); // REPX {paddd  x, m15}, m1, m3, m0, m8
        }
        m[2] = _mm256_add_epi32(m[1], m[4]); // paddd m2, m1, m4 ; idct64 out0  + n (unshifted)
        m[1] = _mm256_sub_epi32(m[1], m[4]); // psubd m1, m4 ; idct64 out63 - n (unshifted)
        m[4] = _mm256_add_epi32(m[3], m[5]); // paddd m4, m3, m5 ; idct64 out15 - n (unshifted)
        m[3] = _mm256_sub_epi32(m[3], m[5]); // psubd m3, m5 ; idct64 out48 + n (unshifted)
        m[5] = _mm256_add_epi32(m[0], m[6]); // paddd m5, m0, m6 ; idct64 out16 + n (unshifted)
        m[0] = _mm256_sub_epi32(m[0], m[6]); // psubd m0, m6 ; idct64 out47 - n (unshifted)
        m[6] = _mm256_add_epi32(m[8], m[7]); // paddd m6, m8, m7 ; idct64 out31 - n (unshifted)
        m[8] = _mm256_sub_epi32(m[8], m[7]); // psubd m8, m7 ; idct64 out32 + n (unshifted)
        st_p8(p5, -32 * 44, m[2]); // mova [r5-32*44], m2
        st_p8(p6, 32 * 3, m[1]); // mova [r6+32* 3], m1
        st_p8(p6, -32 * 45, m[4]); // mova [r6-32*45], m4
        st_p8(p5, 32 * 4, m[3]); // mova [r5+32* 4], m3
        st_p8(p5, -32 * 28, m[5]); // mova [r5-32*28], m5
        st_p8(p6, -32 * 13, m[0]); // mova [r6-32*13], m0
        st_p8(p6, -32 * 29, m[6]); // mova [r6-32*29], m6
        st_p8(p5, -32 * 12, m[8]); // mova [r5-32*12], m8
        *r5 = at(p5, 32); // add r5, 32
        *r6 = at(p6, -32); // sub r6, 32
        if *r5 >= *r6 {
            break; // cmp r5, r6 / jl .main_end_loop
        }
        // .main_end_loop:
        m[0] = ld_p8(*r6, 32 * 3); // mova m0, [r6+32* 3] ; idct8  0  + n
    }
}

/// `%macro IDCT64_SHIFT_TRANSPOSE shift`: the 64 rows of eight 32-bit columns below `r6` are shifted, packed to
/// words and transposed into eight-slot groups from `r6-32*48` (`r5`) up to `r4`; `r6` ends at `r4`.
#[inline(always)]
unsafe fn idct64_shift_transpose<const SHIFT: i32>(
    m: &mut M,
    r6: &mut *mut __m256i,
    r5: &mut *mut __m256i,
    r4: *mut __m256i,
) {
    *r6 = at(*r6, -32 * 48); // sub r6, 32*48
    *r5 = *r6; // mov r5, r6
    loop {
        // %%loop:
        let p = *r6;
        m[0] = ld_p8(p, -32 * 4); // mova m0, [r6-32* 4]
        m[4] = ld_p8(p, 32 * 4); // mova m4, [r6+32* 4]
        m[1] = ld_p8(p, -32 * 3); // mova m1, [r6-32* 3]
        m[5] = ld_p8(p, 32 * 5); // mova m5, [r6+32* 5]
        m[2] = ld_p8(p, -32 * 2); // mova m2, [r6-32* 2]
        m[6] = ld_p8(p, 32 * 6); // mova m6, [r6+32* 6]
        m[3] = ld_p8(p, -32 * 1); // mova m3, [r6-32* 1]
        m[7] = ld_p8(p, 32 * 7); // mova m7, [r6+32* 7]
        for x in [0, 4, 1, 5, 2, 6, 3, 7] {
            m[x] = _mm256_srai_epi32::<SHIFT>(m[x]); // REPX {psrad x, %1}, m0, m4, m1, m5, m2, m6, m3, m7
        }
        m[0] = _mm256_packs_epi32(m[0], m[4]); // packssdw m0, m4
        m[1] = _mm256_packs_epi32(m[1], m[5]); // packssdw m1, m5
        m[2] = _mm256_packs_epi32(m[2], m[6]); // packssdw m2, m6
        m[3] = _mm256_packs_epi32(m[3], m[7]); // packssdw m3, m7
        m[4] = ld_p8(p, 32 * 0); // mova m4, [r6+32* 0]
        m[6] = ld_p8(p, 32 * 8); // mova m6, [r6+32* 8]
        m[5] = ld_p8(p, 32 * 1); // mova m5, [r6+32* 1]
        m[7] = ld_p8(p, 32 * 9); // mova m7, [r6+32* 9]
        for x in [4, 6, 5, 7] {
            m[x] = _mm256_srai_epi32::<SHIFT>(m[x]); // REPX {psrad x, %1}, m4, m6, m5, m7
        }
        m[4] = _mm256_packs_epi32(m[4], m[6]); // packssdw m4, m6
        m[5] = _mm256_packs_epi32(m[5], m[7]); // packssdw m5, m7
        m[6] = ld_p8(p, 32 * 2); // mova m6, [r6+32* 2]
        m[8] = ld_p8(p, 32 * 10); // mova m8, [r6+32*10]
        m[7] = ld_p8(p, 32 * 3); // mova m7, [r6+32* 3]
        m[9] = ld_p8(p, 32 * 11); // mova m9, [r6+32*11]
        for x in [6, 8, 7, 9] {
            m[x] = _mm256_srai_epi32::<SHIFT>(m[x]); // REPX {psrad x, %1}, m6, m8, m7, m9
        }
        m[6] = _mm256_packs_epi32(m[6], m[8]); // packssdw m6, m8
        m[7] = _mm256_packs_epi32(m[7], m[9]); // packssdw m7, m9
        idct_16x8_internal_10bpc_transpose3(m); // call m(idct_16x8_internal_10bpc).transpose3
        let q = *r5;
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            st_p8(q, 32 * x, m[i]); // mova [r5-32*4], m0 … mova [r5+32*3], m7
        }
        *r6 = at(p, 32 * 16); // add r6, 32*16
        *r5 = at(q, 32 * 8); // add r5, 32*8
        if *r5 >= r4 {
            break; // cmp r5, r4 / jl %%loop
        }
    }
    *r6 = r4; // mov r6, r4
}

/// `.shift_transpose` of `inv_txfm_add_dct_dct_64x16_10bpc` (`IDCT64_SHIFT_TRANSPOSE 2`; also the tail of 64x64's
/// `.main`)
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc_shift_transpose(
    m: &mut M,
    r6: &mut *mut __m256i,
    r5: &mut *mut __m256i,
    r4: *mut __m256i,
) {
    idct64_shift_transpose::<2>(m, r6, r5, r4) // IDCT64_SHIFT_TRANSPOSE 2
}

/// `cglobal inv_txfm_add_dct_dct_64x16_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x16_10bpc(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jnz .normal
        let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 16; // or r3d, 16
        return inv_txfm_add_dct_dct_64x16_10bpc_dconly(m, dst, stride, r6d, r3d);
        // .dconly
    }
    // .normal: PROLOGUE 0, 8, 16, 32*96, dst, stride, c, eob. The frame is zeroed whole (see the top of this part).
    let mut frame = [_mm256_setzero_si256(); 96];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut cq = c;
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    let mut r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    let (mut r4, mut r5) = (rsp, rsp);
    inv_txfm_add_dct_dct_64x16_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
    inv_txfm_add_dct_dct_64x16_10bpc_shift_transpose(m, &mut r6, &mut r5, r4); // call .shift_transpose
    if eob < 36 {
        // cmp eobd, 36 / jl .fast
        // .fast:
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        let mut r3d = 4; // mov r3d, 4
        loop {
            // .fast_loop:
            for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
                st_p8(r6, 32 * x, m[0]); // REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
            }
            r6 = at(r6, 32 * 8); // add r6, 32*8
            r3d -= 1; // dec r3d
            if r3d <= 0 {
                break; // jg .fast_loop
            }
        }
    } else {
        inv_txfm_add_dct_dct_64x16_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        inv_txfm_add_dct_dct_64x16_10bpc_shift_transpose(m, &mut r6, &mut r5, r4);
        // call .shift_transpose
        // jmp .pass2
    }
    // .pass2:
    let mut r7 = at(r6, -32 * 64); // lea r7, [r6-32*64]
    r4 = at(r6, -32 * 32); // lea r4, [r6-32*32]
                           // lea r6, [pw_5+128] (the 8 bpc o_base)
    let mut r5d = dst; // mov r5, dstq
    loop {
        // .pass2_loop:
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            m[i] = ld_p8(r7, 32 * x); // mova m0, [r7-32*4] … mova m7, [r7+32*3]
        }
        r7 = at(r7, 32 * 32); // add r7, 32*32
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            m[8 + i] = ld_p8(r7, 32 * x); // mova m8, [r7-32*4] … mova m15, [r7+32*3]
        }
        r7 = at(r7, -32 * 24); // sub r7, 32*24
        st_p8(rsp, 0, m[15]); // mova [rsp], m15
        idct_16x16_internal_8bpc_main(m, s3_p8(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p8(rsp, 32 * 1); // mova m1, [rsp+32*1]
        inv_txfm_add_dct_dct_32x16_10bpc_write_16x16(m, s3_p8(rsp), &mut dst, stride); // call m(inv_txfm_add_dct_dct_32x16_10bpc).write_16x16
        r5d = at(r5d, 32); // add r5, 32
        dst = r5d; // mov dstq, r5
        if r7 >= r4 {
            break; // cmp r7, r4 / jl .pass2_loop
        }
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_64x16_10bpc_avx2,
    inv_txfm_add_dct_dct_64x16_10bpc
);

// --- 64x32 ------------------------------------------------------------------------------------------------------

/// `.main` of `inv_txfm_add_dct_dct_64x32_10bpc`: as 64x16's `.main` with the rect2 scaling (`pd_2896`, `m14`) of
/// eight rows of 32 coefficients (`[cq+128*0..31]`), `IDCT64_SHIFT_TRANSPOSE 1`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x32_10bpc_main(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    let c = *cq;
    let mut r5k = 0; // lea r5, [idct64_mul_16bpc]
    m[0] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 1)); // pmulld m0, m14, [cq+128* 1]
    m[1] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 31)); // pmulld m1, m14, [cq+128*31]
    m[2] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 17)); // pmulld m2, m14, [cq+128*17]
    m[3] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 15)); // pmulld m3, m14, [cq+128*15]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1_rect2(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    m[0] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 7)); // pmulld m0, m14, [cq+128* 7]
    m[1] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 25)); // pmulld m1, m14, [cq+128*25]
    m[2] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 23)); // pmulld m2, m14, [cq+128*23]
    m[3] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 9)); // pmulld m3, m14, [cq+128* 9]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1_rect2(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    m[0] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 5)); // pmulld m0, m14, [cq+128* 5]
    m[1] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 27)); // pmulld m1, m14, [cq+128*27]
    m[2] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 21)); // pmulld m2, m14, [cq+128*21]
    m[3] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 11)); // pmulld m3, m14, [cq+128*11]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1_rect2(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    m[0] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 3)); // pmulld m0, m14, [cq+128* 3]
    m[1] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 29)); // pmulld m1, m14, [cq+128*29]
    m[2] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 19)); // pmulld m2, m14, [cq+128*19]
    m[3] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * 13)); // pmulld m3, m14, [cq+128*13]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1_rect2(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1_rect2
    inv_txfm_add_dct_dct_16x64_10bpc_main_part2(m, r6, r5); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2
    for (i, k) in [2, 14, 18, 30].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 2] … [cq+128*30]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast_rect2(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast_rect2
    for (i, k) in [6, 10, 22, 26].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 6] … [cq+128*26]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast_rect2(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast_rect2
    for (i, k) in [4, 12, 20, 28].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 4] … [cq+128*28]
    }
    idct_8x16_internal_10bpc_main_oddhalf_fast_rect2(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_fast_rect2
    for (i, k) in [0, 8, 16, 24].into_iter().enumerate() {
        m[i] = _mm256_mullo_epi32(m[14], ldu_p8(c, 128 * k)); // pmulld m0, m14, [cq+128* 0] … [cq+128*24]
    }
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r7d: i32 = 128 * 29; // mov r7d, 128*29
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        stu_p8(c, r7 - 128 * 1, m[15]); // mova [cq+r7-128*1], m15
        stu_p8(c, r7 + 128 * 0, m[15]); // mova [cq+r7+128*0], m15
        stu_p8(c, r7 + 128 * 1, m[15]); // mova [cq+r7+128*1], m15
        stu_p8(c, r7 + 128 * 2, m[15]); // mova [cq+r7+128*2], m15
        r7d -= 128 * 4; // sub r7d, 128*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    m[15] = _mm256_srli_epi32::<11>(m[11]); // psrld m15, m11, 11 ; pd_1
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_add_epi32(m[x], m[11]); // REPX {paddd x, m11}, m0, m1, m2, m3
    }
    for x in [0, 1, 2, 3] {
        m[x] = _mm256_srai_epi32::<12>(m[x]); // REPX {psrad x, 12 }, m0, m1, m2, m3
    }
    inv_txfm_add_dct_dct_64x16_10bpc_main_end2(m, cq, r6, r5); // call m(inv_txfm_add_dct_dct_64x16_10bpc).main_end2
    idct64_shift_transpose::<1>(m, r6, r5, *r4); // IDCT64_SHIFT_TRANSPOSE 1
}

/// `cglobal inv_txfm_add_dct_dct_64x32_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x32_10bpc(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let mut r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 32; // or r3d, 32
        r6d = r6d.wrapping_add(128); // add r6d, 128
        r6d >>= 8; // sar r6d, 8
        r6d = r6d.wrapping_mul(181); // imul r6d, 181
        r6d = r6d.wrapping_add(384); // add r6d, 384
        r6d >>= 9; // sar r6d, 9
        return inv_txfm_add_dct_dct_64x16_10bpc_dconly2(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_64x16_10bpc).dconly2
    }
    // PROLOGUE 0, 8, 16, 32*163, dst, stride, c, eob. The frame is zeroed whole (see the top of this part).
    let mut frame = [_mm256_setzero_si256(); 163];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut cq = c;
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    let mut r6 = at(rsp, 32 * 7); // lea r6, [rsp+32*7]
    let (mut r4, mut r5) = (rsp, rsp);
    let fast = 'pass1: {
        inv_txfm_add_dct_dct_64x32_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 36 {
            break 'pass1 true; // cmp eobd, 36 / jl .fast
        }
        inv_txfm_add_dct_dct_64x32_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 136 {
            break 'pass1 true; // cmp eobd, 136 / jl .fast
        }
        inv_txfm_add_dct_dct_64x32_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 300 {
            break 'pass1 true; // cmp eobd, 300 / jl .fast
        }
        inv_txfm_add_dct_dct_64x32_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        false // jmp .pass2
    };
    if fast {
        // .fast:
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        r4 = at(rsp, 32 * 135); // lea r4, [rsp+32*135]
        loop {
            // .fast_loop:
            for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
                st_p8(r6, 32 * x, m[0]); // REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
            }
            r6 = at(r6, 32 * 8); // add r6, 32*8
            if r6 >= r4 {
                break; // cmp r6, r4 / jl .fast_loop
            }
        }
    }
    // .pass2:
    let mut r7 = at(r6, -32 * 32); // lea r7, [r6-32*32]
    r5 = at(r6, 32 * 8); // lea r5, [r6+32*8]
                         // lea r6, [pw_5+128] (the 8 bpc o_base)
    let mut r2 = at(dst, stride * 19); // imul r2, strideq, 19 / add r2, dstq
    let r3 = stride * 3; // lea r3, [strideq*3]
    loop {
        // .pass2_loop:
        for (i, x) in [-99, -97, -95, -93, -67, -65, -63, -61, -35, -33, -31, -29, -3, -1, 1, 3]
            .into_iter()
            .enumerate()
        {
            m[i] = ld_p8(r7, 32 * x); // mova m0, [r7-32*99] … mova m15, [r7+32* 3]
        }
        inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf(m, s3_p8(rsp), r4, r5); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf
        for (i, x) in [
            -100, -98, -96, -94, -68, -66, -64, -62, -36, -34, -32, -30, -4, -2, 0, 2,
        ]
        .into_iter()
        .enumerate()
        {
            m[i] = ld_p8(r7, 32 * x); // mova m0, [r7-32*100] … mova m15, [r7+32* 2]
        }
        r7 = at(r7, 32 * 8); // add r7, 32*8
        st_p8(rsp, 0, m[15]); // mova [rsp], m15
        idct_16x16_internal_8bpc_main(m, s3_p8(rsp)); // call m(idct_16x16_internal_8bpc).main
        inv_txfm_add_dct_dct_16x32_10bpc_pass2_end(
            m,
            s3_p8(rsp),
            &mut *at(rsp, 32 * 3),
            r5,
            &mut dst,
            stride,
            &mut r2,
            r3,
            r4,
        ); // call m(inv_txfm_add_dct_dct_16x32_10bpc).pass2_end
        dst = at(dst, -r3); // sub dstq, r3
        r2 = at(r2, r3 + 32); // lea r2, [r2+r3+32]
        dst = at(dst, 32); // add dstq, 32
        if r7 >= r4 {
            break; // cmp r7, r4 / jl .pass2_loop
        }
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_64x32_10bpc_avx2,
    inv_txfm_add_dct_dct_64x32_10bpc
);

// --- 64x64 ------------------------------------------------------------------------------------------------------

/// `.main` of `inv_txfm_add_dct_dct_64x64_10bpc`: as 64x16's `.main` for eight rows of 32 coefficients
/// (`[cq+128*0..31]`), ending in 64x16's `.main_end` and `.shift_transpose`.
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x64_10bpc_main(
    m: &mut M,
    cq: &mut Coef,
    r6: &mut *mut __m256i,
    r4: &mut *mut __m256i,
    r5: &mut *mut __m256i,
) {
    let c = *cq;
    let mut r5k = 0; // lea r5, [idct64_mul_16bpc]
    m[0] = ldu_p8(c, 128 * 1); // mova m0, [cq+128* 1]
    m[1] = ldu_p8(c, 128 * 31); // mova m1, [cq+128*31]
    m[2] = ldu_p8(c, 128 * 17); // mova m2, [cq+128*17]
    m[3] = ldu_p8(c, 128 * 15); // mova m3, [cq+128*15]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    m[0] = ldu_p8(c, 128 * 7); // mova m0, [cq+128* 7]
    m[1] = ldu_p8(c, 128 * 25); // mova m1, [cq+128*25]
    m[2] = ldu_p8(c, 128 * 23); // mova m2, [cq+128*23]
    m[3] = ldu_p8(c, 128 * 9); // mova m3, [cq+128* 9]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    m[0] = ldu_p8(c, 128 * 5); // mova m0, [cq+128* 5]
    m[1] = ldu_p8(c, 128 * 27); // mova m1, [cq+128*27]
    m[2] = ldu_p8(c, 128 * 21); // mova m2, [cq+128*21]
    m[3] = ldu_p8(c, 128 * 11); // mova m3, [cq+128*11]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    m[0] = ldu_p8(c, 128 * 3); // mova m0, [cq+128* 3]
    m[1] = ldu_p8(c, 128 * 29); // mova m1, [cq+128*29]
    m[2] = ldu_p8(c, 128 * 19); // mova m2, [cq+128*19]
    m[3] = ldu_p8(c, 128 * 13); // mova m3, [cq+128*13]
    inv_txfm_add_dct_dct_16x64_10bpc_main_part1(m, &mut r5k, r6); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part1
    inv_txfm_add_dct_dct_16x64_10bpc_main_part2(m, r6, r5); // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2
    for (i, k) in [2, 14, 18, 30].into_iter().enumerate() {
        m[i] = ldu_p8(c, 128 * k); // mova m0, [cq+128* 2] … mova m3, [cq+128*30]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part1_fast(m, *r6); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part1_fast
    for (i, k) in [6, 10, 22, 26].into_iter().enumerate() {
        m[i] = ldu_p8(c, 128 * k); // mova m0, [cq+128* 6] … mova m3, [cq+128*26]
    }
    inv_txfm_add_dct_dct_8x32_10bpc_main_oddhalf_part2_fast(m, r6, r4, r5); // call m(inv_txfm_add_dct_dct_8x32_10bpc).main_oddhalf_part2_fast
    for (i, k) in [4, 12, 20, 28].into_iter().enumerate() {
        m[i] = ldu_p8(c, 128 * k); // mova m0, [cq+128* 4] … mova m3, [cq+128*28]
    }
    idct_8x16_internal_10bpc_main_oddhalf_fast(m, *r6); // call m(idct_8x16_internal_10bpc).main_oddhalf_fast
    for (i, k) in [0, 8, 16, 24].into_iter().enumerate() {
        m[i] = ldu_p8(c, 128 * k); // mova m0, [cq+128* 0] … mova m3, [cq+128*24]
    }
    m[15] = _mm256_setzero_si256(); // pxor m15, m15
    let mut r7d: i32 = 128 * 29; // mov r7d, 128*29
    loop {
        // .main_zero_loop:
        let r7 = r7d as isize;
        stu_p8(c, r7 - 128 * 1, m[15]); // mova [cq+r7-128*1], m15
        stu_p8(c, r7 + 128 * 0, m[15]); // mova [cq+r7+128*0], m15
        stu_p8(c, r7 + 128 * 1, m[15]); // mova [cq+r7+128*1], m15
        stu_p8(c, r7 + 128 * 2, m[15]); // mova [cq+r7+128*2], m15
        r7d -= 128 * 4; // sub r7d, 128*4
        if r7d <= 0 {
            break; // jg .main_zero_loop
        }
    }
    inv_txfm_add_dct_dct_64x16_10bpc_main_end(m, cq, r6, r5); // call m(inv_txfm_add_dct_dct_64x16_10bpc).main_end
    inv_txfm_add_dct_dct_64x16_10bpc_shift_transpose(m, r6, r5, *r4); // jmp m(inv_txfm_add_dct_dct_64x16_10bpc).shift_transpose
}

/// `cglobal inv_txfm_add_dct_dct_64x64_10bpc, 4, 7, 0, dst, stride, c, eob`
#[inline(always)]
unsafe fn inv_txfm_add_dct_dct_64x64_10bpc(mut dst: Dst, stride: isize, c: Coef, eob: c_int) {
    let mut m = regs();
    let m = &mut m;
    if eob == 0 {
        // test eobd, eobd / jz .dconly
        // .dconly:
        let r6d = c.read_unaligned().wrapping_mul(181); // imul r6d, [cq], 181
        c.write_unaligned(eob); // mov [cq], eobd ; 0
        let r3d = eob | 64; // or r3d, 64
        return inv_txfm_add_dct_dct_64x16_10bpc_dconly(m, dst, stride, r6d, r3d);
        // jmp m(inv_txfm_add_dct_dct_64x16_10bpc).dconly
    }
    // PROLOGUE 0, 11, 16, 32*195, dst, stride, c, eob. The frame is zeroed whole (see the top of this part).
    let mut frame = [_mm256_setzero_si256(); 195];
    let rsp: *mut __m256i = frame.as_mut_ptr();
    let mut cq = c;
    m[11] = pd(2048); // vpbroadcastd m11, [pd_2048]
    m[12] = pd(-0x20000); // vpbroadcastd m12, [clip_18b_min]
    m[13] = pd(0x1ffff); // vpbroadcastd m13, [clip_18b_max]
    m[14] = pd(2896); // vpbroadcastd m14, [pd_2896]
    let mut r6 = at(rsp, 32 * 7); // lea r6, [rsp+32*7]
    let (mut r4, mut r5) = (rsp, rsp);
    let fast = 'pass1: {
        inv_txfm_add_dct_dct_64x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 36 {
            break 'pass1 true; // cmp eobd, 36 / jl .fast
        }
        inv_txfm_add_dct_dct_64x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 136 {
            break 'pass1 true; // cmp eobd, 136 / jl .fast
        }
        inv_txfm_add_dct_dct_64x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        if eob < 300 {
            break 'pass1 true; // cmp eobd, 300 / jl .fast
        }
        inv_txfm_add_dct_dct_64x64_10bpc_main(m, &mut cq, &mut r6, &mut r4, &mut r5); // call .main
        false // jmp .pass2
    };
    if fast {
        // .fast:
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        r4 = at(rsp, 32 * 135); // lea r4, [rsp+32*135]
        loop {
            // .fast_loop:
            for x in [-4, -3, -2, -1, 0, 1, 2, 3] {
                st_p8(r6, 32 * x, m[0]); // REPX {mova [r6+32*x], m0}, -4, -3, -2, -1, 0, 1, 2, 3
            }
            r6 = at(r6, 32 * 8); // add r6, 32*8
            if r6 >= r4 {
                break; // cmp r6, r4 / jl .fast_loop
            }
        }
    }
    // .pass2:
    let mut r10 = at(r6, -32 * 32); // lea r10, [r6-32*32]
                                    // lea r6, [pw_5+128] (the 8 bpc o_base)
    let r8 = stride * 4; // lea r8, [strideq*4]
    let r9 = stride * 5; // lea r9, [strideq*5]
    let r3 = r9 + stride; // lea r3, [r9+strideq*1] ; stride*6
    let r7 = r9 + stride * 2; // lea r7, [r9+strideq*2] ; stride*7
    loop {
        // .pass2_loop:
        m[0] = ld_p8(r10, -32 * 100); // mova m0, [r10-32*100] ; in0
        m[1] = ld_p8(r10, -32 * 96); // mova m1, [r10-32*96]  ; in4
        m[2] = ld_p8(r10, -32 * 68); // mova m2, [r10-32*68]  ; in8
        m[3] = ld_p8(r10, -32 * 64); // mova m3, [r10-32*64]  ; in12
        m[4] = ld_p8(r10, -32 * 36); // mova m4, [r10-32*36]  ; in16
        m[5] = ld_p8(r10, -32 * 32); // mova m5, [r10-32*32]  ; in20
        m[6] = ld_p8(r10, -32 * 4); // mova m6, [r10-32* 4]  ; in24
        m[7] = ld_p8(r10, 32 * 0); // mova m7, [r10+32* 0]  ; in28
        m[8] = _mm256_setzero_si256(); // pxor m8, m8
        for x in [9, 10, 11, 12, 13, 14] {
            m[x] = m[8]; // REPX {mova x, m8}, m9, m10, m11, m12, m13, m14
        }
        st_p8(rsp, 0, m[8]); // mova [rsp], m8
        idct_16x16_internal_8bpc_main(m, s3_p8(rsp)); // call m(idct_16x16_internal_8bpc).main
        m[1] = ld_p8(rsp, 32 * 1); // mova m1, [rsp+32*1]
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            st_p8(r4, 32 * x, m[i]); // mova [r4-32*4], m0 … mova [r4+32*3], m7
        }
        r4 = at(r4, 32 * 8); // add r4, 32*8
        for (i, x) in [-4, -3, -2, -1, 0, 1, 2, 3].into_iter().enumerate() {
            st_p8(r4, 32 * x, m[8 + i]); // mova [r4-32*4], m8 … mova [r4+32*3], m15
        }
        m[0] = ld_p8(r10, -32 * 98); // mova m0, [r10-32*98] ; in2
        m[1] = ld_p8(r10, -32 * 94); // mova m1, [r10-32*94] ; in6
        m[2] = ld_p8(r10, -32 * 66); // mova m2, [r10-32*66] ; in10
        m[3] = ld_p8(r10, -32 * 62); // mova m3, [r10-32*62] ; in14
        m[4] = ld_p8(r10, -32 * 34); // mova m4, [r10-32*34] ; in18
        m[5] = ld_p8(r10, -32 * 30); // mova m5, [r10-32*30] ; in22
        m[6] = ld_p8(r10, -32 * 2); // mova m6, [r10-32* 2] ; in26
        m[7] = ld_p8(r10, 32 * 2); // mova m7, [r10+32* 2] ; in30
        r5 = at(r4, 32 * 16); // lea r5, [r4+32*16]
        r4 = at(r4, 32 * 8); // add r4, 32*8
        inv_txfm_add_dct_dct_16x32_8bpc_main_oddhalf_fast(m, s3_p8(rsp), r4, r5); // call m(inv_txfm_add_dct_dct_16x32_8bpc).main_oddhalf_fast
        m[0] = ld_p8(r10, -32 * 99); // mova m0, [r10-32*99] ; in1
        m[1] = ld_p8(r10, 32 * 3); // mova m1, [r10+32* 3] ; in31
        m[2] = ld_p8(r10, -32 * 35); // mova m2, [r10-32*35] ; in17
        m[3] = ld_p8(r10, -32 * 61); // mova m3, [r10-32*61] ; in15
        m[4] = ld_p8(r10, -32 * 67); // mova m4, [r10-32*67] ; in9
        m[5] = ld_p8(r10, -32 * 29); // mova m5, [r10-32*29] ; in23
        m[6] = ld_p8(r10, -32 * 3); // mova m6, [r10-32* 3] ; in25
        m[7] = ld_p8(r10, -32 * 93); // mova m7, [r10-32*93] ; in7
                                     // lea r6, [idct64_mul - 8]
        r4 = at(r4, 32 * 16); // add r4, 32*16
        r5 = at(r5, 32 * 32); // add r5, 32*32
        inv_txfm_add_dct_dct_16x64_8bpc_main_part1(m, 0, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        m[0] = ld_p8(r10, -32 * 95); // mova m0, [r10-32*95] ; in5
        m[1] = ld_p8(r10, -32 * 1); // mova m1, [r10-32* 1] ; in27
        m[2] = ld_p8(r10, -32 * 31); // mova m2, [r10-32*31] ; in21
        m[3] = ld_p8(r10, -32 * 65); // mova m3, [r10-32*65] ; in11
        m[4] = ld_p8(r10, -32 * 63); // mova m4, [r10-32*63] ; in13
        m[5] = ld_p8(r10, -32 * 33); // mova m5, [r10-32*33] ; in19
        m[6] = ld_p8(r10, 32 * 1); // mova m6, [r10+32* 1] ; in29
        m[7] = ld_p8(r10, -32 * 97); // mova m7, [r10-32*97] ; in3
                                     // add r6, 8
        r4 = at(r4, 32 * 8); // add r4, 32*8
        r5 = at(r5, -32 * 8); // sub r5, 32*8
        inv_txfm_add_dct_dct_16x64_8bpc_main_part1(m, 2, r4, r5); // call m(inv_txfm_add_dct_dct_16x64_8bpc).main_part1
        inv_txfm_add_dct_dct_16x64_10bpc_main_part2_pass2(m, &mut dst, r7, stride, r3, r8, r9, &mut r4, &mut r5);
        // call m(inv_txfm_add_dct_dct_16x64_10bpc).main_part2_pass2
        r10 = at(r10, 32 * 8); // add r10, 32*8
        dst = at(dst, -r8); // sub dstq, r8
        r4 = at(r4, -32 * 44); // sub r4, 32*44
        dst = at(dst, 32); // add dstq, 32
        if r10 >= r4 {
            break; // cmp r10, r4 / jl .pass2_loop
        }
    }
}

inv_txfm_fn!(
    dav1d_inv_txfm_add_dct_dct_64x64_10bpc_avx2,
    inv_txfm_add_dct_dct_64x64_10bpc
);
