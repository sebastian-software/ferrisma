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

//! C ABI export of `src/webp/demux.h` from libwebp v1.6.0.
//!
//! Mirrors the header's structs with `repr(C)` and converts them to the
//! safe representations at the boundary; see
//! `docs/adr/0004-c-abi-boundary.md`.
//!
//! Work package: WP14 (animation decode and demux ABI).
//!
//! # Objects
//!
//! - The translation's demuxer borrows the `WebPData` value that it parses
//!   (WP8). The C caller's `WebPData` struct need not outlive the demuxer, only
//!   its bytes, which `demux.h` requires to stay valid and unchanged until
//!   `WebPDemuxDelete()` ("`WebPDemuxer` keeps internal pointers to 'data'
//!   memory segment"). `WebPDemuxInternal()` therefore leaks a `Box<WebPData<'static>>`
//!   that borrows the caller's bytes under that contract, creates the demuxer
//!   from it and returns a heap handle whose first field is the demuxer; the
//!   handle also keeps the data box, and `WebPDemuxDelete()` drops the demuxer
//!   before it frees the box. The animation decoder is kept the same way
//!   (`WebPAnimDecoderNew()` requires the bitstream to "remain unchanged during
//!   the lifetime of the output `WebPAnimDecoder` object").
//! - A C `WebPDemuxer*` points at the translation's demuxer itself, whether it
//!   comes from `WebPDemuxInternal()` or from `WebPAnimDecoderGetDemuxer()`, so
//!   the demux functions accept both. The demuxer is immutable after
//!   construction, so the pointers stay valid while their owner lives.
//! - `WebPIterator` and `WebPChunkIterator` are mirrored with C's field order;
//!   `private_` is the demuxer pointer. `Next` and `Prev` rebuild the Rust
//!   iterator from the C struct and write back the fields that C writes. The
//!   chunk tag that C reads from the four bytes before `chunk.bytes` is taken
//!   from the demuxer's input at that position.
//! - `WebPAnimDecoderOptions` and `WebPAnimInfo` have no pointers and are shared
//!   with `src/webp/demux.rs`; `WebPDemuxState` and `WebPFormatFeature` are WP1's
//!   newtypes over C's enum type. `tests/wp14_abi_layout.rs` checks the layouts.
//!
//! # Restrictions
//!
//! The caller's bytes are borrowed as an immutable Rust slice for the lifetime
//! of the demuxer or the animation decoder, so they must not change while it
//! lives, which the header asks for; C itself would only read the changed
//! bytes. The canvas that `WebPAnimDecoderGetNext()` returns may be read until
//! the next call on the decoder, as the header says.

use core::ffi::{c_char, c_int, c_void};
use core::mem::ManuallyDrop;
use core::ptr;
use core::slice;
use std::borrow::Cow;

use crate::demux::anim_decode::{self as anim, WebPAnimDecoder as RsAnimDecoder};
use crate::demux::demux::{self as dmx, WebPDemuxer as RsDemuxer};
use crate::webp::demux::{
    WEBP_DEMUX_ABI_VERSION, WebPAnimDecoderOptions, WebPAnimInfo,
    WebPChunkIterator as RsChunkIterator, WebPDemuxState, WebPFormatFeature,
    WebPIterator as RsIterator,
};
use crate::webp::mux_types::{WebPData as RsData, WebPMuxAnimBlend, WebPMuxAnimDispose};
use crate::webp::types::webp_abi_is_incompatible;

// upstream: src/webp/format_constants.h CHUNK_HEADER_SIZE
const CHUNK_HEADER_SIZE: usize = 8;
// upstream: src/webp/format_constants.h TAG_SIZE
const TAG_SIZE: usize = 4;

//------------------------------------------------------------------------------
// Mirrors of the structs with pointers

// upstream: src/webp/mux_types.h WebPData
/// Data type used to describe 'raw' data, e.g., chunk data (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPData {
    /// The bytes.
    pub bytes: *const u8,
    /// Their number.
    pub size: usize,
}

// upstream: src/webp/demux.h WebPIterator
/// Iterator over the frames of a demuxer (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPIterator {
    /// The frame number of this frame.
    pub frame_num: c_int,
    /// Equivalent to `WEBP_FF_FRAME_COUNT`.
    pub num_frames: c_int,
    /// Offset relative to the canvas.
    pub x_offset: c_int,
    /// Offset relative to the canvas.
    pub y_offset: c_int,
    /// Dimensions of this frame.
    pub width: c_int,
    /// Dimensions of this frame.
    pub height: c_int,
    /// Display duration in milliseconds.
    pub duration: c_int,
    /// Dispose method for the frame.
    pub dispose_method: WebPMuxAnimDispose,
    /// True if `fragment` contains a full frame.
    pub complete: c_int,
    /// The frame given by `frame_num`.
    pub fragment: WebPData,
    /// True if the frame contains transparency.
    pub has_alpha: c_int,
    /// Blend operation for the frame.
    pub blend_method: WebPMuxAnimBlend,
    /// Padding for later use.
    pub pad: [u32; 2],
    /// The demuxer (C: for internal use only).
    pub private_: *mut c_void,
}

// upstream: src/webp/demux.h WebPChunkIterator
/// Iterator over the chunks with one tag of a demuxer (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPChunkIterator {
    /// The current chunk number with the tag.
    pub chunk_num: c_int,
    /// The number of chunks with the tag.
    pub num_chunks: c_int,
    /// The payload of the chunk.
    pub chunk: WebPData,
    /// Padding for later use.
    pub pad: [u32; 6],
    /// The demuxer (C: for internal use only).
    pub private_: *mut c_void,
}

/// The object behind a C `WebPDemuxer*`: the translation's demuxer over the
/// caller's bytes (C: `struct WebPDemuxer`, opaque in the header).
pub type WebPDemuxer = RsDemuxer<'static>;

/// The heap object that `WebPDemuxInternal()` returns. `dmux` comes first, so a
/// pointer to the handle is a pointer to its demuxer.
#[repr(C)]
struct DemuxHandle {
    /// The demuxer; dropped by `WebPDemuxDelete()` before `data`.
    dmux: ManuallyDrop<RsDemuxer<'static>>,
    /// The leaked box that the demuxer borrows.
    data: *mut RsData<'static>,
}

/// The object behind a C `WebPAnimDecoder*` (C: `struct WebPAnimDecoder`).
pub struct WebPAnimDecoder {
    /// The translation's decoder; dropped by `WebPAnimDecoderDelete()` before
    /// `data`.
    dec: ManuallyDrop<RsAnimDecoder<'static>>,
    /// The leaked box that the decoder's demuxer borrows.
    data: *mut RsData<'static>,
}

//------------------------------------------------------------------------------
// Helpers

/// Writes an optional output parameter.
///
/// # Safety
///
/// `ptr` is null or points to a writable `T`.
unsafe fn set<T>(ptr: *mut T, value: T) {
    if !ptr.is_null() {
        // SAFETY: a valid, writable `T` (caller's contract).
        unsafe { ptr.write(value) };
    }
}

/// Leaks a `WebPData` over the caller's bytes, for an object that keeps it until
/// it is deleted.
///
/// # Safety
///
/// `bytes` is not null and points to `size` bytes that stay readable and
/// unchanged until the box is freed (the header's contract).
unsafe fn leak_data(bytes: *const u8, size: usize) -> *mut RsData<'static> {
    // SAFETY: the caller's bytes, valid and unchanged for the object's lifetime
    // under `demux.h`; the object frees the box before the bytes may go away.
    let bytes: &'static [u8] = unsafe { slice::from_raw_parts(bytes, size) };
    Box::into_raw(Box::new(RsData {
        bytes: Cow::Borrowed(bytes),
    }))
}

/// The demuxer behind a C `WebPDemuxer*`, or `None` for null.
///
/// # Safety
///
/// `dmux` is null or a live demuxer of this library.
unsafe fn demuxer<'d>(dmux: *const WebPDemuxer) -> Option<&'d RsDemuxer<'static>> {
    // SAFETY: null or a live demuxer (caller's contract), which is never written
    // after construction.
    unsafe { dmux.as_ref() }
}

/// A zeroed C iterator over `dmux` (C: `memset(iter, 0, sizeof(*iter))` and
/// `iter->private_ = (void*)dmux`).
fn zeroed_iterator(dmux: *const WebPDemuxer) -> WebPIterator {
    WebPIterator {
        frame_num: 0,
        num_frames: 0,
        x_offset: 0,
        y_offset: 0,
        width: 0,
        height: 0,
        duration: 0,
        dispose_method: WebPMuxAnimDispose(0),
        complete: 0,
        fragment: WebPData {
            bytes: ptr::null(),
            size: 0,
        },
        has_alpha: 0,
        blend_method: WebPMuxAnimBlend(0),
        pad: [0; 2],
        private_: dmux.cast_mut().cast(),
    }
}

/// A zeroed C chunk iterator over `dmux`.
fn zeroed_chunk_iterator(dmux: *const WebPDemuxer) -> WebPChunkIterator {
    WebPChunkIterator {
        chunk_num: 0,
        num_chunks: 0,
        chunk: WebPData {
            bytes: ptr::null(),
            size: 0,
        },
        pad: [0; 6],
        private_: dmux.cast_mut().cast(),
    }
}

/// The C view of a Rust byte slice that borrows the caller's bytes.
fn data_to_c(data: &RsData<'_>) -> WebPData {
    WebPData {
        bytes: data.bytes.as_ptr(),
        size: data.bytes.len(),
    }
}

/// Writes the fields that C's `SynthesizeFrame` writes.
fn iterator_to_c(it: &RsIterator<'_>, c: &mut WebPIterator) {
    c.frame_num = it.frame_num;
    c.num_frames = it.num_frames;
    c.x_offset = it.x_offset;
    c.y_offset = it.y_offset;
    c.width = it.width;
    c.height = it.height;
    c.has_alpha = it.has_alpha;
    c.duration = it.duration;
    c.dispose_method = it.dispose_method;
    c.blend_method = it.blend_method;
    c.complete = it.complete;
    c.fragment = data_to_c(&it.fragment);
}

/// The Rust iterator over `dmux` at the C iterator's frame, as `Next` and `Prev`
/// read it: only `frame_num` and the demuxer matter to them.
fn iterator_to_rust<'d>(c: &WebPIterator, dmux: &'d RsDemuxer<'d>) -> RsIterator<'d> {
    RsIterator {
        frame_num: c.frame_num,
        num_frames: c.num_frames,
        x_offset: c.x_offset,
        y_offset: c.y_offset,
        width: c.width,
        height: c.height,
        duration: c.duration,
        dispose_method: c.dispose_method,
        complete: c.complete,
        fragment: RsData::default(),
        has_alpha: c.has_alpha,
        blend_method: c.blend_method,
        pad: c.pad,
        dmux,
    }
}

/// Writes the fields that C's `SetChunk` writes.
fn chunk_iterator_to_c(it: &RsChunkIterator<'_>, c: &mut WebPChunkIterator) {
    c.chunk = data_to_c(&it.chunk);
    c.num_chunks = it.num_chunks;
    c.chunk_num = it.chunk_num;
}

/// The tag that C reads at `chunk.bytes - CHUNK_HEADER_SIZE`, taken from the
/// demuxer's input. `None` when those four bytes are not inside the input, which
/// an iterator that `WebPDemuxGetChunk()` filled never gives: a zeroed iterator
/// (C computes the tag address from `NULL`) or a changed `chunk.bytes` make C
/// read outside the input, which is undefined (`docs/status/wp14.md`).
fn chunk_tag(c: &WebPChunkIterator, dmux: &RsDemuxer<'_>) -> Option<[u8; TAG_SIZE]> {
    let input = dmux.input();
    if c.chunk.bytes.is_null() {
        return None;
    }
    let offset = (c.chunk.bytes as usize).wrapping_sub(input.as_ptr() as usize);
    if offset < CHUNK_HEADER_SIZE || offset > input.len() {
        return None;
    }
    let start = offset - CHUNK_HEADER_SIZE;
    input[start..start + TAG_SIZE].try_into().ok()
}

/// The Rust chunk iterator over `dmux` at the C iterator's chunk with `fourcc`.
fn chunk_iterator_to_rust<'d>(
    c: &WebPChunkIterator,
    dmux: &'d RsDemuxer<'d>,
    fourcc: [u8; TAG_SIZE],
) -> RsChunkIterator<'d> {
    RsChunkIterator {
        chunk_num: c.chunk_num,
        num_chunks: c.num_chunks,
        chunk: RsData::default(),
        pad: c.pad,
        dmux,
        fourcc,
    }
}

//------------------------------------------------------------------------------
// Demux API

// upstream: src/demux/demux.c WebPGetDemuxVersion
/// Returns the version number of the demux library, packed in hexadecimal using
/// 8 bits for each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[unsafe(export_name = "WebPGetDemuxVersion")]
#[must_use]
pub extern "C" fn webp_get_demux_version() -> c_int {
    dmx::webp_get_demux_version()
}

// upstream: src/demux/demux.c WebPDemuxInternal
/// Internal, version-checked, entry point of `WebPDemux()` and
/// `WebPDemuxPartial()`.
///
/// # Safety
///
/// `data` is null or a valid `WebPData` whose bytes stay readable and unchanged
/// until `WebPDemuxDelete()`; `state` is null or writable.
#[unsafe(export_name = "WebPDemuxInternal")]
pub unsafe extern "C" fn webp_demux_internal(
    data: *const WebPData,
    allow_partial: c_int,
    state: *mut WebPDemuxState,
    version: c_int,
) -> *mut WebPDemuxer {
    // SAFETY: null or writable (caller's contract).
    unsafe { set(state, WebPDemuxState::WEBP_DEMUX_PARSE_ERROR) };

    if webp_abi_is_incompatible(version, WEBP_DEMUX_ABI_VERSION) {
        return ptr::null_mut();
    }
    // SAFETY: null or a valid `WebPData` (caller's contract).
    let Some(data) = (unsafe { data.as_ref() }) else {
        return ptr::null_mut();
    };
    if data.bytes.is_null() || data.size == 0 {
        return ptr::null_mut();
    }

    // SAFETY: the caller's bytes, valid and unchanged until WebPDemuxDelete
    // (the header's contract), which frees the box.
    let rs_data = unsafe { leak_data(data.bytes, data.size) };
    // SAFETY: the box leaked above, freed only after the demuxer is dropped.
    let rs_data_ref: &'static RsData<'static> = unsafe { &*rs_data };
    let (dmux, rs_state) = dmx::webp_demux_internal(rs_data_ref, allow_partial != 0, version);
    // SAFETY: null or writable (caller's contract).
    unsafe { set(state, rs_state) };
    let Some(dmux) = dmux else {
        // SAFETY: the box leaked above; no demuxer borrows it.
        drop(unsafe { Box::from_raw(rs_data) });
        return ptr::null_mut();
    };
    let handle = Box::into_raw(Box::new(DemuxHandle {
        dmux: ManuallyDrop::new(dmux),
        data: rs_data,
    }));
    // The demuxer is the handle's first field (`repr(C)`).
    handle.cast()
}

// upstream: src/demux/demux.c WebPDemuxDelete
/// Frees memory associated with `dmux`.
///
/// # Safety
///
/// `dmux` is null or a demuxer that `WebPDemuxInternal()` returned and that is
/// not used again (not one of `WebPAnimDecoderGetDemuxer()`).
#[unsafe(export_name = "WebPDemuxDelete")]
pub unsafe extern "C" fn webp_demux_delete(dmux: *mut WebPDemuxer) {
    if dmux.is_null() {
        return;
    }
    // SAFETY: a handle of WebPDemuxInternal, whose first field is the demuxer,
    // which the caller gives up.
    let mut handle = unsafe { Box::from_raw(dmux.cast::<DemuxHandle>()) };
    // SAFETY: dropped once, before the box that it borrows.
    unsafe { ManuallyDrop::drop(&mut handle.dmux) };
    // SAFETY: the box leaked by WebPDemuxInternal, no longer borrowed.
    drop(unsafe { Box::from_raw(handle.data) });
}

// upstream: src/demux/demux.c WebPDemuxGetI
/// Gets the `feature` value from the `dmux`. Returns 0 for a null demuxer or an
/// unknown feature.
///
/// # Safety
///
/// `dmux` is null or a live demuxer of this library.
#[unsafe(export_name = "WebPDemuxGetI")]
#[must_use]
pub unsafe extern "C" fn webp_demux_get_i(
    dmux: *const WebPDemuxer,
    feature: WebPFormatFeature,
) -> u32 {
    // SAFETY: null or a live demuxer (caller's contract).
    let Some(dmux) = (unsafe { demuxer(dmux) }) else {
        return 0;
    };
    dmx::webp_demux_get_i(dmux, feature)
}

// upstream: src/demux/demux.c WebPDemuxGetFrame
/// Retrieves frame `frame` from `dmux`; 0 gives the last frame. On failure the
/// iterator is zeroed with `private_` set to `dmux`, as in C.
///
/// # Safety
///
/// `dmux` is null or a live demuxer of this library; `iter` is null or
/// writable.
#[unsafe(export_name = "WebPDemuxGetFrame")]
#[must_use]
pub unsafe extern "C" fn webp_demux_get_frame(
    dmux: *const WebPDemuxer,
    frame: c_int,
    iter: *mut WebPIterator,
) -> c_int {
    if iter.is_null() {
        return 0;
    }
    let mut c = zeroed_iterator(dmux);
    let mut ok = 0;
    // SAFETY: null or a live demuxer (caller's contract).
    if let Some(rs_dmux) = unsafe { demuxer(dmux) }
        && let Some(it) = dmx::webp_demux_get_frame(rs_dmux, frame)
    {
        iterator_to_c(&it, &mut c);
        ok = 1;
    }
    // SAFETY: a writable iterator (checked non-null); it may be uninitialized,
    // so it is written whole.
    unsafe { iter.write(c) };
    ok
}

/// The common part of `WebPDemuxNextFrame()` and `WebPDemuxPrevFrame()`.
///
/// # Safety
///
/// `iter` is null or an initialized iterator whose `private_` is null or a live
/// demuxer of this library.
unsafe fn step_frame(iter: *mut WebPIterator, step: fn(&mut RsIterator<'_>) -> bool) -> c_int {
    // SAFETY: null or a valid iterator (caller's contract).
    let Some(c) = (unsafe { iter.as_mut() }) else {
        return 0;
    };
    // SAFETY: null or a live demuxer (caller's contract).
    let Some(dmux) = (unsafe { demuxer(c.private_.cast_const().cast()) }) else {
        return 0; // SetFrame: dmux == NULL
    };
    let mut it = iterator_to_rust(c, dmux);
    if !step(&mut it) {
        return 0;
    }
    iterator_to_c(&it, c);
    1
}

// upstream: src/demux/demux.c WebPDemuxNextFrame
/// Sets `iter` to the next frame. Returns false if there is none. On a zeroed
/// iterator whose `private_` is a demuxer it gives frame 1, as in C.
///
/// # Safety
///
/// `iter` is null or an initialized iterator whose `private_` is null or a live
/// demuxer of this library.
#[unsafe(export_name = "WebPDemuxNextFrame")]
#[must_use]
pub unsafe extern "C" fn webp_demux_next_frame(iter: *mut WebPIterator) -> c_int {
    // C: SetFrame(iter->frame_num + 1, iter). The `int` sum overflows for
    // INT_MAX, which is undefined; wrapped, it is negative and fails.
    // SAFETY: null or a valid iterator (caller's contract).
    if let Some(c) = unsafe { iter.as_ref() }
        && c.frame_num == c_int::MAX
    {
        return 0;
    }
    // SAFETY: the caller's contract, passed on.
    unsafe { step_frame(iter, dmx::webp_demux_next_frame) }
}

// upstream: src/demux/demux.c WebPDemuxPrevFrame
/// Sets `iter` to the previous frame. Returns false if there is none.
///
/// # Safety
///
/// `iter` is null or an initialized iterator whose `private_` is null or a live
/// demuxer of this library.
#[unsafe(export_name = "WebPDemuxPrevFrame")]
#[must_use]
pub unsafe extern "C" fn webp_demux_prev_frame(iter: *mut WebPIterator) -> c_int {
    // SAFETY: null or a valid iterator (caller's contract).
    let Some(c) = (unsafe { iter.as_ref() }) else {
        return 0;
    };
    if c.frame_num <= 1 {
        return 0;
    }
    // SAFETY: the caller's contract, passed on.
    unsafe { step_frame(iter, dmx::webp_demux_prev_frame) }
}

// upstream: src/demux/demux.c WebPDemuxReleaseIterator
/// Releases any memory associated with `iter`. C frees nothing.
#[unsafe(export_name = "WebPDemuxReleaseIterator")]
pub extern "C" fn webp_demux_release_iterator(iter: *mut WebPIterator) {
    let _ = iter;
}

// upstream: src/demux/demux.c WebPDemuxGetChunk
/// Retrieves the `chunk_number` instance of the chunk with id `fourcc` from
/// `dmux`; 0 gives the last one. On failure the iterator is zeroed with
/// `private_` set to `dmux`, as in C.
///
/// # Safety
///
/// `dmux` is null or a live demuxer of this library; `fourcc` is null or points
/// to four readable bytes; `iter` is null or writable.
#[unsafe(export_name = "WebPDemuxGetChunk")]
#[must_use]
pub unsafe extern "C" fn webp_demux_get_chunk(
    dmux: *const WebPDemuxer,
    fourcc: *const c_char,
    chunk_number: c_int,
    iter: *mut WebPChunkIterator,
) -> c_int {
    if iter.is_null() {
        return 0;
    }
    let mut c = zeroed_chunk_iterator(dmux);
    let mut ok = 0;
    // SAFETY: null or a live demuxer (caller's contract).
    if let Some(rs_dmux) = unsafe { demuxer(dmux) }
        && !fourcc.is_null()
    {
        // SAFETY: four readable bytes (caller's contract), without alignment.
        let tag = unsafe { fourcc.cast::<[u8; TAG_SIZE]>().read() };
        if let Some(it) = dmx::webp_demux_get_chunk(rs_dmux, tag, chunk_number) {
            chunk_iterator_to_c(&it, &mut c);
            ok = 1;
        }
    }
    // SAFETY: a writable iterator (checked non-null); it may be uninitialized,
    // so it is written whole.
    unsafe { iter.write(c) };
    ok
}

/// The common part of `WebPDemuxNextChunk()` and `WebPDemuxPrevChunk()`.
///
/// # Safety
///
/// `iter` is an initialized iterator whose `private_` is null or a live demuxer
/// of this library.
unsafe fn step_chunk(
    c: &mut WebPChunkIterator,
    step: fn(&mut RsChunkIterator<'_>) -> bool,
) -> c_int {
    // SAFETY: null or a live demuxer (caller's contract).
    let Some(dmux) = (unsafe { demuxer(c.private_.cast_const().cast()) }) else {
        return 0; // SetChunk: dmux == NULL
    };
    let Some(fourcc) = chunk_tag(c, dmux) else {
        return 0;
    };
    let mut it = chunk_iterator_to_rust(c, dmux, fourcc);
    if !step(&mut it) {
        return 0;
    }
    chunk_iterator_to_c(&it, c);
    1
}

// upstream: src/demux/demux.c WebPDemuxNextChunk
/// Sets `iter` to the next chunk with the same id. Returns false if there is
/// none.
///
/// # Safety
///
/// `iter` is null or an iterator that `WebPDemuxGetChunk()` filled, whose
/// `private_` is null or a live demuxer of this library.
#[unsafe(export_name = "WebPDemuxNextChunk")]
#[must_use]
pub unsafe extern "C" fn webp_demux_next_chunk(iter: *mut WebPChunkIterator) -> c_int {
    // SAFETY: null or a valid iterator (caller's contract).
    let Some(c) = (unsafe { iter.as_mut() }) else {
        return 0;
    };
    // C: SetChunk(fourcc, iter->chunk_num + 1, iter). The `int` sum overflows
    // for INT_MAX, which is undefined; wrapped, it is negative and fails.
    if c.chunk_num == c_int::MAX {
        return 0;
    }
    // SAFETY: the caller's contract, passed on.
    unsafe { step_chunk(c, dmx::webp_demux_next_chunk) }
}

// upstream: src/demux/demux.c WebPDemuxPrevChunk
/// Sets `iter` to the previous chunk with the same id. Returns false if there is
/// none.
///
/// # Safety
///
/// `iter` is null or an iterator that `WebPDemuxGetChunk()` filled, whose
/// `private_` is null or a live demuxer of this library.
#[unsafe(export_name = "WebPDemuxPrevChunk")]
#[must_use]
pub unsafe extern "C" fn webp_demux_prev_chunk(iter: *mut WebPChunkIterator) -> c_int {
    // SAFETY: null or a valid iterator (caller's contract).
    let Some(c) = (unsafe { iter.as_mut() }) else {
        return 0;
    };
    if c.chunk_num <= 1 {
        return 0;
    }
    // SAFETY: the caller's contract, passed on.
    unsafe { step_chunk(c, dmx::webp_demux_prev_chunk) }
}

// upstream: src/demux/demux.c WebPDemuxReleaseChunkIterator
/// Releases any memory associated with `iter`. C frees nothing.
#[unsafe(export_name = "WebPDemuxReleaseChunkIterator")]
pub extern "C" fn webp_demux_release_chunk_iterator(iter: *mut WebPChunkIterator) {
    let _ = iter;
}

//------------------------------------------------------------------------------
// WebPAnimDecoder API

// upstream: src/demux/anim_decode.c WebPAnimDecoderOptionsInitInternal
/// Internal, version-checked, entry point of `WebPAnimDecoderOptionsInit()`.
/// Writes `color_mode` and `use_threads` and leaves the padding, as C does.
///
/// # Safety
///
/// `dec_options` is null or writable; its padding may be uninitialized.
#[unsafe(export_name = "WebPAnimDecoderOptionsInitInternal")]
#[must_use]
pub unsafe extern "C" fn webp_anim_decoder_options_init_internal(
    dec_options: *mut WebPAnimDecoderOptions,
    abi_version: c_int,
) -> c_int {
    if dec_options.is_null() {
        return 0;
    }
    let mut options = WebPAnimDecoderOptions::default();
    if !anim::webp_anim_decoder_options_init_internal(&mut options, abi_version) {
        return 0;
    }
    // SAFETY: the address of a field of the caller's structure (checked
    // non-null); nothing is read.
    let color_mode = unsafe { &raw mut (*dec_options).color_mode };
    // SAFETY: as above.
    let use_threads = unsafe { &raw mut (*dec_options).use_threads };
    // SAFETY: a writable field; only the two fields that C writes are written,
    // so uninitialized padding stays as it is.
    unsafe { color_mode.write(options.color_mode) };
    // SAFETY: as above.
    unsafe { use_threads.write(options.use_threads) };
    1
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderNewInternal
/// Internal, version-checked, entry point of `WebPAnimDecoderNew()`.
///
/// # Safety
///
/// `webp_data` is null or a valid `WebPData` whose bytes stay readable and
/// unchanged until `WebPAnimDecoderDelete()`; `dec_options` is null or
/// readable (its padding is not read).
#[unsafe(export_name = "WebPAnimDecoderNewInternal")]
#[must_use]
pub unsafe extern "C" fn webp_anim_decoder_new_internal(
    webp_data: *const WebPData,
    dec_options: *const WebPAnimDecoderOptions,
    abi_version: c_int,
) -> *mut WebPAnimDecoder {
    // SAFETY: null or a valid `WebPData` (caller's contract).
    let Some(data) = (unsafe { webp_data.as_ref() }) else {
        return ptr::null_mut();
    };
    if webp_abi_is_incompatible(abi_version, WEBP_DEMUX_ABI_VERSION) {
        return ptr::null_mut();
    }
    // C: WebPGetFeatures(NULL, ...) fails, so does a NULL bitstream here.
    if data.bytes.is_null() {
        return ptr::null_mut();
    }
    // C copies the options (`options = *dec_options`); only the color mode and
    // the threads flag are read, so the padding is not copied.
    let options = if dec_options.is_null() {
        None
    } else {
        // SAFETY: the address of a field of the caller's structure (checked
        // non-null); nothing is read.
        let color_mode = unsafe { &raw const (*dec_options).color_mode };
        // SAFETY: as above.
        let use_threads = unsafe { &raw const (*dec_options).use_threads };
        // SAFETY: an initialized field of a readable structure (caller's
        // contract); the padding, which may be uninitialized, is not read.
        let color_mode = unsafe { color_mode.read() };
        // SAFETY: as above.
        let use_threads = unsafe { use_threads.read() };
        Some(WebPAnimDecoderOptions {
            color_mode,
            use_threads,
            padding: [0; 7],
        })
    };

    // SAFETY: the caller's bytes, valid and unchanged until
    // WebPAnimDecoderDelete (the header's contract), which frees the box.
    let rs_data = unsafe { leak_data(data.bytes, data.size) };
    // SAFETY: the box leaked above, freed only after the decoder is dropped.
    let rs_data_ref: &'static RsData<'static> = unsafe { &*rs_data };
    let Some(dec) =
        anim::webp_anim_decoder_new_internal(rs_data_ref, options.as_ref(), abi_version)
    else {
        // SAFETY: the box leaked above; no decoder borrows it.
        drop(unsafe { Box::from_raw(rs_data) });
        return ptr::null_mut();
    };
    Box::into_raw(Box::new(WebPAnimDecoder {
        dec: ManuallyDrop::new(dec),
        data: rs_data,
    }))
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderGetInfo
/// Gets global information about the animation. Returns false for null
/// arguments.
///
/// # Safety
///
/// `dec` is null or a live decoder of this library; `info` is null or writable.
#[unsafe(export_name = "WebPAnimDecoderGetInfo")]
#[must_use]
pub unsafe extern "C" fn webp_anim_decoder_get_info(
    dec: *const WebPAnimDecoder,
    info: *mut WebPAnimInfo,
) -> c_int {
    // SAFETY: null or a live decoder (caller's contract).
    let Some(dec) = (unsafe { dec.as_ref() }) else {
        return 0;
    };
    if info.is_null() {
        return 0;
    }
    // SAFETY: writable (checked non-null); written whole, as `*info = dec->info`.
    unsafe { info.write(anim::webp_anim_decoder_get_info(&dec.dec)) };
    1
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderGetNext
/// Fetches the next frame: the canvas in `*buf` and its timestamp in
/// `*timestamp`. Returns false for null arguments, a parsing or decoding error,
/// or when there are no more frames; the outputs are then not written.
///
/// # Safety
///
/// `dec` is null or a live decoder of this library; `buf` and `timestamp` are
/// null or writable.
#[unsafe(export_name = "WebPAnimDecoderGetNext")]
#[must_use]
pub unsafe extern "C" fn webp_anim_decoder_get_next(
    dec: *mut WebPAnimDecoder,
    buf: *mut *mut u8,
    timestamp: *mut c_int,
) -> c_int {
    if dec.is_null() || buf.is_null() || timestamp.is_null() {
        return 0;
    }
    // SAFETY: a live decoder (checked non-null; caller's contract). The demuxer
    // pointer that WebPAnimDecoderGetDemuxer handed out stays valid: the decoder
    // never writes its demuxer.
    let decoder = unsafe { &mut *dec };
    let Some((_, ts)) = anim::webp_anim_decoder_get_next(&mut decoder.dec) else {
        return 0;
    };
    let canvas = decoder.dec.canvas_mut().as_mut_ptr();
    // SAFETY: writable (checked non-null). The canvas stays at this address
    // until the decoder is deleted; C may read it until the next call.
    unsafe { buf.write(canvas) };
    // SAFETY: writable (checked non-null).
    unsafe { timestamp.write(ts) };
    1
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderHasMoreFrames
/// Returns true if `dec` is not null and some frames are yet to be decoded.
///
/// # Safety
///
/// `dec` is null or a live decoder of this library.
#[unsafe(export_name = "WebPAnimDecoderHasMoreFrames")]
#[must_use]
pub unsafe extern "C" fn webp_anim_decoder_has_more_frames(dec: *const WebPAnimDecoder) -> c_int {
    // SAFETY: null or a live decoder (caller's contract).
    let Some(dec) = (unsafe { dec.as_ref() }) else {
        return 0;
    };
    c_int::from(anim::webp_anim_decoder_has_more_frames(&dec.dec))
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderReset
/// Resets `dec`, so that the next `WebPAnimDecoderGetNext()` restarts from the
/// first frame. Does nothing for null.
///
/// # Safety
///
/// `dec` is null or a live decoder of this library.
#[unsafe(export_name = "WebPAnimDecoderReset")]
pub unsafe extern "C" fn webp_anim_decoder_reset(dec: *mut WebPAnimDecoder) {
    // SAFETY: null or a live decoder (caller's contract).
    if let Some(dec) = unsafe { dec.as_mut() } {
        anim::webp_anim_decoder_reset(&mut dec.dec);
    }
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderGetDemuxer
/// Grabs the internal demuxer object, owned by `dec` and valid until
/// `WebPAnimDecoderDelete()`; null for a null decoder.
///
/// # Safety
///
/// `dec` is null or a live decoder of this library.
#[unsafe(export_name = "WebPAnimDecoderGetDemuxer")]
#[must_use]
pub unsafe extern "C" fn webp_anim_decoder_get_demuxer(
    dec: *const WebPAnimDecoder,
) -> *const WebPDemuxer {
    // SAFETY: null or a live decoder (caller's contract).
    let Some(dec) = (unsafe { dec.as_ref() }) else {
        return ptr::null();
    };
    let dmux: &RsDemuxer<'static> = anim::webp_anim_decoder_get_demuxer(&dec.dec);
    dmux
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderDelete
/// Deletes the decoder object.
///
/// # Safety
///
/// `dec` is null or a decoder of this library that is not used again, and
/// neither is its demuxer.
#[unsafe(export_name = "WebPAnimDecoderDelete")]
pub unsafe extern "C" fn webp_anim_decoder_delete(dec: *mut WebPAnimDecoder) {
    if dec.is_null() {
        return;
    }
    // SAFETY: a decoder of this library, which the caller gives up.
    let mut decoder = unsafe { Box::from_raw(dec) };
    // SAFETY: dropped once, before the box that it borrows.
    unsafe { ManuallyDrop::drop(&mut decoder.dec) };
    // SAFETY: the box leaked by WebPAnimDecoderNewInternal, no longer borrowed.
    drop(unsafe { Box::from_raw(decoder.data) });
}
