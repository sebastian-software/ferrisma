// libwebp v1.6.0, src/mux/muxi.h:
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
// Internal header for mux library.
//
// Author: Urvang (urvang@google.com)

//! Translation of `src/mux/muxi.h` from libwebp v1.6.0.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! C keeps singly linked lists of `WebPChunk` and `WebPMuxImage`; the
//! translation keeps vectors with the same insertion and search order, so the
//! `next` fields are gone. A chunk's bytes are a [`WebPData`], whose `Cow` is
//! borrowed where C's chunk does not own its data (`owner == 0`, the bytes were
//! assigned with `copy_data == 0`) and owned where it does, so C's `owner` flag
//! has no field of its own. C's `bytes == NULL` inside a mux is an owned empty
//! `WebPData` ([`data_is_null`]): `WebPDataCopy` leaves `bytes` `NULL` for empty
//! data, and every other source of a chunk's bytes is a non-null pointer.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Lists and ownership.** C's `NULL` bytes inside a mux (an empty copy,
//!    `WebPDataCopy` of empty data) are an owned empty `WebPData`
//!    (`data_is_null`); `CreateVP8XChunk` tests them for the metadata flags and
//!    the image, as C tests `bytes != NULL` (not compared with C for the
//!    metadata, see `# Excluded undefined behavior`, item 2, of
//!    `src/mux/muxinternal.rs`), and the C ABI hands them out as `NULL`.
//!    Functions that return C's next list element return nothing; `ChunkAppend`
//!    needs no cached list end (`chunk_list_ends`); `ChunkSetHead` takes the
//!    chunk and leaves it initialized, where C clears its `owner` flag, so
//!    `MuxImageParse` reads a chunk's size before handing it over.

use std::borrow::Cow;

use crate::webp::format_constants::CHUNK_HEADER_SIZE;
use crate::webp::mux::WebPChunkId;
use crate::webp::mux_types::WebPData;

//------------------------------------------------------------------------------
// Defines and constants.

// upstream: src/mux/muxi.h MUX_MAJ_VERSION
pub const MUX_MAJ_VERSION: i32 = 1;
// upstream: src/mux/muxi.h MUX_MIN_VERSION
pub const MUX_MIN_VERSION: i32 = 6;
// upstream: src/mux/muxi.h MUX_REV_VERSION
pub const MUX_REV_VERSION: i32 = 0;

// upstream: src/mux/muxi.h WebPChunk
/// Chunk object (C: `WebPChunk`). C's `owner` is the ownership of `data` (see
/// the module documentation) and C's `next` is the position in its vector.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebPChunk<'a> {
    /// The chunk's tag; `NIL_TAG` for a void chunk.
    pub tag: u32,
    /// The payload: borrowed for `'a`, or owned by the chunk. VP8X, ANIM and
    /// other internally created chunks like ANMF are always owned.
    pub data: WebPData<'a>,
}

// upstream: src/mux/muxi.h WebPMuxImage
/// `MuxImage` object: stores a full WebP image (including ANMF chunk, ALPH chunk
/// and VP8/VP8L chunk) (C: `WebPMuxImage`). C holds `header`, `alpha` and `img`
/// as lists with at most one chunk; `next` is the position in the mux's vector.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebPMuxImage<'a> {
    /// Corresponds to `WEBP_CHUNK_ANMF`.
    pub header: Option<WebPChunk<'a>>,
    /// Corresponds to `WEBP_CHUNK_ALPHA`.
    pub alpha: Option<WebPChunk<'a>>,
    /// Corresponds to `WEBP_CHUNK_IMAGE`.
    pub img: Option<WebPChunk<'a>>,
    /// Corresponds to `WEBP_CHUNK_UNKNOWN`.
    pub unknown: Vec<WebPChunk<'a>>,
    pub width: i32,
    pub height: i32,
    /// Through ALPH chunk or as part of VP8L.
    pub has_alpha: bool,
    /// True if only some of the chunks are filled.
    pub is_partial: bool,
}

// upstream: src/mux/muxi.h WebPMux
/// Main mux object: stores data chunks (C: `struct WebPMux`). Chunks given
/// without copying borrow their bytes for `'a`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WebPMux<'a> {
    pub images: Vec<WebPMuxImage<'a>>,
    pub iccp: Vec<WebPChunk<'a>>,
    pub exif: Vec<WebPChunk<'a>>,
    pub xmp: Vec<WebPChunk<'a>>,
    pub anim: Vec<WebPChunk<'a>>,
    pub vp8x: Vec<WebPChunk<'a>>,
    pub unknown: Vec<WebPChunk<'a>>,
    pub canvas_width: i32,
    pub canvas_height: i32,
}

// upstream: src/mux/muxi.h CHUNK_INDEX
/// Index within `CHUNKS` (C: `CHUNK_INDEX`). Two enums exist so that two chunks
/// can have the same id (`WEBP_CHUNK_IMAGE` is `IDX_VP8` or `IDX_VP8L`).
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CHUNK_INDEX(pub u32);

impl CHUNK_INDEX {
    // upstream: src/mux/muxi.h IDX_VP8X
    pub const IDX_VP8X: Self = Self(0);
    // upstream: src/mux/muxi.h IDX_ICCP
    pub const IDX_ICCP: Self = Self(1);
    // upstream: src/mux/muxi.h IDX_ANIM
    pub const IDX_ANIM: Self = Self(2);
    // upstream: src/mux/muxi.h IDX_ANMF
    pub const IDX_ANMF: Self = Self(3);
    // upstream: src/mux/muxi.h IDX_ALPHA
    pub const IDX_ALPHA: Self = Self(4);
    // upstream: src/mux/muxi.h IDX_VP8
    pub const IDX_VP8: Self = Self(5);
    // upstream: src/mux/muxi.h IDX_VP8L
    pub const IDX_VP8L: Self = Self(6);
    // upstream: src/mux/muxi.h IDX_EXIF
    pub const IDX_EXIF: Self = Self(7);
    // upstream: src/mux/muxi.h IDX_XMP
    pub const IDX_XMP: Self = Self(8);
    // upstream: src/mux/muxi.h IDX_UNKNOWN
    pub const IDX_UNKNOWN: Self = Self(9);
    // upstream: src/mux/muxi.h IDX_NIL
    pub const IDX_NIL: Self = Self(10);
    // upstream: src/mux/muxi.h IDX_LAST_CHUNK
    pub const IDX_LAST_CHUNK: Self = Self(11);

    /// The index as a position in `CHUNKS`.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

// upstream: src/mux/muxi.h NIL_TAG
/// To signal void chunk.
pub const NIL_TAG: u32 = 0x0000_0000;

// upstream: src/mux/muxi.h ChunkInfo
/// Tag, id and fixed payload size of a chunk type (C: `ChunkInfo`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkInfo {
    pub tag: u32,
    pub id: WebPChunkId,
    pub size: u32,
}

//------------------------------------------------------------------------------
// Chunk sources (no C counterpart).

/// The bytes that a chunk is assigned from (C: the `const WebPData*` of
/// `ChunkAssignData`, or a pointer into the parsed buffer). C keeps the pointer
/// when it does not copy; Rust can keep bytes only when they are borrowed for
/// the mux's lifetime `'a` (`Lent`). Bytes that are only readable during the
/// call (`Read`: a `WebPData` that owns its bytes, or a buffer local to the
/// call) are copied even with `copy_data == 0`; the `# Representation` section
/// of `src/webp/mux.rs` records it.
#[derive(Clone, Copy, Debug)]
pub enum ChunkBytes<'a: 's, 's> {
    /// Bytes that the mux may keep for `'a` without copying them.
    Lent(&'a [u8]),
    /// Bytes that the mux may only read during the call.
    Read(&'s [u8]),
}

impl<'a: 's, 's> ChunkBytes<'a, 's> {
    /// The bytes of `data`: lent when `data` borrows them, readable when it owns
    /// them.
    #[must_use]
    pub fn of(data: &'s WebPData<'a>) -> Self {
        match &data.bytes {
            Cow::Borrowed(bytes) => Self::Lent(bytes),
            Cow::Owned(bytes) => Self::Read(bytes),
        }
    }

    /// The bytes, for reading.
    #[expect(
        clippy::match_same_arms,
        reason = "the arms bind slices of different lifetimes"
    )]
    #[must_use]
    pub fn bytes(self) -> &'s [u8] {
        match self {
            Self::Lent(bytes) => bytes,
            Self::Read(bytes) => bytes,
        }
    }

    /// The `len` bytes at `start`, with the same lending (C: pointer arithmetic
    /// on the source).
    ///
    /// # Panics
    ///
    /// Panics when the range is not inside the bytes.
    #[must_use]
    pub fn range(self, start: usize, len: usize) -> Self {
        match self {
            Self::Lent(bytes) => Self::Lent(&bytes[start..start + len]),
            Self::Read(bytes) => Self::Read(&bytes[start..start + len]),
        }
    }
}

/// Whether a chunk's bytes are C's `NULL`: an owned empty `WebPData`, which is
/// what `WebPDataCopy` gives for empty data (see the module documentation).
#[must_use]
pub fn data_is_null(data: &WebPData<'_>) -> bool {
    matches!(&data.bytes, Cow::Owned(bytes) if bytes.is_empty())
}

//------------------------------------------------------------------------------
// Chunk object management.

// upstream: src/mux/muxi.h SizeWithPadding
/// Returns size of the chunk including chunk header and padding byte (if any).
/// C masks with `~1U`, which keeps 32 bits; `chunk_size` is at most
/// `MAX_CHUNK_PAYLOAD`, so the sum fits them.
#[must_use]
pub fn size_with_padding(chunk_size: usize) -> usize {
    debug_assert!(chunk_size <= crate::webp::format_constants::MAX_CHUNK_PAYLOAD as usize);
    CHUNK_HEADER_SIZE as usize + (chunk_size.wrapping_add(1) & (!1_u32 as usize))
}

// upstream: src/mux/muxi.h ChunkDiskSize
/// Size of a chunk including header and padding.
#[must_use]
pub fn chunk_disk_size(chunk: &WebPChunk<'_>) -> usize {
    let data_size = chunk.data.bytes.len();
    size_with_padding(data_size)
}

//------------------------------------------------------------------------------
// MuxImage object management.

// upstream: src/mux/muxi.h IsWPI
/// Check if given ID corresponds to an image related chunk.
#[must_use]
pub fn is_wpi(id: WebPChunkId) -> bool {
    matches!(
        id,
        WebPChunkId::WEBP_CHUNK_ANMF
            | WebPChunkId::WEBP_CHUNK_ALPHA
            | WebPChunkId::WEBP_CHUNK_IMAGE
    )
}
