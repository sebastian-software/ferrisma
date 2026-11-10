// libwebp v1.6.0, src/dsp/yuv_sse41.c:
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
// YUV->RGB conversion functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/yuv_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE4.1 RGB and BGR row samplers and 32-pixel converters, and the RGB
//! to YUV converters of `yuv_sse41.c`, behind the dispatch functions of
//! `src/dsp/yuv.rs` (ADR 0008). Each `#[target_feature(enable = "sse4.1")]`
//! function keeps the upstream name; each function that an `Init` function
//! assigns to a pointer has a safe `pub(crate)` entry point named after it
//! without the suffix. `WebPInitSamplers` and `WebPInitConvertARGBToYUV` run
//! `WebPInitSamplersSSE41` and `WebPInitConvertARGBToYUVSSE41` after and
//! independently of their SSE2 counterparts, so the dispatch functions test
//! `features().sse4_1` alone, before `features().sse2`.
//!
//! - `WebPInitSamplersSSE41` assigns `WebPSamplers[MODE_RGB, MODE_BGR]`:
//!   [`yuv_to_rgb_row`], [`yuv_to_bgr_row`] behind `webp_samplers`.
//! - `WebPInitConvertARGBToYUVSSE41` assigns `WebPConvertARGBToY`,
//!   `WebPConvertARGBToUV`, `WebPConvertRGB24ToY`, `WebPConvertBGR24ToY` and
//!   `WebPConvertRGBA32ToUV`: [`convert_argb_to_y`], [`convert_argb_to_uv`],
//!   [`convert_rgb24_to_y`], [`convert_bgr24_to_y`],
//!   [`convert_rgba32_to_uv`].
//!
//! `VP8YuvToRgb32_SSE41` and `VP8YuvToBgr32_SSE41` are `pub(crate)`
//! `#[target_feature]` functions for `upsampling_sse41.rs`.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers, macros and repeated blocks**, as in `yuv_sse2.rs`
//!    (deviations 2 and 3): slices with positions for the advancing pointers,
//!    references for the `__m128i*` parameters, `TRANSFORM` with its shift as
//!    a const generic parameter, `MK_CST_16` as a function, one helper for
//!    the four `YUV444ToRGB_SSE41`/`YUV420ToRGB_SSE41` calls and one for the
//!    finish-off loops, and copies of the inputs of
//!    `HorizontalAddPack_SSE41(&rgb[0], &rgb[1], &rgb[0])`. The local
//!    `WEBP_SSE41_SHUFF` macro of `RGB24PackedToPlanar_SSE41`, over the six
//!    loaded registers and the three masks in scope, is a function of them
//!    that returns the two output registers.
//! 3. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3).
//!
//! # Excluded undefined behavior
//!
//! 1. **`WebPConvertRGBA32ToUV` inputs from 32768 up**, as in `yuv_sse2.rs`
//!    (exclusion 1): `ConvertRGBA32ToUV_SSE41` reads them as negative 16-bit
//!    values where the C function computes in `int`, which is upstream's
//!    divergence; the parity tests draw values up to 32767. Every other
//!    function gives the bytes of the C function for every input.
//!
//! # Checks not applicable
//!
//! 1. **`WebPInitSamplersSSE41` and `WebPInitConvertARGBToYUVSSE41`** only
//!    assign pointers (ADR 0003); their assignments are the selection in the
//!    dispatch functions listed above.
//! 2. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_yuv.rs`); the diagnostic
//!    `release-simd` oracle runs the WP4 probes against upstream's SSE4.1.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_adds_epu16, _mm_avg_epu8, _mm_cvtsi32_si128,
    _mm_madd_epi16, _mm_mulhi_epu16, _mm_or_si128, _mm_packs_epi32, _mm_packus_epi16, _mm_set_epi8,
    _mm_set_epi16, _mm_set1_epi16, _mm_set1_epi32, _mm_setzero_si128, _mm_shuffle_epi8,
    _mm_srai_epi16, _mm_srai_epi32, _mm_srli_epi16, _mm_sub_epi16, _mm_subs_epu16,
    _mm_unpackhi_epi8, _mm_unpackhi_epi16, _mm_unpackhi_epi64, _mm_unpacklo_epi8,
    _mm_unpacklo_epi16, _mm_unpacklo_epi32, _mm_unpacklo_epi64,
};

use crate::dsp::common_sse2::{Lane, loadl_epi64, loadu_si128, storeu_si128};
use crate::dsp::common_sse41::{vp8_planar_to24b_sse41, vp8l32b_to_planar_sse41};
use crate::dsp::yuv::{
    YUV_FIX, YUV_HALF, store_u8, vp8_yuv_to_bgr, vp8_yuv_to_rgb, vp8rgb_to_y,
    webp_convert_argb_to_uv_c, webp_convert_rgba32_to_uv_c,
};
use crate::utils::utils::webp_mem_to_int32;

/// A non-negative C `int` length or index as `usize`.
fn to_usize(v: i32) -> usize {
    usize::try_from(v).expect("a non-negative length")
}

//-----------------------------------------------------------------------------
// Convert spans of 32 pixels to various RGB formats for the fancy upsampler.

// upstream: src/dsp/yuv_sse41.c ConvertYUV444ToRGB_SSE41
/// These constants are 14b fixed-point version of ITU-R BT.601 constants.
/// R = (19077 * y             + 26149 * v - 14234) >> 6
/// G = (19077 * y -  6419 * u - 13320 * v +  8708) >> 6
/// B = (19077 * y + 33050 * u             - 17685) >> 6
#[target_feature(enable = "sse4.1")]
#[inline]
fn convert_yuv444_to_rgb_sse41(
    y0: &__m128i,
    u0: &__m128i,
    v0: &__m128i,
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let k19077 = _mm_set1_epi16(19077);
    let k26149 = _mm_set1_epi16(26149);
    let k14234 = _mm_set1_epi16(14234);
    // 33050 doesn't fit in a signed short: only use this with unsigned arithmetic
    let k33050 = _mm_set1_epi16(33050_u16.cast_signed());
    let k17685 = _mm_set1_epi16(17685);
    let k6419 = _mm_set1_epi16(6419);
    let k13320 = _mm_set1_epi16(13320);
    let k8708 = _mm_set1_epi16(8708);

    let y1 = _mm_mulhi_epu16(*y0, k19077);

    let r0 = _mm_mulhi_epu16(*v0, k26149);
    let r1 = _mm_sub_epi16(y1, k14234);
    let r2 = _mm_add_epi16(r1, r0);

    let g0 = _mm_mulhi_epu16(*u0, k6419);
    let g1 = _mm_mulhi_epu16(*v0, k13320);
    let g2 = _mm_add_epi16(y1, k8708);
    let g3 = _mm_add_epi16(g0, g1);
    let g4 = _mm_sub_epi16(g2, g3);

    // be careful with the saturated *unsigned* arithmetic here!
    let b0 = _mm_mulhi_epu16(*u0, k33050);
    let b1 = _mm_adds_epu16(b0, y1);
    let b2 = _mm_subs_epu16(b1, k17685);

    // use logical shift for B2, which can be larger than 32767
    *r = _mm_srai_epi16::<6>(r2); // range: [-14234, 30815]
    *g = _mm_srai_epi16::<6>(g4); // range: [-10953, 27710]
    *b = _mm_srli_epi16::<6>(b2); // range: [0, 34238]
}

// upstream: src/dsp/yuv_sse41.c Load_HI_16_SSE41
/// Load the bytes into the *upper* part of 16b words. That's "<< 8", basically.
#[target_feature(enable = "sse4.1")]
#[inline]
fn load_hi_16_sse41(src: &[u8]) -> __m128i {
    let zero = _mm_setzero_si128();
    _mm_unpacklo_epi8(zero, loadl_epi64(src))
}

// upstream: src/dsp/yuv_sse41.c Load_UV_HI_8_SSE41
/// Load and replicate the U/V samples
#[target_feature(enable = "sse4.1")]
#[inline]
fn load_uv_hi_8_sse41(src: &[u8]) -> __m128i {
    let zero = _mm_setzero_si128();
    let tmp0 = _mm_cvtsi32_si128(webp_mem_to_int32(src));
    let tmp1 = _mm_unpacklo_epi8(zero, tmp0);
    _mm_unpacklo_epi16(tmp1, tmp1) // replicate samples
}

// upstream: src/dsp/yuv_sse41.c YUV444ToRGB_SSE41
/// Convert 32 samples of YUV444 to R/G/B
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv444_to_rgb_sse41(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let y0 = load_hi_16_sse41(y);
    let u0 = load_hi_16_sse41(u);
    let v0 = load_hi_16_sse41(v);
    convert_yuv444_to_rgb_sse41(&y0, &u0, &v0, r, g, b);
}

// upstream: src/dsp/yuv_sse41.c YUV420ToRGB_SSE41
/// Convert 32 samples of YUV420 to R/G/B
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv420_to_rgb_sse41(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let y0 = load_hi_16_sse41(y);
    let u0 = load_uv_hi_8_sse41(u);
    let v0 = load_uv_hi_8_sse41(v);
    convert_yuv444_to_rgb_sse41(&y0, &u0, &v0, r, g, b);
}

// upstream: src/dsp/yuv_sse41.c PlanarTo24b_SSE41
/// Pack the planar buffers
/// rrrr... rrrr... gggg... gggg... bbbb... bbbb....
/// triplet by triplet in the output buffer rgb as rgbrgbrgbrgb ...
#[target_feature(enable = "sse4.1")]
#[inline]
fn planar_to24b_sse41(
    in0: &mut __m128i,
    in1: &mut __m128i,
    in2: &mut __m128i,
    in3: &mut __m128i,
    in4: &mut __m128i,
    in5: &mut __m128i,
    rgb: &mut [u8],
) {
    // The input is 6 registers of sixteen 8b but for the sake of explanation,
    // let's take 6 registers of four 8b values.
    // To pack, we will keep taking one every two 8b integer and move it
    // around as follows:
    // Input:
    //   r0r1r2r3 | r4r5r6r7 | g0g1g2g3 | g4g5g6g7 | b0b1b2b3 | b4b5b6b7
    // Split the 6 registers in two sets of 3 registers: the first set as the even
    // 8b bytes, the second the odd ones:
    //   r0r2r4r6 | g0g2g4g6 | b0b2b4b6 | r1r3r5r7 | g1g3g5g7 | b1b3b5b7
    // Repeat the same permutations twice more:
    //   r0r4g0g4 | b0b4r1r5 | g1g5b1b5 | r2r6g2g6 | b2b6r3r7 | g3g7b3b7
    //   r0g0b0r1 | g1b1r2g2 | b2r3g3b3 | r4g4b4r5 | g5b5r6g6 | b6r7g7b7
    vp8_planar_to24b_sse41(in0, in1, in2, in3, in4, in5);

    storeu_si128(&mut rgb[0..], *in0);
    storeu_si128(&mut rgb[16..], *in1);
    storeu_si128(&mut rgb[32..], *in2);
    storeu_si128(&mut rgb[48..], *in3);
    storeu_si128(&mut rgb[64..], *in4);
    storeu_si128(&mut rgb[80..], *in5);
}

/// The four `YUV444ToRGB_SSE41` calls of `VP8YuvToRgb32_SSE41` and
/// `VP8YuvToBgr32_SSE41`, at `y + 8 * k`, `u + 8 * k`, `v + 8 * k`, into
/// `R0`..`R3`, `G0`..`G3` and `B0`..`B3` (deviation 2).
#[target_feature(enable = "sse4.1")]
#[inline]
fn yuv444_to_rgb_4x8(y: &[u8], u: &[u8], v: &[u8]) -> [[__m128i; 3]; 4] {
    let mut out = [[_mm_setzero_si128(); 3]; 4];
    for (k, [r, g, b]) in out.iter_mut().enumerate() {
        let n = 8 * k;
        yuv444_to_rgb_sse41(&y[n..], &u[n..], &v[n..], r, g, b);
    }
    out
}

// upstream: src/dsp/yuv_sse41.c VP8YuvToRgb32_SSE41
/// Converts 32 YUV 4:4:4 samples to RGB.
#[target_feature(enable = "sse4.1")]
pub(crate) fn vp8_yuv_to_rgb32_sse41(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let [[r0, g0, b0], [r1, g1, b1], [r2, g2, b2], [r3, g3, b3]] = yuv444_to_rgb_4x8(y, u, v);

    // Cast to 8b and store as RRRRGGGGBBBB.
    let mut rgb0 = _mm_packus_epi16(r0, r1);
    let mut rgb1 = _mm_packus_epi16(r2, r3);
    let mut rgb2 = _mm_packus_epi16(g0, g1);
    let mut rgb3 = _mm_packus_epi16(g2, g3);
    let mut rgb4 = _mm_packus_epi16(b0, b1);
    let mut rgb5 = _mm_packus_epi16(b2, b3);

    // Pack as RGBRGBRGBRGB.
    planar_to24b_sse41(
        &mut rgb0, &mut rgb1, &mut rgb2, &mut rgb3, &mut rgb4, &mut rgb5, dst,
    );
}

// upstream: src/dsp/yuv_sse41.c VP8YuvToBgr32_SSE41
/// Converts 32 YUV 4:4:4 samples to BGR.
#[target_feature(enable = "sse4.1")]
pub(crate) fn vp8_yuv_to_bgr32_sse41(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let [[r0, g0, b0], [r1, g1, b1], [r2, g2, b2], [r3, g3, b3]] = yuv444_to_rgb_4x8(y, u, v);

    // Cast to 8b and store as BBBBGGGGRRRR.
    let mut bgr0 = _mm_packus_epi16(b0, b1);
    let mut bgr1 = _mm_packus_epi16(b2, b3);
    let mut bgr2 = _mm_packus_epi16(g0, g1);
    let mut bgr3 = _mm_packus_epi16(g2, g3);
    let mut bgr4 = _mm_packus_epi16(r0, r1);
    let mut bgr5 = _mm_packus_epi16(r2, r3);

    // Pack as BGRBGRBGRBGR.
    planar_to24b_sse41(
        &mut bgr0, &mut bgr1, &mut bgr2, &mut bgr3, &mut bgr4, &mut bgr5, dst,
    );
}

//-----------------------------------------------------------------------------
// Arbitrary-length row conversion functions

/// The finish-off loop of the row functions: converts pixels `n..len` one
/// by one, advancing U and V after each odd pixel, as the C loops do with
/// `u += (n & 1)` (deviation 2).
#[expect(clippy::too_many_arguments, reason = "the C loop state")]
fn finish_off(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    dst: &mut [u8],
    n: i32,
    len: i32,
    xstep: usize,
    func: impl Fn(u8, u8, u8, &mut [u8]),
) {
    let (mut y_pos, mut uv_pos, mut dst_pos) = (0, 0, 0);
    for n in n..len {
        func(y[y_pos], u[uv_pos], v[uv_pos], &mut dst[dst_pos..]);
        dst_pos += xstep;
        y_pos += 1;
        uv_pos += to_usize(n & 1);
    }
}

/// The four `YUV420ToRGB_SSE41` calls of the RGB and BGR row functions, at
/// `y + 8 * k`, `u + 4 * k`, `v + 4 * k`, into `R0`..`R3`, `G0`..`G3` and
/// `B0`..`B3` (deviation 2).
#[target_feature(enable = "sse4.1")]
#[inline]
fn yuv420_to_rgb_4x8(y: &[u8], u: &[u8], v: &[u8]) -> [[__m128i; 3]; 4] {
    let mut out = [[_mm_setzero_si128(); 3]; 4];
    for (k, [r, g, b]) in out.iter_mut().enumerate() {
        yuv420_to_rgb_sse41(&y[8 * k..], &u[4 * k..], &v[4 * k..], r, g, b);
    }
    out
}

// upstream: src/dsp/yuv_sse41.c YuvToRgbRow_SSE41
#[target_feature(enable = "sse4.1")]
fn yuv_to_rgb_row_sse41(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    let (mut y_pos, mut uv_pos, mut dst_pos) = (0, 0, 0);
    let mut n = 0;
    while n + 32 <= len {
        let [[r0, g0, b0], [r1, g1, b1], [r2, g2, b2], [r3, g3, b3]] =
            yuv420_to_rgb_4x8(&y[y_pos..], &u[uv_pos..], &v[uv_pos..]);

        // Cast to 8b and store as RRRRGGGGBBBB.
        let mut rgb0 = _mm_packus_epi16(r0, r1);
        let mut rgb1 = _mm_packus_epi16(r2, r3);
        let mut rgb2 = _mm_packus_epi16(g0, g1);
        let mut rgb3 = _mm_packus_epi16(g2, g3);
        let mut rgb4 = _mm_packus_epi16(b0, b1);
        let mut rgb5 = _mm_packus_epi16(b2, b3);

        // Pack as RGBRGBRGBRGB.
        planar_to24b_sse41(
            &mut rgb0,
            &mut rgb1,
            &mut rgb2,
            &mut rgb3,
            &mut rgb4,
            &mut rgb5,
            &mut dst[dst_pos..],
        );

        y_pos += 32;
        uv_pos += 16;
        n += 32;
        dst_pos += 32 * 3;
    }
    // Finish off
    finish_off(
        &y[y_pos..],
        &u[uv_pos..],
        &v[uv_pos..],
        &mut dst[dst_pos..],
        n,
        len,
        3,
        |y, u, v, out| vp8_yuv_to_rgb(i32::from(y), i32::from(u), i32::from(v), out),
    );
}

/// Entry point of [`yuv_to_rgb_row_sse41`] for `webp_samplers` (`MODE_RGB`).
pub(crate) fn yuv_to_rgb_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse4_1`, the answer of `VP8GetCPUInfo(kSSE4_1)`, selected the
    // SSE4.1 variant, so the CPU has SSE4.1, which the function requires.
    unsafe { yuv_to_rgb_row_sse41(y, u, v, dst, len) }
}

// upstream: src/dsp/yuv_sse41.c YuvToBgrRow_SSE41
#[target_feature(enable = "sse4.1")]
fn yuv_to_bgr_row_sse41(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    let (mut y_pos, mut uv_pos, mut dst_pos) = (0, 0, 0);
    let mut n = 0;
    while n + 32 <= len {
        let [[r0, g0, b0], [r1, g1, b1], [r2, g2, b2], [r3, g3, b3]] =
            yuv420_to_rgb_4x8(&y[y_pos..], &u[uv_pos..], &v[uv_pos..]);

        // Cast to 8b and store as BBBBGGGGRRRR.
        let mut bgr0 = _mm_packus_epi16(b0, b1);
        let mut bgr1 = _mm_packus_epi16(b2, b3);
        let mut bgr2 = _mm_packus_epi16(g0, g1);
        let mut bgr3 = _mm_packus_epi16(g2, g3);
        let mut bgr4 = _mm_packus_epi16(r0, r1);
        let mut bgr5 = _mm_packus_epi16(r2, r3);

        // Pack as BGRBGRBGRBGR.
        planar_to24b_sse41(
            &mut bgr0,
            &mut bgr1,
            &mut bgr2,
            &mut bgr3,
            &mut bgr4,
            &mut bgr5,
            &mut dst[dst_pos..],
        );

        y_pos += 32;
        uv_pos += 16;
        n += 32;
        dst_pos += 32 * 3;
    }
    // Finish off
    finish_off(
        &y[y_pos..],
        &u[uv_pos..],
        &v[uv_pos..],
        &mut dst[dst_pos..],
        n,
        len,
        3,
        |y, u, v, out| vp8_yuv_to_bgr(i32::from(y), i32::from(u), i32::from(v), out),
    );
}

/// Entry point of [`yuv_to_bgr_row_sse41`] for `webp_samplers` (`MODE_BGR`).
pub(crate) fn yuv_to_bgr_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { yuv_to_bgr_row_sse41(y, u, v, dst, len) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/yuv_sse41.c WebPInitSamplersSSE41 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)

//------------------------------------------------------------------------------
// RGB24/32 -> YUV converters

// upstream: src/dsp/yuv_sse41.c LOAD_16
/// Load eight 16b-words from *src.
#[target_feature(enable = "sse4.1")]
#[inline]
fn load_16<T: Lane>(src: &[T]) -> __m128i {
    loadu_si128(src)
}

// upstream: src/dsp/yuv_sse41.c STORE_16
/// Store either 16b-words into *dst
#[target_feature(enable = "sse4.1")]
#[inline]
fn store_16<T: Lane>(v: __m128i, dst: &mut [T]) {
    storeu_si128(dst, v);
}

// upstream: src/dsp/yuv_sse41.c WEBP_SSE41_SHUFF
/// Shuffles the six loaded registers with the three masks and ORs them into
/// the two registers of one channel, `out[OUT + 0]` and `out[OUT + 1]`.
#[target_feature(enable = "sse4.1")]
#[inline]
fn webp_sse41_shuff(
    a: &[__m128i; 6],
    shuff0: __m128i,
    shuff1: __m128i,
    shuff2: __m128i,
) -> [__m128i; 2] {
    let tmp0 = _mm_shuffle_epi8(a[0], shuff0);
    let tmp1 = _mm_shuffle_epi8(a[1], shuff1);
    let tmp2 = _mm_shuffle_epi8(a[2], shuff2);
    let tmp3 = _mm_shuffle_epi8(a[3], shuff0);
    let tmp4 = _mm_shuffle_epi8(a[4], shuff1);
    let tmp5 = _mm_shuffle_epi8(a[5], shuff2);

    // OR everything to get one channel
    let tmp6 = _mm_or_si128(tmp0, tmp1);
    let tmp7 = _mm_or_si128(tmp3, tmp4);
    [_mm_or_si128(tmp6, tmp2), _mm_or_si128(tmp7, tmp5)]
}

// upstream: src/dsp/yuv_sse41.c RGB24PackedToPlanar_SSE41
/// Unpack the 8b input rgbrgbrgbrgb ... as contiguous registers:
/// rrrr... rrrr... gggg... gggg... bbbb... bbbb....
/// Similar to `PlanarTo24bHelper()`, but in reverse order.
#[target_feature(enable = "sse4.1")]
#[inline]
fn rgb24_packed_to_planar_sse41(rgb: &[u8], out: &mut [__m128i; 6]) {
    let a = [
        loadu_si128(&rgb[0..]),
        loadu_si128(&rgb[16..]),
        loadu_si128(&rgb[32..]),
        loadu_si128(&rgb[48..]),
        loadu_si128(&rgb[64..]),
        loadu_si128(&rgb[80..]),
    ];

    // Compute RR.
    {
        let shuff0 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 15, 12, 9, 6, 3, 0);
        let shuff1 = _mm_set_epi8(-1, -1, -1, -1, -1, 14, 11, 8, 5, 2, -1, -1, -1, -1, -1, -1);
        let shuff2 = _mm_set_epi8(13, 10, 7, 4, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
        [out[0], out[1]] = webp_sse41_shuff(&a, shuff0, shuff1, shuff2);
    }
    // Compute GG.
    {
        let shuff0 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 13, 10, 7, 4, 1);
        let shuff1 = _mm_set_epi8(-1, -1, -1, -1, -1, 15, 12, 9, 6, 3, 0, -1, -1, -1, -1, -1);
        let shuff2 = _mm_set_epi8(14, 11, 8, 5, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
        [out[2], out[3]] = webp_sse41_shuff(&a, shuff0, shuff1, shuff2);
    }
    // Compute BB.
    {
        let shuff0 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 14, 11, 8, 5, 2);
        let shuff1 = _mm_set_epi8(-1, -1, -1, -1, -1, -1, 13, 10, 7, 4, 1, -1, -1, -1, -1, -1);
        let shuff2 = _mm_set_epi8(15, 12, 9, 6, 3, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1);
        [out[4], out[5]] = webp_sse41_shuff(&a, shuff0, shuff1, shuff2);
    }
}

// upstream: src/dsp/yuv_sse41.c RGB32PackedToPlanar_SSE41
/// Convert 8 packed ARGB to r[], g[], b[]
#[target_feature(enable = "sse4.1")]
#[inline]
fn rgb32_packed_to_planar_sse41(argb: &[u32], rgb: &mut [__m128i; 6]) {
    let zero = _mm_setzero_si128();
    let mut a0 = load_16(&argb[0..]);
    let mut a1 = load_16(&argb[4..]);
    let mut a2 = load_16(&argb[8..]);
    let mut a3 = load_16(&argb[12..]);
    vp8l32b_to_planar_sse41(&mut a0, &mut a1, &mut a2, &mut a3);
    rgb[0] = _mm_unpacklo_epi8(a1, zero);
    rgb[1] = _mm_unpackhi_epi8(a1, zero);
    rgb[2] = _mm_unpacklo_epi8(a2, zero);
    rgb[3] = _mm_unpackhi_epi8(a2, zero);
    rgb[4] = _mm_unpacklo_epi8(a3, zero);
    rgb[5] = _mm_unpackhi_epi8(a3, zero);
}

// upstream: src/dsp/yuv_sse41.c TRANSFORM
/// This macro computes `(RG * MULT_RG + GB * MULT_GB + ROUNDER) >> DESCALE_FIX`
/// It's a macro and not a function because we need to use immediate values with
/// `srai_epi32`, e.g.
#[target_feature(enable = "sse4.1")]
#[inline]
fn transform<const DESCALE_FIX: i32>(
    rg_lo: __m128i,
    rg_hi: __m128i,
    gb_lo: __m128i,
    gb_hi: __m128i,
    mult_rg: __m128i,
    mult_gb: __m128i,
    rounder: __m128i,
) -> __m128i {
    let v0_lo = _mm_madd_epi16(rg_lo, mult_rg);
    let v0_hi = _mm_madd_epi16(rg_hi, mult_rg);
    let v1_lo = _mm_madd_epi16(gb_lo, mult_gb);
    let v1_hi = _mm_madd_epi16(gb_hi, mult_gb);
    let v2_lo = _mm_add_epi32(v0_lo, v1_lo);
    let v2_hi = _mm_add_epi32(v0_hi, v1_hi);
    let v3_lo = _mm_add_epi32(v2_lo, rounder);
    let v3_hi = _mm_add_epi32(v2_hi, rounder);
    let v5_lo = _mm_srai_epi32::<DESCALE_FIX>(v3_lo);
    let v5_hi = _mm_srai_epi32::<DESCALE_FIX>(v3_hi);
    _mm_packs_epi32(v5_lo, v5_hi)
}

// upstream: src/dsp/yuv_sse41.c MK_CST_16
/// The `int` arguments convert to the `short` parameters of `_mm_set_epi16`,
/// as in C; every value used fits.
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the int arguments to short"
)]
fn mk_cst_16(a: i32, b: i32) -> __m128i {
    let (a, b) = (a as i16, b as i16);
    _mm_set_epi16(b, a, b, a, b, a, b, a)
}

// upstream: src/dsp/yuv_sse41.c ConvertRGBToY_SSE41
#[target_feature(enable = "sse4.1")]
#[inline]
fn convert_rgb_to_y_sse41(r: &__m128i, g: &__m128i, b: &__m128i, y: &mut __m128i) {
    let k_rg_y = mk_cst_16(16839, 33059 - 16384);
    let k_gb_y = mk_cst_16(16384, 6420);
    let k_half_y = _mm_set1_epi32((16 << YUV_FIX) + YUV_HALF);

    let rg_lo = _mm_unpacklo_epi16(*r, *g);
    let rg_hi = _mm_unpackhi_epi16(*r, *g);
    let gb_lo = _mm_unpacklo_epi16(*g, *b);
    let gb_hi = _mm_unpackhi_epi16(*g, *b);
    *y = transform::<YUV_FIX>(rg_lo, rg_hi, gb_lo, gb_hi, k_rg_y, k_gb_y, k_half_y);
}

// upstream: src/dsp/yuv_sse41.c ConvertRGBToUV_SSE41
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_rgb_to_uv_sse41(
    r: &__m128i,
    g: &__m128i,
    b: &__m128i,
    u: &mut __m128i,
    v: &mut __m128i,
) {
    let k_rg_u = mk_cst_16(-9719, -19081);
    let k_gb_u = mk_cst_16(0, 28800);
    let k_rg_v = mk_cst_16(28800, 0);
    let k_gb_v = mk_cst_16(-24116, -4684);
    let k_half_uv = _mm_set1_epi32(((128 << YUV_FIX) + YUV_HALF) << 2);

    let rg_lo = _mm_unpacklo_epi16(*r, *g);
    let rg_hi = _mm_unpackhi_epi16(*r, *g);
    let gb_lo = _mm_unpacklo_epi16(*g, *b);
    let gb_hi = _mm_unpackhi_epi16(*g, *b);
    *u = transform::<{ YUV_FIX + 2 }>(rg_lo, rg_hi, gb_lo, gb_hi, k_rg_u, k_gb_u, k_half_uv);
    *v = transform::<{ YUV_FIX + 2 }>(rg_lo, rg_hi, gb_lo, gb_hi, k_rg_v, k_gb_v, k_half_uv);
}

// upstream: src/dsp/yuv_sse41.c ConvertRGB24ToY_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_rgb24_to_y_sse41(rgb: &[u8], y: &mut [u8], width: i32) {
    let max_width = width & !31;
    let mut rgb_pos = 0;
    let mut i = 0;
    while i < max_width {
        let mut rgb_plane = [_mm_setzero_si128(); 6];

        rgb24_packed_to_planar_sse41(&rgb[rgb_pos..], &mut rgb_plane);

        for j in 0..2 {
            let zero = _mm_setzero_si128();
            let mut y0 = _mm_setzero_si128();
            let mut y1 = _mm_setzero_si128();

            // Convert to 16-bit Y.
            let mut r = _mm_unpacklo_epi8(rgb_plane[j], zero);
            let mut g = _mm_unpacklo_epi8(rgb_plane[2 + j], zero);
            let mut b = _mm_unpacklo_epi8(rgb_plane[4 + j], zero);
            convert_rgb_to_y_sse41(&r, &g, &b, &mut y0);

            // Convert to 16-bit Y.
            r = _mm_unpackhi_epi8(rgb_plane[j], zero);
            g = _mm_unpackhi_epi8(rgb_plane[2 + j], zero);
            b = _mm_unpackhi_epi8(rgb_plane[4 + j], zero);
            convert_rgb_to_y_sse41(&r, &g, &b, &mut y1);

            // Cast to 8-bit and store.
            store_16(_mm_packus_epi16(y0, y1), &mut y[to_usize(i)..]);
            i += 16;
        }
        rgb_pos += 3 * 16 * 2;
    }
    while i < width {
        // left-over
        let px = &rgb[rgb_pos..rgb_pos + 3];
        y[to_usize(i)] = store_u8(vp8rgb_to_y(
            i32::from(px[0]),
            i32::from(px[1]),
            i32::from(px[2]),
            YUV_HALF,
        ));
        i += 1;
        rgb_pos += 3;
    }
}

/// Entry point of [`convert_rgb24_to_y_sse41`] for `webp_convert_rgb24_to_y`.
pub(crate) fn convert_rgb24_to_y(rgb: &[u8], y: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_rgb24_to_y` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { convert_rgb24_to_y_sse41(rgb, y, width) }
}

// upstream: src/dsp/yuv_sse41.c ConvertBGR24ToY_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_bgr24_to_y_sse41(bgr: &[u8], y: &mut [u8], width: i32) {
    let max_width = width & !31;
    let mut bgr_pos = 0;
    let mut i = 0;
    while i < max_width {
        let mut bgr_plane = [_mm_setzero_si128(); 6];

        rgb24_packed_to_planar_sse41(&bgr[bgr_pos..], &mut bgr_plane);

        for j in 0..2 {
            let zero = _mm_setzero_si128();
            let mut y0 = _mm_setzero_si128();
            let mut y1 = _mm_setzero_si128();

            // Convert to 16-bit Y.
            let mut b = _mm_unpacklo_epi8(bgr_plane[j], zero);
            let mut g = _mm_unpacklo_epi8(bgr_plane[2 + j], zero);
            let mut r = _mm_unpacklo_epi8(bgr_plane[4 + j], zero);
            convert_rgb_to_y_sse41(&r, &g, &b, &mut y0);

            // Convert to 16-bit Y.
            b = _mm_unpackhi_epi8(bgr_plane[j], zero);
            g = _mm_unpackhi_epi8(bgr_plane[2 + j], zero);
            r = _mm_unpackhi_epi8(bgr_plane[4 + j], zero);
            convert_rgb_to_y_sse41(&r, &g, &b, &mut y1);

            // Cast to 8-bit and store.
            store_16(_mm_packus_epi16(y0, y1), &mut y[to_usize(i)..]);
            i += 16;
        }
        bgr_pos += 3 * 16 * 2;
    }
    while i < width {
        // left-over
        let px = &bgr[bgr_pos..bgr_pos + 3];
        y[to_usize(i)] = store_u8(vp8rgb_to_y(
            i32::from(px[2]),
            i32::from(px[1]),
            i32::from(px[0]),
            YUV_HALF,
        ));
        i += 1;
        bgr_pos += 3;
    }
}

/// Entry point of [`convert_bgr24_to_y_sse41`] for `webp_convert_bgr24_to_y`.
pub(crate) fn convert_bgr24_to_y(bgr: &[u8], y: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_bgr24_to_y` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { convert_bgr24_to_y_sse41(bgr, y, width) }
}

// upstream: src/dsp/yuv_sse41.c ConvertARGBToY_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(
    clippy::cast_possible_wrap,
    reason = "the masked channels of an ARGB value are below 2^31"
)]
fn convert_argb_to_y_sse41(argb: &[u32], y: &mut [u8], width: i32) {
    let max_width = width & !15;
    let mut i = 0;
    while i < max_width {
        let mut y0 = _mm_setzero_si128();
        let mut y1 = _mm_setzero_si128();
        let mut rgb = [_mm_setzero_si128(); 6];
        rgb32_packed_to_planar_sse41(&argb[to_usize(i)..], &mut rgb);
        convert_rgb_to_y_sse41(&rgb[0], &rgb[2], &rgb[4], &mut y0);
        convert_rgb_to_y_sse41(&rgb[1], &rgb[3], &rgb[5], &mut y1);
        store_16(_mm_packus_epi16(y0, y1), &mut y[to_usize(i)..]);
        i += 16;
    }
    while i < width {
        // left-over
        let p = argb[to_usize(i)];
        y[to_usize(i)] = store_u8(vp8rgb_to_y(
            ((p >> 16) & 0xff) as i32,
            ((p >> 8) & 0xff) as i32,
            (p & 0xff) as i32,
            YUV_HALF,
        ));
        i += 1;
    }
}

/// Entry point of [`convert_argb_to_y_sse41`] for `webp_convert_argb_to_y`.
pub(crate) fn convert_argb_to_y(argb: &[u32], y: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_argb_to_y` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { convert_argb_to_y_sse41(argb, y, width) }
}

// upstream: src/dsp/yuv_sse41.c HorizontalAddPack_SSE41
/// Horizontal add (doubled) of two 16b values, result is 16b.
/// in: A | B | C | D | ... -> out: 2*(A+B) | 2*(C+D) | ...
#[target_feature(enable = "sse4.1")]
#[inline]
fn horizontal_add_pack_sse41(a: &__m128i, b: &__m128i, out: &mut __m128i) {
    let k2 = _mm_set1_epi16(2);
    let c = _mm_madd_epi16(*a, k2);
    let d = _mm_madd_epi16(*b, k2);
    *out = _mm_packs_epi32(c, d);
}

/// `HorizontalAddPack_SSE41(&rgb[k], &rgb[k + 1], &rgb[k])` for k = 0, 2, 4,
/// with the inputs read before the output is written (deviation 2).
#[target_feature(enable = "sse4.1")]
#[inline]
fn horizontal_add_pack_planes(rgb: &mut [__m128i; 6]) {
    for k in [0, 2, 4] {
        let (a, b) = (rgb[k], rgb[k + 1]);
        horizontal_add_pack_sse41(&a, &b, &mut rgb[k]);
    }
}

// upstream: src/dsp/yuv_sse41.c ConvertARGBToUV_SSE41
#[target_feature(enable = "sse4.1")]
fn convert_argb_to_uv_sse41(
    argb: &[u32],
    u: &mut [u8],
    v: &mut [u8],
    src_width: i32,
    do_store: i32,
) {
    let max_width = src_width & !31;
    let mut uv_pos = 0;
    let mut i = 0;
    while i < max_width {
        let mut rgb = [_mm_setzero_si128(); 6];
        let mut u0 = _mm_setzero_si128();
        let mut v0 = _mm_setzero_si128();
        let mut u1 = _mm_setzero_si128();
        let mut v1 = _mm_setzero_si128();
        rgb32_packed_to_planar_sse41(&argb[to_usize(i)..], &mut rgb);
        horizontal_add_pack_planes(&mut rgb);
        convert_rgb_to_uv_sse41(&rgb[0], &rgb[2], &rgb[4], &mut u0, &mut v0);

        rgb32_packed_to_planar_sse41(&argb[to_usize(i + 16)..], &mut rgb);
        horizontal_add_pack_planes(&mut rgb);
        convert_rgb_to_uv_sse41(&rgb[0], &rgb[2], &rgb[4], &mut u1, &mut v1);

        u0 = _mm_packus_epi16(u0, u1);
        v0 = _mm_packus_epi16(v0, v1);
        if do_store == 0 {
            let prev_u = load_16(&u[uv_pos..]);
            let prev_v = load_16(&v[uv_pos..]);
            u0 = _mm_avg_epu8(u0, prev_u);
            v0 = _mm_avg_epu8(v0, prev_v);
        }
        store_16(u0, &mut u[uv_pos..]);
        store_16(v0, &mut v[uv_pos..]);
        i += 32;
        uv_pos += 16;
    }
    if i < src_width {
        // left-over
        webp_convert_argb_to_uv_c(
            &argb[to_usize(i)..],
            &mut u[uv_pos..],
            &mut v[uv_pos..],
            src_width - i,
            do_store,
        );
    }
}

/// Entry point of [`convert_argb_to_uv_sse41`] for `webp_convert_argb_to_uv`.
pub(crate) fn convert_argb_to_uv(
    argb: &[u32],
    u: &mut [u8],
    v: &mut [u8],
    src_width: i32,
    do_store: i32,
) {
    // SAFETY: `webp_convert_argb_to_uv` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { convert_argb_to_uv_sse41(argb, u, v, src_width, do_store) }
}

// upstream: src/dsp/yuv_sse41.c RGBA32PackedToPlanar_16b_SSE41
/// Convert 16 packed ARGB 16b-values to r[], g[], b[]
#[target_feature(enable = "sse4.1")]
#[inline]
fn rgba32_packed_to_planar_16b_sse41(
    rgbx: &[u16],
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let in0 = load_16(&rgbx[0..]); // r0 | g0 | b0 |x| r1 | g1 | b1 |x
    let in1 = load_16(&rgbx[8..]); // r2 | g2 | b2 |x| r3 | g3 | b3 |x
    let in2 = load_16(&rgbx[16..]); // r4 | ...
    let in3 = load_16(&rgbx[24..]); // r6 | ...
    // aarrggbb as 16-bit.
    let shuff0 = _mm_set_epi8(-1, -1, -1, -1, 13, 12, 5, 4, 11, 10, 3, 2, 9, 8, 1, 0);
    let shuff1 = _mm_set_epi8(13, 12, 5, 4, -1, -1, -1, -1, 11, 10, 3, 2, 9, 8, 1, 0);
    let a0 = _mm_shuffle_epi8(in0, shuff0);
    let a1 = _mm_shuffle_epi8(in1, shuff1);
    let a2 = _mm_shuffle_epi8(in2, shuff0);
    let a3 = _mm_shuffle_epi8(in3, shuff1);
    // R0R1G0G1
    // B0B1****
    // R2R3G2G3
    // B2B3****
    // (OR is used to free port 5 for the unpack)
    let b0 = _mm_unpacklo_epi32(a0, a1);
    let b1 = _mm_or_si128(a0, a1);
    let b2 = _mm_unpacklo_epi32(a2, a3);
    let b3 = _mm_or_si128(a2, a3);
    // Gather the channels.
    *r = _mm_unpacklo_epi64(b0, b2);
    *g = _mm_unpackhi_epi64(b0, b2);
    *b = _mm_unpackhi_epi64(b1, b3);
}

// upstream: src/dsp/yuv_sse41.c ConvertRGBA32ToUV_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_rgba32_to_uv_sse41(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    let max_width = width & !15;
    let last_rgb = 4 * to_usize(max_width);
    let mut rgb_pos = 0;
    let mut uv_pos = 0;
    while rgb_pos < last_rgb {
        let mut r = _mm_setzero_si128();
        let mut g = _mm_setzero_si128();
        let mut b = _mm_setzero_si128();
        let mut u0 = _mm_setzero_si128();
        let mut v0 = _mm_setzero_si128();
        let mut u1 = _mm_setzero_si128();
        let mut v1 = _mm_setzero_si128();
        rgba32_packed_to_planar_16b_sse41(&rgb[rgb_pos..], &mut r, &mut g, &mut b);
        convert_rgb_to_uv_sse41(&r, &g, &b, &mut u0, &mut v0);
        rgba32_packed_to_planar_16b_sse41(&rgb[rgb_pos + 32..], &mut r, &mut g, &mut b);
        convert_rgb_to_uv_sse41(&r, &g, &b, &mut u1, &mut v1);
        store_16(_mm_packus_epi16(u0, u1), &mut u[uv_pos..]);
        store_16(_mm_packus_epi16(v0, v1), &mut v[uv_pos..]);
        uv_pos += 16;
        rgb_pos += 2 * 32;
    }
    if max_width < width {
        // left-over
        webp_convert_rgba32_to_uv_c(
            &rgb[rgb_pos..],
            &mut u[uv_pos..],
            &mut v[uv_pos..],
            width - max_width,
        );
    }
}

/// Entry point of [`convert_rgba32_to_uv_sse41`] for
/// `webp_convert_rgba32_to_uv`.
pub(crate) fn convert_rgba32_to_uv(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_rgba32_to_uv` calls this entry point only after
    // `features().sse4_1` selected the SSE4.1 variant, so the CPU has SSE4.1.
    unsafe { convert_rgba32_to_uv_sse41(rgb, u, v, width) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/yuv_sse41.c WebPInitConvertARGBToYUVSSE41 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
