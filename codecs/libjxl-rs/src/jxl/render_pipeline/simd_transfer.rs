// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The vector bodies of base/rational_polynomial-inl.h (EvalRationalPolynomial),
//! base/fast_math-inl.h (FastLog2f, FastPow2f, FastPowf),
//! cms/transfer_functions-inl.h (TF_SRGB, TF_PQ, TF_HLG, TF_709) and
//! cms/tone_mapping-inl.h (HlgOOTF::Apply) for the NEON and AVX2 targets
//! (the decode direction `DisplayFromEncoded` of the transfer functions,
//! `Rec2408ToneMapper` and `GamutMap` are in the `color` block at the end)
//! (ADR 0004), written once against [`SimdF32`]. They are translated
//! operation by operation: MulAdd is fused on both targets, and the
//! `HWY_REP4` coefficient tables are `LoadDup128` of four equal words, that
//! is `Set`. `HWY_FENCE` only orders the source's statements on x86 and has
//! no vector counterpart. The scalar paths remain in `base::fast_math` and
//! `cms::transfer_functions_scalar`.
use crate::jxl::base::matrix_ops::Vector3;
use crate::jxl::cms::tone_mapping::{HlgOotfBase, Range, Rec2408ToneMapperBase};
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

// color: the decode direction (`DisplayFromEncoded`) of the transfer
// functions, used by the SIMD ToLinearStage (ADR 0004 Plan item 1).

/// `TF_SRGB().DisplayFromEncoded(x)` (the `HWY_FULL(float)` overload).
#[allow(clippy::excessive_precision, reason = "the source's binary32 literals")]
#[inline(always)]
pub fn srgb_display_from_encoded<D: SimdF32>(d: D, x: D::V) -> D::V {
    let (x, original_sign, k_sign) = split_sign(d, x);
    // Computed via af_cheb_rational (k=100); replicated 4x.
    const P: [f32; 5] = [
        2.200248328e-04,
        1.043637593e-02,
        1.624820318e-01,
        7.961564959e-01,
        8.210152774e-01,
    ];
    const Q: [f32; 5] = [
        2.631846970e-01,
        1.076976492e+00,
        4.987528350e-01,
        -5.512498495e-02,
        6.521209011e-03,
    ];
    // kLowDivInv = 1.0f / kLowDiv, a binary32 constant expression.
    let linear = d.mul(x, d.set(1.0f32 / 12.92f32));
    let poly = eval_rational_polynomial(d, x, &P, &Q);
    let magnitude = d.if_then_else(d.gt(x, d.set(0.04045f32)), poly, linear);
    d.or(d.and_not(k_sign, magnitude), original_sign)
}

impl TfPqSimd {
    /// `DisplayFromEncoded(d, x)` (maximum error 3e-6).
    #[inline(always)]
    pub fn display_from_encoded<D: SimdF32>(&self, d: D, x: D::V) -> D::V {
        let (x, original_sign, k_sign) = split_sign(d, x);
        // 4-over-4-degree rational polynomial approximation on x+x*x.
        let xpxx = d.mul_add(x, x, x);
        let bits = |w: [u32; 5]| w.map(f32::from_bits);
        let p = bits([0x3989dfff, 0xbbcc536d, 0x3f3d150a, 0x40295064, 0x3f0ccf16]);
        let q = bits([0x43d2acd0, 0xc3d65e50, 0x432e5d5b, 0xc207a1ad, 0x402b570b]);
        let magnitude = eval_rational_polynomial(d, xpxx, &p, &q);
        d.or(
            d.and_not(
                k_sign,
                d.mul(
                    magnitude,
                    d.set(self.display_scaling_factor_from_10000_nits),
                ),
            ),
            original_sign,
        )
    }
}

/// `TF_HLG().DisplayFromEncoded(d, x)`. Double `constexpr` expressions round
/// to binary32 only at `Set`.
#[inline(always)]
pub fn hlg_display_from_encoded<D: SimdF32>(d: D, x: D::V) -> D::V {
    let (x, original_sign, k_sign) = split_sign(d, x);
    let below05 = d.le(x, d.set(0.5f64 as f32));
    let lo = d.mul(x, d.mul(x, d.set((1.0f64 / 3.0) as f32)));
    // kHiPow = kRA * log2(e), kHiMul = exp(-kC * kRA) * kInv12, kHiAdd =
    // kB * kInv12.
    let hi = d.mul_add(
        fast_pow2f(d, d.mul(x, d.set(8.067285659607931f64 as f32))),
        d.set(0.003639807079052639f64 as f32),
        d.set((TfHlgBase::B * TfHlgBase::INV12) as f32),
    );
    let magnitude = d.if_then_else(below05, lo, hi);
    d.or(d.and_not(k_sign, magnitude), original_sign)
}

/// `TF_709().DisplayFromEncoded(d, x)`.
#[inline(always)]
pub fn rec709_display_from_encoded<D: SimdF32>(d: D, x: D::V) -> D::V {
    let low = d.mul(d.set((1.0f64 / 4.5) as f32), x);
    let hi = fast_powf(
        d,
        d.mul_add(
            x,
            d.set((1.0f64 / 1.099) as f32),
            d.set((0.099f64 * (1.0 / 1.099)) as f32),
        ),
        d.set((1.0f64 / 0.45) as f32),
    );
    d.if_then_else(d.lt(x, d.set(0.081f64 as f32)), low, hi)
}

/// `Rec2408ToneMapper<HWY_FULL(float)>` of cms/tone_mapping-inl.h: the
/// original base constructor's fields and its `TF_PQ(1.0)`.
pub struct Rec2408ToneMapperSimd {
    base: Rec2408ToneMapperBase,
    tf_pq: TfPqSimd,
}
impl Rec2408ToneMapperSimd {
    pub fn new(source_range: Range, target_range: Range, luminances: Vector3) -> Self {
        Self {
            base: Rec2408ToneMapperBase::new(source_range, target_range, luminances),
            tf_pq: TfPqSimd::new(1.0),
        }
    }
    /// `ToneMap(&red, &green, &blue)`.
    #[inline(always)]
    pub fn tone_map<D: SimdF32>(&self, df: D, rgb: &mut [D::V; 3]) {
        let s = &self.base;
        let [red_y, green_y, blue_y] = s.luminances;
        let [red, green, blue] = *rgb;
        let luminance = df.mul(
            df.set(s.source[1]),
            df.mul_add(
                df.set(red_y),
                red,
                df.mul_add(df.set(green_y), green, df.mul(df.set(blue_y), blue)),
            ),
        );
        let pq_mastering_min = df.set(s.pq_mastering_min);
        let inv_pq_mastering_range = df.set(s.inv_pq_mastering_range);
        let normalized_pq = df.min(
            df.set(1.0),
            df.mul(
                df.sub(self.inv_eotf(df, luminance), pq_mastering_min),
                inv_pq_mastering_range,
            ),
        );
        let ks = df.set(s.ks);
        let e2 = df.if_then_else(
            df.lt(normalized_pq, ks),
            normalized_pq,
            self.p(df, normalized_pq),
        );
        let one_minus_e2 = df.sub(df.set(1.0), e2);
        let one_minus_e2_2 = df.mul(one_minus_e2, one_minus_e2);
        let one_minus_e2_4 = df.mul(one_minus_e2_2, one_minus_e2_2);
        let b = df.set(s.min_lum);
        let e3 = df.mul_add(b, one_minus_e2_4, e2);
        let pq_mastering_range = df.set(s.pq_mastering_range);
        let e4 = df.mul_add(e3, pq_mastering_range, pq_mastering_min);
        let new_luminance = df.min(
            df.set(s.target[1]),
            df.zero_if_negative(self.tf_pq.display_from_encoded(df, e4)),
        );
        let min_luminance = df.set(1e-6f32);
        let use_cap = df.le(luminance, min_luminance);
        let ratio = df.div(new_luminance, df.max(luminance, min_luminance));
        let cap = df.mul(new_luminance, df.set(s.inv_target_peak));
        let normalizer = df.set(s.normalizer);
        let multiplier = df.mul(ratio, normalizer);
        for val in rgb {
            *val = df.if_then_else(use_cap, cap, df.mul(*val, multiplier));
        }
    }
    #[inline(always)]
    fn inv_eotf<D: SimdF32>(&self, df: D, luminance: D::V) -> D::V {
        self.tf_pq.encoded_from_display(df, luminance)
    }
    #[inline(always)]
    fn t<D: SimdF32>(&self, df: D, a: D::V) -> D::V {
        let ks = df.set(self.base.ks);
        let inv_one_minus_ks = df.set(self.base.inv_one_minus_ks);
        df.mul(df.sub(a, ks), inv_one_minus_ks)
    }
    #[inline(always)]
    fn p<D: SimdF32>(&self, df: D, b: D::V) -> D::V {
        let t_b = self.t(df, b);
        let t_b_2 = df.mul(t_b, t_b);
        let t_b_3 = df.mul(t_b_2, t_b);
        let ks = df.set(self.base.ks);
        let max_lum = df.set(self.base.max_lum);
        df.mul_add(
            df.mul_add(
                df.set(2.0),
                t_b_3,
                df.mul_add(df.set(-3.0), t_b_2, df.set(1.0)),
            ),
            ks,
            df.mul_add(
                df.add(t_b_3, df.mul_add(df.set(-2.0), t_b_2, t_b)),
                df.sub(df.set(1.0), ks),
                df.mul(
                    df.mul_add(df.set(-2.0), t_b_3, df.mul(df.set(3.0), t_b_2)),
                    max_lum,
                ),
            ),
        )
    }
}

/// `GamutMap(&red, &green, &blue, primaries_luminances,
/// preserve_saturation)` of cms/tone_mapping-inl.h.
#[inline(always)]
pub fn gamut_map<D: SimdF32>(
    df: D,
    rgb: &mut [D::V; 3],
    primaries_luminances: &Vector3,
    preserve_saturation: f32,
) {
    let [red, green, blue] = *rgb;
    let luminance = df.mul_add(
        df.set(primaries_luminances[0]),
        red,
        df.mul_add(
            df.set(primaries_luminances[1]),
            green,
            df.mul(df.set(primaries_luminances[2]), blue),
        ),
    );
    // Desaturate out-of-gamut pixels (see the source for the rationale).
    let zero = df.zero();
    let one = df.set(1.0);
    let mut gray_mix_saturation = zero;
    let mut gray_mix_luminance = zero;
    for val in [red, green, blue] {
        let val_minus_gray = df.sub(val, luminance);
        let inv_val_minus_gray = df.div(
            one,
            df.if_then_else(df.eq(val_minus_gray, zero), one, val_minus_gray),
        );
        let val_over_val_minus_gray = df.mul(val, inv_val_minus_gray);
        gray_mix_saturation = df.if_then_else(
            df.ge(val_minus_gray, zero),
            gray_mix_saturation,
            df.max(gray_mix_saturation, val_over_val_minus_gray),
        );
        gray_mix_luminance = df.max(
            gray_mix_luminance,
            df.if_then_else(
                df.le(val_minus_gray, zero),
                gray_mix_saturation,
                df.sub(val_over_val_minus_gray, inv_val_minus_gray),
            ),
        );
    }
    let gray_mix = df.clamp(
        df.mul_add(
            df.set(preserve_saturation),
            df.sub(gray_mix_saturation, gray_mix_luminance),
            gray_mix_luminance,
        ),
        zero,
        one,
    );
    for val in &mut *rgb {
        *val = df.mul_add(gray_mix, df.sub(luminance, *val), *val);
    }
    let [red, green, blue] = *rgb;
    let max_clr = df.max(df.max(one, red), df.max(green, blue));
    let normalizer = df.div(one, max_clr);
    for val in rgb {
        *val = df.mul(*val, normalizer);
    }
}
// end color
