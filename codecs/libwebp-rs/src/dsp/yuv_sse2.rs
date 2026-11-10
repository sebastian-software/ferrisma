// libwebp v1.6.0, src/dsp/yuv_sse2.c:
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

//! Translation of `src/dsp/yuv_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE2 YUV to RGB row samplers and 32-pixel converters, and the RGB to
//! YUV converters of `yuv_sse2.c`, behind the dispatch functions of
//! `src/dsp/yuv.rs` (ADR 0008). Each `#[target_feature]` function keeps the
//! upstream name; each function that an `Init` function assigns to a pointer
//! has a safe `pub(crate)` entry point named after it without the suffix,
//! which the dispatch function calls after testing `features().sse2`.
//! `WebPInitSamplers` and `WebPInitConvertARGBToYUV` run their SSE2 and SSE4.1
//! `Init` functions independently, so an SSE4.1 assignment of `yuv_sse41.rs`
//! overrides these whenever `features().sse4_1` holds.
//!
//! - `WebPInitSamplersSSE2` assigns `WebPSamplers[MODE_RGB, MODE_RGBA,
//!   MODE_BGR, MODE_BGRA, MODE_ARGB]`: `yuv_to_rgb_row`,
//!   `yuv_to_rgba_row`, `yuv_to_bgr_row`, `yuv_to_bgra_row`,
//!   `yuv_to_argb_row` behind `webp_samplers`.
//! - `WebPInitConvertARGBToYUVSSE2` assigns `WebPConvertARGBToY`,
//!   `WebPConvertARGBToUV`, `WebPConvertRGB24ToY`, `WebPConvertBGR24ToY` and
//!   `WebPConvertRGBA32ToUV`: `convert_argb_to_y`, `convert_argb_to_uv`,
//!   `convert_rgb24_to_y`, `convert_bgr24_to_y`,
//!   `convert_rgba32_to_uv`.
//!
//! The 32-pixel converters `VP8YuvToRgba32_SSE2` and its siblings are
//! `pub(crate)` `#[target_feature]` functions for `upsampling_sse2.rs`, as
//! `yuv.h` declares them for `upsampling_sse2.c`.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers.** A pointer that the C loops advance is a slice and a
//!    position in it; the `__m128i*` parameters are references, and the
//!    six-register arrays of `RGB24PackedToPlanar_SSE2` and its siblings are
//!    `[__m128i; 6]`. `HorizontalAddPack_SSE2(&rgb[0], &rgb[1], &rgb[0])`
//!    reads its two inputs into copies first, as C reads them before it
//!    writes. The four `YUV444ToRGB_SSE2` or `YUV420ToRGB_SSE2` calls of the
//!    RGB and BGR functions, into `R0`..`R3`, `G0`..`G3` and `B0`..`B3`, are
//!    a loop that fills an array of the twelve registers.
//! 3. **`TRANSFORM` and `MK_CST_16`** are macros in C, because
//!    `_mm_srai_epi32` needs an immediate; `TRANSFORM` is a function with the
//!    shift as a const generic parameter, `MK_CST_16` a function. The
//!    finish-off loops of the row functions, identical in C up to the
//!    per-pixel conversion and the output step, share one helper.
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
//! 1. **`WEBP_SWAP_16BIT_CSP`.** The swapped 16-bit orders of
//!    `PackAndStore4444_SSE2` and `PackAndStore565_SSE2` are compiled only
//!    with `WEBP_SWAP_16BIT_CSP == 1`, which the reference configuration
//!    leaves 0 (WP4); only the standard order is translated.
//! 2. **`WebPInitSamplersSSE2` and `WebPInitConvertARGBToYUVSSE2`** only
//!    assign pointers (ADR 0003); their assignments are the selection in the
//!    dispatch functions listed above.
//! 3. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_yuv.rs`); the diagnostic
//!    `release-simd` oracle runs the WP4 probes against upstream's SSE2.
//! 4. **Comparison with the reference oracle outside the shared domain.**
//!    `ConvertRGBA32ToUV_SSE2` (`webp_convert_rgba32_to_uv`) multiplies the
//!    `uint16_t` samples in signed 16-bit lanes (`_mm_madd_epi16`), which read
//!    a value from 32768 up as negative, where `WebPConvertRGBA32ToUV_C`
//!    computes in `int`; upstream's SSE2 and C outputs differ there. Its
//!    callers (`picture_csp_enc.c`) pass sums of at most four pixels, 0 to
//!    1020, and the parity tests draw values up to 32767, where SSE2 and C
//!    agree. Outside that domain the port computes what upstream's SSE2 build
//!    computes: `tests/wp4_yuv.rs`, whose `rgba_uv` cases draw full-range
//!    samples, matched the `release-simd` oracle with the SIMD converters
//!    selected; its comparison with the reference oracle selects the C
//!    translation for exactly those cases through `VP8GetCPUInfo = NULL`, as
//!    `-noasm` does (ADR 0008). Every other function gives the bytes of the C
//!    function it replaces for every input (`yuv.h` defines the C conversions
//!    to emulate these 16-bit lanes, `MultHi` for `_mm_mulhi_epu16`).

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_adds_epu16, _mm_and_si128, _mm_avg_epu8,
    _mm_cvtsi32_si128, _mm_madd_epi16, _mm_mulhi_epu16, _mm_or_si128, _mm_packs_epi32,
    _mm_packus_epi16, _mm_set_epi16, _mm_set1_epi8, _mm_set1_epi16, _mm_set1_epi32,
    _mm_setzero_si128, _mm_slli_epi16, _mm_srai_epi16, _mm_srai_epi32, _mm_srli_epi16,
    _mm_sub_epi16, _mm_subs_epu16, _mm_unpackhi_epi8, _mm_unpackhi_epi16, _mm_unpackhi_epi64,
    _mm_unpacklo_epi8, _mm_unpacklo_epi16, _mm_unpacklo_epi64,
};

use crate::dsp::common_sse2::{
    Lane, loadl_epi64, loadu_si128, storeu_si128, vp8_planar_to24b_sse2, vp8l32b_to_planar_sse2,
};
use crate::dsp::yuv::{
    YUV_FIX, YUV_HALF, store_u8, vp8_yuv_to_argb, vp8_yuv_to_bgr, vp8_yuv_to_bgra, vp8_yuv_to_rgb,
    vp8_yuv_to_rgba, vp8rgb_to_y, webp_convert_argb_to_uv_c, webp_convert_rgba32_to_uv_c,
};
use crate::utils::utils::webp_mem_to_int32;

/// A non-negative C `int` length or index as `usize`.
fn to_usize(v: i32) -> usize {
    usize::try_from(v).expect("a non-negative length")
}

//-----------------------------------------------------------------------------
// Convert spans of 32 pixels to various RGB formats for the fancy upsampler.

// upstream: src/dsp/yuv_sse2.c ConvertYUV444ToRGB_SSE2
/// These constants are 14b fixed-point version of ITU-R BT.601 constants.
/// R = (19077 * y             + 26149 * v - 14234) >> 6
/// G = (19077 * y -  6419 * u - 13320 * v +  8708) >> 6
/// B = (19077 * y + 33050 * u             - 17685) >> 6
#[target_feature(enable = "sse2")]
#[inline]
fn convert_yuv444_to_rgb_sse2(
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

// upstream: src/dsp/yuv_sse2.c Load_HI_16_SSE2
/// Load the bytes into the *upper* part of 16b words. That's "<< 8", basically.
#[target_feature(enable = "sse2")]
#[inline]
fn load_hi_16_sse2(src: &[u8]) -> __m128i {
    let zero = _mm_setzero_si128();
    _mm_unpacklo_epi8(zero, loadl_epi64(src))
}

// upstream: src/dsp/yuv_sse2.c Load_UV_HI_8_SSE2
/// Load and replicate the U/V samples
#[target_feature(enable = "sse2")]
#[inline]
fn load_uv_hi_8_sse2(src: &[u8]) -> __m128i {
    let zero = _mm_setzero_si128();
    let tmp0 = _mm_cvtsi32_si128(webp_mem_to_int32(src));
    let tmp1 = _mm_unpacklo_epi8(zero, tmp0);
    _mm_unpacklo_epi16(tmp1, tmp1) // replicate samples
}

// upstream: src/dsp/yuv_sse2.c YUV444ToRGB_SSE2
/// Convert 32 samples of YUV444 to R/G/B
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv444_to_rgb_sse2(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let y0 = load_hi_16_sse2(y);
    let u0 = load_hi_16_sse2(u);
    let v0 = load_hi_16_sse2(v);
    convert_yuv444_to_rgb_sse2(&y0, &u0, &v0, r, g, b);
}

// upstream: src/dsp/yuv_sse2.c YUV420ToRGB_SSE2
/// Convert 32 samples of YUV420 to R/G/B
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv420_to_rgb_sse2(
    y: &[u8],
    u: &[u8],
    v: &[u8],
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let y0 = load_hi_16_sse2(y);
    let u0 = load_uv_hi_8_sse2(u);
    let v0 = load_uv_hi_8_sse2(v);
    convert_yuv444_to_rgb_sse2(&y0, &u0, &v0, r, g, b);
}

// upstream: src/dsp/yuv_sse2.c PackAndStore4_SSE2
/// Pack R/G/B/A results into 32b output.
#[target_feature(enable = "sse2")]
#[inline]
fn pack_and_store4_sse2(r: &__m128i, g: &__m128i, b: &__m128i, a: &__m128i, dst: &mut [u8]) {
    let rb = _mm_packus_epi16(*r, *b);
    let ga = _mm_packus_epi16(*g, *a);
    let rg = _mm_unpacklo_epi8(rb, ga);
    let ba = _mm_unpackhi_epi8(rb, ga);
    let rgba_lo = _mm_unpacklo_epi16(rg, ba);
    let rgba_hi = _mm_unpackhi_epi16(rg, ba);
    storeu_si128(&mut dst[0..], rgba_lo);
    storeu_si128(&mut dst[16..], rgba_hi);
}

// upstream: src/dsp/yuv_sse2.c PackAndStore4444_SSE2
/// Pack R/G/B/A results into 16b output (`WEBP_SWAP_16BIT_CSP == 0`).
#[target_feature(enable = "sse2")]
#[inline]
fn pack_and_store4444_sse2(r: &__m128i, g: &__m128i, b: &__m128i, a: &__m128i, dst: &mut [u8]) {
    let rg0 = _mm_packus_epi16(*r, *g);
    let ba0 = _mm_packus_epi16(*b, *a);
    let mask_0xf0 = _mm_set1_epi8(0xf0_u8.cast_signed());
    let rb1 = _mm_unpacklo_epi8(rg0, ba0); // rbrbrbrbrb...
    let ga1 = _mm_unpackhi_epi8(rg0, ba0); // gagagagaga...
    let rb2 = _mm_and_si128(rb1, mask_0xf0);
    let ga2 = _mm_srli_epi16::<4>(_mm_and_si128(ga1, mask_0xf0));
    let rgba4444 = _mm_or_si128(rb2, ga2);
    storeu_si128(dst, rgba4444);
}

// upstream: src/dsp/yuv_sse2.c PackAndStore565_SSE2
/// Pack R/G/B results into 16b output (`WEBP_SWAP_16BIT_CSP == 0`).
#[target_feature(enable = "sse2")]
#[inline]
fn pack_and_store565_sse2(r: &__m128i, g: &__m128i, b: &__m128i, dst: &mut [u8]) {
    let r0 = _mm_packus_epi16(*r, *r);
    let g0 = _mm_packus_epi16(*g, *g);
    let b0 = _mm_packus_epi16(*b, *b);
    let r1 = _mm_and_si128(r0, _mm_set1_epi8(0xf8_u8.cast_signed()));
    let b1 = _mm_and_si128(_mm_srli_epi16::<3>(b0), _mm_set1_epi8(0x1f));
    let g1 = _mm_srli_epi16::<5>(_mm_and_si128(g0, _mm_set1_epi8(0xe0_u8.cast_signed())));
    let g2 = _mm_slli_epi16::<3>(_mm_and_si128(g0, _mm_set1_epi8(0x1c)));
    let rg = _mm_or_si128(r1, g1);
    let gb = _mm_or_si128(g2, b1);
    let rgb565 = _mm_unpacklo_epi8(rg, gb);
    storeu_si128(dst, rgb565);
}

// upstream: src/dsp/yuv_sse2.c PlanarTo24b_SSE2
/// Pack the planar buffers
/// rrrr... rrrr... gggg... gggg... bbbb... bbbb....
/// triplet by triplet in the output buffer rgb as rgbrgbrgbrgb ...
#[target_feature(enable = "sse2")]
#[inline]
fn planar_to24b_sse2(
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
    vp8_planar_to24b_sse2(in0, in1, in2, in3, in4, in5);

    storeu_si128(&mut rgb[0..], *in0);
    storeu_si128(&mut rgb[16..], *in1);
    storeu_si128(&mut rgb[32..], *in2);
    storeu_si128(&mut rgb[48..], *in3);
    storeu_si128(&mut rgb[64..], *in4);
    storeu_si128(&mut rgb[80..], *in5);
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToRgba32_SSE2
/// Converts 32 YUV 4:4:4 samples to RGBA.
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
pub(crate) fn vp8_yuv_to_rgba32_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let k_alpha = _mm_set1_epi16(255);
    let mut r = _mm_setzero_si128();
    let mut g = _mm_setzero_si128();
    let mut b = _mm_setzero_si128();
    for n in (0..32).step_by(8) {
        yuv444_to_rgb_sse2(&y[n..], &u[n..], &v[n..], &mut r, &mut g, &mut b);
        pack_and_store4_sse2(&r, &g, &b, &k_alpha, &mut dst[n * 4..]);
    }
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToBgra32_SSE2
/// Converts 32 YUV 4:4:4 samples to BGRA.
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
pub(crate) fn vp8_yuv_to_bgra32_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let k_alpha = _mm_set1_epi16(255);
    let mut r = _mm_setzero_si128();
    let mut g = _mm_setzero_si128();
    let mut b = _mm_setzero_si128();
    for n in (0..32).step_by(8) {
        yuv444_to_rgb_sse2(&y[n..], &u[n..], &v[n..], &mut r, &mut g, &mut b);
        pack_and_store4_sse2(&b, &g, &r, &k_alpha, &mut dst[n * 4..]);
    }
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToArgb32_SSE2
/// Converts 32 YUV 4:4:4 samples to ARGB.
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
pub(crate) fn vp8_yuv_to_argb32_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let k_alpha = _mm_set1_epi16(255);
    let mut r = _mm_setzero_si128();
    let mut g = _mm_setzero_si128();
    let mut b = _mm_setzero_si128();
    for n in (0..32).step_by(8) {
        yuv444_to_rgb_sse2(&y[n..], &u[n..], &v[n..], &mut r, &mut g, &mut b);
        pack_and_store4_sse2(&k_alpha, &r, &g, &b, &mut dst[n * 4..]);
    }
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToRgba444432_SSE2
/// Converts 32 YUV 4:4:4 samples to RGBA-4444.
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
pub(crate) fn vp8_yuv_to_rgba444432_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let k_alpha = _mm_set1_epi16(255);
    let mut r = _mm_setzero_si128();
    let mut g = _mm_setzero_si128();
    let mut b = _mm_setzero_si128();
    for n in (0..32).step_by(8) {
        yuv444_to_rgb_sse2(&y[n..], &u[n..], &v[n..], &mut r, &mut g, &mut b);
        pack_and_store4444_sse2(&r, &g, &b, &k_alpha, &mut dst[n * 2..]);
    }
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToRgb56532_SSE2
/// Converts 32 YUV 4:4:4 samples to RGB-565.
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
pub(crate) fn vp8_yuv_to_rgb56532_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let mut r = _mm_setzero_si128();
    let mut g = _mm_setzero_si128();
    let mut b = _mm_setzero_si128();
    for n in (0..32).step_by(8) {
        yuv444_to_rgb_sse2(&y[n..], &u[n..], &v[n..], &mut r, &mut g, &mut b);
        pack_and_store565_sse2(&r, &g, &b, &mut dst[n * 2..]);
    }
}

/// The four `YUV444ToRGB_SSE2` calls of `VP8YuvToRgb32_SSE2` and
/// `VP8YuvToBgr32_SSE2`, at `y + 8 * k`, `u + 8 * k`, `v + 8 * k`, into
/// `R0`..`R3`, `G0`..`G3` and `B0`..`B3` (deviation 2).
#[target_feature(enable = "sse2")]
#[inline]
fn yuv444_to_rgb_4x8(y: &[u8], u: &[u8], v: &[u8]) -> [[__m128i; 3]; 4] {
    let mut out = [[_mm_setzero_si128(); 3]; 4];
    for (k, [r, g, b]) in out.iter_mut().enumerate() {
        let n = 8 * k;
        yuv444_to_rgb_sse2(&y[n..], &u[n..], &v[n..], r, g, b);
    }
    out
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToRgb32_SSE2
/// Converts 32 YUV 4:4:4 samples to RGB.
#[target_feature(enable = "sse2")]
pub(crate) fn vp8_yuv_to_rgb32_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let [[r0, g0, b0], [r1, g1, b1], [r2, g2, b2], [r3, g3, b3]] = yuv444_to_rgb_4x8(y, u, v);

    // Cast to 8b and store as RRRRGGGGBBBB.
    let mut rgb0 = _mm_packus_epi16(r0, r1);
    let mut rgb1 = _mm_packus_epi16(r2, r3);
    let mut rgb2 = _mm_packus_epi16(g0, g1);
    let mut rgb3 = _mm_packus_epi16(g2, g3);
    let mut rgb4 = _mm_packus_epi16(b0, b1);
    let mut rgb5 = _mm_packus_epi16(b2, b3);

    // Pack as RGBRGBRGBRGB.
    planar_to24b_sse2(
        &mut rgb0, &mut rgb1, &mut rgb2, &mut rgb3, &mut rgb4, &mut rgb5, dst,
    );
}

// upstream: src/dsp/yuv_sse2.c VP8YuvToBgr32_SSE2
/// Converts 32 YUV 4:4:4 samples to BGR.
#[target_feature(enable = "sse2")]
pub(crate) fn vp8_yuv_to_bgr32_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8]) {
    let [[r0, g0, b0], [r1, g1, b1], [r2, g2, b2], [r3, g3, b3]] = yuv444_to_rgb_4x8(y, u, v);

    // Cast to 8b and store as BBBBGGGGRRRR.
    let mut bgr0 = _mm_packus_epi16(b0, b1);
    let mut bgr1 = _mm_packus_epi16(b2, b3);
    let mut bgr2 = _mm_packus_epi16(g0, g1);
    let mut bgr3 = _mm_packus_epi16(g2, g3);
    let mut bgr4 = _mm_packus_epi16(r0, r1);
    let mut bgr5 = _mm_packus_epi16(r2, r3);

    // Pack as BGRBGRBGRBGR.
    planar_to24b_sse2(
        &mut bgr0, &mut bgr1, &mut bgr2, &mut bgr3, &mut bgr4, &mut bgr5, dst,
    );
}

//-----------------------------------------------------------------------------
// Arbitrary-length row conversion functions

/// The finish-off loop of the row functions: converts pixels `n..len` one
/// by one, advancing U and V after each odd pixel, as the C loops do with
/// `u += (n & 1)` (deviation 3).
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

// upstream: src/dsp/yuv_sse2.c YuvToRgbaRow_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv_to_rgba_row_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    let k_alpha = _mm_set1_epi16(255);
    let (mut y_pos, mut uv_pos, mut dst_pos) = (0, 0, 0);
    let mut n = 0;
    while n + 8 <= len {
        let mut r = _mm_setzero_si128();
        let mut g = _mm_setzero_si128();
        let mut b = _mm_setzero_si128();
        yuv420_to_rgb_sse2(
            &y[y_pos..],
            &u[uv_pos..],
            &v[uv_pos..],
            &mut r,
            &mut g,
            &mut b,
        );
        pack_and_store4_sse2(&r, &g, &b, &k_alpha, &mut dst[dst_pos..]);
        y_pos += 8;
        uv_pos += 4;
        n += 8;
        dst_pos += 32;
    }
    // Finish off
    finish_off(
        &y[y_pos..],
        &u[uv_pos..],
        &v[uv_pos..],
        &mut dst[dst_pos..],
        n,
        len,
        4,
        vp8_yuv_to_rgba,
    );
}

/// Entry point of [`yuv_to_rgba_row_sse2`] for `webp_samplers` (`MODE_RGBA`,
/// `MODE_rgbA`).
pub(crate) fn yuv_to_rgba_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has the function's only feature.
    unsafe { yuv_to_rgba_row_sse2(y, u, v, dst, len) }
}

// upstream: src/dsp/yuv_sse2.c YuvToBgraRow_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv_to_bgra_row_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    let k_alpha = _mm_set1_epi16(255);
    let (mut y_pos, mut uv_pos, mut dst_pos) = (0, 0, 0);
    let mut n = 0;
    while n + 8 <= len {
        let mut r = _mm_setzero_si128();
        let mut g = _mm_setzero_si128();
        let mut b = _mm_setzero_si128();
        yuv420_to_rgb_sse2(
            &y[y_pos..],
            &u[uv_pos..],
            &v[uv_pos..],
            &mut r,
            &mut g,
            &mut b,
        );
        pack_and_store4_sse2(&b, &g, &r, &k_alpha, &mut dst[dst_pos..]);
        y_pos += 8;
        uv_pos += 4;
        n += 8;
        dst_pos += 32;
    }
    // Finish off
    finish_off(
        &y[y_pos..],
        &u[uv_pos..],
        &v[uv_pos..],
        &mut dst[dst_pos..],
        n,
        len,
        4,
        vp8_yuv_to_bgra,
    );
}

/// Entry point of [`yuv_to_bgra_row_sse2`] for `webp_samplers` (`MODE_BGRA`,
/// `MODE_bgrA`).
pub(crate) fn yuv_to_bgra_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { yuv_to_bgra_row_sse2(y, u, v, dst, len) }
}

// upstream: src/dsp/yuv_sse2.c YuvToArgbRow_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn yuv_to_argb_row_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    let k_alpha = _mm_set1_epi16(255);
    let (mut y_pos, mut uv_pos, mut dst_pos) = (0, 0, 0);
    let mut n = 0;
    while n + 8 <= len {
        let mut r = _mm_setzero_si128();
        let mut g = _mm_setzero_si128();
        let mut b = _mm_setzero_si128();
        yuv420_to_rgb_sse2(
            &y[y_pos..],
            &u[uv_pos..],
            &v[uv_pos..],
            &mut r,
            &mut g,
            &mut b,
        );
        pack_and_store4_sse2(&k_alpha, &r, &g, &b, &mut dst[dst_pos..]);
        y_pos += 8;
        uv_pos += 4;
        n += 8;
        dst_pos += 32;
    }
    // Finish off
    finish_off(
        &y[y_pos..],
        &u[uv_pos..],
        &v[uv_pos..],
        &mut dst[dst_pos..],
        n,
        len,
        4,
        vp8_yuv_to_argb,
    );
}

/// Entry point of [`yuv_to_argb_row_sse2`] for `webp_samplers` (`MODE_ARGB`).
pub(crate) fn yuv_to_argb_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { yuv_to_argb_row_sse2(y, u, v, dst, len) }
}

/// The four `YUV420ToRGB_SSE2` calls of the RGB and BGR row functions, at
/// `y + 8 * k`, `u + 4 * k`, `v + 4 * k`, into `R0`..`R3`, `G0`..`G3` and
/// `B0`..`B3` (deviation 2).
#[target_feature(enable = "sse2")]
#[inline]
fn yuv420_to_rgb_4x8(y: &[u8], u: &[u8], v: &[u8]) -> [[__m128i; 3]; 4] {
    let mut out = [[_mm_setzero_si128(); 3]; 4];
    for (k, [r, g, b]) in out.iter_mut().enumerate() {
        yuv420_to_rgb_sse2(&y[8 * k..], &u[4 * k..], &v[4 * k..], r, g, b);
    }
    out
}

// upstream: src/dsp/yuv_sse2.c YuvToRgbRow_SSE2
#[target_feature(enable = "sse2")]
fn yuv_to_rgb_row_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
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
        planar_to24b_sse2(
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

/// Entry point of [`yuv_to_rgb_row_sse2`] for `webp_samplers` (`MODE_RGB`).
pub(crate) fn yuv_to_rgb_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { yuv_to_rgb_row_sse2(y, u, v, dst, len) }
}

// upstream: src/dsp/yuv_sse2.c YuvToBgrRow_SSE2
#[target_feature(enable = "sse2")]
fn yuv_to_bgr_row_sse2(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
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
        planar_to24b_sse2(
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

/// Entry point of [`yuv_to_bgr_row_sse2`] for `webp_samplers` (`MODE_BGR`).
pub(crate) fn yuv_to_bgr_row(y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32) {
    // SAFETY: `webp_samplers` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { yuv_to_bgr_row_sse2(y, u, v, dst, len) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/yuv_sse2.c WebPInitSamplersSSE2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)

//------------------------------------------------------------------------------
// RGB24/32 -> YUV converters

// upstream: src/dsp/yuv_sse2.c LOAD_16
/// Load eight 16b-words from *src.
#[target_feature(enable = "sse2")]
#[inline]
fn load_16<T: Lane>(src: &[T]) -> __m128i {
    loadu_si128(src)
}

// upstream: src/dsp/yuv_sse2.c STORE_16
/// Store either 16b-words into *dst
#[target_feature(enable = "sse2")]
#[inline]
fn store_16<T: Lane>(v: __m128i, dst: &mut [T]) {
    storeu_si128(dst, v);
}

// upstream: src/dsp/yuv_sse2.c RGB24PackedToPlanarHelper_SSE2
/// Function that inserts a value of the second half of the in buffer in between
/// every two char of the first half.
#[target_feature(enable = "sse2")]
#[inline]
fn rgb24_packed_to_planar_helper_sse2(input: &[__m128i; 6], out: &mut [__m128i; 6]) {
    out[0] = _mm_unpacklo_epi8(input[0], input[3]);
    out[1] = _mm_unpackhi_epi8(input[0], input[3]);
    out[2] = _mm_unpacklo_epi8(input[1], input[4]);
    out[3] = _mm_unpackhi_epi8(input[1], input[4]);
    out[4] = _mm_unpacklo_epi8(input[2], input[5]);
    out[5] = _mm_unpackhi_epi8(input[2], input[5]);
}

// upstream: src/dsp/yuv_sse2.c RGB24PackedToPlanar_SSE2
/// Unpack the 8b input rgbrgbrgbrgb ... as contiguous registers:
/// rrrr... rrrr... gggg... gggg... bbbb... bbbb....
/// Similar to `PlanarTo24bHelper()`, but in reverse order.
#[target_feature(enable = "sse2")]
#[inline]
fn rgb24_packed_to_planar_sse2(rgb: &[u8], out: &mut [__m128i; 6]) {
    let mut tmp = [
        loadu_si128(&rgb[0..]),
        loadu_si128(&rgb[16..]),
        loadu_si128(&rgb[32..]),
        loadu_si128(&rgb[48..]),
        loadu_si128(&rgb[64..]),
        loadu_si128(&rgb[80..]),
    ];

    rgb24_packed_to_planar_helper_sse2(&tmp, out);
    rgb24_packed_to_planar_helper_sse2(out, &mut tmp);
    rgb24_packed_to_planar_helper_sse2(&tmp, out);
    rgb24_packed_to_planar_helper_sse2(out, &mut tmp);
    rgb24_packed_to_planar_helper_sse2(&tmp, out);
}

// upstream: src/dsp/yuv_sse2.c RGB32PackedToPlanar_SSE2
/// Convert 8 packed ARGB to r[], g[], b[]
#[target_feature(enable = "sse2")]
#[inline]
fn rgb32_packed_to_planar_sse2(argb: &[u32], rgb: &mut [__m128i; 6]) {
    let zero = _mm_setzero_si128();
    let mut a0 = load_16(&argb[0..]);
    let mut a1 = load_16(&argb[4..]);
    let mut a2 = load_16(&argb[8..]);
    let mut a3 = load_16(&argb[12..]);
    vp8l32b_to_planar_sse2(&mut a0, &mut a1, &mut a2, &mut a3);
    rgb[0] = _mm_unpacklo_epi8(a1, zero);
    rgb[1] = _mm_unpackhi_epi8(a1, zero);
    rgb[2] = _mm_unpacklo_epi8(a2, zero);
    rgb[3] = _mm_unpackhi_epi8(a2, zero);
    rgb[4] = _mm_unpacklo_epi8(a3, zero);
    rgb[5] = _mm_unpackhi_epi8(a3, zero);
}

// upstream: src/dsp/yuv_sse2.c TRANSFORM
/// This macro computes `(RG * MULT_RG + GB * MULT_GB + ROUNDER) >> DESCALE_FIX`
/// It's a macro and not a function because we need to use immediate values with
/// `srai_epi32`, e.g.
#[target_feature(enable = "sse2")]
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

// upstream: src/dsp/yuv_sse2.c MK_CST_16
/// The `int` arguments convert to the `short` parameters of `_mm_set_epi16`,
/// as in C; every value used fits.
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the int arguments to short"
)]
fn mk_cst_16(a: i32, b: i32) -> __m128i {
    let (a, b) = (a as i16, b as i16);
    _mm_set_epi16(b, a, b, a, b, a, b, a)
}

// upstream: src/dsp/yuv_sse2.c ConvertRGBToY_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn convert_rgb_to_y_sse2(r: &__m128i, g: &__m128i, b: &__m128i, y: &mut __m128i) {
    let k_rg_y = mk_cst_16(16839, 33059 - 16384);
    let k_gb_y = mk_cst_16(16384, 6420);
    let k_half_y = _mm_set1_epi32((16 << YUV_FIX) + YUV_HALF);

    let rg_lo = _mm_unpacklo_epi16(*r, *g);
    let rg_hi = _mm_unpackhi_epi16(*r, *g);
    let gb_lo = _mm_unpacklo_epi16(*g, *b);
    let gb_hi = _mm_unpackhi_epi16(*g, *b);
    *y = transform::<YUV_FIX>(rg_lo, rg_hi, gb_lo, gb_hi, k_rg_y, k_gb_y, k_half_y);
}

// upstream: src/dsp/yuv_sse2.c ConvertRGBToUV_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_rgb_to_uv_sse2(r: &__m128i, g: &__m128i, b: &__m128i, u: &mut __m128i, v: &mut __m128i) {
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

// upstream: src/dsp/yuv_sse2.c ConvertRGB24ToY_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_rgb24_to_y_sse2(rgb: &[u8], y: &mut [u8], width: i32) {
    let max_width = width & !31;
    let mut rgb_pos = 0;
    let mut i = 0;
    while i < max_width {
        let mut rgb_plane = [_mm_setzero_si128(); 6];

        rgb24_packed_to_planar_sse2(&rgb[rgb_pos..], &mut rgb_plane);

        for j in 0..2 {
            let zero = _mm_setzero_si128();
            let mut y0 = _mm_setzero_si128();
            let mut y1 = _mm_setzero_si128();

            // Convert to 16-bit Y.
            let mut r = _mm_unpacklo_epi8(rgb_plane[j], zero);
            let mut g = _mm_unpacklo_epi8(rgb_plane[2 + j], zero);
            let mut b = _mm_unpacklo_epi8(rgb_plane[4 + j], zero);
            convert_rgb_to_y_sse2(&r, &g, &b, &mut y0);

            // Convert to 16-bit Y.
            r = _mm_unpackhi_epi8(rgb_plane[j], zero);
            g = _mm_unpackhi_epi8(rgb_plane[2 + j], zero);
            b = _mm_unpackhi_epi8(rgb_plane[4 + j], zero);
            convert_rgb_to_y_sse2(&r, &g, &b, &mut y1);

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

/// Entry point of [`convert_rgb24_to_y_sse2`] for `webp_convert_rgb24_to_y`.
pub(crate) fn convert_rgb24_to_y(rgb: &[u8], y: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_rgb24_to_y` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_rgb24_to_y_sse2(rgb, y, width) }
}

// upstream: src/dsp/yuv_sse2.c ConvertBGR24ToY_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_bgr24_to_y_sse2(bgr: &[u8], y: &mut [u8], width: i32) {
    let max_width = width & !31;
    let mut bgr_pos = 0;
    let mut i = 0;
    while i < max_width {
        let mut bgr_plane = [_mm_setzero_si128(); 6];

        rgb24_packed_to_planar_sse2(&bgr[bgr_pos..], &mut bgr_plane);

        for j in 0..2 {
            let zero = _mm_setzero_si128();
            let mut y0 = _mm_setzero_si128();
            let mut y1 = _mm_setzero_si128();

            // Convert to 16-bit Y.
            let mut b = _mm_unpacklo_epi8(bgr_plane[j], zero);
            let mut g = _mm_unpacklo_epi8(bgr_plane[2 + j], zero);
            let mut r = _mm_unpacklo_epi8(bgr_plane[4 + j], zero);
            convert_rgb_to_y_sse2(&r, &g, &b, &mut y0);

            // Convert to 16-bit Y.
            b = _mm_unpackhi_epi8(bgr_plane[j], zero);
            g = _mm_unpackhi_epi8(bgr_plane[2 + j], zero);
            r = _mm_unpackhi_epi8(bgr_plane[4 + j], zero);
            convert_rgb_to_y_sse2(&r, &g, &b, &mut y1);

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

/// Entry point of [`convert_bgr24_to_y_sse2`] for `webp_convert_bgr24_to_y`.
pub(crate) fn convert_bgr24_to_y(bgr: &[u8], y: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_bgr24_to_y` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_bgr24_to_y_sse2(bgr, y, width) }
}

// upstream: src/dsp/yuv_sse2.c ConvertARGBToY_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::cast_possible_wrap,
    reason = "the masked channels of an ARGB value are below 2^31"
)]
fn convert_argb_to_y_sse2(argb: &[u32], y: &mut [u8], width: i32) {
    let max_width = width & !15;
    let mut i = 0;
    while i < max_width {
        let mut y0 = _mm_setzero_si128();
        let mut y1 = _mm_setzero_si128();
        let mut rgb = [_mm_setzero_si128(); 6];
        rgb32_packed_to_planar_sse2(&argb[to_usize(i)..], &mut rgb);
        convert_rgb_to_y_sse2(&rgb[0], &rgb[2], &rgb[4], &mut y0);
        convert_rgb_to_y_sse2(&rgb[1], &rgb[3], &rgb[5], &mut y1);
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

/// Entry point of [`convert_argb_to_y_sse2`] for `webp_convert_argb_to_y`.
pub(crate) fn convert_argb_to_y(argb: &[u32], y: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_argb_to_y` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_argb_to_y_sse2(argb, y, width) }
}

// upstream: src/dsp/yuv_sse2.c HorizontalAddPack_SSE2
/// Horizontal add (doubled) of two 16b values, result is 16b.
/// in: A | B | C | D | ... -> out: 2*(A+B) | 2*(C+D) | ...
#[target_feature(enable = "sse2")]
#[inline]
fn horizontal_add_pack_sse2(a: &__m128i, b: &__m128i, out: &mut __m128i) {
    let k2 = _mm_set1_epi16(2);
    let c = _mm_madd_epi16(*a, k2);
    let d = _mm_madd_epi16(*b, k2);
    *out = _mm_packs_epi32(c, d);
}

/// `HorizontalAddPack_SSE2(&rgb[k], &rgb[k + 1], &rgb[k])` for k = 0, 2, 4,
/// with the inputs read before the output is written (deviation 2).
#[target_feature(enable = "sse2")]
#[inline]
fn horizontal_add_pack_planes(rgb: &mut [__m128i; 6]) {
    for k in [0, 2, 4] {
        let (a, b) = (rgb[k], rgb[k + 1]);
        horizontal_add_pack_sse2(&a, &b, &mut rgb[k]);
    }
}

// upstream: src/dsp/yuv_sse2.c ConvertARGBToUV_SSE2
#[target_feature(enable = "sse2")]
fn convert_argb_to_uv_sse2(
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
        rgb32_packed_to_planar_sse2(&argb[to_usize(i)..], &mut rgb);
        horizontal_add_pack_planes(&mut rgb);
        convert_rgb_to_uv_sse2(&rgb[0], &rgb[2], &rgb[4], &mut u0, &mut v0);

        rgb32_packed_to_planar_sse2(&argb[to_usize(i + 16)..], &mut rgb);
        horizontal_add_pack_planes(&mut rgb);
        convert_rgb_to_uv_sse2(&rgb[0], &rgb[2], &rgb[4], &mut u1, &mut v1);

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

/// Entry point of [`convert_argb_to_uv_sse2`] for `webp_convert_argb_to_uv`.
pub(crate) fn convert_argb_to_uv(
    argb: &[u32],
    u: &mut [u8],
    v: &mut [u8],
    src_width: i32,
    do_store: i32,
) {
    // SAFETY: `webp_convert_argb_to_uv` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_argb_to_uv_sse2(argb, u, v, src_width, do_store) }
}

// upstream: src/dsp/yuv_sse2.c RGBA32PackedToPlanar_16b_SSE2
/// Convert 16 packed ARGB 16b-values to r[], g[], b[]
#[target_feature(enable = "sse2")]
#[inline]
fn rgba32_packed_to_planar_16b_sse2(
    rgbx: &[u16],
    r: &mut __m128i,
    g: &mut __m128i,
    b: &mut __m128i,
) {
    let in0 = load_16(&rgbx[0..]); // r0 | g0 | b0 |x| r1 | g1 | b1 |x
    let in1 = load_16(&rgbx[8..]); // r2 | g2 | b2 |x| r3 | g3 | b3 |x
    let in2 = load_16(&rgbx[16..]); // r4 | ...
    let in3 = load_16(&rgbx[24..]); // r6 | ...
    // column-wise transpose
    let a0 = _mm_unpacklo_epi16(in0, in1);
    let a1 = _mm_unpackhi_epi16(in0, in1);
    let a2 = _mm_unpacklo_epi16(in2, in3);
    let a3 = _mm_unpackhi_epi16(in2, in3);
    let b0 = _mm_unpacklo_epi16(a0, a1); // r0 r1 r2 r3 | g0 g1 ..
    let b1 = _mm_unpackhi_epi16(a0, a1); // b0 b1 b2 b3 | x x x x
    let b2 = _mm_unpacklo_epi16(a2, a3); // r4 r5 r6 r7 | g4 g5 ..
    let b3 = _mm_unpackhi_epi16(a2, a3); // b4 b5 b6 b7 | x x x x
    *r = _mm_unpacklo_epi64(b0, b2);
    *g = _mm_unpackhi_epi64(b0, b2);
    *b = _mm_unpacklo_epi64(b1, b3);
}

// upstream: src/dsp/yuv_sse2.c ConvertRGBA32ToUV_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::many_single_char_names,
    reason = "the C names of the Y, U, V and R, G, B samples"
)]
fn convert_rgba32_to_uv_sse2(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
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
        rgba32_packed_to_planar_16b_sse2(&rgb[rgb_pos..], &mut r, &mut g, &mut b);
        convert_rgb_to_uv_sse2(&r, &g, &b, &mut u0, &mut v0);
        rgba32_packed_to_planar_16b_sse2(&rgb[rgb_pos + 32..], &mut r, &mut g, &mut b);
        convert_rgb_to_uv_sse2(&r, &g, &b, &mut u1, &mut v1);
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

/// Entry point of [`convert_rgba32_to_uv_sse2`] for
/// `webp_convert_rgba32_to_uv`.
pub(crate) fn convert_rgba32_to_uv(rgb: &[u16], u: &mut [u8], v: &mut [u8], width: i32) {
    // SAFETY: `webp_convert_rgba32_to_uv` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { convert_rgba32_to_uv_sse2(rgb, u, v, width) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/yuv_sse2.c WebPInitConvertARGBToYUVSSE2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
