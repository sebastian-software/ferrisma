//! The `Lanes` abstraction and its per-level implementations.
//!
//! A vector holds `SLOTS` RGBA pixels of four `f32` channels each. Methods
//! are `#[inline(always)]` and are only ever called from kernel bodies that
//! the `dispatch` module instantiates inside a `#[target_feature]` entry for
//! the same level, after runtime detection. That is the invariant every
//! `unsafe` block below relies on: the types `Neon` and `Avx2` are private
//! to this crate, and `dispatch` is the only module that names them.
//! Loads and stores take slices and check the length before touching memory.

#![allow(unsafe_code)]

pub(crate) trait Lanes {
    /// RGBA pixels per vector.
    const SLOTS: usize;
    type V: Copy;
    fn zero() -> Self::V;
    /// Load `4 * SLOTS` values from the front of `values`.
    fn load(values: &[f32]) -> Self::V;
    /// Store `4 * SLOTS` values to the front of `values`.
    fn store(values: &mut [f32], vector: Self::V);
    fn splat(value: f32) -> Self::V;
    /// Slot `s` holds `weights[s]` in all four channels.
    fn slot_weights(weights: &[f32]) -> Self::V;
    /// `a * b + acc`, fused, per lane.
    fn fma(acc: Self::V, a: Self::V, b: Self::V) -> Self::V;
    /// Sum of the slots, slot 0 first.
    fn reduce_slots(vector: Self::V) -> [f32; 4];
}

/// `0.5_f64.next_down()`: for every `y >= 0.0` in `[0, 65535]`,
/// `trunc(y + HALF_DOWN)` equals `y.round() as u16` (half away from zero).
const HALF_DOWN: f64 = f64::from_bits(0x3FDF_FFFF_FFFF_FFFF);

/// `f64` lane abstraction for high-precision sRGB transfer kernels.
///
/// `Scalar` evaluates `mul_add` as `a * b + acc` so baseline x86-64 (SSE2
/// without hardware FMA) avoids out-of-line `libm::fma` calls; SIMD levels
/// use hardware FMA. Because `linear_to_srgb_u16` refines every curved-segment
/// lane inside `HALF_CODE_LIMIT` with the exact `f64` OETF, `linear_to_srgb_u16`
/// is bit-identical to the `f64` oracle at every ISA level. `srgb_to_linear`
/// agrees within `<= 1` ULP of `f32` between `Scalar` and SIMD (and against
/// `f64::powf`), and is bit-identical across all SIMD levels.
pub(crate) trait LanesF64 {
    /// `f64` elements per vector (`1` Scalar, `2` Neon, `4` Avx2, `8` Avx512).
    const WIDTH: usize;
    type V: Copy;
    fn splat(value: f64) -> Self::V;
    /// Load `WIDTH` `f32` samples from the front of `values` and widen to `f64`.
    fn load_f32(values: &[f32]) -> Self::V;
    /// Store `WIDTH` `f64` samples to the front of `values`.
    fn store_f64(values: &mut [f64], vector: Self::V);
    /// Round `WIDTH` non-negative `f64` values in `[0, 65535]` half away from
    /// zero to `u16` and store to the front of `values`. Returns a per-lane
    /// refinement bitmask where bit `i` is set iff lane `i` satisfies
    /// `|vector[i] - round(vector[i])| > limit[i]` (`0` means all lanes are
    /// final).
    fn store_u16_round_checked(values: &mut [u16], vector: Self::V, limit: Self::V) -> u8;
    fn mul(a: Self::V, b: Self::V) -> Self::V;
    /// `a * b + acc` per lane (separate mul+add on `Scalar`, hardware FMA on SIMD).
    fn mul_add(acc: Self::V, a: Self::V, b: Self::V) -> Self::V;
    /// Scalar counterpart of [`Self::mul_add`] for tails and mixed-range vectors,
    /// matching the level's vector arithmetic bit for bit (separate mul+add on
    /// `Scalar`, hardware `f64::mul_add` inside SIMD `#[target_feature]` entries).
    fn scalar_mul_add(acc: f64, a: f64, b: f64) -> f64;
    fn sqrt(a: Self::V) -> Self::V;
    fn abs(a: Self::V) -> Self::V;
    /// Magnitude of `mag` with the sign bit of `sign`.
    fn copysign(mag: Self::V, sign: Self::V) -> Self::V;
    /// Clamp each lane to `[0.0, 1.0]`, mapping `NaN` to `0.0`.
    fn clamp01(a: Self::V) -> Self::V;
    /// Per lane: `if a <= b { if_le } else { if_gt }`.
    fn select_le(a: Self::V, b: Self::V, if_le: Self::V, if_gt: Self::V) -> Self::V;
    /// `true` iff every lane satisfies `a <= b` (`false` if any lane is `> b` or `NaN`).
    fn all_le(a: Self::V, b: Self::V) -> bool;
}

/// Portable reference: one pixel per "vector".
pub(crate) struct Scalar;

impl Lanes for Scalar {
    const SLOTS: usize = 1;
    type V = [f32; 4];
    #[inline(always)]
    fn zero() -> [f32; 4] {
        [0.0; 4]
    }
    #[inline(always)]
    fn load(values: &[f32]) -> [f32; 4] {
        *values
            .first_chunk::<4>()
            .expect("caller provides four values")
    }
    #[inline(always)]
    fn store(values: &mut [f32], vector: [f32; 4]) {
        *values
            .first_chunk_mut::<4>()
            .expect("caller provides four values") = vector;
    }
    #[inline(always)]
    fn splat(value: f32) -> [f32; 4] {
        [value; 4]
    }
    #[inline(always)]
    fn slot_weights(weights: &[f32]) -> [f32; 4] {
        [weights[0]; 4]
    }
    #[inline(always)]
    fn fma(acc: [f32; 4], a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
        [
            a[0].mul_add(b[0], acc[0]),
            a[1].mul_add(b[1], acc[1]),
            a[2].mul_add(b[2], acc[2]),
            a[3].mul_add(b[3], acc[3]),
        ]
    }
    #[inline(always)]
    fn reduce_slots(vector: [f32; 4]) -> [f32; 4] {
        vector
    }
}

impl LanesF64 for Scalar {
    const WIDTH: usize = 1;
    type V = f64;
    #[inline(always)]
    fn splat(value: f64) -> f64 {
        value
    }
    #[inline(always)]
    fn load_f32(values: &[f32]) -> f64 {
        f64::from(values[0])
    }
    #[inline(always)]
    fn store_f64(values: &mut [f64], vector: f64) {
        values[0] = vector;
    }
    #[inline(always)]
    fn store_u16_round_checked(values: &mut [u16], vector: f64, limit: f64) -> u8 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let code = (vector + HALF_DOWN) as u16;
        values[0] = code;
        u8::from((vector - f64::from(code)).abs() > limit)
    }
    #[inline(always)]
    fn mul(a: f64, b: f64) -> f64 {
        a * b
    }
    #[inline(always)]
    fn mul_add(acc: f64, a: f64, b: f64) -> f64 {
        a * b + acc
    }
    #[inline(always)]
    fn scalar_mul_add(acc: f64, a: f64, b: f64) -> f64 {
        a * b + acc
    }
    #[inline(always)]
    fn sqrt(a: f64) -> f64 {
        a.sqrt()
    }
    #[inline(always)]
    fn abs(a: f64) -> f64 {
        a.abs()
    }
    #[inline(always)]
    fn copysign(mag: f64, sign: f64) -> f64 {
        mag.copysign(sign)
    }
    #[inline(always)]
    fn clamp01(a: f64) -> f64 {
        // `f64::max` maps NaN to 0.0, matching NEON `vmaxnmq_f64` and x86 `maxpd(a, 0.0)`.
        #[allow(clippy::manual_clamp)]
        {
            a.max(0.0).min(1.0)
        }
    }
    #[inline(always)]
    fn select_le(a: f64, b: f64, if_le: f64, if_gt: f64) -> f64 {
        if a <= b { if_le } else { if_gt }
    }
    #[inline(always)]
    fn all_le(a: f64, b: f64) -> bool {
        a <= b
    }
}

#[cfg(target_arch = "aarch64")]
pub(crate) use neon::Neon;

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::{Lanes, LanesF64};
    use std::arch::aarch64::{
        float32x4_t, float64x2_t, vabsq_f64, vbslq_f64, vcgtq_f64, vcleq_f64, vcvt_f64_f32,
        vcvtq_u64_f64, vdupq_n_f32, vdupq_n_f64, vdupq_n_u64, vfmaq_f32, vfmaq_f64, vget_lane_u64,
        vgetq_lane_u64, vld1_f32, vld1q_f32, vmaxnmq_f64, vminnmq_f64, vminvq_u32, vmovn_u64,
        vmulq_f64, vreinterpret_u64_u32, vreinterpretq_u32_u64, vrndaq_f64, vsqrtq_f64, vst1q_f32,
        vst1q_f64, vsubq_f64,
    };

    /// 128-bit Advanced SIMD, one pixel per vector. Same operation order as
    /// `Scalar` (fused multiply-add per channel), hence bit-identical.
    pub(crate) struct Neon;

    impl Lanes for Neon {
        const SLOTS: usize = 1;
        type V = float32x4_t;
        #[inline(always)]
        fn zero() -> float32x4_t {
            // SAFETY: only reached inside a `neon` target-feature entry
            // after detection (module invariant).
            unsafe { vdupq_n_f32(0.0) }
        }
        #[inline(always)]
        fn load(values: &[f32]) -> float32x4_t {
            let values = values
                .first_chunk::<4>()
                .expect("caller provides four values");
            // SAFETY: `values` is four valid, initialized `f32`; NEON is
            // available (module invariant). `vld1q_f32` has no alignment
            // requirement beyond `f32`.
            unsafe { vld1q_f32(values.as_ptr()) }
        }
        #[inline(always)]
        fn store(values: &mut [f32], vector: float32x4_t) {
            let values = values
                .first_chunk_mut::<4>()
                .expect("caller provides four values");
            // SAFETY: `values` is four writable `f32`; NEON is available.
            unsafe { vst1q_f32(values.as_mut_ptr(), vector) }
        }
        #[inline(always)]
        fn splat(value: f32) -> float32x4_t {
            // SAFETY: NEON is available (module invariant).
            unsafe { vdupq_n_f32(value) }
        }
        #[inline(always)]
        fn slot_weights(weights: &[f32]) -> float32x4_t {
            <Self as Lanes>::splat(weights[0])
        }
        #[inline(always)]
        fn fma(acc: float32x4_t, a: float32x4_t, b: float32x4_t) -> float32x4_t {
            // SAFETY: NEON is available. `vfmaq_f32(acc, a, b)` is
            // `acc + a * b` with a single rounding.
            unsafe { vfmaq_f32(acc, a, b) }
        }
        #[inline(always)]
        fn reduce_slots(vector: float32x4_t) -> [f32; 4] {
            let mut out = [0.0; 4];
            Self::store(&mut out, vector);
            out
        }
    }

    impl LanesF64 for Neon {
        const WIDTH: usize = 2;
        type V = float64x2_t;
        #[inline(always)]
        fn splat(value: f64) -> float64x2_t {
            // SAFETY: NEON is available (module invariant).
            unsafe { vdupq_n_f64(value) }
        }
        #[inline(always)]
        fn load_f32(values: &[f32]) -> float64x2_t {
            let values = values
                .first_chunk::<2>()
                .expect("caller provides two values");
            // SAFETY: `values` is two valid, initialized `f32`; NEON is available.
            let pair = unsafe { vld1_f32(values.as_ptr()) };
            // SAFETY: NEON is available (module invariant).
            unsafe { vcvt_f64_f32(pair) }
        }
        #[inline(always)]
        fn store_f64(values: &mut [f64], vector: float64x2_t) {
            let values = values
                .first_chunk_mut::<2>()
                .expect("caller provides two values");
            // SAFETY: `values` is two writable `f64`; NEON is available.
            unsafe { vst1q_f64(values.as_mut_ptr(), vector) }
        }
        #[inline(always)]
        fn store_u16_round_checked(
            values: &mut [u16],
            vector: float64x2_t,
            limit: float64x2_t,
        ) -> u8 {
            let out = values
                .first_chunk_mut::<2>()
                .expect("caller provides two values");
            // SAFETY: NEON is available; `vrndaq_f64` rounds half away from zero.
            let rounded = unsafe { vrndaq_f64(vector) };
            // SAFETY: NEON is available; inputs are non-negative and <= 65535.
            let ints = unsafe { vcvtq_u64_f64(rounded) };
            // SAFETY: NEON is available.
            let lane0 = unsafe { vgetq_lane_u64::<0>(ints) };
            // SAFETY: NEON is available.
            let lane1 = unsafe { vgetq_lane_u64::<1>(ints) };
            #[allow(clippy::cast_possible_truncation)]
            {
                out[0] = lane0 as u16;
                out[1] = lane1 as u16;
            }
            // SAFETY: NEON is available (module invariant).
            let delta = unsafe { vsubq_f64(vector, rounded) };
            let abs_delta = <Self as LanesF64>::abs(delta);
            // SAFETY: NEON is available (module invariant).
            let gt = unsafe { vcgtq_f64(abs_delta, limit) };
            // SAFETY: NEON is available; narrows each 64-bit mask lane to 32 bits.
            let narrowed = unsafe { vmovn_u64(gt) };
            // SAFETY: NEON is available (module invariant).
            let packed = unsafe { vreinterpret_u64_u32(narrowed) };
            // SAFETY: NEON is available; extracts both 32-bit lane masks in one move.
            let bits = unsafe { vget_lane_u64::<0>(packed) };
            #[allow(clippy::cast_possible_truncation)]
            {
                ((bits & 1) as u8) | (((bits >> 31) & 2) as u8)
            }
        }
        #[inline(always)]
        fn mul(a: float64x2_t, b: float64x2_t) -> float64x2_t {
            // SAFETY: NEON is available (module invariant).
            unsafe { vmulq_f64(a, b) }
        }
        #[inline(always)]
        fn mul_add(acc: float64x2_t, a: float64x2_t, b: float64x2_t) -> float64x2_t {
            // SAFETY: NEON is available; `vfmaq_f64(acc, a, b)` is `acc + a * b`.
            unsafe { vfmaq_f64(acc, a, b) }
        }
        #[inline(always)]
        fn scalar_mul_add(acc: f64, a: f64, b: f64) -> f64 {
            a.mul_add(b, acc)
        }
        #[inline(always)]
        fn sqrt(a: float64x2_t) -> float64x2_t {
            // SAFETY: NEON is available (module invariant).
            unsafe { vsqrtq_f64(a) }
        }
        #[inline(always)]
        fn abs(a: float64x2_t) -> float64x2_t {
            // SAFETY: NEON is available (module invariant).
            unsafe { vabsq_f64(a) }
        }
        #[inline(always)]
        fn copysign(mag: float64x2_t, sign: float64x2_t) -> float64x2_t {
            // SAFETY: NEON is available (module invariant).
            let sign_mask = unsafe { vdupq_n_u64(0x8000_0000_0000_0000) };
            // SAFETY: NEON is available; selects sign bit from `sign`, rest from `mag`.
            unsafe { vbslq_f64(sign_mask, sign, mag) }
        }
        #[inline(always)]
        fn clamp01(a: float64x2_t) -> float64x2_t {
            let zero = <Self as LanesF64>::splat(0.0);
            let one = <Self as LanesF64>::splat(1.0);
            // SAFETY: NEON is available; `vmaxnmq_f64` matches `f64::max` on NaN.
            let lo = unsafe { vmaxnmq_f64(a, zero) };
            // SAFETY: NEON is available; `vminnmq_f64` matches `f64::min`.
            unsafe { vminnmq_f64(lo, one) }
        }
        #[inline(always)]
        fn select_le(
            a: float64x2_t,
            b: float64x2_t,
            if_le: float64x2_t,
            if_gt: float64x2_t,
        ) -> float64x2_t {
            // SAFETY: NEON is available (module invariant).
            let mask = unsafe { vcleq_f64(a, b) };
            // SAFETY: NEON is available (module invariant).
            unsafe { vbslq_f64(mask, if_le, if_gt) }
        }
        #[inline(always)]
        fn all_le(a: float64x2_t, b: float64x2_t) -> bool {
            // SAFETY: NEON is available (module invariant).
            let mask = unsafe { vcleq_f64(a, b) };
            // SAFETY: NEON is available (module invariant).
            let words = unsafe { vreinterpretq_u32_u64(mask) };
            // SAFETY: NEON is available (module invariant).
            let min = unsafe { vminvq_u32(words) };
            min != 0
        }
    }
}

#[cfg(target_arch = "x86_64")]
pub(crate) use avx2::Avx2;

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use super::{HALF_DOWN, Lanes, LanesF64};
    use std::arch::x86_64::{
        __m256, __m256d, _CMP_GT_OQ, _CMP_LE_OQ, _mm_add_ps, _mm_cvtsi128_si64, _mm_loadu_ps,
        _mm_packus_epi32, _mm_set1_ps, _mm_storeu_ps, _mm256_add_pd, _mm256_and_pd,
        _mm256_andnot_pd, _mm256_blendv_pd, _mm256_castps256_ps128, _mm256_cmp_pd,
        _mm256_cvtepi32_pd, _mm256_cvtps_pd, _mm256_cvttpd_epi32, _mm256_extractf128_ps,
        _mm256_fmadd_pd, _mm256_fmadd_ps, _mm256_loadu_ps, _mm256_max_pd, _mm256_min_pd,
        _mm256_movemask_pd, _mm256_mul_pd, _mm256_or_pd, _mm256_set_m128, _mm256_set1_pd,
        _mm256_set1_ps, _mm256_setzero_pd, _mm256_setzero_ps, _mm256_sqrt_pd, _mm256_storeu_pd,
        _mm256_storeu_ps, _mm256_sub_pd,
    };

    /// 256-bit AVX2 with FMA, two pixels per vector: slot 0 accumulates the
    /// even taps, slot 1 the odd taps, so sums are reassociated once.
    pub(crate) struct Avx2;

    impl Lanes for Avx2 {
        const SLOTS: usize = 2;
        type V = __m256;
        #[inline(always)]
        fn zero() -> __m256 {
            // SAFETY: only reached inside an `avx2,fma` target-feature
            // entry after detection (module invariant).
            unsafe { _mm256_setzero_ps() }
        }
        #[inline(always)]
        fn load(values: &[f32]) -> __m256 {
            let values = values
                .first_chunk::<8>()
                .expect("caller provides eight values");
            // SAFETY: `values` is eight valid `f32`; `loadu` is unaligned.
            unsafe { _mm256_loadu_ps(values.as_ptr()) }
        }
        #[inline(always)]
        fn store(values: &mut [f32], vector: __m256) {
            let values = values
                .first_chunk_mut::<8>()
                .expect("caller provides eight values");
            // SAFETY: `values` is eight writable `f32`; `storeu` is unaligned.
            unsafe { _mm256_storeu_ps(values.as_mut_ptr(), vector) }
        }
        #[inline(always)]
        fn splat(value: f32) -> __m256 {
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_set1_ps(value) }
        }
        #[inline(always)]
        fn slot_weights(weights: &[f32]) -> __m256 {
            let (low, high) = (weights[0], weights[1]);
            // SAFETY: AVX is available (module invariant).
            let low = unsafe { _mm_set1_ps(low) };
            // SAFETY: as above.
            let high = unsafe { _mm_set1_ps(high) };
            // SAFETY: as above; the first argument is the upper half.
            unsafe { _mm256_set_m128(high, low) }
        }
        #[inline(always)]
        fn fma(acc: __m256, a: __m256, b: __m256) -> __m256 {
            // SAFETY: FMA is available (module invariant).
            unsafe { _mm256_fmadd_ps(a, b, acc) }
        }
        #[inline(always)]
        fn reduce_slots(vector: __m256) -> [f32; 4] {
            // SAFETY: AVX is available (module invariant).
            let low = unsafe { _mm256_castps256_ps128(vector) };
            // SAFETY: as above.
            let high = unsafe { _mm256_extractf128_ps::<1>(vector) };
            // SAFETY: as above.
            let sum = unsafe { _mm_add_ps(low, high) };
            let mut out = [0.0; 4];
            // SAFETY: `out` is four writable `f32`.
            unsafe { _mm_storeu_ps(out.as_mut_ptr(), sum) };
            out
        }
    }

    impl LanesF64 for Avx2 {
        const WIDTH: usize = 4;
        type V = __m256d;
        #[inline(always)]
        fn splat(value: f64) -> __m256d {
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_set1_pd(value) }
        }
        #[inline(always)]
        fn load_f32(values: &[f32]) -> __m256d {
            let values = values
                .first_chunk::<4>()
                .expect("caller provides four values");
            // SAFETY: `values` is four valid `f32`; SSE is available.
            let quad = unsafe { _mm_loadu_ps(values.as_ptr()) };
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_cvtps_pd(quad) }
        }
        #[inline(always)]
        fn store_f64(values: &mut [f64], vector: __m256d) {
            let values = values
                .first_chunk_mut::<4>()
                .expect("caller provides four values");
            // SAFETY: `values` is four writable `f64`; AVX `storeu` is unaligned.
            unsafe { _mm256_storeu_pd(values.as_mut_ptr(), vector) }
        }
        #[inline(always)]
        fn store_u16_round_checked(values: &mut [u16], vector: __m256d, limit: __m256d) -> u8 {
            let out = values
                .first_chunk_mut::<4>()
                .expect("caller provides four values");
            let half = <Self as LanesF64>::splat(HALF_DOWN);
            // SAFETY: AVX is available (module invariant).
            let shifted = unsafe { _mm256_add_pd(vector, half) };
            // SAFETY: AVX is available; values are in `[0, 65535.5]`.
            let ints = unsafe { _mm256_cvttpd_epi32(shifted) };
            // SAFETY: SSE4.1 is implied by AVX2 (module invariant).
            let packed = unsafe { _mm_packus_epi32(ints, ints) };
            // SAFETY: SSE2 is available (module invariant).
            let bits = unsafe { _mm_cvtsi128_si64(packed) }.cast_unsigned();
            #[allow(clippy::cast_possible_truncation)]
            {
                out[0] = bits as u16;
                out[1] = (bits >> 16) as u16;
                out[2] = (bits >> 32) as u16;
                out[3] = (bits >> 48) as u16;
            }
            // SAFETY: AVX is available (module invariant).
            let rounded = unsafe { _mm256_cvtepi32_pd(ints) };
            // SAFETY: AVX is available (module invariant).
            let delta = unsafe { _mm256_sub_pd(vector, rounded) };
            let abs_delta = <Self as LanesF64>::abs(delta);
            // SAFETY: AVX is available (module invariant).
            let gt = unsafe { _mm256_cmp_pd::<_CMP_GT_OQ>(abs_delta, limit) };
            // SAFETY: AVX is available; returns a 4-bit mask in `0..=15`.
            let mask = unsafe { _mm256_movemask_pd(gt) };
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                mask as u8
            }
        }
        #[inline(always)]
        fn mul(a: __m256d, b: __m256d) -> __m256d {
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_mul_pd(a, b) }
        }
        #[inline(always)]
        fn mul_add(acc: __m256d, a: __m256d, b: __m256d) -> __m256d {
            // SAFETY: FMA is available (module invariant).
            unsafe { _mm256_fmadd_pd(a, b, acc) }
        }
        #[inline(always)]
        fn scalar_mul_add(acc: f64, a: f64, b: f64) -> f64 {
            a.mul_add(b, acc)
        }
        #[inline(always)]
        fn sqrt(a: __m256d) -> __m256d {
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_sqrt_pd(a) }
        }
        #[inline(always)]
        fn abs(a: __m256d) -> __m256d {
            let sign_mask = <Self as LanesF64>::splat(-0.0);
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_andnot_pd(sign_mask, a) }
        }
        #[inline(always)]
        fn copysign(mag: __m256d, sign: __m256d) -> __m256d {
            let sign_mask = <Self as LanesF64>::splat(-0.0);
            // SAFETY: AVX is available (module invariant).
            let sign_bit = unsafe { _mm256_and_pd(sign, sign_mask) };
            // SAFETY: AVX is available (module invariant).
            let cleared = unsafe { _mm256_andnot_pd(sign_mask, mag) };
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_or_pd(cleared, sign_bit) }
        }
        #[inline(always)]
        fn clamp01(a: __m256d) -> __m256d {
            // SAFETY: AVX is available (module invariant).
            let zero = unsafe { _mm256_setzero_pd() };
            let one = <Self as LanesF64>::splat(1.0);
            // SAFETY: AVX is available; `maxpd(a, 0.0)` returns `0.0` on NaN.
            let lo = unsafe { _mm256_max_pd(a, zero) };
            // SAFETY: AVX is available (module invariant).
            unsafe { _mm256_min_pd(lo, one) }
        }
        #[inline(always)]
        fn select_le(a: __m256d, b: __m256d, if_le: __m256d, if_gt: __m256d) -> __m256d {
            // SAFETY: AVX is available (module invariant).
            let mask = unsafe { _mm256_cmp_pd::<_CMP_LE_OQ>(a, b) };
            // SAFETY: AVX is available; selects `if_le` where mask sign bit is 1.
            unsafe { _mm256_blendv_pd(if_gt, if_le, mask) }
        }
        #[inline(always)]
        fn all_le(a: __m256d, b: __m256d) -> bool {
            // SAFETY: AVX is available (module invariant).
            let mask = unsafe { _mm256_cmp_pd::<_CMP_LE_OQ>(a, b) };
            // SAFETY: AVX is available (module invariant).
            let bits = unsafe { _mm256_movemask_pd(mask) };
            bits == 0b1111
        }
    }
}

#[cfg(target_arch = "x86_64")]
pub(crate) use avx512::Avx512;

#[cfg(target_arch = "x86_64")]
mod avx512 {
    use super::{HALF_DOWN, Lanes, LanesF64};
    use std::arch::x86_64::{
        __m512, __m512d, _CMP_GT_OQ, _CMP_LE_OQ, _mm_add_ps, _mm_loadu_ps, _mm_storeu_ps,
        _mm_storeu_si128, _mm256_cvtepi32_epi16, _mm256_loadu_ps, _mm512_add_pd, _mm512_and_si512,
        _mm512_andnot_si512, _mm512_broadcast_f32x4, _mm512_castpd_si512, _mm512_castps512_ps128,
        _mm512_castsi512_pd, _mm512_cmp_pd_mask, _mm512_cvtepi32_pd, _mm512_cvtps_pd,
        _mm512_cvttpd_epi32, _mm512_extractf32x4_ps, _mm512_fmadd_pd, _mm512_fmadd_ps,
        _mm512_loadu_ps, _mm512_mask_blend_pd, _mm512_max_pd, _mm512_min_pd, _mm512_mul_pd,
        _mm512_or_si512, _mm512_permutexvar_ps, _mm512_set1_epi64, _mm512_set1_pd, _mm512_set1_ps,
        _mm512_setr_epi32, _mm512_setzero_pd, _mm512_setzero_ps, _mm512_sqrt_pd, _mm512_storeu_pd,
        _mm512_storeu_ps, _mm512_sub_pd,
    };

    /// 512-bit AVX-512, four pixels per vector: slot `s` accumulates taps
    /// `s, s + 4, s + 8, ...`, so sums are reassociated in four partials.
    pub(crate) struct Avx512;

    impl Lanes for Avx512 {
        const SLOTS: usize = 4;
        type V = __m512;
        #[inline(always)]
        fn zero() -> __m512 {
            // SAFETY: only reached inside an AVX-512 target-feature entry
            // after detection (module invariant).
            unsafe { _mm512_setzero_ps() }
        }
        #[inline(always)]
        fn load(values: &[f32]) -> __m512 {
            let values = values
                .first_chunk::<16>()
                .expect("caller provides sixteen values");
            // SAFETY: `values` is sixteen valid `f32`; `loadu` is unaligned.
            unsafe { _mm512_loadu_ps(values.as_ptr()) }
        }
        #[inline(always)]
        fn store(values: &mut [f32], vector: __m512) {
            let values = values
                .first_chunk_mut::<16>()
                .expect("caller provides sixteen values");
            // SAFETY: `values` is sixteen writable `f32`; `storeu` is unaligned.
            unsafe { _mm512_storeu_ps(values.as_mut_ptr(), vector) }
        }
        #[inline(always)]
        fn splat(value: f32) -> __m512 {
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_set1_ps(value) }
        }
        #[inline(always)]
        fn slot_weights(weights: &[f32]) -> __m512 {
            let weights = weights
                .first_chunk::<4>()
                .expect("caller provides four weights");
            // SAFETY: `weights` is four valid `f32`; SSE is part of the
            // AVX-512 feature set (module invariant).
            let quad = unsafe { _mm_loadu_ps(weights.as_ptr()) };
            // SAFETY: AVX-512F is available.
            let repeated = unsafe { _mm512_broadcast_f32x4(quad) };
            // SAFETY: AVX-512F is available.
            let index =
                unsafe { _mm512_setr_epi32(0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3) };
            // SAFETY: AVX-512F is available; lane `i` takes element
            // `index[i]`, so slot `s` holds `weights[s]` four times.
            unsafe { _mm512_permutexvar_ps(index, repeated) }
        }
        #[inline(always)]
        fn fma(acc: __m512, a: __m512, b: __m512) -> __m512 {
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_fmadd_ps(a, b, acc) }
        }
        #[inline(always)]
        fn reduce_slots(vector: __m512) -> [f32; 4] {
            // SAFETY: AVX-512F is available (module invariant).
            let first = unsafe { _mm512_castps512_ps128(vector) };
            // SAFETY: as above.
            let second = unsafe { _mm512_extractf32x4_ps::<1>(vector) };
            // SAFETY: as above.
            let third = unsafe { _mm512_extractf32x4_ps::<2>(vector) };
            // SAFETY: as above.
            let fourth = unsafe { _mm512_extractf32x4_ps::<3>(vector) };
            // SAFETY: as above; slots are summed in order 0, 1, 2, 3.
            let low = unsafe { _mm_add_ps(first, second) };
            // SAFETY: as above.
            let high = unsafe { _mm_add_ps(low, third) };
            // SAFETY: as above.
            let sum = unsafe { _mm_add_ps(high, fourth) };
            let mut out = [0.0; 4];
            // SAFETY: `out` is four writable `f32`.
            unsafe { _mm_storeu_ps(out.as_mut_ptr(), sum) };
            out
        }
    }

    impl LanesF64 for Avx512 {
        const WIDTH: usize = 8;
        type V = __m512d;
        #[inline(always)]
        fn splat(value: f64) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_set1_pd(value) }
        }
        #[inline(always)]
        fn load_f32(values: &[f32]) -> __m512d {
            let values = values
                .first_chunk::<8>()
                .expect("caller provides eight values");
            // SAFETY: `values` is eight valid `f32`; AVX is implied by AVX-512.
            let oct = unsafe { _mm256_loadu_ps(values.as_ptr()) };
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_cvtps_pd(oct) }
        }
        #[inline(always)]
        fn store_f64(values: &mut [f64], vector: __m512d) {
            let values = values
                .first_chunk_mut::<8>()
                .expect("caller provides eight values");
            // SAFETY: `values` is eight writable `f64`; AVX-512F `storeu` is unaligned.
            unsafe { _mm512_storeu_pd(values.as_mut_ptr(), vector) }
        }
        #[inline(always)]
        fn store_u16_round_checked(values: &mut [u16], vector: __m512d, limit: __m512d) -> u8 {
            let out = values
                .first_chunk_mut::<8>()
                .expect("caller provides eight values");
            let half = <Self as LanesF64>::splat(HALF_DOWN);
            // SAFETY: AVX-512F is available (module invariant).
            let shifted = unsafe { _mm512_add_pd(vector, half) };
            // SAFETY: AVX-512F is available; values are in `[0, 65535.5]`.
            let ints = unsafe { _mm512_cvttpd_epi32(shifted) };
            // SAFETY: AVX-512VL is available (module invariant).
            let packed = unsafe { _mm256_cvtepi32_epi16(ints) };
            // SAFETY: `out` is eight writable `u16` (16 bytes); `storeu` is unaligned.
            unsafe { _mm_storeu_si128(out.as_mut_ptr().cast(), packed) };
            // SAFETY: AVX-512F is available (module invariant).
            let rounded = unsafe { _mm512_cvtepi32_pd(ints) };
            // SAFETY: AVX-512F is available (module invariant).
            let delta = unsafe { _mm512_sub_pd(vector, rounded) };
            let abs_delta = <Self as LanesF64>::abs(delta);
            // SAFETY: AVX-512F is available; returns an 8-bit mask.
            unsafe { _mm512_cmp_pd_mask::<_CMP_GT_OQ>(abs_delta, limit) }
        }
        #[inline(always)]
        fn mul(a: __m512d, b: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_mul_pd(a, b) }
        }
        #[inline(always)]
        fn mul_add(acc: __m512d, a: __m512d, b: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_fmadd_pd(a, b, acc) }
        }
        #[inline(always)]
        fn scalar_mul_add(acc: f64, a: f64, b: f64) -> f64 {
            a.mul_add(b, acc)
        }
        #[inline(always)]
        fn sqrt(a: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_sqrt_pd(a) }
        }
        #[inline(always)]
        fn abs(a: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            let sign_mask = unsafe { _mm512_set1_epi64(i64::MIN) };
            // SAFETY: AVX-512F is available (module invariant).
            let bits = unsafe { _mm512_castpd_si512(a) };
            // SAFETY: AVX-512F is available (module invariant).
            let cleared = unsafe { _mm512_andnot_si512(sign_mask, bits) };
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_castsi512_pd(cleared) }
        }
        #[inline(always)]
        fn copysign(mag: __m512d, sign: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            let sign_mask = unsafe { _mm512_set1_epi64(i64::MIN) };
            // SAFETY: AVX-512F is available (module invariant).
            let mag_bits = unsafe { _mm512_castpd_si512(mag) };
            // SAFETY: AVX-512F is available (module invariant).
            let sign_bits = unsafe { _mm512_castpd_si512(sign) };
            // SAFETY: AVX-512F is available (module invariant).
            let cleared = unsafe { _mm512_andnot_si512(sign_mask, mag_bits) };
            // SAFETY: AVX-512F is available (module invariant).
            let kept_sign = unsafe { _mm512_and_si512(sign_mask, sign_bits) };
            // SAFETY: AVX-512F is available (module invariant).
            let combined = unsafe { _mm512_or_si512(cleared, kept_sign) };
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_castsi512_pd(combined) }
        }
        #[inline(always)]
        fn clamp01(a: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            let zero = unsafe { _mm512_setzero_pd() };
            let one = <Self as LanesF64>::splat(1.0);
            // SAFETY: AVX-512F is available; `maxpd(a, 0.0)` returns `0.0` on NaN.
            let lo = unsafe { _mm512_max_pd(a, zero) };
            // SAFETY: AVX-512F is available (module invariant).
            unsafe { _mm512_min_pd(lo, one) }
        }
        #[inline(always)]
        fn select_le(a: __m512d, b: __m512d, if_le: __m512d, if_gt: __m512d) -> __m512d {
            // SAFETY: AVX-512F is available (module invariant).
            let mask = unsafe { _mm512_cmp_pd_mask::<_CMP_LE_OQ>(a, b) };
            // SAFETY: AVX-512F is available; selects `if_le` where mask bit is 1.
            unsafe { _mm512_mask_blend_pd(mask, if_gt, if_le) }
        }
        #[inline(always)]
        fn all_le(a: __m512d, b: __m512d) -> bool {
            // SAFETY: AVX-512F is available (module invariant).
            let mask = unsafe { _mm512_cmp_pd_mask::<_CMP_LE_OQ>(a, b) };
            mask == 0xFF
        }
    }
}
