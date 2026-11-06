// libjpeg-turbo 3.2.1, src/jdapistd.c:
//
// /*
//  * jdapistd.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2015-2020, 2022-2026, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains application interface code for the decompression half
//  * of the JPEG library.  These are the "standard" API routines that are
//  * used in the normal full-decompression case.  They are not used by a
//  * transcoding-only application.  Note that if an application links in
//  * jpeg_start_decompress, it will end up linking in the entire decompressor.
//  * We thus must separate this file from jdapimin.c to avoid linking the
//  * whole decompression library into a transcoder.
//  */
//! Translation of `src/jdapistd.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 6 (Control and API).
