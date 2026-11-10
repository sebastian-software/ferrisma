// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
use bit_reader::BitReader;

// Original native ARM ordinary/ASan captures from the checked-in observer
// tools/oracle/brotli_bit_reader.c, built by its adjacent CMake project against
// the pristine pinned Brotli sources. Feed these exact lines to that observer;
// each response is: success value val bit_pos input_offset guard_offset
// available_in remaining_bytes check_input_amount. These are 64-bit captures.
//
// 4 0 0 17 123456
// -> 1 13330 43 7 3 0 0 0 0
// 5 0 0 32 123456
// -> 0 3735928559 0 0 0 0 3 3 0
// 21 0 0 32 aabb
// -> 0 2 21 5 1 1 1 1 0
// 7 0 1 8 000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f
// -> 1 3735928559 1012195045828461056 57 7 5 25 32 0
// 13 255 7 0 -
// -> 0 3735928559 0 0 0 0 0 0 0
//
// Every expected field was observed from both original graphs, including the
// exact rollback and guard recomputation. Other target branches currently
// have source review, not native acceptance.
#[test]
fn original_safe_read_retains_seven_buffered_bits() {
    let mut br = BitReader::new();
    br.set_input(&[0x12, 0x34, 0x56]);
    let mut value = 0xdeadbeef;
    assert!(br.safe_read_bits32(17, &mut value));
    assert_eq!(
        (
            value,
            br.val,
            br.bit_pos,
            br.position(),
            br.guard_position(),
            br.available_in(),
            br.remaining_bytes()
        ),
        (13330, 43, 7, 3, 0, 0, 0)
    );
}

#[test]
fn original_slow_failure_restores_accumulator_and_input() {
    let mut br = BitReader::new();
    br.set_input(&[0x12, 0x34, 0x56]);
    let mut value = 0xdeadbeef;
    assert!(!br.safe_read_bits32_slow(32, &mut value));
    assert_eq!(
        (
            value,
            br.val,
            br.bit_pos,
            br.position(),
            br.guard_position(),
            br.available_in()
        ),
        (0xdeadbeef, 0, 0, 0, 0, 3)
    );
}

#[test]
fn original_checkpoint_recomputes_short_buffer_guard() {
    let mut br = BitReader::new();
    br.set_input(&[0xaa, 0xbb]);
    let mut value = 0xdeadbeef;
    assert!(br.safe_read_bits(3, &mut value));
    let saved = br.save();
    assert!(!br.safe_read_bits32(32, &mut value));
    br.restore(saved);
    assert_eq!(
        (
            value,
            br.val,
            br.bit_pos,
            br.position(),
            br.guard_position(),
            br.available_in()
        ),
        (2, 21, 5, 1, 1, 1)
    );
}

#[test]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn original_literal_fill_retains_uncredited_spectre_bits() {
    let data: Vec<u8> = (0..32).collect();
    let mut br = BitReader::new();
    br.set_input(&data);
    br.bit_pos = 1;
    br.fill_bit_window_const::<8>();
    assert_eq!(
        (
            br.val,
            br.bit_pos,
            br.position(),
            br.guard_position(),
            br.available_in(),
            br.remaining_bytes()
        ),
        (1012195045828461056, 57, 7, 5, 25, 32)
    );
}

#[test]
fn original_nonzero_padding_is_consumed_before_failure() {
    let mut br = BitReader::new();
    br.set_input(&[]);
    br.val = 255;
    br.bit_pos = 7;
    assert!(!br.jump_to_byte_boundary());
    assert_eq!((br.val, br.bit_pos, br.position()), (0, 0, 0));
}

// Actual ordinary and ASan/UBSan captures, from each exact request:
// `3 0 0 N 12345678`, N=0,25,26,27,28,29,30,31,32. The selected original
// NDEBUG graph omits its <=24 DCHECK; this preserves that source-defined
// native64 domain without changing the while/pull/take operation sequence.
#[test]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
fn original_compiled_safe_read_domain_retains_every_capture_field() {
    let captured = [
        "1 0 0 0 0 0 4 4 0",
        "1 5649426 60 7 4 0 0 0 0",
        "1 5649426 30 6 4 0 0 0 0",
        "1 5649426 15 5 4 0 0 0 0",
        "1 139867154 7 4 4 0 0 0 0",
        "1 408302610 3 3 4 0 0 0 0",
        "1 945173522 1 2 4 0 0 0 0",
        "1 2018915346 0 1 4 0 0 0 0",
        "1 2018915346 0 0 4 0 0 0 0",
    ];
    for (width, expected) in [0, 25, 26, 27, 28, 29, 30, 31, 32]
        .into_iter()
        .zip(captured)
    {
        let mut br = BitReader::new();
        br.set_input(&[0x12, 0x34, 0x56, 0x78]);
        let mut value = 0xdeadbeef;
        let success = br.safe_read_bits(width, &mut value);
        let actual = [
            u64::from(success),
            value,
            br.val,
            br.bit_pos,
            br.position() as u64,
            br.guard_position() as u64,
            br.available_in() as u64,
            br.remaining_bytes() as u64,
            u64::from(br.check_input_amount()),
        ];
        let expected: Vec<u64> = expected
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(actual.as_slice(), expected.as_slice(), "width={width}");
    }
}
