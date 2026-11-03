// Copyright 2009, 2010, 2013-2016 by the Brotli Authors.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

use jxl::brotli::memory::Allocator;
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;

#[derive(Debug, PartialEq)]
enum Event {
    Allocate(usize, usize, bool),
    Free(usize, Vec<u8>),
}

#[derive(Default)]
struct Harness {
    calls: usize,
    fail: usize,
    blocks: Vec<(*mut u8, usize, Layout)>,
    events: Vec<Event>,
}

unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: allocator() supplies an exclusive, live Harness pointer; no
    // decoder borrow touches the harness while the callback runs.
    let h = unsafe { &mut *opaque.cast::<Harness>() };
    h.calls += 1;
    let id = h.calls;
    if id == h.fail {
        h.events.push(Event::Allocate(id, size, false));
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: layout is valid and nonzero, and this test retains the matching
    // layout until exactly one free callback consumes this address.
    let ptr = unsafe { alloc(layout) };
    assert!(!ptr.is_null(), "test host allocation");
    // SAFETY: The allocation covers size bytes and no references to it exist.
    unsafe { ptr.write_bytes(0xa5, size) };
    h.blocks.push((ptr, id, layout));
    h.events.push(Event::Allocate(id, size, true));
    ptr.cast()
}

unsafe extern "C" fn free(opaque: *mut c_void, ptr: *mut c_void) {
    // SAFETY: Same stable, exclusive callback context as allocate().
    let h = unsafe { &mut *opaque.cast::<Harness>() };
    if ptr.is_null() {
        h.events.push(Event::Free(0, vec![]));
        return;
    }
    let index = h
        .blocks
        .iter()
        .position(|b| b.0.cast::<c_void>() == ptr)
        .unwrap();
    let (ptr, id, layout) = h.blocks.remove(index);
    let size = h
        .events
        .iter()
        .find_map(|event| match event {
            Event::Allocate(i, size, true) if *i == id => Some(*size),
            _ => None,
        })
        .unwrap();
    // SAFETY: The allocator initialized every byte, writes retain initialized
    // bytes, and all owner views ended before the free callback. This is the
    // live allocation's exact size, including the zero-length case.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, size) }.to_vec();
    h.events.push(Event::Free(id, bytes));
    // SAFETY: The block's unique owner invokes this callback once, and the
    // producing allocation layout is retained unchanged in the harness.
    unsafe { dealloc(ptr, layout) };
}

fn allocator(h: &mut Harness) -> Allocator {
    // SAFETY: Each test retains the harness until all allocator copies and
    // blocks are dropped. The callbacks provide exclusive aligned blocks,
    // accept null frees, and free using their original layout exactly once.
    unsafe { Allocator::from_callbacks(Some(allocate), Some(free), (h as *mut Harness).cast()) }
        .unwrap()
}

#[test]
fn callback_pair_validation_has_no_calls() {
    let mut h = Harness::default();
    // SAFETY: Neither invalid pair can call a callback or retain the context.
    unsafe {
        assert!(
            Allocator::from_callbacks(Some(allocate), None, (&mut h as *mut Harness).cast())
                .is_none()
        );
        assert!(
            Allocator::from_callbacks(None, Some(free), (&mut h as *mut Harness).cast()).is_none()
        );
    }
    assert!(h.events.is_empty());
}

#[test]
fn callback_failure_and_null_free_are_separate_checkpoints() {
    let mut h = Harness {
        fail: 1,
        ..Harness::default()
    };
    let a = allocator(&mut h);
    assert!(a.allocate(123).is_none());
    assert_eq!(h.events, [Event::Allocate(1, 123, false)]);
    a.release(None);
    assert_eq!(
        h.events,
        [Event::Allocate(1, 123, false), Event::Free(0, vec![])]
    );
    assert!(h.blocks.is_empty());
}

#[test]
fn returned_block_is_the_payload_and_unwritten_bytes_survive() {
    let mut h = Harness::default();
    let a = allocator(&mut h);
    let mut first = a.allocate(32).unwrap();
    let mut second = a.allocate(9).unwrap();
    assert_eq!(first.as_ptr(), h.blocks[0].0);
    assert_eq!(second.as_ptr(), h.blocks[1].0);
    first.storage_mut()[8].write(0x12);
    first.storage_mut()[31].write(0x34);
    second.fill(0);
    // SAFETY: This custom allocator initialized the entire actual block.
    assert_eq!(unsafe { first.initialized_bytes(8..10) }, [0x12, 0xa5]);
    // A Rust move transfers this same block; cleanup order remains explicit.
    let moved = first;
    a.release(None);
    a.release(Some(moved));
    a.release(Some(second));
    let mut expected = vec![0xa5; 32];
    expected[8] = 0x12;
    expected[31] = 0x34;
    assert_eq!(
        h.events,
        [
            Event::Allocate(1, 32, true),
            Event::Allocate(2, 9, true),
            Event::Free(0, vec![]),
            Event::Free(1, expected),
            Event::Free(2, vec![0; 9])
        ]
    );
    assert!(h.blocks.is_empty());
}

#[test]
fn zero_size_callback_keeps_original_request_and_free_identity() {
    let mut h = Harness::default();
    let a = allocator(&mut h);
    let block = a.allocate(0).unwrap();
    assert!(block.is_empty());
    assert_eq!(block.as_ptr(), h.blocks[0].0);
    a.release(Some(block));
    assert_eq!(
        h.events,
        [Event::Allocate(1, 0, true), Event::Free(1, vec![])]
    );
}

#[test]
fn default_allocator_ignores_opaque_and_owns_writable_storage() {
    let mut h = Harness::default();
    // SAFETY: Default selection retains no pointer to opaque and has no
    // custom callback lifetime obligation.
    let a =
        unsafe { Allocator::from_callbacks(None, None, (&mut h as *mut Harness).cast()) }.unwrap();
    let mut block = a.allocate(64).unwrap();
    assert_eq!(block.as_ptr() as usize % 16, 0);
    block.fill(0x7b);
    // SAFETY: fill initialized every byte of the allocation.
    assert_eq!(unsafe { block.initialized_bytes(0..64) }, [0x7b; 64]);
    a.release(Some(block));
    a.release(None);
    assert!(h.events.is_empty());
}
