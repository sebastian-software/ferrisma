// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
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

#[path = "../src/brotli/ringbuffer.rs"]
mod ringbuffer;
#[path = "../src/brotli/status.rs"]
mod status;
use layout::DecoderStateStorage;
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::fmt::Write as _;
use std::io::{self, BufRead};
use std::mem::{MaybeUninit, size_of};

struct Block {
    pointer: *mut u8,
    size: usize,
    live: bool,
}
struct Context {
    blocks: Vec<Block>,
    calls: u32,
    fail: u32,
    seed: u8,
    state: *const DecoderStateStorage,
    trace: String,
}
impl Context {
    fn token(&self, pointer: usize) -> usize {
        if pointer == 0 {
            return 0;
        }
        self.blocks
            .iter()
            .position(|b| b.pointer as usize == pointer)
            .expect("actual source allocation identity")
            + 1
    }
    fn snapshot(&mut self) {
        if self.state.is_null() {
            return;
        }
        // SAFETY: The fixture stores the actual live state allocation, never
        // the movable sidecar. Callback entry has ended its source-field
        // loans; Create initialized these particular fields before binding.
        let (pointer, size, new_size) = unsafe {
            let state = &*self.state;
            (
                state.ringbuffer.assume_init(),
                state.ringbuffer_size.assume_init(),
                state.new_ringbuffer_size.assume_init(),
            )
        };
        write!(self.trace, ":{}:{size}:{new_size}", self.token(pointer)).unwrap();
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        for b in &self.blocks {
            assert!(!b.live);
            // SAFETY: The fixture quarantines unique aligned allocations
            // after each logical source free; this is the sole actual free.
            unsafe {
                dealloc(
                    b.pointer,
                    Layout::from_size_align(b.size.max(1), 16).unwrap(),
                )
            };
        }
    }
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The live fixture context is accessed only inside callbacks;
    // no decoder field borrow survives the actual allocation checkpoint.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    c.calls += 1;
    write!(c.trace, " A{}:{size}", c.calls).unwrap();
    c.snapshot();
    if c.calls == c.fail {
        c.trace.push_str(":0");
        return std::ptr::null_mut();
    }
    // SAFETY: A valid nonzero fixture allocation, with the source alignment.
    let pointer = unsafe { alloc(Layout::from_size_align(size.max(1), 16).unwrap()) };
    assert!(!pointer.is_null());
    // SAFETY: Fixture seeding independently initializes every observed byte;
    // the decoder itself does not perform this blanket initialization.
    unsafe { pointer.write_bytes(c.seed, size) };
    c.blocks.push(Block {
        pointer,
        size,
        live: true,
    });
    write!(c.trace, ":{}", c.token(pointer as usize)).unwrap();
    pointer.cast()
}
unsafe extern "C" fn free(opaque: *mut c_void, pointer: *mut c_void) {
    // SAFETY: The same live non-reentrant fixture context contract as allocate.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    let token = c.token(pointer as usize);
    write!(c.trace, " F{token}").unwrap();
    c.snapshot();
    if token != 0 {
        assert!(c.blocks[token - 1].live);
        c.blocks[token - 1].live = false;
    }
}
fn bytes(output: &mut String, data: &[u8]) {
    for byte in data {
        write!(output, "{byte:02x}").unwrap();
    }
}
fn flag(state: &mut state::StateOwner<'_>, shift: u32, width: u32, value: u32) {
    state.write_flag(shift, width, value);
}

pub fn observe(line: &str) -> String {
    let args: Vec<_> = line.split_whitespace().collect();
    assert_eq!(args.len(), 18);
    let numbers: Vec<i128> = args.iter().map(|s| s.parse().unwrap()).collect();
    let [
        op,
        seed,
        fail,
        window,
        canny,
        old_size,
        new_size,
        pos,
        remaining,
        rounds,
        partial,
        capacity,
        force,
        target,
        running,
        error,
        wrap,
        consumed,
    ] = <[i128; 18]>::try_from(numbers).unwrap();
    let (capacity, rounds, partial) = (capacity as usize, rounds as usize, partial as usize);
    let mut context = Context {
        blocks: Vec::new(),
        calls: 0,
        fail: fail as u32,
        seed: seed as u8,
        state: std::ptr::null(),
        trace: String::new(),
    };
    // SAFETY: The fixture callback pair and context remain live through every
    // allocation/free. Quarantined allocations are exclusive actual backing.
    let allocator = unsafe {
        memory::Allocator::from_callbacks(
            Some(allocate),
            Some(free),
            (&mut context as *mut Context).cast(),
        )
    }
    .unwrap();
    let Some(allocation) = allocator.allocate(size_of::<DecoderStateStorage>()) else {
        return std::mem::take(&mut context.trace) + " X\n";
    };
    let Some(mut owner) = state::StateOwner::initialize(
        allocation,
        allocator,
        allocator,
        allocate as *const () as usize,
        free as *const () as usize,
        (&mut context as *mut Context) as usize,
        (&mut context as *mut Context) as usize,
    ) else {
        return std::mem::take(&mut context.trace) + " X\n";
    };
    context.state = owner.as_ptr().cast();
    if old_size != 0 {
        owner
            .storage_mut()
            .new_ringbuffer_size
            .write(old_size as i32);
        // SAFETY: This fixture requests source-valid sizes>=2 on an empty
        // original state. No payload read occurs during its first allocation.
        if !unsafe { owner.ensure_ringbuffer() } {
            context.trace.push_str(" X");
            context.state = std::ptr::null();
            drop(owner);
            return std::mem::take(&mut context.trace) + "\n";
        }
    }
    let s = owner.storage_mut();
    s.new_ringbuffer_size.write(new_size as i32);
    s.pos.write(pos as i32);
    s.meta_block_remaining_len.write(remaining as i32);
    s.rb_roundtrips.write(rounds);
    s.partial_pos_out.write(partial);
    s.state.write(running as u32);
    s.error_code.write(error as i32);
    s.used_input.write(u64::MAX - 1);
    s.buffer_length.write(wrap as bit_reader::Reg);
    let buffer = s.buffer.as_ptr() as usize;
    // SAFETY: Every nested bit-reader field is MaybeUninit and the whole
    // native storage is valid; write only the two fixture input pointer slots.
    unsafe {
        (*s.br.as_mut_ptr()).next_in.write(buffer);
        (*s.br.as_mut_ptr())
            .last_in
            .write(buffer + usize::from(target != 0));
    }
    flag(&mut owner, 6, 6, window as u32);
    flag(&mut owner, 4, 1, canny as u32);
    flag(&mut owner, 3, 1, wrap as u32);
    if op == 0 {
        flag(&mut owner, 2, 1, force as u32);
    }
    let mut output = vec![MaybeUninit::new(seed as u8); capacity];
    let mut available = capacity;
    let mut total = 777;
    let mut output_size = capacity;
    let mut result = 777;
    let mut next = 0;
    let mut reference = None;
    let mut taken: Option<(usize, Vec<u8>)> = None;
    // SAFETY: All observed bytes began seeded by the fixture allocator; the
    // corpus restricts reached arithmetic/extent domains to actual defined
    // source calls. It does not infer initialization from unseeded capacity.
    unsafe {
        match op {
            0 => owner.calculate_ringbuffer_size(),
            1 => result = i32::from(owner.ensure_ringbuffer()),
            2 => total = owner.unwritten_bytes(wrap != 0),
            3 => {
                result = owner.write_ringbuffer(
                    &mut available,
                    match target {
                        0 => ringbuffer::OutputTarget::Discard,
                        1 => ringbuffer::OutputTarget::Reference(&mut reference),
                        _ => ringbuffer::OutputTarget::Copy {
                            destination: &mut output,
                            position: &mut next,
                        },
                    },
                    Some(&mut total),
                    force != 0,
                )
            }
            4 => owner.wrap_ringbuffer(),
            5 => {
                let base = owner.storage().ringbuffer.assume_init();
                if let Some(data) = owner.take_output(&mut output_size) {
                    taken = Some((data.as_ptr() as usize - base, data.to_vec()));
                }
            }
            6 => {
                result = i32::from(owner.has_more_output());
                total = usize::from(owner.is_finished());
            }
            7 => result = owner.save_error_code(error as i32, consumed as usize) as i32,
            _ => panic!("unknown helper"),
        }
    }
    let s = owner.storage();
    // SAFETY: These fixture-only observations were explicitly seeded or
    // initialized above; no decoder API promises unknown capacity is output.
    let (size, new_size, pos, rounds, partial, error, used, buffer_length, base, end, flags) = unsafe {
        (
            s.ringbuffer_size.assume_init(),
            s.new_ringbuffer_size.assume_init(),
            s.pos.assume_init(),
            s.rb_roundtrips.assume_init(),
            s.partial_pos_out.assume_init(),
            s.error_code.assume_init(),
            s.used_input.assume_init(),
            s.buffer_length.assume_init(),
            s.ringbuffer.assume_init(),
            s.ringbuffer_end.assume_init(),
            s.flags.assume_init(),
        )
    };
    write!(context.trace, " R{result}:{available}:{total}:{output_size}:{next}:{size}:{new_size}:{pos}:{rounds}:{partial}:{}:{error}:{used}:{buffer_length} P{}:", (flags >> 3) & 1, context.token(base)).unwrap();
    if base != 0 {
        write!(context.trace, "{}", end - base).unwrap();
    } else {
        context.trace.push('-');
    }
    context.trace.push_str(" N");
    if let Some(range) = reference {
        write!(context.trace, "{}", range.start).unwrap();
    } else {
        context.trace.push('-');
    }
    context.trace.push_str(" T");
    if let Some((offset, data)) = taken {
        write!(context.trace, "{offset}:").unwrap();
        bytes(&mut context.trace, &data);
    } else {
        context.trace.push('-');
    }
    context.trace.push_str(" O");
    // SAFETY: Fixture output was seeded; copying never uninitializes it.
    bytes(&mut context.trace, unsafe {
        std::slice::from_raw_parts(output.as_ptr().cast::<u8>(), output.len())
    });
    context.trace.push_str(" B");
    if let Some(ring) = owner.ringbuffer.as_ref() {
        // SAFETY: All ring bytes were seeded by this fixture's actual custom
        // allocator and representation copies preserve that initialization.
        bytes(&mut context.trace, unsafe {
            ring.initialized_bytes(0..size as usize + 542)
        });
    }
    context.state = std::ptr::null();
    drop(owner);
    context.trace.push('\n');
    std::mem::take(&mut context.trace)
}
fn main() {
    for line in io::stdin().lock().lines() {
        print!("{}", observe(&line.unwrap()));
    }
}

#[cfg(test)]
mod native {
    use super::*;
    fn original_state() -> state::StateOwner<'static> {
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
    #[test]
    fn metadata_sizing_does_not_read_unknown_metablock_length() {
        let mut s = original_state();
        s.write_flag(6, 6, 16);
        s.write_flag(2, 1, 1);
        // SAFETY: Metadata returns before remaining_len, which has never been
        // initialized by StateInit. Only the decoded window flag is required.
        unsafe { s.calculate_ringbuffer_size() };
        // SAFETY: StateInit wrote this field, and the metadata path retains it.
        assert_eq!(unsafe { s.storage().new_ringbuffer_size.assume_init() }, 0);
    }
    #[test]
    fn ring_growth_copies_only_reached_representation_and_initializes_two_tail_bytes() {
        let mut s = original_state();
        s.storage_mut().new_ringbuffer_size.write(1024);
        // SAFETY: Source first ring allocation needs no old payload reads.
        assert!(unsafe { s.ensure_ringbuffer() });
        let old = s.ringbuffer.as_mut().unwrap();
        for (byte, value) in old.storage_mut().iter_mut().zip([11, 22, 33]) {
            byte.write(value);
        }
        // SAFETY: These are the only last two bytes the source initialized.
        assert_eq!(unsafe { old.initialized_bytes(1022..1024) }, [0, 0]);
        s.storage_mut().pos.write(3);
        s.storage_mut().new_ringbuffer_size.write(2048);
        // SAFETY: The exact source copy extent is the initialized prefix3.
        assert!(unsafe { s.ensure_ringbuffer() });
        let ring = s.ringbuffer.as_ref().unwrap();
        // SAFETY: Growth copied exactly3 known bytes and initialized its last2.
        assert_eq!(unsafe { ring.initialized_bytes(0..3) }, [11, 22, 33]);
        assert_eq!(unsafe { ring.initialized_bytes(2046..2048) }, [0, 0]);
    }
    #[test]
    fn wrapping_unknown_representations_does_not_read_them_as_scalars() {
        let mut s = original_state();
        s.storage_mut().new_ringbuffer_size.write(1024);
        // SAFETY: First allocation on source-initialized state.
        assert!(unsafe { s.ensure_ringbuffer() });
        s.ringbuffer.as_mut().unwrap().storage_mut()[1024].write(9);
        s.ringbuffer.as_mut().unwrap().storage_mut()[1025].write(7);
        s.storage_mut().pos.write(4);
        s.write_flag(3, 1, 1);
        // SAFETY: Two disjoint4-byte extents exist in the actual allocation.
        // Two source bytes remain unknown representation, copied as memcpy.
        unsafe { s.wrap_ringbuffer() };
        // SAFETY: Only these first2 copied bytes had source initialization.
        assert_eq!(
            unsafe { s.ringbuffer.as_ref().unwrap().initialized_bytes(0..2) },
            [9, 7]
        );
    }
    #[test]
    fn output_writes_an_uninitialized_destination_and_borrows_only_known_ring_bytes() {
        let mut s = original_state();
        s.storage_mut().new_ringbuffer_size.write(1024);
        // SAFETY: First allocation on source-initialized state.
        assert!(unsafe { s.ensure_ringbuffer() });
        for (byte, value) in s
            .ringbuffer
            .as_mut()
            .unwrap()
            .storage_mut()
            .iter_mut()
            .zip([9, 7])
        {
            byte.write(value);
        }
        s.storage_mut().pos.write(2);
        s.storage_mut().meta_block_remaining_len.write(0);
        s.write_flag(6, 6, 10);
        let mut output = [MaybeUninit::uninit(); 8];
        let mut position = 0;
        let mut available = 1;
        // SAFETY: The source output range is the known first byte, and the
        // distinct output may be uninitialized before memcpy writes its prefix.
        assert_eq!(
            unsafe {
                s.write_ringbuffer(
                    &mut available,
                    ringbuffer::OutputTarget::Copy {
                        destination: &mut output,
                        position: &mut position,
                    },
                    None,
                    true,
                )
            },
            3
        );
        assert_eq!(position, 1);
        // SAFETY: Only output[0] was initialized by the exact reached memcpy.
        assert_eq!(unsafe { output[0].assume_init() }, 9);
        let mut size = 0;
        // SAFETY: Exactly one initialized ring byte remains; other capacity
        // is unknown and must not become part of the returned source slice.
        assert_eq!(unsafe { s.take_output(&mut size) }, Some([7].as_slice()));
        assert_eq!(size, 1);
        // SAFETY: All reached scalar output fields were initialized above.
        assert!(!unsafe { s.has_more_output() });
        s.storage_mut().state.write(running::BROTLI_STATE_DONE);
        assert!(unsafe { s.is_finished() });
    }
    #[test]
    fn saving_status_with_empty_buffer_does_not_read_unknown_input_pointers() {
        let mut s = original_state();
        // SAFETY: StateInit's zero buffer_length short circuit avoids both
        // bit-reader pointer slots, which remain genuinely uninitialized.
        assert_eq!(unsafe { s.save_error_code(2, 3) }, 2);
        // SAFETY: StateInit and SaveErrorCode initialize these exact fields.
        assert_eq!(unsafe { s.storage().used_input.assume_init() }, 3);
        assert_eq!(unsafe { s.storage().error_code.assume_init() }, 2);
    }
}
