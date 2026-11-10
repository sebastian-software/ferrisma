// libwebp v1.6.0, src/enc/picture_rescale_enc.c:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// WebPPicture tools: copy, crop, rescaling and view.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/picture_rescale_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! `WebPPictureView` shares the planes of its source through the `Shared` storage
//! of ADR 0003, amended 2026-11-07. `WebPPictureCopy` copies the pixels into new
//! planes and gives the copy no callbacks, `extra_info` or `stats` (see the
//! `# Deviations from upstream` section below, item 1). `WebPPictureRescale` and
//! `WebPPictureCrop` replace their picture, so they move the callbacks instead.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Callbacks of a copy and of a view.** `WebPPictureCopy` and
//!    `WebPPictureView` set `writer`, `progress_hook`, `extra_info` and `stats`
//!    to `None`. C copies these pointers, so source and copy share them. In
//!    Rust a boxed closure cannot be shared without shared ownership, and
//!    changing the writer and progress hook types for every user is not
//!    accepted. The internal callers (the animation encoder in
//!    `src/mux/anim_encode.rs`) set their own writer on the pictures they
//!    encode. The copy otherwise matches C: the specifications, the error code,
//!    new planes with the samples of the source, and the four `None` fields.
//!    The oracle test (`tests/wp9_view.rs`) records the C sharing as its
//!    expected output.
//!
//! The rescaling of ARGB pictures reads and writes native-order byte copies of
//! the samples, where C casts them: see `src/enc/picture_csp_enc.rs`,
//! `# Deviations from upstream`, item 1.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. `NULL` planes of a view's source. C computes pointers from a `NULL`
//!    plane, which is a precondition violation; Rust keeps the plane `None` in
//!    the view.

use crate::dsp::alpha_processing::{webp_mult_argb_rows, webp_mult_rows};
use crate::enc::picture_enc::{
    argb_bytes, argb_samples, argb_samples_mut, offset_by, webp_picture_alloc, webp_picture_free,
    yuva_planes_mut, yuva_samples,
};
use crate::enc::webp_enc::webp_encoding_set_error;
use crate::utils::rescaler_utils::{
    rescaler_t, webp_rescaler_export, webp_rescaler_get_scaled_dimensions, webp_rescaler_import,
    webp_rescaler_init,
};
use crate::utils::utils::{webp_copy_plane, webp_safe_malloc};
use crate::webp::encode::{WebPEncodingError, WebPPicture};
use crate::webp::types::{PlaneOffset, PlaneStorage};

// upstream: src/enc/picture_rescale_enc.c HALVE
/// Half of `x`, rounded up.
fn halve(x: i32) -> i32 {
    (x + 1) >> 1
}

/// A sample position as an index; a negative position is a precondition violation.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("the sample position lies in the plane")
}

// upstream: src/enc/picture_rescale_enc.c AdjustAndCheckRectangle
// upstream: src/enc/picture_rescale_enc.c SnapTopLeftPosition
/// Adjusts the top-left corner to the chroma sample position, then checks that the
/// rectangle lies inside the picture (C: `AdjustAndCheckRectangle`).
fn adjust_and_check_rectangle(
    pic: &WebPPicture<'_>,
    left: &mut i32,
    top: &mut i32,
    width: i32,
    height: i32,
) -> bool {
    if pic.use_argb == 0 {
        *left &= !1;
        *top &= !1;
    }
    if *left < 0 || *top < 0 {
        return false;
    }
    if width <= 0 || height <= 0 {
        return false;
    }
    if *left + width > pic.width {
        return false;
    }
    if *top + height > pic.height {
        return false;
    }
    true
}

// upstream: src/enc/picture_rescale_enc.c PictureGrabSpecs
/// Copies the specifications of `src` into a new picture that owns no buffers (C:
/// `PictureGrabSpecs`, which copies the whole struct and resets its buffers). The
/// callbacks and the caller's `extra_info` and `stats` are moved out of `src`, since
/// a boxed closure cannot be copied; `picture_return_callbacks` moves them back when
/// `src` is kept.
fn picture_grab_specs<'a>(src: &mut WebPPicture<'a>) -> WebPPicture<'a> {
    WebPPicture {
        use_argb: src.use_argb,
        colorspace: src.colorspace,
        width: src.width,
        height: src.height,
        pad1: src.pad1,
        pad2: src.pad2,
        writer: src.writer.take(),
        extra_info_type: src.extra_info_type,
        extra_info: src.extra_info.take(),
        stats: src.stats.take(),
        error_code: src.error_code,
        progress_hook: src.progress_hook.take(),
        pad3: src.pad3,
        pad6: src.pad6,
        ..WebPPicture::default()
    }
}

/// Moves the callbacks and the caller's references from `from` back to `to`, which
/// keeps them when a rescale or a crop fails (the C struct copy leaves them in place).
fn picture_return_callbacks<'a>(from: &mut WebPPicture<'a>, to: &mut WebPPicture<'a>) {
    to.writer = from.writer.take();
    to.extra_info = from.extra_info.take();
    to.stats = from.stats.take();
    to.progress_hook = from.progress_hook.take();
}

/// The mutable samples of one plane of a YUVA storage.
fn yuva_plane_mut<'a>(
    storage: &'a mut PlaneStorage<'_, u8>,
    plane: Option<PlaneOffset>,
) -> &'a mut [u8] {
    let [slot, _, _, _] = yuva_planes_mut(storage, [plane, None, None, None]);
    slot.expect("the plane is allocated")
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureIsView
/// Reports whether `picture` borrows its planes instead of owning them (C:
/// `WebPPictureIsView`). A picture with no allocated plane counts as a view, as in C.
#[must_use]
pub fn webp_picture_is_view(picture: &WebPPicture<'_>) -> bool {
    let owned = if picture.use_argb != 0 {
        owns_samples(&picture.argb_storage)
    } else {
        owns_samples(&picture.yuva)
    };
    !owned
}

/// Whether a storage holds memory that the picture allocated (C: `memory_` or
/// `memory_argb_` is not `NULL`).
fn owns_samples<T>(storage: &PlaneStorage<'_, T>) -> bool {
    matches!(storage, PlaneStorage::Internal(samples) if !samples.is_empty())
}

// upstream: src/enc/picture_rescale_enc.c RescalePlane
/// Rescales one plane of `num_channels` samples per pixel (C: `RescalePlane`).
///
/// C reuses the caller's `work` block for every plane. The rescaler owns its two
/// side rows (ADR 0003), so each call gives it a vector of exactly
/// `2 * num_channels * dst_width` elements; the caller keeps C's allocation check.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature has ten parameters; only the work block moved inside"
)]
fn rescale_plane(
    src: &[u8],
    src_width: i32,
    src_height: i32,
    src_stride: i32,
    dst: &mut [u8],
    dst_width: i32,
    dst_height: i32,
    dst_stride: i32,
    num_channels: i32,
) -> bool {
    let work: Vec<rescaler_t> = vec![0; index(2 * num_channels * dst_width)];
    let Some(mut rescaler) = webp_rescaler_init(
        src_width,
        src_height,
        dst_width,
        dst_height,
        dst_stride,
        num_channels,
        work,
    ) else {
        return false;
    };
    let mut y = 0_i32;
    while y < src_height {
        let row = offset_by(0, i64::from(y) * i64::from(src_stride));
        y += webp_rescaler_import(&mut rescaler, src_height - y, &src[row..], src_stride);
        webp_rescaler_export(&mut rescaler, dst);
    }
    true
}

// upstream: src/enc/picture_rescale_enc.c AlphaMultiplyARGB
fn alpha_multiply_argb(pic: &mut WebPPicture<'_>, inverse: bool) {
    let plane = pic.argb.expect("the ARGB plane is allocated");
    let stride = pic.argb_stride;
    let width = pic.width;
    let height = pic.height;
    let samples = argb_samples_mut(&mut pic.argb_storage, plane);
    webp_mult_argb_rows(samples, stride, width, height, inverse);
}

// upstream: src/enc/picture_rescale_enc.c AlphaMultiplyY
fn alpha_multiply_y(pic: &mut WebPPicture<'_>, inverse: bool) {
    if pic.a.is_some() {
        let [y_plane, _, _, a_plane] = yuva_planes_mut(&mut pic.yuva, [pic.y, None, None, pic.a]);
        let y_plane = y_plane.expect("the luma plane is allocated");
        let a_plane = a_plane.expect("the alpha plane is allocated");
        webp_mult_rows(
            y_plane,
            pic.y_stride,
            a_plane,
            pic.a_stride,
            pic.width,
            pic.height,
            inverse,
        );
    }
}

/// Rescales the planes of `picture` into the planes of `tmp`, which has the new
/// dimensions. The error that C records on `picture` is returned for the caller to
/// record (C: `WebPPictureRescale`).
#[expect(
    clippy::too_many_lines,
    reason = "the function follows the C control flow"
)]
fn rescale_into(
    picture: &mut WebPPicture<'_>,
    tmp: &mut WebPPicture<'_>,
    prev_width: i32,
    prev_height: i32,
    width: i32,
    height: i32,
) -> Result<(), WebPEncodingError> {
    if picture.use_argb == 0 {
        // C allocates one work block for every plane; the check stays, and
        // `rescale_plane` gives each rescaler its own side rows.
        if webp_safe_malloc::<rescaler_t>(
            2 * u64::try_from(width).unwrap_or(0),
            std::mem::size_of::<rescaler_t>(),
        )
        .is_none()
        {
            return Err(WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        // If present, we need to rescale alpha first (for AlphaMultiplyY).
        if let Some(a_plane) = picture.a {
            // WebPInitAlphaProcessing() only assigns portable pointers (ADR 0003).
            let src = yuva_samples(&picture.yuva, a_plane);
            let dst = yuva_plane_mut(&mut tmp.yuva, tmp.a);
            if !rescale_plane(
                src,
                prev_width,
                prev_height,
                picture.a_stride,
                dst,
                width,
                height,
                tmp.a_stride,
                1,
            ) {
                return Err(WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION);
            }
        }

        // We take transparency into account on the luma plane only. That's not
        // totally exact blending, but still is a good approximation.
        alpha_multiply_y(picture, false);
        let y_src = yuva_samples(
            &picture.yuva,
            picture.y.expect("the luma plane is allocated"),
        );
        let y_dst = yuva_plane_mut(&mut tmp.yuva, tmp.y);
        let rescaled_y = rescale_plane(
            y_src,
            prev_width,
            prev_height,
            picture.y_stride,
            y_dst,
            width,
            height,
            tmp.y_stride,
            1,
        );
        let u_src = yuva_samples(&picture.yuva, picture.u.expect("the U plane is allocated"));
        let u_dst = yuva_plane_mut(&mut tmp.yuva, tmp.u);
        let rescaled_u = rescaled_y
            && rescale_plane(
                u_src,
                halve(prev_width),
                halve(prev_height),
                picture.uv_stride,
                u_dst,
                halve(width),
                halve(height),
                tmp.uv_stride,
                1,
            );
        let v_src = yuva_samples(&picture.yuva, picture.v.expect("the V plane is allocated"));
        let v_dst = yuva_plane_mut(&mut tmp.yuva, tmp.v);
        let rescaled_v = rescaled_u
            && rescale_plane(
                v_src,
                halve(prev_width),
                halve(prev_height),
                picture.uv_stride,
                v_dst,
                halve(width),
                halve(height),
                tmp.uv_stride,
                1,
            );
        if !rescaled_v {
            return Err(WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION);
        }
        alpha_multiply_y(tmp, true);
    } else {
        if webp_safe_malloc::<rescaler_t>(
            2 * u64::try_from(width).unwrap_or(0) * 4,
            std::mem::size_of::<rescaler_t>(),
        )
        .is_none()
        {
            return Err(WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
        }
        // In order to correctly interpolate colors, we need to apply the alpha
        // weighting first (black-matting), scale the RGB values, and remove
        // the premultiplication afterward (while preserving the alpha channel).
        // WebPInitAlphaProcessing() only assigns portable pointers (ADR 0003).
        alpha_multiply_argb(picture, false);
        let src_plane = picture.argb.expect("the ARGB plane is allocated");
        let dst_plane = tmp.argb.expect("the ARGB plane is allocated");
        // The rescaler works on the bytes of the ARGB samples (C casts them).
        let src_bytes = argb_bytes(argb_samples(&picture.argb_storage, src_plane));
        let mut dst_bytes = argb_bytes(argb_samples(&tmp.argb_storage, dst_plane));
        if !rescale_plane(
            &src_bytes,
            prev_width,
            prev_height,
            picture.argb_stride * 4,
            &mut dst_bytes,
            width,
            height,
            tmp.argb_stride * 4,
            4,
        ) {
            return Err(WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION);
        }
        // Store the rescaled samples back, one native-order word per pixel.
        let stride = index(tmp.argb_stride);
        let dst_samples = argb_samples_mut(&mut tmp.argb_storage, dst_plane);
        for row in 0..index(height) {
            for x in 0..index(width) {
                let at = (row * stride + x) * 4;
                let pixel = [
                    dst_bytes[at],
                    dst_bytes[at + 1],
                    dst_bytes[at + 2],
                    dst_bytes[at + 3],
                ];
                dst_samples[row * stride + x] = u32::from_ne_bytes(pixel);
            }
        }
        alpha_multiply_argb(tmp, true);
    }
    Ok(())
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureRescale
/// Rescales `picture` to `width` by `height`. A zero dimension keeps the aspect
/// ratio (C: `WebPPictureRescale`). The picture is replaced only on success; a
/// failure records the error code on `picture` and returns it.
/// # Errors
///
/// Returns the error that the picture records; the same code is in `error_code`.
pub fn webp_picture_rescale(
    picture: &mut WebPPicture<'_>,
    width: i32,
    height: i32,
) -> Result<(), WebPEncodingError> {
    let prev_width = picture.width;
    let prev_height = picture.height;
    let (mut width, mut height) = (width, height);
    if !webp_rescaler_get_scaled_dimensions(prev_width, prev_height, &mut width, &mut height) {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION,
        ));
    }

    let mut tmp = picture_grab_specs(picture);
    tmp.width = width;
    tmp.height = height;
    if webp_picture_alloc(&mut tmp).is_err() {
        let code = tmp.error_code;
        picture_return_callbacks(&mut tmp, picture);
        return Err(webp_encoding_set_error(picture, code));
    }

    if let Err(code) = rescale_into(picture, &mut tmp, prev_width, prev_height, width, height) {
        picture_return_callbacks(&mut tmp, picture);
        return Err(webp_encoding_set_error(picture, code));
    }

    webp_picture_free(picture);
    *picture = tmp;
    Ok(())
}

//------------------------------------------------------------------------------
// Picture cropping

// upstream: src/enc/picture_rescale_enc.c WebPPictureCrop
/// Crops `pic` to the rectangle at `left` and `top` with `width` and `height`
/// (C: `WebPPictureCrop`). A rectangle outside the picture returns `false` and
/// leaves the picture untouched. The chroma position is snapped for YUV pictures.
/// # Panics
///
/// Panics on a precondition that C leaves undefined, such as a missing plane.
pub fn webp_picture_crop(
    pic: &mut WebPPicture<'_>,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) -> bool {
    let (mut left, mut top) = (left, top);
    if !adjust_and_check_rectangle(pic, &mut left, &mut top, width, height) {
        return false;
    }

    let mut tmp = picture_grab_specs(pic);
    tmp.width = width;
    tmp.height = height;
    if webp_picture_alloc(&mut tmp).is_err() {
        let code = tmp.error_code;
        picture_return_callbacks(&mut tmp, pic);
        webp_encoding_set_error(pic, code);
        return false;
    }

    if pic.use_argb == 0 {
        let y_offset = top * pic.y_stride + left;
        let uv_offset = (top / 2) * pic.uv_stride + left / 2;
        let y_src = pic.y.expect("the luma plane is allocated");
        let y_dst = tmp.y.expect("the luma plane is allocated");
        webp_copy_plane(
            yuva_samples(&pic.yuva, y_src),
            index(y_offset),
            pic.y_stride,
            yuva_plane_mut(&mut tmp.yuva, Some(y_dst)),
            0,
            tmp.y_stride,
            width,
            height,
        );
        let u_src = pic.u.expect("the U plane is allocated");
        let u_dst = tmp.u.expect("the U plane is allocated");
        webp_copy_plane(
            yuva_samples(&pic.yuva, u_src),
            index(uv_offset),
            pic.uv_stride,
            yuva_plane_mut(&mut tmp.yuva, Some(u_dst)),
            0,
            tmp.uv_stride,
            halve(width),
            halve(height),
        );
        let v_src = pic.v.expect("the V plane is allocated");
        let v_dst = tmp.v.expect("the V plane is allocated");
        webp_copy_plane(
            yuva_samples(&pic.yuva, v_src),
            index(uv_offset),
            pic.uv_stride,
            yuva_plane_mut(&mut tmp.yuva, Some(v_dst)),
            0,
            tmp.uv_stride,
            halve(width),
            halve(height),
        );

        if let Some(a_dst) = tmp.a {
            let a_offset = top * pic.a_stride + left;
            let a_src = pic.a.expect("the alpha plane is allocated");
            webp_copy_plane(
                yuva_samples(&pic.yuva, a_src),
                index(a_offset),
                pic.a_stride,
                yuva_plane_mut(&mut tmp.yuva, Some(a_dst)),
                0,
                tmp.a_stride,
                width,
                height,
            );
        }
    } else {
        // C copies `width * 4` bytes per row of ARGB samples, which is `width` samples.
        let src_plane = pic.argb.expect("the ARGB plane is allocated");
        let dst_plane = tmp.argb.expect("the ARGB plane is allocated");
        let src_stride = index(pic.argb_stride);
        let dst_stride = index(tmp.argb_stride);
        let src_origin = index(top) * src_stride + index(left);
        let width_len = index(width);
        let src_samples = argb_samples(&pic.argb_storage, src_plane);
        let dst_samples = argb_samples_mut(&mut tmp.argb_storage, dst_plane);
        for row in 0..index(height) {
            let from = src_origin + row * src_stride;
            let to = row * dst_stride;
            dst_samples[to..to + width_len].copy_from_slice(&src_samples[from..from + width_len]);
        }
    }
    webp_picture_free(pic);
    *pic = tmp;
    true
}

//------------------------------------------------------------------------------
// Views and copies

/// The position of `plane` moved by `delta` samples, as a C pointer offset.
fn moved(plane: PlaneOffset, delta: i64) -> PlaneOffset {
    PlaneOffset {
        region: plane.region,
        offset: offset_by(plane.offset, delta),
    }
}

/// The regions of a storage, borrowed shared for as long as the storage is borrowed.
fn shared_regions<'s, T>(storage: &'s PlaneStorage<'_, T>) -> Vec<&'s [T]> {
    match storage {
        PlaneStorage::Internal(samples) => vec![samples.as_slice()],
        PlaneStorage::External(slices) => slices.iter().map(|slice| &**slice).collect(),
        PlaneStorage::Shared(slices) => slices.clone(),
    }
}

/// The specifications of `src` in a picture that owns no planes and holds no
/// callbacks, `extra_info` or `stats` (C: `PictureGrabSpecs` without the callbacks).
fn picture_specs<'a>(src: &WebPPicture<'_>) -> WebPPicture<'a> {
    WebPPicture {
        use_argb: src.use_argb,
        colorspace: src.colorspace,
        width: src.width,
        height: src.height,
        pad1: src.pad1,
        pad2: src.pad2,
        extra_info_type: src.extra_info_type,
        error_code: src.error_code,
        pad3: src.pad3,
        pad6: src.pad6,
        ..WebPPicture::default()
    }
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureView
/// Returns a view of the `width` by `height` rectangle at `left` and `top` of `src`
/// (C: `WebPPictureView`, which fills `dst`). The view shares the planes of `src`:
/// they are borrowed for `'s` in a `Shared` storage, so the view cannot outlive
/// `src`, and writing into it panics. The view has no callbacks, `extra_info` or
/// `stats`; the error code of `src` is copied, as in C. Returns `None` where C
/// returns 0.
#[must_use]
pub fn webp_picture_view<'s>(
    src: &'s WebPPicture<'_>,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) -> Option<WebPPicture<'s>> {
    let (mut left, mut top) = (left, top);
    if !adjust_and_check_rectangle(src, &mut left, &mut top, width, height) {
        return None;
    }

    let mut dst: WebPPicture<'s> = picture_specs(src);
    dst.width = width;
    dst.height = height;
    if src.use_argb == 0 {
        let y_shift = i64::from(top) * i64::from(src.y_stride) + i64::from(left);
        let uv_shift = i64::from(top >> 1) * i64::from(src.uv_stride) + i64::from(left >> 1);
        dst.y = src.y.map(|plane| moved(plane, y_shift));
        dst.u = src.u.map(|plane| moved(plane, uv_shift));
        dst.v = src.v.map(|plane| moved(plane, uv_shift));
        dst.y_stride = src.y_stride;
        dst.uv_stride = src.uv_stride;
        if let Some(a) = src.a {
            dst.a = Some(moved(
                a,
                i64::from(top) * i64::from(src.a_stride) + i64::from(left),
            ));
            dst.a_stride = src.a_stride;
        }
        dst.yuva = PlaneStorage::Shared(shared_regions(&src.yuva));
    } else {
        let argb_shift = i64::from(top) * i64::from(src.argb_stride) + i64::from(left);
        dst.argb = src.argb.map(|plane| moved(plane, argb_shift));
        dst.argb_stride = src.argb_stride;
        dst.argb_storage = PlaneStorage::Shared(shared_regions(&src.argb_storage));
    }
    Some(dst)
}

/// Copies a plane of `width` by `height` samples from one storage to another (C:
/// `WebPCopyPlane` on the two planes).
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of WebPCopyPlane has eight parameters"
)]
fn copy_yuva_plane(
    src: &PlaneStorage<'_, u8>,
    src_plane: PlaneOffset,
    src_stride: i32,
    dst: &mut PlaneStorage<'_, u8>,
    dst_plane: PlaneOffset,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    let source = yuva_samples(src, src_plane);
    let target = yuva_plane_mut(dst, Some(dst_plane));
    webp_copy_plane(source, 0, src_stride, target, 0, dst_stride, width, height);
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureCopy
/// Copies `src` into `dst`: the specifications, new planes, and the samples of
/// `src` in them (C: `WebPPictureCopy`). `dst` gets no writer, progress hook,
/// `extra_info` or `stats`, where C shares the pointers of `src`; see the
/// `# Deviations from upstream` section of the module documentation (item 1). The
/// error code of `src` is copied, and a failed allocation returns the code that
/// `dst` records.
///
/// # Errors
///
/// Returns the allocation error that `dst` records.
///
/// # Panics
///
/// Panics on a plane that C would dereference while it is missing.
pub fn webp_picture_copy(
    src: &WebPPicture<'_>,
    dst: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    *dst = picture_specs(src);
    webp_picture_alloc(dst)?;
    let width = dst.width;
    let height = dst.height;
    if src.use_argb == 0 {
        let src_y = src.y.expect("the luma plane of the source is allocated");
        let dst_y = dst.y.expect("the luma plane is allocated");
        copy_yuva_plane(
            &src.yuva,
            src_y,
            src.y_stride,
            &mut dst.yuva,
            dst_y,
            dst.y_stride,
            width,
            height,
        );
        let src_u = src.u.expect("the U plane of the source is allocated");
        let dst_u = dst.u.expect("the U plane is allocated");
        copy_yuva_plane(
            &src.yuva,
            src_u,
            src.uv_stride,
            &mut dst.yuva,
            dst_u,
            dst.uv_stride,
            halve(width),
            halve(height),
        );
        let src_v = src.v.expect("the V plane of the source is allocated");
        let dst_v = dst.v.expect("the V plane is allocated");
        copy_yuva_plane(
            &src.yuva,
            src_v,
            src.uv_stride,
            &mut dst.yuva,
            dst_v,
            dst.uv_stride,
            halve(width),
            halve(height),
        );
        if let Some(dst_a) = dst.a {
            let src_a = src.a.expect("the alpha plane of the source is allocated");
            copy_yuva_plane(
                &src.yuva,
                src_a,
                src.a_stride,
                &mut dst.yuva,
                dst_a,
                dst.a_stride,
                width,
                height,
            );
        }
    } else {
        // C copies `4 * width` bytes per row, which is `width` ARGB samples.
        let src_plane = src.argb.expect("the ARGB plane of the source is allocated");
        let dst_plane = dst.argb.expect("the ARGB plane is allocated");
        let src_stride = index(src.argb_stride);
        let dst_stride = index(dst.argb_stride);
        let row_len = index(width);
        let source = argb_samples(&src.argb_storage, src_plane);
        let target = argb_samples_mut(&mut dst.argb_storage, dst_plane);
        for row in 0..index(height) {
            let from = row * src_stride;
            let to = row * dst_stride;
            target[to..to + row_len].copy_from_slice(&source[from..from + row_len]);
        }
    }
    Ok(())
}
