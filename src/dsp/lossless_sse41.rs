// libwebp v1.6.0, src/dsp/lossless_sse41.c:
//
// Copyright 2021 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// SSE41 variant of methods for lossless decoder

//! Translation of `src/dsp/lossless_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE4.1 color inverse transform and the BGRA-to-RGB and BGRA-to-BGR
//! converters that `VP8LDspInitSSE41` assigns, behind the dispatch functions
//! of `src/dsp/lossless.rs` (ADR 0008). `VP8LDspInit` runs `VP8LDspInitSSE41`
//! only inside its SSE2 branch, so a dispatch function selects these entry
//! points when both `features().sse2` and `features().sse4_1` hold, before the
//! SSE2 variant. `VP8LDspInitSSE41` assigns:
//!
//! - `VP8LTransformColorInverse`: [`transform_color_inverse`];
//! - `VP8LConvertBGRAToRGB`: [`convert_bgra_to_rgb`];
//! - `VP8LConvertBGRAToBGR`: [`convert_bgra_to_bgr`];
//! - the "SSE exports for AVX and above" `VP8LTransformColorInverse_SSE` and
//!   `VP8LConvertBGRAToRGB_SSE`, which `lossless_avx2.rs` reaches directly:
//!   `transform_color_inverse_sse41` is `pub(crate)` for it.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **`ARGB_TO_RGB_SSE41`**, a macro over the caller's `in`, `out`,
//!    `num_pixels` and permutations, is a function that takes them by
//!    reference; `CST` is a function over a multiplier.
//! 3. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3).
//!
//! # Excluded undefined behavior
//!
//! None: the SSE4.1 functions compute the C functions' results for every
//! input (upstream's `-noasm` comparison relies on it, and the parity tests
//! cover the full value ranges).
//!
//! # Checks not applicable
//!
//! 1. **`VP8LDspInitSSE41`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.
//! 2. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_lossless.rs`).

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi8, _mm_blend_epi16, _mm_blendv_epi8, _mm_mulhi_epi16, _mm_set1_epi32,
    _mm_setr_epi8, _mm_shuffle_epi8, _mm_shuffle_epi32,
};

use crate::dsp::common_sse2::{loadu_si128, storeu_si128};
use crate::dsp::lossless::{
    VP8LMultipliers, vp8l_convert_bgra_to_bgr_c, vp8l_convert_bgra_to_rgb_c,
    vp8l_transform_color_inverse_c,
};
use crate::dsp::lossless_sse2::idx;

//------------------------------------------------------------------------------
// Color-space conversion functions

// upstream: src/dsp/lossless_sse41.c CST
/// sign-extended multiplying constants, pre-shifted by 5.
#[inline]
fn cst(x: u8) -> i32 {
    i32::from((u16::from(x) << 8).cast_signed()) >> 5 // sign-extend
}

// upstream: src/dsp/lossless_sse41.c TransformColorInverse_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(clippy::many_single_char_names, reason = "the C register names A to G")]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color_inverse"
)]
pub(crate) fn transform_color_inverse_sse41(
    m: &VP8LMultipliers,
    data: &mut [u32],
    src_pos: usize,
    num_pixels: i32,
    dst_pos: usize,
) {
    let mults_rb = _mm_set1_epi32(
        ((cst(m.green_to_red).cast_unsigned() << 16)
            | (cst(m.green_to_blue) & 0xffff).cast_unsigned())
        .cast_signed(),
    );
    let mults_b2 = _mm_set1_epi32(cst(m.red_to_blue));
    let mask_ag = _mm_set1_epi32(0xff00_ff00_u32.cast_signed());
    let perm1 = _mm_setr_epi8(-1, 1, -1, 1, -1, 5, -1, 5, -1, 9, -1, 9, -1, 13, -1, 13);
    let perm2 = _mm_setr_epi8(-1, 2, -1, -1, -1, 6, -1, -1, -1, 10, -1, -1, -1, 14, -1, -1);
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let a = loadu_si128(&data[src_pos + idx(i)..]);
        let b = _mm_shuffle_epi8(a, perm1); // argb -> g0g0
        let c = _mm_mulhi_epi16(b, mults_rb);
        let d = _mm_add_epi8(a, c);
        let e = _mm_shuffle_epi8(d, perm2);
        let f = _mm_mulhi_epi16(e, mults_b2);
        let g = _mm_add_epi8(d, f);
        let out = _mm_blendv_epi8(g, a, mask_ag);
        storeu_si128(&mut data[dst_pos + idx(i)..], out);
        i += 4;
    }
    // Fall-back to C-version for left-overs.
    if i != num_pixels {
        vp8l_transform_color_inverse_c(m, data, src_pos + idx(i), num_pixels - i, dst_pos + idx(i));
    }
}

/// Entry point of [`transform_color_inverse_sse41`] for
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
    // `features().sse2` and `features().sse4_1`, the answers of
    // `VP8GetCPUInfo(kSSE2)` and `VP8GetCPUInfo(kSSE4_1)`, selected the SSE4.1
    // variant, so the CPU has SSE4.1, which the function requires.
    unsafe { transform_color_inverse_sse41(m, data, src_pos, num_pixels, dst_pos) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/lossless_sse41.c ARGB_TO_RGB_SSE41
/// Converts blocks of 16 pixels from `src[*input..]` to 48 bytes at
/// `dst[*out..]` with the four permutations, while `*num_pixels >= 16`.
#[target_feature(enable = "sse4.1")]
#[inline]
fn argb_to_rgb_sse41(
    src: &[u32],
    input: &mut usize,
    dst: &mut [u8],
    out: &mut usize,
    num_pixels: &mut i32,
    perm: &[__m128i; 4],
) {
    while *num_pixels >= 16 {
        let in0 = loadu_si128(&src[*input..]);
        let in1 = loadu_si128(&src[*input + 4..]);
        let in2 = loadu_si128(&src[*input + 8..]);
        let in3 = loadu_si128(&src[*input + 12..]);
        let a0 = _mm_shuffle_epi8(in0, perm[0]);
        let a1 = _mm_shuffle_epi8(in1, perm[1]);
        let a2 = _mm_shuffle_epi8(in2, perm[2]);
        let a3 = _mm_shuffle_epi8(in3, perm[3]);
        let b0 = _mm_blend_epi16::<0xc0>(a0, a1);
        let b1 = _mm_blend_epi16::<0xf0>(a1, a2);
        let b2 = _mm_blend_epi16::<0xfc>(a2, a3);
        storeu_si128(&mut dst[*out..], b0);
        storeu_si128(&mut dst[*out + 16..], b1);
        storeu_si128(&mut dst[*out + 32..], b2);
        *input += 16;
        *out += 48;
        *num_pixels -= 16;
    }
}

// upstream: src/dsp/lossless_sse41.c ConvertBGRAToRGB_SSE41
#[target_feature(enable = "sse4.1")]
fn convert_bgra_to_rgb_sse41(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes
    let perm0 = _mm_setr_epi8(2, 1, 0, 6, 5, 4, 10, 9, 8, 14, 13, 12, -1, -1, -1, -1);
    let perm1 = _mm_shuffle_epi32::<0x39>(perm0);
    let perm2 = _mm_shuffle_epi32::<0x4e>(perm0);
    let perm3 = _mm_shuffle_epi32::<0x93>(perm0);

    argb_to_rgb_sse41(
        src,
        &mut input,
        dst,
        &mut out,
        &mut num_pixels,
        &[perm0, perm1, perm2, perm3],
    );

    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_rgb_c(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_rgb_sse41`] for `vp8l_convert_bgra_to_rgb`.
pub(crate) fn convert_bgra_to_rgb(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_rgb` calls this entry point only after
    // `features().sse2` and `features().sse4_1` selected the SSE4.1 variant,
    // so the CPU has SSE4.1, which the function requires.
    unsafe { convert_bgra_to_rgb_sse41(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse41.c ConvertBGRAToBGR_SSE41
#[target_feature(enable = "sse4.1")]
fn convert_bgra_to_bgr_sse41(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    let mut num_pixels = num_pixels;
    let mut input = 0; // C: in, in pixels
    let mut out = 0; // C: out, in bytes
    let perm0 = _mm_setr_epi8(0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, -1, -1, -1, -1);
    let perm1 = _mm_shuffle_epi32::<0x39>(perm0);
    let perm2 = _mm_shuffle_epi32::<0x4e>(perm0);
    let perm3 = _mm_shuffle_epi32::<0x93>(perm0);

    argb_to_rgb_sse41(
        src,
        &mut input,
        dst,
        &mut out,
        &mut num_pixels,
        &[perm0, perm1, perm2, perm3],
    );

    // left-overs
    if num_pixels > 0 {
        vp8l_convert_bgra_to_bgr_c(&src[input..], num_pixels, &mut dst[out..]);
    }
}

/// Entry point of [`convert_bgra_to_bgr_sse41`] for `vp8l_convert_bgra_to_bgr`.
pub(crate) fn convert_bgra_to_bgr(src: &[u32], num_pixels: i32, dst: &mut [u8]) {
    // SAFETY: `vp8l_convert_bgra_to_bgr` calls this entry point only after
    // `features().sse2` and `features().sse4_1` selected the SSE4.1 variant,
    // so the CPU has SSE4.1, which the function requires.
    unsafe { convert_bgra_to_bgr_sse41(src, num_pixels, dst) }
}

// upstream: src/dsp/lossless_sse41.c VP8LDspInitSSE41 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
