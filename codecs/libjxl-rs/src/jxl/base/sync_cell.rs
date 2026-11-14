// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! A `RefCell` that may be shared between threads (port ADR 0003,
//! "Concurrent decoder tasks"). Owners that the decoder writes between
//! render calls and render stages read while several threads render (the
//! EPF sigma image) use it with `Arc` in place of `Rc<RefCell>`. It keeps
//! `RefCell`'s contract: a conflicting borrow panics instead of waiting, so
//! a borrow conflict that the single-threaded representation would report
//! is still reported, never turned into a wait.

/// `RefCell` semantics over an `RwLock`: any number of shared borrows, or
/// one mutable borrow; a conflicting borrow panics.
#[derive(Default)]
pub struct SyncRefCell<T> {
    value: std::sync::RwLock<T>,
}
impl<T> SyncRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: std::sync::RwLock::new(value),
        }
    }
    pub fn into_inner(self) -> T {
        self.value
            .into_inner()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.value
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
    /// A shared borrow; panics if the value is mutably borrowed.
    pub fn borrow(&self) -> std::sync::RwLockReadGuard<'_, T> {
        match self.value.try_read() {
            Ok(guard) => guard,
            Err(std::sync::TryLockError::Poisoned(e)) => e.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => panic!("already mutably borrowed"),
        }
    }
    /// A mutable borrow; panics if the value is borrowed.
    pub fn borrow_mut(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        match self.value.try_write() {
            Ok(guard) => guard,
            Err(std::sync::TryLockError::Poisoned(e)) => e.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => panic!("already borrowed"),
        }
    }
}

/// The decoder's EPF sigma image, shared with its EPF stages.
pub type SharedSigma = std::sync::Arc<SyncRefCell<crate::jxl::image::Plane<f32>>>;
