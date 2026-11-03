// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[path = "../tools/brotli_ringbuffer_probe.rs"]
mod observer;

#[cfg(feature = "cpp-oracle")]
#[test]
fn original_ring_allocation_output_errors_and_owner_identities() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    // Actual pristine helper observations. The full source-derived corpus is
    // reproducible with tools/brotli_ringbuffer_reference.py; these retained
    // focused rows also run through Cargo in every requested configuration.
    let requests = [
        "0 165 0 16 1 0 2048 17 100 0 0 32 0 0 0 0 0 0",
        "0 165 0 16 1 0 2048 17 100 0 0 32 1 0 0 0 0 0",
        "1 165 0 10 1 8 64 3 100 0 0 32 0 0 0 0 0 0",
        "1 165 4 10 1 8 64 3 100 0 0 32 0 0 0 0 0 0",
        "1 165 3 10 1 0 64 0 100 0 0 32 0 0 0 0 0 0",
        "2 165 0 10 1 64 64 70 100 7 451 32 0 0 0 0 1 0",
        "3 165 0 10 1 1024 2048 1030 0 0 1024 32 1 1 0 0 0 0",
        "3 165 0 10 1 64 64 17 -1 0 0 32 1 2 0 0 0 0",
        "4 165 0 10 1 64 64 17 0 0 0 32 0 0 0 0 1 0",
        "5 165 0 10 1 1024 1024 17 0 0 3 0 0 0 0 0 1 0",
        "6 165 0 10 1 64 64 17 0 0 0 32 0 0 26 -31 0 0",
        "7 165 0 10 1 0 64 0 0 0 0 32 0 0 0 -9 1 18446744073709551615",
    ];
    let payload = requests.join("\n") + "\n";
    let expected = requests
        .iter()
        .map(|r| observer::observe(r))
        .collect::<String>();
    let mut child = Command::new(
        std::env::var_os("BROTLI_RINGBUFFER_ORACLE").expect("set actual pinned source observer"),
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
        .write_all(payload.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{:?}", output.stderr);
    assert!(output.stderr.is_empty(), "{:?}", output.stderr);
    assert_eq!(output.stdout, expected.as_bytes());
}
