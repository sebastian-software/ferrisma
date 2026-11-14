// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Whole decodes through the public decoder with a thread runner of 0, 1, 2
//! and 8 workers, with the SIMD target forced, against the same decode
//! without a runner and against the original's public decoder built for the
//! same single Highway target (port ADR 0003, "Concurrent decoder tasks";
//! ADR 0004, "Scope of the selection").
//!
//! The decoder's AC group tasks decode concurrently on the runner's threads,
//! so every output word must be independent of the worker count. The inputs
//! are multi-group lossy (VarDCT, with Gaborish and every EPF iteration
//! count) and lossless (modular) files that the oracle encodes, and, when
//! `JXL_CONFORMANCE_CORPUS` names the conformance corpus
//! (docs/CONFORMANCE.md), its 40 inputs. Output samples are compared word for
//! word, any NaN equal to any NaN (decision D8). The original runs without a
//! runner (`dps-decode`); its output does not depend on the runner either
//! (the record of 2026-11-14, column (c)).
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::decode::{self as api, JxlDecoderStatus as Status};
use jxl::jxl::hwy::{decoder_target, set_decoder_simd_for_test};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use jxl::threads::thread_parallel_runner::{
    jxl_thread_parallel_runner, jxl_thread_parallel_runner_create,
    jxl_thread_parallel_runner_destroy,
};
use simd_common::{is_nan, target};
use std::path::Path;

/// The worker counts compared; `None` is the decoder without a runner.
const WORKERS: [Option<usize>; 5] = [None, Some(0), Some(1), Some(2), Some(8)];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

/// The scalar oracle, which encodes the lossless inputs as the scalar
/// suite's public-decoder tests do.
fn scalar_oracle_requests(requests: &[String]) -> Vec<String> {
    let path = std::env::var_os("JXL_CPP_ORACLE").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle"),
        std::path::PathBuf::from,
    );
    use std::io::Write as _;
    use std::process::{Command, Stdio};
    let mut child = Command::new(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("start {}: {e}", path.display()));
    let mut input = child.stdin.take().unwrap();
    let mut lines = requests.join("\n");
    lines.push('\n');
    let feed = std::thread::spawn(move || input.write_all(lines.as_bytes()));
    let output = child.wait_with_output().unwrap();
    feed.join().unwrap().unwrap();
    assert!(output.status.success(), "scalar oracle failed");
    let lines: Vec<_> = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(lines.len(), requests.len());
    lines
}

/// Multi-group inputs encoded by the original: VarDCT frames of 3x2 and
/// 4x3 groups through the SIMD oracle's encoder (the `decode_vardct` gate's
/// encoder request), and lossless frames of several groups through the
/// scalar suite's public-decoder encoder.
fn encoded_files() -> Vec<(String, Vec<u8>)> {
    fn image(w: usize, h: usize, seed: u64) -> Vec<u8> {
        let mut seed = seed | 1;
        let mut pixels = Vec::with_capacity(w * h * 3);
        for y in 0..h {
            for x in 0..w {
                for c in 0..3 {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    let base = (x * (3 + c) + y * (5 - c) + (x * y) / (7 + c)) % 256;
                    pixels.push(((base + (seed % 24) as usize) % 256) as u8);
                }
            }
        }
        pixels
    }
    let mut names = Vec::new();
    let mut requests = Vec::new();
    for (w, h) in [(513, 257), (1000, 700)] {
        for (gab, epf) in [(false, 0), (true, 1), (true, 2), (false, 3)] {
            names.push(format!("vardct {w}x{h} gab={} epf={epf}", u8::from(gab)));
            requests.push(format!(
                "vd-encode-effects {w} {h} 0 0 8 7 {} 0 {epf} 0 {}",
                u8::from(gab),
                hex(&image(w, h, (w * 31 + h + epf) as u64))
            ));
        }
    }
    let mut files: Vec<_> = names
        .into_iter()
        .zip(simd_common::oracle(&requests))
        .map(|(name, line)| {
            let fields: Vec<_> = line.split_whitespace().collect();
            assert_eq!(fields[0], "0", "original encoder: {name}");
            (name, unhex(fields[1]))
        })
        .collect();
    let specs = [
        "w=600 h=520 c=3",
        "w=600 h=520 c=3 opt=26:0",
        "w=530 h=300 c=3 a=1 ec=1 effort=3 opt=26:0",
        "w=520 h=290 c=1 bits=16 effort=2",
    ];
    let requests: Vec<_> = specs.iter().map(|s| format!("dpub-encode {s}")).collect();
    for (spec, line) in specs.iter().zip(scalar_oracle_requests(&requests)) {
        files.push((format!("lossless {spec}"), unhex(line.trim())));
    }
    files
}

fn conformance_files(files: &mut Vec<(String, Vec<u8>)>) -> bool {
    let Some(root) = std::env::var_os("JXL_CONFORMANCE_CORPUS") else {
        return false;
    };
    let manifest = include_str!("../tools/conformance/MANIFEST.sha256");
    for line in manifest.lines() {
        let Some((_, name)) = line.split_once("  ") else {
            continue;
        };
        let Some(case) = name
            .strip_prefix("testcases/")
            .and_then(|n| n.strip_suffix("/input.jxl"))
        else {
            continue;
        };
        let path = Path::new(&root).join(name);
        let bytes = std::fs::read(&path)
            .unwrap_or_else(|e| panic!("conformance input {}: {e}", path.display()));
        files.push((format!("conformance/{case}"), bytes));
    }
    true
}

/// The original's outputs of `dps-decode`: every `o:` (image) and `x:`
/// (extra channel) token, in order.
fn original_outputs(line: &str) -> Vec<Vec<u8>> {
    line.split(' ')
        .filter_map(|t| t.strip_prefix("o:").or_else(|| t.strip_prefix("x:")))
        .map(unhex)
        .collect()
}

/// The decode of `dps-decode` (whole input, closed input, no CMS), with an
/// optional thread runner of `workers` workers. Returns the image and extra
/// channel outputs at every full image, in the original's order.
fn port_decode(data_type: u32, file: &[u8], workers: Option<usize>) -> Vec<Vec<u8>> {
    // SAFETY: A null manager selects the default allocator.
    let raw = unsafe { api::jxl_decoder_create(std::ptr::null()) };
    assert!(!raw.is_null());
    // SAFETY: `raw` is live and uniquely owned until destroyed below.
    let dec = unsafe { &mut *raw };
    let runner = workers.map(|n| {
        // SAFETY: A null manager selects the default allocator.
        let runner = unsafe { jxl_thread_parallel_runner_create(std::ptr::null(), n) };
        assert!(!runner.is_null());
        // SAFETY: The translated runner obeys parallel_runner.h and lives
        // until it is destroyed after the decoder.
        let s = unsafe {
            api::jxl_decoder_set_parallel_runner(dec, Some(jxl_thread_parallel_runner), runner)
        };
        assert_eq!(s, Status::SUCCESS);
        runner
    });
    let events =
        Status::BASIC_INFO.0 | Status::COLOR_ENCODING.0 | Status::FRAME.0 | Status::FULL_IMAGE.0;
    assert_eq!(
        api::jxl_decoder_subscribe_events(dec, events as i32),
        Status::SUCCESS
    );
    // SAFETY: `file` outlives the decoder.
    let s = unsafe { api::jxl_decoder_set_input(dec, file.as_ptr(), file.len()) };
    assert_eq!(s, Status::SUCCESS);
    api::jxl_decoder_close_input(dec);
    let mut format = JxlPixelFormat {
        num_channels: 0,
        data_type: JxlDataType(data_type),
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    let ec_format = JxlPixelFormat {
        num_channels: 1,
        ..format
    };
    let mut extras = 0;
    let mut image: Vec<u8> = Vec::new();
    let mut extra: Vec<Vec<u8>> = Vec::new();
    let mut outputs = Vec::new();
    loop {
        // SAFETY: The input and every registered output buffer stay live and
        // unmoved until the decoder is destroyed or they are replaced.
        let status = unsafe { api::jxl_decoder_process_input(dec) };
        if status == Status::BASIC_INFO {
            let mut info = JxlBasicInfo::zeroed();
            assert_eq!(
                api::jxl_decoder_get_basic_info(dec, Some(&mut info)),
                Status::SUCCESS
            );
            format.num_channels = info.num_color_channels + u32::from(info.alpha_bits != 0);
            extras = info.num_extra_channels;
        } else if status == Status::NEED_IMAGE_OUT_BUFFER {
            let mut size = 0usize;
            let s = api::jxl_decoder_image_out_buffer_size(dec, Some(&format), Some(&mut size));
            assert_eq!(s, Status::SUCCESS);
            image = vec![0; size];
            // SAFETY: `image` holds `size` writable bytes and stays unmoved
            // while the decoder may write it.
            let s = unsafe {
                api::jxl_decoder_set_image_out_buffer(
                    dec,
                    Some(&format),
                    image.as_mut_ptr().cast(),
                    size,
                )
            };
            assert_eq!(s, Status::SUCCESS);
            extra = vec![Vec::new(); extras as usize];
            for i in 0..extras {
                let s = api::jxl_decoder_extra_channel_buffer_size(
                    dec,
                    Some(&ec_format),
                    Some(&mut size),
                    i,
                );
                if s == Status::SUCCESS {
                    extra[i as usize] = vec![0; size];
                    // SAFETY: As for `image`.
                    let s = unsafe {
                        api::jxl_decoder_set_extra_channel_buffer(
                            dec,
                            Some(&ec_format),
                            extra[i as usize].as_mut_ptr().cast(),
                            size,
                            i,
                        )
                    };
                    assert_eq!(s, Status::SUCCESS);
                }
            }
        } else if status == Status::FULL_IMAGE {
            outputs.push(image.clone());
            outputs.extend(extra.iter().cloned());
        } else if status == Status::COLOR_ENCODING || status == Status::FRAME {
        } else {
            assert_eq!(status, Status::SUCCESS, "decode status");
            break;
        }
    }
    // SAFETY: The sole owner destroys the decoder once, then its runner.
    unsafe {
        api::jxl_decoder_destroy(raw);
        if let Some(runner) = runner {
            jxl_thread_parallel_runner_destroy(runner);
        }
    }
    outputs
}

/// The number of differing words of two decodes' outputs, any NaN equal to
/// any NaN for float output; None if their layout differs.
fn differing_words(a: &[Vec<u8>], b: &[Vec<u8>], word: usize, float: bool) -> Option<usize> {
    if a.len() != b.len() || a.iter().zip(b).any(|(x, y)| x.len() != y.len()) {
        return None;
    }
    let mut differing = 0;
    for (x, y) in a.iter().zip(b) {
        for (p, q) in x.chunks(word).zip(y.chunks(word)) {
            if p == q {
                continue;
            }
            if float {
                let p = u32::from_ne_bytes(p.try_into().unwrap());
                let q = u32::from_ne_bytes(q.try_into().unwrap());
                if is_nan(p) && is_nan(q) {
                    continue;
                }
            }
            differing += 1;
        }
    }
    Some(differing)
}

#[test]
fn worker_counts_give_the_single_thread_and_the_original_output() {
    let t = target();
    let mut files = encoded_files();
    if !conformance_files(&mut files) {
        println!(
            "JXL_CONFORMANCE_CORPUS is not set: the conformance corpus is not compared (docs/CONFORMANCE.md)"
        );
    }
    let mut failures = Vec::new();
    let mut words = 0usize;
    for (data_type, word, float) in [(0u32, 4usize, true), (2, 1, false)] {
        let requests: Vec<_> = files
            .iter()
            .map(|(_, bytes)| format!("dps-decode {data_type} 0 {}", hex(bytes)))
            .collect();
        let originals = simd_common::oracle(&requests);
        for ((name, bytes), line) in files.iter().zip(&originals) {
            let original = original_outputs(line);
            set_decoder_simd_for_test(Some(true));
            assert_eq!(decoder_target().map(|d| d.name()), Some(t.name()));
            let decodes: Vec<_> = WORKERS
                .iter()
                .map(|&workers| port_decode(data_type, bytes, workers))
                .collect();
            set_decoder_simd_for_test(None);
            words += decodes[0].iter().map(|o| o.len() / word).sum::<usize>();
            let mut cells = Vec::new();
            for (workers, decode) in WORKERS.iter().zip(&decodes) {
                let serial = differing_words(decode, &decodes[0], word, float);
                let against_original = differing_words(decode, &original, word, float);
                cells.push(format!(
                    "{}: {serial:?}/{against_original:?}",
                    workers.map_or("none".into(), |n| n.to_string())
                ));
                if serial != Some(0) || against_original != Some(0) {
                    failures.push(format!(
                        "{name} type {data_type} workers {workers:?}: {serial:?} differing words against no runner, {against_original:?} against the original"
                    ));
                }
            }
            println!(
                "  {name} type {data_type} | {} outputs | workers: differing words against no runner / the original | {}",
                decodes[0].len(),
                cells.join(" | ")
            );
        }
    }
    println!(
        "{} files on {}, {} worker settings, {words} words per setting compared",
        files.len(),
        t.name(),
        WORKERS.len()
    );
    assert!(failures.is_empty(), "{failures:#?}");
}
