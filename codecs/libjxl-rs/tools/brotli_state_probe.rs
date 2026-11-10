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

use layout::{
    BitReaderStorage, DecoderStateStorage as S, HeaderArenaStorage, HuffmanTreeGroupStorage as G,
    SharedDictionaryStorage as D,
};
use std::alloc::{Layout, alloc, dealloc};
use std::ffi::c_void;
use std::fmt::Write as _;
use std::io::{self, BufRead, Write};
use std::mem::{offset_of, size_of};

struct Block {
    ptr: *mut u8,
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
        for block in &self.blocks {
            let layout = Layout::from_size_align(block.size.max(1), 16).unwrap();
            // SAFETY: Fixture quarantine retained each actual global block,
            // its exact layout and unique physical deallocation until now.
            unsafe { dealloc(block.ptr, layout) };
        }
    }
}
fn pointer(c: &Context, value: usize) -> usize {
    if value == 0 {
        return 0;
    }
    let (default_alloc, default_free) = default_memory::callbacks();
    let labels = [
        (allocate as memory::AllocFunc as usize, 0x1001),
        (free as memory::FreeFunc as usize, 0x1002),
        (c as *const Context as usize, 0x1003),
        (default_alloc as usize, 0x1004),
        (default_free as usize, 0x1005),
        (&static_descriptors::DICTIONARY as *const _ as usize, 0x1010),
        (&static_descriptors::TRANSFORMS as *const _ as usize, 0x1011),
        (tables::DICTIONARY_DATA.as_ptr() as usize, 0x1012),
        (tables::PREFIX_SUFFIX.as_ptr() as usize, 0x1013),
        (tables::PREFIX_SUFFIX_MAP.as_ptr() as usize, 0x1014),
        (tables::TRANSFORMS_DATA.as_ptr() as usize, 0x1015),
    ];
    for (address, label) in labels {
        if value == address {
            return label;
        }
    }
    for (i, block) in c.blocks.iter().enumerate() {
        let start = block.ptr as usize;
        if start <= value && value <= start + block.size {
            return 0x10000000 + (i + 1) * 0x100000 + value - start;
        }
    }
    value
}
fn normalize(c: &Context, bytes: &mut [u8], offset: usize) {
    let value = usize::from_ne_bytes(
        bytes[offset..offset + size_of::<usize>()]
            .try_into()
            .unwrap(),
    );
    bytes[offset..offset + size_of::<usize>()].copy_from_slice(&pointer(c, value).to_ne_bytes());
}
fn hex(out: &mut String, bytes: &[u8]) {
    if bytes.is_empty() {
        out.push('-');
    }
    for byte in bytes {
        write!(out, "{byte:02x}").unwrap();
    }
}
fn snapshot(c: &mut Context) {
    for (id, block) in c.blocks.iter().enumerate() {
        if !block.live || (block.size != size_of::<S>() && block.size != size_of::<D>()) {
            continue;
        }
        // SAFETY: This fixture initialized all bytes in allocate. At each
        // actual callback no decoder storage borrow is live, and no mutation
        // occurs while copying this still-live source block's observations.
        let mut bytes = unsafe { std::slice::from_raw_parts(block.ptr, block.size) }.to_vec();
        let mut offsets = Vec::new();
        if id == 0 && block.size == size_of::<S>() {
            offsets.extend([
                offset_of!(S, alloc_func),
                offset_of!(S, free_func),
                offset_of!(S, memory_manager_opaque),
                offset_of!(S, br) + offset_of!(BitReaderStorage, next_in),
                offset_of!(S, br) + offset_of!(BitReaderStorage, guard_in),
                offset_of!(S, br) + offset_of!(BitReaderStorage, last_in),
                offset_of!(S, ringbuffer),
                offset_of!(S, ringbuffer_end),
                offset_of!(S, htree_command),
                offset_of!(S, context_lookup),
                offset_of!(S, context_map_slice),
                offset_of!(S, dist_context_map_slice),
                offset_of!(S, block_type_trees),
                offset_of!(S, block_len_trees),
                offset_of!(S, dist_context_map),
                offset_of!(S, literal_htree),
                offset_of!(S, metadata_start_func),
                offset_of!(S, metadata_chunk_func),
                offset_of!(S, metadata_callback_opaque),
                offset_of!(S, context_map),
                offset_of!(S, context_modes),
                offset_of!(S, dictionary),
                offset_of!(S, compound_dictionary),
                offset_of!(S, arena) + offset_of!(HeaderArenaStorage, symbol_lists),
                offset_of!(S, arena) + offset_of!(HeaderArenaStorage, next),
            ]);
            for group in [
                offset_of!(S, literal_hgroup),
                offset_of!(S, insert_copy_hgroup),
                offset_of!(S, distance_hgroup),
            ] {
                offsets.push(group + offset_of!(G, htrees));
                offsets.push(group + offset_of!(G, codes));
            }
        } else if block.size == size_of::<D>() {
            for i in 0..15 {
                offsets.push(offset_of!(D, prefix) + i * size_of::<usize>());
            }
            for i in 0..64 {
                offsets.push(offset_of!(D, words) + i * size_of::<usize>());
                offsets.push(offset_of!(D, transforms) + i * size_of::<usize>());
            }
            offsets.extend([
                offset_of!(D, words_instances),
                offset_of!(D, transforms_instances),
                offset_of!(D, prefix_suffix_maps),
                offset_of!(D, alloc_func),
                offset_of!(D, free_func),
                offset_of!(D, memory_manager_opaque),
            ]);
        }
        for offset in offsets {
            normalize(c, &mut bytes, offset);
        }
        write!(c.trace, " B{}:", id + 1).unwrap();
        hex(&mut c.trace, &bytes);
    }
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The caller passes this live exclusive fixture context, which
    // outlives every returned owner and is not aliased through decoder blocks.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    c.calls += 1;
    write!(c.trace, " A{}:{size}", c.calls).unwrap();
    snapshot(c);
    if c.calls == c.fail {
        c.trace.push_str(":0");
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: Valid nonzero layout, matching quarantine deallocation retained.
    let ptr = unsafe { alloc(layout) };
    assert!(!ptr.is_null());
    // SAFETY: The actual returned size-byte payload is writable. Fixture
    // initialization deliberately supplies all sentinel/unknown-neighbor data.
    unsafe { ptr.write_bytes(c.seed, size) };
    c.blocks.push(Block {
        ptr,
        size,
        live: true,
    });
    c.trace.push_str(":1");
    ptr.cast()
}
unsafe extern "C" fn free(opaque: *mut c_void, address: *mut c_void) {
    // SAFETY: The same callback pair/context lifetime as allocate applies;
    // no decoder raw-storage borrow survives the callback boundary.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    if address.is_null() {
        c.trace.push_str(" F0");
        snapshot(c);
        return;
    }
    let id = c
        .blocks
        .iter()
        .position(|block| block.ptr.cast::<c_void>() == address)
        .expect("actual producing block");
    assert!(c.blocks[id].live);
    write!(c.trace, " F{}", id + 1).unwrap();
    snapshot(c);
    c.blocks[id].live = false;
}
pub fn observe(line: &str) -> String {
    let mut output = Vec::new();
    let mut words = line.split_whitespace();
    match words.next().unwrap() {
        "bs-state" => {
            let fail = words.next().unwrap().parse().unwrap();
            let seed = words.next().unwrap().parse().unwrap();
            let action: u32 = words.next().unwrap().parse().unwrap();
            let group = words.next().unwrap().parse().unwrap();
            let maximum = words.next().unwrap().parse().unwrap();
            let limit = words.next().unwrap().parse().unwrap();
            let ntrees = words.next().unwrap().parse().unwrap();
            let mut c = Context {
                calls: 0,
                fail,
                seed,
                blocks: Vec::new(),
                trace: String::new(),
            };
            let opaque = (&mut c as *mut Context).cast();
            // SAFETY: Pair returns actual exclusive source-aligned blocks,
            // retains opaque until all owners drop, accepts null, does not
            // unwind/reenter, and inspects only at borrow-ended checkpoints.
            let allocator =
                unsafe { memory::Allocator::from_callbacks(Some(allocate), Some(free), opaque) }
                    .unwrap();
            let outer = allocator.allocate(size_of::<S>());
            let mut owner = outer.and_then(|allocation| {
                state::StateOwner::initialize(
                    allocation,
                    allocator,
                    allocator,
                    allocate as memory::AllocFunc as usize,
                    free as memory::FreeFunc as usize,
                    opaque as usize,
                    opaque as usize,
                )
            });
            write!(c.trace, " C{}", u8::from(owner.is_some())).unwrap();
            if let Some(s) = &mut owner {
                snapshot(&mut c);
                match action {
                    0 => (),
                    1 => s.metablock_begin(),
                    2 => {
                        let success = s.huffman_group_init(group, maximum, limit, ntrees);
                        write!(c.trace, " H{}", u8::from(success)).unwrap();
                    }
                    3 => s.cleanup_after_metablock(),
                    _ => panic!("source fixture action"),
                }
                snapshot(&mut c);
            }
            drop(owner);
            assert!(c.blocks.iter().all(|block| !block.live));
            writeln!(output, "{}", c.trace).unwrap();
        }
        "bs-private-layout" => {
            let (allocate, free) = default_memory::callbacks();
            // SAFETY: Real matched defaults, exact live64-byte allocation,
            // no unknown payload read; all borrows end before the free.
            let address = unsafe { allocate(std::ptr::null_mut(), 64) };
            assert!(!address.is_null());
            writeln!(
                output,
                "{} {} {} {}",
                default_memory::SOURCE_MAX_ALIGNMENT,
                default_memory::DEFAULT_ALIGNMENT,
                default_memory::HEADER_SIZE,
                address as usize % default_memory::SOURCE_MAX_ALIGNMENT
            )
            .unwrap();
            // SAFETY: Unique release by this actual producing default pair.
            unsafe { free(std::ptr::null_mut(), address) };
        }
        "bs-layout" => {
            let c = Context {
                calls: 0,
                fail: 0,
                seed: 0,
                blocks: Vec::new(),
                trace: String::new(),
            };
            let mut text = format!(
                "{} {}:{} {}:{} ",
                default_memory::SOURCE_MAX_ALIGNMENT,
                size_of::<static_descriptors::DictionaryDescriptor>(),
                std::mem::align_of::<static_descriptors::DictionaryDescriptor>(),
                size_of::<static_descriptors::TransformDescriptor>(),
                std::mem::align_of::<static_descriptors::TransformDescriptor>()
            );
            // SAFETY: The source-shaped immutable dictionary has no implicit
            // padding on this admitted native layout. The transform view
            // explicitly initializes each source static-padding byte, and
            // all scalar/pointer fields are initialized immutable statics.
            let mut dictionary = unsafe {
                std::slice::from_raw_parts(
                    (&static_descriptors::DICTIONARY
                        as *const static_descriptors::DictionaryDescriptor)
                        .cast::<u8>(),
                    size_of::<static_descriptors::DictionaryDescriptor>(),
                )
            }
            .to_vec();
            normalize(
                &c,
                &mut dictionary,
                offset_of!(static_descriptors::DictionaryDescriptor, data),
            );
            hex(&mut text, &dictionary);
            text.push(' ');
            // SAFETY: Same exact immutable, all-bytes-initialized descriptor
            // and explicit source-padding proof as the preceding copy.
            let mut transforms = unsafe {
                std::slice::from_raw_parts(
                    (&static_descriptors::TRANSFORMS
                        as *const static_descriptors::TransformDescriptor)
                        .cast::<u8>(),
                    size_of::<static_descriptors::TransformDescriptor>(),
                )
            }
            .to_vec();
            for offset in [
                offset_of!(static_descriptors::TransformDescriptor, prefix_suffix),
                offset_of!(static_descriptors::TransformDescriptor, prefix_suffix_map),
                offset_of!(static_descriptors::TransformDescriptor, transforms),
                offset_of!(static_descriptors::TransformDescriptor, params),
            ] {
                normalize(&c, &mut transforms, offset);
            }
            hex(&mut text, &transforms);
            writeln!(output, "{text}").unwrap();
        }
        "bs-default" => {
            let size: usize = words.next().unwrap().parse().unwrap();
            let fill: u8 = words.next().unwrap().parse().unwrap();
            let stored: u32 = words.next().unwrap().parse().unwrap();
            let opaque = (&size as *const usize).cast_mut().cast::<c_void>();
            let (mut allocate, mut free) = default_memory::callbacks();
            let mut owner = None;
            let mut text = String::new();
            if stored != 0 {
                let allocator = default_memory::allocator();
                let allocation = allocator.allocate(size_of::<S>()).unwrap();
                owner = state::StateOwner::initialize(
                    allocation,
                    allocator,
                    default_memory::allocator_with_opaque(opaque),
                    allocate as usize,
                    free as usize,
                    0,
                    opaque as usize,
                );
                let s = owner.as_ref().unwrap();
                // SAFETY: These fields were initialized by StateInit and
                // shared memset/source writes; none is an unknown capacity.
                let (a, f, o, da, df, d_o) = unsafe {
                    (
                        s.storage().alloc_func.assume_init(),
                        s.storage().free_func.assume_init(),
                        s.storage().memory_manager_opaque.assume_init(),
                        s.dictionary
                            .as_ref()
                            .unwrap()
                            .storage()
                            .alloc_func
                            .assume_init(),
                        s.dictionary
                            .as_ref()
                            .unwrap()
                            .storage()
                            .free_func
                            .assume_init(),
                        s.dictionary
                            .as_ref()
                            .unwrap()
                            .storage()
                            .memory_manager_opaque
                            .assume_init(),
                    )
                };
                write!(
                    text,
                    "{}:{}:{}:{}:{}:{} ",
                    u8::from(a == allocate as usize),
                    u8::from(f == free as usize),
                    u8::from(o == 0),
                    u8::from(da == allocate as usize),
                    u8::from(df == free as usize),
                    u8::from(d_o == opaque as usize)
                )
                .unwrap();
                // SAFETY: The stored addresses are the exact supplied,
                // callable source-signature default pair, not tokens.
                allocate = unsafe { std::mem::transmute::<usize, memory::AllocFunc>(a) };
                // SAFETY: Same exact field initialization/function type.
                free = unsafe { std::mem::transmute::<usize, memory::FreeFunc>(f) };
            }
            // SAFETY: This exact callable default pair returns the actual
            // allocation, accepts the same opaque (ignored), and is freed
            // once below after each payload borrow ends.
            let ptr = unsafe { allocate(opaque, size) };
            write!(text, "{}", u8::from(!ptr.is_null())).unwrap();
            if !ptr.is_null() {
                assert!(size <= 4096);
                write!(
                    text,
                    ":{}:",
                    ptr as usize % default_memory::SOURCE_MAX_ALIGNMENT
                )
                .unwrap();
                // SAFETY: Actual size-byte payload is live and exclusively
                // owned; initialize before inspecting any returned byte.
                unsafe {
                    ptr.cast::<u8>().write_bytes(fill, size);
                    hex(
                        &mut text,
                        std::slice::from_raw_parts(ptr.cast::<u8>(), size),
                    );
                    free(opaque, ptr);
                }
            }
            // SAFETY: The exact source default free accepts null.
            unsafe { free(opaque, std::ptr::null_mut()) };
            drop(owner);
            writeln!(output, "{text}").unwrap();
        }
        _ => panic!("source fixture command"),
    }
    assert!(words.next().is_none());
    String::from_utf8(output).unwrap()
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    for line in stdin.lock().lines() {
        output
            .write_all(observe(&line.unwrap()).as_bytes())
            .unwrap();
    }
}
