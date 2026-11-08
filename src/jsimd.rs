// libjpeg-turbo 3.2.1, simd/jsimd.c:
//
// /*
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2009-2011, 2013-2014, 2016, 2018, 2020, 2022-2026,
//  *           D. R. Commander.
//  * Copyright (C) 2011, Nokia Corporation and/or its subsidiary(-ies).
//  * Copyright (C) 2015-2016, 2018, 2022, Matthieu Darbois.
//  * Copyright (C) 2016-2018, Loongson Technology Corporation Limited, BeiJing.
//  * Copyright (C) 2019, Google LLC.
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2025, Samsung Electronics Co., Ltd.
//  *                     Author:  Filip Wasil
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
//  *
//  * This file contains the interface between the "normal" portions
//  * of the library and the SIMD implementations.
//  */
// libjpeg-turbo 3.2.1, simd/jsimd.h:
//
// /*
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2011, 2014, 2022, 2025, D. R. Commander.
//  * Copyright (C) 2015-2016, 2018, 2022, Matthieu Darbois.
//  * Copyright (C) 2020, Arm Limited.
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
// libjpeg-turbo 3.2.1, simd/jsimdconst.h:
//
// /*
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2011, 2014, 2016, 2018, 2025-2026, D. R. Commander.
//  * Copyright (C) 2016-2017, Loongson Technology Corporation Limited, BeiJing.
//  * Copyright (C) 2025, Samsung Electronics Co., Ltd.
//  *                     Author:  Filip Wasil
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
// libjpeg-turbo 3.2.1, simd/jsimddct.h:
//
// /*
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2025, D. R. Commander.
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
// libjpeg-turbo 3.2.1, simd/jsimdint.h:
//
// /*
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2011, 2014-2016, 2018, 2020, 2022, 2025, D. R. Commander.
//  * Copyright (C) 2014, Linaro Limited.
//  * Copyright (C) 2015-2016, 2018, 2022, Matthieu Darbois.
//  * Copyright (C) 2016-2018, Loongson Technology Corporation Limited, BeiJing.
//  * Copyright (C) 2020, Arm Limited.
//  * Copyright (C) 2022, Institute of Software, Chinese Academy of Sciences.
//  *                     Author:  Zhiyuan Tan
//  * Copyright (C) 2025, Samsung Electronics Co., Ltd.
//  *                     Author:  Filip Wasil
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
//! Translation of `simd/jsimd.c`, `simd/jsimd.h`, `simd/jsimdconst.h`, `simd/jsimddct.h` and `simd/jsimdint.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 9 (NEON SIMD).
//!
//! Not translated: `jsimd_set_rgb_ycc`, `jsimd_set_rgb_gray`, `jsimd_color_convert`, `jsimd_set_h2v1_downsample`, `jsimd_h2v1_downsample`, `jsimd_set_h2v2_downsample`, `jsimd_h2v2_downsample`, `jsimd_set_convsamp`, `jsimd_set_convsamp_float`, `jsimd_set_fdct_islow`, `jsimd_set_fdct_ifast`, `jsimd_set_fdct_float`, `jsimd_set_quantize`, `jsimd_set_quantize_float`, `jsimd_set_huff_encode_one_block`, `jsimd_set_encode_mcu_AC_first_prepare`, `jsimd_set_encode_mcu_AC_refine_prepare` and `jsimd_set_ycc_rgb565`, because they serve the encoder or RGB565 output, which are not translated (ADR 0001), and so do their declarations in the headers; only the AArch64 branches of the decoder's entry points are translated (ADR 0007).
//!
//! The port builds this module only with `WITH_SIMD`, on AArch64, so
//! `SIMD_ARCHITECTURE` is `ARM64` and only that branch of each entry point
//! is translated (ADR 0007). `init_simd` serves decompressors only, and
//! `NO_GETENV` and `WITH_SIMDE` are undefined. `BITS_IN_JSAMPLE` is 8,
//! `JDIMENSION` and `JCOEF` have the sizes the entry points require, and
//! `IFAST_SCALE_BITS` is 2, so those checks always pass and are kept as
//! upstream has them.
//!
//! An entry point `jsimd_set_*` stores a Neon kernel in a `*_simd` slot of
//! the module object, and the wrapper with the kernel's name calls it
//! through the slot. A `#[target_feature]` kernel coerces only to an `unsafe
//! fn` pointer, so each wrapper calls its slot in one `unsafe` block, sound
//! because the slots hold only Neon kernels and every AArch64 target enables
//! Neon. Where `jinit_upsampler` holds the upsampler outside `cinfo` while it
//! selects the methods, the entry points take it as a parameter instead of
//! reaching it through `cinfo->upsample` (ADR 0005, "Disjoint borrows").

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jdcolor_neon::{
    jsimd_ycc_extbgr_convert_neon, jsimd_ycc_extbgrx_convert_neon, jsimd_ycc_extrgb_convert_neon,
    jsimd_ycc_extrgbx_convert_neon, jsimd_ycc_extxbgr_convert_neon, jsimd_ycc_extxrgb_convert_neon,
    jsimd_ycc_rgb_convert_neon,
};
use crate::jdct::{IFAST_MULT_TYPE, IFAST_SCALE_BITS, ISLOW_MULT_TYPE};
use crate::jdmerge::my_merged_upsampler;
use crate::jdmerge_neon::{
    jsimd_h2v1_extbgr_merged_upsample_neon, jsimd_h2v1_extbgrx_merged_upsample_neon,
    jsimd_h2v1_extrgb_merged_upsample_neon, jsimd_h2v1_extrgbx_merged_upsample_neon,
    jsimd_h2v1_extxbgr_merged_upsample_neon, jsimd_h2v1_extxrgb_merged_upsample_neon,
    jsimd_h2v1_merged_upsample_neon, jsimd_h2v2_extbgr_merged_upsample_neon,
    jsimd_h2v2_extbgrx_merged_upsample_neon, jsimd_h2v2_extrgb_merged_upsample_neon,
    jsimd_h2v2_extrgbx_merged_upsample_neon, jsimd_h2v2_extxbgr_merged_upsample_neon,
    jsimd_h2v2_extxrgb_merged_upsample_neon, jsimd_h2v2_merged_upsample_neon,
};
use crate::jdsample::{ColorBuf, my_upsampler, own_rows};
use crate::jdsample_neon::{
    jsimd_h1v2_fancy_upsample_neon, jsimd_h2v1_fancy_upsample_neon, jsimd_h2v1_upsample_neon,
    jsimd_h2v2_fancy_upsample_neon, jsimd_h2v2_upsample_neon,
};
use crate::jidctfst_neon::jsimd_idct_ifast_neon;
use crate::jidctint_neon::jsimd_idct_islow_neon;
use crate::jidctred_neon::{jsimd_idct_2x2_neon, jsimd_idct_4x4_neon};
use crate::jmemmgr::GETENV_S;
use crate::jmorecfg::{JCOEF, JDIMENSION, JSAMPLE, RGB_PIXELSIZE};
use crate::jpegint::jpeg_upsampler;
use crate::jpeglib::{
    J_COLOR_SPACE, JBLOCK, JSAMPROWS, JSAMPROWS_AT, JSAMPROWS_MUT, j_decompress_ptr,
    jpeg_component_info, jpeg_decompress_struct,
};
use crate::jsimdcpu::jpeg_simd_cpu_support;

// upstream: simd/jsimdconst.h JSIMD_NONE
/// No SIMD instruction set.
pub const JSIMD_NONE: u32 = 0x00;
// upstream: simd/jsimdconst.h JSIMD_MMX
pub const JSIMD_MMX: u32 = 0x01;
// upstream: simd/jsimdconst.h JSIMD_3DNOW
pub const JSIMD_3DNOW: u32 = 0x02;
// upstream: simd/jsimdconst.h JSIMD_SSE
pub const JSIMD_SSE: u32 = 0x04;
// upstream: simd/jsimdconst.h JSIMD_SSE2
pub const JSIMD_SSE2: u32 = 0x08;
// upstream: simd/jsimdconst.h JSIMD_NEON
/// The Neon instruction set.
pub const JSIMD_NEON: u32 = 0x10;
// upstream: simd/jsimdconst.h JSIMD_RVV
pub const JSIMD_RVV: u32 = 0x20;
// upstream: simd/jsimdconst.h JSIMD_ALTIVEC
pub const JSIMD_ALTIVEC: u32 = 0x40;
// upstream: simd/jsimdconst.h JSIMD_AVX2
pub const JSIMD_AVX2: u32 = 0x80;
// upstream: simd/jsimdconst.h JSIMD_MMI
pub const JSIMD_MMI: u32 = 0x100;
// upstream: simd/jsimdconst.h JSIMD_MAX
/// The highest bit of the instruction set mask.
pub const JSIMD_MAX: u32 = 0x100;
// upstream: simd/jsimdconst.h JSIMD_UNDEFINED
/// The value of `simd_support` before `init_simd` has run.
pub const JSIMD_UNDEFINED: u32 = !(JSIMD_MAX * 2 - 1);

/// Whether the NUL-terminated contents of `env` are the string `"1"`:
/// `!strcmp(env, "1")`. Not an upstream item.
fn is_one(env: [u8; 2]) -> bool {
    env == *b"1\0"
}

// upstream: simd/jsimd.c init_simd
/// Check what SIMD accelerations are supported.
fn init_simd(cinfo: j_decompress_ptr<'_, '_>) {
    let mut env = [0u8; 2];
    let mut simd_support = cinfo.master().pub_.simd_support;
    let mut simd_huffman = cinfo.master().pub_.simd_huffman;

    if simd_support != JSIMD_UNDEFINED {
        return;
    }

    simd_support = jpeg_simd_cpu_support();

    /* Force different settings through environment variables */
    if GETENV_S(&mut env, "JSIMD_FORCENONE") == 0 && is_one(env) {
        simd_support = 0;
    }
    if GETENV_S(&mut env, "JSIMD_NOHUFFENC") == 0 && is_one(env) {
        simd_huffman = 0;
    }

    cinfo.master_mut().pub_.simd_support = simd_support;
    cinfo.master_mut().pub_.simd_huffman = simd_huffman;
}

// upstream: simd/jsimd.c jsimd_set_ycc_rgb
/// Stores the Neon YCbCr-to-RGB kernel for `cinfo.out_color_space` in
/// `cinfo.cconvert`, as `SET_SIMD_EXTRGB_COLOR_DECONVERTER(neon)` of
/// `jsimdint.h` does, and returns `JSIMD_NEON`, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When the master control object is absent.
pub fn jsimd_set_ycc_rgb(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if (RGB_PIXELSIZE != 3) && (RGB_PIXELSIZE != 4) {
        return JSIMD_NONE;
    }
    if cinfo.cconvert.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        /* SET_SIMD_EXTRGB_COLOR_DECONVERTER(neon) */
        let color_convert_simd = match cinfo.out_color_space {
            J_COLOR_SPACE::JCS_EXT_RGB => jsimd_ycc_extrgb_convert_neon,
            J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                jsimd_ycc_extrgbx_convert_neon
            }
            J_COLOR_SPACE::JCS_EXT_BGR => jsimd_ycc_extbgr_convert_neon,
            J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                jsimd_ycc_extbgrx_convert_neon
            }
            J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                jsimd_ycc_extxbgr_convert_neon
            }
            J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                jsimd_ycc_extxrgb_convert_neon
            }
            _ => jsimd_ycc_rgb_convert_neon,
        };
        cinfo
            .cconvert
            .as_deref_mut()
            .expect("cinfo->cconvert is NULL")
            .pub_
            .color_convert_simd = Some(color_convert_simd);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_color_deconvert
/// Converts `num_rows` rows of `input_buf` from row `input_row` on into
/// `output_buf` with the kernel that `jsimd_set_ycc_rgb` stored.
///
/// # Panics
///
/// When the color deconverter or its kernel is absent, or a row is shorter
/// than the kernel reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call of a Neon kernel")]
pub fn jsimd_color_deconvert(
    cinfo: &jpeg_decompress_struct<'_>,
    input_buf: &[JSAMPROWS<'_>],
    input_row: JDIMENSION,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    num_rows: i32,
) {
    let color_convert_simd = cinfo
        .cconvert
        .as_deref()
        .expect("cinfo->cconvert is NULL")
        .pub_
        .color_convert_simd
        .expect("color_convert_simd is NULL");
    // SAFETY: `jsimd_set_ycc_rgb` stores only Neon kernels in the slot, and
    // this module exists only with `WITH_SIMD` on AArch64, where every target
    // enables Neon.
    unsafe {
        color_convert_simd(
            cinfo.output_width,
            input_buf,
            input_row,
            output_buf,
            num_rows,
        );
    }
}

// upstream: simd/jsimd.c jsimd_set_h2v1_upsample
/// Selects the plain h2v1 upsampling kernel. Returns `JSIMD_NEON` when it
/// stores the Neon kernel in `upsample`, the public part of the upsampler
/// that `jinit_upsampler` holds outside `cinfo.upsample`, and `JSIMD_NONE`
/// otherwise. C's `!cinfo->upsample` check always passes, since `upsample`
/// is a reference.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_h2v1_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    upsample: &mut jpeg_upsampler,
) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        upsample.h2v1_upsample_simd = Some(jsimd_h2v1_upsample_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h2v1_upsample
/// Upsamples one row group of a component through `h2v1_upsample_simd`,
/// which C reaches through `cinfo->upsample`, passing the width
/// `cinfo->output_width`. It has `jdsample.rs`'s `upsample1_ptr` type and
/// writes the upsampler's own rows.
///
/// # Panics
///
/// When `h2v1_upsample_simd` is `None`, `color_buf` is not the upsampler's
/// storage, or a row is shorter than the kernel reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call through a SIMD kernel slot")]
pub fn jsimd_h2v1_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_upsampler,
    _compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let kernel = upsample
        .pub_
        .h2v1_upsample_simd
        .expect("h2v1_upsample_simd is NULL");
    let output_data = own_rows(output_data_ptr);
    // SAFETY: `jsimd_set_*upsample` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe {
        kernel(
            cinfo.max_v_samp_factor,
            cinfo.output_width,
            input_data,
            output_data.as_rows_mut(),
        );
    }
}

// upstream: simd/jsimd.c jsimd_set_h2v2_upsample
/// Selects the plain h2v2 upsampling kernel. Returns `JSIMD_NEON` when it
/// stores the Neon kernel in `upsample`, the public part of the upsampler
/// that `jinit_upsampler` holds outside `cinfo.upsample`, and `JSIMD_NONE`
/// otherwise. C's `!cinfo->upsample` check always passes, since `upsample`
/// is a reference.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_h2v2_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    upsample: &mut jpeg_upsampler,
) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        upsample.h2v2_upsample_simd = Some(jsimd_h2v2_upsample_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h2v2_upsample
/// Upsamples one row group of a component through `h2v2_upsample_simd`,
/// which C reaches through `cinfo->upsample`, passing the width
/// `cinfo->output_width`. It has `jdsample.rs`'s `upsample1_ptr` type and
/// writes the upsampler's own rows.
///
/// # Panics
///
/// When `h2v2_upsample_simd` is `None`, `color_buf` is not the upsampler's
/// storage, or a row is shorter than the kernel reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call through a SIMD kernel slot")]
pub fn jsimd_h2v2_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_upsampler,
    _compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let kernel = upsample
        .pub_
        .h2v2_upsample_simd
        .expect("h2v2_upsample_simd is NULL");
    let output_data = own_rows(output_data_ptr);
    // SAFETY: `jsimd_set_*upsample` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe {
        kernel(
            cinfo.max_v_samp_factor,
            cinfo.output_width,
            input_data,
            output_data.as_rows_mut(),
        );
    }
}

// upstream: simd/jsimd.c jsimd_set_h2v1_fancy_upsample
/// Selects the fancy h2v1 upsampling kernel. Returns `JSIMD_NEON` when it
/// stores the Neon kernel in `upsample`, the public part of the upsampler
/// that `jinit_upsampler` holds outside `cinfo.upsample`, and `JSIMD_NONE`
/// otherwise. C's `!cinfo->upsample` check always passes, since `upsample`
/// is a reference.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_h2v1_fancy_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    upsample: &mut jpeg_upsampler,
) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        upsample.h2v1_upsample_simd = Some(jsimd_h2v1_fancy_upsample_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h2v1_fancy_upsample
/// Upsamples one row group of a component through `h2v1_upsample_simd`,
/// which C reaches through `cinfo->upsample`, passing the width
/// `compptr->downsampled_width`. It has `jdsample.rs`'s `upsample1_ptr` type
/// and writes the upsampler's own rows.
///
/// # Panics
///
/// When `h2v1_upsample_simd` is `None`, `color_buf` is not the upsampler's
/// storage, or a row is shorter than the kernel reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call through a SIMD kernel slot")]
pub fn jsimd_h2v1_fancy_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let kernel = upsample
        .pub_
        .h2v1_upsample_simd
        .expect("h2v1_upsample_simd is NULL");
    let output_data = own_rows(output_data_ptr);
    // SAFETY: `jsimd_set_*upsample` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe {
        kernel(
            cinfo.max_v_samp_factor,
            compptr.downsampled_width,
            input_data,
            output_data.as_rows_mut(),
        );
    }
}

// upstream: simd/jsimd.c jsimd_set_h2v2_fancy_upsample
/// Selects the fancy h2v2 upsampling kernel. Returns `JSIMD_NEON` when it
/// stores the Neon kernel in `upsample`, the public part of the upsampler
/// that `jinit_upsampler` holds outside `cinfo.upsample`, and `JSIMD_NONE`
/// otherwise. C's `!cinfo->upsample` check always passes, since `upsample`
/// is a reference.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_h2v2_fancy_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    upsample: &mut jpeg_upsampler,
) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        upsample.h2v2_upsample_simd = Some(jsimd_h2v2_fancy_upsample_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h2v2_fancy_upsample
/// Upsamples one row group of a component through `h2v2_upsample_simd`,
/// which C reaches through `cinfo->upsample`, passing the width
/// `compptr->downsampled_width`. It has `jdsample.rs`'s `upsample1_ptr` type
/// and writes the upsampler's own rows.
///
/// # Panics
///
/// When `h2v2_upsample_simd` is `None`, `color_buf` is not the upsampler's
/// storage, or a row is shorter than the kernel reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call through a SIMD kernel slot")]
pub fn jsimd_h2v2_fancy_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let kernel = upsample
        .pub_
        .h2v2_upsample_simd
        .expect("h2v2_upsample_simd is NULL");
    let output_data = own_rows(output_data_ptr);
    // SAFETY: `jsimd_set_*upsample` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe {
        kernel(
            cinfo.max_v_samp_factor,
            compptr.downsampled_width,
            input_data,
            output_data.as_rows_mut(),
        );
    }
}

// upstream: simd/jsimd.c jsimd_set_h1v2_fancy_upsample
/// Selects the fancy h1v2 upsampling kernel. Returns `JSIMD_NEON` when it
/// stores the Neon kernel in `upsample`, the public part of the upsampler
/// that `jinit_upsampler` holds outside `cinfo.upsample`, and `JSIMD_NONE`
/// otherwise. C's `!cinfo->upsample` check always passes, since `upsample`
/// is a reference.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_h1v2_fancy_upsample(
    cinfo: j_decompress_ptr<'_, '_>,
    upsample: &mut jpeg_upsampler,
) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        upsample.h1v2_upsample_simd = Some(jsimd_h1v2_fancy_upsample_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h1v2_fancy_upsample
/// Upsamples one row group of a component through `h1v2_upsample_simd`,
/// which C reaches through `cinfo->upsample`, passing the width
/// `compptr->downsampled_width`. It has `jdsample.rs`'s `upsample1_ptr` type
/// and writes the upsampler's own rows.
///
/// # Panics
///
/// When `h1v2_upsample_simd` is `None`, `color_buf` is not the upsampler's
/// storage, or a row is shorter than the kernel reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call through a SIMD kernel slot")]
pub fn jsimd_h1v2_fancy_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_upsampler,
    compptr: &jpeg_component_info,
    input_data: JSAMPROWS_AT<'_>,
    output_data_ptr: &mut ColorBuf,
) {
    let kernel = upsample
        .pub_
        .h1v2_upsample_simd
        .expect("h1v2_upsample_simd is NULL");
    let output_data = own_rows(output_data_ptr);
    // SAFETY: `jsimd_set_*upsample` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe {
        kernel(
            cinfo.max_v_samp_factor,
            compptr.downsampled_width,
            input_data,
            output_data.as_rows_mut(),
        );
    }
}

// upstream: simd/jsimd.c jsimd_set_h2v1_merged_upsample
/// Stores the Neon h2v1 merged kernel for `cinfo.out_color_space` in
/// `cinfo.upsample`, as `SET_SIMD_EXTRGB_MERGED_UPSAMPLER(h2v1, neon)` of
/// `jsimdint.h` does, and returns `JSIMD_NEON`, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When the master control object is absent.
pub fn jsimd_set_h2v1_merged_upsample(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if cinfo.upsample.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        /* SET_SIMD_EXTRGB_MERGED_UPSAMPLER(h2v1, neon) */
        let merged_upsample_simd = match cinfo.out_color_space {
            J_COLOR_SPACE::JCS_EXT_RGB => jsimd_h2v1_extrgb_merged_upsample_neon,
            J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                jsimd_h2v1_extrgbx_merged_upsample_neon
            }
            J_COLOR_SPACE::JCS_EXT_BGR => jsimd_h2v1_extbgr_merged_upsample_neon,
            J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                jsimd_h2v1_extbgrx_merged_upsample_neon
            }
            J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                jsimd_h2v1_extxbgr_merged_upsample_neon
            }
            J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                jsimd_h2v1_extxrgb_merged_upsample_neon
            }
            _ => jsimd_h2v1_merged_upsample_neon,
        };
        cinfo
            .upsample
            .as_deref_mut()
            .expect("cinfo->upsample is NULL")
            .pub_mut()
            .merged_upsample_simd = Some(merged_upsample_simd);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h2v1_merged_upsample
/// Upsamples and color-converts one row group with the kernel that
/// `jsimd_set_h2v1_merged_upsample` stored. The merged upsampler is outside
/// `cinfo` while its method runs, so the kernel is reached through
/// `upsample`, not `cinfo->upsample` (ADR 0005, "Disjoint borrows").
///
/// # Panics
///
/// When the kernel is absent or a row is shorter than it reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call of a Neon kernel")]
pub fn jsimd_h2v1_merged_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_merged_upsampler,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    let merged_upsample_simd = upsample
        .pub_
        .merged_upsample_simd
        .expect("merged_upsample_simd is NULL");
    // SAFETY: `jsimd_set_h2v1_merged_upsample` stores only Neon kernels in
    // the slot, and this module exists only with `WITH_SIMD` on AArch64,
    // where every target enables Neon.
    unsafe {
        merged_upsample_simd(cinfo.output_width, input_buf, in_row_group_ctr, output_buf);
    }
}

// upstream: simd/jsimd.c jsimd_set_h2v2_merged_upsample
/// Stores the Neon h2v2 merged kernel for `cinfo.out_color_space` in
/// `cinfo.upsample`, as `SET_SIMD_EXTRGB_MERGED_UPSAMPLER(h2v2, neon)` of
/// `jsimdint.h` does, and returns `JSIMD_NEON`, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When the master control object is absent.
pub fn jsimd_set_h2v2_merged_upsample(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if cinfo.upsample.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        /* SET_SIMD_EXTRGB_MERGED_UPSAMPLER(h2v2, neon) */
        let merged_upsample_simd = match cinfo.out_color_space {
            J_COLOR_SPACE::JCS_EXT_RGB => jsimd_h2v2_extrgb_merged_upsample_neon,
            J_COLOR_SPACE::JCS_EXT_RGBX | J_COLOR_SPACE::JCS_EXT_RGBA => {
                jsimd_h2v2_extrgbx_merged_upsample_neon
            }
            J_COLOR_SPACE::JCS_EXT_BGR => jsimd_h2v2_extbgr_merged_upsample_neon,
            J_COLOR_SPACE::JCS_EXT_BGRX | J_COLOR_SPACE::JCS_EXT_BGRA => {
                jsimd_h2v2_extbgrx_merged_upsample_neon
            }
            J_COLOR_SPACE::JCS_EXT_XBGR | J_COLOR_SPACE::JCS_EXT_ABGR => {
                jsimd_h2v2_extxbgr_merged_upsample_neon
            }
            J_COLOR_SPACE::JCS_EXT_XRGB | J_COLOR_SPACE::JCS_EXT_ARGB => {
                jsimd_h2v2_extxrgb_merged_upsample_neon
            }
            _ => jsimd_h2v2_merged_upsample_neon,
        };
        cinfo
            .upsample
            .as_deref_mut()
            .expect("cinfo->upsample is NULL")
            .pub_mut()
            .merged_upsample_simd = Some(merged_upsample_simd);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_h2v2_merged_upsample
/// Upsamples and color-converts one row group into two output rows with the
/// kernel that `jsimd_set_h2v2_merged_upsample` stored; see
/// [`jsimd_h2v1_merged_upsample`].
///
/// # Panics
///
/// When the kernel is absent or a row is shorter than it reads or writes.
#[allow(unsafe_code, reason = "ADR 0007: a call of a Neon kernel")]
pub fn jsimd_h2v2_merged_upsample(
    cinfo: &jpeg_decompress_struct<'_>,
    upsample: &my_merged_upsampler,
    input_buf: &[JSAMPROWS_AT<'_>],
    in_row_group_ctr: JDIMENSION,
    output_buf: &mut [&mut [JSAMPLE]],
) {
    let merged_upsample_simd = upsample
        .pub_
        .merged_upsample_simd
        .expect("merged_upsample_simd is NULL");
    // SAFETY: `jsimd_set_h2v2_merged_upsample` stores only Neon kernels in
    // the slot, and this module exists only with `WITH_SIMD` on AArch64,
    // where every target enables Neon.
    unsafe {
        merged_upsample_simd(cinfo.output_width, input_buf, in_row_group_ctr, output_buf);
    }
}

// upstream: simd/jsimd.c jsimd_set_idct_islow
/// Stores the accurate integer IDCT kernel in `idct_simd` and returns its
/// instruction set, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_idct_islow(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if size_of::<JCOEF>() != 2 {
        return JSIMD_NONE;
    }
    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if size_of::<ISLOW_MULT_TYPE>() != 2 {
        return JSIMD_NONE;
    }
    if cinfo.idct.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        cinfo
            .idct
            .as_deref_mut()
            .expect("checked above")
            .pub_
            .idct_simd = Some(jsimd_idct_islow_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_idct_islow
/// Calls the kernel in `idct_simd`.
///
/// # Panics
///
/// When `cinfo.idct`, its `idct_simd` or the component's multiplier table
/// is absent, or as the kernel panics.
#[allow(
    unsafe_code,
    reason = "ADR 0007: a call through a *_simd slot of a Neon kernel"
)]
pub fn jsimd_idct_islow(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let idct_simd = cinfo.idct().idct_simd.expect("idct_simd is NULL");
    let dct_table = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL");
    // SAFETY: `jsimd_set_idct_islow` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe { idct_simd(dct_table, coef_block, output_buf, output_col) }
}

// upstream: simd/jsimd.c jsimd_set_idct_ifast
/// Stores the fast integer IDCT kernel in `idct_simd` and returns its
/// instruction set, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_idct_ifast(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if size_of::<JCOEF>() != 2 {
        return JSIMD_NONE;
    }
    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if size_of::<IFAST_MULT_TYPE>() != 2 {
        return JSIMD_NONE;
    }
    if IFAST_SCALE_BITS != 2 {
        return JSIMD_NONE;
    }
    if cinfo.idct.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        cinfo
            .idct
            .as_deref_mut()
            .expect("checked above")
            .pub_
            .idct_simd = Some(jsimd_idct_ifast_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_idct_ifast
/// Calls the kernel in `idct_simd`.
///
/// # Panics
///
/// When `cinfo.idct`, its `idct_simd` or the component's multiplier table
/// is absent, or as the kernel panics.
#[allow(
    unsafe_code,
    reason = "ADR 0007: a call through a *_simd slot of a Neon kernel"
)]
pub fn jsimd_idct_ifast(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let idct_simd = cinfo.idct().idct_simd.expect("idct_simd is NULL");
    let dct_table = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL");
    // SAFETY: `jsimd_set_idct_ifast` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe { idct_simd(dct_table, coef_block, output_buf, output_col) }
}

// upstream: simd/jsimd.c jsimd_set_idct_float
/// Returns `JSIMD_NONE`: AArch64 has no float IDCT kernel, so the C
/// routine applies. The x86 branch, which checks the sizes and calls
/// `init_simd`, is not compiled for AArch64.
#[must_use]
pub fn jsimd_set_idct_float(_cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_idct_float
/// Calls the kernel in `idct_simd`. `start_pass` selects it only when
/// `jsimd_set_idct_float` succeeds, which it never does on AArch64.
///
/// # Panics
///
/// When `cinfo.idct`, its `idct_simd` or the component's multiplier table
/// is absent, or as the kernel panics.
#[allow(
    unsafe_code,
    reason = "ADR 0007: a call through a *_simd slot of a Neon kernel"
)]
pub fn jsimd_idct_float(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let idct_simd = cinfo.idct().idct_simd.expect("idct_simd is NULL");
    let dct_table = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL");
    // SAFETY: the `jsimd_set_idct_*` entry points store only Neon kernels
    // in the slot, and this module exists only with `WITH_SIMD` on AArch64,
    // where every target enables Neon.
    unsafe { idct_simd(dct_table, coef_block, output_buf, output_col) }
}

// upstream: simd/jsimd.c jsimd_set_idct_2x2
/// Stores the reduced-size 2x2 IDCT kernel in `idct_2x2_simd` and returns
/// its instruction set, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_idct_2x2(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if size_of::<JCOEF>() != 2 {
        return JSIMD_NONE;
    }
    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if size_of::<ISLOW_MULT_TYPE>() != 2 {
        return JSIMD_NONE;
    }
    if cinfo.idct.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        cinfo
            .idct
            .as_deref_mut()
            .expect("checked above")
            .pub_
            .idct_2x2_simd = Some(jsimd_idct_2x2_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_idct_2x2
/// Calls the kernel in `idct_2x2_simd`.
///
/// # Panics
///
/// When `cinfo.idct`, its `idct_2x2_simd` or the component's multiplier
/// table is absent, or as the kernel panics.
#[allow(
    unsafe_code,
    reason = "ADR 0007: a call through a *_simd slot of a Neon kernel"
)]
pub fn jsimd_idct_2x2(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let idct_2x2_simd = cinfo.idct().idct_2x2_simd.expect("idct_2x2_simd is NULL");
    let dct_table = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL");
    // SAFETY: `jsimd_set_idct_2x2` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe { idct_2x2_simd(dct_table, coef_block, output_buf, output_col) }
}

// upstream: simd/jsimd.c jsimd_set_idct_4x4
/// Stores the reduced-size 4x4 IDCT kernel in `idct_4x4_simd` and returns
/// its instruction set, or returns `JSIMD_NONE`.
///
/// # Panics
///
/// When `cinfo.master` is absent.
pub fn jsimd_set_idct_4x4(cinfo: j_decompress_ptr<'_, '_>) -> u32 {
    init_simd(cinfo);

    if size_of::<JCOEF>() != 2 {
        return JSIMD_NONE;
    }
    if BITS_IN_JSAMPLE != 8 {
        return JSIMD_NONE;
    }
    if size_of::<JDIMENSION>() != 4 {
        return JSIMD_NONE;
    }
    if size_of::<ISLOW_MULT_TYPE>() != 2 {
        return JSIMD_NONE;
    }
    if cinfo.idct.is_none() {
        return JSIMD_NONE;
    }

    if cinfo.master().pub_.simd_support & JSIMD_NEON != 0 {
        cinfo
            .idct
            .as_deref_mut()
            .expect("checked above")
            .pub_
            .idct_4x4_simd = Some(jsimd_idct_4x4_neon);
        return JSIMD_NEON;
    }

    JSIMD_NONE
}

// upstream: simd/jsimd.c jsimd_idct_4x4
/// Calls the kernel in `idct_4x4_simd`.
///
/// # Panics
///
/// When `cinfo.idct`, its `idct_4x4_simd` or the component's multiplier
/// table is absent, or as the kernel panics.
#[allow(
    unsafe_code,
    reason = "ADR 0007: a call through a *_simd slot of a Neon kernel"
)]
pub fn jsimd_idct_4x4(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    coef_block: &JBLOCK,
    output_buf: JSAMPROWS_MUT<'_, '_>,
    output_col: JDIMENSION,
) {
    let idct_4x4_simd = cinfo.idct().idct_4x4_simd.expect("idct_4x4_simd is NULL");
    let dct_table = compptr
        .dct_table
        .as_deref()
        .expect("compptr->dct_table is NULL");
    // SAFETY: `jsimd_set_idct_4x4` stores only Neon kernels in the slot,
    // and this module exists only with `WITH_SIMD` on AArch64, where every
    // target enables Neon.
    unsafe { idct_4x4_simd(dct_table, coef_block, output_buf, output_col) }
}
