// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#![allow(dead_code)]
#[path = "../src/brotli/bit_reader.rs"]
mod bit_reader;
#[path = "../src/brotli/bitfield.rs"]
mod bitfield;
#[path = "../src/brotli/default_memory.rs"]
mod default_memory;
#[path = "../src/brotli/layout.rs"]
mod layout;
#[path = "../src/brotli/memory.rs"]
mod memory;
#[path = "../src/brotli/running.rs"]
mod running;
#[path = "../src/brotli/shared_dictionary.rs"]
mod shared_dictionary;
#[path = "../src/brotli/state.rs"]
mod state;
#[path = "../src/brotli/static_descriptors.rs"]
mod static_descriptors;
#[path = "../src/brotli/tables.rs"]
mod tables;

#[path = "../src/brotli/compound_dictionary.rs"]
mod compound_dictionary;
#[path = "../src/brotli/ringbuffer.rs"]
mod ringbuffer;
#[path = "../src/brotli/status.rs"]
mod status;
use compound_dictionary::CompoundDictionaryBindings;
use layout::{CompoundDictionaryStorage, DecoderStateStorage};
use std::mem::size_of;
fn original_state<'a>() -> state::StateOwner<'a> {
    let allocator = default_memory::allocator();
    let (allocate, free) = default_memory::callbacks();
    state::StateOwner::initialize(
        allocator
            .allocate(size_of::<DecoderStateStorage>())
            .unwrap(),
        allocator,
        allocator,
        allocate as *const () as usize,
        free as *const () as usize,
        0,
        0,
    )
    .unwrap()
}
fn addon<'a>(owner: &'a state::StateOwner<'_>) -> &'a CompoundDictionaryStorage {
    let allocation = owner.compound_dictionary.as_ref().unwrap();
    assert_eq!(allocation.len(), size_of::<CompoundDictionaryStorage>());
    // SAFETY: Same actual native allocation; every field is MaybeUninit.
    unsafe { &*allocation.as_ptr().cast() }
}
#[test]
fn empty_addon_copy_never_reads_unknown_index_offsets_or_ring_payload() {
    let mut s = original_state();
    let mut bindings = CompoundDictionaryBindings::default();
    // SAFETY: Source NULL dictionary is the only reached scalar read.
    assert_eq!(unsafe { s.compound_dictionary_size() }, 0);
    // SAFETY: Actual source attachment, empty non-NULL caller-owned prefix.
    assert!(unsafe { s.attach_dictionary(&mut bindings, 0, Some(&[])) });
    // SAFETY: Addon Init initialized only br_length/br_copied, both zero.
    // br_index/br_offset/map and the entire absent ring remain unknown/unread.
    assert_eq!(unsafe { s.copy_from_compound_dictionary(&bindings, 17) }, 0);
}
#[test]
fn failed_copy_bounds_keep_distance_unknown_and_map_tail_uninitialized() {
    let bytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut s = original_state();
    let mut bindings = CompoundDictionaryBindings::default();
    // SAFETY: Live retained prefix, no prior addon or signed overflow.
    assert!(unsafe { s.attach_dictionary(&mut bindings, 0, Some(&bytes)) });
    // SAFETY: Valid address/maps; failed total-size check occurs before the
    // still-unknown StateInit distance_code is read or br_index is written.
    assert!(!unsafe { s.initialize_compound_dictionary_copy(0, 11) });
    // SAFETY: Exactly these scalar fields were initialized by reached writes.
    assert_eq!(unsafe { addon(&s).block_bits.assume_init() }, 0);
    assert_eq!(unsafe { addon(&s).br_length.assume_init() }, 0);
    for i in 0..10 {
        // SAFETY: Only the reached first10 map entries were initialized; the
        // rest of the actual256-byte map is not read or promised initialized.
        assert_eq!(
            unsafe { addon(&s).block_map.as_ptr().cast::<u8>().add(i).read() },
            0
        );
    }
    // SAFETY: Zero-copy short circuit still avoids the unknown index/offset.
    assert_eq!(unsafe { s.copy_from_compound_dictionary(&bindings, 0) }, 0);
}
#[test]
fn compound_copy_retains_exact_chunk_boundary_and_ring_space_checkpoints() {
    let a = [10, 20, 30, 40];
    let b = [1, 2, 3, 4, 5, 6];
    let mut s = original_state();
    let mut bindings = CompoundDictionaryBindings::default();
    // SAFETY: Both retained actual caller-backed prefix extents remain live.
    assert!(unsafe { s.attach_dictionary(&mut bindings, 0, Some(&a)) });
    assert!(unsafe { s.attach_dictionary(&mut bindings, 0, Some(&b)) });
    s.storage_mut().new_ringbuffer_size.write(1024);
    // SAFETY: First source ring allocation; no unknown payload read.
    assert!(unsafe { s.ensure_ringbuffer() });
    s.storage_mut().distance_code.write(99);
    s.storage_mut().meta_block_remaining_len.write(100);
    // SAFETY: Actual address3,length6 spans exactly a[3] and b[0..5].
    assert!(unsafe { s.initialize_compound_dictionary_copy(3, 6) });
    // SAFETY: One ring byte of available space causes the source chunk and
    // output-space checkpoint after exactly one copied initialized byte.
    assert_eq!(
        unsafe { s.copy_from_compound_dictionary(&bindings, 1023) },
        1
    );
    assert_eq!(unsafe { addon(&s).br_index.assume_init() }, 1);
    assert_eq!(unsafe { addon(&s).br_offset.assume_init() }, 0);
    assert_eq!(unsafe { addon(&s).br_copied.assume_init() }, 1);
    assert_eq!(unsafe { s.copy_from_compound_dictionary(&bindings, 0) }, 5);
    let ring = s.ringbuffer.as_ref().unwrap();
    // SAFETY: Only these exact copied ranges became initialized in the ring.
    assert_eq!(unsafe { ring.initialized_bytes(1023..1024) }, [40]);
    assert_eq!(unsafe { ring.initialized_bytes(0..5) }, [1, 2, 3, 4, 5]);
    // SAFETY: Source Copy does not mutate state.pos; InitCopy wrote these
    // initialized distance/accounting fields before any ring copy.
    assert_eq!(unsafe { s.storage().pos.assume_init() }, 0);
    assert_eq!(unsafe { s.storage().dist_rb_idx.assume_init() }, 1);
    assert_eq!(
        unsafe { s.storage().meta_block_remaining_len.assume_init() },
        94
    );
}
#[test]
fn lazy_map_uses_actual_offsets_at_zero_length_chunk_boundaries() {
    let a = [10, 20, 30];
    let b = [7, 8, 9];
    let mut s = original_state();
    let mut bindings = CompoundDictionaryBindings::default();
    // SAFETY: Empty prefixes are valid attachments. This copy begins after
    // them, so original index selection skips their zero-length intervals.
    for prefix in [
        Some([].as_slice()),
        Some(a.as_slice()),
        Some([].as_slice()),
        Some(b.as_slice()),
    ] {
        assert!(unsafe { s.attach_dictionary(&mut bindings, 0, prefix) });
    }
    s.storage_mut().distance_code.write(13);
    s.storage_mut().meta_block_remaining_len.write(3);
    // SAFETY: Address3 is the first actual byte in the fourth attached chunk.
    assert!(unsafe { s.initialize_compound_dictionary_copy(3, 3) });
    assert_eq!(unsafe { addon(&s).br_index.assume_init() }, 3);
    assert_eq!(unsafe { addon(&s).br_offset.assume_init() }, 0);
}

struct Block {
    pointer: *mut u8,
    size: usize,
    live: bool,
}
struct AllocationTrace {
    calls: usize,
    blocks: Vec<Block>,
}
unsafe extern "C" fn failing_allocate(
    opaque: *mut std::ffi::c_void,
    size: usize,
) -> *mut std::ffi::c_void {
    // SAFETY: This native fixture context outlives every decoder allocation;
    // it is accessed only during the source allocation/free checkpoints.
    let c = unsafe { &mut *opaque.cast::<AllocationTrace>() };
    c.calls += 1;
    if c.calls == 3 {
        return std::ptr::null_mut();
    }
    // SAFETY: Nonzero native-aligned actual backing; no payload shadow or
    // blanket initialization is added to the source's requested allocation.
    let pointer =
        unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(size.max(1), 16).unwrap()) };
    assert!(!pointer.is_null());
    c.blocks.push(Block {
        pointer,
        size,
        live: true,
    });
    pointer.cast()
}
unsafe extern "C" fn traced_free(opaque: *mut std::ffi::c_void, pointer: *mut std::ffi::c_void) {
    // SAFETY: Same live callback context contract as failing_allocate.
    let c = unsafe { &mut *opaque.cast::<AllocationTrace>() };
    if pointer.is_null() {
        return;
    }
    let b = c
        .blocks
        .iter_mut()
        .find(|b| b.pointer.cast::<std::ffi::c_void>() == pointer && b.live)
        .unwrap();
    b.live = false;
    // SAFETY: Exact pointer/layout from the corresponding fixture allocation;
    // all source-storage borrows ended before the actual free checkpoint.
    unsafe {
        std::alloc::dealloc(
            b.pointer,
            std::alloc::Layout::from_size_align(b.size.max(1), 16).unwrap(),
        )
    };
}
#[test]
fn failed_addon_allocation_retains_shared_prefix_but_next_addon_starts_with_new_prefix() {
    let first = [1, 2];
    let second = [7, 8, 9];
    let mut trace = AllocationTrace {
        calls: 0,
        blocks: Vec::new(),
    };
    let opaque = (&mut trace as *mut AllocationTrace).cast::<std::ffi::c_void>();
    // SAFETY: Complete actual producing pair/context retained until final free.
    let allocator = unsafe {
        memory::Allocator::from_callbacks(Some(failing_allocate), Some(traced_free), opaque)
    }
    .unwrap();
    let allocation = allocator
        .allocate(size_of::<DecoderStateStorage>())
        .unwrap();
    let mut s = state::StateOwner::initialize(
        allocation,
        allocator,
        allocator,
        failing_allocate as *const () as usize,
        traced_free as *const () as usize,
        opaque as usize,
        opaque as usize,
    )
    .unwrap();
    let mut bindings = CompoundDictionaryBindings::default();
    // SAFETY: The third actual allocation is addon OOM after shared attach.
    assert!(!unsafe { s.attach_dictionary(&mut bindings, 0, Some(&first)) });
    assert_eq!(
        unsafe {
            s.dictionary
                .as_ref()
                .unwrap()
                .storage()
                .num_prefix
                .assume_init()
        },
        1
    );
    assert!(s.compound_dictionary.is_none());
    // SAFETY: Source retries with a NEW prefix; only its new shared slot is
    // attached to the new addon. The retained first prefix is not replayed.
    assert!(unsafe { s.attach_dictionary(&mut bindings, 0, Some(&second)) });
    assert_eq!(
        unsafe {
            s.dictionary
                .as_ref()
                .unwrap()
                .storage()
                .num_prefix
                .assume_init()
        },
        2
    );
    assert_eq!(unsafe { addon(&s).num_chunks.assume_init() }, 1);
    assert_eq!(unsafe { s.compound_dictionary_size() }, 3);
    // SAFETY: Only source-initialized chunk0, not its unknown15 siblings.
    assert_eq!(
        unsafe { addon(&s).chunks.as_ptr().cast::<usize>().read() },
        second.as_ptr() as usize
    );
    s.storage_mut().distance_code.write(42);
    s.storage_mut().meta_block_remaining_len.write(3);
    assert!(unsafe { s.initialize_compound_dictionary_copy(0, 3) });
    s.storage_mut().new_ringbuffer_size.write(1024);
    assert!(unsafe { s.ensure_ringbuffer() });
    assert_eq!(unsafe { s.copy_from_compound_dictionary(&bindings, 0) }, 3);
    // SAFETY: Exactly these bytes were initialized by the reached source copy.
    assert_eq!(
        unsafe { s.ringbuffer.as_ref().unwrap().initialized_bytes(0..3) },
        second
    );
    drop(s);
    assert!(trace.blocks.iter().all(|b| !b.live));
}

#[test]
fn actual_raw_caller_mutation_and_late_initialization_match_original_captures() {
    // Actual pristine C ordinary/ASan observations retained in
    // tools/oracle/brotli_compound_dictionary.c: M0/M1/M2. Both native pointers
    // remain the original allocation; no copied attachment payload exists.
    use shared_dictionary::RawDictionaryView;
    use std::mem::MaybeUninit;
    let expected: [&[u8]; 3] = [
        &[91, 2, 3, 4, 5, 66],
        &[11, 12, 13, 14, 15, 16],
        &[1, 2, 3, 4, 5, 6],
    ];
    for (mode, expected) in expected.iter().enumerate() {
        let mut caller = [MaybeUninit::<u8>::uninit(); 6];
        let pointer = caller.as_mut_ptr().cast::<u8>();
        // All later caller writes use this same derived raw pointer. No byte
        // slice of the caller is constructed or retained across decoder calls.
        if mode != 1 {
            for i in 0..6 {
                // SAFETY: Exclusive live caller allocation, exact in-bounds
                // scalar write before any decoder read.
                unsafe { pointer.add(i).write(i as u8 + 1) };
            }
        }
        let mut s = original_state();
        let mut bindings = CompoundDictionaryBindings::default();
        // SAFETY: Actual caller allocation outlives state/bindings. Its bytes
        // may be unknown now; all reached source copy bytes are initialized
        // below before that copy. No conflicting Rust reference exists.
        let view = unsafe { RawDictionaryView::from_raw_parts(pointer, 6) };
        assert!(unsafe { s.attach_dictionary_raw(&mut bindings, 0, view) });
        if mode == 1 {
            for i in 0..6 {
                // SAFETY: Same live raw allocation, no source loan is active.
                unsafe { pointer.add(i).write(i as u8 + 11) };
            }
        } else if mode == 0 {
            // SAFETY: Caller updates the original backing after attachment;
            // the source retains only pointers, not a shared Rust reference.
            unsafe {
                pointer.write(91);
                pointer.add(5).write(66)
            };
        }
        assert_eq!(
            s.dictionary.as_ref().unwrap().prefix_view(0).as_ptr(),
            pointer
        );
        assert_eq!(
            unsafe { addon(&s).chunks.as_ptr().cast::<usize>().read() },
            pointer as usize
        );
        s.storage_mut().new_ringbuffer_size.write(1024);
        s.write_flag(6, 6, 10);
        assert!(unsafe { s.ensure_ringbuffer() });
        s.storage_mut().distance_code.write(20);
        s.storage_mut().meta_block_remaining_len.write(64);
        assert!(unsafe { s.initialize_compound_dictionary_copy(0, 6) });
        assert_eq!(unsafe { s.copy_from_compound_dictionary(&bindings, 0) }, 6);
        assert_eq!(
            unsafe { s.ringbuffer.as_ref().unwrap().initialized_bytes(0..6) },
            *expected
        );
        if mode == 2 {
            // SAFETY: First ring comparison's short reference has ended. No
            // caller byte reference exists; the next copy sees these writes.
            unsafe {
                pointer.write(91);
                pointer.add(5).write(66)
            };
            assert!(unsafe { s.initialize_compound_dictionary_copy(0, 6) });
            assert_eq!(unsafe { s.copy_from_compound_dictionary(&bindings, 6) }, 6);
            assert_eq!(
                unsafe { s.ringbuffer.as_ref().unwrap().initialized_bytes(6..12) },
                [91, 2, 3, 4, 5, 66]
            );
        }
    }
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn original_raw_caller_domain_capture_is_reproducible() {
    let binary = std::env::var("BROTLI_COMPOUND_DICTIONARY_ORACLE")
        .expect("actual pinned original compound observer path");
    let output = std::process::Command::new(binary).output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stderr.is_empty(), "{:?}", output.stderr);
    assert_eq!(output.stdout, b"M0 A1 I1 C6 P1:1 5b0203040542\nM1 A1 I1 C6 P1:1 0b0c0d0e0f10\nM2 A1 I1 C6 P1:1 010203040506 I1 C6 5b0203040542\nM3 A1 S6 P1:1\n");
}

#[test]
fn raw_null_nonzero_attachment_stores_fields_without_reaching_a_copy() {
    let mut s = original_state();
    let mut bindings = CompoundDictionaryBindings::default();
    // SAFETY: Source attachment stores this NULL/size pair only; no reached
    // pointer arithmetic, byte read or memcpy is attempted by this fixture.
    let view = unsafe { shared_dictionary::RawDictionaryView::from_raw_parts(std::ptr::null(), 6) };
    assert!(unsafe { s.attach_dictionary_raw(&mut bindings, 0, view) });
    assert_eq!(unsafe { s.compound_dictionary_size() }, 6);
    assert_eq!(
        s.dictionary.as_ref().unwrap().prefix_view(0).as_ptr(),
        std::ptr::null()
    );
    assert_eq!(
        unsafe { addon(&s).chunks.as_ptr().cast::<usize>().read() },
        0
    );
}
