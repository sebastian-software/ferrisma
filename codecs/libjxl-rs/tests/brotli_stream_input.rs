// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/stream_input.rs"]
pub mod stream_input;
use layout::BitReaderStorage;
use std::mem::MaybeUninit;
use stream_input::SourceInput;
fn native() -> BitReaderStorage {
    // SAFETY: Every native storage field is MaybeUninit, accepting unknown
    // source pointer slots. Init writes only these two accumulator fields.
    let mut br = unsafe { MaybeUninit::<BitReaderStorage>::uninit().assume_init() };
    br.val.write(0);
    br.bit_pos.write(0);
    br
}
#[test]
fn source_set_input_does_not_read_unknown_accumulator_or_pointer_fields() {
    let input = [1, 2, 3];
    let view = SourceInput::from_slice(&input);
    // SAFETY: Every field MaybeUninit, no unknown scalar reference is formed.
    let mut br = unsafe { MaybeUninit::<BitReaderStorage>::uninit().assume_init() };
    view.set_input(&mut br, 0, 3);
    // SAFETY: Exactly these pointer slots were written by SetInput.
    unsafe {
        assert_eq!(br.next_in.assume_init(), input.as_ptr() as usize);
        assert_eq!(br.guard_in.assume_init(), input.as_ptr() as usize);
        assert_eq!(br.last_in.assume_init(), input.as_ptr().add(3) as usize);
    }
}
#[test]
fn native_null_zero_identity_is_not_replaced_by_a_slice_pointer() {
    let mut br = native();
    // SAFETY: Source NULL/zero input has no payload and is never dereferenced.
    let view = unsafe { SourceInput::from_raw_parts(std::ptr::null(), 0) };
    view.set_input(&mut br, 0, 0);
    // SAFETY: Init and actual SetInput initialized all reached reader fields;
    // zero-bit success and empty pull failure perform no pointer arithmetic.
    unsafe {
        view.with_reader(&mut br, |reader| {
            let mut value = 99;
            assert!(reader.safe_read_bits(0, &mut value));
            assert_eq!(value, 0);
            assert!(!reader.safe_read_bits(1, &mut value));
        });
    }
    // SAFETY: Actual SetInput/flush initialized all three native pointer slots.
    unsafe {
        assert_eq!(br.next_in.assume_init(), 0);
        assert_eq!(br.guard_in.assume_init(), 0);
        assert_eq!(br.last_in.assume_init(), 0);
        assert!(view.next_pointer(&br).is_null());
    }
}
#[test]
fn raw_input_mutation_between_ended_loans_is_read_from_the_same_allocation() {
    let mut input = [1u8, 2, 3, 4, 5, 6];
    let pointer = input.as_mut_ptr();
    let mut br = native();
    // SAFETY: Actual input remains live/initialized; no byte reference is kept
    // across loans. Caller writes use the same raw provenance between calls.
    let view = unsafe { SourceInput::from_raw_parts(pointer, 6) };
    view.set_input(&mut br, 0, 6);
    unsafe {
        view.with_reader(&mut br, |reader| {
            let mut v = 0;
            assert!(reader.safe_read_bits(8, &mut v));
            assert_eq!(v, 1);
        });
        pointer.add(1).write(91);
        view.with_reader(&mut br, |reader| {
            let mut v = 0;
            assert!(reader.safe_read_bits(8, &mut v));
            assert_eq!(v, 91);
        });
    }
    // SAFETY: Both actual source pulls initialized next_in to pointer+2.
    assert_eq!(unsafe { view.next_pointer(&br) }, unsafe { pointer.add(2) });
}
#[test]
fn internal_buffer_unknown_capacity_is_not_borrowed_as_initialized_input() {
    let mut buffer = [MaybeUninit::<u8>::uninit(); 8];
    let p = buffer.as_mut_ptr().cast::<u8>();
    // SAFETY: Only source initialized bytes0..3 are written; bytes3..8 stay
    // genuinely unknown and are outside the lent input extent.
    unsafe {
        p.write(1);
        p.add(1).write(2);
        p.add(2).write(3);
    }
    let view = unsafe { SourceInput::from_raw_parts(p, 3) };
    let mut br = native();
    view.set_input(&mut br, 0, 3);
    unsafe {
        view.with_reader(&mut br, |reader| {
            let mut value = 0;
            assert!(reader.safe_read_bits(24, &mut value));
            assert_eq!(value, 0x030201);
        });
        assert_eq!(br.next_in.assume_init(), p.add(3) as usize);
    }
}
#[test]
fn guard_behind_next_survives_native_loan_flush_and_reload() {
    let input: Vec<u8> = (0..32).collect();
    let view = SourceInput::from_slice(&input);
    let mut br = native();
    view.set_input(&mut br, 0, 32);
    unsafe {
        view.with_reader(&mut br, |reader| reader.drop_bytes(7));
        assert_eq!(br.guard_in.assume_init(), input.as_ptr().add(5) as usize);
        assert_eq!(br.next_in.assume_init(), input.as_ptr().add(7) as usize);
        view.with_reader(&mut br, |reader| {
            assert_eq!(reader.guard_position(), 5);
            assert_eq!(reader.position(), 7);
            assert!(!reader.check_input_amount());
        });
    }
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn original_null_empty_and_mutable_input_observations_are_retained() {
    use std::io::Write;
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
        .write_all(b"2 0 0 0 0 0 0\n3 0 0 0 6 0 0\n")
        .unwrap();
    let actual = child.wait_with_output().unwrap();
    assert!(actual.status.success());
    assert!(actual.stderr.is_empty());
    assert_eq!(actual.stdout, b"N1 0 0 0 0 1 1 1\nM1 91 0 0 2 0 6\n");
}
