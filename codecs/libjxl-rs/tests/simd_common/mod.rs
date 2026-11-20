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
    program_lines(std::path::Path::new(&path), requests, "SIMD oracle")
}

/// The response lines of one oracle executable for `requests`.
fn program_lines(path: &std::path::Path, requests: &[String], what: &str) -> Vec<String> {
    if requests.is_empty() {
        return Vec::new();
    }
    let mut child = Command::new(path)
        .env("ROSETTA_ADVERTISE_AVX", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("start the {what} {}: {e}", path.display()));
    let mut input = child.stdin.take().unwrap();
    let mut lines = requests.join("\n");
    lines.push('\n');
    let feed = std::thread::spawn(move || input.write_all(lines.as_bytes()));
    let output = child.wait_with_output().expect("wait for the oracle");
    let fed = feed.join().unwrap();
    // The exit status first: an oracle that dies early also breaks the
    // request pipe, and the write error alone would hide the cause (#252).
    assert!(
        output.status.success(),
        "{what} failed ({}) after {} response lines; write: {:?}; stderr: {}",
        output.status,
        output.stdout.iter().filter(|&&b| b == b'\n').count(),
        fed.as_ref().err(),
        String::from_utf8_lossy(&output.stderr)
    );
    fed.expect("write the oracle requests");
    let text = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = text.lines().map(str::to_owned).collect();
    assert_eq!(lines.len(), requests.len(), "{what} response count");
    lines
}

/// Two words equal under D8: identical, or both binary32 NaN words.
fn nan_equal(a: &str, b: &str) -> bool {
    a == b
        || matches!((a.parse::<u32>(), b.parse::<u32>()), (Ok(x), Ok(y)) if is_nan(x) && is_nan(y))
}

/// Working rule O1 (owner, 2026-11-15; ADR-0041 section 3; port ADR 0002,
/// rule (2)) for the SIMD gates: for a request whose input contains a NaN or
/// an infinity, a word is compared only where the original's two scalar
/// reference builds for this target agree after treating all NaNs as equal
/// (D8). Elsewhere the requirement is only that the port does not crash and
/// commits no memory error, which running the request already checks.
///
/// The two scalar references are the ones of the scalar gate:
/// `JXL_CPP_ORACLE` (the pinned build) and `JXL_CPP_ORACLE_O3` (the same
/// sources at upstream's normal optimization level), which
/// `tools/check.py` builds for the target under test and passes to the SIMD
/// binaries. `exceptional` says whether a request's input contains a NaN or
/// an infinity; it is computed from the input, never from a result, and a
/// request it does not select never excludes a word.
///
/// Returns one entry per request: `None` (compare every word) for a request
/// that is not exceptional, or whose references differ in their word count
/// (fail closed); otherwise one flag per word of the references' line, set
/// where the references disagree. `JXL_PARITY_STRICT=1` returns `None` for
/// every request, as in the scalar gate.
pub fn reference_masks(
    requests: &[String],
    exceptional: impl Fn(&str) -> bool,
) -> Vec<Option<Vec<bool>>> {
    if std::env::var_os("JXL_PARITY_STRICT").is_some_and(|value| value == "1") {
        return vec![None; requests.len()];
    }
    let chosen: Vec<usize> = (0..requests.len())
        .filter(|&i| exceptional(&requests[i]))
        .collect();
    let subset: Vec<String> = chosen.iter().map(|&i| requests[i].clone()).collect();
    let reference = |name: &str| {
        std::path::PathBuf::from(std::env::var_os(name).unwrap_or_else(|| {
            panic!(
                "set {name} to the original's scalar reference for this target \
                 (python3 tools/check.py builds both) for the NaN rule O1"
            )
        }))
    };
    let (pinned, other) = (reference("JXL_CPP_ORACLE"), reference("JXL_CPP_ORACLE_O3"));
    let (pinned, other) = std::thread::scope(|scope| {
        let other = scope.spawn(|| program_lines(&other, &subset, "second scalar reference"));
        let pinned = program_lines(&pinned, &subset, "pinned scalar reference");
        (pinned, other.join().expect("the second scalar reference"))
    });
    let mut masks = vec![None; requests.len()];
    for ((&i, a), b) in chosen.iter().zip(&pinned).zip(&other) {
        let a: Vec<&str> = a.split_whitespace().collect();
        let b: Vec<&str> = b.split_whitespace().collect();
        if a.len() == b.len() {
            masks[i] = Some(a.iter().zip(&b).map(|(x, y)| !nan_equal(x, y)).collect());
        }
    }
    masks
}

/// [`same`] under rule O1: a word that `mask` flags is not compared. A
/// mask whose length is not the observation's word count excludes nothing.
/// Returns the compared words and the excluded words in which the port and
/// the original differ (after D8).
pub fn same_under_o1(
    rust: &str,
    cpp: &str,
    float: impl Fn(usize) -> bool,
    defined: impl Fn(usize) -> bool,
    mask: Option<&[bool]>,
) -> Result<(usize, usize), String> {
    let words = cpp.split_whitespace().count();
    let mask = mask.filter(|m| m.len() == words);
    let excluded = |i: usize| mask.is_some_and(|m| m[i]);
    let compared = same(rust, cpp, &float, |i| defined(i) && !excluded(i))?;
    let differing = rust
        .split_whitespace()
        .zip(cpp.split_whitespace())
        .enumerate()
        .filter(|&(i, (a, b))| {
            excluded(i) && defined(i) && !(a == b || (float(i) && nan_equal(a, b)))
        })
        .count();
    Ok((compared, differing))
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
