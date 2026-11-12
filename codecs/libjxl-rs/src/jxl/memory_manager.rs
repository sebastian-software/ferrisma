// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! JxlMemoryManager from include/jxl/memory_manager.h and the complete
//! MemoryManagerInit/Alloc/Free/default callbacks in memory_manager_internal.cc.
//! Custom callbacks receive the original size and own the actual payload.
//! The Rust default allocator keeps its deallocation layout before the returned
//! block; neither that private header nor the payload is zero-initialized.
use super::base::status::{Status, StatusCode};
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;

pub type JpegxlAllocFunc = unsafe extern "C" fn(*mut c_void, usize) -> *mut c_void;
pub type JpegxlFreeFunc = unsafe extern "C" fn(*mut c_void, *mut c_void);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct JxlMemoryManager {
    pub opaque: *mut c_void,
    pub alloc: Option<JpegxlAllocFunc>,
    pub free: Option<JpegxlFreeFunc>,
}
impl Default for JxlMemoryManager {
    fn default() -> Self {
        Self {
            opaque: std::ptr::null_mut(),
            alloc: None,
            free: None,
        }
    }
}

// Rust's global allocator needs the original Layout on free. Keep only that
// allocation metadata outside the caller's payload. Custom callback blocks do
// not pass through these functions and receive no prefix or extra request.
const DEFAULT_ALIGNMENT: usize = std::mem::align_of::<u128>();
const DEFAULT_HEADER: usize = DEFAULT_ALIGNMENT;
pub(crate) unsafe extern "C" fn default_alloc(_opaque: *mut c_void, size: usize) -> *mut c_void {
    let Some(physical_size) = size.max(1).checked_add(DEFAULT_HEADER) else {
        return std::ptr::null_mut();
    };
    let Ok(layout) = Layout::from_size_align(physical_size, DEFAULT_ALIGNMENT) else {
        return std::ptr::null_mut();
    };
    // SAFETY: Layout is checked and nonzero. The returned allocation has a
    // private initialized size word; default_free alone releases its base.
    let base = unsafe { alloc(layout) };
    if base.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: The header is aligned for usize and at least one word long. The
    // payload lies within this allocation and remains entirely uninitialized.
    unsafe {
        base.cast::<usize>().write(physical_size);
        base.add(DEFAULT_HEADER).cast()
    }
}
/// `default_alloc` with a zero-initialized payload, obtained from the global
/// allocator's zeroing path (calloc for this alignment) instead of an explicit
/// write. Large zeroed blocks can then stay lazily mapped until first written,
/// as the original's uninitialized blocks do. Released by `default_free`.
pub(crate) fn default_alloc_zeroed(size: usize) -> *mut c_void {
    let Some(physical_size) = size.max(1).checked_add(DEFAULT_HEADER) else {
        return std::ptr::null_mut();
    };
    let Ok(layout) = Layout::from_size_align(physical_size, DEFAULT_ALIGNMENT) else {
        return std::ptr::null_mut();
    };
    // SAFETY: Layout is checked and nonzero; the block layout is identical to
    // default_alloc's, so default_free releases it unchanged.
    let base = unsafe { std::alloc::alloc_zeroed(layout) };
    if base.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: As in default_alloc; the payload is already zeroed.
    unsafe {
        base.cast::<usize>().write(physical_size);
        base.add(DEFAULT_HEADER).cast()
    }
}
/// Whether `manager` allocates through `default_alloc` (its opaque is unused).
///
/// # Safety
/// `manager` is readable and initialized.
pub(crate) unsafe fn uses_default_alloc(manager: *const JxlMemoryManager) -> bool {
    // SAFETY: The caller supplies a readable, initialized manager.
    let alloc = unsafe { (*manager).alloc };
    alloc.map(|f| f as usize) == Some(default_alloc as *const () as usize)
}
pub(crate) unsafe extern "C" fn default_free(_opaque: *mut c_void, address: *mut c_void) {
    if address.is_null() {
        return;
    }
    // SAFETY: This callback's protocol accepts only its still-owned allocation
    // returned by default_alloc (or null). Its initialized header records the
    // exact allocation Layout, and this call ends its unique ownership once.
    unsafe {
        let base = address.cast::<u8>().sub(DEFAULT_HEADER);
        let physical_size = base.cast::<usize>().read();
        let layout = Layout::from_size_align(physical_size, DEFAULT_ALIGNMENT)
            .expect("default allocator retains its original layout");
        dealloc(base, layout);
    }
}

/// Original MemoryManagerInit, including a failed pair's copied partial state.
///
/// # Safety
/// `this` must be writable for one manager. A nonnull `manager` must contain
/// initialized fields and be readable for one manager; it may equal `this`.
/// Function pointers must be null or valid callback pointers. No reference to
/// either storage may conflict with the reached read/write operations.
pub unsafe fn memory_manager_init(
    this: *mut JxlMemoryManager,
    manager: *const JxlMemoryManager,
) -> Status {
    // SAFETY: The caller supplies the original readable/writable object ranges.
    // Copying into a value before writing also preserves the self-alias case.
    unsafe {
        let value = if manager.is_null() {
            JxlMemoryManager::default()
        } else {
            manager.read()
        };
        this.write(value);
        let default_alloc_selected = value.alloc.is_none();
        let default_free_selected = value.free.is_none();
        if default_alloc_selected != default_free_selected {
            return Err(StatusCode::GenericError);
        }
        if default_alloc_selected {
            (*this).alloc = Some(default_alloc);
        }
        if default_free_selected {
            (*this).free = Some(default_free);
        }
    }
    Ok(())
}

/// Original MemoryManagerAlloc.
///
/// # Safety
/// The manager must be readable, initialized, and have a valid allocation
/// callback/opaque pair obeying the original allocator contract. It may be
/// changed by that callback; no Rust reference is retained across the call.
pub unsafe fn memory_manager_alloc(manager: *const JxlMemoryManager, size: usize) -> *mut c_void {
    // SAFETY: The caller supplies initialized live fields and a valid callback.
    // Read only the source's reached fields before invoking user code.
    unsafe {
        let allocate = (*manager)
            .alloc
            .expect("source requires an allocation callback");
        let opaque = (*manager).opaque;
        allocate(opaque, size)
    }
}

/// Original MemoryManagerFree, including forwarding a null allocation.
///
/// # Safety
/// The manager and free callback/opaque pair must remain valid. `address` must
/// be null or a still-owned block accepted by that callback, and must not be
/// used after the callback releases it. No conflicting Rust borrow may live.
pub unsafe fn memory_manager_free(manager: *const JxlMemoryManager, address: *mut c_void) {
    // SAFETY: The caller establishes this exact live manager/free protocol.
    unsafe {
        let release = (*manager).free.expect("source requires a free callback");
        let opaque = (*manager).opaque;
        release(opaque, address);
    }
}
