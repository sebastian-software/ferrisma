// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this source code is governed by Highway's offered BSD alternative in
// licenses/HIGHWAY-BSD-3-Clause.txt. Modified: translated the used HWY_AVX2
// Vec256<float>/Vec256<int32_t> operations of hwy/ops/x86_256-inl.h, with
// the x86_128-inl.h and generic_ops-inl.h definitions that AVX2 uses.
//! `HWY_AVX2` on x86-64: eight float lanes.
use super::{Kernel, SimdF32};
use core::arch::x86_64::*;

/// `HWY_FULL(float)` of `HWY_AVX2`. A value exists only after run-time
/// detection of Highway's complete AVX2 group (`kGroupAVX2` in
/// hwy/targets.cc: SSE2..SSE4.2, CLMUL, AES, AVX, AVX2, BMI, BMI2, FMA, F16C,
/// LZCNT), so every intrinsic below runs on a CPU that supports it.
#[derive(Clone, Copy, Debug)]
pub struct Avx2(());
impl Avx2 {
    pub fn new() -> Option<Self> {
        let supported = std::is_x86_feature_detected!("sse2")
            && std::is_x86_feature_detected!("ssse3")
            && std::is_x86_feature_detected!("sse4.1")
            && std::is_x86_feature_detected!("sse4.2")
            && std::is_x86_feature_detected!("pclmulqdq")
            && std::is_x86_feature_detected!("aes")
            && std::is_x86_feature_detected!("avx")
            && std::is_x86_feature_detected!("avx2")
            && std::is_x86_feature_detected!("bmi1")
            && std::is_x86_feature_detected!("bmi2")
            && std::is_x86_feature_detected!("fma")
            && std::is_x86_feature_detected!("f16c")
            && std::is_x86_feature_detected!("lzcnt");
        supported.then_some(Self(()))
    }
}

/// Run a kernel body inside a function compiled with Highway's AVX2
/// attribute string (`HWY_TARGET_STR_AVX2`), as `HWY_ATTR` does.
#[target_feature(enable = "sse2,ssse3,sse4.1,sse4.2,pclmulqdq,aes,avx,avx2,bmi1,bmi2,fma,f16c")]
fn run_with_features<K: Kernel>(d: Avx2, kernel: K) -> K::Output {
    kernel.run(d)
}
#[inline]
pub(super) fn run<K: Kernel>(d: Avx2, kernel: K) -> K::Output {
    // SAFETY: the Avx2 token proves run-time support of every enabled feature.
    unsafe { run_with_features(d, kernel) }
}

// SAFETY (for every `unsafe` block below that calls a value intrinsic): an
// `Avx2` value exists only after `Avx2::new` detected all features of
// Highway's AVX2 group, which include each intrinsic's required features.
macro_rules! avx {
    ($e:expr) => {
        unsafe { $e }
    };
}

#[inline(always)]
fn sign_bit() -> __m256 {
    avx!(_mm256_castsi256_ps(_mm256_set1_epi32(i32::MIN)))
}

impl SimdF32 for Avx2 {
    type V = __m256;
    type I = __m256i;
    type M = __m256;
    const LANES: usize = 8;
    const NAME: &'static str = "AVX2";

    #[inline(always)]
    fn set(self, v: f32) -> Self::V {
        avx!(_mm256_set1_ps(v))
    }
    #[inline(always)]
    fn zero(self) -> Self::V {
        avx!(_mm256_setzero_ps())
    }
    #[inline(always)]
    fn load(self, src: &[f32]) -> Self::V {
        let src = &src[..8];
        // SAFETY: `src` holds eight readable f32 values; loadu has no
        // alignment requirement; AVX support as for every operation.
        unsafe { _mm256_loadu_ps(src.as_ptr()) }
    }
    #[inline(always)]
    fn store(self, v: Self::V, dst: &mut [f32]) {
        let dst = &mut dst[..8];
        // SAFETY: `dst` holds eight writable, exclusively borrowed f32 values.
        unsafe { _mm256_storeu_ps(dst.as_mut_ptr(), v) }
    }
    #[inline(always)]
    fn load_bits(self, src: &[u32]) -> Self::V {
        let src = &src[..8];
        // SAFETY: as in `load`, for eight u32 values.
        unsafe { _mm256_castsi256_ps(_mm256_loadu_si256(src.as_ptr().cast())) }
    }
    #[inline(always)]
    fn store_bits(self, v: Self::V, dst: &mut [u32]) {
        let dst = &mut dst[..8];
        // SAFETY: as in `store`, for eight u32 values.
        unsafe { _mm256_storeu_si256(dst.as_mut_ptr().cast(), _mm256_castps_si256(v)) }
    }
    #[inline(always)]
    fn store_i32(self, v: Self::I, dst: &mut [i32]) {
        let dst = &mut dst[..8];
        // SAFETY: as in `store`, for eight i32 values.
        unsafe { _mm256_storeu_si256(dst.as_mut_ptr().cast(), v) }
    }
    #[inline(always)]
    fn get_lane(self, v: Self::V) -> f32 {
        avx!(_mm256_cvtss_f32(v))
    }

    #[inline(always)]
    fn add(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_add_ps(a, b))
    }
    #[inline(always)]
    fn sub(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_sub_ps(a, b))
    }
    #[inline(always)]
    fn mul(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_mul_ps(a, b))
    }
    #[inline(always)]
    fn div(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_div_ps(a, b))
    }
    #[inline(always)]
    fn mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        avx!(_mm256_fmadd_ps(mul, x, add))
    }
    #[inline(always)]
    fn neg_mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        avx!(_mm256_fnmadd_ps(mul, x, add))
    }
    #[inline(always)]
    fn mul_sub(self, mul: Self::V, x: Self::V, sub: Self::V) -> Self::V {
        avx!(_mm256_fmsub_ps(mul, x, sub))
    }
    #[inline(always)]
    fn neg_mul_sub(self, mul: Self::V, x: Self::V, sub: Self::V) -> Self::V {
        avx!(_mm256_fnmsub_ps(mul, x, sub))
    }
    // minps/maxps: the second operand when either is NaN (or both are zero).
    #[inline(always)]
    fn min(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_min_ps(a, b))
    }
    #[inline(always)]
    fn max(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_max_ps(a, b))
    }
    // Generic x86 float Abs: v & ~SignMask.
    #[inline(always)]
    fn abs(self, v: Self::V) -> Self::V {
        avx!(_mm256_and_ps(
            v,
            _mm256_castsi256_ps(_mm256_set1_epi32(i32::MAX))
        ))
    }
    // Float Neg: Xor(v, SignBit(d)).
    #[inline(always)]
    fn neg(self, v: Self::V) -> Self::V {
        avx!(_mm256_xor_ps(v, sign_bit()))
    }
    // Float AbsDiff: Abs(a - b).
    #[inline(always)]
    fn abs_diff(self, a: Self::V, b: Self::V) -> Self::V {
        self.abs(self.sub(a, b))
    }
    #[inline(always)]
    fn sqrt(self, v: Self::V) -> Self::V {
        avx!(_mm256_sqrt_ps(v))
    }
    #[inline(always)]
    fn approximate_reciprocal(self, v: Self::V) -> Self::V {
        avx!(_mm256_rcp_ps(v))
    }
    #[inline(always)]
    fn approximate_reciprocal_sqrt(self, v: Self::V) -> Self::V {
        avx!(_mm256_rsqrt_ps(v))
    }
    #[inline(always)]
    fn floor(self, v: Self::V) -> Self::V {
        avx!(_mm256_round_ps::<
            { _MM_FROUND_TO_NEG_INF | _MM_FROUND_NO_EXC },
        >(v))
    }
    #[inline(always)]
    fn ceil(self, v: Self::V) -> Self::V {
        avx!(_mm256_round_ps::<
            { _MM_FROUND_TO_POS_INF | _MM_FROUND_NO_EXC },
        >(v))
    }
    #[inline(always)]
    fn round(self, v: Self::V) -> Self::V {
        avx!(_mm256_round_ps::<
            { _MM_FROUND_TO_NEAREST_INT | _MM_FROUND_NO_EXC },
        >(v))
    }
    #[inline(always)]
    fn trunc(self, v: Self::V) -> Self::V {
        avx!(_mm256_round_ps::<{ _MM_FROUND_TO_ZERO | _MM_FROUND_NO_EXC }>(v))
    }
    // IfThenElse(MaskFromVec(v), zero, v): blendv looks only at the MSB.
    #[inline(always)]
    fn zero_if_negative(self, v: Self::V) -> Self::V {
        avx!(_mm256_blendv_ps(v, _mm256_setzero_ps(), v))
    }
    // BitwiseIfThenElse(SignBit, sign, magn).
    #[inline(always)]
    fn copy_sign(self, magn: Self::V, sign: Self::V) -> Self::V {
        let msb = sign_bit();
        avx!(_mm256_or_ps(
            _mm256_and_ps(msb, sign),
            _mm256_andnot_ps(msb, magn)
        ))
    }
    // OrAnd(abs, SignBit, sign).
    #[inline(always)]
    fn copy_sign_to_abs(self, abs: Self::V, sign: Self::V) -> Self::V {
        avx!(_mm256_or_ps(abs, _mm256_and_ps(sign_bit(), sign)))
    }

    #[inline(always)]
    fn and(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_and_ps(a, b))
    }
    #[inline(always)]
    fn or(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_or_ps(a, b))
    }
    #[inline(always)]
    fn xor(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_xor_ps(a, b))
    }
    #[inline(always)]
    fn and_not(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_andnot_ps(a, b))
    }

    // operator< is b > a; operator<= is b >= a (ordered, quiet).
    #[inline(always)]
    fn lt(self, a: Self::V, b: Self::V) -> Self::M {
        avx!(_mm256_cmp_ps::<_CMP_GT_OQ>(b, a))
    }
    #[inline(always)]
    fn le(self, a: Self::V, b: Self::V) -> Self::M {
        avx!(_mm256_cmp_ps::<_CMP_GE_OQ>(b, a))
    }
    #[inline(always)]
    fn gt(self, a: Self::V, b: Self::V) -> Self::M {
        avx!(_mm256_cmp_ps::<_CMP_GT_OQ>(a, b))
    }
    #[inline(always)]
    fn ge(self, a: Self::V, b: Self::V) -> Self::M {
        avx!(_mm256_cmp_ps::<_CMP_GE_OQ>(a, b))
    }
    #[inline(always)]
    fn eq(self, a: Self::V, b: Self::V) -> Self::M {
        avx!(_mm256_cmp_ps::<_CMP_EQ_OQ>(a, b))
    }
    // Ordered: false when either operand is NaN (unlike NEON's Not(Eq)).
    #[inline(always)]
    fn ne(self, a: Self::V, b: Self::V) -> Self::M {
        avx!(_mm256_cmp_ps::<_CMP_NEQ_OQ>(a, b))
    }
    #[inline(always)]
    fn vec_from_mask(self, m: Self::M) -> Self::V {
        m
    }
    #[inline(always)]
    fn if_then_else(self, m: Self::M, yes: Self::V, no: Self::V) -> Self::V {
        avx!(_mm256_blendv_ps(no, yes, m))
    }
    #[inline(always)]
    fn if_then_zero_else(self, m: Self::M, no: Self::V) -> Self::V {
        avx!(_mm256_andnot_ps(m, no))
    }
    #[inline(always)]
    fn if_then_else_zero(self, m: Self::M, yes: Self::V) -> Self::V {
        avx!(_mm256_and_ps(m, yes))
    }

    // x86 ConvertTo(di, v): lanes >= 2^31 saturate to LimitsMax; cvttps2dq
    // returns 0x80000000 for the other out-of-range lanes and NaN.
    #[inline(always)]
    fn convert_to_i32(self, v: Self::V) -> Self::I {
        let overflow = avx!(_mm256_castps_si256(_mm256_cmp_ps::<_CMP_GE_OQ>(
            v,
            _mm256_set1_ps(2147483648.0)
        )));
        avx!(_mm256_blendv_epi8(
            _mm256_cvttps_epi32(v),
            _mm256_set1_epi32(i32::MAX),
            overflow
        ))
    }
    #[inline(always)]
    fn convert_to_f32(self, v: Self::I) -> Self::V {
        avx!(_mm256_cvtepi32_ps(v))
    }
    // As ConvertTo, with the rounding cvtps2dq.
    #[inline(always)]
    fn nearest_int(self, v: Self::V) -> Self::I {
        let overflow = avx!(_mm256_castps_si256(_mm256_cmp_ps::<_CMP_GE_OQ>(
            v,
            _mm256_set1_ps(2147483648.0)
        )));
        avx!(_mm256_blendv_epi8(
            _mm256_cvtps_epi32(v),
            _mm256_set1_epi32(i32::MAX),
            overflow
        ))
    }
    #[inline(always)]
    fn bitcast_to_i32(self, v: Self::V) -> Self::I {
        avx!(_mm256_castps_si256(v))
    }
    #[inline(always)]
    fn bitcast_to_f32(self, v: Self::I) -> Self::V {
        avx!(_mm256_castsi256_ps(v))
    }

    #[inline(always)]
    fn add_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_add_epi32(a, b))
    }
    #[inline(always)]
    fn sub_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_sub_epi32(a, b))
    }
    #[inline(always)]
    fn min_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_min_epi32(a, b))
    }
    #[inline(always)]
    fn max_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_max_epi32(a, b))
    }
    #[inline(always)]
    fn lt_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_cmpgt_epi32(b, a))
    }
    #[inline(always)]
    fn eq_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_cmpeq_epi32(a, b))
    }
    #[inline(always)]
    fn gt_i32(self, a: Self::I, b: Self::I) -> Self::I {
        avx!(_mm256_cmpgt_epi32(a, b))
    }
    #[inline(always)]
    fn shift_left_i32<const BITS: i32>(self, v: Self::I) -> Self::I {
        avx!(_mm256_slli_epi32::<BITS>(v))
    }
    #[inline(always)]
    fn shift_right_i32<const BITS: i32>(self, v: Self::I) -> Self::I {
        avx!(_mm256_srai_epi32::<BITS>(v))
    }
    #[inline(always)]
    fn shift_right_u32<const BITS: i32>(self, v: Self::I) -> Self::I {
        avx!(_mm256_srli_epi32::<BITS>(v))
    }

    // generic_ops: f(v, SwapAdjacentBlocks(v)), then within blocks
    // f(v3210, Reverse4) and f(that, Reverse2), with these operand orders.
    #[inline(always)]
    fn sum_of_lanes(self, v: Self::V) -> Self::V {
        reduce(v, |a, b| avx!(_mm256_add_ps(a, b)))
    }
    #[inline(always)]
    fn max_of_lanes(self, v: Self::V) -> Self::V {
        reduce(v, |a, b| avx!(_mm256_max_ps(a, b)))
    }
    #[inline(always)]
    fn min_of_lanes(self, v: Self::V) -> Self::V {
        reduce(v, |a, b| avx!(_mm256_min_ps(a, b)))
    }

    // Per 128-bit block, as Highway's AVX2 InterleaveLower/Upper.
    #[inline(always)]
    fn interleave_lower(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_unpacklo_ps(a, b))
    }
    #[inline(always)]
    fn interleave_upper(self, a: Self::V, b: Self::V) -> Self::V {
        avx!(_mm256_unpackhi_ps(a, b))
    }
    // generic_ops StoreInterleaved2 for 32-byte vectors.
    #[inline(always)]
    fn store_interleaved2(self, a: Self::V, b: Self::V, dst: &mut [f32]) {
        let lower = self.interleave_lower(a, b);
        let upper = self.interleave_upper(a, b);
        // ConcatLowerLower(d, upper, lower), ConcatUpperUpper(d, upper, lower).
        self.store(
            avx!(_mm256_permute2f128_ps::<0x20>(lower, upper)),
            &mut dst[..8],
        );
        self.store(
            avx!(_mm256_permute2f128_ps::<0x31>(lower, upper)),
            &mut dst[8..16],
        );
    }
    // Pure data movement: the stored words are fixed by the definition
    // (a0 b0 c0 a1 ...), whatever shuffle sequence produces them.
    #[inline(always)]
    fn store_interleaved3(self, a: Self::V, b: Self::V, c: Self::V, dst: &mut [f32]) {
        let mut lanes = [[0.0f32; 8]; 3];
        self.store(a, &mut lanes[0]);
        self.store(b, &mut lanes[1]);
        self.store(c, &mut lanes[2]);
        let dst = &mut dst[..24];
        for i in 0..8 {
            for (j, plane) in lanes.iter().enumerate() {
                dst[3 * i + j] = plane[i];
            }
        }
    }

    // render stages
    #[inline(always)]
    fn load_dup128(self, src: &[f32]) -> Self::V {
        let src = &src[..4];
        // SAFETY: `src` holds four readable f32 values; loadu has no
        // alignment requirement; AVX support as for every operation.
        unsafe { _mm256_broadcast_ps(&*(src.as_ptr() as *const __m128)) }
    }
    #[inline(always)]
    fn broadcast<const LANE: i32>(self, v: Self::V) -> Self::V {
        // _mm256_shuffle_ps(v, v, 0x55 * LANE), per 128-bit block.
        match LANE {
            0 => avx!(_mm256_shuffle_ps::<0x00>(v, v)),
            1 => avx!(_mm256_shuffle_ps::<0x55>(v, v)),
            2 => avx!(_mm256_shuffle_ps::<0xaa>(v, v)),
            3 => avx!(_mm256_shuffle_ps::<0xff>(v, v)),
            _ => panic!("Broadcast lane {LANE} outside a 128-bit block"),
        }
    }
    // x86_256 DemoteTo(df16, v): _mm256_cvtps_ph(v, _MM_FROUND_NO_EXC), that
    // is round to nearest even with exceptions suppressed. Rust's immediate
    // admits only the rounding bits; exceptions are masked in both builds, so
    // 0 (round to nearest even) yields the same words.
    #[inline(always)]
    fn store_demote_to_f16(self, v: Self::V, dst: &mut [u16]) {
        let dst = &mut dst[..8];
        // SAFETY: `dst` holds eight writable, exclusively borrowed u16 values;
        // F16C is part of Highway's AVX2 group, as for every operation.
        unsafe { _mm_storeu_si128(dst.as_mut_ptr().cast(), _mm256_cvtps_ph::<0>(v)) }
    }
    // end render stages
}

#[inline(always)]
fn reduce(v: __m256, f: impl Fn(__m256, __m256) -> __m256) -> __m256 {
    let v = f(v, avx!(_mm256_permute2f128_ps::<0x01>(v, v)));
    let v0123 = avx!(_mm256_shuffle_ps::<0x1b>(v, v));
    let v03_12_12_03 = f(v, v0123);
    let v12_03_03_12 = avx!(_mm256_shuffle_ps::<0xb1>(v03_12_12_03, v03_12_12_03));
    f(v03_12_12_03, v12_03_03_12)
}
