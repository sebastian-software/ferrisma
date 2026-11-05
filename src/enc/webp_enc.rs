// libwebp v1.6.0, src/enc/webp_enc.c:
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
// WebP encoder: main entry point
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/webp_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! Exception: WP9 translates `WebPEncodingSetError` and `WebPReportProgress` from `src/enc/webp_enc.c`, because the picture functions and the VP8L encoder report errors and progress through them.
//!
//! Exception: WP13 translates `WebPEncode`, `GetPSNR`, `FinalizePSNR` and `StoreStats` from `src/enc/webp_enc.c`, because `WebPEncode` joins picture conversion, the VP8, VP8L and alpha encoders and the statistics.
