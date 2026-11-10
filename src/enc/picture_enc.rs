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
//!
//! # Functions that write planes in place
//!
//! A consumer that needs to write into a picture's planes must know which
//! functions write into their argument (ADR 0003, "Shared views"). A view has
//! no mutable access; each function in the first group panics when the
//! argument is a view.
//!
//! - **Write in place into the argument's planes:**
//!   - `WebPCleanupTransparentArea`: the Y, U and V planes (YUV), or the ARGB
//!     samples.
//!   - `WebPBlendAlpha`: the Y, U, V and A planes (YUV), or the ARGB samples.
//!   - `WebPReplaceTransparentPixels`: the ARGB samples.
//!   - `WebPPictureRescale`: before rescaling, multiplies the source's luma by
//!     its alpha (YUV with alpha) or the ARGB samples by their alpha. The call
//!     leaves them multiplied when it fails, as C does; on success the picture
//!     is replaced.
//! - **Write only to planes that they allocate for the argument**, which
//!   releases the caller's planes without writing them: `WebPPictureAlloc` and
//!   its allocators, `WebPPictureImport*`, `WebPPictureARGBToYUVA` and its
//!   forms, and `WebPPictureYUVAToARGB`.
//! - **Read the argument and write only new planes:** `WebPPictureCrop`,
//!   `WebPPictureCopy`, `WebPPictureView` (no write at all),
//!   `WebPPictureDistortion` (it writes only the ARGB planes of its two views),
//!   `WebPPictureHasTransparency` and `WebPPlaneDistortion`.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Errors.** `WebPEncodingSetError` returns the code that the picture
//!    holds afterwards, which is the code that a caller propagates with `Err`.
//!    The setter keeps the oldest error, as C does. Functions whose every
//!    failure path goes through the setter return
//!    `Result<(), WebPEncodingError>`: `WebPValidatePicture`, the allocators,
//!    `WebPPictureAlloc`, the ARGB-to-YUVA conversions,
//!    `WebPPictureYUVAToARGB`, `WebPPictureRescale`, `WebPPictureCopy` and
//!    `WebPReportProgress`. Functions with a failure that records no code stay
//!    `bool`, with the code left on the picture as in C: `WebPPictureImport*`
//!    (the stride test), `WebPPictureCrop` (the rectangle test),
//!    `WebPPlaneDistortion`, `WebPPictureDistortion`, `WebPMemoryWrite`, and
//!    the predicates. `WebPPictureView` returns `Option` (its failure records
//!    no code).
//! 2. **Mutable receivers.** `WebPValidatePicture`, `WebPEncodingSetError` and
//!    `WebPReportProgress` take `&mut WebPPicture`, where C takes `const` and
//!    casts it away to record the error.
//! 3. **Zero-initialization.** Allocated planes are zero-filled, as ADR 0003
//!    requires. A large successful allocation therefore touches its memory; C
//!    does not.
//! 4. **Negative strides.** A row start before the start of its slice panics. C
//!    reads memory before the pointer in that case, which is valid only when
//!    the caller's memory extends there. The safe encoder API
//!    (`src/api/encode.rs`, ADR 0006) cannot express negative strides, so the
//!    panic is reachable only through this module's own interface.
//! 5. **`WebPMemoryWriter.mem`.** Its length is `max_size`, and
//!    `WebPMemoryWrite` keeps `size` in step with it (`WebPMemoryWriter` in
//!    `src/webp/encode.rs`). `WebPMemoryWriterInit` leaves `mem` empty, which
//!    is C's `NULL`.
//!
//! The version checks of `WebPPictureInitInternal` are recorded with those of
//! `WebPConfigInitInternal` in `src/enc/config_enc.rs`,
//! `# Deviations from upstream`, item 1.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **`WEBP_REDUCE_CSP` and `WEBP_REDUCE_SIZE`** are undefined, so the BGR
//!    functions of the simple API exist, and so do `WebPPictureCopy`, `Crop`
//!    and `Rescale` in the C ABI.
//! 2. `NULL` arguments of every function of the config and picture modules
//!    (`src/enc/config_enc.rs`, `src/enc/picture_*.rs`): `picture`, `config`,
//!    the source and destination pictures, the `bgr`, `rgba` and similar
//!    buffers, and the planes of `WebPPlaneDistortion`. Rust takes references,
//!    so the checks have no counterpart; the ABI mirror (`src/abi/encode.rs`,
//!    ADR 0004) keeps them.
//! 3. Zero dimensions and zero-sized allocations. `WebPValidatePicture` rejects
//!    them before anything is allocated, which the allocation and validation
//!    tests of `tests/wp9_picture.rs` compare.
//! 4. Allocation limits at the byte level. The arithmetic of the allocation
//!    test is that of `WebPSafeMalloc` (`tests/wp1_alloc.rs`).
//!    `tests/wp9_picture.rs` compares the pictures that the limit rejects
//!    before allocating, and not successful large allocations: C leaves them
//!    uncommitted, while Rust zero-fills them (ADR 0003), which would touch the
//!    memory.
//! 5. Negative strides and negative dimensions. A Rust slice cannot address
//!    rows before its start, so a negative row start panics (item 4 of
//!    `# Deviations from upstream`). The tests do not generate them.
//!    `WebPPlaneDistortion` with a negative width returns `false` in C and Rust
//!    by the same comparison, but is not compared.
//! 6. SIMD and threads: none in the reference configuration.

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
        PlaneStorage::Shared(_) => {
            panic!("a shared view of a picture cannot be written");
        }
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

//------------------------------------------------------------------------------
// WP13: simplest high-level calls (exception in tools/units.json)

use crate::enc::picture_csp_enc::{
    webp_picture_import_bgr, webp_picture_import_bgra, webp_picture_import_rgb,
    webp_picture_import_rgba,
};
use crate::enc::webp_enc::webp_encode;
use crate::webp::encode::{WebPConfig, WebPPreset, webp_config_preset, webp_picture_init};

// upstream: src/enc/picture_enc.c Importer
/// An importer of packed samples into a picture (C: the `Importer` typedef).
type Importer = fn(&mut WebPPicture<'_>, &[u8], i32) -> bool;

// upstream: src/enc/picture_enc.c Encode
/// Imports `rgba` with `import` and encodes it with the default preset at
/// `quality_factor`, lossless when `lossless` is set, into memory (C: `Encode`).
/// Returns the encoded bytes, or `None` where C returns 0. C's `output` parameter
/// is the returned vector (its `NULL` check belongs to the C ABI), and the size C
/// returns is its length. C hands out the writer's buffer; the vector holds its
/// `size` written bytes.
fn encode(
    rgba: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    import: Importer,
    quality_factor: f32,
    lossless: bool,
) -> Option<Vec<u8>> {
    let mut wrt = WebPMemoryWriter::default();
    let mut config = WebPConfig::default();
    let ok;

    {
        let mut pic = WebPPicture::default();
        if !webp_config_preset(&mut config, WebPPreset::WEBP_PRESET_DEFAULT, quality_factor)
            || !webp_picture_init(&mut pic)
        {
            return None; // shouldn't happen, except if system installation is broken
        }

        config.lossless = i32::from(lossless);
        pic.use_argb = i32::from(lossless);
        pic.width = width;
        pic.height = height;
        // C sets the writer and custom_ptr first; the closure borrows the memory
        // writer, so it is initialized just before.
        webp_memory_writer_init(&mut wrt);
        let writer = &mut wrt;
        pic.writer = Some(Box::new(move |data: &[u8]| webp_memory_write(writer, data)));

        ok = import(&mut pic, rgba, stride) && webp_encode(&config, &mut pic);
        webp_picture_free(&mut pic);
    }
    if !ok {
        webp_memory_writer_clear(&mut wrt);
        return None; // C: *output = NULL; return 0;
    }
    let mut output = wrt.mem;
    output.truncate(wrt.size);
    Some(output)
}

// upstream: src/enc/picture_enc.c WebPEncodeRGB
/// Encodes RGB rows of `stride` bytes at `quality_factor` (C: `WebPEncodeRGB`).
/// Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_rgb(
    rgb: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    quality_factor: f32,
) -> Option<Vec<u8>> {
    encode(
        rgb,
        width,
        height,
        stride,
        webp_picture_import_rgb,
        quality_factor,
        false,
    )
}

// upstream: src/enc/picture_enc.c WebPEncodeRGBA
/// Encodes RGBA rows of `stride` bytes at `quality_factor` (C: `WebPEncodeRGBA`).
/// Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_rgba(
    rgba: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    quality_factor: f32,
) -> Option<Vec<u8>> {
    encode(
        rgba,
        width,
        height,
        stride,
        webp_picture_import_rgba,
        quality_factor,
        false,
    )
}

// upstream: src/enc/picture_enc.c WebPEncodeBGR
/// Encodes BGR rows of `stride` bytes at `quality_factor` (C: `WebPEncodeBGR`;
/// `WEBP_REDUCE_CSP` is not defined). Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_bgr(
    bgr: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    quality_factor: f32,
) -> Option<Vec<u8>> {
    encode(
        bgr,
        width,
        height,
        stride,
        webp_picture_import_bgr,
        quality_factor,
        false,
    )
}

// upstream: src/enc/picture_enc.c WebPEncodeBGRA
/// Encodes BGRA rows of `stride` bytes at `quality_factor` (C: `WebPEncodeBGRA`;
/// `WEBP_REDUCE_CSP` is not defined). Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_bgra(
    bgra: &[u8],
    width: i32,
    height: i32,
    stride: i32,
    quality_factor: f32,
) -> Option<Vec<u8>> {
    encode(
        bgra,
        width,
        height,
        stride,
        webp_picture_import_bgra,
        quality_factor,
        false,
    )
}

// upstream: src/enc/picture_enc.c LOSSLESS_DEFAULT_QUALITY
/// The quality of the lossless simple API (C: `70.`, passed as a `float`).
pub(crate) const LOSSLESS_DEFAULT_QUALITY: f32 = 70.;

// upstream: src/enc/picture_enc.c WebPEncodeLosslessRGB
/// Encodes RGB rows of `stride` bytes losslessly (C: `WebPEncodeLosslessRGB`).
/// Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_lossless_rgb(
    rgb: &[u8],
    width: i32,
    height: i32,
    stride: i32,
) -> Option<Vec<u8>> {
    encode(
        rgb,
        width,
        height,
        stride,
        webp_picture_import_rgb,
        LOSSLESS_DEFAULT_QUALITY,
        true,
    )
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessRGBA
/// Encodes RGBA rows of `stride` bytes losslessly (C: `WebPEncodeLosslessRGBA`).
/// Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_lossless_rgba(
    rgba: &[u8],
    width: i32,
    height: i32,
    stride: i32,
) -> Option<Vec<u8>> {
    encode(
        rgba,
        width,
        height,
        stride,
        webp_picture_import_rgba,
        LOSSLESS_DEFAULT_QUALITY,
        true,
    )
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessBGR
/// Encodes BGR rows of `stride` bytes losslessly (C: `WebPEncodeLosslessBGR`;
/// `WEBP_REDUCE_CSP` is not defined). Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_lossless_bgr(
    bgr: &[u8],
    width: i32,
    height: i32,
    stride: i32,
) -> Option<Vec<u8>> {
    encode(
        bgr,
        width,
        height,
        stride,
        webp_picture_import_bgr,
        LOSSLESS_DEFAULT_QUALITY,
        true,
    )
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessBGRA
/// Encodes BGRA rows of `stride` bytes losslessly (C: `WebPEncodeLosslessBGRA`;
/// `WEBP_REDUCE_CSP` is not defined). Returns the WebP data, or `None` on error.
#[must_use]
pub fn webp_encode_lossless_bgra(
    bgra: &[u8],
    width: i32,
    height: i32,
    stride: i32,
) -> Option<Vec<u8>> {
    encode(
        bgra,
        width,
        height,
        stride,
        webp_picture_import_bgra,
        LOSSLESS_DEFAULT_QUALITY,
        true,
    )
}
