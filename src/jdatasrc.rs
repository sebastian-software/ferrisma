// libjpeg-turbo 3.2.1, src/jdatasrc.c:
//
// /*
//  * jdatasrc.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * Modified 2009-2011 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2013, 2016, 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains decompression data source routines for the case of
//  * reading JPEG data from memory or from a file (or any stdio stream).
//  * While these routines are sufficient for most applications,
//  * some will want to use a different source manager.
//  * IMPORTANT: we assume that fread() will correctly transcribe an array of
//  * JOCTETs from 8-bit-wide elements on external storage.  If char is wider
//  * than 8 bits on your machine, you may need to do some tweaking.
//  */
//! Translation of `src/jdatasrc.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
//!
//! Not translated: `init_source`, `fill_input_buffer` and `jpeg_stdio_src`, because the stdio source is out of scope; only `jpeg_mem_src` is translated (ADR 0001).
