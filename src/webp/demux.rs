// libwebp v1.6.0, src/webp/demux.h:
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
// Demux API.
// Enables extraction of image and extended format data from WebP files.

//! Translation of `src/webp/demux.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! Exception: WP8 translates `WebPDemux`, `WebPDemuxPartial`, `WebPIterator` and `WebPChunkIterator` from `src/webp/demux.h`, because the functions call `WebPDemuxInternal`, which WP8 translates from `src/demux/demux.c`, and the iterators hold the demuxer's state.
//!
//! Exception: WP14 translates `WebPAnimDecoderOptionsInit` and `WebPAnimDecoderNew` from `src/webp/demux.h`, because each one calls an `Internal` entry point that WP14 translates from `src/demux/anim_decode.c`.
//!
//! The iterators `WebPIterator` and `WebPChunkIterator` are WP8's, because they
//! hold the demuxer's state (exceptions in `tools/units.json`). They hold the
//! `WebPData` that WP1 provides in `mux_types.rs`.

use crate::webp::decode::WEBP_CSP_MODE;

// upstream: src/webp/demux.h WEBP_DEMUX_ABI_VERSION
pub const WEBP_DEMUX_ABI_VERSION: i32 = 0x0107;

// upstream: src/webp/demux.h WebPDemuxState
/// Parser state of a demuxer (C: `WebPDemuxState`); has negative values.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPDemuxState(pub i32);

impl WebPDemuxState {
    // upstream: src/webp/demux.h WEBP_DEMUX_PARSE_ERROR
    pub const WEBP_DEMUX_PARSE_ERROR: Self = Self(-1);
    // upstream: src/webp/demux.h WEBP_DEMUX_PARSING_HEADER
    pub const WEBP_DEMUX_PARSING_HEADER: Self = Self(0);
    // upstream: src/webp/demux.h WEBP_DEMUX_PARSED_HEADER
    pub const WEBP_DEMUX_PARSED_HEADER: Self = Self(1);
    // upstream: src/webp/demux.h WEBP_DEMUX_DONE
    pub const WEBP_DEMUX_DONE: Self = Self(2);
}

// upstream: src/webp/demux.h WebPFormatFeature
/// Features that `WebPDemuxGetI` reports (C: `WebPFormatFeature`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPFormatFeature(pub u32);

impl WebPFormatFeature {
    // upstream: src/webp/demux.h WEBP_FF_FORMAT_FLAGS
    pub const WEBP_FF_FORMAT_FLAGS: Self = Self(0);
    // upstream: src/webp/demux.h WEBP_FF_CANVAS_WIDTH
    pub const WEBP_FF_CANVAS_WIDTH: Self = Self(1);
    // upstream: src/webp/demux.h WEBP_FF_CANVAS_HEIGHT
    pub const WEBP_FF_CANVAS_HEIGHT: Self = Self(2);
    // upstream: src/webp/demux.h WEBP_FF_LOOP_COUNT
    pub const WEBP_FF_LOOP_COUNT: Self = Self(3);
    // upstream: src/webp/demux.h WEBP_FF_BACKGROUND_COLOR
    pub const WEBP_FF_BACKGROUND_COLOR: Self = Self(4);
    // upstream: src/webp/demux.h WEBP_FF_FRAME_COUNT
    pub const WEBP_FF_FRAME_COUNT: Self = Self(5);
}

// upstream: src/webp/demux.h WebPAnimDecoderOptions
/// Global options of the animation decoder (C: `WebPAnimDecoderOptions`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPAnimDecoderOptions {
    /// Output colorspace: `MODE_RGBA`, `MODE_BGRA`, `MODE_rgbA` or `MODE_bgrA`.
    pub color_mode: WEBP_CSP_MODE,
    /// Non-zero uses multi-threaded decoding.
    pub use_threads: i32,
    /// Padding for later use.
    pub padding: [u32; 7],
}

// upstream: src/webp/demux.h WebPAnimInfo
/// Global information about an animation (C: `WebPAnimInfo`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPAnimInfo {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub loop_count: u32,
    pub bgcolor: u32,
    pub frame_count: u32,
    /// Padding for later use.
    pub pad: [u32; 4],
}
