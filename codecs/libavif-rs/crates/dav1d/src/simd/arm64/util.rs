// Copyright © 2015, Martin Storsjo
// Copyright © 2015, VideoLAN and dav1d authors
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Register model and the macros of `upstream/dav1d/src/arm/64/util.S` that
//! the rebuilt kernels use, as Rust intrinsics (engine ADR-0044 D2,
//! libavif-rs ADR 0062).
//!
//! The assembly works on the 32 vector registers `v0`–`v31`. A kernel keeps
//! them as a local array `V` of `int16x8_t` with the register's number as
//! index, so that each Rust line reads against its assembly line. An
//! instruction on a 64-bit arrangement (`.4h`, `.8b`, `.2s`) writes the low
//! half of its destination and clears the high half, as AArch64 does; the
//! helpers with a `Q` parameter model that with `Q = false`. Lane-wise
//! instructions give each lane the same value whatever order they are issued
//! in, so the helpers compute a whole instruction at once.
use core::arch::aarch64::*;

/// The vector register file `v0`–`v31` of one kernel invocation.
pub type V = [int16x8_t; 32];

/// A register file whose registers are all zero.
#[inline(always)]
pub fn regs() -> V {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { [vdupq_n_s16(0); 32] }
}

/// The value a 64-bit destination receives: `x` in the low half, the high
/// half cleared.
#[inline(always)]
pub fn d(x: int16x4_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vcombine_s16(x, vdup_n_s16(0)) }
}

#[inline(always)]
pub fn lo(x: int16x8_t) -> int16x4_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vget_low_s16(x) }
}

/// `sqadd rd.8h/.4h, rn, rm`
#[inline(always)]
pub fn sqadd<const Q: bool>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        if Q {
            vqaddq_s16(a, b)
        } else {
            d(vqadd_s16(lo(a), lo(b)))
        }
    }
}

/// `sqsub rd.8h/.4h, rn, rm`
#[inline(always)]
pub fn sqsub<const Q: bool>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        if Q {
            vqsubq_s16(a, b)
        } else {
            d(vqsub_s16(lo(a), lo(b)))
        }
    }
}

/// `sqneg rd.8h/.4h, rn`
#[inline(always)]
pub fn sqneg<const Q: bool>(a: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        if Q {
            vqnegq_s16(a)
        } else {
            d(vqneg_s16(lo(a)))
        }
    }
}

/// `sqrdmulh rd.8h/.4h, rn, vm.h[i]`, with the element's value `c`.
#[inline(always)]
pub fn sqrdmulh<const Q: bool>(a: int16x8_t, c: i16) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        if Q {
            vqrdmulhq_n_s16(a, c)
        } else {
            d(vqrdmulh_n_s16(lo(a), c))
        }
    }
}

/// `sqshl rd.8h/.4h, rn, #1`
#[inline(always)]
pub fn sqshl1<const Q: bool>(a: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        if Q {
            vqshlq_n_s16::<1>(a)
        } else {
            d(vqshl_n_s16::<1>(lo(a)))
        }
    }
}

/// `srshr rd.8h, rn, #N`
#[inline(always)]
pub fn srshr<const N: i32>(a: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vrshrq_n_s16::<N>(a) }
}

/// `sshr rd.8h, rn, #N`
#[inline(always)]
pub fn sshr<const N: i32>(a: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vshrq_n_s16::<N>(a) }
}

/// `srhadd rd.8h, rn, rm`
#[inline(always)]
pub fn srhadd(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vrhaddq_s16(a, b) }
}

/// `ins rd.d[1], rn.d[0]`
#[inline(always)]
pub fn ins_hi_lo(rd: int16x8_t, rn: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vcombine_s16(lo(rd), lo(rn)) }
}

/// `ins rd.d[0], rn.d[1]`
#[inline(always)]
pub fn ins_lo_hi(rd: int16x8_t, rn: int16x8_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vcombine_s16(vget_high_s16(rn), vget_high_s16(rd)) }
}

/// `smull_smlal`/`smull_smlsl` of itx.S (with `SUB`) followed by the
/// `sqrshrn_sz #12` that narrows its two 32-bit halves into one register:
/// `(s0 * c0 ± s1 * c1 + 2048) >> 12`, saturated to 16 bits, per lane. With
/// `NEG` the 32-bit products are negated (`neg v.4s`) before narrowing.
#[inline(always)]
pub fn mul2<const Q: bool, const SUB: bool, const NEG: bool>(
    s0: int16x8_t,
    s1: int16x8_t,
    c0: i16,
    c1: i16,
) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let mut l = vmull_n_s16(lo(s0), c0);
        l = if SUB {
            vmlsl_n_s16(l, lo(s1), c1)
        } else {
            vmlal_n_s16(l, lo(s1), c1)
        };
        if NEG {
            l = vnegq_s32(l);
        }
        let nl = vqrshrn_n_s32::<12>(l);
        if !Q {
            return d(nl);
        }
        let mut h = vmull_high_n_s16(s0, c0);
        h = if SUB {
            vmlsl_high_n_s16(h, s1, c1)
        } else {
            vmlal_high_n_s16(h, s1, c1)
        };
        if NEG {
            h = vnegq_s32(h);
        }
        vqrshrn_high_n_s32::<12>(nl, h)
    }
}

#[inline(always)]
fn s32(x: int16x8_t) -> int32x4_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vreinterpretq_s32_s16(x) }
}

#[inline(always)]
fn s64(x: int16x8_t) -> int64x2_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vreinterpretq_s64_s16(x) }
}

#[inline(always)]
fn h32(x: int32x4_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vreinterpretq_s16_s32(x) }
}

#[inline(always)]
fn h64(x: int64x2_t) -> int16x8_t {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe { vreinterpretq_s16_s64(x) }
}

/// `transpose_4x4h r0, r1, r2, r3, t4, t5, t6, t7` (util.S)
#[inline(always)]
pub fn transpose_4x4h(v: &mut V, r: [usize; 4], t: [usize; 4]) {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let [r0, r1, r2, r3] = r;
        let [t4, t5, t6, t7] = t;
        v[t4] = d(vtrn1_s16(lo(v[r0]), lo(v[r1])));
        v[t5] = d(vtrn2_s16(lo(v[r0]), lo(v[r1])));
        v[t6] = d(vtrn1_s16(lo(v[r2]), lo(v[r3])));
        v[t7] = d(vtrn2_s16(lo(v[r2]), lo(v[r3])));

        let s = |x: int16x8_t| vreinterpret_s32_s16(lo(x));
        let b = |x: int32x2_t| d(vreinterpret_s16_s32(x));
        v[r0] = b(vtrn1_s32(s(v[t4]), s(v[t6])));
        v[r2] = b(vtrn2_s32(s(v[t4]), s(v[t6])));
        v[r1] = b(vtrn1_s32(s(v[t5]), s(v[t7])));
        v[r3] = b(vtrn2_s32(s(v[t5]), s(v[t7])));
    }
}

/// `transpose_4x8h_mov r0, r1, r2, r3, t4, t5, t6, t7, o0, o1, o2, o3`
/// (util.S); `transpose_4x8h` is the form with `o = r`.
#[inline(always)]
pub fn transpose_4x8h_mov(v: &mut V, r: [usize; 4], t: [usize; 4], o: [usize; 4]) {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let [r0, r1, r2, r3] = r;
        let [t4, t5, t6, t7] = t;
        let [o0, o1, o2, o3] = o;
        v[t4] = vtrn1q_s16(v[r0], v[r1]);
        v[t5] = vtrn2q_s16(v[r0], v[r1]);
        v[t6] = vtrn1q_s16(v[r2], v[r3]);
        v[t7] = vtrn2q_s16(v[r2], v[r3]);

        v[o0] = h32(vtrn1q_s32(s32(v[t4]), s32(v[t6])));
        v[o2] = h32(vtrn2q_s32(s32(v[t4]), s32(v[t6])));
        v[o1] = h32(vtrn1q_s32(s32(v[t5]), s32(v[t7])));
        v[o3] = h32(vtrn2q_s32(s32(v[t5]), s32(v[t7])));
    }
}

/// `transpose_4x8h r0, r1, r2, r3, t4, t5, t6, t7` (util.S)
#[inline(always)]
pub fn transpose_4x8h(v: &mut V, r: [usize; 4], t: [usize; 4]) {
    transpose_4x8h_mov(v, r, t, r);
}

/// `transpose_8x8h_mov r0..r7, t8, t9, o0..o7` (util.S); `transpose_8x8h`
/// is the form with `o = r`.
#[inline(always)]
pub fn transpose_8x8h_mov(v: &mut V, r: [usize; 8], t8: usize, t9: usize, o: [usize; 8]) {
    // SAFETY: NEON value intrinsics; NEON is baseline on AArch64.
    unsafe {
        let [r0, r1, r2, r3, r4, r5, r6, r7] = r;
        v[t8] = vtrn1q_s16(v[r0], v[r1]);
        v[t9] = vtrn2q_s16(v[r0], v[r1]);
        v[r1] = vtrn1q_s16(v[r2], v[r3]);
        v[r3] = vtrn2q_s16(v[r2], v[r3]);
        v[r0] = vtrn1q_s16(v[r4], v[r5]);
        v[r5] = vtrn2q_s16(v[r4], v[r5]);
        v[r2] = vtrn1q_s16(v[r6], v[r7]);
        v[r7] = vtrn2q_s16(v[r6], v[r7]);

        v[r4] = h32(vtrn1q_s32(s32(v[r0]), s32(v[r2])));
        v[r2] = h32(vtrn2q_s32(s32(v[r0]), s32(v[r2])));
        v[r6] = h32(vtrn1q_s32(s32(v[r5]), s32(v[r7])));
        v[r7] = h32(vtrn2q_s32(s32(v[r5]), s32(v[r7])));
        v[r5] = h32(vtrn1q_s32(s32(v[t9]), s32(v[r3])));
        v[t9] = h32(vtrn2q_s32(s32(v[t9]), s32(v[r3])));
        v[r3] = h32(vtrn1q_s32(s32(v[t8]), s32(v[r1])));
        v[t8] = h32(vtrn2q_s32(s32(v[t8]), s32(v[r1])));

        let [o0, o1, o2, o3, o4, o5, o6, o7] = o;
        // The order of the reads matters only where an output register is
        // also a later input (o = r); it follows util.S.
        let (a3, a4) = (v[r3], v[r4]);
        v[o0] = h64(vtrn1q_s64(s64(a3), s64(a4)));
        v[o4] = h64(vtrn2q_s64(s64(a3), s64(a4)));
        let (a5, a6) = (v[r5], v[r6]);
        v[o1] = h64(vtrn1q_s64(s64(a5), s64(a6)));
        v[o5] = h64(vtrn2q_s64(s64(a5), s64(a6)));
        let (a8, a2) = (v[t8], v[r2]);
        v[o6] = h64(vtrn2q_s64(s64(a8), s64(a2)));
        v[o2] = h64(vtrn1q_s64(s64(a8), s64(a2)));
        let (a9, a7) = (v[t9], v[r7]);
        v[o3] = h64(vtrn1q_s64(s64(a9), s64(a7)));
        v[o7] = h64(vtrn2q_s64(s64(a9), s64(a7)));
    }
}

/// `transpose_8x8h r0..r7, t8, t9` (util.S)
#[inline(always)]
pub fn transpose_8x8h(v: &mut V, r: [usize; 8], t8: usize, t9: usize) {
    transpose_8x8h_mov(v, r, t8, t9, r);
}
