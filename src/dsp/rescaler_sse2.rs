// libwebp v1.6.0, src/dsp/rescaler_sse2.c:
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
// SSE2 Rescaling functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/rescaler_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP18 (SIMD x86 decoder).
//!
//! The SSE2 row import and export of the rescaler that
//! `WebPRescalerDspInitSSE2` assigns, behind the dispatch functions of
//! `src/dsp/rescaler.rs` (ADR 0008). Each `#[target_feature]` function keeps
//! the upstream name; each function that the `Init` function assigns to a
//! pointer has a safe `pub(crate)` entry point named after it without the
//! suffix, which the dispatch function calls after testing `features().sse2`.
//! `WebPRescalerDspInitSSE2` assigns `WebPRescalerImportRowExpand`,
//! `WebPRescalerImportRowShrink`, `WebPRescalerExportRowExpand` and
//! `WebPRescalerExportRowShrink`: [`rescaler_import_row_expand`],
//! [`rescaler_import_row_shrink`], [`rescaler_export_row_expand`] and
//! [`rescaler_export_row_shrink`]. The file is compiled without
//! `WEBP_REDUCE_SIZE`, as in the reference configuration.
//!
//! The rescaler follows `src/utils/rescaler_utils.rs` (WP4, deviation 8): it
//! owns its side rows `irow` and `frow`, and the export takes the output
//! buffer per call and writes the row at `dst_pos`, where C writes through
//! `wrk->dst`. A C pointer into a row is an index into it.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **The 32-bit store of `ImportRowExpand`.** C stores the low lane with
//!    `WebPInt32ToMem((uint8_t*)frow, ...)`, a `memcpy` into the `rescaler_t`;
//!    the translation assigns the same 32 bits to the `u32` element.
//! 3. **Panics on short buffers.** Each vector access slices the exact span it
//!    reads or writes, so a short row panics at that slice index, possibly
//!    before or after other writes than the scalar function makes
//!    (`common_sse2.rs`, deviation 3). Valid inputs never reach it.
//!
//! # Excluded undefined behavior
//!
//! 1. **Two or three channels in the expanding import.**
//!    `RescalerImportRowExpand_SSE2` handles four channels and treats every
//!    other count as one channel; for two or three channels with a source of
//!    eight or more pixels (and `x_add` below `1 << 15`), upstream's SSE2 and C
//!    results differ. The translation keeps upstream's SSE2 behavior. libwebp
//!    rescales with one or four channels only, and the parity tests compare
//!    two and three channels only where both take the C function (narrow
//!    sources and the shrinking import, which falls back to C for any count
//!    other than four).
//! 2. **States the rescaler cannot reach.** The export packs its 32-bit
//!    results with signed saturation where C keeps the low byte of a negative
//!    `int`, and the shrinking import keeps 16-bit sums; both agree with C on
//!    every state that `WebPRescalerInit` and the import produce, which the
//!    parity tests drive the rescaler through, and differ only on side rows no
//!    import writes.
//! 3. **Assertions.** The C `assert`s are `debug_assert!`s (ADR 0003).
//!
//! # Checks not applicable
//!
//! 1. **`WEBP_RESCALER_RFIX < 32`.** `WEBP_RESCALER_RFIX` is 32, so the branch
//!    of `ProcessRow_SSE2` for a smaller precision is not compiled and not
//!    translated.
//! 2. **`WebPRescalerDspInitSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.
//! 3. **Differential probe.** The parity tests compare the rescaler driven
//!    through each level with the C translation (`tests/wp18_rescaler.rs`);
//!    the diagnostic `release-simd` oracle runs the WP4 probe against
//!    upstream's SSE2.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi64, _mm_and_si128, _mm_cvtsi32_si128, _mm_cvtsi128_si32,
    _mm_insert_epi16, _mm_madd_epi16, _mm_mul_epu32, _mm_mulhi_epu16, _mm_mullo_epi16,
    _mm_or_si128, _mm_packs_epi32, _mm_packus_epi16, _mm_set_epi32, _mm_set1_epi16, _mm_set1_epi32,
    _mm_setzero_si128, _mm_shuffle_epi32, _mm_slli_epi64, _mm_srli_epi64, _mm_srli_si128,
    _mm_sub_epi32, _mm_sub_epi64, _mm_unpacklo_epi8, _mm_unpacklo_epi16, _mm_unpacklo_epi32,
};

use crate::dsp::common_sse2::{loadl_epi64, loadu_si128, storel_epi64, storeu_si128};
use crate::dsp::rescaler::{webp_rescaler_import_row_expand_c, webp_rescaler_import_row_shrink_c};
use crate::utils::rescaler_utils::{
    WEBP_RESCALER_ONE, WEBP_RESCALER_RFIX, WebPRescaler, rescaler_t, webp_rescaler_frac,
    webp_rescaler_input_done, webp_rescaler_output_done,
};
use crate::utils::utils::webp_mem_to_int32;

//------------------------------------------------------------------------------
// Implementations of critical functions ImportRow / ExportRow

// upstream: src/dsp/rescaler_sse2.c ROUNDER
const ROUNDER: u64 = WEBP_RESCALER_ONE >> 1;

/// `WEBP_RESCALER_RFIX` as the shift count of the 64-bit shift intrinsics.
const RFIX: i32 = WEBP_RESCALER_RFIX.cast_signed();

/// A C `int` index or count that the loops keep non-negative, as `usize`.
fn idx(i: i32) -> usize {
    usize::try_from(i).expect("a non-negative index")
}

/// The low 32 bits of a 64-bit value, as a C conversion to `uint32_t` keeps
/// them.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion keeps the low 32 bits"
)]
fn low32(value: u64) -> u32 {
    value as u32
}

/// The `int` value of a 64-bit value, as the C conversion `(int)` gives it.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion to int keeps the low 32 bits, read as two's complement"
)]
fn as_int(value: u64) -> i32 {
    value as i32
}

/// C's implicit conversion of an `int` to the `short` of `_mm_set1_epi16`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the int to short"
)]
fn to_short(value: i32) -> i16 {
    value as i16
}

/// `(v > 255) ? 255u : (uint8_t)v`: a negative `int` keeps its low byte.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "a C conversion to uint8_t keeps the low eight bits"
)]
fn clip_to_byte(v: i32) -> u8 {
    if v > 255 { 255 } else { v as u8 }
}

/// `ROUNDER` as the `int` argument of `_mm_set_epi32`: C converts the 64-bit
/// constant `1 << 31` to `int`, which keeps its low 32 bits.
fn rounder_int() -> i32 {
    low32(ROUNDER).cast_signed()
}

// upstream: src/dsp/rescaler_sse2.c MULT_FIX
fn mult_fix(x: u32, y: u32) -> u64 {
    (u64::from(x) * u64::from(y) + ROUNDER) >> WEBP_RESCALER_RFIX
}

// upstream: src/dsp/rescaler_sse2.c MULT_FIX_FLOOR
fn mult_fix_floor(x: u32, y: u32) -> u64 {
    (u64::from(x) * u64::from(y)) >> WEBP_RESCALER_RFIX
}

// upstream: src/dsp/rescaler_sse2.c LoadTwoPixels_SSE2
/// input: 8 bytes ABCDEFGH -> output: A0E0B0F0C0G0D0H0
#[target_feature(enable = "sse2")]
#[inline]
fn load_two_pixels_sse2(src: &[u8], out: &mut __m128i) {
    let zero = _mm_setzero_si128();
    let a = loadl_epi64(&src[..8]); // ABCDEFGH
    let b = _mm_unpacklo_epi8(a, zero); // A0B0C0D0E0F0G0H0
    let c = _mm_srli_si128::<8>(b); // E0F0G0H0
    *out = _mm_unpacklo_epi16(b, c);
}

// upstream: src/dsp/rescaler_sse2.c LoadEightPixels_SSE2
/// input: 8 bytes ABCDEFGH -> output: A0B0C0D0E0F0G0H0
#[target_feature(enable = "sse2")]
#[inline]
fn load_eight_pixels_sse2(src: &[u8], out: &mut __m128i) {
    let zero = _mm_setzero_si128();
    let a = loadl_epi64(&src[..8]); // ABCDEFGH
    *out = _mm_unpacklo_epi8(a, zero);
}

// upstream: src/dsp/rescaler_sse2.c RescalerImportRowExpand_SSE2
#[target_feature(enable = "sse2")]
fn rescaler_import_row_expand_sse2(wrk: &mut WebPRescaler, src: &[u8]) {
    let mut frow = 0;
    let frow_end = idx(wrk.dst_width * wrk.num_channels);
    let x_add = wrk.x_add;
    let mut accum = x_add;
    let mut cur_pixels = _mm_setzero_si128();

    // SSE2 implementation only works with 16b signed arithmetic at max.
    if wrk.src_width < 8 || accum >= (1 << 15) {
        webp_rescaler_import_row_expand_c(wrk, src);
        return;
    }

    debug_assert!(!webp_rescaler_input_done(wrk));
    debug_assert!(wrk.x_expand);
    let mut src_pos = 0;
    if wrk.num_channels == 4 {
        load_two_pixels_sse2(&src[src_pos..], &mut cur_pixels);
        src_pos += 4;
        loop {
            let mult = _mm_set1_epi32(((x_add - accum) << 16) | accum);
            let out = _mm_madd_epi16(cur_pixels, mult);
            storeu_si128(&mut wrk.frow[frow..frow + 4], out);
            frow += 4;
            if frow >= frow_end {
                break;
            }
            accum -= wrk.x_sub;
            if accum < 0 {
                load_two_pixels_sse2(&src[src_pos..], &mut cur_pixels);
                src_pos += 4;
                accum += x_add;
            }
        }
    } else {
        let src_limit = idx(wrk.src_width - 8);
        load_eight_pixels_sse2(&src[src_pos..], &mut cur_pixels);
        src_pos += 7;
        let mut left = 7;
        loop {
            let mult = _mm_cvtsi32_si128(((x_add - accum) << 16) | accum);
            let out = _mm_madd_epi16(cur_pixels, mult);
            debug_assert_eq!(size_of::<rescaler_t>(), size_of::<u32>());
            // C: WebPInt32ToMem((uint8_t*)frow, ...) (deviation 2).
            wrk.frow[frow] = _mm_cvtsi128_si32(out).cast_unsigned();
            frow += 1;
            if frow >= frow_end {
                break;
            }
            accum -= wrk.x_sub;
            if accum < 0 {
                left -= 1;
                if left != 0 {
                    cur_pixels = _mm_srli_si128::<2>(cur_pixels);
                } else if src_pos <= src_limit {
                    load_eight_pixels_sse2(&src[src_pos..], &mut cur_pixels);
                    src_pos += 7;
                    left = 7;
                } else {
                    // tail
                    cur_pixels = _mm_srli_si128::<2>(cur_pixels);
                    cur_pixels = _mm_insert_epi16::<1>(cur_pixels, i32::from(src[src_pos + 1]));
                    src_pos += 1;
                    left = 1;
                }
                accum += x_add;
            }
        }
    }
    debug_assert_eq!(accum, 0);
}

/// Entry point of [`rescaler_import_row_expand_sse2`] for
/// `webp_rescaler_import_row_expand`.
pub(crate) fn rescaler_import_row_expand(wrk: &mut WebPRescaler, src: &[u8]) {
    // SAFETY: `webp_rescaler_import_row_expand` calls this entry point only
    // after `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`, selected
    // the SSE2 variant, so the CPU has the function's only feature.
    unsafe { rescaler_import_row_expand_sse2(wrk, src) }
}

// upstream: src/dsp/rescaler_sse2.c RescalerImportRowShrink_SSE2
#[target_feature(enable = "sse2")]
fn rescaler_import_row_shrink_sse2(wrk: &mut WebPRescaler, src: &[u8]) {
    let x_sub = wrk.x_sub;
    let mut accum = 0;
    let zero = _mm_setzero_si128();
    let mult0 = _mm_set1_epi16(to_short(x_sub));
    let mult1 = _mm_set1_epi32(wrk.fx_scale.cast_signed());
    let rounder = _mm_set_epi32(0, rounder_int(), 0, rounder_int());
    let mut sum = zero;
    let mut frow = 0;
    let frow_end = idx(4 * wrk.dst_width);

    if wrk.num_channels != 4 || wrk.x_add > (x_sub << 7) {
        webp_rescaler_import_row_shrink_c(wrk, src);
        return;
    }
    debug_assert!(!webp_rescaler_input_done(wrk));
    debug_assert!(!wrk.x_expand);

    let mut src_pos = 0;
    while frow < frow_end {
        let mut base = zero;
        accum += wrk.x_add;
        while accum > 0 {
            let a = _mm_cvtsi32_si128(webp_mem_to_int32(&src[src_pos..]));
            src_pos += 4;
            base = _mm_unpacklo_epi8(a, zero);
            // To avoid overflow, we need: base * x_add / x_sub < 32768
            // => x_add < x_sub << 7. That's a 1/128 reduction ratio limit.
            sum = _mm_add_epi16(sum, base);
            accum -= x_sub;
        }
        {
            // Emit next horizontal pixel.
            let mult = _mm_set1_epi16(to_short(-accum));
            let frac0 = _mm_mullo_epi16(base, mult); // 16b x 16b -> 32b
            let frac1 = _mm_mulhi_epu16(base, mult);
            let frac = _mm_unpacklo_epi16(frac0, frac1); // frac is 32b
            let a0 = _mm_mullo_epi16(sum, mult0);
            let a1 = _mm_mulhi_epu16(sum, mult0);
            let b0 = _mm_unpacklo_epi16(a0, a1); // sum * x_sub
            let frow_out = _mm_sub_epi32(b0, frac); // sum * x_sub - frac
            let d0 = _mm_srli_epi64::<32>(frac);
            let d1 = _mm_mul_epu32(frac, mult1); // 32b x 16b -> 64b
            let d2 = _mm_mul_epu32(d0, mult1);
            let e1 = _mm_add_epi64(d1, rounder);
            let e2 = _mm_add_epi64(d2, rounder);
            let f1 = _mm_shuffle_epi32::<{ 1 | (3 << 2) }>(e1);
            let f2 = _mm_shuffle_epi32::<{ 1 | (3 << 2) }>(e2);
            let g = _mm_unpacklo_epi32(f1, f2);
            sum = _mm_packs_epi32(g, zero);
            storeu_si128(&mut wrk.frow[frow..frow + 4], frow_out);
        }
        frow += 4;
    }
    debug_assert_eq!(accum, 0);
}

/// Entry point of [`rescaler_import_row_shrink_sse2`] for
/// `webp_rescaler_import_row_shrink`.
pub(crate) fn rescaler_import_row_shrink(wrk: &mut WebPRescaler, src: &[u8]) {
    // SAFETY: `webp_rescaler_import_row_shrink` calls this entry point only
    // after `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { rescaler_import_row_shrink_sse2(wrk, src) }
}

//------------------------------------------------------------------------------
// Row export

// upstream: src/dsp/rescaler_sse2.c LoadDispatchAndMult_SSE2
/// load *src as epi64, multiply by mult and store result in [out0 ... out3]
#[target_feature(enable = "sse2")]
#[inline]
fn load_dispatch_and_mult_sse2(
    src: &[rescaler_t],
    mult: Option<&__m128i>,
    out0: &mut __m128i,
    out1: &mut __m128i,
    out2: &mut __m128i,
    out3: &mut __m128i,
) {
    let a0 = loadu_si128(&src[0..4]);
    let a1 = loadu_si128(&src[4..8]);
    let a2 = _mm_srli_epi64::<32>(a0);
    let a3 = _mm_srli_epi64::<32>(a1);
    if let Some(mult) = mult {
        *out0 = _mm_mul_epu32(a0, *mult);
        *out1 = _mm_mul_epu32(a1, *mult);
        *out2 = _mm_mul_epu32(a2, *mult);
        *out3 = _mm_mul_epu32(a3, *mult);
    } else {
        *out0 = a0;
        *out1 = a1;
        *out2 = a2;
        *out3 = a3;
    }
}

// upstream: src/dsp/rescaler_sse2.c ProcessRow_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn process_row_sse2(
    a0: &__m128i,
    a1: &__m128i,
    a2: &__m128i,
    a3: &__m128i,
    mult: &__m128i,
    dst: &mut [u8],
) {
    let rounder = _mm_set_epi32(0, rounder_int(), 0, rounder_int());
    let mask = _mm_set_epi32(!0, 0, !0, 0);
    let b0 = _mm_mul_epu32(*a0, *mult);
    let b1 = _mm_mul_epu32(*a1, *mult);
    let b2 = _mm_mul_epu32(*a2, *mult);
    let b3 = _mm_mul_epu32(*a3, *mult);
    let c0 = _mm_add_epi64(b0, rounder);
    let c1 = _mm_add_epi64(b1, rounder);
    let c2 = _mm_add_epi64(b2, rounder);
    let c3 = _mm_add_epi64(b3, rounder);
    let d0 = _mm_srli_epi64::<RFIX>(c0);
    let d1 = _mm_srli_epi64::<RFIX>(c1);
    // WEBP_RESCALER_RFIX is 32: the `#else` branch of upstream's
    // `#if (WEBP_RESCALER_RFIX < 32)`.
    let d2 = _mm_and_si128(c2, mask);
    let d3 = _mm_and_si128(c3, mask);
    let e0 = _mm_or_si128(d0, d2);
    let e1 = _mm_or_si128(d1, d3);
    let f = _mm_packs_epi32(e0, e1);
    let g = _mm_packus_epi16(f, f);
    storel_epi64(&mut dst[..8], g);
}

// upstream: src/dsp/rescaler_sse2.c RescalerExportRowExpand_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::many_single_char_names, reason = "the C names a, b, j and v")]
fn rescaler_export_row_expand_sse2(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    debug_assert!(!webp_rescaler_output_done(wrk));
    debug_assert!(wrk.y_accum <= 0 && wrk.y_sub + wrk.y_accum >= 0);
    debug_assert!(wrk.y_expand);
    let dst = &mut dst[wrk.dst_pos..];
    let irow = &wrk.irow;
    let x_out_max = wrk.dst_width * wrk.num_channels;
    let frow = &wrk.frow;
    let fy_scale = wrk.fy_scale.cast_signed();
    let mult = _mm_set_epi32(0, fy_scale, 0, fy_scale);

    let zero = _mm_setzero_si128();
    let (mut a0, mut a1, mut a2, mut a3) = (zero, zero, zero, zero);
    let mut x_out = 0;
    if wrk.y_accum == 0 {
        while x_out + 8 <= x_out_max {
            let k = idx(x_out);
            load_dispatch_and_mult_sse2(&frow[k..k + 8], None, &mut a0, &mut a1, &mut a2, &mut a3);
            process_row_sse2(&a0, &a1, &a2, &a3, &mult, &mut dst[k..k + 8]);
            x_out += 8;
        }
        while x_out < x_out_max {
            let k = idx(x_out);
            let j = frow[k];
            let v = as_int(mult_fix(j, wrk.fy_scale));
            dst[k] = clip_to_byte(v);
            x_out += 1;
        }
    } else {
        let b = webp_rescaler_frac(-wrk.y_accum, wrk.y_sub);
        let a = low32(WEBP_RESCALER_ONE - u64::from(b));
        let m_a = _mm_set_epi32(0, a.cast_signed(), 0, a.cast_signed());
        let m_b = _mm_set_epi32(0, b.cast_signed(), 0, b.cast_signed());
        let rounder = _mm_set_epi32(0, rounder_int(), 0, rounder_int());
        let (mut b0, mut b1, mut b2, mut b3) = (zero, zero, zero, zero);
        while x_out + 8 <= x_out_max {
            let k = idx(x_out);
            load_dispatch_and_mult_sse2(
                &frow[k..k + 8],
                Some(&m_a),
                &mut a0,
                &mut a1,
                &mut a2,
                &mut a3,
            );
            load_dispatch_and_mult_sse2(
                &irow[k..k + 8],
                Some(&m_b),
                &mut b0,
                &mut b1,
                &mut b2,
                &mut b3,
            );
            {
                let c0 = _mm_add_epi64(a0, b0);
                let c1 = _mm_add_epi64(a1, b1);
                let c2 = _mm_add_epi64(a2, b2);
                let c3 = _mm_add_epi64(a3, b3);
                let d0 = _mm_add_epi64(c0, rounder);
                let d1 = _mm_add_epi64(c1, rounder);
                let d2 = _mm_add_epi64(c2, rounder);
                let d3 = _mm_add_epi64(c3, rounder);
                let e0 = _mm_srli_epi64::<RFIX>(d0);
                let e1 = _mm_srli_epi64::<RFIX>(d1);
                let e2 = _mm_srli_epi64::<RFIX>(d2);
                let e3 = _mm_srli_epi64::<RFIX>(d3);
                process_row_sse2(&e0, &e1, &e2, &e3, &mult, &mut dst[k..k + 8]);
            }
            x_out += 8;
        }
        while x_out < x_out_max {
            let k = idx(x_out);
            let i = u64::from(a)
                .wrapping_mul(u64::from(frow[k]))
                .wrapping_add(u64::from(b).wrapping_mul(u64::from(irow[k])));
            let j = low32(i.wrapping_add(ROUNDER) >> WEBP_RESCALER_RFIX);
            let v = as_int(mult_fix(j, wrk.fy_scale));
            dst[k] = clip_to_byte(v);
            x_out += 1;
        }
    }
}

/// Entry point of [`rescaler_export_row_expand_sse2`] for
/// `webp_rescaler_export_row_expand`.
pub(crate) fn rescaler_export_row_expand(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    // SAFETY: `webp_rescaler_export_row_expand` calls this entry point only
    // after `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { rescaler_export_row_expand_sse2(wrk, dst) }
}

// upstream: src/dsp/rescaler_sse2.c RescalerExportRowShrink_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C names mult_xy and mult_y")]
fn rescaler_export_row_shrink_sse2(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    debug_assert!(!webp_rescaler_output_done(wrk));
    debug_assert!(wrk.y_accum <= 0);
    debug_assert!(!wrk.y_expand);
    let dst = &mut dst[wrk.dst_pos..];
    let irow = &mut wrk.irow;
    let x_out_max = wrk.dst_width * wrk.num_channels;
    let frow = &wrk.frow;
    let yscale = wrk.fy_scale.wrapping_mul((-wrk.y_accum).cast_unsigned());
    let zero = _mm_setzero_si128();
    let (mut a0, mut a1, mut a2, mut a3) = (zero, zero, zero, zero);
    let mut x_out = 0;
    if yscale != 0 {
        let scale_xy = wrk.fxy_scale.cast_signed();
        let mult_xy = _mm_set_epi32(0, scale_xy, 0, scale_xy);
        let mult_y = _mm_set_epi32(0, yscale.cast_signed(), 0, yscale.cast_signed());
        let (mut b0, mut b1, mut b2, mut b3) = (zero, zero, zero, zero);
        while x_out + 8 <= x_out_max {
            let k = idx(x_out);
            load_dispatch_and_mult_sse2(&irow[k..k + 8], None, &mut a0, &mut a1, &mut a2, &mut a3);
            load_dispatch_and_mult_sse2(
                &frow[k..k + 8],
                Some(&mult_y),
                &mut b0,
                &mut b1,
                &mut b2,
                &mut b3,
            );
            {
                let d0 = _mm_srli_epi64::<RFIX>(b0); // = frac
                let d1 = _mm_srli_epi64::<RFIX>(b1);
                let d2 = _mm_srli_epi64::<RFIX>(b2);
                let d3 = _mm_srli_epi64::<RFIX>(b3);
                let e0 = _mm_sub_epi64(a0, d0); // irow[x] - frac
                let e1 = _mm_sub_epi64(a1, d1);
                let e2 = _mm_sub_epi64(a2, d2);
                let e3 = _mm_sub_epi64(a3, d3);
                let f2 = _mm_slli_epi64::<32>(d2);
                let f3 = _mm_slli_epi64::<32>(d3);
                let g0 = _mm_or_si128(d0, f2);
                let g1 = _mm_or_si128(d1, f3);
                storeu_si128(&mut irow[k..k + 4], g0);
                storeu_si128(&mut irow[k + 4..k + 8], g1);
                process_row_sse2(&e0, &e1, &e2, &e3, &mult_xy, &mut dst[k..k + 8]);
            }
            x_out += 8;
        }
        while x_out < x_out_max {
            let k = idx(x_out);
            let frac = low32(mult_fix_floor(frow[k], yscale));
            let v = as_int(mult_fix(irow[k].wrapping_sub(frac), wrk.fxy_scale));
            dst[k] = clip_to_byte(v);
            irow[k] = frac; // new fractional start
            x_out += 1;
        }
    } else {
        let scale = wrk.fxy_scale;
        let mult = _mm_set_epi32(0, scale.cast_signed(), 0, scale.cast_signed());
        while x_out + 8 <= x_out_max {
            let k = idx(x_out);
            load_dispatch_and_mult_sse2(&irow[k..k + 8], None, &mut a0, &mut a1, &mut a2, &mut a3);
            storeu_si128(&mut irow[k..k + 4], zero);
            storeu_si128(&mut irow[k + 4..k + 8], zero);
            process_row_sse2(&a0, &a1, &a2, &a3, &mult, &mut dst[k..k + 8]);
            x_out += 8;
        }
        while x_out < x_out_max {
            let k = idx(x_out);
            let v = as_int(mult_fix(irow[k], scale));
            dst[k] = clip_to_byte(v);
            irow[k] = 0;
            x_out += 1;
        }
    }
}

/// Entry point of [`rescaler_export_row_shrink_sse2`] for
/// `webp_rescaler_export_row_shrink`.
pub(crate) fn rescaler_export_row_shrink(wrk: &mut WebPRescaler, dst: &mut [u8]) {
    // SAFETY: `webp_rescaler_export_row_shrink` calls this entry point only
    // after `features().sse2` selected the SSE2 variant, so the CPU has SSE2.
    unsafe { rescaler_export_row_shrink_sse2(wrk, dst) }
}

// upstream: src/dsp/rescaler_sse2.c WebPRescalerDspInitSSE2 (no counterpart: it only assigns pointers, ADR 0003; the module documentation lists them)
