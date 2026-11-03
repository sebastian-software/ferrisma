// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::status_code;
use std::fmt::Write as _;
use std::str::FromStr;
fn number<T: FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T
where
    T::Err: std::fmt::Debug,
{
    args.next().unwrap().parse().unwrap()
}
fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_owned();
    }
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

use jxl::jxl::dec_cache::{ImageOutput, PixelCallback};
use jxl::jxl::image::Plane;
use jxl::jxl::image_metadata::Orientation;
use jxl::jxl::memory_manager::{
    JxlMemoryManager, memory_manager_alloc, memory_manager_free, memory_manager_init,
};
use jxl::jxl::render_pipeline::render_pipeline_stage::*;
use jxl::jxl::render_pipeline::stage_write::get_write_to_output_stage;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::collections::BTreeMap;
use std::ffi::c_void;

pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "wo-run" | "wo-mm")
}

#[derive(Clone, Copy)]
struct Cookie {
    state: *mut State,
    id: usize,
}
struct RunCookie {
    init: *mut Cookie,
    id: usize,
}
struct Block {
    _owner: Vec<u8>,
    payload: *mut u8,
    size: usize,
    id: usize,
}
#[derive(Default)]
struct State {
    trace: Vec<String>,
    blocks: BTreeMap<usize, Block>,
    runs: Vec<*mut RunCookie>,
    cookies: Vec<*mut Cookie>,
    outputs: Vec<ImageOutput>,
    allocation_calls: usize,
    init_calls: usize,
    fail_alloc: usize,
    fail_init: usize,
}
// Raw owners avoid a Box/unique-reference reborrow across callbacks that
// legitimately access the externally retained context. Every temporary Rust
// reference ends before a stage operation invokes one of those callbacks.
struct StateOwner(*mut State);
impl Default for StateOwner {
    fn default() -> Self {
        Self(Box::into_raw(Box::default()))
    }
}
impl std::ops::Deref for StateOwner {
    type Target = State;
    fn deref(&self) -> &State {
        // SAFETY: This owner keeps the allocation live and accesses it only
        // between callbacks; no conflicting state reference remains active.
        unsafe { &*self.0 }
    }
}
impl std::ops::DerefMut for StateOwner {
    fn deref_mut(&mut self) -> &mut State {
        // SAFETY: Same protocol, with exclusive short access between calls.
        unsafe { &mut *self.0 }
    }
}
impl Drop for StateOwner {
    fn drop(&mut self) {
        // SAFETY: Stage destruction precedes this unique context owner's Drop.
        unsafe {
            let mut state = Box::from_raw(self.0);
            for token in state.runs.drain(..) {
                drop(Box::from_raw(token));
            }
            for cookie in state.cookies.drain(..) {
                drop(Box::from_raw(cookie));
            }
        }
    }
}
fn sample_size(kind: JxlDataType) -> usize {
    match kind {
        JxlDataType::FLOAT => 4,
        JxlDataType::FLOAT16 | JxlDataType::UINT16 => 2,
        _ => 1,
    }
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The observer keeps both this cookie and its state owner stable
    // through stage destruction; no state borrow crosses a stage call.
    let cookie = unsafe { *opaque.cast::<Cookie>() };
    let state = unsafe { &mut *cookie.state };
    state.allocation_calls += 1;
    let id = state.allocation_calls;
    if id == state.fail_alloc {
        state.trace.push(format!("A {} {id} {size} 0", cookie.id));
        return std::ptr::null_mut();
    }
    let mut owner = vec![0xa5; size + 2048];
    let base = owner.as_mut_ptr();
    let offset = (base as usize).wrapping_neg() & 2047;
    // SAFETY: The 2048 extra bytes cover this alignment offset and the
    // complete requested payload. The vector is never resized or borrowed
    // until the source free checkpoint ends the block's external use.
    let payload = unsafe { base.add(offset) };
    state.blocks.insert(
        payload as usize,
        Block {
            _owner: owner,
            payload,
            size,
            id,
        },
    );
    state.trace.push(format!("A {} {id} {size} 1", cookie.id));
    payload.cast()
}
unsafe extern "C" fn release(opaque: *mut c_void, address: *mut c_void) {
    // SAFETY: Same lifelong cookie/state protocol as allocate. The allocation
    // remains present exactly until this original free checkpoint.
    let cookie = unsafe { *opaque.cast::<Cookie>() };
    let state = unsafe { &mut *cookie.state };
    if address.is_null() {
        state.trace.push(format!("F {} 0 -", cookie.id));
        return;
    }
    let block = state.blocks.remove(&(address as usize)).unwrap();
    // SAFETY: The caller seeded every physical payload byte to 0xa5, and
    // reached stage writes preserve initialization. The block is still owned.
    let bytes = unsafe { std::slice::from_raw_parts(block.payload, block.size) };
    let first = bytes
        .iter()
        .position(|byte| *byte != 0xa5)
        .unwrap_or(bytes.len());
    let last = bytes
        .iter()
        .rposition(|byte| *byte != 0xa5)
        .map_or(first, |at| at + 1);
    state.trace.push(format!(
        "F {} {} {}",
        cookie.id,
        block.id,
        hex(&bytes[first..last])
    ));
}
unsafe extern "C" fn allocate2(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: Same observer-owned stable context and allocation protocol.
    let cookie = unsafe { *opaque.cast::<Cookie>() };
    unsafe { &mut *cookie.state }.trace.push("V A".to_owned());
    unsafe { allocate(opaque, size) }
}
unsafe extern "C" fn release2(opaque: *mut c_void, address: *mut c_void) {
    // SAFETY: Same matching live observer allocation and callback context.
    let cookie = unsafe { *opaque.cast::<Cookie>() };
    unsafe { &mut *cookie.state }.trace.push("V F".to_owned());
    unsafe {
        release(opaque, address);
    }
}
unsafe extern "C" fn init(opaque: *mut c_void, threads: usize, pixels: usize) -> *mut c_void {
    // SAFETY: Cookies and state live in stable boxes through stage Drop.
    let cookie = unsafe { *opaque.cast::<Cookie>() };
    let state = unsafe { &mut *cookie.state };
    state.init_calls += 1;
    let id = state.init_calls;
    let failed = id == state.fail_init;
    let same = std::ptr::eq(opaque.cast::<Cookie>(), state.cookies[cookie.id]);
    state.trace.push(format!(
        "I {id} {} {threads} {pixels} {} {}",
        cookie.id,
        u8::from(same),
        u8::from(!failed)
    ));
    if failed {
        return std::ptr::null_mut();
    }
    let run = Box::into_raw(Box::new(RunCookie {
        init: opaque.cast(),
        id,
    }));
    state.runs.push(run);
    run.cast()
}
unsafe extern "C" fn run(
    opaque: *mut c_void,
    thread: usize,
    x: usize,
    y: usize,
    n: usize,
    pixels: *const c_void,
) {
    // SAFETY: The stage passes the exact live successful-init result. Its
    // temporary output bytes are initialized and valid for this callback only.
    let run = unsafe { &*opaque.cast::<RunCookie>() };
    let cookie = unsafe { *run.init };
    let state = unsafe { &mut *cookie.state };
    let out = state.outputs[cookie.id];
    let len = n * out.format.num_channels as usize * sample_size(out.format.data_type);
    let address = pixels as usize;
    let owner = state
        .blocks
        .values()
        .find(|block| {
            address >= block.payload as usize
                && address + len <= block.payload as usize + block.size
        })
        .map_or(0, |block| block.id);
    let bytes = unsafe { std::slice::from_raw_parts(pixels.cast::<u8>(), len) };
    state.trace.push(format!(
        "R {} {} {thread} {x} {y} {n} {owner} {} {}",
        cookie.id,
        run.id,
        u8::from(address % 128 == 0),
        hex(bytes)
    ));
}
unsafe extern "C" fn destroy(opaque: *mut c_void) {
    // SAFETY: Only the stage's latest retained successful init token is used;
    // the observer owns every token through this destructor call.
    let run = unsafe { &*opaque.cast::<RunCookie>() };
    let cookie = unsafe { *run.init };
    unsafe { &mut *cookie.state }
        .trace
        .push(format!("D {} {}", cookie.id, run.id));
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "wo-mm" {
        return manager_observation(args);
    }
    let width: usize = number(args);
    let height: usize = number(args);
    let has_alpha = number::<u8>(args) != 0;
    let unpremul = number::<u8>(args) != 0;
    let alpha = number(args);
    let orientation = Orientation(number(args));
    let extra_count: usize = number(args);
    let manager_kind: usize = number(args);
    let mut state = StateOwner::default();
    let state_pointer = state.0;
    let mut buffers = Vec::new();
    let transpose = (5..=8).contains(&orientation.0);
    for id in 0..1 + extra_count {
        let kind = JxlDataType(number(args));
        let bits = number(args);
        let n: u32 = number(args);
        let mode: usize = number(args);
        let padding: usize = number(args);
        let endian = JxlEndianness(number(args));
        let stride =
            if transpose { height } else { width } * n as usize * sample_size(kind) + padding;
        let mut buffer = vec![0xa5; stride * if transpose { width } else { height } + 7];
        let address = if mode == 0 || mode == 2 {
            buffer.as_mut_ptr().cast()
        } else {
            std::ptr::null_mut()
        };
        let cookie = Box::into_raw(Box::new(Cookie {
            state: state_pointer,
            id,
        }));
        let callback = if mode == 1 || mode == 2 {
            PixelCallback::new(Some(init), Some(run), Some(destroy), cookie.cast())
        } else {
            PixelCallback::default()
        };
        state.outputs.push(ImageOutput {
            format: JxlPixelFormat {
                num_channels: n,
                data_type: kind,
                endianness: endian,
                align: 0,
            },
            bits_per_sample: bits,
            callback,
            buffer: address,
            buffer_size: buffer.len(),
            stride,
        });
        state.cookies.push(cookie);
        buffers.push(buffer);
    }
    let count: usize = number(args);
    let actions: Vec<_> = (0..count)
        .map(|_| {
            let kind: usize = number(args);
            let mut values = [0usize; 6];
            for value in values.iter_mut().take(if kind == 1 { 6 } else { 1 }) {
                *value = number(args);
            }
            (kind, values)
        })
        .collect();
    let values: [u32; 9] = std::array::from_fn(|_| number(args));
    let mut first = Cookie {
        state: state_pointer,
        id: 1,
    };
    let mut second = Cookie {
        state: state_pointer,
        id: 2,
    };
    let custom = JxlMemoryManager {
        opaque: (&mut first as *mut Cookie).cast(),
        alloc: Some(allocate),
        free: Some(release),
    };
    let mut manager = JxlMemoryManager::default();
    // SAFETY: These initialized independent carriers are live for all calls;
    // the default case uses the source null/default initialization boundary.
    unsafe {
        memory_manager_init(
            &mut manager,
            if manager_kind == 1 {
                std::ptr::null()
            } else {
                &custom
            },
        )
        .unwrap();
    }
    let manager_pointer = if manager_kind == 2 {
        std::ptr::null()
    } else {
        &raw const manager
    };
    // SAFETY: All source descriptors, cookies, allocations and manager remain
    // live and disjoint through stage destruction; wire cases obey reached
    // conversion ranges and callback no-reentry/no-unwind obligations.
    let mut stage = unsafe {
        get_write_to_output_stage(
            &state.outputs[0],
            width,
            height,
            has_alpha,
            unpremul,
            alpha,
            orientation,
            &state.outputs[1..],
            manager_pointer,
        )
    };
    let settings = stage.settings();
    let mut out = format!(
        "{} {} {} {} {} {}",
        stage.get_name(),
        settings.border_x,
        settings.border_y,
        settings.shift_x,
        settings.shift_y,
        status_code(&stage.is_initialized())
    );
    for c in 0..5 + extra_count {
        write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
    }
    write!(out, " {}", stage.get_channel_mode(usize::MAX) as i32).unwrap();
    let row_size = RENDER_PIPELINE_X_OFFSET + width + 16;
    let mut input = Vec::new();
    for c in 0..3 + extra_count {
        let mut plane = Plane::<f32>::create(row_size, 1).unwrap();
        for x in 0..row_size {
            plane.row_mut(0)[x] = f32::from_bits(values[(c * 7 + x) % 9]);
        }
        input.push(plane);
    }
    let rows = (0..input.len())
        .map(|channel| {
            vec![Row {
                storage: RowStorage::Input,
                channel,
                y: 0,
                x: 0,
            }]
        })
        .collect();
    for (kind, a) in actions {
        match kind {
            0 => {
                let status = stage.prepare_for_threads(a[0]);
                state.trace.push(format!("P {}", status_code(&status)));
            }
            1 => {
                let status = stage.process_row(
                    &rows,
                    &Vec::new(),
                    &mut RowBuffers {
                        input: &mut input,
                        output: &mut [],
                    },
                    a[0],
                    a[1],
                    a[2],
                    a[3],
                    a[4],
                    a[5],
                );
                state.trace.push(format!("S {}", status_code(&status)));
            }
            2 => state.fail_alloc = a[0],
            3 => state.fail_init = a[0],
            4 => {
                // SAFETY: This initialized live carrier is accessed only
                // between stage calls. Its retained raw pointer must observe
                // the original mutable-manager field at the next callback.
                unsafe {
                    std::ptr::addr_of_mut!(manager.opaque).write(if a[0] == 1 {
                        (&raw mut first).cast()
                    } else {
                        (&raw mut second).cast()
                    });
                }
            }
            5 => state.outputs[a[0]].buffer = std::ptr::null_mut(),
            6 => {
                // SAFETY: Update the actual live source carrier between stage
                // operations; both callbacks accept this same provider's blocks.
                unsafe {
                    std::ptr::addr_of_mut!(manager.alloc).write(Some(if a[0] == 1 {
                        allocate
                    } else {
                        allocate2
                    }));
                    std::ptr::addr_of_mut!(manager.free).write(Some(if a[0] == 1 {
                        release
                    } else {
                        release2
                    }));
                }
            }
            _ => panic!("unknown output-stage wire action"),
        }
    }
    drop(stage);
    write!(out, " E {}", state.trace.len()).unwrap();
    for event in &state.trace {
        write!(out, " | {event}").unwrap();
    }
    write!(out, " B {}", buffers.len()).unwrap();
    for buffer in buffers {
        write!(out, " {}", hex(&buffer)).unwrap();
    }
    write!(out, " O {}", state.blocks.len()).unwrap();
    out
}

fn manager_observation(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let null_input = number::<u8>(args) != 0;
    let alias = number::<u8>(args) != 0;
    let opaque = number::<u8>(args) != 0;
    let alloc = number::<u8>(args) != 0;
    let free = number::<u8>(args) != 0;
    let size: usize = number(args);
    let state = StateOwner::default();
    let mut first = Cookie {
        state: state.0,
        id: 1,
    };
    let mut second = Cookie {
        state: state.0,
        id: 2,
    };
    let input = JxlMemoryManager {
        opaque: if opaque {
            (&raw mut first).cast()
        } else {
            std::ptr::null_mut()
        },
        alloc: alloc.then_some(allocate),
        free: free.then_some(release),
    };
    let mut result = JxlMemoryManager {
        opaque: (&raw mut second).cast(),
        alloc: Some(allocate2),
        free: Some(release2),
    };
    if alias {
        result = input;
    }
    let this = &raw mut result;
    // SAFETY: The initialized live source carrier may alias itself; no Rust
    // reference conflicts with that raw read/write protocol.
    let status = unsafe {
        memory_manager_init(
            this,
            if null_input {
                std::ptr::null()
            } else if alias {
                this
            } else {
                &raw const input
            },
        )
    };
    let alloc_kind = result.alloc.map_or(0, |f| {
        if f as *const () == allocate as *const () {
            1
        } else {
            2
        }
    });
    let free_kind = result.free.map_or(0, |f| {
        if f as *const () == release as *const () {
            1
        } else {
            2
        }
    });
    let opaque_kind = if result.opaque.is_null() {
        0
    } else if result.opaque == (&raw mut first).cast() {
        1
    } else {
        2
    };
    let mut out = format!(
        "{} {} {} {} {} {} {opaque_kind} {alloc_kind} {free_kind}",
        std::mem::size_of::<JxlMemoryManager>(),
        std::mem::align_of::<JxlMemoryManager>(),
        std::mem::offset_of!(JxlMemoryManager, opaque),
        std::mem::offset_of!(JxlMemoryManager, alloc),
        std::mem::offset_of!(JxlMemoryManager, free),
        status_code(&status)
    );
    if status.is_ok() && (!result.opaque.is_null() || alloc_kind != 1) {
        // SAFETY: A successful source Init establishes this pair. Custom
        // callbacks receive their real context; default callbacks ignore it.
        let block = unsafe { memory_manager_alloc(&raw const result, size) };
        write!(out, " N {}", u8::from(!block.is_null())).unwrap();
        if !block.is_null() {
            for i in 0..size.min(4) {
                // SAFETY: Successful allocation owns this exact requested extent.
                unsafe {
                    block.cast::<u8>().add(i).write(0x31 + i as u8);
                }
            }
        }
        // SAFETY: Null forwarding and the exact matching live block are the
        // original free callback protocol; no payload view remains borrowed.
        unsafe {
            memory_manager_free(&raw const result, std::ptr::null_mut());
            if !block.is_null() {
                memory_manager_free(&raw const result, block);
            }
        }
    }
    write!(out, " E {}", state.trace.len()).unwrap();
    for event in &state.trace {
        write!(out, " | {event}").unwrap();
    }
    write!(out, " O {}", state.blocks.len()).unwrap();
    out
}
