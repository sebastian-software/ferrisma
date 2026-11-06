// libjpeg-turbo 3.2.1, src/jdapimin.c:
//
// /*
//  * jdapimin.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1998, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2016, 2022, 2024-2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains application interface code for the decompression half
//  * of the JPEG library.  These are the "minimum" API routines that may be
//  * needed in either the normal full-decompression case or the
//  * transcoding-only case.
//  *
//  * Most of the routines intended to be called directly by an application
//  * are in this file or in jdapistd.c.  But also see jcomapi.c for routines
//  * shared by compression and decompression, and jdtrans.c for the transcoding
//  * case.
//  */
//! Translation of `src/jdapimin.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
