// libjpeg-turbo 3.2.1, src/jmemsys.h:
//
// /*
//  * jmemsys.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1992-1997, Thomas G. Lane.
//  * It was modified by The libjpeg-turbo Project to include only code and
//  * information relevant to libjpeg-turbo.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This include file defines the interface between the system-independent
//  * and system-dependent portions of the JPEG memory manager.  No other
//  * modules need include it.  (The system-independent portion is jmemmgr.c;
//  * there are several different versions of the system-dependent portion.)
//  *
//  * This file works as-is for the system-dependent memory managers supplied
//  * in the IJG distribution.  You may need to modify it if you write a
//  * custom memory manager.  If system-dependent changes are needed in
//  * this file, the best method is to #ifdef them based on a configuration
//  * symbol supplied in jconfig.h.
//  */
//! Translation of `src/jmemsys.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
