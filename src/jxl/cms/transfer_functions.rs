// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/cms/transfer_functions.h`. Preserve the source's
//! double intermediates and float copysignf result, even on double APIs.

use super::super::base::float::{max_f64, mul_add_f64};

pub struct TfHlgBase;
impl TfHlgBase {
    pub(crate) const A: f64 = 0.17883277;
    const RA: f64 = 1.0 / Self::A;
    pub(crate) const B: f64 = 1.0 - 4.0 * Self::A;
    pub(crate) const C: f64 = 0.5599107295;
    pub(crate) const INV12: f64 = 1.0 / 12.0;
    pub fn display_from_encoded(e: f64) -> f64 {
        Self::inv_oetf(e)
    }
    pub fn encoded_from_display(d: f64) -> f64 {
        Self::oetf(d)
    }
    fn oetf(mut s: f64) -> f64 {
        if s == 0.0 {
            return 0.0;
        }
        let sign = s as f32;
        s = s.abs();
        if s <= Self::INV12 {
            return f64::from(((3.0 * s).sqrt() as f32).copysign(sign));
        }
        let e = mul_add_f64(Self::A, mul_add_f64(12.0, s, -Self::B).ln(), Self::C);
        debug_assert!(e > 0.0);
        f64::from((e as f32).copysign(sign))
    }
    fn inv_oetf(mut e: f64) -> f64 {
        if e == 0.0 {
            return 0.0;
        }
        let sign = e as f32;
        e = e.abs();
        if e <= 0.5 {
            return f64::from(((e * e * (1.0 / 3.0)) as f32).copysign(sign));
        }
        let s = (((e - Self::C) * Self::RA).exp() + Self::B) * Self::INV12;
        debug_assert!(s >= 0.0);
        f64::from((s as f32).copysign(sign))
    }
}

pub struct TfPqBase;
impl TfPqBase {
    const M1: f64 = 2610.0 / 16384.0;
    const M2: f64 = (2523.0 / 4096.0) * 128.0;
    const C1: f64 = 3424.0 / 4096.0;
    const C2: f64 = (2413.0 / 4096.0) * 32.0;
    const C3: f64 = (2392.0 / 4096.0) * 32.0;
    pub fn display_from_encoded(target: f32, mut e: f64) -> f64 {
        if e == 0.0 {
            return 0.0;
        }
        let sign = e as f32;
        e = e.abs();
        let xp = e.powf(1.0 / Self::M2);
        let num = max_f64(xp - Self::C1, 0.0);
        let den = mul_add_f64(-Self::C3, xp, Self::C2);
        debug_assert!(den != 0.0);
        let d = (num / den).powf(1.0 / Self::M1);
        debug_assert!(d >= 0.0);
        f64::from(((d * f64::from(10000.0f32 / target)) as f32).copysign(sign))
    }
    pub fn encoded_from_display(target: f32, mut d: f64) -> f64 {
        if d == 0.0 {
            return 0.0;
        }
        let sign = d as f32;
        d = d.abs();
        let xp = (d * f64::from(target * (1.0f32 / 10000.0f32))).powf(Self::M1);
        let num = mul_add_f64(xp, Self::C2, Self::C1);
        let den = mul_add_f64(xp, Self::C3, 1.0);
        let e = (num / den).powf(Self::M2);
        debug_assert!(e > 0.0);
        f64::from((e as f32).copysign(sign))
    }
}
