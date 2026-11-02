// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! One-lane TF_PQ from cms/transfer_functions-inl.h and its degree-four
//! EvalRationalPolynomial from base/rational_polynomial-inl.h. Keep exact float
//! words, operation order, sign restoration, and matching x86 HWY_FENCE sites.
//! Other specialized transfer functions remain unported.
use crate::jxl::base::hwy_scalar::mul_add;
fn fence() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    std::sync::atomic::fence(std::sync::atomic::Ordering::AcqRel);
}
fn rational(x: f32, p: [u32; 5], q: [u32; 5]) -> f32 {
    let mut yp = f32::from_bits(p[4]);
    let mut yq = f32::from_bits(q[4]);
    for n in 1..=7 {
        fence();
        if n <= 4 {
            yp = mul_add(yp, x, f32::from_bits(p[4 - n]));
        }
        if n <= 4 {
            yq = mul_add(yq, x, f32::from_bits(q[4 - n]));
        }
    }
    yp / yq
}
pub struct TfPq {
    to_10000: f32,
    from_10000: f32,
}
impl TfPq {
    pub fn new(intensity: f32) -> Self {
        Self {
            to_10000: intensity * (1.0f32 / 10000.0f32),
            from_10000: 10000.0f32 / intensity,
        }
    }
    pub fn display_from_encoded(&self, x: f32) -> f32 {
        let sign = x.to_bits() & 0x80000000;
        let x = f32::from_bits(x.to_bits() & 0x7fffffff);
        let xpxx = mul_add(x, x, x);
        let p = [0x3989dfff, 0xbbcc536d, 0x3f3d150a, 0x40295064, 0x3f0ccf16];
        let q = [0x43d2acd0, 0xc3d65e50, 0x432e5d5b, 0xc207a1ad, 0x402b570b];
        let magnitude = rational(xpxx, p, q);
        f32::from_bits(((magnitude * self.from_10000).to_bits() & 0x7fffffff) | sign)
    }
    pub fn encoded_from_display(&self, x: f32) -> f32 {
        let sign = x.to_bits() & 0x80000000;
        let x = f32::from_bits(x.to_bits() & 0x7fffffff);
        let xto025 = (x * self.to_10000).sqrt().sqrt();
        let p = [0x3c5d697d, 0xbf8c4274, 0x425ce93a, 0x43154069, 0x42418990];
        let q = [0x3f8196d9, 0x41a1562e, 0x42b94632, 0x42e01f14, 0x41cf3bc3];
        let plo = [0x37257b02, 0x3ec6b81c, 0x43074838, 0x4786914f, 0xc88be24d];
        let qlo = [0x4206dfee, 0x44b8b702, 0x467b5314, 0xc72b7ad7, 0xc84a65a6];
        // IfThenElse evaluates both rational branches in the source.
        let lower = x < 1e-4f32;
        let low = rational(xto025, plo, qlo);
        let high = rational(xto025, p, q);
        let magnitude = if lower { low } else { high };
        f32::from_bits((magnitude.to_bits() & 0x7fffffff) | sign)
    }
}

/// Original high-precision scalar TF_SRGB::EncodedFromDisplay. The reference
/// common.h selects JXL_HIGH_PRECISION=1, including FromLinearStage<OpRgb>.
#[allow(clippy::excessive_precision)]
pub fn srgb_encoded_from_display(value: f32) -> f32 {
    let sign = value.to_bits() & 0x8000_0000;
    let x = f32::from_bits(value.to_bits() & 0x7fff_ffff);
    let p = [
        -5.135152395e-04f32,
        5.287254571e-03,
        3.903842876e-01,
        1.474205315,
        7.352629620e-01,
    ];
    let q = [
        1.004519624e-02f32,
        3.036675394e-01,
        1.340816930,
        9.258482155e-01,
        2.424867759e-02,
    ];
    let linear = x * 12.92;
    let poly = rational(x.sqrt(), p.map(f32::to_bits), q.map(f32::to_bits));
    let magnitude = if x > 0.0031308 { poly } else { linear };
    f32::from_bits((magnitude.to_bits() & 0x7fff_ffff) | sign)
}
