// libwebp v1.6.0, src/dsp/ssim_sse2.c:
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
// SSE2 version of distortion calculation
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/ssim_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE2 distortion functions that `VP8SSIMDspInitSSE2` assigns, behind
//! the dispatch functions of `src/dsp/ssim.rs` (ADR 0008). `VP8SSIMDspInit`
//! runs `VP8SSIMDspInitSSE2` when the CPU query is not `NULL` and answers
//! `kSSE2`, so `vp8_accumulate_sse` and `vp8ssim_get` select
//! `accumulate_sse` and `ssim_get` when `features().sse2` holds.
//! `WEBP_DISABLE_STATS` and `WEBP_REDUCE_SIZE` are unset in the reference
//! configuration, so both functions are compiled.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers** are slices; the row advances of `ACCUMULATE_ROW` are
//!    indices into them. `ACCUMULATE_ROW`, a macro over the function's locals,
//!    is a closure-free inline function over a state record.
//! 3. **`VP8DistoStats stats`** starts from `Default`, where C leaves `w`
//!    uninitialized; `VP8SSIMFromStats` does not read `w`.
//! 4. **The kernel rows** are seven bytes: `SSIMGet_SSE2` loads eight bytes
//!    per row and multiplies the eighth by the weight 0, so it never changes
//!    the sums; the translation loads the seven bytes into a zeroed
//!    eight-byte row, because the caller's slices may end after the seventh.
//!    The results are the same.
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions'.
//!
//! # Checks not applicable
//!
//! 1. **`VP8SSIMDspInitSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_cvtsi128_si32, _mm_madd_epi16, _mm_mullo_epi16,
    _mm_or_si128, _mm_set1_epi16, _mm_setzero_si128, _mm_srli_si128, _mm_subs_epu8,
    _mm_unpackhi_epi8, _mm_unpacklo_epi8,
};

use crate::dsp::common_sse2::{loadl_epi64, loadu_si128, storeu_si128};
use crate::dsp::dsp::VP8DistoStats;
use crate::dsp::ssim::vp8ssim_from_stats;

// upstream: src/dsp/ssim_sse2.c SubtractAndSquare_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names C0 and C1")]
fn subtract_and_square_sse2(a: __m128i, b: __m128i) -> __m128i {
    // take abs(a-b) in 8b
    let a_b = _mm_subs_epu8(a, b);
    let b_a = _mm_subs_epu8(b, a);
    let abs_a_b = _mm_or_si128(a_b, b_a);
    // zero-extend to 16b
    let zero = _mm_setzero_si128();
    let C0 = _mm_unpacklo_epi8(abs_a_b, zero);
    let C1 = _mm_unpackhi_epi8(abs_a_b, zero);
    // multiply with self
    let sum1 = _mm_madd_epi16(C0, C0);
    let sum2 = _mm_madd_epi16(C1, C1);
    _mm_add_epi32(sum1, sum2)
}

/// The C `int` index `i` as a slice index; it counts up from 0.
#[inline]
#[expect(clippy::cast_sign_loss, reason = "the C loop index counts up from 0")]
fn idx(i: i32) -> usize {
    i as usize
}

// upstream: src/dsp/ssim_sse2.c AccumulateSSE_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn accumulate_sse_sse2(src1: &[u8], src2: &[u8], len: i32) -> u32 {
    let mut i: i32 = 0;
    let mut sse2: u32 = 0;
    if len >= 16 {
        let limit = len - 32;
        let mut tmp = [0_i32; 4];
        let mut sum = _mm_setzero_si128();
        let mut a0 = loadu_si128(&src1[idx(i)..]);
        let mut b0 = loadu_si128(&src2[idx(i)..]);
        i += 16;
        while i <= limit {
            let a1 = loadu_si128(&src1[idx(i)..]);
            let b1 = loadu_si128(&src2[idx(i)..]);
            i += 16;
            let sum1 = subtract_and_square_sse2(a0, b0);
            sum = _mm_add_epi32(sum, sum1);
            a0 = loadu_si128(&src1[idx(i)..]);
            b0 = loadu_si128(&src2[idx(i)..]);
            i += 16;
            let sum2 = subtract_and_square_sse2(a1, b1);
            sum = _mm_add_epi32(sum, sum2);
        }
        let sum1 = subtract_and_square_sse2(a0, b0);
        sum = _mm_add_epi32(sum, sum1);
        storeu_si128(&mut tmp, sum);
        sse2 = sse2.wrapping_add(
            tmp[3]
                .wrapping_add(tmp[2])
                .wrapping_add(tmp[1])
                .wrapping_add(tmp[0])
                .cast_unsigned(),
        );
    }

    while i < len {
        let diff = i32::from(src1[idx(i)]) - i32::from(src2[idx(i)]);
        sse2 = sse2.wrapping_add((diff * diff).cast_unsigned());
        i += 1;
    }
    sse2
}

/// Entry point of [`accumulate_sse_sse2`] for `vp8_accumulate_sse`.
pub(crate) fn accumulate_sse(src1: &[u8], src2: &[u8], len: i32) -> u32 {
    // SAFETY: `vp8_accumulate_sse` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has SSE2.
    unsafe { accumulate_sse_sse2(src1, src2, len) }
}

// upstream: src/dsp/ssim_sse2.c HorizontalAdd16b_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn horizontal_add16b_sse2(m: __m128i) -> u32 {
    let mut tmp = [0_u16; 8];
    let a = _mm_srli_si128::<8>(m);
    let b = _mm_add_epi16(m, a);
    storeu_si128(&mut tmp, b);
    u32::from(tmp[3]) + u32::from(tmp[2]) + u32::from(tmp[1]) + u32::from(tmp[0])
}

// upstream: src/dsp/ssim_sse2.c HorizontalAdd32b_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn horizontal_add32b_sse2(m: __m128i) -> u32 {
    let a = _mm_srli_si128::<8>(m);
    let b = _mm_add_epi32(m, a);
    let c = _mm_add_epi32(b, _mm_srli_si128::<4>(b));
    _mm_cvtsi128_si32(c).cast_unsigned()
}

// upstream: src/dsp/ssim_sse2.c kWeight
static K_WEIGHT: [u16; 8] = [1, 2, 3, 4, 3, 2, 1, 0];

/// The accumulators and row positions of `SSIMGet_SSE2`, which
/// `ACCUMULATE_ROW` updates.
struct Accumulators {
    xm: __m128i,
    ym: __m128i,
    xxm: __m128i,
    xym: __m128i,
    yym: __m128i,
    src1: usize,
    src2: usize,
}

/// The seven bytes of a kernel row in the low lanes, zero in the eighth:
/// `_mm_loadl_epi64` where upstream loads eight bytes and weights the eighth
/// with `kWeight[7] == 0` (deviation 4).
#[target_feature(enable = "sse2")]
#[inline]
fn load_7_bytes(src: &[u8]) -> __m128i {
    let mut row = [0_u8; 8];
    row[..7].copy_from_slice(&src[..7]);
    loadl_epi64(&row)
}

// upstream: src/dsp/ssim_sse2.c ACCUMULATE_ROW
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names Wx, Wy and W")]
#[expect(clippy::too_many_arguments, reason = "the macro's free variables")]
fn accumulate_row(
    acc: &mut Accumulators,
    weight: i16,
    Wx: __m128i,
    zero: __m128i,
    src1: &[u8],
    stride1: usize,
    src2: &[u8],
    stride2: usize,
) {
    // compute row weight (Wx * Wy)
    let Wy = _mm_set1_epi16(weight);
    let W = _mm_mullo_epi16(Wx, Wy);
    // process 8 bytes at a time (7 bytes, actually)
    let a0 = load_7_bytes(&src1[acc.src1..]);
    let b0 = load_7_bytes(&src2[acc.src2..]);
    // convert to 16b and multiply by weight
    let a1 = _mm_unpacklo_epi8(a0, zero);
    let b1 = _mm_unpacklo_epi8(b0, zero);
    let wa1 = _mm_mullo_epi16(a1, W);
    let wb1 = _mm_mullo_epi16(b1, W);
    // accumulate
    acc.xm = _mm_add_epi16(acc.xm, wa1);
    acc.ym = _mm_add_epi16(acc.ym, wb1);
    acc.xxm = _mm_add_epi32(acc.xxm, _mm_madd_epi16(a1, wa1));
    acc.xym = _mm_add_epi32(acc.xym, _mm_madd_epi16(a1, wb1));
    acc.yym = _mm_add_epi32(acc.yym, _mm_madd_epi16(b1, wb1));
    acc.src1 += stride1;
    acc.src2 += stride2;
}

// upstream: src/dsp/ssim_sse2.c SSIMGet_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C name Wx")]
pub(crate) fn ssim_get_sse2(src1: &[u8], stride1: i32, src2: &[u8], stride2: i32) -> f64 {
    let mut stats = VP8DistoStats::default();
    let zero = _mm_setzero_si128();
    let mut acc = Accumulators {
        xm: zero,
        ym: zero,
        xxm: zero,
        xym: zero,
        yym: zero,
        src1: 0,
        src2: 0,
    };
    let Wx = loadu_si128(&K_WEIGHT);
    let stride1 = usize::try_from(stride1).expect("a non-negative stride");
    let stride2 = usize::try_from(stride2).expect("a non-negative stride");
    for weight in [1, 2, 3, 4, 3, 2, 1] {
        accumulate_row(&mut acc, weight, Wx, zero, src1, stride1, src2, stride2);
    }
    stats.xm = horizontal_add16b_sse2(acc.xm);
    stats.ym = horizontal_add16b_sse2(acc.ym);
    stats.xxm = horizontal_add32b_sse2(acc.xxm);
    stats.xym = horizontal_add32b_sse2(acc.xym);
    stats.yym = horizontal_add32b_sse2(acc.yym);
    vp8ssim_from_stats(&stats)
}

/// Entry point of [`ssim_get_sse2`] for `vp8ssim_get`.
pub(crate) fn ssim_get(src1: &[u8], stride1: i32, src2: &[u8], stride2: i32) -> f64 {
    // SAFETY: `vp8ssim_get` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has SSE2.
    unsafe { ssim_get_sse2(src1, stride1, src2, stride2) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/ssim_sse2.c VP8SSIMDspInitSSE2
// The pointer assignments are the dispatch functions of `src/dsp/ssim.rs`
// named in the module documentation (ADR 0008).
