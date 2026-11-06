// libjpeg-turbo 3.2.1, src/jdmainct.h:
//
// /*
//  * jdmainct.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdmainct.c:
//
// /*
//  * jdmainct.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010, 2016, 2022, 2024, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the main buffer controller for decompression.
//  * The main buffer lies between the JPEG decompressor proper and the
//  * post-processor; it holds downsampled data in the JPEG colorspace.
//  *
//  * Note that this code is bypassed in raw-data mode, since the application
//  * supplies the equivalent of the main buffer in that case.
//  */
//! Translation of `src/jdmainct.h` and `src/jdmainct.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
