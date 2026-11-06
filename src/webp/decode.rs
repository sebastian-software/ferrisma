// libwebp v1.6.0, src/webp/decode.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  Main decoding functions for WebP images.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/webp/decode.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! Not translated yet, pending open questions in `docs/status/wp1.md`: the
//! output buffer structs `WebPRGBABuffer`, `WebPYUVABuffer`, `WebPDecBuffer`
//! and `WebPDecoderConfig` (question 1), and the inline entry points that call
//! `*Internal` functions of later packages (question 5).

// upstream: src/webp/decode.h WEBP_DECODER_ABI_VERSION
pub const WEBP_DECODER_ABI_VERSION: i32 = 0x0210;

// upstream: src/webp/decode.h VP8StatusCode
/// Status codes of the decoder (C: `VP8StatusCode`), keeping their C values.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VP8StatusCode(pub u32);

impl VP8StatusCode {
    // upstream: src/webp/decode.h VP8_STATUS_OK
    pub const VP8_STATUS_OK: Self = Self(0);
    // upstream: src/webp/decode.h VP8_STATUS_OUT_OF_MEMORY
    pub const VP8_STATUS_OUT_OF_MEMORY: Self = Self(1);
    // upstream: src/webp/decode.h VP8_STATUS_INVALID_PARAM
    pub const VP8_STATUS_INVALID_PARAM: Self = Self(2);
    // upstream: src/webp/decode.h VP8_STATUS_BITSTREAM_ERROR
    pub const VP8_STATUS_BITSTREAM_ERROR: Self = Self(3);
    // upstream: src/webp/decode.h VP8_STATUS_UNSUPPORTED_FEATURE
    pub const VP8_STATUS_UNSUPPORTED_FEATURE: Self = Self(4);
    // upstream: src/webp/decode.h VP8_STATUS_SUSPENDED
    pub const VP8_STATUS_SUSPENDED: Self = Self(5);
    // upstream: src/webp/decode.h VP8_STATUS_USER_ABORT
    pub const VP8_STATUS_USER_ABORT: Self = Self(6);
    // upstream: src/webp/decode.h VP8_STATUS_NOT_ENOUGH_DATA
    pub const VP8_STATUS_NOT_ENOUGH_DATA: Self = Self(7);
}

// upstream: src/webp/decode.h WEBP_CSP_MODE
/// Output colorspaces (C: `WEBP_CSP_MODE`); names describe the byte order of packed samples.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WEBP_CSP_MODE(pub u32);

#[expect(
    non_upper_case_globals,
    reason = "C enumerator names such as MODE_rgbA keep their case"
)]
impl WEBP_CSP_MODE {
    // upstream: src/webp/decode.h MODE_RGB
    pub const MODE_RGB: Self = Self(0);
    // upstream: src/webp/decode.h MODE_RGBA
    pub const MODE_RGBA: Self = Self(1);
    // upstream: src/webp/decode.h MODE_BGR
    pub const MODE_BGR: Self = Self(2);
    // upstream: src/webp/decode.h MODE_BGRA
    pub const MODE_BGRA: Self = Self(3);
    // upstream: src/webp/decode.h MODE_ARGB
    pub const MODE_ARGB: Self = Self(4);
    // upstream: src/webp/decode.h MODE_RGBA_4444
    pub const MODE_RGBA_4444: Self = Self(5);
    // upstream: src/webp/decode.h MODE_RGB_565
    pub const MODE_RGB_565: Self = Self(6);
    // upstream: src/webp/decode.h MODE_rgbA
    pub const MODE_rgbA: Self = Self(7);
    // upstream: src/webp/decode.h MODE_bgrA
    pub const MODE_bgrA: Self = Self(8);
    // upstream: src/webp/decode.h MODE_Argb
    pub const MODE_Argb: Self = Self(9);
    // upstream: src/webp/decode.h MODE_rgbA_4444
    pub const MODE_rgbA_4444: Self = Self(10);
    // upstream: src/webp/decode.h MODE_YUV
    pub const MODE_YUV: Self = Self(11);
    // upstream: src/webp/decode.h MODE_YUVA
    pub const MODE_YUVA: Self = Self(12);
    // upstream: src/webp/decode.h MODE_LAST
    pub const MODE_LAST: Self = Self(13);
}

// upstream: src/webp/decode.h WebPIsPremultipliedMode
#[must_use]
pub fn webp_is_premultiplied_mode(mode: WEBP_CSP_MODE) -> bool {
    mode == WEBP_CSP_MODE::MODE_rgbA
        || mode == WEBP_CSP_MODE::MODE_bgrA
        || mode == WEBP_CSP_MODE::MODE_Argb
        || mode == WEBP_CSP_MODE::MODE_rgbA_4444
}

// upstream: src/webp/decode.h WebPIsAlphaMode
#[must_use]
pub fn webp_is_alpha_mode(mode: WEBP_CSP_MODE) -> bool {
    mode == WEBP_CSP_MODE::MODE_RGBA
        || mode == WEBP_CSP_MODE::MODE_BGRA
        || mode == WEBP_CSP_MODE::MODE_ARGB
        || mode == WEBP_CSP_MODE::MODE_RGBA_4444
        || mode == WEBP_CSP_MODE::MODE_YUVA
        || webp_is_premultiplied_mode(mode)
}

// upstream: src/webp/decode.h WebPIsRGBMode
#[must_use]
pub fn webp_is_rgb_mode(mode: WEBP_CSP_MODE) -> bool {
    mode < WEBP_CSP_MODE::MODE_YUV
}

// upstream: src/webp/decode.h WebPBitstreamFeatures
/// Features gathered from the bitstream (C: `WebPBitstreamFeatures`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPBitstreamFeatures {
    /// Width in pixels, as read from the bitstream.
    pub width: i32,
    /// Height in pixels, as read from the bitstream.
    pub height: i32,
    /// Non-zero if the bitstream contains an alpha channel.
    pub has_alpha: i32,
    /// Non-zero if the bitstream is an animation.
    pub has_animation: i32,
    /// 0 = undefined (mixed), 1 = lossy, 2 = lossless.
    pub format: i32,
    /// Padding for later use.
    pub pad: [u32; 5],
}

// upstream: src/webp/decode.h WebPDecoderOptions
/// Decoding options (C: `WebPDecoderOptions`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPDecoderOptions {
    /// Non-zero skips the in-loop filtering.
    pub bypass_filtering: i32,
    /// Non-zero uses the faster pointwise upsampler.
    pub no_fancy_upsampling: i32,
    /// Non-zero applies cropping first.
    pub use_cropping: i32,
    /// Top-left position for cropping; snapped to even values.
    pub crop_left: i32,
    /// Top-left position for cropping; snapped to even values.
    pub crop_top: i32,
    /// Dimensions of the cropping area.
    pub crop_width: i32,
    /// Dimensions of the cropping area.
    pub crop_height: i32,
    /// Non-zero applies scaling afterward.
    pub use_scaling: i32,
    /// Final resolution; if one is 0 it is derived from the other to keep the ratio.
    pub scaled_width: i32,
    /// Final resolution; if one is 0 it is derived from the other to keep the ratio.
    pub scaled_height: i32,
    /// Non-zero uses multi-threaded decoding.
    pub use_threads: i32,
    /// Dithering strength, 0 (off) to 100 (full).
    pub dithering_strength: i32,
    /// Non-zero flips the output vertically.
    pub flip: i32,
    /// Alpha dithering strength in [0..100].
    pub alpha_dithering_strength: i32,
    /// Padding for later use.
    pub pad: [u32; 5],
}
