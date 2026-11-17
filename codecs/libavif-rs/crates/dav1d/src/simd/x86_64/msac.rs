// Copyright © 2019, VideoLAN and dav1d authors
// Copyright © 2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/msac.asm` with SSE2 and AVX2
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062), in its x86-64
//! System V form (`ARCH_X86_64`, `UNIX64`).
//!
//! Seven of the assembly's eight entry points of the entropy decoder: the
//! SSE2 functions `symbol_adapt4/8`, `bool_adapt`, `bool_equi`, `bool` and
//! `hi_tok`, and the AVX2 `symbol_adapt16`. These are the functions the
//! original runs on an AVX2 machine. The eighth, the SSE2
//! `symbol_adapt16`, is not rebuilt: the original selects it only on a CPU
//! without AVX2, which is outside the scope (engine ADR-0041 section 4;
//! ADR 0062, "Levels"). On such a CPU the port's 16-symbol function is the
//! translated C function (`simd/x86/msac.rs`).
//!
//! Correspondence with the assembly:
//! - Each `cglobal` function is one `extern "C"` function of the same name
//!   with its instruction-set suffix. The shared tail of
//!   `msac_decode_symbol_adapt4` that the other functions jump into
//!   (`.renorm2`, `.renorm3`, `.renorm4`, `.renorm5`, with the refill behind
//!   them) is the chain of functions `renorm2` … `renorm5` and `refill`;
//!   the macro `HI_TOK update_cdf` is `hi_tok::<UPDATE_CDF>`, whose copy of
//!   the refill is the same `refill`.
//! - The vector registers are locals `m0` … `m5`, the general-purpose
//!   registers locals named as in the assembly (`t0` … `t8` of
//!   `DECLARE_REG_TMP`, `eax`, `ecx`, `rcx`): a `d` register is a `u32`, a
//!   full register a `u64`, and a shift by `cl` uses the low 5 (32-bit) or
//!   6 (64-bit) bits of the count, as the hardware does. An instruction
//!   that only sets the carry flag for a following `sbb`/`adc`/`setb`/
//!   `cmovb`/`jae`/`jnc` is the same unsigned comparison or the carry of
//!   the same addition written out.
//! - The red-zone buffer (`buf`, `rsp-40`) that holds `rng` and the `v`
//!   values for the indexed load after `tzcnt` is a local array of
//!   halfwords: `buf[0..2]` is the doubleword the assembly stores at
//!   `buf+12` (`buf-4` in the 16-symbol function), the `v` values follow.
//! - The tables `min_prob`, `pw_0xff00` and `pw_32` are one array, as the
//!   assembly addresses `min_prob` with a negative offset from `pw_0xff00`
//!   and reads past its end into `pw_0xff00` and `pw_32` for the lanes
//!   behind the last symbol.
//!
//! `bsr` and `tzcnt` have no defined result for an operand of 0. The
//! assembly relies on `rng` never being 0 (after `dav1d_msac_init` and after
//! every renormalization `0x8000 <= rng <= 0xffff`, and `u - v`, the next
//! `rng` before the shift, is at least `EC_MIN_PROB`), and on the comparison
//! mask always having the bit of lane `n_symbols` (its `v` is 0, and the
//! lanes behind the vector's symbols compare equal as well). The rebuild
//! states both as `NonZero` values, which compile to the bare `bsr` and
//! `bsf`/`tzcnt`.
//!
//! Deviations from the assembly:
//! - For memory safety: near the end of the buffer (`.refill_eob`) the
//!   assembly loads the 8 bytes that end at `buf_end` and shifts out the
//!   ones before `buf_pos`; when fewer than 8 bytes precede `buf_end` in
//!   the buffer that load reaches before it. Here the bytes
//!   `buf_pos..buf_end` are read to the low end of a zeroed 8-byte word,
//!   which is the value the load and the shift produce (as in
//!   `simd/arm64/msac.rs`).
//! - The assembly loads and stores the CDF with aligned moves (`mova`) in
//!   the 8- and 16-symbol functions; here every vector access to caller
//!   memory is an unaligned move of the same bytes, so a CDF that is not
//!   aligned as the original's are is not undefined behavior. Same number
//!   of instructions.
//! - In the SSE2 8-symbol function the compiler, which knows the constant
//!   `pw_0xff00`, turns `punpcklqdq` and `pand m, [pw_0xff00]` on the
//!   replicated `rng` into a shuffle with zero bytes: nine SSE2
//!   instructions (`punpcklbw`, four `pshuf*w`, `pshufd`, `packuswb`,
//!   `pxor`, `punpcklbw`) where the assembly has two. Same values; the
//!   function is seven instructions per call longer for it. A volatile
//!   load of the constant kept the assembly's memory operand and was
//!   removed again (ADR 0062, 2026-11-17): a volatile read to steer the
//!   compiler is not worth carrying for seven instructions.
//! - The stack buffers are zero-initialized arrays. Every element a kernel
//!   loads is stored first in the same block, a few lines above the load,
//!   so the compiler drops the zero stores.
//!
//! Safety (argued once for the family): the callers are the functions of
//! `simd/x86/msac.rs`, called where the original calls the assembly through
//! the macros of `src/x86/msac.h`. They pass what the original passes: `s`
//! points to a live `MsacContext` initialized by `dav1d_msac_init` (layout
//! checked by a test), `buf_pos..buf_end` is readable, and `cdf` points to
//! the CDF array of the symbol, writable and of at least 4 (`adapt4`,
//! `hi_tok`), 8 (`adapt8`) or 16 (`adapt16`) halfwords, as dav1d's
//! `CdfContext` arrays are, with `n_symbols` in the range of the entry
//! point (1–3, 1–7, 3–15); `bool_adapt` reads and writes `cdf[0..2]`. The
//! loads and stores below stay inside the context, those CDF halfwords,
//! `buf_pos..buf_end`, the table and the local buffer.
#![allow(clippy::all, non_snake_case)]

use core::arch::x86_64::*;
use core::ffi::{c_int, c_uint};
use core::num::NonZeroU32;

/// `struc msac` of msac.asm, which is `MsacContext` (src/msac.h) without
/// the `symbol_adapt16` pointer the original adds on x86-64 with assembly
/// (the port keeps the portable layout; see `simd/x86/msac.rs`): `.buf` 0,
/// `.end` 8, `.dif` 16, `.rng` 24, `.cnt` 28, `.update_cdf` 32.
#[repr(C)]
pub struct MsacContext {
    pub buf: *const u8,
    pub end: *const u8,
    pub dif: usize,
    pub rng: c_uint,
    pub cnt: c_int,
    pub update_cdf: c_int,
}

/// `SECTION_RODATA 64`: `min_prob`, `pw_0xff00`, `pw_32`, contiguous.
#[repr(C, align(64))]
struct Rodata([u16; 32]);
static RODATA: Rodata = Rodata([
    // min_prob
    60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0, //
    // pw_0xff00
    0xff00, 0xff00, 0xff00, 0xff00, 0xff00, 0xff00, 0xff00, 0xff00, //
    // pw_32
    32, 32, 32, 32, 32, 32, 32, 32,
]);
/// Halfword offsets of the three tables.
const MIN_PROB: usize = 0;
const PW_0XFF00: usize = 16;
const PW_32: usize = 24;

/// `pshuflw m, m, q3333` (`c_shuf` on x86-64) and `q0000`.
const Q3333: i32 = 0b11_11_11_11;
const Q0000: i32 = 0;

/// `tzcnt eax, eax` on a comparison mask (never 0, see the module
/// documentation).
#[inline(always)]
unsafe fn tzcnt(eax: u32) -> u32 {
    debug_assert!(eax != 0);
    NonZeroU32::new_unchecked(eax).trailing_zeros()
}

/// `bsr ecx, t2d ; xor ecx, 15` (`d`, the number of bits to shift `rng`
/// up by; `rng` is never 0, see the module documentation).
#[inline(always)]
unsafe fn bsr_xor_15(t2d: u32) -> u32 {
    debug_assert!(t2d != 0);
    // bsr = 31 - lzcnt, and (31 - x) ^ 15 == x ^ 16 for x in 0..=31.
    NonZeroU32::new_unchecked(t2d).leading_zeros() ^ 16
}

// --- the tail of msac_decode_symbol_adapt4 -----------------------------------

/// `.renorm2`: `eax` the return value, `t1` = `v`, `t2d` = `u`, `t4` = `dif`.
#[inline(always)]
unsafe fn renorm2(t0: *mut MsacContext, eax: u32, t1: u64, t2d: u32, t4: u64) -> c_uint {
    let t2d = t2d.wrapping_sub(t1 as u32); // sub t2d, t1d ; rng
    let t1 = t1 << 48; // shl t1, gprsize*8-16
    let t4 = t4.wrapping_sub(t1); // sub t4, t1 ; dif - v
    renorm3(t0, eax, t2d, t4)
}

/// `.renorm3`: `t2d` = `rng`, `t4` = `dif`.
#[inline(always)]
unsafe fn renorm3(t0: *mut MsacContext, eax: u32, t2d: u32, t4: u64) -> c_uint {
    let t1d = (*t0).cnt as u32; // mov t1d, [t0+msac.cnt]
    let t7 = t0; // movifnidn t7, t0
                 // .renorm4:
    let ecx = bsr_xor_15(t2d); // bsr ecx, t2d ; xor ecx, 15 ; d
    renorm5(t7, eax, t1d, t2d, t4, ecx)
}

/// `.renorm5`: `t1d` = `cnt`, `ecx` = `d`.
#[inline(always)]
unsafe fn renorm5(t7: *mut MsacContext, eax: u32, t1d: u32, t2d: u32, t4: u64, ecx: u32) -> c_uint {
    let t2d = t2d << (ecx & 31); // shl t2d, cl
    let mut t4 = t4 << (ecx & 63); // shl t4, cl
    (*t7).rng = t2d; // mov [t7+msac.rng], t2d
    let (mut t1d, below) = t1d.overflowing_sub(ecx); // sub t1d, ecx
    if below {
        // jae .end ; no refill required
        (t1d, t4) = refill(t7, t1d, t4);
    }
    // .end:
    (*t7).cnt = t1d as c_int; // mov [t7+msac.cnt], t1d
    (*t7).dif = t4 as usize; // mov [t7+msac.dif], t4
    eax // RET
}

/// The refill behind `.renorm5` (`refill:`, `.refill_end`, `.refill_end2`,
/// `.pad_with_ones`, `.refill_eob`) and its copy in `HI_TOK`: reads the next
/// bytes into `dif` below its valid bits and returns the new `cnt` and `dif`
/// (`t1d`, `t4`; `t5d`, `t4` in `HI_TOK`); `buf` is stored here.
#[inline(always)]
unsafe fn refill(t7: *mut MsacContext, t1d: u32, mut t4: u64) -> (u32, u64) {
    let t2 = (*t7).buf; // mov t2, [t7+msac.buf]
    let t5 = (*t7).end; // mov t5, [t7+msac.end]
    let rcx: u64;
    // lea rcx, [t2+gprsize] ; sub rcx, t5 ; ja .refill_eob
    if (t2 as u64).wrapping_add(8) > t5 as u64 {
        // .refill_eob: ; avoid overreading the input buffer
        // cmp t2, t5 ; jae .pad_with_ones ; eob reached
        if t2 as u64 >= t5 as u64 {
            // .pad_with_ones:
            let ecx = t1d.wrapping_sub(16); // lea ecx, [t1-16]
            let rcx = (ecx as u64).rotate_right(ecx & 63); // ror rcx, cl
            t4 |= rcx; // or t4, rcx
            return (t1d, t4); // jmp .refill_end2
        }
        // mov t5, [t5-gprsize] ; shl ecx, 3 ; shr t5, cl: the bytes
        // buf_pos..buf_end at the low end of the word (see the module
        // documentation), 1 to 7 of them, read as 4 + 2 + 1.
        let left = t5.offset_from(t2) as usize;
        let mut t5q = 0u64;
        let mut i = 0;
        if left & 4 != 0 {
            t5q = core::ptr::read_unaligned(t2.cast::<u32>()) as u64;
            i = 4;
        }
        if left & 2 != 0 {
            t5q |= (core::ptr::read_unaligned(t2.add(i).cast::<u16>()) as u64) << (8 * i);
            i += 2;
        }
        if left & 1 != 0 {
            t5q |= (core::ptr::read(t2.add(i)) as u64) << (8 * i);
        }
        let ecx = t1d.wrapping_add(16).wrapping_sub(64); // lea ecx, [t1+16-gprsize*8]
        t5q = !t5q; // not t5
        t5q = t5q.swap_bytes(); // bswap t5
        t5q >>= ecx & 63; // shr t5, cl
        let ecx = ecx.wrapping_neg(); // neg ecx
        t4 |= t5q; // or t4, t5
        let t5d = (t5 as u64 as u32).wrapping_sub(t2 as u64 as u32); // mov t5d, [t7+msac.end] ; sub t5d, t2d ; num_bytes_left
        let ecx = ecx >> 3; // shr ecx, 3
        rcx = (if ecx >= t5d { t5d } else { ecx }) as u64; // cmp ecx, t5d ; cmovae ecx, t5d ; num_bytes_read
                                                           // jmp .refill_end
    } else {
        let mut t5q = core::ptr::read_unaligned(t2.cast::<u64>()); // mov t5, [t2]
        let ecx = t1d.wrapping_add(16).wrapping_sub(64); // lea ecx, [t1+16-gprsize*8]
        t5q = !t5q; // not t5
        t5q = t5q.swap_bytes(); // bswap t5
        t5q >>= ecx & 63; // shr t5, cl
        let ecx = ecx.wrapping_neg(); // neg ecx
        rcx = (ecx >> 3) as u64; // shr ecx, 3 ; num_bytes_read
        t4 |= t5q; // or t4, t5
    }
    // .refill_end:
    let t2 = t2.add(rcx as usize); // add t2, rcx
    let t1d = t1d.wrapping_add((rcx as u32) << 3); // lea t1d, [t1+rcx*8] ; cnt += num_bits_read
    (*t7).buf = t2; // mov [t7+msac.buf], t2
                    // .refill_end2:
    (t1d, t4)
}

// --- INIT_XMM sse2 -----------------------------------------------------------

/// `cglobal msac_decode_symbol_adapt4` (SSE2):
/// `unsigned dav1d_msac_decode_symbol_adapt4_sse2(MsacContext *s, uint16_t *cdf, size_t n_symbols)`
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt4_sse2(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
) -> c_uint {
    let (t0, t1, mut t2) = (s, cdf, n_symbols as u64);
    // The doubleword at buf+12 and the 16 bytes at buf+16.
    let mut buf = [0u16; 10];
    let rax = RODATA.0.as_ptr().add(PW_0XFF00); // LEA rax, pw_0xff00
    let mut m2 = _mm_cvtsi32_si128((*t0).rng as i32); // movd m2, [t0+msac.rng]
    let mut m1 = _mm_loadl_epi64(t1.cast()); // movq m1, [t1]
    let mut m3 = _mm_cvtsi64_si128((*t0).dif as i64); // movp m3, [t0+msac.dif]
    let mut t3d = (*t0).update_cdf as u32; // mov t3d, [t0+msac.update_cdf]
    let t4d = t2 as u32; // mov t4d, t2d
    t2 = !t2; // not t2 ; -(n_symbols + 1)
    m2 = _mm_shufflelo_epi16::<Q0000>(m2); // pshuflw m2, m2, q0000
    _mm_storeu_si32(buf.as_mut_ptr().cast(), m2); // movd [buf+12], m2
    m2 = _mm_and_si128(m2, _mm_loadu_si128(rax.cast())); // pand m2, [rax]
    let mut m0 = m1; // mova m0, m1
    m1 = _mm_srli_epi16::<6>(m1); // psrlw m1, 6
    m1 = _mm_slli_epi16::<7>(m1); // psllw m1, 7
    m1 = _mm_mulhi_epu16(m1, m2); // pmulhuw m1, m2
    m2 = _mm_loadl_epi64(rax.offset(t2 as i64 as isize).cast()); // movq m2, [rax+t2*2]
    m3 = _mm_shufflelo_epi16::<Q3333>(m3); // pshuflw m3, m3, c_shuf
    m1 = _mm_add_epi16(m1, m2); // paddw m1, m2
    _mm_storeu_si128(buf.as_mut_ptr().add(2).cast(), m1); // mova [buf+16], m1
    m1 = _mm_subs_epu16(m1, m3); // psubusw m1, m3
    m2 = _mm_setzero_si128(); // pxor m2, m2
    m1 = _mm_cmpeq_epi16(m1, m2); // pcmpeqw m1, m2 ; c >= v
    let eax = _mm_movemask_epi8(m1) as u32; // pmovmskb eax, m1
    if t3d != 0 {
        // test t3d, t3d ; jz .renorm ; !allow_update_cdf
        // update_cdf:
        t3d = core::ptr::read(t1.add(t4d as usize)) as u32; // movzx t3d, word [t1+t4*2] ; count
        m2 = _mm_cmpeq_epi16(m2, m2); // pcmpeqw m2, m2
        let mut t2d = t3d; // mov t2d, t3d
        t3d >>= 4; // shr t3d, 4
        t3d = t3d.wrapping_add(5).wrapping_sub((t4d < 3) as u32); // cmp t4d, 3 ; sbb t3d, -5 ; (count >> 4) + (n_symbols > 2) + 4
        t2d += (t2d < 32) as u32; // cmp t2d, 32 ; adc t2d, 0 ; count + (count < 32)
        m3 = _mm_cvtsi32_si128(t3d as i32); // movd m3, t3d
        m2 = _mm_avg_epu16(m2, m1); // pavgw m2, m1 ; i >= val ? -1 : 32768
        m2 = _mm_sub_epi16(m2, m0); // psubw m2, m0 ; for (i = 0; i < val; i++)
        m0 = _mm_sub_epi16(m0, m1); // psubw m0, m1 ;     cdf[i] += (32768 - cdf[i]) >> rate;
        m2 = _mm_sra_epi16(m2, m3); // psraw m2, m3 ; for (; i < n_symbols; i++)
        m0 = _mm_add_epi16(m0, m2); // paddw m0, m2 ;     cdf[i] += ((  -1 - cdf[i]) >> rate) + 1;
        _mm_storel_epi64(t1.cast(), m0); // movq [t1], m0
        core::ptr::write(t1.add(t4d as usize), t2d as u16); // mov [t1+t4*2], t2w
    }
    renorm(t0, &buf, eax)
}

/// `.renorm` of `msac_decode_symbol_adapt4`, which `symbol_adapt8` shares:
/// `buf[0..2]` is the doubleword at `buf+12`, `buf[2..]` the `v` values at
/// `buf+16`; `eax` is the byte mask of `c >= v`.
#[inline(always)]
unsafe fn renorm(t0: *mut MsacContext, buf: &[u16; 10], eax: u32) -> c_uint {
    let eax = tzcnt(eax); // tzcnt eax, eax
    let t4 = (*t0).dif as u64; // mov t4, [t0+msac.dif]
    let p = buf.as_ptr().cast::<u8>().add(eax as usize);
    let t1 = core::ptr::read_unaligned(p.add(4).cast::<u16>()) as u64; // movzx t1d, word [buf+rax+16] ; v
    let t2d = core::ptr::read_unaligned(p.add(2).cast::<u16>()) as u32; // movzx t2d, word [buf+rax+14] ; u
    let eax = eax >> 1; // shr eax, 1
    renorm2(t0, eax, t1, t2d, t4)
}

/// `cglobal msac_decode_symbol_adapt8` (SSE2):
/// `unsigned dav1d_msac_decode_symbol_adapt8_sse2(MsacContext *s, uint16_t *cdf, size_t n_symbols)`
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt8_sse2(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
) -> c_uint {
    let (t0, t1, mut t2) = (s, cdf, n_symbols as u64);
    let mut buf = [0u16; 10];
    let rax = RODATA.0.as_ptr().add(PW_0XFF00); // LEA rax, pw_0xff00
    let mut m2 = _mm_cvtsi32_si128((*t0).rng as i32); // movd m2, [t0+msac.rng]
    let mut m1 = _mm_loadu_si128(t1.cast()); // mova m1, [t1]
    let mut m3 = _mm_cvtsi64_si128((*t0).dif as i64); // movp m3, [t0+msac.dif]
    let mut t3d = (*t0).update_cdf as u32; // mov t3d, [t0+msac.update_cdf]
    let t4d = t2 as u32; // mov t4d, t2d
    t2 = !t2; // not t2
    m2 = _mm_shufflelo_epi16::<Q0000>(m2); // pshuflw m2, m2, q0000
    _mm_storeu_si32(buf.as_mut_ptr().cast(), m2); // movd [buf+12], m2
    m2 = _mm_unpacklo_epi64(m2, m2); // punpcklqdq m2, m2
    let mut m0 = m1; // mova m0, m1
    m1 = _mm_srli_epi16::<6>(m1); // psrlw m1, 6
    m2 = _mm_and_si128(m2, _mm_loadu_si128(rax.cast())); // pand m2, [rax]
    m1 = _mm_slli_epi16::<7>(m1); // psllw m1, 7
    m1 = _mm_mulhi_epu16(m1, m2); // pmulhuw m1, m2
    m2 = _mm_loadu_si128(rax.offset(t2 as i64 as isize).cast()); // movu m2, [rax+t2*2]
    m3 = _mm_shufflelo_epi16::<Q3333>(m3); // pshuflw m3, m3, c_shuf
    m1 = _mm_add_epi16(m1, m2); // paddw m1, m2
    m3 = _mm_unpacklo_epi64(m3, m3); // punpcklqdq m3, m3
    _mm_storeu_si128(buf.as_mut_ptr().add(2).cast(), m1); // mova [buf+16], m1
    m1 = _mm_subs_epu16(m1, m3); // psubusw m1, m3
    m2 = _mm_setzero_si128(); // pxor m2, m2
    m1 = _mm_cmpeq_epi16(m1, m2); // pcmpeqw m1, m2
    let eax = _mm_movemask_epi8(m1) as u32; // pmovmskb eax, m1
    if t3d != 0 {
        // test t3d, t3d ; jz m(msac_decode_symbol_adapt4, SUFFIX).renorm
        t3d = core::ptr::read(t1.add(t4d as usize)) as u32; // movzx t3d, word [t1+t4*2]
        m2 = _mm_cmpeq_epi16(m2, m2); // pcmpeqw m2, m2
        let mut t2d = t3d; // mov t2d, t3d
        t3d >>= 4; // shr t3d, 4
        t3d = t3d.wrapping_add(5).wrapping_sub((t4d < 3) as u32); // cmp t4d, 3 ; sbb t3d, -5 ; may be called with n_symbols <= 2
        t2d += (t2d < 32) as u32; // cmp t2d, 32 ; adc t2d, 0
        m3 = _mm_cvtsi32_si128(t3d as i32); // movd m3, t3d
        m2 = _mm_avg_epu16(m2, m1); // pavgw m2, m1
        m2 = _mm_sub_epi16(m2, m0); // psubw m2, m0
        m0 = _mm_sub_epi16(m0, m1); // psubw m0, m1
        m2 = _mm_sra_epi16(m2, m3); // psraw m2, m3
        m0 = _mm_add_epi16(m0, m2); // paddw m0, m2
        _mm_storeu_si128(t1.cast(), m0); // mova [t1], m0
        core::ptr::write(t1.add(t4d as usize), t2d as u16); // mov [t1+t4*2], t2w
    }
    renorm(t0, &buf, eax) // jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm
}

/// `cglobal msac_decode_bool_adapt` (SSE2 build; general-purpose code):
/// `unsigned dav1d_msac_decode_bool_adapt_sse2(MsacContext *s, uint16_t *cdf)`
pub unsafe extern "C" fn dav1d_msac_decode_bool_adapt_sse2(s: *mut MsacContext, cdf: *mut u16) -> c_uint {
    let (t0, t1) = (s, cdf);
    let mut eax = core::ptr::read(t1) as u32; // movzx eax, word [t1]
    let mut t3d = ((*t0).rng >> 8) & 0xff; // movzx t3d, byte [t0+msac.rng+1]
    let mut t4 = (*t0).dif as u64; // mov t4, [t0+msac.dif]
    let mut t2d = (*t0).rng; // mov t2d, [t0+msac.rng]
    let mut t5d = eax; // mov t5d, eax
    eax &= !63; // and eax, ~63
    eax = eax.wrapping_mul(t3d); // imul eax, t3d
    let t6 = t4; // mov t6, t4
    eax >>= 7; // shr eax, 7
    eax += 4; // add eax, 4 ; v
    t3d = eax; // mov t3d, eax
    let rax = (eax as u64) << 48; // shl rax, gprsize*8-16 ; vw
    t2d = t2d.wrapping_sub(t3d); // sub t2d, t3d ; r - v
    let below;
    (t4, below) = t4.overflowing_sub(rax); // sub t4, rax ; dif - vw
    let eax = below as u32; // setb al
    if below {
        t2d = t3d; // cmovb t2d, t3d
    }
    t3d = (*t0).update_cdf as u32; // mov t3d, [t0+msac.update_cdf]
    if below {
        t4 = t6; // cmovb t4, t6
    }
    if t3d != 0 {
        // test t3d, t3d ; jz m(msac_decode_symbol_adapt4, SUFFIX).renorm3
        let mut t6d = core::ptr::read(t1.add(1)) as u32; // movzx t6d, word [t1+2]
        let mut ecx = t6d + 64; // lea ecx, [t6+64]
        t6d += (t6d < 32) as u32; // cmp t6d, 32 ; adc t6d, 0
        core::ptr::write(t1.add(1), t6d as u16); // mov [t1+2], t6w
        t6d = eax.wrapping_mul(-32769i32 as u32); // imul t6d, eax, -32769
        ecx >>= 4; // shr ecx, 4 ; rate
        t6d = t6d.wrapping_add(t5d); // add t6d, t5d ; if (bit)
        t5d = t5d.wrapping_sub(eax); // sub t5d, eax ;     cdf[0] -= ((cdf[0] - 32769) >> rate) + 1;
        t6d = ((t6d as i32) >> (ecx & 31)) as u32; // sar t6d, cl ; else
        t5d = t5d.wrapping_sub(t6d); // sub t5d, t6d ;     cdf[0] -= cdf[0] >> rate;
        core::ptr::write(t1, t5d as u16); // mov [t1], t5w
    }
    renorm3(t0, eax, t2d, t4) // jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm3
}

/// `cglobal msac_decode_bool_equi` (SSE2 build; general-purpose code):
/// `unsigned dav1d_msac_decode_bool_equi_sse2(MsacContext *s)`
pub unsafe extern "C" fn dav1d_msac_decode_bool_equi_sse2(s: *mut MsacContext) -> c_uint {
    let t0 = s;
    let mut t1d = (*t0).rng; // mov t1d, [t0+msac.rng]
    let mut t4 = (*t0).dif as u64; // mov t4, [t0+msac.dif]
    let mut t2d = t1d; // mov t2d, t1d
    t1d = (t1d & !0xff) | 8; // mov t1b, 8
    let t3 = t4; // mov t3, t4
    let rax = t1d as u64; // mov eax, t1d
    t1d >>= 1; // shr t1d, 1 ; v
    let rax = rax << 47; // shl rax, gprsize*8-17 ; vw
    t2d = t2d.wrapping_sub(t1d); // sub t2d, t1d ; r - v
    let below;
    (t4, below) = t4.overflowing_sub(rax); // sub t4, rax ; dif - vw
    if below {
        t2d = t1d; // cmovb t2d, t1d
    }
    t1d = (*t0).cnt as u32; // mov t1d, [t0+msac.cnt]
    if below {
        t4 = t3; // cmovb t4, t3
    }
    let t7 = t0; // movifnidn t7, t0
    let mut ecx = 0xbfffu32; // mov ecx, 0xbfff
    let eax = below as u32; // setb al ; the upper 32 bits contains garbage but that's OK
    ecx = ecx.wrapping_sub(t2d); // sub ecx, t2d
                                 // In this case of this function, (d =) 16 - clz(v) = 2 - (v >> 14)
                                 //   i.e. (0 <= d <= 2) and v < (3 << 14)
    ecx >>= 14; // shr ecx, 14 ; d
    renorm5(t7, eax, t1d, t2d, t4, ecx) // jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm5
}

/// `cglobal msac_decode_bool` (SSE2 build; general-purpose code):
/// `unsigned dav1d_msac_decode_bool_sse2(MsacContext *s, unsigned f)`
pub unsafe extern "C" fn dav1d_msac_decode_bool_sse2(s: *mut MsacContext, f: c_uint) -> c_uint {
    let (t0, mut t1d) = (s, f);
    let mut eax = ((*t0).rng >> 8) & 0xff; // movzx eax, byte [t0+msac.rng+1] ; r >> 8
    let mut t4 = (*t0).dif as u64; // mov t4, [t0+msac.dif]
    let mut t2d = (*t0).rng; // mov t2d, [t0+msac.rng]
    t1d &= !63; // and t1d, ~63
    eax = eax.wrapping_mul(t1d); // imul eax, t1d
    let t3 = t4; // mov t3, t4
    eax >>= 7; // shr eax, 7
    eax = eax.wrapping_add(4); // add eax, 4 ; v
    t1d = eax; // mov t1d, eax
    let rax = (eax as u64) << 48; // shl rax, gprsize*8-16 ; vw
    t2d = t2d.wrapping_sub(t1d); // sub t2d, t1d ; r - v
    let below;
    (t4, below) = t4.overflowing_sub(rax); // sub t4, rax ; dif - vw
    if below {
        t2d = t1d; // cmovb t2d, t1d
        t4 = t3; // cmovb t4, t3
    }
    let eax = below as u32; // setb al
    renorm3(t0, eax, t2d, t4) // jmp m(msac_decode_symbol_adapt4, SUFFIX).renorm3
}

/// The macro `HI_TOK update_cdf`: the loop of `msac_decode_hi_tok` with
/// (`UPDATE_CDF` true) or without the CDF update, to the function's `RET`.
#[inline(always)]
unsafe fn hi_tok<const UPDATE_CDF: bool>(
    t7: *mut MsacContext,
    t1: *mut u16,
    mut m0: __m128i,
    mut m2: __m128i,
    mut m3: __m128i,
    m4: __m128i,
    m5: __m128i,
    mut t4: u64,
    mut t5d: u32,
    mut t6d: u32,
) -> c_uint {
    // The doubleword at buf+12 and the 8 bytes at buf+16; both are stored
    // in every round of the loop before the loads from them.
    let mut buf = [0u16; 6];
    loop {
        // %%loop:
        let mut t2d = 0;
        if UPDATE_CDF {
            t2d = core::ptr::read(t1.add(3)) as u32; // movzx t2d, word [t1+3*2]
        }
        let mut m1 = m0; // mova m1, m0
        m2 = _mm_shufflelo_epi16::<Q0000>(m2); // pshuflw m2, m2, q0000
        m1 = _mm_srli_epi16::<6>(m1); // psrlw m1, 6
        _mm_storeu_si32(buf.as_mut_ptr().cast(), m2); // movd [buf+12], m2
        m2 = _mm_and_si128(m2, m4); // pand m2, m4
        m1 = _mm_slli_epi16::<7>(m1); // psllw m1, 7
        m1 = _mm_mulhi_epu16(m1, m2); // pmulhuw m1, m2
        m3 = _mm_shufflelo_epi16::<Q3333>(m3); // pshuflw m3, m3, c_shuf
        m1 = _mm_add_epi16(m1, m5); // paddw m1, m5
        _mm_storel_epi64(buf.as_mut_ptr().add(2).cast(), m1); // movq [buf+16], m1
        m1 = _mm_subs_epu16(m1, m3); // psubusw m1, m3
        m2 = _mm_setzero_si128(); // pxor m2, m2
        m1 = _mm_cmpeq_epi16(m1, m2); // pcmpeqw m1, m2
        let mut eax = _mm_movemask_epi8(m1) as u32; // pmovmskb eax, m1
        if UPDATE_CDF {
            let mut ecx = t2d + 80; // lea ecx, [t2+80]
            m2 = _mm_cmpeq_epi16(m2, m2); // pcmpeqw m2, m2
            ecx >>= 4; // shr ecx, 4
            t2d += (t2d < 32) as u32; // cmp t2d, 32 ; adc t2d, 0
            m3 = _mm_cvtsi32_si128(ecx as i32); // movd m3, ecx
            m2 = _mm_avg_epu16(m2, m1); // pavgw m2, m1
            m2 = _mm_sub_epi16(m2, m0); // psubw m2, m0
            m0 = _mm_sub_epi16(m0, m1); // psubw m0, m1
            m2 = _mm_sra_epi16(m2, m3); // psraw m2, m3
            m0 = _mm_add_epi16(m0, m2); // paddw m0, m2
            _mm_storel_epi64(t1.cast(), m0); // movq [t1], m0
            core::ptr::write(t1.add(3), t2d as u16); // mov [t1+3*2], t2w
        }
        eax = tzcnt(eax); // tzcnt eax, eax
        let p = buf.as_ptr().cast::<u8>().add(eax as usize);
        let rcx = core::ptr::read_unaligned(p.add(4).cast::<u16>()) as u64; // movzx ecx, word [buf+rax+16]
        t2d = core::ptr::read_unaligned(p.add(2).cast::<u16>()) as u32; // movzx t2d, word [buf+rax+14]
        t6d = t6d.wrapping_add(5); // add t6d, 5
        eax = eax.wrapping_sub(5); // sub eax, 5 ; setup for merging the tok_br and tok branches
        t2d = t2d.wrapping_sub(rcx as u32); // sub t2d, ecx
        let rcx = rcx << 48; // shl rcx, gprsize*8-16
        t4 = t4.wrapping_sub(rcx); // sub t4, rcx
        let ecx = bsr_xor_15(t2d); // bsr ecx, t2d ; xor ecx, 15
        t2d <<= ecx & 31; // shl t2d, cl
        t4 <<= ecx & 63; // shl t4, cl
        m2 = _mm_cvtsi32_si128(t2d as i32); // movd m2, t2d
        (*t7).rng = t2d; // mov [t7+msac.rng], t2d
        let below;
        (t5d, below) = t5d.overflowing_sub(ecx); // sub t5d, ecx
        if below {
            // jae %%end
            (t5d, t4) = refill(t7, t5d, t4);
        }
        // %%end:
        m3 = _mm_cvtsi64_si128(t4 as i64); // movp m3, t4
        let carry;
        (t6d, carry) = t6d.overflowing_add(eax); // add t6d, eax ; CF = tok_br < 3 || tok == 15
        if carry {
            break; // jnc %%loop
        }
    }
    let mut eax = t6d.wrapping_add(30); // lea eax, [t6+30]
    (*t7).dif = t4 as usize; // mov [t7+msac.dif], t4
    eax >>= 1; // shr eax, 1
    (*t7).cnt = t5d as c_int; // mov [t7+msac.cnt], t5d
    eax // RET
}

/// `cglobal msac_decode_hi_tok` (SSE2):
/// `unsigned dav1d_msac_decode_hi_tok_sse2(MsacContext *s, uint16_t *cdf)`
pub unsafe extern "C" fn dav1d_msac_decode_hi_tok_sse2(s: *mut MsacContext, cdf: *mut u16) -> c_uint {
    let (t0, t1) = (s, cdf);
    let m0 = _mm_loadl_epi64(t1.cast()); // movq m0, [t1]
    let m2 = _mm_cvtsi32_si128((*t0).rng as i32); // movd m2, [t0+msac.rng]
    let eax = (*t0).update_cdf; // mov eax, [t0+msac.update_cdf]
    let m4 = _mm_loadl_epi64(RODATA.0.as_ptr().add(PW_0XFF00).cast()); // movq m4, [base+pw_0xff00]
    let m3 = _mm_cvtsi64_si128((*t0).dif as i64); // movp m3, [t0+msac.dif]
    let m5 = _mm_loadl_epi64(RODATA.0.as_ptr().add(MIN_PROB + 12).cast()); // movq m5, [base+min_prob+12*2]
    let t4 = (*t0).dif as u64; // mov t4, [t0+msac.dif]
    let t5d = (*t0).cnt as u32; // mov t5d, [t0+msac.cnt]
    let t6d = -24i32 as u32; // mov t6d, -24
    let t7 = t0; // movifnidn t7, t0
    if eax != 0 {
        // test eax, eax ; jz .no_update_cdf
        hi_tok::<true>(t7, t1, m0, m2, m3, m4, m5, t4, t5d, t6d) // HI_TOK 1
    } else {
        // .no_update_cdf:
        hi_tok::<false>(t7, t1, m0, m2, m3, m4, m5, t4, t5d, t6d) // HI_TOK 0
    }
}

// --- INIT_YMM avx2 -----------------------------------------------------------

/// `cglobal msac_decode_symbol_adapt16` (AVX2):
/// `unsigned dav1d_msac_decode_symbol_adapt16_avx2(MsacContext *s, uint16_t *cdf, size_t n_symbols)`
///
/// # Safety
/// As for the family, and the CPU supports AVX2 (the caller selects this
/// function under `DAV1D_X86_CPU_FLAG_AVX2`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt16_avx2(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
) -> c_uint {
    let (t0, t1, mut t2) = (s, cdf, n_symbols as u64);
    // The doubleword at buf-4 and the 32 bytes at buf (`r5-32`, the stack
    // pointer aligned down to 32 bytes).
    let mut buf = [0u16; 18];
    let rax = RODATA.0.as_ptr().add(PW_0XFF00); // lea rax, [pw_0xff00]
    let mut m2 = _mm256_set1_epi16((*t0).rng as i16); // vpbroadcastw m2, [t0+msac.rng]
    let mut m0 = _mm256_loadu_si256(t1.cast()); // mova m0, [t1]
    let m3 = _mm256_set1_epi16(((*t0).dif as u64 >> 48) as i16); // vpbroadcastw m3, [t0+msac.dif+6]
    let m4 = _mm256_broadcastsi128_si256(_mm_loadu_si128(rax.cast())); // vbroadcasti128 m4, [rax]
    let mut t3d = (*t0).update_cdf as u32; // mov t3d, [t0+msac.update_cdf]
    let t4d = t2 as u32; // mov t4d, t2d
    t2 = !t2; // not t2
              // mov r5, rsp ; and r5, ~31 ; %define buf r5-32
    let mut m1 = _mm256_srli_epi16::<6>(m0); // psrlw m1, m0, 6
    _mm_storeu_si32(buf.as_mut_ptr().cast(), _mm256_castsi256_si128(m2)); // movd [buf-4], xm2
    m2 = _mm256_and_si256(m2, m4); // pand m2, m4
    m1 = _mm256_slli_epi16::<7>(m1); // psllw m1, 7
    m1 = _mm256_mulhi_epu16(m1, m2); // pmulhuw m1, m2
    m1 = _mm256_add_epi16(m1, _mm256_loadu_si256(rax.offset(t2 as i64 as isize).cast())); // paddw m1, [rax+t2*2]
    _mm256_storeu_si256(buf.as_mut_ptr().add(2).cast(), m1); // mova [buf], m1
    m1 = _mm256_max_epu16(m1, m3); // pmaxuw m1, m3
    m1 = _mm256_cmpeq_epi16(m1, m3); // pcmpeqw m1, m3
    let eax = _mm256_movemask_epi8(m1) as u32; // pmovmskb eax, m1
    if t3d != 0 {
        // test t3d, t3d ; jz .renorm
        t3d = core::ptr::read(t1.add(t4d as usize)) as u32; // movzx t3d, word [t1+t4*2]
        m2 = _mm256_cmpeq_epi16(m2, m2); // pcmpeqw m2, m2
        let mut t2d = t3d + 80; // lea t2d, [t3+80]
        t2d >>= 4; // shr t2d, 4
        t3d += (t3d < 32) as u32; // cmp t3d, 32 ; adc t3d, 0
        let xm3 = _mm_cvtsi32_si128(t2d as i32); // movd xm3, t2d
        m2 = _mm256_avg_epu16(m2, m1); // pavgw m2, m1
        m2 = _mm256_sub_epi16(m2, m0); // psubw m2, m0
        m0 = _mm256_sub_epi16(m0, m1); // psubw m0, m1
        m2 = _mm256_sra_epi16(m2, xm3); // psraw m2, xm3
        m0 = _mm256_add_epi16(m0, m2); // paddw m0, m2
        _mm256_storeu_si256(t1.cast(), m0); // mova [t1], m0
        core::ptr::write(t1.add(t4d as usize), t3d as u16); // mov [t1+t4*2], t3w
    }
    // .renorm:
    let eax = tzcnt(eax); // tzcnt eax, eax
    let t4 = (*t0).dif as u64; // mov t4, [t0+msac.dif]
    let p = buf.as_ptr().cast::<u8>().add(eax as usize);
    let t1 = core::ptr::read_unaligned(p.add(4).cast::<u16>()) as u64; // movzx t1d, word [buf+rax-0]
    let t2d = core::ptr::read_unaligned(p.add(2).cast::<u16>()) as u32; // movzx t2d, word [buf+rax-2]
    let eax = eax >> 1; // shr eax, 1
                        // vzeroupper (the compiler's, at the return)
    renorm2(t0, eax, t1, t2d, t4) // jmp m(msac_decode_symbol_adapt4, _sse2).renorm2
}
