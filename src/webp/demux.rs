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

use crate::demux::demux::{WebPDemuxer, webp_demux_internal};
use crate::webp::decode::WEBP_CSP_MODE;
use crate::webp::mux_types::{WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose};

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

// upstream: src/webp/demux.h WebPIterator
/// Iterator over the frames of a demuxer (C: `WebPIterator`). `dmux` replaces
/// C's `private_`: the demuxer is immutable after construction, so the iterator
/// borrows it, and `fragment` borrows the frame's bytes from the input.
#[derive(Clone, Debug)]
pub struct WebPIterator<'d> {
    /// The frame number of this frame (C: `frame_num`).
    pub frame_num: i32,
    /// Equivalent to `WEBP_FF_FRAME_COUNT` (C: `num_frames`).
    pub num_frames: i32,
    /// Offset relative to the canvas.
    pub x_offset: i32,
    /// Offset relative to the canvas.
    pub y_offset: i32,
    /// Dimensions of this frame.
    pub width: i32,
    /// Dimensions of this frame.
    pub height: i32,
    /// Display duration in milliseconds.
    pub duration: i32,
    /// Dispose method for the frame.
    pub dispose_method: WebPMuxAnimDispose,
    /// Non-zero if `fragment` contains a full frame. Partial images may still be
    /// decoded with the WebP incremental decoder.
    pub complete: i32,
    /// The frame given by `frame_num`. For historical reasons C calls it a fragment.
    pub fragment: WebPData<'d>,
    /// Non-zero if the frame contains transparency.
    pub has_alpha: i32,
    /// Blend operation for the frame.
    pub blend_method: WebPMuxAnimBlend,
    /// Padding for later use.
    pub pad: [u32; 2],
    /// The demuxer that this iterator reads (C: `private_`).
    pub(crate) dmux: &'d WebPDemuxer<'d>,
}

// upstream: src/webp/demux.h WebPChunkIterator
/// Iterator over the chunks with one tag of a demuxer (C: `WebPChunkIterator`).
/// `dmux` replaces C's `private_`, as for [`WebPIterator`]. C recovers the tag
/// from the four bytes before the payload; the iterator keeps the tag that
/// `WebPDemuxGetChunk` was given, which those bytes equal.
#[derive(Clone, Debug)]
pub struct WebPChunkIterator<'d> {
    /// The current number of the chunk with the tag (C: `chunk_num`).
    pub chunk_num: i32,
    /// The total number of chunks with the tag (C: `num_chunks`).
    pub num_chunks: i32,
    /// The payload of the chunk.
    pub chunk: WebPData<'d>,
    /// Padding for later use.
    pub pad: [u32; 6],
    /// The demuxer that this iterator reads (C: `private_`).
    pub(crate) dmux: &'d WebPDemuxer<'d>,
    /// The tag of the chunks that the iterator walks.
    pub(crate) fourcc: [u8; 4],
}

// upstream: src/webp/demux.h WebPDemux
/// Parses the full WebP file given by `data`. For single images the WebP file
/// header alone or the file header and the chunk header may be absent. Returns
/// `None` on a parse error.
#[must_use]
pub fn webp_demux<'d>(data: &'d WebPData<'_>) -> Option<WebPDemuxer<'d>> {
    webp_demux_internal(data, false, WEBP_DEMUX_ABI_VERSION).0
}

// upstream: src/webp/demux.h WebPDemuxPartial
/// Parses the possibly incomplete WebP file given by `data`, and returns the
/// parser state with the demuxer, as C writes `*state`. The demuxer keeps
/// borrows of `data`, so it must be rebuilt when `data` changes. Returns `None`
/// on error, or when there is not enough data to start parsing.
#[must_use]
pub fn webp_demux_partial<'d>(data: &'d WebPData<'_>) -> (Option<WebPDemuxer<'d>>, WebPDemuxState) {
    webp_demux_internal(data, true, WEBP_DEMUX_ABI_VERSION)
}
