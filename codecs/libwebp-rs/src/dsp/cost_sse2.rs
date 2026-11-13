// libwebp v1.6.0, src/dsp/cost_sse2.c:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// SSE2 version of cost functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/cost_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE2 residual-cost functions that `VP8EncDspCostInitSSE2` assigns,
//! behind the dispatch functions of `src/dsp/cost.rs` (ADR 0008).
//! `VP8EncDspCostInit` runs `VP8EncDspCostInitSSE2` when the CPU query is not
//! `NULL` and answers `kSSE2`, so `vp8_set_residual_coeffs` and
//! `vp8_get_residual_cost` select `set_residual_coeffs` and
//! `get_residual_cost` when `features().sse2` holds.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **`res->coeffs`** is a slice of at least 16 coefficients and
//!    `res->costs` the cost table of `src/enc/cost_enc.rs`, whose `row` is the
//!    C `costs[n][ctx]`, as in `src/dsp/cost.rs`.
//! 3. **Assertions** are `debug_assert!` (ADR 0003).
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions'.
//!
//! # Checks not applicable
//!
//! 1. **`VP8EncDspCostInitSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    _mm_cmpeq_epi8, _mm_max_epi16, _mm_min_epu8, _mm_movemask_epi8, _mm_packs_epi16, _mm_set1_epi8,
    _mm_setzero_si128, _mm_sub_epi16,
};

use crate::dsp::common_sse2::{loadu_si128, storeu_si128};
use crate::dsp::cost::{VP8_ENC_BANDS, VP8_LEVEL_FIXED_COSTS};
use crate::enc::cost_enc::{VP8Residual, vp8_bit_cost};
use crate::enc::vp8i_enc::MAX_VARIABLE_LEVEL;
use crate::utils::utils::bits_log2_floor;

// upstream: src/dsp/cost_sse2.c SetResidualCoeffs_SSE2
#[target_feature(enable = "sse2")]
fn set_residual_coeffs_sse2<'a>(coeffs: &'a [i16], res: &mut VP8Residual<'a>) {
    let c0 = loadu_si128(coeffs);
    let c1 = loadu_si128(&coeffs[8..]);
    // Use SSE2 to compare 16 values with a single instruction.
    let zero = _mm_setzero_si128();
    let m0 = _mm_packs_epi16(c0, c1);
    let m1 = _mm_cmpeq_epi8(m0, zero);
    // Get the comparison results as a bitmask into 16bits. Negate the mask to get
    // the position of entries that are not equal to zero. We don't need to mask
    // out least significant bits according to res->first, since coeffs[0] is 0
    // if res->first > 0.
    let mask: u32 = 0x0000_ffff ^ _mm_movemask_epi8(m1).cast_unsigned();
    // The position of the most significant non-zero bit indicates the position of
    // the last non-zero value.
    debug_assert!(res.first == 0 || coeffs[0] == 0);
    res.last = if mask != 0 { bits_log2_floor(mask) } else { -1 };
    res.coeffs = coeffs;
}

/// Entry point of `SetResidualCoeffs_SSE2` for `VP8SetResidualCoeffs`.
pub(crate) fn set_residual_coeffs<'a>(coeffs: &'a [i16], res: &mut VP8Residual<'a>) {
    // SAFETY: `vp8_set_residual_coeffs` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has SSE2.
    unsafe { set_residual_coeffs_sse2(coeffs, res) }
}

// upstream: src/dsp/cost_sse2.c GetResidualCost_SSE2
#[target_feature(enable = "sse2")]
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "MAX_VARIABLE_LEVEL is 67, and the positions and contexts are non-negative indexes"
)]
#[expect(non_snake_case, reason = "the C names D0 to H, and the kCst constants")]
#[expect(clippy::similar_names, reason = "the C names ctx0 and ctxs")]
fn get_residual_cost_sse2(ctx0: i32, res: &VP8Residual<'_>) -> i32 {
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
        let zero = _mm_setzero_si128();
        let kCst2 = _mm_set1_epi8(2);
        let kCst67 = _mm_set1_epi8(MAX_VARIABLE_LEVEL as i8);
        let c0 = loadu_si128(res.coeffs);
        let c1 = loadu_si128(&res.coeffs[8..]);
        let D0 = _mm_sub_epi16(zero, c0);
        let D1 = _mm_sub_epi16(zero, c1);
        let E0 = _mm_max_epi16(c0, D0); // abs(v), 16b
        let E1 = _mm_max_epi16(c1, D1);
        let F = _mm_packs_epi16(E0, E1);
        let G = _mm_min_epu8(F, kCst2); // context = 0,1,2
        let H = _mm_min_epu8(F, kCst67); // clamp_level in [0..67]

        storeu_si128(&mut ctxs, G);
        storeu_si128(&mut levels, H);

        storeu_si128(&mut abs_levels, E0);
        storeu_si128(&mut abs_levels[8..], E1);
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

/// Entry point of `GetResidualCost_SSE2` for `VP8GetResidualCost`.
#[must_use]
pub(crate) fn get_residual_cost(ctx0: i32, res: &VP8Residual<'_>) -> i32 {
    // SAFETY: `vp8_get_residual_cost` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has SSE2.
    unsafe { get_residual_cost_sse2(ctx0, res) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/cost_sse2.c VP8EncDspCostInitSSE2
// The pointer assignments are the dispatch functions of `src/dsp/cost.rs`
// named in the module documentation (ADR 0008).
