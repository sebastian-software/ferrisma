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

#[cfg(target_arch = "aarch64")]
pub(crate) use neon::Neon;

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::Lanes;
    use std::arch::aarch64::{float32x4_t, vdupq_n_f32, vfmaq_f32, vld1q_f32, vst1q_f32};

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
            Self::splat(weights[0])
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
}

#[cfg(target_arch = "x86_64")]
pub(crate) use avx2::Avx2;

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use super::Lanes;
    use std::arch::x86_64::{
        __m256, _mm_add_ps, _mm_set1_ps, _mm_storeu_ps, _mm256_castps256_ps128,
        _mm256_extractf128_ps, _mm256_fmadd_ps, _mm256_loadu_ps, _mm256_set_m128, _mm256_set1_ps,
        _mm256_setzero_ps, _mm256_storeu_ps,
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
}
