// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Speed-critical functions for Sharp YUV.
//
// Author: Skal (pascal.massimino@gmail.com)

// Mechanically translated from upstream/libwebp/sharpyuv/sharpyuv_sse2.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
#[no_mangle]
pub unsafe extern "C" fn InitSharpYuvSSE2() {}
