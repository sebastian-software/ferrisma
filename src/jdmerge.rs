// libjpeg-turbo 3.2.1, src/jdmerge.h:
//
// /*
//  * jdmerge.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2020, 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdmerge.c:
//
// /*
//  * jdmerge.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2009, 2011, 2014-2015, 2020, 2022, 2025, D. R. Commander.
//  * Copyright (C) 2013, Linaro Limited.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains code for merged upsampling/color conversion.
//  *
//  * This file combines functions from jdsample.c and jdcolor.c;
//  * read those files first to understand what's going on.
//  *
//  * When the chroma components are to be upsampled by simple replication
//  * (ie, box filtering), we can save some work in color conversion by
//  * calculating all the output pixels corresponding to a pair of chroma
//  * samples at one time.  In the conversion equations
//  *      R = Y           + K1 * Cr
//  *      G = Y + K2 * Cb + K3 * Cr
//  *      B = Y + K4 * Cb
//  * only the Y term varies among the group of pixels corresponding to a pair
//  * of chroma samples, so the rest of the terms can be calculated just once.
//  * At typical sampling ratios, this eliminates half or three-quarters of the
//  * multiplications needed for color conversion.
//  *
//  * This file currently provides implementations for the following cases:
//  *      YCbCr => RGB color conversion only.
//  *      Sampling ratios of 2h1v or 2h2v.
//  *      No scaling needed at upsample time.
//  *      Corner-aligned (non-CCIR601) sampling alignment.
//  * Other special cases could be added, but in most applications these are
//  * the only common cases.  (For uncommon cases we fall back on the more
//  * general code in jdsample.c and jdcolor.c.)
//  */
//! Translation of `src/jdmerge.h` and `src/jdmerge.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
