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
#[path = "../src/brotli/stream_output.rs"]
pub mod stream_output;
#[path = "../src/brotli/tables.rs"]
pub mod tables;
use instance::DecoderInstance;
use std::fmt::Write;
use std::mem::MaybeUninit;
fn observations() -> String {
    let mut rows = String::new();
    for mode in 0..4 {
        let mut decoder = DecoderInstance::new().unwrap();
        decoder.state.metablock_begin();
        decoder.state.write_flag(6, 6, 10);
        decoder.state.storage_mut().new_ringbuffer_size.write(1024);
        assert!(unsafe { decoder.state.ensure_ringbuffer() });
        decoder.state.storage_mut().pos.write(8);
        for (i, byte) in decoder.state.ringbuffer.as_mut().unwrap().storage_mut()[..8]
            .iter_mut()
            .enumerate()
        {
            byte.write(i as u8 + 1);
        }
        let mut output = [MaybeUninit::new(0xccu8); 2048];
        let destination = output.as_mut_ptr().cast::<u8>();
        let mut ao: usize = if mode == 0 { 3 } else { 8 };
        let mut total = 1234usize;
        let mut next = destination;
        let expected;
        if mode == 0 {
            next = destination;
            expected = unsafe { destination.add(3) };
        } else if mode == 1 {
            decoder.state.ringbuffer.as_mut().unwrap().storage_mut()[..8]
                .copy_from_slice(&20usize.to_ne_bytes().map(MaybeUninit::new));
            next = std::ptr::addr_of_mut!(ao).cast();
            expected = unsafe { next.add(8) };
        } else if mode == 2 {
            decoder.state.ringbuffer.as_mut().unwrap().storage_mut()[..8]
                .copy_from_slice(&20usize.to_ne_bytes().map(MaybeUninit::new));
            next = std::ptr::addr_of_mut!(total).cast();
            expected = unsafe { next.add(8) };
        } else {
            // SAFETY: Copy the actual pointer object's initialized byte
            // representation/provenance into the actual ring backing, matching
            // source memcpy(&destination). No integer fabricates a pointer.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(destination).cast::<MaybeUninit<u8>>(),
                    decoder
                        .state
                        .ringbuffer
                        .as_mut()
                        .unwrap()
                        .storage_mut()
                        .as_mut_ptr(),
                    8,
                );
            }
            next = std::ptr::addr_of_mut!(next).cast();
            expected = unsafe { destination.add(8) };
        }
        let total_out = if mode == 0 {
            std::ptr::addr_of_mut!(ao)
        } else {
            std::ptr::addr_of_mut!(total)
        };
        // SAFETY: Every actual caller cell/output object is live/initialized;
        // byte copies are nonoverlapping. Deliberate scalar/character aliases
        // remain raw, without any simultaneous aliased Rust references.
        let result = unsafe {
            decoder.write_ringbuffer_raw(
                std::ptr::addr_of_mut!(ao),
                std::ptr::addr_of_mut!(next),
                total_out,
                false,
            )
        };
        write!(
            rows,
            "A{mode} R{result} AO{ao} T{} P{} S{} POS{} O",
            if mode == 0 { ao } else { total },
            u8::from(next == expected),
            unsafe { decoder.state.storage().partial_pos_out.assume_init() },
            unsafe { decoder.state.storage().pos.assume_init() }
        )
        .unwrap();
        for byte in &output[..8] {
            write!(rows, "{:02x}", unsafe { byte.assume_init() }).unwrap();
        }
        rows.push('\n');
    }
    rows
}
#[test]
fn raw_cursor_stores_observe_scalar_and_character_aliases() {
    // Actual source capture recipe: feed modes0..3 as rows
    // `4 MODE 0 0 0 0 0` to tools/oracle/brotli_stream_blocks.c.
    let rows = observations();
    assert!(rows.contains("A0 R3 AO3 T3 P1 S3 POS8 O010203cccccccccc\n"));
    assert!(rows.contains("A1 R1 AO12 T8 P1 S8 POS8 Occcccccccccccccc\n"));
    assert!(rows.contains("A2 R1 AO0 T8 P1 S8 POS8 Occcccccccccccccc\n"));
    assert!(rows.contains("A3 R1 AO0 T8 P1 S8 POS8 Occcccccccccccccc\n"));
}
#[cfg(feature = "cpp-oracle")]
#[test]
fn every_raw_cursor_alias_matches_actual_original_checkpoints() {
    use std::io::Write as IoWrite;
    use std::process::{Command, Stdio};
    let mut child = Command::new(std::env::var_os("BROTLI_STREAM_BLOCKS_ORACLE").unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"4 0 0 0 0 0 0\n4 1 0 0 0 0 0\n4 2 0 0 0 0 0\n4 3 0 0 0 0 0\n")
        .unwrap();
    let actual = child.wait_with_output().unwrap();
    assert!(actual.status.success(), "{:?}", actual.stderr);
    assert!(actual.stderr.is_empty());
    assert_eq!(actual.stdout, observations().as_bytes());
}
