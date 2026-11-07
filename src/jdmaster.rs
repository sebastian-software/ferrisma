// libjpeg-turbo 3.2.1, src/jdmaster.h:
//
// /*
//  * jdmaster.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1995, Thomas G. Lane.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the master control structure for the JPEG decompressor.
//  */
// libjpeg-turbo 3.2.1, src/jdmaster.c:
//
// /*
//  * jdmaster.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Modified 2002-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009-2011, 2016, 2019, 2022-2024, 2026, D. R. Commander.
//  * Copyright (C) 2013, Linaro Limited.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains master control logic for the JPEG decompressor.
//  * These routines are concerned with selecting the modules to be executed
//  * and with determining the number of passes and the work to be done in each
//  * pass.
//  */
//! Translation of `src/jdmaster.h` and `src/jdmaster.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 6 (Control and API).
//!
//! Work package 2 translates the struct of `jdmaster.h`, because
//! `jpeg_CreateDecompress` creates it and `jdinput.c` and `jdmarker.c` use
//! its fields (ADR 0005, decisions of 2026-11-07); `jdmaster.c` is work
//! package 6.

use crate::jmorecfg::{C_BOOLEAN, C_INT, C_JDIMENSION, C_POINTER, CLayout, MAX_COMPONENTS};
use crate::jpegint::jpeg_decomp_master;

// upstream: src/jdmaster.h my_decomp_master
/// The private state of the master control module. `quantizer_1pass` and
/// `quantizer_2pass` belong to color quantization, which is out of scope
/// (ADR 0001).
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct my_decomp_master {
    /// Public fields.
    pub pub_: jpeg_decomp_master,
    /// Number of passes completed.
    pub pass_number: i32,
    /// True if using merged upsample/cconvert.
    pub using_merged_upsample: bool,
}

/// `sizeof(my_decomp_master)` on the target, with the method pointers of
/// `struct jpeg_decomp_master` and the quantizer pointers.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_decomp_master: usize = CLayout::structure(&[
    // struct jpeg_decomp_master
    CLayout::structure(&[
        C_POINTER.array(2),
        C_BOOLEAN,
        C_BOOLEAN,
        C_JDIMENSION,
        C_JDIMENSION,
        C_JDIMENSION.array(MAX_COMPONENTS),
        C_JDIMENSION.array(MAX_COMPONENTS),
        C_BOOLEAN,
        C_JDIMENSION,
        C_POINTER,
        C_INT,
        C_INT,
        C_INT,
    ]),
    C_INT,
    C_BOOLEAN,
    C_POINTER,
    C_POINTER,
])
.size;
