// libwebp v1.6.0, src/dec/webp_dec.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Main decoding functions for WEBP images.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/webp_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP12 (decoder integration).
//!
//! Exception: WP7 translates `WebPIoInitFromOptions`, `WebPCheckCropDimensions` and `WebPCheckCropDimensionsBasic` from `src/dec/webp_dec.c`, because `VP8LDecodeImage` calls `WebPIoInitFromOptions`, which checks the crop with the other two.

// The items below are WP7's exception in this module (`tools/units.json`):
// `WebPIoInitFromOptions` and the two crop checks that it calls.

use crate::dec::vp8_dec::VP8Io;
use crate::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;
use crate::webp::decode::{WEBP_CSP_MODE, WebPDecoderOptions, webp_is_rgb_mode};

// upstream: src/dec/webp_dec.c WebPCheckCropDimensionsBasic
/// Returns true if the crop offset is not negative and its size is positive.
fn webp_check_crop_dimensions_basic(x: i32, y: i32, w: i32, h: i32) -> bool {
    !(x < 0 || y < 0 || w <= 0 || h <= 0)
}

// upstream: src/dec/webp_dec.c WebPCheckCropDimensions
/// Returns true if crop dimensions are within image bounds.
#[must_use]
pub fn webp_check_crop_dimensions(
    image_width: i32,
    image_height: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> bool {
    webp_check_crop_dimensions_basic(x, y, w, h)
        && !(x >= image_width
            || w > image_width
            || w > image_width - x
            || y >= image_height
            || h > image_height
            || h > image_height - y)
}

// upstream: src/dec/webp_dec.c WebPIoInitFromOptions
/// Sets up the crop fields, `mb_w` and `mb_h` of `io`, and its scaling,
/// filtering and upsampling fields, from `options` (C: `NULL` is `None`).
/// `src_colorspace` refers to the compressed format, not the output one. Returns
/// false for a crop outside the picture or invalid scaled dimensions; the fields
/// set before the failing check keep their new values, as in C.
///
/// # Panics
///
/// Debug builds panic when `io.width * 3` or `io.height * 3` overflows with
/// scaling enabled, where the C `int` arithmetic is undefined.
#[must_use]
pub fn webp_io_init_from_options(
    options: Option<&WebPDecoderOptions>,
    io: &mut VP8Io,
    src_colorspace: WEBP_CSP_MODE,
) -> bool {
    let big_w = io.width;
    let big_h = io.height;
    let mut x = 0;
    let mut y = 0;
    let mut w = big_w;
    let mut h = big_h;

    // Cropping
    io.use_cropping = options.is_some_and(|options| options.use_cropping != 0);
    if let Some(options) = options.filter(|_| io.use_cropping) {
        w = options.crop_width;
        h = options.crop_height;
        x = options.crop_left;
        y = options.crop_top;
        if !webp_is_rgb_mode(src_colorspace) {
            // only snap for YUV420
            x &= !1;
            y &= !1;
        }
        if !webp_check_crop_dimensions(big_w, big_h, x, y, w, h) {
            return false; // out of frame boundary error
        }
    }
    io.crop_left = x;
    io.crop_top = y;
    io.crop_right = x + w;
    io.crop_bottom = y + h;
    io.mb_w = w;
    io.mb_h = h;

    // Scaling
    io.use_scaling = options.is_some_and(|options| options.use_scaling != 0);
    if let Some(options) = options.filter(|_| io.use_scaling) {
        let mut scaled_width = options.scaled_width;
        let mut scaled_height = options.scaled_height;
        if !webp_rescaler_get_scaled_dimensions(w, h, &mut scaled_width, &mut scaled_height) {
            return false;
        }
        io.scaled_width = scaled_width;
        io.scaled_height = scaled_height;
    }

    // Filter
    io.bypass_filtering = options.is_some_and(|options| options.bypass_filtering != 0);

    // Fancy upsampler (FANCY_UPSAMPLING is defined in src/dsp/dsp.h)
    io.fancy_upsampling = options.is_none_or(|options| options.no_fancy_upsampling == 0);

    if io.use_scaling {
        // disable filter (only for large downscaling ratio).
        io.bypass_filtering |=
            (io.scaled_width < big_w * 3 / 4) && (io.scaled_height < big_h * 3 / 4);
        io.fancy_upsampling = false;
    }
    true
}

// The items below translate the rest of `src/dec/webp_dec.c` (WP12).

use crate::dec::buffer_dec::{
    webp_allocate_dec_buffer, webp_avoid_slow_memory, webp_copy_dec_buffer_pixels,
    webp_flip_buffer, webp_free_dec_buffer,
};
use crate::dec::frame_dec::{vp8_get_thread_method, vp8_init_dithering};
use crate::dec::io_dec::webp_init_custom_io;
use crate::dec::vp8_dec::{vp8_decode, vp8_get_headers, vp8_get_info, vp8_init_io, vp8_new};
use crate::dec::vp8l_dec::{
    vp8l_check_signature, vp8l_decode_header, vp8l_decode_image, vp8l_get_info, vp8l_new,
};
use crate::dec::webpi_dec::{WebPDecParams, WebPHeaderStructure};
use crate::utils::utils::{get_le24, get_le32};
use crate::webp::decode::{
    VP8StatusCode, WEBP_DECODER_ABI_VERSION, WebPBitstreamFeatures, WebPDecBuffer,
    WebPDecBufferPlanes, WebPDecoderConfig, WebPRGBABuffer, WebPYUVABuffer, webp_init_dec_buffer,
};
use crate::webp::format_constants::{
    CHUNK_HEADER_SIZE, MAX_CHUNK_PAYLOAD, MAX_IMAGE_AREA, RIFF_HEADER_SIZE, TAG_SIZE,
    VP8_FRAME_HEADER_SIZE, VP8L_FRAME_HEADER_SIZE, VP8X_CHUNK_SIZE,
};
use crate::webp::mux_types::WebPFeatureFlags;
use crate::webp::types::{PlaneOffset, PlaneStorage, webp_abi_is_incompatible};

/// `TAG_SIZE` as a size.
const TAG: usize = TAG_SIZE.unsigned_abs() as usize;
/// `CHUNK_HEADER_SIZE` as a size.
const CHUNK_HEADER: usize = CHUNK_HEADER_SIZE.unsigned_abs() as usize;
/// `RIFF_HEADER_SIZE` as a size.
const RIFF_HEADER: usize = RIFF_HEADER_SIZE.unsigned_abs() as usize;

/// The status as a `Result`: `VP8_STATUS_OK` is `Ok` (ADR 0003, errors).
fn status_result(status: VP8StatusCode) -> Result<(), VP8StatusCode> {
    if status == VP8StatusCode::VP8_STATUS_OK {
        Ok(())
    } else {
        Err(status)
    }
}

/// C's `memcmp(data + pos, tag, TAG_SIZE) == 0`.
fn has_tag(data: &[u8], pos: usize, tag: [u8; 4]) -> bool {
    data[pos..pos + TAG] == tag[..]
}

//------------------------------------------------------------------------------
// RIFF layout is:
//   Offset  tag
//   0...3   "RIFF" 4-byte tag
//   4...7   size of image data (including metadata) starting at offset 8
//   8...11  "WEBP"   our form-type signature
// The RIFF container (12 bytes) is followed by appropriate chunks:
//   12..15  "VP8 ": 4-bytes tags, signaling the use of VP8 video format
//   16..19  size of the raw VP8 image data, starting at offset 20
//   20....  the VP8 bytes
// Or,
//   12..15  "VP8L": 4-bytes tags, signaling the use of VP8L lossless format
//   16..19  size of the raw VP8L image data, starting at offset 20
//   20....  the VP8L bytes
// Or,
//   12..15  "VP8X": 4-bytes tags, describing the extended-VP8 chunk.
//   16..19  size of the VP8X chunk starting at offset 20.
//   20..23  VP8X flags bit-map corresponding to the chunk-types present.
//   24..26  Width of the Canvas Image.
//   27..29  Height of the Canvas Image.
// There can be extra chunks after the "VP8X" chunk (ICCP, ANMF, VP8, VP8L,
// XMP, EXIF  ...)
// All sizes are in little-endian order.
// Note: chunk data size must be padded to multiple of 2 when written.
//
// The parsers below take the input and C's `*data` and `*data_size` as the
// position `pos` in it and the number of bytes from there on.

// upstream: src/dec/webp_dec.c ParseRIFF
/// Validates the RIFF container (if detected) and skips over it. If a RIFF
/// container is detected, returns `VP8_STATUS_BITSTREAM_ERROR` for an invalid
/// header, `VP8_STATUS_NOT_ENOUGH_DATA` for truncated data if `have_all_data`
/// is true, and `Ok` otherwise. In case there are not enough bytes (partial
/// RIFF container), sets `riff_size` to 0; else to the RIFF size extracted from
/// the header.
fn parse_riff(
    data: &[u8],
    pos: &mut usize,
    data_size: &mut usize,
    have_all_data: bool,
    riff_size: &mut usize,
) -> Result<(), VP8StatusCode> {
    *riff_size = 0; // Default: no RIFF present.
    if *data_size >= RIFF_HEADER && has_tag(data, *pos, *b"RIFF") {
        if !has_tag(data, *pos + 8, *b"WEBP") {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // Wrong image file signature.
        }
        let size = get_le32(&data[*pos + TAG..]);
        // Check that we have at least one chunk (i.e "WEBP" + "VP8?nnnn").
        if size < (TAG_SIZE + CHUNK_HEADER_SIZE).cast_unsigned() {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
        }
        if size > MAX_CHUNK_PAYLOAD {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
        }
        if have_all_data && (size as usize > *data_size - CHUNK_HEADER) {
            return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA); // Truncated bitstream.
        }
        // We have a RIFF container. Skip it.
        *riff_size = size as usize;
        *pos += RIFF_HEADER;
        *data_size -= RIFF_HEADER;
    }
    Ok(())
}

// upstream: src/dec/webp_dec.c ParseVP8X
/// Validates the VP8X header and skips over it. Returns
/// `VP8_STATUS_BITSTREAM_ERROR` for an invalid VP8X header,
/// `VP8_STATUS_NOT_ENOUGH_DATA` in case of insufficient data, and `Ok`
/// otherwise. If a VP8X chunk is found, `found_vp8x` is set to true and
/// `width_ptr`, `height_ptr` and `flags_ptr` to the values extracted from it.
fn parse_vp8x(
    data: &[u8],
    pos: &mut usize,
    data_size: &mut usize,
    found_vp8x: &mut bool,
    width_ptr: Option<&mut i32>,
    height_ptr: Option<&mut i32>,
    flags_ptr: Option<&mut u32>,
) -> Result<(), VP8StatusCode> {
    let vp8x_size = (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE).unsigned_abs() as usize;

    *found_vp8x = false;

    if *data_size < CHUNK_HEADER {
        return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA); // Insufficient data.
    }

    if has_tag(data, *pos, *b"VP8X") {
        let chunk_size = get_le32(&data[*pos + TAG..]);
        if chunk_size != VP8X_CHUNK_SIZE.cast_unsigned() {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // Wrong chunk size.
        }

        // Verify if enough data is available to validate the VP8X chunk.
        if *data_size < vp8x_size {
            return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA); // Insufficient data.
        }
        let flags = get_le32(&data[*pos + 8..]);
        let width = 1 + get_le24(&data[*pos + 12..]);
        let height = 1 + get_le24(&data[*pos + 15..]);
        if i64::from(width).cast_unsigned() * i64::from(height).cast_unsigned() >= MAX_IMAGE_AREA {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // image is too large
        }

        if let Some(flags_ptr) = flags_ptr {
            *flags_ptr = flags;
        }
        if let Some(width_ptr) = width_ptr {
            *width_ptr = width;
        }
        if let Some(height_ptr) = height_ptr {
            *height_ptr = height;
        }
        // Skip over VP8X header bytes.
        *pos += vp8x_size;
        *data_size -= vp8x_size;
        *found_vp8x = true;
    }
    Ok(())
}

// upstream: src/dec/webp_dec.c ParseOptionalChunks
/// Skips to the next VP8/VP8L chunk header in the data given the size of the
/// RIFF chunk `riff_size`. Returns `VP8_STATUS_BITSTREAM_ERROR` if any invalid
/// chunk size is encountered, `VP8_STATUS_NOT_ENOUGH_DATA` in case of
/// insufficient data, and `Ok` otherwise. If an alpha chunk is found,
/// `alpha_data` (a position in `data`) and `alpha_size` are set appropriately.
fn parse_optional_chunks(
    data: &[u8],
    pos: &mut usize,
    data_size: &mut usize,
    riff_size: usize,
    alpha_data: &mut Option<usize>,
    alpha_size: &mut usize,
) -> Result<(), VP8StatusCode> {
    let mut buf = *pos;
    let mut buf_size = *data_size;
    let mut total_size: u32 = (TAG_SIZE // "WEBP".
        + CHUNK_HEADER_SIZE // "VP8Xnnnn".
        + VP8X_CHUNK_SIZE) // data.
        .cast_unsigned();

    *alpha_data = None;
    *alpha_size = 0;

    loop {
        *pos = buf;
        *data_size = buf_size;

        if buf_size < CHUNK_HEADER {
            // Insufficient data.
            return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
        }

        let chunk_size = get_le32(&data[buf + TAG..]);
        if chunk_size > MAX_CHUNK_PAYLOAD {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // Not a valid chunk size.
        }
        // For odd-sized chunk-payload, there's one byte padding at the end.
        let disk_chunk_size = (CHUNK_HEADER_SIZE.cast_unsigned() + chunk_size + 1) & !1u32;
        total_size = total_size.wrapping_add(disk_chunk_size);

        // Check that total bytes skipped so far does not exceed riff_size.
        if riff_size > 0 && (total_size as usize > riff_size) {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // Not a valid chunk size.
        }

        // Start of a (possibly incomplete) VP8/VP8L chunk implies that we have
        // parsed all the optional chunks.
        // Note: This check must occur before the check 'buf_size < disk_chunk_size'
        // below to allow incomplete VP8/VP8L chunks.
        if has_tag(data, buf, *b"VP8 ") || has_tag(data, buf, *b"VP8L") {
            return Ok(());
        }

        if buf_size < disk_chunk_size as usize {
            // Insufficient data.
            return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
        }

        if has_tag(data, buf, *b"ALPH") {
            // A valid ALPH header.
            *alpha_data = Some(buf + CHUNK_HEADER);
            *alpha_size = chunk_size as usize;
        }

        // We have a full and valid chunk; skip it.
        buf += disk_chunk_size as usize;
        buf_size -= disk_chunk_size as usize;
    }
}

// upstream: src/dec/webp_dec.c ParseVP8Header
/// Validates the VP8/VP8L header ("VP8 nnnn" or "VP8L nnnn") and skips over it.
/// Returns `VP8_STATUS_BITSTREAM_ERROR` for an invalid (chunk larger than
/// `riff_size`) VP8/VP8L header, `VP8_STATUS_NOT_ENOUGH_DATA` in case of
/// insufficient data, and `Ok` otherwise. If a VP8/VP8L chunk is found,
/// `chunk_size` is set to the total number of bytes extracted from the chunk
/// header. `is_lossless` is set in case of a VP8L chunk or raw VP8L data. C
/// reads the four tag bytes before it tests the size; its callers make sure
/// that they are there.
fn parse_vp8_header(
    data: &[u8],
    pos: &mut usize,
    data_size: &mut usize,
    have_all_data: bool,
    riff_size: usize,
    chunk_size: &mut usize,
    is_lossless: &mut bool,
) -> Result<(), VP8StatusCode> {
    let is_vp8 = has_tag(data, *pos, *b"VP8 ");
    let is_vp8l = has_tag(data, *pos, *b"VP8L");
    let minimal_size = (TAG_SIZE + CHUNK_HEADER_SIZE).unsigned_abs() as usize; // "WEBP" + "VP8 nnnn" OR
    // "WEBP" + "VP8Lnnnn"

    if *data_size < CHUNK_HEADER {
        return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA); // Insufficient data.
    }

    if is_vp8 || is_vp8l {
        // Bitstream contains VP8/VP8L header.
        let size = get_le32(&data[*pos + TAG..]) as usize;
        if (riff_size >= minimal_size) && (size > riff_size - minimal_size) {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // Inconsistent size information.
        }
        if have_all_data && (size > *data_size - CHUNK_HEADER) {
            return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA); // Truncated bitstream.
        }
        // Skip over CHUNK_HEADER_SIZE bytes from VP8/VP8L Header.
        *chunk_size = size;
        *pos += CHUNK_HEADER;
        *data_size -= CHUNK_HEADER;
        *is_lossless = is_vp8l;
    } else {
        // Raw VP8/VP8L bitstream (no header).
        *is_lossless = vp8l_check_signature(&data[*pos..*pos + *data_size]);
        *chunk_size = *data_size;
    }

    Ok(())
}

//------------------------------------------------------------------------------

// upstream: src/dec/webp_dec.c ParseHeadersInternal
/// Fetches `width`, `height`, `has_alpha` and fills out `headers` based on
/// `data`. All the output parameters may be `None`. If `headers` is `None`
/// only the minimal amount will be read to fetch the remaining parameters. If
/// `headers` is not `None` this function will attempt to locate both alpha data
/// (with or without a VP8X chunk) and the bitstream chunk (VP8/VP8L).
///
/// Note: The following chunk sequences (before the raw VP8/VP8L data) are
/// considered valid by this function:
/// RIFF + VP8(L)
/// RIFF + VP8X + (optional chunks) + VP8(L)
/// ALPH + VP8 <-- Not a valid WebP format: only allowed for internal purpose.
/// VP8(L)     <-- Not a valid WebP format: only allowed for internal purpose.
///
/// C returns `VP8_STATUS_NOT_ENOUGH_DATA` for a `NULL` `data`, which exists
/// only at the C ABI.
#[expect(clippy::too_many_lines, reason = "the C function")]
fn parse_headers_internal<'d>(
    data: &'d [u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
    has_alpha: Option<&mut i32>,
    has_animation: Option<&mut i32>,
    format: Option<&mut i32>,
    headers: Option<&mut WebPHeaderStructure<'d>>,
) -> Result<(), VP8StatusCode> {
    let mut has_alpha = has_alpha;
    let mut format = format;
    let mut canvas_width = 0;
    let mut canvas_height = 0;
    let mut image_width;
    let mut image_height;
    let mut found_vp8x = false;
    let have_all_data = headers
        .as_ref()
        .is_some_and(|headers| headers.have_all_data);
    let has_headers = headers.is_some();

    if data.len() < RIFF_HEADER {
        return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
    }
    // C: memset(&hdrs, 0, sizeof(hdrs)), then the input.
    let mut hdrs = WebPHeaderStructure {
        data,
        ..WebPHeaderStructure::default()
    };
    let mut pos = 0;
    let mut data_size = data.len();

    // Skip over RIFF header.
    parse_riff(
        data,
        &mut pos,
        &mut data_size,
        have_all_data,
        &mut hdrs.riff_size,
    )?;
    let found_riff = hdrs.riff_size > 0;

    let status = 'return_width_height: {
        // Skip over VP8X.
        let animation_present;
        {
            let mut flags = 0u32;
            parse_vp8x(
                data,
                &mut pos,
                &mut data_size,
                &mut found_vp8x,
                Some(&mut canvas_width),
                Some(&mut canvas_height),
                Some(&mut flags),
            )?; // Wrong VP8X / insufficient data.
            animation_present = (flags & WebPFeatureFlags::ANIMATION_FLAG.0) != 0;
            if !found_riff && found_vp8x {
                // Note: This restriction may be removed in the future, if it becomes
                // necessary to send VP8X chunk to the decoder.
                return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
            }
            if let Some(has_alpha) = has_alpha.as_deref_mut() {
                *has_alpha = i32::from((flags & WebPFeatureFlags::ALPHA_FLAG.0) != 0);
            }
            if let Some(has_animation) = has_animation {
                *has_animation = i32::from(animation_present);
            }
            if let Some(format) = format.as_deref_mut() {
                *format = 0; // default = undefined
            }

            image_width = canvas_width;
            image_height = canvas_height;
            if found_vp8x && animation_present && !has_headers {
                break 'return_width_height Ok(()); // Just return features from VP8X header.
            }
        }

        if data_size < TAG {
            break 'return_width_height Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
        }

        // Skip over optional chunks if data started with "RIFF + VP8X" or "ALPH".
        if (found_riff && found_vp8x)
            || (!found_riff && !found_vp8x && has_tag(data, pos, *b"ALPH"))
        {
            let status = parse_optional_chunks(
                data,
                &mut pos,
                &mut data_size,
                hdrs.riff_size,
                &mut hdrs.alpha_data,
                &mut hdrs.alpha_data_size,
            );
            if status.is_err() {
                break 'return_width_height status; // Invalid chunk size / insufficient data.
            }
        }

        // Skip over VP8/VP8L header.
        let status = parse_vp8_header(
            data,
            &mut pos,
            &mut data_size,
            have_all_data,
            hdrs.riff_size,
            &mut hdrs.compressed_size,
            &mut hdrs.is_lossless,
        );
        if status.is_err() {
            break 'return_width_height status; // Wrong VP8/VP8L chunk-header / insufficient data.
        }
        if hdrs.compressed_size > MAX_CHUNK_PAYLOAD as usize {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
        }

        if let Some(format) = format
            && !animation_present
        {
            *format = if hdrs.is_lossless { 2 } else { 1 };
        }

        if hdrs.is_lossless {
            if data_size < VP8L_FRAME_HEADER_SIZE.unsigned_abs() as usize {
                break 'return_width_height Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
            }
            // Validates raw VP8L data.
            let Some((w, h, alpha)) = vp8l_get_info(&data[pos..pos + data_size]) else {
                return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
            };
            image_width = w;
            image_height = h;
            if let Some(has_alpha) = has_alpha.as_deref_mut() {
                *has_alpha = i32::from(alpha);
            }
        } else {
            if data_size < VP8_FRAME_HEADER_SIZE.unsigned_abs() as usize {
                break 'return_width_height Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
            }
            // Validates raw VP8 data. C passes the size as a uint32_t, which
            // holds it after the test above.
            let Some((w, h)) = vp8_get_info(&data[pos..pos + data_size], hdrs.compressed_size)
            else {
                return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
            };
            image_width = w;
            image_height = h;
        }
        // Validates image size coherency.
        if found_vp8x && (canvas_width != image_width || canvas_height != image_height) {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
        }
        if let Some(headers) = headers {
            *headers = hdrs;
            headers.offset = pos;
            debug_assert!((pos as u64) < u64::from(MAX_CHUNK_PAYLOAD));
            debug_assert_eq!(headers.offset, headers.data.len() - data_size);
        }
        Ok(())
    };
    // ReturnWidthHeight:
    if status.is_ok()
        || (status == Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA) && found_vp8x && !has_headers)
    {
        if let Some(has_alpha) = has_alpha {
            // If the data did not contain a VP8X/VP8L chunk the only definitive way
            // to set this is by looking for alpha data (from an ALPH chunk).
            *has_alpha |= i32::from(hdrs.alpha_data.is_some());
        }
        if let Some(width) = width {
            *width = image_width;
        }
        if let Some(height) = height {
            *height = image_height;
        }
        Ok(())
    } else {
        status
    }
}

// upstream: src/dec/webp_dec.c WebPParseHeaders
/// Skips over all valid chunks prior to the first VP8/VP8L frame header.
/// In `headers`, `compressed_size`, `offset`, `alpha_data`, `alpha_size` and
/// `is_lossless` are updated appropriately upon success. (C marks its status
/// `volatile` to work around a clang 3.8 bug on aarch64.)
///
/// # Errors
///
/// `VP8_STATUS_BITSTREAM_ERROR` (invalid header/chunk),
/// `VP8_STATUS_NOT_ENOUGH_DATA` (partial input) or
/// `VP8_STATUS_UNSUPPORTED_FEATURE` in the case of non-decodable features
/// (animation for instance).
pub fn webp_parse_headers(headers: &mut WebPHeaderStructure<'_>) -> Result<(), VP8StatusCode> {
    let mut has_animation = 0;
    let data = headers.data;
    // fill out headers, ignore width/height/has_alpha.
    let mut status = parse_headers_internal(
        data,
        None,
        None,
        None,
        Some(&mut has_animation),
        None,
        Some(headers),
    );
    if status.is_ok() || status == Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA) {
        // The WebPDemux API + libwebp can be used to decode individual
        // uncomposited frames or the WebPAnimDecoder can be used to fully
        // reconstruct them (see webp/demux.h).
        if has_animation != 0 {
            status = Err(VP8StatusCode::VP8_STATUS_UNSUPPORTED_FEATURE);
        }
    }
    status
}

//------------------------------------------------------------------------------
// WebPDecParams

// upstream: src/dec/webp_dec.c WebPResetDecParams
/// Must be called first, before any use of the `WebPDecParams` object: zeroes
/// it. C accepts a `NULL` object, which exists only at the C ABI.
pub fn webp_reset_dec_params(params: &mut WebPDecParams<'_>) {
    *params = WebPDecParams::default();
}

//------------------------------------------------------------------------------
// "Into" decoding variants

// upstream: src/dec/webp_dec.c DecodeInto
/// Main flow. `output` is C's `params->output` (ADR 0003, "Decoding context per
/// call"). C plugs the custom I/O functions into `io` right after setting up
/// the input; they borrow `params` and `output` here, so they are made where
/// `VP8Decode` uses them, after `WebPAllocateDecBuffer` has used `output`.
/// `VP8LDecodeImage` takes the same pair, as C reads it through `io->opaque`.
fn decode_into(
    data: &[u8],
    params: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> Result<(), VP8StatusCode> {
    let mut headers = WebPHeaderStructure {
        data,
        have_all_data: true,
        ..WebPHeaderStructure::default()
    };
    webp_parse_headers(&mut headers)?; // Process Pre-VP8 chunks.

    let mut io = VP8Io::default();
    if !vp8_init_io(&mut io) {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }
    // C: io.data, io.data_size.
    let io_data = &headers.data[headers.offset..];

    let mut status;
    if headers.is_lossless {
        let mut dec = vp8l_new();
        if let Err(dec_status) = vp8l_decode_header(&mut dec, &mut io, io_data) {
            status = Err(dec_status); // An error occurred. Grab error status.
        } else {
            // Allocate/check output buffers.
            status = webp_allocate_dec_buffer(io.width, io.height, params.options, output);
            if status.is_ok() {
                // Decode
                if let Err(dec_status) =
                    vp8l_decode_image(&mut dec, &mut io, io_data, params, output)
                {
                    status = Err(dec_status);
                }
            }
        }
        // C: VP8LDelete(dec).
    } else {
        let mut dec = vp8_new();
        // C: dec->alpha_data, dec->alpha_data_size (ADR 0003: passed per call).
        let alpha_data = headers
            .alpha_data
            .map(|alpha| &headers.data[alpha..alpha + headers.alpha_data_size]);

        // Decode bitstream header, update io->width/io->height.
        if vp8_get_headers(&mut dec, &mut io, io_data).is_err() {
            status = status_result(dec.status); // An error occurred. Grab error status.
        } else {
            // Allocate/check output buffers.
            status = webp_allocate_dec_buffer(io.width, io.height, params.options, output);
            if status.is_ok() {
                // Decode
                // This change must be done before calling VP8Decode()
                debug_assert!(!headers.is_lossless); // C: in VP8GetThreadMethod
                dec.mt_method = vp8_get_thread_method(params.options, io.width, io.height);
                vp8_init_dithering(params.options, &mut dec);
                let mut hooks = webp_init_custom_io(params, output, &mut io);
                if !vp8_decode(&mut dec, &mut io, io_data, alpha_data, Some(&mut hooks)) {
                    status = status_result(dec.status);
                }
            }
        }
        // C: VP8Delete(dec).
    }

    if status.is_err() {
        webp_free_dec_buffer(output);
    } else if params.options.is_some_and(|options| options.flip != 0) {
        // This restores the original stride values if options->flip was used
        // during the call to WebPAllocateDecBuffer above.
        status = webp_flip_buffer(output);
    }
    status
}

// Helpers

// upstream: src/dec/webp_dec.c DecodeIntoRGBABuffer
/// Decodes into the caller's `rgba` (C: the pointer and its `size`). Returns
/// the same slice on success, where C returns the pointer, and `None` on
/// failure. C returns `NULL` for a `NULL` buffer, which exists only at the C
/// ABI.
fn decode_into_rgba_buffer<'a>(
    colorspace: WEBP_CSP_MODE,
    data: &[u8],
    rgba: &'a mut [u8],
    stride: i32,
) -> Option<&'a mut [u8]> {
    let mut params = WebPDecParams::default();
    let mut buf = WebPDecBuffer::default();
    if !webp_init_dec_buffer(&mut buf) {
        return None;
    }
    webp_reset_dec_params(&mut params);
    // C: params.output = &buf.
    buf.colorspace = colorspace;
    buf.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
        rgba: PlaneOffset::default(),
        stride,
        size: rgba.len(),
    });
    // C: buf.is_external_memory = 1.
    buf.storage = PlaneStorage::External(vec![rgba]);
    if decode_into(data, &mut params, &mut buf).is_err() {
        return None;
    }
    external_region(buf.storage)
}

/// The first region of an external storage, which the caller lent: what C's
/// `Into` functions return as the caller's pointer.
fn external_region(storage: PlaneStorage<'_, u8>) -> Option<&mut [u8]> {
    match storage {
        PlaneStorage::External(slices) => slices.into_iter().next(),
        PlaneStorage::Internal(_) | PlaneStorage::Shared(_) => None,
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeRGBInto
/// Decodes into a pre-allocated buffer: `output` holds the samples, `stride`
/// is the distance in bytes between scanlines. Returns `output` on success and
/// `None` if the decoding failed or `output` is too small.
#[must_use]
pub fn webp_decode_rgb_into<'a>(
    data: &[u8],
    output: &'a mut [u8],
    stride: i32,
) -> Option<&'a mut [u8]> {
    decode_into_rgba_buffer(WEBP_CSP_MODE::MODE_RGB, data, output, stride)
}

// upstream: src/dec/webp_dec.c WebPDecodeRGBAInto
/// [`webp_decode_rgb_into`] for RGBA samples.
#[must_use]
pub fn webp_decode_rgba_into<'a>(
    data: &[u8],
    output: &'a mut [u8],
    stride: i32,
) -> Option<&'a mut [u8]> {
    decode_into_rgba_buffer(WEBP_CSP_MODE::MODE_RGBA, data, output, stride)
}

// upstream: src/dec/webp_dec.c WebPDecodeARGBInto
/// [`webp_decode_rgb_into`] for ARGB samples.
#[must_use]
pub fn webp_decode_argb_into<'a>(
    data: &[u8],
    output: &'a mut [u8],
    stride: i32,
) -> Option<&'a mut [u8]> {
    decode_into_rgba_buffer(WEBP_CSP_MODE::MODE_ARGB, data, output, stride)
}

// upstream: src/dec/webp_dec.c WebPDecodeBGRInto
/// [`webp_decode_rgb_into`] for BGR samples.
#[must_use]
pub fn webp_decode_bgr_into<'a>(
    data: &[u8],
    output: &'a mut [u8],
    stride: i32,
) -> Option<&'a mut [u8]> {
    decode_into_rgba_buffer(WEBP_CSP_MODE::MODE_BGR, data, output, stride)
}

// upstream: src/dec/webp_dec.c WebPDecodeBGRAInto
/// [`webp_decode_rgb_into`] for BGRA samples.
#[must_use]
pub fn webp_decode_bgra_into<'a>(
    data: &[u8],
    output: &'a mut [u8],
    stride: i32,
) -> Option<&'a mut [u8]> {
    decode_into_rgba_buffer(WEBP_CSP_MODE::MODE_BGRA, data, output, stride)
}

// upstream: src/dec/webp_dec.c WebPDecodeYUVInto
/// Decodes the Y, U and V planes into the caller's `luma`, `u` and `v` with
/// their strides (C: each pointer with its size). Returns `luma` on success and
/// `None` on failure. C returns `NULL` for a `NULL` luma, which exists only at
/// the C ABI.
#[must_use]
pub fn webp_decode_yuv_into<'a>(
    data: &[u8],
    luma: &'a mut [u8],
    luma_stride: i32,
    u: &'a mut [u8],
    u_stride: i32,
    v: &'a mut [u8],
    v_stride: i32,
) -> Option<&'a mut [u8]> {
    let mut params = WebPDecParams::default();
    let mut output = WebPDecBuffer::default();
    if !webp_init_dec_buffer(&mut output) {
        return None;
    }
    webp_reset_dec_params(&mut params);
    // C: params.output = &output.
    output.colorspace = WEBP_CSP_MODE::MODE_YUV;
    output.u = WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
        y: PlaneOffset {
            region: 0,
            offset: 0,
        },
        y_stride: luma_stride,
        y_size: luma.len(),
        u: PlaneOffset {
            region: 1,
            offset: 0,
        },
        u_stride,
        u_size: u.len(),
        v: PlaneOffset {
            region: 2,
            offset: 0,
        },
        v_stride,
        v_size: v.len(),
        ..WebPYUVABuffer::default()
    });
    // C: output.is_external_memory = 1.
    output.storage = PlaneStorage::External(vec![luma, u, v]);
    if decode_into(data, &mut params, &mut output).is_err() {
        return None;
    }
    external_region(output.storage)
}

//------------------------------------------------------------------------------

// upstream: src/dec/webp_dec.c Decode
/// Decodes `data` into a buffer in `mode` that the buffer allocates. Returns
/// the buffer, where C returns its first plane and copies the side info into
/// `keep_info` with `WebPCopyDecBuffer`; `width` and `height` are written once
/// the header has been read, even if the decoding fails afterwards, as in C.
fn decode(
    mode: WEBP_CSP_MODE,
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<WebPDecBuffer<'static>> {
    let mut params = WebPDecParams::default();
    let mut output = WebPDecBuffer::default();

    if !webp_init_dec_buffer(&mut output) {
        return None;
    }
    webp_reset_dec_params(&mut params);
    // C: params.output = &output.
    output.colorspace = mode;

    // Retrieve (and report back) the required dimensions from bitstream.
    if !webp_get_info(data, Some(&mut output.width), Some(&mut output.height)) {
        return None;
    }
    if let Some(width) = width {
        *width = output.width;
    }
    if let Some(height) = height {
        *height = output.height;
    }

    // Decode
    if decode_into(data, &mut params, &mut output).is_err() {
        return None;
    }
    // return decoded samples (don't clear 'output'!)
    Some(output)
}

/// The samples that a buffer of [`decode`] allocated: what C returns as
/// `output.u.RGBA.rgba`, the start of its private memory.
fn into_samples(buffer: WebPDecBuffer<'static>) -> Vec<u8> {
    match buffer.storage {
        PlaneStorage::Internal(memory) => memory,
        PlaneStorage::External(_) | PlaneStorage::Shared(_) => {
            panic!("Decode allocates the buffer's memory")
        }
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeRGB
/// Decodes WebP images in `data` and returns RGB samples (R, G, B, R, G, B...
/// in scan order, without padding), along with the dimensions in `width` and
/// `height`. Returns `None` in case of error.
#[must_use]
pub fn webp_decode_rgb(
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<Vec<u8>> {
    decode(WEBP_CSP_MODE::MODE_RGB, data, width, height).map(into_samples)
}

// upstream: src/dec/webp_dec.c WebPDecodeRGBA
/// [`webp_decode_rgb`] with RGBA samples (R, G, B, A, R, G, B, A...).
#[must_use]
pub fn webp_decode_rgba(
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<Vec<u8>> {
    decode(WEBP_CSP_MODE::MODE_RGBA, data, width, height).map(into_samples)
}

// upstream: src/dec/webp_dec.c WebPDecodeARGB
/// [`webp_decode_rgb`] with ARGB samples (A, R, G, B, A, R, G, B...).
#[must_use]
pub fn webp_decode_argb(
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<Vec<u8>> {
    decode(WEBP_CSP_MODE::MODE_ARGB, data, width, height).map(into_samples)
}

// upstream: src/dec/webp_dec.c WebPDecodeBGR
/// [`webp_decode_rgb`] with BGR samples (B, G, R, B, G, R...).
#[must_use]
pub fn webp_decode_bgr(
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<Vec<u8>> {
    decode(WEBP_CSP_MODE::MODE_BGR, data, width, height).map(into_samples)
}

// upstream: src/dec/webp_dec.c WebPDecodeBGRA
/// [`webp_decode_rgb`] with BGRA samples (B, G, R, A, B, G, R, A...).
#[must_use]
pub fn webp_decode_bgra(
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<Vec<u8>> {
    decode(WEBP_CSP_MODE::MODE_BGRA, data, width, height).map(into_samples)
}

// upstream: src/dec/webp_dec.c WebPDecodeYUV
/// Decodes WebP images in `data` into Y, U and V planes. Returns the buffer,
/// whose YUVA member holds the planes (the U and V planes have the same stride,
/// `(width + 1) / 2` bytes, and `(height + 1) / 2` rows), where C returns the
/// luma pointer and writes the U and V pointers and the two strides. `width`
/// and `height` are written as in [`webp_decode_rgb`]. C returns `NULL` when
/// the U, V or stride pointers are `NULL`, which exists only at the C ABI.
#[must_use]
pub fn webp_decode_yuv(
    data: &[u8],
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<WebPDecBuffer<'static>> {
    // data, width and height are checked by Decode().
    let output = decode(WEBP_CSP_MODE::MODE_YUV, data, width, height)?;
    if let WebPDecBufferPlanes::YUVA(buf) = &output.u {
        debug_assert_eq!(buf.u_stride, buf.v_stride);
    }
    Some(output)
}

// upstream: src/dec/webp_dec.c DefaultFeatures
fn default_features(features: &mut WebPBitstreamFeatures) {
    *features = WebPBitstreamFeatures::default();
}

// upstream: src/dec/webp_dec.c GetFeatures
/// C returns `VP8_STATUS_INVALID_PARAM` for `NULL` features or data, which
/// exist only at the C ABI.
fn get_features(data: &[u8], features: &mut WebPBitstreamFeatures) -> Result<(), VP8StatusCode> {
    default_features(features);

    // Only parse enough of the data to retrieve the features.
    parse_headers_internal(
        data,
        Some(&mut features.width),
        Some(&mut features.height),
        Some(&mut features.has_alpha),
        Some(&mut features.has_animation),
        Some(&mut features.format),
        None,
    )
}

//------------------------------------------------------------------------------
// WebPGetInfo()

// upstream: src/dec/webp_dec.c WebPGetInfo
/// Retrieves basic header information: width, height. Also validates the
/// header, returning true on success, false otherwise. `width` and `height`
/// are only written on success; either may be `None`. The chunk sequences that
/// `ParseHeadersInternal` accepts apply.
#[must_use]
pub fn webp_get_info(data: &[u8], width: Option<&mut i32>, height: Option<&mut i32>) -> bool {
    let mut features = WebPBitstreamFeatures::default();

    if get_features(data, &mut features).is_err() {
        return false;
    }

    if let Some(width) = width {
        *width = features.width;
    }
    if let Some(height) = height {
        *height = features.height;
    }

    true
}

//------------------------------------------------------------------------------
// Advance decoding API

// upstream: src/dec/webp_dec.c WebPInitDecoderConfigInternal
/// Internal, version-checked, entry point of `WebPInitDecoderConfig`: resets
/// the configuration. Returns false in case of version mismatch. C also
/// returns 0 for a `NULL` configuration, which exists only at the C ABI.
#[must_use]
pub fn webp_init_decoder_config_internal(config: &mut WebPDecoderConfig<'_>, version: i32) -> bool {
    if webp_abi_is_incompatible(version, WEBP_DECODER_ABI_VERSION) {
        return false; // version mismatch
    }
    *config = WebPDecoderConfig::default();
    default_features(&mut config.input);
    if !webp_init_dec_buffer(&mut config.output) {
        return false;
    }
    true
}

// upstream: src/dec/webp_dec.c WebPValidateDecoderConfig
/// Checks that `config` is non-NULL and valid: the colorspace, the crop and
/// scale options, and, if the features are filled in, the crop and scale
/// against the dimensions. C returns 0 for a `NULL` configuration, which exists
/// only at the C ABI.
#[must_use]
pub fn webp_validate_decoder_config(config: &WebPDecoderConfig<'_>) -> bool {
    if !crate::dec::buffer_dec::is_valid_colorspace(config.output.colorspace.0.cast_signed()) {
        return false;
    }

    let options = &config.options;
    // bypass_filtering, no_fancy_upsampling, use_cropping, use_scaling,
    // use_threads, flip can be any integer and are interpreted as boolean.

    // Check for cropping.
    if options.use_cropping != 0
        && !webp_check_crop_dimensions_basic(
            options.crop_left,
            options.crop_top,
            options.crop_width,
            options.crop_height,
        )
    {
        return false;
    }
    // Check for scaling.
    if options.use_scaling != 0
        && (options.scaled_width < 0
            || options.scaled_height < 0
            || (options.scaled_width == 0 && options.scaled_height == 0))
    {
        return false;
    }

    // In case the WebPBitstreamFeatures has been filled in, check further.
    if config.input.width > 0 || config.input.height > 0 {
        let mut scaled_width = options.scaled_width;
        let mut scaled_height = options.scaled_height;
        if options.use_cropping != 0
            && !webp_check_crop_dimensions(
                config.input.width,
                config.input.height,
                options.crop_left,
                options.crop_top,
                options.crop_width,
                options.crop_height,
            )
        {
            return false;
        }
        if options.use_scaling != 0
            && !webp_rescaler_get_scaled_dimensions(
                config.input.width,
                config.input.height,
                &mut scaled_width,
                &mut scaled_height,
            )
        {
            return false;
        }
    }

    // Check for dithering.
    if options.dithering_strength < 0
        || options.dithering_strength > 100
        || options.alpha_dithering_strength < 0
        || options.alpha_dithering_strength > 100
    {
        return false;
    }

    true
}

// upstream: src/dec/webp_dec.c WebPGetFeaturesInternal
/// Internal, version-checked, entry point of `WebPGetFeatures`.
///
/// # Errors
///
/// Returns `VP8_STATUS_INVALID_PARAM` in case of version mismatch, and the
/// status of the header parsing otherwise: `VP8_STATUS_NOT_ENOUGH_DATA` when
/// more data is needed to retrieve the features. C also returns
/// `VP8_STATUS_INVALID_PARAM` for `NULL` features or data, which exist only at
/// the C ABI.
pub fn webp_get_features_internal(
    data: &[u8],
    features: &mut WebPBitstreamFeatures,
    version: i32,
) -> Result<(), VP8StatusCode> {
    if webp_abi_is_incompatible(version, WEBP_DECODER_ABI_VERSION) {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM); // version mismatch
    }
    get_features(data, features)
}

// upstream: src/dec/webp_dec.c WebPDecode
/// Non-incremental version: decodes `data` into `config.output` with
/// `config.options`, after filling `config.input`. Slow external memory is
/// decoded into a temporary buffer and copied at the end. C returns
/// `VP8_STATUS_INVALID_PARAM` for a `NULL` configuration, which exists only at
/// the C ABI.
///
/// # Errors
///
/// Returns the status of the decoding; not enough data is reported as
/// `VP8_STATUS_BITSTREAM_ERROR`, as in C.
pub fn webp_decode(data: &[u8], config: &mut WebPDecoderConfig<'_>) -> Result<(), VP8StatusCode> {
    let mut params = WebPDecParams::default();

    if let Err(status) = get_features(data, &mut config.input) {
        if status == VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA {
            return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR); // Not-enough-data treated as error.
        }
        return Err(status);
    }

    webp_reset_dec_params(&mut params);
    params.options = Some(&config.options);
    // C: params.output = &config->output.
    if webp_avoid_slow_memory(&config.output, Some(&config.input)) {
        // decoding to slow memory: use a temporary in-mem buffer to decode into.
        let mut in_mem_buffer = WebPDecBuffer::default();
        if !webp_init_dec_buffer(&mut in_mem_buffer) {
            return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
        }
        in_mem_buffer.colorspace = config.output.colorspace;
        in_mem_buffer.width = config.input.width;
        in_mem_buffer.height = config.input.height;
        // C: params.output = &in_mem_buffer.
        let mut decoded = decode_into(data, &mut params, &mut in_mem_buffer);
        if decoded.is_ok() {
            // do the slow-copy
            decoded = webp_copy_dec_buffer_pixels(&in_mem_buffer, &mut config.output);
        }
        webp_free_dec_buffer(&mut in_mem_buffer);
        decoded
    } else {
        decode_into(data, &mut params, &mut config.output)
    }
}
