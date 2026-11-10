// libjpeg-turbo 3.2.1, simd/arm/jdmerge-neon.c:
//
// /*
//  * Merged Upsampling/Color Conversion (Arm Neon)
//  *
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2024-2025, D. R. Commander.
//  *
//  * This software is provided 'as-is', without any express or implied
//  * warranty.  In no event will the authors be held liable for any damages
//  * arising from the use of this software.
//  *
//  * Permission is granted to anyone to use this software for any purpose,
//  * including commercial applications, and to alter it and redistribute it
//  * freely, subject to the following restrictions:
//  *
//  * 1. The origin of this software must not be misrepresented; you must not
//  *    claim that you wrote the original software. If you use this software
//  *    in a product, an acknowledgment in the product documentation would be
//  *    appreciated but is not required.
//  * 2. Altered source versions must be plainly marked as such, and must not be
//  *    misrepresented as being the original software.
//  * 3. This notice may not be removed or altered from any source distribution.
//  */
//! Translation of `simd/arm/jdmerge-neon.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! The instances of `jdmrgext-neon.c`'s kernels are constants that name the
//! generic functions of `jdmrgext_neon` with the layout of the C instance, as
//! `jdmerge.rs` names those of `jdmrgext.rs`; `RGB_ALPHA` is -1 where C
//! leaves it undefined. The first instances have `jmorecfg.h`'s default
//! layout.

#![forbid(unsafe_code)]

use crate::jdmrgext_neon;
use crate::jmorecfg::{
    EXT_BGR_BLUE, EXT_BGR_GREEN, EXT_BGR_PIXELSIZE, EXT_BGR_RED, EXT_BGRX_BLUE, EXT_BGRX_GREEN,
    EXT_BGRX_PIXELSIZE, EXT_BGRX_RED, EXT_RGB_BLUE, EXT_RGB_GREEN, EXT_RGB_PIXELSIZE, EXT_RGB_RED,
    EXT_RGBX_BLUE, EXT_RGBX_GREEN, EXT_RGBX_PIXELSIZE, EXT_RGBX_RED, EXT_XBGR_BLUE, EXT_XBGR_GREEN,
    EXT_XBGR_PIXELSIZE, EXT_XBGR_RED, EXT_XRGB_BLUE, EXT_XRGB_GREEN, EXT_XRGB_PIXELSIZE,
    EXT_XRGB_RED, RGB_BLUE, RGB_GREEN, RGB_PIXELSIZE, RGB_RED,
};
use crate::jpegint::merged_upsample_simd_ptr;

/* YCbCr -> RGB conversion constants */

// upstream: simd/arm/jdmerge-neon.c F_0_344
/// 0.3441467 = 11277 * 2^-15
const F_0_344: i16 = 11277;
// upstream: simd/arm/jdmerge-neon.c F_0_714
/// 0.7141418 = 23401 * 2^-15
const F_0_714: i16 = 23401;
// upstream: simd/arm/jdmerge-neon.c F_1_402
/// 1.4020386 = 22971 * 2^-14
const F_1_402: i16 = 22971;
// upstream: simd/arm/jdmerge-neon.c F_1_772
/// 1.7720337 = 29033 * 2^-14
const F_1_772: i16 = 29033;

// upstream: simd/arm/jdmerge-neon.c jsimd_ycc_rgb_convert_neon_consts
/// The multipliers of the conversion, which `jdmrgext_neon` loads.
#[expect(non_upper_case_globals, reason = "C variable name")]
pub static jsimd_ycc_rgb_convert_neon_consts: [i16; 4] = [-F_0_344, F_0_714, F_1_402, F_1_772];

/* Include inline routines for colorspace extensions. */

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_merged_upsample_neon
/// `jdmrgext-neon.c`'s h2v1 kernel with `jmorecfg.h`'s default RGB layout.
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_merged_upsample_neon
/// `jdmrgext-neon.c`'s h2v2 kernel with `jmorecfg.h`'s default RGB layout.
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<RGB_RED, RGB_GREEN, RGB_BLUE, -1, RGB_PIXELSIZE>;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_extrgb_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_extrgb_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<
        EXT_RGB_RED,
        EXT_RGB_GREEN,
        EXT_RGB_BLUE,
        -1,
        EXT_RGB_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_extrgb_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_extrgb_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<
        EXT_RGB_RED,
        EXT_RGB_GREEN,
        EXT_RGB_BLUE,
        -1,
        EXT_RGB_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_extrgbx_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_extrgbx_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<
        EXT_RGBX_RED,
        EXT_RGBX_GREEN,
        EXT_RGBX_BLUE,
        3,
        EXT_RGBX_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_extrgbx_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_extrgbx_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<
        EXT_RGBX_RED,
        EXT_RGBX_GREEN,
        EXT_RGBX_BLUE,
        3,
        EXT_RGBX_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_extbgr_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_extbgr_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<
        EXT_BGR_RED,
        EXT_BGR_GREEN,
        EXT_BGR_BLUE,
        -1,
        EXT_BGR_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_extbgr_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_extbgr_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<
        EXT_BGR_RED,
        EXT_BGR_GREEN,
        EXT_BGR_BLUE,
        -1,
        EXT_BGR_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_extbgrx_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_extbgrx_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<
        EXT_BGRX_RED,
        EXT_BGRX_GREEN,
        EXT_BGRX_BLUE,
        3,
        EXT_BGRX_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_extbgrx_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_extbgrx_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<
        EXT_BGRX_RED,
        EXT_BGRX_GREEN,
        EXT_BGRX_BLUE,
        3,
        EXT_BGRX_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_extxbgr_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_extxbgr_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<
        EXT_XBGR_RED,
        EXT_XBGR_GREEN,
        EXT_XBGR_BLUE,
        0,
        EXT_XBGR_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_extxbgr_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_extxbgr_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<
        EXT_XBGR_RED,
        EXT_XBGR_GREEN,
        EXT_XBGR_BLUE,
        0,
        EXT_XBGR_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v1_extxrgb_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v1_extxrgb_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v1_merged_upsample_neon::<
        EXT_XRGB_RED,
        EXT_XRGB_GREEN,
        EXT_XRGB_BLUE,
        0,
        EXT_XRGB_PIXELSIZE,
    >;

// upstream: simd/arm/jdmerge-neon.c jsimd_h2v2_extxrgb_merged_upsample_neon
#[expect(non_upper_case_globals, reason = "C function name")]
pub const jsimd_h2v2_extxrgb_merged_upsample_neon: merged_upsample_simd_ptr =
    jdmrgext_neon::jsimd_h2v2_merged_upsample_neon::<
        EXT_XRGB_RED,
        EXT_XRGB_GREEN,
        EXT_XRGB_BLUE,
        0,
        EXT_XRGB_PIXELSIZE,
    >;
