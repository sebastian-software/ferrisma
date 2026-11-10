// libwebp v1.6.0, src/dsp/dec_sse2.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// SSE2 version of some decoding functions (idct, loop filtering).
//
// Author: somnath@google.com (Somnath Banerjee)
//         cduvivier@google.com (Christian Duvivier)

//! Translation of `src/dsp/dec_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE2 inverse transform, the simple and complex loop filters and the
//! intra predictors that `VP8DspInitSSE2` assigns, behind the dispatch
//! functions of `src/dsp/dec.rs` (ADR 0008). Each `#[target_feature]`
//! function keeps the upstream name; each function that `VP8DspInitSSE2`
//! assigns to a pointer has a safe `pub(crate)` entry point named after it
//! without the suffix, which the dispatch function calls after testing
//! `features().sse2`. `VP8DspInitSSE2` assigns:
//!
//! - `VP8Transform`: [`transform`];
//! - `VP8VFilter16`, `VP8HFilter16`, `VP8VFilter8`, `VP8HFilter8`,
//!   `VP8VFilter16i`, `VP8HFilter16i`, `VP8VFilter8i`, `VP8HFilter8i`:
//!   [`v_filter16`], [`h_filter16`], [`v_filter8`], [`h_filter8`],
//!   [`v_filter16i`], [`h_filter16i`], [`v_filter8i`], [`h_filter8i`];
//! - `VP8SimpleVFilter16`, `VP8SimpleHFilter16`, `VP8SimpleVFilter16i`,
//!   `VP8SimpleHFilter16i`: [`simple_v_filter16`], [`simple_h_filter16`],
//!   [`simple_v_filter16i`], [`simple_h_filter16i`];
//! - `VP8PredLuma4[1, 2, 4, 5, 6, 7]`: [`tm4`], [`ve4`], [`rd4`], [`vr4`],
//!   [`ld4`], [`vl4`];
//! - `VP8PredLuma16[0..7]`: [`dc16`], [`tm16`], [`ve16`], [`he16`],
//!   [`dc16_no_top`], [`dc16_no_left`], [`dc16_no_top_left`];
//! - `VP8PredChroma8[0, 1, 2, 4, 5, 6]`: [`dc8uv`], [`tm8uv`], [`ve8uv`],
//!   [`dc8uv_no_top`], [`dc8uv_no_left`], [`dc8uv_no_top_left`].
//!
//! The buffers and positions follow `src/dsp/dec.rs`: a transform takes the
//! coefficients and the block from its first sample, a predictor the work
//! buffer and the block's position, a filter the plane, the position of `p`
//! and the stride (WP6).
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Two row pointers.** `Load16x4_SSE2` and `Store16x4_SSE2` take C's
//!    `r0` and `r8` as a buffer and a position each; for the luma filters
//!    both are the same plane. `Store16x4_SSE2` takes `r8` as `None` when its
//!    rows lie in `r0`'s plane, because Rust cannot hold two mutable borrows
//!    of one plane; the stores are the same, in the same order.
//! 3. **Reads beyond the scalar footprint.** Where upstream loads eight or
//!    sixteen bytes and uses fewer (`VE4_SSE2`, `LD4_SSE2`, `VL4_SSE2` and
//!    `RD4_SSE2` read up to eight bytes of the row above, the scalar functions
//!    up to six; `Transform_SSE2` reads eight bytes of each row only when it
//!    transforms two blocks), the slice must hold those bytes too, or the
//!    function panics where the scalar one would not. The decoder's work
//!    buffer and planes always hold them, as in C.
//! 4. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3).
//!
//! # Excluded undefined behavior
//!
//! None: the functions read and write only the bytes of their slices, and
//! their arithmetic is defined in every lane.
//!
//! # Checks not applicable
//!
//! 1. **`TransformAC3_SSE2`** is compiled only with `USE_TRANSFORM_AC3 == 1`,
//!    which upstream disables (`#define USE_TRANSFORM_AC3 0`); it is not
//!    translated, and `VP8TransformAC3` keeps the C function.
//! 2. **`VP8DspInitSSE2`** only assigns pointers (ADR 0003); its assignments
//!    are the selection in the dispatch functions listed above.
//! 3. **Differential probe.** The parity tests compare each function with
//!    the C translation it replaces (`tests/wp18_dec.rs`); the C oracle in the
//!    reference configuration has no SSE2 code, and the diagnostic
//!    `release-simd` oracle runs the WP6 probes against upstream's SSE2.
//! 4. **Comparison with the reference oracle outside the shared domain.**
//!    Upstream's SSE2 functions equal the C functions they replace only in a
//!    domain that valid streams stay in, and differ outside it in upstream
//!    itself:
//!    - `Transform_SSE2` (`vp8_transform`, and `vp8_transform_uv` through it)
//!      computes in wrapping 16-bit lanes where `TransformOne_C` computes in
//!      `int`; they agree for coefficients within ±2048, the range the
//!      parity tests draw from, and differ for larger ones, which only corrupt
//!      streams and probes produce.
//!    - `NeedsFilter_SSE2` (every loop filter) tests
//!      `2 * |p0 - q0| + |p1 - q1| / 2 <= thresh` in saturating 8-bit lanes
//!      with `thresh` truncated to 8 bits, which equals the C test
//!      `4 * |p0 - q0| + |p1 - q1| <= 2 * thresh + 1` for `thresh` up to 254;
//!      `ithresh` and `hev_thresh` are truncated to 8 bits as well and agree
//!      up to 255. The decoder passes `thresh` up to 193 (`2 * 63 + 63 + 4`),
//!      `ithresh` up to 63 and `hev_thresh` up to 2.
//!
//!    Outside that domain the port computes what upstream's SSE2 build
//!    computes: `tests/wp6_dsp.rs` (coefficients to ±15000, thresholds to
//!    1200) and `tests/wp6_decode.rs` (mutated streams) matched the
//!    `release-simd` oracle byte for byte with the SSE2 variants selected.
//!    Their comparisons with the reference oracle, which has no SIMD code,
//!    select the C translation for exactly those cases through
//!    `VP8GetCPUInfo = NULL`, as `-noasm` does (ADR 0008). The valid-domain
//!    evidence with SSE2 selected is the decoding of the 131 test vectors,
//!    the encoder matrix and the Rust-linked upstream scripts.
//!
//! # Notes for WP20
//!
//! WP18 added an x86 block to these dispatch functions of `src/dsp/dec.rs`:
//! `vp8_transform`, `vp8v_filter16`, `vp8h_filter16`, `vp8v_filter8`,
//! `vp8h_filter8`, `vp8v_filter16i`, `vp8h_filter16i`, `vp8v_filter8i`,
//! `vp8h_filter8i`, `vp8_simple_v_filter16`, `vp8_simple_h_filter16`,
//! `vp8_simple_v_filter16i`, `vp8_simple_h_filter16i`, `vp8_pred_luma4`,
//! `vp8_pred_luma16` and `vp8_pred_chroma8` (with `dec_sse41.rs` for
//! `VP8PredLuma16[3]`). The other portable modules have x86 blocks in:
//! `src/dsp/upsampling.rs` (`webp_upsamplers`, `webp_yuv444_converters`),
//! `src/dsp/yuv.rs` (`webp_samplers`, `webp_convert_argb_to_y`,
//! `webp_convert_argb_to_uv`, `webp_convert_rgb24_to_y`,
//! `webp_convert_bgr24_to_y`, `webp_convert_rgba32_to_uv`),
//! `src/dsp/alpha_processing.rs`, `src/dsp/filters.rs`,
//! `src/dsp/rescaler.rs` and `src/dsp/lossless.rs`; each module's SIMD
//! files list their dispatch functions in their documentation. Each x86
//! block stands first in its function, in the layout of the WP18 brief;
//! put the `AArch64` block after it and keep the C call last with its
//! `unreachable_code` allowance. A predictor table selects per mode inside
//! the x86 block with a `match` that falls through to the table.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi8, _mm_add_epi16, _mm_adds_epi8, _mm_adds_epu8, _mm_and_si128,
    _mm_andnot_si128, _mm_avg_epu8, _mm_cmpeq_epi8, _mm_cvtsi32_si128, _mm_cvtsi128_si32,
    _mm_insert_epi16, _mm_max_epu8, _mm_mulhi_epi16, _mm_or_si128, _mm_packs_epi16,
    _mm_packus_epi16, _mm_sad_epu8, _mm_set_epi32, _mm_set1_epi8, _mm_set1_epi16,
    _mm_setzero_si128, _mm_shuffle_epi32, _mm_slli_si128, _mm_srai_epi16, _mm_srli_epi16,
    _mm_srli_si128, _mm_sub_epi8, _mm_sub_epi16, _mm_subs_epi8, _mm_subs_epu8, _mm_unpackhi_epi8,
    _mm_unpackhi_epi16, _mm_unpackhi_epi32, _mm_unpackhi_epi64, _mm_unpacklo_epi8,
    _mm_unpacklo_epi16, _mm_unpacklo_epi32, _mm_unpacklo_epi64, _mm_xor_si128,
};

use crate::dsp::common_sse2::{
    loadl_epi64, loadu_si128, storel_epi64, storeu_si128, vp8_transpose_2_4x4_16b,
};
use crate::dsp::dsp::BPS;
use crate::utils::utils::{webp_int32_to_mem, webp_mem_to_int32};

/// `BPS` as an index step.
const BPS_U: usize = BPS as usize;

/// The position `delta` elements away from `pos`, as C's pointer arithmetic
/// `p + delta` (as in `src/dsp/dec.rs`).
fn at(pos: usize, delta: i32) -> usize {
    pos.checked_add_signed(isize::try_from(delta).expect("an i32 fits in isize"))
        .expect("a position before the start of the buffer")
}

//------------------------------------------------------------------------------
// Transforms (Paragraph 14.4)

// upstream: src/dsp/dec_sse2.c Transform_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::too_many_lines, reason = "the C function")]
fn transform_sse2(input: &[i16], dst: &mut [u8], do_two: bool) {
    // This implementation makes use of 16-bit fixed point versions of two
    // multiply constants:
    //    K1 = sqrt(2) * cos (pi/8) ~= 85627 / 2^16
    //    K2 = sqrt(2) * sin (pi/8) ~= 35468 / 2^16
    //
    // To be able to use signed 16-bit integers, we use the following trick to
    // have constants within range:
    // - Associated constants are obtained by subtracting the 16-bit fixed point
    //   version of one:
    //      k = K - (1 << 16)  =>  K = k + (1 << 16)
    //      K1 = 85267  =>  k1 =  20091
    //      K2 = 35468  =>  k2 = -30068
    // - The multiplication of a variable by a constant become the sum of the
    //   variable and the multiplication of that variable by the associated
    //   constant:
    //      (x * K) >> 16 = (x * (k + (1 << 16))) >> 16 = ((x * k ) >> 16) + x
    let k1 = _mm_set1_epi16(20091);
    let k2 = _mm_set1_epi16(-30068);
    let mut t0 = _mm_setzero_si128();
    let mut t1 = _mm_setzero_si128();
    let mut t2 = _mm_setzero_si128();
    let mut t3 = _mm_setzero_si128();

    // Load and concatenate the transform coefficients (we'll do two transforms
    // in parallel). In the case of only one transform, the second half of the
    // vectors will just contain random value we'll never use nor store.
    let mut in0 = loadl_epi64(&input[0..]);
    let mut in1 = loadl_epi64(&input[4..]);
    let mut in2 = loadl_epi64(&input[8..]);
    let mut in3 = loadl_epi64(&input[12..]);
    // a00 a10 a20 a30   x x x x
    // a01 a11 a21 a31   x x x x
    // a02 a12 a22 a32   x x x x
    // a03 a13 a23 a33   x x x x
    if do_two {
        let in_b0 = loadl_epi64(&input[16..]);
        let in_b1 = loadl_epi64(&input[20..]);
        let in_b2 = loadl_epi64(&input[24..]);
        let in_b3 = loadl_epi64(&input[28..]);
        in0 = _mm_unpacklo_epi64(in0, in_b0);
        in1 = _mm_unpacklo_epi64(in1, in_b1);
        in2 = _mm_unpacklo_epi64(in2, in_b2);
        in3 = _mm_unpacklo_epi64(in3, in_b3);
        // a00 a10 a20 a30   b00 b10 b20 b30
        // a01 a11 a21 a31   b01 b11 b21 b31
        // a02 a12 a22 a32   b02 b12 b22 b32
        // a03 a13 a23 a33   b03 b13 b23 b33
    }

    // Vertical pass and subsequent transpose.
    {
        // First pass, c and d calculations are longer because of the "trick"
        // multiplications.
        let a = _mm_add_epi16(in0, in2);
        let b = _mm_sub_epi16(in0, in2);
        // c = MUL(in1, K2) - MUL(in3, K1) = MUL(in1, k2) - MUL(in3, k1) + in1 - in3
        let c1 = _mm_mulhi_epi16(in1, k2);
        let c2 = _mm_mulhi_epi16(in3, k1);
        let c3 = _mm_sub_epi16(in1, in3);
        let c4 = _mm_sub_epi16(c1, c2);
        let c = _mm_add_epi16(c3, c4);
        // d = MUL(in1, K1) + MUL(in3, K2) = MUL(in1, k1) + MUL(in3, k2) + in1 + in3
        let d1 = _mm_mulhi_epi16(in1, k1);
        let d2 = _mm_mulhi_epi16(in3, k2);
        let d3 = _mm_add_epi16(in1, in3);
        let d4 = _mm_add_epi16(d1, d2);
        let d = _mm_add_epi16(d3, d4);

        // Second pass.
        let tmp0 = _mm_add_epi16(a, d);
        let tmp1 = _mm_add_epi16(b, c);
        let tmp2 = _mm_sub_epi16(b, c);
        let tmp3 = _mm_sub_epi16(a, d);

        // Transpose the two 4x4.
        vp8_transpose_2_4x4_16b(
            &tmp0, &tmp1, &tmp2, &tmp3, &mut t0, &mut t1, &mut t2, &mut t3,
        );
    }

    // Horizontal pass and subsequent transpose.
    {
        // First pass, c and d calculations are longer because of the "trick"
        // multiplications.
        let four = _mm_set1_epi16(4);
        let dc = _mm_add_epi16(t0, four);
        let a = _mm_add_epi16(dc, t2);
        let b = _mm_sub_epi16(dc, t2);
        // c = MUL(T1, K2) - MUL(T3, K1) = MUL(T1, k2) - MUL(T3, k1) + T1 - T3
        let c1 = _mm_mulhi_epi16(t1, k2);
        let c2 = _mm_mulhi_epi16(t3, k1);
        let c3 = _mm_sub_epi16(t1, t3);
        let c4 = _mm_sub_epi16(c1, c2);
        let c = _mm_add_epi16(c3, c4);
        // d = MUL(T1, K1) + MUL(T3, K2) = MUL(T1, k1) + MUL(T3, k2) + T1 + T3
        let d1 = _mm_mulhi_epi16(t1, k1);
        let d2 = _mm_mulhi_epi16(t3, k2);
        let d3 = _mm_add_epi16(t1, t3);
        let d4 = _mm_add_epi16(d1, d2);
        let d = _mm_add_epi16(d3, d4);

        // Second pass.
        let tmp0 = _mm_add_epi16(a, d);
        let tmp1 = _mm_add_epi16(b, c);
        let tmp2 = _mm_sub_epi16(b, c);
        let tmp3 = _mm_sub_epi16(a, d);
        let shifted0 = _mm_srai_epi16::<3>(tmp0);
        let shifted1 = _mm_srai_epi16::<3>(tmp1);
        let shifted2 = _mm_srai_epi16::<3>(tmp2);
        let shifted3 = _mm_srai_epi16::<3>(tmp3);

        // Transpose the two 4x4.
        vp8_transpose_2_4x4_16b(
            &shifted0, &shifted1, &shifted2, &shifted3, &mut t0, &mut t1, &mut t2, &mut t3,
        );
    }

    // Add inverse transform to 'dst' and store.
    {
        let zero = _mm_setzero_si128();
        // Load the reference(s).
        let (mut dst0, mut dst1, mut dst2, mut dst3);
        if do_two {
            // Load eight bytes/pixels per line.
            dst0 = loadl_epi64(&dst[0..]);
            dst1 = loadl_epi64(&dst[BPS_U..]);
            dst2 = loadl_epi64(&dst[2 * BPS_U..]);
            dst3 = loadl_epi64(&dst[3 * BPS_U..]);
        } else {
            // Load four bytes/pixels per line.
            dst0 = _mm_cvtsi32_si128(webp_mem_to_int32(&dst[0..]));
            dst1 = _mm_cvtsi32_si128(webp_mem_to_int32(&dst[BPS_U..]));
            dst2 = _mm_cvtsi32_si128(webp_mem_to_int32(&dst[2 * BPS_U..]));
            dst3 = _mm_cvtsi32_si128(webp_mem_to_int32(&dst[3 * BPS_U..]));
        }
        // Convert to 16b.
        dst0 = _mm_unpacklo_epi8(dst0, zero);
        dst1 = _mm_unpacklo_epi8(dst1, zero);
        dst2 = _mm_unpacklo_epi8(dst2, zero);
        dst3 = _mm_unpacklo_epi8(dst3, zero);
        // Add the inverse transform(s).
        dst0 = _mm_add_epi16(dst0, t0);
        dst1 = _mm_add_epi16(dst1, t1);
        dst2 = _mm_add_epi16(dst2, t2);
        dst3 = _mm_add_epi16(dst3, t3);
        // Unsigned saturate to 8b.
        dst0 = _mm_packus_epi16(dst0, dst0);
        dst1 = _mm_packus_epi16(dst1, dst1);
        dst2 = _mm_packus_epi16(dst2, dst2);
        dst3 = _mm_packus_epi16(dst3, dst3);
        // Store the results.
        if do_two {
            // Store eight bytes/pixels per line.
            storel_epi64(&mut dst[0..], dst0);
            storel_epi64(&mut dst[BPS_U..], dst1);
            storel_epi64(&mut dst[2 * BPS_U..], dst2);
            storel_epi64(&mut dst[3 * BPS_U..], dst3);
        } else {
            // Store four bytes/pixels per line.
            webp_int32_to_mem(&mut dst[0..], _mm_cvtsi128_si32(dst0));
            webp_int32_to_mem(&mut dst[BPS_U..], _mm_cvtsi128_si32(dst1));
            webp_int32_to_mem(&mut dst[2 * BPS_U..], _mm_cvtsi128_si32(dst2));
            webp_int32_to_mem(&mut dst[3 * BPS_U..], _mm_cvtsi128_si32(dst3));
        }
    }
}

/// Entry point of [`transform_sse2`] for `vp8_transform` (C: `VP8Transform =
/// Transform_SSE2`).
pub(crate) fn transform(input: &[i16], dst: &mut [u8], do_two: bool) {
    // SAFETY: `vp8_transform` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has the function's only feature.
    unsafe { transform_sse2(input, dst, do_two) }
}

//------------------------------------------------------------------------------
// Loop Filter (Paragraph 15)

// upstream: src/dsp/dec_sse2.c MM_ABS
/// Compute abs(p - q) = subs(p - q) OR subs(q - p)
#[target_feature(enable = "sse2")]
#[inline]
fn mm_abs(p: __m128i, q: __m128i) -> __m128i {
    _mm_or_si128(_mm_subs_epu8(q, p), _mm_subs_epu8(p, q))
}

// upstream: src/dsp/dec_sse2.c SignedShift8b_SSE2
/// Shift each byte of "x" by 3 bits while preserving by the sign bit.
#[target_feature(enable = "sse2")]
#[inline]
fn signed_shift8b_sse2(x: &mut __m128i) {
    let zero = _mm_setzero_si128();
    let lo_0 = _mm_unpacklo_epi8(zero, *x);
    let hi_0 = _mm_unpackhi_epi8(zero, *x);
    let lo_1 = _mm_srai_epi16::<{ 3 + 8 }>(lo_0);
    let hi_1 = _mm_srai_epi16::<{ 3 + 8 }>(hi_0);
    *x = _mm_packs_epi16(lo_1, hi_1);
}

// upstream: src/dsp/dec_sse2.c FLIP_SIGN_BIT2
#[target_feature(enable = "sse2")]
#[inline]
fn flip_sign_bit2(a: &mut __m128i, b: &mut __m128i, sign_bit: __m128i) {
    *a = _mm_xor_si128(*a, sign_bit);
    *b = _mm_xor_si128(*b, sign_bit);
}

// upstream: src/dsp/dec_sse2.c FLIP_SIGN_BIT4
#[target_feature(enable = "sse2")]
#[inline]
fn flip_sign_bit4(
    a: &mut __m128i,
    b: &mut __m128i,
    c: &mut __m128i,
    d: &mut __m128i,
    sign_bit: __m128i,
) {
    flip_sign_bit2(a, b, sign_bit);
    flip_sign_bit2(c, d, sign_bit);
}

/// C's implicit conversion of an `int` threshold to `char` in `_mm_set1_epi8`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the int to char"
)]
fn to_char(v: i32) -> i8 {
    v as i8
}

// upstream: src/dsp/dec_sse2.c GetNotHEV_SSE2
/// input/output is `uint8_t`
#[target_feature(enable = "sse2")]
#[inline]
fn get_not_hev_sse2(
    p1: &__m128i,
    p0: &__m128i,
    q0: &__m128i,
    q1: &__m128i,
    hev_thresh: i32,
    not_hev: &mut __m128i,
) {
    let zero = _mm_setzero_si128();
    let t_1 = mm_abs(*p1, *p0);
    let t_2 = mm_abs(*q1, *q0);

    let h = _mm_set1_epi8(to_char(hev_thresh));
    let t_max = _mm_max_epu8(t_1, t_2);

    let t_max_h = _mm_subs_epu8(t_max, h);
    *not_hev = _mm_cmpeq_epi8(t_max_h, zero); // not_hev <= t1 && not_hev <= t2
}

// upstream: src/dsp/dec_sse2.c GetBaseDelta_SSE2
/// input pixels are `int8_t`
#[target_feature(enable = "sse2")]
#[inline]
fn get_base_delta_sse2(
    p1: &__m128i,
    p0: &__m128i,
    q0: &__m128i,
    q1: &__m128i,
    delta: &mut __m128i,
) {
    // beware of addition order, for saturation!
    let p1_q1 = _mm_subs_epi8(*p1, *q1); // p1 - q1
    let q0_p0 = _mm_subs_epi8(*q0, *p0); // q0 - p0
    let s1 = _mm_adds_epi8(p1_q1, q0_p0); // p1 - q1 + 1 * (q0 - p0)
    let s2 = _mm_adds_epi8(q0_p0, s1); // p1 - q1 + 2 * (q0 - p0)
    let s3 = _mm_adds_epi8(q0_p0, s2); // p1 - q1 + 3 * (q0 - p0)
    *delta = s3;
}

// upstream: src/dsp/dec_sse2.c DoSimpleFilter_SSE2
/// input and output are `int8_t`
#[target_feature(enable = "sse2")]
#[inline]
fn do_simple_filter_sse2(p0: &mut __m128i, q0: &mut __m128i, fl: &__m128i) {
    let k3 = _mm_set1_epi8(3);
    let k4 = _mm_set1_epi8(4);
    let mut v3 = _mm_adds_epi8(*fl, k3);
    let mut v4 = _mm_adds_epi8(*fl, k4);

    signed_shift8b_sse2(&mut v4); // v4 >> 3
    signed_shift8b_sse2(&mut v3); // v3 >> 3
    *q0 = _mm_subs_epi8(*q0, v4); // q0 -= v4
    *p0 = _mm_adds_epi8(*p0, v3); // p0 += v3
}

// upstream: src/dsp/dec_sse2.c Update2Pixels_SSE2
/// Updates values of 2 pixels at MB edge during complex filtering.
/// Update operations:
/// `q = q - delta` and `p = p + delta`; where `delta = [(a_hi >> 7), (a_lo >> 7)]`
/// Pixels 'pi' and 'qi' are `int8_t` on input, `uint8_t` on output (sign flip).
#[target_feature(enable = "sse2")]
#[inline]
fn update2_pixels_sse2(pi: &mut __m128i, qi: &mut __m128i, a0_lo: &__m128i, a0_hi: &__m128i) {
    let a1_lo = _mm_srai_epi16::<7>(*a0_lo);
    let a1_hi = _mm_srai_epi16::<7>(*a0_hi);
    let delta = _mm_packs_epi16(a1_lo, a1_hi);
    let sign_bit = _mm_set1_epi8(0x80_u8.cast_signed());
    *pi = _mm_adds_epi8(*pi, delta);
    *qi = _mm_subs_epi8(*qi, delta);
    flip_sign_bit2(pi, qi, sign_bit);
}

// upstream: src/dsp/dec_sse2.c NeedsFilter_SSE2
/// input pixels are `uint8_t`
#[target_feature(enable = "sse2")]
#[inline]
fn needs_filter_sse2(
    p1: &__m128i,
    p0: &__m128i,
    q0: &__m128i,
    q1: &__m128i,
    thresh: i32,
    mask: &mut __m128i,
) {
    let m_thresh = _mm_set1_epi8(to_char(thresh));
    let t1 = mm_abs(*p1, *q1); // abs(p1 - q1)
    let k_fe = _mm_set1_epi8(0xFE_u8.cast_signed());
    let t2 = _mm_and_si128(t1, k_fe); // set lsb of each byte to zero
    let t3 = _mm_srli_epi16::<1>(t2); // abs(p1 - q1) / 2

    let t4 = mm_abs(*p0, *q0); // abs(p0 - q0)
    let t5 = _mm_adds_epu8(t4, t4); // abs(p0 - q0) * 2
    let t6 = _mm_adds_epu8(t5, t3); // abs(p0-q0)*2 + abs(p1-q1)/2

    let t7 = _mm_subs_epu8(t6, m_thresh); // mask <= m_thresh
    *mask = _mm_cmpeq_epi8(t7, _mm_setzero_si128());
}

//------------------------------------------------------------------------------
// Edge filtering functions

// upstream: src/dsp/dec_sse2.c DoFilter2_SSE2
/// Applies filter on 2 pixels (p0 and q0)
#[target_feature(enable = "sse2")]
#[inline]
fn do_filter2_sse2(
    p1: &mut __m128i,
    p0: &mut __m128i,
    q0: &mut __m128i,
    q1: &mut __m128i,
    thresh: i32,
) {
    let mut a = _mm_setzero_si128();
    let mut mask = _mm_setzero_si128();
    let sign_bit = _mm_set1_epi8(0x80_u8.cast_signed());
    // convert p1/q1 to int8_t (for GetBaseDelta_SSE2)
    let p1s = _mm_xor_si128(*p1, sign_bit);
    let q1s = _mm_xor_si128(*q1, sign_bit);

    needs_filter_sse2(p1, p0, q0, q1, thresh, &mut mask);

    flip_sign_bit2(p0, q0, sign_bit);
    get_base_delta_sse2(&p1s, p0, q0, &q1s, &mut a);
    a = _mm_and_si128(a, mask); // mask filter values we don't care about
    do_simple_filter_sse2(p0, q0, &a);
    flip_sign_bit2(p0, q0, sign_bit);
}

// upstream: src/dsp/dec_sse2.c DoFilter4_SSE2
/// Applies filter on 4 pixels (p1, p0, q0 and q1)
#[target_feature(enable = "sse2")]
#[inline]
fn do_filter4_sse2(
    p1: &mut __m128i,
    p0: &mut __m128i,
    q0: &mut __m128i,
    q1: &mut __m128i,
    mask: &__m128i,
    hev_thresh: i32,
) {
    let zero = _mm_setzero_si128();
    let sign_bit = _mm_set1_epi8(0x80_u8.cast_signed());
    let k64 = _mm_set1_epi8(64);
    let k3 = _mm_set1_epi8(3);
    let k4 = _mm_set1_epi8(4);
    let mut not_hev = _mm_setzero_si128();

    // compute hev mask
    get_not_hev_sse2(p1, p0, q0, q1, hev_thresh, &mut not_hev);

    // convert to signed values
    flip_sign_bit4(p1, p0, q0, q1, sign_bit);

    let mut t1 = _mm_subs_epi8(*p1, *q1); // p1 - q1
    t1 = _mm_andnot_si128(not_hev, t1); // hev(p1 - q1)
    let mut t2 = _mm_subs_epi8(*q0, *p0); // q0 - p0
    t1 = _mm_adds_epi8(t1, t2); // hev(p1 - q1) + 1 * (q0 - p0)
    t1 = _mm_adds_epi8(t1, t2); // hev(p1 - q1) + 2 * (q0 - p0)
    t1 = _mm_adds_epi8(t1, t2); // hev(p1 - q1) + 3 * (q0 - p0)
    t1 = _mm_and_si128(t1, *mask); // mask filter values we don't care about

    t2 = _mm_adds_epi8(t1, k3); // 3 * (q0 - p0) + hev(p1 - q1) + 3
    let mut t3 = _mm_adds_epi8(t1, k4); // 3 * (q0 - p0) + hev(p1 - q1) + 4
    signed_shift8b_sse2(&mut t2); // (3 * (q0 - p0) + hev(p1 - q1) + 3) >> 3
    signed_shift8b_sse2(&mut t3); // (3 * (q0 - p0) + hev(p1 - q1) + 4) >> 3
    *p0 = _mm_adds_epi8(*p0, t2); // p0 += t2
    *q0 = _mm_subs_epi8(*q0, t3); // q0 -= t3
    flip_sign_bit2(p0, q0, sign_bit);

    // this is equivalent to signed (a + 1) >> 1 calculation
    t2 = _mm_add_epi8(t3, sign_bit);
    t3 = _mm_avg_epu8(t2, zero);
    t3 = _mm_sub_epi8(t3, k64);

    t3 = _mm_and_si128(not_hev, t3); // if !hev
    *q1 = _mm_subs_epi8(*q1, t3); // q1 -= t3
    *p1 = _mm_adds_epi8(*p1, t3); // p1 += t3
    flip_sign_bit2(p1, q1, sign_bit);
}

// upstream: src/dsp/dec_sse2.c DoFilter6_SSE2
/// Applies filter on 6 pixels (p2, p1, p0, q0, q1 and q2)
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
#[expect(clippy::similar_names, reason = "the C names f_lo and f9_lo")]
fn do_filter6_sse2(
    p2: &mut __m128i,
    p1: &mut __m128i,
    p0: &mut __m128i,
    q0: &mut __m128i,
    q1: &mut __m128i,
    q2: &mut __m128i,
    mask: &__m128i,
    hev_thresh: i32,
) {
    let zero = _mm_setzero_si128();
    let sign_bit = _mm_set1_epi8(0x80_u8.cast_signed());
    let mut a = _mm_setzero_si128();
    let mut not_hev = _mm_setzero_si128();

    // compute hev mask
    get_not_hev_sse2(p1, p0, q0, q1, hev_thresh, &mut not_hev);

    flip_sign_bit4(p1, p0, q0, q1, sign_bit);
    flip_sign_bit2(p2, q2, sign_bit);
    get_base_delta_sse2(p1, p0, q0, q1, &mut a);

    {
        // do simple filter on pixels with hev
        let m = _mm_andnot_si128(not_hev, *mask);
        let f = _mm_and_si128(a, m);
        do_simple_filter_sse2(p0, q0, &f);
    }

    {
        // do strong filter on pixels with not hev
        let k9 = _mm_set1_epi16(0x0900);
        let k63 = _mm_set1_epi16(63);

        let m = _mm_and_si128(not_hev, *mask);
        let f = _mm_and_si128(a, m);

        let f_lo = _mm_unpacklo_epi8(zero, f);
        let f_hi = _mm_unpackhi_epi8(zero, f);

        let f9_lo = _mm_mulhi_epi16(f_lo, k9); // Filter (lo) * 9
        let f9_hi = _mm_mulhi_epi16(f_hi, k9); // Filter (hi) * 9

        let a2_lo = _mm_add_epi16(f9_lo, k63); // Filter * 9 + 63
        let a2_hi = _mm_add_epi16(f9_hi, k63); // Filter * 9 + 63

        let a1_lo = _mm_add_epi16(a2_lo, f9_lo); // Filter * 18 + 63
        let a1_hi = _mm_add_epi16(a2_hi, f9_hi); // Filter * 18 + 63

        let a0_lo = _mm_add_epi16(a1_lo, f9_lo); // Filter * 27 + 63
        let a0_hi = _mm_add_epi16(a1_hi, f9_hi); // Filter * 27 + 63

        update2_pixels_sse2(p2, q2, &a2_lo, &a2_hi);
        update2_pixels_sse2(p1, q1, &a1_lo, &a1_hi);
        update2_pixels_sse2(p0, q0, &a0_lo, &a0_hi);
    }
}

// upstream: src/dsp/dec_sse2.c Load8x4_SSE2
/// reads 8 rows across a vertical edge.
#[target_feature(enable = "sse2")]
#[inline]
fn load8x4_sse2(b: &[u8], b_pos: usize, stride: i32, p: &mut __m128i, q: &mut __m128i) {
    let row = |k: i32| webp_mem_to_int32(&b[at(b_pos, k * stride)..]);
    // A0 = 63 62 61 60 23 22 21 20 43 42 41 40 03 02 01 00
    // A1 = 73 72 71 70 33 32 31 30 53 52 51 50 13 12 11 10
    let a0 = _mm_set_epi32(row(6), row(2), row(4), row(0));
    let a1 = _mm_set_epi32(row(7), row(3), row(5), row(1));

    // B0 = 53 43 52 42 51 41 50 40 13 03 12 02 11 01 10 00
    // B1 = 73 63 72 62 71 61 70 60 33 23 32 22 31 21 30 20
    let b0 = _mm_unpacklo_epi8(a0, a1);
    let b1 = _mm_unpackhi_epi8(a0, a1);

    // C0 = 33 23 13 03 32 22 12 02 31 21 11 01 30 20 10 00
    // C1 = 73 63 53 43 72 62 52 42 71 61 51 41 70 60 50 40
    let c0 = _mm_unpacklo_epi16(b0, b1);
    let c1 = _mm_unpackhi_epi16(b0, b1);

    // *p = 71 61 51 41 31 21 11 01 70 60 50 40 30 20 10 00
    // *q = 73 63 53 43 33 23 13 03 72 62 52 42 32 22 12 02
    *p = _mm_unpacklo_epi32(c0, c1);
    *q = _mm_unpackhi_epi32(c0, c1);
}

// upstream: src/dsp/dec_sse2.c Load16x4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn load16x4_sse2(
    r0: &[u8],
    r0_pos: usize,
    r8: &[u8],
    r8_pos: usize,
    stride: i32,
    p1: &mut __m128i,
    p0: &mut __m128i,
    q0: &mut __m128i,
    q1: &mut __m128i,
) {
    // Assume the pixels around the edge (|) are numbered as follows
    //                00 01 | 02 03
    //                10 11 | 12 13
    //                 ...  |  ...
    //                e0 e1 | e2 e3
    //                f0 f1 | f2 f3
    //
    // r0 is pointing to the 0th row (00)
    // r8 is pointing to the 8th row (80)

    // Load
    // p1 = 71 61 51 41 31 21 11 01 70 60 50 40 30 20 10 00
    // q0 = 73 63 53 43 33 23 13 03 72 62 52 42 32 22 12 02
    // p0 = f1 e1 d1 c1 b1 a1 91 81 f0 e0 d0 c0 b0 a0 90 80
    // q1 = f3 e3 d3 c3 b3 a3 93 83 f2 e2 d2 c2 b2 a2 92 82
    load8x4_sse2(r0, r0_pos, stride, p1, q0);
    load8x4_sse2(r8, r8_pos, stride, p0, q1);

    {
        // p1 = f0 e0 d0 c0 b0 a0 90 80 70 60 50 40 30 20 10 00
        // p0 = f1 e1 d1 c1 b1 a1 91 81 71 61 51 41 31 21 11 01
        // q0 = f2 e2 d2 c2 b2 a2 92 82 72 62 52 42 32 22 12 02
        // q1 = f3 e3 d3 c3 b3 a3 93 83 73 63 53 43 33 23 13 03
        let t1 = *p1;
        let t2 = *q0;
        *p1 = _mm_unpacklo_epi64(t1, *p0);
        *p0 = _mm_unpackhi_epi64(t1, *p0);
        *q0 = _mm_unpacklo_epi64(t2, *q1);
        *q1 = _mm_unpackhi_epi64(t2, *q1);
    }
}

// upstream: src/dsp/dec_sse2.c Store4x4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn store4x4_sse2(x: &mut __m128i, dst: &mut [u8], dst_pos: usize, stride: i32) {
    let mut dst_pos = dst_pos;
    for i in 0..4 {
        if i > 0 {
            dst_pos = at(dst_pos, stride);
        }
        webp_int32_to_mem(&mut dst[dst_pos..], _mm_cvtsi128_si32(*x));
        *x = _mm_srli_si128::<4>(*x);
    }
}

// upstream: src/dsp/dec_sse2.c Store16x4_SSE2
/// Transpose back and store. `r8` is `None` when its rows lie in `r0`'s plane
/// at `r8_pos` (deviation 2).
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::too_many_arguments, reason = "the C parameters")]
fn store16x4_sse2(
    p1: &__m128i,
    p0: &__m128i,
    q0: &__m128i,
    q1: &__m128i,
    r0: &mut [u8],
    r0_pos: usize,
    r8: Option<&mut [u8]>,
    r8_pos: usize,
    stride: i32,
) {
    // p0 = 71 70 61 60 51 50 41 40 31 30 21 20 11 10 01 00
    // p1 = f1 f0 e1 e0 d1 d0 c1 c0 b1 b0 a1 a0 91 90 81 80
    let mut t1 = *p0;
    let mut p0_s = _mm_unpacklo_epi8(*p1, t1);
    let mut p1_s = _mm_unpackhi_epi8(*p1, t1);

    // q0 = 73 72 63 62 53 52 43 42 33 32 23 22 13 12 03 02
    // q1 = f3 f2 e3 e2 d3 d2 c3 c2 b3 b2 a3 a2 93 92 83 82
    t1 = *q0;
    let mut q0_s = _mm_unpacklo_epi8(t1, *q1);
    let mut q1_s = _mm_unpackhi_epi8(t1, *q1);

    // p0 = 33 32 31 30 23 22 21 20 13 12 11 10 03 02 01 00
    // q0 = 73 72 71 70 63 62 61 60 53 52 51 50 43 42 41 40
    t1 = p0_s;
    p0_s = _mm_unpacklo_epi16(t1, q0_s);
    q0_s = _mm_unpackhi_epi16(t1, q0_s);

    // p1 = b3 b2 b1 b0 a3 a2 a1 a0 93 92 91 90 83 82 81 80
    // q1 = f3 f2 f1 f0 e3 e2 e1 e0 d3 d2 d1 d0 c3 c2 c1 c0
    t1 = p1_s;
    p1_s = _mm_unpacklo_epi16(t1, q1_s);
    q1_s = _mm_unpackhi_epi16(t1, q1_s);

    store4x4_sse2(&mut p0_s, r0, r0_pos, stride);
    let r0_pos = at(r0_pos, 4 * stride);
    store4x4_sse2(&mut q0_s, r0, r0_pos, stride);

    let r8 = match r8 {
        Some(plane) => plane,
        None => r0,
    };
    store4x4_sse2(&mut p1_s, r8, r8_pos, stride);
    let r8_pos = at(r8_pos, 4 * stride);
    store4x4_sse2(&mut q1_s, r8, r8_pos, stride);
}

//------------------------------------------------------------------------------
// Simple In-loop filtering (Paragraph 15.2)

// upstream: src/dsp/dec_sse2.c SimpleVFilter16_SSE2
#[target_feature(enable = "sse2")]
fn simple_v_filter16_sse2(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // Load
    let mut p1 = loadu_si128(&p[at(pos, -2 * stride)..]);
    let mut p0 = loadu_si128(&p[at(pos, -stride)..]);
    let mut q0 = loadu_si128(&p[pos..]);
    let mut q1 = loadu_si128(&p[at(pos, stride)..]);

    do_filter2_sse2(&mut p1, &mut p0, &mut q0, &mut q1, thresh);

    // Store
    storeu_si128(&mut p[at(pos, -stride)..], p0);
    storeu_si128(&mut p[pos..], q0);
}

/// Entry point of [`simple_v_filter16_sse2`] for `vp8_simple_v_filter16`.
pub(crate) fn simple_v_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: `vp8_simple_v_filter16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { simple_v_filter16_sse2(p, pos, stride, thresh) }
}

// upstream: src/dsp/dec_sse2.c SimpleHFilter16_SSE2
#[target_feature(enable = "sse2")]
fn simple_h_filter16_sse2(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut p1 = _mm_setzero_si128();
    let mut p0 = _mm_setzero_si128();
    let mut q0 = _mm_setzero_si128();
    let mut q1 = _mm_setzero_si128();

    let pos = pos - 2; // beginning of p1

    let r8_pos = at(pos, 8 * stride);
    load16x4_sse2(
        p, pos, p, r8_pos, stride, &mut p1, &mut p0, &mut q0, &mut q1,
    );
    do_filter2_sse2(&mut p1, &mut p0, &mut q0, &mut q1, thresh);
    store16x4_sse2(&p1, &p0, &q0, &q1, p, pos, None, r8_pos, stride);
}

/// Entry point of [`simple_h_filter16_sse2`] for `vp8_simple_h_filter16`.
pub(crate) fn simple_h_filter16(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: `vp8_simple_h_filter16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { simple_h_filter16_sse2(p, pos, stride, thresh) }
}

// upstream: src/dsp/dec_sse2.c SimpleVFilter16i_SSE2
#[target_feature(enable = "sse2")]
fn simple_v_filter16i_sse2(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut pos = pos;
    for _ in 0..3 {
        pos = at(pos, 4 * stride);
        simple_v_filter16_sse2(p, pos, stride, thresh);
    }
}

/// Entry point of [`simple_v_filter16i_sse2`] for `vp8_simple_v_filter16i`.
pub(crate) fn simple_v_filter16i(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: `vp8_simple_v_filter16i` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { simple_v_filter16i_sse2(p, pos, stride, thresh) }
}

// upstream: src/dsp/dec_sse2.c SimpleHFilter16i_SSE2
#[target_feature(enable = "sse2")]
fn simple_h_filter16i_sse2(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    let mut pos = pos;
    for _ in 0..3 {
        pos += 4;
        simple_h_filter16_sse2(p, pos, stride, thresh);
    }
}

/// Entry point of [`simple_h_filter16i_sse2`] for `vp8_simple_h_filter16i`.
pub(crate) fn simple_h_filter16i(p: &mut [u8], pos: usize, stride: i32, thresh: i32) {
    // SAFETY: `vp8_simple_h_filter16i` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { simple_h_filter16i_sse2(p, pos, stride, thresh) }
}

//------------------------------------------------------------------------------
// Complex In-loop filtering (Paragraph 15.3)

// upstream: src/dsp/dec_sse2.c MAX_DIFF1
#[target_feature(enable = "sse2")]
#[inline]
fn max_diff1(p3: __m128i, p2: __m128i, p1: __m128i, p0: __m128i, m: &mut __m128i) {
    *m = mm_abs(p1, p0);
    *m = _mm_max_epu8(*m, mm_abs(p3, p2));
    *m = _mm_max_epu8(*m, mm_abs(p2, p1));
}

// upstream: src/dsp/dec_sse2.c MAX_DIFF2
#[target_feature(enable = "sse2")]
#[inline]
fn max_diff2(p3: __m128i, p2: __m128i, p1: __m128i, p0: __m128i, m: &mut __m128i) {
    *m = _mm_max_epu8(*m, mm_abs(p1, p0));
    *m = _mm_max_epu8(*m, mm_abs(p3, p2));
    *m = _mm_max_epu8(*m, mm_abs(p2, p1));
}

// upstream: src/dsp/dec_sse2.c LOAD_H_EDGES4
#[target_feature(enable = "sse2")]
#[inline]
fn load_h_edges4(p: &[u8], pos: usize, stride: i32) -> [__m128i; 4] {
    [
        loadu_si128(&p[pos..]),
        loadu_si128(&p[at(pos, stride)..]),
        loadu_si128(&p[at(pos, 2 * stride)..]),
        loadu_si128(&p[at(pos, 3 * stride)..]),
    ]
}

// upstream: src/dsp/dec_sse2.c LOADUV_H_EDGE
#[target_feature(enable = "sse2")]
#[inline]
fn loaduv_h_edge(u: &[u8], u_pos: usize, v: &[u8], v_pos: usize, stride: i32) -> __m128i {
    let u_edge = loadl_epi64(&u[at(u_pos, stride)..]);
    let v_edge = loadl_epi64(&v[at(v_pos, stride)..]);
    _mm_unpacklo_epi64(u_edge, v_edge)
}

// upstream: src/dsp/dec_sse2.c LOADUV_H_EDGES4
#[target_feature(enable = "sse2")]
#[inline]
fn loaduv_h_edges4(u: &[u8], u_pos: usize, v: &[u8], v_pos: usize, stride: i32) -> [__m128i; 4] {
    [
        loaduv_h_edge(u, u_pos, v, v_pos, 0),
        loaduv_h_edge(u, u_pos, v, v_pos, stride),
        loaduv_h_edge(u, u_pos, v, v_pos, 2 * stride),
        loaduv_h_edge(u, u_pos, v, v_pos, 3 * stride),
    ]
}

// upstream: src/dsp/dec_sse2.c STOREUV
#[target_feature(enable = "sse2")]
#[inline]
fn storeuv(p: &mut __m128i, u: &mut [u8], u_pos: usize, v: &mut [u8], v_pos: usize, stride: i32) {
    storel_epi64(&mut u[at(u_pos, stride)..], *p);
    *p = _mm_srli_si128::<8>(*p);
    storel_epi64(&mut v[at(v_pos, stride)..], *p);
}

// upstream: src/dsp/dec_sse2.c ComplexMask_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn complex_mask_sse2(
    p1: &__m128i,
    p0: &__m128i,
    q0: &__m128i,
    q1: &__m128i,
    thresh: i32,
    ithresh: i32,
    mask: &mut __m128i,
) {
    let it = _mm_set1_epi8(to_char(ithresh));
    let diff = _mm_subs_epu8(*mask, it);
    let thresh_mask = _mm_cmpeq_epi8(diff, _mm_setzero_si128());
    let mut filter_mask = _mm_setzero_si128();
    needs_filter_sse2(p1, p0, q0, q1, thresh, &mut filter_mask);
    *mask = _mm_and_si128(thresh_mask, filter_mask);
}

// upstream: src/dsp/dec_sse2.c VFilter16_SSE2
/// on macroblock edges
#[target_feature(enable = "sse2")]
fn v_filter16_sse2(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut mask = _mm_setzero_si128();

    // Load p3, p2, p1, p0
    let [t1, mut p2, mut p1, mut p0] = load_h_edges4(p, at(pos, -4 * stride), stride);
    max_diff1(t1, p2, p1, p0, &mut mask);

    // Load q0, q1, q2, q3
    let [mut q0, mut q1, mut q2, t1] = load_h_edges4(p, pos, stride);
    max_diff2(t1, q2, q1, q0, &mut mask);

    complex_mask_sse2(&p1, &p0, &q0, &q1, thresh, ithresh, &mut mask);
    do_filter6_sse2(
        &mut p2, &mut p1, &mut p0, &mut q0, &mut q1, &mut q2, &mask, hev_thresh,
    );

    // Store
    storeu_si128(&mut p[at(pos, -3 * stride)..], p2);
    storeu_si128(&mut p[at(pos, -2 * stride)..], p1);
    storeu_si128(&mut p[at(pos, -stride)..], p0);
    storeu_si128(&mut p[pos..], q0);
    storeu_si128(&mut p[at(pos, stride)..], q1);
    storeu_si128(&mut p[at(pos, 2 * stride)..], q2);
}

/// Entry point of [`v_filter16_sse2`] for `vp8v_filter16`.
pub(crate) fn v_filter16(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8v_filter16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { v_filter16_sse2(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c HFilter16_SSE2
#[target_feature(enable = "sse2")]
fn h_filter16_sse2(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut mask = _mm_setzero_si128();
    let mut p3 = _mm_setzero_si128();
    let mut p2 = _mm_setzero_si128();
    let mut p1 = _mm_setzero_si128();
    let mut p0 = _mm_setzero_si128();
    let mut q0 = _mm_setzero_si128();
    let mut q1 = _mm_setzero_si128();
    let mut q2 = _mm_setzero_si128();
    let mut q3 = _mm_setzero_si128();

    let b = pos - 4;
    let b8 = at(b, 8 * stride);
    load16x4_sse2(p, b, p, b8, stride, &mut p3, &mut p2, &mut p1, &mut p0);
    max_diff1(p3, p2, p1, p0, &mut mask);

    let p8 = at(pos, 8 * stride);
    load16x4_sse2(p, pos, p, p8, stride, &mut q0, &mut q1, &mut q2, &mut q3);
    max_diff2(q3, q2, q1, q0, &mut mask);

    complex_mask_sse2(&p1, &p0, &q0, &q1, thresh, ithresh, &mut mask);
    do_filter6_sse2(
        &mut p2, &mut p1, &mut p0, &mut q0, &mut q1, &mut q2, &mask, hev_thresh,
    );

    store16x4_sse2(&p3, &p2, &p1, &p0, p, b, None, b8, stride);
    store16x4_sse2(&q0, &q1, &q2, &q3, p, pos, None, p8, stride);
}

/// Entry point of [`h_filter16_sse2`] for `vp8h_filter16`.
pub(crate) fn h_filter16(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8h_filter16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { h_filter16_sse2(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c VFilter16i_SSE2
/// on three inner edges
#[target_feature(enable = "sse2")]
fn v_filter16i_sse2(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut pos = pos;
    // loop invariants
    let [mut p3, mut p2, mut p1, mut p0] = load_h_edges4(p, pos, stride); // prologue

    for _ in 0..3 {
        let mut mask = _mm_setzero_si128();
        let b = at(pos, 2 * stride); // beginning of p1
        pos = at(pos, 4 * stride);

        max_diff1(p3, p2, p1, p0, &mut mask); // compute partial mask
        let [q0, q1, tmp1, tmp2] = load_h_edges4(p, pos, stride);
        p3 = q0;
        p2 = q1;
        max_diff2(p3, p2, tmp1, tmp2, &mut mask);

        // p3 and p2 are not just temporary variables here: they will be
        // re-used for next span. And q2/q3 will become p1/p0 accordingly.
        complex_mask_sse2(&p1, &p0, &p3, &p2, thresh, ithresh, &mut mask);
        do_filter4_sse2(&mut p1, &mut p0, &mut p3, &mut p2, &mask, hev_thresh);

        // Store
        storeu_si128(&mut p[b..], p1);
        storeu_si128(&mut p[at(b, stride)..], p0);
        storeu_si128(&mut p[at(b, 2 * stride)..], p3);
        storeu_si128(&mut p[at(b, 3 * stride)..], p2);

        // rotate samples
        p1 = tmp1;
        p0 = tmp2;
    }
}

/// Entry point of [`v_filter16i_sse2`] for `vp8v_filter16i`.
pub(crate) fn v_filter16i(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8v_filter16i` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { v_filter16i_sse2(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c HFilter16i_SSE2
#[target_feature(enable = "sse2")]
fn h_filter16i_sse2(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut pos = pos;
    // loop invariants
    let mut p3 = _mm_setzero_si128();
    let mut p2 = _mm_setzero_si128();
    let mut p1 = _mm_setzero_si128();
    let mut p0 = _mm_setzero_si128();

    load16x4_sse2(
        p,
        pos,
        p,
        at(pos, 8 * stride),
        stride,
        &mut p3,
        &mut p2,
        &mut p1,
        &mut p0,
    ); // prologue

    for _ in 0..3 {
        let mut mask = _mm_setzero_si128();
        let mut tmp1 = _mm_setzero_si128();
        let mut tmp2 = _mm_setzero_si128();
        let b = pos + 2; // beginning of p1

        pos += 4; // beginning of q0 (and next span)

        max_diff1(p3, p2, p1, p0, &mut mask); // compute partial mask
        load16x4_sse2(
            p,
            pos,
            p,
            at(pos, 8 * stride),
            stride,
            &mut p3,
            &mut p2,
            &mut tmp1,
            &mut tmp2,
        );
        max_diff2(p3, p2, tmp1, tmp2, &mut mask);

        complex_mask_sse2(&p1, &p0, &p3, &p2, thresh, ithresh, &mut mask);
        do_filter4_sse2(&mut p1, &mut p0, &mut p3, &mut p2, &mask, hev_thresh);

        let b8 = at(b, 8 * stride);
        store16x4_sse2(&p1, &p0, &p3, &p2, p, b, None, b8, stride);

        // rotate samples
        p1 = tmp1;
        p0 = tmp2;
    }
}

/// Entry point of [`h_filter16i_sse2`] for `vp8h_filter16i`.
pub(crate) fn h_filter16i(
    p: &mut [u8],
    pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8h_filter16i` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { h_filter16i_sse2(p, pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c VFilter8_SSE2
/// 8-pixels wide variant, for chroma filtering
#[target_feature(enable = "sse2")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn v_filter8_sse2(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut mask = _mm_setzero_si128();

    // Load p3, p2, p1, p0
    let [t1, mut p2, mut p1, mut p0] =
        loaduv_h_edges4(u, at(u_pos, -4 * stride), v, at(v_pos, -4 * stride), stride);
    max_diff1(t1, p2, p1, p0, &mut mask);

    // Load q0, q1, q2, q3
    let [mut q0, mut q1, mut q2, t1] = loaduv_h_edges4(u, u_pos, v, v_pos, stride);
    max_diff2(t1, q2, q1, q0, &mut mask);

    complex_mask_sse2(&p1, &p0, &q0, &q1, thresh, ithresh, &mut mask);
    do_filter6_sse2(
        &mut p2, &mut p1, &mut p0, &mut q0, &mut q1, &mut q2, &mask, hev_thresh,
    );

    // Store
    storeuv(&mut p2, u, u_pos, v, v_pos, -3 * stride);
    storeuv(&mut p1, u, u_pos, v, v_pos, -2 * stride);
    storeuv(&mut p0, u, u_pos, v, v_pos, -stride);
    storeuv(&mut q0, u, u_pos, v, v_pos, 0);
    storeuv(&mut q1, u, u_pos, v, v_pos, stride);
    storeuv(&mut q2, u, u_pos, v, v_pos, 2 * stride);
}

/// Entry point of [`v_filter8_sse2`] for `vp8v_filter8`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub(crate) fn v_filter8(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8v_filter8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { v_filter8_sse2(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c HFilter8_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn h_filter8_sse2(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut mask = _mm_setzero_si128();
    let mut p3 = _mm_setzero_si128();
    let mut p2 = _mm_setzero_si128();
    let mut p1 = _mm_setzero_si128();
    let mut p0 = _mm_setzero_si128();
    let mut q0 = _mm_setzero_si128();
    let mut q1 = _mm_setzero_si128();
    let mut q2 = _mm_setzero_si128();
    let mut q3 = _mm_setzero_si128();

    let tu = u_pos - 4;
    let tv = v_pos - 4;
    load16x4_sse2(u, tu, v, tv, stride, &mut p3, &mut p2, &mut p1, &mut p0);
    max_diff1(p3, p2, p1, p0, &mut mask);

    load16x4_sse2(
        u, u_pos, v, v_pos, stride, &mut q0, &mut q1, &mut q2, &mut q3,
    );
    max_diff2(q3, q2, q1, q0, &mut mask);

    complex_mask_sse2(&p1, &p0, &q0, &q1, thresh, ithresh, &mut mask);
    do_filter6_sse2(
        &mut p2, &mut p1, &mut p0, &mut q0, &mut q1, &mut q2, &mask, hev_thresh,
    );

    store16x4_sse2(&p3, &p2, &p1, &p0, u, tu, Some(&mut *v), tv, stride);
    store16x4_sse2(&q0, &q1, &q2, &q3, u, u_pos, Some(v), v_pos, stride);
}

/// Entry point of [`h_filter8_sse2`] for `vp8h_filter8`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub(crate) fn h_filter8(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8h_filter8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { h_filter8_sse2(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c VFilter8i_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn v_filter8i_sse2(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut mask = _mm_setzero_si128();

    // Load p3, p2, p1, p0
    let [t2, t1, mut p1, mut p0] = loaduv_h_edges4(u, u_pos, v, v_pos, stride);
    max_diff1(t2, t1, p1, p0, &mut mask);

    let u_pos = at(u_pos, 4 * stride);
    let v_pos = at(v_pos, 4 * stride);

    // Load q0, q1, q2, q3
    let [mut q0, mut q1, t1, t2] = loaduv_h_edges4(u, u_pos, v, v_pos, stride);
    max_diff2(t2, t1, q1, q0, &mut mask);

    complex_mask_sse2(&p1, &p0, &q0, &q1, thresh, ithresh, &mut mask);
    do_filter4_sse2(&mut p1, &mut p0, &mut q0, &mut q1, &mask, hev_thresh);

    // Store
    storeuv(&mut p1, u, u_pos, v, v_pos, -2 * stride);
    storeuv(&mut p0, u, u_pos, v, v_pos, -stride);
    storeuv(&mut q0, u, u_pos, v, v_pos, 0);
    storeuv(&mut q1, u, u_pos, v, v_pos, stride);
}

/// Entry point of [`v_filter8i_sse2`] for `vp8v_filter8i`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub(crate) fn v_filter8i(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8v_filter8i` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { v_filter8i_sse2(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

// upstream: src/dsp/dec_sse2.c HFilter8i_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
fn h_filter8i_sse2(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    let mut mask = _mm_setzero_si128();
    let mut t1 = _mm_setzero_si128();
    let mut t2 = _mm_setzero_si128();
    let mut p1 = _mm_setzero_si128();
    let mut p0 = _mm_setzero_si128();
    let mut q0 = _mm_setzero_si128();
    let mut q1 = _mm_setzero_si128();
    load16x4_sse2(
        u, u_pos, v, v_pos, stride, &mut t2, &mut t1, &mut p1, &mut p0,
    ); // p3, p2, p1, p0
    max_diff1(t2, t1, p1, p0, &mut mask);

    let u_pos = u_pos + 4; // beginning of q0
    let v_pos = v_pos + 4;
    load16x4_sse2(
        u, u_pos, v, v_pos, stride, &mut q0, &mut q1, &mut t1, &mut t2,
    ); // q0, q1, q2, q3
    max_diff2(t2, t1, q1, q0, &mut mask);

    complex_mask_sse2(&p1, &p0, &q0, &q1, thresh, ithresh, &mut mask);
    do_filter4_sse2(&mut p1, &mut p0, &mut q0, &mut q1, &mask, hev_thresh);

    let u_pos = u_pos - 2; // beginning of p1
    let v_pos = v_pos - 2;
    store16x4_sse2(&p1, &p0, &q0, &q1, u, u_pos, Some(v), v_pos, stride);
}

/// Entry point of [`h_filter8i_sse2`] for `vp8h_filter8i`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with two positions"
)]
pub(crate) fn h_filter8i(
    u: &mut [u8],
    u_pos: usize,
    v: &mut [u8],
    v_pos: usize,
    stride: i32,
    thresh: i32,
    ithresh: i32,
    hev_thresh: i32,
) {
    // SAFETY: `vp8h_filter8i` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { h_filter8i_sse2(u, u_pos, v, v_pos, stride, thresh, ithresh, hev_thresh) }
}

//------------------------------------------------------------------------------
// 4x4 predictions

// upstream: src/dsp/dec_sse2.c DST
/// The position of `DST(x, y)`, `dst[(x) + (y) * BPS]`.
const fn dst_at(dst: usize, x: usize, y: usize) -> usize {
    dst + x + y * BPS_U
}

// upstream: src/dsp/dec_sse2.c AVG3
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the int average in uint8_t"
)]
fn avg3(a: i32, b: i32, c: i32) -> u8 {
    ((a + 2 * b + c + 2) >> 2) as u8
}

// We use the following 8b-arithmetic tricks:
//     (a + 2 * b + c + 2) >> 2 = (AC + b + 1) >> 1
//   where: AC = (a + c) >> 1 = [(a + c + 1) >> 1] - [(a^c) & 1]
// and:
//     (a + 2 * b + c + 2) >> 2 = (AB + BC + 1) >> 1 - (ab|bc)&lsb
//   where: AC = (a + b + 1) >> 1,   BC = (b + c + 1) >> 1
//   and ab = a ^ b, bc = b ^ c, lsb = (AC^BC)&1

// upstream: src/dsp/dec_sse2.c VE4_SSE2
/// vertical
#[target_feature(enable = "sse2")]
fn ve4_sse2(buf: &mut [u8], dst: usize) {
    let one = _mm_set1_epi8(1);
    let abcdefgh = loadl_epi64(&buf[dst - BPS_U - 1..]);
    let bcdefgh0 = _mm_srli_si128::<1>(abcdefgh);
    let cdefgh00 = _mm_srli_si128::<2>(abcdefgh);
    let a = _mm_avg_epu8(abcdefgh, cdefgh00);
    let lsb = _mm_and_si128(_mm_xor_si128(abcdefgh, cdefgh00), one);
    let b = _mm_subs_epu8(a, lsb);
    let avg = _mm_avg_epu8(b, bcdefgh0);
    let vals = _mm_cvtsi128_si32(avg);
    for i in 0..4 {
        webp_int32_to_mem(&mut buf[dst + i * BPS_U..], vals);
    }
}

/// Entry point of [`ve4_sse2`] for `vp8_pred_luma4` mode 2.
pub(crate) fn ve4(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma4` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { ve4_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c LD4_SSE2
/// Down-Left
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C names CDEFGH00 and CDEFGHH0")]
fn ld4_sse2(buf: &mut [u8], dst: usize) {
    let one = _mm_set1_epi8(1);
    let abcdefgh = loadl_epi64(&buf[dst - BPS_U..]);
    let bcdefgh0 = _mm_srli_si128::<1>(abcdefgh);
    let cdefgh00 = _mm_srli_si128::<2>(abcdefgh);
    let cdefghh0 = _mm_insert_epi16::<3>(cdefgh00, i32::from(buf[dst - BPS_U + 7]));
    let avg1 = _mm_avg_epu8(abcdefgh, cdefghh0);
    let lsb = _mm_and_si128(_mm_xor_si128(abcdefgh, cdefghh0), one);
    let avg2 = _mm_subs_epu8(avg1, lsb);
    let abcdefg = _mm_avg_epu8(avg2, bcdefgh0);
    webp_int32_to_mem(&mut buf[dst..], _mm_cvtsi128_si32(abcdefg));
    webp_int32_to_mem(
        &mut buf[dst + BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut buf[dst + 2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<2>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut buf[dst + 3 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<3>(abcdefg)),
    );
}

/// Entry point of [`ld4_sse2`] for `vp8_pred_luma4` mode 6.
pub(crate) fn ld4(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma4` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { ld4_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c VR4_SSE2
/// Vertical-Right
#[target_feature(enable = "sse2")]
fn vr4_sse2(buf: &mut [u8], dst: usize) {
    let one = _mm_set1_epi8(1);
    let i = i32::from(buf[dst - 1]);
    let j = i32::from(buf[dst - 1 + BPS_U]);
    let k = i32::from(buf[dst - 1 + 2 * BPS_U]);
    let x = i32::from(buf[dst - 1 - BPS_U]);
    let xabcd = loadl_epi64(&buf[dst - BPS_U - 1..]);
    let abcd0 = _mm_srli_si128::<1>(xabcd);
    let abcd = _mm_avg_epu8(xabcd, abcd0);
    let z_xabcd = _mm_slli_si128::<1>(xabcd); // C: _XABCD
    #[expect(
        clippy::cast_possible_truncation,
        reason = "C casts the two samples to short"
    )]
    let ix = (i | (x << 8)) as i16;
    let ixabcd = _mm_insert_epi16::<0>(z_xabcd, i32::from(ix));
    let avg1 = _mm_avg_epu8(ixabcd, abcd0);
    let lsb = _mm_and_si128(_mm_xor_si128(ixabcd, abcd0), one);
    let avg2 = _mm_subs_epu8(avg1, lsb);
    let efgh = _mm_avg_epu8(avg2, xabcd);
    webp_int32_to_mem(&mut buf[dst..], _mm_cvtsi128_si32(abcd));
    webp_int32_to_mem(&mut buf[dst + BPS_U..], _mm_cvtsi128_si32(efgh));
    webp_int32_to_mem(
        &mut buf[dst + 2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_slli_si128::<1>(abcd)),
    );
    webp_int32_to_mem(
        &mut buf[dst + 3 * BPS_U..],
        _mm_cvtsi128_si32(_mm_slli_si128::<1>(efgh)),
    );

    // these two are hard to implement in SSE2, so we keep the C-version:
    buf[dst_at(dst, 0, 2)] = avg3(j, i, x);
    buf[dst_at(dst, 0, 3)] = avg3(k, j, i);
}

/// Entry point of [`vr4_sse2`] for `vp8_pred_luma4` mode 5.
pub(crate) fn vr4(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma4` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { vr4_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c VL4_SSE2
/// Vertical-Left
#[target_feature(enable = "sse2")]
fn vl4_sse2(buf: &mut [u8], dst: usize) {
    let one = _mm_set1_epi8(1);
    let abcdefgh = loadl_epi64(&buf[dst - BPS_U..]);
    let bcdefgh_ = _mm_srli_si128::<1>(abcdefgh);
    let cdefgh__ = _mm_srli_si128::<2>(abcdefgh);
    let avg1 = _mm_avg_epu8(abcdefgh, bcdefgh_);
    let avg2 = _mm_avg_epu8(cdefgh__, bcdefgh_);
    let avg3 = _mm_avg_epu8(avg1, avg2);
    let lsb1 = _mm_and_si128(_mm_xor_si128(avg1, avg2), one);
    let ab = _mm_xor_si128(abcdefgh, bcdefgh_);
    let bc = _mm_xor_si128(cdefgh__, bcdefgh_);
    let abbc = _mm_or_si128(ab, bc);
    let lsb2 = _mm_and_si128(abbc, lsb1);
    let avg4 = _mm_subs_epu8(avg3, lsb2);
    let extra_out = _mm_cvtsi128_si32(_mm_srli_si128::<4>(avg4)).cast_unsigned();
    webp_int32_to_mem(&mut buf[dst..], _mm_cvtsi128_si32(avg1));
    webp_int32_to_mem(&mut buf[dst + BPS_U..], _mm_cvtsi128_si32(avg4));
    webp_int32_to_mem(
        &mut buf[dst + 2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(avg1)),
    );
    webp_int32_to_mem(
        &mut buf[dst + 3 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(avg4)),
    );

    // these two are hard to get and irregular
    buf[dst_at(dst, 3, 2)] = (extra_out & 0xff) as u8;
    buf[dst_at(dst, 3, 3)] = ((extra_out >> 8) & 0xff) as u8;
}

/// Entry point of [`vl4_sse2`] for `vp8_pred_luma4` mode 7.
pub(crate) fn vl4(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma4` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { vl4_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c RD4_SSE2
/// Down-right
#[target_feature(enable = "sse2")]
fn rd4_sse2(buf: &mut [u8], dst: usize) {
    let one = _mm_set1_epi8(1);
    let xabcd = loadl_epi64(&buf[dst - BPS_U - 1..]);
    let ____xabcd = _mm_slli_si128::<4>(xabcd);
    let i = u32::from(buf[dst - 1]);
    let j = u32::from(buf[dst - 1 + BPS_U]);
    let k = u32::from(buf[dst - 1 + 2 * BPS_U]);
    let l = u32::from(buf[dst - 1 + 3 * BPS_U]);
    let lkji_____ = _mm_cvtsi32_si128((l | (k << 8) | (j << 16) | (i << 24)).cast_signed());
    let lkjixabcd = _mm_or_si128(lkji_____, ____xabcd);
    let kjixabcd_ = _mm_srli_si128::<1>(lkjixabcd);
    let jixabcd__ = _mm_srli_si128::<2>(lkjixabcd);
    let avg1 = _mm_avg_epu8(jixabcd__, lkjixabcd);
    let lsb = _mm_and_si128(_mm_xor_si128(jixabcd__, lkjixabcd), one);
    let avg2 = _mm_subs_epu8(avg1, lsb);
    let abcdefg = _mm_avg_epu8(avg2, kjixabcd_);
    webp_int32_to_mem(&mut buf[dst + 3 * BPS_U..], _mm_cvtsi128_si32(abcdefg));
    webp_int32_to_mem(
        &mut buf[dst + 2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut buf[dst + BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<2>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut buf[dst..],
        _mm_cvtsi128_si32(_mm_srli_si128::<3>(abcdefg)),
    );
}

/// Entry point of [`rd4_sse2`] for `vp8_pred_luma4` mode 4.
pub(crate) fn rd4(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma4` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { rd4_sse2(buf, dst) }
}

//------------------------------------------------------------------------------
// Luma 16x16

/// C's implicit conversion of an `int` to `short` in `_mm_set1_epi16`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the int to short"
)]
fn to_short(v: i32) -> i16 {
    v as i16
}

// upstream: src/dsp/dec_sse2.c TrueMotion_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn true_motion_sse2(buf: &mut [u8], dst: usize, size: i32) {
    let top = dst - BPS_U;
    let mut dst = dst;
    let zero = _mm_setzero_si128();
    if size == 4 {
        let top_values = _mm_cvtsi32_si128(webp_mem_to_int32(&buf[top..]));
        let top_base = _mm_unpacklo_epi8(top_values, zero);
        for _ in 0..4 {
            let val = i32::from(buf[dst - 1]) - i32::from(buf[top - 1]);
            let base = _mm_set1_epi16(to_short(val));
            let out = _mm_packus_epi16(_mm_add_epi16(base, top_base), zero);
            webp_int32_to_mem(&mut buf[dst..], _mm_cvtsi128_si32(out));
            dst += BPS_U;
        }
    } else if size == 8 {
        let top_values = loadl_epi64(&buf[top..]);
        let top_base = _mm_unpacklo_epi8(top_values, zero);
        for _ in 0..8 {
            let val = i32::from(buf[dst - 1]) - i32::from(buf[top - 1]);
            let base = _mm_set1_epi16(to_short(val));
            let out = _mm_packus_epi16(_mm_add_epi16(base, top_base), zero);
            storel_epi64(&mut buf[dst..], out);
            dst += BPS_U;
        }
    } else {
        let top_values = loadu_si128(&buf[top..]);
        let top_base_0 = _mm_unpacklo_epi8(top_values, zero);
        let top_base_1 = _mm_unpackhi_epi8(top_values, zero);
        for _ in 0..16 {
            let val = i32::from(buf[dst - 1]) - i32::from(buf[top - 1]);
            let base = _mm_set1_epi16(to_short(val));
            let out_0 = _mm_add_epi16(base, top_base_0);
            let out_1 = _mm_add_epi16(base, top_base_1);
            let out = _mm_packus_epi16(out_0, out_1);
            storeu_si128(&mut buf[dst..], out);
            dst += BPS_U;
        }
    }
}

// upstream: src/dsp/dec_sse2.c TM4_SSE2
#[target_feature(enable = "sse2")]
fn tm4_sse2(buf: &mut [u8], dst: usize) {
    true_motion_sse2(buf, dst, 4);
}

/// Entry point of [`tm4_sse2`] for `vp8_pred_luma4` mode 1.
pub(crate) fn tm4(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma4` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { tm4_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c TM8uv_SSE2
#[target_feature(enable = "sse2")]
fn tm8uv_sse2(buf: &mut [u8], dst: usize) {
    true_motion_sse2(buf, dst, 8);
}

/// Entry point of [`tm8uv_sse2`] for `vp8_pred_chroma8` mode 1.
pub(crate) fn tm8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_chroma8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { tm8uv_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c TM16_SSE2
#[target_feature(enable = "sse2")]
fn tm16_sse2(buf: &mut [u8], dst: usize) {
    true_motion_sse2(buf, dst, 16);
}

/// Entry point of [`tm16_sse2`] for `vp8_pred_luma16` mode 1.
pub(crate) fn tm16(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { tm16_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c VE16_SSE2
#[target_feature(enable = "sse2")]
fn ve16_sse2(buf: &mut [u8], dst: usize) {
    let top = loadu_si128(&buf[dst - BPS_U..]);
    for j in 0..16 {
        storeu_si128(&mut buf[dst + j * BPS_U..], top);
    }
}

/// Entry point of [`ve16_sse2`] for `vp8_pred_luma16` mode 2.
pub(crate) fn ve16(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { ve16_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c HE16_SSE2
/// horizontal
#[target_feature(enable = "sse2")]
fn he16_sse2(buf: &mut [u8], dst: usize) {
    let mut dst = dst;
    for _ in 0..16 {
        let values = _mm_set1_epi8(buf[dst - 1].cast_signed());
        storeu_si128(&mut buf[dst..], values);
        dst += BPS_U;
    }
}

/// Entry point of [`he16_sse2`] for `vp8_pred_luma16` mode 3.
pub(crate) fn he16(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { he16_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c Put16_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn put16_sse2(v: u8, buf: &mut [u8], dst: usize) {
    let values = _mm_set1_epi8(v.cast_signed());
    for j in 0..16 {
        storeu_si128(&mut buf[dst + j * BPS_U..], values);
    }
}

/// C's implicit conversion of the `int` DC value to the `uint8_t` parameter.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C converts the int argument to uint8_t"
)]
fn to_u8(v: i32) -> u8 {
    v as u8
}

// upstream: src/dsp/dec_sse2.c DC16_SSE2
/// DC
#[target_feature(enable = "sse2")]
fn dc16_sse2(buf: &mut [u8], dst: usize) {
    let zero = _mm_setzero_si128();
    let top = loadu_si128(&buf[dst - BPS_U..]);
    let sad8x2 = _mm_sad_epu8(top, zero);
    // sum the two sads: sad8x2[0:1] + sad8x2[8:9]
    let sum = _mm_add_epi16(sad8x2, _mm_shuffle_epi32::<2>(sad8x2));
    let mut left = 0;
    for j in 0..16 {
        left += i32::from(buf[dst - 1 + j * BPS_U]);
    }
    {
        let dc = _mm_cvtsi128_si32(sum) + left + 16;
        put16_sse2(to_u8(dc >> 5), buf, dst);
    }
}

/// Entry point of [`dc16_sse2`] for `vp8_pred_luma16` mode 0.
pub(crate) fn dc16(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc16_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c DC16NoTop_SSE2
/// DC with top samples unavailable
#[target_feature(enable = "sse2")]
fn dc16_no_top_sse2(buf: &mut [u8], dst: usize) {
    let mut dc = 8;
    for j in 0..16 {
        dc += i32::from(buf[dst - 1 + j * BPS_U]);
    }
    put16_sse2(to_u8(dc >> 4), buf, dst);
}

/// Entry point of [`dc16_no_top_sse2`] for `vp8_pred_luma16` mode 4.
pub(crate) fn dc16_no_top(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc16_no_top_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c DC16NoLeft_SSE2
/// DC with left samples unavailable
#[target_feature(enable = "sse2")]
fn dc16_no_left_sse2(buf: &mut [u8], dst: usize) {
    let zero = _mm_setzero_si128();
    let top = loadu_si128(&buf[dst - BPS_U..]);
    let sad8x2 = _mm_sad_epu8(top, zero);
    // sum the two sads: sad8x2[0:1] + sad8x2[8:9]
    let sum = _mm_add_epi16(sad8x2, _mm_shuffle_epi32::<2>(sad8x2));
    let dc = _mm_cvtsi128_si32(sum) + 8;
    put16_sse2(to_u8(dc >> 4), buf, dst);
}

/// Entry point of [`dc16_no_left_sse2`] for `vp8_pred_luma16` mode 5.
pub(crate) fn dc16_no_left(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc16_no_left_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c DC16NoTopLeft_SSE2
/// DC with no top & left samples
#[target_feature(enable = "sse2")]
fn dc16_no_top_left_sse2(buf: &mut [u8], dst: usize) {
    put16_sse2(0x80, buf, dst);
}

/// Entry point of [`dc16_no_top_left_sse2`] for `vp8_pred_luma16` mode 6.
pub(crate) fn dc16_no_top_left(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_luma16` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc16_no_top_left_sse2(buf, dst) }
}

//------------------------------------------------------------------------------
// Chroma

// upstream: src/dsp/dec_sse2.c VE8uv_SSE2
/// vertical
#[target_feature(enable = "sse2")]
fn ve8uv_sse2(buf: &mut [u8], dst: usize) {
    let top = loadl_epi64(&buf[dst - BPS_U..]);
    for j in 0..8 {
        storel_epi64(&mut buf[dst + j * BPS_U..], top);
    }
}

/// Entry point of [`ve8uv_sse2`] for `vp8_pred_chroma8` mode 2.
pub(crate) fn ve8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_chroma8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { ve8uv_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c Put8x8uv_SSE2
/// helper for chroma-DC predictions
#[target_feature(enable = "sse2")]
#[inline]
fn put8x8uv_sse2(v: u8, buf: &mut [u8], dst: usize) {
    let values = _mm_set1_epi8(v.cast_signed());
    for j in 0..8 {
        storel_epi64(&mut buf[dst + j * BPS_U..], values);
    }
}

// upstream: src/dsp/dec_sse2.c DC8uv_SSE2
/// DC
#[target_feature(enable = "sse2")]
fn dc8uv_sse2(buf: &mut [u8], dst: usize) {
    let zero = _mm_setzero_si128();
    let top = loadl_epi64(&buf[dst - BPS_U..]);
    let sum = _mm_sad_epu8(top, zero);
    let mut left = 0;
    for j in 0..8 {
        left += i32::from(buf[dst - 1 + j * BPS_U]);
    }
    {
        let dc = _mm_cvtsi128_si32(sum) + left + 8;
        put8x8uv_sse2(to_u8(dc >> 4), buf, dst);
    }
}

/// Entry point of [`dc8uv_sse2`] for `vp8_pred_chroma8` mode 0.
pub(crate) fn dc8uv(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_chroma8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc8uv_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c DC8uvNoLeft_SSE2
/// DC with no left samples
#[target_feature(enable = "sse2")]
fn dc8uv_no_left_sse2(buf: &mut [u8], dst: usize) {
    let zero = _mm_setzero_si128();
    let top = loadl_epi64(&buf[dst - BPS_U..]);
    let sum = _mm_sad_epu8(top, zero);
    let dc = _mm_cvtsi128_si32(sum) + 4;
    put8x8uv_sse2(to_u8(dc >> 3), buf, dst);
}

/// Entry point of [`dc8uv_no_left_sse2`] for `vp8_pred_chroma8` mode 5.
pub(crate) fn dc8uv_no_left(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_chroma8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc8uv_no_left_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c DC8uvNoTop_SSE2
/// DC with no top samples
#[target_feature(enable = "sse2")]
fn dc8uv_no_top_sse2(buf: &mut [u8], dst: usize) {
    let mut dc0 = 4;
    for i in 0..8 {
        dc0 += i32::from(buf[dst - 1 + i * BPS_U]);
    }
    put8x8uv_sse2(to_u8(dc0 >> 3), buf, dst);
}

/// Entry point of [`dc8uv_no_top_sse2`] for `vp8_pred_chroma8` mode 4.
pub(crate) fn dc8uv_no_top(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_chroma8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc8uv_no_top_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c DC8uvNoTopLeft_SSE2
/// DC with nothing
#[target_feature(enable = "sse2")]
fn dc8uv_no_top_left_sse2(buf: &mut [u8], dst: usize) {
    put8x8uv_sse2(0x80, buf, dst);
}

/// Entry point of [`dc8uv_no_top_left_sse2`] for `vp8_pred_chroma8` mode 6.
pub(crate) fn dc8uv_no_top_left(buf: &mut [u8], dst: usize) {
    // SAFETY: `vp8_pred_chroma8` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { dc8uv_no_top_left_sse2(buf, dst) }
}

// upstream: src/dsp/dec_sse2.c VP8DspInitSSE2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
