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
//! Exception: WP12 translates `WebPInitDecBuffer`, `WebPGetFeatures`, `WebPInitDecoderConfig` and `WebPIDecGetYUV` from `src/webp/decode.h`, because each one calls an entry point that WP12 translates from `src/dec/buffer_dec.c`, `src/dec/webp_dec.c` or `src/dec/idec_dec.c`.
//!
//! The output buffers borrow external memory and own internal memory through
//! one storage per buffer (ADR 0003, decided 2026-11-06). The lifetime `'a`
//! reaches `WebPDecParams`, `WebPIDecoder` and the safe decoder API (WP12, WP16).

use crate::webp::types::{PlaneOffset, PlaneStorage};

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

// upstream: src/webp/decode.h WebPRGBABuffer
/// View of a buffer as RGBA samples (C: `WebPRGBABuffer`). The plane is an offset
/// into the buffer's storage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPRGBABuffer {
    /// Start of the samples (C: `rgba`).
    pub rgba: PlaneOffset,
    /// Bytes from one scanline to the next (C: `stride`).
    pub stride: i32,
    /// Total size of the samples in bytes (C: `size`).
    pub size: usize,
}

// upstream: src/webp/decode.h WebPYUVABuffer
/// View of a buffer as YUVA samples (C: `WebPYUVABuffer`). The planes are offsets
/// into the buffer's storage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPYUVABuffer {
    /// Luma samples (C: `y`).
    pub y: PlaneOffset,
    /// Chroma U samples (C: `u`).
    pub u: PlaneOffset,
    /// Chroma V samples (C: `v`).
    pub v: PlaneOffset,
    /// Alpha samples; `None` when the buffer has no alpha plane (C: `a == NULL`).
    pub a: Option<PlaneOffset>,
    /// Luma stride (C: `y_stride`).
    pub y_stride: i32,
    /// Chroma U stride (C: `u_stride`).
    pub u_stride: i32,
    /// Chroma V stride (C: `v_stride`).
    pub v_stride: i32,
    /// Alpha stride (C: `a_stride`).
    pub a_stride: i32,
    /// Luma plane size (C: `y_size`).
    pub y_size: usize,
    /// Chroma U plane size (C: `u_size`).
    pub u_size: usize,
    /// Chroma V plane size (C: `v_size`).
    pub v_size: usize,
    /// Alpha plane size (C: `a_size`).
    pub a_size: usize,
}

/// The `u` member of `WebPDecBuffer`, which its colorspace selects (ADR 0003: the
/// union becomes an enum with one variant per member). The name is Rust-only,
/// because the C union is anonymous. The variant must match the colorspace, as the
/// C member does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebPDecBufferPlanes {
    /// The RGBA or ARGB member, for the RGB colorspaces.
    RGBA(WebPRGBABuffer),
    /// The YUVA member, for `MODE_YUV` and `MODE_YUVA`.
    YUVA(WebPYUVABuffer),
}

impl Default for WebPDecBufferPlanes {
    fn default() -> Self {
        Self::RGBA(WebPRGBABuffer::default())
    }
}

// upstream: src/webp/decode.h WebPDecBuffer
/// Generic structure describing the output sample buffer (C: `WebPDecBuffer`).
///
/// `storage` is C's `private_memory` when the object allocated the samples, and
/// the caller's slices when C would set `is_external_memory`. The C field
/// `is_external_memory` is therefore the variant of `storage`, and its values 2
/// and more, which mark slow external memory, are `slow_memory` (ADR 0003,
/// amended 2026-11-07, WP12).
#[derive(Debug)]
pub struct WebPDecBuffer<'a> {
    /// Colorspace of the samples (C: `colorspace`).
    pub colorspace: WEBP_CSP_MODE,
    /// Width in pixels (C: `width`).
    pub width: i32,
    /// Height in pixels (C: `height`).
    pub height: i32,
    /// Slow external memory (C: `is_external_memory >= 2`): the decoder avoids
    /// multiple reads and writes of it. Meaningful only with `External`
    /// storage, and false for the other storages.
    pub slow_memory: bool,
    /// The planes of the buffer, selected by `colorspace` (C: `u`).
    pub u: WebPDecBufferPlanes,
    /// Padding for later use.
    pub pad: [u32; 4],
    /// The memory that holds the samples (C: `private_memory`, or the caller's memory).
    pub storage: PlaneStorage<'a, u8>,
}

impl Default for WebPDecBuffer<'_> {
    /// An empty buffer in `MODE_RGB` (C: `WebPInitDecBuffer` zeroes the structure).
    fn default() -> Self {
        Self {
            colorspace: WEBP_CSP_MODE::MODE_RGB,
            width: 0,
            height: 0,
            slow_memory: false,
            u: WebPDecBufferPlanes::default(),
            pad: [0; 4],
            storage: PlaneStorage::default(),
        }
    }
}

// upstream: src/webp/decode.h WebPDecoderConfig
/// Main object storing the configuration for advanced decoding (C: `WebPDecoderConfig`).
#[derive(Debug, Default)]
pub struct WebPDecoderConfig<'a> {
    /// Immutable bitstream features (C: `input`).
    pub input: WebPBitstreamFeatures,
    /// Output buffer, which may borrow the caller's memory (C: `output`).
    pub output: WebPDecBuffer<'a>,
    /// Decoding options (C: `options`).
    pub options: WebPDecoderOptions,
}

// The items below are WP12's exception in this module (`tools/units.json`): the
// header's inline wrappers, each of which calls an entry point that WP12
// translates.

use crate::dec::buffer_dec::webp_init_dec_buffer_internal;
use crate::dec::webp_dec::{webp_get_features_internal, webp_init_decoder_config_internal};

// upstream: src/webp/decode.h WebPInitDecBuffer
/// Initializes the structure as empty. Must be called before any other use.
/// Returns false in case of version mismatch.
#[must_use]
pub fn webp_init_dec_buffer(buffer: &mut WebPDecBuffer<'_>) -> bool {
    webp_init_dec_buffer_internal(buffer, WEBP_DECODER_ABI_VERSION)
}

// upstream: src/webp/decode.h WebPGetFeatures
/// Retrieves features from the bitstream. The `features` structure is filled
/// with information gathered from the bitstream.
///
/// # Errors
///
/// `VP8_STATUS_NOT_ENOUGH_DATA` when more data is needed to retrieve the
/// bitstream features (for instance, the input is truncated), and the other
/// status codes of the header parsing otherwise.
pub fn webp_get_features(
    data: &[u8],
    features: &mut WebPBitstreamFeatures,
) -> Result<(), VP8StatusCode> {
    webp_get_features_internal(data, features, WEBP_DECODER_ABI_VERSION)
}

// upstream: src/webp/decode.h WebPInitDecoderConfig
/// Initializes the configuration as empty. This function must always be called
/// first, unless `WebPGetFeatures()` is to be called. Returns false in case of
/// mismatched version.
#[must_use]
pub fn webp_init_decoder_config(config: &mut WebPDecoderConfig<'_>) -> bool {
    webp_init_decoder_config_internal(config, WEBP_DECODER_ABI_VERSION)
}

// upstream: src/webp/decode.h WebPIDecGetYUV
/// Deprecated alpha-less version of `WebPIDecGetYUVA()`: same, ignoring the
/// alpha information.
#[expect(
    clippy::too_many_arguments,
    reason = "the output parameters of the C function"
)]
#[must_use]
pub fn webp_i_dec_get_yuv<'s>(
    idec: &'s crate::dec::idec_dec::WebPIDecoder<'_, '_>,
    last_y: Option<&mut i32>,
    u: Option<&mut crate::dec::idec_dec::PlaneRows<'s>>,
    v: Option<&mut crate::dec::idec_dec::PlaneRows<'s>>,
    width: Option<&mut i32>,
    height: Option<&mut i32>,
    stride: Option<&mut i32>,
    uv_stride: Option<&mut i32>,
) -> crate::dec::idec_dec::PlaneRows<'s> {
    crate::dec::idec_dec::webp_i_dec_get_yuva(
        idec, last_y, u, v, None, width, height, stride, uv_stride, None,
    )
}
