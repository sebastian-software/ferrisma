// libjpeg-turbo 3.2.1, src/jdct.h:
//
// /*
//  * jdct.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015, 2022, 2025, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This include file contains common declarations for the forward and
//  * inverse DCT modules.  These declarations are private to the DCT managers
//  * (jcdctmgr.c, jddctmgr.c) and the individual DCT algorithms.
//  * The individual DCT algorithms are kept in separate files to ease
//  * machine-dependent tuning (e.g., assembly coding).
//  */
//! Translation of `src/jdct.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! Only the inverse-DCT declarations are translated: the forward DCT and its
//! method types serve the encoder (ADR 0001). `WITH_SIMD` is undefined, so
//! `DCTELEM` is `int`. `FIX` and `CONST_SCALE` use the `CONST_BITS` of the
//! file that expands them, so they take it as a parameter, and each IDCT
//! module wraps them with its own value. `SHORTxSHORT_32` and
//! `SHORTxLCONST_32` are undefined, so `MULTIPLY16C16` and `MULTIPLY16V16`
//! are plain products of their `JLONG` operands.

use crate::jmorecfg::{CENTERJSAMPLE, FAST_FLOAT, JSAMPLE, MAXJSAMPLE, MULTIPLIER};
use crate::jpegint::{JLONG, RIGHT_SHIFT};
use crate::jpeglib::jpeg_decompress_struct;

// upstream: src/jdct.h DCTELEM
/// A DCT work element: 16 or 32 bits is fine.
pub type DCTELEM = i32;
// upstream: src/jdct.h UDCTELEM
/// The unsigned type of the same width as `DCTELEM`.
pub type UDCTELEM = u32;
// upstream: src/jdct.h UDCTELEM2
/// An unsigned type twice as wide as `UDCTELEM`.
pub type UDCTELEM2 = u64;

// upstream: src/jdct.h ISLOW_MULT_TYPE
/// Multiplier type of the accurate integer IDCT.
#[expect(non_camel_case_types, reason = "C type name")]
pub type ISLOW_MULT_TYPE = MULTIPLIER;
// upstream: src/jdct.h IFAST_MULT_TYPE
/// Multiplier type of the fast integer IDCT.
#[expect(non_camel_case_types, reason = "C type name")]
pub type IFAST_MULT_TYPE = MULTIPLIER;
// upstream: src/jdct.h IFAST_SCALE_BITS
/// Fractional bits in the fast IDCT's scale factors.
pub const IFAST_SCALE_BITS: u32 = 2;
// upstream: src/jdct.h FLOAT_MULT_TYPE
/// Multiplier type of the floating-point IDCT.
#[expect(non_camel_case_types, reason = "C type name")]
pub type FLOAT_MULT_TYPE = FAST_FLOAT;

// upstream: src/jdct.h IDCT_range_limit
/// The range-limit table as the IDCT indexes it: `CENTERJSAMPLE` samples
/// past `cinfo.sample_range_limit`, so that IDCT outputs, which are centered
/// on zero, map to samples centered on `CENTERJSAMPLE`.
///
/// # Panics
///
/// When the table is absent.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn IDCT_range_limit<'a>(cinfo: &'a jpeg_decompress_struct<'_>) -> &'a [JSAMPLE] {
    &cinfo.sample_range_limit()[CENTERJSAMPLE as usize..]
}

// upstream: src/jdct.h RANGE_MASK
/// Two bits wider than legal samples.
pub const RANGE_MASK: i32 = MAXJSAMPLE * 4 + 3;

// upstream: src/jdct.h ONE
/// 1 as a `JLONG`.
pub const ONE: JLONG = 1;

// upstream: src/jdct.h CONST_SCALE
/// `ONE << CONST_BITS`.
#[expect(non_snake_case, reason = "C macro name and parameter")]
#[inline]
#[must_use]
pub const fn CONST_SCALE(CONST_BITS: u32) -> JLONG {
    ONE << CONST_BITS
}

// upstream: src/jdct.h FIX
/// Converts a positive real constant to an integer scaled by
/// `CONST_SCALE`, rounding to nearest, in C's `double` arithmetic.
#[expect(non_snake_case, reason = "C macro name and parameter")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    reason = "C's conversions between JLONG and double"
)]
#[inline]
#[must_use]
pub const fn FIX(x: f64, CONST_BITS: u32) -> JLONG {
    (x * CONST_SCALE(CONST_BITS) as f64 + 0.5) as JLONG
}

// upstream: src/jdct.h DESCALE
/// Descales and correctly rounds a `JLONG` value that is scaled by
/// `2 ** n`.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn DESCALE(x: JLONG, n: u32) -> JLONG {
    RIGHT_SHIFT(x + (ONE << (n - 1)), n)
}

// upstream: src/jdct.h MULTIPLY16C16
/// Multiplies a variable by a constant.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn MULTIPLY16C16(var: JLONG, const_: JLONG) -> JLONG {
    var * const_
}

// upstream: src/jdct.h MULTIPLY16V16
/// Multiplies two variables.
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn MULTIPLY16V16(var1: JLONG, var2: JLONG) -> JLONG {
    var1 * var2
}
