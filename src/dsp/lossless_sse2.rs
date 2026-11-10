// libwebp v1.6.0, src/dsp/lossless_sse2.c:
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
// SSE2 variant of methods for lossless decoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/lossless_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE2 predictors, inverse transforms and color converters of the
//! lossless decoder that `VP8LDspInitSSE2` assigns, behind the dispatch
//! functions of `src/dsp/lossless.rs` (ADR 0008). Each `#[target_feature]`
//! function keeps the upstream name; each function that `VP8LDspInitSSE2`
//! assigns to a pointer has a safe `pub(crate)` entry point named after it
//! without the suffix, which the dispatch function calls after testing
//! `features().sse2`. `VP8LDspInitSSE2` assigns:
//!
//! - `VP8LPredictors[5..14]`: [`predictor5`] to [`predictor13`], behind
//!   `vp8l_predictors`;
//! - `VP8LPredictorsAdd[0..14]`: [`predictor_add0`] to [`predictor_add13`],
//!   behind `vp8l_predictors_add` (`VP8LPredictorsAdd_C` keeps the C
//!   functions);
//! - `VP8LAddGreenToBlueAndRed`: [`add_green_to_blue_and_red`];
//! - `VP8LTransformColorInverse`: [`transform_color_inverse`];
//! - `VP8LConvertBGRAToRGB`, `VP8LConvertBGRAToRGBA`,
//!   `VP8LConvertBGRAToRGBA4444`, `VP8LConvertBGRAToRGB565`,
//!   `VP8LConvertBGRAToBGR`: [`convert_bgra_to_rgb`], [`convert_bgra_to_rgba`],
//!   [`convert_bgra_to_rgba4444`], [`convert_bgra_to_rgb565`],
//!   [`convert_bgra_to_bgr`];
//! - the "SSE exports for AVX and above" `VP8LPredictorsAdd_SSE`,
//!   `VP8LAddGreenToBlueAndRed_SSE`, `VP8LTransformColorInverse_SSE`,
//!   `VP8LConvertBGRAToRGB_SSE` and `VP8LConvertBGRAToRGBA_SSE`, which
//!   `lossless_avx2.rs` calls directly (its deviation 2): the functions it
//!   reaches through them are `pub(crate)` here.
//!
//! The buffers and positions follow `src/dsp/lossless.rs` (WP3): a batch
//! predictor or an inverse transform takes the one pixel buffer of the decoder
//! and the C pointers as positions in it, a converter its source pixels and its
//! output bytes.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **`GENERATE_PREDICTOR_ADD`.** The portable macro of
//!    `src/dsp/lossless_common.rs` generates plain functions, which cannot call
//!    the SSE2 predictors without `unsafe`; the local
//!    `generate_predictor_add_sse2!` generates the same loop as an SSE2
//!    function for `PredictorAdd5_SSE2`, `PredictorAdd6_SSE2`,
//!    `PredictorAdd7_SSE2` and `PredictorAdd13_SSE2`.
//! 3. **Local macros.** `DO_PRED10`, `DO_PRED11`, `DO_PRED12` and their
//!    `_SHIFT` companions, macros over the caller's local registers, are
//!    functions that take those registers by reference and return the output
//!    pixel, which the caller stores at the same index and in the same order;
//!    `CST` and `MK_CST_16` are functions over the multipliers;
//!    `GENERATE_PREDICTOR_1` and `GENERATE_PREDICTOR_2` are local
//!    `macro_rules!` that take the offset of their `IN` operand from
//!    `upper[i]`.
//! 4. **`Select_SSE2`** stores its difference into a local `int16_t[8]`; the
//!    store goes through the checked `storeu_si128` on a local `[i16; 8]`.
//! 5. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3); a converter's output must hold the bytes of
//!    `num_pixels` pixels, which the vector stores write in blocks.
//!
//! # Excluded undefined behavior
//!
//! None: the SSE2 functions compute the C functions' results for every input
//! (upstream's `-noasm` comparison relies on it, and the parity tests cover
//! the full value ranges). An overflow of `num_pixels * 3` in
//! `ConvertBGRAToBGR_SSE2`, undefined in C for more than 715 million pixels,
//! panics in a checked build.
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_SWAP_16BIT_CSP == 1`.** The swapped byte order of
//!    `ConvertBGRAToRGBA4444_SSE2` and `ConvertBGRAToRGB565_SSE2` is not
//!    compiled in the reference configuration (WP4, not-applicable check 5);
//!    the standard order is translated.
//! 2. **`VP8LDspInitSSE2`** only assigns pointers (ADR 0003); its assignments
//!    are the selection in the dispatch functions listed above.
//! 3. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_lossless.rs`); the WP3 and WP7
//!    probes run against upstream's SSE2 in the diagnostic `release-simd`
//!    oracle.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi8, _mm_add_epi16, _mm_and_si128, _mm_andnot_si128, _mm_avg_epu8,
    _mm_cmpgt_epi16, _mm_cmpgt_epi32, _mm_cvtsi32_si128, _mm_cvtsi128_si32, _mm_mulhi_epi16,
    _mm_or_si128, _mm_packs_epi32, _mm_packus_epi16, _mm_sad_epu8, _mm_set_epi32, _mm_set1_epi8,
    _mm_set1_epi32, _mm_setzero_si128, _mm_shuffle_epi32, _mm_shufflehi_epi16, _mm_shufflelo_epi16,
    _mm_slli_epi16, _mm_slli_si128, _mm_srai_epi16, _mm_srli_epi16, _mm_srli_epi32, _mm_srli_epi64,
    _mm_srli_si128, _mm_sub_epi8, _mm_sub_epi16, _mm_subs_epu8, _mm_unpackhi_epi8,
    _mm_unpackhi_epi32, _mm_unpackhi_epi64, _mm_unpacklo_epi8, _mm_unpacklo_epi32,
    _mm_unpacklo_epi64, _mm_xor_si128,
};

use crate::dsp::common_sse2::{
    loadu_si128, storel_epi64, storeu_si128, vp8_planar_to24b_sse2, vp8l32b_to_planar_sse2,
};
use crate::dsp::lossless::{
    VP8LMultipliers, vp8l_add_green_to_blue_and_red_c, vp8l_convert_bgra_to_bgr_c,
    vp8l_convert_bgra_to_rgb_c, vp8l_convert_bgra_to_rgb565_c, vp8l_convert_bgra_to_rgba_c,
    vp8l_convert_bgra_to_rgba4444_c, vp8l_predictors_add_c, vp8l_transform_color_inverse_c,
};
use crate::dsp::lossless_common::vp8l_add_pixels;
use crate::webp::format_constants::ARGB_BLACK;

/// The C loop counter `i` as an index; it counts up from 0.
#[inline]
#[expect(clippy::cast_sign_loss, reason = "the C loop index counts up from 0")]
pub(crate) fn idx(i: i32) -> usize {
    i as usize
}

/// The position `delta` elements away from `pos`, as C's `p + delta`.
#[inline]
pub(crate) fn at(pos: usize, delta: isize) -> usize {
    pos.checked_add_signed(delta)
        .expect("a position before the start of the buffer")
}

/// The `_MM_SHUFFLE(fp3, fp2, fp1, fp0)` macro of `<xmmintrin.h>`, whose Rust
/// counterpart is not stable: the immediate of the 16- and 32-bit shuffles.
pub(crate) const fn mm_shuffle(fp3: i32, fp2: i32, fp1: i32, fp0: i32) -> i32 {
    (fp3 << 6) | (fp2 << 4) | (fp1 << 2) | fp0
}

//------------------------------------------------------------------------------
// Predictor Transform

// upstream: src/dsp/lossless_sse2.c ClampedAddSubtractFull_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn clamped_add_subtract_full_sse2(c0: u32, c1: u32, c2: u32) -> u32 {
    let zero = _mm_setzero_si128();
    let c0 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(c0.cast_signed()), zero);
    let c1 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(c1.cast_signed()), zero);
    let c2 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(c2.cast_signed()), zero);
    let v1 = _mm_add_epi16(c0, c1);
    let v2 = _mm_sub_epi16(v1, c2);
    let b = _mm_packus_epi16(v2, v2);
    _mm_cvtsi128_si32(b).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c ClampedAddSubtractHalf_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn clamped_add_subtract_half_sse2(c0: u32, c1: u32, c2: u32) -> u32 {
    let zero = _mm_setzero_si128();
    let c0 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(c0.cast_signed()), zero);
    let c1 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(c1.cast_signed()), zero);
    let b0 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(c2.cast_signed()), zero);
    let avg = _mm_add_epi16(c1, c0);
    let a0 = _mm_srli_epi16::<1>(avg);
    let a1 = _mm_sub_epi16(a0, b0);
    let bgt_a = _mm_cmpgt_epi16(b0, a0);
    let a2 = _mm_sub_epi16(a1, bgt_a);
    let a3 = _mm_srai_epi16::<1>(a2);
    let a4 = _mm_add_epi16(a0, a3);
    let a5 = _mm_packus_epi16(a4, a4);
    _mm_cvtsi128_si32(a5).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c Select_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn select_sse2(a: u32, b: u32, c: u32) -> u32 {
    let zero = _mm_setzero_si128();
    let a0 = _mm_cvtsi32_si128(a.cast_signed());
    let b0 = _mm_cvtsi32_si128(b.cast_signed());
    let c0 = _mm_cvtsi32_si128(c.cast_signed());
    let ac0 = _mm_subs_epu8(a0, c0);
    let ca0 = _mm_subs_epu8(c0, a0);
    let bc0 = _mm_subs_epu8(b0, c0);
    let cb0 = _mm_subs_epu8(c0, b0);
    let ac = _mm_or_si128(ac0, ca0);
    let bc = _mm_or_si128(bc0, cb0);
    let pa = _mm_unpacklo_epi8(ac, zero); // |a - c|
    let pb = _mm_unpacklo_epi8(bc, zero); // |b - c|
    let diff = _mm_sub_epi16(pb, pa);
    let pa_minus_pb = {
        let mut out = [0_i16; 8];
        storeu_si128(&mut out, diff);
        i32::from(out[0]) + i32::from(out[1]) + i32::from(out[2]) + i32::from(out[3])
    };
    if pa_minus_pb <= 0 { a } else { b }
}

// upstream: src/dsp/lossless_sse2.c Average2_m128i
#[target_feature(enable = "sse2")]
#[inline]
fn average2_m128i(a0: &__m128i, a1: &__m128i, avg: &mut __m128i) {
    // (a + b) >> 1 = ((a + b + 1) >> 1) - ((a ^ b) & 1)
    let ones = _mm_set1_epi8(1);
    let avg1 = _mm_avg_epu8(*a0, *a1);
    let one = _mm_and_si128(_mm_xor_si128(*a0, *a1), ones);
    *avg = _mm_sub_epi8(avg1, one);
}

// upstream: src/dsp/lossless_sse2.c Average2_uint32_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn average2_uint32_sse2(a0: u32, a1: u32, avg: &mut __m128i) {
    // (a + b) >> 1 = ((a + b + 1) >> 1) - ((a ^ b) & 1)
    let ones = _mm_set1_epi8(1);
    let a0 = _mm_cvtsi32_si128(a0.cast_signed());
    let a1 = _mm_cvtsi32_si128(a1.cast_signed());
    let avg1 = _mm_avg_epu8(a0, a1);
    let one = _mm_and_si128(_mm_xor_si128(a0, a1), ones);
    *avg = _mm_sub_epi8(avg1, one);
}

// upstream: src/dsp/lossless_sse2.c Average2_uint32_16_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn average2_uint32_16_sse2(a0: u32, a1: u32) -> __m128i {
    let zero = _mm_setzero_si128();
    let a0 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(a0.cast_signed()), zero);
    let a1 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(a1.cast_signed()), zero);
    let sum = _mm_add_epi16(a1, a0);
    _mm_srli_epi16::<1>(sum)
}

// upstream: src/dsp/lossless_sse2.c Average2_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn average2_sse2(a0: u32, a1: u32) -> u32 {
    let mut output = _mm_setzero_si128();
    average2_uint32_sse2(a0, a1, &mut output);
    _mm_cvtsi128_si32(output).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c Average3_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn average3_sse2(a0: u32, a1: u32, a2: u32) -> u32 {
    let zero = _mm_setzero_si128();
    let avg1 = average2_uint32_16_sse2(a0, a2);
    let a1 = _mm_unpacklo_epi8(_mm_cvtsi32_si128(a1.cast_signed()), zero);
    let sum = _mm_add_epi16(avg1, a1);
    let avg2 = _mm_srli_epi16::<1>(sum);
    let a2 = _mm_packus_epi16(avg2, avg2);
    _mm_cvtsi128_si32(a2).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c Average4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn average4_sse2(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    let avg1 = average2_uint32_16_sse2(a0, a1);
    let avg2 = average2_uint32_16_sse2(a2, a3);
    let sum = _mm_add_epi16(avg2, avg1);
    let avg3 = _mm_srli_epi16::<1>(sum);
    let a0 = _mm_packus_epi16(avg3, avg3);
    _mm_cvtsi128_si32(a0).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c Predictor5_SSE2
#[target_feature(enable = "sse2")]
fn predictor5_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average3_sse2(left[left_pos], top[top_pos], top[top_pos + 1])
}

// upstream: src/dsp/lossless_sse2.c Predictor6_SSE2
#[target_feature(enable = "sse2")]
fn predictor6_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2_sse2(left[left_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless_sse2.c Predictor7_SSE2
#[target_feature(enable = "sse2")]
fn predictor7_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2_sse2(left[left_pos], top[top_pos])
}

// upstream: src/dsp/lossless_sse2.c Predictor8_SSE2
#[target_feature(enable = "sse2")]
fn predictor8_sse2(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2_sse2(top[top_pos - 1], top[top_pos])
}

// upstream: src/dsp/lossless_sse2.c Predictor9_SSE2
#[target_feature(enable = "sse2")]
fn predictor9_sse2(_left: &[u32], _left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average2_sse2(top[top_pos], top[top_pos + 1])
}

// upstream: src/dsp/lossless_sse2.c Predictor10_SSE2
#[target_feature(enable = "sse2")]
fn predictor10_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    average4_sse2(
        left[left_pos],
        top[top_pos - 1],
        top[top_pos],
        top[top_pos + 1],
    )
}

// upstream: src/dsp/lossless_sse2.c Predictor11_SSE2
#[target_feature(enable = "sse2")]
fn predictor11_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    select_sse2(top[top_pos], left[left_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless_sse2.c Predictor12_SSE2
#[target_feature(enable = "sse2")]
fn predictor12_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    clamped_add_subtract_full_sse2(left[left_pos], top[top_pos], top[top_pos - 1])
}

// upstream: src/dsp/lossless_sse2.c Predictor13_SSE2
#[target_feature(enable = "sse2")]
fn predictor13_sse2(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
    clamped_add_subtract_half_sse2(left[left_pos], top[top_pos], top[top_pos - 1])
}

/// Generates the entry point of an SSE2 one-pixel predictor for
/// `vp8l_predictors`.
macro_rules! predictor_entry {
    ($entry:ident, $body:ident, $mode:literal) => {
        #[doc = concat!(
                    "Entry point of [`", stringify!($body), "`] for `vp8l_predictors` mode ",
                    $mode, "."
                )]
        pub(crate) fn $entry(left: &[u32], left_pos: usize, top: &[u32], top_pos: usize) -> u32 {
            // SAFETY: `vp8l_predictors` calls this entry point only after
            // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`,
            // selected the SSE2 variant, so the CPU has SSE2.
            unsafe { $body(left, left_pos, top, top_pos) }
        }
    };
}

predictor_entry!(predictor5, predictor5_sse2, 5);
predictor_entry!(predictor6, predictor6_sse2, 6);
predictor_entry!(predictor7, predictor7_sse2, 7);
predictor_entry!(predictor8, predictor8_sse2, 8);
predictor_entry!(predictor9, predictor9_sse2, 9);
predictor_entry!(predictor10, predictor10_sse2, 10);
predictor_entry!(predictor11, predictor11_sse2, 11);
predictor_entry!(predictor12, predictor12_sse2, 12);
predictor_entry!(predictor13, predictor13_sse2, 13);

// Batch versions of those functions.

// upstream: src/dsp/lossless_sse2.c PredictorAdd0_SSE2
/// Predictor0: `ARGB_BLACK`.
#[target_feature(enable = "sse2")]
pub(crate) fn predictor_add0_sse2(
    data: &mut [u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let black = _mm_set1_epi32(ARGB_BLACK.cast_signed());
    while i + 4 <= num_pixels {
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let res = _mm_add_epi8(src, black);
        storeu_si128(&mut data[out_pos + idx(i)..], res);
        i += 4;
    }
    if i != num_pixels {
        // C: upper is NULL.
        vp8l_predictors_add_c(
            0,
            data,
            in_pos + idx(i),
            0,
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_sse2.c PredictorAdd1_SSE2
/// Predictor1: left.
#[target_feature(enable = "sse2")]
pub(crate) fn predictor_add1_sse2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let mut prev = _mm_set1_epi32(data[out_pos - 1].cast_signed());
    while i + 4 <= num_pixels {
        // a | b | c | d
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        // 0 | a | b | c
        let shift0 = _mm_slli_si128::<4>(src);
        // a | a + b | b + c | c + d
        let sum0 = _mm_add_epi8(src, shift0);
        // 0 | 0 | a | a + b
        let shift1 = _mm_slli_si128::<8>(sum0);
        // a | a + b | a + b + c | a + b + c + d
        let sum1 = _mm_add_epi8(sum0, shift1);
        let res = _mm_add_epi8(sum1, prev);
        storeu_si128(&mut data[out_pos + idx(i)..], res);
        // replicate prev output on the four lanes
        prev = _mm_shuffle_epi32::<{ 3 | (3 << 2) | (3 << 4) | (3 << 6) }>(res);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_add_c(
            1,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_sse2.c GENERATE_PREDICTOR_1
/// Macro that adds 32-bit integers from IN using mod 256 arithmetic
/// per 8 bit channel. `$delta` is the offset of IN from `upper[i]`.
macro_rules! generate_predictor_1 {
    ($x:literal, $name:ident, $delta:literal) => {
        #[target_feature(enable = "sse2")]
        pub(crate) fn $name(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            let mut i: i32 = 0;
            while i + 4 <= num_pixels {
                let src = loadu_si128(&data[in_pos + idx(i)..]);
                let other = loadu_si128(&data[at(upper_pos + idx(i), $delta)..]);
                let res = _mm_add_epi8(src, other);
                storeu_si128(&mut data[out_pos + idx(i)..], res);
                i += 4;
            }
            if i != num_pixels {
                vp8l_predictors_add_c(
                    $x,
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
// upstream: src/dsp/lossless_sse2.c PredictorAdd2_SSE2
generate_predictor_1!(2, predictor_add2_sse2, 0);
// Predictor3: Top-right.
// upstream: src/dsp/lossless_sse2.c PredictorAdd3_SSE2
generate_predictor_1!(3, predictor_add3_sse2, 1);
// Predictor4: Top-left.
// upstream: src/dsp/lossless_sse2.c PredictorAdd4_SSE2
generate_predictor_1!(4, predictor_add4_sse2, -1);

// upstream: src/dsp/lossless_common.h GENERATE_PREDICTOR_ADD
/// `GENERATE_PREDICTOR_ADD` for an SSE2 predictor, as an SSE2 function
/// (deviation 2).
macro_rules! generate_predictor_add_sse2 {
    ($predictor:ident, $predictor_add:ident) => {
        #[target_feature(enable = "sse2")]
        pub(crate) fn $predictor_add(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            for x in 0..usize::try_from(num_pixels).unwrap_or(0) {
                let pred = $predictor(data, (out_pos + x).wrapping_sub(1), data, upper_pos + x);
                data[out_pos + x] = vp8l_add_pixels(data[in_pos + x], pred);
            }
        }
    };
}

// Due to averages with integers, values cannot be accumulated in parallel for
// predictors 5 to 7.
// upstream: src/dsp/lossless_sse2.c PredictorAdd5_SSE2
generate_predictor_add_sse2!(predictor5_sse2, predictor_add5_sse2);
// upstream: src/dsp/lossless_sse2.c PredictorAdd6_SSE2
generate_predictor_add_sse2!(predictor6_sse2, predictor_add6_sse2);
// upstream: src/dsp/lossless_sse2.c PredictorAdd7_SSE2
generate_predictor_add_sse2!(predictor7_sse2, predictor_add7_sse2);

// upstream: src/dsp/lossless_sse2.c GENERATE_PREDICTOR_2
/// `$delta` is the offset of IN from `upper[i]`.
macro_rules! generate_predictor_2 {
    ($x:literal, $name:ident, $delta:literal) => {
        #[target_feature(enable = "sse2")]
        pub(crate) fn $name(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            let mut i: i32 = 0;
            while i + 4 <= num_pixels {
                let t_other = loadu_si128(&data[at(upper_pos + idx(i), $delta)..]);
                let t = loadu_si128(&data[upper_pos + idx(i)..]);
                let src = loadu_si128(&data[in_pos + idx(i)..]);
                let mut avg = _mm_setzero_si128();
                average2_m128i(&t, &t_other, &mut avg);
                let res = _mm_add_epi8(avg, src);
                storeu_si128(&mut data[out_pos + idx(i)..], res);
                i += 4;
            }
            if i != num_pixels {
                vp8l_predictors_add_c(
                    $x,
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
// upstream: src/dsp/lossless_sse2.c PredictorAdd8_SSE2
generate_predictor_2!(8, predictor_add8_sse2, -1);
// Predictor9: average T TR.
// upstream: src/dsp/lossless_sse2.c PredictorAdd9_SSE2
generate_predictor_2!(9, predictor_add9_sse2, 1);

// upstream: src/dsp/lossless_sse2.c DO_PRED10
/// Predictor10: average of (average of (L,TL), average of (T, TR)). Returns
/// the pixel that C stores at `out[i + OUT]`.
#[target_feature(enable = "sse2")]
#[inline]
fn do_pred10(l: &mut __m128i, tl: &__m128i, avg_ttr: &__m128i, src: &__m128i) -> u32 {
    let mut avg_ltl = _mm_setzero_si128();
    let mut avg = _mm_setzero_si128();
    average2_m128i(l, tl, &mut avg_ltl);
    average2_m128i(avg_ttr, &avg_ltl, &mut avg);
    *l = _mm_add_epi8(avg, *src);
    _mm_cvtsi128_si32(*l).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c DO_PRED10_SHIFT
#[target_feature(enable = "sse2")]
#[inline]
fn do_pred10_shift(avg_ttr: &mut __m128i, tl: &mut __m128i, src: &mut __m128i) {
    // Rotate the pre-computed values for the next iteration.
    *avg_ttr = _mm_srli_si128::<4>(*avg_ttr);
    *tl = _mm_srli_si128::<4>(*tl);
    *src = _mm_srli_si128::<4>(*src);
}

// upstream: src/dsp/lossless_sse2.c PredictorAdd10_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn predictor_add10_sse2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let mut l = _mm_cvtsi32_si128(data[out_pos - 1].cast_signed());
    while i + 4 <= num_pixels {
        let mut src = loadu_si128(&data[in_pos + idx(i)..]);
        let mut tl = loadu_si128(&data[upper_pos + idx(i) - 1..]);
        let t = loadu_si128(&data[upper_pos + idx(i)..]);
        let tr = loadu_si128(&data[upper_pos + idx(i) + 1..]);
        let mut avg_ttr = _mm_setzero_si128();
        average2_m128i(&t, &tr, &mut avg_ttr);
        let out = out_pos + idx(i);
        data[out] = do_pred10(&mut l, &tl, &avg_ttr, &src);
        do_pred10_shift(&mut avg_ttr, &mut tl, &mut src);
        data[out + 1] = do_pred10(&mut l, &tl, &avg_ttr, &src);
        do_pred10_shift(&mut avg_ttr, &mut tl, &mut src);
        data[out + 2] = do_pred10(&mut l, &tl, &avg_ttr, &src);
        do_pred10_shift(&mut avg_ttr, &mut tl, &mut src);
        data[out + 3] = do_pred10(&mut l, &tl, &avg_ttr, &src);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_add_c(
            10,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_sse2.c DO_PRED11
/// Predictor11: select. Returns the pixel that C stores at `out[i + OUT]`.
#[target_feature(enable = "sse2")]
#[inline]
fn do_pred11(l: &mut __m128i, t: &__m128i, tl: &__m128i, src: &__m128i, pa: &__m128i) -> u32 {
    let l_lo = _mm_unpacklo_epi32(*l, *t);
    let tl_lo = _mm_unpacklo_epi32(*tl, *t);
    let pb = _mm_sad_epu8(l_lo, tl_lo); // pb = sum |L-TL|
    let mask = _mm_cmpgt_epi32(pb, *pa);
    let a = _mm_and_si128(mask, *l);
    let b = _mm_andnot_si128(mask, *t);
    let pred = _mm_or_si128(a, b); // pred = (pa > b)? L : T
    *l = _mm_add_epi8(*src, pred);
    _mm_cvtsi128_si32(*l).cast_unsigned()
}

// upstream: src/dsp/lossless_sse2.c DO_PRED11_SHIFT
#[target_feature(enable = "sse2")]
#[inline]
fn do_pred11_shift(t: &mut __m128i, tl: &mut __m128i, src: &mut __m128i, pa: &mut __m128i) {
    // Shift the pre-computed value for the next iteration.
    *t = _mm_srli_si128::<4>(*t);
    *tl = _mm_srli_si128::<4>(*tl);
    *src = _mm_srli_si128::<4>(*src);
    *pa = _mm_srli_si128::<4>(*pa);
}

// upstream: src/dsp/lossless_sse2.c PredictorAdd11_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C names T_lo and TL_lo")]
pub(crate) fn predictor_add11_sse2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let mut l = _mm_cvtsi32_si128(data[out_pos - 1].cast_signed());
    while i + 4 <= num_pixels {
        let mut t = loadu_si128(&data[upper_pos + idx(i)..]);
        let mut tl = loadu_si128(&data[upper_pos + idx(i) - 1..]);
        let mut src = loadu_si128(&data[in_pos + idx(i)..]);
        let mut pa = {
            // We can unpack with any value on the upper 32 bits, provided it's the
            // same on both operands (so that their sum of abs diff is zero). Here we
            // use T.
            let t_lo = _mm_unpacklo_epi32(t, t);
            let tl_lo = _mm_unpacklo_epi32(tl, t);
            let t_hi = _mm_unpackhi_epi32(t, t);
            let tl_hi = _mm_unpackhi_epi32(tl, t);
            let s_lo = _mm_sad_epu8(t_lo, tl_lo);
            let s_hi = _mm_sad_epu8(t_hi, tl_hi);
            _mm_packs_epi32(s_lo, s_hi) // pa = sum |T-TL|
        };
        let out = out_pos + idx(i);
        data[out] = do_pred11(&mut l, &t, &tl, &src, &pa);
        do_pred11_shift(&mut t, &mut tl, &mut src, &mut pa);
        data[out + 1] = do_pred11(&mut l, &t, &tl, &src, &pa);
        do_pred11_shift(&mut t, &mut tl, &mut src, &mut pa);
        data[out + 2] = do_pred11(&mut l, &t, &tl, &src, &pa);
        do_pred11_shift(&mut t, &mut tl, &mut src, &mut pa);
        data[out + 3] = do_pred11(&mut l, &t, &tl, &src, &pa);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_add_c(
            11,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            out_pos + idx(i),
        );
    }
}

// upstream: src/dsp/lossless_sse2.c DO_PRED12
/// Predictor12: `ClampedAddSubtractFull`. Returns the pixel that C stores at
/// `out[i + OUT]`.
#[target_feature(enable = "sse2")]
#[inline]
fn do_pred12(diff: &__m128i, l: &mut __m128i, src: &__m128i, zero: &__m128i) -> u32 {
    let all = _mm_add_epi16(*l, *diff);
    let alls = _mm_packus_epi16(all, all);
    let res = _mm_add_epi8(*src, alls);
    let out = _mm_cvtsi128_si32(res).cast_unsigned();
    *l = _mm_unpacklo_epi8(res, *zero);
    out
}

// upstream: src/dsp/lossless_sse2.c DO_PRED12_SHIFT
#[target_feature(enable = "sse2")]
#[inline]
fn do_pred12_shift(diff: &mut __m128i, lane: i32, src: &mut __m128i) {
    // Shift the pre-computed value for the next iteration.
    if lane == 0 {
        *diff = _mm_srli_si128::<8>(*diff);
    }
    *src = _mm_srli_si128::<4>(*src);
}

// upstream: src/dsp/lossless_sse2.c PredictorAdd12_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C names T_lo and TL_lo")]
pub(crate) fn predictor_add12_sse2(
    data: &mut [u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out_pos: usize,
) {
    let mut i: i32 = 0;
    let zero = _mm_setzero_si128();
    let l8 = _mm_cvtsi32_si128(data[out_pos - 1].cast_signed());
    let mut l = _mm_unpacklo_epi8(l8, zero);
    while i + 4 <= num_pixels {
        // Load 4 pixels at a time.
        let mut src = loadu_si128(&data[in_pos + idx(i)..]);
        let t = loadu_si128(&data[upper_pos + idx(i)..]);
        let t_lo = _mm_unpacklo_epi8(t, zero);
        let t_hi = _mm_unpackhi_epi8(t, zero);
        let tl = loadu_si128(&data[upper_pos + idx(i) - 1..]);
        let tl_lo = _mm_unpacklo_epi8(tl, zero);
        let tl_hi = _mm_unpackhi_epi8(tl, zero);
        let mut diff_lo = _mm_sub_epi16(t_lo, tl_lo);
        let mut diff_hi = _mm_sub_epi16(t_hi, tl_hi);
        let out = out_pos + idx(i);
        data[out] = do_pred12(&diff_lo, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_lo, 0, &mut src);
        data[out + 1] = do_pred12(&diff_lo, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_lo, 1, &mut src);
        data[out + 2] = do_pred12(&diff_hi, &mut l, &src, &zero);
        do_pred12_shift(&mut diff_hi, 0, &mut src);
        data[out + 3] = do_pred12(&diff_hi, &mut l, &src, &zero);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_add_c(
            12,
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
// upstream: src/dsp/lossless_sse2.c PredictorAdd13_SSE2
generate_predictor_add_sse2!(predictor13_sse2, predictor_add13_sse2);

/// Generates the entry point of an SSE2 batch predictor for
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
            // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`,
            // selected the SSE2 variant, so the CPU has SSE2.
            unsafe { $body(data, in_pos, upper_pos, num_pixels, out_pos) }
        }
    };
}

predictor_add_entry!(predictor_add0, predictor_add0_sse2, 0);
predictor_add_entry!(predictor_add1, predictor_add1_sse2, 1);
predictor_add_entry!(predictor_add2, predictor_add2_sse2, 2);
predictor_add_entry!(predictor_add3, predictor_add3_sse2, 3);
predictor_add_entry!(predictor_add4, predictor_add4_sse2, 4);
predictor_add_entry!(predictor_add5, predictor_add5_sse2, 5);
predictor_add_entry!(predictor_add6, predictor_add6_sse2, 6);
predictor_add_entry!(predictor_add7, predictor_add7_sse2, 7);
predictor_add_entry!(predictor_add8, predictor_add8_sse2, 8);
predictor_add_entry!(predictor_add9, predictor_add9_sse2, 9);
predictor_add_entry!(predictor_add10, predictor_add10_sse2, 10);
predictor_add_entry!(predictor_add11, predictor_add11_sse2, 11);
predictor_add_entry!(predictor_add12, predictor_add12_sse2, 12);
predictor_add_entry!(predictor_add13, predictor_add13_sse2, 13);

//------------------------------------------------------------------------------
// Subtract-Green Transform

// upstream: src/dsp/lossless_sse2.c AddGreenToBlueAndRed_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn add_green_to_blue_and_red_sse2(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let input = loadu_si128(&data[src_pos + idx(i)..]); // argb
        let a = _mm_srli_epi16::<8>(input); // 0 a 0 g
        let b = _mm_shufflelo_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(a);
        let c = _mm_shufflehi_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(b); // 0g0g
        let out = _mm_add_epi8(input, c);
        storeu_si128(&mut data[dst_pos + idx(i)..], out);
        i += 4;
    }
    // fallthrough and finish off with plain-C
    if i != num_pixels {
        vp8l_add_green_to_blue_and_red_c(data, src_pos + idx(i), num_pixels - i, dst_pos + idx(i));
    }
}

/// Entry point of [`add_green_to_blue_and_red_sse2`] for
/// `vp8l_add_green_to_blue_and_red`.
pub(crate) fn add_green_to_blue_and_red(
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    // SAFETY: `vp8l_add_green_to_blue_and_red` calls this entry point only
    // after `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { add_green_to_blue_and_red_sse2(data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------
// Color Transform

// upstream: src/dsp/lossless_sse2.c CST
/// sign-extended multiplying constants, pre-shifted by 5.
#[inline]
pub(crate) fn cst(x: u8) -> i32 {
    i32::from((u16::from(x) << 8).cast_signed()) >> 5 // sign-extend
}

// upstream: src/dsp/lossless_sse2.c MK_CST_16
#[target_feature(enable = "sse2")]
#[inline]
fn mk_cst_16(hi: i32, lo: i32) -> __m128i {
    _mm_set1_epi32(((hi.cast_unsigned() << 16) | (lo & 0xffff).cast_unsigned()).cast_signed())
}

// upstream: src/dsp/lossless_sse2.c TransformColorInverse_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::many_single_char_names, reason = "the C register names A to J")]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color_inverse"
)]
fn transform_color_inverse_sse2(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    let mults_rb = mk_cst_16(cst(m.green_to_red), cst(m.green_to_blue));
    let mults_b2 = mk_cst_16(cst(m.red_to_blue), 0);
    let mask_ag = _mm_set1_epi32(0xff00_ff00_u32.cast_signed()); // alpha-green masks
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let input = loadu_si128(&data[src_pos + idx(i)..]); // argb
        let a = _mm_and_si128(input, mask_ag); // a   0   g   0
        let b = _mm_shufflelo_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(a);
        let c = _mm_shufflehi_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(b); // g0g0
        let d = _mm_mulhi_epi16(c, mults_rb); // x dr  x db1
        let e = _mm_add_epi8(input, d); // x r'  x   b'
        let f = _mm_slli_epi16::<8>(e); // r' 0   b' 0
        let g = _mm_mulhi_epi16(f, mults_b2); // x db2  0  0
        let h = _mm_srli_epi32::<8>(g); // 0  x db2  0
        let i_ = _mm_add_epi8(h, f); // r' x  b'' 0
        let j = _mm_srli_epi16::<8>(i_); // 0  r'  0  b''
        let out = _mm_or_si128(j, a);
        storeu_si128(&mut data[dst_pos + idx(i)..], out);
        i += 4;
    }
    // Fall-back to C-version for left-overs.
    if i != num_pixels {
        vp8l_transform_color_inverse_c(m, data, src_pos + idx(i), num_pixels - i, dst_pos + idx(i));
    }
}

/// Entry point of [`transform_color_inverse_sse2`] for
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
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { transform_color_inverse_sse2(m, data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------
// Color-space conversion functions

// upstream: src/dsp/lossless_sse2.c ConvertBGRAToRGB_SSE2
#[target_feature(enable = "sse2")]
fn convert_bgra_to_rgb_sse2(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes

    while num_pixels >= 32 {
        // Load the BGRA buffers.
        let mut in0 = loadu_si128(&src[input..]);
        let mut in1 = loadu_si128(&src[input + 4..]);
        let mut in2 = loadu_si128(&src[input + 8..]);
        let mut in3 = loadu_si128(&src[input + 12..]);
        let mut in4 = loadu_si128(&src[input + 16..]);
        let mut in5 = loadu_si128(&src[input + 20..]);
        let mut in6 = loadu_si128(&src[input + 24..]);
        let mut in7 = loadu_si128(&src[input + 28..]);
        vp8l32b_to_planar_sse2(&mut in0, &mut in1, &mut in2, &mut in3);
        vp8l32b_to_planar_sse2(&mut in4, &mut in5, &mut in6, &mut in7);
        // At this points, in1/in5 contains red only, in2/in6 green only ...
        // Pack the colors in 24b RGB.
        vp8_planar_to24b_sse2(&mut in1, &mut in5, &mut in2, &mut in6, &mut in3, &mut in7);
        storeu_si128(&mut dst[out..], in1);
        storeu_si128(&mut dst[out + 16..], in5);
        storeu_si128(&mut dst[out + 32..], in2);
        storeu_si128(&mut dst[out + 48..], in6);
        storeu_si128(&mut dst[out + 64..], in3);
        storeu_si128(&mut dst[out + 80..], in7);
        input += 32;
        out += 96;
        num_pixels -= 32;
    }
    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_rgb_c(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_rgb_sse2`] for `vp8l_convert_bgra_to_rgb`.
pub(crate) fn convert_bgra_to_rgb(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_rgb` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_bgra_to_rgb_sse2(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse2.c ConvertBGRAToRGBA_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn convert_bgra_to_rgba_sse2(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let red_blue_mask = _mm_set1_epi32(0x00ff_00ff);
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes
    while num_pixels >= 8 {
        let a1 = loadu_si128(&src[input..]);
        let a2 = loadu_si128(&src[input + 4..]);
        input += 8;
        let b1 = _mm_and_si128(a1, red_blue_mask); // R 0 B 0
        let b2 = _mm_and_si128(a2, red_blue_mask); // R 0 B 0
        let c1 = _mm_andnot_si128(red_blue_mask, a1); // 0 G 0 A
        let c2 = _mm_andnot_si128(red_blue_mask, a2); // 0 G 0 A
        let d1 = _mm_shufflelo_epi16::<{ mm_shuffle(2, 3, 0, 1) }>(b1);
        let d2 = _mm_shufflelo_epi16::<{ mm_shuffle(2, 3, 0, 1) }>(b2);
        let e1 = _mm_shufflehi_epi16::<{ mm_shuffle(2, 3, 0, 1) }>(d1);
        let e2 = _mm_shufflehi_epi16::<{ mm_shuffle(2, 3, 0, 1) }>(d2);
        let f1 = _mm_or_si128(e1, c1);
        let f2 = _mm_or_si128(e2, c2);
        storeu_si128(&mut dst[out..], f1);
        storeu_si128(&mut dst[out + 16..], f2);
        out += 32;
        num_pixels -= 8;
    }
    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_rgba_c(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_rgba_sse2`] for `vp8l_convert_bgra_to_rgba`.
pub(crate) fn convert_bgra_to_rgba(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_rgba` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_bgra_to_rgba_sse2(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse2.c ConvertBGRAToRGBA4444_SSE2
#[target_feature(enable = "sse2")]
fn convert_bgra_to_rgba4444_sse2(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mask_0x0f = _mm_set1_epi8(0x0f);
    let mask_0xf0 = _mm_set1_epi8(0xf0_u8.cast_signed());
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes
    while num_pixels >= 8 {
        let bgra0 = loadu_si128(&src[input..]); // bgra0|bgra1|bgra2|bgra3
        let bgra4 = loadu_si128(&src[input + 4..]); // bgra4|bgra5|bgra6|bgra7
        input += 8;
        let v0l = _mm_unpacklo_epi8(bgra0, bgra4); // b0b4g0g4r0r4a0a4...
        let v0h = _mm_unpackhi_epi8(bgra0, bgra4); // b2b6g2g6r2r6a2a6...
        let v1l = _mm_unpacklo_epi8(v0l, v0h); // b0b2b4b6g0g2g4g6...
        let v1h = _mm_unpackhi_epi8(v0l, v0h); // b1b3b5b7g1g3g5g7...
        let v2l = _mm_unpacklo_epi8(v1l, v1h); // b0...b7 | g0...g7
        let v2h = _mm_unpackhi_epi8(v1l, v1h); // r0...r7 | a0...a7
        let ga0 = _mm_unpackhi_epi64(v2l, v2h); // g0...g7 | a0...a7
        let rb0 = _mm_unpacklo_epi64(v2h, v2l); // r0...r7 | b0...b7
        let ga1 = _mm_srli_epi16::<4>(ga0); // g0-|g1-|...|a6-|a7-
        let rb1 = _mm_and_si128(rb0, mask_0xf0); // -r0|-r1|...|-b6|-a7
        let ga2 = _mm_and_si128(ga1, mask_0x0f); // g0-|g1-|...|a6-|a7-
        let rgba0 = _mm_or_si128(ga2, rb1); // rg0..rg7 | ba0..ba7
        let rgba1 = _mm_srli_si128::<8>(rgba0); // ba0..ba7 | 0
        let rgba = _mm_unpacklo_epi8(rgba0, rgba1); // rgba0...rgba7
        storeu_si128(&mut dst[out..], rgba);
        out += 16;
        num_pixels -= 8;
    }
    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_rgba4444_c(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_rgba4444_sse2`] for
/// `vp8l_convert_bgra_to_rgba4444`.
pub(crate) fn convert_bgra_to_rgba4444(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_rgba4444` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_bgra_to_rgba4444_sse2(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse2.c ConvertBGRAToRGB565_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C mask names")]
fn convert_bgra_to_rgb565_sse2(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mask_0xe0 = _mm_set1_epi8(0xe0_u8.cast_signed());
    let mask_0xf8 = _mm_set1_epi8(0xf8_u8.cast_signed());
    let mask_0x07 = _mm_set1_epi8(0x07);
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes
    while num_pixels >= 8 {
        let bgra0 = loadu_si128(&src[input..]); // bgra0|bgra1|bgra2|bgra3
        let bgra4 = loadu_si128(&src[input + 4..]); // bgra4|bgra5|bgra6|bgra7
        input += 8;
        let v0l = _mm_unpacklo_epi8(bgra0, bgra4); // b0b4g0g4r0r4a0a4...
        let v0h = _mm_unpackhi_epi8(bgra0, bgra4); // b2b6g2g6r2r6a2a6...
        let v1l = _mm_unpacklo_epi8(v0l, v0h); // b0b2b4b6g0g2g4g6...
        let v1h = _mm_unpackhi_epi8(v0l, v0h); // b1b3b5b7g1g3g5g7...
        let v2l = _mm_unpacklo_epi8(v1l, v1h); // b0...b7 | g0...g7
        let v2h = _mm_unpackhi_epi8(v1l, v1h); // r0...r7 | a0...a7
        let ga0 = _mm_unpackhi_epi64(v2l, v2h); // g0...g7 | a0...a7
        let rb0 = _mm_unpacklo_epi64(v2h, v2l); // r0...r7 | b0...b7
        let rb1 = _mm_and_si128(rb0, mask_0xf8); // -r0..-r7|-b0..-b7
        let g_lo1 = _mm_srli_epi16::<5>(ga0);
        let g_lo2 = _mm_and_si128(g_lo1, mask_0x07); // g0-...g7-|xx (3b)
        let g_hi1 = _mm_slli_epi16::<3>(ga0);
        let g_hi2 = _mm_and_si128(g_hi1, mask_0xe0); // -g0...-g7|xx (3b)
        let b0 = _mm_srli_si128::<8>(rb1); // -b0...-b7|0
        let rg1 = _mm_or_si128(rb1, g_lo2); // gr0...gr7|xx
        let b1 = _mm_srli_epi16::<3>(b0);
        let gb1 = _mm_or_si128(b1, g_hi2); // bg0...bg7|xx
        let rgba = _mm_unpacklo_epi8(rg1, gb1); // bgrb0...bgrb7
        storeu_si128(&mut dst[out..], rgba);
        out += 16;
        num_pixels -= 8;
    }
    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_rgb565_c(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_rgb565_sse2`] for
/// `vp8l_convert_bgra_to_rgb565`.
pub(crate) fn convert_bgra_to_rgb565(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_rgb565` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_bgra_to_rgb565_sse2(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse2.c ConvertBGRAToBGR_SSE2
#[target_feature(enable = "sse2")]
fn convert_bgra_to_bgr_sse2(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mask_l = _mm_set_epi32(0, 0x00ff_ffff, 0, 0x00ff_ffff);
    let mask_h = _mm_set_epi32(0x00ff_ffff, 0, 0x00ff_ffff, 0);
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out: i32 = 0; // C: dst, in bytes
    let end = num_pixels * 3;
    // the last storel_epi64 below writes 8 bytes starting at offset 18
    while out + 26 <= end {
        let bgra0 = loadu_si128(&src[input..]); // bgra0|bgra1|bgra2|bgra3
        let bgra4 = loadu_si128(&src[input + 4..]); // bgra4|bgra5|bgra6|bgra7
        input += 8;
        let a0l = _mm_and_si128(bgra0, mask_l); // bgr0|0|bgr0|0
        let a4l = _mm_and_si128(bgra4, mask_l); // bgr0|0|bgr0|0
        let a0h = _mm_and_si128(bgra0, mask_h); // 0|bgr0|0|bgr0
        let a4h = _mm_and_si128(bgra4, mask_h); // 0|bgr0|0|bgr0
        let b0h = _mm_srli_epi64::<8>(a0h); // 000b|gr00|000b|gr00
        let b4h = _mm_srli_epi64::<8>(a4h); // 000b|gr00|000b|gr00
        let c0 = _mm_or_si128(a0l, b0h); // rgbrgb00|rgbrgb00
        let c4 = _mm_or_si128(a4l, b4h); // rgbrgb00|rgbrgb00
        let c2 = _mm_srli_si128::<8>(c0);
        let c6 = _mm_srli_si128::<8>(c4);
        let d = idx(out);
        storel_epi64(&mut dst[d..], c0);
        storel_epi64(&mut dst[d + 6..], c2);
        storel_epi64(&mut dst[d + 12..], c4);
        storel_epi64(&mut dst[d + 18..], c6);
        out += 24;
        num_pixels -= 8;
    }
    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_bgr_c(&src[input..], num_pixels, &mut dst[idx(out)..]);
    }
}

/// Entry point of [`convert_bgra_to_bgr_sse2`] for `vp8l_convert_bgra_to_bgr`.
pub(crate) fn convert_bgra_to_bgr(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_bgr` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_bgra_to_bgr_sse2(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse2.c VP8LDspInitSSE2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
