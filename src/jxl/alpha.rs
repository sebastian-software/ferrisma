// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Scalar alpha operations from lib/jxl/alpha.{h,cc}. Borrowed input slices
//! retain source pointer-identity branches; owned temporary output permits
//! the source's in-place callers without aliased mutable references.
use super::base::float::{clamp_f32, mul_add_f32};
pub const SMALL_ALPHA: f32 = 1.0 / (1u32 << 26) as f32;

#[inline(never)]
fn mul(a: f32, b: f32) -> f32 {
    a * b
}
fn new_alpha(fa: f32, ba: f32) -> f32 {
    mul_add_f32(-(1.0 - fa), 1.0 - ba, 1.0)
}
fn blend_sample(bg: f32, ba: f32, fg: f32, fa: f32, premultiplied: bool) -> f32 {
    if premultiplied {
        mul_add_f32(bg, 1.0 - fa, fg)
    } else {
        let a = new_alpha(fa, ba);
        let ra = if a > 0.0 { 1.0 / a } else { 0.0 };
        mul_add_f32(fg, fa, mul(1.0 - fa, mul(ba, bg))) * ra
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Retain the original single-plane alpha boundary."
)]
pub fn perform_alpha_blending(
    bg: &[f32],
    bga: &[f32],
    fg: &[f32],
    fga: &[f32],
    out: &mut [f32],
    n: usize,
    premultiplied: bool,
    clamp: bool,
) {
    let alpha = std::ptr::eq(bg.as_ptr(), bga.as_ptr()) && std::ptr::eq(fg.as_ptr(), fga.as_ptr());
    for x in 0..n {
        let fa = if clamp {
            clamp_f32(fga[x], 0.0, 1.0)
        } else {
            fga[x]
        };
        out[x] = if alpha {
            new_alpha(fa, bga[x])
        } else {
            blend_sample(bg[x], bga[x], fg[x], fa, premultiplied)
        };
    }
}
pub fn perform_alpha_blending_rgba(
    bg: [&[f32]; 4],
    fg: [&[f32]; 4],
    out: &mut [&mut [f32]; 4],
    n: usize,
    premultiplied: bool,
    clamp: bool,
) {
    for x in 0..n {
        let fa = if clamp {
            clamp_f32(fg[3][x], 0.0, 1.0)
        } else {
            fg[3][x]
        };
        for c in 0..3 {
            out[c][x] = blend_sample(bg[c][x], bg[3][x], fg[c][x], fa, premultiplied);
        }
        out[3][x] = new_alpha(fa, bg[3][x]);
    }
}
pub fn perform_alpha_weighted_add(
    bg: &[f32],
    fg: &[f32],
    fga: &[f32],
    out: &mut [f32],
    n: usize,
    clamp: bool,
) {
    if std::ptr::eq(fg.as_ptr(), fga.as_ptr()) {
        out[..n].copy_from_slice(&bg[..n]);
    } else {
        for x in 0..n {
            let fa = if clamp {
                clamp_f32(fga[x], 0.0, 1.0)
            } else {
                fga[x]
            };
            out[x] = mul_add_f32(fg[x], fa, bg[x]);
        }
    }
}
pub fn perform_mul_blending(bg: &[f32], fg: &[f32], out: &mut [f32], n: usize, clamp: bool) {
    for x in 0..n {
        out[x] = bg[x]
            * if clamp {
                clamp_f32(fg[x], 0.0, 1.0)
            } else {
                fg[x]
            };
    }
}
// The selected pristine alpha TU retains std::max's original comparison
// boundary with -fno-inline-functions in both reference graphs. The default
// optimized ARM ordinary graph otherwise emits FMAXNM, whose signaling-NaN
// choice differs from the instrumented graph. tools/alpha_reference.py audits
// all finite results against both unchanged default objects.
fn alpha_max(alpha: f32) -> f32 {
    super::base::float::max_f32(SMALL_ALPHA, alpha)
}
pub fn premultiply_alpha(rgb: &mut [&mut [f32]; 3], alpha: &[f32], n: usize) {
    for x in 0..n {
        let multiplier = alpha_max(alpha[x]);
        for row in rgb.iter_mut() {
            row[x] *= multiplier;
        }
    }
}
pub fn unpremultiply_alpha(rgb: &mut [&mut [f32]; 3], alpha: &[f32], n: usize) {
    for x in 0..n {
        let multiplier = 1.0 / alpha_max(alpha[x]);
        for row in rgb.iter_mut() {
            row[x] *= multiplier;
        }
    }
}
