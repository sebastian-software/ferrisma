// libwebp v1.6.0, src/dsp/dec_sse41.c:
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
// SSE4 version of some decoding functions.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/dec_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE4.1 horizontal 16x16 predictor that `VP8DspInitSSE41` assigns to
//! `VP8PredLuma16[3]`, behind `vp8_pred_luma16` in `src/dsp/dec.rs` (ADR
//! 0008). `VP8DspInit` runs `VP8DspInitSSE41` only inside its SSE2 branch, so
//! the dispatch function selects [`he16`] when both `features().sse2` and
//! `features().sse4_1` hold, before the SSE2 variant.
//!
//! # Deviations from upstream
//!
//! 1. **Reads beyond the scalar footprint.** `HE16_SSE41` reads the four
//!    bytes before each row (`WebPMemToInt32(dst - 4)`) where `HE16_C` reads
//!    one, so the block must start at least four bytes into the buffer. The
//!    decoder's work buffer always has them, as in C.
//!
//! # Excluded undefined behavior
//!
//! None.
//!
//! # Checks not applicable
//!
//! 1. **`VP8DspInitSSE41`** only assigns a pointer (ADR 0003); its assignment
//!    is the selection in `vp8_pred_luma16`.
//! 2. **Differential probe.** The parity test compares the function with the C
//!    translation it replaces (`tests/wp18_dec.rs`).

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{_mm_cvtsi32_si128, _mm_set1_epi8, _mm_shuffle_epi8};

use crate::dsp::common_sse2::storeu_si128;
use crate::dsp::dsp::BPS;
use crate::utils::utils::webp_mem_to_int32;

/// `BPS` as an index step.
const BPS_U: usize = BPS as usize;

// upstream: src/dsp/dec_sse41.c HE16_SSE41
/// horizontal
#[target_feature(enable = "sse4.1")]
fn he16_sse41(buf: &mut [u8], dst: usize) {
    let mut dst = dst;
    let k_shuffle3 = _mm_set1_epi8(3);
    for _ in 0..16 {
        let input = _mm_cvtsi32_si128(webp_mem_to_int32(&buf[dst - 4..]));
        let values = _mm_shuffle_epi8(input, k_shuffle3);
        storeu_si128(&mut buf[dst..], values);
        dst += BPS_U;
    }
}

/// Entry point of [`he16_sse41`] for `vp8_pred_luma16` mode 3.
pub(crate) fn he16(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` and `features().sse4_1`, the answers of
    // `VP8GetCPUInfo(kSSE2)` and `VP8GetCPUInfo(kSSE4_1)`, selected the SSE4.1
    // variant, so the CPU has SSE4.1, which the function requires.
    unsafe { he16_sse41(buf, dst) }
}

// upstream: src/dsp/dec_sse41.c VP8DspInitSSE41 (no counterpart: it only assigns VP8PredLuma16[3], ADR 0003)
