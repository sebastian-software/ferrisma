// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ThreadParallelRunner from `lib/threads/thread_parallel_runner_internal.h`
//! and `lib/threads/thread_parallel_runner_internal.cc`.
//!
//! An implementation using std::thread of a JxlParallelRunner. The number of
//! threads is fixed at construction time and the threads are re-used for every
//! `runner` call. Only one concurrent `runner` call per instance is allowed at
//! a time. Workers reserve guided chunks of the task range with an atomic
//! counter, so millions of tasks need no per-task system call.
//!
//! Language-required deviations:
//! - The object is constructed and destroyed in place (`construct`/`destruct`)
//!   as the source's placement new and explicit destructor call. Workers keep a
//!   raw pointer to it, like the source's `this`; it does not move until the
//!   destructor has joined them.
//! - std::mutex and the variables it guards (`workers_ready_` and
//!   `worker_start_command_`) become one `Mutex`. Lock poisoning has no source
//!   counterpart and cannot occur, since an unwinding runner callback aborts;
//!   a poisoned lock would be entered as the source does.
//! - The fields that the main thread writes and workers read after the mutex
//!   hand-off (`data_func_`, `jpegxl_opaque_`), the thread vector and the
//!   public memory manager are interior-mutable cells, so that workers may
//!   hold a shared reference to the object while the main thread writes them.
//!   Their accesses keep the source's order; the safety comments state the
//!   happens-before edge of each.
//! - `#[repr(C)]` keeps the declaration order, so the two 64-byte paddings
//!   still separate the workers' atomic counter from the other fields.
//! - Thread ids are the source's `int`, converted where the source converts.
//! - A mutex-guarded command epoch makes each worker consume a command once.
//!   The pinned source checks only the command value after waking; a second
//!   wake before the readiness barrier clears that value can execute the same
//!   command twice and overshoot the barrier's exact ready count.
use crate::jxl::base::data_parallel::{
    JXL_PARALLEL_RET_RUNNER_ERROR, JXL_PARALLEL_RET_SUCCESS, JxlParallelRetCode,
    JxlParallelRunFunction, JxlParallelRunInit,
};
use crate::jxl::memory_manager::JxlMemoryManager;
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;

/// Failure mode returned by fallible runner construction and lazy worker
/// startup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunnerError {
    /// The supplied `JxlMemoryManager` set only one of `alloc`/`free`.
    InvalidMemoryManager,
    /// Runner or worker-table allocation failed.
    Allocation,
    /// The operating system refused to start a worker thread.
    ThreadSpawn,
}

impl RunnerError {
    fn to_u8(self) -> u8 {
        match self {
            Self::InvalidMemoryManager => 1,
            Self::Allocation => 2,
            Self::ThreadSpawn => 3,
        }
    }

    fn from_u8(code: u8) -> Option<Self> {
        match code {
            1 => Some(Self::InvalidMemoryManager),
            2 => Some(Self::Allocation),
            3 => Some(Self::ThreadSpawn),
            _ => None,
        }
    }
}

// After construction and between calls to Run, workers are "ready", i.e.
// waiting on worker_start_cv. They are "started" by sending a "command" and
// notifying all worker_start_cv waiters. (That is why all workers must be
// ready/waiting - otherwise, the notification will not reach all of them and
// the main thread waits in vain for them to report readiness.)
type WorkerCommand = u64;

// Special values; all others encode the begin/end parameters. Note that all
// these are no-op ranges (begin >= end) and therefore never used to encode
// ranges.
const WORKER_WAIT: WorkerCommand = !1;
const WORKER_ONCE: WorkerCommand = !2;
const WORKER_EXIT: WorkerCommand = !3;

/// The variables guarded by the source's `mutex_`.
struct Guarded {
    workers_ready: u32,
    ready_target: u32,
    worker_start_command: WorkerCommand,
    command_epoch: u64,
}

/// The runner pointer handed to each worker thread.
struct RunnerPointer(*const ThreadParallelRunner);
// SAFETY: The pointee is Sync (see below) and stays at this address until the
// destructor has joined the thread that holds this pointer.
unsafe impl Send for RunnerPointer {}
impl RunnerPointer {
    /// A method, so that a closure captures the whole Send wrapper rather
    /// than its raw pointer field.
    fn get(&self) -> *const ThreadParallelRunner {
        self.0
    }
}

/// Main helper class implementing the JxlParallelRunner interface.
#[repr(C)]
pub struct ThreadParallelRunner {
    // Serializes worker-pool creation/teardown, `set_max_workers`, and worker
    // dispatch (`run_on_each_thread` / `runner`). Worker threads never acquire
    // this lock (strict lock order: `threads` -> `mutex`), so no deadlock with
    // workers is possible.
    threads: Mutex<Vec<JoinHandle<()>>>,

    num_worker_threads: AtomicU32, // active maximum worker count (<= construction_max_workers)
    construction_max_workers: usize, // immutable cap from construct/construct_lazy
    num_threads: u32,
    lazy: bool,

    depth: AtomicU32,            // detects if Run is re-entered (not supported).
    active_workers: AtomicU32,   // currently live worker threads (`threads.len()`).
    dispatch_workers: AtomicU32, // workers participating in the active `runner` call (<= init count).
    last_error: AtomicU8,        // RunnerError recorded during lazy worker startup.

    mutex: Mutex<Guarded>, // guards both cv and their variables.
    workers_ready_cv: Condvar,
    worker_start_cv: Condvar,

    // Written by caller thread while holding `threads` with all workers idle;
    // read by workers after `start_workers`'s `mutex` release.
    data_func: UnsafeCell<Option<JxlParallelRunFunction>>,
    jpegxl_opaque: UnsafeCell<*mut c_void>,

    // Updated by workers; padding avoids false sharing.
    padding1: [u8; 64],
    num_reserved: AtomicU32,
    padding2: [u8; 64],

    memory_manager: UnsafeCell<JxlMemoryManager>,
}

// SAFETY: Workers and callers share the object. Every field is either
// immutable after construction, an atomic, a std::sync primitive, or an
// UnsafeCell whose accesses are synchronized: `data_func` and `jpegxl_opaque`
// are only written while holding the `threads` mutex with every worker idle at
// `worker_start_cv`, and workers read them only after `start_workers` releases
// `mutex`; `memory_manager` is only accessed through `unsafe` methods whose
// callers exclude concurrent writes.
unsafe impl Sync for ThreadParallelRunner {}

impl ThreadParallelRunner {
    /// Initializes storage in place without spawning workers yet.
    ///
    /// # Safety
    /// `this` is writable, aligned, uninitialized storage.
    unsafe fn write_initial(this: *mut Self, num_worker_threads: i32, lazy: bool) {
        let clamped = num_worker_threads.max(0) as u32;
        // SAFETY: The caller supplies writable, aligned, unused storage.
        unsafe {
            this.write(Self {
                threads: Mutex::new(Vec::new()),
                num_worker_threads: AtomicU32::new(clamped),
                construction_max_workers: clamped as usize,
                num_threads: clamped.max(1),
                lazy,
                depth: AtomicU32::new(0),
                active_workers: AtomicU32::new(0),
                dispatch_workers: AtomicU32::new(0),
                last_error: AtomicU8::new(0),
                // Safely handle spurious worker wakeups.
                mutex: Mutex::new(Guarded {
                    workers_ready: 0,
                    ready_target: 0,
                    worker_start_command: WORKER_WAIT,
                    command_epoch: 0,
                }),
                workers_ready_cv: Condvar::new(),
                worker_start_cv: Condvar::new(),
                data_func: UnsafeCell::new(None),
                jpegxl_opaque: UnsafeCell::new(std::ptr::null_mut()),
                padding1: [0; 64],
                num_reserved: AtomicU32::new(0),
                padding2: [0; 64],
                memory_manager: UnsafeCell::new(JxlMemoryManager::default()),
            });
        }
        // SAFETY: Initialized above; suppress "unused-private-field" warning.
        let runner = unsafe { &*this };
        let _ = (&runner.padding1, &runner.padding2);
    }

    /// Starts the given number of worker threads and blocks until they are
    /// ready. If zero, all tasks run on the main thread. Returns an explicit
    /// [`RunnerError`] if worker allocation or OS thread creation fails; any
    /// workers already started are signalled to exit, joined, and the in-place
    /// runner is dropped before returning `Err`.
    ///
    /// # Safety
    /// `this` is writable storage, aligned for the runner, that holds no live
    /// object. On `Ok(())`, the runner stays at this address until `destruct`;
    /// on `Err(_)`, no live object remains in `this`.
    pub unsafe fn construct(this: *mut Self, num_worker_threads: i32) -> Result<(), RunnerError> {
        // SAFETY: Forward caller's placement-storage contract with the OS
        // fallible thread builder.
        unsafe {
            Self::construct_with_spawner(this, num_worker_threads, |work| {
                std::thread::Builder::new().spawn(work)
            })
        }
    }

    /// Initializes a lazy runner with at most `max_worker_threads` workers.
    /// Zero worker threads are spawned at construction time; single-task
    /// ranges run directly on the calling thread, and multi-task ranges grow
    /// the pool on demand up to `min(max_worker_threads, end - begin)`.
    ///
    /// # Safety
    /// `this` is writable storage, aligned for the runner, that holds no live
    /// object, and stays at this address until `destruct`.
    pub unsafe fn construct_lazy(this: *mut Self, max_worker_threads: i32) {
        // SAFETY: The caller supplies writable, aligned, unused storage.
        unsafe { Self::write_initial(this, max_worker_threads, true) };
    }

    /// Fallible eager worker-pool constructor parameterized by the thread
    /// spawner so tests can inject partial OS thread-creation failures without
    /// global state.
    ///
    /// # Safety
    /// Same as [`Self::construct`].
    unsafe fn construct_with_spawner<S>(
        this: *mut Self,
        num_worker_threads: i32,
        spawn: S,
    ) -> Result<(), RunnerError>
    where
        S: FnMut(Box<dyn FnOnce() + Send>) -> std::io::Result<JoinHandle<()>>,
    {
        // SAFETY: The caller supplies writable, aligned, unused storage.
        unsafe { Self::write_initial(this, num_worker_threads, false) };
        // SAFETY: Initialized above; workers only form shared references.
        let runner = unsafe { &*this };
        let target_workers = runner.max_worker_threads();
        let res = {
            let mut threads = runner.lock_threads();
            runner.ensure_workers_locked_with_spawner(&mut threads, target_workers, spawn)
        };
        if let Err(err) = res {
            // SAFETY: `ensure_workers_locked_with_spawner` has joined every
            // started worker and the `threads` guard was released above; drop
            // the in-place runner before signalling failure.
            unsafe { std::ptr::drop_in_place(this) };
            return Err(err);
        }
        Ok(())
    }

    fn lock_threads(&self) -> MutexGuard<'_, Vec<JoinHandle<()>>> {
        self.threads.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Ensures at least `target_workers` (clamped to `self.num_worker_threads`)
    /// live worker threads exist and are ready at the wait barrier. Caller
    /// holds `self.threads`.
    fn ensure_workers_locked(
        &self,
        threads: &mut Vec<JoinHandle<()>>,
        target_workers: usize,
    ) -> Result<usize, RunnerError> {
        self.ensure_workers_locked_with_spawner(threads, target_workers, |work| {
            std::thread::Builder::new().spawn(work)
        })
    }

    /// Fallible incremental worker spawner parameterized by `spawn` for tests.
    /// Caller holds `self.threads` with all existing workers idle at the
    /// barrier.
    fn ensure_workers_locked_with_spawner<S>(
        &self,
        threads: &mut Vec<JoinHandle<()>>,
        target_workers: usize,
        mut spawn: S,
    ) -> Result<usize, RunnerError>
    where
        S: FnMut(Box<dyn FnOnce() + Send>) -> std::io::Result<JoinHandle<()>>,
    {
        let target = target_workers.min(self.construction_max_workers);
        let prev = threads.len();
        if target <= prev {
            return Ok(prev);
        }
        let delta = target - prev;
        if threads.try_reserve(delta).is_err() {
            return Err(RunnerError::Allocation);
        }
        let start_epoch = {
            let mut lock = self.lock();
            debug_assert_eq!(lock.workers_ready, 0);
            lock.ready_target = delta as u32;
            lock.command_epoch
        };
        for i in prev..target {
            let pointer = RunnerPointer(std::ptr::from_ref(self));
            let handle = spawn(Box::new(move || {
                Self::thread_func(pointer.get(), i as i32, start_epoch);
            }));
            match handle {
                Ok(thread) => threads.push(thread),
                Err(_) => {
                    if !threads.is_empty() {
                        self.start_workers(WORKER_EXIT);
                        let started = std::mem::take(threads);
                        for thread in started {
                            let _ = thread.join();
                        }
                    }
                    self.active_workers.store(0, Ordering::Release);
                    let mut lock = self.lock();
                    lock.workers_ready = 0;
                    lock.ready_target = 0;
                    lock.worker_start_command = WORKER_WAIT;
                    return Err(RunnerError::ThreadSpawn);
                }
            }
        }
        self.wait_for_workers_ready(delta as u32, target as u32);
        self.active_workers.store(target as u32, Ordering::Release);
        Ok(target)
    }

    /// Waits for all threads to exit and drops the object in place.
    ///
    /// # Safety
    /// `this` is a runner from `construct` or `construct_lazy` that no
    /// `runner` call uses; it is not used afterwards.
    pub unsafe fn destruct(this: *mut Self) {
        // SAFETY: The caller transfers the live runner; workers hold only
        // shared references until they are joined below.
        let runner = unsafe { &*this };
        let threads = std::mem::take(&mut *runner.lock_threads());
        if !threads.is_empty() {
            runner.start_workers(WORKER_EXIT);
            for thread in threads {
                // A worker never unwinds: runner callbacks abort on panic. A
                // join error should not ever happen, as in the source.
                let _ = thread.join();
            }
        }
        runner.active_workers.store(0, Ordering::Release);
        // SAFETY: Every worker has exited; no reference to the object lives.
        unsafe { std::ptr::drop_in_place(this) };
    }

    /// The public `memory_manager` member.
    ///
    /// # Safety
    /// No `memory_manager_mut` access is live.
    pub unsafe fn memory_manager(&self) -> JxlMemoryManager {
        // SAFETY: Workers never access this member; the caller excludes a
        // concurrent write.
        unsafe { *self.memory_manager.get() }
    }

    /// Assigns the public `memory_manager` member.
    ///
    /// # Safety
    /// No other access to the member is concurrent.
    pub unsafe fn set_memory_manager(&self, memory_manager: JxlMemoryManager) {
        // SAFETY: Workers never access this member; the caller excludes
        // concurrent accesses.
        unsafe { *self.memory_manager.get() = memory_manager };
    }

    /// Returns maximum number of main/worker threads that may call Func.
    /// Useful for allocating per-thread storage.
    pub fn num_threads(&self) -> usize {
        self.num_threads as usize
    }

    /// Returns the configured maximum number of worker threads.
    pub fn max_worker_threads(&self) -> usize {
        self.num_worker_threads.load(Ordering::Acquire) as usize
    }

    /// Updates the maximum worker count of a lazy runner (for example, once a
    /// frame's actual group count is known). Lock-free and clamped to
    /// `[0, construction_max_workers]`, so callbacks may call it without
    /// deadlocking and worker thread IDs never exceed `num_threads()`. A cap
    /// of `0` runs subsequent lazy dispatches on the calling thread even if
    /// workers were already spawned for an earlier frame. Ignored for eager
    /// runners.
    pub fn set_max_workers(&self, max_workers: usize) {
        if !self.lazy {
            return;
        }
        let clamped = max_workers.min(self.construction_max_workers);
        self.num_worker_threads
            .store(clamped as u32, Ordering::Release);
    }

    /// Returns the number of live worker threads currently spawned.
    pub fn spawned_threads(&self) -> usize {
        self.active_workers.load(Ordering::Acquire) as usize
    }

    /// Returns the failure reason of the last lazy worker startup, if any.
    pub fn last_error(&self) -> Option<RunnerError> {
        RunnerError::from_u8(self.last_error.load(Ordering::Acquire))
    }

    fn record_last_error(&self, err: RunnerError) {
        self.last_error.store(err.to_u8(), Ordering::Release);
    }

    fn lock(&self) -> MutexGuard<'_, Guarded> {
        self.mutex.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Calls f(task, thread). Used for type erasure of Func arguments. The
    /// signature must match JxlParallelRunFunction.
    ///
    /// # Safety
    /// `f` is the live closure that `run_on_each_thread` passed.
    unsafe extern "C" fn call_closure<F: Fn(u32, usize) + Sync>(
        f: *mut c_void,
        task: u32,
        thread: usize,
    ) {
        // SAFETY: The caller forwards the closure of the blocked
        // run_on_each_thread call; only shared access is formed.
        let f = unsafe { &*f.cast_const().cast::<F>() };
        f(task, thread);
    }

    /// Runs func(thread, thread) on all thread(s) that may participate in Run.
    /// If NumThreads() == 0, runs on the main thread with thread == 0,
    /// otherwise concurrently called by each worker thread in
    /// [0, NumThreads()).
    ///
    /// Returns `Err(RunnerError)` if lazy worker allocation or OS thread
    /// creation fails; in that case no callback is invoked.
    pub fn run_on_each_thread<F: Fn(u32, usize) + Sync>(
        &self,
        func: &F,
    ) -> Result<(), RunnerError> {
        self.run_on_each_thread_with_spawner(func, |work| std::thread::Builder::new().spawn(work))
    }

    fn run_on_each_thread_with_spawner<F, S>(&self, func: &F, spawn: S) -> Result<(), RunnerError>
    where
        F: Fn(u32, usize) + Sync,
        S: FnMut(Box<dyn FnOnce() + Send>) -> std::io::Result<JoinHandle<()>>,
    {
        let mut threads = self.lock_threads();
        let max_workers = self.max_worker_threads();
        if max_workers == 0 {
            drop(threads);
            let thread = 0;
            func(thread as u32, thread as usize);
            return Ok(());
        }
        if self.lazy
            && let Err(err) =
                self.ensure_workers_locked_with_spawner(&mut threads, max_workers, spawn)
        {
            self.record_last_error(err);
            return Err(err);
        }
        let participating = if self.lazy {
            max_workers
        } else {
            threads.len()
        };
        let worker_count = threads.len() as u32;

        // SAFETY: Holding `threads` serializes concurrent callers and every
        // worker is ready and waiting, so none reads these cells until after
        // `start_workers`'s mutex release below.
        unsafe {
            *self.data_func.get() = Some(Self::call_closure::<F>);
            *self.jpegxl_opaque.get() = std::ptr::from_ref(func).cast_mut().cast();
        }
        self.dispatch_workers
            .store(participating as u32, Ordering::Relaxed);
        self.start_workers(WORKER_ONCE);
        self.wait_for_workers_ready(worker_count, worker_count);
        Ok(())
    }

    fn wait_for_workers_ready(&self, expected_ready: u32, next_ready_target: u32) {
        let mut lock = self.lock();
        // Typically only a single iteration.
        while lock.workers_ready != expected_ready {
            lock = self
                .workers_ready_cv
                .wait(lock)
                .unwrap_or_else(PoisonError::into_inner);
        }
        lock.workers_ready = 0;
        lock.ready_target = next_ready_target;

        // Safely handle spurious worker wakeups.
        lock.worker_start_command = WORKER_WAIT;
    }

    // Precondition: all workers are ready.
    fn start_workers(&self, worker_command: WorkerCommand) {
        let mut lock = self.lock();
        lock.worker_start_command = worker_command;
        // Every command has a distinct epoch, even if its encoded range is
        // identical to the previous one. Workers consume each epoch once.
        lock.command_epoch = lock.command_epoch.wrapping_add(1);
        // Workers will need this lock, so release it before they wake up.
        drop(lock);
        self.worker_start_cv.notify_all();
    }

    /// JxlParallelRunner interface.
    ///
    /// # Safety
    /// `runner_opaque` is a live runner from `construct` or `construct_lazy`.
    /// `init` and `func` may be called with `jpegxl_opaque` from any of its
    /// threads during this call, as parallel_runner.h specifies.
    pub unsafe extern "C" fn runner(
        runner_opaque: *mut c_void,
        jpegxl_opaque: *mut c_void,
        init: JxlParallelRunInit,
        func: JxlParallelRunFunction,
        start_range: u32,
        end_range: u32,
    ) -> JxlParallelRetCode {
        // SAFETY: The caller passes a live runner; only shared access.
        let this = unsafe { &*runner_opaque.cast_const().cast::<Self>() };
        if start_range > end_range {
            return JXL_PARALLEL_RET_RUNNER_ERROR;
        }
        if start_range == end_range {
            return JXL_PARALLEL_RET_SUCCESS;
        }

        // Use a sequential run when the runner was constructed with zero
        // workers (no worker threads can ever exist).
        if this.construction_max_workers == 0 {
            // SAFETY: The caller's init contract.
            let ret = unsafe { init(jpegxl_opaque, 1) };
            if ret != JXL_PARALLEL_RET_SUCCESS {
                return ret;
            }
            let thread: usize = 0;
            for task in start_range..end_range {
                // SAFETY: The caller's data-function contract.
                unsafe { func(jpegxl_opaque, task, thread) };
            }
            return JXL_PARALLEL_RET_SUCCESS;
        }

        // Reject re-entry before touching `self.threads` so a worker calling
        // `runner` re-entrantly fails immediately instead of deadlocking.
        if this.depth.fetch_add(1, Ordering::AcqRel) != 0 {
            return JXL_PARALLEL_RET_RUNNER_ERROR; // Must not re-enter.
        }

        let num_tasks = end_range - start_range;
        if this.lazy && num_tasks == 1 {
            // Single-task range on a lazy runner: run directly on the calling
            // thread without spawning or waking any worker threads.
            // SAFETY: The caller's init contract.
            let ret = unsafe { init(jpegxl_opaque, 1) };
            if ret != JXL_PARALLEL_RET_SUCCESS {
                let _ = this.depth.fetch_sub(1, Ordering::AcqRel);
                return ret;
            }
            // SAFETY: The caller's data-function contract.
            unsafe { func(jpegxl_opaque, start_range, 0) };
            if this.depth.fetch_sub(1, Ordering::AcqRel) != 1 {
                return JXL_PARALLEL_RET_RUNNER_ERROR;
            }
            return JXL_PARALLEL_RET_SUCCESS;
        }

        let worker_command =
            (WorkerCommand::from(start_range) << 32).wrapping_add(WorkerCommand::from(end_range));
        // Ensure the inputs do not result in a reserved command.
        if worker_command == WORKER_WAIT
            || worker_command == WORKER_ONCE
            || worker_command == WORKER_EXIT
        {
            let _ = this.depth.fetch_sub(1, Ordering::AcqRel);
            return JXL_PARALLEL_RET_RUNNER_ERROR;
        }

        let active_workers = {
            let mut threads = this.lock_threads();
            let max_workers = this.max_worker_threads();
            if max_workers == 0 {
                // Lazy runner capped at 0 workers for a single-group frame: run
                // sequentially on the calling thread without spawning workers.
                drop(threads);
                // SAFETY: The caller's init contract.
                let ret = unsafe { init(jpegxl_opaque, 1) };
                if ret != JXL_PARALLEL_RET_SUCCESS {
                    let _ = this.depth.fetch_sub(1, Ordering::AcqRel);
                    return ret;
                }
                for task in start_range..end_range {
                    // SAFETY: The caller's data-function contract.
                    unsafe { func(jpegxl_opaque, task, 0) };
                }
                if this.depth.fetch_sub(1, Ordering::AcqRel) != 1 {
                    return JXL_PARALLEL_RET_RUNNER_ERROR;
                }
                return JXL_PARALLEL_RET_SUCCESS;
            }

            if this.lazy {
                let target = max_workers.min(num_tasks as usize);
                if let Err(err) = this.ensure_workers_locked(&mut threads, target) {
                    this.record_last_error(err);
                    drop(threads);
                    let _ = this.depth.fetch_sub(1, Ordering::AcqRel);
                    return JXL_PARALLEL_RET_RUNNER_ERROR;
                }
                target
            } else {
                threads.len()
            }
        };

        // Call `init` without holding `self.threads` so an `init` callback can
        // invoke `run_on_each_thread` to initialize per-worker state without
        // deadlocking. `depth` remains elevated to reject re-entrant `runner`
        // calls immediately.
        // SAFETY: The caller's init contract.
        let ret = unsafe { init(jpegxl_opaque, active_workers.max(1)) };
        if ret != JXL_PARALLEL_RET_SUCCESS {
            let _ = this.depth.fetch_sub(1, Ordering::AcqRel);
            return ret;
        }

        let threads = this.lock_threads();
        if threads.len() < active_workers || threads.is_empty() {
            // A concurrent `run_on_each_thread` failed lazy growth and tore
            // down the pool while `init` was running; never report success
            // without running tasks.
            drop(threads);
            let _ = this.depth.fetch_sub(1, Ordering::AcqRel);
            return JXL_PARALLEL_RET_RUNNER_ERROR;
        }
        let worker_count = threads.len() as u32;
        // SAFETY: Holding `threads` serializes concurrent callers and every
        // worker is ready and waiting (constructor, lazy startup, or previous
        // barrier), so none reads these cells until after `start_workers`'s
        // mutex release below. Re-entry was rejected above.
        unsafe {
            *this.data_func.get() = Some(func);
            *this.jpegxl_opaque.get() = jpegxl_opaque;
        }
        this.num_reserved.store(0, Ordering::Relaxed);
        // Bound participating worker IDs to the count reported to `init`, even
        // if `init` itself grew the lazy pool via `run_on_each_thread`.
        this.dispatch_workers
            .store(active_workers.max(1) as u32, Ordering::Relaxed);

        this.start_workers(worker_command);
        this.wait_for_workers_ready(worker_count, worker_count);
        drop(threads);

        // fetch_add(-1) of the source.
        if this.depth.fetch_sub(1, Ordering::AcqRel) != 1 {
            return JXL_PARALLEL_RET_RUNNER_ERROR;
        }
        JXL_PARALLEL_RET_SUCCESS
    }

    /// Calls the data function of the current command.
    fn call_data_func(&self, task: u32, thread: usize) {
        // SAFETY: Written before the mutex release that started this worker
        // and not written again until every worker reports ready.
        let (func, opaque) = unsafe { (*self.data_func.get(), *self.jpegxl_opaque.get()) };
        let func = func.expect("a command carries its data function");
        // SAFETY: The blocked runner call's caller guarantees the data
        // function contract for this opaque value on any runner thread.
        unsafe { func(opaque, task, thread) };
    }

    /// Attempts to reserve and perform some work from the global range of
    /// tasks, which is encoded within "command". Returns after all tasks are
    /// reserved.
    fn run_range(this: &Self, command: WorkerCommand, thread: i32) {
        let dispatch_workers = this.dispatch_workers.load(Ordering::Relaxed);
        if dispatch_workers != 0 && (thread as u32) >= dispatch_workers {
            return;
        }
        let begin = (command >> 32) as u32;
        let end = (command & 0xFFFF_FFFF) as u32;
        let num_tasks = end.wrapping_sub(begin);
        let num_worker_threads = if dispatch_workers != 0 {
            dispatch_workers
        } else {
            this.active_workers.load(Ordering::Relaxed).max(1)
        };

        // OpenMP introduced several "schedule" strategies:
        // "single" (static assignment of exactly one chunk per thread): slower.
        // "dynamic" (allocates k tasks at a time): competitive for well-chosen k.
        // "guided" (allocates k tasks, decreases k): computing k = remaining/n
        //   is faster than halving k each iteration. We prefer this strategy
        //   because it avoids user-specified parameters.
        // The source's disabled "dynamic" branch (#if JXL_FALSE) is not
        // translated.
        loop {
            // guided
            let num_reserved = this.num_reserved.load(Ordering::Relaxed);
            // It is possible that more tasks are reserved than ready to run.
            let num_remaining = num_tasks - num_reserved.min(num_tasks);
            let my_size = (num_remaining / num_worker_threads.wrapping_mul(4)).max(1);
            let my_begin =
                begin.wrapping_add(this.num_reserved.fetch_add(my_size, Ordering::Relaxed));
            let my_end = my_begin
                .wrapping_add(my_size)
                .min(begin.wrapping_add(num_tasks));
            // Another thread already reserved the last task.
            if my_begin >= my_end {
                break;
            }
            for task in my_begin..my_end {
                this.call_data_func(task, thread as usize);
            }
        }
    }

    fn thread_func(this: *const Self, thread: i32, initial_epoch: u64) {
        // SAFETY: The runner outlives this thread: the destructor joins it
        // before dropping the object. Only shared access is formed.
        let this = unsafe { &*this };
        let mut seen_epoch = initial_epoch;
        // Until WORKER_EXIT command received:
        loop {
            let mut lock = this.lock();
            // Notify main thread that this thread is ready.
            lock.workers_ready = lock.workers_ready.wrapping_add(1);
            if lock.workers_ready == lock.ready_target {
                this.workers_ready_cv.notify_one();
            }
            // Wait for a new command. Checking the epoch before waiting ensures
            // a command posted before a worker enters the wait (such as
            // WORKER_EXIT during partial-pool startup cleanup) is never missed,
            // and a duplicate wake for the current epoch never repeats work.
            let command = loop {
                if lock.command_epoch != seen_epoch && lock.worker_start_command != WORKER_WAIT {
                    seen_epoch = lock.command_epoch;
                    break lock.worker_start_command;
                }
                lock = this
                    .worker_start_cv
                    .wait(lock)
                    .unwrap_or_else(PoisonError::into_inner);
            };
            match command {
                WORKER_ONCE => {
                    drop(lock);
                    let dispatch_workers = this.dispatch_workers.load(Ordering::Relaxed);
                    if dispatch_workers == 0 || (thread as u32) < dispatch_workers {
                        this.call_data_func(thread as u32, thread as usize);
                    }
                }
                WORKER_EXIT => return, // exits thread
                _ => {
                    drop(lock);
                    Self::run_range(this, command, thread);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::threads::thread_parallel_runner::{
        jxl_thread_parallel_runner_create_checked, jxl_thread_parallel_runner_create_lazy,
        jxl_thread_parallel_runner_destroy, jxl_thread_parallel_runner_spawned_threads,
    };
    use std::mem::MaybeUninit;
    use std::sync::atomic::AtomicUsize;
    use std::time::{Duration, Instant};

    unsafe extern "C" fn count_task(opaque: *mut c_void, _task: u32, _thread: usize) {
        // SAFETY: The atomic outlives the runner and every worker call.
        unsafe { &*opaque.cast::<AtomicU32>() }.fetch_add(1, Ordering::Relaxed);
    }

    unsafe extern "C" fn record_init(
        opaque: *mut c_void,
        num_threads: usize,
    ) -> JxlParallelRetCode {
        // SAFETY: The test passes a live pair of atomics.
        let counters = unsafe { &*opaque.cast::<[AtomicUsize; 2]>() };
        counters[0].store(num_threads, Ordering::SeqCst);
        JXL_PARALLEL_RET_SUCCESS
    }

    unsafe extern "C" fn record_task(opaque: *mut c_void, _task: u32, _thread: usize) {
        // SAFETY: As above.
        let counters = unsafe { &*opaque.cast::<[AtomicUsize; 2]>() };
        counters[1].fetch_add(1, Ordering::SeqCst);
    }

    unsafe extern "C" fn failing_alloc(_opaque: *mut c_void, _size: usize) -> *mut c_void {
        std::ptr::null_mut()
    }

    unsafe extern "C" fn noop_free(_opaque: *mut c_void, _address: *mut c_void) {}

    #[test]
    fn duplicate_wakeup_does_not_repeat_a_command() {
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: The pinned stack storage stays live until destruct joins the worker.
        assert_eq!(
            unsafe { ThreadParallelRunner::construct(storage.as_mut_ptr(), 1) },
            Ok(())
        );
        // SAFETY: construct initialized the object in this storage.
        let runner = unsafe { storage.assume_init_ref() };
        let calls = AtomicU32::new(0);
        // SAFETY: The worker is waiting after construct's barrier; the atomic
        // stays live until destruct joins it.
        unsafe {
            *runner.data_func.get() = Some(count_task);
            *runner.jpegxl_opaque.get() = std::ptr::from_ref(&calls).cast_mut().cast();
        }
        runner.start_workers(1); // range [0, 1)

        // Observe the worker back in its wait before deliberately waking it
        // again while the previous command remains set.
        let mut lock = runner.lock();
        while lock.workers_ready < 1 {
            lock = runner
                .workers_ready_cv
                .wait(lock)
                .unwrap_or_else(PoisonError::into_inner);
        }
        let first_ready = lock.workers_ready;
        drop(lock);

        runner.worker_start_cv.notify_all();
        let deadline = Instant::now() + Duration::from_millis(100);
        let mut lock = runner.lock();
        while lock.workers_ready < 2 && Instant::now() < deadline {
            let (next, _) = runner
                .workers_ready_cv
                .wait_timeout(lock, Duration::from_millis(10))
                .unwrap_or_else(PoisonError::into_inner);
            lock = next;
        }
        let repeated_ready = lock.workers_ready;
        drop(lock);
        // SAFETY: After the first readiness report, the worker has completed
        // the task and is waiting. Exit wakes and joins it before storage ends.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
        assert_eq!(first_ready, 1);
        assert_eq!(repeated_ready, 1, "one command must be consumed once");
        assert_eq!(calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn partial_spawn_failure_joins_started_workers_and_returns_thread_spawn() {
        for fail_at in 0..4usize {
            let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
            let started = std::sync::Arc::new(AtomicU32::new(0));
            let exited = std::sync::Arc::new(AtomicU32::new(0));
            let mut spawned = 0usize;
            // SAFETY: Pinned stack storage is unused; construct_with_spawner
            // joins any started workers and drops the runner before returning
            // Err(RunnerError::ThreadSpawn).
            let res = unsafe {
                ThreadParallelRunner::construct_with_spawner(storage.as_mut_ptr(), 4, |work| {
                    if spawned == fail_at {
                        return Err(std::io::Error::other("injected worker spawn failure"));
                    }
                    spawned += 1;
                    let started = std::sync::Arc::clone(&started);
                    let exited = std::sync::Arc::clone(&exited);
                    std::thread::Builder::new().spawn(move || {
                        started.fetch_add(1, Ordering::Relaxed);
                        work();
                        exited.fetch_add(1, Ordering::Relaxed);
                    })
                })
            };
            assert_eq!(
                res,
                Err(RunnerError::ThreadSpawn),
                "fail_at={fail_at} must return ThreadSpawn"
            );
            assert_eq!(spawned, fail_at);
            assert_eq!(
                started.load(Ordering::Relaxed),
                fail_at as u32,
                "every started worker must have run"
            );
            assert_eq!(
                exited.load(Ordering::Relaxed),
                fail_at as u32,
                "every started worker must have exited before construct returned"
            );
        }
    }

    #[test]
    fn lazy_runner_spawns_only_for_multi_task_ranges_and_grows_on_demand() {
        // SAFETY: Null manager uses the default allocator.
        let runner = unsafe { jxl_thread_parallel_runner_create_lazy(std::ptr::null(), 8) }
            .expect("lazy creation spawns no threads");
        // SAFETY: `runner` is live and idle.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_spawned_threads(runner) },
            0
        );

        let counters = [AtomicUsize::new(0), AtomicUsize::new(0)];
        let opaque = std::ptr::from_ref(&counters).cast_mut().cast();

        // Single-task range runs on caller without spawning any workers.
        // SAFETY: Live runner and counters.
        let ret =
            unsafe { ThreadParallelRunner::runner(runner, opaque, record_init, record_task, 0, 1) };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(counters[0].load(Ordering::SeqCst), 1);
        assert_eq!(counters[1].load(Ordering::SeqCst), 1);
        // SAFETY: `runner` is live and idle.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_spawned_threads(runner) },
            0
        );

        // Two-task range spawns exactly 2 workers.
        // SAFETY: Live runner and counters.
        let ret =
            unsafe { ThreadParallelRunner::runner(runner, opaque, record_init, record_task, 0, 2) };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(counters[0].load(Ordering::SeqCst), 2);
        assert_eq!(counters[1].load(Ordering::SeqCst), 3);
        // SAFETY: `runner` is live and idle.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_spawned_threads(runner) },
            2
        );

        // Four-task range grows the pool incrementally from 2 to 4 workers.
        // SAFETY: Live runner and counters.
        let ret =
            unsafe { ThreadParallelRunner::runner(runner, opaque, record_init, record_task, 0, 4) };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(counters[0].load(Ordering::SeqCst), 4);
        assert_eq!(counters[1].load(Ordering::SeqCst), 7);
        // SAFETY: `runner` is live and idle.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_spawned_threads(runner) },
            4
        );

        // Subsequent single-task range still takes the calling-thread fast path.
        // SAFETY: Live runner and counters.
        let ret =
            unsafe { ThreadParallelRunner::runner(runner, opaque, record_init, record_task, 0, 1) };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(counters[0].load(Ordering::SeqCst), 1);
        assert_eq!(counters[1].load(Ordering::SeqCst), 8);
        // SAFETY: `runner` is live and idle.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_spawned_threads(runner) },
            4
        );

        // SAFETY: Idle runner destroyed once.
        unsafe { jxl_thread_parallel_runner_destroy(runner) };
    }

    #[test]
    fn lazy_growth_spawn_failure_joins_existing_and_new_workers() {
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until destruct below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 4) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };

        let started = std::sync::Arc::new(AtomicU32::new(0));
        let exited = std::sync::Arc::new(AtomicU32::new(0));
        // Grow 0 -> 2 successfully.
        let s1 = std::sync::Arc::clone(&started);
        let e1 = std::sync::Arc::clone(&exited);
        let ok = {
            let mut threads = runner.lock_threads();
            runner.ensure_workers_locked_with_spawner(&mut threads, 2, move |work| {
                let s = std::sync::Arc::clone(&s1);
                let e = std::sync::Arc::clone(&e1);
                std::thread::Builder::new().spawn(move || {
                    s.fetch_add(1, Ordering::Relaxed);
                    work();
                    e.fetch_add(1, Ordering::Relaxed);
                })
            })
        };
        assert_eq!(ok, Ok(2));
        assert_eq!(runner.spawned_threads(), 2);

        // Grow 2 -> 4, failing on the second new worker (worker #3).
        let mut delta_spawned = 0usize;
        let s2 = std::sync::Arc::clone(&started);
        let e2 = std::sync::Arc::clone(&exited);
        let err = {
            let mut threads = runner.lock_threads();
            runner.ensure_workers_locked_with_spawner(&mut threads, 4, move |work| {
                if delta_spawned == 1 {
                    return Err(std::io::Error::other("injected growth spawn failure"));
                }
                delta_spawned += 1;
                let s = std::sync::Arc::clone(&s2);
                let e = std::sync::Arc::clone(&e2);
                std::thread::Builder::new().spawn(move || {
                    s.fetch_add(1, Ordering::Relaxed);
                    work();
                    e.fetch_add(1, Ordering::Relaxed);
                })
            })
        };
        assert_eq!(err, Err(RunnerError::ThreadSpawn));
        assert_eq!(runner.spawned_threads(), 0);
        assert_eq!(started.load(Ordering::Relaxed), 3);
        assert_eq!(
            exited.load(Ordering::Relaxed),
            3,
            "both existing and newly started workers must exit cleanly"
        );
        // SAFETY: All workers exited; destruct drops the empty runner cleanly.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }

    #[test]
    fn allocation_and_manager_failures_are_distinguished_from_thread_spawn() {
        let oom_manager = JxlMemoryManager {
            opaque: std::ptr::null_mut(),
            alloc: Some(failing_alloc),
            free: Some(noop_free),
        };
        // SAFETY: Valid manager callbacks.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_create_checked(&oom_manager, 4) },
            Err(RunnerError::Allocation)
        );
        // SAFETY: Valid manager callbacks.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_create_lazy(&oom_manager, 4) },
            Err(RunnerError::Allocation)
        );

        let invalid_manager = JxlMemoryManager {
            opaque: std::ptr::null_mut(),
            alloc: Some(failing_alloc),
            free: None,
        };
        // SAFETY: Readable manager struct.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_create_checked(&invalid_manager, 4) },
            Err(RunnerError::InvalidMemoryManager)
        );
        // SAFETY: Readable manager struct.
        assert_eq!(
            unsafe { jxl_thread_parallel_runner_create_lazy(&invalid_manager, 4) },
            Err(RunnerError::InvalidMemoryManager)
        );
    }

    #[test]
    fn set_max_workers_clamps_to_construction_maximum_and_supports_zero_then_raise() {
        // Raising past construction maximum must clamp to num_threads() so
        // run_on_each_thread never passes a thread id >= num_threads().
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until destruct below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 2) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };
        assert_eq!(runner.num_threads(), 2);
        runner.set_max_workers(4);
        assert_eq!(runner.max_worker_threads(), 2);
        let max_tid = AtomicUsize::new(0);
        let calls = AtomicUsize::new(0);
        runner
            .run_on_each_thread(&|_task, tid| {
                assert!(
                    tid < runner.num_threads(),
                    "tid={tid} must stay < num_threads()"
                );
                max_tid.fetch_max(tid, Ordering::SeqCst);
                calls.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 2);
        assert_eq!(max_tid.load(Ordering::SeqCst), 1);
        // SAFETY: Runner is idle and destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };

        // Lowering to 0 for a single-group frame keeps 0 workers spawned; a
        // later multi-group frame can raise the cap back up to the construction
        // maximum and spawn workers on demand.
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until destruct below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 2) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };
        runner.set_max_workers(0);
        assert_eq!(runner.max_worker_threads(), 0);
        let counters = [AtomicUsize::new(0), AtomicUsize::new(0)];
        let opaque = std::ptr::from_ref(&counters).cast_mut().cast();
        let runner_ptr = std::ptr::from_ref(runner).cast_mut().cast::<c_void>();
        // SAFETY: Live runner and counters.
        let ret = unsafe {
            ThreadParallelRunner::runner(runner_ptr, opaque, record_init, record_task, 0, 8)
        };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(runner.spawned_threads(), 0);
        assert_eq!(counters[0].load(Ordering::SeqCst), 1);
        assert_eq!(counters[1].load(Ordering::SeqCst), 8);

        // Raise back above construction maximum (4 -> clamped to 2).
        runner.set_max_workers(4);
        assert_eq!(runner.max_worker_threads(), 2);
        // SAFETY: Live runner and counters.
        let ret = unsafe {
            ThreadParallelRunner::runner(runner_ptr, opaque, record_init, record_task, 0, 8)
        };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(runner.spawned_threads(), 2);
        assert_eq!(counters[0].load(Ordering::SeqCst), 2);
        assert_eq!(counters[1].load(Ordering::SeqCst), 16);
        // SAFETY: Runner is idle and destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }

    #[test]
    fn concurrent_run_on_each_thread_and_set_max_workers_are_race_free() {
        use std::sync::Barrier;

        for round in 0..4usize {
            let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
            // SAFETY: Pinned stack storage stays live until `destruct` below,
            // after `thread::scope` has joined all caller threads.
            unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 4) };
            // SAFETY: Initialized above and shared via `&ThreadParallelRunner`.
            let runner = unsafe { storage.assume_init_ref() };
            let start = Barrier::new(3);
            let total_calls = AtomicUsize::new(0);

            std::thread::scope(|scope| {
                // Two threads concurrently race on safe `run_on_each_thread`
                // starting from an unspawned lazy pool.
                for _ in 0..2 {
                    scope.spawn(|| {
                        start.wait();
                        for _ in 0..8 {
                            runner
                                .run_on_each_thread(&|task, tid| {
                                    assert_eq!(task as usize, tid);
                                    assert!(tid < runner.num_threads());
                                    total_calls.fetch_add(1, Ordering::Relaxed);
                                })
                                .unwrap();
                        }
                    });
                }
                // A third thread concurrently adjusts `set_max_workers` across
                // values below and above `construction_max_workers`.
                scope.spawn(|| {
                    start.wait();
                    for step in 0..16usize {
                        let cap = match (round + step) % 4 {
                            0 => 0,
                            1 => 2,
                            2 => 4,
                            _ => 8,
                        };
                        runner.set_max_workers(cap);
                    }
                });
            });

            assert!(total_calls.load(Ordering::Relaxed) >= 16);
            assert!(runner.spawned_threads() <= runner.num_threads());
            // SAFETY: All scoped callers joined; runner is idle and destroyed once.
            unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
        }
    }

    #[test]
    fn init_calling_run_on_each_thread_does_not_deadlock() {
        struct InitState<'a> {
            runner: &'a ThreadParallelRunner,
            init_num_threads: AtomicUsize,
            init_callbacks: AtomicUsize,
            task_callbacks: AtomicUsize,
        }

        unsafe extern "C" fn init_using_run_on_each_thread(
            opaque: *mut c_void,
            num_threads: usize,
        ) -> JxlParallelRetCode {
            // SAFETY: The test passes a live `InitState`.
            let state = unsafe { &*opaque.cast::<InitState<'_>>() };
            state.init_num_threads.store(num_threads, Ordering::SeqCst);
            state
                .runner
                .run_on_each_thread(&|_task, tid| {
                    assert!(tid < state.runner.num_threads());
                    state.init_callbacks.fetch_add(1, Ordering::SeqCst);
                })
                .unwrap();
            JXL_PARALLEL_RET_SUCCESS
        }

        unsafe extern "C" fn check_tid_task(opaque: *mut c_void, _task: u32, tid: usize) {
            // SAFETY: As above.
            let state = unsafe { &*opaque.cast::<InitState<'_>>() };
            let allowed = state.init_num_threads.load(Ordering::SeqCst);
            assert!(
                tid < allowed,
                "task tid={tid} must stay < num_threads passed to init ({allowed})"
            );
            state.task_callbacks.fetch_add(1, Ordering::SeqCst);
        }

        // Test eager 2-worker runner, lazy 2-worker runner, and lazy 4-worker
        // runner dispatching a 2-task range (where `init` grows the pool from
        // 2 to 4 via `run_on_each_thread`, yet `check_tid_task` must still only
        // receive `tid < 2`).
        for (lazy, workers, tasks, expected_init_threads, expected_init_cbs) in [
            (false, 2i32, 8u32, 2usize, 2usize),
            (true, 2i32, 8u32, 2usize, 2usize),
            (true, 4i32, 2u32, 2usize, 4usize),
        ] {
            let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
            // SAFETY: Pinned stack storage stays live until `destruct` below.
            unsafe {
                if lazy {
                    ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), workers);
                } else {
                    ThreadParallelRunner::construct(storage.as_mut_ptr(), workers).unwrap();
                }
            }
            // SAFETY: Initialized above.
            let runner = unsafe { storage.assume_init_ref() };
            let state = InitState {
                runner,
                init_num_threads: AtomicUsize::new(0),
                init_callbacks: AtomicUsize::new(0),
                task_callbacks: AtomicUsize::new(0),
            };
            let runner_ptr = std::ptr::from_ref(runner).cast_mut().cast::<c_void>();
            let opaque = std::ptr::from_ref(&state).cast_mut().cast::<c_void>();
            // SAFETY: Live runner and state outlive this call.
            let ret = unsafe {
                ThreadParallelRunner::runner(
                    runner_ptr,
                    opaque,
                    init_using_run_on_each_thread,
                    check_tid_task,
                    0,
                    tasks,
                )
            };
            assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
            assert_eq!(
                state.init_num_threads.load(Ordering::SeqCst),
                expected_init_threads
            );
            assert_eq!(
                state.init_callbacks.load(Ordering::SeqCst),
                expected_init_cbs
            );
            assert_eq!(state.task_callbacks.load(Ordering::SeqCst), tasks as usize);
            // SAFETY: Runner is idle and destroyed once.
            unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
        }
    }

    #[test]
    fn run_on_each_thread_returns_err_when_lazy_startup_fails() {
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until `destruct` below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 4) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };

        let started = std::sync::Arc::new(AtomicU32::new(0));
        let exited = std::sync::Arc::new(AtomicU32::new(0));
        let callbacks = AtomicUsize::new(0);
        let mut spawned = 0usize;
        let s = std::sync::Arc::clone(&started);
        let e = std::sync::Arc::clone(&exited);
        let result = runner.run_on_each_thread_with_spawner(
            &|_task, _tid| {
                callbacks.fetch_add(1, Ordering::SeqCst);
            },
            move |work| {
                if spawned == 2 {
                    return Err(std::io::Error::other("injected run_on_each_thread failure"));
                }
                spawned += 1;
                let s = std::sync::Arc::clone(&s);
                let e = std::sync::Arc::clone(&e);
                std::thread::Builder::new().spawn(move || {
                    s.fetch_add(1, Ordering::Relaxed);
                    work();
                    e.fetch_add(1, Ordering::Relaxed);
                })
            },
        );
        assert_eq!(result, Err(RunnerError::ThreadSpawn));
        assert_eq!(runner.last_error(), Some(RunnerError::ThreadSpawn));
        assert_eq!(callbacks.load(Ordering::SeqCst), 0);
        assert_eq!(runner.spawned_threads(), 0);
        assert_eq!(started.load(Ordering::Relaxed), 2);
        assert_eq!(exited.load(Ordering::Relaxed), 2);
        // SAFETY: All started workers exited; runner is destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }

    #[test]
    fn large_and_negative_worker_counts_clamp_without_wraparound() {
        // Passing `i32::MAX as usize + 1` must clamp to `i32::MAX`, never wrap
        // negative and desynchronize `num_threads()` from `max_worker_threads()`.
        // SAFETY: Null manager uses the default allocator.
        let raw = unsafe {
            jxl_thread_parallel_runner_create_lazy(std::ptr::null(), i32::MAX as usize + 1)
        }
        .expect("lazy creation spawns no threads");
        // SAFETY: `raw` is a live runner created above.
        let runner = unsafe { &*raw.cast::<ThreadParallelRunner>() };
        assert_eq!(runner.max_worker_threads(), i32::MAX as usize);
        assert_eq!(runner.num_threads(), i32::MAX as usize);
        assert!(runner.num_threads() >= runner.max_worker_threads());
        // Cap to 2 and verify a 2-task dispatch runs cleanly within `num_threads()`.
        runner.set_max_workers(2);
        assert_eq!(runner.max_worker_threads(), 2);
        let counters = [AtomicUsize::new(0), AtomicUsize::new(0)];
        let opaque = std::ptr::from_ref(&counters).cast_mut().cast();
        // SAFETY: Live runner and counters.
        let ret =
            unsafe { ThreadParallelRunner::runner(raw, opaque, record_init, record_task, 0, 2) };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(counters[0].load(Ordering::SeqCst), 2);
        assert_eq!(counters[1].load(Ordering::SeqCst), 2);
        // SAFETY: Idle runner destroyed once.
        unsafe { jxl_thread_parallel_runner_destroy(raw) };

        // Direct negative `i32` clamps to 0 workers (`num_threads() == 1`).
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until `destruct` below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), -5) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };
        assert_eq!(runner.max_worker_threads(), 0);
        assert_eq!(runner.num_threads(), 1);
        runner.set_max_workers(4);
        assert_eq!(runner.max_worker_threads(), 0);
        // SAFETY: Runner is idle and destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }

    #[test]
    fn runner_returns_error_when_pool_is_torn_down_during_init() {
        struct TearDownState<'a> {
            runner: &'a ThreadParallelRunner,
            task_calls: AtomicUsize,
        }

        unsafe extern "C" fn init_tears_down_pool(
            opaque: *mut c_void,
            num_threads: usize,
        ) -> JxlParallelRetCode {
            assert_eq!(num_threads, 2);
            // SAFETY: Test passes a live `TearDownState`.
            let state = unsafe { &*opaque.cast::<TearDownState<'_>>() };
            // Trigger lazy growth 2 -> 4 that fails on spawn and tears down
            // every live worker while `runner` has released `self.threads`.
            let res = state
                .runner
                .run_on_each_thread_with_spawner(&|_task, _tid| {}, |_work| {
                    Err(std::io::Error::other("injected growth failure during init"))
                });
            assert_eq!(res, Err(RunnerError::ThreadSpawn));
            assert_eq!(state.runner.spawned_threads(), 0);
            // Return SUCCESS from `init` so `runner` must detect the dead pool
            // itself after re-acquiring `self.threads`.
            JXL_PARALLEL_RET_SUCCESS
        }

        unsafe extern "C" fn count_tasks(opaque: *mut c_void, _task: u32, _tid: usize) {
            // SAFETY: As above.
            let state = unsafe { &*opaque.cast::<TearDownState<'_>>() };
            state.task_calls.fetch_add(1, Ordering::SeqCst);
        }

        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until `destruct` below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 4) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };
        let state = TearDownState {
            runner,
            task_calls: AtomicUsize::new(0),
        };
        let runner_ptr = std::ptr::from_ref(runner).cast_mut().cast::<c_void>();
        let opaque = std::ptr::from_ref(&state).cast_mut().cast::<c_void>();
        // Dispatch 2 tasks: spawns 2 workers, then `init` fails growth to 4
        // and clears the pool. `runner` must return RUNNER_ERROR, never SUCCESS.
        // SAFETY: Live runner and state outlive this call.
        let ret = unsafe {
            ThreadParallelRunner::runner(
                runner_ptr,
                opaque,
                init_tears_down_pool,
                count_tasks,
                0,
                2,
            )
        };
        assert_eq!(ret, JXL_PARALLEL_RET_RUNNER_ERROR);
        assert_eq!(runner.last_error(), Some(RunnerError::ThreadSpawn));
        assert_eq!(state.task_calls.load(Ordering::SeqCst), 0);
        // SAFETY: All workers exited; runner is destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }

    #[test]
    fn run_on_each_thread_callback_calling_set_max_workers_does_not_deadlock() {
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until `destruct` below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 4) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };
        let calls = AtomicUsize::new(0);
        runner
            .run_on_each_thread(&|_task, tid| {
                runner.set_max_workers(tid);
                calls.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 4);
        assert!(runner.max_worker_threads() <= 4);
        // SAFETY: Runner is idle and destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }

    #[test]
    fn cap_zero_after_pool_growth_runs_on_caller_without_waking_workers() {
        use std::thread::ThreadId;

        struct CallerCheck {
            caller: ThreadId,
            init_threads: AtomicUsize,
            on_caller: AtomicUsize,
            on_worker: AtomicUsize,
            max_tid: AtomicUsize,
        }

        unsafe extern "C" fn check_init(opaque: *mut c_void, n: usize) -> JxlParallelRetCode {
            // SAFETY: Test passes a live `CallerCheck`.
            let check = unsafe { &*opaque.cast::<CallerCheck>() };
            check.init_threads.store(n, Ordering::SeqCst);
            JXL_PARALLEL_RET_SUCCESS
        }

        unsafe extern "C" fn check_task(opaque: *mut c_void, _task: u32, tid: usize) {
            // SAFETY: As above.
            let check = unsafe { &*opaque.cast::<CallerCheck>() };
            if std::thread::current().id() == check.caller {
                check.on_caller.fetch_add(1, Ordering::SeqCst);
            } else {
                check.on_worker.fetch_add(1, Ordering::SeqCst);
            }
            check.max_tid.fetch_max(tid, Ordering::SeqCst);
        }

        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: Pinned stack storage stays live until `destruct` below.
        unsafe { ThreadParallelRunner::construct_lazy(storage.as_mut_ptr(), 4) };
        // SAFETY: Initialized above.
        let runner = unsafe { storage.assume_init_ref() };
        let runner_ptr = std::ptr::from_ref(runner).cast_mut().cast::<c_void>();

        // 1. Multi-group frame grows the pool to 4 workers.
        let check = CallerCheck {
            caller: std::thread::current().id(),
            init_threads: AtomicUsize::new(0),
            on_caller: AtomicUsize::new(0),
            on_worker: AtomicUsize::new(0),
            max_tid: AtomicUsize::new(0),
        };
        let opaque = std::ptr::from_ref(&check).cast_mut().cast::<c_void>();
        // SAFETY: Live runner and check struct.
        let ret = unsafe {
            ThreadParallelRunner::runner(runner_ptr, opaque, check_init, check_task, 0, 4)
        };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(runner.spawned_threads(), 4);
        assert_eq!(check.init_threads.load(Ordering::SeqCst), 4);
        assert_eq!(check.on_worker.load(Ordering::SeqCst), 4);
        assert_eq!(check.on_caller.load(Ordering::SeqCst), 0);

        // 2. Subsequent single-group frame sets cap=0: must run all 8 tasks on
        //    the calling thread without waking the 4 existing workers.
        runner.set_max_workers(0);
        assert_eq!(runner.max_worker_threads(), 0);
        check.on_caller.store(0, Ordering::SeqCst);
        check.on_worker.store(0, Ordering::SeqCst);
        check.max_tid.store(0, Ordering::SeqCst);
        // SAFETY: Live runner and check struct.
        let ret = unsafe {
            ThreadParallelRunner::runner(runner_ptr, opaque, check_init, check_task, 0, 8)
        };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(check.init_threads.load(Ordering::SeqCst), 1);
        assert_eq!(check.on_caller.load(Ordering::SeqCst), 8);
        assert_eq!(check.on_worker.load(Ordering::SeqCst), 0);
        assert_eq!(check.max_tid.load(Ordering::SeqCst), 0);

        // 3. Subsequent 2-group frame sets cap=2: init sees 2 and every task
        //    runs with tid < 2 even though 4 workers remain live.
        runner.set_max_workers(2);
        assert_eq!(runner.max_worker_threads(), 2);
        check.on_caller.store(0, Ordering::SeqCst);
        check.on_worker.store(0, Ordering::SeqCst);
        check.max_tid.store(0, Ordering::SeqCst);
        // SAFETY: Live runner and check struct.
        let ret = unsafe {
            ThreadParallelRunner::runner(runner_ptr, opaque, check_init, check_task, 0, 8)
        };
        assert_eq!(ret, JXL_PARALLEL_RET_SUCCESS);
        assert_eq!(check.init_threads.load(Ordering::SeqCst), 2);
        assert_eq!(check.on_worker.load(Ordering::SeqCst), 8);
        assert!(check.max_tid.load(Ordering::SeqCst) < 2);

        // SAFETY: Runner is idle and destroyed once.
        unsafe { ThreadParallelRunner::destruct(storage.as_mut_ptr()) };
    }
}
