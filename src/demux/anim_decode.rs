// libwebp v1.6.0, src/demux/anim_decode.c:
//
// Copyright 2015 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  AnimDecoder implementation.
//

//! Translation of `src/demux/anim_decode.c` from libwebp v1.6.0.
//!
//! Work package: WP14 (animation decode and demux ABI).
//!
//! Representation (decided for WP14, see `docs/status/wp14.md`):
//!
//! - [`WebPAnimDecoder`] owns the demuxer that it creates from the caller's
//!   `WebPData`, which therefore outlives the decoder, as `demux.h` requires.
//! - C keeps the iterator of the previous frame (`prev_iter`), whose `private_`
//!   points at the decoder's own demuxer. A Rust iterator that borrows a field of
//!   the struct holding it would be self-referential, so the decoder keeps the
//!   plain fields of that iterator ([`IteratorFields`]) and builds the current
//!   frame's iterator in each `GetNext` call.
//! - C keeps a `WebPDecoderConfig` whose output buffer points into the canvas.
//!   The decoder keeps the options and the colorspace, and lends the canvas to a
//!   configuration built for each `GetNext` call (ADR 0003, "Decoding context
//!   per call").
//! - `src/webp/mux.h` is a reviewed include (`tools/units.json`): only the
//!   `WEBP_MUX_*` enums of `mux_types.h` are used.

use crate::dec::webp_dec::webp_decode;
use crate::demux::demux::{WebPDemuxer, webp_demux_get_frame, webp_demux_get_i};
use crate::utils::utils::{check_size_overflow, webp_safe_calloc};
use crate::webp::decode::{
    WEBP_CSP_MODE, WebPBitstreamFeatures, WebPDecBuffer, WebPDecBufferPlanes, WebPDecoderConfig,
    WebPDecoderOptions, WebPRGBABuffer, webp_get_features, webp_init_decoder_config,
};
use crate::webp::demux::{
    WEBP_DEMUX_ABI_VERSION, WebPAnimDecoderOptions, WebPAnimInfo, WebPFormatFeature, WebPIterator,
    webp_demux,
};
use crate::webp::mux_types::{WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose};
use crate::webp::types::{PlaneOffset, PlaneStorage, webp_abi_is_incompatible};

// upstream: src/demux/anim_decode.c NUM_CHANNELS
const NUM_CHANNELS: u32 = 4;

// upstream: src/demux/anim_decode.c CHANNEL_SHIFT
/// Channel extraction from a `u32` representation of a `u8` RGBA/BGRA buffer
/// (C: `WORDS_BIGENDIAN` selects the big-endian form).
#[cfg(target_endian = "big")]
const fn channel_shift(i: u32) -> u32 {
    24 - i * 8
}

// upstream: src/demux/anim_decode.c CHANNEL_SHIFT
/// Channel extraction from a `u32` representation of a `u8` RGBA/BGRA buffer.
#[cfg(not(target_endian = "big"))]
const fn channel_shift(i: u32) -> u32 {
    i * 8
}

// upstream: src/demux/anim_decode.c BlendRowFunc
/// Blends `num_pixels` pixels of `src` over `dst`, writing into `src`. C casts the
/// canvas to `uint32_t*`; the slices start at the first pixel's byte, and each
/// pixel is read and written in host byte order (ADR 0003, "Untyped sample
/// buffers").
type BlendRowFunc = fn(&mut [u8], &[u8], i32);

/// The fields of a `WebPIterator` that `IsKeyFrame`, `FindBlendRangeAtRow` and
/// the dispose step read (C keeps the whole iterator of the previous frame as
/// `prev_iter`). Zeroed, it is the iterator after `WebPAnimDecoderReset`'s
/// `memset`. No C counterpart.
#[derive(Clone, Copy, Debug, Default)]
struct IteratorFields {
    frame_num: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    dispose_method: WebPMuxAnimDispose,
    has_alpha: i32,
    blend_method: WebPMuxAnimBlend,
}

impl From<&WebPIterator<'_>> for IteratorFields {
    fn from(iter: &WebPIterator<'_>) -> Self {
        Self {
            frame_num: iter.frame_num,
            x_offset: iter.x_offset,
            y_offset: iter.y_offset,
            width: iter.width,
            height: iter.height,
            dispose_method: iter.dispose_method,
            has_alpha: iter.has_alpha,
            blend_method: iter.blend_method,
        }
    }
}

// upstream: src/demux/anim_decode.c WebPAnimDecoder
/// Decoder of a (possibly) animated WebP file into full canvases (C: `struct
/// WebPAnimDecoder`, opaque in `webp/demux.h`). It borrows the caller's
/// `WebPData` for its lifetime through the demuxer it owns.
#[derive(Debug)]
pub struct WebPAnimDecoder<'a> {
    /// Demuxer created from the given WebP bitstream (C: `demux`).
    demux: WebPDemuxer<'a>,
    /// Decoder options (C: `config.options`).
    options: WebPDecoderOptions,
    /// Output colorspace (C: `config.output.colorspace`). The canvas is attached
    /// as the output buffer for each frame.
    colorspace: WEBP_CSP_MODE,
    /// The chosen blend row function (C: `blend_func`).
    blend_func: BlendRowFunc,
    /// Global info about the animation (C: `info`).
    info: WebPAnimInfo,
    /// Current canvas, not disposed (C: `curr_frame`).
    curr_frame: Vec<u8>,
    /// Previous canvas, properly disposed (C: `prev_frame_disposed`).
    prev_frame_disposed: Vec<u8>,
    /// Previous frame timestamp in milliseconds (C: `prev_frame_timestamp`).
    prev_frame_timestamp: i32,
    /// The fields of the previous frame's iterator (C: `prev_iter`).
    prev_iter: IteratorFields,
    /// True if the previous frame was a keyframe (C: `prev_frame_was_keyframe`).
    prev_frame_was_keyframe: bool,
    /// Index of the next frame to be decoded, starting from 1 (C: `next_frame`).
    next_frame: i32,
}

// upstream: src/demux/anim_decode.c DefaultDecoderOptions
fn default_decoder_options(dec_options: &mut WebPAnimDecoderOptions) {
    dec_options.color_mode = WEBP_CSP_MODE::MODE_RGBA;
    dec_options.use_threads = 0;
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderOptionsInitInternal
/// Internal, version-checked, entry point of `WebPAnimDecoderOptionsInit`: sets
/// the color mode to `MODE_RGBA` and turns threads off; the padding is left
/// as it is. Returns false in case of version mismatch. C also returns 0 for a
/// `NULL` structure, which exists only at the C ABI.
#[must_use]
pub fn webp_anim_decoder_options_init_internal(
    dec_options: &mut WebPAnimDecoderOptions,
    abi_version: i32,
) -> bool {
    if webp_abi_is_incompatible(abi_version, WEBP_DEMUX_ABI_VERSION) {
        return false;
    }
    default_decoder_options(dec_options);
    true
}

// upstream: src/demux/anim_decode.c ApplyDecoderOptions
/// Checks the color mode and derives the decoder settings that C writes into
/// `dec`: the blend row function, the decoder options and the colorspace of the
/// output buffer (C: `config->output.is_external_memory = 1`; the canvas is
/// attached as caller memory for each frame). Returns `None` where C returns 0.
fn apply_decoder_options(
    dec_options: &WebPAnimDecoderOptions,
) -> Option<(BlendRowFunc, WebPDecoderOptions, WEBP_CSP_MODE)> {
    let mode = dec_options.color_mode;
    if mode != WEBP_CSP_MODE::MODE_RGBA
        && mode != WEBP_CSP_MODE::MODE_BGRA
        && mode != WEBP_CSP_MODE::MODE_rgbA
        && mode != WEBP_CSP_MODE::MODE_bgrA
    {
        return None;
    }
    let blend_func: BlendRowFunc =
        if mode == WEBP_CSP_MODE::MODE_RGBA || mode == WEBP_CSP_MODE::MODE_BGRA {
            blend_pixel_row_non_premult
        } else {
            blend_pixel_row_premult
        };
    let mut config = WebPDecoderConfig::default();
    if !webp_init_decoder_config(&mut config) {
        return None;
    }
    config.options.use_threads = dec_options.use_threads;
    // Note: config->output.u.RGBA is set at the time of decoding each frame.
    Some((blend_func, config.options, mode))
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderNewInternal
/// Internal, version-checked, entry point of `WebPAnimDecoderNew`. Returns
/// `None` in case of a version mismatch, a parsing error, an invalid option or
/// a memory error. `dec_options` `None` selects the defaults (`MODE_RGBA`, no
/// threads). `webp_data` must stay unchanged for the decoder's lifetime, which
/// the borrow enforces.
#[must_use]
pub fn webp_anim_decoder_new_internal<'a>(
    webp_data: &'a WebPData<'_>,
    dec_options: Option<&WebPAnimDecoderOptions>,
    abi_version: i32,
) -> Option<WebPAnimDecoder<'a>> {
    if webp_abi_is_incompatible(abi_version, WEBP_DEMUX_ABI_VERSION) {
        return None;
    }

    // Validate the bitstream before doing expensive allocations. The demuxer may
    // be more tolerant than the decoder.
    let mut features = WebPBitstreamFeatures::default();
    if webp_get_features(&webp_data.bytes, &mut features).is_err() {
        return None;
    }

    // C: dec = WebPSafeCalloc(1ULL, sizeof(*dec)). The decoder is a struct value
    // here, assembled from the parts below.
    let mut options = WebPAnimDecoderOptions::default();
    match dec_options {
        Some(dec_options) => options = *dec_options,
        None => default_decoder_options(&mut options),
    }
    let (blend_func, decoder_options, colorspace) = apply_decoder_options(&options)?;

    let demux = webp_demux(webp_data)?;

    let info = WebPAnimInfo {
        canvas_width: webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_CANVAS_WIDTH),
        canvas_height: webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT),
        loop_count: webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_LOOP_COUNT),
        bgcolor: webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR),
        frame_count: webp_demux_get_i(&demux, WebPFormatFeature::WEBP_FF_FRAME_COUNT),
        pad: [0; 4],
    };

    // Note: calloc() because we fill frame with zeroes as well.
    let curr_frame = canvas_calloc(info.canvas_width, info.canvas_height)?;
    let prev_frame_disposed = canvas_calloc(info.canvas_width, info.canvas_height)?;

    let mut dec = WebPAnimDecoder {
        demux,
        options: decoder_options,
        colorspace,
        blend_func,
        info,
        curr_frame,
        prev_frame_disposed,
        prev_frame_timestamp: 0,
        prev_iter: IteratorFields::default(),
        prev_frame_was_keyframe: false,
        next_frame: 0,
    };
    webp_anim_decoder_reset(&mut dec);
    Some(dec)
}

/// C's `WebPSafeCalloc(canvas_width * NUM_CHANNELS, canvas_height)`: a zeroed
/// canvas of `canvas_width * 4 * canvas_height` bytes, or `None` where C returns
/// `NULL`. The `uint32_t` product `canvas_width * NUM_CHANNELS` is the element
/// count and the height the element size; the Rust helper takes the byte count
/// with an element size of one. The two size tests agree: for `n > 0`,
/// `h > MAX / n` holds exactly when `n * h > MAX`, which is `1 > MAX / (n * h)`,
/// and the product cannot wrap, because the demuxer limits both dimensions to
/// 2^24.
fn canvas_calloc(canvas_width: u32, canvas_height: u32) -> Option<Vec<u8>> {
    let nmemb = u64::from(canvas_width.wrapping_mul(NUM_CHANNELS));
    webp_safe_calloc::<u8>(nmemb * u64::from(canvas_height), 1)
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderGetInfo
/// Returns the global information about the animation. C writes it to `*info`
/// and returns 1, or 0 for `NULL` arguments, which exist only at the C ABI.
#[must_use]
pub fn webp_anim_decoder_get_info(dec: &WebPAnimDecoder<'_>) -> WebPAnimInfo {
    dec.info
}

// upstream: src/demux/anim_decode.c IsFullFrame
/// Returns true if the frame covers the full canvas.
fn is_full_frame(width: i32, height: i32, canvas_width: i32, canvas_height: i32) -> bool {
    width == canvas_width && height == canvas_height
}

// upstream: src/demux/anim_decode.c ZeroFillCanvas
/// Clears the canvas to transparent. Returns false when the size does not fit
/// in `usize`.
fn zero_fill_canvas(buf: &mut [u8], canvas_width: u32, canvas_height: u32) -> bool {
    // sizeof(*buf) is 1.
    let size = u64::from(canvas_width)
        .wrapping_mul(u64::from(canvas_height))
        .wrapping_mul(u64::from(NUM_CHANNELS));
    if !check_size_overflow(size) {
        return false;
    }
    let Ok(size) = usize::try_from(size) else {
        return false;
    };
    buf[..size].fill(0);
    true
}

// upstream: src/demux/anim_decode.c ZeroFillFrameRect
/// Clears the given frame rectangle to transparent. C computes the offsets in
/// `int`, whose overflow is undefined (`docs/status/wp14.md`); the frame lies
/// inside the canvas, so the offset is never negative otherwise.
fn zero_fill_frame_rect(
    buf: &mut [u8],
    buf_stride: i32,
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
) {
    debug_assert!(width * NUM_CHANNELS.cast_signed() <= buf_stride);
    let mut start = y_offset * buf_stride + x_offset * NUM_CHANNELS.cast_signed();
    let row = usize::try_from(width * NUM_CHANNELS.cast_signed()).unwrap_or(0);
    for _ in 0..height {
        let at = usize::try_from(start).expect("the frame rectangle lies inside the canvas");
        buf[at..at + row].fill(0);
        start += buf_stride;
    }
}

// upstream: src/demux/anim_decode.c CopyCanvas
/// Copies `width * height` pixels from `src` to `dst`. Returns false when the
/// size does not fit in `usize`.
fn copy_canvas(src: &[u8], dst: &mut [u8], width: u32, height: u32) -> bool {
    let size = u64::from(width)
        .wrapping_mul(u64::from(height))
        .wrapping_mul(u64::from(NUM_CHANNELS));
    if !check_size_overflow(size) {
        return false;
    }
    let Ok(size) = usize::try_from(size) else {
        return false;
    };
    dst[..size].copy_from_slice(&src[..size]);
    true
}

// upstream: src/demux/anim_decode.c IsKeyFrame
/// Returns true if the current frame is a key-frame.
#[expect(
    clippy::if_same_then_else,
    reason = "C's IsKeyFrame returns 1 from two separate branches"
)]
fn is_key_frame(
    curr: &IteratorFields,
    prev: &IteratorFields,
    prev_frame_was_key_frame: bool,
    canvas_width: i32,
    canvas_height: i32,
) -> bool {
    if curr.frame_num == 1 {
        true
    } else if (curr.has_alpha == 0 || curr.blend_method == WebPMuxAnimBlend::WEBP_MUX_NO_BLEND)
        && is_full_frame(curr.width, curr.height, canvas_width, canvas_height)
    {
        true
    } else {
        prev.dispose_method == WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND
            && (is_full_frame(prev.width, prev.height, canvas_width, canvas_height)
                || prev_frame_was_key_frame)
    }
}

// upstream: src/demux/anim_decode.c BlendChannelNonPremult
/// Blends a single channel of `src` over `dst`, given their alpha channel
/// values. `src` and `dst` are assumed to be NOT pre-multiplied by alpha.
fn blend_channel_non_premult(
    src: u32,
    src_a: u8,
    dst: u32,
    dst_a: u8,
    scale: u32,
    shift: u32,
) -> u8 {
    let src_channel = ((src >> shift) & 0xff) as u8;
    let dst_channel = ((dst >> shift) & 0xff) as u8;
    // C computes the sum in int; it stays below 2 * 255 * 255.
    let blend_unscaled =
        u32::from(src_channel) * u32::from(src_a) + u32::from(dst_channel) * u32::from(dst_a);
    debug_assert!(u64::from(blend_unscaled) < (1u64 << 32) / u64::from(scale));
    #[expect(
        clippy::cast_possible_truncation,
        reason = "C returns the shifted value as uint8_t"
    )]
    let blended = (blend_unscaled.wrapping_mul(scale) >> channel_shift(3)) as u8;
    blended
}

// upstream: src/demux/anim_decode.c BlendPixelNonPremult
/// Blends `src` over `dst` assuming they are NOT pre-multiplied by alpha.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C assigns the int results to uint8_t variables"
)]
fn blend_pixel_non_premult(src: u32, dst: u32) -> u32 {
    let src_a = ((src >> channel_shift(3)) & 0xff) as u8;

    if src_a == 0 {
        dst
    } else {
        let dst_a = ((dst >> channel_shift(3)) & 0xff) as u8;
        // This is the approximate integer arithmetic for the actual formula:
        // dst_factor_a = (dst_a * (255 - src_a)) / 255.
        let dst_factor_a = ((i32::from(dst_a) * (256 - i32::from(src_a))) >> 8) as u8;
        let blend_a = (i32::from(src_a) + i32::from(dst_factor_a)) as u8;
        let scale = (1u32 << 24) / u32::from(blend_a);

        let blend_r =
            blend_channel_non_premult(src, src_a, dst, dst_factor_a, scale, channel_shift(0));
        let blend_g =
            blend_channel_non_premult(src, src_a, dst, dst_factor_a, scale, channel_shift(1));
        let blend_b =
            blend_channel_non_premult(src, src_a, dst, dst_factor_a, scale, channel_shift(2));
        debug_assert!(i32::from(src_a) + i32::from(dst_factor_a) < 256);

        (u32::from(blend_r) << channel_shift(0))
            | (u32::from(blend_g) << channel_shift(1))
            | (u32::from(blend_b) << channel_shift(2))
            | (u32::from(blend_a) << channel_shift(3))
    }
}

/// The pixel at byte `at` of a canvas, as C reads it through `uint32_t*`.
fn read_pixel(buf: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]])
}

// upstream: src/demux/anim_decode.c BlendPixelRowNonPremult
/// Blends `num_pixels` in `src` over `dst` assuming they are NOT pre-multiplied
/// by alpha.
fn blend_pixel_row_non_premult(src: &mut [u8], dst: &[u8], num_pixels: i32) {
    for i in 0..usize::try_from(num_pixels).unwrap_or(0) {
        let at = 4 * i;
        let src_pixel = read_pixel(src, at);
        let src_alpha = ((src_pixel >> channel_shift(3)) & 0xff) as u8;
        if src_alpha != 0xff {
            let blended = blend_pixel_non_premult(src_pixel, read_pixel(dst, at));
            src[at..at + 4].copy_from_slice(&blended.to_ne_bytes());
        }
    }
}

// upstream: src/demux/anim_decode.c ChannelwiseMultiply
/// Individually multiplies each channel in `pix` by `scale`.
fn channelwise_multiply(pix: u32, scale: u32) -> u32 {
    let mask = 0x00FF_00FF_u32;
    let rb = (pix & mask).wrapping_mul(scale) >> 8;
    let ag = ((pix >> 8) & mask).wrapping_mul(scale);
    (rb & mask) | (ag & !mask)
}

// upstream: src/demux/anim_decode.c BlendPixelPremult
/// Blends `src` over `dst` assuming they are pre-multiplied by alpha.
fn blend_pixel_premult(src: u32, dst: u32) -> u32 {
    let src_a = ((src >> channel_shift(3)) & 0xff) as u8;
    // C passes the int 256 - src_a as the uint32_t scale.
    src.wrapping_add(channelwise_multiply(dst, 256 - u32::from(src_a)))
}

// upstream: src/demux/anim_decode.c BlendPixelRowPremult
/// Blends `num_pixels` in `src` over `dst` assuming they are pre-multiplied by
/// alpha.
fn blend_pixel_row_premult(src: &mut [u8], dst: &[u8], num_pixels: i32) {
    for i in 0..usize::try_from(num_pixels).unwrap_or(0) {
        let at = 4 * i;
        let src_pixel = read_pixel(src, at);
        let src_alpha = ((src_pixel >> channel_shift(3)) & 0xff) as u8;
        if src_alpha != 0xff {
            let blended = blend_pixel_premult(src_pixel, read_pixel(dst, at));
            src[at..at + 4].copy_from_slice(&blended.to_ne_bytes());
        }
    }
}

// upstream: src/demux/anim_decode.c FindBlendRangeAtRow
/// Returns two ranges (`<left, width>` pairs) at row `canvas_y` that belong to
/// `src` but not `dst`, as `(left1, width1, left2, width2)`. A range is empty if
/// its width is 0 (C writes the four values through pointers).
fn find_blend_range_at_row(
    src: &IteratorFields,
    dst: &IteratorFields,
    canvas_y: i32,
) -> (i32, i32, i32, i32) {
    let src_max_x = src.x_offset + src.width;
    let dst_max_x = dst.x_offset + dst.width;
    let dst_max_y = dst.y_offset + dst.height;
    debug_assert!(canvas_y >= src.y_offset && canvas_y < (src.y_offset + src.height));
    let mut left1 = -1;
    let mut width1 = 0;
    let mut left2 = -1;
    let mut width2 = 0;

    if canvas_y < dst.y_offset
        || canvas_y >= dst_max_y
        || src.x_offset >= dst_max_x
        || src_max_x <= dst.x_offset
    {
        left1 = src.x_offset;
        width1 = src.width;
        return (left1, width1, left2, width2);
    }

    if src.x_offset < dst.x_offset {
        left1 = src.x_offset;
        width1 = dst.x_offset - src.x_offset;
    }

    if src_max_x > dst_max_x {
        left2 = dst_max_x;
        width2 = src_max_x - dst_max_x;
    }
    (left1, width1, left2, width2)
}

/// C's `size_t` pixel index `canvas_y * width + left`: the `int` operands are
/// converted to `uint32_t`, and the arithmetic wraps there.
fn pixel_index(canvas_y: i32, width: u32, left: i32) -> usize {
    canvas_y
        .cast_unsigned()
        .wrapping_mul(width)
        .wrapping_add(left.cast_unsigned()) as usize
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderGetNext
/// Fetches the next frame: the fully reconstructed canvas of `canvas_width * 4 *
/// canvas_height` bytes in the decoder's color mode, and its timestamp in
/// milliseconds. Returns `None` if there is a parsing or decoding error, or if
/// there are no more frames (C returns 0 and writes neither output). After a
/// failure the canvas holds whatever the failed step left in it, as in C, and
/// the decoder stays at the same frame.
///
/// # Panics
///
/// Never for a decoder of `webp_anim_decoder_new`: the demuxer keeps every frame
/// inside the canvas, so the frame's offset lies inside the canvas.
#[expect(
    clippy::too_many_lines,
    reason = "C's WebPAnimDecoderGetNext is one function, translated as it is"
)]
pub fn webp_anim_decoder_get_next<'d>(dec: &'d mut WebPAnimDecoder<'_>) -> Option<(&'d [u8], i32)> {
    if !webp_anim_decoder_has_more_frames(dec) {
        return None;
    }

    let width = dec.info.canvas_width;
    let height = dec.info.canvas_height;
    let blend_row = dec.blend_func;

    // Get compressed frame.
    let iter = webp_demux_get_frame(&dec.demux, dec.next_frame)?;
    let timestamp = dec.prev_frame_timestamp + iter.duration;
    let curr = IteratorFields::from(&iter);

    // Initialize.
    let is_key_frame = is_key_frame(
        &curr,
        &dec.prev_iter,
        dec.prev_frame_was_keyframe,
        width.cast_signed(),
        height.cast_signed(),
    );
    if is_key_frame {
        if !zero_fill_canvas(&mut dec.curr_frame, width, height) {
            return None;
        }
    } else if !copy_canvas(&dec.prev_frame_disposed, &mut dec.curr_frame, width, height) {
        return None;
    }

    // Decode.
    {
        let input: &[u8] = &iter.fragment.bytes;
        let stride = width.wrapping_mul(NUM_CHANNELS); // at most 25 + 2 bits
        let out_offset = i64::from(iter.y_offset)
            .cast_unsigned()
            .wrapping_mul(u64::from(stride))
            .wrapping_add(
                i64::from(iter.x_offset)
                    .cast_unsigned()
                    .wrapping_mul(u64::from(NUM_CHANNELS)),
            ); // 53b
        let size = i64::from(iter.height)
            .cast_unsigned()
            .wrapping_mul(u64::from(stride)); // at most 25 + 27b
        if usize::try_from(size).is_err() {
            return None;
        }
        // C points the output at curr_frame + out_offset with the size
        // iter.height * stride, which on the bottom rows reaches x_offset * 4
        // bytes past the canvas, where C never writes. The canvas from that
        // offset to its end is lent with its own length as the size: the buffer
        // check asks only for stride * (height - 1) + width * 4 bytes, so the same
        // frames pass and fail (docs/status/wp14.md). The demuxer keeps every
        // frame inside the canvas, so the offset lies inside it.
        let out_offset =
            usize::try_from(out_offset).expect("the demuxer keeps the frame inside the canvas");
        let canvas = &mut dec.curr_frame[out_offset..];
        let mut config = WebPDecoderConfig {
            input: WebPBitstreamFeatures::default(),
            output: WebPDecBuffer {
                colorspace: dec.colorspace,
                u: WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
                    rgba: PlaneOffset::default(),
                    stride: stride.cast_signed(),
                    size: canvas.len(),
                }),
                storage: PlaneStorage::External(vec![canvas]),
                ..WebPDecBuffer::default()
            },
            options: dec.options,
        };

        if webp_decode(input, &mut config).is_err() {
            return None;
        }
    }

    // During the decoding of current frame, we may have set some pixels to be
    // transparent (i.e. alpha < 255). However, the value of each of these
    // pixels should have been determined by blending it against the value of
    // that pixel in the previous frame if blending method of is WEBP_MUX_BLEND.
    if iter.frame_num > 1 && iter.blend_method == WebPMuxAnimBlend::WEBP_MUX_BLEND && !is_key_frame
    {
        if dec.prev_iter.dispose_method == WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE {
            // Blend transparent pixels with pixels in previous canvas.
            for y in 0..iter.height {
                let offset = 4 * pixel_index(iter.y_offset + y, width, iter.x_offset);
                blend_row(
                    &mut dec.curr_frame[offset..],
                    &dec.prev_frame_disposed[offset..],
                    iter.width,
                );
            }
        } else {
            debug_assert_eq!(
                dec.prev_iter.dispose_method,
                WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND
            );
            // We need to blend a transparent pixel with its value just after
            // initialization. That is, blend it with:
            // * Fully transparent pixel if it belongs to prevRect <-- No-op.
            // * The pixel in the previous canvas otherwise <-- Need alpha-blending.
            for y in 0..iter.height {
                let canvas_y = iter.y_offset + y;
                let (left1, width1, left2, width2) =
                    find_blend_range_at_row(&curr, &dec.prev_iter, canvas_y);
                if width1 > 0 {
                    let offset1 = 4 * pixel_index(canvas_y, width, left1);
                    blend_row(
                        &mut dec.curr_frame[offset1..],
                        &dec.prev_frame_disposed[offset1..],
                        width1,
                    );
                }
                if width2 > 0 {
                    let offset2 = 4 * pixel_index(canvas_y, width, left2);
                    blend_row(
                        &mut dec.curr_frame[offset2..],
                        &dec.prev_frame_disposed[offset2..],
                        width2,
                    );
                }
            }
        }
    }

    // Update info of the previous frame and dispose it for the next iteration.
    dec.prev_frame_timestamp = timestamp;
    // C: WebPDemuxReleaseIterator(&dec->prev_iter), which frees nothing.
    dec.prev_iter = curr;
    dec.prev_frame_was_keyframe = is_key_frame;
    if !copy_canvas(&dec.curr_frame, &mut dec.prev_frame_disposed, width, height) {
        return None;
    }
    if dec.prev_iter.dispose_method == WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND {
        zero_fill_frame_rect(
            &mut dec.prev_frame_disposed,
            width.wrapping_mul(NUM_CHANNELS).cast_signed(),
            dec.prev_iter.x_offset,
            dec.prev_iter.y_offset,
            dec.prev_iter.width,
            dec.prev_iter.height,
        );
    }
    dec.next_frame += 1;

    // All OK, fill in the values.
    Some((&dec.curr_frame, timestamp))
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderHasMoreFrames
/// Returns true if some frames are yet to be decoded. C also returns 0 for a
/// `NULL` decoder, which exists only at the C ABI.
#[must_use]
pub fn webp_anim_decoder_has_more_frames(dec: &WebPAnimDecoder<'_>) -> bool {
    dec.next_frame <= dec.info.frame_count.cast_signed()
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderReset
/// Resets the decoder, so that the next `webp_anim_decoder_get_next` restarts
/// decoding from the first frame. The canvases keep their contents, as in C.
pub fn webp_anim_decoder_reset(dec: &mut WebPAnimDecoder<'_>) {
    dec.prev_frame_timestamp = 0;
    // C: WebPDemuxReleaseIterator(&dec->prev_iter), which frees nothing, then
    // memset(&dec->prev_iter, 0, sizeof(dec->prev_iter)).
    dec.prev_iter = IteratorFields::default();
    dec.prev_frame_was_keyframe = false;
    dec.next_frame = 1;
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderGetDemuxer
/// Returns the decoder's demuxer, for example to read EXIF, XMP or ICC chunks.
/// It is owned by the decoder. C returns `NULL` for a `NULL` decoder, which
/// exists only at the C ABI.
#[must_use]
pub fn webp_anim_decoder_get_demuxer<'d, 'a>(dec: &'d WebPAnimDecoder<'a>) -> &'d WebPDemuxer<'a> {
    &dec.demux
}

// upstream: src/demux/anim_decode.c WebPAnimDecoderDelete
/// Deletes the decoder: dropping it releases the demuxer and both canvases. C
/// accepts `NULL`; in Rust the caller holds no `Option`.
pub fn webp_anim_decoder_delete(dec: WebPAnimDecoder<'_>) {
    drop(dec);
}

impl WebPAnimDecoder<'_> {
    /// The current canvas, mutably, for the C ABI, which hands C its pointer
    /// after a successful `webp_anim_decoder_get_next` (C: `dec->curr_frame`).
    pub(crate) fn canvas_mut(&mut self) -> &mut [u8] {
        &mut self.curr_frame
    }
}
