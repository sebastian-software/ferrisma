// libwebp v1.6.0, src/utils/thread_utils.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Multi-threaded worker
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/utils/thread_utils.h:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Multi-threaded worker
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/utils/thread_utils.c` and `src/utils/thread_utils.h` from libwebp v1.6.0.
//!
//! Work package: WP1 (types, memory, workers).
//!
//! `WebPWorkerInterface`, `WebPSetWorkerInterface` and `WebPGetWorkerInterface`
//! are not translated (ADR 0003, decided 2026-11-06). Callers use the worker
//! methods directly, and no public header declares the replacement table.
//!
//! Every worker starts a `std::thread` on `reset`, as the reference configuration
//! with `WEBP_USE_THREAD` does. The thread and the caller share one `Mutex` with
//! one `Condvar`. The job (hook and two data values) moves to the thread at
//! `launch` and comes back at `sync` or `end`; while the worker is working, the
//! caller has no access to the data, which the borrow checker enforces.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Worker allocation.** The shared synchronization object is allocated in
//!    `WebPWorker::init` (C: `Init`) rather than in `Reset`, so a failed
//!    allocation cannot make `Reset` return `false`. A failed spawn still does:
//!    `Reset` returns `false` and the status stays `NOT_OK`, as ADR 0003
//!    requires.

use std::cmp::Ordering;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;

// upstream: src/utils/thread_utils.h WebPWorkerStatus
/// State of a worker (C: `WebPWorkerStatus`); the values keep the C order.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WebPWorkerStatus(pub u32);

impl WebPWorkerStatus {
    // upstream: src/utils/thread_utils.h NOT_OK
    pub const NOT_OK: Self = Self(0);
    // upstream: src/utils/thread_utils.h OK
    pub const OK: Self = Self(1);
    // upstream: src/utils/thread_utils.h WORK
    pub const WORK: Self = Self(2);
}

// upstream: src/utils/thread_utils.h WebPWorkerHook
/// The function a worker runs on its two data values. It returns `false` on
/// error (C: `WebPWorkerHook`, which returns `int`).
pub type WebPWorkerHook<D1, D2> = fn(&mut D1, &mut D2) -> bool;

/// The hook and the data that it runs on.
struct Job<D1, D2> {
    hook: WebPWorkerHook<D1, D2>,
    data1: D1,
    data2: D2,
}

/// The state that the caller and the thread share (C: the fields of `WebPWorker`
/// that `ThreadLoop` reads and writes under the mutex).
struct State<D1, D2> {
    status: WebPWorkerStatus,
    had_error: bool,
    /// The job while the worker is working; the caller takes it back at `sync`.
    job: Option<Job<D1, D2>>,
}

/// The mutex, the condition and the state that a worker thread shares (C:
/// `WebPWorkerImpl`, plus the status fields of `WebPWorker`).
struct Shared<D1, D2> {
    state: Mutex<State<D1, D2>>,
    changed: Condvar,
}

impl<D1, D2> Shared<D1, D2> {
    fn lock(&self) -> MutexGuard<'_, State<D1, D2>> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn wait<'a>(&self, guard: MutexGuard<'a, State<D1, D2>>) -> MutexGuard<'a, State<D1, D2>> {
        self.changed
            .wait(guard)
            .unwrap_or_else(PoisonError::into_inner)
    }
}

/// Moves a job that the thread returned back into the worker.
fn return_job<D1, D2>(job: &mut Option<Job<D1, D2>>, state: &mut State<D1, D2>) {
    if let Some(returned) = state.job.take() {
        *job = Some(returned);
    }
}

// upstream: src/utils/thread_utils.h WebPWorker
/// A worker that runs one hook on its job, on a thread or on the caller's
/// thread (C: `WebPWorker`).
pub struct WebPWorker<D1, D2> {
    /// The hook and data while the worker is idle; the thread holds them while
    /// the worker works (C: the `hook`, `data1` and `data2` fields).
    job: Option<Job<D1, D2>>,
    shared: Arc<Shared<D1, D2>>,
    /// Present after a successful `reset`, until `end` (C: `impl != NULL`).
    thread: Option<JoinHandle<()>>,
    /// True from `launch` until `sync` or `end` return the job.
    working: bool,
}

// upstream: src/utils/thread_utils.c Init
impl<D1, D2> Default for WebPWorker<D1, D2> {
    fn default() -> Self {
        Self::init()
    }
}

impl<D1, D2> WebPWorker<D1, D2> {
    // upstream: src/utils/thread_utils.c Init
    /// Creates an idle worker with no job and no thread (C: `Init`).
    #[must_use]
    pub fn init() -> Self {
        Self {
            job: None,
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    status: WebPWorkerStatus::NOT_OK,
                    had_error: false,
                    job: None,
                }),
                changed: Condvar::new(),
            }),
            thread: None,
            working: false,
        }
    }

    // upstream: src/utils/thread_utils.h WebPWorker
    /// Sets the hook and the data that the next `launch` or `execute` runs (C:
    /// setting `hook`, `data1` and `data2`).
    ///
    /// # Panics
    ///
    /// Panics while the worker is working: C forbids the change until `sync`.
    pub fn set_job(&mut self, hook: WebPWorkerHook<D1, D2>, data1: D1, data2: D2) {
        assert!(
            !self.working,
            "the job of a working worker belongs to its thread until sync"
        );
        self.job = Some(Job { hook, data1, data2 });
    }

    /// The data of the current job; `None` when there is no job or the worker
    /// is working, because the thread then owns the data.
    #[must_use]
    pub fn job_data(&self) -> Option<(&D1, &D2)> {
        self.job.as_ref().map(|job| (&job.data1, &job.data2))
    }

    /// Mutable access to the data of the current job, under the same rule as
    /// [`WebPWorker::job_data`].
    pub fn job_data_mut(&mut self) -> Option<(&mut D1, &mut D2)> {
        self.job
            .as_mut()
            .map(|job| (&mut job.data1, &mut job.data2))
    }

    // upstream: src/utils/thread_utils.h WebPWorker
    /// The current status (C: the `status` field).
    #[must_use]
    pub fn status(&self) -> WebPWorkerStatus {
        self.shared.lock().status
    }

    /// Whether the hook has failed since the last `reset` (C: the `had_error`
    /// field).
    #[must_use]
    pub fn had_error(&self) -> bool {
        self.shared.lock().had_error
    }

    // upstream: src/utils/thread_utils.c Execute
    /// Runs the hook on the calling thread and records a failure (C: `Execute`).
    /// A worker without a job does nothing.
    pub fn execute(&mut self) {
        if let Some(job) = self.job.as_mut()
            && !(job.hook)(&mut job.data1, &mut job.data2)
        {
            self.shared.lock().had_error = true;
        }
    }

    // upstream: src/utils/thread_utils.c ChangeState
    /// Waits until the worker is idle and moves it to `new_status` when that is
    /// not `OK` (C: `ChangeState`). Without a thread this does nothing.
    fn change_state(&mut self, new_status: WebPWorkerStatus) {
        if self.thread.is_none() {
            return;
        }
        let mut state = self.shared.lock();
        if state.status >= WebPWorkerStatus::OK {
            while state.status != WebPWorkerStatus::OK {
                state = self.shared.wait(state);
            }
            if new_status != WebPWorkerStatus::OK {
                state.status = new_status;
                drop(state);
                self.shared.changed.notify_all();
            }
        }
    }

    // upstream: src/utils/thread_utils.c Launch
    /// Starts the job on the thread (C: `Launch`). Without a thread nothing
    /// starts and the hook does not run, as in C.
    pub fn launch(&mut self) {
        if self.thread.is_none() {
            return;
        }
        let mut state = self.shared.lock();
        if state.status >= WebPWorkerStatus::OK {
            while state.status != WebPWorkerStatus::OK {
                state = self.shared.wait(state);
            }
            return_job(&mut self.job, &mut state);
            state.status = WebPWorkerStatus::WORK;
            state.job = self.job.take();
            self.working = true;
            drop(state);
            self.shared.changed.notify_all();
        }
    }

    // upstream: src/utils/thread_utils.c Sync
    /// Waits until the worker is idle, takes the job back and reports whether no
    /// hook has failed (C: `Sync`). Without a thread the job never left the
    /// worker, so this only reports the error flag.
    #[must_use]
    pub fn sync(&mut self) -> bool {
        self.change_state(WebPWorkerStatus::OK);
        let mut state = self.shared.lock();
        debug_assert!(state.status <= WebPWorkerStatus::OK);
        return_job(&mut self.job, &mut state);
        self.working = false;
        !state.had_error
    }

    // upstream: src/utils/thread_utils.c End
    /// Stops the thread and takes the job back (C: `End`). A worker without a
    /// thread is left as it is.
    pub fn end(&mut self) {
        if self.thread.is_none() {
            return;
        }
        self.change_state(WebPWorkerStatus::NOT_OK);
        if let Some(handle) = self.thread.take() {
            // The thread has finished its loop; joining only reaps it.
            let _ = handle.join();
        }
        let mut state = self.shared.lock();
        return_job(&mut self.job, &mut state);
        self.working = false;
    }
}

impl<D1: Send + 'static, D2: Send + 'static> WebPWorker<D1, D2> {
    // upstream: src/utils/thread_utils.c Reset
    /// Starts the worker thread when the worker is idle, or syncs it when it is
    /// working. Clears the error flag first, as C does. Returns `false` when no
    /// thread can be started; the status then stays `NOT_OK` (C: `Reset`).
    #[must_use]
    pub fn reset(&mut self) -> bool {
        let mut state = self.shared.lock();
        state.had_error = false;
        match state.status.cmp(&WebPWorkerStatus::OK) {
            Ordering::Less => {
                // The lock stays held while the thread starts, as in C, so the new
                // thread sees the `OK` status when it first takes the lock.
                state.status = WebPWorkerStatus::OK;
                let shared = Arc::clone(&self.shared);
                if let Ok(handle) = std::thread::Builder::new().spawn(move || thread_loop(&shared))
                {
                    self.thread = Some(handle);
                    true
                } else {
                    state.status = WebPWorkerStatus::NOT_OK;
                    false
                }
            }
            Ordering::Greater => {
                drop(state);
                self.sync()
            }
            Ordering::Equal => true,
        }
    }
}

impl<D1, D2> Drop for WebPWorker<D1, D2> {
    fn drop(&mut self) {
        // C leaves the thread running unless the caller calls `End`; the Rust
        // worker ends it so that dropping a worker never leaks a thread.
        self.end();
    }
}

/// Runs the hook of the job in `state`, recording a failure (C: `Execute` on the
/// thread). A panicking hook counts as a failed hook, so that `sync` returns
/// instead of waiting for a status that never arrives.
// upstream: src/utils/thread_utils.c Execute
fn run_job<D1, D2>(state: &mut State<D1, D2>) {
    let State { had_error, job, .. } = state;
    let Some(job) = job.as_mut() else {
        return;
    };
    let ok = panic::catch_unwind(AssertUnwindSafe(|| {
        (job.hook)(&mut job.data1, &mut job.data2)
    }))
    .unwrap_or(false);
    if !ok {
        *had_error = true;
    }
}

// upstream: src/utils/thread_utils.c ThreadLoop
/// The loop of a worker thread: it waits while the worker is idle, runs the job
/// when the status is `WORK`, and returns when the status is `NOT_OK`.
fn thread_loop<D1, D2>(shared: &Shared<D1, D2>) {
    let mut done = false;
    while !done {
        let mut state = shared.lock();
        while state.status == WebPWorkerStatus::OK {
            state = shared.wait(state);
        }
        if state.status == WebPWorkerStatus::WORK {
            run_job(&mut state);
            state.status = WebPWorkerStatus::OK;
        } else if state.status == WebPWorkerStatus::NOT_OK {
            done = true;
        }
        drop(state);
        shared.changed.notify_all();
    }
}
