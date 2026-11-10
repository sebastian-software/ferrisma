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
//! `WebPData` is one type with `Cow` semantics (ADR 0003, decided 2026-11-06):
//! borrowed input in most calls, owned output where C allocates the bytes. Its
//! iterators, `WebPIterator` and `WebPChunkIterator`, are WP8's (exceptions in
//! `tools/units.json`).

use std::borrow::Cow;

use crate::utils::utils::webp_safe_malloc;

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

// upstream: src/webp/mux_types.h WebPData
/// Raw data such as chunk payloads or compressed bytes (C: `WebPData`). The bytes
/// are borrowed (input, or a view into a demuxer) or owned (output that
/// `WebPDataCopy` allocates); `Cow` keeps both in one type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebPData<'a> {
    /// The bytes. Empty data is the C state `bytes == NULL, size == 0`, or any empty view.
    pub bytes: Cow<'a, [u8]>,
}

// upstream: src/webp/mux_types.h WebPDataInit
impl Default for WebPData<'_> {
    /// Empty and borrowed (C: `WebPDataInit`, which zeroes the struct).
    fn default() -> Self {
        Self {
            bytes: Cow::Borrowed(&[]),
        }
    }
}

// upstream: src/webp/mux_types.h WebPDataClear
/// Resets the data to empty and drops owned bytes (C: `WebPDataClear`, which frees
/// `bytes` and then calls `WebPDataInit`).
pub fn webp_data_clear(webp_data: &mut WebPData<'_>) {
    *webp_data = WebPData::default();
}

// upstream: src/webp/mux_types.h WebPDataCopy
/// Copies the bytes into owned memory (C: `WebPDataCopy`). Empty data copies to
/// empty data. Returns `None` when the host refuses the allocation, where C
/// returns 0; the allocation test is the one of `WebPMalloc(size)`, which
/// `webp_safe_malloc::<u8>(size, 1)` applies with the same arithmetic.
#[must_use]
pub fn webp_data_copy(src: &WebPData<'_>) -> Option<WebPData<'static>> {
    if src.bytes.is_empty() {
        return Some(WebPData::default_owned());
    }
    let mut owned = webp_safe_malloc::<u8>(src.bytes.len() as u64, 1)?;
    owned.extend_from_slice(&src.bytes);
    Some(WebPData {
        bytes: Cow::Owned(owned),
    })
}

impl WebPData<'static> {
    /// Empty and owned, the result of copying empty data.
    fn default_owned() -> Self {
        Self {
            bytes: Cow::Owned(Vec::new()),
        }
    }
}
