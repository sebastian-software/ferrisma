// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#![expect(
    clippy::duplicate_mod,
    reason = "Independent module instances reproduce the actual CTFE dictionary-backing identity defect"
)]
#[path = "../src/brotli/default_memory.rs"]
pub mod default_memory;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../tools/brotli_state_probe.rs"]
mod observer;

#[path = "../src/brotli/dictionary.rs"]
pub mod dictionary;
#[path = "../src/brotli/static_descriptors.rs"]
pub mod static_descriptors;
#[path = "../src/brotli/tables.rs"]
pub mod tables;

// Regression for a CTFE promotion defect: a static REFERENCE to include_bytes
// could give the descriptor a different anonymous backing allocation in a
// full test binary. The original C names one actual static byte array. The
// generated named Rust array must be the backing of every borrowed view.
#[test]
fn named_dictionary_and_descriptors_retain_the_actual_static_backing() {
    let borrowed = dictionary::brotli_get_dictionary();
    assert_eq!(borrowed.data.as_ptr(), tables::DICTIONARY_DATA.as_ptr());
    assert_eq!(
        static_descriptors::DICTIONARY.data,
        tables::DICTIONARY_DATA.as_ptr()
    );
    assert_eq!(
        static_descriptors::TRANSFORMS.prefix_suffix,
        tables::PREFIX_SUFFIX.as_ptr()
    );
    assert_eq!(
        static_descriptors::TRANSFORMS.prefix_suffix_map,
        tables::PREFIX_SUFFIX_MAP.as_ptr()
    );
    assert_eq!(
        static_descriptors::TRANSFORMS.transforms,
        tables::TRANSFORMS_DATA.as_ptr()
    );
    let metadata = observer::observe("bs-private-layout");
    let values: Vec<usize> = metadata
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(values.len(), 4);
    assert!(values[0] != 0 && values[1] >= values[0] && values[1].is_multiple_of(values[0]));
    assert_eq!(values[2] % values[1], 0);
    assert_eq!(values[3], 0);
}

#[test]
fn actual_callable_defaults_retain_initialized_payloads_and_native_alignment() {
    // Same direct/stored-field calls as the actual-source bs-default protocol.
    for stored in [0, 1] {
        for size in [0, 1, 2, 7, 8, 15, 16, 31, 64, 511, 4096] {
            for fill in [0, 165, 255] {
                let request = format!("bs-default {size} {fill} {stored}");
                let mut expected = if stored != 0 {
                    "1:1:1:1:1:1 ".to_owned()
                } else {
                    String::new()
                };
                expected.push_str("1:0:");
                if size == 0 {
                    expected.push('-');
                } else {
                    expected.push_str(&format!("{fill:02x}").repeat(size));
                }
                expected.push('\n');
                assert_eq!(observer::observe(&request), expected);
            }
        }
    }
}

#[test]
fn private_default_bookkeeping_overflow_returns_null_without_unwinding() {
    let (allocate, free) = default_memory::callbacks();
    for size in [
        usize::MAX,
        usize::MAX - 1,
        usize::MAX - 15,
        isize::MAX as usize,
        isize::MAX as usize + 1,
    ] {
        // SAFETY: The matched static default pair ignores opaque. These
        // checked layout/size failures return null; no payload is inspected.
        let pointer = unsafe { allocate(std::ptr::null_mut(), size) };
        assert!(pointer.is_null());
        // SAFETY: The same original default free accepts null.
        unsafe { free(std::ptr::null_mut(), pointer) };
    }
}

#[test]
fn state_dictionary_failure_does_not_invent_cleanup_null_frees() {
    // Exact checkpoint ordering from actual source constructor failure traces.
    let output = observer::observe("bs-state 2 165 0 0 256 256 1");
    assert!(output.starts_with(" A1:5744:1 A2:1408 B1:"));
    assert!(output.contains(":0 F1 B1:"));
    assert!(output.ends_with(" C0\n"));
    assert!(!output.contains(" F0"));
}

#[test]
fn huffman_failure_still_initializes_source_group_headers_then_cleans_up() {
    let output = observer::observe("bs-state 3 165 2 2 704 704 2");
    assert!(output.contains(" A3:8656 B1:"));
    assert!(output.contains(" H0 B1:"));
    assert_eq!(output.matches(" F0").count(), 12);
    assert_eq!(output.matches(" F2 ").count(), 1);
    assert_eq!(output.matches(" F1 ").count(), 1);
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn initialized_state_bytes_and_callbacks_match_actual_pinned_source() {
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let requests = [
        "bs-state 0 165 0 0 256 256 1",
        "bs-state 2 165 0 0 256 256 1",
        "bs-state 0 0 1 0 256 256 1",
        "bs-state 0 255 2 1 704 704 2",
        "bs-state 3 165 2 2 704 704 2",
        "bs-default 0 165 0",
        "bs-default 64 165 1",
        "bs-layout",
    ];
    let mut child = Command::new(
        std::env::var_os("BROTLI_STATE_ORACLE").expect("build the actual source state observer"),
    )
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();
    let input = requests.join("\n") + "\n";
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()).unwrap());
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(output.status.success(), "{:?}", output.stderr);
    assert!(output.stderr.is_empty(), "{:?}", output.stderr);
    let expected: String = requests
        .iter()
        .map(|line| observer::observe(line))
        .collect();
    let original = String::from_utf8(output.stdout).unwrap();
    // Owner decision of 2026-11-16 (port ADR 0003, "Brotli flag word and
    // transform tail bytes"): the port zeroes the packed flag word when it
    // creates the state; the original leaves the bits it has not written as
    // the allocator returned them. Left out of this comparison are exactly
    // those bits: the 12 padding bits always, and the fields StateInit does
    // not assign (size_nibbles) while no request has read a metablock header.
    // Every bit StateInit assigns, every other byte of every block and every
    // callback stay compared exactly.
    let unwritten = unwritten_flag_bits(&requests);
    let original = mask_flag_word(&original, unwritten.mask, 0);
    let port = mask_flag_word(&expected, unwritten.mask, unwritten.port_zero);
    assert_eq!(original, port);
    eprintln!("Brotli source state checkpoints: {}", requests.len());
}

/// A file of the original's Brotli source, read when the oracle test runs
/// (the upstream submodule is there only where the oracle is built; a
/// compile-time include would make every --all-features build need it).
#[cfg(feature = "cpp-oracle")]
fn original_source(path: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("upstream/libjxl/third_party/brotli")
        .join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("the original's {}: {error}", path.display()))
}

/// The packed flag word of c/dec/state.h (`BrotliDecoderStateStruct`): the
/// consecutive `unsigned int name : width;` members from `is_last_metablock`
/// to `size_nibbles`, as `(name, shift, width)`. Clang and GCC allocate them
/// from the least significant bit of one `unsigned int` on AArch64 and
/// x86-64, as the port's `BitfieldWord` shifts do.
#[cfg(feature = "cpp-oracle")]
fn original_flag_fields() -> Vec<(String, u32, u32)> {
    let header = original_source("c/dec/state.h");
    let start = header
        .find("unsigned int is_last_metablock : 1;")
        .expect("the original's flag bitfield");
    let mut fields = Vec::new();
    let mut shift = 0;
    for line in header[start..].lines() {
        let Some(member) = line.trim().strip_prefix("unsigned int ") else {
            break;
        };
        let (name, width) = member
            .strip_suffix(';')
            .and_then(|m| m.split_once(" : "))
            .expect("a bitfield member");
        let width: u32 = width.parse().unwrap();
        fields.push((name.to_owned(), shift, width));
        shift += width;
    }
    assert!(shift > 0 && shift < 32);
    fields
}

/// The flag fields `BrotliDecoderStateInit` (c/dec/state.c) assigns:
/// every `s->name = ` statement in its body for a field of the word.
#[cfg(feature = "cpp-oracle")]
fn fields_assigned_by_state_init(fields: &[(String, u32, u32)]) -> Vec<String> {
    let source = original_source("c/dec/state.c");
    let start = source
        .find("BROTLI_BOOL BrotliDecoderStateInit(")
        .expect("the original's StateInit");
    let end = start + source[start..].find("\n}").expect("the end of StateInit");
    let body = &source[start..end];
    fields
        .iter()
        .filter(|(name, _, _)| body.contains(&format!("s->{name} = ")))
        .map(|(name, _, _)| name.clone())
        .collect()
}

#[cfg(feature = "cpp-oracle")]
struct UnwrittenFlagBits {
    /// Bits left out of the comparison, on both sides.
    mask: u32,
    /// Of those, the bits the port must hold as zero (its zeroed word).
    port_zero: u32,
}

/// The bits of the flag word the original has not written at this test's
/// checkpoints: the padding (bits the declaration does not use), and the
/// fields StateInit does not assign, as long as no request reaches the code
/// that writes them. In this test (and in tools/brotli_state_reference.py)
/// every request is StateInit followed by at most one of BrotliDecoderState
/// MetablockBegin, HuffmanTreeGroupInit or StateCleanupAfterMetablock (bs-state
/// actions 0..3), a default-allocator call (bs-default) or a layout dump
/// (bs-layout). None reads a metablock header, the only place the original
/// writes size_nibbles (DecodeMetaBlockLength, c/dec/decode.c); the request
/// check below fails for any other request.
#[cfg(feature = "cpp-oracle")]
fn unwritten_flag_bits(requests: &[&str]) -> UnwrittenFlagBits {
    for request in requests {
        let words: Vec<_> = request.split_whitespace().collect();
        match words[0] {
            "bs-state" => assert!(
                matches!(words[3], "0" | "1" | "2" | "3"),
                "a request may write a flag field StateInit leaves unwritten: {request}"
            ),
            "bs-default" | "bs-layout" => {}
            _ => panic!("unclassified request for the flag-word exclusion: {request}"),
        }
    }
    let fields = original_flag_fields();
    let assigned = fields_assigned_by_state_init(&fields);
    let unassigned: Vec<_> = fields
        .iter()
        .filter(|(name, _, _)| !assigned.contains(name))
        .collect();
    // Pinned upstream: StateInit assigns every field but size_nibbles
    // (state.c lines 49 and 82-88). A change of the pin fails here first.
    assert_eq!(
        unassigned.iter().map(|f| f.0.as_str()).collect::<Vec<_>>(),
        ["size_nibbles"]
    );
    let used: u32 = fields.iter().map(|f| f.2).sum();
    let padding = u32::MAX << used;
    let unwritten_fields = unassigned.iter().fold(0, |mask, (_, shift, width)| {
        mask | ((u32::MAX >> (32 - width)) << shift)
    });
    UnwrittenFlagBits {
        mask: padding | unwritten_fields,
        port_zero: padding | unwritten_fields,
    }
}

/// Sets the excluded bits of the flag word to zero in every state-block dump
/// (` B<n>:<hex>` of exactly the state's size) of an observation text, after
/// checking that the bits in `zero` are zero there.
#[cfg(feature = "cpp-oracle")]
fn mask_flag_word(text: &str, mask: u32, zero: u32) -> String {
    let (size, offset) = observer::state_flag_word();
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(" B") {
        let (head, tail) = rest.split_at(at + 2);
        out.push_str(head);
        let colon = tail.find(':').unwrap_or(0);
        let hex_start = colon + 1;
        let hex_len = tail[hex_start..]
            .find(|c: char| !c.is_ascii_hexdigit())
            .unwrap_or(tail.len() - hex_start);
        if colon > 0 && tail[..colon].bytes().all(|b| b.is_ascii_digit()) && hex_len == 2 * size {
            let hex = &tail[hex_start..hex_start + hex_len];
            let word = &hex[2 * offset..2 * offset + 8];
            let bytes: [u8; 4] =
                std::array::from_fn(|i| u8::from_str_radix(&word[2 * i..2 * i + 2], 16).unwrap());
            let value = u32::from_ne_bytes(bytes);
            assert_eq!(
                value & zero,
                0,
                "the port's unwritten flag bits must be zero: {word}"
            );
            out.push_str(&tail[..hex_start]);
            out.push_str(&hex[..2 * offset]);
            for byte in (value & !mask).to_ne_bytes() {
                out.push_str(&format!("{byte:02x}"));
            }
            out.push_str(&hex[2 * offset + 8..]);
            rest = &tail[hex_start + hex_len..];
        } else {
            rest = tail;
        }
    }
    out.push_str(rest);
    out
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn flag_word_exclusion_covers_only_unwritten_bits() {
    let (size, offset) = observer::state_flag_word();
    let unwritten = unwritten_flag_bits(&["bs-state 0 165 0 0 256 256 1"]);
    assert_eq!(unwritten.mask, 0xffff_f000, "padding and size_nibbles");
    let dump = |word: [u8; 4], other: &str| {
        let mut block = "a5".repeat(size);
        let word: String = word.iter().map(|b| format!("{b:02x}")).collect();
        block.replace_range(2 * offset..2 * offset + 8, &word);
        block.replace_range(2 * offset + 8..2 * offset + 10, other);
        format!(" A1:{size} B1:{block} B2:a5a5")
    };
    let same =
        |a: &str, b: &str, mask: u32| mask_flag_word(a, mask, 0) == mask_flag_word(b, mask, 0);
    let original = dump(0xa5a5_a010_u32.to_ne_bytes(), "a5");
    let port = dump(0x0000_0010_u32.to_ne_bytes(), "a5");
    // The real case: padding and the unwritten size_nibbles differ.
    assert!(same(&original, &port, unwritten.mask));
    assert_eq!(
        mask_flag_word(&port, unwritten.mask, unwritten.port_zero),
        mask_flag_word(&port, unwritten.mask, 0)
    );
    // A defined bit of the word (window_bits) or another byte still differs.
    assert!(!same(
        &dump(0x0000_0050_u32.to_ne_bytes(), "a5"),
        &port,
        unwritten.mask
    ));
    assert!(!same(
        &dump(0x0000_0010_u32.to_ne_bytes(), "00"),
        &port,
        unwritten.mask
    ));
    // Once size_nibbles is written, only the padding is left out, and a
    // difference in size_nibbles fails.
    let padding_only = unwritten.mask & 0xfff0_0000;
    assert!(!same(
        &dump(0x0000_5010_u32.to_ne_bytes(), "a5"),
        &dump(0x0000_6010_u32.to_ne_bytes(), "a5"),
        padding_only
    ));
    assert!(same(
        &dump(0xa5a0_5010_u32.to_ne_bytes(), "a5"),
        &dump(0x0000_5010_u32.to_ne_bytes(), "a5"),
        padding_only
    ));
    // A port word with a set unwritten bit is rejected.
    assert!(
        std::panic::catch_unwind(|| mask_flag_word(&original, unwritten.mask, unwritten.port_zero))
            .is_err()
    );
}

#[cfg(feature = "cpp-oracle")]
#[test]
#[should_panic(expected = "may write a flag field")]
fn a_request_that_reads_a_header_ends_the_exclusion() {
    unwritten_flag_bits(&["bs-state 0 165 4 0 256 256 1"]);
}
