// libwebp v1.6.0, src/enc/picture_enc.c:
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
// WebPPicture class basis
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/picture_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP9 (encoder picture and config).
//!
//! Exception: WP13 translates `Encode`, `WebPEncodeRGB`, `WebPEncodeBGR`, `WebPEncodeRGBA`, `WebPEncodeBGRA`, `WebPEncodeLosslessRGB`, `WebPEncodeLosslessBGR`, `WebPEncodeLosslessRGBA` and `WebPEncodeLosslessBGRA` from `src/enc/picture_enc.c`, because the simple encoding API calls `WebPEncode`.
