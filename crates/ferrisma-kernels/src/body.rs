//! Kernel bodies, written once against [`Lanes`]. Safe Rust: slices are
//! bounds-checked, and the `Lanes` methods take fixed-size arrays.

use crate::Taps;
use crate::lanes::{Lanes, LanesF64, Scalar};

/// See `Kernels::horizontal_rgba`. The caller has validated the lengths.
///
/// Interleaving four output positions with separate accumulators was
/// measured and was slower on Apple M1 (24 Mpx transform 106 ms to 116 ms
/// with NEON, 107 ms to 155 ms scalar), so each position runs on its own.
#[inline(always)]
pub(crate) fn horizontal_rgba<L: Lanes>(source: &[f32], taps: &Taps, out: &mut [f32]) {
    let slots = L::SLOTS;
    for (position, pixel) in out.as_chunks_mut::<4>().0.iter_mut().enumerate() {
        let start = taps.starts[position] as usize * 4;
        let weights =
            &taps.weights[taps.offsets[position] as usize..taps.offsets[position + 1] as usize];
        let run = &source[start..start + weights.len() * 4];
        let full = weights.len() / slots * slots;
        let mut acc = L::zero();
        // Iterators instead of indices: one bounds check per run, not per tap.
        for (values, slot_weights) in run[..full * 4]
            .chunks_exact(slots * 4)
            .zip(weights[..full].chunks_exact(slots))
        {
            acc = L::fma(acc, L::load(values), L::slot_weights(slot_weights));
        }
        let mut sum = L::reduce_slots(acc);
        for (k, &weight) in weights.iter().enumerate().skip(full) {
            for (channel, value) in sum.iter_mut().enumerate() {
                *value = run[k * 4 + channel].mul_add(weight, *value);
            }
        }
        *pixel = sum;
    }
}

/// See `Kernels::axpy`. The caller has validated the lengths.
#[inline(always)]
pub(crate) fn axpy<L: Lanes>(acc: &mut [f32], row: &[f32], weight: f32) {
    let width = L::SLOTS * 4;
    let full = acc.len() / width * width;
    let splat = L::splat(weight);
    let (acc_body, acc_tail) = acc.split_at_mut(full);
    let (row_body, row_tail) = row.split_at(full);
    for (a, r) in acc_body
        .chunks_exact_mut(width)
        .zip(row_body.chunks_exact(width))
    {
        L::store(a, L::fma(L::load(a), L::load(r), splat));
    }
    for (a, &r) in acc_tail.iter_mut().zip(row_tail) {
        *a = r.mul_add(weight, *a);
    }
}

/// Degree-8 Remez minimax coefficients for `u^1.6` on
/// `[((0.04045 + 0.055) / 1.055)^0.25, 1.0]`, weighted by `1 / u^1.6`
/// (max relative error `1.39e-10`). With `t = (|x| + 0.055) / 1.055` and
/// `u = sqrt(sqrt(t))`, `t^2 * P(u)` matches `((|x| + 0.055) / 1.055)^2.4`
/// to `< 1.4e-10` relative error (430x finer than 1 ULP of `f32`).
const EOTF_C0: f64 = -6.335_193_213_440_115e-03;
const EOTF_C1: f64 = 1.838_822_428_917_036e-01;
const EOTF_C2: f64 = 1.306_064_415_403_507_3e+00;
const EOTF_C3: f64 = -1.004_001_397_013_697_2e+00;
const EOTF_C4: f64 = 9.792_986_212_645_558e-01;
const EOTF_C5: f64 = -7.356_100_235_950_327e-01;
const EOTF_C6: f64 = 3.759_463_673_632_575e-01;
const EOTF_C7: f64 = -1.151_308_532_324_695e-01;
const EOTF_C8: f64 = 1.588_582_027_037_883_2e-02;

#[inline(always)]
fn srgb_to_linear_unit<L: LanesF64>(x: L::V) -> L::V {
    let ax = L::abs(x);
    let linear_seg = L::mul(ax, L::splat(1.0 / 12.92));
    let t = L::mul_add(L::splat(0.055 / 1.055), ax, L::splat(1.0 / 1.055));
    let u = L::sqrt(L::sqrt(t));
    let u2 = L::mul(u, u);
    let u4 = L::mul(u2, u2);
    let u8 = L::mul(u4, u4);
    let p01 = L::mul_add(L::splat(EOTF_C0), L::splat(EOTF_C1), u);
    let p23 = L::mul_add(L::splat(EOTF_C2), L::splat(EOTF_C3), u);
    let p45 = L::mul_add(L::splat(EOTF_C4), L::splat(EOTF_C5), u);
    let p67 = L::mul_add(L::splat(EOTF_C6), L::splat(EOTF_C7), u);
    let p03 = L::mul_add(p01, p23, u2);
    let p47 = L::mul_add(p45, p67, u2);
    let p07 = L::mul_add(p03, p47, u4);
    let poly = L::mul_add(p07, L::splat(EOTF_C8), u8);
    let curved = L::mul(L::mul(t, t), poly);
    let mag = L::select_le(ax, L::splat(0.040_45), linear_seg, curved);
    L::copysign(mag, x)
}

/// Exact `f32` magnitude bit pattern for the sRGB linear-segment threshold
/// `0.04045_f32` (`f64::from(0.04045_f32) < 0.04045_f64`, while the next `f32`
/// up exceeds `0.04045_f64`).
const EOTF_LINEAR_THRESHOLD_BITS: u32 = 0.040_45_f32.to_bits();
/// Exact `f32` magnitude bit pattern for `1.0_f32`.
const UNIT_BITS: u32 = 1.0_f32.to_bits();

/// Scalar counterpart of [`srgb_to_linear_unit`] using `L::scalar_mul_add`, so
/// tails and mixed-range vectors execute the exact same fused or unfused
/// arithmetic as `L`'s vector path.
#[inline(always)]
fn srgb_to_linear_unit_scalar<L: LanesF64>(x: f64) -> f64 {
    let ax = x.abs();
    if ax <= 0.040_45 {
        return x * (1.0 / 12.92);
    }
    let t = L::scalar_mul_add(0.055 / 1.055, ax, 1.0 / 1.055);
    let u = t.sqrt().sqrt();
    let u2 = u * u;
    let u4 = u2 * u2;
    let u8 = u4 * u4;
    let p01 = L::scalar_mul_add(EOTF_C0, EOTF_C1, u);
    let p23 = L::scalar_mul_add(EOTF_C2, EOTF_C3, u);
    let p45 = L::scalar_mul_add(EOTF_C4, EOTF_C5, u);
    let p67 = L::scalar_mul_add(EOTF_C6, EOTF_C7, u);
    let p03 = L::scalar_mul_add(p01, p23, u2);
    let p47 = L::scalar_mul_add(p45, p67, u2);
    let p07 = L::scalar_mul_add(p03, p47, u4);
    let poly = L::scalar_mul_add(p07, EOTF_C8, u8);
    let curved = (t * t) * poly;
    curved.copysign(x)
}

#[inline(always)]
fn srgb_to_linear_scalar<L: LanesF64>(sample: f32) -> f64 {
    let value = f64::from(sample);
    if value.abs() <= 1.0 {
        srgb_to_linear_unit_scalar::<L>(value)
    } else {
        let magnitude = value.abs();
        value.signum() * ((magnitude + 0.055) / 1.055).powf(2.4)
    }
}

/// Largest unsigned magnitude bit pattern across `samples`. Branchless so
/// LLVM auto-vectorizes it with `umax` / `vpmaxud`.
#[inline(always)]
fn max_abs_bits(samples: &[f32]) -> u32 {
    let mut max_bits = 0_u32;
    for &sample in samples {
        max_bits = max_bits.max(sample.to_bits() & 0x7FFF_FFFF);
    }
    max_bits
}

/// See `Kernels::srgb_to_linear`. The caller has validated the lengths.
#[inline(always)]
pub(crate) fn srgb_to_linear<L: LanesF64>(src: &[f32], dst: &mut [f64]) {
    let width = L::WIDTH;
    let full = src.len() / width * width;
    let (src_body, src_tail) = src.split_at(full);
    let (dst_body, dst_tail) = dst.split_at_mut(full);
    let peak = max_abs_bits(src_body);
    if peak <= EOTF_LINEAR_THRESHOLD_BITS {
        let inv = L::splat(1.0 / 12.92);
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            L::store_f64(d, L::mul(L::load_f32(s), inv));
        }
    } else if peak <= UNIT_BITS {
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            L::store_f64(d, srgb_to_linear_unit::<L>(L::load_f32(s)));
        }
    } else {
        let one = L::splat(1.0);
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            let x = L::load_f32(s);
            if L::all_le(L::abs(x), one) {
                L::store_f64(d, srgb_to_linear_unit::<L>(x));
            } else {
                for (out, &sample) in d.iter_mut().zip(s) {
                    *out = srgb_to_linear_scalar::<L>(sample);
                }
            }
        }
    }
    for (out, &sample) in dst_tail.iter_mut().zip(src_tail) {
        *out = srgb_to_linear_scalar::<L>(sample);
    }
}

/// Maximum distance from the rounded integer (`|scaled - round(scaled)|`) for
/// which the polynomial OETF on the curved segment cannot straddle a half-code
/// rounding threshold `k + 0.5`. The polynomial error on `(0.0031308, 1.0]` is
/// `< 6e-11` (`< 4e-6` of a 16-bit code step, including `a * b + c` vs FMA
/// differences `< 1e-14`); a `1e-4` guard band is >25x wider while keeping
/// over 99.98% of curved lanes on the SIMD fast path. Linear-segment lanes
/// (`<= 0.0031308`) are exact (`12.92 * clamped`) and pass limit `1.0` so they
/// never trigger refinement.
const HALF_CODE_LIMIT: f64 = 0.5 - 1.0e-4;

/// Largest `f32` value whose widened `f64` is `<= 0.0031308_f64`.
const OETF_LINEAR_THRESHOLD_F32: f32 = {
    let x = 0.003_130_8_f32;
    if (x as f64) <= 0.003_130_8_f64 {
        x
    } else {
        x.next_down()
    }
};

/// `true` iff every finite sample in `samples` falls in the exact linear OETF
/// segment (`<= 0.0031308_f64`, including negative values clamped to `0.0`).
/// Short-circuits on the first bright sample so normal images pay ~0 overhead.
#[inline(always)]
fn all_oetf_linear(samples: &[f32]) -> bool {
    samples
        .iter()
        .all(|&sample| sample <= OETF_LINEAR_THRESHOLD_F32)
}

/// Degree-8 Remez minimax approximation of `1.055 * z^(10/3) - 0.055` on
/// `[0.0031308^0.125, 1.0]` with `z = sqrt(sqrt(sqrt(x)))` (max absolute error
/// `5.91e-11`, or `3.9e-6` of a 16-bit code step). Returns `(encoded, limit)`
/// where linear-segment lanes (`<= 0.0031308`) carry `limit = 1.0` (exact, no
/// refinement needed) and curved lanes carry `limit = HALF_CODE_LIMIT`.
#[inline(always)]
fn linear_to_srgb_unit<L: LanesF64>(x: L::V) -> (L::V, L::V) {
    const C0: f64 = -5.542_793_273_847_191_5e-02;
    const C1: f64 = 7.009_144_686_315_251e-03;
    const C2: f64 = -6.102_448_520_459_556e-02;
    const C3: f64 = 6.882_975_397_802_682e-01;
    const C4: f64 = 6.016_079_414_773_584e-01;
    const C5: f64 = -2.670_754_412_189_171_3e-01;
    const C6: f64 = 1.148_725_112_899_608_6e-01;
    const C7: f64 = -3.259_848_748_051_394e-02;
    const C8: f64 = 4.339_209_467_645_605_5e-03;

    let clamped = L::clamp01(x);
    let threshold = L::splat(0.003_130_8);
    let linear_seg = L::mul(clamped, L::splat(12.92));
    let z = L::sqrt(L::sqrt(L::sqrt(clamped)));
    let z2 = L::mul(z, z);
    let z4 = L::mul(z2, z2);
    let z8 = L::mul(z4, z4);
    let p01 = L::mul_add(L::splat(C0), L::splat(C1), z);
    let p23 = L::mul_add(L::splat(C2), L::splat(C3), z);
    let p45 = L::mul_add(L::splat(C4), L::splat(C5), z);
    let p67 = L::mul_add(L::splat(C6), L::splat(C7), z);
    let p03 = L::mul_add(p01, p23, z2);
    let p47 = L::mul_add(p45, p67, z2);
    let p07 = L::mul_add(p03, p47, z4);
    let curved = L::mul_add(p07, L::splat(C8), z8);
    let encoded = L::select_le(clamped, threshold, linear_seg, curved);
    let limit = L::select_le(clamped, threshold, L::splat(1.0), L::splat(HALF_CODE_LIMIT));
    (encoded, limit)
}

/// Exact `f64` sRGB OETF fallback for only the curved-segment lanes whose
/// polynomial output landed inside the half-code guard band around `k + 0.5`
/// (indicated by set bits in `mask`). Keeping this out-of-line avoids spilling
/// SIMD registers in the fast path.
#[cold]
#[inline(never)]
fn refine_linear_to_srgb_u16_masked(src: &[f32], max: u16, dst: &mut [u16], mut mask: u8) {
    let scale = f64::from(max);
    while mask != 0 {
        let lane = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        #[allow(clippy::manual_clamp)]
        let clamped = f64::from(src[lane]).max(0.0).min(1.0);
        let encoded = if clamped <= 0.003_130_8 {
            12.92 * clamped
        } else {
            1.055 * clamped.powf(1.0 / 2.4) - 0.055
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            dst[lane] = (encoded * scale).round() as u16;
        }
    }
}

/// See `Kernels::linear_to_srgb_u16`. The caller has validated the lengths and
/// finiteness of `linear`.
#[inline(always)]
pub(crate) fn linear_to_srgb_u16<L: LanesF64>(linear: &[f32], max: u16, out: &mut [u16]) {
    let width = L::WIDTH;
    let full = linear.len() / width * width;
    let (src_body, src_tail) = linear.split_at(full);
    let (dst_body, dst_tail) = out.split_at_mut(full);
    let scale = L::splat(f64::from(max));
    let scale_scalar = f64::from(max);
    if all_oetf_linear(linear) {
        let slope = L::splat(12.92);
        let one = L::splat(1.0);
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            let encoded = L::mul(L::clamp01(L::load_f32(s)), slope);
            let _ = L::store_u16_round_checked(d, L::mul(encoded, scale), one);
        }
        for (&s, d) in src_tail.iter().zip(dst_tail) {
            let clamped = Scalar::clamp01(f64::from(s));
            let _ = Scalar::store_u16_round_checked(
                std::slice::from_mut(d),
                (clamped * 12.92) * scale_scalar,
                1.0,
            );
        }
        return;
    }
    for (s, d) in src_body
        .chunks_exact(width)
        .zip(dst_body.chunks_exact_mut(width))
    {
        let (encoded, limit) = linear_to_srgb_unit::<L>(L::load_f32(s));
        let mask = L::store_u16_round_checked(d, L::mul(encoded, scale), limit);
        if mask != 0 {
            refine_linear_to_srgb_u16_masked(s, max, d, mask);
        }
    }
    for (&s, d) in src_tail.iter().zip(dst_tail) {
        let clamped = Scalar::clamp01(f64::from(s));
        if clamped <= 0.003_130_8 {
            let _ = Scalar::store_u16_round_checked(
                std::slice::from_mut(d),
                (clamped * 12.92) * scale_scalar,
                1.0,
            );
        } else {
            let (encoded, limit) = linear_to_srgb_unit::<Scalar>(clamped);
            let mask = Scalar::store_u16_round_checked(
                std::slice::from_mut(d),
                encoded * scale_scalar,
                limit,
            );
            if mask != 0 {
                refine_linear_to_srgb_u16_masked(
                    std::slice::from_ref(&s),
                    max,
                    std::slice::from_mut(d),
                    mask,
                );
            }
        }
    }
}
