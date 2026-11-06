// libjpeg-turbo 3.2.1, src/jddctmgr.c:
//
// /*
//  * jddctmgr.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * Modified 2002-2010 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2010, 2015, 2022, 2025-2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the inverse-DCT management logic.
//  * This code selects a particular IDCT implementation to be used,
//  * and it performs related housekeeping chores.  No code in this file
//  * is executed per IDCT step, only during output pass setup.
//  *
//  * Note that the IDCT routines are responsible for performing coefficient
//  * dequantization as well as the IDCT proper.  This module sets up the
//  * dequantization multiplier table needed by the IDCT routine.
//  */
//! Translation of `src/jddctmgr.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
