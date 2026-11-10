// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// lib/threads/thread_parallel_runner_test.cc through ThreadPool, as its
// ThreadPoolForTests does, and the same properties for the resizable runner.
// The runners' task-to-thread assignment is nondeterministic; these tests
// check the source's properties of every run, not one schedule.
use jxl::jxl::base::data_parallel::{
    JXL_PARALLEL_RET_RUNNER_ERROR, JXL_PARALLEL_RET_SUCCESS, JxlParallelRetCode, JxlParallelRunner,
    ThreadPool, run_on_pool_exclusive,
};
use jxl::jxl::base::status::{Status, StatusCode};
use jxl::jxl::memory_manager::JxlMemoryManager;
use jxl::threads::resizable_parallel_runner::{
    jxl_resizable_parallel_runner, jxl_resizable_parallel_runner_create,
    jxl_resizable_parallel_runner_destroy, jxl_resizable_parallel_runner_set_threads,
    jxl_resizable_parallel_runner_suggest_threads,
};
use jxl::threads::thread_parallel_runner::{
    jxl_thread_parallel_runner, jxl_thread_parallel_runner_create,
    jxl_thread_parallel_runner_default_num_worker_threads, jxl_thread_parallel_runner_destroy,
};
use jxl::threads::thread_parallel_runner_internal::ThreadParallelRunner;
use std::ffi::c_void;
use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};

/// A runner and its pool; destroying it joins the workers.
struct PoolForTests {
    runner: *mut c_void,
    resizable: bool,
    pool: ThreadPool,
}
impl PoolForTests {
    /// ThreadPoolForTests: the thread runner with `num_threads` workers.
    fn new(num_threads: usize) -> Self {
        // SAFETY: A null manager selects the default allocator.
        let runner = unsafe { jxl_thread_parallel_runner_create(std::ptr::null(), num_threads) };
        assert!(!runner.is_null());
        Self::with_runner(runner, jxl_thread_parallel_runner, false)
    }
    /// The resizable runner after SetThreads(num_threads).
    fn resizable(num_threads: usize) -> Self {
        let runner = jxl_resizable_parallel_runner_create(std::ptr::null());
        // SAFETY: The live runner is not used concurrently.
        unsafe { jxl_resizable_parallel_runner_set_threads(runner, num_threads) };
        Self::with_runner(runner, jxl_resizable_parallel_runner, true)
    }
    fn with_runner(runner: *mut c_void, function: JxlParallelRunner, resizable: bool) -> Self {
        // SAFETY: The library's runners obey parallel_runner.h; the runner
        // outlives the pool, which is dropped first in Drop.
        let pool = unsafe { ThreadPool::new(Some(function), runner) };
        Self {
            runner,
            resizable,
            pool,
        }
    }
    fn get(&self) -> &ThreadPool {
        &self.pool
    }
}
impl Drop for PoolForTests {
    fn drop(&mut self) {
        // SAFETY: The runner is live, idle and not used again.
        unsafe {
            if self.resizable {
                jxl_resizable_parallel_runner_destroy(self.runner);
            } else {
                jxl_thread_parallel_runner_destroy(self.runner);
            }
        }
    }
}

fn no_init_func(_num_threads: usize) -> Status {
    Ok(())
}

/// TestPool: the task parameter is in bounds, every parameter is reached, the
/// pool can be reused and destroyed, and zero threads run on the caller.
fn test_pool(make: impl Fn(usize) -> PoolForTests, max_threads: usize) {
    for num_threads in 0..=max_threads {
        let pool = make(num_threads);
        for num_tasks in 0..32i32 {
            let mementos: Vec<AtomicI32> = (0..num_tasks).map(|_| AtomicI32::new(0)).collect();
            for begin in 0..32i32 {
                for memento in &mementos {
                    memento.store(0, Ordering::Relaxed);
                }
                let do_task = |task: u32, _thread: usize| -> Status {
                    let task = task as i32;
                    // Parameter is in the given range
                    assert!(task >= begin);
                    assert!(task < begin + num_tasks);
                    // Store mementos to be sure we visited each task.
                    mementos[(task - begin) as usize].store(1000 + task, Ordering::Relaxed);
                    Ok(())
                };
                pool.get()
                    .run(
                        begin as u32,
                        (begin + num_tasks) as u32,
                        &no_init_func,
                        &do_task,
                    )
                    .unwrap();
                for task in begin..begin + num_tasks {
                    assert_eq!(
                        1000 + task,
                        mementos[(task - begin) as usize].load(Ordering::Relaxed)
                    );
                }
            }
        }
    }
}

#[test]
fn thread_parallel_runner_test_pool() {
    test_pool(PoolForTests::new, 18);
}

#[test]
fn resizable_parallel_runner_test_pool() {
    test_pool(PoolForTests::resizable, 18);
}

/// TestSmallAssignments: the "thread" parameter when processing few tasks.
fn test_small_assignments(
    make: impl Fn(usize) -> PoolForTests,
    max_thread: impl Fn(usize) -> usize,
) {
    const MAX_THREADS: usize = 8;
    for num_threads in 1..=MAX_THREADS {
        let pool = make(num_threads);
        // (Avoid mutex because it may perturb the worker thread scheduling)
        let id_bits = AtomicU64::new(0);
        let num_calls = AtomicU32::new(0);
        let limit = max_thread(num_threads);
        let do_task = |_task: u32, thread: usize| -> Status {
            num_calls.fetch_add(1, Ordering::Relaxed);
            assert!(thread < limit);
            id_bits.fetch_or(1 << thread, Ordering::Relaxed);
            Ok(())
        };
        pool.get()
            .run(0, num_threads as u32, &no_init_func, &do_task)
            .unwrap();
        // Correct number of tasks.
        assert_eq!(num_threads as u32, num_calls.load(Ordering::Relaxed));
        let num_participants = id_bits.load(Ordering::Relaxed).count_ones() as usize;
        // Can't expect equality because other workers may have woken up too late.
        assert!(num_participants <= num_threads);
    }
}

#[test]
fn thread_parallel_runner_test_small_assignments() {
    test_small_assignments(PoolForTests::new, |n| n);
}

#[test]
fn resizable_parallel_runner_test_small_assignments() {
    // SetThreads(n) keeps n - 1 workers; the caller is thread 0.
    test_small_assignments(PoolForTests::resizable, |n| n);
}

#[repr(align(128))]
struct Counter {
    counter: AtomicI32,
}

/// TestCounter: per-thread sums assimilate to the sum of all tasks.
fn test_counter(pool: &PoolForTests, num_counters: usize) {
    let counters: Vec<Counter> = (0..num_counters)
        .map(|_| Counter {
            counter: AtomicI32::new(0),
        })
        .collect();
    let num_tasks = 12 * 19;
    let count = |task: u32, thread: usize| -> Status {
        counters[thread]
            .counter
            .fetch_add(task as i32, Ordering::Relaxed);
        Ok(())
    };
    pool.get().run(0, num_tasks, &no_init_func, &count).unwrap();
    let expected: i32 = (0..num_tasks as i32).sum();
    let total: i32 = counters
        .iter()
        .map(|c| c.counter.load(Ordering::Relaxed))
        .sum();
    assert_eq!(expected, total);
}

#[test]
fn thread_parallel_runner_test_counter() {
    const NUM_THREADS: usize = 12;
    test_counter(&PoolForTests::new(NUM_THREADS), NUM_THREADS);
}

#[test]
fn resizable_parallel_runner_test_counter() {
    const NUM_THREADS: usize = 12;
    test_counter(&PoolForTests::resizable(NUM_THREADS), NUM_THREADS);
}

/// The init function receives the runner's thread count before any task, and
/// every thread id stays below it.
#[test]
fn runner_init_thread_counts() {
    type Make = fn(usize) -> PoolForTests;
    let cases: [(Make, usize, u32, usize); 8] = [
        (PoolForTests::new, 0, 5, 1),
        (PoolForTests::new, 1, 5, 1),
        (PoolForTests::new, 4, 2, 4),
        (PoolForTests::new, 4, 9, 4),
        // SetThreads(n) keeps n - 1 workers; init receives min(workers + 1,
        // tasks), and a single task takes the init(1) shortcut.
        (PoolForTests::resizable, 0, 5, 1),
        (PoolForTests::resizable, 4, 2, 2),
        (PoolForTests::resizable, 4, 9, 4),
        (PoolForTests::resizable, 4, 1, 1),
    ];
    for (make, threads, tasks, expected) in cases {
        let pool = make(threads);
        for _ in 0..3 {
            let inits = Mutex::new(Vec::new());
            let calls = AtomicUsize::new(0);
            let init = |num_threads: usize| -> Status {
                assert_eq!(calls.load(Ordering::SeqCst), 0);
                inits.lock().unwrap().push(num_threads);
                Ok(())
            };
            let data = |_task: u32, thread: usize| -> Status {
                assert!(thread < expected);
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            };
            pool.get().run(10, 10 + tasks, &init, &data).unwrap();
            assert_eq!(*inits.lock().unwrap(), [expected]);
            assert_eq!(calls.load(Ordering::SeqCst), tasks as usize);
        }
    }
}

/// An init or data error fails the run; after a data error, the error flag
/// stops tasks that start later, as RunCallState does.
#[test]
fn runner_error_propagation() {
    for make in [PoolForTests::new, PoolForTests::resizable] {
        for threads in [0, 1, 3] {
            let pool = make(threads);
            let calls = AtomicUsize::new(0);
            let data = |_task: u32, _thread: usize| -> Status {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            };
            let failing_init = |_num_threads: usize| -> Status { Err(StatusCode::GenericError) };
            assert!(pool.get().run(0, 7, &failing_init, &data).is_err());
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            let failing = |task: u32, _thread: usize| -> Status {
                if task == 0 {
                    Err(StatusCode::GenericError)
                } else {
                    Ok(())
                }
            };
            assert!(pool.get().run(0, 64, &no_init_func, &failing).is_err());
            // The pool stays usable after a failed run.
            pool.get().run(0, 7, &no_init_func, &data).unwrap();
            assert_eq!(calls.load(Ordering::SeqCst), 7);
        }
    }
}

/// A panicking init or task body, the port's representation of an undefined
/// source use, reaches the caller of Run with and without a runner instead of
/// unwinding into the runner; the remaining tasks are skipped and the pool
/// stays usable.
#[test]
fn runner_body_panics_resume_on_the_caller() {
    let pools: [&dyn Fn(usize) -> Option<PoolForTests>; 3] = [
        &|_| None,
        &|threads| Some(PoolForTests::new(threads)),
        &|threads| Some(PoolForTests::resizable(threads)),
    ];
    for make in pools {
        for threads in [0, 1, 3] {
            let pool = make(threads);
            // SAFETY: A runner-less pool runs only the supplied local functions.
            let local = unsafe { ThreadPool::new(None, std::ptr::null_mut()) };
            let pool = pool.as_ref().map_or(&local, PoolForTests::get);
            let calls = AtomicUsize::new(0);
            let panicking = |task: u32, _thread: usize| -> Status {
                calls.fetch_add(1, Ordering::SeqCst);
                if task == 0 {
                    panic!("task body panic");
                }
                Ok(())
            };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                pool.run(0, 1, &no_init_func, &panicking)
            }));
            let payload = result.expect_err("the task panic reaches the caller");
            assert_eq!(payload.downcast_ref::<&str>(), Some(&"task body panic"));
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            let panicking_init = |_num_threads: usize| -> Status { panic!("init body panic") };
            let data = |_task: u32, _thread: usize| -> Status {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok(())
            };
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                pool.run(0, 7, &panicking_init, &data)
            }));
            let payload = result.expect_err("the init panic reaches the caller");
            assert_eq!(payload.downcast_ref::<&str>(), Some(&"init body panic"));
            assert_eq!(calls.load(Ordering::SeqCst), 1);
            // The pool stays usable after a panicking run.
            pool.run(0, 7, &no_init_func, &data).unwrap();
            assert_eq!(calls.load(Ordering::SeqCst), 8);
        }
    }
}

unsafe extern "C" fn count_init(opaque: *mut c_void, num_threads: usize) -> JxlParallelRetCode {
    // SAFETY: The tests pass a live counter pair.
    let counters = unsafe { &*opaque.cast::<[AtomicUsize; 2]>() };
    counters[0].store(num_threads, Ordering::SeqCst);
    JXL_PARALLEL_RET_SUCCESS
}

unsafe extern "C" fn count_task(opaque: *mut c_void, _task: u32, _thread: usize) {
    // SAFETY: As above.
    let counters = unsafe { &*opaque.cast::<[AtomicUsize; 2]>() };
    counters[1].fetch_add(1, Ordering::SeqCst);
}

/// The runner callback's direct range checks, including an empty range that
/// calls no init function and an inverted range.
#[test]
fn thread_parallel_runner_ranges() {
    for threads in [0, 2] {
        // SAFETY: A null manager selects the default allocator.
        let runner = unsafe { jxl_thread_parallel_runner_create(std::ptr::null(), threads) };
        let counters = [AtomicUsize::new(usize::MAX), AtomicUsize::new(0)];
        let opaque = std::ptr::from_ref(&counters).cast_mut().cast();
        // SAFETY: The live runner and the counters outlive these calls.
        unsafe {
            assert_eq!(
                jxl_thread_parallel_runner(runner, opaque, count_init, count_task, 5, 4),
                JXL_PARALLEL_RET_RUNNER_ERROR
            );
            assert_eq!(
                jxl_thread_parallel_runner(runner, opaque, count_init, count_task, 5, 5),
                JXL_PARALLEL_RET_SUCCESS
            );
            assert_eq!(counters[0].load(Ordering::SeqCst), usize::MAX);
            assert_eq!(
                jxl_thread_parallel_runner(runner, opaque, count_init, count_task, 3, 40),
                JXL_PARALLEL_RET_SUCCESS
            );
            jxl_thread_parallel_runner_destroy(runner);
        }
        assert_eq!(counters[0].load(Ordering::SeqCst), threads.max(1));
        assert_eq!(counters[1].load(Ordering::SeqCst), 37);
    }
    // Destroying a null runner does nothing.
    // SAFETY: Null is accepted.
    unsafe { jxl_thread_parallel_runner_destroy(std::ptr::null_mut()) };
}

struct Reentry {
    runner: *mut c_void,
    nested: AtomicI32,
}

unsafe extern "C" fn reenter_init(_opaque: *mut c_void, _num_threads: usize) -> JxlParallelRetCode {
    JXL_PARALLEL_RET_SUCCESS
}

unsafe extern "C" fn reenter_task(opaque: *mut c_void, task: u32, _thread: usize) {
    // SAFETY: The test passes a live Reentry whose runner is live.
    let reentry = unsafe { &*opaque.cast::<Reentry>() };
    if task == 0 {
        let counters = [AtomicUsize::new(0), AtomicUsize::new(0)];
        let inner = std::ptr::from_ref(&counters).cast_mut().cast();
        // SAFETY: The counters outlive the nested call.
        let ret = unsafe {
            jxl_thread_parallel_runner(reentry.runner, inner, count_init, count_task, 0, 4)
        };
        reentry.nested.store(ret, Ordering::SeqCst);
    }
}

/// Re-entering a runner with workers from one of its own tasks fails, as the
/// source's depth counter detects. The source does not undo the increment on
/// that path, so the outer and every later run of this runner fail as well.
#[test]
fn thread_parallel_runner_reentry() {
    // SAFETY: A null manager selects the default allocator.
    let runner = unsafe { jxl_thread_parallel_runner_create(std::ptr::null(), 2) };
    let reentry = Reentry {
        runner,
        nested: AtomicI32::new(1),
    };
    let opaque = std::ptr::from_ref(&reentry).cast_mut().cast();
    // SAFETY: The runner and the Reentry outlive these calls.
    unsafe {
        assert_eq!(
            jxl_thread_parallel_runner(runner, opaque, reenter_init, reenter_task, 0, 1),
            JXL_PARALLEL_RET_RUNNER_ERROR
        );
        assert_eq!(
            reentry.nested.load(Ordering::SeqCst),
            JXL_PARALLEL_RET_RUNNER_ERROR
        );
        let counters = [AtomicUsize::new(0), AtomicUsize::new(0)];
        let later = std::ptr::from_ref(&counters).cast_mut().cast();
        assert_eq!(
            jxl_thread_parallel_runner(runner, later, count_init, count_task, 0, 4),
            JXL_PARALLEL_RET_RUNNER_ERROR
        );
        assert_eq!(counters[1].load(Ordering::SeqCst), 0);
        jxl_thread_parallel_runner_destroy(runner);
    }
}

/// RunOnEachThread calls func(thread, thread) once on each worker, or once
/// on the caller without workers.
#[test]
fn thread_parallel_runner_run_on_each_thread() {
    for threads in [0usize, 1, 5] {
        // SAFETY: A null manager selects the default allocator.
        let runner = unsafe { jxl_thread_parallel_runner_create(std::ptr::null(), threads) };
        // SAFETY: The created runner is live until destroyed below.
        let object = unsafe { &*runner.cast::<ThreadParallelRunner>() };
        assert_eq!(object.num_threads(), threads.max(1));
        let seen = AtomicU64::new(0);
        object.run_on_each_thread(&|task: u32, thread: usize| {
            assert_eq!(task as usize, thread);
            seen.fetch_or(1 << thread, Ordering::SeqCst);
        });
        assert_eq!(seen.load(Ordering::SeqCst), (1u64 << threads.max(1)) - 1);
        // SAFETY: Idle and not used again.
        unsafe { jxl_thread_parallel_runner_destroy(runner) };
    }
}

struct Allocations {
    allocs: AtomicUsize,
    frees: AtomicUsize,
}
unsafe extern "C" fn counting_alloc(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The test passes a live counter.
    let allocations = unsafe { &*opaque.cast::<Allocations>() };
    allocations.allocs.fetch_add(1, Ordering::SeqCst);
    let layout = std::alloc::Layout::from_size_align(size, 128).unwrap();
    // SAFETY: Nonzero runner size.
    unsafe { std::alloc::alloc(layout).cast() }
}
unsafe extern "C" fn counting_free(opaque: *mut c_void, address: *mut c_void) {
    // SAFETY: As above.
    let allocations = unsafe { &*opaque.cast::<Allocations>() };
    allocations.frees.fetch_add(1, Ordering::SeqCst);
    let size = std::mem::size_of::<ThreadParallelRunner>();
    let layout = std::alloc::Layout::from_size_align(size, 128).unwrap();
    // SAFETY: Allocated by counting_alloc with this layout.
    unsafe { std::alloc::dealloc(address.cast(), layout) };
}

/// The runner object lives in its memory manager's allocation; a manager with
/// only one callback is rejected, as ThreadMemoryManagerInit requires.
#[test]
fn thread_parallel_runner_memory_manager() {
    let allocations = Allocations {
        allocs: AtomicUsize::new(0),
        frees: AtomicUsize::new(0),
    };
    let opaque = std::ptr::from_ref(&allocations).cast_mut().cast();
    let manager = JxlMemoryManager {
        opaque,
        alloc: Some(counting_alloc),
        free: Some(counting_free),
    };
    // SAFETY: The manager and its counters outlive the runner.
    let runner = unsafe { jxl_thread_parallel_runner_create(&manager, 3) };
    assert!(!runner.is_null());
    assert_eq!(allocations.allocs.load(Ordering::SeqCst), 1);
    // SAFETY: Idle and not used again.
    unsafe { jxl_thread_parallel_runner_destroy(runner) };
    assert_eq!(allocations.frees.load(Ordering::SeqCst), 1);
    let partial = JxlMemoryManager {
        opaque,
        alloc: Some(counting_alloc),
        free: None,
    };
    // SAFETY: The partial manager is rejected before any allocation.
    assert!(unsafe { jxl_thread_parallel_runner_create(&partial, 3) }.is_null());
    assert_eq!(allocations.allocs.load(Ordering::SeqCst), 1);
}

/// SetThreads grows and shrinks the resizable pool between runs.
#[test]
fn resizable_parallel_runner_resizes() {
    let pool = PoolForTests::resizable(0);
    for threads in [3usize, 1, 6, 0, 2] {
        // SAFETY: The live runner is idle.
        unsafe { jxl_resizable_parallel_runner_set_threads(pool.runner, threads) };
        let seen = AtomicU64::new(0);
        let calls = AtomicUsize::new(0);
        let data = |_task: u32, thread: usize| -> Status {
            assert!(thread < threads.max(1));
            seen.fetch_or(1 << thread, Ordering::SeqCst);
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        };
        pool.get().run(0, 100, &no_init_func, &data).unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 100);
        // Workers can drain the queue before the caller claims a task. With
        // no workers, however, every task must run on the caller.
        if threads <= 1 {
            assert_eq!(seen.load(Ordering::SeqCst), 1);
        }
        // The single-task fast path always invokes the caller, even when the
        // pool has workers (matching the original runner's Run).
        seen.store(0, Ordering::SeqCst);
        pool.get().run(0, 1, &no_init_func, &data).unwrap();
        assert_eq!(seen.load(Ordering::SeqCst), 1);
    }
    // Destroying a null runner does nothing.
    // SAFETY: Null is accepted.
    unsafe { jxl_resizable_parallel_runner_destroy(std::ptr::null_mut()) };
}

#[test]
fn runner_thread_count_suggestions() {
    let hardware = jxl_thread_parallel_runner_default_num_worker_threads() as u64;
    assert!(hardware >= 1);
    for (xsize, ysize) in [(0, 0), (256, 256), (2048, 2048), (1 << 20, 1 << 20)] {
        let expected = hardware.min(xsize * ysize / (256 * 256)) as u32;
        assert_eq!(
            jxl_resizable_parallel_runner_suggest_threads(xsize, ysize),
            expected
        );
    }
    // The source's uint64_t product wraps.
    assert_eq!(
        jxl_resizable_parallel_runner_suggest_threads(1 << 32, 1 << 32),
        0
    );
}

/// run_on_pool_exclusive keeps the runner's init count and thread ids and runs
/// the bodies one at a time on non-thread-safe state.
#[test]
fn exclusive_runs_keep_the_schedule() {
    for (make, threads, expected) in [
        (PoolForTests::new as fn(usize) -> PoolForTests, 0, 1),
        (PoolForTests::new, 1, 1),
        (PoolForTests::new, 6, 6),
        (PoolForTests::resizable, 6, 6),
    ] {
        let pool = make(threads);
        let mut state = (
            std::rc::Rc::new(std::cell::RefCell::new(Vec::new())),
            0usize,
        );
        // SAFETY: The state is local and used by no other thread.
        let status = unsafe {
            run_on_pool_exclusive(
                Some(pool.get()),
                0,
                500,
                &mut state,
                |(_, n), num_threads| {
                    *n = num_threads;
                    Ok(())
                },
                |(log, n), task, thread| {
                    assert!(thread < *n);
                    log.borrow_mut().push(task);
                    Ok(())
                },
            )
        };
        status.unwrap();
        assert_eq!(state.1, expected);
        let mut tasks = state.0.borrow().clone();
        tasks.sort_unstable();
        assert_eq!(tasks, (0..500).collect::<Vec<u32>>());
    }
}
