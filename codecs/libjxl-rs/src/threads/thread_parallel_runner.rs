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
pub use super::thread_parallel_runner_internal::RunnerError;
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
    // SAFETY: Forward the caller's memory-manager contract.
    unsafe { jxl_thread_parallel_runner_create_checked(memory_manager, num_worker_threads) }
        .unwrap_or(std::ptr::null_mut())
}

/// Rust-typed variant of [`jxl_thread_parallel_runner_create`] that
/// distinguishes invalid memory managers, allocation failures, and OS
/// thread-creation failures.
///
/// # Safety
/// Same as [`jxl_thread_parallel_runner_create`].
pub unsafe fn jxl_thread_parallel_runner_create_checked(
    memory_manager: *const JxlMemoryManager,
    num_worker_threads: usize,
) -> Result<*mut c_void, RunnerError> {
    let mut local_memory_manager = JxlMemoryManager::default();
    // SAFETY: The caller supplies a readable manager or null.
    if !unsafe { thread_memory_manager_init(&mut local_memory_manager, memory_manager) } {
        return Err(RunnerError::InvalidMemoryManager);
    }

    // SAFETY: The initialized callbacks obey the caller's allocator contract.
    let alloc = unsafe {
        thread_memory_manager_alloc(
            &local_memory_manager,
            std::mem::size_of::<ThreadParallelRunner>(),
        )
    };
    if alloc.is_null() {
        return Err(RunnerError::Allocation);
    }
    let runner = alloc.cast::<ThreadParallelRunner>();
    assert!(
        runner.is_aligned(),
        "the source's placement new requires an aligned runner allocation"
    );
    // Placement new constructor on allocated memory. Clamp to `i32::MAX` so
    // narrowing to the source's `int` constructor parameter never wraps
    // negative.
    let workers = i32::try_from(num_worker_threads).unwrap_or(i32::MAX);
    // SAFETY: The callback returned writable storage of the requested size,
    // checked aligned above; on failure `construct` has already joined any
    // started workers and dropped the object so freeing `runner` is safe.
    unsafe {
        if let Err(err) = ThreadParallelRunner::construct(runner, workers) {
            thread_memory_manager_free(&local_memory_manager, runner.cast());
            return Err(err);
        }
        (*runner).set_memory_manager(local_memory_manager);
    }

    Ok(runner.cast())
}

/// Creates a lazy `ThreadParallelRunner` that spawns zero worker threads at
/// creation time. Single-task ranges run directly on the calling thread;
/// multi-task ranges grow the worker pool on demand up to
/// `min(max_worker_threads, end_range - start_range)`.
///
/// # Safety
/// Same as [`jxl_thread_parallel_runner_create`].
pub unsafe fn jxl_thread_parallel_runner_create_lazy(
    memory_manager: *const JxlMemoryManager,
    max_worker_threads: usize,
) -> Result<*mut c_void, RunnerError> {
    let mut local_memory_manager = JxlMemoryManager::default();
    // SAFETY: The caller supplies a readable manager or null.
    if !unsafe { thread_memory_manager_init(&mut local_memory_manager, memory_manager) } {
        return Err(RunnerError::InvalidMemoryManager);
    }

    // SAFETY: The initialized callbacks obey the caller's allocator contract.
    let alloc = unsafe {
        thread_memory_manager_alloc(
            &local_memory_manager,
            std::mem::size_of::<ThreadParallelRunner>(),
        )
    };
    if alloc.is_null() {
        return Err(RunnerError::Allocation);
    }
    let runner = alloc.cast::<ThreadParallelRunner>();
    assert!(
        runner.is_aligned(),
        "the source's placement new requires an aligned runner allocation"
    );
    // Clamp before narrowing to `i32` so values above `i32::MAX` never wrap
    // negative and desynchronize `num_threads()` from the worker cap.
    let workers = i32::try_from(max_worker_threads).unwrap_or(i32::MAX);
    // SAFETY: Writable, aligned storage of the exact runner layout.
    unsafe {
        ThreadParallelRunner::construct_lazy(runner, workers);
        (*runner).set_memory_manager(local_memory_manager);
    }

    Ok(runner.cast())
}

/// Returns the failure reason recorded by a lazy runner when on-demand worker
/// startup failed during [`jxl_thread_parallel_runner`].
///
/// # Safety
/// `runner_opaque` is null or a live runner from
/// `jxl_thread_parallel_runner_create*`.
pub unsafe fn jxl_thread_parallel_runner_last_error(
    runner_opaque: *mut c_void,
) -> Option<RunnerError> {
    let runner = runner_opaque.cast_const().cast::<ThreadParallelRunner>();
    if runner.is_null() {
        return None;
    }
    // SAFETY: The caller passes a live runner; `last_error` reads an atomic.
    unsafe { (*runner).last_error() }
}

/// Updates the maximum worker count of a lazy runner before any workers have
/// been spawned (for example, once a frame's actual group count is known).
///
/// # Safety
/// `runner_opaque` is null or a live, idle runner from
/// `jxl_thread_parallel_runner_create*`.
pub unsafe fn jxl_thread_parallel_runner_set_max_workers(
    runner_opaque: *mut c_void,
    max_workers: usize,
) {
    let runner = runner_opaque.cast_const().cast::<ThreadParallelRunner>();
    if runner.is_null() {
        return;
    }
    // SAFETY: The caller passes a live, idle runner.
    unsafe { (*runner).set_max_workers(max_workers) };
}

/// Returns the number of live worker threads currently spawned by the runner.
///
/// # Safety
/// `runner_opaque` is null or a live runner from
/// `jxl_thread_parallel_runner_create*`.
pub unsafe fn jxl_thread_parallel_runner_spawned_threads(runner_opaque: *mut c_void) -> usize {
    let runner = runner_opaque.cast_const().cast::<ThreadParallelRunner>();
    if runner.is_null() {
        return 0;
    }
    // SAFETY: The caller passes a live runner; `spawned_threads` reads an atomic.
    unsafe { (*runner).spawned_threads() }
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
