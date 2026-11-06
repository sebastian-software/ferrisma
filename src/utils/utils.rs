// libwebp v1.6.0, src/utils/utils.c:
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
// Misc. common utility functions
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/utils.h:
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
// Misc. common utility functions
//
// Authors: Skal (pascal.massimino@gmail.com)
//          Urvang (urvang@google.com)

//! Translation of `src/utils/utils.c` and `src/utils/utils.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! Exception: WP3 translates `WebPGetColorPalette` from `src/utils/utils.c`, because it calls `GetColorPalette` from `src/utils/palette.c`.
//!
//! Not translated here, with the reason:
//! - `WebPMalloc` and `WebPFree` hand memory to C callers; they belong to the C
//!   ABI module (ADR 0004, `src/abi/types.rs`, WP12).
//! - `WebPSafeFree` only frees; the owner of a `Vec` drops it instead.
//! - `WebPLog2FloorC` and `WebPLogTable8bit` are the fallback of `BitsLog2Floor`
//!   and `BitsCtz`, which GCC and Clang never compile; they use the builtins.
//! - The `PRINT_MEM_INFO` bookkeeping is off in the reference configuration.

use crate::webp::encode::WebPPicture;

// upstream: src/utils/utils.h WEBP_ALIGN_CST
/// Mask of the alignment that `webp_align` applies.
pub const WEBP_ALIGN_CST: usize = 31;

// upstream: src/utils/utils.h WEBP_MAX_ALLOCABLE_MEMORY
/// The largest amount of memory that libwebp will try to allocate.
pub const WEBP_MAX_ALLOCABLE_MEMORY: u64 = if usize::MAX as u64 > (1 << 34) {
    1 << 34
} else {
    (1 << 31) - (1 << 16)
};

// upstream: src/utils/utils.h CheckSizeOverflow
/// Reports whether a byte count is representable as a `usize`.
#[must_use]
pub fn check_size_overflow(size: u64) -> bool {
    usize::try_from(size).is_ok()
}

// upstream: src/utils/utils.c CheckSizeArgumentsOverflow
/// The size test of `WebPSafeMalloc` and `WebPSafeCalloc`: zero elements always
/// pass; otherwise `size` must not exceed `WEBP_MAX_ALLOCABLE_MEMORY / nmemb`,
/// and the product must fit in `usize`. The product wraps in 64 bits, as in C.
#[must_use]
fn check_size_arguments_overflow(nmemb: u64, size: usize) -> bool {
    let size = size as u64;
    let total_size = nmemb.wrapping_mul(size);
    if nmemb == 0 {
        return true;
    }
    if size > WEBP_MAX_ALLOCABLE_MEMORY / nmemb {
        return false;
    }
    check_size_overflow(total_size)
}

// upstream: src/utils/utils.c WebPSafeMalloc
/// Reserves room for `nmemb` elements of `T`, after the size test of
/// `WebPSafeMalloc`.
///
/// `size` is the C `sizeof` of one element, so that the decision matches the C
/// function even where the Rust type has another size. The returned vector is
/// empty with a capacity of at least `nmemb`; the caller initializes the
/// elements it reads. Returns `None` when the size test fails or the host
/// refuses the reservation, which is where C returns `NULL`.
#[must_use]
pub fn webp_safe_malloc<T>(nmemb: u64, size: usize) -> Option<Vec<T>> {
    if !check_size_arguments_overflow(nmemb, size) {
        return None;
    }
    let count = usize::try_from(nmemb).ok()?;
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count).ok()?;
    Some(buffer)
}

// upstream: src/utils/utils.c WebPSafeCalloc
/// Like [`webp_safe_malloc`], with `nmemb` elements set to `T::default()`, which
/// is the zero value that C's `calloc` provides for the numeric types.
#[must_use]
pub fn webp_safe_calloc<T: Default + Clone>(nmemb: u64, size: usize) -> Option<Vec<T>> {
    let count = usize::try_from(nmemb).ok()?;
    let mut buffer = webp_safe_malloc::<T>(nmemb, size)?;
    buffer.resize(count, T::default());
    Some(buffer)
}

// upstream: src/utils/utils.h WEBP_ALIGN
/// Rounds an address up to a multiple of 32, as `WEBP_ALIGN` does for a
/// `uintptr_t`. The address is passed as an integer, because Rust code keeps no
/// raw pointers outside the C ABI.
#[must_use]
pub fn webp_align(address: usize) -> usize {
    address.wrapping_add(WEBP_ALIGN_CST) & !WEBP_ALIGN_CST
}

// upstream: src/utils/utils.h WebPMemToUint32
/// Reads four bytes in host byte order, as the C `memcpy` does.
///
/// # Panics
///
/// Panics when `ptr` holds fewer than four bytes; C reads past the buffer there.
#[must_use]
pub fn webp_mem_to_uint32(ptr: &[u8]) -> u32 {
    u32::from_ne_bytes([ptr[0], ptr[1], ptr[2], ptr[3]])
}

// upstream: src/utils/utils.h WebPMemToInt32
/// Reads four bytes in host byte order as an `i32`.
///
/// # Panics
///
/// Panics when `ptr` holds fewer than four bytes.
#[must_use]
pub fn webp_mem_to_int32(ptr: &[u8]) -> i32 {
    webp_mem_to_uint32(ptr).cast_signed()
}

// upstream: src/utils/utils.h WebPUint32ToMem
/// Writes `val` in host byte order into the first four bytes of `ptr`.
///
/// # Panics
///
/// Panics when `ptr` holds fewer than four bytes.
pub fn webp_uint32_to_mem(ptr: &mut [u8], val: u32) {
    ptr[..4].copy_from_slice(&val.to_ne_bytes());
}

// upstream: src/utils/utils.h WebPInt32ToMem
/// Writes `val` in host byte order into the first four bytes of `ptr`.
///
/// # Panics
///
/// Panics when `ptr` holds fewer than four bytes.
pub fn webp_int32_to_mem(ptr: &mut [u8], val: i32) {
    webp_uint32_to_mem(ptr, val.cast_unsigned());
}

// upstream: src/utils/utils.h GetLE16
/// Reads a 16-bit little-endian value.
///
/// # Panics
///
/// Panics when `data` holds fewer than two bytes.
#[must_use]
pub fn get_le16(data: &[u8]) -> i32 {
    i32::from(data[0]) | (i32::from(data[1]) << 8)
}

// upstream: src/utils/utils.h GetLE24
/// Reads a 24-bit little-endian value.
///
/// # Panics
///
/// Panics when `data` holds fewer than three bytes.
#[must_use]
pub fn get_le24(data: &[u8]) -> i32 {
    get_le16(data) | (i32::from(data[2]) << 16)
}

// upstream: src/utils/utils.h GetLE32
/// Reads a 32-bit little-endian value.
///
/// # Panics
///
/// Panics when `data` holds fewer than four bytes.
#[must_use]
pub fn get_le32(data: &[u8]) -> u32 {
    get_le16(data).cast_unsigned() | (get_le16(&data[2..]).cast_unsigned() << 16)
}

/// Keeps the low byte of `value`, as the C conversion `(val >> k) & 0xff` does.
#[expect(
    clippy::cast_sign_loss,
    reason = "the mask makes the value non-negative"
)]
#[must_use]
fn low_byte(value: i32) -> u8 {
    (value & 0xff) as u8
}

// upstream: src/utils/utils.h PutLE16
/// Writes a 16-bit little-endian value.
///
/// # Panics
///
/// Panics when `data` holds fewer than two bytes.
pub fn put_le16(data: &mut [u8], val: i32) {
    debug_assert!(val < (1 << 16));
    data[0] = low_byte(val);
    data[1] = low_byte(val >> 8);
}

// upstream: src/utils/utils.h PutLE24
/// Writes a 24-bit little-endian value.
///
/// # Panics
///
/// Panics when `data` holds fewer than three bytes.
pub fn put_le24(data: &mut [u8], val: i32) {
    debug_assert!(val < (1 << 24));
    put_le16(data, val & 0xffff);
    data[2] = low_byte(val >> 16);
}

// upstream: src/utils/utils.h PutLE32
/// Writes a 32-bit little-endian value.
///
/// # Panics
///
/// Panics when `data` holds fewer than four bytes.
pub fn put_le32(data: &mut [u8], val: u32) {
    put_le16(data, (val & 0xffff).cast_signed());
    put_le16(&mut data[2..], (val >> 16).cast_signed());
}

// upstream: src/utils/utils.h BitsLog2Floor
/// Returns `floor(log2(n))`, the builtin `31 ^ __builtin_clz(n)` of GCC and
/// Clang. `n` must be positive; the builtin is undefined for zero.
#[must_use]
pub fn bits_log2_floor(n: u32) -> i32 {
    debug_assert!(n > 0, "BitsLog2Floor needs a positive argument");
    (31 ^ n.leading_zeros()).cast_signed()
}

// upstream: src/utils/utils.h BitsCtz
/// Returns the number of trailing zero bits, the builtin `__builtin_ctz(n)`.
/// The builtin is undefined for zero; this returns 32, as the portable C
/// fallback does.
#[must_use]
pub fn bits_ctz(n: u32) -> i32 {
    n.trailing_zeros().cast_signed()
}

/// Start of row `row` of a plane whose first row starts at `offset` and whose
/// rows are `stride` bytes apart; a negative stride walks upward.
fn row_start(offset: usize, stride: i32, row: usize) -> usize {
    let offset = i64::try_from(offset).expect("a plane offset fits in i64");
    let row = i64::try_from(row).expect("a row index fits in i64");
    usize::try_from(offset + row * i64::from(stride)).expect("the row starts inside the buffer")
}

// upstream: src/utils/utils.c WebPCopyPlane
/// Copies `width` by `height` bytes from `src` to `dst`, honoring both strides.
/// The first row of `src` starts at `src_offset`, and row `y` at
/// `src_offset + y * src_stride`; `dst` is addressed the same way.
///
/// # Panics
///
/// Panics when a row falls outside its slice, or when `width` is negative and
/// `height` positive; the C function leaves both undefined.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C signature of WebPCopyPlane"
)]
pub fn webp_copy_plane(
    src: &[u8],
    src_offset: usize,
    src_stride: i32,
    dst: &mut [u8],
    dst_offset: usize,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    copy_rows(
        src, src_offset, src_stride, dst, dst_offset, dst_stride, width, height,
    );
}

/// Copies `width` by `height` samples of any type from `src` to `dst`, honoring both
/// strides. This is the row loop of `WebPCopyPlane`, which copies bytes, and of
/// `WebPCopyPixels`, which copies `u32` samples with the same row arithmetic.
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C copy of WebPCopyPlane"
)]
fn copy_rows<T: Copy>(
    src: &[T],
    src_offset: usize,
    src_stride: i32,
    dst: &mut [T],
    dst_offset: usize,
    dst_stride: i32,
    width: i32,
    height: i32,
) {
    debug_assert!(i64::from(src_stride).abs() >= i64::from(width));
    debug_assert!(i64::from(dst_stride).abs() >= i64::from(width));
    if height <= 0 {
        return;
    }
    let row_len = usize::try_from(width).expect("the copy needs a non-negative width");
    for row in 0..usize::try_from(height).expect("height is positive here") {
        let from = row_start(src_offset, src_stride, row);
        let to = row_start(dst_offset, dst_stride, row);
        dst[to..to + row_len].copy_from_slice(&src[from..from + row_len]);
    }
}

// upstream: src/utils/utils.c WebPCopyPixels
/// Copies the ARGB samples of `src` into `dst`, honoring both strides. C copies
/// `4 * width` bytes per row, with `4 * argb_stride` bytes between rows, and this
/// copies the same `u32` samples with the strides in pixels.
///
/// # Panics
///
/// Panics when either picture has no ARGB plane or a row falls outside its storage.
/// C asserts equal dimensions and `use_argb` in debug builds; the same asserts run
/// here in the debug and `checked` profiles.
pub fn webp_copy_pixels(src: &WebPPicture<'_>, dst: &mut WebPPicture<'_>) {
    debug_assert!(src.width == dst.width && src.height == dst.height);
    debug_assert!(src.use_argb != 0 && dst.use_argb != 0);
    let source_plane = src.argb.expect("WebPCopyPixels needs an ARGB source plane");
    let target_plane = dst
        .argb
        .expect("WebPCopyPixels needs an ARGB destination plane");
    let source = src
        .argb_storage
        .region(source_plane.region)
        .expect("the source ARGB region exists");
    let target = dst
        .argb_storage
        .region_mut(target_plane.region)
        .expect("the destination ARGB region exists");
    copy_rows(
        source,
        source_plane.offset,
        src.argb_stride,
        target,
        target_plane.offset,
        dst.argb_stride,
        src.width,
        src.height,
    );
}

//------------------------------------------------------------------------------

// upstream: src/utils/utils.c WebPGetColorPalette
/// Returns count of unique colors in `pic`, assuming `pic.use_argb` is true.
/// If the unique color count is more than `MAX_PALETTE_SIZE`, returns
/// `MAX_PALETTE_SIZE`+1.
/// If `palette` is not `None` and the number of unique colors is less than or
/// equal to `MAX_PALETTE_SIZE`, also outputs the actual unique colors into
/// `palette` in a sorted order. Note: `palette` is assumed to be an array
/// already allocated with at least `MAX_PALETTE_SIZE` elements.
///
/// Exception item of WP3 in WP1's module (`tools/units.json`).
#[must_use]
pub fn webp_get_color_palette(pic: &WebPPicture<'_>, palette: Option<&mut [u32]>) -> i32 {
    crate::utils::palette::get_color_palette(pic, palette)
}
