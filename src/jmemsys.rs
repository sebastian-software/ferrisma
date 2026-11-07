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
//!
//! The functions this header declares are translated in `jmemnobs`, the only
//! system-dependent memory manager in scope. `jmemnobs.c` has no backing
//! store: its `jpeg_open_backing_store` always fails with
//! `JERR_NO_BACKING_STORE`, so no backing-store object is ever opened, and
//! `backing_store_info`, `TEMP_NAME_LENGTH` and the read, write and close
//! methods are not translated (ADR 0005, "Virtual arrays").

use core::ffi::c_long;

// upstream: src/jmemsys.h MAX_ALLOC_CHUNK
/// The largest request `jpeg_get_large` and `jpeg_get_small` receive.
pub const MAX_ALLOC_CHUNK: c_long = 1_000_000_000;
