// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Numeric representation helpers for the reference Clang configuration.
//! Ordinary scalar expressions contract on AArch64, with auto-vectorization
//! disabled by upstream. These helpers mark source contraction sites, not new
//! optimizations. Highway's explicit non-fused operations do not use them.

pub(crate) fn mul_add_f32(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}
pub(crate) fn mul_add_f64(a: f64, b: f64, c: f64) -> f64 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}

// C++ min/max retain the first operand on unordered comparison, unlike Rust's
// floating-point min/max. Clamp1 is from lib/jxl/base/common.h.
pub(crate) fn max_f32(a: f32, b: f32) -> f32 {
    if a < b { b } else { a }
}
pub(crate) fn min_f32(a: f32, b: f32) -> f32 {
    if b < a { b } else { a }
}
pub(crate) fn max_f64(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
pub(crate) fn clamp_f32(x: f32, low: f32, high: f32) -> f32 {
    if x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}
pub(crate) fn clamp_f64(x: f64, low: f64, high: f64) -> f64 {
    if x < low {
        low
    } else if x > high {
        high
    } else {
        x
    }
}
