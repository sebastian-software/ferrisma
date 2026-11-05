// libwebp v1.6.0, src/utils/quant_levels_dec_utils.c:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Implement gradient smoothing: we replace a current alpha value by its
// surrounding average if it's close enough (that is: the change will be less
// than the minimum distance between two quantized level).
// We use sliding window for computing the 2d moving average.
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/quant_levels_dec_utils.h:
//
// Copyright 2013 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Alpha plane de-quantization utility
//
// Author:  Vikas Arora (vikasa@google.com)

//! Translation of `src/utils/quant_levels_dec_utils.c` and `src/utils/quant_levels_dec_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP4 (pixel ops).
