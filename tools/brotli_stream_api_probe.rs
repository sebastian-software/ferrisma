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

#[path = "../src/brotli/stream_api.rs"]
mod stream_api;
use std::fmt::Write;
fn unhex(h: &str) -> Vec<u8> {
    if h == "-" {
        return Vec::new();
    }
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&h[i..i + 2], 16).unwrap())
        .collect()
}
fn hex(v: &[u8]) -> String {
    if v.is_empty() {
        "-".into()
    } else {
        v.iter().map(|b| format!("{b:02x}")).collect()
    }
}
fn state(s: &instance::DecoderInstance<'_>, out: &mut String) {
    let p = s.as_ptr().cast::<layout::DecoderStateStorage>();
    macro_rules! r {
        ($f:ident) => {
            unsafe { (*p).$f.assume_init() }
        };
    }
    write!(
        out,
        " S{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        r!(state),
        s.get_error_code(),
        r!(pos),
        r!(partial_pos_out),
        r!(rb_roundtrips),
        r!(used_input),
        r!(buffer_length),
        s.is_used() as u8,
        s.is_finished() as u8,
        s.has_more_output() as u8
    )
    .unwrap();
}
struct Watch {
    handle: stream_api::DecoderStateHandle,
    setter: instance::MetadataCallbacksHandle,
    mode: u32,
    changed: bool,
    trace: String,
}
unsafe fn observe_watch(w: &mut Watch, kind: char, n: usize) {
    // SAFETY: Actual callback boundary ends all source loans; the producing
    // native allocation is live and no concurrent access/sidecar reference exists.
    write!(
        w.trace,
        " {}{}:{}:{}:{}:{}",
        kind,
        n,
        unsafe { w.handle.get_error_code() },
        unsafe { w.handle.is_used() } as u8,
        unsafe { w.handle.is_finished() } as u8,
        unsafe { w.handle.has_more_output() } as u8
    )
    .unwrap();
}
unsafe extern "C" fn watch_start(p: *mut std::ffi::c_void, n: usize) {
    let w = unsafe { &mut *p.cast::<Watch>() };
    unsafe {
        observe_watch(w, 'A', n);
    }
    if w.mode == 1 && !w.changed {
        unsafe {
            w.setter.set(None, Some(watch_chunk), p);
        }
        w.changed = true;
        w.trace.push_str(" X");
    }
}
unsafe extern "C" fn watch_chunk(p: *mut std::ffi::c_void, data: *const u8, n: usize) {
    let w = unsafe { &mut *p.cast::<Watch>() };
    unsafe {
        observe_watch(w, 'C', n);
    }
    w.trace.push(':');
    for i in 0..n {
        write!(w.trace, "{:02x}", unsafe { data.add(i).read() }).unwrap();
    }
    if w.mode == 2 && !w.changed {
        unsafe {
            w.setter.set(Some(watch_start), None, p);
        }
        w.changed = true;
        w.trace.push_str(" X");
    }
}
pub fn observe(line: &str) -> String {
    let mut w = line.split_whitespace();
    let op = w.next().unwrap();
    let mut dictionary = Vec::new();
    let mut s = instance::DecoderInstance::new().unwrap();
    let mut bindings = compound_dictionary::CompoundDictionaryBindings::default();
    let mut out = String::new();
    match op {
        "api" => {
            let mode: u32 = w.next().unwrap().parse().unwrap();
            let first: usize = w.next().unwrap().parse().unwrap();
            let mut size: usize = w.next().unwrap().parse().unwrap();
            let data = unhex(w.next().unwrap());
            if mode != 3 {
                let mut ai = data.len();
                let mut ao = first;
                let mut total = 999;
                let mut ni = data.as_ptr();
                let mut output = vec![0xcc; first];
                let mut no = output.as_mut_ptr();
                let r = unsafe {
                    s.decompress_stream_raw(
                        &bindings, &mut ai, &mut ni, &mut ao, &mut no, &mut total,
                    )
                };
                write!(
                    out,
                    "R{}:{}:{}:{}:{}",
                    r,
                    ai,
                    ao,
                    total,
                    hex(&output[..first - ao])
                )
                .unwrap();
                state(&s, &mut out);
            } else {
                out.push('F');
                state(&s, &mut out);
            }
            let native = s.as_ptr().cast_mut().cast::<layout::DecoderStateStorage>();
            let pointer = unsafe {
                match mode {
                    1 => std::ptr::addr_of_mut!((*native).partial_pos_out).cast::<usize>(),
                    2 => std::ptr::addr_of_mut!((*native).rb_roundtrips).cast::<usize>(),
                    _ => &mut size,
                }
            };
            let bytes = unsafe { s.take_output_raw(pointer) };
            let count = unsafe { pointer.read() };
            write!(out, " T{}:{}:", count, bytes.is_null() as u8).unwrap();
            if !bytes.is_null() {
                let owner = s.state.ringbuffer.as_ref().expect("actual ring");
                let offset = unsafe { bytes.offset_from(owner.as_ptr()) } as usize;
                assert!(offset <= owner.len() && count <= owner.len() - offset);
                write!(
                    out,
                    "{}:{}",
                    offset,
                    hex(unsafe { std::slice::from_raw_parts(bytes, count) })
                )
                .unwrap();
            } else {
                out.push_str("-:-");
            }
            state(&s, &mut out);
        }
        "attach" => {
            let ty = w.next().unwrap().parse().unwrap();
            let kind: u32 = w.next().unwrap().parse().unwrap();
            let size: usize = w.next().unwrap().parse().unwrap();
            let repeats: usize = w.next().unwrap().parse().unwrap();
            let data = unhex(w.next().unwrap());
            dictionary.extend((0..size).map(|i| (i * 97 + 17) as u8));
            let pointer = if kind != 0 {
                std::ptr::null()
            } else {
                dictionary.as_ptr()
            };
            out.push('D');
            for _ in 0..repeats {
                write!(out, "{}", unsafe {
                    s.attach_dictionary_raw(&mut bindings, ty, size, pointer)
                } as u8)
                .unwrap();
            }
            let native = s.as_ptr().cast::<layout::DecoderStateStorage>();
            let shared = s.state.dictionary.as_ref().unwrap().storage();
            let prefix = unsafe { shared.num_prefix.assume_init() };
            let (chunks, total) = if unsafe { (*native).compound_dictionary.assume_init() } != 0 {
                let p = s
                    .state
                    .compound_dictionary
                    .as_ref()
                    .unwrap()
                    .as_ptr()
                    .cast::<layout::CompoundDictionaryStorage>();
                unsafe { ((*p).num_chunks.assume_init(), (*p).total_size.assume_init()) }
            } else {
                (0, 0)
            };
            write!(out, ":{}:{}:{}", prefix, chunks, total).unwrap();
            let mut ai = data.len();
            let mut ao = 0;
            let mut total = 999;
            let mut ni = data.as_ptr();
            let mut no = std::ptr::null_mut();
            let r = unsafe {
                s.decompress_stream_raw(&bindings, &mut ai, &mut ni, &mut ao, &mut no, &mut total)
            };
            write!(out, " R{}:{}:{}:{}", r, ai, ao, total).unwrap();
            state(&s, &mut out);
            write!(
                out,
                " A{}",
                unsafe { s.attach_dictionary_raw(&mut bindings, 0, size, pointer) } as u8
            )
            .unwrap();
        }
        "watch" => {
            let mode = w.next().unwrap().parse().unwrap();
            let chunk: usize = w.next().unwrap().parse().unwrap();
            let data = unhex(w.next().unwrap());
            let mut watch = Watch {
                handle: s.state_handle(),
                setter: s.metadata_callbacks_handle(),
                mode,
                changed: false,
                trace: String::new(),
            };
            unsafe {
                s.set_metadata_callbacks(
                    Some(watch_start),
                    Some(watch_chunk),
                    std::ptr::addr_of_mut!(watch).cast(),
                );
            }
            let mut offset = 0;
            let mut total = 999;
            for call in 0..1000000 {
                let mut extent = data.len() - offset;
                if chunk != 0 && extent > chunk {
                    extent = chunk;
                }
                let mut ai = extent;
                let mut ao = 0;
                let mut ni = unsafe { data.as_ptr().add(offset) };
                let mut no = std::ptr::null_mut();
                let r = unsafe {
                    s.decompress_stream_raw(
                        &bindings, &mut ai, &mut ni, &mut ao, &mut no, &mut total,
                    )
                };
                offset += extent - ai;
                write!(watch.trace, " R{}:{}:{}", r, ai, total).unwrap();
                if r != 2 || offset == data.len() {
                    break;
                }
                assert_ne!(call, 999999);
            }
            out.push_str(&watch.trace);
            state(&s, &mut out);
        }
        _ => panic!("unknown API fixture"),
    }
    out
}
fn main() {
    use std::io::BufRead;
    for line in std::io::stdin().lock().lines() {
        println!("{}", observe(&line.unwrap()));
    }
}
