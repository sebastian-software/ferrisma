// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::icc_codec::{ICCReader, ICCReaderSnapshot};
use jxl::jxl::padded_bytes::PaddedBytes;

pub(super) fn handles(kind: &str) -> bool {
    kind == "ir-run" || kind == "ir-cache"
}

fn window_address(snapshot: &ICCReaderSnapshot<'_>) -> usize {
    snapshot
        .lz_window
        .map_or(0, |window| window.as_ptr() as usize)
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let seed = unhex(args.next().unwrap());
    let count: usize = number(args);
    let mut output = PaddedBytes::default();
    output.append(&seed).unwrap();
    let mut reader = ICCReader::new();
    let mut defined_prefix = 0;
    let mut out = String::new();
    let mut previous_status = 0;
    let mut cached_addresses = [0; 3];
    for step in 0..count {
        let op: usize = number(args);
        let prefix = number(args);
        let data = unhex(args.next().unwrap());
        let mut bits = BitReader::new(&data);
        bits.skip_bits(prefix);
        let before = reader.observer_snapshot();
        let previous_window = window_address(&before);
        let previous_storage = before.decompressed_address;
        let old_size = before.decompressed.len();
        let old_capacity = before.decompressed_capacity;
        let mut old_i = before.i;
        let mut initial_bits = bits.total_bits_consumed();
        let mut init = Ok(());
        let mut status = Ok(());
        let reset = op == 5 && previous_status > 0;
        if reset {
            reader.reset();
        }
        if op == 0 || op == 2 || op == 5 {
            init = reader.init(&mut bits);
        }
        if op == 1 || ((op == 2 || op == 5) && init.is_ok()) {
            old_i = reader.observer_snapshot().i;
            initial_bits = bits.total_bits_consumed();
            status = reader.process(&mut bits, &mut output);
        } else if op == 3 {
            reader.reset();
        } else if op == 4 {
            status = ICCReader::observer_check_eoi(&mut bits);
        }
        let active = init.and(status);
        previous_status = status_code(&active);
        let consumed = bits.total_bits_consumed();
        let close = bits.close();
        let snapshot = reader.observer_snapshot();
        if old_capacity != snapshot.decompressed_capacity {
            let copied = if reset { 0 } else { old_size };
            defined_prefix = defined_prefix.min(copied);
            if snapshot.decompressed_address != 0 && defined_prefix == copied {
                defined_prefix += 1;
            }
        }
        let mut written = snapshot.i;
        let operation = if op == 2 || op == 5 {
            if init.is_err() { 0 } else { 2 }
        } else {
            op
        };
        if (operation == 1 || operation == 2)
            && active == Err(StatusCode::NotEnoughBytes)
            && !(old_i > 0 && old_i.is_multiple_of(512) && initial_bits > data.len() * 8)
        {
            written = ((snapshot.i / 512 + 1) * 512).min(snapshot.enc_size as usize);
        }
        if operation != 3 && operation != 4 {
            defined_prefix = defined_prefix.max(written);
        }
        let defined = defined_prefix.min(snapshot.decompressed.len());
        write!(
            out,
            "{}{} {} {} {consumed} {} {} {} {} {} {} {} {defined} {} {} {} {}",
            if step == 0 { "S " } else { " S " },
            op,
            status_code(&init),
            status_code(&status),
            status_code(&close),
            snapshot.i,
            snapshot.bits_to_skip,
            snapshot.used_bits_base,
            snapshot.enc_size,
            snapshot.decompressed.len(),
            snapshot.decompressed_capacity,
            hex(&snapshot.decompressed[..defined]),
            output.len(),
            hex(output.as_slice()),
            snapshot.context_map.len(),
        )
        .unwrap();
        for context in snapshot.context_map {
            write!(out, " {context}").unwrap();
        }
        for size in snapshot.code_sizes {
            write!(out, " {size}").unwrap();
        }
        for value in snapshot.ans_scalars {
            write!(out, " {value}").unwrap();
        }
        write!(
            out,
            " {} {}",
            u8::from(snapshot.lz_window.is_some()),
            snapshot.lz_window.map_or(0, <[u32]>::len),
        )
        .unwrap();
        if let Some(window) = snapshot.lz_window {
            for value in window {
                write!(out, " {value}").unwrap();
            }
        }
        write!(
            out,
            " {} {} {}",
            u8::from(previous_window == window_address(&snapshot)),
            if old_capacity == snapshot.decompressed_capacity {
                u8::from(previous_storage == snapshot.decompressed_address)
            } else {
                2
            },
            u8::from(reset),
        )
        .unwrap();
        if kind == "ir-cache" {
            if step == 0 {
                assert!(snapshot.i > 0, "cache fixture requires Create");
                cached_addresses = snapshot.code_addresses;
            }
            write!(
                out,
                " C {} {} {} {} {}",
                u8::from(snapshot.cached_use_prefix_code),
                u8::from(snapshot.owner_use_prefix_code),
                u8::from(cached_addresses[0] == snapshot.code_addresses[0]),
                u8::from(cached_addresses[1] == snapshot.code_addresses[1]),
                u8::from(cached_addresses[2] == snapshot.code_addresses[2]),
            )
            .unwrap();
        }
    }
    out
}

fn source_stream(command: String) -> (usize, Vec<u8>) {
    let output = original_observations(&[command]);
    let mut fields = output[0].split_whitespace();
    assert_eq!(fields.next(), Some("0"));
    let bits = fields.next().unwrap().parse().unwrap();
    (bits, unhex(fields.next().unwrap()))
}

fn run(seed: &[u8], steps: &[(usize, usize, &[u8])]) -> String {
    let mut out = format!("ir-run {} {}", hex(seed), steps.len());
    for (op, prefix, bytes) in steps {
        write!(out, " {op} {prefix} {}", hex(bytes)).unwrap();
    }
    out
}

fn exact(cases: Vec<String>) {
    // Optional audit artifact retains the actual requests of every group.
    // A process-wide lock prevents interleaving even for the large LZ case.
    if let Some(path) = std::env::var_os("JXL_ICC_READER_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        std::io::Write::write_all(&mut file, (cases.join("\n") + "\n").as_bytes()).unwrap();
    }
    for chunk in cases.chunks(8) {
        let cpp = original_observations(chunk);
        for (request, original) in chunk.iter().zip(cpp) {
            let rust = super::observe(request);
            assert_eq!(rust, original, "request: {request}");
        }
    }
    println!("{} C++/Rust differential cases matched", cases.len());
}

#[test]
fn icc_reader_default_reset_and_actual_eoi_checkpoints() {
    let mut cases = Vec::new();
    for bytes in [vec![], vec![0], vec![0xff], vec![0; 9]] {
        for prefix in [0, 1, 7, 8, 63, 73] {
            cases.push(run(&[0x12, 0x34], &[(4, prefix, &bytes)]));
            cases.push(run(&[0x12, 0x34], &[(1, prefix, &bytes)]));
            cases.push(run(
                &[0x12, 0x34],
                &[(3, prefix, &bytes), (1, prefix, &bytes)],
            ));
        }
    }
    exact(cases);
}

fn profile(size: usize, pattern: usize) -> Vec<u8> {
    let mut data = vec![0; size];
    let mut random = 0x123456789abcdefu64;
    let mut period = Vec::new();
    for _ in 0..16381 {
        random ^= random << 13;
        random ^= random >> 7;
        random ^= random << 17;
        period.push(random as u8);
    }
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = match pattern {
            0 => 0,
            1 => (i % 251) as u8,
            2 => period[i % period.len()],
            _ => period[(i * 73 + i / 16381) % period.len()],
        };
    }
    let header = jxl::jxl::icc_codec_common::icc_initial_header_prediction(size as u32);
    let prefix = size.min(128);
    data[..prefix].copy_from_slice(&header[..prefix]);
    if size >= 132 {
        data[128..132].fill(0); // A real zero-entry ICC tag table.
    }
    data
}

fn source_success(request: &str, profile: &[u8]) {
    let original = original_observations(&[request.to_owned()]);
    let fields: Vec<_> = original[0].split_whitespace().collect();
    let start = fields.iter().rposition(|field| *field == "S").unwrap() + 1;
    assert_eq!(fields[start + 1], "0", "source Init");
    assert_eq!(fields[start + 2], "0", "source Process");
    assert_eq!(
        fields[start + 14],
        hex(profile),
        "actual WriteICC roundtrip"
    );
}

#[test]
fn icc_reader_genuine_writeicc_replays_reset_reuse_and_exact_output() {
    let mut cases = Vec::new();
    for pattern in 0..3 {
        for size in [1, 2, 19, 20, 127, 128, 129, 131, 132, 255, 512, 1024, 4096] {
            let profile = profile(size, pattern);
            for prefix in [0, 1, 3, 7] {
                let (_, bytes) = source_stream(format!("ir-write {prefix} {}", hex(&profile)));
                let request = run(
                    &[0x12, 0x34],
                    &[
                        (0, prefix, &bytes),
                        (3, 0, &[]),
                        (2, prefix, &bytes),
                        (2, prefix, &bytes),
                        (3, 0, &[]),
                        (2, prefix, &bytes),
                    ],
                );
                source_success(&request, &profile);
                cases.push(request);
            }
        }
    }
    exact(cases);
}

#[test]
fn icc_reader_chunked_replay_retains_checkpoint_and_speculative_prefixes() {
    let mut cases = Vec::new();
    for size in [17, 128, 132, 511, 512, 513, 1023, 1024, 1025, 4097, 16384] {
        for pattern in [0, 2] {
            let input = profile(size, pattern);
            for prefix in [0, 7] {
                let (_, bytes) = source_stream(format!("ir-write {prefix} {}", hex(&input)));
                let mut cuts: Vec<_> = (0..=bytes.len().min(40)).collect();
                cuts.extend([
                    bytes.len() / 4,
                    bytes.len() / 2,
                    bytes.len() * 3 / 4,
                    bytes.len().saturating_sub(1),
                    bytes.len(),
                ]);
                cuts.sort_unstable();
                cuts.dedup();
                for &cut in &cuts {
                    cases.push(run(
                        &[0xaa, 0x55],
                        &[(2, prefix, &bytes[..cut]), (2, prefix, &bytes)],
                    ));
                }
                let steps: Vec<_> = cuts.iter().map(|&cut| (5, prefix, &bytes[..cut])).collect();
                // Keep unrecovered replay and its fatal state/status alongside
                // the explicit caller recovery; neither outcome is normalized.
                let original_steps: Vec<_> =
                    cuts.iter().map(|&cut| (2, prefix, &bytes[..cut])).collect();
                cases.push(run(&[0xaa, 0x55], &original_steps));
                let request = run(&[0xaa, 0x55], &steps);
                source_success(&request, &input);
                cases.push(request);
            }
        }
    }
    exact(cases);
}

#[test]
fn icc_reader_original_ans_and_huffman_entropy_modes_preserve_errors_and_output() {
    let input = profile(4097, 2);
    let predicted = original_observations(&[format!("ic-encode {}", hex(&input))]);
    let mut words = predicted[0].split_whitespace();
    assert_eq!(words.next(), Some("0"));
    let encoded = unhex(words.next().unwrap());
    let mut cases = Vec::new();
    for prefix in [0, 3, 7] {
        for huffman in [0, 1] {
            for lz in [0, 1, 2] {
                let (_, bytes) = source_stream(format!(
                    "ir-pack {prefix} {} {huffman} {lz} {}",
                    encoded.len(),
                    hex(&encoded),
                ));
                let request = run(&[0xcc, 0x33], &[(2, prefix, &bytes)]);
                source_success(&request, &input);
                cases.push(request);
                for cut in [
                    0,
                    1,
                    2,
                    bytes.len() / 4,
                    bytes.len() / 2,
                    bytes.len().saturating_sub(1),
                ] {
                    cases.push(run(
                        &[0xcc, 0x33],
                        &[(2, prefix, &bytes[..cut]), (2, prefix, &bytes)],
                    ));
                }
                for index in [
                    bytes.len() / 2,
                    bytes.len().saturating_sub(2),
                    bytes.len() - 1,
                ] {
                    for mask in [1, 0x40, 0x80] {
                        let mut mutated = bytes.clone();
                        mutated[index] ^= mask;
                        cases.push(run(
                            &[0xcc, 0x33],
                            &[(2, prefix, &mutated), (3, 0, &[]), (2, prefix, &bytes)],
                        ));
                    }
                }
            }
        }
    }
    exact(cases);
}

#[test]
fn icc_reader_replay_base_offsets_and_shorter_backing_keep_actual_skip_state() {
    let mut cases = Vec::new();
    for size in [132, 1025, 4097] {
        let input = profile(size, 2);
        let (_, first) = source_stream(format!("ir-write 0 {}", hex(&input)));
        let (_, shifted) = source_stream(format!("ir-write 7 {}", hex(&input)));
        let request = run(&[0x44], &[(0, 0, &first), (2, 7, &shifted)]);
        source_success(&request, &input);
        cases.push(request);
        // A shorter replay can skip past its new logical bounds. Keep its
        // actual source status/state, then recover with an explicit Reset.
        cases.push(run(
            &[0x44],
            &[
                (0, 0, &first),
                (2, 7, &shifted[..1]),
                (3, 0, &[]),
                (2, 7, &shifted),
            ],
        ));
    }
    exact(cases);
}

#[test]
fn icc_reader_fatal_header_preamble_and_unpredict_errors_preserve_partial_state() {
    let good_profile = profile(256, 2);
    let (_, good) = source_stream(format!("ir-write 0 {}", hex(&good_profile)));
    let mut cases = Vec::new();
    for prefix in [0, 1, 7] {
        for size in [268_435_457u64, u32::MAX as u64, u64::MAX] {
            let (_, bytes) = source_stream(format!("ir-size {prefix} {size}"));
            cases.push(run(
                &[0x37],
                &[(2, prefix, &bytes), (3, 0, &[]), (2, 0, &good)],
            ));
        }
        for raw in [
            vec![0],
            vec![1],
            vec![0, 0],
            vec![1, 0],
            vec![0xff; 20],
            vec![128, 1, 1, 0xff],
            vec![128, 1, 0, 0],
            vec![0; 32],
        ] {
            for huffman in [0, 1] {
                let (_, bytes) = source_stream(format!(
                    "ir-pack {prefix} {} {huffman} 0 {}",
                    raw.len(),
                    hex(&raw)
                ));
                cases.push(run(
                    &[0x37, 0x83],
                    &[(2, prefix, &bytes), (3, 0, &[]), (2, 0, &good)],
                ));
            }
        }
    }
    exact(cases);
}

#[test]
fn icc_reader_lz77_window_wrap_and_source_expansion_ratio_guard() {
    let input = profile((1 << 20) + 1025, 2);
    let (_, bytes) = source_stream(format!("ir-write 0 {}", hex(&input)));
    let request = run(&[0x12], &[(2, 0, &bytes), (2, 0, &bytes)]);
    source_success(&request, &input);
    let mut excessive = vec![0; 65537];
    excessive[0] = 1;
    let (_, guard) = source_stream(format!(
        "ir-pack 0 {} 1 2 {}",
        excessive.len(),
        hex(&excessive)
    ));
    exact(vec![request, run(&[0x45, 0x67], &[(2, 0, &guard)])]);
}

#[test]
fn icc_reader_cached_mode_and_live_views_survive_defined_fatal_histogram_updates() {
    // Independent pristine observations establish both cached-mode directions.
    // The first exact request is the review counterexample. Cache observations
    // prove source pointer identity for the reached table/config arrays; an
    // unused Huffman view may change while cached ANS reads remain defined.
    let cases = include_str!("../data/icc-reader-cache.requests")
        .lines()
        .map(str::to_owned)
        .collect();
    exact(cases);
}
