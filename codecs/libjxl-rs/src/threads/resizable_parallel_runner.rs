// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ResizeableParallelRunner and the public functions of
//! `lib/threads/resizable_parallel_runner.cc` (declared in
//! `lib/include/jxl/resizable_parallel_runner.h`).
//!
//! A thread pool that allows changing the number of threads it runs. It also
//! runs tasks on the calling thread, which can work better on schedulers for
//! heterogeneous architectures.
//!
//! Language-required deviations:
//! - std::mutex and the variables it protects become one `Mutex`. Lock
//!   poisoning cannot occur, since an unwinding runner callback aborts; a
//!   poisoned lock would be entered as the source does.
//! - The fields that only the main thread's write is protected for (`func_`,
//!   `jxl_opaque_`, `end_task_`) and the worker vector are interior-mutable
//!   cells; the safety comments state the happens-before edge of each access.
//! - Workers keep a raw pointer to the boxed runner, like the source's
//!   captured `this`. The destructor body (SetNumThreads(0)) runs through a
//!   shared reference before the box is released, because workers may still
//!   hold shared references until it joins them.
//! - The functions keep their C signatures as Rust functions; the C ABI
//!   exports are a separate boundary.
use super::hardware_concurrency;
use crate::jxl::base::data_parallel::{
    JxlParallelRetCode, JxlParallelRunFunction, JxlParallelRunInit,
};
use crate::jxl::memory_manager::JxlMemoryManager;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;

/// The variables protected by the source's `state_mutex_`.
///
/// - work_available is set to true after a call to Run() and to false at the
///   end of it.
/// - num_desired_workers represents the number of workers that should be
///   present.
/// - max_running_workers represents the number of workers that should be
///   executing tasks.
/// - num_running_workers represents the number of workers that are executing
///   tasks.
struct State {
    num_desired_workers: usize,
    max_running_workers: usize,
    num_running_workers: usize,
    work_available: bool,
}

/// The runner pointer handed to each worker thread.
struct RunnerPointer(*const ResizeableParallelRunner);
// SAFETY: The pointee is Sync (see below) and stays boxed at this address
// until SetNumThreads has joined the thread that holds this pointer.
unsafe impl Send for RunnerPointer {}
impl RunnerPointer {
    /// A method, so that a closure captures the whole Send wrapper rather
    /// than its raw pointer field.
    fn get(&self) -> *const ResizeableParallelRunner {
        self.0
    }
}

struct ResizeableParallelRunner {
    // Checks when the worker has something to do, which can be one of:
    // - quitting (when worker_id >= num_desired_workers)
    // - having work available for them (work_available is true and worker_id >=
    // max_running_workers)
    workers_can_proceed: Condvar,

    // Workers are done, and the main thread can proceed (num_running_workers ==
    // 0)
    work_done: Condvar,

    workers: UnsafeCell<Vec<JoinHandle<()>>>,

    // Protects all the remaining variables, except for func, jxl_opaque and
    // end_task (for which only the write by the main thread is protected, and
    // subsequent uses by workers happen-after it) and next_task (which is
    // atomic).
    state_mutex: Mutex<State>,

    // Range of tasks still need to be done.
    next_task: AtomicU32,
    end_task: UnsafeCell<u32>,

    // Function to run and its argument.
    func: UnsafeCell<Option<JxlParallelRunFunction>>,
    jxl_opaque: UnsafeCell<*mut c_void>, // not owned
}

// SAFETY: Workers share the runner. The worker vector belongs to the main
// thread; func, jxl_opaque and end_task are written under the state mutex
// while no thread is counted in num_running_workers (the previous Run waited
// for zero), and every reader is counted there, from a later critical section
// until after its last read. All other fields are atomics or std::sync types.
unsafe impl Sync for ResizeableParallelRunner {}

impl ResizeableParallelRunner {
    fn new() -> Self {
        Self {
            workers_can_proceed: Condvar::new(),
            work_done: Condvar::new(),
            workers: UnsafeCell::new(Vec::new()),
            state_mutex: Mutex::new(State {
                num_desired_workers: 0,
                max_running_workers: 0,
                num_running_workers: 0,
                work_available: false,
            }),
            next_task: AtomicU32::new(0),
            end_task: UnsafeCell::new(0),
            func: UnsafeCell::new(None),
            jxl_opaque: UnsafeCell::new(std::ptr::null_mut()),
        }
    }

    fn lock(&self) -> MutexGuard<'_, State> {
        self.state_mutex
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// # Safety
    /// `self` is boxed at a stable address, and no other SetNumThreads, Run
    /// or destruction is concurrent with this call.
    #[expect(
        clippy::implicit_saturating_sub,
        reason = "Mirror the source's guarded decrement."
    )]
    unsafe fn set_num_threads(&self, mut num: usize) {
        if num > 0 {
            num -= 1;
        }
        {
            let mut l = self.lock();
            l.num_desired_workers = num;
            self.workers_can_proceed.notify_all();
        }
        // SAFETY: Only the calling (main) thread accesses the worker vector,
        // and the caller excludes concurrent main-thread calls.
        let workers = unsafe { &mut *self.workers.get() };
        if workers.len() < num {
            for i in workers.len()..num {
                let pointer = RunnerPointer(std::ptr::from_ref(self));
                workers.push(std::thread::spawn(move || {
                    // SAFETY: The boxed runner outlives this worker, which
                    // is joined before the box is released.
                    unsafe { &*pointer.get() }.worker_body(i);
                }));
            }
        }
        if workers.len() > num {
            for worker in workers.drain(num..) {
                // A worker never unwinds: runner callbacks abort on panic.
                let _ = worker.join();
            }
        }
    }

    /// # Safety
    /// As `jxl_resizable_parallel_runner`.
    unsafe fn run(
        &self,
        jxl_opaque: *mut c_void,
        init: JxlParallelRunInit,
        func: JxlParallelRunFunction,
        start: u32,
        end: u32,
    ) -> JxlParallelRetCode {
        if start.wrapping_add(1) == end {
            // SAFETY: The caller's init contract.
            let ret = unsafe { init(jxl_opaque, 1) };
            if ret != 0 {
                return ret;
            }

            // SAFETY: The caller's data-function contract.
            unsafe { func(jxl_opaque, start, 0) };
            return ret;
        }

        // SAFETY: Only the calling (main) thread accesses the worker vector.
        let num_workers = unsafe { &*self.workers.get() }.len();
        let num_workers = (num_workers + 1).min(end.wrapping_sub(start) as usize);
        // SAFETY: The caller's init contract.
        let ret = unsafe { init(jxl_opaque, num_workers) };
        if ret != 0 {
            return ret;
        }

        {
            let mut l = self.lock();
            // Avoid waking up more workers than needed.
            l.max_running_workers = end.wrapping_sub(start).wrapping_sub(1) as usize;
            self.next_task.store(start, Ordering::SeqCst);
            // SAFETY: Under the state mutex, while no thread is counted in
            // num_running_workers; see the Sync implementation.
            unsafe {
                *self.end_task.get() = end;
                *self.func.get() = Some(func);
                *self.jxl_opaque.get() = jxl_opaque;
            }
            l.work_available = true;
            l.num_running_workers = l.num_running_workers.wrapping_add(1);
            self.workers_can_proceed.notify_all();
        }

        self.dequeue_tasks(0);

        loop {
            let l = self.lock();
            if l.num_running_workers == 0 {
                break;
            }
            drop(
                self.work_done
                    .wait(l)
                    .unwrap_or_else(PoisonError::into_inner),
            );
        }

        ret
    }

    fn worker_body(&self, worker_id: usize) {
        loop {
            {
                let mut l = self.lock();
                // Worker pool was reduced, resize down.
                if worker_id >= l.num_desired_workers {
                    return;
                }
                // Nothing to do this time.
                if !l.work_available || worker_id >= l.max_running_workers {
                    drop(
                        self.workers_can_proceed
                            .wait(l)
                            .unwrap_or_else(PoisonError::into_inner),
                    );
                    continue;
                }
                l.num_running_workers = l.num_running_workers.wrapping_add(1);
            }
            self.dequeue_tasks(worker_id + 1);
        }
    }

    fn dequeue_tasks(&self, thread_id: usize) {
        loop {
            let task = self.next_task.fetch_add(1, Ordering::SeqCst);
            // SAFETY: This thread is counted in num_running_workers since a
            // critical section after the write; see the Sync implementation.
            if task >= unsafe { *self.end_task.get() } {
                let mut l = self.lock();
                l.num_running_workers = l.num_running_workers.wrapping_sub(1);
                l.work_available = false;
                if l.num_running_workers == 0 {
                    self.work_done.notify_all();
                }
                break;
            }
            // SAFETY: As for end_task above.
            let (func, opaque) = unsafe { (*self.func.get(), *self.jxl_opaque.get()) };
            let func = func.expect("Run stores its data function before work is available");
            // SAFETY: The blocked Run call's caller guarantees the data
            // function contract for this opaque value on any runner thread.
            unsafe { func(opaque, task, thread_id) };
        }
    }
}

/// JxlResizableParallelRunner: the parallel runner callback of a runner from
/// `jxl_resizable_parallel_runner_create`.
///
/// # Safety
/// `runner_opaque` is a live runner from `jxl_resizable_parallel_runner_create`
/// that no other call uses concurrently. `init` and `func` may be called with
/// `jpegxl_opaque` from any of its threads during this call, as
/// parallel_runner.h specifies.
pub unsafe extern "C" fn jxl_resizable_parallel_runner(
    runner_opaque: *mut c_void,
    jpegxl_opaque: *mut c_void,
    init: JxlParallelRunInit,
    func: JxlParallelRunFunction,
    start_range: u32,
    end_range: u32,
) -> JxlParallelRetCode {
    // SAFETY: The caller passes a live runner and the runner contract.
    unsafe {
        (*runner_opaque
            .cast_const()
            .cast::<ResizeableParallelRunner>())
        .run(jpegxl_opaque, init, func, start_range, end_range)
    }
}

/// JxlResizableParallelRunnerCreate. The source ignores the memory manager
/// and allocates with `new`; the Rust global allocator takes its place.
pub fn jxl_resizable_parallel_runner_create(
    _memory_manager: *const JxlMemoryManager,
) -> *mut c_void {
    Box::into_raw(Box::new(ResizeableParallelRunner::new())).cast()
}

/// JxlResizableParallelRunnerSetThreads.
///
/// # Safety
/// `runner_opaque` is a live runner from `jxl_resizable_parallel_runner_create`
/// that no other call uses concurrently.
pub unsafe fn jxl_resizable_parallel_runner_set_threads(
    runner_opaque: *mut c_void,
    num_threads: usize,
) {
    // SAFETY: The caller passes a live, boxed runner and excludes concurrent
    // calls.
    unsafe {
        (*runner_opaque
            .cast_const()
            .cast::<ResizeableParallelRunner>())
        .set_num_threads(num_threads);
    }
}

/// JxlResizableParallelRunnerDestroy.
///
/// # Safety
/// `runner_opaque` is null or a runner from
/// `jxl_resizable_parallel_runner_create` that no other call uses and that is
/// not used again.
pub unsafe fn jxl_resizable_parallel_runner_destroy(runner_opaque: *mut c_void) {
    let runner = runner_opaque.cast::<ResizeableParallelRunner>();
    if runner.is_null() {
        return;
    }
    // SAFETY: The caller transfers the live, boxed runner. The destructor body
    // joins every worker through a shared reference; afterwards no reference
    // to the runner lives and the box is released once.
    unsafe {
        (*runner).set_num_threads(0);
        drop(Box::from_raw(runner));
    }
}

/// JxlResizableParallelRunnerSuggestThreads: ~one thread per group.
pub fn jxl_resizable_parallel_runner_suggest_threads(xsize: u64, ysize: u64) -> u32 {
    u64::from(hardware_concurrency()).min(xsize.wrapping_mul(ysize) / (256 * 256)) as u32
}
