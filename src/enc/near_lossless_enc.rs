// libwebp v1.6.0, src/enc/near_lossless_enc.c:
//
// Copyright 2014 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Near-lossless image preprocessing adjusts pixel values to help
// compressibility with a guarantee of maximum deviation between original and
// resulting pixel values.
//
// Author: Jyrki Alakuijala (jyrki@google.com)
// Converted to C by Aleksander Kramarz (akramarz@google.com)

//! Translation of `src/enc/near_lossless_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
