// libwebp v1.6.0, src/dsp/lossless_common.h:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Image transforms and color space conversion methods for lossless decoder.
//
// Authors: Vikas Arora (vikaas.arora@gmail.com)
//          Jyrki Alakuijala (jyrki@google.com)
//          Vincent Rabaud (vrabaud@google.com)

//! Translation of `src/dsp/lossless_common.h` from libwebp v1.6.0.
//!
//! Work package: WP3 (lossless shared).
//!
//! Not translated here, with the reason: `VP8LFastLog2`, `VP8LFastSLog2`,
//! `VP8LPrefixEncodeBits` and `VP8LPrefixEncode` read `kLog2Table`,
//! `kSLog2Table`, `VP8LFastLog2Slow`, `VP8LFastSLog2Slow`, `kPrefixEncodeCode`
//! and `kPrefixEncodeExtraBitsValue`, which `src/dsp/lossless_enc.c` defines
//! (WP11). They stay absent until that package's tables exist
//! (`docs/status/wp3.md`).

use crate::utils::utils::bits_log2_floor;

//------------------------------------------------------------------------------
// Decoding

// upstream: src/dsp/lossless_common.h VP8GetARGBIndex
/// Color mapping related functions.
#[must_use]
#[inline]
pub fn vp8_get_argb_index(idx: u32) -> u32 {
    (idx >> 8) & 0xff
}

// upstream: src/dsp/lossless_common.h VP8GetAlphaIndex
#[must_use]
#[inline]
pub fn vp8_get_alpha_index(idx: u8) -> u8 {
    idx
}

// upstream: src/dsp/lossless_common.h VP8GetARGBValue
#[must_use]
#[inline]
pub fn vp8_get_argb_value(val: u32) -> u32 {
    val
}

// upstream: src/dsp/lossless_common.h VP8GetAlphaValue
#[must_use]
#[inline]
pub fn vp8_get_alpha_value(val: u32) -> u8 {
    ((val >> 8) & 0xff) as u8
}

//------------------------------------------------------------------------------
// Misc methods.

// upstream: src/dsp/lossless_common.h VP8LSubSampleSize
/// Computes sampled size of `size` when sampling using `sampling_bits`.
/// The `uint32_t` sum wraps, as in C.
#[must_use]
#[inline]
pub fn vp8l_sub_sample_size(size: u32, sampling_bits: u32) -> u32 {
    size.wrapping_add(1 << sampling_bits).wrapping_sub(1) >> sampling_bits
}

// upstream: src/dsp/lossless_common.h VP8LNearLosslessBits
/// Converts near lossless quality into max number of bits shaved off.
#[must_use]
pub fn vp8l_near_lossless_bits(near_lossless_quality: i32) -> i32 {
    //    100 -> 0
    // 80..99 -> 1
    // 60..79 -> 2
    // 40..59 -> 3
    // 20..39 -> 4
    //  0..19 -> 5
    5 - near_lossless_quality / 20
}

// -----------------------------------------------------------------------------
// Faster logarithm for integers. Small values use a look-up table.

// upstream: src/dsp/lossless_common.h APPROX_LOG_WITH_CORRECTION_MAX
/// The threshold till approximate version of `log_2` can be used.
/// Practically, we can get rid of the call to `log()` as the two values match
/// to very high degree (the ratio of these two is 0.99999x).
/// Keeping a high threshold for now.
pub const APPROX_LOG_WITH_CORRECTION_MAX: i32 = 65536;
// upstream: src/dsp/lossless_common.h APPROX_LOG_MAX
pub const APPROX_LOG_MAX: i32 = 4096;
// upstream: src/dsp/lossless_common.h LOG_2_PRECISION_BITS
/// `VP8LFastLog2` and `VP8LFastSLog2` are used on elements from image
/// histograms. The histogram values cannot exceed the maximum number of
/// pixels, which is (1 << 14) * (1 << 14). Therefore S * log(S) < (1 << 33).
/// No more than 32 bits of precision should be chosen.
/// To match the original float implementation, 23 bits of precision are used.
pub const LOG_2_PRECISION_BITS: i32 = 23;
// upstream: src/dsp/lossless_common.h LOG_2_RECIPROCAL
#[expect(clippy::approx_constant, reason = "the C literal of LOG_2_RECIPROCAL")]
pub const LOG_2_RECIPROCAL: f64 = 1.442_695_040_888_963_387_004_650_940_070_86;
// upstream: src/dsp/lossless_common.h LOG_2_RECIPROCAL_FIXED_DOUBLE
/// `LOG_2_RECIPROCAL * (1 << LOG_2_PRECISION_BITS)`
pub const LOG_2_RECIPROCAL_FIXED_DOUBLE: f64 = 12_102_203.161_561_485_379_934_310_913_085_937_500;
// upstream: src/dsp/lossless_common.h LOG_2_RECIPROCAL_FIXED
pub const LOG_2_RECIPROCAL_FIXED: u64 = 12_102_203;
// upstream: src/dsp/lossless_common.h LOG_LOOKUP_IDX_MAX
pub const LOG_LOOKUP_IDX_MAX: i32 = 256;

// upstream: src/dsp/lossless_common.h RightShiftRound
/// `v` shifted right by `shift`, rounded to nearest. The `uint64_t` sum wraps,
/// as in C.
#[must_use]
#[inline]
pub fn right_shift_round(v: u64, shift: u32) -> u64 {
    v.wrapping_add(1_u64 << shift >> 1) >> shift
}

// upstream: src/dsp/lossless_common.h DivRound
/// `a / b`, rounded half away from zero.
#[must_use]
#[inline]
pub fn div_round(a: i64, b: i64) -> i64 {
    if (a < 0) == (b < 0) {
        (a + b / 2) / b
    } else {
        (a - b / 2) / b
    }
}

// upstream: src/dsp/lossless_common.h WEBP_INT64_MAX
pub const WEBP_INT64_MAX: i64 = ((1_u64 << 63) - 1).cast_signed();
// upstream: src/dsp/lossless_common.h WEBP_UINT64_MAX
pub const WEBP_UINT64_MAX: u64 = !0_u64;

// -----------------------------------------------------------------------------
// PrefixEncode()

// upstream: src/dsp/lossless_common.h VP8LPrefixEncodeBitsNoLUT
/// Splitting of distance and length codes into prefixes and extra bits. The
/// prefixes are encoded with an entropy code while the extra bits are stored
/// just as normal bits.
///
/// Returns `(code, extra_bits)`, the two out-parameters of the C function.
/// `distance` must be at least 3; C shifts by a negative amount below that.
#[must_use]
#[inline]
pub fn vp8l_prefix_encode_bits_no_lut(mut distance: i32) -> (i32, i32) {
    distance -= 1;
    let highest_bit = bits_log2_floor(distance.cast_unsigned());
    let second_highest_bit = (distance >> (highest_bit - 1)) & 1;
    let extra_bits = highest_bit - 1;
    let code = 2 * highest_bit + second_highest_bit;
    (code, extra_bits)
}

// upstream: src/dsp/lossless_common.h VP8LPrefixEncodeNoLUT
/// Returns `(code, extra_bits, extra_bits_value)`, the three out-parameters of
/// the C function. `distance` must be at least 3.
#[must_use]
#[inline]
pub fn vp8l_prefix_encode_no_lut(mut distance: i32) -> (i32, i32, i32) {
    distance -= 1;
    let highest_bit = bits_log2_floor(distance.cast_unsigned());
    let second_highest_bit = (distance >> (highest_bit - 1)) & 1;
    let extra_bits = highest_bit - 1;
    let extra_bits_value = distance & ((1 << extra_bits) - 1);
    let code = 2 * highest_bit + second_highest_bit;
    (code, extra_bits, extra_bits_value)
}

// upstream: src/dsp/lossless_common.h PREFIX_LOOKUP_IDX_MAX
pub const PREFIX_LOOKUP_IDX_MAX: i32 = 512;

// upstream: src/dsp/lossless_common.h VP8LPrefixCode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VP8LPrefixCode {
    pub code: i8,
    pub extra_bits: i8,
}

// upstream: src/dsp/lossless_common.h VP8LAddPixels
/// Sum of each component, mod 256.
#[must_use]
#[inline]
pub fn vp8l_add_pixels(a: u32, b: u32) -> u32 {
    let alpha_and_green = (a & 0xff00_ff00).wrapping_add(b & 0xff00_ff00);
    let red_and_blue = (a & 0x00ff_00ff).wrapping_add(b & 0x00ff_00ff);
    (alpha_and_green & 0xff00_ff00) | (red_and_blue & 0x00ff_00ff)
}

// upstream: src/dsp/lossless_common.h VP8LSubPixels
/// Difference of each component, mod 256.
#[must_use]
#[inline]
pub fn vp8l_sub_pixels(a: u32, b: u32) -> u32 {
    let alpha_and_green = 0x00ff_00ff_u32
        .wrapping_add(a & 0xff00_ff00)
        .wrapping_sub(b & 0xff00_ff00);
    let red_and_blue = 0xff00_ff00_u32
        .wrapping_add(a & 0x00ff_00ff)
        .wrapping_sub(b & 0x00ff_00ff);
    (alpha_and_green & 0xff00_ff00) | (red_and_blue & 0x00ff_00ff)
}

//------------------------------------------------------------------------------
// Transform-related functions used in both encoding and decoding.

// upstream: src/dsp/lossless_common.h GENERATE_PREDICTOR_ADD
/// Macros used to create a batch predictor that iteratively uses a one-pixel
/// predictor.
///
/// The predictor is added to the output pixel (which is therefore considered
/// as a residual) to get the final prediction. The generated function has the
/// form of `VP8LPredictorAddSubFunc` in `src/dsp/lossless.rs`: `in`, `upper`
/// and `out` are positions in one buffer `data`, because the decoder reads the
/// row above the output, and transforms in place.
macro_rules! generate_predictor_add {
    ($predictor:path, $predictor_add:ident) => {
        fn $predictor_add(
            data: &mut [u32],
            in_pos: usize,
            upper_pos: usize,
            num_pixels: i32,
            out_pos: usize,
        ) {
            for x in 0..::core::primitive::usize::try_from(num_pixels).unwrap_or(0) {
                let pred = $predictor(data, (out_pos + x).wrapping_sub(1), data, upper_pos + x);
                data[out_pos + x] =
                    $crate::dsp::lossless_common::vp8l_add_pixels(data[in_pos + x], pred);
            }
        }
    };
}
pub(crate) use generate_predictor_add;
