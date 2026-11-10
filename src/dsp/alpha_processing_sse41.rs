// libwebp v1.6.0, src/dsp/alpha_processing_sse41.c:
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
// Utilities for processing transparent channel, SSE4.1 variant.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/alpha_processing_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE4.1 alpha extraction that `WebPInitAlphaProcessingSSE41` assigns to
//! `WebPExtractAlpha`, behind `webp_extract_alpha` in
//! `src/dsp/alpha_processing.rs` (ADR 0008). `WebPInitAlphaProcessing` runs
//! `WebPInitAlphaProcessingSSE41` only inside its SSE2 branch, so the dispatch
//! function selects [`extract_alpha`] when both `features().sse2` and
//! `features().sse4_1` hold, before the SSE2 variant.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Panics on short buffers.** Each vector access slices the exact span it
//!    reads or writes, so a short buffer panics at that slice index, possibly
//!    before or after other writes than the scalar function makes
//!    (`common_sse2.rs`, deviation 3). Valid inputs never reach it.
//!
//! # Excluded undefined behavior
//!
//! 1. **Negative strides.** A negative stride panics at its checked
//!    conversion, as in `alpha_processing.rs`.
//!
//! # Checks not applicable
//!
//! 1. **`WebPInitAlphaProcessingSSE41`** only assigns a pointer (ADR 0003); its
//!    assignment is the selection in `webp_extract_alpha`.
//! 2. **Differential probe.** The parity test compares the function with the C
//!    translation it replaces (`tests/wp18_alpha.rs`).

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    _mm_and_si128, _mm_cmpeq_epi8, _mm_movemask_epi8, _mm_or_si128, _mm_set_epi8, _mm_set1_epi32,
    _mm_shuffle_epi8,
};

use crate::dsp::common_sse2::{loadu_si128, storeu_si128};

/// A C `int` index or count that the loops keep non-negative, as `usize`.
fn idx(i: i32) -> usize {
    usize::try_from(i).expect("a non-negative index")
}

/// A C `int` stride, which the scalar translation requires to be non-negative.
fn stride(value: i32) -> usize {
    usize::try_from(value).expect("the stride is not negative")
}

/// The low byte of a C `uint32_t`, as a conversion to `uint8_t` keeps it.
#[expect(
    clippy::cast_possible_truncation,
    reason = "a C conversion to uint8_t keeps the low eight bits"
)]
fn low_byte(value: u32) -> u8 {
    value as u8
}

//------------------------------------------------------------------------------

// upstream: src/dsp/alpha_processing_sse41.c ExtractAlpha_SSE41
#[target_feature(enable = "sse4.1")]
fn extract_alpha_sse41(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    // alpha_and stores an 'and' operation of all the alpha[] values. The final
    // value is not 0xff if any of the alpha[] is not equal to 0xff.
    let mut alpha_and: u32 = 0xff;
    let all_0xff = _mm_set1_epi32(!0);
    let mut all_alphas = all_0xff;
    let argb_stride = stride(argb_stride);
    let alpha_stride = stride(alpha_stride);

    // We must be able to access 3 extra bytes after the last written byte
    // 'src[4 * width - 4]', because we don't know if alpha is the first or the
    // last byte of the quadruplet.
    let limit = (width - 1) & !15;
    let k_cst_alpha0 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 12, 8, 4, 0);
    let k_cst_alpha1 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, 12, 8, 4, 0, -1, -1, -1, -1);
    let k_cst_alpha2 = _mm_set_epi8(-1, -1, -1, -1, 12, 8, 4, 0, -1, -1, -1, -1, -1, -1, -1, -1);
    let k_cst_alpha3 = _mm_set_epi8(12, 8, 4, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
    for j in 0..idx(height.max(0)) {
        let argb = &argb[j * argb_stride..];
        let alpha = &mut alpha[j * alpha_stride..];
        let mut src = 0;
        let mut i = 0;
        while i < limit {
            // load 64 argb bytes
            let a0 = loadu_si128(&argb[src..src + 16]);
            let a1 = loadu_si128(&argb[src + 16..src + 32]);
            let a2 = loadu_si128(&argb[src + 32..src + 48]);
            let a3 = loadu_si128(&argb[src + 48..src + 64]);
            let b0 = _mm_shuffle_epi8(a0, k_cst_alpha0);
            let b1 = _mm_shuffle_epi8(a1, k_cst_alpha1);
            let b2 = _mm_shuffle_epi8(a2, k_cst_alpha2);
            let b3 = _mm_shuffle_epi8(a3, k_cst_alpha3);
            let c0 = _mm_or_si128(b0, b1);
            let c1 = _mm_or_si128(b2, b3);
            let d0 = _mm_or_si128(c0, c1);
            // store
            storeu_si128(&mut alpha[idx(i)..idx(i) + 16], d0);
            // accumulate sixteen alpha 'and' in parallel
            all_alphas = _mm_and_si128(all_alphas, d0);
            src += 64;
            i += 16;
        }
        while i < width {
            let alpha_value = u32::from(argb[4 * idx(i)]);
            alpha[idx(i)] = low_byte(alpha_value);
            alpha_and &= alpha_value;
            i += 1;
        }
    }
    // Combine the sixteen alpha 'and' into an 8-bit mask.
    alpha_and |= 0xff00; // pretend the upper bits [8..15] were tested ok.
    alpha_and &= _mm_movemask_epi8(_mm_cmpeq_epi8(all_alphas, all_0xff)).cast_unsigned();
    alpha_and == 0xffff
}

/// Entry point of [`extract_alpha_sse41`] for `webp_extract_alpha`.
pub(crate) fn extract_alpha(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    // SAFETY: `webp_extract_alpha` calls this entry point only after
    // `features().sse2` and `features().sse4_1`, the answers of
    // `VP8GetCPUInfo(kSSE2)` and `VP8GetCPUInfo(kSSE4_1)`, selected the SSE4.1
    // variant, so the CPU has SSE4.1, which the function requires.
    unsafe { extract_alpha_sse41(argb, argb_stride, width, height, alpha, alpha_stride) }
}

// upstream: src/dsp/alpha_processing_sse41.c WebPInitAlphaProcessingSSE41 (no counterpart: it only assigns WebPExtractAlpha, ADR 0003)
