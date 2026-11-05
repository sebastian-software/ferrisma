// libwebp v1.6.0, src/enc/backward_references_cost_enc.c:
//
// Copyright 2017 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Improves a given set of backward references by analyzing its bit cost.
// The algorithm is similar to the Zopfli compression algorithm but tailored to
// images.
//
// Author: Vincent Rabaud (vrabaud@google.com)
//

//! Translation of `src/enc/backward_references_cost_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
