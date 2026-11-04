// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ThreadPool, RunCallState and RunOnPool from `lib/jxl/base/data_parallel.h`,
//! with the runner typedefs from `lib/include/jxl/parallel_runner.h`.
//!
//! Language-required deviations: C++ template functors become generic closure
//! parameters. The source stores `this` as the opaque value of a runner-less
//! pool; a Rust value can move, so `runner_opaque` derives that address when
//! it is read instead of retaining a constructor-time self pointer. Runner
//! callbacks receive monomorphized `extern "C"` trampolines in place of the
//! source's static member functions. No thread runner implementation or public
//! runner registration is part of this module.
use super::status::{Status, StatusCode};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

pub type JxlParallelRetCode = i32;
pub const JXL_PARALLEL_RET_SUCCESS: JxlParallelRetCode = 0;
pub const JXL_PARALLEL_RET_RUNNER_ERROR: JxlParallelRetCode = -1;
pub type JxlParallelRunInit = unsafe extern "C" fn(*mut c_void, usize) -> JxlParallelRetCode;
pub type JxlParallelRunFunction = unsafe extern "C" fn(*mut c_void, u32, usize);
pub type JxlParallelRunner = unsafe extern "C" fn(
    *mut c_void,
    *mut c_void,
    JxlParallelRunInit,
    JxlParallelRunFunction,
    u32,
    u32,
) -> JxlParallelRetCode;

pub struct ThreadPool {
    runner: Option<JxlParallelRunner>,
    runner_opaque: *mut c_void,
}

struct RunCallState<'a, I, D> {
    init_func: &'a I,
    data_func: &'a D,
    has_error: AtomicU32,
}

impl<I, D> RunCallState<'_, I, D>
where
    I: Fn(usize) -> Status + Sync,
    D: Fn(u32, usize) -> Status + Sync,
{
    /// JxlParallelRunInit interface.
    ///
    /// # Safety
    /// `jpegxl_opaque` is the live RunCallState passed to this Run call.
    unsafe extern "C" fn call_init_func(
        jpegxl_opaque: *mut c_void,
        num_threads: usize,
    ) -> JxlParallelRetCode {
        // SAFETY: The runner forwards the original call-state pointer, which
        // stays live for the complete Run call; only shared access is formed.
        let state = unsafe { &*jpegxl_opaque.cast::<Self>() };
        if (state.init_func)(num_threads).is_err() {
            state.has_error.store(1, Ordering::SeqCst);
            return JXL_PARALLEL_RET_RUNNER_ERROR;
        }
        JXL_PARALLEL_RET_SUCCESS
    }

    /// JxlParallelRunFunction interface.
    ///
    /// # Safety
    /// `jpegxl_opaque` is the live RunCallState passed to this Run call.
    unsafe extern "C" fn call_data_func(jpegxl_opaque: *mut c_void, value: u32, thread_id: usize) {
        // SAFETY: As in call_init_func; the closures are Sync for runner threads.
        let state = unsafe { &*jpegxl_opaque.cast::<Self>() };
        if state.has_error.load(Ordering::SeqCst) != 0 {
            return;
        }
        if (state.data_func)(value, thread_id).is_err() {
            state.has_error.store(1, Ordering::SeqCst);
        }
    }

    fn has_error(&self) -> bool {
        self.has_error.load(Ordering::SeqCst) != 0
    }
}

impl ThreadPool {
    pub fn new(runner: Option<JxlParallelRunner>, runner_opaque: *mut c_void) -> Self {
        Self {
            runner,
            runner_opaque,
        }
    }

    pub fn runner(&self) -> Option<JxlParallelRunner> {
        self.runner
    }

    pub fn runner_opaque(&self) -> *mut c_void {
        if self.runner.is_some() {
            self.runner_opaque
        } else {
            std::ptr::from_ref(self).cast_mut().cast()
        }
    }

    /// Runs init_func(num_threads) followed by data_func(task, thread) for
    /// every task in [begin, end). Not reentrant, as in the source.
    ///
    /// # Safety
    /// A present runner and its opaque value obey parallel_runner.h: they call
    /// the supplied init/data functions only with the supplied opaque value,
    /// only during this call, and never unwind.
    pub unsafe fn run<I, D>(&self, begin: u32, end: u32, init_func: &I, data_func: &D) -> Status
    where
        I: Fn(usize) -> Status + Sync,
        D: Fn(u32, usize) -> Status + Sync,
    {
        if begin > end {
            return Err(StatusCode::GenericError);
        }
        if begin == end {
            return Ok(());
        }
        let call_state = RunCallState {
            init_func,
            data_func,
            has_error: AtomicU32::new(0),
        };
        let jpegxl_opaque = std::ptr::from_ref(&call_state).cast_mut().cast::<c_void>();
        let Some(runner) = self.runner else {
            // SAFETY: The opaque value is the live local call state.
            if unsafe { RunCallState::<I, D>::call_init_func(jpegxl_opaque, 1) }
                != JXL_PARALLEL_RET_SUCCESS
            {
                return Err(StatusCode::GenericError);
            }
            for i in begin..end {
                // SAFETY: As above; tasks run sequentially on this thread.
                unsafe {
                    RunCallState::<I, D>::call_data_func(jpegxl_opaque, i, 0);
                }
            }
            if call_state.has_error() {
                return Err(StatusCode::GenericError);
            }
            return Ok(());
        };
        // SAFETY: The caller guarantees the runner protocol; the call state
        // outlives the runner call and is only accessed through shared loans.
        let ret = unsafe {
            runner(
                self.runner_opaque,
                jpegxl_opaque,
                RunCallState::<I, D>::call_init_func,
                RunCallState::<I, D>::call_data_func,
                begin,
                end,
            )
        };
        if ret != JXL_PARALLEL_RET_SUCCESS || call_state.has_error() {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
}

/// RunOnPool; a missing pool runs on a local runner-less ThreadPool.
///
/// # Safety
/// As for ThreadPool::run.
pub unsafe fn run_on_pool<I, D>(
    pool: Option<&ThreadPool>,
    begin: u32,
    end: u32,
    init_func: &I,
    data_func: &D,
) -> Status
where
    I: Fn(usize) -> Status + Sync,
    D: Fn(u32, usize) -> Status + Sync,
{
    match pool {
        None => {
            let default_pool = ThreadPool::new(None, std::ptr::null_mut());
            // SAFETY: A runner-less pool runs only the supplied local functions.
            unsafe { default_pool.run(begin, end, init_func, data_func) }
        }
        // SAFETY: Forward the caller's runner contract.
        Some(pool) => unsafe { pool.run(begin, end, init_func, data_func) },
    }
}

/// The ThreadPoolNoInit overload of RunOnPool.
///
/// # Safety
/// As for ThreadPool::run.
pub unsafe fn run_on_pool_no_init<D>(
    pool: Option<&ThreadPool>,
    begin: u32,
    end: u32,
    data_func: &D,
) -> Status
where
    D: Fn(u32, usize) -> Status + Sync,
{
    let init_func = |_num_threads: usize| -> Status { Ok(()) };
    // SAFETY: Forward the caller's runner contract.
    unsafe { run_on_pool(pool, begin, end, &init_func, data_func) }
}
