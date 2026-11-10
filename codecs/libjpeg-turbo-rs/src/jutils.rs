// libjpeg-turbo 3.2.1, src/jutils.c:
//
// /*
//  * jutils.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains tables and miscellaneous utility routines needed
//  * for both compression and decompression.
//  * Note we prefix all global names with "j" to minimize conflicts with
//  * a surrounding application.
//  */
//! Translation of `src/jutils.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! `jpeg_zigzag_order` is under `#if 0` upstream and is not translated;
//! `j12copy_sample_rows` and `j16copy_sample_rows` are the 12- and 16-bit
//! instantiations (ADR 0001).

use core::ffi::c_long;

use crate::jmorecfg::{JCOEF, JDIMENSION, JSAMPLE};
use crate::jpeglib::{DCTSIZE2, JBLOCK, JSAMPROWS, JSAMPROWS_MUT};

// upstream: src/jutils.c jpeg_natural_order
/// `jpeg_natural_order[i]` is the natural-order position of the i'th element
/// of zigzag order.
///
/// When reading corrupted data, the Huffman decoders could attempt to
/// reference an entry beyond the end of this array (if the decoded zero run
/// length reaches past the end of the block). To prevent wild stores without
/// adding an inner-loop test, we put some extra "63"s after the real entries.
/// This will cause the extra coefficient to be stored in location 63 of the
/// block, not somewhere random. The worst case would be a run-length of 15,
/// which means we need 16 fake entries.
#[expect(non_upper_case_globals, reason = "C table name")]
#[rustfmt::skip]
pub static jpeg_natural_order: [i32; DCTSIZE2 + 16] = [
     0,  1,  8, 16,  9,  2,  3, 10,
    17, 24, 32, 25, 18, 11,  4,  5,
    12, 19, 26, 33, 40, 48, 41, 34,
    27, 20, 13,  6,  7, 14, 21, 28,
    35, 42, 49, 56, 57, 50, 43, 36,
    29, 22, 15, 23, 30, 37, 44, 51,
    58, 59, 52, 45, 38, 31, 39, 46,
    53, 60, 61, 54, 47, 55, 62, 63,
    63, 63, 63, 63, 63, 63, 63, 63, /* extra entries for safety in decoder */
    63, 63, 63, 63, 63, 63, 63, 63,
];

// upstream: src/jutils.c jdiv_round_up
/// Computes `a/b` rounded up to the next integer, i.e. `ceil(a/b)`; assumes
/// `a >= 0` and `b > 0`.
#[must_use]
pub fn jdiv_round_up(a: c_long, b: c_long) -> c_long {
    (a + b - 1) / b
}

// upstream: src/jutils.c jround_up
/// Computes `a` rounded up to the next multiple of `b`, i.e. `ceil(a/b)*b`;
/// assumes `a >= 0` and `b > 0`.
#[must_use]
pub fn jround_up(mut a: c_long, b: c_long) -> c_long {
    a += b - 1;
    a - (a % b)
}

// upstream: src/jutils.c _jcopy_sample_rows
/// Copies `num_rows` rows of `num_cols` samples from
/// `input_array[source_row..]` to `output_array[dest_row..]`. The arrays are
/// distinct here; a copy within one array uses `RowListMut::copy_row` (ADR
/// 0005, "Caller rows").
///
/// # Panics
///
/// When a row is shorter than `num_cols` samples, where C would read or
/// write out of bounds.
#[expect(
    clippy::cast_sign_loss,
    reason = "row indexes and counts are nonnegative"
)]
pub fn jcopy_sample_rows(
    input_array: JSAMPROWS<'_>,
    source_row: i32,
    mut output_array: JSAMPROWS_MUT<'_, '_>,
    dest_row: i32,
    num_rows: i32,
    num_cols: JDIMENSION,
) {
    let count = num_cols as usize * core::mem::size_of::<JSAMPLE>();

    let input_array = input_array.offset(source_row as usize);
    let mut output_array = output_array.offset(dest_row as usize);

    let mut index = 0;
    let mut row = num_rows;
    while row > 0 {
        let inptr = input_array.row(index);
        let outptr = output_array.row_mut(index);
        outptr[..count].copy_from_slice(&inptr[..count]);
        index += 1;
        row -= 1;
    }
}

// upstream: src/jutils.c jcopy_block_row
/// Copies a row of coefficient blocks from one place to another.
///
/// # Panics
///
/// When a row is shorter than `num_blocks` blocks.
pub fn jcopy_block_row(input_row: &[JBLOCK], output_row: &mut [JBLOCK], num_blocks: JDIMENSION) {
    let num_blocks = num_blocks as usize;
    output_row[..num_blocks].copy_from_slice(&input_row[..num_blocks]);
}

// upstream: src/jutils.c jzero_far
/// Zeroes out a chunk of memory: the first `bytestozero` bytes of `target`,
/// which might be sample-array data, block-array data or `alloc_large` data.
///
/// # Panics
///
/// When `bytestozero` is not a whole number of elements or exceeds `target`.
pub fn jzero_far<T: ZeroElement>(target: &mut [T], bytestozero: usize) {
    let size = core::mem::size_of::<T>();
    assert!(
        bytestozero.is_multiple_of(size),
        "jzero_far: {bytestozero} bytes is not a whole number of elements"
    );
    target[..bytestozero / size].fill(T::ZERO);
}

/// An element type that `jzero_far` can zero: its value with every byte
/// zero. Not an upstream item.
pub trait ZeroElement: Copy {
    /// The all-zero value.
    const ZERO: Self;
}

impl ZeroElement for JSAMPLE {
    const ZERO: Self = 0;
}

impl ZeroElement for JCOEF {
    const ZERO: Self = 0;
}

impl ZeroElement for i32 {
    const ZERO: Self = 0;
}

impl ZeroElement for JBLOCK {
    const ZERO: Self = [0; DCTSIZE2];
}
