// libwebp v1.6.0, src/dsp/enc_sse2.c:
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
// SSE2 version of speed-critical encoding functions.
//
// Author: Christian Duvivier (cduvivier@google.com)

//! Translation of `src/dsp/enc_sse2.c` from libwebp v1.6.0.
//!
//! Work package: WP19 (SIMD x86 encoder).
//!
//! The SSE2 variants of the encoder DSP functions of `src/dsp/enc.c`, behind
//! the dispatch functions of `src/dsp/enc.rs` (ADR 0008). `VP8EncDspInit`
//! runs `VP8EncDspInitSSE2` when the CPU query answers `kSSE2`, so each
//! dispatch function selects the entry point here when `features().sse2`
//! holds, after the SSE4.1 variant of `enc_sse41.rs` where one exists.
//! `VP8EncDspInitSSE2` assigns `VP8CollectHistogram`, `VP8EncPredLuma16`
//! (`Intra16Preds_SSE2`), `VP8EncPredChroma8` (`IntraChromaPreds_SSE2`),
//! `VP8EncPredLuma4` (`Intra4Preds_SSE2`), `VP8EncQuantizeBlock`,
//! `VP8EncQuantize2Blocks`, `VP8EncQuantizeBlockWHT`, `VP8ITransform`,
//! `VP8FTransform`, `VP8FTransform2`, `VP8FTransformWHT`, `VP8SSE16x16`,
//! `VP8SSE16x8`, `VP8SSE8x8`, `VP8SSE4x4`, `VP8TDisto4x4`, `VP8TDisto16x16`
//! and `VP8Mean16x4`.
//!
//! # Deviations from upstream
//!
//! 1. **Unaligned loads and stores** (the class of `common_sse2.rs`,
//!    deviation 1): the aligned `_mm_load_si128` and `_mm_store_si128` of
//!    `Put16_SSE2`, `VE16_SSE2`, `HE16_SSE2`, `TM_SSE2`, `DC16_SSE2` and
//!    `DC16NoLeft_SSE2` are unaligned.
//! 2. **Pointers** are slices as in `src/dsp/enc.rs`: `dst` and the blocks
//!    start at the slice, `BPS` rows apart; `left` and `top` of the 16x16 and
//!    chroma predictions are `PredLeft` and `Option<&[u8]>` (`None` for C's
//!    `NULL`), `left[-1]` is the byte before `left`'s position; `top` of the
//!    4x4 predictions is the buffer with the position of `top[0]`.
//! 3. **Out-parameters** of the helpers (`FTransformPass1_SSE2`,
//!    `FTransformWHTRow_SSE2`, `SubtractAndAccumulate_SSE2`) are return
//!    values; `LOAD_8x16b`, `DST`, `AVG2` and `AVG3` are functions.
//! 4. **Four-byte accesses** go through `webp_mem_to_int32`,
//!    `webp_int32_to_mem` and `webp_uint32_to_mem` of `src/utils/utils.rs`,
//!    as upstream's do.
//! 5. **Loads of eight bytes of which four are used.** `FTransform_SSE2`,
//!    `SSE4x4_SSE2` and `TTransform_SSE2` load eight bytes per row with
//!    `_mm_loadl_epi64` and use only the low four (the following
//!    `_mm_unpacklo_epi16`/`_mm_unpacklo_epi32` and `_mm_unpacklo_epi8`
//!    discard the rest); upstream over-allocates its buffers for the extra
//!    bytes. The translation loads the four bytes it uses (`load_4_bytes`),
//!    because the encoder's slices may end four bytes after a block's last
//!    row. The results are the same.
//!
//! # Excluded undefined behavior
//!
//! None beyond the C functions'.
//!
//! # Checks not applicable
//!
//! 1. **`VP8EncDspInitSSE2`** only assigns pointers (ADR 0003); its
//!    assignments are the selection in the dispatch functions listed above.

#[cfg(target_arch = "x86")]
use core::arch::x86 as arch;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as arch;

use arch::{
    __m128i, _mm_add_epi16, _mm_add_epi32, _mm_adds_epi16, _mm_and_si128, _mm_avg_epu8,
    _mm_cmpeq_epi8, _mm_cmpeq_epi16, _mm_cmpgt_epi16, _mm_cvtsi32_si128, _mm_cvtsi128_si32,
    _mm_insert_epi16, _mm_madd_epi16, _mm_max_epi16, _mm_min_epi16, _mm_movemask_epi8,
    _mm_mulhi_epi16, _mm_mulhi_epu16, _mm_mullo_epi16, _mm_or_si128, _mm_packs_epi16,
    _mm_packs_epi32, _mm_packus_epi16, _mm_sad_epu8, _mm_set_epi16, _mm_set1_epi8, _mm_set1_epi16,
    _mm_set1_epi32, _mm_setzero_si128, _mm_shuffle_epi32, _mm_shufflehi_epi16, _mm_shufflelo_epi16,
    _mm_slli_si128, _mm_srai_epi16, _mm_srai_epi32, _mm_srli_epi16, _mm_srli_si128, _mm_sub_epi16,
    _mm_sub_epi32, _mm_subs_epi16, _mm_subs_epu8, _mm_unpackhi_epi8, _mm_unpackhi_epi16,
    _mm_unpackhi_epi32, _mm_unpackhi_epi64, _mm_unpacklo_epi8, _mm_unpacklo_epi16,
    _mm_unpacklo_epi32, _mm_unpacklo_epi64, _mm_xor_si128,
};

use crate::dsp::common_sse2::{
    loadl_epi64, loadu_si128, storel_epi64, storeu_si128, vp8_horizontal_add8b,
    vp8_transpose_2_4x4_16b,
};
use crate::dsp::dsp::{BPS, MAX_COEFF_THRESH, VP8Histogram};
use crate::dsp::enc::{PredLeft, VP8_DSP_SCAN, vp8_set_histogram_data};
use crate::dsp::lossless_sse2::mm_shuffle;
use crate::enc::vp8i_enc::{
    C8DC8, C8HE8, C8TM8, C8VE8, I4DC4, I4HD4, I4HE4, I4HU4, I4LD4, I4RD4, I4TM4, I4VE4, I4VL4,
    I4VR4, I16DC16, I16HE16, I16TM16, I16VE16, MAX_LEVEL, QFIX, VP8Matrix,
};
use crate::utils::utils::{webp_int32_to_mem, webp_mem_to_int32, webp_uint32_to_mem};

const BPS_U: usize = BPS as usize;

/// The position of a prediction block inside the prediction buffer.
#[expect(clippy::cast_sign_loss, reason = "the block offsets are non-negative")]
const fn at(offset: i32) -> usize {
    offset as usize
}

/// The low four bytes of `src` in the low lane, zero above: `_mm_loadl_epi64`
/// where upstream uses only the low four of the eight bytes it loads
/// (deviation 5).
#[target_feature(enable = "sse2")]
#[inline]
pub(crate) fn load_4_bytes(src: &[u8]) -> __m128i {
    _mm_cvtsi32_si128(webp_mem_to_int32(src))
}

//------------------------------------------------------------------------------
// Transforms (Paragraph 14.4)

// upstream: src/dsp/enc_sse2.c ITransform_One_SSE2
/// Does one inverse transform.
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C names T01 and T23")]
#[expect(clippy::similar_names, reason = "the C names")]
fn itransform_one_sse2(ref_: &[u8], input: &[i16], dst: &mut [u8]) {
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
    let k1k2 = _mm_set_epi16(-30068, -30068, -30068, -30068, 20091, 20091, 20091, 20091);
    let k2k1 = _mm_set_epi16(20091, 20091, 20091, 20091, -30068, -30068, -30068, -30068);
    let zero = _mm_setzero_si128();
    let zero_four = _mm_set_epi16(0, 0, 0, 0, 4, 4, 4, 4);
    let mut T01;
    let mut T23;

    // Load and concatenate the transform coefficients.
    let in01 = loadu_si128(input);
    let in23 = loadu_si128(&input[8..]);
    // a00 a10 a20 a30   a01 a11 a21 a31
    // a02 a12 a22 a32   a03 a13 a23 a33

    // Vertical pass and subsequent transpose.
    {
        let in1 = _mm_unpackhi_epi64(in01, in01);
        let in3 = _mm_unpackhi_epi64(in23, in23);

        // First pass, c and d calculations are longer because of the "trick"
        // multiplications.
        // c = MUL(in1, K2) - MUL(in3, K1) = MUL(in1, k2) - MUL(in3, k1) + in1 - in3
        // d = MUL(in1, K1) + MUL(in3, K2) = MUL(in1, k1) + MUL(in3, k2) + in1 + in3
        let a_d3 = _mm_add_epi16(in01, in23);
        let b_c3 = _mm_sub_epi16(in01, in23);
        let c1d1 = _mm_mulhi_epi16(in1, k2k1);
        let c2d2 = _mm_mulhi_epi16(in3, k1k2);
        let c3 = _mm_unpackhi_epi64(b_c3, b_c3);
        let c4 = _mm_sub_epi16(c1d1, c2d2);
        let c = _mm_add_epi16(c3, c4);
        let d4u = _mm_add_epi16(c1d1, c2d2);
        let du = _mm_add_epi16(a_d3, d4u);
        let d = _mm_unpackhi_epi64(du, du);

        // Second pass.
        let comb_ab = _mm_unpacklo_epi64(a_d3, b_c3);
        let comb_dc = _mm_unpacklo_epi64(d, c);

        let tmp01 = _mm_add_epi16(comb_ab, comb_dc);
        let tmp32 = _mm_sub_epi16(comb_ab, comb_dc);
        let tmp23 = _mm_shuffle_epi32::<{ mm_shuffle(1, 0, 3, 2) }>(tmp32);

        let transpose_0 = _mm_unpacklo_epi16(tmp01, tmp23);
        let transpose_1 = _mm_unpackhi_epi16(tmp01, tmp23);
        // a00 a20 a01 a21   a02 a22 a03 a23
        // a10 a30 a11 a31   a12 a32 a13 a33

        T01 = _mm_unpacklo_epi16(transpose_0, transpose_1);
        T23 = _mm_unpackhi_epi16(transpose_0, transpose_1);
        // a00 a10 a20 a30   a01 a11 a21 a31
        // a02 a12 a22 a32   a03 a13 a23 a33
    }

    // Horizontal pass and subsequent transpose.
    {
        let T1 = _mm_unpackhi_epi64(T01, T01);
        let T3 = _mm_unpackhi_epi64(T23, T23);

        // First pass, c and d calculations are longer because of the "trick"
        // multiplications.
        let dc = _mm_add_epi16(T01, zero_four);

        // c = MUL(T1, K2) - MUL(T3, K1) = MUL(T1, k2) - MUL(T3, k1) + T1 - T3
        // d = MUL(T1, K1) + MUL(T3, K2) = MUL(T1, k1) + MUL(T3, k2) + T1 + T3
        let a_d3 = _mm_add_epi16(dc, T23);
        let b_c3 = _mm_sub_epi16(dc, T23);
        let c1d1 = _mm_mulhi_epi16(T1, k2k1);
        let c2d2 = _mm_mulhi_epi16(T3, k1k2);
        let c3 = _mm_unpackhi_epi64(b_c3, b_c3);
        let c4 = _mm_sub_epi16(c1d1, c2d2);
        let c = _mm_add_epi16(c3, c4);
        let d4u = _mm_add_epi16(c1d1, c2d2);
        let du = _mm_add_epi16(a_d3, d4u);
        let d = _mm_unpackhi_epi64(du, du);

        // Second pass.
        let comb_ab = _mm_unpacklo_epi64(a_d3, b_c3);
        let comb_dc = _mm_unpacklo_epi64(d, c);

        let tmp01 = _mm_add_epi16(comb_ab, comb_dc);
        let tmp32 = _mm_sub_epi16(comb_ab, comb_dc);
        let tmp23 = _mm_shuffle_epi32::<{ mm_shuffle(1, 0, 3, 2) }>(tmp32);

        let shifted01 = _mm_srai_epi16::<3>(tmp01);
        let shifted23 = _mm_srai_epi16::<3>(tmp23);
        // a00 a01 a02 a03   a10 a11 a12 a13
        // a20 a21 a22 a23   a30 a31 a32 a33

        let transpose_0 = _mm_unpacklo_epi16(shifted01, shifted23);
        let transpose_1 = _mm_unpackhi_epi16(shifted01, shifted23);
        // a00 a20 a01 a21   a02 a22 a03 a23
        // a10 a30 a11 a31   a12 a32 a13 a33

        T01 = _mm_unpacklo_epi16(transpose_0, transpose_1);
        T23 = _mm_unpackhi_epi16(transpose_0, transpose_1);
        // a00 a10 a20 a30   a01 a11 a21 a31
        // a02 a12 a22 a32   a03 a13 a23 a33
    }

    // Add inverse transform to 'ref' and store.
    {
        // Load the reference(s).
        let mut buf = [0_i32; 4];

        // Load four bytes/pixels per line.
        let ref0 = _mm_cvtsi32_si128(webp_mem_to_int32(ref_));
        let ref1 = _mm_cvtsi32_si128(webp_mem_to_int32(&ref_[BPS_U..]));
        let ref2 = _mm_cvtsi32_si128(webp_mem_to_int32(&ref_[2 * BPS_U..]));
        let ref3 = _mm_cvtsi32_si128(webp_mem_to_int32(&ref_[3 * BPS_U..]));
        let mut ref01 = _mm_unpacklo_epi32(ref0, ref1);
        let mut ref23 = _mm_unpacklo_epi32(ref2, ref3);

        // Convert to 16b.
        ref01 = _mm_unpacklo_epi8(ref01, zero);
        ref23 = _mm_unpacklo_epi8(ref23, zero);
        // Add the inverse transform(s).
        ref01 = _mm_add_epi16(ref01, T01);
        ref23 = _mm_add_epi16(ref23, T23);
        // Unsigned saturate to 8b.
        let ref0123 = _mm_packus_epi16(ref01, ref23);

        storeu_si128(&mut buf, ref0123);

        // Store four bytes/pixels per line.
        webp_int32_to_mem(dst, buf[0]);
        webp_int32_to_mem(&mut dst[BPS_U..], buf[1]);
        webp_int32_to_mem(&mut dst[2 * BPS_U..], buf[2]);
        webp_int32_to_mem(&mut dst[3 * BPS_U..], buf[3]);
    }
}

// upstream: src/dsp/enc_sse2.c ITransform_Two_SSE2
/// Does two inverse transforms.
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C names T0 to T3")]
#[expect(clippy::similar_names, reason = "the C names")]
fn itransform_two_sse2(ref_: &[u8], input: &[i16], dst: &mut [u8]) {
    // See ITransform_One_SSE2 for the constants.
    let k1 = _mm_set1_epi16(20091);
    let k2 = _mm_set1_epi16(-30068);
    let mut T0 = _mm_setzero_si128();
    let mut T1 = _mm_setzero_si128();
    let mut T2 = _mm_setzero_si128();
    let mut T3 = _mm_setzero_si128();

    // Load and concatenate the transform coefficients (we'll do two inverse
    // transforms in parallel).
    let (in0, in1, in2, in3);
    {
        let tmp0 = loadu_si128(input);
        let tmp1 = loadu_si128(&input[8..]);
        let tmp2 = loadu_si128(&input[16..]);
        let tmp3 = loadu_si128(&input[24..]);
        in0 = _mm_unpacklo_epi64(tmp0, tmp2);
        in1 = _mm_unpackhi_epi64(tmp0, tmp2);
        in2 = _mm_unpacklo_epi64(tmp1, tmp3);
        in3 = _mm_unpackhi_epi64(tmp1, tmp3);
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
            &tmp0, &tmp1, &tmp2, &tmp3, &mut T0, &mut T1, &mut T2, &mut T3,
        );
    }

    // Horizontal pass and subsequent transpose.
    {
        // First pass, c and d calculations are longer because of the "trick"
        // multiplications.
        let four = _mm_set1_epi16(4);
        let dc = _mm_add_epi16(T0, four);
        let a = _mm_add_epi16(dc, T2);
        let b = _mm_sub_epi16(dc, T2);
        // c = MUL(T1, K2) - MUL(T3, K1) = MUL(T1, k2) - MUL(T3, k1) + T1 - T3
        let c1 = _mm_mulhi_epi16(T1, k2);
        let c2 = _mm_mulhi_epi16(T3, k1);
        let c3 = _mm_sub_epi16(T1, T3);
        let c4 = _mm_sub_epi16(c1, c2);
        let c = _mm_add_epi16(c3, c4);
        // d = MUL(T1, K1) + MUL(T3, K2) = MUL(T1, k1) + MUL(T3, k2) + T1 + T3
        let d1 = _mm_mulhi_epi16(T1, k1);
        let d2 = _mm_mulhi_epi16(T3, k2);
        let d3 = _mm_add_epi16(T1, T3);
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
            &shifted0, &shifted1, &shifted2, &shifted3, &mut T0, &mut T1, &mut T2, &mut T3,
        );
    }

    // Add inverse transform to 'ref' and store.
    {
        let zero = _mm_setzero_si128();
        // Load the reference(s).
        // Load eight bytes/pixels per line.
        let mut ref0 = loadl_epi64(ref_);
        let mut ref1 = loadl_epi64(&ref_[BPS_U..]);
        let mut ref2 = loadl_epi64(&ref_[2 * BPS_U..]);
        let mut ref3 = loadl_epi64(&ref_[3 * BPS_U..]);
        // Convert to 16b.
        ref0 = _mm_unpacklo_epi8(ref0, zero);
        ref1 = _mm_unpacklo_epi8(ref1, zero);
        ref2 = _mm_unpacklo_epi8(ref2, zero);
        ref3 = _mm_unpacklo_epi8(ref3, zero);
        // Add the inverse transform(s).
        ref0 = _mm_add_epi16(ref0, T0);
        ref1 = _mm_add_epi16(ref1, T1);
        ref2 = _mm_add_epi16(ref2, T2);
        ref3 = _mm_add_epi16(ref3, T3);
        // Unsigned saturate to 8b.
        ref0 = _mm_packus_epi16(ref0, ref0);
        ref1 = _mm_packus_epi16(ref1, ref1);
        ref2 = _mm_packus_epi16(ref2, ref2);
        ref3 = _mm_packus_epi16(ref3, ref3);
        // Store eight bytes/pixels per line.
        storel_epi64(dst, ref0);
        storel_epi64(&mut dst[BPS_U..], ref1);
        storel_epi64(&mut dst[2 * BPS_U..], ref2);
        storel_epi64(&mut dst[3 * BPS_U..], ref3);
    }
}

// upstream: src/dsp/enc_sse2.c ITransform_SSE2
/// Does one or two inverse transforms.
#[target_feature(enable = "sse2")]
fn itransform_sse2(ref_: &[u8], input: &[i16], dst: &mut [u8], do_two: bool) {
    if do_two {
        itransform_two_sse2(ref_, input, dst);
    } else {
        itransform_one_sse2(ref_, input, dst);
    }
}

// upstream: src/dsp/enc_sse2.c FTransformPass1_SSE2
/// Returns `(out01, out32)`.
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::similar_names, reason = "the C names")]
fn ftransform_pass1_sse2(in01: __m128i, in23: __m128i) -> (__m128i, __m128i) {
    let k937 = _mm_set1_epi32(937);
    let k1812 = _mm_set1_epi32(1812);

    let k88p = _mm_set_epi16(8, 8, 8, 8, 8, 8, 8, 8);
    let k88m = _mm_set_epi16(-8, 8, -8, 8, -8, 8, -8, 8);
    let k5352_2217p = _mm_set_epi16(2217, 5352, 2217, 5352, 2217, 5352, 2217, 5352);
    let k5352_2217m = _mm_set_epi16(-5352, 2217, -5352, 2217, -5352, 2217, -5352, 2217);

    // *in01 = 00 01 10 11 02 03 12 13
    // *in23 = 20 21 30 31 22 23 32 33
    let shuf01_p = _mm_shufflehi_epi16::<{ mm_shuffle(2, 3, 0, 1) }>(in01);
    let shuf23_p = _mm_shufflehi_epi16::<{ mm_shuffle(2, 3, 0, 1) }>(in23);
    // 00 01 10 11 03 02 13 12
    // 20 21 30 31 23 22 33 32
    let s01 = _mm_unpacklo_epi64(shuf01_p, shuf23_p);
    let s32 = _mm_unpackhi_epi64(shuf01_p, shuf23_p);
    // 00 01 10 11 20 21 30 31
    // 03 02 13 12 23 22 33 32
    let a01 = _mm_add_epi16(s01, s32);
    let a32 = _mm_sub_epi16(s01, s32);
    // [d0 + d3 | d1 + d2 | ...] = [a0 a1 | a0' a1' | ... ]
    // [d0 - d3 | d1 - d2 | ...] = [a3 a2 | a3' a2' | ... ]

    let tmp0 = _mm_madd_epi16(a01, k88p); // [ (a0 + a1) << 3, ... ]
    let tmp2 = _mm_madd_epi16(a01, k88m); // [ (a0 - a1) << 3, ... ]
    let tmp1_1 = _mm_madd_epi16(a32, k5352_2217p);
    let tmp3_1 = _mm_madd_epi16(a32, k5352_2217m);
    let tmp1_2 = _mm_add_epi32(tmp1_1, k1812);
    let tmp3_2 = _mm_add_epi32(tmp3_1, k937);
    let tmp1 = _mm_srai_epi32::<9>(tmp1_2);
    let tmp3 = _mm_srai_epi32::<9>(tmp3_2);
    let s03 = _mm_packs_epi32(tmp0, tmp2);
    let s12 = _mm_packs_epi32(tmp1, tmp3);
    let s_lo = _mm_unpacklo_epi16(s03, s12); // 0 1 0 1 0 1...
    let s_hi = _mm_unpackhi_epi16(s03, s12); // 2 3 2 3 2 3
    let v23 = _mm_unpackhi_epi32(s_lo, s_hi);
    let out01 = _mm_unpacklo_epi32(s_lo, s_hi);
    let out32 = _mm_shuffle_epi32::<{ mm_shuffle(1, 0, 3, 2) }>(v23); // 3 2 3 2 3 2..
    (out01, out32)
}

// upstream: src/dsp/enc_sse2.c FTransformPass2_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::similar_names, reason = "the C names")]
fn ftransform_pass2_sse2(v01: __m128i, v32: __m128i, out: &mut [i16]) {
    let zero = _mm_setzero_si128();
    let seven = _mm_set1_epi16(7);
    let k5352_2217 = _mm_set_epi16(5352, 2217, 5352, 2217, 5352, 2217, 5352, 2217);
    let k2217_5352 = _mm_set_epi16(2217, -5352, 2217, -5352, 2217, -5352, 2217, -5352);
    let k12000_plus_one = _mm_set1_epi32(12000 + (1 << 16));
    let k51000 = _mm_set1_epi32(51000);

    // Same operations are done on the (0,3) and (1,2) pairs.
    // a3 = v0 - v3
    // a2 = v1 - v2
    let a32 = _mm_sub_epi16(v01, v32);
    let a22 = _mm_unpackhi_epi64(a32, a32);

    let b23 = _mm_unpacklo_epi16(a22, a32);
    let c1 = _mm_madd_epi16(b23, k5352_2217);
    let c3 = _mm_madd_epi16(b23, k2217_5352);
    let d1 = _mm_add_epi32(c1, k12000_plus_one);
    let d3 = _mm_add_epi32(c3, k51000);
    let e1 = _mm_srai_epi32::<16>(d1);
    let e3 = _mm_srai_epi32::<16>(d3);
    // f1 = ((b3 * 5352 + b2 * 2217 + 12000) >> 16)
    // f3 = ((b3 * 2217 - b2 * 5352 + 51000) >> 16)
    let f1 = _mm_packs_epi32(e1, e1);
    let f3 = _mm_packs_epi32(e3, e3);
    // g1 = f1 + (a3 != 0);
    // The compare will return (0xffff, 0) for (==0, !=0). To turn that into the
    // desired (0, 1), we add one earlier through k12000_plus_one.
    // -> g1 = f1 + 1 - (a3 == 0)
    let g1 = _mm_add_epi16(f1, _mm_cmpeq_epi16(a32, zero));

    // a0 = v0 + v3
    // a1 = v1 + v2
    let a01 = _mm_add_epi16(v01, v32);
    let a01_plus_7 = _mm_add_epi16(a01, seven);
    let a11 = _mm_unpackhi_epi64(a01, a01);
    let c0 = _mm_add_epi16(a01_plus_7, a11);
    let c2 = _mm_sub_epi16(a01_plus_7, a11);
    // d0 = (a0 + a1 + 7) >> 4;
    // d2 = (a0 - a1 + 7) >> 4;
    let d0 = _mm_srai_epi16::<4>(c0);
    let d2 = _mm_srai_epi16::<4>(c2);

    let d0_g1 = _mm_unpacklo_epi64(d0, g1);
    let d2_f3 = _mm_unpacklo_epi64(d2, f3);
    storeu_si128(out, d0_g1);
    storeu_si128(&mut out[8..], d2_f3);
}

// upstream: src/dsp/enc_sse2.c FTransform_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C names")]
fn f_transform_sse2(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    let zero = _mm_setzero_si128();
    // Load src.
    let src0 = load_4_bytes(src);
    let src1 = load_4_bytes(&src[BPS_U..]);
    let src2 = load_4_bytes(&src[2 * BPS_U..]);
    let src3 = load_4_bytes(&src[3 * BPS_U..]);
    // 00 01 02 03 *
    // 10 11 12 13 *
    // 20 21 22 23 *
    // 30 31 32 33 *
    // Shuffle.
    let src_0 = _mm_unpacklo_epi16(src0, src1);
    let src_1 = _mm_unpacklo_epi16(src2, src3);
    // 00 01 10 11 02 03 12 13 * * ...
    // 20 21 30 31 22 22 32 33 * * ...

    // Load ref.
    let ref0 = load_4_bytes(ref_);
    let ref1 = load_4_bytes(&ref_[BPS_U..]);
    let ref2 = load_4_bytes(&ref_[2 * BPS_U..]);
    let ref3 = load_4_bytes(&ref_[3 * BPS_U..]);
    let ref_0 = _mm_unpacklo_epi16(ref0, ref1);
    let ref_1 = _mm_unpacklo_epi16(ref2, ref3);

    // Convert both to 16 bit.
    let src_0_16b = _mm_unpacklo_epi8(src_0, zero);
    let src_1_16b = _mm_unpacklo_epi8(src_1, zero);
    let ref_0_16b = _mm_unpacklo_epi8(ref_0, zero);
    let ref_1_16b = _mm_unpacklo_epi8(ref_1, zero);

    // Compute the difference.
    let row01 = _mm_sub_epi16(src_0_16b, ref_0_16b);
    let row23 = _mm_sub_epi16(src_1_16b, ref_1_16b);

    // First pass
    let (v01, v32) = ftransform_pass1_sse2(row01, row23);

    // Second pass
    ftransform_pass2_sse2(v01, v32, out);
}

// upstream: src/dsp/enc_sse2.c FTransform2_SSE2
#[target_feature(enable = "sse2")]
#[expect(clippy::similar_names, reason = "the C names")]
fn f_transform2_sse2(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    let zero = _mm_setzero_si128();

    // Load src and convert to 16b.
    let src0 = loadl_epi64(src);
    let src1 = loadl_epi64(&src[BPS_U..]);
    let src2 = loadl_epi64(&src[2 * BPS_U..]);
    let src3 = loadl_epi64(&src[3 * BPS_U..]);
    let src_0 = _mm_unpacklo_epi8(src0, zero);
    let src_1 = _mm_unpacklo_epi8(src1, zero);
    let src_2 = _mm_unpacklo_epi8(src2, zero);
    let src_3 = _mm_unpacklo_epi8(src3, zero);
    // Load ref and convert to 16b.
    let ref0 = loadl_epi64(ref_);
    let ref1 = loadl_epi64(&ref_[BPS_U..]);
    let ref2 = loadl_epi64(&ref_[2 * BPS_U..]);
    let ref3 = loadl_epi64(&ref_[3 * BPS_U..]);
    let ref_0 = _mm_unpacklo_epi8(ref0, zero);
    let ref_1 = _mm_unpacklo_epi8(ref1, zero);
    let ref_2 = _mm_unpacklo_epi8(ref2, zero);
    let ref_3 = _mm_unpacklo_epi8(ref3, zero);
    // Compute difference. -> 00 01 02 03  00' 01' 02' 03'
    let diff0 = _mm_sub_epi16(src_0, ref_0);
    let diff1 = _mm_sub_epi16(src_1, ref_1);
    let diff2 = _mm_sub_epi16(src_2, ref_2);
    let diff3 = _mm_sub_epi16(src_3, ref_3);

    // Unpack and shuffle
    // 00 01 02 03   0 0 0 0
    // 10 11 12 13   0 0 0 0
    // 20 21 22 23   0 0 0 0
    // 30 31 32 33   0 0 0 0
    let shuf01l = _mm_unpacklo_epi32(diff0, diff1);
    let shuf23l = _mm_unpacklo_epi32(diff2, diff3);
    let shuf01h = _mm_unpackhi_epi32(diff0, diff1);
    let shuf23h = _mm_unpackhi_epi32(diff2, diff3);

    // First pass
    let (v01l, v32l) = ftransform_pass1_sse2(shuf01l, shuf23l);
    let (v01h, v32h) = ftransform_pass1_sse2(shuf01h, shuf23h);

    // Second pass
    ftransform_pass2_sse2(v01l, v32l, out);
    ftransform_pass2_sse2(v01h, v32h, &mut out[16..]);
}

// upstream: src/dsp/enc_sse2.c FTransformWHTRow_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names A01 to D")]
fn ftransform_wht_row_sse2(input: &[i16]) -> __m128i {
    let kMult = _mm_set_epi16(-1, 1, -1, 1, 1, 1, 1, 1);
    let src0 = loadl_epi64(input);
    let src1 = loadl_epi64(&input[16..]);
    let src2 = loadl_epi64(&input[2 * 16..]);
    let src3 = loadl_epi64(&input[3 * 16..]);
    let A01 = _mm_unpacklo_epi16(src0, src1); // A0 A1 | ...
    let A23 = _mm_unpacklo_epi16(src2, src3); // A2 A3 | ...
    let B0 = _mm_adds_epi16(A01, A23); // a0 | a1 | ...
    let B1 = _mm_subs_epi16(A01, A23); // a3 | a2 | ...
    let C0 = _mm_unpacklo_epi32(B0, B1); // a0 | a1 | a3 | a2 | ...
    let C1 = _mm_unpacklo_epi32(B1, B0); // a3 | a2 | a0 | a1 | ...
    let D = _mm_unpacklo_epi64(C0, C1); // a0 a1 a3 a2 a3 a2 a0 a1
    _mm_madd_epi16(D, kMult)
}

// upstream: src/dsp/enc_sse2.c FTransformWHT_SSE2
#[target_feature(enable = "sse2")]
fn f_transform_wht_sse2(input: &[i16], out: &mut [i16]) {
    // Input is 12b signed.
    // Rows are 14b signed.
    let row0 = ftransform_wht_row_sse2(input);
    let row1 = ftransform_wht_row_sse2(&input[64..]);
    let row2 = ftransform_wht_row_sse2(&input[2 * 64..]);
    let row3 = ftransform_wht_row_sse2(&input[3 * 64..]);

    {
        // The a* are 15b signed.
        let a0 = _mm_add_epi32(row0, row2);
        let a1 = _mm_add_epi32(row1, row3);
        let a2 = _mm_sub_epi32(row1, row3);
        let a3 = _mm_sub_epi32(row0, row2);
        let a0a3 = _mm_packs_epi32(a0, a3);
        let a1a2 = _mm_packs_epi32(a1, a2);

        // The b* are 16b signed.
        let b0b1 = _mm_add_epi16(a0a3, a1a2);
        let b3b2 = _mm_sub_epi16(a0a3, a1a2);
        let tmp_b2b3 = _mm_unpackhi_epi64(b3b2, b3b2);
        let b2b3 = _mm_unpacklo_epi64(tmp_b2b3, b3b2);

        storeu_si128(out, _mm_srai_epi16::<1>(b0b1));
        storeu_si128(&mut out[8..], _mm_srai_epi16::<1>(b2b3));
    }
}

//------------------------------------------------------------------------------
// Compute susceptibility based on DCT-coeff histograms:
// the higher, the "easier" the macroblock is to compress.

// upstream: src/dsp/enc_sse2.c CollectHistogram_SSE2
#[target_feature(enable = "sse2")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "MAX_COEFF_THRESH is 31, the scan offsets and bins are non-negative"
)]
fn collect_histogram_sse2(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    let zero = _mm_setzero_si128();
    let max_coeff_thresh = _mm_set1_epi16(MAX_COEFF_THRESH as i16);
    let mut distribution = [0_i32; MAX_COEFF_THRESH as usize + 1];
    for j in start_block..end_block {
        let mut out = [0_i16; 16];
        let scan = VP8_DSP_SCAN[j as usize] as usize;

        f_transform_sse2(&ref_[scan..], &pred[scan..], &mut out);

        // Convert coefficients to bin (within out[]).
        {
            // Load.
            let out0 = loadu_si128(&out);
            let out1 = loadu_si128(&out[8..]);
            let d0 = _mm_sub_epi16(zero, out0);
            let d1 = _mm_sub_epi16(zero, out1);
            let abs0 = _mm_max_epi16(out0, d0); // abs(v), 16b
            let abs1 = _mm_max_epi16(out1, d1);
            // v = abs(out) >> 3
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
// Intra predictions

// upstream: src/dsp/enc_sse2.c Put8x8uv_SSE2
/// helper for chroma-DC predictions
#[target_feature(enable = "sse2")]
#[inline]
fn put8x8uv_sse2(v: u8, dst: &mut [u8]) {
    let values = _mm_set1_epi8(v.cast_signed());
    for j in 0..8 {
        storel_epi64(&mut dst[j * BPS_U..], values);
    }
}

// upstream: src/dsp/enc_sse2.c Put16_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn put16_sse2(v: u8, dst: &mut [u8]) {
    let values = _mm_set1_epi8(v.cast_signed());
    for j in 0..16 {
        storeu_si128(&mut dst[j * BPS_U..], values);
    }
}

// upstream: src/dsp/enc_sse2.c Fill_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C memsets and passes the low byte of the value"
)]
fn fill_sse2(dst: &mut [u8], value: i32, size: usize) {
    if size == 4 {
        for j in 0..4 {
            dst[j * BPS_U..j * BPS_U + 4].fill(value as u8);
        }
    } else if size == 8 {
        put8x8uv_sse2(value as u8, dst);
    } else {
        put16_sse2(value as u8, dst);
    }
}

// upstream: src/dsp/enc_sse2.c VE8uv_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn ve8uv_sse2(dst: &mut [u8], top: &[u8]) {
    let top_values = loadl_epi64(top);
    for j in 0..8 {
        storel_epi64(&mut dst[j * BPS_U..], top_values);
    }
}

// upstream: src/dsp/enc_sse2.c VE16_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn ve16_sse2(dst: &mut [u8], top: &[u8]) {
    let top_values = loadu_si128(top);
    for j in 0..16 {
        storeu_si128(&mut dst[j * BPS_U..], top_values);
    }
}

// upstream: src/dsp/enc_sse2.c VerticalPred_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn vertical_pred_sse2(dst: &mut [u8], top: Option<&[u8]>, size: usize) {
    if let Some(top) = top {
        if size == 8 {
            ve8uv_sse2(dst, top);
        } else {
            ve16_sse2(dst, top);
        }
    } else {
        fill_sse2(dst, 127, size);
    }
}

// upstream: src/dsp/enc_sse2.c HE8uv_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn he8uv_sse2(dst: &mut [u8], left: &[u8]) {
    for j in 0..8 {
        let values = _mm_set1_epi8(left[j].cast_signed());
        storel_epi64(&mut dst[j * BPS_U..], values);
    }
}

// upstream: src/dsp/enc_sse2.c HE16_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn he16_sse2(dst: &mut [u8], left: &[u8]) {
    for j in 0..16 {
        let values = _mm_set1_epi8(left[j].cast_signed());
        storeu_si128(&mut dst[j * BPS_U..], values);
    }
}

// upstream: src/dsp/enc_sse2.c HorizontalPred_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn horizontal_pred_sse2(dst: &mut [u8], left: PredLeft<'_>, size: usize) {
    if let Some((left, pos)) = left {
        if size == 8 {
            he8uv_sse2(dst, &left[pos..]);
        } else {
            he16_sse2(dst, &left[pos..]);
        }
    } else {
        fill_sse2(dst, 129, size);
    }
}

// upstream: src/dsp/enc_sse2.c TM_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C passes the int difference to _mm_set1_epi16 as a short"
)]
fn tm_sse2(dst: &mut [u8], left: &[u8], left_pos: usize, top: &[u8], size: usize) {
    let zero = _mm_setzero_si128();
    let corner = i32::from(left[left_pos - 1]);
    if size == 8 {
        let top_values = loadl_epi64(top);
        let top_base = _mm_unpacklo_epi8(top_values, zero);
        for y in 0..8 {
            let val = i32::from(left[left_pos + y]) - corner;
            let base = _mm_set1_epi16(val as i16);
            let out = _mm_packus_epi16(_mm_add_epi16(base, top_base), zero);
            storel_epi64(&mut dst[y * BPS_U..], out);
        }
    } else {
        let top_values = loadu_si128(top);
        let top_base_0 = _mm_unpacklo_epi8(top_values, zero);
        let top_base_1 = _mm_unpackhi_epi8(top_values, zero);
        for y in 0..16 {
            let val = i32::from(left[left_pos + y]) - corner;
            let base = _mm_set1_epi16(val as i16);
            let out_0 = _mm_add_epi16(base, top_base_0);
            let out_1 = _mm_add_epi16(base, top_base_1);
            let out = _mm_packus_epi16(out_0, out_1);
            storeu_si128(&mut dst[y * BPS_U..], out);
        }
    }
}

// upstream: src/dsp/enc_sse2.c TrueMotion_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn true_motion_sse2(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>, size: usize) {
    if let Some((left_buf, pos)) = left {
        if let Some(top) = top {
            tm_sse2(dst, left_buf, pos, top, size);
        } else {
            horizontal_pred_sse2(dst, left, size);
        }
    } else {
        // true motion without left samples (hence: with default 129 value)
        // is equivalent to VE prediction where you just copy the top samples.
        // Note that if top samples are not available, the default value is
        // then 129, and not 127 as in the VerticalPred case.
        if top.is_some() {
            vertical_pred_sse2(dst, top, size);
        } else {
            fill_sse2(dst, 129, size);
        }
    }
}

// upstream: src/dsp/enc_sse2.c DC8uv_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C passes the DC to a uint8_t parameter"
)]
#[expect(non_snake_case, reason = "the C name DC")]
fn dc8uv_sse2(dst: &mut [u8], left: &[u8], top: &[u8]) {
    let top_values = loadl_epi64(top);
    let left_values = loadl_epi64(left);
    let combined = _mm_unpacklo_epi64(top_values, left_values);
    let DC = vp8_horizontal_add8b(&combined) + 8;
    put8x8uv_sse2((DC >> 4) as u8, dst);
}

// upstream: src/dsp/enc_sse2.c DC8uvNoLeft_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C passes the DC to a uint8_t parameter"
)]
#[expect(non_snake_case, reason = "the C name DC")]
fn dc8uv_no_left_sse2(dst: &mut [u8], top: &[u8]) {
    let zero = _mm_setzero_si128();
    let top_values = loadl_epi64(top);
    let sum = _mm_sad_epu8(top_values, zero);
    let DC = _mm_cvtsi128_si32(sum) + 4;
    put8x8uv_sse2((DC >> 3) as u8, dst);
}

// upstream: src/dsp/enc_sse2.c DC8uvNoTop_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn dc8uv_no_top_sse2(dst: &mut [u8], left: &[u8]) {
    // 'left' is contiguous so we can reuse the top summation.
    dc8uv_no_left_sse2(dst, left);
}

// upstream: src/dsp/enc_sse2.c DC8uvNoTopLeft_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn dc8uv_no_top_left_sse2(dst: &mut [u8]) {
    put8x8uv_sse2(0x80, dst);
}

// upstream: src/dsp/enc_sse2.c DC8uvMode_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn dc8uv_mode_sse2(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    if let Some(top) = top {
        if let Some((left, pos)) = left {
            // top and left present
            dc8uv_sse2(dst, &left[pos..], top);
        } else {
            // top, but no left
            dc8uv_no_left_sse2(dst, top);
        }
    } else if let Some((left, pos)) = left {
        // left but no top
        dc8uv_no_top_sse2(dst, &left[pos..]);
    } else {
        // no top, no left, nothing.
        dc8uv_no_top_left_sse2(dst);
    }
}

// upstream: src/dsp/enc_sse2.c DC16_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C passes the DC to a uint8_t parameter"
)]
#[expect(non_snake_case, reason = "the C name DC")]
fn dc16_sse2(dst: &mut [u8], left: &[u8], top: &[u8]) {
    let top_row = loadu_si128(top);
    let left_row = loadu_si128(left);
    let DC = vp8_horizontal_add8b(&top_row) + vp8_horizontal_add8b(&left_row) + 16;
    put16_sse2((DC >> 5) as u8, dst);
}

// upstream: src/dsp/enc_sse2.c DC16NoLeft_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C passes the DC to a uint8_t parameter"
)]
#[expect(non_snake_case, reason = "the C name DC")]
fn dc16_no_left_sse2(dst: &mut [u8], top: &[u8]) {
    let top_row = loadu_si128(top);
    let DC = vp8_horizontal_add8b(&top_row) + 8;
    put16_sse2((DC >> 4) as u8, dst);
}

// upstream: src/dsp/enc_sse2.c DC16NoTop_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn dc16_no_top_sse2(dst: &mut [u8], left: &[u8]) {
    // 'left' is contiguous so we can reuse the top summation.
    dc16_no_left_sse2(dst, left);
}

// upstream: src/dsp/enc_sse2.c DC16NoTopLeft_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn dc16_no_top_left_sse2(dst: &mut [u8]) {
    put16_sse2(0x80, dst);
}

// upstream: src/dsp/enc_sse2.c DC16Mode_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn dc16_mode_sse2(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    if let Some(top) = top {
        if let Some((left, pos)) = left {
            // top and left present
            dc16_sse2(dst, &left[pos..], top);
        } else {
            // top, but no left
            dc16_no_left_sse2(dst, top);
        }
    } else if let Some((left, pos)) = left {
        // left but no top
        dc16_no_top_sse2(dst, &left[pos..]);
    } else {
        // no top, no left, nothing.
        dc16_no_top_left_sse2(dst);
    }
}

//------------------------------------------------------------------------------
// 4x4 predictions

// upstream: src/dsp/enc_sse2.c DST
fn dst_set(dst: &mut [u8], x: usize, y: usize, v: u8) {
    dst[x + y * BPS_U] = v;
}

// upstream: src/dsp/enc_sse2.c AVG3
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the average in a uint8_t"
)]
fn avg3(a: i32, b: i32, c: i32) -> u8 {
    ((a + 2 * b + c + 2) >> 2) as u8
}

// upstream: src/dsp/enc_sse2.c AVG2
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the average in a uint8_t"
)]
fn avg2(a: i32, b: i32) -> u8 {
    ((a + b + 1) >> 1) as u8
}

// We use the following 8b-arithmetic tricks:
//     (a + 2 * b + c + 2) >> 2 = (AC + b + 1) >> 1
//   where: AC = (a + c) >> 1 = [(a + c + 1) >> 1] - [(a^c) & 1]
// and:
//     (a + 2 * b + c + 2) >> 2 = (AB + BC + 1) >> 1 - (ab|bc)&lsb
//   where: AC = (a + b + 1) >> 1,   BC = (b + c + 1) >> 1
//   and ab = a ^ b, bc = b ^ c, lsb = (AC^BC)&1

// upstream: src/dsp/enc_sse2.c VE4_SSE2
/// vertical
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C register names")]
fn ve4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let one = _mm_set1_epi8(1);
    let ABCDEFGH = loadl_epi64(&top[p - 1..]);
    let BCDEFGH0 = _mm_srli_si128::<1>(ABCDEFGH);
    let CDEFGH00 = _mm_srli_si128::<2>(ABCDEFGH);
    let a = _mm_avg_epu8(ABCDEFGH, CDEFGH00);
    let lsb = _mm_and_si128(_mm_xor_si128(ABCDEFGH, CDEFGH00), one);
    let b = _mm_subs_epu8(a, lsb);
    let avg = _mm_avg_epu8(b, BCDEFGH0);
    let vals = _mm_cvtsi128_si32(avg);
    for i in 0..4 {
        webp_int32_to_mem(&mut dst[i * BPS_U..], vals);
    }
}

// upstream: src/dsp/enc_sse2.c HE4_SSE2
/// horizontal
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names X, I, J, K and L")]
fn he4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let X = i32::from(top[p - 1]);
    let I = i32::from(top[p - 2]);
    let J = i32::from(top[p - 3]);
    let K = i32::from(top[p - 4]);
    let L = i32::from(top[p - 5]);
    webp_uint32_to_mem(dst, 0x0101_0101_u32 * u32::from(avg3(X, I, J)));
    webp_uint32_to_mem(
        &mut dst[BPS_U..],
        0x0101_0101_u32 * u32::from(avg3(I, J, K)),
    );
    webp_uint32_to_mem(
        &mut dst[2 * BPS_U..],
        0x0101_0101_u32 * u32::from(avg3(J, K, L)),
    );
    webp_uint32_to_mem(
        &mut dst[3 * BPS_U..],
        0x0101_0101_u32 * u32::from(avg3(K, L, L)),
    );
}

// upstream: src/dsp/enc_sse2.c DC4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(clippy::cast_possible_wrap, reason = "the DC is at most 1020 >> 3")]
fn dc4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let mut dc: u32 = 4;
    for i in 0..4 {
        dc += u32::from(top[p + i]) + u32::from(top[p - 5 + i]);
    }
    fill_sse2(dst, (dc >> 3) as i32, 4);
}

// upstream: src/dsp/enc_sse2.c LD4_SSE2
/// Down-Left
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C register names")]
fn ld4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let one = _mm_set1_epi8(1);
    let ABCDEFGH = loadl_epi64(&top[p..]);
    let BCDEFGH0 = _mm_srli_si128::<1>(ABCDEFGH);
    let CDEFGH00 = _mm_srli_si128::<2>(ABCDEFGH);
    let CDEFGHH0 = _mm_insert_epi16::<3>(CDEFGH00, i32::from(top[p + 7]));
    let avg1 = _mm_avg_epu8(ABCDEFGH, CDEFGHH0);
    let lsb = _mm_and_si128(_mm_xor_si128(ABCDEFGH, CDEFGHH0), one);
    let avg2 = _mm_subs_epu8(avg1, lsb);
    let abcdefg = _mm_avg_epu8(avg2, BCDEFGH0);
    webp_int32_to_mem(dst, _mm_cvtsi128_si32(abcdefg));
    webp_int32_to_mem(
        &mut dst[BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut dst[2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<2>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut dst[3 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<3>(abcdefg)),
    );
}

// upstream: src/dsp/enc_sse2.c VR4_SSE2
/// Vertical-Right
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C register names")]
fn vr4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let one = _mm_set1_epi8(1);
    let I = i32::from(top[p - 2]);
    let J = i32::from(top[p - 3]);
    let K = i32::from(top[p - 4]);
    let X = i32::from(top[p - 1]);
    let XABCD = loadl_epi64(&top[p - 1..]);
    let ABCD0 = _mm_srli_si128::<1>(XABCD);
    let abcd = _mm_avg_epu8(XABCD, ABCD0);
    let XABCD_shifted = _mm_slli_si128::<1>(XABCD); // C: _XABCD
    // C: (short)(I | (X << 8)); the intrinsic takes the low 16 bits.
    let IXABCD = _mm_insert_epi16::<0>(XABCD_shifted, I | (X << 8));
    let avg1 = _mm_avg_epu8(IXABCD, ABCD0);
    let lsb = _mm_and_si128(_mm_xor_si128(IXABCD, ABCD0), one);
    let avg2 = _mm_subs_epu8(avg1, lsb);
    let efgh = _mm_avg_epu8(avg2, XABCD);
    webp_int32_to_mem(dst, _mm_cvtsi128_si32(abcd));
    webp_int32_to_mem(&mut dst[BPS_U..], _mm_cvtsi128_si32(efgh));
    webp_int32_to_mem(
        &mut dst[2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_slli_si128::<1>(abcd)),
    );
    webp_int32_to_mem(
        &mut dst[3 * BPS_U..],
        _mm_cvtsi128_si32(_mm_slli_si128::<1>(efgh)),
    );

    // these two are hard to implement in SSE2, so we keep the C-version:
    dst_set(dst, 0, 2, avg3(J, I, X));
    dst_set(dst, 0, 3, avg3(K, J, I));
}

// upstream: src/dsp/enc_sse2.c VL4_SSE2
/// Vertical-Left
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C register names")]
fn vl4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let one = _mm_set1_epi8(1);
    let ABCDEFGH = loadl_epi64(&top[p..]);
    let BCDEFGH_ = _mm_srli_si128::<1>(ABCDEFGH);
    let CDEFGH__ = _mm_srli_si128::<2>(ABCDEFGH);
    let avg1 = _mm_avg_epu8(ABCDEFGH, BCDEFGH_);
    let avg2 = _mm_avg_epu8(CDEFGH__, BCDEFGH_);
    let avg3_ = _mm_avg_epu8(avg1, avg2);
    let lsb1 = _mm_and_si128(_mm_xor_si128(avg1, avg2), one);
    let ab = _mm_xor_si128(ABCDEFGH, BCDEFGH_);
    let bc = _mm_xor_si128(CDEFGH__, BCDEFGH_);
    let abbc = _mm_or_si128(ab, bc);
    let lsb2 = _mm_and_si128(abbc, lsb1);
    let avg4 = _mm_subs_epu8(avg3_, lsb2);
    let extra_out = _mm_cvtsi128_si32(_mm_srli_si128::<4>(avg4)).cast_unsigned();
    let [extra_0, extra_1, _, _] = extra_out.to_le_bytes();
    webp_int32_to_mem(dst, _mm_cvtsi128_si32(avg1));
    webp_int32_to_mem(&mut dst[BPS_U..], _mm_cvtsi128_si32(avg4));
    webp_int32_to_mem(
        &mut dst[2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(avg1)),
    );
    webp_int32_to_mem(
        &mut dst[3 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(avg4)),
    );

    // these two are hard to get and irregular
    dst_set(dst, 3, 2, extra_0); // (extra_out >> 0) & 0xff
    dst_set(dst, 3, 3, extra_1); // (extra_out >> 8) & 0xff
}

// upstream: src/dsp/enc_sse2.c RD4_SSE2
/// Down-right
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C register names")]
fn rd4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let one = _mm_set1_epi8(1);
    let LKJIXABC = loadl_epi64(&top[p - 5..]);
    let LKJIXABCD = _mm_insert_epi16::<4>(LKJIXABC, i32::from(top[p + 3]));
    let KJIXABCD_ = _mm_srli_si128::<1>(LKJIXABCD);
    let JIXABCD__ = _mm_srli_si128::<2>(LKJIXABCD);
    let avg1 = _mm_avg_epu8(JIXABCD__, LKJIXABCD);
    let lsb = _mm_and_si128(_mm_xor_si128(JIXABCD__, LKJIXABCD), one);
    let avg2 = _mm_subs_epu8(avg1, lsb);
    let abcdefg = _mm_avg_epu8(avg2, KJIXABCD_);
    webp_int32_to_mem(&mut dst[3 * BPS_U..], _mm_cvtsi128_si32(abcdefg));
    webp_int32_to_mem(
        &mut dst[2 * BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<1>(abcdefg)),
    );
    webp_int32_to_mem(
        &mut dst[BPS_U..],
        _mm_cvtsi128_si32(_mm_srli_si128::<2>(abcdefg)),
    );
    webp_int32_to_mem(dst, _mm_cvtsi128_si32(_mm_srli_si128::<3>(abcdefg)));
}

// upstream: src/dsp/enc_sse2.c HU4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names I, J, K and L")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "L is a byte"
)]
fn hu4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let I = i32::from(top[p - 2]);
    let J = i32::from(top[p - 3]);
    let K = i32::from(top[p - 4]);
    let L = i32::from(top[p - 5]);
    dst_set(dst, 0, 0, avg2(I, J));
    let v = avg2(J, K);
    dst_set(dst, 0, 1, v);
    dst_set(dst, 2, 0, v);
    let v = avg2(K, L);
    dst_set(dst, 0, 2, v);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 1, 0, avg3(I, J, K));
    let v = avg3(J, K, L);
    dst_set(dst, 1, 1, v);
    dst_set(dst, 3, 0, v);
    let v = avg3(K, L, L);
    dst_set(dst, 1, 2, v);
    dst_set(dst, 3, 1, v);
    let l = L as u8;
    dst_set(dst, 3, 3, l);
    dst_set(dst, 2, 3, l);
    dst_set(dst, 1, 3, l);
    dst_set(dst, 0, 3, l);
    dst_set(dst, 2, 2, l);
    dst_set(dst, 3, 2, l);
}

// upstream: src/dsp/enc_sse2.c HD4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names X, I, J, K, L, A, B and C")]
fn hd4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let X = i32::from(top[p - 1]);
    let I = i32::from(top[p - 2]);
    let J = i32::from(top[p - 3]);
    let K = i32::from(top[p - 4]);
    let L = i32::from(top[p - 5]);
    let A = i32::from(top[p]);
    let B = i32::from(top[p + 1]);
    let C = i32::from(top[p + 2]);

    let v = avg2(I, X);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 0, 0, v);
    let v = avg2(J, I);
    dst_set(dst, 2, 2, v);
    dst_set(dst, 0, 1, v);
    let v = avg2(K, J);
    dst_set(dst, 2, 3, v);
    dst_set(dst, 0, 2, v);
    dst_set(dst, 0, 3, avg2(L, K));

    dst_set(dst, 3, 0, avg3(A, B, C));
    dst_set(dst, 2, 0, avg3(X, A, B));
    let v = avg3(I, X, A);
    dst_set(dst, 3, 1, v);
    dst_set(dst, 1, 0, v);
    let v = avg3(J, I, X);
    dst_set(dst, 3, 2, v);
    dst_set(dst, 1, 1, v);
    let v = avg3(K, J, I);
    dst_set(dst, 3, 3, v);
    dst_set(dst, 1, 2, v);
    dst_set(dst, 1, 3, avg3(L, K, J));
}

// upstream: src/dsp/enc_sse2.c TM4_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C passes the int difference to _mm_set1_epi16 as a short"
)]
fn tm4_sse2(dst: &mut [u8], top: &[u8], p: usize) {
    let zero = _mm_setzero_si128();
    let top_values = _mm_cvtsi32_si128(webp_mem_to_int32(&top[p..]));
    let top_base = _mm_unpacklo_epi8(top_values, zero);
    for y in 0..4 {
        let val = i32::from(top[p - 2 - y]) - i32::from(top[p - 1]);
        let base = _mm_set1_epi16(val as i16);
        let out = _mm_packus_epi16(_mm_add_epi16(base, top_base), zero);
        webp_int32_to_mem(&mut dst[y * BPS_U..], _mm_cvtsi128_si32(out));
    }
}

//------------------------------------------------------------------------------
// luma 4x4 prediction

// upstream: src/dsp/enc_sse2.c Intra4Preds_SSE2
/// Left samples are `top[-5 .. -2]`, `top_left` is `top[-1]`, top are
/// located at `top[0..3]`, and top right is `top[4..7]`
#[target_feature(enable = "sse2")]
fn intra4_preds_sse2(dst: &mut [u8], top: &[u8], top_pos: usize) {
    dc4_sse2(&mut dst[at(I4DC4)..], top, top_pos);
    tm4_sse2(&mut dst[at(I4TM4)..], top, top_pos);
    ve4_sse2(&mut dst[at(I4VE4)..], top, top_pos);
    he4_sse2(&mut dst[at(I4HE4)..], top, top_pos);
    rd4_sse2(&mut dst[at(I4RD4)..], top, top_pos);
    vr4_sse2(&mut dst[at(I4VR4)..], top, top_pos);
    ld4_sse2(&mut dst[at(I4LD4)..], top, top_pos);
    vl4_sse2(&mut dst[at(I4VL4)..], top, top_pos);
    hd4_sse2(&mut dst[at(I4HD4)..], top, top_pos);
    hu4_sse2(&mut dst[at(I4HU4)..], top, top_pos);
}

//------------------------------------------------------------------------------
// Chroma 8x8 prediction (paragraph 12.2)

// upstream: src/dsp/enc_sse2.c IntraChromaPreds_SSE2
#[target_feature(enable = "sse2")]
fn intra_chroma_preds_sse2(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    // U block
    dc8uv_mode_sse2(&mut dst[at(C8DC8)..], left, top);
    vertical_pred_sse2(&mut dst[at(C8VE8)..], top, 8);
    horizontal_pred_sse2(&mut dst[at(C8HE8)..], left, 8);
    true_motion_sse2(&mut dst[at(C8TM8)..], left, top, 8);
    // V block
    let dst = &mut dst[8..];
    let top = top.map(|top| &top[8..]);
    let left = left.map(|(left, pos)| (left, pos + 16));
    dc8uv_mode_sse2(&mut dst[at(C8DC8)..], left, top);
    vertical_pred_sse2(&mut dst[at(C8VE8)..], top, 8);
    horizontal_pred_sse2(&mut dst[at(C8HE8)..], left, 8);
    true_motion_sse2(&mut dst[at(C8TM8)..], left, top, 8);
}

//------------------------------------------------------------------------------
// luma 16x16 prediction (paragraph 12.3)

// upstream: src/dsp/enc_sse2.c Intra16Preds_SSE2
#[target_feature(enable = "sse2")]
fn intra16_preds_sse2(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    dc16_mode_sse2(&mut dst[at(I16DC16)..], left, top);
    vertical_pred_sse2(&mut dst[at(I16VE16)..], top, 16);
    horizontal_pred_sse2(&mut dst[at(I16HE16)..], left, 16);
    true_motion_sse2(&mut dst[at(I16TM16)..], left, top, 16);
}

//------------------------------------------------------------------------------
// Metric

// upstream: src/dsp/enc_sse2.c SubtractAndAccumulate_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names C0 and C1")]
fn subtract_and_accumulate_sse2(a: __m128i, b: __m128i) -> __m128i {
    // take abs(a-b) in 8b
    let a_b = _mm_subs_epu8(a, b);
    let b_a = _mm_subs_epu8(b, a);
    let abs_a_b = _mm_or_si128(a_b, b_a);
    // zero-extend to 16b
    let zero = _mm_setzero_si128();
    let C0 = _mm_unpacklo_epi8(abs_a_b, zero);
    let C1 = _mm_unpackhi_epi8(abs_a_b, zero);
    // multiply with self
    let sum1 = _mm_madd_epi16(C0, C0);
    let sum2 = _mm_madd_epi16(C1, C1);
    _mm_add_epi32(sum1, sum2)
}

/// `tmp[3] + tmp[2] + tmp[1] + tmp[0]` over the lanes of `sum`, as C stores
/// them to `int32_t tmp[4]` and adds them.
#[target_feature(enable = "sse2")]
#[inline]
fn sum_lanes(sum: __m128i) -> i32 {
    let mut tmp = [0_i32; 4];
    storeu_si128(&mut tmp, sum);
    tmp[3]
        .wrapping_add(tmp[2])
        .wrapping_add(tmp[1])
        .wrapping_add(tmp[0])
}

// upstream: src/dsp/enc_sse2.c SSE_16xN_SSE2
#[target_feature(enable = "sse2")]
#[inline]
fn sse_16xn_sse2(a: &[u8], b: &[u8], num_pairs: usize) -> i32 {
    let mut sum = _mm_setzero_si128();
    let mut a_pos = 0;
    let mut b_pos = 0;

    for _ in 0..num_pairs {
        let a0 = loadu_si128(&a[a_pos..]);
        let b0 = loadu_si128(&b[b_pos..]);
        let a1 = loadu_si128(&a[a_pos + BPS_U..]);
        let b1 = loadu_si128(&b[b_pos + BPS_U..]);
        let sum1 = subtract_and_accumulate_sse2(a0, b0);
        let sum2 = subtract_and_accumulate_sse2(a1, b1);
        sum = _mm_add_epi32(sum, _mm_add_epi32(sum1, sum2));
        a_pos += 2 * BPS_U;
        b_pos += 2 * BPS_U;
    }
    sum_lanes(sum)
}

// upstream: src/dsp/enc_sse2.c SSE16x16_SSE2
#[target_feature(enable = "sse2")]
fn sse16x16_sse2(a: &[u8], b: &[u8]) -> i32 {
    sse_16xn_sse2(a, b, 8)
}

// upstream: src/dsp/enc_sse2.c SSE16x8_SSE2
#[target_feature(enable = "sse2")]
fn sse16x8_sse2(a: &[u8], b: &[u8]) -> i32 {
    sse_16xn_sse2(a, b, 4)
}

// upstream: src/dsp/enc_sse2.c LOAD_8x16b
#[target_feature(enable = "sse2")]
#[inline]
fn load_8x16b(ptr: &[u8], zero: __m128i) -> __m128i {
    _mm_unpacklo_epi8(loadl_epi64(ptr), zero)
}

// upstream: src/dsp/enc_sse2.c SSE8x8_SSE2
#[target_feature(enable = "sse2")]
fn sse8x8_sse2(a: &[u8], b: &[u8]) -> i32 {
    let zero = _mm_setzero_si128();
    let mut sum = zero;
    let mut a_pos = 0;
    let mut b_pos = 0;
    for _ in 0..4 {
        let a0 = load_8x16b(&a[a_pos..], zero);
        let a1 = load_8x16b(&a[a_pos + BPS_U..], zero);
        let b0 = load_8x16b(&b[b_pos..], zero);
        let b1 = load_8x16b(&b[b_pos + BPS_U..], zero);
        // subtract
        let c0 = _mm_subs_epi16(a0, b0);
        let c1 = _mm_subs_epi16(a1, b1);
        // multiply/accumulate with self
        let d0 = _mm_madd_epi16(c0, c0);
        let d1 = _mm_madd_epi16(c1, c1);
        // collect
        let sum01 = _mm_add_epi32(d0, d1);
        sum = _mm_add_epi32(sum, sum01);
        a_pos += 2 * BPS_U;
        b_pos += 2 * BPS_U;
    }
    sum_lanes(sum)
}

// upstream: src/dsp/enc_sse2.c SSE4x4_SSE2
#[target_feature(enable = "sse2")]
fn sse4x4_sse2(a: &[u8], b: &[u8]) -> i32 {
    let zero = _mm_setzero_si128();

    // Load values. Note that we read 8 pixels instead of 4,
    // but the a/b buffers are over-allocated to that effect.
    let a0 = load_4_bytes(a);
    let a1 = load_4_bytes(&a[BPS_U..]);
    let a2 = load_4_bytes(&a[2 * BPS_U..]);
    let a3 = load_4_bytes(&a[3 * BPS_U..]);
    let b0 = load_4_bytes(b);
    let b1 = load_4_bytes(&b[BPS_U..]);
    let b2 = load_4_bytes(&b[2 * BPS_U..]);
    let b3 = load_4_bytes(&b[3 * BPS_U..]);
    // Combine pair of lines.
    let a01 = _mm_unpacklo_epi32(a0, a1);
    let a23 = _mm_unpacklo_epi32(a2, a3);
    let b01 = _mm_unpacklo_epi32(b0, b1);
    let b23 = _mm_unpacklo_epi32(b2, b3);
    // Convert to 16b.
    let a01s = _mm_unpacklo_epi8(a01, zero);
    let a23s = _mm_unpacklo_epi8(a23, zero);
    let b01s = _mm_unpacklo_epi8(b01, zero);
    let b23s = _mm_unpacklo_epi8(b23, zero);
    // subtract, square and accumulate
    let d0 = _mm_subs_epi16(a01s, b01s);
    let d1 = _mm_subs_epi16(a23s, b23s);
    let e0 = _mm_madd_epi16(d0, d0);
    let e1 = _mm_madd_epi16(d1, d1);
    let sum = _mm_add_epi32(e0, e1);

    sum_lanes(sum)
}

//------------------------------------------------------------------------------

// upstream: src/dsp/enc_sse2.c Mean16x4_SSE2
#[target_feature(enable = "sse2")]
fn mean16x4_sse2(ref_: &[u8], dc: &mut [u32]) {
    let mask = _mm_set1_epi16(0x00ff);
    let a0 = loadu_si128(ref_);
    let a1 = loadu_si128(&ref_[BPS_U..]);
    let a2 = loadu_si128(&ref_[2 * BPS_U..]);
    let a3 = loadu_si128(&ref_[3 * BPS_U..]);
    let b0 = _mm_srli_epi16::<8>(a0); // hi byte
    let b1 = _mm_srli_epi16::<8>(a1);
    let b2 = _mm_srli_epi16::<8>(a2);
    let b3 = _mm_srli_epi16::<8>(a3);
    let c0 = _mm_and_si128(a0, mask); // lo byte
    let c1 = _mm_and_si128(a1, mask);
    let c2 = _mm_and_si128(a2, mask);
    let c3 = _mm_and_si128(a3, mask);
    let d0 = _mm_add_epi32(b0, c0);
    let d1 = _mm_add_epi32(b1, c1);
    let d2 = _mm_add_epi32(b2, c2);
    let d3 = _mm_add_epi32(b3, c3);
    let e0 = _mm_add_epi32(d0, d1);
    let e1 = _mm_add_epi32(d2, d3);
    let f0 = _mm_add_epi32(e0, e1);
    let mut tmp = [0_u16; 8];
    storeu_si128(&mut tmp, f0);
    dc[0] = u32::from(tmp[0]) + u32::from(tmp[1]);
    dc[1] = u32::from(tmp[2]) + u32::from(tmp[3]);
    dc[2] = u32::from(tmp[4]) + u32::from(tmp[5]);
    dc[3] = u32::from(tmp[6]) + u32::from(tmp[7]);
}

//------------------------------------------------------------------------------
// Texture distortion
//
// We try to match the spectral content (weighted) between source and
// reconstructed samples.

// upstream: src/dsp/enc_sse2.c TTransform_SSE2
/// Hadamard transform
/// Returns the weighted sum of the absolute value of transformed coefficients.
/// w[] contains a row-major 4 by 4 symmetric matrix.
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C names inA, inB, A_b0 and the others")]
fn ttransform_sse2(inA: &[u8], inB: &[u8], w: &[u16; 16]) -> i32 {
    let mut sum = [0_i32; 4];
    let mut tmp_0;
    let mut tmp_1;
    let mut tmp_2;
    let mut tmp_3;
    let zero = _mm_setzero_si128();

    // Load and combine inputs.
    {
        let inA_0 = load_4_bytes(inA);
        let inA_1 = load_4_bytes(&inA[BPS_U..]);
        let inA_2 = load_4_bytes(&inA[2 * BPS_U..]);
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
        tmp_0 = _mm_unpacklo_epi8(inAB_0, zero);
        tmp_1 = _mm_unpacklo_epi8(inAB_1, zero);
        tmp_2 = _mm_unpacklo_epi8(inAB_2, zero);
        tmp_3 = _mm_unpacklo_epi8(inAB_3, zero);
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

        {
            let d0 = _mm_sub_epi16(zero, A_b0);
            let d1 = _mm_sub_epi16(zero, A_b2);
            let d2 = _mm_sub_epi16(zero, B_b0);
            let d3 = _mm_sub_epi16(zero, B_b2);
            A_b0 = _mm_max_epi16(A_b0, d0); // abs(v), 16b
            A_b2 = _mm_max_epi16(A_b2, d1);
            B_b0 = _mm_max_epi16(B_b0, d2);
            B_b2 = _mm_max_epi16(B_b2, d3);
        }

        // weighted sums
        A_b0 = _mm_madd_epi16(A_b0, w_0);
        A_b2 = _mm_madd_epi16(A_b2, w_8);
        B_b0 = _mm_madd_epi16(B_b0, w_0);
        B_b2 = _mm_madd_epi16(B_b2, w_8);
        A_b0 = _mm_add_epi32(A_b0, A_b2);
        B_b0 = _mm_add_epi32(B_b0, B_b2);

        // difference of weighted sums
        A_b0 = _mm_sub_epi32(A_b0, B_b0);
        storeu_si128(&mut sum, A_b0);
    }
    sum[0]
        .wrapping_add(sum[1])
        .wrapping_add(sum[2])
        .wrapping_add(sum[3])
}

// upstream: src/dsp/enc_sse2.c Disto4x4_SSE2
#[target_feature(enable = "sse2")]
fn disto4x4_sse2(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let diff_sum = ttransform_sse2(a, b, w);
    diff_sum.wrapping_abs() >> 5
}

// upstream: src/dsp/enc_sse2.c Disto16x16_SSE2
#[target_feature(enable = "sse2")]
#[expect(non_snake_case, reason = "the C name D")]
fn disto16x16_sse2(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let mut D = 0;
    for y in (0..16 * BPS_U).step_by(4 * BPS_U) {
        for x in (0..16).step_by(4) {
            D += disto4x4_sse2(&a[x + y..], &b[x + y..], w);
        }
    }
    D
}

//------------------------------------------------------------------------------
// Quantization
//

// upstream: src/dsp/enc_sse2.c DoQuantizeBlock_SSE2
#[target_feature(enable = "sse2")]
#[inline]
#[expect(non_snake_case, reason = "the C names coeff_iQ0H and the others")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "MAX_LEVEL is 2047, which fits in a 16-bit lane"
)]
fn do_quantize_block_sse2(
    input: &mut [i16],
    out: &mut [i16],
    sharpen: Option<&[u16; 16]>,
    mtx: &VP8Matrix,
) -> i32 {
    let max_coeff_2047 = _mm_set1_epi16(MAX_LEVEL as i16);
    let zero = _mm_setzero_si128();
    let mut coeff0;
    let mut coeff8;
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

    // extract sign(in)  (0x0000 if positive, 0xffff if negative)
    let sign0 = _mm_cmpgt_epi16(zero, in0);
    let sign8 = _mm_cmpgt_epi16(zero, in8);

    // coeff = abs(in) = (in ^ sign) - sign
    coeff0 = _mm_xor_si128(in0, sign0);
    coeff8 = _mm_xor_si128(in8, sign8);
    coeff0 = _mm_sub_epi16(coeff0, sign0);
    coeff8 = _mm_sub_epi16(coeff8, sign8);

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

    // get sign back (if (sign[j]) out_n = -out_n)
    out0 = _mm_xor_si128(out0, sign0);
    out8 = _mm_xor_si128(out8, sign8);
    out0 = _mm_sub_epi16(out0, sign0);
    out8 = _mm_sub_epi16(out8, sign8);

    // in = out * Q
    in0 = _mm_mullo_epi16(out0, q0);
    in8 = _mm_mullo_epi16(out8, q8);

    storeu_si128(input, in0);
    storeu_si128(&mut input[8..], in8);

    // zigzag the output before storing it.
    //
    // The zigzag pattern can almost be reproduced with a small sequence of
    // shuffles. After it, we only need to swap the 7th (ending up in third
    // position instead of twelfth) and 8th values.
    {
        let mut outZ0 = _mm_shufflehi_epi16::<{ mm_shuffle(2, 1, 3, 0) }>(out0);
        outZ0 = _mm_shuffle_epi32::<{ mm_shuffle(3, 1, 2, 0) }>(outZ0);
        outZ0 = _mm_shufflehi_epi16::<{ mm_shuffle(3, 1, 0, 2) }>(outZ0);
        let mut outZ8 = _mm_shufflelo_epi16::<{ mm_shuffle(3, 0, 2, 1) }>(out8);
        outZ8 = _mm_shuffle_epi32::<{ mm_shuffle(3, 1, 2, 0) }>(outZ8);
        outZ8 = _mm_shufflelo_epi16::<{ mm_shuffle(1, 3, 2, 0) }>(outZ8);
        storeu_si128(out, outZ0);
        storeu_si128(&mut out[8..], outZ8);
        packed_out = _mm_packs_epi16(outZ0, outZ8);
    }
    {
        let outZ_12 = out[12];
        let outZ_3 = out[3];
        out[3] = outZ_12;
        out[12] = outZ_3;
    }

    // detect if all 'out' values are zeroes or not
    i32::from(_mm_movemask_epi8(_mm_cmpeq_epi8(packed_out, zero)) != 0xffff)
}

// upstream: src/dsp/enc_sse2.c QuantizeBlock_SSE2
#[target_feature(enable = "sse2")]
fn quantize_block_sse2(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    do_quantize_block_sse2(input, out, Some(&mtx.sharpen), mtx)
}

// upstream: src/dsp/enc_sse2.c QuantizeBlockWHT_SSE2
#[target_feature(enable = "sse2")]
fn quantize_block_wht_sse2(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    do_quantize_block_sse2(input, out, None, mtx)
}

// upstream: src/dsp/enc_sse2.c Quantize2Blocks_SSE2
#[target_feature(enable = "sse2")]
fn quantize2_blocks_sse2(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    let sharpen = &mtx.sharpen;
    let mut nz = do_quantize_block_sse2(input, out, Some(sharpen), mtx);
    nz |= do_quantize_block_sse2(&mut input[16..], &mut out[16..], Some(sharpen), mtx) << 1;
    nz
}

//------------------------------------------------------------------------------
// Entry points (ADR 0008). The dispatch functions of `src/dsp/enc.rs` call
// them only after `features().sse2`, the answer of `VP8GetCPUInfo(kSSE2)`,
// selected the SSE2 variant, so the CPU has SSE2.

/// Entry point of `CollectHistogram_SSE2` for `VP8CollectHistogram`.
pub(crate) fn collect_histogram(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { collect_histogram_sse2(ref_, pred, start_block, end_block, histo) }
}

/// Entry point of `Intra16Preds_SSE2` for `VP8EncPredLuma16`.
pub(crate) fn intra16_preds(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { intra16_preds_sse2(dst, left, top) }
}

/// Entry point of `IntraChromaPreds_SSE2` for `VP8EncPredChroma8`.
pub(crate) fn intra_chroma_preds(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { intra_chroma_preds_sse2(dst, left, top) }
}

/// Entry point of `Intra4Preds_SSE2` for `VP8EncPredLuma4`.
pub(crate) fn intra4_preds(dst: &mut [u8], top: &[u8], top_pos: usize) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { intra4_preds_sse2(dst, top, top_pos) }
}

/// Entry point of `QuantizeBlock_SSE2` for `VP8EncQuantizeBlock`.
pub(crate) fn quantize_block(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { quantize_block_sse2(input, out, mtx) }
}

/// Entry point of `Quantize2Blocks_SSE2` for `VP8EncQuantize2Blocks`.
pub(crate) fn quantize2_blocks(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { quantize2_blocks_sse2(input, out, mtx) }
}

/// Entry point of `QuantizeBlockWHT_SSE2` for `VP8EncQuantizeBlockWHT`.
pub(crate) fn quantize_block_wht(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { quantize_block_wht_sse2(input, out, mtx) }
}

/// Entry point of `ITransform_SSE2` for `VP8ITransform`.
pub(crate) fn i_transform(ref_: &[u8], input: &[i16], dst: &mut [u8], do_two: bool) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { itransform_sse2(ref_, input, dst, do_two) }
}

/// Entry point of `FTransform_SSE2` for `VP8FTransform`.
pub(crate) fn f_transform(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { f_transform_sse2(src, ref_, out) }
}

/// Entry point of `FTransform2_SSE2` for `VP8FTransform2`.
pub(crate) fn f_transform2(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { f_transform2_sse2(src, ref_, out) }
}

/// Entry point of `FTransformWHT_SSE2` for `VP8FTransformWHT`.
pub(crate) fn f_transform_wht(input: &[i16], out: &mut [i16]) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { f_transform_wht_sse2(input, out) }
}

/// Entry point of `SSE16x16_SSE2` for `VP8SSE16x16`.
pub(crate) fn sse16x16(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { sse16x16_sse2(a, b) }
}

/// Entry point of `SSE16x8_SSE2` for `VP8SSE16x8`.
pub(crate) fn sse16x8(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { sse16x8_sse2(a, b) }
}

/// Entry point of `SSE8x8_SSE2` for `VP8SSE8x8`.
pub(crate) fn sse8x8(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { sse8x8_sse2(a, b) }
}

/// Entry point of `SSE4x4_SSE2` for `VP8SSE4x4`.
pub(crate) fn sse4x4(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { sse4x4_sse2(a, b) }
}

/// Entry point of `Disto4x4_SSE2` for `VP8TDisto4x4`.
pub(crate) fn disto4x4(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { disto4x4_sse2(a, b, w) }
}

/// Entry point of `Disto16x16_SSE2` for `VP8TDisto16x16`.
pub(crate) fn disto16x16(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { disto16x16_sse2(a, b, w) }
}

/// Entry point of `Mean16x4_SSE2` for `VP8Mean16x4`.
pub(crate) fn mean16x4(ref_: &[u8], dc: &mut [u32]) {
    // SAFETY: called only after `features().sse2` selected the SSE2 variant.
    unsafe { mean16x4_sse2(ref_, dc) }
}
