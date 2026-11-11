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
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;

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
    // Unmodified after ctor, but cannot be const because we call thread::join().
    threads: UnsafeCell<Vec<JoinHandle<()>>>,

    num_worker_threads: u32, // == threads.len()
    num_threads: u32,

    depth: AtomicU32, // detects if Run is re-entered (not supported).

    mutex: Mutex<Guarded>, // guards both cv and their variables.
    workers_ready_cv: Condvar,
    worker_start_cv: Condvar,

    // Written by main thread, read by workers (after mutex lock/unlock).
    data_func: UnsafeCell<Option<JxlParallelRunFunction>>,
    jpegxl_opaque: UnsafeCell<*mut c_void>,

    // Updated by workers; padding avoids false sharing.
    padding1: [u8; 64],
    num_reserved: AtomicU32,
    padding2: [u8; 64],

    memory_manager: UnsafeCell<JxlMemoryManager>,
}

// SAFETY: Workers share the object. Every field is either immutable after
// construction, an atomic, a std::sync primitive, or an UnsafeCell whose
// accesses are ordered by the mutex hand-off as stated at each access: the
// thread vector and memory manager belong to the main thread, and the data
// function and opaque value are only written while every worker waits.
unsafe impl Sync for ThreadParallelRunner {}

impl ThreadParallelRunner {
    /// Starts the given number of worker threads and blocks until they are
    /// ready. If zero, all tasks run on the main thread.
    ///
    /// # Safety
    /// `this` is writable storage, aligned for the runner, that holds no live
    /// object. The runner stays at this address until `destruct`.
    pub unsafe fn construct(this: *mut Self, num_worker_threads: i32) {
        // SAFETY: The caller supplies writable, aligned, unused storage.
        unsafe {
            this.write(Self {
                threads: UnsafeCell::new(Vec::new()),
                num_worker_threads: num_worker_threads as u32,
                num_threads: num_worker_threads.max(1) as u32,
                depth: AtomicU32::new(0),
                // Safely handle spurious worker wakeups.
                mutex: Mutex::new(Guarded {
                    workers_ready: 0,
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
        // SAFETY: Initialized above; workers only form shared references.
        let runner = unsafe { &*this };
        // Suppress "unused-private-field" warning.
        let _ = (&runner.padding1, &runner.padding2);
        // SAFETY: Only this constructing thread accesses the thread vector.
        let threads = unsafe { &mut *runner.threads.get() };
        threads.reserve(runner.num_worker_threads as usize);
        for i in 0..runner.num_worker_threads {
            let pointer = RunnerPointer(this);
            threads.push(std::thread::spawn(move || {
                Self::thread_func(pointer.get(), i as i32);
            }));
        }
        if runner.num_worker_threads != 0 {
            runner.workers_ready_barrier();
        }
    }

    /// Waits for all threads to exit and drops the object in place.
    ///
    /// # Safety
    /// `this` is a runner from `construct` that no `runner` call uses; it is
    /// not used afterwards.
    pub unsafe fn destruct(this: *mut Self) {
        // SAFETY: The caller transfers the live runner; workers hold only
        // shared references until they are joined below.
        let runner = unsafe { &*this };
        if runner.num_worker_threads != 0 {
            runner.start_workers(WORKER_EXIT);
        }
        // SAFETY: Only the destroying thread accesses the thread vector.
        let threads = std::mem::take(unsafe { &mut *runner.threads.get() });
        for thread in threads {
            // A worker never unwinds: runner callbacks abort on panic. A join
            // error should not ever happen, as in the source.
            let _ = thread.join();
        }
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
    pub fn run_on_each_thread<F: Fn(u32, usize) + Sync>(&self, func: &F) {
        if self.num_worker_threads == 0 {
            let thread = 0;
            func(thread as u32, thread as usize);
            return;
        }

        // SAFETY: Every worker is ready and waiting, so none reads these
        // cells; they read them after StartWorkers' mutex release below.
        unsafe {
            *self.data_func.get() = Some(Self::call_closure::<F>);
            *self.jpegxl_opaque.get() = std::ptr::from_ref(func).cast_mut().cast();
        }
        self.start_workers(WORKER_ONCE);
        self.workers_ready_barrier();
    }

    fn workers_ready_barrier(&self) {
        let mut lock = self.lock();
        // SAFETY: Only the main thread accesses the thread vector.
        let threads = unsafe { (*self.threads.get()).len() };
        // Typically only a single iteration.
        while lock.workers_ready as usize != threads {
            lock = self
                .workers_ready_cv
                .wait(lock)
                .unwrap_or_else(PoisonError::into_inner);
        }
        lock.workers_ready = 0;

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
    /// `runner_opaque` is a live runner from `construct`. `init` and `func`
    /// may be called with `jpegxl_opaque` from any of its threads during this
    /// call, as parallel_runner.h specifies.
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

        // SAFETY: The caller's init contract.
        let ret = unsafe { init(jpegxl_opaque, (this.num_worker_threads as usize).max(1)) };
        if ret != JXL_PARALLEL_RET_SUCCESS {
            return ret;
        }

        // Use a sequential run when num_worker_threads is zero since we have
        // no worker threads.
        if this.num_worker_threads == 0 {
            let thread: usize = 0;
            for task in start_range..end_range {
                // SAFETY: The caller's data-function contract.
                unsafe { func(jpegxl_opaque, task, thread) };
            }
            return JXL_PARALLEL_RET_SUCCESS;
        }

        if this.depth.fetch_add(1, Ordering::AcqRel) != 0 {
            return JXL_PARALLEL_RET_RUNNER_ERROR; // Must not re-enter.
        }

        let worker_command =
            (WorkerCommand::from(start_range) << 32).wrapping_add(WorkerCommand::from(end_range));
        // Ensure the inputs do not result in a reserved command.
        if worker_command == WORKER_WAIT
            || worker_command == WORKER_ONCE
            || worker_command == WORKER_EXIT
        {
            return JXL_PARALLEL_RET_RUNNER_ERROR;
        }

        // SAFETY: Every worker is ready and waiting (constructor or previous
        // barrier), so none reads these cells; they read them only after the
        // StartWorkers mutex release below. Re-entry was rejected above.
        unsafe {
            *this.data_func.get() = Some(func);
            *this.jpegxl_opaque.get() = jpegxl_opaque;
        }
        this.num_reserved.store(0, Ordering::Relaxed);

        this.start_workers(worker_command);
        this.workers_ready_barrier();

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
        let begin = (command >> 32) as u32;
        let end = (command & 0xFFFF_FFFF) as u32;
        let num_tasks = end.wrapping_sub(begin);
        let num_worker_threads = this.num_worker_threads;

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

    fn thread_func(this: *const Self, thread: i32) {
        // SAFETY: The runner outlives this thread: the destructor joins it
        // before dropping the object. Only shared access is formed.
        let this = unsafe { &*this };
        let mut seen_epoch = 0;
        // Until WORKER_EXIT command received:
        loop {
            let mut lock = this.lock();
            // Notify main thread that this thread is ready.
            lock.workers_ready = lock.workers_ready.wrapping_add(1);
            if lock.workers_ready == this.num_threads {
                this.workers_ready_cv.notify_one();
            }
            // Wait for a new command. A second wake for the current command
            // must not execute it again or increment readiness twice.
            let command = loop {
                lock = this
                    .worker_start_cv
                    .wait(lock)
                    .unwrap_or_else(PoisonError::into_inner);
                if lock.command_epoch == seen_epoch || lock.worker_start_command == WORKER_WAIT {
                    continue;
                }
                seen_epoch = lock.command_epoch;
                break lock.worker_start_command;
            };
            match command {
                WORKER_ONCE => {
                    drop(lock);
                    this.call_data_func(thread as u32, thread as usize);
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
    use std::mem::MaybeUninit;
    use std::time::{Duration, Instant};

    unsafe extern "C" fn count_task(opaque: *mut c_void, _task: u32, _thread: usize) {
        // SAFETY: The atomic outlives the runner and every worker call.
        unsafe { &*opaque.cast::<AtomicU32>() }.fetch_add(1, Ordering::Relaxed);
    }

    #[test]
    fn duplicate_wakeup_does_not_repeat_a_command() {
        let mut storage = MaybeUninit::<ThreadParallelRunner>::uninit();
        // SAFETY: The pinned stack storage stays live until destruct joins the worker.
        unsafe { ThreadParallelRunner::construct(storage.as_mut_ptr(), 1) };
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
}
