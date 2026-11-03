// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
use bit_reader::{BitReader, SourceReaderState};

#[test]
fn source_loan_keeps_guard_behind_current_input_position() {
    let input: Vec<u8> = (0..32).collect();
    let mut original = BitReader::new();
    original.set_input(&input);
    original.drop_bytes(7);
    let checkpoint = original.source_state();
    assert_eq!(
        checkpoint,
        SourceReaderState {
            val: 0,
            bit_pos: 0,
            next: 7,
            guard: 5,
            last: 32
        }
    );
    let resumed = BitReader::from_source_state(&input, checkpoint);
    assert_eq!(resumed.source_state(), checkpoint);
    assert!(!resumed.check_input_amount());
    assert_eq!(resumed.available_in(), 25);
}

#[test]
fn empty_source_loan_does_not_claim_native_null_pointer_identity() {
    // Source NULL identity belongs to the storage adapter, never input.as_ptr().
    // No pointer is exposed, rebuilt, or inferred by this value-only loan.
    let fields = SourceReaderState {
        val: 0,
        bit_pos: 0,
        next: 0,
        guard: 0,
        last: 0,
    };
    assert_eq!(
        BitReader::from_source_state(&[], fields).source_state(),
        fields
    );
}

#[cfg(all(
    feature = "cpp-oracle",
    any(target_arch = "aarch64", target_arch = "x86_64")
))]
#[test]
fn actual_source_reader_fields_survive_every_reached_loan_boundary() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut rows = String::new();
    let mut expected = String::new();
    let input: Vec<u8> = (0..32).collect();
    for skip in 0..=32 {
        for width in 0..=32 {
            rows.push_str(&format!("{skip} {width}\n"));
            let mut original = BitReader::new();
            original.set_input(&input);
            original.drop_bytes(skip);
            let mut value = 0xdeadbeef;
            let first = original.safe_read_bits(width, &mut value);
            let fields = original.source_state();
            expected.push_str(&format!(
                "{} {} {} {} {} {} {}",
                u8::from(first),
                value,
                fields.val,
                fields.bit_pos,
                fields.next,
                fields.guard,
                fields.last
            ));
            let mut resumed = BitReader::from_source_state(&input, fields);
            assert_eq!(resumed.source_state(), fields);
            value = 0xdeadbeef;
            let second = resumed.safe_read_bits32_slow(32, &mut value);
            let fields = resumed.source_state();
            expected.push_str(&format!(
                " {} {} {} {} {} {} {}\n",
                u8::from(second),
                value,
                fields.val,
                fields.bit_pos,
                fields.next,
                fields.guard,
                fields.last
            ));
        }
    }
    let mut child = Command::new(std::env::var_os("BROTLI_BIT_READER_STORAGE_ORACLE").unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        stdin.write_all(rows.as_bytes()).unwrap();
    });
    let actual = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(actual.status.success(), "{:?}", actual.stderr);
    assert!(actual.stderr.is_empty());
    assert_eq!(actual.stdout, expected.as_bytes());
}
