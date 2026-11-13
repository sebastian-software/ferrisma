// Copyright 2019 Google LLC
// Copyright (c) The Highway Project Authors. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause
// Use of this source code is governed by Highway's offered BSD alternative in
// licenses/HIGHWAY-BSD-3-Clause.txt. Modified: translated the subset of the
// pinned Highway operations that libjxl's SIMD kernels use, for the HWY_NEON
// (hwy/ops/arm_neon-inl.h) and HWY_AVX2 (hwy/ops/x86_256-inl.h,
// x86_128-inl.h, generic_ops-inl.h) targets only. See the port's ADR 0004.
//! Highway operations for the port's SIMD paths.
//!
//! [`SimdF32`] is the counterpart of a Highway descriptor `HWY_FULL(float)`
//! for one target. A kernel body is written once, generic over `D: SimdF32`,
//! like an original `-inl.h` body inside `HWY_NAMESPACE`, and instantiated
//! for [`Neon`] (four lanes) or [`Avx2`] (eight lanes), like Highway's
//! `foreach_target`. Each operation is a thin mapping to the `core::arch`
//! instruction(s) that Highway itself uses for that target: fused `MulAdd`,
//! `vminnm`/`minps` with their own NaN rules, `vrecpe`/`rcpps` estimates, the
//! per-block lane order of AVX2 shuffles, and so on. Results are therefore
//! target-specific; a SIMD path is only ever compared with the original's
//! SIMD build at the same target (ADR 0004), never with the scalar path.
//!
//! A descriptor value is a capability token: it exists only after run-time
//! detection proved that the CPU supports every feature of Highway's group
//! for that target ([`Target::detect`]). The scalar `HWY_SCALAR` operations
//! remain in `base::hwy_scalar`.

#[cfg(target_arch = "x86_64")]
pub mod avx2;
#[cfg(target_arch = "aarch64")]
pub mod neon;

#[cfg(target_arch = "x86_64")]
pub use avx2::Avx2;
#[cfg(target_arch = "aarch64")]
pub use neon::Neon;

/// One Highway target's full float vector, `HWY_FULL(float)` (with
/// `RebindToSigned` for [`SimdF32::I`]). Methods take the descriptor by value
/// like Highway's tag arguments; the names follow Highway's operations.
///
/// Loads and stores take slices and bounds-check them: Highway's `Load` and
/// `LoadU` (and `Store`/`StoreU`) read the same values, alignment is not
/// observable, so both map to the unaligned instructions.
pub trait SimdF32: Copy + core::fmt::Debug + Send + Sync + 'static {
    /// Float vector.
    type V: Copy;
    /// Signed 32-bit integer vector with the same lane count.
    type I: Copy;
    /// Lane mask (all-ones/all-zero lanes, as Highway's NEON/AVX2 masks).
    type M: Copy;
    /// `Lanes(d)`.
    const LANES: usize;
    /// The Highway target name, as `hwy::TargetName`.
    const NAME: &'static str;

    // Initialization, loads and stores.
    fn set(self, v: f32) -> Self::V;
    fn zero(self) -> Self::V;
    fn load(self, src: &[f32]) -> Self::V;
    fn store(self, v: Self::V, dst: &mut [f32]);
    fn load_bits(self, src: &[u32]) -> Self::V;
    fn store_bits(self, v: Self::V, dst: &mut [u32]);
    fn store_i32(self, v: Self::I, dst: &mut [i32]);
    fn get_lane(self, v: Self::V) -> f32;

    // Arithmetic.
    fn add(self, a: Self::V, b: Self::V) -> Self::V;
    fn sub(self, a: Self::V, b: Self::V) -> Self::V;
    fn mul(self, a: Self::V, b: Self::V) -> Self::V;
    fn div(self, a: Self::V, b: Self::V) -> Self::V;
    /// `MulAdd(mul, x, add)`: fused on both targets.
    fn mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V;
    fn neg_mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V;
    fn mul_sub(self, mul: Self::V, x: Self::V, sub: Self::V) -> Self::V;
    fn neg_mul_sub(self, mul: Self::V, x: Self::V, sub: Self::V) -> Self::V;
    fn min(self, a: Self::V, b: Self::V) -> Self::V;
    fn max(self, a: Self::V, b: Self::V) -> Self::V;
    /// Highway's generic `Clamp(v, lo, hi) = Min(Max(lo, v), hi)`.
    #[inline(always)]
    fn clamp(self, v: Self::V, lo: Self::V, hi: Self::V) -> Self::V {
        self.min(self.max(lo, v), hi)
    }
    fn abs(self, v: Self::V) -> Self::V;
    fn neg(self, v: Self::V) -> Self::V;
    fn abs_diff(self, a: Self::V, b: Self::V) -> Self::V;
    fn sqrt(self, v: Self::V) -> Self::V;
    /// Vendor-specific estimate: `vrecpe` (NEON), `rcpps` (AVX2). See ADR 0004.
    fn approximate_reciprocal(self, v: Self::V) -> Self::V;
    /// Vendor-specific estimate: `vrsqrte` (NEON), `rsqrtps` (AVX2).
    fn approximate_reciprocal_sqrt(self, v: Self::V) -> Self::V;
    fn floor(self, v: Self::V) -> Self::V;
    fn ceil(self, v: Self::V) -> Self::V;
    fn round(self, v: Self::V) -> Self::V;
    fn trunc(self, v: Self::V) -> Self::V;
    /// Highway's `ZeroIfNegative`: lanes with the sign bit set become +0.
    fn zero_if_negative(self, v: Self::V) -> Self::V;
    fn copy_sign(self, magn: Self::V, sign: Self::V) -> Self::V;
    fn copy_sign_to_abs(self, abs: Self::V, sign: Self::V) -> Self::V;

    // Bitwise operations on float vectors.
    fn and(self, a: Self::V, b: Self::V) -> Self::V;
    fn or(self, a: Self::V, b: Self::V) -> Self::V;
    fn xor(self, a: Self::V, b: Self::V) -> Self::V;
    /// `AndNot(a, b) = ~a & b`.
    fn and_not(self, a: Self::V, b: Self::V) -> Self::V;

    // Comparisons and selection.
    fn lt(self, a: Self::V, b: Self::V) -> Self::M;
    fn le(self, a: Self::V, b: Self::V) -> Self::M;
    fn gt(self, a: Self::V, b: Self::V) -> Self::M;
    fn ge(self, a: Self::V, b: Self::V) -> Self::M;
    fn eq(self, a: Self::V, b: Self::V) -> Self::M;
    fn ne(self, a: Self::V, b: Self::V) -> Self::M;
    fn vec_from_mask(self, m: Self::M) -> Self::V;
    fn if_then_else(self, m: Self::M, yes: Self::V, no: Self::V) -> Self::V;
    fn if_then_zero_else(self, m: Self::M, no: Self::V) -> Self::V;
    fn if_then_else_zero(self, m: Self::M, yes: Self::V) -> Self::V;

    // Conversions.
    /// `ConvertTo(di, v)`: truncating, saturating as Highway defines it.
    fn convert_to_i32(self, v: Self::V) -> Self::I;
    fn convert_to_f32(self, v: Self::I) -> Self::V;
    fn nearest_int(self, v: Self::V) -> Self::I;
    fn bitcast_to_i32(self, v: Self::V) -> Self::I;
    fn bitcast_to_f32(self, v: Self::I) -> Self::V;

    // Signed 32-bit integer lanes.
    fn add_i32(self, a: Self::I, b: Self::I) -> Self::I;
    fn sub_i32(self, a: Self::I, b: Self::I) -> Self::I;
    fn min_i32(self, a: Self::I, b: Self::I) -> Self::I;
    fn max_i32(self, a: Self::I, b: Self::I) -> Self::I;
    /// `VecFromMask(di, Lt(a, b))` and friends on integer lanes.
    fn lt_i32(self, a: Self::I, b: Self::I) -> Self::I;
    fn eq_i32(self, a: Self::I, b: Self::I) -> Self::I;
    fn gt_i32(self, a: Self::I, b: Self::I) -> Self::I;
    fn shift_left_i32<const BITS: i32>(self, v: Self::I) -> Self::I;
    /// Arithmetic shift of signed lanes.
    fn shift_right_i32<const BITS: i32>(self, v: Self::I) -> Self::I;
    /// Logical shift of the same lanes reinterpreted as unsigned.
    fn shift_right_u32<const BITS: i32>(self, v: Self::I) -> Self::I;

    // Reductions; the vector forms broadcast the result to all lanes.
    fn sum_of_lanes(self, v: Self::V) -> Self::V;
    fn max_of_lanes(self, v: Self::V) -> Self::V;
    fn min_of_lanes(self, v: Self::V) -> Self::V;
    #[inline(always)]
    fn reduce_sum(self, v: Self::V) -> f32 {
        self.get_lane(self.sum_of_lanes(v))
    }

    // Lane movement.
    fn interleave_lower(self, a: Self::V, b: Self::V) -> Self::V;
    fn interleave_upper(self, a: Self::V, b: Self::V) -> Self::V;
    /// `StoreInterleaved2(a, b, d, dst)`: a0 b0 a1 b1 ...
    fn store_interleaved2(self, a: Self::V, b: Self::V, dst: &mut [f32]);
    /// `StoreInterleaved3(a, b, c, d, dst)`: a0 b0 c0 a1 b1 c1 ...
    fn store_interleaved3(self, a: Self::V, b: Self::V, c: Self::V, dst: &mut [f32]);

    // render stages: operations first used by the SIMD XYB, FromLinear and
    // output stages (ADR 0004 Plan item 1).
    /// `LoadDup128(d, p)`: the four floats at `src` in every 128-bit block.
    fn load_dup128(self, src: &[f32]) -> Self::V;
    /// `Broadcast<LANE>(v)`: lane `LANE` of each 128-bit block, broadcast
    /// within that block.
    fn broadcast<const LANE: i32>(self, v: Self::V) -> Self::V;
    /// `StoreU(BitCast(du16, DemoteTo(df16, v)), du16, dst)`: the binary16
    /// words of the hardware conversion (`fcvtn`, `vcvtps2ph`).
    fn store_demote_to_f16(self, v: Self::V, dst: &mut [u16]);
    // end render stages
    // modular: signed 32-bit lanes of the modular kernels (squeeze, RCT,
    // modular-to-float conversion; port ADR 0004 Plan item 2). Masks are
    // integer vectors with all-ones/all-zero lanes, as `VecFromMask`.
    /// `Set(di, v)`.
    fn set_i32(self, v: i32) -> Self::I;
    /// `Load(di, p)` (alignment is not observable).
    fn load_i32(self, src: &[i32]) -> Self::I;
    /// `And(a, b)` on signed lanes.
    fn and_i32(self, a: Self::I, b: Self::I) -> Self::I;
    /// `Xor(a, b)` on signed lanes.
    fn xor_i32(self, a: Self::I, b: Self::I) -> Self::I;
    /// `Abs(v)`; `i32::MIN` stays `i32::MIN` on both targets.
    fn abs_i32(self, v: Self::I) -> Self::I;
    /// `Neg(v)`, wrapping.
    fn neg_i32(self, v: Self::I) -> Self::I;
    /// `VecFromMask(di, Ne(a, b))`, which Highway forms as `Not(Eq(a, b))`.
    fn ne_i32(self, a: Self::I, b: Self::I) -> Self::I;
    /// `IfThenElse(MaskFromVec(m), yes, no)`.
    fn if_then_else_i32(self, m: Self::I, yes: Self::I, no: Self::I) -> Self::I;
    /// `IfThenZeroElse(MaskFromVec(m), no)`.
    fn if_then_zero_else_i32(self, m: Self::I, no: Self::I) -> Self::I;
    /// `BitCast(di, MulEven(a, b))`: the signed 64-bit products of the even
    /// lanes, each as its low and high 32-bit half.
    fn mul_even_i32(self, a: Self::I, b: Self::I) -> Self::I;
    /// `BitCast(di, MulOdd(a, b))`: as `mul_even_i32` for the odd lanes.
    fn mul_odd_i32(self, a: Self::I, b: Self::I) -> Self::I;
    /// `InterleaveOdd(di, a, b)`: a1 b1 a3 b3 ...
    fn interleave_odd_i32(self, a: Self::I, b: Self::I) -> Self::I;
    // end modular
}

/// The SIMD target a kernel runs on, capped at NEON/AVX2 (decision D6).
#[derive(Clone, Copy, Debug)]
pub enum Target {
    #[cfg(target_arch = "aarch64")]
    Neon(Neon),
    #[cfg(target_arch = "x86_64")]
    Avx2(Avx2),
}
impl Target {
    /// The best supported target of the v1.0 scope: NEON on AArch64, AVX2 on
    /// x86-64 when the CPU has Highway's complete AVX2 group. `None` means the
    /// scalar path. AVX-512 and SVE are deliberately never selected (D6).
    pub fn detect() -> Option<Self> {
        #[cfg(target_arch = "aarch64")]
        {
            Some(Self::Neon(Neon::new()))
        }
        #[cfg(target_arch = "x86_64")]
        {
            Avx2::new().map(Self::Avx2)
        }
        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        {
            None
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            #[cfg(target_arch = "aarch64")]
            Self::Neon(_) => Neon::NAME,
            #[cfg(target_arch = "x86_64")]
            Self::Avx2(_) => Avx2::NAME,
        }
    }
    pub fn lanes(self) -> usize {
        match self {
            #[cfg(target_arch = "aarch64")]
            Self::Neon(_) => Neon::LANES,
            #[cfg(target_arch = "x86_64")]
            Self::Avx2(_) => Avx2::LANES,
        }
    }
}

/// Whether the decoder selects the SIMD paths by default (port ADR 0004,
/// "Decoder path"). Off. Working decision 2026-11-13 (delegated, revisable
/// by the owner): turn it on for a target only when, on that target,
/// (a) every decode-path Highway export of ADR 0004's Plan that an in-scope
/// file can reach is translated (a file that reaches a scalar kernel while
/// the rest runs SIMD matches neither reference build); (b) a whole-decode
/// comparison through the public decoder, including its low-memory
/// pipeline, exists and reports zero differences over the conformance
/// inputs; (c) the scalar gate (`tools/check.py --stage test`) forces the
/// scalar path explicitly, so it still compares the scalar translation with
/// the scalar oracle; and (d) for AVX2, the same evidence exists from real
/// x86-64 hardware. The gate is per target (ADR-0041): AArch64 may be
/// turned on before x86-64 once (a) to (c) hold there. The VarDCT
/// whole-decode test of `decode_vardct`
/// (`whole_decodes_with_the_simd_stages_against_the_original_target`)
/// forces the SIMD target and asserts zero differences independently of
/// this switch.
pub const DECODER_SELECTS_SIMD: bool = false;

/// [`decoder_target`] override, the counterpart of Highway's
/// `SetSupportedTargetsForTest`: 0 follows [`DECODER_SELECTS_SIMD`], 1 is the
/// scalar path, 2 is the detected SIMD target.
static DECODER_TARGET_OVERRIDE: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

/// The target the decoder's dispatch selects, like `HWY_DYNAMIC_DISPATCH`
/// choosing the best supported target: `None` is the scalar path (the
/// `HWY_SCALAR` translation), otherwise [`Target::detect`] (NEON, or AVX2
/// with Highway's complete group; never AVX-512 or SVE).
pub fn decoder_target() -> Option<Target> {
    decoder_target_for(DECODER_SELECTS_SIMD)
}

/// Restrict or widen the decoder's dispatch for tests, like Highway's
/// `SetSupportedTargetsForTest`: `Some(true)` selects the detected SIMD
/// target, `Some(false)` the scalar path, `None` restores the default. It
/// is process-wide; planes allocated afterwards use the selected target's
/// row padding ([`max_vector_size`]).
#[doc(hidden)]
pub fn set_decoder_simd_for_test(simd: Option<bool>) {
    use core::sync::atomic::Ordering;
    let value = match simd {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    };
    DECODER_TARGET_OVERRIDE.store(value, Ordering::Relaxed);
}

/// libjxl's `MaxVectorSize()` (`simd_util.cc`, `HWY_DYNAMIC_DISPATCH` of
/// `Lanes(HWY_FULL(float)) * sizeof(float)`): the vector bytes of the
/// selected target, 4 for the scalar path as in the scalar reference build.
pub fn max_vector_size() -> usize {
    decoder_target().map_or(4, |t| 4 * t.lanes())
}

/// Run a generic kernel body for the target's descriptor, like Highway's
/// `HWY_DYNAMIC_DISPATCH` of a `foreach_target` instantiation. On AVX2 the
/// body runs inside a function compiled with Highway's AVX2 target features,
/// so the intrinsics inline as they do under `HWY_ATTR`.
pub trait Kernel {
    type Output;
    fn run<D: SimdF32>(self, d: D) -> Self::Output;
}
#[inline]
pub fn dispatch<K: Kernel>(target: Target, kernel: K) -> K::Output {
    match target {
        #[cfg(target_arch = "aarch64")]
        Target::Neon(d) => kernel.run(d),
        #[cfg(target_arch = "x86_64")]
        Target::Avx2(d) => avx2::run(d, kernel),
    }
}

// public gate: condition (c) of ADR 0004 "Decoder path".
/// [`decoder_target`] for a given default selection `selects_simd` (the
/// value of [`DECODER_SELECTS_SIMD`]), so that a test can show what the
/// selection would be if that switch were on. An explicit
/// [`set_decoder_simd_for_test`] override wins; otherwise a test build that
/// the scalar gate started with `JXL_DECODER_TARGET=scalar`
/// ([`scalar_forced_by_environment`]) keeps the scalar path whatever the
/// default is, so `tools/check.py --stage test` keeps comparing the scalar
/// translation with the scalar oracle.
#[doc(hidden)]
pub fn decoder_target_for(selects_simd: bool) -> Option<Target> {
    use core::sync::atomic::Ordering;
    match DECODER_TARGET_OVERRIDE.load(Ordering::Relaxed) {
        1 => None,
        2 => Target::detect(),
        _ if scalar_forced_by_environment() => None,
        _ if selects_simd => Target::detect(),
        _ => None,
    }
}

/// Whether the environment forces the scalar decoder path: only in the
/// oracle test builds (feature `cpp-oracle`), read once per process from
/// `JXL_DECODER_TARGET=scalar`, which the scalar test stage of
/// `tools/check.py` sets. Ordinary builds never read the environment.
#[doc(hidden)]
pub fn scalar_forced_by_environment() -> bool {
    #[cfg(feature = "cpp-oracle")]
    {
        static FORCED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *FORCED.get_or_init(|| {
            std::env::var_os("JXL_DECODER_TARGET").is_some_and(|value| value == "scalar")
        })
    }
    #[cfg(not(feature = "cpp-oracle"))]
    {
        false
    }
}
// end public gate
