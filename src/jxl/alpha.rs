// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Scalar alpha operations from lib/jxl/alpha.{h,cc}. Borrowed input slices
//! retain source pointer-identity branches; owned temporary output permits
//! the source's in-place callers without aliased mutable references.
use super::base::float::clamp_f32;
#[cfg(target_arch = "aarch64")]
use super::base::float::mul_add_f32;
pub const SMALL_ALPHA: f32 = 1.0 / (1u32 << 26) as f32;

#[inline(never)]
fn mul(a: f32, b: f32) -> f32 {
    a * b
}
#[cfg(not(target_arch = "aarch64"))]
#[inline(never)]
fn add(a: f32, b: f32) -> f32 {
    a + b
}
fn new_alpha(fa: f32, ba: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        mul_add_f32(-(1.0 - fa), 1.0 - ba, 1.0)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        // Preserve the source subtraction rather than negating one product
        // operand: non-fused multiplication retains the operand's NaN sign.
        1.0 - mul(1.0 - fa, 1.0 - ba)
    }
}
fn blend_sample(bg: f32, ba: f32, fg: f32, fa: f32, premultiplied: bool) -> f32 {
    if premultiplied {
        #[cfg(target_arch = "aarch64")]
        {
            mul_add_f32(bg, 1.0 - fa, fg)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            add(fg, mul(bg, 1.0 - fa))
        }
    } else {
        let a = new_alpha(fa, ba);
        let ra = if a > 0.0 { 1.0 / a } else { 0.0 };
        let background = mul(mul(bg, ba), 1.0 - fa);
        #[cfg(target_arch = "aarch64")]
        let sum = mul_add_f32(fg, fa, background);
        #[cfg(not(target_arch = "aarch64"))]
        let sum = add(mul(fg, fa), background);
        mul(sum, ra)
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
            #[cfg(target_arch = "aarch64")]
            {
                out[x] = mul_add_f32(fg[x], fa, bg[x]);
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                out[x] = add(bg[x], mul(fg[x], fa));
            }
        }
    }
}
pub fn perform_mul_blending(bg: &[f32], fg: &[f32], out: &mut [f32], n: usize, clamp: bool) {
    for x in 0..n {
        // Keep the selected original's literal background-first product across
        // Rust optimization, including competing NaN operands after Clamp.
        out[x] = mul(
            bg[x],
            if clamp {
                clamp_f32(fg[x], 0.0, 1.0)
            } else {
                fg[x]
            },
        );
    }
}
// The selected pristine alpha TU retains scalar expression order and std::max's
// comparison with scoped O0/no-inline, plus strict FP only on x86 Linux. ARM
// retains source contraction. The optimized default ARM graph otherwise emits
// FMAXNM with different signaling-NaN behavior; tools/alpha_reference.py audits
// every finite request against both untouched default objects.
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
