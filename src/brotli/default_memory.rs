// Copyright 2016 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Callable source-signature c/common/platform.c default allocator functions.
//! Rust deallocation requires the original Layout, so default-only blocks
//! have private aligned size bookkeeping before their returned payload. The
//! actual returned payload remains uninitialized; no shadow payload exists.
//! Custom source callbacks never use this header or these default functions.

use super::memory::{AllocFunc, Allocator, FreeFunc};
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::mem::{align_of, size_of};

// The header reserves a whole Rust alignment unit. The payload is aligned
// at least as strictly as native C max_align_t, not assumed equal to it.
pub const DEFAULT_ALIGNMENT: usize = align_of::<u128>();
// Actual macOS ARM source observation: C long double/max_align_t alignment8,
// whereas Rust u128 alignment16 is sufficient but is a different ABI fact.
// The other configured native branch is checked against its actual source
// row before acceptance; no other platform ABI is claimed from this mapping.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub const SOURCE_MAX_ALIGNMENT: usize = align_of::<f64>();
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub const SOURCE_MAX_ALIGNMENT: usize = align_of::<u128>();
pub(crate) const HEADER_SIZE: usize =
    size_of::<usize>().div_ceil(DEFAULT_ALIGNMENT) * DEFAULT_ALIGNMENT;

fn layout(size: usize) -> Option<Layout> {
    let physical = size.max(1).checked_add(HEADER_SIZE)?;
    Layout::from_size_align(physical, DEFAULT_ALIGNMENT).ok()
}

/// Original default allocation signature; opaque is intentionally ignored.
/// A zero-size request may return null or a freeable pointer in the source.
/// This mapping reserves one private physical payload byte but exposes zero
/// payload capacity. Arithmetic/layout/allocation failure returns null.
///
/// # Safety
/// The caller must treat a nonnull result as one uniquely owned allocation,
/// initialize bytes before typed reads, and release it exactly once through
/// brotli_default_free_func. No other allocator may release this pointer.
pub unsafe extern "C" fn brotli_default_alloc_func(
    _opaque: *mut c_void,
    size: usize,
) -> *mut c_void {
    let Some(layout) = layout(size) else {
        return std::ptr::null_mut();
    };
    // SAFETY: Layout validated the nonzero aligned physical extent. No Rust
    // reference or initialized payload value is created from its raw bytes.
    let base = unsafe { alloc(layout) };
    if base.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: The allocation includes HEADER_SIZE aligned bytes before the
    // payload. The header holds one aligned usize initialized here; padding
    // and the entire returned payload remain unknown. add retains the same
    // live allocation provenance, including the preceding private header.
    unsafe {
        base.cast::<usize>().write(size);
        base.add(HEADER_SIZE).cast()
    }
}

/// Original default free signature; null is accepted and opaque is ignored.
///
/// # Safety
/// A nonnull address must be the exact not-yet-freed pointer returned by
/// brotli_default_alloc_func. Every payload borrow must have ended. The
/// pointer cannot be used or released again after this call.
pub unsafe extern "C" fn brotli_default_free_func(_opaque: *mut c_void, address: *mut c_void) {
    if address.is_null() {
        return;
    }
    // SAFETY: The caller establishes producing allocator/pointer identity and
    // unique release. The header is in the same live allocation and its usize
    // was initialized by allocate, independently of unknown payload bytes.
    let base = unsafe { address.cast::<u8>().sub(HEADER_SIZE) };
    // SAFETY: The same producing-allocation proof establishes alignment and
    // initialization of this private size field, with no competing aliases.
    let size = unsafe { base.cast::<usize>().read() };
    let layout = layout(size).expect("the producing default allocation layout");
    // SAFETY: This is exactly the original global allocation and its original
    // Layout; every payload borrow ended and this is its unique release.
    unsafe { dealloc(base, layout) };
}

/// The actual callable default pair, suitable for storing in native fields.
pub fn callbacks() -> (AllocFunc, FreeFunc) {
    (brotli_default_alloc_func, brotli_default_free_func)
}

/// Retain producing allocator identity in the ordinary Allocation owner.
pub fn allocator() -> Allocator {
    allocator_with_opaque(std::ptr::null_mut())
}

/// The shared-dictionary source retains the original opaque argument even
/// when selecting defaults; those functions ignore it but its value remains
/// in the source field and is passed unchanged at every free checkpoint.
#[allow(
    clippy::not_unsafe_ptr_arg_deref,
    reason = "The fixed callable default thunks ignore opaque; this only retains its source field value"
)]
pub fn allocator_with_opaque(opaque: *mut c_void) -> Allocator {
    let (allocate, free) = callbacks();
    // SAFETY: These immutable functions remain callable forever, ignore the
    // supplied opaque context, return exclusive aligned payload blocks, match
    // every allocation with its exact free, and accept free(NULL).
    unsafe { Allocator::from_callbacks(Some(allocate), Some(free), opaque) }
        .expect("the default pair is complete")
}
