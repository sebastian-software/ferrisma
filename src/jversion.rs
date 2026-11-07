// libjpeg-turbo 3.2.1, src/jversion.h.in:
//
// /*
//  * jversion.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-2020, Thomas G. Lane, Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2012-2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains software version identification.
//  */
//! Translation of `src/jversion.h.in` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! `JVERSION` takes the `JPEG_LIB_VERSION` 62 branch, and `JCOPYRIGHT` the
//! `COPYRIGHT_YEAR` that upstream's `CMakeLists.txt` sets (`1991-2026`).

// upstream: src/jversion.h.in JVERSION
pub const JVERSION: &str = "6b  27-Mar-1998";

// upstream: src/jversion.h.in JCOPYRIGHT
pub const JCOPYRIGHT: &str = "Copyright (C) 1991-2026 The libjpeg-turbo Project and many others";
