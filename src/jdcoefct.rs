// libjpeg-turbo 3.2.1, src/jdcoefct.h:
//
// /*
//  * jdcoefct.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2020, Google, Inc.
//  * Copyright (C) 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdcoefct.c:
//
// /*
//  * jdcoefct.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2010, 2015-2016, 2019-2020, 2022-2026, D. R. Commander.
//  * Copyright (C) 2015, 2020, Google, Inc.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the coefficient buffer controller for decompression.
//  * This controller is the top level of the lossy JPEG decompressor proper.
//  * The coefficient buffer lies between entropy decoding and inverse-DCT steps.
//  *
//  * In buffered-image mode, this controller is the interface between
//  * input-oriented processing and output-oriented processing.
//  * Also, the input side (only) is used when reading a file for transcoding.
//  */
//! Translation of `src/jdcoefct.h` and `src/jdcoefct.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
