// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
#[path = "../common/skcms_fitting.rs"]
mod adapter;
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};

const COMMANDS: [&str; 6] = [
    "skcms-fit-approx",
    "skcms-fit-roundtrip",
    "skcms-fit-gradient",
    "skcms-fit-step",
    "skcms-fit-nonlinear",
    "skcms-fit-checked",
];

fn requests(selected: u8) -> Vec<String> {
    let mut data: &[u8] = include_bytes!("../data/skcms-fitting-requests.bin");
    let mut requests = Vec::new();
    let mut total = 0;
    while !data.is_empty() {
        let tag = data[0];
        let count = u32::from_le_bytes(data[1..5].try_into().unwrap()) as usize;
        data = &data[5..];
        assert!(usize::from(tag) < COMMANDS.len() && count >= 17);
        let mut line = COMMANDS[usize::from(tag)].to_owned();
        for (index, word) in data[..count * 4].chunks_exact(4).enumerate() {
            let word = u32::from_le_bytes(word.try_into().unwrap());
            if [4, 7, 8].contains(&index) {
                write!(line, " {}", word as i32).unwrap();
            } else {
                write!(line, " {word}").unwrap();
            }
        }
        data = &data[count * 4..];
        total += 1;
        if tag == selected {
            requests.push(line);
        }
    }
    assert_eq!(total, 11_432);
    requests
}

fn exact(selected: u8) {
    let requests = requests(selected);
    assert_eq!(
        requests.len(),
        [3143, 1089, 2400, 3600, 600, 600][usize::from(selected)]
    );
    let env = if selected <= 1 {
        "JXL_CPP_ORACLE"
    } else {
        "JXL_SKCMS_FITTING_ORACLE"
    };
    let mut child =
        Command::new(std::env::var_os(env).expect("set the actual original fitting observer"))
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
    assert!(
        output.status.success(),
        "original fitting status: {:?}; {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let original = String::from_utf8(output.stdout).unwrap();
    let records: Vec<_> = original.lines().collect();
    assert_eq!(requests.len(), records.len());
    for (index, (request, expected)) in requests.iter().zip(records).enumerate() {
        assert_eq!(
            adapter::observe(request),
            expected,
            "request {index}: {request}"
        );
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}

#[test]
fn original_approximation_guards_tables_and_output_aliases() {
    exact(0);
}
#[test]
fn original_sampled_roundtrip_errors_and_inverse_predicate() {
    exact(1);
}
#[test]
fn original_nonlinear_residuals_and_gradients() {
    exact(2);
}
#[test]
fn original_gauss_newton_step_and_partial_tf_writes() {
    exact(3);
}
#[test]
fn original_eight_iteration_nonlinear_fit_and_restoration() {
    exact(4);
}
#[test]
fn original_checked_inverse_roundtrip_error() {
    exact(5);
}
