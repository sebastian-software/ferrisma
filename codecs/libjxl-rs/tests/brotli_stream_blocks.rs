// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/bitfield.rs"]
pub mod bitfield;
#[path = "../src/brotli/default_memory.rs"]
pub mod default_memory;
#[path = "../src/brotli/instance.rs"]
pub mod instance;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/reader_copy.rs"]
pub mod reader_copy;
#[path = "../src/brotli/ringbuffer.rs"]
pub mod ringbuffer;
#[path = "../src/brotli/running.rs"]
pub mod running;
#[path = "../src/brotli/shared_dictionary.rs"]
pub mod shared_dictionary;
#[path = "../src/brotli/state.rs"]
pub mod state;
#[path = "../src/brotli/static_descriptors.rs"]
pub mod static_descriptors;
#[path = "../src/brotli/status.rs"]
pub mod status;
#[path = "../src/brotli/stream_blocks.rs"]
pub mod stream_blocks;
#[path = "../src/brotli/stream_input.rs"]
pub mod stream_input;
#[path = "../src/brotli/stream_output.rs"]
pub mod stream_output;
#[path = "../src/brotli/tables.rs"]
pub mod tables;
use instance::DecoderInstance;
use stream_input::SourceInput;
#[cfg(feature = "cpp-oracle")]
mod parity {
    use super::*;
    use instance::MetadataCallbacksHandle;
    use layout::DecoderStateStorage;
    use std::ffi::c_void;
    use std::fmt::Write;
    use std::mem::MaybeUninit;
    struct Context {
        root: *mut Root,
        id: u8,
    }
    struct Root {
        state: *const DecoderStateStorage,
        input: *const u8,
        mode: u8,
        calls: usize,
        context: [Context; 2],
        handle: MetadataCallbacksHandle,
        trace: String,
    }
    unsafe extern "C" fn chunk0(opaque: *mut c_void, data: *const u8, size: usize) {
        unsafe { chunk(0, opaque, data, size) }
    }
    unsafe extern "C" fn chunk1(opaque: *mut c_void, data: *const u8, size: usize) {
        unsafe { chunk(1, opaque, data, size) }
    }
    unsafe fn chunk(which: u8, opaque: *mut c_void, data: *const u8, size: usize) {
        // SAFETY: Both exact context objects and the root stay at their original
        // stack addresses throughout the actual decoder/callback lifetime. Copy
        // the child metadata before lending the enclosing root exclusively.
        let (p, id) = unsafe {
            let c = &*opaque.cast::<Context>();
            (c.root, c.id)
        };
        let root = unsafe { &mut *p };
        let (remaining, val, bits, next, direct) = unsafe {
            let state = &*root.state;
            let br = &*state.br.as_ptr();
            (
                state.meta_block_remaining_len.assume_init(),
                br.val.assume_init(),
                br.bit_pos.assume_init(),
                br.next_in.assume_init() - root.input as usize,
                br.next_in.assume_init() == data as usize,
            )
        };
        write!(
            root.trace,
            "C{which}:{id}:{size}:{remaining}:{val}:{bits}:{next}:{}:",
            u8::from(direct)
        )
        .unwrap();
        // SAFETY: Callback advertises exactly the reached initialized metadata
        // prefix. This short data reference ends before callback fields change.
        for byte in unsafe { std::slice::from_raw_parts(data, size) } {
            write!(root.trace, "{byte:02x}").unwrap();
        }
        root.trace.push(' ');
        let first = root.calls == 0;
        root.calls += 1;
        if first {
            let opaque = std::ptr::from_mut(&mut root.context[1]).cast();
            if root.mode == 2 {
                unsafe { root.handle.set(None, Some(chunk1), opaque) };
            }
            if root.mode == 3 {
                unsafe { root.handle.set(None, None, opaque) };
            }
        }
    }
    fn observe(
        op: u8,
        mode: u8,
        remaining: i32,
        bits: u64,
        n: usize,
        capacity: usize,
        position: i32,
    ) -> String {
        let input: Vec<u8> = (0..32).map(|i| 0x80 + i).collect();
        let mut decoder = DecoderInstance::new().unwrap();
        let mut root = Root {
            state: decoder.as_ptr().cast(),
            input: input.as_ptr(),
            mode,
            calls: 0,
            context: [
                Context {
                    root: std::ptr::null_mut(),
                    id: 0,
                },
                Context {
                    root: std::ptr::null_mut(),
                    id: 1,
                },
            ],
            handle: decoder.metadata_callbacks_handle(),
            trace: String::new(),
        };
        let pointer = std::ptr::from_mut(&mut root);
        root.context[0].root = pointer;
        root.context[1].root = pointer;
        if mode != 0 {
            unsafe {
                decoder.set_metadata_callbacks(
                    None,
                    Some(chunk0),
                    std::ptr::from_mut(&mut root.context[0]).cast(),
                );
            }
        }
        let view = SourceInput::from_slice(&input[..n]);
        unsafe {
            let br = &mut *decoder.state.storage_mut().br.as_mut_ptr();
            view.set_input(br, 0, n);
            br.val.write(
                0x0807060504030201
                    & if bits == 64 {
                        u64::MAX
                    } else {
                        (1 << bits) - 1
                    },
            );
            br.bit_pos.write(bits);
        }
        decoder
            .state
            .storage_mut()
            .meta_block_remaining_len
            .write(remaining);
        let mut output = [MaybeUninit::new(0xccu8); 2048];
        let mut ao = capacity;
        let mut total = 1234;
        let result = if op == 0 {
            unsafe { decoder.skip_metadata_block(view) }
        } else {
            decoder.state.write_flag(6, 6, 10);
            decoder
                .state
                .storage_mut()
                .max_backward_distance
                .write(1008);
            decoder.state.storage_mut().new_ringbuffer_size.write(1024);
            assert!(unsafe { decoder.state.ensure_ringbuffer() });
            for byte in
                &mut decoder.state.ringbuffer.as_mut().unwrap().storage_mut()[..position as usize]
            {
                byte.write(0x33);
            }
            decoder.state.storage_mut().pos.write(position);
            let mut next = output.as_mut_ptr().cast::<u8>();
            unsafe {
                decoder.copy_uncompressed_block_to_output_raw(
                    view,
                    std::ptr::addr_of_mut!(ao),
                    std::ptr::addr_of_mut!(next),
                    std::ptr::addr_of_mut!(total),
                )
            }
        };
        unsafe {
            let state = decoder.state.storage();
            let br = &*state.br.as_ptr();
            write!(
                root.trace,
                "R{result} {} {} {} {} {} {}",
                state.meta_block_remaining_len.assume_init(),
                br.val.assume_init(),
                br.bit_pos.assume_init(),
                br.next_in.assume_init() - input.as_ptr() as usize,
                br.guard_in.assume_init() - input.as_ptr() as usize,
                br.last_in.assume_init() - input.as_ptr() as usize
            )
            .unwrap();
            if op == 1 {
                let pos = state.pos.assume_init();
                write!(
                    root.trace,
                    " {pos} {} {ao} {total} {} ",
                    state.partial_pos_out.assume_init(),
                    state.substate_uncompressed.assume_init()
                )
                .unwrap();
                for byte in &output[..32] {
                    write!(root.trace, "{:02x}", byte.assume_init()).unwrap();
                }
                root.trace.push(' ');
                for byte in decoder
                    .state
                    .ringbuffer
                    .as_ref()
                    .unwrap()
                    .initialized_bytes(0..pos as usize)
                {
                    write!(root.trace, "{byte:02x}").unwrap();
                }
            }
        }
        root.trace.push('\n');
        drop(decoder);
        root.trace
    }
    #[cfg(feature = "cpp-oracle")]
    #[test]
    fn actual_metadata_callbacks_and_uncompressed_partial_states_match_original() {
        use std::io::Write as IoWrite;
        use std::process::{Command, Stdio};
        let mut requests = String::new();
        let mut expected = String::new();
        let mut cases = 0;
        for remaining in [0, 1, 2, 3, 7, 8, 9, 32, 40, 1024, 1030] {
            for bits in (0..=64).step_by(8) {
                for n in [0, 1, 3, 7, 32] {
                    for mode in 0..=3 {
                        writeln!(requests, "0 {mode} {remaining} {bits} {n} 0 0").unwrap();
                        expected += &observe(0, mode, remaining, bits, n, 0, 0);
                        cases += 1;
                    }
                    for position in [0, 1018, 1024] {
                        for capacity in [0, 1, 16, 1024, 2048] {
                            writeln!(requests, "1 0 {remaining} {bits} {n} {capacity} {position}")
                                .unwrap();
                            expected += &observe(1, 0, remaining, bits, n, capacity, position);
                            cases += 1;
                        }
                    }
                }
            }
        }
        let mut child = Command::new(std::env::var_os("BROTLI_STREAM_BLOCKS_ORACLE").unwrap())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let capture =
            std::env::var_os("BROTLI_STREAM_BLOCKS_CAPTURE").map(std::path::PathBuf::from);
        if let Some(path) = &capture {
            std::fs::create_dir_all(path).unwrap();
            std::fs::write(path.join("requests.txt"), &requests).unwrap();
            std::fs::write(path.join("rust.txt"), &expected).unwrap();
        }
        let mut stdin = child.stdin.take().unwrap();
        let writer = std::thread::spawn(move || stdin.write_all(requests.as_bytes()).unwrap());
        let actual = child.wait_with_output().unwrap();
        writer.join().unwrap();
        assert!(
            actual.status.success(),
            "{}",
            String::from_utf8_lossy(&actual.stderr)
        );
        assert!(actual.stderr.is_empty());
        if let Some(path) = &capture {
            std::fs::write(path.join("source.txt"), &actual.stdout).unwrap();
            std::fs::write(path.join("counts.txt"), format!("{cases}\n")).unwrap();
        }
        assert_eq!(
            actual.stdout,
            expected.as_bytes(),
            "{cases} source-defined helper fixtures"
        );
    }
}
#[test]
fn zero_metadata_remaining_does_not_read_unknown_reader_pointers() {
    let mut decoder = DecoderInstance::new().unwrap();
    decoder
        .state
        .storage_mut()
        .meta_block_remaining_len
        .write(0);
    // StateInit leaves native reader pointer fields genuinely unknown. The
    // source helper's first remaining==0 return must not inspect them.
    assert_eq!(
        unsafe { decoder.skip_metadata_block(SourceInput::from_slice(&[])) },
        status::BROTLI_DECODER_SUCCESS
    );
}
