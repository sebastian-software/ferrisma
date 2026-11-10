// libwebp v1.6.0, src/dec/io_dec.c:
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
// functions for sample output.
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/io_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! The emit functions write the output buffer that the caller passes next to
//! the parameters, and `CustomSetup`, `CustomPut` and `CustomTeardown` are the
//! methods of [`CustomIo`], which borrows the pair (ADR 0003, "Decoding context
//! per call"). The `Init` calls that only assign portable pointers
//! (`WebPInitUpsamplers`, `WebPInitSamplers`, `WebPInitYUV444Converters`,
//! `WebPInitAlphaProcessing`) have no Rust counterpart (ADR 0003). `FANCY_UPSAMPLING`
//! is defined and `WEBP_REDUCE_SIZE` is not in the reference configuration, so
//! the fancy upsampler and the rescaling paths are translated; `WEBP_SWAP_16BIT_CSP`
//! is 0. The WP4 helpers that take a stride accept only non-negative strides
//! (item 2 of `# Deviations from upstream` in `src/dsp/dsp.rs`); where C passes
//! the negative stride of a flipped output buffer, they are called one row at a
//! time with C's row arithmetic.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Flipped output in `io_dec.c`.** The helpers of `src/dsp` that take a
//!    stride accept only non-negative strides (item 2 of
//!    `# Deviations from upstream` in `src/dsp/dsp.rs`). Where C passes the
//!    negative stride of a flipped buffer to `WebPDispatchAlpha`,
//!    `WebPApplyAlphaMultiply`, `WebPApplyAlphaMultiply4444`, `WebPMultRows` or
//!    `WebPSamplerProcessPlane`, `io_dec.rs` calls them one row at a time with
//!    C's row arithmetic. Row positions are computed per row from the plane's
//!    offset, so no position is formed past the last row. The rescalers receive
//!    the plane's first row as `dst_pos` and the signed stride. In
//!    `EmitRescaledAlphaYUV`, C reads the alpha rows from the output while it
//!    writes the luma rows; the planes may share one region here, so the alpha
//!    rows are copied first.
//! 2. **`WebPInitCustomIo`** returns `CustomIo`, which borrows the parameters
//!    and the output buffer and implements `VP8IoHooks`; `VP8Io` has no hook
//!    fields, so `io` is left as it is. `ALPHInit`'s
//!    `WebPInitCustomIo(NULL, io)` has nothing to set. `p->memory` holds only
//!    what C carves besides the rescalers: the fancy upsampler's rows, or the
//!    RGB rescalers' output rows (the rescalers own their work rows and point
//!    at their row through `dst_pos`). The size test applies to C's total,
//!    rescaler structs and alignment included. `CustomTeardown` drops the
//!    rescalers with the memory, where C leaves their pointers dangling.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Output buffers smaller than the output, overlapping planes, or a
//!    `WebPDecBufferPlanes` variant that does not match the colorspace**, as in
//!    item 3 of `# Excluded undefined behavior` in `src/dec/vp8l_dec.rs`; and a
//!    YUVA output without an A plane while the picture has alpha, which C
//!    writes through `NULL`.

use crate::dec::vp8_dec::{VP8Io, VP8IoHooks, VP8IoRows};
use crate::dec::webp_dec::webp_io_init_from_options;
use crate::dec::webpi_dec::WebPDecParams;
use crate::dsp::alpha_processing::{
    webp_apply_alpha_multiply, webp_apply_alpha_multiply4444, webp_dispatch_alpha, webp_mult_row,
    webp_mult_rows,
};
use crate::dsp::rescaler::webp_rescaler_export_row;
use crate::dsp::upsampling::{webp_upsamplers, webp_yuv444_converters};
use crate::dsp::yuv::{webp_sampler_process_plane, webp_samplers};
use crate::utils::rescaler_utils::{
    WebPRescaler, rescaler_t, webp_rescale_needed_lines, webp_rescaler_export,
    webp_rescaler_has_pending_output, webp_rescaler_import, webp_rescaler_init,
};
use crate::utils::utils::{WEBP_ALIGN_CST, check_size_overflow, webp_copy_plane, webp_safe_malloc};
use crate::webp::decode::{
    WEBP_CSP_MODE, WebPDecBuffer, WebPDecBufferPlanes, WebPRGBABuffer, WebPYUVABuffer,
    webp_is_alpha_mode, webp_is_premultiplied_mode, webp_is_rgb_mode,
};
use crate::webp::types::PlaneOffset;

//------------------------------------------------------------------------------
// Helpers for the output buffer and the rows (no C counterpart)

/// A C `int` count or size that is not negative.
fn count(value: i32) -> usize {
    usize::try_from(value).expect("a count or size is not negative")
}

/// The position of row `y` of a plane that starts at `offset` with the signed
/// `stride`, as C computes `base + (ptrdiff_t)y * stride`.
///
/// # Panics
///
/// Panics when the row starts before the buffer, where the C pointer would be
/// outside it.
fn row_position(offset: usize, y: i32, stride: i32) -> usize {
    let delta = isize::try_from(i64::from(y) * i64::from(stride)).expect("the row offset fits");
    offset
        .checked_add_signed(delta)
        .expect("the row starts inside the buffer")
}

/// The RGBA member of the output buffer, which its colorspace selects.
fn rgba_buffer(output: &WebPDecBuffer<'_>) -> WebPRGBABuffer {
    match output.u {
        WebPDecBufferPlanes::RGBA(buf) => buf,
        WebPDecBufferPlanes::YUVA(_) => panic!("an RGB colorspace selects the RGBA member"),
    }
}

/// The YUVA member of the output buffer, which its colorspace selects.
fn yuva_buffer(output: &WebPDecBuffer<'_>) -> WebPYUVABuffer {
    match output.u {
        WebPDecBufferPlanes::YUVA(buf) => buf,
        WebPDecBufferPlanes::RGBA(_) => panic!("a YUV colorspace selects the YUVA member"),
    }
}

/// The samples of the region that holds `plane`, mutably.
fn region_mut<'b>(output: &'b mut WebPDecBuffer<'_>, plane: PlaneOffset) -> &'b mut [u8] {
    output
        .storage
        .region_mut(plane.region)
        .expect("the plane's region exists")
}

/// Two distinct rows of one region, mutably: the slices from `first` and from
/// `second` on, each ending where the other row starts if it lies behind.
fn two_rows(region: &mut [u8], first: usize, second: usize) -> (&mut [u8], &mut [u8]) {
    if first < second {
        let (low, high) = region.split_at_mut(second);
        (&mut low[first..], high)
    } else {
        let (low, high) = region.split_at_mut(first);
        (high, &mut low[second..])
    }
}

/// C's `io->a` as the alpha plane and the position of the pointer in it.
fn alpha_rows<'r>(rows: &'r VP8IoRows<'_>) -> Option<(&'r [u8], usize)> {
    rows.a.map(|plane| (plane, rows.a_offset))
}

/// `WebPDispatchAlpha` over rows that may be a negative `dst_stride` apart: one
/// call when the stride is not negative, else one call per row, with the masks
/// combined as C's loop combines them.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters with the positions"
)]
fn dispatch_alpha(
    alpha: &[u8],
    alpha_pos: usize,
    alpha_stride: i32,
    width: i32,
    height: i32,
    dst: &mut [u8],
    dst_pos: usize,
    dst_stride: i32,
) -> bool {
    if dst_stride >= 0 {
        return webp_dispatch_alpha(
            &alpha[alpha_pos..],
            alpha_stride,
            width,
            height,
            &mut dst[dst_pos..],
            dst_stride,
        );
    }
    let mut has_alpha = false;
    for j in 0..height {
        let a = row_position(alpha_pos, j, alpha_stride);
        let d = row_position(dst_pos, j, dst_stride);
        has_alpha |= webp_dispatch_alpha(&alpha[a..], 0, width, 1, &mut dst[d..], 0);
    }
    has_alpha
}

/// `WebPApplyAlphaMultiply` over rows that may be a negative `stride` apart.
fn apply_alpha_multiply(
    rgba: &mut [u8],
    pos: usize,
    alpha_first: bool,
    w: i32,
    h: i32,
    stride: i32,
) {
    if stride >= 0 {
        webp_apply_alpha_multiply(&mut rgba[pos..], alpha_first, w, h, stride);
    } else {
        for j in 0..h {
            let row = row_position(pos, j, stride);
            webp_apply_alpha_multiply(&mut rgba[row..], alpha_first, w, 1, 0);
        }
    }
}

/// `WebPApplyAlphaMultiply4444` over rows that may be a negative `stride` apart.
fn apply_alpha_multiply4444(rgba4444: &mut [u8], pos: usize, w: i32, h: i32, stride: i32) {
    if stride >= 0 {
        webp_apply_alpha_multiply4444(&mut rgba4444[pos..], w, h, stride);
    } else {
        for j in 0..h {
            let row = row_position(pos, j, stride);
            webp_apply_alpha_multiply4444(&mut rgba4444[row..], w, 1, 0);
        }
    }
}

//------------------------------------------------------------------------------
// Main YUV<->RGB conversion functions

// upstream: src/dec/io_dec.c EmitYUV
fn emit_yuv(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    _p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let buf = yuva_buffer(output);
    let y_dst = row_position(buf.y.offset, io.mb_y, buf.y_stride);
    let u_dst = row_position(buf.u.offset, io.mb_y >> 1, buf.u_stride);
    let v_dst = row_position(buf.v.offset, io.mb_y >> 1, buf.v_stride);
    let mb_w = io.mb_w;
    let mb_h = io.mb_h;
    let uv_w = (mb_w + 1) / 2;
    let uv_h = (mb_h + 1) / 2;
    webp_copy_plane(
        rows.y,
        0,
        io.y_stride,
        region_mut(output, buf.y),
        y_dst,
        buf.y_stride,
        mb_w,
        mb_h,
    );
    webp_copy_plane(
        rows.u,
        0,
        io.uv_stride,
        region_mut(output, buf.u),
        u_dst,
        buf.u_stride,
        uv_w,
        uv_h,
    );
    webp_copy_plane(
        rows.v,
        0,
        io.uv_stride,
        region_mut(output, buf.v),
        v_dst,
        buf.v_stride,
        uv_w,
        uv_h,
    );
    io.mb_h
}

// upstream: src/dec/io_dec.c EmitSampledRGB
/// Point-sampling U/V sampler.
fn emit_sampled_rgb(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    _p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let colorspace = output.colorspace;
    let buf = rgba_buffer(output);
    let dst = row_position(buf.rgba.offset, io.mb_y, buf.stride);
    let region = region_mut(output, buf.rgba);
    let sampler = |y: &[u8], u: &[u8], v: &[u8], dst: &mut [u8], len: i32| {
        webp_samplers(colorspace, y, u, v, dst, len);
    };
    if buf.stride >= 0 {
        webp_sampler_process_plane(
            rows.y,
            io.y_stride,
            rows.u,
            rows.v,
            io.uv_stride,
            &mut region[dst..],
            buf.stride,
            io.mb_w,
            io.mb_h,
            sampler,
        );
    } else {
        // The rows of WebPSamplerProcessPlane, one at a time: the chroma row
        // advances after every odd row.
        for j in 0..io.mb_h {
            let y = row_position(0, j, io.y_stride);
            let uv = row_position(0, j >> 1, io.uv_stride);
            let d = row_position(dst, j, buf.stride);
            webp_sampler_process_plane(
                &rows.y[y..],
                0,
                &rows.u[uv..],
                &rows.v[uv..],
                0,
                &mut region[d..],
                0,
                io.mb_w,
                1,
                sampler,
            );
        }
    }
    io.mb_h
}

//------------------------------------------------------------------------------
// Fancy upsampling

// upstream: src/dec/io_dec.c EmitFancyRGB
#[expect(clippy::too_many_lines, reason = "the C function")]
fn emit_fancy_rgb(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let mut num_lines_out = io.mb_h; // a priori guess
    let colorspace = output.colorspace;
    let buf = rgba_buffer(output);
    let stride = buf.stride;
    let region = region_mut(output, buf.rgba);
    let mut dst = row_position(buf.rgba.offset, io.mb_y, stride);
    let upsample = |top_y: &[u8],
                    bottom: Option<(&[u8], &mut [u8])>,
                    top_u: &[u8],
                    top_v: &[u8],
                    cur_u: &[u8],
                    cur_v: &[u8],
                    top_dst: &mut [u8],
                    len: i32| {
        webp_upsamplers(
            colorspace, top_y, bottom, top_u, top_v, cur_u, cur_v, top_dst, len,
        );
    };
    let ys = &*rows.y;
    let us = rows.u;
    let vs = rows.v;
    let mut cur_y = 0usize;
    let mut cur_u = 0usize;
    let mut cur_v = 0usize;
    let mut y = io.mb_y;
    let y_end = io.mb_y + io.mb_h;
    let mb_w = io.mb_w;
    let uv_w = count((mb_w + 1) / 2);
    let y_step = |pos: usize, n: i32| row_position(pos, n, io.y_stride);
    let uv_step = |pos: usize| row_position(pos, 1, io.uv_stride);

    {
        // C: top_u = p->tmp_u and top_v = p->tmp_v, in p->memory.
        let mut top_u: &[u8] = &p.memory[p.tmp_u..];
        let mut top_v: &[u8] = &p.memory[p.tmp_v..];
        if y == 0 {
            // First line is special cased. We mirror the u/v samples at boundary.
            upsample(
                &ys[cur_y..],
                None,
                &us[cur_u..],
                &vs[cur_v..],
                &us[cur_u..],
                &vs[cur_v..],
                &mut region[dst..],
                mb_w,
            );
        } else {
            // We can finish the left-over line from previous call.
            let (top_dst, bottom_dst) = two_rows(region, row_position(dst, -1, stride), dst);
            upsample(
                &p.memory[p.tmp_y..],
                Some((&ys[cur_y..], bottom_dst)),
                top_u,
                top_v,
                &us[cur_u..],
                &vs[cur_v..],
                top_dst,
                mb_w,
            );
            num_lines_out += 1;
        }
        // Loop over each output pairs of row.
        while y + 2 < y_end {
            top_u = &us[cur_u..];
            top_v = &vs[cur_v..];
            cur_u = uv_step(cur_u);
            cur_v = uv_step(cur_v);
            dst = row_position(dst, 2, stride);
            cur_y = y_step(cur_y, 2);
            let (top_dst, bottom_dst) = two_rows(region, row_position(dst, -1, stride), dst);
            upsample(
                &ys[y_step(cur_y, -1)..],
                Some((&ys[cur_y..], bottom_dst)),
                top_u,
                top_v,
                &us[cur_u..],
                &vs[cur_v..],
                top_dst,
                mb_w,
            );
            y += 2;
        }
    }
    // move to last row
    cur_y = y_step(cur_y, 1);
    if io.crop_top + y_end < io.crop_bottom {
        // Save the unfinished samples for next call (as we're not done yet).
        let width = count(mb_w);
        let (tmp_y, tmp_u, tmp_v) = (p.tmp_y, p.tmp_u, p.tmp_v);
        p.memory[tmp_y..tmp_y + width].copy_from_slice(&ys[cur_y..cur_y + width]);
        p.memory[tmp_u..tmp_u + uv_w].copy_from_slice(&us[cur_u..cur_u + uv_w]);
        p.memory[tmp_v..tmp_v + uv_w].copy_from_slice(&vs[cur_v..cur_v + uv_w]);
        // The fancy upsampler leaves a row unfinished behind
        // (except for the very last row)
        num_lines_out -= 1;
    } else {
        // Process the very last row of even-sized picture
        if (y_end & 1) == 0 {
            upsample(
                &ys[cur_y..],
                None,
                &us[cur_u..],
                &vs[cur_v..],
                &us[cur_u..],
                &vs[cur_v..],
                &mut region[row_position(dst, 1, stride)..],
                mb_w,
            );
        }
    }
    num_lines_out
}

//------------------------------------------------------------------------------

// upstream: src/dec/io_dec.c FillAlphaPlane
fn fill_alpha_plane(dst: &mut [u8], pos: usize, w: i32, h: i32, stride: i32) {
    let width = count(w);
    for j in 0..h {
        let row = row_position(pos, j, stride);
        dst[row..row + width].fill(0xff);
    }
}

// upstream: src/dec/io_dec.c EmitAlphaYUV
fn emit_alpha_yuv(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    _p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    expected_num_lines_out: i32,
) -> i32 {
    let buf = yuva_buffer(output);
    let mb_w = count(io.mb_w);
    let mb_h = io.mb_h;
    debug_assert_eq!(expected_num_lines_out, mb_h);
    let _ = expected_num_lines_out;
    if let Some((alpha, mut alpha_pos)) = alpha_rows(rows) {
        let plane = buf.a.expect("an alpha output plane");
        let dst = row_position(plane.offset, io.mb_y, buf.a_stride);
        let region = region_mut(output, plane);
        for j in 0..mb_h {
            let row = row_position(dst, j, buf.a_stride);
            region[row..row + mb_w].copy_from_slice(&alpha[alpha_pos..alpha_pos + mb_w]);
            alpha_pos += count(io.width);
        }
    } else if let Some(plane) = buf.a {
        // the user requested alpha, but there is none, set it to opaque.
        let dst = row_position(plane.offset, io.mb_y, buf.a_stride);
        fill_alpha_plane(region_mut(output, plane), dst, io.mb_w, mb_h, buf.a_stride);
    }
    0
}

// upstream: src/dec/io_dec.c GetAlphaSourceRow
/// Returns the first row to blend, moves `alpha` back one row where C does, and
/// sets `num_rows`.
fn get_alpha_source_row(io: &VP8Io, alpha: &mut usize, num_rows: &mut i32) -> i32 {
    let mut start_y = io.mb_y;
    *num_rows = io.mb_h;

    // Compensate for the 1-line delay of the fancy upscaler.
    // This is similar to EmitFancyRGB().
    if io.fancy_upsampling {
        if start_y == 0 {
            // We don't process the last row yet. It'll be done during the next call.
            *num_rows -= 1;
        } else {
            start_y -= 1;
            // Fortunately, *alpha data is persistent, so we can go back
            // one row and finish alpha blending, now that the fancy upscaler
            // completed the YUV->RGB interpolation.
            *alpha = row_position(*alpha, -1, io.width);
        }
        if io.crop_top + io.mb_y + io.mb_h == io.crop_bottom {
            // If it's the very last call, we process all the remaining rows!
            *num_rows = io.crop_bottom - io.crop_top - start_y;
        }
    }
    start_y
}

// upstream: src/dec/io_dec.c EmitAlphaRGB
fn emit_alpha_rgb(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    _p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    expected_num_lines_out: i32,
) -> i32 {
    if let Some((alpha, mut alpha_pos)) = alpha_rows(rows) {
        let mb_w = io.mb_w;
        let colorspace = output.colorspace;
        let alpha_first =
            colorspace == WEBP_CSP_MODE::MODE_ARGB || colorspace == WEBP_CSP_MODE::MODE_Argb;
        let buf = rgba_buffer(output);
        let mut num_rows = 0;
        let start_y = get_alpha_source_row(io, &mut alpha_pos, &mut num_rows);
        let base_rgba = row_position(buf.rgba.offset, start_y, buf.stride);
        let dst = base_rgba + if alpha_first { 0 } else { 3 };
        let region = region_mut(output, buf.rgba);
        let has_alpha = dispatch_alpha(
            alpha, alpha_pos, io.width, mb_w, num_rows, region, dst, buf.stride,
        );
        debug_assert_eq!(expected_num_lines_out, num_rows);
        let _ = expected_num_lines_out;
        // has_alpha is true if there's non-trivial alpha to premultiply with.
        if has_alpha && webp_is_premultiplied_mode(colorspace) {
            apply_alpha_multiply(region, base_rgba, alpha_first, mb_w, num_rows, buf.stride);
        }
    }
    0
}

// upstream: src/dec/io_dec.c EmitAlphaRGBA4444
fn emit_alpha_rgba4444(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    _p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    expected_num_lines_out: i32,
) -> i32 {
    if let Some((alpha, mut alpha_pos)) = alpha_rows(rows) {
        let mb_w = count(io.mb_w);
        let colorspace = output.colorspace;
        let buf = rgba_buffer(output);
        let mut num_rows = 0;
        let start_y = get_alpha_source_row(io, &mut alpha_pos, &mut num_rows);
        let base_rgba = row_position(buf.rgba.offset, start_y, buf.stride);
        // WEBP_SWAP_16BIT_CSP is 0: alpha_dst = base_rgba + 1.
        let alpha_dst = base_rgba + 1;
        let mut alpha_mask: u32 = 0x0f;
        let region = region_mut(output, buf.rgba);
        for j in 0..num_rows {
            let row = row_position(alpha_dst, j, buf.stride);
            for i in 0..mb_w {
                // Fill in the alpha value (converted to 4 bits).
                let alpha_value = u32::from(alpha[alpha_pos + i] >> 4);
                let byte = &mut region[row + 2 * i];
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "C stores the 4-bit value in the uint8_t"
                )]
                {
                    *byte = (*byte & 0xf0) | alpha_value as u8;
                }
                alpha_mask &= alpha_value;
            }
            alpha_pos += count(io.width);
        }
        debug_assert_eq!(expected_num_lines_out, num_rows);
        let _ = expected_num_lines_out;
        if alpha_mask != 0x0f && webp_is_premultiplied_mode(colorspace) {
            apply_alpha_multiply4444(region, base_rgba, io.mb_w, num_rows, buf.stride);
        }
    }
    0
}

//------------------------------------------------------------------------------
// YUV rescaling (no final RGB conversion needed)

// upstream: src/dec/io_dec.c Rescale
/// Imports `new_lines` rows of `src`, `src_stride` bytes apart, into `wrk` and
/// exports the rows that are ready into `dst`, the buffer that C gave to
/// `WebPRescalerInit`. Returns the number of rows written.
fn rescale(
    src: &[u8],
    src_stride: i32,
    new_lines: i32,
    wrk: &mut WebPRescaler,
    dst: &mut [u8],
) -> i32 {
    let mut src_pos = 0usize;
    let mut new_lines = new_lines;
    let mut num_lines_out = 0;
    while new_lines > 0 {
        // import new contributions of source rows.
        let lines_in = webp_rescaler_import(wrk, new_lines, &src[src_pos..], src_stride);
        src_pos = row_position(src_pos, lines_in, src_stride);
        new_lines -= lines_in;
        num_lines_out += webp_rescaler_export(wrk, dst); // emit output row(s)
    }
    num_lines_out
}

// upstream: src/dec/io_dec.c EmitRescaledYUV
fn emit_rescaled_yuv(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let mb_h = io.mb_h;
    let uv_mb_h = (mb_h + 1) >> 1;
    let buf = yuva_buffer(output);
    if webp_is_alpha_mode(output.colorspace)
        && let Some((alpha, alpha_pos)) = rows.a.map(|plane| (plane, rows.a_offset))
    {
        // Before rescaling, we premultiply the luma directly into the io->y
        // internal buffer. This is OK since these samples are not used for
        // intra-prediction (the top samples are saved in cache_y/u/v).
        // But we need to cast the const away, though.
        webp_mult_rows(
            rows.y,
            io.y_stride,
            &alpha[alpha_pos..],
            io.width,
            io.mb_w,
            mb_h,
            false,
        );
    }
    let scaler_y = p.scaler_y.as_mut().expect("the luma rescaler is set up");
    let num_lines_out = rescale(
        rows.y,
        io.y_stride,
        mb_h,
        scaler_y,
        region_mut(output, buf.y),
    );
    let scaler_u = p.scaler_u.as_mut().expect("the U rescaler is set up");
    rescale(
        rows.u,
        io.uv_stride,
        uv_mb_h,
        scaler_u,
        region_mut(output, buf.u),
    );
    let scaler_v = p.scaler_v.as_mut().expect("the V rescaler is set up");
    rescale(
        rows.v,
        io.uv_stride,
        uv_mb_h,
        scaler_v,
        region_mut(output, buf.v),
    );
    num_lines_out
}

// upstream: src/dec/io_dec.c EmitRescaledAlphaYUV
fn emit_rescaled_alpha_yuv(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    expected_num_lines_out: i32,
) -> i32 {
    let buf = yuva_buffer(output);
    let plane_a = buf.a.expect("an alpha output plane");
    let dst_a = row_position(plane_a.offset, p.last_y, buf.a_stride);
    if let Some((alpha, alpha_pos)) = alpha_rows(rows) {
        let dst_y = row_position(buf.y.offset, p.last_y, buf.y_stride);
        let scaler_a = p.scaler_a.as_mut().expect("the alpha rescaler is set up");
        let num_lines_out = rescale(
            &alpha[alpha_pos..],
            io.width,
            io.mb_h,
            scaler_a,
            region_mut(output, plane_a),
        );
        debug_assert_eq!(expected_num_lines_out, num_lines_out);
        if num_lines_out > 0 {
            // unmultiply the Y
            let width = scaler_a.dst_width;
            // C reads the alpha rows from the output while it writes the luma
            // rows; the two planes may share one region here, so the alpha
            // rows are copied first.
            let alpha_region = region_mut(output, plane_a);
            let alpha_rows: Vec<Vec<u8>> = (0..num_lines_out)
                .map(|j| {
                    let row = row_position(dst_a, j, buf.a_stride);
                    alpha_region[row..row + count(width)].to_vec()
                })
                .collect();
            let y_region = region_mut(output, buf.y);
            if buf.y_stride >= 0 {
                let packed: Vec<u8> = alpha_rows.concat();
                webp_mult_rows(
                    &mut y_region[dst_y..],
                    buf.y_stride,
                    &packed,
                    width,
                    width,
                    num_lines_out,
                    true,
                );
            } else {
                for (j, row) in (0..num_lines_out).zip(&alpha_rows) {
                    let y_row = row_position(dst_y, j, buf.y_stride);
                    webp_mult_row(&mut y_region[y_row..], row, width, true);
                }
            }
        }
    } else {
        // the user requested alpha, but there is none, set it to opaque.
        debug_assert!(p.last_y + expected_num_lines_out <= io.scaled_height);
        fill_alpha_plane(
            region_mut(output, plane_a),
            dst_a,
            io.scaled_width,
            expected_num_lines_out,
            buf.a_stride,
        );
    }
    0
}

/// `size_of::<rescaler_t>()` as the C `sizeof(*work)`.
const RESCALER_T_SIZE: u64 = size_of::<rescaler_t>() as u64;

/// The C size test of the one block that `InitYUVRescaler` and `InitRGBRescaler`
/// allocate (`WebPSafeMalloc(1ULL, total_size)`), with the reservation that a
/// refused host allocation fails, before the parts are allocated separately
/// (ADR 0003, "Allocation limits").
fn reserve_rescaler_memory(total_size: u64) -> bool {
    if !check_size_overflow(total_size) {
        return false;
    }
    webp_safe_malloc::<u8>(total_size, 1).is_some()
}

/// A rescaler whose rows go to `dst_pos` of the buffer that its exports
/// receive, as C passes the destination pointer to `WebPRescalerInit`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C parameters of WebPRescalerInit"
)]
fn init_rescaler(
    src_width: i32,
    src_height: i32,
    dst_pos: usize,
    dst_width: i32,
    dst_height: i32,
    dst_stride: i32,
    num_channels: i32,
    work_size: usize,
) -> Option<WebPRescaler> {
    let mut rescaler = webp_rescaler_init(
        src_width,
        src_height,
        dst_width,
        dst_height,
        dst_stride,
        num_channels,
        vec![0; work_size],
    )?;
    rescaler.dst_pos = dst_pos;
    Some(rescaler)
}

// upstream: src/dec/io_dec.c InitYUVRescaler
fn init_yuv_rescaler(io: &VP8Io, p: &mut WebPDecParams<'_>, output: &WebPDecBuffer<'_>) -> bool {
    let has_alpha = webp_is_alpha_mode(output.colorspace);
    let buf = yuva_buffer(output);
    let out_width = io.scaled_width;
    let out_height = io.scaled_height;
    let uv_out_width = (out_width + 1) >> 1;
    let uv_out_height = (out_height + 1) >> 1;
    let uv_in_width = (io.mb_w + 1) >> 1;
    let uv_in_height = (io.mb_h + 1) >> 1;
    // scratch memory for luma rescaler
    let work_size = 2 * count(out_width);
    let uv_work_size = 2 * count(uv_out_width); // and for each u/v ones
    let num_rescalers: u64 = if has_alpha { 4 } else { 3 };

    let mut total_size = (work_size as u64 + 2 * uv_work_size as u64) * RESCALER_T_SIZE;
    if has_alpha {
        total_size += work_size as u64 * RESCALER_T_SIZE;
    }
    let rescaler_size = num_rescalers * WebPRescaler::C_SIZE as u64 + WEBP_ALIGN_CST as u64;
    total_size += rescaler_size;
    if !reserve_rescaler_memory(total_size) {
        return false; // memory error
    }
    // C: p->memory holds the work areas and the rescalers, which own them here.

    let (Some(scaler_y), Some(scaler_u), Some(scaler_v)) = (
        init_rescaler(
            io.mb_w,
            io.mb_h,
            buf.y.offset,
            out_width,
            out_height,
            buf.y_stride,
            1,
            work_size,
        ),
        init_rescaler(
            uv_in_width,
            uv_in_height,
            buf.u.offset,
            uv_out_width,
            uv_out_height,
            buf.u_stride,
            1,
            uv_work_size,
        ),
        init_rescaler(
            uv_in_width,
            uv_in_height,
            buf.v.offset,
            uv_out_width,
            uv_out_height,
            buf.v_stride,
            1,
            uv_work_size,
        ),
    ) else {
        return false;
    };
    p.scaler_y = Some(scaler_y);
    p.scaler_u = Some(scaler_u);
    p.scaler_v = Some(scaler_v);
    p.scaler_a = None;
    p.emit = Some(emit_rescaled_yuv);

    if has_alpha {
        let plane_a = buf.a.expect("an alpha output plane");
        let Some(scaler_a) = init_rescaler(
            io.mb_w,
            io.mb_h,
            plane_a.offset,
            out_width,
            out_height,
            buf.a_stride,
            1,
            work_size,
        ) else {
            return false;
        };
        p.scaler_a = Some(scaler_a);
        p.emit_alpha = Some(emit_rescaled_alpha_yuv);
        // upstream: src/dsp/alpha_processing.c WebPInitAlphaProcessing (omitted, ADR 0003)
    }
    true
}

//------------------------------------------------------------------------------
// RGBA rescaling

// upstream: src/dec/io_dec.c ExportRGB
fn export_rgb(p: &mut WebPDecParams<'_>, output: &mut WebPDecBuffer<'_>, y_pos: i32) -> i32 {
    let colorspace = output.colorspace;
    let output_height = output.height;
    let buf = rgba_buffer(output);
    let dst = row_position(buf.rgba.offset, y_pos, buf.stride);
    let mut num_lines_out = 0;
    let WebPDecParams {
        scaler_y,
        scaler_u,
        scaler_v,
        memory,
        ..
    } = p;
    let scaler_y = scaler_y.as_mut().expect("the luma rescaler is set up");
    let scaler_u = scaler_u.as_mut().expect("the U rescaler is set up");
    let scaler_v = scaler_v.as_mut().expect("the V rescaler is set up");
    let region = region_mut(output, buf.rgba);
    // For RGB rescaling, because of the YUV420, current scan position
    // U/V can be +1/-1 line from the Y one.  Hence the double test.
    while webp_rescaler_has_pending_output(scaler_y) && webp_rescaler_has_pending_output(scaler_u) {
        debug_assert!(y_pos + num_lines_out < output_height);
        debug_assert_eq!(scaler_u.y_accum, scaler_v.y_accum);
        webp_rescaler_export_row(scaler_y, memory);
        webp_rescaler_export_row(scaler_u, memory);
        webp_rescaler_export_row(scaler_v, memory);
        webp_yuv444_converters(
            colorspace,
            &memory[scaler_y.dst_pos..],
            &memory[scaler_u.dst_pos..],
            &memory[scaler_v.dst_pos..],
            &mut region[row_position(dst, num_lines_out, buf.stride)..],
            scaler_y.dst_width,
        );
        num_lines_out += 1;
    }
    num_lines_out
}

// upstream: src/dec/io_dec.c EmitRescaledRGB
fn emit_rescaled_rgb(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> i32 {
    let mb_h = io.mb_h;
    let uv_mb_h = (mb_h + 1) >> 1;
    let mut j = 0;
    let mut uv_j = 0;
    let mut num_lines_out = 0;
    while j < mb_h {
        let scaler_y = p.scaler_y.as_mut().expect("the luma rescaler is set up");
        let y_lines_in = webp_rescaler_import(
            scaler_y,
            mb_h - j,
            &rows.y[row_position(0, j, io.y_stride)..],
            io.y_stride,
        );
        j += y_lines_in;
        let scaler_u = p.scaler_u.as_mut().expect("the U rescaler is set up");
        if webp_rescale_needed_lines(scaler_u, uv_mb_h - uv_j) != 0 {
            let u_lines_in = webp_rescaler_import(
                scaler_u,
                uv_mb_h - uv_j,
                &rows.u[row_position(0, uv_j, io.uv_stride)..],
                io.uv_stride,
            );
            let scaler_v = p.scaler_v.as_mut().expect("the V rescaler is set up");
            let v_lines_in = webp_rescaler_import(
                scaler_v,
                uv_mb_h - uv_j,
                &rows.v[row_position(0, uv_j, io.uv_stride)..],
                io.uv_stride,
            );
            let _ = v_lines_in; // remove a gcc warning
            debug_assert_eq!(u_lines_in, v_lines_in);
            uv_j += u_lines_in;
        }
        num_lines_out += export_rgb(p, output, p.last_y + num_lines_out);
    }
    num_lines_out
}

// upstream: src/dec/io_dec.c ExportAlpha
fn export_alpha(
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    y_pos: i32,
    max_lines_out: i32,
) -> i32 {
    let buf = rgba_buffer(output);
    let base_rgba = row_position(buf.rgba.offset, y_pos, buf.stride);
    let colorspace = output.colorspace;
    let alpha_first =
        colorspace == WEBP_CSP_MODE::MODE_ARGB || colorspace == WEBP_CSP_MODE::MODE_Argb;
    let dst = base_rgba + if alpha_first { 0 } else { 3 };
    let mut num_lines_out = 0;
    let is_premult_alpha = webp_is_premultiplied_mode(colorspace);
    let mut non_opaque = false;
    let output_height = output.height;
    let WebPDecParams {
        scaler_a, memory, ..
    } = p;
    let scaler_a = scaler_a.as_mut().expect("the alpha rescaler is set up");
    let width = scaler_a.dst_width;
    let region = region_mut(output, buf.rgba);

    while webp_rescaler_has_pending_output(scaler_a) && num_lines_out < max_lines_out {
        debug_assert!(y_pos + num_lines_out < output_height);
        webp_rescaler_export_row(scaler_a, memory);
        let row = row_position(dst, num_lines_out, buf.stride);
        non_opaque |= webp_dispatch_alpha(
            &memory[scaler_a.dst_pos..],
            0,
            width,
            1,
            &mut region[row..],
            0,
        );
        num_lines_out += 1;
    }
    if is_premult_alpha && non_opaque {
        apply_alpha_multiply(
            region,
            base_rgba,
            alpha_first,
            width,
            num_lines_out,
            buf.stride,
        );
    }
    num_lines_out
}

// upstream: src/dec/io_dec.c ExportAlphaRGBA4444
fn export_alpha_rgba4444(
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    y_pos: i32,
    max_lines_out: i32,
) -> i32 {
    let buf = rgba_buffer(output);
    let base_rgba = row_position(buf.rgba.offset, y_pos, buf.stride);
    // WEBP_SWAP_16BIT_CSP is 0: alpha_dst = base_rgba + 1.
    let alpha_dst = base_rgba + 1;
    let mut num_lines_out = 0;
    let colorspace = output.colorspace;
    let is_premult_alpha = webp_is_premultiplied_mode(colorspace);
    let mut alpha_mask: u32 = 0x0f;
    let output_height = output.height;
    let WebPDecParams {
        scaler_a, memory, ..
    } = p;
    let scaler_a = scaler_a.as_mut().expect("the alpha rescaler is set up");
    let width = scaler_a.dst_width;
    let region = region_mut(output, buf.rgba);

    while webp_rescaler_has_pending_output(scaler_a) && num_lines_out < max_lines_out {
        debug_assert!(y_pos + num_lines_out < output_height);
        webp_rescaler_export_row(scaler_a, memory);
        let src = scaler_a.dst_pos;
        let row = row_position(alpha_dst, num_lines_out, buf.stride);
        for i in 0..count(width) {
            // Fill in the alpha value (converted to 4 bits).
            let alpha_value = u32::from(memory[src + i] >> 4);
            let byte = &mut region[row + 2 * i];
            #[expect(
                clippy::cast_possible_truncation,
                reason = "C stores the 4-bit value in the uint8_t"
            )]
            {
                *byte = (*byte & 0xf0) | alpha_value as u8;
            }
            alpha_mask &= alpha_value;
        }
        num_lines_out += 1;
    }
    if is_premult_alpha && alpha_mask != 0x0f {
        apply_alpha_multiply4444(region, base_rgba, width, num_lines_out, buf.stride);
    }
    num_lines_out
}

// upstream: src/dec/io_dec.c EmitRescaledAlphaRGB
fn emit_rescaled_alpha_rgb(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
    expected_num_out_lines: i32,
) -> i32 {
    if let Some((alpha, alpha_pos)) = alpha_rows(rows) {
        let mut lines_left = expected_num_out_lines;
        let y_end = p.last_y + lines_left;
        let emit_alpha_row = p.emit_alpha_row.expect("the alpha row emitter is set up");
        while lines_left > 0 {
            let scaler = p.scaler_a.as_mut().expect("the alpha rescaler is set up");
            let row_offset = i64::from(scaler.src_y) - i64::from(io.mb_y);
            let src = alpha_pos
                .checked_add_signed(
                    isize::try_from(row_offset * i64::from(io.width)).expect("the offset fits"),
                )
                .expect("the alpha row starts inside the plane");
            webp_rescaler_import(
                scaler,
                io.mb_h + io.mb_y - scaler.src_y,
                &alpha[src..],
                io.width,
            );
            lines_left -= emit_alpha_row(p, output, y_end - lines_left, lines_left);
        }
    }
    0
}

// upstream: src/dec/io_dec.c InitRGBRescaler
fn init_rgb_rescaler(io: &VP8Io, p: &mut WebPDecParams<'_>, output: &WebPDecBuffer<'_>) -> bool {
    let has_alpha = webp_is_alpha_mode(output.colorspace);
    let out_width = io.scaled_width;
    let out_height = io.scaled_height;
    let uv_in_width = (io.mb_w + 1) >> 1;
    let uv_in_height = (io.mb_h + 1) >> 1;
    // scratch memory for one rescaler
    let work_size = 2 * count(out_width);
    let num_rescalers: u64 = if has_alpha { 4 } else { 3 };

    let tmp_size1 = num_rescalers * work_size as u64;
    let tmp_size2 = num_rescalers * u64::from(out_width.unsigned_abs());
    let mut total_size = tmp_size1 * RESCALER_T_SIZE + tmp_size2;
    let rescaler_size = num_rescalers * WebPRescaler::C_SIZE as u64 + WEBP_ALIGN_CST as u64;
    total_size += rescaler_size;
    if !reserve_rescaler_memory(total_size) {
        return false; // memory error
    }
    // C: p->memory holds the work areas, the tmp storage for scaled YUV444
    // samples before RGB conversion and the rescalers; here it holds the tmp
    // storage, and the rescalers own their work areas.
    p.memory = vec![0; usize::try_from(tmp_size2).expect("an accepted size fits")];
    let width = count(out_width);

    let (Some(scaler_y), Some(scaler_u), Some(scaler_v)) = (
        init_rescaler(io.mb_w, io.mb_h, 0, out_width, out_height, 0, 1, work_size),
        init_rescaler(
            uv_in_width,
            uv_in_height,
            width,
            out_width,
            out_height,
            0,
            1,
            work_size,
        ),
        init_rescaler(
            uv_in_width,
            uv_in_height,
            2 * width,
            out_width,
            out_height,
            0,
            1,
            work_size,
        ),
    ) else {
        return false;
    };
    p.scaler_y = Some(scaler_y);
    p.scaler_u = Some(scaler_u);
    p.scaler_v = Some(scaler_v);
    p.scaler_a = None;
    p.emit = Some(emit_rescaled_rgb);
    // upstream: src/dsp/upsampling.c WebPInitYUV444Converters (omitted, ADR 0003)

    if has_alpha {
        let Some(scaler_a) = init_rescaler(
            io.mb_w,
            io.mb_h,
            3 * width,
            out_width,
            out_height,
            0,
            1,
            work_size,
        ) else {
            return false;
        };
        p.scaler_a = Some(scaler_a);
        p.emit_alpha = Some(emit_rescaled_alpha_rgb);
        p.emit_alpha_row = Some(
            if output.colorspace == WEBP_CSP_MODE::MODE_RGBA_4444
                || output.colorspace == WEBP_CSP_MODE::MODE_rgbA_4444
            {
                export_alpha_rgba4444
            } else {
                export_alpha
            },
        );
        // upstream: src/dsp/alpha_processing.c WebPInitAlphaProcessing (omitted, ADR 0003)
    }
    true
}

//------------------------------------------------------------------------------
// Default custom functions

// upstream: src/dec/io_dec.c CustomSetup
fn custom_setup(io: &mut VP8Io, p: &mut WebPDecParams<'_>, output: &WebPDecBuffer<'_>) -> bool {
    let colorspace = output.colorspace;
    let is_rgb = webp_is_rgb_mode(colorspace);
    let is_alpha = webp_is_alpha_mode(colorspace);

    p.memory = Vec::new();
    p.emit = None;
    p.emit_alpha = None;
    p.emit_alpha_row = None;
    if !webp_io_init_from_options(
        p.options,
        io,
        if is_alpha {
            WEBP_CSP_MODE::MODE_YUV
        } else {
            WEBP_CSP_MODE::MODE_YUVA
        },
    ) {
        return false;
    }
    if is_alpha && webp_is_premultiplied_mode(colorspace) {
        // upstream: src/dsp/upsampling.c WebPInitUpsamplers (omitted, ADR 0003)
    }
    if io.use_scaling {
        let ok = if is_rgb {
            init_rgb_rescaler(io, p, output)
        } else {
            init_yuv_rescaler(io, p, output)
        };
        if !ok {
            return false; // memory error
        }
    } else {
        if is_rgb {
            // upstream: src/dsp/yuv.c WebPInitSamplers (omitted, ADR 0003)
            p.emit = Some(emit_sampled_rgb); // default
            if io.fancy_upsampling {
                let uv_width = (io.mb_w + 1) >> 1;
                // C: WebPSafeMalloc(1ULL, (size_t)(io->mb_w + 2 * uv_width)); the
                // test of one element of that size makes the same decision.
                let size = count(io.mb_w + 2 * uv_width);
                let Some(mut memory) = webp_safe_malloc::<u8>(size as u64, 1) else {
                    return false; // memory error.
                };
                memory.resize(size, 0);
                p.memory = memory;
                p.tmp_y = 0;
                p.tmp_u = p.tmp_y + count(io.mb_w);
                p.tmp_v = p.tmp_u + count(uv_width);
                p.emit = Some(emit_fancy_rgb);
                // upstream: src/dsp/upsampling.c WebPInitUpsamplers (omitted, ADR 0003)
            }
        } else {
            p.emit = Some(emit_yuv);
        }
        if is_alpha {
            // need transparency output
            p.emit_alpha = Some(
                if colorspace == WEBP_CSP_MODE::MODE_RGBA_4444
                    || colorspace == WEBP_CSP_MODE::MODE_rgbA_4444
                {
                    emit_alpha_rgba4444
                } else if is_rgb {
                    emit_alpha_rgb
                } else {
                    emit_alpha_yuv
                },
            );
            if is_rgb {
                // upstream: src/dsp/alpha_processing.c WebPInitAlphaProcessing (omitted, ADR 0003)
            }
        }
    }

    true
}

//------------------------------------------------------------------------------

// upstream: src/dec/io_dec.c CustomPut
fn custom_put(
    io: &VP8Io,
    rows: &mut VP8IoRows<'_>,
    p: &mut WebPDecParams<'_>,
    output: &mut WebPDecBuffer<'_>,
) -> bool {
    let mb_w = io.mb_w;
    let mb_h = io.mb_h;
    debug_assert_eq!((io.mb_y & 1), 0);

    if mb_w <= 0 || mb_h <= 0 {
        return false;
    }
    let emit = p.emit.expect("setup sets the emit function");
    let num_lines_out = emit(io, rows, p, output);
    if let Some(emit_alpha) = p.emit_alpha {
        emit_alpha(io, rows, p, output, num_lines_out);
    }
    p.last_y += num_lines_out;
    true
}

//------------------------------------------------------------------------------

// upstream: src/dec/io_dec.c CustomTeardown
/// Frees the work memory. C frees `p->memory`, which also holds the rescalers
/// and their work areas; here the rescalers own them and are dropped with it.
fn custom_teardown(p: &mut WebPDecParams<'_>) {
    p.memory = Vec::new();
    p.scaler_y = None;
    p.scaler_u = None;
    p.scaler_v = None;
    p.scaler_a = None;
}

//------------------------------------------------------------------------------
// Main entry point

/// The custom hooks of `WebPInitCustomIo` with their user data: C's
/// `io->opaque = params`, and the output buffer that `params->output` points at
/// in C (ADR 0003, "Decoding context per call"). The parameters and the output
/// carry separate lifetimes, because `WebPDecode` borrows the options of a
/// configuration for one call while its output buffer borrows the caller's
/// memory for longer (WP12). This type has no C counterpart.
pub struct CustomIo<'p, 'o, 'a> {
    /// The decoding parameters (C: `io->opaque`).
    pub params: &'p mut WebPDecParams<'o>,
    /// The output buffer (C: `params->output`).
    pub output: &'p mut WebPDecBuffer<'a>,
}

impl VP8IoHooks for CustomIo<'_, '_, '_> {
    // upstream: src/dec/io_dec.c CustomSetup
    fn setup(&mut self, io: &mut VP8Io) -> bool {
        custom_setup(io, self.params, self.output)
    }

    // upstream: src/dec/io_dec.c CustomPut
    fn put(&mut self, io: &VP8Io, rows: &mut VP8IoRows<'_>) -> bool {
        custom_put(io, rows, self.params, self.output)
    }

    // upstream: src/dec/io_dec.c CustomTeardown
    fn teardown(&mut self, _io: &VP8Io) {
        custom_teardown(self.params);
    }
}

// upstream: src/dec/io_dec.c WebPInitCustomIo
/// Plugs the custom functions of this module into `io`: C sets `io->put`,
/// `io->setup`, `io->teardown` and `io->opaque = params`. `VP8Io` has no hook
/// fields (ADR 0003), so the hooks are returned, to be passed to the decoding
/// calls, and `io` is left as it is.
pub fn webp_init_custom_io<'p, 'o, 'a>(
    params: &'p mut WebPDecParams<'o>,
    output: &'p mut WebPDecBuffer<'a>,
    _io: &mut VP8Io,
) -> CustomIo<'p, 'o, 'a> {
    CustomIo { params, output }
}
