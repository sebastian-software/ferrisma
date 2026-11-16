// Copyright © 2019, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/msac.S` with NEON intrinsics
//! (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The seven entry points of the entropy decoder: `symbol_adapt4/8/16`
//! (the `decode_update` macro at `.4h`, `.8h` and two `.8h` registers),
//! `hi_tok`, `bool_equi`, `bool` and `bool_adapt`, with the shared refill
//! `L(refill)`.
//!
//! Correspondence with the assembly:
//! - Each exported `function` is one `extern "C"` function of the same
//!   name; the macro `decode_update` is `decode_update::<N>` with the
//!   macro's `.if \n == …` branches on the const parameter.
//! - The vector registers are the array `v`, indexed by register number,
//!   as `uint16x8_t`; a `.4h`/`.8b` instruction writes the low half and
//!   clears the high half (`d`). The general-purpose registers are locals
//!   named as in the assembly (`w4`, `x7`, …): a `w` result is a `u32`, an
//!   `x` result a `u64`, and a register-specified shift uses the low 5
//!   (`w`) or 6 (`x`) bits of the amount, as the hardware does. A
//!   comparison that only sets the flags for a following `b.lo`/`b.hi`/
//!   `cset`/`csel`/`sbc` is the same unsigned comparison written out.
//! - The 48-byte stack buffer of the 16-symbol case is the local `sp`
//!   (24 halfwords); the table `coeffs` is `COEFFS`.
//! - The refill (`L(refill)` and its copy in `hi_tok`) is `refill`; it
//!   returns the new `cnt` and `dif` (`w6`, `x7`) to the code that stores
//!   them, as the shared tail of the assembly does.
//!
//! One deviation, for memory safety only: near the end of the buffer
//! (`refill_eob`) the assembly loads the 8 bytes that end at `buf_end` and
//! shifts out the ones before `buf_pos`; when fewer than 8 bytes precede
//! `buf_end` in the buffer that load reaches before it. Here the bytes
//! `buf_pos..buf_end` are copied to the low end of a zeroed 8-byte word,
//! which is the value the load and the shift produce, without the bytes the
//! shift discards.
//!
//! Safety (argued once for the family): the callers are the dispatch shims
//! of `simd/arm/msac.rs`, called where the original calls the assembly
//! through the macros of `src/arm/msac.h`. They pass what the original
//! passes: `s` points to a live `MsacContext` initialized by
//! `dav1d_msac_init` (layout checked by a test), `buf_pos..buf_end` is
//! readable, and `cdf` points to the CDF array of the symbol, writable and
//! of at least 4 (`adapt4`, `hi_tok`), 8 (`adapt8`) or 16 (`adapt16`)
//! halfwords, as dav1d's `CdfContext` arrays are, with `n_symbols` in the
//! range of the entry point (1–3, 1–7, 3–15); `bool_adapt` reads and writes
//! `cdf[0..2]`. The loads and stores below stay inside the context, those
//! CDF halfwords, `buf_pos..buf_end` and the local `sp`.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::{c_int, c_uint};

/// `MsacContext` (src/msac.h) as the assembly sees it on AArch64: `BUF_POS`
/// 0, `BUF_END` 8, `DIF` 16, `RNG` 24, `CNT` 28, `ALLOW_UPDATE_CDF` 32.
#[repr(C)]
pub struct MsacContext {
    pub buf_pos: *const u8,
    pub buf_end: *const u8,
    pub dif: usize,
    pub rng: c_uint,
    pub cnt: c_int,
    pub allow_update_cdf: c_int,
}

/// `coeffs`: `EC_MIN_PROB * (15 - i)`, 16 zeros, then `masks8`.
static COEFFS: [u16; 40] = [
    60, 56, 52, 48, 44, 40, 36, 32, 28, 24, 20, 16, 12, 8, 4, 0, //
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, //
    // masks8
    0xfdfe, 0xfdfe, 0xfdfe, 0xfdfe, 0xfdfe, 0xfdfe, 0xfdfe, 0x0f0e,
];
/// `COEFFS_BASE_OFFSET` (30 bytes) and `MASKS8_OFFSET` (34 bytes from it),
/// in halfwords.
const COEFFS_BASE_OFFSET: usize = 15;
const MASKS8_OFFSET: usize = 17;

// --- one instruction each; `q` selects `.8h` (true) or `.4h` (false) -------

/// The value a 64-bit destination receives: the low half, the high half
/// cleared.
#[inline(always)]
fn d(x: uint16x4_t) -> uint16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vcombine_u16(x, vdup_n_u16(0)) }
}

#[inline(always)]
fn lo(x: uint16x8_t) -> uint16x4_t {
    // SAFETY: as above.
    unsafe { vget_low_u16(x) }
}

/// `movi`/`mvni`/`dup` with a halfword value
#[inline(always)]
fn dup(q: bool, x: u16) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        if q {
            vdupq_n_u16(x)
        } else {
            d(vdup_n_u16(x))
        }
    }
}

#[inline(always)]
fn and(q: bool, a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        if q {
            vandq_u16(a, b)
        } else {
            d(vand_u16(lo(a), lo(b)))
        }
    }
}

#[inline(always)]
fn add(q: bool, a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        if q {
            vaddq_u16(a, b)
        } else {
            d(vadd_u16(lo(a), lo(b)))
        }
    }
}

#[inline(always)]
fn sub(q: bool, a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        if q {
            vsubq_u16(a, b)
        } else {
            d(vsub_u16(lo(a), lo(b)))
        }
    }
}

/// `orr vd, #0x80, lsl #8`
#[inline(always)]
fn orr8000(q: bool, a: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        if q {
            vorrq_u16(a, vdupq_n_u16(0x8000))
        } else {
            d(vorr_u16(lo(a), vdup_n_u16(0x8000)))
        }
    }
}

/// `cmhs vd, vn, vm`
#[inline(always)]
fn cmhs(q: bool, a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        if q {
            vcgeq_u16(a, b)
        } else {
            d(vcge_u16(lo(a), lo(b)))
        }
    }
}

/// `sqdmulh vd, vn, vm` (signed halfwords)
#[inline(always)]
fn sqdmulh(q: bool, a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        let (a, b) = (vreinterpretq_s16_u16(a), vreinterpretq_s16_u16(b));
        vreinterpretq_u16_s16(if q {
            vqdmulhq_s16(a, b)
        } else {
            vcombine_s16(vqdmulh_s16(vget_low_s16(a), vget_low_s16(b)), vdup_n_s16(0))
        })
    }
}

/// `sshl vd, vn, vm` (shift by the signed low byte of each lane of `vm`)
#[inline(always)]
fn sshl(q: bool, a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe {
        let (a, b) = (vreinterpretq_s16_u16(a), vreinterpretq_s16_u16(b));
        vreinterpretq_u16_s16(if q {
            vshlq_s16(a, b)
        } else {
            vcombine_s16(vshl_s16(vget_low_s16(a), vget_low_s16(b)), vdup_n_s16(0))
        })
    }
}

/// `umov xN, vM.d[0]`
#[inline(always)]
fn d0(a: uint16x8_t) -> u64 {
    // SAFETY: as above.
    unsafe { vgetq_lane_u64::<0>(vreinterpretq_u64_u16(a)) }
}

/// `addv hd, vn.8h`: the sum in lane 0, the rest of the register cleared.
#[inline(always)]
fn addv(a: uint16x8_t) -> uint16x8_t {
    // SAFETY: as above.
    unsafe { vsetq_lane_u16::<0>(vaddvq_u16(a), vdupq_n_u16(0)) }
}

/// `ld1 {vd.4h|.8h}, [p]`
#[inline(always)]
unsafe fn ld(q: bool, p: *const u16) -> uint16x8_t {
    if q {
        vld1q_u16(p)
    } else {
        d(vld1_u16(p))
    }
}

/// `st1 {vs.4h|.8h}, [p]`
#[inline(always)]
unsafe fn st(q: bool, p: *mut u16, a: uint16x8_t) {
    if q {
        vst1q_u16(p, a)
    } else {
        vst1_u16(p, lo(a))
    }
}

/// `strh wN, [x0, #RNG]`: the low halfword of `rng`.
#[inline(always)]
unsafe fn strh_rng(s: *mut MsacContext, w: u32) {
    // AArch64 is little-endian: the low halfword is at RNG.
    core::ptr::write((&raw mut (*s).rng).cast::<u16>(), w as u16);
}

// --- L(refill) ---------------------------------------------------------------

/// The refill of `L(refill)` (labels 2–6) and of `hi_tok` (3–7): reads
/// the next bytes into `dif` below its valid bits and returns the new
/// `cnt` and `dif` (`w6`, `x7`); `buf_pos` is stored here.
#[inline(always)]
unsafe fn refill(s: *mut MsacContext, mut w6: u32, mut x7: u64) -> (u32, u64) {
    // ldp x3, x4, [x0]  // BUF_POS, BUF_END
    let x3 = (*s).buf_pos;
    let x4 = (*s).buf_end;
    let (a3, a4) = (x3 as u64, x4 as u64);
    // add x5, x3, #8 ; subs x5, x5, x4 ; b.hi 6f
    let x8: u64;
    let w5: u32;
    if a3.wrapping_add(8) > a4 {
        // 6: refill_eob
        // cmp x3, x4 ; b.hs 5b
        if a3 >= a4 {
            // 5: pad_with_ones
            // add w8, w6, #-16 ; ror x8, x8, x8 ; b 3b (refill_end2)
            let w8 = w6.wrapping_sub(16) as u64;
            let x8 = w8.rotate_right((w8 & 63) as u32);
            // 3: orr x7, x7, x8
            x7 |= x8;
            return (w6, x7);
        }
        (x8, w5) = refill_eob(x3, x4, w6);
        // b 2b
    } else {
        // ldr x8, [x3]  // next_bits
        let mut y8 = core::ptr::read_unaligned(x3.cast::<u64>());
        // add w4, w6, #-48  // shift_bits = cnt + 16 (- 64)
        let w4 = w6.wrapping_sub(48);
        // mvn x8, x8
        y8 = !y8;
        // neg w5, w4
        let w5a = w4.wrapping_neg();
        // rev x8, x8  // next_bits = bswap(next_bits)
        y8 = y8.swap_bytes();
        // lsr w5, w5, #3  // num_bytes_read
        w5 = w5a >> 3;
        // lsr x8, x8, x4  // next_bits >>= (shift_bits & 63)
        x8 = y8 >> (w4 & 63);
    }
    // 2: refill_end
    // add x3, x3, x5
    let x3 = x3.add(w5 as usize);
    // add w6, w6, w5, lsl #3  // cnt += num_bits_read
    w6 = w6.wrapping_add(w5 << 3);
    // str x3, [x0, #BUF_POS]
    (*s).buf_pos = x3;
    // 3: refill_end2
    // orr x7, x7, x8  // dif |= next_bits
    x7 |= x8;
    (w6, x7)
}

/// `refill_eob` (label 6, after the `pad_with_ones` test): the next bits
/// and `num_bytes_read` when fewer than 8 bytes are left. Plain loads
/// rather than a copy call or a loop, so that the kernels stay leaf
/// functions without a stack frame, as the assembly is.
#[inline(always)]
unsafe fn refill_eob(x3: *const u8, x4: *const u8, w6: u32) -> (u64, u32) {
    // ldr x8, [x4, #-8] ; lsl w5, w5, #3 ; lsr x8, x8, x5: the bytes
    // buf_pos..buf_end at the low end of the word (see the module
    // documentation).
    let left = x4.offset_from(x3) as usize;
    // 1 to 7 bytes, read as 4 + 2 + 1 (no loop, no copy call).
    let mut y8 = 0u64;
    let mut i = 0;
    if left & 4 != 0 {
        y8 = core::ptr::read_unaligned(x3.cast::<u32>()) as u64;
        i = 4;
    }
    if left & 2 != 0 {
        y8 |= (core::ptr::read_unaligned(x3.add(i).cast::<u16>()) as u64) << (8 * i);
        i += 2;
    }
    if left & 1 != 0 {
        y8 |= (core::ptr::read(x3.add(i)) as u64) << (8 * i);
    }
    // add w5, w6, #-48
    let w5a = w6.wrapping_sub(48);
    // mvn x8, x8
    y8 = !y8;
    // sub w4, w4, w3  // num_bytes_left
    let w4 = (x4 as u64 as u32).wrapping_sub(x3 as u64 as u32);
    // rev x8, x8
    y8 = y8.swap_bytes();
    // lsr x8, x8, x5
    y8 >>= w5a & 63;
    // neg w5, w5 ; lsr w5, w5, #3
    let w5b = w5a.wrapping_neg() >> 3;
    // cmp w5, w4 ; csel w5, w5, w4, lo  // num_bytes_read
    (y8, if w5b < w4 { w5b } else { w4 })
}

/// `L(refill)` with its end: stores `cnt` and `dif` and returns `w15`.
#[inline(always)]
unsafe fn refill_end(s: *mut MsacContext, w6: u32, x7: u64, w15: u32) -> c_uint {
    let (w6, x7) = refill(s, w6, x7);
    // 4: end
    // str w6, [x0, #CNT] ; str x7, [x0, #DIF] ; mov w0, w15
    (*s).cnt = w6 as c_int;
    (*s).dif = x7 as usize;
    w15
}

// --- decode_update -------------------------------------------------------------

/// The macro `decode_update sz, szb, n` for `n` = 4 (`.4h`), 8 (`.8h`) and
/// 16 (two `.8h` registers).
#[inline(always)]
unsafe fn decode_update<const N: usize>(s: *mut MsacContext, x1: *mut u16, x2: usize) -> c_uint {
    let q = N != 4;
    let mut v = [vdupq_n_u16(0); 32];
    // .if \n == 16: sub sp, sp, #48
    let mut sp = [0u16; 24];
    // add x8, x0, #RNG
    // ld1_n v0, v1, x1  // cdf
    v[0] = ld(q, x1);
    if N == 16 {
        v[1] = ld(q, x1.add(8));
    }
    // ld1r {v29\sz}, [x8]  // rng
    v[29] = dup(q, core::ptr::read((&raw const (*s).rng).cast::<u16>()));
    // movrel x9, coeffs, COEFFS_BASE_OFFSET
    let x9 = COEFFS.as_ptr().add(COEFFS_BASE_OFFSET);
    // movi v31\sz, #0x7f, lsl #8  // 0x7f00
    v[31] = dup(q, 0x7f00);
    // sub x10, x9, x2, lsl #1
    let x10 = x9.sub(x2);
    // mvni v30\sz, #0x3f  // 0xffc0
    v[30] = dup(q, 0xffc0);
    // and v7\szb, v29\szb, v31\szb  // rng & 0x7f00
    v[7] = and(q, v[29], v[31]);
    if N == 16 {
        // str h29, [sp, #14]  // store original u = s->rng
        sp[7] = vgetq_lane_u16::<0>(v[29]);
    }
    // and_n v2, v3, v0, v1, v30, v30  // cdf & 0xffc0
    v[2] = and(q, v[0], v[30]);
    if N == 16 {
        v[3] = and(q, v[1], v[30]);
    }
    // ld1_n v4, v5, x10  // EC_MIN_PROB * (n_symbols - ret)
    v[4] = ld(q, x10);
    if N == 16 {
        v[5] = ld(q, x10.add(8));
    }
    // sqdmulh_n v6, v7, v2, v3, v7, v7  // ((cdf >> EC_PROB_SHIFT) * (r - 128)) >> 1
    v[6] = sqdmulh(q, v[2], v[7]);
    if N == 16 {
        v[7] = sqdmulh(q, v[3], v[7]);
    }
    // ldr d28, [x0, #DIF]
    v[28] = d(vcreate_u16((*s).dif as u64));
    // add_n v4, v5, v2, v3, v4, v5  // v = cdf + EC_MIN_PROB * (n_symbols - ret)
    v[4] = add(q, v[2], v[4]);
    if N == 16 {
        v[5] = add(q, v[3], v[5]);
    }
    // add_n v4, v5, v6, v7, v4, v5
    v[4] = add(q, v[6], v[4]);
    if N == 16 {
        v[5] = add(q, v[7], v[5]);
    }
    // dup v30\sz, v28.h[3]  // dif >> (EC_WIN_SIZE - 16)
    v[30] = dup(q, vgetq_lane_u16::<3>(v[28]));
    if N == 8 {
        // ldur q31, [x9, #MASKS8_OFFSET]
        v[31] = vld1q_u16(x9.add(MASKS8_OFFSET));
    } else if N == 16 {
        // str_n q4, q5, sp, #16  // store v values to allow indexed access
        vst1q_u16(sp.as_mut_ptr().add(8), v[4]);
        vst1q_u16(sp.as_mut_ptr().add(16), v[5]);
    }
    // cmhs_n v2, v3, v30, v30, v4, v5  // c >= v
    v[2] = cmhs(q, v[30], v[4]);
    if N == 16 {
        v[3] = cmhs(q, v[30], v[5]);
    }
    let w4: u32;
    let mut w15: u32 = 0;
    let mut x15: u64 = 0;
    if N == 4 {
        // ext v29.8b, v29.8b, v4.8b, #6  // u
        v[29] = d(vext_u16::<3>(lo(v[29]), lo(v[4])));
        // umov x15, v2.d[0]
        x15 = d0(v[2]);
        // ldr w4, [x0, #ALLOW_UPDATE_CDF]
        w4 = (*s).allow_update_cdf as u32;
        // rev x15, x15
        x15 = x15.swap_bytes();
        // sub v29.4h, v29.4h, v4.4h  // rng = u-v
        v[29] = sub(false, v[29], v[4]);
        // clz x15, x15  // 16*ret
        x15 = x15.leading_zeros() as u64;
    } else if N == 8 {
        // and v31.16b, v31.16b, v2.16b
        v[31] = vandq_u16(v[31], v[2]);
        // ext v29.16b, v29.16b, v4.16b, #14  // u
        v[29] = vextq_u16::<7>(v[29], v[4]);
        // addv h31, v31.8h  // ((2*ret + 1) << 8) | (2*ret)
        v[31] = addv(v[31]);
        // ldr w4, [x0, #ALLOW_UPDATE_CDF]
        w4 = (*s).allow_update_cdf as u32;
        // sub v30.8h, v30.8h, v4.8h  // (dif >> 48) - v
        v[30] = vsubq_u16(v[30], v[4]);
        // smov w15, v31.b[0]  // 2*ret
        w15 = vgetq_lane_u8::<0>(vreinterpretq_u8_u16(v[31])) as i8 as i32 as u32;
        // sub v29.8h, v29.8h, v4.8h  // rng = u-v
        v[29] = vsubq_u16(v[29], v[4]);
    } else {
        // add v6.8h, v2.8h, v3.8h
        v[6] = vaddq_u16(v[2], v[3]);
        // addv h31, v6.8h  // -n + ret
        v[31] = addv(v[6]);
        // ldr w4, [x0, #ALLOW_UPDATE_CDF]
        w4 = (*s).allow_update_cdf as u32;
        // smov w15, v31.h[0]
        w15 = vgetq_lane_u16::<0>(v[31]) as i16 as i32 as u32;
    }

    // cbz w4, 0f
    if w4 != 0 {
        // update_cdf
        // ldrh w3, [x1, x2, lsl #1]  // count = cdf[n_symbols]
        let mut w3 = core::ptr::read(x1.add(x2)) as u32;
        let mut w4: u32;
        let mut w14: u32 = 0;
        let mut carry = false;
        if N == 16 {
            // mov w4, #-5
            w4 = -5i32 as u32;
        } else if N == 8 {
            // mvn w14, w2
            w14 = !(x2 as u32);
            // mov w4, #-4
            w4 = -4i32 as u32;
            // cmn w14, #3  // set C if n_symbols <= 2
            carry = (w14 as u64) + 3 > u32::MAX as u64;
        } else {
            // add w14, w2, #17  // (1 + n_symbols) + (4 << 2)
            w14 = (x2 as u32).wrapping_add(17);
            w4 = 0;
        }
        // sub_n v16, v17, v0, v1, v2, v3  // cdf + (i >= val ? 1 : 0)
        v[16] = sub(q, v[0], v[2]);
        if N == 16 {
            v[17] = sub(q, v[1], v[3]);
        }
        // orr v2\sz, #0x80, lsl #8
        v[2] = orr8000(q, v[2]);
        if N == 16 {
            // orr v3\sz, #0x80, lsl #8
            v[3] = orr8000(q, v[3]);
        }
        if N == 16 {
            // sub w4, w4, w3, lsr #4  // -((count >> 4) + 5)
            w4 = w4.wrapping_sub(w3 >> 4);
        } else if N == 8 {
            // lsr w14, w3, #4  // count >> 4
            w14 = w3 >> 4;
            // sbc w4, w4, w14  // -((count >> 4) + (n_symbols > 2) + 4)
            w4 = w4.wrapping_sub(w14).wrapping_sub(!carry as u32);
        } else {
            // neg w4, w14, lsr #2  // -((n_symbols > 2) + 4)
            w4 = (w14 >> 2).wrapping_neg();
            // sub w4, w4, w3, lsr #4  // -((count >> 4) + (n_symbols > 2) + 4)
            w4 = w4.wrapping_sub(w3 >> 4);
        }
        // sub_n v2, v3, v2, v3, v0, v1  // (32768 - cdf[i]) or (-1 - cdf[i])
        v[2] = sub(q, v[2], v[0]);
        if N == 16 {
            v[3] = sub(q, v[3], v[1]);
        }
        // dup v6\sz, w4  // -rate
        v[6] = dup(q, w4 as u16);
        // sub w3, w3, w3, lsr #5  // count - (count == 32)
        w3 = w3.wrapping_sub(w3 >> 5);
        // sshl_n v2, v3, v2, v3, v6, v6  // ({32768,-1} - cdf[i]) >> rate
        v[2] = sshl(q, v[2], v[6]);
        if N == 16 {
            v[3] = sshl(q, v[3], v[6]);
        }
        // add w3, w3, #1  // count + (count < 32)
        w3 = w3.wrapping_add(1);
        // add_n v0, v1, v16, v17, v2, v3  // cdf + (32768 - cdf[i]) >> rate
        v[0] = add(q, v[16], v[2]);
        if N == 16 {
            v[1] = add(q, v[17], v[3]);
        }
        // st1_n v0, v1, x1
        st(q, x1, v[0]);
        if N == 16 {
            st(q, x1.add(8), v[1]);
        }
        // strh w3, [x1, x2, lsl #1]
        core::ptr::write(x1.add(x2), w3 as u16);
    }

    // 0: renorm
    if N == 4 {
        // ldr w6, [x0, #CNT] ; ldr x7, [x0, #DIF]
        let mut w6 = (*s).cnt as u32;
        let mut x7 = (*s).dif as u64;
        // mov x4, v29.d[0]  // rng (packed)
        let mut x4 = d0(v[29]);
        // mov x3, v4.d[0]  // v (packed)
        let mut x3 = d0(v[4]);
        // lsr x4, x4, x15 ; lsr x3, x3, x15
        x4 >>= x15 & 63;
        x3 >>= x15 & 63;
        let mut w4 = x4 as u32;
        // lsl w5, w4, #16  // rng << 16
        let mut w5 = w4 << 16;
        // sub x7, x7, x3, lsl #48  // dif - (v << 48)
        x7 = x7.wrapping_sub(x3 << 48);
        // clz w5, w5  // d = clz(rng << 16)
        w5 = w5.leading_zeros();
        // lsl w4, w4, w5  // rng << d
        w4 <<= w5 & 31;
        // subs w6, w6, w5  // cnt -= d
        let lo = w6 < w5;
        w6 = w6.wrapping_sub(w5);
        // lsl x7, x7, x5  // (dif - (v << 48)) << d
        x7 <<= (w5 as u64) & 63;
        // strh w4, [x0, #RNG]
        strh_rng(s, w4);
        // b.lo 1f
        if lo {
            // 1: lsr w15, w15, #4 ; b L(refill)
            return refill_end(s, w6, x7, (x15 as u32) >> 4);
        }
        // str w6, [x0, #CNT] ; str x7, [x0, #DIF] ; lsr w0, w15, #4
        (*s).cnt = w6 as c_int;
        (*s).dif = x7 as usize;
        (x15 as u32) >> 4
    } else if N == 8 {
        // ldr w6, [x0, #CNT]
        let mut w6 = (*s).cnt as u32;
        let idx = vget_low_u8(vreinterpretq_u8_u16(v[31]));
        // tbl v30.8b, {v30.16b}, v31.8b
        v[30] = d(vreinterpret_u16_u8(vqtbl1_u8(vreinterpretq_u8_u16(v[30]), idx)));
        // tbl v29.8b, {v29.16b}, v31.8b
        v[29] = d(vreinterpret_u16_u8(vqtbl1_u8(vreinterpretq_u8_u16(v[29]), idx)));
        // ins v28.h[3], v30.h[0]  // dif - (v << 48)
        v[28] = vsetq_lane_u16::<3>(vgetq_lane_u16::<0>(v[30]), v[28]);
        // clz v0.4h, v29.4h  // d = clz(rng)
        v[0] = d(vclz_u16(lo(v[29])));
        // umov w5, v0.h[0]
        let w5 = vgetq_lane_u16::<0>(v[0]) as u32;
        // ushl v29.4h, v29.4h, v0.4h  // rng << d
        v[29] = d(vshl_u16(lo(v[29]), vreinterpret_s16_u16(lo(v[0]))));
        // ushl d28, d28, d0  // (dif - (v << 48)) << d
        v[28] = vreinterpretq_u16_u64(vcombine_u64(
            vshl_u64(vget_low_u64(vreinterpretq_u64_u16(v[28])), vreinterpret_s64_u16(lo(v[0]))),
            vdup_n_u64(0),
        ));
        // subs w6, w6, w5  // cnt -= d
        let lo_ = w6 < w5;
        w6 = w6.wrapping_sub(w5);
        // str h29, [x0, #RNG]
        strh_rng(s, vgetq_lane_u16::<0>(v[29]) as u32);
        // b.lo 1f
        if lo_ {
            // 1: lsr w15, w15, #1 ; mov x7, v28.d[0] ; b L(refill)
            return refill_end(s, w6, d0(v[28]), w15 >> 1);
        }
        // str w6, [x0, #CNT] ; str d28, [x0, #DIF] ; lsr w0, w15, #1
        (*s).cnt = w6 as c_int;
        (*s).dif = d0(v[28]) as usize;
        w15 >> 1
    } else {
        // add x8, sp, w15, sxtw #1
        let x8 = (w15 as i32 + 24) as usize;
        // ldrh w3, [x8, #48]  // v
        let w3 = sp[x8] as u32;
        // ldurh w4, [x8, #46]  // u
        let mut w4 = sp[x8 - 1] as u32;
        // ldr w6, [x0, #CNT] ; ldr x7, [x0, #DIF]
        let mut w6 = (*s).cnt as u32;
        let mut x7 = (*s).dif as u64;
        // sub w4, w4, w3  // rng = u - v
        w4 = w4.wrapping_sub(w3);
        // clz w5, w4 ; eor w5, w5, #16  // d = clz(rng) ^ 16
        let w5 = w4.leading_zeros() ^ 16;
        // sub x7, x7, x3, lsl #48  // dif - (v << 48)
        x7 = x7.wrapping_sub((w3 as u64) << 48);
        // lsl w4, w4, w5  // rng << d
        w4 <<= w5 & 31;
        // subs w6, w6, w5  // cnt -= d
        let lo = w6 < w5;
        w6 = w6.wrapping_sub(w5);
        // lsl x7, x7, x5  // (dif - (v << 48)) << d
        x7 <<= (w5 as u64) & 63;
        // str w4, [x0, #RNG]
        (*s).rng = w4;
        // add sp, sp, #48 ; b.lo 1f
        if lo {
            // 1: add w15, w15, #\n ; b L(refill)
            return refill_end(s, w6, x7, w15.wrapping_add(16));
        }
        // str w6, [x0, #CNT] ; str x7, [x0, #DIF] ; add w0, w15, #\n
        (*s).cnt = w6 as c_int;
        (*s).dif = x7 as usize;
        w15.wrapping_add(16)
    }
}

/// `unsigned dav1d_msac_decode_symbol_adapt4_neon(MsacContext *s, uint16_t *cdf, size_t n_symbols)`
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt4_neon(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
) -> c_uint {
    decode_update::<4>(s, cdf, n_symbols)
}

/// `unsigned dav1d_msac_decode_symbol_adapt8_neon(MsacContext *s, uint16_t *cdf, size_t n_symbols)`
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt8_neon(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
) -> c_uint {
    decode_update::<8>(s, cdf, n_symbols)
}

/// `unsigned dav1d_msac_decode_symbol_adapt16_neon(MsacContext *s, uint16_t *cdf, size_t n_symbols)`
pub unsafe extern "C" fn dav1d_msac_decode_symbol_adapt16_neon(
    s: *mut MsacContext,
    cdf: *mut u16,
    n_symbols: usize,
) -> c_uint {
    decode_update::<16>(s, cdf, n_symbols)
}

/// `unsigned dav1d_msac_decode_hi_tok_neon(MsacContext *s, uint16_t *cdf)`
pub unsafe extern "C" fn dav1d_msac_decode_hi_tok_neon(s: *mut MsacContext, x1: *mut u16) -> c_uint {
    let mut v = [vdupq_n_u16(0); 32];
    // ld1 {v0.4h}, [x1]  // cdf
    v[0] = ld(false, x1);
    // add x16, x0, #RNG
    // movi v31.4h, #0x7f, lsl #8  // 0x7f00
    v[31] = dup(false, 0x7f00);
    // movrel x17, coeffs, COEFFS_BASE_OFFSET-2*3
    let x17 = COEFFS.as_ptr().add(COEFFS_BASE_OFFSET - 3);
    // mvni v30.4h, #0x3f  // 0xffc0
    v[30] = dup(false, 0xffc0);
    // ldrh w9, [x1, #6]  // count = cdf[n_symbols]
    let mut w9 = core::ptr::read(x1.add(3)) as u32;
    // ld1r {v3.4h}, [x16]  // rng
    v[3] = dup(false, core::ptr::read((&raw const (*s).rng).cast::<u16>()));
    // ld1 {v29.4h}, [x17]  // EC_MIN_PROB * (n_symbols - ret)
    v[29] = ld(false, x17);
    // add x17, x0, #DIF + 6
    // mov w13, #-24*8
    let mut w13 = (-24i32 * 8) as u32;
    // and v17.8b, v0.8b, v30.8b  // cdf & 0xffc0
    v[17] = and(false, v[0], v[30]);
    // ldr w10, [x0, #ALLOW_UPDATE_CDF]
    let w10 = (*s).allow_update_cdf as u32;
    // ld1r {v1.8h}, [x17]  // dif >> (EC_WIN_SIZE - 16)
    v[1] = dup(true, ((*s).dif as u64 >> 48) as u16);
    // ldr w6, [x0, #CNT] ; ldr x7, [x0, #DIF]
    let mut w6 = (*s).cnt as u32;
    let mut x7 = (*s).dif as u64;
    loop {
        // 1:
        // and v7.8b, v3.8b, v31.8b  // rng & 0x7f00
        v[7] = and(false, v[3], v[31]);
        // sqdmulh v6.4h, v17.4h, v7.4h  // ((cdf >> EC_PROB_SHIFT) * (r - 128)) >> 1
        v[6] = sqdmulh(false, v[17], v[7]);
        // add v4.4h, v17.4h, v29.4h  // v = cdf + EC_MIN_PROB * (n_symbols - ret)
        v[4] = add(false, v[17], v[29]);
        // add v4.4h, v6.4h, v4.4h
        v[4] = add(false, v[6], v[4]);
        // cmhs v2.4h, v1.4h, v4.4h  // c >= v
        v[2] = cmhs(false, v[1], v[4]);
        // add w13, w13, #5*8
        w13 = w13.wrapping_add(5 * 8);
        // ext v18.8b, v3.8b, v4.8b, #6  // u
        v[18] = d(vext_u16::<3>(lo(v[3]), lo(v[4])));
        // umov x15, v2.d[0] ; rev x15, x15
        let mut x15 = d0(v[2]).swap_bytes();
        // sub v18.4h, v18.4h, v4.4h  // rng = u-v
        v[18] = sub(false, v[18], v[4]);
        // clz x15, x15  // 16*ret
        x15 = x15.leading_zeros() as u64;

        // cbz w10, 2f
        if w10 != 0 {
            // update_cdf
            // sub v5.4h, v0.4h, v2.4h  // cdf[i] + (i >= val ? 1 : 0)
            v[5] = sub(false, v[0], v[2]);
            // mov w4, #-5
            let mut w4 = -5i32 as u32;
            // orr v2.4h, #0x80, lsl #8  // i >= val ? -1 : 32768
            v[2] = orr8000(false, v[2]);
            // sub w4, w4, w9, lsr #4  // -((count >> 4) + 5)
            w4 = w4.wrapping_sub(w9 >> 4);
            // sub v2.4h, v2.4h, v0.4h  // (32768 - cdf[i]) or (-1 - cdf[i])
            v[2] = sub(false, v[2], v[0]);
            // dup v6.4h, w4  // -rate
            v[6] = dup(false, w4 as u16);
            // sub w9, w9, w9, lsr #5  // count - (count == 32)
            w9 = w9.wrapping_sub(w9 >> 5);
            // sshl v2.4h, v2.4h, v6.4h  // ({32768,-1} - cdf[i]) >> rate
            v[2] = sshl(false, v[2], v[6]);
            // add w9, w9, #1  // count + (count < 32)
            w9 = w9.wrapping_add(1);
            // add v0.4h, v5.4h, v2.4h  // cdf[i] + (32768 - cdf[i]) >> rate
            v[0] = add(false, v[5], v[2]);
            // st1 {v0.4h}, [x1]
            st(false, x1, v[0]);
            // and v17.8b, v0.8b, v30.8b  // cdf & 0xffc0
            v[17] = and(false, v[0], v[30]);
            // strh w9, [x1, #6]
            core::ptr::write(x1.add(3), w9 as u16);
        }

        // 2:
        // mov x4, v18.d[0]  // rng (packed) ; mov x3, v4.d[0]  // v (packed)
        // lsr x4, x4, x15 ; lsr x3, x3, x15
        let x4 = d0(v[18]) >> (x15 & 63);
        let x3 = d0(v[4]) >> (x15 & 63);
        let mut w4 = x4 as u32;
        // lsl w5, w4, #16  // rng << 16
        // sub x7, x7, x3, lsl #48  // dif - (v << 48)
        x7 = x7.wrapping_sub(x3 << 48);
        // clz w5, w5  // d = clz(rng << 16)
        let w5 = (w4 << 16).leading_zeros();
        // lsl w4, w4, w5  // rng << d
        w4 <<= w5 & 31;
        // subs w6, w6, w5  // cnt -= d
        let lo_ = w6 < w5;
        w6 = w6.wrapping_sub(w5);
        // lsl x7, x7, x5  // (dif - (v << 48)) << d
        x7 <<= (w5 as u64) & 63;
        // strh w4, [x0, #RNG]
        strh_rng(s, w4);
        // dup v3.4h, w4
        v[3] = dup(false, w4 as u16);
        // b.hs 5f
        if lo_ {
            // refill (3, 4, 6, 7)
            (w6, x7) = refill(s, w6, x7);
        }
        // 5: end
        // sub w15, w15, #5*8
        let w15 = (x15 as u32).wrapping_sub(5 * 8);
        // lsr x12, x7, #48
        let x12 = x7 >> 48;
        // adds w13, w13, w15  // carry = tok_br < 3 || tok == 15
        let carry = (w13 as u64) + (w15 as u64) > u32::MAX as u64;
        w13 = w13.wrapping_add(w15);
        // dup v1.8h, w12
        v[1] = dup(true, x12 as u16);
        // b.cc 1b  // loop if !carry
        if carry {
            break;
        }
    }
    // add w13, w13, #30*8
    w13 = w13.wrapping_add(30 * 8);
    // str w6, [x0, #CNT] ; str x7, [x0, #DIF]
    (*s).cnt = w6 as c_int;
    (*s).dif = x7 as usize;
    // lsr w0, w13, #4
    w13 >> 4
}

/// The renormalization that `bool_equi`, `bool` and `bool_adapt` end with:
/// `w4` the new range, `w5` = d, `w6` cnt, `x7` dif, `w15` the bit.
#[inline(always)]
unsafe fn bool_renorm(s: *mut MsacContext, mut w4: u32, w5: u32, mut w6: u32, mut x7: u64, w15: u32) -> c_uint {
    // lsl w4, w4, w5  // rng << d
    w4 <<= w5 & 31;
    // subs w6, w6, w5  // cnt -= d
    let lo = w6 < w5;
    w6 = w6.wrapping_sub(w5);
    // lsl x7, x7, x5  // (dif - (v << 48)) << d
    x7 <<= (w5 as u64) & 63;
    // str w4, [x0, #RNG]
    (*s).rng = w4;
    // b.lo L(refill)
    if lo {
        return refill_end(s, w6, x7, w15);
    }
    // str w6, [x0, #CNT] ; str x7, [x0, #DIF] ; mov w0, w15
    (*s).cnt = w6 as c_int;
    (*s).dif = x7 as usize;
    w15
}

/// `unsigned dav1d_msac_decode_bool_equi_neon(MsacContext *s)`
pub unsafe extern "C" fn dav1d_msac_decode_bool_equi_neon(s: *mut MsacContext) -> c_uint {
    // ldp w5, w6, [x0, #RNG]  // + CNT
    let mut w5 = (*s).rng;
    let w6 = (*s).cnt as u32;
    // ldr x7, [x0, #DIF]
    let mut x7 = (*s).dif as u64;
    // bic w4, w5, #0xff  // r &= 0xff00
    let mut w4 = w5 & !0xff;
    // add w4, w4, #8
    w4 = w4.wrapping_add(8);
    // subs x8, x7, x4, lsl #47  // dif - vw
    let vw = (w4 as u64) << 47;
    let x8 = x7.wrapping_sub(vw);
    let hs = x7 >= vw;
    // lsr w4, w4, #1  // v
    w4 >>= 1;
    // sub w5, w5, w4  // r - v
    w5 = w5.wrapping_sub(w4);
    // cset w15, lo
    let w15 = !hs as u32;
    // csel w4, w5, w4, hs  // if (ret) v = r - v;
    if hs {
        w4 = w5;
        // csel x7, x8, x7, hs  // if (ret) dif = dif - vw;
        x7 = x8;
    }
    // clz w5, w4 ; eor w5, w5, #16  // d = clz(rng) ^ 16
    let w5 = w4.leading_zeros() ^ 16;
    bool_renorm(s, w4, w5, w6, x7, w15)
}

/// `unsigned dav1d_msac_decode_bool_neon(MsacContext *s, unsigned f)`
pub unsafe extern "C" fn dav1d_msac_decode_bool_neon(s: *mut MsacContext, w1: c_uint) -> c_uint {
    // ldp w5, w6, [x0, #RNG]  // + CNT
    let mut w5 = (*s).rng;
    let w6 = (*s).cnt as u32;
    // ldr x7, [x0, #DIF]
    let mut x7 = (*s).dif as u64;
    // lsr w4, w5, #8  // r >> 8
    let mut w4 = w5 >> 8;
    // bic w1, w1, #0x3f  // f &= ~63
    let w1 = w1 & !0x3f;
    // mul w4, w4, w1
    w4 = w4.wrapping_mul(w1);
    // lsr w4, w4, #7
    w4 >>= 7;
    // add w4, w4, #4  // v
    w4 = w4.wrapping_add(4);
    // subs x8, x7, x4, lsl #48  // dif - vw
    let vw = (w4 as u64) << 48;
    let x8 = x7.wrapping_sub(vw);
    let hs = x7 >= vw;
    // sub w5, w5, w4  // r - v
    w5 = w5.wrapping_sub(w4);
    // cset w15, lo
    let w15 = !hs as u32;
    // csel w4, w5, w4, hs ; csel x7, x8, x7, hs
    if hs {
        w4 = w5;
        x7 = x8;
    }
    // clz w5, w4 ; eor w5, w5, #16  // d = clz(rng) ^ 16
    let w5 = w4.leading_zeros() ^ 16;
    bool_renorm(s, w4, w5, w6, x7, w15)
}

/// `unsigned dav1d_msac_decode_bool_adapt_neon(MsacContext *s, uint16_t *cdf)`
pub unsafe extern "C" fn dav1d_msac_decode_bool_adapt_neon(s: *mut MsacContext, x1: *mut u16) -> c_uint {
    // ldr w9, [x1]  // cdf[0-1]
    let mut w9 = core::ptr::read_unaligned(x1.cast::<u32>());
    // ldp w5, w6, [x0, #RNG]  // + CNT
    let mut w5 = (*s).rng;
    let w6 = (*s).cnt as u32;
    // ldr x7, [x0, #DIF]
    let mut x7 = (*s).dif as u64;
    // lsr w4, w5, #8  // r >> 8
    let mut w4 = w5 >> 8;
    // and w2, w9, #0xffc0  // f &= ~63
    let w2 = w9 & 0xffc0;
    // mul w4, w4, w2 ; lsr w4, w4, #7 ; add w4, w4, #4  // v
    w4 = (w4.wrapping_mul(w2) >> 7).wrapping_add(4);
    // subs x8, x7, x4, lsl #48  // dif - vw
    let vw = (w4 as u64) << 48;
    let x8 = x7.wrapping_sub(vw);
    let hs = x7 >= vw;
    // sub w5, w5, w4  // r - v
    w5 = w5.wrapping_sub(w4);
    // cset w15, lo
    let w15 = !hs as u32;
    // csel w4, w5, w4, hs ; csel x7, x8, x7, hs
    if hs {
        w4 = w5;
        x7 = x8;
    }
    // ldr w10, [x0, #ALLOW_UPDATE_CDF]
    let w10 = (*s).allow_update_cdf as u32;
    // clz w5, w4 ; eor w5, w5, #16  // d = clz(rng) ^ 16
    let w5 = w4.leading_zeros() ^ 16;
    // cbz w10, 1f
    if w10 != 0 {
        // lsr w2, w9, #16  // count = cdf[1]
        let mut w2 = w9 >> 16;
        // and w9, w9, #0xffff  // cdf[0]
        w9 &= 0xffff;
        // sub w3, w2, w2, lsr #5  // count - (count >= 32)
        let w3 = w2.wrapping_sub(w2 >> 5);
        // lsr w2, w2, #4  // count >> 4
        w2 >>= 4;
        // add w10, w3, #1  // count + (count < 32)
        let w10 = w3.wrapping_add(1);
        // add w2, w2, #4  // rate = (count >> 4) | 4
        w2 = w2.wrapping_add(4);
        // sub w9, w9, w15  // cdf[0] -= bit
        w9 = w9.wrapping_sub(w15);
        // sub w11, w9, w15, lsl #15  // {cdf[0], cdf[0] - 32769}
        let mut w11 = w9.wrapping_sub(w15 << 15);
        // asr w11, w11, w2  // {cdf[0], cdf[0] - 32769} >> rate
        w11 = ((w11 as i32) >> (w2 & 31)) as u32;
        // sub w9, w9, w11  // cdf[0]
        w9 = w9.wrapping_sub(w11);
        // strh w9, [x1] ; strh w10, [x1, #2]
        core::ptr::write(x1, w9 as u16);
        core::ptr::write(x1.add(1), w10 as u16);
    }
    // 1:
    bool_renorm(s, w4, w5, w6, x7, w15)
}
