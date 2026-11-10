// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Element destruction order of the reference libc++ std::vector. Its clear,
//! shrinking resize, move assignment and destructor destroy elements from the
//! last to the first. Rust's Vec drops them first to last. Owners whose
//! elements release memory-manager allocations use these helpers where the
//! source destroys vector elements, so the release order matches the original.
//! See ADR 0003.

/// A Vec whose destructor releases its elements last first, as std::vector's
/// does. It carries no lifetime of its own, so an owner can use it as a field
/// without a Drop impl of its own (which would constrain borrowed lifetimes).
#[derive(Default)]
pub struct Vector<T>(pub Vec<T>);
impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        clear(&mut self.0);
    }
}
impl<T> std::ops::Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

/// std::vector::clear and the destructor: destroy every element, last first.
pub fn clear<T>(v: &mut Vec<T>) {
    while v.pop().is_some() {}
}

/// clear for a vector of vectors: each inner vector also destroys its
/// elements last first.
pub fn clear_nested<T>(v: &mut Vec<Vec<T>>) {
    while let Some(mut inner) = v.pop() {
        clear(&mut inner);
    }
}

/// A shrinking std::vector::resize: destroy the tail, last first.
pub fn truncate<T>(v: &mut Vec<T>, len: usize) {
    while v.len() > len {
        v.pop();
    }
}

/// std::vector::erase(first, last) of elements whose move assignment releases
/// the destination's previous contents and leaves an empty source. The tail
/// is move-assigned down: the erased elements it overwrites are released in
/// ascending order. The remaining erased elements are then destroyed from the
/// end, last first; moved-from tail shells release nothing.
pub fn erase<T>(v: &mut Vec<T>, first: usize, last: usize) {
    let tail = v.len() - last;
    let mut erased: Vec<T> = v.drain(first..last).collect();
    let remaining = erased.split_off(tail.min(erased.len()));
    for element in erased {
        drop(element);
    }
    let mut remaining = remaining;
    clear(&mut remaining);
}

/// std::vector move assignment: the old elements are destroyed, last first,
/// before the source's storage is adopted.
pub fn move_assign<T>(dest: &mut Vec<T>, source: Vec<T>) {
    clear(dest);
    *dest = source;
}
