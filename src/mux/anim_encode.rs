// libwebp v1.6.0, src/mux/anim_encode.c:
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
//  AnimEncoder implementation.
//

//! Translation of `src/mux/anim_encode.c` from libwebp v1.6.0.
//!
//! Work package: WP15 (mux and animation encode).
//!
//! - The encoder owns its mux, the three canvases, the ring of encoded frames,
//!   both configurations, the options and the error string. C's `curr_canvas`,
//!   a pointer to the caller's frame during `WebPAnimEncoderAdd`, is the frame
//!   argument of the functions that read it; nothing keeps it.
//! - C makes `WebPPictureView`s of the current canvas for the sub-frame
//!   rectangles (`GetSubRect`) and encodes them later (`EncodeCandidate`),
//!   after `IncreaseTransparency` or `FlattenSimilarBlocks` may have changed the
//!   canvas through which the view sees, and the lossless encoding writes into
//!   the canvas through the view (`WebPReplaceTransparentPixels` unless `exact`),
//!   which a later candidate reads. The translation checks the rectangle where C
//!   makes the view and makes the view where it is used: a shared one
//!   (`webp_picture_view`) for reading, and one that borrows the canvas's ARGB
//!   samples exclusively for an encoding ([`sub_frame_view`]), so that the
//!   encoder writes where C writes (`docs/status/wp15.md`).
//! - C copies the frame's progress hook into the canvas copy and from there
//!   into the views. The views here call the frame's hook through a borrow, so
//!   the candidate encodings report progress as in C; the canvas copy holds no
//!   hook, so the re-encoding of `WebPAnimEncoderAssemble` reports none, where C
//!   calls the hook of the last frame (`docs/status/wp15.md`).

use std::borrow::Cow;
use std::cell::RefCell;
use std::io::Write as _;
use std::rc::Rc;

use crate::dec::webp_dec::webp_decode;
use crate::enc::config_enc::webp_validate_config;
use crate::enc::picture_csp_enc::webp_picture_yuva_to_argb;
use crate::enc::picture_enc::{
    argb_samples, argb_samples_mut, webp_memory_write, webp_memory_writer_clear,
    webp_memory_writer_init, webp_picture_alloc,
};
use crate::enc::picture_rescale_enc::{webp_picture_copy, webp_picture_view};
use crate::enc::webp_enc::webp_encode;
use crate::mux::muxedit::{
    webp_mux_assemble, webp_mux_delete, webp_mux_delete_chunk, webp_mux_push_frame,
    webp_mux_set_animation_params, webp_mux_set_canvas_size, webp_mux_set_chunk,
    webp_mux_set_image,
};
use crate::mux::muxi::WebPMux;
use crate::mux::muxread::{webp_mux_get_canvas_size, webp_mux_get_chunk, webp_mux_get_frame};
use crate::utils::utils::{webp_copy_pixels, webp_get_color_palette, webp_safe_calloc};
use crate::webp::decode::{
    WEBP_CSP_MODE, WebPDecBufferPlanes, WebPRGBABuffer, webp_get_features, webp_init_decoder_config,
};
use crate::webp::encode::{
    WebPConfig, WebPEncodingError, WebPMemoryWriter, WebPPicture, webp_config_init,
    webp_picture_init,
};
use crate::webp::format_constants::{MAX_DURATION, MAX_IMAGE_AREA};
use crate::webp::mux::{
    WEBP_MUX_ABI_VERSION, WebPAnimEncoderOptions, WebPChunkId, WebPMuxError, WebPMuxFrameInfo,
    webp_mux_create, webp_mux_new,
};
use crate::webp::mux_types::{
    WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose, webp_data_clear, webp_data_copy,
};
use crate::webp::types::{PlaneOffset, PlaneStorage, webp_abi_is_incompatible};

// upstream: src/mux/anim_encode.c ERROR_STR_MAX_LENGTH
const ERROR_STR_MAX_LENGTH: usize = 100;

//------------------------------------------------------------------------------
// Internal structs.

// upstream: src/mux/anim_encode.c FrameRectangle
/// Stores frame rectangle dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FrameRectangle {
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
}

// upstream: src/mux/anim_encode.c EncodedFrame
/// Used to store two candidates of encoded data for an animation frame. One of
/// the two will be chosen later.
#[expect(clippy::struct_field_names, reason = "the C field names")]
#[derive(Clone, Debug, Default)]
struct EncodedFrame {
    /// Encoded frame rectangle.
    sub_frame: WebPMuxFrameInfo<'static>,
    /// Encoded frame if it is a key-frame.
    key_frame: WebPMuxFrameInfo<'static>,
    /// True if `key_frame` has been chosen.
    is_key_frame: bool,
}

/// C's `sizeof(EncodedFrame)`, for the allocation test of the frame array: two
/// `WebPMuxFrameInfo` (a pointer and a `size_t`, then seven 32-bit fields,
/// padded to the pointer alignment) and an `int`, padded the same way.
const ENCODED_FRAME_SIZE: usize = {
    let word = core::mem::size_of::<usize>();
    let info = (2 * word + 7 * 4).next_multiple_of(word);
    (2 * info + 4).next_multiple_of(word)
};

// upstream: src/mux/anim_encode.c WebPAnimEncoder
/// The animation encoder (C: `struct WebPAnimEncoder`, opaque in `mux.h`).
/// Chunks set without copying (`WebPAnimEncoderSetChunk`) borrow their bytes
/// for `'a`; the frames are always copied into the mux.
#[expect(clippy::struct_excessive_bools, reason = "the C struct's flags")]
pub struct WebPAnimEncoder<'a> {
    /// Canvas width.
    canvas_width: i32,
    /// Canvas height.
    canvas_height: i32,
    /// Global encoding options.
    options: WebPAnimEncoderOptions,

    /// Previous WebP frame rectangle.
    prev_rect: FrameRectangle,
    /// Cached in case a re-encode is needed.
    last_config: WebPConfig,
    /// If `last_config` uses lossless, then this config uses lossy and vice
    /// versa; only valid if `options.allow_mixed` is true.
    last_config_reversed: WebPConfig,

    // C: WebPPicture* curr_canvas, the caller's frame during WebPAnimEncoderAdd;
    // passed as an argument here.

    // Canvas buffers.
    /// Possibly modified current canvas.
    curr_canvas_copy: WebPPicture<'static>,
    /// True if pixels in `curr_canvas_copy` differ from those in `curr_canvas`.
    curr_canvas_copy_modified: bool,

    /// Previous canvas.
    prev_canvas: WebPPicture<'static>,
    /// Previous canvas disposed to background.
    prev_canvas_disposed: WebPPicture<'static>,

    // Encoded data.
    /// Array of encoded frames.
    encoded_frames: Vec<EncodedFrame>,
    /// Number of allocated frames.
    size: usize,
    /// Frame start index.
    start: usize,
    /// Number of valid frames.
    count: usize,
    /// If >0, `flush_count` frames starting from `start` are ready to be added
    /// to mux.
    flush_count: usize,

    // key-frame related.
    /// min(canvas size - frame size) over the frames. Can be negative in certain
    /// cases due to transparent pixels in a frame.
    best_delta: i64,
    /// Index of selected key-frame relative to `start`.
    keyframe: i32,
    /// Frames seen since the last key-frame.
    count_since_key_frame: i32,

    /// Timestamp of the first frame.
    first_timestamp: i32,
    /// Timestamp of the last added frame.
    prev_timestamp: i32,
    /// True if it's not yet decided if previous frame would be a sub-frame or a
    /// key-frame.
    prev_candidate_undecided: bool,

    // Misc.
    /// True if first frame is yet to be added/being added.
    is_first_frame: bool,
    /// True if `WebPAnimEncoderAdd()` has already been called with a NULL frame.
    got_null_frame: bool,

    /// Number of input frames processed so far.
    in_frame_count: usize,
    /// Number of frames added to mux so far. This may be different from
    /// `in_frame_count` due to merging.
    out_frame_count: usize,

    /// Muxer to assemble the WebP bitstream.
    mux: WebPMux<'a>,
    /// Error string. Empty if no error.
    error_str: String,
}

// -----------------------------------------------------------------------------
// Life of WebPAnimEncoder object.

// upstream: src/mux/anim_encode.c DELTA_INFINITY
const DELTA_INFINITY: i64 = 1 << 32;
// upstream: src/mux/anim_encode.c KEYFRAME_NONE
const KEYFRAME_NONE: i32 = -1;

// upstream: src/mux/anim_encode.c ResetCounters
/// Reset the counters in the `WebPAnimEncoder`.
fn reset_counters(enc: &mut WebPAnimEncoder<'_>) {
    enc.start = 0;
    enc.count = 0;
    enc.flush_count = 0;
    enc.best_delta = DELTA_INFINITY;
    enc.keyframe = KEYFRAME_NONE;
}

// upstream: src/mux/anim_encode.c DisableKeyframes
fn disable_keyframes(enc_options: &mut WebPAnimEncoderOptions) {
    enc_options.kmax = i32::MAX;
    enc_options.kmin = enc_options.kmax - 1;
}

// upstream: src/mux/anim_encode.c MAX_CACHED_FRAMES
const MAX_CACHED_FRAMES: i32 = 30;

/// `fprintf(stderr, ...)`: write errors are ignored, as C ignores them.
fn print_stderr(args: core::fmt::Arguments<'_>) {
    let _ = std::io::stderr().write_fmt(args);
}

// upstream: src/mux/anim_encode.c SanitizeEncoderOptions
fn sanitize_encoder_options(enc_options: &mut WebPAnimEncoderOptions) {
    let mut print_warning = enc_options.verbose != 0;

    if enc_options.minimize_size != 0 {
        disable_keyframes(enc_options);
    }

    if enc_options.kmax == 1 {
        // All frames will be key-frames.
        enc_options.kmin = 0;
        enc_options.kmax = 0;
        return;
    } else if enc_options.kmax <= 0 {
        disable_keyframes(enc_options);
        print_warning = false;
    }

    if enc_options.kmin >= enc_options.kmax {
        enc_options.kmin = enc_options.kmax - 1;
        if print_warning {
            print_stderr(format_args!(
                "WARNING: Setting kmin = {}, so that kmin < kmax.\n",
                enc_options.kmin
            ));
        }
    } else {
        let kmin_limit = enc_options.kmax / 2 + 1;
        if enc_options.kmin < kmin_limit && kmin_limit < enc_options.kmax {
            // This ensures that enc.keyframe + kmin >= kmax is always true. So, we
            // can flush all the frames in the 'count_since_key_frame == kmax' case.
            enc_options.kmin = kmin_limit;
            if print_warning {
                print_stderr(format_args!(
                    "WARNING: Setting kmin = {}, so that kmin >= kmax / 2 + 1.\n",
                    enc_options.kmin
                ));
            }
        }
    }
    // Limit the max number of frames that are allocated.
    if enc_options.kmax - enc_options.kmin > MAX_CACHED_FRAMES {
        enc_options.kmin = enc_options.kmax - MAX_CACHED_FRAMES;
        if print_warning {
            print_stderr(format_args!(
                "WARNING: Setting kmin = {}, so that kmax - kmin <= {}.\n",
                enc_options.kmin, MAX_CACHED_FRAMES
            ));
        }
    }
    debug_assert!(enc_options.kmin < enc_options.kmax);
}

// upstream: src/mux/anim_encode.c DefaultEncoderOptions
fn default_encoder_options(enc_options: &mut WebPAnimEncoderOptions) {
    enc_options.anim_params.loop_count = 0;
    enc_options.anim_params.bgcolor = 0xffff_ffff; // White.
    enc_options.minimize_size = 0;
    disable_keyframes(enc_options);
    enc_options.allow_mixed = 0;
    enc_options.verbose = 0;
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderOptionsInitInternal
/// Internal, version-checked, entry point of `WebPAnimEncoderOptionsInit`.
/// Returns false in case of version mismatch (C: also for `NULL` options).
#[must_use]
pub fn webp_anim_encoder_options_init_internal(
    enc_options: &mut WebPAnimEncoderOptions,
    abi_version: i32,
) -> bool {
    if webp_abi_is_incompatible(abi_version, WEBP_MUX_ABI_VERSION) {
        return false;
    }
    default_encoder_options(enc_options);
    true
}

// upstream: src/mux/anim_encode.c TRANSPARENT_COLOR
/// This value is used to match a later call to `WebPReplaceTransparentPixels()`,
/// making it a no-op for lossless (see `WebPEncode()`).
const TRANSPARENT_COLOR: u32 = 0x0000_0000;

/// C's conversion of an `int` to `uint64_t`, which sign-extends.
fn c_u64(value: i32) -> u64 {
    i64::from(value).cast_unsigned()
}

/// The position of the sample at `x`, `y` of an ARGB plane with a stride of
/// `stride` pixels (C: `y * argb_stride + x`, computed in `int`).
fn pixel(x: i32, y: i32, stride: i32) -> usize {
    usize::try_from(y * stride + x).expect("a pixel inside the canvas")
}

/// The ARGB samples of a picture, from its first sample (C: `picture->argb`).
fn argb<'p>(picture: &'p WebPPicture<'_>) -> &'p [u32] {
    argb_samples(
        &picture.argb_storage,
        picture.argb.expect("the canvas has an ARGB plane"),
    )
}

/// The ARGB samples of a picture, mutably.
fn argb_mut<'p>(picture: &'p mut WebPPicture<'_>) -> &'p mut [u32] {
    let plane = picture.argb.expect("the canvas has an ARGB plane");
    argb_samples_mut(&mut picture.argb_storage, plane)
}

// upstream: src/mux/anim_encode.c ClearRectangle
fn clear_rectangle(picture: &mut WebPPicture<'_>, left: i32, top: i32, width: i32, height: i32) {
    let stride = picture.argb_stride;
    let samples = argb_mut(picture);
    for j in top..top + height {
        for i in left..left + width {
            samples[pixel(i, j, stride)] = TRANSPARENT_COLOR;
        }
    }
}

// upstream: src/mux/anim_encode.c WebPUtilClearPic
fn webp_util_clear_pic(picture: &mut WebPPicture<'_>, rect: Option<&FrameRectangle>) {
    if let Some(rect) = rect {
        clear_rectangle(
            picture,
            rect.x_offset,
            rect.y_offset,
            rect.width,
            rect.height,
        );
    } else {
        let (width, height) = (picture.width, picture.height);
        clear_rectangle(picture, 0, 0, width, height);
    }
}

// upstream: src/mux/anim_encode.c MarkNoError
fn mark_no_error(enc: &mut WebPAnimEncoder<'_>) {
    enc.error_str.clear(); // Empty string.
}

/// `snprintf(enc->error_str, ERROR_STR_MAX_LENGTH, ...)`: the message, cut to
/// the `ERROR_STR_MAX_LENGTH - 1` bytes that fit before the terminating `NUL`.
fn set_error_str(enc: &mut WebPAnimEncoder<'_>, message: String) {
    let mut message = message;
    let mut end = message.len().min(ERROR_STR_MAX_LENGTH - 1);
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message.truncate(end);
    enc.error_str = message;
}

// upstream: src/mux/anim_encode.c MarkError
/// Records `"<str>."` as the error string. Crate-visible for the C ABI, which
/// reports frames that it cannot lend this way.
pub(crate) fn mark_error(enc: &mut WebPAnimEncoder<'_>, str: &str) {
    set_error_str(enc, format!("{str}."));
}

// upstream: src/mux/anim_encode.c MarkError2
fn mark_error2(enc: &mut WebPAnimEncoder<'_>, str: &str, error_code: i32) {
    set_error_str(enc, format!("{str}: {error_code}."));
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderNewInternal
/// Internal, version-checked, entry point of `WebPAnimEncoderNew`. Returns
/// `None` on a version mismatch, invalid dimensions or a memory error.
///
/// # Panics
///
/// Never: the sanitized options keep `kmax - kmin + 1` positive.
#[must_use]
pub fn webp_anim_encoder_new_internal<'a>(
    width: i32,
    height: i32,
    enc_options: Option<&WebPAnimEncoderOptions>,
    abi_version: i32,
) -> Option<WebPAnimEncoder<'a>> {
    if webp_abi_is_incompatible(abi_version, WEBP_MUX_ABI_VERSION) {
        return None;
    }
    if width <= 0 || height <= 0 || c_u64(width) * c_u64(height) >= MAX_IMAGE_AREA {
        return None;
    }

    // C: enc = WebPSafeCalloc(1, sizeof(*enc)), a value here.
    let mut enc = WebPAnimEncoder {
        canvas_width: 0,
        canvas_height: 0,
        options: WebPAnimEncoderOptions::default(),
        prev_rect: FrameRectangle::default(),
        last_config: WebPConfig::default(),
        last_config_reversed: WebPConfig::default(),
        curr_canvas_copy: WebPPicture::default(),
        curr_canvas_copy_modified: false,
        prev_canvas: WebPPicture::default(),
        prev_canvas_disposed: WebPPicture::default(),
        encoded_frames: Vec::new(),
        size: 0,
        start: 0,
        count: 0,
        flush_count: 0,
        best_delta: 0,
        keyframe: 0,
        count_since_key_frame: 0,
        first_timestamp: 0,
        prev_timestamp: 0,
        prev_candidate_undecided: false,
        is_first_frame: false,
        got_null_frame: false,
        in_frame_count: 0,
        out_frame_count: 0,
        mux: WebPMux::default(),
        error_str: String::new(),
    };
    mark_no_error(&mut enc);

    // Dimensions and options.
    enc.canvas_width = width;
    enc.canvas_height = height;
    if let Some(enc_options) = enc_options {
        enc.options = *enc_options;
        sanitize_encoder_options(&mut enc.options);
    } else {
        default_encoder_options(&mut enc.options);
    }

    // Canvas buffers.
    if !webp_picture_init(&mut enc.curr_canvas_copy)
        || !webp_picture_init(&mut enc.prev_canvas)
        || !webp_picture_init(&mut enc.prev_canvas_disposed)
    {
        return None;
    }
    enc.curr_canvas_copy.width = width;
    enc.curr_canvas_copy.height = height;
    enc.curr_canvas_copy.use_argb = 1;
    if webp_picture_alloc(&mut enc.curr_canvas_copy).is_err()
        || webp_picture_copy(&enc.curr_canvas_copy, &mut enc.prev_canvas).is_err()
        || webp_picture_copy(&enc.curr_canvas_copy, &mut enc.prev_canvas_disposed).is_err()
    {
        return None;
    }
    webp_util_clear_pic(&mut enc.prev_canvas, None);
    enc.curr_canvas_copy_modified = true;

    // Encoded frames.
    reset_counters(&mut enc);
    // Note: one extra storage is for the previous frame.
    enc.size = usize::try_from(enc.options.kmax - enc.options.kmin + 1)
        .expect("SanitizeEncoderOptions keeps kmin below kmax, or both at 0");
    // We need space for at least 2 frames. But when kmin, kmax are both zero,
    // enc->size will be 1. So we handle that special case below.
    if enc.size < 2 {
        enc.size = 2;
    }
    enc.encoded_frames = webp_safe_calloc::<EncodedFrame>(enc.size as u64, ENCODED_FRAME_SIZE)?;

    enc.mux = webp_mux_new()?;

    enc.count_since_key_frame = 0;
    enc.first_timestamp = 0;
    enc.prev_timestamp = 0;
    enc.prev_candidate_undecided = false;
    enc.is_first_frame = true;
    enc.got_null_frame = false;

    Some(enc) // All OK.
}

// upstream: src/mux/anim_encode.c FrameRelease
/// Release the data contained by `encoded_frame`.
fn frame_release(encoded_frame: &mut EncodedFrame) {
    webp_data_clear(&mut encoded_frame.sub_frame.bitstream);
    webp_data_clear(&mut encoded_frame.key_frame.bitstream);
    *encoded_frame = EncodedFrame::default();
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderDelete
/// Deletes the `WebPAnimEncoder` object (the drop of the value; C frees the
/// pictures, the frames and the mux).
pub fn webp_anim_encoder_delete(enc: WebPAnimEncoder<'_>) {
    let mut enc = enc;
    for encoded_frame in &mut enc.encoded_frames {
        frame_release(encoded_frame);
    }
    webp_mux_delete(enc.mux);
}

// -----------------------------------------------------------------------------
// Frame addition.

// upstream: src/mux/anim_encode.c GetFrame
/// Returns the position of the cached frame at the given `position` relative to
/// `start`. C adds the `size_t` values with wrap-around.
fn get_frame(enc: &WebPAnimEncoder<'_>, position: usize) -> usize {
    let index = enc.start.wrapping_add(position);
    debug_assert!(index < enc.size);
    index
}

// upstream: src/mux/anim_encode.c ComparePixelsFunc
/// Compares `length` pixels of `src` and `dst` from the given positions, with
/// the given steps between pixels.
type ComparePixelsFunc = fn(&[u32], usize, usize, &[u32], usize, usize, i32, i32) -> bool;

// upstream: src/mux/anim_encode.c ComparePixelsLossless
/// Returns true if `length` number of pixels in `src` and `dst` are equal,
/// assuming the given step sizes between pixels. `max_allowed_diff` is unused
/// and only there to allow function pointer use.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature with the positions of its pointers"
)]
fn compare_pixels_lossless(
    src: &[u32],
    src_pos: usize,
    src_step: usize,
    dst: &[u32],
    dst_pos: usize,
    dst_step: usize,
    length: i32,
    _max_allowed_diff: i32,
) -> bool {
    debug_assert!(length > 0);
    let (mut src_pos, mut dst_pos, mut length) = (src_pos, dst_pos, length);
    while length > 0 {
        length -= 1;
        if src[src_pos] != dst[dst_pos] {
            return false;
        }
        src_pos += src_step;
        dst_pos += dst_step;
    }
    true
}

// upstream: src/mux/anim_encode.c PixelsAreSimilar
/// Helper to check if each channel in `src` and `dst` is at most off by
/// `max_allowed_diff`.
fn pixels_are_similar(src: u32, dst: u32, max_allowed_diff: i32) -> bool {
    let src_a = ((src >> 24) & 0xff).cast_signed();
    let src_r = ((src >> 16) & 0xff).cast_signed();
    let src_g = ((src >> 8) & 0xff).cast_signed();
    let src_b = (src & 0xff).cast_signed();
    let dst_a = ((dst >> 24) & 0xff).cast_signed();
    let dst_r = ((dst >> 16) & 0xff).cast_signed();
    let dst_g = ((dst >> 8) & 0xff).cast_signed();
    let dst_b = (dst & 0xff).cast_signed();

    (src_a == dst_a)
        && ((src_r - dst_r).abs() * dst_a <= (max_allowed_diff * 255))
        && ((src_g - dst_g).abs() * dst_a <= (max_allowed_diff * 255))
        && ((src_b - dst_b).abs() * dst_a <= (max_allowed_diff * 255))
}

// upstream: src/mux/anim_encode.c ComparePixelsLossy
/// Returns true if `length` number of pixels in `src` and `dst` are within an
/// error bound, assuming the given step sizes between pixels.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature with the positions of its pointers"
)]
fn compare_pixels_lossy(
    src: &[u32],
    src_pos: usize,
    src_step: usize,
    dst: &[u32],
    dst_pos: usize,
    dst_step: usize,
    length: i32,
    max_allowed_diff: i32,
) -> bool {
    debug_assert!(length > 0);
    let (mut src_pos, mut dst_pos, mut length) = (src_pos, dst_pos, length);
    while length > 0 {
        length -= 1;
        if !pixels_are_similar(src[src_pos], dst[dst_pos], max_allowed_diff) {
            return false;
        }
        src_pos += src_step;
        dst_pos += dst_step;
    }
    true
}

// upstream: src/mux/anim_encode.c IsEmptyRect
fn is_empty_rect(rect: &FrameRectangle) -> bool {
    (rect.width == 0) || (rect.height == 0)
}

// upstream: src/mux/anim_encode.c QualityToMaxDiff
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's (int) conversion of a value in [1, 31] for qualities in [0, 100]"
)]
fn quality_to_max_diff(quality: f32) -> i32 {
    let val = (f64::from(quality) / 100.0).powf(0.5);
    let max_diff = 31.0 * (1.0 - val) + 1.0 * val;
    (max_diff + 0.5) as i32
}

/// A stride in pixels as a step between samples (the canvases have positive
/// strides).
fn step(stride: i32) -> usize {
    usize::try_from(stride).expect("a positive stride")
}

// upstream: src/mux/anim_encode.c MinimizeChangeRectangle
/// Assumes that an initial valid guess of change rectangle `rect` is passed.
#[expect(
    clippy::too_many_lines,
    reason = "the four boundary loops of the C function"
)]
fn minimize_change_rectangle(
    src: &WebPPicture<'_>,
    dst: &WebPPicture<'_>,
    rect: &mut FrameRectangle,
    is_lossless: bool,
    quality: f32,
) {
    let compare_pixels: ComparePixelsFunc = if is_lossless {
        compare_pixels_lossless
    } else {
        compare_pixels_lossy
    };
    let max_allowed_diff_lossy = quality_to_max_diff(quality);
    let max_allowed_diff = if is_lossless {
        0
    } else {
        max_allowed_diff_lossy
    };
    let (src_argb, src_stride) = (argb(src), src.argb_stride);
    let (dst_argb, dst_stride) = (argb(dst), dst.argb_stride);

    // Assumption/correctness checks.
    debug_assert!(src.width == dst.width && src.height == dst.height);
    debug_assert!(rect.x_offset + rect.width <= dst.width);
    debug_assert!(rect.y_offset + rect.height <= dst.height);

    let no_change = 'search: {
        // Left boundary.
        let mut i = rect.x_offset;
        while i < rect.x_offset + rect.width {
            let src_pos = pixel(i, rect.y_offset, src_stride);
            let dst_pos = pixel(i, rect.y_offset, dst_stride);
            if compare_pixels(
                src_argb,
                src_pos,
                step(src_stride),
                dst_argb,
                dst_pos,
                step(dst_stride),
                rect.height,
                max_allowed_diff,
            ) {
                rect.width -= 1; // Redundant column.
                rect.x_offset += 1;
            } else {
                break;
            }
            i += 1;
        }
        if rect.width == 0 {
            break 'search true;
        }

        // Right boundary.
        let mut i = rect.x_offset + rect.width - 1;
        while i >= rect.x_offset {
            let src_pos = pixel(i, rect.y_offset, src_stride);
            let dst_pos = pixel(i, rect.y_offset, dst_stride);
            if compare_pixels(
                src_argb,
                src_pos,
                step(src_stride),
                dst_argb,
                dst_pos,
                step(dst_stride),
                rect.height,
                max_allowed_diff,
            ) {
                rect.width -= 1; // Redundant column.
            } else {
                break;
            }
            i -= 1;
        }
        if rect.width == 0 {
            break 'search true;
        }

        // Top boundary.
        let mut j = rect.y_offset;
        while j < rect.y_offset + rect.height {
            let src_pos = pixel(rect.x_offset, j, src_stride);
            let dst_pos = pixel(rect.x_offset, j, dst_stride);
            if compare_pixels(
                src_argb,
                src_pos,
                1,
                dst_argb,
                dst_pos,
                1,
                rect.width,
                max_allowed_diff,
            ) {
                rect.height -= 1; // Redundant row.
                rect.y_offset += 1;
            } else {
                break;
            }
            j += 1;
        }
        if rect.height == 0 {
            break 'search true;
        }

        // Bottom boundary.
        let mut j = rect.y_offset + rect.height - 1;
        while j >= rect.y_offset {
            let src_pos = pixel(rect.x_offset, j, src_stride);
            let dst_pos = pixel(rect.x_offset, j, dst_stride);
            if compare_pixels(
                src_argb,
                src_pos,
                1,
                dst_argb,
                dst_pos,
                1,
                rect.width,
                max_allowed_diff,
            ) {
                rect.height -= 1; // Redundant row.
            } else {
                break;
            }
            j -= 1;
        }
        if rect.height == 0 {
            break 'search true;
        }

        is_empty_rect(rect)
    };
    if no_change {
        // NoChange:
        rect.x_offset = 0;
        rect.y_offset = 0;
        rect.width = 0;
        rect.height = 0;
    }
}

// upstream: src/mux/anim_encode.c SnapToEvenOffsets
/// Snap rectangle to even offsets (and adjust dimensions if needed).
fn snap_to_even_offsets(rect: &mut FrameRectangle) {
    rect.width += rect.x_offset & 1;
    rect.height += rect.y_offset & 1;
    rect.x_offset &= !1;
    rect.y_offset &= !1;
}

// upstream: src/mux/anim_encode.c SubFrameParams
/// The rectangles of one dispose method. C also keeps the views of the current
/// canvas for the two rectangles (`sub_frame_ll`, `sub_frame_lossy`); the
/// translation makes them where they are used (see the module documentation).
#[derive(Clone, Copy, Debug, Default)]
struct SubFrameParams {
    /// Should try this set of parameters.
    should_try: bool,
    /// Frame with empty rectangle can be skipped.
    empty_rect_allowed: bool,
    /// Frame rectangle for lossless compression.
    rect_ll: FrameRectangle,
    /// Frame rectangle for lossy compression. Could be smaller than `rect_ll` as
    /// pixels with small diffs can be ignored.
    rect_lossy: FrameRectangle,
}

// upstream: src/mux/anim_encode.c SubFrameParamsInit
/// C also initializes the two views (`WebPPictureInit`, which cannot fail for
/// the current version). `SubFrameParamsFree` frees them; the views here are
/// dropped where they are used, so it has no counterpart.
fn sub_frame_params_init(should_try: bool, empty_rect_allowed: bool) -> SubFrameParams {
    SubFrameParams {
        should_try,
        empty_rect_allowed,
        ..SubFrameParams::default()
    }
}

// upstream: src/mux/anim_encode.c GetSubRect
/// Given previous and current canvas, picks the optimal rectangle for the
/// current frame based on `is_lossless` and other parameters. Assumes that the
/// initial guess `rect` is valid. C then makes the view of the rectangle; this
/// checks the rectangle as `WebPPictureView` does and returns the result.
#[expect(
    clippy::too_many_arguments,
    clippy::fn_params_excessive_bools,
    reason = "the C signature without the view"
)]
fn get_sub_rect(
    prev_canvas: &WebPPicture<'_>,
    curr_canvas: &WebPPicture<'_>,
    is_key_frame: bool,
    is_first_frame: bool,
    empty_rect_allowed: bool,
    is_lossless: bool,
    quality: f32,
    rect: &mut FrameRectangle,
) -> bool {
    if !is_key_frame || is_first_frame {
        // Optimize frame rectangle.
        // Note: This behaves as expected for first frame, as 'prev_canvas' is
        // initialized to a fully transparent canvas in the beginning.
        minimize_change_rectangle(prev_canvas, curr_canvas, rect, is_lossless, quality);
    }

    if is_empty_rect(rect) {
        if empty_rect_allowed {
            // No need to get 'sub_frame'.
            return true;
        }
        // Force a 1x1 rectangle.
        rect.width = 1;
        rect.height = 1;
        debug_assert_eq!(rect.x_offset, 0);
        debug_assert_eq!(rect.y_offset, 0);
    }

    snap_to_even_offsets(rect);
    webp_picture_view(
        curr_canvas,
        rect.x_offset,
        rect.y_offset,
        rect.width,
        rect.height,
    )
    .is_some()
}

// upstream: src/mux/anim_encode.c GetSubRects
/// Picks optimal frame rectangle for both lossless and lossy compression. The
/// initial guess for frame rectangles will be the full canvas.
fn get_sub_rects(
    prev_canvas: &WebPPicture<'_>,
    curr_canvas: &WebPPicture<'_>,
    is_key_frame: bool,
    is_first_frame: bool,
    quality: f32,
    params: &mut SubFrameParams,
) -> bool {
    // Lossless frame rectangle.
    params.rect_ll.x_offset = 0;
    params.rect_ll.y_offset = 0;
    params.rect_ll.width = curr_canvas.width;
    params.rect_ll.height = curr_canvas.height;
    if !get_sub_rect(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        params.empty_rect_allowed,
        true,
        quality,
        &mut params.rect_ll,
    ) {
        return false;
    }
    // Lossy frame rectangle.
    params.rect_lossy = params.rect_ll; // seed with lossless rect.
    get_sub_rect(
        prev_canvas,
        curr_canvas,
        is_key_frame,
        is_first_frame,
        params.empty_rect_allowed,
        false,
        quality,
        &mut params.rect_lossy,
    )
}

// upstream: src/mux/anim_encode.c clip
fn clip(v: i32, min_v: i32, max_v: i32) -> i32 {
    if v < min_v {
        min_v
    } else if v > max_v {
        max_v
    } else {
        v
    }
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderRefineRect
/// Picks the optimal rectangle between two pictures, starting with initial
/// values of offsets and dimensions that are passed in. The initial values will
/// be clipped, if necessary, to make sure the rectangle is within the canvas.
/// `use_argb` must be true for both pictures (declared in `animi.h`). Returns
/// true on success; C also returns 0 for `NULL` pictures.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature with its four in/out parameters"
)]
#[must_use]
pub fn webp_anim_encoder_refine_rect(
    prev_canvas: &WebPPicture<'_>,
    curr_canvas: &WebPPicture<'_>,
    is_lossless: i32,
    quality: f32,
    x_offset: &mut i32,
    y_offset: &mut i32,
    width: &mut i32,
    height: &mut i32,
) -> bool {
    if prev_canvas.width != curr_canvas.width
        || prev_canvas.height != curr_canvas.height
        || prev_canvas.use_argb == 0
        || curr_canvas.use_argb == 0
    {
        return false;
    }
    let right = clip(*x_offset + *width, 0, curr_canvas.width);
    let left = clip(*x_offset, 0, curr_canvas.width - 1);
    let bottom = clip(*y_offset + *height, 0, curr_canvas.height);
    let top = clip(*y_offset, 0, curr_canvas.height - 1);
    let mut rect = FrameRectangle {
        x_offset: left,
        y_offset: top,
        ..FrameRectangle::default()
    };
    rect.width = clip(right - left, 0, curr_canvas.width - rect.x_offset);
    rect.height = clip(bottom - top, 0, curr_canvas.height - rect.y_offset);
    minimize_change_rectangle(
        prev_canvas,
        curr_canvas,
        &mut rect,
        is_lossless != 0,
        quality,
    );
    snap_to_even_offsets(&mut rect);
    *x_offset = rect.x_offset;
    *y_offset = rect.y_offset;
    *width = rect.width;
    *height = rect.height;
    true
}

// upstream: src/mux/anim_encode.c DisposeFrameRectangle
fn dispose_frame_rectangle(
    dispose_method: WebPMuxAnimDispose,
    rect: &FrameRectangle,
    curr_canvas: &mut WebPPicture<'_>,
) {
    if dispose_method == WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND {
        webp_util_clear_pic(curr_canvas, Some(rect));
    }
}

// upstream: src/mux/anim_encode.c RectArea
fn rect_area(rect: &FrameRectangle) -> u32 {
    rect.width
        .cast_unsigned()
        .wrapping_mul(rect.height.cast_unsigned())
}

// upstream: src/mux/anim_encode.c IsLosslessBlendingPossible
fn is_lossless_blending_possible(
    src: &WebPPicture<'_>,
    dst: &WebPPicture<'_>,
    rect: &FrameRectangle,
) -> bool {
    let (src_argb, dst_argb) = (argb(src), argb(dst));
    debug_assert!(src.width == dst.width && src.height == dst.height);
    debug_assert!(rect.x_offset + rect.width <= dst.width);
    debug_assert!(rect.y_offset + rect.height <= dst.height);
    for j in rect.y_offset..rect.y_offset + rect.height {
        for i in rect.x_offset..rect.x_offset + rect.width {
            let src_pixel = src_argb[pixel(i, j, src.argb_stride)];
            let dst_pixel = dst_argb[pixel(i, j, dst.argb_stride)];
            let dst_alpha = dst_pixel >> 24;
            if dst_alpha != 0xff && src_pixel != dst_pixel {
                // In this case, if we use blending, we can't attain the desired
                // 'dst_pixel' value for this pixel. So, blending is not possible.
                return false;
            }
        }
    }
    true
}

// upstream: src/mux/anim_encode.c IsLossyBlendingPossible
fn is_lossy_blending_possible(
    src: &WebPPicture<'_>,
    dst: &WebPPicture<'_>,
    rect: &FrameRectangle,
    quality: f32,
) -> bool {
    let max_allowed_diff_lossy = quality_to_max_diff(quality);
    let (src_argb, dst_argb) = (argb(src), argb(dst));
    debug_assert!(src.width == dst.width && src.height == dst.height);
    debug_assert!(rect.x_offset + rect.width <= dst.width);
    debug_assert!(rect.y_offset + rect.height <= dst.height);
    for j in rect.y_offset..rect.y_offset + rect.height {
        for i in rect.x_offset..rect.x_offset + rect.width {
            let src_pixel = src_argb[pixel(i, j, src.argb_stride)];
            let dst_pixel = dst_argb[pixel(i, j, dst.argb_stride)];
            let dst_alpha = dst_pixel >> 24;
            if dst_alpha != 0xff
                && !pixels_are_similar(src_pixel, dst_pixel, max_allowed_diff_lossy)
            {
                // In this case, if we use blending, we can't attain the desired
                // 'dst_pixel' value for this pixel. So, blending is not possible.
                return false;
            }
        }
    }
    true
}

// upstream: src/mux/anim_encode.c IncreaseTransparency
/// For pixels in `rect`, replace those pixels in `dst` that are same as `src`
/// by transparent pixels. Returns true if at least one pixel gets modified.
fn increase_transparency(
    src: &WebPPicture<'_>,
    rect: &FrameRectangle,
    dst: &mut WebPPicture<'_>,
) -> bool {
    let mut modified = false;
    debug_assert!(src.width == dst.width && src.height == dst.height);
    let (src_stride, dst_stride) = (src.argb_stride, dst.argb_stride);
    let psrc = argb(src);
    let pdst = argb_mut(dst);
    for j in rect.y_offset..rect.y_offset + rect.height {
        for i in rect.x_offset..rect.x_offset + rect.width {
            let (s, d) = (pixel(i, j, src_stride), pixel(i, j, dst_stride));
            if psrc[s] == pdst[d] && pdst[d] != TRANSPARENT_COLOR {
                pdst[d] = TRANSPARENT_COLOR;
                modified = true;
            }
        }
    }
    modified
}

// upstream: src/mux/anim_encode.c FlattenSimilarBlocks
/// Replace similar blocks of pixels by a 'see-through' transparent block with
/// uniform average color. Assumes lossy compression is being used. Returns true
/// if at least one pixel gets modified.
fn flatten_similar_blocks(
    src: &WebPPicture<'_>,
    rect: &FrameRectangle,
    dst: &mut WebPPicture<'_>,
    quality: f32,
) -> bool {
    let max_allowed_diff_lossy = quality_to_max_diff(quality);
    let mut modified = false;
    let block_size = 8;
    let y_start = (rect.y_offset + block_size) & !(block_size - 1);
    let y_end = (rect.y_offset + rect.height) & !(block_size - 1);
    let x_start = (rect.x_offset + block_size) & !(block_size - 1);
    let x_end = (rect.x_offset + rect.width) & !(block_size - 1);
    debug_assert!(src.width == dst.width && src.height == dst.height);
    debug_assert_eq!(block_size & (block_size - 1), 0); // must be a power of 2
    let (src_stride, dst_stride) = (src.argb_stride, dst.argb_stride);
    let psrc = argb(src);
    let pdst = argb_mut(dst);
    // Iterate over each block and count similar pixels.
    let mut j = y_start;
    while j < y_end {
        let mut i = x_start;
        while i < x_end {
            let mut cnt = 0;
            let (mut avg_r, mut avg_g, mut avg_b) = (0, 0, 0);
            let src_block = pixel(i, j, src_stride);
            let dst_block = pixel(i, j, dst_stride);
            for y in 0..block_size {
                for x in 0..block_size {
                    let src_pixel = psrc[src_block + pixel(x, y, src_stride)];
                    let alpha = (src_pixel >> 24).cast_signed();
                    if alpha == 0xff
                        && pixels_are_similar(
                            src_pixel,
                            pdst[dst_block + pixel(x, y, dst_stride)],
                            max_allowed_diff_lossy,
                        )
                    {
                        cnt += 1;
                        avg_r += ((src_pixel >> 16) & 0xff).cast_signed();
                        avg_g += ((src_pixel >> 8) & 0xff).cast_signed();
                        avg_b += (src_pixel & 0xff).cast_signed();
                    }
                }
            }
            // If we have a fully similar block, we replace it with an
            // average transparent block. This compresses better in lossy mode.
            if cnt == block_size * block_size {
                // C: (0x00 << 24) | ..., the alpha byte is zero.
                let color =
                    (((avg_r / cnt) << 16) | ((avg_g / cnt) << 8) | (avg_b / cnt)).cast_unsigned();
                for y in 0..block_size {
                    for x in 0..block_size {
                        pdst[dst_block + pixel(x, y, dst_stride)] = color;
                    }
                }
                modified = true;
            }
            i += block_size;
        }
        j += block_size;
    }
    modified
}

// upstream: src/mux/anim_encode.c EncodeFrame
/// Encodes `pic` with `config` into `memory`. C sets `pic->writer` to
/// `WebPMemoryWrite` with `custom_ptr = memory` and leaves them set; the
/// writer here is removed after the call, as it borrows the memory writer.
fn encode_frame(
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
    memory: &mut WebPMemoryWriter,
) -> bool {
    pic.use_argb = 1;
    let shared = Rc::new(RefCell::new(core::mem::take(memory)));
    let writer = Rc::clone(&shared);
    pic.writer = Some(Box::new(move |data: &[u8]| {
        webp_memory_write(&mut writer.borrow_mut(), data)
    }));
    let ok = webp_encode(config, pic);
    pic.writer = None;
    *memory = match Rc::try_unwrap(shared) {
        Ok(cell) => cell.into_inner(),
        Err(_) => unreachable!("the writer that held the other reference is dropped"),
    };
    ok
}

// upstream: src/mux/anim_encode.c Candidate
/// Struct representing a candidate encoded frame including its metadata.
#[derive(Clone, Debug, Default)]
struct Candidate {
    mem: WebPMemoryWriter,
    info: WebPMuxFrameInfo<'static>,
    rect: FrameRectangle,
    /// True if this candidate should be evaluated.
    evaluate: bool,
}

// upstream: src/mux/anim_encode.c EncodeCandidate
/// Generates a candidate encoded frame given a picture and metadata.
fn encode_candidate(
    sub_frame: &mut WebPPicture<'_>,
    rect: &FrameRectangle,
    encoder_config: &WebPConfig,
    use_blending: bool,
    candidate: &mut Candidate,
) -> Result<(), WebPEncodingError> {
    let mut config = *encoder_config;
    *candidate = Candidate::default();

    // Set frame rect and info.
    candidate.rect = *rect;
    candidate.info.id = WebPChunkId::WEBP_CHUNK_ANMF;
    candidate.info.x_offset = rect.x_offset;
    candidate.info.y_offset = rect.y_offset;
    candidate.info.dispose_method = WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE; // Set later.
    candidate.info.blend_method = if use_blending {
        WebPMuxAnimBlend::WEBP_MUX_BLEND
    } else {
        WebPMuxAnimBlend::WEBP_MUX_NO_BLEND
    };
    candidate.info.duration = 0; // Set in next call to WebPAnimEncoderAdd().

    // Encode picture.
    webp_memory_writer_init(&mut candidate.mem);

    if config.lossless == 0 && use_blending {
        // Disable filtering to avoid blockiness in reconstructed frames at the
        // time of decoding.
        config.autofilter = 0;
        config.filter_strength = 0;
    }
    if !encode_frame(&config, sub_frame, &mut candidate.mem) {
        let error_code = sub_frame.error_code;
        webp_memory_writer_clear(&mut candidate.mem);
        return Err(error_code);
    }

    candidate.evaluate = true;
    Ok(())
}

// upstream: src/mux/anim_encode.c CopyCurrentCanvas
/// Copies the frame into the canvas copy when the copy was modified. C also
/// copies the frame's `progress_hook` and `user_data` into the copy; the views
/// borrow the frame's hook where they are encoded instead.
fn copy_current_canvas(enc: &mut WebPAnimEncoder<'_>, curr_canvas: &WebPPicture<'_>) {
    if enc.curr_canvas_copy_modified {
        webp_copy_pixels(curr_canvas, &mut enc.curr_canvas_copy);
        enc.curr_canvas_copy_modified = false;
    }
}

// upstream: src/mux/anim_encode.c LL_DISP_NONE
const LL_DISP_NONE: usize = 0;
// upstream: src/mux/anim_encode.c LL_DISP_BG
const LL_DISP_BG: usize = 1;
// upstream: src/mux/anim_encode.c LOSSY_DISP_NONE
const LOSSY_DISP_NONE: usize = 2;
// upstream: src/mux/anim_encode.c LOSSY_DISP_BG
const LOSSY_DISP_BG: usize = 3;
// upstream: src/mux/anim_encode.c CANDIDATE_COUNT
const CANDIDATE_COUNT: usize = 4;

// upstream: src/mux/anim_encode.c MIN_COLORS_LOSSY
const MIN_COLORS_LOSSY: i32 = 31; // Don't try lossy below this threshold.
// upstream: src/mux/anim_encode.c MAX_COLORS_LOSSLESS
const MAX_COLORS_LOSSLESS: i32 = 194; // Don't try lossless above this threshold.

/// C's view of the `rect` rectangle of `canvas` (`WebPPictureView` in
/// `GetSubRect`), for an encoding: the picture borrows the canvas's ARGB
/// samples exclusively, so that `WebPEncode` writes into the canvas where C
/// writes through the view, and it reports progress through `progress_hook`,
/// the frame's hook that C copies into the view. Like a view, it has the
/// canvas's specifications, no writer and no statistics. No C counterpart.
fn sub_frame_view<'v>(
    canvas: &'v mut WebPPicture<'static>,
    rect: &FrameRectangle,
    progress_hook: Option<&'v mut crate::webp::encode::WebPProgressHook<'_>>,
) -> WebPPicture<'v> {
    // The canvas copy is an ARGB picture: C's view of a YUV picture, after an
    // encoding of WebPAnimEncoderAssemble left the copy in YUV, would need
    // another WebPAnimEncoderAdd after WebPAnimEncoderAssemble, which
    // docs/status/wp15.md excludes.
    debug_assert!(canvas.use_argb != 0);
    let plane = canvas.argb.expect("the canvas has an ARGB plane");
    let offset = plane.offset + pixel(rect.x_offset, rect.y_offset, canvas.argb_stride);
    let mut view = WebPPicture {
        use_argb: canvas.use_argb,
        colorspace: canvas.colorspace,
        width: rect.width,
        height: rect.height,
        pad1: canvas.pad1,
        argb: Some(PlaneOffset {
            region: 0,
            offset: 0,
        }),
        argb_stride: canvas.argb_stride,
        pad2: canvas.pad2,
        extra_info_type: canvas.extra_info_type,
        error_code: canvas.error_code,
        pad3: canvas.pad3,
        pad6: canvas.pad6,
        ..WebPPicture::default()
    };
    let region = canvas
        .argb_storage
        .region_mut(plane.region)
        .expect("the canvas's ARGB region exists");
    view.argb_storage = PlaneStorage::External(vec![&mut region[offset..]]);
    if let Some(hook) = progress_hook {
        view.progress_hook = Some(Box::new(move |percent: i32| hook(percent)));
    }
    view
}

// upstream: src/mux/anim_encode.c GenerateCandidates
/// Generates candidates for a given dispose method given pre-filled sub-frame
/// `params`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature with the caller's frame"
)]
fn generate_candidates(
    enc: &mut WebPAnimEncoder<'_>,
    curr_frame: &mut WebPPicture<'_>,
    candidates: &mut [Candidate; CANDIDATE_COUNT],
    dispose_method: WebPMuxAnimDispose,
    is_lossless: bool,
    is_key_frame: bool,
    params: &SubFrameParams,
    config_ll: &WebPConfig,
    config_lossy: &WebPConfig,
) -> Result<(), WebPEncodingError> {
    let is_dispose_none = dispose_method == WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE;
    let candidate_ll = if is_dispose_none {
        LL_DISP_NONE
    } else {
        LL_DISP_BG
    };
    let candidate_lossy = if is_dispose_none {
        LOSSY_DISP_NONE
    } else {
        LOSSY_DISP_BG
    };

    copy_current_canvas(enc, curr_frame);
    // C: curr_canvas = &enc->curr_canvas_copy, prev_canvas = the previous canvas
    // of the dispose method; borrowed field by field below.
    let prev_canvas = if is_dispose_none {
        &enc.prev_canvas
    } else {
        &enc.prev_canvas_disposed
    };
    let use_blending_ll = !is_key_frame
        && is_lossless_blending_possible(prev_canvas, &enc.curr_canvas_copy, &params.rect_ll);
    let use_blending_lossy = !is_key_frame
        && is_lossy_blending_possible(
            prev_canvas,
            &enc.curr_canvas_copy,
            &params.rect_lossy,
            config_lossy.quality,
        );

    // Pick candidates to be tried.
    let (evaluate_ll, evaluate_lossy) = if enc.options.allow_mixed == 0 {
        (is_lossless, !is_lossless)
    } else if enc.options.minimize_size != 0 {
        (true, true)
    } else {
        // Use a heuristic for trying lossless and/or lossy compression.
        let rect = &params.rect_ll;
        let sub_frame_ll = webp_picture_view(
            &enc.curr_canvas_copy,
            rect.x_offset,
            rect.y_offset,
            rect.width,
            rect.height,
        )
        .expect("GetSubRect checked the rectangle of the view");
        let num_colors = webp_get_color_palette(&sub_frame_ll, None);
        (
            num_colors < MAX_COLORS_LOSSLESS,
            num_colors >= MIN_COLORS_LOSSY,
        )
    };

    // Generate candidates.
    if evaluate_ll {
        copy_current_canvas(enc, curr_frame);
        if use_blending_ll {
            let prev_canvas = if is_dispose_none {
                &enc.prev_canvas
            } else {
                &enc.prev_canvas_disposed
            };
            enc.curr_canvas_copy_modified =
                increase_transparency(prev_canvas, &params.rect_ll, &mut enc.curr_canvas_copy);
        }
        let mut sub_frame_ll = sub_frame_view(
            &mut enc.curr_canvas_copy,
            &params.rect_ll,
            curr_frame.progress_hook.as_mut(),
        );
        encode_candidate(
            &mut sub_frame_ll,
            &params.rect_ll,
            config_ll,
            use_blending_ll,
            &mut candidates[candidate_ll],
        )?;
    }
    if evaluate_lossy {
        copy_current_canvas(enc, curr_frame);
        if use_blending_lossy {
            let prev_canvas = if is_dispose_none {
                &enc.prev_canvas
            } else {
                &enc.prev_canvas_disposed
            };
            enc.curr_canvas_copy_modified = flatten_similar_blocks(
                prev_canvas,
                &params.rect_lossy,
                &mut enc.curr_canvas_copy,
                config_lossy.quality,
            );
        }
        let mut sub_frame_lossy = sub_frame_view(
            &mut enc.curr_canvas_copy,
            &params.rect_lossy,
            curr_frame.progress_hook.as_mut(),
        );
        encode_candidate(
            &mut sub_frame_lossy,
            &params.rect_lossy,
            config_lossy,
            use_blending_lossy,
            &mut candidates[candidate_lossy],
        )?;
        enc.curr_canvas_copy_modified = true;
    }
    Ok(())
}

// upstream: src/mux/anim_encode.c GetEncodedData
/// Moves the written bytes of `memory` into `encoded_data` (C points
/// `encoded_data` at the writer's buffer, which it then owns).
fn get_encoded_data(memory: &mut WebPMemoryWriter, encoded_data: &mut WebPData<'static>) {
    let mut bytes = core::mem::take(&mut memory.mem);
    bytes.truncate(memory.size);
    encoded_data.bytes = Cow::Owned(bytes);
}

// upstream: src/mux/anim_encode.c SetPreviousDisposeMethod
/// Sets dispose method of the previous frame to be `dispose_method`.
fn set_previous_dispose_method(enc: &mut WebPAnimEncoder<'_>, dispose_method: WebPMuxAnimDispose) {
    let position = enc.count.wrapping_sub(2);
    let prev = get_frame(enc, position);
    debug_assert!(enc.count >= 2); // As current and previous frames are in enc.

    let undecided = enc.prev_candidate_undecided;
    let prev_enc_frame = &mut enc.encoded_frames[prev];
    if undecided {
        debug_assert_eq!(dispose_method, WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE);
        prev_enc_frame.sub_frame.dispose_method = dispose_method;
        prev_enc_frame.key_frame.dispose_method = dispose_method;
    } else {
        let prev_info = if prev_enc_frame.is_key_frame {
            &mut prev_enc_frame.key_frame
        } else {
            &mut prev_enc_frame.sub_frame
        };
        prev_info.dispose_method = dispose_method;
    }
}

// upstream: src/mux/anim_encode.c IncreasePreviousDuration
fn increase_previous_duration(enc: &mut WebPAnimEncoder<'_>, duration: i32) -> bool {
    const LOSSLESS_1X1_BYTES: [u8; 28] = [
        0x52, 0x49, 0x46, 0x46, 0x14, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38,
        0x4c, 0x08, 0x00, 0x00, 0x00, 0x2f, 0x00, 0x00, 0x00, 0x10, 0x88, 0x88, 0x08,
    ];
    const LOSSY_1X1_BYTES: [u8; 72] = [
        0x52, 0x49, 0x46, 0x46, 0x40, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50, 0x56, 0x50, 0x38,
        0x58, 0x0a, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x41, 0x4c, 0x50, 0x48, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x56, 0x50, 0x38, 0x20, 0x18,
        0x00, 0x00, 0x00, 0x30, 0x01, 0x00, 0x9d, 0x01, 0x2a, 0x01, 0x00, 0x01, 0x00, 0x02, 0x00,
        0x34, 0x25, 0xa4, 0x00, 0x03, 0x70, 0x00, 0xfe, 0xfb, 0xfd, 0x50, 0x00,
    ];
    let position = enc.count.wrapping_sub(1);
    let prev = get_frame(enc, position);

    debug_assert!(enc.count >= 1);
    debug_assert!(
        !enc.encoded_frames[prev].is_key_frame
            || enc.encoded_frames[prev].sub_frame.duration
                == enc.encoded_frames[prev].key_frame.duration
    );
    debug_assert_eq!(
        enc.encoded_frames[prev].sub_frame.duration,
        enc.encoded_frames[prev].sub_frame.duration & (MAX_DURATION - 1)
    );
    debug_assert_eq!(duration, duration & (MAX_DURATION - 1));

    let new_duration = enc.encoded_frames[prev].sub_frame.duration + duration;
    if new_duration >= MAX_DURATION {
        // Special case.
        // Separate out previous frame from earlier merged frames to avoid
        // overflow. We add a 1x1 transparent frame for the previous frame, with
        // blending on.
        let rect = FrameRectangle {
            x_offset: 0,
            y_offset: 0,
            width: 1,
            height: 1,
        };
        let lossless_1x1 = WebPData {
            bytes: Cow::Borrowed(&LOSSLESS_1X1_BYTES),
        };
        let lossy_1x1 = WebPData {
            bytes: Cow::Borrowed(&LOSSY_1X1_BYTES),
        };
        let can_use_lossless = enc.last_config.lossless != 0 || enc.options.allow_mixed != 0;
        let curr = get_frame(enc, enc.count);
        let curr_enc_frame = &mut enc.encoded_frames[curr];
        curr_enc_frame.is_key_frame = false;
        curr_enc_frame.sub_frame.id = WebPChunkId::WEBP_CHUNK_ANMF;
        curr_enc_frame.sub_frame.x_offset = 0;
        curr_enc_frame.sub_frame.y_offset = 0;
        curr_enc_frame.sub_frame.dispose_method = WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE;
        curr_enc_frame.sub_frame.blend_method = WebPMuxAnimBlend::WEBP_MUX_BLEND;
        curr_enc_frame.sub_frame.duration = duration;
        let source = if can_use_lossless {
            &lossless_1x1
        } else {
            &lossy_1x1
        };
        let Some(copy) = webp_data_copy(source) else {
            curr_enc_frame.sub_frame.bitstream = WebPData::default();
            return false;
        };
        curr_enc_frame.sub_frame.bitstream = copy;
        enc.count += 1;
        enc.count_since_key_frame += 1;
        enc.flush_count = enc.count - 1;
        enc.prev_candidate_undecided = false;
        enc.prev_rect = rect;
    } else {
        // Regular case.
        // Increase duration of the previous frame by 'duration'.
        enc.encoded_frames[prev].sub_frame.duration = new_duration;
        enc.encoded_frames[prev].key_frame.duration = new_duration;
    }
    true
}

// upstream: src/mux/anim_encode.c PickBestCandidate
/// Pick the candidate encoded frame with smallest size and release other
/// candidates.
fn pick_best_candidate(
    enc: &mut WebPAnimEncoder<'_>,
    candidates: &mut [Candidate; CANDIDATE_COUNT],
    is_key_frame: bool,
    encoded_frame: usize,
) {
    let mut best_idx: Option<usize> = None;
    let mut best_size = usize::MAX;
    for (i, candidate) in candidates.iter().enumerate() {
        if candidate.evaluate {
            let candidate_size = candidate.mem.size;
            if candidate_size < best_size {
                best_idx = Some(i);
                best_size = candidate_size;
            }
        }
    }
    debug_assert!(best_idx.is_some());
    for (i, candidate) in candidates.iter_mut().enumerate() {
        if candidate.evaluate {
            if Some(i) == best_idx {
                let frame = &mut enc.encoded_frames[encoded_frame];
                let dst = if is_key_frame {
                    &mut frame.key_frame
                } else {
                    &mut frame.sub_frame
                };
                *dst = candidate.info.clone();
                get_encoded_data(&mut candidate.mem, &mut dst.bitstream);
                if !is_key_frame {
                    // Note: Previous dispose method only matters for non-keyframes.
                    // Also, we don't want to modify previous dispose method that was
                    // selected when a non key-frame was assumed.
                    let prev_dispose_method = if i == LL_DISP_NONE || i == LOSSY_DISP_NONE {
                        WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE
                    } else {
                        WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND
                    };
                    set_previous_dispose_method(enc, prev_dispose_method);
                }
                enc.prev_rect = candidate.rect; // save for next frame.
            } else {
                webp_memory_writer_clear(&mut candidate.mem);
                candidate.evaluate = false;
            }
        }
    }
}

// upstream: src/mux/anim_encode.c SetFrame
/// Depending on the configuration, tries different compressions
/// (lossy/lossless), dispose methods, blending methods etc to encode the
/// current frame and outputs the best one in `encoded_frame`. `frame_skipped`
/// will be set to true if this frame should actually be skipped.
#[expect(clippy::too_many_lines, reason = "follows the C function")]
fn set_frame(
    enc: &mut WebPAnimEncoder<'_>,
    curr_frame: &mut WebPPicture<'_>,
    config: &WebPConfig,
    is_key_frame: bool,
    encoded_frame: usize,
    frame_skipped: &mut bool,
) -> Result<(), WebPEncodingError> {
    let mut candidates: [Candidate; CANDIDATE_COUNT] = Default::default();
    let is_lossless = config.lossless != 0;
    let consider_lossless = is_lossless || enc.options.allow_mixed != 0;
    let consider_lossy = !is_lossless || enc.options.allow_mixed != 0;
    let is_first_frame = enc.is_first_frame;

    // First frame cannot be skipped as there is no 'previous frame' to merge it
    // to. So, empty rectangle is not allowed for the first frame.
    let empty_rect_allowed_none = !is_first_frame;

    // Even if there is exact pixel match between 'disposed previous canvas' and
    // 'current canvas', we can't skip current frame, as there may not be exact
    // pixel match between 'previous canvas' and 'current canvas'. So, we don't
    // allow empty rectangle in this case.
    let empty_rect_allowed_bg = false;

    // If current frame is a key-frame, dispose method of previous frame doesn't
    // matter, so we don't try dispose to background.
    // Also, if key-frame insertion is on, and previous frame could be picked as
    // either a sub-frame or a key-frame, then we can't be sure about what frame
    // rectangle would be disposed. In that case too, we don't try dispose to
    // background.
    let dispose_bg_possible = !is_key_frame && !enc.prev_candidate_undecided;

    let mut config_ll = *config;
    let mut config_lossy = *config;
    config_ll.lossless = 1;
    config_lossy.lossless = 0;
    enc.last_config = *config;
    enc.last_config_reversed = if config.lossless != 0 {
        config_lossy
    } else {
        config_ll
    };
    *frame_skipped = false;

    let mut dispose_none_params = sub_frame_params_init(true, empty_rect_allowed_none);
    let mut dispose_bg_params = sub_frame_params_init(false, empty_rect_allowed_bg);

    let result = (|| {
        // Change-rectangle assuming previous frame was DISPOSE_NONE.
        if !get_sub_rects(
            &enc.prev_canvas,
            &enc.curr_canvas_copy,
            is_key_frame,
            is_first_frame,
            config_lossy.quality,
            &mut dispose_none_params,
        ) {
            return Err(WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
        }

        if (consider_lossless && is_empty_rect(&dispose_none_params.rect_ll))
            || (consider_lossy && is_empty_rect(&dispose_none_params.rect_lossy))
        {
            // Don't encode the frame at all. Instead, the duration of the previous
            // frame will be increased later.
            debug_assert!(empty_rect_allowed_none);
            *frame_skipped = true;
            return Ok(());
        }

        if dispose_bg_possible {
            // Change-rectangle assuming previous frame was DISPOSE_BACKGROUND.
            webp_copy_pixels(&enc.prev_canvas, &mut enc.prev_canvas_disposed);
            dispose_frame_rectangle(
                WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND,
                &enc.prev_rect,
                &mut enc.prev_canvas_disposed,
            );

            if !get_sub_rects(
                &enc.prev_canvas_disposed,
                &enc.curr_canvas_copy,
                is_key_frame,
                is_first_frame,
                config_lossy.quality,
                &mut dispose_bg_params,
            ) {
                return Err(WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION);
            }
            debug_assert!(!is_empty_rect(&dispose_bg_params.rect_ll));
            debug_assert!(!is_empty_rect(&dispose_bg_params.rect_lossy));

            if enc.options.minimize_size != 0 {
                // Try both dispose methods.
                dispose_bg_params.should_try = true;
                dispose_none_params.should_try = true;
            } else if (is_lossless
                && rect_area(&dispose_bg_params.rect_ll) < rect_area(&dispose_none_params.rect_ll))
                || (!is_lossless
                    && rect_area(&dispose_bg_params.rect_lossy)
                        < rect_area(&dispose_none_params.rect_lossy))
            {
                dispose_bg_params.should_try = true; // Pick DISPOSE_BACKGROUND.
                dispose_none_params.should_try = false;
            }
        }

        if dispose_none_params.should_try {
            generate_candidates(
                enc,
                curr_frame,
                &mut candidates,
                WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE,
                is_lossless,
                is_key_frame,
                &dispose_none_params,
                &config_ll,
                &config_lossy,
            )?;
        }

        if dispose_bg_params.should_try {
            debug_assert!(!enc.is_first_frame);
            debug_assert!(dispose_bg_possible);
            generate_candidates(
                enc,
                curr_frame,
                &mut candidates,
                WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND,
                is_lossless,
                is_key_frame,
                &dispose_bg_params,
                &config_ll,
                &config_lossy,
            )?;
        }

        pick_best_candidate(enc, &mut candidates, is_key_frame, encoded_frame);
        Ok(())
    })();

    if result.is_err() {
        // Err:
        for candidate in &mut candidates {
            if candidate.evaluate {
                webp_memory_writer_clear(&mut candidate.mem);
            }
        }
    }
    // End: C calls SubFrameParamsFree on both parameter sets.
    result
}

// upstream: src/mux/anim_encode.c KeyFramePenalty
/// Calculate the penalty incurred if we encode given frame as a key frame
/// instead of a sub-frame.
fn key_frame_penalty(encoded_frame: &EncodedFrame) -> i64 {
    let size = |data: &WebPData<'_>| i64::try_from(data.bytes.len()).expect("a size below 2^63");
    size(&encoded_frame.key_frame.bitstream) - size(&encoded_frame.sub_frame.bitstream)
}

// upstream: src/mux/anim_encode.c CacheFrame
#[expect(clippy::too_many_lines, reason = "follows the C function")]
fn cache_frame(
    enc: &mut WebPAnimEncoder<'_>,
    curr_frame: &mut WebPPicture<'_>,
    config: &WebPConfig,
) -> bool {
    let mut ok = false;
    let mut frame_skipped = false;
    let mut error_code = WebPEncodingError::VP8_ENC_OK;
    let position = enc.count;
    let encoded_frame = get_frame(enc, position);

    enc.count += 1;

    'end: {
        if enc.is_first_frame {
            // Add this as a key-frame.
            if let Err(err) = set_frame(
                enc,
                curr_frame,
                config,
                true,
                encoded_frame,
                &mut frame_skipped,
            ) {
                error_code = err;
                break 'end;
            }
            debug_assert!(!frame_skipped); // First frame can't be skipped, even if empty.
            debug_assert!(position == 0 && enc.count == 1);
            enc.encoded_frames[encoded_frame].is_key_frame = true;
            enc.flush_count = 0;
            enc.count_since_key_frame = 0;
            enc.prev_candidate_undecided = false;
        } else {
            enc.count_since_key_frame += 1;
            if enc.count_since_key_frame <= enc.options.kmin {
                // Add this as a frame rectangle.
                if let Err(err) = set_frame(
                    enc,
                    curr_frame,
                    config,
                    false,
                    encoded_frame,
                    &mut frame_skipped,
                ) {
                    error_code = err;
                    break 'end;
                }
                if !frame_skipped {
                    enc.encoded_frames[encoded_frame].is_key_frame = false;
                    enc.flush_count = enc.count - 1;
                    enc.prev_candidate_undecided = false;
                }
            } else {
                // Add this as a frame rectangle to enc.
                if let Err(err) = set_frame(
                    enc,
                    curr_frame,
                    config,
                    false,
                    encoded_frame,
                    &mut frame_skipped,
                ) {
                    error_code = err;
                    break 'end;
                }
                if !frame_skipped {
                    let prev_rect_sub = enc.prev_rect;

                    // Add this as a key-frame to enc, too.
                    if let Err(err) = set_frame(
                        enc,
                        curr_frame,
                        config,
                        true,
                        encoded_frame,
                        &mut frame_skipped,
                    ) {
                        error_code = err;
                        break 'end;
                    }
                    debug_assert!(!frame_skipped); // Key-frame cannot be an empty rectangle.
                    let prev_rect_key = enc.prev_rect;

                    // Analyze size difference of the two variants.
                    let curr_delta = key_frame_penalty(&enc.encoded_frames[encoded_frame]);
                    if curr_delta <= enc.best_delta {
                        // Pick this as the key-frame.
                        if enc.keyframe != KEYFRAME_NONE {
                            let old_keyframe = get_frame(
                                enc,
                                usize::try_from(enc.keyframe).expect("a frame position"),
                            );
                            debug_assert!(enc.encoded_frames[old_keyframe].is_key_frame);
                            enc.encoded_frames[old_keyframe].is_key_frame = false;
                        }
                        enc.encoded_frames[encoded_frame].is_key_frame = true;
                        enc.prev_candidate_undecided = true;
                        enc.keyframe = i32::try_from(position).expect("at most size frames");
                        enc.best_delta = curr_delta;
                        enc.flush_count = enc.count - 1; // We can flush previous frames.
                    } else {
                        enc.encoded_frames[encoded_frame].is_key_frame = false;
                        enc.prev_candidate_undecided = false;
                    }
                    // Note: We need '>=' below because when kmin and kmax are both
                    // zero, count_since_key_frame will always be > kmax.
                    if enc.count_since_key_frame >= enc.options.kmax {
                        enc.flush_count = enc.count - 1;
                        enc.count_since_key_frame = 0;
                        enc.keyframe = KEYFRAME_NONE;
                        enc.best_delta = DELTA_INFINITY;
                    }
                    if !enc.prev_candidate_undecided {
                        enc.prev_rect = if enc.encoded_frames[encoded_frame].is_key_frame {
                            prev_rect_key
                        } else {
                            prev_rect_sub
                        };
                    }
                }
            }
        }

        if !frame_skipped {
            // Update previous to previous and previous canvases for next call.
            webp_copy_pixels(curr_frame, &mut enc.prev_canvas);
            enc.is_first_frame = false;
        }

        // Skip:
        ok = true;
        enc.in_frame_count += 1;
    }

    // End:
    if !ok || frame_skipped {
        frame_release(&mut enc.encoded_frames[encoded_frame]);
        // We reset some counters, as the frame addition failed/was skipped.
        enc.count -= 1;
        if !enc.is_first_frame {
            enc.count_since_key_frame -= 1;
        }
        if !ok {
            mark_error2(
                enc,
                "ERROR adding frame. WebPEncodingError",
                error_code.0.cast_signed(),
            );
        }
    }
    curr_frame.error_code = error_code; // report error_code
    debug_assert!(ok || error_code != WebPEncodingError::VP8_ENC_OK);
    ok
}

// upstream: src/mux/anim_encode.c FlushFrames
fn flush_frames(enc: &mut WebPAnimEncoder<'_>) -> bool {
    while enc.flush_count > 0 {
        let curr = get_frame(enc, 0);
        let frame = &enc.encoded_frames[curr];
        let info = if frame.is_key_frame {
            &frame.key_frame
        } else {
            &frame.sub_frame
        };
        if let Err(err) = webp_mux_push_frame(&mut enc.mux, info, 1) {
            mark_error2(enc, "ERROR adding frame. WebPMuxError", err.0);
            return false;
        }
        if enc.options.verbose != 0 {
            print_stderr(format_args!(
                "INFO: Added frame. offset:{},{} dispose:{} blend:{}\n",
                info.x_offset, info.y_offset, info.dispose_method.0, info.blend_method.0
            ));
        }
        enc.out_frame_count += 1;
        frame_release(&mut enc.encoded_frames[curr]);
        enc.start += 1;
        enc.flush_count -= 1;
        enc.count -= 1;
        if enc.keyframe != KEYFRAME_NONE {
            enc.keyframe -= 1;
        }
    }

    if enc.count == 1 && enc.start != 0 {
        // Move enc->start to index 0.
        let enc_start_tmp = enc.start;
        enc.encoded_frames.swap(0, enc_start_tmp);
        frame_release(&mut enc.encoded_frames[enc_start_tmp]);
        enc.start = 0;
    }
    true
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderAdd
/// Optimizes the given frame for WebP, encodes it and adds it to the encoder.
/// The last call should be with `frame = None`, which indicates that no more
/// frames are to be added and determines the duration of the last frame.
/// Timestamps must not decrease. A YUV(A) frame is converted to ARGB in place,
/// which incurs a small loss. `None` configuration picks lossless defaults.
/// Returns false on error, with `frame.error_code` set (C also returns 0 for a
/// `NULL` encoder).
#[must_use]
pub fn webp_anim_encoder_add(
    enc: &mut WebPAnimEncoder<'_>,
    frame: Option<&mut WebPPicture<'_>>,
    timestamp: i32,
    encoder_config: Option<&WebPConfig>,
) -> bool {
    let mut frame = frame;
    mark_no_error(enc);

    if enc.is_first_frame {
        enc.first_timestamp = timestamp;
    } else {
        // Make sure timestamps are non-decreasing (integer wrap-around is OK).
        let prev_frame_duration = timestamp
            .cast_unsigned()
            .wrapping_sub(enc.prev_timestamp.cast_unsigned());
        if prev_frame_duration >= MAX_DURATION.cast_unsigned() {
            if let Some(frame) = frame.as_deref_mut() {
                frame.error_code = WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION;
            }
            mark_error(enc, "ERROR adding frame: timestamps must be non-decreasing");
            return false;
        }
        if !increase_previous_duration(enc, prev_frame_duration.cast_signed()) {
            return false;
        }
        // IncreasePreviousDuration() may add a frame to avoid exceeding
        // MAX_DURATION which could cause CacheFrame() to over read
        // 'encoded_frames' before the next flush.
        if enc.count == enc.size && !flush_frames(enc) {
            return false;
        }
    }

    let Some(frame) = frame else {
        // Special: last call.
        enc.got_null_frame = true;
        enc.prev_timestamp = timestamp;
        return true;
    };

    if frame.width != enc.canvas_width || frame.height != enc.canvas_height {
        frame.error_code = WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION;
        mark_error(enc, "ERROR adding frame: Invalid frame dimensions");
        return false;
    }

    if frame.use_argb == 0 {
        // Convert frame from YUV(A) to ARGB.
        if enc.options.verbose != 0 {
            print_stderr(format_args!(
                "WARNING: Converting frame from YUV(A) to ARGB format; this incurs a small loss.\n"
            ));
        }
        if webp_picture_yuva_to_argb(frame).is_err() {
            mark_error(enc, "ERROR converting frame from YUV(A) to ARGB");
            return false;
        }
    }

    let mut config = WebPConfig::default();
    if let Some(encoder_config) = encoder_config {
        if !webp_validate_config(encoder_config) {
            mark_error(enc, "ERROR adding frame: Invalid WebPConfig");
            return false;
        }
        config = *encoder_config;
    } else {
        if !webp_config_init(&mut config) {
            mark_error(enc, "Cannot Init config");
            return false;
        }
        config.lossless = 1;
    }
    // C: enc->curr_canvas = frame, the argument of the calls below.
    debug_assert!(enc.curr_canvas_copy_modified);
    copy_current_canvas(enc, frame);

    let ok = cache_frame(enc, frame, &config) && flush_frames(enc);

    enc.curr_canvas_copy_modified = true;
    if ok {
        enc.prev_timestamp = timestamp;
    }
    ok
}

// -----------------------------------------------------------------------------
// Bitstream assembly.

// upstream: src/mux/anim_encode.c DecodeFrameOntoCanvas
/// Clears `canvas` and decodes the frame onto it at the frame's offsets. C
/// decodes into a view of the canvas (`MODE_BGRA`, whose bytes are the ARGB
/// words in native byte order); the translation decodes into a buffer with the
/// view's stride and size and copies its rows into the canvas, also after a
/// failed decoding, which leaves the canvas as C leaves it.
fn decode_frame_onto_canvas(frame: &WebPMuxFrameInfo<'_>, canvas: &mut WebPPicture<'_>) -> bool {
    let image = &frame.bitstream;
    let mut config = crate::webp::decode::WebPDecoderConfig::default();
    if !webp_init_decoder_config(&mut config) {
        return false;
    }
    webp_util_clear_pic(canvas, None);
    if webp_get_features(&image.bytes, &mut config.input).is_err() {
        return false;
    }
    let (view_offset, view_stride, view_width, view_height) = {
        let Some(sub_image) = webp_picture_view(
            canvas,
            frame.x_offset,
            frame.y_offset,
            config.input.width,
            config.input.height,
        ) else {
            return false;
        };
        let plane = sub_image.argb.expect("a view of an ARGB picture");
        (
            plane.offset,
            sub_image.argb_stride,
            sub_image.width,
            sub_image.height,
        )
    };
    let stride = view_stride * 4;
    let size = usize::try_from(stride * view_height).expect("a view inside the canvas");
    let mut rows = vec![0_u8; size];
    config.output.colorspace = WEBP_CSP_MODE::MODE_BGRA;
    config.output.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
        rgba: PlaneOffset::default(),
        stride,
        size,
    });
    config.output.storage = PlaneStorage::External(vec![&mut rows]);

    let ok = webp_decode(&image.bytes, &mut config).is_ok();
    drop(config);
    let canvas_offset = canvas.argb.expect("the canvas has an ARGB plane").offset;
    let samples = argb_mut(canvas);
    for y in 0..view_height {
        let row = pixel(0, y, stride);
        let dst = view_offset - canvas_offset + pixel(0, y, view_stride);
        for x in 0..usize::try_from(view_width).expect("a positive width") {
            let at = row + 4 * x;
            samples[dst + x] =
                u32::from_ne_bytes([rows[at], rows[at + 1], rows[at + 2], rows[at + 3]]);
        }
    }
    ok
}

// upstream: src/mux/anim_encode.c FrameToFullCanvas
/// Decodes `frame` onto the canvas copy and re-encodes the whole canvas with the
/// last configuration (and its reverse, if mixed compression is allowed, keeping
/// the smaller). Returns the encoded image, or `None` where C returns 0.
fn frame_to_full_canvas(
    enc: &mut WebPAnimEncoder<'_>,
    frame: &WebPMuxFrameInfo<'_>,
) -> Option<WebPData<'static>> {
    let mut mem1 = WebPMemoryWriter::default();
    let mut mem2 = WebPMemoryWriter::default();
    webp_memory_writer_init(&mut mem1);
    webp_memory_writer_init(&mut mem2);
    let mut full_image = WebPData::default();

    let ok = (|| {
        let canvas_buf = &mut enc.curr_canvas_copy;
        if !decode_frame_onto_canvas(frame, canvas_buf) {
            return false;
        }
        if !encode_frame(&enc.last_config, canvas_buf, &mut mem1) {
            return false;
        }
        get_encoded_data(&mut mem1, &mut full_image);

        if enc.options.allow_mixed != 0 {
            if !encode_frame(
                &enc.last_config_reversed,
                &mut enc.curr_canvas_copy,
                &mut mem2,
            ) {
                return false;
            }
            if mem2.size < full_image.bytes.len() {
                get_encoded_data(&mut mem2, &mut full_image);
                webp_memory_writer_clear(&mut mem1);
            } else {
                webp_memory_writer_clear(&mut mem2);
            }
        }
        true
    })();
    if !ok {
        // Err: C also frees the bytes of 'full_image' here, which its caller
        // frees again when the second encoding fails (docs/status/wp15.md).
        webp_memory_writer_clear(&mut mem1);
        webp_memory_writer_clear(&mut mem2);
        return None;
    }
    Some(full_image)
}

// upstream: src/mux/anim_encode.c OptimizeSingleFrame
/// Convert a single-frame animation to a non-animated image if appropriate.
/// C creates the temporary mux over `webp_data` and replaces `webp_data` before
/// it deletes the mux; the mux borrows `webp_data` here, so it is dropped first.
fn optimize_single_frame(
    enc: &mut WebPAnimEncoder<'_>,
    webp_data: &mut WebPData<'static>,
) -> Result<(), WebPMuxError> {
    let mut replacement: Option<WebPData<'static>> = None;
    let result = {
        let input = WebPData {
            bytes: Cow::Borrowed(&webp_data.bytes[..]),
        };
        let Some(mut mux) = webp_mux_create(&input, 0) else {
            return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
        };
        debug_assert_eq!(enc.out_frame_count, 1);

        let result = (|| {
            let frame = webp_mux_get_frame(&mux, 1)?;
            if frame.id != WebPChunkId::WEBP_CHUNK_ANMF {
                return Ok(()); // Non-animation: nothing to do.
            }
            let (_canvas_width, _canvas_height) = webp_mux_get_canvas_size(&mux)?;
            let Some(full_image) = frame_to_full_canvas(enc, &frame) else {
                return Err(WebPMuxError::WEBP_MUX_BAD_DATA);
            };
            webp_mux_set_image(&mut mux, &full_image, 1)?;
            let webp_data2 = webp_mux_assemble(&mut mux)?;

            if webp_data2.bytes.len() < input.bytes.len() {
                // Pick 'webp_data2' if smaller.
                replacement = Some(webp_data2);
            }
            Ok(())
        })();
        webp_mux_delete(mux);
        result
    };
    if let Some(webp_data2) = replacement {
        webp_data_clear(webp_data);
        *webp_data = webp_data2;
    }
    result
}

/// C's `(int)(delta_time / (enc->in_frame_count - 1))`: the frame count is
/// converted to `double` and the quotient truncated to `int`.
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "C's conversions of size_t to double and of double to int"
)]
fn average(delta_time: f64, frames: usize) -> i32 {
    (delta_time / frames as f64) as i32
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderAssemble
/// Assembles all frames added so far into a WebP bitstream in `webp_data`. This
/// call should be preceded by a call to `WebPAnimEncoderAdd` with `frame =
/// None`; if not, the duration of the last frame will be internally estimated.
/// Returns true on success. `webp_data` is written as C writes it: emptied when
/// the mux assembly starts, and holding the assembled data when only the
/// single-frame optimization fails. C also returns 0 for `NULL` arguments; a
/// `NULL` `webp_data` records an error string, which the C ABI does.
#[must_use]
pub fn webp_anim_encoder_assemble(
    enc: &mut WebPAnimEncoder<'_>,
    webp_data: &mut WebPData<'static>,
) -> bool {
    mark_no_error(enc);

    if enc.in_frame_count == 0 {
        mark_error(enc, "ERROR: No frames to assemble");
        return false;
    }

    if !enc.got_null_frame && enc.in_frame_count > 1 && enc.count > 0 {
        // set duration of the last frame to be avg of durations of previous frames.
        let delta_time = f64::from(
            enc.prev_timestamp
                .cast_unsigned()
                .wrapping_sub(enc.first_timestamp.cast_unsigned()),
        );
        let average_duration = average(delta_time, enc.in_frame_count - 1);
        if !increase_previous_duration(enc, average_duration) {
            return false;
        }
    }

    // Flush any remaining frames.
    enc.flush_count = enc.count;
    if !flush_frames(enc) {
        return false;
    }

    let result: Result<(), WebPMuxError> = (|| {
        // Set definitive canvas size.
        webp_mux_set_canvas_size(&mut enc.mux, enc.canvas_width, enc.canvas_height)?;

        webp_mux_set_animation_params(&mut enc.mux, &enc.options.anim_params)?;

        // Assemble into a WebP bitstream.
        *webp_data = WebPData::default(); // C: memset in WebPMuxAssemble.
        *webp_data = webp_mux_assemble(&mut enc.mux)?;

        if enc.out_frame_count == 1 {
            optimize_single_frame(enc, webp_data)?;
        }
        Ok(())
    })();
    if let Err(err) = result {
        mark_error2(enc, "ERROR assembling WebP", err.0);
        return false;
    }
    true
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderGetError
/// Get error string corresponding to the most recent call using `enc`: the
/// error of the last call, or an empty string if it succeeded. C returns `NULL`
/// for a `NULL` encoder; its string lives in the encoder until the next call.
#[must_use]
pub fn webp_anim_encoder_get_error<'e>(enc: &'e WebPAnimEncoder<'_>) -> &'e str {
    &enc.error_str
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderSetChunk
/// Adds a chunk with id `fourcc` and data `chunk_data` to the encoder's mux; any
/// existing chunk(s) with the same id are removed. Without copying, the data
/// must exist until `WebPAnimEncoderAssemble`. C first returns
/// `WEBP_MUX_INVALID_ARGUMENT` for a `NULL` encoder.
///
/// # Errors
///
/// As `WebPMuxSetChunk`.
pub fn webp_anim_encoder_set_chunk<'a>(
    enc: &mut WebPAnimEncoder<'a>,
    fourcc: &[u8; 4],
    chunk_data: &WebPData<'a>,
    copy_data: i32,
) -> Result<(), WebPMuxError> {
    webp_mux_set_chunk(&mut enc.mux, fourcc, chunk_data, copy_data)
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderGetChunk
/// Gets a reference to the data of the chunk with id `fourcc` in the encoder's
/// mux. C first returns `WEBP_MUX_INVALID_ARGUMENT` for a `NULL` encoder.
///
/// # Errors
///
/// As `WebPMuxGetChunk`.
pub fn webp_anim_encoder_get_chunk<'e, 'a>(
    enc: &'e WebPAnimEncoder<'a>,
    fourcc: &[u8; 4],
) -> Result<&'e WebPData<'a>, WebPMuxError> {
    webp_mux_get_chunk(&enc.mux, fourcc)
}

// upstream: src/mux/anim_encode.c WebPAnimEncoderDeleteChunk
/// Deletes the chunk with the given `fourcc` from the encoder's mux. C first
/// returns `WEBP_MUX_INVALID_ARGUMENT` for a `NULL` encoder.
///
/// # Errors
///
/// As `WebPMuxDeleteChunk`.
pub fn webp_anim_encoder_delete_chunk(
    enc: &mut WebPAnimEncoder<'_>,
    fourcc: &[u8; 4],
) -> Result<(), WebPMuxError> {
    webp_mux_delete_chunk(&mut enc.mux, fourcc)
}
