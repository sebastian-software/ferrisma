// libwebp v1.6.0, src/dsp/lossless_enc_sse41.c:
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
// SSE4.1 variant of methods for lossless encoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/lossless_enc_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE4.1 variants that `VP8LEncDspInitSSE41` assigns, behind the
//! dispatch functions of `src/dsp/lossless_enc.rs` (ADR 0008).
//! `VP8LEncDspInit` runs `VP8LEncDspInitSSE41` only inside its SSE2 branch,
//! so a dispatch function selects these entry points when both
//! `features().sse2` and `features().sse4_1` hold, after the AVX2 variant it
//! may prefer and before the SSE2 one. `VP8LEncDspInitSSE41` assigns:
//!
//! - `VP8LExtraCost`: `extra_cost`;
//! - `VP8LSubtractGreenFromBlueAndRed`: `subtract_green_from_blue_and_red`;
//! - `VP8LCollectColorBlueTransforms`: `collect_color_blue_transforms`;
//! - `VP8LCollectColorRedTransforms`: `collect_color_red_transforms`;
//! - the "SSE exports for AVX and above" of the last three, which replace
//!   the SSE2 ones and which `lossless_enc_avx2.rs` reaches as the `_sse41`
//!   functions directly; they are `pub(crate)` for it.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers** are slices as in `src/dsp/lossless_enc.rs`; `argb + y *
//!    stride` is the index of the row.
//! 3. **Macros** `CST_5b` and `MK_CST_16` are the functions of
//!    `lossless_enc_sse2.rs`; `HorizontalSum_SSE41` is a function.
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions': `ExtraCost_SSE41` asserts a length that is a
//! multiple of 8, as C does, and the library passes 40 or 24.
//!
//! # Checks not applicable
//!
//! 1. **`VP8LEncDspInitSSE41`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_and_si128, _mm_cvtsi128_si32, _mm_extract_epi8,
    _mm_hadd_epi32, _mm_madd_epi16, _mm_mulhi_epi16, _mm_mullo_epi32, _mm_set_epi8, _mm_set_epi32,
    _mm_set1_epi32, _mm_setr_epi8, _mm_shuffle_epi8, _mm_srli_epi32, _mm_srli_si128, _mm_sub_epi8,
    _mm_sub_epi16,
};

use crate::dsp::common_sse2::{loadu_si128, storeu_si128};
use crate::dsp::lossless_enc::{
    vp8l_collect_color_blue_transforms_c, vp8l_collect_color_red_transforms_c,
    vp8l_subtract_green_from_blue_and_red_c,
};
use crate::dsp::lossless_enc_sse2::{cst_5b, mk_cst_16};
use crate::dsp::lossless_sse2::idx;

// upstream: src/dsp/lossless_enc_sse41.c HorizontalSum_SSE41
#[target_feature(enable = "sse4.1")]
#[inline]
fn horizontal_sum_sse41(cost: __m128i) -> u32 {
    let cost = _mm_add_epi32(cost, _mm_srli_si128::<8>(cost));
    let cost = _mm_add_epi32(cost, _mm_srli_si128::<4>(cost));
    _mm_cvtsi128_si32(cost).cast_unsigned()
}

// upstream: src/dsp/lossless_enc_sse41.c ExtraCost_SSE41
#[target_feature(enable = "sse4.1")]
pub(crate) fn extra_cost_sse41(a: &[u32], length: i32) -> u32 {
    let mut cost = _mm_set_epi32(
        2_u32.wrapping_mul(a[7]).cast_signed(),
        2_u32.wrapping_mul(a[6]).cast_signed(),
        a[5].cast_signed(),
        a[4].cast_signed(),
    );
    debug_assert_eq!(length % 8, 0);

    let mut i: i32 = 8;
    while i + 8 <= length {
        let j = (i - 2) >> 1;
        let a0 = loadu_si128(&a[idx(i)..]);
        let a1 = loadu_si128(&a[idx(i + 4)..]);
        let w = _mm_set_epi32(j + 3, j + 2, j + 1, j);
        let a2 = _mm_hadd_epi32(a0, a1);
        let mul = _mm_mullo_epi32(a2, w);
        cost = _mm_add_epi32(mul, cost);
        i += 8;
    }
    horizontal_sum_sse41(cost)
}

// upstream: src/dsp/lossless_enc_sse41.c SubtractGreenFromBlueAndRed_SSE41
#[target_feature(enable = "sse4.1")]
pub(crate) fn subtract_green_from_blue_and_red_sse41(argb_data: &mut [u32], num_pixels: i32) {
    let k_cst_shuffle = _mm_set_epi8(-1, 13, -1, 13, -1, 9, -1, 9, -1, 5, -1, 5, -1, 1, -1, 1);
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let input = loadu_si128(&argb_data[idx(i)..]);
        let in_0g0g = _mm_shuffle_epi8(input, k_cst_shuffle);
        let out = _mm_sub_epi8(input, in_0g0g);
        storeu_si128(&mut argb_data[idx(i)..], out);
        i += 4;
    }
    // fallthrough and finish off with plain-C
    if i != num_pixels {
        vp8l_subtract_green_from_blue_and_red_c(&mut argb_data[idx(i)..], num_pixels - i);
    }
}

/// The start of row `y` of a tile, `argb + y * stride` in C.
#[inline]
fn row_start(y: i32, stride: i32) -> usize {
    usize::try_from(i64::from(y) * i64::from(stride)).expect("a tile row lies in its buffer")
}

/// `++histo[value]` for a byte that `_mm_extract_epi8` returned.
#[inline]
fn count(histo: &mut [u32], value: i32) {
    let index = idx(value);
    histo[index] = histo[index].wrapping_add(1);
}

// upstream: src/dsp/lossless_enc_sse41.c CollectColorBlueTransforms_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(non_snake_case, reason = "the C register names A1 to E")]
pub(crate) fn collect_color_blue_transforms_sse41(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_blue: i32,
    red_to_blue: i32,
    histo: &mut [u32],
) {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "the C (uint16_t) cast in CST_5b"
    )]
    let mult = mk_cst_16(cst_5b(red_to_blue as u8) + 256, cst_5b(green_to_blue as u8));
    let perm = _mm_setr_epi8(-1, 1, -1, 2, -1, 5, -1, 6, -1, 9, -1, 10, -1, 13, -1, 14);
    if tile_width >= 4 {
        for y in 0..tile_height {
            let src = row_start(y, stride);
            let A1 = loadu_si128(&argb[src..]);
            let B1 = _mm_shuffle_epi8(A1, perm);
            let C1 = _mm_mulhi_epi16(B1, mult);
            let D1 = _mm_sub_epi16(A1, C1);
            let mut E = _mm_add_epi16(_mm_srli_epi32::<16>(D1), D1);
            let mut x: i32 = 4;
            while x + 4 <= tile_width {
                let A2 = loadu_si128(&argb[src + idx(x)..]);
                count(histo, _mm_extract_epi8::<0>(E));
                let B2 = _mm_shuffle_epi8(A2, perm);
                count(histo, _mm_extract_epi8::<4>(E));
                let C2 = _mm_mulhi_epi16(B2, mult);
                count(histo, _mm_extract_epi8::<8>(E));
                let D2 = _mm_sub_epi16(A2, C2);
                count(histo, _mm_extract_epi8::<12>(E));
                E = _mm_add_epi16(_mm_srli_epi32::<16>(D2), D2);
                x += 4;
            }
            count(histo, _mm_extract_epi8::<0>(E));
            count(histo, _mm_extract_epi8::<4>(E));
            count(histo, _mm_extract_epi8::<8>(E));
            count(histo, _mm_extract_epi8::<12>(E));
        }
    }
    let left_over = tile_width & 3;
    if left_over > 0 {
        vp8l_collect_color_blue_transforms_c(
            &argb[idx(tile_width - left_over)..],
            stride,
            left_over,
            tile_height,
            green_to_blue,
            red_to_blue,
            histo,
        );
    }
}

// upstream: src/dsp/lossless_enc_sse41.c CollectColorRedTransforms_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(non_snake_case, reason = "the C register names A1 to D")]
pub(crate) fn collect_color_red_transforms_sse41(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_red: i32,
    histo: &mut [u32],
) {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "the C (uint16_t) cast in CST_5b"
    )]
    let mult = mk_cst_16(0, cst_5b(green_to_red as u8));
    let mask_g = _mm_set1_epi32(0x0000_ff00);
    if tile_width >= 4 {
        for y in 0..tile_height {
            let src = row_start(y, stride);
            let A1 = loadu_si128(&argb[src..]);
            let B1 = _mm_and_si128(A1, mask_g);
            let C1 = _mm_madd_epi16(B1, mult);
            let mut D = _mm_sub_epi16(A1, C1);
            let mut x: i32 = 4;
            while x + 4 <= tile_width {
                let A2 = loadu_si128(&argb[src + idx(x)..]);
                count(histo, _mm_extract_epi8::<2>(D));
                let B2 = _mm_and_si128(A2, mask_g);
                count(histo, _mm_extract_epi8::<6>(D));
                let C2 = _mm_madd_epi16(B2, mult);
                count(histo, _mm_extract_epi8::<10>(D));
                count(histo, _mm_extract_epi8::<14>(D));
                D = _mm_sub_epi16(A2, C2);
                x += 4;
            }
            count(histo, _mm_extract_epi8::<2>(D));
            count(histo, _mm_extract_epi8::<6>(D));
            count(histo, _mm_extract_epi8::<10>(D));
            count(histo, _mm_extract_epi8::<14>(D));
        }
    }
    let left_over = tile_width & 3;
    if left_over > 0 {
        vp8l_collect_color_red_transforms_c(
            &argb[idx(tile_width - left_over)..],
            stride,
            left_over,
            tile_height,
            green_to_red,
            histo,
        );
    }
}

//------------------------------------------------------------------------------
// Entry points (ADR 0008). The dispatch functions of `src/dsp/lossless_enc.rs`
// call them only after `features().sse2` and `features().sse4_1`, the answers
// of `VP8GetCPUInfo(kSSE2)` and `VP8GetCPUInfo(kSSE4_1)`, selected the SSE4.1
// variant.

/// Entry point of [`extra_cost_sse41`].
pub(crate) fn extra_cost(a: &[u32], length: i32) -> u32 {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1
    // variant, so the CPU has SSE4.1.
    unsafe { extra_cost_sse41(a, length) }
}

/// Entry point of [`subtract_green_from_blue_and_red_sse41`].
pub(crate) fn subtract_green_from_blue_and_red(argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1
    // variant, so the CPU has SSE4.1.
    unsafe { subtract_green_from_blue_and_red_sse41(argb_data, num_pixels) }
}

/// Entry point of [`collect_color_blue_transforms_sse41`].
pub(crate) fn collect_color_blue_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_blue: i32,
    red_to_blue: i32,
    histo: &mut [u32],
) {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1
    // variant, so the CPU has SSE4.1.
    unsafe {
        collect_color_blue_transforms_sse41(
            argb,
            stride,
            tile_width,
            tile_height,
            green_to_blue,
            red_to_blue,
            histo,
        );
    }
}

/// Entry point of [`collect_color_red_transforms_sse41`].
pub(crate) fn collect_color_red_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_red: i32,
    histo: &mut [u32],
) {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1
    // variant, so the CPU has SSE4.1.
    unsafe {
        collect_color_red_transforms_sse41(
            argb,
            stride,
            tile_width,
            tile_height,
            green_to_red,
            histo,
        );
    }
}
