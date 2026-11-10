// libjpeg-turbo 3.2.1, simd/x86_64/jdmerge-avx2.asm:
//
// ;
// ; Merged Upsampling/Color Conversion (64-bit AVX2)
// ;
// ; Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
// ; Copyright (C) 2009, 2016, 2024, D. R. Commander.
// ; Copyright (C) 2015, Intel Corporation.
// ;
// ; Based on the x86 SIMD extension for IJG JPEG library
// ; Copyright (C) 1999-2006, MIYASAKA Masaru.
// ; For conditions of distribution and use, see copyright notice in jsimdext.inc
// ;
// ; This file should be assembled with NASM (Netwide Assembler) or Yasm.
//! Translation of `simd/x86_64/jdmerge-avx2.asm` from libjpeg-turbo 3.2.1.
//!
//! Work package: 10 (x86-64 SIMD).
//!
//! AVX2. The constant table `jconst_merged_upsample_avx2` is a `static` of
//! a `#[repr(C, align(32))]` struct whose fields are the asm's labels, as
//! `ALIGNZ 32` aligns it (ADR 0008, decision 3); NASM's `dw` and `dd` items
//! are `i16` and `i32` arrays. The `equ` constants are `i32`s, as NASM
//! evaluates them. The instances of `jdmrgext-avx2.asm`'s kernels are
//! constants that name the generic functions of `jdmrgext_avx2` with the
//! layout of the asm instance, as `jdmerge_neon.rs` names those of
//! `jdmrgext_neon.rs`; the first instances take `jsimdcfg.inc`'s default
//! layout, which is `jmorecfg.h`'s.

#![forbid(unsafe_code)]

use crate::jdmrgext_avx2;
use crate::jmorecfg::{
    EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_BGRX_BLUE, EXT_BGRX_GREEN,
    EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED,
    EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
    EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE,
    EXT_XRGB_RED, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED,
};
use crate::jpegint::merged_upsample_simd_ptr;

// --------------------------------------------------------------------------

// upstream: simd/x86_64/jdmerge-avx2.asm SCALEBITS
pub const SCALEBITS: i32 = 16;

// upstream: simd/x86_64/jdmerge-avx2.asm F_0_344
/// FIX(0.34414)
const F_0_344: i32 = 22554;
// upstream: simd/x86_64/jdmerge-avx2.asm F_0_714
/// FIX(0.71414)
const F_0_714: i32 = 46802;
// upstream: simd/x86_64/jdmerge-avx2.asm F_1_402
/// FIX(1.40200)
const F_1_402: i32 = 91881;
// upstream: simd/x86_64/jdmerge-avx2.asm F_1_772
/// FIX(1.77200)
const F_1_772: i32 = 116_130;
// upstream: simd/x86_64/jdmerge-avx2.asm F_0_402
/// FIX(1.40200) - FIX(1)
const F_0_402: i32 = F_1_402 - 65536;
// upstream: simd/x86_64/jdmerge-avx2.asm F_0_285
/// FIX(1) - FIX(0.71414)
const F_0_285: i32 = 65536 - F_0_714;
// upstream: simd/x86_64/jdmerge-avx2.asm F_0_228
/// FIX(2) - FIX(1.77200)
const F_0_228: i32 = 131_072 - F_1_772;

/// NASM's `dw` of a constant expression: the 16-bit word, which every word
/// of the table fits. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    reason = "NASM's dw stores the low 16 bits; the assertion checks they hold the value"
)]
const fn dw(value: i32) -> i16 {
    assert!(value >= -32768 && value <= 32767);
    value as i16
}

// --------------------------------------------------------------------------

// upstream: simd/x86_64/jdmerge-avx2.asm jconst_merged_upsample_avx2
/// The layout of `jconst_merged_upsample_avx2`: one field per label. Not an
/// upstream item.
#[expect(non_snake_case, reason = "asm labels")]
#[repr(C, align(32))]
pub struct JconstMergedUpsampleAvx2 {
    pub PW_F0402: [i16; 16],
    pub PW_MF0228: [i16; 16],
    pub PW_MF0344_F0285: [i16; 16],
    pub PW_ONE: [i16; 16],
    pub PD_ONEHALF: [i32; 8],
}

// upstream: simd/x86_64/jdmerge-avx2.asm jconst_merged_upsample_avx2
/// The constants of `jdmrgext_avx2`'s kernels.
#[expect(non_upper_case_globals, reason = "asm symbol name")]
pub static jconst_merged_upsample_avx2: JconstMergedUpsampleAvx2 = JconstMergedUpsampleAvx2 {
    PW_F0402: [dw(F_0_402); 16],
    PW_MF0228: [dw(-F_0_228); 16],
    PW_MF0344_F0285: [
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
        dw(-F_0_344),
        dw(F_0_285),
    ],
    PW_ONE: [dw(1); 16],
    PD_ONEHALF: [1 << (SCALEBITS - 1); 8],
};

// --------------------------------------------------------------------------

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_merged_upsample_avx2
/// `jdmrgext-avx2.asm`'s h2v1 kernel with the default RGB layout.
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_merged_upsample_avx2
/// `jdmrgext-avx2.asm`'s h2v2 kernel with the default RGB layout.
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<RGB_RED, RGB_GREEN, RGB_BLUE, RGB_PIXELSIZE>;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_extrgb_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_extrgb_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<
        EXT_RGB_RED,
        EXT_RGB_GREEN,
        EXT_RGB_BLUE,
        EXT_RGB_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_extrgb_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_extrgb_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<
        EXT_RGB_RED,
        EXT_RGB_GREEN,
        EXT_RGB_BLUE,
        EXT_RGB_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_extrgbx_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_extrgbx_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<
        EXT_RGBX_RED,
        EXT_RGBX_GREEN,
        EXT_RGBX_BLUE,
        EXT_RGBX_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_extrgbx_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_extrgbx_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<
        EXT_RGBX_RED,
        EXT_RGBX_GREEN,
        EXT_RGBX_BLUE,
        EXT_RGBX_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_extbgr_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_extbgr_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<
        EXT_BGR_RED,
        EXT_BGR_GREEN,
        EXT_BGR_BLUE,
        EXT_BGR_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_extbgr_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_extbgr_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<
        EXT_BGR_RED,
        EXT_BGR_GREEN,
        EXT_BGR_BLUE,
        EXT_BGR_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_extbgrx_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_extbgrx_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<
        EXT_BGRX_RED,
        EXT_BGRX_GREEN,
        EXT_BGRX_BLUE,
        EXT_BGRX_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_extbgrx_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_extbgrx_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<
        EXT_BGRX_RED,
        EXT_BGRX_GREEN,
        EXT_BGRX_BLUE,
        EXT_BGRX_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_extxbgr_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_extxbgr_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<
        EXT_XBGR_RED,
        EXT_XBGR_GREEN,
        EXT_XBGR_BLUE,
        EXT_XBGR_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_extxbgr_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_extxbgr_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<
        EXT_XBGR_RED,
        EXT_XBGR_GREEN,
        EXT_XBGR_BLUE,
        EXT_XBGR_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v1_extxrgb_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v1_extxrgb_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v1_merged_upsample_avx2::<
        EXT_XRGB_RED,
        EXT_XRGB_GREEN,
        EXT_XRGB_BLUE,
        EXT_XRGB_PIXELSIZE,
    >;

// upstream: simd/x86_64/jdmerge-avx2.asm jsimd_h2v2_extxrgb_merged_upsample_avx2
#[expect(non_upper_case_globals, reason = "asm function name")]
pub const jsimd_h2v2_extxrgb_merged_upsample_avx2: merged_upsample_simd_ptr =
    jdmrgext_avx2::jsimd_h2v2_merged_upsample_avx2::<
        EXT_XRGB_RED,
        EXT_XRGB_GREEN,
        EXT_XRGB_BLUE,
        EXT_XRGB_PIXELSIZE,
    >;
