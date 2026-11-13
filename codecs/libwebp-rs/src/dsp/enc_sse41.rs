// libwebp v1.6.0, src/dsp/enc_sse41.c:
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
// SSE4 version of some encoding functions.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/enc_sse41.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE4.1 variants that `VP8EncDspInitSSE41` assigns, behind the
//! dispatch functions of `src/dsp/enc.rs` (ADR 0008). `VP8EncDspInit` runs
//! `VP8EncDspInitSSE41` only inside its SSE2 branch, so a dispatch function
//! selects these entry points when both `features().sse2` and
//! `features().sse4_1` hold, before the SSE2 variant. `VP8EncDspInitSSE41`
//! assigns `VP8CollectHistogram`, `VP8EncQuantizeBlock`,
//! `VP8EncQuantize2Blocks`, `VP8EncQuantizeBlockWHT`, `VP8TDisto4x4` and
//! `VP8TDisto16x16`. `CollectHistogram_SSE41` calls the `VP8FTransform`
//! pointer, which is `vp8f_transform` here.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): this file has no aligned access.
//! 2. **Pointers** are slices as in `enc_sse2.rs` (its deviation 2).
//! 3. **`PSHUFB_CST`** is a function.
//! 4. **`TTransform_SSE41`** loads 16 bytes of the first three rows and 8 of
//!    the last, and uses the low four of each; the translation loads the four
//!    it uses (`enc_sse2.rs`, deviation 5). The results are the same.
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions'.
//!
//! # Checks not applicable
//!
//! 1. **`VP8EncDspInitSSE41`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_abs_epi16, _mm_add_epi16, _mm_add_epi32, _mm_cmpeq_epi8, _mm_cvtepu8_epi16,
    _mm_madd_epi16, _mm_min_epi16, _mm_movemask_epi8, _mm_mulhi_epu16, _mm_mullo_epi16,
    _mm_or_si128, _mm_packs_epi16, _mm_packs_epi32, _mm_set_epi8, _mm_set1_epi16,
    _mm_setzero_si128, _mm_shuffle_epi8, _mm_sign_epi16, _mm_srai_epi16, _mm_srai_epi32,
    _mm_sub_epi16, _mm_sub_epi32, _mm_unpackhi_epi16, _mm_unpackhi_epi64, _mm_unpacklo_epi16,
    _mm_unpacklo_epi32, _mm_unpacklo_epi64,
};

use crate::dsp::common_sse2::{loadu_si128, storeu_si128, vp8_transpose_2_4x4_16b};
use crate::dsp::dsp::{BPS, MAX_COEFF_THRESH, VP8Histogram};
use crate::dsp::enc::{VP8_DSP_SCAN, vp8_set_histogram_data, vp8f_transform};
use crate::dsp::enc_sse2::load_4_bytes;
use crate::enc::vp8i_enc::{MAX_LEVEL, QFIX, VP8Matrix};

const BPS_U: usize = BPS as usize;

//------------------------------------------------------------------------------
// Compute susceptibility based on DCT-coeff histograms.

// upstream: src/dsp/enc_sse41.c CollectHistogram_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "MAX_COEFF_THRESH is 31, the scan offsets and bins are non-negative"
)]
fn collect_histogram_sse41(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    let max_coeff_thresh = _mm_set1_epi16(MAX_COEFF_THRESH as i16);
    let mut distribution = [0_i32; MAX_COEFF_THRESH as usize + 1];
    for j in start_block..end_block {
        let mut out = [0_i16; 16];
        let scan = VP8_DSP_SCAN[j as usize] as usize;

        vp8f_transform(&ref_[scan..], &pred[scan..], &mut out);

        // Convert coefficients to bin (within out[]).
        {
            // Load.
            let out0 = loadu_si128(&out);
            let out1 = loadu_si128(&out[8..]);
            // v = abs(out) >> 3
            let abs0 = _mm_abs_epi16(out0);
            let abs1 = _mm_abs_epi16(out1);
            let v0 = _mm_srai_epi16::<3>(abs0);
            let v1 = _mm_srai_epi16::<3>(abs1);
            // bin = min(v, MAX_COEFF_THRESH)
            let bin0 = _mm_min_epi16(v0, max_coeff_thresh);
            let bin1 = _mm_min_epi16(v1, max_coeff_thresh);
            // Store.
            storeu_si128(&mut out, bin0);
            storeu_si128(&mut out[8..], bin1);
        }

        // Convert coefficients to bin.
        for &bin in &out {
            distribution[bin as usize] += 1;
        }
    }
    vp8_set_histogram_data(&distribution, histo);
}

//------------------------------------------------------------------------------
// Texture distortion
//
// We try to match the spectral content (weighted) between source and
// reconstructed samples.

// upstream: src/dsp/enc_sse41.c TTransform_SSE41
/// Hadamard transform
/// Returns the weighted sum of the absolute value of transformed coefficients.
/// w[] contains a row-major 4 by 4 symmetric matrix.
#[target_feature(enable = "sse4.1")]
#[expect(non_snake_case, reason = "the C names inA, inB, A_b0 and the others")]
fn ttransform_sse41(inA: &[u8], inB: &[u8], w: &[u16; 16]) -> i32 {
    let mut sum = [0_i32; 4];
    let mut tmp_0;
    let mut tmp_1;
    let mut tmp_2;
    let mut tmp_3;

    // Load and combine inputs.
    {
        let inA_0 = load_4_bytes(inA);
        let inA_1 = load_4_bytes(&inA[BPS_U..]);
        let inA_2 = load_4_bytes(&inA[2 * BPS_U..]);
        // In SSE4.1, with gcc 4.8 at least (maybe other versions),
        // _mm_loadu_si128 is faster than _mm_loadl_epi64. But for the last lump
        // of inA and inB, _mm_loadl_epi64 is still used not to have an out of
        // bound read.
        let inA_3 = load_4_bytes(&inA[3 * BPS_U..]);
        let inB_0 = load_4_bytes(inB);
        let inB_1 = load_4_bytes(&inB[BPS_U..]);
        let inB_2 = load_4_bytes(&inB[2 * BPS_U..]);
        let inB_3 = load_4_bytes(&inB[3 * BPS_U..]);

        // Combine inA and inB (we'll do two transforms in parallel).
        let inAB_0 = _mm_unpacklo_epi32(inA_0, inB_0);
        let inAB_1 = _mm_unpacklo_epi32(inA_1, inB_1);
        let inAB_2 = _mm_unpacklo_epi32(inA_2, inB_2);
        let inAB_3 = _mm_unpacklo_epi32(inA_3, inB_3);
        tmp_0 = _mm_cvtepu8_epi16(inAB_0);
        tmp_1 = _mm_cvtepu8_epi16(inAB_1);
        tmp_2 = _mm_cvtepu8_epi16(inAB_2);
        tmp_3 = _mm_cvtepu8_epi16(inAB_3);
        // a00 a01 a02 a03   b00 b01 b02 b03
        // a10 a11 a12 a13   b10 b11 b12 b13
        // a20 a21 a22 a23   b20 b21 b22 b23
        // a30 a31 a32 a33   b30 b31 b32 b33
    }

    // Vertical pass first to avoid a transpose (vertical and horizontal passes
    // are commutative because w/kWeightY is symmetric) and subsequent transpose.
    {
        // Calculate a and b (two 4x4 at once).
        let a0 = _mm_add_epi16(tmp_0, tmp_2);
        let a1 = _mm_add_epi16(tmp_1, tmp_3);
        let a2 = _mm_sub_epi16(tmp_1, tmp_3);
        let a3 = _mm_sub_epi16(tmp_0, tmp_2);
        let b0 = _mm_add_epi16(a0, a1);
        let b1 = _mm_add_epi16(a3, a2);
        let b2 = _mm_sub_epi16(a3, a2);
        let b3 = _mm_sub_epi16(a0, a1);
        // a00 a01 a02 a03   b00 b01 b02 b03
        // a10 a11 a12 a13   b10 b11 b12 b13
        // a20 a21 a22 a23   b20 b21 b22 b23
        // a30 a31 a32 a33   b30 b31 b32 b33

        // Transpose the two 4x4.
        vp8_transpose_2_4x4_16b(
            &b0, &b1, &b2, &b3, &mut tmp_0, &mut tmp_1, &mut tmp_2, &mut tmp_3,
        );
    }

    // Horizontal pass and difference of weighted sums.
    {
        // Load all inputs.
        let w_0 = loadu_si128(w);
        let w_8 = loadu_si128(&w[8..]);

        // Calculate a and b (two 4x4 at once).
        let a0 = _mm_add_epi16(tmp_0, tmp_2);
        let a1 = _mm_add_epi16(tmp_1, tmp_3);
        let a2 = _mm_sub_epi16(tmp_1, tmp_3);
        let a3 = _mm_sub_epi16(tmp_0, tmp_2);
        let b0 = _mm_add_epi16(a0, a1);
        let b1 = _mm_add_epi16(a3, a2);
        let b2 = _mm_sub_epi16(a3, a2);
        let b3 = _mm_sub_epi16(a0, a1);

        // Separate the transforms of inA and inB.
        let mut A_b0 = _mm_unpacklo_epi64(b0, b1);
        let mut A_b2 = _mm_unpacklo_epi64(b2, b3);
        let mut B_b0 = _mm_unpackhi_epi64(b0, b1);
        let mut B_b2 = _mm_unpackhi_epi64(b2, b3);

        A_b0 = _mm_abs_epi16(A_b0);
        A_b2 = _mm_abs_epi16(A_b2);
        B_b0 = _mm_abs_epi16(B_b0);
        B_b2 = _mm_abs_epi16(B_b2);

        // weighted sums
        A_b0 = _mm_madd_epi16(A_b0, w_0);
        A_b2 = _mm_madd_epi16(A_b2, w_8);
        B_b0 = _mm_madd_epi16(B_b0, w_0);
        B_b2 = _mm_madd_epi16(B_b2, w_8);
        A_b0 = _mm_add_epi32(A_b0, A_b2);
        B_b0 = _mm_add_epi32(B_b0, B_b2);

        // difference of weighted sums
        A_b2 = _mm_sub_epi32(A_b0, B_b0);
        storeu_si128(&mut sum, A_b2);
    }
    sum[0]
        .wrapping_add(sum[1])
        .wrapping_add(sum[2])
        .wrapping_add(sum[3])
}

// upstream: src/dsp/enc_sse41.c Disto4x4_SSE41
#[target_feature(enable = "sse4.1")]
fn disto4x4_sse41(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let diff_sum = ttransform_sse41(a, b, w);
    diff_sum.wrapping_abs() >> 5
}

// upstream: src/dsp/enc_sse41.c Disto16x16_SSE41
#[target_feature(enable = "sse4.1")]
#[expect(non_snake_case, reason = "the C name D")]
fn disto16x16_sse41(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let mut D = 0;
    for y in (0..16 * BPS_U).step_by(4 * BPS_U) {
        for x in (0..16).step_by(4) {
            D += disto4x4_sse41(&a[x + y..], &b[x + y..], w);
        }
    }
    D
}

//------------------------------------------------------------------------------
// Quantization
//

// upstream: src/dsp/enc_sse41.c PSHUFB_CST
/// Generates a pshufb constant for shuffling 16b words.
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(clippy::many_single_char_names, reason = "the C macro parameters")]
#[expect(clippy::too_many_arguments, reason = "the C macro parameters")]
fn pshufb_cst(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8) -> __m128i {
    _mm_set_epi8(
        2 * h + 1,
        2 * h,
        2 * g + 1,
        2 * g,
        2 * f + 1,
        2 * f,
        2 * e + 1,
        2 * e,
        2 * d + 1,
        2 * d,
        2 * c + 1,
        2 * c,
        2 * b + 1,
        2 * b,
        2 * a + 1,
        2 * a,
    )
}

// upstream: src/dsp/enc_sse41.c DoQuantizeBlock_SSE41
#[target_feature(enable = "sse4.1")]
#[inline]
#[expect(non_snake_case, reason = "the C names coeff_iQ0H and the others")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "MAX_LEVEL is 2047, which fits in a 16-bit lane"
)]
fn do_quantize_block_sse41(
    input: &mut [i16],
    out: &mut [i16],
    sharpen: Option<&[u16; 16]>,
    mtx: &VP8Matrix,
) -> i32 {
    let max_coeff_2047 = _mm_set1_epi16(MAX_LEVEL as i16);
    let zero = _mm_setzero_si128();
    let mut out0;
    let mut out8;
    let packed_out;

    // Load all inputs.
    let mut in0 = loadu_si128(input);
    let mut in8 = loadu_si128(&input[8..]);
    let iq0 = loadu_si128(&mtx.iq);
    let iq8 = loadu_si128(&mtx.iq[8..]);
    let q0 = loadu_si128(&mtx.q);
    let q8 = loadu_si128(&mtx.q[8..]);

    // coeff = abs(in)
    let mut coeff0 = _mm_abs_epi16(in0);
    let mut coeff8 = _mm_abs_epi16(in8);

    // coeff = abs(in) + sharpen
    if let Some(sharpen) = sharpen {
        let sharpen0 = loadu_si128(sharpen);
        let sharpen8 = loadu_si128(&sharpen[8..]);
        coeff0 = _mm_add_epi16(coeff0, sharpen0);
        coeff8 = _mm_add_epi16(coeff8, sharpen8);
    }

    // out = (coeff * iQ + B) >> QFIX
    {
        // doing calculations with 32b precision (QFIX=17)
        // out = (coeff * iQ)
        let coeff_iQ0H = _mm_mulhi_epu16(coeff0, iq0);
        let coeff_iQ0L = _mm_mullo_epi16(coeff0, iq0);
        let coeff_iQ8H = _mm_mulhi_epu16(coeff8, iq8);
        let coeff_iQ8L = _mm_mullo_epi16(coeff8, iq8);
        let mut out_00 = _mm_unpacklo_epi16(coeff_iQ0L, coeff_iQ0H);
        let mut out_04 = _mm_unpackhi_epi16(coeff_iQ0L, coeff_iQ0H);
        let mut out_08 = _mm_unpacklo_epi16(coeff_iQ8L, coeff_iQ8H);
        let mut out_12 = _mm_unpackhi_epi16(coeff_iQ8L, coeff_iQ8H);
        // out = (coeff * iQ + B)
        let bias_00 = loadu_si128(&mtx.bias);
        let bias_04 = loadu_si128(&mtx.bias[4..]);
        let bias_08 = loadu_si128(&mtx.bias[8..]);
        let bias_12 = loadu_si128(&mtx.bias[12..]);
        out_00 = _mm_add_epi32(out_00, bias_00);
        out_04 = _mm_add_epi32(out_04, bias_04);
        out_08 = _mm_add_epi32(out_08, bias_08);
        out_12 = _mm_add_epi32(out_12, bias_12);
        // out = QUANTDIV(coeff, iQ, B, QFIX)
        out_00 = _mm_srai_epi32::<QFIX>(out_00);
        out_04 = _mm_srai_epi32::<QFIX>(out_04);
        out_08 = _mm_srai_epi32::<QFIX>(out_08);
        out_12 = _mm_srai_epi32::<QFIX>(out_12);

        // pack result as 16b
        out0 = _mm_packs_epi32(out_00, out_04);
        out8 = _mm_packs_epi32(out_08, out_12);

        // if (coeff > 2047) coeff = 2047
        out0 = _mm_min_epi16(out0, max_coeff_2047);
        out8 = _mm_min_epi16(out8, max_coeff_2047);
    }

    // put sign back
    out0 = _mm_sign_epi16(out0, in0);
    out8 = _mm_sign_epi16(out8, in8);

    // in = out * Q
    in0 = _mm_mullo_epi16(out0, q0);
    in8 = _mm_mullo_epi16(out8, q8);

    storeu_si128(input, in0);
    storeu_si128(&mut input[8..], in8);

    // zigzag the output before storing it. The re-ordering is:
    //    0 1 2 3 4 5 6 7 | 8  9 10 11 12 13 14 15
    // -> 0 1 4[8]5 2 3 6 | 9 12 13 10 [7]11 14 15
    // There's only two misplaced entries ([8] and [7]) that are crossing the
    // reg's boundaries.
    // We use pshufb instead of pshuflo/pshufhi.
    {
        let kCst_lo = pshufb_cst(0, 1, 4, -1, 5, 2, 3, 6);
        let kCst_7 = pshufb_cst(-1, -1, -1, -1, 7, -1, -1, -1);
        let tmp_lo = _mm_shuffle_epi8(out0, kCst_lo);
        let tmp_7 = _mm_shuffle_epi8(out0, kCst_7); // extract #7
        let kCst_hi = pshufb_cst(1, 4, 5, 2, -1, 3, 6, 7);
        let kCst_8 = pshufb_cst(-1, -1, -1, 0, -1, -1, -1, -1);
        let tmp_hi = _mm_shuffle_epi8(out8, kCst_hi);
        let tmp_8 = _mm_shuffle_epi8(out8, kCst_8); // extract #8
        let out_z0 = _mm_or_si128(tmp_lo, tmp_8);
        let out_z8 = _mm_or_si128(tmp_hi, tmp_7);
        storeu_si128(out, out_z0);
        storeu_si128(&mut out[8..], out_z8);
        packed_out = _mm_packs_epi16(out_z0, out_z8);
    }

    // detect if all 'out' values are zeroes or not
    i32::from(_mm_movemask_epi8(_mm_cmpeq_epi8(packed_out, zero)) != 0xffff)
}

// upstream: src/dsp/enc_sse41.c QuantizeBlock_SSE41
#[target_feature(enable = "sse4.1")]
fn quantize_block_sse41(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    do_quantize_block_sse41(input, out, Some(&mtx.sharpen), mtx)
}

// upstream: src/dsp/enc_sse41.c QuantizeBlockWHT_SSE41
#[target_feature(enable = "sse4.1")]
fn quantize_block_wht_sse41(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    do_quantize_block_sse41(input, out, None, mtx)
}

// upstream: src/dsp/enc_sse41.c Quantize2Blocks_SSE41
#[target_feature(enable = "sse4.1")]
fn quantize2_blocks_sse41(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    let sharpen = &mtx.sharpen;
    let mut nz = do_quantize_block_sse41(input, out, Some(sharpen), mtx);
    nz |= do_quantize_block_sse41(&mut input[16..], &mut out[16..], Some(sharpen), mtx) << 1;
    nz
}

//------------------------------------------------------------------------------
// Entry points (ADR 0008). The dispatch functions of `src/dsp/enc.rs` call
// them only after `features().sse2` and `features().sse4_1`, the answers of
// `VP8GetCPUInfo(kSSE2)` and `VP8GetCPUInfo(kSSE4_1)`, selected the SSE4.1
// variant, so the CPU has SSE4.1.

/// Entry point of `CollectHistogram_SSE41` for `VP8CollectHistogram`.
pub(crate) fn collect_histogram(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1 variant.
    unsafe { collect_histogram_sse41(ref_, pred, start_block, end_block, histo) }
}

/// Entry point of `QuantizeBlock_SSE41` for `VP8EncQuantizeBlock`.
pub(crate) fn quantize_block(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1 variant.
    unsafe { quantize_block_sse41(input, out, mtx) }
}

/// Entry point of `Quantize2Blocks_SSE41` for `VP8EncQuantize2Blocks`.
pub(crate) fn quantize2_blocks(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1 variant.
    unsafe { quantize2_blocks_sse41(input, out, mtx) }
}

/// Entry point of `QuantizeBlockWHT_SSE41` for `VP8EncQuantizeBlockWHT`.
pub(crate) fn quantize_block_wht(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1 variant.
    unsafe { quantize_block_wht_sse41(input, out, mtx) }
}

/// Entry point of `Disto4x4_SSE41` for `VP8TDisto4x4`.
pub(crate) fn disto4x4(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1 variant.
    unsafe { disto4x4_sse41(a, b, w) }
}

/// Entry point of `Disto16x16_SSE41` for `VP8TDisto16x16`.
pub(crate) fn disto16x16(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    // SAFETY: called only after `features().sse4_1` selected the SSE4.1 variant.
    unsafe { disto16x16_sse41(a, b, w) }
}
