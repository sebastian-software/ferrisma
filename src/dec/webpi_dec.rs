// libwebp v1.6.0, src/dec/webpi_dec.h:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Internal header: WebP decoding parameters and custom IO on buffer
//
// Author: somnath@google.com (Somnath Banerjee)

//! Translation of `src/dec/webpi_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP12 (decoder integration).
//!
//! Exception: WP7 translates `WebPDecParams`, `OutputFunc`, `OutputAlphaFunc` and `OutputRowFunc` from `src/dec/webpi_dec.h`, because `VP8LDecodeImage` reads `WebPDecParams` through `io->opaque`.
