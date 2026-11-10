// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Scalar tone/gamut mapping from `lib/jxl/cms/tone_mapping.h`.
//! The separate one-lane derived specialization is in tone_mapping_scalar;
//! other SIMD specializations remain unported.

use super::super::base::float::{clamp_f32, max_f32, min_f32, mul_add_f32};
use super::super::base::matrix_ops::Vector3;
use super::transfer_functions::TfPqBase;
pub type Color = [f32; 3];
pub type Range = [f32; 2];

pub struct Rec2408ToneMapperBase {
    pub(crate) source: Range,
    pub(crate) target: Range,
    pub(crate) luminances: Vector3,
    pub(crate) pq_mastering_min: f32,
    pub(crate) pq_mastering_range: f32,
    pub(crate) inv_pq_mastering_range: f32,
    pub(crate) min_lum: f32,
    pub(crate) max_lum: f32,
    pub(crate) ks: f32,
    pub(crate) inv_one_minus_ks: f32,
    pub(crate) normalizer: f32,
    pub(crate) inv_target_peak: f32,
}
fn luminance(p: Vector3, rgb: Color) -> f32 {
    mul_add_f32(p[2], rgb[2], mul_add_f32(p[0], rgb[0], p[1] * rgb[1]))
}
impl Rec2408ToneMapperBase {
    pub fn new(source: Range, target: Range, luminances: Vector3) -> Self {
        let pq_mastering_min = Self::inv_eotf(source[0]);
        let pq_mastering_max = Self::inv_eotf(source[1]);
        let pq_mastering_range = pq_mastering_max - pq_mastering_min;
        let inv_pq_mastering_range = 1.0 / pq_mastering_range;
        let min_lum = (Self::inv_eotf(target[0]) - pq_mastering_min) * inv_pq_mastering_range;
        let max_lum = (Self::inv_eotf(target[1]) - pq_mastering_min) * inv_pq_mastering_range;
        let ks = mul_add_f32(1.5, max_lum, -0.5);
        Self {
            source,
            target,
            luminances,
            pq_mastering_min,
            pq_mastering_range,
            inv_pq_mastering_range,
            min_lum,
            max_lum,
            ks,
            inv_one_minus_ks: 1.0 / max_f32(1e-6, 1.0 - ks),
            normalizer: source[1] / target[1],
            inv_target_peak: 1.0 / target[1],
        }
    }
    fn inv_eotf(luminance: f32) -> f32 {
        TfPqBase::encoded_from_display(1.0, f64::from(luminance)) as f32
    }
    fn p(&self, b: f32) -> f32 {
        let t = (b - self.ks) * self.inv_one_minus_ks;
        let t2 = t * t;
        let t3 = t2 * t;
        let a = mul_add_f32(2.0, t3, -3.0 * t2) + 1.0;
        let b = mul_add_f32(-2.0, t2, t3) + t;
        let c = mul_add_f32(-2.0, t3, 3.0 * t2);
        mul_add_f32(
            c,
            self.max_lum,
            mul_add_f32(a, self.ks, b * (1.0 - self.ks)),
        )
    }
    pub fn tone_map(&self, rgb: &mut Color) {
        let luminance = self.source[1] * luminance(self.luminances, *rgb);
        let normalized = min_f32(
            1.0,
            (Self::inv_eotf(luminance) - self.pq_mastering_min) * self.inv_pq_mastering_range,
        );
        let e2 = if normalized < self.ks {
            normalized
        } else {
            self.p(normalized)
        };
        let one_minus_e2 = 1.0 - e2;
        let square = one_minus_e2 * one_minus_e2;
        let fourth = square * square;
        let e3 = mul_add_f32(self.min_lum, fourth, e2);
        let e4 = mul_add_f32(e3, self.pq_mastering_range, self.pq_mastering_min);
        let d4 = TfPqBase::display_from_encoded(1.0, f64::from(e4)) as f32;
        let new_luminance = clamp_f32(d4, 0.0, self.target[1]);
        let use_cap = luminance <= 1e-6;
        let ratio = new_luminance / max_f32(luminance, 1e-6);
        let cap = new_luminance * self.inv_target_peak;
        let multiplier = ratio * self.normalizer;
        for val in rgb {
            *val = if use_cap { cap } else { *val * multiplier };
        }
    }
}
pub struct HlgOotfBase {
    pub(crate) exponent: f32,
    pub(crate) apply_ootf: bool,
    pub(crate) luminances: Vector3,
}
impl HlgOotfBase {
    // Original protected gamma constructor, used by the derived scalar stage.
    pub(crate) fn from_gamma(gamma: f32, luminances: Vector3) -> Self {
        let exponent = gamma - 1.0;
        Self {
            exponent,
            apply_ootf: exponent < -0.01 || 0.01 < exponent,
            luminances,
        }
    }
    pub fn new(source: f32, target: f32, luminances: Vector3) -> Self {
        let gamma = 1.111f32.powf((target / source).log2());
        let exponent = gamma - 1.0;
        Self {
            exponent,
            apply_ootf: exponent < -0.01 || 0.01 < exponent,
            luminances,
        }
    }
    pub fn apply(&self, rgb: &mut Color) {
        if !self.apply_ootf {
            return;
        }
        let y = luminance(self.luminances, *rgb);
        let ratio = min_f32(y.powf(self.exponent), 1e9);
        for value in rgb {
            *value *= ratio;
        }
    }
}
pub fn gamut_map_scalar(rgb: &mut Color, primaries_luminances: Vector3, preserve_saturation: f32) {
    let y = luminance(primaries_luminances, *rgb);
    let mut saturation = 0.0;
    let mut luminance = 0.0;
    for &value in &*rgb {
        let delta = value - y;
        let inverse = 1.0 / if delta == 0.0 { 1.0 } else { delta };
        let over = value * inverse;
        if delta < 0.0 {
            saturation = max_f32(saturation, over);
        }
        luminance = max_f32(
            luminance,
            if delta <= 0.0 {
                saturation
            } else {
                over - inverse
            },
        );
    }
    let mix = clamp_f32(
        mul_add_f32(preserve_saturation, saturation - luminance, luminance),
        0.0,
        1.0,
    );
    for value in &mut *rgb {
        *value = mul_add_f32(mix, y - *value, *value);
    }
    let max_clr = max_f32(max_f32(max_f32(1.0, rgb[0]), rgb[1]), rgb[2]);
    let normalizer = 1.0 / max_clr;
    for value in rgb {
        *value *= normalizer;
    }
}
