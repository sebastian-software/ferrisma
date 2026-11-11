//! Shared scoped-worker admission; thread stacks remain outside `MemoryBudget`.

use std::io;
use std::thread::{self, Scope, ScopedJoinHandle};

pub(crate) const MAX_WORKERS: usize = 256;

/// Create a scoped worker without panicking when the OS cannot create a thread.
pub(crate) fn spawn<'scope, 'env, F>(
    scope: &'scope Scope<'scope, 'env>,
    worker: F,
) -> io::Result<ScopedJoinHandle<'scope, ()>>
where
    F: FnOnce() + Send + 'scope,
{
    #[cfg(test)]
    if FAIL_AFTER.with(|remaining| match remaining.get() {
        Some(0) => true,
        Some(n) => {
            remaining.set(Some(n - 1));
            false
        }
        None => false,
    }) {
        return Err(io::Error::other("injected scoped thread spawn failure"));
    }
    thread::Builder::new().spawn_scoped(scope, worker)
}

#[cfg(test)]
thread_local! {
    // Confined to the driver thread so concurrent tests do not share a hook.
    static FAIL_AFTER: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}

/// Fail after `successful` scoped spawns on the calling test thread.
#[cfg(test)]
pub(crate) fn fail_after(successful: usize) -> FailGuard {
    let previous = FAIL_AFTER.with(|limit| limit.replace(Some(successful)));
    FailGuard(previous)
}

#[cfg(test)]
pub(crate) struct FailGuard(Option<usize>);

#[cfg(test)]
impl Drop for FailGuard {
    fn drop(&mut self) {
        FAIL_AFTER.with(|limit| limit.set(self.0));
    }
}
