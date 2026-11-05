// libwebp v1.6.0, src/dsp/yuv.c:
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
// YUV->RGB conversion functions
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/dsp/yuv.h:
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
// inline YUV<->RGB conversion function
//
// The exact naming is Y'CbCr, following the ITU-R BT.601 standard.
// More information at: https://en.wikipedia.org/wiki/YCbCr
// Y = 0.2568 * R + 0.5041 * G + 0.0979 * B + 16
// U = -0.1482 * R - 0.2910 * G + 0.4392 * B + 128
// V = 0.4392 * R - 0.3678 * G - 0.0714 * B + 128
// We use 16bit fixed point operations for RGB->YUV conversion (YUV_FIX).
//
// For the Y'CbCr to RGB conversion, the BT.601 specification reads:
//   R = 1.164 * (Y-16) + 1.596 * (V-128)
//   G = 1.164 * (Y-16) - 0.813 * (V-128) - 0.392 * (U-128)
//   B = 1.164 * (Y-16)                   + 2.017 * (U-128)
// where Y is in the [16,235] range, and U/V in the [16,240] range.
//
// The fixed-point implementation used here is:
//  R = (19077 . y             + 26149 . v - 14234) >> 6
//  G = (19077 . y -  6419 . u - 13320 . v +  8708) >> 6
//  B = (19077 . y + 33050 . u             - 17685) >> 6
// where the '.' operator is the mulhi_epu16 variant:
//   a . b = ((a << 8) * b) >> 16
// that preserves 8 bits of fractional precision before final descaling.

//! Translation of `src/dsp/yuv.c` and `src/dsp/yuv.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).
