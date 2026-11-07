// libjpeg-turbo 3.2.1, src/jpegint.h:
//
// /*
//  * jpegint.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Modified 1997-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015-2017, 2019, 2021-2022, 2024-2026, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * Copyright (C) 2021, Alex Richardson.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file provides common declarations for the various JPEG modules.
//  * These declarations are considered internal to the JPEG library; most
//  * applications using the library shouldn't need to include this file.
//  */
//! Translation of `src/jpegint.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! The module interfaces (`jpeg_decomp_master`, `jpeg_input_controller`,
//! `jpeg_entropy_decoder` and the others) are added by the work package that
//! translates the module, together with its private struct and its slot in
//! `jpeg_decompress_struct` (ADR 0005, "Module interfaces arrive with their
//! modules"). The compression interfaces, the `CSTATE_*` values, `JDIFF` and
//! the `jinit_*` declarations of untranslated modules are out of scope (ADR
//! 0001). `MALLOC` has no Rust form: storage comes from Rust's allocator
//! (ADR 0005, "Owned storage"). `SHIFT_TEMPS` declares nothing without
//! `RIGHT_SHIFT_IS_UNSIGNED`.

use core::ffi::{c_long, c_ulong};

use crate::jpeglib::J_COLOR_SPACE;

// upstream: src/jpegint.h J_BUF_MODE
/// Operating modes for buffer controllers.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J_BUF_MODE(pub u32);

impl J_BUF_MODE {
    /// Plain stripwise operation.
    pub const JBUF_PASS_THRU: J_BUF_MODE = J_BUF_MODE(0);
    /// Run source subobject only, save output.
    pub const JBUF_SAVE_SOURCE: J_BUF_MODE = J_BUF_MODE(1);
    /// Run dest subobject only, using saved data.
    pub const JBUF_CRANK_DEST: J_BUF_MODE = J_BUF_MODE(2);
    /// Run both subobjects, save output.
    pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = J_BUF_MODE(3);
}

// upstream: src/jpegint.h DSTATE_START
/// After `create_decompress`.
pub const DSTATE_START: i32 = 200;
// upstream: src/jpegint.h DSTATE_INHEADER
/// Reading header markers, no SOS yet.
pub const DSTATE_INHEADER: i32 = 201;
// upstream: src/jpegint.h DSTATE_READY
/// Found SOS, ready for `start_decompress`.
pub const DSTATE_READY: i32 = 202;
// upstream: src/jpegint.h DSTATE_PRELOAD
/// Reading multiscan file in `start_decompress`.
pub const DSTATE_PRELOAD: i32 = 203;
// upstream: src/jpegint.h DSTATE_PRESCAN
/// Performing dummy pass for 2-pass quant.
pub const DSTATE_PRESCAN: i32 = 204;
// upstream: src/jpegint.h DSTATE_SCANNING
/// `start_decompress` done, `read_scanlines` OK.
pub const DSTATE_SCANNING: i32 = 205;
// upstream: src/jpegint.h DSTATE_RAW_OK
/// `start_decompress` done, `read_raw_data` OK.
pub const DSTATE_RAW_OK: i32 = 206;
// upstream: src/jpegint.h DSTATE_BUFIMAGE
/// Expecting `jpeg_start_output`.
pub const DSTATE_BUFIMAGE: i32 = 207;
// upstream: src/jpegint.h DSTATE_BUFPOST
/// Looking for SOS/EOI in `jpeg_finish_output`.
pub const DSTATE_BUFPOST: i32 = 208;
// upstream: src/jpegint.h DSTATE_RDCOEFS
/// Reading file in `jpeg_read_coefficients`.
pub const DSTATE_RDCOEFS: i32 = 209;
// upstream: src/jpegint.h DSTATE_STOPPING
/// Looking for EOI in `jpeg_finish_decompress`.
pub const DSTATE_STOPPING: i32 = 210;

// upstream: src/jpegint.h JLONG
/// `long` in upstream (ADR 0005).
pub type JLONG = c_long;

// upstream: src/jpegint.h JUINTPTR
/// `__UINTPTR_TYPE__`, an unsigned integer as wide as a pointer.
pub type JUINTPTR = usize;

// upstream: src/jpegint.h IsExtRGB
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn IsExtRGB(cs: J_COLOR_SPACE) -> bool {
    cs == J_COLOR_SPACE::JCS_RGB
        || (cs >= J_COLOR_SPACE::JCS_EXT_RGB && cs <= J_COLOR_SPACE::JCS_EXT_ARGB)
}

// upstream: src/jpegint.h LEFT_SHIFT
/// Left shift through `unsigned long`, so that a negative operand shifts
/// without signed overflow.
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "the C casts to unsigned long and back to JLONG"
)]
#[inline]
#[must_use]
pub fn LEFT_SHIFT(a: JLONG, b: u32) -> JLONG {
    ((a as c_ulong) << b) as JLONG
}

// upstream: src/jpegint.h MAX
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn MAX<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// upstream: src/jpegint.h MIN
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn MIN<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

// upstream: src/jpegint.h RIGHT_SHIFT
/// A signed right shift of a `JLONG`: arithmetic, since
/// `RIGHT_SHIFT_IS_UNSIGNED` is undefined on both hosts (ADR 0005).
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn RIGHT_SHIFT(x: JLONG, shft: u32) -> JLONG {
    x >> shft
}
