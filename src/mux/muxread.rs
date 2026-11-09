// libwebp v1.6.0, src/mux/muxread.c:
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
// Read APIs for mux.
//
// Authors: Urvang (urvang@google.com)
//          Vikas (vikasa@google.com)

//! Translation of `src/mux/muxread.c` from libwebp v1.6.0.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! The parsed bytes are a [`ChunkBytes`] source: lent for `'a` when the
//! caller's `WebPData` borrows them, and only readable when it owns them, in
//! which case even `copy_data == 0` copies (see `src/mux/muxi.rs`).

use crate::dec::vp8_dec::vp8_get_info;
use crate::dec::vp8l_dec::vp8l_get_info;
use crate::mux::muxedit::webp_mux_delete;
use crate::mux::muxi::{
    CHUNK_INDEX, ChunkBytes, NIL_TAG, WebPChunk, WebPMux, WebPMuxImage, chunk_disk_size, is_wpi,
    size_with_padding,
};
use crate::mux::muxinternal::{
    CHUNKS, chunk_append, chunk_assign_data, chunk_delete, chunk_get_id_from_tag,
    chunk_get_index_from_four_cc, chunk_get_tag_from_four_cc, chunk_info, chunk_init,
    chunk_list_emit, chunk_release, chunk_search_list, chunk_set_head, mux_emit_riff_header,
    mux_get_chunk_list_from_id, mux_get_chunk_list_from_id_mut, mux_image_count, mux_image_delete,
    mux_image_get_nth, mux_image_init, mux_image_push, mux_validate,
};
use crate::utils::utils::{get_le16, get_le24, get_le32, put_le24, put_le32, webp_safe_malloc};
use crate::webp::format_constants::{
    CHUNK_HEADER_SIZE, MAX_CANVAS_SIZE, MAX_CHUNK_PAYLOAD, MAX_IMAGE_AREA, RIFF_HEADER_SIZE,
    TAG_SIZE, VP8X_CHUNK_SIZE, mkfourcc,
};
use crate::webp::mux::{
    WEBP_MUX_ABI_VERSION, WebPChunkId, WebPMuxAnimParams, WebPMuxError, WebPMuxFrameInfo,
    webp_mux_new,
};
use crate::webp::mux_types::{WebPData, WebPFeatureFlags, WebPMuxAnimBlend, WebPMuxAnimDispose};
use crate::webp::types::webp_abi_is_incompatible;

/// C's conversion of an `int` to `uint64_t`, which sign-extends.
fn c_u64(value: i32) -> u64 {
    i64::from(value).cast_unsigned()
}

//------------------------------------------------------------------------------
// Helper method(s).

// upstream: src/mux/muxread.c MuxGet
/// Returns the data of the nth chunk of a non-image chunk type (C: writes it
/// into `*data`, which it first resets with `WebPDataInit`). The `SWITCH_ID_LIST`
/// macro is expanded into the match.
fn mux_get<'m, 'a>(
    mux: &'m WebPMux<'a>,
    idx: CHUNK_INDEX,
    nth: u32,
) -> Result<&'m WebPData<'a>, WebPMuxError> {
    debug_assert!(idx != CHUNK_INDEX::IDX_LAST_CHUNK);
    debug_assert!(!is_wpi(chunk_info(idx).id));

    // upstream: src/mux/muxread.c SWITCH_ID_LIST
    let list = match idx {
        CHUNK_INDEX::IDX_VP8X => &mux.vp8x,
        CHUNK_INDEX::IDX_ICCP => &mux.iccp,
        CHUNK_INDEX::IDX_ANIM => &mux.anim,
        CHUNK_INDEX::IDX_EXIF => &mux.exif,
        CHUNK_INDEX::IDX_XMP => &mux.xmp,
        _ => {
            debug_assert!(idx != CHUNK_INDEX::IDX_UNKNOWN);
            return Err(WebPMuxError::WEBP_MUX_NOT_FOUND);
        }
    };
    match chunk_search_list(list, nth, chunk_info(idx).tag) {
        Some(chunk) => Ok(&chunk.data),
        None => Err(WebPMuxError::WEBP_MUX_NOT_FOUND),
    }
}

// upstream: src/mux/muxread.c ChunkVerifyAndAssign
/// Fill the chunk with the given data (includes chunk header bytes), after some
/// verifications. `data` holds the `data_size` bytes from the chunk's header on.
fn chunk_verify_and_assign<'a>(
    chunk: &mut WebPChunk<'a>,
    data: ChunkBytes<'a, '_>,
    data_size: usize,
    riff_size: usize,
    copy_data: i32,
) -> Result<(), WebPMuxError> {
    let bytes = data.bytes();
    // Correctness checks.
    if data_size < CHUNK_HEADER_SIZE as usize {
        return Err(WebPMuxError::WEBP_MUX_NOT_ENOUGH_DATA);
    }
    let chunk_size = get_le32(&bytes[TAG_SIZE as usize..]);
    if chunk_size > MAX_CHUNK_PAYLOAD {
        return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
    }

    {
        let chunk_disk_size = size_with_padding(chunk_size as usize);
        if chunk_disk_size > riff_size {
            return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
        }
        if chunk_disk_size > data_size {
            return Err(WebPMuxError::WEBP_MUX_NOT_ENOUGH_DATA);
        }
    }

    // Data assignment.
    let chunk_data = data.range(CHUNK_HEADER_SIZE as usize, chunk_size as usize);
    chunk_assign_data(chunk, chunk_data, copy_data, get_le32(bytes))
}

// upstream: src/mux/muxread.c MuxImageFinalize
/// Update width/height/`has_alpha` info from chunks within `wpi`. Also remove ALPH
/// chunk if not needed.
///
/// # Panics
///
/// Panics when `wpi` has no image chunk, which C dereferences; no caller
/// passes one.
pub fn mux_image_finalize(wpi: &mut WebPMuxImage<'_>) -> bool {
    let img = wpi
        .img
        .as_ref()
        .expect("MuxImageFinalize needs an image chunk");
    let image = &img.data;
    let is_lossless = img.tag == chunk_info(CHUNK_INDEX::IDX_VP8L).tag;
    let info = if is_lossless {
        vp8l_get_info(&image.bytes)
    } else {
        vp8_get_info(&image.bytes, image.bytes.len()).map(|(w, h)| (w, h, false))
    };
    let Some((w, h, vp8l_has_alpha)) = info else {
        return false;
    };
    // Ignore ALPH chunk accompanying VP8L.
    if is_lossless && let Some(alpha) = wpi.alpha.take() {
        chunk_delete(alpha);
    }
    wpi.width = w;
    wpi.height = h;
    wpi.has_alpha = vp8l_has_alpha || wpi.alpha.is_some();
    true
}

// upstream: src/mux/muxread.c MuxImageParse
/// Parses the ANMF chunk `chunk` into `wpi`: its header and the ALPH, `VP8`/`VP8L`
/// and unknown chunks inside it.
fn mux_image_parse<'a>(chunk: &WebPChunk<'a>, copy_data: i32, wpi: &mut WebPMuxImage<'a>) -> bool {
    let source = ChunkBytes::of(&chunk.data);
    let mut bytes = 0_usize;
    let mut size = chunk.data.bytes.len();
    let last = size;
    let mut subchunk = WebPChunk::default();
    chunk_init(&mut subchunk);

    debug_assert_eq!(chunk.tag, chunk_info(CHUNK_INDEX::IDX_ANMF).tag);
    debug_assert!(!wpi.is_partial);

    let ok = (|| {
        // ANMF.
        {
            let hdr_size = crate::webp::format_constants::ANMF_CHUNK_SIZE as usize;
            // Each of ANMF chunk contain a header at the beginning. So, its size
            // should be at least 'hdr_size'.
            if size < hdr_size {
                return false;
            }
            let temp = source.range(0, hdr_size);
            if chunk_assign_data(&mut subchunk, temp, copy_data, chunk.tag).is_err() {
                return false;
            }
        }
        // C reads the size after the chunk moved into the list; it is the same.
        let subchunk_size = chunk_disk_size(&subchunk) - CHUNK_HEADER_SIZE as usize;
        if chunk_set_head(&mut subchunk, &mut wpi.header).is_err() {
            return false;
        }
        wpi.is_partial = true; // Waiting for ALPH and/or VP8/VP8L chunks.

        // Rest of the chunks.
        bytes += subchunk_size;
        size -= subchunk_size;

        while bytes != last {
            chunk_init(&mut subchunk);
            if chunk_verify_and_assign(
                &mut subchunk,
                source.range(bytes, size),
                size,
                size,
                copy_data,
            )
            .is_err()
            {
                return false;
            }
            let subchunk_size = chunk_disk_size(&subchunk);
            match chunk_get_id_from_tag(subchunk.tag) {
                WebPChunkId::WEBP_CHUNK_ALPHA => {
                    if wpi.alpha.is_some() {
                        return false; // Consecutive ALPH chunks.
                    }
                    if chunk_set_head(&mut subchunk, &mut wpi.alpha).is_err() {
                        return false;
                    }
                    wpi.is_partial = true; // Waiting for a VP8 chunk.
                }
                WebPChunkId::WEBP_CHUNK_IMAGE => {
                    if wpi.img.is_some() {
                        return false; // Only 1 image chunk allowed.
                    }
                    if chunk_set_head(&mut subchunk, &mut wpi.img).is_err() {
                        return false;
                    }
                    if !mux_image_finalize(wpi) {
                        return false;
                    }
                    wpi.is_partial = false; // wpi is completely filled.
                }
                WebPChunkId::WEBP_CHUNK_UNKNOWN => {
                    if wpi.is_partial {
                        return false; // Encountered an unknown chunk before some image chunks.
                    }
                    if chunk_append(&mut subchunk, &mut wpi.unknown).is_err() {
                        return false;
                    }
                }
                _ => return false,
            }
            bytes += subchunk_size;
            size -= subchunk_size;
        }
        !wpi.is_partial
    })();
    if !ok {
        // Fail:
        chunk_release(&mut subchunk);
    }
    ok
}

//------------------------------------------------------------------------------
// Create a mux object from WebP-RIFF data.

// upstream: src/mux/muxread.c WebPMuxCreateInternal
/// Internal, version-checked, entry point of `WebPMuxCreate`. With
/// `copy_data == 0` the chunks borrow the bitstream's bytes for `'a` where the
/// `WebPData` borrows them, and copy them where it owns them. C first returns
/// `NULL` for a `NULL` bitstream or `bytes`, which only the C ABI can pass.
#[expect(clippy::too_many_lines, reason = "follows the C function")]
#[must_use]
pub fn webp_mux_create_internal<'a>(
    bitstream: &WebPData<'a>,
    copy_data: i32,
    version: i32,
) -> Option<WebPMux<'a>> {
    if webp_abi_is_incompatible(version, WEBP_MUX_ABI_VERSION) {
        return None; // version mismatch
    }

    let source = ChunkBytes::of(bitstream);
    let data = source.bytes();
    let mut size = data.len();

    if size < (RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE) as usize {
        return None;
    }
    if get_le32(data) != mkfourcc(b'R', b'I', b'F', b'F')
        || get_le32(&data[CHUNK_HEADER_SIZE as usize..]) != mkfourcc(b'W', b'E', b'B', b'P')
    {
        return None;
    }

    let mut mux = webp_mux_new()?;
    let mut chunk = WebPChunk::default();
    chunk_init(&mut chunk);
    let mut wpi = WebPMuxImage::default();

    let ok = (|| {
        let tag = get_le32(&data[RIFF_HEADER_SIZE as usize..]);
        if tag != chunk_info(CHUNK_INDEX::IDX_VP8).tag
            && tag != chunk_info(CHUNK_INDEX::IDX_VP8L).tag
            && tag != chunk_info(CHUNK_INDEX::IDX_VP8X).tag
        {
            return false; // First chunk should be VP8, VP8L or VP8X.
        }

        let riff_size = get_le32(&data[TAG_SIZE as usize..]);
        if riff_size > MAX_CHUNK_PAYLOAD {
            return false;
        }

        // Note this padding is historical and differs from demux.c which does not
        // pad the file size.
        let riff_size = size_with_padding(riff_size as usize);
        // Make sure the whole RIFF header is available.
        if riff_size < RIFF_HEADER_SIZE as usize {
            return false;
        }
        if riff_size > size {
            return false;
        }
        // There's no point in reading past the end of the RIFF chunk. Note
        // riff_size includes CHUNK_HEADER_SIZE after SizeWithPadding().
        if size > riff_size {
            size = riff_size;
        }

        let end = size;
        let mut pos = RIFF_HEADER_SIZE as usize;
        size -= RIFF_HEADER_SIZE as usize;

        // C: wpi = WebPSafeMalloc(1ULL, sizeof(*wpi)), a struct value here.
        mux_image_init(&mut wpi);

        // Loop over chunks.
        while pos != end {
            if chunk_verify_and_assign(
                &mut chunk,
                source.range(pos, size),
                size,
                riff_size,
                copy_data,
            )
            .is_err()
            {
                return false;
            }
            let data_size = chunk_disk_size(&chunk);
            let id = chunk_get_id_from_tag(chunk.tag);
            let push_image = match id {
                WebPChunkId::WEBP_CHUNK_ALPHA => {
                    if wpi.alpha.is_some() {
                        return false; // Consecutive ALPH chunks.
                    }
                    if chunk_set_head(&mut chunk, &mut wpi.alpha).is_err() {
                        return false;
                    }
                    wpi.is_partial = true; // Waiting for a VP8 chunk.
                    false
                }
                WebPChunkId::WEBP_CHUNK_IMAGE => {
                    if chunk_set_head(&mut chunk, &mut wpi.img).is_err() {
                        return false;
                    }
                    if !mux_image_finalize(&mut wpi) {
                        return false;
                    }
                    wpi.is_partial = false; // wpi is completely filled.
                    true
                }
                WebPChunkId::WEBP_CHUNK_ANMF => {
                    if wpi.is_partial {
                        return false; // Previous wpi is still incomplete.
                    }
                    if !mux_image_parse(&chunk, copy_data, &mut wpi) {
                        return false;
                    }
                    chunk_release(&mut chunk);
                    true
                }
                _ => {
                    // A non-image chunk.
                    if wpi.is_partial {
                        return false; // Encountered a non-image chunk before getting all chunks of an image.
                    }
                    // C caches the end of each list in chunk_list_ends[]; a
                    // vector appends at its end.
                    if chunk_append(&mut chunk, mux_get_chunk_list_from_id_mut(&mut mux, id))
                        .is_err()
                    {
                        return false;
                    }
                    if id == WebPChunkId::WEBP_CHUNK_VP8X {
                        // grab global specs
                        if data_size < (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as usize {
                            return false;
                        }
                        mux.canvas_width = get_le24(&data[pos + 12..]) + 1;
                        mux.canvas_height = get_le24(&data[pos + 15..]) + 1;
                    }
                    false
                }
            };
            if push_image {
                // PushImage: Add this to mux->images list.
                if mux_image_push(&mut wpi, &mut mux.images).is_err() {
                    return false;
                }
                mux_image_init(&mut wpi); // Reset for reading next image.
            }
            pos += data_size;
            size -= data_size;
            chunk_init(&mut chunk);
        }

        // Incomplete image.
        if wpi.is_partial {
            return false;
        }

        // Validate mux if complete.
        mux_validate(&mux).is_ok()
    })();

    if !ok {
        // Err: Something bad happened.
        chunk_release(&mut chunk);
        mux_image_delete(wpi);
        webp_mux_delete(mux);
        return None;
    }
    mux_image_delete(wpi);
    Some(mux) // All OK;
}

//------------------------------------------------------------------------------
// Get API(s).

// upstream: src/mux/muxread.c ValidateForSingleImage
/// Validates that the given mux has a single image.
fn validate_for_single_image(mux: &WebPMux<'_>) -> Result<(), WebPMuxError> {
    let num_images = mux_image_count(&mux.images, WebPChunkId::WEBP_CHUNK_IMAGE);
    let num_frames = mux_image_count(&mux.images, WebPChunkId::WEBP_CHUNK_ANMF);

    if num_images == 0 {
        // No images in mux.
        Err(WebPMuxError::WEBP_MUX_NOT_FOUND)
    } else if num_images == 1 && num_frames == 0 {
        // Valid case (single image).
        Ok(())
    } else {
        // Frame case OR an invalid mux.
        Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT)
    }
}

// upstream: src/mux/muxread.c MuxGetCanvasInfo
/// Get the canvas width, height and flags after validating that VP8X/VP8/VP8L
/// chunk and canvas size are valid.
fn mux_get_canvas_info(mux: &WebPMux<'_>) -> Result<(i32, i32, u32), WebPMuxError> {
    let w;
    let h;
    let mut f = 0_u32;

    // Check if VP8X chunk is present.
    if let Ok(data) = mux_get(mux, CHUNK_INDEX::IDX_VP8X, 1) {
        if data.bytes.len() < VP8X_CHUNK_SIZE as usize {
            return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
        }
        f = get_le32(&data.bytes);
        w = get_le24(&data.bytes[4..]) + 1;
        h = get_le24(&data.bytes[7..]) + 1;
    } else {
        let wpi = mux.images.first();
        // Grab user-forced canvas size as default.
        let mut forced_w = mux.canvas_width;
        let mut forced_h = mux.canvas_height;
        if forced_w == 0 && forced_h == 0 && validate_for_single_image(mux).is_ok() {
            // single image and not forced canvas size => use dimension of first frame
            let wpi = wpi.expect("a single image");
            forced_w = wpi.width;
            forced_h = wpi.height;
        }
        if let Some(wpi) = wpi
            && wpi.has_alpha
        {
            f |= WebPFeatureFlags::ALPHA_FLAG.0;
        }
        w = forced_w;
        h = forced_h;
    }
    if c_u64(w).wrapping_mul(c_u64(h)) >= MAX_IMAGE_AREA {
        return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
    }

    Ok((w, h, f))
}

// upstream: src/mux/muxread.c WebPMuxGetCanvasSize
/// Gets the canvas size from the mux object: `(width, height)`. This assumes
/// that the VP8X chunk, if present, is up-to-date. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments, which only the C ABI can
/// pass.
///
/// # Errors
///
/// `WEBP_MUX_BAD_DATA` if VP8X/VP8/VP8L chunk or canvas size is invalid.
pub fn webp_mux_get_canvas_size(mux: &WebPMux<'_>) -> Result<(i32, i32), WebPMuxError> {
    mux_get_canvas_info(mux).map(|(width, height, _)| (width, height))
}

// upstream: src/mux/muxread.c WebPMuxGetFeatures
/// Gets the feature flags from the mux object, an OR of `WebPFeatureFlags`
/// values. This assumes that the VP8X chunk, if present, is up-to-date. C first
/// returns `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments.
///
/// # Errors
///
/// `WEBP_MUX_BAD_DATA` if VP8X/VP8/VP8L chunk or canvas size is invalid.
pub fn webp_mux_get_features(mux: &WebPMux<'_>) -> Result<u32, WebPMuxError> {
    mux_get_canvas_info(mux).map(|(_, _, flags)| flags)
}

// upstream: src/mux/muxread.c EmitVP8XChunk
/// Writes a VP8X chunk at `dst[pos..]`; returns the position after it.
fn emit_vp8x_chunk(dst: &mut [u8], pos: usize, width: i32, height: i32, flags: u32) -> usize {
    let vp8x_size = (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as usize;
    debug_assert!(width >= 1 && height >= 1);
    debug_assert!(width <= MAX_CANVAS_SIZE && height <= MAX_CANVAS_SIZE);
    debug_assert!(c_u64(width) * c_u64(height) < MAX_IMAGE_AREA);
    put_le32(&mut dst[pos..], mkfourcc(b'V', b'P', b'8', b'X'));
    put_le32(&mut dst[pos + TAG_SIZE as usize..], VP8X_CHUNK_SIZE as u32);
    put_le32(&mut dst[pos + CHUNK_HEADER_SIZE as usize..], flags);
    put_le24(&mut dst[pos + CHUNK_HEADER_SIZE as usize + 4..], width - 1);
    put_le24(&mut dst[pos + CHUNK_HEADER_SIZE as usize + 7..], height - 1);
    pos + vp8x_size
}

// upstream: src/mux/muxread.c SynthesizeBitstream
/// Assemble a single image WebP bitstream from `wpi`.
fn synthesize_bitstream(wpi: &WebPMuxImage<'_>) -> Result<WebPData<'static>, WebPMuxError> {
    let img = wpi.img.as_ref().expect("an image has an image chunk");
    // Allocate data.
    let need_vp8x = wpi.alpha.is_some();
    let vp8x_size = if need_vp8x {
        (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as usize
    } else {
        0
    };
    let alpha_size = match &wpi.alpha {
        Some(alpha) => chunk_disk_size(alpha),
        None => 0,
    };
    // Note: No need to output ANMF chunk for a single image.
    let size = RIFF_HEADER_SIZE as usize + vp8x_size + alpha_size + chunk_disk_size(img);
    // C: WebPSafeMalloc(1ULL, size), the same size test.
    let mut data =
        webp_safe_malloc::<u8>(size as u64, 1).ok_or(WebPMuxError::WEBP_MUX_MEMORY_ERROR)?;
    data.resize(size, 0);

    // Main RIFF header.
    let mut dst = mux_emit_riff_header(&mut data, size);

    if let Some(alpha) = &wpi.alpha {
        dst = emit_vp8x_chunk(
            &mut data,
            dst,
            wpi.width,
            wpi.height,
            WebPFeatureFlags::ALPHA_FLAG.0,
        ); // VP8X.
        dst = chunk_list_emit(core::slice::from_ref(alpha), &mut data, dst); // ALPH.
    }

    // Bitstream.
    dst = chunk_list_emit(core::slice::from_ref(img), &mut data, dst);
    debug_assert_eq!(dst, size);

    // Output.
    Ok(WebPData {
        bytes: std::borrow::Cow::Owned(data),
    })
}

// upstream: src/mux/muxread.c WebPMuxGetChunk
/// Gets a reference to the data of the chunk with id `fourcc` in the mux
/// object; the data stays in the mux. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments. On `WEBP_MUX_NOT_FOUND`
/// for a known chunk type, C has reset the caller's `WebPData` (`MuxGet`); the C
/// ABI does that.
///
/// # Errors
///
/// `WEBP_MUX_INVALID_ARGUMENT` if `fourcc` corresponds to an image chunk,
/// `WEBP_MUX_NOT_FOUND` if the mux does not contain a chunk with the given id.
pub fn webp_mux_get_chunk<'m, 'a>(
    mux: &'m WebPMux<'a>,
    fourcc: &[u8; 4],
) -> Result<&'m WebPData<'a>, WebPMuxError> {
    let idx = chunk_get_index_from_four_cc(fourcc);
    debug_assert!(idx != CHUNK_INDEX::IDX_LAST_CHUNK);
    if is_wpi(chunk_info(idx).id) {
        // An image chunk.
        Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT)
    } else if idx != CHUNK_INDEX::IDX_UNKNOWN {
        // A known chunk type.
        mux_get(mux, idx, 1)
    } else {
        // An unknown chunk type.
        let chunk = chunk_search_list(&mux.unknown, 1, chunk_get_tag_from_four_cc(fourcc));
        match chunk {
            None => Err(WebPMuxError::WEBP_MUX_NOT_FOUND),
            Some(chunk) => Ok(&chunk.data),
        }
    }
}

// upstream: src/mux/muxread.c MuxGetImageInternal
fn mux_get_image_internal(
    wpi: &WebPMuxImage<'_>,
    info: &mut WebPMuxFrameInfo<'static>,
) -> Result<(), WebPMuxError> {
    // Set some defaults for unrelated fields.
    info.x_offset = 0;
    info.y_offset = 0;
    info.duration = 1;
    info.dispose_method = WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE;
    info.blend_method = WebPMuxAnimBlend::WEBP_MUX_BLEND;
    // Extract data for related fields.
    info.id = chunk_get_id_from_tag(wpi.img.as_ref().expect("an image chunk").tag);
    info.bitstream = synthesize_bitstream(wpi)?;
    Ok(())
}

// upstream: src/mux/muxread.c MuxGetFrameInternal
fn mux_get_frame_internal(
    wpi: &WebPMuxImage<'_>,
    frame: &mut WebPMuxFrameInfo<'static>,
) -> Result<(), WebPMuxError> {
    let header = wpi.header.as_ref().expect("checked by WebPMuxGetFrame");
    let is_frame = header.tag == chunk_info(CHUNK_INDEX::IDX_ANMF).tag;
    if !is_frame {
        return Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT);
    }
    // Get frame chunk.
    let frame_data = &header.data.bytes;
    if (frame_data.len() as u64) < u64::from(chunk_info(CHUNK_INDEX::IDX_ANMF).size) {
        return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
    }
    // Extract info.
    frame.x_offset = 2 * get_le24(frame_data);
    frame.y_offset = 2 * get_le24(&frame_data[3..]);
    {
        let bits = frame_data[15];
        frame.duration = get_le24(&frame_data[12..]);
        frame.dispose_method = if bits & 1 != 0 {
            WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND
        } else {
            WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE
        };
        frame.blend_method = if bits & 2 != 0 {
            WebPMuxAnimBlend::WEBP_MUX_NO_BLEND
        } else {
            WebPMuxAnimBlend::WEBP_MUX_BLEND
        };
    }
    frame.id = chunk_get_id_from_tag(header.tag);
    frame.bitstream = synthesize_bitstream(wpi)?;
    Ok(())
}

// upstream: src/mux/muxread.c WebPMuxGetFrame
/// Gets the nth frame from the mux object, with a copy of its image data as a
/// single-image WebP file (C allocates it; the caller clears it). `nth = 0` has
/// a special meaning: last position. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments. On a memory error C has
/// already written the other fields of `*frame`; the result holds none.
///
/// # Errors
///
/// `WEBP_MUX_NOT_FOUND` if there are less than `nth` frames,
/// `WEBP_MUX_INVALID_ARGUMENT` or `WEBP_MUX_BAD_DATA` if the nth frame chunk is
/// invalid, `WEBP_MUX_MEMORY_ERROR` on memory allocation error.
pub fn webp_mux_get_frame(
    mux: &WebPMux<'_>,
    nth: u32,
) -> Result<WebPMuxFrameInfo<'static>, WebPMuxError> {
    // Get the nth WebPMuxImage.
    let position = mux_image_get_nth(&mux.images, nth)?;
    let wpi = &mux.images[position];

    // Get frame info.
    let mut frame = WebPMuxFrameInfo::default();
    if wpi.header.is_none() {
        mux_get_image_internal(wpi, &mut frame)?;
    } else {
        mux_get_frame_internal(wpi, &mut frame)?;
    }
    Ok(frame)
}

// upstream: src/mux/muxread.c WebPMuxGetAnimationParams
/// Gets the animation parameters from the ANIM chunk of the mux object. C first
/// returns `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments.
///
/// # Errors
///
/// `WEBP_MUX_NOT_FOUND` if the ANIM chunk is not present, `WEBP_MUX_BAD_DATA`
/// if it is too short.
pub fn webp_mux_get_animation_params(mux: &WebPMux<'_>) -> Result<WebPMuxAnimParams, WebPMuxError> {
    let anim = mux_get(mux, CHUNK_INDEX::IDX_ANIM, 1)?;
    if (anim.bytes.len() as u64) < u64::from(CHUNKS[WebPChunkId::WEBP_CHUNK_ANIM.0 as usize].size) {
        return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
    }
    Ok(WebPMuxAnimParams {
        bgcolor: get_le32(&anim.bytes),
        loop_count: get_le16(&anim.bytes[4..]),
    })
}

// upstream: src/mux/muxread.c ChunkGetIndexFromId
/// Get chunk index from chunk id. Returns `IDX_NIL` if not found.
fn chunk_get_index_from_id(id: WebPChunkId) -> CHUNK_INDEX {
    let mut i = 0;
    while CHUNKS[i].id != WebPChunkId::WEBP_CHUNK_NIL {
        if id == CHUNKS[i].id {
            return CHUNK_INDEX(u32::try_from(i).expect("an index of CHUNKS"));
        }
        i += 1;
    }
    CHUNK_INDEX::IDX_NIL
}

// upstream: src/mux/muxread.c CountChunks
/// Count number of chunks matching `tag` in the `chunk_list`. If
/// `tag == NIL_TAG`, any tag will be matched.
fn count_chunks(chunk_list: &[WebPChunk<'_>], tag: u32) -> i32 {
    let mut count = 0;
    for current in chunk_list {
        if tag == NIL_TAG || current.tag == tag {
            count += 1; // Count chunks whose tags match.
        }
    }
    count
}

// upstream: src/mux/muxread.c WebPMuxNumChunks
/// Gets number of chunks with the given `id` in the mux object. C first
/// returns `WEBP_MUX_INVALID_ARGUMENT` for `NULL` arguments; otherwise it
/// succeeds.
///
/// # Errors
///
/// None besides C's `NULL` checks.
pub fn webp_mux_num_chunks(mux: &WebPMux<'_>, id: WebPChunkId) -> Result<i32, WebPMuxError> {
    if is_wpi(id) {
        Ok(mux_image_count(&mux.images, id))
    } else {
        let chunk_list = mux_get_chunk_list_from_id(mux, id);
        let idx = chunk_get_index_from_id(id);
        Ok(count_chunks(chunk_list, chunk_info(idx).tag))
    }
}
