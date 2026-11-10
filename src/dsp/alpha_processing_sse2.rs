// libwebp v1.6.0, src/dsp/alpha_processing_sse2.c:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Utilities for processing transparent channel.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/alpha_processing_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE2 alpha functions that `WebPInitAlphaProcessingSSE2` assigns, behind
//! the dispatch functions of `src/dsp/alpha_processing.rs` (ADR 0008). Each
//! `#[target_feature]` function keeps the upstream name; each function that
//! the `Init` function assigns to a pointer has a safe `pub(crate)` entry point
//! named after it without the suffix, which the dispatch function calls after
//! testing `features().sse2`. `WebPInitAlphaProcessingSSE2` assigns:
//!
//! - `WebPMultARGBRow`, `WebPMultRow`: [`mult_argb_row`], [`mult_row`];
//! - `WebPApplyAlphaMultiply`: [`apply_alpha_multiply`];
//! - `WebPDispatchAlpha`, `WebPDispatchAlphaToGreen`: [`dispatch_alpha`],
//!   [`dispatch_alpha_to_green`];
//! - `WebPExtractAlpha`, `WebPExtractGreen`: [`extract_alpha`],
//!   [`extract_green`] (`alpha_processing_sse41.rs` overrides the former);
//! - `WebPHasAlpha8b`, `WebPHasAlpha32b`, `WebPAlphaReplace`:
//!   [`has_alpha8b`], [`has_alpha32b`], [`alpha_replace`].
//!
//! `WebPApplyAlphaMultiply4444` and `WebPPackRGB` keep their C functions. The
//! buffers, strides and lengths follow `src/dsp/alpha_processing.rs` (WP4).
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Fence after the masked stores.** `DispatchAlpha_SSE2` writes the alpha
//!    bytes with `_mm_maskmoveu_si128`, a non-temporal store. Rust requires
//!    an `_mm_sfence` by the storing thread before any other access to the
//!    stored memory, so the translation issues one `_mm_sfence()` after the
//!    last row, before it returns. Upstream has no fence; the bytes and the
//!    result are the same, only the store ordering is made explicit.
//! 3. **Masked store bounds.** `_mm_maskmoveu_si128` writes only the bytes
//!    whose mask byte has its high bit set, but Rust requires its whole
//!    16-byte range to lie in the slice, and the checked helper
//!    `maskmoveu_si128` asserts so. The loop bound `i + 16 <= width - 1`
//!    keeps that range within the bytes the scalar function touches, so no
//!    valid input reaches the assertion.
//! 4. **Panics on short buffers.** Each vector access slices the exact span it
//!    reads or writes, so a buffer shorter than the function's rows panics at
//!    that slice index, with the message of the scalar translation's index
//!    panic, but possibly before or after other writes than the scalar
//!    function makes (`common_sse2.rs`, deviation 3). Valid inputs never
//!    reach it.
//!
//! # Excluded undefined behavior
//!
//! 1. **Negative strides.** The scalar translation converts the strides with a
//!    checked conversion, so a negative stride panics, as in
//!    `alpha_processing.rs`; this file does the same. Zero and negative
//!    widths, heights and lengths process nothing, as in C.
//!
//! # Checks not applicable
//!
//! 1. **`WebPInitAlphaProcessingSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.
//! 2. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_alpha.rs`); the reference oracle
//!    has no SSE2 code, and the diagnostic `release-simd` oracle runs the WP4
//!    and WP7 probes against upstream's SSE2.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_and_si128, _mm_andnot_si128, _mm_cmpeq_epi8, _mm_cmpeq_epi32,
    _mm_maskmoveu_si128, _mm_movemask_epi8, _mm_mulhi_epu16, _mm_mullo_epi16, _mm_or_si128,
    _mm_packs_epi32, _mm_packus_epi16, _mm_set_epi16, _mm_set_epi32, _mm_set1_epi8, _mm_set1_epi16,
    _mm_set1_epi32, _mm_setzero_si128, _mm_sfence, _mm_shufflehi_epi16, _mm_shufflelo_epi16,
    _mm_srai_epi32, _mm_srli_epi16, _mm_srli_epi32, _mm_unpackhi_epi8, _mm_unpackhi_epi16,
    _mm_unpacklo_epi8, _mm_unpacklo_epi16,
};

use crate::dsp::alpha_processing::{webp_mult_argb_row_c, webp_mult_row_c};
use crate::dsp::common_sse2::{loadl_epi64, loadu_si128, storel_epi64, storeu_si128};

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

/// The `_MM_SHUFFLE(z, y, x, w)` immediate.
const fn mm_shuffle(z: i32, y: i32, x: i32, w: i32) -> i32 {
    (z << 6) | (y << 4) | (x << 2) | w
}

/// Stores the bytes of `value` whose `mask` byte has its high bit set into the
/// first 16 bytes of `dst` (C: `_mm_maskmoveu_si128` at the start of `dst`).
/// The store is non-temporal: the caller issues `_mm_sfence()` before any
/// other access to the stored bytes (deviation 2).
///
/// # Panics
///
/// Panics when `dst` spans fewer than 16 bytes (deviation 3).
#[target_feature(enable = "sse2")]
#[inline]
fn maskmoveu_si128(value: __m128i, mask: __m128i, dst: &mut [u8]) {
    assert!(
        dst.len() >= 16,
        "a 16-byte masked store past the end of a slice"
    );
    // SAFETY: the assertion above checked that `dst` spans the 16 bytes that
    // the masked store may write, any byte is a valid `u8`, and the only
    // caller issues `_mm_sfence()` before the stored bytes are accessed again.
    unsafe { _mm_maskmoveu_si128(value, mask, dst.as_mut_ptr().cast()) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/alpha_processing_sse2.c DispatchAlpha_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::int_plus_one,
    reason = "the C loop bounds i + 16 <= width - 1 and i + 8 <= width - 1"
)]
fn dispatch_alpha_sse2(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_stride: i32,
) -> bool {
    // alpha_and stores an 'and' operation of all the alpha[] values. The final
    // value is not 0xff if any of the alpha[] is not equal to 0xff.
    let mut alpha_and: u32 = 0xff;
    let zero = _mm_setzero_si128();
    let alpha_mask = _mm_set1_epi32(0xff); // to preserve A
    let all_0xff = _mm_set1_epi8(0xff_u8.cast_signed());
    let mut all_alphas16 = all_0xff;
    let mut all_alphas8 = all_0xff;
    let alpha_stride = stride(alpha_stride);
    let dst_stride = stride(dst_stride);

    // We must be able to access 3 extra bytes after the last written byte
    // 'dst[4 * width - 4]', because we don't know if alpha is the first or the
    // last byte of the quadruplet.
    for j in 0..idx(height.max(0)) {
        let alpha = &alpha[j * alpha_stride..];
        let dst = &mut dst[j * dst_stride..];
        let mut ptr = 0;
        let mut i = 0;
        while i + 16 <= width - 1 {
            // load 16 alpha bytes
            let a0 = loadu_si128(&alpha[idx(i)..idx(i) + 16]);
            let a1_lo = _mm_unpacklo_epi8(a0, zero);
            let a1_hi = _mm_unpackhi_epi8(a0, zero);
            let a2_lo_lo = _mm_unpacklo_epi16(a1_lo, zero);
            let a2_lo_hi = _mm_unpackhi_epi16(a1_lo, zero);
            let a2_hi_lo = _mm_unpacklo_epi16(a1_hi, zero);
            let a2_hi_hi = _mm_unpackhi_epi16(a1_hi, zero);
            maskmoveu_si128(a2_lo_lo, alpha_mask, &mut dst[ptr..ptr + 16]);
            maskmoveu_si128(a2_lo_hi, alpha_mask, &mut dst[ptr + 16..ptr + 32]);
            maskmoveu_si128(a2_hi_lo, alpha_mask, &mut dst[ptr + 32..ptr + 48]);
            maskmoveu_si128(a2_hi_hi, alpha_mask, &mut dst[ptr + 48..ptr + 64]);
            // accumulate 16 alpha 'and' in parallel
            all_alphas16 = _mm_and_si128(all_alphas16, a0);
            ptr += 64;
            i += 16;
        }
        if i + 8 <= width - 1 {
            // load 8 alpha bytes
            let a0 = loadl_epi64(&alpha[idx(i)..idx(i) + 8]);
            let a1 = _mm_unpacklo_epi8(a0, zero);
            let a2_lo = _mm_unpacklo_epi16(a1, zero);
            let a2_hi = _mm_unpackhi_epi16(a1, zero);
            maskmoveu_si128(a2_lo, alpha_mask, &mut dst[ptr..ptr + 16]);
            maskmoveu_si128(a2_hi, alpha_mask, &mut dst[ptr + 16..ptr + 32]);
            // accumulate 8 alpha 'and' in parallel
            all_alphas8 = _mm_and_si128(all_alphas8, a0);
            i += 8;
        }
        while i < width {
            let alpha_value = u32::from(alpha[idx(i)]);
            dst[4 * idx(i)] = low_byte(alpha_value);
            alpha_and &= alpha_value;
            i += 1;
        }
    }
    // The masked stores above are non-temporal (deviation 2).
    _mm_sfence();
    // Combine the eight alpha 'and' into a 8-bit mask.
    alpha_and &= (_mm_movemask_epi8(_mm_cmpeq_epi8(all_alphas8, all_0xff)) & 0xff).cast_unsigned();
    alpha_and != 0xff || _mm_movemask_epi8(_mm_cmpeq_epi8(all_alphas16, all_0xff)) != 0xffff
}

/// Entry point of [`dispatch_alpha_sse2`] for `webp_dispatch_alpha`.
pub(crate) fn dispatch_alpha(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_stride: i32,
) -> bool {
    // SAFETY: `webp_dispatch_alpha` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has the function's only feature.
    unsafe { dispatch_alpha_sse2(alpha, alpha_stride, width, height, dst, dst_stride) }
}

// upstream: src/dsp/alpha_processing_sse2.c DispatchAlphaToGreen_SSE2
#[target_feature(enable = "sse2")]
fn dispatch_alpha_to_green_sse2(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u32],
    dst_stride: i32,
) {
    let zero = _mm_setzero_si128();
    let limit = width & !15;
    let alpha_stride = stride(alpha_stride);
    let dst_stride = stride(dst_stride);
    for j in 0..idx(height.max(0)) {
        let alpha = &alpha[j * alpha_stride..];
        let dst = &mut dst[j * dst_stride..];
        let mut i = 0;
        while i < limit {
            // process 16 alpha bytes
            let k = idx(i);
            let a0 = loadu_si128(&alpha[k..k + 16]);
            let a1 = _mm_unpacklo_epi8(zero, a0); // note the 'zero' first!
            let b1 = _mm_unpackhi_epi8(zero, a0);
            let a2_lo = _mm_unpacklo_epi16(a1, zero);
            let b2_lo = _mm_unpacklo_epi16(b1, zero);
            let a2_hi = _mm_unpackhi_epi16(a1, zero);
            let b2_hi = _mm_unpackhi_epi16(b1, zero);
            storeu_si128(&mut dst[k..k + 4], a2_lo);
            storeu_si128(&mut dst[k + 4..k + 8], a2_hi);
            storeu_si128(&mut dst[k + 8..k + 12], b2_lo);
            storeu_si128(&mut dst[k + 12..k + 16], b2_hi);
            i += 16;
        }
        while i < width {
            dst[idx(i)] = u32::from(alpha[idx(i)]) << 8;
            i += 1;
        }
    }
}

/// Entry point of [`dispatch_alpha_to_green_sse2`] for
/// `webp_dispatch_alpha_to_green`.
pub(crate) fn dispatch_alpha_to_green(
    alpha: &[u8],
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u32],
    dst_stride: i32,
) {
    // SAFETY: `webp_dispatch_alpha_to_green` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dispatch_alpha_to_green_sse2(alpha, alpha_stride, width, height, dst, dst_stride) }
}

// upstream: src/dsp/alpha_processing_sse2.c ExtractAlpha_SSE2
#[target_feature(enable = "sse2")]
fn extract_alpha_sse2(
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
    let a_mask = _mm_set1_epi32(0xff); // to preserve alpha
    let all_0xff = _mm_set_epi32(0, 0, !0, !0);
    let mut all_alphas = all_0xff;
    let argb_stride = stride(argb_stride);
    let alpha_stride = stride(alpha_stride);

    // We must be able to access 3 extra bytes after the last written byte
    // 'src[4 * width - 4]', because we don't know if alpha is the first or the
    // last byte of the quadruplet.
    let limit = (width - 1) & !7;

    for j in 0..idx(height.max(0)) {
        let argb = &argb[j * argb_stride..];
        let alpha = &mut alpha[j * alpha_stride..];
        let mut src = 0;
        let mut i = 0;
        while i < limit {
            // load 32 argb bytes
            let a0 = loadu_si128(&argb[src..src + 16]);
            let a1 = loadu_si128(&argb[src + 16..src + 32]);
            let b0 = _mm_and_si128(a0, a_mask);
            let b1 = _mm_and_si128(a1, a_mask);
            let c0 = _mm_packs_epi32(b0, b1);
            let d0 = _mm_packus_epi16(c0, c0);
            // store
            storel_epi64(&mut alpha[idx(i)..idx(i) + 8], d0);
            // accumulate eight alpha 'and' in parallel
            all_alphas = _mm_and_si128(all_alphas, d0);
            src += 32;
            i += 8;
        }
        while i < width {
            let alpha_value = u32::from(argb[4 * idx(i)]);
            alpha[idx(i)] = low_byte(alpha_value);
            alpha_and &= alpha_value;
            i += 1;
        }
    }
    // Combine the eight alpha 'and' into a 8-bit mask.
    alpha_and &= _mm_movemask_epi8(_mm_cmpeq_epi8(all_alphas, all_0xff)).cast_unsigned();
    alpha_and == 0xff
}

/// Entry point of [`extract_alpha_sse2`] for `webp_extract_alpha`.
pub(crate) fn extract_alpha(
    argb: &[u8],
    argb_stride: i32,
    width: i32,
    height: i32,
    alpha: &mut [u8],
    alpha_stride: i32,
) -> bool {
    // SAFETY: `webp_extract_alpha` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { extract_alpha_sse2(argb, argb_stride, width, height, alpha, alpha_stride) }
}

// upstream: src/dsp/alpha_processing_sse2.c ExtractGreen_SSE2
#[target_feature(enable = "sse2")]
fn extract_green_sse2(argb: &[u32], alpha: &mut [u8], size: i32) {
    let mask = _mm_set1_epi32(0xff);
    let mut src = 0;

    let mut i = 0;
    while i + 16 <= size {
        let a0 = loadu_si128(&argb[src..src + 4]);
        let a1 = loadu_si128(&argb[src + 4..src + 8]);
        let a2 = loadu_si128(&argb[src + 8..src + 12]);
        let a3 = loadu_si128(&argb[src + 12..src + 16]);
        let b0 = _mm_srli_epi32::<8>(a0);
        let b1 = _mm_srli_epi32::<8>(a1);
        let b2 = _mm_srli_epi32::<8>(a2);
        let b3 = _mm_srli_epi32::<8>(a3);
        let c0 = _mm_and_si128(b0, mask);
        let c1 = _mm_and_si128(b1, mask);
        let c2 = _mm_and_si128(b2, mask);
        let c3 = _mm_and_si128(b3, mask);
        let d0 = _mm_packs_epi32(c0, c1);
        let d1 = _mm_packs_epi32(c2, c3);
        let e = _mm_packus_epi16(d0, d1);
        // store
        storeu_si128(&mut alpha[idx(i)..idx(i) + 16], e);
        i += 16;
        src += 16;
    }
    if i + 8 <= size {
        let a0 = loadu_si128(&argb[src..src + 4]);
        let a1 = loadu_si128(&argb[src + 4..src + 8]);
        let b0 = _mm_srli_epi32::<8>(a0);
        let b1 = _mm_srli_epi32::<8>(a1);
        let c0 = _mm_and_si128(b0, mask);
        let c1 = _mm_and_si128(b1, mask);
        let d = _mm_packs_epi32(c0, c1);
        let e = _mm_packus_epi16(d, d);
        storel_epi64(&mut alpha[idx(i)..idx(i) + 8], e);
        i += 8;
    }
    while i < size {
        alpha[idx(i)] = low_byte(argb[idx(i)] >> 8);
        i += 1;
    }
}

/// Entry point of [`extract_green_sse2`] for `webp_extract_green`.
pub(crate) fn extract_green(argb: &[u32], alpha: &mut [u8], size: i32) {
    // SAFETY: `webp_extract_green` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { extract_green_sse2(argb, alpha, size) }
}

//------------------------------------------------------------------------------
// Non-dither premultiplied modes

// upstream: src/dsp/alpha_processing_sse2.c MULTIPLIER
fn multiplier(a: u32) -> u32 {
    a * 0x8081
}

// upstream: src/dsp/alpha_processing_sse2.c PREMULTIPLY
fn premultiply(x: u8, m: u32) -> u8 {
    low_byte((u32::from(x) * m) >> 23)
}

// upstream: src/dsp/alpha_processing_sse2.c APPLY_ALPHA
/// Premultiplies the four pixels at the start of `rgbx`.
/// We can't use a 'const int' for the SHUFFLE value, because it has to be an
/// immediate in the `_mm_shufflexx_epi16()` instruction. We really need a macro.
/// We use: `v / 255 = (v * 0x8081) >> 23`, where `v = alpha * {r,g,b}` is a
/// 16bit value.
#[target_feature(enable = "sse2")]
#[inline]
fn apply_alpha<const SHUFFLE: i32>(
    rgbx: &mut [u8],
    zero: __m128i,
    k_mask: __m128i,
    k_mult: __m128i,
) {
    let argb0 = loadu_si128(rgbx);
    let argb1_lo = _mm_unpacklo_epi8(argb0, zero);
    let argb1_hi = _mm_unpackhi_epi8(argb0, zero);
    let alpha0_lo = _mm_or_si128(argb1_lo, k_mask);
    let alpha0_hi = _mm_or_si128(argb1_hi, k_mask);
    let alpha1_lo = _mm_shufflelo_epi16::<SHUFFLE>(alpha0_lo);
    let alpha1_hi = _mm_shufflelo_epi16::<SHUFFLE>(alpha0_hi);
    let alpha2_lo = _mm_shufflehi_epi16::<SHUFFLE>(alpha1_lo);
    let alpha2_hi = _mm_shufflehi_epi16::<SHUFFLE>(alpha1_hi);
    // alpha2 = [ff a0 a0 a0][ff a1 a1 a1]
    let a0_lo = _mm_mullo_epi16(alpha2_lo, argb1_lo);
    let a0_hi = _mm_mullo_epi16(alpha2_hi, argb1_hi);
    let a1_lo = _mm_mulhi_epu16(a0_lo, k_mult);
    let a1_hi = _mm_mulhi_epu16(a0_hi, k_mult);
    let a2_lo = _mm_srli_epi16::<7>(a1_lo);
    let a2_hi = _mm_srli_epi16::<7>(a1_hi);
    let a3 = _mm_packus_epi16(a2_lo, a2_hi);
    storeu_si128(rgbx, a3);
}

// upstream: src/dsp/alpha_processing_sse2.c ApplyAlphaMultiply_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::many_single_char_names, reason = "the C names w, h, i and a")]
fn apply_alpha_multiply_sse2(rgba: &mut [u8], alpha_first: bool, w: i32, h: i32, stride_: i32) {
    let zero = _mm_setzero_si128();
    let k_mult = _mm_set1_epi16(0x8081_u16.cast_signed());
    let k_mask = _mm_set_epi16(0, 0xff, 0xff, 0, 0, 0xff, 0xff, 0);
    let k_span = 4;
    let row_stride = stride(stride_);
    let mut h = h;
    let mut row = 0;
    while h > 0 {
        h -= 1;
        let rgba = &mut rgba[row..];
        let mut i = 0;
        if alpha_first {
            while i + k_span <= w {
                apply_alpha::<{ mm_shuffle(0, 0, 0, 1) }>(
                    &mut rgba[4 * idx(i)..4 * idx(i) + 16],
                    zero,
                    k_mask,
                    k_mult,
                );
                i += k_span;
            }
        } else {
            while i + k_span <= w {
                apply_alpha::<{ mm_shuffle(2, 3, 3, 3) }>(
                    &mut rgba[4 * idx(i)..4 * idx(i) + 16],
                    zero,
                    k_mask,
                    k_mult,
                );
                i += k_span;
            }
        }
        // Finish with left-overs.
        while i < w {
            let rgb = usize::from(alpha_first);
            let alpha = if alpha_first { 0 } else { 3 };
            let a = u32::from(rgba[alpha + 4 * idx(i)]);
            if a != 0xff {
                let mult = multiplier(a);
                let p = rgb + 4 * idx(i);
                rgba[p] = premultiply(rgba[p], mult);
                rgba[p + 1] = premultiply(rgba[p + 1], mult);
                rgba[p + 2] = premultiply(rgba[p + 2], mult);
            }
            i += 1;
        }
        // C: rgba += stride; the position past the last row is never used.
        if h > 0 {
            row += row_stride;
        }
    }
}

/// Entry point of [`apply_alpha_multiply_sse2`] for `webp_apply_alpha_multiply`.
pub(crate) fn apply_alpha_multiply(
    rgba: &mut [u8],
    alpha_first: bool,
    w: i32,
    h: i32,
    stride: i32,
) {
    // SAFETY: `webp_apply_alpha_multiply` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { apply_alpha_multiply_sse2(rgba, alpha_first, w, h, stride) }
}

//------------------------------------------------------------------------------
// Alpha detection

// upstream: src/dsp/alpha_processing_sse2.c HasAlpha8b_SSE2
#[target_feature(enable = "sse2")]
fn has_alpha8b_sse2(src: &[u8], length: i32) -> bool {
    let all_0xff = _mm_set1_epi8(0xff_u8.cast_signed());
    let mut i = 0;
    while i + 16 <= length {
        let v = loadu_si128(&src[idx(i)..idx(i) + 16]);
        let bits = _mm_cmpeq_epi8(v, all_0xff);
        let mask = _mm_movemask_epi8(bits);
        if mask != 0xffff {
            return true;
        }
        i += 16;
    }
    while i < length {
        if src[idx(i)] != 0xff {
            return true;
        }
        i += 1;
    }
    false
}

/// Entry point of [`has_alpha8b_sse2`] for `webp_has_alpha8b`.
pub(crate) fn has_alpha8b(src: &[u8], length: i32) -> bool {
    // SAFETY: `webp_has_alpha8b` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { has_alpha8b_sse2(src, length) }
}

// upstream: src/dsp/alpha_processing_sse2.c HasAlpha32b_SSE2
#[target_feature(enable = "sse2")]
fn has_alpha32b_sse2(src: &[u8], length: i32) -> bool {
    let alpha_mask = _mm_set1_epi32(0xff);
    let all_0xff = _mm_set1_epi8(0xff_u8.cast_signed());
    let mut i = 0;
    // We don't know if we can access the last 3 bytes after the last alpha
    // value 'src[4 * length - 4]' (because we don't know if alpha is the first
    // or the last byte of the quadruplet). Hence the '-3' protection below.
    let length = length * 4 - 3; // size in bytes
    while i + 64 <= length {
        let k = idx(i);
        let a0 = loadu_si128(&src[k..k + 16]);
        let a1 = loadu_si128(&src[k + 16..k + 32]);
        let a2 = loadu_si128(&src[k + 32..k + 48]);
        let a3 = loadu_si128(&src[k + 48..k + 64]);
        let b0 = _mm_and_si128(a0, alpha_mask);
        let b1 = _mm_and_si128(a1, alpha_mask);
        let b2 = _mm_and_si128(a2, alpha_mask);
        let b3 = _mm_and_si128(a3, alpha_mask);
        let c0 = _mm_packs_epi32(b0, b1);
        let c1 = _mm_packs_epi32(b2, b3);
        let d = _mm_packus_epi16(c0, c1);
        let bits = _mm_cmpeq_epi8(d, all_0xff);
        let mask = _mm_movemask_epi8(bits);
        if mask != 0xffff {
            return true;
        }
        i += 64;
    }
    while i + 32 <= length {
        let k = idx(i);
        let a0 = loadu_si128(&src[k..k + 16]);
        let a1 = loadu_si128(&src[k + 16..k + 32]);
        let b0 = _mm_and_si128(a0, alpha_mask);
        let b1 = _mm_and_si128(a1, alpha_mask);
        let c = _mm_packs_epi32(b0, b1);
        let d = _mm_packus_epi16(c, c);
        let bits = _mm_cmpeq_epi8(d, all_0xff);
        let mask = _mm_movemask_epi8(bits);
        if mask != 0xffff {
            return true;
        }
        i += 32;
    }
    while i <= length {
        if src[idx(i)] != 0xff {
            return true;
        }
        i += 4;
    }
    false
}

/// Entry point of [`has_alpha32b_sse2`] for `webp_has_alpha32b`.
pub(crate) fn has_alpha32b(src: &[u8], length: i32) -> bool {
    // SAFETY: `webp_has_alpha32b` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { has_alpha32b_sse2(src, length) }
}

// upstream: src/dsp/alpha_processing_sse2.c AlphaReplace_SSE2
#[target_feature(enable = "sse2")]
fn alpha_replace_sse2(src: &mut [u32], length: i32, color: u32) {
    let m_color = _mm_set1_epi32(color.cast_signed());
    let zero = _mm_setzero_si128();
    let mut i = 0;
    while i + 8 <= length {
        let k = idx(i);
        let a0 = loadu_si128(&src[k..k + 4]);
        let a1 = loadu_si128(&src[k + 4..k + 8]);
        let b0 = _mm_srai_epi32::<24>(a0);
        let b1 = _mm_srai_epi32::<24>(a1);
        let c0 = _mm_cmpeq_epi32(b0, zero);
        let c1 = _mm_cmpeq_epi32(b1, zero);
        let d0 = _mm_and_si128(c0, m_color);
        let d1 = _mm_and_si128(c1, m_color);
        let e0 = _mm_andnot_si128(c0, a0);
        let e1 = _mm_andnot_si128(c1, a1);
        storeu_si128(&mut src[k..k + 4], _mm_or_si128(d0, e0));
        storeu_si128(&mut src[k + 4..k + 8], _mm_or_si128(d1, e1));
        i += 8;
    }
    while i < length {
        if (src[idx(i)] >> 24) == 0 {
            src[idx(i)] = color;
        }
        i += 1;
    }
}

/// Entry point of [`alpha_replace_sse2`] for `webp_alpha_replace`.
pub(crate) fn alpha_replace(src: &mut [u32], length: i32, color: u32) {
    // SAFETY: `webp_alpha_replace` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { alpha_replace_sse2(src, length, color) }
}

// -----------------------------------------------------------------------------
// Apply alpha value to rows

// upstream: src/dsp/alpha_processing_sse2.c MultARGBRow_SSE2
#[target_feature(enable = "sse2")]
fn mult_argb_row_sse2(ptr: &mut [u32], width: i32, inverse: bool) {
    let mut x = 0;
    if !inverse {
        let k_span = 2;
        let zero = _mm_setzero_si128();
        let k128 = _mm_set1_epi16(128);
        let k_mult = _mm_set1_epi16(0x0101);
        let k_mask = _mm_set_epi16(0, 0xff, 0, 0, 0, 0xff, 0, 0);
        while x + k_span <= width {
            // To compute 'result = (int)(a * x / 255. + .5)', we use:
            //   tmp = a * v + 128, result = (tmp * 0x0101u) >> 16
            let a0 = loadl_epi64(&ptr[idx(x)..idx(x) + 2]);
            let a1 = _mm_unpacklo_epi8(a0, zero);
            let a2 = _mm_or_si128(a1, k_mask);
            let a3 = _mm_shufflelo_epi16::<{ mm_shuffle(2, 3, 3, 3) }>(a2);
            let a4 = _mm_shufflehi_epi16::<{ mm_shuffle(2, 3, 3, 3) }>(a3);
            // here, A4 = [ff a0 a0 a0][ff a1 a1 a1]
            let a5 = _mm_mullo_epi16(a4, a1);
            let a6 = _mm_add_epi16(a5, k128);
            let a7 = _mm_mulhi_epu16(a6, k_mult);
            let a10 = _mm_packus_epi16(a7, zero);
            storel_epi64(&mut ptr[idx(x)..idx(x) + 2], a10);
            x += k_span;
        }
    }
    let width = width - x;
    if width > 0 {
        webp_mult_argb_row_c(&mut ptr[idx(x)..], width, inverse);
    }
}

/// Entry point of [`mult_argb_row_sse2`] for `webp_mult_argb_row`.
pub(crate) fn mult_argb_row(ptr: &mut [u32], width: i32, inverse: bool) {
    // SAFETY: `webp_mult_argb_row` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { mult_argb_row_sse2(ptr, width, inverse) }
}

// upstream: src/dsp/alpha_processing_sse2.c MultRow_SSE2
#[target_feature(enable = "sse2")]
fn mult_row_sse2(ptr: &mut [u8], alpha: &[u8], width: i32, inverse: bool) {
    let mut x = 0;
    if !inverse {
        let zero = _mm_setzero_si128();
        let k128 = _mm_set1_epi16(128);
        let k_mult = _mm_set1_epi16(0x0101);
        while x + 8 <= width {
            let k = idx(x);
            let v0 = loadl_epi64(&ptr[k..k + 8]);
            let a0 = loadl_epi64(&alpha[k..k + 8]);
            let v1 = _mm_unpacklo_epi8(v0, zero);
            let a1 = _mm_unpacklo_epi8(a0, zero);
            let v2 = _mm_mullo_epi16(v1, a1);
            let v3 = _mm_add_epi16(v2, k128);
            let v4 = _mm_mulhi_epu16(v3, k_mult);
            let v5 = _mm_packus_epi16(v4, zero);
            storel_epi64(&mut ptr[k..k + 8], v5);
            x += 8;
        }
    }
    let width = width - x;
    if width > 0 {
        webp_mult_row_c(&mut ptr[idx(x)..], &alpha[idx(x)..], width, inverse);
    }
}

/// Entry point of [`mult_row_sse2`] for `webp_mult_row`.
pub(crate) fn mult_row(ptr: &mut [u8], alpha: &[u8], width: i32, inverse: bool) {
    // SAFETY: `webp_mult_row` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { mult_row_sse2(ptr, alpha, width, inverse) }
}

// upstream: src/dsp/alpha_processing_sse2.c WebPInitAlphaProcessingSSE2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
