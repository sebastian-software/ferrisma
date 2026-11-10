// libwebp v1.6.0, src/webp/decode.h:
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
//  Main decoding functions for WebP images.
//
// Author: Skal (pascal.massimino@gmail.com)

//! C ABI export of `src/webp/decode.h` from libwebp v1.6.0.
//!
//! Mirrors the header's structs with `repr(C)` and converts them to the
//! safe representations at the boundary; see
//! `docs/adr/0004-c-abi-boundary.md`.
//!
//! Work package: WP12 (decoder integration).
//!
//! # Memory
//!
//! - Decoded samples that C receives are in blocks of the platform `malloc`
//!   ([`crate::abi::types`]): the simple API copies the translation's buffer into
//!   one, which C releases with `WebPFree()` or `free()`; a buffer whose memory
//!   the library allocates (`private_memory`) receives one after the call that
//!   allocated it, and `WebPFreeDecBuffer()` releases it. This module records the
//!   size of every such block, so that a buffer that still holds one can be
//!   decoded into again, as C allows.
//! - Caller memory (`is_external_memory`, the `...Into` functions and the
//!   incremental decoder's output) is lent to the translation for the duration of
//!   one call: the Rust buffer views it through slices that end with the call, so
//!   that no Rust borrow of C memory survives while C reads or writes it. Planes
//!   whose memory overlaps share one region of the Rust storage, with their
//!   offsets in it.
//! - Inputs are borrowed for the duration of the call.
//!
//! # Cases that this module rejects
//!
//! C writes rows at `pointer + y * stride`; a negative stride in caller memory
//! writes before the pointer, outside the buffer that `decode.h` describes, and
//! input that overlaps the output memory would be read while it is written.
//! Rust cannot form those slices, so such calls fail as an invalid buffer does
//! (`NULL` or `VP8_STATUS_INVALID_PARAM`); `docs/status/wp12.md` records them.
//!
//! # The CPU query
//!
//! `VP8GetCPUInfo` points at the translation's `x86CPUInfo` on x86, and, with
//! the `simd` feature on 64-bit ARM, at its `armCPUInfo`, which answers `kNEON`
//! as upstream's NEON build installs it (WP20, ADR 0008). Elsewhere, and in the
//! scalar build on 64-bit ARM, it is `NULL`, as in the reference
//! configuration. The tools' `-noasm` stores `NULL`, which turns off every
//! selection that consults the query; the NEON variants that upstream
//! compiles in unconditionally stay selected.

use core::ffi::{c_int, c_void};
use core::mem::ManuallyDrop;
use core::ptr;
use core::slice;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Mutex, PoisonError};

use crate::abi::types::{malloc_copy, platform_free};
use crate::dec::buffer_dec::{is_null_plane, webp_init_dec_buffer_internal};
use crate::dec::idec_dec as idec;
use crate::dec::vp8_dec::webp_get_decoder_version as rs_webp_get_decoder_version;
use crate::dec::webp_dec as dec;
use crate::dsp::cpu::CPUFeature;
use crate::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPBitstreamFeatures, WebPDecBuffer as RsDecBuffer,
    WebPDecBufferPlanes, WebPDecoderConfig as RsDecoderConfig, WebPDecoderOptions,
    WebPRGBABuffer as RsRGBABuffer, WebPYUVABuffer as RsYUVABuffer, webp_is_rgb_mode,
};
use crate::webp::types::{PlaneOffset, PlaneStorage};

//------------------------------------------------------------------------------
// Mirrors of the structs with pointers

// upstream: src/webp/decode.h WebPRGBABuffer
/// View as RGBA (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPRGBABuffer {
    /// Pointer to RGBA samples.
    pub rgba: *mut u8,
    /// Stride in bytes from one scanline to the next.
    pub stride: c_int,
    /// Total size of the `rgba` buffer.
    pub size: usize,
}

// upstream: src/webp/decode.h WebPYUVABuffer
/// View as YUVA (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPYUVABuffer {
    /// Pointer to luma samples.
    pub y: *mut u8,
    /// Pointer to chroma U samples.
    pub u: *mut u8,
    /// Pointer to chroma V samples.
    pub v: *mut u8,
    /// Pointer to alpha samples.
    pub a: *mut u8,
    /// Luma stride.
    pub y_stride: c_int,
    /// Chroma U stride.
    pub u_stride: c_int,
    /// Chroma V stride.
    pub v_stride: c_int,
    /// Alpha stride.
    pub a_stride: c_int,
    /// Luma plane size.
    pub y_size: usize,
    /// Chroma U plane size.
    pub u_size: usize,
    /// Chroma V plane size.
    pub v_size: usize,
    /// Alpha plane size.
    pub a_size: usize,
}

/// The nameless union `u` of `WebPDecBuffer` (C layout).
#[expect(
    non_snake_case,
    reason = "the C member names RGBA and YUVA of the WebPDecBuffer union"
)]
#[repr(C)]
#[derive(Clone, Copy)]
pub union WebPDecBufferUnion {
    /// The RGBA view.
    pub RGBA: WebPRGBABuffer,
    /// The YUVA view.
    pub YUVA: WebPYUVABuffer,
}

// upstream: src/webp/decode.h WebPDecBuffer
/// Output buffer (C layout).
#[repr(C)]
pub struct WebPDecBuffer {
    /// Colorspace.
    pub colorspace: WEBP_CSP_MODE,
    /// Width.
    pub width: c_int,
    /// Height.
    pub height: c_int,
    /// If non-zero, `private_memory` is not used; 2 or more marks slow memory.
    pub is_external_memory: c_int,
    /// Nameless union of buffer parameters.
    pub u: WebPDecBufferUnion,
    /// Padding for later use.
    pub pad: [u32; 4],
    /// Internally allocated memory (only when `is_external_memory` is 0).
    pub private_memory: *mut u8,
}

// upstream: src/webp/decode.h WebPDecoderConfig
/// Main object storing the configuration for advanced decoding (C layout).
#[repr(C)]
pub struct WebPDecoderConfig {
    /// Immutable bitstream features (optional).
    pub input: WebPBitstreamFeatures,
    /// Output buffer (can point to external memory).
    pub output: WebPDecBuffer,
    /// Decoding options.
    pub options: WebPDecoderOptions,
}

//------------------------------------------------------------------------------
// Library memory blocks

/// The blocks of samples that this module gave to C buffers as
/// `private_memory`, with their sizes.
static LIBRARY_BLOCKS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());

fn library_blocks() -> std::sync::MutexGuard<'static, BTreeMap<usize, usize>> {
    LIBRARY_BLOCKS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

/// The size of a library block, or `None` for memory this module did not
/// allocate.
fn library_block_size(block: *mut u8) -> Option<usize> {
    library_blocks().get(&(block as usize)).copied()
}

/// Gives a library block to a C buffer, or copies the samples into the block
/// it already holds when the sizes agree. Returns null when `malloc` fails.
fn give_library_block(held: *mut u8, samples: &[u8]) -> *mut u8 {
    if !held.is_null() && library_block_size(held) == Some(samples.len()) {
        // SAFETY: `held` is a live library block of `samples.len()` bytes,
        // which no Rust slice views (the translation worked on a copy).
        unsafe { ptr::copy_nonoverlapping(samples.as_ptr(), held, samples.len()) };
        return held;
    }
    let block = malloc_copy(samples);
    if !block.is_null() {
        library_blocks().insert(block as usize, samples.len());
    }
    block
}

/// Releases a library block that a C buffer gives up.
///
/// # Safety
///
/// `block` is null or a block of this module that is not used again.
unsafe fn release_library_block(block: *mut u8) {
    library_blocks().remove(&(block as usize));
    // SAFETY: the caller gives up a block of the platform `malloc`.
    unsafe { platform_free(block) }
}

//------------------------------------------------------------------------------
// Caller memory

/// Memory that C lends to the translation: the start and the length of each
/// region of a Rust storage.
#[derive(Clone, Debug, Default)]
struct Lent(Vec<(*mut u8, usize)>);

impl Lent {
    /// The external storage over the regions, for the duration of one call.
    ///
    /// # Safety
    ///
    /// Every region is writable memory of its length that only the returned
    /// slices access until they are dropped, and the regions do not overlap.
    unsafe fn storage(&self) -> PlaneStorage<'static, u8> {
        PlaneStorage::External(
            self.0
                .iter()
                .map(|&(start, len)| {
                    // SAFETY: the caller guarantees that the region is valid,
                    // exclusive for the call and disjoint from the others.
                    unsafe { slice::from_raw_parts_mut(start, len) }
                })
                .collect(),
        )
    }

    /// Whether `[start, start + len)` overlaps a region.
    fn overlaps(&self, start: *const u8, len: usize) -> bool {
        self.0
            .iter()
            .any(|&(region, size)| ranges_overlap(region, size, start, len))
    }
}

/// Whether two byte ranges overlap.
fn ranges_overlap(a: *const u8, a_len: usize, b: *const u8, b_len: usize) -> bool {
    let (a, b) = (a as usize, b as usize);
    a < b.saturating_add(b_len) && b < a.saturating_add(a_len)
}

/// Caller planes `(pointer, size)` as regions: overlapping planes share one
/// region. Returns the regions and, per plane, its region and offset (`None`
/// for a null plane).
fn merge_planes(planes: &[(*mut u8, usize)]) -> (Lent, Vec<Option<PlaneOffset>>) {
    let mut order: Vec<usize> = (0..planes.len())
        .filter(|&i| !planes[i].0.is_null())
        .collect();
    order.sort_by_key(|&i| planes[i].0 as usize);
    let mut regions: Vec<(*mut u8, usize)> = Vec::new();
    let mut offsets = vec![None; planes.len()];
    for i in order {
        let (start, size) = planes[i];
        let address = start as usize;
        match regions.last_mut() {
            Some((region, len)) if address < (*region as usize).saturating_add(*len) => {
                let offset = address - *region as usize;
                *len = (*len).max(offset.saturating_add(size));
                offsets[i] = Some(PlaneOffset {
                    region: regions.len() - 1,
                    offset,
                });
            }
            _ => {
                regions.push((start, size));
                offsets[i] = Some(PlaneOffset {
                    region: regions.len() - 1,
                    offset: 0,
                });
            }
        }
    }
    (Lent(regions), offsets)
}

/// A plane offset for a region that does not exist: C's `NULL` in caller
/// memory (`src/dec/buffer_dec.rs`).
const NULL_PLANE: PlaneOffset = PlaneOffset {
    region: usize::MAX,
    offset: 0,
};

/// The memory of a C buffer as the translation sees it during one call.
enum BufferMemory {
    /// Library memory that the translation allocates (C: `private_memory`).
    Library,
    /// Caller memory, or library memory that this module cannot copy.
    Lent(Lent),
}

/// Converts a C output buffer into the translation's form. The storage holds
/// slices of caller memory for the duration of the call (see [`Lent`]).
/// Returns `None` for caller memory with a negative stride, which this module
/// rejects (see the module documentation).
///
/// # Safety
///
/// For `is_external_memory > 0`, the planes point to their declared sizes of
/// writable memory, which only the translation accesses during the call
/// (`decode.h`); `private_memory` is null or a live block.
unsafe fn buffer_to_rust(c: &WebPDecBuffer) -> Option<(RsDecBuffer<'static>, BufferMemory)> {
    let is_rgb = webp_is_rgb_mode(c.colorspace);
    let mut buffer = RsDecBuffer {
        colorspace: c.colorspace,
        width: c.width,
        height: c.height,
        pad: c.pad,
        ..RsDecBuffer::default()
    };
    // The view that C reads, which the colorspace selects.
    let planes: Vec<(*mut u8, usize, c_int)> = if is_rgb {
        // SAFETY: both members are plain data, so either view is initialized.
        let rgba = unsafe { c.u.RGBA };
        vec![(rgba.rgba, rgba.size, rgba.stride)]
    } else {
        // SAFETY: as above.
        let yuva = unsafe { c.u.YUVA };
        vec![
            (yuva.y, yuva.y_size, yuva.y_stride),
            (yuva.u, yuva.u_size, yuva.u_stride),
            (yuva.v, yuva.v_size, yuva.v_stride),
            (yuva.a, yuva.a_size, yuva.a_stride),
        ]
    };
    let library = c.is_external_memory <= 0;
    let memory;
    let offsets: Vec<Option<PlaneOffset>>;
    if library && c.private_memory.is_null() {
        // The translation allocates; C ignores the plane pointers until then.
        memory = BufferMemory::Library;
        offsets = vec![Some(PlaneOffset::default()); planes.len()];
    } else if let Some(size) = library
        .then(|| library_block_size(c.private_memory))
        .flatten()
        .filter(|&size| {
            planes.iter().all(|&(plane, _, _)| {
                plane.is_null() || ranges_overlap(c.private_memory, size, plane, 1)
            })
        })
    {
        // A library block that the buffer still holds: the translation works on
        // a copy, which goes back into the block after the call.
        // SAFETY: the block is a live library block of `size` bytes.
        let block = unsafe { slice::from_raw_parts(c.private_memory, size) };
        buffer.storage = PlaneStorage::Internal(block.to_vec());
        memory = BufferMemory::Library;
        offsets = planes
            .iter()
            .map(|&(plane, _, _)| {
                (!plane.is_null()).then(|| PlaneOffset {
                    region: 0,
                    offset: plane as usize - c.private_memory as usize,
                })
            })
            .collect();
    } else {
        if planes
            .iter()
            .any(|&(plane, _, stride)| !plane.is_null() && stride < 0)
        {
            return None;
        }
        let (lent, plane_offsets) = merge_planes(
            &planes
                .iter()
                .map(|&(plane, size, _)| (plane, size))
                .collect::<Vec<_>>(),
        );
        // SAFETY: the caller lends the planes for the call; merged, the
        // regions are disjoint.
        buffer.storage = unsafe { lent.storage() };
        // C: is_external_memory >= 2 marks slow external memory.
        buffer.slow_memory = c.is_external_memory >= 2;
        memory = BufferMemory::Lent(lent);
        offsets = plane_offsets;
    }
    let plane = |i: usize| offsets[i].unwrap_or(NULL_PLANE);
    buffer.u = if is_rgb {
        // SAFETY: plain data, see above.
        let rgba = unsafe { c.u.RGBA };
        WebPDecBufferPlanes::RGBA(RsRGBABuffer {
            rgba: plane(0),
            stride: rgba.stride,
            size: rgba.size,
        })
    } else {
        // SAFETY: plain data, see above.
        let yuva = unsafe { c.u.YUVA };
        WebPDecBufferPlanes::YUVA(RsYUVABuffer {
            y: plane(0),
            u: plane(1),
            v: plane(2),
            a: if matches!(memory, BufferMemory::Library) && buffer.storage_is_empty() {
                None
            } else {
                offsets[3]
            },
            y_stride: yuva.y_stride,
            u_stride: yuva.u_stride,
            v_stride: yuva.v_stride,
            a_stride: yuva.a_stride,
            y_size: yuva.y_size,
            u_size: yuva.u_size,
            v_size: yuva.v_size,
            a_size: yuva.a_size,
        })
    };
    Some((buffer, memory))
}

/// Whether a buffer holds no memory (no C counterpart).
trait StorageIsEmpty {
    fn storage_is_empty(&self) -> bool;
}

impl StorageIsEmpty for RsDecBuffer<'_> {
    fn storage_is_empty(&self) -> bool {
        matches!(&self.storage, PlaneStorage::Internal(memory) if memory.is_empty())
    }
}

/// The C pointer of a plane: the start of its region plus its offset, or
/// null. `bases` are the starts of the regions.
fn plane_pointer(bases: &[*mut u8], storage: &PlaneStorage<'_, u8>, plane: PlaneOffset) -> *mut u8 {
    if is_null_plane(storage, plane) {
        return ptr::null_mut();
    }
    bases
        .get(plane.region)
        .map_or(ptr::null_mut(), |&base| base.wrapping_add(plane.offset))
}

/// Writes the translation's buffer back into the C buffer: dimensions,
/// strides, sizes and plane pointers, and `private_memory` for library memory.
/// Library samples move into a library block. A plane pointer of a buffer
/// whose library memory was never allocated or was released keeps its C value,
/// as C leaves it. Empty library memory releases the C buffer's block when
/// `released` says that the translation released it (C: `WebPFreeDecBuffer`
/// in `DecodeInto`). Returns false when the block for the samples could not be
/// allocated.
///
/// # Safety
///
/// `c` is the buffer that `buffer` was converted from, and `memory` its memory.
unsafe fn buffer_to_c(
    c: &mut WebPDecBuffer,
    buffer: &RsDecBuffer<'_>,
    memory: &BufferMemory,
    released: bool,
) -> bool {
    let mut ok = true;
    c.width = buffer.width;
    c.height = buffer.height;
    c.pad = buffer.pad;
    let bases: Vec<*mut u8> = match (&buffer.storage, memory) {
        (PlaneStorage::Internal(samples), BufferMemory::Library) if !samples.is_empty() => {
            let block = give_library_block(c.private_memory, samples);
            if block != c.private_memory && !c.private_memory.is_null() {
                // SAFETY: the buffer gives up the block it held.
                unsafe { release_library_block(c.private_memory) };
            }
            c.private_memory = block;
            ok = !block.is_null();
            if block.is_null() {
                Vec::new()
            } else {
                vec![block]
            }
        }
        (PlaneStorage::Internal(_), BufferMemory::Library) => {
            if released && !c.private_memory.is_null() {
                // The translation released the buffer's memory.
                // SAFETY: the buffer gives up its library block.
                unsafe { release_library_block(c.private_memory) };
                c.private_memory = ptr::null_mut();
            }
            Vec::new()
        }
        (_, BufferMemory::Lent(lent)) => lent.0.iter().map(|&(start, _)| start).collect(),
        (_, BufferMemory::Library) => Vec::new(),
    };
    let keep_pointers = bases.is_empty() && matches!(memory, BufferMemory::Library);
    match &buffer.u {
        WebPDecBufferPlanes::RGBA(rgba) if webp_is_rgb_mode(buffer.colorspace) => {
            // SAFETY: writing plain data into the RGBA view.
            let view = unsafe { &mut c.u.RGBA };
            if !keep_pointers {
                view.rgba = plane_pointer(&bases, &buffer.storage, rgba.rgba);
            }
            view.stride = rgba.stride;
            view.size = rgba.size;
        }
        WebPDecBufferPlanes::YUVA(yuva) if !webp_is_rgb_mode(buffer.colorspace) => {
            // SAFETY: writing plain data into the YUVA view.
            let view = unsafe { &mut c.u.YUVA };
            if !keep_pointers {
                view.y = plane_pointer(&bases, &buffer.storage, yuva.y);
                view.u = plane_pointer(&bases, &buffer.storage, yuva.u);
                view.v = plane_pointer(&bases, &buffer.storage, yuva.v);
                if let Some(a) = yuva.a {
                    view.a = plane_pointer(&bases, &buffer.storage, a);
                } else if matches!(memory, BufferMemory::Lent(_)) {
                    view.a = ptr::null_mut();
                }
            }
            view.y_stride = yuva.y_stride;
            view.u_stride = yuva.u_stride;
            view.v_stride = yuva.v_stride;
            view.a_stride = yuva.a_stride;
            view.y_size = yuva.y_size;
            view.u_size = yuva.u_size;
            view.v_size = yuva.v_size;
            view.a_size = yuva.a_size;
        }
        // A member that the colorspace does not select was never written by
        // the translation (`src/dec/buffer_dec.rs`): C's view is unchanged.
        _ => {}
    }
    ok
}

//------------------------------------------------------------------------------
// Helpers

/// The caller's input of `size` bytes.
///
/// # Safety
///
/// `data` is not null and points to `size` readable bytes that nothing writes
/// during the call.
unsafe fn input<'a>(data: *const u8, size: usize) -> &'a [u8] {
    // SAFETY: the caller's contract of `decode.h`.
    unsafe { slice::from_raw_parts(data, size) }
}

/// An optional `int` output parameter.
///
/// # Safety
///
/// `ptr` is null or points to a writable `int`.
unsafe fn out<'a>(ptr: *mut c_int) -> Option<&'a mut i32> {
    // SAFETY: null or a valid, exclusive `int` (caller's contract).
    unsafe { ptr.as_mut() }
}

/// Writes an optional output parameter.
///
/// # Safety
///
/// `ptr` is null or points to a writable `T`.
unsafe fn set<T>(ptr: *mut T, value: T) {
    if !ptr.is_null() {
        // SAFETY: a valid, writable `T` (caller's contract).
        unsafe { ptr.write(value) };
    }
}

//------------------------------------------------------------------------------
// Decoder version and CPU information

// upstream: src/dec/vp8_dec.c WebPGetDecoderVersion
/// Returns the decoder's version number, packed in hexadecimal using 8 bits
/// for each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[unsafe(export_name = "WebPGetDecoderVersion")]
#[must_use]
pub extern "C" fn webp_get_decoder_version() -> c_int {
    rs_webp_get_decoder_version()
}

/// `VP8GetCPUInfo` as C calls it (C: `x86CPUInfo`).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern "C" fn x86_cpu_info(feature: CPUFeature) -> c_int {
    c_int::from(crate::dsp::cpu::vp8_get_cpu_info(feature))
}

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// The CPU query that `src/dsp/cpu.h` declares as a function pointer, which the
/// upstream tools set to `NULL` for `-noasm` (ADR 0004, data symbols). It points
/// at the translation's query on x86, and is `NULL` elsewhere, as in the
/// reference configuration. No dispatch reads it before wave 4 (ADR 0003).
/// Exported here because `dwebp` assigns it.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[unsafe(export_name = "VP8GetCPUInfo")]
pub static VP8_GET_CPU_INFO: AtomicPtr<c_void> = AtomicPtr::new(x86_cpu_info as *mut c_void);

/// `VP8GetCPUInfo` as C calls it in the NEON build (C: `armCPUInfo`).
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
extern "C" fn arm_cpu_info(feature: CPUFeature) -> c_int {
    c_int::from(crate::dsp::cpu::vp8_get_cpu_info(feature))
}

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// See the x86 definition. With the `simd` feature on 64-bit ARM the pointer
/// is `armCPUInfo`, as in upstream's NEON build, which answers `kNEON`
/// (WP20, ADR 0008); the tools' `-noasm` sets it to `NULL`.
#[cfg(all(feature = "simd", target_arch = "aarch64"))]
#[unsafe(export_name = "VP8GetCPUInfo")]
pub static VP8_GET_CPU_INFO: AtomicPtr<c_void> = AtomicPtr::new(arm_cpu_info as *mut c_void);

// upstream: src/dsp/cpu.c VP8GetCPUInfo
/// See the x86 definition: `NULL` without an x86 query, and in the scalar
/// build on 64-bit ARM, as in the reference configuration.
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    all(feature = "simd", target_arch = "aarch64")
)))]
#[unsafe(export_name = "VP8GetCPUInfo")]
pub static VP8_GET_CPU_INFO: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

/// A token of the current `VP8GetCPUInfo` pointer for the dispatch cache of
/// `crate::dsp::cpu::features` (ADR 0008): 0 for `NULL` (`-noasm`), 1 for the
/// translation's own query, and a fold of the address of a query that the
/// caller installed, with its low bits set apart from the two fixed tokens.
pub(crate) fn cpu_query_token() -> u32 {
    let pointer = VP8_GET_CPU_INFO.load(Ordering::Acquire);
    if pointer.is_null() {
        return 0;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if pointer == x86_cpu_info as *mut c_void {
        return 1;
    }
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    if pointer == arm_cpu_info as *mut c_void {
        return 1;
    }
    let address = u64::try_from(pointer.addr()).unwrap_or(u64::MAX);
    #[expect(
        clippy::cast_possible_truncation,
        reason = "a 32-bit fold of the address"
    )]
    let fold = (address ^ (address >> 32)) as u32;
    (fold & !0b11) | 0b10
}

/// Answers `VP8GetCPUInfo(feature)` as C's `Init` functions call it (ADR
/// 0008): `None` for a `NULL` pointer, the translation's own query for its own
/// address, and the caller's function otherwise.
pub(crate) fn cpu_feature_query(feature: CPUFeature) -> Option<bool> {
    let pointer = VP8_GET_CPU_INFO.load(Ordering::Acquire);
    if pointer.is_null() {
        return None;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    if pointer == x86_cpu_info as *mut c_void {
        return Some(crate::dsp::cpu::vp8_get_cpu_info(feature));
    }
    #[cfg(all(feature = "simd", target_arch = "aarch64"))]
    if pointer == arm_cpu_info as *mut c_void {
        return Some(crate::dsp::cpu::vp8_get_cpu_info(feature));
    }
    // SAFETY: a non-null pointer that a caller stored in `VP8GetCPUInfo`, which
    // `cpu.h` declares as `VP8CPUInfo`, a function of one `CPUFeature`
    // returning `int`; calling it with a valid feature is its contract.
    let query =
        unsafe { core::mem::transmute::<*mut c_void, extern "C" fn(CPUFeature) -> c_int>(pointer) };
    Some(query(feature) != 0)
}

//------------------------------------------------------------------------------
// Simple API

// upstream: src/dec/webp_dec.c WebPGetInfo
/// Retrieves basic header information: width, height.
///
/// # Safety
///
/// `data` is null or points to `data_size` readable bytes; `width` and
/// `height` are null or writable.
#[unsafe(export_name = "WebPGetInfo")]
pub unsafe extern "C" fn webp_get_info(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
) -> c_int {
    if data.is_null() {
        return 0; // C: GetFeatures returns VP8_STATUS_INVALID_PARAM.
    }
    // SAFETY: non-null input of `data_size` bytes under the caller's contract.
    let data = unsafe { input(data, data_size) };
    // SAFETY: null or writable, under the caller's contract.
    let width = unsafe { out(width) };
    // SAFETY: as above.
    let height = unsafe { out(height) };
    c_int::from(dec::webp_get_info(data, width, height))
}

/// A function of the translation's simple API.
type SimpleDecode = fn(&[u8], Option<&mut i32>, Option<&mut i32>) -> Option<Vec<u8>>;

/// A function of the translation's `...Into` API for RGB samples.
type IntoDecode = for<'a> fn(&[u8], &'a mut [u8], i32) -> Option<&'a mut [u8]>;

/// The simple API: decodes with `decode` and copies the samples into a
/// `malloc` block.
///
/// # Safety
///
/// As [`webp_get_info`].
unsafe fn simple(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
    decode: SimpleDecode,
) -> *mut u8 {
    if data.is_null() {
        return ptr::null_mut(); // C: WebPGetInfo fails.
    }
    // SAFETY: non-null input of `data_size` bytes under the caller's contract.
    let data = unsafe { input(data, data_size) };
    // SAFETY: null or writable, under the caller's contract.
    let width = unsafe { out(width) };
    // SAFETY: as above.
    let height = unsafe { out(height) };
    decode(data, width, height).map_or(ptr::null_mut(), |samples| malloc_copy(&samples))
}

// upstream: src/dec/webp_dec.c WebPDecodeRGBA
/// Decodes into RGBA samples; release the result with `WebPFree()`.
///
/// # Safety
///
/// As [`webp_get_info`].
#[unsafe(export_name = "WebPDecodeRGBA")]
pub unsafe extern "C" fn webp_decode_rgba(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe { simple(data, data_size, width, height, dec::webp_decode_rgba) }
}

// upstream: src/dec/webp_dec.c WebPDecodeARGB
/// Decodes into ARGB samples; release the result with `WebPFree()`.
///
/// # Safety
///
/// As [`webp_get_info`].
#[unsafe(export_name = "WebPDecodeARGB")]
pub unsafe extern "C" fn webp_decode_argb(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe { simple(data, data_size, width, height, dec::webp_decode_argb) }
}

// upstream: src/dec/webp_dec.c WebPDecodeBGRA
/// Decodes into BGRA samples; release the result with `WebPFree()`.
///
/// # Safety
///
/// As [`webp_get_info`].
#[unsafe(export_name = "WebPDecodeBGRA")]
pub unsafe extern "C" fn webp_decode_bgra(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe { simple(data, data_size, width, height, dec::webp_decode_bgra) }
}

// upstream: src/dec/webp_dec.c WebPDecodeRGB
/// Decodes into RGB samples; release the result with `WebPFree()`.
///
/// # Safety
///
/// As [`webp_get_info`].
#[unsafe(export_name = "WebPDecodeRGB")]
pub unsafe extern "C" fn webp_decode_rgb(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe { simple(data, data_size, width, height, dec::webp_decode_rgb) }
}

// upstream: src/dec/webp_dec.c WebPDecodeBGR
/// Decodes into BGR samples; release the result with `WebPFree()`.
///
/// # Safety
///
/// As [`webp_get_info`].
#[unsafe(export_name = "WebPDecodeBGR")]
pub unsafe extern "C" fn webp_decode_bgr(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe { simple(data, data_size, width, height, dec::webp_decode_bgr) }
}

// upstream: src/dec/webp_dec.c WebPDecodeYUV
/// Decodes into Y, U and V planes in one block, which starts at the returned
/// luma pointer; release it with `WebPFree()`.
///
/// # Safety
///
/// As [`webp_get_info`]; `u`, `v`, `stride` and `uv_stride` are null or
/// writable.
#[unsafe(export_name = "WebPDecodeYUV")]
pub unsafe extern "C" fn webp_decode_yuv(
    data: *const u8,
    data_size: usize,
    width: *mut c_int,
    height: *mut c_int,
    u: *mut *mut u8,
    v: *mut *mut u8,
    stride: *mut c_int,
    uv_stride: *mut c_int,
) -> *mut u8 {
    // data, width and height are checked by Decode().
    if u.is_null() || v.is_null() || stride.is_null() || uv_stride.is_null() || data.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: non-null input of `data_size` bytes under the caller's contract.
    let data = unsafe { input(data, data_size) };
    // SAFETY: null or writable, under the caller's contract.
    let width = unsafe { out(width) };
    // SAFETY: as above.
    let height = unsafe { out(height) };
    let Some(buffer) = dec::webp_decode_yuv(data, width, height) else {
        return ptr::null_mut();
    };
    let (PlaneStorage::Internal(samples), WebPDecBufferPlanes::YUVA(yuva)) =
        (&buffer.storage, &buffer.u)
    else {
        return ptr::null_mut();
    };
    let block = malloc_copy(samples);
    if block.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: the four output parameters are writable (checked above and the
    // caller's contract).
    unsafe {
        set(u, block.wrapping_add(yuva.u.offset));
    }
    // SAFETY: as above.
    unsafe {
        set(v, block.wrapping_add(yuva.v.offset));
    }
    // SAFETY: as above.
    unsafe {
        set(stride, yuva.y_stride);
    }
    // SAFETY: as above.
    unsafe {
        set(uv_stride, yuva.u_stride);
    }
    block.wrapping_add(yuva.y.offset)
}

/// The `...Into` functions for RGB samples.
///
/// # Safety
///
/// `data` as in [`webp_get_info`]; `output` is null or points to
/// `output_buffer_size` writable bytes.
unsafe fn into_rgb(
    data: *const u8,
    data_size: usize,
    output: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
    decode: IntoDecode,
) -> *mut u8 {
    if output.is_null() || data.is_null() || output_stride < 0 {
        return ptr::null_mut();
    }
    if ranges_overlap(data, data_size, output, output_buffer_size) {
        return ptr::null_mut();
    }
    // SAFETY: the caller's input, which does not overlap the output.
    let data = unsafe { input(data, data_size) };
    // SAFETY: the caller lends `output_buffer_size` writable bytes for the
    // call, which nothing else accesses during it.
    let buffer = unsafe { slice::from_raw_parts_mut(output, output_buffer_size) };
    match decode(data, buffer, output_stride) {
        Some(_) => output,
        None => ptr::null_mut(),
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeRGBAInto
/// Decodes into a pre-allocated buffer of `output_buffer_size` bytes with
/// `output_stride`. Returns `output`, or null on failure.
///
/// # Safety
///
/// See [`into_rgb`].
#[unsafe(export_name = "WebPDecodeRGBAInto")]
pub unsafe extern "C" fn webp_decode_rgba_into(
    data: *const u8,
    data_size: usize,
    output: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        into_rgb(
            data,
            data_size,
            output,
            output_buffer_size,
            output_stride,
            dec::webp_decode_rgba_into,
        )
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeARGBInto
/// See [`webp_decode_rgba_into`].
///
/// # Safety
///
/// See [`into_rgb`].
#[unsafe(export_name = "WebPDecodeARGBInto")]
pub unsafe extern "C" fn webp_decode_argb_into(
    data: *const u8,
    data_size: usize,
    output: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        into_rgb(
            data,
            data_size,
            output,
            output_buffer_size,
            output_stride,
            dec::webp_decode_argb_into,
        )
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeBGRAInto
/// See [`webp_decode_rgba_into`].
///
/// # Safety
///
/// See [`into_rgb`].
#[unsafe(export_name = "WebPDecodeBGRAInto")]
pub unsafe extern "C" fn webp_decode_bgra_into(
    data: *const u8,
    data_size: usize,
    output: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        into_rgb(
            data,
            data_size,
            output,
            output_buffer_size,
            output_stride,
            dec::webp_decode_bgra_into,
        )
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeRGBInto
/// See [`webp_decode_rgba_into`].
///
/// # Safety
///
/// See [`into_rgb`].
#[unsafe(export_name = "WebPDecodeRGBInto")]
pub unsafe extern "C" fn webp_decode_rgb_into(
    data: *const u8,
    data_size: usize,
    output: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        into_rgb(
            data,
            data_size,
            output,
            output_buffer_size,
            output_stride,
            dec::webp_decode_rgb_into,
        )
    }
}

// upstream: src/dec/webp_dec.c WebPDecodeBGRInto
/// See [`webp_decode_rgba_into`].
///
/// # Safety
///
/// See [`into_rgb`].
#[unsafe(export_name = "WebPDecodeBGRInto")]
pub unsafe extern "C" fn webp_decode_bgr_into(
    data: *const u8,
    data_size: usize,
    output: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
) -> *mut u8 {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        into_rgb(
            data,
            data_size,
            output,
            output_buffer_size,
            output_stride,
            dec::webp_decode_bgr_into,
        )
    }
}

/// Whether any two of the planes overlap.
fn planes_overlap(planes: &[(*mut u8, usize)]) -> bool {
    planes.iter().enumerate().any(|(i, &(a, a_len))| {
        planes[i + 1..]
            .iter()
            .any(|&(b, b_len)| ranges_overlap(a, a_len, b, b_len))
    })
}

// upstream: src/dec/webp_dec.c WebPDecodeYUVInto
/// Decodes the Y, U and V planes into pre-allocated buffers. Returns `luma`,
/// or null on failure.
///
/// # Safety
///
/// `data` as in [`webp_get_info`]; `luma`, `u` and `v` are null or point to
/// their sizes of writable bytes.
#[unsafe(export_name = "WebPDecodeYUVInto")]
pub unsafe extern "C" fn webp_decode_yuv_into(
    data: *const u8,
    data_size: usize,
    luma: *mut u8,
    luma_size: usize,
    luma_stride: c_int,
    u: *mut u8,
    u_size: usize,
    u_stride: c_int,
    v: *mut u8,
    v_size: usize,
    v_stride: c_int,
) -> *mut u8 {
    // C: a NULL luma fails at once; NULL chroma fails the buffer check before
    // anything is written.
    if luma.is_null() || u.is_null() || v.is_null() || data.is_null() {
        return ptr::null_mut();
    }
    if luma_stride < 0 || u_stride < 0 || v_stride < 0 {
        return ptr::null_mut();
    }
    let planes = [(luma, luma_size), (u, u_size), (v, v_size)];
    if planes_overlap(&planes)
        || planes
            .iter()
            .any(|&(plane, size)| ranges_overlap(data, data_size, plane, size))
    {
        return ptr::null_mut();
    }
    // SAFETY: the caller's input, which does not overlap the planes.
    let data = unsafe { input(data, data_size) };
    // SAFETY: the caller lends each plane for the call; they are disjoint.
    let luma_slice = unsafe { slice::from_raw_parts_mut(luma, luma_size) };
    // SAFETY: as above.
    let u_slice = unsafe { slice::from_raw_parts_mut(u, u_size) };
    // SAFETY: as above.
    let v_slice = unsafe { slice::from_raw_parts_mut(v, v_size) };
    match dec::webp_decode_yuv_into(
        data,
        luma_slice,
        luma_stride,
        u_slice,
        u_stride,
        v_slice,
        v_stride,
    ) {
        Some(_) => luma,
        None => ptr::null_mut(),
    }
}

//------------------------------------------------------------------------------
// Output buffers

// upstream: src/dec/buffer_dec.c WebPInitDecBufferInternal
/// Internal, version-checked, entry point of `WebPInitDecBuffer`.
///
/// # Safety
///
/// `buffer` is null or points to a writable `WebPDecBuffer`.
#[unsafe(export_name = "WebPInitDecBufferInternal")]
pub unsafe extern "C" fn webp_init_dec_buffer_internal_c(
    buffer: *mut WebPDecBuffer,
    version: c_int,
) -> c_int {
    let mut rust = RsDecBuffer::default();
    if !webp_init_dec_buffer_internal(&mut rust, version) || buffer.is_null() {
        return 0; // version mismatch, or NULL
    }
    // SAFETY: a writable buffer (checked non-null); all-zero bytes are a valid
    // `WebPDecBuffer` (null pointers, zero integers), as C's memset makes it.
    unsafe { ptr::write_bytes(buffer, 0, 1) };
    1
}

// upstream: src/dec/buffer_dec.c WebPFreeDecBuffer
/// Frees any memory associated with the buffer. Must always be called last.
/// Doesn't free the `buffer` structure itself.
///
/// # Safety
///
/// `buffer` is null or points to a `WebPDecBuffer` whose `private_memory` is
/// null or a block of this library.
#[unsafe(export_name = "WebPFreeDecBuffer")]
pub unsafe extern "C" fn webp_free_dec_buffer(buffer: *mut WebPDecBuffer) {
    // SAFETY: null or a valid buffer (caller's contract).
    let Some(buffer) = (unsafe { buffer.as_mut() }) else {
        return;
    };
    if buffer.is_external_memory <= 0 {
        // C: WebPSafeFree(buffer->private_memory).
        // SAFETY: the buffer's library block, which it gives up.
        unsafe { release_library_block(buffer.private_memory) };
    }
    buffer.private_memory = ptr::null_mut();
}

//------------------------------------------------------------------------------
// Advanced decoding API

// upstream: src/dec/webp_dec.c WebPGetFeaturesInternal
/// Internal, version-checked, entry point of `WebPGetFeatures`.
///
/// # Safety
///
/// `data` is null or points to `data_size` readable bytes; `features` is null
/// or writable.
#[unsafe(export_name = "WebPGetFeaturesInternal")]
pub unsafe extern "C" fn webp_get_features_internal(
    data: *const u8,
    data_size: usize,
    features: *mut WebPBitstreamFeatures,
    version: c_int,
) -> VP8StatusCode {
    // SAFETY: null or a valid, exclusive features structure.
    let Some(features) = (unsafe { features.as_mut() }) else {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    };
    if data.is_null() {
        // C: the version is checked first, with the same result.
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    }
    // SAFETY: non-null input under the caller's contract.
    let data = unsafe { input(data, data_size) };
    status_code(dec::webp_get_features_internal(data, features, version))
}

/// A `Result` as the C status.
fn status_code(result: Result<(), VP8StatusCode>) -> VP8StatusCode {
    result.err().unwrap_or(VP8StatusCode::VP8_STATUS_OK)
}

// upstream: src/dec/webp_dec.c WebPInitDecoderConfigInternal
/// Internal, version-checked, entry point of `WebPInitDecoderConfig`.
///
/// # Safety
///
/// `config` is null or points to a writable `WebPDecoderConfig`.
#[unsafe(export_name = "WebPInitDecoderConfigInternal")]
pub unsafe extern "C" fn webp_init_decoder_config_internal(
    config: *mut WebPDecoderConfig,
    version: c_int,
) -> c_int {
    let mut rust = RsDecoderConfig::default();
    if !dec::webp_init_decoder_config_internal(&mut rust, version) || config.is_null() {
        return 0;
    }
    // SAFETY: a writable configuration (checked non-null); all-zero bytes are
    // a valid configuration, which is what C's memset, DefaultFeatures and
    // WebPInitDecBuffer leave.
    unsafe { ptr::write_bytes(config, 0, 1) };
    1
}

/// The options and features of a C configuration with the colorspace of its
/// output, for the functions that read nothing else of the output.
fn config_without_memory(config: &WebPDecoderConfig) -> RsDecoderConfig<'static> {
    let mut rust = RsDecoderConfig {
        input: config.input,
        options: config.options,
        ..RsDecoderConfig::default()
    };
    rust.output.colorspace = config.output.colorspace;
    rust
}

// upstream: src/dec/webp_dec.c WebPValidateDecoderConfig
/// Checks that `config` is non-null and valid.
///
/// # Safety
///
/// `config` is null or points to a `WebPDecoderConfig`.
#[unsafe(export_name = "WebPValidateDecoderConfig")]
#[must_use]
pub unsafe extern "C" fn webp_validate_decoder_config(config: *const WebPDecoderConfig) -> c_int {
    // SAFETY: null or a valid configuration.
    let Some(config) = (unsafe { config.as_ref() }) else {
        return 0;
    };
    c_int::from(dec::webp_validate_decoder_config(&config_without_memory(
        config,
    )))
}

// upstream: src/dec/webp_dec.c WebPDecode
/// Non-incremental version: decodes `data` into `config->output`.
///
/// # Safety
///
/// `data` is null or points to `data_size` readable bytes; `config` is null or
/// a valid configuration whose output buffer follows `decode.h`.
#[unsafe(export_name = "WebPDecode")]
pub unsafe extern "C" fn webp_decode(
    data: *const u8,
    data_size: usize,
    config: *mut WebPDecoderConfig,
) -> VP8StatusCode {
    // SAFETY: null or a valid, exclusive configuration.
    let Some(config) = (unsafe { config.as_mut() }) else {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    };
    if data.is_null() {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM; // C: GetFeatures
    }
    // SAFETY: the caller's output buffer under `decode.h`.
    let Some((output, memory)) = (unsafe { buffer_to_rust(&config.output) }) else {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    };
    if let BufferMemory::Lent(lent) = &memory
        && lent.overlaps(data, data_size)
    {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    }
    // SAFETY: the caller's input, which does not overlap the output.
    let data = unsafe { input(data, data_size) };
    let mut rust = RsDecoderConfig {
        input: config.input,
        output,
        options: config.options,
    };
    let status = dec::webp_decode(data, &mut rust);
    config.input = rust.input;
    // SAFETY: the output was converted from this buffer with this memory.
    if !unsafe { buffer_to_c(&mut config.output, &rust.output, &memory, true) } {
        // The samples could not reach C: as C's failed allocation.
        return VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY;
    }
    status_code(status)
}

//------------------------------------------------------------------------------
// Incremental decoding

/// The incremental decoder behind C's opaque `WebPIDecoder*`.
///
/// The translation's decoder borrows the Rust forms of the caller's
/// configuration or output buffer, which live in leaked boxes until
/// `WebPIDelete()` drops the decoder and frees them. The memory of both its own
/// output buffer and the caller's is lent per call (`own_lent`, `user_lent`);
/// library samples move into a block after the call that allocates them: the
/// caller's buffer receives it as `private_memory`, and the decoder's own block
/// is freed at `WebPIDelete()`. With slow memory the decoder works in its own
/// buffer and copies into the caller's at the end.
pub struct WebPIDecoder {
    /// The translation's decoder.
    idec: ManuallyDrop<idec::WebPIDecoder<'static, 'static>>,
    /// The Rust configuration of `WebPIDecode`, or null.
    config: *mut RsDecoderConfig<'static>,
    /// The Rust output buffer of `WebPINewDecoder`, or null.
    user_buffer: *mut RsDecBuffer<'static>,
    /// The caller's C buffer (C: `params.output` or `final_output`), or null.
    user: *mut WebPDecBuffer,
    /// The memory of the decoder's own output buffer, lent for each call.
    own_lent: Lent,
    /// Whether `own_lent` is the decoder's library block.
    own_library_block: bool,
    /// The library block of the decoder's own output buffer, or null.
    own_block: *mut u8,
    /// The memory of the caller's output buffer, lent for each call.
    user_lent: Lent,
    /// The C view of the decoder's own output buffer (C: `idec->output`).
    own: WebPDecBuffer,
}

/// A zeroed C buffer.
fn zeroed_buffer() -> WebPDecBuffer {
    WebPDecBuffer {
        colorspace: WEBP_CSP_MODE::MODE_RGB,
        width: 0,
        height: 0,
        is_external_memory: 0,
        u: WebPDecBufferUnion {
            YUVA: WebPYUVABuffer {
                y: ptr::null_mut(),
                u: ptr::null_mut(),
                v: ptr::null_mut(),
                a: ptr::null_mut(),
                y_stride: 0,
                u_stride: 0,
                v_stride: 0,
                a_stride: 0,
                y_size: 0,
                u_size: 0,
                v_size: 0,
                a_size: 0,
            },
        },
        pad: [0; 4],
        private_memory: ptr::null_mut(),
    }
}

/// Lends `lent` to `buffer` for one call, if it holds caller or block memory.
///
/// # Safety
///
/// As [`Lent::storage`].
unsafe fn lend(buffer: &mut RsDecBuffer<'static>, lent: &Lent) {
    if !lent.0.is_empty() {
        // SAFETY: the caller's guarantee, passed on.
        buffer.storage = unsafe { lent.storage() };
    }
}

/// Ends the loan of a buffer's memory.
fn take_back(buffer: &mut RsDecBuffer<'static>) {
    if matches!(buffer.storage, PlaneStorage::External(_)) {
        buffer.storage = PlaneStorage::External(Vec::new());
    }
}

/// Moves the library samples of `buffer` into a library block, which the
/// buffer then views as lent memory. `held` is the block that the C buffer
/// already holds, or null. Returns the block, or null when there were no
/// samples or no block could be allocated.
fn move_samples(buffer: &mut RsDecBuffer<'static>, lent: &mut Lent, held: *mut u8) -> *mut u8 {
    let PlaneStorage::Internal(samples) = &buffer.storage else {
        return ptr::null_mut();
    };
    if samples.is_empty() {
        return ptr::null_mut();
    }
    let block = give_library_block(held, samples);
    if !block.is_null() {
        *lent = Lent(vec![(block, samples.len())]);
        // The block replaces the samples for the rest of the call.
        // SAFETY: a new block that only this storage views until the decoder
        // takes it back.
        buffer.storage = unsafe { lent.storage() };
    }
    block
}

/// The memory from which the C view of a buffer is written: its lent regions,
/// the translation's own samples when no block could be allocated, or none yet.
fn view_memory(buffer: &mut RsDecBuffer<'static>, lent: &Lent) -> BufferMemory {
    match &mut buffer.storage {
        PlaneStorage::Internal(samples) if !samples.is_empty() => {
            BufferMemory::Lent(Lent(vec![(samples.as_mut_ptr(), samples.len())]))
        }
        // Not allocated yet: C's plane pointers stay as they are.
        PlaneStorage::Internal(_) => BufferMemory::Library,
        _ => BufferMemory::Lent(lent.clone()),
    }
}

impl WebPIDecoder {
    /// Wraps a translation decoder whose own output memory is `own_lent` and
    /// whose caller output memory is `user_lent`.
    fn boxed(
        idec: idec::WebPIDecoder<'static, 'static>,
        own_lent: Lent,
        user_lent: Lent,
        config: *mut RsDecoderConfig<'static>,
        user_buffer: *mut RsDecBuffer<'static>,
        user: *mut WebPDecBuffer,
    ) -> *mut Self {
        let mut decoder = Box::new(Self {
            idec: ManuallyDrop::new(idec),
            config,
            user_buffer,
            user,
            own_lent,
            own_library_block: false,
            own_block: ptr::null_mut(),
            user_lent,
            own: zeroed_buffer(),
        });
        // The translation's buffers still view the lent memory, as during a
        // call: write the C views, then end the loan.
        // SAFETY: the views are written from the translation's buffers, which
        // were converted from them.
        unsafe { decoder.sync() };
        decoder.detach();
        Box::into_raw(decoder)
    }

    /// Lends the output memory to the translation's buffers for one call.
    ///
    /// # Safety
    ///
    /// The lent memory is valid and nothing else accesses it until
    /// [`Self::detach`].
    unsafe fn attach(&mut self) {
        // SAFETY: the caller's guarantee; each buffer's regions are disjoint.
        unsafe { lend(self.idec.own_output_mut(), &self.own_lent) };
        if let Some(caller) = self.idec.caller_output_mut() {
            // SAFETY: as above.
            unsafe { lend(caller, &self.user_lent) };
        }
    }

    /// Ends the loan of the output memory.
    fn detach(&mut self) {
        take_back(self.idec.own_output_mut());
        if let Some(caller) = self.idec.caller_output_mut() {
            take_back(caller);
        }
    }

    /// After a call: moves library samples into blocks and writes the C views
    /// of both output buffers.
    ///
    /// # Safety
    ///
    /// `self.user` is null or the caller's live buffer.
    unsafe fn sync(&mut self) {
        let user = self.user;
        // The decoder's own buffer.
        {
            let own = self.idec.own_output_mut();
            let block = move_samples(own, &mut self.own_lent, ptr::null_mut());
            if !block.is_null() {
                library_blocks().remove(&(block as usize));
                self.own_block = block;
                self.own_library_block = true;
                self.own.private_memory = block;
            }
            let memory = view_memory(own, &self.own_lent);
            let own: &RsDecBuffer<'_> = own;
            self.own.colorspace = own.colorspace;
            self.own.is_external_memory =
                c_int::from(!self.own_library_block && !self.own_lent.0.is_empty());
            // SAFETY: the decoder's own view, written from its own buffer. The
            // incremental decoder never releases output memory before it is
            // deleted (C frees the own buffer's memory only at the slow copy,
            // after which C's view is the caller's structure).
            let _ = unsafe { buffer_to_c(&mut self.own, own, &memory, false) };
        }
        // The caller's buffer.
        if let Some(caller) = self.idec.caller_output_mut() {
            // SAFETY: the caller's live buffer.
            let view = unsafe { &mut *user };
            let block = move_samples(caller, &mut self.user_lent, view.private_memory);
            if !block.is_null() {
                view.private_memory = block;
            }
            let memory = view_memory(caller, &self.user_lent);
            let caller: &RsDecBuffer<'_> = caller;
            // SAFETY: the caller's view, written from the buffer that was
            // converted from it.
            let _ = unsafe { buffer_to_c(view, caller, &memory, false) };
        }
    }

    /// Runs `call` with the output memory lent, then writes the C views.
    ///
    /// # Safety
    ///
    /// As [`Self::attach`] and [`Self::sync`].
    unsafe fn with<T>(
        &mut self,
        call: impl FnOnce(&mut idec::WebPIDecoder<'static, 'static>) -> T,
    ) -> T {
        // SAFETY: the caller's guarantee, passed on.
        unsafe { self.attach() };
        let result = call(&mut self.idec);
        // SAFETY: as above.
        unsafe { self.sync() };
        self.detach();
        result
    }

    /// Whether `[start, start + len)` overlaps the lent memory.
    fn overlaps(&self, start: *const u8, len: usize) -> bool {
        self.own_lent.overlaps(start, len) || self.user_lent.overlaps(start, len)
    }
}

/// The C view of a caller plane or of a plane in the decoder's memory: the
/// pointer that C returns.
fn rows_pointer(decoder: &WebPIDecoder, rows: idec::PlaneRows<'_>) -> *mut u8 {
    let Some((region, offset)) = rows else {
        return ptr::null_mut();
    };
    let base = region.as_ptr();
    let start = decoder
        .own_lent
        .0
        .iter()
        .chain(&decoder.user_lent.0)
        .find(|&&(start, _)| ptr::eq(start, base))
        .map_or(base.cast_mut(), |&(start, _)| start);
    start.wrapping_add(offset)
}

// upstream: src/dec/idec_dec.c WebPINewDecoder
/// Creates a new incremental decoder with the supplied buffer parameter, or a
/// default output buffer (`MODE_RGB`) for null.
///
/// # Safety
///
/// `output_buffer` is null or a valid buffer that outlives the decoder, whose
/// memory follows `decode.h`.
#[unsafe(export_name = "WebPINewDecoder")]
pub unsafe extern "C" fn webp_i_new_decoder(
    output_buffer: *mut WebPDecBuffer,
) -> *mut WebPIDecoder {
    if output_buffer.is_null() {
        let Some(idec) = idec::webp_i_new_decoder(None) else {
            return ptr::null_mut();
        };
        return WebPIDecoder::boxed(
            idec,
            Lent::default(),
            Lent::default(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
    // SAFETY: a valid buffer (checked non-null).
    let output = unsafe { &*output_buffer };
    // SAFETY: the caller's output buffer under `decode.h`.
    let Some((buffer, memory)) = (unsafe { buffer_to_rust(output) }) else {
        return ptr::null_mut();
    };
    let lent = match memory {
        BufferMemory::Lent(lent) => lent,
        BufferMemory::Library => Lent::default(),
    };
    let user_buffer = Box::into_raw(Box::new(buffer));
    // SAFETY: a leaked box, which the decoder borrows until WebPIDelete.
    let Some(idec) = idec::webp_i_new_decoder(Some(unsafe { &mut *user_buffer })) else {
        // SAFETY: the box that was leaked above, no longer borrowed.
        drop(unsafe { Box::from_raw(user_buffer) });
        return ptr::null_mut();
    };
    WebPIDecoder::boxed(
        idec,
        Lent::default(),
        lent,
        ptr::null_mut(),
        user_buffer,
        output_buffer,
    )
}

// upstream: src/dec/idec_dec.c WebPIDecode
/// Instantiates a new incremental decoder object with the requested
/// configuration.
///
/// # Safety
///
/// `data` is null or points to `data_size` readable bytes; `config` is null or
/// a valid configuration that outlives the decoder.
#[unsafe(export_name = "WebPIDecode")]
pub unsafe extern "C" fn webp_i_decode(
    data: *const u8,
    data_size: usize,
    config: *mut WebPDecoderConfig,
) -> *mut WebPIDecoder {
    // C: data != NULL && data_size > 0 parses the features.
    let data: &[u8] = if data.is_null() {
        &[]
    } else {
        // SAFETY: non-null input under the caller's contract.
        unsafe { input(data, data_size) }
    };
    if config.is_null() {
        let Some(idec) = idec::webp_i_decode(data, None) else {
            return ptr::null_mut();
        };
        return WebPIDecoder::boxed(
            idec,
            Lent::default(),
            Lent::default(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
    // SAFETY: a valid configuration (checked non-null).
    let c = unsafe { &mut *config };
    // SAFETY: the caller's output buffer under `decode.h`.
    let Some((output, memory)) = (unsafe { buffer_to_rust(&c.output) }) else {
        return ptr::null_mut();
    };
    let lent = match memory {
        BufferMemory::Lent(lent) => lent,
        BufferMemory::Library => Lent::default(),
    };
    if lent.overlaps(data.as_ptr(), data.len()) {
        return ptr::null_mut();
    }
    let rust = Box::into_raw(Box::new(RsDecoderConfig {
        input: c.input,
        output,
        options: c.options,
    }));
    // SAFETY: a leaked box, which the decoder borrows until WebPIDelete.
    let idec = idec::webp_i_decode(data, Some(unsafe { &mut *rust }));
    // C: the features were written into config->input.
    // SAFETY: `input` is a field that the decoder does not borrow.
    c.input = unsafe { (*rust).input };
    let Some(idec) = idec else {
        // SAFETY: the box that was leaked above, no longer borrowed.
        drop(unsafe { Box::from_raw(rust) });
        return ptr::null_mut();
    };
    let user = &raw mut c.output;
    WebPIDecoder::boxed(idec, Lent::default(), lent, rust, ptr::null_mut(), user)
}

// upstream: src/dec/idec_dec.c WebPINewRGB
/// Creates a decoder that outputs RGB samples into `output_buffer`, or into
/// memory it allocates for null.
///
/// # Safety
///
/// `output_buffer` is null or points to `output_buffer_size` writable bytes
/// that outlive the decoder.
#[unsafe(export_name = "WebPINewRGB")]
pub unsafe extern "C" fn webp_i_new_rgb(
    csp: WEBP_CSP_MODE,
    output_buffer: *mut u8,
    output_buffer_size: usize,
    output_stride: c_int,
) -> *mut WebPIDecoder {
    if !output_buffer.is_null() && output_stride < 0 {
        return ptr::null_mut();
    }
    let lent = if output_buffer.is_null() {
        Lent::default()
    } else {
        Lent(vec![(output_buffer, output_buffer_size)])
    };
    let buffer = (!output_buffer.is_null()).then(|| {
        // SAFETY: the caller lends the buffer; the decoder detaches it before
        // the call returns.
        unsafe { slice::from_raw_parts_mut(output_buffer, output_buffer_size) }
    });
    let Some(idec) = idec::webp_i_new_rgb(csp, buffer, output_stride) else {
        return ptr::null_mut();
    };
    WebPIDecoder::boxed(
        idec,
        lent,
        Lent::default(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    )
}

// upstream: src/dec/idec_dec.c WebPINewYUVA
/// Creates a decoder that outputs YUV(A) samples into the caller's planes, or
/// into memory it allocates when `luma` is null.
///
/// # Safety
///
/// When `luma` is not null, each non-null plane points to its size of
/// writable bytes that outlive the decoder.
#[unsafe(export_name = "WebPINewYUVA")]
pub unsafe extern "C" fn webp_i_new_yuva(
    luma: *mut u8,
    luma_size: usize,
    luma_stride: c_int,
    u: *mut u8,
    u_size: usize,
    u_stride: c_int,
    v: *mut u8,
    v_size: usize,
    v_stride: c_int,
    a: *mut u8,
    a_size: usize,
    a_stride: c_int,
) -> *mut WebPIDecoder {
    if luma.is_null() {
        // C ignores the other arguments.
        let Some(idec) = idec::webp_i_new_yuva(None, 0, None, 0, None, 0, None, 0) else {
            return ptr::null_mut();
        };
        return WebPIDecoder::boxed(
            idec,
            Lent::default(),
            Lent::default(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }
    let planes: Vec<(*mut u8, usize, c_int)> = [
        (luma, luma_size, luma_stride),
        (u, u_size, u_stride),
        (v, v_size, v_stride),
        (a, a_size, a_stride),
    ]
    .into_iter()
    .filter(|&(plane, _, _)| !plane.is_null())
    .collect();
    let ranges: Vec<(*mut u8, usize)> = planes.iter().map(|&(p, size, _)| (p, size)).collect();
    if planes_overlap(&ranges) || planes.iter().any(|&(_, _, stride)| stride < 0) {
        return ptr::null_mut();
    }
    let slice_of = |plane: *mut u8, size: usize| {
        (!plane.is_null()).then(|| {
            // SAFETY: the caller lends the plane; the planes are disjoint, and
            // the decoder detaches them before the call returns.
            unsafe { slice::from_raw_parts_mut(plane, size) }
        })
    };
    let Some(idec) = idec::webp_i_new_yuva(
        slice_of(luma, luma_size),
        luma_stride,
        slice_of(u, u_size),
        u_stride,
        slice_of(v, v_size),
        v_stride,
        slice_of(a, a_size),
        a_stride,
    ) else {
        return ptr::null_mut();
    };
    // The translation keeps the planes in the order Y, U, V and A.
    WebPIDecoder::boxed(
        idec,
        Lent(ranges),
        Lent::default(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    )
}

// upstream: src/dec/idec_dec.c WebPINewYUV
/// Deprecated version of `WebPINewYUVA()`, without alpha.
///
/// # Safety
///
/// As [`webp_i_new_yuva`].
#[unsafe(export_name = "WebPINewYUV")]
pub unsafe extern "C" fn webp_i_new_yuv(
    luma: *mut u8,
    luma_size: usize,
    luma_stride: c_int,
    u: *mut u8,
    u_size: usize,
    u_stride: c_int,
    v: *mut u8,
    v_size: usize,
    v_stride: c_int,
) -> *mut WebPIDecoder {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        webp_i_new_yuva(
            luma,
            luma_size,
            luma_stride,
            u,
            u_size,
            u_stride,
            v,
            v_size,
            v_stride,
            ptr::null_mut(),
            0,
            0,
        )
    }
}

// upstream: src/dec/idec_dec.c WebPIDelete
/// Deletes the decoder object and associated memory. The caller's output
/// buffer keeps its memory.
///
/// # Safety
///
/// `idec` is null or a decoder of this library that is not used again.
#[unsafe(export_name = "WebPIDelete")]
pub unsafe extern "C" fn webp_i_delete(idec: *mut WebPIDecoder) {
    if idec.is_null() {
        return;
    }
    // SAFETY: a decoder of this library, which the caller gives up.
    let mut decoder = unsafe { Box::from_raw(idec) };
    // SAFETY: the output memory is valid until the decoder is deleted.
    unsafe { decoder.attach() };
    // SAFETY: dropped once, before the boxes that it borrows.
    unsafe { ManuallyDrop::drop(&mut decoder.idec) };
    if !decoder.config.is_null() {
        // SAFETY: the leaked configuration, no longer borrowed.
        drop(unsafe { Box::from_raw(decoder.config) });
    }
    if !decoder.user_buffer.is_null() {
        // SAFETY: the leaked buffer, no longer borrowed.
        drop(unsafe { Box::from_raw(decoder.user_buffer) });
    }
    // C: WebPFreeDecBuffer(&idec->output).
    // SAFETY: the decoder's own block, which nothing uses any more.
    unsafe { platform_free(decoder.own_block) };
}

// upstream: src/dec/idec_dec.c WebPIAppend
/// Copies and decodes the next available data.
///
/// # Safety
///
/// `idec` is null or a live decoder; `data` is null or points to `data_size`
/// readable bytes.
#[unsafe(export_name = "WebPIAppend")]
pub unsafe extern "C" fn webp_i_append(
    idec: *mut WebPIDecoder,
    data: *const u8,
    data_size: usize,
) -> VP8StatusCode {
    // SAFETY: null or a live decoder.
    let Some(decoder) = (unsafe { idec.as_mut() }) else {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    };
    if data.is_null() || decoder.overlaps(data, data_size) {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    }
    // SAFETY: the caller's input, which does not overlap the output memory.
    let data = unsafe { input(data, data_size) };
    // SAFETY: the output memory is valid while the decoder lives.
    status_code(unsafe { decoder.with(|idec| idec::webp_i_append(idec, data)) })
}

// upstream: src/dec/idec_dec.c WebPIUpdate
/// A variant of `WebPIAppend()` for a buffer that holds all the data so far.
///
/// # Safety
///
/// As [`webp_i_append`].
#[unsafe(export_name = "WebPIUpdate")]
pub unsafe extern "C" fn webp_i_update(
    idec: *mut WebPIDecoder,
    data: *const u8,
    data_size: usize,
) -> VP8StatusCode {
    // SAFETY: null or a live decoder.
    let Some(decoder) = (unsafe { idec.as_mut() }) else {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    };
    if data.is_null() || decoder.overlaps(data, data_size) {
        return VP8StatusCode::VP8_STATUS_INVALID_PARAM;
    }
    // SAFETY: the caller's input, which does not overlap the output memory.
    let data = unsafe { input(data, data_size) };
    // SAFETY: the output memory is valid while the decoder lives.
    status_code(unsafe { decoder.with(|idec| idec::webp_i_update(idec, data)) })
}

// upstream: src/dec/idec_dec.c WebPIDecGetRGB
/// Returns the RGB/A image decoded so far, or null.
///
/// # Safety
///
/// `idec` is null or a live decoder; the output parameters are null or
/// writable.
#[unsafe(export_name = "WebPIDecGetRGB")]
pub unsafe extern "C" fn webp_i_dec_get_rgb(
    idec: *const WebPIDecoder,
    last_y: *mut c_int,
    width: *mut c_int,
    height: *mut c_int,
    stride: *mut c_int,
) -> *mut u8 {
    // SAFETY: null or a live decoder; C takes a const pointer and changes
    // nothing, and this function only lends and takes back the memory.
    let Some(decoder) = (unsafe { idec.cast_mut().as_mut() }) else {
        return ptr::null_mut();
    };
    // SAFETY: the output memory is valid while the decoder lives.
    unsafe { decoder.attach() };
    // SAFETY: null or writable output parameters (caller's contract).
    let last_y = unsafe { out(last_y) };
    // SAFETY: as above.
    let width = unsafe { out(width) };
    // SAFETY: as above.
    let height = unsafe { out(height) };
    // SAFETY: as above.
    let stride = unsafe { out(stride) };
    let rows = idec::webp_i_dec_get_rgb(&decoder.idec, last_y, width, height, stride);
    let pointer = rows_pointer(decoder, rows);
    decoder.detach();
    pointer
}

// upstream: src/dec/idec_dec.c WebPIDecGetYUVA
/// Same as `WebPIDecGetRGB()` but to retrieve the Y, U, V and A planes.
///
/// # Safety
///
/// As [`webp_i_dec_get_rgb`].
#[unsafe(export_name = "WebPIDecGetYUVA")]
pub unsafe extern "C" fn webp_i_dec_get_yuva(
    idec: *const WebPIDecoder,
    last_y: *mut c_int,
    u: *mut *mut u8,
    v: *mut *mut u8,
    a: *mut *mut u8,
    width: *mut c_int,
    height: *mut c_int,
    stride: *mut c_int,
    uv_stride: *mut c_int,
    a_stride: *mut c_int,
) -> *mut u8 {
    // SAFETY: see `webp_i_dec_get_rgb`.
    let Some(decoder) = (unsafe { idec.cast_mut().as_mut() }) else {
        return ptr::null_mut();
    };
    // SAFETY: the output memory is valid while the decoder lives.
    unsafe { decoder.attach() };
    let (mut u_rows, mut v_rows, mut a_rows) = (None, None, None);
    // SAFETY: null or writable output parameters (caller's contract).
    let last_y_out = unsafe { out(last_y) };
    // SAFETY: as above.
    let width = unsafe { out(width) };
    // SAFETY: as above.
    let height = unsafe { out(height) };
    // SAFETY: as above.
    let stride = unsafe { out(stride) };
    // SAFETY: as above.
    let uv_stride = unsafe { out(uv_stride) };
    // SAFETY: as above.
    let a_stride_out = unsafe { out(a_stride) };
    let y = idec::webp_i_dec_get_yuva(
        &decoder.idec,
        last_y_out,
        (!u.is_null()).then_some(&mut u_rows),
        (!v.is_null()).then_some(&mut v_rows),
        (!a.is_null()).then_some(&mut a_rows),
        width,
        height,
        stride,
        uv_stride,
        a_stride_out,
    );
    let pointers = y.map(|_| {
        (
            rows_pointer(decoder, u_rows),
            rows_pointer(decoder, v_rows),
            rows_pointer(decoder, a_rows),
        )
    });
    let y = rows_pointer(decoder, y);
    decoder.detach();
    if let Some((u_ptr, v_ptr, a_ptr)) = pointers {
        // SAFETY: null or writable output parameters.
        unsafe { set(u, u_ptr) };
        // SAFETY: as above.
        unsafe { set(v, v_ptr) };
        // SAFETY: as above.
        unsafe { set(a, a_ptr) };
    }
    y
}

// upstream: src/dec/idec_dec.c WebPIDecodedArea
/// Retrieves information about the displayable area: returns the output
/// buffer, or null in case the decoder is in an invalid state.
///
/// # Safety
///
/// As [`webp_i_dec_get_rgb`].
#[unsafe(export_name = "WebPIDecodedArea")]
pub unsafe extern "C" fn webp_i_decoded_area(
    idec: *const WebPIDecoder,
    left: *mut c_int,
    top: *mut c_int,
    width: *mut c_int,
    height: *mut c_int,
) -> *const WebPDecBuffer {
    // SAFETY: null or writable output parameters (caller's contract).
    let left = unsafe { out(left) };
    // SAFETY: as above.
    let top = unsafe { out(top) };
    // SAFETY: as above.
    let width = unsafe { out(width) };
    // SAFETY: as above.
    let height = unsafe { out(height) };
    // SAFETY: see `webp_i_dec_get_rgb`.
    let Some(decoder) = (unsafe { idec.cast_mut().as_mut() }) else {
        // C: GetOutputBuffer(NULL) is NULL, and the area is empty.
        for value in [left, top, width, height].into_iter().flatten() {
            *value = 0;
        }
        return ptr::null();
    };
    // SAFETY: the output memory is valid while the decoder lives.
    unsafe { decoder.attach() };
    let area = idec::webp_i_decoded_area(&decoder.idec, left, top, width, height).is_some();
    decoder.detach();
    if !area {
        ptr::null()
    } else if decoder.idec.has_user_output() && decoder.idec.was_slow() {
        // C: after the slow copy, `*output = *idec->final_output`: the
        // decoder's own buffer holds a copy of the caller's structure.
        // SAFETY: the caller's live buffer, read as plain data.
        decoder.own = unsafe { ptr::read(decoder.user) };
        &raw const decoder.own
    } else if decoder.idec.has_user_output() {
        decoder.user
    } else {
        &raw const decoder.own
    }
}
