// libjpeg-turbo 3.2.1, src/jerror.h:
//
// /*
//  * jerror.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * Modified 1997-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2014, 2017, 2021-2023, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file defines the error and message codes for the JPEG library.
//  * Edit this file to add new codes, or to translate the message strings to
//  * some other language.
//  * A set of error-reporting macros are defined too.  Some applications using
//  * the JPEG library may wish to include this file to get the error codes
//  * and/or the macros.
//  */
// libjpeg-turbo 3.2.1, src/jerror.c:
//
// /*
//  * jerror.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1998, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, 2024, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains simple error-reporting and trace-message routines.
//  * These are suitable for Unix-like systems and others where writing to
//  * stderr is the right thing to do.  Many applications will want to replace
//  * some or all of these routines.
//  *
//  * If you define USE_WINDOWS_MESSAGEBOX in jconfig.h or in the makefile,
//  * you get a Windows-specific hack to display error messages in a dialog box.
//  * It ain't much, but it beats dropping error messages into the bit bucket,
//  * which is what happens to output to stderr under most Windows C compilers.
//  *
//  * These routines are used by both the compression and decompression code.
//  */
//! Translation of `src/jerror.h` and `src/jerror.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
