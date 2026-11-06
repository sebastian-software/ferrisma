// libjpeg-turbo 3.2.1, src/jmemmgr.c:
//
// /*
//  * jmemmgr.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2016, 2021-2022, 2024, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the JPEG system-independent memory management
//  * routines.  This code is usable across a wide variety of machines; most
//  * of the system dependencies have been isolated in a separate file.
//  * The major functions provided here are:
//  *   * pool-based allocation and freeing of memory;
//  *   * policy decisions about how to divide available memory among the
//  *     virtual arrays;
//  *   * control logic for swapping virtual arrays between main memory and
//  *     backing storage.
//  * The separate system-dependent file provides the actual backing-storage
//  * access code, and it contains the policy decision about how much total
//  * main memory to use.
//  * This file is system-dependent in the sense that some of its functions
//  * are unnecessary in some systems.  For example, if there is enough virtual
//  * memory so that backing storage will never be used, much of the virtual
//  * array control logic could be removed.  (Of course, if you have that much
//  * memory then you shouldn't care about a little bit of unused code...)
//  */
//! Translation of `src/jmemmgr.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
