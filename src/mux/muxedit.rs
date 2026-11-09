// libwebp v1.6.0, src/mux/muxedit.c:
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
// Set and delete APIs for mux.
//
// Authors: Urvang (urvang@google.com)
//          Vikas (vikasa@google.com)

//! Translation of `src/mux/muxedit.c` from libwebp v1.6.0.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! The setters take the caller's [`WebPData`] by reference: with
//! `copy_data == 0` the mux keeps the bytes for `'a` when the `WebPData`
//! borrows them, and copies them when it owns them (a Rust value cannot be
//! aliased; `docs/status/wp15.md`). C's checks for `NULL` arguments and
//! `NULL` bytes exist only at the C ABI (PORTING rule 8).

use std::borrow::Cow;

use crate::dec::vp8l_dec::vp8l_check_signature;
use crate::mux::muxi::{
    CHUNK_INDEX, ChunkBytes, WebPChunk, WebPMux, WebPMuxImage, data_is_null, is_wpi,
};
use crate::mux::muxinternal::{
    ChunkList, chunk_assign_data, chunk_delete, chunk_get_id_from_tag, chunk_get_index_from_tag,
    chunk_get_tag_from_four_cc, chunk_info, chunk_init, chunk_list_delete, chunk_list_disk_size,
    chunk_list_emit, chunk_release, chunk_set_head, mux_emit_riff_header,
    mux_get_chunk_list_from_id_mut, mux_has_alpha, mux_image_count, mux_image_delete,
    mux_image_delete_nth, mux_image_disk_size, mux_image_emit, mux_image_get_nth, mux_image_init,
    mux_image_push, mux_image_release, mux_validate,
};
use crate::mux::muxread::{mux_image_finalize, webp_mux_num_chunks};
use crate::utils::utils::{get_le24, put_le16, put_le24, put_le32, webp_safe_malloc};
use crate::webp::format_constants::{
    ANIM_CHUNK_SIZE, MAX_CANVAS_SIZE, MAX_CHUNK_PAYLOAD, MAX_DURATION, MAX_IMAGE_AREA,
    MAX_LOOP_COUNT, MAX_POSITION_OFFSET, RIFF_HEADER_SIZE, TAG_SIZE, VP8X_CHUNK_SIZE,
};
use crate::webp::mux::{
    WEBP_MUX_ABI_VERSION, WebPChunkId, WebPMuxAnimParams, WebPMuxError, WebPMuxFrameInfo,
    webp_mux_create,
};
use crate::webp::mux_types::{
    WebPData, WebPFeatureFlags, WebPMuxAnimBlend, WebPMuxAnimDispose, webp_data_clear,
};
use crate::webp::types::webp_abi_is_incompatible;

/// C's conversion of an `int` to `uint64_t`, which sign-extends.
fn c_u64(value: i32) -> u64 {
    i64::from(value).cast_unsigned()
}

//------------------------------------------------------------------------------
// Life of a mux object.

// upstream: src/mux/muxedit.c MuxInit
fn mux_init(mux: &mut WebPMux<'_>) {
    *mux = WebPMux::default();
    mux.canvas_width = 0; // just to be explicit
    mux.canvas_height = 0;
}

// upstream: src/mux/muxedit.c WebPNewInternal
/// Internal, version-checked, entry point of `WebPMuxNew`. Returns `None` on a
/// version mismatch. C's `WebPSafeMalloc` of the object has no counterpart: the
/// mux is a value, which cannot fail.
#[must_use]
pub fn webp_new_internal<'a>(version: i32) -> Option<WebPMux<'a>> {
    if webp_abi_is_incompatible(version, WEBP_MUX_ABI_VERSION) {
        None
    } else {
        let mut mux = WebPMux::default();
        mux_init(&mut mux);
        Some(mux)
    }
}

// upstream: src/mux/muxedit.c DeleteAllImages
/// Delete all images in `wpi_list`.
fn delete_all_images(wpi_list: &mut Vec<WebPMuxImage<'_>>) {
    for wpi in wpi_list.drain(..) {
        mux_image_delete(wpi);
    }
}

// upstream: src/mux/muxedit.c MuxRelease
fn mux_release(mux: &mut WebPMux<'_>) {
    delete_all_images(&mut mux.images);
    chunk_list_delete(&mut mux.vp8x);
    chunk_list_delete(&mut mux.iccp);
    chunk_list_delete(&mut mux.anim);
    chunk_list_delete(&mut mux.exif);
    chunk_list_delete(&mut mux.xmp);
    chunk_list_delete(&mut mux.unknown);
}

// upstream: src/mux/muxedit.c WebPMuxDelete
/// Deletes the mux object (the drop of the value; C frees it).
pub fn webp_mux_delete(mux: WebPMux<'_>) {
    let mut mux = mux;
    mux_release(&mut mux);
}

//------------------------------------------------------------------------------
// Helper method(s).

// upstream: src/mux/muxedit.c MuxSet
/// Creates a chunk from `data` and sets it as the only chunk of the list of
/// its type. The `SWITCH_ID_LIST` macro is expanded into the match; tags of
/// image chunks give `WEBP_MUX_NOT_FOUND`.
fn mux_set<'a>(
    mux: &mut WebPMux<'a>,
    tag: u32,
    data: ChunkBytes<'a, '_>,
    copy_data: i32,
) -> Result<(), WebPMuxError> {
    let mut chunk = WebPChunk::default();
    let idx = chunk_get_index_from_tag(tag);
    debug_assert!(!is_wpi(chunk_info(idx).id));

    chunk_init(&mut chunk);
    // upstream: src/mux/muxedit.c SWITCH_ID_LIST
    let list = match idx {
        CHUNK_INDEX::IDX_VP8X => &mut mux.vp8x,
        CHUNK_INDEX::IDX_ICCP => &mut mux.iccp,
        CHUNK_INDEX::IDX_ANIM => &mut mux.anim,
        CHUNK_INDEX::IDX_EXIF => &mut mux.exif,
        CHUNK_INDEX::IDX_XMP => &mut mux.xmp,
        CHUNK_INDEX::IDX_UNKNOWN => &mut mux.unknown,
        _ => return Err(WebPMuxError::WEBP_MUX_NOT_FOUND),
    };
    chunk_assign_data(&mut chunk, data, copy_data, tag)?;
    let err = chunk_set_head(&mut chunk, list);
    if err.is_err() {
        chunk_release(&mut chunk);
    }
    err
}

// upstream: src/mux/muxedit.c CreateFrameData
/// Create data for frame given image data, offsets and duration.
fn create_frame_data(
    width: i32,
    height: i32,
    info: &WebPMuxFrameInfo<'_>,
) -> Result<WebPData<'static>, WebPMuxError> {
    let frame_size = chunk_info(CHUNK_INDEX::IDX_ANMF).size as usize;

    debug_assert!(width > 0 && height > 0 && info.duration >= 0);
    debug_assert_eq!(info.dispose_method.0, info.dispose_method.0 & 1);
    // Note: assertion on upper bounds is done in PutLE24().

    // C: WebPSafeMalloc(1ULL, frame_size), the same size test.
    let mut frame_bytes =
        webp_safe_malloc::<u8>(frame_size as u64, 1).ok_or(WebPMuxError::WEBP_MUX_MEMORY_ERROR)?;
    frame_bytes.resize(frame_size, 0);

    put_le24(&mut frame_bytes[0..], info.x_offset / 2);
    put_le24(&mut frame_bytes[3..], info.y_offset / 2);

    put_le24(&mut frame_bytes[6..], width - 1);
    put_le24(&mut frame_bytes[9..], height - 1);
    put_le24(&mut frame_bytes[12..], info.duration);
    frame_bytes[15] =
        (if info.blend_method == WebPMuxAnimBlend::WEBP_MUX_NO_BLEND {
            2
        } else {
            0
        }) | u8::from(info.dispose_method == WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND);

    Ok(WebPData {
        bytes: Cow::Owned(frame_bytes),
    })
}

/// The bytes of a chunk that a mux created without copying from lent bytes
/// holds: `ChunkAssignData` lends them, so they are borrowed for `'x`.
fn lent_bytes<'x>(data: &WebPData<'x>) -> &'x [u8] {
    match data.bytes {
        Cow::Borrowed(bytes) => bytes,
        Cow::Owned(_) => unreachable!("an image chunk of a mux over lent bytes is borrowed"),
    }
}

/// The image and alpha chunks of the first image of the WebP file `bitstream`,
/// as `GetImageData` takes them from `WebPMuxCreate(bitstream, 0)`. No C
/// counterpart: it lets [`get_image_data`] keep the lending of its input.
fn image_chunks(bitstream: &[u8]) -> Result<(&[u8], Option<&[u8]>), WebPMuxError> {
    let data = WebPData {
        bytes: Cow::Borrowed(bitstream),
    };
    let Some(mux) = webp_mux_create(&data, 0) else {
        return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
    };
    let wpi = mux.images.first().expect("a valid mux has an image");
    let img = wpi.img.as_ref().expect("an image has an image chunk");
    let image = lent_bytes(&img.data);
    let alpha = wpi.alpha.as_ref().map(|alpha| lent_bytes(&alpha.data));
    webp_mux_delete(mux);
    Ok((image, alpha))
}

// upstream: src/mux/muxedit.c GetImageData
/// Outputs image data given a bitstream. The bitstream can either be a
/// single-image WebP file or raw VP8/VP8L data. Also outputs `is_lossless` to
/// be true if the given bitstream is lossless. The alpha data is `None` where C
/// leaves `alpha->bytes` `NULL`.
fn get_image_data<'a, 's>(
    bitstream: ChunkBytes<'a, 's>,
) -> Result<(ChunkBytes<'a, 's>, Option<ChunkBytes<'a, 's>>, bool), WebPMuxError> {
    // Default: no alpha.
    let bytes = bitstream.bytes();
    let (image, alpha) = if bytes.len() < TAG_SIZE as usize || bytes[..4] != *b"RIFF" {
        // It is NOT webp file data. Return input data as is.
        (bitstream, None)
    } else {
        // It is webp file data. Extract image data from it.
        match bitstream {
            ChunkBytes::Lent(lent) => {
                let (image, alpha) = image_chunks(lent)?;
                (ChunkBytes::Lent(image), alpha.map(ChunkBytes::Lent))
            }
            ChunkBytes::Read(read) => {
                let (image, alpha) = image_chunks(read)?;
                (ChunkBytes::Read(image), alpha.map(ChunkBytes::Read))
            }
        }
    };
    let is_lossless = vp8l_check_signature(image.bytes());
    Ok((image, alpha, is_lossless))
}

// upstream: src/mux/muxedit.c DeleteChunks
fn delete_chunks(chunk_list: &mut Vec<WebPChunk<'_>>, tag: u32) -> Result<(), WebPMuxError> {
    let mut err = Err(WebPMuxError::WEBP_MUX_NOT_FOUND);
    let mut position = 0;
    while position < chunk_list.len() {
        if chunk_list[position].tag == tag {
            chunk_delete(chunk_list.remove(position));
            err = Ok(());
        } else {
            position += 1;
        }
    }
    err
}

// upstream: src/mux/muxedit.c MuxDeleteAllNamedData
fn mux_delete_all_named_data(mux: &mut WebPMux<'_>, tag: u32) -> Result<(), WebPMuxError> {
    let id = chunk_get_id_from_tag(tag);
    if is_wpi(id) {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    delete_chunks(mux_get_chunk_list_from_id_mut(mux, id), tag)
}

//------------------------------------------------------------------------------
// Set API(s).

// upstream: src/mux/muxedit.c WebPMuxSetChunk
/// Adds a chunk with id `fourcc` and data `chunk_data` in the mux object; any
/// existing chunk(s) with the same id are removed. With `copy_data == 0` the
/// chunk keeps the bytes (they must exist until `WebPMuxAssemble`, as the
/// header requires), with 1 it copies them. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments and `NULL` bytes.
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` if the data is larger than `MAX_CHUNK_PAYLOAD`
/// or `fourcc` corresponds to an image chunk, `WEBP_MUX_NOT_FOUND` when an
/// unknown chunk of another tag is already present (C's `ChunkSetHead`),
/// `WEBP_MUX_MEMORY_ERROR` on memory allocation error.
pub fn webp_mux_set_chunk<'a>(
    mux: &mut WebPMux<'a>,
    fourcc: &[u8; 4],
    chunk_data: &WebPData<'a>,
    copy_data: i32,
) -> Result<(), WebPMuxError> {
    if chunk_data.bytes.len() as u64 > u64::from(MAX_CHUNK_PAYLOAD) {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    let tag = chunk_get_tag_from_four_cc(fourcc);

    // Delete existing chunk(s) with the same 'fourcc'.
    match mux_delete_all_named_data(mux, tag) {
        Ok(()) | Err(WebPMuxError::WEBP_MUX_NOT_FOUND) => {}
        Err(err) => return Err(err),
    }

    // Add the given chunk.
    mux_set(mux, tag, ChunkBytes::of(chunk_data), copy_data)
}

// upstream: src/mux/muxedit.c AddDataToChunkList
/// Creates a chunk from given `data` and sets it as 1st chunk in `chunk_list`.
fn add_data_to_chunk_list<'a>(
    data: ChunkBytes<'a, '_>,
    copy_data: i32,
    tag: u32,
    chunk_list: &mut impl ChunkList<'a>,
) -> Result<(), WebPMuxError> {
    let mut chunk = WebPChunk::default();
    chunk_init(&mut chunk);
    let err = chunk_assign_data(&mut chunk, data, copy_data, tag)
        .and_then(|()| chunk_set_head(&mut chunk, chunk_list));
    if err.is_err() {
        chunk_release(&mut chunk);
    }
    err
}

// upstream: src/mux/muxedit.c SetAlphaAndImageChunks
/// Extracts image & alpha data from the given bitstream and then sets
/// `wpi.alpha` and `wpi.img` appropriately.
fn set_alpha_and_image_chunks<'a>(
    bitstream: ChunkBytes<'a, '_>,
    copy_data: i32,
    wpi: &mut WebPMuxImage<'a>,
) -> Result<(), WebPMuxError> {
    let (image, alpha, is_lossless) = get_image_data(bitstream)?;
    let image_tag = if is_lossless {
        chunk_info(CHUNK_INDEX::IDX_VP8L).tag
    } else {
        chunk_info(CHUNK_INDEX::IDX_VP8).tag
    };
    if let Some(alpha) = alpha {
        add_data_to_chunk_list(
            alpha,
            copy_data,
            chunk_info(CHUNK_INDEX::IDX_ALPHA).tag,
            &mut wpi.alpha,
        )?;
    }
    add_data_to_chunk_list(image, copy_data, image_tag, &mut wpi.img)?;
    if mux_image_finalize(wpi) {
        Ok(())
    } else {
        Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT)
    }
}

// upstream: src/mux/muxedit.c WebPMuxSetImage
/// Sets the (non-animated) image in the mux object; any existing images
/// (including frames) are removed first. `bitstream` can be a raw VP8/VP8L
/// bitstream or a single-image WebP file. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments and `NULL` bytes.
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` if the data is larger than `MAX_CHUNK_PAYLOAD`
/// or no image, `WEBP_MUX_BAD_DATA` for an invalid WebP file,
/// `WEBP_MUX_MEMORY_ERROR` on memory allocation error.
pub fn webp_mux_set_image<'a>(
    mux: &mut WebPMux<'a>,
    bitstream: &WebPData<'a>,
    copy_data: i32,
) -> Result<(), WebPMuxError> {
    if bitstream.bytes.len() as u64 > u64::from(MAX_CHUNK_PAYLOAD) {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    if !mux.images.is_empty() {
        // Only one 'simple image' can be added in mux. So, remove present images.
        delete_all_images(&mut mux.images);
    }

    let mut wpi = WebPMuxImage::default();
    mux_image_init(&mut wpi);
    let err = set_alpha_and_image_chunks(ChunkBytes::of(bitstream), copy_data, &mut wpi)
        // Add this WebPMuxImage to mux.
        .and_then(|()| mux_image_push(&mut wpi, &mut mux.images));
    if err.is_err() {
        // Something bad happened.
        mux_image_release(&mut wpi);
    }
    err
}

// upstream: src/mux/muxedit.c WebPMuxPushFrame
/// Adds a frame at the end of the mux object. `info.id` must be
/// `WEBP_CHUNK_ANMF`, and frames of one mux are all of that type; odd offsets
/// are snapped to even ones. C first returns `WEBP_MUX_INVALID_ARGUMENT` for
/// `NULL` arguments and `NULL` bytes.
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` if the content of `info` is invalid,
/// `WEBP_MUX_BAD_DATA` for an invalid WebP file, `WEBP_MUX_MEMORY_ERROR` on
/// memory allocation error.
pub fn webp_mux_push_frame<'a>(
    mux: &mut WebPMux<'a>,
    info: &WebPMuxFrameInfo<'a>,
    copy_data: i32,
) -> Result<(), WebPMuxError> {
    if info.id != WebPChunkId::WEBP_CHUNK_ANMF {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    if info.bitstream.bytes.len() as u64 > u64::from(MAX_CHUNK_PAYLOAD) {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    if let Some(image) = mux.images.first() {
        let image_id = match &image.header {
            Some(header) => chunk_get_id_from_tag(header.tag),
            None => WebPChunkId::WEBP_CHUNK_IMAGE,
        };
        if image_id != info.id {
            return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT); // Conflicting frame types.
        }
    }

    let mut wpi = WebPMuxImage::default();
    mux_image_init(&mut wpi);
    let err = (|| {
        set_alpha_and_image_chunks(ChunkBytes::of(&info.bitstream), copy_data, &mut wpi)?;
        debug_assert!(wpi.img.is_some()); // As SetAlphaAndImageChunks() was successful.

        {
            let tag = chunk_info(CHUNK_INDEX::IDX_ANMF).tag;
            let mut tmp = info.clone();
            tmp.x_offset &= !1; // Snap offsets to even.
            tmp.y_offset &= !1;
            if tmp.x_offset < 0
                || tmp.x_offset >= MAX_POSITION_OFFSET
                || tmp.y_offset < 0
                || tmp.y_offset >= MAX_POSITION_OFFSET
                || (tmp.duration < 0 || tmp.duration >= MAX_DURATION)
                || tmp.dispose_method.0 != (tmp.dispose_method.0 & 1)
            {
                return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
            }
            let mut frame = create_frame_data(wpi.width, wpi.height, &tmp)?;
            // Add frame chunk (with copy_data = 1).
            let err =
                add_data_to_chunk_list(ChunkBytes::Read(&frame.bytes), 1, tag, &mut wpi.header);
            webp_data_clear(&mut frame); // frame owned by wpi.header now.
            err?;
        }

        // Add this WebPMuxImage to mux.
        mux_image_push(&mut wpi, &mut mux.images)
    })();
    if err.is_err() {
        // Something bad happened.
        mux_image_release(&mut wpi);
    }
    err
}

// upstream: src/mux/muxedit.c WebPMuxSetAnimationParams
/// Sets the animation parameters in the mux object; any existing ANIM chunks
/// are removed. C first returns `WEBP_MUX_INVALID_ARGUMENT` for `NULL`
/// arguments.
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` for a loop count outside `[0, MAX_LOOP_COUNT)`,
/// `WEBP_MUX_MEMORY_ERROR` on memory allocation error.
pub fn webp_mux_set_animation_params(
    mux: &mut WebPMux<'_>,
    params: &WebPMuxAnimParams,
) -> Result<(), WebPMuxError> {
    let mut data = [0_u8; ANIM_CHUNK_SIZE as usize];

    if params.loop_count < 0 || params.loop_count >= MAX_LOOP_COUNT {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    // Delete any existing ANIM chunk(s).
    match mux_delete_all_named_data(mux, chunk_info(CHUNK_INDEX::IDX_ANIM).tag) {
        Ok(()) | Err(WebPMuxError::WEBP_MUX_NOT_FOUND) => {}
        Err(err) => return Err(err),
    }

    // Set the animation parameters.
    put_le32(&mut data, params.bgcolor);
    put_le16(&mut data[4..], params.loop_count);
    mux_set(
        mux,
        chunk_info(CHUNK_INDEX::IDX_ANIM).tag,
        ChunkBytes::Read(&data),
        1,
    )
}

// upstream: src/mux/muxedit.c WebPMuxSetCanvasSize
/// Sets the canvas size for the mux object: explicit, or `(0, 0)` for the
/// bounding box of the frames at `WebPMuxAssemble`. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for a `NULL` mux. C computes `width * height` in
/// `int`, which overflows for areas from 2^31 on (`docs/status/wp15.md`).
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` if width or height are invalid or out of bounds.
pub fn webp_mux_set_canvas_size(
    mux: &mut WebPMux<'_>,
    width: i32,
    height: i32,
) -> Result<(), WebPMuxError> {
    if width < 0 || height < 0 || width > MAX_CANVAS_SIZE || height > MAX_CANVAS_SIZE {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    if c_u64(width) * c_u64(height) >= MAX_IMAGE_AREA {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    if (width * height) == 0 && (width | height) != 0 {
        // one of width / height is zero, but not both -> invalid!
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    // If we already assembled a VP8X chunk, invalidate it.
    match mux_delete_all_named_data(mux, chunk_info(CHUNK_INDEX::IDX_VP8X).tag) {
        Ok(()) | Err(WebPMuxError::WEBP_MUX_NOT_FOUND) => {}
        Err(err) => return Err(err),
    }

    mux.canvas_width = width;
    mux.canvas_height = height;
    Ok(())
}

//------------------------------------------------------------------------------
// Delete API(s).

// upstream: src/mux/muxedit.c WebPMuxDeleteChunk
/// Deletes the chunks with the given `fourcc` from the mux object. C first
/// returns `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments.
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` if `fourcc` corresponds to an image chunk,
/// `WEBP_MUX_NOT_FOUND` if the mux does not contain a chunk with it.
pub fn webp_mux_delete_chunk(mux: &mut WebPMux<'_>, fourcc: &[u8; 4]) -> Result<(), WebPMuxError> {
    mux_delete_all_named_data(mux, chunk_get_tag_from_four_cc(fourcc))
}

// upstream: src/mux/muxedit.c WebPMuxDeleteFrame
/// Deletes the nth frame from the mux object; `nth = 0` is the last position.
/// C first returns `WEBP_MUX_INVALID_ARGUMENT` for a `NULL` mux.
///
/// # Errors
///
/// `WEBP_MUX_NOT_FOUND` if there are less than `nth` frames.
pub fn webp_mux_delete_frame(mux: &mut WebPMux<'_>, nth: u32) -> Result<(), WebPMuxError> {
    mux_image_delete_nth(&mut mux.images, nth)
}

//------------------------------------------------------------------------------
// Assembly of the WebP RIFF file.

// upstream: src/mux/muxedit.c GetFrameInfo
fn get_frame_info(frame_chunk: &WebPChunk<'_>) -> Result<(i32, i32, i32), WebPMuxError> {
    let data = &frame_chunk.data.bytes;
    let expected_data_size = crate::webp::format_constants::ANMF_CHUNK_SIZE as usize;
    debug_assert_eq!(frame_chunk.tag, chunk_info(CHUNK_INDEX::IDX_ANMF).tag);
    if data.len() != expected_data_size {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    let x_offset = 2 * get_le24(data);
    let y_offset = 2 * get_le24(&data[3..]);
    let duration = get_le24(&data[12..]);
    Ok((x_offset, y_offset, duration))
}

// upstream: src/mux/muxedit.c GetImageInfo
/// Returns `(x_offset, y_offset, duration, width, height)` of a frame.
fn get_image_info(wpi: &WebPMuxImage<'_>) -> Result<(i32, i32, i32, i32, i32), WebPMuxError> {
    let frame_chunk = wpi
        .header
        .as_ref()
        .expect("a frame of an animation has a header");

    // Get offsets and duration from ANMF chunk.
    let (x_offset, y_offset, duration) = get_frame_info(frame_chunk)?;

    // Get width and height from VP8/VP8L chunk.
    Ok((x_offset, y_offset, duration, wpi.width, wpi.height))
}

// upstream: src/mux/muxedit.c GetAdjustedCanvasSize
/// Returns the tightest dimension for the canvas considering the image list.
#[expect(clippy::similar_names, reason = "the C variable names")]
fn get_adjusted_canvas_size(mux: &WebPMux<'_>) -> Result<(i32, i32), WebPMuxError> {
    let wpi = mux
        .images
        .first()
        .expect("CreateVP8XChunk checked the first image");
    debug_assert!(wpi.img.is_some());

    if mux.images.len() > 1 {
        let mut max_x = 0;
        let mut max_y = 0;
        // if we have a chain of wpi's, header is necessarily set
        debug_assert!(wpi.header.is_some());
        // Aggregate the bounding box for animation frames.
        for wpi in &mux.images {
            let (x_offset, y_offset, _duration, w, h) = get_image_info(wpi)?;
            let max_x_pos = x_offset + w;
            let max_y_pos = y_offset + h;
            debug_assert!(x_offset < MAX_POSITION_OFFSET);
            debug_assert!(y_offset < MAX_POSITION_OFFSET);

            if max_x_pos > max_x {
                max_x = max_x_pos;
            }
            if max_y_pos > max_y {
                max_y = max_y_pos;
            }
        }
        Ok((max_x, max_y))
    } else {
        // For a single image, canvas dimensions are same as image dimensions.
        Ok((wpi.width, wpi.height))
    }
}

// upstream: src/mux/muxedit.c CreateVP8XChunk
/// VP8X format: total size 10; flags 4 bytes, width 3 bytes, height 3 bytes.
fn create_vp8x_chunk(mux: &mut WebPMux<'_>) -> Result<(), WebPMuxError> {
    let mut flags = 0_u32;
    let mut data = [0_u8; VP8X_CHUNK_SIZE as usize];

    match mux.images.first() {
        Some(images)
            if images
                .img
                .as_ref()
                .is_some_and(|img| !data_is_null(&img.data)) => {}
        _ => return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT),
    }

    // If VP8X chunk(s) is(are) already present, remove them (and later add new
    // VP8X chunk with updated flags).
    match mux_delete_all_named_data(mux, chunk_info(CHUNK_INDEX::IDX_VP8X).tag) {
        Ok(()) | Err(WebPMuxError::WEBP_MUX_NOT_FOUND) => {}
        Err(err) => return Err(err),
    }

    // Set flags.
    if mux
        .iccp
        .first()
        .is_some_and(|chunk| !data_is_null(&chunk.data))
    {
        flags |= WebPFeatureFlags::ICCP_FLAG.0;
    }
    if mux
        .exif
        .first()
        .is_some_and(|chunk| !data_is_null(&chunk.data))
    {
        flags |= WebPFeatureFlags::EXIF_FLAG.0;
    }
    if mux
        .xmp
        .first()
        .is_some_and(|chunk| !data_is_null(&chunk.data))
    {
        flags |= WebPFeatureFlags::XMP_FLAG.0;
    }
    let images = &mux.images[0];
    if let Some(header) = &images.header
        && header.tag == chunk_info(CHUNK_INDEX::IDX_ANMF).tag
    {
        // This is an image with animation.
        flags |= WebPFeatureFlags::ANIMATION_FLAG.0;
    }
    if mux_image_count(&mux.images, WebPChunkId::WEBP_CHUNK_ALPHA) > 0 {
        flags |= WebPFeatureFlags::ALPHA_FLAG.0; // Some images have an alpha channel.
    }

    let (mut width, mut height) = get_adjusted_canvas_size(mux)?;

    if width <= 0 || height <= 0 {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    if width > MAX_CANVAS_SIZE || height > MAX_CANVAS_SIZE {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }

    if mux.canvas_width != 0 || mux.canvas_height != 0 {
        if width > mux.canvas_width || height > mux.canvas_height {
            return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
        }
        width = mux.canvas_width;
        height = mux.canvas_height;
    }

    if flags == 0 && mux.unknown.is_empty() {
        // For simple file format, VP8X chunk should not be added.
        return Ok(());
    }

    if mux_has_alpha(&mux.images) {
        // This means some frames explicitly/implicitly contain alpha.
        // Note: This 'flags' update must NOT be done for a lossless image
        // without a VP8X chunk!
        flags |= WebPFeatureFlags::ALPHA_FLAG.0;
    }

    put_le32(&mut data, flags); // VP8X chunk flags.
    put_le24(&mut data[4..], width - 1); // canvas width.
    put_le24(&mut data[7..], height - 1); // canvas height.

    mux_set(
        mux,
        chunk_info(CHUNK_INDEX::IDX_VP8X).tag,
        ChunkBytes::Read(&data),
        1,
    )
}

// upstream: src/mux/muxedit.c MuxCleanup
/// Cleans up `mux` by removing any unnecessary chunks.
fn mux_cleanup(mux: &mut WebPMux<'_>) -> Result<(), WebPMuxError> {
    // If we have an image with a single frame, and its rectangle covers the
    // whole canvas, convert it to a non-animated image (to avoid writing ANMF
    // chunk unnecessarily).
    let mut num_frames = webp_mux_num_chunks(mux, chunk_info(CHUNK_INDEX::IDX_ANMF).id)?;
    if num_frames == 1 {
        let position = mux_image_get_nth(&mux.images, 1)?;
        let (canvas_width, canvas_height) = (mux.canvas_width, mux.canvas_height);
        // We know that one frame does exist.
        let frame = &mut mux.images[position];
        if frame.header.is_some()
            && ((canvas_width == 0 && canvas_height == 0)
                || (frame.width == canvas_width && frame.height == canvas_height))
        {
            let header = frame.header.take().expect("checked above");
            debug_assert_eq!(header.tag, chunk_info(CHUNK_INDEX::IDX_ANMF).tag);
            chunk_delete(header); // Removes ANMF chunk.
            num_frames = 0;
        }
    }
    // Remove ANIM chunk if this is a non-animated image.
    let num_anim_chunks = webp_mux_num_chunks(mux, chunk_info(CHUNK_INDEX::IDX_ANIM).id)?;
    if num_anim_chunks >= 1 && num_frames == 0 {
        mux_delete_all_named_data(mux, chunk_info(CHUNK_INDEX::IDX_ANIM).tag)?;
    }
    Ok(())
}

// upstream: src/mux/muxedit.c ImageListDiskSize
/// Total size of a list of images.
fn image_list_disk_size(wpi_list: &[WebPMuxImage<'_>]) -> usize {
    let mut size = 0_usize;
    for wpi in wpi_list {
        size += mux_image_disk_size(wpi);
    }
    size
}

// upstream: src/mux/muxedit.c ImageListEmit
/// Write out the given list of images at `dst[pos..]`; returns the position
/// after them.
fn image_list_emit(wpi_list: &[WebPMuxImage<'_>], dst: &mut [u8], pos: usize) -> usize {
    let mut pos = pos;
    for wpi in wpi_list {
        pos = mux_image_emit(wpi, dst, pos);
    }
    pos
}

// upstream: src/mux/muxedit.c WebPMuxAssemble
/// Assembles all chunks in WebP RIFF format and returns the bytes, which the
/// caller owns (C writes them into `*assembled_data`, which it first zeroes,
/// and on failure leaves empty). This also validates the mux object, after it
/// has removed unneeded ANMF and ANIM chunks and rewritten the VP8X chunk. C
/// first returns `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments.
///
/// # Errors
///
/// `WEBP_MUX_BAD_DATA` or `WEBP_MUX_INVALID_ARGUMENT` if the mux object is
/// invalid, `WEBP_MUX_MEMORY_ERROR` on memory allocation error.
pub fn webp_mux_assemble(mux: &mut WebPMux<'_>) -> Result<WebPData<'static>, WebPMuxError> {
    // Finalize mux.
    mux_cleanup(mux)?;
    create_vp8x_chunk(mux)?;

    // Allocate data.
    let size = chunk_list_disk_size(&mux.vp8x)
        + chunk_list_disk_size(&mux.iccp)
        + chunk_list_disk_size(&mux.anim)
        + image_list_disk_size(&mux.images)
        + chunk_list_disk_size(&mux.exif)
        + chunk_list_disk_size(&mux.xmp)
        + chunk_list_disk_size(&mux.unknown)
        + RIFF_HEADER_SIZE as usize;

    // C: WebPSafeMalloc(1ULL, size), the same size test.
    let mut data =
        webp_safe_malloc::<u8>(size as u64, 1).ok_or(WebPMuxError::WEBP_MUX_MEMORY_ERROR)?;
    data.resize(size, 0);

    // Emit header & chunks.
    let mut dst = mux_emit_riff_header(&mut data, size);
    dst = chunk_list_emit(&mux.vp8x, &mut data, dst);
    dst = chunk_list_emit(&mux.iccp, &mut data, dst);
    dst = chunk_list_emit(&mux.anim, &mut data, dst);
    dst = image_list_emit(&mux.images, &mut data, dst);
    dst = chunk_list_emit(&mux.exif, &mut data, dst);
    dst = chunk_list_emit(&mux.xmp, &mut data, dst);
    dst = chunk_list_emit(&mux.unknown, &mut data, dst);
    debug_assert_eq!(dst, size);

    // Validate mux.
    mux_validate(mux)?;

    // Finalize data.
    Ok(WebPData {
        bytes: Cow::Owned(data),
    })
}
