// libjpeg-turbo 3.2.1, src/jdinput.c:
//
// /*
//  * jdinput.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2016, 2018, 2022, 2024, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains input control logic for the JPEG decompressor.
//  * These routines are concerned with controlling the decompressor's input
//  * processing (marker reading and coefficient/difference decoding).
//  * The actual input reading is done in jdmarker.c, jdhuff.c, jdphuff.c,
//  * and jdlhuff.c.
//  */
//! Translation of `src/jdinput.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
