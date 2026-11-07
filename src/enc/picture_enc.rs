// libwebp v1.6.0, src/enc/picture_enc.c:
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
// WebPPicture class basis
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/picture_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! Exception: WP13 translates `Encode`, `WebPEncodeRGB`, `WebPEncodeBGR`, `WebPEncodeRGBA`, `WebPEncodeBGRA`, `WebPEncodeLosslessRGB`, `WebPEncodeLosslessBGR`, `WebPEncodeLosslessRGBA` and `WebPEncodeLosslessBGRA` from `src/enc/picture_enc.c`, because the simple encoding API calls `WebPEncode`.
//!
//! The plane helpers at the end of this file have no C counterpart. They give the
//! picture functions of this package mutable access to several planes of one
//! storage at once, which C does with plain pointers (ADR 0003).

use crate::enc::webp_enc::webp_encoding_set_error;
use crate::utils::utils::{WEBP_ALIGN_CST, webp_safe_malloc};
use crate::webp::encode::{
    WEBP_ENCODER_ABI_VERSION, WebPEncCSP, WebPEncodingError, WebPMemoryWriter, WebPPicture,
};
use crate::webp::types::{PlaneOffset, PlaneStorage, webp_abi_is_incompatible};

// upstream: src/enc/picture_enc.c DummyWriter
/// The writer that discards its data and reports success. C also takes the
/// picture, which no caller reads (ADR 0003).
fn dummy_writer(_data: &[u8]) -> bool {
    true
}

// upstream: src/enc/picture_enc.c WebPPictureInitInternal
/// Clears `picture`, installs the discarding writer and records no error (C:
/// `WebPPictureInitInternal`). A version mismatch returns `false` and leaves the
/// picture untouched.
pub fn webp_picture_init_internal(picture: &mut WebPPicture<'_>, version: i32) -> bool {
    if webp_abi_is_incompatible(version, WEBP_ENCODER_ABI_VERSION) {
        return false; // caller/system version mismatch!
    }
    *picture = WebPPicture::default();
    picture.writer = Some(Box::new(dummy_writer));
    let _ = webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_OK);
    true
}

// upstream: src/enc/picture_enc.c WebPValidatePicture
/// Checks the dimensions and the colorspace of `picture` (C:
/// `WebPValidatePicture`). A failure records the error code on the picture.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_validate_picture(picture: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    if picture.width <= 0 || picture.height <= 0 {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION,
        ));
    }
    if picture.width <= 0
        || picture.width / 4 > i32::MAX / 4
        || picture.height <= 0
        || picture.height / 4 > i32::MAX / 4
    {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION,
        ));
    }
    if picture.colorspace != WebPEncCSP::WEBP_YUV420
        && picture.colorspace != WebPEncCSP::WEBP_YUV420A
    {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION,
        ));
    }
    Ok(())
}

// upstream: src/enc/picture_enc.c WebPPictureResetBufferARGB
/// Forgets the ARGB plane. The storage is dropped, which frees memory that the
/// picture allocated and releases the caller's slices without freeing them.
fn webp_picture_reset_buffer_argb(picture: &mut WebPPicture<'_>) {
    picture.argb_storage = PlaneStorage::default();
    picture.argb = None;
    picture.argb_stride = 0;
}

// upstream: src/enc/picture_enc.c WebPPictureResetBufferYUVA
/// Forgets the Y, U, V and A planes, as `webp_picture_reset_buffer_argb` does for
/// the ARGB plane.
fn webp_picture_reset_buffer_yuva(picture: &mut WebPPicture<'_>) {
    picture.yuva = PlaneStorage::default();
    picture.y = None;
    picture.u = None;
    picture.v = None;
    picture.a = None;
    picture.y_stride = 0;
    picture.uv_stride = 0;
    picture.a_stride = 0;
}

// upstream: src/enc/picture_enc.c WebPPictureResetBuffers
/// Forgets every plane of `picture` (C: `WebPPictureResetBuffers`).
pub fn webp_picture_reset_buffers(picture: &mut WebPPicture<'_>) {
    webp_picture_reset_buffer_argb(picture);
    webp_picture_reset_buffer_yuva(picture);
}

// upstream: src/enc/picture_enc.c WebPPictureAllocARGB
/// Allocates a zeroed ARGB plane of `width * height` samples, with the stride equal to
/// the width (C: `WebPPictureAllocARGB`). The allocation test counts the
/// `WEBP_ALIGN_CST` padding that C adds for alignment, so the same sizes fail. The
/// plane starts at offset 0: C's aligned address is not observable here.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_alloc_argb(picture: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    let width = picture.width;
    let height = picture.height;
    let argb_size = u64::try_from(width).unwrap_or(0) * u64::try_from(height).unwrap_or(0);

    webp_validate_picture(picture)?;

    // Drops the previous plane, as WebPSafeFree does, and forgets its pointers.
    webp_picture_reset_buffer_argb(picture);

    // allocate a new buffer.
    let Some(mut samples) = webp_safe_malloc::<u32>(
        argb_size + u64::try_from(WEBP_ALIGN_CST).unwrap_or(0),
        std::mem::size_of::<u32>(),
    ) else {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    };
    // Rust initializes the samples; the reserved capacity covers the padding.
    samples.resize(usize::try_from(argb_size).unwrap_or(0), 0);
    picture.argb_storage = PlaneStorage::Internal(samples);
    picture.argb = Some(PlaneOffset {
        region: 0,
        offset: 0,
    });
    picture.argb_stride = width;
    Ok(())
}

// upstream: src/enc/picture_enc.c WebPPictureAllocYUVA
/// Allocates the Y, U, V and A planes in one block of samples, in that order, with
/// the strides of C (C: `WebPPictureAllocYUVA`). The A plane exists only when the
/// colorspace has alpha.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_alloc_yuva(picture: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    let has_alpha = (picture.colorspace.0 & WebPEncCSP::WEBP_CSP_ALPHA_BIT.0) != 0;
    let width = picture.width;
    let height = picture.height;
    let y_stride = width;
    let uv_width = i32::try_from((i64::from(width) + 1) >> 1).unwrap_or(0);
    let uv_height = i32::try_from((i64::from(height) + 1) >> 1).unwrap_or(0);
    let uv_stride = uv_width;

    webp_validate_picture(picture)?;

    // Drops the previous planes, as WebPSafeFree does, and forgets their pointers.
    webp_picture_reset_buffer_yuva(picture);

    // alpha
    let a_width = if has_alpha { width } else { 0 };
    let a_stride = a_width;
    let y_size = u64::try_from(y_stride).unwrap_or(0) * u64::try_from(height).unwrap_or(0);
    let uv_size = u64::try_from(uv_stride).unwrap_or(0) * u64::try_from(uv_height).unwrap_or(0);
    let a_size = u64::try_from(a_stride).unwrap_or(0) * u64::try_from(height).unwrap_or(0);

    let total_size = y_size + a_size + 2 * uv_size;

    // Security and validation checks
    if width <= 0 || height <= 0 || uv_width <= 0 || uv_height <= 0 {
        // luma/alpha param error, or u/v param error
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION,
        ));
    }
    // allocate a new buffer.
    let Some(mut mem) = webp_safe_malloc::<u8>(total_size, std::mem::size_of::<u8>()) else {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    };
    // Rust initializes the samples.
    mem.resize(usize::try_from(total_size).unwrap_or(0), 0);

    // From now on, we're in the clear, we can no longer fail...
    picture.y_stride = y_stride;
    picture.uv_stride = uv_stride;
    picture.a_stride = a_stride;

    // The planes follow each other in the one block, as C lays them out.
    let y_start = 0;
    let u_start = usize::try_from(y_size).unwrap_or(0);
    let v_start = usize::try_from(y_size + uv_size).unwrap_or(0);
    let a_start = usize::try_from(y_size + 2 * uv_size).unwrap_or(0);
    picture.y = Some(PlaneOffset {
        region: 0,
        offset: y_start,
    });
    picture.u = Some(PlaneOffset {
        region: 0,
        offset: u_start,
    });
    picture.v = Some(PlaneOffset {
        region: 0,
        offset: v_start,
    });
    if a_size > 0 {
        picture.a = Some(PlaneOffset {
            region: 0,
            offset: a_start,
        });
    }
    picture.yuva = PlaneStorage::Internal(mem);
    Ok(())
}

// upstream: src/enc/picture_enc.c WebPPictureAlloc
/// Frees the planes of `picture`, then allocates them in the layout that
/// `use_argb` selects (C: `WebPPictureAlloc`).
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_alloc(picture: &mut WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    webp_picture_free(picture); // erase previous buffer

    if picture.use_argb == 0 {
        webp_picture_alloc_yuva(picture)
    } else {
        webp_picture_alloc_argb(picture)
    }
}

// upstream: src/enc/picture_enc.c WebPPictureFree
/// Frees the planes that `picture` allocated and forgets every plane (C:
/// `WebPPictureFree`). Caller-owned slices are only released, never freed.
pub fn webp_picture_free(picture: &mut WebPPicture<'_>) {
    webp_picture_reset_buffers(picture);
}

//------------------------------------------------------------------------------
// WebPMemoryWriter: Write-to-memory

// upstream: src/enc/picture_enc.c WebPMemoryWriterInit
/// Empties the writer (C: `WebPMemoryWriterInit`).
pub fn webp_memory_writer_init(writer: &mut WebPMemoryWriter) {
    writer.mem = Vec::new();
    writer.size = 0;
    writer.max_size = 0;
}

// upstream: src/enc/picture_enc.c WebPMemoryWrite
/// Appends `data` to the writer, growing the buffer by doubling it, with a minimum
/// of 8192 bytes (C: `WebPMemoryWrite`). The buffer keeps `max_size` bytes, of which
/// the first `size` are written. A failed allocation returns `false`. The closure
/// that the picture calls owns the writer: `move |data| webp_memory_write(&mut w, data)`.
pub fn webp_memory_write(w: &mut WebPMemoryWriter, data: &[u8]) -> bool {
    let data_size = data.len();
    let next_size = u64::try_from(w.size).unwrap_or(0) + u64::try_from(data_size).unwrap_or(0);
    if next_size > u64::try_from(w.max_size).unwrap_or(0) {
        let mut next_max_size = 2 * u64::try_from(w.max_size).unwrap_or(0);
        if next_max_size < next_size {
            next_max_size = next_size;
        }
        if next_max_size < 8192 {
            next_max_size = 8192;
        }
        let Some(mut new_mem) = webp_safe_malloc::<u8>(next_max_size, std::mem::size_of::<u8>())
        else {
            return false;
        };
        let next_len = usize::try_from(next_max_size).unwrap_or(0);
        // Rust initializes the new buffer, then copies the written bytes into it.
        new_mem.resize(next_len, 0);
        if w.size > 0 {
            new_mem[..w.size].copy_from_slice(&w.mem[..w.size]);
        }
        // the old buffer is dropped, as WebPSafeFree(w->mem) does
        w.mem = new_mem;
        w.max_size = next_len;
    }
    if data_size > 0 {
        w.mem[w.size..w.size + data_size].copy_from_slice(data);
        w.size += data_size;
    }
    true
}

// upstream: src/enc/picture_enc.c WebPMemoryWriterClear
/// Frees the buffer and empties the writer (C: `WebPMemoryWriterClear`).
pub fn webp_memory_writer_clear(writer: &mut WebPMemoryWriter) {
    webp_memory_writer_init(writer);
}

//------------------------------------------------------------------------------
// Plane access (Rust-only helpers, ADR 0003)

/// Byte offset `base + delta` in the same buffer. A position before the start of
/// the buffer cannot be addressed; C reads memory outside the buffer there, so the
/// call is a precondition violation and panics.
pub(crate) fn offset_by(base: usize, delta: i64) -> usize {
    let base = i64::try_from(base).expect("the position fits in i64");
    usize::try_from(base + delta).expect("the position lies inside the buffer")
}

/// The samples of a Y, U, V or A plane, from the plane's first sample to the end of
/// its region.
pub(crate) fn yuva_samples<'a>(storage: &'a PlaneStorage<'_, u8>, plane: PlaneOffset) -> &'a [u8] {
    let region = storage
        .region(plane.region)
        .expect("the plane lies in a region of its storage");
    &region[plane.offset..]
}

/// The mutable samples of up to four planes of one YUVA storage, given at the same
/// time: the Y, U, V and A planes in that order. Planes in one region (an owned
/// buffer holds all of them) are split at the start of the next plane in that
/// region; each caller slice is its own region. A plane that is `None` yields `None`.
pub(crate) fn yuva_planes_mut<'a>(
    storage: &'a mut PlaneStorage<'_, u8>,
    planes: [Option<PlaneOffset>; 4],
) -> [Option<&'a mut [u8]>; 4] {
    let mut out: [Option<&'a mut [u8]>; 4] = [None, None, None, None];
    match storage {
        PlaneStorage::External(slices) => {
            let mut regions: Vec<Option<&'a mut [u8]>> =
                slices.iter_mut().map(|slice| Some(&mut **slice)).collect();
            for (index, plane) in planes.iter().enumerate() {
                if let Some(plane) = plane {
                    let region = regions
                        .get_mut(plane.region)
                        .and_then(Option::take)
                        .expect("each plane of an external storage has its own region");
                    out[index] = Some(&mut region[plane.offset..]);
                }
            }
        }
        PlaneStorage::Internal(samples) => {
            let len = samples.len();
            let mut order: Vec<(usize, usize)> = planes
                .iter()
                .enumerate()
                .filter_map(|(index, plane)| {
                    plane.map(|plane| {
                        assert_eq!(plane.region, 0, "an owned storage has one region");
                        (plane.offset, index)
                    })
                })
                .collect();
            order.sort_unstable();
            let mut rest: &mut [u8] = samples.as_mut_slice();
            let mut base = 0_usize;
            for (position, &(start, index)) in order.iter().enumerate() {
                let end = order.get(position + 1).map_or(len, |next| next.0);
                let (_, tail) = std::mem::take(&mut rest).split_at_mut(start - base);
                let (plane, after) = tail.split_at_mut(end - start);
                out[index] = Some(plane);
                rest = after;
                base = end;
            }
        }
    }
    out
}

/// The samples of an ARGB plane, from the plane's first sample to the end of its region.
pub(crate) fn argb_samples<'a>(
    storage: &'a PlaneStorage<'_, u32>,
    plane: PlaneOffset,
) -> &'a [u32] {
    let region = storage
        .region(plane.region)
        .expect("the plane lies in a region of its storage");
    &region[plane.offset..]
}

/// The mutable samples of an ARGB plane, from the plane's first sample to the end of
/// its region.
pub(crate) fn argb_samples_mut<'a>(
    storage: &'a mut PlaneStorage<'_, u32>,
    plane: PlaneOffset,
) -> &'a mut [u32] {
    let region = storage
        .region_mut(plane.region)
        .expect("the plane lies in a region of its storage");
    &mut region[plane.offset..]
}

/// The bytes of ARGB samples in native byte order, which is how a C `uint8_t*` reads
/// a `uint32_t*` buffer. Channel `i` of a pixel is at byte `CHANNEL_OFFSET(i)`.
pub(crate) fn argb_bytes(samples: &[u32]) -> Vec<u8> {
    samples
        .iter()
        .flat_map(|value| value.to_ne_bytes())
        .collect()
}
