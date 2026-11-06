// libwebp v1.6.0, src/webp/types.h:
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
//  Common types + memory wrappers
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/webp/types.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).

// The header also declares `WebPMalloc` and `WebPFree`, which src/utils/utils.c
// defines. They hand memory to C callers, so they belong to the C ABI module
// (ADR 0004, `src/abi/types.rs`, WP12) and have no safe counterpart here.

// upstream: src/webp/types.h WEBP_ABI_IS_INCOMPATIBLE
/// Reports whether two ABI version numbers differ in their major revision.
#[must_use]
pub const fn webp_abi_is_incompatible(a: i32, b: i32) -> bool {
    (a >> 8) != (b >> 8)
}

/// Where the samples of a buffer live (ADR 0003, decided 2026-11-06): memory
/// that the object allocated itself, or the caller's memory, borrowed for `'a`.
/// This type has no C counterpart; it is the safe form of the pointers of
/// `WebPDecBuffer`, `WebPPicture` and their planes.
#[derive(Debug)]
pub enum PlaneStorage<'a, T> {
    /// Memory that the object allocated and frees (C: `private_memory`, `memory_`).
    /// It holds every plane of the buffer, and planes are told apart by offset.
    Internal(Vec<T>),
    /// The caller's slices. An RGBA or ARGB buffer uses one slice; a YUVA buffer
    /// uses up to four, in the order Y, U, V and A.
    External(Vec<&'a mut [T]>),
}

impl<T> Default for PlaneStorage<'_, T> {
    fn default() -> Self {
        Self::Internal(Vec::new())
    }
}

impl<T> PlaneStorage<'_, T> {
    /// The samples of region `region`, or `None` when there is no such region.
    #[must_use]
    pub fn region(&self, region: usize) -> Option<&[T]> {
        match self {
            Self::Internal(samples) => (region == 0).then_some(samples.as_slice()),
            Self::External(slices) => slices.get(region).map(|slice| &**slice),
        }
    }

    /// The samples of region `region`, mutably; see [`PlaneStorage::region`].
    pub fn region_mut(&mut self, region: usize) -> Option<&mut [T]> {
        match self {
            Self::Internal(samples) => (region == 0).then_some(samples.as_mut_slice()),
            Self::External(slices) => slices.get_mut(region).map(|slice| &mut **slice),
        }
    }
}

/// Where a plane starts inside a [`PlaneStorage`] (ADR 0003): the region index,
/// which is always 0 for `Internal` storage, and the element offset inside it.
/// A C `uint8_t*` or `uint32_t*` into a buffer becomes one of these.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlaneOffset {
    /// The region of the storage that holds the plane.
    pub region: usize,
    /// The element offset of the plane's first sample inside the region.
    pub offset: usize,
}
