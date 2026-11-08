// libwebp v1.6.0, src/webp/encode.h:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//   WebP encoder: main interface
//
// Author: Skal (pascal.massimino@gmail.com)

//! C ABI export of `src/webp/encode.h` from libwebp v1.6.0.
//!
//! Mirrors the header's structs with `repr(C)` and converts them to the
//! safe representations at the boundary; see
//! `docs/adr/0004-c-abi-boundary.md`.
//!
//! Work package: WP13 (encoder integration).
//!
//! `WebPConfig` and `WebPAuxStats` have no pointers, so the C ABI uses the
//! `repr(C)` structs of `src/webp/encode.rs`; `WebPPicture` and
//! `WebPMemoryWriter` have `repr(C)` mirrors here. `tests/wp13_abi_layout.rs`
//! compares every mirror and shared struct with the C compiler's layout.
//!
//! # Pictures
//!
//! A C `WebPPicture` is converted for each call (as WP12 converts a
//! `WebPDecBuffer`), and the call's results are written back into it: the
//! dimensions, `use_argb`, `colorspace`, the strides, the plane pointers,
//! `error_code`, `memory_` and `memory_argb_`. Its callback, `custom_ptr`,
//! `user_data`, `stats`, `extra_info` and padding fields stay as the caller set
//! them, except where C copies a whole picture (`WebPPictureCopy`,
//! `WebPPictureView`), which this module does with the C struct itself.
//!
//! - Memory that the library allocates for a picture (`memory_` for the Y, U, V
//!   and A planes, `memory_argb_` for the ARGB plane) is a block of the platform
//!   `malloc` of C's size (`WebPPictureAllocYUVA`, `WebPPictureAllocARGB`, with
//!   the ARGB plane at the `WEBP_ALIGN`ed address), which `WebPPictureFree`
//!   releases. This module records the address and size of each block, and
//!   assumes, as `encode.h` requires, that C releases it only through
//!   `WebPPictureFree` (or the functions that replace the planes): a block that C
//!   frees itself stays recorded, and caller memory that later lands at the same
//!   address in `memory_` would be read at the recorded size. During a call the
//!   translation works on a copy of the block (`PlaneStorage::Internal`), which
//!   goes back into the block after the call; new samples go into a new block.
//! - Caller memory (planes outside the picture's own block, as after
//!   `WebPPictureView` or when the caller sets the plane pointers) is lent to the
//!   translation for the duration of the call, only for the planes that C's path
//!   reads or writes: shared slices for a call that only reads them, exclusive
//!   slices for one that writes them. A plane's slice covers the samples that its
//!   dimensions and stride address, `(height - 1) * stride + width`. Calls that
//!   do not touch the planes (`WebPPictureAlloc`, `WebPPictureFree`, the
//!   importers, a conversion's output, the storage that `use_argb` does not
//!   select) lend nothing, and `WebPPictureView`, which only computes addresses,
//!   lends no sample.
//! - `WebPEncode` lends the planes of C's path, and none when the configuration
//!   or the picture fails C's validation. It gives the translation zeroed copies
//!   of the statistics and of the `extra_info` map and closures around the C
//!   writer and progress hook that pass the caller's picture pointer, as C passes
//!   it. The statistics go back to `*stats` once the validation has passed (C
//!   resets them there). The map goes back to `extra_info` when the lossy encoder
//!   ran, also after a failure, since C fills it one macroblock at a time (its
//!   entries are C's, and those C did not reach are zero where C leaves them
//!   untouched), and when a lossless encoding succeeds (C zeroes it then); the
//!   caller's bytes are never read.
//!
//! # Callbacks
//!
//! While `WebPEncode` runs, the caller's planes are lent exclusively to the
//! translation. A writer or progress hook must therefore not call a library
//! function on the picture being encoded (C allows, for example,
//! `WebPPictureHasTransparency(picture)` there; no upstream callback does it).
//! A picture in library memory is not affected, since the translation works on
//! a copy of its block.
//!
//! # Cases that this module rejects
//!
//! Rust cannot lend these planes, so the calls that read or write them fail: a
//! plane with a negative stride over more than one row (it lies before its
//! pointer), planes that a writing call would write and that overlap, and an
//! ARGB plane not aligned for `uint32_t`. Functions that report errors on the
//! picture record `VP8_ENC_ERROR_INVALID_CONFIGURATION` and return 0, the other
//! functions return 0 or do nothing. Packed input rows with a negative stride
//! (the importers and the simple API) are copied into rows with a positive
//! stride first, which hold the same bytes; rows with a positive stride are lent
//! as they are. `docs/status/wp13.md` records these cases.

use core::ffi::{c_int, c_void};
use core::mem::{MaybeUninit, offset_of, size_of};
use core::ptr;
use core::slice;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::abi::types::{malloc_copy, platform_free, platform_malloc};
use crate::enc::config_enc as config_rs;
use crate::enc::picture_csp_enc as csp;
use crate::enc::picture_enc as picture_rs;
use crate::enc::picture_psnr_enc as psnr;
use crate::enc::picture_rescale_enc as rescale;
use crate::enc::picture_tools_enc as tools;
use crate::enc::webp_enc as enc;
use crate::utils::utils::WEBP_ALIGN_CST;
use crate::webp::encode::{
    WEBP_ENCODER_ABI_VERSION, WEBP_MAX_DIMENSION, WebPAuxStats, WebPConfig, WebPEncCSP,
    WebPEncodingError, WebPMemoryWriter as RsMemoryWriter, WebPPicture as RsPicture, WebPPreset,
    WebPProgressHook as RsProgressHook, WebPWriterFunction as RsWriterFunction, webp_config_preset,
};
use crate::webp::types::{PlaneOffset, PlaneStorage, webp_abi_is_incompatible};

//------------------------------------------------------------------------------
// Mirrors of the structs with pointers

// upstream: src/webp/encode.h WebPWriterFunction
/// Signature of the output function (C layout): writes `data_size` bytes of
/// `data` and returns 0 on failure.
pub type WebPWriterFunction =
    unsafe extern "C" fn(data: *const u8, data_size: usize, picture: *const WebPPicture) -> c_int;

// upstream: src/webp/encode.h WebPProgressHook
/// Progress hook (C layout): returns 0 to request an abort.
pub type WebPProgressHook =
    unsafe extern "C" fn(percent: c_int, picture: *const WebPPicture) -> c_int;

// upstream: src/webp/encode.h WebPMemoryWriter
/// The memory writer of `WebPMemoryWrite` (C layout). `mem` is a block of the
/// platform `malloc` that `WebPMemoryWriterClear()` or `WebPFree()` releases.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPMemoryWriter {
    /// Final buffer (of size `max_size`, larger than `size`).
    pub mem: *mut u8,
    /// Final size.
    pub size: usize,
    /// Total capacity.
    pub max_size: usize,
    /// Padding for later use.
    pub pad: [u32; 1],
}

// upstream: src/webp/encode.h WebPPicture
/// Main exchange structure (C layout).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct WebPPicture {
    /// Main flag for encoder selecting between ARGB or YUV input.
    pub use_argb: c_int,
    /// Colorspace: should be `YUV420` for now (`Y'CbCr`).
    pub colorspace: WebPEncCSP,
    /// Width (less or equal to `WEBP_MAX_DIMENSION`).
    pub width: c_int,
    /// Height (less or equal to `WEBP_MAX_DIMENSION`).
    pub height: c_int,
    /// Luma plane.
    pub y: *mut u8,
    /// Chroma U plane.
    pub u: *mut u8,
    /// Chroma V plane.
    pub v: *mut u8,
    /// Luma stride.
    pub y_stride: c_int,
    /// Chroma stride.
    pub uv_stride: c_int,
    /// Alpha plane.
    pub a: *mut u8,
    /// Stride of the alpha plane.
    pub a_stride: c_int,
    /// Padding for later use.
    pub pad1: [u32; 2],
    /// ARGB (32 bit) plane.
    pub argb: *mut u32,
    /// Stride in pixels, not bytes.
    pub argb_stride: c_int,
    /// Padding for later use.
    pub pad2: [u32; 3],
    /// Byte-emission hook; can be `NULL`.
    pub writer: Option<WebPWriterFunction>,
    /// Can be used by the writer.
    pub custom_ptr: *mut c_void,
    /// 1: intra type, 2: segment, 3: quant, ...
    pub extra_info_type: c_int,
    /// If not `NULL`, an array of one byte per macroblock.
    pub extra_info: *mut u8,
    /// Pointer to side statistics (updated only if not `NULL`).
    pub stats: *mut WebPAuxStats,
    /// Error code for the latest error encountered during encoding.
    pub error_code: WebPEncodingError,
    /// If not `NULL`, report progress during encoding.
    pub progress_hook: Option<WebPProgressHook>,
    /// Free to be set to any value and used during callbacks.
    pub user_data: *mut c_void,
    /// Padding for later use.
    pub pad3: [u32; 3],
    /// Unused for now.
    pub pad4: *mut u8,
    /// Unused for now.
    pub pad5: *mut u8,
    /// Padding for later use.
    pub pad6: [u32; 8],
    /// Row chunk of memory for the YUVA planes.
    pub memory_: *mut c_void,
    /// And for the ARGB plane.
    pub memory_argb_: *mut c_void,
    /// Padding for later use.
    pub pad7: [*mut c_void; 2],
}

//------------------------------------------------------------------------------
// Picture memory blocks

/// The blocks that this module gave to C pictures as `memory_` or
/// `memory_argb_`, with their sizes in bytes.
static BLOCKS: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());

fn blocks() -> MutexGuard<'static, BTreeMap<usize, usize>> {
    BLOCKS.lock().unwrap_or_else(PoisonError::into_inner)
}

/// The size of a picture block, or `None` for memory this module did not
/// allocate.
fn block_size(block: *mut c_void) -> Option<usize> {
    blocks().get(&(block as usize)).copied()
}

/// A new zeroed block of `size` bytes of the platform `malloc`, recorded as a
/// picture block; null when `malloc` fails.
fn new_block(size: usize) -> *mut u8 {
    let block = platform_malloc(size.max(1));
    if !block.is_null() {
        // SAFETY: `block` is a new allocation of at least `size` bytes.
        unsafe { ptr::write_bytes(block, 0, size) };
        blocks().insert(block as usize, size);
    }
    block
}

/// Releases the memory that a C picture gives up (C: `WebPSafeFree`), which is
/// `free()` for a block of this module and for any other pointer.
///
/// # Safety
///
/// `block` is null or a live block of the platform `malloc` that is not used
/// again.
unsafe fn release_block(block: *mut c_void) {
    if block.is_null() {
        return;
    }
    blocks().remove(&(block as usize));
    // SAFETY: the caller gives up a block of the platform `malloc`.
    unsafe { platform_free(block.cast()) };
}

//------------------------------------------------------------------------------
// Picture conversion

/// How a call uses one storage of a picture (the YUVA planes or the ARGB plane).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    /// The call does not touch the planes: nothing is lent.
    Discard,
    /// The call only computes plane addresses (`WebPPictureView`): each plane is
    /// an empty region at [`ADDRESS_OFFSET`], so that the translation's offsets
    /// give C's pointer arithmetic for any stride, and no sample is lent.
    Address,
    /// The call reads the planes: caller memory is lent shared.
    Read,
    /// The call reads and writes the planes: caller memory is lent exclusively.
    Write,
}

/// The offset of a plane in its empty region in `Mode::Address`: far enough from
/// 0 and from `usize::MAX` that a view's offsets, which C computes as pointer
/// differences, stay in range for either sign.
const ADDRESS_OFFSET: usize = usize::MAX / 4 + 1;

/// Why a C picture cannot be converted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refused {
    /// A plane layout that Rust cannot lend (see the module documentation).
    Layout,
    /// The host refused the memory for a copy of the picture's block.
    Memory,
}

/// Where the translation's storage of a picture came from.
#[derive(Debug)]
enum Origin {
    /// Nothing was lent; the storage is an `External` storage without regions,
    /// which no translated function creates, so it marks an untouched storage.
    Nothing,
    /// The picture's own block (`memory_` or `memory_argb_`), copied for the call.
    Block { base: *mut u8, size: usize },
    /// Caller memory lent per plane: the start of each region.
    Lent(Vec<*mut u8>),
}

/// A C picture as the translation sees it during one call.
struct Converted<'a> {
    picture: RsPicture<'a>,
    yuva: Origin,
    argb: Origin,
}

/// The rows of a plane, in samples from its first sample: `(height - 1) *
/// stride + width`, or 0 for an empty plane. `None` for a negative stride over
/// more than one row, or a size that no memory can have.
fn plane_extent(width: i32, height: i32, stride: i32) -> Option<usize> {
    if width <= 0 || height <= 0 {
        return Some(0);
    }
    if height > 1 && stride < 0 {
        return None;
    }
    let rows = if height > 1 {
        i64::from(height - 1) * i64::from(stride)
    } else {
        0
    };
    usize::try_from(rows + i64::from(width)).ok()
}

/// C's `HALVE` on a dimension, `(x + 1) >> 1`, in 64 bits as `WebPPictureAllocYUVA`
/// computes it.
fn halve(x: i32) -> i32 {
    i32::try_from((i64::from(x) + 1) >> 1).unwrap_or(i32::MAX)
}

/// Whether two byte ranges overlap; an empty range overlaps nothing.
fn ranges_overlap(a: *const u8, a_len: usize, b: *const u8, b_len: usize) -> bool {
    let (a, b) = (a as usize, b as usize);
    a_len > 0 && b_len > 0 && a < b.saturating_add(b_len) && b < a.saturating_add(a_len)
}

/// A converted storage: the storage, the planes in it, the origin and the lent
/// byte ranges.
type StorageParts<'a, T> = (
    PlaneStorage<'a, T>,
    Vec<Option<PlaneOffset>>,
    Origin,
    Vec<(*const u8, usize)>,
);

/// The storage of `Mode::Address`: an empty region per non-null plane, with the
/// plane at [`ADDRESS_OFFSET`], and the bases that give C's plane addresses back.
fn address_storage<'a, T>(planes: &[(*mut u8, Option<usize>)]) -> StorageParts<'a, T> {
    let elem = size_of::<T>();
    let mut regions: Vec<&'a [T]> = Vec::new();
    let mut bases = Vec::new();
    let mut offsets = Vec::with_capacity(planes.len());
    for &(plane, _) in planes {
        if plane.is_null() {
            offsets.push(None);
            continue;
        }
        offsets.push(Some(PlaneOffset {
            region: regions.len(),
            offset: ADDRESS_OFFSET,
        }));
        regions.push(&[]);
        bases.push(plane.wrapping_sub(ADDRESS_OFFSET.wrapping_mul(elem)));
    }
    if regions.is_empty() {
        return (
            PlaneStorage::External(Vec::new()),
            offsets,
            Origin::Nothing,
            Vec::new(),
        );
    }
    (
        PlaneStorage::Shared(regions),
        offsets,
        Origin::Lent(bases),
        Vec::new(),
    )
}

/// The storage of a picture whose planes lie inside its own block (`memory_`
/// or `memory_argb_`): a copy of the block. `None` when `memory` is not a block
/// of this module or a plane lies outside it.
///
/// # Safety
///
/// `memory` is null or the picture's memory field.
unsafe fn block_storage<'a, T: Copy>(
    memory: *mut c_void,
    planes: &[(*mut u8, Option<usize>)],
) -> Option<Result<StorageParts<'a, T>, Refused>> {
    let elem = size_of::<T>();
    let base = memory.cast::<u8>();
    let size = block_size(memory)?;
    let inside = planes.iter().all(|&(plane, extent)| {
        plane.is_null()
            || extent.is_some_and(|extent| {
                let offset = (plane as usize).wrapping_sub(base as usize);
                plane as usize >= base as usize
                    && offset.is_multiple_of(elem)
                    && extent
                        .checked_mul(elem)
                        .and_then(|bytes| bytes.checked_add(offset))
                        .is_some_and(|end| end <= size)
            })
    });
    if !inside || size % elem != 0 {
        return None;
    }
    // SAFETY: the block is a live, initialized block of `size` bytes from the
    // platform `malloc` (this module zeroes it when it allocates it), aligned for
    // `T`; the copy ends before the translation runs.
    let block = unsafe { slice::from_raw_parts(base.cast::<T>(), size / elem) };
    let mut samples = Vec::new();
    if samples.try_reserve_exact(block.len()).is_err() {
        return Some(Err(Refused::Memory));
    }
    samples.extend_from_slice(block);
    let offsets = planes
        .iter()
        .map(|&(plane, _)| {
            (!plane.is_null()).then(|| PlaneOffset {
                region: 0,
                offset: (plane as usize - base as usize) / elem,
            })
        })
        .collect();
    Some(Ok((
        PlaneStorage::Internal(samples),
        offsets,
        Origin::Block { base, size },
        Vec::new(),
    )))
}

/// Converts one storage of a C picture. `planes` are the plane pointers with
/// their extents in samples (`None` when the plane cannot be lent). Fails for a
/// layout that this module rejects (see the module documentation) and when the
/// host refuses the memory for a copy of the picture's block.
///
/// # Safety
///
/// For `Read` and `Write`, every non-null plane points to its extent of
/// initialized samples, which only the translation accesses during the call
/// (exclusively for `Write`); `memory` is null or the picture's memory field.
unsafe fn storage_to_rust<'a, T: Copy>(
    mode: Mode,
    memory: *mut c_void,
    planes: &[(*mut u8, Option<usize>)],
) -> Result<StorageParts<'a, T>, Refused> {
    let elem = size_of::<T>();
    let nothing = || {
        Ok((
            PlaneStorage::External(Vec::new()),
            vec![None; planes.len()],
            Origin::Nothing,
            Vec::new(),
        ))
    };
    if mode == Mode::Discard {
        return nothing();
    }
    if mode == Mode::Address {
        return Ok(address_storage(planes));
    }
    // SAFETY: the caller's contract, passed on.
    if let Some(parts) = unsafe { block_storage(memory, planes) } {
        return parts;
    }
    if planes.iter().all(|&(plane, _)| plane.is_null()) {
        return nothing();
    }
    let mut offsets = Vec::with_capacity(planes.len());
    let mut bases = Vec::new();
    let mut ranges: Vec<(*const u8, usize)> = Vec::new();
    for &(plane, extent) in planes {
        if plane.is_null() {
            offsets.push(None);
            continue;
        }
        let extent = extent.ok_or(Refused::Layout)?;
        let bytes = extent.checked_mul(elem).ok_or(Refused::Layout)?;
        if bytes > isize::MAX as usize || !(plane as usize).is_multiple_of(align_of::<T>()) {
            return Err(Refused::Layout);
        }
        if mode == Mode::Write
            && ranges
                .iter()
                .any(|&(start, len)| ranges_overlap(start, len, plane, bytes))
        {
            return Err(Refused::Layout);
        }
        offsets.push(Some(PlaneOffset {
            region: bases.len(),
            offset: 0,
        }));
        bases.push(plane);
        ranges.push((plane, bytes));
    }
    let lengths: Vec<usize> = ranges.iter().map(|&(_, bytes)| bytes / elem).collect();
    let storage = if mode == Mode::Read {
        PlaneStorage::Shared(
            bases
                .iter()
                .zip(&lengths)
                .map(|(&start, &len)| {
                    // SAFETY: the caller lends `len` initialized samples at
                    // `start` for the call; shared slices may overlap.
                    unsafe { slice::from_raw_parts(start.cast::<T>().cast_const(), len) }
                })
                .collect(),
        )
    } else {
        PlaneStorage::External(
            bases
                .iter()
                .zip(&lengths)
                .map(|(&start, &len)| {
                    // SAFETY: the caller lends `len` writable samples at `start`
                    // exclusively for the call, and the regions are disjoint.
                    unsafe { slice::from_raw_parts_mut(start.cast::<T>(), len) }
                })
                .collect(),
        )
    };
    Ok((storage, offsets, Origin::Lent(bases), ranges))
}

/// Converts a C picture for one call, with the given use of its two storages.
/// The translation's picture holds no callbacks, `stats` or `extra_info`.
/// Fails as [`storage_to_rust`] does.
///
/// # Safety
///
/// The planes that `yuva` and `argb` lend meet the contract of
/// [`storage_to_rust`].
unsafe fn picture_to_rust<'a>(
    c: &WebPPicture,
    yuva: Mode,
    argb: Mode,
) -> Result<Converted<'a>, Refused> {
    let (width, height) = (c.width, c.height);
    let (uv_width, uv_height) = (halve(width), halve(height));
    // SAFETY: the caller's contract, passed on.
    let (yuva_storage, yuva_planes, yuva_origin, yuva_ranges) = unsafe {
        storage_to_rust::<u8>(
            yuva,
            c.memory_,
            &[
                (c.y, plane_extent(width, height, c.y_stride)),
                (c.u, plane_extent(uv_width, uv_height, c.uv_stride)),
                (c.v, plane_extent(uv_width, uv_height, c.uv_stride)),
                (c.a, plane_extent(width, height, c.a_stride)),
            ],
        )
    }?;
    // SAFETY: as above.
    let (argb_storage, argb_planes, argb_origin, argb_ranges) = unsafe {
        storage_to_rust::<u32>(
            argb,
            c.memory_argb_,
            &[(c.argb.cast(), plane_extent(width, height, c.argb_stride))],
        )
    }?;
    if (yuva == Mode::Write || argb == Mode::Write)
        && yuva_ranges.iter().any(|&(a, a_len)| {
            argb_ranges
                .iter()
                .any(|&(b, b_len)| ranges_overlap(a, a_len, b, b_len))
        })
    {
        return Err(Refused::Layout);
    }
    let picture = RsPicture {
        use_argb: c.use_argb,
        colorspace: c.colorspace,
        width,
        height,
        y: yuva_planes[0],
        u: yuva_planes[1],
        v: yuva_planes[2],
        y_stride: c.y_stride,
        uv_stride: c.uv_stride,
        a: yuva_planes[3],
        a_stride: c.a_stride,
        pad1: c.pad1,
        argb: argb_planes[0],
        argb_stride: c.argb_stride,
        pad2: c.pad2,
        writer: None,
        extra_info_type: c.extra_info_type,
        extra_info: None,
        stats: None,
        error_code: c.error_code,
        progress_hook: None,
        pad3: c.pad3,
        pad6: c.pad6,
        yuva: yuva_storage,
        argb_storage,
    };
    Ok(Converted {
        picture,
        yuva: yuva_origin,
        argb: argb_origin,
    })
}

/// The C pointers of planes in regions that start at `bases`; null for a
/// missing plane. The arithmetic wraps, as the bases of `Mode::Address` lie
/// outside the planes.
fn plane_pointers<T>(bases: &[*mut u8], planes: &[Option<PlaneOffset>]) -> Vec<*mut u8> {
    planes
        .iter()
        .map(|plane| {
            plane
                .and_then(|plane| {
                    bases
                        .get(plane.region)
                        .map(|&base| base.wrapping_add(plane.offset.wrapping_mul(size_of::<T>())))
                })
                .unwrap_or(ptr::null_mut())
        })
        .collect()
}

/// The memory of one storage after a call: `Ok(None)` when the C planes and
/// memory field stay as they are, `Ok(Some(pointers))` with the new plane
/// pointers otherwise, and `Err(())` when the block for new samples could not be
/// allocated (the old memory is then released and the planes are null). Updates
/// `memory` (C's `memory_` or `memory_argb_`).
///
/// # Safety
///
/// `origin` and `memory` belong to the storage's conversion, and `memory` is
/// null or a live block of the platform `malloc`.
unsafe fn storage_to_c<T: Copy>(
    storage: &PlaneStorage<'_, T>,
    planes: &[Option<PlaneOffset>],
    origin: &Origin,
    memory: &mut *mut c_void,
) -> Result<Option<Vec<*mut u8>>, ()> {
    let elem = size_of::<T>();
    match storage {
        // Untouched: nothing was lent.
        PlaneStorage::External(regions) if regions.is_empty() => Ok(None),
        // Untouched caller memory.
        PlaneStorage::External(_) | PlaneStorage::Shared(_) => match origin {
            Origin::Lent(bases) => Ok(Some(plane_pointers::<T>(bases, planes))),
            _ => Ok(None),
        },
        // The translation released the planes (C: WebPSafeFree of the memory).
        PlaneStorage::Internal(samples) if samples.is_empty() => {
            // SAFETY: the picture gives up its memory, as C frees it.
            unsafe { release_block(*memory) };
            *memory = ptr::null_mut();
            Ok(Some(vec![ptr::null_mut(); planes.len()]))
        }
        PlaneStorage::Internal(samples) => {
            let bytes = samples.len() * elem;
            let data = match origin {
                Origin::Block { base, size } if *size == bytes && base.cast() == *memory => *base,
                _ => {
                    // C's size: the samples, plus WEBP_ALIGN_CST samples for the
                    // alignment of an ARGB plane (WebPPictureAllocARGB).
                    let size = if elem == 1 {
                        bytes
                    } else {
                        (samples.len() + WEBP_ALIGN_CST) * elem
                    };
                    let block = new_block(size);
                    // SAFETY: the picture gives up its old memory.
                    unsafe { release_block(*memory) };
                    *memory = block.cast();
                    if block.is_null() {
                        return Err(());
                    }
                    if elem == 1 {
                        block
                    } else {
                        // C: WEBP_ALIGN(memory), ((uintptr_t)memory + 31) & ~31,
                        // which lies inside the block of WEBP_ALIGN_CST spare
                        // samples.
                        let address = block as usize;
                        let aligned = (address + WEBP_ALIGN_CST) & !WEBP_ALIGN_CST;
                        block.wrapping_add(aligned - address)
                    }
                }
            };
            // SAFETY: `data` starts `bytes` writable bytes of the picture's
            // block, aligned for `T`, which no Rust slice views.
            unsafe { ptr::copy_nonoverlapping(samples.as_ptr(), data.cast::<T>(), samples.len()) };
            Ok(Some(plane_pointers::<T>(&[data], planes)))
        }
    }
}

/// Writes the translation's picture back into the C picture: the dimensions,
/// `use_argb`, `colorspace`, the strides, `error_code`, the planes and the
/// memory fields. Returns false when a block for new samples could not be
/// allocated; the picture then records `VP8_ENC_ERROR_OUT_OF_MEMORY`.
///
/// # Safety
///
/// `c` is the picture that `converted` was converted from, and its memory
/// fields are null or live blocks of the platform `malloc`.
unsafe fn picture_to_c(c: &mut WebPPicture, converted: &Converted<'_>) -> bool {
    let rs = &converted.picture;
    c.use_argb = rs.use_argb;
    c.colorspace = rs.colorspace;
    c.width = rs.width;
    c.height = rs.height;
    c.y_stride = rs.y_stride;
    c.uv_stride = rs.uv_stride;
    c.a_stride = rs.a_stride;
    c.argb_stride = rs.argb_stride;
    c.error_code = rs.error_code;
    let mut ok = true;
    // SAFETY: the storage, origin and memory field of one conversion.
    match unsafe {
        storage_to_c(
            &rs.yuva,
            &[rs.y, rs.u, rs.v, rs.a],
            &converted.yuva,
            &mut c.memory_,
        )
    } {
        Ok(None) => {}
        Ok(Some(pointers)) => {
            c.y = pointers[0];
            c.u = pointers[1];
            c.v = pointers[2];
            c.a = pointers[3];
        }
        Err(()) => {
            (c.y, c.u, c.v, c.a) = (
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            ok = false;
        }
    }
    // SAFETY: as above.
    match unsafe {
        storage_to_c(
            &rs.argb_storage,
            &[rs.argb],
            &converted.argb,
            &mut c.memory_argb_,
        )
    } {
        Ok(None) => {}
        Ok(Some(pointers)) => c.argb = pointers[0].cast(),
        Err(()) => {
            c.argb = ptr::null_mut();
            ok = false;
        }
    }
    if !ok {
        set_error(c, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    ok
}

/// Records `error` on a C picture with the first-error rule of
/// `WebPEncodingSetError`.
fn set_error(c: &mut WebPPicture, error: WebPEncodingError) {
    if c.error_code == WebPEncodingError::VP8_ENC_OK {
        c.error_code = error;
    }
}

/// The mode for the storage that `use_argb` selects, and `Discard` for the other.
fn selected(c: &WebPPicture, mode: Mode) -> (Mode, Mode) {
    if c.use_argb != 0 {
        (Mode::Discard, mode)
    } else {
        (mode, Mode::Discard)
    }
}

/// Runs `call` on the converted C picture and writes the result back. Returns
/// `None` when the picture cannot be converted; when `record` is set, the
/// picture then records `VP8_ENC_ERROR_INVALID_CONFIGURATION` for a rejected
/// layout and `VP8_ENC_ERROR_OUT_OF_MEMORY` when the host refused the copy of
/// its block. The second value is false when the result could not be written
/// back.
///
/// # Safety
///
/// `picture` points to a valid picture whose planes meet the contract of
/// [`storage_to_rust`] for the given modes.
unsafe fn with_picture<'a, R>(
    picture: *mut WebPPicture,
    modes: (Mode, Mode),
    record: bool,
    call: impl FnOnce(&mut RsPicture<'a>) -> R,
) -> Option<(R, bool)> {
    // SAFETY: a valid picture (caller's contract); the borrow ends before the
    // call.
    let c = unsafe { &mut *picture };
    // SAFETY: the planes meet the contract for the modes (caller's contract).
    let mut converted = match unsafe { picture_to_rust(c, modes.0, modes.1) } {
        Ok(converted) => converted,
        Err(refused) => {
            if record {
                set_error(
                    c,
                    if refused == Refused::Memory {
                        WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY
                    } else {
                        WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION
                    },
                );
            }
            return None;
        }
    };
    let result = call(&mut converted.picture);
    // SAFETY: as above; no Rust borrow of the C picture is live.
    let c = unsafe { &mut *picture };
    // SAFETY: `c` is the picture that was converted.
    let written = unsafe { picture_to_c(c, &converted) };
    Some((result, written))
}

/// The C picture with its buffers reset (C: `WebPPictureResetBuffers` on a copy).
fn without_buffers(mut c: WebPPicture) -> WebPPicture {
    c.memory_argb_ = ptr::null_mut();
    c.argb = ptr::null_mut();
    c.argb_stride = 0;
    c.memory_ = ptr::null_mut();
    (c.y, c.u, c.v, c.a) = (
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    );
    c.y_stride = 0;
    c.uv_stride = 0;
    c.a_stride = 0;
    c
}

//------------------------------------------------------------------------------
// Packed input rows

/// The packed rows of `height` rows of `width` pixels of `step` bytes at
/// `input` with `stride` bytes between rows, as `Import` (`src/enc/picture_csp_enc.c`)
/// reads them, and the stride to read them with. Rows with a positive stride
/// are lent as they are; rows with a negative stride are copied into rows with
/// a positive stride, which hold the same bytes. When `Import` fails before it
/// reads (an empty picture, a stride shorter than its minimum), the rows are
/// empty. `None` when the host refuses the memory for the copy.
///
/// # Safety
///
/// `input` is not null and points to the rows of the caller's contract, which
/// nothing writes during the call.
unsafe fn packed_rows<'a>(
    input: *const u8,
    width: i32,
    height: i32,
    stride: i32,
    step: i32,
    import_alpha: bool,
) -> Option<(Cow<'a, [u8]>, i32)> {
    let pixel = if import_alpha { 4 } else { 3 };
    let min_row = i64::from(pixel) * i64::from(width);
    if width <= 0 {
        // Import's stride test passes for every stride, and C reads no
        // sample; the stride 0 keeps every row position inside the empty rows.
        return Some((Cow::Borrowed(&[]), 0));
    }
    if height <= 0 || i64::from(stride).abs() < min_row {
        return Some((Cow::Borrowed(&[]), stride));
    }
    // The bytes of a row that Import reads: up to the last pixel's last channel.
    let last = i64::from(width - 1) * i64::from(step) + i64::from(pixel);
    let last = usize::try_from(last).unwrap_or(0);
    if stride >= 0 || height == 1 {
        let rows = if height > 1 {
            usize::try_from(i64::from(height - 1) * i64::from(stride)).unwrap_or(0)
        } else {
            0
        };
        // SAFETY: the caller's rows span `rows + last` initialized bytes from
        // `input`, which nothing writes during the call.
        let bytes = unsafe { slice::from_raw_parts(input, rows + last) };
        return Some((Cow::Borrowed(bytes), stride));
    }
    let up = stride.unsigned_abs() as usize;
    let copy_stride = up.max(last);
    let rows = usize::try_from(height).unwrap_or(0);
    let mut copy = Vec::new();
    copy.try_reserve_exact((rows - 1) * copy_stride + last)
        .ok()?;
    copy.resize((rows - 1) * copy_stride + last, 0_u8);
    for row in 0..rows {
        let start = input.wrapping_sub(row * up);
        // SAFETY: row `row` starts `row * |stride|` bytes before `input` and
        // holds `last` bytes inside the caller's rows.
        let bytes = unsafe { slice::from_raw_parts(start, last) };
        copy[row * copy_stride..row * copy_stride + last].copy_from_slice(bytes);
    }
    Some((
        Cow::Owned(copy),
        i32::try_from(copy_stride).unwrap_or(i32::MAX),
    ))
}

//------------------------------------------------------------------------------
// Version and simple API

// upstream: src/enc/webp_enc.c WebPGetEncoderVersion
/// Returns the encoder's version number, packed in hexadecimal using 8 bits for
/// each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[unsafe(export_name = "WebPGetEncoderVersion")]
#[must_use]
pub extern "C" fn webp_get_encoder_version() -> c_int {
    enc::webp_get_encoder_version()
}

/// The translation of one lossless simple encoding function.
type SimpleEncode = fn(&[u8], i32, i32, i32) -> Option<Vec<u8>>;

/// The translation of one lossy simple encoding function.
type SimpleEncodeLossy = fn(&[u8], i32, i32, i32, f32) -> Option<Vec<u8>>;

/// The common part of the simple API (C: `Encode`): the `NULL` checks, the
/// configuration check that fails before `*output` is written, the packed rows,
/// and the output block that C releases with `WebPFree()`.
///
/// # Safety
///
/// `input` is null or points to the rows of `encode.h`'s contract; `output` is
/// null or writable.
#[expect(
    clippy::too_many_arguments,
    reason = "the C arguments plus the importer's step, alpha flag and quality"
)]
unsafe fn simple(
    input: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    step: i32,
    import_alpha: bool,
    quality_factor: f32,
    output: *mut *mut u8,
    encode: impl FnOnce(&[u8], i32) -> Option<Vec<u8>>,
) -> usize {
    if output.is_null() {
        return 0;
    }
    // C: `WebPConfigPreset` fails before `*output` is written.
    let mut config = WebPConfig::default();
    if !webp_config_preset(&mut config, WebPPreset::WEBP_PRESET_DEFAULT, quality_factor) {
        return 0;
    }
    // SAFETY: `output` is writable (caller's contract); C writes NULL on failure.
    unsafe { output.write(ptr::null_mut()) };
    if input.is_null() {
        return 0; // C: the importer fails.
    }
    // SAFETY: the caller's rows.
    let Some((rows, stride)) =
        (unsafe { packed_rows(input, width, height, stride, step, import_alpha) })
    else {
        return 0; // The host refused the copy of rows with a negative stride.
    };
    let Some(bytes) = encode(&rows, stride) else {
        return 0;
    };
    let block = malloc_copy(&bytes);
    if block.is_null() {
        return 0;
    }
    // SAFETY: as above.
    unsafe { output.write(block) };
    bytes.len()
}

/// The lossy simple API through `simple`.
///
/// # Safety
///
/// As [`simple`].
#[expect(
    clippy::too_many_arguments,
    reason = "the C arguments plus the importer's"
)]
unsafe fn simple_lossy(
    input: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    quality_factor: f32,
    output: *mut *mut u8,
    step: i32,
    import_alpha: bool,
    encode: SimpleEncodeLossy,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple(
            input,
            width,
            height,
            stride,
            step,
            import_alpha,
            quality_factor,
            output,
            |rows, stride| encode(rows, width, height, stride, quality_factor),
        )
    }
}

/// The lossless simple API through `simple`, at C's quality 70.
///
/// # Safety
///
/// As [`simple`].
#[expect(
    clippy::too_many_arguments,
    reason = "the C arguments plus the importer's"
)]
unsafe fn simple_lossless(
    input: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    output: *mut *mut u8,
    step: i32,
    import_alpha: bool,
    encode: SimpleEncode,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple(
            input,
            width,
            height,
            stride,
            step,
            import_alpha,
            70.,
            output,
            |rows, stride| encode(rows, width, height, stride),
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeRGB
/// Encodes RGB rows into lossy WebP; release the output with `WebPFree()`.
/// Returns the size of the output, 0 on error.
///
/// # Safety
///
/// `rgb` is null or points to `height` rows of `stride` bytes; `output` is null
/// or writable.
#[unsafe(export_name = "WebPEncodeRGB")]
pub unsafe extern "C" fn webp_encode_rgb(
    rgb: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossy(
            rgb,
            width,
            height,
            stride,
            quality_factor,
            output,
            3,
            false,
            picture_rs::webp_encode_rgb,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeBGR
/// Encodes BGR rows into lossy WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeBGR")]
pub unsafe extern "C" fn webp_encode_bgr(
    bgr: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossy(
            bgr,
            width,
            height,
            stride,
            quality_factor,
            output,
            3,
            false,
            picture_rs::webp_encode_bgr,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeRGBA
/// Encodes RGBA rows into lossy WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeRGBA")]
pub unsafe extern "C" fn webp_encode_rgba(
    rgba: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossy(
            rgba,
            width,
            height,
            stride,
            quality_factor,
            output,
            4,
            true,
            picture_rs::webp_encode_rgba,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeBGRA
/// Encodes BGRA rows into lossy WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeBGRA")]
pub unsafe extern "C" fn webp_encode_bgra(
    bgra: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    quality_factor: f32,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossy(
            bgra,
            width,
            height,
            stride,
            quality_factor,
            output,
            4,
            true,
            picture_rs::webp_encode_bgra,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessRGB
/// Encodes RGB rows into lossless WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeLosslessRGB")]
pub unsafe extern "C" fn webp_encode_lossless_rgb(
    rgb: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossless(
            rgb,
            width,
            height,
            stride,
            output,
            3,
            false,
            picture_rs::webp_encode_lossless_rgb,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessBGR
/// Encodes BGR rows into lossless WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeLosslessBGR")]
pub unsafe extern "C" fn webp_encode_lossless_bgr(
    bgr: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossless(
            bgr,
            width,
            height,
            stride,
            output,
            3,
            false,
            picture_rs::webp_encode_lossless_bgr,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessRGBA
/// Encodes RGBA rows into lossless WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeLosslessRGBA")]
pub unsafe extern "C" fn webp_encode_lossless_rgba(
    rgba: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossless(
            rgba,
            width,
            height,
            stride,
            output,
            4,
            true,
            picture_rs::webp_encode_lossless_rgba,
        )
    }
}

// upstream: src/enc/picture_enc.c WebPEncodeLosslessBGRA
/// Encodes BGRA rows into lossless WebP; see [`webp_encode_rgb`].
///
/// # Safety
///
/// As [`webp_encode_rgb`].
#[unsafe(export_name = "WebPEncodeLosslessBGRA")]
pub unsafe extern "C" fn webp_encode_lossless_bgra(
    bgra: *const u8,
    width: c_int,
    height: c_int,
    stride: c_int,
    output: *mut *mut u8,
) -> usize {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        simple_lossless(
            bgra,
            width,
            height,
            stride,
            output,
            4,
            true,
            picture_rs::webp_encode_lossless_bgra,
        )
    }
}

//------------------------------------------------------------------------------
// Configuration

// upstream: src/enc/config_enc.c WebPConfigInitInternal
/// Internal, version-checked entry point of `WebPConfigInit()` and
/// `WebPConfigPreset()`. A version mismatch and a `NULL` config return 0
/// without writing; otherwise every field but the reserved `use_delta_palette`
/// is written, and the result is that of `WebPValidateConfig()`.
///
/// # Safety
///
/// `config` is null or points to writable memory for a `WebPConfig`, which
/// may be uninitialized.
#[unsafe(export_name = "WebPConfigInitInternal")]
pub unsafe extern "C" fn webp_config_init_internal(
    config: *mut WebPConfig,
    preset: WebPPreset,
    quality: f32,
    version: c_int,
) -> c_int {
    if webp_abi_is_incompatible(version, WEBP_ENCODER_ABI_VERSION) {
        return 0; // caller/system version mismatch!
    }
    if config.is_null() {
        return 0;
    }
    let mut rs = WebPConfig::default();
    let ok = config_rs::webp_config_init_internal(&mut rs, preset, quality, version);
    // C does not write the reserved field, which may be uninitialized.
    let reserved = config
        .wrapping_byte_add(offset_of!(WebPConfig, use_delta_palette))
        .cast::<MaybeUninit<i32>>();
    // SAFETY: `reserved` is a field of the caller's `WebPConfig`, read as
    // possibly uninitialized bytes.
    let keep = unsafe { reserved.read() };
    // SAFETY: `config` is writable for a `WebPConfig`.
    unsafe { config.write(rs) };
    // SAFETY: as above, for the field.
    unsafe { reserved.write(keep) };
    c_int::from(ok)
}

// upstream: src/enc/config_enc.c WebPConfigLosslessPreset
/// Activates the lossless compression mode with the desired efficiency level
/// between 0 (fastest) and 9 (slowest); returns 0 for a `NULL` config or a
/// level outside that range.
///
/// # Safety
///
/// `config` is null or points to a valid `WebPConfig`.
#[unsafe(export_name = "WebPConfigLosslessPreset")]
pub unsafe extern "C" fn webp_config_lossless_preset(
    config: *mut WebPConfig,
    level: c_int,
) -> c_int {
    // SAFETY: null or a valid config (caller's contract).
    let Some(config) = (unsafe { config.as_mut() }) else {
        return 0;
    };
    c_int::from(config_rs::webp_config_lossless_preset(config, level))
}

// upstream: src/enc/config_enc.c WebPValidateConfig
/// Returns true if `config` is non-`NULL` and all configuration parameters are
/// within their valid ranges.
///
/// # Safety
///
/// `config` is null or points to a valid `WebPConfig`.
#[unsafe(export_name = "WebPValidateConfig")]
#[must_use]
pub unsafe extern "C" fn webp_validate_config(config: *const WebPConfig) -> c_int {
    // SAFETY: null or a valid config (caller's contract).
    let Some(config) = (unsafe { config.as_ref() }) else {
        return 0;
    };
    c_int::from(config_rs::webp_validate_config(config))
}

//------------------------------------------------------------------------------
// Memory writer

// upstream: src/enc/picture_enc.c WebPMemoryWriterInit
/// Initializes `writer` without releasing its previous buffer, as C does.
///
/// # Safety
///
/// `writer` is null or points to writable memory for a `WebPMemoryWriter`,
/// which may be uninitialized.
#[unsafe(export_name = "WebPMemoryWriterInit")]
pub unsafe extern "C" fn webp_memory_writer_init(writer: *mut WebPMemoryWriter) {
    if writer.is_null() {
        return; // C dereferences it.
    }
    let mut rs = RsMemoryWriter::default();
    picture_rs::webp_memory_writer_init(&mut rs);
    let field = |offset: usize| writer.wrapping_byte_add(offset);
    // SAFETY: a field of the caller's writable `WebPMemoryWriter`; `pad` is not
    // written, as in C.
    unsafe {
        field(offset_of!(WebPMemoryWriter, mem))
            .cast::<*mut u8>()
            .write(ptr::null_mut());
    }
    // SAFETY: as above.
    unsafe {
        field(offset_of!(WebPMemoryWriter, size))
            .cast::<usize>()
            .write(rs.size);
    }
    // SAFETY: as above.
    unsafe {
        field(offset_of!(WebPMemoryWriter, max_size))
            .cast::<usize>()
            .write(rs.max_size);
    }
}

// upstream: src/enc/picture_enc.c WebPMemoryWriterClear
/// Releases the writer's buffer and empties it; a `NULL` writer is ignored.
///
/// # Safety
///
/// `writer` is null or points to a valid `WebPMemoryWriter` whose `mem` is null
/// or a block of the platform `malloc`.
#[unsafe(export_name = "WebPMemoryWriterClear")]
pub unsafe extern "C" fn webp_memory_writer_clear(writer: *mut WebPMemoryWriter) {
    // SAFETY: null or a valid writer (caller's contract).
    let Some(writer) = (unsafe { writer.as_mut() }) else {
        return;
    };
    // The translation releases the buffer, which the C block is here.
    let mut rs = RsMemoryWriter {
        mem: Vec::new(),
        size: writer.size,
        max_size: writer.max_size,
        pad: writer.pad,
    };
    picture_rs::webp_memory_writer_clear(&mut rs);
    // SAFETY: the writer gives up its block (C: WebPSafeFree(writer->mem)).
    unsafe { platform_free(writer.mem) };
    writer.mem = ptr::null_mut();
    writer.size = rs.size;
    writer.max_size = rs.max_size;
}

// upstream: src/enc/picture_enc.c WebPMemoryWrite
/// The writer of `WebPMemoryWriter` as `picture->custom_ptr`: appends the data
/// to the buffer, which grows by doubling, with a minimum of 8192 bytes. Returns
/// 0 when the buffer cannot grow.
///
/// The translation works on a copy of the written bytes; the grown buffer is a
/// new block of the platform `malloc` of the translation's `max_size`.
///
/// # Safety
///
/// `picture` points to a valid picture whose `custom_ptr` is null or a valid
/// `WebPMemoryWriter`; `data` points to `data_size` readable bytes when
/// `data_size` is not 0.
#[unsafe(export_name = "WebPMemoryWrite")]
#[must_use]
pub unsafe extern "C" fn webp_memory_write(
    data: *const u8,
    data_size: usize,
    picture: *const WebPPicture,
) -> c_int {
    // SAFETY: a valid picture (caller's contract); C dereferences it.
    let Some(picture) = (unsafe { picture.as_ref() }) else {
        return 0;
    };
    let w = picture.custom_ptr.cast::<WebPMemoryWriter>();
    // SAFETY: null or a valid writer (caller's contract).
    let Some(w) = (unsafe { w.as_mut() }) else {
        return 1;
    };
    let data = if data_size == 0 || data.is_null() {
        &[][..]
    } else {
        // SAFETY: `data_size` readable bytes (caller's contract).
        unsafe { slice::from_raw_parts(data, data_size) }
    };
    let mut mem = Vec::new();
    if w.max_size > 0 {
        // A refused reservation fails as C's WebPSafeMalloc does.
        if mem.try_reserve_exact(w.max_size).is_err() {
            return 0;
        }
        mem.resize(w.max_size, 0_u8);
        if w.size > 0 {
            // SAFETY: `mem` holds the `size` written bytes of `max_size`.
            mem[..w.size].copy_from_slice(unsafe { slice::from_raw_parts(w.mem, w.size) });
        }
    }
    let old_size = w.size;
    let mut rs = RsMemoryWriter {
        mem,
        size: w.size,
        max_size: w.max_size,
        pad: w.pad,
    };
    if !picture_rs::webp_memory_write(&mut rs, data) {
        return 0;
    }
    if rs.max_size != w.max_size {
        let block = platform_malloc(rs.max_size);
        if block.is_null() {
            return 0;
        }
        // SAFETY: `block` holds `max_size` bytes, a new allocation.
        unsafe { ptr::copy_nonoverlapping(rs.mem.as_ptr(), block, rs.size) };
        // SAFETY: the writer gives up its old block (C: WebPSafeFree(w->mem)).
        unsafe { platform_free(w.mem) };
        w.mem = block;
        w.max_size = rs.max_size;
    } else if rs.size > old_size {
        let end = w.mem.wrapping_add(old_size);
        // SAFETY: the writer's block holds `max_size` bytes, of which the new
        // ones start at `old_size`.
        unsafe {
            ptr::copy_nonoverlapping(rs.mem[old_size..].as_ptr(), end, rs.size - old_size);
        }
    }
    w.size = rs.size;
    1
}

//------------------------------------------------------------------------------
// Picture initialization, allocation and release

/// The writer that `WebPPictureInit()` installs (C: `DummyWriter`), which
/// discards the data and reports success.
unsafe extern "C" fn dummy_writer(
    _data: *const u8,
    _data_size: usize,
    _picture: *const WebPPicture,
) -> c_int {
    1
}

// upstream: src/enc/picture_enc.c WebPPictureInitInternal
/// Internal, version-checked entry point of `WebPPictureInit()`: clears the
/// picture, installs the discarding writer and records no error. Returns 0 on a
/// version mismatch, without writing, and 1 otherwise, also for a `NULL`
/// picture.
///
/// # Safety
///
/// `picture` is null or points to writable memory for a `WebPPicture`, which
/// may be uninitialized.
#[unsafe(export_name = "WebPPictureInitInternal")]
pub unsafe extern "C" fn webp_picture_init_internal(
    picture: *mut WebPPicture,
    version: c_int,
) -> c_int {
    let mut rs = RsPicture::default();
    if !picture_rs::webp_picture_init_internal(&mut rs, version) {
        return 0; // caller/system version mismatch!
    }
    if picture.is_null() {
        return 1;
    }
    // The translation's cleared picture: zero fields, the discarding writer and
    // no error.
    let c = WebPPicture {
        use_argb: rs.use_argb,
        colorspace: rs.colorspace,
        width: rs.width,
        height: rs.height,
        y: ptr::null_mut(),
        u: ptr::null_mut(),
        v: ptr::null_mut(),
        y_stride: rs.y_stride,
        uv_stride: rs.uv_stride,
        a: ptr::null_mut(),
        a_stride: rs.a_stride,
        pad1: rs.pad1,
        argb: ptr::null_mut(),
        argb_stride: rs.argb_stride,
        pad2: rs.pad2,
        writer: rs
            .writer
            .is_some()
            .then_some(dummy_writer as WebPWriterFunction),
        custom_ptr: ptr::null_mut(),
        extra_info_type: rs.extra_info_type,
        extra_info: ptr::null_mut(),
        stats: ptr::null_mut(),
        error_code: rs.error_code,
        progress_hook: None,
        user_data: ptr::null_mut(),
        pad3: rs.pad3,
        pad4: ptr::null_mut(),
        pad5: ptr::null_mut(),
        pad6: rs.pad6,
        memory_: ptr::null_mut(),
        memory_argb_: ptr::null_mut(),
        pad7: [ptr::null_mut(); 2],
    };
    // SAFETY: `picture` is writable for a `WebPPicture` (caller's contract).
    unsafe { picture.write(c) };
    1
}

// upstream: src/enc/picture_enc.c WebPPictureAlloc
/// Releases the picture's planes and allocates new ones of its dimensions, in
/// the layout that `use_argb` selects. Returns 0 on error, recorded in
/// `error_code`; a `NULL` picture returns 1, as in C.
///
/// # Safety
///
/// `picture` is null or points to a valid picture whose `memory_` and
/// `memory_argb_` are null or blocks of the platform `malloc`.
#[unsafe(export_name = "WebPPictureAlloc")]
pub unsafe extern "C" fn webp_picture_alloc(picture: *mut WebPPicture) -> c_int {
    if picture.is_null() {
        return 1;
    }
    // SAFETY: a valid picture; nothing is lent.
    let result = unsafe {
        with_picture(picture, (Mode::Discard, Mode::Discard), false, |rs| {
            picture_rs::webp_picture_alloc(rs).is_ok()
        })
    };
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}

// upstream: src/enc/picture_enc.c WebPPictureFree
/// Releases the memory that `WebPPictureAlloc()` or `WebPPictureImport*()`
/// allocated and resets the plane pointers; a `NULL` picture is ignored.
///
/// # Safety
///
/// As [`webp_picture_alloc`].
#[unsafe(export_name = "WebPPictureFree")]
pub unsafe extern "C" fn webp_picture_free(picture: *mut WebPPicture) {
    if picture.is_null() {
        return;
    }
    // SAFETY: a valid picture; nothing is lent.
    let _ = unsafe {
        with_picture(picture, (Mode::Discard, Mode::Discard), false, |rs| {
            picture_rs::webp_picture_free(rs);
        })
    };
}

//------------------------------------------------------------------------------
// Copy, views, cropping and rescaling

// upstream: src/enc/picture_rescale_enc.c WebPPictureCopy
/// Copies `src` into `dst` with new planes that `dst` owns. As C does, `dst`
/// receives every field of `src` first, and its previous memory is not
/// released. Returns 0 on error and for `NULL` pictures; `src == dst` returns 1.
///
/// # Safety
///
/// `src` and `dst` are null or valid pictures; the planes of `src` that
/// `use_argb` selects are readable for its dimensions.
#[unsafe(export_name = "WebPPictureCopy")]
pub unsafe extern "C" fn webp_picture_copy(
    src: *const WebPPicture,
    dst: *mut WebPPicture,
) -> c_int {
    if src.is_null() || dst.is_null() {
        return 0;
    }
    if src == dst.cast_const() {
        return 1;
    }
    // SAFETY: a valid picture (caller's contract).
    let c_src = unsafe { *src };
    let (yuva, argb) = selected(&c_src, Mode::Read);
    // SAFETY: the selected planes are readable (caller's contract).
    let Ok(source) = (unsafe { picture_to_rust(&c_src, yuva, argb) }) else {
        return 0;
    };
    let mut copy = RsPicture::default();
    let ok = rescale::webp_picture_copy(&source.picture, &mut copy).is_ok();
    // C: PictureGrabSpecs(src, dst), then WebPPictureAlloc(dst) and the copy.
    let mut c_dst = without_buffers(c_src);
    let converted = Converted {
        picture: copy,
        yuva: Origin::Nothing,
        argb: Origin::Nothing,
    };
    // SAFETY: `c_dst` holds no memory; new samples go into new blocks.
    let written = unsafe { picture_to_c(&mut c_dst, &converted) };
    // SAFETY: `dst` is a valid picture (caller's contract), distinct from `src`.
    unsafe { dst.write(c_dst) };
    c_int::from(ok && written)
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureIsView
/// Returns true if the picture does not own its memory (C: its `memory_` or
/// `memory_argb_`, as `use_argb` selects, is `NULL`); 0 for a `NULL` picture.
///
/// The translation reads only `use_argb` and whether the selected storage holds
/// memory, which this function passes as a one-sample owned storage for C's
/// non-null memory field.
///
/// # Safety
///
/// `picture` is null or a valid picture.
#[unsafe(export_name = "WebPPictureIsView")]
#[must_use]
pub unsafe extern "C" fn webp_picture_is_view(picture: *const WebPPicture) -> c_int {
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return 0;
    };
    let mut rs = RsPicture {
        use_argb: c.use_argb,
        ..RsPicture::default()
    };
    if !c.memory_.is_null() {
        rs.yuva = PlaneStorage::Internal(vec![0]);
    }
    if !c.memory_argb_.is_null() {
        rs.argb_storage = PlaneStorage::Internal(vec![0]);
    }
    c_int::from(rescale::webp_picture_is_view(&rs))
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureView
/// Makes `dst` a view of the rectangle of `src`: its planes point into those of
/// `src`, and it owns no memory. As C does, `dst` receives every field of `src`
/// first unless `src == dst`. Returns 0 for an invalid rectangle and for `NULL`
/// pictures.
///
/// # Safety
///
/// `src` and `dst` are null or valid pictures; the planes of `src` that
/// `use_argb` selects are addressable for its dimensions.
#[unsafe(export_name = "WebPPictureView")]
pub unsafe extern "C" fn webp_picture_view(
    src: *const WebPPicture,
    left: c_int,
    top: c_int,
    width: c_int,
    height: c_int,
    dst: *mut WebPPicture,
) -> c_int {
    if src.is_null() || dst.is_null() {
        return 0;
    }
    // SAFETY: a valid picture (caller's contract).
    let c_src = unsafe { *src };
    // C only computes the view's plane addresses: no sample is lent.
    let (yuva, argb) = selected(&c_src, Mode::Address);
    // SAFETY: `Mode::Address` lends no memory.
    let Ok(source) = (unsafe { picture_to_rust(&c_src, yuva, argb) }) else {
        return 0;
    };
    let Some(view) = rescale::webp_picture_view(&source.picture, left, top, width, height) else {
        return 0;
    };
    let bases = |origin: &Origin| match origin {
        Origin::Block { base, .. } => vec![*base],
        Origin::Lent(bases) => bases.clone(),
        Origin::Nothing => Vec::new(),
    };
    let mut c_dst = if src == dst.cast_const() {
        c_src
    } else {
        // C: PictureGrabSpecs(src, dst).
        without_buffers(c_src)
    };
    c_dst.width = view.width;
    c_dst.height = view.height;
    if c_src.use_argb == 0 {
        let pointers =
            plane_pointers::<u8>(&bases(&source.yuva), &[view.y, view.u, view.v, view.a]);
        c_dst.y = pointers[0];
        c_dst.u = pointers[1];
        c_dst.v = pointers[2];
        c_dst.y_stride = view.y_stride;
        c_dst.uv_stride = view.uv_stride;
        if !c_src.a.is_null() {
            c_dst.a = pointers[3];
            c_dst.a_stride = view.a_stride;
        }
    } else {
        c_dst.argb = plane_pointers::<u32>(&bases(&source.argb), &[view.argb])[0].cast();
        c_dst.argb_stride = view.argb_stride;
    }
    drop(view);
    drop(source);
    // SAFETY: `dst` is a valid picture (caller's contract); no Rust borrow of it
    // or of `src` is live.
    unsafe { dst.write(c_dst) };
    1
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureCrop
/// Crops the picture to the rectangle, replacing its planes with new ones.
/// Returns 0 for an invalid rectangle, on error (recorded in `error_code`) and
/// for a `NULL` picture.
///
/// # Safety
///
/// `picture` is null or a valid picture whose planes that `use_argb` selects
/// are readable for its dimensions, and whose memory fields are null or blocks
/// of the platform `malloc`.
#[unsafe(export_name = "WebPPictureCrop")]
pub unsafe extern "C" fn webp_picture_crop(
    picture: *mut WebPPicture,
    left: c_int,
    top: c_int,
    width: c_int,
    height: c_int,
) -> c_int {
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return 0;
    };
    let modes = selected(c, Mode::Read);
    // SAFETY: the selected planes are readable (caller's contract).
    let result = unsafe {
        with_picture(picture, modes, true, |rs| {
            rescale::webp_picture_crop(rs, left, top, width, height)
        })
    };
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}

// upstream: src/enc/picture_rescale_enc.c WebPPictureRescale
/// Rescales the picture to `width` by `height`, replacing its planes with new
/// ones; a 0 dimension keeps the aspect ratio. Returns 0 on error (recorded in
/// `error_code`) and for a `NULL` picture.
///
/// # Safety
///
/// `picture` is null or a valid picture whose planes that `use_argb` selects
/// are writable for its dimensions, and whose memory fields are null or blocks
/// of the platform `malloc`.
#[unsafe(export_name = "WebPPictureRescale")]
pub unsafe extern "C" fn webp_picture_rescale(
    picture: *mut WebPPicture,
    width: c_int,
    height: c_int,
) -> c_int {
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return 0;
    };
    let modes = selected(c, Mode::Write);
    // SAFETY: the selected planes are writable (caller's contract).
    let result = unsafe {
        with_picture(picture, modes, true, |rs| {
            rescale::webp_picture_rescale(rs, width, height).is_ok()
        })
    };
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}

//------------------------------------------------------------------------------
// Distortion

// upstream: src/enc/picture_psnr_enc.c WebPPlaneDistortion
/// Computes the distortion of `ref` against `src` over a plane of `width` by
/// `height` samples, `x_step` bytes apart, with the metric `type` (0: PSNR,
/// 1: SSIM, 2: LSIM). Returns 0 for `NULL` arguments and short strides.
///
/// # Safety
///
/// The planes are null or readable for the samples that C reads: `width *
/// height` bytes when `x_step` is 1, the strided samples otherwise;
/// `distortion` and `result` are null or writable.
#[unsafe(export_name = "WebPPlaneDistortion")]
pub unsafe extern "C" fn webp_plane_distortion(
    src: *const u8,
    src_stride: usize,
    reference: *const u8,
    ref_stride: usize,
    width: c_int,
    height: c_int,
    x_step: usize,
    kind: c_int,
    distortion: *mut f32,
    result: *mut f32,
) -> c_int {
    if src.is_null() || reference.is_null() || result.is_null() || distortion.is_null() {
        return 0;
    }
    // The samples that C reads, or none when it fails before reading.
    let extent = |stride: usize| -> usize {
        let (Ok(w), Ok(h)) = (usize::try_from(width), usize::try_from(height)) else {
            return 0;
        };
        if w == 0
            || h == 0
            || src_stride < x_step.wrapping_mul(w)
            || ref_stride < x_step.wrapping_mul(w)
        {
            return 0;
        }
        if x_step == 1 {
            w * h
        } else {
            (h - 1) * stride + (w - 1) * x_step + 1
        }
    };
    // SAFETY: readable for the samples that C reads (caller's contract).
    let src = unsafe { slice::from_raw_parts(src, extent(src_stride)) };
    // SAFETY: as above.
    let reference = unsafe { slice::from_raw_parts(reference, extent(ref_stride)) };
    let mut d = 0.;
    let mut r = 0.;
    let ok = psnr::webp_plane_distortion(
        src, src_stride, reference, ref_stride, width, height, x_step, kind, &mut d, &mut r,
    );
    if ok {
        // SAFETY: writable (caller's contract).
        unsafe { distortion.write(d) };
        // SAFETY: as above.
        unsafe { result.write(r) };
    }
    c_int::from(ok)
}

// upstream: src/enc/picture_psnr_enc.c WebPPictureDistortion
/// Computes the PSNR, SSIM or LSIM distortion of `ref` against `src`, per
/// channel (B, G, R, A) and over all channels, in `result[5]`. Returns 0 for
/// `NULL` arguments, different dimensions and errors; the results are then not
/// written.
///
/// # Safety
///
/// `src` and `ref` are null or valid pictures whose planes that `use_argb`
/// selects are readable for their dimensions; `result` is null or writable for
/// five floats.
#[unsafe(export_name = "WebPPictureDistortion")]
pub unsafe extern "C" fn webp_picture_distortion(
    src: *const WebPPicture,
    reference: *const WebPPicture,
    kind: c_int,
    result: *mut f32,
) -> c_int {
    if src.is_null() || reference.is_null() || result.is_null() {
        return 0;
    }
    // SAFETY: a valid picture (caller's contract).
    let c_src = unsafe { *src };
    // SAFETY: as above.
    let c_ref = unsafe { *reference };
    let (yuva, argb) = selected(&c_src, Mode::Read);
    // SAFETY: the selected planes are readable (caller's contract).
    let Ok(source) = (unsafe { picture_to_rust(&c_src, yuva, argb) }) else {
        return 0;
    };
    let (yuva, argb) = selected(&c_ref, Mode::Read);
    // SAFETY: as above.
    let Ok(target) = (unsafe { picture_to_rust(&c_ref, yuva, argb) }) else {
        return 0;
    };
    let mut results = [0_f32; 5];
    if !psnr::webp_picture_distortion(&source.picture, &target.picture, kind, &mut results) {
        return 0;
    }
    // SAFETY: writable for five floats (caller's contract).
    unsafe { ptr::copy_nonoverlapping(results.as_ptr(), result, results.len()) };
    1
}

//------------------------------------------------------------------------------
// Import and colorspace conversion

/// The importers through the C ABI: the `NULL` checks, the packed rows and the
/// converted picture, which lends nothing (an importer replaces the planes it
/// writes).
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; `input` is null or points to the picture's rows.
unsafe fn import(
    picture: *mut WebPPicture,
    input: *const u8,
    stride: c_int,
    step: i32,
    import_alpha: bool,
    importer: fn(&mut RsPicture<'_>, &[u8], i32) -> bool,
) -> c_int {
    if input.is_null() {
        return 0;
    }
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return 0;
    };
    // SAFETY: the picture's rows (caller's contract).
    let Some((rows, stride)) =
        (unsafe { packed_rows(input, c.width, c.height, stride, step, import_alpha) })
    else {
        // The host refused the copy of rows with a negative stride.
        // SAFETY: a valid picture (caller's contract), no other borrow.
        set_error(
            unsafe { &mut *picture },
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        );
        return 0;
    };
    // SAFETY: a valid picture; nothing is lent.
    let result = unsafe {
        with_picture(picture, (Mode::Discard, Mode::Discard), false, |rs| {
            importer(rs, &rows, stride)
        })
    };
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportRGB
/// Imports RGB rows of `rgb_stride` bytes into the picture, as `use_argb`
/// selects. Returns 0 on error and for `NULL` arguments.
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; the rows are null or point to `height` rows of
/// the stride, which nothing writes during the call.
#[unsafe(export_name = "WebPPictureImportRGB")]
pub unsafe extern "C" fn webp_picture_import_rgb(
    picture: *mut WebPPicture,
    rgb: *const u8,
    rgb_stride: c_int,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        import(
            picture,
            rgb,
            rgb_stride,
            3,
            false,
            csp::webp_picture_import_rgb,
        )
    }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportRGBA
/// Imports RGBA rows; see [`webp_picture_import_rgb`].
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; the rows are null or point to `height` rows of
/// the stride, which nothing writes during the call.
#[unsafe(export_name = "WebPPictureImportRGBA")]
pub unsafe extern "C" fn webp_picture_import_rgba(
    picture: *mut WebPPicture,
    rgba: *const u8,
    rgba_stride: c_int,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        import(
            picture,
            rgba,
            rgba_stride,
            4,
            true,
            csp::webp_picture_import_rgba,
        )
    }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportRGBX
/// Imports RGBX rows, ignoring the fourth byte; see [`webp_picture_import_rgb`].
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; the rows are null or point to `height` rows of
/// the stride, which nothing writes during the call.
#[unsafe(export_name = "WebPPictureImportRGBX")]
pub unsafe extern "C" fn webp_picture_import_rgbx(
    picture: *mut WebPPicture,
    rgbx: *const u8,
    rgbx_stride: c_int,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        import(
            picture,
            rgbx,
            rgbx_stride,
            4,
            false,
            csp::webp_picture_import_rgbx,
        )
    }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportBGR
/// Imports BGR rows; see [`webp_picture_import_rgb`].
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; the rows are null or point to `height` rows of
/// the stride, which nothing writes during the call.
#[unsafe(export_name = "WebPPictureImportBGR")]
pub unsafe extern "C" fn webp_picture_import_bgr(
    picture: *mut WebPPicture,
    bgr: *const u8,
    bgr_stride: c_int,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        import(
            picture,
            bgr,
            bgr_stride,
            3,
            false,
            csp::webp_picture_import_bgr,
        )
    }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportBGRA
/// Imports BGRA rows; see [`webp_picture_import_rgb`].
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; the rows are null or point to `height` rows of
/// the stride, which nothing writes during the call.
#[unsafe(export_name = "WebPPictureImportBGRA")]
pub unsafe extern "C" fn webp_picture_import_bgra(
    picture: *mut WebPPicture,
    bgra: *const u8,
    bgra_stride: c_int,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        import(
            picture,
            bgra,
            bgra_stride,
            4,
            true,
            csp::webp_picture_import_bgra,
        )
    }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureImportBGRX
/// Imports BGRX rows, ignoring the fourth byte; see [`webp_picture_import_rgb`].
///
/// # Safety
///
/// `picture` is null or a valid picture whose memory fields are null or blocks
/// of the platform `malloc`; the rows are null or point to `height` rows of
/// the stride, which nothing writes during the call.
#[unsafe(export_name = "WebPPictureImportBGRX")]
pub unsafe extern "C" fn webp_picture_import_bgrx(
    picture: *mut WebPPicture,
    bgrx: *const u8,
    bgrx_stride: c_int,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        import(
            picture,
            bgrx,
            bgrx_stride,
            4,
            false,
            csp::webp_picture_import_bgrx,
        )
    }
}

/// A conversion from the ARGB plane to new YUVA planes: the ARGB plane is read,
/// the YUVA planes are replaced.
///
/// # Safety
///
/// `picture` is null or a valid picture whose ARGB plane is readable for its
/// dimensions and whose memory fields are null or blocks of the platform
/// `malloc`.
unsafe fn argb_to_yuva(
    picture: *mut WebPPicture,
    convert: impl FnOnce(&mut RsPicture<'_>) -> Result<(), WebPEncodingError>,
) -> c_int {
    if picture.is_null() {
        return 0;
    }
    // SAFETY: the ARGB plane is readable (caller's contract).
    let result = unsafe {
        with_picture(picture, (Mode::Discard, Mode::Read), true, |rs| {
            convert(rs).is_ok()
        })
    };
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}

// upstream: src/enc/picture_csp_enc.c WebPPictureARGBToYUVA
/// Converts the ARGB samples to the YUV420(A) colorspace. Returns 0 on error
/// (recorded in `error_code`) and for a `NULL` picture.
///
/// # Safety
///
/// `picture` is null or a valid picture whose ARGB plane is readable for its
/// dimensions and whose memory fields are null or blocks of the platform
/// `malloc`.
#[unsafe(export_name = "WebPPictureARGBToYUVA")]
pub unsafe extern "C" fn webp_picture_argb_to_yuva(
    picture: *mut WebPPicture,
    colorspace: WebPEncCSP,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe { argb_to_yuva(picture, |rs| csp::webp_picture_argb_to_yuva(rs, colorspace)) }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureARGBToYUVADithered
/// As [`webp_picture_argb_to_yuva`], with dithering of strength `dithering`.
///
/// # Safety
///
/// `picture` is null or a valid picture whose ARGB plane is readable for its
/// dimensions and whose memory fields are null or blocks of the platform
/// `malloc`.
#[unsafe(export_name = "WebPPictureARGBToYUVADithered")]
pub unsafe extern "C" fn webp_picture_argb_to_yuva_dithered(
    picture: *mut WebPPicture,
    colorspace: WebPEncCSP,
    dithering: f32,
) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe {
        argb_to_yuva(picture, |rs| {
            csp::webp_picture_argb_to_yuva_dithered(rs, colorspace, dithering)
        })
    }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureSharpARGBToYUVA
/// As [`webp_picture_argb_to_yuva`], with the sharp (iterative) conversion.
///
/// # Safety
///
/// `picture` is null or a valid picture whose ARGB plane is readable for its
/// dimensions and whose memory fields are null or blocks of the platform
/// `malloc`.
#[unsafe(export_name = "WebPPictureSharpARGBToYUVA")]
pub unsafe extern "C" fn webp_picture_sharp_argb_to_yuva(picture: *mut WebPPicture) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe { argb_to_yuva(picture, csp::webp_picture_sharp_argb_to_yuva) }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureSmartARGBToYUVA
/// Kept for backward compatibility: [`webp_picture_sharp_argb_to_yuva`].
///
/// # Safety
///
/// `picture` is null or a valid picture whose ARGB plane is readable for its
/// dimensions and whose memory fields are null or blocks of the platform
/// `malloc`.
#[unsafe(export_name = "WebPPictureSmartARGBToYUVA")]
pub unsafe extern "C" fn webp_picture_smart_argb_to_yuva(picture: *mut WebPPicture) -> c_int {
    // SAFETY: the caller's contract, passed on.
    unsafe { argb_to_yuva(picture, csp::webp_picture_smart_argb_to_yuva) }
}

// upstream: src/enc/picture_csp_enc.c WebPPictureYUVAToARGB
/// Converts the YUV420(A) planes to a new ARGB plane and sets `use_argb`.
/// Returns 0 on error (recorded in `error_code`) and for a `NULL` picture.
///
/// # Safety
///
/// `picture` is null or a valid picture whose YUVA planes are readable for its
/// dimensions and whose memory fields are null or blocks of the platform
/// `malloc`.
#[unsafe(export_name = "WebPPictureYUVAToARGB")]
pub unsafe extern "C" fn webp_picture_yuva_to_argb(picture: *mut WebPPicture) -> c_int {
    if picture.is_null() {
        return 0;
    }
    // SAFETY: the YUVA planes are readable (caller's contract).
    let result = unsafe {
        with_picture(picture, (Mode::Read, Mode::Discard), true, |rs| {
            csp::webp_picture_yuva_to_argb(rs).is_ok()
        })
    };
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}

//------------------------------------------------------------------------------
// Picture tools

// upstream: src/enc/picture_tools_enc.c WebPCleanupTransparentArea
/// Flattens the samples under fully transparent areas, so they compress
/// better; a `NULL` picture is ignored.
///
/// # Safety
///
/// `picture` is null or a valid picture whose planes that `use_argb` selects
/// are writable for its dimensions.
#[unsafe(export_name = "WebPCleanupTransparentArea")]
pub unsafe extern "C" fn webp_cleanup_transparent_area(picture: *mut WebPPicture) {
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return;
    };
    let modes = selected(c, Mode::Write);
    // SAFETY: the selected planes are writable (caller's contract).
    let _ = unsafe { with_picture(picture, modes, false, tools::webp_cleanup_transparent_area) };
}

// upstream: src/enc/picture_tools_enc.c WebPPictureHasTransparency
/// Returns true if the picture has an alpha sample below 0xff; 0 for a `NULL`
/// picture.
///
/// # Safety
///
/// `picture` is null or a valid picture whose planes that `use_argb` selects
/// are readable for its dimensions.
#[unsafe(export_name = "WebPPictureHasTransparency")]
#[must_use]
pub unsafe extern "C" fn webp_picture_has_transparency(picture: *const WebPPicture) -> c_int {
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return 0;
    };
    // C reads only the alpha plane of a YUV picture, and the ARGB plane of an
    // ARGB picture.
    let mut read = *c;
    if read.use_argb == 0 {
        (read.y, read.u, read.v) = (ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
    }
    let (yuva, argb) = selected(&read, Mode::Read);
    // SAFETY: the plane that C reads is readable (caller's contract).
    let Ok(converted) = (unsafe { picture_to_rust(&read, yuva, argb) }) else {
        return 0;
    };
    c_int::from(csp::webp_picture_has_transparency(&converted.picture))
}

// upstream: src/enc/picture_tools_enc.c WebPBlendAlpha
/// Blends the samples with the background color `background_rgb` (0x00RRGGBB)
/// and makes them opaque; a `NULL` picture is ignored.
///
/// # Safety
///
/// As [`webp_cleanup_transparent_area`].
#[unsafe(export_name = "WebPBlendAlpha")]
pub unsafe extern "C" fn webp_blend_alpha(picture: *mut WebPPicture, background_rgb: u32) {
    // SAFETY: null or a valid picture (caller's contract).
    let Some(c) = (unsafe { picture.as_ref() }) else {
        return;
    };
    let modes = selected(c, Mode::Write);
    // SAFETY: the selected planes are writable (caller's contract).
    let _ = unsafe {
        with_picture(picture, modes, false, |rs| {
            tools::webp_blend_alpha(rs, background_rgb);
        })
    };
}

//------------------------------------------------------------------------------
// Encoding

/// The C writer as the translation's writer: it passes the caller's picture
/// pointer, as C does.
fn writer_closure<'a>(
    writer: WebPWriterFunction,
    picture: *const WebPPicture,
) -> RsWriterFunction<'a> {
    Box::new(move |data: &[u8]| {
        // SAFETY: the caller's writer, called with the data, its size and the
        // caller's picture, which no Rust reference borrows during the call.
        unsafe { writer(data.as_ptr(), data.len(), picture) != 0 }
    })
}

/// The C progress hook as the translation's hook: it passes the caller's
/// picture pointer, as C does.
fn progress_closure<'a>(hook: WebPProgressHook, picture: *const WebPPicture) -> RsProgressHook<'a> {
    Box::new(move |percent: i32| {
        // SAFETY: the caller's hook, called with the percent and the caller's
        // picture, which no Rust reference borrows during the call.
        unsafe { hook(percent, picture) != 0 }
    })
}

// upstream: src/enc/webp_enc.c WebPEncode
/// Main encoding call: compresses the picture with `config` and writes the
/// data through `picture->writer`. Returns 0 on error, recorded in
/// `picture->error_code`; `NULL` arguments return 0.
///
/// The call lends the planes that C's path reads or writes, and nothing when the
/// configuration or the picture fails C's validation. The translation encodes
/// with zeroed copies of the statistics and of the `extra_info` map (one byte
/// per macroblock): the statistics go back to `*picture->stats` once the
/// validation has passed, where C resets and fills them. The map goes back to
/// `picture->extra_info` when the lossy encoder ran, also after a failure (C
/// fills it one macroblock at a time; the entries it did not reach are zero
/// here and untouched in C), and when a lossless encoding succeeds, where C
/// zeroes it.
///
/// # Safety
///
/// `config` is null or a valid configuration; `picture` is null or a valid
/// picture whose planes are writable for its dimensions, whose memory fields are
/// null or blocks of the platform `malloc`, and whose `writer`, `stats` and
/// `extra_info` meet `encode.h`'s contract.
#[unsafe(export_name = "WebPEncode")]
pub unsafe extern "C" fn webp_encode(
    config: *const WebPConfig,
    picture: *mut WebPPicture,
) -> c_int {
    // SAFETY: null or a valid picture (caller's contract); the borrow ends
    // before the encoding.
    let Some(c) = (unsafe { picture.as_mut() }) else {
        return 0;
    };
    c.error_code = WebPEncodingError::VP8_ENC_OK; // all ok so far
    // SAFETY: null or a valid configuration (caller's contract).
    let Some(config) = (unsafe { config.as_ref() }).copied() else {
        set_error(c, WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER);
        return 0;
    };
    let (c_stats, c_extra) = (c.stats, c.extra_info);
    // C's validation, before WebPEncode touches the statistics or a plane:
    // WebPValidateConfig, WebPValidatePicture and the dimension limit.
    let mut dimensions = RsPicture {
        width: c.width,
        height: c.height,
        colorspace: c.colorspace,
        ..RsPicture::default()
    };
    let validated = config_rs::webp_validate_config(&config)
        && picture_rs::webp_validate_picture(&mut dimensions).is_ok()
        && c.width <= WEBP_MAX_DIMENSION
        && c.height <= WEBP_MAX_DIMENSION;
    // The planes that C's path reads or writes.
    let has_yuv = !c.y.is_null() && !c.u.is_null() && !c.v.is_null();
    let modes = if !validated {
        (Mode::Discard, Mode::Discard)
    } else if config.lossless == 0 {
        if c.use_argb == 0 && has_yuv {
            // The cleanup and show_compressed write the YUVA planes.
            (Mode::Write, Mode::Discard)
        } else {
            // The conversion reads the ARGB plane and replaces the YUVA planes.
            (Mode::Discard, Mode::Read)
        }
    } else if !c.argb.is_null() {
        // WebPReplaceTransparentPixels writes the ARGB plane of an ARGB picture.
        let write = c.use_argb != 0 && config.exact == 0;
        (Mode::Discard, if write { Mode::Write } else { Mode::Read })
    } else {
        // WebPPictureYUVAToARGB reads the YUVA planes and makes the ARGB plane.
        (Mode::Read, Mode::Discard)
    };
    let wants_stats = !c_stats.is_null();
    let mut stats = WebPAuxStats::default();
    let mb_count = if validated && !c_extra.is_null() {
        usize::try_from(((c.width + 15) >> 4) * ((c.height + 15) >> 4)).unwrap_or(0)
    } else {
        0
    };
    let mut extra = Vec::new();
    if extra.try_reserve_exact(mb_count).is_err() {
        set_error(c, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
        return 0;
    }
    extra.resize(mb_count, 0_u8);
    let (writer, hook) = (c.writer, c.progress_hook);
    // SAFETY: the planes of the modes are readable, or writable for `Write`
    // (caller's contract).
    let result = unsafe {
        with_picture(picture, modes, true, |rs| {
            rs.writer = writer.map(|writer| writer_closure(writer, picture));
            rs.progress_hook = hook.map(|hook| progress_closure(hook, picture));
            rs.stats = wants_stats.then_some(&mut stats);
            rs.extra_info = (mb_count > 0).then_some(extra.as_mut_slice());
            let ok = enc::webp_encode(&config, rs);
            rs.writer = None;
            rs.progress_hook = None;
            rs.stats = None;
            rs.extra_info = None;
            ok
        })
    };
    if validated && wants_stats && result.is_some() {
        // SAFETY: a valid, writable `WebPAuxStats` (caller's contract); C has
        // reset it at this point.
        unsafe { c_stats.write(stats) };
    }
    // The map, which is only lent once the validation has passed: whatever the
    // lossy encoder filled, or a successful lossless encoding's zeros.
    let lossy = config.lossless == 0;
    if mb_count > 0 && result.is_some_and(|(encoded, _)| lossy || encoded) {
        // SAFETY: one writable byte per macroblock (caller's contract).
        unsafe { ptr::copy_nonoverlapping(extra.as_ptr(), c_extra, mb_count) };
    }
    c_int::from(result.is_some_and(|(ok, written)| ok && written))
}
