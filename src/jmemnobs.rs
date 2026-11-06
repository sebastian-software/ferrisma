// libjpeg-turbo 3.2.1, src/jmemnobs.c:
//
// /*
//  * jmemnobs.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1992-1996, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2017-2018, 2024, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file provides a really simple implementation of the system-
//  * dependent portion of the JPEG memory manager.  This implementation
//  * assumes that no backing-store files are needed: all required space
//  * can be obtained from malloc().
//  * This is very portable in the sense that it'll compile on almost anything,
//  * but you'd better have lots of main memory (or virtual memory) if you want
//  * to process big images.
//  */
//! Translation of `src/jmemnobs.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
