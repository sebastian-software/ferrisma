// libwebp v1.6.0, src/dsp/common_sse41.h:
//
// Copyright 2016 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// SSE4 code common to several files.
//
// Author: Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/dsp/common_sse41.h` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder); also used by the WP19 encoder modules.
//!
//! The SSE4.1 channel-mixing helpers that `yuv_sse41.rs` uses and the
//! encoder's SSE4.1 modules of WP19 may share: the planar to packed RGB
//! interleave and the packed ARGB to planar split, each a
//! `#[target_feature(enable = "sse4.1")]` function that the SSE4.1 functions
//! of the SIMD modules call safely (ADR 0008). The checked loads and stores
//! are those of `common_sse2.rs`.
//!
//! # Deviations from upstream
//!
//! 1. **Pointer parameters.** The `__m128i* const` parameters become
//!    `&mut __m128i`; `WEBP_SSE41_SHUFF`, a macro over named registers and
//!    the three shuffle masks in scope, becomes a function of the two inputs
//!    and the three masks that returns the six registers.
//!
//! # Excluded undefined behavior
//!
//! None: the helpers are arithmetic on registers.
//!
//! # Checks not applicable
//!
//! 1. **Differential probe.** The helpers have no dispatch pointer of their
//!    own; the parity tests of the functions that use them cover them
//!    (`tests/wp18_yuv.rs`, `tests/wp18_upsampling.rs`).
//!
//! # Notes for WP19
//!
//! - **Helpers.** `vp8_planar_to24b_sse41` and `vp8l32b_to_planar_sse41`
//!   keep the C names under ADR 0001's rule and the C argument order, with
//!   `&mut __m128i` out-parameters, as `common_sse2.rs` does for SSE2. Load
//!   and store through the checked helpers of `common_sse2.rs`
//!   (`loadu_si128`, `storeu_si128`, `loadl_epi64`, `storel_epi64`), which
//!   an SSE4.1 function calls safely because SSE4.1 implies SSE2.
//! - **Entry points and `Init` order.** As in `common_sse2.rs`: a
//!   `#[target_feature(enable = "sse4.1")]` body keeps the upstream name and
//!   a safe `pub(crate)` entry point without the suffix performs the one
//!   `unsafe` call after the dispatch function tested the `features()` flags
//!   that upstream's `Init` tests for that pointer. Mirror the nesting of the
//!   `Init` function: `WebPInitConvertARGBToYUV` and `WebPInitSamplers` test
//!   SSE4.1 independently of SSE2, `VP8DspInit` and `WebPInitAlphaProcessing`
//!   only inside the SSE2 branch.
//! - **Tests.** `tests/wp18_support/mod.rs` selects the SSE4.1 level with a
//!   `VP8GetCPUInfo` query that answers `kSSE2` to `kSSE4_1`; compare at every
//!   level with the C level's output (`tests/wp18_yuv.rs` shows the pattern).

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_or_si128, _mm_set_epi8, _mm_shuffle_epi8, _mm_unpackhi_epi32, _mm_unpackhi_epi64,
    _mm_unpacklo_epi32, _mm_unpacklo_epi64,
};

//------------------------------------------------------------------------------
// Channel mixing.

// upstream: src/dsp/common_sse41.h WEBP_SSE41_SHUFF
/// Shuffles the input buffer as A0 0 0 A1 0 0 A2 ...
#[target_feature(enable = "sse4.1")]
#[inline]
fn webp_sse41_shuff(
    in0: &__m128i,
    in1: &__m128i,
    shuff0: __m128i,
    shuff1: __m128i,
    shuff2: __m128i,
) -> [__m128i; 6] {
    [
        _mm_shuffle_epi8(*in0, shuff0),
        _mm_shuffle_epi8(*in0, shuff1),
        _mm_shuffle_epi8(*in0, shuff2),
        _mm_shuffle_epi8(*in1, shuff0),
        _mm_shuffle_epi8(*in1, shuff1),
        _mm_shuffle_epi8(*in1, shuff2),
    ]
}

// upstream: src/dsp/common_sse41.h VP8PlanarTo24b_SSE41
/// Pack the planar buffers
/// rrrr... rrrr... gggg... gggg... bbbb... bbbb....
/// triplet by triplet in the output buffer rgb as rgbrgbrgbrgb ...
#[target_feature(enable = "sse4.1")]
#[inline]
pub(crate) fn vp8_planar_to24b_sse41(
    in0: &mut __m128i,
    in1: &mut __m128i,
    in2: &mut __m128i,
    in3: &mut __m128i,
    in4: &mut __m128i,
    in5: &mut __m128i,
) {
    // Process R.
    let r = {
        let shuff0 = _mm_set_epi8(5, -1, -1, 4, -1, -1, 3, -1, -1, 2, -1, -1, 1, -1, -1, 0);
        let shuff1 = _mm_set_epi8(-1, 10, -1, -1, 9, -1, -1, 8, -1, -1, 7, -1, -1, 6, -1, -1);
        let shuff2 = _mm_set_epi8(
            -1, -1, 15, -1, -1, 14, -1, -1, 13, -1, -1, 12, -1, -1, 11, -1,
        );
        webp_sse41_shuff(in0, in1, shuff0, shuff1, shuff2)
    };

    // Process G.
    let g = {
        // Same as before, just shifted to the left by one and including the right
        // padding.
        let shuff0 = _mm_set_epi8(-1, -1, 4, -1, -1, 3, -1, -1, 2, -1, -1, 1, -1, -1, 0, -1);
        let shuff1 = _mm_set_epi8(10, -1, -1, 9, -1, -1, 8, -1, -1, 7, -1, -1, 6, -1, -1, 5);
        let shuff2 = _mm_set_epi8(
            -1, 15, -1, -1, 14, -1, -1, 13, -1, -1, 12, -1, -1, 11, -1, -1,
        );
        webp_sse41_shuff(in2, in3, shuff0, shuff1, shuff2)
    };

    // Process B.
    let b = {
        let shuff0 = _mm_set_epi8(-1, 4, -1, -1, 3, -1, -1, 2, -1, -1, 1, -1, -1, 0, -1, -1);
        let shuff1 = _mm_set_epi8(-1, -1, 9, -1, -1, 8, -1, -1, 7, -1, -1, 6, -1, -1, 5, -1);
        let shuff2 = _mm_set_epi8(
            15, -1, -1, 14, -1, -1, 13, -1, -1, 12, -1, -1, 11, -1, -1, 10,
        );
        webp_sse41_shuff(in4, in5, shuff0, shuff1, shuff2)
    };

    // OR the different channels.
    {
        let rg0 = _mm_or_si128(r[0], g[0]);
        let rg1 = _mm_or_si128(r[1], g[1]);
        let rg2 = _mm_or_si128(r[2], g[2]);
        let rg3 = _mm_or_si128(r[3], g[3]);
        let rg4 = _mm_or_si128(r[4], g[4]);
        let rg5 = _mm_or_si128(r[5], g[5]);
        *in0 = _mm_or_si128(rg0, b[0]);
        *in1 = _mm_or_si128(rg1, b[1]);
        *in2 = _mm_or_si128(rg2, b[2]);
        *in3 = _mm_or_si128(rg3, b[3]);
        *in4 = _mm_or_si128(rg4, b[4]);
        *in5 = _mm_or_si128(rg5, b[5]);
    }
}

// upstream: src/dsp/common_sse41.h VP8L32bToPlanar_SSE41
/// Convert four packed four-channel buffers like argbargbargbargb... into the
/// split channels aaaaa ... rrrr ... gggg .... bbbbb ......
#[target_feature(enable = "sse4.1")]
#[inline]
pub(crate) fn vp8l32b_to_planar_sse41(
    in0: &mut __m128i,
    in1: &mut __m128i,
    in2: &mut __m128i,
    in3: &mut __m128i,
) {
    // aaaarrrrggggbbbb
    let shuff0 = _mm_set_epi8(15, 11, 7, 3, 14, 10, 6, 2, 13, 9, 5, 1, 12, 8, 4, 0);
    let a0 = _mm_shuffle_epi8(*in0, shuff0);
    let a1 = _mm_shuffle_epi8(*in1, shuff0);
    let a2 = _mm_shuffle_epi8(*in2, shuff0);
    let a3 = _mm_shuffle_epi8(*in3, shuff0);
    // A0A1R0R1
    // G0G1B0B1
    // A2A3R2R3
    // G0G1B0B1
    let b0 = _mm_unpacklo_epi32(a0, a1);
    let b1 = _mm_unpackhi_epi32(a0, a1);
    let b2 = _mm_unpacklo_epi32(a2, a3);
    let b3 = _mm_unpackhi_epi32(a2, a3);
    *in3 = _mm_unpacklo_epi64(b0, b2);
    *in2 = _mm_unpackhi_epi64(b0, b2);
    *in1 = _mm_unpacklo_epi64(b1, b3);
    *in0 = _mm_unpackhi_epi64(b1, b3);
}
