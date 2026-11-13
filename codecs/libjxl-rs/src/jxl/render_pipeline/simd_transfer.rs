// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The vector bodies of base/rational_polynomial-inl.h (EvalRationalPolynomial),
//! base/fast_math-inl.h (FastLog2f, FastPow2f, FastPowf),
//! cms/transfer_functions-inl.h (TF_SRGB, TF_PQ, TF_HLG, TF_709) and
//! cms/tone_mapping-inl.h (HlgOOTF::Apply) for the NEON and AVX2 targets
//! (ADR 0004), written once against [`SimdF32`]. They are translated
//! operation by operation: MulAdd is fused on both targets, and the
//! `HWY_REP4` coefficient tables are `LoadDup128` of four equal words, that
//! is `Set`. `HWY_FENCE` only orders the source's statements on x86 and has
//! no vector counterpart. The scalar paths remain in `base::fast_math` and
//! `cms::transfer_functions_scalar`.
use crate::jxl::cms::tone_mapping::HlgOotfBase;
use crate::jxl::cms::transfer_functions::TfHlgBase;
use crate::jxl::hwy::SimdF32;

/// `Set(di, v)` of `RebindToSigned`: a broadcast of the word, pure data
/// movement.
#[inline(always)]
fn set_i32<D: SimdF32>(d: D, v: i32) -> D::I {
    d.bitcast_to_i32(d.set(f32::from_bits(v as u32)))
}

/// `EvalRationalPolynomial(d, x, p, q)` with `FastDivision<float>` = `Div`
/// (`JXL_TRUE`). `p[k]` and `q[k]` are the degree-`k` coefficients.
#[inline(always)]
pub fn eval_rational_polynomial<D: SimdF32, const N: usize>(
    d: D,
    x: D::V,
    p: &[f32; N],
    q: &[f32; N],
) -> D::V {
    let deg = N - 1;
    let mut yp = d.set(p[deg]);
    let mut yq = d.set(q[deg]);
    for k in (0..deg).rev() {
        yp = d.mul_add(yp, x, d.set(p[k]));
        yq = d.mul_add(yq, x, d.set(q[k]));
    }
    d.div(yp, yq)
}

/// `FastLog2f(df, x)`: undefined for negative and NaN arguments.
#[inline(always)]
pub fn fast_log2f<D: SimdF32>(df: D, x: D::V) -> D::V {
    // 2,2 rational polynomial approximation of std::log1p(x) / std::log(2):
    // the source's binary32 literals.
    const P: [f32; 3] = [
        f32::from_bits(0xb5f85ab0),
        f32::from_bits(0x3fb6e02b),
        f32::from_bits(0x3f3e11c7),
    ];
    const Q: [f32; 3] = [
        f32::from_bits(0x3f7d8625),
        f32::from_bits(0x3f813ced),
        f32::from_bits(0x3e32458c),
    ];
    let x_bits = df.bitcast_to_i32(x);
    // Range reduction to [-1/3, 1/3] - 3 integer, 2 float ops
    let exp_bits = df.sub_i32(x_bits, set_i32(df, 0x3f2aaaab));
    // Shifted exponent = log2; also used to clear mantissa.
    let exp_shifted = df.shift_right_i32::<23>(exp_bits);
    let mantissa = df.bitcast_to_f32(df.sub_i32(x_bits, df.shift_left_i32::<23>(exp_shifted)));
    let exp_val = df.convert_to_f32(exp_shifted);
    df.add(
        eval_rational_polynomial(df, df.sub(mantissa, df.set(1.0)), &P, &Q),
        exp_val,
    )
}

/// `FastPow2f(df, x)`: max relative error ~3e-7.
#[inline(always)]
pub fn fast_pow2f<D: SimdF32>(df: D, x: D::V) -> D::V {
    let floorx = df.floor(x);
    let exp = df.bitcast_to_f32(
        df.shift_left_i32::<23>(df.add_i32(df.convert_to_i32(floorx), set_i32(df, 127))),
    );
    let frac = df.sub(x, floorx);
    let mut num = df.add(frac, df.set(f32::from_bits(0x4122cc6b)));
    num = df.mul_add(num, frac, df.set(f32::from_bits(0x424379a1)));
    num = df.mul_add(num, frac, df.set(f32::from_bits(0x42c519f0)));
    num = df.mul(num, exp);
    let mut den = df.mul_add(
        frac,
        df.set(f32::from_bits(0x3e5749ee)),
        df.set(f32::from_bits(0xbcb621be)),
    );
    den = df.mul_add(den, frac, df.set(f32::from_bits(0xc19b8831)));
    den = df.mul_add(den, frac, df.set(f32::from_bits(0x42c519f1)));
    df.div(num, den)
}

/// `FastPowf(df, base, exponent)`: max relative error ~3e-5.
#[inline(always)]
pub fn fast_powf<D: SimdF32>(df: D, base: D::V, exponent: D::V) -> D::V {
    fast_pow2f(df, df.mul(fast_log2f(df, base), exponent))
}

/// The sign handling shared by the mirrored transfer functions: returns
/// `(abs(x), original_sign, kSign)`.
#[inline(always)]
fn split_sign<D: SimdF32>(d: D, x: D::V) -> (D::V, D::V, D::V) {
    let k_sign = d.bitcast_to_f32(set_i32(d, i32::MIN));
    let original_sign = d.and(x, k_sign);
    (d.and_not(k_sign, x), original_sign, k_sign)
}

/// `TF_SRGB().EncodedFromDisplay(d, x)` (error ~5e-07).
#[allow(clippy::excessive_precision, reason = "the source's binary32 literals")]
#[inline(always)]
pub fn srgb_encoded_from_display<D: SimdF32>(d: D, x: D::V) -> D::V {
    let (x, original_sign, k_sign) = split_sign(d, x);
    // Computed via af_cheb_rational (k=100); replicated 4x.
    const P: [f32; 5] = [
        -5.135152395e-04,
        5.287254571e-03,
        3.903842876e-01,
        1.474205315e+00,
        7.352629620e-01,
    ];
    const Q: [f32; 5] = [
        1.004519624e-02,
        3.036675394e-01,
        1.340816930e+00,
        9.258482155e-01,
        2.424867759e-02,
    ];
    let linear = d.mul(x, d.set(12.92f32));
    let poly = eval_rational_polynomial(d, d.sqrt(x), &P, &Q);
    let magnitude = d.if_then_else(d.gt(x, d.set(0.0031308f32)), poly, linear);
    d.or(d.and_not(k_sign, magnitude), original_sign)
}

/// `TF_PQ`, holding the constructor's two scale factors.
#[derive(Clone, Copy)]
pub struct TfPqSimd {
    display_scaling_factor_to_10000_nits: f32,
    #[allow(dead_code, reason = "used by DisplayFromEncoded (ToLinear)")]
    display_scaling_factor_from_10000_nits: f32,
}
impl TfPqSimd {
    pub fn new(display_intensity_target: f32) -> Self {
        Self {
            display_scaling_factor_to_10000_nits: display_intensity_target * (1.0f32 / 10000.0f32),
            display_scaling_factor_from_10000_nits: 10000.0f32 / display_intensity_target,
        }
    }
    /// `EncodedFromDisplay(d, x)` (maximum error 7e-7).
    #[inline(always)]
    pub fn encoded_from_display<D: SimdF32>(&self, d: D, x: D::V) -> D::V {
        let (x, original_sign, k_sign) = split_sign(d, x);
        // 4-over-4-degree rational polynomial approximation on x**0.25, with
        // two different polynomials above and below 1e-4.
        let xto025 = d.sqrt(d.sqrt(d.mul(x, d.set(self.display_scaling_factor_to_10000_nits))));
        let bits = |w: [u32; 5]| w.map(f32::from_bits);
        let p = bits([0x3c5d697d, 0xbf8c4274, 0x425ce93a, 0x43154069, 0x42418990]);
        let q = bits([0x3f8196d9, 0x41a1562e, 0x42b94632, 0x42e01f14, 0x41cf3bc3]);
        let plo = bits([0x37257b02, 0x3ec6b81c, 0x43074838, 0x4786914f, 0xc88be24d]);
        let qlo = bits([0x4206dfee, 0x44b8b702, 0x467b5314, 0xc72b7ad7, 0xc84a65a6]);
        let magnitude = d.if_then_else(
            d.lt(x, d.set(1e-4f32)),
            eval_rational_polynomial(d, xto025, &plo, &qlo),
            eval_rational_polynomial(d, xto025, &p, &q),
        );
        d.or(d.and_not(k_sign, magnitude), original_sign)
    }
}

/// `TF_HLG().EncodedFromDisplay(d, x)` (maximum error 5e-7). Double
/// `constexpr` expressions round to binary32 only at `Set`.
#[inline(always)]
pub fn hlg_encoded_from_display<D: SimdF32>(d: D, x: D::V) -> D::V {
    let (x, original_sign, k_sign) = split_sign(d, x);
    let below_inv12 = d.le(x, d.set(TfHlgBase::INV12 as f32));
    let lo = d.sqrt(d.mul(d.set(3.0f64 as f32), x));
    // kInvLog2e in common.h is float; its product with kA is double.
    let hi = d.mul_add(
        d.set((TfHlgBase::A * f64::from(f32::from_bits(0x3f317218))) as f32),
        fast_log2f(d, d.mul_add(d.set(12.0), x, d.set(-TfHlgBase::B as f32))),
        d.set(TfHlgBase::C as f32),
    );
    let magnitude = d.if_then_else(below_inv12, lo, hi);
    d.or(d.and_not(k_sign, magnitude), original_sign)
}

/// `TF_709().EncodedFromDisplay(d, x)` (maximum error 1e-6).
#[inline(always)]
pub fn rec709_encoded_from_display<D: SimdF32>(d: D, x: D::V) -> D::V {
    let low = d.mul(d.set(4.5f64 as f32), x);
    let hi = d.mul_add(
        d.set(1.099f64 as f32),
        fast_powf(d, x, d.set(0.45f64 as f32)),
        d.set(-0.099f64 as f32),
    );
    d.if_then_else(d.le(x, d.set(0.018f64 as f32)), low, hi)
}

/// `HlgOOTF::Apply(&r, &g, &b)` on vectors.
#[inline(always)]
pub fn hlg_ootf_apply<D: SimdF32>(df: D, ootf: &HlgOotfBase, rgb: &mut [D::V; 3]) {
    if !ootf.apply_ootf {
        return;
    }
    let [red_y, green_y, blue_y] = ootf.luminances;
    let [red, green, blue] = *rgb;
    let luminance = df.mul_add(
        df.set(red_y),
        red,
        df.mul_add(df.set(green_y), green, df.mul(df.set(blue_y), blue)),
    );
    let ratio = df.min(
        fast_powf(df, luminance, df.set(ootf.exponent)),
        df.set(1e9f64 as f32),
    );
    *rgb = [
        df.mul(red, ratio),
        df.mul(green, ratio),
        df.mul(blue, ratio),
    ];
}
