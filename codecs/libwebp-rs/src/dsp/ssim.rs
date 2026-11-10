// libwebp v1.6.0, src/dsp/ssim.c:
//
// Copyright 2017 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// distortion calculation
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/ssim.c` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **`ssim_calculation` uses `f64` division** of the two 64-bit sums, as the
//!    C `double` division does; the result is compared bit for bit on the
//!    development host.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Floating point.** The SSIM functions are compared bit for bit on the
//!    development host; the full text is in `# Checks not applicable` (item 1)
//!    of `src/utils/quant_levels_utils.rs`.
//! 2. **Assertions.** The C `assert`s are `debug_assert!` (ADR 0003). The SSIM
//!    assertion `0 <= r <= 1` holds for the statistics of real windows, which
//!    the tests use; the statistics of arbitrary sums can fail it in the C
//!    debug build too, and no caller of the public API produces them.

use crate::dsp::dsp::{VP8_SSIM_KERNEL, VP8DistoStats};

// upstream: src/dsp/ssim.c kWeight
/// Hat-shaped filter. The sum of its coefficients is 16.
const WEIGHT: [u32; (2 * VP8_SSIM_KERNEL + 1) as usize] = [1, 2, 3, 4, 3, 2, 1];

// upstream: src/dsp/ssim.c kWeightSum
/// The sum of the squared weights, `sum{kWeight}^2`.
const WEIGHT_SUM: u32 = 16 * 16;

// upstream: src/dsp/ssim.c SSIMCalculation
/// The SSIM of the statistics of a window of `n` samples. Areas that are too
/// dark to contribute meaningfully score 1.
#[expect(
    clippy::cast_sign_loss,
    reason = "the C code converts the non-negative correlation sum to uint64_t"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "the C code divides the two 64-bit sums as doubles"
)]
#[expect(
    clippy::similar_names,
    reason = "the C names xmym, xmxm and ymym of the SSIM formula"
)]
fn ssim_calculation(stats: &VP8DistoStats, n: u32) -> f64 {
    let w2 = n.wrapping_mul(n);
    let c1 = 20_u32.wrapping_mul(w2);
    let c2 = 60_u32.wrapping_mul(w2);
    let c3 = (8_u32 * 8).wrapping_mul(w2); // 'dark' limit ~= 6
    let xmxm = u64::from(stats.xm) * u64::from(stats.xm);
    let ymym = u64::from(stats.ym) * u64::from(stats.ym);
    if xmxm.wrapping_add(ymym) >= u64::from(c3) {
        let xmym = i64::from(stats.xm) * i64::from(stats.ym);
        let sxy = i64::from(stats.xym) * i64::from(n) - xmym; // can be negative
        let sxx = u64::from(stats.xxm)
            .wrapping_mul(u64::from(n))
            .wrapping_sub(xmxm);
        let syy = u64::from(stats.yym)
            .wrapping_mul(u64::from(n))
            .wrapping_sub(ymym);
        // we descale by 8 to prevent overflow during the fnum/fden multiply.
        let num_s = (2 * (if sxy < 0 { 0 } else { sxy as u64 }) + u64::from(c2)) >> 8;
        let den_s = sxx.wrapping_add(syy).wrapping_add(u64::from(c2)) >> 8;
        let fnum = ((2 * xmym + i64::from(c1)) as u64).wrapping_mul(num_s);
        let fden = xmxm
            .wrapping_add(ymym)
            .wrapping_add(u64::from(c1))
            .wrapping_mul(den_s);
        let r = fnum as f64 / fden as f64;
        debug_assert!((0.0..=1.0).contains(&r));
        return r;
    }
    1.0 // area is too dark to contribute meaningfully
}

// upstream: src/dsp/ssim.c VP8SSIMFromStats
/// The SSIM of the statistics of a full kernel window.
#[must_use]
pub fn vp8ssim_from_stats(stats: &VP8DistoStats) -> f64 {
    ssim_calculation(stats, WEIGHT_SUM)
}

// upstream: src/dsp/ssim.c VP8SSIMFromStatsClipped
/// The SSIM of the statistics of a window, with the sum of its weights as the
/// sample count.
#[must_use]
pub fn vp8ssim_from_stats_clipped(stats: &VP8DistoStats) -> f64 {
    ssim_calculation(stats, stats.w)
}

// upstream: src/dsp/ssim.c SSIMGetClipped_C
/// The SSIM of the window centered on `(xo, yo)`, clipped to the `w` by `h`
/// plane. The planes are read at the absolute column and row positions.
///
/// # Panics
///
/// Panics when a plane is shorter than the window's rows and columns.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of VP8SSIMGetClippedFunc"
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "the clipped window starts at a non-negative row and column"
)]
fn ssim_get_clipped_c(
    src1: &[u8],
    stride1: i32,
    src2: &[u8],
    stride2: i32,
    xo: i32,
    yo: i32,
    w: i32,
    h: i32,
) -> f64 {
    let mut stats = VP8DistoStats::default();
    let kernel = VP8_SSIM_KERNEL;
    let ymin = if yo - kernel < 0 { 0 } else { yo - kernel };
    let ymax = if yo + kernel > h - 1 {
        h - 1
    } else {
        yo + kernel
    };
    let xmin = if xo - kernel < 0 { 0 } else { xo - kernel };
    let xmax = if xo + kernel > w - 1 {
        w - 1
    } else {
        xo + kernel
    };
    let mut row1 = ymin as usize * stride1 as usize;
    let mut row2 = ymin as usize * stride2 as usize;
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let wy = WEIGHT[(kernel + y - yo) as usize];
            let wx = WEIGHT[(kernel + x - xo) as usize];
            let weight = wx * wy;
            let s1 = u32::from(src1[row1 + x as usize]);
            let s2 = u32::from(src2[row2 + x as usize]);
            stats.w += weight;
            stats.xm += weight * s1;
            stats.ym += weight * s2;
            stats.xxm += weight * s1 * s1;
            stats.xym += weight * s1 * s2;
            stats.yym += weight * s2 * s2;
        }
        row1 += stride1 as usize;
        row2 += stride2 as usize;
    }
    vp8ssim_from_stats_clipped(&stats)
}

// upstream: src/dsp/ssim.c SSIMGet_C
/// The SSIM of the full kernel window at the top-left corner of the planes.
///
/// # Panics
///
/// Panics when a plane is shorter than the kernel's rows and columns.
#[must_use]
#[expect(clippy::cast_sign_loss, reason = "the strides are non-negative")]
fn ssim_get_c(src1: &[u8], stride1: i32, src2: &[u8], stride2: i32) -> f64 {
    let mut stats = VP8DistoStats::default();
    let mut row1 = 0_usize;
    let mut row2 = 0_usize;
    for &wy in &WEIGHT {
        for (x, &wx) in WEIGHT.iter().enumerate() {
            let weight = wx * wy;
            let s1 = u32::from(src1[row1 + x]);
            let s2 = u32::from(src2[row2 + x]);
            stats.xm += weight * s1;
            stats.ym += weight * s2;
            stats.xxm += weight * s1 * s1;
            stats.xym += weight * s1 * s2;
            stats.yym += weight * s2 * s2;
        }
        row1 += stride1 as usize;
        row2 += stride2 as usize;
    }
    vp8ssim_from_stats(&stats)
}

// upstream: src/dsp/ssim.c AccumulateSSE_C
/// The sum of squared differences of `len` samples.
///
/// # Panics
///
/// Panics when a slice is shorter than `len`.
#[must_use]
fn accumulate_sse_c(src1: &[u8], src2: &[u8], len: i32) -> u32 {
    let len = usize::try_from(len).unwrap_or(0);
    let mut sse2: u32 = 0;
    for i in 0..len {
        let diff = i32::from(src1[i]) - i32::from(src2[i]);
        sse2 = sse2.wrapping_add((diff * diff).cast_unsigned());
    }
    sse2
}

// upstream: src/dsp/ssim.c VP8SSIMGetClipped
/// The clipped SSIM of a window. The C dispatch pointer is `SSIMGetClipped_C`.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the parameters of VP8SSIMGetClippedFunc"
)]
pub fn vp8ssim_get_clipped(
    src1: &[u8],
    stride1: i32,
    src2: &[u8],
    stride2: i32,
    xo: i32,
    yo: i32,
    w: i32,
    h: i32,
) -> f64 {
    ssim_get_clipped_c(src1, stride1, src2, stride2, xo, yo, w, h)
}

// upstream: src/dsp/ssim.c VP8SSIMGet
/// The SSIM of the full kernel window. The C dispatch pointer is `SSIMGet_C`.
#[must_use]
pub fn vp8ssim_get(src1: &[u8], stride1: i32, src2: &[u8], stride2: i32) -> f64 {
    ssim_get_c(src1, stride1, src2, stride2)
}

// upstream: src/dsp/ssim.c VP8AccumulateSSE
/// The sum of squared differences. The C dispatch pointer is `AccumulateSSE_C`.
#[must_use]
pub fn vp8_accumulate_sse(src1: &[u8], src2: &[u8], len: i32) -> u32 {
    accumulate_sse_c(src1, src2, len)
}
