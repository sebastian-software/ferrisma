// libwebp v1.6.0, src/dsp/lossless_enc_sse2.c:
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
// SSE2 variant of methods for lossless encoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/lossless_enc_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE2 variants of the lossless encoder's transforms, histogram helpers
//! and batch predictors that `VP8LEncDspInitSSE2` assigns, behind the
//! dispatch functions of `src/dsp/lossless_enc.rs` (ADR 0008).
//! `VP8LEncDspInit` runs `VP8LEncDspInitSSE2` when `VP8GetCPUInfo(kSSE2)`
//! holds, so a dispatch function selects these entry points when
//! `features().sse2` holds, after the SSE4.1 and AVX2 variants it may prefer.
//! `VP8LEncDspInitSSE2` assigns:
//!
//! - `VP8LSubtractGreenFromBlueAndRed`: `subtract_green_from_blue_and_red`;
//! - `VP8LTransformColor`: `transform_color`;
//! - `VP8LCollectColorBlueTransforms`: `collect_color_blue_transforms`;
//! - `VP8LCollectColorRedTransforms`: `collect_color_red_transforms`;
//! - `VP8LAddVector`: `add_vector`; `VP8LAddVectorEq`: `add_vector_eq`;
//! - `VP8LCombinedShannonEntropy`: `combined_shannon_entropy` (compiled
//!   because Clang provides `__builtin_ctz`, so `WEBP_HAVE_SLOW_CLZ_CTZ` is
//!   undefined);
//! - `VP8LVectorMismatch`: `vector_mismatch`;
//! - `VP8LBundleColorMap`: `bundle_color_map`;
//! - `VP8LPredictorsSub[0..16]`: `predictor_sub0` to `predictor_sub13`, with
//!   `predictor_sub0` for the sentinels 14 and 15;
//! - the "SSE exports for AVX and above" (`VP8LSubtractGreenFromBlueAndRed_SSE`
//!   and the others, and `VP8LPredictorsSub_SSE`), which
//!   `lossless_enc_avx2.rs` reaches as the `_sse2` functions directly; they
//!   are `pub(crate)` for it.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers** are slices and positions as in `src/dsp/lossless_enc.rs`:
//!    the batch predictors take the scratch buffer with the positions of the
//!    input row and the row above, and the C pointer increments are indices.
//!    `NULL` as the row above (mode 0) is position 0, which mode 0 does not
//!    read.
//! 3. **Macros** (`CST_5b`, `MK_CST_16`, `GENERATE_PREDICTOR_1`,
//!    `GENERATE_PREDICTOR_2`) are functions and `macro_rules!` of the same
//!    shape; `Average2_m128i` and `GetSumAbsDiff32_SSE2` return their result
//!    instead of writing through a pointer.
//! 4. **Panics on short buffers** come at the vector access, before or after
//!    other writes than in the scalar translation (`common_sse2.rs`,
//!    deviation 3).
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions': the SSE2 functions compute the C functions'
//! results for every input the library passes (upstream's `-noasm`
//! comparison relies on it). `AddVector_SSE2` and `AddVectorEq_SSE2` assert
//! `size >= 16` and an even size, as C does; the library passes histogram
//! sizes of at least 40.
//!
//! # Checks not applicable
//!
//! 1. **`VP8LEncDspInitSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi8, _mm_add_epi16, _mm_add_epi32, _mm_and_si128, _mm_andnot_si128,
    _mm_avg_epu8, _mm_cmpeq_epi32, _mm_cmpgt_epi8, _mm_cmpgt_epi16, _mm_cmpgt_epi32,
    _mm_movemask_epi8, _mm_mulhi_epi16, _mm_mullo_epi16, _mm_or_si128, _mm_packs_epi16,
    _mm_packs_epi32, _mm_packus_epi16, _mm_sad_epu8, _mm_set1_epi8, _mm_set1_epi16, _mm_set1_epi32,
    _mm_setzero_si128, _mm_shufflehi_epi16, _mm_shufflelo_epi16, _mm_slli_epi16, _mm_slli_epi64,
    _mm_srai_epi16, _mm_srli_epi16, _mm_srli_epi32, _mm_sub_epi8, _mm_sub_epi16, _mm_unpackhi_epi8,
    _mm_unpackhi_epi16, _mm_unpackhi_epi32, _mm_unpacklo_epi8, _mm_unpacklo_epi16,
    _mm_unpacklo_epi32, _mm_xor_si128,
};

use crate::dsp::common_sse2::{loadl_epi64, loadu_si128, storel_epi64, storeu_si128};
use crate::dsp::lossless::VP8LMultipliers;
use crate::dsp::lossless_common::vp8l_fast_s_log2;
use crate::dsp::lossless_enc::{
    vp8l_bundle_color_map_c, vp8l_collect_color_blue_transforms_c,
    vp8l_collect_color_red_transforms_c, vp8l_predictors_sub_c,
    vp8l_subtract_green_from_blue_and_red_c, vp8l_transform_color_c,
};
use crate::dsp::lossless_sse2::{at, idx, mm_shuffle};
use crate::utils::utils::bits_ctz;
use crate::webp::format_constants::ARGB_BLACK;

// upstream: src/dsp/lossless_enc_sse2.c CST_5b
/// The multiplier `X`, sign-extended from its byte and pre-shifted by 5.
#[inline]
pub(crate) fn cst_5b(x: u8) -> i32 {
    i32::from((u16::from(x) << 8).cast_signed()) >> 5
}

// upstream: src/dsp/lossless_enc_sse2.c MK_CST_16
/// The 32-bit lane `(HI << 16) | (LO & 0xffff)`, splatted.
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn mk_cst_16(hi: i32, lo: i32) -> __m128i {
    _mm_set1_epi32(((hi.cast_unsigned() << 16) | (lo & 0xffff).cast_unsigned()).cast_signed())
}

// upstream: src/dsp/lossless_enc_sse2.c SubtractGreenFromBlueAndRed_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn subtract_green_from_blue_and_red_sse2(argb_data: &mut [u32], num_pixels: i32) {
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let input = loadu_si128(&argb_data[idx(i)..]); // argb
        let a = _mm_srli_epi16::<8>(input); // 0 a 0 g
        let b = _mm_shufflelo_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(a);
        let c = _mm_shufflehi_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(b); // 0g0g
        let out = _mm_sub_epi8(input, c);
        storeu_si128(&mut argb_data[idx(i)..], out);
        i += 4;
    }
    // fallthrough and finish off with plain-C
    if i != num_pixels {
        vp8l_subtract_green_from_blue_and_red_c(&mut argb_data[idx(i)..], num_pixels - i);
    }
}

// upstream: src/dsp/lossless_enc_sse2.c TransformColor_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::many_single_char_names, reason = "the C register names A to I")]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color"
)]
pub(crate) fn transform_color_sse2(m: &VP8LMultipliers, argb_data: &mut [u32], num_pixels: i32) {
    let mults_rb = mk_cst_16(cst_5b(m.green_to_red), cst_5b(m.green_to_blue));
    let mults_b2 = mk_cst_16(cst_5b(m.red_to_blue), 0);
    let mask_ag = _mm_set1_epi32(0xff00_ff00_u32.cast_signed()); // alpha-green masks
    let mask_rb = _mm_set1_epi32(0x00ff_00ff); // red-blue masks
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let input = loadu_si128(&argb_data[idx(i)..]); // argb
        let a = _mm_and_si128(input, mask_ag); // a   0   g   0
        let b = _mm_shufflelo_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(a);
        let c = _mm_shufflehi_epi16::<{ mm_shuffle(2, 2, 0, 0) }>(b); // g0g0
        let d = _mm_mulhi_epi16(c, mults_rb); // x dr  x db1
        let e = _mm_slli_epi16::<8>(input); // r 0   b   0
        let f = _mm_mulhi_epi16(e, mults_b2); // x db2 0   0
        let g = _mm_srli_epi32::<16>(f); // 0 0   x db2
        let h = _mm_add_epi8(g, d); // x dr  x  db
        let i_ = _mm_and_si128(h, mask_rb); // 0 dr  0  db
        let out = _mm_sub_epi8(input, i_);
        storeu_si128(&mut argb_data[idx(i)..], out);
        i += 4;
    }
    // fallthrough and finish off with plain-C
    if i != num_pixels {
        vp8l_transform_color_c(m, &mut argb_data[idx(i)..], num_pixels - i);
    }
}

// upstream: src/dsp/lossless_enc_sse2.c SPAN
const SPAN: i32 = 8;

/// The start of row `y` of a tile, `argb + y * stride` in C.
#[inline]
fn row_start(y: i32, stride: i32) -> usize {
    usize::try_from(i64::from(y) * i64::from(stride)).expect("a tile row lies in its buffer")
}

// upstream: src/dsp/lossless_enc_sse2.c CollectColorBlueTransforms_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn collect_color_blue_transforms_sse2(
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
    let mults_r = mk_cst_16(cst_5b(red_to_blue as u8), 0);
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "the C (uint16_t) cast in CST_5b"
    )]
    let mults_g = mk_cst_16(0, cst_5b(green_to_blue as u8));
    let mask_g = _mm_set1_epi32(0x0000_ff00); // green mask
    let mask_b = _mm_set1_epi32(0x0000_00ff); // blue mask
    for y in 0..tile_height {
        let src = row_start(y, stride);
        let mut x: i32 = 0;
        while x + SPAN <= tile_width {
            let mut values = [0_u16; SPAN as usize];
            let in0 = loadu_si128(&argb[src + idx(x)..]);
            let in1 = loadu_si128(&argb[src + idx(x + SPAN / 2)..]);
            let a0 = _mm_slli_epi16::<8>(in0); // r 0  | b 0
            let a1 = _mm_slli_epi16::<8>(in1);
            let b0 = _mm_and_si128(in0, mask_g); // 0 0  | g 0
            let b1 = _mm_and_si128(in1, mask_g);
            let c0 = _mm_mulhi_epi16(a0, mults_r); // x db | 0 0
            let c1 = _mm_mulhi_epi16(a1, mults_r);
            let d0 = _mm_mulhi_epi16(b0, mults_g); // 0 0  | x db
            let d1 = _mm_mulhi_epi16(b1, mults_g);
            let e0 = _mm_sub_epi8(in0, d0); // x x  | x b'
            let e1 = _mm_sub_epi8(in1, d1);
            let f0 = _mm_srli_epi32::<16>(c0); // 0 0  | x db
            let f1 = _mm_srli_epi32::<16>(c1);
            let g0 = _mm_sub_epi8(e0, f0); // 0 0  | x b'
            let g1 = _mm_sub_epi8(e1, f1);
            let h0 = _mm_and_si128(g0, mask_b); // 0 0  | 0 b
            let h1 = _mm_and_si128(g1, mask_b);
            let i = _mm_packs_epi32(h0, h1); // 0 b' | 0 b'
            storeu_si128(&mut values, i);
            for &value in &values {
                let index = usize::from(value);
                histo[index] = histo[index].wrapping_add(1);
            }
            x += SPAN;
        }
    }
    let left_over = tile_width & (SPAN - 1);
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

// upstream: src/dsp/lossless_enc_sse2.c CollectColorRedTransforms_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn collect_color_red_transforms_sse2(
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
    let mults_g = mk_cst_16(0, cst_5b(green_to_red as u8));
    let mask_g = _mm_set1_epi32(0x0000_ff00); // green mask
    let mask = _mm_set1_epi32(0xff);

    for y in 0..tile_height {
        let src = row_start(y, stride);
        let mut x: i32 = 0;
        while x + SPAN <= tile_width {
            let mut values = [0_u16; SPAN as usize];
            let in0 = loadu_si128(&argb[src + idx(x)..]);
            let in1 = loadu_si128(&argb[src + idx(x + SPAN / 2)..]);
            let a0 = _mm_and_si128(in0, mask_g); // 0 0  | g 0
            let a1 = _mm_and_si128(in1, mask_g);
            let b0 = _mm_srli_epi32::<16>(in0); // 0 0  | x r
            let b1 = _mm_srli_epi32::<16>(in1);
            let c0 = _mm_mulhi_epi16(a0, mults_g); // 0 0  | x dr
            let c1 = _mm_mulhi_epi16(a1, mults_g);
            let e0 = _mm_sub_epi8(b0, c0); // x x  | x r'
            let e1 = _mm_sub_epi8(b1, c1);
            let f0 = _mm_and_si128(e0, mask); // 0 0  | 0 r'
            let f1 = _mm_and_si128(e1, mask);
            let i = _mm_packs_epi32(f0, f1);
            storeu_si128(&mut values, i);
            for &value in &values {
                let index = usize::from(value);
                histo[index] = histo[index].wrapping_add(1);
            }
            x += SPAN;
        }
    }
    let left_over = tile_width & (SPAN - 1);
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

// upstream: src/dsp/lossless_enc_sse2.c AddVector_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn add_vector_sse2(a: &[u32], b: &[u32], out: &mut [u32], size: i32) {
    let mut i: usize = 0;
    let mut size = size;
    let aligned_size = idx(size & !15);
    // Size is, at minimum, NUM_DISTANCE_CODES (40) and may be as large as
    // NUM_LITERAL_CODES (256) + NUM_LENGTH_CODES (24) + (0 or a non-zero power of
    // 2). See the usage in VP8LHistogramAdd().
    debug_assert!(size >= 16);
    debug_assert_eq!(size % 2, 0);

    loop {
        let a0 = loadu_si128(&a[i..]);
        let a1 = loadu_si128(&a[i + 4..]);
        let a2 = loadu_si128(&a[i + 8..]);
        let a3 = loadu_si128(&a[i + 12..]);
        let b0 = loadu_si128(&b[i..]);
        let b1 = loadu_si128(&b[i + 4..]);
        let b2 = loadu_si128(&b[i + 8..]);
        let b3 = loadu_si128(&b[i + 12..]);
        storeu_si128(&mut out[i..], _mm_add_epi32(a0, b0));
        storeu_si128(&mut out[i + 4..], _mm_add_epi32(a1, b1));
        storeu_si128(&mut out[i + 8..], _mm_add_epi32(a2, b2));
        storeu_si128(&mut out[i + 12..], _mm_add_epi32(a3, b3));
        i += 16;
        if i == aligned_size {
            break;
        }
    }

    if (size & 8) != 0 {
        let a0 = loadu_si128(&a[i..]);
        let a1 = loadu_si128(&a[i + 4..]);
        let b0 = loadu_si128(&b[i..]);
        let b1 = loadu_si128(&b[i + 4..]);
        storeu_si128(&mut out[i..], _mm_add_epi32(a0, b0));
        storeu_si128(&mut out[i + 4..], _mm_add_epi32(a1, b1));
        i += 8;
    }

    size &= 7;
    if size == 4 {
        let a0 = loadu_si128(&a[i..]);
        let b0 = loadu_si128(&b[i..]);
        storeu_si128(&mut out[i..], _mm_add_epi32(a0, b0));
    } else if size == 2 {
        let a0 = loadl_epi64(&a[i..]);
        let b0 = loadl_epi64(&b[i..]);
        storel_epi64(&mut out[i..], _mm_add_epi32(a0, b0));
    }
}

// upstream: src/dsp/lossless_enc_sse2.c AddVectorEq_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn add_vector_eq_sse2(a: &[u32], out: &mut [u32], size: i32) {
    let mut i: usize = 0;
    let mut size = size;
    let aligned_size = idx(size & !15);
    // Size is, at minimum, NUM_DISTANCE_CODES (40) and may be as large as
    // NUM_LITERAL_CODES (256) + NUM_LENGTH_CODES (24) + (0 or a non-zero power of
    // 2). See the usage in VP8LHistogramAdd().
    debug_assert!(size >= 16);
    debug_assert_eq!(size % 2, 0);

    loop {
        let a0 = loadu_si128(&a[i..]);
        let a1 = loadu_si128(&a[i + 4..]);
        let a2 = loadu_si128(&a[i + 8..]);
        let a3 = loadu_si128(&a[i + 12..]);
        let b0 = loadu_si128(&out[i..]);
        let b1 = loadu_si128(&out[i + 4..]);
        let b2 = loadu_si128(&out[i + 8..]);
        let b3 = loadu_si128(&out[i + 12..]);
        storeu_si128(&mut out[i..], _mm_add_epi32(a0, b0));
        storeu_si128(&mut out[i + 4..], _mm_add_epi32(a1, b1));
        storeu_si128(&mut out[i + 8..], _mm_add_epi32(a2, b2));
        storeu_si128(&mut out[i + 12..], _mm_add_epi32(a3, b3));
        i += 16;
        if i == aligned_size {
            break;
        }
    }

    if (size & 8) != 0 {
        let a0 = loadu_si128(&a[i..]);
        let a1 = loadu_si128(&a[i + 4..]);
        let b0 = loadu_si128(&out[i..]);
        let b1 = loadu_si128(&out[i + 4..]);
        storeu_si128(&mut out[i..], _mm_add_epi32(a0, b0));
        storeu_si128(&mut out[i + 4..], _mm_add_epi32(a1, b1));
        i += 8;
    }

    size &= 7;
    if size == 4 {
        let a0 = loadu_si128(&a[i..]);
        let b0 = loadu_si128(&out[i..]);
        storeu_si128(&mut out[i..], _mm_add_epi32(a0, b0));
    } else if size == 2 {
        let a0 = loadl_epi64(&a[i..]);
        let b0 = loadl_epi64(&out[i..]);
        storel_epi64(&mut out[i..], _mm_add_epi32(a0, b0));
    }
}

// upstream: src/dsp/lossless_enc_sse2.c CombinedShannonEntropy_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C parameter names X and Y")]
pub(crate) fn combined_shannon_entropy_sse2(X: &[u32], Y: &[u32]) -> u64 {
    let mut retval: u64 = 0;
    let mut sum_x: u32 = 0;
    let mut sum_xy: u32 = 0;
    let zero = _mm_setzero_si128();

    let mut i: usize = 0;
    while i < 256 {
        let x0 = loadu_si128(&X[i..]);
        let y0 = loadu_si128(&Y[i..]);
        let x1 = loadu_si128(&X[i + 4..]);
        let y1 = loadu_si128(&Y[i + 4..]);
        let x2 = loadu_si128(&X[i + 8..]);
        let y2 = loadu_si128(&Y[i + 8..]);
        let x3 = loadu_si128(&X[i + 12..]);
        let y3 = loadu_si128(&Y[i + 12..]);
        let x4 = _mm_packs_epi16(_mm_packs_epi32(x0, x1), _mm_packs_epi32(x2, x3));
        let y4 = _mm_packs_epi16(_mm_packs_epi32(y0, y1), _mm_packs_epi32(y2, y3));
        let mx: i32 = _mm_movemask_epi8(_mm_cmpgt_epi8(x4, zero));
        let mut my: i32 = _mm_movemask_epi8(_mm_cmpgt_epi8(y4, zero)) | mx;
        while my != 0 {
            let j = bits_ctz(my.cast_unsigned());
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
        i += 16;
    }
    vp8l_fast_s_log2(sum_x)
        .wrapping_add(vp8l_fast_s_log2(sum_xy))
        .wrapping_sub(retval)
}

// upstream: src/dsp/lossless_enc_sse2.c VectorMismatch_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn vector_mismatch_sse2(array1: &[u32], array2: &[u32], length: i32) -> i32 {
    let mut match_len: i32;

    if length >= 12 {
        let mut a0 = loadu_si128(&array1[0..]);
        let mut a1 = loadu_si128(&array2[0..]);
        match_len = 0;
        loop {
            // Loop unrolling and early load both provide a speedup of 10% for the
            // current function. Also, max_limit can be MAX_LENGTH=4096 at most.
            let cmp_a = _mm_cmpeq_epi32(a0, a1);
            let b0 = loadu_si128(&array1[idx(match_len + 4)..]);
            let b1 = loadu_si128(&array2[idx(match_len + 4)..]);
            if _mm_movemask_epi8(cmp_a) != 0xffff {
                break;
            }
            match_len += 4;

            {
                let cmp_b = _mm_cmpeq_epi32(b0, b1);
                a0 = loadu_si128(&array1[idx(match_len + 4)..]);
                a1 = loadu_si128(&array2[idx(match_len + 4)..]);
                if _mm_movemask_epi8(cmp_b) != 0xffff {
                    break;
                }
                match_len += 4;
            }
            if match_len + 12 >= length {
                break;
            }
        }
    } else {
        match_len = 0;
        // Unroll the potential first two loops.
        if length >= 4
            && _mm_movemask_epi8(_mm_cmpeq_epi32(
                loadu_si128(&array1[0..]),
                loadu_si128(&array2[0..]),
            )) == 0xffff
        {
            match_len = 4;
            if length >= 8
                && _mm_movemask_epi8(_mm_cmpeq_epi32(
                    loadu_si128(&array1[4..]),
                    loadu_si128(&array2[4..]),
                )) == 0xffff
            {
                match_len = 8;
            }
        }
    }

    while match_len < length && array1[idx(match_len)] == array2[idx(match_len)] {
        match_len += 1;
    }
    match_len
}

// upstream: src/dsp/lossless_enc_sse2.c BundleColorMap_SSE2
#[target_feature(enable = "sse2")]
pub(crate) fn bundle_color_map_sse2(row: &[u8], width: i32, xbits: i32, dst: &mut [u32]) {
    let mut x: i32 = 0;
    let mut d: usize = 0; // the C `dst` pointer as an index
    debug_assert!(xbits >= 0);
    debug_assert!(xbits <= 3);
    match xbits {
        0 => {
            let ff = _mm_set1_epi16(0xff00_u16.cast_signed());
            let zero = _mm_setzero_si128();
            // Store 0xff000000 | (row[x] << 8).
            while x + 16 <= width {
                let input = loadu_si128(&row[idx(x)..]);
                let in_lo = _mm_unpacklo_epi8(zero, input);
                let dst0 = _mm_unpacklo_epi16(in_lo, ff);
                let dst1 = _mm_unpackhi_epi16(in_lo, ff);
                let in_hi = _mm_unpackhi_epi8(zero, input);
                let dst2 = _mm_unpacklo_epi16(in_hi, ff);
                let dst3 = _mm_unpackhi_epi16(in_hi, ff);
                storeu_si128(&mut dst[d..], dst0);
                storeu_si128(&mut dst[d + 4..], dst1);
                storeu_si128(&mut dst[d + 8..], dst2);
                storeu_si128(&mut dst[d + 12..], dst3);
                x += 16;
                d += 16;
            }
        }
        1 => {
            let ff = _mm_set1_epi16(0xff00_u16.cast_signed());
            let mul = _mm_set1_epi16(0x110);
            while x + 16 <= width {
                // 0a0b | (where a/b are 4 bits).
                let input = loadu_si128(&row[idx(x)..]);
                let tmp = _mm_mullo_epi16(input, mul); // aba0
                let pack = _mm_and_si128(tmp, ff); // ab00
                let dst0 = _mm_unpacklo_epi16(pack, ff);
                let dst1 = _mm_unpackhi_epi16(pack, ff);
                storeu_si128(&mut dst[d..], dst0);
                storeu_si128(&mut dst[d + 4..], dst1);
                x += 16;
                d += 8;
            }
        }
        2 => {
            let mask_or = _mm_set1_epi32(0xff00_0000_u32.cast_signed());
            let mul_cst = _mm_set1_epi16(0x0104);
            let mask_mul = _mm_set1_epi16(0x0f00);
            while x + 16 <= width {
                // 000a000b000c000d | (where a/b/c/d are 2 bits).
                let input = loadu_si128(&row[idx(x)..]);
                let mul = _mm_mullo_epi16(input, mul_cst); // 00ab00b000cd00d0
                let tmp = _mm_and_si128(mul, mask_mul); // 00ab000000cd0000
                let shift = _mm_srli_epi32::<12>(tmp); // 00000000ab000000
                let pack = _mm_or_si128(shift, tmp); // 00000000abcd0000
                // Convert to 0xff00**00.
                let res = _mm_or_si128(pack, mask_or);
                storeu_si128(&mut dst[d..], res);
                x += 16;
                d += 4;
            }
        }
        _ => {
            debug_assert_eq!(xbits, 3);
            while x + 16 <= width {
                // 0000000a00000000b... | (where a/b are 1 bit).
                let input = loadu_si128(&row[idx(x)..]);
                let shift = _mm_slli_epi64::<7>(input);
                let mv = _mm_movemask_epi8(shift).cast_unsigned();
                dst[d] = 0xff00_0000 | ((mv & 0xff) << 8);
                dst[d + 1] = 0xff00_0000 | (mv & 0xff00);
                x += 16;
                d += 2;
            }
        }
    }
    if x != width {
        vp8l_bundle_color_map_c(&row[idx(x)..], width - x, xbits, &mut dst[d..]);
    }
}

//------------------------------------------------------------------------------
// Batch version of Predictor Transform subtraction

// upstream: src/dsp/lossless_enc_sse2.c Average2_m128i
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn average2_m128i(a0: __m128i, a1: __m128i) -> __m128i {
    // (a + b) >> 1 = ((a + b + 1) >> 1) - ((a ^ b) & 1)
    let ones = _mm_set1_epi8(1);
    let avg1 = _mm_avg_epu8(a0, a1);
    let one = _mm_and_si128(_mm_xor_si128(a0, a1), ones);
    _mm_sub_epi8(avg1, one)
}

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub0_SSE2
/// Predictor0: `ARGB_BLACK`.
#[target_feature(enable = "sse2")]
pub(crate) fn predictor_sub0_sse2(
    data: &[u32],
    in_pos: usize,
    _upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    let black = _mm_set1_epi32(ARGB_BLACK.cast_signed());
    while i + 4 <= num_pixels {
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let res = _mm_sub_epi8(src, black);
        storeu_si128(&mut out[idx(i)..], res);
        i += 4;
    }
    if i != num_pixels {
        // C: upper is NULL.
        vp8l_predictors_sub_c(
            0,
            data,
            in_pos + idx(i),
            0,
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

/// The row a batch predictor reads a neighbor from: the input row (`in` in C)
/// or the row above (`upper`).
#[derive(Clone, Copy)]
pub(crate) enum Row {
    In,
    Upper,
}

/// The position of the neighbor `delta` pixels from pixel `i` in `row`, as C
/// forms `&in[i + delta]` or `&upper[i + delta]`.
#[inline]
pub(crate) fn neighbor(row: Row, delta: isize, in_pos: usize, upper_pos: usize, i: i32) -> usize {
    let base = match row {
        Row::In => in_pos,
        Row::Upper => upper_pos,
    };
    at(base + idx(i), delta)
}

// upstream: src/dsp/lossless_enc_sse2.c GENERATE_PREDICTOR_1
/// The residual against one neighbor, given by its row and its offset from
/// pixel `i`.
macro_rules! generate_predictor_1 {
    ($x:literal, $name:ident, $row:ident, $delta:literal) => {
        #[target_feature(enable = "sse2")]
        pub(crate) fn $name(
            data: &[u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out: &mut [u32],
        ) {
            let mut i: i32 = 0;
            while i + 4 <= num_pixels {
                let src = loadu_si128(&data[in_pos + idx(i)..]);
                let pred = loadu_si128(&data[neighbor(Row::$row, $delta, in_pos, upper_pos, i)..]);
                let res = _mm_sub_epi8(src, pred);
                storeu_si128(&mut out[idx(i)..], res);
                i += 4;
            }
            if i != num_pixels {
                vp8l_predictors_sub_c(
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

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub1_SSE2
generate_predictor_1!(1, predictor_sub1_sse2, In, -1); // Predictor1: L
// upstream: src/dsp/lossless_enc_sse2.c PredictorSub2_SSE2
generate_predictor_1!(2, predictor_sub2_sse2, Upper, 0); // Predictor2: T
// upstream: src/dsp/lossless_enc_sse2.c PredictorSub3_SSE2
generate_predictor_1!(3, predictor_sub3_sse2, Upper, 1); // Predictor3: TR
// upstream: src/dsp/lossless_enc_sse2.c PredictorSub4_SSE2
generate_predictor_1!(4, predictor_sub4_sse2, Upper, -1); // Predictor4: TL

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub5_SSE2
/// Predictor5: avg2(avg2(L, TR), T)
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names L, T and TR")]
pub(crate) fn predictor_sub5_sse2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let L = loadu_si128(&data[at(in_pos + idx(i), -1)..]);
        let T = loadu_si128(&data[upper_pos + idx(i)..]);
        let TR = loadu_si128(&data[upper_pos + idx(i) + 1..]);
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let avg = average2_m128i(L, TR);
        let pred = average2_m128i(avg, T);
        let res = _mm_sub_epi8(src, pred);
        storeu_si128(&mut out[idx(i)..], res);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_sub_c(
            5,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_sse2.c GENERATE_PREDICTOR_2
/// The residual against the average of two neighbors, each given by its row
/// and its offset from pixel `i`.
macro_rules! generate_predictor_2 {
    ($x:literal, $name:ident, ($a_row:ident, $a_delta:literal), ($b_row:ident, $b_delta:literal)) => {
        #[target_feature(enable = "sse2")]
        pub(crate) fn $name(
            data: &[u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out: &mut [u32],
        ) {
            let mut i: i32 = 0;
            while i + 4 <= num_pixels {
                let t_a =
                    loadu_si128(&data[neighbor(Row::$a_row, $a_delta, in_pos, upper_pos, i)..]);
                let t_b =
                    loadu_si128(&data[neighbor(Row::$b_row, $b_delta, in_pos, upper_pos, i)..]);
                let src = loadu_si128(&data[in_pos + idx(i)..]);
                let pred = average2_m128i(t_a, t_b);
                let res = _mm_sub_epi8(src, pred);
                storeu_si128(&mut out[idx(i)..], res);
                i += 4;
            }
            if i != num_pixels {
                vp8l_predictors_sub_c(
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

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub6_SSE2
generate_predictor_2!(6, predictor_sub6_sse2, (In, -1), (Upper, -1)); // Predictor6: avg(L, TL)
// upstream: src/dsp/lossless_enc_sse2.c PredictorSub7_SSE2
generate_predictor_2!(7, predictor_sub7_sse2, (In, -1), (Upper, 0)); // Predictor7: avg(L, T)
// upstream: src/dsp/lossless_enc_sse2.c PredictorSub8_SSE2
generate_predictor_2!(8, predictor_sub8_sse2, (Upper, -1), (Upper, 0)); // Predictor8: avg(TL, T)
// upstream: src/dsp/lossless_enc_sse2.c PredictorSub9_SSE2
generate_predictor_2!(9, predictor_sub9_sse2, (Upper, 0), (Upper, 1)); // Predictor9: average(T, TR)

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub10_SSE2
/// Predictor10: avg(avg(L,TL), avg(T, TR)).
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names L, TL, T and TR")]
pub(crate) fn predictor_sub10_sse2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let L = loadu_si128(&data[at(in_pos + idx(i), -1)..]);
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let TL = loadu_si128(&data[at(upper_pos + idx(i), -1)..]);
        let T = loadu_si128(&data[upper_pos + idx(i)..]);
        let TR = loadu_si128(&data[upper_pos + idx(i) + 1..]);
        let avg_ttr = average2_m128i(T, TR);
        let avg_ltl = average2_m128i(L, TL);
        let avg = average2_m128i(avg_ttr, avg_ltl);
        let res = _mm_sub_epi8(src, avg);
        storeu_si128(&mut out[idx(i)..], res);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_sub_c(
            10,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_sse2.c GetSumAbsDiff32_SSE2
/// Predictor11: select.
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C parameter names A and B")]
fn get_sum_abs_diff32_sse2(A: __m128i, B: __m128i) -> __m128i {
    // We can unpack with any value on the upper 32 bits, provided it's the same
    // on both operands (to that their sum of abs diff is zero). Here we use *A.
    let a_lo = _mm_unpacklo_epi32(A, A);
    let b_lo = _mm_unpacklo_epi32(B, A);
    let a_hi = _mm_unpackhi_epi32(A, A);
    let b_hi = _mm_unpackhi_epi32(B, A);
    let s_lo = _mm_sad_epu8(a_lo, b_lo);
    let s_hi = _mm_sad_epu8(a_hi, b_hi);
    _mm_packs_epi32(s_lo, s_hi)
}

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub11_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names L, T, TL, A and B")]
pub(crate) fn predictor_sub11_sse2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    while i + 4 <= num_pixels {
        let L = loadu_si128(&data[at(in_pos + idx(i), -1)..]);
        let T = loadu_si128(&data[upper_pos + idx(i)..]);
        let TL = loadu_si128(&data[at(upper_pos + idx(i), -1)..]);
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let pa = get_sum_abs_diff32_sse2(T, TL); // pa = sum |T-TL|
        let pb = get_sum_abs_diff32_sse2(L, TL); // pb = sum |L-TL|
        {
            let mask = _mm_cmpgt_epi32(pb, pa);
            let A = _mm_and_si128(mask, L);
            let B = _mm_andnot_si128(mask, T);
            let pred = _mm_or_si128(A, B); // pred = (L > T)? L : T
            let res = _mm_sub_epi8(src, pred);
            storeu_si128(&mut out[idx(i)..], res);
        }
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_sub_c(
            11,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub12_SSE2
/// Predictor12: `ClampedSubSubtractFull`.
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names L, T and TL")]
pub(crate) fn predictor_sub12_sse2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    let zero = _mm_setzero_si128();
    while i + 4 <= num_pixels {
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let L = loadu_si128(&data[at(in_pos + idx(i), -1)..]);
        let L_lo = _mm_unpacklo_epi8(L, zero);
        let L_hi = _mm_unpackhi_epi8(L, zero);
        let T = loadu_si128(&data[upper_pos + idx(i)..]);
        let T_lo = _mm_unpacklo_epi8(T, zero);
        let T_hi = _mm_unpackhi_epi8(T, zero);
        let TL = loadu_si128(&data[at(upper_pos + idx(i), -1)..]);
        let TL_lo = _mm_unpacklo_epi8(TL, zero);
        let TL_hi = _mm_unpackhi_epi8(TL, zero);
        let diff_lo = _mm_sub_epi16(T_lo, TL_lo);
        let diff_hi = _mm_sub_epi16(T_hi, TL_hi);
        let pred_lo = _mm_add_epi16(L_lo, diff_lo);
        let pred_hi = _mm_add_epi16(L_hi, diff_hi);
        let pred = _mm_packus_epi16(pred_lo, pred_hi);
        let res = _mm_sub_epi8(src, pred);
        storeu_si128(&mut out[idx(i)..], res);
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_sub_c(
            12,
            data,
            in_pos + idx(i),
            upper_pos + idx(i),
            num_pixels - i,
            &mut out[idx(i)..],
        );
    }
}

// upstream: src/dsp/lossless_enc_sse2.c PredictorSub13_SSE2
/// Predictors13: `ClampedAddSubtractHalf`
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C register names L, T, TL and A4")]
pub(crate) fn predictor_sub13_sse2(
    data: &[u32],
    in_pos: usize,
    upper_pos: usize,
    num_pixels: i32,
    out: &mut [u32],
) {
    let mut i: i32 = 0;
    let zero = _mm_setzero_si128();
    while i + 4 <= num_pixels {
        let L = loadu_si128(&data[at(in_pos + idx(i), -1)..]);
        let src = loadu_si128(&data[in_pos + idx(i)..]);
        let T = loadu_si128(&data[upper_pos + idx(i)..]);
        let TL = loadu_si128(&data[at(upper_pos + idx(i), -1)..]);
        // lo.
        let A4_lo = {
            let L_lo = _mm_unpacklo_epi8(L, zero);
            let T_lo = _mm_unpacklo_epi8(T, zero);
            let TL_lo = _mm_unpacklo_epi8(TL, zero);
            let sum_lo = _mm_add_epi16(T_lo, L_lo);
            let avg_lo = _mm_srli_epi16::<1>(sum_lo);
            let A1_lo = _mm_sub_epi16(avg_lo, TL_lo);
            let bit_fix_lo = _mm_cmpgt_epi16(TL_lo, avg_lo);
            let A2_lo = _mm_sub_epi16(A1_lo, bit_fix_lo);
            let A3_lo = _mm_srai_epi16::<1>(A2_lo);
            _mm_add_epi16(avg_lo, A3_lo)
        };
        // hi.
        let A4_hi = {
            let L_hi = _mm_unpackhi_epi8(L, zero);
            let T_hi = _mm_unpackhi_epi8(T, zero);
            let TL_hi = _mm_unpackhi_epi8(TL, zero);
            let sum_hi = _mm_add_epi16(T_hi, L_hi);
            let avg_hi = _mm_srli_epi16::<1>(sum_hi);
            let A1_hi = _mm_sub_epi16(avg_hi, TL_hi);
            let bit_fix_hi = _mm_cmpgt_epi16(TL_hi, avg_hi);
            let A2_hi = _mm_sub_epi16(A1_hi, bit_fix_hi);
            let A3_hi = _mm_srai_epi16::<1>(A2_hi);
            _mm_add_epi16(avg_hi, A3_hi)
        };
        {
            let pred = _mm_packus_epi16(A4_lo, A4_hi);
            let res = _mm_sub_epi8(src, pred);
            storeu_si128(&mut out[idx(i)..], res);
        }
        i += 4;
    }
    if i != num_pixels {
        vp8l_predictors_sub_c(
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
// Entry points (ADR 0008). `VP8LEncDspInitSSE2` assigns these; the dispatch
// functions of `src/dsp/lossless_enc.rs` call them only after
// `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the SSE2
// variant.

/// Entry point of [`subtract_green_from_blue_and_red_sse2`].
pub(crate) fn subtract_green_from_blue_and_red(argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { subtract_green_from_blue_and_red_sse2(argb_data, num_pixels) }
}

/// Entry point of [`transform_color_sse2`].
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the C pointer parameter, as in vp8l_transform_color"
)]
pub(crate) fn transform_color(m: &VP8LMultipliers, argb_data: &mut [u32], num_pixels: i32) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { transform_color_sse2(m, argb_data, num_pixels) }
}

/// Entry point of [`collect_color_blue_transforms_sse2`].
pub(crate) fn collect_color_blue_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_blue: i32,
    red_to_blue: i32,
    histo: &mut [u32],
) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe {
        collect_color_blue_transforms_sse2(
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

/// Entry point of [`collect_color_red_transforms_sse2`].
pub(crate) fn collect_color_red_transforms(
    argb: &[u32],
    stride: i32,
    tile_width: i32,
    tile_height: i32,
    green_to_red: i32,
    histo: &mut [u32],
) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe {
        collect_color_red_transforms_sse2(
            argb,
            stride,
            tile_width,
            tile_height,
            green_to_red,
            histo,
        );
    }
}

/// Entry point of [`add_vector_sse2`].
pub(crate) fn add_vector(a: &[u32], b: &[u32], out: &mut [u32], size: i32) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { add_vector_sse2(a, b, out, size) }
}

/// Entry point of [`add_vector_eq_sse2`].
pub(crate) fn add_vector_eq(a: &[u32], out: &mut [u32], size: i32) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { add_vector_eq_sse2(a, out, size) }
}

/// Entry point of [`combined_shannon_entropy_sse2`].
pub(crate) fn combined_shannon_entropy(x: &[u32], y: &[u32]) -> u64 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { combined_shannon_entropy_sse2(x, y) }
}

/// Entry point of [`vector_mismatch_sse2`].
pub(crate) fn vector_mismatch(array1: &[u32], array2: &[u32], length: i32) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { vector_mismatch_sse2(array1, array2, length) }
}

/// Entry point of [`bundle_color_map_sse2`].
pub(crate) fn bundle_color_map(row: &[u8], width: i32, xbits: i32, dst: &mut [u32]) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant,
    // so the CPU has SSE2.
    unsafe { bundle_color_map_sse2(row, width, xbits, dst) }
}

/// Generates the entry point of an SSE2 batch predictor for
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
            // SAFETY: called only after `features().sse2` selected the SSE2
            // variant, so the CPU has SSE2.
            unsafe { $body(data, in_pos, upper_pos, num_pixels, out) }
        }
    };
}

predictor_sub_entry!(predictor_sub0, predictor_sub0_sse2);
predictor_sub_entry!(predictor_sub1, predictor_sub1_sse2);
predictor_sub_entry!(predictor_sub2, predictor_sub2_sse2);
predictor_sub_entry!(predictor_sub3, predictor_sub3_sse2);
predictor_sub_entry!(predictor_sub4, predictor_sub4_sse2);
predictor_sub_entry!(predictor_sub5, predictor_sub5_sse2);
predictor_sub_entry!(predictor_sub6, predictor_sub6_sse2);
predictor_sub_entry!(predictor_sub7, predictor_sub7_sse2);
predictor_sub_entry!(predictor_sub8, predictor_sub8_sse2);
predictor_sub_entry!(predictor_sub9, predictor_sub9_sse2);
predictor_sub_entry!(predictor_sub10, predictor_sub10_sse2);
predictor_sub_entry!(predictor_sub11, predictor_sub11_sse2);
predictor_sub_entry!(predictor_sub12, predictor_sub12_sse2);
predictor_sub_entry!(predictor_sub13, predictor_sub13_sse2);

/// `VP8LPredictorsSub` as `VP8LEncDspInitSSE2` fills it, with
/// `PredictorSub0_SSE2` for the sentinels 14 and 15; it is also
/// `VP8LPredictorsSub_SSE`, the table the AVX2 functions fall back on.
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
