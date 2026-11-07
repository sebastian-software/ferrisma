// libjpeg-turbo 3.2.1, src/jmorecfg.h:
//
// /*
//  * jmorecfg.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Modified 1997-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009, 2011, 2014-2015, 2018, 2020, 2022, 2026,
//  *           D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains additional configuration options that customize the
//  * JPEG software for special applications or support machine-dependent
//  * optimizations.  Most users will not need to touch this file.
//  */
//! Translation of `src/jmorecfg.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! The 8-bit instantiation only (ADR 0001): `J12SAMPLE`, `J16SAMPLE` and their
//! limits are not translated. `boolean` is `bool`, so `TRUE` and `FALSE` are
//! Rust's `true` and `false` (ADR 0005). The `METHODDEF`, `LOCAL`, `GLOBAL`,
//! `EXTERN`, `JMETHOD` and `FAR` macros only decorate C declarations and have
//! no Rust form. The capability options are branches that the translated code
//! takes or leaves where they are tested; the constants below record which of
//! them the reference configuration defines.

use core::ffi::c_long;

// upstream: src/jmorecfg.h JSAMPLE
/// One image sample, 8-bit data precision.
pub type JSAMPLE = u8;

// upstream: src/jmorecfg.h GETJSAMPLE
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn GETJSAMPLE(value: JSAMPLE) -> i32 {
    i32::from(value)
}

// upstream: src/jmorecfg.h MAXJSAMPLE
pub const MAXJSAMPLE: i32 = 255;
// upstream: src/jmorecfg.h CENTERJSAMPLE
pub const CENTERJSAMPLE: i32 = 128;

// upstream: src/jmorecfg.h JCOEF
/// A DCT frequency coefficient.
pub type JCOEF = i16;

// upstream: src/jmorecfg.h JOCTET
/// One byte of a compressed datastream.
pub type JOCTET = u8;

// upstream: src/jmorecfg.h GETJOCTET
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn GETJOCTET(value: JOCTET) -> JOCTET {
    value
}

// upstream: src/jmorecfg.h UINT8
pub type UINT8 = u8;
// upstream: src/jmorecfg.h UINT16
pub type UINT16 = u16;
// upstream: src/jmorecfg.h INT16
pub type INT16 = i16;
// upstream: src/jmorecfg.h INT32
/// `long` in upstream, so 64 bits on both CI hosts (ADR 0005).
pub type INT32 = c_long;

// upstream: src/jmorecfg.h JDIMENSION
/// An image dimension.
pub type JDIMENSION = u32;

// upstream: src/jmorecfg.h JPEG_MAX_DIMENSION
pub const JPEG_MAX_DIMENSION: c_long = 65500;

// upstream: src/jmorecfg.h DCT_ISLOW_SUPPORTED
pub const DCT_ISLOW_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h DCT_IFAST_SUPPORTED
pub const DCT_IFAST_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h DCT_FLOAT_SUPPORTED
pub const DCT_FLOAT_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h D_MULTISCAN_FILES_SUPPORTED
pub const D_MULTISCAN_FILES_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h D_PROGRESSIVE_SUPPORTED
pub const D_PROGRESSIVE_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h D_LOSSLESS_SUPPORTED
/// Defined upstream, but treated as undefined: lossless JPEG is out of scope
/// (ADR 0001, ADR 0005).
pub const D_LOSSLESS_SUPPORTED: bool = false;
// upstream: src/jmorecfg.h SAVE_MARKERS_SUPPORTED
pub const SAVE_MARKERS_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h BLOCK_SMOOTHING_SUPPORTED
pub const BLOCK_SMOOTHING_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h IDCT_SCALING_SUPPORTED
pub const IDCT_SCALING_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h UPSAMPLE_MERGING_SUPPORTED
pub const UPSAMPLE_MERGING_SUPPORTED: bool = true;
// upstream: src/jmorecfg.h QUANT_1PASS_SUPPORTED
/// Defined upstream, but treated as undefined: color quantization is out of
/// scope (ADR 0001, ADR 0005).
pub const QUANT_1PASS_SUPPORTED: bool = false;
// upstream: src/jmorecfg.h QUANT_2PASS_SUPPORTED
/// Defined upstream, but treated as undefined: color quantization is out of
/// scope (ADR 0001, ADR 0005).
pub const QUANT_2PASS_SUPPORTED: bool = false;

// upstream: src/jmorecfg.h RGB_RED
pub const RGB_RED: i32 = 0;
// upstream: src/jmorecfg.h RGB_GREEN
pub const RGB_GREEN: i32 = 1;
// upstream: src/jmorecfg.h RGB_BLUE
pub const RGB_BLUE: i32 = 2;
// upstream: src/jmorecfg.h RGB_PIXELSIZE
pub const RGB_PIXELSIZE: i32 = 3;

// upstream: src/jmorecfg.h JPEG_NUMCS
pub const JPEG_NUMCS: usize = 17;

// upstream: src/jmorecfg.h EXT_RGB_RED
pub const EXT_RGB_RED: i32 = 0;
// upstream: src/jmorecfg.h EXT_RGB_GREEN
pub const EXT_RGB_GREEN: i32 = 1;
// upstream: src/jmorecfg.h EXT_RGB_BLUE
pub const EXT_RGB_BLUE: i32 = 2;
// upstream: src/jmorecfg.h EXT_RGB_PIXELSIZE
pub const EXT_RGB_PIXELSIZE: i32 = 3;

// upstream: src/jmorecfg.h EXT_RGBX_RED
pub const EXT_RGBX_RED: i32 = 0;
// upstream: src/jmorecfg.h EXT_RGBX_GREEN
pub const EXT_RGBX_GREEN: i32 = 1;
// upstream: src/jmorecfg.h EXT_RGBX_BLUE
pub const EXT_RGBX_BLUE: i32 = 2;
// upstream: src/jmorecfg.h EXT_RGBX_PIXELSIZE
pub const EXT_RGBX_PIXELSIZE: i32 = 4;

// upstream: src/jmorecfg.h EXT_BGR_RED
pub const EXT_BGR_RED: i32 = 2;
// upstream: src/jmorecfg.h EXT_BGR_GREEN
pub const EXT_BGR_GREEN: i32 = 1;
// upstream: src/jmorecfg.h EXT_BGR_BLUE
pub const EXT_BGR_BLUE: i32 = 0;
// upstream: src/jmorecfg.h EXT_BGR_PIXELSIZE
pub const EXT_BGR_PIXELSIZE: i32 = 3;

// upstream: src/jmorecfg.h EXT_BGRX_RED
pub const EXT_BGRX_RED: i32 = 2;
// upstream: src/jmorecfg.h EXT_BGRX_GREEN
pub const EXT_BGRX_GREEN: i32 = 1;
// upstream: src/jmorecfg.h EXT_BGRX_BLUE
pub const EXT_BGRX_BLUE: i32 = 0;
// upstream: src/jmorecfg.h EXT_BGRX_PIXELSIZE
pub const EXT_BGRX_PIXELSIZE: i32 = 4;

// upstream: src/jmorecfg.h EXT_XBGR_RED
pub const EXT_XBGR_RED: i32 = 3;
// upstream: src/jmorecfg.h EXT_XBGR_GREEN
pub const EXT_XBGR_GREEN: i32 = 2;
// upstream: src/jmorecfg.h EXT_XBGR_BLUE
pub const EXT_XBGR_BLUE: i32 = 1;
// upstream: src/jmorecfg.h EXT_XBGR_PIXELSIZE
pub const EXT_XBGR_PIXELSIZE: i32 = 4;

// upstream: src/jmorecfg.h EXT_XRGB_RED
pub const EXT_XRGB_RED: i32 = 1;
// upstream: src/jmorecfg.h EXT_XRGB_GREEN
pub const EXT_XRGB_GREEN: i32 = 2;
// upstream: src/jmorecfg.h EXT_XRGB_BLUE
pub const EXT_XRGB_BLUE: i32 = 3;
// upstream: src/jmorecfg.h EXT_XRGB_PIXELSIZE
pub const EXT_XRGB_PIXELSIZE: i32 = 4;

// upstream: src/jmorecfg.h rgb_red
#[expect(non_upper_case_globals, reason = "C table name")]
pub static rgb_red: [i32; JPEG_NUMCS] = [
    -1,
    -1,
    RGB_RED,
    -1,
    -1,
    -1,
    EXT_RGB_RED,
    EXT_RGBX_RED,
    EXT_BGR_RED,
    EXT_BGRX_RED,
    EXT_XBGR_RED,
    EXT_XRGB_RED,
    EXT_RGBX_RED,
    EXT_BGRX_RED,
    EXT_XBGR_RED,
    EXT_XRGB_RED,
    -1,
];

// upstream: src/jmorecfg.h rgb_green
#[expect(non_upper_case_globals, reason = "C table name")]
pub static rgb_green: [i32; JPEG_NUMCS] = [
    -1,
    -1,
    RGB_GREEN,
    -1,
    -1,
    -1,
    EXT_RGB_GREEN,
    EXT_RGBX_GREEN,
    EXT_BGR_GREEN,
    EXT_BGRX_GREEN,
    EXT_XBGR_GREEN,
    EXT_XRGB_GREEN,
    EXT_RGBX_GREEN,
    EXT_BGRX_GREEN,
    EXT_XBGR_GREEN,
    EXT_XRGB_GREEN,
    -1,
];

// upstream: src/jmorecfg.h rgb_blue
#[expect(non_upper_case_globals, reason = "C table name")]
pub static rgb_blue: [i32; JPEG_NUMCS] = [
    -1,
    -1,
    RGB_BLUE,
    -1,
    -1,
    -1,
    EXT_RGB_BLUE,
    EXT_RGBX_BLUE,
    EXT_BGR_BLUE,
    EXT_BGRX_BLUE,
    EXT_XBGR_BLUE,
    EXT_XRGB_BLUE,
    EXT_RGBX_BLUE,
    EXT_BGRX_BLUE,
    EXT_XBGR_BLUE,
    EXT_XRGB_BLUE,
    -1,
];

// upstream: src/jmorecfg.h rgb_pixelsize
#[expect(non_upper_case_globals, reason = "C table name")]
pub static rgb_pixelsize: [i32; JPEG_NUMCS] = [
    -1,
    -1,
    RGB_PIXELSIZE,
    -1,
    -1,
    -1,
    EXT_RGB_PIXELSIZE,
    EXT_RGBX_PIXELSIZE,
    EXT_BGR_PIXELSIZE,
    EXT_BGRX_PIXELSIZE,
    EXT_XBGR_PIXELSIZE,
    EXT_XRGB_PIXELSIZE,
    EXT_RGBX_PIXELSIZE,
    EXT_BGRX_PIXELSIZE,
    EXT_XBGR_PIXELSIZE,
    EXT_XRGB_PIXELSIZE,
    -1,
];

// upstream: src/jmorecfg.h MULTIPLIER
/// `int` without `WITH_SIMD`, as the reference configuration builds it.
pub type MULTIPLIER = i32;

// upstream: src/jmorecfg.h FAST_FLOAT
#[expect(non_camel_case_types, reason = "C type name")]
pub type FAST_FLOAT = f32;

/// The C size and alignment of one struct member or of a whole struct, for
/// the `sizeof` constants that the memory manager's accounting needs (ADR
/// 0005, "Exact accounting"). Not an upstream item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CLayout {
    pub size: usize,
    pub align: usize,
}

impl CLayout {
    /// The layout of the Rust type `T`, for the C scalar or pointer type that
    /// it stands for.
    #[must_use]
    pub const fn of<T>() -> CLayout {
        CLayout {
            size: core::mem::size_of::<T>(),
            align: core::mem::align_of::<T>(),
        }
    }

    /// An array of `count` elements of this layout.
    #[must_use]
    pub const fn array(self, count: usize) -> CLayout {
        CLayout {
            size: self.size * count,
            align: self.align,
        }
    }

    /// A struct with these members in this order, laid out as Clang and GCC
    /// lay out a C struct: each member at the next multiple of its alignment,
    /// the whole padded to the largest member alignment.
    #[must_use]
    pub const fn structure(members: &[CLayout]) -> CLayout {
        let mut size: usize = 0;
        let mut align: usize = 1;
        let mut index = 0;
        while index < members.len() {
            let member = members[index];
            size = size.div_ceil(member.align) * member.align + member.size;
            if member.align > align {
                align = member.align;
            }
            index += 1;
        }
        CLayout {
            size: size.div_ceil(align) * align,
            align,
        }
    }
}

/// C `int`.
pub const C_INT: CLayout = CLayout::of::<i32>();
/// C `long`.
pub const C_LONG: CLayout = CLayout::of::<c_long>();
/// C `size_t`.
pub const C_SIZE_T: CLayout = CLayout::of::<usize>();
/// Any C data or function pointer.
pub const C_POINTER: CLayout = CLayout::of::<*const u8>();
/// C `boolean` (`int` in upstream).
pub const C_BOOLEAN: CLayout = C_INT;
/// `JDIMENSION`.
pub const C_JDIMENSION: CLayout = CLayout::of::<JDIMENSION>();
