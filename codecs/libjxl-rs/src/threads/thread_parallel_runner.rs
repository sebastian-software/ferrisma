// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The public thread runner functions of `lib/threads/thread_parallel_runner.cc`
//! (declared in `lib/include/jxl/thread_parallel_runner.h`).
//!
//! Language-required deviations: the functions keep their C signatures as
//! Rust functions; the C ABI exports are a separate boundary. The source's
//! private malloc/free pair, "same as the default JxlMemoryManager for the
//! jpegxl library itself", is that library's Rust default pair. The runner
//! object has its Rust layout size. A custom allocation must be aligned for
//! it, as the source's placement new requires; a misaligned block is
//! rejected with a panic instead of being used.
use super::hardware_concurrency;
use super::thread_parallel_runner_internal::ThreadParallelRunner;
use crate::jxl::base::data_parallel::{
    JxlParallelRetCode, JxlParallelRunFunction, JxlParallelRunInit,
};
use crate::jxl::memory_manager::{JxlMemoryManager, default_alloc, default_free};
use std::ffi::c_void;

// Initializes the memory manager instance with the passed one. The
// MemoryManager passed in |memory_manager| may be NULL or contain NULL
// functions which will be initialized with the default ones. If either alloc
// or free are NULL, then both must be NULL, otherwise this function returns an
// error.
//
// # Safety
// A nonnull `memory_manager` is readable and initialized.
unsafe fn thread_memory_manager_init(
    this: &mut JxlMemoryManager,
    memory_manager: *const JxlMemoryManager,
) -> bool {
    if memory_manager.is_null() {
        *this = JxlMemoryManager::default();
    } else {
        // SAFETY: The caller supplies a readable, initialized manager.
        *this = unsafe { memory_manager.read() };
    }
    let is_default_alloc = this.alloc.is_none();
    let is_default_free = this.free.is_none();
    if is_default_alloc != is_default_free {
        return false;
    }
    if is_default_alloc {
        this.alloc = Some(default_alloc);
    }
    if is_default_free {
        this.free = Some(default_free);
    }
    true
}

// # Safety
// The manager's allocation callback obeys the memory-manager contract.
unsafe fn thread_memory_manager_alloc(
    memory_manager: &JxlMemoryManager,
    size: usize,
) -> *mut c_void {
    let alloc = memory_manager.alloc.expect("initialized above");
    // SAFETY: The caller's allocator contract.
    unsafe { alloc(memory_manager.opaque, size) }
}

// # Safety
// `address` is a still-owned block of this manager.
unsafe fn thread_memory_manager_free(memory_manager: &JxlMemoryManager, address: *mut c_void) {
    let free = memory_manager.free.expect("initialized at creation");
    // SAFETY: The caller's allocator contract.
    unsafe { free(memory_manager.opaque, address) };
}

/// JxlThreadParallelRunner: the parallel runner callback of a runner from
/// `jxl_thread_parallel_runner_create`.
///
/// # Safety
/// As `ThreadParallelRunner::runner`.
pub unsafe extern "C" fn jxl_thread_parallel_runner(
    runner_opaque: *mut c_void,
    jpegxl_opaque: *mut c_void,
    init: JxlParallelRunInit,
    func: JxlParallelRunFunction,
    start_range: u32,
    end_range: u32,
) -> JxlParallelRetCode {
    // SAFETY: Forward the caller's runner contract.
    unsafe {
        ThreadParallelRunner::runner(
            runner_opaque,
            jpegxl_opaque,
            init,
            func,
            start_range,
            end_range,
        )
    }
}

/// JxlThreadParallelRunnerCreate. Starts the given number of worker threads
/// and blocks until they are ready. If zero, all tasks run on the main thread.
///
/// # Safety
/// `memory_manager` is null or a readable manager whose callbacks obey the
/// memory-manager contract and stay valid until the runner is destroyed.
pub unsafe fn jxl_thread_parallel_runner_create(
    memory_manager: *const JxlMemoryManager,
    num_worker_threads: usize,
) -> *mut c_void {
    let mut local_memory_manager = JxlMemoryManager::default();
    // SAFETY: The caller supplies a readable manager or null.
    if !unsafe { thread_memory_manager_init(&mut local_memory_manager, memory_manager) } {
        return std::ptr::null_mut();
    }

    // SAFETY: The initialized callbacks obey the caller's allocator contract.
    let alloc = unsafe {
        thread_memory_manager_alloc(
            &local_memory_manager,
            std::mem::size_of::<ThreadParallelRunner>(),
        )
    };
    if alloc.is_null() {
        return std::ptr::null_mut();
    }
    let runner = alloc.cast::<ThreadParallelRunner>();
    assert!(
        runner.is_aligned(),
        "the source's placement new requires an aligned runner allocation"
    );
    // Placement new constructor on allocated memory. The source's int
    // constructor parameter narrows the size_t count.
    // SAFETY: The callback returned writable storage of the requested size,
    // checked aligned above; it stays in place until destruction.
    unsafe {
        ThreadParallelRunner::construct(runner, num_worker_threads as i32);
        (*runner).set_memory_manager(local_memory_manager);
    }

    runner.cast()
}

/// JxlThreadParallelRunnerDestroy.
///
/// # Safety
/// `runner_opaque` is null or a runner from `jxl_thread_parallel_runner_create`
/// that no runner call uses and that is not used again.
pub unsafe fn jxl_thread_parallel_runner_destroy(runner_opaque: *mut c_void) {
    let runner = runner_opaque.cast::<ThreadParallelRunner>();
    if !runner.is_null() {
        // SAFETY: The caller transfers the live runner. Copy its manager,
        // call the destructor directly since a custom free function is used,
        // then release the storage with that manager.
        unsafe {
            let local_memory_manager = (*runner).memory_manager();
            ThreadParallelRunner::destruct(runner);
            thread_memory_manager_free(&local_memory_manager, runner.cast());
        }
    }
}

/// JxlThreadParallelRunnerDefaultNumWorkerThreads: the default value for
/// num_worker_threads of `jxl_thread_parallel_runner_create`.
pub fn jxl_thread_parallel_runner_default_num_worker_threads() -> usize {
    hardware_concurrency() as usize
}
