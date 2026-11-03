// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[path = "../tools/brotli_metablock_probe.rs"]
mod observer;

use observer::bit_reader::BitReader;
use observer::{bitfield, metablock};
use std::mem::MaybeUninit;

fn word(storage: &mut MaybeUninit<u32>) -> bitfield::BitfieldWord<'_> {
    // SAFETY: The exclusive live aligned word contains exactly four bytes.
    // No unknown byte is converted to an initialized Rust scalar.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(storage.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4)
    };
    bitfield::BitfieldWord::new(bytes.try_into().unwrap())
}

#[test]
fn window_reader_does_not_require_unknown_neighbor_fields() {
    for byte in [0, 17] {
        let mut flags = MaybeUninit::uninit();
        word(&mut flags).write(5, 1, 1);
        let mut br = BitReader::new();
        br.val = byte;
        br.bit_pos = 8;
        // SAFETY: The source window entry needs only initialized large_window
        // and eight accumulator bits. Other packed fields remain unknown.
        assert_eq!(
            unsafe { metablock::decode_window_bits(&mut flags, &mut br) },
            1
        );
        // SAFETY: DecodeWindowBits always initializes this selected field.
        assert_eq!(
            unsafe { word(&mut flags).read(5, 1) },
            u32::from(byte == 17)
        );
        if byte == 0 {
            // SAFETY: This branch wrote window_bits=16. The large-window
            // marker branch intentionally leaves it unknown until later.
            assert_eq!(unsafe { word(&mut flags).read(6, 6) }, 16);
        }
    }
}

#[test]
fn variable_reader_initializes_output_only_at_reached_checkpoints() {
    let mut state = MaybeUninit::new(0);
    let mut value = MaybeUninit::uninit();
    let mut br = BitReader::new();
    // SAFETY: NONE does not require output initialization. Empty input must
    // return before its first source output write; the value is not inspected.
    assert_eq!(
        unsafe { metablock::decode_var_len_uint8(&mut state, &mut br, &mut value) },
        2
    );
    br.set_input(&[255]);
    // SAFETY: Same NONE state; the first byte initializes LONG's temporary.
    assert_eq!(
        unsafe { metablock::decode_var_len_uint8(&mut state, &mut br, &mut value) },
        2
    );
    // SAFETY: This partial source path wrote both LONG and temporary width7.
    assert_eq!(unsafe { state.assume_init() }, 2);
    assert_eq!(unsafe { value.assume_init() }, 7);
    br.set_input(&[255]);
    // SAFETY: The actual returned LONG checkpoint and its initialized width
    // are retained with the bit-reader accumulator through the next chunk.
    assert_eq!(
        unsafe { metablock::decode_var_len_uint8(&mut state, &mut br, &mut value) },
        1
    );
    // SAFETY: The successful source branch initialized both scalar fields.
    assert_eq!(unsafe { value.assume_init() }, 255);
    assert_eq!(unsafe { state.assume_init() }, 0);
}

#[test]
fn empty_last_metablock_never_reads_unknown_counter_or_size() {
    let mut state = MaybeUninit::new(0);
    let mut flags = MaybeUninit::uninit();
    let mut length = MaybeUninit::uninit();
    let mut counter = MaybeUninit::uninit();
    let mut br = BitReader::new();
    br.val = 1;
    br.bit_pos = 1;
    // SAFETY: NONE requires only its discriminant and reader. Its first bit
    // initializes last/length/uncompressed/metadata, then EMPTY runs out.
    assert_eq!(
        unsafe {
            metablock::decode_meta_block_length(
                metablock::MetaBlockFields {
                    substate: &mut state,
                    flags: &mut flags,
                    remaining_len: &mut length,
                    loop_counter: &mut counter,
                },
                &mut br,
            )
        },
        2
    );
    // SAFETY: This source checkpoint initialized these selected fields only.
    assert_eq!(unsafe { state.assume_init() }, 1);
    assert_eq!(unsafe { length.assume_init() }, 0);
    assert_eq!(unsafe { word(&mut flags).read(0, 1) }, 1);
    br.set_input(&[1]);
    // SAFETY: EMPTY needs only its retained last flag and length. Size and
    // counter remain genuinely unknown and are not read on this branch.
    assert_eq!(
        unsafe {
            metablock::decode_meta_block_length(
                metablock::MetaBlockFields {
                    substate: &mut state,
                    flags: &mut flags,
                    remaining_len: &mut length,
                    loop_counter: &mut counter,
                },
                &mut br,
            )
        },
        1
    );
    // SAFETY: The successful empty branch initialized the discriminant.
    assert_eq!(unsafe { state.assume_init() }, 0);
}

#[test]
fn length_reader_retains_partial_nibbles_and_saved_loop_index() {
    let mut state = MaybeUninit::new(0);
    let mut flags = MaybeUninit::uninit();
    let mut length = MaybeUninit::uninit();
    let mut counter = MaybeUninit::uninit();
    let mut br = BitReader::new();
    br.set_input(&[0]);
    // SAFETY: NONE initializes every reached field before reading it; the
    // first SIZE loop reads one nibble then saves its exact counter.
    assert_eq!(
        unsafe {
            metablock::decode_meta_block_length(
                metablock::MetaBlockFields {
                    substate: &mut state,
                    flags: &mut flags,
                    remaining_len: &mut length,
                    loop_counter: &mut counter,
                },
                &mut br,
            )
        },
        2
    );
    // SAFETY: The source shortage path initialized each inspected field.
    assert_eq!(unsafe { state.assume_init() }, 3);
    assert_eq!(unsafe { counter.assume_init() }, 1);
    assert_eq!(unsafe { length.assume_init() }, 0);
    br.set_input(&[0, 0]);
    // SAFETY: The retained actual SIZE checkpoint supplies initialized count,
    // nibble size and partial length. Remaining flag neighbors stay unknown.
    assert_eq!(
        unsafe {
            metablock::decode_meta_block_length(
                metablock::MetaBlockFields {
                    substate: &mut state,
                    flags: &mut flags,
                    remaining_len: &mut length,
                    loop_counter: &mut counter,
                },
                &mut br,
            )
        },
        1
    );
    // SAFETY: Successful source completion wrote length; the saved counter
    // stays1 because the source's later loop counter is a local scalar.
    assert_eq!(unsafe { length.assume_init() }, 1);
    assert_eq!(unsafe { counter.assume_init() }, 1);
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn actual_original_header_statuses_and_chunk_traces() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let requests = [
        "0 165 0 17 8 1 0 -",
        "0 90 0 17 8 0 0 -",
        "1 165 0 0 0 0 1 ffff",
        "1 90 0 0 0 0 1 -",
        "2 165 0 0 0 0 1 ff0000000000",
        "2 0 0 0 0 0 1 0000000000",
        "2 90 0 0 0 0 1 0e00000000",
        "2 255 0 0 0 0 2 760000000000",
        "1 165 99 0 0 0 1 -",
        "2 165 99 0 0 0 1 -",
    ];
    let mut child = Command::new(
        std::env::var_os("BROTLI_METABLOCK_ORACLE").expect("set actual pinned source observer"),
    )
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all((requests.join("\n") + "\n").as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let original = String::from_utf8(output.stdout).unwrap();
    let records: Vec<_> = original.lines().collect();
    assert_eq!(records.len(), requests.len());
    for (request, record) in requests.iter().zip(records) {
        assert_eq!(
            observer::observe(request),
            format!("{record}\n"),
            "{request}"
        );
    }
}
