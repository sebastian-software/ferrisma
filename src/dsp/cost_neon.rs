// libwebp v1.6.0, src/dsp/cost_neon.c:
//
// Copyright 2018 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// ARM NEON version of cost functions

//! Translation of `src/dsp/cost_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the residual cost functions of `src/dsp/cost.c`.
//! `VP8EncDspCostInitNEON` assigns `VP8SetResidualCoeffs`
//! (`SetResidualCoeffs_NEON`) and `VP8GetResidualCost`
//! (`GetResidualCost_NEON`). Unlike the other NEON `Init` functions,
//! `VP8EncDspCostInit` calls it only when `VP8GetCPUInfo` is not `NULL` and
//! answers `kNEON`, not unconditionally; that answer is not in
//! `crate::dsp::cpu::Features`, whose `neon` is true on 64-bit ARM whatever
//! the pointer says, so the selection in the dispatch functions of
//! `src/dsp/cost.rs` waits for it (checks not applicable, 4).
//!
//! # Deviations from upstream
//!
//! 1. **Loads and stores** follow the deviations 1 and 2 of
//!    `src/dsp/neon.rs`; `levels`, `ctxs` and `abs_levels` are arrays that
//!    the stores fill.
//! 2. **`VP8Residual`** is the borrow form of `src/enc/cost_enc.rs`:
//!    `costs[n][ctx]` is `costs.row(n, ctx)`, and `res->coeffs` is the slice
//!    of the 16 levels.
//!
//! # Excluded undefined behavior
//!
//! None. A level of magnitude above `MAX_LEVEL` (2047) indexes
//! `VP8LevelFixedCosts` out of its bounds in C as in the C translation; the
//! encoder clamps its levels to `MAX_LEVEL`, and the Rust index panics.
//!
//! # Checks not applicable
//!
//! 1. **The pairwise-maximum branch of `SetResidualCoeffs_NEON`** is for
//!    32-bit ARM; `WEBP_AARCH64` takes `vmaxvq_u8`.
//! 2. **The assertion of `GetResidualCost_NEON`** (`flevel != 0`) is a
//!    `debug_assert!`, as in `src/dsp/cost.rs`.
//! 3. **`WEBP_RESTRICT`** has no counterpart: the slices cannot alias.
//! 4. **Selection.** `vp8_set_residual_coeffs` and `vp8_get_residual_cost`
//!    call the C functions until `Features` carries the query's `kNEON`
//!    answer that `VP8EncDspCostInit` tests; the parity test compares the
//!    entry points below with the C functions directly.

use core::arch::aarch64::{
    vabsq_s16, vandq_u8, vcombine_u8, vdupq_n_s16, vdupq_n_u8, vmaxvq_u8, vminq_u8, vqmovn_u16,
    vreinterpretq_u16_s16, vtstq_s16,
};

use crate::dsp::cost::{VP8_ENC_BANDS, VP8_LEVEL_FIXED_COSTS};
use crate::dsp::neon::{vld1q_s16, vld1q_u8, vst1q_u8, vst1q_u16};
use crate::enc::cost_enc::{VP8Residual, vp8_bit_cost};
use crate::enc::vp8i_enc::MAX_VARIABLE_LEVEL;

// upstream: src/dsp/cost_neon.c position
static POSITION: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

// upstream: src/dsp/cost_neon.c SetResidualCoeffs_NEON
#[target_feature(enable = "neon")]
fn set_residual_coeffs_neon<'a>(coeffs: &'a [i16], res: &mut VP8Residual<'a>) {
    let minus_one = vdupq_n_s16(-1);
    let coeffs_0 = vld1q_s16(coeffs);
    let coeffs_1 = vld1q_s16(&coeffs[8..]);
    let eob_0 = vtstq_s16(coeffs_0, minus_one);
    let eob_1 = vtstq_s16(coeffs_1, minus_one);
    let eob = vcombine_u8(vqmovn_u16(eob_0), vqmovn_u16(eob_1));
    let masked = vandq_u8(eob, vld1q_u8(&POSITION));

    res.last = i32::from(vmaxvq_u8(masked)) - 1;

    res.coeffs = coeffs;
}

/// Entry point of `SetResidualCoeffs_NEON` for `VP8SetResidualCoeffs`.
pub fn set_residual_coeffs<'a>(coeffs: &'a [i16], res: &mut VP8Residual<'a>) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for;
    // upstream's `VP8EncDspCostInit` selects this variant when the CPU query
    // answers `kNEON` (ADR 0008).
    unsafe { set_residual_coeffs_neon(coeffs, res) }
}

// upstream: src/dsp/cost_neon.c GetResidualCost_NEON
#[target_feature(enable = "neon")]
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "MAX_VARIABLE_LEVEL is 67, and the positions and contexts are non-negative indexes"
)]
#[expect(
    non_snake_case,
    reason = "the C names E0, E1, F, G and H, and the kCst constants"
)]
#[expect(clippy::similar_names, reason = "the C names ctx0 and ctxs")]
fn get_residual_cost_neon(ctx0: i32, res: &VP8Residual<'_>) -> i32 {
    let mut levels = [0_u8; 16];
    let mut ctxs = [0_u8; 16];
    let mut abs_levels = [0_u16; 16];
    let mut n = res.first;
    // should be prob[VP8EncBands[n]], but it's equivalent for n=0 or 1
    let p0 = res.prob[n as usize][ctx0 as usize][0];
    let costs = res.costs;
    let mut t = costs.row(n as usize, ctx0 as usize);
    // bit_cost(1, p0) is already incorporated in t[] tables, but only if ctx != 0
    // (as required by the syntax). For ctx0 == 0, we need to add it here or it'll
    // be missing during the loop.
    let mut cost = if ctx0 == 0 { vp8_bit_cost(true, p0) } else { 0 };

    if res.last < 0 {
        return vp8_bit_cost(false, p0);
    }

    {
        // precompute clamped levels and contexts, packed to 8b.
        let kCst2 = vdupq_n_u8(2);
        let kCst67 = vdupq_n_u8(MAX_VARIABLE_LEVEL as u8);
        let c0 = vld1q_s16(res.coeffs);
        let c1 = vld1q_s16(&res.coeffs[8..]);
        let E0 = vreinterpretq_u16_s16(vabsq_s16(c0));
        let E1 = vreinterpretq_u16_s16(vabsq_s16(c1));
        let F = vcombine_u8(vqmovn_u16(E0), vqmovn_u16(E1));
        let G = vminq_u8(F, kCst2); // context = 0,1,2
        let H = vminq_u8(F, kCst67); // clamp_level in [0..67]

        vst1q_u8(&mut ctxs, G);
        vst1q_u8(&mut levels, H);

        vst1q_u16(&mut abs_levels, E0);
        vst1q_u16(&mut abs_levels[8..], E1);
    }
    while n < res.last {
        let ctx = ctxs[n as usize];
        let level = levels[n as usize];
        let flevel = abs_levels[n as usize]; // full level
        // simplified VP8LevelCost()
        cost += i32::from(VP8_LEVEL_FIXED_COSTS[usize::from(flevel)])
            + i32::from(t[usize::from(level)]);
        t = costs.row(n as usize + 1, usize::from(ctx));
        n += 1;
    }
    // Last coefficient is always non-zero
    {
        let level = levels[n as usize];
        let flevel = abs_levels[n as usize];
        debug_assert!(flevel != 0);
        cost += i32::from(VP8_LEVEL_FIXED_COSTS[usize::from(flevel)])
            + i32::from(t[usize::from(level)]);
        if n < 15 {
            let b = VP8_ENC_BANDS[n as usize + 1];
            let ctx = ctxs[n as usize];
            let last_p0 = res.prob[usize::from(b)][usize::from(ctx)][0];
            cost += vp8_bit_cost(false, last_p0);
        }
    }
    cost
}

/// Entry point of `GetResidualCost_NEON` for `VP8GetResidualCost`.
#[must_use]
pub fn get_residual_cost(ctx0: i32, res: &VP8Residual<'_>) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for;
    // upstream's `VP8EncDspCostInit` selects this variant when the CPU query
    // answers `kNEON` (ADR 0008).
    unsafe { get_residual_cost_neon(ctx0, res) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/cost_neon.c VP8EncDspCostInitNEON
// The pointer assignments are the dispatch functions of `src/dsp/cost.rs`
// named in the module documentation (ADR 0008).
