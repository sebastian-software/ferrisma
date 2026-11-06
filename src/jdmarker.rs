// libjpeg-turbo 3.2.1, src/jdmarker.c:
//
// /*
//  * jdmarker.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1998, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2012, 2015, 2022, 2024, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains routines to decode JPEG datastream markers.
//  * Most of the complexity arises from our desire to support input
//  * suspension: if not all of the data for a marker is available,
//  * we must exit back to the application.  On resumption, we reprocess
//  * the marker.
//  */
//! Translation of `src/jdmarker.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
