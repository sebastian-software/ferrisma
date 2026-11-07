// libwebp v1.6.0, src/utils/rescaler_utils.c:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Rescaling functions
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/rescaler_utils.h:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Rescaling functions
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/utils/rescaler_utils.c` and `src/utils/rescaler_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::dsp::rescaler::{webp_rescaler_export_row, webp_rescaler_import_row};
use crate::utils::utils::check_size_overflow;

// upstream: src/utils/rescaler_utils.h WEBP_RESCALER_RFIX
/// Fixed-point precision of the multiplications.
pub const WEBP_RESCALER_RFIX: u32 = 32;
// upstream: src/utils/rescaler_utils.h WEBP_RESCALER_ONE
/// The value 1.0 in the fixed-point precision.
pub const WEBP_RESCALER_ONE: u64 = 1 << WEBP_RESCALER_RFIX;

// upstream: src/utils/rescaler_utils.h rescaler_t
/// Type of the side buffers of the rescaler.
#[expect(non_camel_case_types, reason = "C type name")]
pub type rescaler_t = u32;

// upstream: src/utils/rescaler_utils.h WEBP_RESCALER_FRAC
/// `x / y` in the fixed-point precision, truncated to 32 bits as the C macro
/// truncates it.
///
/// # Panics
///
/// Panics when `y` is not positive, where the C division is undefined.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C macro converts its numerator to uint64_t and keeps the low 32 bits"
)]
pub fn webp_rescaler_frac(x: i32, y: i32) -> u32 {
    let x = x as u64;
    ((x << WEBP_RESCALER_RFIX) / u64::try_from(y).expect("the divisor is positive")) as u32
}

// upstream: src/utils/rescaler_utils.h WebPRescaler
/// Structure used for on-the-fly rescaling.
///
/// The side rows `irow` and `frow` are owned: `webp_rescaler_init` takes them
/// from the work vector it receives, and the import swaps them as the rows
/// advance. The output buffer is not held. `dst_pos` is the start of the row
/// that the next export writes, which the C pointer `dst` denotes, and each
/// export receives the buffer it writes into. `webp_rescaler_init` starts it at
/// 0; an owner whose first output row is elsewhere in the buffer, such as a
/// flipped plane with a negative `dst_stride`, sets it to that row's start.
pub struct WebPRescaler {
    /// true if we're expanding in the x direction
    pub x_expand: bool,
    /// true if we're expanding in the y direction
    pub y_expand: bool,
    /// bytes to jump between pixels
    pub num_channels: i32,
    /// fixed-point scaling factors
    pub fx_scale: u32,
    pub fy_scale: u32,
    pub fxy_scale: u32,
    /// vertical accumulator
    pub y_accum: i32,
    /// vertical increments
    pub y_add: i32,
    pub y_sub: i32,
    /// horizontal increments
    pub x_add: i32,
    pub x_sub: i32,
    /// source dimensions
    pub src_width: i32,
    pub src_height: i32,
    /// destination dimensions
    pub dst_width: i32,
    pub dst_height: i32,
    /// row counters for input and output
    pub src_y: i32,
    pub dst_y: i32,
    /// the start of the output row to write next, and the stride between rows
    pub dst_pos: usize,
    pub dst_stride: i32,
    /// the two side rows, owned by the rescaler
    pub(crate) irow: Vec<rescaler_t>,
    pub(crate) frow: Vec<rescaler_t>,
}

impl WebPRescaler {
    /// The C `sizeof(WebPRescaler)`: seventeen 32-bit fields, then the pointer
    /// `dst`, the `int` `dst_stride` and the pointers `irow` and `frow`, each
    /// aligned as C aligns it, and the whole padded to the pointer alignment.
    /// This is 104 on 64-bit targets and 84 on 32-bit targets.
    pub const C_SIZE: usize = {
        let pointer = size_of::<usize>();
        let after_dst = (17 * 4_usize).next_multiple_of(pointer) + pointer + 4;
        (after_dst.next_multiple_of(pointer) + 2 * pointer).next_multiple_of(pointer)
    };
}

// upstream: src/utils/rescaler_utils.c WebPRescalerInit
/// Initializes a rescaler that writes output rows `dst_stride` bytes apart. The
/// work vector holds the two side rows, `num_channels * dst_width` elements
/// each, and the rescaler takes ownership of it. The side rows are zeroed, as
/// the C `memset` zeroes them. Returns `None` when the work size does not fit
/// in the address space, which the C code reports as 0.
///
/// # Panics
///
/// Panics when `dst_width` or `num_channels` is not positive, which the C code
/// does not define, or when `work` does not hold exactly the two side rows.
#[expect(
    clippy::similar_names,
    reason = "the C names fxy_scale, fy_scale and fx_scale of the scaling factors"
)]
#[must_use]
pub fn webp_rescaler_init(
    src_width: i32,
    src_height: i32,
    dst_width: i32,
    dst_height: i32,
    dst_stride: i32,
    num_channels: i32,
    work: Vec<rescaler_t>,
) -> Option<WebPRescaler> {
    let x_add = src_width;
    let x_sub = dst_width;
    let y_add = src_height;
    let y_sub = dst_height;
    let row_len = usize::try_from(num_channels * dst_width).expect("the row length is positive");
    let total_size = 2_u64
        * u64::try_from(dst_width).expect("dst_width is positive")
        * u64::try_from(num_channels).expect("the channels are positive")
        * 4;
    if !check_size_overflow(total_size) {
        return None;
    }
    assert!(
        work.len() == 2 * row_len,
        "the work holds {} elements, and the two side rows need {}",
        work.len(),
        2 * row_len
    );
    let x_expand = src_width < dst_width;
    let y_expand = src_height < dst_height;
    let x_add_out = if x_expand { x_sub - 1 } else { x_add };
    let x_sub_out = if x_expand { x_add - 1 } else { x_sub };
    let fx_scale = if x_expand {
        0
    } else {
        webp_rescaler_frac(1, x_sub_out)
    };
    // vertical scaling parameters
    let y_add_out = if y_expand { y_add - 1 } else { y_add };
    let y_sub_out = if y_expand { y_sub - 1 } else { y_sub };
    let y_accum = if y_expand { y_sub_out } else { y_add_out };
    let (fy_scale, fxy_scale) = if y_expand {
        (webp_rescaler_frac(1, x_add_out), 0)
    } else {
        // This is WEBP_RESCALER_FRAC(dst_height, x_add * y_add) without the cast.
        // Its value is <= WEBP_RESCALER_ONE, because dst_height <= rescaler->y_add
        // and rescaler->x_add >= 1; a ratio that does not fit in 32 bits is the
        // special case of WebPRescalerExportRow, which stores 0.
        let num = u64::try_from(dst_height).expect("dst_height is positive") * WEBP_RESCALER_ONE;
        let den = u64::try_from(x_add_out).expect("x_add is positive")
            * u64::try_from(y_add).expect("src_height is positive");
        let ratio = num / den;
        let fxy_ratio = u32::try_from(ratio).unwrap_or(0);
        (webp_rescaler_frac(1, y_sub_out), fxy_ratio)
    };
    // The C code points `irow` at the start of `work` and `frow` at the element
    // that follows the first row; the split gives the same two rows.
    let mut irow = work;
    let mut frow = irow.split_off(row_len);
    irow.fill(0);
    frow.fill(0);
    Some(WebPRescaler {
        x_expand,
        y_expand,
        num_channels,
        fx_scale,
        fy_scale,
        fxy_scale,
        y_accum,
        y_add: y_add_out,
        y_sub: y_sub_out,
        x_add: x_add_out,
        x_sub: x_sub_out,
        src_width,
        src_height,
        dst_width,
        dst_height,
        src_y: 0,
        dst_y: 0,
        dst_pos: 0,
        dst_stride,
        irow,
        frow,
    })
}

// upstream: src/utils/rescaler_utils.c WebPRescalerGetScaledDimensions
/// Completes a missing scaled dimension, zero, from the other one, preserving
/// the aspect ratio. Returns whether both dimensions are then positive and at
/// most `INT_MAX / 2`; the arguments change only on success.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C casts of the scaled dimensions keep the low 32 bits"
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "the C code converts the signed dimensions to uint64_t"
)]
pub fn webp_rescaler_get_scaled_dimensions(
    src_width: i32,
    src_height: i32,
    scaled_width: &mut i32,
    scaled_height: &mut i32,
) -> bool {
    let mut width = *scaled_width;
    let mut height = *scaled_height;
    let max_size = i32::MAX / 2;
    // if width is unspecified, scale original proportionally to height ratio.
    if width == 0 && src_height > 0 {
        width = ((((src_width as u64).wrapping_mul(height as u64))
            .wrapping_add(src_height as u64)
            .wrapping_sub(1))
            / (src_height as u64)) as i32;
    }
    // if height is unspecified, scale original proportionally to width ratio.
    if height == 0 && src_width > 0 {
        height = ((((src_height as u64).wrapping_mul(width as u64))
            .wrapping_add(src_width as u64)
            .wrapping_sub(1))
            / (src_width as u64)) as i32;
    }
    // Check if the overall dimensions still make sense.
    if width <= 0 || height <= 0 || width > max_size || height > max_size {
        return false;
    }
    *scaled_width = width;
    *scaled_height = height;
    true
}

// upstream: src/utils/rescaler_utils.c WebPRescaleNeededLines
/// The number of input lines needed to produce the next output line, at most
/// `max_num_lines`.
#[must_use]
pub fn webp_rescale_needed_lines(rescaler: &WebPRescaler, max_num_lines: i32) -> i32 {
    let num_lines = (rescaler.y_accum + rescaler.y_sub - 1) / rescaler.y_sub;
    if num_lines > max_num_lines {
        max_num_lines
    } else {
        num_lines
    }
}

// upstream: src/utils/rescaler_utils.c WebPRescalerImport
/// Imports up to `num_lines` rows of `src`, `src_stride` bytes apart, until a
/// row is ready to export. Returns the number of rows imported.
///
/// # Panics
///
/// Panics when `src` holds fewer rows than the import reads.
pub fn webp_rescaler_import(
    rescaler: &mut WebPRescaler,
    num_lines: i32,
    src: &[u8],
    src_stride: i32,
) -> i32 {
    let mut total_imported = 0_i32;
    let mut src_pos = 0_usize;
    let row_len = usize::try_from(rescaler.num_channels * rescaler.dst_width)
        .expect("the row length is positive");
    while total_imported < num_lines && !webp_rescaler_has_pending_output(rescaler) {
        if rescaler.y_expand {
            std::mem::swap(&mut rescaler.irow, &mut rescaler.frow);
        }
        webp_rescaler_import_row(rescaler, &src[src_pos..]);
        if !rescaler.y_expand {
            // Accumulate the contribution of the new row.
            for x in 0..row_len {
                rescaler.irow[x] = rescaler.irow[x].wrapping_add(rescaler.frow[x]);
            }
        }
        rescaler.src_y += 1;
        src_pos = src_pos
            .checked_add_signed(isize::try_from(src_stride).expect("the stride fits in isize"))
            .expect("the source row starts inside the buffer");
        total_imported += 1;
        rescaler.y_accum -= rescaler.y_sub;
    }
    total_imported
}

// upstream: src/utils/rescaler_utils.c WebPRescalerExport
/// Exports every row that is ready into `dst`, the output buffer, from the row
/// `dst_pos` on. Returns the number of rows written.
///
/// # Panics
///
/// Panics when `dst` is shorter than the rows that the export writes.
pub fn webp_rescaler_export(rescaler: &mut WebPRescaler, dst: &mut [u8]) -> i32 {
    let mut total_exported = 0_i32;
    while webp_rescaler_has_pending_output(rescaler) {
        webp_rescaler_export_row(rescaler, dst);
        total_exported += 1;
    }
    total_exported
}

// upstream: src/utils/rescaler_utils.h WebPRescalerInputDone
/// Whether the input is finished.
#[must_use]
pub fn webp_rescaler_input_done(rescaler: &WebPRescaler) -> bool {
    rescaler.src_y >= rescaler.src_height
}

// upstream: src/utils/rescaler_utils.h WebPRescalerOutputDone
/// Whether the output is finished.
#[must_use]
pub fn webp_rescaler_output_done(rescaler: &WebPRescaler) -> bool {
    rescaler.dst_y >= rescaler.dst_height
}

// upstream: src/utils/rescaler_utils.h WebPRescalerHasPendingOutput
/// Whether output rows are ready.
#[must_use]
pub fn webp_rescaler_has_pending_output(rescaler: &WebPRescaler) -> bool {
    !webp_rescaler_output_done(rescaler) && rescaler.y_accum <= 0
}
