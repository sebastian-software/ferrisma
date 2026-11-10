// libwebp v1.6.0, src/dsp/enc.c:
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
// Speed-critical encoding functions.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dsp/enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **DSP forms.** Predictors take the destination buffer, `left` as
//!    `Option<(&[u8], usize)>` (the buffer and the position of `left[0]`,
//!    because C reads `left[-1]`), and `top` as `Option<&[u8]>`; `None` is C's
//!    `NULL`. `VP8EncPredLuma4` takes the boundary and the position of
//!    `top[0]`, because C reads `top[-5]` to `top[7]`. `VP8Mean16x4` writes
//!    four entries of a slice. Transforms and quantizers take slices from the
//!    block's first element.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Signed overflow in `VP8ITransform`.** Coefficients above about 15,700
//!    in magnitude overflow the `int` products of the horizontal pass; the dev
//!    and `checked` profiles report it. The DSP test bounds the coefficients at
//!    15,000; the encoder's dequantized levels stay far below.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **SIMD variants.** `enc_sse2.c`, `enc_sse41.c`, `enc_neon.c`,
//!    `cost_sse2.c`, `cost_neon.c` and `IsFlat_NEON` of `quant.h` belong to the
//!    SIMD modules (ADR 0008); the reference configuration compiles none of
//!    them, so `IsFlat` is `IsFlat_C`. MIPS and MSA variants are excluded
//!    (`tools/units.json`).

use std::sync::LazyLock;

use crate::dsp::dsp::{
    BPS, MAX_COEFF_THRESH, VP8Histogram, webp_transform_ac3_mul1, webp_transform_ac3_mul2,
};
use crate::enc::vp8i_enc::{
    C8DC8, C8HE8, C8TM8, C8VE8, I4DC4, I4HD4, I4HE4, I4HU4, I4LD4, I4RD4, I4TM4, I4VE4, I4VL4,
    I4VR4, I16DC16, I16HE16, I16TM16, I16VE16, MAX_LEVEL, VP8Matrix, quantdiv,
};

/// `BPS` as an index stride.
const BPS_U: usize = BPS as usize;

// upstream: src/dsp/enc.c clip_8b
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the value is in 0..=255 on the branch that converts it"
)]
fn clip_8b(v: i32) -> u8 {
    if v & !0xff == 0 {
        v as u8
    } else if v < 0 {
        0
    } else {
        255
    }
}

// upstream: src/dsp/enc.c clip_max
fn clip_max(v: i32, max: i32) -> i32 {
    if v > max { max } else { v }
}

//------------------------------------------------------------------------------
// Compute susceptibility based on DCT-coeff histograms:
// the higher, the "easier" the macroblock is to compress.

// upstream: src/dsp/enc.c VP8DspScan
pub static VP8_DSP_SCAN: [i32; 16 + 4 + 4] = [
    // Luma
    0,
    4,
    8,
    12,
    4 * BPS,
    4 + 4 * BPS,
    8 + 4 * BPS,
    12 + 4 * BPS,
    8 * BPS,
    4 + 8 * BPS,
    8 + 8 * BPS,
    12 + 8 * BPS,
    12 * BPS,
    4 + 12 * BPS,
    8 + 12 * BPS,
    12 + 12 * BPS,
    // U
    0,
    4,
    4 * BPS,
    4 + 4 * BPS,
    // V
    8,
    12,
    8 + 4 * BPS,
    12 + 4 * BPS,
];

// upstream: src/dsp/enc.c VP8SetHistogramData
/// General-purpose util function to help `VP8CollectHistogram()`.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "k is in 0..=MAX_COEFF_THRESH"
)]
pub fn vp8_set_histogram_data(
    distribution: &[i32; MAX_COEFF_THRESH as usize + 1],
    histo: &mut VP8Histogram,
) {
    let mut max_value = 0;
    let mut last_non_zero = 1;
    for (k, &value) in distribution.iter().enumerate() {
        if value > 0 {
            if value > max_value {
                max_value = value;
            }
            last_non_zero = k as i32;
        }
    }
    histo.max_value = max_value;
    histo.last_non_zero = last_non_zero;
}

// upstream: src/dsp/enc.c CollectHistogram_C
#[expect(
    clippy::cast_sign_loss,
    reason = "the block offsets and clipped values are non-negative"
)]
fn collect_histogram_c(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    let mut distribution = [0_i32; MAX_COEFF_THRESH as usize + 1];
    for j in start_block..end_block {
        let mut out = [0_i16; 16];
        let off = VP8_DSP_SCAN[j as usize] as usize;

        vp8f_transform(&ref_[off..], &pred[off..], &mut out);

        // Convert coefficients to bin.
        for &coeff in &out {
            let v = i32::from(coeff).abs() >> 3;
            let clipped_value = clip_max(v, MAX_COEFF_THRESH);
            distribution[clipped_value as usize] += 1;
        }
    }
    vp8_set_histogram_data(&distribution, histo);
}

//------------------------------------------------------------------------------
// run-time tables (~4k)

// upstream: src/dsp/enc.c clip1
/// Clips [-255,510] to [0,255]; index `255 + i` holds `clip_8b(i)`. C fills it
/// once in `InitTables`, guarded by `tables_ok`.
static CLIP1: LazyLock<[u8; 255 + 510 + 1]> = LazyLock::new(init_tables);

// upstream: src/dsp/enc.c InitTables
fn init_tables() -> [u8; 255 + 510 + 1] {
    let mut clip1 = [0_u8; 255 + 510 + 1];
    for (index, entry) in clip1.iter_mut().enumerate() {
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_possible_wrap,
            reason = "the index is below 766"
        )]
        let i = index as i32 - 255;
        *entry = clip_8b(i);
    }
    clip1
}

//------------------------------------------------------------------------------
// Transforms (Paragraph 14.4)

// upstream: src/dsp/enc.c STORE
fn store(dst: &mut [u8], ref_: &[u8], x: usize, y: usize, v: i32) {
    dst[x + y * BPS_U] = clip_8b(i32::from(ref_[x + y * BPS_U]) + (v >> 3));
}

// upstream: src/dsp/enc.c ITransformOne
fn i_transform_one(ref_: &[u8], input: &[i16], dst: &mut [u8]) {
    let mut c = [0_i32; 4 * 4];
    for i in 0..4 {
        // vertical pass
        let inp = &input[i..];
        let a = i32::from(inp[0]) + i32::from(inp[8]);
        let b = i32::from(inp[0]) - i32::from(inp[8]);
        let c1 = webp_transform_ac3_mul2(i32::from(inp[4]))
            - webp_transform_ac3_mul1(i32::from(inp[12]));
        let d = webp_transform_ac3_mul1(i32::from(inp[4]))
            + webp_transform_ac3_mul2(i32::from(inp[12]));
        let tmp = &mut c[4 * i..];
        tmp[0] = a + d;
        tmp[1] = b + c1;
        tmp[2] = b - c1;
        tmp[3] = a - d;
    }

    for i in 0..4 {
        // horizontal pass
        let tmp = &c[i..];
        let dc = tmp[0] + 4;
        let a = dc + tmp[8];
        let b = dc - tmp[8];
        let c1 = webp_transform_ac3_mul2(tmp[4]) - webp_transform_ac3_mul1(tmp[12]);
        let d = webp_transform_ac3_mul1(tmp[4]) + webp_transform_ac3_mul2(tmp[12]);
        store(dst, ref_, 0, i, a + d);
        store(dst, ref_, 1, i, b + c1);
        store(dst, ref_, 2, i, b - c1);
        store(dst, ref_, 3, i, a - d);
    }
}

// upstream: src/dsp/enc.c ITransform_C
fn i_transform_c(ref_: &[u8], input: &[i16], dst: &mut [u8], do_two: bool) {
    i_transform_one(ref_, input, dst);
    if do_two {
        i_transform_one(&ref_[4..], &input[16..], &mut dst[4..]);
    }
}

// upstream: src/dsp/enc.c FTransform_C
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the 12-bit coefficients into int16_t"
)]
fn f_transform_c(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    let mut tmp = [0_i32; 16];
    for i in 0..4 {
        let s = &src[i * BPS_U..];
        let r = &ref_[i * BPS_U..];
        let d0 = i32::from(s[0]) - i32::from(r[0]); // 9bit dynamic range ([-255,255])
        let d1 = i32::from(s[1]) - i32::from(r[1]);
        let d2 = i32::from(s[2]) - i32::from(r[2]);
        let d3 = i32::from(s[3]) - i32::from(r[3]);
        let a0 = d0 + d3; // 10b                      [-510,510]
        let a1 = d1 + d2;
        let a2 = d1 - d2;
        let a3 = d0 - d3;
        tmp[i * 4] = (a0 + a1) * 8; // 14b                      [-8160,8160]
        tmp[1 + i * 4] = (a2 * 2217 + a3 * 5352 + 1812) >> 9; // [-7536,7542]
        tmp[2 + i * 4] = (a0 - a1) * 8;
        tmp[3 + i * 4] = (a3 * 2217 - a2 * 5352 + 937) >> 9;
    }
    for i in 0..4 {
        let a0 = tmp[i] + tmp[12 + i]; // 15b
        let a1 = tmp[4 + i] + tmp[8 + i];
        let a2 = tmp[4 + i] - tmp[8 + i];
        let a3 = tmp[i] - tmp[12 + i];
        out[i] = ((a0 + a1 + 7) >> 4) as i16; // 12b
        out[4 + i] = (((a2 * 2217 + a3 * 5352 + 12000) >> 16) + i32::from(a3 != 0)) as i16;
        out[8 + i] = ((a0 - a1 + 7) >> 4) as i16;
        out[12 + i] = ((a3 * 2217 - a2 * 5352 + 51000) >> 16) as i16;
    }
}

// upstream: src/dsp/enc.c FTransform2_C
fn f_transform2_c(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    vp8f_transform(src, ref_, out);
    vp8f_transform(&src[4..], &ref_[4..], &mut out[16..]);
}

// upstream: src/dsp/enc.c FTransformWHT_C
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the 15-bit coefficients into int16_t"
)]
fn f_transform_wht_c(input: &[i16], out: &mut [i16]) {
    // input is 12b signed
    let mut tmp = [0_i32; 16];
    for i in 0..4 {
        let inp = &input[i * 64..];
        let a0 = i32::from(inp[0]) + i32::from(inp[2 * 16]); // 13b
        let a1 = i32::from(inp[16]) + i32::from(inp[3 * 16]);
        let a2 = i32::from(inp[16]) - i32::from(inp[3 * 16]);
        let a3 = i32::from(inp[0]) - i32::from(inp[2 * 16]);
        tmp[i * 4] = a0 + a1; // 14b
        tmp[1 + i * 4] = a3 + a2;
        tmp[2 + i * 4] = a3 - a2;
        tmp[3 + i * 4] = a0 - a1;
    }
    for i in 0..4 {
        let a0 = tmp[i] + tmp[8 + i]; // 15b
        let a1 = tmp[4 + i] + tmp[12 + i];
        let a2 = tmp[4 + i] - tmp[12 + i];
        let a3 = tmp[i] - tmp[8 + i];
        let b0 = a0 + a1; // 16b
        let b1 = a3 + a2;
        let b2 = a3 - a2;
        let b3 = a0 - a1;
        out[i] = (b0 >> 1) as i16; // 15b
        out[4 + i] = (b1 >> 1) as i16;
        out[8 + i] = (b2 >> 1) as i16;
        out[12 + i] = (b3 >> 1) as i16;
    }
}

//------------------------------------------------------------------------------
// Intra predictions

/// The left samples of a prediction: the buffer and the position of `left[0]`.
/// C reads `left[-1]`, the top-left sample, too.
pub type PredLeft<'a> = Option<(&'a [u8], usize)>;

/// The position of a prediction block inside the prediction buffer.
#[expect(clippy::cast_sign_loss, reason = "the block offsets are non-negative")]
const fn at(offset: i32) -> usize {
    offset as usize
}

// upstream: src/dsp/enc.c Fill
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C memsets the low byte of the value"
)]
fn fill(dst: &mut [u8], value: i32, size: usize) {
    for j in 0..size {
        dst[j * BPS_U..j * BPS_U + size].fill(value as u8);
    }
}

// upstream: src/dsp/enc.c VerticalPred
fn vertical_pred(dst: &mut [u8], top: Option<&[u8]>, size: usize) {
    if let Some(top) = top {
        for j in 0..size {
            dst[j * BPS_U..j * BPS_U + size].copy_from_slice(&top[..size]);
        }
    } else {
        fill(dst, 127, size);
    }
}

// upstream: src/dsp/enc.c HorizontalPred
fn horizontal_pred(dst: &mut [u8], left: PredLeft<'_>, size: usize) {
    if let Some((left, pos)) = left {
        for j in 0..size {
            dst[j * BPS_U..j * BPS_U + size].fill(left[pos + j]);
        }
    } else {
        fill(dst, 129, size);
    }
}

// upstream: src/dsp/enc.c TrueMotion
fn true_motion(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>, size: usize) {
    if let Some((left_buf, pos)) = left {
        if let Some(top) = top {
            // C: clip = clip1 + 255 - left[-1]
            let clip = 255 - usize::from(left_buf[pos - 1]);
            for y in 0..size {
                let clip_table = clip + usize::from(left_buf[pos + y]);
                let row = &mut dst[y * BPS_U..];
                for x in 0..size {
                    row[x] = CLIP1[clip_table + usize::from(top[x])];
                }
            }
        } else {
            horizontal_pred(dst, left, size);
        }
    } else {
        // true motion without left samples (hence: with default 129 value)
        // is equivalent to VE prediction where you just copy the top samples.
        // Note that if top samples are not available, the default value is
        // then 129, and not 127 as in the VerticalPred case.
        if top.is_some() {
            vertical_pred(dst, top, size);
        } else {
            fill(dst, 129, size);
        }
    }
}

// upstream: src/dsp/enc.c DCMode
fn dc_mode(
    dst: &mut [u8],
    left: PredLeft<'_>,
    top: Option<&[u8]>,
    size: usize,
    round: i32,
    shift: i32,
) {
    let mut dc = 0_i32;
    if let Some(top) = top {
        for &sample in &top[..size] {
            dc += i32::from(sample);
        }
        if let Some((left, pos)) = left {
            // top and left present
            for &sample in &left[pos..pos + size] {
                dc += i32::from(sample);
            }
        } else {
            // top, but no left
            dc += dc;
        }
        dc = (dc + round) >> shift;
    } else if let Some((left, pos)) = left {
        // left but no top
        for &sample in &left[pos..pos + size] {
            dc += i32::from(sample);
        }
        dc += dc;
        dc = (dc + round) >> shift;
    } else {
        // no top, no left, nothing.
        dc = 0x80;
    }
    fill(dst, dc, size);
}

//------------------------------------------------------------------------------
// Chroma 8x8 prediction (paragraph 12.2)

// upstream: src/dsp/enc.c IntraChromaPreds_C
fn intra_chroma_preds_c(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    // U block
    dc_mode(&mut dst[at(C8DC8)..], left, top, 8, 8, 4);
    vertical_pred(&mut dst[at(C8VE8)..], top, 8);
    horizontal_pred(&mut dst[at(C8HE8)..], left, 8);
    true_motion(&mut dst[at(C8TM8)..], left, top, 8);
    // V block
    let dst = &mut dst[8..];
    let top = top.map(|top| &top[8..]);
    let left = left.map(|(left, pos)| (left, pos + 16));
    dc_mode(&mut dst[at(C8DC8)..], left, top, 8, 8, 4);
    vertical_pred(&mut dst[at(C8VE8)..], top, 8);
    horizontal_pred(&mut dst[at(C8HE8)..], left, 8);
    true_motion(&mut dst[at(C8TM8)..], left, top, 8);
}

//------------------------------------------------------------------------------
// luma 16x16 prediction (paragraph 12.3)

// upstream: src/dsp/enc.c Intra16Preds_C
fn intra16_preds_c(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    dc_mode(&mut dst[at(I16DC16)..], left, top, 16, 16, 5);
    vertical_pred(&mut dst[at(I16VE16)..], top, 16);
    horizontal_pred(&mut dst[at(I16HE16)..], left, 16);
    true_motion(&mut dst[at(I16TM16)..], left, top, 16);
}

//------------------------------------------------------------------------------
// luma 4x4 prediction

// upstream: src/dsp/enc.c DST
fn dst_set(dst: &mut [u8], x: usize, y: usize, v: u8) {
    dst[x + y * BPS_U] = v;
}

// upstream: src/dsp/enc.c AVG3
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C casts the average to uint8_t"
)]
fn avg3(a: i32, b: i32, c: i32) -> u8 {
    ((a + 2 * b + c + 2) >> 2) as u8
}

// upstream: src/dsp/enc.c AVG2
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the average of two samples fits uint8_t"
)]
fn avg2(a: i32, b: i32) -> u8 {
    ((a + b + 1) >> 1) as u8
}

/// The sample `top[offset]` of the boundary `top` at position `pos`, which C
/// reads at negative offsets for the left samples.
fn tp(top: &[u8], pos: usize, offset: isize) -> i32 {
    i32::from(
        top[pos
            .checked_add_signed(offset)
            .expect("the boundary sample exists")],
    )
}

// upstream: src/dsp/enc.c VE4
/// vertical
fn ve4(dst: &mut [u8], top: &[u8], p: usize) {
    let vals = [
        avg3(tp(top, p, -1), tp(top, p, 0), tp(top, p, 1)),
        avg3(tp(top, p, 0), tp(top, p, 1), tp(top, p, 2)),
        avg3(tp(top, p, 1), tp(top, p, 2), tp(top, p, 3)),
        avg3(tp(top, p, 2), tp(top, p, 3), tp(top, p, 4)),
    ];
    for i in 0..4 {
        dst[i * BPS_U..i * BPS_U + 4].copy_from_slice(&vals);
    }
}

// upstream: src/dsp/enc.c HE4
/// horizontal
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn he4(dst: &mut [u8], top: &[u8], p: usize) {
    let x = tp(top, p, -1);
    let i = tp(top, p, -2);
    let j = tp(top, p, -3);
    let k = tp(top, p, -4);
    let l = tp(top, p, -5);
    // C: WebPUint32ToMem(dst + n * BPS, 0x01010101U * AVG3(...))
    dst[..4].fill(avg3(x, i, j));
    dst[BPS_U..BPS_U + 4].fill(avg3(i, j, k));
    dst[2 * BPS_U..2 * BPS_U + 4].fill(avg3(j, k, l));
    dst[3 * BPS_U..3 * BPS_U + 4].fill(avg3(k, l, l));
}

// upstream: src/dsp/enc.c DC4
#[expect(
    clippy::cast_possible_wrap,
    reason = "the sum of eight samples fits int"
)]
fn dc4(dst: &mut [u8], top: &[u8], p: usize) {
    let mut dc: u32 = 4;
    for i in 0..4 {
        #[expect(clippy::cast_sign_loss, reason = "samples are non-negative")]
        {
            dc += (tp(top, p, i) + tp(top, p, -5 + i)) as u32;
        }
    }
    fill(dst, (dc >> 3) as i32, 4);
}

// upstream: src/dsp/enc.c RD4
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn rd4(dst: &mut [u8], top: &[u8], p: usize) {
    let x = tp(top, p, -1);
    let i = tp(top, p, -2);
    let j = tp(top, p, -3);
    let k = tp(top, p, -4);
    let l = tp(top, p, -5);
    let a = tp(top, p, 0);
    let b = tp(top, p, 1);
    let c = tp(top, p, 2);
    let d = tp(top, p, 3);
    dst_set(dst, 0, 3, avg3(j, k, l));
    let v = avg3(i, j, k);
    dst_set(dst, 0, 2, v);
    dst_set(dst, 1, 3, v);
    let v = avg3(x, i, j);
    dst_set(dst, 0, 1, v);
    dst_set(dst, 1, 2, v);
    dst_set(dst, 2, 3, v);
    let v = avg3(a, x, i);
    dst_set(dst, 0, 0, v);
    dst_set(dst, 1, 1, v);
    dst_set(dst, 2, 2, v);
    dst_set(dst, 3, 3, v);
    let v = avg3(b, a, x);
    dst_set(dst, 1, 0, v);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 3, 2, v);
    let v = avg3(c, b, a);
    dst_set(dst, 2, 0, v);
    dst_set(dst, 3, 1, v);
    dst_set(dst, 3, 0, avg3(d, c, b));
}

// upstream: src/dsp/enc.c LD4
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn ld4(dst: &mut [u8], top: &[u8], p: usize) {
    let a = tp(top, p, 0);
    let b = tp(top, p, 1);
    let c = tp(top, p, 2);
    let d = tp(top, p, 3);
    let e = tp(top, p, 4);
    let f = tp(top, p, 5);
    let g = tp(top, p, 6);
    let h = tp(top, p, 7);
    dst_set(dst, 0, 0, avg3(a, b, c));
    let v = avg3(b, c, d);
    dst_set(dst, 1, 0, v);
    dst_set(dst, 0, 1, v);
    let v = avg3(c, d, e);
    dst_set(dst, 2, 0, v);
    dst_set(dst, 1, 1, v);
    dst_set(dst, 0, 2, v);
    let v = avg3(d, e, f);
    dst_set(dst, 3, 0, v);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 1, 2, v);
    dst_set(dst, 0, 3, v);
    let v = avg3(e, f, g);
    dst_set(dst, 3, 1, v);
    dst_set(dst, 2, 2, v);
    dst_set(dst, 1, 3, v);
    let v = avg3(f, g, h);
    dst_set(dst, 3, 2, v);
    dst_set(dst, 2, 3, v);
    dst_set(dst, 3, 3, avg3(g, h, h));
}

// upstream: src/dsp/enc.c VR4
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn vr4(dst: &mut [u8], top: &[u8], p: usize) {
    let x = tp(top, p, -1);
    let i = tp(top, p, -2);
    let j = tp(top, p, -3);
    let k = tp(top, p, -4);
    let a = tp(top, p, 0);
    let b = tp(top, p, 1);
    let c = tp(top, p, 2);
    let d = tp(top, p, 3);
    let v = avg2(x, a);
    dst_set(dst, 0, 0, v);
    dst_set(dst, 1, 2, v);
    let v = avg2(a, b);
    dst_set(dst, 1, 0, v);
    dst_set(dst, 2, 2, v);
    let v = avg2(b, c);
    dst_set(dst, 2, 0, v);
    dst_set(dst, 3, 2, v);
    dst_set(dst, 3, 0, avg2(c, d));

    dst_set(dst, 0, 3, avg3(k, j, i));
    dst_set(dst, 0, 2, avg3(j, i, x));
    let v = avg3(i, x, a);
    dst_set(dst, 0, 1, v);
    dst_set(dst, 1, 3, v);
    let v = avg3(x, a, b);
    dst_set(dst, 1, 1, v);
    dst_set(dst, 2, 3, v);
    let v = avg3(a, b, c);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 3, 3, v);
    dst_set(dst, 3, 1, avg3(b, c, d));
}

// upstream: src/dsp/enc.c VL4
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn vl4(dst: &mut [u8], top: &[u8], p: usize) {
    let a = tp(top, p, 0);
    let b = tp(top, p, 1);
    let c = tp(top, p, 2);
    let d = tp(top, p, 3);
    let e = tp(top, p, 4);
    let f = tp(top, p, 5);
    let g = tp(top, p, 6);
    let h = tp(top, p, 7);
    dst_set(dst, 0, 0, avg2(a, b));
    let v = avg2(b, c);
    dst_set(dst, 1, 0, v);
    dst_set(dst, 0, 2, v);
    let v = avg2(c, d);
    dst_set(dst, 2, 0, v);
    dst_set(dst, 1, 2, v);
    let v = avg2(d, e);
    dst_set(dst, 3, 0, v);
    dst_set(dst, 2, 2, v);

    dst_set(dst, 0, 1, avg3(a, b, c));
    let v = avg3(b, c, d);
    dst_set(dst, 1, 1, v);
    dst_set(dst, 0, 3, v);
    let v = avg3(c, d, e);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 1, 3, v);
    let v = avg3(d, e, f);
    dst_set(dst, 3, 1, v);
    dst_set(dst, 2, 3, v);
    dst_set(dst, 3, 2, avg3(e, f, g));
    dst_set(dst, 3, 3, avg3(f, g, h));
}

// upstream: src/dsp/enc.c HU4
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "L is a sample"
)]
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn hu4(dst: &mut [u8], top: &[u8], p: usize) {
    let i = tp(top, p, -2);
    let j = tp(top, p, -3);
    let k = tp(top, p, -4);
    let l = tp(top, p, -5);
    dst_set(dst, 0, 0, avg2(i, j));
    let v = avg2(j, k);
    dst_set(dst, 2, 0, v);
    dst_set(dst, 0, 1, v);
    let v = avg2(k, l);
    dst_set(dst, 2, 1, v);
    dst_set(dst, 0, 2, v);
    dst_set(dst, 1, 0, avg3(i, j, k));
    let v = avg3(j, k, l);
    dst_set(dst, 3, 0, v);
    dst_set(dst, 1, 1, v);
    let v = avg3(k, l, l);
    dst_set(dst, 3, 1, v);
    dst_set(dst, 1, 2, v);
    let v = l as u8;
    dst_set(dst, 3, 2, v);
    dst_set(dst, 2, 2, v);
    dst_set(dst, 0, 3, v);
    dst_set(dst, 1, 3, v);
    dst_set(dst, 2, 3, v);
    dst_set(dst, 3, 3, v);
}

// upstream: src/dsp/enc.c HD4
#[expect(clippy::many_single_char_names, reason = "the C sample names")]
fn hd4(dst: &mut [u8], top: &[u8], p: usize) {
    let x = tp(top, p, -1);
    let i = tp(top, p, -2);
    let j = tp(top, p, -3);
    let k = tp(top, p, -4);
    let l = tp(top, p, -5);
    let a = tp(top, p, 0);
    let b = tp(top, p, 1);
    let c = tp(top, p, 2);

    let v = avg2(i, x);
    dst_set(dst, 0, 0, v);
    dst_set(dst, 2, 1, v);
    let v = avg2(j, i);
    dst_set(dst, 0, 1, v);
    dst_set(dst, 2, 2, v);
    let v = avg2(k, j);
    dst_set(dst, 0, 2, v);
    dst_set(dst, 2, 3, v);
    dst_set(dst, 0, 3, avg2(l, k));

    dst_set(dst, 3, 0, avg3(a, b, c));
    dst_set(dst, 2, 0, avg3(x, a, b));
    let v = avg3(i, x, a);
    dst_set(dst, 1, 0, v);
    dst_set(dst, 3, 1, v);
    let v = avg3(j, i, x);
    dst_set(dst, 1, 1, v);
    dst_set(dst, 3, 2, v);
    let v = avg3(k, j, i);
    dst_set(dst, 1, 2, v);
    dst_set(dst, 3, 3, v);
    dst_set(dst, 1, 3, avg3(l, k, j));
}

// upstream: src/dsp/enc.c TM4
#[expect(clippy::cast_sign_loss, reason = "samples are non-negative")]
fn tm4(dst: &mut [u8], top: &[u8], p: usize) {
    // C: clip = clip1 + 255 - top[-1]
    let clip = 255 - tp(top, p, -1) as usize;
    for y in 0..4 {
        #[expect(clippy::cast_possible_wrap, reason = "y is below 4")]
        let clip_table = clip + tp(top, p, -2 - y as isize) as usize;
        let row = &mut dst[y * BPS_U..];
        for x in 0..4 {
            #[expect(clippy::cast_possible_wrap, reason = "x is below 4")]
            {
                row[x] = CLIP1[clip_table + tp(top, p, x as isize) as usize];
            }
        }
    }
}

// upstream: src/dsp/enc.c Intra4Preds_C
/// Left samples are `top[-5 .. -2]`, `top_left` is `top[-1]`, top are located
/// at `top[0..3]`, and top right is `top[4..7]`. `top` is the boundary buffer and `p`
/// the position of `top[0]` in it.
fn intra4_preds_c(dst: &mut [u8], top: &[u8], p: usize) {
    dc4(&mut dst[at(I4DC4)..], top, p);
    tm4(&mut dst[at(I4TM4)..], top, p);
    ve4(&mut dst[at(I4VE4)..], top, p);
    he4(&mut dst[at(I4HE4)..], top, p);
    rd4(&mut dst[at(I4RD4)..], top, p);
    vr4(&mut dst[at(I4VR4)..], top, p);
    ld4(&mut dst[at(I4LD4)..], top, p);
    vl4(&mut dst[at(I4VL4)..], top, p);
    hd4(&mut dst[at(I4HD4)..], top, p);
    hu4(&mut dst[at(I4HU4)..], top, p);
}

//------------------------------------------------------------------------------
// Metric

// upstream: src/dsp/enc.c GetSSE
fn get_sse(a: &[u8], b: &[u8], w: usize, h: usize) -> i32 {
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let diff = i32::from(a[x + y * BPS_U]) - i32::from(b[x + y * BPS_U]);
            count += diff * diff;
        }
    }
    count
}

// upstream: src/dsp/enc.c SSE16x16_C
fn sse16x16_c(a: &[u8], b: &[u8]) -> i32 {
    get_sse(a, b, 16, 16)
}
// upstream: src/dsp/enc.c SSE16x8_C
fn sse16x8_c(a: &[u8], b: &[u8]) -> i32 {
    get_sse(a, b, 16, 8)
}
// upstream: src/dsp/enc.c SSE8x8_C
fn sse8x8_c(a: &[u8], b: &[u8]) -> i32 {
    get_sse(a, b, 8, 8)
}
// upstream: src/dsp/enc.c SSE4x4_C
fn sse4x4_c(a: &[u8], b: &[u8]) -> i32 {
    get_sse(a, b, 4, 4)
}

// upstream: src/dsp/enc.c Mean16x4_C
fn mean16x4_c(ref_: &[u8], dc: &mut [u32]) {
    for k in 0..4 {
        let mut avg: u32 = 0;
        for y in 0..4 {
            for x in 0..4 {
                avg += u32::from(ref_[4 * k + x + y * BPS_U]);
            }
        }
        dc[k] = avg;
        // go to next 4x4 block.
    }
}

//------------------------------------------------------------------------------
// Texture distortion
//
// We try to match the spectral content (weighted) between source and
// reconstructed samples.

// upstream: src/dsp/enc.c TTransform
/// Hadamard transform. Returns the weighted sum of the absolute value of
/// transformed coefficients. `w[]` contains a row-major 4 by 4 symmetric matrix.
fn t_transform(input: &[u8], w: &[u16; 16]) -> i32 {
    let mut sum = 0;
    let mut tmp = [0_i32; 16];
    // horizontal pass
    for i in 0..4 {
        let inp = &input[i * BPS_U..];
        let a0 = i32::from(inp[0]) + i32::from(inp[2]);
        let a1 = i32::from(inp[1]) + i32::from(inp[3]);
        let a2 = i32::from(inp[1]) - i32::from(inp[3]);
        let a3 = i32::from(inp[0]) - i32::from(inp[2]);
        tmp[i * 4] = a0 + a1;
        tmp[1 + i * 4] = a3 + a2;
        tmp[2 + i * 4] = a3 - a2;
        tmp[3 + i * 4] = a0 - a1;
    }
    // vertical pass
    for i in 0..4 {
        let a0 = tmp[i] + tmp[8 + i];
        let a1 = tmp[4 + i] + tmp[12 + i];
        let a2 = tmp[4 + i] - tmp[12 + i];
        let a3 = tmp[i] - tmp[8 + i];
        let b0 = a0 + a1;
        let b1 = a3 + a2;
        let b2 = a3 - a2;
        let b3 = a0 - a1;

        sum += i32::from(w[i]) * b0.abs();
        sum += i32::from(w[4 + i]) * b1.abs();
        sum += i32::from(w[8 + i]) * b2.abs();
        sum += i32::from(w[12 + i]) * b3.abs();
    }
    sum
}

// upstream: src/dsp/enc.c Disto4x4_C
fn disto4x4_c(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let sum1 = t_transform(a, w);
    let sum2 = t_transform(b, w);
    (sum2 - sum1).abs() >> 5
}

// upstream: src/dsp/enc.c Disto16x16_C
fn disto16x16_c(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let mut d = 0;
    for y in (0..16 * BPS_U).step_by(4 * BPS_U) {
        for x in (0..16).step_by(4) {
            d += disto4x4_c(&a[x + y..], &b[x + y..], w);
        }
    }
    d
}

//------------------------------------------------------------------------------
// Quantization
//

// upstream: src/dsp/enc.c kZigzag
static ZIGZAG: [u8; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

// upstream: src/dsp/enc.c QuantizeBlock_C
/// Simple quantization.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "C converts between int, int16_t and uint32_t at these points"
)]
fn quantize_block_c(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    let mut last = -1;
    for n in 0..16 {
        let j = usize::from(ZIGZAG[n]);
        let sign = input[j] < 0;
        let coeff = (if sign {
            -i32::from(input[j])
        } else {
            i32::from(input[j])
        } + i32::from(mtx.sharpen[j])) as u32;
        if coeff > mtx.zthresh[j] {
            let q = u32::from(mtx.q[j]);
            let i_q = u32::from(mtx.iq[j]);
            let b = mtx.bias[j];
            let mut level = quantdiv(coeff, i_q, b);
            if level > MAX_LEVEL {
                level = MAX_LEVEL;
            }
            if sign {
                level = -level;
            }
            input[j] = (level * q as i32) as i16;
            out[n] = level as i16;
            if level != 0 {
                last = n as i32;
            }
        } else {
            out[n] = 0;
            input[j] = 0;
        }
    }
    i32::from(last >= 0)
}

// upstream: src/dsp/enc.c Quantize2Blocks_C
fn quantize2_blocks_c(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    let mut nz = vp8_enc_quantize_block(&mut input[..16], &mut out[..16], mtx);
    nz |= vp8_enc_quantize_block(&mut input[16..32], &mut out[16..32], mtx) << 1;
    nz
}

//------------------------------------------------------------------------------
// Block copy

// upstream: src/dsp/enc.c Copy
fn copy(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    for y in 0..h {
        dst[y * BPS_U..y * BPS_U + w].copy_from_slice(&src[y * BPS_U..y * BPS_U + w]);
    }
}

// upstream: src/dsp/enc.c Copy4x4_C
fn copy4x4_c(src: &[u8], dst: &mut [u8]) {
    copy(src, dst, 4, 4);
}

// upstream: src/dsp/enc.c Copy16x8_C
fn copy16x8_c(src: &[u8], dst: &mut [u8]) {
    copy(src, dst, 16, 8);
}

//------------------------------------------------------------------------------
// Initialization
//
// Speed-critical function pointers. C initializes them to the default
// implementations within VP8EncDspInit(); each is a plain function over the
// portable code here (ADR 0003).

// upstream: src/dsp/enc.c VP8CollectHistogram
/// Collects the coefficient histogram of the blocks `start_block..end_block` of
/// `ref_` against `pred` (C: the `VP8CollectHistogram` pointer).
pub fn vp8_collect_histogram(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    collect_histogram_c(ref_, pred, start_block, end_block, histo);
}

// upstream: src/dsp/enc.c VP8ITransform
/// Does one of two inverse transforms. If `do_two` is set, the transforms will
/// be done for (ref, in, dst) and (ref + 4, in + 16, dst + 4) (C: the
/// `VP8ITransform` pointer).
pub fn vp8i_transform(ref_: &[u8], input: &[i16], dst: &mut [u8], do_two: bool) {
    i_transform_c(ref_, input, dst, do_two);
}

// upstream: src/dsp/enc.c VP8FTransform
/// C: the `VP8FTransform` pointer.
pub fn vp8f_transform(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    f_transform_c(src, ref_, out);
}

// upstream: src/dsp/enc.c VP8FTransform2
/// Performs two transforms at a time (C: the `VP8FTransform2` pointer).
pub fn vp8f_transform2(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    f_transform2_c(src, ref_, out);
}

// upstream: src/dsp/enc.c VP8FTransformWHT
/// C: the `VP8FTransformWHT` pointer.
pub fn vp8f_transform_wht(input: &[i16], out: &mut [i16]) {
    f_transform_wht_c(input, out);
}

// upstream: src/dsp/enc.c VP8EncPredLuma4
/// The ten intra4 predictions of the block whose boundary `top[0]` is at
/// position `top_pos` of `top` (C: the `VP8EncPredLuma4` pointer).
pub fn vp8_enc_pred_luma4(dst: &mut [u8], top: &[u8], top_pos: usize) {
    intra4_preds_c(dst, top, top_pos);
}

// upstream: src/dsp/enc.c VP8EncPredLuma16
/// The four intra16 predictions; `left` and `top` are `None` where C passes
/// `NULL` (C: the `VP8EncPredLuma16` pointer).
pub fn vp8_enc_pred_luma16(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    intra16_preds_c(dst, left, top);
}

// upstream: src/dsp/enc.c VP8EncPredChroma8
/// The four chroma predictions of U and V (C: the `VP8EncPredChroma8` pointer).
pub fn vp8_enc_pred_chroma8(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    intra_chroma_preds_c(dst, left, top);
}

// upstream: src/dsp/enc.c VP8SSE16x16
#[must_use]
pub fn vp8sse16x16(a: &[u8], b: &[u8]) -> i32 {
    sse16x16_c(a, b)
}

// upstream: src/dsp/enc.c VP8SSE16x8
#[must_use]
pub fn vp8sse16x8(a: &[u8], b: &[u8]) -> i32 {
    sse16x8_c(a, b)
}

// upstream: src/dsp/enc.c VP8SSE8x8
#[must_use]
pub fn vp8sse8x8(a: &[u8], b: &[u8]) -> i32 {
    sse8x8_c(a, b)
}

// upstream: src/dsp/enc.c VP8SSE4x4
#[must_use]
pub fn vp8sse4x4(a: &[u8], b: &[u8]) -> i32 {
    sse4x4_c(a, b)
}

// upstream: src/dsp/enc.c VP8TDisto4x4
/// The weights contain a row-major 4 by 4 symmetric matrix.
#[must_use]
pub fn vp8t_disto4x4(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    disto4x4_c(a, b, w)
}

// upstream: src/dsp/enc.c VP8TDisto16x16
#[must_use]
pub fn vp8t_disto16x16(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    disto16x16_c(a, b, w)
}

// upstream: src/dsp/enc.c VP8Mean16x4
/// Compute the average (DC) of four 4x4 blocks. Each sub-4x4 block #i sum is
/// stored in `dc[i]`.
pub fn vp8_mean16x4(ref_: &[u8], dc: &mut [u32]) {
    mean16x4_c(ref_, dc);
}

// upstream: src/dsp/enc.c VP8EncQuantizeBlock
/// Quantizes one block (C: the `VP8EncQuantizeBlock` pointer).
pub fn vp8_enc_quantize_block(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    quantize_block_c(input, out, mtx)
}

// upstream: src/dsp/enc.c VP8EncQuantize2Blocks
/// Same as `VP8EncQuantizeBlock`, but quantizes two consecutive blocks.
pub fn vp8_enc_quantize2_blocks(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    quantize2_blocks_c(input, out, mtx)
}

// upstream: src/dsp/enc.c VP8EncQuantizeBlockWHT
/// Specific to 2nd transform; C assigns `QuantizeBlock_C` too.
pub fn vp8_enc_quantize_block_wht(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    quantize_block_c(input, out, mtx)
}

// upstream: src/dsp/enc.c VP8Copy4x4
pub fn vp8_copy4x4(src: &[u8], dst: &mut [u8]) {
    copy4x4_c(src, dst);
}

// upstream: src/dsp/enc.c VP8Copy16x8
pub fn vp8_copy16x8(src: &[u8], dst: &mut [u8]) {
    copy16x8_c(src, dst);
}

// upstream: src/dsp/enc.c VP8EncDspInit (no counterpart: it calls VP8DspInit, fills
// clip1 through InitTables, which CLIP1 computes on first use, and assigns
// portable pointers, ADR 0003)
