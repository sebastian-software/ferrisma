// libwebp v1.6.0, src/dsp/lossless_enc_avx2.c:
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
// AVX2 variant of methods for lossless encoder
//
// Author: Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/dsp/lossless_enc_avx2.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The AVX2 variants that `VP8LEncDspInitAVX2` assigns, behind the dispatch
//! functions of `src/dsp/lossless_enc.rs` (ADR 0008). `VP8LEncDspInit` runs
//! `VP8LEncDspInitAVX2` only inside its SSE2 and SSE4.1 branches, so a
//! dispatch function selects these entry points when `features().sse2`,
//! `features().sse4_1` and `features().avx2` hold. `VP8LEncDspInitAVX2`
//! assigns `VP8LSubtractGreenFromBlueAndRed`, `VP8LTransformColor`,
//! `VP8LCollectColorBlueTransforms`, `VP8LCollectColorRedTransforms`,
//! `VP8LAddVector`, `VP8LAddVectorEq`, `VP8LCombinedShannonEntropy`,
//! `VP8LVectorMismatch`, `VP8LBundleColorMap` and `VP8LPredictorsSub[0..16]`
//! (with `PredictorSub0_AVX2` for the sentinels).
//!
//! The left-overs go to the "SSE exports" (`VP8LSubtractGreenFromBlueAndRed_SSE`
//! and the others) as upstream's `Init` functions leave them on an AVX2
//! machine: `VP8LEncDspInitSSE41` replaces the subtract-green and the two
//! collect exports with its SSE4.1 functions, `VP8LEncDspInitSSE2` sets the
//! others and copies its `VP8LPredictorsSub` table to `VP8LPredictorsSub_SSE`.
//! The translation calls those functions directly.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** through the checked `loadu_si256` and
//!    `storeu_si256` of `lossless_avx2.rs` and the 128-bit ones of
//!    `common_sse2.rs`; this file has no aligned access.
//!    `_mm256_storeu2_m128i(hi, lo, a)` is the two 128-bit stores of the low
//!    and the high half, which write the same bytes.
//! 2. **Pointers** are slices and positions as in `lossless_enc_sse2.rs`
//!    (its deviation 2).
//! 3. **Macros** as in `lossless_enc_sse2.rs` (its deviation 3).
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions': `AddVector_AVX2` and `AddVectorEq_AVX2`
//! assert `size >= 32` and an even size, as C does; the library passes
//! histogram sizes of at least 40.
//!
//! # Checks not applicable
//!
//! 1. **`VP8LEncDspInitAVX2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m256i, _mm256_add_epi8, _mm256_add_epi16, _mm256_add_epi32, _mm256_and_si256,
    _mm256_andnot_si256, _mm256_avg_epu8, _mm256_castsi256_si128, _mm256_cmpeq_epi32,
    _mm256_cmpgt_epi8, _mm256_cmpgt_epi16, _mm256_cmpgt_epi32, _mm256_extracti128_si256,
    _mm256_madd_epi16, _mm256_movemask_epi8, _mm256_mulhi_epi16, _mm256_mullo_epi16,
    _mm256_or_si256, _mm256_packs_epi16, _mm256_packs_epi32, _mm256_packus_epi16,
    _mm256_permutevar8x32_epi32, _mm256_sad_epu8, _mm256_set_epi8, _mm256_set_epi32,
    _mm256_set1_epi8, _mm256_set1_epi16, _mm256_set1_epi32, _mm256_setr_epi8, _mm256_setzero_si256,
    _mm256_shuffle_epi8, _mm256_slli_epi16, _mm256_slli_epi64, _mm256_srai_epi16,
    _mm256_srli_epi16, _mm256_srli_epi32, _mm256_sub_epi8, _mm256_sub_epi16, _mm256_unpackhi_epi8,
    _mm256_unpackhi_epi16, _mm256_unpackhi_epi32, _mm256_unpacklo_epi8, _mm256_unpacklo_epi16,
    _mm256_unpacklo_epi32, _mm256_xor_si256,
};

use crate::dsp::common_sse2::storeu_si128;
use crate::dsp::lossless::VP8LMultipliers;
use crate::dsp::lossless_avx2::{loadu_si256, storeu_si256};
use crate::dsp::lossless_common::vp8l_fast_s_log2;
use crate::dsp::lossless_enc_sse2::{
    Row, bundle_color_map_sse2, cst_5b, neighbor, predictor_sub0_sse2, predictor_sub1_sse2,
    predictor_sub2_sse2, predictor_sub3_sse2, predictor_sub4_sse2, predictor_sub5_sse2,
    predictor_sub6_sse2, predictor_sub7_sse2, predictor_sub8_sse2, predictor_sub9_sse2,
    predictor_sub10_sse2, predictor_sub11_sse2, predictor_sub12_sse2, predictor_sub13_sse2,
    transform_color_sse2,
};
use crate::dsp::lossless_enc_sse41::{
    collect_color_blue_transforms_sse41, collect_color_red_transforms_sse41,
    subtract_green_from_blue_and_red_sse41,
};
use crate::dsp::lossless_sse2::{at, idx};
use crate::utils::utils::bits_ctz;
use crate::webp::format_constants::ARGB_BLACK;

// upstream: src/dsp/lossless_enc_avx2.c MK_CST_16
#[target_feature(enable = "avx2")]
#[inline]
fn mk_cst_16(hi: i32, lo: i32) -> __m256i {
    _mm256_set1_epi32(((hi.cast_unsigned() << 16) | (lo & 0xffff).cast_unsigned()).cast_signed())
}

/// `VP8LPredictorsSub_SSE[mode]`: the SSE2 table that `VP8LEncDspInitSSE2`
/// copies, with `PredictorSub0_SSE2` for the sentinels.
#[target_feature(enable = "avx2")]
fn predictors_sub_sse(
    mode: usize,
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    match mode {
        1 => predictor_sub1_sse2(data, in_pos, upper_pos, num_pixels, out),
        2 => predictor_sub2_sse2(data, in_pos, upper_pos, num_pixels, out),
        3 => predictor_sub3_sse2(data, in_pos, upper_pos, num_pixels, out),
        4 => predictor_sub4_sse2(data, in_pos, upper_pos, num_pixels, out),
        5 => predictor_sub5_sse2(data, in_pos, upper_pos, num_pixels, out),
        6 => predictor_sub6_sse2(data, in_pos, upper_pos, num_pixels, out),
        7 => predictor_sub7_sse2(data, in_pos, upper_pos, num_pixels, out),
        8 => predictor_sub8_sse2(data, in_pos, upper_pos, num_pixels, out),
        9 => predictor_sub9_sse2(data, in_pos, upper_pos, num_pixels, out),
        10 => predictor_sub10_sse2(data, in_pos, upper_pos, num_pixels, out),
        11 => predictor_sub11_sse2(data, in_pos, upper_pos, num_pixels, out),
        12 => predictor_sub12_sse2(data, in_pos, upper_pos, num_pixels, out),
        13 => predictor_sub13_sse2(data, in_pos, upper_pos, num_pixels, out),
        _ => predictor_sub0_sse2(data, in_pos, upper_pos, num_pixels, out),
    }
}

// upstream: src/dsp/lossless_enc_avx2.c SubtractGreenFromBlueAndRed_AVX2
#[target_feature(enable = "avx2")]
pub(crate) fn subtract_green_from_blue_and_red_avx2(argb_data: &mut [u32], num_pixels: i32) {
    let k_cst_shuffle = _mm256_set_epi8(
        -1, 29, -1, 29, -1, 25, -1, 25, -1, 21, -1, 21, -1, 17, -1, 17, -1, 13, -1, 13, -1, 9, -1,
        9, -1, 5, -1, 5, -1, 1, -1, 1,
    );
    let mut i: i32 = 0;
    while i + 8 <= num_pixels {
        let input = loadu_si256(&argb_data[idx(i)..]); // argb
        let in_0g0g = _mm256_shuffle_epi8(input, k_cst_shuffle);
        let out = _mm256_sub_epi8(input, in_0g0g);
        storeu_si256(&mut argb_data[idx(i)..], out);
        i += 8;
    }
    // fallthrough and finish off with plain-SSE
    if i != num_pixels {
        subtract_green_from_blue_and_red_sse41(&mut argb_data[idx(i)..], num_pixels - i);
    }
}

// upstream: src/dsp/lossless_enc_avx2.c TransformColor_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names A to G")]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color"
)]
pub(crate) fn transform_color_avx2(m: &VP8LMultipliers, argb_data: &mut [u32], num_pixels: i32) {
    let mults_rb = mk_cst_16(cst_5b(m.green_to_red), cst_5b(m.green_to_blue));
    let mults_b2 = mk_cst_16(cst_5b(m.red_to_blue), 0);
    let mask_rb = _mm256_set1_epi32(0x00ff_00ff); // red-blue masks
    let k_cst_shuffle = _mm256_set_epi8(
        29, -1, 29, -1, 25, -1, 25, -1, 21, -1, 21, -1, 17, -1, 17, -1, 13, -1, 13, -1, 9, -1, 9,
        -1, 5, -1, 5, -1, 1, -1, 1, -1,
    );
    let mut i: i32 = 0;
    while i + 8 <= num_pixels {
        let input = loadu_si256(&argb_data[idx(i)..]); // argb
        let A = _mm256_shuffle_epi8(input, k_cst_shuffle); // g0g0
        let B = _mm256_mulhi_epi16(A, mults_rb); // x dr  x db1
        let C = _mm256_slli_epi16::<8>(input); // r 0   b   0
        let D = _mm256_mulhi_epi16(C, mults_b2); // x db2 0   0
        let E = _mm256_srli_epi32::<16>(D); // 0 0   x db2
        let F = _mm256_add_epi8(E, B); // x dr  x  db
        let G = _mm256_and_si256(F, mask_rb); // 0 dr  0  db
        let out = _mm256_sub_epi8(input, G);
        storeu_si256(&mut argb_data[idx(i)..], out);
        i += 8;
    }
    // fallthrough and finish off with plain-C
    if i != num_pixels {
        transform_color_sse2(m, &mut argb_data[idx(i)..], num_pixels - i);
    }
}

/// The start of row `y` of a tile, `argb + y * stride` in C.
#[inline]
fn row_start(y: i32, stride: i32) -> usize {
    usize::try_from(i64::from(y) * i64::from(stride)).expect("a tile row lies in its buffer")
}

// upstream: src/dsp/lossless_enc_avx2.c CollectColorBlueTransforms_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names A1 to E")]
pub(crate) fn collect_color_blue_transforms_avx2(
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
    let perm = _mm256_setr_epi8(
        -1, 1, -1, 2, -1, 5, -1, 6, -1, 9, -1, 10, -1, 13, -1, 14, -1, 17, -1, 18, -1, 21, -1, 22,
        -1, 25, -1, 26, -1, 29, -1, 30,
    );
    if tile_width >= 8 {
        for y in 0..tile_height {
            let mut values = [0_u8; 32];
            let src = row_start(y, stride);
            let A1 = loadu_si256(&argb[src..]);
            let B1 = _mm256_shuffle_epi8(A1, perm);
            let C1 = _mm256_mulhi_epi16(B1, mult);
            let D1 = _mm256_sub_epi16(A1, C1);
            let mut E = _mm256_add_epi16(_mm256_srli_epi32::<16>(D1), D1);
            let mut x: i32 = 8;
            while x + 8 <= tile_width {
                let A2 = loadu_si256(&argb[src + idx(x)..]);
                storeu_si256(&mut values, E);
                for &value in values.iter().step_by(4) {
                    let index = usize::from(value);
                    histo[index] = histo[index].wrapping_add(1);
                }
                let B2 = _mm256_shuffle_epi8(A2, perm);
                let C2 = _mm256_mulhi_epi16(B2, mult);
                let D2 = _mm256_sub_epi16(A2, C2);
                E = _mm256_add_epi16(_mm256_srli_epi32::<16>(D2), D2);
                x += 8;
            }
            storeu_si256(&mut values, E);
            for &value in values.iter().step_by(4) {
                let index = usize::from(value);
                histo[index] = histo[index].wrapping_add(1);
            }
        }
    }
    let left_over = tile_width & 7;
    if left_over > 0 {
        collect_color_blue_transforms_sse41(
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

// upstream: src/dsp/lossless_enc_avx2.c CollectColorRedTransforms_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names A1 to D")]
pub(crate) fn collect_color_red_transforms_avx2(
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
    let mask_g = _mm256_set1_epi32(0x0000_ff00);
    if tile_width >= 8 {
        for y in 0..tile_height {
            let mut values = [0_u8; 32];
            let src = row_start(y, stride);
            let A1 = loadu_si256(&argb[src..]);
            let B1 = _mm256_and_si256(A1, mask_g);
            let C1 = _mm256_madd_epi16(B1, mult);
            let mut D = _mm256_sub_epi16(A1, C1);
            let mut x: i32 = 8;
            while x + 8 <= tile_width {
                let A2 = loadu_si256(&argb[src + idx(x)..]);
                storeu_si256(&mut values, D);
                for &value in values[2..].iter().step_by(4) {
                    let index = usize::from(value);
                    histo[index] = histo[index].wrapping_add(1);
                }
                let B2 = _mm256_and_si256(A2, mask_g);
                let C2 = _mm256_madd_epi16(B2, mult);
                D = _mm256_sub_epi16(A2, C2);
                x += 8;
            }
            storeu_si256(&mut values, D);
            for &value in values[2..].iter().step_by(4) {
                let index = usize::from(value);
                histo[index] = histo[index].wrapping_add(1);
            }
        }
    }
    let left_over = tile_width & 7;
    if left_over > 0 {
        collect_color_red_transforms_sse41(
            &argb[idx(tile_width - left_over)..],
            stride,
            left_over,
            tile_height,
            green_to_red,
            histo,
        );
    }
}

// upstream: src/dsp/lossless_enc_avx2.c AddVector_AVX2
#[target_feature(enable = "avx2")]
pub(crate) fn add_vector_avx2(a: &[u32], b: &[u32], out: &mut [u32], size: i32) {
    let mut i: usize = 0;
    let mut size = size;
    let aligned_size = idx(size & !31);
    // Size is, at minimum, NUM_DISTANCE_CODES (40) and may be as large as
    // NUM_LITERAL_CODES (256) + NUM_LENGTH_CODES (24) + (0 or a non-zero power of
    // 2). See the usage in VP8LHistogramAdd().
    debug_assert!(size >= 32);
    debug_assert_eq!(size % 2, 0);

    loop {
        let a0 = loadu_si256(&a[i..]);
        let a1 = loadu_si256(&a[i + 8..]);
        let a2 = loadu_si256(&a[i + 16..]);
        let a3 = loadu_si256(&a[i + 24..]);
        let b0 = loadu_si256(&b[i..]);
        let b1 = loadu_si256(&b[i + 8..]);
        let b2 = loadu_si256(&b[i + 16..]);
        let b3 = loadu_si256(&b[i + 24..]);
        storeu_si256(&mut out[i..], _mm256_add_epi32(a0, b0));
        storeu_si256(&mut out[i + 8..], _mm256_add_epi32(a1, b1));
        storeu_si256(&mut out[i + 16..], _mm256_add_epi32(a2, b2));
        storeu_si256(&mut out[i + 24..], _mm256_add_epi32(a3, b3));
        i += 32;
        if i == aligned_size {
            break;
        }
    }

    if (size & 16) != 0 {
        let a0 = loadu_si256(&a[i..]);
        let a1 = loadu_si256(&a[i + 8..]);
        let b0 = loadu_si256(&b[i..]);
        let b1 = loadu_si256(&b[i + 8..]);
        storeu_si256(&mut out[i..], _mm256_add_epi32(a0, b0));
        storeu_si256(&mut out[i + 8..], _mm256_add_epi32(a1, b1));
        i += 16;
    }

    size &= 15;
    if size == 8 {
        let a0 = loadu_si256(&a[i..]);
        let b0 = loadu_si256(&b[i..]);
        storeu_si256(&mut out[i..], _mm256_add_epi32(a0, b0));
    } else {
        while size != 0 {
            size -= 1;
            out[i] = a[i].wrapping_add(b[i]);
            i += 1;
        }
    }
}

// upstream: src/dsp/lossless_enc_avx2.c AddVectorEq_AVX2
#[target_feature(enable = "avx2")]
pub(crate) fn add_vector_eq_avx2(a: &[u32], out: &mut [u32], size: i32) {
    let mut i: usize = 0;
    let mut size = size;
    let aligned_size = idx(size & !31);
    // Size is, at minimum, NUM_DISTANCE_CODES (40) and may be as large as
    // NUM_LITERAL_CODES (256) + NUM_LENGTH_CODES (24) + (0 or a non-zero power of
    // 2). See the usage in VP8LHistogramAdd().
    debug_assert!(size >= 32);
    debug_assert_eq!(size % 2, 0);

    loop {
        let a0 = loadu_si256(&a[i..]);
        let a1 = loadu_si256(&a[i + 8..]);
        let a2 = loadu_si256(&a[i + 16..]);
        let a3 = loadu_si256(&a[i + 24..]);
        let b0 = loadu_si256(&out[i..]);
        let b1 = loadu_si256(&out[i + 8..]);
        let b2 = loadu_si256(&out[i + 16..]);
        let b3 = loadu_si256(&out[i + 24..]);
        storeu_si256(&mut out[i..], _mm256_add_epi32(a0, b0));
        storeu_si256(&mut out[i + 8..], _mm256_add_epi32(a1, b1));
        storeu_si256(&mut out[i + 16..], _mm256_add_epi32(a2, b2));
        storeu_si256(&mut out[i + 24..], _mm256_add_epi32(a3, b3));
        i += 32;
        if i == aligned_size {
            break;
        }
    }

    if (size & 16) != 0 {
        let a0 = loadu_si256(&a[i..]);
        let a1 = loadu_si256(&a[i + 8..]);
        let b0 = loadu_si256(&out[i..]);
        let b1 = loadu_si256(&out[i + 8..]);
        storeu_si256(&mut out[i..], _mm256_add_epi32(a0, b0));
        storeu_si256(&mut out[i + 8..], _mm256_add_epi32(a1, b1));
        i += 16;
    }

    size &= 15;
    if size == 8 {
        let a0 = loadu_si256(&a[i..]);
        let b0 = loadu_si256(&out[i..]);
        storeu_si256(&mut out[i..], _mm256_add_epi32(a0, b0));
    } else {
        while size != 0 {
            size -= 1;
            out[i] = out[i].wrapping_add(a[i]);
            i += 1;
        }
    }
}

// upstream: src/dsp/lossless_enc_avx2.c CombinedShannonEntropy_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C parameter names X and Y")]
pub(crate) fn combined_shannon_entropy_avx2(X: &[u32], Y: &[u32]) -> u64 {
    let mut retval: u64 = 0;
    let mut sum_x: u32 = 0;
    let mut sum_xy: u32 = 0;
    let zero = _mm256_setzero_si256();

    let mut i: usize = 0;
    while i < 256 {
        let x0 = loadu_si256(&X[i..]);
        let y0 = loadu_si256(&Y[i..]);
        let x1 = loadu_si256(&X[i + 8..]);
        let y1 = loadu_si256(&Y[i + 8..]);
        let x2 = loadu_si256(&X[i + 16..]);
        let y2 = loadu_si256(&Y[i + 16..]);
        let x3 = loadu_si256(&X[i + 24..]);
        let y3 = loadu_si256(&Y[i + 24..]);
        let x4 = _mm256_packs_epi16(_mm256_packs_epi32(x0, x1), _mm256_packs_epi32(x2, x3));
        let y4 = _mm256_packs_epi16(_mm256_packs_epi32(y0, y1), _mm256_packs_epi32(y2, y3));
        // Packed pixels are actually in order: ... 17 16 12 11 10 9 8 3 2 1 0
        let x5 = _mm256_permutevar8x32_epi32(x4, _mm256_set_epi32(7, 3, 6, 2, 5, 1, 4, 0));
        let y5 = _mm256_permutevar8x32_epi32(y4, _mm256_set_epi32(7, 3, 6, 2, 5, 1, 4, 0));
        let mx: u32 = _mm256_movemask_epi8(_mm256_cmpgt_epi8(x5, zero)).cast_unsigned();
        let mut my: u32 = _mm256_movemask_epi8(_mm256_cmpgt_epi8(y5, zero)).cast_unsigned() | mx;
        while my != 0 {
            let j = bits_ctz(my);
            if (mx >> j) & 1 != 0 {
                let x = X[i + idx(j)];
                sum_xy = sum_xy.wrapping_add(x);
                retval = retval.wrapping_add(vp8l_fast_s_log2(x));
            }
            let xy = X[i + idx(j)].wrapping_add(Y[i + idx(j)]);
            sum_x = sum_x.wrapping_add(xy);
            retval = retval.wrapping_add(vp8l_fast_s_log2(xy));
            my &= my - 1;
        }
        i += 32;
    }
    vp8l_fast_s_log2(sum_x)
        .wrapping_add(vp8l_fast_s_log2(sum_xy))
        .wrapping_sub(retval)
}

// upstream: src/dsp/lossless_enc_avx2.c VectorMismatch_AVX2
#[target_feature(enable = "avx2")]
pub(crate) fn vector_mismatch_avx2(array1: &[u32], array2: &[u32], length: i32) -> i32 {
    let mut match_len: i32;

    if length >= 24 {
        let mut a0 = loadu_si256(&array1[0..]);
        let mut a1 = loadu_si256(&array2[0..]);
        match_len = 0;
        loop {
            // Loop unrolling and early load both provide a speedup of 10% for the
            // current function. Also, max_limit can be MAX_LENGTH=4096 at most.
            let cmp_a = _mm256_cmpeq_epi32(a0, a1);
            let b0 = loadu_si256(&array1[idx(match_len + 8)..]);
            let b1 = loadu_si256(&array2[idx(match_len + 8)..]);
            if _mm256_movemask_epi8(cmp_a).cast_unsigned() != 0xffff_ffff {
                break;
            }
            match_len += 8;

            {
                let cmp_b = _mm256_cmpeq_epi32(b0, b1);
                a0 = loadu_si256(&array1[idx(match_len + 8)..]);
                a1 = loadu_si256(&array2[idx(match_len + 8)..]);
                if _mm256_movemask_epi8(cmp_b).cast_unsigned() != 0xffff_ffff {
                    break;
                }
                match_len += 8;
            }
            if match_len + 24 >= length {
                break;
            }
        }
    } else {
        match_len = 0;
        // Unroll the potential first two loops.
        if length >= 8
            && _mm256_movemask_epi8(_mm256_cmpeq_epi32(
                loadu_si256(&array1[0..]),
                loadu_si256(&array2[0..]),
            ))
            .cast_unsigned()
                == 0xffff_ffff
        {
            match_len = 8;
            if length >= 16
                && _mm256_movemask_epi8(_mm256_cmpeq_epi32(
                    loadu_si256(&array1[8..]),
                    loadu_si256(&array2[8..]),
                ))
                .cast_unsigned()
                    == 0xffff_ffff
            {
                match_len = 16;
            }
        }
    }

    while match_len < length && array1[idx(match_len)] == array2[idx(match_len)] {
        match_len += 1;
    }
    match_len
}

/// `_mm256_storeu2_m128i(&dst[hi], &dst[lo], a)`: the low half of `a` at
/// `lo`, the high half at `hi` (deviation 1).
#[target_feature(enable = "avx2")]
#[inline]
fn storeu2_m128i(dst: &mut [u32], hi: usize, lo: usize, a: __m256i) {
    storeu_si128(&mut dst[lo..], _mm256_castsi256_si128(a));
    storeu_si128(&mut dst[hi..], _mm256_extracti128_si256::<1>(a));
}

// upstream: src/dsp/lossless_enc_avx2.c BundleColorMap_AVX2
#[target_feature(enable = "avx2")]
pub(crate) fn bundle_color_map_avx2(row: &[u8], width: i32, xbits: i32, dst: &mut [u32]) {
    let mut x: i32 = 0;
    let mut d: usize = 0; // the C `dst` pointer as an index
    debug_assert!(xbits >= 0);
    debug_assert!(xbits <= 3);
    match xbits {
        0 => {
            let ff = _mm256_set1_epi16(0xff00_u16.cast_signed());
            let zero = _mm256_setzero_si256();
            // Store 0xff000000 | (row[x] << 8).
            while x + 32 <= width {
                let input = loadu_si256(&row[idx(x)..]);
                let in_lo = _mm256_unpacklo_epi8(zero, input);
                let dst0 = _mm256_unpacklo_epi16(in_lo, ff);
                let dst1 = _mm256_unpackhi_epi16(in_lo, ff);
                let in_hi = _mm256_unpackhi_epi8(zero, input);
                let dst2 = _mm256_unpacklo_epi16(in_hi, ff);
                let dst3 = _mm256_unpackhi_epi16(in_hi, ff);
                storeu2_m128i(dst, d + 16, d, dst0);
                storeu2_m128i(dst, d + 20, d + 4, dst1);
                storeu2_m128i(dst, d + 24, d + 8, dst2);
                storeu2_m128i(dst, d + 28, d + 12, dst3);
                x += 32;
                d += 32;
            }
        }
        1 => {
            let ff = _mm256_set1_epi16(0xff00_u16.cast_signed());
            let mul = _mm256_set1_epi16(0x110);
            while x + 32 <= width {
                // 0a0b | (where a/b are 4 bits).
                let input = loadu_si256(&row[idx(x)..]);
                let tmp = _mm256_mullo_epi16(input, mul); // aba0
                let pack = _mm256_and_si256(tmp, ff); // ab00
                let dst0 = _mm256_unpacklo_epi16(pack, ff);
                let dst1 = _mm256_unpackhi_epi16(pack, ff);
                storeu2_m128i(dst, d + 8, d, dst0);
                storeu2_m128i(dst, d + 12, d + 4, dst1);
                x += 32;
                d += 16;
            }
        }
        2 => {
            let mask_or = _mm256_set1_epi32(0xff00_0000_u32.cast_signed());
            let mul_cst = _mm256_set1_epi16(0x0104);
            let mask_mul = _mm256_set1_epi16(0x0f00);
            while x + 32 <= width {
                // 000a000b000c000d | (where a/b/c/d are 2 bits).
                let input = loadu_si256(&row[idx(x)..]);
                let mul = _mm256_mullo_epi16(input, mul_cst); // 00ab00b000cd00d0
                let tmp = _mm256_and_si256(mul, mask_mul); //  00ab000000cd0000
                let shift = _mm256_srli_epi32::<12>(tmp); // 00000000ab000000
                let pack = _mm256_or_si256(shift, tmp); // 00000000abcd0000
                // Convert to 0xff00**00.
                let res = _mm256_or_si256(pack, mask_or);
                storeu_si256(&mut dst[d..], res);
                x += 32;
                d += 8;
            }
        }
        _ => {
            debug_assert_eq!(xbits, 3);
            while x + 32 <= width {
                // 0000000a00000000b... | (where a/b are 1 bit).
                let input = loadu_si256(&row[idx(x)..]);
                let shift = _mm256_slli_epi64::<7>(input);
                let mv = _mm256_movemask_epi8(shift).cast_unsigned();
                dst[d] = 0xff00_0000 | ((mv & 0xff) << 8);
                dst[d + 1] = 0xff00_0000 | (mv & 0xff00);
                dst[d + 2] = 0xff00_0000 | ((mv & 0x00ff_0000) >> 8);
                dst[d + 3] = 0xff00_0000 | ((mv & 0xff00_0000) >> 16);
                x += 32;
                d += 4;
            }
        }
    }
    if x != width {
        bundle_color_map_sse2(&row[idx(x)..], width - x, xbits, &mut dst[d..]);
    }
}

// upstream: src/dsp/lossless_enc_avx2.c Average2_m256i
#[target_feature(enable = "avx2")]
#[inline]
fn average2_m256i(a0: __m256i, a1: __m256i) -> __m256i {
    // (a + b) >> 1 = ((a + b + 1) >> 1) - ((a ^ b) & 1)
    let ones = _mm256_set1_epi8(1);
    let avg1 = _mm256_avg_epu8(a0, a1);
    let one = _mm256_and_si256(_mm256_xor_si256(a0, a1), ones);
    _mm256_sub_epi8(avg1, one)
}

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub0_AVX2
#[target_feature(enable = "avx2")]
pub(crate) fn predictor_sub0_avx2(
    data: &[u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    let black = _mm256_set1_epi32(ARGB_BLACK.cast_signed());
    while i + 8 <= num_pixels {
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let res = _mm256_sub_epi8(src, black);
        storeu_si256(&mut out[idx(i)..], res);
        i += 8;
    }
    if i != num_pixels {
        // C: upper is NULL.
        predictors_sub_sse(
            0,
            data,
            in_pos + idx(i),
            0,
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_avx2.c GENERATE_PREDICTOR_1
/// The residual against one neighbor, given by its row and its offset from
/// pixel `i`.
macro_rules! generate_predictor_1 {
    ($x:literal, $name:ident, $row:ident, $delta:literal) => {
        #[target_feature(enable = "avx2")]
        pub(crate) fn $name(
            data: &[u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out: &mut [u32],
        ) {
            let mut i: i32 = 0;
            while i + 8 <= num_pixels {
                let src = loadu_si256(&data[in_pos + idx(i)..]);
                let pred = loadu_si256(&data[neighbor(Row::$row, $delta, in_pos, upper_pos, i)..]);
                let res = _mm256_sub_epi8(src, pred);
                storeu_si256(&mut out[idx(i)..], res);
                i += 8;
            }
            if i != num_pixels {
                predictors_sub_sse(
                    $x,
                    data,
                    in_pos + idx(i),
                    upper_pos + idx(i),
                    num_pixels - i,
                    &mut out[idx(i)..],
                );
            }
        }
    };
}

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub1_AVX2
generate_predictor_1!(1, predictor_sub1_avx2, In, -1); // Predictor1: L
// upstream: src/dsp/lossless_enc_avx2.c PredictorSub2_AVX2
generate_predictor_1!(2, predictor_sub2_avx2, Upper, 0); // Predictor2: T
// upstream: src/dsp/lossless_enc_avx2.c PredictorSub3_AVX2
generate_predictor_1!(3, predictor_sub3_avx2, Upper, 1); // Predictor3: TR
// upstream: src/dsp/lossless_enc_avx2.c PredictorSub4_AVX2
generate_predictor_1!(4, predictor_sub4_avx2, Upper, -1); // Predictor4: TL

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub5_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names L, T and TR")]
pub(crate) fn predictor_sub5_avx2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    while i + 8 <= num_pixels {
        let L = loadu_si256(&data[at(in_pos + idx(i), -1)..]);
        let T = loadu_si256(&data[upper_pos + idx(i)..]);
        let TR = loadu_si256(&data[upper_pos + idx(i) + 1..]);
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let avg = average2_m256i(L, TR);
        let pred = average2_m256i(avg, T);
        let res = _mm256_sub_epi8(src, pred);
        storeu_si256(&mut out[idx(i)..], res);
        i += 8;
    }
    if i != num_pixels {
        predictors_sub_sse(
            5,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_avx2.c GENERATE_PREDICTOR_2
/// The residual against the average of two neighbors, each given by its row
/// and its offset from pixel `i`.
macro_rules! generate_predictor_2 {
    ($x:literal, $name:ident, ($a_row:ident, $a_delta:literal), ($b_row:ident, $b_delta:literal)) => {
        #[target_feature(enable = "avx2")]
        pub(crate) fn $name(
            data: &[u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out: &mut [u32],
        ) {
            let mut i: i32 = 0;
            while i + 8 <= num_pixels {
                let t_a =
                    loadu_si256(&data[neighbor(Row::$a_row, $a_delta, in_pos, upper_pos, i)..]);
                let t_b =
                    loadu_si256(&data[neighbor(Row::$b_row, $b_delta, in_pos, upper_pos, i)..]);
                let src = loadu_si256(&data[in_pos + idx(i)..]);
                let pred = average2_m256i(t_a, t_b);
                let res = _mm256_sub_epi8(src, pred);
                storeu_si256(&mut out[idx(i)..], res);
                i += 8;
            }
            if i != num_pixels {
                predictors_sub_sse(
                    $x,
                    data,
                    in_pos + idx(i),
                    upper_pos + idx(i),
                    num_pixels - i,
                    &mut out[idx(i)..],
                );
            }
        }
    };
}

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub6_AVX2
generate_predictor_2!(6, predictor_sub6_avx2, (In, -1), (Upper, -1)); // Predictor6: avg(L, TL)
// upstream: src/dsp/lossless_enc_avx2.c PredictorSub7_AVX2
generate_predictor_2!(7, predictor_sub7_avx2, (In, -1), (Upper, 0)); // Predictor7: avg(L, T)
// upstream: src/dsp/lossless_enc_avx2.c PredictorSub8_AVX2
generate_predictor_2!(8, predictor_sub8_avx2, (Upper, -1), (Upper, 0)); // Predictor8: avg(TL, T)
// upstream: src/dsp/lossless_enc_avx2.c PredictorSub9_AVX2
generate_predictor_2!(9, predictor_sub9_avx2, (Upper, 0), (Upper, 1)); // Predictor9: average(T, TR)

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub10_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names L, TL, T and TR")]
pub(crate) fn predictor_sub10_avx2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    while i + 8 <= num_pixels {
        let L = loadu_si256(&data[at(in_pos + idx(i), -1)..]);
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let TL = loadu_si256(&data[at(upper_pos + idx(i), -1)..]);
        let T = loadu_si256(&data[upper_pos + idx(i)..]);
        let TR = loadu_si256(&data[upper_pos + idx(i) + 1..]);
        let avg_ttr = average2_m256i(T, TR);
        let avg_ltl = average2_m256i(L, TL);
        let avg = average2_m256i(avg_ttr, avg_ltl);
        let res = _mm256_sub_epi8(src, avg);
        storeu_si256(&mut out[idx(i)..], res);
        i += 8;
    }
    if i != num_pixels {
        predictors_sub_sse(
            10,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_avx2.c GetSumAbsDiff32_AVX2
#[target_feature(enable = "avx2")]
#[inline]
#[expect(non_snake_case, reason = "the C parameter names A and B")]
fn get_sum_abs_diff32_avx2(A: __m256i, B: __m256i) -> __m256i {
    // We can unpack with any value on the upper 32 bits, provided it's the same
    // on both operands (to that their sum of abs diff is zero). Here we use *A.
    let a_lo = _mm256_unpacklo_epi32(A, A);
    let b_lo = _mm256_unpacklo_epi32(B, A);
    let a_hi = _mm256_unpackhi_epi32(A, A);
    let b_hi = _mm256_unpackhi_epi32(B, A);
    let s_lo = _mm256_sad_epu8(a_lo, b_lo);
    let s_hi = _mm256_sad_epu8(a_hi, b_hi);
    _mm256_packs_epi32(s_lo, s_hi)
}

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub11_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names L, T, TL, A and B")]
pub(crate) fn predictor_sub11_avx2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    while i + 8 <= num_pixels {
        let L = loadu_si256(&data[at(in_pos + idx(i), -1)..]);
        let T = loadu_si256(&data[upper_pos + idx(i)..]);
        let TL = loadu_si256(&data[at(upper_pos + idx(i), -1)..]);
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let pa = get_sum_abs_diff32_avx2(T, TL); // pa = sum |T-TL|
        let pb = get_sum_abs_diff32_avx2(L, TL); // pb = sum |L-TL|
        {
            let mask = _mm256_cmpgt_epi32(pb, pa);
            let A = _mm256_and_si256(mask, L);
            let B = _mm256_andnot_si256(mask, T);
            let pred = _mm256_or_si256(A, B); // pred = (L > T)? L : T
            let res = _mm256_sub_epi8(src, pred);
            storeu_si256(&mut out[idx(i)..], res);
        }
        i += 8;
    }
    if i != num_pixels {
        predictors_sub_sse(
            11,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub12_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names L, T and TL")]
pub(crate) fn predictor_sub12_avx2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    let zero = _mm256_setzero_si256();
    while i + 8 <= num_pixels {
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let L = loadu_si256(&data[at(in_pos + idx(i), -1)..]);
        let L_lo = _mm256_unpacklo_epi8(L, zero);
        let L_hi = _mm256_unpackhi_epi8(L, zero);
        let T = loadu_si256(&data[upper_pos + idx(i)..]);
        let T_lo = _mm256_unpacklo_epi8(T, zero);
        let T_hi = _mm256_unpackhi_epi8(T, zero);
        let TL = loadu_si256(&data[at(upper_pos + idx(i), -1)..]);
        let TL_lo = _mm256_unpacklo_epi8(TL, zero);
        let TL_hi = _mm256_unpackhi_epi8(TL, zero);
        let diff_lo = _mm256_sub_epi16(T_lo, TL_lo);
        let diff_hi = _mm256_sub_epi16(T_hi, TL_hi);
        let pred_lo = _mm256_add_epi16(L_lo, diff_lo);
        let pred_hi = _mm256_add_epi16(L_hi, diff_hi);
        let pred = _mm256_packus_epi16(pred_lo, pred_hi);
        let res = _mm256_sub_epi8(src, pred);
        storeu_si256(&mut out[idx(i)..], res);
        i += 8;
    }
    if i != num_pixels {
        predictors_sub_sse(
            12,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_avx2.c PredictorSub13_AVX2
#[target_feature(enable = "avx2")]
#[expect(non_snake_case, reason = "the C register names L, T, TL and A4")]
pub(crate) fn predictor_sub13_avx2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    let zero = _mm256_setzero_si256();
    while i + 8 <= num_pixels {
        let L = loadu_si256(&data[at(in_pos + idx(i), -1)..]);
        let src = loadu_si256(&data[in_pos + idx(i)..]);
        let T = loadu_si256(&data[upper_pos + idx(i)..]);
        let TL = loadu_si256(&data[at(upper_pos + idx(i), -1)..]);
        // lo.
        let L_lo = _mm256_unpacklo_epi8(L, zero);
        let T_lo = _mm256_unpacklo_epi8(T, zero);
        let TL_lo = _mm256_unpacklo_epi8(TL, zero);
        let sum_lo = _mm256_add_epi16(T_lo, L_lo);
        let avg_lo = _mm256_srli_epi16::<1>(sum_lo);
        let A1_lo = _mm256_sub_epi16(avg_lo, TL_lo);
        let bit_fix_lo = _mm256_cmpgt_epi16(TL_lo, avg_lo);
        let A2_lo = _mm256_sub_epi16(A1_lo, bit_fix_lo);
        let A3_lo = _mm256_srai_epi16::<1>(A2_lo);
        let A4_lo = _mm256_add_epi16(avg_lo, A3_lo);
        // hi.
        let L_hi = _mm256_unpackhi_epi8(L, zero);
        let T_hi = _mm256_unpackhi_epi8(T, zero);
        let TL_hi = _mm256_unpackhi_epi8(TL, zero);
        let sum_hi = _mm256_add_epi16(T_hi, L_hi);
        let avg_hi = _mm256_srli_epi16::<1>(sum_hi);
        let A1_hi = _mm256_sub_epi16(avg_hi, TL_hi);
        let bit_fix_hi = _mm256_cmpgt_epi16(TL_hi, avg_hi);
        let A2_hi = _mm256_sub_epi16(A1_hi, bit_fix_hi);
        let A3_hi = _mm256_srai_epi16::<1>(A2_hi);
        let A4_hi = _mm256_add_epi16(avg_hi, A3_hi);

        let pred = _mm256_packus_epi16(A4_lo, A4_hi);
        let res = _mm256_sub_epi8(src, pred);
        storeu_si256(&mut out[idx(i)..], res);
        i += 8;
    }
    if i != num_pixels {
        predictors_sub_sse(
            13,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

//------------------------------------------------------------------------------
// Entry points (ADR 0008). The dispatch functions of `src/dsp/lossless_enc.rs`
// call them only after `features().sse2`, `features().sse4_1` and
// `features().avx2`, the answers of `VP8GetCPUInfo(kSSE2)`, `(kSSE4_1)` and
// `(kAVX2)`, selected the AVX2 variant.

/// Entry point of [`subtract_green_from_blue_and_red_avx2`].
pub(crate) fn subtract_green_from_blue_and_red(argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { subtract_green_from_blue_and_red_avx2(argb_data, num_pixels) }
}

/// Entry point of [`transform_color_avx2`].
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color"
)]
pub(crate) fn transform_color(m: &VP8LMultipliers, argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { transform_color_avx2(m, argb_data, num_pixels) }
}

/// Entry point of [`collect_color_blue_transforms_avx2`].
pub(crate) fn collect_color_blue_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_blue: i32,
    red_to_blue: i32,
    histo: &mut [u32],
) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe {
        collect_color_blue_transforms_avx2(
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

/// Entry point of [`collect_color_red_transforms_avx2`].
pub(crate) fn collect_color_red_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_red: i32,
    histo: &mut [u32],
) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe {
        collect_color_red_transforms_avx2(
            argb,
            stride,
            tile_width,
            tile_height,
            green_to_red,
            histo,
        );
    }
}

/// Entry point of [`add_vector_avx2`].
pub(crate) fn add_vector(a: &[u32], b: &[u32], out: &mut [u32], size: i32) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { add_vector_avx2(a, b, out, size) }
}

/// Entry point of [`add_vector_eq_avx2`].
pub(crate) fn add_vector_eq(a: &[u32], out: &mut [u32], size: i32) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { add_vector_eq_avx2(a, out, size) }
}

/// Entry point of [`combined_shannon_entropy_avx2`].
pub(crate) fn combined_shannon_entropy(x: &[u32], y: &[u32]) -> u64 {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { combined_shannon_entropy_avx2(x, y) }
}

/// Entry point of [`vector_mismatch_avx2`].
pub(crate) fn vector_mismatch(array1: &[u32], array2: &[u32], length: i32) -> i32 {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { vector_mismatch_avx2(array1, array2, length) }
}

/// Entry point of [`bundle_color_map_avx2`].
pub(crate) fn bundle_color_map(row: &[u8], width: i32, xbits: i32, dst: &mut [u32]) {
    // SAFETY: called only after `features().avx2` selected the AVX2 variant,
    // so the CPU has AVX2.
    unsafe { bundle_color_map_avx2(row, width, xbits, dst) }
}

/// Generates the entry point of an AVX2 batch predictor for
/// `vp8l_predictors_sub`.
macro_rules! predictor_sub_entry {
    ($entry:ident, $body:ident) => {
        #[doc = concat!("Entry point of [`", stringify!($body), "`].")]
        pub(crate) fn $entry(
            data: &[u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out: &mut [u32],
        ) {
            // SAFETY: called only after `features().avx2` selected the AVX2
            // variant, so the CPU has AVX2.
            unsafe { $body(data, in_pos, upper_pos, num_pixels, out) }
        }
    };
}

predictor_sub_entry!(predictor_sub0, predictor_sub0_avx2);
predictor_sub_entry!(predictor_sub1, predictor_sub1_avx2);
predictor_sub_entry!(predictor_sub2, predictor_sub2_avx2);
predictor_sub_entry!(predictor_sub3, predictor_sub3_avx2);
predictor_sub_entry!(predictor_sub4, predictor_sub4_avx2);
predictor_sub_entry!(predictor_sub5, predictor_sub5_avx2);
predictor_sub_entry!(predictor_sub6, predictor_sub6_avx2);
predictor_sub_entry!(predictor_sub7, predictor_sub7_avx2);
predictor_sub_entry!(predictor_sub8, predictor_sub8_avx2);
predictor_sub_entry!(predictor_sub9, predictor_sub9_avx2);
predictor_sub_entry!(predictor_sub10, predictor_sub10_avx2);
predictor_sub_entry!(predictor_sub11, predictor_sub11_avx2);
predictor_sub_entry!(predictor_sub12, predictor_sub12_avx2);
predictor_sub_entry!(predictor_sub13, predictor_sub13_avx2);

/// `VP8LPredictorsSub` as `VP8LEncDspInitAVX2` fills it, with
/// `PredictorSub0_AVX2` for the sentinels 14 and 15.
pub(crate) static PREDICTORS_SUB: [crate::dsp::lossless_enc::PredictorSubFunc; 16] = [
    predictor_sub0,
    predictor_sub1,
    predictor_sub2,
    predictor_sub3,
    predictor_sub4,
    predictor_sub5,
    predictor_sub6,
    predictor_sub7,
    predictor_sub8,
    predictor_sub9,
    predictor_sub10,
    predictor_sub11,
    predictor_sub12,
    predictor_sub13,
    predictor_sub0, // <- padding security sentinels
    predictor_sub0,
];
