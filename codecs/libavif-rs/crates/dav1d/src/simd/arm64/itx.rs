// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/itx.S` (8 bpc) with NEON
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! All 156 entry points that `itx_dsp_init_arm` assigns at 8 bpc: every
//! transform type of every size (WHT, DCT, ADST, flipped ADST, identity,
//! the 32- and 64-point DCTs), with the 1-D transforms, the DC-only paths
//! and the eob thresholds of the assembly.
//!
//! Correspondence with the assembly:
//! - Each exported `function` is one `extern "C"` function of the same name;
//!   each internal `function` and each `.macro` is a Rust function of the
//!   same name. `blr x4`/`blr x5` become calls of the 1-D transform the
//!   entry point loaded into that register.
//! - The register file is `v: V` (`util.rs`), indexed by register number.
//!   The scratch registers `v2`–`v7` that the assembly only uses inside a
//!   macro are locals there. Where the assembly keeps a transform's
//!   intermediate values in registers that also hold its inputs and outputs
//!   (`iadst_8/16`, `inv_dct32_odd`, the 64-point steps), the values are
//!   named locals (`t17a`, …) as in the assembly's comments, and only the
//!   outputs are written back, to the registers the assembly leaves them in.
//! - The stack buffers of the assembly (`sub sp`) are local arrays. `v0`/`v1` hold constants that the assembly
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
    let px = ld_s2(p, stride);
    let s = shift_by::<SHIFT>(x);
    let sum = vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(s), px));
    st_s2(p, stride, vqmovun_s16(sum));
}

/// `srshr #SHIFT` for the shifts the kernels use; no instruction for 0.
#[inline(always)]
fn shift_by<const SHIFT: i32>(x: int16x8_t) -> int16x8_t {
    match SHIFT {
        0 => x,
        1 => srshr::<1>(x),
        2 => srshr::<2>(x),
        3 => srshr::<3>(x),
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

/// Two rows of four pixels into one 64-bit register:
/// `ld1 {vN.s}[0], [x0], x1; ld1 {vN.s}[1], [x0], x1`. The `.s` lane
/// accesses carry no alignment requirement (unaligned 32-bit reads).
#[inline(always)]
unsafe fn ld_s2(p: Dst, stride: isize) -> uint8x8_t {
    let a = (p as *const u32).read_unaligned();
    let b = (p.offset(stride) as *const u32).read_unaligned();
    vreinterpret_u8_u32(vset_lane_u32::<1>(b, vset_lane_u32::<0>(a, vdup_n_u32(0))))
}

/// `st1 {vN.s}[0], [x0], x1; st1 {vN.s}[1], [x0], x1`
#[inline(always)]
unsafe fn st_s2(p: Dst, stride: isize, x: uint8x8_t) {
    let x = vreinterpret_u32_u8(x);
    (p as *mut u32).write_unaligned(vget_lane_u32::<0>(x));
    (p.offset(stride) as *mut u32).write_unaligned(vget_lane_u32::<1>(x));
}

/// `uaddw vd.8h, v16.8h, vs.8b; sqxtun vd.8b, vd.8h`
#[inline(always)]
fn add_dc8(v16: int16x8_t, px: uint8x8_t) -> uint8x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vqmovun_s16(vreinterpretq_s16_u16(vaddw_u8(vreinterpretq_u16_s16(v16), px))) }
}

/// `uaddw`/`uaddw2` of 16 pixels and `sqxtun`/`sqxtun2`
#[inline(always)]
fn add_dc16(v16: int16x8_t, px: uint8x16_t) -> uint8x16_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let lo = vaddw_u8(vreinterpretq_u16_s16(v16), vget_low_u8(px));
        let hi = vaddw_high_u8(vreinterpretq_u16_s16(v16), px);
        vqmovun_high_s16(vqmovun_s16(vreinterpretq_s16_u16(lo)), vreinterpretq_s16_u16(hi))
    }
}

/// `idct_dc_w4_neon` … `idct_dc_w64_neon`: add `v16` to `h` (`w4`) rows of
/// `w` pixels, four rows per step for widths 4 to 16, two for 32, one for
/// 64, as the assembly's loops do.
#[inline(always)]
unsafe fn idct_dc_w<const W: usize>(dst: Dst, stride: isize, h: usize, v16: int16x8_t) {
    let mut x0 = dst;
    let mut w4 = h as isize;
    loop {
        match W {
            4 => {
                // 1: ld1 {v0.s}[0..1], {v1.s}[0..1]; uaddw; sqxtun; st1
                let v0 = ld_s2(x0, stride);
                let v1 = ld_s2(row(x0, stride, 2), stride);
                w4 -= 4;
                st_s2(x0, stride, add_dc8(v16, v0));
                st_s2(row(x0, stride, 2), stride, add_dc8(v16, v1));
                x0 = row(x0, stride, 4);
            }
            8 => {
                let v0 = vld1_u8(x0);
                let v1 = vld1_u8(row(x0, stride, 1));
                let v2 = vld1_u8(row(x0, stride, 2));
                let v3 = vld1_u8(row(x0, stride, 3));
                w4 -= 4;
                vst1_u8(x0, add_dc8(v16, v0));
                vst1_u8(row(x0, stride, 1), add_dc8(v16, v1));
                vst1_u8(row(x0, stride, 2), add_dc8(v16, v2));
                vst1_u8(row(x0, stride, 3), add_dc8(v16, v3));
                x0 = row(x0, stride, 4);
            }
            16 => {
                let v0 = vld1q_u8(x0);
                let v1 = vld1q_u8(row(x0, stride, 1));
                let v2 = vld1q_u8(row(x0, stride, 2));
                let v3 = vld1q_u8(row(x0, stride, 3));
                w4 -= 4;
                vst1q_u8(x0, add_dc16(v16, v0));
                vst1q_u8(row(x0, stride, 1), add_dc16(v16, v1));
                vst1q_u8(row(x0, stride, 2), add_dc16(v16, v2));
                vst1q_u8(row(x0, stride, 3), add_dc16(v16, v3));
                x0 = row(x0, stride, 4);
            }
            32 => {
                let r1 = row(x0, stride, 1);
                let (v0, v1) = (vld1q_u8(x0), vld1q_u8(x0.add(16)));
                w4 -= 2;
                let (v2, v3) = (vld1q_u8(r1), vld1q_u8(r1.add(16)));
                vst1q_u8(x0, add_dc16(v16, v0));
                vst1q_u8(x0.add(16), add_dc16(v16, v1));
                vst1q_u8(r1, add_dc16(v16, v2));
                vst1q_u8(r1.add(16), add_dc16(v16, v3));
                x0 = row(x0, stride, 2);
            }
            _ => {
                let v0 = vld1q_u8(x0);
                let v1 = vld1q_u8(x0.add(16));
                let v2 = vld1q_u8(x0.add(32));
                let v3 = vld1q_u8(x0.add(48));
                w4 -= 1;
                vst1q_u8(x0, add_dc16(v16, v0));
                vst1q_u8(x0.add(16), add_dc16(v16, v1));
                vst1q_u8(x0.add(32), add_dc16(v16, v2));
                vst1q_u8(x0.add(48), add_dc16(v16, v3));
                x0 = row(x0, stride, 1);
            }
        }
        // b.gt 1b
        if w4 <= 0 {
            break;
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

// iadst8_coeffs (v0) and its idct_coeffs tail (v1).
const A8: [i16; 8] = [4076, 401, 3612, 1931, 2598, 3166, 1189, 3920];
const A8B: [i16; 4] = [2896, 0, 1567, 3784];
// iadst16_coeffs (v0, v1).
const A16: [i16; 8] = [4091, 201, 3973, 995, 3703, 1751, 3290, 2440];
const A16B: [i16; 8] = [2751, 3035, 2106, 3513, 1380, 3857, 601, 4052];

/// `iadst_4x4`/`iadst_8x4` (`Q`) on v16–v19: the outputs in order (the
/// `inv_flipadst_*` entry points store them reversed). All sums are the
/// assembly's wrapping 32-bit `add`/`sub`/`mul` before `sqrshrn #12`.
#[inline(always)]
fn iadst_4<const Q: bool>(v: &V) -> [int16x8_t; 4] {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        // iadst4_coeffs: 1321, 3803, 2482, 3344, and .s[2] = 3344.
        let half = |i0: int16x4_t, i1: int16x4_t, i2: int16x4_t, i3: int16x4_t| {
            let v3 = vaddw_s16(vsubl_s16(i0, i2), i3);
            let v4 = vmlal_n_s16(vmlal_n_s16(vmull_n_s16(i0, 1321), i2, 3803), i3, 2482);
            let v7 = vmull_n_s16(i1, 3344);
            let v5 = vmlsl_n_s16(vmlsl_n_s16(vmull_n_s16(i0, 2482), i2, 1321), i3, 3803);
            let o3 = vsubq_s32(vaddq_s32(v4, v5), v7);
            let o2 = vmulq_n_s32(v3, 3344);
            let o0 = vaddq_s32(v4, v7);
            let o1 = vaddq_s32(v5, v7);
            [o0, o1, o2, o3]
        };
        let l = half(lo(v[16]), lo(v[17]), lo(v[18]), lo(v[19]));
        if !Q {
            return l.map(|x| d(vqrshrn_n_s32::<12>(x)));
        }
        let hi = |x: int16x8_t| vget_high_s16(x);
        let h = half(hi(v[16]), hi(v[17]), hi(v[18]), hi(v[19]));
        [0, 1, 2, 3].map(|k| vqrshrn_high_n_s32::<12>(vqrshrn_n_s32::<12>(l[k]), h[k]))
    }
}

/// `iadst_8 o0..o7, sz` on v16–v23: the outputs `out0..out7` in order.
#[inline(always)]
fn iadst_8<const Q: bool>(v: &V) -> [int16x8_t; 8] {
    let [i0, i1, i2, i3, i4, i5, i6, i7] = [v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]];
    let c = A8;
    let t0a = mul2::<Q, false, false>(i7, i0, c[0], c[1]);
    let t1a = mul2::<Q, true, false>(i7, i0, c[1], c[0]);
    let t2a = mul2::<Q, false, false>(i5, i2, c[2], c[3]);
    let t3a = mul2::<Q, true, false>(i5, i2, c[3], c[2]);
    let t4a = mul2::<Q, false, false>(i3, i4, c[4], c[5]);
    let t5a = mul2::<Q, true, false>(i3, i4, c[5], c[4]);
    let t6a = mul2::<Q, false, false>(i1, i6, c[6], c[7]);
    let t7a = mul2::<Q, true, false>(i1, i6, c[7], c[6]);

    let t0 = sqadd::<Q>(t0a, t4a);
    let t4 = sqsub::<Q>(t0a, t4a);
    let t1 = sqadd::<Q>(t1a, t5a);
    let t5 = sqsub::<Q>(t1a, t5a);
    let t2 = sqadd::<Q>(t2a, t6a);
    let t6 = sqsub::<Q>(t2a, t6a);
    let t3 = sqadd::<Q>(t3a, t7a);
    let t7 = sqsub::<Q>(t3a, t7a);

    let b = A8B;
    let t4a = mul2::<Q, false, false>(t4, t5, b[3], b[2]);
    let t5a = mul2::<Q, true, false>(t4, t5, b[2], b[3]);
    let t6a = mul2::<Q, true, false>(t7, t6, b[3], b[2]);
    let t7a = mul2::<Q, false, false>(t7, t6, b[2], b[3]);

    let out0 = sqadd::<Q>(t0, t2);
    let t2 = sqsub::<Q>(t0, t2);
    let out7 = sqneg::<Q>(sqadd::<Q>(t1, t3));
    let t3 = sqsub::<Q>(t1, t3);
    let out1 = sqneg::<Q>(sqadd::<Q>(t4a, t6a));
    let t6 = sqsub::<Q>(t4a, t6a);
    let out6 = sqadd::<Q>(t5a, t7a);
    let t7 = sqsub::<Q>(t5a, t7a);

    let out3 = sqneg::<Q>(mul2::<Q, false, false>(t2, t3, b[0], b[0]));
    let out4 = mul2::<Q, true, false>(t2, t3, b[0], b[0]);
    let out5 = sqneg::<Q>(mul2::<Q, true, false>(t6, t7, b[0], b[0]));
    let out2 = mul2::<Q, false, false>(t6, t7, b[0], b[0]);
    [out0, out1, out2, out3, out4, out5, out6, out7]
}

/// `iadst_16 o0..o15, sz, szb` on v16–v31: the outputs `out0..out15` in
/// order.
#[inline(always)]
fn iadst_16<const Q: bool>(v: &V) -> [int16x8_t; 16] {
    let i = |k: usize| v[16 + k];
    let (a, b) = (A16, A16B);
    let t0 = mul2::<Q, false, false>(i(15), i(0), a[0], a[1]);
    let t1 = mul2::<Q, true, false>(i(15), i(0), a[1], a[0]);
    let t2 = mul2::<Q, false, false>(i(13), i(2), a[2], a[3]);
    let t3 = mul2::<Q, true, false>(i(13), i(2), a[3], a[2]);
    let t4 = mul2::<Q, false, false>(i(11), i(4), a[4], a[5]);
    let t5 = mul2::<Q, true, false>(i(11), i(4), a[5], a[4]);
    let t6 = mul2::<Q, false, false>(i(9), i(6), a[6], a[7]);
    let t7 = mul2::<Q, true, false>(i(9), i(6), a[7], a[6]);
    let t8 = mul2::<Q, false, false>(i(7), i(8), b[0], b[1]);
    let t9 = mul2::<Q, true, false>(i(7), i(8), b[1], b[0]);
    let t10 = mul2::<Q, false, false>(i(5), i(10), b[2], b[3]);
    let t11 = mul2::<Q, true, false>(i(5), i(10), b[3], b[2]);
    let t12 = mul2::<Q, false, false>(i(3), i(12), b[4], b[5]);
    let t13 = mul2::<Q, true, false>(i(3), i(12), b[5], b[4]);
    let t14 = mul2::<Q, false, false>(i(1), i(14), b[6], b[7]);
    let t15 = mul2::<Q, true, false>(i(1), i(14), b[7], b[6]);

    // ldrel q0, idct_coeffs
    let t8a = sqsub::<Q>(t0, t8);
    let t0a = sqadd::<Q>(t0, t8);
    let t9a = sqsub::<Q>(t1, t9);
    let t1a = sqadd::<Q>(t1, t9);
    let t2a = sqadd::<Q>(t2, t10);
    let t10a = sqsub::<Q>(t2, t10);
    let t3a = sqadd::<Q>(t3, t11);
    let t11a = sqsub::<Q>(t3, t11);
    let t4a = sqadd::<Q>(t4, t12);
    let t12a = sqsub::<Q>(t4, t12);
    let t5a = sqadd::<Q>(t5, t13);
    let t13a = sqsub::<Q>(t5, t13);
    let t6a = sqadd::<Q>(t6, t14);
    let t14a = sqsub::<Q>(t6, t14);
    let t7a = sqadd::<Q>(t7, t15);
    let t15a = sqsub::<Q>(t7, t15);

    let t8 = mul2::<Q, false, false>(t8a, t9a, C0[5], C0[4]);
    let t9 = mul2::<Q, true, false>(t8a, t9a, C0[4], C0[5]);
    let t10 = mul2::<Q, false, false>(t10a, t11a, C0[7], C0[6]);
    let t11 = mul2::<Q, true, false>(t10a, t11a, C0[6], C0[7]);
    let t12 = mul2::<Q, true, false>(t13a, t12a, C0[5], C0[4]);
    let t13 = mul2::<Q, false, false>(t13a, t12a, C0[4], C0[5]);
    let t14 = mul2::<Q, true, false>(t15a, t14a, C0[7], C0[6]);
    let t15 = mul2::<Q, false, false>(t15a, t14a, C0[6], C0[7]);

    let t4 = sqsub::<Q>(t0a, t4a);
    let t0 = sqadd::<Q>(t0a, t4a);
    let t5 = sqsub::<Q>(t1a, t5a);
    let t1 = sqadd::<Q>(t1a, t5a);
    let t2 = sqadd::<Q>(t2a, t6a);
    let t6 = sqsub::<Q>(t2a, t6a);
    let t3 = sqadd::<Q>(t3a, t7a);
    let t7 = sqsub::<Q>(t3a, t7a);
    let t8a = sqadd::<Q>(t8, t12);
    let t12a = sqsub::<Q>(t8, t12);
    let t9a = sqadd::<Q>(t9, t13);
    let t13a = sqsub::<Q>(t9, t13);
    let t10a = sqadd::<Q>(t10, t14);
    let t14a = sqsub::<Q>(t10, t14);
    let t11a = sqadd::<Q>(t11, t15);
    let t15a = sqsub::<Q>(t11, t15);

    let t4a = mul2::<Q, false, false>(t4, t5, C0[3], C0[2]);
    let t5a = mul2::<Q, true, false>(t4, t5, C0[2], C0[3]);
    let t6a = mul2::<Q, true, false>(t7, t6, C0[3], C0[2]);
    let t7a = mul2::<Q, false, false>(t7, t6, C0[2], C0[3]);
    let t12 = mul2::<Q, false, false>(t12a, t13a, C0[3], C0[2]);
    let t13 = mul2::<Q, true, false>(t12a, t13a, C0[2], C0[3]);
    let t14 = mul2::<Q, true, false>(t15a, t14a, C0[3], C0[2]);
    let t15 = mul2::<Q, false, false>(t15a, t14a, C0[2], C0[3]);

    let t2a = sqsub::<Q>(t0, t2);
    let out0 = sqadd::<Q>(t0, t2);
    let t3a = sqsub::<Q>(t1, t3);
    let out15 = sqneg::<Q>(sqadd::<Q>(t1, t3));
    let t15a = sqsub::<Q>(t13, t15);
    let out13 = sqneg::<Q>(sqadd::<Q>(t13, t15));
    let out2 = sqadd::<Q>(t12, t14);
    let t14a = sqsub::<Q>(t12, t14);
    let out1 = sqneg::<Q>(sqadd::<Q>(t8a, t10a));
    let t10 = sqsub::<Q>(t8a, t10a);
    let out14 = sqadd::<Q>(t9a, t11a);
    let t11 = sqsub::<Q>(t9a, t11a);
    let out3 = sqneg::<Q>(sqadd::<Q>(t4a, t6a));
    let t6 = sqsub::<Q>(t4a, t6a);
    let out12 = sqadd::<Q>(t5a, t7a);
    let t7 = sqsub::<Q>(t5a, t7a);

    let h0 = C0[0];
    let out8 = mul2::<Q, true, false>(t2a, t3a, h0, h0);
    let out7 = sqneg::<Q>(mul2::<Q, false, false>(t2a, t3a, h0, h0));
    let out5 = sqneg::<Q>(mul2::<Q, false, false>(t14a, t15a, h0, h0));
    let out10 = mul2::<Q, true, false>(t14a, t15a, h0, h0);
    let out4 = mul2::<Q, false, false>(t6, t7, h0, h0);
    let out11 = sqneg::<Q>(mul2::<Q, true, false>(t6, t7, h0, h0));
    let out6 = mul2::<Q, false, false>(t10, t11, h0, h0);
    let out9 = sqneg::<Q>(mul2::<Q, true, false>(t10, t11, h0, h0));
    [
        out0, out1, out2, out3, out4, out5, out6, out7, out8, out9, out10, out11, out12, out13, out14, out15,
    ]
}

/// The kind of a 1-D transform an entry point loads into `x4`/`x5`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum K {
    Dct,
    Adst,
    Flip,
    Id,
}

/// `blr` to `inv_<k>_8h_x<n>_neon` (`Q`) or `inv_<k>_4h_x<n>_neon` on
/// v16 onwards.
#[inline(always)]
fn call<const Q: bool>(k: K, n: usize, v: &mut V) {
    match (k, n) {
        (K::Dct, 4) => idct_4::<Q>(v, 16, 17, 18, 19),
        (K::Dct, 8) => idct_8::<Q>(v, [16, 17, 18, 19, 20, 21, 22, 23]),
        (K::Dct, _) => idct_16::<Q>(v),
        (K::Adst | K::Flip, 4) => {
            let o = iadst_4::<Q>(v);
            for j in 0..4 {
                v[if k == K::Flip { 19 - j } else { 16 + j }] = o[j];
            }
        }
        (K::Adst | K::Flip, 8) => {
            let o = iadst_8::<Q>(v);
            for j in 0..8 {
                v[if k == K::Flip { 23 - j } else { 16 + j }] = o[j];
            }
        }
        (K::Adst | K::Flip, _) => {
            let o = iadst_16::<Q>(v);
            for j in 0..16 {
                v[if k == K::Flip { 31 - j } else { 16 + j }] = o[j];
            }
        }
        (K::Id, 4) => {
            // mov w16, #(5793-4096)*8; sqrdmulh; sqadd
            for i in 16..20 {
                let t = sqrdmulh::<Q>(v[i], (5793 - 4096) * 8);
                v[i] = sqadd::<Q>(v[i], t);
            }
        }
        (K::Id, 8) => {
            for i in 16..24 {
                v[i] = sqshl1::<Q>(v[i]);
            }
        }
        (K::Id, _) => {
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

/// `iwht4` on v16–v19 (`.4h`, wrapping `add`/`sub`, `sshr #1`).
#[inline(always)]
fn iwht4(v: &mut V) {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let add = |a: int16x8_t, b: int16x8_t| d(vadd_s16(lo(a), lo(b)));
        let sub = |a: int16x8_t, b: int16x8_t| d(vsub_s16(lo(a), lo(b)));
        v[16] = add(v[16], v[17]);
        v[21] = sub(v[18], v[19]);
        v[20] = sub(v[16], v[21]);
        v[20] = d(vshr_n_s16::<1>(lo(v[20])));
        v[18] = sub(v[20], v[17]);
        v[17] = sub(v[20], v[19]);
        v[19] = add(v[21], v[18]);
        v[16] = sub(v[16], v[17]);
    }
}

// --- 4x4 -------------------------------------------------------------------

/// `inv_txfm_add_4x4_neon` with `L(itx_4x4_end)`.
#[inline(always)]
unsafe fn inv_txfm_add_4x4(dst: Dst, stride: isize, coeff: Coef, x4: K, x5: K) {
    let mut v = regs();
    v[16] = ld4h(coeff);
    v[17] = ld4h(coeff.add(4));
    v[18] = ld4h(coeff.add(8));
    v[19] = ld4h(coeff.add(12));
    zero8(coeff);
    call::<false>(x4, 4, &mut v);
    zero8(coeff.add(8));
    transpose_4x4h(&mut v, [16, 17, 18, 19], [20, 21, 22, 23]);
    call::<false>(x5, 4, &mut v);
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

/// `def_fn_4x4 txfm1, txfm2`
#[inline(always)]
unsafe fn itx_4x4(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc_4x4(dst, stride, coeff);
    }
    inv_txfm_add_4x4(dst, stride, coeff, t1, t2);
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_wht_wht_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    _eob: c_int,
) {
    let mut v = regs();
    v[16] = ld4h(coeff);
    v[17] = ld4h(coeff.add(4));
    v[18] = ld4h(coeff.add(8));
    v[19] = ld4h(coeff.add(12));
    zero8(coeff);
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    for i in 16..20 {
        v[i] = d(vshr_n_s16::<2>(lo(v[i])));
    }
    iwht4(&mut v);
    zero8(coeff.add(8));
    transpose_4x4h(&mut v, [16, 17, 18, 19], [20, 21, 22, 23]);
    iwht4(&mut v);
    v[16] = ins_hi_lo(v[16], v[17]);
    v[18] = ins_hi_lo(v[18], v[19]);
    // b L(itx_4x4_end): no rounding shift.
    add_store4x2::<0>(dst, stride, v[16]);
    add_store4x2::<0>(row(dst, stride, 2), stride, v[18]);
}

// --- 8x8 -------------------------------------------------------------------

/// `inv_txfm_add_8x8_neon` and `inv_txfm_identity_add_8x8_neon`
/// (`x4 == None`).
#[inline(always)]
unsafe fn inv_txfm_add_8x8(dst: Dst, stride: isize, coeff: Coef, x4: Option<K>, x5: K) {
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
        call::<true>(x4, 8, &mut v);
        for i in 16..24 {
            v[i] = srshr::<1>(v[i]);
        }
    }
    // The identity shl #1 and downshift srshr #1 cancel out.
    // L(itx_8x8_epilog):
    transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 24, 25);
    call::<true>(x5, 8, &mut v);
    for i in 0..8 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// `def_fn_8x8 txfm1, txfm2`
#[inline(always)]
unsafe fn itx_8x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<8, 8, 1>(dst, stride, coeff);
    }
    inv_txfm_add_8x8(dst, stride, coeff, if t1 == K::Id { None } else { Some(t1) }, t2);
}

// --- 8x4, 4x8 --------------------------------------------------------------

/// `inv_txfm_add_8x4_neon`: `x4` is `inv_*_4h_x8`, `x5` `inv_*_8h_x4`.
#[inline(always)]
unsafe fn inv_txfm_add_8x4(dst: Dst, stride: isize, coeff: Coef, x4: K, x5: K) {
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
    call::<false>(x4, 8, &mut v);
    transpose_4x4h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);
    transpose_4x4h(&mut v, [20, 21, 22, 23], [4, 5, 6, 7]);
    v[16] = ins_hi_lo(v[16], v[20]);
    v[17] = ins_hi_lo(v[17], v[21]);
    v[18] = ins_hi_lo(v[18], v[22]);
    v[19] = ins_hi_lo(v[19], v[23]);
    call::<true>(x5, 4, &mut v);
    for i in 0..4 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// `inv_txfm_add_4x8_neon`: `x4` is `inv_*_8h_x4`, `x5` `inv_*_4h_x8`.
#[inline(always)]
unsafe fn inv_txfm_add_4x8(dst: Dst, stride: isize, coeff: Coef, x4: K, x5: K) {
    let mut v = regs();
    for i in 0..4 {
        v[16 + i] = ld8h(coeff.add(8 * i));
    }
    for i in 0..4 {
        zero8(coeff.add(8 * i));
    }
    scale_input::<true>(&mut v, 16..20);
    call::<true>(x4, 4, &mut v);
    transpose_4x8h(&mut v, [16, 17, 18, 19], [4, 5, 6, 7]);
    v[20] = ins_lo_hi(v[20], v[16]);
    v[21] = ins_lo_hi(v[21], v[17]);
    v[22] = ins_lo_hi(v[22], v[18]);
    v[23] = ins_lo_hi(v[23], v[19]);
    call::<false>(x5, 8, &mut v);
    // load_add_store_4x8: rows 2k, 2k+1 from v(16+2k) | v(17+2k).
    for k in 0..4 {
        let x = ins_hi_lo(v[16 + 2 * k], v[17 + 2 * k]);
        add_store4x2::<4>(row(dst, stride, 2 * k as isize), stride, x);
    }
}

/// `def_fn_48 4, 8, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_4x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<4, 8, 0>(dst, stride, coeff);
    }
    inv_txfm_add_4x8(dst, stride, coeff, t1, t2);
}

/// `def_fn_48 8, 4, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_8x4(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<8, 4, 0>(dst, stride, coeff);
    }
    inv_txfm_add_8x4(dst, stride, coeff, t1, t2);
}

// --- 16x16 and the shared 16-point passes ---------------------------------

/// `inv_txfm_horz_16x8_neon` (`def_horz_16 shift=2`), its `_scale` form
/// (`SCALE`, shift 1) and `inv_txfm_horz_identity_16x8_neon` (`x4 ==
/// None`): eight columns of coefficients from `x7` (16 rows, `x8` values
/// apart) into 128 values at `x6`.
#[inline(always)]
unsafe fn horz_16x8<const SCALE: bool>(v: &mut V, x6: *mut i16, x7: Coef, x8: usize, x4: Option<K>) {
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(x8 * i));
        zero8(x7.add(x8 * i));
    }
    if SCALE {
        scale_input::<true>(v, 16..32);
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
            call::<true>(x4, 16, v);
            for i in 16..32 {
                v[i] = if SCALE { srshr::<1>(v[i]) } else { srshr::<2>(v[i]) };
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

/// `inv_txfm_add_vert_8x16_neon`: 16 rows of 8 values from `x7` (`x8`
/// values apart), `x5`, `load_add_store_8x16` to `x6`.
#[inline(always)]
unsafe fn vert_8x16(v: &mut V, x6: Dst, stride: isize, x7: *const i16, x8: usize, x5: K) {
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(x8 * i));
    }
    call::<true>(x5, 16, v);
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
    x4: Option<K>,
    x5: K,
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
        horz_16x8::<false>(&mut v, x6, coeff.add(i), 16, x4);
    }
    for i in [0usize, 8] {
        vert_8x16(&mut v, dst.add(i), stride, sp.add(i), 16, x5);
    }
}

/// `def_fn_16x16 txfm1, txfm2, eob_half`
#[inline(always)]
unsafe fn itx_16x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<16, 16, 2>(dst, stride, coeff);
    }
    let eob_half = if (t1 == K::Dct && t2 == K::Id) || (t1 == K::Id && t2 == K::Dct) {
        8
    } else {
        36
    };
    inv_txfm_add_16x16(
        dst,
        stride,
        coeff,
        eob,
        if t1 == K::Id { None } else { Some(t1) },
        t2,
        eob_half,
    );
}

// --- 16x4, 4x16 ------------------------------------------------------------

/// `inv_txfm_add_16x4_neon` (`x4 == Some`, `inv_*_4h_x16`) and
/// `inv_txfm_identity_add_16x4_neon`; `x5` is `inv_*_8h_x4`.
#[inline(always)]
unsafe fn inv_txfm_add_16x4(dst: Dst, stride: isize, coeff: Coef, x4: Option<K>, x5: K) {
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
            call::<false>(x4, 16, &mut v);
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
    call::<true>(x5, 4, &mut v);
    for i in 0..4 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
    transpose_4x8h_mov(&mut v, [20, 21, 22, 23], [2, 3, 4, 5], [16, 17, 18, 19]);
    call::<true>(x5, 4, &mut v);
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
    x4: Option<K>,
    x5: K,
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
                call::<true>(x4, 4, &mut v);
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
            call::<true>(x4, 4, &mut v);
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
    call::<false>(x5, 16, &mut v);
    // load_add_store_4x16: rows 2k, 2k+1 from v(16+2k) | v(17+2k).
    for k in 0..8 {
        let x = ins_hi_lo(v[16 + 2 * k], v[17 + 2 * k]);
        add_store4x2::<4>(row(dst, stride, 2 * k as isize), stride, x);
    }
}

/// `def_fn_416 w, h, txfm1, txfm2, eob_half`: the threshold of the 4x16 form.
#[inline(always)]
fn eob_half_416(t1: K, t2: K) -> c_int {
    match (t1, t2) {
        (K::Id, K::Id) => 29,
        (_, K::Id) => 8,
        (K::Id, _) => 32,
        _ => 29,
    }
}

/// `def_fn_416 16, 4, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_16x4(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<16, 4, 1>(dst, stride, coeff);
    }
    inv_txfm_add_16x4(dst, stride, coeff, if t1 == K::Id { None } else { Some(t1) }, t2);
}

/// `def_fn_416 4, 16, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_4x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<4, 16, 1>(dst, stride, coeff);
    }
    let x4 = if t1 == K::Id { None } else { Some(t1) };
    inv_txfm_add_4x16(dst, stride, coeff, eob, x4, t2, eob_half_416(t1, t2));
}

// --- 16x8, 8x16 ------------------------------------------------------------

/// `inv_txfm_add_16x8_neon` (`x4 == Some`, `inv_*_8h_x16`) and
/// `inv_txfm_identity_add_16x8_neon`; `x5` is `inv_*_8h_x8`.
#[inline(always)]
unsafe fn inv_txfm_add_16x8(dst: Dst, stride: isize, coeff: Coef, x4: Option<K>, x5: K) {
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
            call::<true>(x4, 16, &mut v);
            for i in 16..32 {
                v[i] = srshr::<1>(v[i]);
            }
        }
    }
    // L(itx_16x8_epilog):
    transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 2, 3);
    call::<true>(x5, 8, &mut v);
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
    call::<true>(x5, 8, &mut v);
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
    x4: Option<K>,
    x5: K,
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
                call::<true>(x4, 8, &mut v);
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
        call::<true>(x4, 8, &mut v);
        for i in 16..24 {
            v[i] = srshr::<1>(v[i]);
        }
    }
    // L(itx_8x16_epilog):
    transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 2, 3);
    call::<true>(x5, 16, &mut v);
    for i in 0..16 {
        add_store8::<4>(row(dst, stride, i as isize), v[16 + i]);
    }
}

/// `def_fn_816 w, h, txfm1, txfm2, eob_half`: the threshold of the 8x16 form.
#[inline(always)]
fn eob_half_816(t1: K, t2: K) -> c_int {
    match (t1, t2) {
        (K::Id, K::Id) => 43,
        (_, K::Id) => 8,
        (K::Id, _) => 64,
        _ => 43,
    }
}

/// `def_fn_816 16, 8, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_16x8(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<16, 8, 1>(dst, stride, coeff);
    }
    inv_txfm_add_16x8(dst, stride, coeff, if t1 == K::Id { None } else { Some(t1) }, t2);
}

/// `def_fn_816 8, 16, txfm1, txfm2`
#[inline(always)]
unsafe fn itx_8x16(dst: Dst, stride: isize, coeff: Coef, eob: c_int, t1: K, t2: K) {
    if t1 == K::Dct && t2 == K::Dct && eob == 0 {
        return idct_dc::<8, 16, 1>(dst, stride, coeff);
    }
    let x4 = if t1 == K::Id { None } else { Some(t1) };
    inv_txfm_add_8x16(dst, stride, coeff, eob, x4, t2, eob_half_816(t1, t2));
}

// --- 32-point -------------------------------------------------------------

// idct_coeffs, idct32 part (v0, v1 of inv_dct32_odd_8h_x16_neon).
const C32A: [i16; 8] = [201, 4091, 3035, 2751, 1751, 3703, 3857, 1380];
const C32B: [i16; 8] = [995, 3973, 3513, 2106, 2440, 3290, 4052, 601];

/// `inv_dct32_odd_8h_x16_neon` on v16–v31 (the odd inputs of a 32-point
/// DCT); the outputs replace the inputs as in the assembly.
#[inline(always)]
fn idct32_odd(v: &mut V) {
    let a = |k: usize, v: &V| v[k];
    let (p, q) = (C32A, C32B);
    let m = |s0, s1, c0, c1, sub: bool| {
        if sub {
            mul2::<true, true, false>(s0, s1, c0, c1)
        } else {
            mul2::<true, false, false>(s0, s1, c0, c1)
        }
    };
    let mn = |s0, s1, c0, c1| mul2::<true, false, true>(s0, s1, c0, c1);
    let (ad, sb) = (sqadd::<true>, sqsub::<true>);

    let t16a = m(a(16, v), a(31, v), p[0], p[1], true);
    let t31a = m(a(16, v), a(31, v), p[1], p[0], false);
    let t17a = m(a(24, v), a(23, v), p[2], p[3], true);
    let t30a = m(a(24, v), a(23, v), p[3], p[2], false);
    let t18a = m(a(20, v), a(27, v), p[4], p[5], true);
    let t29a = m(a(20, v), a(27, v), p[5], p[4], false);
    let t19a = m(a(28, v), a(19, v), p[6], p[7], true);
    let t28a = m(a(28, v), a(19, v), p[7], p[6], false);
    let t20a = m(a(18, v), a(29, v), q[0], q[1], true);
    let t27a = m(a(18, v), a(29, v), q[1], q[0], false);
    let t21a = m(a(26, v), a(21, v), q[2], q[3], true);
    let t26a = m(a(26, v), a(21, v), q[3], q[2], false);
    let t22a = m(a(22, v), a(25, v), q[4], q[5], true);
    let t25a = m(a(22, v), a(25, v), q[5], q[4], false);
    let t23a = m(a(30, v), a(17, v), q[6], q[7], true);
    let t24a = m(a(30, v), a(17, v), q[7], q[6], false);

    // ld1 {v0.8h}, [x16] (idct_coeffs)
    let t17 = sb(t16a, t17a);
    let t16 = ad(t16a, t17a);
    let t30 = sb(t31a, t30a);
    let t31 = ad(t31a, t30a);
    let t18 = sb(t19a, t18a);
    let t19 = ad(t19a, t18a);
    let t20 = ad(t20a, t21a);
    let t21 = sb(t20a, t21a);
    let t22 = sb(t23a, t22a);
    let t23 = ad(t23a, t22a);
    let t24 = ad(t24a, t25a);
    let t25 = sb(t24a, t25a);
    let t26 = sb(t27a, t26a);
    let t27 = ad(t27a, t26a);
    let t28 = ad(t28a, t29a);
    let t29 = sb(t28a, t29a);

    let t17a = m(t30, t17, C0[4], C0[5], true);
    let t30a = m(t30, t17, C0[5], C0[4], false);
    let t18a = mn(t29, t18, C0[5], C0[4]);
    let t29a = m(t29, t18, C0[4], C0[5], true);
    let t21a = m(t26, t21, C0[6], C0[7], true);
    let t26a = m(t26, t21, C0[7], C0[6], false);
    let t22a = mn(t25, t22, C0[7], C0[6]);
    let t25a = m(t25, t22, C0[6], C0[7], true);

    let t29 = sb(t30a, t29a);
    let t30 = ad(t30a, t29a);
    let t18 = sb(t17a, t18a);
    let t17 = ad(t17a, t18a);
    let t19a = sb(t16, t19);
    let t16a = ad(t16, t19);
    let t20a = sb(t23, t20);
    let t23a = ad(t23, t20);
    let t21 = sb(t22a, t21a);
    let t22 = ad(t22a, t21a);
    let t24a = ad(t24, t27);
    let t27a = sb(t24, t27);
    let t25 = ad(t25a, t26a);
    let t26 = sb(t25a, t26a);
    let t28a = sb(t31, t28);
    let t31a = ad(t31, t28);

    let t18a = m(t29, t18, C0[2], C0[3], true);
    let t29a = m(t29, t18, C0[3], C0[2], false);
    let t19 = m(t28a, t19a, C0[2], C0[3], true);
    let t28 = m(t28a, t19a, C0[3], C0[2], false);
    let t20 = mn(t27a, t20a, C0[3], C0[2]);
    let t27 = m(t27a, t20a, C0[2], C0[3], true);
    let t21a = mn(t26, t21, C0[3], C0[2]);
    let t26a = m(t26, t21, C0[2], C0[3], true);

    let t23 = sb(t16a, t23a);
    let out16 = ad(t16a, t23a);
    let t24 = sb(t31a, t24a);
    let out31 = ad(t31a, t24a);
    let t22a = sb(t17, t22);
    let out17 = ad(t17, t22);
    let out30 = ad(t30, t25);
    let t25a = sb(t30, t25);
    let t21 = sb(t18a, t21a);
    let out18 = ad(t18a, t21a);
    let out19 = ad(t19, t20);
    let t20a = sb(t19, t20);
    let out29 = ad(t29a, t26a);
    let t26 = sb(t29a, t26a);
    let out28 = ad(t28, t27);
    let t27a = sb(t28, t27);

    let h0 = C0[0];
    let t20 = m(t27a, t20a, h0, h0, true);
    let t27 = m(t27a, t20a, h0, h0, false);
    let t26a = m(t26, t21, h0, h0, false);
    let t21a = m(t26, t21, h0, h0, true);
    let t22 = m(t25a, t22a, h0, h0, true);
    let t25 = m(t25a, t22a, h0, h0, false);
    let t23a = m(t24, t23, h0, h0, true);
    let t24a = m(t24, t23, h0, h0, false);

    let out = [
        out16, out17, out18, out19, t20, t21a, t22, t23a, t24a, t25, t26a, t27, out28, out29, out30, out31,
    ];
    for (k, x) in out.into_iter().enumerate() {
        v[16 + k] = x;
    }
}

/// Reverses the eight lanes: `rev64 .8h` followed by `ext .16b, #8`.
#[inline(always)]
fn rev8(x: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let r = vrev64q_s16(x);
        vextq_s16::<4>(r, r)
    }
}

/// `inv_txfm_horz_dct_32x8_neon` (shift 2) and
/// `inv_txfm_horz_scale_dct_32x8_neon` (`SCALE`, shift 1): eight columns
/// of coefficients from `x7` (32 rows, `x8` values apart) into eight rows
/// of 32 values at `x6`.
#[inline(always)]
unsafe fn horz_dct_32x8<const SCALE: bool>(v: &mut V, x6: *mut i16, x7: Coef, x8: usize) {
    // lsl x8, x8, #1: the even rows, then (add x7, x7, x8, lsr #1) the odd.
    let x8 = 2 * x8;
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(x8 * i));
        zero8(x7.add(x8 * i));
    }
    let x7 = x7.add(x8 / 2);
    if SCALE {
        scale_input::<true>(v, 16..32);
    }
    idct_16::<true>(v);
    transpose_8x8h(v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);
    transpose_8x8h(v, [24, 25, 26, 27, 28, 29, 30, 31], 4, 5);
    // store1: row j gets v(16+j) and v(24+j) in its first 16 values.
    for j in 0..8 {
        vst1q_s16(x6.add(32 * j), v[16 + j]);
        vst1q_s16(x6.add(32 * j + 8), v[24 + j]);
    }
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(x8 * i));
        zero8(x7.add(x8 * i));
    }
    if SCALE {
        // v0.h[1] after inv_dct_8h_x16_neon is 2896*8.
        scale_input::<true>(v, 16..32);
    }
    idct32_odd(v);
    transpose_8x8h(v, [31, 30, 29, 28, 27, 26, 25, 24], 4, 5);
    transpose_8x8h(v, [23, 22, 21, 20, 19, 18, 17, 16], 4, 5);
    // store2 r0, r1 for (v31, v23), (v30, v22) … (v24, v16)
    for j in 0..8 {
        let (r0, r1) = (v[31 - j], v[23 - j]);
        let p = x6.add(32 * j);
        let v4 = vld1q_s16(p);
        let v5 = vld1q_s16(p.add(8));
        let v7 = sqsub::<true>(v4, r0);
        let v6 = sqsub::<true>(v5, r1);
        let v4 = sqadd::<true>(v4, r0);
        let v5 = sqadd::<true>(v5, r1);
        let sh = |x| if SCALE { srshr::<1>(x) } else { srshr::<2>(x) };
        vst1q_s16(p, sh(v4));
        vst1q_s16(p.add(8), sh(v5));
        vst1q_s16(p.add(16), rev8(sh(v6)));
        vst1q_s16(p.add(24), rev8(sh(v7)));
    }
}

/// `inv_txfm_add_vert_dct_8x32_neon`: a 32-point DCT down eight columns of
/// `x7` (rows `x8` values apart), added to 32 rows of 8 pixels at `x6`.
#[inline(always)]
unsafe fn vert_dct_8x32(v: &mut V, x6: Dst, stride: isize, x7: *mut i16, x8: usize) {
    let x8 = 2 * x8;
    for i in 0..16 {
        v[16 + i] = ld8h(x7.add(x8 * i));
    }
    idct_16::<true>(v);
    for i in 0..16 {
        vst1q_s16(x7.add(x8 * i), v[16 + i]);
    }
    let odd = x7.add(x8 / 2);
    for i in 0..16 {
        v[16 + i] = ld8h(odd.add(x8 * i));
    }
    idct32_odd(v);
    // combine: rows 0..15 = even[m] + v(31-m); rows 16..31 = even[15-m] - v(16+m).
    let mut y = 0isize;
    for m in 0..16 {
        let e = ld8h(x7.add(x8 * m));
        add_store8::<4>(row(x6, stride, y), sqadd::<true>(e, v[31 - m]));
        y += 1;
    }
    for m in 0..16 {
        let e = ld8h(x7.add(x8 * (15 - m)));
        add_store8::<4>(row(x6, stride, y), sqsub::<true>(e, v[16 + m]));
        y += 1;
    }
}

const EOB_32X32: [c_int; 4] = [36, 136, 300, 1024];
const EOB_16X32: [c_int; 4] = [36, 151, 279, 512];
const EOB_16X32_SHORTSIDE: [c_int; 2] = [36, 512];
const EOB_8X32: [c_int; 4] = [43, 107, 171, 256];

/// `inv_txfm_add_identity_identity_32x32_8bpc_neon` and
/// `def_identity_1632 16, 32` / `32, 16`: 8x8 blocks, columns while `eob`
/// reaches the inner thresholds, rows while it reaches the outer ones.
/// `W`/`H` select the scaling of the block.
#[inline(always)]
unsafe fn identity_32<const W: usize, const H: usize>(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
    inner: &[c_int],
    outer: &[c_int],
) {
    let mut v = regs();
    let x8 = H;
    let mut x0 = dst;
    let mut x2 = coeff;
    let mut x13 = 0;
    loop {
        let mut w9 = 0usize;
        let mut x12 = 0;
        loop {
            w9 += 8;
            for i in 0..8 {
                v[16 + i] = ld8h(x2);
                zero8(x2);
                x2 = x2.add(x8);
            }
            if W == 16 && H == 32 {
                scale_input::<true>(&mut v, 16..24);
                // identity_8x8_shift1 v1.h[1] = 2*(5793-4096)*8
                for i in 16..24 {
                    let v2 = sqrdmulh::<true>(v[i], 2 * (5793 - 4096) * 8);
                    let v2 = srshr::<1>(v2);
                    v[i] = sqadd::<true>(v[i], v2);
                }
            } else if W == 32 && H == 16 {
                scale_input::<true>(&mut v, 16..24);
                for i in 16..24 {
                    v[i] = sqshl1::<true>(v[i]);
                }
                // identity_8x8 v1.h[1]
                for i in 16..24 {
                    let v2 = sqrdmulh::<true>(v[i], 2 * (5793 - 4096) * 8);
                    v[i] = sqadd::<true>(v[i], v[i]);
                    v[i] = sqadd::<true>(v[i], v2);
                }
            }
            transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);
            for i in 0..8 {
                let p = row(x0, stride, i as isize);
                if W == 32 && H == 16 {
                    add_store8::<4>(p, v[16 + i]);
                } else {
                    add_store8::<2>(p, v[16 + i]);
                }
            }
            let w11 = inner[x12];
            x12 += 1;
            x0 = x0.add(8);
            if eob < w11 {
                break;
            }
        }
        let w11 = outer[x13];
        x13 += 1;
        if eob < w11 {
            return;
        }
        x0 = row(x0.sub(w9), stride, 8);
        x2 = x2.sub(x8 * w9).add(8);
    }
}

/// `def_identity_832 8, 32` / `32, 8`.
#[inline(always)]
unsafe fn identity_832<const W: usize, const H: usize>(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    let mut v = regs();
    let x8 = H;
    let mut x0 = dst;
    let mut x2 = coeff;
    let mut x13 = 0;
    loop {
        let w12 = EOB_8X32[x13];
        x13 += 1;
        for i in 0..8 {
            v[16 + i] = ld8h(x2);
            zero8(x2);
            x2 = x2.add(x8);
        }
        if W == 8 {
            for i in 16..24 {
                v[i] = srshr::<1>(v[i]);
            }
        }
        transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);
        for i in 0..8 {
            let p = row(x0, stride, i as isize);
            if W == 8 {
                add_store8::<2>(p, v[16 + i]);
            } else {
                add_store8::<3>(p, v[16 + i]);
            }
        }
        if eob < w12 {
            return;
        }
        if W == 8 {
            // the next 8 coefficients; x0 has moved down 8 rows
            x0 = row(x0, stride, 8);
            x2 = x2.sub(8 * x8).add(8);
        } else {
            x0 = x0.add(8);
        }
    }
}

/// The zero fill at label `1:` of the 32- and 64-point entry points: `n`
/// values from `x6` on, in steps of `step` values (`subs w8, w8, #k` with
/// four `st1 {v4–v7}` per step).
#[inline(always)]
unsafe fn zero_fill(x6: *mut i16, rows: usize, rows_per_step: usize, row_len: usize) {
    let mut w8 = rows as isize;
    let mut p = x6;
    loop {
        w8 -= rows_per_step as isize;
        for k in 0..16 {
            zero8(p.add(8 * k));
        }
        p = p.add(128);
        if w8 <= 0 {
            break;
        }
    }
    debug_assert_eq!(rows_per_step * row_len, 128);
}

#[inline(always)]
unsafe fn itx_32x32_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<32, 32, 2>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut buf = [0i16; 32 * 32];
    let sp = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = EOB_32X32[x13];
    x13 += 1;
    for i in [0usize, 8, 16, 24] {
        let x6 = sp.add(i * 32);
        if i > 0 {
            if eob < w12 {
                zero_fill(x6, 32 - i, 4, 32);
                break;
            }
            if i < 24 {
                w12 = EOB_32X32[x13];
                x13 += 1;
            }
        }
        horz_dct_32x8::<false>(&mut v, x6, coeff.add(i), 32);
    }
    for i in [0usize, 8, 16, 24] {
        vert_dct_8x32(&mut v, dst.add(i), stride, sp.add(i), 32);
    }
}

#[inline(always)]
unsafe fn itx_16x32_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<16, 32, 1>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut buf = [0i16; 16 * 32];
    let sp = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = EOB_16X32[x13];
    x13 += 1;
    for i in [0usize, 8, 16, 24] {
        let x6 = sp.add(i * 16);
        if i > 0 {
            if eob < w12 {
                zero_fill(x6, 32 - i, 8, 16);
                break;
            }
            if i < 24 {
                w12 = EOB_16X32[x13];
                x13 += 1;
            }
        }
        horz_16x8::<true>(&mut v, x6, coeff.add(i), 32, Some(K::Dct));
    }
    for i in [0usize, 8] {
        vert_dct_8x32(&mut v, dst.add(i), stride, sp.add(i), 16);
    }
}

#[inline(always)]
unsafe fn itx_32x16_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<32, 16, 1>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut buf = [0i16; 32 * 16];
    let sp = buf.as_mut_ptr();
    for i in [0usize, 8] {
        let x6 = sp.add(i * 32);
        if i > 0 && eob < 36 {
            zero_fill(x6, 16 - i, 4, 32);
            break;
        }
        horz_dct_32x8::<true>(&mut v, x6, coeff.add(i), 16);
    }
    for i in [0usize, 8, 16, 24] {
        vert_8x16(&mut v, dst.add(i), stride, sp.add(i), 32, K::Dct);
    }
}

#[inline(always)]
unsafe fn itx_8x32_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<8, 32, 2>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut buf = [0i16; 8 * 32];
    let mut x6 = buf.as_mut_ptr();
    let x8 = 32;
    let mut w9 = 32isize;
    let mut x2 = coeff;
    let mut x13 = 0;
    loop {
        for i in 0..8 {
            v[16 + i] = ld8h(x2.add(x8 * i));
            zero8(x2.add(x8 * i));
        }
        let w12 = EOB_8X32[x13];
        x13 += 1;
        x2 = x2.add(8);
        w9 -= 8;
        call::<true>(K::Dct, 8, &mut v);
        for i in 16..24 {
            v[i] = srshr::<2>(v[i]);
        }
        transpose_8x8h(&mut v, [16, 17, 18, 19, 20, 21, 22, 23], 24, 25);
        for i in 0..8 {
            vst1q_s16(x6.add(8 * i), v[16 + i]);
        }
        x6 = x6.add(64);
        if eob < w12 {
            break;
        }
    }
    if w9 != 0 {
        loop {
            w9 -= 8;
            for k in 0..8 {
                zero8(x6.add(8 * k));
            }
            x6 = x6.add(64);
            if w9 <= 0 {
                break;
            }
        }
    }
    vert_dct_8x32(&mut v, dst, stride, buf.as_mut_ptr(), 8);
}

#[inline(always)]
unsafe fn itx_32x8_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<32, 8, 2>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut buf = [0i16; 32 * 8];
    let sp = buf.as_mut_ptr();
    horz_dct_32x8::<false>(&mut v, sp, coeff, 8);
    for w9 in [0usize, 8, 16, 24] {
        let x6 = dst.add(w9);
        let x7 = sp.add(w9);
        for i in 0..8 {
            v[16 + i] = ld8h(x7.add(32 * i));
        }
        call::<true>(K::Dct, 8, &mut v);
        for i in 0..8 {
            add_store8::<4>(row(x6, stride, i as isize), v[16 + i]);
        }
    }
}

// --- 64-point -------------------------------------------------------------

// idct64_coeffs: per inv_dct64_step1_neon call, v0 (8 values) and v1 (the
// first four used).
const C64: [[i16; 12]; 4] = [
    [
        101 * 8,
        4095 * 8,
        2967 * 8,
        -2824 * 8,
        1660 * 8,
        3745 * 8,
        3822 * 8,
        -1474 * 8,
        4076,
        401,
        4017,
        799,
    ],
    [
        4036 * 8,
        -700 * 8,
        2359 * 8,
        3349 * 8,
        3461 * 8,
        -2191 * 8,
        897 * 8,
        3996 * 8,
        -3166,
        -2598,
        -799,
        -4017,
    ],
    [
        501 * 8,
        4065 * 8,
        3229 * 8,
        -2520 * 8,
        2019 * 8,
        3564 * 8,
        3948 * 8,
        -1092 * 8,
        3612,
        1931,
        2276,
        3406,
    ],
    [
        4085 * 8,
        -301 * 8,
        2675 * 8,
        3102 * 8,
        3659 * 8,
        -1842 * 8,
        1285 * 8,
        3889 * 8,
        -3920,
        -1189,
        -3406,
        -2276,
    ],
];

/// `inv_dct64_step1_neon`: four inputs (v16–v19) to eight values at `x6`.
#[inline(always)]
unsafe fn dct64_step1(v: &V, x6: *mut i16, c: &[i16; 12]) {
    let (ad, sb) = (sqadd::<true>, sqsub::<true>);
    let mul = |x, k| sqrdmulh::<true>(x, k);
    let t63a = mul(v[16], c[1]);
    let t32a = mul(v[16], c[0]);
    let t62a = mul(v[17], c[2]);
    let t33a = mul(v[17], c[3]);
    let t61a = mul(v[18], c[5]);
    let t34a = mul(v[18], c[4]);
    let t60a = mul(v[19], c[6]);
    let t35a = mul(v[19], c[7]);

    let t32 = ad(t32a, t33a);
    let t33 = sb(t32a, t33a);
    let t34 = sb(t35a, t34a);
    let t35 = ad(t35a, t34a);
    let t60 = ad(t60a, t61a);
    let t61 = sb(t60a, t61a);
    let t62 = sb(t63a, t62a);
    let t63 = ad(t63a, t62a);

    let (d0, d1, d2, d3) = (c[8], c[9], c[10], c[11]);
    let t34a = mul2::<true, false, true>(t61, t34, d0, d1);
    let t61a = mul2::<true, true, false>(t61, t34, d1, d0);
    let t33a = mul2::<true, true, false>(t62, t33, d1, d0);
    let t62a = mul2::<true, false, false>(t62, t33, d0, d1);

    let t32a = ad(t32, t35);
    let t35a = sb(t32, t35);
    let t33 = ad(t33a, t34a);
    let t34 = sb(t33a, t34a);
    let t60a = sb(t63, t60);
    let t63a = ad(t63, t60);
    let t61 = sb(t62a, t61a);
    let t62 = ad(t62a, t61a);

    let t61a = mul2::<true, false, false>(t61, t34, d2, d3);
    let t34a = mul2::<true, true, false>(t61, t34, d3, d2);
    let t60 = mul2::<true, false, false>(t60a, t35a, d2, d3);
    let t35 = mul2::<true, true, false>(t60a, t35a, d3, d2);

    for (k, x) in [t32a, t33, t34a, t35, t60, t61a, t62, t63a]
        .into_iter()
        .enumerate()
    {
        vst1q_s16(x6.add(8 * k), x);
    }
}

/// `inv_dct64_step2_neon`: the butterflies of t32..t63 in place, from the
/// outside in (`x6` upwards from t32a, `x9` downwards from t39a).
#[inline(always)]
unsafe fn dct64_step2(x6: *mut i16) {
    let (ad, sb) = (sqadd::<true>, sqsub::<true>);
    let h0 = C0[0];
    let (h2, h3) = (C0[2], C0[3]);
    let mut a = x6;
    let mut b = x6.add(8 * 7);
    while a < b {
        let at = |k: usize| a.add(8 * 8 * k);
        let bt = |k: usize| b.add(8 * 8 * k);
        let v16 = vld1q_s16(at(0));
        let v17 = vld1q_s16(bt(1));
        let v18 = vld1q_s16(bt(0));
        let v19 = vld1q_s16(at(1));
        let v20 = vld1q_s16(at(2));
        let v21 = vld1q_s16(bt(3));
        let v22 = vld1q_s16(bt(2));
        let v23 = vld1q_s16(at(3));

        let t32 = ad(v16, v17);
        let t39 = sb(v16, v17);
        let t63 = ad(v18, v19);
        let t56 = sb(v18, v19);
        let t40 = sb(v21, v20);
        let t47 = ad(v21, v20);
        let t48 = ad(v23, v22);
        let t55 = sb(v23, v22);

        let t56a = mul2::<true, false, false>(t56, t39, h3, h2);
        let t39a = mul2::<true, true, false>(t56, t39, h2, h3);
        let t40a = mul2::<true, false, true>(t55, t40, h3, h2);
        let t55a = mul2::<true, true, false>(t55, t40, h2, h3);

        let t32a = ad(t32, t47);
        let t47a = sb(t32, t47);
        let t39 = ad(t39a, t40a);
        let t40 = sb(t39a, t40a);
        let t48a = sb(t63, t48);
        let t63a = ad(t63, t48);
        let t55 = sb(t56a, t55a);
        let t56 = ad(t56a, t55a);

        let t40a = mul2::<true, true, false>(t55, t40, h0, h0);
        let t55a = mul2::<true, false, false>(t55, t40, h0, h0);
        let t47 = mul2::<true, true, false>(t48a, t47a, h0, h0);
        let t48 = mul2::<true, false, false>(t48a, t47a, h0, h0);

        vst1q_s16(at(0), t32a);
        vst1q_s16(bt(0), t39);
        vst1q_s16(at(1), t40a);
        vst1q_s16(bt(1), t47);
        vst1q_s16(at(2), t48);
        vst1q_s16(bt(2), t55a);
        vst1q_s16(at(3), t56);
        vst1q_s16(bt(3), t63a);

        a = a.add(8);
        b = b.sub(8);
    }
}

/// `inv_txfm_dct{,_clear,_clear_scale}_8h_x64_neon`: a 64-point DCT of
/// eight columns of `x7` (32 rows, `x8` values apart; the upper 32 inputs
/// are zero) into 64 vectors at `sp` (the assembly's stack scratch).
#[inline(always)]
unsafe fn dct_8h_x64<const CLEAR: bool, const SCALE: bool>(v: &mut V, sp: *mut i16, x7: Coef, x8: usize) {
    let x8_4 = 4 * x8;
    // load8 x7, x8 (rows 0, 4, … 28); clear_upper8
    for i in 0..8 {
        v[16 + i] = ld8h(x7.add(x8_4 * i));
        if CLEAR {
            zero8(x7.add(x8_4 * i));
        }
    }
    for i in 24..32 {
        v[i] = vdupq_n_s16(0);
    }
    if SCALE {
        scale_input::<true>(v, 16..24);
    }
    idct_16::<true>(v);
    for i in 0..16 {
        vst1q_s16(sp.add(8 * i), v[16 + i]);
    }
    // rows 2, 6, … 30
    let x7b = x7.add(2 * x8);
    for i in 0..8 {
        v[16 + i] = ld8h(x7b.add(x8_4 * i));
        if CLEAR {
            zero8(x7b.add(x8_4 * i));
        }
    }
    for i in 24..32 {
        v[i] = vdupq_n_s16(0);
    }
    if SCALE {
        scale_input::<true>(v, 16..24);
    }
    idct32_odd(v);
    // store_addsub v31..v28, v27..v24, v23..v20, v19..v16
    for k in 0..16 {
        let r = v[31 - k];
        let e = vld1q_s16(sp.add(8 * k));
        vst1q_s16(sp.add(8 * k), sqadd::<true>(e, r));
        vst1q_s16(sp.add(8 * (31 - k)), sqsub::<true>(e, r));
    }
    // The odd inputs: offsets in rows from x7 (x7 then points at row 1).
    let groups: [[usize; 4]; 4] = [[1, 31, 17, 15], [7, 25, 23, 9], [5, 27, 21, 11], [3, 29, 19, 13]];
    let mut x6 = sp.add(8 * 32);
    for (g, rows) in groups.iter().enumerate() {
        for (j, &r) in rows.iter().enumerate() {
            v[16 + j] = ld8h(x7.add(x8 * r));
        }
        if CLEAR {
            for &r in rows {
                zero8(x7.add(x8 * r));
            }
        }
        if SCALE {
            scale_input::<true>(v, 16..20);
        }
        dct64_step1(v, x6, &C64[g]);
        x6 = x6.add(64);
    }
    dct64_step2(sp.add(8 * 32));
}

/// `inv_txfm_horz_dct_64x8_neon`: the last butterflies of the 64-point
/// DCT at `sp`, transposed into eight rows of 64 values at `x6`, with
/// `srshl` by `w12` (-2 or -1).
#[inline(always)]
unsafe fn horz_dct_64x8(v: &mut V, sp: *const i16, x6: *mut i16, w12: i16) {
    let mut x7 = sp;
    let mut x8 = sp.add(8 * 60);
    let mut x6 = x6;
    let mut x9 = x6.add(56);
    let v7 = vdupq_n_s16(w12);
    while x7 < x8 {
        for i in 0..4 {
            v[16 + i] = vld1q_s16(x7.add(8 * i));
            v[28 + i] = vld1q_s16(x8.add(8 * i));
        }
        x7 = x7.add(32);
        x8 = x8.sub(32);
        for i in 0..4 {
            v[20 + i] = vld1q_s16(x7.add(8 * i));
            v[24 + i] = vld1q_s16(x8.add(8 * i));
        }
        x7 = x7.add(32);
        x8 = x8.sub(32);
        transpose_8x8h(v, [16, 17, 18, 19, 20, 21, 22, 23], 4, 5);
        transpose_8x8h(v, [31, 30, 29, 28, 27, 26, 25, 24], 4, 5);
        for (s0, s1, s2, s3) in [
            (16, 31, 17, 30),
            (18, 29, 19, 28),
            (20, 27, 21, 26),
            (22, 25, 23, 24),
        ] {
            let v1 = vrshlq_s16(sqsub::<true>(v[s0], v[s1]), v7);
            let v0 = vrshlq_s16(sqadd::<true>(v[s0], v[s1]), v7);
            let v3 = vrshlq_s16(sqsub::<true>(v[s2], v[s3]), v7);
            let v2 = vrshlq_s16(sqadd::<true>(v[s2], v[s3]), v7);
            vst1q_s16(x6, v0);
            vst1q_s16(x9, rev8(v1));
            vst1q_s16(x6.add(64), v2);
            vst1q_s16(x9.add(64), rev8(v3));
            x6 = x6.add(128);
            x9 = x9.add(128);
        }
        x6 = x6.sub(8 * 64).add(8);
        x9 = x9.sub(8 * 64).sub(8);
    }
}

/// `inv_txfm_add_vert_dct_8x64_neon`: the last butterflies of the 64-point
/// DCT at `sp`, added to 64 rows of 8 pixels at `x6` (top and bottom halves
/// meeting in the middle).
#[inline(always)]
unsafe fn vert_dct_8x64(v: &mut V, sp: *const i16, x6: Dst, stride: isize) {
    let mut x7 = sp;
    let mut x8 = sp.add(8 * 60);
    let mut top = x6;
    let mut bottom = row(x6, stride, 63);
    while x7 < x8 {
        for i in 0..4 {
            v[16 + i] = vld1q_s16(x7.add(8 * i));
            v[28 + i] = vld1q_s16(x8.add(8 * i));
        }
        x7 = x7.add(32);
        x8 = x8.sub(32);
        for i in 0..4 {
            v[20 + i] = vld1q_s16(x7.add(8 * i));
            v[24 + i] = vld1q_s16(x8.add(8 * i));
        }
        x7 = x7.add(32);
        x8 = x8.sub(32);
        for (s0, s1, s2, s3) in [
            (16, 31, 17, 30),
            (18, 29, 19, 28),
            (20, 27, 21, 26),
            (22, 25, 23, 24),
        ] {
            add_store8::<4>(top, sqadd::<true>(v[s0], v[s1]));
            add_store8::<4>(bottom, sqsub::<true>(v[s0], v[s1]));
            add_store8::<4>(row(top, stride, 1), sqadd::<true>(v[s2], v[s3]));
            add_store8::<4>(row(bottom, stride, -1), sqsub::<true>(v[s2], v[s3]));
            top = row(top, stride, 2);
            bottom = row(bottom, stride, -2);
        }
    }
}

/// The scratch of the 64-point entry points (`sub_sp`), 64 vectors.
type Scratch = [i16; 64 * 8];

#[inline(always)]
unsafe fn itx_64x64_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<64, 64, 2>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut scratch: Scratch = [0; 64 * 8];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i16; 64 * 32];
    let x5 = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = 0;
    for i in [0usize, 8, 16, 24] {
        let x6 = x5.add(i * 64);
        if i > 0 && eob < w12 {
            zero_fill(x6, 32 - i, 2, 64);
            break;
        }
        dct_8h_x64::<true, false>(&mut v, sp, coeff.add(i), 32);
        horz_dct_64x8(&mut v, sp, x6, -2);
        if i < 24 {
            w12 = EOB_32X32[x13];
            x13 += 1;
        }
    }
    for i in (0..64).step_by(8) {
        dct_8h_x64::<false, false>(&mut v, sp, x5.add(i), 64);
        vert_dct_8x64(&mut v, sp, dst.add(i), stride);
    }
}

#[inline(always)]
unsafe fn itx_64x32_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<64, 32, 1>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut scratch: Scratch = [0; 64 * 8];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i16; 64 * 32];
    let x5 = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = 0;
    for i in [0usize, 8, 16, 24] {
        let x6 = x5.add(i * 64);
        if i > 0 && eob < w12 {
            zero_fill(x6, 32 - i, 2, 64);
            break;
        }
        dct_8h_x64::<true, true>(&mut v, sp, coeff.add(i), 32);
        horz_dct_64x8(&mut v, sp, x6, -1);
        if i < 24 {
            w12 = EOB_32X32[x13];
            x13 += 1;
        }
    }
    for i in (0..64).step_by(8) {
        vert_dct_8x32(&mut v, dst.add(i), stride, x5.add(i), 64);
    }
}

#[inline(always)]
unsafe fn itx_32x64_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<32, 64, 1>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut scratch: Scratch = [0; 64 * 8];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i16; 32 * 32];
    let x5 = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = EOB_32X32[x13];
    x13 += 1;
    for i in [0usize, 8, 16, 24] {
        let x6 = x5.add(i * 32);
        if i > 0 {
            if eob < w12 {
                zero_fill(x6, 32 - i, 4, 32);
                break;
            }
            if i < 24 {
                w12 = EOB_32X32[x13];
                x13 += 1;
            }
        }
        horz_dct_32x8::<true>(&mut v, x6, coeff.add(i), 32);
    }
    for i in [0usize, 8, 16, 24] {
        dct_8h_x64::<false, false>(&mut v, sp, x5.add(i), 32);
        vert_dct_8x64(&mut v, sp, dst.add(i), stride);
    }
}

#[inline(always)]
unsafe fn itx_64x16_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<64, 16, 2>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut scratch: Scratch = [0; 64 * 8];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i16; 64 * 16];
    let x4 = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = 0;
    for i in [0usize, 8] {
        let x6 = x4.add(i * 64);
        if i > 0 && eob < w12 {
            zero_fill(x6, 16 - i, 2, 64);
            break;
        }
        dct_8h_x64::<true, false>(&mut v, sp, coeff.add(i), 16);
        horz_dct_64x8(&mut v, sp, x6, -2);
        if i < 8 {
            w12 = EOB_16X32[x13];
            x13 += 1;
        }
    }
    for i in (0..64).step_by(8) {
        vert_8x16(&mut v, dst.add(i), stride, x4.add(i), 64, K::Dct);
    }
}

#[inline(always)]
unsafe fn itx_16x64_dct(dst: Dst, stride: isize, coeff: Coef, eob: c_int) {
    if eob == 0 {
        return idct_dc::<16, 64, 2>(dst, stride, coeff);
    }
    let mut v = regs();
    let mut scratch: Scratch = [0; 64 * 8];
    let sp = scratch.as_mut_ptr();
    let mut buf = [0i16; 16 * 32];
    let x5 = buf.as_mut_ptr();
    let mut x13 = 0;
    let mut w12 = EOB_16X32[x13];
    x13 += 1;
    for i in [0usize, 8, 16, 24] {
        let x6 = x5.add(i * 16);
        if i > 0 {
            if eob < w12 {
                zero_fill(x6, 32 - i, 8, 16);
                break;
            }
            if i < 24 {
                w12 = EOB_16X32[x13];
                x13 += 1;
            }
        }
        horz_16x8::<false>(&mut v, x6, coeff.add(i), 32, Some(K::Dct));
    }
    for i in [0usize, 8] {
        dct_8h_x64::<false, false>(&mut v, sp, x5.add(i), 16);
        vert_dct_8x64(&mut v, sp, dst.add(i), stride);
    }
}

// --- entry points (generated from the def_fn_* lists of itx.S) -----------

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_4x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x4(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_4x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x8(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_8x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x4(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_8x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x8(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_4x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_4x16(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_16x4_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x4(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_8x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x16(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_identity_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Adst, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_identity_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Flip, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_adst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Id, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_flipadst_16x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x8(dst, stride, coeff, eob, K::Id, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Dct, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Id, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_adst_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Dct, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_flipadst_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Dct, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_identity_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Dct, K::Id)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_dct_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Adst, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_adst_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Adst, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_adst_flipadst_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Adst, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_dct_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Flip, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_adst_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Flip, K::Adst)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_flipadst_flipadst_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Flip, K::Flip)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_dct_16x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x16(dst, stride, coeff, eob, K::Id, K::Dct)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_8x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_8x32_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_8x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    identity_832::<8, 32>(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_32x8_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_32x8_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    identity_832::<32, 8>(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x32_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_16x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    identity_32::<16, 32>(dst, stride, coeff, eob, &EOB_16X32_SHORTSIDE, &EOB_16X32)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_32x16_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_32x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    identity_32::<32, 16>(dst, stride, coeff, eob, &EOB_16X32, &EOB_16X32_SHORTSIDE)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_32x32_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_identity_identity_32x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    identity_32::<32, 32>(dst, stride, coeff, eob, &EOB_32X32, &EOB_32X32)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_16x64_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_16x64_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_32x64_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_32x64_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_64x16_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_64x16_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_64x32_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_64x32_dct(dst, stride, coeff, eob)
}

pub unsafe extern "C" fn dav1d_inv_txfm_add_dct_dct_64x64_8bpc_neon(
    dst: Dst,
    stride: isize,
    coeff: Coef,
    eob: c_int,
) {
    itx_64x64_dct(dst, stride, coeff, eob)
}
