// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[path = "../tools/brotli_instance_probe.rs"]
mod observer;

#[test]
fn callback_setter_targets_the_live_source_fields_after_the_owner_moves() {
    observer::callback_handle_keeps_the_actual_allocation_after_owner_move();
}

// Actual original captures from both untouched source graphs. Reproduce with
// tools/oracle/brotli_instance; each string is the exact request/response pair.
#[test]
fn original_mixed_pairs_make_no_allocator_calls() {
    for pair in [1, 2] {
        assert_eq!(
            observer::observe(&format!("bi-create {pair} 0 165 3 0")),
            " R0\n"
        );
    }
}

#[test]
fn original_constructor_failure_frees_only_the_reached_state_owner() {
    assert_eq!(
        observer::observe("bi-create 3 1 165 3 0"),
        " A1:5744:0 R0\n"
    );
    assert_eq!(
        observer::observe("bi-create 3 2 165 3 0"),
        " A1:5744:1 A2:1408:0 F1 R0\n"
    );
}

#[test]
fn original_defaults_and_settings_keep_callable_metadata_fields() {
    assert_eq!(
        observer::observe("bi-create 0 0 165 3 0"),
        " R1 O0:0:1 M1:1:1\n"
    );
    assert_eq!(
        observer::observe("bi-create 3 0 165 3 3 0 1 1 4294967295 2 0"),
        " A1:5744:1 A2:1408:1 R1 P1:0:0 P1:0:1 P0:0:1 O0:0:1 M1:1:1 F0 F0 F0 F0 F0 F0 F0 F0 F0 F0 F2 F0 F0 F1\n"
    );
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn actual_original_lifecycle_configuration_and_metadata_field_corpus() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let mut requests = Vec::new();
    for seed in 0..256 {
        requests.push(format!("bi-create 3 0 {seed} 3 4 0 0 1 0 0 1 1 1"));
    }
    for pair in 0..4 {
        for fail in 0..4 {
            for seed in [0, 165, 255] {
                for metadata in 0..4 {
                    requests.push(format!("bi-create {pair} {fail} {seed} {metadata} 0"));
                }
            }
        }
    }
    for pair in [0, 3] {
        for parameter in [0, 1, 2, 3, 0x7fffffff_u32, 0xffffffff] {
            for value in [0, 1, 2, 255, 0x7fffffff_u32, 0xffffffff] {
                requests.push(format!(
                    "bi-create {pair} 0 165 3 5 0 0 1 0 {parameter} {value} 0 1 1 1"
                ));
            }
        }
    }
    assert_eq!(requests.len(), 520);
    let mut child = Command::new(
        std::env::var_os("BROTLI_INSTANCE_ORACLE").expect("set actual source observer"),
    )
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .unwrap();
    let payload = requests.join("\n") + "\n";
    let mut input = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || input.write_all(payload.as_bytes()).unwrap());
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(output.status.success(), "{:?}", output.status);
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let original = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = original.lines().collect();
    assert_eq!(lines.len(), requests.len());
    for (index, (request, original)) in requests.iter().zip(lines).enumerate() {
        assert_eq!(
            observer::observe(request),
            format!("{original}\n"),
            "request {index}: {request}"
        );
    }
}
