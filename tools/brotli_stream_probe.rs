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

#[path = "../src/brotli/compound_dictionary.rs"]
mod compound_dictionary;
#[path = "../src/brotli/ringbuffer.rs"]
mod ringbuffer;
#[path = "../src/brotli/status.rs"]
mod status;

#[path = "../src/brotli/instance.rs"]
mod instance;
#[path = "../src/brotli/reader_copy.rs"]
mod reader_copy;
#[path = "../src/brotli/stream_blocks.rs"]
mod stream_blocks;
#[path = "../src/brotli/stream_input.rs"]
mod stream_input;

#[path = "../src/brotli/stream_output.rs"]
mod stream_output;

#[path = "../src/brotli/constants.rs"]
mod constants;
#[path = "../src/brotli/context_map.rs"]
mod context_map;
#[path = "../src/brotli/huffman.rs"]
mod huffman;
#[path = "../src/brotli/huffman_reader.rs"]
mod huffman_reader;
#[path = "../src/brotli/metablock.rs"]
mod metablock;
#[path = "../src/brotli/mtf.rs"]
mod mtf;
#[path = "../src/brotli/stream_header.rs"]
mod stream_header;
#[path = "../src/brotli/symbols.rs"]
mod symbols;
#[path = "../src/brotli/tree_group.rs"]
mod tree_group;

#[path = "../src/brotli/block_length.rs"]
mod block_length;
#[path = "../src/brotli/stream_switch.rs"]
mod stream_switch;
#[path = "../src/brotli/stream_tables.rs"]
mod stream_tables;

#[path = "../src/brotli/command.rs"]
mod command;
#[path = "../src/brotli/distance.rs"]
mod distance;
#[path = "../src/brotli/raw_bytes.rs"]
mod raw_bytes;
#[path = "../src/brotli/raw_transform.rs"]
mod raw_transform;
#[path = "../src/brotli/stream.rs"]
mod stream;
#[path = "../src/brotli/stream_commands.rs"]
mod stream_commands;
#[path = "../src/brotli/transform.rs"]
mod transform;

use std::ffi::c_void;
use std::fmt::Write;
fn unhex(s: &str) -> Vec<u8> {
    if s == "-" {
        return Vec::new();
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
fn hex(v: &[u8]) -> String {
    if v.is_empty() {
        return "-".into();
    }
    v.iter().map(|x| format!("{x:02x}")).collect()
}
struct Allocations {
    calls: usize,
    fail: usize,
    seed: u8,
    blocks: Vec<(*mut c_void, usize)>,
    events: String,
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    let a = unsafe { &mut *opaque.cast::<Allocations>() };
    a.calls += 1;
    let id = a.calls;
    write!(a.events, " A{id}:{size}").unwrap();
    if id == a.fail {
        a.events.push_str(":0");
        return std::ptr::null_mut();
    }
    let p = unsafe { default_memory::brotli_default_alloc_func(std::ptr::null_mut(), size) };
    assert!(!p.is_null());
    unsafe {
        p.cast::<u8>().write_bytes(a.seed, size);
    }
    a.blocks.push((p, id));
    a.events.push_str(":1");
    p
}
unsafe extern "C" fn free(opaque: *mut c_void, p: *mut c_void) {
    let a = unsafe { &mut *opaque.cast::<Allocations>() };
    if p.is_null() {
        a.events.push_str(" F0");
        return;
    }
    let b = a
        .blocks
        .iter_mut()
        .find(|b| b.0 == p)
        .expect("producing observer block");
    write!(a.events, " F{}", b.1).unwrap();
    b.0 = std::ptr::null_mut();
    unsafe {
        default_memory::brotli_default_free_func(std::ptr::null_mut(), p);
    }
}
fn state(s: &mut instance::DecoderInstance<'_>, out: &mut String) {
    let p = s.as_ptr().cast::<layout::DecoderStateStorage>();
    macro_rules! r {
        ($f:ident) => {
            unsafe { (*p).$f.assume_init() }
        };
    }
    let br = unsafe { &*(*p).br.as_ptr() };
    // Only source StateInit's initialized selected fields; no arena/capacity view.
    write!(
        out,
        " S{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        r!(state),
        r!(loop_counter),
        r!(pos),
        r!(buffer_length),
        unsafe { br.val.assume_init() },
        unsafe { br.bit_pos.assume_init() },
        r!(ringbuffer_size),
        r!(new_ringbuffer_size),
        r!(ringbuffer_mask),
        r!(partial_pos_out),
        r!(rb_roundtrips),
        r!(used_input),
        r!(max_distance),
        r!(dist_rb_idx),
        unsafe { (*p).dist_rb.as_ptr().cast::<i32>().read() },
        unsafe { (*p).dist_rb.as_ptr().cast::<i32>().add(1).read() },
        unsafe { (*p).dist_rb.as_ptr().cast::<i32>().add(2).read() },
        unsafe { (*p).dist_rb.as_ptr().cast::<i32>().add(3).read() }
    )
    .unwrap();
    // This shared observer uses initialized selected flag bits only.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(
            std::ptr::addr_of!((*p).flags)
                .cast_mut()
                .cast::<std::mem::MaybeUninit<u8>>(),
            4,
        )
    };
    let w = bitfield::BitfieldWord::new(bytes.try_into().unwrap());
    for (shift, width) in [(6, 6), (5, 1), (0, 1), (1, 1), (2, 1), (3, 1), (4, 1)] {
        write!(out, ":{}", unsafe { w.read(shift, width) }).unwrap();
    }
}
struct Control {
    native: *const layout::DecoderStateStorage,
    handle: instance::MetadataCallbacksHandle,
    mode: u32,
    changed: bool,
    b: *mut Context,
    trace: String,
}
struct Context {
    control: *mut Control,
    id: u32,
}
unsafe fn change(c: &mut Control, start: bool, chunk: bool) {
    let sf = if start {
        Some(start_b as instance::MetadataStartFunc)
    } else {
        None
    };
    let cf = if chunk {
        Some(chunk_b as instance::MetadataChunkFunc)
    } else {
        None
    };
    unsafe {
        c.handle.set(sf, cf, c.b.cast());
    }
    c.changed = true;
    let p = c.native;
    write!(
        c.trace,
        " S{}{}{}",
        (unsafe { (*p).metadata_start_func.assume_init() } == sf.map_or(0, |f| f as usize)) as u8,
        (unsafe { (*p).metadata_chunk_func.assume_init() } == cf.map_or(0, |f| f as usize)) as u8,
        (unsafe { (*p).metadata_callback_opaque.assume_init() } == c.b as usize) as u8
    )
    .unwrap();
}
unsafe fn start(p: *mut c_void, n: usize) {
    let ctx = unsafe { &*p.cast::<Context>() };
    let c = unsafe { &mut *ctx.control };
    let native = c.native;
    write!(
        c.trace,
        " A{}:{}:{}:{}:{}",
        ctx.id,
        n,
        unsafe { (*native).meta_block_remaining_len.assume_init() },
        unsafe { (*(*native).br.as_ptr()).bit_pos.assume_init() },
        unsafe { (*native).error_code.assume_init() }
    )
    .unwrap();
    if ctx.id == 0 && !c.changed {
        if c.mode == 1 {
            unsafe {
                change(c, true, true);
            }
        }
        if c.mode == 3 {
            unsafe {
                change(c, false, false);
            }
        }
    }
}
unsafe fn chunk(p: *mut c_void, data: *const u8, n: usize) {
    let ctx = unsafe { &*p.cast::<Context>() };
    let c = unsafe { &mut *ctx.control };
    let native = c.native;
    write!(
        c.trace,
        " C{}:{}:{}:{}:{}:",
        ctx.id,
        n,
        unsafe { (*native).meta_block_remaining_len.assume_init() },
        unsafe { (*(*native).br.as_ptr()).bit_pos.assume_init() },
        unsafe { (*native).error_code.assume_init() }
    )
    .unwrap();
    for i in 0..n {
        write!(c.trace, "{:02x}", unsafe { data.add(i).read() }).unwrap();
    }
    if ctx.id == 0 && !c.changed {
        if c.mode == 2 {
            unsafe {
                change(c, true, true);
            }
        }
        if c.mode == 4 {
            unsafe {
                change(c, true, false);
            }
        }
        if c.mode == 5 {
            unsafe {
                change(c, false, true);
            }
        }
    }
}
unsafe extern "C" fn start_a(p: *mut c_void, n: usize) {
    unsafe {
        start(p, n);
    }
}
unsafe extern "C" fn start_b(p: *mut c_void, n: usize) {
    unsafe {
        start(p, n);
    }
}
unsafe extern "C" fn chunk_a(p: *mut c_void, d: *const u8, n: usize) {
    unsafe {
        chunk(p, d, n);
    }
}
unsafe extern "C" fn chunk_b(p: *mut c_void, d: *const u8, n: usize) {
    unsafe {
        chunk(p, d, n);
    }
}
fn callbacks(words: &mut std::str::SplitWhitespace<'_>) -> String {
    let mode = words.next().unwrap().parse().unwrap();
    let chunk: usize = words.next().unwrap().parse().unwrap();
    let data = unhex(words.next().unwrap());
    let mut s = instance::DecoderInstance::new().unwrap();
    let bindings = compound_dictionary::CompoundDictionaryBindings::default();
    let mut c = Control {
        native: s.as_ptr().cast(),
        handle: s.metadata_callbacks_handle(),
        mode,
        changed: false,
        b: std::ptr::null_mut(),
        trace: String::new(),
    };
    let mut a = Context {
        control: &mut c,
        id: 0,
    };
    let mut b = Context {
        control: &mut c,
        id: 1,
    };
    c.b = &mut b;
    unsafe {
        s.set_metadata_callbacks(
            Some(start_a),
            Some(chunk_a),
            std::ptr::addr_of_mut!(a).cast(),
        );
    }
    let mut offset = 0;
    let mut total = 123;
    let mut result;
    for call in 0..1000000 {
        let mut extent = data.len() - offset;
        if chunk != 0 && extent > chunk {
            extent = chunk;
        }
        let mut ai = extent;
        let mut ao = 0;
        let mut ni = unsafe { data.as_ptr().add(offset) };
        let mut no = std::ptr::null_mut();
        result = unsafe {
            s.decompress_stream_raw(&bindings, &mut ai, &mut ni, &mut ao, &mut no, &mut total)
        };
        let consumed = extent - ai;
        let bit_pos = unsafe { (*(*c.native).br.as_ptr()).bit_pos.assume_init() };
        write!(
            c.trace,
            " R{}:{}:{}:{}:{}:{}:{}",
            result,
            consumed,
            ai,
            total,
            bit_pos,
            s.get_error_code(),
            unsafe { s.state.is_finished() } as u8
        )
        .unwrap();
        offset += consumed;
        if result != 2 || offset == data.len() {
            write!(
                c.trace,
                " Z{}:{}:{}:{}",
                (result == 1) as u8,
                offset,
                data.len(),
                total
            )
            .unwrap();
            return c.trace;
        }
        assert_eq!(ai, 0);
        assert_ne!(call, 999999);
    }
    unreachable!()
}
fn entry(data: &[u8]) -> String {
    let bindings = compound_dictionary::CompoundDictionaryBindings::default();
    let mut out = String::new();
    let mut s = instance::DecoderInstance::new().unwrap();
    let mut ai = 0;
    let mut ao = 1;
    let mut total = 7;
    let r = unsafe {
        s.decompress_stream_raw(
            &bindings,
            &mut ai,
            std::ptr::null_mut(),
            &mut ao,
            std::ptr::null_mut(),
            &mut total,
        )
    };
    write!(
        out,
        "E0:{}:{}:{}:{}:{}",
        r,
        ai,
        ao,
        total,
        s.get_error_code()
    )
    .unwrap();
    state(&mut s, &mut out);
    total = 8;
    let r = unsafe {
        s.decompress_stream_raw(
            &bindings,
            &mut ai,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut total,
        )
    };
    write!(out, " E1:{}:{}:{}:{}", r, ai, total, s.get_error_code()).unwrap();
    state(&mut s, &mut out);
    drop(s);
    let mut s = instance::DecoderInstance::new().unwrap();
    ai = data.len();
    ao = 1;
    let mut ni = data.as_ptr();
    let mut byte = 0xccu8;
    let mut no = &mut byte as *mut u8;
    let a = std::ptr::addr_of_mut!(ai);
    let r = unsafe { s.decompress_stream_raw(&bindings, a, &mut ni, &mut ao, &mut no, a) };
    write!(
        out,
        " E2:{}:{}:{}:{}:{}:{}",
        r,
        ai,
        ao,
        (ni == data.as_ptr()) as u8,
        byte,
        s.get_error_code()
    )
    .unwrap();
    state(&mut s, &mut out);
    drop(s);
    let mut s = instance::DecoderInstance::new().unwrap();
    ai = data.len();
    ao = 7;
    ni = data.as_ptr();
    let mut output = [0xcc; 7];
    no = output.as_mut_ptr();
    let a = std::ptr::addr_of_mut!(ao);
    let r = unsafe { s.decompress_stream_raw(&bindings, &mut ai, &mut ni, a, &mut no, a) };
    write!(
        out,
        " E3:{}:{}:{}:{}:{}:{}",
        r,
        ai,
        ao,
        (no == output.as_mut_ptr()) as u8,
        s.get_error_code(),
        hex(&output)
    )
    .unwrap();
    state(&mut s, &mut out);
    drop(s);
    let mut s = instance::DecoderInstance::new().unwrap();
    ai = data.len();
    ni = data.as_ptr();
    let mut output = vec![0xcc; data.len()];
    no = output.as_mut_ptr();
    total = 99;
    let a = std::ptr::addr_of_mut!(ai);
    let r = unsafe { s.decompress_stream_raw(&bindings, a, &mut ni, a, &mut no, &mut total) };
    let count = unsafe { no.offset_from(output.as_ptr()) } as usize;
    write!(
        out,
        " E4:{}:{}:{}:{}:{}",
        r,
        ai,
        total,
        s.get_error_code(),
        hex(&output[..count])
    )
    .unwrap();
    state(&mut s, &mut out);
    out
}
pub fn observe(line: &str) -> String {
    let mut words = line.split_whitespace();
    let op = words.next().unwrap();
    if op == "entry" {
        return entry(&unhex(words.next().unwrap()));
    }
    if op == "callbacks" {
        return callbacks(&mut words);
    }
    assert!(
        [
            "stream",
            "state",
            "alloc",
            "dict",
            "dictalloc",
            "take",
            "swap",
            "replay"
        ]
        .contains(&op)
    );
    let ci: usize = words.next().unwrap().parse().unwrap();
    let co: usize = words.next().unwrap().parse().unwrap();
    let disable = words.next().unwrap().parse().unwrap();
    let large = words.next().unwrap().parse().unwrap();
    let fail = if op == "alloc" || op == "dictalloc" {
        words.next().unwrap().parse().unwrap()
    } else {
        0
    };
    let seed = if op == "alloc" || op == "dictalloc" {
        words.next().unwrap().parse().unwrap()
    } else {
        165
    };
    let dictionaries = if op == "dict" || op == "dictalloc" {
        let count: usize = words.next().unwrap().parse().unwrap();
        (0..count)
            .map(|_| unhex(words.next().unwrap()))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let data = unhex(words.next().unwrap());
    let mut allocator = Allocations {
        calls: 0,
        fail,
        seed,
        blocks: Vec::new(),
        events: String::new(),
    };
    let created = if op == "alloc" || op == "dictalloc" {
        unsafe {
            instance::DecoderInstance::create(
                Some(allocate),
                Some(free),
                std::ptr::addr_of_mut!(allocator).cast(),
            )
        }
    } else {
        instance::DecoderInstance::new()
    };
    let Some(mut s) = created else {
        return format!("NULL{}", allocator.events);
    };
    let mut bindings = compound_dictionary::CompoundDictionaryBindings::default();
    let mut out = String::new();
    write!(
        out,
        "P{}{}",
        s.set_parameter(0, disable) as u8,
        s.set_parameter(1, large) as u8
    )
    .unwrap();
    for dictionary in &dictionaries {
        write!(out, " D{}", unsafe {
            s.state
                .attach_dictionary(&mut bindings, 0, Some(dictionary))
        } as u8)
        .unwrap();
    }
    let mut offset = 0;
    let mut total = 999;
    for call in 0..1000000 {
        let mut ai = ci.min(data.len() - offset);
        let mut ao = if op == "take" { 0 } else { co };
        let swapped = if op == "swap" {
            data[offset..offset + ai].to_vec()
        } else {
            Vec::new()
        };
        let start = if op == "swap" {
            swapped.as_ptr()
        } else {
            unsafe { data.as_ptr().add(offset) }
        };
        let mut ni = start;
        let mut output = vec![0xcc; co];
        let mut no = output.as_mut_ptr();
        let result = unsafe {
            s.decompress_stream_raw(&bindings, &mut ai, &mut ni, &mut ao, &mut no, &mut total)
        };
        let consumed = unsafe { ni.offset_from(start) } as usize;
        offset += consumed;
        write!(
            out,
            " R{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            result,
            ai,
            ao,
            total,
            consumed,
            s.get_error_code(),
            s.is_used() as u8,
            unsafe { s.state.is_finished() } as u8,
            unsafe { s.state.has_more_output() } as u8,
            hex(&output[..if op == "take" { 0 } else { co - ao }])
        )
        .unwrap();
        if op != "stream" {
            state(&mut s, &mut out);
        }
        if op == "take" {
            while unsafe { s.state.has_more_output() } {
                let mut size = co;
                let bytes =
                    unsafe { s.state.take_output(&mut size) }.expect("source pending output");
                write!(out, " T{}:{}", size, hex(bytes)).unwrap();
                assert_ne!(size, 0);
            }
            let mut size = 0;
            let none = unsafe { s.state.take_output(&mut size) }.is_none();
            write!(out, " E{}:{}", size, none as u8).unwrap();
        }
        assert!(
            output[if op == "take" { 0 } else { co - ao }..]
                .iter()
                .all(|x| *x == 0xcc)
        );
        if result <= 1 || (result == 2 && offset == data.len()) {
            break;
        }
        assert_ne!(call, 999999);
    }
    if op == "replay" {
        for _ in 0..3 {
            let mut ai = data.len();
            let mut ao = co;
            let mut ni = data.as_ptr();
            let mut output = vec![0xcc; co];
            let mut no = output.as_mut_ptr();
            let result = unsafe {
                s.decompress_stream_raw(&bindings, &mut ai, &mut ni, &mut ao, &mut no, &mut total)
            };
            write!(
                out,
                " Q{}:{}:{}:{}:{}:{}:{}",
                result,
                ai,
                ao,
                total,
                s.get_error_code(),
                s.set_parameter(1, 1) as u8,
                hex(&output[..co - ao])
            )
            .unwrap();
            state(&mut s, &mut out);
        }
    }
    drop(s);
    if op == "alloc" || op == "dictalloc" {
        assert!(allocator.blocks.iter().all(|b| b.0.is_null()));
        out.push_str(&allocator.events);
    }
    out
}
fn main() {
    use std::io::BufRead;
    for line in std::io::stdin().lock().lines() {
        println!("{}", observe(&line.unwrap()));
    }
}
