// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The parallel runners of `lib/threads`: the fixed-size thread runner and
//! the resizable runner, with the public functions of
//! `lib/include/jxl/thread_parallel_runner.h` and
//! `lib/include/jxl/resizable_parallel_runner.h`.
//!
//! The runners use `std::thread` and the `std::sync` primitives in place of
//! their C++ standard library counterparts; no C or C++ code is compiled or
//! linked. `abi` exports the public functions under their original C names.

pub mod abi;
pub mod resizable_parallel_runner;
pub mod thread_parallel_runner;
pub mod thread_parallel_runner_internal;

/// std::thread::hardware_concurrency: the number of concurrent threads the
/// system supports, or 0 when that is unknown.
///
/// Language-required deviation: Rust's standard library exposes
/// `available_parallelism`, which also honors the process's CPU affinity and,
/// on Linux, its cgroup quota. libc++ reports the online processors instead,
/// so the two values can differ on a restricted host. Only the default and
/// suggested thread counts read it; decoded outputs do not depend on them.
pub(crate) fn hardware_concurrency() -> u32 {
    std::thread::available_parallelism().map_or(0, |n| u32::try_from(n.get()).unwrap_or(u32::MAX))
}
