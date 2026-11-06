// libwebp v1.6.0, src/utils/quant_levels_dec_utils.c:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Implement gradient smoothing: we replace a current alpha value by its
// surrounding average if it's close enough (that is: the change will be less
// than the minimum distance between two quantized level).
// We use sliding window for computing the 2d moving average.
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/quant_levels_dec_utils.h:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Alpha plane de-quantization utility
//
// Author:  Vikas Arora (vikasa@google.com)

//! Translation of `src/utils/quant_levels_dec_utils.c` and `src/utils/quant_levels_dec_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).

use crate::utils::utils::webp_safe_malloc;

// upstream: src/utils/quant_levels_dec_utils.c FIX
/// Fixed-point precision for averaging.
const FIX: u32 = 16;
// upstream: src/utils/quant_levels_dec_utils.c LFIX
/// Extra precision for the look-up table.
const LFIX: i32 = 2;
// upstream: src/utils/quant_levels_dec_utils.c LUT_SIZE
/// Look-up table size, on each side of zero.
const LUT_SIZE: i32 = (1 << (8 + LFIX)) - 1;
// upstream: src/utils/quant_levels_dec_utils.c DFIX
/// Extra precision for ordered dithering. Dithering is not compiled
/// (`USE_DITHERING` is undefined), so the precision is zero.
const DFIX: i32 = 0;

// upstream: src/utils/quant_levels_dec_utils.c CLIP_8b_MASK
/// The bits above the byte, which `clip_8b` tests.
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C mask is an int made from an unsigned shift"
)]
const CLIP_8B_MASK: i32 = (!0_u32 << (8 + DFIX)) as i32;

// upstream: src/utils/quant_levels_dec_utils.c clip_8b
/// Clips an `int` to a byte, after the descaling by `DFIX`.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the low eight bits of the descaled value"
)]
fn clip_8b(v: i32) -> u8 {
    if (v & CLIP_8B_MASK) == 0 {
        (v >> DFIX) as u8
    } else if v < 0 {
        0
    } else {
        255
    }
}

/// The low 16 bits of a value, as a C conversion to `uint16_t` keeps them.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversion keeps the low 16 bits, as two's complement for negative values"
)]
fn wrap16(value: i32) -> u16 {
    value as u16
}

/// The low 16 bits of an unsigned value.
#[must_use]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion keeps the low 16 bits"
)]
fn low16(value: u32) -> u16 {
    value as u16
}

// upstream: src/utils/quant_levels_dec_utils.c SmoothParams
/// The state of the smoothing filter. The C scratch memory is one allocation:
/// a ring of `R` rows of accumulated sums, the output row `end` after it, the
/// horizontal averages and the correction table. The pointers of the C code
/// become offsets into the ring, the averages and the table (ADR 0003).
///
/// `src` and `dst` are the input and output rows of `data`, which the C
/// pointers `src` and `dst` denote; `dst` lags behind `src` by `radius` rows.
struct SmoothParams<'a> {
    /// dimension
    width: usize,
    height: usize,
    /// stride in bytes
    stride: usize,
    /// current input row being processed
    row: i32,
    /// the plane
    data: &'a mut [u8],
    /// offset of the input row
    src: usize,
    /// offset of the output row
    dst: usize,
    /// filter radius (=delay)
    radius: i32,
    /// normalization factor, in FIX bits precision
    scale: u32,
    /// the ring of accumulated sums, `(2 * radius + 2)` rows of `width`
    start: Vec<u16>,
    /// offsets into `start` of the current row, the previous row and the output row
    cur: usize,
    top: usize,
    end: usize,
    /// horizontal averages
    average: Vec<u16>,
    /// the correction table, of `1 + 2 * LUT_SIZE` entries centered on `LUT_SIZE`
    correction: Vec<i16>,
    /// number of quantized levels
    num_levels: i32,
    /// min and max level values
    min: i32,
    max: i32,
    /// smallest distance between two consecutive levels
    min_level_dist: i32,
}

// upstream: src/utils/quant_levels_dec_utils.c VFilter
/// Accumulates one input row vertically, and writes the vertical window sum of
/// the last rows to the output row.
fn v_filter(p: &mut SmoothParams<'_>) {
    let w = p.width;
    let mut sum: u16 = 0; // all arithmetic is modulo 16bit
    for x in 0..w {
        sum = sum.wrapping_add(u16::from(p.data[p.src + x]));
        let new_value = p.start[p.top + x].wrapping_add(sum);
        let out = p.end + x;
        p.start[out] = new_value.wrapping_sub(p.start[p.cur + x]); // vertical sum of 'r' pixels.
        p.start[p.cur + x] = new_value;
    }
    // move input pointers one row down
    p.top = p.cur;
    p.cur += w;
    if p.cur == p.end {
        p.cur = 0; // roll-over
    }
    // We replicate edges, as it's somewhat easier as a boundary condition.
    // That's why we don't update the 'src' pointer on top/bottom area:
    if p.row >= 0 && p.row < i32::try_from(p.height).expect("the height fits in int") - 1 {
        p.src += p.stride;
    }
}

// upstream: src/utils/quant_levels_dec_utils.c HFilter
/// Averages the vertical sums of the output row along the rows, with mirrored
/// edges.
#[expect(
    clippy::many_single_char_names,
    reason = "the C names x, r, w and p of the horizontal filter"
)]
fn h_filter(p: &mut SmoothParams<'_>) {
    let input = p.end;
    let scale = p.scale;
    let w = p.width;
    let r = usize::try_from(p.radius).expect("the radius is positive");
    let mut x = 0_usize;
    while x <= r {
        // left mirroring
        let delta = p.start[input + x + r - 1].wrapping_add(p.start[input + r - x]);
        p.average[x] = low16((u32::from(delta).wrapping_mul(scale)) >> FIX);
        x += 1;
    }
    while x + r < w {
        // bulk middle run
        let delta = p.start[input + x + r].wrapping_sub(p.start[input + x - r - 1]);
        p.average[x] = low16((u32::from(delta).wrapping_mul(scale)) >> FIX);
        x += 1;
    }
    while x < w {
        // right mirroring
        let a = 2 * i32::from(p.start[input + w - 1]);
        let b = i32::from(p.start[input + 2 * w - 2 - r - x]);
        let c = i32::from(p.start[input + x - r - 1]);
        let delta = wrap16(a - b - c);
        p.average[x] = low16((u32::from(delta).wrapping_mul(scale)) >> FIX);
        x += 1;
    }
}

// upstream: src/utils/quant_levels_dec_utils.c ApplyFilter
/// Emits one filtered output row: each sample that lies strictly between the
/// extreme levels moves towards its horizontal average, by the correction table.
fn apply_filter(p: &mut SmoothParams<'_>) {
    let w = p.width;
    let dst = p.dst;
    for x in 0..w {
        let v = i32::from(p.data[dst + x]);
        if v < p.max && v > p.min {
            let index = i32::from(p.average[x]) - (v << LFIX);
            let table =
                usize::try_from(LUT_SIZE + index).expect("the correction index is in the table");
            let c = (v << DFIX) + i32::from(p.correction[table]);
            p.data[dst + x] = clip_8b(c);
        }
    }
    p.dst += p.stride; // advance output pointer
}

// upstream: src/utils/quant_levels_dec_utils.c InitCorrectionLUT
/// Fills the correction table. The correction curve is `f(x) = x` up to
/// `threshold2`, `0` from `threshold1`, and linear in between, with `f(-x) =
/// -f(x)`; `threshold2` is three quarters of `threshold1`.
fn init_correction_lut(lut: &mut [i16], min_dist: i32) {
    let threshold1 = min_dist << LFIX;
    let threshold2 = (3 * threshold1) >> 2;
    let max_threshold = threshold2 << DFIX;
    let delta = threshold1 - threshold2;
    for i in 1..=LUT_SIZE {
        let c = if i <= threshold2 {
            i << DFIX
        } else if i < threshold1 {
            max_threshold * (threshold1 - i) / delta
        } else {
            0
        };
        let c = c >> LFIX;
        let plus = usize::try_from(LUT_SIZE + i).expect("the table index is positive");
        let minus = usize::try_from(LUT_SIZE - i).expect("the table index is positive");
        lut[plus] = i16::try_from(c).expect("a correction fits in 16 bits");
        lut[minus] = i16::try_from(-c).expect("a correction fits in 16 bits");
    }
    lut[usize::try_from(LUT_SIZE).expect("the table centre is positive")] = 0;
}

// upstream: src/utils/quant_levels_dec_utils.c CountLevels
/// Finds the range of the plane, its number of levels and the smallest distance
/// between two of its levels.
fn count_levels(p: &mut SmoothParams<'_>) {
    let mut used_levels = [false; 256];
    p.min = 255;
    p.max = 0;
    for j in 0..p.height {
        let row = p.src + j * p.stride;
        for i in 0..p.width {
            let v = i32::from(p.data[row + i]);
            if v < p.min {
                p.min = v;
            }
            if v > p.max {
                p.max = v;
            }
            used_levels[p.data[row + i] as usize] = true;
        }
    }
    // Compute the mininum distance between two non-zero levels.
    p.min_level_dist = p.max - p.min;
    let mut last_level = -1_i32;
    for (level, &used) in used_levels.iter().enumerate() {
        let i = i32::try_from(level).expect("a level fits in int");
        if used {
            p.num_levels += 1;
            if last_level >= 0 {
                let level_dist = i - last_level;
                if level_dist < p.min_level_dist {
                    p.min_level_dist = level_dist;
                }
            }
            last_level = i;
        }
    }
}

// upstream: src/utils/quant_levels_dec_utils.c InitParams
/// Allocates the scratch memory and sets the state of the filter over `data`.
/// Returns `None` when the allocation fails, which the C code reports as 0.
fn init_params(
    data: &mut [u8],
    width: i32,
    height: i32,
    stride: i32,
    radius: i32,
) -> Option<SmoothParams<'_>> {
    let r_size = 2 * radius + 1; // total size of the kernel
    let width_u = usize::try_from(width).expect("the width is positive");
    let size_scratch_m = usize::try_from(r_size + 1).expect("the kernel is positive") * width_u * 2;
    let size_m = width_u * 2;
    let size_lut = (1 + 2 * usize::try_from(LUT_SIZE).expect("the table size is positive")) * 2;
    let total_size = size_scratch_m + size_m + size_lut;
    // The allocation test of WebPSafeMalloc(1U, total_size), which is the same
    // as the one the C code applies to its scratch memory.
    webp_safe_malloc::<u8>(1, total_size)?;
    let rows = usize::try_from(r_size).expect("the kernel is positive");
    let mut p = SmoothParams {
        width: width_u,
        height: usize::try_from(height).expect("the height is positive"),
        stride: usize::try_from(stride).expect("the stride is positive"),
        row: -radius,
        data,
        src: 0,
        dst: 0,
        radius,
        scale: u32::try_from(
            (1_i32 << (i32::try_from(FIX).expect("FIX fits in int") + LFIX)) / (r_size * r_size),
        )
        .expect("the normalization is positive"),
        start: vec![0_u16; (rows + 1) * width_u],
        cur: 0,
        top: 0,
        end: rows * width_u,
        average: vec![0_u16; width_u],
        correction: vec![
            0_i16;
            1 + 2 * usize::try_from(LUT_SIZE).expect("the table size is positive")
        ],
        num_levels: 0,
        min: 0,
        max: 0,
        min_level_dist: 0,
    };
    // p->top = p->end - width, which is zeroed (the vectors start zeroed).
    p.top = p.end - width_u;
    count_levels(&mut p);
    init_correction_lut(&mut p.correction, p.min_level_dist);
    Some(p)
}

// upstream: src/utils/quant_levels_dec_utils.c WebPDequantizeLevels
/// Smooths a plane that was quantized to a few levels, with a filter whose
/// radius grows with `strength` (0 to 100). Returns `false` for a missing plane,
/// a non-positive size, a strength outside `[0, 100]`, or an allocation failure;
/// the plane is then unchanged.
///
/// # Panics
///
/// Panics when `data` is shorter than `stride * height` samples.
#[must_use]
pub fn webp_dequantize_levels(
    data: Option<&mut [u8]>,
    width: i32,
    height: i32,
    stride: i32,
    strength: i32,
) -> bool {
    let mut radius = 4 * strength / 100;
    if !(0..=100).contains(&strength) {
        return false;
    }
    let Some(data) = data else {
        return false;
    };
    if width <= 0 || height <= 0 {
        return false; // bad params
    }
    // limit the filter size to not exceed the image dimensions
    if 2 * radius + 1 > width {
        radius = (width - 1) >> 1;
    }
    if 2 * radius + 1 > height {
        radius = (height - 1) >> 1;
    }
    if radius > 0 {
        let Some(mut p) = init_params(data, width, height, stride, radius) else {
            return false;
        };
        if p.num_levels > 2 {
            while p.row < i32::try_from(p.height).expect("the height fits in int") {
                v_filter(&mut p); // accumulate average of input
                // Need to wait few rows in order to prime the filter,
                // before emitting some output.
                if p.row >= p.radius {
                    h_filter(&mut p);
                    apply_filter(&mut p);
                }
                p.row += 1;
            }
        }
    }
    true
}
