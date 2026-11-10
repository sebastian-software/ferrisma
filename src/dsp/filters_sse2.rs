// libwebp v1.6.0, src/dsp/filters_sse2.c:
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
// SSE2 variant of alpha filters
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/filters_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE2 alpha-plane filters and unfilters that `VP8FiltersInitSSE2`
//! assigns, behind `webp_filters` and `webp_unfilters` in `src/dsp/filters.rs`
//! (ADR 0008). Each `#[target_feature]` function keeps the upstream name; each
//! function that the `Init` function assigns to a table entry has a safe
//! `pub(crate)` entry point named after it without the suffix, which the
//! dispatch function calls after testing `features().sse2`.
//! `VP8FiltersInitSSE2` assigns:
//!
//! - `WebPUnfilters[WEBP_FILTER_HORIZONTAL]`, `[WEBP_FILTER_VERTICAL]`,
//!   `[WEBP_FILTER_GRADIENT]`: [`horizontal_unfilter`],
//!   [`vertical_unfilter`], [`gradient_unfilter`] (the vertical one unless
//!   `CHROMIUM` is defined, which the reference configuration does not);
//! - `WebPFilters[WEBP_FILTER_HORIZONTAL]`, `[WEBP_FILTER_VERTICAL]`,
//!   `[WEBP_FILTER_GRADIENT]`: [`horizontal_filter`], [`vertical_filter`],
//!   [`gradient_filter`].
//!
//! `WebPUnfilters[WEBP_FILTER_NONE]` keeps the C function, and
//! `WebPFilters[WEBP_FILTER_NONE]` stays NULL. The planes, rows and strides
//! follow `src/dsp/filters.rs` (WP4): a C pointer from which the code also
//! reads backwards (`src[i - 1]`, `top[i - 1]`, `row[-1]`) is a slice and the
//! position of the pointer in it.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **No in-place forms.** C may unfilter in place (`in == out`) or with the
//!    previous row equal to the output (`prev == out`); the Rust signature of
//!    `WebPUnfilterFunc` takes distinct slices, and a caller that needs the
//!    in-place form passes a copy (WP4, deviation 15). The SSE2 unfilters read
//!    each input span before they write the same span, so the copy gives
//!    upstream's bytes for `in == out`. For `prev == out` upstream's
//!    `GradientUnfilter_SSE2` does not: it writes `out[0]` and then
//!    `GradientPredictInverse_SSE2` reads `top[-1]`, the same byte, where
//!    `GradientUnfilter_C` reads `prev` before writing ("in case prev==out").
//!    With the copy the translation gives the scalar result there; upstream's
//!    SSE2 gives another (the `release-simd` oracle shows it for every width
//!    from 2 on in `tests/wp4_filters.rs`). The alpha decoder passes distinct
//!    rows, so no decoding reaches the aliasing form.
//! 3. **`DCHECK`** becomes `debug_assert!`s of the width, height and stride
//!    conditions (ADR 0003); its `NULL` and `in != out` assertions hold by the
//!    borrow rules.
//! 4. **Panics on short buffers.** Each vector access slices the exact span it
//!    reads or writes, so a short buffer panics at that slice index, possibly
//!    before or after other writes than the scalar function makes
//!    (`common_sse2.rs`, deviation 3). Valid inputs never reach it.
//!
//! # Excluded undefined behavior
//!
//! 1. **Zero-width unfilters.** `HorizontalUnfilter_SSE2`, and through it the
//!    vertical and gradient unfilters without a previous row, and
//!    `GradientUnfilter_SSE2` with one, write `out[0]` before they test the
//!    width, where the C functions write nothing for a width of 0. This is
//!    upstream's SSE2 behavior, which the translation keeps; a zero width
//!    with empty slices panics at `in[0]`. The alpha decoder never passes a
//!    width below 1, and the parity tests compare widths from 1 on (and 0 for
//!    the vertical unfilter with a previous row, where both agree).
//! 2. **Filter sizes.** `DCHECK` requires a positive width and height and a
//!    stride of at least the width; the forward filters are compared only
//!    there, as the scalar translation panics on a non-positive width.
//!
//! # Checks not applicable
//!
//! 1. **`VP8FiltersInitSSE2`** only assigns table entries (ADR 0003); its
//!    assignments are the selection in `webp_filters` and `webp_unfilters`.
//!    The `CHROMIUM` branch, which keeps the C vertical unfilter, is not
//!    compiled in the reference configuration.
//! 2. **Differential probe.** The parity tests compare each function with the
//!    C translation it replaces (`tests/wp18_filters.rs`); the diagnostic
//!    `release-simd` oracle runs the WP4 probe against upstream's SSE2.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    _mm_add_epi8, _mm_add_epi16, _mm_and_si128, _mm_or_si128, _mm_packus_epi16, _mm_set_epi32,
    _mm_setzero_si128, _mm_slli_si128, _mm_srli_epi64, _mm_srli_si128, _mm_sub_epi8, _mm_sub_epi16,
    _mm_unpacklo_epi8,
};

use crate::dsp::common_sse2::{loadl_epi64, loadu_si128, storel_epi64, storeu_si128};

/// A C `int` index or count that the loops keep non-negative, as `usize`.
fn idx(i: i32) -> usize {
    usize::try_from(i).expect("a non-negative index")
}

/// The low byte of a C `int`, as a conversion to `uint8_t` keeps it.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "a C conversion to uint8_t keeps the low eight bits"
)]
fn low_byte(value: i32) -> u8 {
    value as u8
}

//------------------------------------------------------------------------------
// Helpful macro.

// upstream: src/dsp/filters_sse2.c DCHECK
/// The `DCHECK` assertions of the forward filters (deviation 3).
fn dcheck(width: i32, height: i32, stride: i32) {
    debug_assert!(width > 0);
    debug_assert!(height > 0);
    debug_assert!(stride >= width);
}

// upstream: src/dsp/filters_sse2.c PredictLineTop_SSE2
#[target_feature(enable = "sse2")]
fn predict_line_top_sse2(src: &[u8], pred: &[u8], dst: &mut [u8], length: i32) {
    let max_pos = length & !31;
    debug_assert!(length >= 0);
    let mut i = 0;
    while i < max_pos {
        let k = idx(i);
        let a0 = loadu_si128(&src[k..k + 16]);
        let a1 = loadu_si128(&src[k + 16..k + 32]);
        let b0 = loadu_si128(&pred[k..k + 16]);
        let b1 = loadu_si128(&pred[k + 16..k + 32]);
        let c0 = _mm_sub_epi8(a0, b0);
        let c1 = _mm_sub_epi8(a1, b1);
        storeu_si128(&mut dst[k..k + 16], c0);
        storeu_si128(&mut dst[k + 16..k + 32], c1);
        i += 32;
    }
    while i < length {
        let k = idx(i);
        dst[k] = src[k].wrapping_sub(pred[k]);
        i += 1;
    }
}

// upstream: src/dsp/filters_sse2.c PredictLineLeft_SSE2
/// Special case for left-based prediction (when preds==dst-1 or preds==src-1).
/// C's `src` is `src[src_pos..]`, which the function also reads one byte
/// before.
#[target_feature(enable = "sse2")]
fn predict_line_left_sse2(src: &[u8], src_pos: usize, dst: &mut [u8], length: i32) {
    let max_pos = length & !31;
    debug_assert!(length >= 0);
    let mut i = 0;
    while i < max_pos {
        let k = src_pos + idx(i);
        let a0 = loadu_si128(&src[k..k + 16]);
        let b0 = loadu_si128(&src[k - 1..k + 15]);
        let a1 = loadu_si128(&src[k + 16..k + 32]);
        let b1 = loadu_si128(&src[k + 15..k + 31]);
        let c0 = _mm_sub_epi8(a0, b0);
        let c1 = _mm_sub_epi8(a1, b1);
        let d = idx(i);
        storeu_si128(&mut dst[d..d + 16], c0);
        storeu_si128(&mut dst[d + 16..d + 32], c1);
        i += 32;
    }
    while i < length {
        let k = src_pos + idx(i);
        dst[idx(i)] = src[k].wrapping_sub(src[k - 1]);
        i += 1;
    }
}

//------------------------------------------------------------------------------
// Horizontal filter.

// upstream: src/dsp/filters_sse2.c DoHorizontalFilter_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn do_horizontal_filter_sse2(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    dcheck(width, height, stride);
    let stride = idx(stride);

    // Leftmost pixel is the same as input for topmost scanline.
    out[0] = input[0];
    predict_line_left_sse2(input, 1, &mut out[1..], width - 1);

    // Filter line-by-line.
    for row in 1..idx(height) {
        let cur = row * stride;
        // Leftmost pixel is predicted from above.
        out[cur] = input[cur].wrapping_sub(input[cur - stride]);
        predict_line_left_sse2(input, cur + 1, &mut out[cur + 1..], width - 1);
    }
}

//------------------------------------------------------------------------------
// Vertical filter.

// upstream: src/dsp/filters_sse2.c DoVerticalFilter_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn do_vertical_filter_sse2(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    dcheck(width, height, stride);
    let stride = idx(stride);

    // Very first top-left pixel is copied.
    out[0] = input[0];
    // Rest of top scan-line is left-predicted.
    predict_line_left_sse2(input, 1, &mut out[1..], width - 1);

    // Filter line-by-line.
    for row in 1..idx(height) {
        let cur = row * stride;
        predict_line_top_sse2(
            &input[cur..],
            &input[cur - stride..],
            &mut out[cur..],
            width,
        );
    }
}

//------------------------------------------------------------------------------
// Gradient filter.

// upstream: src/dsp/filters_sse2.c GradientPredictor_SSE2
fn gradient_predictor_sse2(a: u8, b: u8, c: u8) -> i32 {
    let g = i32::from(a) + i32::from(b) - i32::from(c);
    if (g & !0xff) == 0 {
        g
    } else if g < 0 {
        0
    } else {
        255
    } // clip to 8bit
}

// upstream: src/dsp/filters_sse2.c GradientPredictDirect_SSE2
/// C's `row` and `top` are `row[row_pos..]` and `top[top_pos..]`, which the
/// function also reads one byte before.
#[target_feature(enable = "sse2")]
#[expect(clippy::many_single_char_names, reason = "the C register names")]
fn gradient_predict_direct_sse2(
    row: &[u8],
    row_pos: usize,
    top: &[u8],
    top_pos: usize,
    out: &mut [u8],
    length: i32,
) {
    let max_pos = length & !7;
    let zero = _mm_setzero_si128();
    let mut i = 0;
    while i < max_pos {
        let r = row_pos + idx(i);
        let t = top_pos + idx(i);
        let a0 = loadl_epi64(&row[r - 1..r + 7]);
        let b0 = loadl_epi64(&top[t..t + 8]);
        let c0 = loadl_epi64(&top[t - 1..t + 7]);
        let d = loadl_epi64(&row[r..r + 8]);
        let a1 = _mm_unpacklo_epi8(a0, zero);
        let b1 = _mm_unpacklo_epi8(b0, zero);
        let c1 = _mm_unpacklo_epi8(c0, zero);
        let e = _mm_add_epi16(a1, b1);
        let f = _mm_sub_epi16(e, c1);
        let g = _mm_packus_epi16(f, zero);
        let h = _mm_sub_epi8(d, g);
        let o = idx(i);
        storel_epi64(&mut out[o..o + 8], h);
        i += 8;
    }
    while i < length {
        let r = row_pos + idx(i);
        let t = top_pos + idx(i);
        let delta = gradient_predictor_sse2(row[r - 1], top[t], top[t - 1]);
        out[idx(i)] = low_byte(i32::from(row[r]) - delta);
        i += 1;
    }
}

// upstream: src/dsp/filters_sse2.c DoGradientFilter_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn do_gradient_filter_sse2(input: &[u8], width: i32, height: i32, stride: i32, out: &mut [u8]) {
    dcheck(width, height, stride);
    let stride = idx(stride);

    // left prediction for top scan-line
    out[0] = input[0];
    predict_line_left_sse2(input, 1, &mut out[1..], width - 1);

    // Filter line-by-line.
    for row in 1..idx(height) {
        let cur = row * stride;
        out[cur] = low_byte(i32::from(input[cur]) - i32::from(input[cur - stride]));
        gradient_predict_direct_sse2(
            input,
            cur + 1,
            input,
            cur + 1 - stride,
            &mut out[cur + 1..],
            width - 1,
        );
    }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/filters_sse2.c HorizontalFilter_SSE2
#[target_feature(enable = "sse2")]
fn horizontal_filter_sse2(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_horizontal_filter_sse2(data, width, height, stride, filtered_data);
}

/// Entry point of [`horizontal_filter_sse2`] for `webp_filters` with
/// `WEBP_FILTER_HORIZONTAL`.
pub(crate) fn horizontal_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    // SAFETY: `webp_filters` calls this entry point only after
    // `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected the
    // SSE2 variant, so the CPU has the function's only feature.
    unsafe { horizontal_filter_sse2(data, width, height, stride, filtered_data) }
}

// upstream: src/dsp/filters_sse2.c VerticalFilter_SSE2
#[target_feature(enable = "sse2")]
fn vertical_filter_sse2(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_vertical_filter_sse2(data, width, height, stride, filtered_data);
}

/// Entry point of [`vertical_filter_sse2`] for `webp_filters` with
/// `WEBP_FILTER_VERTICAL`.
pub(crate) fn vertical_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    // SAFETY: `webp_filters` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { vertical_filter_sse2(data, width, height, stride, filtered_data) }
}

// upstream: src/dsp/filters_sse2.c GradientFilter_SSE2
#[target_feature(enable = "sse2")]
fn gradient_filter_sse2(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    do_gradient_filter_sse2(data, width, height, stride, filtered_data);
}

/// Entry point of [`gradient_filter_sse2`] for `webp_filters` with
/// `WEBP_FILTER_GRADIENT`.
pub(crate) fn gradient_filter(
    data: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    filtered_data: &mut [u8],
) {
    // SAFETY: `webp_filters` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { gradient_filter_sse2(data, width, height, stride, filtered_data) }
}

//------------------------------------------------------------------------------
// Inverse transforms

// upstream: src/dsp/filters_sse2.c HorizontalUnfilter_SSE2
#[target_feature(enable = "sse2")]
fn horizontal_unfilter_sse2(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    out[0] = input[0].wrapping_add(prev.map_or(0, |prev| prev[0]));
    if width <= 1 {
        return;
    }
    let mut last = _mm_set_epi32(0, 0, 0, i32::from(out[0]));
    let mut i = 1;
    while i + 8 <= width {
        let k = idx(i);
        let a0 = loadl_epi64(&input[k..k + 8]);
        let a1 = _mm_add_epi8(a0, last);
        let a2 = _mm_slli_si128::<1>(a1);
        let a3 = _mm_add_epi8(a1, a2);
        let a4 = _mm_slli_si128::<2>(a3);
        let a5 = _mm_add_epi8(a3, a4);
        let a6 = _mm_slli_si128::<4>(a5);
        let a7 = _mm_add_epi8(a5, a6);
        storel_epi64(&mut out[k..k + 8], a7);
        last = _mm_srli_epi64::<56>(a7);
        i += 8;
    }
    while i < width {
        let k = idx(i);
        out[k] = input[k].wrapping_add(out[k - 1]);
        i += 1;
    }
}

/// Entry point of [`horizontal_unfilter_sse2`] for `webp_unfilters` with
/// `WEBP_FILTER_HORIZONTAL`.
pub(crate) fn horizontal_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    // SAFETY: `webp_unfilters` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { horizontal_unfilter_sse2(prev, input, out, width) }
}

// upstream: src/dsp/filters_sse2.c VerticalUnfilter_SSE2
#[target_feature(enable = "sse2")]
fn vertical_unfilter_sse2(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    match prev {
        None => horizontal_unfilter_sse2(None, input, out, width),
        Some(prev) => {
            let max_pos = width & !31;
            debug_assert!(width >= 0);
            let mut i = 0;
            while i < max_pos {
                let k = idx(i);
                let a0 = loadu_si128(&input[k..k + 16]);
                let a1 = loadu_si128(&input[k + 16..k + 32]);
                let b0 = loadu_si128(&prev[k..k + 16]);
                let b1 = loadu_si128(&prev[k + 16..k + 32]);
                let c0 = _mm_add_epi8(a0, b0);
                let c1 = _mm_add_epi8(a1, b1);
                storeu_si128(&mut out[k..k + 16], c0);
                storeu_si128(&mut out[k + 16..k + 32], c1);
                i += 32;
            }
            while i < width {
                let k = idx(i);
                out[k] = input[k].wrapping_add(prev[k]);
                i += 1;
            }
        }
    }
}

/// Entry point of [`vertical_unfilter_sse2`] for `webp_unfilters` with
/// `WEBP_FILTER_VERTICAL`.
pub(crate) fn vertical_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    // SAFETY: `webp_unfilters` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { vertical_unfilter_sse2(prev, input, out, width) }
}

// upstream: src/dsp/filters_sse2.c GradientPredictInverse_SSE2
/// C's `top` and `row` are `top[top_pos..]` and `row[row_pos..]`, which the
/// function also reads one byte before.
#[target_feature(enable = "sse2")]
#[expect(clippy::many_single_char_names, reason = "the C register names")]
fn gradient_predict_inverse_sse2(
    input: &[u8],
    top: &[u8],
    top_pos: usize,
    row: &mut [u8],
    row_pos: usize,
    length: i32,
) {
    if length > 0 {
        let max_pos = length & !7;
        let zero = _mm_setzero_si128();
        let mut a = _mm_set_epi32(0, 0, 0, i32::from(row[row_pos - 1])); // left sample
        let mut i = 0;
        while i < max_pos {
            let p = idx(i);
            let t = top_pos + p;
            let tmp0 = loadl_epi64(&top[t..t + 8]);
            let tmp1 = loadl_epi64(&top[t - 1..t + 7]);
            let b = _mm_unpacklo_epi8(tmp0, zero);
            let c = _mm_unpacklo_epi8(tmp1, zero);
            let d = loadl_epi64(&input[p..p + 8]); // base input
            let e = _mm_sub_epi16(b, c); // unclipped gradient basis B - C
            let mut out = zero; // accumulator for output
            let mut mask_hi = _mm_set_epi32(0, 0, 0, 0xff);
            let mut k = 8;
            loop {
                let tmp3 = _mm_add_epi16(a, e); // delta = A + B - C
                let tmp4 = _mm_packus_epi16(tmp3, zero); // saturate delta
                let tmp5 = _mm_add_epi8(tmp4, d); // add to in[]
                a = _mm_and_si128(tmp5, mask_hi); // 1-complement clip
                out = _mm_or_si128(out, a); // accumulate output
                k -= 1;
                if k == 0 {
                    break;
                }
                a = _mm_slli_si128::<1>(a); // rotate left sample
                mask_hi = _mm_slli_si128::<1>(mask_hi); // rotate mask
                a = _mm_unpacklo_epi8(a, zero); // convert 8b->16b
            }
            a = _mm_srli_si128::<7>(a); // prepare left sample for next iteration
            let r = row_pos + p;
            storel_epi64(&mut row[r..r + 8], out);
            i += 8;
        }
        while i < length {
            let p = idx(i);
            let t = top_pos + p;
            let r = row_pos + p;
            let delta = gradient_predictor_sse2(row[r - 1], top[t], top[t - 1]);
            row[r] = low_byte(i32::from(input[p]) + delta);
            i += 1;
        }
    }
}

// upstream: src/dsp/filters_sse2.c GradientUnfilter_SSE2
#[target_feature(enable = "sse2")]
fn gradient_unfilter_sse2(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    match prev {
        None => horizontal_unfilter_sse2(None, input, out, width),
        Some(prev) => {
            out[0] = input[0].wrapping_add(prev[0]); // predict from above
            gradient_predict_inverse_sse2(&input[1..], prev, 1, out, 1, width - 1);
        }
    }
}

/// Entry point of [`gradient_unfilter_sse2`] for `webp_unfilters` with
/// `WEBP_FILTER_GRADIENT`.
pub(crate) fn gradient_unfilter(prev: Option<&[u8]>, input: &[u8], out: &mut [u8], width: i32) {
    // SAFETY: `webp_unfilters` calls this entry point only after
    // `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { gradient_unfilter_sse2(prev, input, out, width) }
}

// upstream: src/dsp/filters_sse2.c VP8FiltersInitSSE2 (no counterpart: it only assigns table entries, ADR 0003; the module documentation lists them)
