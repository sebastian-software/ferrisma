// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![allow(dead_code)]
#[path = "../src/brotli/bit_reader.rs"]
mod bit_reader;
#[path = "../src/brotli/bitfield.rs"]
mod bitfield;
#[path = "../src/brotli/default_memory.rs"]
mod default_memory;
#[path = "../src/brotli/instance.rs"]
mod instance;
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

use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::fmt::Write as _;
use std::io::{self, BufRead};
use std::mem::MaybeUninit;

struct Block {
    pointer: *mut u8,
    size: usize,
    live: bool,
}
struct Context {
    calls: usize,
    fail: usize,
    seed: u8,
    blocks: Vec<Block>,
    trace: String,
}
impl Drop for Context {
    fn drop(&mut self) {
        for b in &self.blocks {
            let layout = Layout::from_size_align(b.size.max(1), 16).unwrap();
            // SAFETY: This fixture quarantines the exact producing allocation
            // after each logical source free, then uniquely releases it here.
            unsafe { dealloc(b.pointer, layout) };
        }
    }
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The live fixture context is accessed only during this callback;
    // decoder field borrows do not borrow it and callbacks never reenter.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    c.calls += 1;
    write!(c.trace, " A{}:{size}", c.calls).unwrap();
    if c.calls == c.fail {
        c.trace.push_str(":0");
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: The fixture allocates a valid nonzero native-aligned extent.
    let pointer = unsafe { alloc(layout) };
    assert!(!pointer.is_null());
    // SAFETY: This newly owned extent contains at least size writable bytes.
    unsafe { pointer.write_bytes(c.seed, size) };
    c.blocks.push(Block {
        pointer,
        size,
        live: true,
    });
    c.trace.push_str(":1");
    pointer.cast()
}
unsafe extern "C" fn free(opaque: *mut c_void, pointer: *mut c_void) {
    // SAFETY: Same live, non-reentrant context contract as allocate.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    if pointer.is_null() {
        c.trace.push_str(" F0");
        return;
    }
    let index = c
        .blocks
        .iter()
        .position(|b| b.pointer.cast::<c_void>() == pointer)
        .unwrap();
    assert!(c.blocks[index].live);
    c.blocks[index].live = false;
    write!(c.trace, " F{}", index + 1).unwrap();
}
unsafe extern "C" fn start(_: *mut c_void, _: usize) {}
unsafe extern "C" fn chunk(_: *mut c_void, _: *const u8, _: usize) {}

struct Replacement {
    handle: instance::MetadataCallbacksHandle,
    target: *mut c_void,
}
unsafe extern "C" fn replace_from_callback(opaque: *mut c_void, _: usize) {
    // SAFETY: The fixture keeps this context and the producing allocation
    // alive. The caller ended every actual source-storage reference before
    // entering this callback; this context points to no Rust owner sidecar.
    let r = unsafe { &*opaque.cast::<Replacement>() };
    // SAFETY: Only the three unborrowed callback fields are replaced. The new
    // callable function and its initialized counter context remain live.
    unsafe { r.handle.set(None, Some(replaced_chunk), r.target) };
}
unsafe extern "C" fn replaced_chunk(opaque: *mut c_void, _: *const u8, size: usize) {
    // SAFETY: The fixture supplies its live uniquely accessed usize counter.
    let count = unsafe { &mut *opaque.cast::<usize>() };
    *count += size;
}

/// Field-API regression only; this does not simulate a streaming decoder.
/// The actual full-source callback replacement path is separately captured
/// by tools/oracle/brotli_metadata.cc under both complete original graphs.
pub fn callback_handle_keeps_the_actual_allocation_after_owner_move() {
    let mut decoder = instance::DecoderInstance::new().unwrap();
    let actual = decoder.as_ptr();
    let handle = decoder.metadata_callbacks_handle();
    let mut counter = 0usize;
    let target = (&mut counter as *mut usize).cast::<c_void>();
    let mut replacement = Replacement { handle, target };
    let opaque = (&mut replacement as *mut Replacement).cast::<c_void>();
    // SAFETY: These fixture callbacks/contexts remain live until destruction.
    unsafe { decoder.set_metadata_callbacks(Some(replace_from_callback), None, opaque) };
    // Move only the ownership sidecar. The callback handle addresses the real
    // stable source block, not any prior Rust stack address or copied state.
    let mut owners = vec![decoder];
    let decoder = owners.pop().unwrap();
    assert_eq!(decoder.as_ptr(), actual);
    // SAFETY: The source setter initialized these fields. Copy only the
    // callable address/opaque, then end the storage borrow before the call.
    let (callback, opaque) = unsafe {
        let storage = decoder.state.storage();
        (
            storage.metadata_start_func.assume_init(),
            storage.metadata_callback_opaque.assume_init(),
        )
    };
    assert_eq!(callback, replace_from_callback as *const () as usize);
    // SAFETY: The address came from this exact typed function stored above;
    // function ABI, live opaque, owner lifetime and no field aliases hold.
    let callback = unsafe { std::mem::transmute::<usize, instance::MetadataStartFunc>(callback) };
    unsafe { callback(opaque as *mut c_void, 3) };
    // SAFETY: Replacement initialized exactly these native source fields.
    let (start, callback, opaque) = unsafe {
        let storage = decoder.state.storage();
        (
            storage.metadata_start_func.assume_init(),
            storage.metadata_chunk_func.assume_init(),
            storage.metadata_callback_opaque.assume_init(),
        )
    };
    assert_eq!(start, 0);
    assert_eq!(callback, replaced_chunk as *const () as usize);
    assert_eq!(opaque, target as usize);
    // SAFETY: Same actual typed-function origin and live unique counter; the
    // initialized advertised two-byte chunk lives throughout the callback.
    let callback = unsafe { std::mem::transmute::<usize, instance::MetadataChunkFunc>(callback) };
    unsafe { callback(opaque as *mut c_void, b"xy".as_ptr(), 2) };
    assert_eq!(counter, 2);
}

pub fn observe(line: &str) -> String {
    let mut input = line.split_whitespace();
    assert_eq!(input.next(), Some("bi-create"));
    let values: Vec<usize> = input.map(|s| s.parse().unwrap()).collect();
    let [pair, fail, seed, metadata, count] = values[..5] else {
        unreachable!()
    };
    assert_eq!(values.len(), 5 + count * 2);
    let mut c = Context {
        calls: 0,
        fail,
        seed: seed as u8,
        blocks: Vec::new(),
        trace: String::new(),
    };
    let opaque = (&mut c as *mut Context).cast::<c_void>();
    // SAFETY: Both fixture callbacks retain stable exclusive payloads and the
    // live context through destruction; mixed pairs cannot be called.
    let mut decoder = unsafe {
        instance::DecoderInstance::create(
            if pair & 1 != 0 { Some(allocate) } else { None },
            if pair & 2 != 0 { Some(free) } else { None },
            opaque,
        )
    };
    write!(c.trace, " R{}", u8::from(decoder.is_some())).unwrap();
    if let Some(d) = &mut decoder {
        for i in 0..count {
            let accepted = d.set_parameter(values[5 + 2 * i] as u32, values[6 + 2 * i] as u32);
            let flags = &mut d.state.storage_mut().flags;
            // SAFETY: This is a four-byte aligned MaybeUninit field in the
            // actual exclusively borrowed block; no unknown u32 is read.
            let bytes = unsafe {
                std::slice::from_raw_parts_mut(flags.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4)
            };
            let word = bitfield::BitfieldWord::new(bytes.try_into().unwrap());
            // SAFETY: StateInit and settings initialize these selected bits;
            // other allocator-seeded/unknown bits remain inside the helper.
            let (canny, large) = unsafe { (word.read(4, 1), word.read(5, 1)) };
            write!(c.trace, " P{}:{canny}:{large}", u8::from(accepted)).unwrap();
        }
        // SAFETY: StateInit initializes opaque and all metadata fields.
        let right_opaque = unsafe { d.state.storage().memory_manager_opaque.assume_init() }
            == if pair == 3 { opaque as usize } else { 0 };
        write!(
            c.trace,
            " O{}:{}:{}",
            u8::from(d.is_used()),
            d.get_error_code(),
            u8::from(right_opaque)
        )
        .unwrap();
        let start_fn: Option<instance::MetadataStartFunc> =
            if metadata & 1 != 0 { Some(start) } else { None };
        let chunk_fn: Option<instance::MetadataChunkFunc> =
            if metadata & 2 != 0 { Some(chunk) } else { None };
        // SAFETY: These immutable fixture functions and opaque remain valid
        // through destruction and never unwind or reenter the decoder.
        unsafe { d.set_metadata_callbacks(start_fn, chunk_fn, opaque) };
        // SAFETY: The setter just initialized these exact fields in order.
        let (a, b, o) = unsafe {
            (
                d.state.storage().metadata_start_func.assume_init()
                    == start_fn.map_or(0, |f| f as usize),
                d.state.storage().metadata_chunk_func.assume_init()
                    == chunk_fn.map_or(0, |f| f as usize),
                d.state.storage().metadata_callback_opaque.assume_init() == opaque as usize,
            )
        };
        write!(c.trace, " M{}:{}:{}", u8::from(a), u8::from(b), u8::from(o)).unwrap();
    }
    instance::destroy_instance(decoder);
    c.trace.push('\n');
    c.trace.clone()
}

fn main() {
    for line in io::stdin().lock().lines() {
        print!("{}", observe(&line.unwrap()));
    }
}
