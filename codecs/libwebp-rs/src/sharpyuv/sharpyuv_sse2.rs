// libwebp v1.6.0, sharpyuv/sharpyuv_sse2.c:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Speed-critical functions for Sharp YUV.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `sharpyuv/sharpyuv_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE2 `SharpYUV` kernels that `InitSharpYuvSSE2` assigns, behind the
//! dispatch functions of `src/sharpyuv/sharpyuv_dsp.rs` (ADR 0008).
//! `SharpYuvInitDsp` runs `InitSharpYuvSSE2` when `SharpYuvGetCPUInfo` is not
//! `NULL` and answers `kSSE2`; the encoder installs `VP8GetCPUInfo` there
//! (`SharpYuvInit(VP8GetCPUInfo)` in `picture_csp_enc.c`), so the dispatch
//! functions select these entry points when `features().sse2` holds.
//! `InitSharpYuvSSE2` assigns `SharpYuvUpdateY`, `SharpYuvUpdateRGB` and
//! `SharpYuvFilterRow`.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `src/dsp/common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers** are slices, as in `sharpyuv_dsp.rs`.
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions'.
//!
//! # Checks not applicable
//!
//! 1. **`InitSharpYuvSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_cmpgt_epi16, _mm_madd_epi16, _mm_max_epi16,
    _mm_min_epi16, _mm_or_si128, _mm_packs_epi32, _mm_set1_epi16, _mm_set1_epi32,
    _mm_setzero_si128, _mm_srai_epi16, _mm_srai_epi32, _mm_sub_epi16, _mm_unpackhi_epi16,
    _mm_unpackhi_epi32, _mm_unpacklo_epi16, _mm_unpacklo_epi32,
};

use crate::dsp::common_sse2::{loadl_epi64, loadu_si128, storeu_si128};

/// The C `int` index `i` as a slice index; it counts up from 0.
#[inline]
#[expect(clippy::cast_sign_loss, reason = "the C loop index counts up from 0")]
fn idx(i: i32) -> usize {
    i as usize
}

// upstream: sharpyuv/sharpyuv_sse2.c clip_SSE2
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C returns (uint16_t)v for v in [0, max]"
)]
fn clip_sse2(v: i32, max: i32) -> u16 {
    if v < 0 {
        0
    } else if v > max {
        max as u16
    } else {
        v as u16
    }
}

// upstream: sharpyuv/sharpyuv_sse2.c SharpYuvUpdateY_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names A to I")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C passes max_y (at most 65535) to _mm_set1_epi16 as a short"
)]
fn sharp_yuv_update_y_sse2(
    r#ref: &[u16],
    src: &[u16],
    dst: &mut [u16],
    len: i32,
    bit_depth: i32,
) -> u64 {
    let max_y = (1 << bit_depth) - 1;
    let mut diff: u64;
    let mut tmp = [0_u32; 4];
    let zero = _mm_setzero_si128();
    let max = _mm_set1_epi16(max_y as i16);
    let one = _mm_set1_epi16(1);
    let mut sum = zero;

    let mut i: i32 = 0;
    while i + 8 <= len {
        let A = loadu_si128(&r#ref[idx(i)..]);
        let B = loadu_si128(&src[idx(i)..]);
        let C = loadu_si128(&dst[idx(i)..]);
        let D = _mm_sub_epi16(A, B); // diff_y
        let E = _mm_cmpgt_epi16(zero, D); // sign (-1 or 0)
        let F = _mm_add_epi16(C, D); // new_y
        let G = _mm_or_si128(E, one); // -1 or 1
        let H = _mm_max_epi16(_mm_min_epi16(F, max), zero);
        let I = _mm_madd_epi16(D, G); // sum(abs(...))
        storeu_si128(&mut dst[idx(i)..], H);
        sum = _mm_add_epi32(sum, I);
        i += 8;
    }
    storeu_si128(&mut tmp, sum);
    diff = u64::from(
        tmp[3]
            .wrapping_add(tmp[2])
            .wrapping_add(tmp[1])
            .wrapping_add(tmp[0]),
    );
    while i < len {
        let k = idx(i);
        let diff_y = i32::from(r#ref[k]) - i32::from(src[k]);
        let new_y = i32::from(dst[k]) + diff_y;
        dst[k] = clip_sse2(new_y, max_y);
        diff += u64::from(diff_y.unsigned_abs());
        i += 1;
    }
    diff
}

/// Entry point of `SharpYuvUpdateY_SSE2` for `SharpYuvUpdateY`.
pub(crate) fn sharp_yuv_update_y(
    r#ref: &[u16],
    src: &[u16],
    dst: &mut [u16],
    len: i32,
    bit_depth: i32,
) -> u64 {
    // SAFETY: the dispatch function calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has SSE2.
    unsafe { sharp_yuv_update_y_sse2(r#ref, src, dst, len, bit_depth) }
}

// upstream: sharpyuv/sharpyuv_sse2.c SharpYuvUpdateRGB_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names A to E")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C adds the int difference to the int16_t element"
)]
fn sharp_yuv_update_rgb_sse2(r#ref: &[i16], src: &[i16], dst: &mut [i16], len: i32) {
    let mut i: i32 = 0;
    while i + 8 <= len {
        let A = loadu_si128(&r#ref[idx(i)..]);
        let B = loadu_si128(&src[idx(i)..]);
        let C = loadu_si128(&dst[idx(i)..]);
        let D = _mm_sub_epi16(A, B); // diff_uv
        let E = _mm_add_epi16(C, D); // new_uv
        storeu_si128(&mut dst[idx(i)..], E);
        i += 8;
    }
    while i < len {
        let k = idx(i);
        let diff_uv = i32::from(r#ref[k]) - i32::from(src[k]);
        dst[k] = (i32::from(dst[k]) + diff_uv) as i16;
        i += 1;
    }
}

/// Entry point of `SharpYuvUpdateRGB_SSE2` for `SharpYuvUpdateRGB`.
pub(crate) fn sharp_yuv_update_rgb(r#ref: &[i16], src: &[i16], dst: &mut [i16], len: i32) {
    // SAFETY: the dispatch function calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { sharp_yuv_update_rgb_sse2(r#ref, src, dst, len) }
}

/// The scalar tail of `SharpYuvFilterRow16_SSE2` and
/// `SharpYuvFilterRow32_SSE2` for the pair `i`, which upstream repeats in
/// both functions.
#[expect(non_snake_case, reason = "the C parameter names A and B")]
fn filter_row_tail(A: &[i16], B: &[i16], i: usize, best_y: &[u16], out: &mut [u16], max_y: i32) {
    //   (9 * A0 + 3 * A1 + 3 * B0 + B1 + 8) >> 4 =
    // = (8 * A0 + 2 * (A1 + B0) + (A0 + A1 + B0 + B1 + 8)) >> 4
    // We reuse the common sub-expressions.
    let a0b1 = i32::from(A[i]) + i32::from(B[i + 1]);
    let a1b0 = i32::from(A[i + 1]) + i32::from(B[i]);
    let a0a1b0b1 = a0b1 + a1b0 + 8;
    let v0 = (8 * i32::from(A[i]) + 2 * a1b0 + a0a1b0b1) >> 4;
    let v1 = (8 * i32::from(A[i + 1]) + 2 * a0b1 + a0a1b0b1) >> 4;
    out[2 * i] = clip_sse2(i32::from(best_y[2 * i]) + v0, max_y);
    out[2 * i + 1] = clip_sse2(i32::from(best_y[2 * i + 1]) + v1, max_y);
}

// upstream: sharpyuv/sharpyuv_sse2.c SharpYuvFilterRow16_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C parameter names A and B")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C passes max_y to _mm_set1_epi16 as a short"
)]
fn sharp_yuv_filter_row16_sse2(
    A: &[i16],
    B: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    let max_y = (1 << bit_depth) - 1;
    let kCst8 = _mm_set1_epi16(8);
    let max = _mm_set1_epi16(max_y as i16);
    let zero = _mm_setzero_si128();
    let mut i: i32 = 0;
    while i + 8 <= len {
        let k = idx(i);
        let a0 = loadu_si128(&A[k..]);
        let a1 = loadu_si128(&A[k + 1..]);
        let b0 = loadu_si128(&B[k..]);
        let b1 = loadu_si128(&B[k + 1..]);
        let a0b1 = _mm_add_epi16(a0, b1);
        let a1b0 = _mm_add_epi16(a1, b0);
        let a0a1b0b1 = _mm_add_epi16(a0b1, a1b0); // A0+A1+B0+B1
        let a0a1b0b1_8 = _mm_add_epi16(a0a1b0b1, kCst8);
        let a0b1_2 = _mm_add_epi16(a0b1, a0b1); // 2*(A0+B1)
        let a1b0_2 = _mm_add_epi16(a1b0, a1b0); // 2*(A1+B0)
        let c0 = _mm_srai_epi16::<3>(_mm_add_epi16(a0b1_2, a0a1b0b1_8));
        let c1 = _mm_srai_epi16::<3>(_mm_add_epi16(a1b0_2, a0a1b0b1_8));
        let d0 = _mm_add_epi16(c1, a0);
        let d1 = _mm_add_epi16(c0, a1);
        let e0 = _mm_srai_epi16::<1>(d0);
        let e1 = _mm_srai_epi16::<1>(d1);
        let f0 = _mm_unpacklo_epi16(e0, e1);
        let f1 = _mm_unpackhi_epi16(e0, e1);
        let g0 = loadu_si128(&best_y[2 * k..]);
        let g1 = loadu_si128(&best_y[2 * k + 8..]);
        let h0 = _mm_add_epi16(g0, f0);
        let h1 = _mm_add_epi16(g1, f1);
        let i0 = _mm_max_epi16(_mm_min_epi16(h0, max), zero);
        let i1 = _mm_max_epi16(_mm_min_epi16(h1, max), zero);
        storeu_si128(&mut out[2 * k..], i0);
        storeu_si128(&mut out[2 * k + 8..], i1);
        i += 8;
    }
    while i < len {
        filter_row_tail(A, B, idx(i), best_y, out, max_y);
        i += 1;
    }
}

// upstream: sharpyuv/sharpyuv_sse2.c s16_to_s32
#[target_feature(enable = "sse2")]
#[inline]
fn s16_to_s32(input: __m128i) -> __m128i {
    _mm_srai_epi32::<16>(_mm_unpacklo_epi16(input, input))
}

// upstream: sharpyuv/sharpyuv_sse2.c SharpYuvFilterRow32_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C parameter names A and B")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C passes max_y to _mm_set1_epi16 as a short"
)]
fn sharp_yuv_filter_row32_sse2(
    A: &[i16],
    B: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    let max_y = (1 << bit_depth) - 1;
    let kCst8 = _mm_set1_epi32(8);
    let max = _mm_set1_epi16(max_y as i16);
    let zero = _mm_setzero_si128();
    let mut i: i32 = 0;
    while i + 4 <= len {
        let k = idx(i);
        let a0 = s16_to_s32(loadl_epi64(&A[k..]));
        let a1 = s16_to_s32(loadl_epi64(&A[k + 1..]));
        let b0 = s16_to_s32(loadl_epi64(&B[k..]));
        let b1 = s16_to_s32(loadl_epi64(&B[k + 1..]));
        let a0b1 = _mm_add_epi32(a0, b1);
        let a1b0 = _mm_add_epi32(a1, b0);
        let a0a1b0b1 = _mm_add_epi32(a0b1, a1b0); // A0+A1+B0+B1
        let a0a1b0b1_8 = _mm_add_epi32(a0a1b0b1, kCst8);
        let a0b1_2 = _mm_add_epi32(a0b1, a0b1); // 2*(A0+B1)
        let a1b0_2 = _mm_add_epi32(a1b0, a1b0); // 2*(A1+B0)
        let c0 = _mm_srai_epi32::<3>(_mm_add_epi32(a0b1_2, a0a1b0b1_8));
        let c1 = _mm_srai_epi32::<3>(_mm_add_epi32(a1b0_2, a0a1b0b1_8));
        let d0 = _mm_add_epi32(c1, a0);
        let d1 = _mm_add_epi32(c0, a1);
        let e0 = _mm_srai_epi32::<1>(d0);
        let e1 = _mm_srai_epi32::<1>(d1);
        let f0 = _mm_unpacklo_epi32(e0, e1);
        let f1 = _mm_unpackhi_epi32(e0, e1);
        let g = loadu_si128(&best_y[2 * k..]);
        let h_16 = _mm_add_epi16(g, _mm_packs_epi32(f0, f1));
        let final_ = _mm_max_epi16(_mm_min_epi16(h_16, max), zero);
        storeu_si128(&mut out[2 * k..], final_);
        i += 4;
    }
    while i < len {
        filter_row_tail(A, B, idx(i), best_y, out, max_y);
        i += 1;
    }
}

// upstream: sharpyuv/sharpyuv_sse2.c SharpYuvFilterRow_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C parameter names A and B")]
fn sharp_yuv_filter_row_sse2(
    A: &[i16],
    B: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    if bit_depth <= 10 {
        sharp_yuv_filter_row16_sse2(A, B, len, best_y, out, bit_depth);
    } else {
        sharp_yuv_filter_row32_sse2(A, B, len, best_y, out, bit_depth);
    }
}

/// Entry point of `SharpYuvFilterRow_SSE2` for `SharpYuvFilterRow`.
pub(crate) fn sharp_yuv_filter_row(
    a: &[i16],
    b: &[i16],
    len: i32,
    best_y: &[u16],
    out: &mut [u16],
    bit_depth: i32,
) {
    // SAFETY: the dispatch function calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { sharp_yuv_filter_row_sse2(a, b, len, best_y, out, bit_depth) }
}

//------------------------------------------------------------------------------

// upstream: sharpyuv/sharpyuv_sse2.c InitSharpYuvSSE2
// The pointer assignments are the dispatch functions of
// `src/sharpyuv/sharpyuv_dsp.rs` named in the module documentation (ADR 0008).
