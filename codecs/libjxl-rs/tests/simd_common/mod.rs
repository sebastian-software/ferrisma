// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD parity helpers of `tests/simd_parity.rs` (oracle pipe, word
//! comparison with the NaN rule D8, value generators) for the SIMD test
//! binaries added after it. Copies, so that the existing test file and its
//! behavior stay untouched; keep them identical in meaning.
#![allow(dead_code)]

use jxl::jxl::hwy::Target;
use std::io::Write as _;
use std::process::{Command, Stdio};

pub fn target() -> Target {
    Target::detect().expect("the host has no NEON/AVX2 target")
}

pub fn oracle(requests: &[String]) -> Vec<String> {
    let path = std::env::var_os("JXL_CPP_ORACLE_SIMD")
        .expect("set JXL_CPP_ORACLE_SIMD to the NEON/AVX2 oracle (python3 tools/check.py)");
    let mut child = Command::new(path)
        .env("ROSETTA_ADVERTISE_AVX", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start the SIMD oracle");
    let mut input = child.stdin.take().unwrap();
    let mut lines = requests.join("\n");
    lines.push('\n');
    let feed = std::thread::spawn(move || input.write_all(lines.as_bytes()));
    let output = child.wait_with_output().expect("wait for the SIMD oracle");
    let fed = feed.join().unwrap();
    // The exit status first: an oracle that dies early also breaks the
    // request pipe, and the write error alone would hide the cause (#252).
    assert!(
        output.status.success(),
        "SIMD oracle failed ({}) after {} response lines; write: {:?}; stderr: {}",
        output.status,
        output.stdout.iter().filter(|&&b| b == b'\n').count(),
        fed.as_ref().err(),
        String::from_utf8_lossy(&output.stderr)
    );
    fed.expect("write the oracle requests");
    let text = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = text.lines().map(str::to_owned).collect();
    assert_eq!(lines.len(), requests.len(), "oracle response count");
    lines
}

pub fn is_nan(word: u32) -> bool {
    f32::from_bits(word).is_nan()
}

/// Whether a binary16 word is a NaN.
pub fn is_nan16(word: u32) -> bool {
    word & 0x7c00 == 0x7c00 && word & 0x03ff != 0
}

/// Compare whitespace-separated observations word by word. `float` says
/// whether word `i` is a binary32 sample (NaN-equal) and `defined` whether
/// the original computes it from initialized memory.
pub fn same(
    rust: &str,
    cpp: &str,
    float: impl Fn(usize) -> bool,
    defined: impl Fn(usize) -> bool,
) -> Result<usize, String> {
    let r: Vec<&str> = rust.split_whitespace().collect();
    let c: Vec<&str> = cpp.split_whitespace().collect();
    if r.len() != c.len() {
        return Err(format!("word count {} != {}", r.len(), c.len()));
    }
    let mut compared = 0;
    for (i, (a, b)) in r.iter().zip(&c).enumerate() {
        if !defined(i) {
            continue;
        }
        compared += 1;
        if a == b {
            continue;
        }
        if float(i)
            && let (Ok(x), Ok(y)) = (a.parse::<u32>(), b.parse::<u32>())
            && is_nan(x)
            && is_nan(y)
        {
            continue;
        }
        return Err(format!("word {i}: rust {a} != original {b}"));
    }
    Ok(compared)
}

pub fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// Adversarial binary32 words: zeros, signs, subnormals, extremes,
/// infinities, quiet and signaling NaNs of both signs, rounding ties, integer
/// conversion limits, and reciprocal-estimate boundaries (as simd_parity.rs).
pub fn special_words() -> Vec<u32> {
    let mut words = vec![
        0x0000_0000,
        0x8000_0000,
        0x3f80_0000,
        0xbf80_0000,
        0x3f00_0000,
        0xbf00_0000,
        0x3fc0_0000,
        0x4020_0000,
        0xc020_0000,
        0x0080_0000,
        0x8080_0000,
        0x0000_0001,
        0x8000_0001,
        0x007f_ffff,
        0x7f7f_ffff,
        0xff7f_ffff,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0xffc0_0000,
        0x7fa1_2345,
        0xffa5_4321,
        0x7fc9_8765,
        0x4f00_0000,
        0xcf00_0000,
        0x4eff_ffff,
        0xcf00_0001,
        0x4b00_0000,
        0x4aff_fffe,
        0xcaff_ffff,
        0x3eff_ffff,
        0x3eaa_aaab,
        0x7e80_0000,
        0x7f00_0000,
        0x0040_0000,
        0x4000_0000,
        0x4040_0000,
        0x447a_0000,
        0xc47a_0000,
    ];
    let mut seed = 0x1234_5678_9abc_def1;
    for _ in 0..40 {
        words.push(next(&mut seed) as u32);
    }
    words
}

pub fn join(words: impl IntoIterator<Item = u32>) -> String {
    words
        .into_iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn status(s: &jxl::jxl::base::status::Status) -> i32 {
    jxl::jxl::base::status::status_code(s)
}

/// Compare every request's Rust observation with the original's, printing
/// the family's request and word counts.
pub fn check_rows(
    family: &str,
    requests: &[String],
    observe: impl Fn(&str) -> (String, Box<dyn Fn(usize) -> bool>),
    float: impl Fn(usize) -> bool + Copy,
) -> usize {
    let observed = oracle(requests);
    let mut words = 0;
    for (request, cpp) in requests.iter().zip(&observed) {
        let (rust, defined) = observe(request);
        words += same(&rust, cpp, float, defined)
            .unwrap_or_else(|e| panic!("{family}: {e}\nrequest: {request}"));
    }
    println!(
        "{family} on {}: {} requests, {words} words matched",
        target().name(),
        requests.len()
    );
    words
}
