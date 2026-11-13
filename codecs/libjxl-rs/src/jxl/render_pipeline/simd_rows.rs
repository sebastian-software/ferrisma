// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Row access of the SIMD render stages (ADR 0004).
//!
//! The original SIMD stages loop over whole vectors from
//! `x_start = -RoundUpTo(xextra_left, Lanes(d))` while `x < xsize +
//! xextra_right`, so they read and write up to `Lanes(d) - 1` samples beyond
//! the requested span; the source's row padding (`BytesPerRow` with
//! `MaxVectorSize()`) makes that memory valid. These helpers copy the read
//! spans out of the owner-relative rows and write a computed span back, so a
//! stage may read rows that it also writes (the source's in-place stages).
use super::render_pipeline_stage::{Row, RowBuffers};

/// The original loop bounds: `(x_start, iterations)` for `lanes`.
pub fn vector_span(left: usize, right: usize, width: usize, lanes: usize) -> (isize, usize) {
    let x_start = -(left.div_ceil(lanes) as isize * lanes as isize);
    let x_end = width.wrapping_add(right) as isize;
    let n = (x_end - x_start).max(0) as usize;
    (x_start, n.div_ceil(lanes))
}

/// Copy `len` samples of `row` starting at sample offset `from` (relative to
/// the row descriptor, may be negative).
///
/// public gate: in the low-memory pipeline of the public decoder, the
/// whole-vector span can begin before the first sample of the row's plane
/// row: the group input rows start `kRenderPipelineXOffset -
/// group_data_x_border` samples into their row, and a stage with a small
/// `xextra_left` and a vertical border (EPF) reads up to `Lanes(d) - 1 +
/// border` samples to the left. The original reads the preceding row's
/// padding (or the plane's pre-padding) there; those samples only reach
/// lanes left of `-xextra_left`, which no later stage or output uses. The
/// port reads 0.0 for samples outside its row slice, on either side.
pub fn read(buffers: &RowBuffers<'_>, row: Row, from: isize, len: usize) -> Vec<f32> {
    let span = buffers.row(Row { x: 0, ..row });
    let start = row.x as isize + from;
    if start >= 0 && start as usize + len <= span.len() {
        let start = start as usize;
        return span[start..start + len].to_vec();
    }
    (0..len as isize)
        .map(|i| {
            usize::try_from(start + i)
                .ok()
                .and_then(|k| span.get(k).copied())
                .unwrap_or(0.0)
        })
        .collect()
}

/// Write `values` to `row` starting at sample offset `from`.
pub fn write(buffers: &mut RowBuffers<'_>, row: Row, from: isize, values: &[f32]) {
    let span = buffers.row_mut(Row { x: 0, ..row });
    let start = row.x.wrapping_add_signed(from);
    span[start..start + values.len()].copy_from_slice(values);
}
