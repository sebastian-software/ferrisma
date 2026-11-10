// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The exported C ABI of `lib/include/jxl/thread_parallel_runner.h` and
//! `lib/include/jxl/resizable_parallel_runner.h`: every `JXL_THREADS_EXPORT`
//! function that `lib/threads/thread_parallel_runner.cc` and
//! `lib/threads/resizable_parallel_runner.cc` define, under its original
//! unmangled name, forwarding to the Rust port in
//! [`super::thread_parallel_runner`] and [`super::resizable_parallel_runner`].
//!
//! Language-required deviations, each local to its representation:
//! - The original exports these functions from its separate `jxl_threads`
//!   library. The Rust crate has one static and one dynamic library, which
//!   export them next to the decoder C ABI of `crate::jxl::decode_abi`; a C
//!   program links that library in place of both `jxl` and `jxl_threads`.
//! - `JxlParallelRunInit` and `JxlParallelRunFunction` are non-nullable Rust
//!   function pointers. The decoder always passes both; a C caller that
//!   invokes a runner directly must pass valid functions as well, even for an
//!   empty range in which the source would not call them.
//! - A panic, which the source does not have, cannot unwind into C: the
//!   `extern "C"` boundary aborts the process.

use super::resizable_parallel_runner as resizable;
use super::thread_parallel_runner as thread;
use crate::jxl::base::data_parallel::{
    JxlParallelRetCode, JxlParallelRunFunction, JxlParallelRunInit, JxlParallelRunner,
};
use crate::jxl::memory_manager::JxlMemoryManager;
use std::ffi::c_void;

// Both exported runners are JxlParallelRunner callbacks.
const _: [JxlParallelRunner; 2] = [JxlThreadParallelRunner, JxlResizableParallelRunner];

// ---------------------------------------------------------------------------
// thread_parallel_runner.h, in declaration order.

/// JxlThreadParallelRunner.
///
/// # Safety
/// `runner_opaque` is a live runner from JxlThreadParallelRunnerCreate.
/// `init` and `func` may be called with `jpegxl_opaque` from any of its
/// threads during this call, as parallel_runner.h specifies, and never unwind.
#[unsafe(no_mangle)] // SAFETY: The only definition of this thread_parallel_runner.h symbol.
pub unsafe extern "C" fn JxlThreadParallelRunner(
    runner_opaque: *mut c_void,
    jpegxl_opaque: *mut c_void,
    init: JxlParallelRunInit,
    func: JxlParallelRunFunction,
    start_range: u32,
    end_range: u32,
) -> JxlParallelRetCode {
    // SAFETY: Forwards the caller's runner contract.
    unsafe {
        thread::jxl_thread_parallel_runner(
            runner_opaque,
            jpegxl_opaque,
            init,
            func,
            start_range,
            end_range,
        )
    }
}

/// JxlThreadParallelRunnerCreate.
///
/// # Safety
/// `memory_manager` is null or a readable manager whose callbacks obey
/// memory_manager.h, never unwind and stay valid until the runner is
/// destroyed.
#[unsafe(no_mangle)] // SAFETY: The only definition of this thread_parallel_runner.h symbol.
pub unsafe extern "C" fn JxlThreadParallelRunnerCreate(
    memory_manager: *const JxlMemoryManager,
    num_worker_threads: usize,
) -> *mut c_void {
    // SAFETY: Forwards the caller's memory manager contract.
    unsafe { thread::jxl_thread_parallel_runner_create(memory_manager, num_worker_threads) }
}

/// JxlThreadParallelRunnerDestroy.
///
/// # Safety
/// `runner_opaque` is null or a runner from JxlThreadParallelRunnerCreate
/// that no runner call uses and that is not used again.
#[unsafe(no_mangle)] // SAFETY: The only definition of this thread_parallel_runner.h symbol.
pub unsafe extern "C" fn JxlThreadParallelRunnerDestroy(runner_opaque: *mut c_void) {
    // SAFETY: Forwards the caller's runner contract.
    unsafe { thread::jxl_thread_parallel_runner_destroy(runner_opaque) }
}

/// JxlThreadParallelRunnerDefaultNumWorkerThreads.
#[unsafe(no_mangle)] // SAFETY: The only definition of this thread_parallel_runner.h symbol.
pub extern "C" fn JxlThreadParallelRunnerDefaultNumWorkerThreads() -> usize {
    thread::jxl_thread_parallel_runner_default_num_worker_threads()
}

// ---------------------------------------------------------------------------
// resizable_parallel_runner.h, in declaration order.

/// JxlResizableParallelRunner.
///
/// # Safety
/// `runner_opaque` is a live runner from JxlResizableParallelRunnerCreate
/// that no other call uses concurrently. `init` and `func` may be called with
/// `jpegxl_opaque` from any of its threads during this call, as
/// parallel_runner.h specifies, and never unwind.
#[unsafe(no_mangle)] // SAFETY: The only definition of this resizable_parallel_runner.h symbol.
pub unsafe extern "C" fn JxlResizableParallelRunner(
    runner_opaque: *mut c_void,
    jpegxl_opaque: *mut c_void,
    init: JxlParallelRunInit,
    func: JxlParallelRunFunction,
    start_range: u32,
    end_range: u32,
) -> JxlParallelRetCode {
    // SAFETY: Forwards the caller's runner contract.
    unsafe {
        resizable::jxl_resizable_parallel_runner(
            runner_opaque,
            jpegxl_opaque,
            init,
            func,
            start_range,
            end_range,
        )
    }
}

/// JxlResizableParallelRunnerCreate. The source ignores the memory manager.
#[unsafe(no_mangle)] // SAFETY: The only definition of this resizable_parallel_runner.h symbol.
pub extern "C" fn JxlResizableParallelRunnerCreate(
    memory_manager: *const JxlMemoryManager,
) -> *mut c_void {
    resizable::jxl_resizable_parallel_runner_create(memory_manager)
}

/// JxlResizableParallelRunnerSetThreads.
///
/// # Safety
/// `runner_opaque` is a live runner from JxlResizableParallelRunnerCreate
/// that no other call uses concurrently.
#[unsafe(no_mangle)] // SAFETY: The only definition of this resizable_parallel_runner.h symbol.
pub unsafe extern "C" fn JxlResizableParallelRunnerSetThreads(
    runner_opaque: *mut c_void,
    num_threads: usize,
) {
    // SAFETY: Forwards the caller's runner contract.
    unsafe { resizable::jxl_resizable_parallel_runner_set_threads(runner_opaque, num_threads) }
}

/// JxlResizableParallelRunnerSuggestThreads.
#[unsafe(no_mangle)] // SAFETY: The only definition of this resizable_parallel_runner.h symbol.
pub extern "C" fn JxlResizableParallelRunnerSuggestThreads(xsize: u64, ysize: u64) -> u32 {
    resizable::jxl_resizable_parallel_runner_suggest_threads(xsize, ysize)
}

/// JxlResizableParallelRunnerDestroy.
///
/// # Safety
/// `runner_opaque` is null or a runner from JxlResizableParallelRunnerCreate
/// that no other call uses and that is not used again.
#[unsafe(no_mangle)] // SAFETY: The only definition of this resizable_parallel_runner.h symbol.
pub unsafe extern "C" fn JxlResizableParallelRunnerDestroy(runner_opaque: *mut c_void) {
    // SAFETY: Forwards the caller's runner contract.
    unsafe { resizable::jxl_resizable_parallel_runner_destroy(runner_opaque) }
}
