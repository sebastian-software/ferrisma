// libwebp v1.6.0, src/webp/mux_types.h:
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
// Data-types common to the mux and demux libraries.
//
// Author: Urvang (urvang@google.com)

//! Translation of `src/webp/mux_types.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! Not translated yet, pending open question 2 in `docs/status/wp1.md`: the
//! `WebPData` struct, which is borrowed input in some calls and owned output in
//! others, and its inline helpers `WebPDataInit`, `WebPDataClear` and
//! `WebPDataCopy`.

// upstream: src/webp/mux_types.h WebPFeatureFlags
/// VP8X feature flags (C: `WebPFeatureFlags`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPFeatureFlags(pub u32);

impl WebPFeatureFlags {
    // upstream: src/webp/mux_types.h ANIMATION_FLAG
    pub const ANIMATION_FLAG: Self = Self(0x0000_0002);
    // upstream: src/webp/mux_types.h XMP_FLAG
    pub const XMP_FLAG: Self = Self(0x0000_0004);
    // upstream: src/webp/mux_types.h EXIF_FLAG
    pub const EXIF_FLAG: Self = Self(0x0000_0008);
    // upstream: src/webp/mux_types.h ALPHA_FLAG
    pub const ALPHA_FLAG: Self = Self(0x0000_0010);
    // upstream: src/webp/mux_types.h ICCP_FLAG
    pub const ICCP_FLAG: Self = Self(0x0000_0020);
    // upstream: src/webp/mux_types.h ALL_VALID_FLAGS
    pub const ALL_VALID_FLAGS: Self = Self(0x0000_003e);
}

// upstream: src/webp/mux_types.h WebPMuxAnimDispose
/// Dispose method of an animation frame (C: `WebPMuxAnimDispose`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPMuxAnimDispose(pub u32);

impl WebPMuxAnimDispose {
    // upstream: src/webp/mux_types.h WEBP_MUX_DISPOSE_NONE
    pub const WEBP_MUX_DISPOSE_NONE: Self = Self(0);
    // upstream: src/webp/mux_types.h WEBP_MUX_DISPOSE_BACKGROUND
    pub const WEBP_MUX_DISPOSE_BACKGROUND: Self = Self(1);
}

// upstream: src/webp/mux_types.h WebPMuxAnimBlend
/// Blend operation of an animation frame (C: `WebPMuxAnimBlend`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPMuxAnimBlend(pub u32);

impl WebPMuxAnimBlend {
    // upstream: src/webp/mux_types.h WEBP_MUX_BLEND
    pub const WEBP_MUX_BLEND: Self = Self(0);
    // upstream: src/webp/mux_types.h WEBP_MUX_NO_BLEND
    pub const WEBP_MUX_NO_BLEND: Self = Self(1);
}
