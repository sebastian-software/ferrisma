// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/dictionary.rs"]
pub mod dictionary;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/raw_bytes.rs"]
pub mod raw_bytes;
#[path = "../src/brotli/raw_transform.rs"]
pub mod raw_transform;
#[path = "../src/brotli/tables.rs"]
pub mod tables;
#[path = "../src/brotli/transform.rs"]
pub mod transform;

use raw_transform::RawByteDestination;
use std::mem::MaybeUninit;
use transform::ByteDestination;

#[test]
fn byte_xor_retains_every_initialized_value() {
    for value in 0..=255u8 {
        for mask in [0, 5, 32, 255] {
            let mut byte = MaybeUninit::new(value);
            raw_bytes::xor_byte(&mut byte, mask);
            // SAFETY: The initial byte was initialized and XOR preserves it.
            assert_eq!(unsafe { byte.assume_init() }, value ^ mask);
        }
    }
}

#[test]
fn unknown_tail_xor_does_not_claim_an_initialized_byte() {
    let mut owner = memory::Allocator::default().allocate(64).unwrap();
    let mut dst = RawByteDestination::new(owner.storage_mut());
    let dictionary = dictionary::brotli_get_dictionary();
    let offset = dictionary.offsets_by_length[4] as usize + 436 * 4;
    let len = transform::brotli_transform_dictionary_word_to(
        &mut dst,
        &dictionary.data[offset..offset + 4],
        4,
        transform::brotli_get_transforms(),
        44,
    );
    assert_eq!(len, 4);
    assert_eq!(dst.initialized_prefix_len(), 4);
    assert_eq!(dst.initialized_prefix(), &[b'Z', b'H', b':', 0xe5]);
    // Byte5 was XORed by the original operation. It remains outside the
    // initialized prefix and is intentionally never inspected as a Rust u8.
}

#[test]
fn allocator_seeded_tail_and_same_block_identity_are_retained() {
    use std::ffi::c_void;
    struct Context {
        block: [MaybeUninit<u8>; 64],
        allocation: *mut c_void,
        free_calls: usize,
    }
    unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
        // SAFETY: This test passes its exclusive live Context as opaque; the
        // callback is synchronous and never retains a reference to Context.
        let context = unsafe { &mut *opaque.cast::<Context>() };
        assert_eq!(size, 64);
        for byte in &mut context.block {
            byte.write(0xa5);
        }
        let ptr = context.block.as_mut_ptr().cast();
        context.allocation = ptr;
        ptr
    }
    unsafe extern "C" fn free(opaque: *mut c_void, ptr: *mut c_void) {
        // SAFETY: The context remains live until the sole owner is dropped.
        let context = unsafe { &mut *opaque.cast::<Context>() };
        assert_eq!(ptr, context.allocation);
        context.free_calls += 1;
    }
    let mut context = Context {
        block: [MaybeUninit::uninit(); 64],
        allocation: std::ptr::null_mut(),
        free_calls: 0,
    };
    // SAFETY: The pair returns the actual live64-byte Context block, accepts
    // its exact pointer once, does not unwind/reenter/access borrowed storage,
    // and the context outlives the allocation and all views.
    let allocator = unsafe {
        memory::Allocator::from_callbacks(
            Some(allocate),
            Some(free),
            (&mut context as *mut Context).cast(),
        )
        .unwrap()
    };
    let mut owner = allocator.allocate(64).unwrap();
    assert_eq!(
        owner.as_ptr().cast::<c_void>(),
        context.allocation.cast_const()
    );
    {
        // SAFETY: The allocation callback initialized all64 actual bytes.
        let mut dst =
            unsafe { RawByteDestination::with_initialized_prefix(owner.storage_mut(), 64) };
        let dictionary = dictionary::brotli_get_dictionary();
        let offset = dictionary.offsets_by_length[4] as usize + 436 * 4;
        let len = transform::brotli_transform_dictionary_word_to(
            &mut dst,
            &dictionary.data[offset..offset + 4],
            4,
            transform::brotli_get_transforms(),
            44,
        );
        assert_eq!(len, 4);
        assert_eq!(
            &dst.initialized_prefix()[..7],
            &[b'Z', b'H', b':', 0xe5, 0xa5, 0xa0, 0xa5]
        );
    }
    drop(owner);
    assert_eq!(context.free_calls, 1);
    // SAFETY: The callback initialized all bytes and the owner's borrow ended.
    assert_eq!(unsafe { context.block[5].assume_init() }, 0xa0);
}

#[test]
#[should_panic]
fn unknown_tail_cannot_be_read_as_a_scalar() {
    let mut block = [MaybeUninit::uninit(); 8];
    let mut dst = RawByteDestination::new(&mut block);
    dst.write(0, 0xe5);
    dst.xor(2, 5);
    let _ = dst.read(2);
}

#[test]
#[should_panic]
fn a_write_cannot_claim_an_unwritten_gap_initialized() {
    let mut block = [MaybeUninit::uninit(); 8];
    let mut dst = RawByteDestination::new(&mut block);
    dst.write(2, 7);
}
