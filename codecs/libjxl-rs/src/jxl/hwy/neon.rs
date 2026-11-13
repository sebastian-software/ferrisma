// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this source code is governed by Highway's offered BSD alternative in
// licenses/HIGHWAY-BSD-3-Clause.txt. Modified: translated the used HWY_NEON
// float32x4/int32x4 operations of hwy/ops/arm_neon-inl.h (AArch64 branches)
// and the generic_ops-inl.h definitions that NEON uses.
//! `HWY_NEON` on AArch64: four float lanes.
use super::SimdF32;
use core::arch::aarch64::*;

/// `HWY_FULL(float)` of `HWY_NEON` (AArch64 with AES; Highway's `HWY_NEON`
/// target, not `HWY_NEON_BF16`). NEON is part of the AArch64 baseline.
#[derive(Clone, Copy, Debug)]
pub struct Neon(());
impl Neon {
    pub fn new() -> Self {
        Self(())
    }
}
impl Default for Neon {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY (for every `neon!` block): NEON is a mandatory part of AArch64 and
// enabled by every aarch64 Rust target; these value intrinsics have no other
// precondition.
macro_rules! neon {
    ($e:expr) => {
        unsafe { $e }
    };
}

#[inline(always)]
fn bits(v: float32x4_t) -> uint32x4_t {
    neon!(vreinterpretq_u32_f32(v))
}
#[inline(always)]
fn float(v: uint32x4_t) -> float32x4_t {
    neon!(vreinterpretq_f32_u32(v))
}

impl SimdF32 for Neon {
    type V = float32x4_t;
    type I = int32x4_t;
    type M = uint32x4_t;
    const LANES: usize = 4;
    const NAME: &'static str = "NEON";

    #[inline(always)]
    fn set(self, v: f32) -> Self::V {
        neon!(vdupq_n_f32(v))
    }
    #[inline(always)]
    fn zero(self) -> Self::V {
        neon!(vdupq_n_f32(0.0))
    }
    #[inline(always)]
    fn load(self, src: &[f32]) -> Self::V {
        let src = &src[..4];
        // SAFETY: `src` holds four readable f32 values; vld1q has no alignment
        // requirement beyond that of f32.
        unsafe { vld1q_f32(src.as_ptr()) }
    }
    #[inline(always)]
    fn store(self, v: Self::V, dst: &mut [f32]) {
        let dst = &mut dst[..4];
        // SAFETY: `dst` holds four writable f32 values, exclusively borrowed.
        unsafe { vst1q_f32(dst.as_mut_ptr(), v) }
    }
    #[inline(always)]
    fn load_bits(self, src: &[u32]) -> Self::V {
        let src = &src[..4];
        // SAFETY: as in `load`, for four u32 values.
        float(unsafe { vld1q_u32(src.as_ptr()) })
    }
    #[inline(always)]
    fn store_bits(self, v: Self::V, dst: &mut [u32]) {
        let dst = &mut dst[..4];
        // SAFETY: as in `store`, for four u32 values.
        unsafe { vst1q_u32(dst.as_mut_ptr(), bits(v)) }
    }
    #[inline(always)]
    fn store_i32(self, v: Self::I, dst: &mut [i32]) {
        let dst = &mut dst[..4];
        // SAFETY: as in `store`, for four i32 values.
        unsafe { vst1q_s32(dst.as_mut_ptr(), v) }
    }
    #[inline(always)]
    fn get_lane(self, v: Self::V) -> f32 {
        neon!(vgetq_lane_f32::<0>(v))
    }

    #[inline(always)]
    fn add(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vaddq_f32(a, b))
    }
    #[inline(always)]
    fn sub(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vsubq_f32(a, b))
    }
    #[inline(always)]
    fn mul(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vmulq_f32(a, b))
    }
    // A64 has float64, so operator/ is vdiv (not the vrecpe iteration).
    #[inline(always)]
    fn div(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vdivq_f32(a, b))
    }
    // HWY_NATIVE_FMA: detail::MulAdd(add, mul, x) = vfma.
    #[inline(always)]
    fn mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        neon!(vfmaq_f32(add, mul, x))
    }
    #[inline(always)]
    fn neg_mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        neon!(vfmsq_f32(add, mul, x))
    }
    // MulSub(mul, x, sub) = MulAdd(mul, x, Neg(sub)).
    #[inline(always)]
    fn mul_sub(self, mul: Self::V, x: Self::V, sub: Self::V) -> Self::V {
        self.mul_add(mul, x, self.neg(sub))
    }
    // NegMulSub(mul, x, sub) = Neg(MulAdd(mul, x, sub)).
    #[inline(always)]
    fn neg_mul_sub(self, mul: Self::V, x: Self::V, sub: Self::V) -> Self::V {
        self.neg(self.mul_add(mul, x, sub))
    }
    // AArch64: IEEE minimumNumber/maximumNumber.
    #[inline(always)]
    fn min(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vminnmq_f32(a, b))
    }
    #[inline(always)]
    fn max(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vmaxnmq_f32(a, b))
    }
    #[inline(always)]
    fn abs(self, v: Self::V) -> Self::V {
        neon!(vabsq_f32(v))
    }
    #[inline(always)]
    fn neg(self, v: Self::V) -> Self::V {
        neon!(vnegq_f32(v))
    }
    #[inline(always)]
    fn abs_diff(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vabdq_f32(a, b))
    }
    #[inline(always)]
    fn sqrt(self, v: Self::V) -> Self::V {
        neon!(vsqrtq_f32(v))
    }
    #[inline(always)]
    fn approximate_reciprocal(self, v: Self::V) -> Self::V {
        neon!(vrecpeq_f32(v))
    }
    #[inline(always)]
    fn approximate_reciprocal_sqrt(self, v: Self::V) -> Self::V {
        neon!(vrsqrteq_f32(v))
    }
    #[inline(always)]
    fn floor(self, v: Self::V) -> Self::V {
        neon!(vrndmq_f32(v))
    }
    #[inline(always)]
    fn ceil(self, v: Self::V) -> Self::V {
        neon!(vrndpq_f32(v))
    }
    #[inline(always)]
    fn round(self, v: Self::V) -> Self::V {
        neon!(vrndnq_f32(v))
    }
    #[inline(always)]
    fn trunc(self, v: Self::V) -> Self::V {
        neon!(vrndq_f32(v))
    }
    // Generic: IfThenZeroElse(IsNegative(v), v); IsNegative broadcasts the
    // sign bit (BroadcastSignBit = arithmetic shift by 31).
    #[inline(always)]
    fn zero_if_negative(self, v: Self::V) -> Self::V {
        let negative = neon!(vreinterpretq_u32_s32(vshrq_n_s32::<31>(
            vreinterpretq_s32_f32(v)
        )));
        self.if_then_zero_else(negative, v)
    }
    // BitwiseIfThenElse(SignBit, sign, magn) = vbsl.
    #[inline(always)]
    fn copy_sign(self, magn: Self::V, sign: Self::V) -> Self::V {
        neon!(vbslq_f32(vdupq_n_u32(0x8000_0000), sign, magn))
    }
    // OrAnd(abs, SignBit, sign).
    #[inline(always)]
    fn copy_sign_to_abs(self, abs: Self::V, sign: Self::V) -> Self::V {
        float(neon!(vorrq_u32(
            bits(abs),
            vandq_u32(vdupq_n_u32(0x8000_0000), bits(sign)),
        )))
    }

    #[inline(always)]
    fn and(self, a: Self::V, b: Self::V) -> Self::V {
        float(neon!(vandq_u32(bits(a), bits(b))))
    }
    #[inline(always)]
    fn or(self, a: Self::V, b: Self::V) -> Self::V {
        float(neon!(vorrq_u32(bits(a), bits(b))))
    }
    #[inline(always)]
    fn xor(self, a: Self::V, b: Self::V) -> Self::V {
        float(neon!(veorq_u32(bits(a), bits(b))))
    }
    // AndNot(not_mask, mask) = vbic(mask, not_mask).
    #[inline(always)]
    fn and_not(self, a: Self::V, b: Self::V) -> Self::V {
        float(neon!(vbicq_u32(bits(b), bits(a))))
    }

    #[inline(always)]
    fn lt(self, a: Self::V, b: Self::V) -> Self::M {
        neon!(vcltq_f32(a, b))
    }
    #[inline(always)]
    fn le(self, a: Self::V, b: Self::V) -> Self::M {
        neon!(vcleq_f32(a, b))
    }
    #[inline(always)]
    fn gt(self, a: Self::V, b: Self::V) -> Self::M {
        neon!(vcgtq_f32(a, b))
    }
    #[inline(always)]
    fn ge(self, a: Self::V, b: Self::V) -> Self::M {
        neon!(vcgeq_f32(a, b))
    }
    #[inline(always)]
    fn eq(self, a: Self::V, b: Self::V) -> Self::M {
        neon!(vceqq_f32(a, b))
    }
    // operator!= is Not(operator==).
    #[inline(always)]
    fn ne(self, a: Self::V, b: Self::V) -> Self::M {
        neon!(vmvnq_u32(vceqq_f32(a, b)))
    }
    #[inline(always)]
    fn vec_from_mask(self, m: Self::M) -> Self::V {
        float(m)
    }
    #[inline(always)]
    fn if_then_else(self, m: Self::M, yes: Self::V, no: Self::V) -> Self::V {
        neon!(vbslq_f32(m, yes, no))
    }
    // AndNot(VecFromMask(mask), no).
    #[inline(always)]
    fn if_then_zero_else(self, m: Self::M, no: Self::V) -> Self::V {
        float(neon!(vbicq_u32(bits(no), m)))
    }
    // And(VecFromMask(mask), yes).
    #[inline(always)]
    fn if_then_else_zero(self, m: Self::M, yes: Self::V) -> Self::V {
        float(neon!(vandq_u32(bits(yes), m)))
    }

    // Clang >= 12 on AArch64: vcvtq_s32_f32 (fcvtzs saturates; NaN -> 0).
    #[inline(always)]
    fn convert_to_i32(self, v: Self::V) -> Self::I {
        neon!(vcvtq_s32_f32(v))
    }
    #[inline(always)]
    fn convert_to_f32(self, v: Self::I) -> Self::V {
        neon!(vcvtq_f32_s32(v))
    }
    #[inline(always)]
    fn nearest_int(self, v: Self::V) -> Self::I {
        neon!(vcvtnq_s32_f32(v))
    }
    #[inline(always)]
    fn bitcast_to_i32(self, v: Self::V) -> Self::I {
        neon!(vreinterpretq_s32_f32(v))
    }
    #[inline(always)]
    fn bitcast_to_f32(self, v: Self::I) -> Self::V {
        neon!(vreinterpretq_f32_s32(v))
    }

    #[inline(always)]
    fn add_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vaddq_s32(a, b))
    }
    #[inline(always)]
    fn sub_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vsubq_s32(a, b))
    }
    #[inline(always)]
    fn min_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vminq_s32(a, b))
    }
    #[inline(always)]
    fn max_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vmaxq_s32(a, b))
    }
    #[inline(always)]
    fn lt_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_u32(vcltq_s32(a, b)))
    }
    #[inline(always)]
    fn eq_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_u32(vceqq_s32(a, b)))
    }
    #[inline(always)]
    fn gt_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_u32(vcgtq_s32(a, b)))
    }
    #[inline(always)]
    fn shift_left_i32<const BITS: i32>(self, v: Self::I) -> Self::I {
        neon!(vshlq_n_s32::<BITS>(v))
    }
    #[inline(always)]
    fn shift_right_i32<const BITS: i32>(self, v: Self::I) -> Self::I {
        neon!(vshrq_n_s32::<BITS>(v))
    }
    #[inline(always)]
    fn shift_right_u32<const BITS: i32>(self, v: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_u32(vshrq_n_u32::<BITS>(
            vreinterpretq_u32_s32(v)
        )))
    }

    // Armv8: SumOfLanes = Set(d, ReduceSum) with ReduceSum = vaddv;
    // Min/MaxOfLanes = Set(d, vminv/vmaxv) (NaN-propagating, unlike Min/Max).
    #[inline(always)]
    fn sum_of_lanes(self, v: Self::V) -> Self::V {
        neon!(vdupq_n_f32(vaddvq_f32(v)))
    }
    #[inline(always)]
    fn max_of_lanes(self, v: Self::V) -> Self::V {
        neon!(vdupq_n_f32(vmaxvq_f32(v)))
    }
    #[inline(always)]
    fn min_of_lanes(self, v: Self::V) -> Self::V {
        neon!(vdupq_n_f32(vminvq_f32(v)))
    }

    #[inline(always)]
    fn interleave_lower(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vzip1q_f32(a, b))
    }
    #[inline(always)]
    fn interleave_upper(self, a: Self::V, b: Self::V) -> Self::V {
        neon!(vzip2q_f32(a, b))
    }
    #[inline(always)]
    fn store_interleaved2(self, a: Self::V, b: Self::V, dst: &mut [f32]) {
        let dst = &mut dst[..8];
        // SAFETY: `dst` holds eight writable f32 values, exclusively borrowed.
        unsafe { vst2q_f32(dst.as_mut_ptr(), float32x4x2_t(a, b)) }
    }
    #[inline(always)]
    fn store_interleaved3(self, a: Self::V, b: Self::V, c: Self::V, dst: &mut [f32]) {
        let dst = &mut dst[..12];
        // SAFETY: `dst` holds twelve writable f32 values, exclusively borrowed.
        unsafe { vst3q_f32(dst.as_mut_ptr(), float32x4x3_t(a, b, c)) }
    }

    // render stages
    // A full 128-bit vector: LoadDup128 is Load (vld1q).
    #[inline(always)]
    fn load_dup128(self, src: &[f32]) -> Self::V {
        self.load(src)
    }
    #[inline(always)]
    fn broadcast<const LANE: i32>(self, v: Self::V) -> Self::V {
        neon!(vdupq_laneq_f32::<LANE>(v))
    }
    // HWY_NEON_HAVE_F16C: DemoteTo(df16, v) = vcvt_f16_f32, the FCVTN
    // instruction (the f16 intrinsic types are not stable in Rust).
    #[inline(always)]
    fn store_demote_to_f16(self, v: Self::V, dst: &mut [u16]) {
        let dst = &mut dst[..4];
        let half: uint16x4_t;
        // SAFETY: FCVTN is part of the AArch64 baseline floating point and
        // reads/writes only the named vector registers; `dst` holds four
        // writable, exclusively borrowed u16 values.
        unsafe {
            core::arch::asm!(
                "fcvtn {0:v}.4h, {1:v}.4s",
                out(vreg) half,
                in(vreg) v,
                options(pure, nomem, nostack, preserves_flags)
            );
            vst1_u16(dst.as_mut_ptr(), half);
        }
    }
    // end render stages
    // modular
    #[inline(always)]
    fn set_i32(self, v: i32) -> Self::I {
        neon!(vdupq_n_s32(v))
    }
    #[inline(always)]
    fn load_i32(self, src: &[i32]) -> Self::I {
        let src = &src[..4];
        // SAFETY: `src` holds four readable i32 values; vld1q has no
        // alignment requirement beyond that of i32.
        unsafe { vld1q_s32(src.as_ptr()) }
    }
    #[inline(always)]
    fn and_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vandq_s32(a, b))
    }
    #[inline(always)]
    fn xor_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(veorq_s32(a, b))
    }
    #[inline(always)]
    fn abs_i32(self, v: Self::I) -> Self::I {
        neon!(vabsq_s32(v))
    }
    #[inline(always)]
    fn neg_i32(self, v: Self::I) -> Self::I {
        neon!(vnegq_s32(v))
    }
    // Ne = Not(Eq): vmvn of vceq.
    #[inline(always)]
    fn ne_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_u32(vmvnq_u32(vceqq_s32(a, b))))
    }
    #[inline(always)]
    fn if_then_else_i32(self, m: Self::I, yes: Self::I, no: Self::I) -> Self::I {
        neon!(vbslq_s32(vreinterpretq_u32_s32(m), yes, no))
    }
    // AndNot(VecFromMask(mask), no) = vbic(no, mask).
    #[inline(always)]
    fn if_then_zero_else_i32(self, m: Self::I, no: Self::I) -> Self::I {
        neon!(vbicq_s32(no, m))
    }
    // ConcatEven(d, a, a) (vuzp1), then vmull_s32 of the lower halves.
    #[inline(always)]
    fn mul_even_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_s64(vmull_s32(
            vget_low_s32(vuzp1q_s32(a, a)),
            vget_low_s32(vuzp1q_s32(b, b))
        )))
    }
    // ConcatOdd(d, a, a) (vuzp2), then vmull_s32 of the lower halves.
    #[inline(always)]
    fn mul_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_s64(vmull_s32(
            vget_low_s32(vuzp2q_s32(a, a)),
            vget_low_s32(vuzp2q_s32(b, b))
        )))
    }
    // AArch64: detail::InterleaveOdd = vtrn2.
    #[inline(always)]
    fn interleave_odd_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vtrn2q_s32(a, b))
    }
    // end modular
    // stages2
    #[inline(always)]
    fn mul_i32(self, a: Self::I, b: Self::I) -> Self::I {
        neon!(vmulq_s32(a, b))
    }
    #[inline(always)]
    fn table_lookup_bytes(self, bytes: Self::I, from: Self::I) -> Self::I {
        neon!(vreinterpretq_s32_u8(vqtbl1q_u8(
            vreinterpretq_u8_s32(bytes),
            vreinterpretq_u8_s32(from)
        )))
    }
    // end stages2
}
