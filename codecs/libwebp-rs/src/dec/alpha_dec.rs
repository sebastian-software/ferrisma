// libwebp v1.6.0, src/dec/alpha_dec.c:
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
// Alpha-plane decompression.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/alpha_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! The VP8 decoder owns the alpha decoder and the alpha plane. C's
//! `dec->alpha_data` points into the caller's input; it is the `alpha_data`
//! argument of the functions that read it (ADR 0003, "Decoding context per
//! call"), and positions in the alpha plane replace C's pointers into it.
//! `ALPHInit` calls `VP8FiltersInit`, which only assigns portable pointers and
//! has no Rust counterpart (ADR 0003), and `WebPInitCustomIo(NULL, io)`, whose
//! hooks on the alpha decoder's io the lossless decoder never calls; `VP8Io` has
//! no hook fields, so the call has nothing to set.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **The rows of the alpha plane that a decoding has not reached.** The
//!    plane is `malloc`ed; the alpha test (`alpha_rows_match_the_c_decoder` in
//!    `tests/wp6_decode.rs`) prints only the decoded rows, from the crop top on
//!    (the lossless path may skip the rows above it).

use crate::dec::alphai_dec::ALPHDecoder;
use crate::dec::vp8_dec::{VP8Io, vp8_init_io, vp8_set_error};
use crate::dec::vp8i_dec::{VP8AlphaPlane, VP8Decoder};
use crate::dec::vp8l_dec::{vp8l_decode_alpha_header, vp8l_decode_alpha_image_stream};
use crate::dsp::dsp::WEBP_FILTER_TYPE;
use crate::dsp::filters::webp_unfilters;
use crate::utils::quant_levels_dec_utils::webp_dequantize_levels;
use crate::utils::utils::webp_safe_malloc;
use crate::webp::decode::VP8StatusCode;
use crate::webp::format_constants::{
    ALPHA_HEADER_LEN, ALPHA_LOSSLESS_COMPRESSION, ALPHA_NO_COMPRESSION, ALPHA_PREPROCESSED_LEVELS,
};

/// `ALPHA_HEADER_LEN` as an index.
const HEADER_LEN: usize = ALPHA_HEADER_LEN.unsigned_abs() as usize;

/// A C `int` count or position that is not negative.
fn index(value: i32) -> usize {
    usize::try_from(value).expect("a count or position is not negative")
}

/// The alpha plane that `dec.alpha_plane` designates (C: `dec->alpha_plane`).
///
/// # Panics
///
/// Panics when the decoder has no alpha plane, where C has `NULL`.
fn alpha_plane_mut(dec: &mut VP8Decoder) -> &mut [u8] {
    match dec.alpha_plane {
        VP8AlphaPlane::Memory => &mut dec.alpha_plane_mem,
        VP8AlphaPlane::Frame => &mut dec.frame_alpha,
        VP8AlphaPlane::None => panic!("the decoder has no alpha plane"),
    }
}

//------------------------------------------------------------------------------
// ALPHDecoder object.

// upstream: src/dec/alpha_dec.c ALPHNew
/// Allocates a new alpha decoder instance. The size test of `WebPSafeCalloc`
/// always passes for one object; a refused host allocation aborts.
#[expect(
    clippy::unnecessary_box_returns,
    reason = "C returns the heap object that dec->alph_dec holds"
)]
fn alph_new() -> Box<ALPHDecoder> {
    Box::<ALPHDecoder>::default()
}

// upstream: src/dec/alpha_dec.c ALPHDelete
/// Clears and deallocates an alpha decoder instance: dropping the box frees the
/// lossless decoder with it.
fn alph_delete(dec: Option<Box<ALPHDecoder>>) {
    drop(dec);
}

//------------------------------------------------------------------------------
// Decoding.

// upstream: src/dec/alpha_dec.c ALPHInit
/// Initialize alpha decoding by parsing the alpha header and decoding the image
/// header for alpha data stored using lossless compression.
/// Returns false in case of error in alpha header (data too short, invalid
/// compression method or filter, error in lossless header data etc).
/// `data` is the alpha chunk with its header byte (C: `data`, `data_size`). C
/// also stores `output`, the alpha plane, which the lossless calls take per call
/// here (ADR 0003).
fn alph_init(dec: &mut ALPHDecoder, data: &[u8], src_io: &VP8Io) -> bool {
    // upstream: src/dsp/filters.c VP8FiltersInit (omitted, ADR 0003)
    dec.width = src_io.width;
    dec.height = src_io.height;
    debug_assert!(dec.width > 0 && dec.height > 0);

    if data.len() <= HEADER_LEN {
        return false;
    }
    let alpha_data = &data[HEADER_LEN..];

    dec.method = i32::from(data[0] & 0x03);
    dec.filter = WEBP_FILTER_TYPE(u32::from((data[0] >> 2) & 0x03));
    dec.pre_processing = i32::from((data[0] >> 4) & 0x03);
    let rsrv = (data[0] >> 6) & 0x03;
    if dec.method < ALPHA_NO_COMPRESSION
        || dec.method > ALPHA_LOSSLESS_COMPRESSION
        || dec.filter >= WEBP_FILTER_TYPE::WEBP_FILTER_LAST
        || dec.pre_processing > ALPHA_PREPROCESSED_LEVELS
        || rsrv != 0
    {
        return false;
    }

    // Copy the necessary parameters from src_io to io
    let io = &mut dec.io;
    if !vp8_init_io(io) {
        return false;
    }
    // C: WebPInitCustomIo(NULL, io); io->opaque = dec. Nothing to set here.
    io.width = src_io.width;
    io.height = src_io.height;

    io.use_cropping = src_io.use_cropping;
    io.crop_left = src_io.crop_left;
    io.crop_right = src_io.crop_right;
    io.crop_top = src_io.crop_top;
    io.crop_bottom = src_io.crop_bottom;
    // No need to copy the scaling parameters.

    if dec.method == ALPHA_NO_COMPRESSION {
        let alpha_decoded_size = index(dec.width * dec.height);
        alpha_data.len() >= alpha_decoded_size
    } else {
        debug_assert_eq!(dec.method, ALPHA_LOSSLESS_COMPRESSION);
        vp8l_decode_alpha_header(dec, alpha_data)
    }
}

// upstream: src/dec/alpha_dec.c ALPHDecode
/// Decodes, unfilters and dequantizes *at least* `num_rows` rows of alpha
/// starting from row number `row`. It assumes that rows up to (row - 1) have
/// already been decoded.
/// Returns false in case of bitstream error.
fn alph_decode(dec: &mut VP8Decoder, alpha_data: &[u8], row: i32, num_rows: i32) -> bool {
    let mut alph_dec = dec.alph_dec.take().expect("the alpha decoder is set up");
    let width = alph_dec.width;
    let height = alph_dec.io.crop_bottom;
    let ok = if alph_dec.method == ALPHA_NO_COMPRESSION {
        let mut prev_line = dec.alpha_prev_line;
        let mut deltas = HEADER_LEN + index(row * width);
        let mut dst = index(row * width);
        let w = index(width);
        debug_assert!(deltas <= alpha_data.len());
        let filter = alph_dec.filter;
        let plane = alpha_plane_mut(dec);
        for _ in 0..num_rows {
            // C passes prev_line, deltas and dst, which point at different
            // rows: the previous row is before dst in the plane.
            let (before, after) = plane.split_at_mut(dst);
            webp_unfilters(
                filter,
                prev_line.map(|prev| &before[prev..]),
                &alpha_data[deltas..],
                &mut after[..w],
                width,
            );
            prev_line = Some(dst);
            dst += w;
            deltas += w;
        }
        dec.alpha_prev_line = prev_line;
        true
    } else {
        // alph_dec->method == ALPHA_LOSSLESS_COMPRESSION
        debug_assert!(alph_dec.vp8l_dec.is_some());
        vp8l_decode_alpha_image_stream(
            &mut alph_dec,
            row + num_rows,
            &alpha_data[HEADER_LEN..],
            alpha_plane_mut(dec),
        )
        .is_ok()
    };
    dec.alph_dec = Some(alph_dec);
    if !ok {
        return false;
    }

    if row + num_rows >= height {
        dec.is_alpha_decoded = true;
    }
    true
}

// upstream: src/dec/alpha_dec.c AllocateAlphaPlane
/// Allocates the alpha plane, `io.width` by `io.crop_bottom` bytes. Returns the
/// recorded status when the allocation fails.
fn allocate_alpha_plane(dec: &mut VP8Decoder, io: &VP8Io) -> Result<(), VP8StatusCode> {
    let stride = io.width;
    let height = io.crop_bottom;
    let alpha_size = u64::from(stride.unsigned_abs()) * u64::from(height.unsigned_abs());
    debug_assert_eq!(dec.alpha_plane_mem.len(), 0);
    let Some(mut plane) = webp_safe_malloc::<u8>(alpha_size, 1) else {
        return Err(vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY,
            "Alpha decoder initialization failed.",
        ));
    };
    plane.resize(
        usize::try_from(alpha_size).expect("an accepted size fits"),
        0,
    );
    dec.alpha_plane_mem = plane;
    dec.alpha_plane = VP8AlphaPlane::Memory;
    dec.alpha_prev_line = None;
    Ok(())
}

// upstream: src/dec/alpha_dec.c WebPDeallocateAlphaMemory
/// Deallocate memory associated to `dec.alpha_plane` decoding.
pub fn webp_deallocate_alpha_memory(dec: &mut VP8Decoder) {
    dec.alpha_plane_mem = Vec::new();
    dec.alpha_plane = VP8AlphaPlane::None;
    alph_delete(dec.alph_dec.take());
}

//------------------------------------------------------------------------------
// Main entry point.

// upstream: src/dec/alpha_dec.c VP8DecompressAlphaRows
#[expect(clippy::if_not_else, reason = "the C condition")]
/// Decodes *at least* `num_rows` rows of alpha from row `row` on and returns the
/// position of row `row` in the alpha plane; `None` on failure, where C returns
/// `NULL`. `alpha_data` is the alpha chunk (C: `dec->alpha_data[..alpha_data_size]`,
/// ADR 0003).
///
/// # Panics
///
/// Panics when `io` has a negative width or crop window.
pub fn vp8_decompress_alpha_rows(
    dec: &mut VP8Decoder,
    io: &VP8Io,
    alpha_data: &[u8],
    row: i32,
    num_rows: i32,
) -> Option<usize> {
    let width = io.width;
    let height = io.crop_bottom;
    let mut num_rows = num_rows;

    if row < 0 || num_rows <= 0 || row + num_rows > height {
        return None;
    }

    if !dec.is_alpha_decoded {
        if dec.alph_dec.is_none() {
            // Initialize decoder.
            dec.alph_dec = Some(alph_new());
            if allocate_alpha_plane(dec, io).is_err() {
                webp_deallocate_alpha_memory(dec);
                return None;
            }
            let alph_dec = dec.alph_dec.as_deref_mut().expect("just set");
            if !alph_init(alph_dec, alpha_data, io) {
                let status = alph_dec
                    .vp8l_dec
                    .as_ref()
                    .map_or(VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY, |vp8l_dec| {
                        vp8l_dec.status
                    });
                let _ = vp8_set_error(dec, status, "Alpha decoder initialization failed.");
                webp_deallocate_alpha_memory(dec);
                return None;
            }
            // if we allowed use of alpha dithering, check whether it's needed at all
            if alph_dec.pre_processing != ALPHA_PREPROCESSED_LEVELS {
                dec.alpha_dithering = 0; // disable dithering
            } else {
                num_rows = height - row; // decode everything in one pass
            }
        }

        debug_assert!(dec.alph_dec.is_some());
        debug_assert!(row + num_rows <= height);
        if !alph_decode(dec, alpha_data, row, num_rows) {
            webp_deallocate_alpha_memory(dec);
            return None;
        }

        if dec.is_alpha_decoded {
            // finished?
            alph_delete(dec.alph_dec.take());
            if dec.alpha_dithering > 0 {
                let alpha = index(io.crop_top * width + io.crop_left);
                let strength = dec.alpha_dithering;
                if !webp_dequantize_levels(
                    Some(&mut alpha_plane_mut(dec)[alpha..]),
                    io.crop_right - io.crop_left,
                    io.crop_bottom - io.crop_top,
                    width,
                    strength,
                ) {
                    webp_deallocate_alpha_memory(dec);
                    return None;
                }
            }
        }
    }

    // Return a pointer to the current decoded row.
    Some(index(row * width))
}
