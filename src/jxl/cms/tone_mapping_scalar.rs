// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete one-lane Rec2408ToneMapper, HlgOOTF, and GamutMap from
//! cms/tone_mapping-inl.h. Composition borrows the original base constructor
//! fields in place of C++ inheritance; vector MulAdd remains unfused.
use super::tone_mapping::{Color, HlgOotfBase, Range, Rec2408ToneMapperBase};
use super::transfer_functions_scalar::TfPq;
use crate::jxl::base::fast_math::fast_powf;
use crate::jxl::base::hwy_scalar::{clamp, max, min, mul_add, zero_if_negative};
use crate::jxl::base::matrix_ops::Vector3;
pub struct Rec2408ToneMapper {
    base: Rec2408ToneMapperBase,
    pq: TfPq,
}
impl Rec2408ToneMapper {
    pub fn new(source: Range, target: Range, luminances: Vector3) -> Self {
        Self {
            base: Rec2408ToneMapperBase::new(source, target, luminances),
            pq: TfPq::new(1.0),
        }
    }
    fn p(&self, b: f32) -> f32 {
        let s = &self.base;
        let t = (b - s.ks) * s.inv_one_minus_ks;
        let t2 = t * t;
        let t3 = t2 * t;
        mul_add(
            mul_add(2.0, t3, mul_add(-3.0, t2, 1.0)),
            s.ks,
            mul_add(
                t3 + mul_add(-2.0, t2, t),
                1.0 - s.ks,
                mul_add(-2.0, t3, 3.0 * t2) * s.max_lum,
            ),
        )
    }
    pub fn tone_map(&self, rgb: &mut Color) {
        let s = &self.base;
        let p = s.luminances;
        let luminance = s.source[1] * mul_add(p[0], rgb[0], mul_add(p[1], rgb[1], p[2] * rgb[2]));
        let normalized = min(
            1.0,
            (self.pq.encoded_from_display(luminance) - s.pq_mastering_min)
                * s.inv_pq_mastering_range,
        );
        let e2 = if normalized < s.ks {
            normalized
        } else {
            self.p(normalized)
        };
        let one = 1.0 - e2;
        let square = one * one;
        let fourth = square * square;
        let e3 = mul_add(s.min_lum, fourth, e2);
        let e4 = mul_add(e3, s.pq_mastering_range, s.pq_mastering_min);
        let new_luminance = min(
            s.target[1],
            zero_if_negative(self.pq.display_from_encoded(e4)),
        );
        let use_cap = luminance <= 1e-6f32;
        let ratio = new_luminance / max(luminance, 1e-6f32);
        let cap = new_luminance * s.inv_target_peak;
        let multiplier = ratio * s.normalizer;
        for v in rgb {
            *v = if use_cap { cap } else { *v * multiplier };
        }
    }
}
pub struct HlgOotf {
    base: HlgOotfBase,
}
impl HlgOotf {
    pub fn new(source: f32, target: f32, luminances: Vector3) -> Self {
        Self {
            base: HlgOotfBase::new(source, target, luminances),
        }
    }
    pub fn from_scene_light(display: f32, luminances: Vector3) -> Self {
        Self {
            base: HlgOotfBase::from_gamma(
                1.2f32 * 1.111f32.powf((display / 1000.0).log2()),
                luminances,
            ),
        }
    }
    pub fn to_scene_light(display: f32, luminances: Vector3) -> Self {
        Self {
            base: HlgOotfBase::from_gamma(
                (1.0f32 / 1.2f32) * 1.111f32.powf(-(display / 1000.0).log2()),
                luminances,
            ),
        }
    }
    pub fn apply(&self, rgb: &mut Color) {
        let s = &self.base;
        if !s.apply_ootf {
            return;
        }
        let p = s.luminances;
        let luminance = mul_add(p[0], rgb[0], mul_add(p[1], rgb[1], p[2] * rgb[2]));
        let ratio = min(fast_powf(luminance, s.exponent), 1e9f32);
        for v in rgb {
            *v *= ratio;
        }
    }
    pub fn warrants_gamut_mapping(&self) -> bool {
        self.base.apply_ootf && self.base.exponent < 0.0
    }
}
// Match the selected x86 scalar object's ADDSS destination operand without
// allowing the surrounding release function to commute competing NaN payloads.
#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn add_gamut(first: f32, second: f32) -> f32 {
    first + second
}

pub fn gamut_map(rgb: &mut Color, p: Vector3, preserve: f32) {
    #[cfg(target_arch = "x86_64")]
    let luminance = add_gamut(add_gamut(p[2] * rgb[2], p[1] * rgb[1]), p[0] * rgb[0]);
    #[cfg(not(target_arch = "x86_64"))]
    let luminance = mul_add(p[0], rgb[0], mul_add(p[1], rgb[1], p[2] * rgb[2]));
    let mut saturation = 0.0;
    let mut lum = 0.0;
    for &value in &*rgb {
        let delta = value - luminance;
        let inverse = 1.0 / if delta == 0.0 { 1.0 } else { delta };
        let over = value * inverse;
        saturation = if delta >= 0.0 {
            saturation
        } else {
            max(saturation, over)
        };
        lum = max(
            lum,
            if delta <= 0.0 {
                saturation
            } else {
                over - inverse
            },
        );
    }
    let mix = clamp(mul_add(preserve, saturation - lum, lum), 0.0, 1.0);
    for value in &mut *rgb {
        // Clang's scalar GamutMap emits FADD(original, product) here. Retain
        // that operand order for NaN payloads; multiplication remains separate.
        *value += mix * (luminance - *value);
    }
    let max_clr = max(max(1.0, rgb[0]), max(rgb[1], rgb[2]));
    let normalizer = 1.0 / max_clr;
    for value in rgb {
        *value *= normalizer;
    }
}
