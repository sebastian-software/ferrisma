// libwebp v1.6.0, src/dsp/lossless_avx2.c:
//
// Copyright 2025 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// AVX2 variant of methods for lossless decoder
//
// Author: Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/dsp/lossless_avx2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The AVX2 batch predictors, inverse transforms and BGRA-to-RGBA converter
//! that `VP8LDspInitAVX2` assigns, behind the dispatch functions of
//! `src/dsp/lossless.rs` (ADR 0008). `VP8LDspInit` runs `VP8LDspInitAVX2`
//! only inside its SSE4.1 branch, itself inside the SSE2 branch, so a
//! dispatch function selects these entry points when `features().sse2`,
//! `features().sse4_1` and `features().avx2` all hold, before the SSE4.1 and
//! SSE2 variants. `VP8LDspInitAVX2` assigns:
//!
//! - `VP8LPredictorsAdd[0, 1, 2, 3, 4, 8, 9, 10, 11, 12]`: `predictor_add0`
//!   to `predictor_add4`, `predictor_add8` to `predictor_add12`, behind
//!   `vp8l_predictors_add`;
//! - `VP8LAddGreenToBlueAndRed`: `add_green_to_blue_and_red`;
//! - `VP8LTransformColorInverse`: `transform_color_inverse`;
//! - `VP8LConvertBGRAToRGBA`: `convert_bgra_to_rgba`.
//!
//! The module also holds the checked 256-bit loads and stores of ADR 0008,
//! operation 1: `loadu_si256` and `storeu_si256` assert that the slice
//! spans the 32 bytes the intrinsic reads or writes and then perform the one
//! unsafe operation, as `common_sse2.rs` does for 128 bits. They are
//! `pub(crate)` for WP19's `lossless_enc_avx2.rs`.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **The "SSE exports".** Upstream finishes the left-over pixels through
//!    the pointers `VP8LPredictorsAdd_SSE`, `VP8LAddGreenToBlueAndRed_SSE`,
//!    `VP8LTransformColorInverse_SSE` and `VP8LConvertBGRAToRGBA_SSE`, which
//!    `VP8LDspInitSSE2` and `VP8LDspInitSSE41` set before `VP8LDspInitAVX2`
//!    runs: at that point they hold `PredictorAdd0_SSE2` to
//!    `PredictorAdd13_SSE2`, `AddGreenToBlueAndRed_SSE2`,
//!    `TransformColorInverse_SSE41` and `ConvertBGRAToRGBA_SSE2`, since the
//!    AVX2 functions are assigned only under SSE2 and SSE4.1. Rust has no
//!    such pointers (ADR 0003): the AVX2 functions call those SSE2 and SSE4.1
//!    functions directly, which their AVX2 feature implies.
//! 3. **Local macros.** `DO_PRED10`, `DO_PRED11`, `DO_PRED12` and their
//!    `_SHIFT` companions are functions that take the caller's registers by
//!    reference and return the output pixel, which the caller stores at the
//!    same index and in the same order; `CST` is a function over a
//!    multiplier; `GENERATE_PREDICTOR_1` and `GENERATE_PREDICTOR_2` are local
//!    `macro_rules!` that take the offset of their `IN` operand from
//!    `upper[i]`.
//! 4. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3).
//!
//! # Excluded undefined behavior
//!
//! None: the AVX2 functions compute the C functions' results for every input
//! (upstream's `-noasm` comparison relies on it, and the parity tests cover
//! the full value ranges).
//!
//! # Checks not applicable
//!
//! 1. **`VP8LDspInitAVX2`** only assigns pointers (ADR 0003); its assignments
//!    are the selection in the dispatch functions listed above.
//! 2. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_lossless.rs`). They reach the
//!    AVX2 level only on a CPU that reports AVX2; under Rosetta 2 that needs
//!    `ROSETTA_ADVERTISE_AVX=1`.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m256i, _mm256_add_epi8, _mm256_add_epi16, _mm256_and_si256, _mm256_andnot_si256,
    _mm256_avg_epu8, _mm256_blendv_epi8, _mm256_cmpgt_epi32, _mm256_cvtsi256_si32,
    _mm256_extract_epi32, _mm256_loadu_si256, _mm256_mulhi_epi16, _mm256_or_si256,
    _mm256_packs_epi32, _mm256_packus_epi16, _mm256_permute2x128_si256,
    _mm256_permutevar8x32_epi32, _mm256_sad_epu8, _mm256_set_epi8, _mm256_set_epi32,
    _mm256_set1_epi8, _mm256_set1_epi32, _mm256_setr_epi8, _mm256_setr_epi32, _mm256_setzero_si256,
    _mm256_shuffle_epi8, _mm256_slli_si256, _mm256_srli_si256, _mm256_storeu_si256,
    _mm256_sub_epi8, _mm256_sub_epi16, _mm256_unpackhi_epi8, _mm256_unpackhi_epi32,
    _mm256_unpacklo_epi8, _mm256_unpacklo_epi32, _mm256_xor_si256,
};

use crate::dsp::common_sse2::Lane;
use crate::dsp::lossless::VP8LMultipliers;
use crate::dsp::lossless_sse2::{
    add_green_to_blue_and_red_sse2, at, convert_bgra_to_rgba_sse2, idx, predictor_add0_sse2,
    predictor_add1_sse2, predictor_add2_sse2, predictor_add3_sse2, predictor_add4_sse2,
    predictor_add8_sse2, predictor_add9_sse2, predictor_add10_sse2, predictor_add11_sse2,
    predictor_add12_sse2,
};
use crate::dsp::lossless_sse41::transform_color_inverse_sse41;
use crate::webp::format_constants::ARGB_BLACK;

//------------------------------------------------------------------------------
// Checked loads and stores (ADR 0008, operation 1). No upstream counterpart.

/// Loads the first 32 bytes of `src` (C: `_mm256_loadu_si256` at the start of
/// `src`).
///
/// # Panics
///
/// Panics when `src` spans fewer than 32 bytes, where C would read past it.
#[target_feature(enable = "avx2")]
#[inline]
pub(crate) fn loadu_si256<T: Lane>(src: &[T]) -> __m256i {
    assert!(
        size_of_val(src) >= 32,
        "a 32-byte load past the end of a slice"
    );
    // SAFETY: the assertion above checked that `src` spans the 32 bytes that
    // the unaligned load reads, and `Lane` types have no uninitialized bytes.
    unsafe { _mm256_loadu_si256(src.as_ptr().cast()) }
}

/// Stores `value` into the first 32 bytes of `dst` (C: `_mm256_storeu_si256`
/// at the start of `dst`).
///
/// # Panics
///
/// Panics when `dst` spans fewer than 32 bytes, where C would write past it.
#[target_feature(enable = "avx2")]
#[inline]
pub(crate) fn storeu_si256<T: Lane>(dst: &mut [T], value: __m256i) {
    assert!(
        size_of_val(dst) >= 32,
        "a 32-byte store past the end of a slice"
    );
    // SAFETY: the assertion above checked that `dst` spans the 32 bytes that
    // the unaligned store writes, and every byte pattern is a valid `Lane`.
    unsafe { _mm256_storeu_si256(dst.as_mut_ptr().cast(), value) }
}

//------------------------------------------------------------------------------
// Predictor Transform

// upstream: src/dsp/lossless_avx2.c Average2_m256i
#[target_feature(enable = "avx2")]
#[inline]
fn average2_m256i(a0: &__m256i, a1: &__m256i, avg: &mut __m256i) {
    // (a + b) >> 1 = ((a + b + 1) >> 1) - ((a ^ b) & 1)
    let ones = _mm256_set1_epi8(1);
    let avg1 = _mm256_avg_epu8(*a0, *a1);
    let one = _mm256_and_si256(_mm256_xor_si256(*a0, *a1), ones);
    *avg = _mm256_sub_epi8(avg1, one);
}

// Batch versions of those functions.

// upstream: src/dsp/lossless_avx2.c PredictorAdd0_AVX2
/// Predictor0: `ARGB_BLACK`.
#[target_feature(enable = "avx2")]
fn predictor_add0_avx2(
    data: &mut [u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let black = _mm256_set1_epi32(ARGB_BLACK.cast_signed());
    while i + 8 <= num_pixels {
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let res = _mm256_add_epi8(src, black);
        storeu_si256(&mut data[out_pos + idx(i)..], res);
        i += 8;
    }
    if i != num_pixels {
        // C: VP8LPredictorsAdd_SSE[0] (deviation 2), upper is NULL.
        predictor_add0_sse2(data, in_pos + idx(i), 0, num_pixels - i, out_pos + idx(i));
    }
}

// upstream: src/dsp/lossless_avx2.c PredictorAdd1_AVX2
/// Predictor1: left.
#[target_feature(enable = "avx2")]
fn predictor_add1_avx2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let mut prev = _mm256_set1_epi32(data[out_pos - 1].cast_signed());
    while i + 8 <= num_pixels {
        // h | g | f | e | d | c | b | a
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        // g | f | e | 0 | c | b | a | 0
        let shift0 = _mm256_slli_si256::<4>(src);
        // g + h | f + g | e + f | e | c + d | b + c | a + b | a
        let sum0 = _mm256_add_epi8(src, shift0);
        // e + f | e | 0 | 0 | a + b | a | 0 | 0
        let shift1 = _mm256_slli_si256::<8>(sum0);
        // e + f + g + h | e + f + g | e + f | e | a + b + c + d | a + b + c | a + b
        // | a
        let sum1 = _mm256_add_epi8(sum0, shift1);
        // Add a + b + c + d to the upper lane.
        let sum_abcd = _mm256_extract_epi32::<3>(sum1);
        let sum2 = _mm256_add_epi8(
            sum1,
            _mm256_set_epi32(sum_abcd, sum_abcd, sum_abcd, sum_abcd, 0, 0, 0, 0),
        );

        let res = _mm256_add_epi8(sum2, prev);
        storeu_si256(&mut data[out_pos + idx(i)..], res);
        // replicate last res output in prev.
        prev = _mm256_permutevar8x32_epi32(res, _mm256_set_epi32(7, 7, 7, 7, 7, 7, 7, 7));
        i += 8;
    }
    if i != num_pixels {
        // C: VP8LPredictorsAdd_SSE[1] (deviation 2).
        predictor_add1_sse2(
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_avx2.c GENERATE_PREDICTOR_1
/// Macro that adds 32-bit integers from IN using mod 256 arithmetic
/// per 8 bit channel. `$delta` is the offset of IN from `upper[i]`;
/// `$left_over` is `VP8LPredictorsAdd_SSE[X]` (deviation 2).
macro_rules! generate_predictor_1 {
    ($name:ident, $delta:literal, $left_over:ident) => {
        #[target_feature(enable = "avx2")]
        fn $name(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            let mut i: i32 = 0;
            while i + 8 <= num_pixels {
                let src = loadu_si256(&data[in_pos + idx(i)..]);
                let other = loadu_si256(&data[at(upper_pos + idx(i), $delta)..]);
                let res = _mm256_add_epi8(src, other);
                storeu_si256(&mut data[out_pos + idx(i)..], res);
                i += 8;
            }
            if i != num_pixels {
                $left_over(
                    data,
                    in_pos + idx(i),
                    upper_pos + idx(i),
                    num_pixels - i,
                    out_pos + idx(i),
                );
            }
        }
    };
}

// Predictor2: Top.
// upstream: src/dsp/lossless_avx2.c PredictorAdd2_AVX2
generate_predictor_1!(predictor_add2_avx2, 0, predictor_add2_sse2);
// Predictor3: Top-right.
// upstream: src/dsp/lossless_avx2.c PredictorAdd3_AVX2
generate_predictor_1!(predictor_add3_avx2, 1, predictor_add3_sse2);
// Predictor4: Top-left.
// upstream: src/dsp/lossless_avx2.c PredictorAdd4_AVX2
generate_predictor_1!(predictor_add4_avx2, -1, predictor_add4_sse2);

// Due to averages with integers, values cannot be accumulated in parallel for
// predictors 5 to 7.

// upstream: src/dsp/lossless_avx2.c GENERATE_PREDICTOR_2
/// `$delta` is the offset of IN from `upper[i]`; `$left_over` is
/// `VP8LPredictorsAdd_SSE[X]` (deviation 2).
macro_rules! generate_predictor_2 {
    ($name:ident, $delta:literal, $left_over:ident) => {
        #[target_feature(enable = "avx2")]
        fn $name(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            let mut i: i32 = 0;
            while i + 8 <= num_pixels {
                let t_other = loadu_si256(&data[at(upper_pos + idx(i), $delta)..]);
                let t = loadu_si256(&data[upper_pos + idx(i)..]);
                let src = loadu_si256(&data[in_pos + idx(i)..]);
                let mut avg = _mm256_setzero_si256();
                average2_m256i(&t, &t_other, &mut avg);
                let res = _mm256_add_epi8(avg, src);
                storeu_si256(&mut data[out_pos + idx(i)..], res);
                i += 8;
            }
            if i != num_pixels {
                $left_over(
                    data,
                    in_pos + idx(i),
                    upper_pos + idx(i),
                    num_pixels - i,
                    out_pos + idx(i),
                );
            }
        }
    };
}

// Predictor8: average TL T.
// upstream: src/dsp/lossless_avx2.c PredictorAdd8_AVX2
generate_predictor_2!(predictor_add8_avx2, -1, predictor_add8_sse2);
// Predictor9: average T TR.
// upstream: src/dsp/lossless_avx2.c PredictorAdd9_AVX2
generate_predictor_2!(predictor_add9_avx2, 1, predictor_add9_sse2);

// upstream: src/dsp/lossless_avx2.c DO_PRED10
/// Predictor10: average of (average of (L,TL), average of (T, TR)). Returns
/// the pixel that C stores at `out[i + OUT]`.
#[target_feature(enable = "avx2")]
#[inline]
fn do_pred10(l: &mut __m256i, tl: &__m256i, avg_ttr: &__m256i, src: &__m256i) -> u32 {
    let mut avg_ltl = _mm256_setzero_si256();
    let mut avg = _mm256_setzero_si256();
    average2_m256i(l, tl, &mut avg_ltl);
    average2_m256i(avg_ttr, &avg_ltl, &mut avg);
    *l = _mm256_add_epi8(avg, *src);
    _mm256_cvtsi256_si32(*l).cast_unsigned()
}

// upstream: src/dsp/lossless_avx2.c DO_PRED10_SHIFT
#[target_feature(enable = "avx2")]
#[inline]
fn do_pred10_shift(avg_ttr: &mut __m256i, tl: &mut __m256i, src: &mut __m256i) {
    // Rotate the pre-computed values for the next iteration.
    *avg_ttr = _mm256_srli_si256::<4>(*avg_ttr);
    *tl = _mm256_srli_si256::<4>(*tl);
    *src = _mm256_srli_si256::<4>(*src);
}

// upstream: src/dsp/lossless_avx2.c PredictorAdd10_AVX2
#[target_feature(enable = "avx2")]
fn predictor_add10_avx2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let mut l = _mm256_setr_epi32(data[out_pos - 1].cast_signed(), 0, 0, 0, 0, 0, 0, 0);
    while i + 8 <= num_pixels {
        let mut src = loadu_si256(&data[in_pos + idx(i)..]);
        let mut tl = loadu_si256(&data[upper_pos + idx(i) - 1..]);
        let t = loadu_si256(&data[upper_pos + idx(i)..]);
        let tr = loadu_si256(&data[upper_pos + idx(i) + 1..]);
        let mut avg_ttr = _mm256_setzero_si256();
        average2_m256i(&t, &tr, &mut avg_ttr);
        {
            let avg_ttr_bak = avg_ttr;
            let tl_bak = tl;
            let src_bak = src;
            let out = out_pos + idx(i);
            let mut j = 0;
            while j < 4 {
                data[out + j] = do_pred10(&mut l, &tl, &avg_ttr, &src);
                do_pred10_shift(&mut avg_ttr, &mut tl, &mut src);
                j += 1;
            }
            avg_ttr = _mm256_permute2x128_si256::<1>(avg_ttr_bak, avg_ttr_bak);
            tl = _mm256_permute2x128_si256::<1>(tl_bak, tl_bak);
            src = _mm256_permute2x128_si256::<1>(src_bak, src_bak);
            while j < 8 {
                data[out + j] = do_pred10(&mut l, &tl, &avg_ttr, &src);
                do_pred10_shift(&mut avg_ttr, &mut tl, &mut src);
                j += 1;
            }
        }
        i += 8;
    }
    if i != num_pixels {
        // C: VP8LPredictorsAdd_SSE[10] (deviation 2).
        predictor_add10_sse2(
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_avx2.c DO_PRED11
/// Predictor11: select. Returns the pixel that C stores at `out[i + OUT]`.
#[target_feature(enable = "avx2")]
#[inline]
fn do_pred11(l: &mut __m256i, t: &__m256i, tl: &__m256i, src: &__m256i, pa: &__m256i) -> u32 {
    let l_lo = _mm256_unpacklo_epi32(*l, *t);
    let tl_lo = _mm256_unpacklo_epi32(*tl, *t);
    let pb = _mm256_sad_epu8(l_lo, tl_lo); // pb = sum |L-TL|
    let mask = _mm256_cmpgt_epi32(pb, *pa);
    let a = _mm256_and_si256(mask, *l);
    let b = _mm256_andnot_si256(mask, *t);
    let pred = _mm256_or_si256(a, b); // pred = (pa > b)? L : T
    *l = _mm256_add_epi8(*src, pred);
    _mm256_cvtsi256_si32(*l).cast_unsigned()
}

// upstream: src/dsp/lossless_avx2.c DO_PRED11_SHIFT
#[target_feature(enable = "avx2")]
#[inline]
fn do_pred11_shift(t: &mut __m256i, tl: &mut __m256i, src: &mut __m256i, pa: &mut __m256i) {
    // Shift the pre-computed value for the next iteration.
    *t = _mm256_srli_si256::<4>(*t);
    *tl = _mm256_srli_si256::<4>(*tl);
    *src = _mm256_srli_si256::<4>(*src);
    *pa = _mm256_srli_si256::<4>(*pa);
}

// upstream: src/dsp/lossless_avx2.c PredictorAdd11_AVX2
#[target_feature(enable = "avx2")]
#[expect(clippy::similar_names, reason = "the C names T_lo and TL_lo")]
fn predictor_add11_avx2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let mut l = _mm256_setr_epi32(data[out_pos - 1].cast_signed(), 0, 0, 0, 0, 0, 0, 0);
    while i + 8 <= num_pixels {
        let mut t = loadu_si256(&data[upper_pos + idx(i)..]);
        let mut tl = loadu_si256(&data[upper_pos + idx(i) - 1..]);
        let mut src = loadu_si256(&data[in_pos + idx(i)..]);
        let mut pa = {
            // We can unpack with any value on the upper 32 bits, provided it's the
            // same on both operands (so that their sum of abs diff is zero). Here we
            // use T.
            let t_lo = _mm256_unpacklo_epi32(t, t);
            let tl_lo = _mm256_unpacklo_epi32(tl, t);
            let t_hi = _mm256_unpackhi_epi32(t, t);
            let tl_hi = _mm256_unpackhi_epi32(tl, t);
            let s_lo = _mm256_sad_epu8(t_lo, tl_lo);
            let s_hi = _mm256_sad_epu8(t_hi, tl_hi);
            _mm256_packs_epi32(s_lo, s_hi) // pa = sum |T-TL|
        };
        {
            let t_bak = t;
            let tl_bak = tl;
            let src_bak = src;
            let pa_bak = pa;
            let out = out_pos + idx(i);
            let mut j = 0;
            while j < 4 {
                data[out + j] = do_pred11(&mut l, &t, &tl, &src, &pa);
                do_pred11_shift(&mut t, &mut tl, &mut src, &mut pa);
                j += 1;
            }
            t = _mm256_permute2x128_si256::<1>(t_bak, t_bak);
            tl = _mm256_permute2x128_si256::<1>(tl_bak, tl_bak);
            src = _mm256_permute2x128_si256::<1>(src_bak, src_bak);
            pa = _mm256_permute2x128_si256::<1>(pa_bak, pa_bak);
            while j < 8 {
                data[out + j] = do_pred11(&mut l, &t, &tl, &src, &pa);
                do_pred11_shift(&mut t, &mut tl, &mut src, &mut pa);
                j += 1;
            }
        }
        i += 8;
    }
    if i != num_pixels {
        // C: VP8LPredictorsAdd_SSE[11] (deviation 2).
        predictor_add11_sse2(
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_avx2.c DO_PRED12
/// Predictor12: `ClampedAddSubtractFull`. Returns the pixel that C stores at
/// `out[i + OUT]`.
#[target_feature(enable = "avx2")]
#[inline]
fn do_pred12(diff: &__m256i, l: &mut __m256i, src: &__m256i, zero: &__m256i) -> u32 {
    let all = _mm256_add_epi16(*l, *diff);
    let alls = _mm256_packus_epi16(all, all);
    let res = _mm256_add_epi8(*src, alls);
    let out = _mm256_cvtsi256_si32(res).cast_unsigned();
    *l = _mm256_unpacklo_epi8(res, *zero);
    out
}

// upstream: src/dsp/lossless_avx2.c DO_PRED12_SHIFT
#[target_feature(enable = "avx2")]
#[inline]
fn do_pred12_shift(diff: &mut __m256i, lane: i32, src: &mut __m256i) {
    // Shift the pre-computed value for the next iteration.
    if lane == 0 {
        *diff = _mm256_srli_si256::<8>(*diff);
    }
    *src = _mm256_srli_si256::<4>(*src);
}

// upstream: src/dsp/lossless_avx2.c PredictorAdd12_AVX2
#[target_feature(enable = "avx2")]
#[expect(clippy::similar_names, reason = "the C names T_lo and TL_lo")]
fn predictor_add12_avx2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let zero = _mm256_setzero_si256();
    let l8 = _mm256_setr_epi32(data[out_pos - 1].cast_signed(), 0, 0, 0, 0, 0, 0, 0);
    let mut l = _mm256_unpacklo_epi8(l8, zero);
    while i + 8 <= num_pixels {
        // Load 8 pixels at a time.
        let mut src = loadu_si256(&data[in_pos + idx(i)..]);
        let t = loadu_si256(&data[upper_pos + idx(i)..]);
        let t_lo = _mm256_unpacklo_epi8(t, zero);
        let t_hi = _mm256_unpackhi_epi8(t, zero);
        let tl = loadu_si256(&data[upper_pos + idx(i) - 1..]);
        let tl_lo = _mm256_unpacklo_epi8(tl, zero);
        let tl_hi = _mm256_unpackhi_epi8(tl, zero);
        let mut diff_lo = _mm256_sub_epi16(t_lo, tl_lo);
        let mut diff_hi = _mm256_sub_epi16(t_hi, tl_hi);
        let diff_lo_bak = diff_lo;
        let diff_hi_bak = diff_hi;
        let src_bak = src;
        let out = out_pos + idx(i);
        data[out] = do_pred12(&diff_lo, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_lo, 0, &mut src);
        data[out + 1] = do_pred12(&diff_lo, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_lo, 0, &mut src);
        data[out + 2] = do_pred12(&diff_hi, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_hi, 0, &mut src);
        data[out + 3] = do_pred12(&diff_hi, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_hi, 0, &mut src);

        // Process the upper lane.
        diff_lo = _mm256_permute2x128_si256::<1>(diff_lo_bak, diff_lo_bak);
        diff_hi = _mm256_permute2x128_si256::<1>(diff_hi_bak, diff_hi_bak);
        src = _mm256_permute2x128_si256::<1>(src_bak, src_bak);

        data[out + 4] = do_pred12(&diff_lo, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_lo, 0, &mut src);
        data[out + 5] = do_pred12(&diff_lo, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_lo, 1, &mut src);
        data[out + 6] = do_pred12(&diff_hi, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_hi, 0, &mut src);
        data[out + 7] = do_pred12(&diff_hi, &mut l, &src, &zero);
        i += 8;
    }
    if i != num_pixels {
        // C: VP8LPredictorsAdd_SSE[12] (deviation 2).
        predictor_add12_sse2(
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// Due to averages with integers, values cannot be accumulated in parallel for
// predictors 13.

/// Generates the entry point of an AVX2 batch predictor for
/// `vp8l_predictors_add`.
macro_rules! predictor_add_entry {
    ($entry:ident, $body:ident, $mode:literal) => {
        #[doc = concat!(
                    "Entry point of [`", stringify!($body), "`] for `vp8l_predictors_add` mode ",
                    $mode, "."
                )]
        pub(crate) fn $entry(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            // SAFETY: `vp8l_predictors_add` calls this entry point only after
            // `features().sse2`, `features().sse4_1` and `features().avx2`, the
            // answers of `VP8GetCPUInfo` for `kSSE2`, `kSSE4_1` and `kAVX2`,
            // selected the AVX2 variant, so the CPU has AVX2, which the
            // function requires.
            unsafe { $body(data, in_pos, upper_pos, num_pixels, out_pos) }
        }
    };
}

predictor_add_entry!(predictor_add0, predictor_add0_avx2, 0);
predictor_add_entry!(predictor_add1, predictor_add1_avx2, 1);
predictor_add_entry!(predictor_add2, predictor_add2_avx2, 2);
predictor_add_entry!(predictor_add3, predictor_add3_avx2, 3);
predictor_add_entry!(predictor_add4, predictor_add4_avx2, 4);
predictor_add_entry!(predictor_add8, predictor_add8_avx2, 8);
predictor_add_entry!(predictor_add9, predictor_add9_avx2, 9);
predictor_add_entry!(predictor_add10, predictor_add10_avx2, 10);
predictor_add_entry!(predictor_add11, predictor_add11_avx2, 11);
predictor_add_entry!(predictor_add12, predictor_add12_avx2, 12);

//------------------------------------------------------------------------------
// Subtract-Green Transform

// upstream: src/dsp/lossless_avx2.c AddGreenToBlueAndRed_AVX2
#[target_feature(enable = "avx2")]
fn add_green_to_blue_and_red_avx2(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    let mut i: i32 = 0;
    let k_cst_shuffle = _mm256_set_epi8(
        -1, 29, -1, 29, -1, 25, -1, 25, -1, 21, -1, 21, -1, 17, -1, 17, -1, 13, -1, 13, -1, 9, -1,
        9, -1, 5, -1, 5, -1, 1, -1, 1,
    );
    while i + 8 <= num_pixels {
        let input = loadu_si256(&data[src_pos + idx(i)..]); // argb
        let in_0g0g = _mm256_shuffle_epi8(input, k_cst_shuffle); // 0g0g
        let out = _mm256_add_epi8(input, in_0g0g);
        storeu_si256(&mut data[dst_pos + idx(i)..], out);
        i += 8;
    }
    // fallthrough and finish off with SSE.
    if i != num_pixels {
        // C: VP8LAddGreenToBlueAndRed_SSE (deviation 2).
        add_green_to_blue_and_red_sse2(data, src_pos + idx(i), num_pixels - i, dst_pos + idx(i));
    }
}

/// Entry point of [`add_green_to_blue_and_red_avx2`] for
/// `vp8l_add_green_to_blue_and_red`.
pub(crate) fn add_green_to_blue_and_red(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    // SAFETY: `vp8l_add_green_to_blue_and_red` calls this entry point only
    // after `features().sse2`, `features().sse4_1` and `features().avx2`
    // selected the AVX2 variant, so the CPU has AVX2, which the function
    // requires.
    unsafe { add_green_to_blue_and_red_avx2(data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------
// Color Transform

// upstream: src/dsp/lossless_avx2.c CST
/// sign-extended multiplying constants, pre-shifted by 5.
#[inline]
fn cst(x: u8) -> i32 {
    i32::from((u16::from(x) << 8).cast_signed()) >> 5 // sign-extend
}

// upstream: src/dsp/lossless_avx2.c TransformColorInverse_AVX2
#[target_feature(enable = "avx2")]
#[expect(clippy::many_single_char_names, reason = "the C register names A to G")]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color_inverse"
)]
fn transform_color_inverse_avx2(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    let mults_rb = _mm256_set1_epi32(
        ((cst(m.green_to_red).cast_unsigned() << 16)
            | (cst(m.green_to_blue) & 0xffff).cast_unsigned())
        .cast_signed(),
    );
    let mults_b2 = _mm256_set1_epi32(cst(m.red_to_blue));
    let mask_ag = _mm256_set1_epi32(0xff00_ff00_u32.cast_signed());
    let perm1 = _mm256_setr_epi8(
        -1, 1, -1, 1, -1, 5, -1, 5, -1, 9, -1, 9, -1, 13, -1, 13, -1, 17, -1, 17, -1, 21, -1, 21,
        -1, 25, -1, 25, -1, 29, -1, 29,
    );
    let perm2 = _mm256_setr_epi8(
        -1, 2, -1, -1, -1, 6, -1, -1, -1, 10, -1, -1, -1, 14, -1, -1, -1, 18, -1, -1, -1, 22, -1,
        -1, -1, 26, -1, -1, -1, 30, -1, -1,
    );
    let mut i: i32 = 0;
    while i + 8 <= num_pixels {
        let a = loadu_si256(&data[src_pos + idx(i)..]);
        let b = _mm256_shuffle_epi8(a, perm1); // argb -> g0g0
        let c = _mm256_mulhi_epi16(b, mults_rb);
        let d = _mm256_add_epi8(a, c);
        let e = _mm256_shuffle_epi8(d, perm2);
        let f = _mm256_mulhi_epi16(e, mults_b2);
        let g = _mm256_add_epi8(d, f);
        let out = _mm256_blendv_epi8(g, a, mask_ag);
        storeu_si256(&mut data[dst_pos + idx(i)..], out);
        i += 8;
    }
    // Fall-back to SSE-version for left-overs.
    if i != num_pixels {
        // C: VP8LTransformColorInverse_SSE (deviation 2).
        transform_color_inverse_sse41(m, data, src_pos + idx(i), num_pixels - i, dst_pos + idx(i));
    }
}

/// Entry point of [`transform_color_inverse_avx2`] for
/// `vp8l_transform_color_inverse`.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color_inverse"
)]
pub(crate) fn transform_color_inverse(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    // SAFETY: `vp8l_transform_color_inverse` calls this entry point only after
    // `features().sse2`, `features().sse4_1` and `features().avx2` selected
    // the AVX2 variant, so the CPU has AVX2, which the function requires.
    unsafe { transform_color_inverse_avx2(m, data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------
// Color-space conversion functions

// upstream: src/dsp/lossless_avx2.c ConvertBGRAToRGBA_AVX2
#[target_feature(enable = "avx2")]
fn convert_bgra_to_rgba_avx2(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes
    while num_pixels >= 8 {
        let a = loadu_si256(&src[input..]);
        input += 8;
        let b = _mm256_shuffle_epi8(
            a,
            _mm256_set_epi8(
                15, 12, 13, 14, 11, 8, 9, 10, 7, 4, 5, 6, 3, 0, 1, 2, 15, 12, 13, 14, 11, 8, 9, 10,
                7, 4, 5, 6, 3, 0, 1, 2,
            ),
        );
        storeu_si256(&mut dst[out..], b);
        out += 32;
        num_pixels -= 8;
    }
    // left-overs
    if num_pixels > 0 {
        // C: VP8LConvertBGRAToRGBA_SSE (deviation 2).
        convert_bgra_to_rgba_sse2(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_rgba_avx2`] for `vp8l_convert_bgra_to_rgba`.
pub(crate) fn convert_bgra_to_rgba(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_rgba` calls this entry point only after
    // `features().sse2`, `features().sse4_1` and `features().avx2` selected
    // the AVX2 variant, so the CPU has AVX2, which the function requires.
    unsafe { convert_bgra_to_rgba_avx2(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_avx2.c VP8LDspInitAVX2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
