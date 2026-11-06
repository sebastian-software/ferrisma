// libjpeg-turbo 3.2.1, src/jdsample.h:
//
// /*
//  * jdsample.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  */
// libjpeg-turbo 3.2.1, src/jdsample.c:
//
// /*
//  * jdsample.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2010, 2015-2016, 2022, 2024-2026, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * Copyright (C) 2019-2020, Arm Limited.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains upsampling routines.
//  *
//  * Upsampling input data is counted in "row groups".  A row group is defined to
//  * be (v_samp_factor * DCT_scaled_size / min_DCT_scaled_size) sample rows of
//  * each component.  Upsampling will normally produce max_v_samp_factor rows of
//  * each component from each row group (but this could vary if the upsampler is
//  * applying a scale factor of its own).
//  *
//  * An excellent reference for image resampling is
//  *   Digital Image Warping, George Wolberg, 1990.
//  *   Pub. by IEEE Computer Society Press, Los Alamitos, CA. ISBN 0-8186-8944-7.
//  */
//! Translation of `src/jdsample.h` and `src/jdsample.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 5 (Upsampling, color, buffers).
