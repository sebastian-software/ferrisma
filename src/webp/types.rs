// libwebp v1.6.0, src/webp/types.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  Common types + memory wrappers
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/webp/types.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).

// The header also declares `WebPMalloc` and `WebPFree`, which src/utils/utils.c
// defines. They hand memory to C callers, so they belong to the C ABI module
// (ADR 0004, `src/abi/types.rs`, WP12) and have no safe counterpart here.

// upstream: src/webp/types.h WEBP_ABI_IS_INCOMPATIBLE
/// Reports whether two ABI version numbers differ in their major revision.
#[must_use]
pub const fn webp_abi_is_incompatible(a: i32, b: i32) -> bool {
    (a >> 8) != (b >> 8)
}
