//! Bounded, RAII-tracked engine buffers and resize-plan admission.
//!
//! The limit covers engine-managed buffers and a conservative reservation for
//! resize planning. It does not claim to include all graph or runtime metadata,
//! decoder internals, OS allocations, or the caller-owned output sink. A shared
//! atomic counter allows concurrent plans to honor the same managed budget.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::error::Error;

/// Cache-line-padded wrapper to prevent false sharing between frequently
/// mutated atomic counters and read-only configuration. 128 bytes matches
/// Apple Silicon M-series cache lines and is a multiple of x86-64's 64-byte
/// cache lines.
#[repr(align(128))]
#[derive(Debug)]
struct CachePadded<T>(T);

#[derive(Debug)]
struct State {
    limit: CachePadded<usize>,
    live: CachePadded<AtomicUsize>,
    peak: CachePadded<AtomicUsize>,
    allocations: CachePadded<AtomicUsize>,
    allocated_bytes: CachePadded<AtomicUsize>,
}

#[derive(Clone, Debug)]
pub(crate) struct MemoryBudget {
    state: Arc<State>,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    not(test),
    allow(dead_code, reason = "diagnostic counters asserted by tests")
)]
pub(crate) struct MemorySnapshot {
    pub limit: usize,
    pub live: usize,
    pub peak: usize,
    pub allocations: usize,
    pub allocated_bytes: usize,
}

impl MemoryBudget {
    pub(crate) fn new(limit: usize) -> Self {
        Self {
            state: Arc::new(State {
                limit: CachePadded(limit),
                live: CachePadded(AtomicUsize::new(0)),
                peak: CachePadded(AtomicUsize::new(0)),
                allocations: CachePadded(AtomicUsize::new(0)),
                allocated_bytes: CachePadded(AtomicUsize::new(0)),
            }),
        }
    }

    pub(crate) fn snapshot(&self) -> MemorySnapshot {
        let live = self.state.live.0.load(Ordering::Acquire);
        let peak = self.state.peak.0.load(Ordering::Acquire).max(live);
        MemorySnapshot {
            limit: self.state.limit.0,
            live,
            peak,
            allocations: self.state.allocations.0.load(Ordering::Relaxed),
            allocated_bytes: self.state.allocated_bytes.0.load(Ordering::Relaxed),
        }
    }

    /// Remaining managed bytes currently available under the budget limit.
    pub(crate) fn available(&self) -> usize {
        self.state
            .limit
            .0
            .saturating_sub(self.state.live.0.load(Ordering::Acquire))
    }

    pub(crate) fn allocate(&self, bytes: usize) -> Result<ManagedBytes, Error> {
        self.allocate_typed(bytes)
    }

    /// Account for typed row storage (e.g. f32) as well as bytes.
    pub(crate) fn allocate_typed<T: Default + Clone>(
        &self,
        elements: usize,
    ) -> Result<ManagedBuffer<T>, Error> {
        let bytes = elements
            .checked_mul(std::mem::size_of::<T>())
            .ok_or(Error::GeometryOverflow)?;
        let reservation = self.reserve_bytes(bytes)?;
        let mut data = Vec::new();
        data.try_reserve_exact(elements)
            .map_err(|_| Error::AllocationFailed)?;
        data.resize(elements, T::default());
        self.state.allocations.0.fetch_add(1, Ordering::Relaxed);
        self.state
            .allocated_bytes
            .0
            .fetch_add(bytes, Ordering::Relaxed);
        Ok(ManagedBuffer { data, reservation })
    }

    /// Allocate a budgeted buffer initialized directly from `src`, avoiding a
    /// redundant zero-fill before copying.
    pub(crate) fn allocate_copied<T: Copy>(&self, src: &[T]) -> Result<ManagedBuffer<T>, Error> {
        let bytes = src
            .len()
            .checked_mul(std::mem::size_of::<T>())
            .ok_or(Error::GeometryOverflow)?;
        let reservation = self.reserve_bytes(bytes)?;
        let mut data = Vec::new();
        data.try_reserve_exact(src.len())
            .map_err(|_| Error::AllocationFailed)?;
        data.extend_from_slice(src);
        self.state.allocations.0.fetch_add(1, Ordering::Relaxed);
        self.state
            .allocated_bytes
            .0
            .fetch_add(bytes, Ordering::Relaxed);
        Ok(ManagedBuffer { data, reservation })
    }

    /// Charge memory that a component allocates on the engine's request and
    /// whose size the engine bounds, such as an encoder frame or resize plan.
    /// The reservation is released when the returned value is dropped.
    pub(crate) fn reserve_external(&self, bytes: usize) -> Result<ExternalReservation, Error> {
        Ok(ExternalReservation {
            reservation: self.reserve_bytes(bytes)?,
        })
    }

    fn reserve_bytes(&self, bytes: usize) -> Result<Reservation, Error> {
        let mut live = self.state.live.0.load(Ordering::Acquire);
        let limit = self.state.limit.0;
        let next = loop {
            let available = limit.saturating_sub(live);
            if bytes > available {
                return Err(Error::BudgetExceeded {
                    requested: bytes,
                    available,
                });
            }
            let next = live.checked_add(bytes).ok_or(Error::GeometryOverflow)?;
            match self.state.live.0.compare_exchange_weak(
                live,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break next,
                Err(observed) => live = observed,
            }
        };

        // The reservation, not the operating system allocation, defines
        // our managed-memory high-water mark. `peak` is monotonically
        // non-decreasing, so a shared Acquire read avoids exclusive RMW
        // cache-line bouncing once steady-state peak is reached.
        if next > self.state.peak.0.load(Ordering::Acquire) {
            self.state.peak.0.fetch_max(next, Ordering::AcqRel);
        }

        // On allocation failure the reservation automatically releases its bytes.
        Ok(Reservation {
            state: Arc::clone(&self.state),
            bytes,
        })
    }
}

/// Allocate planning metadata fallibly. The caller must hold its planning
/// reservation for at least as long as the returned vector is alive.
pub(crate) fn try_vec<T>(capacity: usize) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| Error::AllocationFailed)?;
    Ok(values)
}

pub(crate) fn try_filled_vec<T: Clone>(count: usize, value: T) -> Result<Vec<T>, Error> {
    let mut values = try_vec(count)?;
    values.resize(count, value);
    Ok(values)
}

/// Budgeted bytes owned by another component (see `reserve_external`).
#[derive(Debug)]
pub(crate) struct ExternalReservation {
    // Held for its `Drop`, which releases the bytes.
    #[allow(dead_code)]
    reservation: Reservation,
}

#[derive(Debug)]
struct Reservation {
    state: Arc<State>,
    bytes: usize,
}

impl Drop for Reservation {
    fn drop(&mut self) {
        self.state.live.0.fetch_sub(self.bytes, Ordering::AcqRel);
    }
}

pub(crate) type ManagedBytes = ManagedBuffer<u8>;

pub(crate) struct ManagedBuffer<T> {
    // Released before `reservation` (field drop order) to avoid prematurely
    // advertising available memory while the allocation is still live.
    data: Vec<T>,
    reservation: Reservation,
}

impl<T> ManagedBuffer<T> {
    pub(crate) fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> std::fmt::Debug for ManagedBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManagedBytes")
            .field("len", &self.data.len())
            .field("reservation", &self.reservation.bytes)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{CachePadded, MemoryBudget, State};
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn budget_atomics_are_cache_line_padded() {
        assert_eq!(std::mem::align_of::<CachePadded<AtomicUsize>>(), 128);
        assert_eq!(std::mem::align_of::<CachePadded<usize>>(), 128);
        assert!(std::mem::size_of::<State>() >= 5 * 128);
    }

    #[test]
    fn allocate_copied_preserves_bytes_and_accounting() {
        let budget = MemoryBudget::new(1024);
        let src = [10_u8, 20, 30, 40, 50];
        let buf = budget.allocate_copied(&src).unwrap();
        assert_eq!(buf.as_slice(), &src);
        let snap = budget.snapshot();
        assert_eq!(snap.live, 5);
        assert_eq!(snap.peak, 5);
        assert_eq!(snap.allocations, 1);
        assert_eq!(snap.allocated_bytes, 5);
        drop(buf);
        assert_eq!(budget.snapshot().live, 0);
        assert_eq!(budget.snapshot().peak, 5);
    }

    #[test]
    fn concurrent_reservations_keep_peak_exact_and_monotonic() {
        let budget = MemoryBudget::new(1 << 20);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                let budget = budget.clone();
                scope.spawn(move || {
                    for _ in 0..200 {
                        let a = budget.allocate_typed::<u32>(64).unwrap();
                        let b = budget.allocate_copied(&[1_u8; 128]).unwrap();
                        let snap = budget.snapshot();
                        assert!(snap.peak >= snap.live);
                        assert!(snap.peak <= snap.limit);
                        drop(a);
                        drop(b);
                    }
                });
            }
        });
        let final_snap = budget.snapshot();
        assert_eq!(final_snap.live, 0);
        assert!(final_snap.peak >= 64 * 4 + 128);
        assert_eq!(final_snap.allocations, 8 * 200 * 2);
    }
}
