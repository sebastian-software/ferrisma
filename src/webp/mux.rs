// libwebp v1.6.0, src/webp/mux.h:
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
//  RIFF container manipulation and encoding for WebP images.
//
// Authors: Urvang (urvang@google.com)
//          Vikas (vikasa@google.com)

//! Translation of `src/webp/mux.h` from libwebp v1.6.0.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! The opaque objects of the header are the translation's structs:
//! `WebPMux` is [`crate::mux::muxi::WebPMux`] (C defines it in `muxi.h`) and
//! `WebPAnimEncoder` is [`crate::mux::anim_encode::WebPAnimEncoder`] (C defines
//! it in `anim_encode.c`). `WebPMuxAnimParams` and `WebPAnimEncoderOptions`
//! hold no pointers, so they are `repr(C)` and shared with the C ABI;
//! `WebPMuxFrameInfo` holds a [`WebPData`] and has a `repr(C)` mirror in
//! `src/abi/mux.rs`.
//!
//! Functions that C writes through output pointers return `Result<T,
//! WebPMuxError>`, whose `Err` never holds `WEBP_MUX_OK` (ADR 0003, "Errors").
//! A mux keeps the chunks that it was given without copying (`copy_data == 0`)
//! as borrowed bytes for `'a`; see `docs/status/wp15.md` for how the lifetime
//! is chosen.

use crate::mux::anim_encode::{WebPAnimEncoder, webp_anim_encoder_new_internal};
use crate::mux::muxedit::webp_new_internal;
use crate::mux::muxi::WebPMux;
use crate::mux::muxread::webp_mux_create_internal;
use crate::webp::mux_types::{WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose};

// upstream: src/webp/mux.h WEBP_MUX_ABI_VERSION
/// Version of the mux ABI: MAJOR (8 bits) and MINOR (8 bits).
pub const WEBP_MUX_ABI_VERSION: i32 = 0x0109;

// upstream: src/webp/mux.h WebPMuxError
/// Error codes of the mux (C: `WebPMuxError`, an `int` enum because it has
/// negative values).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPMuxError(pub i32);

impl WebPMuxError {
    // upstream: src/webp/mux.h WEBP_MUX_OK
    pub const WEBP_MUX_OK: Self = Self(1);
    // upstream: src/webp/mux.h WEBP_MUX_NOT_FOUND
    pub const WEBP_MUX_NOT_FOUND: Self = Self(0);
    // upstream: src/webp/mux.h WEBP_MUX_INVALID_ARGUMENT
    pub const WEBP_MUX_INVALID_ARGUMENT: Self = Self(-1);
    // upstream: src/webp/mux.h WEBP_MUX_BAD_DATA
    pub const WEBP_MUX_BAD_DATA: Self = Self(-2);
    // upstream: src/webp/mux.h WEBP_MUX_MEMORY_ERROR
    pub const WEBP_MUX_MEMORY_ERROR: Self = Self(-3);
    // upstream: src/webp/mux.h WEBP_MUX_NOT_ENOUGH_DATA
    pub const WEBP_MUX_NOT_ENOUGH_DATA: Self = Self(-4);

    /// The C code of a result: `WEBP_MUX_OK` for `Ok`, the error otherwise.
    #[must_use]
    pub fn of<T>(result: &Result<T, Self>) -> Self {
        match result {
            Ok(_) => Self::WEBP_MUX_OK,
            Err(err) => *err,
        }
    }
}

// upstream: src/webp/mux.h WebPChunkId
/// IDs for different types of chunks (C: `WebPChunkId`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPChunkId(pub u32);

impl WebPChunkId {
    // upstream: src/webp/mux.h WEBP_CHUNK_VP8X
    pub const WEBP_CHUNK_VP8X: Self = Self(0);
    // upstream: src/webp/mux.h WEBP_CHUNK_ICCP
    pub const WEBP_CHUNK_ICCP: Self = Self(1);
    // upstream: src/webp/mux.h WEBP_CHUNK_ANIM
    pub const WEBP_CHUNK_ANIM: Self = Self(2);
    // upstream: src/webp/mux.h WEBP_CHUNK_ANMF
    pub const WEBP_CHUNK_ANMF: Self = Self(3);
    // upstream: src/webp/mux.h WEBP_CHUNK_DEPRECATED
    pub const WEBP_CHUNK_DEPRECATED: Self = Self(4);
    // upstream: src/webp/mux.h WEBP_CHUNK_ALPHA
    pub const WEBP_CHUNK_ALPHA: Self = Self(5);
    // upstream: src/webp/mux.h WEBP_CHUNK_IMAGE
    pub const WEBP_CHUNK_IMAGE: Self = Self(6);
    // upstream: src/webp/mux.h WEBP_CHUNK_EXIF
    pub const WEBP_CHUNK_EXIF: Self = Self(7);
    // upstream: src/webp/mux.h WEBP_CHUNK_XMP
    pub const WEBP_CHUNK_XMP: Self = Self(8);
    // upstream: src/webp/mux.h WEBP_CHUNK_UNKNOWN
    pub const WEBP_CHUNK_UNKNOWN: Self = Self(9);
    // upstream: src/webp/mux.h WEBP_CHUNK_NIL
    pub const WEBP_CHUNK_NIL: Self = Self(10);
}

//------------------------------------------------------------------------------
// Life of a Mux object

// upstream: src/webp/mux.h WebPMuxNew
/// Creates an empty mux object (C: the inline `WebPMuxNew`). Returns `None` in
/// case of memory error, as C returns `NULL`.
#[must_use]
pub fn webp_mux_new<'a>() -> Option<WebPMux<'a>> {
    webp_new_internal(WEBP_MUX_ABI_VERSION)
}

//------------------------------------------------------------------------------
// Mux creation.

// upstream: src/webp/mux.h WebPMuxCreate
/// Creates a mux object from raw data given in WebP RIFF format (C: the inline
/// `WebPMuxCreate`). With `copy_data == 1` the data is copied into the mux;
/// with `copy_data == 0` the mux keeps borrowed bytes for `'a`, which must stay
/// valid for the lifetime of the mux, as the header requires. Returns `None`
/// for invalid data or a memory error.
#[must_use]
pub fn webp_mux_create<'a>(bitstream: &WebPData<'a>, copy_data: i32) -> Option<WebPMux<'a>> {
    webp_mux_create_internal(bitstream, copy_data, WEBP_MUX_ABI_VERSION)
}

//------------------------------------------------------------------------------
// Images.

// upstream: src/webp/mux.h WebPMuxFrameInfo
/// Encapsulates data about a single frame (C: `WebPMuxFrameInfo`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebPMuxFrameInfo<'a> {
    /// Image data: can be a raw VP8/VP8L bitstream or a single-image WebP file.
    pub bitstream: WebPData<'a>,
    /// x-offset of the frame.
    pub x_offset: i32,
    /// y-offset of the frame.
    pub y_offset: i32,
    /// Duration of the frame (in milliseconds).
    pub duration: i32,
    /// Frame type: should be one of `WEBP_CHUNK_ANMF` or `WEBP_CHUNK_IMAGE`.
    pub id: WebPChunkId,
    /// Disposal method for the frame.
    pub dispose_method: WebPMuxAnimDispose,
    /// Blend operation for the frame.
    pub blend_method: WebPMuxAnimBlend,
    /// Padding for later use.
    pub pad: [u32; 1],
}

//------------------------------------------------------------------------------
// Animation.

// upstream: src/webp/mux.h WebPMuxAnimParams
/// Animation parameters (C: `WebPMuxAnimParams`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPMuxAnimParams {
    /// Background color of the canvas stored (in MSB order) as: bits 00 to 07
    /// alpha, 08 to 15 red, 16 to 23 green, 24 to 31 blue.
    pub bgcolor: u32,
    /// Number of times to repeat the animation [0 = infinite].
    pub loop_count: i32,
}

//------------------------------------------------------------------------------
// WebPAnimEncoder API

// upstream: src/webp/mux.h WebPAnimEncoderOptions
/// Global options of the animation encoder (C: `WebPAnimEncoderOptions`).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WebPAnimEncoderOptions {
    /// Animation parameters.
    pub anim_params: WebPMuxAnimParams,
    /// If true, minimize the output size (slow). Implicitly disables key-frame
    /// insertion.
    pub minimize_size: i32,
    /// Minimum distance between consecutive key frames in the output.
    pub kmin: i32,
    /// Maximum distance between consecutive key frames in the output. The library
    /// may insert some key frames as needed to satisfy this criteria. These
    /// conditions should hold: `kmax > kmin` and `kmin >= kmax / 2 + 1`. If
    /// `kmax <= 0`, key-frame insertion is disabled; if `kmax == 1`, all frames
    /// are key-frames (`kmin` does not matter for these special cases).
    pub kmax: i32,
    /// If true, use mixed compression mode; may choose either lossy or lossless
    /// for each frame.
    pub allow_mixed: i32,
    /// If true, print info and warning messages to stderr.
    pub verbose: i32,
    /// Padding for later use.
    pub padding: [u32; 4],
}

// upstream: src/webp/mux.h WebPAnimEncoderOptionsInit
/// Initializes a fresh `WebPAnimEncoderOptions` structure before modification
/// (C: the inline `WebPAnimEncoderOptionsInit`). Returns false in case of
/// version mismatch.
#[must_use]
pub fn webp_anim_encoder_options_init(enc_options: &mut WebPAnimEncoderOptions) -> bool {
    crate::mux::anim_encode::webp_anim_encoder_options_init_internal(
        enc_options,
        WEBP_MUX_ABI_VERSION,
    )
}

// upstream: src/webp/mux.h WebPAnimEncoderNew
/// Creates and initializes a `WebPAnimEncoder` object for a canvas of `width`
/// by `height` (C: the inline `WebPAnimEncoderNew`). `None` options pick
/// reasonable defaults. Returns `None` for invalid dimensions or a memory error.
#[must_use]
pub fn webp_anim_encoder_new<'a>(
    width: i32,
    height: i32,
    enc_options: Option<&WebPAnimEncoderOptions>,
) -> Option<WebPAnimEncoder<'a>> {
    webp_anim_encoder_new_internal(width, height, enc_options, WEBP_MUX_ABI_VERSION)
}
