// libwebp v1.6.0, src/webp/encode.h:
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
//   WebP encoder: main interface
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/webp/encode.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! Exception: WP9 translates `WebPConfigInit`, `WebPConfigPreset` and `WebPPictureInit` from `src/webp/encode.h`, because each one calls an entry point that WP9 translates from `src/enc/config_enc.c` or `src/enc/picture_enc.c`.
//!
//! The picture keeps its samples in storages with the lifetime `'a`, as the decoder
//! buffers do (ADR 0003, decided 2026-11-06). Its writer and progress hook are
//! closures that own their context, which replaces `custom_ptr` and `user_data`.
//! The inline initializers `WebPConfigInit`, `WebPConfigPreset` and
//! `WebPPictureInit` are WP9's (exceptions in `tools/units.json`).

use crate::webp::types::{PlaneOffset, PlaneStorage};

// upstream: src/webp/encode.h WEBP_ENCODER_ABI_VERSION
pub const WEBP_ENCODER_ABI_VERSION: i32 = 0x0210;

// upstream: src/webp/encode.h WEBP_MAX_DIMENSION
/// Largest width and height allowed for a picture, in pixels (inclusive).
pub const WEBP_MAX_DIMENSION: i32 = 16383;

// upstream: src/webp/encode.h WebPImageHint
/// Image characteristics hint for the encoder (C: `WebPImageHint`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPImageHint(pub u32);

impl WebPImageHint {
    // upstream: src/webp/encode.h WEBP_HINT_DEFAULT
    pub const WEBP_HINT_DEFAULT: Self = Self(0);
    // upstream: src/webp/encode.h WEBP_HINT_PICTURE
    pub const WEBP_HINT_PICTURE: Self = Self(1);
    // upstream: src/webp/encode.h WEBP_HINT_PHOTO
    pub const WEBP_HINT_PHOTO: Self = Self(2);
    // upstream: src/webp/encode.h WEBP_HINT_GRAPH
    pub const WEBP_HINT_GRAPH: Self = Self(3);
    // upstream: src/webp/encode.h WEBP_HINT_LAST
    pub const WEBP_HINT_LAST: Self = Self(4);
}

// upstream: src/webp/encode.h WebPConfig
/// Compression parameters (C: `WebPConfig`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebPConfig {
    /// Lossless encoding: 0 = lossy (default), 1 = lossless.
    pub lossless: i32,
    /// Between 0 and 100; the meaning depends on `lossless`.
    pub quality: f32,
    /// Quality/speed trade-off: 0 = fast, 6 = slower and better.
    pub method: i32,
    /// Hint for the image type (lossless only for now).
    pub image_hint: WebPImageHint,
    /// If non-zero, the desired target size in bytes.
    pub target_size: i32,
    /// If non-zero, the minimal distortion to try to achieve.
    pub target_psnr: f32,
    /// Maximum number of segments to use, in [1..4].
    pub segments: i32,
    /// Spatial noise shaping: 0 = off, 100 = maximum.
    pub sns_strength: i32,
    /// Filter strength, in [0..100].
    pub filter_strength: i32,
    /// Filter sharpness, in [0..7].
    pub filter_sharpness: i32,
    /// Filter type: 0 = simple, 1 = strong.
    pub filter_type: i32,
    /// Automatic adjustment of the filter strength: 0 = off, 1 = on.
    pub autofilter: i32,
    /// Alpha plane compression: 0 = none, 1 = lossless.
    pub alpha_compression: i32,
    /// Predictive filtering of the alpha plane: 0 = none, 1 = fast, 2 = best.
    pub alpha_filtering: i32,
    /// Alpha quality, between 0 (smallest) and 100 (lossless).
    pub alpha_quality: i32,
    /// Number of entropy-analysis passes, in [1..10].
    pub pass: i32,
    /// Non-zero exports the compressed picture back.
    pub show_compressed: i32,
    /// Preprocessing: 0 = none, 1 = segment smoothing, 2 = dithering.
    pub preprocessing: i32,
    /// Log2 of the number of token partitions, in [0..3].
    pub partitions: i32,
    /// Quality degradation allowed to fit the 512k limit, in [0..100].
    pub partition_limit: i32,
    /// Non-zero remaps the parameters to match the JPEG output size.
    pub emulate_jpeg_size: i32,
    /// Non-zero tries multi-threaded encoding.
    pub thread_level: i32,
    /// Non-zero reduces memory use at the cost of CPU.
    pub low_memory: i32,
    /// Near-lossless encoding, [0 = max loss .. 100 = off].
    pub near_lossless: i32,
    /// Non-zero preserves the RGB values under transparent areas.
    pub exact: i32,
    /// Reserved.
    pub use_delta_palette: i32,
    /// Non-zero uses sharp (and slow) RGB to YUV conversion.
    pub use_sharp_yuv: i32,
    /// Minimum permissible quality factor.
    pub qmin: i32,
    /// Maximum permissible quality factor.
    pub qmax: i32,
}

// upstream: src/webp/encode.h WebPPreset
/// Predefined configuration sets (C: `WebPPreset`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPPreset(pub u32);

impl WebPPreset {
    // upstream: src/webp/encode.h WEBP_PRESET_DEFAULT
    pub const WEBP_PRESET_DEFAULT: Self = Self(0);
    // upstream: src/webp/encode.h WEBP_PRESET_PICTURE
    pub const WEBP_PRESET_PICTURE: Self = Self(1);
    // upstream: src/webp/encode.h WEBP_PRESET_PHOTO
    pub const WEBP_PRESET_PHOTO: Self = Self(2);
    // upstream: src/webp/encode.h WEBP_PRESET_DRAWING
    pub const WEBP_PRESET_DRAWING: Self = Self(3);
    // upstream: src/webp/encode.h WEBP_PRESET_ICON
    pub const WEBP_PRESET_ICON: Self = Self(4);
    // upstream: src/webp/encode.h WEBP_PRESET_TEXT
    pub const WEBP_PRESET_TEXT: Self = Self(5);
}

// upstream: src/webp/encode.h WebPAuxStats
/// Auxiliary statistics of an encoding (C: `WebPAuxStats`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WebPAuxStats {
    /// Final size.
    pub coded_size: i32,
    /// Peak signal-to-noise ratio for Y, U, V, All and Alpha.
    pub psnr: [f32; 5],
    /// Number of intra4, intra16 and skipped macroblocks.
    pub block_count: [i32; 3],
    /// Approximate bytes spent on the header and mode partition #0.
    pub header_bytes: [i32; 2],
    /// Approximate bytes for DC, AC and UV coefficients, per segment.
    pub residual_bytes: [[i32; 4]; 3],
    /// Number of macroblocks in each segment.
    pub segment_size: [i32; 4],
    /// Quantizer value of each segment.
    pub segment_quant: [i32; 4],
    /// Filtering strength of each segment, in [0..63].
    pub segment_level: [i32; 4],
    /// Size of the transparency data.
    pub alpha_data_size: i32,
    /// Size of the enhancement layer data.
    pub layer_data_size: i32,
    /// Lossless features: bit 0 predictor, bit 1 cross-color, bit 2 subtract-green, bit 3 color indexing.
    pub lossless_features: u32,
    /// Precision bits of the histogram.
    pub histogram_bits: i32,
    /// Precision bits of the predictor transform.
    pub transform_bits: i32,
    /// Number of bits of the color cache lookup.
    pub cache_bits: i32,
    /// Number of colors in the palette, if used.
    pub palette_size: i32,
    /// Final lossless size.
    pub lossless_size: i32,
    /// Size of the lossless header (transforms, Huffman codes).
    pub lossless_hdr_size: i32,
    /// Size of the lossless image data.
    pub lossless_data_size: i32,
    /// Precision bits of the cross-color transform.
    pub cross_color_transform_bits: i32,
    /// Padding for later use.
    pub pad: [u32; 1],
}

// upstream: src/webp/encode.h WebPEncCSP
/// Color spaces of the encoder input (C: `WebPEncCSP`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPEncCSP(pub u32);

impl WebPEncCSP {
    // upstream: src/webp/encode.h WEBP_YUV420
    pub const WEBP_YUV420: Self = Self(0);
    // upstream: src/webp/encode.h WEBP_YUV420A
    pub const WEBP_YUV420A: Self = Self(4);
    // upstream: src/webp/encode.h WEBP_CSP_UV_MASK
    pub const WEBP_CSP_UV_MASK: Self = Self(3);
    // upstream: src/webp/encode.h WEBP_CSP_ALPHA_BIT
    pub const WEBP_CSP_ALPHA_BIT: Self = Self(4);
}

// upstream: src/webp/encode.h WebPEncodingError
/// Encoding error conditions (C: `WebPEncodingError`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPEncodingError(pub u32);

impl WebPEncodingError {
    // upstream: src/webp/encode.h VP8_ENC_OK
    pub const VP8_ENC_OK: Self = Self(0);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_OUT_OF_MEMORY
    pub const VP8_ENC_ERROR_OUT_OF_MEMORY: Self = Self(1);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY
    pub const VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY: Self = Self(2);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_NULL_PARAMETER
    pub const VP8_ENC_ERROR_NULL_PARAMETER: Self = Self(3);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_INVALID_CONFIGURATION
    pub const VP8_ENC_ERROR_INVALID_CONFIGURATION: Self = Self(4);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_BAD_DIMENSION
    pub const VP8_ENC_ERROR_BAD_DIMENSION: Self = Self(5);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_PARTITION0_OVERFLOW
    pub const VP8_ENC_ERROR_PARTITION0_OVERFLOW: Self = Self(6);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_PARTITION_OVERFLOW
    pub const VP8_ENC_ERROR_PARTITION_OVERFLOW: Self = Self(7);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_BAD_WRITE
    pub const VP8_ENC_ERROR_BAD_WRITE: Self = Self(8);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_FILE_TOO_BIG
    pub const VP8_ENC_ERROR_FILE_TOO_BIG: Self = Self(9);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_USER_ABORT
    pub const VP8_ENC_ERROR_USER_ABORT: Self = Self(10);
    // upstream: src/webp/encode.h VP8_ENC_ERROR_LAST
    pub const VP8_ENC_ERROR_LAST: Self = Self(11);
}

// upstream: src/webp/encode.h WebPWriterFunction
/// Writes `data` and returns `false` on failure (C: `WebPWriterFunction`). The
/// picture parameter of C is omitted: no upstream callback reads a field of it, and
/// the closure owns the context that C passes as `picture->custom_ptr` (ADR 0003).
/// The ABI mirror (WP13) passes its `repr(C)` picture pointer to C callbacks itself.
pub type WebPWriterFunction<'a> = Box<dyn FnMut(&[u8]) -> bool + 'a>;

// upstream: src/webp/encode.h WebPProgressHook
/// Reports `percent` and returns `false` to request an abort (C: `WebPProgressHook`).
/// The picture parameter of C is omitted for the reason given for
/// [`WebPWriterFunction`]; the closure owns the context that C passes as
/// `picture->user_data` (ADR 0003).
pub type WebPProgressHook<'a> = Box<dyn FnMut(i32) -> bool + 'a>;

// upstream: src/webp/encode.h WebPMemoryWriter
/// The buffer that `WebPMemoryWrite` fills (C: `WebPMemoryWriter`). `mem` owns the
/// bytes and holds `max_size` of them, of which the first `size` are written; WP9's
/// writer keeps that invariant.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebPMemoryWriter {
    /// Final buffer, of length `max_size`.
    pub mem: Vec<u8>,
    /// Final size of the written data.
    pub size: usize,
    /// Total capacity.
    pub max_size: usize,
    /// Padding for later use.
    pub pad: [u32; 1],
}

// upstream: src/webp/encode.h WebPPicture
/// Main exchange structure: input samples, output bytes and statistics (C:
/// `WebPPicture`).
///
/// A plane that C holds as `NULL` is `None`. The Y, U, V and A planes live in
/// `yuva` and the ARGB plane in `argb_storage`; each storage is owned or borrowed
/// as a whole, as ADR 0003 requires. The caller's `extra_info` and `stats` are
/// borrowed for `'a`. The fields `custom_ptr` and `user_data` are replaced by the
/// closures of `writer` and `progress_hook`, and the unused pointer padding of C is
/// not kept.
#[derive(Default)]
pub struct WebPPicture<'a> {
    // INPUT
    /// Main flag: ARGB input when non-zero, YUV input otherwise (C: `use_argb`).
    pub use_argb: i32,
    /// Colorspace of the YUV input (C: `colorspace`).
    pub colorspace: WebPEncCSP,
    /// Dimensions, at most `WEBP_MAX_DIMENSION` (C: `width`, `height`).
    pub width: i32,
    pub height: i32,
    /// Luma, chroma U and chroma V planes (C: `y`, `u`, `v`).
    pub y: Option<PlaneOffset>,
    pub u: Option<PlaneOffset>,
    pub v: Option<PlaneOffset>,
    /// Luma and chroma strides (C: `y_stride`, `uv_stride`).
    pub y_stride: i32,
    pub uv_stride: i32,
    /// Alpha plane and its stride (C: `a`, `a_stride`).
    pub a: Option<PlaneOffset>,
    pub a_stride: i32,
    /// Padding for later use.
    pub pad1: [u32; 2],
    /// ARGB plane, and its stride in pixels (C: `argb`, `argb_stride`).
    pub argb: Option<PlaneOffset>,
    pub argb_stride: i32,
    /// Padding for later use.
    pub pad2: [u32; 3],

    // OUTPUT
    /// Byte-emission hook; `None` when C's `writer` is `NULL` (C: `writer`).
    pub writer: Option<WebPWriterFunction<'a>>,
    /// Map of extra information, one entry per macroblock, when requested (C: `extra_info_type`).
    pub extra_info_type: i32,
    /// The caller's extra-information map (C: `extra_info`).
    pub extra_info: Option<&'a mut [u8]>,
    /// Side statistics, when requested (C: `stats`).
    pub stats: Option<&'a mut WebPAuxStats>,
    /// The first error recorded during encoding (C: `error_code`).
    pub error_code: WebPEncodingError,
    /// Progress report; `None` when C's `progress_hook` is `NULL`.
    pub progress_hook: Option<WebPProgressHook<'a>>,
    /// Padding for later use.
    pub pad3: [u32; 3],
    /// Padding for later use.
    pub pad6: [u32; 8],

    // PRIVATE
    /// The Y, U, V and A planes: C's `memory_` when the picture allocated them.
    pub yuva: PlaneStorage<'a, u8>,
    /// The ARGB plane: C's `memory_argb_` when the picture allocated it.
    pub argb_storage: PlaneStorage<'a, u32>,
}

// upstream: src/webp/encode.h WebPConfigInit
/// Initializes `config` with the default preset and quality 75 (C: the inline
/// `WebPConfigInit`). Its translation is WP9's (see the module header).
pub fn webp_config_init(config: &mut WebPConfig) -> bool {
    crate::enc::config_enc::webp_config_init_internal(
        config,
        WebPPreset::WEBP_PRESET_DEFAULT,
        75.0,
        WEBP_ENCODER_ABI_VERSION,
    )
}

// upstream: src/webp/encode.h WebPConfigPreset
/// Initializes `config` with `preset` and `quality` (C: the inline
/// `WebPConfigPreset`). Its translation is WP9's (see the module header).
pub fn webp_config_preset(config: &mut WebPConfig, preset: WebPPreset, quality: f32) -> bool {
    crate::enc::config_enc::webp_config_init_internal(
        config,
        preset,
        quality,
        WEBP_ENCODER_ABI_VERSION,
    )
}

// upstream: src/webp/encode.h WebPPictureInit
/// Initializes `picture` for the current ABI version (C: the inline
/// `WebPPictureInit`). Its translation is WP9's (see the module header).
pub fn webp_picture_init(picture: &mut WebPPicture<'_>) -> bool {
    crate::enc::picture_enc::webp_picture_init_internal(picture, WEBP_ENCODER_ABI_VERSION)
}
