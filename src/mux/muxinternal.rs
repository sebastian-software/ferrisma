// libwebp v1.6.0, src/mux/muxinternal.c:
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
// Internal objects and utils for mux.
//
// Authors: Urvang (urvang@google.com)
//          Vikas (vikasa@google.com)

//! Translation of `src/mux/muxinternal.c` from libwebp v1.6.0.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! The chunk lists and the image list are vectors (see `src/mux/muxi.rs`):
//! appending is a push and C's `next` pointers are positions. Functions that
//! return the next element of a C list (`ChunkRelease`, `ChunkDelete`,
//! `MuxImageRelease`, `MuxImageDelete`) return nothing; their callers remove
//! the element from its vector. A failed host reservation for a new list
//! element is `WEBP_MUX_MEMORY_ERROR`, where C's `WebPSafeMalloc` fails.

use std::borrow::Cow;

use crate::mux::muxi::{
    CHUNK_INDEX, ChunkBytes, ChunkInfo, MUX_MAJ_VERSION, MUX_MIN_VERSION, MUX_REV_VERSION, NIL_TAG,
    WebPChunk, WebPMux, WebPMuxImage, chunk_disk_size,
};
use crate::mux::muxread::{webp_mux_get_features, webp_mux_num_chunks};
use crate::utils::utils::put_le32;
use crate::webp::format_constants::{
    ANIM_CHUNK_SIZE, ANMF_CHUNK_SIZE, CHUNK_HEADER_SIZE, CHUNK_SIZE_BYTES, RIFF_HEADER_SIZE,
    TAG_SIZE, VP8X_CHUNK_SIZE, mkfourcc,
};
use crate::webp::mux::{WebPChunkId, WebPMuxError};
use crate::webp::mux_types::{WebPData, WebPFeatureFlags, webp_data_copy};

// upstream: src/mux/muxinternal.c UNDEFINED_CHUNK_SIZE
const UNDEFINED_CHUNK_SIZE: u32 = u32::MAX;

/// `MKFOURCC` of four ASCII characters, for the table below.
const fn fourcc(tag: [u8; 4]) -> u32 {
    (tag[0] as u32) | ((tag[1] as u32) << 8) | ((tag[2] as u32) << 16) | ((tag[3] as u32) << 24)
}

// upstream: src/mux/muxinternal.c kChunks
/// Tag, id and fixed payload size of every chunk type, indexed by `CHUNK_INDEX`.
pub static CHUNKS: [ChunkInfo; CHUNK_INDEX::IDX_LAST_CHUNK.index()] = [
    ChunkInfo {
        tag: fourcc(*b"VP8X"),
        id: WebPChunkId::WEBP_CHUNK_VP8X,
        size: VP8X_CHUNK_SIZE as u32,
    },
    ChunkInfo {
        tag: fourcc(*b"ICCP"),
        id: WebPChunkId::WEBP_CHUNK_ICCP,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: fourcc(*b"ANIM"),
        id: WebPChunkId::WEBP_CHUNK_ANIM,
        size: ANIM_CHUNK_SIZE as u32,
    },
    ChunkInfo {
        tag: fourcc(*b"ANMF"),
        id: WebPChunkId::WEBP_CHUNK_ANMF,
        size: ANMF_CHUNK_SIZE as u32,
    },
    ChunkInfo {
        tag: fourcc(*b"ALPH"),
        id: WebPChunkId::WEBP_CHUNK_ALPHA,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: fourcc(*b"VP8 "),
        id: WebPChunkId::WEBP_CHUNK_IMAGE,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: fourcc(*b"VP8L"),
        id: WebPChunkId::WEBP_CHUNK_IMAGE,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: fourcc(*b"EXIF"),
        id: WebPChunkId::WEBP_CHUNK_EXIF,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: fourcc(*b"XMP "),
        id: WebPChunkId::WEBP_CHUNK_XMP,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: NIL_TAG,
        id: WebPChunkId::WEBP_CHUNK_UNKNOWN,
        size: UNDEFINED_CHUNK_SIZE,
    },
    ChunkInfo {
        tag: NIL_TAG,
        id: WebPChunkId::WEBP_CHUNK_NIL,
        size: UNDEFINED_CHUNK_SIZE,
    },
];

/// The table entry of `idx` (C: `kChunks[idx]`).
#[must_use]
pub fn chunk_info(idx: CHUNK_INDEX) -> &'static ChunkInfo {
    &CHUNKS[idx.index()]
}

//------------------------------------------------------------------------------

// upstream: src/mux/muxinternal.c WebPGetMuxVersion
/// Returns the version number of the mux library, packed in hexadecimal using
/// 8 bits for each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[must_use]
pub fn webp_get_mux_version() -> i32 {
    (MUX_MAJ_VERSION << 16) | (MUX_MIN_VERSION << 8) | MUX_REV_VERSION
}

//------------------------------------------------------------------------------
// Life of a chunk object.

// upstream: src/mux/muxinternal.c ChunkInit
/// Initialize: no tag, no data.
pub fn chunk_init(chunk: &mut WebPChunk<'_>) {
    *chunk = WebPChunk::default();
    chunk.tag = NIL_TAG;
}

// upstream: src/mux/muxinternal.c ChunkRelease
/// Releases the chunk's data (owned data is dropped, as C clears it when the
/// chunk is its owner) and initializes the chunk. C returns `chunk->next`; the
/// chunk's position in its vector replaces it.
pub fn chunk_release(chunk: &mut WebPChunk<'_>) {
    chunk_init(chunk);
}

//------------------------------------------------------------------------------
// Chunk misc methods.

// upstream: src/mux/muxinternal.c ChunkGetIndexFromTag
/// Get chunk index from chunk tag. Returns `IDX_UNKNOWN` if not found.
///
/// # Panics
///
/// Never: the table has fewer than 2^32 entries.
#[must_use]
pub fn chunk_get_index_from_tag(tag: u32) -> CHUNK_INDEX {
    let mut i = 0;
    while CHUNKS[i].tag != NIL_TAG {
        if tag == CHUNKS[i].tag {
            return CHUNK_INDEX(u32::try_from(i).expect("an index of CHUNKS"));
        }
        i += 1;
    }
    CHUNK_INDEX::IDX_UNKNOWN
}

// upstream: src/mux/muxinternal.c ChunkGetIdFromTag
/// Get chunk id from chunk tag. Returns `WEBP_CHUNK_UNKNOWN` if not found.
#[must_use]
pub fn chunk_get_id_from_tag(tag: u32) -> WebPChunkId {
    let mut i = 0;
    while CHUNKS[i].tag != NIL_TAG {
        if tag == CHUNKS[i].tag {
            return CHUNKS[i].id;
        }
        i += 1;
    }
    WebPChunkId::WEBP_CHUNK_UNKNOWN
}

// upstream: src/mux/muxinternal.c ChunkGetTagFromFourCC
/// Convert a fourcc string to a tag. The bytes are unsigned; C passes `char`
/// values, which sign-extend where `char` is signed (`docs/status/wp15.md`,
/// undefined behavior excluded from the oracle).
#[must_use]
pub fn chunk_get_tag_from_four_cc(fourcc: &[u8; 4]) -> u32 {
    mkfourcc(fourcc[0], fourcc[1], fourcc[2], fourcc[3])
}

// upstream: src/mux/muxinternal.c ChunkGetIndexFromFourCC
/// Get chunk index from fourcc. Returns `IDX_UNKNOWN` if given fourcc is unknown.
#[must_use]
pub fn chunk_get_index_from_four_cc(fourcc: &[u8; 4]) -> CHUNK_INDEX {
    let tag = chunk_get_tag_from_four_cc(fourcc);
    chunk_get_index_from_tag(tag)
}

//------------------------------------------------------------------------------
// Chunk search methods.

// upstream: src/mux/muxinternal.c ChunkSearchNextInList
/// Returns the position of the next chunk with the given tag, from `start` on.
fn chunk_search_next_in_list(chunks: &[WebPChunk<'_>], start: usize, tag: u32) -> Option<usize> {
    let mut chunk = start;
    while chunk < chunks.len() && chunks[chunk].tag != tag {
        chunk += 1;
    }
    (chunk < chunks.len()).then_some(chunk)
}

// upstream: src/mux/muxinternal.c ChunkSearchList
/// Search for nth chunk with given `tag` in the chunk list. `nth = 0` means
/// "last of the list".
#[must_use]
pub fn chunk_search_list<'c, 'a>(
    first: &'c [WebPChunk<'a>],
    nth: u32,
    tag: u32,
) -> Option<&'c WebPChunk<'a>> {
    let mut iter = nth;
    let mut found = chunk_search_next_in_list(first, 0, tag)?;

    loop {
        iter = iter.wrapping_sub(1);
        if iter == 0 {
            break;
        }
        let Some(next_chunk) = chunk_search_next_in_list(first, found + 1, tag) else {
            break;
        };
        found = next_chunk;
    }
    if nth > 0 && iter > 0 {
        None
    } else {
        Some(&first[found])
    }
}

//------------------------------------------------------------------------------
// Chunk writer methods.

// upstream: src/mux/muxinternal.c ChunkAssignData
/// Fill the chunk with the given data. VP8X and ANIM chunks are always copied.
/// Without copying, the chunk keeps lent bytes; bytes that are only readable
/// are copied, and empty readable bytes become an empty borrowed slice, which is
/// a non-null pointer to no bytes, as C keeps the caller's pointer. C accepts a
/// `NULL` `data`, which no caller passes.
///
/// # Errors
///
/// `WEBP_MUX_MEMORY_ERROR` when the copy cannot be allocated.
pub fn chunk_assign_data<'a>(
    chunk: &mut WebPChunk<'a>,
    data: ChunkBytes<'a, '_>,
    copy_data: i32,
    tag: u32,
) -> Result<(), WebPMuxError> {
    let mut copy_data = copy_data;
    // For internally allocated chunks, always copy data & make it owner of data.
    if tag == chunk_info(CHUNK_INDEX::IDX_VP8X).tag || tag == chunk_info(CHUNK_INDEX::IDX_ANIM).tag
    {
        copy_data = 1;
    }

    chunk_release(chunk);

    if copy_data != 0 {
        // Copy data.
        let source = WebPData {
            bytes: Cow::Borrowed(data.bytes()),
        };
        chunk.data = webp_data_copy(&source).ok_or(WebPMuxError::WEBP_MUX_MEMORY_ERROR)?;
    } else {
        // Don't copy data.
        chunk.data = match data {
            ChunkBytes::Lent(bytes) => WebPData {
                bytes: Cow::Borrowed(bytes),
            },
            ChunkBytes::Read([]) => WebPData {
                bytes: Cow::Borrowed(&[]),
            },
            ChunkBytes::Read(bytes) => {
                let source = WebPData {
                    bytes: Cow::Borrowed(bytes),
                };
                webp_data_copy(&source).ok_or(WebPMuxError::WEBP_MUX_MEMORY_ERROR)?
            }
        };
    }
    chunk.tag = tag;
    Ok(())
}

/// A C chunk list (`WebPChunk**`) as `ChunkSetHead` uses it: a vector, or one
/// of the single-chunk slots of a `WebPMuxImage`. No C counterpart.
pub trait ChunkList<'a> {
    /// Whether the list is empty (C: `*chunk_list == NULL`).
    fn is_null(&self) -> bool;
    /// Reserves room for one more chunk; false when the host refuses.
    fn reserve_one(&mut self) -> bool;
    /// Appends `chunk` at the end.
    fn append(&mut self, chunk: WebPChunk<'a>);
}

impl<'a> ChunkList<'a> for Vec<WebPChunk<'a>> {
    fn is_null(&self) -> bool {
        self.is_empty()
    }

    fn reserve_one(&mut self) -> bool {
        self.try_reserve(1).is_ok()
    }

    fn append(&mut self, chunk: WebPChunk<'a>) {
        self.push(chunk);
    }
}

impl<'a> ChunkList<'a> for Option<WebPChunk<'a>> {
    fn is_null(&self) -> bool {
        self.is_none()
    }

    fn reserve_one(&mut self) -> bool {
        true
    }

    fn append(&mut self, chunk: WebPChunk<'a>) {
        debug_assert!(self.is_none());
        *self = Some(chunk);
    }
}

// upstream: src/mux/muxinternal.c ChunkSetHead
/// Sets `chunk` as the only element in `chunk_list` if it is empty. On success
/// ownership is transferred from `chunk` to the `chunk_list`, and `chunk` is
/// left initialized (C clears its `owner` flag instead).
///
/// # Errors
///
/// `WEBP_MUX_NOT_FOUND` when the list is not empty, `WEBP_MUX_MEMORY_ERROR`
/// when the host refuses the memory for the element.
pub fn chunk_set_head<'a>(
    chunk: &mut WebPChunk<'a>,
    chunk_list: &mut impl ChunkList<'a>,
) -> Result<(), WebPMuxError> {
    if !chunk_list.is_null() {
        return Err(WebPMuxError::WEBP_MUX_NOT_FOUND);
    }

    if !chunk_list.reserve_one() {
        return Err(WebPMuxError::WEBP_MUX_MEMORY_ERROR);
    }
    chunk_list.append(core::mem::take(chunk));
    Ok(())
}

// upstream: src/mux/muxinternal.c ChunkAppend
/// Sets `chunk` at last position in the `chunk_list`. On success ownership is
/// transferred from `chunk` to the `chunk_list`. C also moves its cached list
/// end to the new last element; appending to a vector needs no cache.
///
/// # Errors
///
/// `WEBP_MUX_MEMORY_ERROR` when the host refuses the memory for the element.
pub fn chunk_append<'a>(
    chunk: &mut WebPChunk<'a>,
    chunk_list: &mut Vec<WebPChunk<'a>>,
) -> Result<(), WebPMuxError> {
    if chunk_list.is_empty() {
        chunk_set_head(chunk, chunk_list)
    } else {
        if !chunk_list.reserve_one() {
            return Err(WebPMuxError::WEBP_MUX_MEMORY_ERROR);
        }
        chunk_list.push(core::mem::take(chunk));
        Ok(())
    }
}

//------------------------------------------------------------------------------
// Chunk deletion method(s).

// upstream: src/mux/muxinternal.c ChunkDelete
/// Deletes given chunk. C returns `chunk->next`; the caller removes the chunk
/// from its vector or slot and passes it here.
pub fn chunk_delete(chunk: WebPChunk<'_>) {
    let mut chunk = chunk;
    chunk_release(&mut chunk);
}

// upstream: src/mux/muxinternal.c ChunkListDelete
/// Deletes all chunks in the given chunk list.
pub fn chunk_list_delete(chunk_list: &mut Vec<WebPChunk<'_>>) {
    for chunk in chunk_list.drain(..) {
        chunk_delete(chunk);
    }
}

//------------------------------------------------------------------------------
// Chunk serialization methods.

// upstream: src/mux/muxinternal.c ChunkEmit
/// Writes the chunk at `dst[pos..]` and returns the position after it. C writes
/// the size as `(uint32_t)chunk_size`, which holds at most `MAX_CHUNK_PAYLOAD`.
#[expect(clippy::cast_possible_truncation, reason = "C's (uint32_t) conversion")]
fn chunk_emit(chunk: &WebPChunk<'_>, dst: &mut [u8], pos: usize) -> usize {
    let chunk_size = chunk.data.bytes.len();
    debug_assert!(chunk.tag != NIL_TAG);
    put_le32(&mut dst[pos..], chunk.tag);
    put_le32(&mut dst[pos + TAG_SIZE as usize..], chunk_size as u32);
    debug_assert_eq!(chunk_size, chunk_size as u32 as usize);
    let payload = pos + CHUNK_HEADER_SIZE as usize;
    dst[payload..payload + chunk_size].copy_from_slice(&chunk.data.bytes);
    if chunk_size & 1 != 0 {
        dst[payload + chunk_size] = 0; // Add padding.
    }
    pos + chunk_disk_size(chunk)
}

// upstream: src/mux/muxinternal.c ChunkListEmit
/// Write out the given list of chunks at `dst[pos..]`; returns the position
/// after them.
#[must_use]
pub fn chunk_list_emit(chunk_list: &[WebPChunk<'_>], dst: &mut [u8], pos: usize) -> usize {
    let mut pos = pos;
    for chunk in chunk_list {
        pos = chunk_emit(chunk, dst, pos);
    }
    pos
}

// upstream: src/mux/muxinternal.c ChunkListDiskSize
/// Total size of a list of chunks.
#[must_use]
pub fn chunk_list_disk_size(chunk_list: &[WebPChunk<'_>]) -> usize {
    let mut size = 0_usize;
    for chunk in chunk_list {
        size += chunk_disk_size(chunk);
    }
    size
}

//------------------------------------------------------------------------------
// Life of a MuxImage object.

// upstream: src/mux/muxinternal.c MuxImageInit
/// Initialize.
pub fn mux_image_init(wpi: &mut WebPMuxImage<'_>) {
    *wpi = WebPMuxImage::default();
}

// upstream: src/mux/muxinternal.c MuxImageRelease
/// Releases image `wpi` and initializes it. C returns `wpi->next`; the image's
/// position in its vector replaces it.
pub fn mux_image_release(wpi: &mut WebPMuxImage<'_>) {
    // There should be at most one chunk of 'header', 'alpha', 'img' but C calls
    // ChunkListDelete to be safe; the slots hold at most one.
    if let Some(chunk) = wpi.header.take() {
        chunk_delete(chunk);
    }
    if let Some(chunk) = wpi.alpha.take() {
        chunk_delete(chunk);
    }
    if let Some(chunk) = wpi.img.take() {
        chunk_delete(chunk);
    }
    chunk_list_delete(&mut wpi.unknown);

    mux_image_init(wpi);
}

//------------------------------------------------------------------------------
// MuxImage search methods.

// upstream: src/mux/muxinternal.c GetChunkListFromId
/// Get a reference to appropriate chunk slot within an image given chunk id;
/// `None` (C: `NULL`) for the ids of other chunks.
fn get_chunk_list_from_id<'w, 'a>(
    wpi: &'w WebPMuxImage<'a>,
    id: WebPChunkId,
) -> Option<&'w Option<WebPChunk<'a>>> {
    match id {
        WebPChunkId::WEBP_CHUNK_ANMF => Some(&wpi.header),
        WebPChunkId::WEBP_CHUNK_ALPHA => Some(&wpi.alpha),
        WebPChunkId::WEBP_CHUNK_IMAGE => Some(&wpi.img),
        _ => None,
    }
}

// upstream: src/mux/muxinternal.c MuxImageCount
/// Count number of images matching the given tag id in the `wpi_list`. If
/// `id == WEBP_CHUNK_NIL`, all images will be matched.
///
/// # Panics
///
/// Panics for an id of a non-image chunk other than `WEBP_CHUNK_NIL`, where C
/// dereferences `NULL`; no caller passes one.
#[must_use]
pub fn mux_image_count(wpi_list: &[WebPMuxImage<'_>], id: WebPChunkId) -> i32 {
    let mut count = 0;
    for current in wpi_list {
        if id == WebPChunkId::WEBP_CHUNK_NIL {
            count += 1; // Special case: count all images.
        } else {
            let wpi_chunk = get_chunk_list_from_id(current, id)
                .expect("MuxImageCount takes the id of an image chunk");
            if let Some(wpi_chunk) = wpi_chunk {
                let wpi_chunk_id = chunk_get_id_from_tag(wpi_chunk.tag);
                if wpi_chunk_id == id {
                    count += 1; // Count images with a matching 'id'.
                }
            }
        }
    }
    count
}

// upstream: src/mux/muxinternal.c SearchImageToGetOrDelete
/// Returns the position of the nth image (C outputs a pointer to the previous
/// image's `next` field), or `None` if it was not found.
fn search_image_to_get_or_delete(wpi_list: &[WebPMuxImage<'_>], nth: u32) -> Option<usize> {
    let mut nth = nth;
    let mut count = 0_u32;

    if nth == 0 {
        nth = mux_image_count(wpi_list, WebPChunkId::WEBP_CHUNK_NIL).cast_unsigned();
        if nth == 0 {
            return None; // Not found.
        }
    }

    for position in 0..wpi_list.len() {
        count += 1;
        if count == nth {
            return Some(position); // Found.
        }
    }
    None // Not found.
}

//------------------------------------------------------------------------------
// MuxImage writer methods.

// upstream: src/mux/muxinternal.c MuxImagePush
/// Pushes `wpi` at the end of `wpi_list`. On success the image moves into the
/// list and `wpi` is left initialized (C copies the struct; its callers then
/// initialize their copy or keep it without releasing it).
///
/// # Errors
///
/// `WEBP_MUX_MEMORY_ERROR` when the host refuses the memory for the element.
pub fn mux_image_push<'a>(
    wpi: &mut WebPMuxImage<'a>,
    wpi_list: &mut Vec<WebPMuxImage<'a>>,
) -> Result<(), WebPMuxError> {
    if wpi_list.try_reserve(1).is_err() {
        return Err(WebPMuxError::WEBP_MUX_MEMORY_ERROR);
    }
    wpi_list.push(core::mem::take(wpi));
    Ok(())
}

//------------------------------------------------------------------------------
// MuxImage deletion methods.

// upstream: src/mux/muxinternal.c MuxImageDelete
/// Delete image `wpi`. C returns the next image; the caller removes the image
/// from its vector and passes it here.
pub fn mux_image_delete(wpi: WebPMuxImage<'_>) {
    let mut wpi = wpi;
    mux_image_release(&mut wpi);
}

// upstream: src/mux/muxinternal.c MuxImageDeleteNth
/// Delete nth image in the image list.
///
/// # Errors
///
/// `WEBP_MUX_NOT_FOUND` when there is no nth image.
pub fn mux_image_delete_nth(
    wpi_list: &mut Vec<WebPMuxImage<'_>>,
    nth: u32,
) -> Result<(), WebPMuxError> {
    let Some(position) = search_image_to_get_or_delete(wpi_list, nth) else {
        return Err(WebPMuxError::WEBP_MUX_NOT_FOUND);
    };
    mux_image_delete(wpi_list.remove(position));
    Ok(())
}

//------------------------------------------------------------------------------
// MuxImage reader methods.

// upstream: src/mux/muxinternal.c MuxImageGetNth
/// Get the position of the nth image in the image list (C outputs a pointer to
/// it).
///
/// # Errors
///
/// `WEBP_MUX_NOT_FOUND` when there is no nth image.
pub fn mux_image_get_nth(wpi_list: &[WebPMuxImage<'_>], nth: u32) -> Result<usize, WebPMuxError> {
    search_image_to_get_or_delete(wpi_list, nth).ok_or(WebPMuxError::WEBP_MUX_NOT_FOUND)
}

//------------------------------------------------------------------------------
// MuxImage serialization methods.

// upstream: src/mux/muxinternal.c MuxImageDiskSize
/// Size of an image.
#[must_use]
pub fn mux_image_disk_size(wpi: &WebPMuxImage<'_>) -> usize {
    let mut size = 0_usize;
    if let Some(header) = &wpi.header {
        size += chunk_disk_size(header);
    }
    if let Some(alpha) = &wpi.alpha {
        size += chunk_disk_size(alpha);
    }
    if let Some(img) = &wpi.img {
        size += chunk_disk_size(img);
    }
    if !wpi.unknown.is_empty() {
        size += chunk_list_disk_size(&wpi.unknown);
    }
    size
}

// upstream: src/mux/muxinternal.c ChunkEmitSpecial
/// Special case as ANMF chunk encapsulates other image chunks.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's (uint32_t) conversions"
)]
fn chunk_emit_special(
    header: &WebPChunk<'_>,
    total_size: usize,
    dst: &mut [u8],
    pos: usize,
) -> usize {
    let header_size = header.data.bytes.len();
    let offset_to_next = total_size.wrapping_sub(CHUNK_HEADER_SIZE as usize);
    debug_assert_eq!(header.tag, chunk_info(CHUNK_INDEX::IDX_ANMF).tag);
    put_le32(&mut dst[pos..], header.tag);
    put_le32(&mut dst[pos + TAG_SIZE as usize..], offset_to_next as u32);
    debug_assert_eq!(header_size, header_size as u32 as usize);
    let payload = pos + CHUNK_HEADER_SIZE as usize;
    dst[payload..payload + header_size].copy_from_slice(&header.data.bytes);
    if header_size & 1 != 0 {
        dst[payload + header_size] = 0; // Add padding.
    }
    pos + chunk_disk_size(header)
}

// upstream: src/mux/muxinternal.c MuxImageEmit
/// Write out the given image at `dst[pos..]`; returns the position after it.
#[must_use]
pub fn mux_image_emit(wpi: &WebPMuxImage<'_>, dst: &mut [u8], pos: usize) -> usize {
    // Ordering of chunks to be emitted is strictly as follows:
    // 1. ANMF chunk (if present).
    // 2. ALPH chunk (if present).
    // 3. VP8/VP8L chunk.
    let mut pos = pos;
    if let Some(header) = &wpi.header {
        pos = chunk_emit_special(header, mux_image_disk_size(wpi), dst, pos);
    }
    if let Some(alpha) = &wpi.alpha {
        pos = chunk_emit(alpha, dst, pos);
    }
    if let Some(img) = &wpi.img {
        pos = chunk_emit(img, dst, pos);
    }
    if !wpi.unknown.is_empty() {
        pos = chunk_list_emit(&wpi.unknown, dst, pos);
    }
    pos
}

//------------------------------------------------------------------------------
// Helper methods for mux.

// upstream: src/mux/muxinternal.c MuxHasAlpha
/// Checks if the given image list contains at least one image with alpha.
#[must_use]
pub fn mux_has_alpha(images: &[WebPMuxImage<'_>]) -> bool {
    images.iter().any(|image| image.has_alpha)
}

// upstream: src/mux/muxinternal.c MuxEmitRiffHeader
/// Write out RIFF header into `data`, given total data size `size`; returns the
/// position after it.
#[expect(clippy::cast_possible_truncation, reason = "C's (uint32_t) conversion")]
#[must_use]
pub fn mux_emit_riff_header(data: &mut [u8], size: usize) -> usize {
    put_le32(data, mkfourcc(b'R', b'I', b'F', b'F'));
    put_le32(
        &mut data[TAG_SIZE as usize..],
        (size as u32).wrapping_sub(CHUNK_HEADER_SIZE as u32),
    );
    debug_assert_eq!(size, size as u32 as usize);
    put_le32(
        &mut data[(TAG_SIZE + CHUNK_SIZE_BYTES) as usize..],
        mkfourcc(b'W', b'E', b'B', b'P'),
    );
    RIFF_HEADER_SIZE as usize
}

// upstream: src/mux/muxinternal.c MuxGetChunkListFromId
/// Returns the list where chunk with given ID is to be inserted in mux.
#[must_use]
pub fn mux_get_chunk_list_from_id<'m, 'a>(
    mux: &'m WebPMux<'a>,
    id: WebPChunkId,
) -> &'m Vec<WebPChunk<'a>> {
    match id {
        WebPChunkId::WEBP_CHUNK_VP8X => &mux.vp8x,
        WebPChunkId::WEBP_CHUNK_ICCP => &mux.iccp,
        WebPChunkId::WEBP_CHUNK_ANIM => &mux.anim,
        WebPChunkId::WEBP_CHUNK_EXIF => &mux.exif,
        WebPChunkId::WEBP_CHUNK_XMP => &mux.xmp,
        _ => &mux.unknown,
    }
}

// upstream: src/mux/muxinternal.c MuxGetChunkListFromId
/// [`mux_get_chunk_list_from_id`] for the callers that change the list (C
/// returns a `WebPChunk**` from a `const WebPMux*`).
pub fn mux_get_chunk_list_from_id_mut<'m, 'a>(
    mux: &'m mut WebPMux<'a>,
    id: WebPChunkId,
) -> &'m mut Vec<WebPChunk<'a>> {
    match id {
        WebPChunkId::WEBP_CHUNK_VP8X => &mut mux.vp8x,
        WebPChunkId::WEBP_CHUNK_ICCP => &mut mux.iccp,
        WebPChunkId::WEBP_CHUNK_ANIM => &mut mux.anim,
        WebPChunkId::WEBP_CHUNK_EXIF => &mut mux.exif,
        WebPChunkId::WEBP_CHUNK_XMP => &mut mux.xmp,
        _ => &mut mux.unknown,
    }
}

// upstream: src/mux/muxinternal.c IsNotCompatible
fn is_not_compatible(feature: u32, num_items: i32) -> bool {
    (feature != 0) != (num_items > 0)
}

// upstream: src/mux/muxinternal.c NO_FLAG
const NO_FLAG: WebPFeatureFlags = WebPFeatureFlags(0);

// upstream: src/mux/muxinternal.c ValidateChunk
/// Test basic constraints: retrieval, maximum number of chunks by index (use
/// -1 to skip) and feature incompatibility (use `NO_FLAG` to skip). On success
/// returns the chunk count (C: `WEBP_MUX_OK` and `*num`).
fn validate_chunk(
    mux: &WebPMux<'_>,
    idx: CHUNK_INDEX,
    feature: WebPFeatureFlags,
    vp8x_flags: u32,
    max: i32,
) -> Result<i32, WebPMuxError> {
    let num = webp_mux_num_chunks(mux, chunk_info(idx).id)?;
    if max > -1 && num > max {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    if feature != NO_FLAG && is_not_compatible(vp8x_flags & feature.0, num) {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    Ok(num)
}

// upstream: src/mux/muxinternal.c MuxValidate
/// Validates the given mux object. C first returns `WEBP_MUX_INVALID_ARGUMENT`
/// for a `NULL` mux, which no caller passes.
///
/// # Errors
///
/// The first constraint that fails, as C reports it.
pub fn mux_validate(mux: &WebPMux<'_>) -> Result<(), WebPMuxError> {
    // Verify mux has at least one image.
    if mux.images.is_empty() {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    let flags = webp_mux_get_features(mux)?;

    // At most one color profile chunk.
    let _num_iccp = validate_chunk(
        mux,
        CHUNK_INDEX::IDX_ICCP,
        WebPFeatureFlags::ICCP_FLAG,
        flags,
        1,
    )?;

    // At most one EXIF metadata.
    let _num_exif = validate_chunk(
        mux,
        CHUNK_INDEX::IDX_EXIF,
        WebPFeatureFlags::EXIF_FLAG,
        flags,
        1,
    )?;

    // At most one XMP metadata.
    let _num_xmp = validate_chunk(
        mux,
        CHUNK_INDEX::IDX_XMP,
        WebPFeatureFlags::XMP_FLAG,
        flags,
        1,
    )?;

    // Animation: ANIMATION_FLAG, ANIM chunk and ANMF chunk(s) are consistent.
    // At most one ANIM chunk.
    let num_anim = validate_chunk(mux, CHUNK_INDEX::IDX_ANIM, NO_FLAG, flags, 1)?;
    let num_frames = validate_chunk(mux, CHUNK_INDEX::IDX_ANMF, NO_FLAG, flags, -1)?;

    {
        let has_animation = flags & WebPFeatureFlags::ANIMATION_FLAG.0 != 0;
        if has_animation && (num_anim == 0 || num_frames == 0) {
            return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
        }
        if !has_animation && (num_anim == 1 || num_frames > 0) {
            return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
        }
        if !has_animation {
            let images = &mux.images;
            // There can be only one image.
            if images.len() != 1 {
                return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
            }
            // Size must match.
            if mux.canvas_width > 0
                && (images[0].width != mux.canvas_width || images[0].height != mux.canvas_height)
            {
                return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
            }
        }
    }

    // Verify either VP8X chunk is present OR there is only one elem in
    // mux->images.
    let num_vp8x = validate_chunk(mux, CHUNK_INDEX::IDX_VP8X, NO_FLAG, flags, 1)?;
    let num_images = validate_chunk(mux, CHUNK_INDEX::IDX_VP8, NO_FLAG, flags, -1)?;
    if num_vp8x == 0 && num_images != 1 {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    // ALPHA_FLAG & alpha chunk(s) are consistent.
    // Note: ALPHA_FLAG can be set when there is actually no Alpha data present.
    if mux_has_alpha(&mux.images) {
        if num_vp8x > 0 {
            // VP8X chunk is present, so it should contain ALPHA_FLAG.
            if flags & WebPFeatureFlags::ALPHA_FLAG.0 == 0 {
                return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
            }
        } else {
            // VP8X chunk is not present, so ALPH chunks should NOT be present
            // either.
            let num_alpha = webp_mux_num_chunks(mux, WebPChunkId::WEBP_CHUNK_ALPHA)?;
            if num_alpha > 0 {
                return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
            }
        }
    }

    Ok(())
}
