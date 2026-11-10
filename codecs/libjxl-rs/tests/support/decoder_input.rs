// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::decoder_input::{CodestreamSpan, DecoderInput};
use jxl::jxl::padded_bytes_callbacks::{MemoryManager, PaddedBytes};
use std::alloc::{Layout, alloc, dealloc};
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::fmt::Write;

fn hex(data: &[u8]) -> String {
    if data.is_empty() {
        return "-".into();
    }
    data.iter().map(|v| format!("{v:02x}")).collect()
}
fn unhex(data: &str) -> Vec<u8> {
    if data == "-" {
        return Vec::new();
    }
    data.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect()
}
fn number<T: std::str::FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T {
    args.next().unwrap().parse().ok().unwrap()
}
struct Allocator {
    calls: usize,
    fail: usize,
    next: usize,
    live: BTreeMap<usize, (usize, Layout)>,
    events: Vec<String>,
    mutation: *mut u8,
    mutation_value: u8,
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: Driver retains this actual allocator and exclusive callback scope.
    let a = unsafe { &mut *opaque.cast::<Allocator>() };
    a.calls += 1;
    if !a.mutation.is_null() {
        unsafe {
            a.mutation.write(a.mutation_value);
        }
    }
    a.events.push(format!("A{size}"));
    if a.calls == a.fail {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size, 1).unwrap();
    // SAFETY: Nonzero checked allocation layout, retained for matching free.
    let p = unsafe { alloc(layout) };
    if !p.is_null() {
        a.next += 1;
        a.live.insert(p as usize, (a.next, layout));
    }
    p.cast()
}
unsafe extern "C" fn release(opaque: *mut c_void, p: *mut c_void) {
    // SAFETY: Same live exclusive driver; callback owns exactly one allocation.
    let a = unsafe { &mut *opaque.cast::<Allocator>() };
    if p.is_null() {
        a.events.push("F0".into());
        return;
    }
    let (id, layout) = a.live.remove(&(p as usize)).unwrap();
    a.events.push(format!("F{id}"));
    // SAFETY: Unique callback-returned pointer and its exact saved layout.
    unsafe {
        dealloc(p.cast(), layout);
    }
}

pub fn observe(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut a = Allocator {
        calls: 0,
        fail: number(args),
        next: 0,
        live: BTreeMap::new(),
        events: Vec::new(),
        mutation: std::ptr::null_mut(),
        mutation_value: 0,
    };
    let mm = MemoryManager {
        opaque: (&mut a as *mut Allocator).cast(),
        alloc: Some(allocate),
        free: Some(release),
    };
    // SAFETY: Stable manager/driver outlive decoder; all operations below use
    // retained disjoint input buffers and source-defined histories.
    let mut dec = unsafe { DecoderInput::new(&mm) };
    dec.box_contents_unbounded = true;
    let mut buffers: Vec<Vec<u8>> = Vec::new();
    let mut span = CodestreamSpan {
        data: std::ptr::null(),
        size: 0,
    };
    let mut out = String::new();
    while let Some(op) = args.next() {
        let status: i32;
        let mut detail = "-".to_owned();
        match op {
            "set" => {
                let null = number::<u8>(args) != 0;
                let mut b = unhex(args.next().unwrap());
                let n = b.len();
                b.resize(n + 1, 0);
                buffers.push(b);
                let p = if null {
                    std::ptr::null()
                } else {
                    buffers.last().unwrap().as_ptr()
                };
                status = unsafe { dec.set_input(p, n) }.0 as i32;
            }
            "mutate" => {
                let index: usize = number(args);
                let offset: usize = number(args);
                let value: u8 = number(args);
                a.mutation = unsafe { buffers[index].as_mut_ptr().add(offset) };
                a.mutation_value = value;
                status = 0;
            }
            "release" => status = dec.release_input() as i32,
            "close" => {
                dec.close_input();
                status = 0;
            }
            "advance" => {
                unsafe {
                    dec.advance_codestream(number(args));
                }
                status = 0;
            }
            "raw" => {
                unsafe {
                    dec.advance_input(number(args));
                }
                status = 0;
            }
            "request" => status = unsafe { dec.request_more_input() }.0 as i32,
            "get" | "get-check" => {
                let previous = span;
                status = unsafe { dec.get_codestream_input(&mut span) }.0 as i32;
                if status == 0 {
                    detail = if span.size == 0 {
                        "-".into()
                    } else {
                        hex(unsafe { std::slice::from_raw_parts(span.data, span.size) })
                    };
                }
                if op == "get-check" {
                    if status != 0 {
                        detail = format!(
                            "U{}",
                            u8::from(span.data == previous.data && span.size == previous.size)
                        );
                    } else {
                        detail = format!(
                            "{}:{detail}:{}",
                            u8::from(!span.data.is_null()),
                            u8::from(
                                span.data
                                    == dec
                                        .codestream_copy
                                        .as_ptr()
                                        .wrapping_add(dec.codestream_pos)
                            )
                        );
                    }
                }
            }
            "inject" => {
                let mut success = true;
                let result = unsafe { dec.inject_next_buffered_jxlp_box(&mut success) };
                status = result.map_or_else(|e| e as i32, |_| 0);
                detail = u8::from(success).to_string();
            }
            "queue" => {
                let index = number(args);
                let last = number::<u8>(args) != 0;
                let data = unhex(args.next().unwrap());
                let mut padded = Box::new(unsafe { PaddedBytes::new(&mm) });
                let result =
                    unsafe { padded.append_raw(data.as_ptr(), data.as_ptr().add(data.len())) };
                status = result.map_or_else(|e| e as i32, |_| 0);
                if result.is_ok() {
                    dec.jxlp_ooo_buffer_total += padded.len();
                    dec.jxlp_ooo_buffer.entry(index).or_insert((padded, last));
                }
            }
            "box" => {
                dec.box_contents_unbounded = number::<u8>(args) != 0;
                dec.box_contents_end = number(args);
                status = 0;
            }
            "seed" => {
                dec.codestream_pos = number(args);
                dec.codestream_unconsumed = number(args);
                dec.jxlp_ooo_buffer_total = number(args);
                dec.next_jxlp_index = number(args);
                dec.file_pos = number(args);
                status = 0;
            }
            "limit" => status = i32::from(dec.can_add_buffer(number(args))),
            _ => panic!("input operation"),
        }
        let mut owner = -1i64;
        let mut offset = 0;
        if !unsafe { dec.input.next_in.assume_init() }.is_null() {
            for (i, b) in buffers.iter().enumerate() {
                let lo = b.as_ptr() as usize;
                let p = unsafe { dec.input.next_in.assume_init() } as usize;
                if p >= lo && p < lo + b.len() {
                    owner = i as i64;
                    offset = p - lo;
                }
            }
        }
        write!(
            out,
            " [ {op} {status} {detail} {owner} {offset} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            unsafe { dec.input.avail_in.assume_init() },
            u8::from(unsafe { dec.input.input_closed.assume_init() }),
            dec.file_pos,
            u8::from(dec.box_contents_unbounded),
            dec.box_contents_end,
            dec.codestream_copy.len(),
            dec.codestream_copy.capacity(),
            hex(dec.codestream_copy.as_slice()),
            dec.codestream_pos,
            dec.codestream_unconsumed,
            dec.jxlp_ooo_buffer_total,
            dec.next_jxlp_index,
            u8::from(dec.last_codestream_seen),
            dec.jxlp_ooo_buffer.len()
        )
        .unwrap();
        for (index, (bytes, last)) in &dec.jxlp_ooo_buffer {
            write!(
                out,
                " {index}:{}:{}",
                u8::from(*last),
                hex(bytes.as_slice())
            )
            .unwrap();
        }
        out.push_str(" ]");
    }
    drop(dec);
    out.push_str(" E");
    for e in &a.events {
        write!(out, " {e}").unwrap();
    }
    assert!(a.live.is_empty());
    out
}

pub fn observe_short(args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::jxl::decoder_input_cursor::InputCursor;
    use std::mem::MaybeUninit;
    let mode: usize = number(args);
    let n: usize = number(args);
    let byte = 0u8;
    let mut cursor = InputCursor {
        next_in: MaybeUninit::uninit(),
        avail_in: MaybeUninit::uninit(),
        input_closed: MaybeUninit::uninit(),
    };
    // SAFETY: Each mode initializes precisely the fields reached by that source
    // short-circuit. Outputs observe only fields actually assigned/retained there.
    unsafe {
        match mode {
            0 => {
                cursor.next_in.write(&byte);
                let s = cursor.set_input(std::ptr::null(), n);
                format!(
                    "{} {}",
                    s.0,
                    u8::from(cursor.next_in.assume_init() == &byte)
                )
            }
            1 => {
                cursor.next_in.write(std::ptr::null());
                cursor.input_closed.write(true);
                let s = cursor.set_input(&byte, n);
                format!(
                    "{} {} {}",
                    s.0,
                    u8::from(!cursor.next_in.assume_init().is_null()),
                    u8::from(cursor.input_closed.assume_init())
                )
            }
            2 => {
                cursor.avail_in.write(n);
                let s = cursor.release_input();
                format!(
                    "{s} {} {}",
                    u8::from(!cursor.next_in.assume_init().is_null()),
                    cursor.avail_in.assume_init()
                )
            }
            3 => {
                cursor.close_input();
                format!("{}", u8::from(cursor.input_closed.assume_init()))
            }
            _ => {
                cursor.next_in.write(std::ptr::null());
                cursor.input_closed.write(false);
                let s = cursor.set_input(&byte, 0);
                format!(
                    "{} {} {}",
                    s.0,
                    u8::from(cursor.next_in.assume_init() == &byte),
                    cursor.avail_in.assume_init()
                )
            }
        }
    }
}
