// libwebp v1.6.0, src/dec/vp8li_dec.h:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Lossless decoder: internal header.
//
// Author: Skal (pascal.massimino@gmail.com)
//         Vikas Arora(vikaas.arora@gmail.com)

//! Translation of `src/dec/vp8li_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP7 (VP8L decoder).
//!
//! Exception: WP3 translates `VP8LTransform` from `src/dec/vp8li_dec.h`, because the inverse transforms in `src/dsp/lossless.c` take it.

use crate::webp::format_constants::VP8LImageTransformType;

// upstream: src/dec/vp8li_dec.h VP8LTransform
/// One VP8L image transform. Exception item of WP3 in WP7's module
/// (`tools/units.json`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VP8LTransform {
    /// Transform type.
    pub r#type: VP8LImageTransformType,
    /// Subsampling bits defining transform window.
    pub bits: i32,
    /// Transform window X index.
    pub xsize: i32,
    /// Transform window Y index.
    pub ysize: i32,
    /// Transform data, which the transform owns; empty where C has
    /// `data == NULL`.
    pub data: Vec<u32>,
}
