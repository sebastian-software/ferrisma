// libwebp v1.6.0, src/dec/buffer_dec.c:
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
// Everything about WebPDecBuffer
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/buffer_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP12 (decoder integration).
//!
//! The samples of a buffer are one [`PlaneStorage`] (ADR 0003, buffers and
//! planes): `Internal` is C's `private_memory`, `External` the caller's memory of
//! `is_external_memory`. A C plane pointer is `NULL` here when the buffer holds
//! no memory of its own (an empty `Internal` storage is C's
//! `private_memory == NULL`) or when its region does not exist in the caller's
//! storage (the C ABI gives a `NULL` plane no region); an empty caller slice is
//! a pointer to no bytes, as in C. The
//! member of the `u` union that C reads is the one that the colorspace selects;
//! a Rust buffer whose `u` holds the other member describes no valid plane, and
//! `CheckDecBuffer` rejects it (`docs/status/wp12.md`, deviations).
//!
//! C's `is_external_memory >= 2`, slow external memory, is the buffer's
//! `slow_memory` flag (ADR 0003, amended 2026-11-07).

use crate::dec::webp_dec::webp_check_crop_dimensions;
use crate::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;
use crate::utils::utils::{webp_copy_plane, webp_safe_malloc};
use crate::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WEBP_DECODER_ABI_VERSION, WebPBitstreamFeatures, WebPDecBuffer,
    WebPDecBufferPlanes, WebPDecoderOptions, WebPRGBABuffer, WebPYUVABuffer, webp_is_alpha_mode,
    webp_is_premultiplied_mode, webp_is_rgb_mode,
};
use crate::webp::types::{PlaneOffset, PlaneStorage, webp_abi_is_incompatible};

//------------------------------------------------------------------------------
// WebPDecBuffer

// upstream: src/dec/buffer_dec.c kModeBpp
/// Number of bytes per pixel for the different color-spaces.
const MODE_BPP: [u8; WEBP_CSP_MODE::MODE_LAST.0 as usize] = [
    3, 4, 3, 4, 4, 2, 2, //
    4, 4, 4, 2, // pre-multiplied modes
    1, 1,
];

/// `kModeBpp[mode]` for a valid colorspace.
fn mode_bpp(mode: WEBP_CSP_MODE) -> u8 {
    MODE_BPP[mode.0 as usize]
}

// upstream: src/dec/buffer_dec.c IsValidColorspace
/// Converts to an integer to handle both the unsigned and the signed enum
/// cases, as C does: callers pass the colorspace as an `int` (C declares the
/// function in `src/dec/common_dec.h`).
#[must_use]
pub fn is_valid_colorspace(webp_csp_mode: i32) -> bool {
    webp_csp_mode >= WEBP_CSP_MODE::MODE_RGB.0.cast_signed()
        && webp_csp_mode < WEBP_CSP_MODE::MODE_LAST.0.cast_signed()
}

/// The colorspace as the `int` that C passes to `IsValidColorspace`.
fn colorspace_int(mode: WEBP_CSP_MODE) -> i32 {
    mode.0.cast_signed()
}

// upstream: src/dec/buffer_dec.c MIN_BUFFER_SIZE
/// Strictly speaking, the very last (or first, if flipped) row doesn't require
/// padding. C computes `(uint64_t)(STRIDE) * ((HEIGHT) - 1) + (WIDTH)`, with
/// the `int` operands converted to `uint64_t`.
fn min_buffer_size(width: u64, height: i32, stride: i32) -> u64 {
    u64::from(stride.cast_unsigned())
        .wrapping_mul(i64::from(height - 1).cast_unsigned())
        .wrapping_add(width)
}

/// An `int` widened to `uint64_t`, as the C conversion does.
fn to_u64(value: i32) -> u64 {
    i64::from(value).cast_unsigned()
}

/// Whether the plane is C's `NULL` pointer (see the module documentation).
#[must_use]
pub fn is_null_plane(storage: &PlaneStorage<'_, u8>, plane: PlaneOffset) -> bool {
    match storage {
        PlaneStorage::Internal(memory) => memory.is_empty(),
        PlaneStorage::External(_) | PlaneStorage::Shared(_) => {
            storage.region(plane.region).is_none()
        }
    }
}

// upstream: src/dec/buffer_dec.c CheckDecBuffer
fn check_dec_buffer(buffer: &WebPDecBuffer<'_>) -> Result<(), VP8StatusCode> {
    let mut ok = true;
    let mode = buffer.colorspace;
    let width = buffer.width;
    let height = buffer.height;
    if !is_valid_colorspace(colorspace_int(mode)) {
        ok = false;
    } else if !webp_is_rgb_mode(mode) {
        // YUV checks
        if let WebPDecBufferPlanes::YUVA(buf) = &buffer.u {
            let uv_width = (width + 1) / 2;
            let uv_height = (height + 1) / 2;
            let y_stride = buf.y_stride.abs();
            let u_stride = buf.u_stride.abs();
            let v_stride = buf.v_stride.abs();
            let a_stride = buf.a_stride.abs();
            let y_size = min_buffer_size(to_u64(width), height, y_stride);
            let u_size = min_buffer_size(to_u64(uv_width), uv_height, u_stride);
            let v_size = min_buffer_size(to_u64(uv_width), uv_height, v_stride);
            let a_size = min_buffer_size(to_u64(width), height, a_stride);
            ok &= y_size <= buf.y_size as u64;
            ok &= u_size <= buf.u_size as u64;
            ok &= v_size <= buf.v_size as u64;
            ok &= y_stride >= width;
            ok &= u_stride >= uv_width;
            ok &= v_stride >= uv_width;
            ok &= !is_null_plane(&buffer.storage, buf.y);
            ok &= !is_null_plane(&buffer.storage, buf.u);
            ok &= !is_null_plane(&buffer.storage, buf.v);
            if mode == WEBP_CSP_MODE::MODE_YUVA {
                ok &= a_stride >= width;
                ok &= a_size <= buf.a_size as u64;
                ok &= buf.a.is_some_and(|a| !is_null_plane(&buffer.storage, a));
            }
        } else {
            ok = false; // the RGBA member describes no YUVA plane
        }
    } else {
        // RGB checks
        if let WebPDecBufferPlanes::RGBA(buf) = &buffer.u {
            let bpp = mode_bpp(mode);
            let stride = buf.stride.abs();
            let size = min_buffer_size(to_u64(width).wrapping_mul(u64::from(bpp)), height, stride);
            ok &= size <= buf.size as u64;
            ok &= stride >= width * i32::from(bpp);
            ok &= !is_null_plane(&buffer.storage, buf.rgba);
        } else {
            ok = false; // the YUVA member describes no RGBA plane
        }
    }
    if ok {
        Ok(())
    } else {
        Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM)
    }
}

// upstream: src/dec/buffer_dec.c AllocateBuffer
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the uint64_t sizes to size_t, which holds them after WebPSafeMalloc"
)]
fn allocate_buffer(buffer: &mut WebPDecBuffer<'_>) -> Result<(), VP8StatusCode> {
    let w = buffer.width;
    let h = buffer.height;
    let mode = buffer.colorspace;

    if w <= 0 || h <= 0 || !is_valid_colorspace(colorspace_int(mode)) {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }

    if matches!(&buffer.storage, PlaneStorage::Internal(memory) if memory.is_empty()) {
        let mut uv_stride = 0;
        let mut a_stride = 0;
        let mut uv_size: u64 = 0;
        let mut a_size: u64 = 0;
        // We need memory and it hasn't been allocated yet.
        // => initialize output buffer, now that dimensions are known.
        let bpp = mode_bpp(mode);
        if to_u64(w) * u64::from(bpp) >= (1u64 << 31) {
            return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
        }
        let stride = w * i32::from(bpp);
        let size = to_u64(stride) * to_u64(h);
        if !webp_is_rgb_mode(mode) {
            uv_stride = (w + 1) / 2;
            uv_size = to_u64(uv_stride) * to_u64((h + 1) / 2);
            if mode == WEBP_CSP_MODE::MODE_YUVA {
                a_stride = w;
                a_size = to_u64(a_stride) * to_u64(h);
            }
        }
        let total_size = size + 2 * uv_size + a_size;

        let Some(mut output) = webp_safe_malloc::<u8>(total_size, 1) else {
            return Err(VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
        };
        // Rust initializes the allocation (ADR 0003, allocation limits).
        output.resize(total_size as usize, 0);
        buffer.storage = PlaneStorage::Internal(output);

        let plane = |offset: u64| PlaneOffset {
            region: 0,
            offset: offset as usize,
        };
        if webp_is_rgb_mode(mode) {
            // RGBA initialization
            buffer.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
                rgba: plane(0),
                stride,
                size: size as usize,
            });
        } else {
            // YUVA initialization. C leaves `a` alone for MODE_YUV; the RGBA
            // member that the union held before has no `a`, which reads as NULL.
            let a = match &buffer.u {
                WebPDecBufferPlanes::YUVA(buf) => buf.a,
                WebPDecBufferPlanes::RGBA(_) => None,
            };
            buffer.u = WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
                y: plane(0),
                y_stride: stride,
                y_size: size as usize,
                u: plane(size),
                u_stride: uv_stride,
                u_size: uv_size as usize,
                v: plane(size + uv_size),
                v_stride: uv_stride,
                v_size: uv_size as usize,
                a: if mode == WEBP_CSP_MODE::MODE_YUVA {
                    Some(plane(size + 2 * uv_size))
                } else {
                    a
                },
                a_size: a_size as usize,
                a_stride,
            });
        }
    }
    check_dec_buffer(buffer)
}

/// `pointer += delta` for a plane, as C moves a plane pointer by a signed
/// number of bytes.
///
/// # Panics
///
/// Panics when the plane would start before its region, where the C pointer
/// would leave the buffer.
fn move_plane(plane: &mut PlaneOffset, delta: i64) {
    plane.offset = plane
        .offset
        .checked_add_signed(isize::try_from(delta).expect("the plane offset fits"))
        .expect("the flipped plane starts inside its region");
}

// upstream: src/dec/buffer_dec.c WebPFlipBuffer
/// Flips the buffer vertically by negating the various strides. C returns
/// `VP8_STATUS_INVALID_PARAM` for a `NULL` buffer, which exists only at the C
/// ABI.
///
/// # Errors
///
/// None: the result is always `Ok`, as C returns `VP8_STATUS_OK` for every
/// buffer.
///
/// # Panics
///
/// Panics when a plane would start before its region, where C would compute a
/// pointer outside the buffer.
pub fn webp_flip_buffer(buffer: &mut WebPDecBuffer<'_>) -> Result<(), VP8StatusCode> {
    let height = i64::from(buffer.height);
    if webp_is_rgb_mode(buffer.colorspace) {
        if let WebPDecBufferPlanes::RGBA(buf) = &mut buffer.u {
            move_plane(&mut buf.rgba, (height - 1) * i64::from(buf.stride));
            buf.stride = -buf.stride;
        }
    } else if let WebPDecBufferPlanes::YUVA(buf) = &mut buffer.u {
        let h = height;
        move_plane(&mut buf.y, (h - 1) * i64::from(buf.y_stride));
        buf.y_stride = -buf.y_stride;
        move_plane(&mut buf.u, ((h - 1) >> 1) * i64::from(buf.u_stride));
        buf.u_stride = -buf.u_stride;
        move_plane(&mut buf.v, ((h - 1) >> 1) * i64::from(buf.v_stride));
        buf.v_stride = -buf.v_stride;
        if let Some(a) = &mut buf.a {
            move_plane(a, (h - 1) * i64::from(buf.a_stride));
            buf.a_stride = -buf.a_stride;
        }
    }
    Ok(())
}

// upstream: src/dec/buffer_dec.c WebPAllocateDecBuffer
/// Prepares `buffer` with the requested initial dimensions `width` and
/// `height`. If no external storage is supplied, initializes the buffer by
/// allocating output memory and setting up the stride information. Validates
/// the parameters. If `options` is not `None`, also verifies that the options'
/// parameters are valid and applies them to the width and height of the output
/// buffer: cropping, scaling and the vertical flip. C returns
/// `VP8_STATUS_INVALID_PARAM` for a `NULL` buffer, which exists only at the C
/// ABI.
///
/// # Errors
///
/// Returns `VP8_STATUS_INVALID_PARAM` for invalid dimensions, options, strides
/// or sizes and `VP8_STATUS_OUT_OF_MEMORY` when the allocation fails. The width
/// and height are written before the buffer is checked, as in C.
pub fn webp_allocate_dec_buffer(
    width: i32,
    height: i32,
    options: Option<&WebPDecoderOptions>,
    buffer: &mut WebPDecBuffer<'_>,
) -> Result<(), VP8StatusCode> {
    let mut width = width;
    let mut height = height;
    if width <= 0 || height <= 0 {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }
    if let Some(options) = options {
        // First, apply options if there is any.
        if options.use_cropping != 0 {
            let cw = options.crop_width;
            let ch = options.crop_height;
            let x = options.crop_left & !1;
            let y = options.crop_top & !1;
            if !webp_check_crop_dimensions(width, height, x, y, cw, ch) {
                return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM); // out of frame boundary.
            }
            width = cw;
            height = ch;
        }

        if options.use_scaling != 0 {
            // WEBP_REDUCE_SIZE is not defined in the reference configuration.
            let mut scaled_width = options.scaled_width;
            let mut scaled_height = options.scaled_height;
            if !webp_rescaler_get_scaled_dimensions(
                width,
                height,
                &mut scaled_width,
                &mut scaled_height,
            ) {
                return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
            }
            width = scaled_width;
            height = scaled_height;
        }
    }
    buffer.width = width;
    buffer.height = height;

    // Then, allocate buffer for real.
    allocate_buffer(buffer)?;

    // Use the stride trick if vertical flip is needed.
    if options.is_some_and(|options| options.flip != 0) {
        webp_flip_buffer(buffer)?;
    }
    Ok(())
}

//------------------------------------------------------------------------------
// constructors / destructors

// upstream: src/dec/buffer_dec.c WebPInitDecBufferInternal
/// Internal, version-checked, entry point of `WebPInitDecBuffer`: resets the
/// buffer to empty unless the ABI versions are incompatible. Returns false in
/// case of version mismatch. C also returns 0 for a `NULL` buffer, which exists
/// only at the C ABI. C zeroes the structure without freeing it; here the old
/// storage is dropped.
#[must_use]
pub fn webp_init_dec_buffer_internal(buffer: &mut WebPDecBuffer<'_>, version: i32) -> bool {
    if webp_abi_is_incompatible(version, WEBP_DECODER_ABI_VERSION) {
        return false; // version mismatch
    }
    *buffer = WebPDecBuffer::default();
    true
}

// upstream: src/dec/buffer_dec.c WebPFreeDecBuffer
/// Frees any memory associated with the buffer: the memory that the buffer
/// allocated itself. External memory stays the caller's, and the buffer keeps
/// describing it, as C keeps the plane pointers. C accepts a `NULL` buffer,
/// which exists only at the C ABI.
pub fn webp_free_dec_buffer(buffer: &mut WebPDecBuffer<'_>) {
    if let PlaneStorage::Internal(memory) = &mut buffer.storage {
        *memory = Vec::new();
    }
}

// upstream: src/dec/buffer_dec.c WebPCopyDecBuffer
/// Copies `src` into `dst`, making sure `dst` does not own the memory, which
/// `src` still holds. No pixels are copied: `dst` gets a shared view of every
/// region of `src`, with the same plane offsets, where C copies the pointers
/// and marks `dst` as external. C accepts `NULL` buffers, which exist only at
/// the C ABI.
pub fn webp_copy_dec_buffer<'s>(src: &'s WebPDecBuffer<'_>, dst: &mut WebPDecBuffer<'s>) {
    dst.colorspace = src.colorspace;
    dst.width = src.width;
    dst.height = src.height;
    // C copies `is_external_memory`; the view is not external storage, so the
    // flag that only external storage carries is false.
    dst.slow_memory = false;
    dst.u = src.u;
    dst.pad = src.pad;
    dst.storage = PlaneStorage::Shared(match &src.storage {
        PlaneStorage::Internal(memory) => vec![memory.as_slice()],
        PlaneStorage::External(slices) => slices.iter().map(|slice| &**slice).collect(),
        PlaneStorage::Shared(slices) => slices.clone(),
    });
}

// upstream: src/dec/buffer_dec.c WebPGrabDecBuffer
/// Copies and transfers ownership from `src` to `dst` (beware of the parameter
/// order). `dst` receives the storage of `src`. In C, `src` keeps pointers to
/// the memory, now marked external; safe Rust cannot keep a second view of
/// memory that `dst` owns or borrows mutably, so `src` keeps its plane
/// descriptions with an empty storage of the same kind (`docs/status/wp12.md`,
/// deviations). C accepts `NULL` buffers, which exist only at the C ABI.
pub fn webp_grab_dec_buffer<'a>(src: &mut WebPDecBuffer<'a>, dst: &mut WebPDecBuffer<'a>) {
    dst.colorspace = src.colorspace;
    dst.width = src.width;
    dst.height = src.height;
    dst.slow_memory = src.slow_memory;
    dst.u = src.u;
    dst.pad = src.pad;
    let empty = match &src.storage {
        PlaneStorage::Internal(_) => PlaneStorage::Internal(Vec::new()),
        PlaneStorage::External(_) => PlaneStorage::External(Vec::new()),
        PlaneStorage::Shared(_) => PlaneStorage::Shared(Vec::new()),
    };
    dst.storage = std::mem::replace(&mut src.storage, empty);
}

/// The samples of `plane` for reading.
///
/// # Panics
///
/// Panics when the region does not exist.
fn region<'b>(storage: &'b PlaneStorage<'_, u8>, plane: PlaneOffset) -> &'b [u8] {
    storage
        .region(plane.region)
        .expect("the plane's region exists")
}

/// The samples of `plane` for writing.
///
/// # Panics
///
/// Panics when the region does not exist or is shared.
fn region_mut<'b>(storage: &'b mut PlaneStorage<'_, u8>, plane: PlaneOffset) -> &'b mut [u8] {
    storage
        .region_mut(plane.region)
        .expect("the plane's region exists")
}

/// One plane copy of `WebPCopyDecBufferPixels`.
#[expect(
    clippy::too_many_arguments,
    reason = "the arguments of WebPCopyPlane with the two storages"
)]
fn copy_plane(
    src: &PlaneStorage<'_, u8>,
    src_plane: PlaneOffset,
    src_stride: i32,
    dst: &mut PlaneStorage<'_, u8>,
    dst_plane: PlaneOffset,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    webp_copy_plane(
        region(src, src_plane),
        src_plane.offset,
        src_stride,
        region_mut(dst, dst_plane),
        dst_plane.offset,
        dst_stride,
        width,
        height,
    );
}

// upstream: src/dec/buffer_dec.c WebPCopyDecBufferPixels
/// Copies the pixels of `src_buf` into the preallocated `dst_buf`.
///
/// # Errors
///
/// Returns `VP8_STATUS_INVALID_PARAM` if `dst_buf` is not set up correctly for
/// the copy; its width and height are set to those of `src_buf` first, as in C.
///
/// # Panics
///
/// Debug builds panic when the colorspaces differ, as C asserts. A plane of
/// `src_buf` outside its region panics, where C would read outside the buffer.
pub fn webp_copy_dec_buffer_pixels(
    src_buf: &WebPDecBuffer<'_>,
    dst_buf: &mut WebPDecBuffer<'_>,
) -> Result<(), VP8StatusCode> {
    debug_assert_eq!(src_buf.colorspace, dst_buf.colorspace);

    dst_buf.width = src_buf.width;
    dst_buf.height = src_buf.height;
    if check_dec_buffer(dst_buf).is_err() {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }
    match (&src_buf.u, &dst_buf.u) {
        (WebPDecBufferPlanes::RGBA(src), WebPDecBufferPlanes::RGBA(dst))
            if webp_is_rgb_mode(src_buf.colorspace) =>
        {
            let (src, dst) = (*src, *dst);
            copy_plane(
                &src_buf.storage,
                src.rgba,
                src.stride,
                &mut dst_buf.storage,
                dst.rgba,
                dst.stride,
                src_buf.width * i32::from(mode_bpp(src_buf.colorspace)),
                src_buf.height,
            );
        }
        (WebPDecBufferPlanes::YUVA(src), WebPDecBufferPlanes::YUVA(dst))
            if !webp_is_rgb_mode(src_buf.colorspace) =>
        {
            let (src, dst) = (*src, *dst);
            copy_plane(
                &src_buf.storage,
                src.y,
                src.y_stride,
                &mut dst_buf.storage,
                dst.y,
                dst.y_stride,
                src_buf.width,
                src_buf.height,
            );
            copy_plane(
                &src_buf.storage,
                src.u,
                src.u_stride,
                &mut dst_buf.storage,
                dst.u,
                dst.u_stride,
                (src_buf.width + 1) / 2,
                (src_buf.height + 1) / 2,
            );
            copy_plane(
                &src_buf.storage,
                src.v,
                src.v_stride,
                &mut dst_buf.storage,
                dst.v,
                dst.v_stride,
                (src_buf.width + 1) / 2,
                (src_buf.height + 1) / 2,
            );
            if webp_is_alpha_mode(src_buf.colorspace) {
                copy_plane(
                    &src_buf.storage,
                    src.a.expect("a YUVA buffer has an alpha plane"),
                    src.a_stride,
                    &mut dst_buf.storage,
                    dst.a.expect("the checked YUVA buffer has an alpha plane"),
                    dst.a_stride,
                    src_buf.width,
                    src_buf.height,
                );
            }
        }
        _ => panic!("the source buffer's member matches its colorspace"),
    }
    Ok(())
}

// upstream: src/dec/buffer_dec.c WebPAvoidSlowMemory
/// Returns true if decoding will be slow with the current configuration and
/// bitstream features: slow external memory, a premultiplied colorspace and an
/// image with alpha.
#[must_use]
pub fn webp_avoid_slow_memory(
    output: &WebPDecBuffer<'_>,
    features: Option<&WebPBitstreamFeatures>,
) -> bool {
    // C: output->is_external_memory >= 2.
    output.slow_memory
        && webp_is_premultiplied_mode(output.colorspace)
        && features.is_some_and(|features| features.has_alpha != 0)
}
