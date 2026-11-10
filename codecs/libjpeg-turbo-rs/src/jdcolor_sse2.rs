// libjpeg-turbo 3.2.1, simd/x86_64/jdcolor-sse2.asm:
//
// ;
// ; YCbCr-to-RGB Color Conversion (64-bit SSE2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2024, D. R. Commander.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/x86_64/jdcolor-sse2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! SSE2. The constant table `jconst_ycc_rgb_convert_sse2` is a `static` of
//! [`JconstYccRgbConvertSse2`], whose fields are the asm's labels and which
//! `#[repr(C, align(32))]` aligns as `ALIGNZ 32` does; `jsimd` checks its
//! address with `IS_ALIGNED_SSE` as `jsimd.c` does. The asm includes
//! `jdcolext-sse2.asm` once per pixel layout, with `RGB_RED`, `RGB_GREEN`,
//! `RGB_BLUE` and `RGB_PIXELSIZE` defined and the function renamed. That file
//! is the generic function of `jdcolext_sse2`, and the instances are
//! constants that name it with the layout of each instance, as
//! `jdcolor_neon.rs` names those of `jdcolext_neon`. The first instance has
//! the default layout of `jsimdcfg.inc`, `jmorecfg.h`'s `RGB_*`.

#![forbid(unsafe_code)]

use crate::jdcolext_sse2;
use crate::jmorecfg::{
    EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_BGRX_BLUE, EXT_BGRX_GREEN,
    EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED,
    EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
    EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE,
    EXT_XRGB_RED, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED,
};
use crate::jpegint::color_convert_simd_ptr;

// --------------------------------------------------------------------------

// upstream: simd/x86_64/jdcolor-sse2.asm SCALEBITS
pub const SCALEBITS: i32 = 16;

// upstream: simd/x86_64/jdcolor-sse2.asm F_0_344
/// FIX(0.34414)
const F_0_344: i32 = 22554;
// upstream: simd/x86_64/jdcolor-sse2.asm F_0_714
/// FIX(0.71414)
const F_0_714: i32 = 46802;
// upstream: simd/x86_64/jdcolor-sse2.asm F_1_402
/// FIX(1.40200)
const F_1_402: i32 = 91881;
// upstream: simd/x86_64/jdcolor-sse2.asm F_1_772
/// FIX(1.77200)
const F_1_772: i32 = 116_130;
// upstream: simd/x86_64/jdcolor-sse2.asm F_0_402
/// FIX(1.40200) - FIX(1)
const F_0_402: i32 = F_1_402 - 65536;
// upstream: simd/x86_64/jdcolor-sse2.asm F_0_285
/// FIX(1) - FIX(0.71414)
const F_0_285: i32 = 65536 - F_0_714;
// upstream: simd/x86_64/jdcolor-sse2.asm F_0_228
/// FIX(2) - FIX(1.77200)
const F_0_228: i32 = 131_072 - F_1_772;

/// NASM's `dw`: `value` as a 16-bit data word. Every value of the table fits,
/// which the assertion checks when the table is built. Not an upstream item.
#[expect(clippy::cast_possible_truncation, reason = "the asm's dw")]
const fn dw(value: i32) -> i16 {
    assert!(value >= i16::MIN as i32 && value <= i16::MAX as i32);
    value as i16
}

// --------------------------------------------------------------------------

/// The layout of `jconst_ycc_rgb_convert_sse2`: one field per label, aligned
/// as `ALIGNZ 32` aligns the table. Not an upstream item.
#[expect(non_snake_case, reason = "the asm's labels")]
#[repr(C, align(32))]
#[derive(Debug)]
pub struct JconstYccRgbConvertSse2 {
    /// `PW_F0402 times 8 dw F_0_402`
    pub PW_F0402: [i16; 8],
    /// `PW_MF0228 times 8 dw -F_0_228`
    pub PW_MF0228: [i16; 8],
    /// `PW_MF0344_F0285 times 4 dw -F_0_344, F_0_285`
    pub PW_MF0344_F0285: [i16; 8],
    /// `PW_ONE times 8 dw 1`
    pub PW_ONE: [i16; 8],
    /// `PD_ONEHALF times 4 dd 1 << (SCALEBITS - 1)`
    pub PD_ONEHALF: [i32; 4],
}

// upstream: simd/x86_64/jdcolor-sse2.asm jconst_ycc_rgb_convert_sse2
#[expect(non_upper_case_globals, reason = "asm symbol name")]
pub static jconst_ycc_rgb_convert_sse2: JconstYccRgbConvertSse2 = JconstYccRgbConvertSse2 {
    PW_F0402: [dw(F_0_402); 8],
    PW_MF0228: [dw(-F_0_228); 8],
    PW_MF0344_F0285: [
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
    ],
    PW_ONE: [1; 8],
    PD_ONEHALF: [1 << (SCALEBITS - 1); 4],
};

// --------------------------------------------------------------------------

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_rgb_convert_sse2
/// `jdcolext-sse2.asm`'s kernel with the default RGB layout.
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_rgb_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>;

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_extrgb_convert_sse2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_extrgb_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<
        EXT_RGB_RED,
        EXT_RGB_GREEN,
        EXT_RGB_BLUE,
        EXT_RGB_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_extrgbx_convert_sse2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_extrgbx_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<
        EXT_RGBX_RED,
        EXT_RGBX_GREEN,
        EXT_RGBX_BLUE,
        EXT_RGBX_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_extbgr_convert_sse2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_extbgr_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<
        EXT_BGR_RED,
        EXT_BGR_GREEN,
        EXT_BGR_BLUE,
        EXT_BGR_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_extbgrx_convert_sse2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_extbgrx_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<
        EXT_BGRX_RED,
        EXT_BGRX_GREEN,
        EXT_BGRX_BLUE,
        EXT_BGRX_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_extxbgr_convert_sse2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_extxbgr_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<
        EXT_XBGR_RED,
        EXT_XBGR_GREEN,
        EXT_XBGR_BLUE,
        EXT_XBGR_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdcolor-sse2.asm jsimd_ycc_extxrgb_convert_sse2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_ycc_extxrgb_convert_sse2: color_convert_simd_ptr =
    jdcolext_sse2::jsimd_ycc_rgb_convert_sse2::<
        EXT_XRGB_RED,
        EXT_XRGB_GREEN,
        EXT_XRGB_BLUE,
        EXT_XRGB_PIXELSIZE,
    >;
