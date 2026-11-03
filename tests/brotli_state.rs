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
    assert_eq!(output.stdout, expected.as_bytes());
    eprintln!("Brotli source state checkpoints: {}", requests.len());
}
