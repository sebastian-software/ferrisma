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

//! C ABI export of `src/webp/mux.h` from libwebp v1.6.0.
//!
//! Mirrors the header's structs with `repr(C)` and converts them to the
//! safe representations at the boundary; see
//! `docs/adr/0004-c-abi-boundary.md`.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! # Objects
//!
//! - A C `WebPMux*` is a heap box of the translation's mux, and a
//!   `WebPAnimEncoder*` a heap box of the translation's encoder with a buffer
//!   for its error string; the `Delete` functions drop them.
//! - The translation's mux keeps the chunks that it is given without copying
//!   (`copy_data == 0`) as borrowed bytes, so the caller's bytes are lent to it
//!   as a `WebPData` that borrows them for `'static`, under the header's
//!   contract that they exist as long as the mux reads them ("it must exist for
//!   the lifetime of the mux object" for `WebPMuxCreate()`, "until a call to
//!   `WebPMuxAssemble()`" for the setters). Unlike the demux ABI no box over the
//!   caller's `WebPData` is needed: the mux borrows the bytes, not the struct.
//!   Bytes given with copying are lent only for the call, which copies them.
//! - `WebPMuxFrameInfo` is mirrored with C's field order and reuses the public
//!   `WebPData` mirror of `src/abi/demux.rs`; `WebPMuxAnimParams` and
//!   `WebPAnimEncoderOptions` have no pointers and are shared with
//!   `src/webp/mux.rs`; `WebPMuxError` and `WebPChunkId` are the newtypes of
//!   `src/webp/mux.rs` over C's enum types. `tests/wp15_abi_layout.rs` checks
//!   the layouts.
//! - Data that C allocates for the caller (`WebPMuxAssemble()`,
//!   `WebPMuxGetFrame()`, `WebPAnimEncoderAssemble()`) is a block of the
//!   platform `malloc` (`malloc_copy`), which `WebPDataClear()` releases with
//!   `WebPFree()`. Data that C returns from inside the mux
//!   (`WebPMuxGetChunk()`, `WebPAnimEncoderGetChunk()`) points into the mux; C's
//!   `NULL` bytes of an empty copied chunk are a null pointer.
//! - `WebPAnimEncoderAdd()` converts the caller's frame for the call as
//!   `src/abi/encode.rs` converts pictures (`with_picture`), lending the ARGB
//!   plane, or the YUVA planes that the conversion to ARGB reads, and writes
//!   the results back: the error code and, after a conversion, the new ARGB
//!   plane. The frame's progress hook is called with the frame's pointer, where
//!   C passes its view of the encoder's canvas (`docs/status/wp15.md`).
//! - The error string that `WebPAnimEncoderGetError()` returns lives in the
//!   encoder's box, NUL-terminated in `ERROR_STR_MAX_LENGTH` bytes, and is
//!   updated by every call that changes it, as C's string is.
//!
//! # Restrictions
//!
//! Bytes lent to a mux are an immutable Rust slice while the mux may read
//! them, so they must not change then; C would read the changed bytes. A
//! frame's progress hook must not touch the frame (WP13 deviation 15).

use core::ffi::{c_char, c_int};
use core::ptr;
use core::slice;
use std::borrow::Cow;

use crate::abi::demux::WebPData;
use crate::abi::encode::{Mode, WebPPicture, progress_closure, with_picture};
use crate::abi::types::malloc_copy;
use crate::mux::anim_encode::{self as anim, WebPAnimEncoder as RsAnimEncoder};
use crate::mux::muxedit as edit;
use crate::mux::muxi::{CHUNK_INDEX, WebPMux as RsMux};
use crate::mux::muxinternal::{self as internal, chunk_get_index_from_four_cc};
use crate::mux::muxread as read;
use crate::webp::encode::WebPConfig;
use crate::webp::format_constants::MAX_CHUNK_PAYLOAD;
use crate::webp::mux::{
    WEBP_MUX_ABI_VERSION, WebPAnimEncoderOptions, WebPChunkId, WebPMuxAnimParams, WebPMuxError,
    WebPMuxFrameInfo as RsFrameInfo,
};
use crate::webp::mux_types::{WebPData as RsData, WebPMuxAnimBlend, WebPMuxAnimDispose};
use crate::webp::types::webp_abi_is_incompatible;

// upstream: src/webp/format_constants.h MAX_IMAGE_AREA
const MAX_IMAGE_AREA: u64 = crate::webp::format_constants::MAX_IMAGE_AREA;
// upstream: src/mux/anim_encode.c ERROR_STR_MAX_LENGTH
const ERROR_STR_MAX_LENGTH: usize = 100;

//------------------------------------------------------------------------------
// Mirrors and objects

// upstream: src/webp/mux.h WebPMuxFrameInfo
/// Encapsulates data about a single frame (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPMuxFrameInfo {
    /// Image data: can be a raw VP8/VP8L bitstream or a single-image WebP file.
    pub bitstream: WebPData,
    /// x-offset of the frame.
    pub x_offset: c_int,
    /// y-offset of the frame.
    pub y_offset: c_int,
    /// Duration of the frame (in milliseconds).
    pub duration: c_int,
    /// Frame type: should be one of `WEBP_CHUNK_ANMF` or `WEBP_CHUNK_IMAGE`.
    pub id: WebPChunkId,
    /// Disposal method for the frame.
    pub dispose_method: WebPMuxAnimDispose,
    /// Blend operation for the frame.
    pub blend_method: WebPMuxAnimBlend,
    /// Padding for later use.
    pub pad: [u32; 1],
}

/// The object behind a C `WebPMux*` (C: `struct WebPMux`, opaque in the
/// header): the translation's mux over lent bytes.
pub type WebPMux = RsMux<'static>;

/// The object behind a C `WebPAnimEncoder*` (C: `struct WebPAnimEncoder`).
pub struct WebPAnimEncoder {
    /// The translation's encoder.
    enc: RsAnimEncoder<'static>,
    /// The error string as C keeps it: NUL-terminated (C: `error_str`).
    error: [c_char; ERROR_STR_MAX_LENGTH],
}

impl WebPAnimEncoder {
    /// Copies the encoder's error string into `error`, where C keeps it.
    fn sync_error(&mut self) {
        let message = anim::webp_anim_encoder_get_error(&self.enc).as_bytes();
        let length = message.len().min(ERROR_STR_MAX_LENGTH - 1);
        for (target, &byte) in self.error.iter_mut().zip(&message[..length]) {
            *target = byte.cast_signed();
        }
        self.error[length] = 0;
    }
}

//------------------------------------------------------------------------------
// Helpers

/// Writes an output parameter that the caller checked for null.
///
/// # Safety
///
/// `ptr` points to a writable `T`.
unsafe fn put<T>(ptr: *mut T, value: T) {
    // SAFETY: a valid, writable `T` (caller's contract).
    unsafe { ptr.write(value) };
}

/// The bytes of a C `WebPData` as a `WebPData` that borrows them.
///
/// # Safety
///
/// `data.bytes` is not null and points to `data.size` readable bytes, which
/// stay valid and unchanged while the translation may read them: for the call
/// when it copies them, for as long as the mux keeps them otherwise (the
/// header's contract).
unsafe fn lend(data: &WebPData) -> RsData<'static> {
    // SAFETY: readable for `size` bytes and valid while borrowed (caller's
    // contract).
    let bytes: &'static [u8] = unsafe { slice::from_raw_parts(data.bytes, data.size) };
    RsData {
        bytes: Cow::Borrowed(bytes),
    }
}

/// The C view of bytes inside a mux: a null pointer for C's `NULL` bytes (an
/// empty copy), the bytes' address otherwise.
fn data_to_c(data: &RsData<'_>) -> WebPData {
    WebPData {
        bytes: if crate::mux::muxi::data_is_null(data) {
            ptr::null()
        } else {
            data.bytes.as_ptr()
        },
        size: data.bytes.len(),
    }
}

/// Bytes that C allocates for the caller: a block of the platform `malloc`,
/// which `WebPFree()` releases; `None` when the allocation fails.
fn data_to_caller(data: &RsData<'_>) -> Option<WebPData> {
    let block = malloc_copy(&data.bytes);
    (!block.is_null()).then_some(WebPData {
        bytes: block,
        size: data.bytes.len(),
    })
}

/// The four characters at `fourcc`.
///
/// # Safety
///
/// `fourcc` is not null and points to four readable characters.
unsafe fn read_fourcc(fourcc: *const c_char) -> [u8; 4] {
    // SAFETY: four readable characters (caller's contract).
    unsafe { fourcc.cast::<[u8; 4]>().read_unaligned() }
}

/// The mux behind a C `WebPMux*`, or `None` for null.
///
/// # Safety
///
/// `mux` is null or a live mux of this library that no other reference uses.
unsafe fn mux_mut<'m>(mux: *mut WebPMux) -> Option<&'m mut RsMux<'static>> {
    // SAFETY: null or a live mux (caller's contract).
    unsafe { mux.as_mut() }
}

/// The mux behind a C `const WebPMux*`, or `None` for null.
///
/// # Safety
///
/// `mux` is null or a live mux of this library.
unsafe fn mux_ref<'m>(mux: *const WebPMux) -> Option<&'m RsMux<'static>> {
    // SAFETY: null or a live mux (caller's contract).
    unsafe { mux.as_ref() }
}

/// The encoder behind a C `WebPAnimEncoder*`, or `None` for null.
///
/// # Safety
///
/// `enc` is null or a live encoder of this library that no other reference
/// uses.
unsafe fn encoder<'e>(enc: *mut WebPAnimEncoder) -> Option<&'e mut WebPAnimEncoder> {
    // SAFETY: null or a live encoder (caller's contract).
    unsafe { enc.as_mut() }
}

/// The C code of a result.
fn code<T>(result: &Result<T, WebPMuxError>) -> WebPMuxError {
    WebPMuxError::of(result)
}

//------------------------------------------------------------------------------
// Life of a Mux object

// upstream: src/mux/muxinternal.c WebPGetMuxVersion
/// Returns the version number of the mux library, packed in hexadecimal using
/// 8 bits for each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[unsafe(export_name = "WebPGetMuxVersion")]
#[must_use]
pub extern "C" fn webp_get_mux_version() -> c_int {
    internal::webp_get_mux_version()
}

// upstream: src/mux/muxedit.c WebPNewInternal
/// Internal, version-checked, entry point of `WebPMuxNew()`: an empty mux, or
/// null for a version mismatch.
#[unsafe(export_name = "WebPNewInternal")]
#[must_use]
pub extern "C" fn webp_new_internal(version: c_int) -> *mut WebPMux {
    match edit::webp_new_internal(version) {
        Some(mux) => Box::into_raw(Box::new(mux)),
        None => ptr::null_mut(),
    }
}

// upstream: src/mux/muxedit.c WebPMuxDelete
/// Deletes the mux object.
///
/// # Safety
///
/// `mux` is null or a mux of this library that is not used again.
#[unsafe(export_name = "WebPMuxDelete")]
pub unsafe extern "C" fn webp_mux_delete(mux: *mut WebPMux) {
    if mux.is_null() {
        return;
    }
    // SAFETY: a mux of this library, which the caller gives up.
    let mux = unsafe { Box::from_raw(mux) };
    edit::webp_mux_delete(*mux);
}

//------------------------------------------------------------------------------
// Mux creation.

// upstream: src/mux/muxread.c WebPMuxCreateInternal
/// Internal, version-checked, entry point of `WebPMuxCreate()`: a mux from WebP
/// RIFF data, or null for invalid data or a memory error. With `copy_data ==
/// 0` the mux keeps pointers into the bitstream, which must exist for the
/// lifetime of the mux.
///
/// # Safety
///
/// `bitstream` is null or a valid `WebPData` whose bytes are readable; without
/// copying they stay valid and unchanged until `WebPMuxDelete()`.
#[unsafe(export_name = "WebPMuxCreateInternal")]
#[must_use]
pub unsafe extern "C" fn webp_mux_create_internal(
    bitstream: *const WebPData,
    copy_data: c_int,
    version: c_int,
) -> *mut WebPMux {
    if webp_abi_is_incompatible(version, WEBP_MUX_ABI_VERSION) {
        return ptr::null_mut(); // version mismatch
    }
    // SAFETY: null or a valid `WebPData` (caller's contract).
    let Some(bitstream) = (unsafe { bitstream.as_ref() }) else {
        return ptr::null_mut();
    };
    if bitstream.bytes.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: readable bytes, valid while the mux keeps them (caller's
    // contract).
    let data = unsafe { lend(bitstream) };
    match read::webp_mux_create_internal(&data, copy_data, version) {
        Some(mux) => Box::into_raw(Box::new(mux)),
        None => ptr::null_mut(),
    }
}

//------------------------------------------------------------------------------
// Non-image chunks.

// upstream: src/mux/muxedit.c WebPMuxSetChunk
/// Adds a chunk with id `fourcc` and data `chunk_data` in the mux object; any
/// existing chunk(s) with the same id are removed. Without copying the data
/// must exist until a call to `WebPMuxAssemble()`.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `fourcc` is null or four
/// readable characters; `chunk_data` is null or a valid `WebPData` whose
/// non-null bytes are readable for `size` when it is at most
/// `MAX_CHUNK_PAYLOAD`, and without copying stay valid and unchanged while the
/// mux keeps them.
#[unsafe(export_name = "WebPMuxSetChunk")]
#[must_use]
pub unsafe extern "C" fn webp_mux_set_chunk(
    mux: *mut WebPMux,
    fourcc: *const c_char,
    chunk_data: *const WebPData,
    copy_data: c_int,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let mux = unsafe { mux_mut(mux) };
    // SAFETY: null or a valid `WebPData` (caller's contract).
    let chunk_data = unsafe { chunk_data.as_ref() };
    let (Some(mux), Some(chunk_data)) = (mux, chunk_data) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if fourcc.is_null()
        || chunk_data.bytes.is_null()
        || chunk_data.size as u64 > u64::from(MAX_CHUNK_PAYLOAD)
    {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: four readable characters (checked non-null; caller's contract).
    let tag = unsafe { read_fourcc(fourcc) };
    // SAFETY: readable bytes of a checked size, valid while the mux keeps them
    // (caller's contract).
    let data = unsafe { lend(chunk_data) };
    code(&edit::webp_mux_set_chunk(mux, &tag, &data, copy_data))
}

// upstream: src/mux/muxread.c WebPMuxGetChunk
/// Gets a reference to the data of the chunk with id `fourcc` in the mux
/// object; the caller must not free it. On `WEBP_MUX_NOT_FOUND` for a known
/// chunk type `*chunk_data` is reset, as C's `MuxGet` does.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `fourcc` is null or four
/// readable characters; `chunk_data` is null or writable.
#[unsafe(export_name = "WebPMuxGetChunk")]
#[must_use]
pub unsafe extern "C" fn webp_mux_get_chunk(
    mux: *const WebPMux,
    fourcc: *const c_char,
    chunk_data: *mut WebPData,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_ref(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if fourcc.is_null() || chunk_data.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: four readable characters (checked non-null; caller's contract).
    let tag = unsafe { read_fourcc(fourcc) };
    // SAFETY: a writable `WebPData` (checked non-null; caller's contract).
    unsafe { get_chunk(read::webp_mux_get_chunk(mux, &tag), &tag, chunk_data) }
}

/// Writes the result of a chunk getter as C's `WebPMuxGetChunk()` does.
///
/// # Safety
///
/// `chunk_data` points to a writable `WebPData`.
unsafe fn get_chunk(
    result: Result<&RsData<'_>, WebPMuxError>,
    tag: &[u8; 4],
    chunk_data: *mut WebPData,
) -> WebPMuxError {
    match result {
        Ok(data) => {
            // SAFETY: writable (caller's contract).
            unsafe { put(chunk_data, data_to_c(data)) };
            WebPMuxError::WEBP_MUX_OK
        }
        Err(err) => {
            if err == WebPMuxError::WEBP_MUX_NOT_FOUND
                && chunk_get_index_from_four_cc(tag) != CHUNK_INDEX::IDX_UNKNOWN
            {
                // C: WebPDataInit(data) in MuxGet.
                let empty = WebPData {
                    bytes: ptr::null(),
                    size: 0,
                };
                // SAFETY: writable (caller's contract).
                unsafe { put(chunk_data, empty) };
            }
            err
        }
    }
}

// upstream: src/mux/muxedit.c WebPMuxDeleteChunk
/// Deletes the chunk with the given `fourcc` from the mux object.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `fourcc` is null or four
/// readable characters.
#[unsafe(export_name = "WebPMuxDeleteChunk")]
#[must_use]
pub unsafe extern "C" fn webp_mux_delete_chunk(
    mux: *mut WebPMux,
    fourcc: *const c_char,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_mut(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if fourcc.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: four readable characters (checked non-null; caller's contract).
    let tag = unsafe { read_fourcc(fourcc) };
    code(&edit::webp_mux_delete_chunk(mux, &tag))
}

//------------------------------------------------------------------------------
// Images.

// upstream: src/mux/muxedit.c WebPMuxSetImage
/// Sets the (non-animated) image in the mux object; any existing images
/// (including frames) are removed. Without copying the data must exist until a
/// call to `WebPMuxAssemble()`.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `bitstream` is null or a valid
/// `WebPData` whose non-null bytes are readable for `size` when it is at most
/// `MAX_CHUNK_PAYLOAD`, and without copying stay valid and unchanged while the
/// mux keeps them.
#[unsafe(export_name = "WebPMuxSetImage")]
#[must_use]
pub unsafe extern "C" fn webp_mux_set_image(
    mux: *mut WebPMux,
    bitstream: *const WebPData,
    copy_data: c_int,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let mux = unsafe { mux_mut(mux) };
    // SAFETY: null or a valid `WebPData` (caller's contract).
    let bitstream = unsafe { bitstream.as_ref() };
    let (Some(mux), Some(bitstream)) = (mux, bitstream) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if bitstream.bytes.is_null() || bitstream.size as u64 > u64::from(MAX_CHUNK_PAYLOAD) {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: readable bytes of a checked size, valid while the mux keeps them
    // (caller's contract).
    let data = unsafe { lend(bitstream) };
    code(&edit::webp_mux_set_image(mux, &data, copy_data))
}

// upstream: src/mux/muxedit.c WebPMuxPushFrame
/// Adds a frame at the end of the mux object; `frame->id` must be
/// `WEBP_CHUNK_ANMF`. Without copying the data must exist until a call to
/// `WebPMuxAssemble()`.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `frame` is null or a valid
/// frame whose non-null bitstream bytes are readable for their size when it is
/// at most `MAX_CHUNK_PAYLOAD`, and without copying stay valid and unchanged
/// while the mux keeps them.
#[unsafe(export_name = "WebPMuxPushFrame")]
#[must_use]
pub unsafe extern "C" fn webp_mux_push_frame(
    mux: *mut WebPMux,
    frame: *const WebPMuxFrameInfo,
    copy_data: c_int,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let mux = unsafe { mux_mut(mux) };
    // SAFETY: null or a valid frame (caller's contract).
    let frame = unsafe { frame.as_ref() };
    let (Some(mux), Some(frame)) = (mux, frame) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if frame.id != WebPChunkId::WEBP_CHUNK_ANMF {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    if frame.bitstream.bytes.is_null() || frame.bitstream.size as u64 > u64::from(MAX_CHUNK_PAYLOAD)
    {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    let info = RsFrameInfo {
        // SAFETY: readable bytes of a checked size, valid while the mux keeps
        // them (caller's contract).
        bitstream: unsafe { lend(&frame.bitstream) },
        x_offset: frame.x_offset,
        y_offset: frame.y_offset,
        duration: frame.duration,
        id: frame.id,
        dispose_method: frame.dispose_method,
        blend_method: frame.blend_method,
        pad: frame.pad,
    };
    code(&edit::webp_mux_push_frame(mux, &info, copy_data))
}

// upstream: src/mux/muxread.c WebPMuxGetFrame
/// Gets the nth frame from the mux object (`nth = 0` is the last position).
/// The bitstream is a single-image WebP file allocated for the caller, who
/// releases it with `WebPDataClear()`. On failure `*frame` is unchanged;
/// `pad` is never written.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `frame` is null or writable.
#[unsafe(export_name = "WebPMuxGetFrame")]
#[must_use]
pub unsafe extern "C" fn webp_mux_get_frame(
    mux: *const WebPMux,
    nth: u32,
    frame: *mut WebPMuxFrameInfo,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_ref(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if frame.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    let info = match read::webp_mux_get_frame(mux, nth) {
        Ok(info) => info,
        Err(err) => return err,
    };
    // C writes the fields and then allocates the bitstream; a failed
    // allocation leaves them written. The fields are written one by one, so
    // `pad` keeps the caller's value and the frame may be uninitialized.
    let bitstream = data_to_caller(&info.bitstream);
    // SAFETY: a writable frame (checked non-null; caller's contract); only
    // the field's address is computed.
    let field = unsafe { &raw mut (*frame).x_offset };
    // SAFETY: the writable field.
    unsafe { put(field, info.x_offset) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*frame).y_offset };
    // SAFETY: as above.
    unsafe { put(field, info.y_offset) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*frame).duration };
    // SAFETY: as above.
    unsafe { put(field, info.duration) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*frame).id };
    // SAFETY: as above.
    unsafe { put(field, info.id) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*frame).dispose_method };
    // SAFETY: as above.
    unsafe { put(field, info.dispose_method) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*frame).blend_method };
    // SAFETY: as above.
    unsafe { put(field, info.blend_method) };
    let Some(bitstream) = bitstream else {
        return WebPMuxError::WEBP_MUX_MEMORY_ERROR;
    };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*frame).bitstream };
    // SAFETY: as above.
    unsafe { put(field, bitstream) };
    WebPMuxError::WEBP_MUX_OK
}

// upstream: src/mux/muxedit.c WebPMuxDeleteFrame
/// Deletes a frame from the mux object (`nth = 0` is the last position).
///
/// # Safety
///
/// `mux` is null or a live mux of this library.
#[unsafe(export_name = "WebPMuxDeleteFrame")]
#[must_use]
pub unsafe extern "C" fn webp_mux_delete_frame(mux: *mut WebPMux, nth: u32) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_mut(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    code(&edit::webp_mux_delete_frame(mux, nth))
}

//------------------------------------------------------------------------------
// Animation.

// upstream: src/mux/muxedit.c WebPMuxSetAnimationParams
/// Sets the animation parameters in the mux object; any existing ANIM chunks
/// are removed.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `params` is null or valid.
#[unsafe(export_name = "WebPMuxSetAnimationParams")]
#[must_use]
pub unsafe extern "C" fn webp_mux_set_animation_params(
    mux: *mut WebPMux,
    params: *const WebPMuxAnimParams,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let mux = unsafe { mux_mut(mux) };
    // SAFETY: null or valid (caller's contract).
    let params = unsafe { params.as_ref() };
    let (Some(mux), Some(params)) = (mux, params) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    code(&edit::webp_mux_set_animation_params(mux, params))
}

// upstream: src/mux/muxread.c WebPMuxGetAnimationParams
/// Gets the animation parameters from the mux object.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `params` is null or writable.
#[unsafe(export_name = "WebPMuxGetAnimationParams")]
#[must_use]
pub unsafe extern "C" fn webp_mux_get_animation_params(
    mux: *const WebPMux,
    params: *mut WebPMuxAnimParams,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_ref(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if params.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    match read::webp_mux_get_animation_params(mux) {
        Ok(value) => {
            // SAFETY: writable (checked non-null; caller's contract).
            unsafe { put(params, value) };
            WebPMuxError::WEBP_MUX_OK
        }
        Err(err) => err,
    }
}

//------------------------------------------------------------------------------
// Misc Utilities.

// upstream: src/mux/muxedit.c WebPMuxSetCanvasSize
/// Sets the canvas size for the mux object; `(0, 0)` takes the bounding box of
/// the frames at `WebPMuxAssemble()`.
///
/// # Safety
///
/// `mux` is null or a live mux of this library.
#[unsafe(export_name = "WebPMuxSetCanvasSize")]
#[must_use]
pub unsafe extern "C" fn webp_mux_set_canvas_size(
    mux: *mut WebPMux,
    width: c_int,
    height: c_int,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_mut(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    code(&edit::webp_mux_set_canvas_size(mux, width, height))
}

// upstream: src/mux/muxread.c WebPMuxGetCanvasSize
/// Gets the canvas size from the mux object.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `width` and `height` are null
/// or writable.
#[unsafe(export_name = "WebPMuxGetCanvasSize")]
#[must_use]
pub unsafe extern "C" fn webp_mux_get_canvas_size(
    mux: *const WebPMux,
    width: *mut c_int,
    height: *mut c_int,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_ref(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if width.is_null() || height.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    match read::webp_mux_get_canvas_size(mux) {
        Ok((w, h)) => {
            // SAFETY: writable (checked non-null; caller's contract).
            unsafe { put(width, w) };
            // SAFETY: writable (checked non-null; caller's contract).
            unsafe { put(height, h) };
            WebPMuxError::WEBP_MUX_OK
        }
        Err(err) => err,
    }
}

// upstream: src/mux/muxread.c WebPMuxGetFeatures
/// Gets the feature flags from the mux object.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `flags` is null or writable.
#[unsafe(export_name = "WebPMuxGetFeatures")]
#[must_use]
pub unsafe extern "C" fn webp_mux_get_features(
    mux: *const WebPMux,
    flags: *mut u32,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_ref(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if flags.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    match read::webp_mux_get_features(mux) {
        Ok(value) => {
            // SAFETY: writable (checked non-null; caller's contract).
            unsafe { put(flags, value) };
            WebPMuxError::WEBP_MUX_OK
        }
        Err(err) => err,
    }
}

// upstream: src/mux/muxread.c WebPMuxNumChunks
/// Gets number of chunks with the given `id` in the mux object.
///
/// # Safety
///
/// `mux` is null or a live mux of this library; `num_elements` is null or
/// writable.
#[unsafe(export_name = "WebPMuxNumChunks")]
#[must_use]
pub unsafe extern "C" fn webp_mux_num_chunks(
    mux: *const WebPMux,
    id: WebPChunkId,
    num_elements: *mut c_int,
) -> WebPMuxError {
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_ref(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if num_elements.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    match read::webp_mux_num_chunks(mux, id) {
        Ok(value) => {
            // SAFETY: writable (checked non-null; caller's contract).
            unsafe { put(num_elements, value) };
            WebPMuxError::WEBP_MUX_OK
        }
        Err(err) => err,
    }
}

// upstream: src/mux/muxedit.c WebPMuxAssemble
/// Assembles all chunks in WebP RIFF format into `assembled_data`, which is
/// allocated for the caller and released with `WebPDataClear()`; it is zeroed
/// first, so that it is empty on failure.
///
/// # Safety
///
/// `mux` is null or a live mux of this library whose lent bytes are valid;
/// `assembled_data` is null or writable.
#[unsafe(export_name = "WebPMuxAssemble")]
#[must_use]
pub unsafe extern "C" fn webp_mux_assemble(
    mux: *mut WebPMux,
    assembled_data: *mut WebPData,
) -> WebPMuxError {
    if assembled_data.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    let empty = WebPData {
        bytes: ptr::null(),
        size: 0,
    };
    // Clean up returned data, in case something goes wrong.
    // SAFETY: writable (checked non-null; caller's contract).
    unsafe { put(assembled_data, empty) };
    // SAFETY: null or a live mux (caller's contract).
    let Some(mux) = (unsafe { mux_mut(mux) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    let data = match edit::webp_mux_assemble(mux) {
        Ok(data) => data,
        Err(err) => return err,
    };
    let Some(data) = data_to_caller(&data) else {
        return WebPMuxError::WEBP_MUX_MEMORY_ERROR;
    };
    // SAFETY: writable (checked non-null; caller's contract).
    unsafe { put(assembled_data, data) };
    WebPMuxError::WEBP_MUX_OK
}

//------------------------------------------------------------------------------
// WebPAnimEncoder API

// upstream: src/mux/anim_encode.c WebPAnimEncoderOptionsInitInternal
/// Internal, version-checked, entry point of `WebPAnimEncoderOptionsInit()`.
/// Returns false for null options or a version mismatch. It writes only the
/// fields that C's `DefaultEncoderOptions` writes, not the padding.
///
/// # Safety
///
/// `enc_options` is null or writable.
#[unsafe(export_name = "WebPAnimEncoderOptionsInitInternal")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_options_init_internal(
    enc_options: *mut WebPAnimEncoderOptions,
    abi_version: c_int,
) -> c_int {
    if enc_options.is_null() {
        return 0;
    }
    let mut defaults = WebPAnimEncoderOptions::default();
    if !anim::webp_anim_encoder_options_init_internal(&mut defaults, abi_version) {
        return 0;
    }
    // C writes one field at a time; the options may be uninitialized.
    // SAFETY: writable options (checked non-null; caller's contract); only the
    // field's address is computed.
    let field = unsafe { &raw mut (*enc_options).anim_params };
    // SAFETY: the writable field.
    unsafe { put(field, defaults.anim_params) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*enc_options).minimize_size };
    // SAFETY: as above.
    unsafe { put(field, defaults.minimize_size) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*enc_options).kmin };
    // SAFETY: as above.
    unsafe { put(field, defaults.kmin) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*enc_options).kmax };
    // SAFETY: as above.
    unsafe { put(field, defaults.kmax) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*enc_options).allow_mixed };
    // SAFETY: as above.
    unsafe { put(field, defaults.allow_mixed) };
    // SAFETY: as above.
    let field = unsafe { &raw mut (*enc_options).verbose };
    // SAFETY: as above.
    unsafe { put(field, defaults.verbose) };
    1
}

/// The fields of the caller's options that the encoder reads; C copies the
/// whole struct, whose padding the encoder never reads.
///
/// # Safety
///
/// `options` points to valid options.
unsafe fn read_options(options: *const WebPAnimEncoderOptions) -> WebPAnimEncoderOptions {
    /// Reads one field.
    ///
    /// # Safety
    ///
    /// `field` points to an initialized `T`.
    unsafe fn get<T>(field: *const T) -> T {
        // SAFETY: an initialized field (caller's contract).
        unsafe { field.read() }
    }
    // SAFETY: valid options (caller's contract); only the field's address is
    // computed.
    let anim_params = unsafe { &raw const (*options).anim_params };
    // SAFETY: as above.
    let minimize_size = unsafe { &raw const (*options).minimize_size };
    // SAFETY: as above.
    let kmin = unsafe { &raw const (*options).kmin };
    // SAFETY: as above.
    let kmax = unsafe { &raw const (*options).kmax };
    // SAFETY: as above.
    let allow_mixed = unsafe { &raw const (*options).allow_mixed };
    // SAFETY: as above.
    let verbose = unsafe { &raw const (*options).verbose };
    WebPAnimEncoderOptions {
        // SAFETY: an initialized field of valid options.
        anim_params: unsafe { get(anim_params) },
        // SAFETY: as above.
        minimize_size: unsafe { get(minimize_size) },
        // SAFETY: as above.
        kmin: unsafe { get(kmin) },
        // SAFETY: as above.
        kmax: unsafe { get(kmax) },
        // SAFETY: as above.
        allow_mixed: unsafe { get(allow_mixed) },
        // SAFETY: as above.
        verbose: unsafe { get(verbose) },
        padding: [0; 4],
    }
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderNewInternal
/// Internal, version-checked, entry point of `WebPAnimEncoderNew()`: an encoder
/// for a `width` by `height` canvas, with default options for null options;
/// null for a version mismatch, invalid dimensions or a memory error.
///
/// # Safety
///
/// `enc_options` is null or valid options.
#[unsafe(export_name = "WebPAnimEncoderNewInternal")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_new_internal(
    width: c_int,
    height: c_int,
    enc_options: *const WebPAnimEncoderOptions,
    abi_version: c_int,
) -> *mut WebPAnimEncoder {
    // C reads the options only once these checks have passed.
    if webp_abi_is_incompatible(abi_version, WEBP_MUX_ABI_VERSION)
        || width <= 0
        || height <= 0
        || u64::from(width.unsigned_abs()) * u64::from(height.unsigned_abs()) >= MAX_IMAGE_AREA
    {
        return ptr::null_mut();
    }
    let options = if enc_options.is_null() {
        None
    } else {
        // SAFETY: valid options (checked non-null; caller's contract).
        Some(unsafe { read_options(enc_options) })
    };
    let Some(enc) =
        anim::webp_anim_encoder_new_internal(width, height, options.as_ref(), abi_version)
    else {
        return ptr::null_mut();
    };
    let mut handle = Box::new(WebPAnimEncoder {
        enc,
        error: [0; ERROR_STR_MAX_LENGTH],
    });
    handle.sync_error();
    Box::into_raw(handle)
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderAdd
/// Optimizes the given frame for WebP, encodes it and adds it to the encoder;
/// a null frame ends the sequence. Returns false on error, with
/// `frame->error_code` set. A frame whose planes this module cannot lend (see
/// `src/abi/encode.rs`) records `VP8_ENC_ERROR_INVALID_CONFIGURATION` and an
/// error string, where C would encode it.
///
/// # Safety
///
/// `enc` is null or a live encoder of this library; `frame` is null or a valid
/// picture whose ARGB plane (or YUVA planes, without `use_argb`) is readable for
/// its dimensions and whose memory fields are null or blocks of the platform
/// `malloc`; `config` is null or a valid configuration.
#[unsafe(export_name = "WebPAnimEncoderAdd")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_add(
    enc: *mut WebPAnimEncoder,
    frame: *mut WebPPicture,
    timestamp: c_int,
    encoder_config: *const WebPConfig,
) -> c_int {
    // SAFETY: null or a live encoder (caller's contract).
    let Some(handle) = (unsafe { encoder(enc) }) else {
        return 0;
    };
    // SAFETY: null or a valid configuration (caller's contract).
    let config: Option<WebPConfig> = unsafe { encoder_config.as_ref() }.copied();
    // SAFETY: null or a valid picture (caller's contract); the borrow ends
    // before the conversion.
    let ok = match unsafe { frame.as_ref() } {
        None => anim::webp_anim_encoder_add(&mut handle.enc, None, timestamp, config.as_ref()),
        Some(c) => {
            let hook = c.progress_hook;
            // WebPCopyPixels reads the ARGB plane; WebPPictureYUVAToARGB reads
            // the YUVA planes and makes the ARGB plane.
            let modes = if c.use_argb != 0 {
                (Mode::Discard, Mode::Read)
            } else {
                (Mode::Read, Mode::Discard)
            };
            let enc = &mut handle.enc;
            // SAFETY: the planes of the modes are readable (caller's contract).
            let result = unsafe {
                with_picture(frame, modes, true, |rs| {
                    rs.progress_hook = hook.map(|hook| progress_closure(hook, frame));
                    let ok = anim::webp_anim_encoder_add(enc, Some(rs), timestamp, config.as_ref());
                    rs.progress_hook = None;
                    ok
                })
            };
            if let Some((ok, written)) = result {
                ok && written
            } else {
                anim::mark_error(
                    &mut handle.enc,
                    "ERROR adding frame: the C ABI cannot lend the frame's planes",
                );
                false
            }
        }
    };
    handle.sync_error();
    c_int::from(ok)
}

/// A slice that no caller passes: marks a `WebPData` that
/// `webp_anim_encoder_assemble` did not write.
static UNWRITTEN: [u8; 1] = [0];

// upstream: src/mux/anim_encode.c WebPAnimEncoderAssemble
/// Assembles all frames added so far into a WebP bitstream in `webp_data`,
/// which is allocated for the caller and released with `WebPDataClear()`.
/// Returns true on success. `webp_data` is written where C writes it.
///
/// # Safety
///
/// `enc` is null or a live encoder of this library whose lent chunk bytes are
/// valid; `webp_data` is null or writable.
#[unsafe(export_name = "WebPAnimEncoderAssemble")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_assemble(
    enc: *mut WebPAnimEncoder,
    webp_data: *mut WebPData,
) -> c_int {
    // SAFETY: null or a live encoder (caller's contract).
    let Some(handle) = (unsafe { encoder(enc) }) else {
        return 0;
    };
    if webp_data.is_null() {
        anim::mark_error(&mut handle.enc, "ERROR assembling: NULL input");
        handle.sync_error();
        return 0;
    }
    let mut data = RsData {
        bytes: Cow::Borrowed(&UNWRITTEN),
    };
    let mut ok = anim::webp_anim_encoder_assemble(&mut handle.enc, &mut data);
    if !ptr::eq(data.bytes.as_ptr(), UNWRITTEN.as_ptr()) {
        let value = if data.bytes.is_empty() {
            WebPData {
                bytes: ptr::null(),
                size: 0,
            }
        } else if let Some(value) = data_to_caller(&data) {
            value
        } else {
            ok = false;
            WebPData {
                bytes: ptr::null(),
                size: 0,
            }
        };
        // SAFETY: writable (checked non-null; caller's contract).
        unsafe { put(webp_data, value) };
    }
    handle.sync_error();
    c_int::from(ok)
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderGetError
/// Get error string corresponding to the most recent call using `enc`: null for
/// a null encoder, the error of the last call, or an empty string. The string
/// is owned by `enc` and changes with the next call to `WebPAnimEncoderAdd()`
/// or `WebPAnimEncoderAssemble()`.
///
/// # Safety
///
/// `enc` is null or a live encoder of this library.
#[unsafe(export_name = "WebPAnimEncoderGetError")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_get_error(enc: *mut WebPAnimEncoder) -> *const c_char {
    // SAFETY: null or a live encoder (caller's contract).
    let Some(handle) = (unsafe { encoder(enc) }) else {
        return ptr::null();
    };
    handle.sync_error();
    handle.error.as_ptr()
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderDelete
/// Deletes the `WebPAnimEncoder` object.
///
/// # Safety
///
/// `enc` is null or an encoder of this library that is not used again.
#[unsafe(export_name = "WebPAnimEncoderDelete")]
pub unsafe extern "C" fn webp_anim_encoder_delete(enc: *mut WebPAnimEncoder) {
    if enc.is_null() {
        return;
    }
    // SAFETY: an encoder of this library, which the caller gives up.
    let handle = unsafe { Box::from_raw(enc) };
    anim::webp_anim_encoder_delete(handle.enc);
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderSetChunk
/// Adds a chunk with id `fourcc` and data `chunk_data` to the encoder's mux;
/// any existing chunk(s) with the same id are removed. Without copying the data
/// must exist until a call to `WebPAnimEncoderAssemble()`.
///
/// # Safety
///
/// As `WebPMuxSetChunk()`, with `enc` null or a live encoder of this library.
#[unsafe(export_name = "WebPAnimEncoderSetChunk")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_set_chunk(
    enc: *mut WebPAnimEncoder,
    fourcc: *const c_char,
    chunk_data: *const WebPData,
    copy_data: c_int,
) -> WebPMuxError {
    // SAFETY: null or a live encoder (caller's contract).
    let Some(handle) = (unsafe { encoder(enc) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    // SAFETY: null or a valid `WebPData` (caller's contract).
    let Some(chunk_data) = (unsafe { chunk_data.as_ref() }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if fourcc.is_null()
        || chunk_data.bytes.is_null()
        || chunk_data.size as u64 > u64::from(MAX_CHUNK_PAYLOAD)
    {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: four readable characters (checked non-null; caller's contract).
    let tag = unsafe { read_fourcc(fourcc) };
    // SAFETY: readable bytes of a checked size, valid while the encoder keeps
    // them (caller's contract).
    let data = unsafe { lend(chunk_data) };
    code(&anim::webp_anim_encoder_set_chunk(
        &mut handle.enc,
        &tag,
        &data,
        copy_data,
    ))
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderGetChunk
/// Gets a reference to the data of the chunk with id `fourcc` in the encoder's
/// mux; the caller must not free it.
///
/// # Safety
///
/// As `WebPMuxGetChunk()`, with `enc` null or a live encoder of this library.
#[unsafe(export_name = "WebPAnimEncoderGetChunk")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_get_chunk(
    enc: *const WebPAnimEncoder,
    fourcc: *const c_char,
    chunk_data: *mut WebPData,
) -> WebPMuxError {
    // SAFETY: null or a live encoder (caller's contract).
    let Some(handle) = (unsafe { enc.as_ref() }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if fourcc.is_null() || chunk_data.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: four readable characters (checked non-null; caller's contract).
    let tag = unsafe { read_fourcc(fourcc) };
    // SAFETY: a writable `WebPData` (checked non-null; caller's contract).
    unsafe {
        get_chunk(
            anim::webp_anim_encoder_get_chunk(&handle.enc, &tag),
            &tag,
            chunk_data,
        )
    }
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderDeleteChunk
/// Deletes the chunk with the given `fourcc` from the encoder's mux.
///
/// # Safety
///
/// `enc` is null or a live encoder of this library; `fourcc` is null or four
/// readable characters.
#[unsafe(export_name = "WebPAnimEncoderDeleteChunk")]
#[must_use]
pub unsafe extern "C" fn webp_anim_encoder_delete_chunk(
    enc: *mut WebPAnimEncoder,
    fourcc: *const c_char,
) -> WebPMuxError {
    // SAFETY: null or a live encoder (caller's contract).
    let Some(handle) = (unsafe { encoder(enc) }) else {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    };
    if fourcc.is_null() {
        return WebPMuxError::WEBP_MUX_INVALID_ARGUMENT;
    }
    // SAFETY: four readable characters (checked non-null; caller's contract).
    let tag = unsafe { read_fourcc(fourcc) };
    code(&anim::webp_anim_encoder_delete_chunk(&mut handle.enc, &tag))
}
