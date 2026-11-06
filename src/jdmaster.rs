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
