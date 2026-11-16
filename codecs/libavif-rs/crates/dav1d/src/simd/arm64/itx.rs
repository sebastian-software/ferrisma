// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/itx.S` (8 bpc) with NEON
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! First slice: the DCT_DCT and IDTX entry points for every size up to
//! 16x16 with the 1-D transforms they use (`idct_4/8/16` and the identity
//! transforms in both arrangements). The other entry points stay on the
//! translated C until they are rebuilt.
//!
//! Correspondence with the assembly:
//! - Each exported `function` is one `extern "C"` function of the same name;
//!   each internal `function` and each `.macro` is a Rust function of the
//!   same name. `blr x4`/`blr x5` become calls of the 1-D transform the
//!   entry point loaded into that register.
//! - The register file is `v: V` (`util.rs`), indexed by register number.
//!   The scratch registers `v2`–`v7` that the assembly only uses inside a
//!   macro are locals there. `v0`/`v1` hold constants that the assembly
//!   loads from `idct_coeffs` or creates with `dup`; the helpers take those
//!   lane values directly (same values).
//! - `smull`/`smlal` pairs and their `sqrshrn #12` are `mul2`; a 64-bit
//!   arrangement (`.4h`) is `Q = false` and clears the high half.
//! - The general-purpose registers (`x0` dst, `x1` stride, `x2` coeff,
//!   `w3` eob, `x13` eob threshold) are function arguments; post-increment
//!   addressing becomes explicit offsets.
//!
//! Safety (argued once for the family): the translated caller `recon_tmpl`
//! passes what the original passes to the assembly: `dst` points to a
//! block of `h` rows of `w` writable pixels, `stride` bytes apart, and
//! `coeff` to the block's coefficient buffer of at least `w * h` (capped at
//! 32x32) 16-bit values, both live for the call and not otherwise accessed
//! during it. Every load and store below stays inside those two regions:
//! pixel accesses are rows `0..h` and columns `0..w` of `dst`, coefficient
//! accesses are offsets `0..w*h` of `coeff`.
#![allow(clippy::all, non_snake_case)]

use core::arch::aarch64::*;
use core::ffi::c_int;

use super::util::*;

type Dst = *mut u8;
type Coef = *mut i16;

// idct_coeffs: idct4 .short 2896, 2896*8, 1567, 3784; idct8 799, 4017,
// 3406, 2276 (v0.h[0..8]); idct16 401, 4076, 3166, 2598, 1931, 3612, 3920,
// 1189 (v1.h[0..8]).
const C0: [i16; 8] = [2896, 2896 * 8, 1567, 3784, 799, 4017, 3406, 2276];
const C1: [i16; 8] = [401, 4076, 3166, 2598, 1931, 3612, 3920, 1189];

// --- memory access ---------------------------------------------------------

/// `ld1 {vN.8h}, [p]`
#[inline(always)]
unsafe fn ld8h(p: *const i16) -> int16x8_t {
    vld1q_s16(p)
}

/// `ld1 {vN.4h}, [p]` (clears the high half)
#[inline(always)]
unsafe fn ld4h(p: *const i16) -> int16x8_t {
    d(vld1_s16(p))
}

/// `st1 {vZ.8h}, [p]` with a zero register
#[inline(always)]
unsafe fn zero8(p: *mut i16) {
    vst1q_s16(p, vdupq_n_s16(0));
}

/// `st1 {vZ.4h}, [p]` with a zero register
#[inline(always)]
unsafe fn zero4(p: *mut i16) {
    vst1_s16(p, vdup_n_s16(0));
}

#[inline(always)]
unsafe fn row(dst: Dst, stride: isize, y: isize) -> Dst {
    dst.offset(y * stride)
}

/// One row of `load_add_store`: `srshr #shift`, `uaddw` with the 8 pixels
/// at `p`, `sqxtun`, store.
#[inline(always)]
unsafe fn add_store8<const SHIFT: i32>(p: Dst, x: int16x8_t) {
    let px = vld1_u8(p);
    let s = shift_by::<SHIFT>(x);
    let sum = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(s), px));
    vst1_u8(p, vqmovun_s16(sum));
}

/// Two rows of four pixels (`load_add_store4`/`itx_4x4_end`): `x` holds row
/// `p` in its low half and row `p + stride` in its high half; `srshr #shift`,
/// `uaddw`, `sqxtun`, store. The `.s` lane accesses carry no alignment
/// requirement, so they are unaligned 32-bit reads and writes.
#[inline(always)]
unsafe fn add_store4x2<const SHIFT: i32>(p: Dst, stride: isize, x: int16x8_t) {
    let q = p.offset(stride);
    let a = (p as *const u32).read_unaligned() as u64;
    let b = (q as *const u32).read_unaligned() as u64;
    let px = vcreate_u8(a | (b << 32));
    let s = shift_by::<SHIFT>(x);
    let sum = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(s), px));
    let out = vreinterpret_u32_u8(vqmovun_s16(sum));
    (p as *mut u32).write_unaligned(vget_lane_u32::<0>(out));
    (q as *mut u32).write_unaligned(vget_lane_u32::<1>(out));
}

/// `srshr #SHIFT` for the shifts the kernels use; no instruction for 0.
#[inline(always)]
fn shift_by<const SHIFT: i32>(x: int16x8_t) -> int16x8_t {
    match SHIFT {
        0 => x,
        1 => srshr::<1>(x),
        2 => srshr::<2>(x),
        4 => srshr::<4>(x),
        _ => unreachable!(),
    }
}

// --- idct_dc ---------------------------------------------------------------

/// The `idct_dc w, h, shift` macro when `eob == 0`, followed by
/// `idct_dc_w<w>_neon`.
#[inline(always)]
unsafe fn idct_dc<const W: usize, const H: usize, const SHIFT: i32>(dst: Dst, stride: isize, coeff: Coef) {
    // ld1r {v16.8h}, [x2]; strh wzr, [x2]
    let mut v16 = vld1q_dup_s16(coeff);
    *coeff = 0;
    v16 = sqrdmulh::<true>(v16, 2896 * 8);
    if W == 2 * H || 2 * W == H {
        v16 = sqrdmulh::<true>(v16, 2896 * 8);
    }
    v16 = shift_by::<SHIFT>(v16);
    v16 = sqrdmulh::<true>(v16, 2896 * 8);
    v16 = srshr::<4>(v16);
    idct_dc_w::<W>(dst, stride, H, v16);
}

/// `idct_dc_w4_neon` … `idct_dc_w64_neon`: add `v16` to `h` rows of `w`
/// pixels.
#[inline(always)]
unsafe fn idct_dc_w<const W: usize>(dst: Dst, stride: isize, h: usize, v16: int16x8_t) {
    for y in 0..h as isize {
        let p = row(dst, stride, y);
        match W {
            4 => {
                let px = vcreate_u8((p as *const u32).read_unaligned() as u64);
                let sum = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(v16), px));
                let out = vreinterpret_u32_u8(vqmovun_s16(sum));
                (p as *mut u32).write_unaligned(vget_lane_u32::<0>(out));
            }
            8 => {
                let sum = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(v16), vld1_u8(p)));
                vst1_u8(p, vqmovun_s16(sum));
            }
            _ => {
                let mut x = 0;
                while x < W {
                    let px = vld1q_u8(p.add(x));
                    let a = vaddw_u8(vreinterpretq_u16_s16(v16), vget_low_u8(px));
                    let b = vaddw_high_u8(vreinterpretq_u16_s16(v16), px);
                    let out =
                        vqmovun_high_s16(vqmovun_s16(vreinterpretq_s16_u16(a)), vreinterpretq_s16_u16(b));
                    vst1q_u8(p.add(x), out);
                    x += 16;
                }
            }
        }
    }
}

// --- 1-D transforms --------------------------------------------------------

/// `idct_4 r0, r1, r2, r3, sz`
#[inline(always)]
fn idct_4<const Q: bool>(v: &mut V, r0: usize, r1: usize, r2: usize, r3: usize) {
    let v6 = mul2::<Q, false, false>(v[r1], v[r3], C0[3], C0[2]);
    let v7 = mul2::<Q, true, false>(v[r1], v[r3], C0[2], C0[3]);
    let v2 = mul2::<Q, false, false>(v[r0], v[r2], C0[0], C0[0]);
    let v3 = mul2::<Q, true, false>(v[r0], v[r2], C0[0], C0[0]);
    v[r0] = sqadd::<Q>(v2, v6);
    v[r3] = sqsub::<Q>(v2, v6);
    v[r1] = sqadd::<Q>(v3, v7);
    v[r2] = sqsub::<Q>(v3, v7);
}

/// `idct_8 r0, …, r7, sz, szb`
#[inline(always)]
fn idct_8<const Q: bool>(v: &mut V, r: [usize; 8]) {
    let [r0, r1, r2, r3, r4, r5, r6, r7] = r;
    idct_4::<Q>(v, r0, r2, r4, r6);

    let t4a = mul2::<Q, true, false>(v[r1], v[r7], C0[4], C0[5]);
    let t7a = mul2::<Q, false, false>(v[r1], v[r7], C0[5], C0[4]);
    let t5a = mul2::<Q, true, false>(v[r5], v[r3], C0[6], C0[7]);
    let t6a = mul2::<Q, false, false>(v[r5], v[r3], C0[7], C0[6]);
    v[r1] = t4a;
    v[r7] = t7a;
    v[r3] = t5a;
    v[r5] = t6a;

    let v2 = sqadd::<Q>(v[r1], v[r3]); // t4
    v[r1] = sqsub::<Q>(v[r1], v[r3]); // t5a
    let v3 = sqadd::<Q>(v[r7], v[r5]); // t7
    v[r3] = sqsub::<Q>(v[r7], v[r5]); // t6a

    let v4 = mul2::<Q, true, false>(v[r3], v[r1], C0[0], C0[0]); // t5
    let v5 = mul2::<Q, false, false>(v[r3], v[r1], C0[0], C0[0]); // t6

    v[r7] = sqsub::<Q>(v[r0], v3); // out7
    v[r0] = sqadd::<Q>(v[r0], v3); // out0
    v[r1] = sqadd::<Q>(v[r2], v5); // out1
    let v6 = sqsub::<Q>(v[r2], v5); // out6
    v[r2] = sqadd::<Q>(v[r4], v4); // out2
    v[r5] = sqsub::<Q>(v[r4], v4); // out5
    v[r3] = sqadd::<Q>(v[r6], v2); // out3
    v[r4] = sqsub::<Q>(v[r6], v2); // out4
    v[r6] = v6; // out6
}

/// `idct_16 sz, szb` on v16–v31.
#[inline(always)]
fn idct_16<const Q: bool>(v: &mut V) {
    idct_8::<Q>(v, [16, 18, 20, 22, 24, 26, 28, 30]);

    let t8a = mul2::<Q, true, false>(v[17], v[31], C1[0], C1[1]);
    let t15a = mul2::<Q, false, false>(v[17], v[31], C1[1], C1[0]);
    let t9a = mul2::<Q, true, false>(v[25], v[23], C1[2], C1[3]);
    let t14a = mul2::<Q, false, false>(v[25], v[23], C1[3], C1[2]);
    let t10a = mul2::<Q, true, false>(v[21], v[27], C1[4], C1[5]);
    let t13a = mul2::<Q, false, false>(v[21], v[27], C1[5], C1[4]);
    let t11a = mul2::<Q, true, false>(v[29], v[19], C1[6], C1[7]);
    let t12a = mul2::<Q, false, false>(v[29], v[19], C1[7], C1[6]);
    v[17] = t8a;
    v[31] = t15a;
    v[23] = t9a;
    v[25] = t14a;
    v[21] = t10a;
    v[27] = t13a;
    v[19] = t11a;
    v[29] = t12a;

    let v2 = sqsub::<Q>(v[17], v[23]); // t9
    v[17] = sqadd::<Q>(v[17], v[23]); // t8
    let v3 = sqsub::<Q>(v[31], v[25]); // t14
    v[31] = sqadd::<Q>(v[31], v[25]); // t15
    v[23] = sqsub::<Q>(v[19], v[21]); // t10
    v[19] = sqadd::<Q>(v[19], v[21]); // t11
    v[25] = sqadd::<Q>(v[29], v[27]); // t12
    v[29] = sqsub::<Q>(v[29], v[27]); // t13

    v[21] = mul2::<Q, true, false>(v3, v2, C0[2], C0[3]); // t9a
    v[27] = mul2::<Q, false, false>(v3, v2, C0[3], C0[2]); // t14a

    let t13a = mul2::<Q, true, false>(v[29], v[23], C0[2], C0[3]);
    let t10a = mul2::<Q, false, true>(v[29], v[23], C0[3], C0[2]); // neg v6.4s/v7.4s
    v[29] = t13a;
    v[23] = t10a;

    let v2 = sqsub::<Q>(v[17], v[19]); // t11a
    v[17] = sqadd::<Q>(v[17], v[19]); // t8a
    let v3 = sqsub::<Q>(v[31], v[25]); // t12a
    v[31] = sqadd::<Q>(v[31], v[25]); // t15a
    v[19] = sqadd::<Q>(v[21], v[23]); // t9
    v[21] = sqsub::<Q>(v[21], v[23]); // t10
    v[25] = sqsub::<Q>(v[27], v[29]); // t13
    v[27] = sqadd::<Q>(v[27], v[29]); // t14

    let v4 = mul2::<Q, true, false>(v3, v2, C0[0], C0[0]); // t11
    let v5 = mul2::<Q, false, false>(v3, v2, C0[0], C0[0]); // t12
    let v2 = mul2::<Q, true, false>(v[25], v[21], C0[0], C0[0]); // t10a
    let v3 = mul2::<Q, false, false>(v[25], v[21], C0[0], C0[0]); // t13a

    let v6 = sqadd::<Q>(v[16], v[31]); // out0
    v[31] = sqsub::<Q>(v[16], v[31]); // out15
    v[16] = v6;
    v[23] = sqadd::<Q>(v[30], v[17]); // out7
    let v7 = sqsub::<Q>(v[30], v[17]); // out8
    v[17] = sqadd::<Q>(v[18], v[27]); // out1
    v[30] = sqsub::<Q>(v[18], v[27]); // out14
    v[18] = sqadd::<Q>(v[20], v3); // out2
    v[29] = sqsub::<Q>(v[20], v3); // out13
    let v3 = sqadd::<Q>(v[28], v[19]); // out6
    v[25] = sqsub::<Q>(v[28], v[19]); // out9
    v[19] = sqadd::<Q>(v[22], v5); // out3
    v[28] = sqsub::<Q>(v[22], v5); // out12
    v[20] = sqadd::<Q>(v[24], v4); // out4
    v[27] = sqsub::<Q>(v[24], v4); // out11
    v[21] = sqadd::<Q>(v[26], v2); // out5
    v[26] = sqsub::<Q>(v[26], v2); // out10
    v[24] = v7;
    v[22] = v3;
}

/// The 1-D transforms an entry point loads into `x4`/`x5`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Tx {
    /// `inv_dct_8h_x4_neon`, `inv_dct_4h_x4_neon`
    DctX4,
    /// `inv_dct_8h_x8_neon`, `inv_dct_4h_x8_neon`
    DctX8,
    /// `inv_dct_8h_x16_neon`, `inv_dct_4h_x16_neon`
    DctX16,
    /// `inv_identity_8h_x4_neon`, `inv_identity_4h_x4_neon`
    IdX4,
    /// `inv_identity_8h_x8_neon`, `inv_identity_4h_x8_neon`
    IdX8,
    /// `inv_identity_8h_x16_neon`, `inv_identity_4h_x16_neon`
    IdX16,
}

/// `blr` to `inv_<tx>_8h_<n>_neon` (`Q`) or `inv_<tx>_4h_<n>_neon`.
#[inline(always)]
fn call<const Q: bool>(tx: Tx, v: &mut V) {
    match tx {
        Tx::DctX4 => idct_4::<Q>(v, 16, 17, 18, 19),
        Tx::DctX8 => idct_8::<Q>(v, [16, 17, 18, 19, 20, 21, 22, 23]),
        Tx::DctX16 => idct_16::<Q>(v),
        Tx::IdX4 => {
            // mov w16, #(5793-4096)*8; sqrdmulh; sqadd
            for i in 16..20 {
                let t = sqrdmulh::<Q>(v[i], (5793 - 4096) * 8);
                v[i] = sqadd::<Q>(v[i], t);
            }
        }
        Tx::IdX8 => {
            for i in 16..24 {
                v[i] = sqshl1::<Q>(v[i]);
            }
        }
        Tx::IdX16 => {
            // mov w16, #2*(5793-4096)*8; sqrdmulh v2; sqadd vi, vi, vi; sqadd vi, vi, v2
            for i in 16..32 {
                let v2 = sqrdmulh::<Q>(v[i], 2 * (5793 - 4096) * 8);
                v[i] = sqadd::<Q>(v[i], v[i]);
                v[i] = sqadd::<Q>(v[i], v2);
            }
        }
    }
}

/// `scale_input .8h/.4h, v0.h[0] (2896*8), …` on the listed registers.
#[inline(always)]
fn scale_input<const Q: bool>(v: &mut V, regs: core::ops::Range<usize>) {
    for i in regs {
        v[i] = sqrdmulh::<Q>(v[i], 2896 * 8);
    }
}

// --- 4x4 -------------------------------------------------------------------

/// `inv_txfm_add_4x4_neon` with `L(itx_4x4_end)`.
#[inline(always)]
unsafe fn inv_txfm_add_4x4(dst: Dst, stride: isize, coeff: Coef, x4: Tx, x5: Tx) {
    let mut v = regs();
    v[16] = ld4h(coeff);
    v[17] = ld4h(coeff.add(4));
    v[18] = ld4h(coeff.add(8));
    v[19] = ld4h(coeff.add(12));
    zero8(coeff);
    call::<false>(x4, &mut v);
    zero8(coeff.add(8));
    transpose_4x4h(&mut v, [16, 17, 18, 19], [20, 21, 22, 23]);
    call::<false>(x5, &mut v);
    v[16] = ins_hi_lo(v[16], v[17]);
    v[18] = ins_hi_lo(v[18], v[19]);
    add_store4x2::<4>(dst, stride, v[16]);
    add_store4x2::<4>(row(dst, stride, 2), stride, v[18]);
}

/// `def_fn_4x4 dct, dct`: the DC-only path for `eob == 0`.
#[inline(always)]
unsafe fn idct_dc_4x4(dst: Dst, stride: isize, coeff: Coef) {
    let mut v16 = vld1q_dup_s16(coeff);
    *coeff = 0;
    v16 = sqrdmulh::<true>(v16, 2896 * 8);
    let v20 = sqrdmulh::<true>(v16, 2896 * 8);
    let v16 = srshr::<4>(v20);
    let v18 = srshr::<4>(v20);
    // L(itx_4x4_end) without the srshr of the general path.
    add_store4x2::<0>(dst, stride, v16);
    add_store4x2::<0>(row(dst, stride, 2), stride, v18);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc_4x4(dst, stride, coeff);
    }
    inv_txfm_add_4x4(dst, stride, coeff, Tx::DctX4, Tx::DctX4);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    inv_txfm_add_4x4(dst, stride, coeff, Tx::IdX4, Tx::IdX4);
}

// --- 8x8 -------------------------------------------------------------------

/// `inv_txfm_add_8x8_neon` and `inv_txfm_identity_add_8x8_neon`
/// (`x4 == None`).
#[inline(always)]
unsafe fn inv_txfm_add_8x8(dst: Dst, stride: isize, coeff: Coef, x4: Option<Tx>, x5: Tx) {
    let mut v = regs();
    for i in 0..4 {
        v[16 + i] = ld8h(coeff.add(8 * i));
    }
    for i in 0..4 {
        zero8(coeff.add(8 * i));
    }
    for i in 0..4 {
        v[20 + i] = ld8h(coeff.add(32 + 8 * i));
    }
    for i in 0..4 {
        zero8(coeff.add(32 + 8 * i));
    }
    if let Some(x4) = x4 {
        call::<true>(x4, &mut v);
        for i in 16..24 {
            v[i] = srshr::<1>(v[i]);
        }
    }
    // The identity shl #1 and downshift srshr #1 cancel out.
    // L(itx_8x8_epilog):
    transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 24, 25);
    call::<true>(x5, &mut v);
    for i in 0..8 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<8, 8, 1>(dst, stride, coeff);
    }
    inv_txfm_add_8x8(dst, stride, coeff, Some(Tx::DctX8), Tx::DctX8);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    inv_txfm_add_8x8(dst, stride, coeff, None, Tx::IdX8);
}

// --- 8x4, 4x8 --------------------------------------------------------------

/// `inv_txfm_add_8x4_neon`: `x4` is `inv_*_4h_x8`, `x5` `inv_*_8h_x4`.
#[inline(always)]
unsafe fn inv_txfm_add_8x4(dst: Dst, stride: isize, coeff: Coef, x4: Tx, x5: Tx) {
    let mut v = regs();
    for i in 0..4 {
        v[16 + i] = ld4h(coeff.add(4 * i));
    }
    zero8(coeff);
    zero8(coeff.add(8));
    for i in 0..4 {
        v[20 + i] = ld4h(coeff.add(16 + 4 * i));
    }
    zero8(coeff.add(16));
    zero8(coeff.add(24));
    scale_input::<false>(&mut v, 16..24);
    call::<false>(x4, &mut v);
    transpose_4x4h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);
    transpose_4x4h(&mut v, [20, 21, 22, 23], [4, 5, 6, 7]);
    v[16] = ins_hi_lo(v[16], v[20]);
    v[17] = ins_hi_lo(v[17], v[21]);
    v[18] = ins_hi_lo(v[18], v[22]);
    v[19] = ins_hi_lo(v[19], v[23]);
    call::<true>(x5, &mut v);
    for i in 0..4 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// `inv_txfm_add_4x8_neon`: `x4` is `inv_*_8h_x4`, `x5` `inv_*_4h_x8`.
#[inline(always)]
unsafe fn inv_txfm_add_4x8(dst: Dst, stride: isize, coeff: Coef, x4: Tx, x5: Tx) {
    let mut v = regs();
    for i in 0..4 {
        v[16 + i] = ld8h(coeff.add(8 * i));
    }
    for i in 0..4 {
        zero8(coeff.add(8 * i));
    }
    scale_input::<true>(&mut v, 16..20);
    call::<true>(x4, &mut v);
    transpose_4x8h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);
    v[20] = ins_lo_hi(v[20], v[16]);
    v[21] = ins_lo_hi(v[21], v[17]);
    v[22] = ins_lo_hi(v[22], v[18]);
    v[23] = ins_lo_hi(v[23], v[19]);
    call::<false>(x5, &mut v);
    // load_add_store_4x8: rows 2k, 2k+1 from v(16+2k) | v(17+2k).
    for k in 0..4 {
        let x = ins_hi_lo(v[16 + 2 * k], v[17 + 2 * k]);
        add_store4x2::<4>(row(dst, stride, 2 * k as isize), stride, x);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<4, 8, 0>(dst, stride, coeff);
    }
    inv_txfm_add_4x8(dst, stride, coeff, Tx::DctX4, Tx::DctX8);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    inv_txfm_add_4x8(dst, stride, coeff, Tx::IdX4, Tx::IdX8);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<8, 4, 0>(dst, stride, coeff);
    }
    inv_txfm_add_8x4(dst, stride, coeff, Tx::DctX8, Tx::DctX4);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    inv_txfm_add_8x4(dst, stride, coeff, Tx::IdX8, Tx::IdX4);
}

// --- 16x16 -----------------------------------------------------------------

/// `inv_txfm_horz_16x8_neon` (`def_horz_16 shift=2`, `x4 == Some`) and
/// `inv_txfm_horz_identity_16x8_neon` (`x4 == None`): eight columns of
/// coefficients from `x7` (stride `x8` = 16 values) into 128 values at `x6`.
#[inline(always)]
unsafe fn horz_16x8(v: &mut V, x6: *mut i16, x7: Coef, x4: Option<Tx>) {
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(16 * i));
        zero8(x7.add(16 * i));
    }
    match x4 {
        None => {
            // identity_8x16_shift2 v0.h[0] = 2*(5793-4096)*8
            for i in 16..32 {
                let v2 = sqrdmulh::<true>(v[i], 2 * (5793 - 4096) * 8);
                let v2 = sshr::<1>(v2);
                v[i] = srhadd(v[i], v2);
            }
        }
        Some(x4) => {
            call::<true>(x4, v);
            for i in 16..32 {
                v[i] = srshr::<2>(v[i]);
            }
        }
    }
    // L(horz_16x8_epilog):
    transpose_8x8h(v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);
    transpose_8x8h(v, [24, 25, 26, 27, 28, 29, 30, 31], 4, 5);
    let order = [16, 24, 17, 25, 18, 26, 19, 27, 20, 28, 21, 29, 22, 30, 23, 31];
    for (k, &r) in order.iter().enumerate() {
        vst1q_s16(x6.add(8 * k), v[r]);
    }
}

/// `inv_txfm_add_vert_8x16_neon`: 16 rows of 8 values from `x7` (stride 16
/// values), `x5`, `load_add_store_8x16` to `x6`.
#[inline(always)]
unsafe fn vert_8x16(v: &mut V, x6: Dst, stride: isize, x7: *const i16, x5: Tx) {
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(16 * i));
    }
    call::<true>(x5, v);
    for i in 0..16 {
        add_store8::<4>(row(x6, stride, i as isize), v[16 + i]);
    }
}

/// `inv_txfm_add_16x16_neon` with `x9` = horizontal pass, `x13` = eob
/// threshold.
#[inline(always)]
unsafe fn inv_txfm_add_16x16(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    x4: Option<Tx>,
    x5: Tx,
    x13: c_int,
) {
    let mut v = regs();
    // sub sp, sp, #512
    let mut buf = [0i16; 256];
    let sp = buf.as_mut_ptr();
    for i in [0usize, 8] {
        let x6 = sp.add(i * 16);
        if i == 8 && eob < x13 {
            // movi v4–v7, #0; st1 ×4 (256 bytes)
            for k in 0..16 {
                zero8(x6.add(8 * k));
            }
            break;
        }
        horz_16x8(&mut v, x6, coeff.add(i), x4);
    }
    for i in [0usize, 8] {
        vert_8x16(&mut v, dst.add(i), stride, sp.add(i), x5);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<16, 16, 2>(dst, stride, coeff);
    }
    inv_txfm_add_16x16(dst, stride, coeff, eob, Some(Tx::DctX16), Tx::DctX16, 36);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    inv_txfm_add_16x16(dst, stride, coeff, eob, None, Tx::IdX16, 36);
}

// --- 16x4, 4x16 ------------------------------------------------------------

/// `inv_txfm_add_16x4_neon` (`x4 == Some`, `inv_*_4h_x16`) and
/// `inv_txfm_identity_add_16x4_neon`; `x5` is `inv_*_8h_x4`.
#[inline(always)]
unsafe fn inv_txfm_add_16x4(dst: Dst, stride: isize, coeff: Coef, x4: Option<Tx>, x5: Tx) {
    let mut v = regs();
    match x4 {
        None => {
            let mut p = coeff;
            for i in 16..20 {
                v[i] = ld4h(p);
                zero4(p);
                p = p.add(4);
            }
            for i in 16..20 {
                // ld1 {vi.d}[1], [x2]
                v[i] = vcombine_s16(lo(v[i]), vld1_s16(p));
                zero4(p);
                p = p.add(4);
            }
            for i in 20..24 {
                v[i] = ld4h(p);
                zero4(p);
                p = p.add(4);
            }
            for i in 20..24 {
                v[i] = vcombine_s16(lo(v[i]), vld1_s16(p));
                zero4(p);
                p = p.add(4);
            }
            // identity_8x16_shift1 v0.h[0] = 2*(5793-4096)*8 (on v16–v31;
            // only v16–v23 are used afterwards)
            for i in 16..32 {
                let v2 = sqrdmulh::<true>(v[i], 2 * (5793 - 4096) * 8);
                let v2 = srshr::<1>(v2);
                v[i] = sqadd::<true>(v[i], v2);
            }
        }
        Some(x4) => {
            for i in 0..16 {
                v[16 + i] = ld4h(coeff.add(4 * i));
                zero4(coeff.add(4 * i));
            }
            call::<false>(x4, &mut v);
            v[16] = ins_hi_lo(v[16], v[20]);
            v[17] = ins_hi_lo(v[17], v[21]);
            v[18] = ins_hi_lo(v[18], v[22]);
            v[19] = ins_hi_lo(v[19], v[23]);
            for i in 16..20 {
                v[i] = srshr::<1>(v[i]);
            }
            v[24] = ins_hi_lo(v[24], v[28]);
            v[25] = ins_hi_lo(v[25], v[29]);
            v[26] = ins_hi_lo(v[26], v[30]);
            v[27] = ins_hi_lo(v[27], v[31]);
            v[20] = srshr::<1>(v[24]);
            v[21] = srshr::<1>(v[25]);
            v[22] = srshr::<1>(v[26]);
            v[23] = srshr::<1>(v[27]);
        }
    }
    // L(itx_16x4_epilog):
    transpose_4x8h(&mut v, [16, 17, 18, 19], [2, 3, 4, 5]);
    call::<true>(x5, &mut v);
    for i in 0..4 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
    transpose_4x8h_mov(&mut v, [20, 21, 22, 23], [2, 3, 4, 5], [16, 17, 18, 19]);
    call::<true>(x5, &mut v);
    for i in 0..4 {
        add_store8::<4>(row(dst.add(8), stride, i as isize), v[16 + i]);
    }
}

/// `inv_txfm_add_4x16_neon` (`x4 == Some`, `inv_*_8h_x4`) and
/// `inv_txfm_identity_add_4x16_neon`; `x5` is `inv_*_4h_x16`, `w13` the eob
/// threshold.
#[inline(always)]
unsafe fn inv_txfm_add_4x16(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    x4: Option<Tx>,
    x5: Tx,
    w13: c_int,
) {
    let mut v = regs();
    if eob < w13 {
        // 1: movi v24.4h–v31.4h, #0
        for i in 24..32 {
            v[i] = vdupq_n_s16(0);
        }
    } else {
        let x6 = coeff.add(8);
        match x4 {
            None => {
                for i in 0..4 {
                    v[24 + i] = ld8h(x6.add(16 * i));
                    zero8(x6.add(16 * i));
                }
                // identity_8x4_shift1 v24–v27, v0.h[0] = (5793-4096)*8
                for i in 24..28 {
                    let v2 = sqrdmulh::<true>(v[i], (5793 - 4096) * 8);
                    v[i] = srhadd(v[i], v2);
                }
            }
            Some(x4) => {
                for i in 0..4 {
                    v[16 + i] = ld8h(x6.add(16 * i));
                    zero8(x6.add(16 * i));
                }
                call::<true>(x4, &mut v);
                v[24] = srshr::<1>(v[16]);
                v[25] = srshr::<1>(v[17]);
                v[26] = srshr::<1>(v[18]);
                v[27] = srshr::<1>(v[19]);
            }
        }
        transpose_4x8h(&mut v, [24, 25, 26, 27], [4, 5, 6, 7]);
        v[28] = ins_lo_hi(v[28], v[24]);
        v[29] = ins_lo_hi(v[29], v[25]);
        v[30] = ins_lo_hi(v[30], v[26]);
        v[31] = ins_lo_hi(v[31], v[27]);
    }
    // 2:
    for i in 0..4 {
        v[16 + i] = ld8h(coeff.add(16 * i));
        zero8(coeff.add(16 * i));
    }
    match x4 {
        None => {
            for i in 16..20 {
                let v2 = sqrdmulh::<true>(v[i], (5793 - 4096) * 8);
                v[i] = srhadd(v[i], v2);
            }
        }
        Some(x4) => {
            call::<true>(x4, &mut v);
            for i in 16..20 {
                v[i] = srshr::<1>(v[i]);
            }
        }
    }
    // L(itx_4x16_epilog):
    transpose_4x8h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);
    v[20] = ins_lo_hi(v[20], v[16]);
    v[21] = ins_lo_hi(v[21], v[17]);
    v[22] = ins_lo_hi(v[22], v[18]);
    v[23] = ins_lo_hi(v[23], v[19]);
    call::<false>(x5, &mut v);
    // load_add_store_4x16: rows 2k, 2k+1 from v(16+2k) | v(17+2k).
    for k in 0..8 {
        let x = ins_hi_lo(v[16 + 2 * k], v[17 + 2 * k]);
        add_store4x2::<4>(row(dst, stride, 2 * k as isize), stride, x);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<16, 4, 1>(dst, stride, coeff);
    }
    inv_txfm_add_16x4(dst, stride, coeff, Some(Tx::DctX16), Tx::DctX4);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    inv_txfm_add_16x4(dst, stride, coeff, None, Tx::IdX4);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<4, 16, 1>(dst, stride, coeff);
    }
    inv_txfm_add_4x16(dst, stride, coeff, eob, Some(Tx::DctX4), Tx::DctX16, 29);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    inv_txfm_add_4x16(dst, stride, coeff, eob, None, Tx::IdX16, 29);
}

// --- 16x8, 8x16 ------------------------------------------------------------

/// `inv_txfm_add_16x8_neon` (`x4 == Some`, `inv_*_8h_x16`) and
/// `inv_txfm_identity_add_16x8_neon`; `x5` is `inv_*_8h_x8`.
#[inline(always)]
unsafe fn inv_txfm_add_16x8(dst: Dst, stride: isize, coeff: Coef, x4: Option<Tx>, x5: Tx) {
    let mut v = regs();
    for i in 0..16 {
        v[16 + i] = ld8h(coeff.add(8 * i));
        zero8(coeff.add(8 * i));
    }
    scale_input::<true>(&mut v, 16..32);
    match x4 {
        None => {
            // identity_8x16_shift1 v0.h[0] = 2*(5793-4096)*8
            for i in 16..32 {
                let v2 = sqrdmulh::<true>(v[i], 2 * (5793 - 4096) * 8);
                let v2 = srshr::<1>(v2);
                v[i] = sqadd::<true>(v[i], v2);
            }
        }
        Some(x4) => {
            call::<true>(x4, &mut v);
            for i in 16..32 {
                v[i] = srshr::<1>(v[i]);
            }
        }
    }
    // L(itx_16x8_epilog):
    transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 2, 3);
    call::<true>(x5, &mut v);
    for i in 0..8 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
    transpose_8x8h_mov(
        &mut v,
        [24, 25, 26, 27, 28, 29, 30, 31],
        2,
        3,
        [16, 17, 18, 19, 20, 21, 22, 23],
    );
    call::<true>(x5, &mut v);
    for i in 0..8 {
        add_store8::<4>(row(dst.add(8), stride, i as isize), v[16 + i]);
    }
}

/// `inv_txfm_add_8x16_neon` (`x4 == Some`, `inv_*_8h_x8`) and
/// `inv_txfm_identity_add_8x16_neon`; `x5` is `inv_*_8h_x16`, `x13` the eob
/// threshold.
#[inline(always)]
unsafe fn inv_txfm_add_8x16(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    x4: Option<Tx>,
    x5: Tx,
    x13: c_int,
) {
    let mut v = regs();
    if eob < x13 {
        // 1: movi v24–v31, #0
        for i in 24..32 {
            v[i] = vdupq_n_s16(0);
        }
    } else {
        let x6 = coeff.add(8);
        match x4 {
            None => {
                for i in 0..8 {
                    v[24 + i] = ld8h(x6.add(16 * i));
                    zero8(x6.add(16 * i));
                }
                scale_input::<true>(&mut v, 24..32);
                // The identity shl #1 and downshift srshr #1 cancel out.
            }
            Some(x4) => {
                for i in 0..8 {
                    v[16 + i] = ld8h(x6.add(16 * i));
                    zero8(x6.add(16 * i));
                }
                scale_input::<true>(&mut v, 16..24);
                call::<true>(x4, &mut v);
                for i in 0..8 {
                    v[24 + i] = srshr::<1>(v[16 + i]);
                }
            }
        }
        transpose_8x8h(&mut v, [24, 25, 26, 27, 28, 29, 30, 31], 2, 3);
    }
    // 2:
    for i in 0..8 {
        v[16 + i] = ld8h(coeff.add(16 * i));
        zero8(coeff.add(16 * i));
    }
    scale_input::<true>(&mut v, 16..24);
    if let Some(x4) = x4 {
        call::<true>(x4, &mut v);
        for i in 16..24 {
            v[i] = srshr::<1>(v[i]);
        }
    }
    // L(itx_8x16_epilog):
    transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 2, 3);
    call::<true>(x5, &mut v);
    for i in 0..16 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<16, 8, 1>(dst, stride, coeff);
    }
    inv_txfm_add_16x8(dst, stride, coeff, Some(Tx::DctX16), Tx::DctX8);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    inv_txfm_add_16x8(dst, stride, coeff, None, Tx::IdX8);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    if eob == 0 {
        return idct_dc::<8, 16, 1>(dst, stride, coeff);
    }
    inv_txfm_add_8x16(dst, stride, coeff, eob, Some(Tx::DctX8), Tx::DctX16, 43);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    inv_txfm_add_8x16(dst, stride, coeff, eob, None, Tx::IdX16, 43);
}
